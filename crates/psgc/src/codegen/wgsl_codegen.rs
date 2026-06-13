//! # WGSL Code Generator
//!
//! Generates WGSL shader code from node graphs.
//!
//! Shader graphs are **fully pure**: every node (math, color, texture, input)
//! is a side-effect-free expression, and the graph is a single dataflow DAG
//! that terminates in one `fragment_output`/`vertex_output` node. There is no
//! separate "entry"/execution node — the output node *is* the entry point,
//! and the surrounding `@vertex`/`@fragment` function is generated around it.

use std::collections::HashSet;

use crate::metadata::ShaderMetadataProvider;
use graphy::{DataResolver, DataSource, GraphDescription, GraphyError, NodeInstance, NodeMetadataProvider, ParamInfo};

/// Shader stage type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

/// WGSL shader code generator
pub struct WGSLCodeGenerator<'a> {
    graph: &'a GraphDescription,
    metadata_provider: &'a ShaderMetadataProvider,
    data_resolver: &'a DataResolver,
    stage: ShaderStage,
}

impl<'a> WGSLCodeGenerator<'a> {
    pub fn new(
        graph: &'a GraphDescription,
        metadata_provider: &'a ShaderMetadataProvider,
        data_resolver: &'a DataResolver,
        stage: ShaderStage,
    ) -> Self {
        Self {
            graph,
            metadata_provider,
            data_resolver,
            stage,
        }
    }

    /// Generate complete WGSL shader
    pub fn generate_shader(&self) -> Result<String, GraphyError> {
        let mut code = String::new();

        // Add header
        code.push_str("// Auto-generated WGSL shader from Pulsar Shader Graph\n");
        code.push_str("// DO NOT EDIT - Changes will be overwritten\n");
        code.push_str("// Compiled with PSGC (Pulsar Shader Graph Compiler)\n\n");

        // A pure shader graph's entry point is its output node — there is no
        // separate event/exec entry node to look for.
        let output_node_type = match self.stage {
            ShaderStage::Vertex => "vertex_output",
            ShaderStage::Fragment => "fragment_output",
            ShaderStage::Compute => {
                return Err(GraphyError::CodeGeneration(
                    "Compute shaders are not yet supported by the shader graph compiler".to_string(),
                ));
            }
        };

        let output_node = self
            .graph
            .nodes
            .values()
            .find(|node| node.node_type == output_node_type)
            .ok_or_else(|| {
                GraphyError::CodeGeneration(format!(
                    "No {} node found in graph — every shader graph needs exactly one output node",
                    output_node_type
                ))
            })?;

        code.push_str(&self.generate_entry_function(output_node)?);

        Ok(code)
    }

    /// Generate the `@vertex`/`@fragment` entry function that wraps the pure
    /// dataflow graph feeding into `output_node`.
    fn generate_entry_function(&self, output_node: &NodeInstance) -> Result<String, GraphyError> {
        let mut code = String::new();

        // Always declare the host-provided `uniforms` binding (matching the
        // preview renderer's vertex shader) so the generated fragment
        // module's bind group layout stays compatible whether or not this
        // particular graph reads from it (e.g. via the `time` input node).
        code.push_str("struct Uniforms {\n");
        code.push_str("    view_proj: mat4x4<f32>,\n");
        code.push_str("    model: mat4x4<f32>,\n");
        code.push_str("    time: f32,\n");
        code.push_str("};\n\n");
        code.push_str("@group(0) @binding(0) var<uniform> uniforms: Uniforms;\n\n");

        // Generate function signature based on stage
        let (output_param, output_type) = match self.stage {
            ShaderStage::Vertex => {
                code.push_str("@vertex\n");
                code.push_str("fn vertex_main(\n");
                code.push_str("    @builtin(vertex_index) vertex_index: u32,\n");
                code.push_str(") -> @builtin(position) vec4<f32> {\n");
                ("position", "vec4<f32>")
            }
            ShaderStage::Fragment => {
                code.push_str("@fragment\n");
                code.push_str("fn fragment_main(\n");
                code.push_str("    @builtin(position) frag_coord: vec4<f32>,\n");
                // Matches the preview renderer's hand-written vertex shader
                // `VertexOutput` locations, so `frag_uv`/`frag_normal` input
                // nodes can read the interpolated per-fragment values
                // instead of always defaulting to zero.
                code.push_str("    @location(0) uv: vec2<f32>,\n");
                code.push_str("    @location(1) normal: vec3<f32>,\n");
                code.push_str("    @location(2) world_pos: vec3<f32>,\n");
                code.push_str(") -> @location(0) vec4<f32> {\n");
                ("color", "vec4<f32>")
            }
            ShaderStage::Compute => unreachable!("compute shaders are rejected in generate_shader"),
        };

        // Disconnected node chains (not reachable from the output) are
        // allowed to be incomplete/invalid in the editor — only emit
        // bindings for nodes the output actually depends on, in topological
        // (dependency-first) order. Without this, an unused node with a
        // dangling/invalid input could generate WGSL that fails to compile
        // and crashes the renderer even though it has no effect on the
        // result.
        let reachable = self.reachable_nodes(output_node);

        for node_id in self.data_resolver.get_pure_evaluation_order() {
            if !reachable.contains(node_id) {
                continue;
            }

            let node = self
                .graph
                .nodes
                .get(node_id)
                .ok_or_else(|| GraphyError::NodeNotFound(node_id.clone()))?;
            let node_meta = self
                .metadata_provider
                .get_node_metadata(&node.node_type)
                .ok_or_else(|| GraphyError::NodeNotFound(node.node_type.clone()))?;
            let var_name = self
                .data_resolver
                .get_result_variable(node_id)
                .ok_or_else(|| GraphyError::Custom(format!("No result variable for node: {}", node_id)))?;
            let return_type = node_meta
                .return_type
                .as_ref()
                .map(|t| t.type_string.as_str())
                .unwrap_or("f32");

            let expr = if node_meta.category == "Input" {
                self.input_binding(&node.node_type, return_type)
            } else {
                let mut args = Vec::with_capacity(node_meta.params.len());
                for param in &node_meta.params {
                    args.push(self.generate_input_expression(node_id, &param.name, &param.param_type)?);
                }
                expand_function_source(&node_meta.function_source, &node_meta.params, &args)
            };

            code.push_str(&format!("    let {} = {};\n", var_name, expr));
        }

        // The function's return value is whatever flows into the output
        // node's single parameter (defaulting if nothing is connected).
        let final_expr = self.generate_input_expression(&output_node.id, output_param, output_type)?;
        code.push_str(&format!("    return {};\n", final_expr));
        code.push_str("}\n");

        Ok(code)
    }

    /// Walk data connections backward from `output_node` to find every node
    /// it (transitively) depends on. Nodes outside this set are dead code —
    /// they don't feed into the shader's output — and are skipped during
    /// codegen so a broken/incomplete chain sitting off to the side can't
    /// produce invalid WGSL.
    fn reachable_nodes(&self, output_node: &NodeInstance) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut stack = vec![output_node.id.clone()];

        while let Some(node_id) = stack.pop() {
            let Some(node) = self.graph.nodes.get(&node_id) else {
                continue;
            };

            for pin in &node.inputs {
                if let Some(DataSource::Connection { source_node_id, .. }) =
                    self.data_resolver.get_input_source(&node_id, &pin.id)
                {
                    if visited.insert(source_node_id.clone()) {
                        stack.push(source_node_id.clone());
                    }
                }
            }
        }

        visited
    }

    /// Resolve the expression that should be substituted for a node input.
    ///
    /// Since every pure node already has a `let` binding emitted (in
    /// dependency order) before any of its dependents, a connected input is
    /// simply a reference to that variable.
    fn generate_input_expression(
        &self,
        node_id: &str,
        pin_name: &str,
        param_type: &str,
    ) -> Result<String, GraphyError> {
        match self.data_resolver.get_input_source(node_id, pin_name) {
            Some(DataSource::Connection { source_node_id, .. }) => self
                .data_resolver
                .get_result_variable(source_node_id)
                .cloned()
                .ok_or_else(|| GraphyError::Custom(format!("No result variable for node: {}", source_node_id))),
            Some(DataSource::Constant(value)) => Ok(value.clone()),
            Some(DataSource::Default) | None => Ok(default_value_for_type(param_type)),
        }
    }

    /// Map a built-in "Input" category node (e.g. `frag_position`,
    /// `vertex_uv`, `time`) to the WGSL expression that provides its value.
    ///
    /// `frag_position` maps to the `frag_coord` builtin parameter and `time`
    /// maps to the host-provided `uniforms.time` binding (see
    /// `generate_entry_function`); the remaining inputs are placeholders
    /// until vertex-buffer/interstage wiring is added to PSGC.
    fn input_binding(&self, node_type: &str, return_type: &str) -> String {
        match (self.stage, node_type) {
            (ShaderStage::Fragment, "frag_position") => "frag_coord".to_string(),
            (ShaderStage::Fragment, "frag_uv") => "uv".to_string(),
            (ShaderStage::Fragment, "frag_normal") => "normal".to_string(),
            (_, "time") => "uniforms.time".to_string(),
            _ => default_value_for_type(return_type),
        }
    }
}

/// Expand a node's `function_source` expression template, substituting each
/// parameter name with its resolved argument expression.
///
/// `function_source` is a small WGSL expression written in terms of the
/// node's own parameter names, e.g. `"a + b"` for `add` or `"mix(a, b, t)"`
/// for `lerp`. Identifiers matching a parameter name are replaced with the
/// (parenthesized) argument expression; everything else (operators, WGSL
/// built-in function names, literals) is passed through unchanged.
fn expand_function_source(source: &str, params: &[ParamInfo], args: &[String]) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut result = String::with_capacity(source.len());
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();
            if let Some(pos) = params.iter().position(|p| p.name == ident) {
                result.push('(');
                result.push_str(&args[pos]);
                result.push(')');
            } else {
                result.push_str(&ident);
            }
        } else {
            result.push(c);
            i += 1;
        }
    }

    result
}

/// A sensible zero value for a WGSL type, used for unconnected inputs.
///
/// `vec4<f32>` defaults to opaque black (`w = 1.0`) since it's almost always
/// used for colors or homogeneous positions, both of which want `w = 1.0`.
fn default_value_for_type(type_str: &str) -> String {
    match type_str {
        "f32" => "0.0".to_string(),
        "i32" => "0".to_string(),
        "u32" => "0u".to_string(),
        "bool" => "false".to_string(),
        "vec2<f32>" => "vec2<f32>(0.0, 0.0)".to_string(),
        "vec3<f32>" => "vec3<f32>(0.0, 0.0, 0.0)".to_string(),
        "vec4<f32>" => "vec4<f32>(0.0, 0.0, 0.0, 1.0)".to_string(),
        _ => "0.0".to_string(),
    }
}

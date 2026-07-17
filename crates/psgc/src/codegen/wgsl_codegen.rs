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

use graphy::{DataResolver, DataSource, GraphDescription, GraphyError, NodeInstance, NodeMetadataProvider, ParamInfo};

/// Shader stage type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

/// WGSL shader code generator
pub struct WGSLCodeGenerator<'a, P: NodeMetadataProvider> {
    graph: &'a GraphDescription,
    metadata_provider: &'a P,
    data_resolver: &'a DataResolver,
    stage: ShaderStage,
}

impl<'a, P: NodeMetadataProvider> WGSLCodeGenerator<'a, P> {
    pub fn new(
        graph: &'a GraphDescription,
        metadata_provider: &'a P,
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
    ///
    /// For the fragment stage the output node's params become the fields of a
    /// `FragmentOutput` struct, each with a sequential `@location(N)` — this
    /// lets the fragment shader output multiple PBR properties (base colour,
    /// metallic, roughness, …) simultaneously.
    ///
    /// For the vertex stage a `VertexOutput` struct is emitted: the
    /// `position` param becomes `@builtin(position)` and the remaining params
    /// become interstage `@location` attributes.
    fn generate_entry_function(&self, output_node: &NodeInstance) -> Result<String, GraphyError> {
        let mut code = String::new();

        // Only nodes the output actually depends on contribute code — both
        // helper functions here and `let` bindings below.
        let reachable = self.reachable_nodes(output_node);

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

        // Emit each reachable node's module-scope helper functions exactly
        // once (dedup by name, first definition wins). WGSL permits forward
        // references between module-scope functions, so emission order is a
        // readability nicety, not a correctness requirement.
        let mut emitted: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        for node_id in self.data_resolver.get_pure_evaluation_order() {
            if !reachable.contains(node_id) {
                continue;
            }
            let Some(node) = self.graph.nodes.get(node_id) else { continue };
            let Some(node_meta) = self.metadata_provider.get_node_metadata(&node.node_type) else {
                continue;
            };
            for (name, source) in &node_meta.helper_functions {
                match emitted.get(name.as_str()) {
                    None => {
                        emitted.insert(name.as_str(), source.as_str());
                        code.push_str(source);
                        code.push_str("\n\n");
                    }
                    Some(prev) if *prev != source.as_str() => {
                        tracing::warn!(
                            "WGSL helper '{}' redefined with different source by node type '{}'; keeping first definition",
                            name,
                            node.node_type
                        );
                    }
                    _ => {}
                }
            }
        }

        let output_meta = self
            .metadata_provider
            .get_node_metadata(&output_node.node_type)
            .ok_or_else(|| GraphyError::NodeNotFound(output_node.node_type.clone()))?;

        // Emit the output struct and function signature.
        match self.stage {
            ShaderStage::Fragment => {
                let struct_name = "FragmentOutput";
                code.push_str(&format!("struct {} {{\n", struct_name));
                for (i, param) in output_meta.params.iter().enumerate() {
                    code.push_str(&format!("    @location({}) {}: {},\n", i, param.name, param.param_type));
                }
                code.push_str("};\n\n");

                code.push_str("@fragment\n");
                code.push_str("fn fragment_main(\n");
                code.push_str("    @builtin(position) frag_coord: vec4<f32>,\n");
                code.push_str("    @location(0) uv: vec2<f32>,\n");
                code.push_str("    @location(1) normal: vec3<f32>,\n");
                code.push_str("    @location(2) world_pos: vec3<f32>,\n");
                code.push_str(&format!(") -> {} {{\n", struct_name));
            }
            ShaderStage::Vertex => {
                let struct_name = "VertexOutput";
                let mut loc_idx = 0u32;
                code.push_str(&format!("struct {} {{\n", struct_name));
                for param in &output_meta.params {
                    if param.name == "position" {
                        code.push_str(&format!(
                            "    @builtin(position) {}: {},\n",
                            param.name, param.param_type
                        ));
                    } else {
                        code.push_str(&format!(
                            "    @location({}) {}: {},\n",
                            loc_idx, param.name, param.param_type
                        ));
                        loc_idx += 1;
                    }
                }
                code.push_str("};\n\n");

                code.push_str("@vertex\n");
                code.push_str("fn vertex_main(\n");
                code.push_str("    @builtin(vertex_index) vertex_index: u32,\n");
                code.push_str(&format!(") -> {} {{\n", struct_name));
            }
            ShaderStage::Compute => {
                unreachable!("compute shaders are rejected in generate_shader")
            }
        };

        // Disconnected node chains (not reachable from the output) are
        // allowed to be incomplete/invalid in the editor — only emit
        // bindings for nodes the output actually depends on, in topological
        // (dependency-first) order. Without this, an unused node with a
        // dangling/invalid input could generate WGSL that fails to compile
        // and crashes the renderer even though it has no effect on the
        // result.
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

        // Build the struct literal from the output node's params.  Each
        // field resolves to whatever is connected (or a type-appropriate
        // default if nothing is connected).
        let struct_name = match self.stage {
            ShaderStage::Fragment => "FragmentOutput",
            ShaderStage::Vertex => "VertexOutput",
            ShaderStage::Compute => unreachable!(),
        };
        code.push_str(&format!("    return {}(", struct_name));
        for (j, param) in output_meta.params.iter().enumerate() {
            if j > 0 {
                code.push_str(", ");
            }
            let expr =
                self.generate_input_expression(&output_node.id, &param.name, &param.param_type)?;
            code.push_str(&expr);
        }
        code.push_str(");\n");
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

#[cfg(test)]
mod helper_emission_tests {
    use super::*;
    use graphy::core::{NodeMetadata, NodeMetadataProvider};
    use graphy::{
        Connection, ConnectionType, DataResolver, DataType, GraphDescription, NodeInstance,
        NodeTypes, Pin, PinInstance, PinType, Position,
    };
    use std::collections::HashMap;

    struct TestProvider {
        nodes: HashMap<String, NodeMetadata>,
    }

    impl TestProvider {
        fn new(metas: Vec<NodeMetadata>) -> Self {
            Self {
                nodes: metas.into_iter().map(|m| (m.name.clone(), m)).collect(),
            }
        }
    }

    impl NodeMetadataProvider for TestProvider {
        fn get_node_metadata(&self, node_type: &str) -> Option<&NodeMetadata> {
            self.nodes.get(node_type)
        }
        fn get_all_nodes(&self) -> Vec<&NodeMetadata> {
            self.nodes.values().collect()
        }
        fn get_nodes_by_category(&self, category: &str) -> Vec<&NodeMetadata> {
            self.nodes.values().filter(|m| m.category == category).collect()
        }
    }

    fn scalar_node(graph: &mut GraphDescription, id: &str, node_type: &str) {
        let mut n = NodeInstance::new(id, node_type, Position { x: 0.0, y: 0.0 });
        n.inputs.push(PinInstance::new(
            format!("{id}_x"),
            Pin::new(format!("{id}_x"), "x", DataType::Data(crate::TypeInfo::new("f32")), PinType::Input),
        ));
        n.outputs.push(PinInstance::new(
            format!("{id}_result"),
            Pin::new(format!("{id}_result"), "result", DataType::Data(crate::TypeInfo::new("f32")), PinType::Output),
        ));
        graph.add_node(n);
    }

    fn build_graph_and_provider() -> (GraphDescription, TestProvider) {
        let provider = TestProvider::new(vec![
            NodeMetadata::new("noisy_scalar", NodeTypes::pure, "Test")
                .with_params(vec![graphy::ParamInfo::new("x", "f32")])
                .with_return_type("f32")
                .with_helpers(&[
                    ("pn_shared_hash", "fn pn_shared_hash(x: f32) -> f32 { return fract(x * 0.1031); }"),
                    ("pn_noisy", "fn pn_noisy(x: f32) -> f32 { return pn_shared_hash(x) * 2.0; }"),
                ])
                .with_source("pn_noisy(x)"),
            NodeMetadata::new("noisy_to_color", NodeTypes::pure, "Test")
                .with_params(vec![graphy::ParamInfo::new("x", "f32")])
                .with_return_type("vec4<f32>")
                .with_helpers(&[
                    ("pn_shared_hash", "fn pn_shared_hash(x: f32) -> f32 { return fract(x * 0.1031); }"),
                    ("pn_to_color", "fn pn_to_color(x: f32) -> vec4<f32> { let v = pn_shared_hash(x); return vec4<f32>(v, v, v, 1.0); }"),
                ])
                .with_source("pn_to_color(x)"),
            NodeMetadata::new("fragment_output", NodeTypes::pure, "Output")
                .with_params(vec![graphy::ParamInfo::new("color", "vec4<f32>")])
                .with_return_type("vec4<f32>")
                .with_source("color"),
        ]);

        let mut graph = GraphDescription::new("helper_test");
        scalar_node(&mut graph, "n1", "noisy_scalar");
        scalar_node(&mut graph, "n2", "noisy_to_color");

        let mut out = NodeInstance::new("out", "fragment_output", Position { x: 0.0, y: 0.0 });
        out.inputs.push(PinInstance::new(
            "out_color",
            Pin::new("out_color", "color", DataType::Data(crate::TypeInfo::new("vec4<f32>")), PinType::Input),
        ));
        graph.add_node(out);

        graph.add_connection(Connection::new("n1", "n1_result", "n2", "n2_x", ConnectionType::Data));
        graph.add_connection(Connection::new("n2", "n2_result", "out", "out_color", ConnectionType::Data));
        (graph, provider)
    }

    fn generate(graph: &GraphDescription, provider: &TestProvider) -> String {
        let resolver = DataResolver::build(graph, provider).expect("data flow");
        WGSLCodeGenerator::new(graph, provider, &resolver, ShaderStage::Fragment)
            .generate_shader()
            .expect("codegen")
    }

    #[test]
    fn helpers_emitted_once_at_module_scope() {
        let (graph, provider) = build_graph_and_provider();
        let wgsl = generate(&graph, &provider);

        assert_eq!(wgsl.matches("fn pn_shared_hash").count(), 1, "shared helper deduped:\n{wgsl}");
        assert_eq!(wgsl.matches("fn pn_noisy").count(), 1);
        assert_eq!(wgsl.matches("fn pn_to_color").count(), 1);

        let entry = wgsl.find("@fragment").expect("entry marker");
        for helper in ["fn pn_shared_hash", "fn pn_noisy", "fn pn_to_color"] {
            assert!(wgsl.find(helper).unwrap() < entry, "{helper} must precede @fragment");
        }
    }

    #[test]
    fn unreachable_nodes_contribute_no_helpers() {
        let (mut graph, mut provider) = build_graph_and_provider();
        // A node type with a UNIQUE helper, instantiated but never connected
        // to the output — its helper must not be emitted.
        provider.nodes.insert(
            "orphan_only".to_string(),
            NodeMetadata::new("orphan_only", NodeTypes::pure, "Test")
                .with_params(vec![graphy::ParamInfo::new("x", "f32")])
                .with_return_type("f32")
                .with_helpers(&[("pn_orphan_helper", "fn pn_orphan_helper(x: f32) -> f32 { return x; }")])
                .with_source("pn_orphan_helper(x)"),
        );
        scalar_node(&mut graph, "orphan", "orphan_only");
        let wgsl = generate(&graph, &provider);
        assert!(!wgsl.contains("fn pn_orphan_helper"), "unreachable node's helper must not be emitted:\n{wgsl}");
        assert_eq!(wgsl.matches("fn pn_shared_hash").count(), 1);
    }

    #[test]
    fn helper_free_nodes_generate_unchanged() {
        let provider = TestProvider::new(vec![NodeMetadata::new(
            "fragment_output",
            NodeTypes::pure,
            "Output",
        )
        .with_params(vec![graphy::ParamInfo::new("color", "vec4<f32>")])
        .with_return_type("vec4<f32>")
        .with_source("color")]);
        let mut graph = GraphDescription::new("plain");
        let mut out = NodeInstance::new("out", "fragment_output", Position { x: 0.0, y: 0.0 });
        out.inputs.push(PinInstance::new(
            "out_color",
            Pin::new("out_color", "color", DataType::Data(crate::TypeInfo::new("vec4<f32>")), PinType::Input),
        ));
        graph.add_node(out);
        let wgsl = generate(&graph, &provider);
        assert!(!wgsl.contains("fn pn_"), "no helpers expected:\n{wgsl}");
        assert!(wgsl.contains("@fragment"));
    }
}

//! # WGSL Code Generator
//!
//! Generates WGSL shader code from node graphs.

use crate::metadata::ShaderMetadataProvider;
use graphy::{
    GraphDescription, GraphyError, NodeTypes, NodeInstance,
    DataResolver, ExecutionRouting,
};
use graphy::core::NodeMetadataProvider;
use std::collections::{HashMap, HashSet};

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
    exec_routing: &'a ExecutionRouting,
    stage: ShaderStage,
    visited: HashSet<String>,
}

impl<'a> WGSLCodeGenerator<'a> {
    pub fn new(
        graph: &'a GraphDescription,
        metadata_provider: &'a ShaderMetadataProvider,
        data_resolver: &'a DataResolver,
        exec_routing: &'a ExecutionRouting,
        stage: ShaderStage,
    ) -> Self {
        Self {
            graph,
            metadata_provider,
            data_resolver,
            exec_routing,
            stage,
            visited: HashSet::new(),
        }
    }

    /// Generate complete WGSL shader
    pub fn generate_shader(&self) -> Result<String, GraphyError> {
        let mut code = String::new();

        // Add header
        code.push_str("// Auto-generated WGSL shader from Pulsar Shader Graph\n");
        code.push_str("// DO NOT EDIT - Changes will be overwritten\n");
        code.push_str("// Compiled with PSGC (Pulsar Shader Graph Compiler)\n\n");

        // Find entry point based on stage
        let entry_node_type = match self.stage {
            ShaderStage::Vertex => "vertex_main",
            ShaderStage::Fragment => "fragment_main",
            ShaderStage::Compute => "compute_main",
        };

        let entry_nodes: Vec<_> = self.graph
            .nodes
            .values()
            .filter(|node| node.node_type == entry_node_type)
            .collect();

        if entry_nodes.is_empty() {
            return Err(GraphyError::CodeGeneration(format!(
                "No {} entry point found in graph",
                entry_node_type
            )));
        }

        // Generate entry function
        for entry_node in entry_nodes {
            let entry_code = self.generate_entry_function(entry_node)?;
            code.push_str(&entry_code);
            code.push_str("\n");
        }

        Ok(code)
    }

    /// Generate entry function
    fn generate_entry_function(&self, entry_node: &NodeInstance) -> Result<String, GraphyError> {
        let mut code = String::new();

        // Get entry metadata
        let metadata = self.metadata_provider
            .get_node_metadata(&entry_node.node_type)
            .ok_or_else(|| GraphyError::NodeNotFound(entry_node.node_type.clone()))?;

        // Generate function signature based on stage
        match self.stage {
            ShaderStage::Vertex => {
                code.push_str("@vertex\n");
                code.push_str("fn vertex_main(\n");
                code.push_str("    @builtin(vertex_index) vertex_index: u32,\n");
                code.push_str(") -> @builtin(position) vec4<f32> {\n");
            }
            ShaderStage::Fragment => {
                code.push_str("@fragment\n");
                code.push_str("fn fragment_main(\n");
                code.push_str("    @builtin(position) frag_coord: vec4<f32>,\n");
                code.push_str(") -> @location(0) vec4<f32> {\n");
            }
            ShaderStage::Compute => {
                code.push_str("@compute @workgroup_size(8, 8, 1)\n");
                code.push_str("fn compute_main(\n");
                code.push_str("    @builtin(global_invocation_id) global_id: vec3<u32>,\n");
                code.push_str(") {\n");
            }
        }

        // Generate body
        if let Some(body_pin) = metadata.exec_outputs.first() {
            let connected = self.exec_routing.get_connected_nodes(&entry_node.id, body_pin);
            for next_node_id in connected {
                if let Some(next_node) = self.graph.nodes.get(next_node_id) {
                    let mut generator = self.clone_with_new_visited();
                    let node_code = generator.generate_node_chain(next_node, 1)?;
                    code.push_str(&node_code);
                }
            }
        }

        // Return statement based on stage
        match self.stage {
            ShaderStage::Vertex => {
                code.push_str("    return vec4<f32>(0.0, 0.0, 0.0, 1.0);\n");
            }
            ShaderStage::Fragment => {
                code.push_str("    return vec4<f32>(1.0, 0.0, 1.0, 1.0);\n");
            }
            ShaderStage::Compute => {}
        }

        code.push_str("}\n");

        Ok(code)
    }

    /// Generate node chain
    fn generate_node_chain(&mut self, node: &NodeInstance, indent_level: usize) -> Result<String, GraphyError> {
        let mut code = String::new();

        // Prevent infinite loops
        if self.visited.contains(&node.id) {
            return Ok(code);
        }
        self.visited.insert(node.id.clone());

        let node_meta = self.metadata_provider
            .get_node_metadata(&node.node_type)
            .ok_or_else(|| GraphyError::NodeNotFound(node.node_type.clone()))?;

        match node_meta.node_type {
            NodeTypes::pure => {
                // Pure nodes are inlined as expressions
                Ok(code)
            }
            NodeTypes::fn_ => {
                self.generate_function_node(node, node_meta, indent_level)
            }
            NodeTypes::control_flow => {
                self.generate_control_flow_node(node, node_meta, indent_level)
            }
            NodeTypes::event => {
                // Event nodes are entry points
                Ok(code)
            }
        }
    }

    /// Generate function node
    fn generate_function_node(
        &mut self,
        node: &NodeInstance,
        node_meta: &graphy::core::NodeMetadata,
        indent_level: usize,
    ) -> Result<String, GraphyError> {
        let mut code = String::new();
        let indent = "    ".repeat(indent_level);

        // Collect arguments
        let args = self.collect_arguments(node, node_meta)?;

        // Check if this function returns a value
        let has_return = node_meta.return_type.is_some();

        if has_return {
            let result_var = self.data_resolver
                .get_result_variable(&node.id)
                .ok_or_else(|| GraphyError::Custom(format!("No result variable for node: {}", node.id)))?;

            code.push_str(&format!(
                "{}let {} = {}({});\n",
                indent,
                result_var,
                self.map_function_name(&node_meta.name),
                args.join(", ")
            ));
        } else {
            code.push_str(&format!(
                "{}{}({});\n",
                indent,
                self.map_function_name(&node_meta.name),
                args.join(", ")
            ));
        }

        // Follow execution chain
        if let Some(exec_out) = node_meta.exec_outputs.first() {
            let connected = self.exec_routing.get_connected_nodes(&node.id, exec_out);
            for next_node_id in connected {
                if let Some(next_node) = self.graph.nodes.get(next_node_id) {
                    let next_code = self.generate_node_chain(next_node, indent_level)?;
                    code.push_str(&next_code);
                }
            }
        }

        Ok(code)
    }

    /// Generate control flow node
    fn generate_control_flow_node(
        &mut self,
        _node: &NodeInstance,
        _node_meta: &graphy::core::NodeMetadata,
        indent_level: usize,
    ) -> Result<String, GraphyError> {
        let indent = "    ".repeat(indent_level);
        // Placeholder for control flow
        Ok(format!("{}// Control flow node\n", indent))
    }

    /// Collect arguments for a function call
    fn collect_arguments(&self, node: &NodeInstance, node_meta: &graphy::core::NodeMetadata) -> Result<Vec<String>, GraphyError> {
        let mut args = Vec::new();

        for param in &node_meta.params {
            let value = self.generate_input_expression(&node.id, &param.name)?;
            args.push(value);
        }

        Ok(args)
    }

    /// Generate expression for an input value
    fn generate_input_expression(&self, node_id: &str, pin_name: &str) -> Result<String, GraphyError> {
        use graphy::analysis::DataSource;

        match self.data_resolver.get_input_source(node_id, pin_name) {
            Some(DataSource::Connection { source_node_id, source_pin: _ }) => {
                let source_node = self.graph.nodes.get(source_node_id)
                    .ok_or_else(|| GraphyError::NodeNotFound(source_node_id.clone()))?;

                // Check if source is pure - if so, inline it
                if let Some(node_meta) = self.metadata_provider.get_node_metadata(&source_node.node_type) {
                    if node_meta.node_type == NodeTypes::pure {
                        return self.generate_pure_node_expression(source_node);
                    }
                }

                // Non-pure: use result variable
                if let Some(var_name) = self.data_resolver.get_result_variable(source_node_id) {
                    Ok(var_name.clone())
                } else {
                    Err(GraphyError::Custom(format!("No variable for source node: {}", source_node_id)))
                }
            }
            Some(DataSource::Constant(value)) => Ok(value.clone()),
            Some(DataSource::Default) => {
                Ok("0.0".to_string()) // WGSL default
            }
            None => Err(GraphyError::Custom(format!("No data source for input: {}.{}", node_id, pin_name))),
        }
    }

    /// Generate inlined expression for a pure node
    fn generate_pure_node_expression(&self, node: &NodeInstance) -> Result<String, GraphyError> {
        let node_meta = self.metadata_provider
            .get_node_metadata(&node.node_type)
            .ok_or_else(|| GraphyError::NodeNotFound(node.node_type.clone()))?;

        // Recursively generate arguments
        let mut args = Vec::new();
        for param in &node_meta.params {
            let arg_expr = self.generate_input_expression(&node.id, &param.name)?;
            args.push(arg_expr);
        }

        Ok(format!("{}({})", self.map_function_name(&node_meta.name), args.join(", ")))
    }

    /// Map function names to WGSL built-ins
    fn map_function_name(&self, name: &str) -> String {
        match name {
            "add" => "add",
            "multiply" => "multiply",
            "dot" => "dot",
            "normalize" => "normalize",
            "vec3" => "vec3<f32>",
            "vec4" => "vec4<f32>",
            "sample_texture" => "textureSample",
            _ => name,
        }
        .to_string()
    }

    /// Clone with new visited set
    fn clone_with_new_visited(&self) -> Self {
        Self {
            graph: self.graph,
            metadata_provider: self.metadata_provider,
            data_resolver: self.data_resolver,
            exec_routing: self.exec_routing,
            stage: self.stage,
            visited: HashSet::new(),
        }
    }
}

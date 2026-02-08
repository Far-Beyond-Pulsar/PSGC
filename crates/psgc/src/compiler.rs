//! # Shader Compiler
//!
//! Main entry points for compiling shader graphs to WGSL code.

use crate::metadata::ShaderMetadataProvider;
use crate::codegen::{WGSLCodeGenerator, ShaderStage};
use graphy::{GraphDescription, GraphyError, DataResolver, ExecutionRouting};
use graphy::core::NodeMetadataProvider;

/// Compile a shader graph to WGSL code
///
/// Automatically detects the shader stage from entry nodes.
///
/// # Arguments
///
/// * `graph` - The shader graph to compile
///
/// # Returns
///
/// * `Ok(String)` - The generated WGSL source code
/// * `Err(GraphyError)` - A descriptive error if compilation fails
pub fn compile_shader(graph: &GraphDescription) -> Result<String, GraphyError> {
    // Auto-detect stage from graph
    let has_vertex = graph.nodes.values().any(|n| n.node_type == "vertex_main");
    let has_fragment = graph.nodes.values().any(|n| n.node_type == "fragment_main");

    if has_fragment {
        compile_fragment_shader(graph)
    } else if has_vertex {
        compile_vertex_shader(graph)
    } else {
        Err(GraphyError::CodeGeneration(
            "No shader entry point found (vertex_main or fragment_main)".to_string(),
        ))
    }
}

/// Compile a vertex shader
pub fn compile_vertex_shader(graph: &GraphDescription) -> Result<String, GraphyError> {
    compile_shader_with_stage(graph, ShaderStage::Vertex)
}

/// Compile a fragment shader
pub fn compile_fragment_shader(graph: &GraphDescription) -> Result<String, GraphyError> {
    compile_shader_with_stage(graph, ShaderStage::Fragment)
}

/// Compile a shader with a specific stage
fn compile_shader_with_stage(
    graph: &GraphDescription,
    stage: ShaderStage,
) -> Result<String, GraphyError> {
    tracing::info!("[PSGC] Starting shader compilation");
    tracing::info!("[PSGC] Graph: {} ({} nodes, {} connections)",
        graph.metadata.name,
        graph.nodes.len(),
        graph.connections.len());
    tracing::info!("[PSGC] Stage: {:?}", stage);

    // Phase 1: Get shader metadata
    tracing::info!("[PSGC] Phase 1: Loading shader node metadata...");
    let metadata_provider = ShaderMetadataProvider::new();
    tracing::info!("[PSGC] Loaded {} shader node types",
        metadata_provider.get_all_nodes().len());

    // Phase 2: Build data flow resolver
    tracing::info!("[PSGC] Phase 2: Analyzing data flow...");
    let data_resolver = DataResolver::build(graph, &metadata_provider)?;
    tracing::info!("[PSGC] Data flow analysis complete");

    // Phase 3: Build execution routing
    tracing::info!("[PSGC] Phase 3: Analyzing execution flow...");
    let exec_routing = ExecutionRouting::build_from_graph(graph);
    tracing::info!("[PSGC] Execution flow analysis complete");

    // Phase 4: Generate WGSL code
    tracing::info!("[PSGC] Phase 4: Generating WGSL code...");
    let code_generator = WGSLCodeGenerator::new(
        graph,
        &metadata_provider,
        &data_resolver,
        &exec_routing,
        stage,
    );
    let code = code_generator.generate_shader()?;

    tracing::info!("[PSGC] Code generation complete ({} bytes)", code.len());
    tracing::info!("[PSGC] Compilation successful!");

    Ok(code)
}

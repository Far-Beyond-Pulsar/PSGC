//! # Pulsar Shader Graph Compiler (PSGC)
//!
//! Production-ready compiler for transforming visual shader node graphs
//! into WGSL (WebGPU Shading Language) shader code.
//!
//! PSGC builds on the [graphy](https://github.com/yourusername/graphy) library,
//! providing WGSL-specific functionality including:
//! - Shader node metadata system
//! - WGSL code generation
//! - Texture sampling and uniform handling
//! - Vertex and fragment shader support
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use psgc::compile_shader;
//! use graphy::GraphDescription;
//!
//! let graph = GraphDescription::new("my_shader");
//! // ... build graph with shader nodes
//!
//! match compile_shader(&graph) {
//!     Ok(wgsl_code) => {
//!         std::fs::write("shader.wgsl", wgsl_code)?;
//!     }
//!     Err(e) => eprintln!("Compilation failed: {}", e),
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Architecture
//!
//! PSGC follows a multi-phase compilation pipeline:
//!
//! 1. **Metadata Loading** - Load shader node definitions
//! 2. **Data Flow Analysis** - Build dependency graph (via Graphy)
//! 3. **Execution Flow Analysis** - Map shader stages (via Graphy)
//! 4. **Code Generation** - Generate WGSL shader code

pub mod metadata;
pub mod codegen;
pub mod compiler;

// Re-export the main compilation API
pub use compiler::{
    compile_shader,
    compile_vertex_shader,
    compile_fragment_shader,
};
pub use codegen::ShaderStage;

// Re-export Graphy types for convenience
pub use graphy::{
    GraphDescription, NodeInstance, Connection, Pin, PinInstance,
    DataType, NodeTypes, Position, ConnectionType, PropertyValue,
    GraphMetadata, Result, GraphyError, PinType,
};

// Re-export core types
pub use graphy::core::TypeInfo;

// Re-export metadata types
pub use metadata::{
    ShaderMetadataProvider,
    get_shader_nodes,
};

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

pub mod codegen;
pub mod compiler;
pub mod metadata;

// Re-export the main compilation API
pub use codegen::ShaderStage;
pub use compiler::{compile_fragment_shader, compile_shader, compile_vertex_shader};

// Re-export Graphy types for convenience
pub use graphy::{
    Connection, ConnectionType, DataType, GraphDescription, GraphMetadata, GraphyError,
    NodeInstance, NodeTypes, Pin, PinInstance, PinType, Position, PropertyValue, Result,
};

// Re-export core types
pub use graphy::core::TypeInfo;

// Re-export metadata types
pub use metadata::{get_shader_nodes, ShaderMetadataProvider};

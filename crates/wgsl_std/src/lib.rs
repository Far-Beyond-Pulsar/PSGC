//! # WGSL Standard Library
//!
//! Standard shader node definitions for WGSL (WebGPU Shading Language).
//!
//! This crate provides a comprehensive library of shader nodes organized by category:
//! - **Math**: Basic arithmetic, trigonometry, interpolation
//! - **Vector**: Vector operations, dot/cross products, normalization
//! - **Color**: Color space conversions, blending
//! - **Texture**: Texture sampling operations
//! - **Input**: Shader inputs (position, UV, normals, etc.)
//! - **Output**: Fragment shader outputs
//!
//! ## Usage
//!
//! ```rust
//! use wgsl_std::SHADER_REGISTRY;
//!
//! // Access all registered shader nodes
//! for node_fn in SHADER_REGISTRY.iter() {
//!     let node = node_fn();
//!     println!("Node: {} ({})", node.name, node.category);
//! }
//! ```
//!
//! ## Node Registration
//!
//! Nodes are automatically registered using the `linkme` distributed slice:
//!
//! ```rust
//! use wgsl_std::SHADER_REGISTRY;
//! use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
//! use linkme::distributed_slice;
//!
//! #[distributed_slice(SHADER_REGISTRY)]
//! pub fn my_custom_node() -> NodeMetadata {
//!     NodeMetadata::new("my_node", NodeTypes::pure, "Custom")
//!         .with_params(vec![
//!             ParamInfo::new("input", "f32"),
//!         ])
//!         .with_return_type("f32")
//!         .with_source("input * 2.0")
//! }
//! ```

pub mod shader;

use graphy::core::NodeMetadata;
use linkme::distributed_slice;

/// Global registry of all shader nodes
/// 
/// This slice is populated at compile-time by the `#[distributed_slice]` attribute
#[distributed_slice]
pub static SHADER_REGISTRY: [fn() -> NodeMetadata] = [..];

/// Re-export graphy types for convenience
pub use graphy::core::{NodeTypes, ParamInfo, TypeInfo};
pub use graphy::DataType;

// Re-export shader node categories
pub use shader::nodes::*;

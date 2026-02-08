//! Shader input nodes
//!
//! Vertex shader inputs and fragment shader inputs

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Vertex Inputs
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_position() -> NodeMetadata {
    NodeMetadata::new("vertex_position", NodeTypes::pure, "Input")
        .with_return_type("vec3<f32>")
        .with_source("vertex_position")
        }

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_normal() -> NodeMetadata {
    NodeMetadata::new("vertex_normal", NodeTypes::pure, "Input")
        .with_return_type("vec3<f32>")
        .with_source("vertex_normal")
        }

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_uv() -> NodeMetadata {
    NodeMetadata::new("vertex_uv", NodeTypes::pure, "Input")
        .with_return_type("vec2<f32>")
        .with_source("vertex_uv")
        }

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_color() -> NodeMetadata {
    NodeMetadata::new("vertex_color", NodeTypes::pure, "Input")
        .with_return_type("vec4<f32>")
        .with_source("vertex_color")
        }

// ============================================================================
// Fragment Inputs
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn frag_position() -> NodeMetadata {
    NodeMetadata::new("frag_position", NodeTypes::pure, "Input")
        .with_return_type("vec4<f32>")
        .with_source("frag_position")
        }

#[distributed_slice(SHADER_REGISTRY)]
pub fn frag_uv() -> NodeMetadata {
    NodeMetadata::new("frag_uv", NodeTypes::pure, "Input")
        .with_return_type("vec2<f32>")
        .with_source("frag_uv")
        }

#[distributed_slice(SHADER_REGISTRY)]
pub fn frag_normal() -> NodeMetadata {
    NodeMetadata::new("frag_normal", NodeTypes::pure, "Input")
        .with_return_type("vec3<f32>")
        .with_source("frag_normal")
        }


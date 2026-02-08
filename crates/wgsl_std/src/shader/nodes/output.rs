//! Shader output nodes
//!
//! Output nodes for vertex and fragment shaders

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Fragment Outputs
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn fragment_output() -> NodeMetadata {
    NodeMetadata::new("fragment_output", NodeTypes::event, "Output")
        .with_params(vec![
            ParamInfo::new("color", "vec4<f32>"),
        ])
        }

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_output() -> NodeMetadata {
    NodeMetadata::new("vertex_output", NodeTypes::event, "Output")
        .with_params(vec![
            ParamInfo::new("position", "vec4<f32>"),
        ])
        }


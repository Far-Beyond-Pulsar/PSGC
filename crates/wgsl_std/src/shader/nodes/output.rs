//! Shader output nodes
//!
//! Every shader graph is a pure dataflow graph rooted at a single output
//! node (`fragment_output`/`vertex_output`). These are pure sink nodes —
//! they take the final color/position as input and have no return value or
//! side effects of their own — and PSGC's code generator builds the
//! `@vertex`/`@fragment` entry function around them.

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

#[distributed_slice(SHADER_REGISTRY)]
pub fn fragment_output() -> NodeMetadata {
    NodeMetadata::new("fragment_output", NodeTypes::pure, "Output")
        .with_params(vec![
            ParamInfo::new("color", "vec4<f32>"),
        ])
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_output() -> NodeMetadata {
    NodeMetadata::new("vertex_output", NodeTypes::pure, "Output")
        .with_params(vec![
            ParamInfo::new("position", "vec4<f32>"),
        ])
}

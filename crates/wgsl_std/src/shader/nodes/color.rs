//! Color shader nodes
//!
//! Color operations and transformations

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Color Construction
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn rgb() -> NodeMetadata {
    NodeMetadata::new("rgb", NodeTypes::pure, "Color")
        .with_params(vec![
            ParamInfo::new("r", "f32"),
            ParamInfo::new("g", "f32"),
            ParamInfo::new("b", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_source("vec3(r, g, b)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn rgba() -> NodeMetadata {
    NodeMetadata::new("rgba", NodeTypes::pure, "Color")
        .with_params(vec![
            ParamInfo::new("r", "f32"),
            ParamInfo::new("g", "f32"),
            ParamInfo::new("b", "f32"),
            ParamInfo::new("a", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("vec4(r, g, b, a)")
}

// ============================================================================
// Color Operations
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn color_lerp() -> NodeMetadata {
    NodeMetadata::new("color_lerp", NodeTypes::pure, "Color")
        .with_params(vec![
            ParamInfo::new("a", "vec4<f32>"),
            ParamInfo::new("b", "vec4<f32>"),
            ParamInfo::new("t", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("mix(a, b, t)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn color_multiply() -> NodeMetadata {
    NodeMetadata::new("color_multiply", NodeTypes::pure, "Color")
        .with_params(vec![
            ParamInfo::new("color", "vec4<f32>"),
            ParamInfo::new("factor", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("color * factor")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn color_add() -> NodeMetadata {
    NodeMetadata::new("color_add", NodeTypes::pure, "Color")
        .with_params(vec![
            ParamInfo::new("a", "vec4<f32>"),
            ParamInfo::new("b", "vec4<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("a + b")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn desaturate() -> NodeMetadata {
    NodeMetadata::new("desaturate", NodeTypes::pure, "Color")
        .with_params(vec![
            ParamInfo::new("color", "vec3<f32>"),
            ParamInfo::new("amount", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_source(
            "mix(color, vec3(dot(color, vec3(0.299, 0.587, 0.114))), amount)"
        )
}


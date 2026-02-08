//! Vector shader nodes
//!
//! Vector operations for 2D, 3D, and 4D vectors

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Vector Construction
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec2_construct() -> NodeMetadata {
    NodeMetadata::new("vec2", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
        ])
        .with_return_type("vec2<f32>")
        .with_source("vec2(x, y)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_construct() -> NodeMetadata {
    NodeMetadata::new("vec3", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
            ParamInfo::new("z", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_source("vec3(x, y, z)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec4_construct() -> NodeMetadata {
    NodeMetadata::new("vec4", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
            ParamInfo::new("z", "f32"),
            ParamInfo::new("w", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("vec4(x, y, z, w)")
}

// ============================================================================
// Vector Operations
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_normalize() -> NodeMetadata {
    NodeMetadata::new("normalize", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("vec3<f32>")
        .with_source("normalize(v)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_dot() -> NodeMetadata {
    NodeMetadata::new("dot", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("a", "vec3<f32>"),
            ParamInfo::new("b", "vec3<f32>"),
        ])
        .with_return_type("f32")
        .with_source("dot(a, b)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_cross() -> NodeMetadata {
    NodeMetadata::new("cross", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("a", "vec3<f32>"),
            ParamInfo::new("b", "vec3<f32>"),
        ])
        .with_return_type("vec3<f32>")
        .with_source("cross(a, b)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_length() -> NodeMetadata {
    NodeMetadata::new("length", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("f32")
        .with_source("length(v)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_distance() -> NodeMetadata {
    NodeMetadata::new("distance", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("a", "vec3<f32>"),
            ParamInfo::new("b", "vec3<f32>"),
        ])
        .with_return_type("f32")
        .with_source("distance(a, b)")
}

// ============================================================================
// Component Access
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_split() -> NodeMetadata {
    NodeMetadata::new("vec3_split", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("vec3<f32>")
        .with_source("v")
}

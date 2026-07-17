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
// Make nodes — combine scalars into vectors
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn make_vec2() -> NodeMetadata {
    NodeMetadata::new("make_vec2", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
        ])
        .with_return_type("vec2<f32>")
        .with_source("vec2<f32>(x, y)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn make_vec3() -> NodeMetadata {
    NodeMetadata::new("make_vec3", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
            ParamInfo::new("z", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_source("vec3<f32>(x, y, z)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn make_vec4() -> NodeMetadata {
    NodeMetadata::new("make_vec4", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
            ParamInfo::new("z", "f32"),
            ParamInfo::new("w", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("vec4<f32>(x, y, z, w)")
}

// ============================================================================
// Break nodes — multi-output component extraction
// ============================================================================
//
// Each Break node stores the whole input vector in a single `let` binding.
// Downstream consumers reference individual components via the output pin's
// accessor string (e.g. `.r`, `.g`), which the codegen appends at the call
// site — the Break node itself produces exactly one variable.
//
//   let N_result = v;          // single binding
//   … = N_result.r * 2.0;      // consumer appends .r

#[distributed_slice(SHADER_REGISTRY)]
pub fn break_vec2() -> NodeMetadata {
    NodeMetadata::new("break_vec2", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec2<f32>")])
        .with_return_type("vec2<f32>")
        .with_source("v")
        .with_outputs(vec![
            graphy::core::OutputParam::new("r", "f32", ".r"),
            graphy::core::OutputParam::new("g", "f32", ".g"),
        ])
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn break_vec3() -> NodeMetadata {
    NodeMetadata::new("break_vec3", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("vec3<f32>")
        .with_source("v")
        .with_outputs(vec![
            graphy::core::OutputParam::new("r", "f32", ".r"),
            graphy::core::OutputParam::new("g", "f32", ".g"),
            graphy::core::OutputParam::new("b", "f32", ".b"),
        ])
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn break_vec4() -> NodeMetadata {
    NodeMetadata::new("break_vec4", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec4<f32>")])
        .with_return_type("vec4<f32>")
        .with_source("v")
        .with_outputs(vec![
            graphy::core::OutputParam::new("r", "f32", ".r"),
            graphy::core::OutputParam::new("g", "f32", ".g"),
            graphy::core::OutputParam::new("b", "f32", ".b"),
            graphy::core::OutputParam::new("a", "f32", ".a"),
        ])
}

// ============================================================================
// Legacy component access (kept for backward compatibility)
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_split() -> NodeMetadata {
    NodeMetadata::new("vec3_split", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("vec3<f32>")
        .with_source("v")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_x() -> NodeMetadata {
    NodeMetadata::new("vec3_x", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("f32")
        .with_source("v.x")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_y() -> NodeMetadata {
    NodeMetadata::new("vec3_y", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("f32")
        .with_source("v.y")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_z() -> NodeMetadata {
    NodeMetadata::new("vec3_z", NodeTypes::pure, "Vector")
        .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
        .with_return_type("f32")
        .with_source("v.z")
}

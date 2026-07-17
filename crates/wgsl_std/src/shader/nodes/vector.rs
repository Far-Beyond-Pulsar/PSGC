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
// Break nodes — extract individual components from vectors
// ============================================================================

const PN_BREAK_VEC2: (&str, &str) = ("pn_break_vec2", r#"fn pn_break_vec2(v: vec2<f32>, i: i32) -> f32 {
    return select(v.g, v.r, i == 0);
}"#);

const PN_BREAK_VEC3: (&str, &str) = ("pn_break_vec3", r#"fn pn_break_vec3(v: vec3<f32>, i: i32) -> f32 {
    return select(select(v.b, v.g, i == 1), v.r, i == 0);
}"#);

const PN_BREAK_VEC4: (&str, &str) = ("pn_break_vec4", r#"fn pn_break_vec4(v: vec4<f32>, i: i32) -> f32 {
    return select(select(v.a, v.b, i == 2), select(v.g, v.r, i == 0), i == 1);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn break_vec2() -> NodeMetadata {
    NodeMetadata::new("break_vec2", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("v", "vec2<f32>"),
            ParamInfo::new("index", "i32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_BREAK_VEC2])
        .with_source("pn_break_vec2(v, index)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn break_vec3() -> NodeMetadata {
    NodeMetadata::new("break_vec3", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("v", "vec3<f32>"),
            ParamInfo::new("index", "i32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_BREAK_VEC3])
        .with_source("pn_break_vec3(v, index)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn break_vec4() -> NodeMetadata {
    NodeMetadata::new("break_vec4", NodeTypes::pure, "Vector")
        .with_params(vec![
            ParamInfo::new("v", "vec4<f32>"),
            ParamInfo::new("index", "i32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_BREAK_VEC4])
        .with_source("pn_break_vec4(v, index)")
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

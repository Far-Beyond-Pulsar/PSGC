//! Utility nodes — constants, component access, branching

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Constant literals
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn constant_f32() -> NodeMetadata {
    NodeMetadata::new("constant_f32", NodeTypes::pure, "Utility")
        .with_params(vec![ParamInfo::new("value", "f32")])
        .with_return_type("f32")
        .with_source("value")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn constant_vec2() -> NodeMetadata {
    NodeMetadata::new("constant_vec2", NodeTypes::pure, "Utility")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
        ])
        .with_return_type("vec2<f32>")
        .with_source("vec2<f32>(x, y)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn constant_vec3() -> NodeMetadata {
    NodeMetadata::new("constant_vec3", NodeTypes::pure, "Utility")
        .with_params(vec![
            ParamInfo::new("x", "f32"),
            ParamInfo::new("y", "f32"),
            ParamInfo::new("z", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_source("vec3<f32>(x, y, z)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn constant_vec4() -> NodeMetadata {
    NodeMetadata::new("constant_vec4", NodeTypes::pure, "Utility")
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
// Component extraction (vec4 -> f32)
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn component_r() -> NodeMetadata {
    NodeMetadata::new("component_r", NodeTypes::pure, "Utility")
        .with_params(vec![ParamInfo::new("vec", "vec4<f32>")])
        .with_return_type("f32")
        .with_source("vec.r")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn component_g() -> NodeMetadata {
    NodeMetadata::new("component_g", NodeTypes::pure, "Utility")
        .with_params(vec![ParamInfo::new("vec", "vec4<f32>")])
        .with_return_type("f32")
        .with_source("vec.g")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn component_b() -> NodeMetadata {
    NodeMetadata::new("component_b", NodeTypes::pure, "Utility")
        .with_params(vec![ParamInfo::new("vec", "vec4<f32>")])
        .with_return_type("f32")
        .with_source("vec.b")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn component_a() -> NodeMetadata {
    NodeMetadata::new("component_a", NodeTypes::pure, "Utility")
        .with_params(vec![ParamInfo::new("vec", "vec4<f32>")])
        .with_return_type("f32")
        .with_source("vec.a")
}

// ============================================================================
// Conditional
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn if_ternary() -> NodeMetadata {
    NodeMetadata::new("if_ternary", NodeTypes::pure, "Utility")
        .with_params(vec![
            ParamInfo::new("condition", "bool"),
            ParamInfo::new("true_val", "f32"),
            ParamInfo::new("false_val", "f32"),
        ])
        .with_return_type("f32")
        .with_source("select(false_val, true_val, condition)")
}

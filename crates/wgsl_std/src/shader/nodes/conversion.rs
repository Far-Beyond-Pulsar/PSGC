//! Type-conversion shader nodes
//!
//! Each node carries `.with_conversion(from, to, lossless)` metadata so that
//! graph editors and compilers can auto-insert them when a connection between
//! incompatible types has a valid conversion path.

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo, TypeInfo};
use linkme::distributed_slice;

// ============================================================================
// Vector dimensionality conversions
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec2_to_vec3() -> NodeMetadata {
    NodeMetadata::new("vec2_to_vec3", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec2<f32>")])
        .with_return_type("vec3<f32>")
        .with_source("vec3<f32>(input, 0.0)")
        .with_conversion(TypeInfo::new("vec2<f32>"), TypeInfo::new("vec3<f32>"), true)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec2_to_vec4() -> NodeMetadata {
    NodeMetadata::new("vec2_to_vec4", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec2<f32>")])
        .with_return_type("vec4<f32>")
        .with_source("vec4<f32>(input, 0.0, 1.0)")
        .with_conversion(TypeInfo::new("vec2<f32>"), TypeInfo::new("vec4<f32>"), true)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_to_vec4() -> NodeMetadata {
    NodeMetadata::new("vec3_to_vec4", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec3<f32>")])
        .with_return_type("vec4<f32>")
        .with_source("vec4<f32>(input, 1.0)")
        .with_conversion(TypeInfo::new("vec3<f32>"), TypeInfo::new("vec4<f32>"), true)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec4_to_vec3() -> NodeMetadata {
    NodeMetadata::new("vec4_to_vec3", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec4<f32>")])
        .with_return_type("vec3<f32>")
        .with_source("input.rgb")
        .with_conversion(TypeInfo::new("vec4<f32>"), TypeInfo::new("vec3<f32>"), false)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_to_vec2() -> NodeMetadata {
    NodeMetadata::new("vec3_to_vec2", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec3<f32>")])
        .with_return_type("vec2<f32>")
        .with_source("input.xy")
        .with_conversion(TypeInfo::new("vec3<f32>"), TypeInfo::new("vec2<f32>"), false)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec4_to_vec2() -> NodeMetadata {
    NodeMetadata::new("vec4_to_vec2", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec4<f32>")])
        .with_return_type("vec2<f32>")
        .with_source("input.xy")
        .with_conversion(TypeInfo::new("vec4<f32>"), TypeInfo::new("vec2<f32>"), false)
}

// ============================================================================
// Scalar ↔ vector conversions
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn f32_to_vec2() -> NodeMetadata {
    NodeMetadata::new("f32_to_vec2", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "f32")])
        .with_return_type("vec2<f32>")
        .with_source("vec2<f32>(input)")
        .with_conversion(TypeInfo::new("f32"), TypeInfo::new("vec2<f32>"), true)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn f32_to_vec3() -> NodeMetadata {
    NodeMetadata::new("f32_to_vec3", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "f32")])
        .with_return_type("vec3<f32>")
        .with_source("vec3<f32>(input)")
        .with_conversion(TypeInfo::new("f32"), TypeInfo::new("vec3<f32>"), true)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn f32_to_vec4() -> NodeMetadata {
    NodeMetadata::new("f32_to_vec4", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "f32")])
        .with_return_type("vec4<f32>")
        .with_source("vec4<f32>(input)")
        .with_conversion(TypeInfo::new("f32"), TypeInfo::new("vec4<f32>"), true)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec2_to_f32() -> NodeMetadata {
    NodeMetadata::new("vec2_to_f32", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec2<f32>")])
        .with_return_type("f32")
        .with_source("input.r")
        .with_conversion(TypeInfo::new("vec2<f32>"), TypeInfo::new("f32"), false)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec3_to_f32() -> NodeMetadata {
    NodeMetadata::new("vec3_to_f32", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec3<f32>")])
        .with_return_type("f32")
        .with_source("input.r")
        .with_conversion(TypeInfo::new("vec3<f32>"), TypeInfo::new("f32"), false)
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vec4_to_f32() -> NodeMetadata {
    NodeMetadata::new("vec4_to_f32", NodeTypes::pure, "Conversion")
        .with_params(vec![ParamInfo::new("input", "vec4<f32>")])
        .with_return_type("f32")
        .with_source("input.r")
        .with_conversion(TypeInfo::new("vec4<f32>"), TypeInfo::new("f32"), false)
}

//! Texture sampling nodes
//!
//! Texture operations for WGSL shaders

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Texture Sampling
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn sample_texture() -> NodeMetadata {
    NodeMetadata::new("sample_texture", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("texture", "texture_2d<f32>"),
            ParamInfo::new("sampler", "sampler"),
            ParamInfo::new("uv", "vec2<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("textureSample(texture, sampler, uv)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn sample_texture_level() -> NodeMetadata {
    NodeMetadata::new("sample_texture_level", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("texture", "texture_2d<f32>"),
            ParamInfo::new("sampler", "sampler"),
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("level", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("textureSampleLevel(texture, sampler, uv, level)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn sample_texture_grad() -> NodeMetadata {
    NodeMetadata::new("sample_texture_grad", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("texture", "texture_2d<f32>"),
            ParamInfo::new("sampler", "sampler"),
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("ddx", "vec2<f32>"),
            ParamInfo::new("ddy", "vec2<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("textureSampleGrad(texture, sampler, uv, ddx, ddy)")
}


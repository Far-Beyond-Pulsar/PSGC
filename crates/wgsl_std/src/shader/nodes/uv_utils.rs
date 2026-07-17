//! UV utility nodes — panner, rotator, and related coordinate transforms

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

#[distributed_slice(SHADER_REGISTRY)]
pub fn panner() -> NodeMetadata {
    NodeMetadata::new("panner", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("speed", "vec2<f32>"),
            ParamInfo::new("time", "f32"),
        ])
        .with_return_type("vec2<f32>")
        .with_source("uv + speed * time")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn rotator() -> NodeMetadata {
    NodeMetadata::new("rotator", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("center", "vec2<f32>"),
            ParamInfo::new("angle", "f32"),
        ])
        .with_return_type("vec2<f32>")
        .with_source("let c = cos(angle); let s = sin(angle); let d = uv - center; center + vec2<f32>(d.x * c - d.y * s, d.x * s + d.y * c)")
}

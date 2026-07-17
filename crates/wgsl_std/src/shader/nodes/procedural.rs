//! Procedural pattern generation nodes
//!
//! Pure-WGSL procedural texture patterns (no sampling required).

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

const PN_CHECKERBOARD: (&str, &str) = ("pn_checkerboard", r#"fn pn_checkerboard(uv: vec2<f32>, scale: f32, color_a: vec4<f32>, color_b: vec4<f32>) -> vec4<f32> {
    let i = i32(floor(uv.x * scale));
    let j = i32(floor(uv.y * scale));
    return select(color_b, color_a, (i + j) % 2 == 1);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn checkerboard() -> NodeMetadata {
    NodeMetadata::new("checkerboard", NodeTypes::pure, "Procedural")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("color_a", "vec4<f32>"),
            ParamInfo::new("color_b", "vec4<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_CHECKERBOARD])
        .with_source("pn_checkerboard(uv, scale, color_a, color_b)")
}

const PN_GRADIENT_LINEAR: (&str, &str) = ("pn_gradient_linear", r#"fn pn_gradient_linear(uv: vec2<f32>, angle: f32, color_a: vec4<f32>, color_b: vec4<f32>) -> vec4<f32> {
    let d = dot(uv, vec2<f32>(cos(angle), sin(angle)));
    return mix(color_a, color_b, clamp(d, 0.0, 1.0));
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn gradient_linear() -> NodeMetadata {
    NodeMetadata::new("gradient_linear", NodeTypes::pure, "Procedural")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("angle", "f32"),
            ParamInfo::new("color_a", "vec4<f32>"),
            ParamInfo::new("color_b", "vec4<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_GRADIENT_LINEAR])
        .with_source("pn_gradient_linear(uv, angle, color_a, color_b)")
}

const PN_GRADIENT_RADIAL: (&str, &str) = ("pn_gradient_radial", r#"fn pn_gradient_radial(uv: vec2<f32>, center: vec2<f32>, radius: f32, color_a: vec4<f32>, color_b: vec4<f32>) -> vec4<f32> {
    let d = distance(uv, center) / radius;
    return mix(color_a, color_b, clamp(d, 0.0, 1.0));
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn gradient_radial() -> NodeMetadata {
    NodeMetadata::new("gradient_radial", NodeTypes::pure, "Procedural")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("center", "vec2<f32>"),
            ParamInfo::new("radius", "f32"),
            ParamInfo::new("color_a", "vec4<f32>"),
            ParamInfo::new("color_b", "vec4<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_GRADIENT_RADIAL])
        .with_source("pn_gradient_radial(uv, center, radius, color_a, color_b)")
}

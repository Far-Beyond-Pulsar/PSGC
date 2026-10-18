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
            ParamInfo::new("texture", "TextureSrc"),
            ParamInfo::new("uv", "vec2<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("textureSample(texture, texture_sampler, uv)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn sample_texture_level() -> NodeMetadata {
    NodeMetadata::new("sample_texture_level", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("texture", "TextureSrc"),
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("level", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("textureSampleLevel(texture, texture_sampler, uv, level)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn sample_texture_grad() -> NodeMetadata {
    NodeMetadata::new("sample_texture_grad", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("texture", "TextureSrc"),
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("ddx", "vec2<f32>"),
            ParamInfo::new("ddy", "vec2<f32>"),
        ])
        .with_return_type("vec4<f32>")
        .with_source("textureSampleGrad(texture, texture_sampler, uv, ddx, ddy)")
}

// ============================================================================
// Normal mapping
// ============================================================================

/// Tangent-space normal map decode.
///
/// The fragment entry has no vertex tangents, so the tangent frame comes from
/// screen-space derivatives of the world position and the UVs (Schueler's
/// cotangent frame). The texel's rgb is a tangent-space vector in `[0, 1]`;
/// the result is the world-space unit normal the `normal` output pin takes.
pub(crate) const PN_NORMAL_MAP: (&str, &str) = (
    "pn_normal_map",
    r#"fn pn_normal_map(texel: vec4<f32>, uv: vec2<f32>, n: vec3<f32>, p: vec3<f32>) -> vec3<f32> {
    let tangent_normal = texel.xyz * 2.0 - vec3<f32>(1.0, 1.0, 1.0);
    let dp1 = dpdx(p);
    let dp2 = dpdy(p);
    let duv1 = dpdx(uv);
    let duv2 = dpdy(uv);
    let dp2perp = cross(dp2, n);
    let dp1perp = cross(n, dp1);
    let t = dp2perp * duv1.x + dp1perp * duv2.x;
    let b = dp2perp * duv1.y + dp1perp * duv2.y;
    let scale = inverseSqrt(max(max(dot(t, t), dot(b, b)), 1.0e-20));
    return normalize(t * scale * tangent_normal.x + b * scale * tangent_normal.y + n * tangent_normal.z);
}"#,
);

/// A sampled tangent-space normal map as a world-space normal (`vec3<f32>`),
/// ready for a `fragment_output` `normal` pin. Wire a `sample_texture` result
/// to `texel`, `frag_uv` to `uv`, `frag_normal` to `normal` and
/// `world_position` to `position`.
#[distributed_slice(SHADER_REGISTRY)]
pub fn normal_map() -> NodeMetadata {
    NodeMetadata::new("normal_map", NodeTypes::pure, "Texture")
        .with_params(vec![
            ParamInfo::new("texel", "vec4<f32>"),
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("normal", "vec3<f32>"),
            ParamInfo::new("position", "vec3<f32>"),
        ])
        .with_return_type("vec3<f32>")
        .with_helpers(&[PN_NORMAL_MAP])
        .with_source("pn_normal_map(texel, uv, normal, position)")
}

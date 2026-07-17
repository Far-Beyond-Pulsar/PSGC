//! Texture-equivalent noise nodes — each wraps a scalar/vector noise function
//! into a `vec4<f32>` RGBA output suitable for procedural texture generation.
//!
//! The `uv` parameter replaces the generic `p` position of the base noise
//! nodes.  Each noise type produces a distinct visual pattern:
//!
//! | Noise type | Pattern | Mapping |
//! |---|---|---|
//! | White / Value / Ridged | Grayscale | `(n, n, n, 1.0)` |
//! | Perlin | Marble | `sin(x·freq + n·amp)` modulated |
//! | Simplex | Organic grayscale | `(n, n, n, 1.0)` (isotropic) |
//! | Voronoi | Cell visualisation | `(F1, F2, cell_random, 1.0)` |
//! | FBM | Warm clouds | `(n, n·0.8, n·0.6, 1.0)` |
//! | Turbulence | Fire / plasma | `(n·2, n·1.2, n·0.4, 1.0)` |

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;
use super::noise::*;

// ============================================================================
// White noise texture — grayscale static
// ============================================================================

const PN_WHITE_TEXTURE_2D: (&str, &str) = ("pn_white_texture_2d", r#"fn pn_white_texture_2d(uv: vec2<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_white_2d(uv, scale, seed);
    return vec4<f32>(n, n, n, 1.0);
}"#);

const PN_WHITE_TEXTURE_3D: (&str, &str) = ("pn_white_texture_3d", r#"fn pn_white_texture_3d(uv: vec3<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_white_3d(uv, scale, seed);
    return vec4<f32>(n, n, n, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn white_noise_texture_2d() -> NodeMetadata {
    NodeMetadata::new("white_noise_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_WHITE_2D, PN_WHITE_TEXTURE_2D])
        .with_source("pn_white_texture_2d(uv, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn white_noise_texture_3d() -> NodeMetadata {
    NodeMetadata::new("white_noise_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH31, PN_WHITE_3D, PN_WHITE_TEXTURE_3D])
        .with_source("pn_white_texture_3d(uv, scale, seed)")
}

// ============================================================================
// Value noise texture — smooth grayscale
// ============================================================================

const PN_VALUE_TEXTURE_2D: (&str, &str) = ("pn_value_texture_2d", r#"fn pn_value_texture_2d(uv: vec2<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_value_2d(uv, scale, seed);
    return vec4<f32>(n, n, n, 1.0);
}"#);

const PN_VALUE_TEXTURE_3D: (&str, &str) = ("pn_value_texture_3d", r#"fn pn_value_texture_3d(uv: vec3<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_value_3d(uv, scale, seed);
    return vec4<f32>(n, n, n, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn value_noise_texture_2d() -> NodeMetadata {
    NodeMetadata::new("value_noise_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_VALUE_2D, PN_VALUE_TEXTURE_2D])
        .with_source("pn_value_texture_2d(uv, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn value_noise_texture_3d() -> NodeMetadata {
    NodeMetadata::new("value_noise_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH31, PN_VALUE_3D, PN_VALUE_TEXTURE_3D])
        .with_source("pn_value_texture_3d(uv, scale, seed)")
}

// ============================================================================
// Perlin noise texture — classic marble pattern
// ============================================================================

const PN_PERLIN_TEXTURE_2D: (&str, &str) = ("pn_perlin_texture_2d", r#"fn pn_perlin_texture_2d(uv: vec2<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_perlin_2d(uv, scale, seed);
    let marble = sin(uv.x * 6.28318530718 + n * 3.0) * 0.5 + 0.5;
    return vec4<f32>(marble, marble, marble, 1.0);
}"#);

const PN_PERLIN_TEXTURE_3D: (&str, &str) = ("pn_perlin_texture_3d", r#"fn pn_perlin_texture_3d(uv: vec3<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_perlin_3d(uv, scale, seed);
    let marble = sin(uv.x * 6.28318530718 + n * 3.0) * 0.5 + 0.5;
    return vec4<f32>(marble, marble, marble, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn perlin_texture_2d() -> NodeMetadata {
    NodeMetadata::new("perlin_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_PERLIN_2D, PN_PERLIN_TEXTURE_2D])
        .with_source("pn_perlin_texture_2d(uv, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn perlin_texture_3d() -> NodeMetadata {
    NodeMetadata::new("perlin_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_PERLIN_3D, PN_PERLIN_TEXTURE_3D])
        .with_source("pn_perlin_texture_3d(uv, scale, seed)")
}

// ============================================================================
// Simplex noise texture — organic grayscale
// ============================================================================

const PN_SIMPLEX_TEXTURE_2D: (&str, &str) = ("pn_simplex_texture_2d", r#"fn pn_simplex_texture_2d(uv: vec2<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_simplex_2d(uv, scale, seed);
    return vec4<f32>(n, n, n, 1.0);
}"#);

const PN_SIMPLEX_TEXTURE_3D: (&str, &str) = ("pn_simplex_texture_3d", r#"fn pn_simplex_texture_3d(uv: vec3<f32>, scale: f32, seed: f32) -> vec4<f32> {
    let n = pn_simplex_3d(uv, scale, seed);
    return vec4<f32>(n, n, n, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn simplex_texture_2d() -> NodeMetadata {
    NodeMetadata::new("simplex_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_SIMPLEX_2D, PN_SIMPLEX_TEXTURE_2D])
        .with_source("pn_simplex_texture_2d(uv, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn simplex_texture_3d() -> NodeMetadata {
    NodeMetadata::new("simplex_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_SIMPLEX_3D, PN_SIMPLEX_TEXTURE_3D])
        .with_source("pn_simplex_texture_3d(uv, scale, seed)")
}

// ============================================================================
// Voronoi noise texture — cell visualisation (F1, F2, cell_random)
// ============================================================================

const PN_VORONOI_TEXTURE_2D: (&str, &str) = ("pn_voronoi_texture_2d", r#"fn pn_voronoi_texture_2d(uv: vec2<f32>, scale: f32, seed: f32) -> vec4<f32> {
    return vec4<f32>(pn_voronoi_2d(uv, scale, seed), 1.0);
}"#);

const PN_VORONOI_TEXTURE_3D: (&str, &str) = ("pn_voronoi_texture_3d", r#"fn pn_voronoi_texture_3d(uv: vec3<f32>, scale: f32, seed: f32) -> vec4<f32> {
    return vec4<f32>(pn_voronoi_3d(uv, scale, seed), 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn voronoi_texture_2d() -> NodeMetadata {
    NodeMetadata::new("voronoi_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_HASH22, PN_VORONOI_2D, PN_VORONOI_TEXTURE_2D])
        .with_source("pn_voronoi_texture_2d(uv, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn voronoi_texture_3d() -> NodeMetadata {
    NodeMetadata::new("voronoi_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH31, PN_HASH33, PN_VORONOI_3D, PN_VORONOI_TEXTURE_3D])
        .with_source("pn_voronoi_texture_3d(uv, scale, seed)")
}

// ============================================================================
// FBM texture — warm clouds (cream/brown tint)
// ============================================================================

const PN_FBM_TEXTURE_2D: (&str, &str) = ("pn_fbm_texture_2d", r#"fn pn_fbm_texture_2d(uv: vec2<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> vec4<f32> {
    let n = pn_fbm_2d(uv, scale, seed, octaves, lacunarity, gain);
    return vec4<f32>(n, n * 0.8, n * 0.6, 1.0);
}"#);

const PN_FBM_TEXTURE_3D: (&str, &str) = ("pn_fbm_texture_3d", r#"fn pn_fbm_texture_3d(uv: vec3<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> vec4<f32> {
    let n = pn_fbm_3d(uv, scale, seed, octaves, lacunarity, gain);
    return vec4<f32>(n, n * 0.8, n * 0.6, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn fbm_texture_2d() -> NodeMetadata {
    NodeMetadata::new("fbm_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_FBM_2D, PN_FBM_TEXTURE_2D])
        .with_source("pn_fbm_texture_2d(uv, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn fbm_texture_3d() -> NodeMetadata {
    NodeMetadata::new("fbm_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_FBM_3D, PN_FBM_TEXTURE_3D])
        .with_source("pn_fbm_texture_3d(uv, scale, seed, octaves, lacunarity, gain)")
}

// ============================================================================
// Turbulence texture — fire / plasma tint
// ============================================================================

const PN_TURBULENCE_TEXTURE_2D: (&str, &str) = ("pn_turbulence_texture_2d", r#"fn pn_turbulence_texture_2d(uv: vec2<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> vec4<f32> {
    let n = pn_turbulence_2d(uv, scale, seed, octaves, lacunarity, gain);
    return vec4<f32>(n * 2.0, n * 1.2, n * 0.4, 1.0);
}"#);

const PN_TURBULENCE_TEXTURE_3D: (&str, &str) = ("pn_turbulence_texture_3d", r#"fn pn_turbulence_texture_3d(uv: vec3<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> vec4<f32> {
    let n = pn_turbulence_3d(uv, scale, seed, octaves, lacunarity, gain);
    return vec4<f32>(n * 2.0, n * 1.2, n * 0.4, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn turbulence_texture_2d() -> NodeMetadata {
    NodeMetadata::new("turbulence_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_TURBULENCE_2D, PN_TURBULENCE_TEXTURE_2D])
        .with_source("pn_turbulence_texture_2d(uv, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn turbulence_texture_3d() -> NodeMetadata {
    NodeMetadata::new("turbulence_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_TURBULENCE_3D, PN_TURBULENCE_TEXTURE_3D])
        .with_source("pn_turbulence_texture_3d(uv, scale, seed, octaves, lacunarity, gain)")
}

// ============================================================================
// Ridged noise texture — grayscale terrain / veins
// ============================================================================

const PN_RIDGED_TEXTURE_2D: (&str, &str) = ("pn_ridged_texture_2d", r#"fn pn_ridged_texture_2d(uv: vec2<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> vec4<f32> {
    let n = pn_ridged_2d(uv, scale, seed, octaves, lacunarity, gain);
    return vec4<f32>(n, n, n, 1.0);
}"#);

const PN_RIDGED_TEXTURE_3D: (&str, &str) = ("pn_ridged_texture_3d", r#"fn pn_ridged_texture_3d(uv: vec3<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> vec4<f32> {
    let n = pn_ridged_3d(uv, scale, seed, octaves, lacunarity, gain);
    return vec4<f32>(n, n, n, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn ridged_texture_2d() -> NodeMetadata {
    NodeMetadata::new("ridged_texture_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_RIDGED_2D, PN_RIDGED_TEXTURE_2D])
        .with_source("pn_ridged_texture_2d(uv, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn ridged_texture_3d() -> NodeMetadata {
    NodeMetadata::new("ridged_texture_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("uv", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("vec4<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_RIDGED_3D, PN_RIDGED_TEXTURE_3D])
        .with_source("pn_ridged_texture_3d(uv, scale, seed, octaves, lacunarity, gain)")
}

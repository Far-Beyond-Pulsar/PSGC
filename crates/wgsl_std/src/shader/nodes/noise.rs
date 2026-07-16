//! Noise generation shader nodes (Pulsar-Native#81)
//!
//! Hash-based procedural noise: white, value, Perlin, simplex, Voronoi, and
//! fractal combinators. All helpers use the `pn_` prefix and PCG hashes
//! (Jarzynski & Olano) — no sin()-based hashing. Scalar outputs are in
//! [0, 1]; Voronoi returns vec3(F1, F2, cell_random).

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Shared WGSL helpers (deduplicated by name at codegen)
// ============================================================================

pub(crate) const PN_PCG2D: (&str, &str) = ("pn_pcg2d", r#"fn pn_pcg2d(p: vec2<u32>) -> vec2<u32> {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * 1664525u;
    v.y += v.x * 1664525u;
    v = v ^ (v >> vec2<u32>(16u));
    v.x += v.y * 1664525u;
    v.y += v.x * 1664525u;
    v = v ^ (v >> vec2<u32>(16u));
    return v;
}"#);

pub(crate) const PN_PCG3D: (&str, &str) = ("pn_pcg3d", r#"fn pn_pcg3d(p: vec3<u32>) -> vec3<u32> {
    var v = p * 1664525u + 1013904223u;
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    v = v ^ (v >> vec3<u32>(16u));
    v.x += v.y * v.z;
    v.y += v.z * v.x;
    v.z += v.x * v.y;
    return v;
}"#);

pub(crate) const PN_HASH21: (&str, &str) = ("pn_hash21", r#"fn pn_hash21(p: vec2<f32>) -> f32 {
    let u = pn_pcg2d(bitcast<vec2<u32>>(vec2<i32>(floor(p))));
    return f32(u.x) * (1.0 / 4294967296.0);
}"#);

pub(crate) const PN_HASH22: (&str, &str) = ("pn_hash22", r#"fn pn_hash22(p: vec2<f32>) -> vec2<f32> {
    let u = pn_pcg2d(bitcast<vec2<u32>>(vec2<i32>(floor(p))));
    return vec2<f32>(u) * (1.0 / 4294967296.0);
}"#);

pub(crate) const PN_HASH31: (&str, &str) = ("pn_hash31", r#"fn pn_hash31(p: vec3<f32>) -> f32 {
    let u = pn_pcg3d(bitcast<vec3<u32>>(vec3<i32>(floor(p))));
    return f32(u.x) * (1.0 / 4294967296.0);
}"#);

pub(crate) const PN_HASH33: (&str, &str) = ("pn_hash33", r#"fn pn_hash33(p: vec3<f32>) -> vec3<f32> {
    let u = pn_pcg3d(bitcast<vec3<u32>>(vec3<i32>(floor(p))));
    return vec3<f32>(u) * (1.0 / 4294967296.0);
}"#);

pub(crate) const PN_GRAD2: (&str, &str) = ("pn_grad2", r#"fn pn_grad2(ip: vec2<f32>) -> vec2<f32> {
    let a = pn_hash21(ip) * 6.28318530718;
    return vec2<f32>(cos(a), sin(a));
}"#);

pub(crate) const PN_GRAD3: (&str, &str) = ("pn_grad3", r#"fn pn_grad3(ip: vec3<f32>) -> vec3<f32> {
    let h = pn_hash33(ip);
    let z = h.x * 2.0 - 1.0;
    let r = sqrt(max(1.0 - z * z, 0.0));
    let a = h.y * 6.28318530718;
    return vec3<f32>(r * cos(a), r * sin(a), z);
}"#);

// ============================================================================
// White noise
// ============================================================================

const PN_WHITE_2D: (&str, &str) = ("pn_white_2d", r#"fn pn_white_2d(p: vec2<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    return pn_hash21(floor(q));
}"#);

const PN_WHITE_3D: (&str, &str) = ("pn_white_3d", r#"fn pn_white_3d(p: vec3<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    return pn_hash31(floor(q));
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn white_noise_2d() -> NodeMetadata {
    NodeMetadata::new("white_noise_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_WHITE_2D])
        .with_source("pn_white_2d(p, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn white_noise_3d() -> NodeMetadata {
    NodeMetadata::new("white_noise_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH31, PN_WHITE_3D])
        .with_source("pn_white_3d(p, scale, seed)")
}

// ============================================================================
// Value noise (quintic-interpolated lattice)
// ============================================================================

const PN_VALUE_2D: (&str, &str) = ("pn_value_2d", r#"fn pn_value_2d(p: vec2<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    let i = floor(q);
    let f = fract(q);
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let a = pn_hash21(i);
    let b = pn_hash21(i + vec2<f32>(1.0, 0.0));
    let c = pn_hash21(i + vec2<f32>(0.0, 1.0));
    let d = pn_hash21(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}"#);

const PN_VALUE_3D: (&str, &str) = ("pn_value_3d", r#"fn pn_value_3d(p: vec3<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    let i = floor(q);
    let f = fract(q);
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let n000 = pn_hash31(i);
    let n100 = pn_hash31(i + vec3<f32>(1.0, 0.0, 0.0));
    let n010 = pn_hash31(i + vec3<f32>(0.0, 1.0, 0.0));
    let n110 = pn_hash31(i + vec3<f32>(1.0, 1.0, 0.0));
    let n001 = pn_hash31(i + vec3<f32>(0.0, 0.0, 1.0));
    let n101 = pn_hash31(i + vec3<f32>(1.0, 0.0, 1.0));
    let n011 = pn_hash31(i + vec3<f32>(0.0, 1.0, 1.0));
    let n111 = pn_hash31(i + vec3<f32>(1.0, 1.0, 1.0));
    let nx00 = mix(n000, n100, u.x);
    let nx10 = mix(n010, n110, u.x);
    let nx01 = mix(n001, n101, u.x);
    let nx11 = mix(n011, n111, u.x);
    return mix(mix(nx00, nx10, u.y), mix(nx01, nx11, u.y), u.z);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn value_noise_2d() -> NodeMetadata {
    NodeMetadata::new("value_noise_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_VALUE_2D])
        .with_source("pn_value_2d(p, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn value_noise_3d() -> NodeMetadata {
    NodeMetadata::new("value_noise_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH31, PN_VALUE_3D])
        .with_source("pn_value_3d(p, scale, seed)")
}

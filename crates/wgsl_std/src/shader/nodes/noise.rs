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

// ============================================================================
// Perlin (gradient) noise
// ============================================================================

pub(crate) const PN_PERLIN_2D_RAW: (&str, &str) = ("pn_perlin_2d_raw", r#"fn pn_perlin_2d_raw(q: vec2<f32>) -> f32 {
    let i = floor(q);
    let f = fract(q);
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let n00 = dot(pn_grad2(i), f);
    let n10 = dot(pn_grad2(i + vec2<f32>(1.0, 0.0)), f - vec2<f32>(1.0, 0.0));
    let n01 = dot(pn_grad2(i + vec2<f32>(0.0, 1.0)), f - vec2<f32>(0.0, 1.0));
    let n11 = dot(pn_grad2(i + vec2<f32>(1.0, 1.0)), f - vec2<f32>(1.0, 1.0));
    return mix(mix(n00, n10, u.x), mix(n01, n11, u.x), u.y) * 1.41421356;
}"#);

const PN_PERLIN_2D: (&str, &str) = ("pn_perlin_2d", r#"fn pn_perlin_2d(p: vec2<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    return clamp(pn_perlin_2d_raw(q) * 0.5 + 0.5, 0.0, 1.0);
}"#);

pub(crate) const PN_PERLIN_3D_RAW: (&str, &str) = ("pn_perlin_3d_raw", r#"fn pn_perlin_3d_raw(q: vec3<f32>) -> f32 {
    let i = floor(q);
    let f = fract(q);
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    let n000 = dot(pn_grad3(i), f);
    let n100 = dot(pn_grad3(i + vec3<f32>(1.0, 0.0, 0.0)), f - vec3<f32>(1.0, 0.0, 0.0));
    let n010 = dot(pn_grad3(i + vec3<f32>(0.0, 1.0, 0.0)), f - vec3<f32>(0.0, 1.0, 0.0));
    let n110 = dot(pn_grad3(i + vec3<f32>(1.0, 1.0, 0.0)), f - vec3<f32>(1.0, 1.0, 0.0));
    let n001 = dot(pn_grad3(i + vec3<f32>(0.0, 0.0, 1.0)), f - vec3<f32>(0.0, 0.0, 1.0));
    let n101 = dot(pn_grad3(i + vec3<f32>(1.0, 0.0, 1.0)), f - vec3<f32>(1.0, 0.0, 1.0));
    let n011 = dot(pn_grad3(i + vec3<f32>(0.0, 1.0, 1.0)), f - vec3<f32>(0.0, 1.0, 1.0));
    let n111 = dot(pn_grad3(i + vec3<f32>(1.0, 1.0, 1.0)), f - vec3<f32>(1.0, 1.0, 1.0));
    let nx00 = mix(n000, n100, u.x);
    let nx10 = mix(n010, n110, u.x);
    let nx01 = mix(n001, n101, u.x);
    let nx11 = mix(n011, n111, u.x);
    return mix(mix(nx00, nx10, u.y), mix(nx01, nx11, u.y), u.z) * 1.15470054;
}"#);

const PN_PERLIN_3D: (&str, &str) = ("pn_perlin_3d", r#"fn pn_perlin_3d(p: vec3<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    return clamp(pn_perlin_3d_raw(q) * 0.5 + 0.5, 0.0, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn perlin_2d() -> NodeMetadata {
    NodeMetadata::new("perlin_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_PERLIN_2D])
        .with_source("pn_perlin_2d(p, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn perlin_3d() -> NodeMetadata {
    NodeMetadata::new("perlin_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_PERLIN_3D])
        .with_source("pn_perlin_3d(p, scale, seed)")
}

// ============================================================================
// Simplex noise (Gustavson construction)
// ============================================================================

const PN_SIMPLEX_2D: (&str, &str) = ("pn_simplex_2d", r#"fn pn_simplex_2d(p: vec2<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    let f2 = 0.36602540378;
    let g2 = 0.21132486540;
    let s = (q.x + q.y) * f2;
    let i = floor(q + s);
    let t = (i.x + i.y) * g2;
    let x0 = q - (i - t);
    var i1 = vec2<f32>(0.0, 1.0);
    if (x0.x > x0.y) {
        i1 = vec2<f32>(1.0, 0.0);
    }
    let x1 = x0 - i1 + g2;
    let x2 = x0 - vec2<f32>(1.0, 1.0) + 2.0 * g2;
    var n = vec3<f32>(0.0);
    var t0 = 0.5 - dot(x0, x0);
    if (t0 > 0.0) { t0 = t0 * t0; n.x = t0 * t0 * dot(pn_grad2(i), x0); }
    var t1 = 0.5 - dot(x1, x1);
    if (t1 > 0.0) { t1 = t1 * t1; n.y = t1 * t1 * dot(pn_grad2(i + i1), x1); }
    var t2 = 0.5 - dot(x2, x2);
    if (t2 > 0.0) { t2 = t2 * t2; n.z = t2 * t2 * dot(pn_grad2(i + vec2<f32>(1.0, 1.0)), x2); }
    let v = 70.0 * (n.x + n.y + n.z);
    return clamp(v * 0.5 + 0.5, 0.0, 1.0);
}"#);

const PN_SIMPLEX_3D: (&str, &str) = ("pn_simplex_3d", r#"fn pn_simplex_3d(p: vec3<f32>, scale: f32, seed: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    let f3 = 1.0 / 3.0;
    let g3 = 1.0 / 6.0;
    let s = (q.x + q.y + q.z) * f3;
    let i = floor(q + s);
    let t = (i.x + i.y + i.z) * g3;
    let x0 = q - (i - t);
    var i1 = vec3<f32>(0.0, 0.0, 0.0);
    var i2 = vec3<f32>(0.0, 0.0, 0.0);
    if (x0.x >= x0.y) {
        if (x0.y >= x0.z) {
            i1 = vec3<f32>(1.0, 0.0, 0.0);
            i2 = vec3<f32>(1.0, 1.0, 0.0);
        } else if (x0.x >= x0.z) {
            i1 = vec3<f32>(1.0, 0.0, 0.0);
            i2 = vec3<f32>(1.0, 0.0, 1.0);
        } else {
            i1 = vec3<f32>(0.0, 0.0, 1.0);
            i2 = vec3<f32>(1.0, 0.0, 1.0);
        }
    } else {
        if (x0.y < x0.z) {
            i1 = vec3<f32>(0.0, 0.0, 1.0);
            i2 = vec3<f32>(0.0, 1.0, 1.0);
        } else if (x0.x < x0.z) {
            i1 = vec3<f32>(0.0, 1.0, 0.0);
            i2 = vec3<f32>(0.0, 1.0, 1.0);
        } else {
            i1 = vec3<f32>(0.0, 1.0, 0.0);
            i2 = vec3<f32>(1.0, 1.0, 0.0);
        }
    }
    let x1 = x0 - i1 + g3;
    let x2 = x0 - i2 + 2.0 * g3;
    let x3 = x0 - vec3<f32>(1.0, 1.0, 1.0) + 3.0 * g3;
    var n = vec4<f32>(0.0);
    var t0 = 0.6 - dot(x0, x0);
    if (t0 > 0.0) { t0 = t0 * t0; n.x = t0 * t0 * dot(pn_grad3(i), x0); }
    var t1 = 0.6 - dot(x1, x1);
    if (t1 > 0.0) { t1 = t1 * t1; n.y = t1 * t1 * dot(pn_grad3(i + i1), x1); }
    var t2 = 0.6 - dot(x2, x2);
    if (t2 > 0.0) { t2 = t2 * t2; n.z = t2 * t2 * dot(pn_grad3(i + i2), x2); }
    var t3 = 0.6 - dot(x3, x3);
    if (t3 > 0.0) { t3 = t3 * t3; n.w = t3 * t3 * dot(pn_grad3(i + vec3<f32>(1.0, 1.0, 1.0)), x3); }
    let v = 32.0 * (n.x + n.y + n.z + n.w);
    return clamp(v * 0.5 + 0.5, 0.0, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn simplex_2d() -> NodeMetadata {
    NodeMetadata::new("simplex_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_SIMPLEX_2D])
        .with_source("pn_simplex_2d(p, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn simplex_3d() -> NodeMetadata {
    NodeMetadata::new("simplex_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_SIMPLEX_3D])
        .with_source("pn_simplex_3d(p, scale, seed)")
}

// ============================================================================
// Voronoi / Worley — returns vec3(F1, F2, cell_random)
// ============================================================================

const PN_VORONOI_2D: (&str, &str) = ("pn_voronoi_2d", r#"fn pn_voronoi_2d(p: vec2<f32>, scale: f32, seed: f32) -> vec3<f32> {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    let i = floor(q);
    let f = fract(q);
    var f1 = 8.0;
    var f2 = 8.0;
    var cell = 0.0;
    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            let n = vec2<f32>(f32(x), f32(y));
            let o = pn_hash22(i + n);
            let d = length(n + o - f);
            if (d < f1) {
                f2 = f1;
                f1 = d;
                cell = pn_hash21(i + n);
            } else if (d < f2) {
                f2 = d;
            }
        }
    }
    return vec3<f32>(f1, f2, cell);
}"#);

const PN_VORONOI_3D: (&str, &str) = ("pn_voronoi_3d", r#"fn pn_voronoi_3d(p: vec3<f32>, scale: f32, seed: f32) -> vec3<f32> {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    let i = floor(q);
    let f = fract(q);
    var f1 = 8.0;
    var f2 = 8.0;
    var cell = 0.0;
    for (var z = -1; z <= 1; z = z + 1) {
        for (var y = -1; y <= 1; y = y + 1) {
            for (var x = -1; x <= 1; x = x + 1) {
                let n = vec3<f32>(f32(x), f32(y), f32(z));
                let o = pn_hash33(i + n);
                let d = length(n + o - f);
                if (d < f1) {
                    f2 = f1;
                    f1 = d;
                    cell = pn_hash31(i + n);
                } else if (d < f2) {
                    f2 = d;
                }
            }
        }
    }
    return vec3<f32>(f1, f2, cell);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn voronoi_2d() -> NodeMetadata {
    NodeMetadata::new("voronoi_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_HASH22, PN_VORONOI_2D])
        .with_source("pn_voronoi_2d(p, scale, seed)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn voronoi_3d() -> NodeMetadata {
    NodeMetadata::new("voronoi_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
        ])
        .with_return_type("vec3<f32>")
        .with_helpers(&[PN_PCG3D, PN_HASH31, PN_HASH33, PN_VORONOI_3D])
        .with_source("pn_voronoi_3d(p, scale, seed)")
}

// ============================================================================
// Fractal combinators over signed Perlin (octaves clamped 1..10)
// ============================================================================

const PN_FBM_2D: (&str, &str) = ("pn_fbm_2d", r#"fn pn_fbm_2d(p: vec2<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    let n = i32(clamp(octaves, 1.0, 10.0));
    var amp = 0.5;
    var freq = 1.0;
    var sum = 0.0;
    var norm = 0.0;
    for (var o = 0; o < n; o = o + 1) {
        sum = sum + amp * pn_perlin_2d_raw(q * freq);
        norm = norm + amp;
        freq = freq * lacunarity;
        amp = amp * gain;
    }
    return clamp((sum / max(norm, 0.00001)) * 0.5 + 0.5, 0.0, 1.0);
}"#);

const PN_FBM_3D: (&str, &str) = ("pn_fbm_3d", r#"fn pn_fbm_3d(p: vec3<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    let n = i32(clamp(octaves, 1.0, 10.0));
    var amp = 0.5;
    var freq = 1.0;
    var sum = 0.0;
    var norm = 0.0;
    for (var o = 0; o < n; o = o + 1) {
        sum = sum + amp * pn_perlin_3d_raw(q * freq);
        norm = norm + amp;
        freq = freq * lacunarity;
        amp = amp * gain;
    }
    return clamp((sum / max(norm, 0.00001)) * 0.5 + 0.5, 0.0, 1.0);
}"#);

const PN_TURBULENCE_2D: (&str, &str) = ("pn_turbulence_2d", r#"fn pn_turbulence_2d(p: vec2<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    let n = i32(clamp(octaves, 1.0, 10.0));
    var amp = 0.5;
    var freq = 1.0;
    var sum = 0.0;
    var norm = 0.0;
    for (var o = 0; o < n; o = o + 1) {
        sum = sum + amp * abs(pn_perlin_2d_raw(q * freq));
        norm = norm + amp;
        freq = freq * lacunarity;
        amp = amp * gain;
    }
    return clamp(sum / max(norm, 0.00001), 0.0, 1.0);
}"#);

const PN_TURBULENCE_3D: (&str, &str) = ("pn_turbulence_3d", r#"fn pn_turbulence_3d(p: vec3<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    let n = i32(clamp(octaves, 1.0, 10.0));
    var amp = 0.5;
    var freq = 1.0;
    var sum = 0.0;
    var norm = 0.0;
    for (var o = 0; o < n; o = o + 1) {
        sum = sum + amp * abs(pn_perlin_3d_raw(q * freq));
        norm = norm + amp;
        freq = freq * lacunarity;
        amp = amp * gain;
    }
    return clamp(sum / max(norm, 0.00001), 0.0, 1.0);
}"#);

const PN_RIDGED_2D: (&str, &str) = ("pn_ridged_2d", r#"fn pn_ridged_2d(p: vec2<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> f32 {
    let q = (p + vec2<f32>(seed * 127.1, seed * 311.7)) * scale;
    let n = i32(clamp(octaves, 1.0, 10.0));
    var amp = 0.5;
    var freq = 1.0;
    var sum = 0.0;
    var norm = 0.0;
    for (var o = 0; o < n; o = o + 1) {
        let r = 1.0 - abs(pn_perlin_2d_raw(q * freq));
        sum = sum + amp * r * r;
        norm = norm + amp;
        freq = freq * lacunarity;
        amp = amp * gain;
    }
    return clamp(sum / max(norm, 0.00001), 0.0, 1.0);
}"#);

const PN_RIDGED_3D: (&str, &str) = ("pn_ridged_3d", r#"fn pn_ridged_3d(p: vec3<f32>, scale: f32, seed: f32, octaves: f32, lacunarity: f32, gain: f32) -> f32 {
    let q = (p + vec3<f32>(seed * 127.1, seed * 311.7, seed * 74.7)) * scale;
    let n = i32(clamp(octaves, 1.0, 10.0));
    var amp = 0.5;
    var freq = 1.0;
    var sum = 0.0;
    var norm = 0.0;
    for (var o = 0; o < n; o = o + 1) {
        let r = 1.0 - abs(pn_perlin_3d_raw(q * freq));
        sum = sum + amp * r * r;
        norm = norm + amp;
        freq = freq * lacunarity;
        amp = amp * gain;
    }
    return clamp(sum / max(norm, 0.00001), 0.0, 1.0);
}"#);

#[distributed_slice(SHADER_REGISTRY)]
pub fn fbm_2d() -> NodeMetadata {
    NodeMetadata::new("fbm_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_FBM_2D])
        .with_source("pn_fbm_2d(p, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn fbm_3d() -> NodeMetadata {
    NodeMetadata::new("fbm_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_FBM_3D])
        .with_source("pn_fbm_3d(p, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn turbulence_2d() -> NodeMetadata {
    NodeMetadata::new("turbulence_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_TURBULENCE_2D])
        .with_source("pn_turbulence_2d(p, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn turbulence_3d() -> NodeMetadata {
    NodeMetadata::new("turbulence_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_TURBULENCE_3D])
        .with_source("pn_turbulence_3d(p, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn ridged_2d() -> NodeMetadata {
    NodeMetadata::new("ridged_2d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec2<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG2D, PN_HASH21, PN_GRAD2, PN_PERLIN_2D_RAW, PN_RIDGED_2D])
        .with_source("pn_ridged_2d(p, scale, seed, octaves, lacunarity, gain)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn ridged_3d() -> NodeMetadata {
    NodeMetadata::new("ridged_3d", NodeTypes::pure, "Noise")
        .with_params(vec![
            ParamInfo::new("p", "vec3<f32>"),
            ParamInfo::new("scale", "f32"),
            ParamInfo::new("seed", "f32"),
            ParamInfo::new("octaves", "f32"),
            ParamInfo::new("lacunarity", "f32"),
            ParamInfo::new("gain", "f32"),
        ])
        .with_return_type("f32")
        .with_helpers(&[PN_PCG3D, PN_HASH33, PN_GRAD3, PN_PERLIN_3D_RAW, PN_RIDGED_3D])
        .with_source("pn_ridged_3d(p, scale, seed, octaves, lacunarity, gain)")
}

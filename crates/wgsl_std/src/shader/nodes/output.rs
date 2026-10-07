//! Shader output nodes
//!
//! Every shader graph is a pure dataflow graph rooted at a single output
//! node (`fragment_output`/`vertex_output`). These are pure sink nodes —
//! they take the final material properties / position as input and have no
//! return value or side effects of their own — and PSGC's code generator
//! builds the `@vertex`/`@fragment` entry function around them.
//!
//! ## Fragment output (PBR metal-rough)
//!
//! The `fragment_output` node exposes the full PBR material pin set:
//!
//! | Pin | Type | Default | Description |
//! |---|---|---|---|
//! | base_color | `vec4<f32>` | `(1,1,1,1)` | Albedo / diffuse colour (RGBA); alpha is multiplied by opacity |
//! | metallic | `f32` | `0.0` | Metalness (0 = dielectric, 1 = metal) |
//! | roughness | `f32` | `0.5` | Surface roughness (0 = smooth, 1 = rough) |
//! | emissive_color | `vec4<f32>` | `(0,0,0,0)` | Self-illumination colour |
//! | normal | `vec3<f32>` | interpolated mesh normal | World-space surface normal |
//! | ambient_occlusion | `f32` | `1.0` | Bent-normal occlusion |
//! | opacity | `f32` | `1.0` | Multiplier for base-color alpha (1 = preserve alpha) |
//! | opacity_mask | `f32` | `1.0` | Binary opacity clip threshold |
//!
//! > **Note:** `@location(0)` is `base_color` (vec4 — matches the existing
//! > RGBA render target).  Remaining outputs at locations 1–7 require the
//! > renderer to create matching colour attachments (deferred / GBuffer
//! > setup).  Graphs that only connect `base_color` produce the same
//! > single-pass forward output as before.

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

/// WGSL expressions for unconnected fragment output pins.
///
/// These are material defaults, not numeric type defaults: zero opacity
/// discards the surface and zero ambient occlusion removes indirect light.
/// `normal` refers to the interpolated world normal in PSGC's fragment entry.
/// Explicit constants and connections always take precedence over these values.
pub fn fragment_output_default(pin: &str) -> Option<&'static str> {
    match pin {
        "base_color" => Some("vec4<f32>(1.0, 1.0, 1.0, 1.0)"),
        "metallic" => Some("0.0"),
        "roughness" => Some("0.5"),
        "emissive_color" => Some("vec4<f32>(0.0, 0.0, 0.0, 0.0)"),
        "normal" => Some("normal"),
        "ambient_occlusion" | "opacity" | "opacity_mask" => Some("1.0"),
        _ => None,
    }
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn fragment_output() -> NodeMetadata {
    NodeMetadata::new("fragment_output", NodeTypes::pure, "Output")
        .with_params(vec![
            ParamInfo::new("base_color", "vec4<f32>"),
            ParamInfo::new("metallic", "f32"),
            ParamInfo::new("roughness", "f32"),
            ParamInfo::new("emissive_color", "vec4<f32>"),
            ParamInfo::new("normal", "vec3<f32>"),
            ParamInfo::new("ambient_occlusion", "f32"),
            ParamInfo::new("opacity", "f32"),
            ParamInfo::new("opacity_mask", "f32"),
        ])
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn vertex_output() -> NodeMetadata {
    NodeMetadata::new("vertex_output", NodeTypes::pure, "Output")
        .with_params(vec![
            ParamInfo::new("position", "vec4<f32>"),
            ParamInfo::new("uv", "vec2<f32>"),
            ParamInfo::new("normal", "vec3<f32>"),
            ParamInfo::new("world_position", "vec3<f32>"),
        ])
}

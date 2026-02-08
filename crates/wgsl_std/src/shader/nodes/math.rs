//! Math shader nodes
//!
//! Basic mathematical operations for shaders

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

// ============================================================================
// Basic Arithmetic
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn add() -> NodeMetadata {
    NodeMetadata::new("add", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("a", "f32"),
            ParamInfo::new("b", "f32"),
        ])
        .with_return_type("f32")
        .with_source("a + b")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn subtract() -> NodeMetadata {
    NodeMetadata::new("subtract", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("a", "f32"),
            ParamInfo::new("b", "f32"),
        ])
        .with_return_type("f32")
        .with_source("a - b")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn multiply() -> NodeMetadata {
    NodeMetadata::new("multiply", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("a", "f32"),
            ParamInfo::new("b", "f32"),
        ])
        .with_return_type("f32")
        .with_source("a * b")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn divide() -> NodeMetadata {
    NodeMetadata::new("divide", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("a", "f32"),
            ParamInfo::new("b", "f32"),
        ])
        .with_return_type("f32")
        .with_source("a / b")
}

// ============================================================================
// Trigonometry
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn sin() -> NodeMetadata {
    NodeMetadata::new("sin", NodeTypes::pure, "Math")
        .with_params(vec![ParamInfo::new("x", "f32")])
        .with_return_type("f32")
        .with_source("sin(x)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn cos() -> NodeMetadata {
    NodeMetadata::new("cos", NodeTypes::pure, "Math")
        .with_params(vec![ParamInfo::new("x", "f32")])
        .with_return_type("f32")
        .with_source("cos(x)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn tan() -> NodeMetadata {
    NodeMetadata::new("tan", NodeTypes::pure, "Math")
        .with_params(vec![ParamInfo::new("x", "f32")])
        .with_return_type("f32")
        .with_source("tan(x)")
}

// ============================================================================
// Interpolation
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn lerp() -> NodeMetadata {
    NodeMetadata::new("lerp", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("a", "f32"),
            ParamInfo::new("b", "f32"),
            ParamInfo::new("t", "f32"),
        ])
        .with_return_type("f32")
        .with_source("mix(a, b, t)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn clamp() -> NodeMetadata {
    NodeMetadata::new("clamp", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("value", "f32"),
            ParamInfo::new("min", "f32"),
            ParamInfo::new("max", "f32"),
        ])
        .with_return_type("f32")
        .with_source("clamp(value, min, max)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn smoothstep() -> NodeMetadata {
    NodeMetadata::new("smoothstep", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("edge0", "f32"),
            ParamInfo::new("edge1", "f32"),
            ParamInfo::new("x", "f32"),
        ])
        .with_return_type("f32")
        .with_source("smoothstep(edge0, edge1, x)")
}

// ============================================================================
// Other Math
// ============================================================================

#[distributed_slice(SHADER_REGISTRY)]
pub fn pow() -> NodeMetadata {
    NodeMetadata::new("pow", NodeTypes::pure, "Math")
        .with_params(vec![
            ParamInfo::new("base", "f32"),
            ParamInfo::new("exponent", "f32"),
        ])
        .with_return_type("f32")
        .with_source("pow(base, exponent)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn sqrt() -> NodeMetadata {
    NodeMetadata::new("sqrt", NodeTypes::pure, "Math")
        .with_params(vec![ParamInfo::new("x", "f32")])
        .with_return_type("f32")
        .with_source("sqrt(x)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn abs() -> NodeMetadata {
    NodeMetadata::new("abs", NodeTypes::pure, "Math")
        .with_params(vec![ParamInfo::new("x", "f32")])
        .with_return_type("f32")
        .with_source("abs(x)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn fract() -> NodeMetadata {
    NodeMetadata::new("fract", NodeTypes::pure, "Math")
        .with_params(vec![ParamInfo::new("x", "f32")])
        .with_return_type("f32")
        .with_source("fract(x)")
}


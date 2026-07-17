//! Coordinate input nodes — world/object/camera/screen positions

use crate::SHADER_REGISTRY;
use graphy::core::{NodeMetadata, NodeTypes, ParamInfo};
use linkme::distributed_slice;

#[distributed_slice(SHADER_REGISTRY)]
pub fn texture_coordinate() -> NodeMetadata {
    NodeMetadata::new("texture_coordinate", NodeTypes::pure, "Coordinates")
        .with_params(vec![ParamInfo::new("index", "i32")])
        .with_return_type("vec2<f32>")
        .with_source("select(vec2<f32>(0.0, 0.0), uv, index == 0)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn world_position() -> NodeMetadata {
    NodeMetadata::new("world_position", NodeTypes::pure, "Coordinates")
        .with_return_type("vec3<f32>")
        .with_source("world_pos")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn object_position() -> NodeMetadata {
    NodeMetadata::new("object_position", NodeTypes::pure, "Coordinates")
        .with_return_type("vec3<f32>")
        .with_source("(uniforms.model * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn camera_vector() -> NodeMetadata {
    NodeMetadata::new("camera_vector", NodeTypes::pure, "Coordinates")
        .with_return_type("vec3<f32>")
        .with_source("normalize(uniforms.view_proj[3].xyz - world_pos)")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn screen_position() -> NodeMetadata {
    NodeMetadata::new("screen_position", NodeTypes::pure, "Coordinates")
        .with_return_type("vec4<f32>")
        .with_source("frag_coord")
}

#[distributed_slice(SHADER_REGISTRY)]
pub fn pixel_normal_ws() -> NodeMetadata {
    NodeMetadata::new("pixel_normal_ws", NodeTypes::pure, "Coordinates")
        .with_return_type("vec3<f32>")
        .with_source("normalize(normal)")
}

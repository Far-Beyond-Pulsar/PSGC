//! # Shader Node Metadata
//!
//! Metadata system for WGSL shader nodes.

use graphy::core::{NodeMetadata, NodeMetadataProvider, NodeTypes, ParamInfo, TypeInfo};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Cached shader node metadata
static SHADER_METADATA: OnceLock<HashMap<String, NodeMetadata>> = OnceLock::new();

/// Initialize built-in shader nodes
fn create_shader_nodes() -> HashMap<String, NodeMetadata> {
    let mut nodes = HashMap::new();

    // Math nodes
    nodes.insert(
        "add".to_string(),
        NodeMetadata::new("add", NodeTypes::pure, "Math")
            .with_params(vec![
                ParamInfo::new("a", "f32"),
                ParamInfo::new("b", "f32"),
            ])
            .with_return_type(TypeInfo::new("f32"))
            .with_source("fn add(a: f32, b: f32) -> f32 { return a + b; }"),
    );

    nodes.insert(
        "multiply".to_string(),
        NodeMetadata::new("multiply", NodeTypes::pure, "Math")
            .with_params(vec![
                ParamInfo::new("a", "f32"),
                ParamInfo::new("b", "f32"),
            ])
            .with_return_type(TypeInfo::new("f32"))
            .with_source("fn multiply(a: f32, b: f32) -> f32 { return a * b; }"),
    );

    nodes.insert(
        "dot".to_string(),
        NodeMetadata::new("dot", NodeTypes::pure, "Math")
            .with_params(vec![
                ParamInfo::new("a", "vec3<f32>"),
                ParamInfo::new("b", "vec3<f32>"),
            ])
            .with_return_type(TypeInfo::new("f32"))
            .with_source("fn dot_product(a: vec3<f32>, b: vec3<f32>) -> f32 { return dot(a, b); }"),
    );

    nodes.insert(
        "normalize".to_string(),
        NodeMetadata::new("normalize", NodeTypes::pure, "Math")
            .with_params(vec![ParamInfo::new("v", "vec3<f32>")])
            .with_return_type(TypeInfo::new("vec3<f32>"))
            .with_source("fn normalize_vec(v: vec3<f32>) -> vec3<f32> { return normalize(v); }"),
    );

    // Texture sampling
    nodes.insert(
        "sample_texture".to_string(),
        NodeMetadata::new("sample_texture", NodeTypes::pure, "Texture")
            .with_params(vec![
                ParamInfo::new("tex", "texture_2d<f32>"),
                ParamInfo::new("samp", "sampler"),
                ParamInfo::new("uv", "vec2<f32>"),
            ])
            .with_return_type(TypeInfo::new("vec4<f32>"))
            .with_source("fn sample_texture(tex: texture_2d<f32>, samp: sampler, uv: vec2<f32>) -> vec4<f32> { return textureSample(tex, samp, uv); }"),
    );

    // Vector operations
    nodes.insert(
        "vec3".to_string(),
        NodeMetadata::new("vec3", NodeTypes::pure, "Vector")
            .with_params(vec![
                ParamInfo::new("x", "f32"),
                ParamInfo::new("y", "f32"),
                ParamInfo::new("z", "f32"),
            ])
            .with_return_type(TypeInfo::new("vec3<f32>"))
            .with_source("fn make_vec3(x: f32, y: f32, z: f32) -> vec3<f32> { return vec3<f32>(x, y, z); }"),
    );

    nodes.insert(
        "vec4".to_string(),
        NodeMetadata::new("vec4", NodeTypes::pure, "Vector")
            .with_params(vec![
                ParamInfo::new("x", "f32"),
                ParamInfo::new("y", "f32"),
                ParamInfo::new("z", "f32"),
                ParamInfo::new("w", "f32"),
            ])
            .with_return_type(TypeInfo::new("vec4<f32>"))
            .with_source("fn make_vec4(x: f32, y: f32, z: f32, w: f32) -> vec4<f32> { return vec4<f32>(x, y, z, w); }"),
    );

    // Shader entry points
    nodes.insert(
        "vertex_main".to_string(),
        NodeMetadata::new("vertex_main", NodeTypes::event, "Entry")
            .with_exec_outputs(vec!["Body".to_string()])
            .with_source("@vertex fn vertex_main() -> @builtin(position) vec4<f32> { /* body */ }"),
    );

    nodes.insert(
        "fragment_main".to_string(),
        NodeMetadata::new("fragment_main", NodeTypes::event, "Entry")
            .with_exec_outputs(vec!["Body".to_string()])
            .with_source("@fragment fn fragment_main() -> @location(0) vec4<f32> { /* body */ }"),
    );

    nodes
}

/// Get shader node metadata
pub fn get_shader_nodes() -> &'static HashMap<String, NodeMetadata> {
    SHADER_METADATA.get_or_init(|| {
        let nodes = create_shader_nodes();
        tracing::info!("[PSGC] Loaded {} shader node definitions", nodes.len());
        nodes
    })
}

/// Shader metadata provider
pub struct ShaderMetadataProvider {
    metadata: &'static HashMap<String, NodeMetadata>,
}

impl ShaderMetadataProvider {
    pub fn new() -> Self {
        Self {
            metadata: get_shader_nodes(),
        }
    }
}

impl Default for ShaderMetadataProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeMetadataProvider for ShaderMetadataProvider {
    fn get_node_metadata(&self, node_type: &str) -> Option<&NodeMetadata> {
        self.metadata.get(node_type)
    }

    fn get_all_nodes(&self) -> Vec<&NodeMetadata> {
        self.metadata.values().collect()
    }

    fn get_nodes_by_category(&self, category: &str) -> Vec<&NodeMetadata> {
        self.metadata
            .values()
            .filter(|m| m.category == category)
            .collect()
    }
}

//! Shader node metadata management
//!
//! Integrates wgsl_std shader nodes into PSGC's metadata system

use graphy::NodeMetadataProvider;
use graphy::core::NodeMetadata;
use std::collections::HashMap;

/// PSGC shader metadata provider
/// 
/// Loads shader nodes from wgsl_std registry and provides them to the compiler
pub struct ShaderMetadataProvider {
    nodes: HashMap<String, NodeMetadata>,
}

impl ShaderMetadataProvider {
    /// Create a new provider by loading all nodes from wgsl_std
    pub fn new() -> Self {
        let mut nodes = HashMap::new();

        // Load all shader nodes from wgsl_std registry
        for node_fn in wgsl_std::SHADER_REGISTRY.iter() {
            let metadata = node_fn();
            tracing::debug!("[PSGC] Loaded shader node: {} ({})", metadata.name, metadata.category);
            nodes.insert(metadata.name.clone(), metadata);
        }

        tracing::info!("[PSGC] Loaded {} shader nodes from wgsl_std", nodes.len());

        Self { nodes }
    }
}

impl Default for ShaderMetadataProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeMetadataProvider for ShaderMetadataProvider {
    fn get_node_metadata(&self, node_type: &str) -> Option<&NodeMetadata> {
        self.nodes.get(node_type)
    }

    fn get_all_nodes(&self) -> Vec<&NodeMetadata> {
        self.nodes.values().collect()
    }
    
    fn get_nodes_by_category(&self, category: &str) -> Vec<&NodeMetadata> {
        self.nodes.values().filter(|m| m.category == category).collect()
    }
}

/// Get all available shader nodes
pub fn get_shader_nodes() -> Vec<NodeMetadata> {
    wgsl_std::SHADER_REGISTRY.iter().map(|f| f()).collect()
}

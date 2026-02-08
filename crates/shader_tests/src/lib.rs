//! # WGSL Shader Compilation Tests
//!
//! Test suite that compiles sample Blueprint graphs into WGSL shader code

#[cfg(test)]
mod tests {
    use psgc::ShaderMetadataProvider;
    use graphy::NodeMetadataProvider;

    fn init_logging() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .try_init();
    }

    /// Test: Load all shader nodes from wgsl_std
    #[test]
    fn test_load_shader_nodes() {
        init_logging();

        println!("\n=== Loading WGSL Shader Nodes ===\n");

        let metadata_provider = ShaderMetadataProvider::new();
        let nodes = metadata_provider.get_all_nodes();

        println!("Loaded {} shader nodes from wgsl_std", nodes.len());
        
        // Show sample nodes by category
        for category in &["Math", "Vector", "Color", "Texture", "Input", "Output"] {
            let category_nodes = metadata_provider.get_nodes_by_category(category);
            if !category_nodes.is_empty() {
                println!("  - {} ({} nodes): {}", 
                    category, 
                    category_nodes.len(),
                    category_nodes.iter()
                        .take(5)
                        .map(|n| n.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }

        assert!(nodes.len() > 30, "Should load many shader nodes");
        println!("\n✓ Successfully loaded all shader nodes");
    }
}

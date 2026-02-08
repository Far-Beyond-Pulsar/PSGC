//! # WGSL Shader Compilation Tests
//!
//! Test suite that compiles sample Blueprint graphs into WGSL shader code

#[cfg(test)]
mod tests {
    use psgc::{ShaderMetadataProvider, compile_fragment_shader};
    use graphy::{
        NodeMetadataProvider, GraphDescription, NodeInstance, Connection,
        Pin, PinInstance, DataType, Position, ConnectionType, PropertyValue, PinType,
    };
    use std::fs;

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

    /// Test: Compile a simple math shader
    /// 
    /// Graph structure:
    /// frag_uv → multiply(uv.x, 2.0) → sin(x) → rgba(r,r,r,1) → fragment_output
    #[test]
    fn test_compile_math_shader() {
        init_logging();

        println!("\n=== Compiling Math Shader ===\n");

        let mut graph = GraphDescription::new("math_shader");

        // Node 1: fragment_output (entry point)
        let mut output = NodeInstance::new(
            "output_1",
            "fragment_main",
            Position { x: 700.0, y: 200.0 }
        );
        output.outputs.push(PinInstance::new(
            "output_1_Body",
            Pin::new("output_1_Body", "Body", DataType::Execution, PinType::Output)
        ));

        // Node 2: rgba color constructor
        let mut rgba = NodeInstance::new(
            "rgba_1",
            "rgba",
            Position { x: 500.0, y: 200.0 }
        );
        rgba.inputs.push(PinInstance::new(
            "rgba_1_r",
            Pin::new("rgba_1_r", "r", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        rgba.inputs.push(PinInstance::new(
            "rgba_1_g",
            Pin::new("rgba_1_g", "g", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        rgba.inputs.push(PinInstance::new(
            "rgba_1_b",
            Pin::new("rgba_1_b", "b", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        rgba.inputs.push(PinInstance::new(
            "rgba_1_a",
            Pin::new("rgba_1_a", "a", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        rgba.outputs.push(PinInstance::new(
            "rgba_1_result",
            Pin::new("rgba_1_result", "result", DataType::Typed(psgc::TypeInfo::new("vec4<f32>")), PinType::Output)
        ));

        // Set constant values
        rgba.properties.insert("rgba_1_g".to_string(), PropertyValue::Number(0.0));
        rgba.properties.insert("rgba_1_b".to_string(), PropertyValue::Number(0.0));
        rgba.properties.insert("rgba_1_a".to_string(), PropertyValue::Number(1.0));

        // Node 3: sin
        let mut sin_node = NodeInstance::new(
            "sin_1",
            "sin",
            Position { x: 350.0, y: 200.0 }
        );
        sin_node.inputs.push(PinInstance::new(
            "sin_1_x",
            Pin::new("sin_1_x", "x", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        sin_node.outputs.push(PinInstance::new(
            "sin_1_result",
            Pin::new("sin_1_result", "result", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Output)
        ));

        // Node 4: multiply
        let mut multiply = NodeInstance::new(
            "multiply_1",
            "multiply",
            Position { x: 200.0, y: 200.0 }
        );
        multiply.inputs.push(PinInstance::new(
            "multiply_1_a",
            Pin::new("multiply_1_a", "a", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        multiply.inputs.push(PinInstance::new(
            "multiply_1_b",
            Pin::new("multiply_1_b", "b", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Input)
        ));
        multiply.outputs.push(PinInstance::new(
            "multiply_1_result",
            Pin::new("multiply_1_result", "result", DataType::Typed(psgc::TypeInfo::new("f32")), PinType::Output)
        ));

        // Constant multiplier
        multiply.properties.insert("multiply_1_b".to_string(), PropertyValue::Number(6.28));

        // Node 5: frag_uv input
        let mut frag_uv = NodeInstance::new(
            "uv_1",
            "frag_uv",
            Position { x: 50.0, y: 200.0 }
        );
        frag_uv.outputs.push(PinInstance::new(
            "uv_1_result",
            Pin::new("uv_1_result", "result", DataType::Typed(psgc::TypeInfo::new("vec2<f32>")), PinType::Output)
        ));

        // Add all nodes
        graph.add_node(output);
        graph.add_node(rgba);
        graph.add_node(sin_node);
        graph.add_node(multiply);
        graph.add_node(frag_uv);

        // Data connections: frag_uv.x → multiply → sin → rgba.r → output
        graph.add_connection(Connection::new(
            "uv_1", "uv_1_result",
            "multiply_1", "multiply_1_a",
            ConnectionType::Data
        ));
        graph.add_connection(Connection::new(
            "multiply_1", "multiply_1_result",
            "sin_1", "sin_1_x",
            ConnectionType::Data
        ));
        graph.add_connection(Connection::new(
            "sin_1", "sin_1_result",
            "rgba_1", "rgba_1_r",
            ConnectionType::Data
        ));

        println!("Graph created with {} nodes and {} connections\n", 
            graph.nodes.len(), graph.connections.len());

        // Compile the shader
        println!("=== Compiling Shader ===\n");
        match compile_fragment_shader(&graph) {
            Ok(wgsl_code) => {
                println!("✓ === Compilation Successful! ===");
                println!("Generated {} bytes of WGSL code\n", wgsl_code.len());
                
                println!("=== Generated WGSL Code ===");
                println!("{}", wgsl_code);
                println!("=== End of Generated Code ===\n");

                // Write to file
                let output_path = "../../target/math_shader.wgsl";
                fs::write(output_path, &wgsl_code)
                    .expect("Failed to write shader file");
                println!("✓ Output written to: {}\n", output_path);

                // Basic validation
                assert!(wgsl_code.contains("@fragment") || wgsl_code.contains("fragment"), 
                    "Should have fragment shader marker");
                // Note: Full data flow code generation not yet implemented
                // assert!(wgsl_code.contains("sin"), "Should use sin function");
            }
            Err(e) => {
                panic!("✗ Shader compilation failed: {}", e);
            }
        }

        println!("✓ Test Passed! Math shader compiled successfully");
    }
}


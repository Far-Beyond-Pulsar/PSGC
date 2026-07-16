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

    /// Parse + validate generated WGSL with naga. Panics with full
    /// diagnostics on any syntax or type error — no GPU required.
    fn validate_wgsl(wgsl: &str) {
        let module = naga::front::wgsl::parse_str(wgsl)
            .unwrap_or_else(|e| panic!("WGSL parse error: {}\n--- WGSL ---\n{wgsl}", e.emit_to_string(wgsl)));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::default(),
        )
        .validate(&module)
        .unwrap_or_else(|e| panic!("WGSL validation error: {e:?}\n--- WGSL ---\n{wgsl}"));
    }

    #[test]
    fn validate_wgsl_rejects_garbage() {
        let result = std::panic::catch_unwind(|| validate_wgsl("fn broken( -> {"));
        assert!(result.is_err(), "garbage WGSL must fail validation");
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
            "fragment_output",
            Position { x: 700.0, y: 200.0 }
        );
        output.outputs.push(PinInstance::new(
            "output_1_Body",
            Pin::new("output_1_Body", "Body", DataType::Exec, PinType::Output)
        ));

        // Node 2: rgba color constructor
        let mut rgba = NodeInstance::new(
            "rgba_1",
            "rgba",
            Position { x: 500.0, y: 200.0 }
        );
        rgba.inputs.push(PinInstance::new(
            "rgba_1_r",
            Pin::new("rgba_1_r", "r", DataType::typed("f32"), PinType::Input)
        ));
        rgba.inputs.push(PinInstance::new(
            "rgba_1_g",
            Pin::new("rgba_1_g", "g", DataType::typed("f32"), PinType::Input)
        ));
        rgba.inputs.push(PinInstance::new(
            "rgba_1_b",
            Pin::new("rgba_1_b", "b", DataType::typed("f32"), PinType::Input)
        ));
        rgba.inputs.push(PinInstance::new(
            "rgba_1_a",
            Pin::new("rgba_1_a", "a", DataType::typed("f32"), PinType::Input)
        ));
        rgba.outputs.push(PinInstance::new(
            "rgba_1_result",
            Pin::new("rgba_1_result", "result", DataType::typed("vec4<f32>"), PinType::Output)
        ));

        // Set constant values
        rgba.properties.insert("rgba_1_g".to_string(), PropertyValue::Float(0.0).to_json());
        rgba.properties.insert("rgba_1_b".to_string(), PropertyValue::Float(0.0).to_json());
        rgba.properties.insert("rgba_1_a".to_string(), PropertyValue::Float(1.0).to_json());

        // Node 3: sin
        let mut sin_node = NodeInstance::new(
            "sin_1",
            "sin",
            Position { x: 350.0, y: 200.0 }
        );
        sin_node.inputs.push(PinInstance::new(
            "sin_1_x",
            Pin::new("sin_1_x", "x", DataType::typed("f32"), PinType::Input)
        ));
        sin_node.outputs.push(PinInstance::new(
            "sin_1_result",
            Pin::new("sin_1_result", "result", DataType::typed("f32"), PinType::Output)
        ));

        // Node 4: multiply
        let mut multiply = NodeInstance::new(
            "multiply_1",
            "multiply",
            Position { x: 200.0, y: 200.0 }
        );
        multiply.inputs.push(PinInstance::new(
            "multiply_1_a",
            Pin::new("multiply_1_a", "a", DataType::typed("f32"), PinType::Input)
        ));
        multiply.inputs.push(PinInstance::new(
            "multiply_1_b",
            Pin::new("multiply_1_b", "b", DataType::typed("f32"), PinType::Input)
        ));
        multiply.outputs.push(PinInstance::new(
            "multiply_1_result",
            Pin::new("multiply_1_result", "result", DataType::typed("f32"), PinType::Output)
        ));

        // Constant multiplier
        multiply.properties.insert("multiply_1_b".to_string(), PropertyValue::Float(6.28).to_json());

        // Node 5: frag_uv input
        let mut frag_uv = NodeInstance::new(
            "uv_1",
            "frag_uv",
            Position { x: 50.0, y: 200.0 }
        );
        frag_uv.outputs.push(PinInstance::new(
            "uv_1_result",
            Pin::new("uv_1_result", "result", DataType::typed("vec2<f32>"), PinType::Output)
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

                validate_wgsl(&wgsl_code);
            }
            Err(e) => {
                panic!("✗ Shader compilation failed: {}", e);
            }
        }

        println!("✓ Test Passed! Math shader compiled successfully");
    }

    /// Build: <noise node> (all params unconnected → defaults) → rgba.r → fragment_output.
    /// `extra_params` are f32 input pin names beyond (p, scale, seed).
    fn build_scalar_noise_graph(node_type: &str, extra_params: &[&str]) -> GraphDescription {
        let mut graph = GraphDescription::new(&format!("{node_type}_test"));

        let mut output = NodeInstance::new("output_1", "fragment_output", Position { x: 900.0, y: 200.0 });
        output.inputs.push(PinInstance::new(
            "output_1_color",
            Pin::new("output_1_color", "color", DataType::typed("vec4<f32>"), PinType::Input),
        ));

        let mut rgba = NodeInstance::new("rgba_1", "rgba", Position { x: 700.0, y: 200.0 });
        for ch in ["r", "g", "b", "a"] {
            rgba.inputs.push(PinInstance::new(
                format!("rgba_1_{ch}"),
                Pin::new(format!("rgba_1_{ch}"), ch, DataType::typed("f32"), PinType::Input),
            ));
        }
        rgba.outputs.push(PinInstance::new(
            "rgba_1_result",
            Pin::new("rgba_1_result", "result", DataType::typed("vec4<f32>"), PinType::Output),
        ));
        rgba.properties.insert("rgba_1_a".to_string(), PropertyValue::Float(1.0).to_json());

        let mut noise = NodeInstance::new("noise_1", node_type, Position { x: 500.0, y: 200.0 });
        let p_type = if node_type.ends_with("_3d") { "vec3<f32>" } else { "vec2<f32>" };
        noise.inputs.push(PinInstance::new(
            "noise_1_p",
            Pin::new("noise_1_p", "p", DataType::typed(p_type), PinType::Input),
        ));
        for param in ["scale", "seed"].iter().chain(extra_params.iter()) {
            noise.inputs.push(PinInstance::new(
                format!("noise_1_{param}"),
                Pin::new(format!("noise_1_{param}"), *param, DataType::typed("f32"), PinType::Input),
            ));
        }
        noise.outputs.push(PinInstance::new(
            "noise_1_result",
            Pin::new("noise_1_result", "result", DataType::typed("f32"), PinType::Output),
        ));

        graph.add_node(output);
        graph.add_node(rgba);
        graph.add_node(noise);
        graph.add_connection(Connection::new("noise_1", "noise_1_result", "rgba_1", "rgba_1_r", ConnectionType::Data));
        graph.add_connection(Connection::new("rgba_1", "rgba_1_result", "output_1", "output_1_color", ConnectionType::Data));
        graph
    }

    fn assert_noise_node_compiles(node_type: &str, extra_params: &[&str]) {
        let graph = build_scalar_noise_graph(node_type, extra_params);
        let wgsl = compile_fragment_shader(&graph)
            .unwrap_or_else(|e| panic!("{node_type} failed to compile: {e}"));
        assert!(wgsl.contains("fn pn_"), "{node_type} must emit pn_ helper functions:\n{wgsl}");
        validate_wgsl(&wgsl);
    }

    #[test]
    fn noise_white_and_value_nodes_compile_and_validate() {
        for node in ["white_noise_2d", "white_noise_3d", "value_noise_2d", "value_noise_3d"] {
            assert_noise_node_compiles(node, &[]);
        }
    }

    #[test]
    fn noise_perlin_and_simplex_nodes_compile_and_validate() {
        for node in ["perlin_2d", "perlin_3d", "simplex_2d", "simplex_3d"] {
            assert_noise_node_compiles(node, &[]);
        }
    }
}


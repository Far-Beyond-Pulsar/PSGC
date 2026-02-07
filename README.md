# PSGC - Pulsar Shader Graph Compiler

Compiler for visual shader graphs to WGSL (WebGPU Shading Language) code.

## Features

- Compiles visual shader node graphs to WGSL code
- Support for vertex, fragment, and compute shaders
- Built-in shader nodes (math, textures, vectors)
- Shares infrastructure with PBGC via Graphy library
- Type-safe shader compilation

## Quick Start

```rust
use psgc::{compile_shader, ShaderStage};
use graphy::GraphDescription;

let graph = GraphDescription::new("my_shader");
// ... build graph with shader nodes

match compile_shader(&graph) {
    Ok(wgsl_code) => std::fs::write("shader.wgsl", wgsl_code)?,
    Err(e) => eprintln!("Error: {}", e),
}
```

## Built-in Shader Nodes

### Math
- `add(a, b)` - Add two floats
- `multiply(a, b)` - Multiply two floats
- `dot(a, b)` - Dot product of vec3s
- `normalize(v)` - Normalize a vector

### Vector
- `vec3(x, y, z)` - Create vec3<f32>
- `vec4(x, y, z, w)` - Create vec4<f32>

### Texture
- `sample_texture(tex, sampler, uv)` - Sample 2D texture

### Entry Points
- `vertex_main` - Vertex shader entry
- `fragment_main` - Fragment shader entry

## Example Shader Graph

```rust
use psgc::*;
use graphy::*;

let mut graph = GraphDescription::new("gradient_shader");

// Fragment entry
let mut fragment = NodeInstance::new("frag", "fragment_main", Position::zero());
fragment.add_output_pin("Body", DataType::Execution);
graph.add_node(fragment);

// The compiler generates valid WGSL:
let wgsl = compile_fragment_shader(&graph)?;
```

Output WGSL:
```wgsl
// Auto-generated WGSL shader from Pulsar Shader Graph

@fragment
fn fragment_main(
    @builtin(position) frag_coord: vec4<f32>,
) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);
}
```

## Architecture

Built on [Graphy](https://github.com/Far-Beyond-Pulsar/Graphy) library with shader-specific:
- WGSL node metadata
- WGSL code generation
- Shader stage handling
- Texture/uniform support

## Extending with Custom Nodes

Add custom shader nodes by extending the metadata:

```rust
use psgc::metadata::get_shader_nodes;

// Add your custom nodes to the registry
// See metadata.rs for examples
```

## Shader Stages

PSGC supports three shader stages:

- **Vertex** - `compile_vertex_shader()`
- **Fragment** - `compile_fragment_shader()`
- **Compute** - Coming soon

## Integration with Pulsar

PSGC is designed to integrate with Pulsar's shader system, providing visual shader authoring with production-ready compilation.

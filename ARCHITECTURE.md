# PSGC Architecture

## Overview

PSGC (Pulsar Shader Graph Compiler) follows the same layered architecture as PBGC:

```
┌─────────────────────────────────────┐
│            PSGC (Shader)            │
│  - Shader node metadata             │
│  - WGSL code generation             │
│  - Texture/uniform handling         │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│         Graphy (General)            │
│  - Graph data structures            │
│  - Data flow analysis               │
│  - Execution flow analysis          │
└─────────────────────────────────────┘
```

## Module Structure

### PSGC Modules

- **`lib.rs`** - Public API and re-exports
- **`metadata.rs`** - Shader node definitions
- **`compiler.rs`** - Main compilation entry points
- **`codegen/`** - WGSL code generation
  - `wgsl_codegen.rs` - Shader graph → WGSL generator

## Compilation Pipeline

1. **Load Metadata** - Load built-in shader nodes
2. **Data Flow Analysis** - Build dependency graph (Graphy)
3. **Execution Flow** - Map shader stages (Graphy)
4. **Code Generation** - Generate WGSL code (PSGC)

## Shader Node Types

All shader nodes are **pure** functions that get inlined:

```rust
// Node definition
NodeMetadata::new("add", NodeTypes::pure, "Math")
    .with_params(vec![
        ParamInfo::new("a", "f32"),
        ParamInfo::new("b", "f32"),
    ])
    .with_return_type(TypeInfo::new("f32"))
```

Generated WGSL:
```wgsl
let result = add(5.0, 3.0);
```

## Shader Stages

PSGC supports multiple shader stages:

- **Vertex**: `@vertex fn vertex_main()`
- **Fragment**: `@fragment fn fragment_main()`
- **Compute**: `@compute fn compute_main()` (future)

Each stage has specific entry point requirements and return types.

## WGSL Generation

The code generator:

1. Finds entry point node (vertex_main/fragment_main)
2. Generates function signature with correct attributes
3. Inlines pure nodes as expressions
4. Adds appropriate return statements

### Example

Graph: `add(5.0, 3.0)` → Fragment output

Generated:
```wgsl
@fragment
fn fragment_main(
    @builtin(position) frag_coord: vec4<f32>,
) -> @location(0) vec4<f32> {
    let node_add_1_result = add(5.0, 3.0);
    return vec4<f32>(node_add_1_result, 0.0, 0.0, 1.0);
}
```

## Shared Infrastructure

PSGC and PBGC share:
- Graph analysis (via Graphy)
- Data flow resolution
- Execution routing
- AST utilities (for complex nodes)

This means improvements to one compiler benefit the other!

## Future Extensions

Planned features:
- Compute shader support
- Custom texture formats
- Uniform buffer generation
- Material system integration

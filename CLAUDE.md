# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

**Build and run:**
```bash
cargo run --release
```

**Install and run:**
```bash
cargo install --path .
flow
```

**Development:**
```bash
cargo build
cargo check
```

## Architecture

This is a Rust-based particle flow field visualization using WGPU (WebGPU) for rendering and EGUI for the user interface.

### Core Structure

- **App struct**: Main application loop managing WGPU surface, EGUI state, and the Model
- **Model**: Central state containing flow vectors, particles, and all configuration
- **Renderer**: WGPU-based line renderer for particles using custom shaders
- **Flow System**: Grid-based vector field that guides particle movement

### Key Components

**Model (`src/model/mod.rs`)**: 
- Contains all simulation state (particles, flow vectors, UI settings)
- Manages noise generation for flow field patterns
- Handles particle spawning and lifecycle

**Renderer (`src/renderer.rs`)**:
- WGPU pipeline rendering particles as colored line segments
- Uses custom vertex shader (`src/shaders/shader.wgsl`)
- Builds vertex buffers dynamically from particle positions

**Flow System**:
- `FlowVector`: Grid-based directional vectors using noise functions
- `FlowParticle`: Individual particles that follow the flow field
- Multiple noise types available (Billow, etc.)

**Interactive Controls**:
- Mouse spawning (left click, right drag)
- Keyboard shortcuts for noise regeneration, palette cycling
- EGUI interface for parameter adjustment

### Data Flow

1. Model generates flow vector grid using noise functions
2. Particles spawn at mouse/random positions  
3. Update loop moves particles along flow vectors
4. Renderer converts particle trails to line segments
5. WGPU renders lines with alpha blending

The main event loop is commented out in `main.rs:350-360` - particles currently don't move or update.
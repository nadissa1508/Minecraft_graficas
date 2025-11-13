# Minecraft Raytracer - Diorama

A Minecraft-style diorama renderer using ray tracing, implemented in Rust with the Raylib library for window and display management.

## Features

- Real-time ray tracing with reflections and refractions
- Minecraft-style voxel rendering
- Cherry tree diorama scene with axolotl and pond
- **Dynamic day/night cycle with visible sun and moon**
- Adjustable quality levels (Low, Medium, High)
- Multithreading support for improved performance
- Interactive orbital camera controls
- Material system with:
  - Textured surfaces
  - Transparent/refractive materials (water, glass, grass) with Fresnel effects
  - Reflective materials

## Controls

- **Arrow Keys**: Rotate camera around target
- **Q/E**: Zoom in/out
- **1/2/3**: Change quality level (Low/Medium/High)
- **T**: Toggle multithreading
- **N**: Advance day/night cycle

## Building and Running

### Prerequisites

- Rust (latest stable version)
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Rubric Requirements

1. **Day/night cycle with visible sun** - Implemented in `src/skybox.rs` and `src/main.rs`
   - Sun/moon rendering in skybox with day_time control (N key)

2. **Multithreading** - Implemented in `src/renderer.rs`
   - 4 worker threads with std::thread (toggle with T key)

3. **Camera rotation and zoom** - Implemented in `src/camera.rs`
   - Orbital rotation with arrow keys, zoom with Q/E, safe angle/distance clamping

4. **Four materials with full parameters** - Implemented in `src/scene.rs` and `src/material.rs`
   - **Stone**: texture (stone.jpg), albedo, specular, reflectivity
   - **Glass**: texture (glass.png), albedo, specular, reflectivity, transparency (0.9), IOR (1.5)
   - **Water**: texture (water.jpeg), albedo, specular, reflectivity, transparency (0.85), IOR (1.33)
   - **Grass**: texture (grass.jpg), albedo, specular, reflectivity

5. **Reflection implementation** - Implemented in `src/renderer.rs`
   - Fresnel-based reflection with effective_reflectivity calculation

6. **3D OBJ model loading** - Implemented in `src/obj_loader.rs`
   - Axolotl mesh loaded from `assets/models/axolotl.obj`

7. **Skybox with textures** - Implemented in `src/skybox.rs`
   - Cubemap with 6 texture faces from `assets/skybox/`


## Project Structure

```
minecraft-raytracer/
├── Cargo.toml
├── README.md
├── assets/
│   ├── models/
│   │   └── axolotl.obj          
│   ├── textures/
│   │   ├── cherry_wood.jpg      
│   │   ├── cherry_leaves.png    
│   │   ├── cherry_log.png       
│   │   ├── grass.jpg            
│   │   ├── grass_side.jpg       
│   │   ├── dirt.jpg             
│   │   ├── stone.jpg            
│   │   ├── water.jpeg           
│   │   ├── glass.png            
│   │   ├── wood.png             
│   │   ├── torch.png            
│   │   └── emissive_lantern.png 
│   └── skybox/
│       ├── top.jpeg             
│       ├── bottom.jpg           
│       └── side.jpeg            
└── src/
    ├── main.rs          - Game loop and window management
    ├── camera.rs        - Orbital camera controls
    ├── ray.rs           - Ray structure and operations
    ├── color.rs         - Color struct with conversions
    ├── material.rs      - Surface materials
    ├── texture.rs       - Texture loading and sampling
    ├── intersection.rs  - Ray-geometry intersection data
    ├── cube.rs          - Textured cube blocks
    ├── light.rs         - Lighting system
    ├── skybox.rs        - Skybox with day/night cycle
    ├── obj_loader.rs    - OBJ model loader (placeholder)
    ├── scene.rs         - Scene management
    ├── renderer.rs      - Ray tracing renderer
    └── utils.rs         - Vec3 and math utilities
```

## Module Responsibilities

- **main.rs**: Game loop, Raylib window initialization, keyboard input, and rendering control
- **renderer.rs**: Ray tracing system with multithreading support and recursive ray bouncing
- **ray.rs**: Ray structure with origin and direction, along with position calculation
- **intersection.rs**: Stores intersection data between rays and geometry
- **cube.rs**: Minecraft-style textured cube blocks with ray intersection
- **obj_loader.rs**: OBJ model loader for meshes (placeholder implementation)
- **scene.rs**: Scene management and diorama building
- **material.rs**: Surface materials with albedo, reflectivity, emissive, and refractive properties
- **texture.rs**: Texture loading and UV sampling (placeholder with checkerboard pattern)
- **color.rs**: Color structure with arithmetic operations and raylib conversion
- **camera.rs**: Orbital camera with rotation, vertical movement, and zoom controls
- **light.rs**: Directional and point lights
- **skybox.rs**: Cubemap skybox with day/night cycle and sun/moon rendering
- **utils.rs**: Vec3 math library with dot, cross, normalization, reflection, and refraction

## Implementation Status

### Completed
- Core ray tracing engine with Fresnel effects
- Voxel cube rendering
- Material system (albedo, reflective, transparent with Fresnel)
- Orbital camera system with crash-safe clamping
- Multithreading support (4 threads)
- Day/night cycle with visible sun and moon
- Cherry tree diorama scene with pond, house, and axolotl
- Quality level adjustments with auto-performance scaling
- Water with realistic transparency and reflection
- **Skybox with texture cubemap**
- **OBJ model loading (axolotl mesh)**

## Performance Notes

- Quality levels adjust render scale:
  - Low: 4x downscale
  - Medium: 2x downscale
  - High: 1x (native resolution)

- Multithreading uses 4 worker threads
- Maximum ray bounce depth: 8



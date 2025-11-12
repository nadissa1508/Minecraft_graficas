# Minecraft Raytracer - Diorama

A Minecraft-style diorama renderer using ray tracing, implemented in Rust with the Raylib library for window and display management.

## Features

- Real-time ray tracing with reflections and refractions
- Minecraft-style voxel rendering
- Cherry tree diorama scene
- Dynamic day/night cycle
- Adjustable quality levels (Low, Medium, High)
- Multithreading support for improved performance
- Interactive orbital camera controls
- Material system with:
  - Textured surfaces
  - Emissive blocks (lanterns)
  - Transparent/refractive materials (water, glass)
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

## Project Structure

```
minecraft-raytracer/
├── Cargo.toml
├── README.md
├── assets/
│   ├── models/
│   │   └── axolotl.obj          (TODO)
│   ├── textures/
│   │   ├── cherry_wood.png      (TODO)
│   │   ├── cherry_leaves.png    (TODO)
│   │   ├── grass.png            (TODO)
│   │   ├── stone.png            (TODO)
│   │   ├── water.png            (TODO)
│   │   ├── glass.png            (TODO)
│   │   └── emissive_lantern.png (TODO)
│   └── skybox/
│       ├── right.png            (TODO)
│       ├── left.png             (TODO)
│       ├── top.png              (TODO)
│       ├── bottom.png           (TODO)
│       ├── front.png            (TODO)
│       └── back.png             (TODO)
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
- **skybox.rs**: Gradient skybox with day/night cycle blending
- **utils.rs**: Vec3 math library with dot, cross, normalization, reflection, and refraction

## Implementation Status

### Completed
- Core ray tracing engine
- Voxel cube rendering
- Material system (albedo, reflective, emissive, transparent)
- Orbital camera system
- Multithreading support
- Day/night cycle
- Cherry tree diorama scene
- Quality level adjustments

### TODO
- Load actual texture files (currently uses placeholder checkerboard)
- Load OBJ models (currently uses placeholder pyramid)
- Implement skybox cubemap loading
- Add anti-aliasing
- Optimize ray-cube intersection
- Add more block types
- Implement BVH acceleration structure

## Performance Notes

- Quality levels adjust render scale:
  - Low: 4x downscale
  - Medium: 2x downscale
  - High: 1x (native resolution)

- Multithreading uses 4 worker threads
- Maximum ray bounce depth: 5

## License

This project is provided as-is for educational purposes.

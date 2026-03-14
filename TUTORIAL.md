# Vulkan Practice — Module Roadmap & Integration Guide

## Current Type Inventory

| Module | Types |
|--------|-------|
| **geometry** | `Position`, `Vertex`, `Pixel`, `Mesh`, `Matrix` (incomplete) |
| **physics** | `Velocity`, `Acceleration`, `Force`, `Wave`, `VectorOps` trait |
| **optical** | `RGBColor`, `RGBAColor`, `Light`, `DirectionalLight`, `PointLight`, `Ray`, `Beam`, `PolarizationState` |
| **material** | `Material` |
| **math** | Constants (`PI`, `TWO_PI`, `EPSILON`), placeholders for `matrices` and `quaternions` |

---

## Suggested Additions Per Module

### geometry

- **Transform** `struct` — position (`Position`) + rotation (`Quat`) + scale (`Vec3`). The universal "where is this thing and how is it oriented." Every renderable entity needs one.
  - `to_mat4() -> Mat4` method — composes the model matrix via `Mat4::from_scale_rotation_translation`
  - `forward() -> Vec3`, `right() -> Vec3`, `up() -> Vec3` methods — local axes derived from rotation
- **AABB** `struct` — `min: Vec3`, `max: Vec3`. Axis-aligned bounding box for frustum culling, broadphase collision, spatial partitioning.
  - `contains_point(point) -> bool` method
  - `intersects_aabb(other) -> bool` method
  - `from_mesh(mesh) -> AABB` associated function — compute bounds from vertex data
- **Sphere** `struct` — `center: Position`, `radius: f32`. Simplest bounding volume, good for distance checks.
  - `contains_point(point) -> bool` method
  - `intersects_sphere(other) -> bool` method
  - `intersects_ray(ray) -> Option<f32>` method
- **Plane** `struct` — `normal: Vec3`, `distance: f32`. Half-space representation, needed for frustum planes and reflection.
  - `from_point_and_normal(point, normal) -> Plane` associated function
  - `from_three_points(a, b, c) -> Plane` associated function
  - `signed_distance(point) -> f32` method
  - `intersect_ray(ray) -> Option<(f32, Position)>` method
  - `classify_point(point) -> Side` method
- **Side** `enum` — `Front`, `Back`, `On`. Result of spatial classification against a `Plane`.
- **Frustum** `struct` — `planes: [Plane; 6]`. Camera view volume for culling objects before they ever hit the GPU.
  - `from_view_projection(mat4) -> Frustum` associated function — extract planes from VP matrix
  - `contains_point(point) -> bool` method
  - `contains_sphere(sphere) -> bool` method
  - `contains_aabb(aabb) -> bool` method
- **Edge** `struct` — `start: u32`, `end: u32` (vertex indices). For adjacency queries and wireframe rendering.
- **Face** `struct` — `indices: [u32; 3]`, `normal: Vec3`. For per-face operations, CSG, subdivision.

### physics

- **RigidBody** `struct` — `mass: f32`, `inertia: Mat3`, `velocity: Velocity`, `angular_velocity: Vec3`, `forces: Vec<Force>`, `transform: Transform`. The thing that moves.
  - `apply_force(force)` method — accumulates into force list
  - `integrate(dt: f32)` method — steps position/rotation forward
  - `clear_forces()` method — called after each integration step
- **Impulse** `struct` — `vector: Vec3`, `point: Option<Position>`. Instantaneous change in momentum (collision response, explosions).
  - `apply_to(body: &mut RigidBody)` method
- **Momentum** `struct` — `linear: Vec3`, `angular: Vec3`. Conserved quantities useful for stable simulation.
  - `from_body(body: &RigidBody) -> Momentum` associated function
- **Damping** `struct` — `linear: f32`, `angular: f32`. Drag coefficients that prevent runaway velocities.
  - `apply_to(velocity: &mut Velocity, angular: &mut Vec3, dt: f32)` method
- **Constraint** `trait` — connects rigid bodies with limits.
  - `fn solve(&self, bodies: &mut [RigidBody], dt: f32)` method
- **DistanceConstraint** `struct` — implements `Constraint`. Keeps two bodies at fixed distance.
- **HingeConstraint** `struct` — implements `Constraint`. Allows rotation around one axis.
- **Integrator** `trait` — `fn step(&mut self, body: &mut RigidBody, dt: f32)`. Abstracts Euler, Verlet, RK4 behind one interface so you can swap freely.
- **EulerIntegrator** `struct` — implements `Integrator`. Simplest, least stable.
- **VerletIntegrator** `struct` — implements `Integrator`. Good for constraints and cloth.

### optical

- **SpotLight** `struct` — `position: Position`, `direction: Vec3`, `inner_cone: f32`, `outer_cone: f32`, `color: RGBColor`, `intensity: f32`, `falloff_rate: f32`. The missing light type between point and directional.
- **AmbientLight** `struct` — `intensity: f32`, `color: RGBColor`. Global fill that prevents pure-black shadows.
- **LightAttenuation** `struct` — `constant: f32`, `linear: f32`, `quadratic: f32`. Reusable across point and spot lights.
  - `attenuate(distance: f32) -> f32` method — returns the multiplier at a given distance
- **Spectrum** `struct` — `samples: Vec<(f32, f32)>` (wavelength, intensity pairs). If your `Wave` type is meant to be physically grounded, this is the container for multi-wavelength data.
  - `sample_at(wavelength: f32) -> f32` method — interpolated lookup
  - `to_rgb() -> RGBColor` method — convert visible spectrum to display color
- `reflect_ray(ray, normal) -> Ray` free function — reflection via `r = d - 2(d . n)n`
- `refract_ray(ray, normal, eta) -> Option<Ray>` free function — Snell's law, returns `None` for total internal reflection
- `fresnel_schlick(cos_theta, f0) -> f32` free function — Fresnel approximation for blend between reflection/refraction

### material

- **Texture** `struct` — `width: u32`, `height: u32`, `format: TextureFormat`, `data: Vec<u8>`. The raw image a shader samples.
  - `from_file(path) -> Result<Texture>` associated function
  - `pixel_at(x, y) -> RGBAColor` method
- **TextureFormat** `enum` — `RGBA8`, `RGB8`, `R8`, `RGBA16F`. Maps to `VkFormat` variants.
- **Sampler** `struct` — `filter: FilterMode`, `address_mode: AddressMode`, `max_anisotropy: f32`, `mip_levels: u32`. Tells the GPU *how* to read the texture.
- **FilterMode** `enum` — `Nearest`, `Linear`, `Anisotropic`. Maps to `VkFilter`.
- **AddressMode** `enum` — `Repeat`, `Clamp`, `Mirror`. Maps to `VkSamplerAddressMode`.
- **PBRMaterial** `struct` — `albedo: RGBAColor`, `metallic: f32`, `roughness: f32`, `normal_map: Option<usize>`, `ao_map: Option<usize>`, `emissive: RGBColor`. Industry-standard physically-based parameters that map directly to a PBR shader.
  - `#[repr(C)]` — the numeric fields pack into a UBO
- **MaterialInstance** `struct` — `base: usize` (index into `PBRMaterial` table), `tint: RGBAColor`, `uv_offset: [f32; 2]`, `uv_scale: [f32; 2]`. Lets many objects share one material with slight variations.
- **ShaderBinding** `struct` — `descriptor_set: u32`, `binding_index: u32`, `material_id: usize`. Associates a material with descriptor set layout indices. The bridge between your material data and the GPU pipeline.

### math

- `model_matrix(position, rotation, scale) -> Mat4` free function — wraps `glam::Mat4::from_scale_rotation_translation`. Lives in `matrices.rs`.
- `normal_matrix(model: &Mat4) -> Mat4` free function — `model.inverse().transpose()`. Needed for correct lighting after non-uniform scale.
- `perspective_matrix(fov, aspect, near, far) -> Mat4` free function — wraps `glam::Mat4::perspective_rh`. Lives in `matrices.rs`.
- `orthographic_matrix(left, right, bottom, top, near, far) -> Mat4` free function — wraps `glam::Mat4::orthographic_rh`.
- `look_at(eye, target, up) -> Mat4` free function — wraps `glam::Mat4::look_at_rh`.
- `lerp(a: f32, b: f32, t: f32) -> f32` free function — linear interpolation. Lives in a new `interpolation.rs`.
- `slerp(a: Quat, b: Quat, t: f32) -> Quat` free function — spherical linear interpolation for smooth rotation blending.
- `smoothstep(edge0: f32, edge1: f32, x: f32) -> f32` free function — Hermite interpolation, matches the GLSL built-in.
- `ease_in_out(t: f32) -> f32` free function — cubic ease for animation curves.
- **Noise** `struct` — `seed: u64`. Procedural generation for terrain, textures, particle effects. Lives in a new `noise.rs`.
  - `perlin_2d(x, y) -> f32` method
  - `perlin_3d(x, y, z) -> f32` method
  - `simplex_2d(x, y) -> f32` method

---

## Vulkan Buffers You Need

Vulkan doesn't know about your Rust structs. It sees raw bytes in GPU memory. Your job is to fill buffers with data laid out exactly as your shaders expect. Here are the buffers a typical renderer needs:

### 1. Vertex Buffer

Holds per-vertex attributes. Your `Vertex` struct maps directly:

```
┌────────────────────────────────────────────────────────┐
│  position (vec3)  │  normal (vec3)  │  uv (vec2)       │
│  12 bytes         │  12 bytes       │  8 bytes          │
├────────────────────────────────────────────────────────┤
│  vertex 0         │  vertex 1       │  vertex 2  │ ...  │
└────────────────────────────────────────────────────────┘
```

- **Vulkan type**: `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT`
- **Rust side**: `&[Vertex]` cast to `&[u8]` via `bytemuck` or `std::slice::from_raw_parts`
- **Binding**: `vkCmdBindVertexBuffers` before draw calls
- **Shader input**: `layout(location = 0) in vec3 position;` etc.

### 2. Index Buffer

Triangle connectivity. Your `Mesh.indices` maps directly:

- **Vulkan type**: `VK_BUFFER_USAGE_INDEX_BUFFER_BIT`
- **Rust side**: `&[u32]` → `VK_INDEX_TYPE_UINT32`
- **Binding**: `vkCmdBindIndexBuffer`
- **Why**: Reuses vertices. A cube has 8 vertices but 36 indices (6 faces × 2 triangles × 3).

### 3. Uniform Buffers (UBOs)

Small, frequently-updated data read by every vertex/fragment. Typically:

**Per-frame UBO** (updated once per frame):
```rust
#[repr(C)]
struct FrameUniforms {
    view: Mat4,        // 64 bytes
    projection: Mat4,  // 64 bytes
    camera_pos: Vec4,  // 16 bytes (vec3 padded to vec4)
    time: f32,         // 4 bytes
    _pad: [f32; 3],    // 12 bytes padding to align to 16
}
```

**Per-object UBO** (updated per draw call):
```rust
#[repr(C)]
struct ObjectUniforms {
    model: Mat4,       // 64 bytes
    normal_matrix: Mat4, // 64 bytes (inverse transpose of model)
}
```

**Light UBO**:
```rust
#[repr(C)]
struct LightUniforms {
    dir_light_direction: Vec4,    // 16 bytes
    dir_light_color: Vec4,        // 16 bytes
    point_lights: [PointLightGPU; 4],
    num_active_lights: u32,       // 4 bytes
    _pad: [u32; 3],               // 12 bytes
}

#[repr(C)]
struct PointLightGPU {
    position: Vec4,    // 16 bytes (vec3 + falloff packed in w)
    color: Vec4,       // 16 bytes (rgb + intensity packed in w)
}
```

- **Vulkan type**: `VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT`
- **Binding**: descriptor sets, `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`

### 4. Staging Buffers

CPU-visible memory used to transfer data to GPU-local memory. You write your vertex/index/texture data here first, then `vkCmdCopyBuffer` to the final device-local buffer.

- **Vulkan type**: `VK_BUFFER_USAGE_TRANSFER_SRC_BIT`
- **Memory**: `VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | HOST_COHERENT_BIT`

### 5. Image / Texture Buffers

Not technically "buffers" — they're `VkImage` objects:

- **Vulkan type**: `VK_IMAGE_USAGE_SAMPLED_BIT | TRANSFER_DST_BIT`
- **Combined with**: `VkImageView` + `VkSampler`
- **Descriptor type**: `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
- **Formats**: `VK_FORMAT_R8G8B8A8_SRGB` for color textures, `VK_FORMAT_R8G8B8A8_UNORM` for data textures (normal maps)

### 6. Depth Buffer

A `VkImage` used as the depth attachment:

- **Format**: `VK_FORMAT_D32_SFLOAT` or `VK_FORMAT_D24_UNORM_S8_UINT` (with stencil)
- **Usage**: `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
- **Matches**: your `Pixel.depth` field conceptually

### 7. Storage Buffers (SSBOs) — later

For compute shaders, particle systems, or GPU-driven rendering. Larger and more flexible than UBOs but slower random access.

- **Vulkan type**: `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT`
- **Descriptor type**: `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`

---

## What Vulkan Expects: Type Alignment Rules

GLSL/SPIR-V std140/std430 layout rules **do not match C struct packing**. Key rules:

| GLSL type | Size | Alignment | Rust equivalent |
|-----------|------|-----------|-----------------|
| `float` | 4 | 4 | `f32` |
| `vec2` | 8 | 8 | `[f32; 2]` |
| `vec3` | 12 | **16** | `Vec3` — **must pad to 16 bytes** |
| `vec4` | 16 | 16 | `Vec4` or `[f32; 4]` |
| `mat4` | 64 | 16 | `Mat4` or `[[f32; 4]; 4]` |
| `int` | 4 | 4 | `i32` |
| `uint` | 4 | 4 | `u32` |

**The vec3 trap**: A `vec3` in a UBO/SSBO aligns to 16 bytes, not 12. Either use `Vec4` with an unused `.w` component, or explicitly pad:

```rust
// BAD — the GPU will read wrong data
#[repr(C)]
struct Bad {
    position: [f32; 3],  // 12 bytes, next field starts at offset 12
    intensity: f32,      // GPU expects this at offset 16!
}

// GOOD — explicit padding
#[repr(C)]
struct Good {
    position: [f32; 3],
    _pad: f32,           // pushes intensity to offset 16
    intensity: f32,
}

// BETTER — just use vec4
#[repr(C)]
struct Better {
    position_and_intensity: [f32; 4], // xyz = position, w = intensity
}
```

---

## Pseudocode: Tying Your Modules to a Vulkan Instance

This shows how your existing abstractions connect to `ash` API calls. Not compilable — meant to show the data flow.

```
── INITIALIZATION ──────────────────────────────────────

entry       = ash::Entry::load()
instance    = entry.create_instance(app_info, extensions)
surface     = create_surface(instance, window)        // via winit + ash-window
phys_device = pick_physical_device(instance, surface)
device      = create_logical_device(instance, phys_device, queue_families)
swapchain   = create_swapchain(device, surface, extent)
render_pass = create_render_pass(device, swapchain.format)
depth_image = create_depth_image(device, extent, VK_FORMAT_D32_SFLOAT)
framebuffers = create_framebuffers(device, render_pass, swapchain.views, depth_image.view)

── PIPELINE SETUP ──────────────────────────────────────

// Your Vertex struct defines the pipeline's vertex input
vertex_binding = VkVertexInputBindingDescription {
    binding: 0,
    stride:  size_of::<Vertex>(),       // 32 bytes (position + normal + uv)
    rate:    VERTEX,
}
vertex_attributes = [
    { location: 0, format: R32G32B32_SFLOAT, offset: offset_of!(Vertex, position) },
    { location: 1, format: R32G32B32_SFLOAT, offset: offset_of!(Vertex, normal)   },
    { location: 2, format: R32G32_SFLOAT,    offset: offset_of!(Vertex, uv)       },
]

descriptor_layout = create_descriptor_set_layout(device, [
    { binding: 0, type: UNIFORM_BUFFER, stage: VERTEX },          // FrameUniforms
    { binding: 1, type: UNIFORM_BUFFER, stage: VERTEX },          // ObjectUniforms
    { binding: 2, type: UNIFORM_BUFFER, stage: FRAGMENT },        // LightUniforms
    { binding: 3, type: COMBINED_IMAGE_SAMPLER, stage: FRAGMENT }, // Material texture
])

pipeline = create_graphics_pipeline(device, render_pass, descriptor_layout,
    vert_shader, frag_shader, vertex_binding, vertex_attributes)

── RESOURCE UPLOAD ─────────────────────────────────────

// Your Mesh data → GPU buffers
mesh = Mesh::new(vertices, indices, material_ids, materials)

staging_buf  = create_buffer(device, HOST_VISIBLE, TRANSFER_SRC, mesh.vertices.as_bytes())
vertex_buf   = create_buffer(device, DEVICE_LOCAL, VERTEX_BUFFER | TRANSFER_DST, size)
copy_buffer(cmd, staging_buf, vertex_buf)

staging_buf  = create_buffer(device, HOST_VISIBLE, TRANSFER_SRC, mesh.indices.as_bytes())
index_buf    = create_buffer(device, DEVICE_LOCAL, INDEX_BUFFER | TRANSFER_DST, size)
copy_buffer(cmd, staging_buf, index_buf)

// Your Material → texture + sampler
for material in mesh.materials:
    if material.texture_id.is_some():
        image     = load_image_from_disk(texture_path)
        vk_image  = create_image(device, RGBA8_SRGB, image.width, image.height)
        upload_image_via_staging(cmd, image.pixels, vk_image)
        vk_view   = create_image_view(device, vk_image)
        vk_sampler = create_sampler(device, LINEAR, REPEAT)

// Your lights → uniform buffer
light_data = LightUniforms {
    dir_light_direction: vec4(dir_light.direction, 0.0),
    dir_light_color:     vec4(dir_light.color.r, .g, .b, dir_light.intensity),
    point_lights:        convert_point_lights(scene.point_lights),
    num_active_lights:   scene.point_lights.len(),
}
light_ubo = create_buffer(device, HOST_VISIBLE | HOST_COHERENT, UNIFORM_BUFFER,
                           &light_data as bytes)

── FRAME LOOP ──────────────────────────────────────────

loop {
    // Physics step — uses your physics module
    for body in scene.bodies:
        body.velocity.apply_acceleration(gravity, dt)
        body.transform.position += body.velocity.vector * dt

    // Camera — uses your math module
    view       = mat4_look_at(camera.position, camera.target, UP)
    projection = mat4_perspective(fov, aspect, near, far)

    // Update per-frame UBO
    map_memory(frame_ubo, &FrameUniforms { view, projection, camera_pos, time })

    // Acquire swapchain image
    image_index = acquire_next_image(device, swapchain, semaphore)

    // Record command buffer
    begin_render_pass(cmd, render_pass, framebuffers[image_index], clear_color, clear_depth)
    bind_pipeline(cmd, pipeline)

    for object in scene.objects:
        // Update per-object UBO — uses your geometry::Transform
        map_memory(object_ubo, &ObjectUniforms {
            model: object.transform.to_mat4(),
            normal_matrix: object.transform.to_mat4().inverse().transpose(),
        })

        // Bind descriptor set (uniforms + textures for this object's material)
        bind_descriptor_sets(cmd, pipeline_layout, object.descriptor_set)

        // Draw — uses your Mesh data already on the GPU
        bind_vertex_buffers(cmd, [object.vertex_buf])
        bind_index_buffer(cmd, object.index_buf, UINT32)
        draw_indexed(cmd, object.mesh.indices.len(), 1, 0, 0, 0)

    end_render_pass(cmd)

    // Submit and present
    queue_submit(graphics_queue, cmd, wait: image_available, signal: render_finished)
    queue_present(present_queue, swapchain, image_index, wait: render_finished)
}
```

### How your modules map to this flow:

| Your module | Where it lives in the pipeline |
|---|---|
| `geometry::Vertex` | Vertex buffer layout, `VkVertexInputAttributeDescription` |
| `geometry::Mesh` | Vertex + index buffer data |
| `geometry::Position` | Inside `Vertex`, inside `Transform` (once you add it) |
| `geometry::Pixel` | Conceptual framebuffer output — the GPU writes these |
| `material::Material` | Descriptor set bindings (textures, colors → fragment shader UBO) |
| `optical::DirectionalLight` | Light UBO, fragment shader |
| `optical::PointLight` | Light UBO, fragment shader |
| `optical::RGBColor/RGBAColor` | Packed into UBOs as `vec3`/`vec4`, clear values |
| `physics::Velocity/Force/Accel` | CPU-side simulation loop, updates `Transform` each frame |
| `math::Mat4/Quat` | View, projection, model matrices → UBOs |

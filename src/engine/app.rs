// Application lifecycle
//
// This ties together all the Vulkan init pieces in the correct order.
// Creation order matters — each step depends on the previous ones.
// Destruction must happen in reverse order.
//
// CREATION ORDER:
//   1. Entry          — loads the Vulkan library
//   2. Instance       — connection to Vulkan, enables layers/extensions
//   3. Debug messenger — validation callback (debug builds only)
//   4. Surface        — window's drawable area (needs ash-window)
//   5. Physical device — pick a GPU
//   6. Logical device  — your handle to the GPU, creates queues
//   --- everything above is done ---
//   --- below is what's needed next for rendering ---
//   7. Swapchain      — chain of images to render into and present
//   8. Image views    — how to interpret each swapchain image
//   9. Render pass    — describes attachments (color, depth) and subpasses
//  10. Pipeline layout — descriptor set layouts, push constant ranges
//  11. Graphics pipeline — shaders, vertex input, rasterization state
//  12. Framebuffers   — one per swapchain image, references render pass
//  13. Command pool   — allocator for command buffers
//  14. Vertex buffer  — upload mesh data to GPU (staging → device-local)
//  15. Command buffers — record draw commands
//  16. Sync objects   — semaphores (GPU-GPU) + fences (GPU-CPU)
//
// RENDER LOOP (per frame):
//   1. Wait for previous frame's fence
//   2. Acquire next swapchain image (signals image_available semaphore)
//   3. Record command buffer: begin render pass, bind pipeline,
//      bind vertex/index buffers, bind descriptor sets, draw, end
//   4. Submit command buffer (waits on image_available, signals render_finished)
//   5. Present (waits on render_finished)
//
// DESTRUCTION ORDER (reverse of creation):
//  16. Sync objects
//  15. Command buffers (freed with pool)
//  14. Vertex buffer
//  13. Command pool
//  12. Framebuffers
//  11. Graphics pipeline
//  10. Pipeline layout
//   9. Render pass
//   8. Image views
//   7. Swapchain
//   6. Logical device
//   5. (physical device has no destroy call)
//   4. Surface
//   3. Debug messenger
//   2. Instance
//   1. (entry has no destroy call)

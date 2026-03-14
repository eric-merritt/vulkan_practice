// Surface creation
//
// A VkSurfaceKHR is the bridge between Vulkan and your window system.
// It represents the drawable area of your window. You need it to:
// - Check which queue families can present to the screen
// - Query supported swapchain formats and present modes
// - Create the swapchain itself
//
// Surface creation is platform-specific (X11, Wayland, Win32, Metal).
// ash-window handles this automatically using raw-window-handle.

// To create a surface from a winit window:
//
//   use ash_window;
//   use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
//
//   let surface = unsafe {
//       ash_window::create_surface(
//           &entry,
//           &instance,
//           window.display_handle()?.as_raw(),
//           window.window_handle()?.as_raw(),
//           None,
//       )?
//   };
//
//   let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
//
// The surface_loader provides methods for querying surface capabilities,
// formats, and present modes — all needed before creating a swapchain.
//
// Cleanup (in destroy, in reverse creation order):
//   surface_loader.destroy_surface(surface, None);

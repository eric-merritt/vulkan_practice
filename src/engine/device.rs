// Physical and logical device selection
//
// Physical device: the actual GPU hardware. You query available GPUs
// and pick one that supports the features you need (graphics queue,
// surface presentation, swapchain extension).
//
// Logical device: your application's handle to the GPU. You create
// queues from it and use it for all subsequent Vulkan operations
// (buffers, pipelines, command pools, etc).

use ash::vk;

use super::queue::QueueFamilyIndices;
use super::debug::{VALIDATION_ENABLED, VALIDATION_LAYER_NAME};

// Picks the first suitable physical device (GPU).
// A device is suitable if it has queue families for graphics + present.
pub unsafe fn pick_physical_device(
    instance: &ash::Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> Option<(vk::PhysicalDevice, QueueFamilyIndices)> {
    let devices = unsafe {
        instance
            .enumerate_physical_devices()
            .unwrap_or_default()
    };

    for device in devices {
        let indices = unsafe {
            QueueFamilyIndices::find(instance, device, surface_loader, surface)
        };

        if let Some(indices) = indices {
            let props = unsafe { instance.get_physical_device_properties(device) };
            let name = unsafe {
                std::ffi::CStr::from_ptr(props.device_name.as_ptr())
            };
            log::info!("Selected physical device: {:?}", name);
            return Some((device, indices));
        }
    }

    None
}

// Creates a logical device with graphics and present queues.
// The logical device is what you use for all GPU operations.
pub unsafe fn create_logical_device(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    indices: &QueueFamilyIndices,
) -> Result<ash::Device, vk::Result> {
    let queue_priorities = [1.0_f32];

    let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = indices
        .unique_families()
        .iter()
        .map(|&family| {
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(family)
                .queue_priorities(&queue_priorities)
        })
        .collect();

    let device_features = vk::PhysicalDeviceFeatures::default();

    // Swapchain extension is required for presenting to a surface
    let device_extensions = [ash::khr::swapchain::NAME.as_ptr()];

    let layer_names: Vec<*const i8> = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER_NAME.as_ptr()]
    } else {
        vec![]
    };

    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&device_features)
        .enabled_extension_names(&device_extensions)
        .enabled_layer_names(&layer_names);

    unsafe { instance.create_device(physical_device, &create_info, None) }
}

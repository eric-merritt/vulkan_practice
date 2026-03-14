// Queue family detection
//
// Vulkan devices expose queue families — groups of queues that
// support specific operations (graphics, compute, transfer, present).
//
// For rendering you need at minimum:
// - A graphics queue (draws triangles)
// - A present queue (puts images on screen)
//
// Often these are the same queue family, but they don't have to be.

use ash::vk;

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
}

impl QueueFamilyIndices {
    // Finds queue families that support graphics and presentation.
    // Returns None if the device can't do both.
    pub unsafe fn find(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_loader: &ash::khr::surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> Option<Self> {
        let properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut graphics = None;
        let mut present = None;

        for (index, family) in properties.iter().enumerate() {
            let index = index as u32;

            if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                graphics = Some(index);
            }

            let present_support = unsafe {
                surface_loader
                    .get_physical_device_surface_support(physical_device, index, surface)
                    .unwrap_or(false)
            };

            if present_support {
                present = Some(index);
            }

            // Prefer a family that does both
            if graphics.is_some() && present.is_some() {
                break;
            }
        }

        match (graphics, present) {
            (Some(g), Some(p)) => Some(Self {
                graphics: g,
                present: p,
            }),
            _ => None,
        }
    }

    // Returns the unique set of queue family indices.
    // If graphics and present are the same family, returns one index.
    pub fn unique_families(&self) -> Vec<u32> {
        if self.graphics == self.present {
            vec![self.graphics]
        } else {
            vec![self.graphics, self.present]
        }
    }
}

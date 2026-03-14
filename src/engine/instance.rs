// Vulkan instance creation
//
// The instance is the connection between your application and the
// Vulkan library. Creating it involves specifying your app info,
// which validation layers to enable, and which extensions you need
// (e.g. surface extensions for windowing, debug utils for validation).
//
// This is always the first Vulkan object you create and the last
// one you destroy.

use ash::vk;
use ash::Entry;

use super::debug::{self, VALIDATION_ENABLED, VALIDATION_LAYER_NAME};

use std::ffi::CStr;

const APP_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"Vulkan Practice\0") };
const ENGINE_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"No Engine\0") };

pub unsafe fn create_instance(
    entry: &Entry,
    required_extensions: &[*const i8],
) -> Result<ash::Instance, ash::vk::Result> {
    let app_info = vk::ApplicationInfo::default()
        .application_name(APP_NAME)
        .application_version(vk::make_api_version(0, 1, 0, 0))
        .engine_name(ENGINE_NAME)
        .engine_version(vk::make_api_version(0, 1, 0, 0))
        .api_version(vk::API_VERSION_1_3);

    // Collect extensions: window surface + debug utils if validation is on
    let mut extensions = required_extensions.to_vec();
    if VALIDATION_ENABLED {
        extensions.push(ash::ext::debug_utils::NAME.as_ptr());
    }

    // Validation layers
    let layer_names: Vec<*const i8> = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER_NAME.as_ptr()]
    } else {
        vec![]
    };

    let mut debug_info = debug::debug_messenger_create_info();

    let mut create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&extensions)
        .enabled_layer_names(&layer_names);

    if VALIDATION_ENABLED {
        create_info = create_info.push_next(&mut debug_info);
    }

    unsafe { entry.create_instance(&create_info, None) }
}

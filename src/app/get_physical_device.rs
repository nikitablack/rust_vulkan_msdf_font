use ash::version::InstanceV1_0;
use ash::vk;

fn check_required_device_extensions(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    required_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<(), String> {
    let supported_device_extensions =
        match unsafe { instance.enumerate_device_extension_properties(physical_device) } {
            Ok(props) => props,
            Err(_) => {
                return Err(String::from(
                    "failed to enumerate instance extension properies",
                ))
            }
        };

    let mut supported_device_extensions_set = std::collections::HashSet::new();
    for vk::ExtensionProperties { extension_name, .. } in &supported_device_extensions {
        supported_device_extensions_set
            .insert(unsafe { std::ffi::CStr::from_ptr(extension_name.as_ptr()) });
    }

    for extension_name in required_extensions {
        if !supported_device_extensions_set.contains(extension_name) {
            return Err(format!(
                "device extension {:?} is not supported",
                extension_name
            ));
        }
    }

    Ok(())
}

fn check_device_suitability(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    required_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<(), String> {
    let properties = unsafe { instance.get_physical_device_properties(physical_device) };

    if vk::version_major(properties.api_version) < 1
        && vk::version_minor(properties.api_version) < 1
    {
        return Err(format!(
            "the device {:?} does not support API version 1.1.0",
            unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
        ));
    }

    if properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
        return Err(format!("the device {:?} is not a discrete GPU", unsafe {
            std::ffi::CStr::from_ptr(properties.device_name.as_ptr())
        }));
    }

    let features = unsafe { instance.get_physical_device_features(physical_device) };

    if features.tessellation_shader == 0 {
        return Err(format!(
            "the device {:?} does not support tesselation shader",
            unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
        ));
    }

    if features.fill_mode_non_solid == 0 {
        return Err(format!(
            "the device {:?} does not support fill mode non solid",
            unsafe { std::ffi::CStr::from_ptr(properties.device_name.as_ptr()) }
        ));
    }

    check_required_device_extensions(instance, physical_device, required_extensions)?;

    Ok(())
}

pub fn get_physical_device(
    instance: &ash::Instance,
    req_device_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<vk::PhysicalDevice, String> {
    let devices = match unsafe { instance.enumerate_physical_devices() } {
        Ok(devices) => devices,
        Err(_) => return Err(String::from("failed to enumerate physical devices")),
    };

    for device in devices {
        if let Err(_) = check_device_suitability(instance, device, req_device_extensions) {
            continue;
        }

        return Ok(device);
    }

    Err(String::from("failed to find suitable device"))
}

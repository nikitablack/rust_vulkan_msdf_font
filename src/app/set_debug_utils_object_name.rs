use ash::extensions::ext;
use ash::vk;

pub fn set_debug_utils_object_name<T: vk::Handle>(
    debug_utils_loader: &ext::DebugUtils,
    device: vk::Device,
    object_handle: T,
    object_name: String,
) {
    let name_cstr = std::ffi::CString::new(object_name).expect("wrong string parameter");

    let name_info = vk::DebugUtilsObjectNameInfoEXT::builder()
        .object_type(T::TYPE)
        .object_handle(object_handle.as_raw())
        .object_name(&name_cstr);

    let _ = unsafe { debug_utils_loader.debug_utils_set_object_name(device, &name_info) };
}

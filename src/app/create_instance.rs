use ash::version::EntryV1_0;
use ash::vk;

pub fn create_instance(
    entry: &ash::Entry,
    req_inst_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<ash::Instance, String> {
    let app_name = std::ffi::CString::new("Teapot").unwrap();

    let extension_names_raw = req_inst_extensions
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();

    let appinfo = ash::vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk::make_version(1, 1, 0));

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo)
        .enabled_extension_names(&extension_names_raw);

    let instance = match unsafe { entry.create_instance(&create_info, None) } {
        Ok(instance) => instance,
        Err(_) => return Err(String::from("failed to create instance")),
    };

    Ok(instance)
}

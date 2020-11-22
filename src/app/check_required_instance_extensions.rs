use ash::version::EntryV1_0;
use ash::vk;

#[allow(dead_code)]
pub fn check_required_instance_extensions(
    entry: &ash::Entry,
    req_inst_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<(), String> {
    let supported_instance_extensions = match entry.enumerate_instance_extension_properties() {
        Ok(props) => props,
        Err(_) => {
            return Err(String::from(
                "failed to enumerate instance extension properies",
            ))
        }
    };

    let mut supported_instance_extensions_set = std::collections::HashSet::new();
    for vk::ExtensionProperties { extension_name, .. } in &supported_instance_extensions {
        supported_instance_extensions_set
            .insert(unsafe { std::ffi::CStr::from_ptr(extension_name.as_ptr()) });
    }

    for &extension_name in req_inst_extensions {
        if !supported_instance_extensions_set.contains(extension_name) {
            return Err(format!(
                "instance extension {:?} is not supported",
                extension_name
            ));
        }
    }

    Ok(())
}

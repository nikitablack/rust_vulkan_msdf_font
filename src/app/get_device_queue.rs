use ash::version::DeviceV1_0;

pub fn get_device_queue(device: &ash::Device, queue_family: u32) -> ash::vk::Queue {
    let queue = unsafe { device.get_device_queue(queue_family, 0) };

    queue
}

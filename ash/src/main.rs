use std::ffi::CStr;

mod app;

use ash::{
    vk::{
        ApplicationInfo, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel,
        CommandBufferUsageFlags, CommandPoolCreateFlags, CommandPoolCreateInfo, DeviceCreateInfo,
        DeviceQueueCreateInfo, InstanceCreateInfo, QueueFlags,
    },
    Entry,
};

fn main() {
    let entry = unsafe { Entry::load().expect("Failed to load vulkan") };

    let app_name = c"Vulkan Learn with rust";
    let api_version = ash::vk::make_api_version(0, 1, 4, 0);

    let app_info = ApplicationInfo::default()
        .application_name(app_name)
        .application_version(0)
        .engine_name(app_name)
        .engine_version(0)
        .api_version(api_version);

    let create_info = InstanceCreateInfo::default().application_info(&app_info);

    let instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create vulkan instance")
    };

    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };

    for (i, &physical_device) in physical_devices.iter().enumerate() {
        println!("Physical device {}:", i);
        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_str()
            .unwrap_or("Unknown");
        println!("GPU: {}: {}", i, device_name);
    }

    let device = physical_devices[0];
    let queue_families = unsafe { instance.get_physical_device_queue_family_properties(device) };

    let queue_family_index = queue_families
        .iter()
        .position(|q| q.queue_flags.contains(QueueFlags::GRAPHICS))
        .expect("No graphics queue family found") as u32;

    let queue_properties = [1.0_f32];
    let queue_create_info = DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_properties);

    let queue_create_infos = [queue_create_info];
    let device_create_info = DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

    let device = unsafe {
        instance
            .create_device(device, &device_create_info, None)
            .expect("Failed to create logical device")
    };

    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    let command_pool_create_info = CommandPoolCreateInfo::default()
        .queue_family_index(queue_family_index)
        .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

    let command_pool = unsafe {
        device
            .create_command_pool(&command_pool_create_info, None)
            .expect("Failed to create command pool")
    };

    let allocate_info = CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    let command_buffers = unsafe {
        device
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffer")
    };

    let command_buffer = command_buffers[0];

    let command_buffer_begin_info =
        CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
}

use ash::{vk, Entry};
use std::ffi::CStr;

fn main() {
    println!("=== Ash Vulkan Learning Project ===\n");

    // Load the Vulkan library
    let entry = unsafe { Entry::load().expect("Failed to load Vulkan library") };

    // Create a Vulkan instance
    let app_name = c"Ash Learn";
    let engine_name = c"No Engine";

    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name)
        .application_version(vk::make_api_version(0, 1, 0, 0))
        .engine_name(engine_name)
        .engine_version(vk::make_api_version(0, 1, 0, 0))
        .api_version(vk::API_VERSION_1_3);

    let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);

    let instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create Vulkan instance")
    };

    // Enumerate physical devices
    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };

    println!("Found {} physical device(s):\n", physical_devices.len());

    for (i, &physical_device) in physical_devices.iter().enumerate() {
        let properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_str()
            .unwrap_or("Unknown");

        let device_type = match properties.device_type {
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            vk::PhysicalDeviceType::CPU => "CPU",
            _ => "Other",
        };

        let api_version = properties.api_version;
        println!("  [{}] {} ({})", i, device_name, device_type);
        println!(
            "      Vulkan API: {}.{}.{}",
            vk::api_version_major(api_version),
            vk::api_version_minor(api_version),
            vk::api_version_patch(api_version),
        );

        // Print queue families
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        println!("      Queue families: {}", queue_families.len());
        for (qi, qf) in queue_families.iter().enumerate() {
            let mut capabilities = Vec::new();
            if qf.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                capabilities.push("Graphics");
            }
            if qf.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                capabilities.push("Compute");
            }
            if qf.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                capabilities.push("Transfer");
            }
            if qf.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING) {
                capabilities.push("Sparse Binding");
            }
            println!(
                "        [{}] count={}, capabilities=[{}]",
                qi,
                qf.queue_count,
                capabilities.join(", ")
            );
        }

        // Print memory properties
        let memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };

        println!(
            "      Memory heaps: {}",
            memory_properties.memory_heap_count
        );
        for hi in 0..memory_properties.memory_heap_count as usize {
            let heap = memory_properties.memory_heaps[hi];
            let size_mb = heap.size / (1024 * 1024);
            let is_device_local = heap.flags.contains(vk::MemoryHeapFlags::DEVICE_LOCAL);
            println!(
                "        [{}] {} MB {}",
                hi,
                size_mb,
                if is_device_local {
                    "(device local)"
                } else {
                    "(host)"
                }
            );
        }
        println!();
    }

    // Select the first physical device
    let physical_device = physical_devices[0];

    // Find a queue family that supports graphics
    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let queue_family_index = queue_families
        .iter()
        .position(|qf| qf.queue_flags.contains(vk::QueueFlags::GRAPHICS))
        .expect("No graphics queue family found") as u32;

    println!(
        "Selected queue family index {} for graphics\n",
        queue_family_index
    );

    // Create a logical device
    let queue_priorities = [1.0_f32];
    let queue_create_info = vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities);

    let queue_create_infos = [queue_create_info];
    let device_create_info =
        vk::DeviceCreateInfo::default().queue_create_infos(&queue_create_infos);

    let device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .expect("Failed to create logical device")
    };

    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    println!("Successfully created logical device and queue!");

    // Create a command pool
    let command_pool_create_info = vk::CommandPoolCreateInfo::default()
        .queue_family_index(queue_family_index)
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

    let command_pool = unsafe {
        device
            .create_command_pool(&command_pool_create_info, None)
            .expect("Failed to create command pool")
    };

    println!("Successfully created command pool!");

    // Allocate a command buffer
    let allocate_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    let command_buffers = unsafe {
        device
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffer")
    };

    let command_buffer = command_buffers[0];

    // Record and submit a simple command buffer (no-op, just to prove it works)
    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe {
        device
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        device
            .end_command_buffer(command_buffer)
            .expect("Failed to end command buffer");
    }

    let command_buffers_to_submit = [command_buffer];
    let submit_info = vk::SubmitInfo::default().command_buffers(&command_buffers_to_submit);

    let submit_infos = [submit_info];

    unsafe {
        device
            .queue_submit(queue, &submit_infos, vk::Fence::null())
            .expect("Failed to submit command buffer");

        device
            .queue_wait_idle(queue)
            .expect("Failed to wait for queue idle");
    }

    println!("Successfully recorded and submitted a command buffer!");

    // Cleanup
    println!("\nCleaning up...");
    unsafe {
        device.destroy_command_pool(command_pool, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }

    println!("\n=== Ash Vulkan setup complete! ===");
}

mod into_buffer_data;

use std::{
    mem::{size_of},
    ptr::{copy_nonoverlapping as memcpy},
    marker::{PhantomData}
};

use anyhow::{anyhow, Result};
use vulkanalia::{
    prelude::v1_0::*
};

pub use into_buffer_data::{IntoBufferData};

#[derive(Debug, Copy, Clone, Default)]
pub struct Buffer<T> where T : Copy + Clone {
    usage: vk::BufferUsageFlags,
    curr_element_count: usize,
    max_element_count: usize,
    buffer: Option<vk::Buffer>,
    buffer_memory: Option<vk::DeviceMemory>,
    phantom: PhantomData<T>
}

impl<T> Buffer<T> where T : Copy + Clone {
    pub fn new(usage: vk::BufferUsageFlags, max_element_count: usize) -> Self {
        Self {
            usage,
            curr_element_count: 0,
            max_element_count,
            buffer: None,
            buffer_memory: None,
            phantom: PhantomData
        }
    }

    fn get_memory_type_index(&self, memory: vk::PhysicalDeviceMemoryProperties, properties: vk::MemoryPropertyFlags, requirements: vk::MemoryRequirements) -> Result<u32> {
        (0..memory.memory_type_count)
            .find(|i| {
                let is_suitable = (requirements.memory_type_bits & (1 << i)) != 0;
                let memory_type = memory.memory_types[*i as usize];
                is_suitable && memory_type.property_flags.contains(properties)
            })
            .ok_or_else(|| anyhow!("Failed to find suitable memory type for buffer"))
    }

    fn allocated_buffer_size(&self) -> u64 {
        (size_of::<T>() * self.max_element_count) as u64
    }

    fn used_buffer_size(&self) -> u64 {
        (size_of::<T>() * self.curr_element_count) as u64
    }

    pub unsafe fn raw_buffer(&self) -> Option<vk::Buffer> {
        self.buffer
    }

    #[allow(unused)]
    pub unsafe fn raw_buffer_memory(&self) -> Option<vk::DeviceMemory> {
        self.buffer_memory
    }

    pub fn create(&mut self, device: &Device, memory: vk::PhysicalDeviceMemoryProperties) -> Result<()> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(self.allocated_buffer_size())
            .usage(self.usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buff: vk::Buffer;
        let requirements: vk::MemoryRequirements;
        unsafe {
            debug!("Creating vertex buffer...");
            buff = device.create_buffer(&buffer_info, None)?;
            requirements = device.get_buffer_memory_requirements(buff);
        }
        self.buffer = Some(buff);

        let memory_type_index = self.get_memory_type_index(memory, vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE, requirements)?;
        let memory_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);

        let buff_memory: vk::DeviceMemory;
        unsafe {
            buff_memory = device.allocate_memory(&memory_info, None)?;
            device.bind_buffer_memory(buff, buff_memory, 0)?;
        }
        self.buffer_memory = Some(buff_memory);

        Ok(())
    }

    pub fn set_data(&mut self, device: &Device, data: &impl IntoBufferData<T>) -> Result<()> {
        let count = data.element_count();
        if count > self.max_element_count {
            return Err(anyhow!("Can't set data from source with more elements than specified when creating buffer."));
        }
        self.curr_element_count = count;
        if count > 0 {
            unsafe {
                self.set_data_from_ptr(device, data.as_buffer_ptr(), count)
            }
        }
        else {
            Ok(())
        }
    }

    unsafe fn set_data_from_ptr(&mut self, device: &Device, data_ptr: *const T, count: usize) -> Result<()> {
        let buff_memory = self.buffer_memory.unwrap();

        let memory = device.map_memory(buff_memory, 0, self.used_buffer_size(), vk::MemoryMapFlags::empty())?;
        memcpy(data_ptr, memory.cast(), count);
        device.unmap_memory(buff_memory);

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device) {
        if let Some(buff) = self.buffer.take() {
            unsafe {
                device.destroy_buffer(buff, None);
            }
        }

        if let Some(buff_memory) = self.buffer_memory.take() {
            unsafe {
                device.free_memory(buff_memory, None);
            }
        }
    }
}

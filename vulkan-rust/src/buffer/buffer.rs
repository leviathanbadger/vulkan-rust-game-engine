use super::{IntoBufferData};

use std::{
    mem::{size_of},
    ptr::{copy_nonoverlapping as memcpy},
    marker::{PhantomData}
};
use anyhow::{anyhow, Result};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::bootstrap::{CommandPoolsInfo};

#[derive(Debug, Copy, Clone, Default)]
pub struct Buffer<T> where T : Copy + Clone {
    usage: vk::BufferUsageFlags,
    curr_element_count: usize,
    max_element_count: usize,
    buffer: Option<vk::Buffer>,
    buffer_memory: Option<vk::DeviceMemory>,

    require_submit: bool,
    staging_buffer: Option<vk::Buffer>,
    staging_buffer_memory: Option<vk::DeviceMemory>,

    phantom: PhantomData<T>
}

pub fn get_memory_type_index(memory: &vk::PhysicalDeviceMemoryProperties, properties: vk::MemoryPropertyFlags, requirements: vk::MemoryRequirements) -> Result<u32> {
    (0..memory.memory_type_count)
        .find(|i| {
            let is_suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];
            is_suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type for buffer"))
}

impl<T> Buffer<T> where T : Copy + Clone {
    pub fn new(usage: vk::BufferUsageFlags, max_element_count: usize, require_submit: bool) -> Self {
        Self {
            usage,
            curr_element_count: 0,
            max_element_count,
            buffer: None,
            buffer_memory: None,

            require_submit,
            staging_buffer: None,
            staging_buffer_memory: None,

            phantom: PhantomData
        }
    }

    fn allocated_buffer_size(&self) -> u64 {
        (size_of::<T>() * self.max_element_count) as u64
    }

    fn used_buffer_size(&self) -> u64 {
        (size_of::<T>() * self.curr_element_count) as u64
    }

    #[allow(unused)]
    pub fn allocated_element_count(&self) -> usize {
        self.max_element_count
    }

    pub fn used_element_count(&self) -> usize {
        self.curr_element_count
    }

    pub unsafe fn raw_buffer(&self) -> Option<vk::Buffer> {
        self.buffer
    }

    #[allow(unused)]
    pub unsafe fn raw_buffer_memory(&self) -> Option<vk::DeviceMemory> {
        self.buffer_memory
    }

    #[allow(unused)]
    pub unsafe fn raw_staging_buffer(&self) -> Option<vk::Buffer> {
        self.staging_buffer
    }

    #[allow(unused)]
    pub unsafe fn raw_staging_buffer_memory(&self) -> Option<vk::DeviceMemory> {
        self.staging_buffer_memory
    }

    fn create_buffer(&self, device: &Device, usage_flags: vk::BufferUsageFlags, memory_flags: vk::MemoryPropertyFlags, memory: &vk::PhysicalDeviceMemoryProperties) -> Result<(vk::Buffer, vk::DeviceMemory)> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(self.allocated_buffer_size())
            .usage(usage_flags)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let buff: vk::Buffer;
        let requirements: vk::MemoryRequirements;
        unsafe {
            buff = device.create_buffer(&buffer_info, None)?;
            requirements = device.get_buffer_memory_requirements(buff);
        }

        let memory_type_index = get_memory_type_index(memory, memory_flags, requirements)?;
        let memory_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);

        let buff_memory: vk::DeviceMemory;
        unsafe {
            //TODO: write custom memory allocator so we don't need to do this for every individual buffer
            buff_memory = device.allocate_memory(&memory_info, None)?;
            device.bind_buffer_memory(buff, buff_memory, 0)?;
        }

        Ok((buff, buff_memory))
    }

    pub fn create(&mut self, device: &Device, memory: &vk::PhysicalDeviceMemoryProperties) -> Result<()> {
        {
            let usage_flags = if self.require_submit { self.usage | vk::BufferUsageFlags::TRANSFER_DST } else { self.usage };
            let memory_flags = if self.require_submit { vk::MemoryPropertyFlags::DEVICE_LOCAL } else { vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE };
            let (buff, buff_memory) = self.create_buffer(device, usage_flags, memory_flags, memory)?;

            self.buffer = Some(buff);
            self.buffer_memory = Some(buff_memory);
        }

        if self.require_submit {
            let usage_flags = vk::BufferUsageFlags::TRANSFER_SRC;
            let memory_flags = vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE;
            let (buff, buff_memory) = self.create_buffer(device, usage_flags, memory_flags, memory)?;

            self.staging_buffer = Some(buff);
            self.staging_buffer_memory = Some(buff_memory);
        }

        Ok(())
    }

    pub fn set_data(&mut self, device: &Device, data: &impl IntoBufferData<T>) -> Result<()> {
        let is_created = if self.require_submit { self.staging_buffer_memory.is_some() } else { self.buffer_memory.is_some() };
        if !is_created {
            return Err(anyhow!("Buffer is not created. Can't call set_data until it is created."));
        }

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
        let buff_memory = if self.require_submit { self.staging_buffer_memory.unwrap() } else { self.buffer_memory.unwrap() };

        let memory = device.map_memory(buff_memory, 0, self.used_buffer_size(), vk::MemoryMapFlags::empty())?;
        memcpy(data_ptr, memory.cast(), count);
        device.unmap_memory(buff_memory);

        Ok(())
    }

    #[allow(unused)]
    pub fn submit(&self, device: &Device, command_pools: &CommandPoolsInfo) -> Result<()> {
        if !self.require_submit {
            warn!("Buffer submitted that doesn't require data to be submitted.");
            return Ok(());
        }

        command_pools.submit_command_transient_sync(device, |command_buffer| {
            self.write_submit_to_command_buffer(device, command_buffer)
        })
    }

    pub fn write_submit_to_command_buffer(&self, device: &Device, command_buffer: &vk::CommandBuffer) -> Result<()> {
        unsafe {
            let src = self.staging_buffer.unwrap();
            let dst = self.buffer.unwrap();
            let regions = vk::BufferCopy::builder()
                .size(self.used_buffer_size());
            device.cmd_copy_buffer(*command_buffer, src, dst, &[regions]);
        }

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

        if let Some(buff) = self.staging_buffer.take() {
            unsafe {
                device.destroy_buffer(buff, None);
            }
        }

        if let Some(buff_memory) = self.staging_buffer_memory.take() {
            unsafe {
                device.free_memory(buff_memory, None);
            }
        }
    }
}

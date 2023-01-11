use super::{
    resource_load_job::{ResourceLoadJob},
    ResourceLoader
};

use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    resources::{Buffer},
    app_data::{AppData}
};

#[derive(Debug)]
pub struct BufferSubmitJob {
    src_buffer: vk::Buffer,
    dst_buffer: vk::Buffer,
    used_buffer_size: u64
}

impl BufferSubmitJob {
    pub(super) fn create_for<T>(buffer: Buffer<T>) -> Self where T : Copy + Clone {
        Self {
            src_buffer: unsafe { buffer.raw_staging_buffer().unwrap() },
            dst_buffer: unsafe { buffer.raw_buffer().unwrap() },
            used_buffer_size: buffer.used_buffer_size()
        }
    }
}

impl ResourceLoadJob for BufferSubmitJob {
    fn needs_transient_command(&self) -> bool {
        true
    }

    fn load(&mut self, _resource_loader: &mut ResourceLoader, _device: &Device, _app_data: &AppData) -> anyhow::Result<()> {
        Ok(())
    }

    fn write_to_command_buffer(&self, _resource_loader: &mut ResourceLoader, device: &vulkanalia::Device, _app_data: &AppData, command_buffer: &vulkanalia::vk::CommandBuffer) -> anyhow::Result<()> {
        unsafe {
            let regions = vk::BufferCopy::builder()
                .size(self.used_buffer_size);
            device.cmd_copy_buffer(*command_buffer, self.src_buffer, self.dst_buffer, &[regions]);
        }

        Ok(())
    }
}

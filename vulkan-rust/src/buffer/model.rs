use super::{Buffer, IntoBufferData};

use core::hash::{Hash};
use std::{
    path::{Path},
    fs::{File},
    io::{BufReader},
    collections::{HashMap}
};
use anyhow::{anyhow, Result};
use nalgebra_glm as glm;
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    shader_input::push_constants::{PushConstants},
    bootstrap::{CommandPoolsInfo}
};

pub trait CanBeVertexBufferType : Copy + Clone + Hash + PartialEq + Eq + ::std::fmt::Debug {
    fn create_vertex_from_opts(pos: glm::Vec3, normal: Option<glm::Vec3>, color: Option<glm::Vec3>, uv: Option<glm::Vec2>) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Model<TVert> where TVert : CanBeVertexBufferType {
    vertex_buffer: Buffer<TVert>,
    index_buffer_16: Option<Buffer<u16>>,
    index_buffer_32: Option<Buffer<u32>>,
    index_type: vk::IndexType,
    require_submit: bool
}

impl<TVert> Model<TVert> where TVert : CanBeVertexBufferType {
    pub fn new(max_vertex_count: usize, max_index_count: usize, require_submit: bool) -> Result<Self> {
        let index_buffer_16: Option<Buffer<u16>>;
        let index_buffer_32: Option<Buffer<u32>>;
        let index_type: vk::IndexType;

        if max_vertex_count > u32::MAX as usize {
            return Err(anyhow!("Can't create model with more than {} vertices", u32::MAX));
        } else if max_vertex_count > u16::MAX as usize {
            index_buffer_16 = None;
            index_buffer_32 = Some(Buffer::new(vk::BufferUsageFlags::INDEX_BUFFER, max_index_count, require_submit));
            index_type = vk::IndexType::UINT32;
        } else {
            index_buffer_16 = Some(Buffer::new(vk::BufferUsageFlags::INDEX_BUFFER, max_index_count, require_submit));
            index_buffer_32 = None;
            index_type = vk::IndexType::UINT16;
        }

        Ok(Self {
            vertex_buffer: Buffer::<TVert>::new(vk::BufferUsageFlags::VERTEX_BUFFER, max_vertex_count, require_submit),
            index_buffer_16,
            index_buffer_32,
            index_type,
            require_submit
        })
    }

    pub fn new_and_create_from_obj_file<P: AsRef<Path>>(path: P, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pools_info: &CommandPoolsInfo) -> Result<Self> {
        let obj_file = File::open(path)?;
        let mut reader = BufReader::new(obj_file);

        let load_opts = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };
        let (models, _) = tobj::load_obj_buf(&mut reader, &load_opts, |_| Ok(Default::default()))?;

        //Merge vertices and indices for all meshes into a single mesh
        let mut vertices: Vec<TVert> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut vertex_indices = HashMap::<TVert, u32>::new();
        for model in &models {
            for mesh_index in &model.mesh.indices {
                let vertex: TVert;
                {
                    let mesh = &model.mesh;

                    let pos_offset = (mesh_index * 3) as usize;
                    let pos = glm::vec3(mesh.positions[pos_offset], mesh.positions[pos_offset + 2], mesh.positions[pos_offset + 1]); //Swap Y and Z - this engine uses Z as the up direction, but assets are created with Y as the up direction
                    // let pos = glm::vec3(mesh.positions[pos_offset + 1], mesh.positions[pos_offset + 0], mesh.positions[pos_offset + 2]);

                    let normal_offset = (mesh_index * 3) as usize;
                    let normal = if normal_offset + 3 > mesh.normals.len() {
                        None
                    } else {
                        Some(glm::vec3(mesh.normals[normal_offset], mesh.normals[normal_offset + 2], mesh.normals[normal_offset + 1])) //Swap Y and Z - this engine uses Z as the up direction, but assets are created with Y as the up direction
                        // Some(glm::vec3(mesh.normals[pos_offset + 1], mesh.normals[pos_offset + 0], mesh.normals[pos_offset + 2])
                    };

                    let color_offset = (mesh_index * 3) as usize;
                    let color = if color_offset + 3 > mesh.vertex_color.len() {
                        None
                    } else {
                        Some(glm::vec3(mesh.vertex_color[color_offset], mesh.vertex_color[color_offset + 1], mesh.vertex_color[color_offset + 2]))
                    };

                    let uv_offset = (mesh_index * 2) as usize;
                    let uv = if uv_offset + 2 > mesh.texcoords.len() {
                        None
                    } else {
                        Some(glm::vec2(mesh.texcoords[uv_offset], 1.0 - mesh.texcoords[uv_offset + 1]))
                    };

                    vertex = TVert::create_vertex_from_opts(pos, normal, color, uv);
                }

                if let Some(model_index) = vertex_indices.get(&vertex) {
                    indices.push(*model_index);
                } else {
                    let model_index = vertices.len() as u32;
                    vertex_indices.insert(vertex, model_index);

                    vertices.push(vertex);
                    indices.push(model_index);
                }
            }
        }

        let mut model = Self::new(vertices.len(), indices.len(), true)?;

        model.create(device, memory_properties)?;
        model.set_data(device, &vertices, &indices)?;
        model.submit(device, command_pools_info)?;

        Ok(model)
    }

    pub fn create(&mut self, device: &Device, memory: &vk::PhysicalDeviceMemoryProperties) -> Result<()> {
        if let Some(index_buffer) = self.index_buffer_16.as_mut() {
            index_buffer.create(device, memory)?;
        } else if let Some(index_buffer) = self.index_buffer_32.as_mut() {
            index_buffer.create(device, memory)?;
        } else {
            return Err(anyhow!("No index buffer to initialize... WTF?"));
        }

        self.vertex_buffer.create(device, memory)?;

        Ok(())
    }

    pub fn set_data(&mut self, device: &Device, vertex_data: &impl IntoBufferData<TVert>, index_data: &Vec<u32>) -> Result<()> {
        if let Some(index_buffer) = self.index_buffer_16.as_mut() {
            let index_data_16 = index_data.iter().map(|i| *i as u16).collect::<Vec<_>>();
            index_buffer.set_data(device, &index_data_16)?;
        } else if let Some(index_buffer) = self.index_buffer_32.as_mut() {
            index_buffer.set_data(device, index_data)?;
        } else {
            return Err(anyhow!("No index buffer to set the data for... WTF?"));
        }

        self.vertex_buffer.set_data(device, vertex_data)?;

        Ok(())
    }

    pub fn submit(&self, device: &Device, command_pools: &CommandPoolsInfo) -> Result<()> {
        if !self.require_submit {
            warn!("Model submitted that doesn't require data to be submitted.");
            return Ok(());
        }

        command_pools.submit_command_transient_sync(device, |command_buffer| {
            self.write_submit_to_command_buffer(device, command_buffer)
        })
    }

    pub fn write_submit_to_command_buffer(&self, device: &Device, command_buffer: &vk::CommandBuffer) -> Result<()> {
        if let Some(index_buffer) = self.index_buffer_16.as_ref() {
            index_buffer.write_submit_to_command_buffer(device, command_buffer)?;
        } else if let Some(index_buffer) = self.index_buffer_32.as_ref() {
            index_buffer.write_submit_to_command_buffer(device, command_buffer)?;
        } else {
            return Err(anyhow!("No index buffer to submit... WTF?"));
        }

        self.vertex_buffer.write_submit_to_command_buffer(device, command_buffer)?;

        Ok(())
    }

    pub fn write_render_to_command_buffer(&self, device: &Device, command_buffer: &vk::CommandBuffer, pipeline_layout: &vk::PipelineLayout, viewmodel: &glm::Mat4, normal_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        unsafe {
            let vertex_buffer = self.vertex_buffer.raw_buffer().ok_or_else(|| anyhow!("Could not unwrap vertex buffer. Has this model been initialized?"))?;
            device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer], &[0]);

            let used_element_count: u32;
            if let Some(index_buffer) = self.index_buffer_16.as_ref() {
                used_element_count = index_buffer.used_element_count() as u32;
                let raw_buffer = index_buffer.raw_buffer().ok_or_else(|| anyhow!("Could not unwrap index buffer. Has this model been initialized?"))?;
                device.cmd_bind_index_buffer(*command_buffer, raw_buffer, 0, self.index_type);
            } else if let Some(index_buffer) = self.index_buffer_32.as_ref() {
                used_element_count = index_buffer.used_element_count() as u32;
                let raw_buffer = index_buffer.raw_buffer().ok_or_else(|| anyhow!("Could not unwrap index buffer. Has this model been initialized?"))?;
                device.cmd_bind_index_buffer(*command_buffer, raw_buffer, 0, self.index_type);
            } else {
                return Err(anyhow!("No index buffer to unwrap for render... WTF?"));
            }

            let normal_viewmodel: glm::Mat4 = if let Some(nm_vm) = normal_viewmodel { *nm_vm } else { glm::transpose(&glm::inverse(viewmodel)) };

            let push_constants = PushConstants {
                viewmodel: *viewmodel,
                normal_viewmodel
            };
            let push_constants_bytes = push_constants.as_bytes();
            device.cmd_push_constants(*command_buffer, *pipeline_layout, vk::ShaderStageFlags::ALL_GRAPHICS, 0, push_constants_bytes);

            device.cmd_draw_indexed(*command_buffer, used_element_count, 1, 0, 0, 0);
        }

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device) {
        if let Some(index_buffer) = self.index_buffer_16.as_mut() {
            index_buffer.destroy(device);
        } else if let Some(index_buffer) = self.index_buffer_32.as_mut() {
            index_buffer.destroy(device);
        }

        self.vertex_buffer.destroy(device);
    }
}

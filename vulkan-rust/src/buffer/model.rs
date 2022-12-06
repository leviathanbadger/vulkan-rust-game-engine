use super::{Buffer, IntoBufferData, SingleModelRenderInfo, SingleFrameRenderInfo};

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
    bootstrap::{CommandPoolsInfo},
    shader_input::vertex_attribute_builder::{EmptyVertex}
};

pub trait CanBeVertexBufferType : Copy + Clone + Hash + PartialEq + Eq + ::std::fmt::Debug {
    fn create_vertex_from_opts(pos: glm::Vec3, normal: Option<glm::Vec3>, color: Option<glm::Vec3>, uv: Option<glm::Vec2>, face_normal: Option<glm::Vec3>) -> Self;
}
pub trait CanBeInstVertexBufferType : Copy + Clone + Hash + PartialEq + Eq + ::std::fmt::Debug {
}

impl CanBeInstVertexBufferType for EmptyVertex { }

#[derive(Debug, Copy, Clone)]
pub struct Model<TVert, TInstVert = EmptyVertex> where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
    vertex_buffer: Buffer<TVert>,
    inst_vertex_buffer: Option<Buffer<TInstVert>>,
    index_buffer_16: Option<Buffer<u16>>,
    index_buffer_32: Option<Buffer<u32>>,
    index_type: vk::IndexType,
    require_submit: bool
}

impl<TVert, TInstVert> Model<TVert, TInstVert> where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
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
            inst_vertex_buffer: None,
            index_buffer_16,
            index_buffer_32,
            index_type,
            require_submit
        })
    }
    pub fn new_instanced(max_vertex_count: usize, max_inst_vertex_count: usize, max_index_count: usize, require_submit: bool) -> Result<Self> {
        let mut model = Self::new(max_vertex_count, max_index_count, require_submit)?;

        model.inst_vertex_buffer = Some(Buffer::<TInstVert>::new(vk::BufferUsageFlags::VERTEX_BUFFER, max_inst_vertex_count, require_submit));

        Ok(model)
    }

    fn new_and_create_from_obj_file_impl<P: AsRef<Path>>(path: P, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pools_info: &CommandPoolsInfo, instances: Option<&impl IntoBufferData<TInstVert>>) -> Result<Self> {
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
            let mesh = &model.mesh;

            let mut next_face = 0usize;
            let mut face_normals = vec![];
            for _q in 0..(model.mesh.indices.len() / 3) {
                let end = next_face + 3;

                let pt_indices = &model.mesh.indices[next_face..end];
                let pt1_offset = (pt_indices[0] * 3) as usize;
                let pt1 = glm::vec3(mesh.positions[pt1_offset], mesh.positions[pt1_offset + 2], mesh.positions[pt1_offset + 1]);
                let pt2_offset = (pt_indices[1] * 3) as usize;
                let pt2 = glm::vec3(mesh.positions[pt2_offset], mesh.positions[pt2_offset + 2], mesh.positions[pt2_offset + 1]);
                let pt3_offset = (pt_indices[2] * 3) as usize;
                let pt3 = glm::vec3(mesh.positions[pt3_offset], mesh.positions[pt3_offset + 2], mesh.positions[pt3_offset + 1]);

                let face_normal = glm::cross(&glm::normalize(&(pt1 - pt2)), &glm::normalize(&(pt3 - pt2)));
                face_normals.push(Some(face_normal));

                next_face = end;
            }

            for (q, mesh_index) in model.mesh.indices.iter().enumerate() {
                let face_normal = *face_normals.get(q / 3).unwrap_or(&None);

                let vertex: TVert;
                {
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

                    vertex = TVert::create_vertex_from_opts(pos, normal, color, uv, face_normal);
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

        let mut model: Self;
        if let Some(insts) = instances {
            model = Self::new_instanced(vertices.len(), insts.element_count(), indices.len(), true)?;
        } else {
            model = Self::new(vertices.len(), indices.len(), true)?;
        }

        model.create(device, memory_properties)?;
        model.set_data(device, &vertices, &indices)?;

        if let Some(insts) = instances {
            model.set_inst_data(device, insts)?;
        }

        model.submit(device, command_pools_info)?;

        Ok(model)
    }
    pub fn new_and_create_from_obj_file<P: AsRef<Path>>(path: P, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pools_info: &CommandPoolsInfo) -> Result<Self> {
        Self::new_and_create_from_obj_file_impl(path, device, memory_properties, command_pools_info, None as Option<&Vec<TInstVert>>)
    }
    pub fn new_and_create_from_obj_file_instanced<P: AsRef<Path>>(path: P, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pools_info: &CommandPoolsInfo, inst_data: &impl IntoBufferData<TInstVert>) -> Result<Self> {
        Self::new_and_create_from_obj_file_impl(path, device, memory_properties, command_pools_info, Some(inst_data))
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

        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer.as_mut() {
            inst_vertex_buffer.create(device, memory)?;
        }

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
    pub fn set_inst_data(&mut self, device: &Device, inst_data: &impl IntoBufferData<TInstVert>) -> Result<()> {
        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer.as_mut() {
            inst_vertex_buffer.set_data(device, inst_data)?;

            Ok(())
        } else {
            Err(anyhow!("Can't set instance data. This Model has no instance vertex type!"))
        }
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

        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer {
            inst_vertex_buffer.write_submit_to_command_buffer(device, command_buffer)?;
        }

        Ok(())
    }

    pub fn create_frame_render_info(&self, frame_info: &mut SingleFrameRenderInfo, is_static: bool, is_opaque: bool, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        let raw_vertex_buffer = unsafe { self.vertex_buffer.raw_buffer() }.ok_or_else(|| anyhow!("Could not unwrap vertex buffer. Has this model been initialized?"))?;

        let raw_index_buffer: vk::Buffer;
        let used_element_count: u32;
        if let Some(index_buffer) = self.index_buffer_16.as_ref() {
            used_element_count = index_buffer.used_element_count() as u32;
            let raw_buffer = unsafe { index_buffer.raw_buffer() }.ok_or_else(|| anyhow!("Could not unwrap index buffer. Has this model been initialized?"))?;
            raw_index_buffer = raw_buffer;
        } else if let Some(index_buffer) = self.index_buffer_32.as_ref() {
            used_element_count = index_buffer.used_element_count() as u32;
            let raw_buffer = unsafe { index_buffer.raw_buffer() }.ok_or_else(|| anyhow!("Could not unwrap index buffer. Has this model been initialized?"))?;
            raw_index_buffer = raw_buffer;
        } else {
            return Err(anyhow!("No index buffer to unwrap for render... WTF?"));
        }

        let previous_viewmodel: glm::Mat4 = if let Some(prev_vm) = previous_viewmodel { *prev_vm } else { *viewmodel };

        let mut instance_count = 1;
        let mut raw_inst_vertex_buffer = None;
        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer {
            instance_count = inst_vertex_buffer.used_element_count() as u32;
            raw_inst_vertex_buffer = unsafe { Some(inst_vertex_buffer.raw_buffer().unwrap()) };
        }

        let model_render_info = SingleModelRenderInfo {
            is_static,
            is_opaque,
            viewmodel: *viewmodel,
            previous_viewmodel,

            vertex_buffer: raw_vertex_buffer,
            inst_vertex_buffer: raw_inst_vertex_buffer,
            index_buffer: Some(raw_index_buffer),
            index_type: self.index_type,

            element_count: used_element_count,

            instance_count,

            ..Default::default()
        };

        frame_info.models_to_render.push(model_render_info);

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device) {
        if let Some(index_buffer) = self.index_buffer_16.as_mut() {
            index_buffer.destroy(device);
        } else if let Some(index_buffer) = self.index_buffer_32.as_mut() {
            index_buffer.destroy(device);
        }

        self.vertex_buffer.destroy(device);

        if let Some(inst_vertex_buffer) = self.inst_vertex_buffer.as_mut() {
            inst_vertex_buffer.destroy(device);
        }
    }
}

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

use crate::shader_input::{
    empty_vertex::{EmptyVertex},
    vertex_attribute_builder::{HasVertexAttributeBindings}
};

pub trait CanBeVertexBufferType : HasVertexAttributeBindings + Copy + Clone + Default + Hash + PartialEq + Eq + ::std::fmt::Debug {
    fn create_vertex_from_opts(pos: glm::Vec3, normal: Option<glm::Vec3>, color: Option<glm::Vec3>, uv: Option<glm::Vec2>, face_normal: Option<glm::Vec3>, face_tangent: Option<glm::Vec3>) -> Self;
}
pub trait CanBeInstVertexBufferType : HasVertexAttributeBindings + Copy + Clone + Default + Hash + PartialEq + Eq + ::std::fmt::Debug {
}

#[doc(hidden)]
impl HasVertexAttributeBindings for u8 {
    fn binding_descriptions() -> &'static [vk::VertexInputBindingDescription] {
        panic!("Not actually supported.")
    }

    fn attribute_descriptions() -> &'static [vk::VertexInputAttributeDescription] {
        panic!("Not actually supported.")
    }
}
#[doc(hidden)]
impl CanBeVertexBufferType for u8 {
    fn create_vertex_from_opts(_pos: glm::Vec3, _normal: Option<glm::Vec3>, _color: Option<glm::Vec3>, _uv: Option<glm::Vec2>, _face_normal: Option<glm::Vec3>, _face_tangent: Option<glm::Vec3>) -> Self {
        panic!("Not actually supported.")
    }
}

impl CanBeInstVertexBufferType for EmptyVertex { }

#[derive(Debug, Copy, Clone)]
pub struct Model<TVert> where TVert : CanBeVertexBufferType {
    vertex_buffer: Buffer<TVert>,
    index_buffer_16: Option<Buffer<u16>>,
    index_buffer_32: Option<Buffer<u32>>,
    index_type: vk::IndexType,
    require_submit: bool,

    is_readonly: bool
}

pub type ReadonlyModel = Model<u8>;

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
            require_submit,

            is_readonly: false
        })
    }

    pub(super) fn new_and_create_from_obj_file<P: AsRef<Path>>(path: P, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties) -> Result<Self> {
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
            let mut face_tangents = vec![];
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

                let pt1_uv_offset = (pt_indices[0] * 2) as usize;
                let pt2_uv_offset = (pt_indices[1] * 2) as usize;
                let pt3_uv_offset = (pt_indices[2] * 2) as usize;
                if pt1_uv_offset + 2 > mesh.texcoords.len() || pt2_uv_offset + 2 > mesh.texcoords.len() || pt3_uv_offset + 2 > mesh.texcoords.len() {
                    face_tangents.push(None);
                } else {
                    let pt1_uv = glm::vec2(mesh.texcoords[pt1_uv_offset], 1.0 - mesh.texcoords[pt1_uv_offset + 1]);
                    let pt2_uv = glm::vec2(mesh.texcoords[pt2_uv_offset], 1.0 - mesh.texcoords[pt2_uv_offset + 1]);
                    let pt3_uv = glm::vec2(mesh.texcoords[pt3_uv_offset], 1.0 - mesh.texcoords[pt3_uv_offset + 1]);

                    let edge1 = pt2 - pt1;
                    let edge2 = pt3 - pt1;
                    let deltauv1 = pt2_uv - pt1_uv;
                    let deltauv2 = pt3_uv - pt1_uv;
                    let f = 1.0 / (deltauv1[0] * deltauv2[1] - deltauv2[0] * deltauv1[1]);

                    let mut tangent = f * glm::vec3(
                        deltauv2[1] * edge1[0] - deltauv1[1] * edge2[0],
                        deltauv2[1] * edge1[1] - deltauv1[1] * edge2[1],
                        deltauv2[1] * edge1[2] - deltauv1[1] * edge2[2]
                    );
                    tangent = tangent - (face_normal * glm::dot(&face_normal, &tangent));
                    tangent = glm::normalize(&tangent);
                    face_tangents.push(Some(tangent));
                }

                next_face = end;
            }

            for (q, mesh_index) in model.mesh.indices.iter().enumerate() {
                let face_normal = *face_normals.get(q / 3).unwrap_or(&None);
                let face_tangent = *face_tangents.get(q / 3).unwrap_or(&None);

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

                    vertex = TVert::create_vertex_from_opts(pos, normal, color, uv, face_normal, face_tangent);
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
        if self.is_readonly {
            return Err(anyhow!("Model is readonly. Can't call set_data!"));
        }

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

    pub fn write_submit_to_command_buffer(&self, device: &Device, command_buffer: &vk::CommandBuffer) -> Result<()> {
        if !self.require_submit {
            warn!("Model submitted that doesn't require data to be submitted.");
            return Ok(());
        }

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

    pub fn destroy(&mut self, device: &Device) {
        if let Some(index_buffer) = self.index_buffer_16.as_mut() {
            index_buffer.destroy(device);
        } else if let Some(index_buffer) = self.index_buffer_32.as_mut() {
            index_buffer.destroy(device);
        }

        self.vertex_buffer.destroy(device);
    }

    pub fn reinterpret_readonly(self) -> ReadonlyModel {
        ReadonlyModel {
            vertex_buffer: self.vertex_buffer.reinterpret_readonly(),
            index_buffer_16: self.index_buffer_16,
            index_buffer_32: self.index_buffer_32,
            index_type: self.index_type,
            require_submit: self.require_submit,

            is_readonly: true
        }
    }
}

pub trait HasModelDetails {
    fn get_model_details(&self) -> Result<(vk::Buffer, vk::DeviceSize, Option<vk::Buffer>, vk::DeviceSize, vk::IndexType, u32)>;
}

impl<TVert> HasModelDetails for Model<TVert> where TVert : CanBeVertexBufferType {
    fn get_model_details(&self) -> Result<(vk::Buffer, vk::DeviceSize, Option<vk::Buffer>, vk::DeviceSize, vk::IndexType, u32)> {
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

        Ok((raw_vertex_buffer, 0, Some(raw_index_buffer), 0, self.index_type, used_element_count))
    }
}

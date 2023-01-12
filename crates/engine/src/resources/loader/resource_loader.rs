use super::{
    resource_load_job::{ResourceLoadJob},
    load_model_job::{LoadModelJob}
};

use std::{
    marker::{PhantomData},
    collections::{
        VecDeque,
        HashMap,
        hash_map::{DefaultHasher}
    },
    hash::{Hash, Hasher}
};
use anyhow::{Result, Ok};
use nalgebra_glm as glm;
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    shader_input::empty_vertex::{EmptyVertex},
    resources::{
        CanBeVertexBufferType,
        CanBeInstVertexBufferType,
        Buffer,
        IntoBufferData,
        SingleFrameRenderInfo,
        SingleModelRenderInfo,
        loader::{
            buffer_submit_job::{BufferSubmitJob},
            load_material_job::{LoadMaterialJob}
        },
        material::{Material},
        model::{ReadonlyModel},
        buffer::{ReadonlyBuffer}
    },
    app_data::{AppData}
};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct MaterialRef {
    id: u32
}

impl MaterialRef {
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct ModelRef {
    id: u32
}

impl ModelRef {
    pub fn create_frame_render_info_impl<TInstVert>(&self, frame_info: &mut SingleFrameRenderInfo, material: MaterialRef, is_static: bool, is_opaque: bool, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>, inst_vertex_buffer: Option<Buffer<TInstVert>>) -> Result<()> where TInstVert : CanBeInstVertexBufferType {
        let previous_viewmodel: glm::Mat4 = if let Some(prev_vm) = previous_viewmodel { *prev_vm } else { *viewmodel };

        let mut instance_count = 1;
        let mut raw_inst_vertex_buffer = None;
        if let Some(inst_vertex_buffer) = inst_vertex_buffer {
            instance_count = inst_vertex_buffer.used_element_count() as u32;
            raw_inst_vertex_buffer = unsafe { Some(inst_vertex_buffer.raw_buffer().unwrap()) };
        }

        let model_render_info = SingleModelRenderInfo {
            is_static,
            is_opaque,
            viewmodel: *viewmodel,
            previous_viewmodel,

            model: *self,
            material,

            inst_vertex_buffer: raw_inst_vertex_buffer,
            instance_count,

            ..Default::default()
        };

        frame_info.models_to_render.push(model_render_info);

        Ok(())
    }
    pub fn create_frame_render_info(&self, frame_info: &mut SingleFrameRenderInfo, material: MaterialRef, is_static: bool, is_opaque: bool, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>) -> Result<()> {
        self.create_frame_render_info_impl(frame_info, material, is_static, is_opaque, viewmodel, previous_viewmodel, None as Option<Buffer<EmptyVertex>>)
    }
    pub fn create_frame_render_info_instanced<TInstVert>(&self, frame_info: &mut SingleFrameRenderInfo, material: MaterialRef, is_static: bool, is_opaque: bool, viewmodel: &glm::Mat4, previous_viewmodel: Option<&glm::Mat4>, inst_vertex_buffer: Buffer<TInstVert>) -> Result<()> where TInstVert : CanBeInstVertexBufferType {
        self.create_frame_render_info_impl(frame_info, material, is_static, is_opaque, viewmodel, previous_viewmodel, Some(inst_vertex_buffer))
    }

    #[allow(unused)]
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub struct MaterialProperties<TVert, TInstVert = EmptyVertex> where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
    pub is_opaque: bool,
    pub shader_name: &'static str,
    pub shader_entry: &'static str,

    #[doc(hidden)]
    pub phantom_vert: PhantomData<TVert>,
    #[doc(hidden)]
    pub phantom_inst_vert: PhantomData<TInstVert>
}

impl<TVert, TInstVert> Default for MaterialProperties<TVert, TInstVert> where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
    fn default() -> Self {
        Self {
            is_opaque: true,
            shader_name: "standard",
            shader_entry: "main",

            phantom_vert: Default::default(),
            phantom_inst_vert: Default::default()
        }
    }
}

impl<TVert, TInstVert> ::core::hash::Hash for MaterialProperties<TVert, TInstVert> where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.is_opaque.hash(state);
        self.shader_name.hash(state);
        self.shader_entry.hash(state);

        self.phantom_vert.hash(state);
        self.phantom_inst_vert.hash(state);

        let vert = TVert::default();
        vert.hash(state);
        let inst_vert = TInstVert::default();
        inst_vert.hash(state);
    }
}

#[derive(Debug, Default)]
pub struct ModelProperties<TVert> where TVert : CanBeVertexBufferType {
    pub obj_path: Option<String>,

    #[doc(hidden)]
    pub phantom_vert: PhantomData<TVert>
}

impl<TVert> ::core::hash::Hash for ModelProperties<TVert> where TVert : CanBeVertexBufferType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj_path.hash(state);

        self.phantom_vert.hash(state);

        let vert = TVert::default();
        vert.hash(state);
    }
}

#[derive(Debug)]
pub struct ResourceLoader {
    device: Device,
    memory: vk::PhysicalDeviceMemoryProperties,
    job_queue: VecDeque<Box<dyn ResourceLoadJob>>,

    buffers: Vec<ReadonlyBuffer>,

    hashed_material_props: HashMap<u64, MaterialRef>,
    materials: HashMap<MaterialRef, Material>,
    next_mat_ref_id: u32,

    hashed_model_props: HashMap<u64, ModelRef>,
    models: HashMap<ModelRef, ReadonlyModel>,
    next_model_ref_id: u32
}

impl ResourceLoader {
    pub fn new(device: Device, memory: vk::PhysicalDeviceMemoryProperties) -> Self {
        ResourceLoader {
            device,
            memory,
            job_queue: VecDeque::new(),

            buffers: Vec::new(),

            hashed_material_props: HashMap::new(),
            materials: HashMap::new(),
            next_mat_ref_id: 1,

            hashed_model_props: HashMap::new(),
            models: HashMap::new(),
            next_model_ref_id: 1
        }
    }

    fn add_job(&mut self, job: impl ResourceLoadJob + 'static) -> Result<()> {
        self.job_queue.push_back(Box::new(job));

        Ok(())
    }

    pub fn get_or_load_material<TVert, TInstVert>(&mut self, props: &MaterialProperties<TVert, TInstVert>) -> Result<MaterialRef> where TVert : CanBeVertexBufferType, TInstVert : CanBeInstVertexBufferType {
        let hashed_props = {
            let mut hasher = DefaultHasher::new();
            props.hash(&mut hasher);
            hasher.finish()
        };
        if let Some(mat_ref) = self.hashed_material_props.get(&hashed_props) {
            //TODO: reference count
            return Ok(*mat_ref);
        }

        let mat_ref = MaterialRef { id: self.next_mat_ref_id };
        self.next_mat_ref_id += 1;
        self.hashed_material_props.insert(hashed_props, mat_ref);

        let job = LoadMaterialJob::create_for(mat_ref, props);
        self.add_job(job)?;

        Ok(mat_ref)
    }
    pub(super) fn finish_loading_material(&mut self, mat_ref: MaterialRef, mat: Material) -> Result<()> {
        self.materials.insert(mat_ref, mat);

        Ok(())
    }
    pub fn get_render_material(&self, mat_ref: MaterialRef) -> Option<Material> {
        self.materials.get(&mat_ref)
            .filter(|m| m.is_loaded)
            .map(|m| *m)
    }

    pub fn get_or_load_model<TVert>(&mut self, props: &ModelProperties<TVert>) -> Result<ModelRef> where TVert : CanBeVertexBufferType + 'static {
        let hashed_props = {
            let mut hasher = DefaultHasher::new();
            props.hash(&mut hasher);
            hasher.finish()
        };
        if let Some(model_ref) = self.hashed_model_props.get(&hashed_props) {
            //TODO: reference count
            return Ok(*model_ref);
        }

        let model_ref = ModelRef { id: self.next_model_ref_id };
        self.next_model_ref_id += 1;
        self.hashed_model_props.insert(hashed_props, model_ref);

        let job = LoadModelJob::create_for(model_ref, props);
        self.add_job(job)?;

        Ok(model_ref)
    }
    pub(super) fn finish_loading_model(&mut self, model_ref: ModelRef, model: ReadonlyModel) -> Result<()> {
        self.models.insert(model_ref, model);

        Ok(())
    }
    pub fn get_render_model(&self, model_ref: ModelRef) -> Option<ReadonlyModel> {
        self.models.get(&model_ref).map(|m| *m)
    }

    fn create_buffer<T>(&mut self, data: &impl IntoBufferData<T>, usage: vk::BufferUsageFlags) -> Result<Buffer<T>> where T : Copy + Clone + std::fmt::Debug {
        let mut buffer = Buffer::<T>::new(usage, data.element_count(), true);
        buffer.create(&self.device, &self.memory)?;
        buffer.set_data(&self.device, data)?;

        let job = BufferSubmitJob::create_for(buffer);
        self.add_job(job)?;

        self.buffers.push(buffer.clone().reinterpret_readonly());

        Ok(buffer)
    }
    pub fn create_inst_buffer<T>(&mut self, data: &impl IntoBufferData<T>) -> Result<Buffer<T>> where T : Copy + Clone + std::fmt::Debug {
        self.create_buffer(data, vk::BufferUsageFlags::VERTEX_BUFFER)
    }

    pub fn unload_material(&mut self, _material: MaterialRef) -> () {
        //TODO: reference count
    }

    pub fn unload_model(&mut self, _model: ModelRef) -> () {
        //TODO: reference count
    }

    pub fn force_unload_all(&mut self) -> () {
        self.hashed_model_props.clear();
        for model in self.models.values_mut() {
            model.destroy(&self.device);
        }
        self.models.clear();

        self.hashed_material_props.clear();
        for mat in self.materials.values_mut() {
            mat.destroy(&self.device);
        }
        self.materials.clear();

        for buffer in self.buffers.iter_mut() {
            buffer.destroy(&self.device);
        }
        self.buffers.clear();
    }

    pub fn tick(&mut self, app_data: &AppData) -> Result<()> {
        if self.job_queue.len() == 0 {
            return Ok(());
        }

        let device = self.device.clone();

        let mut jobs_to_write_to_command_buffer = vec![];

        while let Some(mut job) = self.job_queue.pop_front() {
            job.load(self, &device, app_data)?;
            if job.needs_transient_command() {
                jobs_to_write_to_command_buffer.push(job);
            }
        }

        let command_pools = app_data.command_pools.as_ref().unwrap();
        let jobs_to_write_to_command_buffer2 = &jobs_to_write_to_command_buffer[..];
        command_pools.submit_command_transient_sync(&device, |command_buffer| {
            for job in jobs_to_write_to_command_buffer2 {
                job.write_to_command_buffer(self, &device, app_data, command_buffer)?;
            }

            Ok(())
        })?;

        for job in jobs_to_write_to_command_buffer.iter_mut() {
            job.after_command(self, &device, app_data)?;
        }

        Ok(())
    }
}

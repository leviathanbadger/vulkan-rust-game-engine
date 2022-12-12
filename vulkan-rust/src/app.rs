use std::{
    collections::{HashSet},
    ffi::{CStr},
    time::{Instant, Duration},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering}
    },
    thread
};
use anyhow::{anyhow, Result};
use thiserror::Error;
use winit::{
    dpi::{LogicalSize},
    window::{Window, WindowBuilder},
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent, VirtualKeyCode}
};
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    window as vk_window,
    prelude::v1_0::*,
    vk::{KhrSurfaceExtension, StringArray, KhrSwapchainExtension}
};

use crate::{
    app_data::{AppData, VulkanQueueInfo},
    bootstrap::{BootstrapLoader, QueueFamilyIndices},
    shader_input::uniform_buffer_object::{UniformBufferObject},
    game::scene::{Scene},
    frame_info::{FrameInfo},
    resources::{SingleFrameRenderInfo, ResourceLoader, SingleModelRenderInfo, Material}
};

#[derive(Debug, Error)]
#[error("Missing {0}")]
pub struct GraphicsCardSuitabilityError(pub &'static str);

#[derive(Debug)]
pub struct App {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Window,
    pub app_data: AppData,
    //entry can't be disposed of before inst or it causes a segmentation fault when you attempt to use inst
    #[allow(unused)]
    entry: Entry,
    pub inst: Instance,
    pub device: Device,
    pub bootstrap_loaders: Vec<Box<dyn BootstrapLoader>>,

    pub scene: Box<Scene>,
    pub resource_loader: ResourceLoader,
    pub frame_info: FrameInfo,

    destroying: bool,
    needs_new_swapchain: bool,
    shutdown_requested: Arc<AtomicBool>
}

impl App {
    pub fn create(initial_title: &'static str, default_size: LogicalSize<i32>, bootstrap_loaders: Vec<Box<dyn BootstrapLoader>>) -> Result<Self> {
        debug!("Creating window and window event loop...");
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(initial_title)
            .with_inner_size(default_size)
            .build(&event_loop)?;

        let mut app_data = AppData::default();

        let inst: Instance;
        let entry: Entry;
        unsafe {
            let loader = LibloadingLoader::new(LIBRARY)?;
            entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
            inst = Self::create_instance(initial_title, &bootstrap_loaders, &window, &mut app_data, &entry)?;
        }

        unsafe {
            debug!("Creating Vulkan surface KHR...");
            app_data.surface = Some(vk_window::create_surface(&inst, &window)?);
        }

        let device: Device;
        unsafe {
            debug!("Selecting graphics card (physical device) and creating logical device...");

            let mut request_layers_ptrs = vec![];
            let mut request_extensions_ptrs = vec![];
            for loader in bootstrap_loaders.iter() {
                loader.add_required_device_layers(&mut request_layers_ptrs)?;
                loader.add_required_device_extensions(&mut request_extensions_ptrs)?;
            }

            Self::select_graphics_card(&inst, &bootstrap_loaders, &mut app_data, &request_layers_ptrs, &request_extensions_ptrs)?;
            device = Self::create_logical_device(&inst, &bootstrap_loaders, &mut app_data, &request_layers_ptrs, &request_extensions_ptrs)?;
        }

        for loader in bootstrap_loaders.iter() {
            loader.after_create_logical_device(&inst, &device, &window, &mut app_data)?;
        }

        let scene = Scene::new();
        let resource_loader = ResourceLoader::new(device.clone(), app_data.memory_properties);

        let app = Self {
            event_loop: Some(event_loop),
            window,
            app_data,
            entry,
            inst,
            device,
            bootstrap_loaders: bootstrap_loaders,

            scene: Box::new(scene),
            resource_loader,
            frame_info: FrameInfo::default(),

            destroying: false,
            needs_new_swapchain: false,
            shutdown_requested: Arc::new(AtomicBool::new(false))
        };

        Ok(app)
    }

    unsafe fn create_instance(initial_title: &str, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, window: &Window, app_data: &mut AppData, entry: &Entry) -> Result<Instance> {
        debug!("Selecting instance extensions and layers, and creating instance...");
        let mut zero_terminated: String = "".to_owned();
        zero_terminated.push_str(initial_title);
        zero_terminated.push_str("\0");

        let app_info = vk::ApplicationInfo::builder()
            .application_name(zero_terminated.as_bytes())
            .application_version(vk::make_version(0, 0, 1))
            .engine_name(b"No engine\0")
            .engine_version(vk::make_version(0, 0, 1))
            .api_version(vk::make_version(0, 0, 1));
        trace!("Vulkan app_info created to present to create_instance. App Name: {:?}; Engine Name: {:?}", CStr::from_ptr(app_info.application_name), CStr::from_ptr(app_info.engine_name));

        let mut request_layers_ptrs = vec![];

        let mut request_extensions_ptrs = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|n| n.as_ptr())
            .collect::<Vec<_>>();

        for loader in bootstrap_loaders.iter() {
            loader.add_required_instance_layers(&mut request_layers_ptrs)?;
            loader.add_required_instance_extensions(&mut request_extensions_ptrs)?;
        }

        Self::check_instance(entry, &request_layers_ptrs, &request_extensions_ptrs)?;

        debug!("Creating Vulkan instance with requested layers and extensions...");
        let inst_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&request_layers_ptrs)
            .enabled_extension_names(&request_extensions_ptrs);

        let last_callback = move |inst_info: vk::InstanceCreateInfoBuilder| -> Result<Instance> {
            trace!("Final callback. Creating Vulkan instance...");
            let inst = entry.create_instance(&inst_info, None)?;
            debug!("Vulkan instance created: {:?}", inst);
            Ok(inst)
        };

        fn create_and_invoke_callback(index: usize, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &mut AppData, last_callback: &dyn Fn(vk::InstanceCreateInfoBuilder) -> Result<Instance>, inst_info: vk::InstanceCreateInfoBuilder) -> Result<Instance> {
            trace!("Invoking callback for index {} to create Vulkan instance...", index);
            let loader_res = bootstrap_loaders.get(index);
            match loader_res {
                Some(loader) => {
                    let next_callback = |inst_info: vk::InstanceCreateInfoBuilder, app_data: &mut AppData| create_and_invoke_callback(index + 1, bootstrap_loaders, app_data, last_callback, inst_info);
                    loader.instance_create(inst_info, app_data, &next_callback)
                },
                None => {
                    last_callback(inst_info)
                }
            }
        }

        let inst = create_and_invoke_callback(0, bootstrap_loaders, app_data, &last_callback, inst_info)?;

        Ok(inst)
    }

    unsafe fn check_instance(entry: &Entry, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<()> {
        let available_layers = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();
        let request_layers = request_layers_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        debug!("Available Vulkan layers: {:?}", available_layers);
        info!("Requesting Vulkan layers: {:?}", request_layers);

        for layer in request_layers {
            if !available_layers.contains(&StringArray::from_cstr(layer)) {
                return Err(anyhow!("Vulkan layer ({:?}) requested but not supported.", layer));
            }
        }

        let available_extensions = entry
            .enumerate_instance_extension_properties(None)?
            .iter()
            .map(|e| e.extension_name)
            .collect::<HashSet<_>>();
        let request_extensions = request_extensions_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        debug!("Available Vulkan extensions: {:?}", available_extensions);
        info!("Requesting Vulkan extensions: {:?}", request_extensions);

        for ext in request_extensions {
            if !available_extensions.contains(&StringArray::from_cstr(ext)) {
                return Err(anyhow!("Vulkan extension ({:?}) requested but not supported.", ext));
            }
        }

        Ok(())
    }

    unsafe fn select_graphics_card(inst: &Instance, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &mut AppData, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<()> {
        let physical_devices = inst.enumerate_physical_devices()?;

        for physical_device in physical_devices {
            let properties = inst.get_physical_device_properties(physical_device);

            if let Err(error) = Self::check_graphics_card(inst, bootstrap_loaders, app_data, physical_device, request_layers_ptrs, request_extensions_ptrs) {
                warn!("Skipping graphics card ({} - {}): {}", physical_device.as_raw(), properties.device_name, error);
            } else {
                //TODO: select _best_ graphics card, not just the first one in the list
                info!("Using graphics card ({} - {}).", physical_device.as_raw(), properties.device_name);
                app_data.physical_device = Some(physical_device);
                app_data.memory_properties = inst.get_physical_device_memory_properties(physical_device);
                return Ok(());
            }
        }

        Err(anyhow!(GraphicsCardSuitabilityError("No suitable graphics card was found")))
    }

    unsafe fn check_graphics_card(inst: &Instance, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &AppData, physical_device: vk::PhysicalDevice, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<()> {
        //Check for layers and extensions before calling check_physical_device_compatibility. Some bootstrap loaders assume their requested extensions are already confirmed to be present
        Self::check_physical_device(inst, physical_device, request_layers_ptrs, request_extensions_ptrs, false)?;

        let properties = inst.get_physical_device_properties(physical_device);
        let features = inst.get_physical_device_features(physical_device);

        for loader in bootstrap_loaders.iter() {
            loader.check_physical_device_compatibility(inst, app_data, physical_device, properties, features)?;
        }

        let queue_family_indices = QueueFamilyIndices::get(inst, app_data, physical_device)?;
        if let None = queue_family_indices.graphics {
            return Err(anyhow!(GraphicsCardSuitabilityError("No queue family on this physical device supports graphics operations.")));
        }
        if let None = queue_family_indices.present {
            return Err(anyhow!(GraphicsCardSuitabilityError("No queue family on this physical device supports KHR present operations.")));
        }

        Ok(())
    }

    unsafe fn create_logical_device(inst: &Instance, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, app_data: &mut AppData, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>) -> Result<Device> {
        let physical_device = app_data.physical_device.unwrap();
        let indices = QueueFamilyIndices::get(&inst, app_data, physical_device)?;

        let mut unique_queue_families = HashSet::new();
        let graphics_queue_family = indices.graphics.unwrap();
        let present_queue_family = indices.present.unwrap();
        unique_queue_families.insert(graphics_queue_family);
        unique_queue_families.insert(present_queue_family);

        let queue_priorities = &[1.0];
        let queue_infos = unique_queue_families
            .iter()
            .map(|i| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*i)
                    .queue_priorities(queue_priorities)
            })
            .collect::<Vec<_>>();

        let mut features = vk::PhysicalDeviceFeatures::builder();

        for loader in bootstrap_loaders.iter() {
            loader.add_required_device_features(&mut features)?;
        }

        // Sanity check. At this point the physical device should have been selected based (in part) on having the requested layers and extensions
        Self::check_physical_device(inst, physical_device, request_layers_ptrs, request_extensions_ptrs, true)?;

        debug!("Creating Vulkan logical device with requested layers and features.");
        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_layer_names(&request_layers_ptrs)
            .enabled_extension_names(&request_extensions_ptrs)
            .enabled_features(&features);

        let device = inst.create_device(physical_device, &device_info, None)?;

        let graphics_queue = device.get_device_queue(graphics_queue_family, 0);
        debug!("Vulkan graphics queue handle: {}", graphics_queue.as_raw());

        let present_queue = device.get_device_queue(present_queue_family, 0);
        debug!("Vulkan KHR present queue handle: {}", present_queue.as_raw());

        app_data.queue_info = Some(Arc::new(VulkanQueueInfo {
            graphics_queue,
            graphics_queue_family,
            present_queue,
            present_queue_family
        }));

        Ok(device)
    }

    unsafe fn check_physical_device(inst: &Instance, physical_device: vk::PhysicalDevice, request_layers_ptrs: &Vec<*const i8>, request_extensions_ptrs: &Vec<*const i8>, log_check: bool) -> Result<()> {
        let available_layers = inst
            .enumerate_device_layer_properties(physical_device)?
            .iter()
            .map(|l| l.layer_name)
            .collect::<HashSet<_>>();
        let request_layers = request_layers_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        if log_check {
            debug!("Available Vulkan device layers: {:?}", available_layers);
            info!("Requesting Vulkan device layers: {:?}", request_layers);
        }

        for layer in request_layers {
            if !available_layers.contains(&StringArray::from_cstr(layer)) {
                return Err(anyhow!("Vulkan device layer ({:?}) requested but not supported.", layer));
            }
        }

        let available_extensions = inst
            .enumerate_device_extension_properties(physical_device, None)?
            .iter()
            .map(|l| l.extension_name)
            .collect::<HashSet<_>>();
        let request_extensions = request_extensions_ptrs
            .iter()
            .map(|name| CStr::from_ptr(*name))
            .collect::<Vec<_>>();
        if log_check {
            debug!("Available Vulkan device extensions: {:?}", available_extensions);
            info!("Requesting Vulkan device extensions: {:?}", request_extensions);
        }

        for ext in request_extensions {
            if !available_extensions.contains(&StringArray::from_cstr(ext)) {
                return Err(anyhow!("Vulkan device extension ({:?}) requested but not supported.", ext));
            }
        }

        Ok(())
    }

    fn recreate_swapchain(&mut self) -> Result<()> {
        unsafe {
            self.device.device_wait_idle()?;
        }

        debug!("Recreating swapchain and related resources (possibly due to window resize)...");

        let last_callback = move |_inst: &Instance, _device: &Device, _window: &Window, _app_data: &mut AppData| -> Result<()> {
            Ok(())
        };

        fn create_and_invoke_callback(index: usize, bootstrap_loaders: &Vec<Box<dyn BootstrapLoader>>, last_callback: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData) -> Result<()> {
            trace!("Invoking callback for index {} to recreate swapchain and related resources...", index);
            let loader_count = bootstrap_loaders.len();
            let loader_res = if index == loader_count { None } else { bootstrap_loaders.get(loader_count - index - 1) };
            match loader_res {
                Some(loader) => {
                    let next_callback = |inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData| create_and_invoke_callback(index + 1, bootstrap_loaders, last_callback, inst, device, window, app_data);
                    loader.recreate_swapchain(inst, device, window, app_data, &next_callback)
                },
                None => {
                    last_callback(inst, device, window, app_data)
                }
            }
        }

        create_and_invoke_callback(0, &self.bootstrap_loaders, &last_callback, &self.inst, &self.device, &self.window, &mut self.app_data)?;

        self.needs_new_swapchain = false;

        Ok(())
    }

    //Deliberately not a ref, because the run method needs to own "self"
    pub fn run(mut self) -> ! {
        let mut minimized = false;
        info!("Starting window event loop.");

        #[allow(unused_must_use)] {
            self.set_ctrlc_handler(self.shutdown_requested.clone());
        }

        //TODO: Don't abuse Option<> in the struct in order to call run on the event loop without causing an ownership error
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, _, control_flow| {
            if self.destroying {
                *control_flow = ControlFlow::Exit;
                debug!("Window message received after shutdown began: {:?}", event);
                return ();
            }

            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared => {
                    if self.shutdown_requested.load(Ordering::Relaxed) {
                        info!("Shutdown requested via Ctrl+C or other asynchronous method. Shutting down application...");
                        self.shutdown();
                    }

                    if !self.destroying && !minimized {
                        self.game_loop().unwrap();
                    }
                }
                Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                    self.needs_new_swapchain = true;
                    minimized = size.width == 0 || size.height == 0;
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    info!("Window close requested. Shutting down application...");
                    self.shutdown();
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    //TODO: pass keyboard input to game state

                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Escape {
                            info!("Escape key pressed. Shutting down application...");
                            self.shutdown();
                        }
                    }
                }
                _ => { }
            }

            if self.destroying {
                *control_flow = ControlFlow::Exit;
            }
        });
    }

    fn set_ctrlc_handler(&self, shutdown_requested: Arc<AtomicBool>) -> Result<()> {
        ctrlc::set_handler(move || {
            warn!("Ctrl+C handled. Application will shut down asynchronously before rendering the next frame.");
            shutdown_requested.store(true, Ordering::SeqCst);
        })?;

        Ok(())
    }

    fn game_loop(&mut self) -> Result<()> {
        if !self.needs_new_swapchain {
            self.frame_info.current_frame_time = Instant::now();
            self.frame_info.current_frame_delta_time = self.frame_info.current_frame_time - self.frame_info.last_frame_start_time;
            self.scene.tick(&self.frame_info)?;
            self.frame_info.last_frame_start_time = self.frame_info.current_frame_time;

            {
                self.scene.load_and_unload(&mut self.resource_loader)?;
                self.resource_loader.tick(&self.app_data)?;
            }

            let bounds = self.app_data.render_images.as_ref().unwrap().base_render_extent;
            let frame_render_info = self.create_frame_render_info(bounds)?;
            let frame_render_info = frame_render_info.as_ref();

            self.scene.end_frame(bounds)?;
            self.frame_info.current_frame_index += 1;

            self.render(frame_render_info)?;
        }

        if self.needs_new_swapchain {
            self.recreate_swapchain()?;
        }

        //TODO: sleep until next frame, not just some arbitrary amount
        thread::sleep(Duration::from_millis(10));

        Ok(())
    }

    #[allow(unused)]
    pub fn create_request_shutdown(&self) -> Arc<AtomicBool> {
        self.shutdown_requested.clone()
    }

    fn create_frame_render_info(&self, bounds: vk::Extent2D) -> Result<Box<SingleFrameRenderInfo>> {
        let mut frame_info = SingleFrameRenderInfo {
            frame_index: self.frame_info.current_frame_index,
            time_in_seconds: self.frame_info.current_frame_delta_time.as_secs_f32(),

            ..Default::default()
        };

        self.scene.create_frame_render_info(&mut frame_info, bounds)?;

        Ok(Box::new(frame_info))
    }

    fn render(&mut self, frame_info: &SingleFrameRenderInfo) -> Result<()> {
        let swapchain = self.app_data.swapchain.as_ref().unwrap().swapchain;

        let sync_frame = (self.frame_info.current_frame_index % self.app_data.max_frames_in_flight()) as usize;
        let sync_objects_info = self.app_data.sync_objects.as_mut().unwrap();
        let frame_sync = sync_objects_info.get_sync_objects(sync_frame)?;

        let image_index: usize;
        unsafe {
            self.device.wait_for_fences(&[frame_sync.in_flight_fence], true, u64::MAX)?;

            let result = self.device.acquire_next_image_khr(swapchain, u64::MAX, frame_sync.image_available, vk::Fence::null());
            match result {
                Ok((idx, _)) => image_index = idx as usize,
                Err(vk::ErrorCode::OUT_OF_DATE_KHR) => {
                    warn!("Suboptimal or out-of-date swapchain detected before frame render");
                    self.needs_new_swapchain = true;
                    return Ok(());
                },
                Err(e) => return Err(anyhow!(e))
            }

            let image_in_flight = sync_objects_info.images_in_flight[image_index];
            if !image_in_flight.is_null() {
                self.device.wait_for_fences(&[image_in_flight], true, u64::MAX)?;
            }
        }

        sync_objects_info.images_in_flight[image_index] = frame_sync.in_flight_fence;

        self.update_uniform_buffer(image_index, frame_info)?;

        let command_pools_info = self.app_data.command_pools.as_ref();
        let command_buffer = command_pools_info.unwrap().command_buffers[image_index];
        let wait_semaphores = &[frame_sync.image_available];
        let signal_semaphores = &[frame_sync.render_finished];

        command_pools_info.unwrap().submit_command_async(
            &self.device,
            &command_buffer,
            wait_semaphores,
            &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
            signal_semaphores,
            &frame_sync.in_flight_fence,
            |cb| {
                self.update_command_buffer(image_index, cb, frame_info)
            })?;

        let swapchains = &[swapchain];
        let image_indices = &[image_index as u32];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        let present_queue = self.app_data.queue_info.as_ref().unwrap().present_queue;
        unsafe {
            let result = self.device.queue_present_khr(present_queue, &present_info);

            if result == Ok(vk::SuccessCode::SUBOPTIMAL_KHR) || result == Err(vk::ErrorCode::OUT_OF_DATE_KHR) {
                warn!("Suboptimal or out-of-date swapchain detected during frame render");
                self.needs_new_swapchain = true;
                return Ok(());
            }

            if let Err(e) = result {
                return Err(anyhow!(e))
            }
        }

        Ok(())
    }

    fn update_uniform_buffer(&mut self, image_index: usize, frame_info: &SingleFrameRenderInfo) -> Result<()> {
        let buffer = &mut self.app_data.uniforms.as_mut().unwrap().uniform_buffers[image_index];

        let ubo = UniformBufferObject {
            proj: frame_info.proj,
            previous_proj: frame_info.previous_proj,
            ambient_light: frame_info.ambient_light,
            directional_light_color: frame_info.directional_light_color,
            directional_light_direction: frame_info.directional_light_direction,
            frame_index: frame_info.frame_index,
            time_in_seconds: frame_info.time_in_seconds,

            ..Default::default() //Necessary for the manual padding
        };
        buffer.set_data(&self.device, &ubo)?;

        Ok(())
    }

    fn update_command_buffer(&self, image_index: usize, command_buffer: &vk::CommandBuffer, frame_info: &SingleFrameRenderInfo) -> Result<()> {
        let descriptor_set_info = &self.app_data.descriptor_sets.as_ref().unwrap();

        let framebuffer_info = &self.app_data.framebuffer.as_ref().unwrap();

        self.render_base(command_buffer, &framebuffer_info.base_render_framebuffers[image_index], &[descriptor_set_info.base_descriptor_sets[image_index]], frame_info)?;

        self.render_postprocessing(command_buffer, &framebuffer_info.postprocessing_framebuffers[image_index], &[descriptor_set_info.postprocessing_descriptor_sets[image_index]])?;

        Ok(())
    }
    fn render_base(&self, command_buffer: &vk::CommandBuffer, framebuffer: &vk::Framebuffer, descriptor_sets: &[vk::DescriptorSet], frame_info: &SingleFrameRenderInfo) -> Result<()> {
        let pipeline_info = &self.app_data.pipeline.as_ref().unwrap();
        let render_extent = self.app_data.render_images.as_ref().unwrap().base_render_extent;

        let base_render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(render_extent);

        let clear_color = frame_info.clear_color;
        let color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [clear_color[0], clear_color[1], clear_color[2], 1.0]
            }
        };
        let motion_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0]
            }
        };
        let depth_clear_value = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0
            }
        };

        let base_render_clear_values = &[color_clear_value, motion_clear_value, depth_clear_value];
        let base_render_pass_info = vk::RenderPassBeginInfo::builder()
            .render_pass(pipeline_info.base_render_pass)
            .framebuffer(*framebuffer)
            .render_area(base_render_area)
            .clear_values(base_render_clear_values);

        //TODO: sort models to render to ensure the least fragment redraws
        //TODO: filter models that are obviously outside of the view frustum
        //TODO: filter models that are occluded completely
        //TODO: render translucent models

        let mut opaque_models = vec![];
        let mut translucent_models = vec![];
        for model in &frame_info.models_to_render {
            if model.is_opaque {
                opaque_models.push(model);
            } else {
                translucent_models.push(model);
            }
        }

        opaque_models.sort_by(|a, b| a.material.get_id().cmp(&b.material.get_id()));

        unsafe {
            self.device.cmd_begin_render_pass(*command_buffer, &base_render_pass_info, vk::SubpassContents::INLINE);

            self.render_models(command_buffer, &opaque_models, pipeline_info.depth_motion_layout, descriptor_sets, true, render_extent, |mat| mat.depth_motion)?;

            self.device.cmd_next_subpass(*command_buffer, vk::SubpassContents::INLINE);

            self.render_models(command_buffer, &opaque_models, pipeline_info.base_render_layout, descriptor_sets, false, render_extent, |mat| mat.base_render)?;

            self.device.cmd_end_render_pass(*command_buffer);
        }

        Ok(())
    }
    unsafe fn render_models(&self, command_buffer: &vk::CommandBuffer, models: &Vec<&SingleModelRenderInfo>, pipeline_layout: vk::PipelineLayout, descriptor_sets: &[vk::DescriptorSet], is_depth_motion_pass: bool, extent: vk::Extent2D, pipeline_selector: impl Fn(&Material) -> Option<vk::Pipeline>) -> Result<()> {
        let mut current_mat_id = 0u32;

        self.device.cmd_bind_descriptor_sets(*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline_layout, 0, descriptor_sets, &[]);

        let mut is_viewport_scissor_set = false;

        for model in models {
            let mat = model.material;
            if mat.get_id() != current_mat_id {
                let render_mat = self.resource_loader.get_render_material(mat);
                if let Some(render_mat) = render_mat {
                    if let Some(pipeline) = pipeline_selector(&render_mat) {
                        current_mat_id = mat.get_id();
                        self.device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if !is_viewport_scissor_set {
                let viewport = vk::Viewport::builder()
                    .x(0.0)
                    .y(0.0)
                    .width(extent.width as f32)
                    .height(extent.height as f32)
                    .min_depth(0.0)
                    .max_depth(1.0);

                let scissor = vk::Rect2D::builder()
                    .offset(vk::Offset2D { x: 0, y: 0 })
                    .extent(extent);

                self.device.cmd_set_viewport(*command_buffer, 0, &[viewport]);
                self.device.cmd_set_scissor(*command_buffer, 0, &[scissor]);

                is_viewport_scissor_set = true;
            }

            model.render(&self.device, command_buffer, &pipeline_layout, is_depth_motion_pass, &self.resource_loader)?;
        }

        Ok(())
    }
    fn render_postprocessing(&self, command_buffer: &vk::CommandBuffer, framebuffer: &vk::Framebuffer, descriptor_sets: &[vk::DescriptorSet]) -> Result<()> {
        let pipeline_info = &self.app_data.pipeline.as_ref().unwrap();
        let swapchain_extent = self.app_data.swapchain.as_ref().unwrap().extent;

        let postprocessing_area = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(swapchain_extent);

        let postprocessing_color_clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0]
            }
        };
        let postprocessing_clear_values = &[postprocessing_color_clear_value];
        let postprocessing_pass_info = vk::RenderPassBeginInfo::builder()
            .render_pass(pipeline_info.postprocessing_render_pass)
            .framebuffer(*framebuffer)
            .render_area(postprocessing_area)
            .clear_values(postprocessing_clear_values);

        let pipeline_info = &self.app_data.pipeline.as_ref().unwrap();
        let pipeline = pipeline_info.postprocessing_pipeline;
        let pipeline_layout = pipeline_info.postprocessing_layout;

        let descriptor_set_info = &self.app_data.descriptor_sets.as_ref().unwrap();

        unsafe {
            self.device.cmd_begin_render_pass(*command_buffer, &postprocessing_pass_info, vk::SubpassContents::INLINE);

            {
                self.device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
                self.device.cmd_bind_descriptor_sets(*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline_layout, 0, descriptor_sets, &[]);

                let buffer = &descriptor_set_info.postprocessing_vertex_buffer;
                let raw_buffer = buffer.raw_buffer().unwrap();
                self.device.cmd_bind_vertex_buffers(*command_buffer, 0, &[raw_buffer], &[0]);
                self.device.cmd_draw(*command_buffer, buffer.used_element_count() as u32, 1, 0, 0);
            }

            self.device.cmd_end_render_pass(*command_buffer);
        }

        Ok(())
    }

    fn shutdown(&mut self) {
        if self.destroying {
            warn!("App::shutdown invoked more than once. Ignoring repeat.");
            return;
        }

        let duration = self.frame_info.app_start_time.elapsed();
        let current_frame_index = self.frame_info.current_frame_index;
        let avg_fps = (current_frame_index as f32) / duration.as_secs_f32();
        info!("Rendered {} frames total over {:?}. Average FPS: {}. Calculation may be incorrect if the window was minimized at any point", current_frame_index, duration, avg_fps);
        self.destroying = true;

        self.destroy();
    }

    fn destroy(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            self.scene.unload(&mut self.resource_loader);
            self.resource_loader.force_unload_all();

            for loader in self.bootstrap_loaders.iter().rev() {
                loader.before_destroy_logical_device(&self.inst, &self.device, &mut self.app_data);
            }

            debug!("Destroying Vulkan logical device...");
            self.device.destroy_device(None);

            if let Some(surface) = self.app_data.surface.take() {
                debug!("Destroying Vulkan surface KHR...");
                self.inst.destroy_surface_khr(surface, None);
            }

            for loader in self.bootstrap_loaders.iter().rev() {
                loader.before_destroy_instance(&self.inst, &mut self.app_data);
            }

            debug!("Destroying Vulkan instance...");
            self.inst.destroy_instance(None);
        }
    }
}

//TODO: add asynchronous loading of assets; move asset loading onto other threads (placeholder models/textures if things don't load fast enough)
//TODO: decouple render target format from swapchain format
//TODO: learn to use (and actually use) HDR color space
//TODO: render at a lower resolution than the swapchain-created images

//TODO: only create one sampler resource, not one per image
//TODO: use bindless rendering to support multiple textures
//TODO: deprecate static_screen_space shader, or update it to use screen coordinates and support textures/ETC
//TODO: single location for GPU memory management (allocation/freeing)
//TODO: improve game object abstraction
//TODO: add support for keyboard/mouse input
//TODO: add support for DLSS2
//TODO: add support for FSR2
//TODO: add support for fullscreen
//TODO: make texture loading asynchronous - remove usages of queue_wait_idle in favor of proper synchronization
//TODO: figure out how to use screen refresh rate
//TODO: find and integrate 3D physics engine
//TODO: support rendering text
//TODO: look into ray tracing
//TODO: add support for in-scene light sources in shaders

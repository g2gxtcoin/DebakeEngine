pub mod buffer;
pub mod cfg;
pub mod cmd;
pub mod pass;
pub mod pipeline;

//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {

    use crate::{
        ________________dev_stop________________, cast_ref, dbg_dev, dev_dbg,
        ext_api::graphic::env::{name, VkAshAPID},
        get, get_mut,
        hardware::{self, gpu::env::DseGPU},
        log::send2logger,
        manager::{
            datum::{self, env::Datum},
            execute::{
                env::TaskQueue,
                template::call_back_template::{self},
            },
        },
        model::{
            env::{ModelD, ModelE},
            mesh::{self, env::MeshD},
        },
        shader::env::ShaderModuleD,
        time::env::TimerE,
        tool::slice,
        workarea::resolution_default,
        DatumM,
    };
    use ash::{
        extensions::khr::{Surface, Swapchain, Win32Surface},
        vk::{
            self, FramebufferAttachmentsCreateInfoKHR, GraphicsPipelineCreateInfo,
            ShaderModuleCreateInfo, StructureType, SurfaceCapabilitiesKHR, SwapchainKHR,
        },
    };
    use toml::de;

    use std::{
        any::{Any, TypeId},
        fmt::Debug,
        ops::Div,
        ptr::{null, null_mut},
        thread::sleep,
    };
    use winapi::{
        ctypes::c_void,
        shared::{
            minwindef::{FALSE, HINSTANCE},
            windef::HWND,
        },
    };

    use crate::manager::execute::sub::task_interface::{self};

    use super::{
        buffer::env::{DeviceBuffer, DeviceBufferTrait, DeviceBufferUsage, SurfaceIMGBuffer},
        cfg::{
            self,
            env::{
                IMG2VIEW::{DEFAULT_COLOR, DEFAULT_DEPTH},
                IMG_FORMAT::{DEFAULT_COLOR_IMG, DEFAULT_DEPTH_IMG},
            },
        },
        cmd::{
            env::RenderCmdE,
            //sync::{env::CmdSyncD, DEFAULT_SEMAPHORE_COUNT, INDEX_GET_IMG_STAGE_SEMAPHORE},
        },
        pipeline::{
            self,
            env::{
                GraphicPipeLinePCO, GraphicPipeLinePSO, PCOTrait, PSOTrait, RenderPipelineD,
                RenderPipelineType,
            },
        },
    };

    pub enum RendererTask {
        None,
        CreateCmdBuffer(
            call_back_template::Callback2MR1R<
                // mut
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
                RenderCmdE,
                // ref
                RendererE,
            >,
        ),
        CreateSurfaceImg(
            usize, // dat surface index
            call_back_template::Callback5MR1R<
                Datum<DeviceBuffer<SurfaceIMGBuffer>>,
                VkAshAPID,
                vk::ImageCreateInfo,
                vk::ImageViewCreateInfo,
                RendererE,
                usize, // device buffer usage bitflag
            >,
            usize, //DeviceBufferUsage param usage
        ),

        CreateSurfaceColorImg(
            usize, //index
            call_back_template::Callback2MR1R<
                Datum<DeviceBuffer<SurfaceIMGBuffer>>,
                RendererE,
                i32,
            >,
            i32, //param priority
        ),

        CreateShaderMoudule(call_back_template::Callback2MR0R<Datum<ShaderModuleD>, RendererE>),

        CreateGraphicPipelineLayout(
            call_back_template::Callback2MR0R<
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
            >,
        ),

        CreateGraphicPipelinePass(
            call_back_template::Callback2MR0R<
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
            >,
        ),

        CreateGraphicPipeline(
            call_back_template::Callback2MR0R<
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
            >,
        ),

        // create framebuffer object callback
        // 用途：创建渲染管线的 framebuffer
        // mut ref: Datum<vk::Framebuffer>,
        // 用途：获取渲染管线的 framebuffer
        // mut ref: Datum<SurfaceIMG>,
        // 用途：指定渲染管线的 framebuffer 需要的 surface img
        // ref: RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>,
        // 用途：指定对应渲染管线，更新渲染管线状态
        CreateFBO(
            call_back_template::Callback4MR0R<
                Datum<DeviceBuffer<vk::Framebuffer>>,
                Datum<DeviceBuffer<SurfaceIMGBuffer>>,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
            >,
        ),

        // create vertex buffer object callback
        // mut ref: Datum<vk::Buffer>,
        // 用途：向logical device载入vk buffer数据
        // mut ref: Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
        // 用途：指定对应渲染管线，更新渲染管线状态
        // ref: Datum<MeshD>,
        // 用途：获取并载入mesh数据
        CreateVBO(
            usize, // usage
            bool,  // switch share mode
            call_back_template::Callback4MR4R<
                Datum<DeviceBuffer<vk::Buffer>>,
                Datum<ModelD>,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
                usize, // usage
                Datum<MeshD>,
                VkAshAPID,
                bool,
            >,
        ),

        // 刷新顶点缓存映射
        //
        UpdateVBO(
            usize, // mesh index
            call_back_template::Callback3MR2R<
                Datum<DeviceBuffer<vk::Buffer>>,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
                MeshD,
                VkAshAPID,
            >,
        ),

        //
        // MapMemoryBuffer(
        //     usize,// target mem buffer index
        //     Datum<>
        // )

        // also see RenderCmdE
        #[deprecated = "Abandoned enum"]
        RecordCMD(
            usize, // cmd index
            call_back_template::Callback2MR2R<
                Datum<vk::CommandBuffer>,
                RendererE,
                usize,
                vk::CommandBufferUsageFlags,
            >,
        ),

        // also see RenderCmdE
        #[deprecated = "Abandoned enum"]
        Bind_VBO(
            call_back_template::Callback3MR2R<
                Datum<DeviceBuffer<vk::Buffer>>,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
                Datum<MeshD>,
                VkAshAPID,
            >,
        ),

        // CreateCmdSync(
        //     bool, // is call in next frame
        //     call_back_template::Callback2MR1R<Datum<CmdSyncD>, RendererE, bool>,
        // ),

        // WaitFences(call_back_template::Callback0MR2R<Datum<CmdSyncD>, RendererE>),

        // CreateSemaphore(
        //     usize, // cmddsync idnex
        //     call_back_template::Callback2MR1R<Datum<CmdSyncD>, RendererE, usize>,
        // ),
        GetSurfaceImage(call_back_template::Callback1MR1R<RenderCmdE, RendererE>),
    }

    //
    #[repr(C, align(8))]
    pub struct RendererAttachment {
        // 性能优先模式，否则将按照最大适配原则开启设备可支持的尽可能多的功能
        // performance first mode, otherwise will open the most suitable device to support as many features as possible
        // 目前影响的参数
        // MemoryPropertyFlag
        // VK_MEMORY_PROPERTY_HOST_COHERENT_BIT 开启之后默认不使用 顶点缓存映射
        pub is_performance_first: bool,

        pub is_muti_queue_shared: bool,
        pub is_vertical_blank: bool, //垂直同步
        pub is_clip: bool,
        pub is_cube_surface: bool,

        pub surface_pixle_format: vk::Format,
        pub swap_level: u32,
        pub cube_surface_width: u32, // 渲染表面深度

        pub index_surface_task: usize,
        //pub index_cmd_task: usize,
        pub index_shader_mod_task: usize,
        pub index_pipeline_task: usize,
        pub index_fbo_task: usize,
        pub index_vbo_task: usize,
        pub index_cmd_buffer_task: usize,
        pub index_cmdsync_task: usize,

        pub index_gpu: usize,
        pub index_swapimg_bind_queue: u32,

        pub count_cmd: usize,
    }

    pub struct RendererExtFn {
        acquire_next_image_khr: vk::PFN_vkAcquireNextImageKHR,
    }

    impl Default for RendererExtFn {
        fn default() -> Self {
            unsafe {
                let _vk_lib = libloading::Library::new("vulkan-1.dll").unwrap();
                let _pfn_acquire_next_image_khr: libloading::Symbol<vk::PFN_vkAcquireNextImageKHR> =
                    _vk_lib.get(b"vkAcquireNextImageKHR").unwrap();
                return Self {
                    acquire_next_image_khr: std::mem::transmute(_pfn_acquire_next_image_khr),
                };
            }
        }
    }

    pub struct RendererE {
        pub id: u64,
        pub frame_stride_ns: u64,

        pub renderer_attachment: RendererAttachment,
        pub wnd_handle: HWND,
        pub mod_handle: HINSTANCE,

        gpu_p: Option<usize>,
        timer_p: Option<usize>,

        // pub gpu_properties: Option<ash::vk::PhysicalDeviceProperties>,
        pub swapchain: Option<vk::SwapchainKHR>,
        pub swapchain_create_info: Option<vk::SwapchainCreateInfoKHR>,
        pub swapchain_loader: Option<Swapchain>,

        pub surface_create_info: Option<vk::Win32SurfaceCreateInfoKHR>, //hwnd:c_void,
        pub renderer_surface: Option<vk::SurfaceKHR>,

        pub semaphore_create_info: Option<vk::SemaphoreCreateInfo>,
    }

    impl Default for RendererE {
        fn default() -> Self {
            return Self {
                id: 0,
                frame_stride_ns: cfg::env::RENDERER::DEFAULT_RENDER_FRAME_STRIDE,
                gpu_p: Option::None,
                swapchain: Option::None,
                surface_create_info: Option::None,
                wnd_handle: null_mut(),
                mod_handle: null_mut(),
                renderer_surface: Option::None,
                swapchain_loader: Option::None,

                renderer_attachment: RendererAttachment::default(),
                swapchain_create_info: Option::None,

                timer_p: Option::None,
                semaphore_create_info: Option::None,
                // gpu_properties: Option::None,
            };
        }
    }

    impl RendererE {
        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }

        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }

        pub fn index_gpu(&self) -> usize {
            return self.renderer_attachment.index_gpu;
        }

        pub fn device_ref(&self) -> Result<&ash::Device, ()> {
            return Ok(cast_ref!(DseGPU, self.gpu_p.unwrap())
                .logical_p
                .as_ref()
                .unwrap());
        }

        pub fn gpu_ref(&self) -> Result<&DseGPU, ()> {
            return Ok(&cast_ref!(DseGPU, self.gpu_p.unwrap()));
        }

        /// # Abstract
        /// - 创建并分配 渲染器所需要使用缓存
        /// - device memory 将在其中创建,并由device_buffer: &mut DeviceBuffer<vk::Buffer>参数输出.
        /// ## Example
        /**
            renderer_slice.alloc_device_mem(
                mesh.buffer_mem_size(),
                vk::BufferUsageFlags::_,
                *api_bind.gpu_mem_properties_current_ref().unwrap(),
                &mut _vbo,
            );
        */
        /// ## Parameter
        pub fn alloc_device_mem(
            &self,
            mem_size: u64,
            mem_usage: vk::BufferUsageFlags,
            current_mem_properties: &vk::PhysicalDeviceMemoryProperties,
            device_buffer: &mut DeviceBuffer<vk::Buffer>,
            switch_share_mode: bool,
        ) {
            let _gpu = cast_ref!(DseGPU, self.gpu_p.unwrap());
            // request mem from device by vk instance
            let buffer_info = vk::BufferCreateInfo {
                s_type: vk::StructureType::BUFFER_CREATE_INFO,
                p_next: null(),
                flags: vk::BufferCreateFlags::default(),
                size: mem_size,
                usage: mem_usage,
                sharing_mode: match switch_share_mode {
                    true => vk::SharingMode::CONCURRENT,
                    false => vk::SharingMode::EXCLUSIVE,
                },
                queue_family_index_count: _gpu.queue_families.as_ref().unwrap().len() as u32,
                p_queue_family_indices: match switch_share_mode {
                    true => slice::to_num_slice(_gpu.queue_families.as_ref().unwrap().len()),
                    false => null(),
                },
            };

            let buffer: vk::Buffer = unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .create_buffer(&buffer_info, Option::None)
                    .unwrap()
            };

            // get mem requirement info from device by vk instance
            let memory_type_index = self._find_suitable_mem_type(
                DeviceBufferUsage::get_vk_mem_mapping_type(device_buffer.usage_ref().unwrap()),
                &buffer,
                &current_mem_properties,
            );

            let _alloc_info: vk::MemoryAllocateInfo = vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: null(),
                allocation_size: mem_size,
                memory_type_index: memory_type_index as u32,
            };
            let mem = unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .allocate_memory(&_alloc_info, Option::None)
                    .unwrap()
            };

            device_buffer.set_devicemem(mem);
            device_buffer.set_buffer(buffer);

            // bind memory
            unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .bind_buffer_memory(
                        *device_buffer.buffer_ref().unwrap(),
                        *device_buffer.device_mem_ref().unwrap(),
                        0,
                    )
                    .expect("bind vk buffer_memory fail");
            };
        }

        pub fn build() -> Self {
            return Default::default();
        }

        pub fn build_bind_gpu(mut self, index: usize) -> Self {
            self.renderer_attachment.index_gpu = index;
            return self;
        }

        pub fn build_bind_timer_exe(mut self, timer_in: &TimerE) -> Self {
            self.timer_p = Some(timer_in as *const TimerE as usize);
            return self;
        }

        pub fn build_set_performance_first(mut self, bool_in: bool) -> Self {
            self.renderer_attachment.is_performance_first = bool_in;
            return self;
        }

        pub fn build_specify_handle(mut self, hwnd_in: HWND, mod_handle_in: HINSTANCE) -> Self {
            self.wnd_handle = hwnd_in;
            self.mod_handle = mod_handle_in;
            return self;
        }

        pub fn build_specify_api_base2create_surface(mut self, api_in: &mut VkAshAPID) -> Self {
            self.gpu_p = Some(api_in.gpu_ref(self.renderer_attachment.index_gpu).unwrap()
                as *const DseGPU as usize);

            self._create_surface(api_in);
            return self;
        }

        pub fn build_device_suitable_surface(self, api_in: &mut VkAshAPID) -> Self {
            let _gpu = api_in.gpu_ref(self.renderer_attachment.index_gpu).unwrap();
            for index in 0.._gpu.queue_families.as_ref().unwrap().len() {
                unsafe {
                    match Win32Surface::new(
                        api_in.ash_entry_ref().unwrap(),
                        api_in.ash_instance_ref().unwrap(),
                    )
                    .get_physical_device_win32_presentation_support(
                        *_gpu.physical_p.as_ref().unwrap(),
                        index as u32,
                    ) {
                        true => crate::log::send2logger(
                            crate::log::code::TYPE_EXE_INFO
                                | crate::log::code::CONDI_GPU_SURFACE_SUILTABAL
                                | crate::log::code::FILE_RENDERER
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        line!() as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_LINE,
                                    )
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        self.id as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                                    )
                                    .get_code(),
                        ),
                        false => crate::log::send2logger(
                            crate::log::code::TYPE_EXE_ERROR
                                | crate::log::code::CONDI_GPU_SURFACE_NOT_SUILTABAL
                                | crate::log::code::FILE_RENDERER
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        line!() as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_LINE,
                                    )
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        self.id as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                                    )
                                    .get_code(),
                        ),
                    }
                };
            }
            return self;
        }

        pub fn build_fps(mut self, fps_in: u64) -> Self {
            self.frame_stride_ns = 1000_0000.div(fps_in);
            return self;
        }

        pub fn build_api_surpport(mut self) -> Self {
            return self;
        }

        // ensure call it before build_swap_buffer
        pub fn build_set_surface_pixle_formate(mut self, formate_in: vk::Format) -> Self {
            self.renderer_attachment.surface_pixle_format = formate_in;
            return self;
        }

        // ensure call it before build_swap_buffer
        pub fn build_set_swap_buffer_level(mut self, level_in: u32) -> Self {
            self.renderer_attachment.swap_level = level_in;
            return self;
        }

        // ensure call it before build_swap_buffer
        pub fn build_set_pixle_format(mut self, formate_in: vk::Format) -> Self {
            self.renderer_attachment.surface_pixle_format = formate_in;
            return self;
        }

        // ensure call it before build_swap_buffer
        pub fn build_set_defer_rendering(mut self, bool_in: bool) -> Self {
            self.renderer_attachment.is_muti_queue_shared = bool_in;
            return self;
        }

        // ensure call it before build_swap_buffer
        pub fn build_set_vertical_blank(mut self, bool_in: bool) -> Self {
            self.renderer_attachment.is_vertical_blank = bool_in;
            return self;
        }

        // ensure call it before build_swap_buffer
        pub fn build_set_clip(mut self, bool_in: bool) -> Self {
            self.renderer_attachment.is_clip = bool_in;
            return self;
        }

        // ensure call it after build a device
        pub fn build_swap_buffer(mut self, api_in: &mut VkAshAPID) -> Self {
            let _gpu = api_in.gpu_ref(self.renderer_attachment.index_gpu).unwrap();

            self.swapchain_loader = Some(Swapchain::new(
                api_in.ash_instance_ref().unwrap(),
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap(),
            ));
            let surface_capabilities = unsafe {
                api_in
                    .surface_loader_ref()
                    .as_ref()
                    .unwrap()
                    .get_physical_device_surface_capabilities(
                        *_gpu.physical_p.as_ref().unwrap(),
                        *self.renderer_surface.as_ref().unwrap(),
                    )
                    .unwrap_or_else(|_| -> SurfaceCapabilitiesKHR {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_EXE_ERROR
                                | crate::log::code::CONDI_UNEXPECTED_RESULT
                                | crate::log::code::FILE_RENDERER
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        line!() as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                    )
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        self.id as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                    )
                                    .get_code()
                        );
                        SurfaceCapabilitiesKHR::default()
                    })
            };

            let surface_formate = unsafe {
                api_in
                    .surface_loader_ref()
                    .as_ref()
                    .unwrap()
                    .get_physical_device_surface_formats(
                        *_gpu.physical_p.as_ref().unwrap(),
                        *self.renderer_surface.as_ref().unwrap(),
                    )
                    .unwrap()
            };
            let surface_formate = match surface_formate
                .iter()
                .find(|&&x| x.format == self.renderer_attachment.surface_pixle_format)
            {
                Some(val) => val,
                None => &(surface_formate[0]),
            };

            let surface_present_mode = unsafe {
                api_in
                    .surface_loader_ref()
                    .as_ref()
                    .unwrap()
                    .get_physical_device_surface_present_modes(
                        *_gpu.physical_p.as_ref().unwrap(),
                        *self.renderer_surface.as_ref().unwrap(),
                    )
                    .unwrap()
            };
            let surface_present_mode = match self.renderer_attachment.is_vertical_blank {
                true => {
                    match surface_present_mode
                        .iter()
                        .find(|&&x| x == vk::PresentModeKHR::MAILBOX)
                    {
                        Some(val) => val,
                        None => &vk::PresentModeKHR::FIFO,
                    }
                }
                false => &vk::PresentModeKHR::IMMEDIATE,
            };

            // dbg!(&surface_capabilities);

            self.swapchain_create_info = Option::Some(vk::SwapchainCreateInfoKHR {
                s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
                p_next: null(),
                flags: vk::SwapchainCreateFlagsKHR::default(),
                surface: *self.renderer_surface.as_ref().unwrap(),
                min_image_count: surface_capabilities.min_image_count,
                //+ self.renderer_attachment.swap_level, /* - 1 */
                image_format: surface_formate.format,
                image_color_space: surface_formate.color_space,
                image_extent: surface_capabilities.current_extent, // will be change when custom redecide wnd height&wide
                image_array_layers: match self.renderer_attachment.is_cube_surface {
                    true => 1 + self.renderer_attachment.cube_surface_width,
                    false => 1,
                },
                image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
                image_sharing_mode: match self.renderer_attachment.is_muti_queue_shared {
                    true => match _gpu.queue_families.as_ref().unwrap().len() {
                        0 => vk::SharingMode::EXCLUSIVE,
                        1 => vk::SharingMode::EXCLUSIVE,
                        _ => vk::SharingMode::CONCURRENT,
                    },
                    false => vk::SharingMode::EXCLUSIVE,
                },
                queue_family_index_count: _gpu.queue_families.as_ref().unwrap().len() as u32,
                p_queue_family_indices: match self.renderer_attachment.is_muti_queue_shared {
                    false => &self.renderer_attachment.index_swapimg_bind_queue,
                    true => null(),
                },
                pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
                composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
                present_mode: *surface_present_mode,
                clipped: self.renderer_attachment.is_clip as u32,
                old_swapchain: vk::SwapchainKHR::default(),
            });

            self.swapchain = unsafe {
                Option::Some(
                    self.swapchain_loader
                        .as_ref()
                        .unwrap()
                        .create_swapchain(
                            self.swapchain_create_info.as_ref().unwrap(),
                            Option::None,
                        )
                        .unwrap(),
                )
            };

            //api_in.
            return self;
        }

        pub fn build_set_pipeline_dynamic_state_auto(mut self) -> Self {
            //self.pipeline_dynamic_state.as_mut().unwrap().push(vk::DynamicState::);
            return self;
        }

        pub fn build_semap_info(mut self) -> Self {
            self.semaphore_create_info = Some(vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: null(),
                flags: vk::SemaphoreCreateFlags::default(),
            });
            return self;
        }

        pub fn tak_prepare_next_surfimg(&self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            let _task = get_mut!(tin.vec_mut(), self.renderer_attachment.index_cmdsync_task);

            _task.push_task(RendererTask::GetSurfaceImage(
                Self::_callback_prepare_next_surfimg,
            ));
        }

        pub fn prepare_surfimg(&mut self, cmd_slice: &mut RenderCmdE) {
            Self::_callback_prepare_next_surfimg(cmd_slice, self);
        }

        pub fn present_surfimg_color(&self, cmd_slice: &mut RenderCmdE) {
            Self::_callback_present_surfimg_color(cmd_slice, self)
        }

        fn _callback_present_surfimg_color(cmd_slice: &mut RenderCmdE, renderer_slice: &RendererE) {
            let _queue = unsafe { renderer_slice.device_ref().unwrap().get_device_queue(0, 0) };
            let _swap = renderer_slice.swapchain_loader.as_ref().unwrap();

            let _info = vk::PresentInfoKHR {
                s_type: vk::StructureType::PRESENT_INFO_KHR,
                p_next: null(),
                wait_semaphore_count: cmd_slice.semaps_ref().len() as u32,
                p_wait_semaphores: cmd_slice.semaps_ref().as_ptr(),
                swapchain_count: 1,
                p_swapchains: renderer_slice.swapchain.as_ref().unwrap(),
                p_image_indices: &cmd_slice.cmd_attachment.index_current_surfimg_buf,
                p_results: null_mut(),
            };
            unsafe {
                _swap.queue_present(_queue, &_info).unwrap();
            }
        }

        fn _callback_prepare_next_surfimg(cmd_slice: &mut RenderCmdE, renderer_slice: &RendererE) {
            let _swap = renderer_slice.swapchain_loader.as_ref().unwrap();

            unsafe {
                cmd_slice.cmd_attachment.index_current_surfimg_buf = _swap
                    .acquire_next_image(
                        *renderer_slice.swapchain.as_ref().unwrap(),
                        u64::MAX,
                        *get!(
                            cmd_slice.semaps_ref(),
                            crate::renderer::cmd::sync::INDEX_GET_IMG_STAGE_SEMAPHORE
                        ),
                        *get!(cmd_slice.fences_ref(), 0), //todo!()
                    )
                    .unwrap()
                    .0;

                // let _r = (RendererExtFn::default().;acquire_next_image_khr)(
                //     _device_p,
                //     *renderer_slice.swapchain.as_ref().unwrap(),
                //     u64::MAX,
                //     *get!(
                //         csi.as_ref().unwrap().semaphore_vec_ref(),
                //         INDEX_GET_IMG_STAGE_SEMAPHORE
                //     ),
                //     *get!(
                //         csi.as_ref().unwrap().fences_ref(),
                //         0
                //     ),
                //     &mut renderer_slice
                //         .renderer_attachment
                //         .index_current_color_swapimg,
                // );
            }
        }

        pub fn tak_create_pipeline_layout(
            &mut self,
            pipe_type: RenderPipelineType,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            match pipe_type {
                RenderPipelineType::None => todo!(),
                RenderPipelineType::Graphic => {
                    get_mut!(tin.vec_mut(), self.renderer_attachment.index_pipeline_task).push_task(
                        RendererTask::CreateGraphicPipelineLayout(
                            Self::_callback_create_pipeline_layout,
                        ),
                    )
                }
                RenderPipelineType::Compute => todo!(),
                RenderPipelineType::RayTracing => todo!(),
            }
        }

        pub fn tak_create_graphic_pipeline_pass(&self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_pipeline_task).push_task(
                RendererTask::CreateGraphicPipelinePass(
                    Self::_callback_create_graphic_pipeline_pass,
                ),
            )
        }

        pub fn tak_create_graphic_pipeline(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_pipeline_task).push_task(
                RendererTask::CreateGraphicPipeline(Self::_callback_create_graphic_pipeline),
            )
        }

        /// 创建命令缓冲
        /// # Abstract
        /// - index 0 is main cmd buffer
        /// - index behine 0 are secondary cmd buffer
        /// ## Example
        /// ## Parameter
        pub fn tak_create_cmd_buffer(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            //judge inherit task queue offset
            get_mut!(
                tin.vec_mut(),
                self.renderer_attachment.index_cmd_buffer_task
            )
            .push_task(RendererTask::CreateCmdBuffer(
                Self::_callback_create_cmd_buffer,
            ))
        }

        #[allow(unused)]
        pub fn tak_create_color_surface_img_view(
            &mut self,
            surf_img_index: usize,
            priority: i32,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_surface_task).push_task(
                RendererTask::CreateSurfaceColorImg(
                    surf_img_index,
                    Self::_callback_create_color_surface_img_view,
                    priority,
                ),
            );
        }

        pub fn tak_create_custom_surface_img_view(
            &mut self,
            surf_img_index: usize,
            usage: usize,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_surface_task).push_task(
                RendererTask::CreateSurfaceImg(
                    surf_img_index,
                    Self::_callback_create_custom_surface_img_view,
                    usage,
                ),
            );
        }

        pub fn tak_create_shader_module(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            get_mut!(
                tin.vec_mut(),
                self.renderer_attachment.index_shader_mod_task
            )
            .push_task(RendererTask::CreateShaderMoudule(
                Self::_callback_create_shader_moudule,
            ));
        }

        #[deprecated = "use `RenderCmdE internal fn instead` instead"]
        pub fn tak_wait_fences(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            // get_mut!(tin.vec_mut(), self.renderer_attachment.index_cmdsync_task)
            //     .as_mut()
            //     .unwrap()
            //     .push_task(RendererTask::WaitFences(Self::_callback_wait_fences));
        }

        #[allow(unused)]
        #[deprecated = "use `tak_create_cmdsync` instead"]
        pub fn tak_create_semaphore(
            &mut self,
            index: usize,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            // let task = get_mut!(tin.vec_mut(), self.renderer_attachment.index_cmdsync_task);
            // task.as_mut()
            //     .unwrap()
            //     .push_task(RendererTask::CreateSemaphore(
            //         index,
            //         Self::_callback_create_semaphore,
            //     ));
        }

        #[allow(unused)]
        #[deprecated = "use `_callback_create_cmdsync` instead"]
        fn _callback_create_semaphore(
            // datum_sync: &mut Datum<CmdSyncD>,
            renderer_slice: &mut RendererE,
            index_cmdsync: &usize,
        ) {
            // let _r: vk::Semaphore;

            // unsafe {
            //     _r = renderer_slice
            //         .device_ref()
            //         .unwrap()
            //         .create_semaphore(&renderer_slice.semaphore_create_info.unwrap(), Option::None)
            //         .unwrap();
            // }

            // get_mut!(datum_sync.vec_mut(), *index_cmdsync)
            //     .as_mut()
            //     .unwrap()
            //     .semaphore_vec_mut()
            //     .push(_r);
        }

        // pub fn exe_render_cmdsync(
        //     &mut self,
        //     datum_sync: &mut Datum<CmdSyncD>,
        //     tin: &mut Datum<TaskQueue<RendererTask>>,
        // ) {
        //     let mut _tasks = get_mut!(tin.vec_mut(), self.renderer_attachment.index_cmdsync_task)
        //         .as_mut()
        //         .unwrap();
        //     _tasks.begin_execute();
        //     for ti in _tasks.task_iter_mut().unwrap() {
        //         match task_interface::TaskTrait::task_mut(ti) {
        //             RendererTask::CreateCmdSync(singal, call) => {
        //                 call(datum_sync, self, singal);
        //             }
        //             RendererTask::WaitFences(call) => {
        //                 call(datum_sync, self);
        //             }
        //             RendererTask::GetSurfaceImage(call) => {
        //                 call(datum_sync, self);
        //             }
        //             _ => {}
        //         }
        //     }
        //     _tasks.end_execute();
        // }

        /// # Abstract
        /// - 创建顶点缓存对象
        /// - 前置条件：渲染管线 交换链
        /// ## Example
        /**
         * exe.renderer1.create_vbo(
         *     DeviceBufferUsage::MEM_TYPE_RAM_COHERENT, // 指定内存映射方式:
         *     tak.render_task.get_data_mut(exe.renderer1.id).unwrap(),
         * );   
         */
        /// ## Parameter Explain
        /**
         * &mut self: renderer 的可变引用
         * usage_in: DeviceBufferUsage 用于指定 内存映射方式
         * tin: &mut Datum<TaskQueue<RendererTask>>
         */
        pub fn tak_create_vbo(
            &mut self,
            usage_in: usize,
            switch_share_mode: bool,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_vbo_task).push_task(
                RendererTask::CreateVBO(usage_in, switch_share_mode, Self::_callback_create_vbo),
            );
        }

        /// 前置条件：
        /// 渲染管线
        /// 交换链
        ///
        pub fn tak_create_fbo(&self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_fbo_task)
                .push_task(RendererTask::CreateFBO(Self::_callback_create_fbo))
        }

        pub fn update_swapcahin(&mut self) {
            todo!();
        }

        pub fn tak_update_specific_vbo(
            &mut self,
            mesh_index: usize,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            get_mut!(tin.vec_mut(), self.renderer_attachment.index_vbo_task).push_task(
                RendererTask::UpdateVBO(mesh_index, Self::_callback_update_vbo),
            );
        }

        /// # Abstract
        /// - Abandoned feature
        /// - Feature has include in create_vbo
        /// - Define as your device_buffer_usage custom or device property default setting.
        #[deprecated = "abandoned feature"]
        pub fn tak_map_vertex_buffer(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            todo!();
        }

        /// use proc macro to rebuild it
        pub fn bind_task_queue(&mut self, tqin: &mut Datum<TaskQueue<RendererTask>>) {
            self.renderer_attachment.index_surface_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.renderer_attachment.index_shader_mod_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.renderer_attachment.index_pipeline_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.renderer_attachment.index_fbo_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.renderer_attachment.index_vbo_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.renderer_attachment.index_cmd_buffer_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.renderer_attachment.index_cmdsync_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
        }

        pub fn bind_timer_exe(&mut self, timer_in: &TimerE) {
            self.timer_p = Some(timer_in as *const TimerE as usize);
        }

        // 执行与 vertex buffer 相关所有指令
        // 1.创建
        // 2.销毁
        // 3.挂起
        pub fn exe_vertex_buffer(
            &mut self,
            datum_vkbuf: &mut Datum<DeviceBuffer<vk::Buffer>>,
            datum_model: &mut Datum<ModelD>,
            datum_mesh: &Datum<MeshD>,
            pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            api_bind: &VkAshAPID,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = get_mut!(tin.vec_mut(), self.renderer_attachment.index_vbo_task);
            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateVBO(uin, switch, call) => {
                        call(
                            datum_vkbuf,
                            datum_model,
                            pipeline,
                            self,
                            uin,
                            datum_mesh,
                            api_bind,
                            switch,
                        );
                    }
                    RendererTask::UpdateVBO(mesh_index, call) => {
                        call(
                            datum_vkbuf,
                            pipeline,
                            self,
                            get!(datum_mesh.vec_ref(), *mesh_index),
                            api_bind,
                        );
                    }
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        //
        pub fn exe_fbo(
            &mut self,
            datum_fbo: &mut Datum<DeviceBuffer<vk::Framebuffer>>,
            datum_surfimg: &mut Datum<DeviceBuffer<SurfaceIMGBuffer>>,
            datum_pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = get_mut!(tin.vec_mut(), self.renderer_attachment.index_fbo_task);
            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateFBO(call) => {
                        call(datum_fbo, datum_surfimg, datum_pipeline, self);
                    }
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn exe_shader_module(
            &mut self,
            datum: &mut Datum<ShaderModuleD>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _task = get_mut!(
                tin.vec_mut(),
                self.renderer_attachment.index_shader_mod_task
            );
            _task.begin_execute();
            for ti in _task.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateShaderMoudule(call) => call(datum, self),
                    _ => {}
                }
            }
            _task.end_execute();
        }

        pub fn exe_surface_img(
            &mut self,
            datum: &mut Datum<DeviceBuffer<SurfaceIMGBuffer>>,
            api_in: &mut VkAshAPID,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = get_mut!(tin.vec_mut(), self.renderer_attachment.index_surface_task);
            _tasks.begin_execute();
            // let self_rc=Rc::new(RefCell::new(self)) ;
            // let self_rc_i=Rc::clone(&self_rc);
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateSurfaceColorImg(index, call, priority) => {
                        call(datum, self, &priority)
                    }
                    RendererTask::CreateSurfaceImg(index, call, usage) => call(
                        datum,
                        api_in,
                        DeviceBufferUsage::get_img_info(*usage).as_mut().unwrap(),
                        DeviceBufferUsage::get_img_view_info(*usage)
                            .as_mut()
                            .unwrap(),
                        self,
                        &usage,
                    ),

                    _ => todo!(),
                };
            }

            _tasks.end_execute();
        }

        pub fn exe_model(
            &mut self,
            data: &mut Datum<DeviceBuffer<ModelE>>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            todo!();
        }

        /// #[deprecated = "Abandoned Feature"]
        /// Abandoned Feature
        /// also see same name feature in crate::renderer::cmd::RenderCmdE
        pub fn exe_cmd_buffer(
            &mut self,
            cmd_slice: &mut RenderCmdE,
            data: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = get_mut!(
                tin.vec_mut(),
                self.renderer_attachment.index_cmd_buffer_task
            );

            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_ref(ti) {
                    RendererTask::None => {}

                    //  RendererTask::InitSubmitInfo4QueueCmd(call) => call(cmd_slice, self),
                    RendererTask::CreateCmdBuffer(call) => call(
                        data, cmd_slice,
                        self,
                        // cast_ref!(DseGPU, self.gpu_p.unwrap())
                        //     .logical_p
                        //     .as_ref()
                        //     .unwrap(),
                    ),
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn exe_graphic_pipeline(
            &mut self,
            data: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _task = get_mut!(tin.vec_mut(), self.renderer_attachment.index_pipeline_task);
            _task.begin_execute();
            for ti in _task.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateGraphicPipelinePass(call) => call(data, self),
                    RendererTask::CreateGraphicPipelineLayout(call) => call(data, self),
                    RendererTask::CreateGraphicPipeline(call) => call(data, self),
                    _ => {}
                }
            }
            _task.end_execute();
        }

        pub fn exe_compute_pipeline(&mut self) {}

        pub fn exe_ray_trace_pipeline(&mut self) {}

        fn _find_suitable_mem_type(
            &self,
            default_type: vk::MemoryPropertyFlags,
            buf_in: &vk::Buffer,
            property_in: &vk::PhysicalDeviceMemoryProperties,
        ) -> usize {
            // get mem requirement info from device by vk instance
            let mem_req: vk::MemoryRequirements = unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .get_buffer_memory_requirements(*buf_in)
            };
            let _req_type =
                vk::MemoryPropertyFlags::from_raw(default_type.as_raw() | mem_req.memory_type_bits);

            // dev_dbg!(&default_type);
            // dev_dbg!(_req_type);
            // dev_dbg!(vk::MemoryPropertyFlags::from_raw(mem_req.memory_type_bits));

            let mut _r = usize::MAX;
            let mut _r_type_flag = 0;

            // dev_dbg!(self.renderer_attachment.is_performance_first);
            // dev_dbg!(proper_in);
            for tei in property_in.memory_types.iter().enumerate() {
                if tei.1.property_flags.contains(_req_type) {
                    if self.renderer_attachment.is_performance_first {
                        return tei.0;
                    } else if _r_type_flag < tei.1.property_flags.as_raw() {
                        _r_type_flag = tei.1.property_flags.as_raw();
                        _r = tei.0;
                    }
                }
            }
            if _r == usize::MAX {
                for tei in property_in.memory_types.iter().enumerate() {
                    if tei
                        .1
                        .property_flags
                        .contains(vk::MemoryPropertyFlags::from_raw(mem_req.memory_type_bits))
                    {
                        if self.renderer_attachment.is_performance_first {
                            return tei.0;
                        } else if _r_type_flag < tei.1.property_flags.as_raw() {
                            _r_type_flag = tei.1.property_flags.as_raw();
                            _r = tei.0;
                        }
                    }
                }
            }
            if _r == usize::MAX {
                for tei in property_in.memory_types.iter().enumerate() {
                    if tei.1.property_flags.contains(default_type) {
                        if self.renderer_attachment.is_performance_first {
                            return tei.0;
                        } else if _r_type_flag < tei.1.property_flags.as_raw()
                            && vk::MemoryPropertyFlags::from_raw(mem_req.memory_type_bits)
                                .contains(tei.1.property_flags)
                        {
                            _r_type_flag = tei.1.property_flags.as_raw();
                            _r = tei.0;
                        }
                    }
                }
            }
            if _r == usize::MAX {
                for tei in property_in.memory_types.iter().enumerate() {
                    if tei.1.property_flags.contains(default_type) {
                        if self.renderer_attachment.is_performance_first {
                            return tei.0;
                        } else if _r_type_flag < tei.1.property_flags.as_raw()
                        // && vk::MemoryPropertyFlags::from_raw(mem_req.memory_type_bits)
                        //     .contains(tei.1.property_flags)
                        {
                            _r_type_flag = tei.1.property_flags.as_raw();
                            _r = tei.0;
                        }
                    }
                }
            }
            if _r == usize::MAX {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXE_WARN
                        | crate::log::code::CONDI_CUSTOM_PARAM_NOT_FOUND
                        | crate::log::code::FILE_RENDERER
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                            .get_code()
                );
                return 0;
            }
            // dev_dbg!(vk::MemoryPropertyFlags::from_raw(_r_type_flag));
            return _r;
        }

        fn _alloc_device_mem_surfimg(
            &self,
            mem_size: u64,
            mem_usage: vk::BufferUsageFlags,
            current_mem_properties: &vk::PhysicalDeviceMemoryProperties,
            surf_buffer: &mut DeviceBuffer<SurfaceIMGBuffer>,
        ) {
            // request mem from device by vk instance
            let buffer_info = vk::BufferCreateInfo {
                s_type: vk::StructureType::BUFFER_CREATE_INFO,
                p_next: null(),
                flags: vk::BufferCreateFlags::default(),
                size: mem_size,
                usage: mem_usage,
                sharing_mode: match self.renderer_attachment.is_muti_queue_shared {
                    true => vk::SharingMode::CONCURRENT,
                    false => vk::SharingMode::EXCLUSIVE,
                },
                queue_family_index_count: self
                    .gpu_ref()
                    .unwrap()
                    .queue_families
                    .as_ref()
                    .unwrap()
                    .len() as u32,
                p_queue_family_indices: null(),
            };

            let _vkbuffer: vk::Buffer = unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .create_buffer(&buffer_info, Option::None)
                    .unwrap()
            };

            // get mem requirement type from current gpu
            let memory_type_index = self._find_suitable_mem_type(
                DeviceBufferUsage::get_vk_mem_mapping_type(surf_buffer.usage_ref().unwrap()),
                &_vkbuffer,
                current_mem_properties,
            );

            let _alloc_info: vk::MemoryAllocateInfo = vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: null(),
                allocation_size: mem_size,
                memory_type_index: memory_type_index as u32,
            };
            let mem = unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .allocate_memory(&_alloc_info, Option::None)
                    .expect("Failed to allocate memory")
            };

            surf_buffer.set_devicemem(mem);
            surf_buffer.buffer_mut().unwrap().set_vkbuffer(_vkbuffer);

            unsafe {
                cast_ref!(DseGPU, self.gpu_p.unwrap())
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .bind_image_memory(
                        *surf_buffer.buffer_mut().unwrap().img_mut(),
                        *surf_buffer.device_mem_ref().unwrap(),
                        0,
                    )
                    .expect("bind custom image_memory fail")
            };
        }

        fn _create_surface(&mut self, api_in: &mut VkAshAPID) {
            if api_in.check_ext_name_exist(name::khr::WIN32_SURFACE.as_ptr())
                && !self.wnd_handle.is_null()
                && !self.mod_handle.is_null()
            {
                let mut api_win32_surface = Option::Some(Win32Surface::new(
                    api_in.ash_entry_ref().unwrap(),
                    api_in.ash_instance_ref().unwrap(),
                ));
                self.surface_create_info = Option::Some(vk::Win32SurfaceCreateInfoKHR {
                    s_type: vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
                    p_next: null(),
                    flags: vk::Win32SurfaceCreateFlagsKHR::default(),
                    hinstance: self.mod_handle as *const c_void,
                    hwnd: self.wnd_handle as *const c_void,
                });
                unsafe {
                    self.renderer_surface = Option::Some(
                        api_win32_surface
                            .as_mut()
                            .unwrap()
                            .create_win32_surface(
                                self.surface_create_info.as_ref().unwrap(),
                                Option::None,
                            )
                            .expect("no! create_win32_surface fail "),
                    );
                }
            } else {
                crate::log::send2logger(
                    crate::log::code::TYPE_EXE_ERROR
                        | crate::log::code::CONDI_CREATE_SURFACE_FALI
                        | crate::log::code::FILE_RENDERER
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                            .get_code(),
                )
            }
        }

        fn _callback_create_fbo(
            datum_fbo: &mut Datum<DeviceBuffer<vk::Framebuffer>>,
            datum_surfimg: &mut Datum<DeviceBuffer<SurfaceIMGBuffer>>,
            datum_pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
        ) {
            let mut _attachments = Vec::new();
            for si in datum_surfimg.vec_ref() {
                // dbg!(&((si.as_ref().unwrap())));
                _attachments.push(si.buffer_ref().unwrap().view_ref());
            }

            if renderer_slice.swapchain_create_info.is_some() {
                for rpi in datum_pipeline.vec_mut().iter() {
                    let mut _buf: DeviceBuffer<vk::Framebuffer> =
                        DeviceBuffer::<vk::Framebuffer>::default();
                    let _count = rpi
                        .pso_ref()
                        .renderpass_info_ref()
                        .unwrap()
                        .attachment_count;

                    let _p_attachment = rpi.pso_ref().renderpass_info_ref().unwrap().p_attachments;

                    let _info = vk::FramebufferCreateInfo {
                        s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                        p_next: null(),
                        flags: match _count == 0 {
                            true => vk::FramebufferCreateFlags::IMAGELESS,
                            false => Default::default(),
                        },
                        render_pass: rpi.pco_ref().pass_ref().unwrap().clone(), //?
                        attachment_count: _count,
                        p_attachments: _attachments.as_ptr(), //?
                        width: renderer_slice
                            .swapchain_create_info
                            .unwrap()
                            .image_extent
                            .width,
                        height: renderer_slice
                            .swapchain_create_info
                            .unwrap()
                            .image_extent
                            .height,
                        layers: _attachments.len() as u32,
                    };

                    unsafe {
                        _buf.set_buffer(
                            renderer_slice
                                .gpu_ref()
                                .unwrap()
                                .logical_p
                                .as_ref()
                                .unwrap()
                                .create_framebuffer(&_info, Option::None)
                                .unwrap(),
                        )
                    }

                    datum_fbo.alloc_data(_buf, Option::None);
                }
            }
        }

        /// 创建图形管线内部回调
        fn _callback_create_graphic_pipeline<TStates, TArrays>(
            datum: &mut Datum<RenderPipelineD<TStates, TArrays>>,
            renderer_slice: &mut RendererE,
        ) where
            TStates: super::pipeline::env::PSOTrait + Clone + Any,
            TArrays: super::pipeline::env::PCOTrait + Any,
        {
            let mut _pipeline_info_slice = Vec::<GraphicsPipelineCreateInfo>::new();
            for ri in datum.vec_mut() {
                match ri.pipeline_info() {
                    pipeline::env::PipelineCreateInfoResult::None => {}
                    pipeline::env::PipelineCreateInfoResult::Graphic(val) => {
                        // 开发模式下打印 图形管线信息
                        // dev_dbg!(&val);
                        _pipeline_info_slice.push(val);
                    }
                    pipeline::env::PipelineCreateInfoResult::Compute(_) => {}
                    pipeline::env::PipelineCreateInfoResult::RayTracing(_) => {}
                }
            }

            let _pipelines = unsafe {
                renderer_slice
                    .gpu_ref()
                    .unwrap()
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .create_graphics_pipelines(
                        Default::default(),
                        _pipeline_info_slice.as_slice(),
                        Option::None,
                    )
                    .unwrap()
            };

            for ri in datum
                .vec_mut()
                .iter_mut()
                .filter(|x| match x.render_pipeline_type() {
                    RenderPipelineType::Graphic => true,
                    _ => false,
                })
                .enumerate()
            {
                ri.1.set_pipeline(_pipelines[ri.0]);
            }
        }

        fn _callback_create_pipeline_layout<TStates, TArrays>(
            datum: &mut Datum<RenderPipelineD<TStates, TArrays>>,
            renderer_slice: &mut RendererE,
        ) where
            TStates: super::pipeline::env::PSOTrait + Clone + Any,
            TArrays: super::pipeline::env::PCOTrait + Any,
        {
            for pi in datum.iter_mut() {
                let _layout = unsafe {
                    renderer_slice
                        .gpu_ref()
                        .unwrap()
                        .logical_p
                        .as_ref()
                        .unwrap()
                        .create_pipeline_layout(pi.layout_create_info_ref(), Option::None)
                        .unwrap_or(
                            renderer_slice
                                .gpu_ref()
                                .unwrap()
                                .logical_p
                                .as_ref()
                                .unwrap()
                                .create_pipeline_layout(
                                    &cfg::env::PSO::DEFAULT_LAYOUT,
                                    Option::None,
                                )
                                .unwrap(),
                        )
                };
                pi.set_layout(_layout);
            }
        }

        fn _callback_create_shader_moudule(
            datum: &mut Datum<ShaderModuleD>,
            renderer_slice: &mut RendererE,
        ) {
            for si in datum.vec_mut().iter_mut() {
                si.entity = Some(unsafe {
                    renderer_slice
                        .gpu_ref()
                        .unwrap()
                        .logical_p
                        .as_ref()
                        .unwrap()
                        .create_shader_module(&si.info, Option::None)
                        .unwrap()
                })
            }
        }

        fn _callback_create_graphic_pipeline_pass(
            datum: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
        ) {
            for pi in datum.iter_mut() {
                let _pass = unsafe {
                    renderer_slice
                        .gpu_ref()
                        .unwrap()
                        .logical_p
                        .as_ref()
                        .unwrap()
                        .create_render_pass(
                            pi.pso_ref().renderpass_info_ref().unwrap(),
                            Option::None,
                        )
                        .unwrap_or(
                            renderer_slice
                                .gpu_ref()
                                .unwrap()
                                .logical_p
                                .as_ref()
                                .unwrap()
                                .create_render_pass(
                                    &&crate::renderer::cfg::env::PSO::DEFAULT_RENDER_PASS,
                                    Option::None,
                                )
                                .unwrap(),
                        )
                };
                pi.pco_mut().set_render_pass(_pass);
            }
        }

        fn _callback_create_custom_surface_img_view(
            datum_surfimg: &mut Datum<DeviceBuffer<SurfaceIMGBuffer>>,
            api_bind: &mut VkAshAPID,
            vk_img_format: &mut vk::ImageCreateInfo,
            vk_render_img2surface_config: &mut vk::ImageViewCreateInfo,
            renderer_slice: &mut RendererE,
            usage: &usize,
        ) {
            vk_img_format.extent = vk::Extent3D::builder()
                .width(
                    renderer_slice
                        .swapchain_create_info
                        .as_mut()
                        .unwrap()
                        .image_extent
                        .width,
                )
                .height(
                    renderer_slice
                        .swapchain_create_info
                        .as_mut()
                        .unwrap()
                        .image_extent
                        .height,
                )
                .depth(1)
                .build();
            vk_img_format.queue_family_index_count = renderer_slice
                .swapchain_create_info
                .as_mut()
                .unwrap()
                .queue_family_index_count;
            vk_img_format.p_queue_family_indices = renderer_slice
                .swapchain_create_info
                .as_mut()
                .unwrap()
                .p_queue_family_indices;
            //  create IMG
            let img = unsafe {
                renderer_slice
                    .gpu_ref()
                    .unwrap()
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .create_image(vk_img_format, Option::None)
                    .unwrap()
            };

            vk_render_img2surface_config.image = img;
            vk_render_img2surface_config.format = vk_img_format.format;

            let alloc_size = unsafe {
                renderer_slice
                    .gpu_ref()
                    .unwrap()
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .get_image_memory_requirements(img)
                    .size
            };

            let _surfbuf = SurfaceIMGBuffer::default().build_img(img);
            let mut _buf: DeviceBuffer<SurfaceIMGBuffer> =
                DeviceBuffer::<SurfaceIMGBuffer>::default()
                    .build_buffer(_surfbuf)
                    .build_usage(*usage);
            let _pro = renderer_slice
                .gpu_ref()
                .as_ref()
                .unwrap()
                .mem_info
                .as_ref()
                .unwrap();

            renderer_slice._alloc_device_mem_surfimg(
                alloc_size,
                DeviceBufferUsage::get_vk_usage(*usage),
                _pro,
                &mut _buf,
            );

            _buf.buffer_mut().unwrap().set_view(unsafe {
                renderer_slice
                    .gpu_ref()
                    .unwrap()
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .create_image_view(&vk_render_img2surface_config, Option::None)
                    .unwrap()
            });

            datum_surfimg.alloc_data(_buf, Option::None).end();
        }

        fn _callback_create_color_surface_img_view(
            datum: &mut Datum<DeviceBuffer<SurfaceIMGBuffer>>,
            renderer_slice: &mut RendererE,

            _priority_level: &i32,
        ) {
            let images = unsafe {
                renderer_slice
                    .swapchain_loader
                    .as_mut()
                    .unwrap()
                    .get_swapchain_images(*renderer_slice.swapchain.as_mut().unwrap())
                    .unwrap()
            };

            for imgi in images {
                let mut _buf: DeviceBuffer<SurfaceIMGBuffer> = Default::default();

                let info = vk::ImageViewCreateInfo {
                    s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                    p_next: null(),
                    flags: vk::ImageViewCreateFlags::default(),
                    image: imgi,
                    view_type: match renderer_slice.renderer_attachment.is_cube_surface {
                        true => vk::ImageViewType::TYPE_3D,
                        false => vk::ImageViewType::TYPE_2D,
                    },
                    format: renderer_slice
                        .swapchain_create_info
                        .as_mut()
                        .unwrap()
                        .image_format,
                    components: vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    },
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                };
                // todo!(); // leak device buffer alloc

                let view = unsafe {
                    renderer_slice
                        .gpu_ref()
                        .unwrap()
                        .logical_p
                        .as_ref()
                        .unwrap()
                        .create_image_view(&info, Option::None)
                        .unwrap()
                };

                let alloc_size = unsafe {
                    renderer_slice
                        .gpu_ref()
                        .unwrap()
                        .logical_p
                        .as_ref()
                        .unwrap()
                        .get_image_memory_requirements(imgi)
                        .size
                };

                let _surfbuf = SurfaceIMGBuffer::default().build_img(imgi).build_view(view);
                let mut _buf: DeviceBuffer<SurfaceIMGBuffer> =
                    DeviceBuffer::<SurfaceIMGBuffer>::default()
                        .build_buffer(_surfbuf)
                        .build_usage(DeviceBufferUsage::SURF_IMG_UNIFORM_COLOR);

                // renderer_slice.alloc_device_mem(
                //     alloc_size,
                //     DeviceBufferUsage::get_vk_usage(DeviceBufferUsage::SURF_IMG_UNIFORM_COLOR),
                //     *api_bind.gpu_mem_properties_current_ref().unwrap(),
                //     &mut _buf,
                // );

                unsafe {
                    renderer_slice
                        .gpu_ref()
                        .unwrap()
                        .logical_p
                        .as_ref()
                        .unwrap()
                        .bind_image_memory(imgi, *_buf.device_mem_ref().unwrap(), 0)
                        .expect("bind custom image_memory fail")
                };

                datum.alloc_data(_buf, Option::None).end();
            }
        }

        /// also see crate::renderer::cmd::RenderCmdE
        ///
        fn _callback_create_cmd_buffer(
            datum: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            bind_cmdslicce: &mut RenderCmdE,
            renderer_slice: &RendererE,
        ) {
            let _pool = bind_cmdslicce.pool_ref().unwrap();
            let _priority_level = bind_cmdslicce.cmd_attachment.priority_level;
            let _logical_device = renderer_slice
                .gpu_ref()
                .unwrap()
                .logical_p
                .as_ref()
                .unwrap();

            let cmd_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: null(),
                command_pool: *_pool,
                level: vk::CommandBufferLevel::from_raw(_priority_level),
                command_buffer_count: 1,
            };

            let command_buffers = unsafe {
                _logical_device
                    .allocate_command_buffers(&cmd_buffer_allocate_info)
                    .unwrap()
            };

            let _device_buf = DeviceBuffer::<Vec<vk::CommandBuffer>>::default()
                .build_buffer(command_buffers)
                .build_usage(DeviceBufferUsage::CMD_BUFFER);

            bind_cmdslicce.update_cbib(_device_buf.buffer_ref().unwrap().len());

            bind_cmdslicce.cmd_attachment.index_current_cmd_buffers = datum
                .alloc_data(_device_buf, Some(bind_cmdslicce.id))
                .index();

            // for ci in command_buffers.into_iter() {
            //     let device_buf = DeviceBuffer::<vk::CommandBuffer>::default()
            //         .build_buffer(ci)
            //         .build_usage(DeviceBufferUsage::CMD_BUFFER);
            //     datum.alloc_data(device_buf, Some(bind_cmdslicce.id)).end();
            // }
        }

        pub fn _callback_update_vbo(
            datum: &mut Datum<DeviceBuffer<vk::Buffer>>,
            pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
            mesh: &MeshD,
            api_in: &VkAshAPID,
        ) {
            todo!();
        }

        /// # Abstract
        /// - Abandoned feature
        /// - Feature has include in create_vbo
        /// - Define as your device_buffer_usage custom or device property default setting.
        #[deprecated = "abandoned feature"]
        fn _callback_map_vertex_buffer(
            &mut self,
            datum_model: &mut Datum<DeviceBuffer<ModelE>>,
            datum_mesh: &mut Datum<MeshD>,
            datum_vkbuf: &mut Datum<DeviceBuffer<vk::Buffer>>,
        ) {
            todo!();
        }

        fn _callback_unmap_buffer_mem(&self) {
            todo!();
        }

        fn _callback_unmap_buffer(
            datum_buf: &mut Datum<DeviceBuffer<vk::Buffer>>,
            datum_model: &mut Datum<ModelD>,
            renderer_slice: &mut RendererE,
            api_bind: &VkAshAPID,
        ) {
            todo!();
        }

        fn _callback_create_vbo(
            datum_buf: &mut Datum<DeviceBuffer<vk::Buffer>>,
            datum_model: &mut Datum<ModelD>,
            pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
            usage_in: &usize,
            mesh_datum: &Datum<MeshD>,
            api_bind: &VkAshAPID,
            switch_share_mode: &bool,
        ) {
            for mmi in datum_model.vec_mut().iter_mut() {
                let mut _vbo: DeviceBuffer<vk::Buffer> =
                    DeviceBuffer::default().build_usage(*usage_in);

                let _mesh_index = mmi
                    .get_attechment_index(crate::model::mtid::MTID_DAT_MESH)
                    .unwrap();
                let _mesh: &MeshD = get!(mesh_datum.vec_ref(), _mesh_index);
                //分配顶点缓存
                renderer_slice.alloc_device_mem(
                    _mesh.buffer_mem_size(),
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                    renderer_slice
                        .gpu_ref()
                        .as_ref()
                        .unwrap()
                        .mem_info
                        .as_ref()
                        .unwrap(),
                    &mut _vbo,
                    *switch_share_mode,
                );

                let mut _mem_p: *mut c_void = null_mut();
                // 判断是否需要主存映射
                unsafe {
                    match usage_in & (0xff << 24) {
                        DeviceBufferUsage::MEM_TYPE_LOCAL_HOST => {}
                        DeviceBufferUsage::MEM_TYPE_RAM_PROTECED => {}
                        DeviceBufferUsage::MEM_TYPE_RAM_UNVISIBLE => {}
                        DeviceBufferUsage::MEM_TYPE_RAM_CACHED => {
                            _mem_p = renderer_slice
                                .gpu_ref()
                                .unwrap()
                                .logical_p
                                .as_ref()
                                .unwrap()
                                .map_memory(
                                    *_vbo.device_mem_ref().unwrap(),
                                    0,
                                    _mesh.buffer_mem_size(),
                                    vk::MemoryMapFlags::default(),
                                )
                                .unwrap();
                        }
                        DeviceBufferUsage::MEM_TYPE_RAM_VISIBLE => {
                            _mem_p = renderer_slice
                                .gpu_ref()
                                .unwrap()
                                .logical_p
                                .as_ref()
                                .unwrap()
                                .map_memory(
                                    *_vbo.device_mem_ref().unwrap(),
                                    0,
                                    _mesh.buffer_mem_size(),
                                    vk::MemoryMapFlags::default(),
                                )
                                .unwrap();
                        }
                        DeviceBufferUsage::MEM_TYPE_RAM_COHERENT => {
                            _mem_p = renderer_slice
                                .gpu_ref()
                                .unwrap()
                                .logical_p
                                .as_ref()
                                .unwrap()
                                .map_memory(
                                    *_vbo.device_mem_ref().unwrap(),
                                    0,
                                    _mesh.buffer_mem_size(),
                                    vk::MemoryMapFlags::default(),
                                )
                                .unwrap();
                        }
                        _ => {
                            crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_INFO
                                    | crate::log::code::CONDI_NOMAP_DEVICE_MEM_MODE
                                    | crate::log::code::FILE_RENDERER
                                    | crate::log::LogCodeD::new()
                                        .encode(
                                            line!() as u128,
                                            crate::log::LogPartFlag::LOGGER_PART_LINE
                                        )
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(
                                            mmi.id as u128,
                                            crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                        )
                                        .get_code()
                            );
                        }
                    }
                    let _vbo_index = datum_buf.alloc_data(_vbo, Option::None).index();
                    mmi.push_attechment(crate::model::mtid::MTID_DAT_VERTEX_BUF, _vbo_index);
                }
            }

            // for mi in mesh_datum.vec_ref().iter() {
            //     let mut _vbo: DeviceBuffer<vk::Buffer> =
            //         DeviceBuffer::default().build_usage(*usage_in);
            //     //分配顶点缓存
            //     renderer_slice.alloc_device_mem(
            //         mi.as_ref().unwrap().buffer_mem_size(),
            //         vk::BufferUsageFlags::VERTEX_BUFFER,
            //         *api_bind.gpu_mem_properties_current_ref().unwrap(),
            //         &mut _vbo,
            //     );

            // }
        }

        pub fn drop(mut self) {
            unsafe {
                // ash::extensions::khr::Surface::destroy_surface(
                //     self.vk_surface.as_ref().unwrap(),
                //     self.renderer_surface.unwrap(),
                //     Option::None,
                // )
            };
        }
    }

    impl Default for RendererAttachment {
        fn default() -> Self {
            return Self {
                surface_pixle_format: vk::Format::R8G8B8A8_UNORM,
                swap_level: 2,
                is_muti_queue_shared: false,
                is_clip: true,
                is_vertical_blank: false,
                is_cube_surface: false,
                is_performance_first:
                    crate::renderer::cfg::env::RENDERER::DEFAULT_IS_PERFORMANCE_FIRST,

                cube_surface_width: 1,

                index_surface_task: usize::MAX,
                // index_cmd_task: 1,
                index_shader_mod_task: usize::MAX,
                index_pipeline_task: usize::MAX,
                index_fbo_task: usize::MAX,
                index_vbo_task: usize::MAX,
                index_cmd_buffer_task: usize::MAX,
                index_cmdsync_task: usize::MAX,
                index_gpu: 0,
                index_swapimg_bind_queue: 0,
                count_cmd: 0,
            };
        }
    }

    impl Default for RendererTask {
        fn default() -> Self {
            RendererTask::None
        }
    }
}

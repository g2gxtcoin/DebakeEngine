pub mod cfg;
pub mod cmd;
pub mod pass;
pub mod pipeline;

//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {

    use crate::{
        dbg_dev,
        ext_api::graphic::env::{name, VkAshAPID},
        get,
        manager::{
            datum::env::Datum,
            execute::{
                env::TaskQueue,
                template::call_back_template::{self},
            },
        },
        model::mesh::{self, env::MeshD},
        shader::env::ShaderModuleD,
        DatumM,
    };
    use ash::{
        extensions::khr::{Surface, Swapchain, Win32Surface},
        vk::{
            self, FramebufferAttachmentsCreateInfoKHR, GraphicsPipelineCreateInfo,
            ShaderModuleCreateInfo, StructureType, SurfaceCapabilitiesKHR, SwapchainKHR,
        },
    };

    use std::{
        any::Any,
        fmt::Debug,
        ptr::{null, null_mut},
    };
    use winapi::{
        ctypes::c_void,
        shared::{minwindef::HINSTANCE, windef::HWND},
    };

    use crate::manager::execute::sub::task_interface::{self};

    use super::{
        cfg::{
            self,
            env::{
                IMG2VIEW::{DEFAULT_COLOR, DEFAULT_DEPTH},
                IMG_FORMAT::{DEFAULT_COLOR_IMG, DEFAULT_DEPTH_IMG},
            },
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
        PushCmdBuffer(
            usize, //dat cmd index
            call_back_template::Callback3MR1R<
                Datum<vk::CommandBuffer>,
                vk::CommandPool,
                ash::Device,
                i32,
            >,
            i32, // param: priority
        ),
        CreateSurfaceImg(
            usize, // dat surface index
            call_back_template::Callback5MR1R<
                Datum<SurfaceIMG>,
                RendererE,
                VkAshAPID,
                vk::ImageCreateInfo,
                vk::ImageViewCreateInfo,
                SurfaceIMGUsage,
            >,
            SurfaceIMGUsage, //param usage
        ),

        CreateSurfaceColorImg(
            usize, //index
            call_back_template::Callback2MR1R<Datum<SurfaceIMG>, RendererE, i32>,
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
                Datum<vk::Framebuffer>,
                Datum<SurfaceIMG>,
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
            call_back_template::Callback3MR2R<
                Datum<vk::Buffer>,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
                Datum<MeshD>,
                VkAshAPID
            >,
        ),

        UpdateVBO(
            usize, // mesh index
            call_back_template::Callback3MR2R<
                Datum<vk::Buffer>,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RendererE,
                MeshD,
                VkAshAPID,
            >,
        ),

        RecordCMD(
            usize, // cmd index
            call_back_template::Callback2MR2R<
                Datum<vk::CommandBuffer>,
                RendererE,
                usize,
                vk::CommandBufferUsageFlags,
            >,
        ),
    }
    #[derive(Clone, Copy)]
    pub enum SurfaceIMGUsage {
        None,
        Uniform(PreTypeSurfaceIMG),
        Storage(PreTypeSurfaceIMG),
    }

    #[cfg(feature = "log_mode_dev")]
    impl Debug for SurfaceIMGUsage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::None => write!(f, "None"),
                Self::Uniform(arg0) => f.debug_tuple("Uniform").field(arg0).finish(),
                Self::Storage(arg0) => f.debug_tuple("Storage").field(arg0).finish(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PreTypeSurfaceIMG {
        None,
        DefaultDepth,
        DefaultColor,
    }

    #[derive(Debug)]
    pub struct SurfaceIMG {
        pub usage: SurfaceIMGUsage,
        pub img: vk::Image,
        pub view: vk::ImageView,
    }

    //
    #[repr(C, align(8))]
    pub struct RendererAttachment {
        pub is_muti_queue_shared: bool,
        pub is_vertical_blank: bool,
        pub is_clip: bool,
        pub is_cube_surface: bool,

        pub surface_pixle_format: vk::Format,
        pub swap_level: u32,
        pub cube_surface_width: u32,
        pub device_queue_count: u32,

        pub index_surface_task: u64,
        pub index_cmd_task: u64,
        pub index_shader_mod_task: u64,
        pub index_pipeline_task: u64,
        pub index_fbo_task: u64,
        pub index_vbo_task: u64,
    }

    pub struct RendererE {
        pub id: u64,
        pub renderer_attachment: RendererAttachment,
        pub wnd_handle: HWND,
        pub mod_handle: HINSTANCE,

        pub device: Option<ash::Device>,
        pub device_mem: Option<Vec<vk::DeviceMemory>>,

        pub gpu_properties: Option<ash::vk::PhysicalDeviceProperties>,

        pub swapchain: Option<vk::SwapchainKHR>,
        pub swapchain_create_info: Option<vk::SwapchainCreateInfoKHR>,
        pub swapchain_loader: Option<Swapchain>,

        pub surface_create_info: Option<vk::Win32SurfaceCreateInfoKHR>, //hwnd:c_void,
        pub renderer_surface: Option<vk::SurfaceKHR>,

        pub cmd_buffer_pool: Option<vk::CommandPool>,
    }

    impl SurfaceIMGUsage {
        pub fn get_img_buffer_flag(&self) -> vk::BufferUsageFlags {
            match self {
                SurfaceIMGUsage::None => vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER,
                SurfaceIMGUsage::Uniform(_) => vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER,
                SurfaceIMGUsage::Storage(_) => vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER,
            }
        }
    }

    impl PreTypeSurfaceIMG {
        pub fn get_img_info(&self) -> Result<vk::ImageCreateInfo, ()> {
            match self {
                PreTypeSurfaceIMG::None => Err(crate::log::sorry(
                    crate::log::code::TYPE_EXE_ERROR
                        | crate::log::code::CONDI_OPTION_NONE
                        | crate::log::code::FILE_RENDERER
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(
                                u16::MAX as u128,
                                crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                            )
                            .get_code(),
                )),
                PreTypeSurfaceIMG::DefaultDepth => return Ok(DEFAULT_DEPTH_IMG.clone()),
                PreTypeSurfaceIMG::DefaultColor => return Ok(DEFAULT_COLOR_IMG.clone()),
            }
        }
        pub fn get_img_view_info(&self) -> Result<vk::ImageViewCreateInfo, ()> {
            match self {
                PreTypeSurfaceIMG::None => Err(crate::log::sorry(
                    crate::log::code::TYPE_EXE_ERROR
                        | crate::log::code::CONDI_OPTION_NONE
                        | crate::log::code::FILE_RENDERER
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(
                                u16::MAX as u128,
                                crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                            )
                            .get_code(),
                )),
                PreTypeSurfaceIMG::DefaultDepth => return Ok(DEFAULT_DEPTH.clone()),
                PreTypeSurfaceIMG::DefaultColor => return Ok(DEFAULT_COLOR.clone()),
            }
        }
        pub fn register_new_pre_surface_img_type() {}
    }

    impl Default for RendererE {
        fn default() -> Self {
            return Self {
                id: 0,
                device: Option::None,
                swapchain: Option::None,
                surface_create_info: Option::None,
                wnd_handle: null_mut(),
                mod_handle: null_mut(),
                renderer_surface: Option::None,
                swapchain_loader: Option::None,

                cmd_buffer_pool: Option::None,
                renderer_attachment: RendererAttachment::default(),
                swapchain_create_info: Option::None,

                device_mem: Option::Some(Vec::with_capacity(8)),

                gpu_properties: Option::None,
            };
        }
    }

    impl RendererE {
        // pub fn record_cmd(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
        //     tin.exe_data_mut(self.renderer_attachment.index_pipeline_task)
        //         .unwrap()
        //         .push_task(RendererTask::RecordCMD(
        //             Self::_callback_record_cmd,
        //         ))
        // }

        fn _callback_record_cmd(
            datum_cmd: &mut Datum<vk::CommandBuffer>,
            renderer_slice: &mut RendererE,
            index_in: &usize,
            flag_in: &vk::CommandBufferUsageFlags,
        ) {
            let _cmd = get!(datum_cmd.vec_mut(), *index_in).unwrap();
            let _begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: null(),
                flags: *flag_in,
                p_inheritance_info: null(),
            };

            match unsafe {
                renderer_slice
                    .device
                    .as_mut()
                    .unwrap()
                    .begin_command_buffer(_cmd, &_begin_info)
            } {
                Ok(_) => {}
                Err(_) => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_UNDEFINE_CONDI
                            | crate::log::code::FILE_RENDERER
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    renderer_slice.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    );
                }
            }

            unsafe {
                renderer_slice
                    .device
                    .as_mut()
                    .unwrap()
                    .end_command_buffer(_cmd)
            };
        }

        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }
        pub fn recreate_swapcahin(&mut self) {}

        pub fn draw(&mut self) {
            let framebuffer = vk::FramebufferCreateInfo {
                s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next: null(),
                flags: vk::FramebufferCreateFlags::default(),
                render_pass: todo!(),
                attachment_count: todo!(),
                p_attachments: todo!(),
                width: todo!(),
                height: todo!(),
                layers: todo!(),
            };
        }

        pub fn build() -> Self {
            return Default::default();
        }

        pub fn build_gpu_properties(mut self, api_in: &mut VkAshAPID) -> Self {
            self.gpu_properties = Option::Some(api_in.gpu_properties_clone().unwrap_or_default());
            return self;
        }

        pub fn build_specify_handle(mut self, hwnd_in: HWND, mod_handle_in: HINSTANCE) -> Self {
            self.wnd_handle = hwnd_in;
            self.mod_handle = mod_handle_in;
            return self;
        }

        pub fn build_specify_api_base2create_surface(mut self, api_in: &mut VkAshAPID) -> Self {
            self.device = Option::Some(api_in.ash_device_clone().unwrap());
            self.renderer_attachment.device_queue_count =
                api_in.gpu_suitable_queue_count_currrent();

            self._create_surface(api_in);
            return self;
        }

        pub fn build_device_suitable_surface(self, api_in: &mut VkAshAPID) -> Self {
            for index in 0..api_in.gueue_info_ref().as_ref().unwrap().iter().len() {
                unsafe {
                    match Win32Surface::new(
                        api_in.ash_entry_ref().unwrap(),
                        api_in.ash_instance_ref().unwrap(),
                    )
                    .get_physical_device_win32_presentation_support(
                        *api_in.gpu_instance_ref().unwrap(),
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
            self.swapchain_loader = Some(Swapchain::new(
                api_in.ash_instance_ref().unwrap(),
                self.device.as_ref().unwrap(),
            ));
            let surface_capabilities = unsafe {
                api_in
                    .surface_loader_ref()
                    .as_ref()
                    .unwrap()
                    .get_physical_device_surface_capabilities(
                        *api_in.gpu_instance_ref().unwrap(),
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
                        *api_in.gpu_instance_ref().unwrap(),
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
                        *api_in.gpu_instance_ref().unwrap(),
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

            self.swapchain_create_info = Option::Some(vk::SwapchainCreateInfoKHR {
                s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
                p_next: null(),
                flags: vk::SwapchainCreateFlagsKHR::default(),
                surface: *self.renderer_surface.as_ref().unwrap(),
                min_image_count: surface_capabilities.min_image_count,
                //+ self.renderer_attachment.swap_level, /* - 1 */
                image_format: surface_formate.format,
                image_color_space: surface_formate.color_space,
                image_extent: surface_capabilities.current_extent, // will be chage when custom redecide wnd height&wide
                image_array_layers: match self.renderer_attachment.is_cube_surface {
                    true => 1 + self.renderer_attachment.cube_surface_width,
                    false => 1,
                },
                image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
                image_sharing_mode: match self.renderer_attachment.is_muti_queue_shared {
                    true => match self.renderer_attachment.device_queue_count {
                        0 => vk::SharingMode::EXCLUSIVE,
                        1 => vk::SharingMode::EXCLUSIVE,
                        _ => vk::SharingMode::CONCURRENT,
                    },
                    false => vk::SharingMode::EXCLUSIVE,
                },
                queue_family_index_count: self.renderer_attachment.device_queue_count,
                p_queue_family_indices: &self.renderer_attachment.device_queue_count,
                pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
                composite_alpha: vk::CompositeAlphaFlagsKHR::INHERIT,
                present_mode: *surface_present_mode,
                clipped: self.renderer_attachment.is_clip as u32,
                old_swapchain: vk::SwapchainKHR::default(),
            });

            //vk::KhrPortabilitySubsetFn::name()
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

        pub fn build_cmd_pool(mut self) -> Self {
            let pool_create_info = vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
                p_next: null(),
                flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER, // 部分低性能设备 请使用protected模式
                queue_family_index: 0,
            };

            self.cmd_buffer_pool = unsafe {
                Option::Some(
                    self.device
                        .as_ref()
                        .unwrap()
                        .create_command_pool(&pool_create_info, None)
                        .unwrap(),
                )
            };

            return self;
        }

        pub fn build_set_pipeline_dynamic_state_auto(mut self) -> Self {
            //self.pipeline_dynamic_state.as_mut().unwrap().push(vk::DynamicState::);
            return self;
        }

        pub fn create_pipeline_layout(
            &mut self,
            pipe_type: RenderPipelineType,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            match pipe_type {
                RenderPipelineType::None => todo!(),
                RenderPipelineType::Graphic => tin
                    .get_data_mut(self.renderer_attachment.index_pipeline_task)
                    .unwrap()
                    .push_task(RendererTask::CreateGraphicPipelineLayout(
                        Self::_callback_create_pipeline_layout,
                    )),
                RenderPipelineType::Compute => todo!(),
                RenderPipelineType::RayTracing => todo!(),
            }
        }

        pub fn create_graphic_pipeline_pass(&self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            tin.get_data_mut(self.renderer_attachment.index_pipeline_task)
                .unwrap()
                .push_task(RendererTask::CreateGraphicPipelinePass(
                    Self::_callback_create_graphic_pipeline_pass,
                ))
        }

        pub fn create_graphic_pipeline(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            tin.get_data_mut(self.renderer_attachment.index_pipeline_task)
                .unwrap()
                .push_task(RendererTask::CreateGraphicPipeline(
                    Self::_callback_create_graphic_pipeline,
                ))
        }

        // push new cmd buffer associate device in specify cmd buffer datum
        pub fn create_cmd_buffer(
            &mut self,
            cmd_index: usize,
            priority_level: i32,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            //judge inherit task queue offset

            tin.get_data_mut(self.renderer_attachment.index_cmd_task)
                .unwrap()
                .push_task(RendererTask::PushCmdBuffer(
                    cmd_index,
                    Self::_callback_create_cmd_buffer,
                    priority_level,
                ))
        }

        pub fn create_color_surface_img_view(
            &mut self,
            surf_img_index: usize,
            priority: i32,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            tin.get_data_mut(self.renderer_attachment.index_surface_task)
                .unwrap()
                .push_task(RendererTask::CreateSurfaceColorImg(
                    surf_img_index,
                    Self::_callback_create_color_surface_img_view,
                    priority,
                ));
        }

        pub fn create_custom_surface_img_view(
            &mut self,
            surf_img_index: usize,
            usage: SurfaceIMGUsage,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            tin.get_data_mut(self.renderer_attachment.index_surface_task)
                .unwrap()
                .push_task(RendererTask::CreateSurfaceImg(
                    surf_img_index,
                    Self::_callback_create_custom_surface_img_view,
                    usage,
                ));
        }

        pub fn create_shader_module(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            tin.get_data_mut(self.renderer_attachment.index_shader_mod_task)
                .unwrap()
                .push_task(RendererTask::CreateShaderMoudule(
                    Self::_callback_create_shader_moudule,
                ));
        }

        // 前置条件：
        // 渲染管线
        // 交换链
        //
        pub fn create_vbo(&mut self, api_in:&mut VkAshAPID,tin: &mut Datum<TaskQueue<RendererTask>>) {
            tin.get_data_mut(self.renderer_attachment.index_vbo_task)
                .unwrap()
                .push_task(RendererTask::CreateVBO(Self::_callback_create_vbo));
        }

        pub fn update_vbo(&mut self, mesh_index: usize, tin: &mut Datum<TaskQueue<RendererTask>>) {
            tin.get_data_mut(self.renderer_attachment.index_vbo_task)
                .unwrap()
                .push_task(RendererTask::UpdateVBO(
                    mesh_index,
                    Self::_callback_update_vbo,
                ));
        }

        pub fn _callback_update_vbo(
            datum: &mut Datum<vk::Buffer>,
            pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
            mesh: &MeshD,
            api_in: &VkAshAPID,
        ) {
            todo!();
        }

        fn _callback_create_vbo(
            datum: &mut Datum<vk::Buffer>,
            pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
            mesh_datum: &Datum<MeshD>,
            api_bind: &VkAshAPID,
        ) {
            let mut buffer_info = vk::BufferCreateInfo {
                s_type: vk::StructureType::BUFFER_CREATE_INFO,
                p_next: std::ptr::null(),
                flags: vk::BufferCreateFlags::default(),
                size: 0,
                usage: vk::BufferUsageFlags::VERTEX_BUFFER,
                sharing_mode: match renderer_slice.renderer_attachment.is_muti_queue_shared {
                    true => vk::SharingMode::CONCURRENT,
                    false => vk::SharingMode::EXCLUSIVE,
                },
                queue_family_index_count: renderer_slice.renderer_attachment.device_queue_count,
                p_queue_family_indices: std::ptr::null(),
            };

            for mi in mesh_datum.vec_ref().iter() {
                // 更新 vertex buffer createinfo 中bufer size
                buffer_info.size = mi.as_ref().unwrap().buffer_mem_size();

                //分配顶点缓存
                renderer_slice.alloc_device_mem(
                    buffer_info.size,
                    vk::BufferUsageFlags::VERTEX_BUFFER,
                    *api_bind.gpu_mem_properties_current_ref().unwrap()
                );
            }

            // let buffer = unsafe {
            //     renderer_slice
            //         .device
            //         .as_mut()
            //         .unwrap()
            //         .create_buffer(&buffer_info, None)
            //         .expect("Failed to create buffer")
            // };

            // datum.alloc_data(buffer, None);
        }

        // 前置条件：
        // 渲染管线
        // 交换链
        //
        pub fn create_fbo(&self, tin: &mut Datum<TaskQueue<RendererTask>>) {
            tin.get_data_mut(self.renderer_attachment.index_fbo_task)
                .unwrap()
                .push_task(RendererTask::CreateFBO(Self::_callback_create_fbo))
        }

        // create vk buffer
        pub fn alloc_device_mem(
            &mut self,
            mem_size: u64,
            mem_usage: vk::BufferUsageFlags,
            current_mem_prop: vk::PhysicalDeviceMemoryProperties,
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
                queue_family_index_count: self.renderer_attachment.device_queue_count,
                p_queue_family_indices: null(),
            };

            let buffer = unsafe {
                self.device
                    .as_mut()
                    .unwrap()
                    .create_buffer(&buffer_info, Option::None)
                    .unwrap()
            };

            // request mem from device by vk instance
            let mem_req = unsafe {
                self.device
                    .as_mut()
                    .unwrap()
                    .get_buffer_memory_requirements(buffer)
            };

            // request mem from device by vk instance
            let memory_type_index = current_mem_prop
                .memory_types
                .iter()
                .enumerate()
                .find(|&(index, data)| {
                    data.property_flags == vk::MemoryPropertyFlags::DEVICE_LOCAL
                        && ((1 << index) & mem_req.memory_type_bits) != 0
                })
                .unwrap()
                .0;

            let info = vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
                p_next: null(),
                allocation_size: mem_size,
                memory_type_index: memory_type_index as u32,
            };
            let mem = unsafe {
                self.device
                    .as_mut()
                    .unwrap()
                    .allocate_memory(&info, Option::None)
                    .expect("memory allocate fail")
            };
            self.device_mem.as_mut().unwrap().push(mem);
        }

        pub fn link_task_queue(&mut self, tqin: &mut Datum<TaskQueue<RendererTask>>) {
            tqin.alloc_data(
                TaskQueue::default(),
                Some(self.renderer_attachment.index_surface_task),
            );
            tqin.alloc_data(
                TaskQueue::default(),
                Some(self.renderer_attachment.index_cmd_task),
            );
            tqin.alloc_data(
                TaskQueue::default(),
                Some(self.renderer_attachment.index_shader_mod_task),
            );
            tqin.alloc_data(
                TaskQueue::default(),
                Some(self.renderer_attachment.index_pipeline_task),
            );
            tqin.alloc_data(
                TaskQueue::default(),
                Some(self.renderer_attachment.index_fbo_task),
            );
            tqin.alloc_data(
                TaskQueue::default(),
                Some(self.renderer_attachment.index_vbo_task),
            );
        }

        // 执行与 vertex buffer 相关所有指令
        // 1.创建
        // 2.销毁
        // 3.挂起
        pub fn exe_vbo(
            &mut self,
            datum_vkbuf: &mut Datum<vk::Buffer>,
            datum_mesh: &mut Datum<MeshD>,
            pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            api_bind: &VkAshAPID,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = tin
                .get_data_mut(self.renderer_attachment.index_vbo_task)
                .unwrap();
            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateVBO(call) => {
                        call(datum_vkbuf, pipeline, self, datum_mesh,api_bind);
                    }
                    RendererTask::UpdateVBO(mesh_index, call) => {
                        call(
                            datum_vkbuf,
                            pipeline,
                            self,
                            get!(datum_mesh.vec_ref(), *mesh_index).as_ref().unwrap(),
                            api_bind
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
            datum_fbo: &mut Datum<vk::Framebuffer>,
            datum_surfimg: &mut Datum<SurfaceIMG>,
            datum_pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = tin
                .get_data_mut(self.renderer_attachment.index_fbo_task)
                .unwrap();
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
            let mut _task = tin
                .get_data_mut(self.renderer_attachment.index_shader_mod_task)
                .unwrap();
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
            datum: &mut Datum<SurfaceIMG>,
            api_in: &mut VkAshAPID,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = tin
                .get_data_mut(self.renderer_attachment.index_surface_task)
                .unwrap();
            _tasks.begin_execute();
            // let self_rc=Rc::new(RefCell::new(self)) ;
            // let self_rc_i=Rc::clone(&self_rc);
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RendererTask::CreateSurfaceColorImg(index, call, priority) => {
                        call(datum, self, &priority)
                    }
                    RendererTask::CreateSurfaceImg(index, call, usage) => match usage {
                        SurfaceIMGUsage::None => todo!(),
                        SurfaceIMGUsage::Uniform(pre) => call(
                            datum,
                            self,
                            api_in,
                            &mut pre.get_img_info().unwrap(),
                            &mut pre.get_img_view_info().unwrap(),
                            &usage,
                        ),
                        SurfaceIMGUsage::Storage(pre) => call(
                            datum,
                            self,
                            api_in,
                            &mut pre.get_img_info().unwrap(),
                            &mut pre.get_img_view_info().unwrap(),
                            &usage,
                        ),
                    },
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn exe_cmdbuffer(
            &mut self,
            data: &mut Datum<vk::CommandBuffer>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = tin
                .get_data_mut(self.renderer_attachment.index_cmd_task)
                .unwrap();
            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_ref(ti) {
                    RendererTask::None => {}
                    RendererTask::PushCmdBuffer(index, call, priority_level) => call(
                        data,
                        self.cmd_buffer_pool.as_mut().unwrap(),
                        self.device.as_mut().unwrap(),
                        &priority_level,
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
            let mut _task = tin
                .get_data_mut(self.renderer_attachment.index_pipeline_task)
                .unwrap();
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
            datum_fbo: &mut Datum<vk::Framebuffer>,
            datum_surfimg: &mut Datum<SurfaceIMG>,
            datum_pipeline: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
        ) {
            let mut _attachments = Vec::new();
            for si in datum_surfimg.vec_ref() {
                // dbg!(&((si.as_ref().unwrap())));
                _attachments.push(si.as_ref().unwrap().view);
            }

            if renderer_slice.swapchain_create_info.is_some() {
                for rpi in datum_pipeline.vec_mut().iter() {
                    let _count = rpi
                        .as_ref()
                        .unwrap()
                        .pso_ref()
                        .renderpass_info_ref()
                        .unwrap()
                        .attachment_count;

                    let _p_attachment = rpi
                        .as_ref()
                        .unwrap()
                        .pso_ref()
                        .renderpass_info_ref()
                        .unwrap()
                        .p_attachments;

                    let _info = vk::FramebufferCreateInfo {
                        s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                        p_next: null(),
                        flags: match _count == 0 {
                            true => vk::FramebufferCreateFlags::IMAGELESS,
                            false => Default::default(),
                        },
                        render_pass: rpi.as_ref().unwrap().pco_ref().pass_ref().unwrap().clone(), //?
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

                    datum_fbo.alloc_data(
                        unsafe {
                            renderer_slice
                                .device
                                .as_mut()
                                .unwrap()
                                .create_framebuffer(&_info, Option::None)
                                .unwrap()
                        },
                        Option::None,
                    );
                }
            }
        }

        fn _callback_create_graphic_pipeline<TStates, TArrays>(
            datum: &mut Datum<RenderPipelineD<TStates, TArrays>>,
            renderer_slice: &mut RendererE,
        ) where
            TStates: super::pipeline::env::PSOTrait + Clone + Any,
            TArrays: super::pipeline::env::PCOTrait + Any,
        {
            let mut _pipeline_info_slice = Vec::<GraphicsPipelineCreateInfo>::new();
            for ri in datum.vec_mut() {
                match ri.as_mut().unwrap().pipeline_info() {
                    pipeline::env::PipelineCreateInfoResult::None => {}
                    pipeline::env::PipelineCreateInfoResult::Graphic(val) => {
                        _pipeline_info_slice.push(val)
                    }
                    pipeline::env::PipelineCreateInfoResult::Compute(_) => {}
                    pipeline::env::PipelineCreateInfoResult::RayTracing(_) => {}
                }
            }

            let _pipelines = unsafe {
                renderer_slice
                    .device
                    .as_mut()
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
                .filter(|x| match x.as_ref().unwrap().render_pipeline_type() {
                    RenderPipelineType::Graphic => true,
                    _ => false,
                })
                .enumerate()
            {
                ri.1.as_mut().unwrap().set_pipeline(_pipelines[ri.0]);
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
                        .device
                        .as_mut()
                        .unwrap()
                        .create_pipeline_layout(
                            pi.as_mut().unwrap().layout_create_info_ref(),
                            Option::None,
                        )
                        .unwrap_or(
                            renderer_slice
                                .device
                                .as_mut()
                                .unwrap()
                                .create_pipeline_layout(
                                    &cfg::env::PSO::DEFAULT_LAYOUT,
                                    Option::None,
                                )
                                .unwrap(),
                        )
                };
                pi.as_mut().unwrap().set_layout(_layout);
            }
        }

        fn _callback_create_shader_moudule(
            datum: &mut Datum<ShaderModuleD>,
            renderer_slice: &mut RendererE,
        ) {
            for si in datum.vec_mut().iter_mut() {
                match si {
                    Some(val) => unsafe {
                        val.entity = Some(
                            renderer_slice
                                .device
                                .as_mut()
                                .unwrap()
                                .create_shader_module(&val.info, Option::None)
                                .unwrap(),
                        )
                    },
                    None => {}
                }
            }
        }

        fn _callback_create_graphic_pipeline_pass(
            datum: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            renderer_slice: &mut RendererE,
        ) {
            for pi in datum.iter_mut() {
                let _pass = unsafe {
                    renderer_slice
                        .device
                        .as_mut()
                        .unwrap()
                        .create_render_pass(
                            pi.as_ref()
                                .unwrap()
                                .pso_ref()
                                .renderpass_info_ref()
                                .unwrap(),
                            Option::None,
                        )
                        .unwrap_or(
                            renderer_slice
                                .device
                                .as_mut()
                                .unwrap()
                                .create_render_pass(
                                    &&crate::renderer::cfg::env::PSO::DEFAULT_RENDER_PASS,
                                    Option::None,
                                )
                                .unwrap(),
                        )
                };
                pi.as_mut().unwrap().pco_mut().set_render_pass(_pass);
            }
        }

        //fn _callback_create_color_surface_img_view

        fn _callback_create_custom_surface_img_view(
            datum: &mut Datum<SurfaceIMG>,
            renderer_slice: &mut RendererE,
            api_bind: &mut VkAshAPID,

            vk_img_format: &mut vk::ImageCreateInfo,
            vk_render_img2surface_config: &mut vk::ImageViewCreateInfo,
            usage: &SurfaceIMGUsage,
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
            let img = unsafe {
                renderer_slice
                    .device
                    .as_mut()
                    .unwrap()
                    .create_image(vk_img_format, Option::None)
                    .unwrap()
            };
            vk_render_img2surface_config.image = img;
            vk_render_img2surface_config.format = vk_img_format.format;

            let alloc_size = unsafe {
                renderer_slice
                    .device
                    .as_mut()
                    .unwrap()
                    .get_image_memory_requirements(img)
                    .size
            };

            renderer_slice.alloc_device_mem(
                alloc_size,
                usage.get_img_buffer_flag(),
                *api_bind.gpu_mem_properties_current_ref().unwrap(),
            );

            unsafe {
                renderer_slice
                    .device
                    .as_mut()
                    .unwrap()
                    .bind_image_memory(
                        img,
                        *renderer_slice.device_mem.as_mut().unwrap().last().unwrap(),
                        0,
                    )
                    .expect("msg")
            };

            let view = unsafe {
                renderer_slice
                    .device
                    .as_mut()
                    .unwrap()
                    .create_image_view(&vk_render_img2surface_config, Option::None)
                    .unwrap()
            };
            datum
                .alloc_data(
                    SurfaceIMG {
                        img,
                        view,
                        usage: usage.clone(),
                    },
                    Option::None,
                )
                .end();
        }

        fn _callback_create_color_surface_img_view(
            datum: &mut Datum<SurfaceIMG>,
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
                let view = unsafe {
                    renderer_slice
                        .device
                        .as_mut()
                        .unwrap()
                        .create_image_view(&info, Option::None)
                        .unwrap()
                };

                let _surface_img = SurfaceIMG {
                    img: imgi,
                    view,
                    usage: SurfaceIMGUsage::Uniform(PreTypeSurfaceIMG::DefaultColor),
                };

                datum.alloc_data(_surface_img, Option::None).end();
            }
        }

        fn _callback_create_cmd_buffer(
            datum: &mut Datum<vk::CommandBuffer>,
            pool: &mut vk::CommandPool,
            logical_device: &mut ash::Device,

            priority_level: &i32,
        ) {
            let cmd_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
                p_next: null(),
                command_pool: *pool,
                level: vk::CommandBufferLevel::from_raw(*priority_level),
                command_buffer_count: 1,
            };

            let command_buffers = unsafe {
                logical_device
                    .allocate_command_buffers(&cmd_buffer_allocate_info)
                    .unwrap()
            };
            datum.alloc_data(command_buffers[0], Option::None);
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
                cube_surface_width: 1,
                device_queue_count: 0,

                index_surface_task: 0,
                index_cmd_task: 1,
                index_shader_mod_task: 2,
                index_pipeline_task: 3,
                index_fbo_task: 4,
                index_vbo_task: 5,
            };
        }
    }

    impl Default for RendererTask {
        fn default() -> Self {
            RendererTask::None
        }
    }
}

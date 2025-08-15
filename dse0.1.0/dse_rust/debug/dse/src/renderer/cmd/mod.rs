//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::{ptr::null, u64};

    use ash::vk::{self, CommandBuffer, Rect2D, RenderPass};

    use crate::{
        ________________dev_stop________________, dev_dbg,
        ext_api::graphic::env::VkAshAPID,
        get, get_mut,
        log::send2logger,
        manager::{
            datum::{self, env::Datum},
            execute::{env::TaskQueue, sub::task_interface, template::call_back_template},
        },
        model::{self, env::ModelD, rectangle::env::Rect},
        renderer::{
            self,
            buffer::env::{DeviceBuffer, DeviceBufferTrait},
            cfg::env::RENDERER,
            env::{RendererE, RendererTask},
            pipeline::env::{GraphicPipeLinePCO, GraphicPipeLinePSO, PCOTrait, RenderPipelineD},
        },
    };

    #[derive(Default)]
    pub enum RenderCmdTask {
        #[default]
        None,

        RenderPass(),
    }

    pub struct RenderCmdAttachment {
        usage_flag: u64,

        index_cmd_task: u64,
        index_graphic_pipeline_task: u64,
        idnex_cmd_buffer: u64,
    }

    pub struct CmdUsage {}

    #[allow(unused)]
    impl CmdUsage {
        pub const DEFAULT: u64 = 0x0;
        //
        pub const MANUAL_MODE: u64 = 0b0000; // default
        pub const AUTO_MODE: u64 = 0b0001;
        pub const PEOTECTED_MODE: u64 = 0b0010;
        pub const SUBCMD_MODE: u64 = 0b0100;
        //
        pub const SURPPORT_GRAPHIC: u64 = 0b0000_0001 << 4;
        pub const SURPPORT_COMPUTE: u64 = 0b0000_0010 << 4;
        pub const SURPPORT_TRANSFER: u64 = 0b0000_0100 << 4;
        pub const SURPORT_SPARSE_MEM: u64 = 0b0000_1000 << 4;
        pub const SURPPORT_VIDEO: u64 = 0b0001_0000 << 4;

        pub fn from_renderer(rin: &renderer::env::RendererE) -> u64 {
            let mut _r = CmdUsage::DEFAULT;
            // judge if performance first
            if !rin.renderer_attachment.is_performance_first {
                _r = _r | CmdUsage::AUTO_MODE;
            } else {
                _r = _r | CmdUsage::MANUAL_MODE;
            }
            // judge if slectable queue families type

            return _r;
        }

        pub fn to_queue_flag(usage_in: u64) -> u32 {
            let mut _r = 0;
            if usage_in & CmdUsage::PEOTECTED_MODE != 0 {
                _r = _r | vk::QueueFlags::PROTECTED.as_raw();
            }
            if usage_in & CmdUsage::SURPPORT_GRAPHIC != 0 {
                _r = _r | vk::QueueFlags::GRAPHICS.as_raw();
            }
            if usage_in & CmdUsage::SURPPORT_COMPUTE != 0 {
                _r = _r | vk::QueueFlags::COMPUTE.as_raw();
            }
            if usage_in & CmdUsage::SURPPORT_TRANSFER != 0 {
                _r = _r | vk::QueueFlags::TRANSFER.as_raw();
            }
            if usage_in & CmdUsage::SURPORT_SPARSE_MEM != 0 {
                _r = _r | vk::QueueFlags::SPARSE_BINDING.as_raw();
            }
            if usage_in & CmdUsage::SURPPORT_VIDEO != 0 {
                _r = _r | vk::QueueFlags::VIDEO_DECODE_KHR.as_raw();
            }
            return _r;
        }

        pub fn to_pool_create_info(usage_in: u64, api_in: &VkAshAPID) -> vk::CommandPoolCreateInfo {
            let mut _r = vk::CommandPoolCreateInfo::default();
            //
            match usage_in & CmdUsage::AUTO_MODE {
                0 => _r.flags = vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                _ => _r.flags = vk::CommandPoolCreateFlags::TRANSIENT,
            }
            //
            _r.queue_family_index = 0;
            let _queue_vec = unsafe {
                api_in
                    .ash_instance_ref()
                    .unwrap()
                    .get_physical_device_queue_family_properties(
                        *api_in.vk_gpu_device_ref().unwrap(),
                    )
            };

            let mut _queue_index = 0;
            if CmdUsage::to_queue_flag(usage_in) != 0 {
                for qfi in _queue_vec.iter().enumerate() {
                    if qfi
                        .1
                        .queue_flags
                        .contains(vk::QueueFlags::from_raw(CmdUsage::to_queue_flag(usage_in)))
                        && qfi.1.queue_flags.as_raw() > _queue_index
                    {
                        _queue_index = qfi.0 as u32;
                        _r.queue_family_index = _queue_index;
                    }
                }
            }

            _r.queue_family_index = _queue_index;

            return _r;
        }
    }

    #[repr(align(4))]
    pub struct RenderCmdE {
        pub id: u64,
        pub is_lock: bool,
        device_p: Option<ash::Device>,
        render_area: Option<vk::Extent2D>,
        cmd_attachment: RenderCmdAttachment,
        cmd_buffer_pool: Option<vk::CommandPool>,

        buf_create_info: Option<vk::CommandBufferAllocateInfo>,
        buf_begin_info: Option<vk::CommandBufferBeginInfo>,
        buf_inhernit_info: Option<vk::CommandBufferInheritanceInfo>,
        pool_create_info: Option<vk::CommandPoolCreateInfo>,
    }

    // The mind you have is all disaster you deserved.
    // The way you choose is all tear you suffered.
    // The belief you believe will not live you in future.
    //
    impl RenderCmdE {
        pub const CMD_TYPE_RENDER: u64 = 0;
        pub const CMD_TYPE_COMPUTE: u64 = 1;

        pub fn bind_task_queue(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            tin.alloc_data(
                TaskQueue::default(),
                Some(self.cmd_attachment.index_cmd_task),
            );
            tin.alloc_data(
                TaskQueue::default(),
                Some(self.cmd_attachment.index_graphic_pipeline_task),
            );
            tin.alloc_data(
                TaskQueue::default(),
                Some(self.cmd_attachment.idnex_cmd_buffer),
            );
        }

        #[deprecated = "Abandoned Feature"]
        pub fn build() -> Self {
            let mut _r: Self = Default::default();
            return _r;
        }

        pub fn build_active_inhernit_cmd_info(mut self) -> Self {
            todo!();
            self.buf_inhernit_info = Default::default();
            self.buf_begin_info = Some(vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: null(),
                flags: self.buf_begin_info.unwrap().flags
                    | vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE,
                p_inheritance_info: self.buf_inhernit_info.as_ref().unwrap(),
            });
            return self;
        }

        /// use it before bind renderer
        pub fn build_cmd_usage(mut self, uin: u64) -> Self {
            self.cmd_attachment.usage_flag = self.cmd_attachment.usage_flag | uin;
            return self;
        }

        pub fn build_buf_record_mode(mut self, uin: vk::CommandBufferUsageFlags) -> Self {
            self.buf_begin_info.unwrap().flags = uin;
            return self;
        }

        pub fn build_bind_renderer(mut self, ref_rin: &RendererE) -> Self {
            self.render_area = Some(ref_rin.swapchain_create_info.unwrap().image_extent.clone());
            return self;
        }

        pub fn build_api_device(mut self, api_in: &VkAshAPID) -> Self {
            self.device_p = Some(api_in.ash_device_clone().unwrap());

            self.pool_create_info = Some(CmdUsage::to_pool_create_info(
                self.cmd_attachment.usage_flag,
                api_in,
            ));

            unsafe {
                self.cmd_buffer_pool = Some(
                    self.device_p
                        .as_ref()
                        .unwrap()
                        .create_command_pool(&self.pool_create_info.unwrap(), None)
                        .unwrap(),
                );
            }

            return self;
        }

        pub fn bind_specify_model_sync(&mut self, datum_model: Datum<ModelD>) {
            todo!()
        }

        pub fn bind_specify_vertex_sync(
            &mut self,
            datum_cmd: &mut Datum<vk::CommandBuffer>,
            datum_mesh: &mut Datum<model::mesh::env::MeshD>,
            index_cmd: usize,
            index_mesh: usize,
        ) {
            let _cmd = get_mut!(datum_cmd.vec_mut(), index_cmd).as_mut().unwrap();
            let _mesh = get_mut!(datum_mesh.vec_mut(), index_mesh).as_mut().unwrap();

            unsafe {
                self.device_p.as_mut().unwrap().cmd_bind_vertex_buffers(
                    *_cmd,
                    todo!(),
                    todo!(),
                    todo!(),
                )
            }

            todo!();
        }

        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }

        pub fn set_render_rect(&mut self, height: u64, width: u64) {
            self.render_area = Some(vk::Extent2D {
                width: u32::try_from(width).unwrap(),
                height: u32::try_from(height).unwrap(),
            });
        }

        pub fn update_binding_renderer(&mut self, ref_rin: &RendererE) {
            self.render_area = Some(ref_rin.swapchain_create_info.unwrap().image_extent.clone());
        }

        pub fn bind_pipe() {}

        pub fn record_cmd_buf(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            // tin.get_data_mut(self.cmd_attachment.index_cmd_task)
            //     .unwrap()
            //     .push_task(RendererTask::RecordCMD(
            //         Self::_callback_record_cmd,
            //     ));

            todo!();
        }
        // fn _callback_record_cmd

        pub fn exe_graphic_rander_pipeline(
            &mut self,
            data: &mut Datum<
                DeviceBuffer<
                    crate::renderer::pipeline::env::RenderPipelineD<
                        GraphicPipeLinePSO,
                        GraphicPipeLinePCO,
                    >,
                >,
            >,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            todo!();
        }

        pub fn exe_cmd_buffer(
            &mut self,
            data: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
            tin: &mut Datum<TaskQueue<RendererTask>>,
        ) {
            let mut _tasks = tin
                .get_data_mut(self.cmd_attachment.index_cmd_task)
                .unwrap();

            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_ref(ti) {
                    RendererTask::None => {}
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        fn _calllback_render_pass(cmd_slice: &mut RenderCmdE) {}

        pub fn pool_ref(&self) -> Option<&vk::CommandPool> {
            return self.cmd_buffer_pool.as_ref();
        }

        pub fn begin_cmd_sync(
            &mut self,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
            index: usize,
        ) {
            let _cmd = get!(datum_cmd.vec_ref(), index)
                .as_ref()
                .unwrap()
                .buffer_ref();
            let begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: null(),
                flags: Default::default(),
                p_inheritance_info: null(),
            };

            unsafe {
                self.device_p
                    .as_mut()
                    .unwrap()
                    .begin_command_buffer(*_cmd.unwrap(), &begin_info)
                    .unwrap()
            };
        }

        pub fn end_cmd_sync(
            &mut self,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
            index: usize,
        ) {
            let _cmd = get!(datum_cmd.vec_ref(), index)
                .as_ref()
                .unwrap()
                .buffer_ref();
            unsafe {
                self.device_p
                    .as_mut()
                    .unwrap()
                    .end_command_buffer(*_cmd.unwrap())
                    .unwrap()
            };
        }

        pub fn sync_destroy_cmd(
            &mut self,
            index: usize,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            todo!();
        }

        pub fn sync_end_render_pass(
            &mut self,
            cmd_index: usize,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            let _cmd: &CommandBuffer = get!(datum_cmd.vec_ref(), cmd_index)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();
            unsafe {
                self.device_p
                    .as_mut()
                    .unwrap()
                    .cmd_end_render_pass(_cmd.clone())
            }
        }

        // push reder
        pub fn sync_begin_render_pass(
            &mut self,
            cmd_index: usize,
            rp_index: usize,
            fb_index: usize,
            datum_renderpass: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
            datum_fbo: &Datum<DeviceBuffer<vk::Framebuffer>>,
        ) {
            let _cmd: &CommandBuffer = get!(datum_cmd.vec_ref(), cmd_index)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();
            let _pipe = get!(datum_renderpass.vec_ref(), rp_index)
                .as_ref()
                .unwrap()
                .pipeline_ref();
            let _rpass: RenderPass = get!(datum_renderpass.vec_ref(), rp_index)
                .as_ref()
                .unwrap()
                .pco_ref()
                .pass_ref()
                .unwrap()
                .clone();

            let _fb: &vk::Framebuffer = get!(datum_fbo.vec_ref(), fb_index)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();

            let _info: vk::RenderPassBeginInfo = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: null(),
                render_pass: _rpass,
                framebuffer: *_fb,
                render_area: Rect2D {
                    offset: Default::default(),
                    extent: self.render_area.unwrap(),
                },
                clear_value_count: RENDERER::DEFAULT_ERROR_COLOR.len() as u32,
                p_clear_values: RENDERER::DEFAULT_ERROR_COLOR.as_ptr(),
            };

            unsafe {
                self.device_p.as_mut().unwrap().cmd_begin_render_pass(
                    _cmd.clone(),
                    &_info,
                    vk::SubpassContents::INLINE,
                )
            }
        }

        // #[deprecated = "test feature;abandoned feature"]
        pub fn sync_bind_pipe(
            &mut self,
            index_cmd: usize,
            index_pipe: usize,
            datum_cmd: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
        ) {
            let _cmd = get_mut!(datum_cmd.vec_mut(), index_cmd)
                .as_mut()
                .unwrap()
                .buffer_mut();
            let _pipe = get_mut!(datum_pipe.vec_mut(), index_pipe).as_mut().unwrap();

            unsafe {
                self.device_p.as_mut().unwrap().cmd_bind_pipeline(
                    *_cmd.unwrap(),
                    match _pipe.render_pipeline_type() {
                        crate::renderer::pipeline::env::RenderPipelineType::None => {
                            vk::PipelineBindPoint::GRAPHICS
                        }
                        crate::renderer::pipeline::env::RenderPipelineType::Graphic => {
                            vk::PipelineBindPoint::GRAPHICS
                        }
                        crate::renderer::pipeline::env::RenderPipelineType::Compute => {
                            vk::PipelineBindPoint::COMPUTE
                        }
                        crate::renderer::pipeline::env::RenderPipelineType::RayTracing => {
                            vk::PipelineBindPoint::RAY_TRACING_KHR
                        }
                    },
                    *_pipe.pipeline_mut(),
                )
            };
        }

        pub fn sync_bind_vertex(
            &mut self,
            index_cmd: usize,
            index_mesh: usize,
            index_model: usize,
            datum_cmdbuf: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
            datum_model: &mut Datum<model::env::ModelD>,
            datum_mesh: &mut Datum<model::mesh::env::MeshD>,
        ) {
            let _cmd = get!(datum_cmdbuf.vec_ref(), index_cmd)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();
            let _mesh = get!(datum_mesh.vec_ref(), index_mesh).as_ref().unwrap();
            unsafe {
                // self.device_p
                //     .as_mut()
                //     .unwrap()
                //     .cmd_bind_vertex_buffers(*_cmd, 0);

                todo!();
            }
        }

        pub fn sync_draw(
            &mut self,
            index_cmd: usize,
            command_buffers: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            let _cmdbuf: &CommandBuffer = get!(command_buffers.vec_ref(), index_cmd)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();
            unsafe {
                self.device_p
                    .as_mut()
                    .unwrap()
                    .cmd_draw(*_cmdbuf, 3, 1, 0, 0)
            };
        }

        pub fn sync_destroy_cmd_pool(&mut self) {
            unsafe {
                self.device_p
                    .as_mut()
                    .unwrap()
                    .destroy_command_pool(self.cmd_buffer_pool.unwrap(), None)
            };
        }

        pub fn sync_render_pass(&mut self) {}
    }

    impl Default for RenderCmdAttachment {
        fn default() -> Self {
            Self {
                index_cmd_task: 0,
                index_graphic_pipeline_task: 1,
                idnex_cmd_buffer: 2,
                usage_flag: Default::default(),
            }
        }
    }

    impl Default for RenderCmdE {
        fn default() -> Self {
            Self {
                id: Default::default(),
                device_p: Default::default(),
                render_area: Some(crate::workarea::MUL_PHYSICAL_SCALING_MAT(
                    crate::renderer::cfg::env::RECT::DEFAULT_RECT2D.extent,
                )),
                cmd_attachment: Default::default(),
                cmd_buffer_pool: Default::default(),
                buf_create_info: Default::default(),
                pool_create_info: Default::default(),
                buf_begin_info: Option::Some(Default::default()),
                buf_inhernit_info: Option::None,
                is_lock: false,
            }
        }
    }
}

pub mod sync;

//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::{ptr::null, u64, usize};

    use ash::vk::{self, CommandBuffer, Rect2D, RenderPass};
    use gltf::json::camera::Type;
    use toml::de;

    use crate::{
        ________________dev_stop________________, cast_mut, cast_ref, dev_dbg,
        ext_api::graphic::env::VkAshAPID,
        get, get_mut,
        log::send2logger,
        manager::{
            datum::{self, env::Datum},
            execute::{
                env::TaskQueue,
                sub::task_interface,
                template::{
                    self,
                    call_back_template::{
                        self, Callback0MR0R, Callback0MR1R, Callback1MR1R, Callback1MR2R,
                        Callback2MR1R,
                    },
                },
            },
        },
        model::{self, env::ModelD, mesh::env::MeshD, mtid, rectangle::env::Rect},
        renderer::{
            self,
            buffer::env::{DeviceBuffer, DeviceBufferTrait},
            cfg::env::RENDERER,
            env::{RendererE, RendererTask},
            pipeline::env::{
                GraphicPipeLinePCO, GraphicPipeLinePSO, PCOTrait, PSOTrait, RenderPipelineD,
            },
        },
    };

    #[derive(Default)]
    #[derive(Debug)]
    pub enum RenderCmdTask {
        #[default]
        None,
        BeginCmd(
            call_back_template::Callback1MR1R<RenderCmdE, Datum<DeviceBuffer<vk::CommandBuffer>>>,
        ),
        BindRenderPipe(
            call_back_template::Callback3MR0R<
                RenderCmdE,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                Datum<DeviceBuffer<vk::CommandBuffer>>,
            >,
        ),
        BeginRenderPass(
            usize, // index of fbo
            call_back_template::Callback1MR4R<
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RenderCmdE,
                Datum<DeviceBuffer<vk::CommandBuffer>>,
                Datum<DeviceBuffer<vk::Framebuffer>>,
                usize,
            >,
        ),
    }

    #[allow(unused)]
    impl CmdUsage {
        pub const DEFAULT: u64 = 0x0;
        //
        pub const MANUAL_MODE: u64 = 0b0000; // default
        pub const AUTO_MODE: u64 = 0b0001;
        pub const PEOTECTED_MODE: u64 = 0b0010;
        pub const SUBCMD_MODE: u64 = 0b0100;
        //
        pub const PIPE_GRAPHIC: u64 = 0b0000_0001 << 4;
        pub const PIPE_COMPUTE: u64 = 0b0000_0010 << 4;
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
            if usage_in & CmdUsage::PIPE_GRAPHIC != 0 {
                _r = _r | vk::QueueFlags::GRAPHICS.as_raw();
            }
            if usage_in & CmdUsage::PIPE_COMPUTE != 0 {
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

    pub struct RenderCmdAttachment {
        pub usage_flag: u64,

        pub index_cmd_buffer_task: usize,
        pub index_graphic_pipeline_task: usize,
        pub idnex_sync_task: usize,
        pub index_pipeline_task: usize,

        pub id_bind_exe_renderer: u64,
        pub index_current_pipeline: usize,
        pub index_current_cmd_buffer: usize,
    }

    pub struct CmdUsage {}

    pub struct RenderCmdQueueComponet {}

    #[repr(align(4))]
    pub struct RenderCmdE {
        pub id: u64,
        pub is_lock: bool,
        cmd_attachment: RenderCmdAttachment,
        // device_p: Option<ash::Device>,
        // device_p: Option<*const ash::Device>,
        pub device_p: Option<usize>,
        render_area: Option<vk::Extent2D>,
        cmd_buffer_pool: Option<vk::CommandPool>,

        cmd_semph: Option<Vec<vk::Semaphore>>,
        min_swapchainsurf_num: u32, // define the nunme of semafore

        buf_create_info: Option<vk::CommandBufferAllocateInfo>,
        buf_begin_info: Option<vk::CommandBufferBeginInfo>,
        buf_inhernit_info: Option<vk::CommandBufferInheritanceInfo>,
        buf_fence_info: Option<vk::FenceCreateInfo>,
        pool_create_info: Option<vk::CommandPoolCreateInfo>,
    }

    // The mind you have is all disaster you deserved.
    // The way you choose is all tear you suffered.
    // The belief you believe will not live you in future.
    //
    impl RenderCmdE {
        pub const CMD_TYPE_RENDER: u64 = 0;
        pub const CMD_TYPE_COMPUTE: u64 = 1;

        pub fn pool_ref(&self) -> Option<&vk::CommandPool> {
            return self.cmd_buffer_pool.as_ref();
        }

        pub fn build_submit_info(mut self) -> Self {
            let _info: vk::SubmitInfo = vk::SubmitInfo {
                s_type: vk::StructureType::SUBMIT_INFO,
                p_next: null(),
                wait_semaphore_count: match self.cmd_semph.as_ref() {
                    Some(val) => val.len() as u32,
                    None => 0,
                },
                p_wait_semaphores: match self.cmd_semph {
                    Some(val) => val.as_ptr(),
                    None => null(),
                },
                p_wait_dst_stage_mask: todo!(),
                command_buffer_count: todo!(),
                p_command_buffers: todo!(),
                signal_semaphore_count: todo!(),
                p_signal_semaphores: todo!(),
            };
            todo!();
            return self;
        }

        pub fn build() -> Self {
            let mut _r: Self = Default::default();
            _r._init_fence_info(); // default: signaled
            return _r;
        }

        fn _init_fence_info(&mut self) {
            self.buf_fence_info = Some(vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: null(),
                flags: vk::FenceCreateFlags::SIGNALED,
            });
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

        /// default: main command buffer
        pub fn build_bind_cmd_buf(mut self, uin: usize) -> Self {
            self.cmd_attachment.index_current_cmd_buffer = uin;
            return self;
        }

        pub fn build_buf_record_mode(mut self, uin: vk::CommandBufferUsageFlags) -> Self {
            self.buf_begin_info.as_mut().unwrap().flags = uin;
            return self;
        }

        pub fn build_bind_renderer(mut self, ref_rin: &RendererE) -> Self {
            self.render_area = Some(ref_rin.swapchain_create_info.unwrap().image_extent.clone());
            self.min_swapchainsurf_num = ref_rin.swapchain_create_info.unwrap().min_image_count;
            self.cmd_attachment.id_bind_exe_renderer = ref_rin.id;
            return self;
        }

        pub fn build_pipeline_index(mut self, iin: usize) -> Self {
            self.cmd_attachment.index_current_pipeline = iin;
            return self;
        }

        pub fn build_api_device(mut self, api_in: &VkAshAPID) -> Self {
            self.device_p = Some(api_in.ash_device_ref().unwrap() as *const ash::Device as usize);

            self.pool_create_info = Some(CmdUsage::to_pool_create_info(
                self.cmd_attachment.usage_flag,
                api_in,
            ));

            unsafe {
                self.cmd_buffer_pool = Some(
                    cast_ref!(ash::Device, self.device_p.unwrap())
                        .create_command_pool(&self.pool_create_info.unwrap(), None)
                        .unwrap(),
                );
            }

            return self;
        }

        pub fn bind_task_queue(&mut self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            self.cmd_attachment.index_cmd_buffer_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.cmd_attachment.index_graphic_pipeline_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.cmd_attachment.idnex_sync_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
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

        pub fn set_cmd_buf_index(&mut self, uin: usize) {
            self.cmd_attachment.index_current_cmd_buffer = uin;
        }

        pub fn set_pipe_index(&mut self, index: usize) {
            self.cmd_attachment.index_current_pipeline = index as usize;
        }

        pub fn update_binding_renderer(&mut self, ref_rin: &RendererE) {
            self.render_area = Some(ref_rin.swapchain_create_info.unwrap().image_extent.clone());
        }

        pub fn record_cmd_buf(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            // tin.get_data_mut(self.cmd_attachment.index_cmd_task)
            //     .unwrap()
            //     .push_task(RendererTask::RecordCMD(
            //         Self::_callback_record_cmd,
            //     ));

            todo!();
        }

        pub fn exe_cmd_buffer(
            &mut self,
            data: &Datum<DeviceBuffer<vk::CommandBuffer>>,
            tin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(tin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task)
                .as_mut()
                .unwrap();

            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_ref(ti) {
                    RenderCmdTask::BeginCmd(call) => {
                        call(self, data);
                    }
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn exe_graphic_rander_pipeline(
            &mut self,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
            datum_fbo: &mut Datum<DeviceBuffer<vk::Framebuffer>>,
            tin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(
                tin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            )
            .as_mut()
            .unwrap();

           _tasks.begin_execute();
           dbg!(&_tasks.task_iter_mut().unwrap());
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RenderCmdTask::BeginRenderPass(index_fbo, call) => {
                        call(datum_pipe, self, datum_cmd, datum_fbo, index_fbo);
                    }
                    RenderCmdTask::BindRenderPipe(call) => {
                        call(self, datum_pipe, datum_cmd);
                    }

                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn tak_begin_cmd(&mut self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let mut _tasks = get_mut!(tqin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task)
                .as_mut()
                .unwrap();
            _tasks.push_task(RenderCmdTask::BeginCmd(Self::_callback_begin_cmd));

        }

        pub fn tak_create_semaphore(&mut self) -> Option<Vec<vk::Semaphore>> {
            todo!();
        }

        pub fn tak_bind_vertex(
            &mut self,
            datum_cmd: &mut Datum<vk::CommandBuffer>,
            datum_mesh: &mut Datum<model::mesh::env::MeshD>,
            index_cmd: usize,
            index_mesh: usize,
        ) {
            let _cmd = get_mut!(datum_cmd.vec_mut(), index_cmd).as_mut().unwrap();
            let _mesh = get_mut!(datum_mesh.vec_mut(), index_mesh).as_mut().unwrap();

            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap()).cmd_bind_vertex_buffers(
                    *_cmd,
                    todo!(),
                    todo!(),
                    todo!(),
                )
            }

            todo!();
        }

        pub fn tak_bind_render_pipe(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _tak = get_mut!(
                tin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            )
            .as_mut()
            .unwrap();

            _tak.push_task(RenderCmdTask::BindRenderPipe(
                Self::_callback_bind_render_pipe,
            ))
        }

        pub fn tak_begin_render_pass(
            &mut self,
            fb_index: usize,
            tqin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(
                tqin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            )
            .as_mut()
            .unwrap();
            _tasks.push_task(RenderCmdTask::BeginRenderPass(
                fb_index,
                Self::_callback_begin_render_pass,
            ));
        }

        // push reder
        fn _callback_begin_render_pass(
            datum_renderpipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            cmd_slicce: &RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
            datum_fbo: &Datum<DeviceBuffer<vk::Framebuffer>>,
            fb_index: &usize,
        ) {
            let _cmd: &CommandBuffer = get!(
                datum_cmd.vec_ref(),
                cmd_slicce.cmd_attachment.index_current_cmd_buffer
            )
            .as_ref()
            .unwrap()
            .buffer_ref()
            .unwrap();
            let _pipe = get!(
                datum_renderpipe.vec_ref(),
                cmd_slicce.cmd_attachment.index_current_pipeline
            )
            .as_ref()
            .unwrap()
            .pipeline_ref();
            let _rpass: &RenderPass = get!(
                datum_renderpipe.vec_ref(),
                cmd_slicce.cmd_attachment.index_current_pipeline
            )
            .as_ref()
            .unwrap()
            .pco_ref()
            .pass_ref()
            .unwrap();

            let _fb: &vk::Framebuffer = get!(datum_fbo.vec_ref(), *fb_index)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();

            let _info: vk::RenderPassBeginInfo = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: null(),
                render_pass: *_rpass,
                framebuffer: *_fb,
                render_area: Rect2D {
                    offset: Default::default(),
                    extent: cmd_slicce.render_area.unwrap(),
                },
                clear_value_count: RENDERER::DEFAULT_ERROR_COLOR.len() as u32,
                p_clear_values: RENDERER::DEFAULT_ERROR_COLOR.as_ptr(),
            };

            unsafe {
                cast_ref!(ash::Device, cmd_slicce.device_p.unwrap()).cmd_begin_render_pass(
                    _cmd.clone(),
                    &_info,
                    match cmd_slicce.cmd_attachment.usage_flag & CmdUsage::SUBCMD_MODE {
                        0 => vk::SubpassContents::INLINE,
                        _ => vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
                    },
                )
            }
        }

        fn _callback_begin_cmd(
            cmd_slice: &mut RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_current_cmd_buffer
            )
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
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap())
                    .begin_command_buffer(*_cmd.unwrap(), &begin_info)
                    .unwrap()
            };
        }

        fn _callback_bind_render_pipe(
            cmd_slice: &mut RenderCmdE,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            let _cmd = get_mut!(
                datum_cmd.vec_mut(),
                cmd_slice.cmd_attachment.index_current_cmd_buffer as usize
            )
            .as_mut()
            .unwrap()
            .buffer_mut();
            let _pipe = get_mut!(
                datum_pipe.vec_mut(),
                cmd_slice.cmd_attachment.index_current_pipeline
            )
            .as_mut()
            .unwrap();

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap()).cmd_bind_pipeline(
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

        ///
        /// # Abstract
        /// ## Example
        /// ## Parameter
        /**
         * switch_vertex_bind_index: None:{auto find suitable vad binding index}
         */
        pub fn bind_specify_vertex(
            &mut self,
            index_cmd: usize,
            index_pipe: usize,
            index_model: usize,
            switch_vertex_bind_index: Option<u32>,
            datum_cmdbuf: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
            dattum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_model: &mut Datum<model::env::ModelD>,
            datum_mesh: &mut Datum<model::mesh::env::MeshD>,
            datum_vbuf: &mut Datum<DeviceBuffer<vk::Buffer>>,
        ) {
            let _model = get!(datum_model.vec_ref(), index_model).as_ref().unwrap();
            let _pipe = get!(dattum_pipe.vec_ref(), index_pipe).as_ref().unwrap();

            let _cmd_buf = get!(datum_cmdbuf.vec_ref(), index_cmd)
                .as_ref()
                .unwrap()
                .buffer_ref()
                .unwrap();
            let _vertex_buf = get!(
                datum_vbuf.vec_ref(),
                _model
                    .get_attechment_index(mtid::MTID_DAT_VERTEX_BUF)
                    .unwrap()
            )
            .as_ref()
            .unwrap()
            .buffer_ref()
            .unwrap();
            let _mesh_index = _model.get_attechment_index(mtid::MTID_DAT_MESH).unwrap();
            let _mesh = get!(datum_mesh.vec_ref(), _mesh_index).as_ref().unwrap();
            let _binding_index: u32 = match switch_vertex_bind_index {
                Some(val) => val,
                None => Self::_find_suitable_vad_binding_index(_pipe, _mesh),
            };
            // todo!();
            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap()).cmd_bind_vertex_buffers(
                    *_cmd_buf,
                    _binding_index,
                    &[*_vertex_buf],
                    &[0],
                );
                // todo!();
            }
        }

        #[allow(unused)]
        pub fn begin_cmd(&mut self, datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>) {
            Self::_callback_begin_cmd(self, datum_cmd);
        }
        #[allow(unused)]
        pub fn begin_render_pass(
            &self,
            fb_index: usize,
            datum_render_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
            datum_fbo: &Datum<DeviceBuffer<vk::Framebuffer>>,
        ) {
            Self::_callback_begin_render_pass(
                datum_render_pipe,
                self,
                datum_cmd,
                datum_fbo,
                &fb_index,
            );
        }

        pub fn end_cmd(&mut self, datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                self.cmd_attachment.index_current_cmd_buffer
            )
            .as_ref()
            .unwrap()
            .buffer_ref();
            unsafe {
                let a = cast_ref!(ash::Device, self.device_p.unwrap())
                    .end_command_buffer(*_cmd.unwrap())
                    .unwrap();
                dbg!(a);
            };
        }

        pub fn destroy_cmd(
            &mut self,
            index: usize,
            datum_cmd: &Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            todo!();
        }

        pub fn end_render_pass(
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
                cast_ref!(ash::Device, self.device_p.unwrap()).cmd_end_render_pass(_cmd.clone())
            }
        }

        pub fn bind_render_pipe(
            &mut self,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
        ) {
            Self::_callback_bind_render_pipe(self, datum_pipe, datum_cmd);
        }

        ///
        /// # Abstract
        pub fn _find_suitable_vad_binding_index(
            pipe: &RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>,
            mesh: &MeshD,
        ) -> u32 {
            return 0;
            todo!();
        }

        pub fn draw(
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
                cast_ref!(ash::Device, self.device_p.unwrap()).cmd_draw(*_cmdbuf, 3, 1, 0, 0)
            };
        }

        pub fn submit(&mut self) {
            //unsafe { self.device_p.as_mut().unwrap().queue_submit(queue, submits, fence) };
        }

        pub fn create_semaphore(&mut self) {
            todo!();

            let _info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
                p_next: null(),
                flags: Default::default(),
            };

            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap())
                    .create_semaphore(&_info, Option::None)
                    .unwrap();
            }
        }

        pub fn create_fence(&mut self) {
            let _info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: null(),
                flags: Default::default(),
            };

            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap())
                    .create_fence(&_info, Option::None)
                    .unwrap();
            };

            todo!();
        }

        pub fn presnet(&mut self) {}

        pub fn drop(&mut self) {}

        // pub fn clear_frame(
        //     &mut self,
        //     index_cmd: usize,
        //     command_buffers: &mut Datum<DeviceBuffer<vk::CommandBuffer>>,
        // ) {
        //     let _cmdbuf: &CommandBuffer = get!(command_buffers.vec_ref(), index_cmd)
        //         .as_ref()
        //         .unwrap()
        //         .buffer_ref()
        //         .unwrap();
        //     unsafe {
        //         self.device_p
        //             .as_mut()
        //             .unwrap()
        //             .cmd_clear_color_image(*_cmdbuf, todo!(), todo!(), todo!(), todo!())
        //     };
        // }

        pub fn destroy_cmd_pool(&mut self) {
            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap())
                    .destroy_command_pool(self.cmd_buffer_pool.unwrap(), None)
            };
        }

        pub fn render_pass(&mut self) {}
    }

    impl Default for RenderCmdAttachment {
        fn default() -> Self {
            Self {
                index_cmd_buffer_task: usize::MAX,
                index_graphic_pipeline_task: usize::MAX,
                idnex_sync_task: usize::MAX,
                index_pipeline_task: usize::MAX,
                usage_flag: Default::default(),

                id_bind_exe_renderer: u64::MAX,

                index_current_pipeline: 0,
                index_current_cmd_buffer: 0,
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

                pool_create_info: Default::default(),
                is_lock: false,
                cmd_semph: Default::default(),
                buf_create_info: Default::default(),
                buf_begin_info: Option::Some(Default::default()),
                buf_inhernit_info: Option::None,
                buf_fence_info: Option::None,
                min_swapchainsurf_num: 2,
            }
        }
    }
}

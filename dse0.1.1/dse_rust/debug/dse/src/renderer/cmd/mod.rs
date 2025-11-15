pub mod sync;

//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::{ptr::null, u64, usize};

    use ash::vk::{self, CommandBuffer, Rect2D, RenderPass};

    use toml::de;

    use crate::{
        ________________dev_break________________, cast_mut, cast_ref, dev_dbg,
        ext_api::graphic::env::VkAshAPID,
        get, get_mut,
        hardware::gpu::env::DseGPU,
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
            cfg::{
                self,
                env::{COMMAND, RENDERER},
            },
            env::RendererE,
            pipeline::env::{
                GraphicPipeLinePCO, GraphicPipeLinePSO, PCOTrait, PSOTrait, RenderPipelineD,
            },
        },
        time::{self, env::TimerE},
    };

    #[derive(Default, Debug)]
    #[allow(unused)]
    pub enum RenderCmdTask {
        #[default]
        None,
        BeginCmd(
            call_back_template::Callback0MR2R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
        ResetCmd(
            call_back_template::Callback0MR2R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
        BindRenderPipe(
            call_back_template::Callback3MR0R<
                RenderCmdE,
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
        BeginRenderPass(
            // usize, // index of fbo
            call_back_template::Callback1MR3R<
                Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
                Datum<DeviceBuffer<vk::Framebuffer>>,
                // usize,
            >,
        ),
        BindModel(
            u32, // first_bind_index
            call_back_template::Callback0MR6R<
                RenderCmdE,
                Datum<ModelD>,
                u32,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
                Datum<model::mesh::env::MeshD>,
                Datum<DeviceBuffer<vk::Buffer>>,
            >,
        ),
        Draw(
            call_back_template::Callback0MR2R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
        DrawIndex(
            call_back_template::Callback0MR2R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
        EndRenderPass(
            call_back_template::Callback0MR2R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
        SubmitCmd(
            call_back_template::Callback0MR3R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
                usize,
            >,
            usize, // exe index
        ),
        EndCmd(
            // call_back_template::Callback0MR2R<RenderCmdE, Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>>,
        ),
        WaitFences(call_back_template::Callback0MR2R<RenderCmdE, TimerE>),
        InitSubmitInfo(
            call_back_template::Callback1MR1R<
                RenderCmdE,
                Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            >,
        ),
    }

    #[allow(unused)]
    impl CmdUsage {
        pub const DEFAULT: u64 = 0x0;
        //
        pub const MANUAL_MODE: u64 = 0b0000; // default | performance first
        pub const AUTO_MODE: u64 = 0b0001; //
        pub const PEOTECTED_MODE: u64 = 0b0010;
        pub const SUBCMD_MODE: u64 = 0b0100;
        //
        pub const PIPE_GRAPHIC: u64 = 0b0000_0001 << 4;
        pub const PIPE_COMPUTE: u64 = 0b0000_0010 << 4;
        //
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
        pub fn to_pool_create_info(usage_in: u64, gpu_in: &DseGPU) -> vk::CommandPoolCreateInfo {
            let mut _r = vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
                p_next: null(),
                flags: Default::default(),
                queue_family_index: 0,
            };
            //
            match usage_in & CmdUsage::AUTO_MODE {
                0 => _r.flags = vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                _ => _r.flags = vk::CommandPoolCreateFlags::TRANSIENT,
            }
            //

            let _queue_family_vec = gpu_in.queue_families.as_ref().unwrap();
            let mut _queue_family_index = 0;

            if CmdUsage::to_queue_flag(usage_in) != 0 {
                for qfi in _queue_family_vec.iter().enumerate() {
                    match usage_in & CmdUsage::AUTO_MODE {
                        0 => {
                            if qfi.1.queue_flags.contains(vk::QueueFlags::from_raw(
                                CmdUsage::to_queue_flag(usage_in),
                            )) && qfi.1.queue_flags.as_raw() < _queue_family_index
                            {
                                _queue_family_index = qfi.0 as u32;
                                _r.queue_family_index = _queue_family_index;
                            }
                        }
                        _ => {
                            if qfi.1.queue_flags.contains(vk::QueueFlags::from_raw(
                                CmdUsage::to_queue_flag(usage_in),
                            )) && qfi.1.queue_flags.as_raw() > _queue_family_index
                            {
                                _queue_family_index = qfi.0 as u32;
                                _r.queue_family_index = _queue_family_index;
                            }
                        }
                    }
                }
            }

            _r.queue_family_index = _queue_family_index;

            return _r;
        }
    }

    pub struct RenderCmdAttachment {
        pub priority_level: i32,
        pub usage_flag: u64,
        pub stage_flag: vk::PipelineStageFlags,

        //
        pub index_cmd_buffer_task: usize,
        pub index_graphic_pipeline_task: usize,
        pub idnex_sync_task: usize,
        pub index_pipeline_task: usize,
        pub index_model_task: usize,
        //
        pub id_bind_exe_renderer: u64,
        pub index_binding_pipeline: usize,
        pub index_binding_cmd_buffers: usize,
        // set by binding render
        pub index_gpu_queue: usize,
        pub index_gpu_queue_family: usize,

        pub count_active_swapimg: u32,
        pub index_current_surfimg_buf: u32,

        // RenderCmdBindingIndex
        // cbib : command buffer index binding
        pub switch_cmd_buffer_index_bind_auto: bool,
        pub cbib_vertex: u32,
        pub cbib_draw: u32,
        pub cbib_pipeline: u32,
        pub cbib_cmdbuf: u32,
        //
    }

    pub struct CmdUsage(u64);

    pub struct RenderCmdQueueComponet {}

    #[repr(align(4))]
    pub struct RenderCmdE {
        pub id: u64,
        pub is_lock: bool,
        pub cmd_attachment: RenderCmdAttachment,

        // device_p: Option<ash::Device>,
        // device_p: Option<*const ash::Device>,
        pub device_p: Option<usize>,

        pub frame_stride_ns: u64,
        render_area: Option<vk::Extent2D>,
        cmd_buffer_pool: Option<vk::CommandPool>,

        min_swapchainsurf_num: u32, // define the nunme of semafore

        buf_create_info: Option<vk::CommandBufferAllocateInfo>,
        buf_begin_info: Option<vk::CommandBufferBeginInfo>,
        buf_inhernit_info: Option<vk::CommandBufferInheritanceInfo>,
        buf_fence_info: Option<vk::FenceCreateInfo>,
        pool_create_info: Option<vk::CommandPoolCreateInfo>,

        submit_info: Option<vk::SubmitInfo>,

        // sync obj
        semaphore: Vec<ash::vk::Semaphore>,
        fence: Vec<ash::vk::Fence>,
        #[allow(unused)]
        event: Vec<ash::vk::Event>,
    }

    // The mind you have is all disaster you deserved.
    // The way you choose is all tear you suffered.
    // The belief you believe will not live you in future.
    //
    impl RenderCmdE {
        pub const CMD_TYPE_RENDER: u64 = 0;
        pub const CMD_TYPE_COMPUTE: u64 = 1;

        pub fn build_index_current_surfimg_buf(mut self, index: u32) -> Self {
            self.cmd_attachment.index_current_surfimg_buf = index;
            return self;
        }

        pub fn build_priority_level(mut self, lin: i32) -> Self {
            self.cmd_attachment.priority_level = lin;
            return self;
        }

        pub fn submit_info_ref(&self) -> Result<&vk::SubmitInfo, ()> {
            match self.submit_info {
                Some(ref val) => {
                    return Ok(val);
                }
                None => {
                    return Err(());
                }
            }
        }

        pub fn submit_info_mut(&mut self) -> Result<&mut vk::SubmitInfo, ()> {
            if self.submit_info.is_none() {
                return Err(());
            }
            return Ok(self.submit_info.as_mut().unwrap());
        }

        pub fn semaps_ref(&self) -> &Vec<vk::Semaphore> {
            return &self.semaphore;
        }

        pub fn semaps_mut(&mut self) -> &mut Vec<vk::Semaphore> {
            return &mut self.semaphore;
        }

        pub fn pool_ref(&self) -> Option<&vk::CommandPool> {
            return self.cmd_buffer_pool.as_ref();
        }

        pub fn build() -> Self {
            let mut _r: Self = Default::default();
            _r._init_fence_info();
            return _r;
        }

        pub fn build_bind_gpu_info(
            mut self,
            queue_family_index: usize,
            queue_index: usize,
        ) -> Self {
            self.cmd_attachment.index_gpu_queue_family = queue_family_index;
            self.cmd_attachment.index_gpu_queue = queue_index;
            return self;
        }

        pub fn build_cmd_usein_pipe_stage(mut self, sin: vk::PipelineStageFlags) -> Self {
            self.cmd_attachment.stage_flag = sin;
            return self;
        }

        fn _init_fence_info(&mut self) {
            self.buf_fence_info = Some(vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: null(),
                flags: vk::FenceCreateFlags::empty(),
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
            self.cmd_attachment.index_binding_cmd_buffers = uin;
            return self;
        }

        pub fn build_buf_record_mode(mut self, uin: vk::CommandBufferUsageFlags) -> Self {
            self.buf_begin_info.as_mut().unwrap().flags = uin;
            return self;
        }

        pub fn build_bind_renderer(mut self, render_slice: &RendererE) -> Self {
            self.render_area = Some(
                render_slice
                    .swapchain_create_info
                    .unwrap()
                    .image_extent
                    .clone(),
            );
            self.min_swapchainsurf_num =
                render_slice.swapchain_create_info.unwrap().min_image_count;
            self.cmd_attachment.id_bind_exe_renderer = render_slice.id;
            self.device_p = Some(render_slice.device_ref().unwrap() as *const ash::Device as usize);
            self.frame_stride_ns = render_slice.frame_stride_ns;
            self.pool_create_info = Some(CmdUsage::to_pool_create_info(
                self.cmd_attachment.usage_flag,
                render_slice.gpu_ref().unwrap(),
            ));

            // judge if index_gpu_queue is valid
            if self.cmd_attachment.index_gpu_queue_family
                >= render_slice
                    .gpu_ref()
                    .unwrap()
                    .queue_families
                    .as_ref()
                    .unwrap()
                    .len()
            {
                self.cmd_attachment.index_gpu_queue_family = 0;
            }

            if self.cmd_attachment.index_gpu_queue
                >= get!(
                    render_slice
                        .gpu_ref()
                        .unwrap()
                        .queue_create_info_vec
                        .as_ref()
                        .unwrap(),
                    self.cmd_attachment.index_gpu_queue_family
                )
                .queue_count as usize
            {
                self.cmd_attachment.index_gpu_queue = self.cmd_attachment.index_gpu_queue
                    % render_slice
                        .gpu_ref()
                        .unwrap()
                        .queue_vec
                        .as_ref()
                        .unwrap()
                        .len();
            }

            unsafe {
                self.cmd_buffer_pool = Some(
                    cast_ref!(ash::Device, self.device_p.unwrap())
                        .create_command_pool(&self.pool_create_info.unwrap(), None)
                        .unwrap(),
                );
            }

            return self;
        }

        pub fn build_pipeline_index(mut self, iin: usize) -> Self {
            self.cmd_attachment.index_binding_pipeline = iin;
            return self;
        }

        pub fn update_cmdbuf_slice(
            mut self,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) -> Self {
            todo!();
            return self;
        }

        pub fn bind_task_queue(&mut self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            self.cmd_attachment.index_cmd_buffer_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.cmd_attachment.index_graphic_pipeline_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.cmd_attachment.idnex_sync_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.cmd_attachment.index_model_task =
                tqin.alloc_data(TaskQueue::default(), Option::None).index();
            self.cmd_attachment.index_pipeline_task =
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
            self.cmd_attachment.index_binding_cmd_buffers = uin;
        }

        pub fn set_pipe_index(&mut self, index: usize) {
            self.cmd_attachment.index_binding_pipeline = index as usize;
        }

        pub fn exe_cmd_buffer(
            &mut self,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            tin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(tin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task);

            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_ref(ti) {
                    RenderCmdTask::BeginCmd(call) => {
                        call(self, datum_cmd);
                    }
                    RenderCmdTask::SubmitCmd(call, _index) => {
                        call(self, datum_cmd, _index);
                    }
                    RenderCmdTask::ResetCmd(call) => {
                        call(self, datum_cmd);
                    }
                    RenderCmdTask::InitSubmitInfo(call) => {
                        call(self, datum_cmd);
                    }
                    RenderCmdTask::EndCmd() => todo!(),
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn exe_graphic_rander_pipeline(
            &mut self,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            datum_fbo: &mut Datum<DeviceBuffer<vk::Framebuffer>>,
            tin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(
                tin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            );

            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RenderCmdTask::BeginRenderPass(call) => {
                        call(datum_pipe, self, datum_cmd, datum_fbo);
                    }
                    RenderCmdTask::BindRenderPipe(call) => {
                        call(self, datum_pipe, datum_cmd);
                    }
                    RenderCmdTask::EndRenderPass(call) => {
                        call(self, datum_cmd);
                    }
                    RenderCmdTask::Draw(call) => {
                        call(self, datum_cmd);
                    }
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn exe_model(
            &self,
            datum_model: &Datum<ModelD>,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            datum_vbuf: &Datum<DeviceBuffer<vk::Buffer>>,
            datum_mesh: &Datum<model::mesh::env::MeshD>,
            tin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(tin.vec_mut(), self.cmd_attachment.index_model_task);

            _tasks.begin_execute();
            for ti in _tasks.task_iter_mut().unwrap() {
                match task_interface::TaskTrait::task_mut(ti) {
                    RenderCmdTask::BindModel(first_bind_index, call) => {
                        call(
                            self,
                            datum_model,
                            first_bind_index,
                            datum_cmd,
                            datum_mesh,
                            datum_vbuf,
                        );
                    }
                    _ => {}
                }
            }
            _tasks.end_execute();
        }

        pub fn tak_create_semaphore(&mut self) -> Option<Vec<vk::Semaphore>> {
            todo!();
        }

        // exe model
        pub fn tak_bind_model(
            &mut self,
            first_bind_index: u32,
            tqin: &mut Datum<TaskQueue<RenderCmdTask>>,
        ) {
            let mut _tasks = get_mut!(tqin.vec_mut(), self.cmd_attachment.index_model_task);
            _tasks.push_task(RenderCmdTask::BindModel(
                first_bind_index,
                Self::_callback_bind_model,
            ));
        }

        // exe graphic pipeline
        pub fn tak_bind_render_pipe(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _tak = get_mut!(
                tin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            );

            _tak.push_task(RenderCmdTask::BindRenderPipe(
                Self::_callback_bind_render_pipe,
            ))
        }

        // exe graphic pipeline
        pub fn tak_begin_render_pass(&mut self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let mut _tasks = get_mut!(
                tqin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            );
            _tasks.push_task(RenderCmdTask::BeginRenderPass(
                Self::_callback_begin_render_pass,
            ));
        }

        // exe graphic pipeline
        pub fn tak_end_render_pass(&mut self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let mut _tasks = get_mut!(
                tqin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            );

            _tasks.push_task(RenderCmdTask::EndRenderPass(
                Self::_callback_end_render_pass,
            ));
        }

        // exe graphic pipeline
        pub fn tak_draw(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _task = get_mut!(
                tin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            );

            _task.push_task(RenderCmdTask::Draw(Self::_callback_draw));
        }

        pub fn tak_draw_index(&mut self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _task = get_mut!(
                tin.vec_mut(),
                self.cmd_attachment.index_graphic_pipeline_task
            );

            _task.push_task(RenderCmdTask::DrawIndex(Self::_callback_draw_index));
        }

        fn _callback_draw_index(
            cmd_slice: &RenderCmdE,
            command_buffers: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmdbuf = get!(
                command_buffers.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            unsafe {
                todo!();
                //cast_ref!(ash::Device, cmd_slice.device_p.unwrap()).cmd_draw_indexed(*_cmdbuf, 3, 1, 0, 0)
            };
        }

        fn _callback_bind_model(
            cmd_slice: &RenderCmdE,
            datum_model: &Datum<ModelD>,
            first_bind_index: &u32,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            datum_mesh: &Datum<model::mesh::env::MeshD>,
            datum_vbuf: &Datum<DeviceBuffer<vk::Buffer>>,
        ) {
            //
            let _device = cast_ref!(ash::Device, cmd_slice.device_p.unwrap());
            //
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmd: &CommandBuffer = get!(
                _cmd,
                (cmd_slice.cmd_attachment.index_binding_cmd_buffers) as usize
            );
            //
            let _binding_index: u32 = *first_bind_index;

            for mi in datum_model.vec_ref().iter() {
                let _vbuf_index = mi.get_attechment_index(mtid::MTID_DAT_VERTEX_BUF).unwrap();
                let _mesh_index = mi.get_attechment_index(mtid::MTID_DAT_MESH).unwrap();
                let _mesh = get!(datum_mesh.vec_ref(), _mesh_index);
                let _vbuf = get!(datum_cmd.vec_ref(), _vbuf_index);

                //
                let _vertex_buf = get!(datum_vbuf.vec_ref(), _vbuf_index)
                    .buffer_ref()
                    .unwrap();
                //
                unsafe {
                    _device.cmd_bind_vertex_buffers(*_cmd, _binding_index, &[*_vertex_buf], &[0])
                }
            }
        }

        fn _callback_draw(
            cmd_slice: &RenderCmdE,
            command_buffers: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmd = get!(
                command_buffers.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmdbuf: &CommandBuffer = get!(_cmd, (cmd_slice.cmd_attachment.cbib_draw) as usize);

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap()).cmd_draw(*_cmdbuf, 3, 1, 0, 0)
            };
        }

        // push reder
        fn _callback_begin_render_pass(
            datum_renderpipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            cmd_slice: &RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            datum_fbo: &Datum<DeviceBuffer<vk::Framebuffer>>,
        ) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmd: &CommandBuffer =
                get!(_cmd, (cmd_slice.cmd_attachment.cbib_pipeline) as usize);

            let _pipe = get!(
                datum_renderpipe.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_pipeline
            )
            .pipeline_ref();
            let _rpass: &RenderPass = get!(
                datum_renderpipe.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_pipeline
            )
            .pco_ref()
            .pass_ref()
            .unwrap();

            let _fb: &vk::Framebuffer = get!(
                datum_fbo.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();

            let _info: vk::RenderPassBeginInfo = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: null(),
                render_pass: *_rpass,
                framebuffer: *_fb,
                render_area: Rect2D {
                    offset: Default::default(),
                    extent: cmd_slice.render_area.unwrap(),
                },
                clear_value_count: RENDERER::DEFAULT_ERROR_COLOR.len() as u32,
                p_clear_values: RENDERER::DEFAULT_ERROR_COLOR.as_ptr(),
            };

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap()).cmd_begin_render_pass(
                    _cmd.clone(),
                    &_info,
                    match cmd_slice.cmd_attachment.usage_flag & CmdUsage::SUBCMD_MODE {
                        0 => vk::SubpassContents::INLINE,
                        _ => vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
                    },
                )
            }
        }

        pub fn tak_reset_cmd(&self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _task = get_mut!(tqin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task);
            _task.push_task(RenderCmdTask::ResetCmd(Self::_callback_reset_cmd));
        }

        pub fn tak_begin_cmd(&self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _task = get_mut!(tqin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task);
            _task.push_task(RenderCmdTask::BeginCmd(Self::_callback_begin_cmd));
        }

        pub fn reset_cmd(&self, datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>) {
            Self::_callback_reset_cmd(&self, datum_cmd)
        }

        fn _callback_reset_cmd(
            cmd_slice: &RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmd = get!(_cmd, (cmd_slice.cmd_attachment.cbib_cmdbuf) as usize);

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap())
                    .reset_command_buffer(*_cmd, vk::CommandBufferResetFlags::default())
                    .unwrap();
            };
        }

        fn _callback_begin_cmd(
            cmd_slice: &RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmd = get!(_cmd, (cmd_slice.cmd_attachment.cbib_cmdbuf) as usize);

            let begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: null(),
                flags: Default::default(),
                p_inheritance_info: null(),
            };

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap())
                    .begin_command_buffer(*_cmd, &begin_info)
                    .unwrap()
            };
        }

        fn _callback_bind_render_pipe(
            cmd_slice: &mut RenderCmdE,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmd = get_mut!(
                datum_cmd.vec_mut(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers as usize
            )
            .buffer_mut()
            .unwrap();
            let _cmd = get!(_cmd, (cmd_slice.cmd_attachment.cbib_pipeline) as usize);

            let _pipe = get_mut!(
                datum_pipe.vec_mut(),
                cmd_slice.cmd_attachment.index_binding_pipeline
            );

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap()).cmd_bind_pipeline(
                    *_cmd,
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

        pub fn _callback_end_render_pass(
            cmd_slice: &RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_cmd_buffer_task
            )
            .buffer_ref()
            .unwrap();
            let _cmd = get!(_cmd, (cmd_slice.cmd_attachment.cbib_pipeline) as usize);

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap())
                    .cmd_end_render_pass(_cmd.clone())
            }
        }

        ///
        /// # Abstract
        /// ## Example
        /// ## Parameter
        /**
         * switch_vertex_bind_index: None:{auto find suitable vad binding index}
         */
        #[allow(unused)]
        pub fn bind_specify_vertex(
            &mut self,
            index_model: usize,
            switch_vertex_bind_index: Option<u32>,
            datum_cmdbuf: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            dattum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_model: &mut Datum<model::env::ModelD>,
            datum_mesh: &mut Datum<model::mesh::env::MeshD>,
            datum_vbuf: &mut Datum<DeviceBuffer<vk::Buffer>>,
        ) {
            let _model = get!(datum_model.vec_ref(), index_model);
            let _pipe = get!(
                dattum_pipe.vec_ref(),
                self.cmd_attachment.index_binding_pipeline
            );

            let _cmd = get!(
                datum_cmdbuf.vec_ref(),
                self.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmd = get!(_cmd, (self.cmd_attachment.cbib_vertex) as usize);
            let _vertex_buf = get!(
                datum_vbuf.vec_ref(),
                _model
                    .get_attechment_index(mtid::MTID_DAT_VERTEX_BUF)
                    .unwrap()
            )
            .buffer_ref()
            .unwrap();
            let _mesh_index = _model.get_attechment_index(mtid::MTID_DAT_MESH).unwrap();
            let _mesh = get!(datum_mesh.vec_ref(), _mesh_index);
            let _binding_index: u32 = match switch_vertex_bind_index {
                Some(val) => val,
                None => Self::_find_suitable_vad_binding_index(_pipe, _mesh),
            };
            // todo!();
            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap()).cmd_bind_vertex_buffers(
                    *_cmd,
                    _binding_index,
                    &[*_vertex_buf],
                    &[0],
                );
                // todo!();
            }
        }

        #[allow(unused)]
        pub fn draw(&self, command_buffers: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>) {
            Self::_callback_draw(self, command_buffers);
        }

        #[allow(unused)]
        pub fn begin_cmd(&mut self, datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>) {
            Self::_callback_begin_cmd(self, datum_cmd);
        }

        #[allow(unused)]
        pub fn begin_render_pass(
            &self,
            fb_index: usize,
            datum_render_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            datum_fbo: &Datum<DeviceBuffer<vk::Framebuffer>>,
        ) {
            Self::_callback_begin_render_pass(datum_render_pipe, self, datum_cmd, datum_fbo);
        }

        #[allow(unused)]
        pub fn end_cmd(&mut self, datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                self.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();
            let _cmd = get!(_cmd, (self.cmd_attachment.cbib_cmdbuf) as usize);
            unsafe {
                cast_ref!(ash::Device, self.device_p.unwrap())
                    .end_command_buffer(*_cmd)
                    .unwrap();
            };
        }

        pub fn destroy_cmd(
            &mut self,
            index: usize,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            todo!();
        }

        #[allow(unused)]
        pub fn end_render_pass(&mut self, datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>) {
            Self::_callback_end_render_pass(self, datum_cmd);
        }

        #[allow(unused)]
        pub fn bind_render_pipe(
            &mut self,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            Self::_callback_bind_render_pipe(self, datum_pipe, datum_cmd);
        }

        //
        pub fn tak_init_submit_info(&self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let mut _task = get_mut!(tin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task);

            _task.push_task(RenderCmdTask::InitSubmitInfo(
                Self::_callback_init_submit_info,
            ))
        }

        pub fn init_submit_info(
            &mut self,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            Self::_callback_init_submit_info(self, datum_cmd);
        }

        /// update command buffer index binding in cmd attachment
        pub fn update_cbib(&mut self, cmd_bufs_len: usize) {
            self.cmd_attachment.cbib_vertex = self.cmd_attachment.cbib_vertex % cmd_bufs_len as u32;

            self.cmd_attachment.cbib_draw = self.cmd_attachment.cbib_draw % cmd_bufs_len as u32;

            self.cmd_attachment.cbib_vertex = self.cmd_attachment.cbib_vertex % cmd_bufs_len as u32;

            self.cmd_attachment.cbib_pipeline =
                self.cmd_attachment.cbib_pipeline % cmd_bufs_len as u32;

            self.cmd_attachment.cbib_cmdbuf = self.cmd_attachment.cbib_cmdbuf % cmd_bufs_len as u32;
        }

        fn _callback_init_submit_info(
            cmd_slice: &mut RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        ) {
            let _cmds = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();

            let mut _r = vk::SubmitInfo {
                s_type: vk::StructureType::SUBMIT_INFO,
                p_next: null(),
                wait_semaphore_count: cmd_slice.semaps_ref().len() as u32,
                p_wait_semaphores: cmd_slice.semaps_ref().as_ptr(),
                p_wait_dst_stage_mask: &cmd_slice.cmd_attachment.stage_flag,
                command_buffer_count: _cmds.len() as u32,
                p_command_buffers: _cmds.as_ptr(),
                signal_semaphore_count: cmd_slice.semaps_ref().len() as u32,
                p_signal_semaphores: cmd_slice.semaps_ref().as_ptr(),
            };

            cmd_slice.submit_info = Some(_r);
        }

        // exe cmd
        pub fn tak_submit_cmd_pipeline(&mut self, tqin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _task = get_mut!(tqin.vec_mut(), self.cmd_attachment.index_pipeline_task);

            _task.push_task(RenderCmdTask::SubmitCmd(
                Self::_callback_submit_cmd,
                self.cmd_attachment.index_pipeline_task,
            ));
        }

        pub fn submit_cmd(
            &mut self,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            cmd_index: usize,
        ) {
            Self::_callback_submit_cmd(self, datum_cmd, &cmd_index);
        }

        pub fn build_usage(mut self, uin: u64) -> Self {
            self.cmd_attachment.usage_flag = uin;
            return self;
        }

        /// # abstract
        /// use it before bind renderer
        /// ## parameter
        /// cmd_slice: &RenderCmdE,
        /// datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
        /// index_exe: &usize send it from exe_XXXX
        /// ## example
        fn _callback_submit_cmd(
            cmd_slice: &RenderCmdE,
            datum_cmd: &Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
            index_exe: &usize,
        ) {
            let _cmd = get!(
                datum_cmd.vec_ref(),
                cmd_slice.cmd_attachment.index_binding_cmd_buffers
            )
            .buffer_ref()
            .unwrap();

            let device_ref = cast_ref!(ash::Device, cmd_slice.device_p.unwrap());
            let _queue = unsafe {
                device_ref.get_device_queue(
                    cmd_slice.cmd_attachment.index_gpu_queue_family as u32,
                    cmd_slice.cmd_attachment.index_gpu_queue as u32,
                )
            };

            unsafe {
                cast_ref!(ash::Device, cmd_slice.device_p.unwrap())
                    .queue_submit(
                        _queue,
                        cmd_slice.submit_info.as_slice(),
                        *get!(cmd_slice.fence, index_exe % cmd_slice.fence.len()),
                    )
                    .unwrap();
            };
        }

        pub fn fences_slice_ref(&self) -> Result<&[vk::Fence], ()> {
            return Ok(self.fence.as_slice());
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

        pub fn tak_wait_fences(&self, tin: &mut Datum<TaskQueue<RenderCmdTask>>) {
            let _task = get_mut!(tin.vec_mut(), self.cmd_attachment.index_cmd_buffer_task);
            _task.push_task(RenderCmdTask::WaitFences(Self::_callback_wait_fences));
        }

        // uncommand
        pub fn wait_fences(&self, timer_slice: &TimerE) {
            Self::_callback_wait_fences(self, timer_slice);
        }

        fn _callback_wait_fences(cmd_slice: &RenderCmdE, timer_slice: &TimerE) {
            let _time_ref = timer_slice;
            let _device_ref = cast_ref!(ash::Device, cmd_slice.device_p.unwrap());

            let _wait_time: u64;
            // judge if time out
            if cmd_slice.frame_stride_ns > _time_ref.delta_time_ns() {
                _wait_time = cmd_slice.frame_stride_ns - _time_ref.delta_time_ns();
            } else {
                _wait_time = 0;
            }

            unsafe {
                let _ = _device_ref.wait_for_fences(
                    cmd_slice.fences_ref().as_slice(),
                    false,
                    _wait_time,
                );
            }
        }

        pub fn fences_ref(&self) -> &Vec<vk::Fence> {
            return self.fence.as_ref();
        }

        pub fn build_sync(mut self, renderer_slice: &RendererE, call_in_next_frame: bool) -> Self {
            let _info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FENCE_CREATE_INFO,
                p_next: null(),
                flags: match call_in_next_frame {
                    true => vk::FenceCreateFlags::empty(),
                    false => Default::default(),
                },
            };
            let _semap_info = crate::renderer::cfg::env::COMMAND::DEFAULT_SEMAP_INFO;
            let _semap: vk::Semaphore;

            let _fence = unsafe {
                renderer_slice
                    .gpu_ref()
                    .unwrap()
                    .logical_p
                    .as_ref()
                    .unwrap()
                    .create_fence(&_info, Option::None)
                    .unwrap()
            };

            self.fence.push(_fence);

            unsafe {
                _semap = renderer_slice
                    .device_ref()
                    .unwrap()
                    .create_semaphore(&_semap_info, Option::None)
                    .unwrap();
            }

            self.semaphore.resize(
                crate::renderer::cfg::env::COMMAND::DEFAULT_SEMAPHORE_COUNT,
                _semap,
            );

            return self;
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
        //     command_buffers: &mut Datum<DeviceBuffer<Vec<vk::CommandBuffer>>>,
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
    }

    impl Default for RenderCmdAttachment {
        fn default() -> Self {
            Self {
                index_cmd_buffer_task: usize::MAX,
                index_graphic_pipeline_task: usize::MAX,
                idnex_sync_task: usize::MAX,
                index_model_task: usize::MAX,
                index_pipeline_task: usize::MAX,
                usage_flag: Default::default(),

                id_bind_exe_renderer: u64::MAX,

                index_binding_pipeline: 0,
                index_binding_cmd_buffers: 0,
                index_gpu_queue: 0,
                index_gpu_queue_family: 0,
                count_active_swapimg: 0,
                index_current_surfimg_buf: 0,
                stage_flag: vk::PipelineStageFlags::ALL_GRAPHICS,
                priority_level: vk::CommandBufferLevel::PRIMARY.as_raw(),
                cbib_vertex: COMMAND::DEFAULT_COMMAND_BUFFER_INDEX_VERTEX,
                switch_cmd_buffer_index_bind_auto:
                    COMMAND::DEFAULT_COMMAND_BUFFER_INDEX_BINDING_AUTO,
                cbib_draw: COMMAND::DEFAULT_COMMAND_BUFFER_INDEX_BINDING_DRAW,
                cbib_pipeline: COMMAND::DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE,
                cbib_cmdbuf: COMMAND::DEFAULT_COMMAND_BUFFER_INDEX_BINDING_CMDBUF,
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
                buf_create_info: Default::default(),
                buf_begin_info: Option::Some(Default::default()),
                buf_inhernit_info: Option::None,
                buf_fence_info: Option::None,
                min_swapchainsurf_num: 2,
                semaphore: Default::default(),
                fence: Default::default(),
                event: Default::default(),
                frame_stride_ns: Default::default(),
                submit_info: Default::default(),
            }
        }
    }
}

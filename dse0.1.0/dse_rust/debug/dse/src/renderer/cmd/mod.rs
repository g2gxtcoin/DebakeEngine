//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::ptr::null;

    use ash::vk::{self, CommandBuffer, Rect2D, RenderPass};

    use crate::{
        get, get_mut,
        manager::{datum::env::Datum, execute::env::TaskQueue},
        model::{self, env::ModelD, rectangle::env::Rect},
        renderer::{
            env::{RendererE, RendererTask},
            pipeline::env::{GraphicPipeLinePCO, GraphicPipeLinePSO, PCOTrait, RenderPipelineD},
        },
    };

    pub struct CommandRenderAttachment {
        index_cmd: u64,
        index_pipe: u64,
    }

    impl Default for CommandRenderAttachment {
        fn default() -> Self {
            Self {
                index_cmd: Default::default(),
                index_pipe: Default::default(),
            }
        }
    }

    #[derive(Default)]
    pub struct CommandRenderE {
        id: u64,
        device_p: Option<ash::Device>,
        render_area: Option<vk::Extent2D>,
        cmd_attachment: CommandRenderAttachment,
    }

    #[derive(Default)]
    pub enum RenderCmdTask {
        #[default]
        None,
        RenderPass(),
    }

    // The mind you have is all disaster you deserved.
    // The way you choose is all tear you suffered.
    // The belief you believe will not live you in future.
    //
    impl CommandRenderE {
        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }

        pub fn build() -> Self {
            return Default::default();
        }

        pub fn set_render_rect(&mut self, height: u64, width: u64) {
            self.render_area = Some(vk::Extent2D {
                width: u32::try_from(width).unwrap(),
                height: u32::try_from(height).unwrap(),
            });
        }

        pub fn link_renderer(&mut self, rin: &RendererE) {
            self.device_p = Some(rin.device.clone().unwrap());
            self.render_area = Some(rin.swapchain_create_info.unwrap().image_extent.clone());
        }

        pub fn link_task_queue(&mut self, tin: &mut Datum<TaskQueue<RendererTask>>) {}

        pub fn bind_pipe_sync(
            &mut self,
            datum_cmd: &mut Datum<vk::CommandBuffer>,
            datum_pipe: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            index_cmd: usize,
            index_pipe: usize,
        ) {
            let _cmd = get_mut!(datum_cmd.vec_mut(), index_cmd).as_mut().unwrap();
            let _pipe = get_mut!(datum_pipe.vec_mut(), index_pipe).as_mut().unwrap();

            unsafe {
                self.device_p.as_mut().unwrap().cmd_bind_pipeline(
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

        pub fn bind_specify_model_sync(
            &mut self,
            datum_model: Datum<ModelD>
        ){
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

        pub fn bind_vertex_sync(
            &mut self,
            datum_cmd: &mut Datum<vk::CommandBuffer>,
            datum_mesh: &mut Datum<model::mesh::env::MeshD>,
            index_cmd: usize,
        ) {
            let _cmd: &mut CommandBuffer =
                get_mut!(datum_cmd.vec_mut(), index_cmd).as_mut().unwrap();
            let _meshs = datum_mesh.vec_mut();

            todo!();
        }

        pub fn begin_cmd_sync(&mut self, datum_cmd: &Datum<vk::CommandBuffer>, index: usize) {
            let _cmd = get!(datum_cmd.vec_ref(), index).as_ref().unwrap();
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
                    .begin_command_buffer(*_cmd, &begin_info)
                    .unwrap()
            };
        }

        pub fn end_render_pass_sync(
            &mut self,
            cmd_index: usize,
            datum_cmd: &Datum<vk::CommandBuffer>,
        ) {
            let _cmd = get!(datum_cmd.vec_ref(), cmd_index).as_ref().unwrap();
            unsafe {
                self.device_p
                    .as_mut()
                    .unwrap()
                    .cmd_end_render_pass(_cmd.clone())
            }
        }

        // push reder
        pub fn begin_render_pass_sync(
            &mut self,
            cmd_index: usize,
            rp_index: usize,
            fb_index: usize,
            datum_renderpass: &mut Datum<RenderPipelineD<GraphicPipeLinePSO, GraphicPipeLinePCO>>,
            datum_cmd: &Datum<vk::CommandBuffer>,
            datum_fbo: &Datum<vk::Framebuffer>,
        ) {
            let _cmd = get!(datum_cmd.vec_ref(), cmd_index).as_ref().unwrap();
            let _rpass = get!(datum_renderpass.vec_ref(), rp_index)
                .as_ref()
                .unwrap()
                .pco_ref()
                .pass_ref()
                .unwrap()
                .clone();

            let _fb = get!(datum_fbo.vec_ref(), fb_index)
                .as_ref()
                .unwrap()
                .clone();

            let _info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: null(),
                render_pass: _rpass,
                framebuffer: _fb,
                render_area: Rect2D {
                    offset: Default::default(),
                    extent: self.render_area.unwrap(),
                },
                clear_value_count: 0,
                p_clear_values: null(),
            };

            unsafe {
                self.device_p.as_mut().unwrap().cmd_begin_render_pass(
                    _cmd.clone(),
                    &_info,
                    vk::SubpassContents::INLINE,
                )
            }
        }

        pub fn draw_sync(&mut self, command_buffers: &mut Datum<CommandBuffer>) {
            let _dc = get!(command_buffers.vec_ref(), 0).unwrap();
            unsafe { self.device_p.as_mut().unwrap().cmd_draw(_dc, 3, 1, 0, 0) };
        }

        pub fn render_pass_sync(&mut self) {}

        fn _calllback_render_pass(cmd_slice: &mut CommandRenderE) {}

        pub fn exe_cmd_buffer() {}
    }
}

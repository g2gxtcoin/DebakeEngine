#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::{
        any::{Any, TypeId},
        default,
        ptr::{addr_of, null},
    };

    use ash::vk::{
        self, PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo, SubpassDescription,
    };

    use crate::{________________dev_stop________________, dev_dbg, renderer::cfg};

    use crate::{
        manager::datum::env::Datum, send2logger_dev, shader::env::ShaderModuleD,
        workarea::resolution_default,
    };
    pub trait PSOTrait
    where
        Self: Default,
    {
        fn renderpass_info_ref(&self) -> Option<&vk::RenderPassCreateInfo> {
            return Option::None;
        }

        fn renderpass_info_mut(&mut self) -> Option<&mut vk::RenderPassCreateInfo> {
            return Option::None;
        }
        // Graphic ref
        fn vertex_input_state_ref(&self) -> Option<&vk::PipelineVertexInputStateCreateInfo> {
            return Option::None;
        }

        fn input_assembly_state_ref(&self) -> Option<&vk::PipelineInputAssemblyStateCreateInfo> {
            return Option::None;
        }

        fn tessellation_state_ref(&self) -> Option<&vk::PipelineTessellationStateCreateInfo> {
            return Option::None;
        }

        fn viewport_state_ref(&self) -> Option<&vk::PipelineViewportStateCreateInfo> {
            return Option::None;
        }

        fn rasterization_state_ref(&self) -> Option<&vk::PipelineRasterizationStateCreateInfo> {
            return Option::None;
        }

        fn multisample_state_ref(&self) -> Option<&vk::PipelineMultisampleStateCreateInfo> {
            return Option::None;
        }

        fn depth_stencil_state_ref(&self) -> Option<&vk::PipelineDepthStencilStateCreateInfo> {
            return Option::None;
        }

        fn color_blend_state_ref(&self) -> Option<&vk::PipelineColorBlendStateCreateInfo> {
            return Option::None;
        }

        // Graphic mut
        fn vertex_input_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineVertexInputStateCreateInfo> {
            return Option::None;
        }

        fn input_assembly_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineInputAssemblyStateCreateInfo> {
            return Option::None;
        }

        fn tessellation_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineTessellationStateCreateInfo> {
            return Option::None;
        }

        fn viewport_state_mut(&mut self) -> Option<&mut vk::PipelineViewportStateCreateInfo> {
            return Option::None;
        }

        fn rasterization_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineRasterizationStateCreateInfo> {
            return Option::None;
        }

        fn multisample_state_mut(&mut self) -> Option<&mut vk::PipelineMultisampleStateCreateInfo> {
            return Option::None;
        }

        fn depth_stencil_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineDepthStencilStateCreateInfo> {
            return Option::None;
        }

        fn color_blend_state_mut(&mut self) -> Option<&mut vk::PipelineColorBlendStateCreateInfo> {
            return Option::None;
        }
    }

    // 超出限制部分将被丢弃
    pub trait PCOTrait
    where
        Self: Default,
    {
        fn pass_ref(&self) -> Result<&vk::RenderPass, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }

        fn pass_mut(&mut self) -> Result<&mut vk::RenderPass, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }

        fn subpass_descriptions_ref(&self) -> Result<&Vec<vk::SubpassDescription>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }

        fn subpass_descriptions_mut(&mut self) -> Result<&mut Vec<vk::SubpassDescription>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }

        // Graphic ref
        fn vertex_binding_description_ref(
            &self,
        ) -> Result<&Vec<vk::VertexInputBindingDescription>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn vertex_attribute_description_ref(
            &self,
        ) -> Result<&Vec<vk::VertexInputAttributeDescription>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn viewports_ref(&self) -> Result<&Vec<vk::Viewport>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn rect_scissors_ref(&self) -> Result<&Vec<vk::Rect2D>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn blend_attachments_ref(&self) -> Result<&Vec<vk::PipelineColorBlendAttachmentState>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }

        fn sample_masks_ref(&self) -> Result<&Vec<vk::SampleMask>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        // Graphic mut

        fn sample_masks_mut(&mut self) -> Result<&mut Vec<vk::SampleMask>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }

        fn vertex_binding_description_mut(
            &mut self,
        ) -> Result<&mut Vec<vk::VertexInputBindingDescription>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn vertex_attribute_description_mut(
            &mut self,
        ) -> Result<&mut Vec<vk::VertexInputAttributeDescription>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn viewports_mut(&mut self) -> Result<&mut Vec<vk::Viewport>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn rect_scissors_mut(&mut self) -> Result<&mut Vec<vk::Rect2D>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
        fn blend_attachments_mut(
            &mut self,
        ) -> Result<&mut Vec<vk::PipelineColorBlendAttachmentState>, ()> {
            return Err(crate::send2logger_dev!(
                crate::log::code::TYPE_TRAIT_ERROR
                    | crate::log::code::CONDI_UNDEFINE_TRAIT_MEM
                    | crate::log::code::FILE_RENDERER_PIPELINE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code()
            ));
        }
    }

    pub enum PipelineCreateInfoResult {
        None,
        Graphic(vk::GraphicsPipelineCreateInfo),
        Compute(vk::ComputePipelineCreateInfo),
        RayTracing(vk::RayTracingPipelineCreateInfoKHR),
    }

    pub enum RenderPipelineType {
        None,
        Graphic,
        Compute,
        RayTracing,
    }

    pub mod state_optional {
        use crate::renderer::cfg::env::PSO;

        use super::GraphicPipeLinePSO;
        /// 预设选项中出现 u32::MAX 意味着自动依据gpu属性匹配最大值
        pub const DEFAULT_GRAPHIC_PIPELINE: GraphicPipeLinePSO = GraphicPipeLinePSO {
            vertex_input_state: Some(PSO::DEFAULT_VERTEX_INPUT_STATE),
            input_assembly_state: Some(PSO::DEFAULT_INPUT_ASSEMBLY_STATE),
            tessellation_state: Some(PSO::DEFAULT_TESSELLATION_STATE),
            viewport_state: Some(PSO::DEFAULT_VIEWPORT_STATE),
            rasterization_state: Some(PSO::DEFAULT_RASTERIZATION_STATE),
            multisample_state: Some(PSO::DEFAULT_MULTISAMPLE_STATE),
            depth_stencil_state: Some(PSO::DEFAULT_DEPTH_STENCIL),
            color_blend_state: Some(PSO::DEFAULT_COLOR_BLEND),
            renderpass_info: Some(PSO::DEFAULT_RENDER_PASS),
        };
    }

    // pipeline statge obj
    #[allow(unused)]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
    #[derive(Clone, Copy, Debug)]
    pub struct GraphicPipeLinePSO {
        vertex_input_state: Option<vk::PipelineVertexInputStateCreateInfo>,
        input_assembly_state: Option<vk::PipelineInputAssemblyStateCreateInfo>,
        tessellation_state: Option<vk::PipelineTessellationStateCreateInfo>,
        viewport_state: Option<vk::PipelineViewportStateCreateInfo>,
        rasterization_state: Option<vk::PipelineRasterizationStateCreateInfo>,
        multisample_state: Option<vk::PipelineMultisampleStateCreateInfo>,
        depth_stencil_state: Option<vk::PipelineDepthStencilStateCreateInfo>,
        color_blend_state: Option<vk::PipelineColorBlendStateCreateInfo>,
        renderpass_info: Option<vk::RenderPassCreateInfo>,
    }

    // pipeline componet obj
    #[derive(Default, Debug)]
    pub struct GraphicPipeLinePCO {
        vertex_binding_description: Vec<vk::VertexInputBindingDescription>,
        vertex_attribute_description: Vec<vk::VertexInputAttributeDescription>,
        viewports: Vec<vk::Viewport>,
        rect_scissors: Vec<vk::Rect2D>,
        blend_attachments: Vec<vk::PipelineColorBlendAttachmentState>,
        sample_masks: Vec<vk::SampleMask>,
        subpass_description: Vec<vk::SubpassDescription>,
        main_pass: Option<vk::RenderPass>,
    }

    #[derive(Default, Debug)]
    pub struct RenderPipelineAttachment {
        index_fbo: usize,
        index_vbo: usize,
    }

    #[derive(Default, Debug)]
    pub struct RenderPipelineD<TStates, TComponents>
    where
        TStates: PSOTrait,
        TComponents: PCOTrait,
    {
        id: u64,
        pipeline_attachment: RenderPipelineAttachment,

        states: TStates,
        components: TComponents,

        shader_stages: Vec<vk::PipelineShaderStageCreateInfo>,
        layout_create_info: Option<vk::PipelineLayoutCreateInfo>,
        layout: Option<vk::PipelineLayout>,

        pipeline: Option<vk::Pipeline>,
    }

    impl<TStates, TComponents> RenderPipelineD<TStates, TComponents>
    where
        TStates: PSOTrait + Clone + Any,
        TComponents: PCOTrait + Any,
    {
        fn _refresh_states_shader_satges_info(&mut self) {
            match self.render_pipeline_type() {
                RenderPipelineType::None => todo!(),
                RenderPipelineType::Graphic => {
                    //self.states.
                }
                RenderPipelineType::Compute => todo!(),
                RenderPipelineType::RayTracing => todo!(),
            }
        }

        pub fn pipeline_mut(&mut self) -> &mut vk::Pipeline {
            return self.pipeline.as_mut().unwrap();
        }

        pub fn pipeline_ref(&self) -> &vk::Pipeline {
            return self.pipeline.as_ref().unwrap();
        }

        pub fn set_pipeline(&mut self, pin: vk::Pipeline) {
            self.pipeline = Some(pin);
        }

        pub fn pco_ref(&self) -> &TComponents {
            return &self.components;
        }

        pub fn pco_mut(&mut self) -> &mut TComponents {
            return &mut self.components;
        }

        pub fn pso_ref(&self) -> &TStates {
            return &self.states;
        }

        pub fn pso_mut(&mut self) -> &mut TStates {
            return &mut self.states;
        }

        pub fn layout_create_info_ref(&self) -> &vk::PipelineLayoutCreateInfo {
            return self
                .layout_create_info
                .as_ref()
                .unwrap_or(&cfg::env::PSO::DEFAULT_LAYOUT);
        }

        pub fn render_pipeline_type(&self) -> RenderPipelineType {
            if TypeId::of::<GraphicPipeLinePCO>() == TypeId::of::<TComponents>()
                && TypeId::of::<GraphicPipeLinePSO>() == TypeId::of::<TStates>()
            {
                return RenderPipelineType::Graphic;
            }
            return RenderPipelineType::None;
        }

        pub fn layout_ref(&self) -> &vk::PipelineLayout {
            return self.layout.as_ref().unwrap();
        }

        pub fn set_layout(&mut self, lin: vk::PipelineLayout) {
            self.layout = Some(lin);
        }

        fn _valid_vertex_input_state(&mut self, gpu_properties: &vk::PhysicalDeviceProperties) {
            // 判断已设vbd不为空
            if !self
                .components
                .vertex_binding_description_ref()
                .unwrap()
                .is_empty()
            {
                // 优先使用已设vbd
                self.states
                    .vertex_input_state_mut()
                    .unwrap()
                    .p_vertex_binding_descriptions = self
                    .components
                    .vertex_binding_description_ref()
                    .unwrap()
                    .as_ptr();

                // 判断已设vbd超出硬件限制
                if self
                    .components
                    .vertex_binding_description_ref()
                    .unwrap()
                    .len() as u32
                    > gpu_properties.limits.max_vertex_input_attributes
                {
                    self.states
                        .vertex_input_state_mut()
                        .unwrap()
                        .vertex_binding_description_count =
                        gpu_properties.limits.max_vertex_input_attributes;
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_WARN
                            | crate::log::code::CONDI_VK_GPU_LIMITIS
                            | crate::log::code::FILE_RENDERER_PIPELINE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    );
                }
                // 已设vbd未超出硬件限制
                else {
                    self.states
                        .vertex_input_state_mut()
                        .unwrap()
                        .vertex_binding_description_count = self
                        .components
                        .vertex_binding_description_ref()
                        .unwrap()
                        .len() as u32;
                }
            }
            // 已设vbd为空 指向默认vbd
            else {
                self.states
                    .vertex_input_state_mut()
                    .unwrap()
                    .p_vertex_binding_descriptions = &cfg::env::PSO::DEFAULT_VBD;
            }

            // 判断已设vad是否不为空
            if !self
                .components
                .vertex_attribute_description_mut()
                .unwrap()
                .is_empty()
            {
                // 判断已设vad超出硬件限制
                if self
                    .components
                    .vertex_attribute_description_mut()
                    .unwrap()
                    .len() as u32
                    > gpu_properties.limits.max_vertex_input_attributes
                {
                    self.states
                        .vertex_input_state_mut()
                        .unwrap()
                        .vertex_attribute_description_count =
                        gpu_properties.limits.max_vertex_input_attributes;
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_WARN
                            | crate::log::code::CONDI_VK_GPU_LIMITIS
                            | crate::log::code::FILE_RENDERER_PIPELINE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    );
                }
                // 已设vad未超出硬件限制
                else {
                    self.states
                        .vertex_input_state_mut()
                        .unwrap()
                        .vertex_attribute_description_count = self
                        .components
                        .vertex_attribute_description_mut()
                        .unwrap()
                        .len() as u32;
                }
            }
            // 已设vad为空 判断是否指向预设vad
            else if self
                .states
                .vertex_input_state_mut()
                .unwrap()
                .p_vertex_attribute_descriptions
                .is_null()
            {
                self.states
                    .vertex_input_state_mut()
                    .unwrap()
                    .p_vertex_attribute_descriptions = &cfg::env::PSO::DEFAULT_VAD;
            }
        }
        fn _valid_viewport_state(&mut self, gpu_properties: &vk::PhysicalDeviceProperties) {
            if self.states.viewport_state_mut().is_some() {
                if !self.components.viewports_mut().unwrap().is_empty() {
                    if self.components.viewports_mut().unwrap().len() as u32
                        > gpu_properties.limits.max_viewports
                    {
                        self.states.viewport_state_mut().unwrap().viewport_count =
                            gpu_properties.limits.max_viewports;
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_WARN
                                | crate::log::code::CONDI_VK_GPU_LIMITIS
                                | crate::log::code::FILE_RENDERER_PIPELINE
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
                    }
                    self.states.viewport_state_mut().unwrap().p_viewports =
                        self.components.viewports_mut().unwrap().as_ptr();
                }
                if !self.components.rect_scissors_mut().unwrap().is_empty() {
                    if self.components.rect_scissors_mut().unwrap().len() as u32
                        > gpu_properties.limits.max_viewports
                    {
                        self.states.viewport_state_mut().unwrap().scissor_count =
                            gpu_properties.limits.max_viewports;
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_WARN
                                | crate::log::code::CONDI_VK_GPU_LIMITIS
                                | crate::log::code::FILE_RENDERER_PIPELINE
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
                    }
                    self.states.viewport_state_mut().unwrap().p_scissors =
                        self.components.rect_scissors_mut().unwrap().as_ptr();
                }
            }
        }
        fn _valid_tessellation_state(&mut self, gpu_properties: &vk::PhysicalDeviceProperties) {
            if self.states.tessellation_state_mut().is_some() {
                if self
                    .states
                    .tessellation_state_mut()
                    .unwrap()
                    .patch_control_points
                    > gpu_properties.limits.max_tessellation_patch_size
                {
                    self.states
                        .tessellation_state_mut()
                        .unwrap()
                        .patch_control_points = gpu_properties.limits.max_tessellation_patch_size;
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_WARN
                            | crate::log::code::CONDI_VK_GPU_LIMITIS
                            | crate::log::code::FILE_RENDERER_PIPELINE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    );
                }
            }
        }
        fn _valid_multisample_state(&mut self, gpu_properties: &vk::PhysicalDeviceProperties) {
            if self.states.multisample_state_mut().is_some() {
                let mut _max_multisample_count =
                    gpu_properties.limits.framebuffer_color_sample_counts
                        & gpu_properties.limits.framebuffer_depth_sample_counts;
                match _max_multisample_count {
                    vk::SampleCountFlags::TYPE_1 => {}
                    vk::SampleCountFlags::TYPE_2 => {}
                    vk::SampleCountFlags::TYPE_4 => {}
                    vk::SampleCountFlags::TYPE_8 => {}
                    vk::SampleCountFlags::TYPE_16 => {}
                    vk::SampleCountFlags::TYPE_32 => {}
                    vk::SampleCountFlags::TYPE_64 => {}
                    //vk::SampleCountFlags::TYPE_128 =>{},
                    //vk::SampleCountFlags::TYPE_256 =>{},
                    _ => {
                        _max_multisample_count = vk::SampleCountFlags::TYPE_1;
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_WARN
                                | crate::log::code::CONDI_UNMATCH_ENUM
                                | crate::log::code::FILE_RENDERER_PIPELINE
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
                    }
                }
                self.states
                    .multisample_state_mut()
                    .unwrap()
                    .rasterization_samples = _max_multisample_count;
            }
        }

        /// 预设验证函数
        /// 该验证函数会在构建渲染管线时被调用
        fn _valid_graphic(&mut self, gpu_properties: &vk::PhysicalDeviceProperties) {
            self._valid_vertex_input_state(gpu_properties);
            self._valid_viewport_state(gpu_properties);
            self._valid_tessellation_state(gpu_properties);
            self._valid_multisample_state(gpu_properties);
        }

        pub fn build() -> Self {
            let mut _r = Self::default();
            _r.layout_create_info = Some(cfg::env::PSO::DEFAULT_LAYOUT);
            return _r;
        }

        pub fn build_layout_info(mut self, info_in: vk::PipelineLayoutCreateInfo) -> Self {
            self.layout_create_info = Some(info_in);
            return self;
        }

        /// 引擎内置验证函数
        /// 与vulkan自带验证层不同
        /// 提前使用该验证函数 使得渲染管线不会由于设置超出硬件限制
        /// 而导致程序panic
        /// 机理是预先判断选用安全的默认值 或将超出部分进行钳制
        pub fn build_valid_pso(mut self, gpu_properties: &vk::PhysicalDeviceProperties) -> Self {
            //match  self.states {}
            self._valid_graphic(gpu_properties);

            return self;
        }

        pub fn build_specify_pso(mut self, pso_in: &TStates) -> Self {
            let mut _op = pso_in.clone();
            self.states = _op;
            return self;
        }

        pub fn build_render_pass(mut self, cin: vk::RenderPassCreateInfo) -> Self {
            match self.states.renderpass_info_mut() {
                Some(val) => *val = cin,
                None => {}
            }
            return self;
        }

        pub fn build_push_subpass(mut self, din: vk::SubpassDescription) -> Self {
            match self.components.subpass_descriptions_mut() {
                Ok(val) => val.push(din),
                Err(_) => {}
            }
            match self.states.renderpass_info_mut() {
                Some(val) => {
                    val.subpass_count =
                        self.components.subpass_descriptions_mut().unwrap().len() as u32;
                    val.p_subpasses = self.components.subpass_descriptions_ref().unwrap().as_ptr();
                }
                None => {}
            }
            return self;
        }

        pub fn build_push_blend_attachment(
            mut self,
            bain: vk::PipelineColorBlendAttachmentState,
        ) -> Self {
            match self.components.blend_attachments_mut() {
                Ok(val) => val.push(bain),
                Err(_) => {
                    send2logger_dev!(
                        crate::log::code::TYPE_DAT_WARN
                            | crate::log::code::CONDI_UNEXPECTED_RESULT
                            | crate::log::code::FILE_RENDERER_PIPELINE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    )
                }
            }

            return self;
        }

        pub fn build_push_sample_mask(mut self, sin: vk::SampleMask) -> Self {
            match self.components.sample_masks_mut() {
                Ok(val) => val.push(sin),
                Err(_) => {}
            }
            return self;
        }

        pub fn build_push_viewport_scissors(mut self, vin: vk::Viewport, rin: vk::Rect2D) -> Self {
            match self.components.viewports_mut() {
                Ok(val) => val.push(vin),
                Err(_) => {}
            }
            match self.components.rect_scissors_mut() {
                Ok(val) => val.push(rin),
                Err(_) => {}
            }

            return self;
        }

        pub fn build_push_vbd(mut self, vbd_in: vk::VertexInputBindingDescription) -> Self {
            match self.components.vertex_binding_description_mut() {
                Ok(val) => val.push(vbd_in),
                Err(_) => {}
            }

            return self;
        }

        pub fn build_push_vad(mut self, vad_in: vk::VertexInputAttributeDescription) -> Self {
            match self.components.vertex_attribute_description_mut() {
                Ok(val) => val.push(vad_in),
                Err(_) => {}
            }

            return self;
        }

        pub fn build_vertex_input_state(mut self) -> Self {
            let _s = vk::PipelineVertexInputStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                p_next: null(),
                flags: Default::default(),
                vertex_binding_description_count: self
                    .components
                    .vertex_binding_description_ref()
                    .unwrap()
                    .len() as u32,
                p_vertex_binding_descriptions: self
                    .components
                    .vertex_binding_description_mut()
                    .unwrap()
                    .as_ptr(),
                vertex_attribute_description_count: self
                    .components
                    .vertex_attribute_description_mut()
                    .unwrap()
                    .len() as u32,
                p_vertex_attribute_descriptions: self
                    .components
                    .vertex_attribute_description_mut()
                    .unwrap()
                    .as_ptr(),
            };
            *self.states.vertex_input_state_mut().unwrap() = _s;
            return self;
        }

        pub fn build_push_shader_stage(mut self, sin: &mut ShaderModuleD) -> Self {
            self.shader_stages.push(sin.pipe_stage_info().unwrap());
            return self;
        }

        pub fn build_push_shader_stages(mut self, sin: &mut Datum<ShaderModuleD>) -> Self {
            for si in sin.vec_mut() {
                self.shader_stages
                    .push(si.as_mut().unwrap().pipe_stage_info().unwrap());
            }

            return self;
        }

        pub fn build_update_pass_rect(mut self) -> Self {
            return self;
        }

        fn _graphic_pipeline_info(&mut self) -> vk::GraphicsPipelineCreateInfo {
            return vk::GraphicsPipelineCreateInfo {
                s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
                p_next: null(),
                flags:  Default::default(),
                stage_count: self.shader_stages.len() as u32,
                p_stages: self.shader_stages.as_ptr(),
                p_vertex_input_state: match self.states.vertex_input_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_input_assembly_state: match self.states.input_assembly_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_tessellation_state: match self.states.tessellation_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_viewport_state: match self.states.viewport_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_rasterization_state: match self.states.rasterization_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_multisample_state: match self.states.multisample_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_depth_stencil_state: match self.states.depth_stencil_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_color_blend_state: match self.states.color_blend_state_mut() {
                    Some(val) => val,
                    None => null(),
                },
                p_dynamic_state: null(),
                layout: match self.layout {
                    Some(val) => val,
                    None => Default::default(),
                },
                render_pass: *self.components.pass_ref().as_deref().unwrap(),
                subpass: 0,
                base_pipeline_handle: vk::Pipeline::null(),
                base_pipeline_index: 0,
            };
        }

        pub fn pipeline_info(&mut self) -> PipelineCreateInfoResult {
            match self.render_pipeline_type() {
                RenderPipelineType::None => PipelineCreateInfoResult::None,
                RenderPipelineType::Graphic => {
                    return PipelineCreateInfoResult::Graphic(self._graphic_pipeline_info())
                }
                RenderPipelineType::Compute => todo!(),
                RenderPipelineType::RayTracing => todo!(),
            }
        }

        pub fn get_dynamic_state_cmdbuffer(&self) {}
    }

    impl PSOTrait for GraphicPipeLinePSO {
        fn renderpass_info_mut(&mut self) -> Option<&mut vk::RenderPassCreateInfo> {
            return self.renderpass_info.as_mut();
        }

        fn renderpass_info_ref(&self) -> Option<&vk::RenderPassCreateInfo> {
            return self.renderpass_info.as_ref();
        }

        fn vertex_input_state_ref(&self) -> Option<&vk::PipelineVertexInputStateCreateInfo> {
            return self.vertex_input_state.as_ref();
        }

        fn input_assembly_state_ref(&self) -> Option<&vk::PipelineInputAssemblyStateCreateInfo> {
            return self.input_assembly_state.as_ref();
        }

        fn tessellation_state_ref(&self) -> Option<&vk::PipelineTessellationStateCreateInfo> {
            return self.tessellation_state.as_ref();
        }

        fn viewport_state_ref(&self) -> Option<&vk::PipelineViewportStateCreateInfo> {
            return self.viewport_state.as_ref();
        }

        fn rasterization_state_ref(&self) -> Option<&vk::PipelineRasterizationStateCreateInfo> {
            return self.rasterization_state.as_ref();
        }

        fn multisample_state_ref(&self) -> Option<&vk::PipelineMultisampleStateCreateInfo> {
            return self.multisample_state.as_ref();
        }

        fn depth_stencil_state_ref(&self) -> Option<&vk::PipelineDepthStencilStateCreateInfo> {
            return self.depth_stencil_state.as_ref();
        }

        fn color_blend_state_ref(&self) -> Option<&vk::PipelineColorBlendStateCreateInfo> {
            return self.color_blend_state.as_ref();
        }

        //
        fn vertex_input_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineVertexInputStateCreateInfo> {
            return self.vertex_input_state.as_mut();
        }

        fn input_assembly_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineInputAssemblyStateCreateInfo> {
            return self.input_assembly_state.as_mut();
        }

        fn tessellation_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineTessellationStateCreateInfo> {
            return self.tessellation_state.as_mut();
        }

        fn viewport_state_mut(&mut self) -> Option<&mut vk::PipelineViewportStateCreateInfo> {
            return self.viewport_state.as_mut();
        }

        fn rasterization_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineRasterizationStateCreateInfo> {
            return self.rasterization_state.as_mut();
        }

        fn multisample_state_mut(&mut self) -> Option<&mut vk::PipelineMultisampleStateCreateInfo> {
            return self.multisample_state.as_mut();
        }

        fn depth_stencil_state_mut(
            &mut self,
        ) -> Option<&mut vk::PipelineDepthStencilStateCreateInfo> {
            return self.depth_stencil_state.as_mut();
        }

        fn color_blend_state_mut(&mut self) -> Option<&mut vk::PipelineColorBlendStateCreateInfo> {
            return self.color_blend_state.as_mut();
        }
    }

    impl PCOTrait for GraphicPipeLinePCO {
        fn pass_ref(&self) -> Result<&vk::RenderPass, ()> {
            return Ok(self.main_pass.as_ref().unwrap());
        }

        fn pass_mut(&mut self) -> Result<&mut vk::RenderPass, ()> {
            return Ok(self.main_pass.as_mut().unwrap());
        }

        fn subpass_descriptions_ref(&self) -> Result<&Vec<vk::SubpassDescription>, ()> {
            return Ok(&self.subpass_description);
        }

        fn subpass_descriptions_mut(&mut self) -> Result<&mut Vec<vk::SubpassDescription>, ()> {
            return Ok(&mut self.subpass_description);
        }
        // Graphic ref
        fn vertex_binding_description_ref(
            &self,
        ) -> Result<&Vec<vk::VertexInputBindingDescription>, ()> {
            return Ok(&self.vertex_binding_description);
        }
        fn vertex_attribute_description_ref(
            &self,
        ) -> Result<&Vec<vk::VertexInputAttributeDescription>, ()> {
            return Ok(&self.vertex_attribute_description);
        }
        fn viewports_ref(&self) -> Result<&Vec<vk::Viewport>, ()> {
            return Ok(&self.viewports);
        }
        fn rect_scissors_ref(&self) -> Result<&Vec<vk::Rect2D>, ()> {
            return Ok(&self.rect_scissors);
        }
        fn blend_attachments_ref(&self) -> Result<&Vec<vk::PipelineColorBlendAttachmentState>, ()> {
            return Ok(&self.blend_attachments);
        }
        // Graphic mut

        fn vertex_binding_description_mut(
            &mut self,
        ) -> Result<&mut Vec<vk::VertexInputBindingDescription>, ()> {
            return Ok(&mut self.vertex_binding_description);
        }
        fn vertex_attribute_description_mut(
            &mut self,
        ) -> Result<&mut Vec<vk::VertexInputAttributeDescription>, ()> {
            return Ok(&mut self.vertex_attribute_description);
        }
        fn viewports_mut(&mut self) -> Result<&mut Vec<vk::Viewport>, ()> {
            return Ok(&mut self.viewports);
        }
        fn rect_scissors_mut(&mut self) -> Result<&mut Vec<vk::Rect2D>, ()> {
            return Ok(&mut self.rect_scissors);
        }
        fn blend_attachments_mut(
            &mut self,
        ) -> Result<&mut Vec<vk::PipelineColorBlendAttachmentState>, ()> {
            return Ok(&mut self.blend_attachments);
        }
    }

    impl Default for GraphicPipeLinePSO {
        fn default() -> Self {
            return state_optional::DEFAULT_GRAPHIC_PIPELINE.clone();
        }
    }

    impl GraphicPipeLinePCO {
        pub fn set_render_pass(&mut self, pin: vk::RenderPass) {
            self.main_pass = Option::Some(pin);
        }
    }
}

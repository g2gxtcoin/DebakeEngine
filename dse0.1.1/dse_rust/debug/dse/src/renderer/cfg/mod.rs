#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
#[allow(non_snake_case)]
#[allow(unused)]
pub mod env {

    pub mod RENDERER {
        use ash::vk;

        #[cfg(feature = "config_DEFAULT_RENDER_FPS_120")]
        pub const DEFAULT_RENDER_FRAME_STRIDE: u64 = 1000_0000_00 / 120;
        #[cfg(feature = "config_DEFAULT_RENDER_FPS_60")]
        pub const DEFAULT_RENDER_FRAME_STRIDE: u64 = 1000_0000_00 / 60;
        #[cfg(feature = "config_DEFAULT_RENDER_FPS_30")]
        pub const DEFAULT_RENDER_FRAME_STRIDE: u64 = 1000_0000_00 / 30;
        #[cfg(feature = "config_DEFAULT_RENDER_FPS_UNLIMITED")]
        pub const DEFAULT_RENDER_FRAME_STRIDE: u64 = 0;

        #[cfg(feature = "config_DEFAULT_LOGICAL_FPS_120")]
        pub const DEFAULT_LOGICAL_FRAME_STRIDE: u64 = 1000_0000_00 / 120;
        #[cfg(feature = "config_DEFAULT_LOGICAL_FPS_60")]
        pub const DEFAULT_LOGICAL_FRAME_STRIDE: u64 = 1000_0000_00 / 60;
        #[cfg(feature = "config_DEFAULT_LOGICAL_FPS_30")]
        pub const DEFAULT_LOGICAL_FRAME_STRIDE: u64 = 1000_0000_00 / 30;
        #[cfg(feature = "config_DEFAULT_LOGICAL_FPS_UNLIMITED")]
        pub const DEFAULT_LOGICAL_FRAME_STRIDE: u64 = 0;

        #[cfg(feature = "config_DEFAULT_PHYSICS_FPS_120")]
        pub const DEFAULT_PHYSICS_FRAME_STRIDE: u64 = 1000_0000_00 / 120;
        #[cfg(feature = "config_DEFAULT_PHYSICS_FPS_60")]
        pub const DEFAULT_PHYSICS_FRAME_STRIDE: u64 = 1000_0000_00 / 60;
        #[cfg(feature = "config_DEFAULT_PHYSICS_FPS_30")]
        pub const DEFAULT_PHYSICS_FRAME_STRIDE: u64 = 1000_0000_00 / 30;
        #[cfg(feature = "config_DEFAULT_PHYSICS_FPS_UNLIMITED")]
        pub const DEFAULT_PHYSICS_FRAME_STRIDE: u64 = 0;

        #[cfg(feature = "config_DEFAULT_IS_PERFORMANCE_FIRST_true")]
        pub const DEFAULT_IS_PERFORMANCE_FIRST: bool = true;

        #[cfg(feature = "config_DEFAULT_IS_PERFORMANCE_FIRST_false")]
        pub const DEFAULT_IS_PERFORMANCE_FIRST: bool = false;

        pub const DEFAULT_ERROR_COLOR1: [f32; 4] = [1.0, 0.0, 1.0, 0.0];
        pub const DEFAULT_ERROR_COLOR2: [f32; 4] = [0.0, 1.0, 0.0, 0.0];
        pub const DEFAULT_ERROR_COLOR: [vk::ClearValue; 2] = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: DEFAULT_ERROR_COLOR1,
                },
            },
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: DEFAULT_ERROR_COLOR2,
                },
            },
        ];
    }
    pub mod COMMAND {
        use std::ptr::null;

        use ash::vk;

        #[cfg(feature = "config_DEFAULT_SEMAPHORE_COUNT_1")]
        pub const DEFAULT_SEMAPHORE_COUNT: usize = 1;
        pub const INDEX_INIT_IMG_STAGE_SEMAPHORE: usize = 0;
        pub const INDEX_GET_IMG_STAGE_SEMAPHORE: usize = 1;
        pub const INDEX_OUT_IMG_STAGE_SEMAPHORE: usize = 2;

        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_MODE_CUSTOM")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_AUTO: bool = true;
        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_MODE_AUTO")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_AUTO: bool = false;

        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_VERTEX_0")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_VERTEX: u32 = 0;
        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_VERTEX_1")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_VERTEX: u32 = 1;

        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_DRAW_0")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_DRAW: u32 = 0;
        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_DRAW_1")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_DRAW: u32 = 1;

        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE_2")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE: u32 = 2;
        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE_1")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE: u32 = 1;
        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE_0")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_PIPELINE: u32 = 0;

        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_CMDBUF_0")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_CMDBUF: u32 = 0;
        #[cfg(feature = "config_DEFAULT_COMMAND_BUFFER_INDEX_BINDING_CMDBUF_1")]
        pub const DEFAULT_COMMAND_BUFFER_INDEX_BINDING_CMDBUF: u32 = 1;

        pub const DEFAULT_SEMAP_INFO: vk::SemaphoreCreateInfo = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: null(),
            flags: vk::SemaphoreCreateFlags::from_raw(0),
        };
        // #[cfg(feature = "config_DEFAULT_CMD_POOL_TYPE_TRANSIENT")]
        // pub const DEFAULT_CMD_POOL_TYPE: u32 = 0b1;
        // #[cfg(feature = "config_DEFAULT_CMD_POOL_TYPE_RESET")]
        // pub const DEFAULT_CMD_POOL_TYPE: u32 = 0b10;
        // #[cfg(feature = "config_DEFAULT_CMD_POOL_TYPE_PROTECTED")]
        // pub const DEFAULT_CMD_POOL_TYPE: u32 = 0b100;
    }

    pub mod API_BUFFER {

        #[cfg(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_DEVICE_LOCAL_BIT")]
        pub const SWITCH_HOST_LOCAL: u32 = 0b1;
        #[cfg(not(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_DEVICE_LOCAL_BIT"))]
        pub const SWITCH_HOST_LOCAL: u32 = 0;

        #[cfg(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_HOST_VISIBLE_BIT")]
        pub const SWITCH_HOST_VISIBLE: u32 = 0b10;
        #[cfg(not(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_HOST_VISIBLE_BIT"))]
        pub const SWITCH_HOST_VISIBLE: u32 = 0;

        #[cfg(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_HOST_COHERENT_BIT")]
        pub const SWITCH_HOST_COHERENT: u32 = 0b100;
        #[cfg(not(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_HOST_COHERENT_BIT"))]
        pub const SWITCH_HOST_COHERENT: u32 = 0;

        #[cfg(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_HOST_CACHED_BIT")]
        pub const SWITCH_HOST_CACHED: u32 = 0b1000;
        #[cfg(not(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_HOST_CACHED_BIT"))]
        pub const SWITCH_HOST_CACHED: u32 = 0;

        #[cfg(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_LAZILY_ALLOCATED_BIT")]
        pub const SWITCH_LAZILY_ALLOCATED: u32 = 0b10000;
        #[cfg(not(feature = "config_DEFAULT_MEM_PROPERTY_TYPE_LAZILY_ALLOCATED_BIT"))]
        pub const SWITCH_LAZILY_ALLOCATED: u32 = 0;

        pub const DEFAULT_MEMORY_PROPERTY: u32 = SWITCH_HOST_LOCAL
            | SWITCH_HOST_VISIBLE
            | SWITCH_HOST_COHERENT
            | SWITCH_HOST_CACHED
            | SWITCH_LAZILY_ALLOCATED;
    }

    pub mod RECT {
        use ash::vk::{self, Extent2D, Offset2D};

        pub const DEFAULT_RECT2D: vk::Rect2D = vk::Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: Extent2D {
                width: { crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH },
                height: { crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT },
            },
        };
    }

    //call it before in loop
    #[allow(unused)]
    pub mod IMG_FORMAT {
        use std::ptr::null;

        use ash::vk;
        pub const DEFAULT_DEPTH_IMG: vk::ImageCreateInfo = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageCreateFlags::TYPE_2D_VIEW_COMPATIBLE_EXT,
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::D16_UNORM,
            extent: vk::Extent3D {
                width: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH
                },
                height: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT
                },
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            sharing_mode: vk::SharingMode::EXCLUSIVE, //set up it sync the device you cllocate from
            queue_family_index_count: 0,
            p_queue_family_indices: &0,
            initial_layout: vk::ImageLayout::PREINITIALIZED,
        };

        pub const DEFAULT_COLOR_IMG: vk::ImageCreateInfo = vk::ImageCreateInfo {
            s_type: vk::StructureType::IMAGE_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageCreateFlags::TYPE_2D_VIEW_COMPATIBLE_EXT,
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::R8G8B8A8_UNORM,
            extent: vk::Extent3D {
                width: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH
                },
                height: unsafe {
                    crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT
                },
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            sharing_mode: vk::SharingMode::EXCLUSIVE, //set up it sync the device you cllocate from
            queue_family_index_count: 0,
            p_queue_family_indices: &0,
            initial_layout: vk::ImageLayout::PREINITIALIZED,
        };
    }
    #[allow(unused)]
    pub mod IMG2VIEW {
        use std::ptr::{null, null_mut};

        use ash::vk;
        pub const DEFAULT_DEPTH: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image: vk::Image::null(), //set up it sync the img you bind
            view_type: vk::ImageViewType::TYPE_2D,
            format: vk::Format::from_raw(0), //set up it sync the img you bind
            components: vk::ComponentMapping {
                r: vk::ComponentSwizzle::R,
                g: vk::ComponentSwizzle::G,
                b: vk::ComponentSwizzle::B,
                a: vk::ComponentSwizzle::A,
            },
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        pub const DEFAULT_COLOR: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image: vk::Image::null(), //set up it sync the img you bind
            view_type: vk::ImageViewType::TYPE_2D,
            format: vk::Format::from_raw(0), //set up it sync the img you bind
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
    }

    #[allow(unused)]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
    pub mod PSO {
        use std::ptr::null;

        use crate::renderer::cfg::env::RECT::DEFAULT_RECT2D;

        use ash::vk::{
            self, DescriptorSetLayout, Extent2D, Offset2D, PipelineInputAssemblyStateCreateFlags,
            PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo,
            PipelineTessellationStateCreateFlags, PipelineTessellationStateCreateInfo,
            PipelineVertexInputStateCreateFlags, PipelineVertexInputStateCreateInfo,
            PipelineViewportStateCreateFlags, PipelineViewportStateCreateInfo, PrimitiveTopology,
            Rect2D, VertexInputAttributeDescription, VertexInputBindingDescription, Viewport,
        };

        //
        pub const INDEX_ATTACHMENT_DEFAULT: usize = 0;
        pub const INDEX_ATTACHMENT_COLOR_PASS: usize = 0;
        pub const INDEX_ATTACHMENT_DEPTH_PASS: usize = 1;
        pub const INDEX_ATTACHMENT_STENCIL_PASS: usize = 2;

        //
        pub const DEFAULT_ATTACHMENT_VEC: [vk::AttachmentDescription; 2] = [
            DEFAULT_ATTACHMENT_PRESENT_SURF,
            DEFAULT_ATTACHMENT_DEPTH_PASS,
            // DEFAULT_ATTACHMENT_STENCIL_PASS,
        ];

        pub const DEFAULT_ATTACHMENT: vk::AttachmentDescription = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: vk::Format::R8G8B8A8_UNORM,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::LOAD,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::LOAD,
            stencil_store_op: vk::AttachmentStoreOp::STORE,
            initial_layout: vk::ImageLayout::GENERAL,
            final_layout: vk::ImageLayout::GENERAL,
        };

        pub const DEFAULT_ATTACHMENT_DEPTH_PASS: vk::AttachmentDescription =
            vk::AttachmentDescription {
                flags: vk::AttachmentDescriptionFlags::empty(),
                format: vk::Format::D16_UNORM,
                //format: vk::Format::R8G8B8A8_UNORM,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::LOAD,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::LOAD,
                stencil_store_op: vk::AttachmentStoreOp::STORE,
                initial_layout: vk::ImageLayout::PREINITIALIZED,
                final_layout: vk::ImageLayout::GENERAL,
            };

        pub const DEFAULT_ATTACHMENT_STENCIL_PASS: vk::AttachmentDescription =
            vk::AttachmentDescription {
                flags: vk::AttachmentDescriptionFlags::empty(),
                format: vk::Format::D16_UNORM,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::LOAD,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::LOAD,
                stencil_store_op: vk::AttachmentStoreOp::STORE,
                initial_layout: vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL,
                final_layout: vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL,
            };

        pub const DEFAULT_ATTACHMENT_COLOR_PASS: vk::AttachmentDescription =
            vk::AttachmentDescription {
                flags: vk::AttachmentDescriptionFlags::empty(),
                format: vk::Format::R8G8B8A8_UNORM,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::LOAD,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::LOAD,
                stencil_store_op: vk::AttachmentStoreOp::STORE,
                initial_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            };

        pub const DEFAULT_ATTACHMENT_PRESENT_SURF: vk::AttachmentDescription =
            vk::AttachmentDescription {
                flags: vk::AttachmentDescriptionFlags::empty(),
                format: vk::Format::R8G8B8A8_UNORM,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::LOAD,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::LOAD,
                stencil_store_op: vk::AttachmentStoreOp::STORE,
                initial_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            };

        // 
        pub const DEFAULT_ATTACHEMENT_REF_COLOR_PASS_VEC: [vk::AttachmentReference; 1] =
            [DEFAULT_ATTACHEMENT_REF_COLOR_PASS];
        pub const DEFAULT_ATTACHEMENT_REF_INPUT_PASS_VEC: [vk::AttachmentReference; 1] =
            [DEFAULT_ATTACHEMENT_REF_COLOR_PASS];

        pub const DEFAULT_ATTACHEMENT_REF: vk::AttachmentReference = vk::AttachmentReference {
            attachment: INDEX_ATTACHMENT_DEFAULT as u32,
            layout: vk::ImageLayout::GENERAL,
        };

        pub const DEFAULT_ATTACHEMENT_REF_INPUT_PASS: vk::AttachmentReference =
            vk::AttachmentReference {
                attachment: INDEX_ATTACHMENT_COLOR_PASS as u32,
                layout: vk::ImageLayout::GENERAL,
            };

        pub const DEFAULT_ATTACHEMENT_REF_COLOR_PASS: vk::AttachmentReference =
            vk::AttachmentReference {
                attachment: INDEX_ATTACHMENT_COLOR_PASS as u32,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            };

        pub const DEFAULT_ATTACHEMENT_REF_DEPTH_PASS: vk::AttachmentReference =
            vk::AttachmentReference {
                attachment: INDEX_ATTACHMENT_DEPTH_PASS as u32,
                layout: vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
            };

        pub const DEFAULT_ATTACHEMENT_REF_STENCIL_PASS: vk::AttachmentReference =
            vk::AttachmentReference {
                attachment: INDEX_ATTACHMENT_STENCIL_PASS as u32,
                layout: vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL,
            };

        ///
        pub const DEFAULT_VAD_4X4_RGBA16F: VertexInputAttributeDescription =
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R16G16B16A16_SFLOAT,
                offset: 0,
            };

        pub const DEFAULT_VAD_4X4_RGBA32F: VertexInputAttributeDescription =
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: 0,
            };

        pub const DEFAULT_VAD_4X4_RGBA64F: VertexInputAttributeDescription =
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R64G64B64A64_SFLOAT,
                offset: 0,
            };

        pub const DEFAULT_VBD: VertexInputBindingDescription = VertexInputBindingDescription {
            binding: 0,
            stride: 4 * 4 * 64, //unit:[vertex,uv,normal,bitangent] size: 4*4*64
            input_rate: vk::VertexInputRate::VERTEX, // default no going to use GPU instance
        };

        pub const DEFAULT_VAD: VertexInputAttributeDescription = DEFAULT_VAD_4X4_RGBA64F;

        /// 如果渲染管线有定义 则优先使用渲染管线中视窗
        pub const DEFAULT_VIEWPORT: Viewport = Viewport {
            x: 0.0,
            y: 0.0,
            width: unsafe {
                crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH as f32
            },
            height: unsafe {
                crate::workarea::resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT as f32
            },
            min_depth: 0.0,
            max_depth: 1.0,
        };

        /// 如果渲染管线有定义 则优先使用渲染管线中裁剪
        pub const DEFAULT_SCISSROS: Rect2D = DEFAULT_RECT2D;

        pub const DEFAULT_VERTEX_INPUT_STATE: PipelineVertexInputStateCreateInfo =
            PipelineVertexInputStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineVertexInputStateCreateFlags::empty(),
                vertex_binding_description_count: 1,
                p_vertex_binding_descriptions: &DEFAULT_VBD,
                vertex_attribute_description_count: 1,
                p_vertex_attribute_descriptions: &DEFAULT_VAD_4X4_RGBA64F,
            };

        /// 默认 顺时针三角形配装图元
        /// 每三个顶点对应专门的三角形图元
        /// 不会压缩 不会重复使用
        /// 默认不开启GPU实例化
        pub const DEFAULT_INPUT_ASSEMBLY_STATE: PipelineInputAssemblyStateCreateInfo =
            PipelineInputAssemblyStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineInputAssemblyStateCreateFlags::empty(),
                topology: PrimitiveTopology::TRIANGLE_LIST,
                primitive_restart_enable: vk::FALSE,
            };

        /// 默认 不开启细分着色器
        /// 注： vk API原则上控制点必须大于0
        /// 程序上将
        /// 控制点小于0指定为不开启细分着色
        /// 控制点为MAX指定为依据选定GPU匹配最大细分控制点
        pub const DEFAULT_TESSELLATION_STATE: PipelineTessellationStateCreateInfo =
            PipelineTessellationStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineTessellationStateCreateFlags::empty(),
                patch_control_points: 0,
            };

        /// 默认与配置一致 不可更改
        /// 需要动态配置 请于配装器中构造
        pub const DEFAULT_VIEWPORT_STATE: PipelineViewportStateCreateInfo =
            PipelineViewportStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
                p_next: null(),
                flags: PipelineViewportStateCreateFlags::empty(),
                viewport_count: 1,
                p_viewports: &DEFAULT_VIEWPORT,
                scissor_count: 1,
                p_scissors: &DEFAULT_SCISSROS,
            };

        /// 默认 关闭抗锯齿
        /// 建议 在管线执行器中动态设置
        pub const DEFAULT_MULTISAMPLE_STATE: vk::PipelineMultisampleStateCreateInfo =
            vk::PipelineMultisampleStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineMultisampleStateCreateFlags::empty(),
                rasterization_samples: vk::SampleCountFlags::TYPE_1,
                sample_shading_enable: vk::FALSE,
                min_sample_shading: 0.0,
                p_sample_mask: null(),
                alpha_to_coverage_enable: vk::FALSE,
                alpha_to_one_enable: vk::FALSE,
            };

        /// 默认丢弃视窗锥体外片元 (包括远平面之外)
        /// 默认绘制模式为 填充
        /// 默认 背面渲染片元丢弃
        /// 默认 正面片元符合左手定则
        /// 默认关闭 斜率深度偏置 （用于避免阴影重影）
        pub const DEFAULT_RASTERIZATION_STATE: vk::PipelineRasterizationStateCreateInfo =
            vk::PipelineRasterizationStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineRasterizationStateCreateFlags::empty(),
                depth_clamp_enable: vk::FALSE,
                rasterizer_discard_enable: vk::FALSE,
                polygon_mode: vk::PolygonMode::FILL,
                cull_mode: vk::CullModeFlags::BACK,
                front_face: vk::FrontFace::COUNTER_CLOCKWISE,
                depth_bias_enable: vk::FALSE,
                depth_bias_constant_factor: 0.0,
                depth_bias_clamp: 0.0,
                depth_bias_slope_factor: 0.0,
                line_width: 1.0,
            };

        pub const DEFAULT_DEPTH_STENCIL_STATE: vk::PipelineDepthStencilStateCreateInfo =
            vk::PipelineDepthStencilStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineDepthStencilStateCreateFlags::empty(),
                depth_test_enable: vk::TRUE,
                depth_write_enable: vk::TRUE,
                depth_compare_op: vk::CompareOp::LESS,
                depth_bounds_test_enable: vk::FALSE,
                stencil_test_enable: vk::TRUE,
                front: vk::StencilOpState {
                    fail_op: vk::StencilOp::ZERO,
                    pass_op: vk::StencilOp::KEEP,
                    depth_fail_op: vk::StencilOp::ZERO,
                    compare_op: vk::CompareOp::GREATER_OR_EQUAL,
                    compare_mask: 1,
                    write_mask: 1,
                    reference: 1,
                },
                back: vk::StencilOpState {
                    fail_op: vk::StencilOp::KEEP,
                    pass_op: vk::StencilOp::KEEP,
                    depth_fail_op: vk::StencilOp::KEEP,
                    compare_op: vk::CompareOp::ALWAYS,
                    compare_mask: 0,
                    write_mask: 0,
                    reference: 0,
                },
                min_depth_bounds: 0.0,
                max_depth_bounds: 0.0,
            };

        /// 默认关闭颜色混合 渲染模式为不透明
        pub const DEFAULT_COLOR_BLEND_STATE: vk::PipelineColorBlendStateCreateInfo =
            vk::PipelineColorBlendStateCreateInfo {
                s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
                p_next: null(),
                flags: vk::PipelineColorBlendStateCreateFlags::empty(),
                logic_op_enable: vk::FALSE,
                logic_op: vk::LogicOp::AND,
                attachment_count: 0,
                p_attachments: null(),
                blend_constants: [0.0, 0.0, 0.0, 0.0],
            };

        /// 默认仅仅传入 translate数据
        ///
        pub const DEFAULT_LAYOUT: PipelineLayoutCreateInfo = PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: 0,
            p_set_layouts: null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: null(),
        };

        pub const DEFAULT_SUBPASS_DESCRIPTION_VEC: [vk::SubpassDescription; 1] =
            [DEFAULT_SUBPASS_DESCRIPTION];

        //
        pub const DEFAULT_SUBPASS_DESCRIPTION: vk::SubpassDescription = vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: null(),
            color_attachment_count: DEFAULT_ATTACHEMENT_REF_COLOR_PASS_VEC.len() as u32,
            p_color_attachments: DEFAULT_ATTACHEMENT_REF_COLOR_PASS_VEC.as_ptr(),
            p_resolve_attachments: null(),
            p_depth_stencil_attachment: &DEFAULT_ATTACHEMENT_REF_DEPTH_PASS,
            preserve_attachment_count: 0,
            p_preserve_attachments: null(),
        };

        pub const DEFAULT_SUBPASS_DEPENDENCY_VEC: [vk::SubpassDependency; 1] =
            [DEFAULT_SUBPASS_DEPENDENCY];

        pub const DEFAULT_PASS_DST_MASK: u32 = vk::AccessFlags::COLOR_ATTACHMENT_READ.as_raw()
            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE.as_raw();
        pub const DEFAULT_PASS_SRC_MASK: u32 = vk::AccessFlags::empty().as_raw();
        //let
        pub const DEFAULT_SUBPASS_DEPENDENCY: vk::SubpassDependency = vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::from_raw(DEFAULT_PASS_SRC_MASK),
            dst_access_mask: vk::AccessFlags::from_raw(DEFAULT_PASS_DST_MASK),
            dependency_flags: vk::DependencyFlags::empty(),
        };

        //
        pub const DEFAULT_RENDER_SURF_PASS: vk::RenderPassCreateInfo = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: null(),
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: DEFAULT_ATTACHMENT_VEC.len() as u32,
            p_attachments: DEFAULT_ATTACHMENT_VEC.as_ptr(),
            subpass_count: DEFAULT_SUBPASS_DESCRIPTION_VEC.len() as u32,
            p_subpasses: DEFAULT_SUBPASS_DESCRIPTION_VEC.as_ptr(),
            dependency_count: DEFAULT_SUBPASS_DEPENDENCY_VEC.len() as u32,
            p_dependencies: DEFAULT_SUBPASS_DEPENDENCY_VEC.as_ptr(),
        };
    }
}

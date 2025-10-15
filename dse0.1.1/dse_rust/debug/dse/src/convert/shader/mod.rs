#[cfg(feature = "graphic_api_vulkan_1_3")]
pub static SHADER_SPRIV_VERSION: shaderc::SpirvVersion = shaderc::SpirvVersion::V1_0;

#[cfg(feature = "config_SOURCE_SHADER_TYPE_GLSL")]
pub static SOURCE_SHADER_TYPE: shaderc::SourceLanguage = shaderc::SourceLanguage::GLSL;

#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {

    use crate::manager::datum::env::Datum;
    use crate::manager::execute::env::TaskQueue;
    use crate::shader::env::ShaderTextD;
    use ash::vk;
    use shaderc::{self, CompileOptions, Compiler};

    use super::SHADER_SPRIV_VERSION;
    use super::SOURCE_SHADER_TYPE;

    #[derive(Clone)]
    pub enum ShaderDecoderTask {
        None,
    }

    pub struct ShaderDecoderE {
        pub id: u64,
        pub attachment: ShaderDecoderAttachment,
    }

    pub struct ShaderDecoderAttachment {}

    pub struct ShaderDecodeResult {
        id: u64,
        stage: vk::ShaderStageFlags,
        raw: String,
    }

    impl Default for ShaderDecodeResult {
        fn default() -> Self {
            Self {
                id: Default::default(),
                stage: Default::default(),
                raw: Default::default(),
            }
        }
    }

    #[derive(Default, Clone, Debug)]
    pub struct ShaderResult<T>
    where
        T: Clone,
    {
        stage: vk::ShaderStageFlags,
        source: T,
    }

    impl<T> ShaderResult<T>
    where
        T: Clone,
    {
        pub fn get_source(&self) -> T {
            return self.source.clone();
        }
        pub fn get_stage(&self) -> vk::ShaderStageFlags {
            return self.stage;
        }
        pub fn build(tin: T) -> Self {
            return Self {
                stage: Default::default(),
                source: tin,
            };
        }

        pub fn build_stage(mut self, sin: vk::ShaderStageFlags) -> Self {
            self.stage = sin;
            return self;
        }
    }

    impl ShaderDecodeResult {
        fn _shader_stage_flags2kind(fin: &vk::ShaderStageFlags) -> shaderc::ShaderKind {
            match *fin {
                vk::ShaderStageFlags::VERTEX => shaderc::ShaderKind::Vertex,
                vk::ShaderStageFlags::FRAGMENT => shaderc::ShaderKind::Fragment,
                vk::ShaderStageFlags::TESSELLATION_CONTROL => shaderc::ShaderKind::TessControl,
                vk::ShaderStageFlags::TESSELLATION_EVALUATION => {
                    shaderc::ShaderKind::TessEvaluation
                }
                vk::ShaderStageFlags::GEOMETRY => shaderc::ShaderKind::Geometry,
                vk::ShaderStageFlags::COMPUTE => shaderc::ShaderKind::Compute,
                _ => todo!(),
            }
        }

        pub fn decode_to_binary_u8(self) -> Result<Vec<u8>, bool> {
            let _compiler = Compiler::new().unwrap();

            let mut _options = CompileOptions::new().unwrap();
            _options.set_target_spirv(SHADER_SPRIV_VERSION);
            _options.set_source_language(SOURCE_SHADER_TYPE);

            let mut _r = _compiler.compile_into_spirv(
                &self.raw,
                Self::_shader_stage_flags2kind(&self.stage),
                &self.id.to_string(),
                "main",
                Some(&_options),
            );
            return Ok(_r.as_mut().unwrap().as_binary_u8().to_vec());
        }

        pub fn decode_to_binary_u32(self) -> Result<ShaderResult<Vec<u32>>, bool> {
            let _compiler = Compiler::new().unwrap();

            let mut _options = CompileOptions::new().unwrap();
            _options.set_target_spirv(SHADER_SPRIV_VERSION);
            _options.set_source_language(SOURCE_SHADER_TYPE);

            let mut _r = ShaderResult::build(
                _compiler
                    .compile_into_spirv(
                        &self.raw,
                        Self::_shader_stage_flags2kind(&self.stage),
                        &self.id.to_string(),
                        crate::shader::DEFAULT_SHADER_ENTRY_NAME,
                        Some(&_options),
                    )
                    .as_mut()
                    .unwrap()
                    .as_binary()
                    .to_vec(),
            )
            .build_stage(self.stage);
            return Ok(_r);
        }

        pub fn decode_to_str(self) -> Result<String, bool> {
            let compiler = Compiler::new().unwrap();
            let mut _options = CompileOptions::new().unwrap();
            _options.set_target_spirv(SHADER_SPRIV_VERSION);
            _options.set_source_language(SOURCE_SHADER_TYPE);
            let mut _r = compiler
                .compile_into_spirv_assembly(
                    &self.raw,
                    Self::_shader_stage_flags2kind(&self.stage),
                    &self.id.to_string(),
                    "main",
                    Some(&_options),
                )
                .as_mut()
                .unwrap()
                .as_text();
            return Ok(_r);
        }
    }

    impl Default for ShaderDecoderAttachment {
        fn default() -> Self {
            Self {}
        }
    }

    impl ShaderDecoderE {
        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }
        pub fn builder() -> Self {
            Self {
                id: 0,
                attachment: Default::default(),
            }
        }
        pub fn build_option(mut self, oin: ShaderDecoderAttachment) -> Self {
            self.attachment = oin;
            return self;
        }

        pub fn decode_sync(&mut self, stin: &mut ShaderTextD) -> ShaderDecodeResult {
            return ShaderDecodeResult {
                id: stin.get_id(),
                raw: stin.get_source(),
                stage: stin.get_stage(),
            };
        }

        fn _call_back_decode() {}

        pub fn exe_decode(
            &mut self,
            datum: &mut Datum<ShaderTextD>,
            tin: &mut Datum<TaskQueue<ShaderDecoderTask>>,
        ) {
            tin.back_mut().unwrap().begin_execute();
            for ti in tin.back_mut().unwrap().task_iter_ref().unwrap() {}
            tin.back_mut().unwrap().end_execute();
        }
    }

    impl Default for ShaderDecoderE {
        fn default() -> Self {
            Self {
                id: Default::default(),
                attachment: Default::default(),
            }
        }
    }

    impl Default for ShaderDecoderTask {
        fn default() -> Self {
            return Self::None;
        }
    }
}

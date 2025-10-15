pub mod edge;
pub mod face;
pub mod vertex;

#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_os_win10")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
pub mod env {
    use std::mem;

    use ash::vk;
    use glam::DVec4;
    use std::fmt::Debug;

    use crate::get;

    #[repr(C, align(8))]
    #[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT")]
    #[cfg(feature = "env_bit_64bit")]
    pub struct MeshD {
        id: u64,
        flag: MeshActiveFlag,
        buffer: Vec<f64>,
    }

    impl Debug for MeshD {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unsafe {
                f.debug_struct("MeshD")
                    .field("id", &self.id)
                    .field("flag", &self.flag.flag)
                    .field("buffer", &self.buffer)
                    .finish()
            }
        }
    }

    #[cfg(feature = "env_bit_64bit")]
    pub union MeshActiveFlag {
        flag: u64,
        part: [u8; 8],
        bit: [bool; 64],
    }

    #[cfg(feature = "env_bit_64bit")]
    impl Default for MeshActiveFlag {
        fn default() -> Self {
            return unsafe { mem::zeroed() };
        }
    }

    #[cfg(feature = "env_bit_64bit")]
    impl MeshActiveFlag {
        //0b|00000000|*00000000*|000000000000000000000000000000000000000000000000
        //0b|*offset*|custom num|************inherent*index**********************|
        pub const NONE: u64 = /**********************************************/ 0b0; /* offset:0 */
        pub const VERTEX: u64 = /********************************************/ 0b1; /* offset:4 */
        pub const NORMAL: u64 = /*******************************************/ 0b10; /* offset:4 */
        pub const BITANGENT: u64 = /****************************************/ 0b100; /* offset:4 */
        pub const UV: u64 = /*********************************************/ 0b1000; /* offset:2 */
        pub const UVW: u64 = /*******************************************/ 0b10000; /* offset:3 */
        pub const WEIGHT: u64 = /***************************************/ 0b100000; /* offset:1 */
        pub const CLUSTER: u64 = /*************************************/ 0b1000000; /* offset:1 */
        pub const ELEMENT: u64 = /************************************/ 0b10000000; /* offset:1 */
        pub const ALL: u64 = Self::CLUSTER
            | Self::ELEMENT
            | Self::NONE
            | Self::NORMAL
            | Self::BITANGENT
            | Self::UVW
            | Self::VERTEX
            | Self::WEIGHT;
        pub const OPTION2D: u64 = Self::VERTEX | Self::UV;

        pub fn include_array(&self, fin: u64) -> bool {
            return unsafe { fin & self.flag != 0 };
        }

        pub fn update_offset(&mut self) {
            todo!()
        }

        pub fn offset(&self) -> u64 {
            return unsafe { *get!(self.part, 7) as u64 };
        }
    }

    impl MeshD {
        pub fn build() -> Self {
            Self {
                id: Default::default(),
                buffer: Default::default(),
                flag: Default::default(),
            }
        }

        pub fn build_id(mut self, id_in: u64) -> Self {
            self.id = id_in;
            return self;
        }

        pub fn build_active_flag(mut self, flag_in: u64) -> Self {
            unsafe { self.flag.flag = self.flag.flag | flag_in };
            return self;
        }

        pub fn build_update_offset(mut self) -> Self {
            todo!();
            return self;
        }

        pub fn build_update_vertex_count() {
            todo!();
        }

        pub fn build_default_2D_spirit(mut self) -> Self {
            self = self.build_active_flag(MeshActiveFlag::OPTION2D);

            //v0
            self.buffer.append(&mut vec![1.0, 1.0, 0.0, 0.0]); //vertex_buffer
            self.buffer.append(&mut vec![1.0, 1.0, 0.0, 0.0]);
            //v1
            self.buffer.append(&mut vec![-1.0, 1.0, 0.0, 0.0]);
            self.buffer.append(&mut vec![0.0, 1.0, 0.0, 0.0]);
            //v2
            self.buffer.append(&mut vec![1.0, -1.0, 0.0, 0.0]);
            self.buffer.append(&mut vec![1.0, 0.0, 0.0, 0.0]);
            //v3
            self.buffer.append(&mut vec![-1.0, -1.0, 0.0, 0.0]);
            self.buffer.append(&mut vec![0.0, 0.0, 0.0, 0.0]);

            return self;
        }

        pub fn bind_description_ref(&self) -> Result<vk::VertexInputBindingDescription, ()> {
            todo!();
            Err(())
        }

        pub fn buffer_mem_size(&self) -> u64 {
            return (self.buffer.len() * std::mem::size_of::<f64>()) as u64;
        }

        pub fn buffer_unit_offset(&self) -> u64 {
            return std::mem::size_of::<f64>() as u64;
        }

        pub fn buffer_ref(&self) -> &[f64] {
            return &self.buffer;
        }
    }

    impl Default for MeshD {
        fn default() -> Self {
            Self {
                id: Default::default(),
                buffer: Default::default(),
                flag: Default::default(),
            }
        }
    }
}

#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::fmt::Debug;

    #[repr(C, align(4))]
    #[derive(Default,Debug)]
    pub struct Rect {
        pub buffer: glam::U64Vec4,
        pub parent_buffer: glam::U64Vec4,
    }

    impl Rect {
        pub fn build() -> Self {
            return Default::default();
        }

        pub fn update_parent(&mut self, vin: glam::U64Vec4) {
            self.parent_buffer = vin;
        }

        pub fn height(&self, mode: u64) -> Result<u64, ()> {
            match mode {
                RectMode::DEFAULT => {
                    return Ok(self.buffer.w - self.buffer.y);
                }
                RectMode::CONFIG1 => {
                    return Ok(2 * self.buffer.w);
                }
                _ => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_ERROR
                            | crate::log::code::CONDI_INVAILD_PARAMETER_INPUT
                            | crate::log::code::FILE_MODEL_RECTANGLE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ));
                }
            }
        }

        

        pub fn weight(&self, mode: u64) -> Result<u64, ()> {
            match mode {
                RectMode::DEFAULT => {
                    return Ok(self.buffer.z - self.buffer.x);
                    //return Ok(1);
                }
                RectMode::CONFIG1 => {
                    return Ok(2 * self.buffer.z);
                }
                _ => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_ERROR
                            | crate::log::code::CONDI_INVAILD_PARAMETER_INPUT
                            | crate::log::code::FILE_MODEL_RECTANGLE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ));
                }
            }
        }
    }

    #[allow(unused)]
    pub struct RectMode {}

    impl RectMode {
        pub const DEFAULT: u64 = Self::FIXED | Self::CENTER | Self::PIXEL;

        pub const CONFIG1: u64 = RectMode::FIXED | RectMode::CENTER | RectMode::ANCHOR;
        //
        // 固定模式：默认全局矩形中位置(无父矩形)
        // 比例模式：依据提供的父矩形确定全局矩形 单位：1~0.1%
        pub const FIXED: u64 = 0x1;
        pub const PROPORTION: u64 = 0x2;
        //
        // 中心模式：由原点(锚点)到自身边框距离
        // 边距模式：从自身边框到父边框距离 数据结构为左下右上：xyzw
        pub const CENTER: u64 = 0x10;
        pub const MARGIN: u64 = 0x20;
        //
        // 像素模式：左下角向量，右上角向量
        // 锚点模式：锚点向量，边框半宽,边框半高
        pub const PIXEL: u64 = 0x100;
        pub const ANCHOR: u64 = 0x200;
    }
}

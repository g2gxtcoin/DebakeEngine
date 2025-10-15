mod env {
    use ash::vk::{Extent2D, Offset2D};

    use crate::model::rectangle::env::Rect;
    impl From<ash::vk::Rect2D> for Rect {
        fn from(value: ash::vk::Rect2D) -> Self {
            return Self {
                buffer: glam::U64Vec4 {
                    x: value.offset.x as u64,
                    y: value.offset.y as u64,
                    z: (value.offset.x as u32 + value.extent.width) as u64,
                    w: (value.offset.y as u32 + value.extent.height) as u64,
                },
                parent_buffer: glam::U64Vec4::ZERO,
            };
        }
    }

    impl Into<ash::vk::Rect2D> for Rect {
        fn into(self) -> ash::vk::Rect2D {
            let _r = self.buffer - self.parent_buffer;
            return ash::vk::Rect2D {
                offset: Offset2D {
                    x: i32::try_from(_r.x).unwrap(),
                    y: i32::try_from(_r.y).unwrap(),
                },
                extent: Extent2D {
                    width: u32::try_from(_r.z - _r.x).unwrap(),
                    height: u32::try_from(_r.w - _r.y).unwrap(),
                },
            };
        }
    }
}

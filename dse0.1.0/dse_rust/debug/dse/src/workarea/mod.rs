use std::ops::Mul;

use glam::Vec4Swizzles;

use crate::{
    get,
    model::rectangle::env::{Rect, RectMode},
    tool::stop_point,
};

#[cfg(feature = "env_os_win")]
pub mod win;

pub trait TScreenScaling {
    fn mul_physical_scaling(self) -> Self;
    fn mul_virtual_scaling(self) -> Self;
}

pub mod resolution_default {
    #[cfg(feature = "resolution_full")]
    pub const WINDOW_DEFAULT_RESOLUTION_WIDTH: u32 = u32::MIN;
    #[cfg(feature = "resolution_full")]
    pub const WINDOW_DEFAULT_RESOLUTION_HEIGHT: u32 = u32::MIN;

    #[cfg(feature = "resolution_width_1600")]
    pub const WINDOW_DEFAULT_RESOLUTION_WIDTH: u32 = 1600u32;
    #[cfg(feature = "resolution_height_900")]
    pub const WINDOW_DEFAULT_RESOLUTION_HEIGHT: u32 = 900u32;

    #[cfg(feature = "resolution_width_640")]
    pub const WINDOW_DEFAULT_RESOLUTION_WIDTH: u32 = 640u32;
    #[cfg(feature = "resolution_height_480")]
    pub const WINDOW_DEFAULT_RESOLUTION_HEIGHT: u32 = 480u32;
}

// 应用程序主窗口 Rect
// 渲染区域与  应用程序主窗口 Rect 一致
// 循序： 宽度 高度
// 以屏幕左下角为 原点
// [左下角向量，右上角向量]
// 父矩框为物理屏幕像素

pub static mut DEFAULT_WORKAREA_RECT: Rect = Rect {
    buffer: glam::U64Vec4::new(
        0,
        0,
        resolution_default::WINDOW_DEFAULT_RESOLUTION_WIDTH as u64,
        resolution_default::WINDOW_DEFAULT_RESOLUTION_HEIGHT as u64,
    ),
    parent_buffer: glam::U64Vec4::ZERO, // phyics_screen_rect
};

pub static mut WORKAREA_OFFSET_X: u64 = 0u64;
pub static mut WORKAREA_OFFSET_Y: u64 = 0u64;

// 缩放因子 单位:%
pub static mut SCREEN_SCALING_FACTOR: u64 = 0u64;
pub static mut SCREEN_REFLESH_FREQUENCY: u64 = 0u64;

pub static mut WORKAREA_CLOSE: bool = false;

//物理分辨率 -> 虚拟分辨率
#[allow(non_snake_case)]
pub fn MUL_VIRTUAL_SCALING(lhs: u64) -> u64 {
    return unsafe { ((lhs as f64) * 100.0 / (SCREEN_SCALING_FACTOR as f64)).ceil() as u64 };
}

//虚拟分辨率 -> 物理分辨率
#[allow(non_snake_case)]
pub fn MUL_PHYSICAL_SCALING(lhs: u64) -> u64 {
    return unsafe { ((lhs as f64) * (SCREEN_SCALING_FACTOR as f64) / 100.0).ceil() as u64 };
}

#[allow(non_snake_case)]
pub fn VIRTUAL_WORKAREA_RECT() -> glam::U64Vec4 {
    return unsafe {
        (DEFAULT_WORKAREA_RECT.buffer.as_dvec4() * 100.0 / (SCREEN_SCALING_FACTOR as f64))
            .ceil()
            .as_u64vec4()
    };
}

#[allow(non_snake_case)]
pub fn VIRTUAL_SCREEN_RECT() -> glam::U64Vec4 {
    return unsafe {
        (DEFAULT_WORKAREA_RECT.parent_buffer.as_dvec4() * 100.0 / (SCREEN_SCALING_FACTOR as f64))
            .ceil()
            .as_u64vec4()
    };
}

#[allow(non_snake_case)]
pub fn UPDATE_SCREEN_RECT(virtual_screen_rect: glam::U64Vec4, physical_screen_rect: glam::U64Vec4) {
    unsafe {
        SCREEN_SCALING_FACTOR = (((physical_screen_rect.zw() - physical_screen_rect.xy())
            .length_squared() as f64)
            .powf(0.5)
            / ((virtual_screen_rect.zw() - virtual_screen_rect.xy()).length_squared() as f64)
                .powf(0.5))
        .mul(100.0)
        .ceil() as u64;
        DEFAULT_WORKAREA_RECT.parent_buffer = physical_screen_rect;

        // crate::dev_dbg!(&WINDOW_RECT);
        // crate::dev_dbg!(&SCREEN_SCALING_FACTOR);
        // crate::dev_dbg!(VIRTUAL_SCREEN_RECT());
        // crate::dev_dbg!(VIRTUAL_WINDOW_RECT());
    }
}

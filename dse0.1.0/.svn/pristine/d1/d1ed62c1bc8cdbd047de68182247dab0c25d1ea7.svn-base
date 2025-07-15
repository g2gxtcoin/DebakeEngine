use std::ptr::null;

#[cfg(feature = "env_os_win")]
pub mod win;

#[cfg(feature = "config_INPUT_UNIT_BUFFER_LEN_4")]
pub const UNIFORM_LEN: usize = 4 + 4;

#[cfg(feature = "config_INPUT_UNIT_BUFFER_LEN_0")]
pub const UNIFORM_LEN: usize = 4 + 0;

#[cfg(feature = "config_UNIFORM_INPUT_BUFFER_CLEAR_COUNT_PER_FRAME_0")]
pub const MAX_CLEAR_COUNT: usize = 0;

#[cfg(feature = "config_INPUT_BUFFER_DIM_NUM_4")]
pub const DIM_NUM: usize = 4;

pub static mut CURSOR_LEFT_DOWN: bool = false;
pub static mut CURSOR_RIGHT_DOWN: bool = false;
pub static mut CURSOR_MID_DOWN: bool = false;
pub static mut CURSOR_LEFT_UP: bool = false;
pub static mut CURSOR_RIGHT_UP: bool = false;
pub static mut CURSOR_MID_UP: bool = false;

pub static mut CURSOR_AXIS: glam::I64Vec2 = glam::i64vec2(0, 0);
pub static mut CURSOR_LAST_AXIS: glam::I64Vec2 = glam::i64vec2(0, 0);

pub static mut UNIFORM1D: [Option<env::InputUnit1D>; UNIFORM_LEN] = [Option::None; UNIFORM_LEN];
pub static mut UNIFORM2D: [Option<env::InputUnit2D>; UNIFORM_LEN] = [Option::None; UNIFORM_LEN];
pub static mut UNIFORM4D: [Option<env::InputUnit4D>; UNIFORM_LEN] = [Option::None; UNIFORM_LEN];
pub static mut UNIFORM3D: [Option<env::InputUnit3D>; UNIFORM_LEN] = [Option::None; UNIFORM_LEN];

pub static mut _3D_OFFSET: usize = 0;
pub static mut _2D_OFFSET: usize = 0;
pub static mut _1D_OFFSET: usize = 0;
pub static mut _4D_OFFSET: usize = 0;

pub static mut ACTIVE_KEYS_1D_PTR: *const Vec<u64> = null();
pub static mut ACTIVE_KEYS_2D_PTR: *const Vec<u64> = null();
pub static mut ACTIVE_KEYS_3D_PTR: *const Vec<u64> = null();
pub static mut ACTIVE_KEYS_4D_PTR: *const Vec<u64> = null();

pub static mut INPUT_LOCK: bool = false;

#[cfg(feature = "env_bit_64bit")]
#[allow(unused)]
pub mod env {
    use std::{
        ops::{Deref, DerefMut},
        time::Duration,
    };

    use glam::I64Vec3;

    use crate::{get, get_mut};

    use super::DIM_NUM;

    pub trait InputUnitT {
        type ValueType;
        fn key(&self) -> u64;
        fn value(&self) -> Self::ValueType;
        fn set_value(&mut self, vin: Self::ValueType);
        fn get(&self) -> bool;
        fn read(&self) -> bool;
    }

    pub mod active_optional {

        use super::{InputKey1D, InputKey2D};

        pub const DEFAULT_4X_1D: [u64; 10] = [
            // InputKey1D::MOUSE_LEFT_BUTTON,
            // InputKey1D::MOUSE_RIGHT_BUTTON,
            InputKey1D::SPACE,
            InputKey1D::SHIFT,
            InputKey1D::Z_,
            InputKey1D::X_,
            InputKey1D::C_,
            InputKey1D::V_,
            InputKey1D::A_,
            InputKey1D::S_,
            InputKey1D::D_,
            InputKey1D::F_,
        ];

        pub const DEFAULT_4X_2D: [u64; 1] = [InputKey2D::MOUSE_AXIS];
    }

    #[derive(Clone, Copy, Default, Debug)]
    #[repr(C, align(4))]
    // 输入流 用于检测组合输入
    pub struct InputStream {}

    // 输入单元 用于检测单次输入
    #[derive(Clone, Copy, Default, Debug)]
    #[repr(C, align(4))]
    pub struct InputUnit1D {
        pub key: u64,
        pub value: Option<i64>,
    }

    #[derive(Clone, Copy, Default, Debug)]
    pub struct InputUnit2D {
        pub key: u64,
        pub value: Option<glam::I64Vec2>,
    }

    #[derive(Clone, Copy, Default, Debug)]
    pub struct InputUnit3D {
        pub key: u64,
        pub value: Option<glam::I64Vec3>,
    }

    #[derive(Clone, Copy, Default, Debug)]
    pub struct InputUnit4D {
        pub key: u64,
        pub value: Option<glam::I64Vec4>,
    }

    pub struct InputKey1D(u64);
    pub struct InputValue1D(i64);

    pub struct InputKey2D(u64);

    // 最好确保这玩意项目内唯一
    // 即便我没把它变成一个全局变量
    pub struct InputE {
        id: u64,
        active_keys: Vec<Vec<u64>>,
        count: usize,
    }

    impl InputE {
        pub fn cleaning() {
            unsafe {
                super::_1D_OFFSET = 0;
                super::_2D_OFFSET = 0;
                super::_3D_OFFSET = 0;
                super::_4D_OFFSET = 0;
            }
        }

        pub fn clear(&mut self) {
            self.count = self.count + 1;
            if self.count > super::MAX_CLEAR_COUNT {
                unsafe {
                    super::_1D_OFFSET = 0;
                    super::_2D_OFFSET = 0;
                    super::_3D_OFFSET = 0;
                    super::_4D_OFFSET = 0;
                }
                self.count = 0;
            };
        }

        pub fn cursor() -> Option<InputUnit1D> {
            unsafe {
                if super::CURSOR_LEFT_DOWN != false {
                    return Some(
                        InputUnit1D::build(InputKey1D::MOUSE_LEFT_BUTTON)
                            .build_value(InputValue1D::DOWM),
                    );
                }
                if super::CURSOR_RIGHT_DOWN != false {
                    return Some(
                        InputUnit1D::build(InputKey1D::MOUSE_RIGHT_BUTTON)
                            .build_value(InputValue1D::DOWM),
                    );
                }
                if super::CURSOR_MID_DOWN != false {
                    return Some(
                        InputUnit1D::build(InputKey1D::MOUSE_MID_BUTTON)
                            .build_value(InputValue1D::DOWM),
                    );
                }
                if super::CURSOR_LEFT_UP != false {
                    return Some(
                        InputUnit1D::build(InputKey1D::MOUSE_LEFT_BUTTON)
                            .build_value(InputValue1D::UP),
                    );
                }
                if super::CURSOR_RIGHT_UP != false {
                    return Some(
                        InputUnit1D::build(InputKey1D::MOUSE_RIGHT_BUTTON)
                            .build_value(InputValue1D::UP),
                    );
                }
                if super::CURSOR_MID_UP != false {
                    return Some(
                        InputUnit1D::build(InputKey1D::MOUSE_MID_BUTTON)
                            .build_value(InputValue1D::UP),
                    );
                }
            }
            return Option::None;
        }

        pub fn cursor_current_axis() -> glam::I64Vec2 {
            return unsafe { super::CURSOR_AXIS };
        }

        pub fn cursor_last_axis() -> glam::I64Vec2 {
            return unsafe { super::CURSOR_LAST_AXIS };
        }

        pub fn cursor_speed() -> glam::I64Vec2 {
            let _r;
            unsafe {
                _r = super::CURSOR_AXIS - super::CURSOR_LAST_AXIS;
                super::CURSOR_LAST_AXIS = super::CURSOR_AXIS;
            };
            return _r;
        }

        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }

        pub fn build() -> Self {
            let mut _r: Self = Default::default();
            return _r;
        }

        pub fn build_active_key(mut self, key_in: u64, dim_in: u64) -> Self {
            let _index = dim_in - 1;
            unsafe {
                if get!(self.active_keys, _index as usize).len()
                    < get!(self.active_keys, _index as usize).capacity()
                {
                    get_mut!(self.active_keys, _index as usize).push(key_in);
                } else {
                    get_mut!(self.active_keys, _index as usize).push(key_in);
                    self._update_active_key_ptr();
                }
            };

            return self;
        }

        pub fn build_active_keys(mut self, keys_in: Vec<u64>, dim_in: u64) -> Self {
            let _index = dim_in - 1;
            unsafe {
                self.active_keys
                    .get_mut(_index as usize)
                    .unwrap()
                    .extend(keys_in.iter())
            };

            self._update_active_key_ptr();
            return self;
        }

        pub fn send_buffer_unit1d(uin: InputUnit1D) {
            unsafe {
                while super::INPUT_LOCK {
                    std::thread::sleep(Duration::from_nanos(1));
                }
                super::INPUT_LOCK = true;
                if super::_1D_OFFSET < super::UNIFORM_LEN {
                    *get_mut!(super::UNIFORM1D, super::_1D_OFFSET) = Some(uin);
                    super::_1D_OFFSET = super::_1D_OFFSET + 1;
                } else {
                    *super::UNIFORM1D.last_mut().unwrap() = Some(uin);
                }
                super::INPUT_LOCK = false;
            }
        }

        pub fn send_buffer_unit2d(uin: InputUnit2D) {
            unsafe {
                while super::INPUT_LOCK {
                    std::thread::sleep(Duration::from_micros(1));
                }
                super::INPUT_LOCK = true;
                if super::_1D_OFFSET < super::UNIFORM_LEN {
                    *get_mut!(super::UNIFORM2D, super::_1D_OFFSET) = Some(uin);
                    super::_1D_OFFSET = super::_1D_OFFSET + 1;
                } else {
                    *super::UNIFORM2D.last_mut().unwrap() = Some(uin);
                }
                super::INPUT_LOCK = false;
            }
        }

        pub fn send_buffer_unit3d(uin: InputUnit3D) {
            unsafe {
                while super::INPUT_LOCK {
                    std::thread::sleep(Duration::from_micros(1));
                }
                super::INPUT_LOCK = true;
                if super::_1D_OFFSET < super::UNIFORM_LEN {
                    *get_mut!(super::UNIFORM3D, super::_1D_OFFSET) = Some(uin);
                    super::_1D_OFFSET = super::_1D_OFFSET + 1;
                } else {
                }
                super::INPUT_LOCK = false;
            }
        }

        pub fn send_buffer_unit4d(uin: InputUnit4D) {
            unsafe {
                while super::INPUT_LOCK {
                    std::thread::sleep(Duration::from_micros(1));
                }
                super::INPUT_LOCK = true;
                if super::_1D_OFFSET < super::UNIFORM_LEN {
                    *get_mut!(super::UNIFORM4D, super::_1D_OFFSET) = Some(uin);
                    super::_1D_OFFSET = super::_1D_OFFSET + 1;
                } else {
                }
                super::INPUT_LOCK = false;
            }
        }

        fn _update_active_key_ptr(&self) {
            unsafe {
                while super::INPUT_LOCK {
                    std::thread::sleep(Duration::from_micros(1));
                }
                super::INPUT_LOCK = true;
                for i in 0..super::DIM_NUM {
                    match i {
                        1 => {
                            super::ACTIVE_KEYS_1D_PTR = &self.active_keys[0];
                        }
                        2 => {
                            super::ACTIVE_KEYS_2D_PTR = &self.active_keys[1];
                        }
                        3 => {
                            super::ACTIVE_KEYS_3D_PTR = &self.active_keys[2];
                        }
                        4 => {
                            super::ACTIVE_KEYS_4D_PTR = &self.active_keys[3];
                        }
                        _ => {}
                    }
                }
                super::INPUT_LOCK = false;
            }
        }
    }

    impl InputUnit1D {
        pub fn build(key: u64) -> Self {
            return Self {
                key,
                value: Option::None,
            };
        }

        pub fn build_value(mut self, value: i64) -> Self {
            self.value = Some(value);
            return self;
        }
    }

    impl InputUnit2D {
        pub fn new(key: u64) -> Self {
            return Self {
                key,
                value: Option::None,
            };
        }

        pub fn build_value(mut self, value: glam::I64Vec2) -> Self {
            self.value = Some(value);
            return self;
        }
    }

    impl InputUnit3D {
        pub fn new(key: u64) -> Self {
            return Self {
                key,
                value: Option::None,
            };
        }

        pub fn build_value(mut self, value: glam::I64Vec3) -> Self {
            self.value = Some(value);
            return self;
        }
    }

    impl InputUnit4D {
        pub fn new(key: u64) -> Self {
            return Self {
                key,
                value: Option::None,
            };
        }

        pub fn build_value(mut self, value: glam::I64Vec4) -> Self {
            self.value = Some(value);
            return self;
        }
    }

    impl InputUnitT for InputUnit1D {
        type ValueType = i64;

        // 读取指定单元
        fn read(&self) -> bool {
            unsafe {
                // 遍历缓冲
                for ui in 0..super::_1D_OFFSET {
                    // 判断是否是可用缓存
                    if let Some(_buffer_unit) = get!((super::UNIFORM1D), ui) {
                        // 判断key是否相等
                        if _buffer_unit.key == self.key() {
                            // 判断value是否设置
                            match self.value {
                                Some(_) => {
                                    {
                                        // 判断value是否相等
                                        if self.value() == _buffer_unit.value() {
                                            return true;
                                        }
                                    }
                                }
                                None => {
                                    return true;
                                }
                            }
                        };
                    };
                }
            }
            return false;
        }

        // 弹出指定输入单元
        // 当不设置value时，会相应所有
        // 输入参数 键盘单元value为None
        //      缓冲区中存在key
        //          返回true
        //
        // 输入参数 键盘单元value为Some
        //      缓冲区中存在key 且key的value与输入参数相同
        //          返回true
        // 其他情况： 返回false
        fn get(&self) -> bool {
            unsafe {
                // 遍历缓冲
                for ui in 0..super::_1D_OFFSET {
                    // 判断是否是可用缓存
                    if let Some(_buffer_unit) = get!((super::UNIFORM1D), ui) {
                        // 判断key是否相等
                        if _buffer_unit.key == self.key() {
                            // 判断value是否设置
                            match self.value {
                                Some(_) => {
                                    {
                                        // 判断value是否相等
                                        if self.value() == _buffer_unit.value() {
                                            super::UNIFORM1D.swap(ui, super::_1D_OFFSET - 1);
                                            super::_1D_OFFSET = super::_1D_OFFSET - 1;

                                            return true;
                                        }
                                    }
                                }
                                None => {
                                    super::UNIFORM1D.swap(ui, super::_1D_OFFSET - 1);
                                    super::_1D_OFFSET = super::_1D_OFFSET - 1;

                                    return true;
                                }
                            }
                        };
                    };
                }
            }
            return false;
        }

        fn key(&self) -> u64 {
            self.key
        }

        fn value(&self) -> Self::ValueType {
            self.value.unwrap()
        }

        fn set_value(&mut self, vin: Self::ValueType) {
            self.value = Some(vin);
        }
    }

    impl InputUnitT for InputUnit2D {
        type ValueType = glam::I64Vec2;

                // 读取指定单元
                fn read(&self) -> bool {
                    unsafe {
                        // 遍历缓冲
                        for ui in 0..super::_2D_OFFSET {
                            // 判断是否是可用缓存
                            if let Some(_buffer_unit) = get!((super::UNIFORM2D), ui) {
                                // 判断key是否相等
                                if _buffer_unit.key == self.key() {
                                    // 判断value是否设置
                                    match self.value {
                                        Some(_) => {
                                            {
                                                // 判断value是否相等
                                                if self.value() == _buffer_unit.value() {
                                                    return true;
                                                }
                                            }
                                        }
                                        None => {
                                            return true;
                                        }
                                    }
                                };
                            };
                        }
                    }
                    return false;
                }

        // 弹出指定输入单元
        // 当不设置value时，会相应所有
        // 输入参数 键盘单元value为None
        //      缓冲区中存在key
        //          返回true
        //
        // 输入参数 键盘单元value为Some
        //      缓冲区中存在key 且key的value与输入参数相同
        //          返回true
        // 其他情况： 返回false
        fn get(&self) -> bool {
            unsafe {
                // 遍历缓冲
                for ui in 0..super::_2D_OFFSET {
                    // 判断是否是可用缓存
                    if let Some(_buffer_unit) = get!((super::UNIFORM2D), ui) {
                        // 判断key是否相等
                        if _buffer_unit.key == self.key() {
                            // 判断value是否设置
                            match self.value {
                                Some(_) => {
                                    {
                                        // 判断value是否相等
                                        if self.value() == _buffer_unit.value() {
                                            super::UNIFORM2D.swap(ui, super::_2D_OFFSET - 1);
                                            super::_2D_OFFSET = super::_2D_OFFSET - 1;

                                            return true;
                                        }
                                    }
                                }
                                None => {
                                    super::UNIFORM2D.swap(ui, super::_2D_OFFSET - 1);
                                    super::_2D_OFFSET = super::_2D_OFFSET - 1;

                                    return true;
                                }
                            }
                        };
                    };
                }
            }
            return false;
        }

        fn key(&self) -> u64 {
            self.key
        }

        fn value(&self) -> Self::ValueType {
            self.value.unwrap()
        }

        fn set_value(&mut self, vin: Self::ValueType) {
            self.value = Some(vin);
        }
    }

    impl InputUnitT for InputUnit3D {
        type ValueType = I64Vec3;

                // 读取指定单元
                fn read(&self) -> bool {
                    unsafe {
                        // 遍历缓冲
                        for ui in 0..super::_3D_OFFSET {
                            // 判断是否是可用缓存
                            if let Some(_buffer_unit) = get!((super::UNIFORM3D), ui) {
                                // 判断key是否相等
                                if _buffer_unit.key == self.key() {
                                    // 判断value是否设置
                                    match self.value {
                                        Some(_) => {
                                            {
                                                // 判断value是否相等
                                                if self.value() == _buffer_unit.value() {
                                                    return true;
                                                }
                                            }
                                        }
                                        None => {
                                            return true;
                                        }
                                    }
                                };
                            };
                        }
                    }
                    return false;
                }

        // 弹出指定输入单元
        // 当不设置value时，会相应所有
        // 输入参数 键盘单元value为None
        //      缓冲区中存在key
        //          返回true
        //
        // 输入参数 键盘单元value为Some
        //      缓冲区中存在key 且key的value与输入参数相同
        //          返回true
        // 其他情况： 返回false
        fn get(&self) -> bool {
            unsafe {
                // 遍历缓冲
                for ui in 0..super::_3D_OFFSET {
                    // 判断是否是可用缓存
                    if let Some(_buffer_unit) = get!((super::UNIFORM3D), ui) {
                        // 判断key是否相等
                        if _buffer_unit.key == self.key() {
                            // 判断value是否设置
                            match self.value {
                                Some(_) => {
                                    {
                                        // 判断value是否相等
                                        if self.value() == _buffer_unit.value() {
                                            super::UNIFORM3D.swap(ui, super::_3D_OFFSET - 1);
                                            super::_3D_OFFSET = super::_3D_OFFSET - 1;

                                            return true;
                                        }
                                    }
                                }
                                None => {
                                    super::UNIFORM3D.swap(ui, super::_3D_OFFSET - 1);
                                    super::_3D_OFFSET = super::_3D_OFFSET - 1;

                                    return true;
                                }
                            }
                        };
                    };
                }
            }
            return false;
        }

        fn key(&self) -> u64 {
            self.key
        }

        fn value(&self) -> Self::ValueType {
            self.value.unwrap()
        }

        fn set_value(&mut self, vin: Self::ValueType) {
            self.value = Some(vin);
        }
    }

    impl InputUnitT for InputUnit4D {
        type ValueType = glam::I64Vec4;

                // 读取指定单元
                fn read(&self) -> bool {
                    unsafe {
                        // 遍历缓冲
                        for ui in 0..super::_4D_OFFSET {
                            // 判断是否是可用缓存
                            if let Some(_buffer_unit) = get!((super::UNIFORM4D), ui) {
                                // 判断key是否相等
                                if _buffer_unit.key == self.key() {
                                    // 判断value是否设置
                                    match self.value {
                                        Some(_) => {
                                            {
                                                // 判断value是否相等
                                                if self.value() == _buffer_unit.value() {
                                                    return true;
                                                }
                                            }
                                        }
                                        None => {
                                            return true;
                                        }
                                    }
                                };
                            };
                        }
                    }
                    return false;
                }

        // 弹出指定输入单元
        // 当不设置value时，会相应所有
        // 输入参数 键盘单元value为None
        //      缓冲区中存在key
        //          返回true
        //
        // 输入参数 键盘单元value为Some
        //      缓冲区中存在key 且key的value与输入参数相同
        //          返回true
        // 其他情况： 返回false
        fn get(&self) -> bool {
            unsafe {
                // 遍历缓冲
                for ui in 0..super::_4D_OFFSET {
                    // 判断是否是可用缓存
                    if let Some(_buffer_unit) = get!((super::UNIFORM4D), ui) {
                        // 判断key是否相等
                        if _buffer_unit.key == self.key() {
                            // 判断value是否设置
                            match self.value {
                                Some(_) => {
                                    {
                                        // 判断value是否相等
                                        if self.value() == _buffer_unit.value() {
                                            super::UNIFORM4D.swap(ui, super::_4D_OFFSET - 1);
                                            super::_4D_OFFSET = super::_4D_OFFSET - 1;

                                            return true;
                                        }
                                    }
                                }
                                None => {
                                    super::UNIFORM4D.swap(ui, super::_4D_OFFSET - 1);
                                    super::_4D_OFFSET = super::_4D_OFFSET - 1;

                                    return true;
                                }
                            }
                        };
                    };
                }
            }
            return false;
        }

        fn key(&self) -> u64 {
            self.key
        }

        fn value(&self) -> Self::ValueType {
            self.value.unwrap()
        }

        fn set_value(&mut self, vin: Self::ValueType) {
            self.value = Some(vin);
        }
    }

    #[allow(unused)]
    impl InputKey2D {
        pub const MOUSE_AXIS: u64 = 0x200;
        pub const CURSOR_AXIS: u64 = 0x100;
    }

    #[allow(unused)]
    impl InputValue1D {
        pub const DOWM: i64 = 0x01;
        pub const UP: i64 = 0x02;
        pub const HOLD: i64 = 0x10;
    }

    #[cfg(feature = "env_os_win")]
    #[allow(unused)]
    impl InputKey1D {
        pub const MOUSE_LEFT_BUTTON: u64 = 0x201;
        pub const MOUSE_RIGHT_BUTTON: u64 = 0x204;
        pub const MOUSE_MID_BUTTON: u64 = 0x207;

        pub const BACKSPACE: u64 = 0x08;
        pub const TAB: u64 = 0x09;

        pub const CLEAR: u64 = 0x0C;
        pub const ENTER: u64 = 0x0D;

        pub const SHIFT: u64 = 0x10;
        pub const CONTROL: u64 = 0x11;
        pub const ALT: u64 = 0x12;
        pub const PAUSE: u64 = 0x13;
        pub const CAPS_LOCK: u64 = 0x14;
        pub const ESCAPE: u64 = 0x1B;

        pub const SPACE: u64 = 0x20;
        pub const PAGE_UP: u64 = 0x21;
        pub const PAGE_DOWN: u64 = 0x22;
        pub const END: u64 = 0x23;
        pub const HOME: u64 = 0x24;
        pub const LEFT_ARROW: u64 = 0x25;
        pub const UP_ARROW: u64 = 0x26;
        pub const RIGHT_ARROW: u64 = 0x27;
        pub const DOWN_ARROW: u64 = 0x28;
        pub const SELECT: u64 = 0x29;
        pub const PRINT: u64 = 0x2A;
        pub const EXECUTE: u64 = 0x2B;
        pub const PRINT_SCREEN: u64 = 0x2C;
        pub const INSERT: u64 = 0x2D;
        pub const DELETE: u64 = 0x2E;
        pub const HELP: u64 = 0x2F;

        pub const _0: u64 = 0x30;
        pub const _1: u64 = 0x31;
        pub const _2: u64 = 0x32;
        pub const _3: u64 = 0x33;
        pub const _4: u64 = 0x34;
        pub const _5: u64 = 0x35;
        pub const _6: u64 = 0x36;
        pub const _7: u64 = 0x37;
        pub const _8: u64 = 0x38;
        pub const _9: u64 = 0x39;

        pub const A_: u64 = 0x41;
        pub const B_: u64 = 0x42;
        pub const C_: u64 = 0x43;
        pub const D_: u64 = 0x44;
        pub const E_: u64 = 0x45;
        pub const F_: u64 = 0x46;
        pub const G_: u64 = 0x47;
        pub const H_: u64 = 0x48;
        pub const I_: u64 = 0x49;
        pub const J_: u64 = 0x4A;
        pub const K_: u64 = 0x4B;
        pub const L_: u64 = 0x4C;
        pub const M_: u64 = 0x4D;
        pub const N_: u64 = 0x4E;
        pub const O_: u64 = 0x4F;
        pub const P_: u64 = 0x50;
        pub const Q_: u64 = 0x51;
        pub const R_: u64 = 0x52;
        pub const S_: u64 = 0x53;
        pub const T_: u64 = 0x54;
        pub const U_: u64 = 0x55;
        pub const V_: u64 = 0x56;
        pub const W_: u64 = 0x57;
        pub const X_: u64 = 0x58;
        pub const Y_: u64 = 0x59;
        pub const Z_: u64 = 0x5A;
        pub const LEFT_WIN: u64 = 0x5B;
        pub const RIGHT_WIN: u64 = 0x5C;

        pub const HOSTSLEEP: u64 = 0x5F;
        pub const NUMPAD0: u64 = 0x60;
        pub const NUMPAD1: u64 = 0x61;
        pub const NUMPAD2: u64 = 0x62;
        pub const NUMPAD3: u64 = 0x63;
        pub const NUMPAD4: u64 = 0x64;
        pub const NUMPAD5: u64 = 0x65;
        pub const NUMPAD6: u64 = 0x66;
        pub const NUMPAD7: u64 = 0x67;
        pub const NUMPAD8: u64 = 0x68;
        pub const NUMPAD9: u64 = 0x69;
        pub const MULTIPLY: u64 = 0x6A;
        pub const ADD: u64 = 0x6B;
        pub const SEPARATOR: u64 = 0x6C;
        pub const SUBTRACT: u64 = 0x6D;
        pub const DECIMAL: u64 = 0x6E;
        pub const DIVIDE: u64 = 0x6F;
        pub const F1: u64 = 0x70;
        pub const F2: u64 = 0x71;
        pub const F3: u64 = 0x72;
        pub const F4: u64 = 0x73;
        pub const F5: u64 = 0x74;
        pub const F6: u64 = 0x75;
        pub const F7: u64 = 0x76;
        pub const F8: u64 = 0x77;
        pub const F9: u64 = 0x78;
        pub const F10: u64 = 0x79;
        pub const F11: u64 = 0x7A;
        pub const F12: u64 = 0x7B;
        pub const F13: u64 = 0x7C;
        pub const F14: u64 = 0x7D;
        pub const F15: u64 = 0x7E;
        pub const F16: u64 = 0x7F;
        pub const F17: u64 = 0x80;
        pub const F18: u64 = 0x81;
        pub const F19: u64 = 0x82;
        pub const F20: u64 = 0x83;
        pub const F21: u64 = 0x84;
        pub const F22: u64 = 0x85;
        pub const F23: u64 = 0x86;
        pub const F24: u64 = 0x87;

        pub const NUM_LOCK: u64 = 0x90;
        pub const SCROLL: u64 = 0x91;

        pub const LEFT_SHIFT: u64 = 0xA0;
        pub const RIGHT_SHIFT: u64 = 0xA1;
        pub const LEFT_CONTROL: u64 = 0xA2;
        pub const RIGHTCONTROL: u64 = 0xA3;
        pub const LEFT_ALT: u64 = 0xA4;
        pub const RIGHT_ALT: u64 = 0xA5;
    }

    impl Default for InputE {
        fn default() -> Self {
            let mut _active_keys = Vec::with_capacity(super::DIM_NUM);
            for _i in 0..DIM_NUM {
                _active_keys.push(Default::default());
            }
            Self {
                id: Default::default(),
                active_keys: _active_keys,
                count: 0,
            }
        }
    }
}

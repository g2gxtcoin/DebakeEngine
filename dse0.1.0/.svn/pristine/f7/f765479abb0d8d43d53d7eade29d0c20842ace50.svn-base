#[cfg(feature = "env_os_win")]
static mut GLOBAL_KRYBORAD_HOOK: winapi::shared::windef::HHOOK = std::ptr::null_mut();
#[cfg(feature = "env_os_win")]
static mut GLOBAL_MOUSE_HOOK: winapi::shared::windef::HHOOK = std::ptr::null_mut();

#[cfg(feature = "env_os_win")]
#[cfg(feature = "config_DEFAULT_INPUT_DETECT_OFFSET_MS_33")]
const DEFAULT_INPUT_DETECT_OFFSET_MS: u32 = 0030000000;

#[cfg(feature = "env_os_win")]
#[cfg(feature = "config_DEFAULT_INPUT_DETECT_OFFSET_MS_16")]
const DEFAULT_INPUT_DETECT_OFFSET_MS: u32 = 0016000000;

//input
#[cfg(feature = "env_os_win")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {
    extern crate winapi;
    use ash::vk::DWORD;
    use std::ptr::{null_mut, swap};
    use winapi::shared::minwindef::UINT;
    use winapi::shared::windef::HHOOK;
    use winapi::um::processthreadsapi::GetCurrentThreadId;
    use winapi::um::winuser::{
        CallNextHookEx, DispatchMessageW, PeekMessageW, PostMessageA, PostMessageW,
        PostThreadMessageW, SetWindowsHookExW, TranslateMessage, MOUSEHOOKSTRUCT, PM_QS_INPUT,
        WH_MOUSE, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONDOWN, WM_RBUTTONUP,
    };
    use winapi::um::winuser::{MSG, PM_REMOVE, WH_KEYBOARD};
    use winapi::{
        shared::{
            minwindef::{LPARAM, LRESULT, WPARAM},
            windef::{HWND, POINT},
        },
        um::winuser::WM_KEYDOWN,
    };

    use crate::input::env::InputUnit1D;
    use crate::{dev_dbg, dev_stop, input};

    use super::{DEFAULT_INPUT_DETECT_OFFSET_MS, GLOBAL_KRYBORAD_HOOK, GLOBAL_MOUSE_HOOK};

    pub struct WinInputE {
        id: u64,
        msg_ptr: *mut MSG,
        msg_entity: MSG,
        keybord_hook: Option<HHOOK>,
        mouse_hook: Option<HHOOK>,
        current_thread_id: u32,
    }

    impl Default for WinInputE {
        fn default() -> Self {
            Self {
                msg_ptr: null_mut(),
                msg_entity: MSG {
                    hwnd: null_mut(),
                    message: 0,
                    wParam: 0,
                    lParam: 0,
                    time: 0,
                    pt: POINT { x: 0, y: 0 },
                },
                id: 0,
                keybord_hook: Option::None,
                mouse_hook: Option::None,
                current_thread_id: 0,
            }
        }
    }

    impl WinInputE {
        pub fn build() -> Self {
            return Default::default();
        }

        pub fn build_link_wndhandle(mut self, hin: HWND) -> Self {
            self.msg_entity.hwnd = hin;
            self.msg_ptr = &mut self.msg_entity as *mut MSG;
            return self;
        }

        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }

        // 键盘钩子
        // 用于将输入独立于窗口运行
        pub fn build_hook_keyboard(mut self) -> Self {
            unsafe {
                self.current_thread_id = GetCurrentThreadId();
                self.keybord_hook = Some(SetWindowsHookExW(
                    WH_KEYBOARD,
                    Option::Some(keyboard_proc_func),
                    null_mut(),
                    self.current_thread_id,
                ));
                self.mouse_hook = Some(SetWindowsHookExW(
                    WH_MOUSE,
                    Option::Some(mouse_proc_func),
                    null_mut(),
                    self.current_thread_id,
                ));
                GLOBAL_KRYBORAD_HOOK = self.keybord_hook.unwrap();
                GLOBAL_MOUSE_HOOK = self.mouse_hook.unwrap();
            }
            return self;
        }

        // 检查线程消息队列缓存
        // 并将消息传递至窗口回调
        pub unsafe fn peek(&mut self) {
            match PeekMessageW(
                //self.msg_ptr ,
                self.msg_ptr,
                /*self.wnd_msg_entity.hwnd*/
                null_mut(),
                0,
                0,
                PM_REMOVE,
            ) {
                -1 => {
                    crate::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_UNDEFINE_BEHAVIOR
                            | crate::log::code::FILE_INPUT_WIN
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogCodePart::Id)
                                .get_code(),
                    );
                }
                0 => {}

                _ => {
                    // crate::send2logger_dev!(
                    //     crate::log::code::TYPE_CORE_INFO
                    //         | crate::log::code::CONDI_RECIEVE_WIN_MSG_SUSSECE
                    //         | crate::log::code::FILE_INPUT_WIN
                    //         | crate::log::LogCodeD::new()
                    //             .encode(line!() as u128, crate::log::LogCodePart::Line)
                    //             .get_code()
                    //         | crate::log::LogCodeD::new()
                    //             .encode(self.id as u128, crate::log::LogCodePart::Id)
                    //             .get_code()
                    // );

                    //TranslateMessage(self.msg_ptr );
                    DispatchMessageW(self.msg_ptr);
                }
            }
            dev_dbg!(std::io::Error::last_os_error());
        }

        // 未完工
        #[warn(dead_code)]
        pub unsafe fn run_peek(self) {
            let _mpin = self.msg_ptr as u32;
            let _thread_id_in = self.current_thread_id;
            let _hin = self.msg_entity.hwnd as u32;

            std::thread::spawn(move || {
                Self::peek_asyn(_mpin, _thread_id_in, _hin);
            });
        }

        // 未完工
        #[warn(dead_code)]
        pub unsafe fn peek_asyn(mpin: u32, thread_id_in: u32, hin: u32) {
            let _mp = mpin as *mut MSG;
            let _h = hin as HWND;
            let _thread = thread_id_in as DWORD;
            while crate::workarea::WORKAREA_CLOSE == false {
                match PeekMessageW(
                    //_thread,
                    /*self.wnd_msg_entity.hwnd*/
                    _mp, _h, 0, 0, PM_REMOVE,
                ) {
                    -1 => {
                        crate::sorry(
                            crate::log::code::TYPE_CORE_ERROR
                                | crate::log::code::CONDI_UNDEFINE_BEHAVIOR
                                | crate::log::code::FILE_INPUT_WIN
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(0, crate::log::LogCodePart::Id)
                                    .get_code(),
                        );
                    }
                    0 => {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_CORE_INFO
                                | crate::log::code::CONDI_RECIEVE_WIN_MSG_SUSSECE
                                | crate::log::code::FILE_INPUT_WIN
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(0 as u128, crate::log::LogCodePart::Id)
                                    .get_code()
                        );
                    }

                    _ => {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_CORE_INFO
                                | crate::log::code::CONDI_RECIEVE_WIN_MSG_SUSSECE
                                | crate::log::code::FILE_INPUT_WIN
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(0 as u128, crate::log::LogCodePart::Id)
                                    .get_code()
                        );
                        //TranslateMessage(&(*self.wnd_msg));
                        DispatchMessageW(_mp);
                        SetWindowsHookExW(
                            WH_KEYBOARD,
                            Option::Some(keyboard_proc_func),
                            null_mut(),
                            GetCurrentThreadId(),
                        );
                        SetWindowsHookExW(
                            WH_MOUSE,
                            Option::Some(mouse_proc_func),
                            null_mut(),
                            thread_id_in,
                        );
                    }
                }
                //dev_dbg!(std::io::Error::last_os_error());
                std::thread::sleep(std::time::Duration::new(
                    0,
                    DEFAULT_INPUT_DETECT_OFFSET_MS * 100,
                ));
            }
        }

        pub unsafe fn get_raw_deviceinput() {}
    }

    impl Drop for WinInputE {
        fn drop(&mut self) {
            println!()
        }
    }

    unsafe fn _is_active_key(wpin: &WPARAM) -> bool {
        return !crate::input::ACTIVE_KEYS_1D_PTR.is_null()
            && (*crate::input::ACTIVE_KEYS_1D_PTR).contains(&(*wpin as u64));
    }

    pub unsafe extern "system" fn keyboard_proc_func(
        _code: i32,
        _w_param: WPARAM,
        _l_param: LPARAM,
    ) -> LRESULT {
        if _is_active_key(&_w_param) {
            
            crate::input::env::InputE::send_buffer_unit1d(
                InputUnit1D::build(_w_param as u64)
                    .build_value(_translte_keyboard_lparam(_l_param)),
            );
        }

        return CallNextHookEx(GLOBAL_KRYBORAD_HOOK, _code, _w_param, _l_param);
    }

    fn _translte_keyboard_lparam(lin: LPARAM) -> i64 {
        let _l = lin as u64;
        if (_l & 0xffff) > 1 {
            return input::env::InputValue1D::HOLD;
        }
        // hold
        else {
            if (_l >> 31) & 0x1 == 1 {
                return input::env::InputValue1D::UP;
            }
            // up
            else {
                return input::env::InputValue1D::DOWM;
            } // down
        } // click
    }

    pub unsafe extern "system" fn mouse_proc_func(
        _code: i32,
        _w_param: WPARAM,
        _l_param: LPARAM,
    ) -> LRESULT {
        let _p = _l_param as *const MOUSEHOOKSTRUCT;

        match _w_param {
            // mouse move
            0x200 => {
                crate::input::CURSOR_AXIS = glam::i64vec2((*_p).pt.x as i64, (*_p).pt.y as i64);
            }

            // mouse left down
            0x201 => {
                crate::input::CURSOR_LEFT_DOWN = true;
                crate::input::CURSOR_LEFT_UP = false
            }
            // mouse left up
            0x202 => {
                crate::input::CURSOR_LEFT_DOWN = false;
                crate::input::CURSOR_LEFT_UP = true
            }
            // mouse right down
            0x204 => {
                crate::input::CURSOR_RIGHT_DOWN = true;
                crate::input::CURSOR_RIGHT_UP = false
            }
            // mouse right up
            0x205 => {
                crate::input::CURSOR_RIGHT_DOWN = false;
                crate::input::CURSOR_RIGHT_UP = true
            }

            // mouse mid down
            0x207 => {
                crate::input::CURSOR_MID_DOWN = true;
                crate::input::CURSOR_MID_UP = false
            }

            // mouse mid move
            0x20a => {
                crate::input::CURSOR_MID_DOWN = false;
                crate::input::CURSOR_MID_UP = true
            }

            _ => {}
        }

        return CallNextHookEx(GLOBAL_KRYBORAD_HOOK, _code, _w_param, _l_param);
    }
}

use std::{fmt::Debug, mem};

use winapi::{
    shared::windef::RECT,
    um::{
        wingdi::DEVMODEA,
        winuser::{MONITORINFO, WINDOWINFO},
    },
};

#[allow(unused)]
pub(super) fn default_rect() -> RECT {
    RECT {
        left: unsafe { mem::zeroed() },
        top: unsafe { mem::zeroed() },
        right: unsafe { mem::zeroed() },
        bottom: unsafe { mem::zeroed() },
    }
}

#[allow(unused)]
pub(super) fn default_windowinfo() -> WINDOWINFO {
    WINDOWINFO {
        cbSize: size_of::<WINDOWINFO>() as u32,
        rcWindow: unsafe { mem::zeroed() },
        rcClient: unsafe { mem::zeroed() },
        dwStyle: unsafe { mem::zeroed() },
        dwExStyle: unsafe { mem::zeroed() },
        dwWindowStatus: unsafe { mem::zeroed() },
        cxWindowBorders: unsafe { mem::zeroed() },
        cyWindowBorders: unsafe { mem::zeroed() },
        atomWindowType: unsafe { mem::zeroed() },
        wCreatorVersion: unsafe { mem::zeroed() },
    }
}

#[allow(unused)]
pub(super) fn default_monitor_info() -> MONITORINFO {
    MONITORINFO {
        cbSize: mem::size_of::<MONITORINFO>() as u32,
        rcMonitor: default_rect(),
        rcWork: default_rect(),
        dwFlags: unsafe { mem::zeroed() },
    }
}

#[allow(unused)]
pub(super) fn default_devmode() -> DEVMODEA {
    DEVMODEA {
        dmDeviceName: unsafe { mem::zeroed() },
        dmSpecVersion: unsafe { mem::zeroed() },
        dmDriverVersion: unsafe { mem::zeroed() },
        dmSize: size_of::<DEVMODEA>() as u16,
        dmDriverExtra: unsafe { mem::zeroed() },
        dmFields: unsafe { mem::zeroed() },
        u1: unsafe { mem::zeroed() },
        dmColor: unsafe { mem::zeroed() },
        dmDuplex: unsafe { mem::zeroed() },
        dmYResolution: unsafe { mem::zeroed() },
        dmTTOption: unsafe { mem::zeroed() },
        dmCollate: unsafe { mem::zeroed() },
        dmFormName: unsafe { mem::zeroed() },
        dmLogPixels: unsafe { mem::zeroed() },
        dmBitsPerPel: unsafe { mem::zeroed() },
        dmPelsWidth: unsafe { mem::zeroed() },
        dmPelsHeight: unsafe { mem::zeroed() },
        u2: unsafe { mem::zeroed() },
        dmDisplayFrequency: unsafe { mem::zeroed() },
        dmICMMethod: unsafe { mem::zeroed() },
        dmICMIntent: unsafe { mem::zeroed() },
        dmMediaType: unsafe { mem::zeroed() },
        dmDitherType: unsafe { mem::zeroed() },
        dmReserved1: unsafe { mem::zeroed() },
        dmReserved2: unsafe { mem::zeroed() },
        dmPanningWidth: unsafe { mem::zeroed() },
        dmPanningHeight: unsafe { mem::zeroed() },
    }
}

#[allow(unused)]
#[cfg(feature = "log_mode_dev")]
pub fn debug_devmode(dmin: &DEVMODEA) {
    println!("DEVMODEA");

    println!("dmSpecVersion{}", &dmin.dmSpecVersion);
    println!("dmDriverVersion{}", &dmin.dmDriverVersion);
    println!("dmSize{}", &dmin.dmSize);
    println!("dmDriverExtra{}", &dmin.dmDriverExtra);
    println!("dmFields{}", &dmin.dmFields);

    println!("dmColor{}", &dmin.dmColor);
    println!("dmDuplex{}", &dmin.dmDuplex);
    println!("dmYResolution{}", &dmin.dmYResolution);
    println!("dmTTOption{}", &dmin.dmTTOption);
    println!("dmCollate{}", &dmin.dmCollate);

    println!("dmLogPixels{}", &dmin.dmLogPixels);
    println!("dmBitsPerPel{}", &dmin.dmBitsPerPel);
    println!("dmPelsWidth{}", &dmin.dmPelsWidth);
    println!("dmPelsHeight{}", &dmin.dmPelsHeight);

    println!("dmDisplayFrequency{}", &dmin.dmDisplayFrequency);
    println!("dmICMMethod{}", &dmin.dmICMMethod);
    println!("dmICMIntent{}", &dmin.dmICMIntent);
    println!("dmMediaType{}", &dmin.dmMediaType);
    println!("dmDitherType{}", &dmin.dmDitherType);
    println!("dmReserved1{}", &dmin.dmReserved1);
    println!("dmReserved2{}", &dmin.dmReserved2);
    println!("dmPanningWidth{}", &dmin.dmPanningWidth);
    println!("dmPanningHeight{}", &dmin.dmPanningHeight);
}

#[allow(unused)]
#[cfg(feature = "log_mode_dev")]
pub(super) fn debug_rect(rin: &RECT) {
    println! {"RECT"};
    println!("left:{}", &rin.left);
    println!("top:{}", &rin.top);
    println!("right:{}", &rin.right);
    println!("bottom:{}", &rin.bottom);
    println!("");
}

#[allow(unused)]
#[cfg(feature = "log_mode_dev")]
pub(super) fn debug_windowinfo(win: &WINDOWINFO) {
    println! {"WINDOWINFO"};
    println!("cbSize{}", &win.cbSize);
    debug_rect( &win.rcWindow);
    debug_rect( &win.rcClient);
    println!("dwStyle{}", &win.dwStyle);
    println!("dwExStyle{}", &win.dwExStyle);
    println!("dwWindowStatus{}", &win.dwWindowStatus);
    println!("cxWindowBorders{}", &win.cxWindowBorders);
    println!("cyWindowBorders{}", &win.cyWindowBorders);
    println!("atomWindowType{}", &win.atomWindowType);
    println!("wCreatorVersion{}", &win.wCreatorVersion);
    println!("");
}

#[allow(unused)]
#[cfg(feature = "log_mode_dev")]
pub(super) fn debug_monitor_info(min: &MONITORINFO) {
    println! {"MONITORINFO"};
    println!("cbSize:{}", &min.cbSize);
    debug_rect(&min.rcMonitor);
    debug_rect(&min.rcWork);
    println!("dwFlags:{}", &min.dwFlags);
    println!("");
}

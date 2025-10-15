#[cfg(feature = "debake_dev")]
#[cfg(feature = "log_mode_dev")]
pub fn os_err(){
    crate::dev_dbg!(std::io::Error::last_os_error()) ;
} 
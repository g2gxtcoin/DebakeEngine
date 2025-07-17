
//global var use for coding debuge info collects
#[cfg(feature = "log_prf_profile_on")]
static mut PERFORMANCE_BUFFER: [u128; MAX_PRF_BUFFER_LEN] = [0; MAX_PRF_BUFFER_LEN];
#[cfg(feature = "log_prf_profile_on")]
static mut PERFORMANCE_BEGING_TIME: u128 = 0;
#[cfg(feature = "log_prf_profile_on")]
static mut PERFORMANCE_OFFSET: usize = 0;
#[cfg(feature = "log_prf_profile_on")]
static mut PERFORMANCE_LAST_PRINT_OFFSET: usize = 0;
#[cfg(feature = "log_prf_profile_on")]
static mut PERFORMANCE_LAST_OUT_OFFSET: usize = 0;
//set up these feature before compiling

pub const RELEVENT_PRF_PATH: &'static str = "performance";
#[cfg(feature = "config_MAX_PRF_BUFFER_LEN_256")]
pub const MAX_PRF_BUFFER_LEN: usize = 256;
#[cfg(feature = "config_MAX_PRF_BUFFER_LEN_4096")]
pub const MAX_PRF_BUFFER_LEN: usize = 4096;

pub struct PRF_flag {
}
impl  PRF_flag {
    
    pub const PRF_PROFILE_PART_DEFAULT:usize = 0x1;
}

#[cfg(feature = "debake_dev")]
#[cfg(feature = "log_mode_dev")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "env_os_win")]
pub mod env {
}

///about prf info code
///use hexadecimal to construct error code
///0-3 pos is logical type
/// 4-7 pos is exe id
/// 8-11 pos is hardware id
/// 12-19 pos is begine time
/// 20-27 pos is end time
/// 28-31 pos is size of data

#[allow(dead_code)]
pub mod code {
    /************************************************************************************size****time1*****time0**hid**eid**type */
    pub const EXE_ID_DEFAULT:/**************************************************/ u128 = 0x0000_00000000_00000000_0000_0000_0000;
    /************************************************************************************size****time1*****time0**hid**eid**type */

}
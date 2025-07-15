

//global var use for coding debuge info collects
static mut PERFORMANCE_BUFFER: [u128; crate::log::MAX_LOGGER_BUFFER_LEN] = [0; crate::log::MAX_LOGGER_BUFFER_LEN];
static mut PERFORMANCE_BEGING_TIME: u128 = 0;
static mut PERFORMANCE_OFFSET: usize = 0;
static mut PERFORMANCE_LAST_PRINT_OFFSET: usize = 0;
static mut PERFORMANCE_LAST_OUT_OFFSET: usize = 0;
//set up these feature before compiling

#[cfg(feature = "debake_dev")]
#[cfg(feature = "log_mode_dev")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "env_os_win")]
pub mod env {
    use std::mem;

    use crate::get;

    #[repr(C, align(8))]
    pub struct PerformanceUnitD {
        id: u64,
        flag: PerformanceFlag,
        buffer: Vec<f64>,
    }

    pub union PerformanceFlag {
        flag: u64,
        part: [u8; 8],
        bit: [bool; 64],
    }

    impl Default for PerformanceFlag {
        fn default() -> Self {
            unsafe { mem::zeroed() }
        }
    }

    impl PerformanceFlag {
        pub const NONE: u64 = 0b0;
        pub const CPU_USAGE: u64 = 0b1;
        pub const MEMORY_USAGE: u64 = 0b10;
        pub const ALL: u64 = Self::CPU_USAGE | Self::MEMORY_USAGE;

        pub fn include_array(&self, fin: u64) -> bool {
            unsafe { fin & self.flag != 0 }
        }
    }
}
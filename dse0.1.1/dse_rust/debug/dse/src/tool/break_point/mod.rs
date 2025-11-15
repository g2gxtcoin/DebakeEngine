#[cfg(feature = "debake_dev")]
#[macro_export]
macro_rules! ________________dev_break________________ {
    ( ) => {{
        // crate::tool::break_point::asm::int1_break();
        // crate::tool::break_point::asm::int3_break();
        // std::thread::sleep(std::time::Duration::new(u64::MAX, 0));
        println!("dev_stop at {:?} in {:?} {:?}.\npause?(-p).\ncontinues?(press Enter)", std::time::SystemTime::now(),file!(),line!());
        let mut _buf = String::new();
        let _ = std::io::stdin().read_line(&mut _buf);
        if _buf== "-p\r\n" {
            std::thread::sleep(std::time::Duration::new(u64::MAX, 0));
        }
    }};
    ($msg:expr) =>{
        println!($msg);
        ________________dev_break________________!();
    }
}

#[cfg(feature = "debake_dev")]
#[macro_export]
macro_rules! ________________dev_process_break________________ {
    ( ) => {{
        // std::thread::sleep(std::time::Duration::new(u64::MAX, 0));
        println!("dev_stop at {:?} in {:?} {:?}.\npause?(-p).\ncontinues?(press Enter)", std::time::SystemTime::now(),file!(),line!());
        let mut _buf = String::new();
        let _ = std::io::stdin().read_line(&mut _buf);
        if _buf== "-p\r\n" {
            std::thread::sleep(std::time::Duration::new(u64::MAX, 0));
        }
    }};
    ($msg:expr) =>{
        println!($msg);
        ________________dev_process_break________________!();
    }
}

#[cfg(feature = "debake_dev")]
#[macro_export]
macro_rules! rt_break {
    ( ) => {};
}


#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! rt_break {
    ( ) => {};
}

#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! ________________dev_break________________ {
    ( ) => {};
}

pub mod asm{
    pub use std::arch::asm;

    use crate::tool::break_point::asm;

    pub fn int1_break(){
        unsafe{
            asm!("int1");
        }
    }
    
    pub fn int3_break(){
        unsafe{
            asm!("int3");
        }
    }
}
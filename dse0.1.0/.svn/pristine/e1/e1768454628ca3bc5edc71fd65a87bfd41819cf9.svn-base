#[cfg(feature = "debake_dev")]
#[macro_export]
macro_rules! dev_stop {
    ( ) => {
        std::thread::sleep(std::time::Duration::new(u64::MAX, 0))
    };
}


#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! rt_stop {
    ( ) => {};
}

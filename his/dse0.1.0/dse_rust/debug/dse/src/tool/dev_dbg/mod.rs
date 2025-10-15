#[cfg(feature = "debake_dev")]
#[cfg(feature = "log_mode_dev")]
#[macro_export]
macro_rules! dev_dbg {
    ( $content:expr ) => {
        dbg!(&$content);
    };
}

#[cfg(feature = "debake_dev")]
#[cfg(feature = "log_mode_dev")]
#[macro_export]
macro_rules! dev_dbg_iter {
    ($content:expr) => {
        for pi in $content.iter() {
            dbg!(pi);
        }
    };
}

#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! dev_dbg_iter {
    ($content:expr) => {
    };
}

#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! dev_dbg {
    (  $content:expr ) => {
    };
}

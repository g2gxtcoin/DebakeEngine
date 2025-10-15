#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! get {
    ( $vec:ident , $index:expr ) => {
        unsafe{ $vec.get_unchecked($index).unwrap_unchecked()}
    };
}

#[cfg(feature = "debake_rt")]
#[macro_export]
macro_rules! get_mut {
    ($vec:expr,$index:expr ) => {
        unsafe{$vec.get_unchecked_mut($index).unwrap_unchecked()} 
    };
}

#[cfg(feature = "debake_dev")]
#[macro_export]
macro_rules! get {
    ($vec:expr,$index:expr ) => {
        $vec.get($index).unwrap()
    };
}

#[cfg(feature = "debake_dev")]
#[macro_export]
macro_rules! get_mut {
    ($vec:expr,$index:expr ) => {
        $vec.get_mut($index).unwrap()
    };
}

#[test]
fn test(){
    let mut  a = Vec::<u32>::new();
    a.push(1);
    dbg!(get_mut!(a,6)); 
}
use std::usize;

/// 建议仅仅针对Execute使用
/// 不要对Dat、Buf数据使用类型强制转换
/// Datum包含数据去碎片功能，其中会改变数据指针位置
/// 使得程序无法按照预期运行
pub fn _cast_ref<Target>(uin: usize) -> &'static Target {
    let _p = unsafe { std::mem::transmute::<usize, &Target>(uin) };
    return _p;
}

/// 建议仅仅针对Exe使用
/// 不要对Dat、Buf数据使用类型强制转换
/// Datum包含数据去碎片功能，其中会改变数据指针位置
/// 使得程序无法按照预期运行
pub fn _cast_mut<Target>(uin: usize) -> &'static mut Target {
    let _p = unsafe { std::mem::transmute::<usize, &mut Target>(uin) };
    return _p;
}

#[macro_export]
macro_rules! cast_ref {
    ($target_type:ty,$uin:expr) => {
        crate::tool::cast::_cast_ref::<$target_type>($uin)
    };
    // ($uin:tt) => {
    //     crate::cast_custom_type!($uin)
    // };
    // (_ash_device_p) => {
    //     crate::cast_custom_type!(_ash_device_p)
    // };
}

#[macro_export]
macro_rules! cast_mut {
    ($target_type:ty,$uin:expr) => {
        crate::tool::cast::_cast_mut::<$target_type>($uin)
    };
    // ($uin:ident) => {
    //     crate::cast_custom_type!($uin,mut)
    // };
}


// #[macro_export]
// macro_rules! cast_custom_type {
//     () => {};
//     (_ash_device_p) => { 
//         crate::tool::cast::_cast_ref::<ash::Device>(_ash_device_p)
//     };
//     (_ash_device_p,mut) => {
//         crate::tool::cast::_cast_mut::<ash::Device>(_ash_device_p)
//     };
// }

#[test]
pub fn test() {
    let _i  = 32;
    dbg!(&_i);
    let _ip = &_i as *const i32 as usize;
    dbg!(&_ip);
    let a = Some(cast_mut!(i32,_ip));
    let b = crate::tool::cast::_cast_ref::<i32>(_ip);
    {
        a.is_some();
        b;
    }
    dbg!(a);
}

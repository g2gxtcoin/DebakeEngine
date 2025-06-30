#[cfg(feature = "env_bit_64bit")]
mod env {
    /// #[derive(MetaSerialize2Toml)]
    /// 过程宏 特征
    /// 将 运行数据 序列化为 toml格式
    pub trait MetaSerialize2Toml {
        fn set_block_name() -> Option<String>{
            return Option::None;
        }
        fn to_toml(&self) -> String;
    }

    /// #[derive(MetaDeSerializeFromToml)]
    /// 过程宏 特征
    /// 将 Toml格式字符串数据 反序列化为 运行数据
    pub trait MetaDeSerializeFromToml {
        fn from_toml(sin:String) -> Self;
    }
}

/*
//the mod witch join task queue should link this mod
//this mod include the data {pub struct MetaDataModule} will tell
//manager mod how to deal with your own moudule

//design mind:
// [custom struct] => [serialize] => [meta data] => [custom to_string] => [String]
// [file] => [String] => [custom prase] => [meta data] => [deserialize]  => [target struct]

//beside implenment serialize will help you
// DSE data struct

/* meta data struct exampl:

key0_0=table{
    key1_0=value
    key1_1=table{
        key2_0=value
        ...
    }
    key1_2=table{
        key2_0=value
        ...
    }
    ...
}

 */

//

// #[macro_export]
// macro_rules! build_in_serialize {
//     ($meta_type:ty,$name:expr) => {
//         <$meta_type>::serialize($name,stringify!($name).replace("self.",""), 0)
//                     .expect_primitive()
//                     .unwrap()
//     };
// }

#[cfg(feature = "config_META_VALUE_TYPE_NUM_32")]
static META_VALUE_TYPE_NUM: i32 = 32;

pub static META_ROOT: MetaD<Option<bool>> = MetaD {
    parent_hash: Option::Some(0),
    key: String::new(),
    value: Option::None,
};

pub static META_ROOT_HASH: u64 = 0;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ptr,
};

pub enum MetaResult<P, C> {
    Err,
    PreImplement(P),
    CustomStruct(C),
}

pub trait MetaBase
where
    Self: Sized,
{
    type ValueType: Default + MetaBase + Sized;

    // type ValueLayout: MetaBase + Sized=Option<bool>;
    fn is_custom_struct() -> bool;
    // over write it when you need to serialize a struct into meta as table
    // this func will consum itself
    fn serialize_table(self) -> Result<Self::ValueType, ()> {
        return Err(crate::log::sorry(
            crate::log::code::TYPE_MEAT_ERROR
                | crate::log::code::CONDI_FUNC_NOT_IMPLENMENT
                | crate::log::code::FILE_META
                | crate::log::LogCodeD::new()
                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                    .get_code()
                | crate::log::LogCodeD::new()
                    .encode(0 as u128, crate::log::LogCodePart::Id)
                    .get_code(),
        ));
    }

    #[cfg(feature = "os_info_64bit")]
    fn get_offset_byte() -> usize {
        if std::mem::size_of::<Self>() < 32 {
            return 4 + 32;
        } else if Self::is_custom_struct() {
            return std::mem::size_of::<Self>() / 8;
        } else {
            return (std::mem::size_of::<Self>() + 32) / 8;
        }
    }
}

// using derive macro init it but your hand
pub trait MetaInnerIter
where
    Self: Sized + MetaBase,
{
    #[cfg(feature = "config_META_VALUE_TYPE_NUM_32")]
    dse_macros::proc_macro_build_meta_inner_iter_type_num!(32);

    #[cfg(feature = "config_META_VALUE_TYPE_NUM_64")]
    dse_macros::proc_macro_build_meta_inner_iter_type_num!(64);

    #[cfg(feature = "config_META_VALUE_TYPE_NUM_16")]
    dse_macros::proc_macro_build_meta_inner_iter_type_num!(16);

    fn iter(&mut self) -> Result<MetaIterator<Self>, ()> {
        if Self::is_custom_struct() {
            todo!()
        } else {
            return Err(crate::log::sorry(
                crate::log::code::TYPE_MEAT_ERROR
                    | crate::log::code::CONDI_FUNC_NOT_IMPLENMENT
                    | crate::log::code::FILE_META
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(0 as u128, crate::log::LogCodePart::Id)
                        .get_code()
            ));
        }
    }
}

pub trait MetaBasePreImplement {}

pub trait Serialize
where
    Self: Sized,
{
    type MetaType;
    type MetaTableType;

    // data to meta
    fn serialize(
        content: Self,
        key: String,
        parent: u64,
    ) -> MetaResult<Self::MetaType, Self::MetaTableType> {
        crate::send2logger_dev!(
            crate::log::code::TYPE_MEAT_WARN
                | crate::log::code::CONDI_FUNC_NOT_IMPLENMENT
                | crate::log::code::FILE_META
                | crate::log::LogCodeD::new()
                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                    .get_code()
                | crate::log::LogCodeD::new()
                    .encode(0 as u128, crate::log::LogCodePart::Id)
                    .get_code()
        );
        return MetaResult::Err;
    }
}

pub trait DeSerialize
where
    Self: Sized,
{
    type TargetType;

    // meta to data
    // it will consume meta data
    fn deserialize(self) -> Result<Self::TargetType, ()> {
        return Err(crate::log::sorry(
            crate::log::code::TYPE_MEAT_WARN
                | crate::log::code::CONDI_FUNC_NOT_IMPLENMENT
                | crate::log::code::FILE_META
                | crate::log::LogCodeD::new()
                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                    .get_code()
                | crate::log::LogCodeD::new()
                    .encode(0 as u128, crate::log::LogCodePart::Id)
                    .get_code(),
        ));
    }

    fn deserialize_from(target: Self) -> Result<Self::TargetType, ()> {
        return Err(crate::log::sorry(
            crate::log::code::TYPE_MEAT_WARN
                | crate::log::code::CONDI_FUNC_NOT_IMPLENMENT
                | crate::log::code::FILE_META
                | crate::log::LogCodeD::new()
                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                    .get_code()
                | crate::log::LogCodeD::new()
                    .encode(0 as u128, crate::log::LogCodePart::Id)
                    .get_code(),
        ));
    }

    // meta to data
    // it will not consume meta data
    // you can use it as prefabe
    fn deserialize_clone(&self) -> Result<Self::TargetType, ()> {

        return Err(crate::log::sorry(
            crate::log::code::TYPE_MEAT_WARN
                | crate::log::code::CONDI_FUNC_NOT_IMPLENMENT
                | crate::log::code::FILE_META
                | crate::log::LogCodeD::new()
                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                    .get_code()
                | crate::log::LogCodeD::new()
                    .encode(0 as u128, crate::log::LogCodePart::Id)
                    .get_code(),
        ));
    }

    unsafe fn deserialize_cast_as_ptr<T>(&self) -> Result<*const T, String> {
        let p: *const Self = ptr::addr_of!(*self);
        return Ok(p as *const T);
    }

    unsafe fn deserialize_cast_as_mut_ptr<T>(&mut self) -> Result<*const T, String> {
        let p: *const Self = ptr::addr_of_mut!(*self);
        return Ok(p as *const T);
    }
}

// warning: because build_in meta struct need a inherit value layout to iterate itself
// the data wrap in meta struct will not as same layout as before; compiler will not relay value as well
#[derive(Debug)]
#[repr(C)]
pub struct MetaD<TData> {
    pub parent_hash: Option<u64>,
    pub key: String,
    pub value: TData,
}

pub struct MetaIterator<T> {
    data: T,
    ptr_offset: usize,
    count: usize,
}

#[cfg(feature = "config_META_VALUE_TYPE_NUM_32")]
impl<T> Iterator for MetaIterator<T>
where
    T: MetaBase + MetaInnerIter,
{
    type Item = *const u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count + 1;
        match self.ptr_offset {
            _ => {
                return Option::None;
            }
        }
    }
}

impl<T> Default for MetaIterator<T>
where
    T: Default,
{
    fn default() -> Self {
        return Self {
            data: Default::default(),
            ptr_offset: Default::default(),
            count: Default::default(),
        };
    }
}

impl<P, C> MetaResult<P, C> {
    pub fn expect_pre_implement(self) -> Result<P, ()> {
        match self {
            MetaResult::Err => {

                return Err(crate::log::sorry(
                    crate::log::code::TYPE_MEAT_ERROR
                        | crate::log::code::CONDI_UNEXPECTED_RESULT
                        | crate::log::code::FILE_META
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(0 as u128, crate::log::LogCodePart::Id)
                            .get_code(),
                ));
            }
            MetaResult::PreImplement(val) => Ok(val),
            MetaResult::CustomStruct(_) => {

                return Err(crate::log::sorry(
                    crate::log::code::TYPE_MEAT_ERROR
                        | crate::log::code::CONDI_UNEXPECTED_RESULT
                        | crate::log::code::FILE_META
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(0 as u128, crate::log::LogCodePart::Id)
                            .get_code(),
                ));
            }
        }
    }

    pub fn expect_custom_struct(self) -> Result<C, ()> {
        match self {
            MetaResult::Err => {
                return Err(crate::log::sorry(
                    crate::log::code::TYPE_MEAT_ERROR
                        | crate::log::code::CONDI_UNEXPECTED_RESULT
                        | crate::log::code::FILE_META
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(0 as u128, crate::log::LogCodePart::Id)
                            .get_code(),
                ));
            }
            MetaResult::CustomStruct(val) => Ok(val),
            MetaResult::PreImplement(_) => {
                return Err(crate::log::sorry(
                    crate::log::code::TYPE_MEAT_ERROR
                        | crate::log::code::CONDI_UNEXPECTED_RESULT
                        | crate::log::code::FILE_META
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(0 as u128, crate::log::LogCodePart::Id)
                            .get_code(),
                ));
            }
        }
    }
}

impl<TData> Default for MetaD<TData>
where
    TData: Default,
{
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            parent_hash: Option::None,
        }
    }
}

impl<Tdata> MetaD<Tdata>
where
    Tdata: Default + MetaBase,
{
    pub fn get_hash(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();
        (self
            .key
            .push_str(self.parent_hash.unwrap().to_string().as_ref()))
        .hash(&mut hasher);
        return hasher.finish();
    }

    pub fn build() -> Self {
        return Self::default();
    }

    pub fn build_key(mut self, key: String) -> Self {
        self.key = key;
        return self;
    }

    pub fn build_value(mut self, value: Tdata) -> Self {
        self.value = value;
        return self;
    }

    pub fn build_set_parent(mut self, hash_in: u64) -> Self {
        self.parent_hash = Option::Some(hash_in);
        return self;
    }

    pub fn build_set_table_parent(mut self) -> Self {
        return self;
    }

    pub fn get_value(&self) -> Result<&Tdata, String> {
        return Ok(&self.value);
    }

    pub fn iter() -> MetaIterator<Tdata> {
        todo!()
    }
}

#[warn(unused)]
pub mod serialize_impl {
    use super::{DeSerialize, MetaBase, MetaBasePreImplement, MetaD, MetaResult, Serialize};
    //
    impl MetaBase for Option<bool> {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }

    impl MetaBase for u8 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for u16 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for u32 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for u64 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for u128 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for usize {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for f32 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for f64 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for bool {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for char {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for i8 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for i16 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for i32 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for i64 {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }
    impl MetaBase for String {
        type ValueType = Self;
        fn is_custom_struct() -> bool {
            return false;
        }
    }

    impl MetaBasePreImplement for String {}
    impl MetaBasePreImplement for u8 {}
    impl MetaBasePreImplement for u16 {}
    impl MetaBasePreImplement for u32 {}
    impl MetaBasePreImplement for u64 {}
    impl MetaBasePreImplement for u128 {}
    impl MetaBasePreImplement for usize {}
    impl MetaBasePreImplement for f32 {}
    impl MetaBasePreImplement for f64 {}
    impl MetaBasePreImplement for bool {}
    impl MetaBasePreImplement for char {}
    impl MetaBasePreImplement for i8 {}
    impl MetaBasePreImplement for i16 {}
    impl MetaBasePreImplement for i32 {}
    impl MetaBasePreImplement for i64 {}
    impl<T> MetaBasePreImplement for Vec<T> {}

    #[cfg(feature = "config_META_ARRAY_MAX_IMPL_NUM_128")]
    dse_macros::proc_macro_build_array_impl_metabase!(128);

    #[cfg(feature = "config_META_ARRAY_MAX_IMPL_NUM_256")]
    dse_macros::proc_macro_build_array_impl_metabase!(256);

    #[cfg(feature = "config_META_ARRAY_MAX_IMPL_NUM_64")]
    dse_macros::proc_macro_build_array_impl_metabase!(64);

    impl<T> MetaBase for Vec<T>
    where
        T: Default + MetaBase,
    {
        type ValueType = T;
        fn is_custom_struct() -> bool {
            return false;
        }
    }

    impl<T> DeSerialize for MetaD<T>
    where
        T: Default + MetaBase + MetaBasePreImplement,
    {
        type TargetType = T;
        fn deserialize(self) -> Result<Self::TargetType, ()> {
            return Ok(self.value);
        }

        fn deserialize_from(target: Self) -> Result<Self::TargetType, ()> {
            return Ok(target.value);
        }

        // fn deserialize_clone(&self) -> Result<Self::TargetType, String> {
        //     return Ok(self.value);
        // }
    }

    impl<T> Serialize for T
    where
        T: Default + MetaBase,
    {
        type MetaType = MetaD<T>;
        type MetaTableType = MetaD<T::ValueType>;

        // serialize specify struct to metadata
        // will consum the struct
        fn serialize(
            content: Self,
            key: String,
            parent: u64,
        ) -> super::MetaResult<Self::MetaType, Self::MetaTableType> {
            match T::is_custom_struct() {
                true => super::MetaResult::CustomStruct(
                    MetaD::<T::ValueType>::build()
                        .build_key(key)
                        .build_value(content.serialize_table().unwrap())
                        .build_set_parent(parent)
                        .build_set_table_parent(),
                ),
                false => super::MetaResult::PreImplement(
                    MetaD::build()
                        .build_key(key)
                        .build_value(content)
                        .build_set_parent(parent),
                ),
            }
        }
    }
}

*/

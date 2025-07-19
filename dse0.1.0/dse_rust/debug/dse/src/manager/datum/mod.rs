#[cfg(feature = "config_DATUM_DEFAULT_CAPACITY_0")]
static DATUM_DEFAULT_CAPACITY: usize = 0;

#[cfg(feature = "config_DATUM_DEFAULT_CAPACITY_4")]
static DATUM_DEFAULT_CAPACITY: usize = 4;

#[cfg(feature = "config_DATUM_DEFAULT_CAPACITY_8")]
static DATUM_DEFAULT_CAPACITY: usize = 8;

#[cfg(feature = "config_DATUM_DEFAULT_CAPACITY_16")]
static DATUM_DEFAULT_CAPACITY: usize = 16;

#[cfg(feature = "config_DATUM_DEFAULT_LOCK_SIZE_1")]
static DATUM_LOCK_SIZE: bool = true;
#[cfg(feature = "config_DATUM_DEFAULT_LOCK_SIZE_0")]
static DATUM_DEFAULT_LOCK_SIZE: bool = false;

// An exe can only bind an index, but an index can link not only an exe

#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::fmt::Debug;

    use crate::{get, get_mut};

    use super::{DATUM_DEFAULT_CAPACITY, DATUM_DEFAULT_LOCK_SIZE};

    pub enum AllocResult {
        None,
        Index(usize),
    }
    impl AllocResult {
        pub fn index(self) -> usize {
            match self {
                AllocResult::None => usize::MAX,
                AllocResult::Index(val) => val,
            }
        }
        pub fn end(self) {}
    }

    #[repr(C, align(4))]
    pub struct Datum<DT> {
        pub id: u64,
        pub parent_id: u64,
        data: Vec<Option<DT>>,
        pub register: Vec<(u64, usize)>, // exe_index register
        is_size_lock: bool,
    }

    pub struct DatumIter<'a, DT> {
        data: &'a Datum<DT>,
        offset: usize,
    }

    #[cfg(feature = "log_mode_dev")]
    impl<DT> Debug for Datum<DT>
    where
        DT: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Datum")
                .field("id", &self.id)
                .field("parent_id", &self.parent_id)
                .field("data", &self.data)
                .field("register", &self.register)
                .field("is_size_lock", &self.is_size_lock)
                .finish()
        }
    }

    #[cfg(feature = "log_mode_rt")]
    impl<DT> Debug for Datum<DT>
    where
        DT: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl<'a, DT> Iterator for DatumIter<'a, DT> {
        type Item = &'a DT;

        fn next(&mut self) -> Option<Self::Item> {
            if self.offset >= self.data.data.len() {
                self.offset = 0;
                return Option::None;
            } else {
                let _r = get!(self.data.data, self.offset).as_ref().unwrap();
                self.offset = self.offset + 1;
                return Some(_r);
            }
        }
    }

    impl<DT> Datum<DT> {
        pub fn vec_ref(&self) -> &Vec<Option<DT>> {
            return &self.data;
        }

        pub fn vec_mut(&mut self) -> &mut Vec<Option<DT>> {
            return &mut self.data;
        }

        pub fn iter(&self) -> DatumIter<'_, DT> {
            return DatumIter::<DT> {
                data: self,
                offset: 0,
            };
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<Option<DT>> {
            return self.data.iter_mut();
        }

        pub fn build_with_capacity(mut self, size: usize) -> Self {
            self.data = Vec::with_capacity(size);
            self.register = Vec::with_capacity(size);
            return self;
        }

        pub fn build_empty() -> Self {
            return Self::default().build_with_capacity(0);
        }

        pub fn build_push_data(
            mut self,
            initdata_in: DT,
            exe_id: u64, /*u64::MAX mean not bind exe */
        ) -> Self {
            if self.is_size_lock {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_ERROR
                        | crate::log::code::CONDI_DAT_SIZE_LOCK_PUSH_FAIL
                        | crate::log::code::FILE_MANAGER_DATUM
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                            .get_code()
                )
            } else {
                self.data.push(Option::Some(initdata_in));
                if exe_id == u64::MAX {
                } else {
                    self.bind_exe_id2data(exe_id, self.data.len() - 1);
                }
            }
            return self;
        }

        pub fn alloc_data(
            &mut self,
            initdata_in: DT,
            exe_id: Option<u64>, /*none mean not bind exe */
        ) -> AllocResult {
            let mut index: Option<usize> = Option::None;
            match self.data.iter().enumerate().find(|x| x.1.is_none()) {
                Some(eval) => {
                    index = Option::Some(eval.0);
                }
                None => {}
            }
            match index {
                Some(val) => {
                    *get_mut!(self.data, index.unwrap()) = Option::Some(initdata_in);
                    if exe_id.is_some() {
                        self.bind_exe_id2data(exe_id.unwrap(), val);
                    }
                    return AllocResult::Index(index.unwrap());
                }
                None => {
                    if self.is_size_lock {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_ERROR
                                | crate::log::code::CONDI_DAT_SIZE_LOCK_PUSH_FAIL
                                | crate::log::code::FILE_MANAGER_DATUM
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                    .get_code()
                        )
                    } else {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_WARN
                                | crate::log::code::CONDI_MEM_LEAK_DAT_REALLOC
                                | crate::log::code::FILE_MANAGER_DATUM
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                    .get_code()
                        );
                        self.data.push(Option::Some(initdata_in));
                        if exe_id.is_some() {
                            self.bind_exe_id2data(exe_id.unwrap(), self.data.len() - 1);
                        }
                        return AllocResult::Index(self.data.len() - 1);
                    }
                }
            }
            return AllocResult::None;
        }

        pub fn alloc_data_with_lambda<Func>(
            &mut self,
            initdata_in: DT,
            exe_id: u64, /*u64::MAX mean not bind exe */
            function_in: Func,
        ) where
            Func: FnMut() -> (),
        {
        }

        pub fn release_data(&mut self, index: usize) {
            *get_mut!(self.data, index) = Option::None;
        }

        pub fn release_exe_data(&mut self, id_in: u64) {
            match self.register.iter().find(|&&x| x.0 == id_in) {
                Some(val) => *get_mut!(self.data, val.1) = Option::None,
                None => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_ERROR
                            | crate::log::code::CONDI_INVAILD_EXE_ID
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    )
                }
            };
        }

        //return first value mut ref
        pub fn back_mut(&mut self) -> Result<&mut DT, ()> {
            match self.data.is_empty() {
                true => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ))
                }
                false => return Ok(self.data[0].as_mut().unwrap()),
            }
        }

        //return first value mut ref
        pub fn back_ref(&self) -> Result<&DT, ()> {
            match self.data.is_empty() {
                true => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ))
                }
                false => return Ok(self.data[0].as_ref().unwrap()),
            }
        }

        /// 通过exeid 获取数据集自身索引
        pub fn get_data_index(&self, id_in: u64) -> Result<usize, ()> {
            match self.register.iter().find(|&&x| x.0 == id_in) {
                Some(val) => return Ok(val.1),
                None => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_ERROR
                            | crate::log::code::CONDI_INVAILD_EXE_ID
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ))
                }
            };
        }

        /// 通过exeid 获取数据集自身可变引用
        pub fn get_data_mut(&mut self, exeid_in: u64) -> Result<&mut DT, ()> {
            match self.register.iter().find(|&&x| x.0 == exeid_in) {
                Some(val) => return Ok(get_mut!(self.data, val.1).as_mut().unwrap()),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ))
                }
            };
        }

        /// 通过exeid 获取数据集自身引用
        pub fn get_data_ref(&self, exe_id: u64) -> Result<&DT, ()> {
            match self.register.iter().find(|&&x| x.0 == exe_id) {
                Some(val) => return Ok(unsafe { get!(self.data, val.1).as_ref().unwrap() }),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ))
                }
            };
        }

        /// 将特定索引上数据 绑定到执行上
        pub fn bind_exe_id2data(&mut self, exe_id: u64, data_index: usize) {
            // make sure register only once
            let mut is_only: bool = true;
            match self.register.iter().find_map(|(exe, index)| {
                if exe.to_owned() == exe_id {
                    return Option::Some(index.to_owned());
                } else {
                    return Option::None;
                }
            }) {
                Some(val) => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_CORE_WARN
                            | crate::log::code::CONDI_ONLY_DATA_EXIST
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    );
                    is_only = false;
                }
                None => {}
            }
            if is_only {
                self.register.push((exe_id, data_index));
            };
        }

        pub fn into_simd(&self) {}

        pub fn as_mut(&mut self) -> &mut Self {
            return self;
        }

        pub fn as_ref(&self) -> &Self {
            return self;
        }
    }

    impl<DT> Default for Datum<DT> {
        fn default() -> Self {
            return Self {
                id: 0,
                data: Vec::with_capacity(DATUM_DEFAULT_CAPACITY),
                register: Vec::with_capacity(DATUM_DEFAULT_CAPACITY),
                parent_id: 0,
                is_size_lock: DATUM_DEFAULT_LOCK_SIZE,
            };
        }
    }
}

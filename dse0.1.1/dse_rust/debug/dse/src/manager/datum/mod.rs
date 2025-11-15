#[cfg(feature = "config_DATUM_DEFAULT_CAPACITY_0")]
static DATUM_DEFAULT_CAPACITY: usize =  0;

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

        heap: Vec<u16>,
        block: Vec<DT>,
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
                .field("heap", &self.heap)
                .field("data", &self.block)
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
            if self.offset >= self.data.block.len() {
                self.offset = 0;
                return Option::None;
            } else {
                while self.data.is_empty_index(self.offset) {
                    self.offset = self.offset + 1;
                    if self.offset >= self.data.block.len() {
                        self.offset = 0;
                        return Option::None;
                    }
                }

                let _r = get!(self.data.block, self.offset);
                self.offset = self.offset + 1;
                return Some(_r);
            }
        }
    }

    impl<DT> Datum<DT> {
        pub fn is_empty(&self) -> bool {
            return self.block.is_empty();
        }

        pub fn is_empty_index(&self, index: usize) -> bool {
            let _index_vec = (index | 0xFFF0) >> 16;
            let _index_u16 = index | 0xF;
            let _res: bool = (*get!(self.heap, _index_vec) | (0b1 << _index_u16)) == 0;

            return _res;
        }

        // warning:
        pub fn vec_ref(&self) -> &Vec<DT> {
            let _r = &self.block;
            return _r;
            //return &self.data;
        }

        // warning:
        pub fn vec_mut(&mut self) -> &mut Vec<DT> {
            let _r = &mut self.block;
            return _r;
            //return &self.data;
        }

        pub fn vec_warp_clone(&self) -> Vec<Option<DT>>
        where
            DT: Clone + Copy,
        {
            let _r: Vec<Option<DT>> = self
                .block
                .iter()
                .enumerate()
                .map(|x| {
                    if self.is_empty_index(x.0) {
                        return Option::None;
                    } else {
                        return Option::Some(*x.clone().1);
                    }
                })
                .collect();
            return _r;
            //return &self.data;
        }

        #[deprecated = "datum struct has been changed. use vec_mut instead"]
        pub fn vec_warp_mut(&mut self) -> Vec<Option<&mut DT>> {
            todo!();
            // let _r: Vec<Option<&mut DT>> = self
            //     .data
            //     .iter_mut()
            //     .enumerate()
            //     .map(|x| {
            //         if self.is_empty_index(x.0) {
            //             return Option::None;
            //         } else {
            //             return Option::Some(x.1);
            //         }
            //     })
            //     .collect();
            // return _r;
        }

        #[deprecated = "datum struct has been changed. use vec_ref or vec_mut instead"]
        pub fn vec_some_clone(&self) -> Vec<DT>
        where
            DT: Clone,
        {
            todo!();
            //let  _r:Vec<DT>  = self.data.iter().filter(|x| x.is_some()).map(|x| x.clone().unwrap()).collect();
            //return _r;
        }

        pub fn iter(&self) -> DatumIter<'_, DT> {
            return DatumIter::<DT> {
                data: self,
                offset: 0,
            };
        }

        pub fn iter_mut(&mut self) -> std::slice::IterMut<DT> {
            return self.block.iter_mut();
        }

        pub fn build_with_capacity(mut self, size: usize) -> Self {
            self.block = Vec::with_capacity(size);
            self.register = Vec::with_capacity(size);
            return self;
        }

        pub fn build_empty() -> Self {
            return Self::default().build_with_capacity(0);
        }

        //
        // #[deprecated = "datum struct has been changed. use alloc_data instead"]
        pub fn build_alloc_data(
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
                self.block.push(initdata_in);
                if exe_id == u64::MAX {
                } else {
                    self.bind_exe_id2data(exe_id, self.block.len() - 1);
                }
            }
            return self;
        }

        fn _alloc_specify_heap(&mut self, index: usize) {
            let _index_vec = (index | 0xFFF0) >> 16;
            let _index_u16 = index | 0xF;
            let _res = *get!(self.heap, _index_vec) ^ (0b1 << _index_u16);
            *get_mut!(self.heap, _index_vec) = _res;
        }

        fn _inactive_specify_heap(&mut self, index: usize) {
            let _index_vec = (index | 0xFFF0) >> 16;
            let _index_u16 = index | 0xF;
            let _res = *get!(self.heap, _index_vec) | (!(0b1 << _index_u16));
            *get_mut!(self.heap, _index_vec) = _res;
        }

        fn _alloc_data2specify_index(
            &mut self,
            target_index: usize,
            initdata_in: DT,
            exe_id: Option<u64>,
        ) {
            if target_index == self.block.len() {
                self.block.push(initdata_in);
            } else if target_index > self.block.len() {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_ERROR
                        | crate::log::code::CONDI_HEAP_ERROR
                        | crate::log::code::FILE_MANAGER_DATUM
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                            .get_code()
                );
                todo!();
            } else {
                *get_mut!(self.block, target_index) = initdata_in;
            }
            self._alloc_specify_heap(target_index);
            if exe_id.is_some() {
                self.bind_exe_id2data(exe_id.unwrap(), target_index);
            }
        }

        //
        pub fn alloc_data(
            &mut self,
            initdata_in: DT,
            exe_id: Option<u64>, /*none mean not bind exe */
        ) -> AllocResult {
            // insert index
            let mut _index: Option<usize> = Option::None;

            for i in 0..self.block.len() {
                if self.is_empty_index(i) {
                    _index = Option::Some(i);
                    break;
                }
            }

            // judge if data is full

            // match self.data.iter().enumerate().find(|x| x.1.is_none()) {
            //     Some(eval) => {
            //         _index = Option::Some(eval.0);
            //     }
            //     None => {}
            // }

            // if self.heap.is_empty(){
            //     self.heap.push(0);
            // }
            if self.heap.is_empty() || *self.heap.last().unwrap() != 0 {
                self.heap.push(0);
            }

            match _index {
                Some(val) => {
                    self._alloc_data2specify_index(val, initdata_in, exe_id);
                    return AllocResult::Index(_index.unwrap());
                }
                None => {
                    //
                    if self.block.len() < self.block.capacity() {
                        self._alloc_data2specify_index(self.block.len(), initdata_in, exe_id);
                        return AllocResult::Index(self.block.len() - 1);
                    }
                    //
                    else if self.is_size_lock {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_ERROR
                                | crate::log::code::CONDI_DAT_SIZE_LOCK_PUSH_FAIL
                                | crate::log::code::FILE_MANAGER_DATUM
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        line!() as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                    )
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        self.id as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                    )
                                    .get_code()
                        )
                    } else {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_DAT_WARN
                                | crate::log::code::CONDI_MEM_LEAK_DAT_REALLOC
                                | crate::log::code::FILE_MANAGER_DATUM
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        line!() as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                    )
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(
                                        self.id as u128,
                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                    )
                                    .get_code()
                        );
                        self._alloc_data2specify_index(self.block.len(), initdata_in, exe_id);
                        return AllocResult::Index(self.block.len() - 1);
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
            todo!();
        }

        pub fn release_data(&mut self, index: usize) {
            self._inactive_specify_heap(index);
            // *get_mut!(self.block, index) = Option::None;
        }

        /// Waring : dont call it when your datum is binding 2 exe
        /// It will cause unexpected behavior， due to mem order changed
        pub fn clear_mem_fragment() {
            todo!();
        }

        pub fn release_exe_data(&mut self, id_in: u64) {
            match self.register.iter().find(|&&x| x.0 == id_in) {
                Some(val) => self._inactive_specify_heap(val.1),
                None => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_DAT_ERROR
                            | crate::log::code::CONDI_INVAILD_EXE_ID
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    )
                }
            };
        }

        //return first value mut ref
        pub fn back_mut(&mut self) -> Result<&mut DT, ()> {
            match self.block.is_empty() {
                true => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                                )
                                .get_code(),
                    ))
                }
                false => return Ok(get_mut!(self.block, 0)),
            }
        }

        //return first value mut ref
        pub fn back_ref(&self) -> Result<&DT, ()> {
            match self.block.is_empty() {
                true => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                                )
                                .get_code(),
                    ))
                }
                false => return Ok(get!(self.block, 0)),
            }
        }

        /// # Abstract
        /// 通过exeid 获取数据集自身索引
        /// ## Example
        /**
         *
         */
        /// ## Parameter
        ///
        /// ## Also see
        /// - 如果需要直接通过索引获取数据引用，请使用
        /**
         * get!(<datum>.vec_ref(),index); //or
         * get_mut!(<datum>.vec_mut(),index);
         */
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
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
                                .get_code()
                    ))
                }
            };
        }

        /// 通过exeid 获取数据集自身可变引用
        pub fn get_data_mut(&mut self, exeid_in: u64) -> Result<&mut DT, ()> {
            match self.register.iter().find(|&&x| x.0 == exeid_in) {
                Some(val) => return Ok(get_mut!(self.block, val.1)),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                                )
                                .get_code(),
                    ))
                }
            };
        }

        /// 通过exeid 获取数据集自身引用
        pub fn get_data_ref(&self, exe_id: u64) -> Result<&DT, ()> {
            match self.register.iter().find(|&&x| x.0 == exe_id) {
                Some(val) => return Ok(unsafe { get!(self.block, val.1) }),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_CORE_ERROR
                            | crate::log::code::CONDI_DATA_NOT_FOUND
                            | crate::log::code::FILE_MANAGER_DATUM
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID,
                                )
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
                                .encode(
                                    self.id as u128,
                                    crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                )
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
            let _heap_vec = vec![0u16; DATUM_DEFAULT_CAPACITY];
            return Self {
                id: 0,
                heap: _heap_vec,
                block: Vec::with_capacity(DATUM_DEFAULT_CAPACITY),
                register: Vec::with_capacity(DATUM_DEFAULT_CAPACITY),
                parent_id: 0,
                is_size_lock: DATUM_DEFAULT_LOCK_SIZE,
            };
        }
    }
}

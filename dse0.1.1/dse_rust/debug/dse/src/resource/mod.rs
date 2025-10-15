pub mod dsepack;

#[cfg(windows)]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "env_os_win10")]
pub mod env {
    use crate::manager::execute::env::TaskQueue;
    use std::{
        fmt::Debug,
        fs::File,
        io::{self, Read, Seek},
        path::PathBuf,
    };

    use walkdir::{self, DirEntry};

    pub enum ResourceTask {
        None,
    }

    #[derive(Debug,Default)]
    pub struct ResourceAttachment {}


    #[derive(Debug)]
    pub struct ResourceE {
        id: u64,
        resource_attachment: ResourceAttachment,
        current_path: PathBuf,
        absolute_path: PathBuf,
    }

    //only once creator
    pub struct FileCreator<T> {
        data: Option<T>,
        file_name: String,
        target_path: Option<PathBuf>,
        file_stream: Option<File>,
    }

    pub enum ResourceLoaderTask {
        None,
    }

    impl ResourceE {
        pub fn sub(&mut self)-> walkdir::WalkDir{
            return walkdir::WalkDir::new(self.current_path.clone());
        }

        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }
        pub fn build() -> Self {
            return Self {
                id: 0,
                current_path: PathBuf::new(),
                absolute_path: PathBuf::new(),
                resource_attachment: Default::default(),
            };
        }

        pub fn build_set_id(mut self, id_in: u64) -> Self {
            self.id = id_in;
            return self;
        }

        pub fn build_current_path(mut self, path_in: PathBuf) -> Self {
            self.current_path = path_in;
            self.absolute_path = self.current_path.clone();
            return self;
        }

        pub fn set_current_path_from_abs_path(&mut self, path_in: PathBuf) {
            self.current_path = self.current_path.join(path_in);
        }

        pub fn set_current_path(&mut self, path_in: PathBuf) {
            self.current_path = path_in;
        }


        /// 任务队列提交函数
        /// 异步加载指定单个文件
        /// 可指定缓存读取位置
        pub fn load_single(&mut self) {}

        /// 执行函数
        /// 异步将磁盘资源加载至 缓冲数据集
        /// 需要指定任务队列
        pub fn exe_load(&mut self) {}

        /// 同步加载指定单个文件
        /// 返回文件流
        /// 不会创建 文件数据缓存
        pub fn load_single_sync(&mut self) -> Result<File, ()> {
            match std::fs::File::open(self.current_path.clone()) {
                Ok(val) => return Ok(val),
                Err(_) => {}
            };
            return Err(crate::log::sorry(
                crate::log::code::TYPE_EXE_ERROR
                    | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                    | crate::log::code::FILE_RESOURCE
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
                    | crate::log::LogCodeD::new()
                        .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                        .get_code(),
            ));
        }

        /// 同步加载当前目录所有文件
        /// 返回资源文件流迭代器
        /// 不会当即创建 文件流以及文件数据缓存
        pub fn load_current_sync(&mut self) -> Result<ResourceFileIter, bool> {
            let _w = walkdir::WalkDir::new(self.current_path.clone())
                .min_depth(0)
                .max_depth(1);
            return Ok(ResourceFileIter::build(_w));
        }

        /// 同步加载当前目录所有文件 并指定后缀
        /// 返回资源文件流迭代器
        /// 不会当即创建 文件流以及文件数据缓存
        pub fn load_current_specify_suffix_sync(
            &mut self,
            suffix: String,
        ) -> Result<ResourceFileIter, bool> {
            let _w = walkdir::WalkDir::new(self.current_path.clone())
                .min_depth(0)
                .max_depth(1);
            return Ok(ResourceFileIter::build(_w).build_suffix(suffix));
        }

        /// 同步加载当前目录开始所有文件(包含子目录)
        /// 返回资源文件流迭代器
        /// 不会当即创建 文件流以及文件数据缓存
        pub fn load_all_sync(&mut self) -> Result<ResourceFileIter, bool> {
            let _w = walkdir::WalkDir::new(self.current_path.clone())
                .min_depth(0)
                .max_depth(usize::MAX);
            return Ok(ResourceFileIter::build(_w));
        }

        pub fn load_all_specify_suffix_sync(&mut self) -> Result<ResourceFileIter, bool> {
            let _w = walkdir::WalkDir::new(self.current_path.clone())
                .min_depth(0)
                .max_depth(usize::MAX);
            return Ok(ResourceFileIter::build(_w));
        }
    }

    /// 资源管理器 文件迭代器
    ///
    pub struct ResourceFileIter {
        path_iter: walkdir::IntoIter,
        suffix: Option<String>,
        result_type: ResourceFileIterResult,
        begin: u64,
        end: u64,
    }

    pub enum ResourceFileIterResult {
        FileStream(Option<File>),
        FileBuffer(Vec<u8>),
        FileFlagBuffer(Vec<u8>),
    }

    impl ResourceFileIterResult {
        pub fn expect_stream(self) -> Result<File, ()> {
            match self {
                ResourceFileIterResult::FileStream(val) => match val {
                    Some(fv) => return Ok(fv),
                    None => {
                        return Err(crate::send2logger_dev!(
                            crate::log::code::TYPE_EXE_ERROR
                                | crate::log::code::CONDI_OPTION_NONE
                                | crate::log::code::FILE_RESOURCE
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                    .get_code()
                        ))
                    }
                },
                _ => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_UNEXPECTED_RESULT
                            | crate::log::code::FILE_RESOURCE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0 as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ))
                }
            }
        }

        pub fn expect_buffer(self) -> Result<Vec<u8>, ()> {
            match self {
                ResourceFileIterResult::FileBuffer(val) => return Ok(val),
                ResourceFileIterResult::FileFlagBuffer(val) => return Ok(val),
                _ => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_UNEXPECTED_RESULT
                            | crate::log::code::FILE_RESOURCE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ))
                }
            }
        }
    }

    impl ResourceFileIter {
        // default all file
        pub fn build(pin: walkdir::WalkDir) -> Self {
            return Self {
                path_iter: pin.into_iter(),
                suffix: Option::None,
                result_type: ResourceFileIterResult::FileStream(Option::None),
                begin: 0,
                end: u64::MAX,
            };
        }

        pub fn build_suffix(mut self, suffix: String) -> Self {
            self.suffix = Option::Some(suffix);
            return self;
        }

        pub fn build_file_buffer_offset(mut self, begin_byte: u64, end_byte: u64) -> Self {
            self.begin = begin_byte;
            self.end = end_byte;
            return self;
        }

        pub fn build_result_type(mut self, ty: ResourceFileIterResult) -> Self {
            match ty {
                ResourceFileIterResult::FileStream(_) => {
                    self.result_type = ResourceFileIterResult::FileStream(Option::None)
                }
                ResourceFileIterResult::FileBuffer(_) => {
                    self.result_type = ResourceFileIterResult::FileBuffer(Default::default())
                }
                ResourceFileIterResult::FileFlagBuffer(_) => {
                    self.result_type = ResourceFileIterResult::FileFlagBuffer(Default::default())
                }
                _ => {
                    todo!()
                }
            }
            return self;
        }

        fn _iter_return_file_stream(&mut self) -> Option<Option<ResourceFileIterResult>> {
            match self.path_iter.next() {
                Some(val) => {
                    let _path = val.as_ref().unwrap();
                    let is_satisfy = match self.suffix.is_some() {
                        true => _path
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .contains(self.suffix.as_ref().unwrap()),
                        false => true,
                    };
                    if is_satisfy {
                        match std::fs::File::open(_path.to_owned().path()) {
                            Ok(val) => {
                                return Option::Some(Option::Some(
                                    ResourceFileIterResult::FileStream(Option::Some(val)),
                                ))
                            }
                            Err(_) => crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_ERROR
                                    | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(0xffff as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                        .get_code()
                            ),
                        };
                    } else {
                        return Option::Some(Option::None);
                    }
                }
                None => {}
            }
            return Option::None;
        }

        fn _iter_return_file_buffer(&mut self) -> Option<Option<ResourceFileIterResult>> {
            match self.path_iter.next() {
                Some(val) => {
                    let _path = val.as_ref().unwrap();
                    let is_satisfy = match self.suffix.is_some() {
                        true => _path
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .contains(self.suffix.as_ref().unwrap()),
                        false => true,
                    };
                    if is_satisfy {
                        match std::fs::File::open(_path.to_owned().path()) {
                            Ok(val) => {
                                let mut _buf = io::BufReader::new(val);
                                let mut _r: Vec<u8> = Default::default();
                                match _buf.read_to_end(_r.as_mut()) {
                                    Ok(val) => {
                                        crate::send2logger_dev!(
                                            crate::log::code::TYPE_EXE_INFO
                                                | crate::log::code::CONDI_DEFAULT
                                                | crate::log::code::FILE_RESOURCE
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        line!() as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                                    )
                                                    .get_code()
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        0xffff as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                                    )
                                                    .get_code()
                                        );
                                        return Option::Some(Option::Some(
                                            ResourceFileIterResult::FileBuffer(_r),
                                        ));
                                    }
                                    Err(_) => {
                                        crate::send2logger_dev!(
                                            crate::log::code::TYPE_EXE_ERROR
                                                | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                                | crate::log::code::FILE_RESOURCE
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        line!() as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                                    )
                                                    .get_code()
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        0xffff as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                                    )
                                                    .get_code()
                                        )
                                    }
                                };
                            }
                            Err(_) => crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_ERROR
                                    | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(0xffff as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                        .get_code()
                            ),
                        };
                    } else {
                        return Option::Some(Option::None);
                    }
                }
                None => {}
            }
            return Option::None;
        }

        /// 现阶段没用
        /// 这就留个接口
        /// 由于rust 的文件缓冲光标不够完善
        /// 这玩意的本质还是将整个外存文件数据
        /// 存入内存然后在拆分
        /// 还是老老实实将文件拆分吧
        fn _iter_return_file_flag_buffer(&mut self) -> Option<Option<ResourceFileIterResult>> {
            match self.path_iter.next() {
                Some(val) => {
                    let _path = val.as_ref().unwrap();
                    let is_satisfy = match self.suffix.is_some() {
                        true => _path
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .contains(self.suffix.as_ref().unwrap()),
                        false => true,
                    };
                    if is_satisfy {
                        match std::fs::File::open(_path.to_owned().path()) {
                            Ok(val) => {
                                let mut _buf = io::BufReader::new(val);
                                let mut _r: Vec<u8> = Default::default();
                                match _buf.read_to_end(_r.as_mut()) {
                                    Ok(val) => {
                                        crate::send2logger_dev!(
                                            crate::log::code::TYPE_EXE_INFO
                                                | crate::log::code::CONDI_DEFAULT
                                                | crate::log::code::FILE_RESOURCE
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        line!() as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                                    )
                                                    .get_code()
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        0xffff as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                                    )
                                                    .get_code()
                                        );
                                        if val > self.end as usize {
                                            return Option::Some(Option::Some(
                                                ResourceFileIterResult::FileBuffer(
                                                    _r[(self.begin as usize)..(self.end as usize)]
                                                        .to_vec(),
                                                ),
                                            ));
                                        } else {
                                            crate::send2logger_dev!(
                                                crate::log::code::TYPE_EXE_ERROR
                                                    | crate::log::code::CONDI_NUM_OVERFLOW
                                                    | crate::log::code::FILE_RESOURCE
                                                    | crate::log::LogCodeD::new()
                                                        .encode(
                                                            line!() as u128,
                                                            crate::log::LogPartFlag::LOGGER_PART_LINE
                                                        )
                                                        .get_code()
                                                    | crate::log::LogCodeD::new()
                                                        .encode(
                                                            0xffff as u128,
                                                            crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                                        )
                                                        .get_code()
                                            )
                                        }
                                    }
                                    Err(_) => {
                                        crate::send2logger_dev!(
                                            crate::log::code::TYPE_EXE_ERROR
                                                | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                                | crate::log::code::FILE_RESOURCE
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        line!() as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_LINE
                                                    )
                                                    .get_code()
                                                | crate::log::LogCodeD::new()
                                                    .encode(
                                                        0xffff as u128,
                                                        crate::log::LogPartFlag::LOGGER_PART_EXE_ID
                                                    )
                                                    .get_code()
                                        )
                                    }
                                };
                            }
                            Err(_) => crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_ERROR
                                    | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(0xffff as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                        .get_code()
                            ),
                        };
                    } else {
                        return Option::Some(Option::None);
                    }
                }
                None => {}
            }
            return Option::None;
        }
    }

    impl Iterator for ResourceFileIter {
        type Item = Option<ResourceFileIterResult>;

        // Option::None means not correct file suffix
        fn next(&mut self) -> Option<Self::Item> {
            match self.result_type {
                ResourceFileIterResult::FileStream(_) => return self._iter_return_file_stream(),
                ResourceFileIterResult::FileBuffer(_) => return self._iter_return_file_buffer(),
                ResourceFileIterResult::FileFlagBuffer(_) => {
                    return self._iter_return_file_flag_buffer()
                }
                _ => {
                    todo!()
                }
            }
        }
    }

    impl Default for ResourceE {
        fn default() -> Self {
            return Self::build();
        }
    }

    impl<T> FileCreator<T> {
        pub fn build(data: T) -> Self {
            Self {
                data: Option::Some(data),
                file_name: Default::default(),
                target_path: Option::Some(std::env::current_dir().unwrap()),
                file_stream: Option::None,
            }
        }
        pub fn build_set_path(mut self, path: PathBuf) -> Self {
            self.target_path = Option::Some(path);
            return self;
        }
        pub fn build_set_file_name(mut self, name: String) -> Self {
            self.file_name = name;
            return self;
        }
        pub fn build_link_resource_excute(self, re: &ResourceE) -> Self {
            return self;
        }
        pub fn build_load_stencil(self) -> Self {
            return self;
        }
        pub fn create(self) {}
    }
}

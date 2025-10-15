pub mod bounding;
pub mod coord;
pub mod mesh;
pub mod rectangle;
pub mod texture;
pub mod transform;

///////////////////////////////////////
///                                 ///
///    没完成 要大改  确保可拓展性    ///
///                                 ///
///////////////////////////////////////

#[cfg(feature = "config_MODEL_MAX_ATTECHMENT_NUM_64")]
pub static MODEL_MAX_ATTECHMENT_NUM: usize = 64;
#[cfg(feature = "config_MODEL_MAX_ATTECHMENT_NUM_32")]
pub static MODEL_MAX_ATTECHMENT_NUM: usize = 32;
#[cfg(feature = "config_MODEL_MAX_ATTECHMENT_NUM_16")]
pub static MODEL_MAX_ATTECHMENT_NUM: usize = 16;
#[cfg(feature = "config_MODEL_MAX_ATTECHMENT_NUM_8")]
pub static MODEL_MAX_ATTECHMENT_NUM: usize = 8;

// 该模块静态序列化模块
// 主要用于将 main下 tak exe dat buf 四大基本模块中子变量
// 等标序到运行内存中
// ！！！！！！临时模块，测试使用，因改变频繁,后续将使用静态宏完成！！！！！
// todo!();
#[allow(unused)]
pub mod mtid {
    // exe model type id
    pub const MTID_EXE_TIMER: u64 = 0;
    pub const MTID_EXE_WIN_INPUT: u64 = 1;
    pub const MTID_EXE_INPUT: u64 = 2;
    pub const MTID_EXE_WIN_WINDOW: u64 = 3;
    pub const MTID_EXE_RESOURCE_LOADER: u64 = 4;
    pub const MTID_EXE_SHADER_DECODER: u64 = 5;
    pub const MTID_EXE_MODEL: u64 = 6;
    pub const MTID_EXE_RENDERER1: u64 = 7;
    pub const MTID_EXE_RENDER_CMD: u64 = 8;

    // dat model type id
    pub const MTID_DAT_APPLICATION: u64 = 0;
    pub const MTID_DAT_VK_API: u64 = 1;
    pub const MTID_DAT_GRAPHIC_RENDERER_PIPELINE: u64 = 2;
    pub const MTID_DAT_CMD_BUFFER: u64 = 3;
    pub const MTID_DAT_FRAME_BUF: u64 = 4;
    pub const MTID_DAT_VERTEX_BUF: u64 = 5;
    pub const MTID_DAT_SURFACE_IMG: u64 = 6;
    pub const MTID_DAT_SHADER_MOD: u64 = 7;
    pub const MTID_DAT_MODEL: u64 = 8;
    pub const MTID_DAT_TRANSFORM: u64 = 9;
    pub const MTID_DAT_MESH: u64 = 10;
    pub const MTID_DAT_RENDER_FNECE: u64 = 11;

    // tak model type id
    pub const MTID_TAK_RENDER_TASK: u64 = 0;
    pub const MTID_TAK_DECODER_TASK: u64 = 1;
    pub const MTID_TAK_RENDERCMD_TASK: u64 = 2;
}

#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_os_win10")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
pub mod env {
    use crate::{
        ________________dev_stop________________,
        manager::{
            self,
            datum::{self, env::Datum},
            execute::{env::TaskQueue, template::call_back_template::Callback0MR0R},
        },
        node::env::{NodeD, NodeT},
    };

    use super::{transform::env::TransformD, MODEL_MAX_ATTECHMENT_NUM};

    pub enum ModelTask {
        None,
        UpdateGlobalTransform(
            usize, //dat model index
            Callback0MR0R,
        ),
        EulerRotate(),
        ResetQuaternion(),
        Dispalce(),
        Scale(),
    }

    #[repr(C, align(8))]
    pub struct ModelExeAttachment {
        index_transform_task: usize,
    }

    /// # Abstract
    /// 模型附件
    /// ## example
    /**
     */
    /// ## parameter
    /**
     * attachment_index_buffer: 用于记录模型的附件类型 与 索引信息,
     * index_child_offset: 内部变量 用于记录,
     * index_offset: usize : 内部变量 用于记录,
     * switch_transform_update: bool,
     * is_active: bool
     */
    #[repr(C, align(8))]
    #[derive(Debug)]

    pub struct ModelAttachment {
        pub attechments_index_buffer: Vec<(u64, usize)>,
        index_child_offset: usize,
        index_offset: usize,

        switch_transform_update: bool,

        is_active: bool,
    }

    #[derive(Default)]
    #[repr(C, align(8))]
    pub struct ModelE {
        pub attachment: ModelExeAttachment,
        pub id: u64,
    }

    impl ModelE {
        pub fn set_id(&mut self, id_in: u64) {
            self.id = id_in;
        }
        pub fn id_mut(&mut self) -> &mut u64 {
            return &mut self.id;
        }
        pub fn build() -> Self {
            return Self {
                attachment: Default::default(),
                id: 0,
            };
        }

        pub fn bind_task_queue(&mut self, tqin: &mut Datum<TaskQueue<ModelTask>>) {
            self.attachment.index_transform_task = tqin.alloc_data(
                TaskQueue::default(),
                Option::None,
            ).index();
        }

        pub fn update_global_transform_sync(
            datum_trans: &mut Datum<TransformD>,
            datum_model: &mut Datum<ModelD>,
        ) {
            let mut _node_index_array = Vec::<u64>::default();
            let mut _parent_count: usize = 0;
            // check if need change transform
            // if yes, then update global transform
            // if no, then do nothing
            for mi in datum_model.iter_mut() {
                if mi.as_mut().unwrap().attachment.switch_transform_update {
                    mi.as_mut().unwrap().attachment.switch_transform_update = false;
                    _node_index_array.push(mi.as_mut().unwrap().node_ref().index_self);

                    _parent_count = _parent_count + 1;
                }
            }
            if !_node_index_array.is_empty() {
                for i in 0.._parent_count {
                    _node_index_array
                        .extend(ModelD::find_all_subnode_index(datum_model, i as u64).unwrap())
                }
                for i in _parent_count.._node_index_array.len() {
                    TransformD::update_global(
                        datum_trans,
                        datum_model.vec_ref()[_node_index_array[i] as usize]
                            .as_ref()
                            .unwrap()
                            .node_ref()
                            .index_parent,
                        _node_index_array[i],
                    )
                }
            }
        }

        pub fn euler_rotate() {}

        pub fn quat_rotate() {}

        pub fn dispalce() {}

        pub fn scale() {}

        pub fn exe_transform() {}
    }

    #[repr(C, align(8))]
    #[derive(Debug)]
    pub struct ModelD {
        //node_option
        pub id: u64,
        attachment: ModelAttachment,
        node: NodeD,
    }

    impl ModelD {
        pub fn build() -> Self {
            Self {
                id: u64::MAX,
                node: Default::default(),
                attachment: Default::default(),
            }
        }
        pub fn build_buf_capacity(mut self, capacity_in: usize) -> Self {
            self.attachment.attechments_index_buffer = Vec::with_capacity(capacity_in);
            return self;
        }

        pub fn build_from_meta(mut self) -> Self {
            return self;
        }

        /// -  record attachment
        /// -  attechment_type: u64, index_in: usize
        /// -  绑定附件类型和索引
        /// -  会对类型进行重新排序
        /// -  该函数虽然可以但不建议用于绑定子模型
        pub fn push_attechment(&mut self, attechment_type: u64, index_in: usize) {
            if self.attachment.attechments_index_buffer.capacity() == 0 {
                self.attachment.attechments_index_buffer =
                    Vec::with_capacity(super::MODEL_MAX_ATTECHMENT_NUM);
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_WARN
                        | crate::log::code::CONDI_INIT_IMCOMPLETE
                        | crate::log::code::FILE_MODEL
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_DAT_ID)
                            .get_code()
                )
            }
            if self.attachment.attechments_index_buffer.len()
                < self.attachment.attechments_index_buffer.capacity()
            {
                self.attachment
                    .attechments_index_buffer
                    .push((attechment_type, index_in));
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_ERROR
                        | crate::log::code::CONDI_NUM_OVERFLOW
                        | crate::log::code::FILE_MODEL
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_DAT_ID)
                            .get_code()
                )
            }
        }

        /// - 啊 是的，你没有看错。
        /// -  这玩意可以录入数据指针
        /// -  当然，十分不建议两个模式混用
        /// -  一般来说，还是推荐使用索引模式
        pub fn push_pointer<T>(&mut self, attechment_type: u64, target_p: &T) {
            let _p = target_p as *const T as usize;
            self.push_attechment(attechment_type, _p);
        }

        pub fn contains_attechment(&self, attechment_type: u64, index_in: usize) -> bool {
            todo!();
        }

        /// 该函数用于绑定子模型
        pub fn bind_child(&mut self, index_in: usize) {
            // check if init
            if self.attachment.attechments_index_buffer.capacity() == 0 {
                self.attachment.attechments_index_buffer =
                    Vec::with_capacity(super::MODEL_MAX_ATTECHMENT_NUM);
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_WARN
                        | crate::log::code::CONDI_INIT_IMCOMPLETE
                        | crate::log::code::FILE_MODEL
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_DAT_ID)
                            .get_code()
                )
            }
            //
            let _len = self.attachment.attechments_index_buffer.len();
            if _len < self.attachment.attechments_index_buffer.capacity() {
                self.attachment
                    .attechments_index_buffer
                    .push((super::mtid::MTID_DAT_MODEL, index_in));
                self.attachment
                    .attechments_index_buffer
                    .swap(_len - 1, self.attachment.index_child_offset);
                self.attachment.index_child_offset = self.attachment.index_child_offset + 1;
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_DAT_ERROR
                        | crate::log::code::CONDI_NUM_OVERFLOW
                        | crate::log::code::FILE_MODEL
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_DAT_ID)
                            .get_code()
                )
            }
        }

        pub fn get_child_index_slice_ref(&self) -> &[(u64, usize)] {
            let _r =
                &self.attachment.attechments_index_buffer[0..self.attachment.index_child_offset];
            return _r;
        }

        /// 后进先出
        /// 寻找第一个符合条件的附件索引
        /// 如果需要返回指针，请取出usize之后出去使用“cast_ref!(target_type, attr)投影至对应类型”
        pub fn get_attechment_index(&self, attechment_type: u64) -> Result<usize, ()> {
            let _r = self
                .attachment
                .attechments_index_buffer
                .iter()
                .find(|&&ti| ti.0 == attechment_type)
                .unwrap()
                .1;
            return Ok(_r);
        }

        /// 寻找第所有符合条件的附件 并组装为一个Vec
        pub fn get_attechment_index_vec(&self, attechment_type: u64) -> Result<Vec<usize>, ()> {
            let _r: Vec<usize> = self
                .attachment
                .attechments_index_buffer
                .iter()
                .filter_map(|ain| {
                    if ain.0 == attechment_type {
                        Some(ain.1)
                    } else {
                        Option::None
                    }
                })
                .collect();

            return Ok(_r);
        }
    }

    impl NodeT for ModelD {
        fn node_ref(self: &Self) -> &NodeD {
            return &self.node;
        }

        fn node_mut(self: &mut Self) -> &mut NodeD {
            return &mut self.node;
        }
    }

    impl Default for ModelAttachment {
        fn default() -> Self {
            Self {
                attechments_index_buffer: Vec::with_capacity(MODEL_MAX_ATTECHMENT_NUM),
                index_child_offset: 0,
                index_offset: 0,
                is_active: true,
                switch_transform_update: false,
            }
        }
    }

    impl Default for ModelExeAttachment {
        fn default() -> Self {
            Self {
                index_transform_task: 0,
            }
        }
    }

    impl Default for ModelTask {
        fn default() -> Self {
            return Self::None;
        }
    }

    impl Default for ModelD {
        fn default() -> Self {
            todo!();
            Self {
                id: u64::MAX,
                attachment: Default::default(),
                node: Default::default(),
            }
        }
    }
}

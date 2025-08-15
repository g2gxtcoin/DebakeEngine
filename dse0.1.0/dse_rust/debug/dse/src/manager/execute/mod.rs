//#[cfg(feature="execute_use_subexe_true")]
pub mod sub;
pub mod template;

#[cfg(feature = "env_bit_64bit")]
pub mod env {
    #[cfg(feature = "config_TASK_DEFAULT_QUEUE_LEN_0")]
    pub const TASK_DEFAULT_QUEUE_LEN: usize = 0;
    #[cfg(feature = "config_TASK_DEFAULT_QUEUE_LEN_16")]
    pub const TASK_DEFAULT_QUEUE_LEN: usize = 16;
    #[cfg(feature = "config_TASK_DEFAULT_QUEUE_LEN_8")]
    pub const TASK_DEFAULT_QUEUE_LEN: usize = 8;
    #[cfg(feature = "config_TASK_DEFAULT_QUEUE_LEN_32")]
    pub const TASK_DEFAULT_QUEUE_LEN: usize = 32;
    #[cfg(feature = "config_TASK_DEFAULT_QUEUE_LEN_64")]
    pub const TASK_DEFAULT_QUEUE_LEN: usize = 64;

    #[cfg(feature = "std_use_time")]
    #[cfg(feature = "execute_use_subexe_true")]
    use std::time::{self, Duration, SystemTime};

    #[repr(C, align(4))]
    struct TaskQueueConfig {
        id: u32,
        queue_len: usize,
        #[cfg(feature = "execute_use_subexe_true")]
        offset: usize,
        is_subtime_sort: bool,
    }

    #[repr(C, align(4))]
    pub struct TaskQueue<Task>
    where
        Task: Default,
    {
        config: TaskQueueConfig,
        #[cfg(feature = "execute_use_subexe_false")]
        queue: Vec<Task>,
        #[cfg(feature = "execute_use_subexe_true")]
        queue: Vec<(Task, u128)>,
        // intervla:
    }

    // wqndyd sb rust
    // pub struct TaskQueueIter<'lt_queue,Task>
    // where Task:Default {
    //     count:usize,
    //     queue:&'lt_queue mut TaskQueue<Task>,
    // }
    //
    // impl<'lt_queue,Task> Iterator for TaskQueueIter<'lt_queue,Task>
    // where Task:Default {
    //     type Item = &'lt_queue mut Task;

    //     fn next(&mut self) ->Option<Self::Item> {
    //         if self.count<self.queue.queue.len(){
    //             self.count=self.count+1;
    //             return Option::Some(&mut self.queue.queue[self.count-1]);
    //         }else if self.count < (self.queue.queue.len()+self.queue.flex_queue.len()){
    //             self.count=self.count+1;
    //             return Option::Some(&mut self.queue.flex_queue[self.count-self.queue.queue.len()-1]);
    //         }
    //         return  Option::None;
    //     }
    // }

    impl<Task> TaskQueue<Task>
    where
        Task: Default,
    {
        pub fn build() -> Self {
            return Self {
                queue: Vec::with_capacity(TASK_DEFAULT_QUEUE_LEN),
                config: Default::default(),
            };
        }

        pub fn build_reset_queue_len(mut self, size: usize) -> Self {
            self.queue = Vec::with_capacity(size);
            self.config.queue_len = size;
            return self;
        }

        
        #[cfg(feature = "execute_use_subexe_false")]
        pub fn begin_execute(&mut self) {}

        // to set inhernit exe step len.
        // fench task from queue by CURRENT_EXE_TICK setting
        // if CURRENT_EXE_TICK seting s 1, it mean proccess it transinite but Inefficient
        // When you face some unexpected problem ,
        // recommand use short CURRENT_EXE_TICK in order strictly ordering env.
        #[cfg(feature = "execute_use_subexe_true")]
        pub fn begin_execute(&mut self) {
            let len = self.queue.len();
            let current = unsafe { crate::manager::execute::sub::CURRENT_EXE_TICK.clone() };
            self.queue[(len - self.config.offset)..].sort_unstable_by_key(|x| x.1);
            self.config.offset = self
                .queue
                .iter()
                .enumerate()
                .find(|x| x.1 .1 > current)
                .unwrap()
                .0;
            self.config.is_subtime_sort = true;
        }

        #[cfg(feature = "execute_use_subexe_false")]
        pub fn end_execute(&mut self) {
            self.clear_queue();
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn end_execute(&mut self) {
            let len = self.queue.len();
            for i in 0..self.config.offset {
                self.queue.remove(i);
            }
            self.config.is_subtime_sort = false;
        }

        // pub fn into_iter(&mut self)->TaskQueueIter<Task>{
        //     return TaskQueueIter{ count: 0, queue: self };
        // }

        #[cfg(feature = "execute_use_subexe_false")]
        pub fn push_task(&mut self, vin: Task) {
            if self.config.execute_lock == true {
                self.queue.push(vin);
            }
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn push_task(&mut self, vin: Task) {
            self.queue.push((
                vin,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_micros(),
            ));
            self.config.is_subtime_sort = false;
        }

        pub fn len(&self) -> usize {
            return self.queue.len();
        }

        pub fn clear(&mut self) {
            self.queue.clear();
        }

        pub fn fit_queue(&mut self) {
            self.queue.shrink_to_fit();
        }

        pub fn reset_queue_len(&mut self, size: usize) {
            self.queue = Vec::with_capacity(size);
            self.config.queue_len = size;
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn task_iter_mut(&mut self) -> Result<core::slice::IterMut<(Task, u128)>, String> {
            return Ok(self.queue.iter_mut());
        }

        #[cfg(feature = "execute_use_subexe_false")]
        pub fn task_iter_mut(&mut self) -> Result<core::slice::IterMut<Task>, String> {
            return Ok(self.queue.iter_mut());
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn task_iter_clone(&mut self) -> Result<core::slice::IterMut<(Task, u128)>, String> {
            return Ok(self.queue.iter_mut());
        }

        #[cfg(feature = "execute_use_subexe_false")]
        pub fn task_iter_clone(&mut self) -> Result<core::slice::IterMut<Task>, String> {
            return Ok(self.queue.iter_mut());
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn task_iter_ref(&self) -> Result<core::slice::Iter<(Task, u128)>, String> {
            return Ok(self.queue.iter());
        }

        #[cfg(feature = "execute_use_subexe_false")]
        pub fn task_iter_ref(&self) -> Result<core::slice::Iter<Task>, String> {
            return Ok(self.queue.iter());
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn current_sub_task_iter_mut(&mut self) -> Result<core::slice::Iter<(Task, u128)>, ()> {
            match self.config.is_subtime_sort {
                false => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_TASK_QUEUE_UNSORT
                            | crate::log::code::FILE_EXECUTE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.config.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ));
                }
                true => {
                    return Ok(self.queue[0..self.config.offset].iter());
                }
            }
        }

        #[cfg(feature = "execute_use_subexe_true")]
        pub fn current_sub_task_iter_ref(&self) -> Result<core::slice::Iter<(Task, u128)>, ()> {
            match self.config.is_subtime_sort {
                false => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_TASK_QUEUE_UNSORT
                            | crate::log::code::FILE_EXECUTE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.config.id as u128, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ));
                }
                true => {
                    return Ok(self.queue[0..self.config.offset].iter());
                }
            }
        }

        pub fn as_mut(&mut self) -> &mut Self {
            return self;
        }

        pub fn as_ref(&self) -> &Self {
            return self;
        }
    }

    impl<Task> Default for TaskQueue<Task>
    where
        Task: Default,
    {
        fn default() -> Self {
            return Self {
                queue: Vec::with_capacity(TASK_DEFAULT_QUEUE_LEN),
                config: Default::default(),
            };
        }
    }

    impl Default for TaskQueueConfig {
        fn default() -> Self {
            return Self {
                id: 0,
                queue_len: TASK_DEFAULT_QUEUE_LEN,
                #[cfg(feature = "execute_use_subexe_true")]
                offset: 0,
                is_subtime_sort: false,
            };
        }
    }
}

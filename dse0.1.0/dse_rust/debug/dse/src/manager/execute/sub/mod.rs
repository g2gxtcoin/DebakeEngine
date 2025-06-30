use std::time::{self, Duration, SystemTime};

#[cfg(feature = "config_EXECUTE_SUB_STEP_LEN_1")]
pub const EXECUTE_SUB_STEP_LEN: usize = 1;

#[cfg(feature = "config_EXECUTE_SUB_STEP_LEN_8")]
pub const EXECUTE_SUB_STEP_LEN: usize = 8; //120fps

#[cfg(feature = "config_EXECUTE_SUB_STEP_LEN_16")]
pub const EXECUTE_SUB_STEP_LEN: usize = 16; //60fps

#[cfg(feature = "config_EXECUTE_SUB_STEP_LEN_32")]
pub const EXECUTE_SUB_STEP_LEN: usize = 32; //30fps

pub static mut CURRENT_EXE_TICK: u128 = 0;

pub fn sub_exe_loop() -> bool {
    unsafe {
        CURRENT_EXE_TICK = CURRENT_EXE_TICK + u128::try_from(EXECUTE_SUB_STEP_LEN).unwrap();

        if CURRENT_EXE_TICK
            < SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros()
        {
            return true;
        }
    }
    return false;
}

pub mod task_interface {
    pub trait TaskTrait {
        type Task;
        fn task(self) -> Self::Task;
        fn task_ref(&self) ->  &Self::Task;
        fn task_mut(&mut self) ->&mut Self::Task;
    }

    #[cfg(feature = "execute_use_subexe_true")]
    impl<Task> TaskTrait for (Task, u128) {
        type Task = Task;
        fn task(self) -> Self::Task {
            return self.0;
        }

        fn task_ref(&self) -> &Self::Task {
            return &self.0;
        }

        fn task_mut(&mut self) -> &mut Self::Task {
            return &mut self.0;
        }
    }

    #[cfg(feature = "execute_use_subexe_false")]
    impl<Task> TaskTrait for Task {
        type Task = Task;
        fn task(self) -> Self::Task {
            return self;
        }

        fn task_ref(&self) -> &Self::Task {
            return &self;
        }

        fn task_mut(&mut self) -> &mut Self::Task {
            return &mut self;
        }
    }
    
}

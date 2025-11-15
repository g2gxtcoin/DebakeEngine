// pub const DEFAULT_SEMAPHORE_COUNT: usize = 3;
// pub const INDEX_INIT_IMG_STAGE_SEMAPHORE: usize = 0;
// pub const INDEX_GET_IMG_STAGE_SEMAPHORE: usize = 1;
// pub const INDEX_OUT_IMG_STAGE_SEMAPHORE: usize = 2;


pub mod env {}

//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "")]
pub mod env {

    #[derive(Default)]
    pub struct CmdSyncUsage(u64);

    #[allow(unused)]
    impl CmdSyncUsage {
        pub const DEFAULT: u64 = 0x0;
        pub const SwapColor: u64 = 0b0000_0001;
        pub const SwapDepth: u64 = 0b0000_0010;
    }

    pub struct CmdSyncAttachment {
        pub id_cmd: u64,
        pub id_renderer: u64,
        pub usage: CmdSyncUsage,

    }

    pub struct CmdSyncD {

    }

    impl CmdSyncD {
        pub fn build() -> Self {
            return Default::default();
        }
        pub fn build_push_semaphore(mut self, semaphore: ash::vk::Semaphore) -> Self {
            self.semaphore.push(semaphore);
            return self;
        }
        pub fn build_push_event(mut self, event: ash::vk::Event) -> Self {
            self.event.push(event);
            return self;
        }

        pub fn build_bind_renderer(mut self, ref_rin: &crate::renderer::env::RendererE) -> Self {
            self.attachment.id_renderer = ref_rin.id;
            return self;
        }

        pub fn id_mut(&mut self) -> &mut u64 {
            &mut self.id
        }
        pub fn id_ref(&self) -> &u64 {
            &self.id
        }

        pub fn semaphore_vec_mut(&mut self) -> &mut Vec<ash::vk::Semaphore> {
            &mut self.semaphore
        }
        pub fn event_vec_mut(&mut self) -> &mut Vec<ash::vk::Event> {
            &mut self.event
        }
        pub fn fences_mut(&mut self) -> &mut Vec<ash::vk::Fence> {
            &mut self.fence
        }

        pub fn semaphore_vec_ref(&self) -> &Vec<ash::vk::Semaphore> {
            &self.semaphore
        }
        pub fn event_vec_renf(&self) -> &Vec<ash::vk::Event> {
            &self.event
        }
        pub fn fences_ref(&self) -> &Vec<ash::vk::Fence> {
            &self.fence
        }

        pub fn destroy(self) {}
    }

    impl Default for CmdSyncAttachment {
        fn default() -> Self {
            Self {
                id_cmd: u64::MAX,
                id_renderer: u64::MAX,
                usage: Default::default(),
                count_active_swapimg: 0,
                index_current_swapimg: 0,
            }
        }
    }

    impl Default for CmdSyncD {
        fn default() -> Self {
            Self {
                id: 0,
                attachment: Default::default(),
                semaphore: Default::default(),
                fence: Default::default(),
                event: Default::default(),
            }
        }
    }
}

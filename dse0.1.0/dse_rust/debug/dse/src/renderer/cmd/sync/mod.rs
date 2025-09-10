//#[allow(unused,dead_code)]
#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_bit_64bit")]
pub mod env {

    pub struct CmdSyncAttachment {
        pub id_cmd: u64,
        pub id_renderer: u64,

        pub index_init_img_stage_semaphore: usize,
        pub index_get_img_stage_semaphore: usize,
        pub index_out_img_stage_semaphore: usize,
    }

    pub struct CmdSyncD {
        pub id: u64,
        pub attachment: CmdSyncAttachment,
        semaphore: Vec<ash::vk::Semaphore>,
        fence: Vec<ash::vk::Fence>,
        event: Vec<ash::vk::Event>,
    }

    impl CmdSyncD {
        pub fn build() -> Self {
            return Default::default();
        }
        pub fn build_push_semaphore(mut self, semaphore: ash::vk::Semaphore)  -> Self {
            self.semaphore.push(semaphore);
            return  self;
        }
        pub fn build_push_event(mut self, event: ash::vk::Event)  -> Self {
            self.event.push(event);
            return  self;
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

        pub fn semaphore_vec_renf(&self) -> &Vec<ash::vk::Semaphore> {
            &self.semaphore
        }
        pub fn event_vec_renf(&self) -> &Vec<ash::vk::Event> {
            &self.event
        }
        pub fn fences_ref(&self) -> &Vec<ash::vk::Fence> {
            &self.fence
        }

        pub fn destroy(self){

        }
    }

    impl Default for CmdSyncAttachment {
        fn default() -> Self {
            Self {
                id_cmd: u64::MAX,
                id_renderer: u64::MAX,
                
                index_get_img_stage_semaphore: usize::MAX,
                index_init_img_stage_semaphore: usize::MAX,
                index_out_img_stage_semaphore: usize::MAX,
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

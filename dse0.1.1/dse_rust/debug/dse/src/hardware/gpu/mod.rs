#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "graphic_api_vulkan_1_3")]
pub mod env {
    use std::{fmt::Debug, ptr::null};

    use ash::vk;

    use crate::________________dev_break________________;

    pub struct DseGPU {
        pub score: usize,
        pub priority: [f32; 16],

        pub physical_p: Option<vk::PhysicalDevice>,
        pub physical_info: Option<vk::PhysicalDeviceProperties>,
        pub physical_features: Option<vk::PhysicalDeviceFeatures>,
        pub mem_info: Option<vk::PhysicalDeviceMemoryProperties>,

        pub logical_p: Option<ash::Device>,
        pub logical_info: Option<vk::DeviceCreateInfo>,

        pub queue_families: Option<Vec<vk::QueueFamilyProperties>>,
        pub queue_create_info_vec: Option<Vec<vk::DeviceQueueCreateInfo>>,
        pub queue_vec: Option<Vec<vk::Queue>>, // todo()!
    }

    impl DseGPU {
        pub fn mem_info_ref(&self) -> Result<&vk::PhysicalDeviceMemoryProperties, ()> {
            match self.mem_info.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn physical_device_ref(&self) -> Result<&vk::PhysicalDevice, ()> {
            match self.physical_p.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn physical_info_ref(&self) -> Result<&vk::PhysicalDeviceProperties, ()> {
            match self.physical_info.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ));
                }
            }
        }
        pub fn physical_feature_ref(&self) -> Result<&vk::PhysicalDeviceFeatures, ()> {
            match self.physical_features.as_ref() {
                Some(s) => return std::result::Result::Ok(s),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn queue_families_info_ref(&self) -> Result<&Vec<vk::QueueFamilyProperties>, ()> {
            match self.queue_families {
                Some(ref i) => return std::result::Result::Ok(i),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ));
                }
            }
        }

        pub fn physical_p_ref(&self) -> Result<&vk::PhysicalDevice, ()> {
            match self.physical_p {
                Some(ref i) => return std::result::Result::Ok(i),
                None => {
                    return Err(crate::log::sorry(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_INSTANCE_NOT_FOUND
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code(),
                    ));
                }
            }
        }
        pub fn physical_info_clone(&self) -> Result<vk::PhysicalDeviceProperties, ()> {
            match self.physical_info {
                Some(val) => Ok(val.clone()),
                None => {
                    return Err(crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_OPTION_NONE
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    ));
                }
            }
        }

        pub fn create_logical_info(&mut self, feature_vec_ref: &Vec<*const i8>) {
            self.create_device_queue_create_info();
            let _info = vk::DeviceCreateInfo {
                s_type: vk::StructureType::DEVICE_CREATE_INFO,
                p_next: null(),
                flags: vk::DeviceCreateFlags::default(),
                queue_create_info_count: self.queue_create_info_vec.as_ref().unwrap().len() as u32,
                p_queue_create_infos: self.queue_create_info_vec.as_ref().unwrap().as_ptr(),
                enabled_layer_count: 0,
                pp_enabled_layer_names: null(),
                enabled_extension_count: feature_vec_ref.len() as u32,
                pp_enabled_extension_names: feature_vec_ref.as_ptr(),
                p_enabled_features: self.physical_features.as_ref().unwrap(),
            };
            self.logical_info = Option::Some(_info);
        }

        pub fn create_device_queue_create_info(&mut self) {
            if !self.queue_create_info_vec.is_some() {
                if !self.queue_families.is_some() {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_EXT_ERROR
                            | crate::log::code::CONDI_VK_BUILDER_PREBUILD_NOT_BUILD
                            | crate::log::code::FILE_EXTAPI_GRAPHIC_VK
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(0, crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                                .get_code()
                    )
                } else {
                    self.queue_create_info_vec = Option::Some(Default::default());
                    for eqi in self.queue_families.as_ref().unwrap().iter().enumerate() {
                        let _info = vk::DeviceQueueCreateInfo {
                            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                            p_next: null(),
                            flags: vk::DeviceQueueCreateFlags::default(),
                            queue_family_index: eqi.0 as u32,
                            queue_count: eqi.1.queue_count,
                            p_queue_priorities: self.priority.as_ptr(),
                        };
                        // ensure this pcie is pipe toward gpu
                        self.queue_create_info_vec.as_mut().unwrap().push(_info);
                    }
                }
            } else {
                for eqi in self.queue_families.as_ref().unwrap().iter().enumerate() {
                    let _info = vk::DeviceQueueCreateInfo {
                        s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                        p_next: null(),
                        flags: vk::DeviceQueueCreateFlags::default(),
                        queue_family_index: eqi.0 as u32,
                        queue_count: eqi.1.queue_count,
                        p_queue_priorities: self.priority.as_ptr(),
                    };
                    // ensure this pcie is pipe toward gpu
                    self.queue_create_info_vec.as_mut().unwrap().push(_info);
                }
            }
        }
    }

    impl Default for DseGPU {
        fn default() -> Self {
            Self {
                score: 0,
                priority: [0.0f32; 16],
                logical_p: Option::None,
                logical_info: Option::None,
                physical_p: Option::None,
                physical_info: Option::None,
                physical_features: Option::None,
                mem_info: Option::None,
                queue_families: Option::None,
                queue_create_info_vec: Option::None,
                queue_vec: Option::None,
            }
        }
    }

    impl Debug for DseGPU {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let _r = self.logical_p.as_ref().unwrap() as *const ash::Device as usize;
            f.debug_struct("HardwareInfo")
                .field("score", &self.score)
                .field("priority", &self.priority)
                .field("logical_device", &_r)
                .field("logical_info", &self.logical_info)
                .field("hardware_device_p", &self.physical_p)
                .field("property", &self.physical_info)
                .field("features", &self.physical_features)
                .field("mem_property", &self.mem_info)
                .field("queue_families", &self.queue_families)
                .field("queue_create_info_vec", &self.queue_create_info_vec)
                .field("queue_vec", &self.queue_vec)
                .finish()
        }
    }
}

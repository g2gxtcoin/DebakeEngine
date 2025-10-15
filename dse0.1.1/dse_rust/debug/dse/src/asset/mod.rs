

#[cfg(feature = "env_bit_64bit")]
pub mod env {
    use std::{path::PathBuf, str::FromStr};

    use crate::{application::env::ApplicationD, resource::env::ResourceE};

    /// 注册表根节点
    /// 遍历文件以此开始
    /// counter : 引用计数器
    #[derive(Default, serde::Deserialize)]
    pub struct RegRoot {
        path: String,
        name: String,
    }

    impl RegRoot {
        pub fn build() -> Self {
            return Default::default();
        }

        pub fn build_name(mut self, sin: String) -> Self {
            self.name = sin;
            return self;
        }

        pub fn build_path(mut self, sin: String) -> Self {
            self.path = sin;
            return self;
        }

        pub fn from_path(pin: PathBuf) -> Self {
            return Self {
                path: pin.parent().unwrap().to_str().unwrap().to_string(),
                name: pin.file_name().unwrap().to_str().unwrap().to_string(),
            };
        }
    }

    /// 资产注册表执行
    ///
    pub struct RegisterE {
        id: u64,
    }

    /// 资产注册表
    ///
    #[derive(Default)]
    pub struct RegistyD {
        root: Vec<RegRoot>,
    }

    impl RegistyD {
        pub fn build() -> Self {
            return Self {
                root: Vec::with_capacity(1),
            };
        }

        pub fn build_set_root(mut self, path_in: PathBuf) -> Self {
            self.root.push(
                RegRoot::build()
                    .build_name("root".to_string())
                    .build_path(path_in.to_str().unwrap().to_string()),
            );

            return self;
        }

        pub fn link_app(&mut self, app_in: &ApplicationD) {
            self.root.push(
                RegRoot::build()
                    .build_name("root".to_string())
                    .build_path(app_in.get_asset_path().to_str().unwrap().to_string()),
            );
        }
    }

    impl RegisterE {}
}

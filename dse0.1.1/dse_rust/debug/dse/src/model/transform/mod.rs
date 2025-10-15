use glam::DVec4;

use crate::test;

#[cfg(feature = "graphic_api_vulkan_1_3")]
#[cfg(feature = "env_os_win10")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
pub mod env {
    use std::f64::consts::PI;

    use glam::{DMat4, DQuat, DVec4};

    use crate::{get, get_mut, manager::datum::env::Datum};

    #[derive(Clone, Copy,Debug)]
    #[repr(C, align(8))]
    pub struct Transform {
        position: DVec4,   // w : coord sys id
        quaternion: DVec4, // x axis:
        scale: DVec4,
    }
    impl Default for Transform {
        fn default() -> Self {
            Self {
                position: DVec4::new(0.0, 0.0, 0.0, 0.0),
                quaternion: DVec4::new(0.0, 0.0, 0.0, 1.0),
                scale: DVec4::ONE,
            }
        }
    }

    #[derive(Default,Debug)]
    #[repr(C, align(8))]
    pub struct TransformD {
        local: Transform,
        global: Transform,
    }

    impl TransformD {
        pub fn update_global(
            datum_in: &mut Datum<TransformD>,
            parent_index_in: u64,
            self_index_in: u64,
        ) {
            let mut _parent= get!(datum_in.vec_ref(), parent_index_in as  usize).global_clone();
            let mut _self= get_mut!(datum_in.vec_mut(), parent_index_in as  usize);

            _self.global.position = Self::dispalce_from2(_parent.position, _self.local_clone().position);
            _self.global.quaternion =
                Self::rotate_quat_from2(_parent.quaternion, _self.local_clone().quaternion);
            _self.local.scale = Self::scaled_form2(_parent.scale, _self.local_clone().scale);
        }

        // XYZ order global euler rotate matrix
        pub fn rotate_euler_radian_sync(&mut self, ein: DVec4) {
            let _rhs = DQuat::from_euler(glam::EulerRot::XYZ, ein.x, ein.y, ein.z);
            let _self = DQuat::from_vec4(self.local.quaternion);
            let _global = DQuat::from_vec4(self.global.quaternion);
            self.local.quaternion = DVec4::from(DQuat::mul_quat(_self, _rhs));

            self.global.quaternion = DVec4::from(DQuat::mul_quat(_global, _rhs));
        }

        pub fn rotate_euler_degree_sync(&mut self, ein: DVec4) {
            let ein = ein * (PI / 180.0);
            let _rhs = DQuat::from_euler(glam::EulerRot::XYZ, ein.x, ein.y, ein.z);
            let _self = DQuat::from_vec4(self.local.quaternion);
            let _global = DQuat::from_vec4(self.global.quaternion);
            self.local.quaternion = DVec4::from(DQuat::mul_quat(_self, _rhs));

            self.global.quaternion = DVec4::from(DQuat::mul_quat(_global, _rhs));
        }

        pub fn rotate_quat_sync(&mut self, quat_in: DVec4) {
            self.local.quaternion = DVec4::from(DQuat::mul_quat(
                DQuat::from_vec4(self.local.quaternion),
                DQuat::from_vec4(quat_in),
            ));

            self.global.quaternion = DVec4::from(DQuat::mul_quat(
                DQuat::from_vec4(self.global.quaternion),
                DQuat::from_vec4(self.local.quaternion),
            ));
        }

        pub fn rotate_quat_from2(quat_in: DVec4, quat_rhs_in: DVec4) -> DVec4 {
            return DVec4::from(DQuat::mul_quat(
                DQuat::from_vec4(quat_in),
                DQuat::from_vec4(quat_rhs_in),
            ));
        }

        pub fn dispalce_sync(&mut self, vin: DVec4) {
            self.local.position = self.local.position + vin;
        }

        pub fn dispalce_from2(from_in: DVec4, to_in: DVec4) -> DVec4 {
            return from_in + to_in;
        }

        pub fn scaled_sync(&mut self, vin: DVec4) {
            self.local.scale = self.local.scale + vin;
        }

        pub fn scaled_form2(from_in: DVec4, to_in: DVec4) -> DVec4 {
            return from_in + to_in;
        }

        pub fn position(&self) -> DVec4 {
            return self.local.position.clone();
        }

        pub fn quaternion(&self) -> DVec4 {
            return self.local.quaternion.clone();
        }

        pub fn euler_degree(&self) -> DVec4 {
            let _a =180.0/PI;
            let _r: (f64, f64, f64) = DQuat::from_vec4(self.local.quaternion).to_euler(glam::EulerRot::XYZ);
            return DVec4::new(_r.0*_a, _r.1*_a, _r.2*_a, 0.0);
        }

        pub fn local_ref(&self)->&Transform{
            return &self.local;
        }

        pub fn local_mut(&mut self)->&mut Transform{
            return &mut self.local;
        }

        pub fn local_clone(&self)->Transform{
            return self.local.clone();
        }

        pub fn global_ref(&self)->&Transform{
            return &self.global;
        }

        pub fn global_mut(&mut self)->&mut Transform{
            return &mut self.global;
        }

        pub fn global_clone(&self)->Transform{
            return self.global.clone();
        }

        pub fn scale(&self) -> DVec4 {
            return self.local.scale.clone();
        }

        pub fn s_mat(&self) -> DMat4 {
            return DMat4 {
                x_axis: DVec4::new(self.global.scale.x, 0.0, 0.0, 0.0),
                y_axis: DVec4::new(0.0, self.global.scale.y, 0.0, 0.0),
                z_axis: DVec4::new(0.0, 0.0, self.global.scale.z, 0.0),
                w_axis: DVec4::new(0.0, 0.0, 0.0, self.global.scale.w),
            };
        }

        pub fn r_mat(&self) -> DMat4 {
            return DMat4::from_quat(DQuat::from_vec4(self.global.quaternion).normalize());
        }

        pub fn d_mat(&self) -> DMat4 {
            return DMat4 {
                x_axis: DVec4::new(1.0, 0.0, 0.0, self.global.position.x),
                y_axis: DVec4::new(0.0, 1.0, 0.0, self.global.position.y),
                z_axis: DVec4::new(0.0, 0.0, 1.0, self.global.position.z),
                w_axis: DVec4::new(0.0, 0.0, 0.0, 1.0),
            };
        }

        pub fn model_mat(&self) -> DMat4 {
            return self.d_mat() * self.r_mat() * self.s_mat();
        }
    }

}

 

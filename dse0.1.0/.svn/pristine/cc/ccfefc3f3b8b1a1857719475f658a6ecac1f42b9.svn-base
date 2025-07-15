#[cfg(feature = "env_os_win10")]
#[cfg(feature = "env_bit_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_STEP_64bit")]
#[cfg(feature = "config_ENGINE_VERTEX_BUFFER_FLOAT_true")]
pub mod env{
    use crate::manager::datum::env::Datum;

    
    pub struct NodeD {
        pub index_self: u64,
        pub index_parent: u64,
    }

    impl Default for NodeD {
        fn default() -> Self {
            Self {
                index_self: u64::MAX,
                index_parent: u64::MAX,
            }
        }
    }

    pub trait NodeT
    where
        Self: Sized,
    {
        fn node_ref(self: &Self) -> &NodeD;

        fn node_mut(self: &mut Self) -> &mut NodeD;

        fn build_self_index(mut sin: Self, index_in: u64) -> Self {
            sin.node_mut().index_self = index_in;
            return sin;
        }

        
        fn build_parent(mut sin: Self, node_in: &NodeD) -> Self {
            sin.node_mut().index_parent = node_in.index_self;
            return sin;
        }

        fn set_parent(sin:&mut Self, node_in: &NodeD){
            sin.node_mut().index_parent = node_in.index_self;
        }


        // O(n * m * m) case
        // will not alloc assist mem
        // cache frendly
        fn find_all_subnode_index(mdin: &Datum<Self>, index_in: u64) -> Result<Vec<u64>, ()> {
            let mut _r = Vec::<u64>::default();
            let mut offset:usize = 0;
            for si in mdin.iter() {
                if si.node_ref().index_parent == index_in {
                    _r.push(si.node_ref().index_self);
                }
            }
            while offset <= _r.len() {
                for si in mdin.iter() {
                    if !_r.contains(&si.node_ref().index_self) && si.node_ref().index_parent == _r[offset] {
                        _r.push(si.node_ref().index_self);
                    }
                }
                offset=offset+1;
            }

            return Ok(_r);
        }

        // O(n * m) case
        // WARNING: unexpected node parent relyon loop will pull out a error log
        // such as A->B B->A
        // will not alloc assist mem
        // cache frendly
        fn find_all_subnode_index_uncheck(mdin: &Datum<Self>, parent_index_in: u64) -> Result<Vec<u64>, ()> {
            let mut _r = Vec::<u64>::default();
            let mut offset:usize = 0;
            for si in mdin.iter() {
                if si.node_ref().index_parent == parent_index_in {
                    _r.push(si.node_ref().index_self);
                }
            }
            while offset <= _r.len() {
                for si in mdin.iter() {
                        _r.push(si.node_ref().index_self);
                }
                offset=offset+1;
                if offset > mdin.vec_ref().len(){
                    return Err(crate::log::send2logger(
                        crate::log::code::TYPE_TRAIT_ERROR
                        | crate::log::code::CONDI_UNEXPECTED_RESULT
                        | crate::log::code::FILE_NODE
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(0, crate::log::LogCodePart::Id)
                            .get_code()
                    )
                    );
                }
            }

            return Ok(_r);
        }


        fn find_near_subnode_index(mdin: &mut Datum<Self>, parent_index_in: u64) -> Result<Vec<u64>, ()> {
            let mut _r = Vec::<u64>::default();
            for si in mdin.iter() {
                if si.node_ref().index_parent == parent_index_in {
                    _r.push(si.node_ref().index_self);
                }
            }
            return Ok(_r);
        }
    }
}
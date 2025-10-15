pub mod example {

    use std::fmt::Debug;

    use crate::meta_serialize::{MetaBase, MetaD, Serialize, META_ROOT_HASH,DeSerialize};

    #[repr(C)]
    #[derive(
        Default,
        Debug,
        //crate::dse_macros::MetaInnerIter,
        crate::dse_macros::BuildCustomStructMeta,
        crate::dse_macros::MetaBaseCustomStruct,
    )]
    //#[derive(Default, Debug)]
    struct Struct1 {    
        id: i32,
        name: String,
    }

    impl Debug for Struct1_innertablemeta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct1_innertablemeta")
                .field("id", &self.id)
                .field("name", &self.name)
                .finish()
        }
    }

    

    // #[derive(Default, Debug)]
    // struct struct1_innertablemeta {
    //     id: MetaD<i32>,
    //     name: MetaD<String>,
    // }

    // // MetaBaseInner
    // impl MetaBase for struct1_innertablemeta {
    //     type ValueType = Option<bool>;
    //     fn is_custom_struct() -> bool {
    //         todo!()
    //     }
    // }

    // // MetaBaseCustomStruct
    // impl MetaBase for struct1 {
    //     type ValueType = struct1_innertablemeta;

    //     fn is_custom_struct() -> bool {
    //         return true;
    //     }
    //     fn serialize_table(mut self) -> Result<struct1_innertablemeta, String> {
    //         return Ok(struct1_innertablemeta {
    //             id: <i32>::serialize(self.id, stringify!(id).to_string(), 0)
    //                 .expect_pre_implement()
    //                 .unwrap(),
    //             name: <String>::serialize(self.name, stringify!(id).to_string(), 0)
    //             .expect_pre_implement()
    //             .unwrap(),
    //         });
    //     }
    // }

    #[repr(C)]
    #[derive(
        Default,
        Debug,
        crate::dse_macros::MetaInnerIter,
        crate::dse_macros::BuildCustomStructMeta,
        crate::dse_macros::MetaBaseCustomStruct,
    )]
    struct Struct2 {
        id: i32,
        s1: Struct1,
        v1: Vec<Struct1>,
    }

    impl Debug for Struct2_innertablemeta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct2_innertablemeta")
                .field("id", &self.id)
                .field("s1", &self.s1)
                .field("v1", &self.v1)
                .finish()
        }
    }

    #[repr(C)]
    #[derive(
        Default,
        Debug,
        crate::dse_macros::MetaInnerIter,
        crate::dse_macros::BuildCustomStructMeta,
        crate::dse_macros::MetaBaseCustomStruct,
    )]
    struct Struct3 {
        id: i32,
        s1: Struct2,
        a1:[Struct1;3],
    }

    impl Debug for Struct3_innertablemeta {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct2_innertablemeta")
                .field("id", &self.id)
                .field("s1", &self.s1)
                .field("a1", &self.a1)
                .finish()
        }
    }

    //
    //
    //
    //

    #[test]
    pub fn example_init() {
        let mut my_struct = Struct1::default();
        my_struct.name = "hellow dse meta".to_string();
        dbg!(&my_struct);

        let meta_data = <Struct1>::serialize(my_struct, stringify!(v1).to_string(), META_ROOT_HASH)
            .expect_custom_struct()
            .unwrap();
        println!("{:?}", &meta_data);

        let mut my_struct = Struct2::default();
        dbg!(&my_struct);

        let meta_data = <Struct2>::serialize(my_struct, stringify!(v1).to_string(), META_ROOT_HASH)
            .expect_custom_struct()
            .unwrap();
        println!("{:?}", &meta_data);

        let mut my_struct = Struct3::default();
        dbg!(&my_struct);

        let meta_data = <Struct3>::serialize(my_struct, stringify!(v1).to_string(), META_ROOT_HASH)
            .expect_custom_struct()
            .unwrap();
        println!("{:?}", &meta_data);

        std::thread::sleep(std::time::Duration::new(10, 0));
    }


    #[test]
    #[cfg(feature = "example_use_serialize_example")]
    pub fn example_iter(){
        let mut my_struct = Struct3::default();
        my_struct.s1.v1.push(Struct1 { id: 5, name: "hellow meta".to_string()});
        my_struct.s1.v1.push(Struct1 { id: 55, name: "enjoy meta".to_string()});
        my_struct.s1.v1.push(Struct1 { id: 5, name: "fk meta".to_string()});
        my_struct.a1=[Struct1{ id: 2, name: "todo!()".to_string() },Struct1{ id: 22, name: "todo!()".to_string() },Struct1{ id: 222, name: "todo!()".to_string() }];
        dbg!(&my_struct);

        let meta_data_D =Box::new(<Struct3>::serialize(my_struct, stringify!(v1).to_string(), META_ROOT_HASH)
            .expect_custom_struct()
            .unwrap()) ;
        println!("{:?}", &meta_data_D);

        let de_struct=meta_data_D.value.a1.deserialize();
        println!("{:?}", &de_struct);
        
        //for i in Struct3
    }
}

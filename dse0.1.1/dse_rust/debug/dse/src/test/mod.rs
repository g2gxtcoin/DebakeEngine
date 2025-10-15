/* extern crate dse_macros;
use crate::meta_serialize::MetaInnerIter;

#[derive(Default, Debug, dse_macros::MetaInnerIter)]
#[repr(C)]
pub struct TestStructMeta {
    id: i32,
    name: String,
    value: f32,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct TestStructMetaMeta {
    id: crate::meta_serialize::MetaD<i32>,
    name: crate::meta_serialize::MetaD<String>,
    value: crate::meta_serialize::MetaD<f32>,
}

impl crate::meta_serialize::MetaBase for TestStructMetaMeta {
    type ValueType = Option<bool>;

    fn is_custom_struct() -> bool {
        todo!()
    }
}

impl crate::meta_serialize::MetaBase for TestStructMeta {
    type ValueType = TestStructMetaMeta;

    fn is_custom_struct() -> bool {
        true
    }
}

#[cfg(feature = "test")]
#[test]
fn macros_test() {
    // dse_macros::dse_proc_macro_example!(test_struct_meta::default());
    // dse_macros::dse_proc_macro_ext_test!(test_struct_meta::default());
    let a = TestStructMeta::default();
    dbg!(a);
}

*/

type TestCallback<DT, PT> = fn(self_: &mut DT, parameter: &PT);
pub const QueueLen: usize = 16;

struct TestTask {
    add: TestCallback<f32, f32>,
    data_id: u32,
    paramreter: f32,
}

struct TestExecute {
    id: u32,
    queue: [Option<TestTask>; QueueLen],
    task_offset: usize,
}
struct TestDatum<DT> {
    id: u32,
    data: Vec<DT>,
}
struct TestRelation {
    id: u32,
}

impl<DT> TestDatum<DT> {
    pub fn new() -> Self {
        return Self {
            id: 0,
            data: Vec::<DT>::new(),
        };
    }
    pub fn push_data(mut self, din: DT) -> Self {
        self.data.push(din);
        return self;
    }
}

impl TestExecute {
    pub fn new() -> Self {
        return Self {
            id: 0,
            task_offset: 0,
            queue: Default::default(),
        };
    }
    pub fn run(&mut self, datum: &mut TestDatum<f32>) {
        /*
        judge code ensure data exsist
        */
        for ti in 0..self.task_offset {
            let t = self.queue[ti].as_ref().unwrap();
            Self::call(t.add, &mut datum.data[t.data_id as usize], &t.paramreter);
        }
        self.task_offset = 0;
    }

    pub fn execute(&mut self, data_id: u32, paramreter: f32) {
        /*
        judge queue offset if illegle
        */
        self.queue[self.task_offset] = Option::Some(TestTask {
            add: Self::add,
            data_id: data_id,
            paramreter: paramreter,
        });
        if self.task_offset < QueueLen - 1 {
            self.task_offset = self.task_offset + 1;
        }
    }

    pub fn add(data: &mut f32, parameter: &f32) {
        *data = *data + parameter;
    }

    fn call(f: TestCallback<f32, f32>, d: &mut f32, p: &f32) {
        f(d, p)
    }
}

impl<DT> TestDatum<DT> {}

#[cfg(feature = "test")]
#[test]
fn DOP_press_test() {
    let mut count = 0u32;
    // you can use this method to express strong relation ship between datum
    let mut data1: TestDatum<TestDatum<f32>> = TestDatum::<TestDatum<f32>>::new()
        .push_data(TestDatum::<f32>::new().push_data(0f32))
        .push_data(TestDatum::<f32>::new().push_data(0f32))
        .push_data(TestDatum::<f32>::new().push_data(0f32));

    //excute define to decide thread behavior

    let main_exe1 = std::sync::Arc::new(std::sync::Mutex::new(TestExecute::new()));
    let th_exe1 = std::sync::Arc::clone(&main_exe1);

    let main_exe2 = std::sync::Arc::new(std::sync::Mutex::new(TestExecute::new()));
    let th_exe2 = std::sync::Arc::clone(&main_exe2);

    let main_exe3 = std::sync::Arc::new(std::sync::Mutex::new(TestExecute::new()));
    let th_exe3 = std::sync::Arc::clone(&main_exe3);

    //thread1
    std::thread::spawn(move || {
        let mut thread1_count = 0u32;
        loop {
            thread1_count = thread1_count + 1;
            (*th_exe1).lock().unwrap().execute(0, 2f32);
            std::thread::sleep(std::time::Duration::new(0, 0));
            // dbg!(thread1_count);
        }
    });
    //thread2
    std::thread::spawn(move || {
        let mut thread2_count = 0u32;
        loop {
            thread2_count = thread2_count + 1;
            (*th_exe2).lock().unwrap().execute(0, 2f32);
            std::thread::sleep(std::time::Duration::new(0, 0));
            // dbg!(thread2_count);
        }
    });

    std::thread::spawn(move || {
        let mut thread3_count = 0u32;
        loop {
            thread3_count = thread3_count + 1;
            (*th_exe3).lock().unwrap().execute(0, 2f32);
            std::thread::sleep(std::time::Duration::new(0, 0));
            // dbg!(thread3_count);
        }
    });
    // main thread
    loop {
        count = count + 1;
        (*main_exe1).lock().unwrap().run(&mut data1.data[0]);
        (*main_exe2).lock().unwrap().run(&mut data1.data[1]);
        (*main_exe3).lock().unwrap().run(&mut data1.data[2]);
        // dbg!(count);
        // dbg!(data1.data[0].data[0]);
        // dbg!(data1.data[1].data[0]);
        // std::thread::sleep(std::time::Duration::new(0, 00_0333_3333));
    }
}

#[cfg(feature = "test")]
#[test]
pub fn create_exe_call_back_template_test() {
    use std::usize;

    let mut result = String::new();
    let MRcount: usize = 32 + 1;
    let Rcount: usize = 32 + 1;
    let mut absolote_path = std::env::current_dir().unwrap().join("test.txt");
    let mut log_stream = match std::fs::File::create(absolote_path) {
        Err(why) => panic!("couldn't create log file: {}", why),
        Ok(file) => file,
    };
    for mr in 0..MRcount {
        for r in 0..Rcount {
            result = result + "pub type Callback" + &mr.to_string() + "MR" + &r.to_string() + "R<";
            for i in 0..mr {
                result = result + "MRT" + &i.to_string() + ","
            }
            for i in 0..r {
                if i == r {
                    result = result + "RT" + &i.to_string()
                } else {
                    result = result + "RT" + &i.to_string() + ","
                }
            }
            result = result + "> = fn(\n";
            for i in 0..mr {
                result = result + "dmut" + &i.to_string() + ": &mut MRT" + &i.to_string() + ",\n"
            }
            for i in 0..r {
                result = result + "d_ref" + &i.to_string() + ": &RT" + &i.to_string() + ",\n"
            }
            result = result + ");\n";
        }
    }
    match std::io::Write::write(&mut log_stream, result.as_bytes()) {
        Ok(_) => {}
        Err(_err) => println!("file save fail"),
    }
}

#[cfg(feature = "test")]
#[test]
fn vec_slice() {
    let mut a = vec![1, 2, 3];
    dbg!(&a[1..]);
    a.clear();
    dbg!(a.is_empty());
    dbg!(a.capacity());
}

#[cfg(feature = "test")]
#[test]
fn vec_align_to() {
    let mut a = vec![1, 2, 3, 4, 5, 6];
    dbg!(&a[0..]);

    let r: Vec<u128> = TryFrom::try_from(a).unwrap();
    dbg!(r);
}

#[cfg(feature = "test")]
#[test]
fn test_glam() {
    use std::f64::consts::PI;

    use glam::{DMat4, DVec4, EulerRot};

    let m = DMat4 {
        x_axis: DVec4::new(1.0, 0.0, -17.0, 11.0),
        y_axis: DVec4::new(0.0, 2.0, 0.0, 0.0),
        z_axis: DVec4::new(0.0, 0.0, 3.0, 0.0),
        w_axis: DVec4::new(-7.0, 13.0, 0.0, 5.0),
    };
    let vi = DVec4::ONE;
    let mi = DMat4::default();

    // dbg!(vi);
    // dbg!(m*vi);
    // dbg!(m.transpose()*vi);
    // dbg!(m.inverse()*vi);
    // dbg!(mi);
    dbg!(DMat4::from_euler(EulerRot::XYZ, 0.0, 0.0, PI));
    dbg!(
        DMat4::from_euler(EulerRot::XYZ, 0.0, 0.0, PI / 2.0).inverse()
            * DVec4::new(1.0, 1.0, 0.0, 0.0)
    );
}

#[cfg(feature = "test")]
#[test]
fn test_model_matrix() {
    use glam::{DVec3, DVec4, Vec4Swizzles};

    use crate::model::transform::env::TransformD;

    let mut t = TransformD::default();
    t.rotate_euler_degree_sync(glam::DVec4 {
        x: 60.0,
        y: 30.0,
        z: -45.0,
        w: 0.0,
    });
    dbg!(t.model_mat());
    let mut i = DVec4::ONE;
    let s = &mut i.xyz();
    dbg!(s);
}

#[cfg(feature = "test")]
#[test]
fn test_assert() {
    let mut a: usize = 6;
    a = a + unsafe { libc::rand() as usize };
    let v = [1, 2, 3];

    unsafe {
        dbg!(&v);
        debug_assert!(a < v.len());
        dbg!(&v[a]);
    }
}

#[cfg(feature = "test")]
#[allow(unused)]
mod test_toml_pack {
    use toml;

    #[derive(serde::Deserialize, Debug)]
    struct Config {
        ip: String,
        port: Option<u16>,
        vector: Option<Vec<u32>>,
        array: [i32; 4],
        keys: Keys,
    }

    #[derive(serde::Deserialize, Debug)]
    struct Keys {
        github: String,
        travis: Option<String>,
        val: Value,
    }

    #[derive(serde::Deserialize, Debug)]
    struct Value {
        val0: u32,
        val2: u8,
    }

    #[test]
    fn test() {
        let config: Vec<Config> = toml::from_str(
            r#"
            ip = '127.0.0.1'
            port = 12
            vector = [1,2,3]
            array = [1,2,3,4]
            [config.keys]
            github = 'xxxxxxxxxxxxxxxxx'
            travis = 'yyyyyyyyyyyyyyyyy'
            [config.keys.val]
            val0= 32
            val2= 8
            "#,
        )
        .unwrap();

        dbg!(config);
    }
}

#[cfg(feature = "test")]
#[test]
#[allow(arithmetic_overflow)]
fn test_overflow_u64_substance() {
    let a: glam::U64Vec4 = glam::U64Vec4 {
        x: 1,
        y: 2,
        z: 3,
        w: 4,
    };
    let b = a.x - a.y;
    match b {
        0 => {
            dbg!(&b);
        }
        _ => {}
    }
}

#[test]
pub fn test_trans() {
    let mut a = crate::model::transform::env::TransformD::default();
    let mut b = crate::model::transform::env::TransformD::default();
    dbg!(&a.model_mat());
    dbg!(&a.euler_degree());
    a.rotate_euler_degree_sync(glam::DVec4::new(0.0, 45.0, 0.0, 0.0));
    dbg!(&a.euler_degree());
    a.rotate_euler_degree_sync(glam::DVec4::new(0.0, 45.0, 20.0, 0.0));
    dbg!(&a.euler_degree());
    b.rotate_quat_sync(a.quaternion());
    dbg!(&b.euler_degree());
    dbg!(&b.quaternion());
    b.rotate_quat_sync(a.quaternion());
    dbg!(&b.euler_degree());
    dbg!(&b.quaternion());
}

#[cfg(feature = "test")]
#[test]
fn test_bit_proccesss() {
    let a = 0b1111111;
    let b = 0b10;
    let c = 1 << b;
    let d = a >> c; // 右移
    let e = 1 << d; // 左移
    let f = e & a; // 或
    let g = e | b; // 并
    let h = g ^ a; // 异或
    let i = !h; // 取反
    println!("a:{:b}, b:{:b}, c:{:b}, d:{:b}, e:{:b}", a, b, c, d, e);
    println!("f:{:b}, g:{:b}, h:{:b}, i:{:b}", f, g, h, i);
}



#[cfg(feature = "test")]
#[test]
fn test_vec_cast() {
    let a:Vec<u8> = vec![1, 2, 3];
    let b: &Vec<bool> = unsafe { core::mem::transmute(a.as_ptr()) };
    dbg!(b);
}


#[cfg(feature = "test")]
#[test]
fn test() {
}

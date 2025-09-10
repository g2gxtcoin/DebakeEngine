#[cfg(feature = "log_prf_profile_on")]
mod performance;
use std::{fs::File, io::Write, path, time};

use crate::application;

pub type LogSenderFunc = fn(logcode: u128, expect_mode: LogMode);

#[cfg(feature = "log_mode_editor")]
static CUREENT_LOG_MODE: LogMode = LogMode::Editor;
#[cfg(feature = "log_mode_rt")]
static CUREENT_LOG_MODE: LogMode = LogMode::RT;
#[cfg(feature = "log_mode_dev")]
static CUREENT_LOG_MODE: LogMode = LogMode::Dev;
#[cfg(feature = "log_mode_auto")]
static CUREENT_LOG_MODE: LogMode = LogMode::Auto;

pub const RELEVENT_LOG_PATH: &'static str = "log.txt";
#[cfg(feature = "config_MAX_LOGGER_BUFFER_LEN_8")]
pub const MAX_LOGGER_BUFFER_LEN: usize = 8;
#[cfg(feature = "config_MAX_LOGGER_BUFFER_LEN_32")]
pub const MAX_LOGGER_BUFFER_LEN: usize = 32;
#[cfg(feature = "config_MAX_LOGGER_BUFFER_LEN_64")]
pub const MAX_LOGGER_BUFFER_LEN: usize = 64;

//global var use for coding debuge info collects
static mut LOGGER_BUFFER: [u128; MAX_LOGGER_BUFFER_LEN] = [0; MAX_LOGGER_BUFFER_LEN];
static mut LOGGER_BEGING_TIME: u128 = 0;
static mut LOG_OFFSET: usize = 0;
static mut LOG_LAST_PRINT_OFFSET: usize = 0;
static mut LOG_LAST_OUT_OFFSET: usize = 0;
//set up these feature before compiling

//encode or decode u128-logcode builder
pub union LogCodeD {
    data: u128,
    part: [u16; 8],
}

pub struct LogPartFlag {}

impl LogPartFlag {
    pub const PART_0: usize = 0x1;
    pub const PART_1: usize = 0x2;
    pub const PART_2: usize = 0x4;
    pub const PART_3: usize = 0x8;
    pub const PART_4: usize = 0x10;
    pub const PART_5: usize = 0x20;
    pub const PART_6: usize = 0x40;
    pub const PART_7: usize = 0x80;

    pub const LOGGER_PART_DEFAULT: usize = 0x1;
    pub const LOGGER_PART_TYPE: usize = 0x1;
    pub const LOGGER_PART_CONDITION: usize = 0x2;
    pub const LOGGER_PART_LINE: usize = 0x4;
    pub const LOGGER_PART_EXE_ID: usize = 0x8;
    pub const LOGGER_PART_DAT_ID: usize = 0x10;
    pub const LOGGER_PART_TIME: usize = 0x20;
    pub const LOGGER_PART_FILE_ID: usize = 0x80;
    pub const LOGGER_PART_ALL: usize = Self::LOGGER_PART_TYPE
        | Self::LOGGER_PART_CONDITION
        | Self::LOGGER_PART_LINE
        | Self::LOGGER_PART_EXE_ID
        | Self::LOGGER_PART_DAT_ID
        | Self::LOGGER_PART_TIME
        | Self::LOGGER_PART_FILE_ID;
}

#[allow(dead_code)]
impl LogCodeD {
    pub fn new() -> Self {
        Self { data: 0u128 }
    }
    pub fn new_from(uin: u128) -> Self {
        Self { data: uin }
    }
    pub fn encode(&mut self, uin: u128, code_part: usize) -> Self {
        match code_part {
            crate::log::LogPartFlag::PART_0 => {
                return Self {
                    data: unsafe { self.data | (uin << 0) },
                }
            }
            crate::log::LogPartFlag::PART_1 => {
                return Self {
                    data: unsafe { self.data | (uin << 8) },
                }
            }
            crate::log::LogPartFlag::PART_2 => {
                return Self {
                    data: unsafe { self.data | (uin << 16) },
                }
            }
            crate::log::LogPartFlag::PART_3 => {
                return Self {
                    data: unsafe { self.data | (uin << 24) },
                }
            }
            crate::log::LogPartFlag::PART_4 => {
                return Self {
                    data: unsafe { self.data | (uin << 32) },
                }
            }
            crate::log::LogPartFlag::PART_5 => {
                return Self {
                    data: unsafe { self.data | (uin << 40) },
                }
            }
            crate::log::LogPartFlag::PART_6 => {
                return Self {
                    data: unsafe { self.data | (uin << 48) },
                }
            }
            crate::log::LogPartFlag::PART_7 => {
                return Self {
                    data: unsafe { self.data | (uin << 56) },
                }
            }
            _ => {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_CORE_WARN
                        | crate::log::code::CONDI_UNEXPECTED_RESULT
                        | crate::log::code::FILE_LOG
                        | LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                            .get_code()
                );
                return Self {
                    data: unsafe { self.data | uin * 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff },
                };
            }
        }
    }
    pub fn decode(&self, code_part: usize) -> u128 {
        unsafe {
            match code_part {
                crate::log::LogPartFlag::LOGGER_PART_TYPE => {
                    return self.data & (0xffff << 0)
                }
                crate::log::LogPartFlag::LOGGER_PART_CONDITION => {
                    return self.data & (0xffff << 8)
                }
                crate::log::LogPartFlag::LOGGER_PART_LINE => {
                    return self.data & (0xffff << 16)
                }
                crate::log::LogPartFlag::LOGGER_PART_EXE_ID => {
                    return self.data & (0xffff << 24)
                }
                crate::log::LogPartFlag::LOGGER_PART_DAT_ID => {
                    return self.data & (0xffff << 32)
                }
                LogPartFlag::LOGGER_PART_TIME => {
                    return self.data & (0xffffffff << 40)
                }
                crate::log::LogPartFlag::LOGGER_PART_FILE_ID => {
                    return self.data & (0xffff << 56)
                }
                crate::log::LogPartFlag::LOGGER_PART_ALL => {
                    return self.data & 0xffff_ffffffff_ffff_ffff_ffff_ffff_ffff
                }
                _ => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_CORE_WARN
                            | crate::log::code::CONDI_UNEXPECTED_RESULT
                            | crate::log::code::FILE_LOG
                            | LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                    );
                    return self.data & 0xffff_ffffffff_ffff_ffff_ffff_ffff_ffff;
                }
            }
        }
    }
    pub fn get_subcode(&self, code_part: usize) -> u16 {
        unsafe {
            match code_part {
                crate::log::LogPartFlag::LOGGER_PART_TYPE => self.part[0],
                crate::log::LogPartFlag::LOGGER_PART_CONDITION => self.part[1],
                crate::log::LogPartFlag::LOGGER_PART_LINE => self.part[2],
                crate::log::LogPartFlag::LOGGER_PART_EXE_ID => self.part[3],
                crate::log::LogPartFlag::LOGGER_PART_DAT_ID => self.part[4],
                crate::log::LogPartFlag::LOGGER_PART_TIME => todo!(),
                crate::log::LogPartFlag::LOGGER_PART_FILE_ID => self.part[7],
                crate::log::LogPartFlag::LOGGER_PART_ALL => todo!(),
                _ => {
                    crate::send2logger_dev!(
                        crate::log::code::TYPE_CORE_WARN
                            | crate::log::code::CONDI_UNEXPECTED_RESULT
                            | crate::log::code::FILE_LOG
                            | LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                                .get_code()
                    );
                    return self.part[0];
                }
            }
        }
    }
    pub fn get_time_ms(&self) -> u32 {
        unsafe {
            return u32::from(self.part[5]) + u32::from(self.part[6]) * 0x10000u32;
        }
    }
    pub fn get_code(&self) -> u128 {
        return unsafe { self.data };
    }
}

enum LogMode {
    Editor,
    RT,
    Dev,
    Auto,
}

fn output_base_log_info() {
    let mut absolote_logpath = std::env::current_dir().unwrap();
    absolote_logpath = absolote_logpath.join(RELEVENT_LOG_PATH);
    let log_mode_str = match CUREENT_LOG_MODE {
        LogMode::Editor => String::from("Editor"),
        LogMode::RT => String::from("RT"),
        LogMode::Dev => String::from("Dev"),
        LogMode::Auto => String::from("AUTO"),
    };
    let s1 = String::from(
        "###########DSE Log file Header######### \n".to_owned()
            + "progress start time:"
            + &crate::time::env::UtcTimeD::build()
                .build_from1970(
                    &time::SystemTime::now()
                        .duration_since(time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )
                .get_all_as_str()
            + "\n"
            + "log mode:"
            + &log_mode_str
            + "\n"
            + "########DSE Log file Header END######### \n",
    );
    let log_header = String::from(s1);
    let mut log_stream = match File::create(absolote_logpath) {
        Err(why) => panic!("couldn't create log file: {}", why),
        Ok(file) => file,
    };
    match log_stream.write(log_header.as_bytes()) {
        Ok(_) => {}
        Err(_err) => println!("log file save fail"),
    }
}

fn get_log_time() -> u128 {
    unsafe {
        return time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            - LOGGER_BEGING_TIME;
    }
}

fn decode_into_str(uin: u128) -> String {
    let code = LogCodeD::new_from(uin);
    return String::from(
        "type:".to_owned()
            + &code
                .get_subcode(crate::log::LogPartFlag::LOGGER_PART_TYPE)
                .to_string()
            + "|"
            + "condition:"
            + &code
                .get_subcode(crate::log::LogPartFlag::LOGGER_PART_CONDITION)
                .to_string()
            + "|"
            + "exeID:"
            + &code
                .get_subcode(crate::log::LogPartFlag::LOGGER_PART_EXE_ID)
                .to_string()
            + "|"
            + "datID:"
            + &code
                .get_subcode(crate::log::LogPartFlag::LOGGER_PART_DAT_ID)
                .to_string()
            + "|"
            + "time:"
            + &code.get_time_ms().to_string()
            + "|"
            + "fileID:"
            + &code
                .get_subcode(crate::log::LogPartFlag::LOGGER_PART_FILE_ID)
                .to_string()
            + "|"
            + "line:"
            + &code
                .get_subcode(crate::log::LogPartFlag::LOGGER_PART_LINE)
                .to_string()
            + "|",
    );
}

pub fn init() {
    unsafe {
        LOGGER_BEGING_TIME = get_log_time();
    }
    output_base_log_info();
}

//send log code to global log sys
pub fn send2logger(uin: u128) {
    unsafe {
        if LOG_OFFSET < MAX_LOGGER_BUFFER_LEN {
            LOGGER_BUFFER[LOG_OFFSET] = uin
                | LogCodeD::new()
                    .encode(get_log_time(), LogPartFlag::LOGGER_PART_TIME)
                    .get_code();
            LOG_OFFSET = LOG_OFFSET + 1;
        } else if LOG_OFFSET == MAX_LOGGER_BUFFER_LEN {
            LOGGER_BUFFER[MAX_LOGGER_BUFFER_LEN - 1] = code::TYPE_CORE_ERROR
                | code::CONDI_ARRAY_OVERFLOW
                | LogCodeD::new()
                    .encode(get_log_time(), LogPartFlag::LOGGER_PART_TIME)
                    .get_code();
            output_log2file();
            output_custom2file(String::from(
                "#########!!!!#####LOGGER_ARRAY_OVERFLOW#####!!!!#########\n",
            ));
            clear_logbuffer();
        } else {
            panic!("unexpected log error,log buffer over flow!!");
        }
    }
}

// send log code to global log sys and output all log to log file imidiatly
pub fn sorry(uin: u128) {
    unsafe {
        if LOG_OFFSET < MAX_LOGGER_BUFFER_LEN {
            LOGGER_BUFFER[LOG_OFFSET] = uin
                | LogCodeD::new()
                    .encode(get_log_time(), LogPartFlag::LOGGER_PART_TIME)
                    .get_code();
            LOG_OFFSET = LOG_OFFSET + 1;
        } else if LOG_OFFSET == MAX_LOGGER_BUFFER_LEN {
            LOGGER_BUFFER[MAX_LOGGER_BUFFER_LEN - 1] = code::TYPE_CORE_ERROR
                | code::CONDI_ARRAY_OVERFLOW
                | LogCodeD::new()
                    .encode(get_log_time(), LogPartFlag::LOGGER_PART_TIME)
                    .get_code();
            output_log2file();
            output_custom2file(String::from(
                "\n\n##############LOGGER_ARRAY_OVERFLOW##############\n\n",
            ));
            clear_logbuffer();
        } else {
            panic!(
                "
            unexpected LOG error behavior!!!!!\n
            it means log from now to last output has been lost \n
            good luck for you"
            );
        }
    }
    println!("{}", KOKOMI_SORRY_IMG);
    print2console();
    output_log2file();
}

//this func will not encode time into your code
pub fn send_complete_code2logger(uin: u128) {
    unsafe {
        if LOG_OFFSET < MAX_LOGGER_BUFFER_LEN {
            LOGGER_BUFFER[LOG_OFFSET] = uin;
            LOG_OFFSET = LOG_OFFSET + 1;
        } else if LOG_OFFSET == MAX_LOGGER_BUFFER_LEN {
            LOGGER_BUFFER[MAX_LOGGER_BUFFER_LEN - 1] = code::TYPE_CORE_ERROR
                | code::CONDI_ARRAY_OVERFLOW
                | LogCodeD::new()
                    .encode(get_log_time(), LogPartFlag::LOGGER_PART_TIME)
                    .get_code();
            output_log2file();
            output_custom2file(String::from(
                "##############LOGGER_ARRAY_OVERFLOW##############\n",
            ));
            clear_logbuffer();
        } else {
            panic!("unexpect log error,log buffer over flow!!");
        }
    }
}

//print all log buffer formate to console
#[allow(dead_code)]
#[cfg(feature = "log_print_mode_limit_decode")]
pub fn print2console() {
    unsafe {
        for logi in 0..LOG_OFFSET {
            println!("{}", decode_into_str(LOGGER_BUFFER[logi]));
        }
    }
}

#[cfg(feature = "log_print_mode_entire_code")]
pub fn print2console() {
    unsafe {
        for logi in 0..LOG_OFFSET {
            println!("{:x}", LOGGER_BUFFER[logi]);
        }
    }
}

//print all log buffer formate to console if have not been clear
#[cfg(feature = "log_print_mode_limit_decode")]
pub fn print2console_once() {
    unsafe {
        if LOG_OFFSET > LOG_LAST_PRINT_OFFSET {
            for logi in LOG_LAST_PRINT_OFFSET..LOG_OFFSET {
                println!("{}", decode_into_str(LOGGER_BUFFER[logi]));
            }
            LOG_LAST_PRINT_OFFSET = LOG_OFFSET;
        } else if LOG_OFFSET < LOG_LAST_PRINT_OFFSET {
            crate::send2logger_dev!(
                crate::log::code::TYPE_CORE_ERROR
                    | crate::log::code::CONDI_UNEXPECTED_RESULT
                    | crate::log::code::FILE_LOG
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code()
            );
        }
    }
}

#[cfg(feature = "log_print_mode_entire_code")]
pub fn print2console_once() {
    unsafe {
        if LOG_OFFSET > LOG_LAST_PRINT_OFFSET {
            for logi in LOG_LAST_PRINT_OFFSET..LOG_OFFSET {
                println!("{:x}", LOGGER_BUFFER[logi]);
            }
            LOG_LAST_PRINT_OFFSET = LOG_OFFSET;
        } else if LOG_OFFSET < LOG_LAST_PRINT_OFFSET {
            crate::log::send2logger(
                crate::log::code::TYPE_CORE_ERROR
                    | crate::log::code::CONDI_UNEXPECTED_RESULT
                    | crate::log::code::FILE_LOG
                    | crate::log::LogCodeD::new()
                        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
                        .get_code(),
            );
        }
    }
}

pub fn clear_logbuffer() {
    unsafe {
        LOG_OFFSET = 0;
        LOG_LAST_OUT_OFFSET = 0;
        LOG_LAST_PRINT_OFFSET = 0;
    }
}

#[allow(dead_code)]
pub fn drop_logbuffer() {
    unsafe {
        drop(LOGGER_BUFFER);
        drop(LOGGER_BEGING_TIME);
        drop(LOG_OFFSET);
        drop(RELEVENT_LOG_PATH);
    }
}

//remember to clear buffer
//recommend to use [output_clear_log2file_once] function
#[cfg(feature = "log_print_mode_limit_decode")]
pub fn output_log2file() {
    let mut absolote_logpath = std::env::current_dir().unwrap();
    absolote_logpath = absolote_logpath.join(RELEVENT_LOG_PATH);
    let mut log_stream = match File::options().append(true).open(absolote_logpath) {
        Err(why) => panic!("couldn't create log file: {}", why),
        Ok(file) => file,
    };
    let mut str_buffer = String::new();
    unsafe {
        for logi in 0..LOG_OFFSET {
            str_buffer.push('\n');
            str_buffer.push_str(decode_into_str(LOGGER_BUFFER[logi]).as_str());
        }
    }
    match log_stream.write(str_buffer.as_bytes()) {
        Ok(_) => {}
        Err(_err) => println!("log file save fail"),
    }
}

#[cfg(feature = "log_print_mode_entire_code")]
pub fn output_log2file() {
    let mut absolote_logpath = std::env::current_dir().unwrap();
    absolote_logpath = absolote_logpath.join(RELEVENT_LOG_PATH);
    let mut log_stream = match File::options().append(true).open(absolote_logpath) {
        Err(why) => panic!("couldn't create log file: {}", why),
        Ok(file) => file,
    };
    let mut str_buffer = String::new();
    unsafe {
        for logi in 0..LOG_OFFSET {
            str_buffer.push('\n');
            for ui in LOGGER_BUFFER[logi].to_le_bytes().iter() {
                str_buffer.push_str(&ui.to_string());
            }
        }
    }
    match log_stream.write(str_buffer.as_bytes()) {
        Ok(_) => {}
        Err(_err) => println!("log file save fail"),
    }
}

//out put all log buffer format to local exe dir in log.txt
pub fn output_custom2file(sin: String) {
    let mut absolote_logpath = std::env::current_dir().unwrap();
    absolote_logpath = absolote_logpath.join(RELEVENT_LOG_PATH);
    let mut log_stream = match File::options().append(true).open(absolote_logpath) {
        Err(why) => panic!("couldn't create log file: {}", why),
        Ok(file) => file,
    };
    match log_stream.write(sin.as_bytes()) {
        Ok(_) => {}
        Err(_err) => println!("log file save fail"),
    }
}

// out put all log buffer format to local exe dir in log.txt
// this func will decided if should put&clear log buffer by linear harf devided judge
// this func can reduce io performance consumption Significantly
// by write log buffer in group
#[cfg(feature = "log_print_mode_limit_decode")]
pub fn output_clear_log2file_once() {
    unsafe {
        if (2 * LOG_OFFSET - LOG_LAST_OUT_OFFSET) >= MAX_LOGGER_BUFFER_LEN && LOG_OFFSET != 0 {
            let mut absolote_logpath = std::env::current_dir().unwrap();
            absolote_logpath = absolote_logpath.join(RELEVENT_LOG_PATH);
            let mut log_stream = match File::options().append(true).open(absolote_logpath) {
                Err(why) => panic!("couldn't create log file: {}", why),
                Ok(file) => file,
            };
            let mut str_buffer = String::new();

            for logi in 0..LOG_OFFSET {
                str_buffer.push('\n');
                str_buffer.push_str(decode_into_str(LOGGER_BUFFER[logi]).as_str());
            }

            match log_stream.write(str_buffer.as_bytes()) {
                Ok(_) => {}
                Err(_err) => println!("log file save fail"),
            }
            clear_logbuffer();
        } else {
            LOG_LAST_OUT_OFFSET = LOG_OFFSET;
        }
    }
}

#[cfg(feature = "log_print_mode_entire_code")]
pub fn output_clear_log2file_once() {
    unsafe {
        if (2 * LOG_OFFSET - LOG_LAST_OUT_OFFSET) >= MAX_LOGGER_BUFFER_LEN && LOG_OFFSET != 0 {
            let mut absolote_logpath = std::env::current_dir().unwrap();
            absolote_logpath = absolote_logpath.join(RELEVENT_LOG_PATH);
            let mut log_stream = match File::options().append(true).open(absolote_logpath) {
                Err(why) => panic!("couldn't create log file: {}", why),
                Ok(file) => file,
            };
            let mut str_buffer = String::new();

            for logi in 0..LOG_OFFSET {
                str_buffer.push('\n');
                for ui in LOGGER_BUFFER[logi].to_le_bytes().iter() {
                    str_buffer.push_str(&ui.to_string());
                }
            }

            match log_stream.write(str_buffer.as_bytes()) {
                Ok(_) => {}
                Err(_err) => println!("log file save fail"),
            }
            clear_logbuffer();
        } else {
            LOG_LAST_OUT_OFFSET = LOG_OFFSET;
        }
    }
}

#[cfg(feature = "log_mode_dev")]
#[macro_export]
macro_rules! send2logger_dev {
    ($state: expr) => {
        crate::log::send2logger($state)
    };
}

#[cfg(not(feature = "log_mode_dev"))]
#[macro_export]
macro_rules! send2logger_dev {
    ($state: expr) => {
        ()
    };
}

#[cfg(feature = "log_mode_rt")]
#[macro_export]
macro_rules! send2logger_rt {
    ($state: expr) => {
        crate::log::send2logger($state)
    };
}

#[cfg(not(feature = "log_mode_rt"))]
#[macro_export]
macro_rules! send2logger_rt {
    ($state: expr) => {
        ()
    };
}

#[cfg(feature = "log_mode_editor")]
#[macro_export]
macro_rules! send2logger_editor {
    ($state: expr) => {
        crate::log::send2logger($state)
    };
}

#[cfg(not(feature = "log_mode_editor"))]
#[macro_export]
macro_rules! send2logger_editor {
    ($state: expr) => {
        ()
    };
}

#[macro_export]
macro_rules! send2logger_auto {
    ($state: expr) => {
        crate::log::send2logger($state)
    };
}

#[cfg(feature = "log_mode_dev")]
#[macro_export]
macro_rules! dbg_dev {
    ($state: expr) => {
        dbg!($state)
    };
}

#[cfg(not(feature = "log_mode_dev"))]
#[macro_export]
macro_rules! dbg_dev {
    ($state: expr) => {
        ()
    };
}

/*********send**log**macro**stencil******
**********日志**发送**宏函数**模板*********

 /// 基础款
 /// 会在任何时候 进行编译并执行
 /// 性能消耗较大
 /// 好处是 编写时编译器会给你提示
 /// 很爽
 ///
  crate::send2logger(
    crate::log::code::TYPE_
    | crate::log::code::CONDI_
    | crate::log::code::FILE_
    | crate::log::LogCodeD::new()
        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
        .get_code()
    | crate::log::LogCodeD::new()
        .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_PART_ID)
        .get_code()
);

/// 宏函数款
/// 其示例为:开发模式打印日志
/// 仅在规定的模式下执行
/// 非规定的模式 会执行一段空函数
/// 也就是什么都不干
/// 性能消耗小
/// 坏处是 编写时编译器不会给你提示
/// 不过吧
/// 其实可以先用上面的写完然后加3字母和一个感叹号规定模式
/// 这两参数一样的
///
crate::send2logger_dev!(
    crate::log::code::TYPE_
    | crate::log::code::CONDI_
    | crate::log::code::FILE_
    | crate::log::LogCodeD::new()
        .encode(line!() as u128, crate::log::LogPartFlag::LOGGER_PART_LINE)
        .get_code()
    | crate::log::LogCodeD::new()
        .encode(self.id as u128, crate::log::LogPartFlag::LOGGER_)
        .get_code()
);

**********send**log****stencil******/

///about log info code
///use hexadecimal to construct error code
///0-3 pos is log type
///4-7 pos is condition info
/// 8-11 pos is line info
///12-15 pos is exeID info
/// 16-19 pos is datID info
/// 20-27 pos is time info
/// 28-31 pos is fileID info
#[allow(dead_code)]
pub mod code {
    /************************************************************************************file***time*****did**eid**lin**con**type */
    pub const TYPE_DEFAULT:/****************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0000;
    pub const TYPE_CORE_ERROR:/*************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0001;
    pub const TYPE_CORE_WARN:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0002;
    pub const TYPE_CORE_INFO:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0003;
    pub const TYPE_EXT_ERROR:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0004;
    pub const TYPE_EXT_WARN:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0005;
    pub const TYPE_EXT_INFO:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0006;
    pub const TYPE_EXE_ERROR:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0007;
    pub const TYPE_EXE_WARN:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0008;
    pub const TYPE_EXE_INFO:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0009;
    pub const TYPE_DAT_ERROR:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_000a;
    pub const TYPE_DAT_WARN:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_000b;
    pub const TYPE_DAT_INFO:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_000c;
    pub const TYPE_MEAT_ERROR:/*************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_000d;
    pub const TYPE_MEAT_WARN:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_000e;
    pub const TYPE_MEAT_INFO:/**************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_000f;
    pub const TYPE_TRAIT_ERROR:/************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0010;
    pub const TYPE_TRAIT_WARN:/*************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0011;
    pub const TYPE_TRAIT_INFO:/*************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0012;
    /************************************************************************************file***time*****did**eid**lin**con**type */

    /************************************************************************************file***time*****did**eid**lin**con**type */
    pub const CONDI_DEFAULT:/***************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0000;
    pub const CONDI_UNDEFINE_CONDI:/********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0000;
    pub const CONDI_UNEXPECTED_RESULT:/*****************************************/ u128 = 0x0000_00000000_0000_0000_0000_0001_0000;
    pub const CONDI_ARRAY_OVERFLOW:/********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0002_0000;
    pub const CONDI_NUM_OVERFLOW:/**********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0002_0000;
    pub const CONDI_RECIEVE_WIN_MSG_SUSSECE:/***********************************/ u128 = 0x0000_00000000_0000_0000_0000_0003_0000;
    pub const CONDI_RECIEVE_WIN_MSG_FAIL:/**************************************/ u128 = 0x0000_00000000_0000_0000_0000_0004_0000;
    pub const CONDI_DROP_FAIL:/*************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0005_0000;
    pub const CONDI_DROP_SUSSECE:/**********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0006_0000;
    pub const CONDI_VK_DEBUG_UTIL:/*********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0007_0000;
    pub const CONDI_VK_DEVICE_NOT_FOUND:/***************************************/ u128 = 0x0000_00000000_0000_0000_0000_0008_0000;
    pub const CONDI_VK_BUILDER_PREBUILD_NOT_BUILD:/*****************************/ u128 = 0x0000_00000000_0000_0000_0000_0009_0000;
    pub const CONDI_VK_UNEXPRCTED_EXT_NAME:/************************************/ u128 = 0x0000_00000000_0000_0000_0000_000a_0000;
    pub const CONDI_VK_INSTANCE_NOT_FOUND:/*************************************/ u128 = 0x0000_00000000_0000_0000_0000_000b_0000;
    pub const CONDI_VK_EXT_NAME_NOT_FOUND:/*************************************/ u128 = 0x0000_00000000_0000_0000_0000_000c_0000;
    pub const CONDI_CREATE_SURFACE_FALI:/***************************************/ u128 = 0x0000_00000000_0000_0000_0000_000d_0000;
    pub const CONDI_GPU_SURFACE_SUILTABAL:/*************************************/ u128 = 0x0000_00000000_0000_0000_0000_000e_0000;
    pub const CONDI_GPU_SURFACE_NOT_SUILTABAL:/*********************************/ u128 = 0x0000_00000000_0000_0000_0000_000f_0000;
    pub const CONDI_VK_DEBUG_PRINT2NEXT_LOG:/***********************************/ u128 = 0x0000_00000000_0000_0000_0000_0010_0000;
    pub const CONDI_VK_SURFACE_LOADER_NOT_FOUND:/*******************************/ u128 = 0x0000_00000000_0000_0000_0000_0011_0000;
    pub const CONDI_DAT_SIZE_LOCK_PUSH_FAIL:/***********************************/ u128 = 0x0000_00000000_0000_0000_0000_0012_0000;
    pub const CONDI_FILE_IS_EMPTY:/*********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0013_0000;
    pub const CONDI_FILE_LOAD2BUFFER_FAIL:/*************************************/ u128 = 0x0000_00000000_0000_0000_0000_0014_0000;
    pub const CONDI_OPTION_NONE:/***********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0015_0000;
    pub const CONDI_NULL_STRING_EXPECTED:/**************************************/ u128 = 0x0000_00000000_0000_0000_0000_0016_0000;
    pub const CONDI_BUILD_START:/***********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0017_0000;
    pub const CONDI_BUILD_END:/*************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0018_0000;
    pub const CONDI_FUNC_NOT_IMPLENMENT:/***************************************/ u128 = 0x0000_00000000_0000_0000_0000_0019_0000;
    pub const CONDI_MEM_LEAK_DAT_REALLOC:/**************************************/ u128 = 0x0000_00000000_0000_0000_0000_001a_0000;
    pub const CONDI_TASK_QUEUE_UNSORT:/*****************************************/ u128 = 0x0000_00000000_0000_0000_0000_001b_0000;
    pub const CONDI_DATA_NOT_FOUND:/********************************************/ u128 = 0x0000_00000000_0000_0000_0000_001c_0000;
    pub const CONDI_INVAILD_EXE_ID:/********************************************/ u128 = 0x0000_00000000_0000_0000_0000_001d_0000;
    pub const CONDI_UNDEFINE_BEHAVIOR:/*****************************************/ u128 = 0x0000_00000000_0000_0000_0000_001e_0000;
    pub const CONDI_ELEMENT_NOT_FOUND:/*****************************************/ u128 = 0x0000_00000000_0000_0000_0000_001f_0000;
    pub const CONDI_VK_GPU_LIMITIS:/********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0020_0000;
    pub const CONDI_UNMATCH_ENUM:/**********************************************/ u128 = 0x0000_00000000_0000_0000_0000_0021_0000;
    pub const CONDI_UNDEFINE_TRAIT_MEM:/****************************************/ u128 = 0x0000_00000000_0000_0000_0000_0022_0000;
    pub const CONDI_ENTITY_NOT_INIT:/*******************************************/ u128 = 0x0000_00000000_0000_0000_0000_0023_0000;
    pub const CONDI_ONLY_DATA_EXIST:/*******************************************/ u128 = 0x0000_00000000_0000_0000_0000_0024_0000;
    pub const CONDI_RENDER_PIPELINE_NOT_FOUND:/*********************************/ u128 = 0x0000_00000000_0000_0000_0000_0025_0000;
    pub const CONDI_INVAILD_PARAMETER_INPUT:/***********************************/ u128 = 0x0000_00000000_0000_0000_0000_0026_0000;
    pub const CONDI_NOMAP_DEVICE_MEM_MODE:/*************************************/ u128 = 0x0000_00000000_0000_0000_0000_0027_0000;
    pub const CONDI_INIT_IMCOMPLETE:/*******************************************/ u128 = 0x0000_00000000_0000_0000_0000_0028_0000;
    /************************************************************************************file***time*****did**eid**lin**con**type */

    /************************************************************************************file***time*****did**eid**lin**con**type */
    pub const FILE_LOG:/********************************************************/ u128 = 0xffff_00000000_0000_0000_0000_0000_0000;
    pub const FILE_MAIN:/*******************************************************/ u128 = 0x0000_00000000_0000_0000_0000_0000_0000;
    pub const FILE_WINDOW:/*****************************************************/ u128 = 0x0001_00000000_0000_0000_0000_0000_0000;
    pub const FILE_EXTAPI_GRAPHIC_VK:/******************************************/ u128 = 0x0002_00000000_0000_0000_0000_0000_0000;
    pub const FILE_RENDERER:/***************************************************/ u128 = 0x0003_00000000_0000_0000_0000_0000_0000;
    pub const FILE_INPUT_WIN:/**************************************************/ u128 = 0x0004_00000000_0000_0000_0000_0000_0000;
    pub const FILE_MANAGER_DATUM:/**********************************************/ u128 = 0x0005_00000000_0000_0000_0000_0000_0000;
    pub const FILE_RESOURCE:/***************************************************/ u128 = 0x0006_00000000_0000_0000_0000_0000_0000;
    pub const FILE_SHADER:/*****************************************************/ u128 = 0x0007_00000000_0000_0000_0000_0000_0000;
    pub const FILE_APPLICATION:/************************************************/ u128 = 0x0008_00000000_0000_0000_0000_0000_0000;
    pub const FILE_META:/*******************************************************/ u128 = 0x0009_00000000_0000_0000_0000_0000_0000;
    pub const FILE_EXECUTE:/****************************************************/ u128 = 0x000a_00000000_0000_0000_0000_0000_0000;
    pub const FILE_TIME:/*******************************************************/ u128 = 0x000b_00000000_0000_0000_0000_0000_0000;
    pub const FILE_NODE:/*******************************************************/ u128 = 0x000c_00000000_0000_0000_0000_0000_0000;
    pub const FILE_MODEL:/******************************************************/ u128 = 0x000d_00000000_0000_0000_0000_0000_0000;
    pub const FILE_REGISTER:/***************************************************/ u128 = 0x000e_00000000_0000_0000_0000_0000_0000;
    pub const FILE_CONVERT_TOML:/***********************************************/ u128 = 0x000f_00000000_0000_0000_0000_0000_0000;
    pub const FILE_RENDERER_PIPELINE:/******************************************/ u128 = 0x0010_00000000_0000_0000_0000_0000_0000;
    pub const FILE_INPUT:/******************************************************/ u128 = 0x0011_00000000_0000_0000_0000_0000_0000;
    pub const FILE_MODEL_RECTANGLE:/********************************************/ u128 = 0x0012_00000000_0000_0000_0000_0000_0000;
    pub const FILE_RENDERER_BUFFER:/********************************************/ u128 = 0x0013_00000000_0000_0000_0000_0000_0000;
    /************************************************************************************file***time*****did**eid**lin**con**type */
}

//
#[allow(unused)]
const FOLD: &'static str = "hahaha";
const KOKOMI_SORRY_IMG: &'static str = r#"
############################################################################################################
############################################################################################################
                                                                                                            
    *@`               ^\ =@....@ ******************\@@@@       [*  [ \@@@[***********\ .. @\\               
      *@*             ^==@.....@ *********************@@^    * ]@ ]@@ ********* @******^...**               
    *@@@@@*[@         @=@...... @************************  ^@ [@@@************ @@@*  *=^....@^**            
    *@@@****=         =@.........*]********************* @* *@@***************=@@O*^* @.....=@              
     =@@@OO@ * *[^o*^=@......*.....[@*]*]]@@@@@[[[[[[[[[[*@@@@]] *************@@@@^@@ .......@^=            
     =@O@@OO@*[@   = @^.......  . ]@@[ ...........................[*@]]****** @@@@@^@........@^ ^           
      @@@OOOOO@*@@  *@ ......]@@*]] .................... ]]]]] ........ ** ...=@@@@@ @.......@^@            
    * =@@@OOOOOOO@@] @^.. @@[........................[ .........[@* .......[@ .@@@@@@^@ .....@^             
    *@@@@@@OOOOOOo*@@@@@ ........................................... ** ......*@@@@@@@O**.   @              
    =@@*=@@@OOOOOo@O@ ..................................................**......*@@@@@@@****@^=*            
     @@@@OO@@OOOo@@....................................................... * ....=@@@@@@@O**@ [@            
     *@@OOOO@@O@ ...........................................  .............. *...* @@@@@@OOOO@@@@           
     @@@@OOOO@@................@..............................* ..............=**=@@@@@@@OOOOOO@*=          
     =@@@@OO@................ ^.......................*........*^...............* @@@@@@@OOOOOOOo@ @        
     *@O@@@^................@ .........................@........=^...............=@@@@@O@@OOOOOoo^@^^       
      @@O@ ................@ .......................... *........*^............... @@@@O@@OOOOOooo @^       
     @ @@.................@ ............................@ ........* ............... @O@OOO@OOOOoo* =@       
     **@ ............ @..@^.............................=^.........@................ @@@OO^*OOOooo^=@       
     *@ ............=^..=^...............................@......... *..............@O@@@OOO*OOOOoo==@*      
     @@............ @...@...@............................@^.........=^............. @OOO@OOOOOOoo^ @^=      
    =@.............@...=^...@............................=@ .........@...............*@OOoOOOoo@**@@        
  =^@^............@ .. @^..=^............................=^@*........=^...............=@OOoOOOo^*@^[        
 ^o=@............=^..=^@...@^..........*..............*..=^ @*.....*..@.................@OOoOOo^*=@         
  ==@............@..=@.@...@@..........O..............*..=^*=*  ....*.=^...............=@@@Oo o^ =@         
 @ @^...........=^. @..@ .=^@..........=^.............=^.=^  =*  *..  *@................@**@O^  @@^^*       
 ^^@ ...........@..@...=^.@[*^.........=^............*=@.=@].* @   .  **^...............=@*=@o  =@ ]        
 @ @............@.@^*.=@^.@.=^.@.......=@............*=@.@ ]]@].*^*   *=^................@ **@* @^^         
  =@...........=^ @....*@.@*.@.=^.......@^...........*@@.@.....   @   * @................=@  @O@@ [         
 ==@...........=^@^*....=^@ *=*.@ ......@@..........*=@^@^*.....  * @   @............... .@*=@@ *           
 *=@...........=*@......*@=^*.*^@@......=@^.........*@=*@*........  * @ @...............  *^@@] @           
  =@............@^.......=@^...*@@*......@=^....... @*@@...........    .@...............O==@@               
 * @........*...@^*.....]]@@@@**@@@@ ....=*=*....* @.@OO[[[[[O@] ....  .@.............=.O=@@^               
  ^@^. @.... ...=^. . ]]@@@@@@@]]]@  @@@[*@^.* ..[ ]]]]@@@@@@@@*]]]] ..=@.......].....= @]^@@=              
 * =@.=@^...**...@]@@@@@@[*@@@@@@@@*..*........... @@@@@@@@@@@@@@@@@@ .=^......=^.... **@=]@@               
  =*@^=@@...== .. @ *@..]]@@@@@@@@@...................@@@@@@@@@@..@@@ .@..... .@^....@@=@=*@@=*             
    @@ @=@..*** ..=*....*@O[ * *@@ .................=@O[... O@@^. ....=^..... =@ ...=* @@@*^@^^             
     @@@***..=*@ ..=*....[***]OOO[...................[*O]]]]O@[......=^.... o=@^...@*@ ...=^@^[             
     * @@^ @*@@@@]^* @..............................................@^.... @@*@.]]@@......=^@@              
        *[@^.......@[*@@................  ........................ @ ... ]@@.[ .@.........=^=@              
    * * ==@........@............................................ @@@[[[[....... @.........=^=@^^            
     *^=*@^........=^............................................ @............=^..........@.@**            
    *  *=@..........@.....................]]  @..................@.............@ ..........@.*@@            
     *^[@^..........@@ ......................= .................@ .............@...........=^=@^            
    *  *@.......... @@@* ......................................@@.............=@^...........@.@@            
     *==@..........@@   [@@]...............................]@@@@ .............=^@...........=^@=^* @        
     ^^=^....=^...=@*        *@@*] ...................]@@@@@@@^@=^............@* *...........*@^@ ] @       
     * @^....@*.* @*          @    [*@@@@]]]]]]]@@@@@@@@@@@@@@@@@............=@   @ ..........*@ @@@^@*@    
     =*@^....@  =@           ^ @@@@@@^      @@@@@@@@@@@@@@@@@ =@^............@^    **... *..... *.@@* @@^   
     * @^....@*@@]    ]* @* @*@@..@@[ ..[[[@@@^=@. =@@@@@@[. @@@ ........... @]     *** .. *].... [[[[@^]=* 
    **^@^....=@ ^*  [[[[ @*=@.*^@ .........@@@@@]@OO@^. ]@@@@@@@............@^.. **  *]@**].. *@@@@@@@@ * * 
    ****^.....@^ *@@ . @[**=@..@@@@@@@ .....=@@@@@@@@@@@@@@@@@@@........... @...]@@@@ . @  ]]  *=^..@]* *   
       =@......*@ ......**@@@.@=*@@....[@ ...*@@@@@@@@@@@@@@@[.@.....*.....@@ @^...[....@@ .=^  .*.. @=*   *
     ^]*=@.....@.........[@*@=*@......... @@*.@@O@@@@ .[[[ ... @....*.... =@.*^.........^...@^  *.^..=@     
     =*^***..*@...........=^@@ ........... @^=@]OO@...........=^...* ...**@.]@@@@@@@@@@@@@@*@    *= ..=**   
        * @@*.@]...........*@ .............=^.=@@=^.......... @....*@.. =@@@@@@@@@@@@@@@@@@@^    *.....*^@ *
         **  *@@ ..........=@............. @...=@@ ......... @...*^@^. @^* [@@@*@@@@@@@@@@@@^  ]]]]] ...*^*]
            * =@*^........=@=*.........]@@[...... .**....@@@ ... @@@ @@@@@*]  *@@@@@@@@@@@@@[@@ ...... * O *
              @^* **... @@@..=^......@ .............. @@OOOO@@@@@@@@ @@@@@@@@@@O@@@@@@@   ]    =*.......*.o.
             @@^       *=@.....**.  =@.................*@OOOOOO@OO@^ @@@@@@@@@@@o@ *@O@@@@@ **  @.......=^.O
            @* @*      *@......@*[@@@@...................@@ ......=^*=@@@@@@@@@@@@@o@@@@@@^..@ *@........@. 
            @@@@     * =^] ...=@    @^....................* ]]@OOOO@^   *@@@@@@[[[*@@@@@@@...@* @........=^.
        ^    @@       *=@=^[*.=@  =^............ ..........=@@OOOO@@@@@  * O@ ....  *OO@@...@  @^.........=^
        **@ @@@      * * @ ..*]@   @............. * @@@@]]]]@@@@@@@@@@@@@@@@@.....  =@***[*.  *@@@@@*].....@
                                                                                                            
############################################################################################################
############################################################################################################
                                                                                                            
@@@@@@@@` @. .@^   .@^@@@@@@@@@@@  /^   ]/OO@@@@@O.            =O.        .\@` O@@@@@@@@@. .@/[[\O=@@@@@@@@`
@O    .@^ @^ .@^  .O/............  ,@@@@^]..=O.  // =O.       .]=O`                     =@. .@OOOOO ...@....
@@@@@@@@^ @^ .@^ ,@^=OOOOOOOOOO/  ,@`.=O.,@. \\.//         ,@^  =O. \@.   .@^ .]]]]]]]. =@. .@]]]/O @^.,..O^
..=@.  @. @^ .@^.O\]]]]]]]]]]]]   .`=O..  .=O.  .         .@^   =O.  \@.  .@^ .@.   .@. =@. .[....` @^ O^ O^
@@@@@@@@^ @^ .@^             =O     =O .[[\O[[[[@[[.     .@/    =O.   \@. .@^ .@.   .@. =@..[[[\O[[`@^.O^ O^
  /@  .@^ @^ .@^             =O..   =O   =@]. ,@^       .@/.    =O.   .\@..@^ .@@@@@@@. =@. .@.=@@@^..=@O`..
 /@.  .@` .. .@^             ,@`=O  =O/@`  .=@@\.        ..     =O.     . .@^ ...       =@. =@O/O  ,/@`  ,@\
@/.O@@@/  \@@@@.              =@@^ .\/.,]O@/`. .[@\.        ,@@@@/        .@^       .]]]OO.,@`.,\@@@@@@@@@@@
                                                                                                            
############################################################################################################
############################################################################################################
 "#;

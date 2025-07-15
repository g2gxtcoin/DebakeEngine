#[cfg(windows)]
#[cfg(feature="env_bit_64bit")]
#[cfg(feature="env_os_win10")] 
pub mod env{
    use std::{fs::File, io::{self, Read}};

    use crate::manager::datum::env::Datum;

    pub struct  Defile{
        id:u64,
        pub fstream:Option<File>
    }

    impl Defile {
        pub fn to_string(&mut self)-> Result<String,()>{
            let mut strbuf = std::string::String::new();
            if self.fstream.is_some() {
                match self.fstream.as_mut().unwrap().read_to_string(&mut strbuf) {
                    Ok(s) => {
                        if s == 0 {
                            crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_WARN
                                    | crate::log::code::CONDI_FILE_IS_EMPTY
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode( self.id as u128, crate::log::LogCodePart::Id)
                                        .get_code()
                            )
                        }
                    }
                    Err(_) => {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_EXE_ERROR
                                | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                | crate::log::code::FILE_RESOURCE
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(self.id as u128, crate::log::LogCodePart::Id)
                                    .get_code()
                        );
                    }
                }
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXE_WARN
                        | crate::log::code::CONDI_OPTION_NONE
                        | crate::log::code::FILE_RESOURCE
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }
            return Ok(strbuf);
        }
    
        pub fn to_byte(&mut self)-> Result<Vec<u8>,()>{
            let mut buf = Vec::new();

            if self.fstream.is_some() {
                match self.fstream.as_mut().unwrap().read_to_end(&mut buf) {
                    Ok(s) => {
                        if s == 0 {
                            crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_WARN
                                    | crate::log::code::CONDI_FILE_IS_EMPTY
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(self.id as u128, crate::log::LogCodePart::Id)
                                        .get_code()
                            )
                        }
                    }
                    Err(_) => crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                            | crate::log::code::FILE_RESOURCE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code()
                    ),
                }
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXE_WARN
                        | crate::log::code::CONDI_OPTION_NONE
                        | crate::log::code::FILE_RESOURCE
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }
            Ok(buf)
        }
    
        pub fn into_string(mut self)-> Result<String,()>{
            let mut strbuf = std::string::String::new();
            if self.fstream.is_some() {
                match self.fstream.as_mut().unwrap().read_to_string(&mut strbuf) {
                    Ok(s) => {
                        if s == 0 {
                            crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_WARN
                                    | crate::log::code::CONDI_FILE_IS_EMPTY
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(self.id as u128, crate::log::LogCodePart::Id)
                                        .get_code()
                            )
                        }
                    }
                    Err(_) => {
                        crate::send2logger_dev!(
                            crate::log::code::TYPE_EXE_WARN
                                | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                                | crate::log::code::FILE_RESOURCE
                                | crate::log::LogCodeD::new()
                                    .encode(line!() as u128, crate::log::LogCodePart::Line)
                                    .get_code()
                                | crate::log::LogCodeD::new()
                                    .encode(self.id as u128, crate::log::LogCodePart::Id)
                                    .get_code()
                        );
                    }
                }
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXE_WARN
                        | crate::log::code::CONDI_OPTION_NONE
                        | crate::log::code::FILE_RESOURCE
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }
            return Ok(strbuf);
        }
    
        pub fn into_byte(mut self)-> Result<Vec<u8>,()>{
            let mut buf = Vec::new();

            if self.fstream.is_some() {
                match self.fstream.as_mut().unwrap().read_to_end(&mut buf) {
                    Ok(s) => {
                        if s == 0 {
                            crate::send2logger_dev!(
                                crate::log::code::TYPE_EXE_ERROR
                                    | crate::log::code::CONDI_FILE_IS_EMPTY
                                    | crate::log::code::FILE_RESOURCE
                                    | crate::log::LogCodeD::new()
                                        .encode(line!() as u128, crate::log::LogCodePart::Line)
                                        .get_code()
                                    | crate::log::LogCodeD::new()
                                        .encode(self.id as u128, crate::log::LogCodePart::Id)
                                        .get_code()
                            )
                        }
                    }
                    Err(_) => crate::send2logger_dev!(
                        crate::log::code::TYPE_EXE_ERROR
                            | crate::log::code::CONDI_FILE_LOAD2BUFFER_FAIL
                            | crate::log::code::FILE_RESOURCE
                            | crate::log::LogCodeD::new()
                                .encode(line!() as u128, crate::log::LogCodePart::Line)
                                .get_code()
                            | crate::log::LogCodeD::new()
                                .encode(self.id as u128, crate::log::LogCodePart::Id)
                                .get_code()
                    ),
                }
            } else {
                crate::send2logger_dev!(
                    crate::log::code::TYPE_EXE_ERROR
                        | crate::log::code::CONDI_OPTION_NONE
                        | crate::log::code::FILE_RESOURCE
                        | crate::log::LogCodeD::new()
                            .encode(line!() as u128, crate::log::LogCodePart::Line)
                            .get_code()
                        | crate::log::LogCodeD::new()
                            .encode(self.id as u128, crate::log::LogCodePart::Id)
                            .get_code()
                )
            }
            Ok(buf)
        }
        
        pub fn build(fin:File) -> Self {
            Self { fstream: Option::Some(fin), id: 0 }
        }
    }

    impl Default for Defile {
        fn default() -> Self {
        Self { fstream: Option::None, id: 0 }
    }
    }
}
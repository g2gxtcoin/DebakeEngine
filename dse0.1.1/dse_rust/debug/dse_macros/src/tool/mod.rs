use std::string::String;

pub const META_TABLE_POSTFIX:&'static str="innertablemeta";

pub mod primitive_type_str {
    pub const I8: &'static str = "i8";
    pub const I16: &'static str = "i16";
    pub const I32: &'static str = "i32";
    pub const I64: &'static str = "i64";
    pub const I128: &'static str = "i128";

    pub const USIZE: &'static str = "usize";
    pub const U8: &'static str = "u8";
    pub const U16: &'static str = "u16";
    pub const U32: &'static str = "u32";
    pub const U64: &'static str = "u64";
    pub const U128: &'static str = "u128";

    pub const F32: &'static str = "f32";
    pub const F64: &'static str = "f64";

    pub const CHAR: &'static str = "char";
    pub const BOOL: &'static str = "bool";

    pub const STRING: &'static str = "String";
    pub const STR: &'static str = "str";
    pub const POINTER: &'static str = "pointer";
}

pub enum TokenType2InnerMeta {
    None,
    CustomStruct,
    Preimplement,
    Array,
    Vector,
}

//
pub fn assert_token_type(tin: String) -> TokenType2InnerMeta {
     if tin.eq(primitive_type_str::I8)
        || tin.eq(primitive_type_str::I16)
        || tin.eq(primitive_type_str::I32)
        || tin.eq(primitive_type_str::I64)
        || tin.eq(primitive_type_str::I128)
        || tin.eq(primitive_type_str::USIZE)
        || tin.eq(primitive_type_str::U8)
        || tin.eq(primitive_type_str::U16)
        || tin.eq(primitive_type_str::U32)
        || tin.eq(primitive_type_str::U64)
        || tin.eq(primitive_type_str::U128)
        || tin.eq(primitive_type_str::F32)
        || tin.eq(primitive_type_str::F64)
        || tin.eq(primitive_type_str::CHAR)
        || tin.eq(primitive_type_str::BOOL)
        || tin.eq(primitive_type_str::STRING)
        || tin.eq(primitive_type_str::STR)
        || tin.eq(primitive_type_str::POINTER){
            return TokenType2InnerMeta::Preimplement;
        }else if 
        tin.contains("[")
        && tin.contains("]"){
            return TokenType2InnerMeta::Array;
        }else if tin.contains("Vec")&&tin.contains(">"){
            return TokenType2InnerMeta::Vector;
        }
        else {
            return TokenType2InnerMeta::CustomStruct;
        }
}


pub fn get_array_index_by_ident(tin: String)-> Result<(String,usize),String>{
    if tin.contains("[")
    && tin.contains("]"){
        let ty=&tin[1..tin.find(";").unwrap()].to_string();
        let count=&tin[tin.find(";").unwrap()..(tin.len()-1)].to_string();
        let count:usize=count.parse().unwrap();
        return Ok((ty.to_string(),count));
    }
    return Err("get_array_index_by_ident err".to_string());
}


pub fn replace_key_char_into_ident(tin: String)-> String{
    let mut tin =tin;
    tin=tin.replace(" ", "");
    tin = tin.replace(";", "_");
    tin = tin.replace("[", "_A_");
    tin = tin.replace("]", "_A_");
    tin = tin.replace("<", "_T_");
    tin = tin.replace(">", "_T_");
    tin=tin.replace(":", "_B_");

    return tin;
}

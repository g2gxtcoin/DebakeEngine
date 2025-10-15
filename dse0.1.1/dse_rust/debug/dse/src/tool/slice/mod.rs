use std::ptr::null;

pub static _0u32: [u32; 1] = [0];
pub static _1u32: [u32; 1] = [1];
pub static _2u32: [u32; 1] = [2];
pub static _3u32: [u32; 1] = [3];
pub static _4u32: [u32; 1] = [4];
pub static _5u32: [u32; 1] = [5];
pub static _6u32: [u32; 1] = [6];
pub static _7u32: [u32; 1] = [7];
pub static _8u32: [u32; 1] = [8];
pub static _9u32: [u32; 1] = [9];
pub static _01u32: [u32; 2] = [0, 1];
pub static _02u32: [u32; 2] = [0, 2];
pub static _03u32: [u32; 2] = [0, 3];
pub static _04u32: [u32; 2] = [0, 4];
pub static _05u32: [u32; 2] = [0, 5];
pub static _06u32: [u32; 2] = [0, 6];
pub static _07u32: [u32; 2] = [0, 7];
pub static _08u32: [u32; 2] = [0, 8];
pub static _09u32: [u32; 2] = [0, 9];

pub static _12u32: [u32; 2] = [1, 2];
pub static _13u32: [u32; 2] = [1, 3];
pub static _14u32: [u32; 2] = [1, 4];
pub static _15u32: [u32; 2] = [1, 5];
pub static _16u32: [u32; 2] = [1, 6];
pub static _17u32: [u32; 2] = [1, 7];
pub static _18u32: [u32; 2] = [1, 8];
pub static _19u32: [u32; 2] = [1, 9];

pub static _23u32: [u32; 2] = [2, 3];
pub static _24u32: [u32; 2] = [2, 4];
pub static _25u32: [u32; 2] = [2, 5];
pub static _26u32: [u32; 2] = [2, 6];
pub static _27u32: [u32; 2] = [2, 7];
pub static _28u32: [u32; 2] = [2, 8];
pub static _29u32: [u32; 2] = [2, 9];

pub static _34u32: [u32; 2] = [3, 4];
pub static _35u32: [u32; 2] = [3, 5];
pub static _36u32: [u32; 2] = [3, 6];
pub static _37u32: [u32; 2] = [3, 7];
pub static _38u32: [u32; 2] = [3, 8];
pub static _39u32: [u32; 2] = [3, 9];

pub static _45u32: [u32; 2] = [4, 5];
pub static _46u32: [u32; 2] = [4, 6];
pub static _47u32: [u32; 2] = [4, 7];
pub static _48u32: [u32; 2] = [4, 8];
pub static _49u32: [u32; 2] = [4, 9];

pub static _56u32: [u32; 2] = [5, 6];
pub static _57u32: [u32; 2] = [5, 7];
pub static _58u32: [u32; 2] = [5, 8];
pub static _59u32: [u32; 2] = [5, 9];

pub static _67u32: [u32; 2] = [6, 7];
pub static _68u32: [u32; 2] = [6, 8];
pub static _69u32: [u32; 2] = [6, 9];

pub static _78u32: [u32; 2] = [7, 8];
pub static _79u32: [u32; 2] = [7, 9];

pub static _89u32: [u32; 2] = [8, 9];

pub static _012u32: [u32; 3] = [0, 1, 2];
pub static _0123u32: [u32; 4] = [0, 1, 2, 3];
pub static _01234u32: [u32; 5] = [0, 1, 2, 3, 4];
pub static _012345u32: [u32; 6] = [0, 1, 2, 3, 4, 5];
pub static _0123456u32: [u32; 7] = [0, 1, 2, 3, 4, 5, 6];
pub static _01234567u32: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
pub static _012345678u32: [u32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
pub static _0123456789u32: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn to_num_slice(num_in:usize)-> *const u32{
    match num_in {
        1 => &_1u32[0],
        2 => &_01u32[0],
        3 => &_012u32[0],
        4 => &_0123u32[0],
        5 => &_01234u32[0],
        6 => &_012345u32[0],
        7 => &_0123456u32[0],
        8 => &_01234567u32[0],
        9 => &_012345678u32[0],
        _ => null(),
    }
}
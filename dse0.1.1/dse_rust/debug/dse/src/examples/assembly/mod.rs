use std::arch::asm;

#[test]
pub fn hellow_asm() {
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    dbg!(x);
    assert_eq!(x, 4 * 6);
}

#[test]
pub fn int3() {
    unsafe {
        asm!(
            "int3",
        );
    }
}
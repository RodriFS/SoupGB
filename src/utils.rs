use super::registers::Flags;

pub fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

pub fn set_bit_at(input: u8, bit: u8) -> u8 {
    let bits = 1 << bit;
    input | bits
}

pub fn clear_bit_at(input: u8, bit: u8) -> u8 {
    let bits = !(1 << bit);
    input & bits
}

pub fn test_flag_add(a: u8, b: u8, flag: Flags) -> bool {
    match flag {
        Flags::Z => a.wrapping_add(b) == 0,
        Flags::C => (a as u16 & 0xff) + (b as u16 & 0xff) > 0xff,
        Flags::H => (a as u16 & 0x0f) + (b as u16 & 0x0f) > 0x0f,
        _ => panic!("Not supported fn test_flag_add_u8"),
    }
}

pub fn test_flag_add_carry(a: u8, b: u8, carry: u8, flag: Flags) -> bool {
    match flag {
        Flags::Z => a.wrapping_add(b).wrapping_add(carry) == 0,
        Flags::C => (a as u16 & 0xff) + (b as u16 & 0xff) + (carry as u16 & 0xff) > 0xff,
        Flags::H => (a as u16 & 0x0f) + (b as u16 & 0x0f) + (carry as u16 & 0x0f) > 0x0f,
        _ => panic!("Not supported fn test_flag_add_u8"),
    }
}

pub fn test_flag_add_16(a: u16, b: u16, flag: Flags) -> bool {
    match flag {
        Flags::Z => a.wrapping_add(b) == 0,
        Flags::C => (a as u32 & 0xffff) + (b as u32 & 0xffff) > 0xffff,
        Flags::H => (a as u32 & 0x0fff) + (b as u32 & 0x0fff) > 0x0fff,
        _ => panic!("Not supported fn test_flag_add_u16"),
    }
}

pub fn test_flag_sub(a: u8, b: u8, flag: Flags) -> bool {
    match flag {
        Flags::Z => a.wrapping_sub(b) == 0,
        Flags::C => a < b,
        Flags::H => (a & 0x0f) < (b & 0x0f),
        _ => panic!("Not supported fn test_flag_add_u8"),
    }
}

pub fn test_flag_sub_carry(a: u8, b: u8, carry: u8, flag: Flags) -> bool {
    match flag {
        Flags::Z => a.wrapping_sub(b).wrapping_sub(carry) == 0,
        Flags::C => (a as u16) < ((b as u16) + (carry as u16)),
        Flags::H => (a as u16 & 0x0f) < ((b as u16 & 0x0f) + (carry as u16 & 0x0f)),
        _ => panic!("Not supported fn test_flag_add_u8"),
    }
}

pub fn test_flag_sub_16(a: u16, b: u16, flag: Flags) -> bool {
    match flag {
        Flags::Z => a.wrapping_sub(b) == 0,
        Flags::C => a < b,
        Flags::H => (a & 0x0fff) < (b & 0x0fff),
        _ => panic!("Not supported fn test_flag_add_u16"),
    }
}

pub fn swap_nibbles(a: u8) -> u8 {
    (a & 0x0F) << 4 | (a & 0xF0) >> 4
}

#[test]
fn test_get_bit_at() {
    let bit = get_bit_at(0x01, 0);
    assert_eq!(bit, true);
    let bit = get_bit_at(0x01, 1);
    assert_eq!(bit, false);
}

#[test]
fn test_set_bit_at() {
    let bit = set_bit_at(0b0000_0000, 0);
    assert_eq!(bit, 0b0000_0001);
    let bit = set_bit_at(0b0000_0000, 1);
    assert_eq!(bit, 0b0000_0010);
    let bit = set_bit_at(0b0000_0000, 2);
    assert_eq!(bit, 0b0000_0100);
    let bit = set_bit_at(0b0000_0000, 3);
    assert_eq!(bit, 0b0000_1000);
}

#[test]
fn test_clear_bit_at() {
    let bit = clear_bit_at(0b1111_1111, 0);
    assert_eq!(bit, 0b1111_1110);
    let bit = clear_bit_at(0b1111_1111, 1);
    assert_eq!(bit, 0b1111_1101);
    let bit = clear_bit_at(0b1111_1111, 2);
    assert_eq!(bit, 0b1111_1011);
    let bit = clear_bit_at(0b1111_1111, 3);
    assert_eq!(bit, 0b1111_0111);
}

#[test]
fn test_test_flag_u8() {
    let res = test_flag_add(254, 8, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_add(1, 1, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_add(125, 8, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_add(1, 1, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_add(0, 0, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_add(8, 8, Flags::Z);
    assert_eq!(res, false);

    let res = test_flag_sub(1, 8, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_sub(100, 1, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_sub(130, 8, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_sub(120, 1, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_sub(8, 8, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_sub(3, 8, Flags::Z);
    assert_eq!(res, false);
}

#[test]
fn test_test_flag_u16() {
    let res = test_flag_add_16(65534, 8, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_add_16(65500, 1, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_add_16(4094, 8, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_add_16(2000, 1, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_add_16(0, 0, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_add_16(8, 8, Flags::Z);
    assert_eq!(res, false);

    let res = test_flag_sub_16(1, 8, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_sub_16(100, 1, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_sub_16(4099, 100, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_sub_16(4099, 1, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_sub_16(4000, 4000, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_sub_16(3, 8, Flags::Z);
    assert_eq!(res, false);
}

#[test]
fn test_test_flag_carry_u8() {
    let res = test_flag_add_carry(254, 8, 0, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_add_carry(1, 1, 0, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_add_carry(125, 8, 0, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_add_carry(1, 1, 0, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_add_carry(0, 0, 0, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_add_carry(8, 8, 0, Flags::Z);
    assert_eq!(res, false);

    let res = test_flag_sub_carry(1, 8, 0, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_sub_carry(100, 1, 0, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_sub_carry(130, 8, 0, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_sub_carry(120, 1, 0, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_sub_carry(8, 8, 0, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_sub_carry(3, 8, 0, Flags::Z);
    assert_eq!(res, false);

    let res = test_flag_add_carry(254, 8, 1, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_add_carry(1, 1, 1, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_add_carry(125, 8, 1, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_add_carry(1, 1, 1, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_add_carry(0, 0, 1, Flags::Z);
    assert_eq!(res, false);
    let res = test_flag_add_carry(8, 8, 1, Flags::Z);
    assert_eq!(res, false);

    let res = test_flag_sub_carry(1, 8, 1, Flags::C);
    assert_eq!(res, true);
    let res = test_flag_sub_carry(100, 1, 1, Flags::C);
    assert_eq!(res, false);
    let res = test_flag_sub_carry(130, 8, 1, Flags::H);
    assert_eq!(res, true);
    let res = test_flag_sub_carry(120, 1, 1, Flags::H);
    assert_eq!(res, false);
    let res = test_flag_sub_carry(8, 7, 1, Flags::Z);
    assert_eq!(res, true);
    let res = test_flag_sub_carry(3, 8, 1, Flags::Z);
    assert_eq!(res, false);
}

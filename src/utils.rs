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

#[test]
fn gets_correct_bit() {
    let bit = get_bit_at(0x01, 0);
    assert_eq!(bit, true);
    let bit = get_bit_at(0x01, 1);
    assert_eq!(bit, false);
}

#[test]
fn sets_correct_bits() {
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
fn clears_correct_bits() {
    let bit = clear_bit_at(0b1111_1111, 0);
    assert_eq!(bit, 0b1111_1110);
    let bit = clear_bit_at(0b1111_1111, 1);
    assert_eq!(bit, 0b1111_1101);
    let bit = clear_bit_at(0b1111_1111, 2);
    assert_eq!(bit, 0b1111_1011);
    let bit = clear_bit_at(0b1111_1111, 3);
    assert_eq!(bit, 0b1111_0111);
}

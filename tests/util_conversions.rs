use std::net::Ipv4Addr;

use winsysutil::util::*;

#[test]
fn test_convert_1() {
    let n: u32 = 4059231220;
    let m: [u8; 4] = [0xf4, 0xf3, 0xf2, 0xf1];

    assert_eq!(n, convert_4u8_be_to_1u32_le(m));
}

#[test]
fn test_convert_2() {
    let n: u32 = 4059231220;
    let m: [u8; 4] = [0xf4, 0xf3, 0xf2, 0xf1];

    assert_eq!(convert_1u32_le_to_4u8_be(n), m);
}

#[test]
fn test_convert_3() {
    let n: u16 = 61938;
    let m: [u8; 2] = [0xf2, 0xf1];

    assert_eq!(convert_1u16_le_to_2u8_be(n), m);
}

#[test]
fn test_convert_4() {
    let n: u16 = 61938;
    let m: [u8; 2] = [0xf2, 0xf1];

    assert_eq!(convert_1u16_le_to_2u8_be(n), m);
}

#[test]
fn test_get_mask_bits() {
    let mask1 = Ipv4Addr::new(255, 255, 255, 0);
    let mask2 = Ipv4Addr::new(255, 255, 0, 0);

    let result1 = get_bits_from_mask(mask1);
    let result2 = get_bits_from_mask(mask2);

    assert_eq!(result1, 24);
    assert_eq!(result2, 16);
}
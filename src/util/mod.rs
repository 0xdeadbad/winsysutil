pub mod win;

use std::net::Ipv4Addr;

#[inline]
pub fn convert_4u8_be_to_1u32_le(src: [u8; 4]) -> u32 {
    ((src[3] as u32) << 24) | ((src[2] as u32) << 16) | ((src[1] as u32) << 8) | src[0] as u32
}

#[inline]
pub fn convert_1u32_le_to_4u8_be(src: u32) -> [u8; 4] {
    [
        (src & 0x000000ff) as u8,
        ((src & 0x0000ff00) >> 8) as u8,
        ((src & 0x00ff0000) >> 16) as u8,
        ((src & 0xff000000) >> 24) as u8,
    ]
}

#[inline]
pub fn convert_2u8_be_to_1u16_le(src: [u8; 2]) -> u16 {
    ((src[1] as u16) << 8) | src[0] as u16
}

#[inline]
pub fn convert_1u16_le_to_2u8_be(src: u16) -> [u8; 2] {
    [(src & 0x00ff) as u8, ((src & 0xff00) >> 8) as u8]
}

#[inline]
pub fn get_bits_from_mask(mask: Ipv4Addr) -> u32 {
    let mask: u32 = mask.into();

    (0..32).fold(0, |acc, i| acc + ((mask >> i) & 1))
}

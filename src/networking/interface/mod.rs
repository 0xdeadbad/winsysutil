mod interface;

use std::fmt::Display;
use std::net::Ipv4Addr;
use crate::util::get_bits_from_mask;

#[derive(Debug)]
pub struct InterfaceAddress {
    pub address: Ipv4Addr,
    pub mask: Ipv4Addr,
}

impl Display for InterfaceAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.address, get_bits_from_mask(self.mask))
    }
}

#[derive(Debug)]
pub struct Interface {
    pub addresses: Option<Vec<InterfaceAddress>>,
    pub index: u32,
}

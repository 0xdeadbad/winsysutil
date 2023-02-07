use crate::util::{convert_4u8_be_to_1u32_le, win::HeapCalloc};
use core::slice;
use std::net::Ipv4Addr;
use std::{mem, ptr};
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper::{AddIPAddress, GetIpAddrTable, MIB_IPADDRTABLE};

#[derive(Debug)]
pub struct InterfaceAddress {
    pub address: Ipv4Addr,
    pub mask: Ipv4Addr,
}

#[derive(Debug)]
pub struct Interface {
    pub addresses: Option<Vec<InterfaceAddress>>,
    pub index: u32,
}

impl Interface {
    pub fn get_addresses(
        index: u32,
    ) -> std::result::Result<Vec<InterfaceAddress>, Box<dyn std::error::Error>> {
        let mut memalloc = HeapCalloc::alloc(mem::size_of::<MIB_IPADDRTABLE>())?;
        let mut ip_addr_table = memalloc.get_mem()? as *mut MIB_IPADDRTABLE; 
        let mut dw_size: u32 = 0;

        if unsafe { GetIpAddrTable::<bool>(Some(ip_addr_table), &mut dw_size as *mut _, false) }
            == ERROR_INSUFFICIENT_BUFFER.0
        {
            //free(ip_addr_table as *mut c_void)?;
            memalloc.free()?;
            memalloc.alloc_again(dw_size as usize)?;
            //ip_addr_table = heapalloc(dw_size as usize)? as *mut MIB_IPADDRTABLE;
            ip_addr_table = memalloc.get_mem()? as *mut MIB_IPADDRTABLE;
        }
        if ip_addr_table == ptr::null_mut() {
            return Err("Memory allocation failed for GetIpAddrTable".into());
        }

        let dw_retval =
            unsafe { GetIpAddrTable::<bool>(Some(ip_addr_table), &mut dw_size as *mut _, false) };
        if dw_retval != NO_ERROR.0 {
            return Err(format!("GetIpAddrTable failed with error {}", dw_retval).into());
        }

        let tables = unsafe {
            slice::from_raw_parts(
                (*ip_addr_table).table.as_ptr(),
                (*ip_addr_table).dwNumEntries as usize,
            )
        };

        let mut addresses: Vec<InterfaceAddress> = Vec::new();

        for table in tables {
            if table.dwIndex == index {
                addresses.push(InterfaceAddress {
                    address: Ipv4Addr::from(table.dwAddr.to_be()),
                    mask: Ipv4Addr::from(table.dwMask.to_be()),
                })
            }
        }

        Ok(addresses)
    }

    pub fn add_ip_address(
        index: u32,
        ip_address: Ipv4Addr,
        subnet_mask: Ipv4Addr,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut nte_context: u32 = 0;
        let mut nte_instance: u32 = 0;

        let ip_address: u32 = convert_4u8_be_to_1u32_le(ip_address.octets());
        let subnet_mask: u32 = convert_4u8_be_to_1u32_le(subnet_mask.octets());

        let dw_retval = unsafe {
            AddIPAddress(
                ip_address,
                subnet_mask,
                index,
                &mut nte_context as *mut _,
                &mut nte_instance as *mut _,
            )
        };
        if dw_retval != NO_ERROR.0 {
            return Err(format!("AddIPAddress failed with error {}", dw_retval).into());
        }

        Ok(())
    }
}

use crate::util::{convert_4u8_be_to_1u32_le, win::HeapCalloc};
use core::slice;
use std::mem;
use std::net::Ipv4Addr;
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper::{
    CreateIpForwardEntry, GetBestRoute, GetIpForwardTable, MIB_IPFORWARDROW, MIB_IPFORWARDTABLE,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoutingError {
    #[error("invalid value for ForwardType parameter")]
    InvalidForwardTypeValue(u32),
    #[error("invalid value for ForwardProto parameter")]
    InvalidForwardProtoValue(u32)
}

#[repr(u32)]
#[derive(Debug)]
pub enum ForwardType {
    Other = 1,
    Invalid,
    Direct,
    Indirect
}

/*impl From<u32> for ForwardType {
    fn from(n: u32) -> Self {
        match n {
            1 => ForwardType::Other,
            2 => ForwardType::Invalid,
            3 => ForwardType::Direct,
            4 => ForwardType::Indirect,
            _ => ForwardType::NOT_VALID("Invalid ForwardType {}\nOptions are: ForwardType::Other, ForwardType::Invalid, ForwardType::Direct, ForwardType::Indirect".into()),
        }
    }
}*/

impl TryFrom<u32> for ForwardType {
    type Error = RoutingError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(ForwardType::Other),
            2 => Ok(ForwardType::Invalid),
            3 => Ok(ForwardType::Direct),
            4 => Ok(ForwardType::Indirect),
            _ => Err(RoutingError::InvalidForwardTypeValue(value)),
        }
    }
}

impl Into<u32> for ForwardType {
    fn into(self) -> u32 {
        self as u32
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum ForwardProto {
    Other = 1,
    Local,
    Mgmt,
    Icmp,
    Egp,
    Ggp,
    Hello,
    Rip,
    IsIs,
    EsIs,
    Cisco,
    Bbn,
    Ospf,
    Bgp,
    NtAutoStatic = 10002,
    NtStatic = 10006,
    NtStatisNonDod = 10007,
}

impl TryFrom<u32> for ForwardProto {
    type Error = RoutingError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(ForwardProto::Other),
            2 => Ok(ForwardProto::Local),
            3 => Ok(ForwardProto::Mgmt),
            4 => Ok(ForwardProto::Icmp),
            5 => Ok(ForwardProto::Egp),
            6 => Ok(ForwardProto::Ggp),
            7 => Ok(ForwardProto::Hello),
            8 => Ok(ForwardProto::Rip),
            9 => Ok(ForwardProto::IsIs),
            10 => Ok(ForwardProto::EsIs),
            11 => Ok(ForwardProto::Cisco),
            12 => Ok(ForwardProto::Bbn),
            13 => Ok(ForwardProto::Ospf),
            14 => Ok(ForwardProto::Bgp),
            10002 => Ok(ForwardProto::NtAutoStatic),
            10006 => Ok(ForwardProto::NtStatic),
            10007 => Ok(ForwardProto::NtStatisNonDod),
            _ => Err(RoutingError::InvalidForwardProtoValue(value)),
        }
    }
}

impl Into<u32> for ForwardProto {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Debug)]
pub struct ForwardingRow {
    pub destiny: Ipv4Addr,
    pub mask: Ipv4Addr,
    pub next_hop: Ipv4Addr,
    pub metric: u32,
    pub if_index: u32,
    pub forward_type: ForwardType,
    pub forward_proto: ForwardProto,
}

impl ForwardingRow {
    pub fn new(
        destiny: Ipv4Addr,
        mask: Ipv4Addr,
        next_hop: Ipv4Addr,
        metric: u32,
        if_index: u32,
        forward_type: ForwardType,
        forward_proto: ForwardProto,
    ) -> Self {
        Self {
            destiny,
            mask,
            next_hop,
            metric,
            if_index,
            forward_type,
            forward_proto,
        }
    }
}

/*impl From<MIB_IPFORWARDROW> for ForwardingRow {
    fn from(table: MIB_IPFORWARDROW) -> Self {
        Self {
            destiny: Ipv4Addr::from(table.dwForwardDest.to_be()),
            mask: Ipv4Addr::from(table.dwForwardMask.to_be()),
            next_hop: Ipv4Addr::from(table.dwForwardNextHop.to_be()),
            metric: table.dwForwardMetric1,
            if_index: table.dwForwardIfIndex,
            forward_type: unsafe { ForwardType::try_from(table.Anonymous1.dwForwardType).unwrap() },
            forward_proto: unsafe { ForwardType::try_from(table.Anonymous2.dwForwardProto) },
        }
    }
}*/

impl TryFrom<MIB_IPFORWARDROW> for ForwardingRow {
    type Error = RoutingError;

    fn try_from(table: MIB_IPFORWARDROW) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            destiny: Ipv4Addr::from(table.dwForwardDest.to_be()),
            mask: Ipv4Addr::from(table.dwForwardMask.to_be()),
            next_hop: Ipv4Addr::from(table.dwForwardNextHop.to_be()),
            metric: table.dwForwardMetric1,
            if_index: table.dwForwardIfIndex,
            forward_type: unsafe { ForwardType::try_from(table.Anonymous1.dwForwardType)? },
            forward_proto: unsafe { ForwardProto::try_from(table.Anonymous2.dwForwardProto)? },
        })
    }
}

pub struct ForwardingTable;

impl ForwardingTable {
    pub fn add_route(
        forwarding_row: ForwardingRow,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut raw_row = MIB_IPFORWARDROW::default();
        raw_row.dwForwardDest = convert_4u8_be_to_1u32_le(forwarding_row.destiny.octets());
        raw_row.dwForwardMask = convert_4u8_be_to_1u32_le(forwarding_row.mask.octets());
        raw_row.dwForwardNextHop = convert_4u8_be_to_1u32_le(forwarding_row.next_hop.octets());
        raw_row.dwForwardMetric1 = forwarding_row.metric;
        raw_row.dwForwardIfIndex = forwarding_row.if_index;
        raw_row.Anonymous1.dwForwardType = forwarding_row.forward_type.into();
        raw_row.Anonymous2.dwForwardProto = forwarding_row.forward_proto.into();

        let dw_status = unsafe { CreateIpForwardEntry(&mut raw_row as *mut _ as *const _) };
        if dw_status != NO_ERROR.0 {
            return Err(format!("Error on CreateIpForwardEntry(): {}", dw_status).into());
        }

        Ok(())
    }

    pub fn get_routes() -> std::result::Result<Vec<ForwardingRow>, Box<dyn std::error::Error>> {
        let mut memalloc = HeapCalloc::alloc(mem::size_of::<MIB_IPFORWARDTABLE>())?;
        let mut ip_forward_table = memalloc.get_mem()?;
        // let mut ip_forward_table = heapalloc(mem::size_of::<MIB_IPFORWARDTABLE>())?;
        let mut dw_size: u32 = 0;
        let mut dw_status = unsafe {
            GetIpForwardTable(
                Some(ip_forward_table as *mut _),
                &mut dw_size as *mut _,
                false,
            )
        };
        if dw_status == ERROR_INSUFFICIENT_BUFFER.0 {
            //free(ip_forward_table)?;
            memalloc.free()?;
            memalloc.alloc_again(dw_size as usize)?;
            ip_forward_table = memalloc.get_mem()?;
            dw_status = unsafe {
                GetIpForwardTable(
                    Some(ip_forward_table as *mut _),
                    &mut dw_size as *mut _,
                    false,
                )
            };
        }
        if dw_status != ERROR_SUCCESS.0 {
            if !ip_forward_table.is_null() {
                // free(ip_forward_table)?;
                memalloc.free()?;
            }
            return Err("GetIpForwardTable failed".into());
        }

        let tables = unsafe {
            slice::from_raw_parts_mut(
                (*(ip_forward_table as *mut MIB_IPFORWARDTABLE))
                    .table
                    .as_mut_ptr(),
                (*(ip_forward_table as *mut MIB_IPFORWARDTABLE)).dwNumEntries as usize,
            )
        };

        let mut result: Vec<ForwardingRow> = Vec::new();
        for table in tables {
            result.push(ForwardingRow::try_from(*table)?);
        }

        Ok(result)
    }

    pub fn get_best_route(
        destiny: Ipv4Addr,
    ) -> std::result::Result<ForwardingRow, Box<dyn std::error::Error>> {
        let mut rout = MIB_IPFORWARDROW::default();
        let result = unsafe {
            GetBestRoute(
                convert_4u8_be_to_1u32_le(destiny.octets()),
                0,
                &mut rout as *mut _,
            )
        };

        if result != NO_ERROR.0 {
            return Err(format!("Error on GetBestRoute(): {}", result).into());
        }

        Ok(ForwardingRow::try_from(rout)?)
    }
}

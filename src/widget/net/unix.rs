use std::{collections::HashMap, os::unix::io::RawFd};

use nix::{
    ifaddrs::getifaddrs,
    net::if_::InterfaceFlags,
    sys::socket::{socket, AddressFamily, IpAddr, SockAddr, SockFlag, SockProtocol, SockType},
};

#[cfg(target_os = "linux")]
use super::linux::is_wireless_if;
use super::{IfInfo, IfType, InterfaceBlacklist};

#[derive(Copy, Clone, Debug)]
pub struct InetStreamSock(RawFd);

impl InetStreamSock {
    #[inline]
    pub fn new() -> Result<Self, nix::Error> {
        socket(
            AddressFamily::Inet,
            SockType::Stream,
            SockFlag::empty(),
            SockProtocol::Tcp,
        )
        .map(InetStreamSock)
    }

    #[inline]
    pub fn fd(&self) -> RawFd {
        self.0
    }
}

pub fn update_ifs(
    cache: &mut HashMap<String, IfInfo>,
    blacklist: &InterfaceBlacklist,
    sock: &InetStreamSock,
) {
    let addrs = getifaddrs().unwrap();

    for addr in addrs.filter(|addr| !blacklist.contains(&addr.interface_name)) {
        if let Some(ent) = cache.get_mut(&addr.interface_name) {
            if let Some(SockAddr::Inet(inet_addr)) = addr.address {
                ent.is_running = addr.flags.contains(InterfaceFlags::IFF_RUNNING);
                match inet_addr.ip() {
                    IpAddr::V4(ipv4) => {
                        ent.ipv4 = Some(ipv4);
                    }
                    IpAddr::V6(ipv6) => {
                        ent.ipv6 = Some(ipv6);
                    }
                }
            }
        } else {
            let type_ = if is_wireless_if(sock, &addr.interface_name) {
                IfType::Wireless
            } else {
                IfType::Ethernet
            };

            let ent = IfInfo {
                is_running: addr.flags.contains(InterfaceFlags::IFF_RUNNING),
                type_,
                ipv4: None,
                ipv6: None,
            };

            cache.insert(
                addr.interface_name,
                match addr.address {
                    Some(SockAddr::Inet(inet_addr)) => match inet_addr.ip() {
                        IpAddr::V4(ipv4) => IfInfo {
                            ipv4: Some(ipv4),
                            ..ent
                        },
                        IpAddr::V6(ipv6) => IfInfo {
                            ipv6: Some(ipv6),
                            ..ent
                        },
                    },
                    _ => ent,
                },
            );
        }
    }
}

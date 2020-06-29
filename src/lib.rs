#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

extern crate serde;
extern crate serde_json;

use std::net::IpAddr;
use std::str;
use std::str::FromStr;

/// 一些 helper 方法
mod helper;

use helper::IPDB;

/// 根据 IP 查找地址,
/// 例如 find("58.250.137.36", "CN")
#[inline]
pub fn find(addr: &str, lan: &str) -> Result<Vec<String>, &'static str> {
    if !IPDB.meta.languages.contains_key(lan) {
        return Err("not support language!");
    }
    //let off: u32 = *IPDB.meta.languages.get(lan).unwrap();
    let mut ipv: Vec<u8> = Vec::with_capacity(32);

    if let Ok(ip) = IpAddr::from_str(addr) {
        match ip {
            IpAddr::V6(v6) => {
                if (IPDB.meta.ip_version & 0x02) != 0x02 {
                    return Err("no support ipv6 !");
                }
                ipv.extend(v6.octets().iter());
            }
            IpAddr::V4(v4) => {
                if (IPDB.meta.ip_version & 0x01) != 0x01 {
                    return Err("no support ipv4 !");
                }
                ipv.extend(v4.octets().iter());
            }
        }
    } else {
        return Err("ip addr error!");
    }

    let node = helper::find_node(&ipv);
    if node == 0 {
        return Err("ip not found");
    }
    match helper::resolve(node) {
        Ok(data) => return Ok(data
            .splitn(IPDB.meta.fields.len() * IPDB.meta.languages.len(), '\t')
            .map(|s| s.to_string())
            .collect()),
        Err(err) => Err(err),
    }
}

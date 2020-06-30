#![warn(clippy::all, clippy::restriction, clippy::pedantic, clippy::nursery, clippy::cargo)]

extern crate serde;
extern crate serde_json;

use std::net::IpAddr;
use std::str;
use std::str::FromStr;

/// 一些 helper 方法
mod helper;

use helper::IPDB;

/// 根据 IP 查找地址, 返回的是 `[国,省,市]`, 参考 ipdb 官网 https://github.com/ipipdotnet
/// 例如
/// ```rust
///     use ipdb_rs::find;
///     if let Ok(addr) = find("58.250.137.36", "CN") {
///         println!("addr {:?}", addr); // ["中国", "广东", "深圳"]
///     };
/// ```
/// # Errors
/// 如果有任何异常, 只返回描述
#[inline]
pub fn find(addr: &str, lan: &str) -> Result<Vec<&'static str>, &'static str> {
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
        Ok(data) => Ok(data
            .splitn(IPDB.meta.fields.len() * IPDB.meta.languages.len(), '\t')
            .collect::<Vec<&str>>()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_round() {
        //warmup
        for _i in 0..10000 {
            find("58.250.137.36", "CN");
        }
        if let Ok(addr) = find("58.250.137.36", "CN") {
            println!("addr {:?}", addr);
        }
        let now = Instant::now();
        let mut s = 0;
        for _i in 0..1000000 {
            if let Ok(_v) = find("58.250.137.36", "CN") {
                s += 1;
            }
        }
        println!("ok {}, err {}, cost {} ms", s, 1000000 - s, now.elapsed().as_millis());
    }
}

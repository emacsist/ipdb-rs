/// 一些 helper 方法及初始化
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io::Read;

extern crate serde;
extern crate serde_json;

/// ipdb 的文件名
const IPDB_FILE_NAME: &str = "ipipfree.ipdb";

/// IPDB 的元数据结构
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    /// 文件构建的 timestamp
    pub build: usize,
    /// IP 版本
    pub ip_version: u8,
    /// 节点数
    pub node_count: usize,
    /// 语言
    pub languages: HashMap<String, u32>,
    /// 字段
    pub fields: Vec<String>,
    /// 总大小
    pub total_size: usize,
}

/// IPDB 的整体对象
pub struct IpdbObject {
    /// 文件大小
    pub file_size: usize,
    /// 节点数
    pub node_count: usize,
    /// 二进制数据
    pub data: Vec<u8>,
    /// IPv4 的偏移
    pub v4offset: usize,
    /// Meta 对象
    pub meta: MetaData,
}

impl IpdbObject {
    /// 设置 byte 数据, 不包括 meta
    pub fn set_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    /// 设置 meta 对象
    pub fn set_meta(&mut self, meta: MetaData) {
        self.meta = meta
    }
}

// 初始化
lazy_static! {
    /// IPDB 全局静态对象, 只初始化一次
    pub static ref IPDB: IpdbObject = {
        let mut ipdb = IpdbObject {
            file_size: 0,
            node_count: 0,
            data: Vec::new(),
            v4offset: 0,
            meta: MetaData {
                build: 0,
                ip_version: 0,
                node_count: 0,
                languages: HashMap::new(),
                fields: Vec::new(),
                total_size: 0,
            },
        };
        init_ipdb(&mut ipdb);
        ipdb
    };
}

/// 初始化 ipdb
fn init_ipdb(ipdb: &mut IpdbObject) {
    let mut file_bytes = fs::read(IPDB_FILE_NAME).expect("failed to open ipdb file!");
    ipdb.file_size = file_bytes.len();
    //ipdb.set_data(dat);

    let meta_len: usize =
        u32::from_be_bytes([file_bytes[0], file_bytes[1], file_bytes[2], file_bytes[3]])
            .try_into()
            .expect("unexpectd u32 to usize in meta len");

    let to_index = meta_len + 4;
    let meta_bytes = &file_bytes[4..to_index];
    let meta_str = unsafe { std::str::from_utf8_unchecked(meta_bytes) };
    let meta_result: Result<MetaData, Error> = serde_json::from_str(meta_str);
    if let Ok(meta) = meta_result {
        ipdb.node_count = meta.node_count;
        ipdb.set_meta(meta);
        println!(
            "meta => {}",
            serde_json::to_string_pretty(&ipdb.meta).unwrap()
        );
    } else {
        panic!("parse meta error !");
    }

    ipdb.set_data(&file_bytes[to_index..]);
    println!(
        "file len {}, meta len {}, data len {}. is equals = {}",
        ipdb.file_size,
        meta_bytes.len() + 4,
        ipdb.data.len(),
        ipdb.file_size == (meta_bytes.len() + 4 + ipdb.data.len())
    );

    if 0x01 == (ipdb.meta.ip_version & 0x01) {
        let mut node = 0;
        let mut i: u32 = 0;
        while i < 96 && node < ipdb.node_count {
            if i >= 80 {
                node = read_node(ipdb, node, 1);
            } else {
                node = read_node(ipdb, node, 0);
            }
            i += 1;
        }
        ipdb.v4offset = node;
    }
}

fn read_node(ipdb: &IpdbObject, node: usize, index: usize) -> usize {
    let off = node * 8 + index * 4;
    u32::from_be_bytes([
        ipdb.data[off],
        ipdb.data[off + 1],
        ipdb.data[off + 2],
        ipdb.data[off + 3],
    ])
    .try_into()
    .expect("Unexpected u32 to usize in read_node")
}

pub fn find_node(binary: &[u8]) -> usize {
    let mut node: usize = 0;
    let bit: u8 = (binary.len() * 8).try_into().expect("convert to u8 error");
    if bit == 32 {
        node = IPDB.v4offset;
    }
    for i in 0..bit {
        if node > IPDB.node_count {
            break;
        }
        let ii: usize = i.try_into().expect("convert to usize error");
        let index: usize = (1 & ((binary[ii / 8]) >> (7 - (i % 8))))
            .try_into()
            .expect("Unexpect to usize in find_node");
        node = read_node(&IPDB, node, index);
    }
    if node > IPDB.node_count {
        return node;
    }
    0
}

pub fn resolve(node: usize) -> Result<&'static str, &'static str> {
    let resoloved: usize = (node - IPDB.node_count + IPDB.node_count * 8)
        .try_into()
        .expect("Unexpected usize in resolve ");
    if resoloved > IPDB.file_size {
        return Err("database resolve error");
    }
    let size: usize = u32::from_be_bytes([0, 0, IPDB.data[resoloved], IPDB.data[resoloved + 1]])
        .try_into()
        .expect("Unexpected usize in resolve");
    if IPDB.data.len() < (resoloved + 2 + size) {
        return Err("database resolve error");
    }

    let end = resoloved + 2 + size;
    let data = unsafe { std::str::from_utf8_unchecked(&IPDB.data[resoloved + 2..end]) };
    Ok(data)
    //    Err("database resolve error. decode error!")
}

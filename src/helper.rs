use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process::exit;

extern crate serde;
extern crate serde_json;

const IPDB_FILE_NAME: &'static str = "ipipfree.ipdb";

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaData {
    pub build: usize,
    pub ip_version: u8,
    pub node_count: usize,
    pub languages: HashMap<String, u32>,
    pub fields: Vec<String>,
    pub total_size: usize,
}

pub struct IpdbObject {
    pub file_size: usize,
    pub node_count: usize,
    pub data: Vec<u8>,
    pub v4offset: usize,
    pub meta: MetaData,
}

impl IpdbObject {
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data.extend_from_slice(&data);
    }

    pub fn set_meta(&mut self, meta: MetaData) {
        self.meta = meta
    }
}

// 初始化
lazy_static! {
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

fn init_ipdb(ipdb: &mut IpdbObject) {
    let mut file = File::open(IPDB_FILE_NAME).unwrap();
    let mut file_bytes: Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    if let Err(err) = file.read_to_end(&mut file_bytes) {
        println!("{:?}", err);
        exit(-1);
    }

    ipdb.file_size = file_bytes.len();
    //ipdb.set_data(dat);

    let meta_len = u32::from_be_bytes([file_bytes[0], file_bytes[1], file_bytes[2], file_bytes[3]]);

    let to_index = (meta_len + 4) as usize;
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

    ipdb.set_data(file_bytes[to_index..].to_vec());
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
    ]) as usize
}

pub fn find_node(binary: &Vec<u8>) -> usize {
    let mut node: usize = 0;
    let bit = binary.len() * 8;
    if bit == 32 {
        node = IPDB.v4offset;
    }
    for i in 0..bit {
        if node > IPDB.node_count {
            break;
        }
        let index = (1 & ((binary[i / 8]) >> (7 - (i % 8)) as u8)) as usize;
        node = read_node(&IPDB, node, index);
    }
    if node > IPDB.node_count {
        return node;
    }
    return 0;
}

pub fn resolve(node: usize) -> Result<&'static str, &'static str> {
    let resoloved = (node - IPDB.node_count + IPDB.node_count * 8) as usize;
    if resoloved > IPDB.file_size as usize {
        return Err("database resolve error");
    }
    let size = u32::from_be_bytes([0, 0, IPDB.data[resoloved], IPDB.data[resoloved + 1]]) as usize;
    if IPDB.data.len() < (resoloved + 2 + size) {
        return Err("database resolve error");
    }

    let end = resoloved + 2 + size;
    let data = unsafe { std::str::from_utf8_unchecked(&IPDB.data[resoloved + 2..end]) };
    return Ok(data);
    //    Err("database resolve error. decode error!")
}

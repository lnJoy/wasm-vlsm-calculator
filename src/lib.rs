use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
pub struct Subnet {
    name: String,
    needed_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
pub struct VLSM {
    name: String,
    needed_size: u32,
    allocated_size: u32,
    network_address: String,
    prefix: u8,
    subnet_mask: String,
    range: String,
    broadcast: String
}

pub fn convert_prefix_to_mask(prefix: u8) -> Vec<u8> {
    let mask: u32 = (0xFFFFFFFF << (32 - prefix)) & 0xFFFFFFFF;
    return vec![(mask >> 24)            as u8,
                ((mask >> 16) & 0xFF)   as u8,
                ((mask >> 8) & 0xFF)    as u8,
                (mask & 0xFF)           as u8];
}

pub fn convert_cidr_to_string(cidr: Vec<u8>) -> String {
    format!("{}.{}.{}.{}", cidr[0], cidr[1], cidr[2], cidr[3])
}

pub fn convert_cidr_to_binary(cidr: Vec<u8>) -> u64 {
    let _cidr = cidr.iter().map(|octet| *octet as u64).collect::<Vec<u64>>();

    let mut binary: u64 = _cidr[0];
    binary = (binary << 8) + _cidr[1];
    binary = (binary << 8) + _cidr[2];
    binary = (binary << 8) + _cidr[3];

    return binary;
}

pub fn convert_binary_to_cidr(binary: u64) -> Vec<u8> {
    let mut network_id: Vec<u8> = Vec::new();

    network_id.push((binary >> 24 & 0xFF).try_into().unwrap());
    network_id.push((binary >> 16 & 0xFF).try_into().unwrap());
    network_id.push((binary >> 8 & 0xFF).try_into().unwrap());
    network_id.push((binary & 0xFF).try_into().unwrap());

    return network_id;
}

pub fn get_network_id(binary_ip: u64, prefix: u8) -> u64 {
    let offset = 32 - prefix;
    (binary_ip >> offset) << offset
}

// https://recordsoflife.tistory.com/694
pub fn get_allocated_size_and_prefix(current_prefix: u8, needed_size: u32) -> (u8, u32) {
    let power: u8 = ((needed_size + 2u32) as f32 + 1.).log2().ceil() as u8;
    
    let prefix: u8 = ((32u8 - current_prefix) - power) + current_prefix;
    let allocated_size: u32 = 2u32.pow(power as u32);
    
    return (prefix, allocated_size);
}

pub fn network_parser(network: String) -> (Vec<u8>, u8) {
    let ipv4_and_mask: Vec<&str> = network.split("/").collect();
    let major_ipv4: Vec<u8> = ipv4_and_mask[0].split(".")
        .map(|octet| octet.parse().unwrap())
        .collect();

    let current_prefix: u8 = ipv4_and_mask[1].parse().unwrap();

    return (major_ipv4, current_prefix);
}

pub fn sort(subnets: &mut Vec<Subnet>) {
    subnets.sort_by(|a, b| b.needed_size.cmp(&a.needed_size));
}

#[wasm_bindgen]
pub fn vlsm_calculate(network_id: &JsValue, value: &JsValue) -> JsValue {
    let ipv4: String = network_id.into_serde().unwrap();
    let subnets: Vec<Subnet> = value.into_serde().unwrap();
    let result = vlsm(ipv4, subnets);
    
    JsValue::from_serde(&result).unwrap()
}

pub fn vlsm(ipv4: String, mut input_subnets: Vec<Subnet>) -> Vec<VLSM> {
    let (major_ipv4, current_prefix) = network_parser(ipv4);
    let mut current_binary_network_id = get_network_id(convert_cidr_to_binary(major_ipv4), current_prefix);

    sort(&mut input_subnets);

    let mut vlsm_subnets: Vec<VLSM> = Vec::new();
    for subnet in input_subnets.iter() {
        let name = subnet.name.clone();
        let (prefix, allocated_size) = get_allocated_size_and_prefix(current_prefix, subnet.needed_size);
        let range = format!("{} - {}", convert_cidr_to_string(convert_binary_to_cidr(current_binary_network_id + 1)), convert_cidr_to_string(convert_binary_to_cidr(current_binary_network_id + (allocated_size as u64) - 2)));
        vlsm_subnets.push(VLSM {
            name: name,
            needed_size: subnet.needed_size,
            allocated_size: allocated_size,
            network_address: convert_cidr_to_string(convert_binary_to_cidr(current_binary_network_id)),
            prefix: prefix,
            subnet_mask: convert_cidr_to_string(convert_prefix_to_mask(prefix)),
            range: range,
            broadcast: convert_cidr_to_string(convert_binary_to_cidr(current_binary_network_id + (allocated_size as u64) - 1u64)),
        });

        current_binary_network_id += allocated_size as u64;
    }

    return vlsm_subnets;
}
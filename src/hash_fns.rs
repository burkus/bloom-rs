use sha2::{Digest, Sha224, Sha512};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn default_hash(value: &String) -> u64 {
    let mut hasher: DefaultHasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

pub fn string_fold_hash(s: &String) -> u64 {
    let mut sum: u64 = 0;
    let mut mul: u64 = 1;
    for (index, byte) in s.bytes().enumerate() {
        mul = if index % 4 == 0 { 1 } else { mul * 256 };
        sum += byte as u64 * mul;
    }
    sum
}

pub fn string_sum_hash(s: &String) -> u64 {
    let out = s
        .bytes()
        .map(|byte| byte as u64)
        .reduce(|accum, cur| accum + cur);

    if out.is_some() {
        out.unwrap()
    } else {
        0
    }
}

fn parse_8_bytes(hex_string: String) -> u64 {
    u64::from_str_radix(&&hex_string[..16], 16).expect("Error Parsing Hex")
}

pub fn sha224(s: &String) -> u64 {
    let mut hasher = Sha224::new();
    hasher.update(s.as_bytes());
    let hex = format!("{:x}", hasher.finalize());
    parse_8_bytes(hex)
}

pub fn sha512(s: &String) -> u64 {
    let mut hasher = Sha512::new();
    hasher.update(s.as_bytes());
    let hex = format!("{:x}", hasher.finalize());
    parse_8_bytes(hex)
}

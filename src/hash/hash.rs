use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn transform_u32_to_array_of_u8(x: u32) -> [u8; 4] {
    let b1: u8 = ((x >> 24) & 0xff) as u8;
    let b2: u8 = ((x >> 16) & 0xff) as u8;
    let b3: u8 = ((x >> 8) & 0xff) as u8;
    let b4: u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4];
}

pub fn hash_int(num: u32) -> u64 {
    let mut s = DefaultHasher::new();
    num.hash(&mut s);
    s.finish()
}

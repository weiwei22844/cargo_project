extern crate byteorder;
extern crate rand;
use byteorder::{BigEndian, ByteOrder};
use rand::Rng;
//use rand::{self, Rng};

fn main() {
    let header_pre = "0001000000000001d99e000000005cad7ffc0000248c9d205389577ea31d56f22e220a05bed69180f616fa8059b8b5be4a869f9b2163382c3f6f4b1234c59a5b7c118222d3d655fe0a474606905cf76c4e9c0e2d2ec9af100983048fbe6f91e931c61081600f52fd545e0ab246bc07353570f27a9b48633c4434a636c8634377129437196def04549ae4e57ef170fedd41ec6c0c56d1dfd85de5f34135ee5791028c4ad2de8feff12b1ffdc0d4f1db68140d0b9df4e10396492a4d38741c604355fe3842b09b98dfab3bfd7883796e7d61b00000000000196fee00000000000be96800014ba8f91f75fb000004f7";
    let header_post = "";
    
	let header = get_next_header_data(&header_pre, &header_post);
    println!("header 0: {}", header.0);
    println!("header 1 len {}: {:?}", header.1.len(), header.1);
    println!("header 2: {}", header.2);
    
    println!("Hello, world!");
}

pub fn header_data(pre_nonce: &str, post_nonce: &str, nonce: u64) -> (Vec<u8>, u32) {
	// Turn input strings into vectors
	let mut pre_vec = from_hex_string(pre_nonce);
	let mut post_vec = from_hex_string(post_nonce);

	let sec_scaling_bytes = &pre_vec.clone()[pre_vec.len()-4..pre_vec.len()];
	let sec_scaling = BigEndian::read_u32(&sec_scaling_bytes);

	let mut nonce_bytes = [0; 8];
	BigEndian::write_u64(&mut nonce_bytes, nonce);
	let mut nonce_vec = nonce_bytes.to_vec();

	// Generate new header
	pre_vec.append(&mut nonce_vec);
	pre_vec.append(&mut post_vec);

	(pre_vec, sec_scaling)
}

pub fn get_next_header_data(pre_nonce: &str, post_nonce: &str) -> (u64, Vec<u8>, u32) {
	let nonce: u64 = rand::OsRng::new().unwrap().gen();
    println!("nonce: {}", nonce);
	let (hd, sec_scaling) = header_data(pre_nonce, post_nonce, nonce);
	(nonce, hd, sec_scaling)
}

/// Helper to convert a hex string
pub fn from_hex_string(in_str: &str) -> Vec<u8> {
	let mut bytes = Vec::new();
	for i in 0..(in_str.len() / 2) {
		let res = u8::from_str_radix(&in_str[2 * i..2 * i + 2], 16);
		match res {
			Ok(v) => bytes.push(v),
			Err(e) => println!("Problem with hex: {}", e),
		}
	}
	bytes
}

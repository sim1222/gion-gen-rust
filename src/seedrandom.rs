use std::vec::Vec;
use std::str;
// use crypto::{rc4::Rc4, symmetriccipher::Encryptor};
// use arc4::Arc4;]
// use hex_literal::hex;

// seed はnumberからstringにしたもの
// 長いと指数表示のstringになる 例 9722779.1e+23
const WIDTH: u32 = 256;
const MASK: u8 = 255;


pub fn seed_random(seed: &str) {
    // let mut CHUNKS: i32 = 6;
    // let mut DIGITS: i32 = 52;
    // let mut START_DENOM: i32 = WIDTH.pow(CHUNKS as u32);
    // let mut SIGN_IFICANCE: i32 = (2 as i32).pow(DIGITS as u32);
    // let mut OVERFLOW: i32 = SIGN_IFICANCE * 2;
	// let mut n: u32 = 122299249329477;
	// let mut d: u32 = 281474976710656;

    let mut key: Vec<u8> = Vec::new(); // max 256 (mask) length u8 array

	let short_seed = mix_key(seed, &mut key);

	println!("shortSeed: {}", short_seed);
	println!("key: {:?}", &key);
	println!("key length: {}", key.len());


	// println!("{}", CHUNKS);

    // let mut n = [CHUNKS as u8];

    // rc4.apply_keystream(&mut n);

    // println!("n: {:?}", n);
}

fn mix_key(seed: &str, key: &mut Vec<u8>) -> String {
	let mut j: u8 = 0;
	let mut smear = 0;

	while j < seed.len() as u8 {
		// println!("{:?}", (MASK & j));
		smear ^= match key.get((MASK & j) as usize) {
			None => 0,
			Some(x) => *x * 19,
		};

		match key.get((MASK & j) as usize) {
			None => key.push(MASK & smear + u8::from(seed.chars().nth(j as usize).unwrap() as u8)),
			Some(_x) => key[(MASK & j) as usize] = MASK & smear + u8::from(seed.chars().nth(j as usize).unwrap() as u8),
		}
		j += 1;
	}
	return str::from_utf8(&key).unwrap().to_string();
}


// fn ARC4(key: [u8]) {
// 	let t = key.len();
// 	let key_length = key.len();

// 	let mut i = 0;
// 	let mut j = 0;
// 	let mut s = vec![];

//     let WIDTH = 256;

// 	while i < WIDTH {
// 		s[i] = i += 1;
// 		t += 1;
// 	}
		
// 	}
// }



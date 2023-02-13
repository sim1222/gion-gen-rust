// use std::vec::Vec;
// // use crypto::{rc4::Rc4, symmetriccipher::Encryptor};
// // use arc4::Arc4;]
// // use hex_literal::hex;
// use rc4::{consts::*, KeyInit, StreamCipher};
// use rc4::{Key, Rc4};

// pub fn seed_random(seed: &str) {
//     let WIDTH = 256;
//     // let mut CHUNKS: i32 = 6;
//     // let mut DIGITS: i32 = 52;
//     // let mut START_DENOM: i32 = WIDTH.pow(CHUNKS as u32);
//     // let mut SIGN_IFICANCE: i32 = (2 as i32).pow(DIGITS as u32);
//     // let mut OVERFLOW: i32 = SIGN_IFICANCE * 2;
//     // let mut MASK: i32 = WIDTH - 1;
// 	let mut n = 122299249329477;
// 	let mut d = 281474976710656;

//     let key = b"a";

//     let mut rc4 = Rc4::new(key.into());

// 	println!("{}", CHUNKS);

//     let mut n = [CHUNKS as u8];

//     rc4.apply_keystream(&mut n);

//     println!("n: {:?}", n);
// }

// // fn mixkey(seed: &str, key: &mut Vec<u32>) {
// // 	let mut j = 0;
// // 	let mut smear = 0;
// // 	for c in seed.chars() {
// // 		key[(MASK & j) as usize] = MASK & (smear ^= key[(MASK & j) as usize] * 19 + u32::from(c));
// // 		j += 1;
// // 	}
// // }


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



use num_bigint::{BigUint, ToBigInt};
use num_traits::zero;
use rand::Rng;
use std::borrow::BorrowMut;
use std::vec::Vec;
use std::{cell::RefCell, str};
// use crypto::{rc4::Rc4, symmetriccipher::Encryptor};
// use arc4::Arc4;]
// use hex_literal::hex;

// seed はnumberからstringにしたもの
// 長いと指数表示のstringになる 例 9722779.1e+23
const WIDTH: u32 = 256;
const MASK: u8 = 255;
const CHUNKS: u32 = 6;
const DIGITS: u32 = 52;
const SIGN_IFICANCE: u64 = (2 as u64).pow(DIGITS);
const OVERFLOW: u64 = SIGN_IFICANCE * 2;
const START_DENOM: u64 = (WIDTH as u64).pow(CHUNKS);

pub struct SeedRandom {
    pool: Vec<u8>,
    key: Vec<u8>,
    short_seed: String,
    arc: ARC4,
    n: u128,
    d: u128,
    x: u128,
}

impl SeedRandom {
    pub fn new(seed: &str) -> Self {
        let mut key: Vec<u8> = Vec::new(); // max 256 (mask) length u8 array
        let mut pool: Vec<u8> = Vec::new();

        let mut rng = rand::thread_rng();

        mix_key(&rng.gen::<f64>().to_string(), &mut pool);

        let short_seed = mix_key(seed, &mut key);

        println!("shortSeed: {}", short_seed);
        println!("key: {:?}", &key);
        println!("key length: {}", key.len());

        let arc = ARC4::new(&mut key);

        println!("arg.g()");

        // start generating random numbers
        let mut n = arc.clone().g(CHUNKS);
        let mut d = START_DENOM as u128;
        let mut x = 0;

        while n < SIGN_IFICANCE as u128 {
            // println!("n: {}", n);
            n = (n + x) * WIDTH as u128;
            d *= WIDTH as u128;
            x = arc.clone().g(1);
            // println!("n: {}", n);
            // println!("d: {}", d);
        }
        while n as u64 >= OVERFLOW {
            n /= 2;
            d /= 2;
            x >>= 1;
        }
        // println!("(n + x) / d: {}", (n + x) as f64 / d as f64);
        // end generating random numbers

        println!("n: {}", n);

        println!("mixkey 2");
        // seed is arc.s, pool is key
        mix_key(&arc.s.borrow_mut().to_string(), &mut pool);

        println!("mixkey 3 pool: {:?}", pool); // ここまでできた

        // println!("{}", CHUNKS);

        // let mut n = [CHUNKS as u8];

        // rc4.apply_keystream(&mut n);

        // println!("n: {:?}", n);

        Self { pool, key, short_seed, arc, n, d, x}
    }

    pub fn generate(mut self) -> f64 {
        println!("prng now");
        // start generating random numbers
        println!("CHUNKS: {}", CHUNKS);
        let mut n = self.arc.borrow_mut().clone().g(CHUNKS); // TODO: cannot change arc.s in g function
        let mut d = self.d;
        let mut x = self.x;
        println!("s: {:?}", self.arc.borrow_mut().clone().s);

        while n < SIGN_IFICANCE as u128 {
            // println!("n: {}", n);
            n = (n + x) * WIDTH as u128;
            d *= WIDTH as u128;
            x = self.arc.borrow_mut().clone().g(1);
            // println!("n: {}", n);
            // println!("d: {}", d);
        }
        while n as u64 >= OVERFLOW {
            n /= 2;
            d /= 2;
            x >>= 1;
        }
        println!("(n + x) / d: {}", (n + x) as f64 / d as f64);
        // end generating random numbers
        return (n + x) as f64 / d as f64;
    }
}

fn mix_key(seed: &str, key: &mut Vec<u8>) -> String {
    let mut j: u8 = 0;
    let mut smear: u32 = 0;

    println!("seed: {}", seed);

    while j < seed.len() as u8 {
        // println!("{:?}", (MASK & j));
        smear ^= match key.get((MASK & j) as usize) {
            None => 0,
            Some(x) => {
                println!("x * 19: {}", *x as u32 * 19);
                *x as u32 * 19
            }
        };
        // println!("smear: {}", smear);

        match key.get((MASK & j) as usize) {
            None => key.push(
                (MASK as u32 & (smear + u32::from(seed.chars().nth(j as usize).unwrap() as u32)))
                    .try_into()
                    .unwrap(),
            ),
            Some(_x) => {
                key[(MASK & j) as usize] = (MASK as u32
                    & (smear + u32::from(seed.chars().nth(j as usize).unwrap() as u32)))
                .try_into()
                .unwrap()
            }
        }
        j += 1;
    }
    return key.to_string();
}

#[derive(Clone)]
struct ARC4 {
    key: Vec<u8>,
    s: RefCell<Vec<u16>>,
    i: u32,
    j: u32,
}

impl ARC4 {
    fn new(key: &mut Vec<u8>) -> Self {
        let mut t: u16;
        let mut i = 0;
        let mut j = i;
        let mut s: Vec<u16> = vec![];

        match key.len() {
            0 => key.push(0),
            _ => (),
        }

        while i < WIDTH {
            match s.get(i as usize) {
                None => s.push(i as u16),
                Some(_x) => (),
            }
            i += 1;
        }

        println!("s before: {:?}", s);

        i = 0;
        while i < WIDTH {
            t = s[i as usize];
            // println!("t: {}", t);
            j = MASK as u32 & (j + key[i as usize % key.len()] as u32 + t as u32);
            // println!("j: {}", j);
            s[i as usize] = s[j as usize];
            // println!("s[i]: {}", s[i as usize]);
            s[j as usize] = t;
            // println!("s[j]: {}", s[j as usize]);
            i += 1;
        }

        println!("s after: {:?}", s);

        let mut t: u16;
        j = 0;
        i = 0;
        let mut c = 0;
        while c < WIDTH {
            let ti = i + 1;
            i = MASK as u32 & ti;
            println!("i: {}", i);
            t = s[i as usize];
            println!("t: {}", t);
            let tj = j + t as u32;
            // println!("tj: {}", tj);
            j = MASK as u32 & tj;
            println!("j: {}", j);
            s[i as usize] = s[j as usize];
            println!("s[i]: {}", s[i as usize]);
            s[j as usize] = t;
            println!("s[j]: {}", s[j as usize]);
            c += 1;
        }

        println!("s after2: {:?}", s);

        Self {
            key: key.clone(),
            s: RefCell::new(s),
            i,
            j,
        }
    }
    fn g(self, count: u32) -> u128 {
        let mut i = 0;
        let mut j = self.j;
        let mut s = self.s.borrow_mut();
        let mut t: u16;
        let mut r: u128 = 0;
        println!("s: {:?}", s);
        println!("i: {}", i);
        println!("j: {}", j);
        println!("count: {}", count);
        println!("-----------------");

        let mut c = 0;
        while c < count {
            let ti = i + 1;
            i = MASK as u32 & ti;
            println!("i: {}", i);
            t = s[i as usize];
            println!("t: {}", t);
            let tj = j + t as u32;
            println!("tj: {}", tj);
            j = MASK as u32 & tj;
            println!("j: {}", j);
            s[i as usize] = s[j as usize];
            println!("s[i]: {}", s[i as usize]);
            s[j as usize] = t;
            println!("s[j]: {}", s[j as usize]);
            let ts: u128 = s[i as usize] as u128 + s[j as usize] as u128;
            println!("ts: {}", ts);
            println!("r * WIDTH: {}", r * WIDTH as u128);
            r = r * WIDTH as u128 + s[MASK as usize & ts as usize] as u128;
            println!("r: {}", r);
            println!("------------------");
            c += 1;
        }
        println!("r: {}", r);
        println!("i: {}", i);
        println!("j: {}", j);
        return r;
    }
}

trait VecToString {
    fn to_string(&self) -> String;
}

impl VecToString for Vec<u8> {
    fn to_string(&self) -> String {
        // Vec<u8> -> Vec<u16>
        let mut v: Vec<u16> = vec![];
        for i in 0..self.len() {
            v.push(self[i] as u16);
        }
        return String::from_utf16(v.as_slice()).unwrap();
    }
}

impl VecToString for Vec<u16> {
    fn to_string(&self) -> String {
        return String::from_utf16(&self.clone()).unwrap();
    }
}

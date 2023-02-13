mod patterns;
mod seedrandom;

use std::thread;

use rand::{seq::{IteratorRandom, SliceRandom}, Rng};
use tokio_stream::StreamExt;

use std::time::{Duration, Instant};

fn get_random_hiragana() -> String {
    const HIRAGANA: &str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわんがぎぐげござじずぜぞだじづでどばびぶべぼぱぴぷぺぽ";
    HIRAGANA
        .chars()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}

fn get_random_hiragana_with_small_character() -> String {
    const HIRAGANA: &str = "きしちにひみりぎじぢびぴ";
    const SMALL_HIRAGANA: &str = "ゃゅょ";
    let res = HIRAGANA
        .chars()
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
        + &SMALL_HIRAGANA
            .chars()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
    return res;
}

fn parse(input: &str) -> String {
    let mut res = String::new();

    let A: String = get_random_hiragana();
    let a: String = get_random_hiragana();
    let B: String = get_random_hiragana_with_small_character();
    let b: String = get_random_hiragana_with_small_character();

    for c in input.chars() {
        match c {
            'A' => res.push_str(&A),
            'a' => res.push_str(&a),
            'B' => res.push_str(&B),
            'b' => res.push_str(&b),
            _ => res.push(c),
        }
    }
    return res;
}

fn generate(seed: Option<u64>) -> String {
    let mut salt = 0;
    let mut seed = match seed {
        Some(seed) => seed,
        None => {
            (rand::thread_rng().gen::<f64>() * 1000000.0).floor() as u64
        }
    };

    let case_number = (rand::thread_rng().gen::<f64>() * patterns::patterns().len() as f64).floor() * 2.0;

    let binding = patterns::patterns();
    let random_pattern = binding.get(case_number as usize);
    let random_pattern = match random_pattern {
        Some(pattern) => pattern,
        None => "AaAa",
    };

    return parse(random_pattern) + " " + &seed.to_string();
}

#[tokio::main]
async fn main() {

    let start = Instant::now();

    let num = 100;

    for _ in 0..num {
        println!("{}", generate(None));
        // generate();
        // thread::spawn(move || {
        //     generate();
        // });
    }

    let end = start.elapsed();
    println!(
        "{}個をを{}.{:03}秒で生成しました",
        num,
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

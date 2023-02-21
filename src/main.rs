mod patterns;
mod seedrandom;

use std::cell::RefCell;

use rand::Rng;

// use std::time::{Duration, Instant};

struct GionGenerator {
    seed_random: RefCell<seedrandom::SeedRandom>,
    // seed: RefCell<String>,
    // salt: u64,
}
impl GionGenerator {
    fn new() -> Self {
        let seed = ((rand::thread_rng().gen::<f64>() * 1000000.0).floor() as u64).to_string();
        let seed_random = seedrandom::SeedRandom::new(seed.clone());
        Self {
            seed_random: RefCell::new(seed_random),
            // seed: RefCell::new(seed),
            // salt: 0,
        }
    }

    fn pick_random_character(&self, string: String) -> String {
        // return string.chars().choose().unwrap().to_string();
        let num =
            (self.seed_random.borrow().generate() * string.chars().count() as f64).floor() as usize;
        // println!("num: {}", num);
        return string.chars().nth(num).unwrap().to_string();
    }

    fn get_random_hiragana(&self) -> String {
        const HIRAGANA: &str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわんがぎぐげござじずぜぞだじづでどばびぶべぼぱぴぷぺぽ";
        // println!("hiragana len: {}", HIRAGANA.chars().count());
        return self.pick_random_character(HIRAGANA.to_string());
    }

    fn get_random_hiragana_with_small_character(&self) -> String {
        const HIRAGANA: &str = "きしちにひみりぎじぢびぴ";
        const SMALL_HIRAGANA: &str = "ゃゅょ";

        let res = format!(
            "{}{}",
            self.pick_random_character(HIRAGANA.to_string()),
            self.pick_random_character(SMALL_HIRAGANA.to_string())
        );
        return res;
    }

    fn parse(&self, input: &str) -> String {
        let mut res = String::new();

        // println!("input: {}", input);

        let A: String = self.get_random_hiragana();
        let B: String = self.get_random_hiragana_with_small_character();
        let a: String = self.get_random_hiragana();
        let b: String = self.get_random_hiragana_with_small_character();

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

    fn generate(&self, seed: String) -> String {
        // let salt = self.salt;
        // *self.seed.borrow_mut() = seed;
        *self.seed_random.borrow_mut() = seedrandom::SeedRandom::new(seed);

        let random_number = self.seed_random.borrow();
        let random_number = random_number.generate();
        // println!("seed in generate: {}", seed);
        // println!("random_number: {}", random_number);

        let case_number = (random_number * patterns::patterns().len() as f64).floor() * 2.0;
        // println!("case_number: {}", case_number);

        let random_pattern = match patterns::patterns().get(case_number as usize) {
            Some(pattern) => pattern,
            None => "AaAa",
        };
        // println!("random_pattern: {}", random_pattern);

        return self.parse(random_pattern);
    }
}

#[tokio::main]
async fn main() {
    // let start = Instant::now();

    // let num = 100000;

    // for _ in 0..num {
    //     // println!("{}", generate(None));
    //     generate(None);
    //     // thread::spawn(move || {
    //     //     println!("{}", generate(None));
    //     // });
    // }

    // let end = start.elapsed();
    // println!(
    //     "{}個をを{}.{:03}秒で生成しました",
    //     num,
    //     end.as_secs(),
    //     end.subsec_nanos() / 1_000_000
    // );

    let gion_generator = GionGenerator::new();
    let res = gion_generator.generate("8816980".to_string());
    println!("{}", res);
}

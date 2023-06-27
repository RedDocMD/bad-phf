use std::{env, fs::File, io::{BufReader, BufRead}};

use phf_generator::generate_hash;
use tinyrand::Rand;

fn random_keys() -> Vec<u64> {
    let mut rng = tinyrand::StdRand::default();
    const KEY_CNT: usize = 1 << 12;
    (1..KEY_CNT).map(|_| rng.next_u64() & 0xFFFFFF).collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let keys = if args.len() > 1 {
        let file = File::open(&args[1]).unwrap();
        let mut rdr = BufReader::new(file);
        let mut line = String::new();
        let mut keys = Vec::new();
        while let Ok(_) = rdr.read_line(&mut line) {
            let key: u64 = line.parse().unwrap();
            keys.push(key);
        }
        keys
    } else {
        random_keys()
    };
    generate_hash(&keys);
}

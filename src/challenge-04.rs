mod lib;
use lib::hex;
use lib::single_byte_xor;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "data/challenge-04.txt";
    let file = File::open(filename).unwrap();
    let mut scored: Vec<(char, String, usize)> = 
        BufReader::new(file)
            .lines()
            .flat_map(|result| {
                let bytes = hex::decode(result.unwrap().as_str());
                single_byte_xor::score_against_range(b'\0'..=b'\x7f', &bytes)
            })
            .collect();
    scored.sort_by_key(|tuple| !tuple.2);
    assert_eq!(scored[0].1, "Now that the party is jumping\n")
}
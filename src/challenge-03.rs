mod lib;
use lib::hex;
use std::collections::HashSet;



fn xor(input: &[u8], character: u8) -> Vec<u8> {
    input.iter().map(|x| x ^ character).collect()
}

fn score(input: &[u8], frequency_table: &HashSet<char>) -> usize {
    input
    .iter()
    .filter(|&b| {
        let mut c = *b as char;
        c.make_ascii_lowercase();
        frequency_table.contains(&c)
    })
    .count()
}

fn main() {
    let bytes = hex::decode(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    );
    let frequency_table: HashSet<char> = vec![
        'e', 't', 'a', 'o',
        'i', 'n', 's', 'h',
        'd', 'l', 'u', ' ',
    ].into_iter().collect();

    let mut scored: Vec<(String, usize)> = (b'\0'..=b'\x7f')
        .into_iter()
        .map(|character| {
            let xord = xor(&bytes, character);
            (
                String::from_utf8(xord.clone()).expect("Found invalid UTF-8"), 
                score(&xord, &frequency_table)
            )
        })
        .collect();
    scored.sort_by_key(|tuple| !tuple.1);
    println!("{:#?}", scored[0..5].to_vec());
}
mod lib;
use lib::hex;
use lib::single_byte_xor;

fn main() {
    let bytes = hex::decode(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    );
    let mut scored = single_byte_xor::score_against_range(b'\0'..=b'\x7f', &bytes);
    scored.sort_by_key(|tuple| !tuple.2);
    assert_eq!(scored[0].1, "Cooking MC's like a pound of bacon");
}
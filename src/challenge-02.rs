mod lib;
use lib::hex;

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter()
     .zip(b.iter())
     .map(|(a, b)| {
        a ^ b
     })
     .collect()
}

fn main() {
    let out = xor(
        &hex::decode("1c0111001f010100061a024b53535009181c"),
        &hex::decode("686974207468652062756c6c277320657965"),
    );
    assert_eq!(hex::encode(&out), "746865206b696420646f6e277420706c6179");
}



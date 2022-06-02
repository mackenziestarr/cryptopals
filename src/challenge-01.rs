mod lib;
use lib::hex;
use lib::base64;

fn main() {
    let hex: &str = 
    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let answer = base64::encode(&hex::decode(hex));
    assert_eq!(answer, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
}



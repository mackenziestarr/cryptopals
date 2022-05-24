mod lib;
use lib::hex;

fn main() {
    // construct vector of all base64 characters
    let base64: Vec<u8> = {
        let mut start: Vec<u8> = (b'A'..=b'Z').collect();
        start.extend(b'a'..=b'z');
        start.extend(b'0'..=b'9');
        start.extend(vec![b'+', b'/', b'=']);
        start     
    };

    let hex: &str = 
    "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    // TODO: add support for padding
    // let hex: &str = "492";

    let base64_string: Vec<u8> = hex::decode(hex)
        .chunks(3)
        .map(|c| {
            let big_endian_bytes: [u8; 4] = [c[0], c[1], c[2], 0x00];
            u32::from_be_bytes(big_endian_bytes)
        })
        .flat_map(|big_endian_bytes| {
            (1..=4)
                .rev()
                .map(|index| {
                    let sextet = ((big_endian_bytes >> (6 * index)) & 0b1111_1100) >> 2;
                    let x: usize = (sextet).try_into().unwrap();
                    base64[x]
                })
                .collect::<Vec<u8>>()
        })
        .collect();
 
    println!("{}",String::from_utf8(base64_string).expect("Found invalid UTF-8"));
}



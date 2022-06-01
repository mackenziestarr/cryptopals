mod lib;
use lib::hex;

fn main() {
    let input =
    b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let output: Vec<u8> = 
        input
            .iter()
            .zip(b"ICE".iter().cycle())
            .map(|(a, b)| a ^ b)  
            .collect();
    let hex = hex::encode(&output);
    assert_eq!(&hex[0..75], "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272");
    assert_eq!(&hex[75..], "a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
}
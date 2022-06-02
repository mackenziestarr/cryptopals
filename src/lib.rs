pub mod single_byte_xor {
    use std::ops::RangeInclusive;

    static FREQUENCY_TABLE: [char; 12] = [
        'e', 't', 'a', 'o',
        'i', 'n', 's', 'h',
        'd', 'l', 'u', ' ',
    ];

    pub fn xor(input: &[u8], character: u8) -> Vec<u8> {
        input.iter().map(|x| x ^ character).collect()
    }

    pub fn score(input: &[u8]) -> usize {
        input
        .iter()
        .filter(|&b| {
            let mut c = *b as char;
            c.make_ascii_lowercase();
            FREQUENCY_TABLE.contains(&c)
        })
        .count()
    }

    pub fn score_against_range(range: RangeInclusive<u8>, bytes: &[u8]) -> Vec<(char, String, usize)> {
        range
            .into_iter()
            .map(|character| {
                let xord = xor(&bytes, character);
                (
                    character as char,
                    String::from_utf8_lossy(&xord).into_owned(), 
                    score(&xord)
                )
            })
            .collect()
    }

}

pub mod hex {
    use std::fmt;

    #[derive(Debug)]
    pub struct InvalidHexadecimal {
        byte: u8
    }
    impl fmt::Display for InvalidHexadecimal {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid hexadecimal: {:08b}", self.byte)
        }
    }

    fn hex_to_num(hex: &u8) -> Result<u8, InvalidHexadecimal> {
        match hex {
            b'0'..=b'9' => Ok(hex - b'0'),
            b'a'..=b'f' => Ok(hex - b'a' + 10),
            b'A'..=b'F' => Ok(hex - b'A' + 10),
            b => Err(InvalidHexadecimal{byte: *b})
        }
    }

    static HEXADECIMAL: [char; 16] = [
        '0', '1', '2', '3', 
        '4', '5', '6', '7', 
        '8', '9', 'a', 'b', 
        'c', 'd', 'e', 'f'
    ];
    pub fn encode(hex: &[u8]) -> String {
        let mut out = String::new();
        for byte in hex {
            out.push(HEXADECIMAL[((byte & 0xF0) >> 4) as usize]);
            out.push(HEXADECIMAL[(byte & 0x0F) as usize]);
        }
        out
    }

    pub fn decode(hex: &str) -> Vec<u8> {
        hex
            .as_bytes()
            .iter()
            .map(hex_to_num)
            .collect::<Result<Vec<u8>, InvalidHexadecimal>>()
            .unwrap()
            .chunks(2)
            .map(|w| w[0] << 4 | w[1])
            .collect()
    }

}

pub mod base64 {
    // TODO(@mstarr) add support for padding
    use std::collections::HashMap;

    static BASE64: &[u8; 65] = 
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    #[test]
    pub fn test_decode() {
        assert_eq!(
            decode(b"bGlnaHQgd29y"),
            b"light wor"
        )
    }

    pub fn decode(input: &[u8]) -> Vec<u8> {
        // map of base 64 chars to their index value e.g. 'B' => 2
        let mut base64_index = HashMap::new();
        BASE64.iter().enumerate().for_each(|(index, char)| {
            base64_index.insert(char, index as u8);        
        });
        input
            .iter()
            .map(|byte| base64_index[byte])
            .collect::<Vec<u8>>()
            .chunks(4)
            .flat_map(|chunk| {
                vec![
                    &chunk[0] << 2 | (&chunk[1] & 0b0011_0000) >> 4,
                    (&chunk[1] & 0b0000_1111) << 4 | (&chunk[2] & 0b0011_1100) >> 2,
                    (&chunk[2] & 0b0000_0011) << 6 | (&chunk[3] & 0b0011_1111)
                ]
            })
            .collect()
    }
    pub fn encode(input: &[u8]) -> String {
        let output: Vec<u8> = input
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
                        BASE64[x]
                    })
                    .collect::<Vec<u8>>()
            })
            .collect();
        String::from_utf8(output).expect("Found invalid UTF-8")    
    }
}
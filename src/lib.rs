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
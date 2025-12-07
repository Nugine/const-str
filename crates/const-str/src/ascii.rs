pub const fn num_to_hex_digit(x: u8) -> u8 {
    match x {
        0..=9 => b'0' + x,
        10..=15 => b'a' + (x - 10),
        _ => panic!("invalid hex number"),
    }
}

pub const fn num_from_dec_digit(d: u8) -> u8 {
    match d {
        b'0'..=b'9' => d - b'0',
        _ => panic!("invalid dec digit"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_hex_digit() {
        assert_eq!(num_to_hex_digit(0), b'0');
        assert_eq!(num_to_hex_digit(9), b'9');
        assert_eq!(num_to_hex_digit(10), b'a');
        assert_eq!(num_to_hex_digit(15), b'f');

        const HEX_5: u8 = num_to_hex_digit(5);
        assert_eq!(HEX_5, b'5');

        const HEX_12: u8 = num_to_hex_digit(12);
        assert_eq!(HEX_12, b'c');
    }

    #[test]
    fn test_num_from_dec_digit() {
        assert_eq!(num_from_dec_digit(b'0'), 0);
        assert_eq!(num_from_dec_digit(b'5'), 5);
        assert_eq!(num_from_dec_digit(b'9'), 9);

        const NUM_7: u8 = num_from_dec_digit(b'7');
        assert_eq!(NUM_7, 7);
    }
}

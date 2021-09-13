pub const fn reversed_bytes<const N: usize>(mut arr: [u8; N]) -> [u8; N] {
    let mut i = 0;
    while i * 2 < N {
        let a = arr[i];
        let b = arr[N - 1 - i];
        arr[i] = b;
        arr[N - 1 - i] = a;
        i += 1;
    }
    arr
}

#[test]
fn test_reversed_bytes() {
    let arr = [0, 1];
    assert_eq!(reversed_bytes(arr), [1, 0]);

    let arr = [0, 1, 2];
    assert_eq!(reversed_bytes(arr), [2, 1, 0]);
}

pub const fn merge_bytes<const N: usize>(mut buf: [u8; N], bytes: &[u8], count: usize) -> [u8; N] {
    const_assert!(N <= bytes.len());
    let mut i = 0;
    while i < count {
        buf[i] = bytes[i];
        i += 1;
    }
    buf
}

/// Copied from <https://github.com/rust-lang/rust/blob/0273e3bce7a0ce49e96a9662163e2380cb87e0be/library/core/src/char/methods.rs#L1600-L1645>
pub const fn encode_utf8(c: char) -> ([u8; 4], usize) {
    // UTF-8 ranges and tags for encoding characters
    const TAG_CONT: u8 = 0b1000_0000;
    const TAG_TWO_B: u8 = 0b1100_0000;
    const TAG_THREE_B: u8 = 0b1110_0000;
    const TAG_FOUR_B: u8 = 0b1111_0000;

    let code = c as u32;
    let mut buf = [0; 4];

    match c.len_utf8() {
        1 => {
            buf[0] = code as u8;
        }
        2 => {
            buf[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
            buf[1] = (code & 0x3F) as u8 | TAG_CONT;
        }
        3 => {
            buf[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
            buf[1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
            buf[2] = (code & 0x3F) as u8 | TAG_CONT;
        }
        4 => {
            buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
            buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
            buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
            buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        }
        _ => {}
    };

    (buf, c.len_utf8())
}

#[test]
fn test_encode_utf8() {
    let c = '我';
    let (buf, len) = encode_utf8(c);
    let output = core::str::from_utf8(&buf[..len]).unwrap();

    let mut ans = [0; 4];
    let ans = c.encode_utf8(&mut ans);

    assert_eq!(output, ans);
}

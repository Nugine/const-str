use crate::slice::advance;

pub struct CharEncodeUtf16 {
    buf: [u16; 2],
}

impl CharEncodeUtf16 {
    /// Copied from [char::encode_utf16](https://github.com/rust-lang/rust/blob/0273e3bce7a0ce49e96a9662163e2380cb87e0be/library/core/src/char/methods.rs#L1647-L1682)
    pub const fn new(ch: char) -> Self {
        let mut code = ch as u32;
        let mut buf = [0; 2];
        if (code & 0xFFFF) == code {
            buf[0] = code as u16;
        } else {
            code -= 0x1_0000;
            buf[0] = 0xD800 | ((code >> 10) as u16);
            buf[1] = 0xDC00 | ((code as u16) & 0x3FF);
        }
        Self { buf }
    }

    pub const fn has_second(&self) -> bool {
        self.buf[1] != 0
    }

    pub const fn first(&self) -> u16 {
        self.buf[0]
    }
    pub const fn second(&self) -> u16 {
        self.buf[1]
    }
}

pub const fn str_len_utf16(s: &str) -> usize {
    let mut s = s.as_bytes();
    let mut ans = 0;
    while let Some((ch, count)) = crate::utf8::next_char(s) {
        s = advance(s, count);
        ans += ch.len_utf16(); // const since 1.52
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_encode_utf16() {
        const E1: CharEncodeUtf16 = CharEncodeUtf16::new('A');
        assert_eq!(E1.first(), 'A' as u16);
        assert!(!E1.has_second());
        
        const E2: CharEncodeUtf16 = CharEncodeUtf16::new('我');
        assert_eq!(E2.first(), '我' as u16);
        assert!(!E2.has_second());
        
        // Test a character that requires a surrogate pair
        const E3: CharEncodeUtf16 = CharEncodeUtf16::new('𐍈'); // Gothic letter
        assert!(E3.has_second());
        assert_eq!(E3.first(), 0xD800);
        assert_eq!(E3.second(), 0xDF48);
    }
    
    #[test]
    fn test_str_len_utf16() {
        const LEN1: usize = str_len_utf16("hello");
        assert_eq!(LEN1, 5);
        
        const LEN2: usize = str_len_utf16("你好");
        assert_eq!(LEN2, 2);
        
        const LEN3: usize = str_len_utf16("");
        assert_eq!(LEN3, 0);
        
        // Emoji and characters requiring surrogate pairs
        const LEN4: usize = str_len_utf16("𐍈");
        assert_eq!(LEN4, 2); // Requires surrogate pair
        
        const LEN5: usize = str_len_utf16("A𐍈B");
        assert_eq!(LEN5, 4); // A(1) + 𐍈(2) + B(1)
    }
}

pub struct IsAscii<T>(pub T);

impl IsAscii<&[u8]> {
    pub const fn const_eval(&self) -> bool {
        let bytes = self.0;
        let mut i = 0;
        while i < bytes.len() {
            if !bytes[i].is_ascii() {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl IsAscii<&str> {
    pub const fn const_eval(&self) -> bool {
        IsAscii(self.0.as_bytes()).const_eval()
    }
}

impl<const N: usize> IsAscii<&[u8; N]> {
    pub const fn const_eval(&self) -> bool {
        IsAscii(self.0.as_slice()).const_eval()
    }
}

/// Checks if all characters in this (string) slice are within the ASCII range.
///
/// The input type must be one of:
/// + [`&str`](str)
/// + [`&[u8]`](slice)
/// + [`&[u8; N]`](array)
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
///
/// # Examples
///
/// ```
/// const S1: &str = "hello!\n";
/// const S2: &str = "你好！";
///
/// const _: () = {
///     assert!(const_str::is_ascii!(S1));              // true
///     assert!(!const_str::is_ascii!(S2));             // false
///     assert!(!const_str::is_ascii!(b"\x80\x00"));    // false
/// };
/// ```
///
#[macro_export]
macro_rules! is_ascii {
    ($s:expr) => {
        $crate::__ctfe::IsAscii($s).const_eval()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_ascii() {
        const S1: &str = "hello!\n";
        const S2: &str = "你好！";
        const S3: &str = "";
        const S4: &str = "ASCII123";

        let r1 = is_ascii!(S1);
        let r2 = is_ascii!(S2);
        let r3 = is_ascii!(S3);
        let r4 = is_ascii!(S4);

        assert!(r1);
        assert!(!r2);
        assert!(r3); // empty string is ASCII
        assert!(r4);

        // Test with byte slices
        const B1: &[u8] = b"hello";
        const B2: &[u8] = b"\x80\x00";

        let rb1 = is_ascii!(B1);
        let rb2 = is_ascii!(B2);

        assert!(rb1);
        assert!(!rb2);

        // Test with byte arrays
        const A1: &[u8; 5] = b"hello";
        const A2: &[u8; 2] = b"\x80\x00";

        let ra1 = is_ascii!(A1);
        let ra2 = is_ascii!(A2);

        assert!(ra1);
        assert!(!ra2);
    }
}

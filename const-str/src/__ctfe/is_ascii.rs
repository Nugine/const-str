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

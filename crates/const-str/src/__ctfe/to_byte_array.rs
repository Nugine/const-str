pub struct ToByteArray<T>(pub T);

impl ToByteArray<&str> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(self.0.as_bytes())
    }
}

impl<const L: usize> ToByteArray<&[u8; L]> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(self.0)
    }
}

/// Converts a string slice or a byte string to a byte array.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
/// ```
/// const S: &str = "hello";
/// const B: &[u8; 6] = b"hello\0";
/// assert_eq!(const_str::to_byte_array!(S), [b'h', b'e', b'l', b'l', b'o']);
/// assert_eq!(const_str::to_byte_array!(B), [b'h', b'e', b'l', b'l', b'o', b'\0']);
/// ```
///
#[macro_export]
macro_rules! to_byte_array {
    ($s: expr) => {{
        const OUTPUT_LEN: usize = $s.len();
        $crate::__ctfe::ToByteArray($s).const_eval::<OUTPUT_LEN>()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_byte_array() {
        const S: &str = "hello";
        const B: &[u8; 6] = b"hello\0";
        const R1: [u8; 5] = to_byte_array!(S);
        const R2: [u8; 6] = to_byte_array!(B);

        assert_eq!(R1, [b'h', b'e', b'l', b'l', b'o']);
        assert_eq!(R2, [b'h', b'e', b'l', b'l', b'o', b'\0']);

        const EMPTY: &str = "";
        const R3: [u8; 0] = to_byte_array!(EMPTY);
        assert_eq!(R3, []);

        const BYTES: &[u8; 3] = b"abc";
        const R4: [u8; 3] = to_byte_array!(BYTES);
        assert_eq!(R4, [b'a', b'b', b'c']);
    }

    #[test]
    fn test_to_byte_array_runtime() {
        // Runtime tests for ToByteArray with &str
        let to_arr_str = ToByteArray("test");
        let result: [u8; 4] = to_arr_str.const_eval();
        assert_eq!(result, [b't', b'e', b's', b't']);

        // Runtime tests for ToByteArray with &[u8; N]
        let arr: &[u8; 3] = b"xyz";
        let to_arr_bytes = ToByteArray(arr);
        let result2: [u8; 3] = to_arr_bytes.const_eval();
        assert_eq!(result2, [b'x', b'y', b'z']);

        // Test empty
        let to_arr_empty = ToByteArray("");
        let result3: [u8; 0] = to_arr_empty.const_eval();
        assert_eq!(result3, []);
    }
}

pub struct ToByteArray<T>(pub T);

impl ToByteArray<&str> {
    pub const fn const_eval<const N: usize>(&self, init_val: u8) -> [u8; N] {
        crate::bytes::clone(self.0.as_bytes(), init_val)
    }
}

impl<const L: usize> ToByteArray<&[u8; L]> {
    pub const fn const_eval<const N: usize>(&self, init_val: u8) -> [u8; N] {
        crate::bytes::clone(self.0, init_val)
    }
}

/// Converts a string slice or a byte string to a byte array.
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
        $crate::__ctfe::ToByteArray($s).const_eval::<OUTPUT_LEN>(0)
    }};
}

/// Converts a string slice or a byte string to a byte array,
/// provide a length of the array you'd create and fill with the initial value you expect.
///
/// # Examples
/// ```
/// const S: &str = "hello";
/// const B: &[u8; 6] = b"hello\0";
/// assert_eq!(const_str::to_byte_array_with_len!(S, 7, 0), [b'h', b'e', b'l', b'l', b'o', 0, 0]);
/// assert_eq!(const_str::to_byte_array_with_len!(B, 7, 0), [b'h', b'e', b'l', b'l', b'o', b'\0', 0]);
/// ```
///
#[macro_export]
macro_rules! to_byte_array_with_len {
    ($s: expr, $len: expr, $init: expr) => {{
        $crate::__ctfe::ToByteArray($s).const_eval::<$len>($init)
    }};
}

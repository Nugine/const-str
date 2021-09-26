pub struct Len<T>(pub T);

impl Len<&str> {
    pub const fn const_eval(&self) -> usize {
        self.0.len()
    }
}

impl<const L: usize> Len<&[u8; L]> {
    pub const fn const_eval(&self) -> usize {
        L
    }
}

/// Returns the length of a string slice or a byte string
///
/// # Examples
/// ```
/// const S: &str = "hello";
/// const B: &[u8; 6] = b"hello\0";
/// assert_eq!(const_str::len!(S), 5_usize);
/// assert_eq!(const_str::len!(B), 6_usize);
/// ```
///
#[macro_export]
macro_rules! len {
    ($s: expr) => {{
        $crate::__ctfe::Len($s).const_eval()
    }};
}

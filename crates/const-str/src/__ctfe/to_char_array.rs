pub struct ToCharArray<T>(pub T);

impl ToCharArray<&str> {
    pub const fn output_len(&self) -> usize {
        crate::utf8::str_count_chars(self.0)
    }

    pub const fn const_eval<const N: usize>(&self) -> [char; N] {
        crate::utf8::str_chars(self.0)
    }
}

/// Converts a string slice into an array of its characters.
///
/// This macro is [const-context only](./index.html#const-context-only).
///
/// # Examples
/// ```
/// const CHARS: [char; 5] = const_str::to_char_array!("Hello");
/// assert_eq!(CHARS, ['H', 'e', 'l', 'l', 'o']);
/// ```
///
#[macro_export]
macro_rules! to_char_array {
    ($s: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::ToCharArray($s).output_len();
        const OUTPUT_BUF: [char; OUTPUT_LEN] =
            $crate::__ctfe::ToCharArray($s).const_eval::<OUTPUT_LEN>();
        OUTPUT_BUF
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_char_array() {
        const CHARS: [char; 5] = to_char_array!("Hello");
        assert_eq!(CHARS, ['H', 'e', 'l', 'l', 'o']);
        
        const EMPTY: [char; 0] = to_char_array!("");
        assert_eq!(EMPTY, []);
        
        const UNICODE: [char; 3] = to_char_array!("你好！");
        assert_eq!(UNICODE, ['你', '好', '！']);
        
        const SINGLE: [char; 1] = to_char_array!("A");
        assert_eq!(SINGLE, ['A']);
    }
}

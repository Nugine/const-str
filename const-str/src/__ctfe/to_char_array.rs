pub struct ToCharArray<T>(pub T);

impl ToCharArray<&str> {
    pub const fn output_len(&self) -> usize {
        crate::utf8::str_count_chars(self.0)
    }

    pub const fn const_eval<const N: usize>(&self, fill: char) -> [char; N] {
        crate::utf8::str_chars(self.0, fill)
    }
}

/// Converts a string slice into an array of its characters.
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
            $crate::__ctfe::ToCharArray($s).const_eval::<OUTPUT_LEN>('\0');
        OUTPUT_BUF
    }};
}

/// Converts a string slice into an array of its characters, 
/// provide a length of the array you'd create and fill with the initial value you expect.
///
/// # Examples
/// ```
/// const CHARS: [char; 7] = const_str::to_char_array_with_len!("Hello", 7, '\0');
/// assert_eq!(CHARS, ['H', 'e', 'l', 'l', 'o', '\0', '\0']);
/// ```
///
#[macro_export]
macro_rules! to_char_array_with_len {
    ($s: expr, $len: expr, $fill: expr) => {{
        const OUTPUT_BUF: [char; $len] =
            $crate::__ctfe::ToCharArray($s).const_eval::<$len>($fill);
        OUTPUT_BUF
    }};
}
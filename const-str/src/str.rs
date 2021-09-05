/// Returns the lowercase equivalent of this string literal, as a new string literal.
///
/// See [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase).
///
/// # Examples
///
/// ```
/// assert_eq!("hello", const_str::to_lowercase!("HELLO"));
/// ```
///
#[macro_export]
macro_rules! to_lowercase {
    ($str:literal) => {
        $crate::__imp::to_lowercase!($str)
    };
}

/// Returns the uppercase equivalent of this string literal, as a new string literal.
///
/// See [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase).
///
/// # Examples
///
/// ```
/// assert_eq!("HELLO", const_str::to_uppercase!("hello"));
/// ```
///
#[macro_export]
macro_rules! to_uppercase {
    ($str:literal) => {
        $crate::__imp::to_uppercase!($str)
    };
}

/// Replaces all matches of a pattern with another string literal.
///
/// See [`str::replace`](https://doc.rust-lang.org/std/primitive.str.html#method.replace).
///
/// # Examples
///
/// ```
/// assert_eq!("this is new", const_str::replace!("this is old", "old", "new"));
/// ```
///
#[macro_export]
macro_rules! replace {
    ($str:literal, $from:literal, $to:literal) => {
        $crate::__imp::replace!($str, $from, $to)
    };
}

/// Converts a string literal to a byte string literal
///
/// # Examples
/// ```
/// let bytes: &'static [u8;4] = const_str::as_bytes!("file");
/// assert_eq!(bytes, b"file");
/// ```
///
#[macro_export]
macro_rules! as_bytes {
    ($str:literal) => {
        $crate::__imp::as_bytes!($str)
    };
}

/// Converts a byte string literal to a string literal
///
/// # Examples
/// ```
/// let name: &'static str = const_str::from_utf8!(b"file");
/// assert_eq!(name, "file");
/// ```
///
#[macro_export]
macro_rules! from_utf8 {
    ($str:literal) => {
        $crate::__imp::from_utf8!($str)
    };
}

/// Returns the length of a string literal or byte string literal
///
/// # Examples
/// ```
/// assert_eq!(const_str::len!("file"), 4_usize);
/// assert_eq!(const_str::len!(b"file\0"), 5_usize);
/// ```
///
#[macro_export]
macro_rules! len {
    ($str:literal) => {
        $crate::__imp::len!($str)
    };
}

/// Converts a string literal into an array of its characters.
///
/// # Examples
/// ```
/// let chars = const_str::to_char_array!("Hello");
/// assert_eq!(chars[..], ['H', 'e', 'l', 'l', 'o']);
/// ```
///
#[macro_export]
macro_rules! to_char_array {
    ($str:literal) => {
        $crate::__imp::to_char_array!($str)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __convert_str_bytes {
    ($str: expr, $f: ident) => {{
        type str = ::core::primitive::str;
        type usize = ::core::primitive::usize;
        type u8 = ::core::primitive::u8;

        const INPUT: &str = $str;

        const OUTPUT_LEN: usize = INPUT.len();

        const OUTPUT_BYTES: [u8; OUTPUT_LEN] = {
            let mut buf = [0u8; OUTPUT_LEN];
            let bytes = INPUT.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                buf[i] = $f(bytes[i]);
                i += 1;
            }
            buf
        };

        OUTPUT_BYTES
    }};
}

/// Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.
///
/// # Examples
/// ```
/// const S: &str = "Hello, World";
/// assert_eq!(const_str::to_ascii_uppercase!(S), "HELLO, WORLD");
/// ```
///
#[macro_export]
macro_rules! to_ascii_uppercase {
    ($str: expr) => {{
        type u8 = ::core::primitive::u8;

        const fn upper(x: u8) -> u8 {
            match x {
                b'a'..=b'z' => x - (b'a' - b'A'),
                _ => x,
            }
        }

        unsafe { ::core::str::from_utf8_unchecked(&{ $crate::__convert_str_bytes!($str, upper) }) }
    }};
}

/// Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.
///
/// # Examples
/// ```
/// const S: &str = "Hello, World";
/// assert_eq!(const_str::to_ascii_lowercase!(S), "hello, world");
/// ```
///
#[macro_export]
macro_rules! to_ascii_lowercase {
    ($str: expr) => {{
        type u8 = ::core::primitive::u8;

        const fn lower(x: u8) -> u8 {
            match x {
                b'A'..=b'Z' => x + (b'a' - b'A'),
                _ => x,
            }
        }
        unsafe { ::core::str::from_utf8_unchecked(&{ $crate::__convert_str_bytes!($str, lower) }) }
    }};
}

/// Converts a string to a byte array.
///
/// # Examples
/// ```
/// const S: &str = "hello";
/// const B: [u8; S.len()] = const_str::to_byte_array!(S);
/// assert_eq!(B, [b'h', b'e', b'l', b'l', b'o']);
/// ```
///
#[macro_export]
macro_rules! to_byte_array {
    ($str: expr) => {{
        const fn pass(x: u8) -> u8 {
            x
        }
        $crate::__convert_str_bytes!($str, pass)
    }};
}

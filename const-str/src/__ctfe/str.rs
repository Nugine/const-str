#![allow(unsafe_code)]

pub struct StrBuf<const N: usize>([u8; N]);

impl<const N: usize> StrBuf<N> {
    /// # Safety
    /// `buf` must contain valid utf-8 bytes.
    pub const unsafe fn new_unchecked(buf: [u8; N]) -> Self {
        #[cfg(debug_assertions)]
        {
            assert!(core::str::from_utf8(&buf).is_ok())
        }
        Self(buf)
    }

    // const since 1.55
    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.0) }
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn from_str(s: &str) -> Self {
        let buf = crate::bytes::clone::<N>(s.as_bytes());
        unsafe { Self::new_unchecked(buf) }
    }
}

/// Converts a byte string to a string slice
///
/// # Examples
/// ```
/// const BYTE_PATH: &[u8] = b"/tmp/file";
/// const PATH: &str = const_str::from_utf8!(BYTE_PATH);
///
/// assert_eq!(PATH, "/tmp/file");
/// ```
///
#[macro_export]
macro_rules! from_utf8 {
    ($s: expr) => {{
        use ::core::primitive::str;
        // const since 1.63
        const OUTPUT: &str = match ::core::str::from_utf8($s) {
            Ok(s) => s,
            Err(_) => panic!("invalid utf-8 bytes"),
        };
        OUTPUT
    }};
}

#![allow(unsafe_code)]

pub struct StrBuf<const N: usize>([u8; N]);

impl<const N: usize> StrBuf<N> {
    /// # Safety
    /// `buf` must contain valid utf-8 bytes.
    pub const unsafe fn new_unchecked(buf: [u8; N]) -> Self {
        Self(buf)
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.0) } // const since 1.55.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub const fn from_str(s: &str) -> Self {
        let buf = crate::bytes::clone::<N>(s.as_bytes());
        unsafe { Self::new_unchecked(buf) }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __strbuf_as_str {
    ($b: expr) => {{
        #[allow(clippy::transmute_bytes_to_str, unsafe_code)]
        unsafe {
            ::core::mem::transmute::<&[u8], &str>($crate::__ctfe::StrBuf::as_bytes($b))
        }
    }};
}

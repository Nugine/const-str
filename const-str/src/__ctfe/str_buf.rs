#![allow(unsafe_code)]

pub struct StrBuf<const N: usize>([u8; N]);

impl<const N: usize> StrBuf<N> {
    /// # Safety
    /// `buf` must contain valid utf-8 bytes.
    pub const unsafe fn new_unchecked(buf: [u8; N]) -> Self {
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

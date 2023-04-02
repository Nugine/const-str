#![allow(unsafe_code)]

pub struct ToCStr<T>(pub T);

impl ToCStr<&str> {
    const fn check_nul(&self) {
        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            assert!(bytes[i] != 0);
            i += 1;
        }
    }

    pub const fn output_len(&self) -> usize {
        self.check_nul();
        self.0.as_bytes().len() + 1
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0; N];
        let mut pos = 0;
        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            assert!(bytes[i] != 0);
            buf[pos] = bytes[i];
            pos += 1;
            i += 1;
        }
        pos += 1;
        assert!(pos == N);
        buf
    }
}

/// Converts a string slice to [`*const c_char`](core::ffi::c_char).
///
/// The C-style string is guaranteed to be terminated by a nul byte.
/// This trailing nul byte will be appended by this macro.
/// The provided data should not contain any nul bytes in it.
///
/// See also [`cstr!`](crate::cstr)
///
/// # Examples
///
/// ```
/// use core::ffi::c_char;
/// const PRINTF_FMT: *const c_char = const_str::raw_cstr!("%d\n");
/// ```
#[macro_export]
macro_rules! raw_cstr {
    ($s: expr) => {
        $crate::cstr!($s).as_ptr()
    };
}

/// Converts a string slice to [`&CStr`](core::ffi::CStr).
///
/// The C-style string is guaranteed to be terminated by a nul byte.
/// This trailing nul byte will be appended by this macro.
/// The provided data should not contain any nul bytes in it.
///
/// See also [`raw_cstr!`](crate::raw_cstr)
///
/// # Examples
///
/// ```
/// use core::ffi::CStr;;
/// const PRINTF_FMT: &CStr = const_str::cstr!("%d\n");
/// ```
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        const OUTPUT_LEN: ::core::primitive::usize = $crate::__ctfe::ToCStr($s).output_len();
        const OUTPUT_BUF: [u8; OUTPUT_LEN] = $crate::__ctfe::ToCStr($s).const_eval();
        const OUTPUT: &::core::ffi::CStr =
            unsafe { ::core::ffi::CStr::from_bytes_with_nul_unchecked(&OUTPUT_BUF) };
        OUTPUT
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_raw_cstr() {
        const FMT: &str = "%d\n";
        let fmt = raw_cstr!(FMT);
        let len = FMT.len() + 1;
        let bytes: &[u8] = unsafe { core::slice::from_raw_parts(fmt.cast(), len) };
        assert_eq!(bytes, b"%d\n\0");
    }
}

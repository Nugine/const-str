pub struct ConcatBytesPart<T>(pub T);

impl ConcatBytesPart<u8> {
    pub const fn output_len(&self) -> usize {
        1
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(&[self.0])
    }
}

impl<const L: usize> ConcatBytesPart<&[u8; L]> {
    pub const fn output_len(&self) -> usize {
        L
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(self.0)
    }
}

impl ConcatBytesPart<&[u8]> {
    pub const fn output_len(&self) -> usize {
        self.0.len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(self.0)
    }
}

impl<const L: usize> ConcatBytesPart<[u8; L]> {
    pub const fn output_len(&self) -> usize {
        L
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(&self.0)
    }
}

impl ConcatBytesPart<&str> {
    pub const fn output_len(&self) -> usize {
        self.0.len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        crate::bytes::clone(self.0.as_bytes())
    }
}

pub struct ConcatBytes<'a>(pub &'a [&'a [u8]]);

impl ConcatBytes<'_> {
    pub const fn output_len(&self) -> usize {
        let parts = self.0;
        let mut sum = 0;
        let mut i = 0;
        while i < parts.len() {
            sum += parts[i].len();
            i += 1;
        }
        sum
    }

    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        let mut buf = [0; N];
        let mut pos = 0;

        macro_rules! push {
            ($x:expr) => {
                buf[pos] = $x;
                pos += 1;
            };
        }

        let parts = self.0;
        let mut i = 0;
        while i < parts.len() {
            let part = parts[i];
            let mut j = 0;
            while j < part.len() {
                push!(part[j]);
                j += 1;
            }
            i += 1;
        }

        assert!(pos == N);
        buf
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concat_bytes_part {
    ($x: expr) => {{
        const OUTPUT_LEN: usize = $crate::__ctfe::ConcatBytesPart($x).output_len();
        const OUTPUT_BUF: [u8; OUTPUT_LEN] = $crate::__ctfe::ConcatBytesPart($x).const_eval();
        const OUTPUT: &[u8] = &OUTPUT_BUF;
        OUTPUT
    }};
}

/// Concatenates values into a byte slice.
///
/// The input type must be one of
/// + [`u8`]
/// + [`&[u8]`](slice)
/// + [`[u8; N]`](array), [`&[u8; N]`](array)
/// + [`&str`](str)
///
/// The output type is [`&[u8; _]`](array).
///
/// # Examples
///
/// ```rust
/// const S1: &[u8; 7] = const_str::concat_bytes!(b'A', b"BC", [68, b'E', 70], "G");
/// const S2: &[u8] = const_str::concat_bytes!(S1, "/123", 0u8);
/// assert_eq!(S1, b"ABCDEFG");
/// assert_eq!(S2, b"ABCDEFG/123\x00");
/// ```
///
#[macro_export]
macro_rules! concat_bytes {
    ($($x: expr),+ $(,)?) => {{
        const PARTS: &[&[u8]] = &[$( $crate::__concat_bytes_part!($x) ),+];
        const OUTPUT_LEN: usize = $crate::__ctfe::ConcatBytes(PARTS).output_len();
        const OUTPUT_BUF: [u8; OUTPUT_LEN] = $crate::__ctfe::ConcatBytes(PARTS).const_eval();
        &OUTPUT_BUF
    }};
}

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
/// This macro is [const-context only](./index.html#const-context-only).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_bytes() {
        const S1: &[u8; 7] = concat_bytes!(b'A', b"BC", [68, b'E', 70], "G");
        const S2: &[u8] = concat_bytes!(S1, "/123", 0u8);
        assert_eq!(S1, b"ABCDEFG");
        assert_eq!(S2, b"ABCDEFG/123\x00");

        const S3: &[u8] = concat_bytes!(b"hello", b" ", b"world");
        assert_eq!(S3, b"hello world");

        const S4: &[u8] = concat_bytes!(b'x');
        assert_eq!(S4, b"x");

        const S5: &[u8] = concat_bytes!("test", b"123");
        assert_eq!(S5, b"test123");

        const ARR: [u8; 3] = [1, 2, 3];
        const S6: &[u8] = concat_bytes!(ARR, [4, 5]);
        assert_eq!(S6, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_concat_bytes_runtime() {
        // Runtime tests for ConcatBytesPart
        let part_u8 = ConcatBytesPart(42u8);
        assert_eq!(part_u8.output_len(), 1);
        let buf: [u8; 1] = part_u8.const_eval();
        assert_eq!(buf, [42]);

        let arr: &[u8; 3] = b"abc";
        let part_arr = ConcatBytesPart(arr);
        assert_eq!(part_arr.output_len(), 3);
        let buf_arr: [u8; 3] = part_arr.const_eval();
        assert_eq!(buf_arr, [b'a', b'b', b'c']);

        let slice: &[u8] = b"hello";
        let part_slice = ConcatBytesPart(slice);
        assert_eq!(part_slice.output_len(), 5);
        let buf_slice: [u8; 5] = part_slice.const_eval();
        assert_eq!(buf_slice, *b"hello");

        let owned_arr: [u8; 2] = [1, 2];
        let part_owned = ConcatBytesPart(owned_arr);
        assert_eq!(part_owned.output_len(), 2);
        let buf_owned: [u8; 2] = part_owned.const_eval();
        assert_eq!(buf_owned, [1, 2]);

        let str_part = ConcatBytesPart("test");
        assert_eq!(str_part.output_len(), 4);
        let buf_str: [u8; 4] = str_part.const_eval();
        assert_eq!(buf_str, *b"test");

        // Runtime tests for ConcatBytes
        let parts: &[&[u8]] = &[b"hello", b"world"];
        let concat = ConcatBytes(parts);
        assert_eq!(concat.output_len(), 10);
        let buf: [u8; 10] = concat.const_eval();
        assert_eq!(&buf, b"helloworld");

        let empty_parts: &[&[u8]] = &[];
        let concat_empty = ConcatBytes(empty_parts);
        assert_eq!(concat_empty.output_len(), 0);
        let buf_empty: [u8; 0] = concat_empty.const_eval();
        assert_eq!(&buf_empty, b"");

        // Test multiple parts
        let multi_parts: &[&[u8]] = &[b"a", b"b", b"c"];
        let concat_multi = ConcatBytes(multi_parts);
        assert_eq!(concat_multi.output_len(), 3);
        let buf_multi: [u8; 3] = concat_multi.const_eval();
        assert_eq!(&buf_multi, b"abc");
    }
}

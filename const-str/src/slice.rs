#![allow(unsafe_code)]

use core::ops::Range;
use core::slice;

pub const fn advance<T>(s: &[T], count: usize) -> &[T] {
    let len = s.len();
    assert!(count <= len);

    let base = s.as_ptr();
    unsafe { slice::from_raw_parts(base.add(count), len - count) }
}

pub const fn subslice<T>(s: &[T], range: Range<usize>) -> &[T] {
    let len = s.len();
    assert!(range.start <= range.end && range.end <= len);

    let base = s.as_ptr();
    unsafe { slice::from_raw_parts(base.add(range.start), range.end - range.start) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subslice() {
        let buf = b"abcdefgh";

        assert_eq!(subslice(buf, 0..0), &[]);
        assert_eq!(subslice(buf, 0..1), b"a");
        assert_eq!(subslice(buf, 1..3), b"bc");
        assert_eq!(subslice(buf, 7..8), b"h");

        assert_eq!(subslice::<u8>(&[], 0..0), &[]);
    }
}

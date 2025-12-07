pub struct EqIgnoreAsciiCase<T1, T2>(pub T1, pub T2);

const fn eq_ignore_ascii_case(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        let l = lhs[i].to_ascii_lowercase();
        let r = rhs[i].to_ascii_lowercase();
        if l != r {
            return false;
        }
        i += 1;
    }
    true
}

impl EqIgnoreAsciiCase<&[u8], &[u8]> {
    pub const fn const_eval(&self) -> bool {
        eq_ignore_ascii_case(self.0, self.1)
    }
}

impl EqIgnoreAsciiCase<&str, &str> {
    pub const fn const_eval(&self) -> bool {
        eq_ignore_ascii_case(self.0.as_bytes(), self.1.as_bytes())
    }
}

impl<const N1: usize, const N2: usize> EqIgnoreAsciiCase<&[u8; N1], &[u8; N2]> {
    pub const fn const_eval(&self) -> bool {
        eq_ignore_ascii_case(self.0.as_slice(), self.1.as_slice())
    }
}

/// Checks that two (string) slices are an ASCII case-insensitive match.
///
/// The input type must be one of:
/// + [`&str`](str)
/// + [`&[u8]`](slice)
/// + [`&[u8; N]`](array)
///
/// The output type is [`bool`].
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
///
/// # Examples
///
/// ```
/// use const_str::eq_ignore_ascii_case;
///
/// const _: () = {
///     assert!(eq_ignore_ascii_case!("Ferris", "FERRIS"));     // true
///     assert!(!eq_ignore_ascii_case!(b"Ferris", b"FERRI"));   // false
///
///     assert!(eq_ignore_ascii_case!("Ferrös", "FERRöS"));     // true
///     //                              ^^^ ^     ^^^ ^     
///
///     assert!(!eq_ignore_ascii_case!("Ferrös", "FERRÖS"));    // false
///     //                                  ^         ^
/// };
/// ```
#[macro_export]
macro_rules! eq_ignore_ascii_case {
    ($lhs:expr, $rhs:expr) => {
        $crate::__ctfe::EqIgnoreAsciiCase($lhs, $rhs).const_eval()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_eq_ignore_ascii_case_str() {
        const S1: &str = "Ferris";
        const S2: &str = "FERRIS";
        const S3: &str = "ferris";
        const S4: &str = "FERRI";

        let r1 = eq_ignore_ascii_case!(S1, S2);
        let r2 = eq_ignore_ascii_case!(S1, S3);
        let r3 = eq_ignore_ascii_case!(S1, S4);

        assert!(r1);
        assert!(r2);
        assert!(!r3);

        const S5: &str = "Hello";
        const S6: &str = "hello";
        let r4 = eq_ignore_ascii_case!(S5, S6);
        assert!(r4);

        // Non-ASCII characters should match themselves
        const S7: &str = "Ferrös";
        const S8: &str = "FERRöS";
        let r5 = eq_ignore_ascii_case!(S7, S8);
        assert!(r5);

        // Non-ASCII characters won't match their "uppercase" versions
        const S9: &str = "Ferrös";
        const S10: &str = "FERRÖS";
        let r6 = eq_ignore_ascii_case!(S9, S10);
        assert!(!r6);
    }

    #[test]
    fn test_eq_ignore_ascii_case_bytes() {
        const B1: &[u8] = b"Ferris";
        const B2: &[u8] = b"FERRIS";
        const B3: &[u8] = b"FERRI";

        let r1 = eq_ignore_ascii_case!(B1, B2);
        let r2 = eq_ignore_ascii_case!(B1, B3);

        assert!(r1);
        assert!(!r2);
    }

    #[test]
    fn test_eq_ignore_ascii_case_byte_arrays() {
        const A1: &[u8; 6] = b"Ferris";
        const A2: &[u8; 6] = b"FERRIS";
        const A3: &[u8; 5] = b"FERRI";

        let r1 = eq_ignore_ascii_case!(A1, A2);
        let r2 = eq_ignore_ascii_case!(A1, A3);

        assert!(r1);
        assert!(!r2);
    }
}

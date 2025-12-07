pub struct Equal<T1, T2>(pub T1, pub T2);

impl Equal<&[u8], &[u8]> {
    pub const fn const_eval(&self) -> bool {
        crate::bytes::equal(self.0, self.1)
    }
}

impl<const L1: usize, const L2: usize> Equal<&[u8; L1], &[u8; L2]> {
    pub const fn const_eval(&self) -> bool {
        crate::bytes::equal(self.0, self.1)
    }
}

impl Equal<&str, &str> {
    pub const fn const_eval(&self) -> bool {
        crate::str::equal(self.0, self.1)
    }
}

/// Checks that two strings are equal.
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
///
/// # Examples
///
/// ```
/// const A: &str = "hello";
/// const B: &str = "world";
/// const C: &str = "hello";
/// const EQ_AB: bool = const_str::equal!(A, B);
/// const EQ_AC: bool = const_str::equal!(A, C);
/// assert_eq!([EQ_AB, EQ_AC], [false, true]);
///
#[macro_export]
macro_rules! equal {
    ($lhs: expr, $rhs: expr) => {
        $crate::__ctfe::Equal($lhs, $rhs).const_eval()
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_equal_str() {
        const A: &str = "hello";
        const B: &str = "world";
        const C: &str = "hello";
        const D: &str = "";
        const E: &str = "";

        let eq1 = equal!(A, B);
        let eq2 = equal!(A, C);
        let eq3 = equal!(D, E);

        assert!(!eq1);
        assert!(eq2);
        assert!(eq3);
    }

    #[test]
    fn test_equal_bytes() {
        const A: &[u8] = b"hello";
        const B: &[u8] = b"world";
        const C: &[u8] = b"hello";

        let eq1 = equal!(A, B);
        let eq2 = equal!(A, C);

        assert!(!eq1);
        assert!(eq2);
    }

    #[test]
    fn test_equal_byte_arrays() {
        const A: &[u8; 5] = b"hello";
        const B: &[u8; 5] = b"world";
        const C: &[u8; 5] = b"hello";

        let eq1 = equal!(A, B);
        let eq2 = equal!(A, C);

        assert!(!eq1);
        assert!(eq2);
    }
}

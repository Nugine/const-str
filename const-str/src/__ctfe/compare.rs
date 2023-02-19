use core::cmp::Ordering;

pub struct Compare<T1, T2>(pub T1, pub T2);

impl Compare<&[u8], &[u8]> {
    pub const fn const_eval(&self) -> Ordering {
        crate::bytes::compare(self.0, self.1)
    }
}

impl<const L1: usize, const L2: usize> Compare<&[u8; L1], &[u8; L2]> {
    pub const fn const_eval(&self) -> Ordering {
        crate::bytes::compare(self.0, self.1)
    }
}

impl Compare<&str, &str> {
    pub const fn const_eval(&self) -> Ordering {
        crate::str::compare(self.0, self.1)
    }
}

/// Compares two strings lexicographically.
///
/// # Examples
///
/// ```
/// use core::cmp::Ordering;
///
/// const A: &str = "1";
/// const B: &str = "10";
/// const C: &str = "2";
///
/// const ORD: Ordering = const_str::compare!(A, B);
/// assert_eq!(ORD, Ordering::Less);
///
/// assert!(const_str::compare!(<, A, B));
/// assert!(const_str::compare!(<=, A, B));
///
/// assert!(const_str::compare!(>, C, A));
/// assert!(const_str::compare!(>=, C, A));
///
/// assert!(const_str::compare!(==, A, A));
/// ```
///
#[macro_export]
macro_rules! compare {
    (<, $lhs: expr, $rhs: expr) => {{
        use ::core::cmp::Ordering;
        let ordering = $crate::__ctfe::Compare($lhs, $rhs).const_eval();
        matches!(ordering, Ordering::Less)
    }};
    (>, $lhs: expr, $rhs: expr) => {{
        use ::core::cmp::Ordering;
        let ordering = $crate::__ctfe::Compare($lhs, $rhs).const_eval();
        matches!(ordering, Ordering::Greater)
    }};
    (==, $lhs: expr, $rhs: expr) => {{
        use ::core::cmp::Ordering;
        let ordering = $crate::__ctfe::Compare($lhs, $rhs).const_eval();
        matches!(ordering, Ordering::Equal)
    }};
    (<=, $lhs: expr, $rhs: expr) => {{
        use ::core::cmp::Ordering;
        let ordering = $crate::__ctfe::Compare($lhs, $rhs).const_eval();
        matches!(ordering, Ordering::Less | Ordering::Equal)
    }};
    (>=, $lhs: expr, $rhs: expr) => {{
        use ::core::cmp::Ordering;
        let ordering = $crate::__ctfe::Compare($lhs, $rhs).const_eval();
        matches!(ordering, Ordering::Greater | Ordering::Equal)
    }};
    ($lhs: expr, $rhs: expr) => {
        $crate::__ctfe::Compare($lhs, $rhs).const_eval()
    };
}

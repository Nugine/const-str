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
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
///
/// See also [`equal!`](crate::equal).
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

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    #[test]
    fn test_compare_str() {
        const A: &str = "apple";
        const B: &str = "banana";
        const C: &str = "apple";
        
        const ORD1: Ordering = compare!(A, B);
        const ORD2: Ordering = compare!(B, A);
        const ORD3: Ordering = compare!(A, C);
        
        assert_eq!(ORD1, Ordering::Less);
        assert_eq!(ORD2, Ordering::Greater);
        assert_eq!(ORD3, Ordering::Equal);
        
        const LT: bool = compare!(<, A, B);
        const GT: bool = compare!(>, B, A);
        const EQ: bool = compare!(==, A, C);
        const LE1: bool = compare!(<=, A, B);
        const LE2: bool = compare!(<=, A, C);
        const GE1: bool = compare!(>=, B, A);
        const GE2: bool = compare!(>=, A, C);
        
        assert!(LT);
        assert!(GT);
        assert!(EQ);
        assert!(LE1);
        assert!(LE2);
        assert!(GE1);
        assert!(GE2);
    }
    
    #[test]
    fn test_compare_bytes() {
        const A: &[u8] = b"apple";
        const B: &[u8] = b"banana";
        const C: &[u8] = b"apple";
        
        const ORD1: Ordering = compare!(A, B);
        const ORD2: Ordering = compare!(B, A);
        const ORD3: Ordering = compare!(A, C);
        
        assert_eq!(ORD1, Ordering::Less);
        assert_eq!(ORD2, Ordering::Greater);
        assert_eq!(ORD3, Ordering::Equal);
    }
    
    #[test]
    fn test_compare_byte_arrays() {
        const A: &[u8; 5] = b"apple";
        const B: &[u8; 6] = b"banana";
        const C: &[u8; 5] = b"apple";
        
        const ORD1: Ordering = compare!(A, B);
        const ORD2: Ordering = compare!(B, A);
        const ORD3: Ordering = compare!(A, C);
        
        assert_eq!(ORD1, Ordering::Less);
        assert_eq!(ORD2, Ordering::Greater);
        assert_eq!(ORD3, Ordering::Equal);
    }
}

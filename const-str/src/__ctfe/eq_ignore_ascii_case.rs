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

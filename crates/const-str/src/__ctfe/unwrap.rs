pub struct Unwrap<T>(pub T);

impl<T: Copy> Unwrap<Option<T>> {
    pub const fn const_eval(self) -> T {
        match self.0 {
            Some(x) => x,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

/// Unwraps a container, returns the content.
///
/// The input type must be one of
/// + [`Option<T>`], where `T: Copy`.
///
/// The [`Copy`] bound may be relaxed in the future.
///
/// This macro is [const-fn compatible](./index.html#const-fn-compatible).
#[macro_export]
macro_rules! unwrap {
    ($expr: expr) => {{
        $crate::__ctfe::Unwrap($expr).const_eval()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unwrap_some() {
        const X: Option<i32> = Some(42);
        const Y: i32 = unwrap!(X);
        assert_eq!(Y, 42);
        
        const S: Option<&str> = Some("hello");
        const T: &str = unwrap!(S);
        assert_eq!(T, "hello");
        
        const B: Option<bool> = Some(true);
        const C: bool = unwrap!(B);
        assert_eq!(C, true);
    }
}

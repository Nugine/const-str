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
#[macro_export]
macro_rules! unwrap {
    ($expr: expr) => {{
        $crate::__ctfe::Unwrap($expr).const_eval()
    }};
}

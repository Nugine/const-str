pub use const_str_proc_macro::from_utf8;

// -----------------------------------------------------------------------------

/// Converts a byte string literal to a string literal
///
/// # Examples
/// ```
/// let name: &'static str = const_str::from_utf8!(b"file");
/// assert_eq!(name, "file");
/// ```
///
#[cfg_attr(docsrs, doc(cfg(feature = "proc")))]
#[macro_export]
macro_rules! from_utf8 {
    ($s: literal) => {
        $crate::__proc::from_utf8!($s)
    };
}

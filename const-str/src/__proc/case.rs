pub use const_str_proc_macro::convert_case;

/// Converts a string literal to a specified case.
///
/// These variants require the feature `case`.
///
/// + camel
/// + kebab
/// + snake
/// + shouty_snake
/// + shouty_kebab
///
// #[doc = convert_case_doc!() ] // stable since 1.54
#[cfg(not(feature = "case"))]
#[macro_export]
macro_rules! convert_case {
    (lower, $s: literal) => {
        $crate::__proc::to_lowercase!($s)
    };
    (upper, $s: literal) => {
        $crate::__proc::to_uppercase!($s)
    };
}

/// Converts a string literal to a specified case.
///
/// These variants require the feature `case`.
///
/// + camel
/// + kebab
/// + snake
/// + shouty_snake
/// + shouty_kebab
///
/// # Examples
/// ```
/// use const_str::convert_case;
///
/// const S1: &str = convert_case!(lower, "Lower Case");
/// const S2: &str = convert_case!(upper, "Upper Case");
/// # #[cfg(feature = "case")]
/// const S3: &str = convert_case!(camel, "camel case");
/// # #[cfg(feature = "case")]
/// const S4: &str = convert_case!(snake, "snake case");
/// # #[cfg(feature = "case")]
/// const S5: &str = convert_case!(kebab, "kebab case");
/// # #[cfg(feature = "case")]
/// const S6: &str = convert_case!(shouty_snake, "shouty snake case");
/// # #[cfg(feature = "case")]
/// const S7: &str = convert_case!(shouty_kebab, "shouty kebab case");
///
/// assert_eq!(S1, "lower case");
/// assert_eq!(S2, "UPPER CASE");
/// # #[cfg(feature = "case")]
/// assert_eq!(S3, "CamelCase");
/// # #[cfg(feature = "case")]
/// assert_eq!(S4, "snake_case");
/// # #[cfg(feature = "case")]
/// assert_eq!(S5, "kebab-case");
/// # #[cfg(feature = "case")]
/// assert_eq!(S6, "SHOUTY_SNAKE_CASE");
/// # #[cfg(feature = "case")]
/// assert_eq!(S7, "SHOUTY-KEBAB-CASE");
/// ```
#[cfg(feature = "case")]
#[macro_export]
macro_rules! convert_case {
    (lower, $s: literal) => {
        $crate::__proc::convert_case!(lower, $s)
    };
    (upper, $s: literal) => {
        $crate::__proc::convert_case!(upper, $s)
    };
    (camel, $s: literal) => {
        $crate::__proc::convert_case!(camel, $s)
    };
    (snake, $s: literal) => {
        $crate::__proc::convert_case!(snake, $s)
    };
    (kebab, $s: literal) => {
        $crate::__proc::convert_case!(kebab, $s)
    };
    (shouty_snake, $s: literal) => {
        $crate::__proc::convert_case!(shouty_snake, $s)
    };
    (shouty_kebab, $s: literal) => {
        $crate::__proc::convert_case!(shouty_kebab, $s)
    };
}

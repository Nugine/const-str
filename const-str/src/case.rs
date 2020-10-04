/// Converts a string literal to camel case.
///
/// # Examples
/// ```
/// assert_eq!(const_str::to_camel_case!("camel case"), "CamelCase");
/// ```
#[macro_export]
macro_rules! to_camel_case {
    ($str:literal) => {
        $crate::__imp::to_camel_case!($str)
    };
}

/// Converts a string literal to kebab case.
///
/// # Examples
/// ```
/// assert_eq!(const_str::to_kebab_case!("kebab case"), "kebab-case");
/// ```
#[macro_export]
macro_rules! to_kebab_case {
    ($str:literal) => {
        $crate::__imp::to_kebab_case!($str)
    };
}

/// Converts a string literal to snake case.
///
/// # Examples
/// ```
/// assert_eq!(const_str::to_snake_case!("snake case"), "snake_case");
/// ```
#[macro_export]
macro_rules! to_snake_case {
    ($str:literal) => {
        $crate::__imp::to_snake_case!($str)
    };
}

/// Converts a string literal to shouty snake case.
///
/// # Examples
/// ```
/// assert_eq!(const_str::to_shouty_snake_case!("shouty snake case"), "SHOUTY_SNAKE_CASE");
/// ```
#[macro_export]
macro_rules! to_shouty_snake_case {
    ($str:literal) => {
        $crate::__imp::to_shouty_snake_case!($str)
    };
}

/// Converts a string literal to shouty kebab case.
///
/// # Examples
/// ```
/// assert_eq!(const_str::to_shouty_kebab_case!("shouty kebab case"), "SHOUTY-KEBAB-CASE");
/// ```
#[macro_export]
macro_rules! to_shouty_kebab_case {
    ($str:literal) => {
        $crate::__imp::to_shouty_kebab_case!($str)
    };
}

pub use const_str_proc_macro::convert_case;

macro_rules! convert_case_doc {
    () => {
        r#"
Converts a string literal to a specified case.

These variants require the feature `case`.

+ lower_camel
+ upper_camel
+ title
+ train
+ kebab
+ snake
+ shouty_snake
+ shouty_kebab

# Examples
```
use const_str::convert_case;

const S1: &str = convert_case!(lower, "Lower Case");
const S2: &str = convert_case!(upper, "Upper Case");
# #[cfg(feature = "case")]
const S3: &str = convert_case!(lower_camel, "lower camel case");
# #[cfg(feature = "case")]
const S4: &str = convert_case!(upper_camel, "upper camel case");
# #[cfg(feature = "case")]
const S5: &str = convert_case!(title, "title case");
# #[cfg(feature = "case")]
const S6: &str = convert_case!(train, "train case");
# #[cfg(feature = "case")]
const S7: &str = convert_case!(snake, "snake case");
# #[cfg(feature = "case")]
const S8: &str = convert_case!(kebab, "kebab case");
# #[cfg(feature = "case")]
const S9: &str = convert_case!(shouty_snake, "shouty snake case");
# #[cfg(feature = "case")]
const S10: &str = convert_case!(shouty_kebab, "shouty kebab case");

assert_eq!(S1, "lower case");
assert_eq!(S2, "UPPER CASE");
# #[cfg(feature = "case")]
assert_eq!(S3, "lowerCamelCase");
# #[cfg(feature = "case")]
assert_eq!(S4, "UpperCamelCase");
# #[cfg(feature = "case")]
assert_eq!(S5, "Title Case");
# #[cfg(feature = "case")]
assert_eq!(S6, "Train-Case");
# #[cfg(feature = "case")]
assert_eq!(S7, "snake_case");
# #[cfg(feature = "case")]
assert_eq!(S8, "kebab-case");
# #[cfg(feature = "case")]
assert_eq!(S9, "SHOUTY_SNAKE_CASE");
# #[cfg(feature = "case")]
assert_eq!(S10, "SHOUTY-KEBAB-CASE");
```
"#
    };
}

#[doc = convert_case_doc!() ] // stable since 1.54
#[cfg(not(feature = "case"))]
#[macro_export]
macro_rules! convert_case {
    (lower, $s: literal) => {
        $crate::__proc::convert_case!(lower, $s)
    };
    (upper, $s: literal) => {
        $crate::__proc::convert_case!(upper, $s)
    };
}

#[cfg_attr(docsrs, doc(cfg(any(feature = "proc", feature = "case"))))]
#[doc = convert_case_doc!() ] // stable since 1.54
#[cfg(feature = "case")]
#[macro_export]
macro_rules! convert_case {
    (lower, $s: literal) => {
        $crate::__proc::convert_case!(lower, $s)
    };
    (upper, $s: literal) => {
        $crate::__proc::convert_case!(upper, $s)
    };
    (lower_camel, $s: literal) => {
        $crate::__proc::convert_case!(lower_camel, $s)
    };
    (upper_camel, $s: literal) => {
        $crate::__proc::convert_case!(upper_camel, $s)
    };
    (title, $s: literal) => {
        $crate::__proc::convert_case!(title, $s)
    };
    (train, $s: literal) => {
        $crate::__proc::convert_case!(train, $s)
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

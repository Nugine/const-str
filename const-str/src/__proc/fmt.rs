pub use const_str_proc_macro::format;

/// Creates a string slice using interpolation of const expressions.
///
/// The input type must be one of
///
/// + [`&str`]
/// + [`char`]
/// + [`bool`]
/// + [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`]
/// + [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`isize`]
///
/// # Examples
///
/// ```
/// use const_str::format as const_format;
///
/// const PROMPT: &str = "The answer is";
/// const ANSWER: usize = 42;
///
/// const MESSAGE_1: &str = const_format!("{PROMPT} {ANSWER}");
/// const MESSAGE_2: &str = const_format!("{} {}", PROMPT, ANSWER);
/// const MESSAGE_3: &str = const_format!("{0} {1}", PROMPT, ANSWER);
/// const MESSAGE_4: &str = const_format!("{a} {b}", a = PROMPT, b = ANSWER);
/// const MESSAGE_5: &str = const_format!("{} {a}", PROMPT, a = ANSWER);
///
/// assert_eq!(MESSAGE_1, "The answer is 42");
/// assert_eq!(MESSAGE_1, MESSAGE_2);
/// assert_eq!(MESSAGE_1, MESSAGE_3);
/// assert_eq!(MESSAGE_1, MESSAGE_4);
/// assert_eq!(MESSAGE_1, MESSAGE_5);
/// ```
///
#[macro_export]
macro_rules! format {
    ($fmt: literal $($args:tt)*) => {
        $crate::__proc::format!($fmt $($args)*)
    };
}

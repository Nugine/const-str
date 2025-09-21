/// Chains multiple macro calls together.
///
/// `_` is used as a placeholder for the value that is being passed through the chained calls.
///
/// # Examples
///
/// ```
/// use const_str::{chain, concat, replace, split};
///
/// const TOP: &str = "std";
///
/// const PARTS: &[&str] = &chain! {
///    stringify!(std::sync::atomic::Ordering::Relaxed),
///    replace!(_, { concat!(TOP, "::") }, ""),
///    split!(_, "::"),
/// };
///
/// assert_eq!(PARTS, &["sync", "atomic", "Ordering", "Relaxed"]);
/// ```
#[macro_export]
macro_rules! chain {
    ($init:expr, $( $call:ident!($($arg:tt),+), )+) => {
        $crate::__chain_impl!(@chain $init, $( $call!($($arg),+) ),+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __chain_impl {
    (@chain $init:expr, $call:ident!($($arg:tt),+)) => {
        $crate::__chain_impl!(@call $init, $call!($($arg),+))
    };

    (@chain $init:expr, $call:ident!($($arg:tt),+), $($rest:tt)+) => {
        $crate::__chain_impl!(@chain $crate::__chain_impl!(@call $init, $call!($($arg),+)), $($rest)+)
    };

    (@call $e: expr, $call:ident!($($arg:tt),+)) => {
        $call!(
            $(
                $crate::__chain_impl!(@replace $e, $arg)
            ),+
        )
    };

    (@replace $e:expr, _) => {
        $e
    };

    (@replace $e:expr, $tt:tt) => {
        $tt
    };
}

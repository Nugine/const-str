/// Returns a compile-time verified header name string literal.
///
/// # Examples
///
/// ```
/// use http::header::HeaderName;
/// let name = const_str::verified_header_name!("content-md5");
/// assert_eq!(HeaderName::from_static(name).as_str(), "content-md5");
/// ```
///
#[macro_export]
macro_rules! verified_header_name {
    ($name: literal) => {
        $crate::__proc::verified_header_name!($name)
    };
}

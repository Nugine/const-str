# const-str

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][docs-badge]][docs-url]
![CI][ci-badge]

[crates-badge]: https://img.shields.io/crates/v/const-str.svg
[crates-url]: https://crates.io/crates/const-str
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[docs-badge]: https://docs.rs/const-str/badge.svg
[docs-url]: https://docs.rs/const-str/
[ci-badge]: https://github.com/Nugine/const-str/workflows/CI/badge.svg

compile-time string operations

## Examples

```rust
assert_eq!(const_str::to_lowercase!("HELLO"), "hello");

assert_eq!(const_str::to_uppercase!("hello"), "HELLO");

assert_eq!(const_str::replace!("this is old", "old", "new"), "this is new");
```

feature `verify-regex`

```rust
use regex::Regex;
let re = const_str::verified_regex!(r"^\d{4}-\d{2}-\d{2}$");
assert!(Regex::new(re).is_ok());

const_str::regex_assert_match!(r"^\d{4}-\d{2}-\d{2}$", "2014-01-01");
```

feature `verify-http`

```rust
use http::header::HeaderName;
let name = const_str::verified_header_name!("content-md5");
assert_eq!(HeaderName::from_static(name).as_str(), "content-md5");
```

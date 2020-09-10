# const-str

compile-time string operations

## Examples

```rust
assert_eq!(const_str::to_lowercase!("HELLO"), "hello");

assert_eq!(const_str::to_uppercase!("hello"), "HELLO");

assert_eq!(const_str::replace!("this is old", "old", "new"), "this is new");
```

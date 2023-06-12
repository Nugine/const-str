const fn str_clone<'a, const N: usize>(ss: &[&'a str]) -> [&'a str; N] {
    assert!(ss.len() == N);
    let mut buf = [""; N];
    let mut i = 0;
    while i < ss.len() {
        buf[i] = ss[i];
        i += 1;
    }
    buf
}

const fn str_sorted<'a, const N: usize>(ss: &[&'a str]) -> [&'a str; N] {
    let mut buf = str_clone(ss);

    let mut l = N;
    while l > 1 {
        let mut swapped = false;

        let mut i = 0;
        while i < l - 1 {
            let (lhs, rhs) = (buf[i], buf[i + 1]);
            if crate::compare!(>, lhs, rhs) {
                (buf[i], buf[i + 1]) = (rhs, lhs);
                swapped = true;
            }
            i += 1;
        }

        if !swapped {
            break;
        }

        l -= 1;
    }

    buf
}

pub struct Sorted<T>(pub T);

impl<'a> Sorted<&[&'a str]> {
    pub const fn output_len(&self) -> usize {
        self.0.len()
    }

    pub const fn const_eval<const N: usize>(&self) -> [&'a str; N] {
        str_sorted(self.0)
    }
}

impl<'a, const L: usize> Sorted<[&'a str; L]> {
    pub const fn output_len(&self) -> usize {
        L
    }

    pub const fn const_eval(&self) -> [&'a str; L] {
        str_sorted(&self.0)
    }
}

impl<'a, const L: usize> Sorted<&[&'a str; L]> {
    pub const fn output_len(&self) -> usize {
        L
    }

    pub const fn const_eval(&self) -> [&'a str; L] {
        str_sorted(self.0)
    }
}

/// Sorts string slices and returns a new array.
///
/// The input type must be one of:
/// + [`&[&str]`](slice)
/// + [`[&str; N]`](array)
/// + [`&[&str; N]`](array)
///
/// # Examples
///
/// ```rust
/// const SORTED1: &[&str] = &const_str::sorted!(["one", "two", "three"]);
/// assert_eq!(SORTED1, &["one", "three", "two"]);
///
/// const SORTED2: [&str; 3] = const_str::sorted!(["1", "2", "10"]);
/// assert_eq!(SORTED2, ["1", "10", "2"]);
/// ```
///
#[macro_export]
macro_rules! sorted {
    ($s:expr) => {{
        const N: usize = $crate::__ctfe::Sorted($s).output_len();
        const SS: [&str; N] = $crate::__ctfe::Sorted($s).const_eval();
        SS
    }};
}

#[cfg(test)]
mod tests {
    fn std_sorted<'a>(iter: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
        let mut v: Vec<_> = iter.into_iter().collect();
        v.sort_unstable();
        v
    }

    #[test]
    fn test_sorted() {
        macro_rules! testcase {
            ($s:expr) => {
                let const_sorted = sorted!($s);
                let std_sorted = std_sorted($s);
                assert_eq!(const_sorted, &*std_sorted);
            };
        }

        testcase!([]);
        testcase!(["a"]);
        testcase!(["a", "a"]);
        testcase!(["b", "a"]);
        testcase!(["a", "b", "c"]);
        testcase!(["b", "a", "c"]);
        testcase!(["c", "b", "a"]);
        testcase!(["1", "2", "10", "20", "3"]);
    }
}

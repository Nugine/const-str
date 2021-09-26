use const_str::format as const_format;

#[test]
fn test_const_format() {
    {
        const A: usize = 1;
        const X: &str = const_format!("{:?}", A);
        let ans = format!("{:?}", A);
        assert_eq!(X, ans);
    }

    {
        const A: bool = true;
        const B: bool = false;
        const X: &str = const_format!("{1:?} {0:?} {:?}", A, B);
        let ans = format!("{1:?} {0:?} {:?}", A, B);
        assert_eq!(X, ans);
    }

    {
        const A: char = '我';
        const X: &str = const_format!("{a:?} {0}", A, a = A);
        let ans = format!("{a} {0}", A, a = A.escape_default());
        assert_eq!(X, ans);
    }

    {
        const A: &str = "团长\0\t\r\n\"'and希望之花";
        const X: &str = const_format!("{:?}", A);
        let ans = A.escape_default().to_string();
        assert_eq!(X, ans)
    }

    {
        const A: u32 = 42;
        const X: &str = const_format!("{0:x} {0:X} {0:#x} {0:#X} {0:b} {0:#b}", A);
        let ans = format!("{0:x} {0:X} {0:#x} {0:#X} {0:b} {0:#b}", A);
        assert_eq!(X, ans)
    }

    {
        const A: i32 = -42;
        const X: &str = const_format!("{A:x} {A:X} {A:#x} {A:#X} {A:b} {A:#b}");
        let ans = format!("{0:x} {0:X} {0:#x} {0:#X} {0:b} {0:#b}", A);
        assert_eq!(X, ans)
    }
}

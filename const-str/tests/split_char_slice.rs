use const_str::{split, split_inclusive};

#[test]
fn test_split_char_slice_integration() {
    // Test split with char slice
    const TEXT1: &str = "hello,world;foo:bar";
    const DELIMITERS1: &[char] = &[',', ';', ':'];
    const RESULT1: &[&str] = &split!(TEXT1, DELIMITERS1);
    
    // Verify it matches standard library behavior
    let std_result1: Vec<&str> = TEXT1.split(&[',', ';', ':'][..]).collect();
    assert_eq!(RESULT1, std_result1.as_slice());
    assert_eq!(RESULT1, &["hello", "world", "foo", "bar"]);
    
    // Test split_inclusive with char slice
    const TEXT2: &str = "a,b;c:d";
    const DELIMITERS2: &[char] = &[',', ';', ':'];
    const RESULT2: &[&str] = &split_inclusive!(TEXT2, DELIMITERS2);
    
    let std_result2: Vec<&str> = TEXT2.split_inclusive(&[',', ';', ':'][..]).collect();
    assert_eq!(RESULT2, std_result2.as_slice());
    assert_eq!(RESULT2, &["a,", "b;", "c:", "d"]);
    
    // Test with Unicode characters
    const TEXT3: &str = "你好，世界；测试：完成";
    const DELIMITERS3: &[char] = &['，', '；', '：'];
    const RESULT3: &[&str] = &split!(TEXT3, DELIMITERS3);
    
    let std_result3: Vec<&str> = TEXT3.split(&['，', '；', '：'][..]).collect();
    assert_eq!(RESULT3, std_result3.as_slice());
    assert_eq!(RESULT3, &["你好", "世界", "测试", "完成"]);
    
    // Test edge cases
    const TEXT4: &str = "no delimiters here";
    const DELIMITERS4: &[char] = &[',', ';'];
    const RESULT4: &[&str] = &split!(TEXT4, DELIMITERS4);
    
    let std_result4: Vec<&str> = TEXT4.split(&[',', ';'][..]).collect();
    assert_eq!(RESULT4, std_result4.as_slice());
    assert_eq!(RESULT4, &["no delimiters here"]);
    
    // Test empty pattern
    const TEXT5: &str = "test";
    const DELIMITERS5: &[char] = &[];
    const RESULT5: &[&str] = &split!(TEXT5, DELIMITERS5);
    
    let std_result5: Vec<&str> = TEXT5.split(&[][..]).collect();
    assert_eq!(RESULT5, std_result5.as_slice());
    assert_eq!(RESULT5, &["test"]);
    
    // Test single character
    const TEXT6: &str = "a,b,c";
    const DELIMITERS6: &[char] = &[','];
    const RESULT6: &[&str] = &split!(TEXT6, DELIMITERS6);
    
    let std_result6: Vec<&str> = TEXT6.split(&[','][..]).collect();
    assert_eq!(RESULT6, std_result6.as_slice());
    assert_eq!(RESULT6, &["a", "b", "c"]);
}
use const_str::{split, split_inclusive};

// Demonstrate the new split! by char slice functionality

fn main() {
    println!("=== const-str split! by char slice demonstration ===\n");

    // Example 1: Basic usage with multiple delimiters
    const TEXT1: &str = "apple,banana;orange:grape";
    const DELIMS1: &[char] = &[',', ';', ':'];
    const RESULT1: &[&str] = &split!(TEXT1, DELIMS1);
    println!("split!({:?}, {:?})", TEXT1, DELIMS1);
    println!("Result: {:?}\n", RESULT1);

    // Example 2: split_inclusive with multiple delimiters
    const TEXT2: &str = "line1\nline2\rline3\r\n";
    const DELIMS2: &[char] = &['\n', '\r'];
    const RESULT2: &[&str] = &split_inclusive!(TEXT2, DELIMS2);
    println!("split_inclusive!({:?}, {:?})", TEXT2, DELIMS2);
    println!("Result: {:?}\n", RESULT2);

    // Example 3: Unicode characters
    const TEXT3: &str = "你好，世界；测试：完成";
    const DELIMS3: &[char] = &['，', '；', '：'];
    const RESULT3: &[&str] = &split!(TEXT3, DELIMS3);
    println!("split!({:?}, {:?})", TEXT3, DELIMS3);
    println!("Result: {:?}\n", RESULT3);

    // Example 4: Array syntax (automatically converted to slice)
    const TEXT4: &str = "a|b&c";
    const RESULT4: &[&str] = &split!(TEXT4, &['|', '&']);
    println!("split!({:?}, &['|', '&'])", TEXT4);
    println!("Result: {:?}\n", RESULT4);

    // Verify compatibility with standard library
    println!("=== Verification against standard library ===");
    let std_result1: Vec<&str> = TEXT1.split(&DELIMS1[..]).collect();
    println!("Standard library result: {:?}", std_result1);
    println!("Results match: {}", RESULT1 == std_result1.as_slice());

    let std_result2: Vec<&str> = TEXT2.split_inclusive(&DELIMS2[..]).collect();
    println!("Standard library inclusive result: {:?}", std_result2);
    println!("Results match: {}", RESULT2 == std_result2.as_slice());

    println!("\n✅ All examples completed successfully!");
}
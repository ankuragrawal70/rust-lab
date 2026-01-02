/// ============================================================================
/// OPTION & RESULT MODULE - Rust's Safe Error Handling
/// ============================================================================

/// ============================================================================
/// 13. OPTION<T> TYPE - Rust Without Null
/// ============================================================================
/// Key Concepts:
/// - Rust has no null! Instead, use `Option<T>`
/// - `Some(T)` → Contains a value of type T
/// - `None`    → Represents absence of value
///
/// Why Option?
/// - Compiler forces you to handle the "no value" case
/// - No null pointer exceptions at runtime
/// - Makes "might not exist" explicit in the type
///
/// Common Methods:
/// - `.unwrap()`      → Get value or panic (dangerous!)
/// - `.unwrap_or(x)`  → Get value or use default x
/// - `.map(f)`        → Transform Some(v) to Some(f(v))
/// - `.and_then(f)`   → Chain operations that return Option
/// - `.is_some()`     → Returns true if Some
/// - `.is_none()`     → Returns true if None
/// ============================================================================
pub fn learn_option_type() {
    println!("\n============================================================");
    println!("📘 LESSON 13: Option<T> Type");
    println!("============================================================\n");

    // Basic Option usage with match
    println!("--- Basic Option with Match ---");
    basic_option_examples();

    // Option combinators
    println!("\n--- Option Combinators ---");
    option_combinator_examples();

    // Real-world example: safe division
    println!("\n--- Real-World: Safe Division ---");
    safe_division_examples();
}

fn basic_option_examples() {
    // Function that might not return a value
    fn find_first_even(numbers: &[i32]) -> Option<i32> {
        for &n in numbers {
            if n % 2 == 0 {
                return Some(n);
            }
        }
        None
    }

    let nums1 = [1, 3, 5, 6, 7];
    let nums2 = [1, 3, 5, 7, 9];

    // Handle with match
    match find_first_even(&nums1) {
        Some(n) => println!("Found even number: {}", n),
        None => println!("No even number found"),
    }

    match find_first_even(&nums2) {
        Some(n) => println!("Found even number: {}", n),
        None => println!("No even number found"),
    }
}

fn option_combinator_examples() {
    let some_value: Option<i32> = Some(5);
    let no_value: Option<i32> = None;

    // .unwrap_or() - provide default
    println!("some_value.unwrap_or(0) = {}", some_value.unwrap_or(0));
    println!("no_value.unwrap_or(0) = {}", no_value.unwrap_or(0));

    // .map() - transform the value if present
    let doubled = some_value.map(|x| x * 2);
    let doubled_none = no_value.map(|x| x * 2);
    println!("some_value.map(|x| x * 2) = {:?}", doubled);
    println!("no_value.map(|x| x * 2) = {:?}", doubled_none);

    // Chaining with .map()
    let result = Some(10)
        .map(|x| x * 2)      // Some(20)
        .map(|x| x + 5)      // Some(25)
        .map(|x| x.to_string()); // Some("25")
    println!("Chained maps: {:?}", result);

    // .and_then() for operations that return Option
    fn half_if_even(n: i32) -> Option<i32> {
        if n % 2 == 0 {
            Some(n / 2)
        } else {
            None
        }
    }

    let result1 = Some(10).and_then(half_if_even); // Some(5)
    let result2 = Some(11).and_then(half_if_even); // None (11 is odd)
    let result3 = None.and_then(half_if_even);     // None

    println!("Some(10).and_then(half_if_even) = {:?}", result1);
    println!("Some(11).and_then(half_if_even) = {:?}", result2);
    println!("None.and_then(half_if_even) = {:?}", result3);
}

fn safe_division_examples() {
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    // Safe handling
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Some(v) => println!("10.0 / 2.0 = {}", v),
        None => println!("Cannot divide by zero!"),
    }

    match result2 {
        Some(v) => println!("10.0 / 0.0 = {}", v),
        None => println!("Cannot divide by zero!"),
    }

    // Using combinators for pipelines
    let doubled = divide(10.0, 2.0)
        .map(|v| v * 2.0)
        .unwrap_or(0.0);
    println!("(10.0 / 2.0) * 2 = {}", doubled);
}

/// ============================================================================
/// 14. RESULT<T, E> TYPE - Robust Error Handling
/// ============================================================================
/// Key Concepts:
/// - `Ok(T)`  → Success, contains value of type T
/// - `Err(E)` → Failure, contains error of type E
///
/// Why Result?
/// - Makes errors explicit in the type system
/// - Forces handling of error cases
/// - Enables elegant error propagation with `?`
///
/// Common Methods:
/// - `.unwrap()`      → Get value or panic
/// - `.expect(msg)`   → Get value or panic with message
/// - `.unwrap_or(x)`  → Get value or use default
/// - `.map(f)`        → Transform Ok value
/// - `.map_err(f)`    → Transform Err value
/// - `.and_then(f)`   → Chain operations returning Result
/// - `?`              → Propagate error to caller
/// ============================================================================
pub fn learn_error_handling_with_result() {
    println!("\n============================================================");
    println!("📘 LESSON 14: Result<T, E> Type");
    println!("============================================================\n");

    // Basic Result usage
    println!("--- Basic Result with Match ---");
    basic_result_examples();

    // Result combinators
    println!("\n--- Result Combinators ---");
    result_combinator_examples();

    // Error propagation with ?
    println!("\n--- Error Propagation with ? ---");
    error_propagation_examples();
}

fn basic_result_examples() {
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    // Handle with match
    match divide(10, 2) {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10, 0) {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

fn result_combinator_examples() {
    fn parse_and_double(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|_| format!("Failed to parse '{}'", s))
            .map(|n| n * 2)
    }

    println!("parse_and_double(\"5\") = {:?}", parse_and_double("5"));
    println!("parse_and_double(\"abc\") = {:?}", parse_and_double("abc"));

    // Chaining with and_then
    fn safe_sqrt(n: f64) -> Result<f64, String> {
        if n < 0.0 {
            Err(String::from("Cannot take sqrt of negative"))
        } else {
            Ok(n.sqrt())
        }
    }

    fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    // Chain: divide then sqrt
    let result = safe_divide(16.0, 2.0).and_then(safe_sqrt);
    println!("safe_divide(16, 2).and_then(safe_sqrt) = {:?}", result);

    let result2 = safe_divide(16.0, 0.0).and_then(safe_sqrt);
    println!("safe_divide(16, 0).and_then(safe_sqrt) = {:?}", result2);
}

fn error_propagation_examples() {
    // The ? operator propagates errors automatically
    fn process_number(s: &str) -> Result<i32, String> {
        let n: i32 = s
            .parse()
            .map_err(|_| format!("Invalid number: {}", s))?;

        if n < 0 {
            return Err(String::from("Number must be positive"));
        }

        Ok(n * 10)
    }

    println!("process_number(\"5\") = {:?}", process_number("5"));
    println!("process_number(\"-3\") = {:?}", process_number("-3"));
    println!("process_number(\"abc\") = {:?}", process_number("abc"));

    // Pipeline with multiple ? operations
    fn complex_pipeline(input: &str) -> Result<String, String> {
        let n: i32 = input
            .parse()
            .map_err(|_| "Parse failed")?;

        let doubled = n.checked_mul(2).ok_or("Overflow")?;

        let sqrt = if doubled >= 0 {
            Ok((doubled as f64).sqrt())
        } else {
            Err("Negative number")
        }?;

        Ok(format!("Result: {:.2}", sqrt))
    }

    println!("\nComplex pipeline:");
    println!("  \"8\" → {:?}", complex_pipeline("8"));
    println!("  \"abc\" → {:?}", complex_pipeline("abc"));
}

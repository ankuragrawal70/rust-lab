// ============================================================
// ERROR HANDLING - The Rust Way
// ============================================================
// Rust doesn't have exceptions. Instead, it has:
// - Result<T, E> for recoverable errors
// - panic! for unrecoverable errors
//
// This makes error handling EXPLICIT and impossible to ignore.

use std::fs::File;
use std::io::{self, Read};

pub fn learn_error_handling() {
    println!("\n============================================================");
    println!("  ERROR HANDLING - The Rust Way");
    println!("============================================================\n");

    part1_panic_vs_result();
    part2_result_basics();
    part3_question_mark_operator();
    part4_custom_error_types();
    part5_error_conversion();
    part6_when_to_panic();
    part7_practical_patterns();
}

// ============================================================
// PART 1: PANIC VS RESULT
// ============================================================

fn part1_panic_vs_result() {
    println!("--- PART 1: Panic vs Result ---\n");

    // ===== PANIC - UNRECOVERABLE ERRORS =====
    // Use when your program cannot continue.
    // Examples: Bug in code, corrupted state, violated invariants

    // panic!("Something went terribly wrong!");  // Would crash here

    // Common ways to panic:
    // 1. panic! macro
    // panic!("explicit panic");

    // 2. unwrap() on None or Err
    // let x: Option<i32> = None;
    // x.unwrap();  // panics!

    // 3. Array out of bounds
    // let arr = [1, 2, 3];
    // arr[10];  // panics!

    // 4. assert! macros
    // assert!(false);  // panics!

    println!("Panic scenarios (not executing to avoid crash):");
    println!("  panic!(\"message\")");
    println!("  None.unwrap()");
    println!("  vec![1,2,3][10]");
    println!("  assert!(false)");

    // ===== RESULT - RECOVERABLE ERRORS =====
    // Use for operations that might fail.
    // Examples: File I/O, network, parsing, validation

    // Result<T, E> has two variants:
    // - Ok(T): Success, contains the value
    // - Err(E): Failure, contains the error

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // ===== THE KEY INSIGHT =====
    println!("\n--- When to use which ---");
    println!("Panic: Programming bugs, unrecoverable state");
    println!("Result: Expected failures that caller should handle");
}

// ============================================================
// PART 2: RESULT BASICS
// ============================================================

fn part2_result_basics() {
    println!("\n--- PART 2: Result Basics ---\n");

    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }

    // ===== HANDLING WITH MATCH =====
    let input = "42";
    match parse_number(input) {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }

    // ===== HANDLING WITH IF LET =====
    if let Ok(n) = parse_number("123") {
        println!("Got number: {}", n);
    }

    // ===== unwrap() - PANIC ON ERROR =====
    let n = parse_number("42").unwrap();
    println!("Unwrapped: {}", n);
    // parse_number("not a number").unwrap();  // Would panic!

    // ===== expect() - PANIC WITH CUSTOM MESSAGE =====
    let n = parse_number("42").expect("Failed to parse");
    println!("Expected: {}", n);

    // ===== unwrap_or() - DEFAULT ON ERROR =====
    let n = parse_number("bad").unwrap_or(0);
    println!("Unwrap or default: {}", n);

    // ===== unwrap_or_else() - COMPUTE DEFAULT =====
    let n = parse_number("bad").unwrap_or_else(|e| {
        println!("Parse error: {}, using default", e);
        -1
    });
    println!("Unwrap or else: {}", n);

    // ===== unwrap_or_default() - TYPE'S DEFAULT =====
    let n: i32 = parse_number("bad").unwrap_or_default();
    println!("Unwrap or default: {}", n);  // 0 (i32's default)

    // ===== ok() - CONVERT TO OPTION =====
    let maybe: Option<i32> = parse_number("42").ok();
    println!("As Option: {:?}", maybe);

    // ===== is_ok() / is_err() - CHECK WITHOUT CONSUMING =====
    let result = parse_number("42");
    if result.is_ok() {
        println!("It's OK!");
    }

    // ===== map() - TRANSFORM SUCCESS VALUE =====
    let doubled = parse_number("21").map(|n| n * 2);
    println!("Doubled: {:?}", doubled);

    // ===== map_err() - TRANSFORM ERROR VALUE =====
    let result = parse_number("bad")
        .map_err(|e| format!("Custom error: {}", e));
    println!("Mapped error: {:?}", result);

    // ===== and_then() - CHAIN OPERATIONS =====
    fn double_positive(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| e.to_string())
            .and_then(|n| {
                if n > 0 {
                    Ok(n * 2)
                } else {
                    Err("Number must be positive".to_string())
                }
            })
    }
    println!("Chained (42): {:?}", double_positive("42"));
    println!("Chained (-5): {:?}", double_positive("-5"));
}

// ============================================================
// PART 3: THE ? OPERATOR
// ============================================================

fn part3_question_mark_operator() {
    println!("\n--- PART 3: The ? Operator ---\n");

    // The ? operator is syntactic sugar for propagating errors.
    // It returns early if Err, otherwise unwraps Ok.

    // ===== WITHOUT ? OPERATOR =====
    fn read_file_verbose(path: &str) -> Result<String, io::Error> {
        let file_result = File::open(path);
        let mut file = match file_result {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Ok(contents),
            Err(e) => Err(e),
        }
    }

    // ===== WITH ? OPERATOR =====
    fn read_file_concise(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;  // Returns Err early if fails
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;  // Same here
        Ok(contents)
    }

    // Even more concise with chaining:
    fn read_file_chain(path: &str) -> Result<String, io::Error> {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    // Or use std::fs::read_to_string which does all this:
    // std::fs::read_to_string(path)

    // ===== EXAMPLE USAGE =====
    match read_file_concise("/nonexistent/file.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // ===== ? WITH OPTION =====
    fn get_first_char(s: &str) -> Option<char> {
        let first = s.chars().next()?;  // Returns None early if empty
        Some(first.to_ascii_uppercase())
    }

    println!("First char of 'hello': {:?}", get_first_char("hello"));
    println!("First char of '': {:?}", get_first_char(""));

    // ===== MULTIPLE ERROR TYPES =====
    // When using ? with different error types, you need conversion.
    // Options:
    // 1. Use Box<dyn Error> (simplest)
    // 2. Create custom error enum
    // 3. Use anyhow or thiserror crates

    fn combined_operation() -> Result<i32, Box<dyn std::error::Error>> {
        let _file = File::open("Cargo.toml")?;  // io::Error
        let num: i32 = "42".parse()?;           // ParseIntError
        Ok(num)
    }

    match combined_operation() {
        Ok(n) => println!("Combined result: {}", n),
        Err(e) => println!("Combined error: {}", e),
    }

    println!("\n--- ? Operator Summary ---");
    println!("value?  is equivalent to:");
    println!("  match value {{");
    println!("      Ok(v) => v,");
    println!("      Err(e) => return Err(e.into()),");
    println!("  }}");
}

// ============================================================
// PART 4: CUSTOM ERROR TYPES
// ============================================================

fn part4_custom_error_types() {
    println!("\n--- PART 4: Custom Error Types ---\n");

    // ===== SIMPLE CUSTOM ERROR =====
    #[derive(Debug)]
    struct ValidationError {
        message: String,
        field: String,
    }

    impl std::fmt::Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Validation error in '{}': {}", self.field, self.message)
        }
    }

    impl std::error::Error for ValidationError {}

    fn validate_username(name: &str) -> std::result::Result<(), ValidationError> {
        if name.len() < 3 {
            return Err(ValidationError {
                message: "Too short (min 3 chars)".to_string(),
                field: "username".to_string(),
            });
        }
        if name.len() > 20 {
            return Err(ValidationError {
                message: "Too long (max 20 chars)".to_string(),
                field: "username".to_string(),
            });
        }
        Ok(())
    }

    match validate_username("ab") {
        Ok(()) => println!("Username valid"),
        Err(e) => println!("Error: {}", e),
    }

    // ===== ERROR ENUM FOR MULTIPLE ERROR TYPES =====
    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Parse(std::num::ParseIntError),
        Custom(String),
    }

    impl std::fmt::Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                AppError::Io(e) => write!(f, "IO error: {}", e),
                AppError::Parse(e) => write!(f, "Parse error: {}", e),
                AppError::Custom(msg) => write!(f, "Error: {}", msg),
            }
        }
    }

    impl std::error::Error for AppError {}

    // Implement From for automatic conversion with ?
    impl From<io::Error> for AppError {
        fn from(err: io::Error) -> AppError {
            AppError::Io(err)
        }
    }

    impl From<std::num::ParseIntError> for AppError {
        fn from(err: std::num::ParseIntError) -> AppError {
            AppError::Parse(err)
        }
    }

    fn do_stuff() -> std::result::Result<i32, AppError> {
        // Both these can use ? because we implemented From
        let _file = File::open("Cargo.toml")?;  // Converts io::Error
        let num: i32 = "42".parse()?;            // Converts ParseIntError
        Ok(num)
    }

    match do_stuff() {
        Ok(n) => println!("Success: {}", n),
        Err(e) => println!("Failed: {}", e),
    }

    // ===== TYPE ALIAS FOR CONVENIENCE =====
    type Result<T> = std::result::Result<T, AppError>;

    fn another_function() -> Result<String> {
        Ok(String::from("Success"))
    }

    println!("Type alias result: {:?}", another_function());
}

// ============================================================
// PART 5: ERROR CONVERSION
// ============================================================

fn part5_error_conversion() {
    println!("\n--- PART 5: Error Conversion ---\n");

    // ===== Box<dyn Error> - EASIEST APPROACH =====
    fn easy_errors() -> Result<(), Box<dyn std::error::Error>> {
        let _file = File::open("Cargo.toml")?;  // io::Error
        let _num: i32 = "42".parse()?;          // ParseIntError
        Ok(())
    }

    if let Err(e) = easy_errors() {
        println!("Error: {}", e);
    }

    // ===== map_err() - MANUAL CONVERSION =====
    fn manual_conversion() -> Result<i32, String> {
        let num: i32 = "42"
            .parse()
            .map_err(|e| format!("Parse failed: {}", e))?;
        Ok(num)
    }

    println!("Manual: {:?}", manual_conversion());

    // ===== ok_or() - CONVERT OPTION TO RESULT =====
    fn find_value(key: &str) -> Option<i32> {
        if key == "magic" { Some(42) } else { None }
    }

    let result: Result<i32, &str> = find_value("missing").ok_or("Key not found");
    println!("ok_or: {:?}", result);

    // ===== ok_or_else() - LAZY ERROR CREATION =====
    let result = find_value("missing")
        .ok_or_else(|| format!("Key '{}' not found", "missing"));
    println!("ok_or_else: {:?}", result);

    // ===== THISERROR CRATE (RECOMMENDED) =====
    // In real projects, use the `thiserror` crate:
    //
    // use thiserror::Error;
    //
    // #[derive(Error, Debug)]
    // enum AppError {
    //     #[error("IO error: {0}")]
    //     Io(#[from] io::Error),
    //     
    //     #[error("Parse error: {0}")]
    //     Parse(#[from] std::num::ParseIntError),
    //     
    //     #[error("Custom error: {message}")]
    //     Custom { message: String },
    // }

    // ===== ANYHOW CRATE (FOR APPLICATIONS) =====
    // For applications (not libraries), `anyhow` is great:
    //
    // use anyhow::{Context, Result};
    //
    // fn main() -> Result<()> {
    //     let file = File::open("config.toml")
    //         .context("Failed to open config file")?;
    //     Ok(())
    // }

    println!("\n--- Recommended Crates ---");
    println!("Libraries: thiserror (derive macros for Error trait)");
    println!("Applications: anyhow (flexible error handling + context)");
}

// ============================================================
// PART 6: WHEN TO PANIC
// ============================================================

fn part6_when_to_panic() {
    println!("\n--- PART 6: When to Panic ---\n");

    // ===== PANIC IS APPROPRIATE WHEN =====
    println!("Panic is appropriate when:");
    println!("  1. Programming bugs (logic errors that shouldn't happen)");
    println!("  2. Violated invariants (impossible state)");
    println!("  3. In tests");
    println!("  4. Examples and prototypes");
    println!("  5. When there's truly no way to recover");

    // ===== EXAMPLES OF APPROPRIATE PANIC =====

    // 1. Unreachable code
    fn process(value: i32) -> i32 {
        if value > 0 { value * 2 }
        else if value < 0 { value / 2 }
        else { 0 }
        // No panic needed - all cases covered
    }
    println!("Process: {}", process(5));

    // 2. Assert invariants
    fn set_percentage(value: i32) {
        assert!(value >= 0 && value <= 100, "Percentage must be 0-100");
        println!("Setting percentage to {}", value);
    }
    set_percentage(50);
    // set_percentage(150);  // Would panic

    // 3. Unwrap in tests
    #[cfg(test)]
    fn test_example() {
        let result = "42".parse::<i32>().unwrap();  // OK in tests
        assert_eq!(result, 42);
    }

    // ===== USE RESULT INSTEAD OF PANIC WHEN =====
    println!("\nUse Result when:");
    println!("  1. Errors are expected (file not found, network timeout)");
    println!("  2. Caller should decide how to handle");
    println!("  3. You're writing a library");
    println!("  4. Recovery is possible");

    // ===== UNWRAP VS EXPECT =====
    println!("\nunwrap() vs expect():");
    println!("  unwrap(): \"panicked at 'called unwrap on None'\"");
    println!("  expect(): \"panicked at 'Config file should exist'\"");
    println!("  → Always prefer expect() with descriptive message");
}

// ============================================================
// PART 7: PRACTICAL PATTERNS
// ============================================================

fn part7_practical_patterns() {
    println!("\n--- PART 7: Practical Patterns ---\n");

    // ===== PATTERN 1: Early return on error =====
    fn process_input(input: &str) -> Result<i32, String> {
        if input.is_empty() {
            return Err("Input cannot be empty".to_string());
        }
        
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("Input cannot be whitespace only".to_string());
        }
        
        trimmed.parse::<i32>()
            .map_err(|e| format!("Parse error: {}", e))
    }

    println!("Process '  42  ': {:?}", process_input("  42  "));
    println!("Process '': {:?}", process_input(""));

    // ===== PATTERN 2: Collect Results =====
    let inputs = vec!["1", "2", "three", "4", "5"];
    
    // Stop at first error
    let results: Result<Vec<i32>, _> = inputs
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Collect (stop on error): {:?}", results);

    // Collect all successes, ignore errors
    let successes: Vec<i32> = inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Filter successes: {:?}", successes);

    // Partition into successes and failures
    let (oks, errs): (Vec<_>, Vec<_>) = inputs
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Partition - Oks: {}, Errs: {}", oks.len(), errs.len());

    // ===== PATTERN 3: Builder with validation =====
    struct ConfigBuilder {
        port: Option<u16>,
        host: Option<String>,
    }

    struct Config {
        port: u16,
        host: String,
    }

    impl ConfigBuilder {
        fn new() -> Self {
            ConfigBuilder { port: None, host: None }
        }

        fn port(mut self, port: u16) -> Self {
            self.port = Some(port);
            self
        }

        fn host(mut self, host: &str) -> Self {
            self.host = Some(host.to_string());
            self
        }

        fn build(self) -> Result<Config, String> {
            let port = self.port.ok_or("Port is required")?;
            let host = self.host.ok_or("Host is required")?;
            
            if port == 0 {
                return Err("Port cannot be 0".to_string());
            }
            
            Ok(Config { port, host })
        }
    }

    let config = ConfigBuilder::new()
        .port(8080)
        .host("localhost")
        .build();
    
    match config {
        Ok(c) => println!("Config: {}:{}", c.host, c.port),
        Err(e) => println!("Config error: {}", e),
    }

    // ===== PATTERN 4: Fallback chain =====
    fn load_config() -> String {
        std::env::var("APP_CONFIG")
            .or_else(|_| std::fs::read_to_string("config.toml"))
            .unwrap_or_else(|_| String::from("default config"))
    }
    
    println!("Config: {}", load_config());

    // ===== PATTERN 5: Log and continue =====
    let values = vec!["1", "bad", "3", "worse", "5"];
    
    let results: Vec<i32> = values
        .iter()
        .filter_map(|s| {
            match s.parse::<i32>() {
                Ok(n) => Some(n),
                Err(e) => {
                    eprintln!("Warning: '{}' - {}", s, e);
                    None
                }
            }
        })
        .collect();
    
    println!("Parsed with logging: {:?}", results);
}

// ============================================================
// ERROR HANDLING CHEAT SHEET
// ============================================================
//
// RESULT METHODS:
//   unwrap()           → Panic on Err, return Ok value
//   expect("msg")      → Panic with message on Err
//   unwrap_or(default) → Default value on Err
//   unwrap_or_else(f)  → Compute default on Err
//   unwrap_or_default()→ Use Default trait
//   ok()               → Convert to Option<T>
//   is_ok() / is_err() → Boolean check
//   map(f)             → Transform Ok value
//   map_err(f)         → Transform Err value
//   and_then(f)        → Chain Result operations
//
// ? OPERATOR:
//   file.read()?       → Return Err early, unwrap Ok
//   Works in functions returning Result or Option
//
// ERROR TYPES:
//   Box<dyn Error>     → Easy, accepts any error
//   Custom enum        → Full control, type-safe
//   thiserror crate    → Derive Error trait
//   anyhow crate       → Flexible, with context
//
// WHEN TO PANIC:
//   - Programming bugs
//   - Violated invariants
//   - Tests and prototypes
//   - No recovery possible
//
// WHEN TO USE RESULT:
//   - Expected failures
//   - Caller decides handling
//   - Library code
//   - Recovery is possible
//
// ============================================================

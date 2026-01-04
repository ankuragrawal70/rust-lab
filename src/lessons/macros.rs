// ============================================================
// MACROS - Compile-Time Code Generation
// ============================================================
// Macros are code that writes code (metaprogramming).
// They run at COMPILE TIME, not runtime.
//
// Why macros exist:
// - Reduce boilerplate code
// - Accept variable number of arguments
// - Generate repetitive implementations
// - Create domain-specific languages (DSLs)
//
// Two types in Rust:
// 1. Declarative macros (macro_rules!) - Pattern matching
// 2. Procedural macros - Code manipulation (advanced)

pub fn learn_macros() {
    println!("\n============================================================");
    println!("  MACROS - Compile-Time Code Generation");
    println!("============================================================\n");

    part1_macros_vs_functions();
    part2_using_builtin_macros();
    part3_derive_macros();
    part4_macro_rules_basics();
    part5_macro_patterns();
    part6_practical_macros();
    part7_when_to_use_macros();
}

// ============================================================
// PART 1: MACROS VS FUNCTIONS
// ============================================================

fn part1_macros_vs_functions() {
    println!("--- PART 1: Macros vs Functions ---\n");

    // ===== HOW TO SPOT A MACRO =====
    // Macros always end with ! (bang)
    
    println!("This is a macro!");           // println! macro
    let v = vec![1, 2, 3];                  // vec! macro
    let s = format!("Hello {}", "world");   // format! macro
    
    println!("vec! created: {:?}", v);
    println!("format! created: {}", s);

    // ===== KEY DIFFERENCES =====
    println!("\n--- Macros vs Functions ---");
    println!("
    | Feature              | Function          | Macro                |
    |----------------------|-------------------|----------------------|
    | When runs            | Runtime           | Compile time         |
    | Arguments            | Fixed count       | Variable count       |
    | Type checking        | On arguments      | On generated code    |
    | Can generate code    | No                | Yes                  |
    | Syntax               | fn name()         | macro_rules! name    |
    | Call syntax          | name()            | name!()              |
    | Hygiene              | N/A               | Hygienic by default  |
    ");

    // ===== WHY USE MACROS? =====
    
    // 1. Variable arguments - functions can't do this!
    println!("One: {}", 1);
    println!("Two: {} {}", 1, 2);
    println!("Three: {} {} {}", 1, 2, 3);
    // A single function can't accept 1, 2, or 3 arguments!

    // 2. Code generation
    // vec![1, 2, 3] expands to something like:
    // {
    //     let mut temp = Vec::new();
    //     temp.push(1);
    //     temp.push(2);
    //     temp.push(3);
    //     temp
    // }

    // 3. Compile-time computation
    // assert_eq!(2 + 2, 4);  // Error message includes file:line

    println!("\nMacros are powerful but add complexity.");
    println!("Rule: Use functions first, macros only when needed.");
}

// ============================================================
// PART 2: USING BUILT-IN MACROS
// ============================================================

fn part2_using_builtin_macros() {
    println!("\n--- PART 2: Built-in Macros ---\n");

    // ===== PRINTING MACROS =====
    print!("No newline ");
    println!("With newline");
    
    // Debug printing
    let value = vec![1, 2, 3];
    println!("Normal: {:?}", value);
    println!("Pretty: {:#?}", value);
    
    // Print to stderr
    eprintln!("This goes to stderr");

    // ===== FORMATTING MACROS =====
    let s = format!("Hello, {}!", "Rust");
    println!("format!: {}", s);

    // Format specifiers
    let num = 42;
    println!("Decimal: {}", num);
    println!("Binary: {:b}", num);
    println!("Hex: {:x}", num);
    println!("Octal: {:o}", num);
    println!("Padded: {:05}", num);  // 00042
    println!("Left align: |{:<10}|", num);
    println!("Right align: |{:>10}|", num);
    println!("Center: |{:^10}|", num);

    // ===== VECTOR MACRO =====
    let v1: Vec<i32> = vec![];           // Empty
    let v2 = vec![0; 5];                 // [0, 0, 0, 0, 0]
    let v3 = vec![1, 2, 3];              // [1, 2, 3]
    println!("vec![]: {:?}", v1);
    println!("vec![0; 5]: {:?}", v2);
    println!("vec![1,2,3]: {:?}", v3);

    // ===== ASSERTION MACROS =====
    assert!(true, "This won't panic");
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
    
    // debug_assert! only runs in debug builds
    debug_assert!(true);
    debug_assert_eq!(1, 1);
    
    println!("All assertions passed!");

    // ===== PANIC MACRO =====
    // panic!("Something went wrong!");  // Would crash
    
    // Unreachable code marker
    fn check_value(x: i32) -> &'static str {
        match x {
            0 => "zero",
            1 => "one",
            _ => "other",
            // unreachable!() could go here if we knew x is always 0 or 1
        }
    }
    println!("check_value(0): {}", check_value(0));

    // ===== TODO AND UNIMPLEMENTED =====
    fn work_in_progress() {
        // todo!("Implement this later");  // Panics with message
    }

    fn not_yet_done() {
        // unimplemented!("This feature isn't ready");  // Panics
    }
    
    // These are useful placeholders during development
    let _ = work_in_progress;
    let _ = not_yet_done;

    // ===== COMPILE-TIME MACROS =====
    println!("\nCompile-time information:");
    println!("  file!(): {}", file!());
    println!("  line!(): {}", line!());
    println!("  column!(): {}", column!());
    println!("  module_path!(): {}", module_path!());

    // Include file contents at compile time
    // let contents = include_str!("../Cargo.toml");
    // let bytes = include_bytes!("image.png");

    // Compile-time environment variable
    // let version = env!("CARGO_PKG_VERSION");

    // ===== DBG! MACRO =====
    let x = 5;
    let y = dbg!(x * 2);  // Prints: [file:line] x * 2 = 10
    println!("y = {}", y);

    // dbg! returns the value, so you can chain it
    let result = dbg!(dbg!(2) + dbg!(3));  // Shows each step
    println!("Result: {}", result);
}

// ============================================================
// PART 3: DERIVE MACROS
// ============================================================

fn part3_derive_macros() {
    println!("\n--- PART 3: Derive Macros ---\n");

    // Derive macros automatically implement traits for your types.
    // They're the most commonly used "procedural" macros.

    // ===== COMMON DERIVE TRAITS =====

    #[derive(Debug)]  // Enables {:?} formatting
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 20 };
    println!("Debug: {:?}", p);
    println!("Pretty: {:#?}", p);

    // ===== MULTIPLE DERIVES =====
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    let c1 = Color { r: 255, g: 0, b: 0 };
    let c2 = c1;        // Copy (no move!)
    let c3 = c1.clone(); // Clone
    
    println!("c1 == c2: {}", c1 == c2);  // PartialEq
    println!("Default color: {:?}", Color::default());  // Default

    // Use in HashSet/HashMap (requires Hash + Eq)
    use std::collections::HashSet;
    let mut colors = HashSet::new();
    colors.insert(c1);
    colors.insert(c2);  // Duplicate, won't be added
    println!("Unique colors: {}", colors.len());

    // ===== DERIVE TRAIT REQUIREMENTS =====
    println!("\n--- What each derive needs ---");
    println!("
    | Trait      | Requirement                    | Use Case                |
    |------------|--------------------------------|-------------------------|
    | Debug      | All fields implement Debug     | Debugging, {{:?}}       |
    | Clone      | All fields implement Clone     | Deep copy               |
    | Copy       | All fields are Copy, need Clone| Implicit copy           |
    | PartialEq  | All fields impl PartialEq      | == comparison           |
    | Eq         | Need PartialEq, all values eq  | Hash maps/sets          |
    | PartialOrd | Need PartialEq                 | <, >, <=, >= comparison |
    | Ord        | Need Eq + PartialOrd           | Sorting                 |
    | Hash       | All fields implement Hash      | HashMap/HashSet keys    |
    | Default    | All fields implement Default   | Default::default()      |
    ");

    // ===== DERIVE FOR ENUMS =====
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    println!("Direction: {:?}", dir);
    println!("Same? {}", dir == Direction::North);

    // ===== WHEN DERIVE DOESN'T WORK =====
    // Sometimes you need manual implementation
    
    struct CustomDebug {
        password: String,
        username: String,
    }

    // Manual Debug to hide password
    impl std::fmt::Debug for CustomDebug {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_struct("CustomDebug")
                .field("username", &self.username)
                .field("password", &"[REDACTED]")
                .finish()
        }
    }

    let user = CustomDebug {
        username: String::from("alice"),
        password: String::from("secret123"),
    };
    println!("User: {:?}", user);  // Password hidden!

    // ===== POPULAR EXTERNAL DERIVE MACROS =====
    println!("\n--- Popular derive macros from crates ---");
    println!("  serde:      #[derive(Serialize, Deserialize)]");
    println!("  thiserror:  #[derive(Error)]");
    println!("  clap:       #[derive(Parser)]");
    println!("  sqlx:       #[derive(FromRow)]");
    println!("  strum:      #[derive(EnumString, Display)]");
}

// ============================================================
// PART 4: MACRO_RULES! BASICS
// ============================================================

fn part4_macro_rules_basics() {
    println!("\n--- PART 4: macro_rules! Basics ---\n");

    // macro_rules! defines pattern-matching macros
    // Syntax: macro_rules! name { (pattern) => { expansion }; }

    // ===== SIMPLEST MACRO =====
    macro_rules! say_hello {
        () => {
            println!("Hello from macro!");
        };
    }

    say_hello!();  // Expands to println!("Hello from macro!")

    // ===== MACRO WITH ARGUMENTS =====
    macro_rules! greet {
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
    }

    greet!("World");
    greet!("Rust");

    // ===== DESIGNATORS (FRAGMENT TYPES) =====
    // $name:designator - captures different syntax elements
    
    println!("\n--- Macro Designators ---");
    println!("
    | Designator | Matches                        | Example           |
    |------------|--------------------------------|-------------------|
    | expr       | Any expression                 | 1 + 2, foo()      |
    | ident      | Identifier                     | variable_name     |
    | ty         | Type                           | i32, Vec<String>  |
    | tt         | Token tree (anything)          | any tokens        |
    | literal    | Literal value                  | 42, \"hello\"       |
    | stmt       | Statement                      | let x = 1;        |
    | block      | Block {{{{ ... }}}}            | {{{{ code }}}}      |
    | path       | Path                           | std::io::Result   |
    | pat        | Pattern                        | Some(x), _        |
    | item       | Item (fn, struct, etc)         | fn foo() {{}}       |
    | meta       | Attribute contents             | derive(Debug)     |
    ");

    // ===== EXAMPLE: CREATE FUNCTION =====
    macro_rules! create_function {
        ($name:ident) => {
            fn $name() {
                println!("Function {:?} was called", stringify!($name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);
    
    foo();
    bar();

    // ===== MULTIPLE PATTERNS =====
    macro_rules! calculate {
        // Pattern 1: add
        (add $a:expr, $b:expr) => {
            $a + $b
        };
        // Pattern 2: mul
        (mul $a:expr, $b:expr) => {
            $a * $b
        };
        // Pattern 3: default (just the value)
        ($a:expr) => {
            $a
        };
    }

    println!("add 2, 3 = {}", calculate!(add 2, 3));
    println!("mul 4, 5 = {}", calculate!(mul 4, 5));
    println!("just 10 = {}", calculate!(10));

    // ===== STRINGIFY! AND CONCAT! =====
    let name = stringify!(my_variable_name);
    println!("stringify!: {}", name);  // "my_variable_name"

    let combined = concat!("Hello", " ", "World", "!");
    println!("concat!: {}", combined);  // "Hello World!"
}

// ============================================================
// PART 5: MACRO PATTERNS (REPETITION)
// ============================================================

fn part5_macro_patterns() {
    println!("\n--- PART 5: Macro Patterns (Repetition) ---\n");

    // The real power of macros: repetition patterns
    // $(...),* - zero or more, comma separated
    // $(...),+ - one or more, comma separated
    // $(...)? - zero or one (optional)

    // ===== VARIABLE ARGUMENTS =====
    macro_rules! print_all {
        // $(...),* means: repeat for each comma-separated item
        ($($x:expr),*) => {
            $(
                println!("Value: {}", $x);
            )*
        };
    }

    print_all!(1, 2, 3);
    print_all!("hello", "world");

    // ===== RECREATING VEC! =====
    macro_rules! my_vec {
        // Empty case
        () => {
            Vec::new()
        };
        // Elements case: [elem, elem, ...]
        ($($element:expr),+ $(,)?) => {
            {
                let mut v = Vec::new();
                $(
                    v.push($element);
                )+
                v
            }
        };
        // Repeat case: [value; count]
        ($element:expr; $count:expr) => {
            vec![$element; $count]  // Delegate to standard vec!
        };
    }

    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![1, 2, 3];
    let v3 = my_vec![0; 5];
    let v4 = my_vec![1, 2, 3,];  // Trailing comma OK

    println!("my_vec![]: {:?}", v1);
    println!("my_vec![1,2,3]: {:?}", v2);
    println!("my_vec![0; 5]: {:?}", v3);
    println!("my_vec! trailing comma: {:?}", v4);

    // ===== HASHMAP LITERAL =====
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert($key, $value);
                )*
                map
            }
        };
    }

    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 92,
    };
    println!("HashMap: {:?}", scores);

    // ===== COUNTING ARGUMENTS =====
    macro_rules! count_args {
        () => { 0 };
        ($first:expr $(, $rest:expr)*) => {
            1 + count_args!($($rest),*)
        };
    }

    println!("count_args!(): {}", count_args!());
    println!("count_args!(a): {}", count_args!(1));
    println!("count_args!(a,b,c): {}", count_args!(1, 2, 3));

    // ===== OPTIONAL ARGUMENTS =====
    macro_rules! with_default {
        // With custom default
        ($value:expr, default = $default:expr) => {
            if $value > 0 { $value } else { $default }
        };
        // Without default (use 0)
        ($value:expr) => {
            with_default!($value, default = 0)
        };
    }

    println!("with_default!(5): {}", with_default!(5));
    println!("with_default!(-1): {}", with_default!(-1));
    println!("with_default!(-1, default=42): {}", with_default!(-1, default = 42));
}

// ============================================================
// PART 6: PRACTICAL MACRO EXAMPLES
// ============================================================

fn part6_practical_macros() {
    println!("\n--- PART 6: Practical Macro Examples ---\n");

    // ===== LOG MACRO WITH CONTEXT =====
    macro_rules! log_debug {
        ($($arg:tt)*) => {
            println!("[DEBUG {}:{}] {}", 
                file!(), 
                line!(), 
                format!($($arg)*))
        };
    }

    log_debug!("Starting operation");
    log_debug!("Value is {}", 42);

    // ===== MEASURE TIME MACRO =====
    macro_rules! time_it {
        ($name:expr, $block:block) => {{
            let start = std::time::Instant::now();
            let result = $block;
            let elapsed = start.elapsed();
            println!("{} took {:?}", $name, elapsed);
            result
        }};
    }

    let sum = time_it!("Sum calculation", {
        (0..1000).sum::<i32>()
    });
    println!("Sum: {}", sum);

    // ===== BUILDER MACRO =====
    macro_rules! builder_field {
        ($name:ident: $type:ty) => {
            pub fn $name(mut self, value: $type) -> Self {
                self.$name = Some(value);
                self
            }
        };
    }

    #[derive(Default)]
    struct RequestBuilder {
        url: Option<String>,
        method: Option<String>,
        timeout: Option<u32>,
    }

    impl RequestBuilder {
        fn new() -> Self {
            Self::default()
        }
        
        builder_field!(url: String);
        builder_field!(method: String);
        builder_field!(timeout: u32);
        
        fn build(self) -> String {
            format!(
                "{} {} (timeout: {:?})",
                self.method.unwrap_or_else(|| "GET".into()),
                self.url.unwrap_or_else(|| "/".into()),
                self.timeout
            )
        }
    }

    let request = RequestBuilder::new()
        .url("https://example.com".into())
        .method("POST".into())
        .timeout(30)
        .build();
    println!("Request: {}", request);

    // ===== ENUM DISPATCH MACRO =====
    macro_rules! enum_dispatch {
        ($enum_name:ident, $method:ident, $($variant:ident),+) => {
            impl $enum_name {
                fn $method(&self) -> &'static str {
                    match self {
                        $(
                            $enum_name::$variant => stringify!($variant),
                        )+
                    }
                }
            }
        };
    }

    #[derive(Debug)]
    enum Animal {
        Dog,
        Cat,
        Bird,
    }

    enum_dispatch!(Animal, name, Dog, Cat, Bird);

    let pet = Animal::Cat;
    println!("Animal name: {}", pet.name());

    // ===== TEST HELPER MACRO =====
    macro_rules! test_case {
        ($name:ident: $input:expr => $expected:expr) => {
            // In real code this would be #[test]
            fn $name() {
                let result = $input;
                assert_eq!(result, $expected, 
                    "Test {} failed: {:?} != {:?}", 
                    stringify!($name), result, $expected);
                println!("✓ Test {} passed", stringify!($name));
            }
        };
    }

    test_case!(test_addition: 2 + 2 => 4);
    test_case!(test_string: "hello".to_uppercase() => "HELLO");

    test_addition();
    test_string();

    // ===== TRY MACRO (LIKE ? BUT CUSTOM) =====
    macro_rules! try_or_return {
        ($expr:expr, $default:expr) => {
            match $expr {
                Some(v) => v,
                None => return $default,
            }
        };
    }

    fn find_and_double(values: &[i32], target: i32) -> i32 {
        let found = try_or_return!(values.iter().find(|&&x| x == target), -1);
        found * 2
    }

    println!("find_and_double([1,2,3], 2): {}", find_and_double(&[1, 2, 3], 2));
    println!("find_and_double([1,2,3], 5): {}", find_and_double(&[1, 2, 3], 5));
}

// ============================================================
// PART 7: WHEN TO USE MACROS
// ============================================================

fn part7_when_to_use_macros() {
    println!("\n--- PART 7: When to Use Macros ---\n");

    // ===== USE MACROS WHEN =====
    println!("✓ USE macros when:");
    println!("  • You need variable number of arguments");
    println!("  • Reducing significant boilerplate");
    println!("  • Generating repetitive code patterns");
    println!("  • Need compile-time code generation");
    println!("  • Creating DSLs (domain-specific languages)");
    println!("  • Auto-implementing traits (derive)");

    // ===== DON'T USE MACROS WHEN =====
    println!("\n✗ DON'T use macros when:");
    println!("  • A function would work fine");
    println!("  • Generics can solve the problem");
    println!("  • It makes code harder to understand");
    println!("  • Error messages become confusing");
    println!("  • You're learning (start with functions!)");

    // ===== MACRO DOWNSIDES =====
    println!("\n--- Macro Downsides ---");
    println!("  • Harder to debug (expand with cargo expand)");
    println!("  • Error messages can be cryptic");
    println!("  • IDE support is limited");
    println!("  • Increase compile time");
    println!("  • Can be hard to read/maintain");

    // ===== ALTERNATIVES TO MACROS =====
    println!("\n--- Alternatives to Consider ---");
    
    // Instead of macro for multiple types:
    fn print_value<T: std::fmt::Display>(value: T) {
        println!("Value: {}", value);
    }
    print_value(42);
    print_value("hello");

    // Instead of macro for builder:
    // Use derive_builder crate or typed-builder

    // Instead of macro for enum variants:
    // Use strum crate for enum utilities

    println!("\n--- Decision Tree ---");
    println!("
    Need variable args?
    ├─ Yes → macro
    └─ No → Can generics help?
            ├─ Yes → use generics
            └─ No → Is it significant boilerplate?
                    ├─ Yes → consider macro
                    └─ No → use function/method
    ");

    // ===== DEBUGGING MACROS =====
    println!("\n--- Debugging Macros ---");
    println!("1. cargo install cargo-expand");
    println!("2. cargo expand module_name");
    println!("3. See the expanded code!");
    println!();
    println!("Example expansion of vec![1, 2, 3]:");
    println!("  {{");
    println!("      let mut temp = Vec::new();");
    println!("      temp.push(1);");
    println!("      temp.push(2);");
    println!("      temp.push(3);");
    println!("      temp");
    println!("  }}");
}

// ============================================================
// MACRO CHEAT SHEET
// ============================================================
//
// BUILT-IN MACROS:
//   println!/print!    → Print to stdout
//   eprintln!/eprint!  → Print to stderr
//   format!            → Create formatted String
//   vec!               → Create Vec
//   panic!             → Crash with message
//   assert!/assert_eq! → Test assertions
//   dbg!               → Debug print with location
//   todo!/unimplemented! → Placeholder panics
//   file!/line!/column! → Source location
//   stringify!         → Convert tokens to string
//   concat!            → Concatenate literals
//   include_str!/include_bytes! → Include file
//
// COMMON DERIVE MACROS:
//   Debug      → {:?} formatting
//   Clone      → .clone() method
//   Copy       → Implicit copy
//   PartialEq  → == comparison
//   Eq         → Full equality
//   Hash       → Hashable for maps/sets
//   Default    → Default::default()
//   PartialOrd → <, >, comparisons
//   Ord        → Total ordering
//
// MACRO_RULES SYNTAX:
//   macro_rules! name {
//       (pattern) => { expansion };
//   }
//
// DESIGNATORS:
//   $x:expr    → Expression
//   $x:ident   → Identifier
//   $x:ty      → Type
//   $x:tt      → Token tree
//   $x:literal → Literal
//   $x:stmt    → Statement
//   $x:block   → Block { }
//   $x:path    → Path
//   $x:pat     → Pattern
//
// REPETITION:
//   $(...),*   → Zero or more
//   $(...),+   → One or more
//   $(...)?    → Optional
//
// ============================================================

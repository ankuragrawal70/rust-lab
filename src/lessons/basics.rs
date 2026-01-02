/// ============================================================================
/// BASICS MODULE - Variables, Arithmetic, Conditionals, Loops
/// ============================================================================

/// ============================================================================
/// 1. VARIABLES & MUTABILITY
/// ============================================================================
/// Key Concepts:
/// - `let x = 5;`      → Immutable variable (cannot be changed)
/// - `let mut y = 10;` → Mutable variable (can be changed)
/// - `z: i32`          → Explicit type annotation
/// - `{}`              → Placeholder in formatted output
/// - `println!`        → Macro (not a function), indicated by `!`
/// ============================================================================
pub fn learn_variables_and_mutability() {
    println!("\n============================================================");
    println!("📘 LESSON 1: Variables & Mutability");
    println!("============================================================\n");

    // Immutable variable - integers are Copy types
    // You cannot change the value of x because it is immutable
    let x = 5;
    // x += 3; // ❌ This will cause a compile-time error
    println!("Immutable x = {}", x);

    // Mutable variable - you can change the value
    let mut y = 10;
    println!("Mutable y (before) = {}", y);
    y += 3;
    println!("Mutable y (after += 3) = {}", y);

    // Explicit type annotation
    let z: i32 = 20;
    println!("Explicitly typed z: i32 = {}", z);

    // Practical example: temperature that changes
    let mut temperature = 30;
    println!("Temperature is {} degrees Celsius.", temperature);
    temperature += 5;
    println!("Temperature is now {} degrees Celsius.", temperature);
}

/// ============================================================================
/// 2. ARITHMETIC OPERATIONS & OWNERSHIP INTRODUCTION
/// ============================================================================
/// Key Concepts:
/// - Each value has an owner (the variable that holds it)
/// - Arithmetic operations create new values with new owners
/// - For Copy types (integers), the original values remain valid
/// ============================================================================
pub fn learn_arithmetic_operations() {
    println!("\n============================================================");
    println!("📘 LESSON 2: Arithmetic Operations & Ownership Intro");
    println!("============================================================\n");

    // Owner is the variable that holds the value
    let a = 10;  // a owns the value 10
    let b = 3;   // b owns the value 3

    // Each operation creates a new value with a new owner
    // a and b remain valid (Copy types)
    let sum = a + b;        // sum owns the result
    let diff = a - b;       // diff owns the result
    let prod = a * b;       // prod owns the result
    let div = a / b;        // div owns the result (integer division)
    let remainder = a % b;  // remainder owns the result

    println!("a = {}, b = {}", a, b);
    println!("sum (a + b) = {}", sum);
    println!("difference (a - b) = {}", diff);
    println!("product (a * b) = {}", prod);
    println!("division (a / b) = {}", div);
    println!("remainder (a % b) = {}", remainder);
}

/// ============================================================================
/// 3. CONDITIONAL EXPRESSIONS
/// ============================================================================
/// Key Concepts:
/// - `if`, `else if`, `else` for branching logic
/// - Conditions must be boolean expressions
/// - No parentheses required around conditions (unlike C/Java)
/// ============================================================================
pub fn learn_conditionals() {
    println!("\n============================================================");
    println!("📘 LESSON 3: Conditional Expressions");
    println!("============================================================\n");

    let temperature = 40;
    println!("Temperature is {} degrees", temperature);

    if temperature > 30 {
        println!("It's a hot day! 🔥");
    } else if temperature < 15 {
        println!("It's a cold day! ❄️");
    } else {
        println!("The weather is nice. 🌤️");
    }
}

/// ============================================================================
/// 4. LOOPS (while, loop, for)
/// ============================================================================
/// Key Concepts:
/// - `while condition { }` → Loop while condition is true
/// - `loop { }`            → Infinite loop (use `break` to exit)
/// - `for i in range { }`  → Iterate over a range or collection
/// - `1..5`  → Range from 1 to 4 (exclusive end)
/// - `1..=5` → Range from 1 to 5 (inclusive end)
/// ============================================================================
pub fn learn_loops() {
    println!("\n============================================================");
    println!("📘 LESSON 4: Loops (while, loop, for)");
    println!("============================================================\n");

    // While loop - runs while condition is true
    println!("--- While Loop ---");
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }

    // Infinite loop with break
    println!("\n--- Infinite Loop with Break ---");
    let mut infinite_loop = 0;
    loop {
        if infinite_loop >= 2 {
            println!("Breaking the loop now.");
            break;
        } else {
            println!("This will run forever unless we break.");
            infinite_loop += 1;
        }
    }

    // For loop with inclusive range (1..=6 means 1 to 6)
    println!("\n--- For Loop (inclusive range 1..=6) ---");
    for i in 1..=6 {
        println!("i = {}", i);
    }

    // For loop with exclusive range and condition
    println!("\n--- For Loop (even numbers in 1..10) ---");
    for i in 1..10 {
        if i % 2 == 0 {
            println!("even i = {}", i);
        }
    }
}

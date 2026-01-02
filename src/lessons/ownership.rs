/// ============================================================================
/// OWNERSHIP MODULE - Ownership Deep Dive
/// ============================================================================

/// ============================================================================
/// 6. OWNERSHIP DEEP DIVE
/// ============================================================================
/// Key Concepts:
/// | Type                    | Behavior                    |
/// |-------------------------|------------------------------|
/// | Copy type (i32, bool)   | Duplicated bit-for-bit       |
/// | Heap type (String, Vec) | Ownership moves              |
/// | &T                      | Borrowed reference           |
///
/// - Each value has a single owner
/// - When owner goes out of scope, value is dropped (freed)
/// - Assignment moves ownership for heap types
/// - Assignment copies for stack types that implement Copy
/// ============================================================================
pub fn learn_ownership() {
    println!("\n============================================================");
    println!("📘 LESSON 6: Ownership Deep Dive");
    println!("============================================================\n");

    // Copy types: integers are copied, not moved
    // a ──copy──▶ b
    println!("--- Copy Types (Integers) ---");
    let a = 5;
    let b = a; // Copy occurs, both a and b are valid
    println!("a = {}, b = {} (both valid, copy occurred)", a, b);

    // Heap types: String ownership moves
    // s1 ──move──▶ s2
    println!("\n--- Move Semantics (String) ---");
    let s1 = String::from("test ownership");
    let s2 = s1; // Ownership moves, s1 is no longer valid
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // ❌ Compile error: value borrowed after move

    // Arrays of Copy types: array is copied
    println!("\n--- Arrays of Copy Types ---");
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1; // Array copied (elements implement Copy)
    println!("arr1 = {:?}, arr2 = {:?}", arr1, arr2);

    // Two ways to iterate: by value vs by reference
    println!("\n--- Iteration: by value vs by reference ---");
    println!("for n in arr1 (by value):");
    for n in arr1 {
        println!("  n = {}", n);
    }

    println!("for val in arr1.iter() (by reference):");
    for val in arr1.iter() {
        println!("  val = {}", val);
    }

    // Arrays of heap types: ownership moves!
    println!("\n--- Arrays of Heap Types (String) ---");
    let str_arr1 = [
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];
    let str_arr2 = str_arr1; // Ownership of entire array moves
    // println!("{:?}", str_arr1); // ❌ Compile error

    // Using .iter() borrows, so array stays valid
    println!("Iterating with .iter() (borrows):");
    for s in str_arr2.iter() {
        println!("  s = {}", s);
    }
    println!("Array still valid: {:?}", str_arr2);

    // Direct iteration moves each element
    println!("\nDirect iteration (moves each element):");
    for st in str_arr2 {
        println!("  st = {}", st);
    }
    // println!("{:?}", str_arr2); // ❌ Array no longer valid after moving elements
}

/// ============================================================================
/// 8. FUNCTIONS & OWNERSHIP
/// ============================================================================
/// Key Concepts:
/// - Passing Copy types to functions: value is copied
/// - Passing heap types to functions: ownership moves
/// - To keep ownership, return the value or pass by reference
///
/// | Parameter Type | What Happens                    |
/// |----------------|----------------------------------|
/// | T (owned)      | Ownership moves into function    |
/// | &T             | Immutable borrow (read-only)     |
/// | &mut T         | Mutable borrow (can modify)      |
/// ============================================================================
pub fn learn_functions_and_ownership() {
    println!("\n============================================================");
    println!("📘 LESSON 8: Functions & Ownership");
    println!("============================================================\n");

    // Copy type: value is copied into function
    println!("--- Copy Types in Functions ---");
    let x = 5;
    let y = add_one(x);
    println!("x = {}, y = {} (x still usable - Copy type)", x, y);

    // Non-Copy type: ownership moves into function
    println!("\n--- Move Semantics in Functions ---");
    let s = String::from("Hello");
    let s2 = take_ownership(s); // s moves into function
    println!("s2 = {}", s2);
    // println!("{}", s); // ❌ s is no longer valid
}

/// Adds one to the given integer (Copy type - original value unaffected)
fn add_one(n: i32) -> i32 {
    n + 1
}

/// Takes ownership of a String and returns it back to caller
fn take_ownership(str_val: String) -> String {
    println!("Inside function: {}", str_val);
    str_val // Return ownership to caller
}

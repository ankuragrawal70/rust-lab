/// ============================================================================
/// BORROWING MODULE - Borrowing Basics & Borrowing with Functions
/// ============================================================================

/// ============================================================================
/// 7. BORROWING BASICS
/// ============================================================================
/// Key Mental Model:
/// - Owner → has responsibility to free memory
/// - Borrower → just reads or temporarily modifies, cannot free
/// - Multiple immutable borrows (&T) → safe ✅
/// - One mutable borrow (&mut T) → safe ✅
/// - Both at same time → compile error ❌
///
/// 💡 Analogy:
/// - Owner = person who owns a book
/// - Immutable borrow = friend who just reads it
/// - Mutable borrow = friend who edits it
/// - Rust ensures no friend can edit while others read
///
/// | Active borrows   | Can owner read? | Can owner write? |
/// |------------------|-----------------|------------------|
/// | None             | ✅              | ✅               |
/// | One or more &T   | ✅              | ❌               |
/// | One &mut T       | ❌              | ❌               |
/// ============================================================================
pub fn learn_borrowing() {
    println!("\n============================================================");
    println!("📘 LESSON 7: Borrowing Basics");
    println!("============================================================\n");

    // Immutable borrowing: multiple references allowed
    println!("--- Immutable Borrowing ---");
    let original = String::from("Hello, Rust!");
    let borrow1 = &original; // First immutable borrow
    let borrow2 = &original; // Second immutable borrow - OK!
    println!("original = {}", original);
    println!("borrow1 = {}", borrow1);
    println!("borrow2 = {}", borrow2);

    // Mutable borrowing: only one mutable reference at a time
    println!("\n--- Mutable Borrowing ---");
    let mut mutable_string = String::from("Hello");
    let mutable_borrow = &mut mutable_string;
    mutable_borrow.push_str(", world!");
    println!("After first mutable borrow: {}", mutable_string);

    // Second mutable borrow works because first is no longer used (NLL)
    let mutable_borrow2 = &mut mutable_string;
    mutable_borrow2.push_str(" Welcome to Rust!");
    println!("After second mutable borrow: {}", mutable_string);
    // println!("{}", mutable_borrow); // ❌ Error: first borrow invalidated
    println!("Owner still valid: {}", mutable_string);

    // Mutable borrow of array
    println!("\n--- Mutable Borrow of Array ---");
    let mut arr = [1, 2, 3, 4, 5];
    let arr_borrow = &mut arr;
    for i in 0..arr_borrow.len() {
        arr_borrow[i] += 10;
    }
    println!("Modified array: {:?}", arr);

    // Direct mutation by owner
    println!("\n--- Owner Direct Mutation ---");
    let mut arr_test = [12, 23, 34, 45, 56];
    arr_test[0] += 1; // Valid: owner can modify when no active borrows
    println!("arr_test[0] = {}", arr_test[0]);
}

/// ============================================================================
/// 9. BORROWING WITH FUNCTIONS
/// ============================================================================
/// Key Concepts:
/// - Pass &T for read-only access without taking ownership
/// - Pass &mut T for modifying data without taking ownership
/// - Function can borrow multiple times (non-overlapping lifetimes)
/// ============================================================================
pub fn learn_borrowing_with_functions() {
    println!("\n============================================================");
    println!("📘 LESSON 9: Borrowing with Functions");
    println!("============================================================\n");

    // Immutable borrow: function reads but doesn't take ownership
    println!("--- Immutable Borrow in Function ---");
    let original = String::from("Hello, Rust!");
    print_length(&original); // Pass reference, not ownership
    println!("Original still valid: {}", original);

    // Mutable borrow: function modifies without taking ownership
    println!("\n--- Mutable Borrow in Function ---");
    let mut s2 = String::from("Hi");
    append_world(&mut s2);
    println!("After first append: {}", s2);

    append_world(&mut s2); // Can borrow again (previous borrow ended)
    println!("After second append: {}", s2);

    // Creating another mutable reference
    let second_borrow = &mut s2;
    println!("Through second_borrow: {}", second_borrow);
}

/// Prints the length of a string (immutable borrow - read only)
fn print_length(s: &String) {
    println!("Length = {}", s.len());
}

/// Appends ", world!" to a string (mutable borrow - can modify)
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

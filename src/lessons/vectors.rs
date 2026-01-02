/// ============================================================================
/// VECTORS MODULE - Vectors with Ownership & Borrowing
/// ============================================================================

/// ============================================================================
/// 11. VECTORS WITH OWNERSHIP & BORROWING
/// ============================================================================
/// Key Concepts:
/// - `vec![]` macro creates a heap-allocated vector
/// - `.iter()` → borrows elements immutably
/// - `.iter_mut()` → borrows elements mutably
/// - `for item in vec` → moves/consumes the vector
/// - Slices `&vec[a..b]` borrow a portion of the vector
///
/// | Active borrows   | Can owner read? | Can owner write? |
/// |------------------|-----------------|------------------|
/// | None             | ✅              | ✅               |
/// | One or more &T   | ✅              | ❌               |
/// | One &mut T       | ❌              | ❌               |
/// ============================================================================
pub fn learn_vectors_with_ownership() {
    println!("\n============================================================");
    println!("📘 LESSON 11: Vectors with Ownership & Borrowing");
    println!("============================================================\n");

    // Vector of Strings (heap types)
    println!("--- Vector of Strings ---");
    let mut vec = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    // Immutable iteration with .iter()
    println!("Immutable iteration:");
    for s in vec.iter() {
        println!("  {}", s);
    }

    // Mutable iteration with .iter_mut()
    println!("\nMutable iteration (appending ' modified'):");
    for s in vec.iter_mut() {
        s.push_str(" modified");
    }

    // Verify modifications
    println!("\nAfter modification:");
    for s in vec.iter() {
        println!("  {}", s);
    }

    // Owner can modify when no active borrows
    vec.push(String::from("four"));
    println!("\nAfter push: {:?}", vec);

    // Vector of integers (Copy types)
    println!("\n--- Vector of Integers ---");
    let mut int_vec = vec![1, 2, 3, 4, 5];
    println!("Before modifications: {:?}", int_vec);

    int_vec.push(6);
    int_vec[0] = 10;
    println!("After push and modify: {:?}", int_vec);

    // Slices: borrowing a portion of the vector
    println!("\n--- Slices (Borrowing a Portion) ---");
    let slice = &int_vec[1..4]; // Immutable borrow of indices 1, 2, 3
    println!("Slice [1..4]: {:?}", slice);

    // Owner can modify after slice is no longer used
    int_vec[2] = 100;
    println!("After owner modification: {:?}", int_vec);

    // Mutable slice
    println!("\n--- Mutable Slice ---");
    let slice_mut = &mut int_vec[0..3];
    slice_mut[1] = 20;
    println!("Mutable slice [0..3]: {:?}", slice_mut);
    println!("Full vector after slice modification: {:?}", int_vec);

    // Print borrowing law summary
    print_borrowing_law_summary();
}

fn print_borrowing_law_summary() {
    println!("\n============================================================");
    println!("📚 SUMMARY: The Borrowing Law");
    println!("============================================================");
    println!(
        "
At any moment, ONE of these is allowed:
┌────────────────────────────────────┬─────────┐
│ Situation                          │ Allowed │
├────────────────────────────────────┼─────────┤
│ Any number of &T (immutable)       │   ✅    │
│ Exactly one &mut T (mutable)       │   ✅    │
│ Both at the same time              │   ❌    │
└────────────────────────────────────┴─────────┘

Owner access during active borrows:
┌──────────────────┬─────────────┬──────────────┐
│ Active Borrows   │ Owner Read? │ Owner Write? │
├──────────────────┼─────────────┼──────────────┤
│ None             │     ✅      │      ✅      │
│ One or more &T   │     ✅      │      ❌      │
│ One &mut T       │     ❌      │      ❌      │
└──────────────────┴─────────────┴──────────────┘

Why? To guarantee:
• No data races
• No dangling references
• No memory corruption
• Thread safety without garbage collection
"
    );
}

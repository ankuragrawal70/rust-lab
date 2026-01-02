/// ============================================================================
/// ARRAYS MODULE - Arrays & Indexing
/// ============================================================================

/// ============================================================================
/// 5. ARRAYS & INDEXING
/// ============================================================================
/// Key Concepts:
/// - Arrays have fixed size, known at compile time
/// - Zero-indexed: first element is at index 0
/// - `array[i]` → Access element at index i
/// - `for n in array` → Iterate by value (moves for non-Copy)
/// - `for n in array.iter()` → Iterate by reference (borrows)
/// ============================================================================
pub fn learn_arrays_and_indexing() {
    println!("\n============================================================");
    println!("📘 LESSON 5: Arrays & Indexing");
    println!("============================================================\n");

    let numbers = [10, 20, 30, 40, 50];

    // Direct indexing
    println!("--- Direct Indexing ---");
    println!("First number: {}", numbers[0]);
    println!("Second number: {}", numbers[1]);
    println!("Third number: {}", numbers[2]);
    println!("Fourth number: {}", numbers[3]);
    println!("Fifth number: {}", numbers[4]);

    // Iterating by value (works for Copy types like i32)
    println!("\n--- Iterating by Value ---");
    for n in numbers {
        println!("number = {}", n);
    }

    // Copying an element and modifying the copy
    let mut m = numbers[0];
    m = m + 1;
    println!("\nCopied and modified: m = {}", m);
    println!("Original unchanged: numbers[0] = {}", numbers[0]);

    // Iterating by reference using .iter()
    println!("\n--- Iterating by Reference (.iter()) ---");
    for k in numbers.iter() {
        println!("k = {}", k);
    }
}

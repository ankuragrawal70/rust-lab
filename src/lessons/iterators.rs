/// ============================================================================
/// ITERATORS MODULE - Iterators & Functional Combinators
/// ============================================================================

/// ============================================================================
/// 15. ITERATORS & FUNCTIONAL COMBINATORS
/// ============================================================================
/// Key Concepts:
/// - `.iter()`      → Immutable borrow of elements
/// - `.iter_mut()`  → Mutable borrow of elements
/// - `.into_iter()` → Takes ownership of elements
/// - `.map()`       → Transform each element
/// - `.filter()`    → Keep elements matching a condition
/// - `.collect()`   → Materialize iterator into a collection
/// - `.enumerate()` → Get (index, value) pairs
/// - `.chain()`     → Concatenate iterators
/// ============================================================================
pub fn learn_iterators_and_functional_combinators() {
    println!("\n============================================================");
    println!("📘 LESSON 15: Iterators & Functional Combinators");
    println!("============================================================\n");

    // Three ways to iterate
    println!("--- Three Ways to Iterate ---");
    iterate_examples();

    // Map: transform elements
    println!("\n--- map(): Transform Elements ---");
    map_examples();

    // Filter: select elements
    println!("\n--- filter(): Select Elements ---");
    filter_examples();

    // Chain: concatenate iterators
    println!("\n--- chain(): Concatenate Iterators ---");
    chain_examples();

    // Chaining multiple operations
    println!("\n--- Chaining Multiple Operations ---");
    chaining_examples();

    // Enumerate: index + value
    println!("\n--- enumerate(): Index + Value ---");
    enumerate_examples();
}

fn iterate_examples() {
    let v = vec![1, 2, 3];

    // Immutable borrow - v stays valid
    println!(".iter() - Immutable borrow:");
    for x in v.iter() {
        println!("  x = {}", x);
    }
    println!("  v still valid: {:?}", v);

    // Mutable borrow - can modify elements
    println!("\n.iter_mut() - Mutable borrow:");
    let mut v2 = vec![1, 2, 3];
    for x in v2.iter_mut() {
        *x += 10;
    }
    println!("  v2 after modification: {:?}", v2);

    // Takes ownership - v consumed
    println!("\n.into_iter() - Takes ownership:");
    let v3 = vec![1, 2, 3];
    for x in v3.into_iter() {
        println!("  x = {}", x);
    }
    // println!("{:?}", v3); // ❌ v3 is consumed
}

fn map_examples() {
    let v = vec![1, 2, 3];

    // .map() transforms each element
    // Returns a lazy iterator, must .collect() to materialize
    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("Original: {:?}", v);
    println!("Squared:  {:?}", squared);

    // Map with ownership
    let strings: Vec<String> = v.iter().map(|x| format!("num_{}", x)).collect();
    println!("As strings: {:?}", strings);
}

fn filter_examples() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // .filter() keeps elements matching the predicate
    // Note: filter gives &T, so we use *x to dereference
    let evens: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Original: {:?}", v);
    println!("Evens:    {:?}", evens);

    // Filter with different condition
    let greater_than_3: Vec<&i32> = v.iter().filter(|x| **x > 3).collect();
    println!("Greater than 3: {:?}", greater_than_3);
}

fn chain_examples() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // .chain() concatenates two iterators
    let chained: Vec<i32> = v1.iter().chain(v2.iter()).cloned().collect();
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("Chained: {:?}", chained);
}

fn chaining_examples() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // Chain multiple operations together
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0) // Keep even numbers
        .map(|x| x * 10)         // Multiply each by 10
        .collect();

    println!("Original: {:?}", numbers);
    println!("Evens × 10: {:?}", result);

    // More complex pipeline
    let complex: Vec<String> = numbers
        .iter()
        .filter(|x| *x % 2 == 1)           // Keep odd numbers
        .map(|x| x * x)                     // Square them
        .filter(|x| *x > 5)                 // Keep if > 5
        .map(|x| format!("val={}", x))      // Convert to string
        .collect();

    println!("Complex pipeline: {:?}", complex);
}

fn enumerate_examples() {
    let v = vec!["apple", "banana", "cherry"];

    // .enumerate() gives (index, value) pairs
    println!("Fruits with indices:");
    for (index, value) in v.iter().enumerate() {
        println!("  [{}] = {}", index, value);
    }

    // Using enumerate in a pipeline
    let indexed: Vec<String> = v
        .iter()
        .enumerate()
        .map(|(i, val)| format!("{}. {}", i + 1, val))
        .collect();

    println!("\nNumbered list:");
    for item in indexed {
        println!("  {}", item);
    }
}

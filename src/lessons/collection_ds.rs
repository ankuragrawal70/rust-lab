//. rust collection and data structures lesson
/// ============================================================================
/// 10. COLLECTIONS AND DATA STRUCTURES
/// ============================================================================
/// Key Concepts:
/// - Common collections: Vec, String, HashMap
/// - Using collections with ownership and borrowing
/// - Iterating over collections
/// ============================================================================
/// This lesson covers common collections in Rust, including vectors, strings, and hash maps.
/// It demonstrates how to use these collections while adhering to Rust's ownership and borrowing rules.
///     
/// Examples:
/// - Creating and modifying a vector
/// - Creating and modifying a string
/// - Creating and modifying a hash map
/// - Iterating over collections
/// ============================================================================
/// 
use std::collections::HashMap;
use std::collections::HashSet;

use std::collections::VecDeque;

pub fn learn_collections_and_data_structures() {
    println!("\n============================================================");
    println!("📘 LESSON 10: Collections and Data Structures");
    println!("============================================================\n");

    // Vector example
    println!("--- Vector Example ---");
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Vector: {:?}", numbers);

    // Iterating over vector
    println!("Iterating over vector:");
    for num in &numbers {
        println!("Number: {}", num);
    }

    for n in numbers.iter_mut() {
        *n += 5;
    }
    println!("Modified Vector: {:?}", numbers);

    // String example
    println!("\n--- String Example ---");
    let mut greeting = String::from("Hello");
    greeting.push_str(", world!");
    println!("String: {}", greeting);

    // HashMap example

    println!("\n--- HashMap Example ---");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 60);
    println!("HashMap: {:?}", scores);
    // get value
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    // Iterating over HashMap
    println!("Iterating over HashMap:");
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(1); // duplicate ignored

    println!("{:?}", set); // {1,2}

    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("{:?}", deque); // [0,1,2]

}
/// ============================================================================
/// STRUCTS MODULE - Structs with Ownership & Borrowing
/// ============================================================================

/// A simple Person struct to demonstrate ownership with custom types
pub struct Person {
    pub name: String,
    pub age: u32,
}

/// ============================================================================
/// 10. STRUCTS WITH OWNERSHIP & BORROWING
/// ============================================================================
/// Key Concepts:
/// - Structs containing String own that heap data
/// - Borrow struct with &T for reading fields
/// - Borrow struct with &mut T for modifying fields
/// - Same borrowing rules apply to custom types
/// ============================================================================
pub fn learn_structs_with_ownership() {
    println!("\n============================================================");
    println!("📘 LESSON 10: Structs with Ownership & Borrowing");
    println!("============================================================\n");

    let mut person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("--- Reading Struct (Immutable Borrow) ---");
    print_person(&person);

    println!("\n--- Modifying Struct (Mutable Borrow) ---");
    update_age(&mut person, 31);
    print_person(&person);
}

/// Prints person details (immutable borrow)
pub fn print_person(p: &Person) {
    println!("Name: {}, Age: {}", p.name, p.age);
}

/// Updates person's age (mutable borrow)
pub fn update_age(p: &mut Person, new_age: u32) {
    p.age = new_age;
}

/// ============================================================================
/// 🦀 RUST LEARNING GUIDE - Modular Organization
/// ============================================================================
/// This project demonstrates Rust concepts organized into a nested module.
///
/// Project Structure:
/// src/
/// ├── main.rs              → Entry point
/// └── lessons/             → All learning modules
///     ├── mod.rs           → Module declarations & run_all_lessons()
///     ├── basics.rs        → Variables, arithmetic, conditionals, loops
///     ├── arrays.rs        → Arrays and indexing
///     ├── ownership.rs     → Ownership deep dive, functions & ownership
///     ├── borrowing.rs     → Borrowing basics, borrowing with functions
///     ├── structs.rs       → Structs with ownership & borrowing
///     ├── vectors.rs       → Vectors with ownership & borrowing
///     ├── enums.rs         → Enums and pattern matching
///     ├── iterators.rs     → Iterators and functional combinators
///     └── option_result.rs → Option<T> and Result<T, E> types
/// ============================================================================

mod lessons;

fn main() {
    // Run all lessons from the lessons module
    lessons::run_all_lessons();

    // Or run individual lessons:
    // lessons::basics::learn_variables_and_mutability();
    // lessons::ownership::learn_ownership();
    // lessons::enums::learn_enums_and_pattern_matching();
}




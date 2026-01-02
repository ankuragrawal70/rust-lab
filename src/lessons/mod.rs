/// ============================================================================
/// LESSONS MODULE - All Rust Learning Topics
/// ============================================================================
/// This module contains all learning lessons organized by topic.
///
/// Submodules:
/// - basics        → Variables, arithmetic, conditionals, loops
/// - arrays        → Arrays and indexing
/// - ownership     → Ownership deep dive, functions & ownership
/// - borrowing     → Borrowing basics, borrowing with functions
/// - structs       → Structs with ownership & borrowing
/// - vectors       → Vectors with ownership & borrowing
/// - enums         → Enums and pattern matching
/// - iterators     → Iterators and functional combinators
/// - option_result → Option<T> and Result<T, E> types
/// ============================================================================

pub mod basics;
pub mod arrays;
pub mod ownership;
pub mod borrowing;
pub mod structs;
pub mod vectors;
pub mod enums;
pub mod iterators;
pub mod option_result;
pub mod collection_ds;

/// Run all lessons in order
pub fn run_all_lessons() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          🦀 RUST LEARNING GUIDE - All Lessons 🦀            ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Phase 1: Basics
    basics::learn_variables_and_mutability();
    basics::learn_arithmetic_operations();
    basics::learn_conditionals();
    basics::learn_loops();

    // Phase 2: Data Structures
    arrays::learn_arrays_and_indexing();

    // Phase 3: Ownership & Borrowing
    ownership::learn_ownership();
    borrowing::learn_borrowing();
    ownership::learn_functions_and_ownership();
    borrowing::learn_borrowing_with_functions();

    // Phase 4: Custom Types
    structs::learn_structs_with_ownership();
    vectors::learn_vectors_with_ownership();
    enums::learn_enums_and_pattern_matching();

    // Phase 5: Functional Programming & Error Handling
    option_result::learn_option_type();
    option_result::learn_error_handling_with_result();
    iterators::learn_iterators_and_functional_combinators();
    collection_ds::learn_collections_and_data_structures();

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║         ✅ All Lessons Completed Successfully! ✅          ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}

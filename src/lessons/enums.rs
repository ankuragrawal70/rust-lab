/// ============================================================================
/// ENUMS MODULE - Enums & Pattern Matching
/// ============================================================================

/// Direction enum to demonstrate basic enum usage
#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Returns a message based on the direction
    pub fn message(&self) -> &'static str {
        match self {
            Direction::North => "You are heading North!",
            Direction::South => "You are heading South!",
            Direction::East => "You are heading East!",
            Direction::West => "You are heading West!",
        }
    }
}

/// ============================================================================
/// 12. ENUMS & PATTERN MATCHING
/// ============================================================================
/// Key Concepts:
/// - Enums define a type with a fixed set of variants
/// - Each variant is a value of the enum type (NOT a string!)
/// - `match` ensures all variants are handled (exhaustive)
/// - Enums can hold data (shown in advanced examples)
///
/// | Aspect          | Enum              | String            |
/// |-----------------|-------------------|-------------------|
/// | Storage         | Integer (efficient)| Heap-allocated   |
/// | Type safety     | Compiler checks   | Any text allowed  |
/// | Pattern match   | Exhaustive        | No compile checks |
/// ============================================================================
pub fn learn_enums_and_pattern_matching() {
    println!("\n============================================================");
    println!("📘 LESSON 12: Enums & Pattern Matching");
    println!("============================================================\n");

    // Basic enum usage
    println!("--- Basic Enum Usage ---");
    let my_direction = Direction::East;
    let message = my_direction.message();
    println!("{}", message);

    // Pattern matching with match
    println!("\n--- Pattern Matching ---");
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    for dir in directions {
        match dir {
            Direction::North => println!("⬆️  North - Cold regions ahead"),
            Direction::South => println!("⬇️  South - Warm weather coming"),
            Direction::East => println!("➡️  East - Sunrise direction"),
            Direction::West => println!("⬅️  West - Sunset direction"),
        }
    }

    // Enum with data
    println!("\n--- Enums with Data ---");
    learn_enums_with_data();
}

/// Demonstrates enums that can hold different types of data
fn learn_enums_with_data() {
    // Enum variants can hold different data types
    #[derive(Debug)]
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },    // Named fields (struct-like)
        Write(String),              // Single value (tuple-like)
        ChangeColor(u8, u8, u8),    // Multiple values (RGB)
    }

    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello!")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in messages {
        match msg {
            Message::Quit => {
                println!("Quit: No data, just a signal to exit");
            }
            Message::Move { x, y } => {
                println!("Move: to position ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("Write: message = '{}'", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("ChangeColor: RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

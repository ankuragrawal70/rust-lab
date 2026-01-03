/// ============================================================================
/// TRAITS & IMPL MODULE - Structs, Impl Blocks, and Traits
/// ============================================================================
/// This is the foundation of Rust's abstraction system (like interfaces in
/// other languages, but more powerful).
///
/// Key Concepts:
/// - struct     → Define custom data types
/// - impl       → Add methods to structs
/// - trait      → Define shared behavior (like interfaces)
/// - impl Trait → Implement a trait for a type
/// ============================================================================

/// ============================================================================
/// PART 1: STRUCTS - Defining Custom Types
/// ============================================================================
/// Structs group related data together. Three kinds:
/// 1. Named fields (most common)
/// 2. Tuple structs
/// 3. Unit structs (no fields)
/// ============================================================================

// Named field struct
#[derive(Debug)]  // Allows printing with {:?}
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

// Tuple struct - fields accessed by index
#[derive(Debug)]
pub struct Point(pub f64, pub f64);

// Unit struct - no data, used as markers
#[derive(Debug)]
pub struct Marker;

/// ============================================================================
/// PART 2: IMPL BLOCKS - Adding Methods to Structs
/// ============================================================================
/// `impl` blocks let you define:
/// - Associated functions (like static methods) → called with Type::function()
/// - Methods (take &self, &mut self, or self) → called with instance.method()
///
/// | Self Type   | What It Means                        |
/// |-------------|--------------------------------------|
/// | &self       | Borrow immutably (read-only)         |
/// | &mut self   | Borrow mutably (can modify)          |
/// | self        | Takes ownership (consumes the value) |
/// ============================================================================

impl Person {
    // =========================================================================
    // Associated Function (no self) - like a constructor
    // Called as: Person::new(...)
    // =========================================================================
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        Self {
            name: String::from(name),
            age,
            email: String::from(email),
        }
    }

    // Another associated function
    pub fn default_person() -> Self {
        Self::new("Unknown", 0, "unknown@example.com")
    }

    // =========================================================================
    // Methods (take self in some form)
    // Called as: person.greet()
    // =========================================================================

    // Immutable borrow - read-only access
    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I'm {} years old!", self.name, self.age)
    }

    // Immutable borrow - getter
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // Mutable borrow - can modify the struct
    pub fn have_birthday(&mut self) {
        self.age += 1;
        println!("🎂 Happy Birthday! {} is now {} years old!", self.name, self.age);
    }

    // Mutable borrow - update email
    pub fn update_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }

    // Takes ownership - consumes self (rare, but useful)
    pub fn into_parts(self) -> (String, u32, String) {
        (self.name, self.age, self.email)
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point(x, y)
    }

    pub fn distance_from_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

/// ============================================================================
/// PART 3: TRAITS - Defining Shared Behavior
/// ============================================================================
/// Traits define functionality that types can implement.
/// Think of them as "contracts" or "interfaces".
///
/// trait TraitName {
///     fn required_method(&self);              // Must implement
///     fn optional_method(&self) { ... }       // Has default implementation
/// }
/// ============================================================================

// Define a trait for anything that can speak
pub trait Speak {
    fn speak(&self) -> String;

    // Default implementation (can be overridden)
    fn shout(&self) -> String {
        self.speak().to_uppercase() + "!"
    }
}

// Define a trait for anything that can be summarized
pub trait Summary {
    fn summarize(&self) -> String;

    // Default implementation using another trait method
    fn summarize_short(&self) -> String {
        format!("{}...", &self.summarize()[..20.min(self.summarize().len())])
    }
}

// Define a trait for anything that has an area
pub trait Area {
    fn area(&self) -> f64;
}

/// ============================================================================
/// PART 4: IMPLEMENTING TRAITS FOR TYPES
/// ============================================================================
/// Syntax: impl TraitName for TypeName { ... }
/// ============================================================================

// Implement Speak for Person
impl Speak for Person {
    fn speak(&self) -> String {
        format!("Hi, I'm {}!", self.name)
    }
    // shout() uses the default implementation
}

// Implement Summary for Person
impl Summary for Person {
    fn summarize(&self) -> String {
        format!("{} ({}) - {}", self.name, self.age, self.email)
    }
}

// Let's create another struct to show traits work across types
#[derive(Debug)]
pub struct Dog {
    pub name: String,
    pub breed: String,
}

impl Dog {
    pub fn new(name: &str, breed: &str) -> Self {
        Self {
            name: String::from(name),
            breed: String::from(breed),
        }
    }
}

// Implement Speak for Dog - same trait, different behavior!
impl Speak for Dog {
    fn speak(&self) -> String {
        format!("Woof! I'm {} the {}!", self.name, self.breed)
    }

    // Override the default shout
    fn shout(&self) -> String {
        "WOOF WOOF WOOF!".to_string()
    }
}

// Shapes to demonstrate Area trait
#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// ============================================================================
/// PART 5: TRAIT BOUNDS - Using Traits in Functions
/// ============================================================================
/// You can write functions that accept any type implementing a trait.
///
/// Three equivalent syntaxes:
/// 1. fn foo(item: &impl Trait)           → Simple, inline
/// 2. fn foo<T: Trait>(item: &T)          → Generic with bound
/// 3. fn foo<T>(item: &T) where T: Trait  → Where clause (complex bounds)
/// ============================================================================

// Function that accepts anything that can Speak
pub fn make_speak(speaker: &impl Speak) {
    println!("{}", speaker.speak());
}

// Same thing with generic syntax
pub fn make_speak_generic<T: Speak>(speaker: &T) {
    println!("{}", speaker.speak());
}

// Function requiring multiple traits
pub fn describe<T: Speak + std::fmt::Debug>(item: &T) {
    println!("Debug: {:?}", item);
    println!("Speaks: {}", item.speak());
}

// Print area of any shape
pub fn print_area<T: Area>(shape: &T) {
    println!("Area: {:.2}", shape.area());
}

/// ============================================================================
/// LEARNING FUNCTION - Demonstrates All Concepts
/// ============================================================================
pub fn learn_traits_and_impl() {
    println!("\n============================================================");
    println!("📘 LESSON: Structs, Impl Blocks & Traits");
    println!("============================================================\n");

    // =========================================================================
    // Part 1: Creating Structs
    // =========================================================================
    println!("--- Part 1: Creating Structs ---\n");

    // Using associated function (constructor)
    let alice = Person::new("Alice", 30, "alice@example.com");
    println!("Created: {:?}", alice);

    // Using default constructor
    let unknown = Person::default_person();
    println!("Default: {:?}", unknown);

    // Tuple struct
    let point = Point::new(3.0, 4.0);
    println!("Point: {:?}", point);
    println!("Distance from origin: {}", point.distance_from_origin());

    // =========================================================================
    // Part 2: Using Methods
    // =========================================================================
    println!("\n--- Part 2: Using Methods ---\n");

    // Immutable methods (&self)
    println!("{}", alice.greet());
    println!("Is adult? {}", alice.is_adult());

    // Mutable methods (&mut self)
    let mut bob = Person::new("Bob", 17, "bob@example.com");
    println!("Before birthday: {} is {} years old", bob.name, bob.age);
    bob.have_birthday();
    println!("Is adult now? {}", bob.is_adult());

    // Method that consumes (self)
    let carol = Person::new("Carol", 25, "carol@example.com");
    let (name, age, email) = carol.into_parts();
    println!("Destructured: name={}, age={}, email={}", name, age, email);
    // println!("{:?}", carol); // ❌ Error: carol was moved

    // =========================================================================
    // Part 3: Using Traits
    // =========================================================================
    println!("\n--- Part 3: Using Traits ---\n");

    let person = Person::new("Dave", 28, "dave@example.com");
    let dog = Dog::new("Buddy", "Golden Retriever");

    // Same trait, different implementations
    println!("Person speaks: {}", person.speak());
    println!("Dog speaks: {}", dog.speak());

    // Default vs overridden methods
    println!("Person shouts: {}", person.shout());
    println!("Dog shouts: {}", dog.shout());

    // Summary trait
    println!("Person summary: {}", person.summarize());

    // =========================================================================
    // Part 4: Trait Bounds in Functions
    // =========================================================================
    println!("\n--- Part 4: Trait Bounds in Functions ---\n");

    // Function accepts any Speak implementor
    print!("make_speak(person): ");
    make_speak(&person);

    print!("make_speak(dog): ");
    make_speak(&dog);

    // Function with multiple trait bounds
    println!("\ndescribe(person):");
    describe(&person);

    println!("\ndescribe(dog):");
    describe(&dog);

    // =========================================================================
    // Part 5: Polymorphism with Traits
    // =========================================================================
    println!("\n--- Part 5: Polymorphism with Traits ---\n");

    let rect = Rectangle::new(10.0, 5.0);
    let circle = Circle::new(3.0);

    println!("Rectangle {:?}", rect);
    print_area(&rect);

    println!("\nCircle {:?}", circle);
    print_area(&circle);

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n============================================================");
    println!("📚 SUMMARY: Struct + Impl + Trait");
    println!("============================================================");
    println!(
        "
┌─────────────────────────────────────────────────────────────┐
│ STRUCT - Define custom data types                           │
├─────────────────────────────────────────────────────────────┤
│ struct Person {{ name: String, age: u32 }}                    │
│ struct Point(f64, f64);    // Tuple struct                  │
│ struct Marker;             // Unit struct                   │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ IMPL - Add methods to types                                 │
├─────────────────────────────────────────────────────────────┤
│ impl Person {{                                                │
│     fn new() -> Self {{ }}      // Associated function       │
│     fn greet(&self) {{ }}       // Immutable borrow          │
│     fn update(&mut self) {{ }}  // Mutable borrow            │
│     fn consume(self) {{ }}      // Takes ownership           │
│ }}                                                            │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRAIT - Define shared behavior                              │
├─────────────────────────────────────────────────────────────┤
│ trait Speak {{                                                │
│     fn speak(&self) -> String;    // Required                │
│     fn shout(&self) {{ ... }}      // Optional (default)      │
│ }}                                                            │
│                                                             │
│ impl Speak for Person {{                                      │
│     fn speak(&self) -> String {{ ... }}                       │
│ }}                                                            │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRAIT BOUNDS - Generic functions with constraints           │
├─────────────────────────────────────────────────────────────┤
│ fn process(item: &impl Trait) {{ }}                          │
│ fn process<T: Trait>(item: &T) {{ }}                         │
│ fn process<T: Trait1 + Trait2>(item: &T) {{ }}               │
└─────────────────────────────────────────────────────────────┘
"
    );
}

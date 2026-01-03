/// ============================================================================
/// ADVANCED TRAITS MODULE - Multiple Traits, Supertraits, and More
/// ============================================================================
/// Building on traits_impl.rs, this module covers:
/// - Multiple trait bounds
/// - Supertraits (trait inheritance)
/// - Generic structs with trait bounds
/// - Trait objects (dyn Trait) for runtime polymorphism
/// - Associated types
/// - Common standard library traits
/// ============================================================================

use std::fmt::{Debug, Display};

/// ============================================================================
/// PART 1: MULTIPLE TRAIT BOUNDS - Requiring Multiple Capabilities
/// ============================================================================
/// Sometimes you need a type that implements several traits at once.
/// Use `+` to combine trait bounds.
/// ============================================================================

// Traits for our examples
pub trait Printable {
    fn print_info(&self) -> String;
}

pub trait Saveable {
    fn save(&self) -> Result<(), String>;
}

pub trait Loadable {
    fn load(id: u32) -> Result<Self, String>
    where
        Self: Sized;
}

pub trait Validatable {
    fn is_valid(&self) -> bool;
}

// A document that implements multiple traits
#[derive(Debug, Clone)]
pub struct Document {
    pub id: u32,
    pub title: String,
    pub content: String,
}

impl Document {
    pub fn new(id: u32, title: &str, content: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

impl Printable for Document {
    fn print_info(&self) -> String {
        format!("Document #{}: {} ({} chars)", self.id, self.title, self.content.len())
    }
}

impl Saveable for Document {
    fn save(&self) -> Result<(), String> {
        println!("💾 Saving document: {}", self.title);
        Ok(())
    }
}

impl Loadable for Document {
    fn load(id: u32) -> Result<Self, String> {
        // Simulated load
        Ok(Document::new(id, "Loaded Doc", "Content from storage"))
    }
}

impl Validatable for Document {
    fn is_valid(&self) -> bool {
        !self.title.is_empty() && !self.content.is_empty()
    }
}

// Implement Display for Document
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.id, self.title)
    }
}

/// ============================================================================
/// FUNCTIONS WITH MULTIPLE TRAIT BOUNDS
/// ============================================================================

// Requires TWO traits: Printable + Saveable
pub fn print_and_save<T: Printable + Saveable>(item: &T) {
    println!("Info: {}", item.print_info());
    let _ = item.save();
}

// Requires THREE traits using where clause (cleaner for many bounds)
pub fn process_document<T>(item: &T)
where
    T: Printable + Saveable + Validatable,
{
    if item.is_valid() {
        println!("✅ Valid! {}", item.print_info());
        let _ = item.save();
    } else {
        println!("❌ Invalid document, cannot save");
    }
}

// Requires trait + standard library traits
pub fn debug_and_print<T: Printable + Debug + Clone>(item: &T) {
    let cloned = item.clone();
    println!("Debug view: {:?}", cloned);
    println!("Print info: {}", item.print_info());
}

// Mix of custom and std traits
pub fn display_and_save<T>(item: &T)
where
    T: Display + Saveable,
{
    println!("Displaying: {}", item);
    let _ = item.save();
}

/// ============================================================================
/// PART 2: SUPERTRAITS - Trait Inheritance
/// ============================================================================
/// A supertrait is a trait that requires another trait to be implemented.
/// Think of it as: "To implement B, you must first implement A"
/// ============================================================================

// Supertrait: Entity requires Debug
pub trait Entity: Debug {
    fn entity_id(&self) -> u32;
    fn entity_type(&self) -> &str;
}

// Supertrait chain: PersistentEntity requires Entity (which requires Debug)
pub trait PersistentEntity: Entity {
    fn persist(&self) -> Result<(), String>;
    fn delete(&self) -> Result<(), String>;
}

// Implementing for a type - must implement ALL traits in the chain
#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, username: &str, email: &str) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
        }
    }
}

// Must implement Entity first (and User already derives Debug)
impl Entity for User {
    fn entity_id(&self) -> u32 {
        self.id
    }

    fn entity_type(&self) -> &str {
        "User"
    }
}

// Now can implement PersistentEntity
impl PersistentEntity for User {
    fn persist(&self) -> Result<(), String> {
        println!("💾 Persisting {} #{}: {}", self.entity_type(), self.entity_id(), self.username);
        Ok(())
    }

    fn delete(&self) -> Result<(), String> {
        println!("🗑️ Deleting {} #{}", self.entity_type(), self.entity_id());
        Ok(())
    }
}

// Function using supertrait - automatically has access to parent trait methods
pub fn save_entity<T: PersistentEntity>(entity: &T) {
    // Can use Debug (grandparent trait)
    println!("Entity debug: {:?}", entity);
    
    // Can use Entity methods (parent trait)
    println!("Type: {}, ID: {}", entity.entity_type(), entity.entity_id());
    
    // Can use PersistentEntity methods
    let _ = entity.persist();
}

/// ============================================================================
/// PART 3: GENERIC STRUCTS WITH TRAIT BOUNDS
/// ============================================================================
/// You can require trait bounds on struct fields and impl blocks
/// ============================================================================

// Generic struct - T can be any type
#[derive(Debug)]
pub struct Container<T> {
    pub value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

// Add methods only when T implements specific traits
impl<T: Display> Container<T> {
    pub fn display_value(&self) {
        println!("Container holds: {}", self.value);
    }
}

impl<T: Clone> Container<T> {
    pub fn get_clone(&self) -> T {
        self.value.clone()
    }
}

impl<T: Default> Container<T> {
    pub fn new_default() -> Self {
        Self {
            value: T::default(),
        }
    }
}

// Multiple bounds on struct itself
#[derive(Debug)]
pub struct PrintableContainer<T: Display + Debug> {
    pub item: T,
    pub label: String,
}

impl<T: Display + Debug> PrintableContainer<T> {
    pub fn new(item: T, label: &str) -> Self {
        Self {
            item,
            label: label.to_string(),
        }
    }

    pub fn print_all(&self) {
        println!("Label: {}", self.label);
        println!("Display: {}", self.item);
        println!("Debug: {:?}", self.item);
    }
}

/// ============================================================================
/// PART 4: TRAIT OBJECTS - Runtime Polymorphism (dyn Trait)
/// ============================================================================
/// Static dispatch (generics): Compiler generates code for each concrete type
/// Dynamic dispatch (dyn): Uses vtable at runtime, allows mixed types in collections
///
/// Use `dyn Trait` when:
/// - You need a collection of different types implementing same trait
/// - You don't know the concrete type at compile time
/// - You want to reduce binary size (fewer monomorphized copies)
/// ============================================================================

pub trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self) -> String;
    fn describe(&self) -> String {
        format!("{} says: {}", self.name(), self.make_sound())
    }
}

#[derive(Debug)]
pub struct Cat {
    pub name: String,
}

#[derive(Debug)]
pub struct Cow {
    pub name: String,
}

#[derive(Debug)]
pub struct Duck {
    pub name: String,
}

impl Cat {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Cow {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Duck {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }
}

impl Animal for Cow {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_sound(&self) -> String {
        "Moo!".to_string()
    }
}

impl Animal for Duck {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_sound(&self) -> String {
        "Quack!".to_string()
    }
}

// Static dispatch - ONE type at a time
pub fn animal_speak_static<T: Animal>(animal: &T) {
    println!("{}", animal.describe());
}

// Dynamic dispatch - accepts ANY Animal at runtime
pub fn animal_speak_dynamic(animal: &dyn Animal) {
    println!("{}", animal.describe());
}

// The real power: heterogeneous collection
pub fn create_farm() -> Vec<Box<dyn Animal>> {
    vec![
        Box::new(Cat::new("Whiskers")),
        Box::new(Cow::new("Bessie")),
        Box::new(Duck::new("Donald")),
        Box::new(Cat::new("Mittens")),
    ]
}

/// ============================================================================
/// PART 5: COMMON STANDARD LIBRARY TRAITS
/// ============================================================================

// Demonstrate implementing common std traits
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub price_cents: u32,
}

impl Product {
    pub fn new(sku: &str, name: &str, price_cents: u32) -> Self {
        Self {
            sku: sku.to_string(),
            name: name.to_string(),
            price_cents,
        }
    }
}

// Display - for user-friendly printing with {}
impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - ${:.2}", self.name, self.price_cents as f64 / 100.0)
    }
}

// Default - for creating default instances
impl Default for Product {
    fn default() -> Self {
        Self {
            sku: "UNKNOWN".to_string(),
            name: "Default Product".to_string(),
            price_cents: 0,
        }
    }
}

// Ord - for sorting (requires PartialOrd, Eq, PartialEq)
impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price_cents.cmp(&other.price_cents)
    }
}

/// ============================================================================
/// LEARNING FUNCTION - Demonstrates All Advanced Concepts
/// ============================================================================
pub fn learn_advanced_traits() {
    println!("\n============================================================");
    println!("📘 LESSON: Advanced Traits - Multiple Bounds & More");
    println!("============================================================\n");

    // =========================================================================
    // Part 1: Multiple Trait Bounds
    // =========================================================================
    println!("--- Part 1: Multiple Trait Bounds ---\n");

    let doc = Document::new(1, "My Report", "This is the content of my report.");

    // Function requiring Printable + Saveable
    print_and_save(&doc);

    println!();

    // Function requiring Printable + Saveable + Validatable
    process_document(&doc);

    println!();

    // Function requiring Printable + Debug + Clone
    debug_and_print(&doc);

    println!();

    // Function requiring Display + Saveable
    display_and_save(&doc);

    // =========================================================================
    // Part 2: Supertraits
    // =========================================================================
    println!("\n--- Part 2: Supertraits (Trait Inheritance) ---\n");

    let user = User::new(42, "alice", "alice@example.com");

    // save_entity requires PersistentEntity, which requires Entity, which requires Debug
    save_entity(&user);

    // =========================================================================
    // Part 3: Generic Structs with Bounds
    // =========================================================================
    println!("\n--- Part 3: Generic Structs with Trait Bounds ---\n");

    // Container with Display type
    let num_container = Container::new(42);
    num_container.display_value(); // Works because i32: Display

    // Container with Clone type
    let cloned = num_container.get_clone();
    println!("Cloned value: {}", cloned);

    // Container with Default
    let default_container: Container<i32> = Container::new_default();
    println!("Default container: {:?}", default_container);

    // PrintableContainer requires T: Display + Debug
    let printable = PrintableContainer::new("Hello, Traits!", "greeting");
    printable.print_all();

    // =========================================================================
    // Part 4: Trait Objects (dyn Trait)
    // =========================================================================
    println!("\n--- Part 4: Trait Objects (dyn Trait) ---\n");

    let cat = Cat::new("Whiskers");
    let cow = Cow::new("Bessie");

    // Static dispatch - compiler knows exact type
    println!("Static dispatch:");
    animal_speak_static(&cat);
    animal_speak_static(&cow);

    // Dynamic dispatch - runtime polymorphism
    println!("\nDynamic dispatch:");
    animal_speak_dynamic(&cat);
    animal_speak_dynamic(&cow);

    // The real power: heterogeneous collection!
    println!("\nFarm animals (mixed types in one Vec):");
    let farm = create_farm();
    for animal in &farm {
        println!("  {}", animal.describe());
    }

    // =========================================================================
    // Part 5: Standard Library Traits
    // =========================================================================
    println!("\n--- Part 5: Common Std Traits ---\n");

    let p1 = Product::new("SKU001", "Widget", 1999);
    let p2 = Product::new("SKU002", "Gadget", 2999);
    let p3 = Product::default();

    // Display
    println!("Display: {}", p1);

    // Debug
    println!("Debug: {:?}", p1);

    // Clone
    let p1_clone = p1.clone();
    println!("Cloned: {}", p1_clone);

    // PartialEq
    println!("p1 == p1_clone? {}", p1 == p1_clone);
    println!("p1 == p2? {}", p1 == p2);

    // Ord - sorting
    let mut products = vec![p2.clone(), p1.clone(), p3.clone()];
    products.sort();
    println!("\nSorted by price:");
    for p in &products {
        println!("  {}", p);
    }

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n============================================================");
    println!("📚 SUMMARY: Advanced Traits");
    println!("============================================================");
    println!(
        "
┌─────────────────────────────────────────────────────────────┐
│ MULTIPLE TRAIT BOUNDS                                       │
├─────────────────────────────────────────────────────────────┤
│ fn process<T: TraitA + TraitB>(item: &T)                    │
│ fn process<T>(item: &T) where T: TraitA + TraitB + TraitC   │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ SUPERTRAITS (Trait Inheritance)                             │
├─────────────────────────────────────────────────────────────┤
│ trait Child: Parent {{ }}   // Child requires Parent         │
│ trait A: B + C {{ }}        // A requires both B and C       │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ GENERIC STRUCTS WITH BOUNDS                                 │
├─────────────────────────────────────────────────────────────┤
│ struct Container<T: Display> {{ value: T }}                  │
│ impl<T: Clone> Container<T> {{ fn get_clone(&self) -> T }}   │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ TRAIT OBJECTS (dyn Trait)                                   │
├─────────────────────────────────────────────────────────────┤
│ Static:  fn foo<T: Trait>(x: &T)    // Compile-time type   │
│ Dynamic: fn foo(x: &dyn Trait)       // Runtime type        │
│ Collection: Vec<Box<dyn Trait>>      // Mixed types!        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ COMMON STD TRAITS                                           │
├─────────────────────────────────────────────────────────────┤
│ Debug      → {{:?}} formatting                               │
│ Display    → {{}} formatting                                 │
│ Clone      → .clone() method                                │
│ Default    → Type::default()                                │
│ PartialEq  → == and !=                                      │
│ Ord        → Sorting, comparisons                           │
│ Hash       → Use in HashMap/HashSet                         │
└─────────────────────────────────────────────────────────────┘
"
    );
}

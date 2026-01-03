// ============================================================
// DEEP DIVE: Ownership, Borrowing, Lifetimes & Smart Pointers
// ============================================================
// This is THE core of Rust. Understanding this = Understanding Rust.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn learn_ownership_deep_dive() {
    println!("\n============================================================");
    println!("  DEEP DIVE: Ownership, Borrowing, Lifetimes & Smart Pointers");
    println!("============================================================\n");

    part1_ownership_fundamentals();
    part2_move_semantics();
    part3_borrowing_rules();
    part4_lifetimes();
    part5_smart_pointers();
    part6_interior_mutability();
    part7_reference_cycles();
}

// ============================================================
// PART 1: OWNERSHIP FUNDAMENTALS
// ============================================================

fn part1_ownership_fundamentals() {
    println!("\n--- PART 1: Ownership Fundamentals ---\n");

    // THE THREE RULES OF OWNERSHIP:
    // 1. Each value in Rust has exactly ONE owner
    // 2. When the owner goes out of scope, the value is dropped
    // 3. There can only be one owner at a time

    // ----- Stack vs Heap -----
    // Stack: Fixed size, fast, LIFO. Examples: integers, booleans, chars
    // Heap: Dynamic size, slower, requires allocation. Examples: String, Vec, Box

    // Stack data - copied automatically (implements Copy trait)
    let x = 5;          // x owns 5 (on stack)
    let y = x;          // y gets a COPY of 5 (x is still valid)
    println!("Stack copy: x = {}, y = {}", x, y);  // Both valid!

    // Heap data - moved by default
    let s1 = String::from("hello");  // s1 owns the String (data on heap)
    let s2 = s1;                      // s1's ownership MOVES to s2
    // println!("{}", s1);            // ERROR! s1 is no longer valid
    println!("Heap move: s2 = {}", s2);

    // ----- Why does Rust do this? -----
    // Without move semantics, you'd have double-free:
    // s1 and s2 both pointing to same heap memory
    // When both go out of scope, Rust would try to free twice = crash!
    // Rust's solution: Only ONE owner. s1 becomes invalid after move.

    // ----- Memory Layout -----
    // String on stack: { ptr: 0x..., len: 5, capacity: 5 }
    // String data on heap: ['h', 'e', 'l', 'l', 'o']
    //
    // After move: s1's stack data is marked invalid, s2 owns everything

    // ----- The Copy Trait -----
    // Types that implement Copy are copied instead of moved:
    // - All integer types (i32, u64, etc.)
    // - bool
    // - char
    // - Floating point types (f32, f64)
    // - Tuples containing only Copy types: (i32, i32) is Copy
    //
    // Why can these types be Copy?
    // Because they're entirely on the stack - cheap to duplicate!

    #[derive(Debug, Clone, Copy)]  // We can derive Copy for simple structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;  // COPIED, not moved (because Point is Copy)
    println!("Both valid: p1 = {:?}, p2 = {:?}", p1, p2);

    // ----- Clone: Explicit Deep Copy -----
    let s3 = String::from("world");
    let s4 = s3.clone();  // Explicit deep copy
    println!("After clone: s3 = {}, s4 = {}", s3, s4);  // Both valid!
    // clone() allocates new heap memory and copies data
}

// ============================================================
// PART 2: MOVE SEMANTICS IN DEPTH
// ============================================================

fn part2_move_semantics() {
    println!("\n--- PART 2: Move Semantics in Depth ---\n");

    // ----- Moves happen in many places -----

    // 1. Assignment
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // MOVE
    // v1 is now invalid

    // 2. Function calls
    fn take_ownership(v: Vec<i32>) {
        println!("I now own: {:?}", v);
    }  // v is dropped here

    take_ownership(v2);  // v2 moves into function
    // v2 is now invalid

    // 3. Returning from functions
    fn give_ownership() -> Vec<i32> {
        vec![4, 5, 6]  // ownership moves OUT to caller
    }

    let v3 = give_ownership();  // v3 now owns the vector
    println!("Received ownership: {:?}", v3);

    // 4. Struct/enum construction
    let name = String::from("Alice");
    let person = Person { name };  // name moves into struct
    // name is now invalid
    println!("Person: {:?}", person);

    // ----- Partial Moves -----
    #[derive(Debug)]
    struct Container {
        data: String,
        id: i32,
    }

    let container = Container {
        data: String::from("important"),
        id: 42,
    };

    // Move just one field
    let data = container.data;  // data field is moved out
    // container.data is now invalid
    // But container.id is still valid!
    println!("Moved data: {}", data);
    println!("Still valid id: {}", container.id);
    // println!("{:?}", container);  // ERROR! Can't use container as whole

    // ----- Move in Loops -----
    let strings = vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ];

    // This moves each string out of the vector
    for s in strings {  // strings is moved here
        println!("Moved string: {}", s);
    }
    // strings is now invalid

    // To iterate without moving, use references:
    let strings2 = vec![String::from("x"), String::from("y")];
    for s in &strings2 {  // Borrowing!
        println!("Borrowed: {}", s);
    }
    println!("Still valid: {:?}", strings2);  // strings2 still valid!
}

#[derive(Debug)]
struct Person {
    name: String,
}

// ============================================================
// PART 3: BORROWING RULES
// ============================================================

fn part3_borrowing_rules() {
    println!("\n--- PART 3: Borrowing Rules ---\n");

    // THE BORROWING RULES:
    // 1. At any given time, you can have EITHER:
    //    - One mutable reference (&mut T), OR
    //    - Any number of immutable references (&T)
    // 2. References must always be valid (no dangling references)

    // ----- Immutable Borrows (Shared References) -----
    let data = String::from("hello");

    let r1 = &data;  // First immutable borrow
    let r2 = &data;  // Second immutable borrow - OK!
    let r3 = &data;  // Third - still OK!

    println!("Multiple readers: {}, {}, {}", r1, r2, r3);
    // All three can read, none can modify

    // ----- Mutable Borrows (Exclusive References) -----
    let mut data2 = String::from("hello");

    let m1 = &mut data2;  // Mutable borrow
    // let m2 = &mut data2;  // ERROR! Can't have two mutable borrows
    // let r = &data2;       // ERROR! Can't mix mutable and immutable

    m1.push_str(" world");
    println!("Modified: {}", m1);

    // After m1 is no longer used, we can borrow again
    let r = &data2;
    println!("Now immutable borrow works: {}", r);

    // ----- Non-Lexical Lifetimes (NLL) -----
    // Rust is smart! References are considered "dead" when last used,
    // not when they go out of scope.

    let mut s = String::from("test");

    let r1 = &s;           // Immutable borrow starts
    println!("{}", r1);    // Last use of r1
    // r1's borrow is now considered ended (NLL)

    let m = &mut s;        // Mutable borrow - OK! r1 is "dead"
    m.push_str("ing");
    println!("{}", m);

    // ----- Borrowing in Functions -----
    fn print_len(s: &String) {  // Borrows, doesn't take ownership
        println!("Length: {}", s.len());
    }  // s goes out of scope, but since it was a reference, nothing happens

    let my_string = String::from("hello");
    print_len(&my_string);  // Pass a reference
    println!("Still mine: {}", my_string);  // my_string still valid!

    // ----- Mutable References in Functions -----
    fn append_world(s: &mut String) {
        s.push_str(" world");
    }

    let mut my_string2 = String::from("hello");
    append_world(&mut my_string2);  // Pass mutable reference
    println!("Modified: {}", my_string2);

    // ----- The Slice Pattern -----
    // Slices are references to a portion of a collection
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];  // References elements 1, 2, 3
    println!("Slice: {:?}", slice);

    let s = String::from("hello world");
    let hello: &str = &s[0..5];     // String slice
    let world: &str = &s[6..11];
    println!("Slices: '{}' and '{}'", hello, world);

    // ----- Why These Rules? -----
    // They prevent:
    // 1. Data races (two writers, or reader + writer simultaneously)
    // 2. Use-after-free (dangling pointers)
    // 3. Iterator invalidation
    //
    // Example of what Rust prevents:
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];      // Immutable borrow
    // v.push(4);              // ERROR! Can't mutate while borrowed
    // println!("{}", first);   // first might be invalid after push!
}

// ============================================================
// PART 4: LIFETIMES
// ============================================================

fn part4_lifetimes() {
    println!("\n--- PART 4: Lifetimes ---\n");

    // WHAT ARE LIFETIMES?
    // - Every reference has a lifetime (how long it's valid)
    // - Usually Rust infers lifetimes automatically
    // - Sometimes we need to annotate them explicitly

    // ----- The Problem Lifetimes Solve -----
    // fn get_longer(a: &str, b: &str) -> &str {
    //     if a.len() > b.len() { a } else { b }
    // }
    // This won't compile! Rust doesn't know if the returned reference
    // should live as long as 'a' or as long as 'b'

    // ----- Solution: Explicit Lifetime Annotations -----
    fn get_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() { a } else { b }
    }
    // 'a is a lifetime parameter
    // This says: the returned reference lives as long as BOTH inputs

    let string1 = String::from("long string");
    let string2 = String::from("short");
    let result = get_longer(&string1, &string2);
    println!("Longer: {}", result);

    // ----- Lifetime Annotations DON'T Change How Long Values Live -----
    // They just describe relationships between lifetimes
    // They help the compiler verify references are valid

    // ----- Different Lifetime Relationships -----
    fn first_word<'a>(s: &'a str) -> &'a str {
        s.split_whitespace().next().unwrap_or("")
    }
    // Return lifetime is tied to input lifetime

    // Multiple lifetime parameters when needed:
    fn _complex<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
        x  // Only returns x, so only needs 'a
    }

    // ----- Lifetimes in Structs -----
    // If a struct holds references, it needs lifetime annotations
    #[derive(Debug)]
    struct Excerpt<'a> {
        part: &'a str,  // This reference must outlive the struct
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence };
    println!("Excerpt: {:?}", excerpt);
    // excerpt cannot outlive novel (because it borrows from it)

    // ----- Lifetime Elision Rules -----
    // Rust has 3 rules that often let you skip lifetime annotations:
    //
    // Rule 1: Each reference parameter gets its own lifetime
    //   fn foo(x: &str, y: &str) becomes fn foo<'a, 'b>(x: &'a str, y: &'b str)
    //
    // Rule 2: If there's exactly one input lifetime, it's assigned to all output lifetimes
    //   fn foo(x: &str) -> &str becomes fn foo<'a>(x: &'a str) -> &'a str
    //
    // Rule 3: If one of the parameters is &self or &mut self, its lifetime
    //         is assigned to all output lifetimes
    //   impl Foo { fn method(&self) -> &str } works without annotations

    // ----- The 'static Lifetime -----
    // 'static means the reference lives for the entire program
    let s: &'static str = "I live forever!";  // String literals are 'static
    println!("{}", s);

    // You can also make owned data 'static with Box::leak (rarely needed)
    let leaked: &'static str = Box::leak(String::from("leaked").into_boxed_str());
    println!("Leaked static: {}", leaked);

    // ----- Lifetime Bounds on Generics -----
    fn _print_ref<'a, T: std::fmt::Display + 'a>(x: &'a T) {
        println!("{}", x);
    }
    // T: 'a means T must not contain references shorter than 'a

    // ----- Common Lifetime Patterns -----

    // Pattern 1: Same lifetime for input and output
    fn _identity<'a>(x: &'a str) -> &'a str { x }

    // Pattern 2: Struct that borrows data
    struct Parser<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> Parser<'a> {
        fn new(input: &'a str) -> Parser<'a> {
            Parser { input, pos: 0 }
        }

        fn current(&self) -> Option<char> {
            self.input.chars().nth(self.pos)
        }
    }

    let text = String::from("hello");
    let parser = Parser::new(&text);
    println!("First char: {:?}", parser.current());
}

// ============================================================
// PART 5: SMART POINTERS
// ============================================================

fn part5_smart_pointers() {
    println!("\n--- PART 5: Smart Pointers ---\n");

    // Smart pointers are structs that act like pointers but have extra capabilities
    // They implement Deref (act like references) and Drop (custom cleanup)

    // ----- Box<T>: Heap Allocation -----
    // Puts data on the heap instead of stack
    // Use cases:
    // 1. When size isn't known at compile time
    // 2. Large data you don't want to copy
    // 3. Recursive types

    let b = Box::new(5);  // 5 is on the heap
    println!("Box value: {}", b);  // Deref lets us use it like a reference

    // Recursive types require Box
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3, 
                Box::new(List::Nil))))));
    println!("Recursive list: {:?}", list);

    // Box for trait objects
    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) { println!("Woof!"); }
    }
    impl Animal for Cat {
        fn speak(&self) { println!("Meow!"); }
    }

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];
    for animal in &animals {
        animal.speak();
    }

    // ----- Rc<T>: Reference Counting (Single-Threaded) -----
    // Multiple ownership for single-threaded scenarios
    // Keeps track of how many references exist

    let data = Rc::new(String::from("shared data"));
    println!("Reference count: {}", Rc::strong_count(&data));

    let data2 = Rc::clone(&data);  // Increases ref count, doesn't deep copy
    println!("After clone: {}", Rc::strong_count(&data));

    let data3 = Rc::clone(&data);
    println!("After another clone: {}", Rc::strong_count(&data));

    drop(data3);
    println!("After dropping one: {}", Rc::strong_count(&data));

    // ----- Rc for Shared Data Structures -----
    #[derive(Debug)]
    enum SharedList {
        Cons(i32, Rc<SharedList>),
        Nil,
    }

    // Two lists sharing the same tail
    let tail = Rc::new(SharedList::Cons(3, 
        Rc::new(SharedList::Nil)));
    
    let list1 = SharedList::Cons(1, Rc::clone(&tail));
    let list2 = SharedList::Cons(2, Rc::clone(&tail));
    
    println!("list1: {:?}", list1);
    println!("list2: {:?}", list2);
    println!("Tail ref count: {}", Rc::strong_count(&tail));

    // ----- Arc<T>: Atomic Reference Counting (Thread-Safe) -----
    // Like Rc but safe to share across threads
    use std::sync::Arc;
    use std::thread;

    let arc_data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        let arc_clone = Arc::clone(&arc_data);
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, arc_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ============================================================
// PART 6: INTERIOR MUTABILITY
// ============================================================

fn part6_interior_mutability() {
    println!("\n--- PART 6: Interior Mutability ---\n");

    // Interior mutability allows mutation through shared references
    // It moves borrow checking from compile-time to runtime

    // ----- RefCell<T>: Runtime Borrow Checking -----
    // Single-threaded only!
    // Panics at runtime if borrowing rules are violated

    let data = RefCell::new(5);

    // Immutable borrow (read)
    {
        let borrowed = data.borrow();
        println!("Borrowed immutably: {}", *borrowed);
    }  // borrowed goes out of scope

    // Mutable borrow (write)
    {
        let mut borrowed_mut = data.borrow_mut();
        *borrowed_mut += 1;
        println!("After mutation: {}", *borrowed_mut);
    }

    // Multiple immutable borrows are OK
    {
        let b1 = data.borrow();
        let b2 = data.borrow();
        println!("Multiple borrows: {} and {}", b1, b2);
    }

    // This would PANIC at runtime:
    // let borrow1 = data.borrow();
    // let borrow2 = data.borrow_mut();  // PANIC! Can't borrow mutably while borrowed immutably

    // ----- Rc<RefCell<T>>: Shared Mutable State -----
    // Combine Rc (multiple owners) with RefCell (mutability)

    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let owner1 = Rc::clone(&shared);
    let owner2 = Rc::clone(&shared);

    // Both owners can mutate!
    owner1.borrow_mut().push(4);
    owner2.borrow_mut().push(5);

    println!("Shared mutable data: {:?}", shared.borrow());

    // ----- Use Case: Mocking in Tests -----
    trait Logger {
        fn log(&self, message: &str);
    }

    struct MockLogger {
        messages: RefCell<Vec<String>>,  // Can mutate through &self
    }

    impl Logger for MockLogger {
        fn log(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    let logger = MockLogger {
        messages: RefCell::new(vec![]),
    };

    logger.log("First message");  // Works even though log takes &self
    logger.log("Second message");

    println!("Logged messages: {:?}", logger.messages.borrow());

    // ----- Cell<T>: For Copy Types -----
    // Like RefCell but for Copy types, without borrowing overhead
    use std::cell::Cell;

    let counter = Cell::new(0);
    counter.set(counter.get() + 1);
    counter.set(counter.get() + 1);
    println!("Cell counter: {}", counter.get());
}

// ============================================================
// PART 7: REFERENCE CYCLES & WEAK REFERENCES
// ============================================================

fn part7_reference_cycles() {
    println!("\n--- PART 7: Reference Cycles & Weak References ---\n");

    // Rc can create memory leaks through reference cycles!
    // Weak<T> breaks cycles

    // ----- The Problem: Reference Cycles -----
    // If A points to B and B points to A, neither can be dropped

    // ----- Solution: Weak<T> -----
    // Weak references don't keep the value alive
    // Must call .upgrade() to use (returns Option<Rc<T>>)

    // Example: Tree with parent references
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,      // Weak to parent
        children: RefCell<Vec<Rc<Node>>>,  // Strong to children
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Leaf strong count: {}, weak count: {}",
             Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Set leaf's parent (using Weak to avoid cycle)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Branch strong: {}, weak: {}",
             Rc::strong_count(&branch), Rc::weak_count(&branch));

    // When branch goes out of scope, it can be dropped
    // even though leaf.parent still has a Weak reference

    // ----- Weak Pointer Patterns -----
    // 1. Observer pattern: observers hold Weak<Subject>
    // 2. Caches: cache holds Weak, returns None if expired
    // 3. Any circular structure: use Weak for the "back" reference

    // ----- Checking if Weak is still valid -----
    let data = Rc::new(42);
    let weak = Rc::downgrade(&data);

    // While data exists
    if let Some(strong) = weak.upgrade() {
        println!("Still alive: {}", strong);
    }

    drop(data);  // Drop the Rc

    // After data is dropped
    if weak.upgrade().is_none() {
        println!("Data was dropped, Weak is now invalid");
    }
}

// ============================================================
// SUMMARY: When to Use What
// ============================================================
//
// OWNERSHIP PATTERNS:
// - Default: Use owned values (T), move semantics
// - Borrowing: Use &T or &mut T when possible
// - Clone: When you need independent copies
//
// SMART POINTERS:
// - Box<T>: Heap allocation, recursive types, trait objects
// - Rc<T>: Multiple owners, single-threaded
// - Arc<T>: Multiple owners, multi-threaded
// - RefCell<T>: Interior mutability, single-threaded
// - Mutex<T>: Interior mutability, multi-threaded
//
// LIFETIMES:
// - Usually inferred, annotate when compiler asks
// - Struct with references: struct Foo<'a> { field: &'a T }
// - Functions returning references: fn foo<'a>(x: &'a T) -> &'a T
// - 'static: Lives for entire program
//
// COMMON COMBINATIONS:
// - Rc<RefCell<T>>: Shared mutable state (single-threaded)
// - Arc<Mutex<T>>: Shared mutable state (multi-threaded)
// - Weak<T>: Break reference cycles
// - Box<dyn Trait>: Trait objects on heap
// ============================================================

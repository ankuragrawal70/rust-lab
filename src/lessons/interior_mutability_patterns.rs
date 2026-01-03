// ============================================================
// INTERIOR MUTABILITY & REFERENCE CYCLES - Deep Dive
// ============================================================
// Understanding RefCell, Cell, and how to handle reference cycles.
//
// INTERIOR MUTABILITY = mutating data through shared (&T) references
// This is normally forbidden! But sometimes necessary.
//
// Rust's answer: Move borrow checking from COMPILE-TIME to RUNTIME
// If you violate rules at runtime → panic instead of undefined behavior

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

pub fn learn_interior_mutability_patterns() {
    println!("\n============================================================");
    println!("  INTERIOR MUTABILITY & REFERENCE CYCLES");
    println!("============================================================\n");

    part1_the_problem();
    part2_refcell_deep_dive();
    part3_cell_for_copy_types();
    part4_rc_refcell_pattern();
    part5_reference_cycles_problem();
    part6_weak_references_solution();
    part7_real_world_patterns();
}

// ============================================================
// PART 1: THE PROBLEM INTERIOR MUTABILITY SOLVES
// ============================================================

fn part1_the_problem() {
    println!("--- PART 1: The Problem ---\n");

    // NORMAL RUST RULES:
    // - &T = shared reference = read-only
    // - &mut T = exclusive reference = read-write
    //
    // This is great for safety, but sometimes too restrictive!

    // EXAMPLE: Mock object for testing
    // You have a trait like this:
    trait Messenger {
        fn send(&self, msg: &str);  // Takes &self, not &mut self
    }

    // Real implementation sends to network
    // For testing, you want a mock that RECORDS messages
    // But how? send() only has &self!

    // BAD: This won't compile
    // struct MockMessenger {
    //     messages: Vec<String>,
    // }
    // impl Messenger for MockMessenger {
    //     fn send(&self, msg: &str) {
    //         self.messages.push(msg.to_string());  // ERROR! Can't mutate through &self
    //     }
    // }

    // SOLUTION: RefCell provides interior mutability
    struct MockMessenger {
        messages: RefCell<Vec<String>>,  // Wrap in RefCell
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // borrow_mut() gives us &mut to the inner Vec
            self.messages.borrow_mut().push(msg.to_string());  // Works!
        }
    }

    let mock = MockMessenger {
        messages: RefCell::new(vec![]),
    };

    mock.send("Hello");
    mock.send("World");

    println!("Messages recorded: {:?}", mock.messages.borrow());

    // The borrowing rules are STILL enforced - just at RUNTIME
    // If you borrow_mut() twice at same time → PANIC
}

// ============================================================
// PART 2: RefCell<T> DEEP DIVE
// ============================================================

fn part2_refcell_deep_dive() {
    println!("\n--- PART 2: RefCell<T> Deep Dive ---\n");

    // RefCell tracks borrows at RUNTIME using counters:
    // - borrow() → increments reader count, returns Ref<T>
    // - borrow_mut() → sets "exclusive" flag, returns RefMut<T>
    // - When Ref/RefMut is dropped, counter is decremented

    let data = RefCell::new(String::from("hello"));

    // ===== IMMUTABLE BORROWS =====
    {
        let r1 = data.borrow();  // Reader count: 1
        let r2 = data.borrow();  // Reader count: 2 (OK!)
        let r3 = data.borrow();  // Reader count: 3 (OK!)

        println!("Multiple readers: {}, {}, {}", r1, r2, r3);
        
        // All Ref guards dropped here, reader count → 0
    }

    // ===== MUTABLE BORROWS =====
    {
        let mut writer = data.borrow_mut();  // Exclusive access
        writer.push_str(" world");
        println!("After mutation: {}", writer);
        
        // RefMut guard dropped here
    }

    // ===== THE PANIC SCENARIO =====
    println!("\n--- Demonstrating borrow rules (would panic) ---");
    
    // This would PANIC at runtime:
    // let r1 = data.borrow();
    // let w1 = data.borrow_mut();  // PANIC! Already borrowed immutably
    
    // This would also PANIC:
    // let w1 = data.borrow_mut();
    // let w2 = data.borrow_mut();  // PANIC! Already borrowed mutably

    println!("(Skipped panic examples - they would crash the program)");

    // ===== try_borrow() AND try_borrow_mut() =====
    // Non-panicking versions that return Result
    
    let data2 = RefCell::new(42);
    
    let borrowed = data2.borrow();
    
    // Try to borrow mutably while already borrowed
    match data2.try_borrow_mut() {
        Ok(_) => println!("Got mutable borrow"),
        Err(_) => println!("Couldn't borrow mutably - already borrowed"),
    }

    drop(borrowed);  // Release immutable borrow

    match data2.try_borrow_mut() {
        Ok(mut val) => {
            *val = 100;
            println!("Now got mutable borrow! Value: {}", *val);
        }
        Err(_) => println!("Still couldn't borrow"),
    }

    // ===== RefCell METHODS =====
    println!("\n--- RefCell Methods ---");

    let cell = RefCell::new(5);

    // into_inner() - Consumes RefCell, returns inner value
    // (Useful when you're done with the RefCell)
    let inner = cell.into_inner();
    println!("into_inner: {}", inner);

    // replace() - Replaces value, returns old value
    let cell2 = RefCell::new(10);
    let old = cell2.replace(20);
    println!("replace: old={}, new={}", old, *cell2.borrow());

    // swap() - Swaps contents of two RefCells
    let a = RefCell::new(1);
    let b = RefCell::new(2);
    a.swap(&b);
    println!("After swap: a={}, b={}", a.borrow(), b.borrow());
}

// ============================================================
// PART 3: Cell<T> FOR COPY TYPES
// ============================================================

fn part3_cell_for_copy_types() {
    println!("\n--- PART 3: Cell<T> for Copy Types ---\n");

    // Cell<T> is simpler than RefCell<T>
    // Works only with Copy types (integers, bools, etc.)
    // No borrowing needed - just get/set

    // WHY Cell?
    // - No runtime borrow tracking overhead
    // - No Ref/RefMut guards to manage
    // - Just copies values in and out

    let counter = Cell::new(0);

    // get() returns a COPY of the value
    println!("Initial: {}", counter.get());

    // set() replaces the value
    counter.set(counter.get() + 1);
    counter.set(counter.get() + 1);
    counter.set(counter.get() + 1);

    println!("After 3 increments: {}", counter.get());

    // ===== Cell IN STRUCTS =====
    struct Point {
        x: Cell<i32>,
        y: Cell<i32>,
    }

    let point = Point {
        x: Cell::new(0),
        y: Cell::new(0),
    };

    // Mutate through shared reference!
    fn move_point(p: &Point, dx: i32, dy: i32) {
        p.x.set(p.x.get() + dx);
        p.y.set(p.y.get() + dy);
    }

    move_point(&point, 10, 20);
    println!("Point: ({}, {})", point.x.get(), point.y.get());

    // ===== Cell vs RefCell =====
    println!("\n--- Cell vs RefCell ---");
    println!("┌────────────┬───────────────────────────┬───────────────────────────┐");
    println!("│            │ Cell<T>                   │ RefCell<T>                │");
    println!("├────────────┼───────────────────────────┼───────────────────────────┤");
    println!("│ Types      │ Only Copy types           │ Any type                  │");
    println!("│ API        │ get()/set() (copies)      │ borrow()/borrow_mut()     │");
    println!("│ Overhead   │ None                      │ Runtime borrow checking   │");
    println!("│ Panic?     │ Never panics              │ Panics on rule violation  │");
    println!("│ Use case   │ Counters, flags           │ Complex data, collections │");
    println!("└────────────┴───────────────────────────┴───────────────────────────┘");

    // ===== Cell::update() =====
    // Convenient for modifying in place (requires nightly or recent stable)
    let val = Cell::new(5);
    val.set(val.get() * 2);  // Manual update
    println!("After *2: {}", val.get());

    // ===== Cell FOR LAZY INITIALIZATION =====
    struct LazyValue {
        computed: Cell<Option<i32>>,
    }

    impl LazyValue {
        fn get(&self) -> i32 {
            match self.computed.get() {
                Some(v) => v,
                None => {
                    let value = 42;  // Expensive computation
                    self.computed.set(Some(value));
                    value
                }
            }
        }
    }

    let lazy = LazyValue { computed: Cell::new(None) };
    println!("First access (computes): {}", lazy.get());
    println!("Second access (cached): {}", lazy.get());
}

// ============================================================
// PART 4: Rc<RefCell<T>> PATTERN
// ============================================================

fn part4_rc_refcell_pattern() {
    println!("\n--- PART 4: Rc<RefCell<T>> Pattern ---\n");

    // PROBLEM:
    // - Rc<T> allows multiple owners but only shared access
    // - RefCell<T> allows mutation but only single owner
    //
    // SOLUTION: Combine them!
    // Rc<RefCell<T>> = Multiple owners + Interior mutability

    // ===== SIMPLE EXAMPLE =====
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Clone gives another owner
    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);
    let owner3 = Rc::clone(&shared_data);

    // Each owner can mutate!
    owner1.borrow_mut().push(4);
    owner2.borrow_mut().push(5);
    owner3.borrow_mut().push(6);

    println!("Shared data: {:?}", shared_data.borrow());
    println!("All three owners see the same data!");

    // ===== BUILDING A SHARED MUTABLE LIST =====
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<RefCell<Node>>>,
    }

    let node3 = Rc::new(RefCell::new(Node { value: 3, next: None }));
    let node2 = Rc::new(RefCell::new(Node { value: 2, next: Some(Rc::clone(&node3)) }));
    let node1 = Rc::new(RefCell::new(Node { value: 1, next: Some(Rc::clone(&node2)) }));

    println!("\nLinked list:");
    
    // Traverse and print
    let mut current = Some(Rc::clone(&node1));
    while let Some(node) = current {
        print!("{} -> ", node.borrow().value);
        current = node.borrow().next.clone();
    }
    println!("None");

    // Mutate node2's value through node1!
    if let Some(ref next) = node1.borrow().next {
        next.borrow_mut().value = 200;
    }

    println!("After mutating node2:");
    println!("node2.value = {}", node2.borrow().value);

    // ===== OBSERVER PATTERN =====
    println!("\n--- Observer Pattern with Rc<RefCell<T>> ---");

    trait Observer {
        fn update(&mut self, value: i32);
    }

    struct Subject {
        observers: Vec<Rc<RefCell<dyn Observer>>>,
        value: i32,
    }

    impl Subject {
        fn new() -> Self {
            Subject { observers: vec![], value: 0 }
        }

        fn attach(&mut self, observer: Rc<RefCell<dyn Observer>>) {
            self.observers.push(observer);
        }

        fn set_value(&mut self, value: i32) {
            self.value = value;
            self.notify();
        }

        fn notify(&self) {
            for observer in &self.observers {
                observer.borrow_mut().update(self.value);
            }
        }
    }

    struct Logger {
        name: String,
        log: Vec<i32>,
    }

    impl Observer for Logger {
        fn update(&mut self, value: i32) {
            println!("{} received: {}", self.name, value);
            self.log.push(value);
        }
    }

    let logger1 = Rc::new(RefCell::new(Logger {
        name: String::from("Logger1"),
        log: vec![],
    }));
    let logger2 = Rc::new(RefCell::new(Logger {
        name: String::from("Logger2"),
        log: vec![],
    }));

    let mut subject = Subject::new();
    subject.attach(Rc::clone(&logger1) as Rc<RefCell<dyn Observer>>);
    subject.attach(Rc::clone(&logger2) as Rc<RefCell<dyn Observer>>);

    subject.set_value(10);
    subject.set_value(20);

    println!("Logger1 log: {:?}", logger1.borrow().log);
    println!("Logger2 log: {:?}", logger2.borrow().log);
}

// ============================================================
// PART 5: REFERENCE CYCLES - THE MEMORY LEAK PROBLEM
// ============================================================

fn part5_reference_cycles_problem() {
    println!("\n--- PART 5: Reference Cycles - Memory Leak ---\n");

    // THE PROBLEM:
    // If A has Rc to B, and B has Rc to A:
    // - A's count never drops to 0 (B holds a reference)
    // - B's count never drops to 0 (A holds a reference)
    // - NEITHER CAN BE FREED = MEMORY LEAK!

    // Let's create a cycle (intentionally)
    #[derive(Debug)]
    struct CycleNode {
        value: i32,
        next: RefCell<Option<Rc<CycleNode>>>,
    }

    impl Drop for CycleNode {
        fn drop(&mut self) {
            println!("Dropping CycleNode with value: {}", self.value);
        }
    }

    println!("Creating nodes:");
    let a = Rc::new(CycleNode {
        value: 1,
        next: RefCell::new(None),
    });
    println!("  a created, count: {}", Rc::strong_count(&a));

    let b = Rc::new(CycleNode {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&a))),  // b -> a
    });
    println!("  b created (points to a), a count: {}", Rc::strong_count(&a));

    // Create the cycle: a -> b
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    println!("  a now points to b");
    println!("  a count: {}, b count: {}", 
             Rc::strong_count(&a), Rc::strong_count(&b));

    // Now we have: a <-> b (cycle!)

    println!("\nDropping a and b references...");
    // When a and b go out of scope:
    // - a's local Rc is dropped: a count goes from 2 to 1 (b still has reference)
    // - b's local Rc is dropped: b count goes from 2 to 1 (a still has reference)
    // NEITHER reaches 0! Memory leak!

    println!("(In real code, this would leak memory)");
    println!("Notice: Drop is never called because cycle keeps refs alive!");

    // Note: In this example, we'll force a manual break to avoid the leak
    *a.next.borrow_mut() = None;  // Break the cycle
    *b.next.borrow_mut() = None;  // Break the cycle
    // Now they can be dropped
}

// ============================================================
// PART 6: WEAK REFERENCES - THE SOLUTION
// ============================================================

fn part6_weak_references_solution() {
    println!("\n--- PART 6: Weak<T> - Breaking Cycles ---\n");

    // SOLUTION: Use Weak<T> for "back" references
    //
    // Weak<T> is a non-owning reference:
    // - Doesn't increment strong_count
    // - Increments weak_count instead
    // - Value can be dropped while Weak exists
    // - Must call .upgrade() to use (returns Option<Rc<T>>)

    // ===== HOW WEAK WORKS =====
    let strong = Rc::new(String::from("I exist"));
    println!("Strong count: {}, Weak count: {}", 
             Rc::strong_count(&strong), Rc::weak_count(&strong));

    let weak = Rc::downgrade(&strong);  // Create Weak from Rc
    println!("After downgrade:");
    println!("  Strong count: {}, Weak count: {}", 
             Rc::strong_count(&strong), Rc::weak_count(&strong));

    // Use Weak - must upgrade first
    if let Some(rc) = weak.upgrade() {
        println!("Upgraded successfully: {}", rc);
    }

    drop(strong);  // Drop the Rc

    // Now upgrade fails
    match weak.upgrade() {
        Some(_) => println!("Still exists"),
        None => println!("Value was dropped! Weak is now invalid."),
    }

    // ===== TREE WITH PARENT POINTERS =====
    println!("\n--- Tree with Parent Pointers ---");

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        parent: RefCell<Weak<TreeNode>>,       // Weak to parent!
        children: RefCell<Vec<Rc<TreeNode>>>,  // Strong to children
    }

    // Build tree:
    //       root (1)
    //       /    \
    //   child1   child2
    //    (2)      (3)

    let root = Rc::new(TreeNode {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let child1 = Rc::new(TreeNode {
        value: 2,
        parent: RefCell::new(Rc::downgrade(&root)),  // Weak reference to parent
        children: RefCell::new(vec![]),
    });

    let child2 = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Rc::downgrade(&root)),  // Weak reference to parent
        children: RefCell::new(vec![]),
    });

    // Add children to root
    root.children.borrow_mut().push(Rc::clone(&child1));
    root.children.borrow_mut().push(Rc::clone(&child2));

    println!("Tree structure:");
    println!("  Root ({})", root.value);
    println!("  Root strong count: {}, weak count: {}", 
             Rc::strong_count(&root), Rc::weak_count(&root));
    
    for child in root.children.borrow().iter() {
        println!("    Child ({})", child.value);
        
        // Access parent through Weak
        if let Some(parent) = child.parent.borrow().upgrade() {
            println!("      -> Parent value: {}", parent.value);
        }
    }

    // When root goes out of scope, everything can be cleaned up
    // because children only have Weak references back to root

    // ===== DOUBLY LINKED LIST =====
    println!("\n--- Doubly Linked List ---");

    #[derive(Debug)]
    struct DLLNode {
        value: i32,
        prev: RefCell<Weak<DLLNode>>,          // Weak to previous
        next: RefCell<Option<Rc<DLLNode>>>,    // Strong to next
    }

    // Build: None <- 1 <-> 2 <-> 3 -> None
    let node1 = Rc::new(DLLNode {
        value: 1,
        prev: RefCell::new(Weak::new()),
        next: RefCell::new(None),
    });

    let node2 = Rc::new(DLLNode {
        value: 2,
        prev: RefCell::new(Rc::downgrade(&node1)),
        next: RefCell::new(None),
    });

    let node3 = Rc::new(DLLNode {
        value: 3,
        prev: RefCell::new(Rc::downgrade(&node2)),
        next: RefCell::new(None),
    });

    // Set next pointers
    *node1.next.borrow_mut() = Some(Rc::clone(&node2));
    *node2.next.borrow_mut() = Some(Rc::clone(&node3));

    // Traverse forward
    print!("Forward: ");
    let mut current = Some(Rc::clone(&node1));
    while let Some(node) = current {
        print!("{} -> ", node.value);
        current = node.next.borrow().clone();
    }
    println!("None");

    // Traverse backward from node3
    print!("Backward from 3: ");
    let mut current = Some(Rc::clone(&node3));
    while let Some(node) = current {
        print!("{} -> ", node.value);
        current = node.prev.borrow().upgrade();
    }
    println!("None");
}

// ============================================================
// PART 7: REAL-WORLD PATTERNS
// ============================================================

fn part7_real_world_patterns() {
    println!("\n--- PART 7: Real-World Patterns ---\n");

    // ===== PATTERN 1: CACHING WITH WEAK =====
    println!("--- Pattern 1: Cache with Weak References ---");

    struct Cache {
        entries: RefCell<Vec<Weak<String>>>,
    }

    impl Cache {
        fn new() -> Self {
            Cache { entries: RefCell::new(vec![]) }
        }

        fn add(&self, item: &Rc<String>) {
            self.entries.borrow_mut().push(Rc::downgrade(item));
        }

        fn get_all_valid(&self) -> Vec<Rc<String>> {
            self.entries
                .borrow()
                .iter()
                .filter_map(|w| w.upgrade())
                .collect()
        }

        fn cleanup(&self) {
            self.entries.borrow_mut().retain(|w| w.upgrade().is_some());
        }
    }

    let cache = Cache::new();

    let item1 = Rc::new(String::from("Item 1"));
    let item2 = Rc::new(String::from("Item 2"));
    let item3 = Rc::new(String::from("Item 3"));

    cache.add(&item1);
    cache.add(&item2);
    cache.add(&item3);

    println!("Cache entries: {:?}", cache.get_all_valid());

    drop(item2);  // Drop one item
    println!("After dropping item2: {:?}", cache.get_all_valid());

    cache.cleanup();
    println!("After cleanup: {:?}", cache.get_all_valid());

    // ===== PATTERN 2: EVENT SYSTEM =====
    println!("\n--- Pattern 2: Event System ---");

    type Callback = Box<dyn Fn(i32)>;

    struct EventEmitter {
        listeners: RefCell<Vec<Callback>>,
    }

    impl EventEmitter {
        fn new() -> Self {
            EventEmitter { listeners: RefCell::new(vec![]) }
        }

        fn on(&self, callback: Callback) {
            self.listeners.borrow_mut().push(callback);
        }

        fn emit(&self, value: i32) {
            for callback in self.listeners.borrow().iter() {
                callback(value);
            }
        }
    }

    let emitter = EventEmitter::new();
    
    emitter.on(Box::new(|v| println!("Listener 1 received: {}", v)));
    emitter.on(Box::new(|v| println!("Listener 2 received: {}", v * 2)));

    emitter.emit(42);

    // ===== PATTERN 3: LAZY INITIALIZATION =====
    println!("\n--- Pattern 3: Lazy Initialization ---");

    struct LazyComputed<T> {
        value: RefCell<Option<T>>,
        compute: fn() -> T,
    }

    impl<T: Clone> LazyComputed<T> {
        fn new(compute: fn() -> T) -> Self {
            LazyComputed {
                value: RefCell::new(None),
                compute,
            }
        }

        fn get(&self) -> T {
            let mut value = self.value.borrow_mut();
            if value.is_none() {
                println!("  (Computing value...)");
                *value = Some((self.compute)());
            }
            value.clone().unwrap()
        }
    }

    fn expensive_computation() -> i32 {
        // Simulate expensive work
        42 * 2
    }

    let lazy = LazyComputed::new(expensive_computation);
    
    println!("First access: {}", lazy.get());
    println!("Second access: {}", lazy.get());
    println!("Third access: {}", lazy.get());

    // ===== SUMMARY =====
    println!("\n--- Interior Mutability Summary ---");
    println!("┌─────────────────────────┬───────────────────────────────────┐");
    println!("│ Pattern                 │ Use Case                          │");
    println!("├─────────────────────────┼───────────────────────────────────┤");
    println!("│ RefCell<T>              │ Mutate through &self              │");
    println!("│ Cell<T>                 │ Simple counters/flags             │");
    println!("│ Rc<RefCell<T>>          │ Shared + mutable data             │");
    println!("│ Weak<T>                 │ Break cycles, caches              │");
    println!("│ Arc<Mutex<T>>           │ Thread-safe shared mutable        │");
    println!("└─────────────────────────┴───────────────────────────────────┘");
}

// ============================================================
// MEMORY SAFETY GUARANTEES
// ============================================================
//
// Interior mutability does NOT break Rust's safety guarantees:
//
// 1. NO DATA RACES
//    - RefCell is single-threaded only
//    - For threads, use Mutex/RwLock which use locks
//
// 2. NO DANGLING POINTERS
//    - Weak::upgrade() returns Option, must check if valid
//    - Rc ensures data lives as long as any strong ref exists
//
// 3. NO USE-AFTER-FREE
//    - Drop is still automatic
//    - Borrow checker still works (just at runtime)
//
// 4. PANICS INSTEAD OF UB
//    - If you violate borrowing rules → predictable panic
//    - NOT undefined behavior like C/C++
//
// ============================================================

// ============================================================
// SMART POINTERS EXPLAINED - Complete Deep Dive
// ============================================================
// What they are, why we need them, and when to use each one.
//
// A "smart pointer" is a struct that:
// 1. Acts like a pointer (stores a memory address)
// 2. Has additional metadata/capabilities
// 3. Implements Deref trait (so you can use * operator)
// 4. Implements Drop trait (custom cleanup when going out of scope)
//
// Regular references (&T) are "dumb" pointers - they just point.
// Smart pointers OWN the data they point to.

use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

pub fn learn_smart_pointers_explained() {
    println!("\n============================================================");
    println!("  SMART POINTERS - Complete Deep Dive");
    println!("============================================================\n");

    part1_why_smart_pointers();
    part2_box_in_depth();
    part3_building_linked_list();
    part4_rc_shared_ownership();
    part5_arc_for_threads();
    part6_deref_and_drop_traits();
    part7_cow_and_other_smart_pointers();
}

// ============================================================
// PART 1: WHY DO WE NEED SMART POINTERS?
// ============================================================

fn part1_why_smart_pointers() {
    println!("--- PART 1: Why Smart Pointers? ---\n");

    // PROBLEM 1: Rust needs to know sizes at compile time
    // ------------------------------------------------
    // The stack requires fixed-size data. But what about:
    // - Recursive types (a Node containing another Node)?
    // - Dynamic-sized collections?
    // - Data you don't know the size of until runtime?

    // This WON'T compile:
    // enum BadList {
    //     Cons(i32, BadList),  // ERROR: infinite size!
    //     Nil,
    // }
    // Each BadList contains another BadList... forever!
    // Compiler: "I need to know the size, and it's infinite!"

    // SOLUTION: Put recursive part on HEAP with a pointer
    // The pointer has fixed size (8 bytes on 64-bit systems)

    // PROBLEM 2: Single ownership is sometimes too restrictive
    // -------------------------------------------------------
    // What if multiple parts of your program need to own the same data?
    // Example: A graph where multiple nodes point to the same neighbor
    
    // SOLUTION: Reference counting (Rc<T>, Arc<T>)

    // PROBLEM 3: Sometimes you need to mutate through shared references
    // ----------------------------------------------------------------
    // The borrowing rules say: shared (&T) = read-only
    // But what if you really need shared + mutable?
    
    // SOLUTION: Interior mutability (RefCell<T>, Cell<T>)

    println!("Smart pointers solve:");
    println!("  1. Heap allocation → Box<T>");
    println!("  2. Recursive types → Box<T>");
    println!("  3. Multiple owners → Rc<T>, Arc<T>");
    println!("  4. Interior mutability → RefCell<T>, Cell<T>");
    println!("  5. Thread-safe sharing → Arc<T> + Mutex<T>");
}

// ============================================================
// PART 2: BOX<T> IN DEPTH
// ============================================================

fn part2_box_in_depth() {
    println!("\n--- PART 2: Box<T> In Depth ---\n");

    // Box<T> is the simplest smart pointer
    // It allocates data on the HEAP and owns it

    // ===== WHAT IS BOX? =====
    // Stack: Box { ptr: 0x7fff1234 }  (8 bytes - just a pointer)
    // Heap:  [actual data here]
    //
    // When Box goes out of scope, it:
    // 1. Runs Drop on the heap data
    // 2. Deallocates the heap memory

    // ===== USE CASE 1: Heap Allocation =====
    // Sometimes you WANT data on the heap
    
    let stack_value = 5;  // On stack
    let heap_value = Box::new(5);  // On heap
    
    println!("Stack value: {}", stack_value);
    println!("Heap value: {}", *heap_value);  // Deref to get the value
    
    // Box automatically derefs, so this also works:
    println!("Auto-deref: {}", heap_value);  // Rust calls *heap_value for us

    // ===== USE CASE 2: Large Data =====
    // Moving large data on stack = copying bytes
    // Moving Box = copying 8 bytes (just the pointer)
    
    #[derive(Debug)]
    struct LargeData {
        data: [u8; 1000],  // 1000 bytes!
    }
    
    let large = Box::new(LargeData { data: [0; 1000] });
    let moved = large;  // Only moves 8 bytes (the Box pointer)
    println!("Large data moved efficiently: {} bytes", moved.data.len());

    // ===== USE CASE 3: Trait Objects =====
    // When you don't know the concrete type at compile time
    
    trait Drawable {
        fn draw(&self);
    }
    
    struct Circle { radius: f64 }
    struct Square { side: f64 }
    
    impl Drawable for Circle {
        fn draw(&self) { println!("Drawing circle with radius {}", self.radius); }
    }
    impl Drawable for Square {
        fn draw(&self) { println!("Drawing square with side {}", self.side); }
    }
    
    // Box<dyn Trait> = "pointer to something that implements Trait"
    // Size is known: pointer + vtable pointer = 16 bytes
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Square { side: 4.0 }),
    ];
    
    for shape in &shapes {
        shape.draw();  // Dynamic dispatch via vtable
    }

    // ===== BOX MEMORY LAYOUT =====
    println!("\n--- Box Memory Layout ---");
    println!("Box<i32>:");
    println!("  Stack: [ptr: 0x...] (8 bytes)");
    println!("  Heap:  [4 bytes for i32]");
    println!();
    println!("Box<dyn Trait>:");
    println!("  Stack: [data_ptr: 0x..., vtable_ptr: 0x...] (16 bytes)");
    println!("  Heap:  [actual struct data]");
    println!("  Vtable: [function pointers for trait methods]");
}

// ============================================================
// PART 3: BUILDING A LINKED LIST WITH BOX
// ============================================================

fn part3_building_linked_list() {
    println!("\n--- PART 3: Building a Linked List with Box ---\n");

    // A linked list is a CLASSIC example of why we need Box
    // Each node contains a value AND a reference to the next node

    // ===== THE PROBLEM =====
    // enum BadNode {
    //     Node(i32, BadNode),  // What's the size? 4 + sizeof(BadNode) = infinite!
    //     Nil,
    // }

    // ===== THE SOLUTION: Box =====
    #[derive(Debug)]
    enum List<T> {
        Node(T, Box<List<T>>),  // Size: sizeof(T) + 8 bytes (Box pointer)
        Nil,
    }

    // Memory layout of Node(5, Box::new(Node(10, Box::new(Nil)))):
    //
    // Stack:
    //   [5, ptr_to_heap_1]
    //
    // Heap:
    //   heap_1: [10, ptr_to_heap_2]
    //   heap_2: [Nil variant tag]

    use List::{Node, Nil};

    // Build a list: 1 -> 2 -> 3 -> Nil
    let list = Node(1, 
        Box::new(Node(2, 
            Box::new(Node(3, 
                Box::new(Nil))))));

    println!("List: {:?}", list);

    // ===== A MORE PRACTICAL LINKED LIST =====
    #[derive(Debug)]
    struct LinkedList<T> {
        head: Option<Box<LinkedNode<T>>>,
    }

    #[derive(Debug)]
    struct LinkedNode<T> {
        value: T,
        next: Option<Box<LinkedNode<T>>>,
    }

    impl<T> LinkedList<T> {
        fn new() -> Self {
            LinkedList { head: None }
        }

        fn push_front(&mut self, value: T) {
            let new_node = Box::new(LinkedNode {
                value,
                next: self.head.take(),  // Take ownership of current head
            });
            self.head = Some(new_node);
        }

        fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.value
            })
        }
    }

    impl<T: std::fmt::Debug> LinkedList<T> {
        fn print(&self) {
            let mut current = &self.head;
            print!("List: ");
            while let Some(node) = current {
                print!("{:?} -> ", node.value);
                current = &node.next;
            }
            println!("None");
        }
    }

    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.print();  // List: 1 -> 2 -> 3 -> None

    let popped = list.pop_front();
    println!("Popped: {:?}", popped);
    list.print();

    // ===== WHY BOX AND NOT JUST REFERENCES? =====
    println!("\n--- Why Box, not &T? ---");
    println!("1. Box OWNS the data. References only borrow.");
    println!("2. With &T, the data must live somewhere else.");
    println!("3. Box lets the list manage its own memory.");
    println!("4. When list is dropped, all nodes are automatically freed.");
}

// ============================================================
// PART 4: Rc<T> - SHARED OWNERSHIP
// ============================================================

fn part4_rc_shared_ownership() {
    println!("\n--- PART 4: Rc<T> - Shared Ownership ---\n");

    // PROBLEM: What if TWO things need to own the same data?
    //
    // Example: Two lists sharing a common tail
    //   list1: [1] -> [2] ─┐
    //                      ├──> [3] -> [4] -> Nil  (shared!)
    //   list2: [5] ───────┘
    //
    // With Box, this is impossible! Box = single owner.

    // ===== WHAT IS Rc? =====
    // Rc = Reference Counted
    // Multiple Rc<T> can point to the same heap data
    // A counter tracks how many references exist
    // When counter hits 0, data is dropped

    // ===== HOW Rc WORKS =====
    //
    // Rc::new(value):
    //   Heap: [strong_count: 1, weak_count: 0, value: ...]
    //
    // Rc::clone(&rc):
    //   Just increments strong_count (doesn't copy data!)
    //
    // drop(rc):
    //   Decrements strong_count
    //   If strong_count == 0, drops value and frees memory

    let data = Rc::new(String::from("shared data"));
    println!("Created Rc. Count: {}", Rc::strong_count(&data));

    {
        let clone1 = Rc::clone(&data);
        println!("After clone1. Count: {}", Rc::strong_count(&data));
        
        let clone2 = Rc::clone(&data);
        println!("After clone2. Count: {}", Rc::strong_count(&data));
        
        // clone1 and clone2 go out of scope here
    }
    
    println!("After clones dropped. Count: {}", Rc::strong_count(&data));

    // ===== SHARED TAIL EXAMPLE =====
    #[derive(Debug)]
    enum SharedList {
        Node(i32, Rc<SharedList>),
        Nil,
    }
    use SharedList::{Node as SNode, Nil as SNil};

    // Shared tail: 3 -> 4 -> Nil
    let shared_tail = Rc::new(SNode(3, 
        Rc::new(SNode(4, 
            Rc::new(SNil)))));
    
    println!("Tail count: {}", Rc::strong_count(&shared_tail));

    // list1: 1 -> 2 -> [shared_tail]
    let list1 = SNode(1, 
        Rc::new(SNode(2, Rc::clone(&shared_tail))));
    
    // list2: 5 -> [shared_tail]
    let list2 = SNode(5, Rc::clone(&shared_tail));
    
    println!("After creating two lists sharing tail:");
    println!("  Tail count: {}", Rc::strong_count(&shared_tail));
    println!("  list1: {:?}", list1);
    println!("  list2: {:?}", list2);

    // ===== Rc FOR GRAPH STRUCTURES =====
    println!("\n--- Rc for Graphs ---");
    
    #[derive(Debug)]
    struct GraphNode {
        value: i32,
        neighbors: Vec<Rc<GraphNode>>,
    }

    // Create nodes
    let node_c = Rc::new(GraphNode { value: 3, neighbors: vec![] });
    let node_b = Rc::new(GraphNode { 
        value: 2, 
        neighbors: vec![Rc::clone(&node_c)] 
    });
    let node_a = Rc::new(GraphNode { 
        value: 1, 
        neighbors: vec![Rc::clone(&node_b), Rc::clone(&node_c)]  // A -> B, A -> C
    });

    println!("Node A: {:?}", node_a);
    println!("Node C reference count: {}", Rc::strong_count(&node_c));
    // C is referenced by: node_c variable, node_b, and node_a = 3 references

    // ===== IMPORTANT LIMITATIONS OF Rc =====
    println!("\n--- Rc Limitations ---");
    println!("1. Rc<T> is NOT thread-safe (use Arc<T> for threads)");
    println!("2. Rc<T> only allows SHARED (immutable) access");
    println!("   - To mutate, combine with RefCell: Rc<RefCell<T>>");
    println!("3. Rc can create MEMORY LEAKS via reference cycles");
    println!("   - Solution: Use Weak<T> for back-references");
}

// ============================================================
// PART 5: Arc<T> - THREAD-SAFE REFERENCE COUNTING
// ============================================================

fn part5_arc_for_threads() {
    println!("\n--- PART 5: Arc<T> - Thread-Safe Rc ---\n");

    // PROBLEM: Rc uses non-atomic counter increments
    // If two threads increment simultaneously: RACE CONDITION!
    //
    // Thread 1 reads count: 1
    // Thread 2 reads count: 1
    // Thread 1 writes count + 1: 2
    // Thread 2 writes count + 1: 2  ← WRONG! Should be 3!

    // SOLUTION: Arc = Atomic Reference Counted
    // Uses atomic CPU instructions for thread-safe counting
    // Slightly slower than Rc, but safe across threads

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("Main thread: {:?}", data);

    let mut handles = vec![];

    // Spawn 5 threads, each gets a clone of the Arc
    for i in 0..5 {
        let data_clone = Arc::clone(&data);  // Atomic increment
        let handle = thread::spawn(move || {
            // Each thread can read the data
            println!("Thread {}: sum = {}", i, data_clone.iter().sum::<i32>());
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads done. Final count: {}", Arc::strong_count(&data));

    // ===== Arc + Mutex FOR SHARED MUTABLE STATE =====
    use std::sync::Mutex;

    println!("\n--- Arc + Mutex: Shared Mutable State ---");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mutex::lock() gives us mutable access
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            // Lock is released when `num` goes out of scope
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    // ===== Rc vs Arc COMPARISON =====
    println!("\n--- Rc vs Arc ---");
    println!("┌────────────┬──────────────────┬──────────────────┐");
    println!("│            │ Rc<T>            │ Arc<T>           │");
    println!("├────────────┼──────────────────┼──────────────────┤");
    println!("│ Thread-safe│ NO               │ YES              │");
    println!("│ Performance│ Faster           │ Slightly slower  │");
    println!("│ Use case   │ Single-threaded  │ Multi-threaded   │");
    println!("│ Counter    │ Non-atomic       │ Atomic           │");
    println!("└────────────┴──────────────────┴──────────────────┘");
}

// ============================================================
// PART 6: DEREF AND DROP TRAITS
// ============================================================

fn part6_deref_and_drop_traits() {
    println!("\n--- PART 6: Deref and Drop Traits ---\n");

    // Smart pointers implement two key traits:
    // 1. Deref - makes them act like references
    // 2. Drop - custom cleanup when going out of scope

    // ===== DEREF TRAIT =====
    // Allows using * operator and enables "deref coercion"

    // Custom smart pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0  // Return reference to inner value
        }
    }

    let x = 5;
    let boxed = MyBox::new(x);
    
    // * operator works because of Deref
    assert_eq!(5, *boxed);  // Rust calls *(boxed.deref())
    println!("Deref works! *boxed = {}", *boxed);

    // ===== DEREF COERCION =====
    // Rust automatically calls deref() when types don't match

    fn print_str(s: &str) {
        println!("String: {}", s);
    }

    let my_string = MyBox::new(String::from("hello"));
    
    // MyBox<String> -> &String -> &str (via deref coercion)
    print_str(&my_string);  // Works! Rust does: &(*my_string) -> &String -> &str

    // ===== DROP TRAIT =====
    // Called automatically when value goes out of scope
    
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data: {}", self.data);
        }
    }

    println!("\nCreating CustomSmartPointers:");
    let _c = CustomSmartPointer { data: String::from("first") };
    let _d = CustomSmartPointer { data: String::from("second") };
    println!("CustomSmartPointers created.");
    
    // They'll be dropped at end of function, in REVERSE order (LIFO)

    // ===== EARLY DROP WITH std::mem::drop =====
    println!("\nEarly drop:");
    let e = CustomSmartPointer { data: String::from("early drop") };
    println!("Before drop()");
    drop(e);  // Explicitly drop now
    println!("After drop()");
    // e is now invalid, can't use it

    // Note: You can't call .drop() directly - that would cause double-free
    // Always use std::mem::drop() or let it go out of scope
}

// ============================================================
// PART 7: OTHER SMART POINTERS
// ============================================================

fn part7_cow_and_other_smart_pointers() {
    println!("\n--- PART 7: Other Smart Pointers ---\n");

    // ===== Cow<T> - Clone on Write =====
    // Holds either borrowed OR owned data
    // Only clones when you need to mutate borrowed data
    use std::borrow::Cow;

    fn process_data(input: &str) -> Cow<str> {
        if input.contains("bad") {
            // Need to modify - return owned String
            Cow::Owned(input.replace("bad", "good"))
        } else {
            // No modification needed - return borrowed &str
            Cow::Borrowed(input)
        }
    }

    let good_input = "this is fine";
    let bad_input = "this is bad";

    let result1 = process_data(good_input);
    let result2 = process_data(bad_input);

    println!("Result 1 (borrowed): {}", result1);
    println!("Result 2 (owned): {}", result2);

    match &result1 {
        Cow::Borrowed(_) => println!("  → Was borrowed (zero-copy!)"),
        Cow::Owned(_) => println!("  → Was cloned"),
    }
    match &result2 {
        Cow::Borrowed(_) => println!("  → Was borrowed"),
        Cow::Owned(_) => println!("  → Was cloned (had to modify)"),
    }

    // ===== Pin<T> - Prevent Moving =====
    // Used for self-referential structs and async
    println!("\n--- Pin<T> (brief) ---");
    println!("Pin<T> prevents a value from being moved in memory.");
    println!("Critical for:");
    println!("  - Self-referential structs");
    println!("  - Async/await (Futures often reference themselves)");
    println!("  - Interfacing with C code that expects stable addresses");

    // ===== SUMMARY TABLE =====
    println!("\n--- Smart Pointer Summary ---");
    println!("┌───────────────┬─────────────────────────────────────────┐");
    println!("│ Smart Pointer │ Use Case                                │");
    println!("├───────────────┼─────────────────────────────────────────┤");
    println!("│ Box<T>        │ Heap allocation, recursive types        │");
    println!("│ Rc<T>         │ Multiple owners (single-threaded)       │");
    println!("│ Arc<T>        │ Multiple owners (multi-threaded)        │");
    println!("│ RefCell<T>    │ Interior mutability (runtime checks)    │");
    println!("│ Cell<T>       │ Interior mutability (Copy types only)   │");
    println!("│ Mutex<T>      │ Thread-safe interior mutability         │");
    println!("│ RwLock<T>     │ Multiple readers OR one writer          │");
    println!("│ Cow<T>        │ Clone-on-write optimization             │");
    println!("│ Weak<T>       │ Non-owning reference (breaks cycles)    │");
    println!("│ Pin<T>        │ Prevent value from moving               │");
    println!("└───────────────┴─────────────────────────────────────────┘");
}

// ============================================================
// WHEN TO USE WHAT - DECISION TREE
// ============================================================
//
// Do you need heap allocation?
// └─ YES → Box<T>
//
// Do you need multiple owners?
// └─ YES → Is it multi-threaded?
//          ├─ NO  → Rc<T>
//          └─ YES → Arc<T>
//
// Do you need to mutate through shared reference?
// └─ YES → Is it multi-threaded?
//          ├─ NO  → RefCell<T> or Cell<T>
//          └─ YES → Mutex<T> or RwLock<T>
//
// Do you have a reference cycle?
// └─ YES → Use Weak<T> for the "back" reference
//
// Do you want to avoid cloning when possible?
// └─ YES → Cow<T>
//
// Do you need a stable memory address?
// └─ YES → Pin<T>
// ============================================================

// ============================================================
// THE REAL DECISION TABLE (MEMORIZE THIS!)
// ============================================================
//
// ┌─────────────────────────────────────────┬─────────────────┬─────────────────────────────────┐
// │ You need...                             │ Use this        │ Why                             │
// ├─────────────────────────────────────────┼─────────────────┼─────────────────────────────────┤
// │ One owner, full mutation, zero overhead │ Box<T>/String/  │ Fastest, simplest               │
// │                                         │ Vec<T>          │                                 │
// ├─────────────────────────────────────────┼─────────────────┼─────────────────────────────────┤
// │ Many owners, read-only, single thread   │ Rc<T>           │ Shared ownership, no mutation   │
// ├─────────────────────────────────────────┼─────────────────┼─────────────────────────────────┤
// │ Many owners, mutable, single thread     │ Rc<RefCell<T>>  │ Shared ownership + interior mut │
// ├─────────────────────────────────────────┼─────────────────┼─────────────────────────────────┤
// │ Many owners, mutable, multi-thread      │ Arc<Mutex<T>>   │ Thread-safe shared mutation     │
// ├─────────────────────────────────────────┼─────────────────┼─────────────────────────────────┤
// │ Many owners, read-only, multi-thread    │ Arc<T>          │ Lock-free shared read           │
// └─────────────────────────────────────────┴─────────────────┴─────────────────────────────────┘
//
// ============================================================
// WHAT EACH ONE REALLY PROMISES
// ============================================================
//
// Box<T> / String / Vec<T>
// ─────────────────────────
//   "I am the ONLY owner. I can mutate freely."
//
// Rc<T>
// ─────
//   "We all own it, but NOBODY can mutate."
//
// Rc<RefCell<T>>
// ──────────────
//   "We all own it, but only ONE of us may mutate at a time.
//    If someone cheats → PANIC."
//
// Arc<Mutex<T>>
// ─────────────
//   "We all own it, ACROSS THREADS.
//    Only one thread mutates at a time.
//    The OS & CPU guarantee safety."
//
// ============================================================
// THE RUST GUARANTEE LADDER
// ============================================================
//
// ┌─────────────────┬─────────────────────────────┐
// │ Type            │ Enforced when?              │
// ├─────────────────┼─────────────────────────────┤
// │ Box<T>          │ Compile time                │
// │ Rc<T>           │ Compile time                │
// │ Rc<RefCell<T>>  │ Runtime                     │
// │ Arc<Mutex<T>>   │ Runtime + Hardware          │
// └─────────────────┴─────────────────────────────┘
//
// ============================================================
// FINAL MENTAL MODEL (THE COMPLETE PICTURE)
// ============================================================
//
// ✔ Box<T>         → Single owner, full control
// ✔ Rc<T>          → Many owners, read-only
// ✔ Rc<RefCell<T>> → Many owners, mutable, one at a time
// ✔ Arc<T>         → Rc, but thread-safe (read-only)
// ✔ Arc<Mutex<T>>  → Rc<RefCell<T>>, but thread-safe
//
// ────────────────────────────────────────────────────────────
// This is the point where Rust stops being confusing
// and starts being POWERFUL.
//
// Once you internalize these patterns, you'll find that
// Rust's "restrictions" are actually its greatest strength:
// - No data races, ever
// - No use-after-free, ever
// - No null pointer dereferences, ever
// - The compiler catches your mistakes before users do
//
// Welcome to fearless concurrency and memory safety! 🦀
// ────────────────────────────────────────────────────────────

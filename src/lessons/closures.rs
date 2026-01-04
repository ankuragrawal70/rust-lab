// ============================================================
// CLOSURES - Anonymous Functions That Capture Their Environment
// ============================================================
// Closures are one of Rust's most powerful features.
// They're anonymous functions that can capture variables from their scope.
//
// Three closure traits:
// - Fn:     Borrows captured variables immutably (can call multiple times)
// - FnMut:  Borrows captured variables mutably (can call multiple times)
// - FnOnce: Takes ownership of captured variables (can only call once)

pub fn learn_closures() {
    println!("\n============================================================");
    println!("  CLOSURES - Functions That Capture Their Environment");
    println!("============================================================\n");

    part1_closure_basics();
    part2_capturing_variables();
    part3_fn_fnmut_fnonce();
    part4_closures_as_parameters();
    part5_returning_closures();
    part6_closures_with_iterators();
    part7_practical_patterns();
}

// ============================================================
// PART 1: CLOSURE BASICS
// ============================================================

fn part1_closure_basics() {
    println!("--- PART 1: Closure Basics ---\n");

    // ===== WHAT IS A CLOSURE? =====
    // A closure is an anonymous function you can save in a variable
    // or pass to other functions.

    // Regular function
    fn add_one_fn(x: i32) -> i32 {
        x + 1
    }

    // Equivalent closure
    let add_one_closure = |x: i32| -> i32 { x + 1 };

    // Even shorter - Rust can infer types!
    let add_one = |x| x + 1;

    println!("Function: {}", add_one_fn(5));
    println!("Closure (explicit): {}", add_one_closure(5));
    println!("Closure (inferred): {}", add_one(5));

    // ===== CLOSURE SYNTAX =====
    // |params| body
    // |params| -> ReturnType { body }

    // No parameters
    let say_hi = || println!("Hi!");
    say_hi();

    // One parameter (type inferred)
    let double = |x| x * 2;
    println!("Double 5: {}", double(5));

    // Multiple parameters
    let add = |a, b| a + b;
    println!("3 + 4 = {}", add(3, 4));

    // With explicit types and body block
    let multiply = |a: i32, b: i32| -> i32 {
        let result = a * b;
        result  // Last expression is returned
    };
    println!("3 * 4 = {}", multiply(3, 4));

    // ===== KEY DIFFERENCE FROM FUNCTIONS =====
    // Closures can capture variables from their enclosing scope!
    
    let factor = 10;
    let multiply_by_factor = |x| x * factor;  // Captures 'factor'
    println!("5 * factor(10) = {}", multiply_by_factor(5));

    // A regular function CAN'T do this:
    // fn multiply_fn(x: i32) -> i32 {
    //     x * factor  // ERROR: can't access 'factor'
    // }
}

// ============================================================
// PART 2: CAPTURING VARIABLES
// ============================================================

fn part2_capturing_variables() {
    println!("\n--- PART 2: Capturing Variables ---\n");

    // Closures can capture variables in three ways:
    // 1. By reference (&T) - borrows immutably
    // 2. By mutable reference (&mut T) - borrows mutably  
    // 3. By value (T) - takes ownership

    // ===== CAPTURE BY IMMUTABLE REFERENCE =====
    let message = String::from("Hello");
    
    let print_message = || {
        println!("Message: {}", message);  // Borrows 'message'
    };
    
    print_message();
    print_message();  // Can call multiple times
    println!("Still valid: {}", message);  // 'message' still accessible

    // ===== CAPTURE BY MUTABLE REFERENCE =====
    let mut count = 0;
    
    let mut increment = || {
        count += 1;  // Mutably borrows 'count'
        println!("Count: {}", count);
    };
    
    increment();
    increment();
    increment();
    // Note: Can't use 'count' here while closure exists
    // println!("{}", count);  // ERROR if increment is used again
    
    // After closure is done, we can use count again
    println!("Final count: {}", count);

    // ===== CAPTURE BY VALUE (MOVE) =====
    let data = vec![1, 2, 3];
    
    // 'move' keyword forces ownership transfer
    let consume_data = move || {
        println!("Data: {:?}", data);  // Now OWNS 'data'
    };
    
    consume_data();
    // println!("{:?}", data);  // ERROR: data was moved into closure

    // ===== WHY USE MOVE? =====
    // 1. When closure outlives the current scope (e.g., threads)
    // 2. When you explicitly want to transfer ownership
    
    use std::thread;
    
    let numbers = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        // Thread might outlive this function!
        // 'move' ensures it owns the data
        println!("Thread got: {:?}", numbers);
    });
    
    handle.join().unwrap();

    // ===== RUST INFERS THE CAPTURE MODE =====
    // It picks the least restrictive mode needed:
    // - If closure only reads: &T (immutable borrow)
    // - If closure modifies: &mut T (mutable borrow)
    // - If closure consumes/moves: T (ownership)
    
    let s = String::from("hello");
    
    // Only reads s → captures by &String
    let len = || s.len();
    println!("Length: {}", len());
    println!("Still have s: {}", s);
    
    // Consumes s → captures by String (ownership)
    let consume = || {
        drop(s);  // Drops s, needs ownership
    };
    consume();
    // println!("{}", s);  // ERROR: s was moved
}

// ============================================================
// PART 3: Fn, FnMut, FnOnce TRAITS
// ============================================================

fn part3_fn_fnmut_fnonce() {
    println!("\n--- PART 3: Fn, FnMut, FnOnce Traits ---\n");

    // Every closure implements one or more of these traits:
    //
    // FnOnce: All closures implement this. Can be called at least once.
    //         Takes ownership of captured values.
    //
    // FnMut:  Can be called multiple times, may mutate captured values.
    //         Requires &mut self.
    //
    // Fn:     Can be called multiple times, won't mutate captured values.
    //         Requires &self.
    //
    // Hierarchy: Fn ⊂ FnMut ⊂ FnOnce
    // If a closure is Fn, it's also FnMut and FnOnce

    // ===== Fn - Immutable borrow, callable many times =====
    let x = 10;
    let fn_closure = || x + 1;  // Only reads x
    
    // Fn can be called any number of times
    println!("Fn closure: {}, {}, {}", fn_closure(), fn_closure(), fn_closure());

    // ===== FnMut - Mutable borrow, callable many times =====
    let mut sum = 0;
    let mut fnmut_closure = || {
        sum += 1;  // Mutates sum
        sum
    };
    
    println!("FnMut closure: {}", fnmut_closure());
    println!("FnMut closure: {}", fnmut_closure());
    println!("FnMut closure: {}", fnmut_closure());

    // ===== FnOnce - Takes ownership, callable once =====
    let data = String::from("consumed");
    let fnonce_closure = || {
        drop(data);  // Consumes data
        println!("Data dropped!");
    };
    
    fnonce_closure();  // Can only call once!
    // fnonce_closure();  // ERROR: closure already consumed

    // ===== TRAIT BOUNDS IN PRACTICE =====
    
    // Function that accepts any closure (FnOnce)
    fn call_once<F: FnOnce()>(f: F) {
        f();
    }
    
    // Function that may call multiple times (FnMut)
    fn call_twice<F: FnMut()>(mut f: F) {
        f();
        f();
    }
    
    // Function that won't mutate (Fn)
    fn call_three_times<F: Fn() -> i32>(f: F) {
        println!("{}, {}, {}", f(), f(), f());
    }
    
    let x = 5;
    call_once(|| println!("Called once with x = {}", x));
    
    let mut count = 0;
    call_twice(|| {
        count += 1;
        println!("Count: {}", count);
    });
    
    call_three_times(|| x * 2);

    // ===== SUMMARY TABLE =====
    println!("\n--- Closure Trait Summary ---");
    println!("┌──────────┬────────────────┬───────────────┬────────────────────┐");
    println!("│ Trait    │ self type      │ Call count    │ Captures           │");
    println!("├──────────┼────────────────┼───────────────┼────────────────────┤");
    println!("│ FnOnce   │ self           │ Once          │ By value (move)    │");
    println!("│ FnMut    │ &mut self      │ Many          │ By mutable ref     │");
    println!("│ Fn       │ &self          │ Many          │ By immutable ref   │");
    println!("└──────────┴────────────────┴───────────────┴────────────────────┘");
}

// ============================================================
// PART 4: CLOSURES AS FUNCTION PARAMETERS
// ============================================================

fn part4_closures_as_parameters() {
    println!("\n--- PART 4: Closures as Parameters ---\n");

    // ===== GENERIC APPROACH (Preferred) =====
    // Use generics with trait bounds

    fn apply<F>(value: i32, f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(value)
    }

    let double = |x| x * 2;
    let square = |x| x * x;

    println!("apply(5, double) = {}", apply(5, double));
    println!("apply(5, square) = {}", apply(5, square));

    // ===== MULTIPLE CLOSURES =====
    fn combine<F, G>(value: i32, f: F, g: G) -> i32
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
    {
        g(f(value))
    }

    println!("combine(3, double, square) = {}", combine(3, double, square));  // (3*2)^2 = 36

    // ===== TRAIT OBJECTS (Dynamic dispatch) =====
    // When you need to store different closures in a collection
    
    fn apply_dyn(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
        f(value)
    }

    println!("apply_dyn(5, &double) = {}", apply_dyn(5, &double));

    // Store different closures in a Vec
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    let mut result = 2;
    for op in &operations {
        result = op(result);
        println!("After operation: {}", result);
    }

    // ===== FUNCTION POINTERS VS CLOSURES =====
    // If you don't need to capture, you can use fn type
    
    fn apply_fn_ptr(value: i32, f: fn(i32) -> i32) -> i32 {
        f(value)
    }

    fn triple(x: i32) -> i32 { x * 3 }

    // Both regular functions and non-capturing closures work
    println!("apply_fn_ptr with fn: {}", apply_fn_ptr(5, triple));
    println!("apply_fn_ptr with closure: {}", apply_fn_ptr(5, |x| x * 3));

    // But capturing closures DON'T work with fn pointers:
    // let factor = 4;
    // apply_fn_ptr(5, |x| x * factor);  // ERROR! Captures 'factor'
}

// ============================================================
// PART 5: RETURNING CLOSURES
// ============================================================

fn part5_returning_closures() {
    println!("\n--- PART 5: Returning Closures ---\n");

    // Returning closures is tricky because closures have anonymous types
    // We need to use impl Trait or Box<dyn Trait>

    // ===== USING impl Fn (Static dispatch) =====
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n  // 'move' because n must be owned by closure
    }

    let add_5 = make_adder(5);
    let add_10 = make_adder(10);

    println!("add_5(3) = {}", add_5(3));
    println!("add_10(3) = {}", add_10(3));

    // ===== USING Box<dyn Fn> (Dynamic dispatch) =====
    // Needed when you might return different closures
    
    fn make_operation(op: &str) -> Box<dyn Fn(i32) -> i32> {
        match op {
            "double" => Box::new(|x| x * 2),
            "square" => Box::new(|x| x * x),
            "negate" => Box::new(|x| -x),
            _ => Box::new(|x| x),  // identity
        }
    }

    let double = make_operation("double");
    let square = make_operation("square");
    let negate = make_operation("negate");

    println!("double(5) = {}", double(5));
    println!("square(5) = {}", square(5));
    println!("negate(5) = {}", negate(5));

    // ===== CLOSURE FACTORIES =====
    fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    let times_2 = multiplier(2);
    let times_10 = multiplier(10);
    let times_100 = multiplier(100);

    println!("times_2(5) = {}", times_2(5));
    println!("times_10(5) = {}", times_10(5));
    println!("times_100(5) = {}", times_100(5));

    // ===== COMPOSING CLOSURES =====
    fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }

    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    
    let add_one_then_double = compose(add_one, double);
    println!("(5 + 1) * 2 = {}", add_one_then_double(5));  // 12
}

// ============================================================
// PART 6: CLOSURES WITH ITERATORS
// ============================================================

fn part6_closures_with_iterators() {
    println!("\n--- PART 6: Closures with Iterators ---\n");

    // Closures are ESSENTIAL for iterator methods!
    // map, filter, fold, for_each, find, etc.

    let numbers = vec![1, 2, 3, 4, 5];

    // ===== map - Transform each element =====
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // ===== filter - Keep elements matching predicate =====
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // ===== Chaining =====
    let result: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 1)  // Keep odd
        .map(|x| x * 10)           // Multiply by 10
        .collect();
    println!("Odd * 10: {:?}", result);

    // ===== fold - Reduce to single value =====
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    // ===== for_each - Side effects =====
    print!("Numbers: ");
    numbers.iter().for_each(|x| print!("{} ", x));
    println!();

    // ===== find - First matching element =====
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    // ===== any / all - Boolean checks =====
    let has_even = numbers.iter().any(|x| x % 2 == 0);
    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);

    // ===== CAPTURING STATE IN ITERATOR CLOSURES =====
    let threshold = 3;
    let above_threshold: Vec<_> = numbers
        .iter()
        .filter(|x| **x > threshold)  // Captures 'threshold'
        .collect();
    println!("Above {}: {:?}", threshold, above_threshold);

    // ===== MUTABLE STATE WITH for_each =====
    let mut running_total = 0;
    numbers.iter().for_each(|x| {
        running_total += x;
        println!("After {}: total = {}", x, running_total);
    });
}

// ============================================================
// PART 7: PRACTICAL PATTERNS
// ============================================================

fn part7_practical_patterns() {
    println!("\n--- PART 7: Practical Patterns ---\n");

    // ===== PATTERN 1: Callbacks =====
    struct Button {
        on_click: Box<dyn Fn()>,
    }

    impl Button {
        fn new<F: Fn() + 'static>(callback: F) -> Self {
            Button {
                on_click: Box::new(callback),
            }
        }

        fn click(&self) {
            (self.on_click)();
        }
    }

    let button = Button::new(|| println!("Button clicked!"));
    button.click();

    // ===== PATTERN 2: Strategy Pattern =====
    fn process_data<F>(data: Vec<i32>, strategy: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        data.into_iter().map(strategy).collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled = process_data(numbers.clone(), |x| x * 2);
    let squared = process_data(numbers.clone(), |x| x * x);
    
    println!("Doubled: {:?}", doubled);
    println!("Squared: {:?}", squared);

    // ===== PATTERN 3: Lazy Evaluation =====
    struct Lazy<T> {
        value: Option<T>,
        init: Option<Box<dyn FnOnce() -> T>>,
    }

    impl<T> Lazy<T> {
        fn new<F: FnOnce() -> T + 'static>(init: F) -> Self {
            Lazy {
                value: None,
                init: Some(Box::new(init)),
            }
        }

        fn get(&mut self) -> &T {
            if self.value.is_none() {
                let init = self.init.take().unwrap();
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }

    let mut lazy_value = Lazy::new(|| {
        println!("Computing expensive value...");
        42 * 2
    });

    println!("Before first access");
    println!("Value: {}", lazy_value.get());  // Computes now
    println!("Value: {}", lazy_value.get());  // Uses cached

    // ===== PATTERN 4: Builder with Closures =====
    struct QueryBuilder {
        filters: Vec<Box<dyn Fn(&i32) -> bool>>,
    }

    impl QueryBuilder {
        fn new() -> Self {
            QueryBuilder { filters: vec![] }
        }

        fn filter<F: Fn(&i32) -> bool + 'static>(mut self, f: F) -> Self {
            self.filters.push(Box::new(f));
            self
        }

        fn execute(&self, data: &[i32]) -> Vec<i32> {
            data.iter()
                .filter(|x| self.filters.iter().all(|f| f(x)))
                .copied()
                .collect()
        }
    }

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let results = QueryBuilder::new()
        .filter(|x| *x > 3)      // Greater than 3
        .filter(|x| *x < 8)      // Less than 8
        .filter(|x| x % 2 == 0)  // Even
        .execute(&data);

    println!("Filtered results: {:?}", results);  // [4, 6]

    // ===== PATTERN 5: Event Handlers =====
    type EventHandler = Box<dyn Fn(i32)>;

    struct EventEmitter {
        handlers: Vec<EventHandler>,
    }

    impl EventEmitter {
        fn new() -> Self {
            EventEmitter { handlers: vec![] }
        }

        fn on<F: Fn(i32) + 'static>(&mut self, handler: F) {
            self.handlers.push(Box::new(handler));
        }

        fn emit(&self, value: i32) {
            for handler in &self.handlers {
                handler(value);
            }
        }
    }

    let mut emitter = EventEmitter::new();
    emitter.on(|x| println!("Handler 1 got: {}", x));
    emitter.on(|x| println!("Handler 2 got: {} doubled = {}", x, x * 2));

    emitter.emit(42);
}

// ============================================================
// CLOSURE CHEAT SHEET
// ============================================================
//
// SYNTAX:
//   |x|      x + 1              // Single param, expression body
//   |x, y|   x + y              // Multiple params
//   |x: i32| -> i32 { x + 1 }   // Explicit types
//   ||       println!("hi")     // No params
//   move |x| x + captured       // Force move capture
//
// TRAITS:
//   Fn     - borrows immutably, callable many times
//   FnMut  - borrows mutably, callable many times
//   FnOnce - takes ownership, callable once
//
// AS PARAMETERS:
//   fn foo<F: Fn(i32) -> i32>(f: F)     // Generic (preferred)
//   fn foo(f: &dyn Fn(i32) -> i32)      // Trait object
//   fn foo(f: fn(i32) -> i32)           // Function pointer
//
// RETURNING:
//   fn bar() -> impl Fn(i32) -> i32     // Static dispatch
//   fn bar() -> Box<dyn Fn(i32) -> i32> // Dynamic dispatch
//
// WITH ITERATORS:
//   .map(|x| x * 2)
//   .filter(|x| *x > 0)
//   .fold(0, |acc, x| acc + x)
//   .for_each(|x| println!("{}", x))
//   .find(|x| *x == target)
//   .any(|x| x > 0)
//   .all(|x| x > 0)
//
// ============================================================

// ============================================================
// CONCURRENCY IN RUST - Threads, Channels, Send & Sync
// ============================================================
// Rust's type system prevents data races at compile time!
// "Fearless concurrency" - the compiler catches your mistakes.
//
// Key concepts:
// - Thread spawning and joining
// - Message passing with channels (mpsc)
// - Shared state with Arc<Mutex<T>>
// - Send and Sync traits

use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::thread;
use std::time::Duration;

pub fn learn_concurrency() {
    println!("\n============================================================");
    println!("  CONCURRENCY - Threads, Channels, Send & Sync");
    println!("============================================================\n");

    part1_thread_basics();
    part2_sharing_data_move();
    part3_message_passing_channels();
    part4_shared_state_mutex();
    part5_rwlock_for_readers();
    part6_send_and_sync();
    part7_practical_patterns();
}

// ============================================================
// PART 1: THREAD BASICS
// ============================================================

fn part1_thread_basics() {
    println!("--- PART 1: Thread Basics ---\n");

    // ===== SPAWNING THREADS =====
    // thread::spawn takes a closure and runs it in a new thread
    
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  Spawned thread: count {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("Main thread: count {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    // ===== JOINING THREADS =====
    // Wait for thread to finish
    handle.join().unwrap();  // unwrap because join returns Result
    println!("Thread finished!");

    // ===== THREAD HANDLES =====
    let handle = thread::spawn(|| {
        // Return a value from thread
        let sum: i32 = (1..=100).sum();
        sum
    });

    // Get the return value
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);

    // ===== MULTIPLE THREADS =====
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("  Thread {} started", i);
            thread::sleep(Duration::from_millis(100));
            println!("  Thread {} finished", i);
            i * 10  // Return value
        });
        handles.push(handle);
    }

    // Collect all results
    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("All threads finished: {:?}", results);

    // ===== THREAD::CURRENT AND THREAD::PARK =====
    let current = thread::current();
    println!("Current thread name: {:?}", current.name());
}

// ============================================================
// PART 2: SHARING DATA - THE MOVE KEYWORD
// ============================================================

fn part2_sharing_data_move() {
    println!("\n--- PART 2: Sharing Data - The Move Keyword ---\n");

    // ===== THE PROBLEM =====
    // This won't compile:
    // let data = vec![1, 2, 3];
    // thread::spawn(|| {
    //     println!("{:?}", data);  // ERROR: might outlive borrowed data
    // });

    // The thread might outlive the current function,
    // but `data` will be dropped when function ends!

    // ===== SOLUTION 1: MOVE OWNERSHIP =====
    let data = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        // 'move' transfers ownership of 'data' into closure
        println!("Thread owns data: {:?}", data);
    });
    
    // data is now moved, can't use it here
    // println!("{:?}", data);  // ERROR: value moved
    
    handle.join().unwrap();

    // ===== SOLUTION 2: CLONE BEFORE MOVE =====
    let original = vec![1, 2, 3];
    let cloned = original.clone();

    let handle = thread::spawn(move || {
        println!("Thread has clone: {:?}", cloned);
    });

    println!("Main still has original: {:?}", original);
    handle.join().unwrap();

    // ===== SOLUTION 3: Arc FOR SHARED OWNERSHIP =====
    // Arc = Atomically Reference Counted (thread-safe Rc)
    
    let shared = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&shared);  // Clone the Arc, not the Vec
        let handle = thread::spawn(move || {
            println!("Thread {}: sum = {}", i, data.iter().sum::<i32>());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main thread: data = {:?}", shared);
}

// ============================================================
// PART 3: MESSAGE PASSING WITH CHANNELS
// ============================================================

fn part3_message_passing_channels() {
    println!("\n--- PART 3: Message Passing - Channels ---\n");

    // "Do not communicate by sharing memory;
    //  share memory by communicating." - Go proverb, also Rust!

    // ===== BASIC CHANNEL (mpsc) =====
    // mpsc = Multiple Producer, Single Consumer
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });

    // recv() blocks until message arrives
    let message = rx.recv().unwrap();
    println!("Received: {}", message);

    // ===== SENDING MULTIPLE MESSAGES =====
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["hi", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Iterate over received messages
    for received in rx {
        println!("Got: {}", received);
    }
    println!("Channel closed");

    // ===== MULTIPLE PRODUCERS =====
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();  // Clone the sender
        thread::spawn(move || {
            tx_clone.send(format!("Message from thread {}", i)).unwrap();
        });
    }

    drop(tx);  // Drop original sender so channel can close

    for msg in rx {
        println!("Received: {}", msg);
    }

    // ===== NON-BLOCKING TRY_RECV =====
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("delayed message").unwrap();
    });

    // Try without blocking
    match rx.try_recv() {
        Ok(msg) => println!("Got immediately: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("No message yet"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Channel closed"),
    }

    // Wait for the message
    let msg = rx.recv().unwrap();
    println!("Got after waiting: {}", msg);

    // ===== SYNC CHANNEL (BOUNDED) =====
    let (tx, rx) = mpsc::sync_channel(2);  // Buffer size 2

    tx.send(1).unwrap();
    tx.send(2).unwrap();
    // tx.send(3).unwrap();  // Would block! Buffer full

    println!("Sync channel: {}, {}", rx.recv().unwrap(), rx.recv().unwrap());
}

// ============================================================
// PART 4: SHARED STATE WITH MUTEX
// ============================================================

fn part4_shared_state_mutex() {
    println!("\n--- PART 4: Shared State - Mutex ---\n");

    // Mutex = Mutual Exclusion
    // Only one thread can access the data at a time

    // ===== BASIC MUTEX USAGE =====
    let m = Mutex::new(5);

    {
        // lock() returns MutexGuard (smart pointer)
        let mut num = m.lock().unwrap();
        *num = 6;
        // MutexGuard dropped here, lock released
    }

    println!("Mutex value: {:?}", m);

    // ===== MUTEX + ARC FOR MULTI-THREADED =====
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Lock released when num goes out of scope
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    // ===== MUTEX POISONING =====
    // If a thread panics while holding a lock, the mutex is "poisoned"
    
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        let mut guard = data_clone.lock().unwrap();
        guard.push(4);
        // If we panicked here, mutex would be poisoned
    });

    handle.join().unwrap();

    // Recovering from poisoned mutex:
    let guard = data.lock().unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering...");
        poisoned.into_inner()
    });
    println!("Data: {:?}", *guard);

    // ===== DEADLOCK WARNING =====
    println!("\n--- Deadlock Warning ---");
    println!("Deadlock can occur when:");
    println!("  1. Thread A locks Mutex 1, waits for Mutex 2");
    println!("  2. Thread B locks Mutex 2, waits for Mutex 1");
    println!("Prevention:");
    println!("  - Always lock mutexes in the same order");
    println!("  - Use lock_timeout if available");
    println!("  - Keep lock scopes as small as possible");
}

// ============================================================
// PART 5: RwLock - MULTIPLE READERS OR ONE WRITER
// ============================================================

fn part5_rwlock_for_readers() {
    println!("\n--- PART 5: RwLock - Readers & Writers ---\n");

    // RwLock allows:
    // - Multiple readers (shared access)
    // - OR one writer (exclusive access)
    // Better performance when reads >> writes

    let lock = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Spawn multiple readers
    for i in 0..3 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let data = lock.read().unwrap();  // Read lock
            println!("Reader {}: {:?}", i, *data);
            // Multiple readers can hold this simultaneously
        });
        handles.push(handle);
    }

    // Spawn a writer
    {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let mut data = lock.write().unwrap();  // Write lock
            data.push(4);
            println!("Writer: added 4");
            // Only one writer, blocks all readers
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final data: {:?}", *lock.read().unwrap());

    // ===== MUTEX VS RWLOCK =====
    println!("\n--- Mutex vs RwLock ---");
    println!("┌────────────┬──────────────────────────────┬──────────────────────────────┐");
    println!("│            │ Mutex<T>                     │ RwLock<T>                    │");
    println!("├────────────┼──────────────────────────────┼──────────────────────────────┤");
    println!("│ Access     │ Exclusive only               │ Shared read OR Exclusive     │");
    println!("│ Use when   │ Mostly writes                │ Mostly reads, few writes     │");
    println!("│ Overhead   │ Lower                        │ Slightly higher              │");
    println!("│ Deadlock   │ Double lock = deadlock       │ Write waiting can deadlock   │");
    println!("└────────────┴──────────────────────────────┴──────────────────────────────┘");
}

// ============================================================
// PART 6: SEND AND SYNC TRAITS
// ============================================================

fn part6_send_and_sync() {
    println!("\n--- PART 6: Send and Sync Traits ---\n");

    // These are MARKER TRAITS - they have no methods
    // The compiler uses them to verify thread safety

    // ===== SEND =====
    // A type is Send if ownership can be transferred between threads
    // Most types are Send. Notable exceptions:
    // - Rc<T> - not thread-safe reference counting
    // - Raw pointers
    // - Types containing non-Send types

    // ===== SYNC =====
    // A type is Sync if it's safe to be shared between threads
    // T is Sync if &T is Send
    // Notable exceptions:
    // - RefCell<T> - runtime borrow checking is not thread-safe
    // - Cell<T> - same reason
    // - Rc<T>

    // ===== EXAMPLES =====
    println!("Send types: i32, String, Vec<T>, Arc<T>, Mutex<T>");
    println!("NOT Send: Rc<T>, raw pointers");
    println!();
    println!("Sync types: i32, String, Arc<T>, Mutex<T>");
    println!("NOT Sync: Rc<T>, RefCell<T>, Cell<T>");

    // ===== WHY THIS MATTERS =====
    // The compiler won't let you share non-Sync types between threads:
    //
    // use std::rc::Rc;
    // let rc = Rc::new(5);
    // thread::spawn(move || {
    //     println!("{}", rc);
    // });
    // ERROR: `Rc<i32>` cannot be sent between threads safely

    // ===== THE MAPPING =====
    println!("\n--- Single-threaded to Multi-threaded ---");
    println!("┌────────────────────┬────────────────────┐");
    println!("│ Single-threaded    │ Multi-threaded     │");
    println!("├────────────────────┼────────────────────┤");
    println!("│ Rc<T>              │ Arc<T>             │");
    println!("│ RefCell<T>         │ Mutex<T>           │");
    println!("│ Cell<T>            │ Atomic types       │");
    println!("│ Rc<RefCell<T>>     │ Arc<Mutex<T>>      │");
    println!("└────────────────────┴────────────────────┘");

    // ===== IMPLEMENTING SEND/SYNC =====
    // These are auto-traits - compiler implements them automatically
    // You rarely need to implement them manually
    // When you do, it requires `unsafe`:
    //
    // unsafe impl Send for MyType {}
    // unsafe impl Sync for MyType {}
    //
    // Only do this if you KNOW it's safe!
}

// ============================================================
// PART 7: PRACTICAL PATTERNS
// ============================================================

fn part7_practical_patterns() {
    println!("\n--- PART 7: Practical Patterns ---\n");

    // ===== PATTERN 1: WORKER POOL =====
    println!("--- Pattern 1: Worker Pool ---");

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));  // Share receiver among workers
    let mut handles = vec![];

    // Create worker threads
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                // Try to get a job
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(task) => println!("Worker {} processing: {}", id, task),
                    Err(_) => {
                        println!("Worker {} shutting down", id);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Send jobs
    for i in 0..9 {
        tx.send(format!("Task {}", i)).unwrap();
    }
    
    drop(tx);  // Close channel to signal shutdown

    for handle in handles {
        handle.join().unwrap();
    }

    // ===== PATTERN 2: SCOPED THREADS =====
    println!("\n--- Pattern 2: Scoped Threads ---");
    
    // std::thread::scope lets threads borrow from parent
    let mut data = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Scoped thread reads: {:?}", data);
        });
        
        s.spawn(|| {
            println!("Another thread reads: {:?}", data);
        });
    });  // Threads must finish before scope ends

    // Can safely mutate after scope
    data.push(4);
    println!("After scope: {:?}", data);

    // ===== PATTERN 3: PARALLEL MAP =====
    println!("\n--- Pattern 3: Parallel Map ---");

    fn parallel_map<T, R, F>(items: Vec<T>, f: F) -> Vec<R>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Clone + 'static,
    {
        let handles: Vec<_> = items
            .into_iter()
            .map(|item| {
                let f = f.clone();
                thread::spawn(move || f(item))
            })
            .collect();

        handles.into_iter()
            .map(|h| h.join().unwrap())
            .collect()
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let squared = parallel_map(numbers, |x| x * x);
    println!("Parallel squared: {:?}", squared);

    // ===== PATTERN 4: CONCURRENT COUNTER =====
    println!("\n--- Pattern 4: Atomic Counter ---");

    use std::sync::atomic::{AtomicUsize, Ordering};

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Atomic counter: {}", counter.load(Ordering::SeqCst));
}

// ============================================================
// CONCURRENCY CHEAT SHEET
// ============================================================
//
// THREADS:
//   thread::spawn(|| {...})       // Start new thread
//   handle.join().unwrap()        // Wait for completion
//   thread::sleep(Duration)       // Sleep
//   thread::scope(|s| {...})      // Scoped threads (borrow ok)
//
// CHANNELS (mpsc):
//   let (tx, rx) = mpsc::channel()  // Create channel
//   tx.send(value)                  // Send (blocks if sync)
//   rx.recv()                       // Receive (blocks)
//   rx.try_recv()                   // Non-blocking receive
//   tx.clone()                      // Multiple producers
//
// SHARED STATE:
//   Arc<T>         // Thread-safe Rc (read-only sharing)
//   Mutex<T>       // Exclusive lock
//   RwLock<T>      // Multiple readers OR one writer
//   Atomic types   // Lock-free for primitives
//
// COMMON PATTERNS:
//   Arc<Mutex<T>>  // Shared mutable state
//   Arc<RwLock<T>> // Many readers, few writers
//
// MARKER TRAITS:
//   Send // Can transfer ownership to another thread
//   Sync // Can share references between threads
//
// CONVERSIONS:
//   Rc<T>        → Arc<T>
//   RefCell<T>   → Mutex<T>
//   Cell<T>      → AtomicXxx
//
// ============================================================

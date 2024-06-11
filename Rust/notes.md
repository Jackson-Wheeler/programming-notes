# General
## Style Conventions
- four spaces for each indent

## Project Setup
Create new project:
```sh
cargo new <project-name>
```

Build project:
```sh
cargo build
```

Compile & run project:
```sh
cargo run
```

Syntax Check:
```sh
cargo check
```

Format Command (Can set up format on save in code editor):
```sh
rustfmt <file-name>
```

Build for Release:
```sh
cargo build --release
```

Check Documentation for Project & Dependencies:
```sh
cargo doc --open
```

## Managing Projects
### Overview
* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and
  privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module

### Packages and Crates
*Crate*: the smallest amount of code that the Rust compiler considers at a time. Can contain modules, and the modules may be defined in other files that get compiled with the crate.

*Binary crates*: programs you can compile to an executable tha tyou can run (must have `main`)

*Library crates*: define functionality

*Package*: A bundle of one or more crates that provides a set of functionality (contains `Cargo.toml` file)

Root of project: Cargo follows convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.

### Documentation
Use `///` for *documentation comments*. Only for documenting the public API items to tell how the API should be *used* (not how it is implemented). Supports Markdown notation.

Method example:
```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

#### Generate Documentation
```sh
cargo doc
cargo doc --open // opens the results in a web browser
```
Generates documentation for current crate (and all crate's dependencies)

#### Documentation Comment Tests
`cargo test` automatically runs the code examples included in documentation

#### Commenting Contained Items
`//!` adds documentation to the item that contains the comments rather than to the items following the comments.

Example in `src/lib.rs` file (crate root):
```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

### Exporting Public API with `pub use`
Allows the organization of your code to be different than the hiearchy when exporting the public API.

Example:
```rs
// the below three statements put these all at the top level for public use (Re-exports)
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

### Cargo Workspaces
Helps manage multiple related packages that are developed in tandem.

A *workspace* is a set of packages that share the same *Cargo.lock* and output directory.

#### Set up
https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

#### Testing
Run tests for a specific crate:
```sh
cargo test -p crate_name
```

### Installing Binaries with `cargo install`
Allows you to install and use binary crates locally (on the command line).


  
## Writing Automated Tests
To change a function into a test function, add `#[test]` on the line before `fn`

### Run Tests
```sh
cargo test
```

### Writing Tests
#### Asserts
```rs
assert!(bool)
assert_eq!(exp1, exp2) // exps must implement `PartialEq` and `Debug` traits
assert_ne!(exp1, exp2) // exps must implement `PartialEq` and `Debug` traits
```

When asserting equal structs and enums I define myself, need to implement `PartialEq` to assert equality of those types & `Debug` to print the values when the assertion fails. Usuall can be accomplished by adding `#[derive(PartialEq, Debug)]` annotation to the struct or enum definition.

#### Error Messages in Asserts
Add arguments to assert macros to pass format strings as custom messages:
```rs
assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was `{}`",
    result
);
```

#### Checking for Panics
use `should_panic` to check for panics.
```rs
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

Adding the `expected` in the parenthesis ensure the given text is found in the panic message.

#### Using `Results<T, E>` in Tests
```rs
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

Allows you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.

#### Assert Err
To assert that an operation returns an `Err`: 
```rs
assert!(value.is_err())
```

### Running Tests
#### Help
```sh
cargo test --help # display the options you can use with `cargo test`
cargo test -- --help # display the options you can use after the seperator (arguments to the test binary)
```

#### Running Tests Consecutively
```sh
cargo test -- --test-threads=1
```

#### Show Output of Passing Tests
```sh
cargo test -- --show-output
```

#### Run Subset of Tests by Name
```sh
cargo test substring // run every test containing `substring`
```
Run all the tests in a module by filtering on the module's name.

#### Ignoring Tests
```rs
#[test]
#[ignore] // add this line
fn expensive_test() {}
```

Run only the ignored test:
```sh
cargo test -- --ignored
```

Run all tests, ignored or not:
```sh
cargo test -- --include-ignored
```

#### Running Integration Tests
```sh
cargo test --test file_name # run all tests in given integration test file
```

### Test Organization
*Unit tests*: small and more focused, testing one module in isolation at a time, and can test private interfaces

*Integration tests*: entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test

#### Unit Test
Put unit tests in the *src* directory in each file with the code that they're testing. Create a module named `tests` in each file to containt the test functions (annotation: `#[cfg(test)]`).

Private functions *can* be tested in Rust.

#### Integration Tests
Create `tests` directory in project's top level for integration test files (Each file in `tests` directory is a seperate crate)
```rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

Cannot create integration tests for binary crates.

#### Submodules in `tests`
Add a submodule (directory) to `tests` in order to provide shared code.

Example (adding `tests/common/mod.rs`):
```rs
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```
Calling from `tests/integration_test.rs`:
```rs
use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

# Concepts
## Ownership
- All heap data must be owned by exactly one variable

### Deallocation
- Rust deallocates heap data once its owner goes out of scope (calls `drop()` function of the owner)


### Moving
- Ownership can be transferred by moves, which happen on assignments and function calls

The following code assigns `b` to point to `a`'s data on the heap, and `a` is invalidated. It has been *moved*.
```rs
let a = Box::new([0; 1_000_000]);
let b = a;
```
Thus, `a` owns the box at the first line. Then the next line moves ownership of the box from `a` to `b`. This is done whenever the type in question (`Box` in this case) does not implement the `Copy` trait.

When the `Copy` trait is implemented & the data is stored on the stack (they go hand in hand): The following code *copies* the contents of `a` into `b`, so they have seperate values. This is because arrays and tuples implement the `Copy` trait if all their elements do.
```rs
let a = [0; 1_000_000];
let b = a;
```

### Current Owner Use
- Heap data can only be accessed through its current owner, not a previous owner (i.e. variables cannot be used/accessed after they have been *moved*)

Ex: when a string is given as an argument in a method, this moves ownership of the string to the parameter in the function. The pointer to the data is copied: (https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html#collections-use-boxes)

### Cloning Avoids Moves
- `.clone()` performs a deep copy and does not *move* data
```rs
let first = String::from("Ferris");
let first_clone = first.clone();
let full = add_suffix(first_clone);
println!("{full}, originally {first}");
```
At the end of this both `first` and `full` are safe to use.

### Ownership in Functions
Passing a value to a function will move or copy, just as assignment does.

## References and Borrowing
### Rules
- At any given time, you can have either one mutable reference or any number of immutable references
- References must always be valid

### References
- References are non-owning pointers (They *borrow*)
```rs
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable References
- If you have a mutable reference to a value, you can have no other references to that value (while that mutable reference is being used)
```rs
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### Dereferencing Pointers
Rust *implicitly* inserts dereferences and references in certain cases, such as calling a method with the dot operator
```rs
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
```

Explicit dereferencing:
```rs
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value,
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value
```


## Concurrency
### Threads for Simultaneous Code
Rust uses 1:1 model of thread implementation: a program uses one OS thread per one language thread.

#### Creating New Thread
Use `thread::spawn` function and pass it a closure (the code we want to run in the new thread)
```rs
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
Note: when the main thread of a Rust program completes, al spawned threads are shut down.

#### Waitting for all Threads to Finish
Use `join` Handles to wait for all threads to finish.

`thread:spawn` returns an owned `JoinHandle`, on which that the `join` method can be called.
`join` method will wait for it's thread to finish.
```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // blocking call
}
```

#### Using `move` Closures with Threads
Use `move` keyword with closures passed to `thread::spawn` in order to force the closure to take ownership of the values it uses from the environment.

Problem (compile error), where `v` is only borrowed on not moved to the new thread:
```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

Solution is to force the closure to take ownership of the values it's using rather than just borrowing it:
```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

### Message Passing to Transfer Data Between Threads
Rust provides an implementation of *channels*.
* A *channel* is closed if either the transmitter or receiver half is dropped.
* Channels can have multiple *sending* ends, but only one *receiving* end.

#### Create new Channel
Use `mpsc::channel` function to create new channel (`mpsc` stand for *multiple producer, single consumer*)
```rs
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

#### Transmitting
```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```
* `recv`: Blocking call to receive a value
* `try_recv`: Non-blocking call that returns immediately

#### Ownership Transference
`send` function takes ownership of its parameter, then the receiver takes ownership of it.

#### Sending Multiple Values
Can iterate over receiver to get messages:
```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

#### Creating Multiple Producers
Can create multiple producers by cloning the transmitter.
```rs
let (tx, rx) = mpsc::channel();

let tx1 = tx.clone();
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

### Shared-State Concurrency
Multiple threads access the same shared data.

#### Mutexes to Access Data
*Mutex* allows only one thread to access some data at any given time. *Lock* is the data structure that is a part of the mutex that keeps track of who currently has exclusive access to the data.

Use Rules:
* Attempt to acquire the lock before usage
* Unlock the data after use

##### `Mutex<T>` API
```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // blocking call
        *num = 6;
    }

    println!("m = {:?}", m);
}
```
`lock` method used to *acquire* the lock, a blocking call. Fails if another thread holding the lock panics. Returns `MutexGuard`, a smart pointer that points to the data (mutalbe reference). This smart pointer releases the lock automaticaly when a `MutexGuard` goes out of scope.

#### Sharing a `Mutex<T>` Between Threads - Atomic Reference Counting with `Arc<T>`
Use Atomic Reference Counting with `Arc<T>`, which is like `Rc<T>` but safe to use in concurrent situations (*atomically reference counted*).

```rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Note: for simple numerical operations, there are types simpler than `Mutex<T>`.


### Extensible Concurrency with `Sync` and `Send` Traits
#### `Send` Trait
Allows transference of ownership between threads. 

`Send` (`std::marker`) trait indicates that ownership of values of the type implementing `Send` can be trasnferred between threads.

Almost all primitive types are `Send`, aside from raw pointers.

#### `Sync` Trait
Allows access from Multiple Threads. 

The `Sync` (`std::marker`) trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread.

The smart pointer `Mutex<T>` is `Sync` and can be used to share access with multiple threads.

#### Implenting `Send` and `Sync`
Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and `Sync`, we don't have to implement those traits manually.



## Object-Oriented Programming Features of Rust
### Characteristics of Object-Oriented Languages
#### Encapsulation Example
Only public members of `AveragedCollection` are `add`, `remove`, and `average` methods:
```rs
pub struct AveragedCollection {
    list: Vec<i32>, 
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

#### Inheritance in Rust
Rust does not have inheritance, but solves the two use cases.

1. For reuse of code: can be implemented with traits
2. Polymorphism (enable child type to be used in the same places as the parent type): Rust uses generics to abstract over different possible types and trais bounds to impose constraints on what those types must provide.


### Using Trait Objects For Polymorphism
#### Trait Objects
A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtiime.

Create a trait object by specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then using the `dyn` keyword, and then specifying the relevant trait.

Define `Draw` trait:
```rs
pub trait Draw {
    fn draw(&self);
}
```

Define `Screen` struct with vector holding types implementing `Draw`:
```rs
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

While a generic type parameter can only be substituted with on concrete type at a tyime, trait objects allow for multiple concrete types to fill in for the trait object at runtime.

Types implementing `Draw`:
```rs
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

All together:
```rs
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

#### Performance: Trait Objects Perform Dynamic Dispatch
Compiler emits code that at runtime will figure out which method to call.

This lookup incurs a **runtime cost** that doesn't occur with static dispatch. It also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations.


### Implementing an Object-Oriented Design Pattern
Implementing the *state pattern*.

#### Traditional implementation using the trait objects
https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

#### Encoding States and Behavior as Types
```rs
// src/lib.rs
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// src/main.rs
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```






# Built-In Package
## Variables
### Mutability
- Variables are immutable by default

Mutable variable:
```rs
let mut x = 5;
```

### Constants
- Constants are always immutable
- The type of the value is required
- Must be set only to a constant expression
- Only they can be used in global scope (not `letf` variables)
- Naming convention: SNAKE_CASE
```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Shadowing
- Shadowing: redefining a variable name

## Data Types
- Scalar types: integers, floating-point numbers, Booleans, characters
- Compound types:
    - Rust primitive: tuples, arrays

### Integer Types
- Default Integer type: `i32`
- `isize` & `usize` may be used when indexing some sort of collection

Integer Types:
| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Integer Literals:
| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

### Floating-Point Types
- Default floating-point type: `f64`. 
- Type options: `f32`, `f64`

### The Boolean Type
```rs
let t = true;
let f: bool = false; // with explicit type annotation
```

### The Character Type
- `char` type is 4 bytes in size
```rs
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
```

### The Tuple Type
- Fixed length: cannot grow or shrink once declared
```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
#### Destructuring (Pattern Matching)
```rs
let (x, y, z) = tup;
```

#### Accessing
```rs
let five_hundred = tup.0;
let six_point_four = tup.1;
```

#### Empty Tuple
- Called a `unit`: `()`
- Default expression return value if not something else

### The Array Type
- Arrays must have fixed length
- Data allocated on the stack
- `vector` type (standard library) *is* allowed to grow/shrink
```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // equals let a = [3, 3, 3, 3, 3];
```

#### Accessing Array Elements
```rs
let first = a[0];
let second = a[1];
```

#### Invalid Array Element Access
- Index in bounds runtime check

### The Slice Type
- Reference a contiguous sequence of elements in a collection rather than the whole collection. Since a slice is a kind of reference, it does not have ownership.

```rs
let s: String = String::from("hello world");
let hello: &str = &s[0..5];
let world: &str = &s[6..11];
```

```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];
```

#### Range Syntax
- `let slice = &s[0..2];` is the same as `let slice = &s[..2];`
- `let slice = &s[3..len];` is the same as `let slice = &s[3..];`
- `let slice = &s[0..len];` is the same as `let slice = &s[..];`

#### String Literals
```rs
let s = "Hello, world!";
```
The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

#### Slice Type as Parameter Type
```rs
fn my_function(s: &str)
```
The above allows the same function on both `&String` and `&str` values
```rs
// `my_function` works on slices of `String`s, whether partial or whole
my_function(&my_string[0..6]);
my_function(&my_string[..]); 
// `my_function` also works on references to `String`s, which are equivalent
// to whole slices of `String`s
my_function(&my_string);
```


## Functions
- Naming convention is *snake_case*

### Parameters
```rs
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

### Statements and Expressions
Expression:
```rs
let y = {
    let x = 3;
    x + 1
}; // curly braces block is an epression with return `x+1`
```

### Functions with Return Values
```rs
fn main() {
    let x = plus_one(5);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## Control Flow
### If
```rs
if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

If in a let statement:
```rs
let condition = true;
let number = if condition { 5 } else { 6 };
```

### Loop
- Infinite loop
```rs
loop {
    println!("again!");
}
```

#### Returning Values from Loops
- Purpose: retrying an operation that might fail
```rs
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

#### Loop Labels
- Can specify a *loop label* on a loop that then can use with `break` or `continue`
```rs
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;
    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }
    count += 1;
}
```

### While
```rs
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}
```

### For
```rs
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {element}");
}
```

#### For Loop Range
```rs
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

#### For Loop Pattern Matching
```rs
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

## Structs
### Basics
```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

### Field Init Shorthand
```rs
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Struct Update Syntax
Shorthand for assigning one struct to another, while updating variables.
```rs
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Struct update syntax uses `=` like an assignment. So it *moves* any field data that is on the heap, and *copies* field data that is on the stack. Thus it is possible that the original struct is no longer valid.

### Tuple Structs
Struct with no names to the fields. Useful when wanting to create a distinct type for a tuple.
```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-Like Structs
Useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
```rs
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

### Debug Printing
Adding `#[derive(Debug)]` above struct definition allows debug printing of your struct.
```rs
println!("rect1 is {:?}", rect1); // regular debug print of struct
println!("rect1 is {:#?}", rect1); // prettier debug print of struct
```
#### Debug Macro
```rs
dbg!(&rect1);
```

### Methods
- their first parameter is always `self`, which represents the instance of the struct the method is being called on
```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
Called with dot operator on Rectangle instance: `.area()`

### Associated Functions
Does not take the `self` parameter. Called with `::` syntax.
```rs
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```
Call: `let sq = Rectangle::square(3)`

## Enums and Pattern Matching
### Basics
```rs
enum IpAddrKind {
    V4,
    V6,
}
```
```rs
let four: IpAddrKind = IpAddrKind::V4;
let six: IpAddrKind = IpAddrKind::V6;
```
Both of the above are of type `IpAddrKind`

Attach data to each variant of the enum directly:
```rs
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

Attach structs to each variant of the enum:
```rs
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Another Example:
```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // named fields, like a struct
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Use `impl` with enums:
```rs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### Option Enum
```rs
enum Option<T> {
    None,
    Some(T),
}
``` 

### Match Statement
#### Basics
```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Accessing a value:
```rs
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

#### Matching with `Option<T>`
```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

#### Catch-all Patterns
Cover all possible values:
```rs
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // catch all pattern
}
```

Use `_` to catch-all, but not use the value
```rs
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // do nothing - empty tuple type (unit value)
}
```

#### Multiple Patterns
Use `|` to match multiple patterns
```rs
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

#### Matching to Range
Use `..=` to match to an *inclusive* range of values
```rs
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

### If Let Statement
Less verbose way to handle values that match one pattern while ignoring the rest.

Instead of:
```rs
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

Do:
```rs
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

Can also use `else`:
```rs
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

### While Let Conditional Loop
Allows a `while` loop to run fo as long as a pattern continues to match:
```rs
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### Let statement
```rs
let (x, y, z) = (1, 2, 3);
```

### Function Parameters
```rs
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

### Refutability: Whether a Pattern Might Fail to Match
*irrefutable*: patterns that will match for any possible values passed. Ex: `x` in `let x = 5`.

*refutable*: patterns that can fail to match for some possible value: Ex: `Some(x)` in `if let Some(x) = a_value`

### Destructuring
#### Destructuring structs
```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // **This line**
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

With same field names:
```rs
fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

Matching:
```rs
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

#### Destructuring Enums
```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```

#### Destructuring Nested Structs and Enums
```rs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```

Structs and Tuples:
```rs
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

#### Ignoring Remaing Parts of a Value with `..`
Use `..` to use specific parts and ignore the rest:
```rs
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

With a tuple:
```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

### Match Guards
*match guard*: an additional `if` condition, specified after the pattern in a `match` arm, that must also match for that arm to be chosen.
```rs
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {} is even", x),
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
```

### At Operator/Bindings (`@`)
The *at* operator `@` lets us create a variable that holds a value at the same time as we're testing that value for pattern match.

```rs
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
}
```

## Error Handling
### Panic
```rs
panic!("crash and burn");
```

### Recoverable Errors with `Result`
```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Using `match` statement:
```rs
use std::fs::File;
fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

#### Matching on Different Errors
```rs
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

#### `unwrap`
If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:
```rs
let greeting_file = File::open("hello.txt").unwrap();
```

#### `except`
Similarly, the `expect` method lets us also choose the `panic!` error message.
```rs
let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
```

### Propogating Errors
Return a `Result`:
```rs
use std::fs::File;
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

Using the question mark operator:
```rs
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```
If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.

There is a difference between what the `match` expression from Listing 9-6 does
and what the `?` operator does: error values that have the `?` operator called
on them go through the `from` function, defined in the `From` trait in the
standard library, which is used to convert values from one type into another.
When the `?` operator calls the `from` function, the error type received is
converted into the error type defined in the return type of the current
function. This is useful when a function returns one error type to represent
all the ways a function might fail, even if parts might fail for many different
reasons.

Even Shorter Code:
```rs
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

#### More on the `?` operator
The `?` operator can only be used in functions whose return type is compatible
with the value the `?` is used on. This is because the `?` operator is defined
to perform an early return of a value out of the function.

The error message also mentioned that `?` can be used with `Option<T>` values
as well.

### Custom Types for Validation
Custom type for integers from 1 to 100:
```rs
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## Generic Types, Traits, and Lifetimes
### Generic Data Types
#### Functions
```rs
fn largest<T>(list: &[T]) -> &T {
```

#### In Struct
```rs
struct Point<T> {
    x: T,
    y: T,
}
```

#### In enums
```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### In Methods
```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

### Traits
#### Definition
```rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```

#### Implementation
```rs
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Calling Implementation
```rs
use aggregator::{Summary, Tweet};
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
```

#### Default Implementations
```rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}
```

#### Traits as Parameters
```rs
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
This parameter accepts any type that implements the specified trait.

Shorthand form:
```rs
pub fn notify(item: &impl Summary) {
```


These two parameters can have different type that each implement `Summary`:
```rs
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

These two parameters must have the same type:
```rs
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

##### Multiple Trait Bounds
```rs
pub fn notify<T: Summary + Display>(item: &T) {
```
```rs
pub fn notify(item: &(impl Summary + Display)) {
```

##### `where` Clause
Instead of:
```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
Have:
```rs
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

#### Return Types
```rs
fn returns_summarizable() -> impl Summary {
```

### Lifetimes
Ensure that references are valid as long as we need them to be.

```rs
&'a i32     // a reference with an explicit lifetime
```

Show that the returned reference will be valid as long as both the parameters are valid:
```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rs
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```
When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.

#### In Structs
```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
The instance of `ImportantExcerpt` can't outlive the reference it holds in its `part` field

#### Lifetiime Elision
Full lifetime annotation:
```rs
fn first_word<'a>(s: &'a str) -> &'a str {
```

Shortcut for this case:
```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

#### In Methods
```rs
impl<'a> ImportantExcerpt<'a> { // lifetime annotations required if they appear in struct definition
```

#### The Static Lifetime
`'static`: the affected reference *can* live for the entire duration of the program. All string literals have the `'static` lifetime. 
```rs
let s: &'static str = "I have a static lifetime.";
```
The text of this string is stored directly in the program's binary, which is always available.

## Iterators and Closures
### Closures
Examples:
```rs
user_preference.unwrap_or_else(|| self.most_stocked())
```
```rs
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```
```rs
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

#### Assigning Types
If necessary compiler will infer closure types, often when the closue is first used. 
```rs
let example_closure = |x| x;
let s = example_closure(String::from("hello")); 
let n = example_closure(5); // this will cause COMPILE ERROR - changing type of closur
```

#### Capturing References or Moving Ownership
Immutable reference in closure:
```rs
let list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

let only_borrows = || println!("From closure: {:?}", list); // closure: immutable reference

println!("Before calling closure: {:?}", list);
only_borrows();
println!("After calling closure: {:?}", list);
```

Mutable reference in closure:
```rs
let mut list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

let mut borrows_mutably = || list.push(7); // closure: mutable reference
// immutable reference not allowed, due to mutable reference in use
borrows_mutably();
println!("After calling closure: {:?}", list); // mutable reference no longer in use
```

Force closure to tak ownership of the values it uses in the environment, use the `move` keyword before the parameter list. 
```rs
let list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

thread::spawn(move || println!("From thread: {:?}", list)) // move ownership of `list` to new thread
    .join()
    .unwrap();
```

#### Closure Traits
Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion, depending on how the closure’s body handles the values:
1. `FnOnce` applies to closures that can be called once. All closures implement
   at least this trait, because all closures can be called. A closure that
   moves captured values out of its body will only implement `FnOnce` and none
   of the other `Fn` traits, because it can only be called once.
2. `FnMut` applies to closures that don’t move captured values out of their
   body, but that might mutate the captured values. These closures can be
   called more than once.
3. `Fn` applies to closures that don’t move captured values out of their body
   and that don’t mutate captured values, as well as closures that capture
   nothing from their environment. These closures can be called more than once
   without mutating their environment, which is important in cases such as
   calling a closure multiple times concurrently.

### Iterators
#### `Iterator` Trait
All iterators implement the `Iterator` trait
```rs
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}
```

#### Get iterator
```rs
let mut v1_iter = v1.iter();
assert_eq!(v1_iter.next(), Some(&1));
```
`iter()`: returns immutable references
`into_iter()`: returns owned values
`iter_mut()`: returns mutable references

#### Methods that Produce Other Iterators
`collect()` method consumes the iterator (returned by `map()`) and collects the resulting values into a collection data type.
```rs
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

#### Filter
```rs
shoes.into_iter().filter(|s| s.size == shoe_size).collect()
```

## Smart Pointers
Provide functionality beyond what references provide. Allows you to have multiple owners.

In many cases smart pointers *own* the data they point to.

`String` and `Vec<T>` are smart pointers.

Usuall implemnted using structs, implementing the `Deref` and `Drop` traits.

### Using `Box<T>` to Point to Data on the Heap
Allow you to store data on the heap rather than the stack.

Use cases:
- When you have a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so
- When you want to own a value and you car only that it's a type that implements a particular trait rather than being of a specific type

#### Syntax
```rs
let b = Box::new(5);
```

#### Enabling Recursive Types with Boxes
```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

### `Deref` for Smart Pointers
Implementing the `Deref` trait allows you to customize the behavior of the *dereference* operator `*`.

#### Implementing `Deref`
```rs
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
When `*y` is called, `*(y.deref())` is run behind the scenes.

#### Deref Coercion
*Deref coercion*: in arguments to functions and methods if we pass a type that implements the `Deref` trait but doesn't match the parameter type in the function/method definition, a sequence of calls to the `deref` method converts the type we provided into the type the parameter needs (if possible).

Implement `DerefMut` trait to override the `*` operator on mutalbe references.

### `Drop` Trait
Specifies the code to run when a value goes out of scope (cleanup code).

```rs
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

#### Dropping a Value Early
Use the `std::mem::drop` function to cleanup the value early.
```rs
let c = CustomSmartPointer {
    data: String::from("some data"),
};
drop(c);
```

### `Rc<T>`, the Reference Counted Smart Pointer
Use `Rc<T>` (reference counting) to enable mutliple ownership (eg. a graph with each of a node's edge owning that node).

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

Calling `Rc::clone` causes the reference count to the data within Rc<T> to increase.

Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only.

### `RefCell<T>` and the Interior Mutability Pattern
*Interior mutability*: a design pattern that allows you to mutate data even when there are immutable references to that data

With `RefCell<T>`, the type represents single owernship over the data it holds, however the borrowing rules' invariants are enforced at *runtime* rather than *compile* time.

#### Choosing `Box<T>`, `Rc<T>`, or `RefCell<T>`
* `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
  have single owners.
* `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
  allows only immutable borrows checked at compile time; `RefCell<T>` allows
  immutable or mutable borrows checked at runtime.
* Because `RefCell<T>` allows mutable borrows checked at runtime, you can
  mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is
  immutable.

#### Interior Mutability Use Case: Mock Objects
See https://doc.rust-lang.org/book/ch15-05-interior-mutability.html for example

#### Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
`Rc<T>` holding a `RefCell<T>` allows you to have a value that can have multiple owners *and* that you can mutate: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

### Reference Cycles Can Leak Memory
Rust allows memory leaks by using `Rc<t>` and `RefCell<T>`. Possible to create references where items refere to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html



## Advanced Rust Features
### Unsafe Rust




# Standard Library
## Collections
- The data is stored on the heap, unlike the built-in array and tuple types

### Vectors
```rs
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3]; // using `vec!` macro
```
```rs
v.push(5);
```
```rs
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

#### Iterating over Vector
```rs
let v = vec![100, 32, 57];
for elem in &v {
    println!("{elem}");
}
```
Modifying elements:
```rs
let mut v = vec![100, 32, 57];
for elem in &mut v {
    *elem += 50;
}
```

#### Holding Multiple Types
'Mask' each type with Enum
```rs
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Alternative: use the trait object to make this work

### Strings
#### Creating
```rs
let mut s = String::new();
```
`to_string` method:
```rs
let data: &str = "initial contents";
let s: String = data.to_string();
// the method also works on a literal directly:
let s: String = "initial contents".to_string();
```
`String::from` method:
```rs
let s = String::from("initial contents");
```

#### Updating
Appending:
```rs
let mut s = String::from("foo");
s.push_str("bar");
s.push('s'); // for single char
// s = "foobars"
```
Concatenation:
```rs
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```
```rs
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

#### Slicing Strings
Return the first four bytes of the string
```rs
let hello = "Здравствуйте";
let s = &hello[0..4];
```

Note: if make a slice resulting in invalid characters -> program panic

#### Iterating over Strings
Iterate over characters:
```rs
for c in "Зд".chars() {
    println!("{c}");
}
// З
// д
```

Iterate over bytes:
```rs
for b in "Зд".bytes() {
    println!("{b}");
}
// 208
// 151
// 208
// 180
```

### Hash Maps
#### Including
```rs
use std::collections::HashMap;
```

#### Creating
```rs
let mut scores = HashMap::new();
```

#### Inserting
```rs
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

#### Accessing
```rs
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```
This program handles the `Option` by calling `copied` to get an `Option<i32>` rather than an `Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` doesn’t have an entry for the key.

Iterating:
```rs
for (key, value) in &scores {
    println!("{key}: {value}");
}
```

#### Ownership
For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.
```rs
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point
```

If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid.

#### Updating
Overwrite:
```rs
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

Add if not present:
```rs
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
```
The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not, it
inserts the parameter as the new value for this key and returns a mutable
reference to the new value.

Updating:
```rs
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word: &str in text.split_whitespace() {
    let count: &mut i32 = map.entry(word).or_insert(0);
    *count += 1;
}
```



# Other Packages
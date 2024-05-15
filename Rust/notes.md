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
### Overview:
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

### `match` Control Flow
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

Accessing a value:
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

### `if` `let`
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
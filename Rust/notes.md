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



# Standard Library Package

# Other Packages
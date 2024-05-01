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
- Only they can be used in global scope (not `let` variables)
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
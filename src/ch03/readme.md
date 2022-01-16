# Chapter 03 - Common Programming Concepts

- `//` for comments

- Variables are immutable by default (and its favoured most of the time)
    * Can be made mutable using `mut`
    * Can't be declared globally
    * Can be set to result of a value which is only known at runtime
    * Can be shadowed to by variable of same name but different types or values
    * Naming convention: lower case with underscores

    ```rust
    let mut b = 5;
    b = 7; // allowed
    
    let a = 5;
    a = 6; // not allowed
    let a = '6'; // shadowing allowed
    ```

- Constants are values bound to a variable and are not allowed to change
    * Can't use `mut` with constants
    * Type must be annotated
    * Can be declared globally
    * May only be set to constant expressions (result known at compile time)
    * Naming convention: all caps with underscores
    * Cannot be shadowed

    ```rust
    const SECONDS_IN_HOUR: u32 = 60 * 60
    ```

- Type annotation is not necessary when it can be inferred

- Scalar types: integers, floating point, characters and boolean

- Integers:
    * `isize` or `usize` depends on platform
    * `iN` or `uN` where `N` can be 8, 16, 32, 64 and 128 bits
    * Integer literal is `i32` by default
    * Literals can be underscore separated and suffixed with type. e.g. `10_000u32`
    * `0xaa` for hex, `0o77` for octal, `0b1101` for binary and `b'A'` for byte
    * Compiling in debug mode will raise error on integer overflow

- Other scalar types:
    * `f32` or `f64` (default) for floating point numbers
    * `true` or `false` for boolean
    * `char` type is 4 bytes in size representing unicode scalar value. e.g. `'\u{0000}'`

- Compound types: tuple and array (both have fixed lengths and allocated on stack)

- Tuples:

    ```rust
    let tup = (2, '4', 6.6);
    assert_eq!(tup.0, 2)    // access members
    
    let tup: (i32, char, f64) = (2, '4', 6.6); // with annotation

    let (x, y, z) = tup;    // unpack tuples

    let empty_tuple = ();
    ```

- Arrays:

    ```rust
    let arr = [2, 6, 8, 4];
    let a = [3, 'A'];           // not allowed

    let arr: [i32; 4] = [2, 6, 8, 4];   // with annotation
    let arr = [3; 5];           // initialize 5 elements with value 3

    println!("{}", a[10]);      // will panic during runtime
    ```

- Functions:
    * `Statements` do not return a value, but `Expressions` do
    * `let y = 5` is a statement and not an expression
    * Expressions ending with semicolons are statements
    * Parameter's type annotation is part of function signature, while argument is the actually value passed to function
    * Explicit `return` is not required when returning a value (or result of expression)

    ```rust
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    fn main() {
        // example of function call
        assert_eq!(sum(5, 6), 11);

        // example of expression
        let y = {
            let x = 5;
            x * 2
        };
    }
    ```

- Conditionals
    * Only expressions evaluating to boolean value can be used in conditionals
    * When using `if` / `else` with `let`, values returned in either condition should be of same type
    * When using loops with `let`, `break` can be used to return value
    * `break` and `continue` apply to inner-most loop only
    * `1..10` represents a range with last value excluded
    
    ```rust
    // simple if / else
    if 4 < 5 {
        println!("true");
    } else if 5 < 6 {
        println!("another true");
    } else {
        println!("false");
    }

    // if / else with let
    let val = if 4 < 5 { 100 } else { 200 }
    // loop with let
    let val = loop {
        println!("loop !");
        break 60;
    };
    // while loop
    while true {
        println!("loop !");
        break;
    }
    // for loop with array
    let arr = [2, 3, 5];
    for item in arr {
        println!("{}", item);
    }
    // for loop with range
    for i in (1..10).rev() {
        println!("{}", i);
    }
    ```

- Names ending with `!` are macros. e.g. `println!()`
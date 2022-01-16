# Chapter 04 - Understanding Ownership

- Memory is managed through a system of `ownership` with a set of rules checked by compiler

- Accessing / putting things in `stack` is faster than in `heap`

- Ownership rules:
    * Each value in rust has a variable that's called its `owner`
    * There can only be one owner at a time
    * When the owner goes out of scope, the value will be dropped

- Strings:
    * `String`s are mutable but String literals (`&str`) are not
    * String literals are hardcoded directly into the executables
    * Memory allocated on heap (when using String) is automatically returned when the owning variable goes out of scope
    * Assigning or returning strings `moves` it (instead of shallow/deep copy)
    * Use `clone` method on string to deep copy

- `Copy` trait is implemented by types whose values reside on stack
    * It can't be implemented for types for which `Drop` is implemented

- Unlike pointer, a `reference` is guaranteed to point to a valid value of particular type
    * `&` for referencing and `*` for dereferencing
    * Creating a reference is usually known as `borrowing`
    * `reference`s are immutable by default, hence use `&mut` for mutable references
    * One can have multiple immutable references or only one mutable reference at a time
    * If a variable has a reference to value, doesn't mean it owns the value

- Rust compiler guarantees:
    * no data races
    * no dangling references

- Rust compiler is smart enough to detect whether a variable is used anymore (before reaching its scope) by using something called `Non-Lexical Lifetimes (NLL)`

    ```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    ```

- `Slices` hold reference to a contiguous sequence of elements
    * String literals are slices

    ```rust
    let s = String::from("hello world");
    // string slices
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s;

    let a = [1, 2, 3];
    // array slices
    let b = &a[..2];
    ```
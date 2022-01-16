# Chapter 06 - Enums and Pattern Matching

- Enums:

    ```rust
    // example of enum with variants containing different types
    enum Message {
        Quit,                           // no data associated with it
        Move { x: i32, y: i32 },        // named fields
        Write(String),                  // String
        ChangeColor(i32, i32, i32),     // tuple of i32
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    ```

- `Option` enum replaces the need to use `null`
    * `Some` holds a value of generic type
    * Need to annotate type of `Option` when used in let and assigning `None`
    * To use the underlying value, need to exhaustively check for each variant

    ```rust
    enum Option<T> {
        None,
        Some(T),
    }

    let some_string = Some("a string");
    let none: Option<i32> = None;
    ```

- `match` is way more powerful than `switch` since it incorporates a concept of pattern matching and exhaustive checking of variants

    * Use `_` as catchall pattern

    ```rust
    let some_string = Some("a string");

    match some_string {
        Some(s) => println!("string: {}", s),
        None => println!("none"),
    }
    ```

- `if let` can be used as an alternative for shorter match

    ```rust
    let some_string = Some("a string");

    if let Some(s) = some_string {
        println!("string: {}", s);
    } else {
        println!("none");
    }
    ```
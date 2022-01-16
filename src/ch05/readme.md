# Chapter 05 - Using structures

- `struct` is a custom data type that lets you name and package together multiple related values

    ```rust
    // a normal struct with derived trait (to print fields)
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // unit struct
    struct AlwaysEqual;

    // usual struct init
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    let b = build_user("name", "user");
    println!("{:?}", b);

    // field init shorthand when variable names are same as field names
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    // struct update syntax
    fn build_user(user: User) -> User {
        User {
            active: true,
            sign_in_count: 1,
            ..user
        }
    }
    ```

- To modify fields in `struct`s, entire struct needs to be mutable

- Use `derived traits` to print struct
    * `Display` for `println!("{}", v)`
    * `Debug` for `println!("{:?}", v)` or `println!("{:#?}", v)` (pretty-print)
    * `Debug` for `dbg!(&v)`

- All the struct fields need to be valid as long as entire struct is valid
    * Need to use `lifetimes` if a field holds data owned by others

- Tuple struct:

    ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    ```

- `Methods` are defined within context of a struct
    * Type `Self` is an alias for type that `impl` block is for
    * `&self` is a short for `self: &Self`
    * Methods can have same name as fields (and needs to be called with parentheses)
    * Because of `automatic referencing and dereferencing`, we don't need to `(*obj).method()`
    * Use `::` for calling `associated functions` (functions defined in context of type and not its instance)
    * Can have multiple `impl` blocks

    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn new(width: i32, height: i32) -> Rectangle {
            Rectangle {
                width,
                height,
            }
        }
    }

    fn main() {
        let rect1 = Rectangle::new(30, 40);

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }
    ```
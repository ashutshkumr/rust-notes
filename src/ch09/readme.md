# Chapter 08 - Error Handling

- `Result<T, E>` for recoverable errors and `panic!()` for unrecoverable errors

- Add following in `Cargo.toml` to abort instead of unwinding stack upon calling `panic!`

    ```toml
    [profile.release]
    panic = 'abort'
    ```

- Execute binary with `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full` environment variables to print backtrace upon `panic!`

- Example of error handling:

    ```rust
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }
    ```

- Helpers on `Result<T,E>`:
    * `unwrap` will return value inside `Ok()` or will panic with error inside `Err()`
    * `expect` works similar to unwrap, additionally letting users prefix an error message
    * `?` at the end of expression returning `Result` returns value inside `Ok()` or returns `Err()` from current function (uses `From` trait to convert error types if supported)
    * `?` can only be used for functions which return compatible error type as part of `Result` or `Option`
    * Can't mix and match `Result` and `Option` when using `?`
    
    ```rust
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    ```

- `main` function can return types that implement `std::process::Termination` trait

    ```rust
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
    ```
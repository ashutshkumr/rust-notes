# Chapter 07 - Packages, Crates and Modules

- A `package` can contain multiple *binary crates* and optionally one *library crate*
    * Contains `Cargo.toml` file describing how to build the crates
    * `crate root` is a source file that rust compiler starts from (`src/main.rs` for binary and `src/lib.rs` for library)
    * For multiple binary crates, crate root is inside `src/bin`

- A `crate` is a tree of modules that produces binary or library

- A `module` organizes code within a crate and controls privacy (public or private) of each item in it
    * All items are private by default unless they're preceded with a `pub` keyword
    * Struct fields will explicitly be needed to be marked as public after the struct itself has been marked as public
    * Enum variants are all public after marking enum itself as public

- Items inside modules are identified using `paths`
    * `use` brings a module or any of its public items into scope
    * `as` can be used to alias a name that has been brought into scope using `use` keyword
    * It's idiomatic to bring only parent name in scope for functions and bring identifier names itself in scope for structs, enums, etc.
    * `pub use` can re-expose a used item from current module

- `paths` can take two forms:
    * *absolute* - starts from crate root, using literal `crate::` (as root module)
    * *relative* - starts from current module, using literals `self::`, `super::` or an identifier in current module

- One can reference private functions from parent module

- One can reference private modules from parent module (sibling modules)

- Three ways of defining modules:
    * Define all modules in one file
    * Create separate file with the same name as of module inside `src`. Module name will need to be declared in parent module too.
    * Create separate folder with the same name as of module inside `src` and keep `mod.rs` inside it. Module name will need to be declared in parent module too.

- Prefer specifying absolute paths because it’s more likely to move code definitions and item calls independently of each other

- When using external package, add its name / version in `Cargo.toml` dependencies section

- Nested paths and glob
    ```rust
    use std::{cmp::Ordering, io};
    use std::io::{self, Write};
    use std::collections::*;
    ```
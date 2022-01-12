# Chapter 01 - Getting Started

- `rustup` to manage installation / upgrade of rust

    ```sh
    # Install:
    # on ubuntu (for c compiler and linker)
    sudo apt-get install build-essential
    # download and install rust
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

    # Update:
    rustup update

    # Uninstall:
    rustup self uninstall
    ```

- `rustc` is the compiler

    ```sh
    # compile
    rustc main.rs
    # run (on linux)
    ./main
    ```

- `rustup doc` for offline documentation; otherwise on linux servers:
    
    ```sh
    # this works for python3; replace toolchain if needed
    python -m http.server -d $HOME/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/share/doc/rust/html
    ```
# Learn Rust syntax
I write simple programs in Rust to understand the basic syntax and concepts.

# Overview

## Compilation
- Rust is compiled ahead of time, similar to C/C++.
- Use `rustc main.rs` to compile a source file into an executable.
- The generated executable can be run directly from the command line.

## Cargo
- Cargo is Rust’s build system and package manager, similar to npm.
- Create a new project using `cargo new <project_name>`.
- The `Cargo.toml` file defines project metadata and dependencies.
- Source files are placed in the `src/` directory.
- `cargo build` compiles the project and creates a `target/` directory.
- `cargo run` builds and runs the project in one command.

## Syntax
- `let a: [i32; 5] = [10, 20, 30, 40, 50];` defines an array with a fixed size.
- The array syntax is `[<type>; <number_of_elements>]`.
- Input can be read using `use std::io;` and `io::stdin().read_line(...)`.
- The `->` symbol is used to specify a function’s return type.
- Printing to the console requires format strings, e.g.  
  `println!("Fibonacci(10) = {}", res);`, similar to `%d` or `%s` in C/C++.

### `const` / `let` / `mut`
- `const` defines a constant value that is always immutable.
- Constants are evaluated at compile time and usually stored in read-only memory.
- `let` is used to declare variables at runtime.
- Variables declared with `let` are immutable by default.
- Adding `mut` allows the variable to be modified.

### Arrays and Vectors
- Arrays in Rust have a fixed length known at compile time.
- Array elements are stored contiguously in memory.
- Vectors (`Vec<T>`) are growable and allocated on the heap.
- Vectors are used when the size of the collection can change at runtime.

### Structs
- A `struct` is a custom data type that groups related values together.
- Struct fields are private by default.
- Structs are commonly used to represent state or complex data.

### `impl`
- The `impl` block is used to define methods for a struct.
- Functions inside an `impl` block are called methods.
- `Self` refers to the type being implemented.
- `impl` separates data definition from behavior.

### `pub`
- The `pub` keyword controls visibility.
- `pub` allows a struct, function, or method to be accessed from other modules.
- Without `pub`, items are private to the current module.
- Struct fields must be marked `pub` individually if they should be accessible.

### `unsafe`
- `unsafe` allows operations that the Rust compiler cannot guarantee to be safe.
- This includes raw pointer dereferencing and calling unsafe functions.
- `unsafe` does not disable safety checks globally; it only affects the marked block.
- It is used when low-level control is required.

### Macros and Functions
- Functions are defined using the `fn` keyword.
- Macros are invoked using `!`, such as `println!`.
- Macros operate on syntax and are expanded at compile time.
- Functions operate on values and are checked by the type system.

## Crates
- A crate is a compilation unit in Rust.
- Crates are similar to packages in Java or modules in other languages.
- External crates are added as dependencies in `Cargo.toml`.
- The Rust standard library is provided as the `std` crate.

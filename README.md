# Rust Fundamentals 3

This is a basic project to learn about `rust` language

## Rust Tools

  1. `rustup` - a tool for managing different versions of rust
  2. `rustc`  - a tool for compiling rust code
  3. `cargo`  - a tool for managing rust packages and projects
  4. `rustc --version` or `rustc -V` - to check `rust` version
  
  - This app was built using version `1.57.0`
  - To make this project work, You should have [rust](https://www.rust-lang.org/learn/get-started) installed globally in your machine.

## How to create a new rust project?

  1. Create a project directory

  ```bash
mkdir rust-fundamentals
  ```

  2. Open it in an editor(say VSC) - it's an optional stage

  ```bash
cd rust-fundamental && code .
  ```

  3. Update the rust versions - it's also an optional command

  ```bash
rustup update
  ```

  4. Create a new project by the following command

  ```bash
cargo init
  ```

## How to run a rust program?

```bash
cargo run
```

Note: `cargo` will use `rustc` to compile behind the scene

```bash
cargo run -q
```

The above one is to skip the extra status info after compilation

### VSC extensions

  - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
  - [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

### Resources

  * [Rust Package registry](https://crates.io/)
  * [Learn TOML in Y minutes](https://learnxinyminutes.com/docs/toml/)
  * [Rustup Book](https://rust-lang.github.io/rustup/)
  * [Awesome WebAssembly Languages](https://github.com/appcypher/awesome-wasm-langs)
  * [Primitive Data Types in Rust](https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types)
  * [Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)

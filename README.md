# Input
Utility library, which provides simple functionality to read and parse user input for command-line interfaces.

All types that implement **FromStr** are automatically supported.

## Usage

in **Cargo.toml**\:
```toml
[dependencies]
input = { git = "https://github.com/TheXimeng/input.git" }
```

### Examples
in **src/...** :
```rust
use input::*;

fn do_smth() {
  // primitives
  let line: String = input("> ");
  let x: u32 = input("u32: ");
  let y: f64 = input("f64: ");
  //...

  // complex
  let addr: std::net::IpAddr = input("ip: ");
  //...

  // custom - when T : FromStr, T::Err : Display
  let c: Custom = input("Custom: ");
  //...
}
```

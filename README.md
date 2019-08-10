# vmprotect
WIP VMProtect SDK for rust

# Features
## Basic VMProtect features:
### `protected!` macro for code

Syntax: 
```rust
use vmprotect::protected;

protected!("NAME"; TYPE [KEY]; {/*CODE*/})
```
- NAME: Which name will be displayed in VMProtect GUI
- TYPE: Protection type (mutate/virtualize/ultra per VMProtect docs)
- [KEY]: For virtualize/ultra only, require licenze activation to get this function to work
- CODE: Your code goes here

Protected code block is works like any other rust block, i.e:

```rust
use vmprotect::protected;

// Before protection
let a = {2+3};
// After protection
let a = protected!("Addiction"; virtualize false; {2+3});
```

Example:
```rust
#![feature(test)] // For black_box support

use vmprotect::protected;
use std::hint::black_box;

fn main() {
    // Blackboxes here is to not inline the math
    let a = black_box(2);
    let b = black_box(3);
    println!("{} + {} = {}", a, b, protected!("Adding"; ultra false; {
        a + b
    }));
}
```
### `protected!` macro for texts

Syntax: 
```rust
use vmprotect::protected;

protected!(TYPE "TEXT")
```
- TYPE: Text type, possible values: A (for normal c strings)/W (for uint16_t c strings (Used i.e in windows))
- TEXT: Your text, should be supported by your selected text type

This macro returns string, which can be transformed to normal one. This string is freed when dropped, implementations is located at `vmprotect::strings::{encrypted_a::EncryptedStringA, encrypted_w::EncryptedStringW}`

```rust
use vmprotect::protected;

// Before protection
let a = "Hello, world!";
// After protection
let a = protected!(A; "Hello, world!");
// Also for wide-strings:
let a = protected!(W; "Привет, мир!");
```

Example:
```rust
#![feature(type_ascription)] // For `.into(): T` syntax support

use vmprotect::protected;

fn main() {
    println!("Hello, {:?}!", protected!(A; "%Username%").into(): String);
}
```

## Licensing:
TODO Section
### HWID
Example:
```rust
println!("Your hwid is \"{}\"", vmprotect::licensing::get_hwid().to_str().unwrap());
```
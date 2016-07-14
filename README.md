# File I/O 1-liners

Read and write `Vec<u8>` with one function call.

```rust
extern crate file;

fn main() {
    let data = file::get("some_input_file.dat").unwrap();
    file::put("a.out", &data).unwrap();
}
```

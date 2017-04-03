# File I/O 1-liners

## `Vec<u8>`

`file::get()` and `file::put()` — read and write `Vec<u8>` with one function call.

```rust
extern crate file;

fn example() -> file::Result<()> {
    let data = file::get("some_input_file.dat")?;
    file::put("a.out", &data)?;
    Ok(())
}
```

`file::Result` is an alias for `std::io::Result`. You can use `Result<(), Box<std::error::Error>>` in places where you don't want to expose the error type.

## `String`

`file::get_text()` and `file::put_text()` — read and write `String` with one function call.

```rust
extern crate file;

fn example() -> file::Result<()> {
    let string = file::get_text("hello.txt")?;
    file::put_text("bye.txt", &string)?;
    Ok(())
}
```


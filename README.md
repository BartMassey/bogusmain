# status: do C-style return-from-main in Rust
Bart Massey 2021

This tiny nightly-only library is a start at doing
return-from-`main` style exit status reporting in Rust.

This is very much a work-in-progress and not yet ready for
use.

Here's an example:

```rust
fn main() -> Status {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.len() {
        0 => {
            status!(1, "usage: main <arg>")
        }
        1 => {
            println!("{}", args[0]);
            status!(0)
        }
        _ => {
            status!(-1, "unexpected argument {}", args[1])
        }
    }
}
```

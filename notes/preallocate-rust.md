# Preallocate file

If you know the file size and want to reduce reallocation while reading:

```rust
use std::fs::{self, File};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("large.txt")?;
    let mut contents = String::with_capacity(metadata.len() as usize);

    File::open("large.txt")?.read_to_string(&mut contents)?;
    Ok(())
}
```

This still stores the entire file in RAM, but avoids repeated growth of the internal buffer.

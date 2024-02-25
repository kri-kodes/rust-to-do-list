## Basic to-do list created with rust

### Requirements

Please check Cargo.toml file for all the required crates

### Running

There are multiple ways you can run the program

- `Cargo run -- -j <file-name> <actions>`
- Run it in a release mode where you do not have to deal with cargo again
  1. `cargo run --release`
  2. `export PATH="$PATH:/path/to/your/file/target/release"` (located in target/release/ directory)
  3. `journalname -j <file-name> <actions>`

# count_exts

A fast and efficient command-line utility that counts file extensions from stdin input, providing sorted statistics of file types in a directory.

## Features

- Reads file paths from stdin
- Case-insensitive extension counting
- Sorts results by frequency
- Handles files without extensions
- Efficient processing of large inputs
- Async support with Tokio runtime

## Installation

### From Source

Make sure you have Rust installed ([rustup](https://rustup.rs/)), then:

```bash
git clone https://github.com/rakki194/count_exts
cd count_exts
cargo install --path .
```

## Usage

The utility reads file paths from stdin and outputs extension statistics. You can pipe the output of commands like `find` or `ls` into it:

```bash
# Count extensions in current directory
find . -type f | count_exts

# Count extensions in specific directory
find /path/to/directory -type f | count_exts

# Using with ls
ls -1 /path/to/directory | count_exts
```

### Example Output

```bash
.txt: 42
.rs: 15
.toml: 3
[no extension]: 2
.md: 1
```

## Output Format

- Extensions are displayed with a leading dot (e.g., `.txt`)
- Files without extensions are shown as `[no extension]`
- Results are sorted by frequency (ascending order)
- Each line shows the extension followed by its count: `extension: count`

## Dependencies

- `xio` (v0.1.4) - Internal I/O utilities
- `anyhow` (v1.0.96) - Error handling
- `tokio` (v1.43.0) - Async runtime support

## Development

### Running Tests

```bash
cargo test
```

The test suite includes various scenarios:

- Basic extension counting
- Handling files without extensions
- Case sensitivity
- Empty input handling
- Empty line handling

### Building

```bash
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Repository

- GitHub: [https://github.com/rakki194/count_exts](https://github.com/rakki194/count_exts)
- Documentation: [https://docs.rs/count_exts](https://docs.rs/count_exts)

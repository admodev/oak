# Oak MVP Build and Deployment Guide

## System Requirements

### Minimum Requirements

- **OS:** Linux, macOS, or Windows
- **Rust:** 1.70 or later
- **LLVM:** 18.0
- **Disk Space:** ~2GB (for dependencies)

### Recommended Setup

- **CPU:** Multi-core processor (4+ cores)
- **RAM:** 4GB minimum (8GB+ recommended)
- **Disk:** SSD for faster builds

## Installation

### 1. Install Rust

Visit [https://rustup.rs/](https://rustup.rs/) and follow the installation steps.

```bash
# On Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows (PowerShell)
# Download and run the installer from rustup.rs
```

### 2. Install LLVM 18

#### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install llvm-18 llvm-18-dev clang-18
```

#### macOS

```bash
brew install llvm@18
```

#### Windows

Download from [LLVM releases](https://github.com/llvm/llvm-project/releases)

### 3. Clone and Build Oak

```bash
# Clone the repository
git clone https://github.com/admodev/oak.git
cd oak

# Build the project
cargo build --release

# The binary will be at: target/release/oak
```

## Usage

### Running a Script

```bash
oak examples/hello_world.oak
```

### Interactive REPL

```bash
oak -r
```

### Compiling to Intermediate Representation

```bash
# Generate LLVM IR
oak -c examples/math_operations.oak -o output.ll

# View the generated IR
cat output.ll
```

### Compiling to Object File

```bash
# Using LLVM tools
llc -filetype=obj output.ll -o output.o

# View object file info
objdump -d output.o | head -50
```

### Creating Executable

```bash
# Link the object file
gcc output.o -o program

# Run the executable
./program
```

## Project Structure

```
oak/
├── Cargo.toml              # Project manifest
├── src/
│   ├── lib.rs             # Library root
│   ├── bin/
│   │   └── oak.rs         # Main CLI entry point
│   ├── tokenizer/
│   │   └── mod.rs         # Lexer
│   ├── parser/
│   │   └── mod.rs         # Parser & AST
│   ├── types.rs           # Type system
│   ├── semantic.rs        # Semantic analysis
│   ├── codegen.rs         # LLVM code generation
│   ├── errors.rs          # Error types
│   ├── interpreter/
│   │   └── mod.rs         # AST interpreter (legacy)
│   ├── runtime/
│   │   └── mod.rs         # Runtime entry point
│   ├── repl/
│   │   └── mod.rs         # REPL implementation
│   ├── math/
│   │   └── mod.rs         # Math library
│   └── tests/
│       └── mod.rs         # Tests
├── examples/
│   ├── hello_world.oak
│   ├── math_operations.oak
│   ├── arrays_pointers.oak
│   ├── extern_c_functions.oak
│   └── structs.oak
├── docs/
│   └── MVP_REFERENCE.md   # This file
└── README.md
```

## Development Workflow

### Making Changes

1. **Edit source files** in `src/`
2. **Check syntax:** `cargo check`
3. **Run tests:** `cargo test`
4. **Build:** `cargo build`
5. **Test examples:** `oak examples/*.oak`

### Adding New Features

1. Update tokenizer if new syntax needed
2. Extend parser for new constructs
3. Add type checking in semantic analyzer
4. Implement code generation
5. Add tests and examples

## Debugging

### Enable Debug Output

```bash
# Build with debug symbols
cargo build

# Run with verbose output
RUST_BACKTRACE=1 cargo run -- examples/hello_world.oak
```

### Using LLDB/GDB

```bash
# Linux/macOS
lldb ./target/debug/oak
# or
gdb ./target/debug/oak

# Windows
windbg.exe ./target/debug/oak.exe
```

### Inspecting LLVM IR

```bash
# Generate and inspect IR
oak -c program.oak -o program.ll
cat program.ll
```

## Performance Optimization

### Release Build

```bash
cargo build --release --opt-level=3
```

### LLVM Optimization Passes

```bash
# Aggressive optimization
opt -O3 program.ll -o program.opt.ll
```

## Continuous Integration

### GitHub Actions Workflow

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: sudo apt-get install llvm-18
      - run: cargo build --release
      - run: cargo test
```

## Troubleshooting

### Common Issues

**Issue:** `LLVM not found`

```bash
# Solution: Set environment variable
export LLVM_CONFIG=/usr/bin/llvm-config-18
```

**Issue:** `Cargo build fails with linking errors`

```bash
# Solution: Clean and rebuild
cargo clean
cargo build
```

**Issue:** `Command 'oak' not found`

```bash
# Solution: Add to PATH
export PATH="$PATH:$(pwd)/target/release"
```

## Performance Benchmarks

Typical build times on modern hardware:

- **Debug build:** 5-15 seconds
- **Release build:** 30-60 seconds
- **Parsing small program:** <1ms
- **Semantic analysis:** <1ms
- **Code generation:** <10ms

## Deployment

### Creating Distributable Binary

```bash
# Build release binary
cargo build --release

# Strip binary (reduce size)
strip target/release/oak

# Create distribution
mkdir oak-v0.1.0
cp target/release/oak oak-v0.1.0/
cp README.md oak-v0.1.0/
tar czf oak-v0.1.0.tar.gz oak-v0.1.0/
```

### Docker Support

Create `Dockerfile`:

```dockerfile
FROM rust:1.70
RUN apt-get update && apt-get install -y llvm-18
WORKDIR /app
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/oak"]
```

Build and run:

```bash
docker build -t oak:0.1.0 .
docker run oak:0.1.0 examples/hello_world.oak
```

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
# Test parsing
cargo test parser

# Test semantic analysis
cargo test semantic

# Test code generation
cargo test codegen
```

### Example Verification

```bash
# Run all examples
for file in examples/*.oak; do
    echo "Testing $file"
    oak "$file"
done
```

## Version Control

### Git Workflow

```bash
# Create feature branch
git checkout -b feature/new-feature

# Make changes and commit
git add .
git commit -m "Description of changes"

# Push and create pull request
git push origin feature/new-feature
```

## Documentation

### Generating Docs

```bash
# Generate Rust documentation
cargo doc --open

# Generate language documentation (in docs/)
ls docs/
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let tokens = tokenize("let x := 42;");
        assert!(!tokens.is_empty());
    }
}
```

## Getting Help

- **Documentation:** See `docs/MVP_REFERENCE.md`
- **Examples:** Check `examples/` directory
- **Repository:** https://github.com/admodev/oak
- **Issues:** Report bugs on GitHub

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make improvements
4. Submit a pull request
5. Wait for review and merge

---

**Last Updated:** 2025-11-13  
**Maintained By:** Adolfo Moyano  
**License:** MIT

# Oak Language MVP - Functional Reference

## Overview

This document describes the Minimum Viable Product (MVP) of the Oak programming language, a Rust-implemented compiler targeting LLVM with a focus on mathematical operations and systems programming.

## Architecture

### Compilation Pipeline

```
Source Code (.oak)
    ↓
Tokenizer (Lexer)
    ↓
Parser (builds AST)
    ↓
Semantic Analysis (type checking)
    ↓
Code Generation (LLVM IR)
    ↓
LLVM Optimizer
    ↓
Machine Code / Linkable Object
```

## 1. Lexer (Tokenizer)

**Location:** `src/tokenizer/mod.rs`

### Supported Tokens

- **Keywords:** `var`, `let`, `mut`, `fn`, `ret`, `if`, `else`, `while`, `for`, `break`, `continue`, `struct`, `enum`, `extern`, `as`
- **Operators:** `+`, `-`, `*`, `/`, `%`, `^`, `&`, `|`, `!`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `:=` (assignment), `->` (return type), `::`
- **Delimiters:** `(`, `)`, `{`, `}`, `[`, `]`, `,`, `.`, `:`, `;`
- **Comments:** Lines starting with `#` are ignored
- **Literals:** Numbers (float64), Strings (in quotes)
- **Identifiers:** Variable and function names

### Example

```oak
# This is a comment
let x: i32 := 42;
let message: Text := "Hello, World!";
```

## 2. Parser

**Location:** `src/parser/mod.rs`

### AST Nodes

#### Expression Types

- **Literals:** Numbers, Strings, Booleans
- **Identifiers:** Variable references
- **Binary Operations:** `a + b`, `a < b`, etc.
- **Unary Operations:** `-x`, `!flag`, `*ptr`, `&var`
- **Type Casting:** `value as TargetType`
- **Arrays:** `[1, 2, 3]`
- **Indexing:** `arr[0]`
- **Function Calls:** `func(arg1, arg2)`

#### Statement Types

- **Variable Declaration:** `let name: type := value;`
- **Expression Statements:** `expr;`
- **Blocks:** `{ ... }`
- **Control Flow:** `if`, `while`, `for`, `ret`, `break`, `continue`

#### Top-Level Declarations

- **Function Declaration:** `fn name(params) -> ReturnType { ... }`
- **Struct Declaration:** `struct Name { fields }`
- **Extern Function:** `extern fn cFunc(types) -> type;`

## 3. Type System

**Location:** `src/types.rs`

### Primitive Types

#### Unsigned Integers

- `u8`, `u16`, `u32`, `u64` - Standard unsigned integers

#### Signed Integers

- `i8`, `i16`, `i32`, `i64` - Standard signed integers

#### Positive Integers (Oak-specific)

- `posint8`, `posint16`, `posint32`, `posint64` - For values >= 0
- Runtime checking ensures values stay non-negative

#### Floating Point

- `f32`, `f64` - IEEE 754 floating point numbers

#### Other Types

- `bool` - Boolean type
- `str` - Fixed-length strings
- `Text` - Dynamic large strings
- `void` - No value

### Composite Types

#### Arrays

```
[ElementType; Size]  # Fixed-size array
[ElementType]        # Dynamic array (pointer)
```

#### Pointers

```
*Type  # Pointer to Type
```

#### References

```
&Type  # Reference to Type
```

#### Functions

```
fn(param_types) -> return_type
```

### Type Operations

#### Type Checking

- Automatic type inference when possible
- Runtime type assertions via `as` keyword
- Compile-time type checking

#### Type Compatibility

| From       | To         | Allowed |
| ---------- | ---------- | ------- |
| Numeric    | Numeric    | ✓       |
| Pointer    | void\*     | ✓       |
| Same types | Same types | ✓       |

## 4. Semantic Analysis

**Location:** `src/semantic.rs`

### Responsibilities

1. **Variable Resolution** - Ensure all variables are declared
2. **Type Checking** - Verify operations are type-safe
3. **Function Validation** - Check function signatures match calls
4. **Scope Management** - Track variable scope and lifetime
5. **Array Bounds** - Validate array indexing at compile-time when possible

### Type Inference

Oak automatically infers types when not explicitly specified:

```oak
let x := 42;        # x: f64 (inferred from literal)
let arr := [1, 2];  # arr: [f64; 2] (inferred from elements)
```

## 5. Code Generation

**Location:** `src/codegen.rs`

### LLVM IR Generation

The code generator produces LLVM Intermediate Representation (IR) that can be:

1. **Optimized** by LLVM's optimizer
2. **Compiled** to native machine code
3. **Linked** with external C libraries

### Example Translation

**Oak Code:**

```oak
fn add(a: i32, b: i32) -> i32 {
    ret a + b;
}
```

**LLVM IR:**

```llvm
define i32 @add(i32 %a, i32 %b) {
  %tmp.0 = add i32 %a, %b
  ret i32 %tmp.0
}
```

### Supported LLVM Operations

- Arithmetic: `add`, `sub`, `mul`, `sdiv`, `udiv`, `srem`
- Floating-point: `fadd`, `fsub`, `fmul`, `fdiv`
- Comparison: `icmp`, `fcmp`
- Memory: `alloca`, `load`, `store`, `getelementptr`
- Control Flow: `br`, `ret`

## 6. Error Module

**Location:** `src/errors.rs`

### Error Types

```rust
pub enum OakError {
    LexerError { message, location },
    ParseError { message, location },
    TypeError { message, expected, found, location },
    SemanticError { message, location },
    CodegenError { message, location },
    RuntimeError { message, location },
    IOError { message },
}
```

### Error Reporting

Errors include source location (`file:line:column`) for easy debugging.

## 7. REPL (Read-Eval-Print Loop)

**Location:** `src/repl/mod.rs`

Interactive mode for quick testing:

```bash
oak -r
oak> let x: i32 := 42;
oak> x + 8
oak> exit
```

## 8. FFI (Foreign Function Interface)

### Extern C Functions

Call C functions directly:

```oak
extern fn printf(i8*, ...) -> i32;
extern fn malloc(u64) -> *void;
extern fn free(*void) -> void;

fn main() -> void {
    let buf: *void := malloc(1024);
    free(buf);
}
```

### Pointer Operations

```oak
let x: i32 := 42;
let ptr: *i32 := &x;      # Take address
let val: i32 := *ptr;     # Dereference
```

## 9. Arrays and Bounds Checking

### Array Declaration

```oak
# Fixed-size array
let arr: [i32; 10] := [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

# Accessing elements
let first: i32 := arr[0];
let last: i32 := arr[9];
```

### Bounds Checking

The semantic analyzer validates array accesses at compile time when possible:

```oak
let arr: [i32; 5] := [1, 2, 3, 4, 5];
let x: i32 := arr[10];  # Compile-time error: index out of bounds
```

## 10. Type Assertions and Casting

### Type Assertions

```oak
let x: i32 := 42;
let y: i64 := x as i64;        # i32 to i64
let z: f64 := (x as i32) as f64;  # Numeric to float
```

### Supported Casts

- Numeric ↔ Numeric (any numeric types)
- Pointer → Pointer (via void\*)
- Numeric → Pointer (with explicit cast)

## 11. Building and Deployment

### Compilation Targets

Oak compiles to:

1. **LLVM IR** - Intermediate representation
2. **Object Files** (.o) - Linkable with C/C++ libraries
3. **Executable Binaries** - Cross-platform executables

### Cross-Platform Support

- Linux (x86_64)
- macOS (x86_64, ARM64)
- Windows (x86_64)

### Linking with External Code

```bash
# Generate LLVM IR
oak -c program.oak -o program.ll

# Compile to object file
llc -filetype=obj program.ll -o program.o

# Link with C libraries
gcc program.o -o program -lm -lc
```

## 12. Examples

### Example 1: Simple Function

```oak
fn square(x: f64) -> f64 {
    ret x * x;
}

fn main() -> void {
    let result: f64 := square(5.0);
}
```

### Example 2: Array Processing

```oak
fn sum_array(arr: [i32; 10]) -> i32 {
    let sum: i32 := 0;
    let i: i32 := 0;

    while i < 10 {
        sum := sum + arr[i];
        i := i + 1;
    }

    ret sum;
}
```

### Example 3: Extern C Integration

```oak
extern fn strlen(i8*) -> u64;
extern fn printf(i8*, ...) -> i32;

fn print_string_info(s: i8*) -> void {
    let len: u64 := strlen(s);
    printf("%s has length %lu\n", s, len);
}
```

## Features Implemented

✅ **Completed MVP Features:**

- [x] Lexer with comment support (#)
- [x] Parser with full AST
- [x] Type system with primitive types
- [x] Type assertions and casting
- [x] Semantic analysis
- [x] LLVM code generation
- [x] Error module with source locations
- [x] Array support with indexing
- [x] Pointers and references
- [x] Type inference
- [x] Extern C function declarations
- [x] REPL interactive mode
- [x] Cross-platform compilation support

## Building the Project

### Prerequisites

- Rust 1.70+
- LLVM 18
- Cargo

### Build Commands

```bash
# Check code compiles
cargo check

# Build debug binary
cargo build

# Build release binary
cargo build --release

# Run tests
cargo test

# Build and run with script
oak program.oak

# Start REPL
oak -r
```

## Next Steps

For full production use, consider implementing:

- Ownership and borrowing system (like Rust)
- Generic types and trait system
- Standard library (std)
- Module system and imports
- Better error recovery in parser
- Optimization passes
- Debugging information (DWARF)
- Profile-guided optimization

---

**Version:** 0.1.0 (MVP)  
**Created:** 2025-11-13  
**License:** MIT

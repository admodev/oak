# Oak Language Reference

## Table of Contents

1. [Lexical Elements](#lexical-elements)
2. [Data Types](#data-types)
3. [Expressions](#expressions)
4. [Statements](#statements)
5. [Functions](#functions)
6. [Structs](#structs)
7. [Arrays and Pointers](#arrays-and-pointers)
8. [Type System](#type-system)
9. [Memory Management](#memory-management)
10. [FFI (C Interop)](#ffi-c-interop)

## Lexical Elements

### Comments

Single-line comments start with `#`:

```oak
# This is a comment
let x: i32 := 42;  # Initialize x with 42
```

### Identifiers

- Start with letter or underscore
- Contain alphanumeric characters and underscores
- Case-sensitive

```oak
let myVariable: i32 := 10;
let _private: i32 := 20;
let MY_CONSTANT: i32 := 30;
```

### Keywords

```
as       break    continue  else     enum     extern
false    fn       for       if       let      mut
ret      struct   true      var      while
```

### Operators

#### Arithmetic

```
+    # Addition
-    # Subtraction
*    # Multiplication
/    # Division
%    # Modulo (remainder)
^    # Power/Exponent
```

#### Comparison

```
==   # Equal
!=   # Not equal
<    # Less than
>    # Greater than
<=   # Less than or equal
>=   # Greater than or equal
```

#### Logical

```
&&   # Logical AND
||   # Logical OR
!    # Logical NOT
```

#### Assignment

```
:=   # Assignment
```

#### Memory

```
&    # Address of (reference)
*    # Dereference / Pointer
```

#### Type

```
as   # Type cast
->   # Function return type
::   # Scope resolution
```

## Data Types

### Primitive Integer Types

#### Unsigned Integers

```oak
let a: u8 := 255;      # 0 to 255
let b: u16 := 65535;   # 0 to 65535
let c: u32 := 4_294_967_295;
let d: u64 := 18_446_744_073_709_551_615;
```

#### Signed Integers

```oak
let a: i8 := -128;     # -128 to 127
let b: i16 := -32768;  # -32768 to 32767
let c: i32 := 42;
let d: i64 := -9_223_372_036_854_775_808;
```

#### Positive Integers (Oak-specific)

```oak
let x: posint8 := 100;    # 0 to 127 (with compile-time checks)
let y: posint16 := 1000;
let z: posint32 := 100_000;
let w: posint64 := 1_000_000;
```

### Floating-Point Types

```oak
let f32_val: f32 := 3.14;
let f64_val: f64 := 2.71828;
let scientific: f64 := 1.23e-4;
```

### Boolean Type

```oak
let t: bool := true;
let f: bool := false;

# Boolean expressions
let result: bool := 5 > 3;
```

### String Types

```oak
# Fixed-length string
let small: str := "hello";

# Large dynamic string
let large: Text := "This is a longer message";
```

### Void Type

```oak
fn returns_nothing() -> void {
    # No return value
}
```

## Expressions

### Literals

```oak
# Numbers
42              # Numeric literal (f64)
3.14            # Float literal
0xFF            # Hexadecimal
1_000_000       # With separators

# Strings
"hello"         # String literal
"line1\nline2"  # With escape sequences

# Booleans
true
false

# Arrays
[1, 2, 3]       # Array literal
[]              # Empty array
```

### Binary Operations

```oak
# Arithmetic
let sum: i32 := 10 + 5;
let diff: i32 := 10 - 5;
let prod: i32 := 10 * 5;
let quot: i32 := 10 / 5;
let rem: i32 := 10 % 3;
let power: f64 := 2.0 ^ 10.0;

# Comparison
let eq: bool := 5 == 5;
let ne: bool := 5 != 3;
let lt: bool := 3 < 5;
let gt: bool := 5 > 3;
let le: bool := 5 <= 5;
let ge: bool := 5 >= 3;

# Logical
let and_result: bool := true && false;
let or_result: bool := true || false;
```

### Unary Operations

```oak
# Negation
let neg: i32 := -42;
let pos: i32 := +42;

# Logical NOT
let not_true: bool := !true;

# References
let x: i32 := 42;
let ref: *i32 := &x;

# Dereference
let value: i32 := *ref;
```

### Type Casting

```oak
let x: i32 := 42;

# Cast to different type
let as_float: f64 := x as f64;
let as_i64: i64 := x as i64;

# Pointer casts
let ptr: *void := &x as *void;
let back: *i32 := ptr as *i32;
```

### Function Calls

```oak
fn add(a: i32, b: i32) -> i32 {
    ret a + b;
}

# Call with arguments
let result: i32 := add(3, 4);

# Call with type arguments (if supported)
# let result: i32 := add::<i32>(3, 4);
```

### Array Indexing

```oak
let arr: [i32; 5] := [10, 20, 30, 40, 50];

# Get element
let first: i32 := arr[0];
let last: i32 := arr[4];

# Dynamic index
let idx: i32 := 2;
let elem: i32 := arr[idx];
```

## Statements

### Variable Declaration

```oak
# With explicit type
let x: i32 := 42;

# With type inference
let y := 3.14;

# Mutable binding
let mut z: i32 := 10;

# Without initialization
let uninitialized: i32;

# Multiple declarations
let a: i32 := 1;
let b: i32 := 2;
```

### Expression Statements

```oak
42;           # Expression as statement
add(1, 2);    # Function call statement
```

### Block Statement

```oak
{
    let x: i32 := 42;
    let y: i32 := x + 8;
    y
}
```

### If-Else Statement

```oak
if x > 0 {
    # Positive branch
}

if x > 0 {
    # Positive
} else {
    # Non-positive
}

if x > 0 {
    # Positive
} else if x < 0 {
    # Negative
} else {
    # Zero
}
```

### While Loop

```oak
let i: i32 := 0;
while i < 10 {
    i := i + 1;
}
```

### For Loop

```oak
for i in array {
    # i takes each element
}
```

### Return Statement

```oak
fn get_answer() -> i32 {
    ret 42;
}

fn early_exit(x: i32) -> void {
    if x < 0 {
        ret;  # Exit early
    }
    # More code
}
```

### Break Statement

```oak
while true {
    break;  # Exit loop
}
```

### Continue Statement

```oak
for i in [1, 2, 3] {
    if i == 2 {
        continue;  # Skip to next iteration
    }
}
```

## Functions

### Function Declaration

```oak
# Minimal function
fn hello() -> void {
    # No parameters, no return
}

# With parameters
fn add(a: i32, b: i32) -> i32 {
    ret a + b;
}

# Multiple parameters
fn complex(x: f64, y: f64, z: f64) -> f64 {
    ret (x * y) + z;
}
```

### Function Parameters

```oak
# Basic parameters
fn sum(a: i32, b: i32) -> i32 {
    ret a + b;
}

# Parameters of different types
fn mixed(count: i32, message: Text) -> void {
    # Use both
}
```

### Return Types

```oak
# Explicit return
fn get_value() -> i32 {
    ret 42;
}

# Void return
fn do_nothing() -> void {
    # No return needed
}

# Complex type
fn get_array() -> [i32; 5] {
    ret [1, 2, 3, 4, 5];
}
```

## Structs

### Struct Definition

```oak
struct Person {
    name: Text,
    age: posint32,
    height: f64,
}
```

### Creating Instances

```oak
let person: Person := {
    name: "Alice",
    age: 30,
    height: 5.7,
};
```

### Accessing Fields

```oak
let name: Text := person.name;
let age: posint32 := person.age;
```

## Arrays and Pointers

### Array Declaration

```oak
# Fixed-size array
let arr: [i32; 5] := [1, 2, 3, 4, 5];

# Type inference
let numbers := [10, 20, 30];  # [i32; 3]

# Empty array
let empty: [i32; 0] := [];
```

### Array Operations

```oak
let arr: [i32; 5] := [1, 2, 3, 4, 5];

# Access element
let first: i32 := arr[0];

# Array length (compile-time constant)
let len: i32 := 5;  # Known from declaration
```

### Pointers

```oak
let x: i32 := 42;

# Create pointer
let ptr: *i32 := &x;

# Dereference
let val: i32 := *ptr;

# Pointer arithmetic
let ptr2: *i32 := ptr;  # Copy pointer
```

### Pointer to Arrays

```oak
let arr: [i32; 5] := [1, 2, 3, 4, 5];

# Get pointer to first element
let ptr: *i32 := &arr[0];

# Access through pointer
let first: i32 := *ptr;
let second: i32 := *(ptr + 1);  # Pointer arithmetic
```

## Type System

### Type Compatibility

```oak
# Numeric types are compatible
let x: i32 := 42;
let y: f64 := x as f64;

# Pointers to void*
let ptr: *i32 := &x;
let generic: *void := ptr as *void;
```

### Type Inference

```oak
# Inferred from assignment
let x := 42;        # x: f64

# Inferred from function call
let result := add(1, 2);  # result: i32

# Inferred from array elements
let arr := [1, 2, 3];  # arr: [f64; 3]
```

## Memory Management

### Stack Allocation

```oak
# Variables on stack
let x: i32 := 42;
let y: Text := "hello";
```

### Heap Allocation (via FFI)

```oak
extern fn malloc(u64) -> *void;
extern fn free(*void) -> void;

fn allocate() -> void {
    let ptr: *void := malloc(1024);
    free(ptr);
}
```

### Lifetimes

Currently, Oak doesn't have explicit lifetime management. All values are currently stack-allocated or manually managed through pointers.

## FFI (C Interop)

### Declaring External Functions

```oak
# External C function
extern fn strlen(i8*) -> u64;
extern fn printf(i8*, ...) -> i32;
extern fn malloc(u64) -> *void;
```

### Calling External Functions

```oak
extern fn puts(i8*) -> i32;

fn main() -> void {
    let result: i32 := puts("Hello from C!");
}
```

### Type Mapping

| Oak Type | C Type       |
| -------- | ------------ |
| i8       | int8_t       |
| i32      | int32_t      |
| i64      | int64_t      |
| u8       | uint8_t      |
| f64      | double       |
| bool     | \_Bool       |
| Text     | const char\* |
| \*T      | T\*          |

## Language Grammar (Simplified EBNF)

```ebnf
program         ::= (top_level)*
top_level       ::= var_decl | fn_decl | struct_decl | extern_fn | statement

var_decl        ::= ('let' | 'var' | 'mut') identifier (':' type)? (':=' expr)? ';'
fn_decl         ::= 'fn' identifier '(' params? ')' ('->' type)? '{' statements '}'
struct_decl     ::= 'struct' identifier '{' fields '}'
extern_fn       ::= 'extern' 'fn' identifier '(' types? ')' ('->' type)? ';'

statement       ::= expr_stmt | block_stmt | if_stmt | while_stmt | for_stmt | ret_stmt
expr_stmt       ::= expr ';'
block_stmt      ::= '{' statement* '}'
if_stmt         ::= 'if' expr statement ('else' statement)?
while_stmt      ::= 'while' expr statement
for_stmt        ::= 'for' identifier 'in' expr statement
ret_stmt        ::= 'ret' expr? ';'

expr            ::= or_expr
or_expr         ::= and_expr ('||' and_expr)*
and_expr        ::= eq_expr ('&&' eq_expr)*
eq_expr         ::= cmp_expr (('==' | '!=') cmp_expr)*
cmp_expr        ::= add_expr (('<' | '>' | '<=' | '>=') add_expr)*
add_expr        ::= mul_expr (('+' | '-') mul_expr)*
mul_expr        ::= unary_expr (('*' | '/' | '%') unary_expr)*
unary_expr      ::= (('!' | '-' | '+' | '*' | '&') unary_expr) | postfix_expr
postfix_expr    ::= primary_expr ('[' expr ']')* ('as' type)*
primary_expr    ::= number | string | bool | identifier | '(' expr ')'
                 | '[' expr_list ']'

type            ::= '*' type | '&' type
                 | '[' type (';' number)? ']'
                 | primitive_type | struct_name

primitive_type  ::= 'u8' | 'u16' | 'u32' | 'u64'
                 | 'i8' | 'i16' | 'i32' | 'i64'
                 | 'posint8' | 'posint16' | 'posint32' | 'posint64'
                 | 'f32' | 'f64'
                 | 'bool' | 'str' | 'Text' | 'void'
```

---

**Last Updated:** 2025-11-13  
**Language Version:** 0.1.0 (MVP)  
**License:** MIT

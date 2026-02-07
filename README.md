# RSSLI - Rust Simple Lisp Interpreter

A toy Lisp interpreter written in Rust as a learning project. The goal is
to explore how parsers and interpreters work by building one from scratch:
tokenization, parsing S-expressions into an AST, and tree-walking
evaluation with lexical scoping.

This is **not** intended for production use. It is a personal playground for
experimenting with language implementation concepts.

## Building

```bash
cargo build
```

## Running

RSSLI has three modes of operation, specified by a subcommand.

### File Mode

Evaluates a Lisp script from a file.

```bash
cargo run -- file examples/simple-print.lsp
```

Output:

```
hello world!
10
```

### Eval Mode

Evaluates a single Lisp expression from a string.

```bash
cargo run -- eval "(+ 10 20)"
```

### Interactive Mode (REPL)

Starts an interactive Read-Eval-Print Loop.

```bash
cargo run -- interactive
```

You can type expressions at the prompt:

```
rssli> (+ 1 2)
=> Int(3)
rssli> (def x 10)
=> Nil
rssli> x
=> Int(10)
```


### Logging

Enable debug logging with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run -- examples/simple-print.lsp
RUST_LOG=trace cargo run -- examples/simple-funcs.lsp
```

## Language Guide

### Data Types

| Type | Examples | Description |
|---|---|---|
| Integer | `42`, `-3`, `0x1A`, `0b1010` | 64-bit signed integers (decimal, hex, binary) |
| Float | `3.14`, `0.5` | 64-bit floating point |
| String | `"hello world"` | UTF-8 strings, supports `\"` and `\\` escapes |
| Boolean | `true`, `false` | |
| Nil | `nil` | Null value |
| List | `(1 2 3)` | Ordered collection |
| Function | `(fn (x) (* x 2))` | First-class, can be stored in variables |

### Variables

```lisp
(def x 42)
(def name "world")
(def result (+ x 8))
(undef x)
```

### Arithmetic

Operators auto-promote types: Int + Float = Float, anything + String = String concatenation.

```lisp
(+ 1 2)          ; => 3
(+ 1.5 2)        ; => 3.5
(+ "hello" " " "world")  ; => "hello world"
(- 10 3)         ; => 7
(* 4 5)          ; => 20
(/ 15 3)         ; => 5
```

### Comparison

```lisp
(== 1 1)    ; => true
(!= 1 2)    ; => true
(< 1 2)     ; => true
(> 3 1)     ; => true
```

### Logical Operators

Short-circuit evaluation:

```lisp
(&& true true)    ; => true
(|| false true)   ; => true
```

### Conditionals

```lisp
(if (< x 10)
    (print "small")
    (print "large"))
```

The else branch is optional:

```lisp
(if (== x 0) (print "zero"))
```

### Functions

Named functions:

```lisp
(fn add (a b) (+ a b))
(add 3 4)    ; => 7
```

Anonymous functions stored in variables:

```lisp
(def double (fn (x) (* x 2)))
(double 5)   ; => 10
```

Functions can reference other functions by name (aliases):

```lisp
(def add2 +)
(add2 1 2)   ; => 3
```

Recursion works:

```lisp
(fn factorial (n)
    (if (< n 1)
        1
        (* n (factorial (- n 1)))))

(factorial 5)    ; => 120
```

### Imports

Import code from other files using the `import` function. Imported files are evaluated in the current scope, so all functions and variables defined in the imported file become available.

```lisp
(import "math-utils.lsp")
(square 5)       ; => 25 (if square is defined in math-utils.lsp)
```

**Path resolution:**
- Relative paths are resolved relative to the directory of the file that contains the import statement
- If no file context is available (e.g., in interactive mode), paths are resolved relative to the current working directory
- If the file extension is omitted, `.lsp` is automatically appended
- Absolute paths are also supported

**Circular imports:**
- Circular imports are automatically detected and prevented
- An error is raised if a file tries to import itself (directly or indirectly)

**Example:**

```lisp
; math-utils.lsp
(fn square (x) (* x x))
(def pi 3.14159)

; main.lsp
(import "math-utils.lsp")
(io.print "Square of 5:" (square 5))
(io.print "Pi:" pi)
```

### Loops

**For loop** over a sequence:

```lisp
(for i (list.seq 0 10) (print i))
```

**While loop:**

```lisp
(def counter 0)
(while (< counter 5)
    (def counter (+ counter 1)))
```

### Type Casting

```lisp
(cast.int "42")       ; => 42
(cast.float 10)       ; => 10.0
(cast.string 42)      ; => "42"
(cast.bool 1)         ; => true
(cast.list 5)         ; => (5)
```

### List Operations

```lisp
(list.seq 0 5)        ; => (0, 1, 2, 3, 4)
(list.seq 0 10 2)     ; => (0, 2, 4, 6, 8)
(head (1 2 3))        ; first element
(last (1 2 3))        ; last element
```

### I/O

```lisp
(print "hello" "world")          ; prints: hello world
(io.print "same as print")
(def name (io.readline "Name:")) ; reads line from stdin
```

### Assertions

```lisp
(assert (== 1 1))
(assert.eq 3 (+ 1 2))
```

### Debug / Introspection

```lisp
(internal.func.list)     ; lists all registered functions
(internal.printenv)       ; dumps all functions and variables
```

## Examples

The `examples/` directory contains sample programs:

| File | Description |
|---|---|
| `simple-aritmetic.lsp` | Basic arithmetic operations |
| `simple-print.lsp` | Variables and printing |
| `simple-funcs.lsp` | Function definition, recursion (factorial) |
| `simple-conditionals.lsp` | User input and if/else |
| `simple-cycles.lsp` | For loop with sequence |
| `funcs-in-vars.lsp` | Anonymous functions, function aliases, assertions |
| `internal-sample.lsp` | Listing all built-in functions |
| `simple-import.lsp` | File imports and using imported functions |
| `math-utils.lsp` | Math utility library (used by simple-import.lsp) |
| `guess-number.lsp` | A simple guess number game |

## Testing

```bash
cargo test
```

## License

This is a personal learning project.

# RSSLI Architecture

How the interpreter works internally.

## Pipeline Overview

RSSLI follows a classic interpreter pipeline. Source code flows through
three stages before producing a result:

```
Source Code (string)
       |
       v
  [Tokenizer]  tokenizer.rs
       |        Splits raw text into a flat list of string tokens
       v
   [Parser]     parser.rs
       |        Converts tokens into a tree of Value nodes (AST)
       v
  [Evaluator]   env.rs + runtime.rs
       |        Walks the AST, resolves symbols, calls functions
       v
   Result (Value)
```

The entry point is the `cli` module (`src/cli/mod.rs`), which parses command-line
arguments and then calls `Runtime::eval_string()` in `runtime.rs:30-46`.
This function chains tokenize -> parse -> eval and returns the final `Value`.

## Stage 1: Tokenizer

**File:** `src/tokenizer.rs`

The tokenizer (`tokenize()`) takes a raw source string and produces a
`Vec<String>` of tokens. It is a single-pass character-by-character
scanner.

### Rules

- **Parentheses** `(` and `)` are always separate tokens.
- **Whitespace** separates tokens but is not emitted.
- **Strings** start with `"` and end with `"`. Content between quotes
  (including spaces) is collected into a single token prefixed with `"`.
  Backslash escapes `\"` and `\\` are handled.
- **Everything else** accumulates into a buffer until a delimiter
  (whitespace, parens, quote) is encountered.

### Example

Input:
```lisp
(+ "hello" 42)
```

Tokens:
```
["(", "+", "\"hello", "42", ")"]
```

Note: string tokens carry a leading `"` prefix, which the parser uses to
identify them.

## Stage 2: Parser

**File:** `src/parser.rs`

The parser (`parse_tokens()`) takes the flat token list and builds a tree
of `Value` nodes. It calls `parse_tokens_from()` recursively to handle
nested parentheses.

### Value Enum

```rust
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    List(Vec<Value>),
    Bool(bool),
    Func(FuncKind),
    Nil,
}
```

### Parsing Rules (in order of evaluation)

| Token | Parsed As |
|---|---|
| `nil` | `Value::Nil` |
| `true` / `false` | `Value::Bool` |
| Starts with `"` | `Value::String` (prefix stripped) |
| `(` | Begin `Value::List` (recursive descent) |
| `)` | End current list, return to parent |
| Parses as `i64` | `Value::Int` |
| Parses as `f64` | `Value::Float` |
| Starts with `0x` | `Value::Int` (hexadecimal) |
| Starts with `0b` | `Value::Int` (binary) |
| Anything else | `Value::Symbol` |

### Example

Tokens `["(", "+", "5", "3", ")"]` become:

```
Value::List([
    Value::Symbol("+"),
    Value::Int(5),
    Value::Int(3),
])
```

### Type Conversions

`Value` implements `From` traits for bidirectional conversion between
Rust types and Value variants. These are used throughout the evaluator:

- `From<&Value> for bool` - truthiness rules
- `From<&Value> for i64` - integer coercion
- `From<&Value> for f64` - float coercion
- `From<&Value> for String` - string representation
- `From<&Value> for Vec<Value>` - list coercion

## Stage 3: Evaluator

**Files:** `src/env.rs`, `src/runtime.rs`

The evaluator is a tree-walking interpreter. The core method is
`Environment::eval()` at `env.rs:56-67`.

### Evaluation Rules

```rust
fn eval(&mut self, value: &Value) -> Value {
    match value {
        Value::List(l)    => self.eval_list(l),
        Value::Symbol(n)  => self.get_var_or_func(n),
        Value::Func(fx)   => self.eval_any_func(...),
        _                 => value.clone(),  // primitives return themselves
    }
}
```

1. **Primitives** (Int, Float, String, Bool, Nil) evaluate to themselves.
2. **Symbols** are looked up first in variables, then in functions. If
   found as a function, it's wrapped in `Value::Func`.
3. **Lists** are evaluated depending on their first element (see below).
4. **Functions** are called with no arguments.

### List Evaluation

`eval_list()` at `env.rs:110-143` decides how to handle a list:

- If the first element is a **Symbol**, treat it as a function call:
  look up the function by name, pass remaining elements as arguments.
- If the first element is a **Func**, call it directly with remaining
  elements as arguments.
- If the first element is a **List** (IIFE / do-block disambiguation):
  evaluate the first element. If the result is a function **and** not
  all elements in the outer list are lists, treat it as a function call
  (IIFE pattern, e.g. `((make-adder 5) 10)`). Otherwise, evaluate the
  remaining elements sequentially and return the last result (do-block
  pattern, e.g. `((expr1) (expr2) (expr3))`).
- Otherwise, evaluate each element and return as a new list.

### Symbol Resolution

`get_var_or_func()` at `env.rs:69-78`:

1. Check the variable store (`vars`) first.
2. If not found, check the function store (`funcs`).
3. If not found in either, return an error.

This means variables shadow functions with the same name.

## Environment and Scoping

**Files:** `src/env.rs`, `src/utils.rs`

### Environment Structure

```rust
pub struct Environment {
    pub funcs: HierCellMapWrap<String, FuncDef>,
    pub vars: HierCellMapWrap<String, Value>,
}
```

The environment holds two separate stores:
- **funcs** for function definitions
- **vars** for variable bindings

### Hierarchical Scope (HierCellMapWrap)

**File:** `src/utils.rs`

The scope system is built on `HierCellMapWrap<K, V>`, a wrapper around
`Rc<RefCell<HierCellMap<K, V>>>`. This is the key data structure enabling
lexical scoping.

```
Root Environment
  funcs: { +, -, *, /, fn, def, if, print, ... }
  vars:  {}
       |
       v  (make_child)
Child Environment
  funcs: { user-defined-fn, ... }
  vars:  { x: 5, y: 10 }
       |
       v  (make_child)
Grandchild Environment
  funcs: {}
  vars:  { local_var: 42 }
```

### Lookup semantics

- **`get(key)`** searches the current scope's HashMap first, then
  recursively checks the parent chain. Returns the first match.
- **`set(key, value)`** always writes to the current scope only.
- **`set_or_update(key, value)`** walks the parent chain to find an
  existing binding and updates it in place. If no existing binding is
  found anywhere in the chain, creates a new one in the current scope.
  This is used by `def` and enables closures to mutate captured
  variables (e.g. a counter pattern).
- **`update(key, value)`** searches current scope first, then parents,
  updating in-place where found.
- **`unset(key)`** removes from the current scope only.

Child environments hold an `Rc` reference to their parent, so parent
data is shared (not copied). Changes to a parent are visible to all
children.

### Child Environment Creation

`Environment::make_child()` at `env.rs:48-55` creates a new environment
whose `funcs` and `vars` are children of the current ones. This is called
when entering a native function body (for functions with
`same_env = false`) and when executing closures (child of the captured
environment).

## Functions

**File:** `src/func.rs`

### FuncDef

```rust
pub struct FuncDef {
    pub metadata: FuncMetadata,
    pub kind: FuncKind,
}
```

### FuncKind

```rust
pub enum FuncKind {
    Native(FuncType),                   // fn(&[Value], &mut Environment) -> Value
    Closure(FuncValue, Environment),    // user-defined: args + body + captured env
}
```

- **Native** functions are Rust function pointers. They receive raw
  (unevaluated) arguments and a mutable environment reference. The
  function decides whether and when to evaluate its arguments.
- **Closure** functions are user-created via `(fn name (args) body)`.
  They store parameter names, a body expression, and a captured
  environment (the lexical scope at the point of definition). The
  captured environment is an `Rc`-based clone, so it shares state
  with the defining scope.

### FuncValue (Closure Body)

```rust
pub struct FuncValue {
    pub args: Vec<String>,
    pub body: Box<Value>,
}
```

### The `same_env` Flag

`FuncMetadata.same_env` controls scoping during **native** function calls
(`env.rs:160-167`):

- `same_env = true`: the native function executes in the **caller's**
  environment. Used by special forms like `fn`, `def`, `if`, `while`,
  `for` that need to read or modify the calling scope.
- `same_env = false` (default for most native functions): a **child
  environment** is created. Arguments and local variables don't leak
  into the caller's scope.

The `same_env` flag is **not used** by closures. Closures always execute
in a child of their captured environment, regardless of this flag.

### Function Call Flow

`eval_any_func()` at `env.rs:150-194`:

1. For **native** functions: create a child environment; if `same_env`
   is true, use the caller's environment instead. Call the Rust function
   pointer with raw args and the chosen environment.
2. For **closure** functions: create a child of the **captured**
   environment (not the caller's). Evaluate each argument in the
   **caller's** environment, bind the results to parameter names in the
   closure child environment, then evaluate the body in that closure
   environment. This ensures lexical scoping — the closure body sees
   its definition-time bindings, not the caller's.
3. If the result is a single-element list, unwrap it (convenience
   behavior).

## Standard Library (corelib)

**File:** `src/corelib/`

All built-in functions are registered in `corelib::register()` which
calls each module's `register()` function. Each module adds native
functions to the environment via `env.add_native(name, fn_ptr, same_env)`.

### Core (`corelib/core.rs`)

| Function | `same_env` | Description |
|---|---|---|
| `fn` | true | Defines a named or anonymous function (closure) |
| `def` | true | Binds a variable (uses `set_or_update` to walk parent chain) |
| `undef` | true | Removes a variable from the current scope |
| `if` | true | Conditional: `(if cond then [else])` |
| `while` | true | Loop: `(while cond body)` |
| `for` | true | Iteration: `(for var sequence body)` |

All core forms use `same_env = true` and receive **unevaluated**
arguments. `fn`, `def`, and `undef` need to modify the caller's scope
(define functions/variables that persist after the call returns).
`if`, `while`, and `for` also operate in the caller's scope so that
variables defined or mutated inside branches and loop bodies are
visible after the form completes. They evaluate the condition and
branches/body manually, implementing lazy evaluation (only the taken
branch is evaluated).

### Operators (`corelib/ops.rs`)

| Function | Description |
|---|---|
| `+` | Addition (Int/Float) or string concatenation |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `==` | Equality (chained) |
| `!=` | Inequality (chained) |
| `<` | Less than (chained) |
| `>` | Greater than (chained) |
| `&&` | Logical AND (short-circuit) |
| `\|\|` | Logical OR (short-circuit) |

**Type promotion in arithmetic:** operators scan all arguments to
determine the result type. If any argument is a String, result is String
(concatenation). Otherwise, if any is Float, result is Float. Default is
Int.

### I/O (`corelib/io.rs`)

| Function | Description |
|---|---|
| `print` / `io.print` | Print arguments separated by spaces to stdout |
| `io.readline` | Read a line from stdin, optional prompt argument |

### Type Casting (`corelib/cast.rs`)

| Function | Description |
|---|---|
| `cast.string` | Convert to String |
| `cast.int` | Convert to Int |
| `cast.float` | Convert to Float |
| `cast.bool` | Convert to Bool |
| `cast.list` | Convert to List |

### List Operations (`corelib/list.rs`)

| Function | Description |
|---|---|
| `head` | First element of a list |
| `last` | Last element of a list |
| `list.seq` | Generate integer sequence: `(list.seq start end [step])` |

### Assertions (`corelib/assert.rs`)

| Function | Description |
|---|---|
| `assert` | Panics if condition is falsy |
| `assert.eq` | Panics if two values are not equal |

Both use `same_env = true` to evaluate arguments in the caller's scope.

### Introspection (`corelib/internal.rs`)

| Function | Description |
|---|---|
| `internal.func.list` | Print all registered function names |
| `internal.printenv` | Print all functions and variables with their values |
| `internal.func.nat.call` | Call a function by string name |

## Runtime

**File:** `src/runtime.rs`

`Runtime` is the public API. It wraps an `Environment` and provides:

- `Runtime::new()` - empty runtime with logger setup.
- `Runtime::new_default()` - runtime with all corelib functions
  registered.
- `Runtime::eval_string(prog)` - full pipeline: tokenize, parse,
  evaluate, return final result.

When multiple top-level expressions are parsed, they are evaluated
sequentially. The final result is the **last** expression's value
(matching typical Lisp semantics where a program's value is its last
expression). If the last result is itself a list, the last element of
that list is returned.

## File Map

```
src/
├── main.rs          Entry point: delegates to the `cli` module
├── cli/
│   ├── mod.rs       Handles command-line argument parsing and mode selection (eval, file, interactive)
│   └── interactive.rs Interactive REPL implementation
├── lib.rs           Module declarations, re-exports Runtime
├── tokenizer.rs     tokenize() - string to tokens
├── parser.rs        parse_tokens() - tokens to Value AST, Value type + conversions
├── runtime.rs       Runtime struct - public API, pipeline orchestration
├── env.rs           Environment - eval, scoping, function dispatch
├── func.rs          FuncDef, FuncKind, FuncMetadata, FuncType
├── utils.rs         HierCellMap, HierCellMapWrap - hierarchical scope data structure
└── corelib/
    ├── mod.rs       register() - registers all modules
    ├── core.rs      fn, def, undef, if, while, for
    ├── ops.rs       +, -, *, /, ==, !=, <, >, &&, ||
    ├── io.rs        print, io.print, io.readline
    ├── cast.rs      cast.string, cast.int, cast.float, cast.bool, cast.list
    ├── list.rs      head, last, list.seq
    ├── assert.rs    assert, assert.eq
    └── internal.rs  internal.func.list, internal.printenv, internal.func.nat.call
```

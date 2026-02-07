# Closures and Scoping

How RSSLI implements lexical scoping, closures, and variable resolution.

## Overview

RSSLI uses **lexical scoping** with closures. Every function created with
`fn` captures its defining environment at the time of creation. When the
function is later called — possibly from a completely different scope —
it executes in a child of that captured environment, not in the caller's
scope.

This means:

- A function can access variables that existed where it was defined.
- A function **cannot** access variables that exist only in its caller's
  scope.
- Multiple closures over the same environment share mutable state via
  `Rc<RefCell>`.

## Environment Chain

Environments form a parent-child chain. Each environment holds two
hierarchical maps (`HierCellMapWrap`): one for functions, one for
variables.

```
Root Environment
  funcs: { +, -, *, fn, def, if, print, ... }
  vars:  {}
       |
       v  make_child()
User Environment
  funcs: { make-adder, counter, ... }
  vars:  { x: 5, n: 10 }
       |
       v  make_child()
Function Body Environment
  funcs: {}
  vars:  { param: 42 }
```

**File:** `src/utils.rs`

`HierCellMapWrap<K, V>` wraps an `Rc<RefCell<HierCellMap<K, V>>>`.
This design has two properties:

1. **`make_child()`** creates a new scope whose parent is an `Rc` clone
   of the current scope. The child holds a reference to the parent, not
   a copy. Changes to the parent's data are visible to the child (and
   vice versa for mutations through the parent chain).

2. **`get(key)`** walks up the parent chain. It checks the current
   scope's `HashMap` first, then recursively checks ancestors. The first
   match wins.

## Closure Capture

**File:** `src/corelib/core.rs` — `bi_func_def()`

When `(fn name (params) body)` is evaluated:

1. The parameter names and body expression are stored in a `FuncValue`.
2. A `FuncKind::Closure(func_value, fenv.clone())` is created, where
   `fenv` is the environment in which `fn` is executing.
3. `fenv.clone()` is cheap — it clones `Rc` pointers, not the underlying
   data. The closure and its defining scope share the same `RefCell`
   backing store.

```rust
// from src/corelib/core.rs
let kind = FuncKind::Closure(func, fenv.clone());
```

For named functions, the `FuncDef` is also registered in `fenv.funcs`.
Because the closure's captured environment and `fenv` share the same
`Rc`, the function can see its own name during recursion — no special
self-reference mechanism is needed.

```lisp
; Recursion works naturally because `factorial` is visible in its own
; captured environment via Rc sharing.
(fn factorial (x)
    (if (< x 1)
        1
        (* x (factorial (- x 1)))))
(factorial 5)  ; => 120
```

## Closure Execution

**File:** `src/env.rs` — `eval_any_func()`

When a closure is called, execution proceeds in four steps:

```
Caller Environment ──────────────────────────┐
  (args evaluated here)                      │
                                             │
Captured Environment ──── make_child() ──> Closure Body Environment
  (from definition time)                     (params bound here,
                                              body evaluated here)
```

1. **Create child of captured env:** `captured_env.make_child()` — not
   a child of the caller.
2. **Evaluate arguments in caller:** each argument expression is
   evaluated in `self` (the caller's environment).
3. **Bind parameters:** evaluated values are bound in the closure body
   environment via `set()`.
4. **Evaluate body:** the body expression runs in the closure body
   environment.

```rust
// from src/env.rs
FuncKind::Closure(func_val, captured_env) => {
    let mut closure_env = captured_env.make_child();
    for (i, param) in func_val.args.iter().enumerate() {
        if i < args.len() {
            let value = self.eval(&args[i])?;
            closure_env.vars.set(param, &value);
        }
    }
    closure_env.eval(&func_val.body)?
}
```

This contrasts with **native** functions, which create a child of the
caller's environment (or use the caller directly when `same_env = true`).

## Variable Assignment and Mutation

**File:** `src/corelib/core.rs` — `bi_setvar()` (the `def` form)

`def` uses `set_or_update()` to assign variables:

```rust
// from src/corelib/core.rs
fenv.vars.set_or_update(&name, &value);
```

`set_or_update()` (in `src/utils.rs`) follows this algorithm:

1. If the key exists in the **current** scope → update it there.
2. Otherwise, walk the **parent chain**. If found in any ancestor →
   update it in that ancestor.
3. If not found anywhere → create a new binding in the current scope.

This enables closures to mutate captured variables:

```lisp
(def counter 0)
(fn inc () (def counter (+ counter 1)))
(inc)    ; counter is now 1
(inc)    ; counter is now 2
(inc)    ; counter is now 3
counter  ; => 3
```

When `inc` executes, `def counter ...` triggers `set_or_update`. It
walks from the closure body environment up to the captured environment,
finds the existing `counter` binding there, and updates it in place.
Because the captured environment is shared via `Rc<RefCell>`, the
mutation is visible to the outer scope.

In contrast, `vars.set()` (used internally for parameter binding) always
writes to the current scope without checking parents. This is why
parameters shadow outer variables without modifying them.

## IIFE and Do-Blocks

**File:** `src/env.rs` — `eval_list()`

When the first element of a list is itself a list, the evaluator must
decide between two interpretations:

- **IIFE** (Immediately Invoked Function Expression):
  `((make-adder 5) 10)` — evaluate the first list to get a function,
  then call it with the remaining elements as arguments.
- **Do-block**: `((expr1) (expr2) (expr3))` — evaluate each sub-list
  sequentially, return the last result.

The disambiguating heuristic:

1. Evaluate the first element.
2. If the result is a function **and** not all elements in the outer
   list are lists → treat as IIFE.
3. Otherwise → treat as do-block (evaluate remaining elements
   sequentially).

```lisp
; IIFE — (make-adder 5) returns a closure, 10 is not a list
((make-adder 5) 10)  ; => 15

; Do-block — all elements are lists
((def x 5) (def y 10) (+ x y))  ; => 15
```

## Known Limitations

### Rc Reference Cycles

Recursive closures create reference cycles: the closure holds an `Rc`
to the environment, and the environment's function map holds the closure.
These cycles are not collected by Rust's `Rc` reference counting. In
practice, closures are typically long-lived (program lifetime), so this
is acceptable for the interpreter's use case.

### IIFE Ambiguity

The `all_lists` heuristic for IIFE detection has an edge case: if a
closure is invoked immediately and all arguments happen to be list
expressions, the evaluator treats it as a do-block instead of a function
call.

```lisp
; Ambiguous: (fn (x) ...) returns a closure, but (list.seq 1 3) is a list
; expression — so all elements are lists and this is treated as a do-block.
((fn (x) (+ x 1)) (list.seq 1 3))
```

Workaround: bind the closure or argument to a variable first:

```lisp
(def add1 (fn (x) (+ x 1)))
(add1 (list.seq 1 3))
```

## Examples

### make-adder (returning a closure)

```lisp
(fn make-adder (n) (fn (x) (+ n x)))

(def add5 (make-adder 5))
(add5 10)   ; => 15
(add5 20)   ; => 25
```

`make-adder` returns an anonymous closure that captures `n`. Each call
to `make-adder` creates a fresh environment with its own `n` binding.

### Shared Mutable State (counter)

```lisp
(def count 0)
(fn increment () (def count (+ count 1)))
(fn get-count () count)

(increment)
(increment)
(get-count)  ; => 2
```

Both `increment` and `get-count` close over the same environment that
contains `count`. Mutations through `increment` are visible to
`get-count`.

### Recursive Factorial

```lisp
(fn factorial (x)
    (if (< x 1)
        1
        (* x (factorial (- x 1)))))
(factorial 5)  ; => 120
```

`factorial` is visible inside its own body because the function is
registered in the environment before (via `Rc` sharing — the captured
env and the env where the function is registered are the same `Rc`).

### Closure Isolation (cannot see caller variables)

```lisp
(fn func (x) (+ c x))
(fn func2 (c) (func 5))
(func2 10)  ; ERROR: Undeclared variable or function: c
```

`func` was defined in a scope where `c` does not exist. Even though
`func2` has `c` as a parameter and calls `func`, `func` executes in a
child of its **own** captured environment — not in `func2`'s scope.
This is the defining characteristic of lexical scoping.

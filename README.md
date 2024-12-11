# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust programming: creating multiple mutable borrows of the same variable.  Rust's ownership system prevents data races and ensures memory safety by restricting mutable access. This example showcases the error and its solution.

## Bug
The `bug.rs` file contains code that attempts to create two mutable references (`y` and `z`) to the same variable `x`. This leads to a compiler error.

## Solution
The `bugSolution.rs` file demonstrates a correct approach, either by using a single mutable reference or by cloning the data if mutability is required in separate scopes.
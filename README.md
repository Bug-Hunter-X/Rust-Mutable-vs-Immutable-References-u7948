# Rust Mutable vs Immutable References

This example demonstrates a common error in Rust related to mutable and immutable references.  The core issue is that after creating a mutable reference (`&mut`), trying to use an existing immutable reference (`&`) to modify the underlying data results in a compile-time error.  Rust's borrow checker prevents this to ensure memory safety.

The `bug.rs` file contains the code exhibiting the error.  `bugSolution.rs` shows how the problem can be avoided by either avoiding mutable references altogether or correctly managing their lifetimes.
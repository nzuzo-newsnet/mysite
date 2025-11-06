#####
date = "2024-01-29"
author = "Nzuzo Magagula"
summary = "Understanding Rust's ownership system - the feature that makes Rust unique and guarantees memory safety."
topics = ["Rust", "Ownership", "Memory Management"]
tags = ["rust", "ownership", "memory-safety", "intermediate"]
thumbnail = "https://www.rust-lang.org/static/images/rust-logo-blk.svg"
reading_time = "15 min"
category = "Tutorial"
#####

# Understanding Rust Ownership

Ownership is Rust's most unique feature and enables Rust to make memory safety guarantees without needing a garbage collector. Understanding ownership is crucial to mastering Rust.

## The Three Rules of Ownership

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

## Example: Ownership Transfer

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // This would error! s1 no longer valid
    println!("{}", s2); // This works fine
}
```

## Borrowing

Instead of transferring ownership, you can borrow a reference:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

## Mutable References

You can have mutable references, but with restrictions:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

## The Rules of References

1. At any given time, you can have either one mutable reference OR any number of immutable references
2. References must always be valid

## Why This Matters

This system eliminates:
- Data races at compile time
- Dangling pointers
- Double-free errors
- Use-after-free bugs

In the next article, we'll explore structs and enums!

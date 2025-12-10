#####
date = "2025-12-12"
author = "Nzuzo Magagula"
summary = "Exploring why one bug broke the internet, the role of centralization in modern tech, and how our consumption choices shape internet infrastructure"
thumbnail = "https://i.postimg.cc/3wMCP1N0/cracked-white-plaster-wall-texture-background.jpg"
category = "Opinion"
show_references = true

[[article_series]]
name = "Cloudflare Outage & Infrastructure Fragility"
prev = "Cloudflare Outage/01-centralization-and-infrastructure"
#####
# **Rust, Reality, and the Strange State of Language Discourse**

Programming languages rarely inspire neutrality. For better or worse, they become symbols—of ideals, of identity, of “the right way to build software.” Nowhere has this been more apparent in the last decade than in the discussion around Rust. The language’s meteoric rise, bold claims, and distinctive constraints have created an environment where people don’t just debate technical tradeoffs; they defend worldviews.

Before diving into the specifics of Rust’s strengths and limitations, I want to frame the discussion clearly:

1. **What Rust actually promises.**
   Rust does not promise perfection. It promises a *specific* kind of safety—primarily memory safety—delivered through compile-time enforcement rather than runtime checks or manual discipline.

2. **How Rust is perceived and marketed.**
   Over time, this technical promise has morphed into a cultural narrative: that Rust eliminates whole categories of bugs everywhere, for everyone, effortlessly.

3. **Where Rust’s real drawbacks are—and why they differ from the common complaints.**
   Rust **does** have costs: complexity, friction, mental load. But those costs are rarely the ones people complain about. Instead, discourse gets caught up in strawmen.

4. **The inherent cost of changing languages.**
   Switching languages is expensive: tooling, training, patterns, idioms, and mental models must all shift. Many anti-Rust hot takes arise from underestimating this cost.

5. **The nature and harm of tribalistic developer behavior.**
   Technical debates often become identity battles. Rust discourse has suffered heavily from this: nuance gets smothered, misinformation survives indefinitely, and criticism is treated as betrayal.

With that framing in place, let’s dig deeper.

---

# **The Strange Ride Rust Has Had**

Rust’s trajectory has been unusually volatile. When I first encountered Rust—before ever touching C++—I noticed two things very quickly:

1. **Absolutely anything can become controversial**
2. **Developers *really* don’t like change**

Over time, I realized those are related but distinct phenomena. What I originally mistook for simple resistance turned out to be good old human tribalism: people heavily identify with the tools they invest their time into. Rust just happens to sit in a domain (systems programming) where that identification runs deep.

This lens makes the more unhinged parts of Rust discourse—especially after recent Cloudflare outages—make unfortunate sense. People didn’t want to talk about the bug. They wanted the bug to validate their worldview.

At this point, let me be honest: I am a big fan of Rust. This article is not neutral. But I am tired of the same hollow arguments looping endlessly online.

One in particular:

> **“If Rust is so perfect, why can X still happen?”**

Let me paraphrase the usual shape of this argument (charitably):

1. Rust promises memory safety.
2. Memory safety should eliminate certain classes of bugs.
3. Therefore anything bad happening in Rust is a betrayal of its promises.
4. `unsafe` exists, therefore Rust is lying.
5. Rust’s benefits are therefore “fake.”

This is not what Rust claims. Let’s actually look at what it does.

---

# **What *Does* Rust Promise?**

Rust’s “safety guarantees” are built on a tiny, almost boring set of rules—first explained in *The Book*:

> **1. Each value in Rust has a single owner.**
> **2. Only one owner may exist at a time.**
> **3. When the owner goes out of scope, the value is dropped.**

These rules prevent:

* use-after-free
* double free
* dangling pointers
* data races
* aliasing violations

## **Borrowing and Lifetimes**

Those three ownership rules are augmented by rules about references:

> **You may have *either*:**
>
> * **One mutable reference**, or
> * **Any number of immutable references**
>   **at a time. But not both.**

And:

> **No reference may outlive the data it points to.**

These rules are checked *at compile time*. And they eliminate entire *categories* of bugs by making them impossible.

These are the “Rust promises.” Not some mythical “bug-free utopia.”

---

# **Examples: What Rust Actually Prevents**

### **1. Dangling pointers**

**C/C++:**

```cpp
int* ptr;
{
    int x = 5;
    ptr = &x;
}
return *ptr; // UB, but compiles fine
```

**Rust:**

```rust
let r = {
    let x = 5;
    &x
}; // ❌ error: borrowed value does not live long enough
```

Rust stops you *before the program runs*.

### **2. Aliasing with mutation**

**C++:**

```cpp
void break_it(int* a, int* b) {
    *a = 10;
    *b = 20; // may mutate same memory -> UB
}
```

**Rust:**

```rust
fn break_it(a: &mut i32, b: &mut i32) { /* ... */ }
// ❌ error if both point to same memory
```

This is the core of Rust’s safety.

---

# **“If Rust Is Safe, Why `unsafe`?”**

Rust includes `unsafe` because **the hardware is unsafe**.

You can’t write:

* OS kernels
* device drivers
* memory allocators
* FFI bindings
* custom data structures

…without direct control over raw pointers.

`unsafe` exists to mark code where **you**, not the compiler, take responsibility for safety guarantees.

### **An Example of a Legitimate `unsafe`**

```rust
pub unsafe fn copy(src: *const u8, dst: *mut u8, len: usize) {
    std::ptr::copy_nonoverlapping(src, dst, len);
}
```

Safe Rust cannot do this. But unsafe code can be **isolated**, **audited**, and **minimized**.

---

# **`unwrap()` in Production**

During the Cloudflare outage, an error bubbled up into an `unwrap()`. This caused a panic.

Cue the takes:

> *“Rust shouldn’t allow bugs like this!”*

**No.**
`unwrap` is a deliberate opt-out of safety.

### **What `unwrap` Actually Does**

```rust
let val = maybe_value.unwrap();
```

* If `maybe_value` is `Some(_)` → fine.
* If it’s `None` → panic.

**This is intentional.**
**This is visible.**
**This is your responsibility.**

Rust *forces* you to acknowledge that something might fail.
`unwrap` is you saying:

> *“It won’t fail. Trust me.”*

Sometimes you're wrong.

---

# **Examples: Safer Alternatives to `unwrap`**

### **1. Explicit error propagation with `?`**

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
```

### **2. Match explicitly**

```rust
match db.get(key) {
    Ok(v) => v,
    Err(e) => return Err(MyError::DbFailure(e)),
}
```

### **3. Use `expect` with context**

```rust
config.get("api_key")
    .expect("missing key: api_key")
```

### **4. Use `todo!()` when scaffolding**

```rust
fn parse_advanced_mode() {
    todo!("Advanced mode parsing unimplemented");
}
```

---

# **Where Does This Leave Rust?**

Rust isn’t collapsing, nor is it taking over the world overnight. What we have instead is:

### **1. A language with real costs.**

Migration is expensive. Rewrites are expensive.
Institutional knowledge does not magically transfer.

Ignoring this does a disservice to everyone.

### **2. A language with real benefits.**

Memory-safety by construction is not marketing fluff.
Rust *does* prevent classes of bugs that have cost companies billions.

### **3. A community that sometimes gets lost in identity battles.**

Rust is not a religion.
C++ is not a religion.
Go is not a religion.

Technical criticism is not an attack.
Refusal to acknowledge flaws is not loyalty—it’s insecurity.

### **4. A language that attracts new developers.**

People *like* Rust.
Not all of them.
Not universally.
But enough that its momentum is real and durable.

That matters.

---

# **Conclusion: The Real State of Rust Today**

Rust is neither the savior nor the villain portrayed online.
It is a powerful, opinionated tool that:

* dramatically improves memory safety
* imposes real costs in learnability
* forces explicitness where other languages allow ambiguity
* requires careful design, especially around error handling
* enables high performance without sacrificing correctness
* attracts passionate (sometimes overly passionate) communities

Rust is **not perfect**, and its users must stop treating it as if it should be.
But it is *meaningful*.
It is *useful*.
It is *different*.
And in an industry drowning in decades of preventable memory bugs, those differences matter.

If the goal is safer, more robust software, Rust is one of the most interesting and pragmatic tools we have.
If the goal is winning discourse battles online… well, good luck with that. The internet already moved on.

Rust didn’t promise perfection.
It promised a tradeoff.
A hard one.
A worthwhile one, for many domains.

And like any tradeoff, the value comes not from pretending it’s flawless—
but from understanding it fully, using it responsibly, and letting it stand on its actual merits.

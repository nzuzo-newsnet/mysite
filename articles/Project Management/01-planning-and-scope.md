#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "How should we decide what to do?"
thumbnail = "https://i.postimg.cc/pd1RWNGX/G2CM-BI108-Backlog-Images-Project-mgmt-approach-V1b.png"
category = "Educational"
show_references = true

[[article-series]]
name = "Project Management"
next = "Project Management/02-people"
#####
# Planning and Scope
# The 5 Levels of Project Design

Before we dive in, it's worth noting that there are roughly **three main methodologies** practiced in project management.

I won't go into their details here, but they're all worth mentioning because they each attempt to model the same underlying concept — the **Software Development Life Cycle (SDLC)**.

The **SDLC** is simply a structured way to describe how software is created and maintained. It's a little strange to think about in project management terms, since projects are typically considered *instances* (with a start and end), not *cycles*.

Still, software must come from *somewhere* — and that somewhere follows a repeating pattern:

1. **Design**
2. **Implementation**
3. **Maintenance**
4. **Planning**
5. **Analysis**

Different methodologies interpret and apply these phases in their own ways:

* **Waterfall Model** — breaks the SDLC into distinct, sequential phases.
* **Incremental Development** — overlaps multiple phases so functionality is built gradually in versions.
* **Integration and Configuration** — assembles reusable components into a complete system.

This series will primarily focus on the **Waterfall** and **Incremental** approaches.

The following "Levels" of project design should be approached **sequentially**, since each builds upon the previous one.

---

## Level 1: The Project

At the start of the design process, our main goal is to decide **what we are building**.

To figure that out, we need to define **what our "thing" needs to do** — in other words, our **requirements** — and how far we'll go when implementing them (our **scope**).

A vague requirement such as *"I need a vehicle that moves"* is not useful until scope is defined. Are you moving around your neighborhood? Then a **bicycle** works. Across the country? You'll need a **plane**.

### Why Scope Often Matters More Than Requirements

1. **Most people don't know exactly what they want.**
   * Customers often don't know what's possible.
   * Developers often don't know what's feasible or efficient.
   * Ambiguity can inspire creativity, but it's more often a source of confusion and rework.

2. **Scoping enables better estimation.**
   * Experienced developers learn how long different types of work take.
   * Scope provides a frame for realistic time, effort, and cost calculations.

> **In short:** Requirements define *what* to build; scope defines *how much* of it to build.

---

### What Are Requirements?

Requirements describe the **functionality and usability** that the end user or customer expects from the project.

They're the criteria against which the success of the software is measured.

#### Requirements Dictate:

* **Features and functionality**  
  Define *what* the system must do — e.g., register users, process payments, generate reports.

* **Data inputs and outputs**  
  Define *what data* enters and leaves the system, including any compliance constraints (GDPR, HIPAA, etc.).

* **User content and interfaces**  
  Define *how* information is presented — what the user sees, interacts with, or provides.

* **Constraints**  
  Define limits on performance, usability, or environment. For example:
  > "The mobile app must load within 2 seconds on a 3G connection."

#### Example Requirements

```text
The system shall allow users to register, log in, and reset passwords via email verification.
The system shall restrict administrative features to users with the 'Admin' role.
The system shall validate input forms and reject incomplete submissions.
All passwords shall be stored using salted SHA-256 hashing.
The codebase shall be modularized for reusability and follow company naming conventions.
```

These statements follow the typical structure:

> "The system (or component) shall [perform some action]."

They describe **what** must happen — not **how** or **why**.

---

### Why Prepare Requirements at All?

In large organizations, requirements come pre-defined.

But for hobbyists or small teams, it's tempting to skip this part — *"I'll just do it in my head."*

Still, formalizing requirements is invaluable when facing uncertainty.

#### Benefits

1. **Reduced Risk**
   * Keeps focus on concrete goals.
   * Prevents over-engineering and feature creep.
   * Forces early identification of constraints.

2. **Improved Efficiency**
   * Aligns understanding across team members.
   * Reduces miscommunication and rework.
   * Enables measurable progress tracking.

#### Example: Turning a Requirement Into a Test

```rust
// Requirement: The system shall reject empty usernames.
fn validate_username(name: &str) -> bool {
    !name.trim().is_empty()
}

#[test]
fn rejects_empty_username() {
    assert!(!validate_username(""));
}
```

By encoding requirements into tests, teams can **continuously verify** that functionality matches intent.

---

### Requirement Checklist

**Functional Requirements** determine whether the software *does* what it's supposed to.

* Example aspects: Inputs, Outputs, Hardware, UI

**Non-Functional Requirements** determine *how well* the software performs its function.

* Example aspects: Performance, Latency, Security, Reliability

**Completeness Requirements** determine whether the system feels "finished."

* Example aspects: Documentation, Logging, Cleanup

**Correctness Requirements** ensure that functionality and documentation are accurate and testable.

* Example aspects: Testability, Readability, Relevance

---

## What Is the Scope of a Project?

Scope defines **how much** of the system is built and **under what constraints**.

It turns abstract requirements into concrete, testable goals.

Scope can be described in two ways:

### 1. Narrative Descriptions

A **narrative scope** reads like a high-level story of the system.

> **Requirement:**  
> "Provide clinicians with a consolidated, real-time view of patient physiological data."
>
> **Narrative Description:**  
> "The system displays a patient vitals dashboard showing heart rate, blood pressure, oxygen saturation, and trends over time. Nurses can filter by time range and annotate readings."

This helps rule out unrelated functionality (e.g., storing voice notes, editing medical history).

For example, if a requirement states that "the system should help doctors make informed decisions," a narrative scope might clarify: "The system will provide decision support by highlighting abnormal values in red and suggesting possible diagnoses based on stored medical protocols. It will *not* include treatment planning, medication management, or direct patient communication features."

---

### 2. Use Cases

A **use case** is more concrete, describing exactly how and when a user interacts with the system.

> **Requirement:**  
> "User should be able to report and block missing or lost credit cards."
>
> **Narrative Description:**  
> "The system allows users to report a missing card and block further transactions."
>
> **Use Case:**  
> "As a customer who has just realized my card is missing, I can open the mobile app, tap 'Report Lost Card,' and instantly block transactions.
>
> **Acceptance Criteria:**
>
> 1. The 'Report Lost Card' button is visible from the home screen.
> 2. The block takes effect immediately.
> 3. Confirmation appears within 3 seconds."

Use cases are **easier to test**, **communicate**, and **reason about** than narratives.

Let's expand on this with another example:

> **Requirement:**  
> "The system should allow teachers to grade student assignments."
>
> **Use Case:**  
> "As a teacher who has just finished reviewing a student's essay, I want to assign a grade and provide feedback so the student can improve.
>
> **Steps:**
>
> 1. Teacher navigates to the 'Assignments' page.
> 2. Teacher selects a specific student's submission.
> 3. Teacher enters a numerical grade (0-100) and optional written feedback.
> 4. Teacher clicks 'Submit Grade.'
> 5. System saves the grade and feedback, and sends a notification to the student.
>
> **Acceptance Criteria:**
>
> 1. Grade must be between 0 and 100.
> 2. Feedback is optional but limited to 500 characters.
> 3. Student receives notification within 10 seconds.
> 4. Grade is immediately visible in the student's grade book."

This level of specificity eliminates ambiguity. A developer reading this knows exactly what to build, and a tester knows exactly what to verify.

---

## Determining Feasibility

Once requirements and scope are defined, assess feasibility:

What's possible given the available **resources**?

### Resources Include:

* **People**
  * More people ≠ faster progress.
  * Adding developers to a late project can slow it down (this is known as Brooks's Law).
  * Analyze *why* a project lags before scaling the team.
  * Consider: onboarding time, communication overhead, and task divisibility.

* **Tooling and Software**
  * IDEs, CI pipelines, and project tracking software can drastically improve productivity.
  * But tools have cost and complexity — adopt only what truly helps.
  * Example: A small team might benefit from GitHub Issues and a simple Makefile, while a larger organization might need Jira, Jenkins, and comprehensive monitoring.

* **Hardware**
  * Ensure development environments meet the project's needs.
  * Learn to do more with less — mastering command-line tools and lightweight workflows pays off.
  * Consider: Do developers need high-performance machines for compilation? Is cloud infrastructure required for testing at scale?

* **Reusable Components**
  * Frameworks, libraries, or shared modules accelerate development.
  * Allocate time to learn and prototype before fully committing.
  * Example: Using a well-established web framework like Django or Axum can save months of development time compared to building from scratch.

> **Tip:**  
> Treat feasibility as an *ongoing process*, not a one-time gate.  
> Reevaluate resources at each project milestone.

For instance, you might discover midway through development that your chosen database technology doesn't scale as expected. By continuously reassessing feasibility, you can pivot to a different solution before the problem becomes critical.

---

## By the End of Design Level 1, You Should Know:

* Programming language(s)
* Subsystems / code organization
* Main classes and responsibilities
* Database schema
* Business rules and constraints
* UI overview
* Resource limitations
* Security and performance requirements
* Scalability goals
* Error handling and fault tolerance
* Frameworks and dependencies

This comprehensive understanding forms the foundation for all subsequent design levels. Without clarity at this stage, later decisions become increasingly difficult and prone to error.

---

## Level 2: Subsystems

At this stage, you should already know **which architectural pattern** you'll use (e.g., layered, hexagonal, microservice-based).

Now it's time to explore how modules will **interact** with each other.

Understanding subsystem interactions is crucial because it determines the overall structure and maintainability of your codebase. Poor subsystem design leads to tight coupling, making changes expensive and risky.

---

### Fan-out

**Fan-out** measures *how many* other modules a given module depends on.

It's a **quantitative** measure of system interdependence.

> **High fan-out** → brittle architecture  
> **Low fan-out** → more modular and maintainable

#### Example

```rust
// High fan-out example: depends on too many components
fn process_order() {
    update_inventory();
    charge_payment();
    send_confirmation_email();
    log_transaction();
}
```

A refactor could introduce an **order service** that encapsulates these interactions:

```rust
fn process_order() {
    order_service::process();
}
```

Let's expand on why high fan-out is problematic. Imagine that each of those four functions (`update_inventory`, `charge_payment`, etc.) changes its interface or behavior. Your `process_order` function now has **four separate reasons to change**. This violates the Single Responsibility Principle and makes testing difficult — you must mock or stub all four dependencies to test `process_order` in isolation.

By introducing an intermediate service, you reduce the direct dependencies and create a more stable interface.

---

### Fan-in

While **fan-out** measures *how many other modules a given module depends on*,  
**fan-in** measures *how many other modules depend on a given module*.

Put differently:

> **Fan-out**: "How many things do I depend on?"  
> **Fan-in**: "How many things depend on me?"

Both metrics help describe the **interconnectivity** of a system.

A healthy architecture maintains a balance between the two.

---

### Understanding Fan-in

A high **fan-in** value means that the module is **widely reused** — it's probably a **core utility or service**.

That's often a good thing, but it also means that **changes to this module can ripple throughout the system**.

> Think of fan-in as a measure of *importance* and *risk*:  
> the higher the fan-in, the greater the potential impact of a change.

---

### Example

```rust
// High fan-in example — A common utility used across modules
pub struct Logger;

impl Logger {
    pub fn log(&self, msg: &str) {
        println!("[LOG]: {}", msg);
    }
}

// Used by multiple modules:
mod auth {
    use super::Logger;
    pub fn authenticate(user: &str, logger: &Logger) {
        logger.log(&format!("Authenticating user: {}", user));
    }
}

mod billing {
    use super::Logger;
    pub fn charge(amount: f64, logger: &Logger) {
        logger.log(&format!("Charging user: ${}", amount));
    }
}

fn main() {
    let logger = Logger;
    auth::authenticate("Alice", &logger);
    billing::charge(42.0, &logger);
}
```

Here, `Logger` has a **high fan-in** because many modules depend on it.

If `Logger`'s interface changes (say, to include timestamps or write to a file), every dependent module may need modification.

---

### Evaluating Fan-in

**Low Fan-in:** Module isn't reused often

* *Implication:* May indicate code duplication or missed abstraction opportunities
* *Example:* A utility function for parsing dates that exists in three different modules instead of being centralized

**Medium Fan-in:** Module reused in a few related places

* *Implication:* Generally healthy and maintainable
* *Example:* A validation module used by authentication and user profile management

**High Fan-in:** Module reused system-wide

* *Implication:* Indicates central utility, but changes carry risk
* *Example:* A logging framework, database connection pool, or configuration manager

---

### Fan-in vs Fan-out

**Fan-out** asks: "How many modules do I depend on?"

* *Indicates:* **Coupling (outgoing)**
* *High value means:* Complex or over-reliant design

**Fan-in** asks: "How many modules depend on me?"

* *Indicates:* **Reusability (incoming)**
* *High value means:* Centralized or critical module

---

### Balancing the Two

* **High fan-in + Low fan-out** → Desirable. The module is simple yet widely useful (e.g., utility libraries, common data types).
  * *Example:* A date formatting utility that depends only on the standard library but is used throughout the application.

* **Low fan-in + High fan-out** → Dangerous. The module depends on many others but isn't reused — a maintenance burden.
  * *Example:* A legacy report generator that imports from ten different modules but is only called from one place.

* **High fan-in + High fan-out** → Risky. The module is central *and* complex — any change can have cascading effects.
  * *Example:* A poorly designed "god object" that coordinates multiple subsystems and is called from everywhere.

* **Low fan-in + Low fan-out** → Isolated. Safe, but potentially underutilized.
  * *Example:* A specialized encryption module used only during initial setup.

---

### Reducing Risk in High Fan-in Modules

1. **Encapsulate functionality tightly**
   * Keep interfaces minimal and stable.
   * Expose only what's needed via `pub(crate)` or traits.
   * Example: Instead of exposing ten different logging methods, expose one parameterized method that handles all cases internally.

2. **Write strong integration tests**
   * Ensure downstream modules don't break with internal refactors.
   * Example: Test that changing the internal implementation of your logger doesn't affect any calling code.

3. **Apply versioning discipline**
   * For shared crates or libraries, use semantic versioning to control compatibility.
   * Example: A breaking change to a widely-used module should trigger a major version bump and clear migration documentation.

4. **Document the interface**
   * High fan-in modules often become de facto APIs for the team. Treat them that way.
   * Example: Provide clear documentation on expected inputs, outputs, error conditions, and usage examples.

---

### Quick Example of Balancing Fan-in and Fan-out

```rust
// Good balance example
pub mod date_utils {
    use chrono::{DateTime, Utc};
    
    pub fn now_iso() -> String {
        Utc::now().to_rfc3339()
    }
}

// Used by multiple subsystems
mod audit_log;
mod reports;
mod analytics;
```

Here, `date_utils` has a **high fan-in** (many depend on it), but a **low fan-out** (it depends only on `chrono`).

That's a clean, stable dependency structure — exactly what we want for shared components.

---

### Cohesion

Cohesion measures how **strongly related and focused** the responsibilities of a single module are. A highly cohesive module does *one thing well* and contains only elements that directly contribute to that single purpose.

In contrast, a poorly cohesive (or *low-cohesion*) module mixes unrelated concerns—like data handling, UI formatting, and network calls—all in one place.

Cohesion is usually a *qualitative* property that reflects how understandable, maintainable, and reusable a module is.

High cohesion tends to reduce bugs and side effects because the logic is well-contained and has a clear reason to exist.

For example:

```rust
// Low cohesion example
pub mod account_service {
    use uuid::Uuid;

    pub struct AccountService {
        pub db_conn: String,
    }

    impl AccountService {
        pub fn create_account(&self, user_name: &str) -> Uuid {
            // Handles database logic
            println!("Connecting to DB: {}", self.db_conn);
            let id = Uuid::new_v4();
            println!("Inserting new account for {}", user_name);
            
            // Also handles unrelated responsibilities:
            // formatting, validation, and even sending an email!
            if user_name.is_empty() {
                panic!("Invalid username");
            }
            
            self.send_welcome_email(user_name);
            id
        }

        fn send_welcome_email(&self, user_name: &str) {
            println!("Sending welcome email to {}", user_name);
        }
    }
}
```

Here, the `AccountService` is doing **too many unrelated things**:

* database management,
* input validation,
* logging, and
* sending emails.

If you change how emails work or how validation happens, you'll have to modify the same module — this is **low cohesion**. It becomes hard to reason about, and unrelated changes start to interfere.

---

### Improved (High-Cohesion) Version

Let's refactor this so that each module has a **clear, single responsibility**.

The `AccountService` will focus purely on *account creation*, while specialized modules handle their own domains.

```rust
pub mod account_service {
    use uuid::Uuid;
    use crate::{database::Database, email::EmailService, validation::Validator};

    pub struct AccountService<'a> {
        pub db: &'a Database,
        pub email_service: &'a EmailService,
        pub validator: &'a Validator,
    }

    impl<'a> AccountService<'a> {
        pub fn create_account(&self, user_name: &str) -> Result<Uuid, String> {
            self.validator.validate_username(user_name)?;
            let account_id = self.db.insert_new_account(user_name)?;
            self.email_service.send_welcome(user_name)?;
            Ok(account_id)
        }
    }
}

// Cohesive, focused modules below:

pub mod validation {
    pub struct Validator;

    impl Validator {
        pub fn validate_username(&self, user_name: &str) -> Result<(), String> {
            if user_name.is_empty() {
                Err("Username cannot be empty".into())
            } else {
                Ok(())
            }
        }
    }
}

pub mod email {
    pub struct EmailService;

    impl EmailService {
        pub fn send_welcome(&self, user_name: &str) -> Result<(), String> {
            println!("Sent welcome email to {user_name}");
            Ok(())
        }
    }
}

pub mod database {
    use uuid::Uuid;

    pub struct Database;

    impl Database {
        pub fn insert_new_account(&self, user_name: &str) -> Result<Uuid, String> {
            println!("Inserted new account for {user_name}");
            Ok(Uuid::new_v4())
        }
    }
}
```

Now, each module has **high cohesion**:

**account_service:** Coordinates the account creation process.

**validation:** Handles validation logic.

**database:** Encapsulates database operations.

**email:** Handles outbound emails.

This separation ensures each module has *one reason to change*.

For example, if the email API changes, you modify only `email.rs` — not `account_service.rs`.

### Bonus Example: Testing Cohesion in Rust

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_account_successfully() {
        let db = Database;
        let email = EmailService;
        let validator = Validator;
        let service = AccountService { 
            db: &db, 
            email_service: &email, 
            validator: &validator 
        };
        
        assert!(service.create_account("Alice").is_ok());
    }

    #[test]
    fn rejects_invalid_usernames() {
        let db = Database;
        let email = EmailService;
        let validator = Validator;
        let service = AccountService { 
            db: &db, 
            email_service: &email, 
            validator: &validator 
        };
        
        assert!(service.create_account("").is_err());
    }
}
```

By keeping each concern separate, testing becomes simpler and more reliable — a hallmark of high cohesion.

---

## Level 3: Classes

The third level of design builds upon the second. After you have defined rules and general interactions between your modules, you need to move a bit more granularly into defining the system's classes.

Some important considerations and design goals at this level include:

**Understanding the responsibilities of each module, and designing its classes (or structs) accordingly.** Being conscious of the cohesion of the system can help guide these decisions as it can quickly become overwhelming to attempt to design responsibilities and interactions while maintaining a reasonable dependency model.

For example, if you have decided that a module will be in charge of networking, it may be tempting to include logic handlers in that module to manage network events directly. This may be faster to develop, but your business logic becomes spread out and disjointed between modules. You could instead create a class in your business logic module that specifically handles network events, keeping the networking module focused solely on connection management, data transmission, and protocol handling.

Consider a concrete example: You're building a multiplayer game. Your networking module should handle TCP/UDP connections, packet serialization, and retry logic. But when a "player joined" packet arrives, the *game logic* module—not the networking module—should decide how to spawn that player, update the game state, and notify other systems. The networking module simply raises an event; the game logic module consumes it.

**In an OOP-focused design, you need to be wary of what you want in your *class* vs what you want in your *objects*.** Once again, conflating the two may make it difficult (and confusing) to manage.

For example, it may be tempting to store global information about your Users (like total user count) in your User class, which is also the instance class for a user. This can very quickly lead to a messy architecture as it can be difficult to manage the static variable amongst instances. Things like concurrency and testing become more difficult if your user object is spread across the codebase.

While this may be a relatively intuitive mistake to avoid, the convenience becomes more tempting when there is a lot of boilerplate involved.

Consider a more complex example, where User functions operate on a database connection. If you have a lot of classes that need to reference that connection, you may want to store the connection statically, or even within the instance to avoid the need to include it in every single function call.

However, this creates several problems:

1. **Testing becomes difficult** — You can't easily mock or replace the database for unit tests.
2. **Concurrency issues arise** — Multiple threads accessing a shared static connection can lead to race conditions.
3. **Lifecycle management becomes unclear** — When should the connection be opened? Closed? Refreshed?

A better approach is to use dependency injection: pass the database connection (or a connection pool) as a parameter to functions that need it, or store it in a context object that's explicitly passed through the call chain.

```rust
// Avoid this: static connection embedded in the class
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    // Bad: relies on global state
    pub fn save(&self) {
        GLOBAL_DB_CONNECTION.execute("INSERT INTO users ...");
    }
}

// Prefer this: explicit dependency
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    // Good: explicit dependency
    pub fn save(&self, db: &Database) -> Result<(), DbError> {
        db.execute("INSERT INTO users (id, name) VALUES (?, ?)", &[&self.id, &self.name])
    }
}
```

This level of design is a little tricky to work with in isolation. There are probably still a few unanswered questions about if various classes *can* interact the way you initially thought. If you have less experience with a language or architectural design, you may need to spend a lot more time researching the types of relations you intend on creating and how your classes should be accommodating those limitations.

Avoiding general anti-patterns, like circular dependencies or excessive function sizes, makes it easier to have these heuristics applied "by default."

For instance, if Class A depends on Class B, and Class B depends on Class A, you have a circular dependency. This usually indicates a design flaw. The solution often involves introducing a third class or interface that both can depend on, or rethinking the responsibilities so the dependency only flows in one direction.

Sometimes, you may need to think about the application of your design outside of an OOP framework. This can be challenging because it is difficult to think about your codebase more abstractly, but learning this skill can improve performance and management of classes drastically.

For example, instead of creating classes for each particle or object in your game engine, you might want to explore Data-Oriented Design (DOD) that moves your "objects" into various classes that are instead responsible for the change in data, not the object itself.

In DOD, instead of having a `Particle` class with position, velocity, and color, you might have separate arrays for positions, velocities, and colors. Operations then iterate over these arrays in parallel, which is more cache-friendly and often faster.

```rust
// Object-Oriented approach
struct Particle {
    position: Vec3,
    velocity: Vec3,
    color: Color,
}

let particles: Vec<Particle> = vec![/* ... */];

// Data-Oriented approach
struct ParticleSystem {
    positions: Vec<Vec3>,
    velocities: Vec<Vec3>,
    colors: Vec<Color>,
}

impl ParticleSystem {
    fn update(&mut self, delta_time: f32) {
        for i in 0..self.positions.len() {
            self.positions[i] += self.velocities[i] * delta_time;
        }
    }
}
```

This level of design is best executed in very close conjunction with the next.

---

## Level 4: Classes → Routines

At this point, you need to decide how the flow of data, events, or operations is going to be in your system. While it may be simpler to use objects to abstract real-world entities, this level of design requires you to consider exactly *how your data is going to change* so that you can figure out *what to do to change your data*.

My favorite way to start this process is by asking myself what I would like the API of a module or class to look like for its use case.

For example:

Let's say you would like to insert data into some store. Your library requires that the data is verified in some way before interacting with the backend.

If you want your API to be super clean, you may want this:

```rust
database.put(some_data);
```

The general flow would look like this:

user insert → our database API → we verify data → insert

The issues begin when you start thinking about verification:

```rust
fn put(&self, data: T) -> Result<(), SomeError> {
    // Verify
    match data {
        OneThing => if data.id != 0 { /* ... */ } else { return Err(SomeError) },
        AnotherThing => if data.other_condition() { /* ... */ } else { return Err(SomeError) },
        _ => // ...
    }
    
    // Transform
    let encoded = data.encode();
    
    // Insert
    backend.insert(encoded)
}
```

There is a clear flow of data being encapsulated into one function. This is difficult to debug because if there are other places with this same flow, you may need to visit all of them to see where the encoding error arose.

Additionally, this violates the Single Responsibility Principle—`put` is doing verification, transformation, *and* insertion. A better approach separates these concerns:

```rust
pub struct Database<'a> {
    validator: &'a Validator,
    encoder: &'a Encoder,
    backend: &'a Backend,
}

impl<'a> Database<'a> {
    pub fn put(&self, data: T) -> Result<(), SomeError> {
        let validated = self.validator.validate(data)?;
        let encoded = self.encoder.encode(validated)?;
        self.backend.insert(encoded)?;
        Ok(())
    }
}
```

Now, each step is isolated and testable. If encoding fails, you know exactly where to look. You can also reuse the validator and encoder in other contexts.

---

In another example, let's say you have an array of objects that need to be updated frequently at some event (a fairly common pattern in simulations, game engines, or GUI libraries):

```rust
let some_vect = vec![ThisObject, ThisObject, ThisObject, ThisObject];

fn update_property(some_objects: Vec<ThisObject>, some_mutation: u8) {
    for object in some_objects {
        object.property = object.property + some_mutation;
    }
}

fn update_another_property(some_objects: Vec<ThisObject>, some_mutation: u8) {
    for object in some_objects {
        object.other_property = object.other_property + some_mutation;
    }
}
```

Here, if you wanted to update two properties, you would need to iterate over all objects twice. This is inefficient and error-prone.

Here's what you could do instead:

```rust
struct ThisObject {
    property: u8,
    other_property: u8,
}

impl ThisObject {
    // Encapsulate updates within the object
    fn update(&mut self, mutation: u8, other_mutation: u8) {
        self.property = self.property.saturating_add(mutation);
        self.other_property = self.other_property.saturating_add(other_mutation);
    }
}

fn update_all(objects: &mut [ThisObject], mutation: u8, other_mutation: u8) {
    for object in objects {
        object.update(mutation, other_mutation);
    }
}
```

Now, you iterate once and update both properties simultaneously. This is faster and easier to maintain.

Alternatively, if you're working with large datasets and performance is critical, consider a data-oriented approach:

```rust
struct ObjectSystem {
    properties: Vec<u8>,
    other_properties: Vec<u8>,
}

impl ObjectSystem {
    fn update_all(&mut self, mutation: u8, other_mutation: u8) {
        for prop in &mut self.properties {
            *prop = prop.saturating_add(mutation);
        }
        for other_prop in &mut self.other_properties {
            *other_prop = other_prop.saturating_add(other_mutation);
        }
    }
}
```

This enables SIMD optimizations and better cache locality, which can significantly improve performance in tight loops.

---

### More Examples: Designing Routine Interactions

Let's consider a more complex example: a notification system.

**Requirement:** "The system shall notify users of important events via email and push notifications."

**Poor Design (Low Cohesion, High Coupling):**

```rust
fn notify_user(user_id: UserId, message: &str) {
    // Send email
    let email_client = EmailClient::new();
    email_client.send(user_id, message);
    
    // Send push notification
    let push_client = PushClient::new();
    push_client.send(user_id, message);
    
    // Log notification
    println!("Notified user {} with message: {}", user_id, message);
}
```

Problems:

1. **Hard to test** — You can't test email and push separately.
2. **Hard to extend** — Adding SMS requires modifying `notify_user`.
3. **Tight coupling** — `notify_user` knows about email, push, and logging.

**Better Design (High Cohesion, Low Coupling):**

```rust
trait NotificationChannel {
    fn send(&self, user_id: UserId, message: &str) -> Result<(), NotificationError>;
}

struct EmailChannel {
    client: EmailClient,
}

impl NotificationChannel for EmailChannel {
    fn send(&self, user_id: UserId, message: &str) -> Result<(), NotificationError> {
        self.client.send(user_id, message)
    }
}

struct PushChannel {
    client: PushClient,
}

impl NotificationChannel for PushChannel {
    fn send(&self, user_id: UserId, message: &str) -> Result<(), NotificationError> {
        self.client.send(user_id, message)
    }
}

struct NotificationService {
    channels: Vec<Box<dyn NotificationChannel>>,
}

impl NotificationService {
    fn notify(&self, user_id: UserId, message: &str) -> Vec<Result<(), NotificationError>> {
        self.channels
            .iter()
            .map(|channel| channel.send(user_id, message))
            .collect()
    }
}
```

Now:

* Each channel is isolated and testable.
* Adding a new channel (SMS, Slack, etc.) doesn't require modifying `NotificationService`.
* The service orchestrates channels without knowing their implementation details.

---

## Level 5: Design Routine Interactions and Data

The first step in this phase is to make sure that, at a high level, the defined functions fulfill the requirements. You may not know all the requirements or the details of how they will be fulfilled, but at the very least you have an intuition of "The system will execute function x to fulfill requirement a."

Make sure that you consider all requirements **before** you begin writing code. Performance and security considerations are difficult to refactor into an existing codebase without an extensive amount of reworking.

For example, if you discover late in development that your authentication system doesn't meet security requirements (e.g., passwords aren't properly hashed, sessions aren't invalidated correctly), you'll need to refactor significant portions of the codebase. This is costly and risky. By considering these requirements upfront, you design the authentication system correctly from the start.

It is at this point that you will spend some time designing and/or documenting your *intention* for the system.

This level is by far the best point to begin reducing the work for the implementation phase, as specificity reduces the mental work necessary to implement. If done well, you can reduce the number of instances where you ask yourself how to implement specific functionality.

Additionally, it helps create a clear consensus for everybody involved (including Future You) so that you can more easily divide work. Knowing that if one team relies on the outputs of another, the postconditions are clear even before the functions are created.

To maximize the benefits here, here are a few things that you might want to finalize at this stage:

* **Requirements and documentation of them**  
  Every function should map back to at least one requirement. If a function doesn't fulfill a requirement, question why it exists.

* **Chosen design patterns**  
  Document which patterns you're using and where (e.g., Factory for object creation, Observer for event handling, Strategy for interchangeable algorithms).

* **Parameters and return values**  
  Define the signature of each function. What does it take as input? What does it return? What are the edge cases?
  
  Example:
  ```rust
  /// Authenticates a user by username and password.
  /// 
  /// # Parameters
  /// - `username`: The user's unique identifier
  /// - `password`: The plaintext password
  /// 
  /// # Returns
  /// - `Ok(SessionToken)` if authentication succeeds
  /// - `Err(AuthError::InvalidCredentials)` if credentials are wrong
  /// - `Err(AuthError::AccountLocked)` if the account is locked
  /// 
  /// # Security
  /// - Passwords are compared using constant-time comparison
  /// - Failed attempts are rate-limited
  fn authenticate(username: &str, password: &str) -> Result<SessionToken, AuthError>;
  ```

* **Pre- and post-conditions**  
  What must be true before the function executes? What will be true after?
  
  Example:
  ```rust
  /// Withdraws money from an account.
  /// 
  /// # Preconditions
  /// - `account_id` must exist in the database
  /// - `amount` must be positive
  /// - Account balance must be >= amount
  /// 
  /// # Postconditions
  /// - Account balance is reduced by `amount`
  /// - Transaction is logged
  /// - If balance falls below minimum, a warning flag is set
  fn withdraw(account_id: Uuid, amount: f64) -> Result<(), WithdrawError>;
  ```

* **Assertions and checks to enforce these**  
  Use debug assertions for internal invariants and runtime checks for external inputs.
  
  Example:
  ```rust
  fn withdraw(account_id: Uuid, amount: f64) -> Result<(), WithdrawError> {
      // Precondition checks
      if amount <= 0.0 {
          return Err(WithdrawError::InvalidAmount);
      }
      
      let account = get_account(account_id)?;
      if account.balance < amount {
          return Err(WithdrawError::InsufficientFunds);
      }
      
      // Perform withdrawal
      account.balance -= amount;
      log_transaction(account_id, amount);
      
      // Postcondition check (in debug builds)
      debug_assert!(account.balance >= 0.0, "Balance went negative!");
      
      Ok(())
  }
  ```

* **Some high-level implementation plan**  
  Sketch out the algorithm or approach. You don't need to write pseudocode for every line, but outline the major steps.
  
  Example:
  ```text
  Function: process_payment
  1. Validate payment details (card number, CVV, expiry)
  2. Check for fraud (via external API)
  3. Charge card (via payment gateway)
  4. If successful:
     a. Update order status to "paid"
     b. Send confirmation email
     c. Log transaction
  5. If failed:
     a. Log failure reason
     b. Notify user
     c. Retry if transient error (up to 3 times)
  ```

By documenting these details, you create a blueprint that makes implementation almost mechanical. Developers (including yourself) can focus on writing correct code rather than figuring out what to write.

---

# Goals and Deliverables

At some point during the previous, or future design and project activities, you may start to think about the outputs of the project.

Depending on your methodology and communication style, you may need to spend some time considering what measuring stick you are going to use to track progress.

As overused as it is, SMART is still a fairly reasonable way to guide your goal setting.

### Specific

For big and small teams alike, it is important that everyone is on the same page. This is different from making sure that everybody understands.

Understanding can only really be measured by the person that is being assessed, and it is still possible that everybody *understands* the same thing differently. Specificity is the easiest way to mitigate this risk.

**Example of vague goal:** "Improve the website."

**Example of specific goal:** "Reduce the homepage load time from 4 seconds to under 2 seconds for users on 3G connections."

The second goal is specific because it defines *what* (load time), *how much* (from 4s to <2s), and *for whom* (3G users). There's no ambiguity.

### Measurable

There are many reasons you would need your goals to be measurable. Mentality, progress tracking, or even simply tangibility are important benefits of measurable goals. I personally like to use measures to ensure that understanding carries through effectively. Measures aid in specificity.

**Example:** "Improve system reliability" is not measurable. "Achieve 99.9% uptime over the next quarter" is measurable—you can track uptime and know exactly when you've met the goal.

Measurability also enables intermediate checkpoints. If your goal is 99.9% uptime in three months, you can measure weekly uptime and course-correct if you're falling short.

### Agreed Upon

All stakeholders must agree on the goal. This includes developers, managers, customers, and anyone else affected by the project.

Disagreement leads to misaligned expectations and conflict. If the development team thinks the goal is "build a functional prototype" but the customer expects "production-ready software," you'll have problems.

**Example:** Before starting work, hold a meeting where all parties review and sign off on the goals. Document this agreement.

### Realistic

Goals should be challenging but achievable given the available resources.

Unrealistic goals demoralize teams and lead to burnout. They also encourage corner-cutting and technical debt.

**Example of unrealistic goal:** "Rebuild the entire application from scratch in two weeks with one developer."

**Example of realistic goal:** "Refactor the authentication module to use OAuth2 within the next sprint (two weeks) with the current team of three developers."

To assess realism, consider:

* Historical data — How long did similar tasks take in the past?
* Team capacity — How much time can each person dedicate?
* External dependencies — Are there blockers outside your control?

### Time-Based

Every goal needs a deadline. Without a deadline, there's no urgency, and work expands to fill available time (Parkinson's Law).

**Example:** "Implement user authentication" is open-ended. "Implement user authentication by November 15th" is time-bound.

Time-based goals also enable retrospectives. After the deadline, you can review what worked, what didn't, and adjust future estimates accordingly.

---

Often, goals can be paired with deliverables to assess them.

## Deliverables

Deliverables are essentially any item that needs to be produced/delivered to meet the goals of a project.

Like goals, they need to be specific and verifiable, but more importantly they *themselves* need to tick boxes, outside of the SMART checklist.

It is helpful to create quality standards for both your goals and their deliverables to ensure that everybody can agree that the measure is met.

**Example:**

**Goal:** "Launch the new feature by December 1st."

**Deliverables:**

1. **Design Document** (due November 1st)
   * Quality standard: Reviewed and approved by at least two senior engineers and the product manager.

2. **Functional Code** (due November 20th)
   * Quality standard: All acceptance criteria met, code passes automated tests, code review approved.

3. **Documentation** (due November 25th)
   * Quality standard: User-facing documentation complete, internal API docs generated, no broken links.

4. **Deployed to Production** (due December 1st)
   * Quality standard: Feature flag enabled for 10% of users, no critical bugs reported in first 48 hours.

By defining deliverables and their quality standards, you make progress tangible and remove subjectivity from "done."

---

### Example: The Difference Between Goals and Deliverables

**Goal:** "Improve system security."

This is vague and not directly measurable. What does "improve" mean? How will you know when you've achieved it?

**Better Goal:** "Eliminate all critical and high-severity security vulnerabilities identified in the latest penetration test by the end of Q1."

**Deliverables:**

1. **Penetration Test Report** (received January 5th)
   * Quality standard: Report provided by certified third-party security firm.

2. **Vulnerability Remediation Plan** (due January 15th)
   * Quality standard: Plan reviewed by CTO and security team, includes timeline and assigned owners for each vulnerability.

3. **Patched Code** (due March 15th)
   * Quality standard: All critical and high-severity vulnerabilities addressed, verified by follow-up penetration test.

4. **Follow-Up Penetration Test** (due March 25th)
   * Quality standard: No critical or high-severity vulnerabilities remain.

Here, the goal is clear, and the deliverables provide checkpoints to track progress. If the patched code deliverable is delayed, you know the overall goal is at risk and can take corrective action.

---

### The Danger of Goals Without Deliverables (or Vice Versa)

**Goals without deliverables:** You have a target but no roadmap. Teams don't know what to produce, leading to confusion and wasted effort.

**Example:** "Make the app faster" is a goal without deliverables. Does this mean optimizing database queries? Caching API responses? Reducing image sizes? Without deliverables, developers might optimize the wrong things.

**Deliverables without goals:** You have outputs but no purpose. You might produce artifacts that don't contribute to success.

**Example:** A team delivers a 50-page design document, 10,000 lines of code, and comprehensive tests. But if the goal was "validate feasibility with a quick prototype," these deliverables represent wasted effort. A simple proof-of-concept would have sufficed.

---

## Scheduling

This article does not cover this section in great detail (future series might), but scheduling in the planning phase is a bit weird.

Unless you are undertaking a typical project that everybody has experience in implementing, scheduling can be difficult to adhere to when the implementations are unknown.

On the one hand, you do not want to create schedules that are too tight, because you do not know what hiccups may arise. At the same time, you also would like to actually finish the project, and too much leeway might either not be possible or may slow the momentum of the team.

For example, let's say task A and task B are both scheduled to be completed in 5 days, and their outputs are needed for task C. If task A completes in 1 day, the team spends 4 days idle. If task B needs 7 days, task C is delayed.

**Strategies to mitigate scheduling problems:**

1. **Estimate in ranges, not points.** Instead of "5 days," use "3-7 days" to account for uncertainty.

2. **Build in buffer time.** Add 20-30% buffer for unexpected issues (this is sometimes called "management reserve").

3. **Parallelize when possible.** If task A and task B are independent, assign them to different people so they can run concurrently.

4. **Use rolling wave planning.** Plan near-term tasks in detail and far-future tasks at a high level. As you learn more, refine future plans.

5. **Track actual vs. estimated time.** Over time, you'll develop better intuition for how long things take.

In the next few articles, we will walk through various techniques to manage this issue and how we can estimate work.

---

# Supporting Plans

Below are some additional plans that can be useful for managing teams. They are valuable because they take into account the nature of humans and the need to standardize our interactions and risk management.

Everybody would resolve an obstacle in their own way, but it is important that the measure used to overcome challenges is predictable and efficient.

---

## Human Resources Plan

This is spoken about in the next article, but for now, understand that a human resources plan is a way to manage how we can handle workloads. Humans are themselves resources, and often during a project, may need to be reorganized and optimized.

A human resources plan establishes the key roles and responsibilities so that when time comes to change things, we can ensure that nobody is out of place, unprepared, or ill-suited for change.

**Example components of a human resources plan:**

* **Role definitions:** Who is responsible for what? (e.g., Project Manager, Lead Developer, QA Engineer)
* **Staffing plan:** How many people do we need, and when?
* **Onboarding process:** How do we bring new team members up to speed?
* **Training plan:** What skills do team members need to develop?
* **Conflict resolution process:** How do we handle disagreements or performance issues?

By documenting these details upfront, you avoid ad-hoc decisions that can disrupt the team.

---

## Communications Plan

Depending on your chosen methodology, you may need to define how people interact with one another and *why*. Knowing why an interaction should take place has a few benefits.

### 1. It can focus the interaction

Meetings with a purpose are more productive as there is no place to entertain unrelated work or issues. Regular updates should not include interpersonal issues; instead, a specific sequence of events needs to be defined to handle those.

**Example:** A daily standup meeting should answer three questions:

1. What did I accomplish yesterday?
2. What will I work on today?
3. Are there any blockers?

This structure keeps the meeting short (15 minutes) and focused. Detailed technical discussions or design debates should happen separately.

### 2. They minimize unnecessary communication

Often, people may not necessarily know where to bring up a concern or issue. This is especially problematic if those issues are only urgent in some contexts.

For example, if a team member notices that a module from another class is not working as expected, waiting until the next meeting may be harmful for production. Defining where to go for these kinds of issues prevents that.

**Example communication channels:**

* **Slack/Email:** Non-urgent updates, questions, and discussions.
* **Daily standup:** Quick status updates and blocker identification.
* **Weekly planning meeting:** Sprint planning, backlog grooming.
* **On-call system:** Critical production issues that need immediate attention.
* **One-on-ones:** Career development, personal concerns, sensitive feedback.

By defining these channels, you ensure that information flows to the right people at the right time.

---

Projects are complicated and difficult to execute, but planning how to manage them is the first step to ensuring a project's success.

In essence, there are 5 main activities that you need to plan for to cover your bases:

* **People** — Who's involved, what are their roles, how do they communicate?
* **Product** — What are you building, what are the requirements?
* **Price** — What's the budget, what resources are available?
* **Process** — What methodology will you use, how will work be tracked?
* **Project** — What's the schedule, what are the milestones?

By addressing each of these areas during the planning phase, you set your project up for success. You reduce uncertainty, align stakeholders, and create a shared understanding of what needs to be done and how to do it.

#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "How should we decide what to do?"
thumbnail = "https://th.bing.com/th/id/R.c8fbc3dcf3682cd5713888b8343fe1c9?rik=JXeTCeHX3aWbAw&riu=http%3a%2f%2fducecc.com%2fwp-content%2fuploads%2f2016%2f10%2fBlueprint-of-Home.jpg&ehk=o2QGPTyNMi9c8VjHF4PbPajVbpxvDPfgNUecxqVrcQU%3d&risl=&pid=ImgRaw&r=0"
category = "Educational"
show_references = true

[[article-series]]
name = "Project Management"
#####
# The 5 Levels of Project Design

Before we dive in, it’s worth noting that there are roughly **three main methodologies** practiced in project management.
I won’t go into their details here, but they’re all worth mentioning because they each attempt to model the same underlying concept — the **Software Development Life Cycle (SDLC)**.

The **SDLC** is simply a structured way to describe how software is created and maintained. It’s a little strange to think about in project management terms, since projects are typically considered *instances* (with a start and end), not *cycles*.
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

The following “Levels” of project design should be approached **sequentially**, since each builds upon the previous one.

---

## Level 1: The Project

At the start of the design process, our main goal is to decide **what we are building**.
To figure that out, we need to define **what our “thing” needs to do** — in other words, our **requirements** — and how far we’ll go when implementing them (our **scope**).

A vague requirement such as *“I need a vehicle that moves”* is not useful until scope is defined. Are you moving around your neighborhood? Then a **bicycle** works. Across the country? You’ll need a **plane**.

### Why Scope Often Matters More Than Requirements

1. **Most people don’t know exactly what they want.**

   * Customers often don’t know what’s possible.
   * Developers often don’t know what’s feasible or efficient.
   * Ambiguity can inspire creativity, but it’s more often a source of confusion and rework.

2. **Scoping enables better estimation.**

   * Experienced developers learn how long different types of work take.
   * Scope provides a frame for realistic time, effort, and cost calculations.

> **In short:** Requirements define *what* to build; scope defines *how much* of it to build.

---

### What Are Requirements?

Requirements describe the **functionality and usability** that the end user or customer expects from the project.
They’re the criteria against which the success of the software is measured.

#### Requirements Dictate:

* **Features and functionality**
  Define *what* the system must do — e.g. register users, process payments, generate reports.

* **Data inputs and outputs**
  Define *what data* enters and leaves the system, including any compliance constraints (GDPR, HIPAA, etc.).

* **User content and interfaces**
  Define *how* information is presented — what the user sees, interacts with, or provides.

* **Constraints**
  Define limits on performance, usability, or environment. For example:

  > “The mobile app must load within 2 seconds on a 3G connection.”

#### Example Requirements

```text
The system shall allow users to register, log in, and reset passwords via email verification.
The system shall restrict administrative features to users with the 'Admin' role.
The system shall validate input forms and reject incomplete submissions.
All passwords shall be stored using salted SHA-256 hashing.
The codebase shall be modularized for reusability and follow company naming conventions.
```

These statements follow the typical structure:

> “The system (or component) shall [perform some action].”

They describe **what** must happen — not **how** or **why**.

---

### Why Prepare Requirements at All?

In large organizations, requirements come pre-defined.
But for hobbyists or small teams, it’s tempting to skip this part — *“I’ll just do it in my head.”*

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

| Type               | Description                                                             | Example Aspects                             |
| ------------------ | ----------------------------------------------------------------------- | ------------------------------------------- |
| **Functional**     | Determines whether the software *does* what it’s supposed to.           | Inputs, Outputs, Hardware, UI               |
| **Non-Functional** | Determines *how well* the software performs its function.               | Performance, Latency, Security, Reliability |
| **Completeness**   | Determines whether the system feels “finished.”                         | Documentation, Logging, Cleanup             |
| **Correctness**    | Ensures that functionality and documentation are accurate and testable. | Testability, Readability, Relevance         |

---

## What Is the Scope of a Project?

Scope defines **how much** of the system is built and **under what constraints**.
It turns abstract requirements into concrete, testable goals.

Scope can be described in two ways:

### 1. Narrative Descriptions

A **narrative scope** reads like a high-level story of the system.

> **Requirement:**
> “Provide clinicians with a consolidated, real-time view of patient physiological data.”
>
> **Narrative Description:**
> “The system displays a patient vitals dashboard showing heart rate, blood pressure, oxygen saturation, and trends over time. Nurses can filter by time range and annotate readings.”

This helps rule out unrelated functionality (e.g., storing voice notes, editing medical history).

---

### 2. Use Cases

A **use case** is more concrete, describing exactly how and when a user interacts with the system.

> **Requirement:**
> “User should be able to report and block missing or lost credit cards.”
>
> **Narrative Description:**
> “The system allows users to report a missing card and block further transactions.”
>
> **Use Case:**
> “As a customer who has just realized my card is missing, I can open the mobile app, tap ‘Report Lost Card,’ and instantly block transactions.
> **Acceptance Criteria:**
>
> 1. The ‘Report Lost Card’ button is visible from the home screen.
> 2. The block takes effect immediately.
> 3. Confirmation appears within 3 seconds.”

Use cases are **easier to test**, **communicate**, and **reason about** than narratives.

---

## Determining Feasibility

Once requirements and scope are defined, assess feasibility:
What’s possible given the available **resources**?

### Resources Include:

* **People**

  * More people ≠ faster progress.
  * Adding developers to a late project can slow it down.
  * Analyze *why* a project lags before scaling the team.

* **Tooling and Software**

  * IDEs, CI pipelines, and project tracking software can drastically improve productivity.
  * But tools have cost and complexity — adopt only what truly helps.

* **Hardware**

  * Ensure development environments meet the project’s needs.
  * Learn to do more with less — mastering command-line tools and lightweight workflows pays off.

* **Reusable Components**

  * Frameworks, libraries, or shared modules accelerate development.
  * Allocate time to learn and prototype before fully committing.

> **Tip:**
> Treat feasibility as an *ongoing process*, not a one-time gate.
> Reevaluate resources at each project milestone.

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

---

## Level 2: Subsystems

At this stage, you should already know **which architectural pattern** you’ll use (e.g., layered, hexagonal, microservice-based).
Now it’s time to explore how modules will **interact** with each other.

---

### Fan-out

**Fan-out** measures *how many* other modules a given module depends on.
It’s a **quantitative** measure of system interdependence.

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

---

## Fan-in

While **fan-out** measures *how many other modules a given module depends on*,
**fan-in** measures *how many other modules depend on a given module*.

Put differently:

> **Fan-out**: “How many things do I depend on?”
> **Fan-in**: “How many things depend on me?”

Both metrics help describe the **interconnectivity** of a system.
A healthy architecture maintains a balance between the two.

---

### Understanding Fan-in

A high **fan-in** value means that the module is **widely reused** — it’s probably a **core utility or service**.
That’s often a good thing, but it also means that **changes to this module can ripple throughout the system**.

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
If `Logger`’s interface changes (say, to include timestamps or write to a file), every dependent module may need modification.

---

### Evaluating Fan-in

| Fan-in Level | Interpretation                        | Implication                                                       |
| ------------ | ------------------------------------- | ----------------------------------------------------------------- |
| **Low**      | Module isn’t reused often             | May indicate code duplication or missed abstraction opportunities |
| **Medium**   | Module reused in a few related places | Generally healthy and maintainable                                |
| **High**     | Module reused system-wide             | Indicates central utility, but changes carry risk                 |

---

### Fan-in vs Fan-out

| Measure     | Question                           | Indicates                  | High Value Means               |
| ----------- | ---------------------------------- | -------------------------- | ------------------------------ |
| **Fan-out** | “How many modules do I depend on?” | **Coupling (outgoing)**    | Complex or over-reliant design |
| **Fan-in**  | “How many modules depend on me?”   | **Reusability (incoming)** | Centralized or critical module |

---

### Balancing the Two

* **High fan-in + Low fan-out** → Desirable. The module is simple yet widely useful (e.g., utility libraries, common data types).
* **Low fan-in + High fan-out** → Dangerous. The module depends on many others but isn’t reused — a maintenance burden.
* **High fan-in + High fan-out** → Risky. The module is central *and* complex — any change can have cascading effects.
* **Low fan-in + Low fan-out** → Isolated. Safe, but potentially underutilized.

---

### Reducing Risk in High Fan-in Modules

1. **Encapsulate functionality tightly**

   * Keep interfaces minimal and stable.
   * Expose only what’s needed via `pub(crate)` or traits.

2. **Write strong integration tests**

   * Ensure downstream modules don’t break with internal refactors.

3. **Apply versioning discipline**

   * For shared crates or libraries, use semantic versioning to control compatibility.

4. **Document the interface**

   * High fan-in modules often become de facto APIs for the team. Treat them that way.

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
That’s a clean, stable dependency structure — exactly what we want for shared components.

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

If you change how emails work or how validation happens, you’ll have to modify the same module — this is **low cohesion**. It becomes hard to reason about, and unrelated changes start to interfere.

---

### Improved (High-Cohesion) Version

Let’s refactor this so that each module has a **clear, single responsibility**.
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

| Module            | Responsibility                            |
| ----------------- | ----------------------------------------- |
| `account_service` | Coordinates the account creation process. |
| `validation`      | Handles validation logic.                 |
| `database`        | Encapsulates database operations.         |
| `email`           | Handles outbound emails.                  |

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
        let service = AccountService { db: &db, email_service: &email, validator: &validator };

        assert!(service.create_account("Alice").is_ok());
    }

    #[test]
    fn rejects_invalid_usernames() {
        let db = Database;
        let email = EmailService;
        let validator = Validator;
        let service = AccountService { db: &db, email_service: &email, validator: &validator };

        assert!(service.create_account("").is_err());
    }
}
```

By keeping each concern separate, testing becomes simpler and more reliable — a hallmark of high cohesion.


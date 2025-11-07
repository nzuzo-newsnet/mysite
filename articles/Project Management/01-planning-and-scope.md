#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "How should we decide what to do?"
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

### Coupling

**Coupling** measures how *tightly* modules depend on each other’s internals.
It’s a **qualitative** measure of how fragile the relationships are.

#### Tight Coupling Example

(As shown in your version — `Account` calling `Bank` directly.)

#### Improved (Message-Passing) Version

(Your Rust message-passing refactor — excellent example retained.)

---

### Cohesion

**Cohesion** measures how *focused* a module’s responsibilities are.
A cohesive module has **one reason to change**.

#### Low Cohesion Example

(Your original `AccountService` example retained.)

#### High Cohesion Example

(Your refactored version retained — it’s clear, idiomatic, and well-explained.)

---

### Quick Cohesion Summary

| Property        | High Cohesion                  | Low Cohesion     |
| --------------- | ------------------------------ | ---------------- |
| Responsibility  | Single, well-defined           | Mixed, unrelated |
| Maintainability | Easy                           | Difficult        |
| Reusability     | High                           | Low              |
| Example         | `AccountService` + `Validator` | One “god” module |

---

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


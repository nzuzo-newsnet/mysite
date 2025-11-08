#####
short_summary = "Software projects often fail not because of bad code, but because of poor planning and unclear structure. This series explores how project management principles — even from corporate settings — can help individual developers design and maintain cleaner, more sustainable software. Learn when (and why) to plan, what counts as a project, and how to adapt management practices for personal or open-source work."
name = "summary"
#####
# **Designing and Managing Maintainable Software Projects**

### **Introduction**

Designing software is inherently complex.
Early in my career, I often caught myself saying:

> “Oh, so that’s why they did it that way. I should probably leave it alone,”

only moments after thinking:

> “Why on earth is this here? Let me just move it — that feels cleaner.”

That back-and-forth between *intuition* and *understanding* is universal.
It doesn’t only affect large, formal architectures; even small, personal projects can quickly spiral into confusion when design decisions aren’t made deliberately.

One of the most valuable lessons I’ve learned is this:

> **Future Me is not Current Me — and they do not know each other.**

Design isn’t just a professional process — it’s a mental one.
Whether you’re working solo or within a team, structured planning helps align intention and action, making your code more readable, maintainable, and predictable.

---

## **What Is Project Management?**

**Project Management** is the discipline of coordinating complex work so that software can be delivered within specific **constraints** — time, scope, and resources.

It’s often associated with large organizations, and for good reason: businesses require **predictable delivery** and **accountability**.
However, smaller teams and individual developers can also benefit from these principles.

> **Example:**
>
> * A startup building a new mobile app may use *sprints* and *burndown charts* to track progress.
> * A solo developer might simply list weekly milestones and estimate hours, but both are practicing project management — at different scales.

Even when the tools differ, the mindset remains the same: deliberate planning leads to better outcomes.

---

## **What Defines a Project?**

To understand what a *project* is, it helps to clarify what it is **not**.

Projects are often mistaken for **tasks**, **products**, or **processes**, but these terms represent different levels of abstraction in software creation.

---

### **Common Misconceptions**

#### **Programs or Portfolios**

A **program** (or **portfolio**) is a *collection of related projects* grouped by theme or strategic goal.

> **Example:**
>
> * A “Payments Program” might include separate projects for building a payment API, designing a dashboard, and integrating fraud detection.
>   Each project is distinct but contributes to a broader objective.

#### **Products**

A **product** is the **result** of a project, not the project itself.

> **Example:**
> The *project* might be “Build a cross-platform note-taking app.”
> The *product* is the app that users eventually download and use.

Over time, products often spawn new **refactor or update projects** — like adding a sync feature or migrating to a new framework.

#### **Processes**

A **process** is a repeatable set of activities performed regularly.

> **Example:**
>
> * “Deploy the backend service every Friday” — that’s a *process*.
> * “Migrate the backend from AWS to Azure” — that’s a *project*, because it’s a one-time effort with a defined goal.

Processes can be *outputs* of projects (for example, creating an automated testing pipeline) or *targets* of improvement projects (like optimizing build time).

> **Analogy:**
> If you own a bakery, your method for baking vanilla cakes is a process.
> Creating a new cheesecake recipe, however, is a project — and the recipe it produces becomes a new process.

---

### **Key Differences Between Projects and Processes**

| **Property**                     | **Project**                                              | **Process**                                     | **Consequence**                                                                                  |
| -------------------------------- | -------------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Initiation**                   | Created ad hoc for a specific purpose; executed once.    | Routine; designed to repeat indefinitely.       | Projects focus on achieving a specific outcome; processes prioritize consistency and efficiency. |
| **Duration**                     | Has one start and one end date (though these may shift). | Runs continuously or cyclically.                | Projects require flexible goals; processes thrive on predictability.                             |
| **Pre/Postconditions and Tasks** | Often uncertain or exploratory.                          | Well-documented and predictable.                | Processes are easier to onboard new people into; projects require deeper context.                |
| **Success Rate**                 | Typically lower; failures are common and instructive.    | Improves through iteration and standardization. | Processes mature through lessons learned from projects.                                          |
| **Change Scope**                 | Large; can redefine goals or outputs entirely.           | Small; focuses on refining execution.           | Projects evolve rapidly, while processes stabilize over time.                                    |
| **Metrics**                      | Measured by **output quality** and goal fulfillment.     | Measured by **efficiency** and consistency.     | Inefficient processes compound costs; projects conclude and reset.                               |

---

## **What Should You Plan?**

Not every idea needs a detailed plan.
Planning should match the **scale**, **risk**, and **uncertainty** of your work.

A key distinction:

* **Project planning** defines *how* something is done — scheduling, cost estimation, risk mitigation.
* **Software design** defines *what* is done — architecture, data models, and system behavior.

Still, both overlap in practical ways. Ask questions like:

* When should this be completed?
* How will future contributors (or Future You) understand it?
* What dependencies are required?
* How will it be tested?
* How should APIs or interfaces evolve?
* What sequence of events should occur to achieve this goal?

> **Example:**
> A beginner building a “to-do app” might not need a Gantt chart, but writing down *“Finish the CRUD backend before UI work”* can save hours later.
> That’s already lightweight project management in action.

Throughout this series, some techniques may be overkill for what you would like to do. The context of your project is the best way to decide what you need.
Again, over time, you'll gain some intuition about what you need to plan and why.

---

### **When Should You Plan?**

My personal heuristic:

> Estimate the effort.
> Double it.
> Then decide if it’s worth planning in advance.

For small utilities, such as a script to parse CSV files, planning may be minimal.
For multi-component systems — say, an IoT dashboard that collects, stores, and visualizes data — planning becomes critical because there are multiple layers of coordination and long-term maintenance.

Planning exists because most software is **intended to persist**.
Maintaining a project over years requires documentation, consistency, and clarity across all future maintainers (including you).

In production environments, this is more than convenience — it’s survival.
A lack of planning can bring down critical systems or stall delivery due to misunderstood dependencies.

---

## **Why Plan at All?**

Sometimes, you shouldn’t.

Project Management can become a trap where planning overtakes doing.
Over-planning often appears safe but can stall creativity and delay execution.

However, **no planning** can be equally catastrophic.
For paid or mission-critical projects, failing to plan invites cost overruns, scope creep, or outright failure.

The balance depends on two variables:

* **Uncertainty** — how much you don’t yet know
* **Scale** — how much coordination is required

The more uncertainty or coordination, the more valuable planning becomes.

> **Example:**
>
> * Writing a quick script to rename files? Just start coding.
> * Building a REST API for multiple clients? Outline endpoints, error cases, and versioning strategy first.
> * Contributing to an open-source compiler? Absolutely plan — the complexity demands it.

---

### **A Practical Example**

When developing [`netabase_store`](https://github.com/newsnet-africa/netabase_store), integrating with [**libp2p**](https://docs.rs/libp2p/latest/libp2p/index.html) turned out far trickier than expected.

What started as a “simple wrapper” project quickly spiraled into issues like:

* Serialization failures on non-serializable record types
* Excessive cloning for multithreaded communication
* Rust’s borrow checker exposing unplanned design flaws

Each refactor introduced new dependencies and side effects.
Eventually, I realized something fundamental:

> **Uncertainty is the signal that planning is needed.**

Each unexpected problem made later planning more informed.
Once I understood the constraints, development became straightforward — because the hard thinking had already been done.

---

# **This Series**

This series draws from *Software Engineering* by [**Ian Sommerville**](https://books.google.co.za/books/about/Software_Engineering_Global_Edition.html?id=W_LjCwAAQBAJ&redir_esc=y) and adapts its lessons for both **individual developers** and **smaller-scale teams**.

---

## **1. Planning and Control Foundations**

Explores how and why project plans are established — defining goals, setting requirements, and allocating tasks to achieve them effectively.

> **Example Topics:**
>
> * How to define measurable project goals
> * Estimating costs and timelines realistically
> * Balancing flexibility and accountability in solo work

---

## **2. Execution and Risk Management**

Examines practical techniques for executing project plans and managing uncertainty.
We’ll look at methodologies like Agile, Waterfall, and Spiral, and explore how to adapt them for small or individual projects.

> **Example Topics:**
>
> * Managing iterative feedback loops
> * Identifying and quantifying project risks
> * Translating enterprise techniques for personal productivity

---

### **Next Steps**

In upcoming articles, we’ll move from theory to implementation:
how to plan effectively, manage scope, schedule milestones, and handle risk — whether you’re leading a team or just managing your own long-term codebase.

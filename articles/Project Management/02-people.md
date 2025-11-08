#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "How can we manage people effectively?"
thumbnail = "https://postimg.cc/hJxHsYRH"
category = "Educational"
show_references = true

[[article-series]]
name = "Project Management"
prev = "Project Management/01-planning-and-scope"
next = "Project Management/03-metrics"
#####
# People

So far in this series, we have explored how to plan projects and mitigate the risks that arise from their unpredictable nature.
In this article, we continue that exploration — focusing on **project management through the lens of people management.**

---

## The Human Factor in Projects

The relationship between people and projects is fascinating — and complex.
People are nuanced, emotional, and sometimes unpredictable. One of the great challenges of project management is managing these *human resources* so that they can execute and complete a complex project together.

While there is no universal formula for managing people, there **are** effective ways to organize them to realize a shared goal.

---

## Roles and Responsibilities

The first and most straightforward way to manage people is through **the assignment of roles and responsibilities.**

Humans are uniquely valuable because of their identities — each person brings a different combination of:

* Skills
* Preferences
* Strengths
* Perspectives

When harnessed correctly, these differences become the engine of project success.

---

### Skills

A strong team is one with **diverse skill sets and experiences.**
This diversity fuels creativity, but more importantly, it creates resilience — allowing the team to tackle problems from multiple angles.

#### Example: A Mobile App Project

Imagine a project team designing a mobile application.
Naturally, people might organize into sub-teams:

* **UI Team** – focuses on interface design and usability
* **Networking Team** – handles data flow and APIs
* **Systems Team** – manages backend logic and infrastructure

But the benefits of diverse skills go beyond these obvious divisions.

---

#### Technical vs. Non-Technical Strengths

A UI expert may not do what a backend developer can, and vice versa — but *intangible* skills are equally valuable.

Consider:

* **Public speaking & facilitation skills:**
  A person who communicates well under pressure or enjoys presenting may be the ideal choice to lead sprint reviews or stakeholder demos. Their calmness and clarity can boost both morale and stakeholder confidence.

* **Attention to detail:**
  Some people naturally notice inconsistencies or edge cases. Such individuals excel in **code reviews**, **QA**, and **compliance**. Their perceptiveness prevents subtle bugs from turning into costly issues later.

* **Empathy and emotional intelligence:**
  Others are gifted at sensing team tension or unspoken concerns. These individuals thrive in **user research**, **stakeholder relations**, or **conflict resolution** — areas where understanding people is more important than understanding code.

Using both **tangible** and **intangible** skills increases your project’s chance of success.

---

### People Are Not Static

Remember: **people grow, learn, and adapt.**

In long-term projects, rigid role definitions can lead to **stagnation or burnout**. While some stability is necessary to maintain continuity, it’s equally important to allow opportunities for growth and rotation where appropriate.

---

### Benefits of Skill Development

Investing in your team’s development yields several long-term advantages:

1. **Improved cross-team communication**
   When developers understand each other’s domains, collaboration becomes smoother.

   > For instance, a frontend engineer who grasps backend principles will design UI requests that are API-efficient and easier to implement.

2. **More resilient, flexible teams**
   Cross-trained members can fill in for one another.

   > If only one person knows the deployment process, their absence is a risk.
   > But if three people share that knowledge, the team stays productive even under unexpected changes.

3. **Increased creativity through broader knowledge**
   Understanding multiple domains allows for innovative bridges between them.

   > A developer familiar with both UX and async operations might propose **optimistic UI updates**, improving both performance and user experience simultaneously.

---

## Range of Duties and Responsibilities

When planning your team, it helps to think in **responsibility clusters** — broad categories of roles that ensure coverage of all critical aspects of project success.

---

### Project Management Roles

The **Project Manager** (or management team) often encompasses several distinct sub-roles:

#### 🧭 Strategist

Defines long-term vision and aligns it with business priorities.
Balances **technical ambition** with **business pragmatism** — for instance, deciding whether to release a minimal viable product (MVP) now or a fully-featured system later.

#### 🧑‍🤝‍🧑 Leader

Inspires and motivates the team.
Creates psychological safety, celebrates wins, and reframes obstacles as learning opportunities.

#### 🗳️ Politician

Navigates stakeholder politics and competing interests.
Translates project progress into *business language* — e.g., “ROI improvement” instead of “reduced latency.”

#### 🧩 Facilitator

Ensures processes and meetings are productive.
Recognizes when discussions drift off-topic, and keeps collaboration focused and balanced.

#### 🧾 Administrator

Manages logistics: budgets, schedules, documentation, and compliance.
Enables rather than directs — handling procurement, tracking deliverables, and maintaining institutional memory.

---

### Systems Analyst Roles

The **Systems Analyst** bridges the gap between business, users, and technical teams.

#### 🎯 Stakeholder Needs

Engages deeply to uncover *real* needs beneath stated requests.

> Example: A stakeholder asks for “a dashboard like our competitor’s,” but the analyst discovers the underlying goal is reducing customer support calls.

#### 🧠 Interaction Designer

Designs workflows that are intuitive and stress-tolerant.

> In high-stakes environments like hospitals, this might mean clear hierarchies, confirmation prompts, and strong accessibility standards.

#### 💰 Cost Estimator

Produces realistic estimates by considering complexity, integration time, testing, and risk — not just development hours.

---

### User Interface Designer

The **UI Designer** crafts how the system *feels* and *communicates*.

They ensure:

* **Visual hierarchy** – highlights what matters
* **Consistency** – reduces cognitive load
* **Accessibility** – ensures inclusivity
* **Responsiveness** – adapts to multiple devices

> Example: On a project dashboard, upcoming deadlines and critical items should dominate visual space, while historical data is tucked into expandable sections.

They also maintain a **design system** — a living style guide ensuring consistency across components, typography, color usage, and layout.

---

### Architect

The **Software Architect** defines and maintains the technical structure.

#### 🧱 Application Overview

Creates a coherent big picture of how components interact.

> E.g., a React frontend ↔ Node.js API ↔ microservices backend ↔ PostgreSQL database ↔ Redis cache.

#### ⚙️ Performance

Plans for scalability, caching, and load balancing.
Sets **performance budgets** and ensures monitoring tools exist to measure them.

#### 🔌 Middleware

Designs how services communicate and integrate.

> For example, RabbitMQ for asynchronous messaging or Kong for API management.

---

### Documentation

The **Documentation Specialist** ensures that all knowledge — technical, operational, or user-facing — is captured and maintained.

They create and oversee:

* **User guides & tutorials**
* **API and architecture documentation**
* **Deployment & troubleshooting procedures**
* **Contribution guidelines**

> Example: If the team’s API docs exist but lack examples, the documentation specialist might collaborate with developers to add real-world use cases for each endpoint.

---

### Domain-Specific Specialists

These experts bring *deep contextual understanding* of the project’s target industry or discipline.

> Example:
> In healthcare, specialists ensure the scheduling logic reflects real clinic workflows (e.g., 15-minute checkups vs. 60-minute consultations).
> In finance, they ensure regulatory compliance and correct accounting logic.

They validate the system’s real-world fitness — not just its technical correctness.

---

## Choosing People

In reality, you often **don’t get to pick your team**.
You work with the people you have — and your task is to **organize and empower them effectively**.

When you *do* have a say in team composition:

* Identify **gaps** in skill, temperament, or experience.
* Introduce **new members** strategically, balancing short-term disruption against long-term gains.
* Evaluate both **technical** and **interpersonal** fit.

### What to Assess

When evaluating potential team members, consider:

* Problem-solving style
* Communication clarity
* Comfort with ambiguity
* Reaction to feedback
* Conflict management
* Initiative and collaboration balance

Different evaluation tools uncover different insights:

| Evaluation Type           | Reveals                                 |
| ------------------------- | --------------------------------------- |
| **Technical interviews**  | Problem-solving & domain knowledge      |
| **Behavioral interviews** | Values, teamwork, and conflict handling |
| **Work samples / trials** | Actual performance in context           |
| **Reference checks**      | Strengths, weaknesses, and reputation   |

---

## Management for Success

Ultimately, software projects succeed when **motivated people work toward a shared goal.**
Modern software is too complex for individuals to build alone — coordination and shared understanding are essential.

### 1. Encourage Communication

Information flow is the lifeblood of collaboration.
Choose communication patterns that fit your team:

* Daily stand-ups for tight synchronization
* Async updates in chat for distributed teams
* Retrospectives to reflect and realign

Balance **information availability** with **focus time.**

---

### 2. Remove Obstacles

A good manager:

* Shields the team from politics
* Ensures access to tools and resources
* Resolves conflicts early
* Enables productivity rather than dictates it

This is **servant leadership** in action.

---

### 3. Create Psychological Safety

Teams thrive when people can:

* Admit mistakes early
* Ask “dumb” questions safely
* Challenge decisions constructively
* Propose unconventional ideas

Fear suppresses communication; safety enables innovation.

---

### 4. Recognize and Grow People

Beyond pay, people need:

* **Recognition** — feeling their work matters
* **Constructive feedback** — clear, actionable improvement
* **Growth** — opportunities to learn and advance
* **Purpose** — understanding how their work impacts the whole

Motivation grows where meaning is visible.

---

### 5. Stay Adaptive

No plan survives contact with reality.
Requirements evolve, contexts change, people come and go.
Successful managers **adjust course** rather than resist change.

The goal isn’t to eliminate uncertainty — it’s to **navigate it gracefully.**

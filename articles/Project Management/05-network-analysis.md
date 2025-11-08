#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "A Practical guide to network analysis"
thumbnail = "https://i.postimg.cc/pd1RWNGX/G2CM-BI108-Backlog-Images-Project-mgmt-approach-V1b.png"
category = "Educational"
show_references = true
name = "Network Analysis"

[[article-series]]
name = "Project Management"
prev = "Project Management/04-people"
#####
# Network Analysis
Network analysis is the practice of evaluating tasks in a project, their durations and dependencies to find a critical path.
Critical paths are valuable as they highlight the sequence of events that define the projects main progress.
Most tasks that branch out of the Critical path are eventually going to feed back into it as we start collecting parallel work.
That means that if there is a delay in the critical path, the entire project is delayed as it cannot be parallelised.

// An example about assembling a car and measurement verifications along the critical path while tasks like door and seat manufacring feeds into it.

The purpose of network analysis is so that we can calculate the time buffers for tasks.
This information is helpful when delays occur as they allow teams to focus on completion, as opposed to rectifying the delay.

> Slippage can be defined as the result of a project task, or group of tasks falls behind the intended schedule.

Let's take note of some terminology:
**Earliest Start/Finish**: The earliest a task can begin/end if all preceding tasks are completed in the shortest time. Basically the best case scenario for beginning a task.
**Late Start/Finish**: The latest a task can begin/end without delaying the **minimum** completion time.
**Critical Path**: The chain of paths that determine the overall project duration.
  - Multiple paths can be critical.
**Stack/Float**: The amount of leeway permissible in a task, without affecting the critical path.

There are generally two ways to perform a network analysis. Critical Path Management and Critical Chain Project Management. Both of these methods undergo similar sequence of events, which we will explore now.

## Critical Path Management
There are generally six steps involved in Critical Path Management:
// Summarise each step as a "cheat sheet".
1. Label and Estimate your tasks
2. Perform a "forward pass"
3. Perform a "backward pass"
4. Find total float
5. Find free float
6. Identify critical path

### Activity on Node for CPM
Let us start with a basic example.
Say you want to perform a Network analysis on the assembly of a toy car.
This is how you would carry out the steps in a CPM Network analysis:

#### Step 1: Identify and Estimate task duration
Here, we need to find *what* needs to happen and guess how long we think it would take. In my opinion, this skill can only really be developed well through experience.
Estimating is quite an important aspect of Project planning and humans are infamously bad at guessing things they don't have information or experience with.
Nonetheless, it is something that you need to carry out with caution as extremely inaccurate guessed can be detrimental for a project.

Below are the activities, estimated durations $(ED)$ and dependencies:
**Toy car assembly activities**
1. Gather parts: 2 hours | -> 2
2. Assemble chassis: 4 hours | -> 3, 4
3. Install wheels: 1 hours | -> 6
4. Paint body: 2 hours | -> 5
5. Install decals: 1 hours | -> 6
6. Final inspection: 1 hours

At this point, we should have a graph that looks like this:
[![Step-1-NA-drawio-1.jpg](https://i.postimg.cc/nLzWP5GM/Step-1-NA-drawio-1.jpg)](https://postimg.cc/jDpvWM9r)

After that, the guessing is done!

#### Step 2: Forward Pass
Now, from the estimates, you can start the forward pass to populate the **Earliest** fields.
These are essentially the optimistic (not really, they are more so just super trusting of your estimates) estimates for your activities: "Assuming everything has been guess correctly, here are the times".

The earliest start of a node is the highest finish of all preceding nodes:
$Earliest Start (ES) = max(EF(prev))$

The earliest finish of a node is it's own earliest start, plus it's own estimate:
$Earliest Finish (EF) = ES(self) + ED(self)$

For the first node(s), your earliest start is always 0, and therefore the earliest finish will be the same as the task duration estimate:
[![Node0-drawio-2.jpg](https://i.postimg.cc/Y9gzDKjD/Node0-drawio-2.jpg)](https://postimg.cc/wyqN3rXD)

Node 2:
[![Node2-drawio.jpg](https://i.postimg.cc/pXnR0sB2/Node2-drawio.jpg)](https://postimg.cc/nCZyrY55)

This carries on until we reach a join at node 6. Notice that for joins, the *highest* EF of *all* previous nodes is accounted for:
[![Node6-drawio.jpg](https://i.postimg.cc/ZYwxfc5D/Node6-drawio.jpg)](https://postimg.cc/4nhcyptz)

By the end, you should have a complete forward pass:
[![Complete-Net-drawio.jpg](https://i.postimg.cc/MTFXBvz5/Complete-Net-drawio.jpg)](https://postimg.cc/VrXYQsr0)

#### Step 3: Backwward Pass
The values calculated here are calculating the time that a task **absolutely needs to be completed** without delaying the project.
This process is essentially the same in reverse where:

The latest start of a node is the highest finish of all preceding nodes:
$Latest Start (LS) = LF(self) - ED(self)$

The latest finish of a node is it's own latest start, plus it's own estimate:
$Latest Finish (LF) = min(LS(next))$

For the backward pass, you start with the *last* node(s) and fill the network.
For node 6, the numbers are the same as in the forward pass as there is no LS(next):
[![Act6back-drawio.jpg](https://i.postimg.cc/FFcXQ72z/Act6back-drawio.jpg)](https://postimg.cc/XXVzf7N6)

By the end of the backward pass, you should have a graph that looks like this:
[![Complete-Back-drawio.jpg](https://i.postimg.cc/d1hk08Gg/Complete-Back-drawio.jpg)](https://postimg.cc/R6zVsJsT)

#### Step 4: Total Float
Total float is the time that an activity can be delayed without affecting completion date.
If the forward/backward passes informs us of *when* a task should start, total float informs us of *how much extra time* a task can be delayed, without affecting any other activity.
By this point, you already have the necessary values to begin. Float time is just a quick way of showing us the delta between earliest and latest start at a glance.

Total float is defined by:
$TF (self) = LS (self) – ES (self)$

[![Float-drawio.jpg](https://i.postimg.cc/vT9v4R5L/Float-drawio.jpg)](https://postimg.cc/zHzhQctV)

#### Step 5: Free Float
Similarily, free float is the amount of extra time that a project can be extended without delaying the start of finish of any other activity
If the forward/backward passes informs us of *when* a task should start, free float informs us of *how much extra time* a task can take up, without affecting any other activity.
This is a useful metric to use as an early indicator of slippage. If you are working on a task, you may encounter issues that you did not forsee, and this value may help you see if the time taken to solve the issue, and complete the task is feasible.

The calculation is the same as the total float calculation, but for the **end** times:
$TF (self) = min(ES (next)) – EF (self)$

[![Complete-drawio.jpg](https://i.postimg.cc/D0DtTKnq/Complete-drawio.jpg)](https://postimg.cc/4H686qKd)

#### Step 6: Critical Path
For this section, it is important to pay attention to the semantics of the equations and what they mean. You need to understand *what, when and why* you are delaying.
For example, when a decision is made to delay Activity 3, if activity 3 has started, then you can only delay the end time, and the maximum time you can delay the end time for that activity is the free float. In this instance, your delay is because *Acticity 3* is taking longer than usual.
Here, you are delaying your current task (Free float).
If you instead need to delay task 3 because task 2 is running over time (assume that nodes 4 and 5 did not exist), then you would need to delay the *start* time of task 3 and extend the *end* time of task 2.
This intuition is helpful when trying to understand critical path.
The critical path is the string of activities that **cannot** be delayed at all. In the above example, our critical path would be 1, 2, 4, 5, 6 because both their free floats and total floats are 0.

[![Critical-path-drawio.jpg](https://i.postimg.cc/T2DwK25Z/Critical-path-drawio.jpg)](https://postimg.cc/JtRmgLqq)


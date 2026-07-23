# AIOS
### The Open Source AI Operating System

> **Build AI agents. Load them dynamically. Execute them in parallel.**

AIOS is an experimental AI Operating System kernel written entirely in Rust.

Instead of building one massive AI application, AIOS treats AI agents as independent applications that can be loaded dynamically, discovered automatically, scheduled in parallel, and orchestrated by a central Master Agent.

The long-term vision is an operating system where developers build AI applications exactly like writing programs for Linux or Windows.

---

# Why AIOS?

Today's AI systems are mostly monolithic.

One application.

One model.

One execution pipeline.

AIOS takes a different approach.

It introduces an operating-system architecture for AI.

Instead of one AI doing everything...

Many specialized AI agents collaborate to solve problems.

---

# Current MVP Features

- Rust-native AI kernel
- Master Agent orchestration
- Parallel Worker Pool
- Task Scheduler
- Task Graph execution
- Dependency management
- Capability Registry
- Provider Registry
- Built-in Agents
- Dynamic `.so` plugin loading
- External agent SDK
- Agent template generator
- Recovery Engine
- Verification Engine
- Result Aggregator
- Interactive CLI

---

# Architecture

AIOS executes every request through a complete orchestration pipeline.

```text
User
 │
 ▼
CLI
 │
 ▼
Master Agent
 │
 ▼
Goal Analyzer
 │
 ▼
Planner
 │
 ▼
Task Decomposer
 │
 ▼
Dependency Builder
 │
 ▼
Execution Plan (Task Graph)
 │
 ▼
Task Scheduler
 │
 ▼
Worker Pool
 │
 ├──────────────┐
 ▼              ▼
Worker 1    Worker 2 ... Worker N
 │              │
 ▼              ▼
Agent Registry
 │
 ▼
Capability Registry
 │
 ▼
Built-in or External Agent
 │
 ▼
Execution Result
 │
 ▼
Recovery Engine (if execution fails)
 │
 ▼
Verification Engine
 │
 ▼
Result Aggregator
 │
 ▼
Master Agent
 │
 ▼
CLI
 │
 ▼
User
```

---

# Execution Pipeline

Every request follows the same lifecycle.

1. User enters a command.
2. CLI forwards the request.
3. Master Agent receives the goal.
4. Goal Analyzer interprets the objective.
5. Planner creates a workflow.
6. Task Decomposer splits work into smaller tasks.
7. Dependency Builder creates a task graph.
8. Scheduler dispatches ready tasks.
9. Worker Pool executes tasks concurrently.
10. Agent Registry finds capable agents.
11. Capability Registry selects the appropriate provider.
12. Built-in or external agents perform execution.
13. Recovery Engine retries failed work.
14. Verification Engine validates outputs.
15. Result Aggregator merges successful results.
16. Master Agent returns the final response.

---

# Dynamic Plugin System

AIOS supports external AI agents.

Developers can build agents independently from the kernel.

Simply compile an agent as a shared library and copy it into:

```
plugins/
```

When AIOS starts it automatically discovers and loads every compatible plugin.

No kernel modification required.

No recompilation required.

---

# External Agent SDK

AIOS ships with an SDK that allows anyone to create new agents.

Example:

```rust
impl Agent for HelloAgent {

    fn manifest(&self) -> AgentManifest {

        AgentManifest::new()
            .name("Hello Agent")
            .capability("hello")

    }

    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {

        AgentResponse::success(
            request.task_id,
            "Hello AIOS!"
        )

    }

}
```

Compile:

```
cargo build --release
```

Copy:

```
cp target/release/libmy_agent.so plugins/
```

Restart AIOS.

Your agent is now available.

---

# Built-in Commands

```
help
version
agents
services
memory list
memory get <key>
new-agent <name>
exit
```

---

# Example

```
AIOS> agents
```

Output

```
Echo Agent
Math Agent
Hello External Agent
My External Agent
```

---

# Project Structure

```
src/
 ├── aggregator/
 ├── agents/
 ├── capability/
 ├── cli/
 ├── command/
 ├── kernel/
 ├── master/
 ├── memory/
 ├── planner/
 ├── plugin/
 ├── recovery/
 ├── runtime/
 ├── sdk/
 ├── security/
 ├── service/
 ├── task/
 └── verification/

plugins/

examples/

docs/
```

---

# Roadmap

## MVP ✅

- Master Agent
- Scheduler
- Worker Pool
- Plugin Loader
- SDK
- Dynamic Agents

## Next

- Agent Store
- Agent Sandbox
- Remote Agents
- Capability Marketplace
- Distributed Execution
- AI Memory Engine
- Multi-machine Scheduling
- Web Dashboard
- REST API
- Agent Networking
- Performance Metrics
- Plugin Signing
- Agent Permissions

---

# Contributing

AIOS is designed to become an ecosystem.

We welcome contributions in:

- Rust
- AI Infrastructure
- Operating Systems
- Scheduling
- Distributed Systems
- Plugin Development
- Documentation
- Examples
- Testing

Pull requests are welcome.

---

# Vision

The goal is not to build another chatbot.

The goal is to build the operating system that future AI applications run on.

Imagine writing AI software the same way developers write applications for Linux.

That is the long-term vision of AIOS.

---

# License

MIT License

---

## ⭐ If you like AIOS

- Star the repository
- Build an external agent
- Open an issue
- Suggest features
- Contribute code
- Help shape the future of AI operating systems

# VERB_BETA

**VERB_BETA** is an experimental Rust playground created to practice and deeply understand modern Rust networking and service libraries, including:

- **tokio** — async runtime and low-level async I/O
- **axum** — HTTP server framework built on Tower
- **tower / tower-http** — middleware, services, and composable networking layers

---

## Concept

A **verb**, by dictionary definition, is an _action word_.
In software terms, actions translate naturally to **todos**.

**VERB** is a todo application with both:

- a **CLI interface**
- a **web application**

`VERB_BETA` exists purely as a **learning and experimentation ground** before ideas are promoted into the real **VERB** repository.

---

## Purpose of VERB_BETA

This repository is intentionally **experimental**.

It is a place to:

- Practice **Rust networking concepts**
- Explore different **protocols and architectures**
- Write working but intentionally **non-polished** implementations
- Break things, rebuild them, and understand _why_ they work

Only concepts that feel solid, clean, and well-understood will be migrated into the main **VERB** repo.

---

## Planned Experiments

This repo may include small, focused experiments involving:

- **HTTP**

  - REST APIs
  - Middleware
  - Request/response lifecycles

- **WebSockets**

  - Real-time updates
  - Stateful connections

- **gRPC**

  - Binary protocols
  - Strongly-typed APIs

- **Raw TCP**

  - Manual framing
  - Backpressure
  - Async I/O without abstractions

- **UDP**

  - Streaming concepts
  - Fire-and-forget messaging
  - Packet-level experimentation

All implementations are expected to be:

- **byte-sized**
- **intentionally scoped**
- **educational over complete**

They will work — but not necessarily be production-ready.

---

## Philosophy

- This is a **text zone** — a place to explore ideas freely
- Code clarity and understanding matter more than features
- Nothing here is final
- Everything here is a lesson

In the future, `VERB_BETA` may:

- Evolve into a permanent experimental repo
- Become the official beta branch of **VERB**
- Or simply remain a record of learning and exploration

---

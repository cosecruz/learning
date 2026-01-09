# 🧪 LAB 1 — PURE CONCURRENCY (NO ASYNC)

## Project: **Bounded Work-Stealing Job System**

### Goal

Build a **high-performance CPU-bound job executor** using threads only.

This teaches:

- when async is the _wrong_ tool

- cache locality

- contention management

- backpressure without async

---

## Constraints

- **No Tokio**

- **No async/await**

- Must scale with core count

- Must shut down cleanly

---

## Required Crates

```toml
rayon
crossbeam
parking_lot
num_cpus
```

---

## Core Requirements

### 1️⃣ Thread pool with work-stealing

- global injector queue

- per-worker local deque

- steal on starvation

👉 (`crossbeam::deque`)

---

### 2️⃣ Backpressure

- bounded submission queue

- producers must block or fail

---

### 3️⃣ Cancellation & shutdown

- cooperative cancellation flag

- no leaked threads

- join on shutdown

---

### 4️⃣ Failure handling

- panic isolation per job

- supervisor decides:

  - retry

  - drop

  - shutdown pool

---

## Required Patterns

- work stealing

- scoped threads

- lock-free queues

- parking instead of spinning

- memory ordering awareness

---

## Stretch Goals

- job priorities

- CPU pinning

- metrics via atomics

- integration with Rayon as backend

---

# Database Transaction — Abstract Model (RAII View)

## What a Transaction _Is_

A **transaction** is a **scoped state transition guard** over a shared system.

> It defines a _region of time_ during which:
>
> - Changes are **tentative**
> - Invariants must hold at **entry** and **exit**
> - Failure causes **automatic rollback**
> - Success causes **atomic commit**

This is **RAII for system state**.

---

## Core Purpose (One Sentence)

A transaction ensures that **complex multi-step changes either happen completely or not at all**, while preserving system invariants — even in the presence of failure.

---

## Transaction as a State Machine

At an abstract level, a transaction moves through **well-defined states**:

```text
┌─────────┐
│ Created │  (BEGIN)
└────┬────┘
     ↓
┌───────────┐
│ In-Flight │  (reads + writes)
└────┬──────┘
     ↓
┌───────────┐        ┌──────────┐
│ Committed │◄──────▶│ RolledBack│
└───────────┘        └──────────┘
```

Once it reaches **Committed** or **RolledBack**, it is **done forever**.

No resurrection. No partial state.

---

## What a Transaction _Has_

Abstractly, every transaction contains:

### 1. A **Snapshot**

A view of the system at the moment the transaction starts.

- Defines _what you see_
- Isolated from other concurrent changes
- Can be:

  - logical (versioned)
  - physical (locks)

---

### 2. A **Write Set**

A list of _intended changes_.

- Changes are **buffered**
- Not visible to others
- Can be validated or rejected

This is like Rust holding ownership of a resource **but not exposing it yet**.

---

### 3. A **Guard / Scope**

A boundary that ensures:

- Entry invariants hold
- Exit invariants are enforced
- Cleanup is guaranteed

This is exactly RAII.

---

### 4. A **Commit Protocol**

A final check + state transition:

- Are invariants satisfied?
- Are conflicts resolved?
- If yes → publish changes
- If no → discard all changes

---

## What Changes During a Transaction

### What Changes

- Internal buffers
- Temporary versions
- Locks or version numbers
- Intent logs

### What Does _Not_ Change (Until Commit)

- Global visible state
- External observers’ view
- Durable system invariants

This mirrors Rust:

> You can mutate through `&mut T`, but the rest of the world doesn’t see it until the scope ends.

---

## When Is a Transaction “Done”?

A transaction is **done** when it reaches a _terminal state_:

- **Committed** → changes become globally visible
- **Rolled back** → system behaves as if it never existed

After this:

- It cannot be used again
- It cannot observe or mutate state
- Any further access is invalid

(Think: using a moved value in Rust)

---

## Transaction Invariants (The Contract)

Abstract invariants every transaction must maintain:

### 1. **Atomicity**

All changes appear as one indivisible transition.

> No partial visibility.

---

### 2. **Consistency**

System invariants hold:

- before the transaction starts
- after it finishes

The transaction is responsible for _restoring consistency_.

---

### 3. **Isolation**

Intermediate states are not visible.

Equivalent to:

> No aliasing of mutable state across scopes.

---

### 4. **Durability**

Once committed, changes persist beyond failure.

This is _outside_ the transaction’s lifetime but guaranteed by it.

---

## RAII Mapping (Rust Analogy)

Let’s model a transaction **as a Rust guard**:

```rust
struct Transaction {
    pending_changes: Vec<Change>,
    state: TxState,
}

impl Transaction {
    fn commit(mut self) {
        apply(self.pending_changes);
        self.state = TxState::Committed;
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if self.state != TxState::Committed {
            rollback(self.pending_changes.clone());
        }
    }
}
```

### What This Guarantees

- If `commit()` is called → changes apply
- If scope exits early (panic, error, return) → rollback happens
- Cleanup is **guaranteed**

This is textbook RAII.

---

## Concrete Scenario (Bank Transfer)

### Invariant

> Total money in the system must not change.

---

### Steps Without a Transaction (Broken)

1. Subtract from Alice
2. System crashes
3. Bob never receives money

Invariant broken.

---

### Steps With a Transaction (Abstract)

```text
BEGIN TRANSACTION
  snapshot balances
  plan:
    Alice -= 100
    Bob   += 100
  verify Alice >= 100
COMMIT
```

### State Evolution

| Phase     | Alice | Bob | Visible |
| --------- | ----- | --- | ------- |
| Start     | 1000  | 500 | yes     |
| In-flight | 900   | 600 | no      |
| Commit    | 900   | 600 | yes     |

If _anything_ fails:

- All changes discarded
- State returns to start

---

## Why Transactions Feel “Heavy”

Because they are:

- Coordinating **shared mutable state**
- Across **time**
- With **failure tolerance**
- Under **concurrency**

This is the _hardest problem_ in systems.

Rust solves it for memory.
Transactions solve it for data.

---

## Key Insight (This Is the Mental Model)

> A database transaction is a **temporal ownership boundary over global state**, enforced with RAII-like guarantees.

Once you see this:

- ACID makes sense
- Locking makes sense
- MVCC makes sense
- Distributed transactions become _obviously painful_

---

If you want, next we can:

- Map **transactions ↔ borrow checker rules**
- Compare **transactions vs mutexes**
- Model **nested transactions**
- Show **why distributed transactions are hell**
- Or design a **transaction API in Rust from scratch**

Just tell me the direction.

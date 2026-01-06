````md
# Drop Order & RAII — Transaction Guard

## What You'll Learn

- How `Drop` enables automatic cleanup
- Drop order guarantees and why they matter
- Exception safety (what happens when panics occur)
- `mem::forget` and why it exists
- How RAII enforces invariants automatically

---

## The Challenge

> Build a database transaction guard that:

- Begins a transaction on creation
- Commits on explicit `.commit()`
- Rolls back automatically if dropped without commit

---

## Requirements

```rust
struct Transaction<'conn> {
    // Your fields
}

impl<'conn> Transaction<'conn> {
    fn begin(conn: &'conn mut Connection) -> Result<Self, Error>;
    fn execute(&mut self, sql: &str) -> Result<(), Error>;
    fn commit(self) -> Result<(), Error>;
    // No explicit rollback - happens in Drop!
}

// Usage:
let mut conn = Connection::new()?;

{
    let mut tx = Transaction::begin(&mut conn)?;
    tx.execute("INSERT ...")?;
    tx.execute("UPDATE ...")?;
    tx.commit()?;
}  // committed — no rollback

{
    let mut tx = Transaction::begin(&mut conn)?;
    tx.execute("INSERT ...")?;
    // Oops, forgot to commit!
}  // dropped — automatically rolled back
```
````

---

## Mental Maths

### What do I have?

- A **database connection** representing access to shared mutable global state
- The transaction holds:

  - an **exclusive mutable reference** to the connection: `&'conn mut Connection`
  - internal transaction state (tracking progress and finalization)

> The mutable borrow is the isolation mechanism:
> while a transaction exists, nothing else can access or mutate the connection.

---

### What changes in transition?

- **On creation**

  - `BEGIN` is issued on the connection
  - transaction enters the **InFlight** state

- **During execution**

  - SQL statements are executed
  - changes are written to **transaction-local buffers**
  - changes are:

    - tentative
    - isolated
    - not globally visible

- **On commit**

  - all buffered changes are atomically published
  - transaction enters the **Committed** state

- **On drop (without commit)**

  - `ROLLBACK` is issued
  - all buffered changes are discarded
  - system returns to the pre-transaction state

---

### Transaction states

- **Created**

  - transaction object constructed
  - `BEGIN` executed

- **InFlight**

  - statements executed
  - changes buffered and isolated

- **Committed**

  - changes made durable and globally visible
  - rollback must never occur after this point

- **RolledBack**

  - all tentative changes discarded
  - system behaves as if the transaction never existed

A transaction may reach **exactly one terminal state**.

---

### When am I done?

- When `.commit()` completes successfully
- Or when the transaction is dropped and rollback finishes

After this point:

- the transaction must not be reused
- the connection is returned in a valid state
- no further state transitions are allowed

This mirrors a moved value in Rust: once finalized, it is gone.

---

### Invariants

- At most **one transaction** may hold mutable access to a connection at a time
- Global database state changes **only** on successful commit
- Intermediate states are never observable outside the transaction
- If a transaction is dropped while `InFlight`, rollback **must** occur
- Early returns and panics must not violate database invariants

---

## RAII Mapping

- Constructor (`begin`) → `BEGIN`
- Guard lifetime → in-flight transaction
- Explicit finalization (`commit`) → `COMMIT`
- `Drop` → automatic `ROLLBACK` if not committed

> If the guard is not explicitly finalized, cleanup is guaranteed.

---

## About `mem::forget`

- `commit(self)` consumes the transaction
- Normally, `Drop` would still run afterward
- After a successful commit, rollback must **not** occur

`mem::forget(self)`:

- prevents `Drop` from running
- is correct **only because** the transaction has already reached a terminal state
- does not leak database state or violate invariants

Rule of thumb:

> `mem::forget` is only valid when all external side effects are already final and irreversible.

---

## Key Insight

> A database transaction is **temporal ownership of global state**, enforced with RAII.

`Drop` is not just cleanup logic.
`Drop` is the **final enforcer of invariants when everything else fails**.

If the type exists, the invariants hold.
If the type is gone, the system is consistent.

```

```

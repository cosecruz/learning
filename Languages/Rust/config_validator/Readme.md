# Config Validatory - Shared vs Mutability

## What to learn

- Aliasing XOR mutability in practice with real data structures
- How to accumulate state without violating borrow rules
- The difference between collecting errors vs early returns
- When &T → &mut T transitions are needed

---

## The Challenge

Build a configuration validator that **checks multiple rules** and **reports all errors at once**, not just the first one.

---

## Mind maths

- ### What do i have?

- Accumulative state
- Shared access to config
- Mutable access to error collection

> shared immutable access to config
> exclusive mutable access to error

- a config struct

  ```rust
  struct Config{
    port:u16,
    host: String,
    max_connections: usize,
    timeout_ms: u64
  }

  ```

- a validation error struct

  ```rust
  struct ValidationError{
    field: String,
    message: String,
  }

  ```

  > decision: i want the structs to own their data

  #### **Rules**

  ```md
  - all fields must not be empty
  - port: a valid port is from 1>= and <=9999; must be a number
  - host: must be a string; non empty string; localhost or 127.0.0.1; or nay valid host;
  - max_connections; >=1 and <=3
  - timeout_ms; max 1000; whats a valid
  ```

- ### What changes each time?

- config never change
- error accumulates

Each validation step:

- read &config
- mutate mut error collection

- ### When am i done?

- when all rules have run
- not when first error occurs

- ### Invariant(must always hold)

  > At any time

- config is validly borrowed immutably
- error collection is validly borrowed mutably
- no rule can validate another rules assumptions

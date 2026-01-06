# Interior Mutability- Cached Computation Wrapper

## What You'll Learn

- When interior mutability is the right solution
- The differences between Cell<T>, RefCell<T>, and OnceCell<T>
- How to design immutable APIs that cache internally
- Runtime vs compile-time borrow checking trade-offs

---

## The Challengs

Build a wrapper that computes a value lazily and caches it, but exposes only an immutable API.

### Requirements

```rust
struct Cached<T, F> {
    // Your fields here
}

impl<T, F> Cached<T, F>
where
    F: FnOnce() -> T,
{
    fn new(compute: F) -> Self;
    fn get(&self) -> &T;  // Note: &self, not &mut self!
}

// Usage:
let expensive = Cached::new(|| {
    // Expensive computation
    42
});

println!("{}", expensive.get());  // Computes
println!("{}", expensive.get());  // Returns cached value

```

- wrapper that computes a value lazily
- caches it
- exposes only an immutable API: so needs interior mutability and since data not Copy; use RefCell

---

## Mental Maths

- ### What do we have?

- need a cache struct that takes in data
- data: HashMap<K,V>
- computes value lazily: that means if value is not there then compute value
- then cache it
- but if there then return what is already there
- exposes only an immutable API; cannot mutate data;

- ### What changes in transition?

- cache initialization with closure
- API call -> check cache; if not there then compute and cache
- subsequent calls -> retrieve value

- ### When am i done?

- value is computed, cached and retrieve successfully

- ### Invaraint- What must be true for the duration

- API exposes immutable reference

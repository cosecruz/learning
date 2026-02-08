Oh yeah — this is _exactly_ a place where **builders + typestate** shine. What you’re feeling (“`Target` must always be valid, even when inputs are partial”) is the canonical motivation for typestate.

Let’s walk this **conceptually first**, then I’ll show a clean shape you can adopt without over-engineering.

---

## Re-state the core requirement (this matters)

You want:

- `language` → **mandatory**
- `framework`, `kind`, `architecture` → **optional inputs**
- `Target` → **always valid**
- Inference happens automatically
- Invalid combinations are impossible (or rejected early)
- Defaults are applied _only when safe_

That means:

> **Construction is a process, not a single constructor**

This screams _builder with state_.

---

## Why your current approach is “almost there”

Right now you likely have something like:

```rust
Target::new(
    language,
    framework: Option<_>,
    kind: Option<_>,
    architecture: Option<_>,
)
```

Then:

- validate
- infer
- error

This works, but:

- You can call `new()` in invalid ways
- The compiler can’t help you
- “Only language is required” is a _runtime rule_

Typestate moves those rules into the **type system**.

---

## The mental model: construction pipeline

Think of building a `Target` like this:

```
Start
  ↓ (language required)
LanguageSet
  ↓ (optional hints)
HintsAdded
  ↓ (inference + validation)
ResolvedTarget (final)
```

Only the **last state** can give you a `Target`.

---

## The typestate skeleton (minimal, not scary)

### Marker types (zero-sized)

```rust
struct NoLanguage;
struct HasLanguage;

struct Unresolved;
struct Resolved;
```

These are just **compile-time flags**.

---

## The Builder with state parameters

```rust
pub struct TargetBuilder<L, R> {
    language: Option<Language>,
    framework: Option<Framework>,
    kind: Option<ProjectKind>,
    architecture: Option<Architecture>,
    _language: std::marker::PhantomData<L>,
    _resolved: std::marker::PhantomData<R>,
}
```

Yes, this looks heavy — but users never see it.

---

## Step 1: creation (no language yet)

```rust
impl TargetBuilder<NoLanguage, Unresolved> {
    pub fn new() -> Self {
        Self {
            language: None,
            framework: None,
            kind: None,
            architecture: None,
            _language: PhantomData,
            _resolved: PhantomData,
        }
    }
}
```

---

## Step 2: language is mandatory

```rust
impl TargetBuilder<NoLanguage, Unresolved> {
    pub fn language(self, lang: Language) -> TargetBuilder<HasLanguage, Unresolved> {
        TargetBuilder {
            language: Some(lang),
            framework: self.framework,
            kind: self.kind,
            architecture: self.architecture,
            _language: PhantomData,
            _resolved: PhantomData,
        }
    }
}
```

✨ After this point:

- You **cannot** forget to set `language`
- The compiler enforces it

---

## Step 3: optional hints (only after language)

```rust
impl TargetBuilder<HasLanguage, Unresolved> {
    pub fn framework(mut self, fw: Framework) -> Self {
        self.framework = Some(fw);
        self
    }

    pub fn kind(mut self, pt: ProjectKind) -> Self {
        self.kind = Some(pt);
        self
    }

    pub fn architecture(mut self, arch: Architecture) -> Self {
        self.architecture = Some(arch);
        self
    }
}
```

These are safe _hints_, not commitments.

---

## Step 4: resolve (the only way to get Target)

```rust
impl TargetBuilder<HasLanguage, Unresolved> {
    pub fn resolve(self) -> Result<Target, TargetError> {
        let language = self.language.unwrap();

        let kind =
            infer_kind(language, self.framework, self.kind)?;

        let framework =
            infer_framework(language, kind, self.framework)?;

        let architecture =
            infer_architecture(language, self.architecture);

        Ok(Target {
            language,
            framework,
            kind,
            architecture,
        })
    }
}
```

🔥 **This is the key moment**:

- All inference happens in one place
- All validation happens once
- The output is **guaranteed valid**

No `Option` survives past this point.

---

## The final type is boring (that’s good)

```rust
pub struct Target {
    pub language: Language,
    pub framework: Framework,
    pub kind: ProjectKind,
    pub architecture: Architecture,
}
```

No `validate()` method.
No “invalid state”.
No partial construction.

---

## Ergonomics for the user (CLI / API)

### Minimal input

```rust
let target = TargetBuilder::new()
    .language(Language::Rust)
    .resolve()?;
```

### With hints

```rust
let target = TargetBuilder::new()
    .language(Language::TypeScript)
    .kind(ProjectKind::Frontend)
    .resolve()?;
```

### Fully explicit

```rust
let target = TargetBuilder::new()
    .language(Language::Rust)
    .framework(Framework::Axum)
    .kind(ProjectKind::Backend)
    .architecture(Architecture::X86_64)
    .resolve()?;
```

All roads lead to a **valid `Target`**.

---

## When should defaults REFUSE and ask the user?

This becomes very clean with typestate.

You refuse **only in `resolve()`**, when:

| Situation                            | Action         |
| ------------------------------------ | -------------- |
| Language supports only one framework | Auto-infer     |
| Language supports many frameworks    | ❌ error       |
| Framework contradicts project type   | ❌ error       |
| Project type missing but ambiguous   | ❌ error       |
| Architecture missing                 | Default safely |

Errors are now **semantic**, not structural.

---

## Why this fits _your_ system especially well

You’re building:

- an **intent inference engine**
- with **progressive narrowing**
- driven by **domain rules**

Typestate:

- encodes intent stages
- prevents illegal transitions
- keeps your domain logic honest

This is not “clever Rust”.
This is **domain modeling done right**.

---

## Rule of thumb (important)

> Use typestate when **order matters**
> Use builders when **partial input is expected**
> Use both when **validity is non-negotiable**

You hit all three.

---

## If you want next

We can:

- collapse this into fewer type parameters (simplify)
- expose a _non-typestate_ facade for scripting
- add `TryFrom<CliArgs> for Target`
- formalize “ambiguity errors” vs “invalid errors”

Just say the word.

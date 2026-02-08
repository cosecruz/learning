Love this question. You’re thinking about this at the **right abstraction level**: templates are _not files_, they’re **decisions crystallized into structure**.

Let’s do this in layers so it stays sane and future-proof.

---

## 0️⃣ First principle (this will guide everything)

> **Directory structure is for humans.
> Matching logic is for the domain model.**

Your template **folder layout must be navigable without knowing the full model**, but **must not encode business logic** that will rot when options explode.

So:

- Humans browse directories
- Code reads metadata + matcher rules
- Paths are _suggestive_, not authoritative

---

## 1️⃣ Core dimensions (freeze these concepts)

These are **orthogonal axes**. Never mix them.

| Dimension      | Meaning              | Examples                       |
| -------------- | -------------------- | ------------------------------ |
| Language       | Tooling ecosystem    | rust, python, typescript       |
| Project Type   | What the system does | cli, web_api, worker, frontend |
| Framework      | Opinionated stack    | axum, django, react            |
| Architecture   | Internal structure   | layered, mvc, modular          |
| Flavor (later) | Variants             | workspace, monorepo, lib+bin   |

**Key insight:**
👉 _Not every dimension must exist at every level._

---

## 2️⃣ Canonical directory structure (human-first)

You’re already 80% correct. Let’s formalize it.

```
templates/
└── rust/
    ├── cli/
    │   ├── _defaults/
    │   ├── bare/
    │   └── clap/
    │       ├── layered/
    │       │   └── rust_cli_clap_layered/
    │       └── modular/
    │           └── rust_cli_clap_modular/
    │
    ├── web_api/
    │   ├── _defaults/
    │   ├── axum/
    │   │   ├── _defaults/
    │   │   ├── layered/
    │   │   │   ├── single_crate/
    │   │   │   │   └── rust_web_api_axum_layered_single/
    │   │   │   └── workspace/
    │   │   │       └── rust_web_api_axum_layered_workspace/
    │   │   └── hexagonal/
    │   │       └── rust_web_api_axum_hexagonal/
    │   └── actix/
    │       └── layered/
    │           └── rust_web_api_actix_layered/
```

### Why this works

- You can **visually narrow choices**
- You can add depth without breaking old paths
- `_defaults/` enables inheritance
- No directory name ever needs to change

---

## 3️⃣ What `_defaults/` actually means

Defaults are **partial templates**, not runnable projects.

```
templates/rust/web_api/_defaults/
├── template.toml
└── tree/
    ├── src/
    │   └── lib.rs.template
    └── Cargo.toml.template
```

They define:

- baseline layout
- shared configs
- shared conventions

Later templates _overlay_ these.

---

## 4️⃣ Template = metadata + tree (not path-based)

Your domain model is already correct. Let’s formalize expectations.

### `template.toml`

```toml
id = "rust_web_api_axum_layered_workspace"
version = "1.0.0"

[target]
language = "rust"
kind = "web_api"
framework = "axum"
architecture = "layered"

[features]
workspace = true
bin = true
lib = true

[priority]
weight = 100
```

> Path helps discovery
> Metadata defines truth

---

## 5️⃣ Mapping user input → directory traversal (optional but powerful)

User input:

```
rust web_api axum layered
```

You can _try_ this path order:

```
rust/web_api/axum/layered/
rust/web_api/axum/_defaults/
rust/web_api/_defaults/
rust/_defaults/
```

Each layer:

- merges trees
- resolves conflicts last-write-wins
- produces a final `ProjectStructure`

This gives you **progressive specificity**.

---

## 6️⃣ Handling explosion (this is the hard part)

### Don’t encode everything in directories

Bad ❌:

```
rust/web_api/axum/layered/workspace/lib/bin/opentelemetry/
```

Good ✅:

```
rust/web_api/axum/layered/
  ├── single/
  ├── workspace/
```

And then **feature flags** in metadata:

```toml
[features]
telemetry = true
grpc = false
```

---

## 7️⃣ Language-specific structuring (YES, but scoped)

### Rust dimensions (nested under rust only)

| Dimension     | Examples                  |
| ------------- | ------------------------- |
| Crate shape   | single, lib, bin, lib+bin |
| Workspace     | yes / no                  |
| Async runtime | tokio, async-std          |
| CLI parsing   | clap, argh                |

Directory example:

```
rust/web_api/axum/layered/workspace/
```

Metadata handles the rest.

---

## 8️⃣ Architecture is NOT universal

Don’t force this:

```
frontend/react/layered ❌
```

Instead:

- Architecture is **optional**
- Some frameworks imply architecture

Example:

- Django → MVC
- Next.js → App Router
- NestJS → Modular

Your inference logic already models this correctly 👍

---

## 9️⃣ Naming conventions (important)

Template directory names should be:

```
<language>_<project>_<framework>_<architecture>[_flavor]
```

Examples:

- `rust_web_api_axum_layered`
- `ts_fullstack_nextjs_app_router`
- `python_backend_django_mvc`

This helps:

- search
- logs
- registry sync
- debugging

---

## 🔟 What NOT to do (hard-earned lessons)

❌ Don’t:

- encode logic in folder names
- rely on directory depth to imply compatibility
- assume future architectures are known
- force users to choose everything

✅ Do:

- let Target inference fill gaps
- allow partial templates
- allow multiple valid templates per target
- rank templates, don’t assume uniqueness

---

## Final mental model (this matters)

> **Templates are snapshots of architectural decisions.
> Directories are a map, not the territory.**

Your current direction is _excellent_.
You’re designing something closer to **Nix flakes + Cargo templates + Yeoman**, but with **stronger domain guarantees**.

---

Next good steps if you want:

- template inheritance & overlays
- best-match ranking (scoring)
- interactive disambiguation UX
- runtime support (Node/Bun/Deno)

Say where you want to go next — this system is becoming very real.

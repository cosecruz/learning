Perfect — now we’re doing **proper system design**, not ad-hoc enums 👍
Below is a **single, structured, layered decision table** that you can treat as:

- canonical documentation
- a future data file (YAML / JSON / Rust consts)
- the mental model behind your template tree

I’ll structure it **top → down**, with **clear separation of concerns**, and with **defaults + constraints** baked in.

---

# MASTER DECISION TABLE

**Language → Project Type → Framework → Architecture**

This table answers:

> “Given _this_ input, what _valid_ targets exist?”

---

## 1️⃣ Supported Languages (Top 10)

| Lang       | Primary Domains            |
| ---------- | -------------------------- |
| TypeScript | Web, Fullstack, Frontend   |
| JavaScript | Web, Frontend, CLI         |
| Python     | Web API, Fullstack, Worker |
| Java       | Enterprise APIs            |
| C#         | Enterprise APIs            |
| Rust       | Systems, Web API, CLI      |
| Go         | Cloud services             |
| PHP        | Web apps                   |
| Kotlin     | JVM APIs                   |
| Swift      | Apple ecosystem            |

---

## 2️⃣ Canonical Project Types

| Code      | Description        |
| --------- | ------------------ |
| WEB_API   | HTTP / RPC backend |
| FRONTEND  | UI application     |
| FULLSTACK | Integrated FE + BE |
| CLI       | Command-line tool  |
| WORKER    | Jobs / background  |

---

## 3️⃣ Language × Project Type Matrix

(❌ = not typical / not supported)

| Language   | WEB_API | FRONTEND | FULLSTACK | CLI | WORKER |
| ---------- | ------- | -------- | --------- | --- | ------ |
| TypeScript | ✅      | ✅       | ✅        | ✅  | ✅     |
| JavaScript | ✅      | ✅       | ✅        | ✅  | ⚠️     |
| Python     | ✅      | ❌       | ✅        | ✅  | ✅     |
| Java       | ✅      | ❌       | ⚠️        | ⚠️  | ✅     |
| C#         | ✅      | ❌       | ⚠️        | ⚠️  | ✅     |
| Rust       | ✅      | ⚠️       | ⚠️        | ✅  | ✅     |
| Go         | ✅      | ❌       | ❌        | ✅  | ✅     |
| PHP        | ✅      | ❌       | ✅        | ⚠️  | ⚠️     |
| Kotlin     | ✅      | ❌       | ⚠️        | ⚠️  | ✅     |
| Swift      | ⚠️      | ✅       | ❌        | ✅  | ⚠️     |

---

## 4️⃣ Framework Decision Table

(Per Language + Project Type)

### TypeScript / JavaScript

| Project Type | Frameworks (Top 3)       | Default   |
| ------------ | ------------------------ | --------- |
| WEB_API      | Express, NestJS, Fastify | Express   |
| FRONTEND     | React, Vue, Angular      | React     |
| FULLSTACK    | Next.js, Remix, Nuxt     | Next.js   |
| CLI          | oclif, commander, zx     | commander |
| WORKER       | BullMQ, Temporal, custom | BullMQ    |

---

### Python

| Project Type | Frameworks             | Default |
| ------------ | ---------------------- | ------- |
| WEB_API      | FastAPI, Django, Flask | FastAPI |
| FULLSTACK    | Django                 | Django  |
| CLI          | Click, Typer           | Typer   |
| WORKER       | Celery, RQ             | Celery  |

---

### Rust

| Project Type | Frameworks          | Default |
| ------------ | ------------------- | ------- |
| WEB_API      | Axum, Actix, Rocket | Axum    |
| FRONTEND     | Yew, Leptos, Dioxus | Leptos  |
| CLI          | Clap                | Clap    |
| WORKER       | Tokio               | Tokio   |

---

### Go

| Project Type | Frameworks       | Default |
| ------------ | ---------------- | ------- |
| WEB_API      | Gin, Echo, Fiber | Gin     |
| CLI          | Cobra            | Cobra   |
| WORKER       | Temporal, custom | custom  |

---

### Java

| Project Type | Frameworks                      | Default      |
| ------------ | ------------------------------- | ------------ |
| WEB_API      | Spring Boot, Quarkus, Micronaut | Spring Boot  |
| FULLSTACK    | Spring MVC                      | Spring MVC   |
| WORKER       | Spring Batch                    | Spring Batch |

---

### C

| Project Type | Frameworks   | Default      |
| ------------ | ------------ | ------------ |
| WEB_API      | ASP.NET Core | ASP.NET Core |
| FULLSTACK    | ASP.NET MVC  | ASP.NET MVC  |
| WORKER       | Hangfire     | Hangfire     |

---

### PHP

| Project Type | Frameworks       | Default |
| ------------ | ---------------- | ------- |
| WEB_API      | Laravel, Symfony | Laravel |
| FULLSTACK    | Laravel          | Laravel |

---

## 5️⃣ Architecture Decision Table

(Framework × Project Type → Allowed Architectures)

### Canonical Architectures

| Code      | Meaning                         |
| --------- | ------------------------------- |
| LAYERED   | Controllers → services → domain |
| MVC       | Model–View–Controller           |
| MODULAR   | Feature-based                   |
| HEXAGONAL | Ports & adapters                |
| COMPONENT | UI components                   |

---

### Architecture Compatibility

| Project Type | Allowed Architectures       | Default   |
| ------------ | --------------------------- | --------- |
| WEB_API      | Layered, Modular, Hexagonal | Layered   |
| FRONTEND     | Component, Modular          | Component |
| FULLSTACK    | MVC, Layered                | MVC       |
| CLI          | Layered, Modular            | Layered   |
| WORKER       | Layered, Modular, Hexagonal | Layered   |

---

### Framework Constraints (examples)

| Framework   | Allowed Architectures |
| ----------- | --------------------- |
| Axum        | Layered, Hexagonal    |
| Actix       | Layered               |
| Django      | MVC                   |
| FastAPI     | Layered               |
| Spring Boot | Layered, Hexagonal    |
| Next.js     | Component             |
| React       | Component             |
| Laravel     | MVC                   |

---

## 6️⃣ Default Inference Rules (formal)

| Input Given                        | Inference Rule                                       |
| ---------------------------------- | ---------------------------------------------------- |
| Language only                      | Infer default ProjectKind + Framework + Architecture |
| Language + ProjectKind             | Infer default Framework + Architecture               |
| Language + Framework               | Infer ProjectKind + Architecture                     |
| Language + ProjectKind + Framework | Infer Architecture                                   |
| Invalid combo                      | ❌ Reject (ask user)                                 |

---

## 7️⃣ Template Directory Mapping (final)

This table maps **1-to-1** with your filesystem.

```
templates/
└── rust/
    └── web_api/
        └── axum/
            ├── layered/
            │   └── rust_web_api_axum_layered/
            └── hexagonal/
                └── rust_web_api_axum_hexagonal/
```

General rule:

```
templates/{language}/{kind}/{framework}/{architecture}/{template_id}
```

Defaults live at higher levels:

```
templates/rust/_defaults/
templates/rust/web_api/_defaults/
templates/rust/web_api/axum/_defaults/
```

---

## 8️⃣ Why this structure scales

- Adding **new language** → add one column
- Adding **new framework** → one folder
- Adding **new architecture** → no explosion
- Matching logic becomes **data-driven**
- Template discovery becomes **O(depth)**, not search

---

## Final Insight (important)

> **Scarff is not choosing templates — it is resolving intent.**
> Templates are _artifacts_, Target is _truth_.

If you want next:

- convert this into YAML
- generate Rust enums + lookup tables
- design a scoring-based matcher instead of strict matching
- formalize “refuse & ask user” rules

Just say the word.

---

Yep — here’s a **clean, reproducible shell command setup** that creates the **scalable template hierarchy** we designed, without you losing your sanity later 😄

I’ll give you:

1. **Canonical directory shape**
2. **One-shot shell command (safe, idempotent)**
3. **How to extend it later without rewriting scripts**

---

## 1️⃣ Canonical Template Shape (recap)

```
templates/
└── {language}/
    ├── _defaults/
    └── {kind}/
        ├── _defaults/
        └── {framework}/
            ├── _defaults/
            └── {architecture}/
                └── {template_id}/
```

Example resolved path:

```
templates/rust/web_api/axum/layered/rust_web_api_axum_layered
```

---

## 2️⃣ One-shot Shell Command (mkdir -p)

### Rust – Web API – Axum – Layered + Hexagonal

```sh
mkdir -p templates/rust/{_defaults,web_api/{_defaults,axum/{_defaults,layered/{rust_web_api_axum_layered},hexagonal/{rust_web_api_axum_hexagonal}}}}
```

---

### TypeScript – Fullstack – Next.js – Component

```sh
mkdir -p templates/typescript/{_defaults,fullstack/{_defaults,nextjs/{_defaults,component/{ts_fullstack_nextjs_component}}}}
```

---

### Python – Web API – FastAPI – Layered

```sh
mkdir -p templates/python/{_defaults,web_api/{_defaults,fastapi/{_defaults,layered/{py_web_api_fastapi_layered}}}}
```

---

## 3️⃣ Create All Core Languages + Project Types (Base Skeleton)

This creates **empty slots** you can fill over time.

```sh
mkdir -p templates/{rust,python,typescript}/{_defaults,web_api,frontend,fullstack,cli,worker}/{_defaults}
```

---

## 4️⃣ Add Framework Layer Lazily (recommended)

Example: adding Axum later

```sh
mkdir -p templates/rust/web_api/axum/{_defaults,layered,hexagonal}
```

Example: adding React frontend

```sh
mkdir -p templates/typescript/frontend/react/{_defaults,component,modular}
```

---

## 5️⃣ Why this shell layout is correct

✔ matches your **Target resolution model**
✔ supports **default fallbacks**
✔ enables **auto-discovery** (`walkdir`)
✔ avoids template explosion
✔ future-proof for plugins / remote registries

This structure lets you do:

```text
Exact match → architecture
Fallback → framework/_defaults
Fallback → kind/_defaults
Fallback → language/_defaults
```

Which is _chef’s kiss_ for a CLI tool.

---

## 6️⃣ Optional: Generate from a Bash Script (dynamic)

If you want a reusable script later:

```sh
LANG=rust
PROJECT=web_api
FRAMEWORK=axum
ARCH=layered
ID=${LANG}_${PROJECT}_${FRAMEWORK}_${ARCH}

mkdir -p templates/$LANG/$PROJECT/$FRAMEWORK/$ARCH/$ID
```

---

If you want next, I can:

- write a **template loader that walks this tree**
- define **fallback resolution order in code**
- auto-generate `TemplateMatcher` from path
- generate this structure from a YAML spec

Just point 👉

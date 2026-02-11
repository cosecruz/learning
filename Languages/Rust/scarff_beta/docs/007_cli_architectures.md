For **Rust CLI tools**, here are the best architecture patterns that are compatible, idiomatic, and production-proven as of 2026. I'll rank them by suitability for CLI domains and explain when to use each:

---

## 1. **Layered Architecture** (Already implemented)

**Best for**: Traditional business logic CLIs, CRUD tools, data processors

- **Pros**: Clear separation, easy to test, familiar to most developers
- **Cons**: Can be overkill for simple scripts
- **When to use**: When you have complex domain rules that outlive the CLI interface

---

## 2. **Hexagonal Architecture (Ports & Adapters)**

**Best for**: CLIs that might evolve into services, need multiple UIs, or heavy external integrations

```
┌─────────────────────────────────────┐
│         Primary Adapter             │
│         (CLI with Clap)             │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         Application Core            │
│  (Use Cases, Domain Logic)          │
│  ┌─────────┐  ┌─────────┐          │
│  │  Port   │  │  Port   │          │
│  │  (Trait)│  │  (Trait)│          │
│  │Repository│  │Notifier │          │
│  └────┬────┘  └────┬────┘          │
└───────┼────────────┼────────────────┘
        │            │
   ┌────┴────┐  ┌────┴────┐
   │Secondary│  │Secondary│
   │ Adapter │  │ Adapter │
   │SQLite   │  │Webhook  │
   └─────────┘  └─────────┘
```

**Key difference from Layered**: Dependencies point **inward** via traits (ports). Core has zero external deps.

**When to choose over Layered**:

- You plan to add a TUI, web API, or GUI later
- Heavy external integrations (databases, APIs, message queues)
- Need to swap implementations (local SQLite → cloud API) without touching business logic

**Rust-specific traits**:

```rust
// Port (in core)
#[async_trait]
pub trait ForEntityRepository: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<Entity, Error>;
}

// Adapter (in infrastructure)
pub struct SqliteEntityRepository { ... }
pub struct RestApiEntityRepository { ... }  // Easy swap
```

---

## 3. **Command Pattern (CQRS-lite)**

**Best for**: Git-like CLIs with complex subcommands, undo/redo needs, or event sourcing

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   CLI Args  │────▶│   Command    │────▶│   Handler   │
│  (Clap)     │     │   (Enum)     │     │  (Executor) │
└─────────────┘     └──────────────┘     └──────┬──────┘
                                                │
                    ┌───────────────────────────┼───────────┐
                    ▼                           ▼           ▼
            ┌──────────────┐          ┌──────────────┐ ┌──────────┐
            │  CreateCmd   │          │  UpdateCmd   │ │ DeleteCmd│
            │  +validate() │          │  +validate() │ │+validate()│
            │  +execute()  │          │  +execute()  │ │+execute() │
            └──────────────┘          └──────────────┘ └──────────┘
```

**When to use**:

- Complex command hierarchies (`tool config set`, `tool config get`, `tool config unset`)
- Need command history, dry-run mode, or undo functionality
- Event sourcing (commands become events)
- CLI as API client (commands map 1:1 to API operations)

**Rust implementation**:

```rust
// Each command is a struct implementing a trait
#[async_trait]
trait Command: Send + Sync {
    fn validate(&self) -> Result<(), Error>;
    async fn execute(&self, ctx: &Context) -> Result<Output, Error>;
}

// Clap args convert to commands
impl From<CliArgs> for Box<dyn Command> {
    fn from(args: CliArgs) -> Self {
        match args.subcommand {
            Create(args) => Box::new(CreateCommand::new(args)),
            ...
        }
    }
}
```

---

## 4. **Plugin/Extension Architecture**

**Best for**: Extensible CLIs, dev tools, build systems, or platform ecosystems

```
┌─────────────────────────────────────────┐
│           Core CLI (Host)               │
│  ┌─────────┐  ┌─────────┐  ┌──────────┐ │
│  │Command  │  │ Plugin  │  │  Hook    │ │
│  │Registry │  │ Loader  │  │  System  │ │
│  └────┬────┘  └────┬────┘  └────┬─────┘ │
└───────┼────────────┼────────────┼───────┘
        │            │            │
   ┌────┴────┐  ┌────┴────┐  ┌────┴──────┐
   │Built-in │  │WASM     │  │External   │
   │Commands │  │Plugins  │  │Binaries   │
   │         │  │(Sandbox)│  │(Git, etc) │
   └─────────┘  └─────────┘  └───────────┘
```

**When to use**:

- Building the next `cargo`, `kubectl`, or `docker`
- Users need custom subcommands (`mycli custom-tool`)
- Security requirements (WASM sandboxing for user plugins)
- Hot-reloading extensions without restarting CLI

**Rust 2026 approach**:

- **WASM**: `wasmtime` for sandboxed plugins (safe, portable)
- **Dynamic linking**: `libloading` for native plugins (performance)
- **External binaries**: Convention-based discovery (`mycli-plugin-*` in PATH)

---

## 5. **Functional Core / Imperative Shell**

**Best for**: Data transformation tools, compilers, linters, parsers

```
┌──────────────────────────────────────────┐
│         Imperative Shell                 │
│  (I/O, CLI args, File system, Network)   │
│         ┌──────────────┐                 │
│         │  Effect System│                │
│         │  (IO/Result)  │                │
│         └──────┬───────┘                 │
└────────────────┼─────────────────────────┘
                 │
                 ▼ Pure Functions
┌──────────────────────────────────────────┐
│           Functional Core                │
│  (Parsing, Validation, Business Logic)   │
│                                          │
│  InputData → [Pure Transform] → Output   │
│  No side effects, no async, no mut       │
└──────────────────────────────────────────┘
```

**When to use**:

- Heavy data processing (CSV, JSON, log analysis)
- Compiler-like tools (parsing, AST manipulation)
- Need property-based testing (QuickCheck/proptest)
- Audit trails and reproducibility are critical

**Rust traits**:

- Core functions are `fn(Input) -> Result<Output, Error>` (no async, no IO)
- Shell handles all `async` and `std::fs` operations
- Easy to test: core is 100% deterministic

---

## 6. **Event-Driven / Actor Model**

**Best for**: Real-time CLIs, monitoring tools, chatbots, or async-heavy workflows

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   CLI Input │────▶│  Event Bus   │◄────│  File Watch │
│             │     │  (tokio::mpsc│     │  (notify)   │
└─────────────┘     └──────┬───────┘     └─────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        ▼                  ▼                  ▼
   ┌─────────┐       ┌─────────┐       ┌──────────┐
   │ Handler │       │ Handler │       │ Handler  │
   │ (Spawn) │       │ (Spawn) │       │ (Spawn)  │
   │ Actor   │       │ Actor   │       │ Actor    │
   └─────────┘       └─────────┘       └──────────┘
```

**When to use**:

- Long-running processes (`tail -f` style)
- Real-time data processing (logs, metrics)
- Interactive TUIs with background tasks
- Need backpressure and load shedding

**Rust implementation**:

- `tokio::sync::mpsc` or `tokio::sync::broadcast` for events
- `actix` or custom actors for handlers
- `crossterm` + `ratatui` for interactive UI

---

## Decision Matrix

| Architecture        | Complexity  | Testability | Flexibility | Best For                               |
| ------------------- | ----------- | ----------- | ----------- | -------------------------------------- |
| **Layered**         | Medium      | High        | Medium      | Business CRUD tools                    |
| **Hexagonal**       | Medium-High | Very High   | Very High   | Evolving products, multiple interfaces |
| **Command**         | Medium      | Very High   | High        | Git-like CLIs, undo/redo               |
| **Plugin**          | High        | Medium      | Very High   | Platform tools, ecosystems             |
| **Functional Core** | Low-Medium  | Very High   | Low         | Data processors, compilers             |
| **Event-Driven**    | High        | Medium      | High        | Real-time, interactive, long-running   |

---

## My Recommendations for Scarff

Given Scarff's purpose (scaffolding tool), I'd suggest:

1. **Default**: **Hexagonal** — Most future-proof, handles evolution well
2. **Simple scripts**: **Functional Core** — If generating simple file transformers
3. **Complex scaffolding**: **Command Pattern** — If supporting `scarff add`, `scarff remove`, `scarff migrate` with undo
4. **Extensible**: **Plugin** — If allowing community templates as WASM plugins

**Want me to generate the Hexagonal Architecture template next?** It's the most popular 2026 pattern for serious Rust CLIs and would complement your Layered template well.

---

\*Command** | Medium | Very High | High | Git-like CLIs, undo/redo |
| **Plugin** | High | Medium | Very High | Platform tools, ecosystems |
| **Functional Core** | Low-Medium | Very High | Low | Data processors, compilers |
| **Event-Driven\*\* | High | Medium | High | Real-time, interactive, long-running |

---

## My Recommendations for Scarff

Given Scarff's purpose (scaffolding tool), I'd suggest:

1. **Default**: **Hexagonal** — Most future-proof, handles evolution well
2. **Simple scripts**: **Functional Core** — If generating simple file transformers
3. **Complex scaffolding**: **Command Pattern** — If supporting `scarff add`, `scarff remove`, `scarff migrate` with undo
4. **Extensible**: **Plugin** — If allowing community templates as WASM plugins

**Want me to generate the Hexagonal Architecture template next?** It's the most popular 2026 pattern for serious Rust CLIs and would complement your Layered template well.

---

**No, but they're close cousins.** Hexagonal and Clean Architecture share DNA but have different focuses and constraints. Here's the breakdown:

---

## The Core Relationship

| Aspect           | Hexagonal (Ports & Adapters)                    | Clean Architecture                                     |
| ---------------- | ----------------------------------------------- | ------------------------------------------------------ |
| **Origin**       | Alistair Cockburn (2005)                        | Robert C. Martin (2012)                                |
| **Primary Goal** | Testability via dependency inversion            | Independence from frameworks, UI, database             |
| **Structure**    | Concentric layers with strict dependency rule   | Concentric layers with strict dependency rule          |
| **Key Concept**  | Ports (interfaces) & Adapters (implementations) | Entities → Use Cases → Interface Adapters → Frameworks |
| **Flexibility**  | More pragmatic, less prescriptive               | More rigid, explicit layer rules                       |

---

## Visual Comparison

### Hexagonal Architecture

```
         ┌─────────┐
         │   CLI   │◄────── External
         │ Adapter │
         └────┬────┘
              │
    ┌─────────▼──────────┐
    │    Application     │◄────── Core (no external deps)
    │    (Use Cases)     │
    │  ┌──────────────┐  │
    │  │    Domain    │  │
    │  │   (Entities) │  │
    │  └──────────────┘  │
    └─────────┬──────────┘
              │
         ┌────┴────┐
         │  Port   │ (Trait/Interface)
         └────┬────┘
              │
    ┌─────────▼──────────┐
    │  Database Adapter  │◄────── External
    │   (SQLite/Postgres)│
    └────────────────────┘
```

**Key**: Everything points inward. Core knows **nothing** about the outside world. Adapters implement ports defined by the core.

---

### Clean Architecture

```
┌─────────────────────────────────────┐
│      Frameworks & Drivers           │
│  (CLI, Web, External APIs, DB)      │
├─────────────────────────────────────┤
│      Interface Adapters             │
│  (Controllers, Presenters, Gateways)│
├─────────────────────────────────────┤
│      Application Business Rules     │
│  (Use Cases, Interactors)           │
├─────────────────────────────────────┤
│      Enterprise Business Rules      │
│  (Entities)                         │
└─────────────────────────────────────┘
         ▲
         │
    Dependencies point INWARD only
```

**Key**: Same inward dependency rule, but **explicit layer hierarchy** and emphasis on "Enterprise" vs "Application" business rules.

---

## Critical Differences

### 1. **Layer Definition**

- **Hexagonal**: Two main regions — _Inside_ (application + domain) and _Outside_ (adapters). Less ceremony about how many layers inside.
- **Clean**: Exactly four layers — Entities, Use Cases, Interface Adapters, Frameworks. Very specific.

### 2. **Cross-Cutting Concerns**

- **Hexagonal**: Easier to have multiple adapters of same type (CLI + API + TUI all driving the same core).
- **Clean**: More structured but can feel heavy for simple CLIs.

### 3. **Rust Implementation**

```rust
// HEXAGONAL: Port is just a trait in core
pub trait ForUserRepository: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<User, Error>;
}

// Infrastructure implements it
pub struct SqliteUserRepository { ... }
impl ForUserRepository for SqliteUserRepository { ... }

// CLEAN: More explicit layer separation
// entities/ (no deps)
// use_cases/ (depends on entities)
// interface_adapters/ (depends on use_cases, maps to external)
// frameworks/ (concrete implementations)
```

### 4. **Pragmatism vs. Purity**

- **Hexagonal**: "Make the core testable and swappable." Allows practical shortcuts if testability is preserved.
- **Clean**: "Follow the layers religiously." More boilerplate, clearer boundaries.

---

## When They're Effectively the Same

In Rust CLI projects, they often **converge** to the same code structure:

```
scarff-core/
├── domain/          # Entities (Clean) == Domain (Hexagonal)
├── application/     # Use Cases (Clean) == Application (Hexagonal)
└── ports/           # Interfaces/Traits (both)

scarff-infrastructure/
├── adapters/        # Adapters (Hexagonal) == Interface Adapters + Frameworks (Clean)
│   ├── cli/
│   ├── persistence/
│   └── ...
```

**The difference is mostly philosophical:**

- Hexagonal asks: "How do we test this without the database?"
- Clean asks: "How do we delay decisions about frameworks?"

---

## For Scarff Specifically

| Your Use Case                           | Recommendation                                        |
| --------------------------------------- | ----------------------------------------------------- |
| Simple CLI, one database, standard I/O  | **Hexagonal** — less boilerplate, same benefits       |
| CLI that might become a service/library | **Hexagonal** — easier to add HTTP adapter later      |
| Enterprise tool with complex policies   | **Clean** — clearer separation of business rule types |
| Multiple UIs (CLI + TUI + Web)          | **Hexagonal** — multiple primary adapters naturally   |

---

## Bottom Line

> **Hexagonal is "Clean Architecture light"** — same dependency inversion principle, less prescriptive structure, more focused on practical testability.

For a scaffolding tool like Scarff where you want **speed of development + future flexibility**, **Hexagonal wins**. You get 90% of Clean's benefits with 50% of the ceremony.

---

**Want me to generate the Hexagonal template now?** It will look similar to the Layered one but with:

- Explicit `ports/` module for traits
- Clearer separation between primary (driving) and secondary (driven) adapters
- Easier to swap SQLite for a cloud API later without touching core logic

- future flexibility**,**Hexagonal wins\*\*. You get 90% of Clean's benefits with 50% of the ceremony.

---

**Want me to generate the Hexagonal template now?** It will look similar to the Layered one but with:

- Explicit `ports/` module for traits
- Clearer separation between primary (driving) and secondary (driven) adapters
- Easier to swap SQLite for a cloud API later without touching core logic

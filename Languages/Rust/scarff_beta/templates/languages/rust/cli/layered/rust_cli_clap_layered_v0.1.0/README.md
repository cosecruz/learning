# {project_name}

A production-ready Rust CLI template with layered architecture.

## Quick Start

```bash
# Run with in-memory storage (development)
cargo run --bin {project_name} -- --memory create "My Entity"

# Run with SQLite persistence (production)
cargo run --bin {project_name} -- create "My Entity"

# List entities
cargo run --bin {project_name} -- list

# Get entity details
cargo run --bin {project_name} -- get &lt;uuid&gt;

# Archive entity
cargo run --bin {project_name} -- archive &lt;uuid&gt;
```

Architecture

    scarff-core: Domain logic and application use cases (pure Rust, no I/O)
    scarff-infrastructure: SQLite, config files, CLI output formatting
    scarff-cli: Thin binary, argument parsing, dependency wiring

Development

```bash
# Install just (task runner)
cargo install just

# Run tests
just test

# Lint
just lint

# Build release
just build
```

t build

```

```

Configuration
Configuration files (optional):

    ./scarff.toml (project-local)
    ~/.config/scarff/config.toml (user-global)

Environment variables: SCARFF_LOG_LEVEL, SCARFF_DATA_DIR, etc.

Environment variables: SCARFF_LOG_LEVEL, SCARFF_DATA_DIR, etc.

---

```plain
## Summary of 2026 Best Practices Applied

| Aspect | Implementation |
|--------|---------------|
| **Rust Edition** | 2024 (latest stable) |
| **Resolver** | Version 3 |
| **Async Runtime** | Tokio 1.43 with full features |
| **CLI Framework** | Clap 4.5 with derive macros |
| **Error Handling** | thiserror + anyhow + miette for UX |
| **Observability** | tracing with structured JSON support |
| **Database** | SQLx 0.8 with compile-time checked queries |
| **Security** | cargo-deny for supply chain auditing |
| **Task Runner** | just (modern make alternative) |
| **CI/CD** | GitHub Actions with matrix testing |
| **Architecture** | Strict layered: Domain → Application → Infrastructure |
| **Testing** | mockall for mocking, in-memory repos for unit tests |

---

**Next template to generate?** Suggest another combination (e.g., *"Go CLI with Cobra, clean architecture"* or *"Python API with FastAPI, hexagonal architecture"*)
```

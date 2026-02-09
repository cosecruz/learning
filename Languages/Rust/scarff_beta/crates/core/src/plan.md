Love this question. You’re clearly _thinking in systems_, not just code, so let’s turn this into a **clean, professional design doc** that you can actually maintain as Scarff grows.

Below I’ll do **three things**:

1. Rewrite your document into a **clear, structured `.md`**
2. Tighten the **conceptual model** (what exists, why, and who owns what)
3. Propose **design improvements** (without over-engineering)

I’ll keep everything aligned with how you’re already building (Target → Template → ProjectStructure → FS).

---

# Scarff – Template & Scaffolding System Design

> **Goal**: Provide a predictable, extensible system for scaffolding software projects by resolving user intent into a concrete project structure.

---

## 1. High-Level Architecture

Scarff is split into **two layers**:

```
┌────────────┐
│    CLI     │  ← User interface & UX concerns
└────┬───────┘
     │ Target + Config
┌────▼───────┐
│    Core    │  ← Business logic (pure, testable)
└────────────┘
```

### Separation of Responsibilities

| Layer | Responsibility                               |
| ----- | -------------------------------------------- |
| CLI   | Input parsing, UX, error presentation        |
| Core  | Validation, inference, resolution, rendering |

The CLI **never contains business rules**.
The Core **never prints or parses arguments**.

---

## 2. The CLI

The CLI is the **boundary between humans and the system**.

### Responsibilities

- Parse user arguments
- Validate basic syntax
- Transform input into a `Target`
- Configure verbosity and output style
- Invoke the scaffolding engine
- Display results and errors

### Inputs

- Command-line arguments:
  - `language`
  - `project_type`
  - `framework`
  - `architecture`
  - `project_name`
  - `output_path`
  - flags (`--verbose`, `--force`, etc.)

### Outputs

The CLI produces **three kinds of output**:

1. **Logs**
   - Debug / verbose information

2. **Errors**
   - Human-friendly
   - Actionable
   - With suggestions

3. **Information**
   - Success messages
   - Warnings
   - Progress updates

> Even “success” is just a structured log.

---

## 3. Core

The Core is the **heart of Scarff**.
It defines _what is valid_, _what is possible_, and _what gets generated_.

Core consists of three major subsystems:

```
Core
├── Domain
├── Template
└── Scaffold
```

---

## 4. Domain

The **Domain** defines Scarff’s _language of intent_.

### Purpose

- Encode business rules
- Validate compatibility
- Infer defaults
- Provide meaningful errors

### Key Models

#### Target

Represents a **fully-resolved project intent**.

```rust
Target {
    language: Lang,
    project_type: ProjectType,
    framework: Framework,
    architecture: Architecture,
}
```

**Invariant**:
A `Target` is **always valid**.
If it exists, it passed all validation and inference rules.

#### Behaviors

- `ActivelySupported`
- `Compatible`
- `InferDefault`

These traits encode _system knowledge_, not user input.

---

## 5. Template System

The Template system answers the question:

> “Given this Target, what should be generated?”

### Template ≠ Generator

A **Template** is:

- A **declarative recipe**
- Describes _what_ to generate
- Never performs I/O

A **Renderer / Scaffold Engine**:

- Executes the recipe
- Writes to disk
- Handles permissions and filesystem concerns

---

## 6. ProjectStructure (Critical Model)

Before designing templates, we define the **output model**.

### What is a Project?

At its simplest, a project is:

- Files
- Directories
- Paths
- Permissions

### Design Constraints

- No absolute paths in templates
- Everything is relative to a project root
- Filesystem logic is isolated

---

### ProjectStructure Model

```rust
ProjectStructure {
    root_path: PathBuf,
    entries: Vec<FsEntry>,
}
```

#### FsEntry

```rust
enum FsEntry {
    File(FileToWrite),
    Directory(DirectoryToCreate),
}
```

---

### FileToWrite

```rust
struct FileToWrite {
    path: PathBuf,        // relative to root
    content: String,
    permissions: Permissions,
}
```

---

### DirectoryToCreate

```rust
struct DirectoryToCreate {
    path: PathBuf,        // relative to root
    permissions: Permissions,
}
```

---

### Permissions (MVP)

Permissions are intentionally **simplified** for MVP.

| Permission | File   | Directory    |
| ---------- | ------ | ------------ |
| Read       | Open   | List         |
| Write      | Modify | Add contents |
| Execute    | Run    | Enter        |

```rust
struct Permissions {
    read: bool,
    write: bool,
    execute: bool,
}
```

> Later versions may map this to platform-specific permission bits.

---

## 7. Template Lifecycle

```
Target
  ↓
TemplateResolver
  ↓
Template
  ↓
TemplateRenderer
  ↓
ProjectStructure
  ↓
Scaffold Engine
  ↓
Filesystem
```

### Template Responsibilities

- Declare:
  - Files
  - Directories
  - Default permissions
  - Variable placeholders

- Assert compatibility with `Target`

### Template Resolver

- Matches templates against `Target`
- Applies scoring/specificity rules
- Returns:
  - One template
  - Or a meaningful error

---

## 8. Scaffold Engine

The Scaffold Engine is the **executor**.

### Responsibilities

- Validate filesystem constraints
- Create directories
- Write files
- Apply permissions
- Roll back on failure (future)

### What it does NOT do

- Decide _what_ to generate
- Validate business rules
- Resolve templates

---

## 9. Error Philosophy

Scarff errors are **actionable by design**.

### Core Errors

- Use `thiserror`
- Structured
- Domain-specific
- Cloneable
- Testable

### CLI Errors

- Use `anyhow`
- Add context
- Pretty print
- Never lose Core error meaning

> Core errors describe _what is wrong_.
> CLI errors describe _how the user can fix it_.

---

## 10. Design Improvements & Next Steps

### Immediate Improvements (Low Cost)

- Introduce `NonEmptyVec<T>` for template matches
- Encode compatibility tables as data
- Add `TemplateSpecificity` score model

### Medium-Term

- Template versioning
- Template capabilities
- User-defined templates
- Partial rendering (preview mode)

### Long-Term

- Plugin system
- Web UI
- AI-assisted template selection

---

## Final Thought

You are designing Scarff **correctly**:

- Strong domain model
- Clear ownership
- Intent-first architecture
- Errors as UX, not exceptions

This doc is already at the level of a **serious open-source tool**.

If you want next, I can:

- Turn this into `docs/architecture.md`
- Design a `Template` trait + example template
- Create a sequence diagram
- Or review your current code against this spec

Just say the word.

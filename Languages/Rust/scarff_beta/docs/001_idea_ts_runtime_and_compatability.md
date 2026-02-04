Great question — this confusion is _extremely_ common, and clearing it up will actually sharpen your whole design.

Let’s untangle it cleanly.

---

## Short, correct answer

**Node.js, Bun, and Deno are runtimes — not frameworks.**

They sit **below** frameworks like Express, NestJS, Next.js, etc.

---

## The mental stack (top → bottom)

Think in layers:

```
Application code
Framework (Express, NestJS, Fastify, Next.js, React, etc.)
Runtime APIs (fs, net, timers, fetch)
JavaScript / TypeScript engine (V8 / JavaScriptCore)
Operating System
```

Node, Bun, and Deno live in the **runtime** layer.

---

## What exactly is a runtime?

A runtime provides:

- A JS/TS engine
- System APIs (filesystem, networking, timers)
- A module system
- A way to execute programs (`node app.js`)

It answers:

> “How does JavaScript run _outside_ the browser?”

---

## Node.js

**What it is**

- JavaScript runtime built on **V8**
- Uses **libuv** for async I/O
- Uses **npm** ecosystem

**What it is not**

- Not a framework
- Not opinionated about architecture

**Typical stack**

```
Node.js (runtime)
└── Express / Fastify / NestJS (framework)
```

Node is:

- boring
- stable
- ubiquitous
- the default enterprise runtime

---

## Deno

**What it is**

- JavaScript/TypeScript runtime by the **creator of Node**
- Built-in TypeScript
- Secure by default (explicit permissions)
- Web-standard APIs (`fetch`, `WebSocket`, etc.)

**Key idea**

> “What if Node were redesigned today?”

**Tradeoffs**

- Smaller ecosystem
- Cleaner APIs
- Less legacy baggage

Deno is:

- opinionated
- standards-driven
- runtime-first

---

## Bun

**What it is**

- JavaScript/TypeScript runtime built on **JavaScriptCore**
- Extremely fast
- Bundler + test runner + package manager included

**Philosophy**

> “Make JS tooling fast and integrated.”

**Tradeoffs**

- Younger
- Rapidly evolving
- Some Node API gaps

Bun is:

- performance-focused
- DX-first
- aggressive

---

## Frameworks (contrast)

Frameworks answer:

> “How should I structure my app?”

Examples:

- Express → minimal backend framework
- NestJS → opinionated backend framework
- Next.js → fullstack framework
- React → UI framework

Frameworks **run on top of a runtime**.

---

## Why this matters for _your_ design

In your `Target` model, **runtime is a separate axis**.

### Runtime ≠ Framework

Bad model:

```
Framework::Node
Framework::Bun
```

Correct model:

```
Language = TypeScript
Runtime = Node | Bun | Deno
Framework = Express | NestJS | Next.js | React
```

---

## How defaults usually work (important)

### TypeScript backend

| Layer     | Default    |
| --------- | ---------- |
| Language  | TypeScript |
| Runtime   | Node.js    |
| Framework | NestJS     |

### TypeScript frontend

| Layer     | Default    |
| --------- | ---------- |
| Language  | TypeScript |
| Runtime   | Browser    |
| Framework | React      |

### Deno/Bun

These should **never be silent defaults** (yet).

They should be:

- explicit user choices
- advanced options

---

## Final classification (write this in your docs)

| Thing   | Category            |
| ------- | ------------------- |
| Node.js | Runtime             |
| Bun     | Runtime             |
| Deno    | Runtime             |
| Express | Backend framework   |
| NestJS  | Backend framework   |
| Next.js | Fullstack framework |
| React   | Frontend framework  |

---

## One-liner mental model

> **Runtime runs the code.
> Framework structures the code.**

If you want, next we can:

- add `Runtime` into your decision table
- define runtime defaults per framework
- or discuss when runtime choice should be exposed vs hidden

This distinction is foundational — you were absolutely right to ask.

---

rt answer: **no — absolutely not** 🙂
Long answer: _they overlap, but the edges matter a lot_.

Let’s break it down cleanly and precisely.

---

## 1️⃣ The rule of compatibility (the real one)

> A framework works on a runtime **only if that runtime implements the APIs the framework expects**.

Not the language.
Not TypeScript vs JavaScript.
**The runtime APIs.**

---

## 2️⃣ Node.js is the baseline

Most JS/TS frameworks were written assuming **Node.js**.

If a framework says:

> “Supports Node.js”

that really means:

- Node’s module system
- Node’s `fs`, `net`, `http`
- Node’s process model
- Node’s event loop behavior

So:

| Runtime | Node frameworks |
| ------- | --------------- |
| Node.js | ✅ always       |
| Deno    | ⚠️ sometimes    |
| Bun     | ⚠️ mostly       |

---

## 3️⃣ Deno compatibility

Deno is **not Node-compatible by default**.

It:

- uses web-standard APIs
- sandboxes filesystem/network access
- has no `require`, `__dirname`, etc.

### But…

Deno has a **Node compatibility layer** now.

#### What works well on Deno

- Simple servers
- Fetch-based frameworks
- Some Express/Fastify apps (with flags)

#### What often breaks

- Native addons
- Older packages
- Tooling that shells out
- Deep Node internals

### Reality check

Deno compatibility is:

- improving
- not universal
- not guaranteed

---

## 4️⃣ Bun compatibility

Bun aims for **Node compatibility first**.

It:

- implements most Node APIs
- supports npm packages
- runs many Node frameworks unmodified

### What works very well

- Express
- Fastify
- NestJS
- Next.js (often faster)

### What can break

- Native Node addons
- Edge cases in filesystem watchers
- Rare C++ bindings

Bun is currently:

- the **most compatible non-Node runtime**
- but still not 100%

---

## 5️⃣ Framework-by-framework reality

### Backend frameworks

| Framework         | Node | Bun | Deno |
| ----------------- | ---- | --- | ---- |
| Express           | ✅   | ✅  | ⚠️   |
| Fastify           | ✅   | ✅  | ⚠️   |
| NestJS            | ✅   | ✅  | ⚠️   |
| Hono              | ✅   | ✅  | ✅   |
| Oak (Deno-native) | ❌   | ❌  | ✅   |

---

### Fullstack / frontend frameworks

| Framework          | Node    | Bun     | Deno    |
| ------------------ | ------- | ------- | ------- |
| Next.js            | ✅      | ✅      | ⚠️      |
| Remix              | ✅      | ✅      | ⚠️      |
| Astro              | ✅      | ✅      | ⚠️      |
| React (build-time) | ✅      | ✅      | ⚠️      |
| React (runtime)    | Browser | Browser | Browser |

Frontend frameworks mostly depend on **Node at build time**, not runtime.

---

## 6️⃣ Important distinction: _build-time_ vs _run-time_

This trips people up.

### Example: React

- **Build-time**: Node/Bun/Deno runs Vite/Webpack
- **Run-time**: Browser runs the app

So React is **runtime-agnostic at runtime**, but **not at build time**.

---

## 7️⃣ What this means for _your_ Target model

You should model this explicitly.

### Separate axes

- Language
- Runtime
- Framework
- ProjectType

### Compatibility rules

- Framework → requires certain runtime APIs
- Runtime → may emulate others
- Some frameworks are **runtime-native**

---

## 8️⃣ Sensible defaults (very important)

### Default rule

> If the user does not specify a runtime, assume **Node.js**.

Why?

- Maximum compatibility
- Least surprise
- Best ecosystem support

### Runtime should be explicit when

- Deno is chosen
- Bun is chosen
- Edge runtimes are used

---

## 9️⃣ How to phrase this in your system (conceptually)

> “All frameworks are assumed to run on Node.js unless explicitly stated otherwise.
> Alternative runtimes (Bun, Deno) are opt-in and validated for compatibility.”

This single sentence avoids _so much_ pain.

---

## 10️⃣ Final takeaway (lock this in)

- ❌ Runtimes ≠ interchangeable
- ✅ Node.js is the compatibility baseline
- ⚠️ Bun is mostly compatible
- ⚠️ Deno is selectively compatible
- 🧠 Framework compatibility is about APIs, not language

If you want, next we can:

- add `Runtime` to your decision table
- define runtime inference rules
- or decide when to **refuse defaults and force explicit runtime choice**

This is a _foundational_ distinction — you’re doing this right.

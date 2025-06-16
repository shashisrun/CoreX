# CoreX Local Agent Development Notes

This file is meant to help your local AI agent (e.g., Codex, GPT-based assistant) write, debug, and extend the CoreX framework.

---

## ✅ Project Goals
- TypeScript & AssemblyScript only framework.
- Rust + Tokio as the webserver.
- Deno as the runtime for TS.
- AssemblyScript compiled to Rust or WebAssembly, accessed directly.
- Extremely fast, async-first, native execution model.

---

## ✅ Tech Stack
- **Rust** (Tokio, Hyper) - Native HTTP server.
- **Deno Core** - TS execution.
- **AssemblyScript** - For performance-critical routes.
- **Serde/Anyhow** - JSON and error handling.

---

## ✅ Core Concepts

### 1. API Routing:
- Uses folder structure (`routes/`) with `.ts` and `.as.ts` files.
- Decorators (like FastAPI) register routes, validate requests, and generate docs.

### 2. Server
- `Tokio` + `Hyper` handles routing.
- Each request spins up a new isolated Deno runtime in `spawn_blocking`.
- AssemblyScript functions are compiled to Rust or WASM.

### 3. Memory Sharing
- TS <-> AS interop via function references and shared memory.
- Store `global state` (like DB connections) in `Rust`, pass references to both runtimes.

### 4. Tooling
- Custom CLI: `corex dev`, `corex build`, `corex routes`
- Codegen to create route bindings and decorators.

---

## ✅ Todo for Agent

### 🔧 Compiler Integration
- [ ] Transpile `.ts` files to Deno.
- [ ] Transpile `.as.ts` to Rust/WASM.
- [ ] Generate route bindings for each function.

### ⚙️ Runtime Setup
- [x] Basic Hyper server.
- [x] Tokio runtime.
- [x] Deno Core integration with per-request isolation.
- [ ] Decorator parsing.

### 🔍 Optimization
- [ ] Replace Deno's core features (`fetch`, `console`) with Rust equivalents.
- [ ] Shared in-memory store via `Arc<RwLock>` or custom allocator.
- [ ] AOT compilation for routes.

---

## ✅ Example TS Handler
```ts
// routes/hello.ts
@route("/hello", "GET")
@validate({ query: { name: "string" } })
export function handler(req) {
  return `Hello, ${req.query.name}`;
}
```

## ✅ Example AS Function
```ts
// routes/add.as.ts
@route("/add", "POST")
@types({ a: "i32", b: "i32" })
export function add(a: i32, b: i32): i32 {
  return a + b;
}
```

---

## ✨ Goals
- Fastest possible full-stack API runtime.
- Developer-friendly DX like Next.js + FastAPI.
- Scalable via WASM-native and multi-core Rust webservers.

---

> Update this file as your AI agent adds new capabilities, compilers, bindings, or decorators.

---

## 🧠 Start Agent With:
```bash
corex init --with ts as --server rust --runtime deno --decorate
```

Then implement CLI scaffolding + bootstrap routes.

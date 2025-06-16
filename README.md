
```markdown
# CoreX 🚀

**CoreX** is a blazing-fast, developer-first API framework that lets you write your application logic in **TypeScript** and **AssemblyScript**, while compiling and executing it on a high-performance **Rust-native runtime**.

Built for edge APIs, serverless infrastructure, and real-world DX, CoreX fuses the **developer ergonomics of TypeScript** with the **performance and safety of Rust** — all without requiring users to write a single line of Rust.

---

## ⚙️ Core Philosophy

**You write only this:**
- ✍️ `TypeScript` for routes, validation, and business logic
- ⚡ `AssemblyScript` for fast, compute-heavy functions

**CoreX handles this:**
- ⚙️ Route handling and decorators
- 🧠 Shared memory + global constants
- 🧱 AS → Rust transpilation and compilation
- 🚀 Native HTTP server with Tokio
- 🔄 Deno ↔ Rust ↔ AS function calls

---

## 🛠 Tech Stack

| Layer        | Technology         | Purpose                                |
|--------------|--------------------|----------------------------------------|
| Application  | **TypeScript (Deno)** | API routes, decorators, validations     |
| Plugin Logic | **AssemblyScript** | Fast functions, custom business logic   |
| Runtime Core | **Rust + Tokio**   | Web server, router, registry, memory    |
| Bridge       | **Deno Core Embed**| Execute TS code from Rust, or vice versa|

---

## 📁 Project Structure

```

corex/
├── runtime/             # Rust runtime: router, registry, deno embed
├── routes/
│   ├── ts/              # User-written TS API handlers
│   └── as/              # User-written AS modules (transpiled to Rust)
├── decorators/          # Decorator parser (ts-morph or Babel)
├── shared/              # Shared constants, types, and memory
└── README.md

````

---

## 🧪 Example Usage

```ts
// routes/ts/greet.ts
@sharedConst("GREETING")
export const GREETING = "Hello";

@exposeToRust()
export function greet(name: string) {
  return `${GREETING}, ${name}`;
}
````

```ts
// routes/as/math.as.ts (AssemblyScript subset)
@exposeToTS()
export function add(a: i32, b: i32): i32 {
  return a + b;
}
```

```rust
// Generated from AS transpiler → routes/native/math.rs
#[expose_to_ts]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 🧠 Features

* ✅ Zero-config decorators: `@get`, `@validate`, `@post`
* ✅ Deno V8 snapshotting for fast cold starts
* ✅ Shared memory registry between Deno ↔ Rust
* ✅ Compile AssemblyScript → Rust → native code
* ✅ TS ↔ AS ↔ Rust function bridge
* ✅ Typed validation and input schemas from TS

---

## 🚀 Getting Started

```bash
git clone https://github.com/yourname/corex.git
cd corex

# Install Deno
deno task dev        # Starts Deno + Rust runtime

# Compile runtime core
cargo build --release

# Scaffold new route
corex create route hello --lang ts
corex create route compute --lang as
```

---

## 🧩 Roadmap

* [ ] Rust native runtime with TS decorators
* [ ] Shared constants between TS and Rust
* [ ] AssemblyScript-to-Rust transpiler
* [ ] WASM plugin fallback for AS
* [ ] CLI: `corex build`, `corex dev`, `corex new`
* [ ] OpenAPI generator from TS metadata
* [ ] Hot reloading and live compiler
* [ ] Auth, cookies, and session middleware
* [ ] Built-in DB connectors

---

## 📜 License

MIT © 2025 CoreX Contributors

---

## 🙌 Contribute

If you're excited about the future of fast, typed APIs with native execution — join us!

PRs welcome. Reach out on [GitHub Discussions](https://github.com/shashisrun/CoreX/discussions) or [open an issue](https://github.com/shashisrun/CoreX/issues).

```

---

Let me know if you'd like:
- The `corex dev` and `corex build` CLI scaffolds
- A logo or branding assets
- GitHub Actions CI/CD workflow
- Domain name availability checks (e.g. `corex.dev`, `usecorex.com`)

Happy to keep iterating with you!
```

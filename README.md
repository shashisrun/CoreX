# CoreX 🚀

**CoreX** is a blazing-fast hybrid API framework that fuses the power of **TypeScript (Deno)**, **Rust**, and **AssemblyScript** into a single, ultra-performant runtime. Built for scale, safety, and developer delight — CoreX helps you write APIs with TypeScript while compiling performance-critical logic to native binaries.

---

## ⚡ Why CoreX?

- 🔥 **Native Speed** — Compile AssemblyScript or Rust directly to machine code
- 🧵 **Cross-Runtime Memory** — Seamlessly share constants, memory, and logic across TS ↔ Rust
- 🎯 **Zero-Cost Abstractions** — Decorators auto-generate bindings, validators, and routing
- 🧩 **Composable Design** — Use only what you need, from edge APIs to full backend systems
- 🧪 **Developer Experience** — Inspired by FastAPI, with decorators and type safety out of the box

---

## 🛠 Tech Stack

| Layer         | Tech                  | Purpose                      |
|--------------|-----------------------|------------------------------|
| Application  | **TypeScript (Deno)** | Developer-friendly API logic |
| Native Core  | **Rust**              | Native speed, registry, ops  |
| Plugins      | **AssemblyScript**    | Safe, fast custom logic      |
| Interop      | Decorators & Registry | TS ↔ Rust ↔ AS communication  |

---

## 📁 Project Structure

corex/
├── runtime/ # Rust runtime (function registry + Deno embed)
├── routes/
│ ├── ts/ # API routes written in TypeScript
│ └── native/ # Native Rust routes or AS-transpiled
├── decorators/ # Metadata extractor for TS decorators
├── shared/ # Shared constants, types, and memory
└── README.md


---

## 🧪 Quick Start

```bash
git clone https://github.com/shashisrun/CoreX.git
cd corex
deno task dev        # Starts the Deno + Rust server
cargo build --release # Compile Rust core

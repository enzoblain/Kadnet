# Kadnet

[![Rust](https://img.shields.io/badge/Rust-1.91-orange?logo=rust)](https://www.rust-lang.org/)
[![Workspace](https://img.shields.io/badge/Cargo-Workspace-blue)]()
[![Docs](https://img.shields.io/badge/docs-docs.rs-blue)](https://docs.rs/kadnet-sdk)
[![Crates.io](https://img.shields.io/crates/v/kadnet)](https://crates.io/crates/kadnet-sdk)
[![License](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc/4.0/)

---

**Kadnet** is a **Rust workspace** implementing a modern, modular version of the  
**Kademlia P2P Distributed Hash Table (DHT)** — built for **speed**, **safety**, and **scalability**.

The workspace is organized into three crates:

- **`core/`** — low-level engine (routing table, XOR distance, protocol, networking)  
- **`sdk/`** — high-level developer-friendly API  
- **`client/`** — CLI / example node using the SDK  

Kadnet aims to be a **production-ready foundation** for decentralized networks,  
distributed storage, and P2P communication protocols.

---

## 🧭 Project Status

🚧 **Work in Progress**  
- Core networking and routing table logic are under active development.  
- Feedback, design discussions, and contributions are **highly encouraged**!

---

## ✨ Key Features

- 🦀 **Pure Rust** — idiomatic, memory-safe, dependency-light  
- 🌐 **P2P Networking** — Kademlia node discovery & message routing  
- ⚡ **SIMD Optimization** — accelerated XOR-distance calculations  
- 🧵 **Optional Parallelism** — multi-core accelerated search  
- 🧪 **Extensive Testing** — unit, integration, and property-based tests  
- 📚 **Well-Documented Architecture** — high-level docs in `docs/`  

---

## 📦 Installation

Kadnet is a *multi-crate workspace*.  
Developers will typically depend on the **SDK crate**:

``` toml
[dependencies]
kadnet-sdk = { path = "sdk" }
```

Once published on crates.io:

``` toml
[dependencies]
kadnet-sdk = "0.1"
```

Optional features:

``` toml
kadnet-sdk = { version = "0.1", features = ["parallel"] }
```

The low-level `kadnet-core` crate is **not intended** for direct use  
(except for advanced internal integrations).

---

## 📂 Workspace Structure

```
kadnet/
│
├── core/        # Low-level Kademlia engine
│── sdk/         # Developer-facing API
└── client/      # CLI / example node
```

---

## 🧪 Examples (planned)

Examples will be provided under `sdk/examples/`.

| Example          | Description                                         |
|------------------|-----------------------------------------------------|
| `basic.rs`       | Minimal DHT example via the SDK                     |
| `network.rs`     | Peer discovery & async networking                   |
| `parallel.rs`    | SIMD + parallel accelerated lookups                 |

Run examples:

``` bash
cargo run -p sdk --example <example_name>
```

Or run the client directly:

``` bash
cargo run -p kadnet-client
```

---

## 📖 Documentation

- High-level documentation: `docs/`  
- API docs (once published): **docs.rs/kadnet-sdk**  
- Internal architecture notes: `core/docs/`

---

## 🤝 Contributing

Contributions welcome — especially regarding the core protocol, 
async networking, performance, and SDK design.

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md).

Workflow:

``` bash
cargo fmt
cargo clippy
cargo test --workspace
```

---

## 📄 License

Licensed under **CC BY-NC 4.0**.  
See [`LICENSE`](LICENSE).

---

## 📬 Contact / Support

Questions or feedback?

**Discord:** enzoblain  
**Email:** [enzoblain@proton.me](enzoblain@proton.me)

Please include context for faster replies.
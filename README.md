# Kadnet

[![Rust](https://img.shields.io/badge/Rust-1.91-orange?logo=rust)](https://www.rust-lang.org/)
[![Workspace](https://img.shields.io/badge/Cargo-Workspace-blue)]()
[![Crates.io](https://img.shields.io/crates/v/kadnet-sdk)](https://crates.io/crates/kadnet-sdk)
[![Docs](https://img.shields.io/badge/docs.rs-kadnet--sdk-green)](https://docs.rs/kadnet-sdk)
[![License](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](LICENSE)

---

**Kadnet** is a modern, efficient, and modular implementation of the  
**Kademlia Distributed Hash Table (DHT)** written in **Rust**.  
It serves as the **peer-to-peer networking backbone** of *Nebula*,  
a broader decentralized ecosystem designed to restore **sovereignty**,  
**data ownership**, and **digital autonomy** to users.

Kadnet provides reliable peer discovery, deterministic routing, and  
a communication layer capable of supporting large-scale decentralized  
applications and infrastructures.

---

# 🌌 Role in the Nebula Ecosystem

Nebula is an ambitious initiative aiming to build a **fully decentralized global platform**,  
serving as an open alternative to large integrated digital ecosystems.

Within this architecture, Kadnet acts as the **foundational networking layer**:

- It enables nodes to find each other autonomously.  
- It routes messages efficiently across the distributed topology.  
- It ensures the network remains resilient without central servers.  
- It provides the communication substrate used by higher Nebula layers  
  such as storage (CosmaDB), blockchain consensus, and the decentralized app store.

Kadnet is therefore not just a standalone DHT — it is the **entry point**  
through which every Nebula service communicates.

---

## ✨ Key Features

- 🦀 **Pure Rust Implementation**  
  Safe, efficient, memory-controlled, and dependency-light.

- ⚡ **Kademlia-Based Peer Routing**  
  Scalable XOR-based lookup, optimized for distributed environments.

- 🧩 **Modular Workspace Architecture**  
  Clean separation between the low-level engine, a high-level SDK,  
  and supporting cryptographic utilities.

- 🔐 **Integrated Cryptography Layer**  
  Through the `cryptography/` crate, Kadnet includes primitives like  
  **SHA-256**, **U256**, and additional building blocks used by Nebula.

- 🧱 **Designed for Larger Distributed Systems**  
  Kadnet is built to serve as the backbone of Nebula’s:  
  - distributed storage layer (CosmaDB),  
  - hybrid blockchain consensus,  
  - decentralized application store,  
  - user-to-user secure communication tools.

- 🧪 **Testability First**  
  Structure prepared for unit, integration, and property-based testing.

---

## 🧭 Project Status

🚧 **Active Development**  
Kadnet is currently being implemented as a foundational component of Nebula.

Work in progress includes:

- XOR distance metric & routing table  
- Routing bucket management  
- Query engine (FIND_NODE, FIND_VALUE, STORE)  
- Async networking and message transport  
- Node identity & serialization format  
- High-level SDK architecture  

Feedback and contributions are highly encouraged.

---

## 📦 Installation

Kadnet is structured as a multi-crate Rust workspace.

During development:

```toml
[dependencies]
kadnet-sdk = { path = "sdk" }
```

Once published:

```toml
[dependencies]
kadnet-sdk = "0.1"
```

`kadnet-core` is intended for advanced/internal use only.

---

## 📂 Workspace Structure

```
kadnet/
│
├── core/           # Kademlia engine (routing, queries, RPC, distance metric)
├── sdk/            # Developer-facing API for applications
├── client/         # CLI example node / testing environment
└── cryptography/   # SHA-256, U256, and shared cryptographic utilities
```

---

## 🤝 Contributing

Contributions are welcome — especially regarding:

- routing performance & correctness  
- async networking design  
- serialization formats  
- SDK ergonomics  
- testing infrastructure  

Standard workflow:

```bash
cargo fmt
cargo clippy
cargo test --workspace
```

Check [`CONTRIBUTING.md`](CONTRIBUTING.md) for details.

---

## 📄 License Philosophy

Kadnet is licensed under the **Server Side Public License (SSPL) v1**.

This license is intentionally chosen to protect the integrity of the Nebula ecosystem.  
While the project is fully open for **contribution, improvement, and transparency**,  
SSPL prevents third parties from creating competing platforms, proprietary versions,  
or commercial services derived from the project.

Nebula is designed to grow as **one unified, community-driven network**.  
By using SSPL, we ensure that:

- all improvements remain open and benefit the ecosystem,  
- the network does not fragment into multiple incompatible forks,  
- companies cannot exploit the project without contributing back,  
- contributors retain full access to the entire codebase.

In short, SSPL ensures that Kadnet — and the Nebula ecosystem built on top of it —  
remains **open to the community, but protected from fragmentation and exploitation**.

---

## 📬 Contact

**Discord:** enzoblain  
**Email:** enzoblain@proton.me  

Feel free to reach out with technical questions or design discussions.
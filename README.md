# Kadnet

[![Rust](https://img.shields.io/badge/Rust-1.91-orange?logo=rust)](https://www.rust-lang.org/)
[![Docs](https://img.shields.io/badge/docs-docs.rs-blue)](https://docs.rs/kadnet)
[![Crates.io](https://img.shields.io/crates/v/kadnet)](https://crates.io/crates/kadnet)
[![License](https://img.shields.io/badge/License-CC%20BY--NC%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc/4.0/)

---

**Kadnet** is a **pure-Rust implementation** of the **Kademlia P2P Distributed Hash Table (DHT)** — built for **speed**, **safety**, and **scalability**.  
It combines asynchronous networking with SIMD-optimized and optionally parallel routing operations to deliver fast and efficient peer discovery in decentralized systems.

Kadnet aims to be a **modern, production-ready foundation** for distributed networks, decentralized storage, and peer-to-peer communication protocols.

---

### 🧭 Project Status

🚧 **Work in Progress**  
- Core networking and routing table logic are under active development.  
- Feedback, design discussions, and contributions are **highly encouraged**!

---

### ✨ Key Features

- 🦀 **Pure Rust** — idiomatic, memory-safe, and dependency-light.  
- 🌐 **P2P Networking** — peer discovery, node lookup, and message routing via Kademlia.  
- ⚡ **SIMD Optimization** — accelerate hash and XOR distance calculations.  
- 🧵 **Parallel Processing (optional)** — leverage multicore systems for large-scale DHT operations.  
- ✅ **Comprehensive Testing** — unit, integration, and property-based coverage.  
- 📚 **Well-Documented** — clean examples and guides on [docs.rs](https://docs.rs/kadnet).  

---

### 🚀 Installation (coming soon)

Once published, add **Kadnet** to your `Cargo.toml`:

``` toml
[dependencies]
kadnet = "0.1"
```

To enable optional parallelism:

``` toml
kadnet = { version = "0.1", features = ["parallel"] }
```

---

### 📂 Examples (planned)

Examples will be added under `examples/` as the implementation matures:

| Example         | Description                                         |
|------------------|-----------------------------------------------------|
| `basic.rs`       | Minimal Kademlia DHT example.                      |
| `network.rs`     | Demonstrates peer discovery and async networking.  |
| `parallel.rs`    | Shows SIMD and parallel processing in lookups.     |

Run examples via:

``` bash
cargo run --example <example_name>
```

---

### 📖 Documentation

Full API documentation will be published on **[docs.rs/kadnet](https://docs.rs/kadnet)**.  
Early design notes, protocol details, and architecture discussions can be found in the `docs/` directory.

---

### 🤝 Contributing

Contributions are welcome — especially on network design, async implementation, and performance improvements.  
Please read the [**CONTRIBUTING.md**](CONTRIBUTING.md) file for detailed contribution guidelines.

1. **Fork** the repo  
2. Create a new branch: `git checkout -b my-feature`  
3. Add your code or tests  
4. Run checks:  
   ``` bash
   cargo fmt
   cargo clippy
   cargo test
   ```  
5. Open a **pull request** with a clear summary of your changes

---

### 📄 License

Licensed under the **CC BY-NC 4.0 License**.  
See the [`LICENSE`](LICENSE) file for details.

---

### 📬 Contact / Support

Questions, suggestions, or discussions welcome:  
**Discord:** [enzoblain](https://discord.com/)  
**Mail:** enzoblain@proton.me

Please include a short context or topic so we can respond effectively.

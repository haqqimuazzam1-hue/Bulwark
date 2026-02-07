🛡️ Bulwark

A lightweight, extensible security middleware for Rust APIs.

Bulwark helps you inspect, validate, and make decisions about incoming requests before they reach your application logic. It is designed to be simple, composable, and framework-agnostic.

> ⚠️ Status: Early-stage (pre-1.0). APIs may change.




---

✨ Features

🔍 Request inspection via pluggable inspectors

🧠 Central decision engine

🦀 Written in pure Rust, no unsafe code

⚡ Lightweight and minimal dependencies

🧩 Easy to extend with custom rules



---

📦 Installation

> Bulwark is not published on crates.io yet. Planned release: v1.0.0.



For now, you can depend on Bulwark directly from GitHub:
```Toml
[dependencies]
bulwark = { git = "https://github.com/haqqimuazzam1-hue/Bulwark" }
```

---

🚀 Quick Example
```Rust
use bulwark::request::context::{RequestContext, Method};

fn main() {
    let ctx = RequestContext::new(Method::POST, "/login");

    println!("Method: {:?}", ctx.method);
    println!("Path: {}", ctx.path);
}
```

---

🧠 Core Concepts

RequestContext

Represents an incoming request with basic metadata such as:

HTTP method

Request path


Inspector

Inspectors analyze a request and provide signals to the decision engine.

DecisionEngine

The central engine that evaluates all inspectors and produces a final decision.


---

🧪 Development

Run formatting:
```bash
cargo fmt
```
Run lint checks:
```bash
cargo clippy
```
Run tests:
```bash
cargo test
```

---

🤝 Contributing

Contributions are welcome! 🎉

Bug reports

Feature requests

Documentation improvements

Code contributions


Please read CONTRIBUTING.md before submitting a pull request.


---

🗺️ Roadmap

[ ] Stable public API

[ ] More built-in inspectors

[ ] Better documentation & examples

[ ] Publish to crates.io (v1.0.0)



---

📄 License

MIT License © 2026 Bulwark

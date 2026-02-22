# Interactive CLI Business Card

A fast, interactive CLI business card implemented in Rust.

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)
![uvx](https://img.shields.io/badge/uvx-entrypoint-green.svg)
![bunx](https://img.shields.io/badge/bunx-entrypoint-green.svg)

## 🚀 Quick Start

### Cargo (direct Rust install)

```bash
cargo install carlosferreyra
carlosferreyra
```

### uvx (wrapper entrypoint)

```bash
uvx carlosferreyra
```

### bunx (wrapper entrypoint)

```bash
bunx carlosferreyra
```

## Features

- ⚡ Fast Rust CLI
- 📧 Direct email contact
- 🌐 Portfolio and social links
- 🖥️ Interactive menu
- 🔧 Non-interactive mode via `--open`
- 🎯 Config-driven behavior from a single `config.json`

## Project Structure

```text
business-card/
├── src/                  # Rust source code
│   └── main.rs
├── Cargo.toml            # Rust package manifest
├── Cargo.lock
├── config.json           # Centralized configuration
├── config.schema.json    # JSON Schema validation
├── CONFIGURATION.md      # Configuration docs
└── README.md
```

## Configuration

The CLI reads settings from root `config.json`:

- Personal information (name, title, company, location, skills)
- URLs (email, resume, portfolio, github, linkedin, twitter)
- Theme settings

For full configuration details, see [CONFIGURATION.md](CONFIGURATION.md).

## Distribution Model

- **Source code in this repo:** Rust only
- **`uvx carlosferreyra`:** Python ecosystem entrypoint wrapper (generated/published in release
  pipeline)
- **`bunx carlosferreyra`:** Node ecosystem entrypoint wrapper (generated/published in release
  pipeline)

The `uvx` and `bunx` packages are distribution entrypoints, not source implementations in this
repository.

## Development

```bash
cargo run
cargo run -- --open portfolio
cargo check
cargo build --release
```

## Connect with Carlos

- **GitHub**: [github.com/carlosferreyra](https://github.com/carlosferreyra)
- **LinkedIn**: [linkedin.com/in/eduferreyraok](https://linkedin.com/in/eduferreyraok)
- **Website**: [carlosferreyra.com.ar](https://carlosferreyra.com.ar)
- **Email**: [eduferreyraok@gmail.com](mailto:eduferreyraok@gmail.com)

## License

MIT

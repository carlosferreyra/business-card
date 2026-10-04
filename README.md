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
- 🎯 Resume-driven behavior from `resume.json` with runtime refresh from GitHub

## Project Structure

```text
business-card/
├── src/                  # Rust source code
│   └── main.rs
├── Cargo.toml            # Rust package manifest
├── Cargo.lock
├── resume.json           # Embedded fallback resume snapshot
└── README.md
```

## Configuration

The CLI fetches resume metadata at startup from:

```text
https://raw.githubusercontent.com/carlosferreyra/carlosferreyra/main/resume.json
```

If the fetch fails, times out, or returns invalid data, the CLI falls back to the embedded root
`resume.json` in this repository.

- Personal information (name, title, company, location, skills)
- Links (email, portfolio, GitHub, LinkedIn, Twitter, LeetCode)
- Business-card-labeled portfolio projects

The CLI renders the `profiles.business-card` profile plus links, skills, and projects whose labels
include `business-card`.

To refresh the embedded fallback snapshot before a future release, run:

```bash
uv run scripts/sync_resume.py
```

## Distribution Model

- **Source code in this repo:** Rust only
- **`uvx carlosferreyra`:** Python ecosystem entrypoint wrapper (generated/published in release
  pipeline)
- **`bunx carlosferreyra`:** Node ecosystem entrypoint wrapper (generated/published in release
  pipeline)

The `uvx` and `bunx` packages are distribution entrypoints, not source implementations in this
repository.

## Release Pipeline

- Before cutting any patch/minor/major release, refresh the embedded offline fallback:

  ```bash
  uv run scripts/sync_resume.py
  ```

- `Release` workflow (`.github/workflows/release.yml`) is the source of truth.
- It runs on version-tag pushes and pull requests; pull requests run distribution planning only.
- Version-tag pushes build binaries and installers with `dist`, then publish a GitHub release.
- Version bumps, tag creation, and crates.io publication are separate maintainer steps.
- After a successful release push, `release_pypi.yml` and `release_npm.yml` verify that the
  matching GitHub release and installer exist and that its tag points to the checked-out commit,
  then publish the wrappers.
- Those workflows generate Python/npm wrapper package metadata from `Cargo.toml` on the fly (no
  dedicated `python/` or `typescript/` source folders).

## Development

```bash
cargo run
cargo run -- --open portfolio
cargo check
cargo build --release
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
uv run --with httpx python -m unittest discover -s tests -v
node --test tests/npm_wrapper.test.cjs
```

## Connect with Carlos

- **GitHub**: [github.com/carlosferreyra](https://github.com/carlosferreyra)
- **LinkedIn**: [linkedin.com/in/eduferreyraok](https://linkedin.com/in/eduferreyraok)
- **Website**: [carlosferreyra.com.ar](https://carlosferreyra.com.ar)
- **Email**: [eduferreyraok@gmail.com](mailto:eduferreyraok@gmail.com)

## License

MIT

---

## Appendix: Package Index Wrappers

The packages published to PyPI and npm are entrypoint wrappers around the Rust CLI binary.

- `pipx/uvx` package: invokes the Rust `carlosferreyra` executable.
- `npx/bunx` package: invokes the Rust `carlosferreyra` executable.

The wrappers install the latest prebuilt executable when it is missing or predates self-update
support (1.2.17). Cargo and Rust are not required. A newer installed binary is never downgraded to
match an older wrapper.

The Rust executable checks the latest stable GitHub release on every invocation, including
`--version`, `--help`, and `--open`. Checks have a 1.5-second request timeout; downloads have a
30-second request timeout. New releases are verified against `sha256.sum`, installed automatically,
and restarted with the original arguments. There is no confirmation prompt or interval cache.
Update notices go to stderr. If a check, download, verification, or installation fails, the existing
card remains usable. Network access is therefore needed for updating, but not for viewing the card.

For a manual installation:

```bash
cargo install carlosferreyra --locked --force
```

Automatic wrapper installation supports macOS and Linux. Windows users must provide a separately
built Rust executable; published releases currently contain no Windows binaries.

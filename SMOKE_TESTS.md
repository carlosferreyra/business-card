# Self-update verification — October 4, 2026

Policy: check the latest stable release on every external invocation. Allow 1.5 seconds per
metadata request and 30 seconds per update download request. Install and restart automatically
when a newer version is available. Keep the presentation card usable when updating fails.

## Automated checks

```sh
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo fmt --check
uv run --with httpx python -m unittest discover -s tests -v
node --test tests/npm_wrapper.test.cjs
git diff --check
```

Results: 14 Rust tests, 12 Python tests, and 7 npm tests passed. The Rust count includes the child
entrypoint used by the restart smoke test. Clippy, formatting, and whitespace checks passed. The generated Python wheel/sdist built successfully,
and npm package dry-run validation passed in temporary directories.

The updater tests use a local HTTP fixture and temporary installation paths:

| Scenario | Observed result |
| --- | --- |
| Run the check twice with the current release | Two HTTP requests; no interval cache |
| Published version is older than installed version | No downgrade |
| New stable release is available | Exact platform `.tar.xz` selected; companion updater and checksum assets ignored |
| Download and checksum are valid | Temporary installed executable replaced |
| Restart after replacement | Disposable process executes the replacement and forwards `--version` |
| Restart marker is present | Marker removed; user arguments preserved; second check skipped |
| Checksum does not match | Update rejected; original file preserved |
| GitHub rejects the check (HTTP 403) | Error returned to the best-effort startup handler; original file preserved |
| Metadata server stalls | A 100 ms test timeout returns before 350 ms; production timeout is 1.5 s |
| Legacy binary exists | Wrapper bootstraps the latest binary |
| Installed binary is newer than the wrapper | Wrapper retains it |
| Legacy migration download fails | Wrapper still runs the existing card |
| Initial installation fails with no binary | Nonzero exit and explanatory stderr |

## Actual CLI smoke tests

Ran the newly built `target/debug/carlosferreyra`:

| Invocation | Observed result |
| --- | --- |
| `--version` with network available | Exit 0; stdout `carlosferreyra 1.2.16`; no update notice |
| `--help` with network available | Exit 0; normal CLI help |
| `--version` with network unavailable | Exit 0; version preserved; brief update-unavailable notice on stderr |
| `--help` with network unavailable | Exit 0; help preserved; brief notice on stderr |
| No arguments, network unavailable, stdin closed | Exit 0; embedded card rendered with Carlos's name and skills |
| `--open nonexistent` with network unavailable | Exit 1; normal unknown-link error; no browser opened |

Network-unavailable tests used an unreachable local proxy. For example:

```sh
HTTPS_PROXY=http://127.0.0.1:1 HTTP_PROXY=http://127.0.0.1:1 \
ALL_PROXY=http://127.0.0.1:1 NO_PROXY='' \
./target/debug/carlosferreyra --version
```

## Real initial installation without Cargo

Both generated wrappers were run in disposable harnesses with:

- An empty temporary installation directory.
- `PATH=/usr/bin:/bin:/usr/sbin:/sbin`; verified that Cargo was absent.
- A temporary home supplied to the harness, matching a temporary `CARGO_HOME`.
- A temporary receipt directory and installer PATH modification disabled.

The Python and npm wrappers each downloaded the real prebuilt macOS ARM64 GitHub release and
successfully printed `carlosferreyra 1.2.16`. Neither invoked Cargo or compiled Rust. The normal
user installation was untouched; all temporary installations were removed afterward.

## Release boundary

The public release is still 1.2.16. The refactor is local and unreleased, so the live installation
smokes verified initial download of 1.2.16. Updating and restarting were verified against controlled
release fixtures, not an unpublished GitHub release.

The wrappers treat 1.2.17 as the first self-updating version. Publish this refactor as 1.2.17 or later.
Users on older cached wrappers need a one-time wrapper refresh, such as
`uvx --refresh carlosferreyra --version`, after that release exists. Direct binary users need a
one-time installation of the updater-enabled release. Subsequent invocations check automatically.

This is best-effort freshness: offline execution, rate limits, unavailable platform assets, or
installation failures can leave the existing version running. Cross-platform replacement still
needs confirmation in the release's macOS/Linux CI; local smoke tests ran on macOS ARM64.

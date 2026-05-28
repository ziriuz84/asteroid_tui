# Contributing

When contributing to this repository, please first discuss substantial changes via an [issue](https://github.com/ziriuz84/asteroid_tui/issues) or [discussion](https://github.com/ziriuz84/asteroid_tui/discussions) when possible.

Please follow the [Code of Conduct](CODE_OF_CONDUCT.md) in all interactions with the project.

## Opening issues

Use the [issue templates](https://github.com/ziriuz84/asteroid_tui/issues/new/choose) when reporting problems or ideas:

| Kind of report | Template |
|----------------|----------|
| Something broken | **Bug Report** |
| New capability or improvement | **Feature Request** |
| README, `docs/`, rustdoc, or UI copy | **Documentation** |
| Questions, help, informal ideas | [GitHub Discussions](https://github.com/ziriuz84/asteroid_tui/discussions) |

Blank issues are disabled; pick the closest template so maintainers get the context they need.

## Development setup

Prerequisites: Rust (stable), Cargo.

```bash
git clone https://github.com/ziriuz84/asteroid_tui.git
cd asteroid_tui
cargo build
cargo test
cargo clippy -- -D warnings
cargo doc --no-deps
```

Tests are offline by default (fixtures in `response_examples/`). To run live API smoke tests (7timer, sunrise-sunset, MPC):

```bash
cargo test --features network-tests
```

Optional: open local API docs with `cargo doc --no-deps --open`.

## Documentation expectations

When your change affects users or public APIs, update:

| Change type | Update |
|-------------|--------|
| Menu flow, config keys, install | [README.md](README.md) |
| Module layout, external APIs | [docs/architecture.md](docs/architecture.md) |
| Public Rust API | Doc comments (`///`) in source; verify with `cargo doc --no-deps` |
| User-visible behaviour | [CHANGELOG.md](CHANGELOG.md) (or release notes via `release.sh`) |

See [docs/README.md](docs/README.md) for the full documentation index.

Commit messages: follow [COMMIT_MESSAGES.md](COMMIT_MESSAGES.md) (Conventional Commits).

## Pull request process

1. Ensure the branch builds and tests pass (`cargo test`, `cargo clippy -- -D warnings`).
2. Update [README.md](README.md) when changing the CLI, configuration, or install steps.
3. Add or update rustdoc for new or changed `pub` items (`#![warn(missing_docs)]` on the crate).
4. Describe the change clearly in the PR; link related issues when applicable.

## Releases

Maintainers use [release.sh](release.sh) for version bumps, changelog entries, tags, and push. Jenkins publishes to crates.io and GitHub Releases when a version tag is on `main`.

![asteroid_tui](https://socialify.git.ci/ziriuz84/asteroid_tui/image?description=1&descriptionEditable=Terminal%20tools%20for%20minor%20planet%20observation%20planning&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Light)

# asteroid-tui

Terminal tools for minor-planet researchers and enthusiasts: observation scheduling and planning from the command line.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![crates.io](https://img.shields.io/crates/v/asteroid-tui.svg)](https://crates.io/crates/asteroid-tui)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/ziriuz84/asteroid_tui/total)

**Crate:** [crates.io/crates/asteroid-tui](https://crates.io/crates/asteroid-tui) · **Repository:** [github.com/ziriuz84/asteroid_tui](https://github.com/ziriuz84/asteroid_tui)

## Features

### Available

- **Weather forecast** — astronomical seeing, transparency, clouds, wind (via [7timer](http://www.7timer.info/))
- **Sun & moon times** — sunrise, sunset, twilight, moon phase (via [sunrise-sunset.org](https://sunrise-sunset.org/))
- **Observing target list** — visible minor planets and related objects for your site (via [MPC What's Up](https://www.minorplanetcenter.net/whatsup/))
- **Observatory settings** — coordinates, horizon limits, MPC code, observer details
- **Bilingual UI** — English and Italian

### Planned

- NeoCP listing
- Object ephemeris

## Installation

### From crates.io (recommended)

Requires a [Rust toolchain](https://www.rust-lang.org/tools/install):

```bash
cargo install asteroid-tui
```

### Pre-built binary (Linux)

Download the latest `.tar.gz` from [GitHub Releases](https://github.com/ziriuz84/asteroid_tui/releases), extract it, and run `asteroid-tui`.

### From source

```bash
git clone https://github.com/ziriuz84/asteroid_tui.git
cd asteroid_tui
cargo build --release
./target/release/asteroid-tui
```

Or install locally:

```bash
cargo install --path .
```

## Quick start

```bash
asteroid-tui
```

On first run, a default configuration file is created at `~/.config/asteroid_tui/config.toml`.

Use the numeric menu:

| Menu | Options |
|------|---------|
| **Main** | `1` Settings · `2` Scheduling · `0` Quit |
| **Settings** | General (language, MPC token) · Observatory (site coordinates and limits) |
| **Scheduling** | Weather forecast · Sun & moon times · Observing target list |

Configure your observatory under **Settings** before using scheduling features that depend on your location.

## Configuration

Config path (Linux): `~/.config/asteroid_tui/config.toml`

Key settings:

| Section | Purpose |
|---------|---------|
| `[general]` | `lang` (`en` or `it`), optional `mpc_auth_token` |
| `[observatory]` | `latitude`, `longitude`, `altitude`, `mpc_code`, horizon limits (`north_altitude`, etc.) |

**MPC authentication:** for What's Up and related services, set a valid token either in config or via environment variable (env takes precedence):

```bash
export MPC_AUTH_TOKEN="your-token-here"
```

You can also change language and observatory data from the in-app **Settings** menus.

## Data sources

| Feature | Source |
|---------|--------|
| Weather | [7timer.info](http://www.7timer.info/) |
| Sun & moon | [api.sunrise-sunset.org](https://api.sunrise-sunset.org/) |
| Target list | [Minor Planet Center — What's Up](https://www.minorplanetcenter.net/whatsup/) |

## Development

Prerequisites: Rust (stable), Cargo.

```bash
cargo test
cargo clippy -- -D warnings
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines and [COMMIT_MESSAGES.md](COMMIT_MESSAGES.md) for commit message conventions. Please follow the [Code of Conduct](CODE_OF_CONDUCT.md).

Release history: [CHANGELOG.md](CHANGELOG.md).

## Roadmap

- NeoCP listing
- Object ephemeris

## Support

Open an [issue](https://github.com/ziriuz84/asteroid_tui/issues) or use [Discussions](https://github.com/ziriuz84/asteroid_tui/discussions) on GitHub.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.

## Authors

- [@ziriuz84](https://github.com/ziriuz84)

## Used by

- [Osservatorio "Il Coreggiolo" — La Spezia](https://ilcoreggiolo.netlify.app/)
- [Osservatorio "L. Zannoni" — La Spezia](https://astrofilispezzini.org)

## License

This project is licensed under the [GNU General Public License v3.0 or later](LICENSE) (GPL-3.0-or-later).

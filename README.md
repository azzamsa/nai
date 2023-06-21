<div align="center">
  <h1>Nai (나이)</h1>

<img src='docs/time.svg' width=80px />

Measure the duration of meaningful pursuits.

<a href="https://github.com/azzamsa/nai/actions/workflows/ci.yml">
    <img src="https://github.com/azzamsa/nai/actions/workflows/ci.yml/badge.svg" alt="Build status" />
  </a>

<a href="https://crates.io/crates/nai">
    <img src="https://img.shields.io/crates/v/nai.svg">
  </a>

<a href=" https://docs.rs/nai/">
    <img src="https://docs.rs/nai/badge.svg">
  </a>

<a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

<p><p/>

![demo](https://github.com/azzamsa/nai/assets/17734314/7742a2f6-f2a9-4ef1-85bf-e4dceda0e638)

</div>

---

## Features

- Measure the duration of cherished moments.
- Fancy error message and colorful output.
- Cross-platform and single binary.

## Why?

I want to have a fast way to calculate my family age, how long I have been working for a company, marriage, and other stuff.

## Usage

Create a file named `config.ron` in ~/.config/nai/. If you are on Windows, put it under \AppData\Nai\. Then add your configuration:

```rust
Config(
    moments: [
        // My birthday
        (
            start_date: "1980-Oct-15",
            // Currently, only `start_date` and `duration` are available
            format: "👶 {{ 'Aragorn II Elessar age (Me)' | cyan | bold }}\nBorn at: {{ start_date | red }}\nAge: {{ duration }}\n",
        ),
    ],
)
```

To see more configuration, see the `example` directory.

## Installation

### From binaries

The [release page](https://github.com/azzamsa/nai/releases) includes
pre-compiled binaries for GNU/Linux, macOS, and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
$ cargo binstall nai
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install nai
```

## Development

```bash
git clone https://github.com/azzamsa/nai

# Build
cd nai
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [the development guide](docs/dev/README.md)

## Origin of the name

Nai (나이) means "age" in Korean.

## Credits

- [Noto Emoji](https://github.com/googlefonts/noto-emoji)

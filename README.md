## stable-eyre

[![Build Status][actions-badge]][actions-url]
[![Latest Version][version-badge]][version-url]
[![Rust Documentation][docs-badge]][docs-url]

[actions-badge]: https://github.com/yaahc/stable-eyre/workflows/Continuous%20integration/badge.svg
[actions-url]: https://github.com/yaahc/stable-eyre/actions?query=workflow%3A%22Continuous+integration%22
[version-badge]: https://img.shields.io/crates/v/stable-eyre.svg
[version-url]: https://crates.io/crates/stable-eyre
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg
[docs-url]: https://docs.rs/stable-eyre

This library provides a custom [`eyre::EyreHandler`] type for usage with [`eyre`] that provides
all the same features as `eyre::DefaultHandler` except it works on stable by capturing a
[`backtrace::Backtrace`] via backtrace-rs.

## Setup

Add the following to your toml file:

```toml
[dependencies]
stable-eyre = "0.2"
```

Then install the hook handler before constructing any `eyre::Report` types.

# Example

```rust
use stable_eyre::eyre::{eyre, Report, WrapErr};

fn main() -> Result<(), Report> {
    stable_eyre::install()?;

    let e: Report = eyre!("oh no this program is just bad!");

    Err(e).wrap_err("usage example successfully experienced a failure")
}
```

[`eyre::EyreHandler`]: https://docs.rs/eyre/*/eyre/trait.EyreHandler.html
[`eyre`]: https://docs.rs/eyre
[`backtrace::Backtrace`]: https://docs.rs/backtrace/*/backtrace/struct.Backtrace.html

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

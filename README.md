stable-eyre
-----------

A custom context for [`eyre`] that captures a `Backtrace` on stable.

## Explanation

This crate works by defining a `Context` type which implements [`eyre::EyreContext`]
and a pair of type aliases for setting this context type as the parameter of
[`eyre::Report`].

```rust
pub type Report = eyre::Report<Context>;
pub type Result<T, E = Report> = core::result::Result<T, E>;
```

## Setup

Add the following to your toml file:

```toml
[dependencies]
eyre = "0.3.8"
stable-eyre = "0.1.0"
```

And then import the type alias from color-eyre for [`eyre::Report`] or [`eyre::Result`].

```rust
use stable_eyre::Report;

// or

fn example(&self) -> stable_eyre::Result<()> {
    // ...
}
```

# Example


```rust
use eyre::WrapErr;
use stable_eyre::Report;

fn main() -> Result<(), Report> {
    Ok(read_config()?)
}

fn read_file(path: &str) -> Result<(), Report> {
    Ok(std::fs::read_to_string(path).map(drop)?)
}

fn read_config() -> Result<(), Report> {
    read_file("fake_file")
        .wrap_err("Unable to read config")
}
```

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

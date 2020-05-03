//! This library provides a custom [`eyre::EyreContext`] type for usage with [`eyre`] that provides
//! all the same features as `eyre::DefaultContext` except it works on stable by capturing a
//! [`backtrace::Backtrace`] via backtrace-rs.
//!
//! # Example
//!
//! ```rust,should_panic
//! use eyre::{eyre, WrapErr};
//! use stable_eyre::Report;
//!
//! fn main() -> Result<(), Report> {
//!     let e: Report = eyre!("oh no this program is just bad!");
//!
//!     Err(e).wrap_err("usage example successfully experienced a failure")
//! }
//! ```
//!
//! [`eyre::EyreContext`]: https://docs.rs/eyre/0.3.8/eyre/trait.EyreContext.html
//! [`eyre`]: https://docs.rs/eyre
//! [`backtrace::Backtrace`]: https://docs.rs/backtrace/0.3.46/backtrace/struct.Backtrace.html
use backtrace::Backtrace;
use eyre::Chain;
use eyre::EyreContext;
use std::error::Error;

/// A custom context type for capturing backtraces on stable with `eyre`
#[derive(Debug)]
pub struct Context {
    backtrace: Backtrace,
}

impl EyreContext for Context {
    #[allow(unused_variables)]
    fn default(error: &(dyn Error + 'static)) -> Self {
        let backtrace = Backtrace::new();

        Self { backtrace }
    }

    fn debug(
        &self,
        error: &(dyn Error + 'static),
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        use core::fmt::Write as _;

        if f.alternate() {
            return core::fmt::Debug::fmt(error, f);
        }

        write!(f, "{}", error)?;

        if let Some(cause) = error.source() {
            write!(f, "\n\nCaused by:")?;
            let multiple = cause.source().is_some();
            for (n, error) in Chain::new(cause).enumerate() {
                writeln!(f)?;
                if multiple {
                    write!(indenter::Indented::numbered(f, n), "{}", error)?;
                } else {
                    write!(indenter::Indented::new(f), "{}", error)?;
                }
            }
        }

        let backtrace = &self.backtrace;
        write!(f, "\n\n{:?}", backtrace)?;

        Ok(())
    }
}

/// A type alias for `eyre::Report<stable_eyre::Context>`
///
/// # Example
///
/// ```rust
/// use stable_eyre::Report;
///
/// # struct Config;
/// fn try_thing(path: &str) -> Result<Config, Report> {
///     // ...
/// # Ok(Config)
/// }
/// ```
pub type Report = eyre::Report<Context>;

/// A type alias for `Result<T, stable_eyre::Report>`
///
/// # Example
///
///```
/// fn main() -> stable_eyre::Result<()> {
///
///     // ...
///
///     Ok(())
/// }
/// ```
pub type Result<T, E = Report> = core::result::Result<T, E>;

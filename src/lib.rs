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
use std::{env, error::Error};

pub use eyre;
#[doc(hidden)]
pub use eyre::{Report, Result};
use indenter::indented;

/// A custom context type for capturing backtraces on stable with `eyre`
#[derive(Debug)]
pub struct Handler {
    backtrace: Option<Backtrace>,
}

impl eyre::EyreHandler for Handler {
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
                    write!(indented(f).ind(n), "{}", error)?;
                } else {
                    write!(indented(f), "{}", error)?;
                }
            }
        }

        if let Some(backtrace) = &self.backtrace {
            write!(f, "\n\nStack backtrace:\n{:?}", backtrace)?;
        }

        Ok(())
    }
}

pub struct Hook {
    capture_backtrace_by_default: bool,
}

impl Hook {
    #[allow(unused_variables)]
    fn make_handler(&self, error: &(dyn Error + 'static)) -> Handler {
        let backtrace = if self.capture_enabled() {
            Some(Backtrace::new())
        } else {
            None
        };

        Handler { backtrace }
    }

    fn capture_enabled(&self) -> bool {
        env::var("RUST_LIB_BACKTRACE")
            .or_else(|_| env::var("RUST_BACKTRACE"))
            .map(|val| val != "0")
            .unwrap_or(self.capture_backtrace_by_default)
    }

    pub fn capture_backtrace_by_default(mut self, cond: bool) -> Self {
        self.capture_backtrace_by_default = cond;
        self
    }

    pub fn install(self) -> Result<()> {
        crate::eyre::set_hook(Box::new(move |e| Box::new(self.make_handler(e))))?;

        Ok(())
    }
}

impl Default for Hook {
    fn default() -> Self {
        Self {
            capture_backtrace_by_default: false,
        }
    }
}

pub fn install() -> Result<()> {
    Hook::default().install()
}

//! This library provides a custom [`eyre::EyreHandler`] type for usage with [`eyre`] that provides
//! all the same features as `eyre::DefaultHandler` except it works on stable by capturing a
//! [`backtrace::Backtrace`] via backtrace-rs.
//!
//! ## Setup
//!
//! Add the following to your toml file:
//!
//! ```toml
//! [dependencies]
//! stable-eyre = "0.2"
//! ```
//!
//! Then install the hook handler before constructing any `eyre::Report` types.
//!
//! # Example
//!
//! ```rust,should_panic
//! use stable_eyre::eyre::{eyre, Report, WrapErr};
//!
//! fn main() -> Result<(), Report> {
//!     stable_eyre::install()?;
//!
//!     let e: Report = eyre!("oh no this program is just bad!");
//!
//!     Err(e).wrap_err("usage example successfully experienced a failure")
//! }
//! ```
//!
//! [`eyre::EyreHandler`]: https://docs.rs/eyre/*/eyre/trait.EyreHandler.html
//! [`eyre`]: https://docs.rs/eyre
//! [`backtrace::Backtrace`]: https://docs.rs/backtrace/*/backtrace/struct.Backtrace.html
#![doc(html_root_url = "https://docs.rs/stable-eyre/0.2.2")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

pub use eyre;
#[doc(hidden)]
pub use eyre::{Report, Result};

use ::backtrace::Backtrace;
use indenter::indented;
use std::{env, error::Error, iter};

/// Extension trait to extract a backtrace from an `eyre::Report`, assuming
/// stable-eyre's hook is installed.
pub trait BacktraceExt {
    /// Returns a reference to the captured backtrace if one exists
    ///
    /// # Example
    ///
    /// ```rust
    /// use stable_eyre::{BacktraceExt, eyre::eyre};
    /// stable_eyre::install();
    /// std::env::set_var("RUST_BACKTRACE", "1");
    ///
    /// let report = eyre!("capture a report");
    /// assert!(report.backtrace().is_some());
    /// ```
    fn backtrace(&self) -> Option<&Backtrace>;
}

impl BacktraceExt for eyre::Report {
    fn backtrace(&self) -> Option<&Backtrace> {
        self.handler()
            .downcast_ref::<crate::Handler>()
            .and_then(|handler| handler.backtrace.as_ref())
    }
}

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
            let errors = iter::successors(Some(cause), |e| (*e).source());

            for (n, error) in errors.enumerate() {
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

/// Builder for customizing the behavior of the global error report hook
#[derive(Debug)]
pub struct HookBuilder {
    capture_backtrace_by_default: bool,
}

impl HookBuilder {
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

    /// Configures the default capture mode for `Backtraces` in error reports
    pub fn capture_backtrace_by_default(mut self, cond: bool) -> Self {
        self.capture_backtrace_by_default = cond;
        self
    }

    /// Install the given hook as the global error report hook
    pub fn install(self) -> Result<()> {
        crate::eyre::set_hook(Box::new(move |e| Box::new(self.make_handler(e))))?;

        Ok(())
    }
}

impl Default for HookBuilder {
    fn default() -> Self {
        Self {
            capture_backtrace_by_default: false,
        }
    }
}

/// Install the default error report hook provided by `stable-eyre`
///
/// # Details
///
/// This function must be called to enable the customization of `eyre::Report`
/// provided by `stable-eyre`. This function should be called early, ideally
/// before any errors could be encountered.
///
/// Only the first install will succeed. Calling this function after another
/// report handler has been installed will cause an error. **Note**: This
/// function _must_ be called before any `eyre::Report`s are constructed to
/// prevent the default handler from being installed.
pub fn install() -> Result<()> {
    HookBuilder::default().install()
}

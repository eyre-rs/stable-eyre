use eyre::{eyre, WrapErr};
use stable_eyre::Report;

fn main() -> Result<(), Report> {
    let e: Report = eyre!("oh no this program is just bad!");

    Err(e).wrap_err("usage example successfully experienced a failure")
}


#![feature(let_chains)]
mod display;
mod state;
use state::*;
fn main() -> Result<(), std::io::Error> {
    display::dynamic::start_terminal()?;
    Ok(())
}

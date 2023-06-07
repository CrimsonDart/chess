
#![feature(let_chains)]
mod display;
mod state;
mod board;
mod check;
mod types;
mod piece;
fn main() -> Result<(), std::io::Error> {
    display::dynamic::start_terminal()?;
    Ok(())
}

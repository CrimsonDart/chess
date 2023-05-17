
use ColorSelection::*;
use std::fmt::Write;
use colored::{Colorize, ColoredString};
use crate::state::*;
use crate::Piece::*;


static mut DARK_TILE: bool = false;

// prints the board to the screen
pub fn print_state() {

    let mut s = String::new();

    let top_row = [" ~ ", " A ", " B ", " C ", " D ", " E ", " F ", " G ", " H "];
    for c in top_row {
        write!(&mut s, "{}", color(c, ColorSelection::Auto)).ok();
    }


    let mut i: u8 = 9;
    for row in get_board().iter().rev() {
        i = i - 1;

        let mut number_s = String::new();
        write!(&mut number_s, " {} ", i.to_string()).ok();
        write!(&mut s, "\n{}", color(number_s.as_str(), ColorSelection::Auto)).ok();

        // runs through each entry in the row.
        for space in row {
            match space {
                Some(data) => {
                    write!(&mut s, "{}",
                           color(match data.0 {
                               Pawn => " P ",
                               Rook => " R ",
                               Knight => " N ",
                               Bishop => " B ",
                               Queen => " Q ",
                               King => " K "
                           }, match data.1 {
                               true => ColorSelection::White,
                               false => ColorSelection::Black

                           })
                    ).ok();
                },
                None => {
                    write!(&mut s, "{}",
                        color("   ", ColorSelection::Auto)
                    ).ok();
                }
            }
        }
    }
    print!("{}\n", s);
}


enum ColorSelection {
    Auto,
    White,
    Black
}

fn is_dark() -> bool {
    unsafe {
        DARK_TILE
    }
}

// this function handles the color of the input text.
// automatically swaps between dark and light tiles, for the chess board.
fn color(string: &str, color: ColorSelection ) -> ColoredString {

    string.bold();

    // handles text color
    let string = match color {
        Auto => match is_dark() {
            true => string.truecolor(180, 180, 180),
            false => string.truecolor(200, 200, 200)
        },
        White => string.truecolor(230, 230, 230),
        Black => string.truecolor(16, 16, 16)
    };

    // handles background coloring
    let string = match is_dark() {
        true => string.on_truecolor(50, 50, 50),
        false => string.on_truecolor(64, 64, 64)
    };

    //toggles dark tile
    unsafe {
        match DARK_TILE {
            true => DARK_TILE = false,
            false => DARK_TILE = true
        }
    }

    string
}

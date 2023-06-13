



use super::events::UserState;
use crate::{types::{Space, Movement::*}, board::{Loc, read_board}, piece::move_list, check::deep_checks};
use ratatui::{
    layout::Rect,
    buffer::{Buffer, Cell},
    widgets::Widget,
    style::{Style, Color, Modifier}
};

impl UserState {
    pub fn get_rect() -> Rect {
        Rect { x: 5, y: 5, width: 30, height: 10 }
    }
}

enum FColor {
    Auto,
    White,
    Black
}

fn write_cell(cells: &mut Vec<Cell>, is_dark: &bool, color: FColor, c: char) -> bool {

    let black_tile: Style = {

        Style::default()
            .fg(Color::Rgb(120,120,120))
            .bg(Color::Rgb(50,50,50))
            .add_modifier(Modifier::BOLD)
    };

    let white_tile: Style = {

        Style::default()
            .fg(Color::Rgb(130,130,130))
            .bg(Color::Rgb(64,64,64))
            .add_modifier(Modifier::BOLD)
    };

    let style = match is_dark {
        true => black_tile,
        false => white_tile
    };

    let mut arr = [Cell::default(), Cell::default(), Cell::default()];

    arr[0].set_style(style).set_char(' ');
    arr[1].set_style(style).set_char(c);

    match color {
        FColor::White => {arr[1].set_fg(Color::Rgb(230,230,230));},
        FColor::Black => {arr[1].set_fg(Color::Rgb(16,16,16));},
        _ => ()
    }

    arr[2].set_style(style).set_char(' ');

    for cell in arr {
        cells.push(cell);
    }
    !is_dark
}

// stands for "board position to vector cell"
fn bpvc(location: Loc) -> usize {
    return ((((location[1]) * 10) + location[0]) * 3) as usize;
}

fn set_background_color(location: Loc, color: Color, cells: &mut Vec<Cell>) {

    let i = bpvc(location);
    cells[i].set_bg(color);
    cells[i + 1].set_bg(color);
    cells[i + 2].set_bg(color);
}

// each grid space will be rendered 3 cells wide, with a char in the center.
// the chess board is 8x8 spaces, and will be bordered by a numbered legend (or whatever you call it).

// the board will be 24 cells wide, 8 tall
// including the border, it is 30 wide, 10 tall.

impl Widget for &UserState {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let mut is_dark = false;
        let mut cells: Vec<Cell> = Vec::new();
        let top_bottom_row = "~ABCDEFGH~";

        // top row numbers
        for c in top_bottom_row.chars() {
            is_dark = write_cell(&mut cells, &is_dark, FColor::Auto, c);
        }

        // render board and side numbers.
        let mut row_n:usize = 8;
        for row in self.board.iter() {

            row_n = row_n - 1;
            let n = ['1', '2', '3', '4', '5', '6', '7', '8'][row_n];

            is_dark = write_cell(&mut cells, &!is_dark, FColor::Auto, n);
            for piece in row {

                is_dark = write_cell(&mut cells, &is_dark,
                    if Space::Open == *piece {
                        FColor::Auto
                    } else {
                        if piece.is_white() {
                            FColor::White
                        } else {
                            FColor::Black
                        }
                    },
                    piece.clone().into()
                );
            }
            is_dark = write_cell(&mut cells, &is_dark, FColor::Auto, n);
        }

        // bottom row numbers
        is_dark = !is_dark;
        for c in top_bottom_row.chars() {
            is_dark = write_cell(&mut cells, &is_dark, FColor::Auto, c);
        }
        // renders cursors.

        if let Some(c) = self.selected {
            set_background_color(c, Color::Rgb(220,139,0), &mut cells);
            let from = read_board(&self.board, c).unwrap();

            let mut move_list = move_list(&self.board, c, from);
            deep_checks(&self.board, c, &mut move_list);
            for move_data in move_list {

                set_background_color(move_data.to, match move_data.relation {

                    Empty | PawnSkip | QueenSide | KingSide => Color::Rgb(13, 255, 00),
                    Enemy => Color::Rgb(255, 70, 70),
                    Blocked => continue,
                    Check => Color::Rgb(32, 48, 32)

                }, &mut cells);
            }
        }

        if self.cursor_blink {
            set_background_color(self.cursor, Color::Rgb(23,74,255), &mut cells);
        }

        // maps the local Vec<Cell> to the full terminal buffer.
        let mut i = 0;
        for ry in area.top()..area.bottom() {
            for rx in area.left()..area.right() {
                let cell = buf.get_mut(rx, ry);
                let copier = &cells[i];

                cell.set_style(copier.style());
                cell.set_symbol(copier.symbol.as_str());

                i = i +1;
            }
        }
    }
}

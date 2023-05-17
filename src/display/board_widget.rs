




use crate::state::{Space, PieceInteraction};
use ratatui::{
    layout::Rect,
    buffer::{Buffer, Cell},
    widgets::Widget,
    style::{Style, Color, Modifier}
};

use super::events::CursorBlink;



pub struct DisplayState<'a> {
    pub board: &'static [[Option<Space> ;8]; 8],
    pub user: &'a super::events::UserState,
}



impl DisplayState<'_> {
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
fn bpvc(x: usize, y: usize) -> usize {
    return (((y) * 10) + x) * 3;
}

fn set_background_color(x: usize, y: usize, color: Color, cells: &mut Vec<Cell>) {

    let i = bpvc(x, y);
    cells[i].set_bg(color);
    cells[i + 1].set_bg(color);
    cells[i + 2].set_bg(color);
}

// each grid space will be rendered 3 cells wide, with a char in the center.
// the chess board is 8x8 spaces, and will be bordered by a numbered legend (or whatever you call it).

// the board will be 24 cells wide, 8 tall
// including the border, it is 30 wide, 10 tall.

impl Widget for DisplayState<'_> {
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
            for p in row {

                if let Some(piece) = p {
                    is_dark = write_cell(&mut cells, &is_dark,
                        match piece.1 {
                            true => FColor::White,
                            false => FColor::Black
                        },
                        piece.0.into()
                    )
                } else {
                    is_dark = write_cell(&mut cells, &is_dark, FColor::Auto, ' ');
                }
            }
            is_dark = write_cell(&mut cells, &is_dark, FColor::Auto, n);
        }

        // bottom row numbers
        is_dark = !is_dark;
        for c in top_bottom_row.chars() {
            is_dark = write_cell(&mut cells, &is_dark, FColor::Auto, c);
        }


        // renders cursors.


        if let Some(c) = self.user.mouse_cursor {
            set_background_color(c[0], c[1], Color::Rgb(128, 128, 255), &mut cells);
        }
        if let Some(c) = self.user.selected {
            set_background_color(c[0], c[1], Color::Rgb(220,139,0), &mut cells);

            // set colors of possible moves.
            if let Ok(Some(space)) = crate::state::read_board(c[0] as usize, c[1] as usize) {
                for mov in space.move_list(c[0] as usize, c[1] as usize) {


                    set_background_color(mov.0, mov.1, match mov.2 {

                        PieceInteraction::Empty => Color::Rgb(13, 255, 00),
                        PieceInteraction::Enemy => Color::Rgb(255, 70, 70)

                    }, &mut cells);







                }

            }




        }

        if let CursorBlink::On(_) | CursorBlink::Cooldown(_) = self.user.cursor_blink {
            set_background_color(self.user.key_cursor[0], self.user.key_cursor[1], Color::Rgb(23,74,255), &mut cells);
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

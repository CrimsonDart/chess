
mod display;
mod state;
use state::*;
fn main() {

    display::print_state();




    println!("{}", state::read_board(3, 7).unwrap().unwrap());

    println!("{:?}", Space::move_list(&Space::new(Piece::Bishop, true), 3, 3));


}

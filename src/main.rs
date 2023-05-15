
mod state;

fn main() {

    state::print_board();

    println!("{}", state::read_board(3, 7).unwrap().unwrap());


}

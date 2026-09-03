mod game_board;

fn main() {

    let gb = game_board::game_board::GameBoard::new(1);

    println!("{:?}", gb.get_board());
    println!("{:?}", gb.get_master()); 

}
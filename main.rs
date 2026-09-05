mod game_board;


fn main() {

    let gb = game_board::game_board::GameBoard::new(1);

    loop {
        let input =std::io::stdin().read_line(input);
        match input {
            "q" => std::process::exit(1),
            _ => println!("Enter a valid input.")
        };
    }
}

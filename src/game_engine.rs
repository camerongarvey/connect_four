pub mod game_engine {


pub struct Board {
    pub board: [[char; 6]; 7], //7 columns each containing 6 items
    pub turn: char,
}

impl Board {

    pub fn new() -> Board {
        Board { board: [['0', '0', '0', '0', '0', '0'],
                        ['0', '0', '0', '0', '0', '0'],
                        ['0', '0', '0', '0', '0', '0'],
                        ['0', '0', '0', '0', '0', '0'],
                        ['0', '0', '0', '0', '0', '0'],
                        ['0', '0', '0', '0', '0', '0'],
                        ['0', '0', '0', '0', '0', '0']],
                turn: 'y' }
    }

    pub fn print_board(&self) { //prints the board to standard output
        for i in 0..6 {
            for j in 0..7 {
                print!("{}", self.board[j][i])
            }
            print!("\n")
        }
        print!("1234567 \n")
    }

    pub fn player_move(&mut self) { //player move function
        let target: usize = self.get_move();
        if self.cheack_legal_move(target) {
            //self.make_move(target, self.turn);
            println!("valid move!")
        } else {
            self.player_move();
        }
    }
    fn get_move(&self) -> usize { //Reads input and returns it
        println!("It is {}'s turn. Input move below: ", self.turn);
        
            let mut input_string: String = String::new();
        
            std::io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read input");
        
            match input_string.parse::<usize>() {
                    Ok(parsed_value) => {
                        return parsed_value; //could be invalid move
                    },
                    Err(_) => {
                        return 0; //is invalid move, will cause function to be run again
                    }
                }
    } 

    fn cheack_legal_move(&self, target:usize) -> bool {
        if self.board[target].contains(&'0') { 
            return true
        }
        false
    }
} 

}
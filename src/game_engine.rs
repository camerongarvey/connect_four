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

    pub fn player_move(&mut self) -> bool{ //player move function
        let target: usize = self.get_move();
        if self.check_legal_move(target) {
            self.make_move(target);
            if self.check_win(target) {
                println!("{} Wins!", self.turn);
                return true
            } else if !self.board.iter().any(|row| row.contains(&'0')){
                println!("It's a tie!");
                return true
            }
            return false
        } else {
            self.player_move();
        }
        false
        
    }
   
    fn get_move(&self) -> usize { //Reads input and returns it
        println!("It is {}'s turn. Input move below: ", self.turn);
        
            let mut input_string: String = String::new();
        
            std::io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read input");
            let trimmed_input = input_string.trim();

            match trimmed_input.parse::<usize>() {
                    Ok(parsed_value) => {
                        return parsed_value; //could be invalid move
                    },
                    Err(_) => {
                        return 99; //is invalid move, will cause function to be run again
                    }
                }
    } 

    fn check_legal_move(&self, target:usize) -> bool { //checks if move is legal or not
        if target <= 7 && target > 0 && self.board[target-1].contains(&'0') { //-1 to offset array indexing 
            return true
        }
        false
    }

    fn make_move(&mut self, target:usize) { //edits board
        for row in 0..5 {
            if self.board[target-1][row+1] != '0' {
                self.board[target-1][row] = self.turn;
                return
            }
        }
        self.board[target-1][5] = self.turn;
    }
    

    pub fn check_win(&self, mut x:usize) -> bool {
        x-=1;
        let mut y: usize = 0;
        let board = self.board;
        for tile in 0..self.board[x].len() {
            if self.board[x][tile] != '0' {
                y=tile;
                break
            }
        }
        
        //vert
        for i in 0..4 {
            if y+i <= 5 && (y as i32-3 + i as i32) >= 0{ 
                if board[x][y+i-3] == board[x][y+i-2] && board[x][y+i-1] == board[x][y+i] && board[x][y+i-3] == board[x][y+i] {
                    return true
                }
            }
        }
        
        //horz
        for i in 0..4 {
            if x+i <= 6 && (x as i32-3 + i as i32) >= 0{ 
                if board[x+i-3][y] == board[x+i-2][y] && board[x+i-1][y] == board[x+i][y] && board[x+i-3][y] == board[x+i][y] {
                    return true
                }
            }
        }
        //dig
        for i in 0..4 {
            if x+i <= 6 && (x as i32-3 + i as i32) >= 0 && y+i <= 5 && (y as i32-3 + i as i32) >= 0{
                if board[x+i-3][y+i-3] == board[x+i-2][y+i-2] && board[x+i-1][y+i-1] == board[x+i][y+i] && board[x+i-3][y+i-3] == board[x+i][y+i] {
                    return true
                }
            }
        }
        for i in 0..4 {
            if (x as i32 - i as i32) >= 0 && (x as i32+3 - i as i32) <= 6 && y+i <= 5 && (y as i32-3 + i as i32) >= 0{
                if board[x-i+3][y+i-3] == board[x-i+2][y+i-2] && board[x-i+1][y+i-1] == board[x-i][y+i] && board[x-i+3][y+i-3] == board[x+i][y+i] {
                    return true
                }
            }
        }
        false
    }

} 

}
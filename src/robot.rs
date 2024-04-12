pub mod robot {
    use std::collections::HashMap;

    pub struct Iterator {
        pub board: [[char; 6]; 7], //7 columns each containing 6 items
        pub turn: char,
        pub history: Vec<char>,
        pub robot_turn: char,
        pub lifetime: u8,
    }

    impl Iterator {
        pub fn new(board: [[char; 6]; 7], turn: char, history: Vec<char>, robot_turn: char, lifetime: u8) -> Iterator {
            Iterator { board, turn, history, robot_turn, lifetime}

        }
   
        pub fn iterate(&mut self, mut list: &mut Vec<Vec<char>>) {
            if self.lifetime == 0 {
                self.history.push('i');
                list.push(self.history.clone());
                //return
            } else if !self.board.iter().any(|row| row.contains(&'0')) {
                self.history.push('t');
                list.push(self.history.clone());
                return
            } else {
                for column in 0..7 {
                    if self.board[column].contains(&'0') {
                        let mut new_bot = Iterator::new(self.board.clone(), self.turn.clone(), self.history.clone(), self.robot_turn.clone(), self.lifetime-1);
                        new_bot.history.push(char::from_digit(column as u32, 10).unwrap());
                        new_bot.make_move(column);
    
                        if new_bot.check_win(column) {
                            if new_bot.turn == new_bot.robot_turn {
                                new_bot.history.push('w');
                                list.push(new_bot.history.clone());
                                //return;
                            } else {
                                new_bot.history.push('l');
                                list.push(new_bot.history.clone());
                                    //return;
                                }
                        
                        } else {
                            if self.turn == 'r' {new_bot.turn = 'y'} else {new_bot.turn = 'r'}
                            new_bot.iterate(&mut list);
                        }

                    }
                }
            }
        }

        fn make_move(&mut self, target:usize) { //edits board
            for row in 0..5 {
                if self.board[target][row+1] != '0' {
                    self.board[target][row] = self.turn;
                    return
                }
            }
            self.board[target][5] = self.turn;
        }
    
    
        pub fn check_win(&self, x:usize) -> bool {
            let mut y: usize = 0;
            for tile in 0..self.board[x].len() {
                if self.board[x][tile] != '0' {
                    y=tile;
                    break
                }
            }
            //vert
            for i in 0..4 {
                if y+i <= 5 && (y as i32-3 + i as i32) >= 0{ 
                    if self.board[x][y+i-3] == self.board[x][y+i-2] && self.board[x][y+i-1] == self.board[x][y+i] && self.board[x][y+i-3] == self.board[x][y+i] {
                        return true
                    }
                }
            }
            
            //horz
            for i in 0..4 {
                if x+i <= 6 && (x as i32-3 + i as i32) >= 0{ 
                    if self.board[x+i-3][y] == self.board[x+i-2][y] && self.board[x+i-1][y] == self.board[x+i][y] && self.board[x+i-3][y] == self.board[x+i][y] {
                        return true
                    }
                }
            }
            //dig
            for i in 0..4 {
                if x+i <= 6 && (x as i32-3 + i as i32) >= 0 && y+i <= 5 && (y as i32-3 + i as i32) >= 0{
                    if self.board[x+i-3][y+i-3] == self.board[x+i-2][y+i-2] && self.board[x+i-1][y+i-1] == self.board[x+i][y+i] && self.board[x+i-3][y+i-3] == self.board[x+i][y+i] {
                        return true
                    }
                }
            }
            for i in 0..4 {
                if (x as i32 - i as i32) >= 0 && (x as i32+3 - i as i32) <= 6 && y+i <= 5 && (y as i32-3 + i as i32) >= 0{
                    if self.board[x-i+3][y+i-3] == self.board[x-i+2][y+i-2] && self.board[x-i+1][y+i-1] == self.board[x-i][y+i] && self.board[x-i+3][y+i-3] == self.board[x-i][y+i] {
                        return true
                    }
                }
            }
            false
        }
    
    }



pub struct Master {
    pub moves: Vec<Vec<char>>,
    pub depth: u8,
}

impl Master {
    pub fn new(depth: u8) -> Master {
        Master {moves: Vec::new(), depth}
    }
    pub fn get_move(&mut self, board:[[char; 6]; 7], robot_turn: char, turn: char,) -> usize {
        self.moves = vec![];
        let mut bot = Iterator::new(board, turn, Vec::new(), robot_turn, self.depth);
        bot.iterate(&mut self.moves);

        let mut win_moves: Vec<Vec<char>> = Vec::new();
        let mut tie_moves: Vec<Vec<char>> = Vec::new();
        let mut lose_moves: Vec<Vec<char>> = Vec::new();
        let mut incomplete_moves: Vec<Vec<char>> = Vec::new();


        for item in self.moves.clone() {
            if item.contains(&'w') {
                win_moves.push(item);
            } else if item.contains(&'l') {
                lose_moves.push(item);
            } else if item.contains(&'t') {
                tie_moves.push(item);
            } else {
                incomplete_moves.push(item)
            }
        }

        win_moves.sort_by(|a, b| a.len().cmp(&b.len()));
        lose_moves.sort_by(|a, b| a.len().cmp(&b.len()));

        if win_moves.len() > 0 && win_moves[0].len() == 2{
            return win_moves[0][0] as usize - '0' as usize
        } else if lose_moves.len() > 0 {
            if lose_moves[0].len() == 3{
                return lose_moves[0][1] as usize - '0' as usize
            }
            return self.get_smallest(self.map_moves(&lose_moves)) as usize - '0' as usize
        } else if incomplete_moves.len() > 0 {
            return incomplete_moves[0][0] as usize - '0' as usize
            
        } else if tie_moves.len() > 0{
            return tie_moves[0][0] as usize - '0' as usize
        } else {
            let mut all_moves: Vec<Vec<char>> = Vec::new();
            all_moves.extend(win_moves);
            all_moves.extend(tie_moves);
            all_moves.extend(lose_moves);
            all_moves.extend(incomplete_moves);
            return all_moves[0][0] as usize - '0' as usize;
        }
        
    }
   


    fn map_moves(&self, list:&Vec<Vec<char>>) -> HashMap<char, i32>{
        let mut map = HashMap::new();
        for item in list {
            let count = map.entry(item[0]).or_insert(0);
            *count += 1;
        }
        map
    }  

    fn get_smallest(&self, hash_map:HashMap<char, i32>) -> char {
        let mut min_key: char = ' ';
        let mut min_value = i32::max_value(); 
    
        for (key, &value) in &hash_map {
            if value < min_value {
                min_key = *key;
                min_value = value;
            }
        }
        min_key
    }

    }

}
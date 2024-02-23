use std::io;

#[derive(Clone, PartialEq, Debug)]
enum Player {
    Red,
    Black,
}

struct Move {
    player: Player,
    column: i32,
}

impl Move {
    // reads a move from a column string
    fn read_move(c: String, player: &Player) -> Move{
        Move {
            player: player.clone(),
            column: match c.parse::<i32>() {
                Ok(num) => {
                    num
                }
                Err(_) => {
                    println!("Failed to parse the column!");
                    panic!();
                }
            }
        }
    }
}

struct Board {
    gameBoard: Vec<Vec<Option<Player>>>,
}

impl Board {
    fn display(&self) -> (){ //prints out the board to the screen
        for i in 0..6{ //iterate through rows
            print!(" | ");
            for j in 0..7{//iterate thru cols
                match self.gameBoard[i][j] {
                    Some(Player::Red) => print!("X | "),
                    Some(Player::Black) => print!("O | "),
                    None => print!("- | "),
                }
            }
            println!("\n");
        }
    }
    
    fn new_board() -> Board {
        Board { 
            gameBoard: vec![vec![None; 7]; 6]
        }
    }

    fn update_board(&mut self, m: Move) {
        for i in (0..6).rev() {
            let j: usize =  m.column.try_into().unwrap();
            if self.gameBoard[i][j] != None {
                self.gameBoard[i][j] = Some(m.player);
                break; 
            }
        }
    }

    fn is_full(&self) -> bool {
        for j in 0..=6{//move horizontally
            if self.gameBoard[0][j] == None { //go through the top row and find if any spot is open
                return false
            }
        }
        true
    }
    
    
    fn check_winner(&self) -> Option<Player> {
        let mut winner: Option<Player> = None;
        //horizontal check
        
        // for i in 0..=3{
        //     for j in 0..=5{
        //         if self.gameBoard[j][i]== self.gameBoard[j][i+1] && self.gameBoard[j][i]== self.gameBoard[j][i+2] && self.gameBoard[j][i]== self.gameBoard[j][i+3] {
        //             if self.gameBoard[j][i] != None {
        //                 winner = self.gameBoard[j][i]
        //             }
        //         }
        //     }
        // }
    
        //vertical check

        //diagonal check
        
        // fix compiler error temporary
        winner
    }

}


fn main() {
    // initialize a new game
    let mut game = Board::new_board();
    println!("Let's play Connect 4\n");
    game.display();
    // playing the game
    let mut current_player = Player::Red;
    let mut winner: Option<Player> = None;
    
    // start the main loop
    while winner == None || !game.is_full() {
        // read input
        let mut move_column = String::new();
        //if current_player == Player::Red {
            //println!("Input move for Red");
        //}
        //else println!("Input move for Black");

        // println!("Input move for {}: ", &current_player);
        io::stdin().read_line(&mut move_column).expect("Failed to read line");
        let current_move = Move::read_move(move_column, &current_player);
        
        // update the board and display the board
        game.update_board(current_move);
        game.display();

        // change the current player
        current_player = match current_player{
            Player::Red => Player::Black,
            Player::Black => Player::Red,
        };

        // check to see if there is a winner
        winner = game.check_winner();
    }

    // check to see if there is a winner or if there was a tie
    match winner {
        Some(player) => {println!("Winner is {:?}!", player)},
        None=> {println!("Tie!")}
    };   
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
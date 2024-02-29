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
    fn read_move(c: String, player: &Player) -> Option<Move>{
        let trimmed_c = c.trim().parse::<i32>(); //trim the newline character and parse the int

        match trimmed_c{
            // if int parsed, return the move
            Ok(num) => {
                Some(Move{
                    player: player.clone(),
                    column: num,
                })
            },

            // return None
            Err(_) => {
                println!("Failed to parse the column!");
                None
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
            for j in 0..7{//iterate thru cols
                match self.gameBoard[i][j] {
                    Some(Player::Red) => print!("1 "),
                    Some(Player::Black) => print!("2 "),
                    None => print!("0 "),
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

    fn update_board(&self, _m: Move) {
        
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
    let game = Board::new_board();
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

        // repeatedly scan input until valid
        let mut current_move: Option<Move> = None;
        while current_move.is_none() {
            let mut move_column = String::new();
            io::stdin().read_line(&mut move_column).expect("Failed to read line");
            current_move = Move::read_move(move_column, &current_player);
        }
        //unwrap the optional
        let current_move = current_move.unwrap();
        
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

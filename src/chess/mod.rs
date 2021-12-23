
use termion::{color};

mod board;
mod piece;
mod pos;
mod validator;

pub struct Game{
    board: board::Board,
}

fn get_input() -> String {
    let mut res = String::new();
    loop {
        match std::io::stdin()
        .read_line(&mut res){
            Ok(_) => return res,
            Err(msg) => {
                println!("something went wrong getting input: {}", msg);
                continue;
            },
        }
    }
}

fn parse_simple_input(input: &str, player: piece::Player) -> Result::<pos::MoveCandidate, String> {
    
    let trimput: String = input.chars()
    .filter(|c| !c.is_whitespace())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    let old_pos = match pos::Square::from_string(&trimput[0..2]) {
        Ok(s) => s,
        Err(msg) => {
            return Err(msg);
        }
    };
    let new_pos = match pos::Square::from_string(&trimput[2..4]) {
        Ok(s) => s,
        Err(msg) => {
            return Err(msg);
        }
    };
    let promote_to = if trimput.len() < "e7e8Q".len() {
        None
    }
    else {
        match &trimput[4..5] {
            "q" => Some(piece::Piece::Queen(player)),
            "r" => Some(piece::Piece::Rook(player)),
            "b" => Some(piece::Piece::Bishop(player)),
            "n" => Some(piece::Piece::Knight(player)),
            _ => None,
        }
    };

    Ok(pos::MoveCandidate {
        old_pos,
        new_pos,
        promote_to,
    })
}

impl Game{
    pub fn new() -> Self{
        let board = board::Board::new();
        Self {
            board,
        }
    }
    pub fn from_fen(fen: &str) -> Self {
        let board = board::Board::from_fen(fen);
        Self {
            board,
        }
    }
    pub fn play(&mut self) {
        loop {
            let check_tuple = self.board.in_check(self.board.active_player());
            if let (Some(king_pos), check_vec) = &check_tuple {
                if validator::get_possible_moves_from_square(
                    &self.board, 
                    *king_pos, 
                    &check_tuple
                ).len() == 0 {
                    if check_vec.len() > 0 { 
                        self.board.print(self.board.active_player().invert()); 
                        println!(
                            "{}Checkmate! {} wins!", 
                            color::Fg(color::Red), 
                            match self.board.active_player() {
                                piece::Player::White => "Black",
                                piece::Player::Black => "White",
                            }
                        );
                        print!("{}", color::Fg(color::Reset));
                        return;
                    } else if self.board.get_pieces(self.board.active_player())
                    .iter()
                    .all(|p| {
                        validator::get_possible_moves_from_square(
                            &self.board, 
                            p.pos, 
                            &check_tuple
                        ).len() == 0
                    }) {
                        self.board.print(self.board.active_player().invert()); 
                        println!(
                            "{}Draw!", 
                            color::Fg(color::Red),
                        );
                    }
                }
            }
            self.board.print(self.board.active_player());
            let user_input = get_input();
            let user_move = match parse_simple_input(&user_input, self.board.active_player()) {
                Ok(n) => n,
                Err(msg) => {
                    println!("{}{}", color::Fg(color::Red), msg);
                    print!("{}", color::Fg(color::Reset));
                    continue;
                }
            };
            let moves = validator::get_possible_moves_from_square(
               &self.board,
               user_move.old_pos,
               &check_tuple,
            );
            if let Some(valid_move) = moves.iter().find(|m| {
                if let pos::MoveType::Promotion(p) = m.move_type {
                    m.new_pos == user_move.new_pos &&
                    p == user_move.promote_to.unwrap_or(piece::Piece::Queen(self.board.active_player()))
                }
                else{
                    m.new_pos == user_move.new_pos 
                }
            }) {
                self.board.make_move(valid_move);
            }
            else {
                
                println!("{}Invalid Move!", color::Fg(color::Red));
                print!("{}", color::Fg(color::Reset));
            }
            
        }
    }
}
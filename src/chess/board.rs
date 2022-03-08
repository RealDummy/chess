use termion::{color};

use crate::chess::{pos, piece};

use piece::{Piece, Player};

use std::vec::Vec;
type Squares = [[piece::Piece; 8]; 8];
//A chess piece, and where it is on the board
//used in a vector for a piece orineted view of the board
#[derive(Clone)]
pub struct PieceState {
    pub piece_type: Piece,
    pub pos: pos::Square,
}

//players castleing rights, it should be noted that 
//the kindside and queenside are mirrored for the diff. colors
#[derive(Clone)]
struct CastleRights {
    pub kingside: bool,
    pub queenside: bool,
}

// gcm of the dist between 2 pieces lookup table
const FACTOR_LOOKUP: [[i8; 8]; 8] = [
    [8, 1, 2, 3, 4, 5, 6, 7],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [2, 1, 2, 1, 2, 1, 2, 1],
    [3, 1, 1, 3, 1, 1, 3, 1],
    [4, 1, 2, 1, 4, 1, 2, 1],
    [5, 1, 1, 1, 1, 5, 1, 1],
    [6, 1, 2, 3, 2, 1, 6, 1],
    [7, 1, 1, 1, 1, 1, 1, 7],
];

// used to figure out if a piece can attack another piece, probably stupid
pub fn chess_factor(old_pos: pos::Square, new_pos: pos::Square) -> pos::Square {
    let rank_diff = new_pos.rank - old_pos.rank;
    let file_diff = new_pos.file - old_pos.file;
    let dr = rank_diff.abs() as usize;
    let df = file_diff.abs() as usize;
    let gcm = FACTOR_LOOKUP[dr][df];
    pos::Square {
        rank: rank_diff / gcm,
        file : file_diff / gcm, 
    }
}

//generates a piece oriented view from a square oriented view
fn pieces_generator(player: piece::Player, squares: &Squares) -> std::vec::Vec::<PieceState> {
    squares.iter()
                .flatten()
                .enumerate()
                .filter_map(|(i,&p)| {
                    if p.owner() == Some(player) {
                        Some(PieceState {
                        pos: pos::Square {
                            file: (i % 8 + 1) as i8,
                            rank: (i / 8 + 1) as i8,
                        },
                        piece_type: p,
                        })
                    } else {
                        None
                    }
                })
                .collect()
}

impl PieceState {
    //ignores enemy kings, cuz it basically only is for finding illegal king moves
    pub fn is_attacking(&self, board: &Board, square: pos::Square) -> bool {
        // check if piece can move to that square
        let move_dir = match self.piece_type {
            Piece::Empty => {return false;},
            Piece::Pawn(pc) => {
                return ((square.file - self.pos.file).abs()) == 1 && ((square.rank - self.pos.rank) == (1 * pc.sign()));
            },
            Piece::King(_) => {
                return (square.file - self.pos.file).abs() <= 1 && (square.rank - self.pos.rank).abs() <= 1
            },
            Piece::Knight(_) => {
                let pos::Square{rank: dr, file: df} = square - self.pos;
                return std::cmp::max(dr.abs(), df.abs()) == 2 && std::cmp::min(dr.abs(), df.abs()) == 1;
            }
            Piece::Queen(_) => {
                let factored = chess_factor(self.pos, square);
                if factored.rank.abs() <= 1 && factored.file.abs() <= 1 {
                    factored
                }
                else{
                    return false;
                }
            },
            Piece::Bishop(_) => {
                let factored = chess_factor(self.pos, square);
                if factored.rank.abs() == 1 && factored.file.abs() == 1 {
                    factored
                }
                else{
                    return false;
                }
            }
            Piece::Rook(_) => {
                let factored = chess_factor(self.pos, square);
                if (factored.rank == 0 && factored.file.abs() == 1) 
                || (factored.rank.abs() == 1 && factored.file == 0) {
                    factored
                }
                else{
                    return false;
                }
            }
        };
        // check if any pieces are in the way
        let mut temp_square = self.pos + move_dir;
        while temp_square != square {
            if let Some(_) = board.get(temp_square).owner() {
                if board.get(temp_square) != piece::Piece::King(self.piece_type.owner().unwrap().invert()) {
                    return false;
                }
            }
            temp_square += move_dir;
        }
        true
    }
}

#[derive(Clone)]
pub struct Board{
    squares : Squares,
    move_count: u32,
    last_pawn_move: u32,
    last_move: Option::<pos::Move>,
    white_pieces: Vec::<PieceState>,
    black_pieces: Vec::<PieceState>,
    active_player: piece::Player,
    white_castle_rights: CastleRights,
    black_castle_rights: CastleRights, 
}

impl Board{
    pub fn new() -> Self{
        let back_rank = |p : Player| {
            [
                Piece::Rook,
                Piece::Knight,
                Piece::Bishop,
                Piece::Queen,
                Piece::King,
                Piece::Bishop,
                Piece::Knight,
                Piece::Rook,
            ]
            .map(|t| t(p))
        };
        
        let empty_rank = || {[Piece::Empty; 8]};
        let pawn_rank = |p| {[Piece::Pawn(p); 8]};
        let  squares = [
            back_rank(piece::Player::White),
            pawn_rank(Player::White),
            empty_rank(), empty_rank(), empty_rank(), empty_rank(),
            pawn_rank(Player::Black),
            back_rank(Player::Black),
        ];

        Board{
            squares,
            move_count: 0,
            last_pawn_move: 0,
            last_move: None,
            white_pieces: pieces_generator(Player::White, &squares),
            black_pieces: pieces_generator(Player::Black, &squares),
            active_player: piece::Player::White,
            white_castle_rights: CastleRights {queenside:true, kingside: true},
            black_castle_rights: CastleRights {queenside:true, kingside: true},
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        use piece::Piece::*;
        use piece::Player::*;
        let mut squares = [[Empty; 8]; 8];
        let mut rank: usize = 7;
        let mut file: usize = 0;
        let mut chiter = fen.chars();

        //set up board
        chiter.any(|c| {
            match c {
                '1'..='8' => {file += c.to_digit(10).unwrap() as usize - 1;},
                'k' => {squares[rank][file] = King(Black);},
                'K' => {squares[rank][file] = King(White);},
                'q' => {squares[rank][file] = Queen(Black);},
                'Q' => {squares[rank][file] = Queen(White);},
                'r' => {squares[rank][file] = Rook(Black);},
                'R' => {squares[rank][file] = Rook(White);},
                'b' => {squares[rank][file] = Bishop(Black);},
                'B' => {squares[rank][file] = Bishop(White);},
                'n' => {squares[rank][file] = Knight(Black);},
                'N' => {squares[rank][file] = Knight(White);},
                'p' => {squares[rank][file] = Pawn(Black);},
                'P' => {squares[rank][file] = Pawn(White);},
                '/' => {return false;},
                ' ' => {return true;},
                _ => {return false},
            }
            file += 1;
            if file > 7 && rank > 0 {
                file = 0;
                rank -= 1;
            }
            false
        });
        //get active player
        let mut active_player = piece::Player::White;
        chiter.any(|c| {
            if c == ' ' {
                return true;
            }
            active_player = match c {
                'b' => piece::Player::Black,
                'w' | _ => piece::Player::White,
            };
            false

        });
        //castling rights
        let mut black_castle_rights = CastleRights {kingside: false, queenside: false};
        let mut white_castle_rights = CastleRights {kingside: false, queenside: false};
        chiter.any(|c| {
            if c == ' ' {
                return true;
            }
            match c {
                'q' => {black_castle_rights.queenside = true;}
                'k' => {black_castle_rights.kingside = true;},
                'Q' => {white_castle_rights.queenside = true;},
                'K' => {white_castle_rights.kingside = true;},
                _ => (),
            }
            false
        });
        let last_move_pos = pos::Square::from_string(
            &chiter.as_str()[0..2]
        ).ok();
        let last_move = if let Some(m) = last_move_pos {
            Some(pos::Move{
                old_pos: m - pos::Square {rank: 2, file: 0},
                new_pos: m + pos::Square {rank: 1, file: 0},
                move_type: pos::MoveType::Move,
                piece: piece::Piece::Pawn(active_player.invert()),
            })
        } else {
            None
        };
        chiter.any(|c| c == ' ');
        
        let last_pawn_move: String = chiter.by_ref().take_while(|c| c.is_ascii_digit()).collect();
        let last_pawn_move: u32 = last_pawn_move.parse().ok().unwrap_or(0);

        let move_count: String = chiter.by_ref().take_while(|d| d.is_ascii_digit()).collect();
        let move_count: u32 = move_count.parse().ok().unwrap_or(0) * 2 
            + if active_player == piece::Player::Black {1} else {0};
        return Self {
            squares,
            white_pieces: pieces_generator(White, &squares),
            black_pieces: pieces_generator(Black, &squares),
            last_move,
            move_count,
            last_pawn_move,
            active_player,
            white_castle_rights,
            black_castle_rights,
        }

    }

    pub fn any_piece_attacking_except(&self, enemy: Player, square: pos::Square, except: pos::Square) -> bool {
        self.get_pieces(enemy)
        .iter()
        .filter(|piece| piece.pos != except)
        //if any enemy piece is atacking the where the king could be
        .any(|enemy_piece| enemy_piece.is_attacking(&self, square))
    }

    pub fn any_piece_attacking(&self, enemy: Player, square: pos::Square) -> bool {
        self.get_pieces(enemy)
        .iter()
        //if any enemy piece is atacking the where the king could be
        .any(|enemy_piece| enemy_piece.is_attacking(&self, square))
    }
    pub fn has_castle_rights(&self, player: piece::Player, rook_pos: pos::Square) -> bool {
        match rook_pos.file {
            8 => match player {
                piece::Player::White => self.white_castle_rights.kingside,
                piece::Player::Black => self.black_castle_rights.kingside,
            }
            1 => match player {
                piece::Player::White => self.white_castle_rights.queenside,
                piece::Player::Black => self.black_castle_rights.queenside,
            }
            _ => false
        }
    }

    fn player_piece_at(&self, player_color: Player, pos: pos::Square) -> Option::<usize> {
        self.get_pieces(player_color)
                    .iter()
                    .position(|p| {
                        p.pos == pos
                    })
    }
    pub fn print(&self, active_player: piece::Player){
        match active_player {
            piece::Player::White => {
                println!("  ┏━━━━━━━━━━━━━━━━━━━━━━━━┓");
                self.squares.iter()
                    .enumerate()
                    .rev()
                    .for_each(|(i, rank)| {
                        print!("{} ┃",i + 1);
                        print!("{}", color::Fg(color::White));
                        rank.iter()
                            .enumerate()
                            .for_each(|(j,p)| {
                                let fill = if let Some(m) = &self.last_move{
                                    if m.new_pos.rank == (i + 1) as i8 && m.new_pos.file == (j + 1) as i8 {true} else {false}
                                }
                               else {
                                   false
                               };
                                match (i%2) ^ (j%2){
                                    0 => print!("{}{}{}{}", 
                                            color::Bg(color::Black),
                                            if fill {'['} else {' '},
                                            p.to_char(),
                                            if fill {']'} else {' '},
                                    ),
                                    _ => print!("{}{}{}{}", 
                                            color::Bg(color::LightBlack),
                                            if fill {'['} else {' '},
                                            p.to_char(),
                                            if fill {']'} else {' '},
                                    ),
                                }
                            });
                        print!("{}", color::Fg(color::Reset));
                        println!("{}┃", color::Bg(color::Reset));
                    });
                println!("  ┗━━━━━━━━━━━━━━━━━━━━━━━━┛");
                println!("    A  B  C  D  E  F  G  H ");
            },
            piece::Player::Black => {
                println!("  ┏━━━━━━━━━━━━━━━━━━━━━━━━┓");
                self.squares.iter()
                    .enumerate()
                    .for_each(|(i,rank)| {
                        print!("{} ┃",i + 1);
                        print!("{}", color::Fg(color::White));
                        rank.iter()
                            .enumerate()
                            .rev()
                            .for_each(|(j,p)| {
                                let fill = if let Some(m) = &self.last_move{
                                    if m.new_pos.rank == (i + 1) as i8 && m.new_pos.file == (j + 1) as i8 {true} else {false}
                                }
                                else{
                                    false
                                };
                                match (i%2) ^ (j%2){
                                    0 => print!("{}{}{}{}", 
                                            color::Bg(color::Black),
                                            if fill {'['} else {' '},
                                            p.to_char(),
                                            if fill {']'} else {' '},
                                    ),
                                    _ => print!("{}{}{}{}", 
                                            color::Bg(color::LightBlack),
                                            if fill {'['} else {' '},
                                            p.to_char(),
                                            if fill {']'} else {' '},
                                    ),
                                }
                            });
                        print!("{}", color::Fg(color::Reset));
                        println!("{}┃", color::Bg(color::Reset));
                    });
                println!("  ┗━━━━━━━━━━━━━━━━━━━━━━━━┛");
                println!("    H  G  F  E  D  C  B  A ");
            }
        };
        
    }
    pub fn en_passant_possible(&self, player: piece::Player, attacking_pawn_pos: pos::Square, end: pos::Square) -> bool {
        if self.last_pawn_move < self.move_count {
            false
        }
        else {
            if attacking_pawn_pos.rank == match player {
                piece::Player::White => 5,
                piece::Player::Black => 4,
            }
            {
                if let Some(last_moved_pawn) = &self.last_move {
                    if (last_moved_pawn.new_pos - last_moved_pawn.old_pos).rank == 2 {
                        end - pos::Square{rank: player.sign(), file: 0} == last_moved_pawn.new_pos
                    }
                    else{
                        false
                    }
                }
                else {
                    false
                }
            }
            else {
                false
            }
        }
    }
    // moves a piece and updates its piece state in the piece oriented view
    fn move_piece(&mut self, owner: Player, old_pos: pos::Square, new_pos: pos::Square) {
        let index = self.player_piece_at(owner, old_pos).unwrap();
        let mut state = &mut self.get_pieces_mut(owner)[index];
        state.pos = new_pos; 
        self.force_move(old_pos, new_pos);
    }

    pub fn in_check(&self, player_color: Player) -> (Option::<pos::Square>, std::vec::Vec::<pos::Square>) {
        
        //find king's position
        let king_pos = match self.get_pieces(player_color)
        .iter()
        .find(|p| {
            if let Piece::King(_) = p.piece_type {
                true
            } 
            else {
                false
            }
        }) {
            Some(king) => king.pos,
            None => {return (None, vec![])},
        };
        // any enemy pieces atacking king?
        let res_vec = self.get_pieces(player_color.invert())
        .iter()
        .filter(|p| p.is_attacking(&self, king_pos) )
        .map(|p| p.pos)
        .collect::<std::vec::Vec::<pos::Square>>();
        (Some(king_pos), res_vec)

    }
    pub fn get(&self, pos: pos::Square) -> Piece {
        self.squares[pos.get_rank() - 1][pos.get_file() - 1]
    }
    fn set(&mut self, pos: pos::Square, p:Piece) -> Piece {
        let res = self.squares[pos.get_rank() - 1][pos.get_file() - 1];
        self.squares[pos.get_rank() - 1][pos.get_file() - 1] = p;
        return res;
    }
    //makes a chess move, panics if not legal. Ensure move is legal with legal_move first!
    pub fn make_move(&mut self, valid_move: &pos::Move){
        match valid_move.move_type {
            pos::MoveType::Capture(captured) => {
                let owner = valid_move.piece.owner().unwrap();
                let captured_pos = self.get_pieces(captured.owner().expect("Board Mangled"))
                    .iter()
                    .position(|p| {
                        p.pos == valid_move.new_pos
                    }).expect("Board Mangled");
                
                self.get_pieces_mut(captured.owner().expect("Board Mangled"))
                    .swap_remove(captured_pos);
                    
                self.move_piece(owner, valid_move.old_pos, valid_move.new_pos);
            }
            pos::MoveType::Castle(rook_pos) => {
                let dir = pos::Square{
                    rank: 0,
                    file: if rook_pos.file == 8 {1} else {-1},
                };
                let owner = valid_move.piece.owner().expect("Board Mangled");

                //move rook
                self.move_piece(owner, rook_pos, valid_move.new_pos - dir);
                //move king
                self.move_piece(owner, valid_move.old_pos, valid_move.new_pos);
            }
            pos::MoveType::EnPassant => {
                let owner = valid_move.piece.owner().expect("Board Mangled");
                
                let captured = owner.invert();
                let captured_pos = self.get_pieces(captured)
                    .iter()
                    .position(|p| {
                        p.pos == valid_move.new_pos - pos::Square {
                            rank: owner.sign(),
                            file: 0,
                        }
                    }).expect("Board Mangled");
                
                //remove Pawn from game
                let cap_square = self.get_pieces_mut(captured)
                    .swap_remove(captured_pos).pos;
                self.set(cap_square, piece::Piece::Empty);

                self.move_piece(owner, valid_move.old_pos, valid_move.new_pos);
            },
            pos::MoveType::Move => {
                let owner = valid_move.piece.owner().expect("Board Mangled");
                self.move_piece(owner, valid_move.old_pos, valid_move.new_pos);
            },
            pos::MoveType::Promotion(to_piece) => {
                let owner = valid_move.piece.owner().expect("Board Mangled");
                let pawn_index = self.player_piece_at(owner, valid_move.old_pos).expect("Board Mangled");
                let pawn_state = &mut self.get_pieces_mut(owner)[pawn_index];
                pawn_state.pos = valid_move.new_pos;
                pawn_state.piece_type = to_piece;
                self.set(valid_move.new_pos, to_piece);
                self.set(valid_move.old_pos, piece::Piece::Empty);
            },
        }
        self.move_count += 1;
        self.last_move = Some(valid_move.clone());
        if let piece::Piece::Pawn(_) = valid_move.piece {
            self.last_pawn_move = self.move_count;
        }
        self.active_player = self.active_player.invert();
    }
    //for seeing if the king is in check after a move
    pub fn king_safe_after_move(&mut self, king_pos: pos::Square, temp_move: pos::Move) -> Option::<pos::Move>{
        use pos::MoveType::*;
        let king_safe = match temp_move.move_type {
           Castle(_) => {return Some(temp_move);},
           EnPassant => {
               self.force_move(temp_move.old_pos, temp_move.new_pos);
               let captured_pawn = self.set(
                   temp_move.new_pos - pos::Square{rank: temp_move.piece.owner().unwrap().sign(), file: 0}, 
                    piece::Piece::Empty
                );

                let king_safe = !self.any_piece_attacking(self.active_player().invert(), king_pos);
                self.force_move(temp_move.new_pos, temp_move.old_pos);
                self.set(
                    temp_move.new_pos - pos::Square{rank: temp_move.piece.owner().unwrap().sign(), file: 0}, 
                    captured_pawn
                );
                king_safe
            },
           _ => {
               if let piece::Piece::King(pc) = temp_move.piece {
                    !self.any_piece_attacking(pc.invert(), temp_move.new_pos)
                }
                else{
                    let captured_piece = self.force_move(temp_move.old_pos, temp_move.new_pos);
                    let king_safe = !self.any_piece_attacking_except(self.active_player().invert(), king_pos, temp_move.new_pos);
                    self.force_move(temp_move.new_pos, temp_move.old_pos);
                    self.set(temp_move.new_pos, captured_piece);
                    king_safe
                }
            }
        };
        if king_safe {
            Some(temp_move)
        }
        else{
            None
        }
    }

    fn force_move(&mut self, old_pos: pos::Square, new_pos: pos::Square) -> Piece{
        let old_piece = self.get(old_pos);
        self.set(old_pos, Piece::Empty);
        return self.set(new_pos, old_piece);
    }
    pub fn get_pieces(&self, pc: Player) -> &Vec::<PieceState> {
        match pc {
            Player::White => &self.white_pieces,
            Player::Black => &self.black_pieces,
        }
    }
    fn get_pieces_mut(&mut self, pc: Player) -> &mut Vec::<PieceState> {
        match pc {
            Player::White => &mut self.white_pieces,
            Player::Black => &mut self.black_pieces,
        }
    }
    pub fn active_player(&self) -> piece::Player {
        self.active_player
    }
}
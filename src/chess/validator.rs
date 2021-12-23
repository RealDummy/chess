use crate::chess::{board, piece, pos};
use std::vec::Vec;

const PAWN_MOVES: &'static [(i8,i8)] = &[(0,1),(0,1),(0,1),(0,1),(0,1),(0,2),(1,1),(-1,1),];

const KNIGHT_MOVES: &'static [(i8,i8)] = &[
    ( 1,  2), ( 2,  1),
    ( 2, -1), ( 1, -2),
    (-1, -2), (-2, -1), 
    (-2,  1), (-1,  2)
];
const BISHOP_MOVES: &'static [(i8,i8)] = &[
    ( 1,  1),( 2,  2),( 3,  3),( 4,  4),( 5,  5),( 6,  6),( 7,  7),
    (-1,  1),(-2,  2),(-3,  3),(-4,  4),(-5,  5),(-6,  6),(-7,  7),
    ( 1, -1),( 2, -2),( 3, -3),( 4, -4),( 5, -5),( 6, -6),( 7, -7),
    (-1, -1),(-2, -2),(-3, -3),(-4, -4),(-5, -5),(-6, -6),(-7, -7),
];
const ROOK_MOVES: &'static [(i8,i8)] = &[
    ( 0,  1), ( 0,  2), ( 0,  3), ( 0,  4), ( 0,  5), ( 0,  6), ( 0,  7),
    ( 0, -1), ( 0, -2), ( 0, -3), ( 0, -4), ( 0, -5), ( 0, -6), ( 0, -7),
    ( 1,  0), ( 2,  0), ( 3,  0), ( 4,  0), ( 5,  0), ( 6,  0), ( 7,  0),
    (-1,  0), (-2,  0), (-3,  0), (-4,  0), (-5,  0), (-6,  0), (-7,  0),
    
];
const QUEEN_MOVES: &'static [(i8,i8)] = &[
    ( 1,  1), ( 2,  2), ( 3,  3), ( 4,  4), ( 5,  5), ( 6,  6), ( 7,  7),
    (-1,  1), (-2,  2), (-3,  3), (-4,  4), (-5,  5), (-6,  6), (-7,  7),
    ( 1, -1), ( 2, -2), ( 3, -3), ( 4, -4), ( 5, -5), ( 6, -6), ( 7, -7),
    (-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7),
    ( 0,  1), ( 0,  2), ( 0,  3), ( 0,  4), ( 0,  5), ( 0,  6), ( 0,  7),
    ( 0, -1), ( 0, -2), ( 0, -3), ( 0, -4), ( 0, -5), ( 0, -6), ( 0, -7),
    ( 1,  0), ( 2,  0), ( 3,  0), ( 4,  0), ( 5,  0), ( 6,  0), ( 7,  0),
    (-1,  0), (-2,  0), (-3,  0), (-4,  0), (-5,  0), (-6,  0), (-7,  0),
];
const KING_MOVES: &'static [(i8,i8)] = &[
    (-1,  1), ( 0,  1), ( 1,  1),  
    (-1,  0),           ( 1,  0),
    ( 1, -1), ( 0, -1), (-1, -1),
    ( 2,  0), (-2,  0),
];

fn player_sign(piece: piece::Piece) -> i8 {
    match piece.owner() {
        None => 0,
        Some(p) => p.sign(),
    }
}

fn on_board(p: &pos::Square) -> bool {
    p.rank <= 8 && p.rank >= 1 && p.file <= 8 && p.file >= 1
}

fn move_diff_sign(dy: i8, dx: i8) -> pos::Square {
    pos::Square{
        rank: dy.signum() as i8, 
        file: dx.signum() as i8,
    }
}

fn get_move_dir(old_pos: pos::Square, new_pos: pos::Square) -> pos::Square{
    let pos::Square{ rank: y1, file: x1 } = old_pos; 
    let pos::Square{ rank: y2, file: x2 } = new_pos;
    move_diff_sign(
        y2 - y1,
        x2 - x1,
    )
}

pub fn legal_move(board: &board::Board, move_candidate: pos::MoveCandidate,
    check_tuple: &(Option::<pos::Square>, std::vec::Vec::<pos::Square>) ) -> Option<pos::Move> {   
    let pos::MoveCandidate{ old_pos, new_pos, promote_to } = move_candidate;
    let piece_type = board.get(old_pos);
    use piece::Piece::*;
    if let Empty = piece_type {
        return None;
    };
    if piece_type.owner() == board.get(new_pos).owner() {
        return None;
    }

    let potential_move = match piece_type {
        Pawn(pc) => {
            
            let enemy_piece_type = board.get(new_pos);
            match enemy_piece_type.owner() {
                //not attacking(or en passant attacking)
                None => {
                    if get_move_dir(old_pos, new_pos).file != 0 { //en passant
                        if board.en_passant_possible(pc,old_pos) {
                            Some(pos::Move{
                                move_type: pos::MoveType::EnPassant,
                                old_pos,
                                new_pos,
                                piece: piece_type,
                            })
                        }
                        else {
                            return None;
                        }
                    }
                    else {
                        let move_type = if new_pos.rank == 8 || new_pos.rank == 1 {
                            pos::MoveType::Promotion(match promote_to {
                                Some(p) => p,
                                None => piece::Piece::Queen(board.active_player()),
                            })
                        } else {
                            pos::MoveType::Move
                        };
                        Some(pos::Move{
                            move_type,
                            old_pos,
                            new_pos,
                            piece: piece_type,
                        })
                    }
                },
                //atacking
                Some(_) => {
                    if get_move_dir(old_pos, new_pos).file == 0 {
                        return None;
                    }
                    else{
                        Some(pos::Move{
                            move_type: pos::MoveType::Capture(enemy_piece_type),
                            old_pos,
                            new_pos,
                            piece: piece_type,
                        })
                    } 
                }
            }
        },
        Knight(_) => {
            let enemy_piece_type = board.get(new_pos);
            match enemy_piece_type.owner() {
                None => Some(pos::Move {
                    move_type: pos::MoveType::Move,
                    old_pos,
                    new_pos,
                    piece: piece_type,
                }),
                Some(_) => Some(pos::Move{
                    move_type: pos::MoveType::Capture(enemy_piece_type),
                    old_pos,
                    new_pos,
                    piece: piece_type,
                })
            }
        },
        Bishop(_) | Rook(_) | Queen(_) => {
            let dir = get_move_dir(old_pos, new_pos);
            let mut sliding_square = old_pos + dir;
            while sliding_square != new_pos {
                if let Some(_) = board.get(sliding_square).owner(){
                    return None;
                }
                sliding_square += dir;
            }
            let enemy_piece_type = board.get(new_pos);
            match enemy_piece_type.owner() {
                None => Some(pos::Move {
                    move_type: pos::MoveType::Move,
                    old_pos,
                    new_pos,
                    piece: piece_type,
                }),
                Some(_) => Some(pos::Move{
                    move_type: pos::MoveType::Capture(enemy_piece_type),
                    old_pos,
                    new_pos,
                    piece: piece_type,
                })
            }
        },
        King(pc) => {
             //first check if new_pos is under attack!
             if board.any_piece_attacking(pc.invert(), new_pos) {
                return None;
            }
            //check if rook/king has moved
            match (new_pos - old_pos).file {
                2|-2 => { //king castles
                    //no castleing in check
                    if let (Some(_), _) = check_tuple {
                        if check_tuple.1.len() > 0{
                            return None;
                        }
                    }
                    let dir = get_move_dir(old_pos, new_pos);
                    
                    //find where rook is
                    let rook_pos = match dir {
                        pos::Square{rank:_, file: 1} => {
                            pos::Square{
                                rank: old_pos.rank,
                                file: 8,
                            }
                        },
                        pos::Square{rank:_, file: -1} => {
                            pos::Square{
                                rank: old_pos.rank,
                                file: 1,
                            }
                        },
                        _ => {
                            println!("how did we get here");
                            return None;
                        }
                    };
                    //make sure castleing rights are preserved
                    if !board.has_castle_rights(
                        board.active_player(),
                        rook_pos
                    ) {
                        return None;
                    }
                    
                    //iterate over squares between king and king's target 
                    //to make sure it doesnt travel through check
                    let mut sliding_square = old_pos + dir;
                    while sliding_square != new_pos {
                        if let Some(_) = board.get(sliding_square).owner() {
                            return None;
                        }
                        if board.any_piece_attacking(pc.invert(), sliding_square) {
                            return None;
                        }
                        sliding_square += dir;
                    }

                    //make sure rook is where it should be(probably not required)
                    if board.get(rook_pos) != Rook(pc) {
                        return None;
                    }
                    //make sure castling side is empty
                    while sliding_square != rook_pos {
                        if let Some(_) = board.get(sliding_square).owner() {
                            return None
                        }
                        sliding_square += dir;
                    }
                    Some(pos::Move{
                        move_type: pos::MoveType::Castle(rook_pos),
                        old_pos,
                        new_pos,
                        piece: piece_type,
                    })
                }
                
                _ => { //any other king move
                    let enemy_piece_type = board.get(new_pos);
                    //move already known to not move into check
                    match enemy_piece_type.owner() {
                        None => Some(pos::Move {
                            move_type: pos::MoveType::Move,
                            old_pos,
                            new_pos,
                            piece: piece_type,
                        }),
                        Some(_) => {
                            Some(pos::Move{
                                move_type: pos::MoveType::Capture(enemy_piece_type),
                                old_pos,
                                new_pos,
                                piece: piece_type,
                            })
                        }
                    }
                }
            }
        },
        Empty => {return None;}
    }.unwrap();

    if let (Some(king_pos), check_vec) = check_tuple {
        if check_vec.len() == 0 || *king_pos == potential_move.old_pos {
            return Some(potential_move);
        }
        for &check_square in check_vec {
            let mut sliding_square = check_square;
            use piece::Piece::*;
            match board.get(check_square) {
                Empty => {
                    println!("oh no");
                    return None;
                }
                Knight(_) => {
                    if potential_move.new_pos != check_square {
                        return None;
                    }
                }
                _ => { //is a sliding piece or pawn
                    let dir = get_move_dir(check_square, *king_pos);
                    let mut curr_check_blocked = false;
                    while sliding_square != *king_pos {
                        if potential_move.new_pos == sliding_square {
                            curr_check_blocked = true;
                            break;
                        }
                        sliding_square += dir;
                    }
                    if !curr_check_blocked {
                        return None;
                    }
                }

            }
            
        }
    }
    Some(potential_move)
}

pub fn get_possible_moves_from_square(
    board: &board::Board, 
    piece_pos: pos::Square, 
    check_tuple: &(Option::<pos::Square>, std::vec::Vec::<pos::Square>)
) -> Vec::<pos::Move> {
    let piece_type = board.get(piece_pos);
    if piece_type.owner() != Some(board.active_player()) {
        return vec![];
    }
    use piece::Piece::*;
    let mut last_pawn_promotion: Option::<piece::Piece> = Some(Empty); //invalid state that gets turned to none later
    match piece_type {
        Empty     => &[],
        Pawn(_)   => PAWN_MOVES,
        Knight(_) => KNIGHT_MOVES,
        Bishop(_) => BISHOP_MOVES,
        Rook(_)   => ROOK_MOVES,
        Queen(_)  => QUEEN_MOVES,
        King(_)   => KING_MOVES,
    }
        .iter()
        .map(|(df, dr)| {
            pos::Square{
                file: piece_pos.file + df * player_sign(piece_type),
                rank: piece_pos.rank + dr * player_sign(piece_type),
            }
        })
        .filter(|p| {
            on_board(p)
        })
        .filter_map(|new_pos| {
            last_pawn_promotion = if let Pawn(_) = piece_type {
                match last_pawn_promotion {
                    None             => Some(Queen(board.active_player())),
                    Some(Queen(pc))  => Some(Rook(pc)),
                    Some(Rook(pc))   => Some(Bishop(pc)),
                    Some(Bishop(pc)) => Some(Knight(pc)),
                    _ => None
                }
            }
            else {
                None
            };
            let move_candidate = pos::MoveCandidate {
                old_pos: piece_pos,
                new_pos,
                promote_to: last_pawn_promotion,
            };
            legal_move(board, move_candidate, check_tuple)
        })
        .collect()
}
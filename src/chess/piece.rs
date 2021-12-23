#[derive(Copy, Clone, PartialEq)]
pub enum Player{
    Black,
    White,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece{
    Empty,
    Pawn(Player),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
}



impl Player {
    pub fn invert(&self) -> Self{
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
    pub fn sign(&self) -> i8 {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }
}

impl Piece{
    pub fn to_char(&self)-> char {
        match self {
            Piece::Pawn(p) => {
                match p {
                    Player::Black => '♙',
                    Player::White => '♟',
                }
            }
            Piece::Knight(p) => {
                match p {
                    Player::Black => '♘',
                    Player::White => '♞',
                }
            }
            Piece::Bishop(p) => {
                match p {
                    Player::Black => '♗',
                    Player::White => '♝',
                }
            }
            Piece::Rook(p) => {
                match p {
                    Player::Black => '♖',
                    Player::White => '♜',
                }
            }
            Piece::Queen(p) => {
                match p {
                    Player::Black => '♕',
                    Player::White => '♛',
                }
            }
            Piece::King(p) => {
                match p {
                    Player::Black => '♔',
                    Player::White => '♚',
                }
            }
            _ => ' '
        }
    }
    pub fn owner(&self) -> Option::<Player> {
        match self {
            Self::Empty => None,
            Self::Pawn(p) | Self::Knight(p) 
            | Self::Bishop(p) | Self::Rook(p) 
            | Self::Queen(p) | Self::King(p) 
                => Some(*p),
        }
    }
}

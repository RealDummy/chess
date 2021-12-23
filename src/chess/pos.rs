
use crate::chess::piece;

#[derive(Copy,Clone,PartialEq)]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}

impl Square {
    pub fn new(r: usize, f: usize) -> Self { 
        let rank: i8 = r.try_into().expect("Bad rank number");
        let file: i8 = f.try_into().expect("Bad file number");

        Self {
            rank,
            file,
        }
    }

    pub fn from_string(pos: &str) -> Result::<Self, String> {
        if pos.len() < 2 {
            return Err("input too short".to_string());
        }
        let mut char_iter = pos.chars();
        let char_file = char_iter.next().unwrap().to_ascii_lowercase();
        let char_rank = char_iter.next().unwrap();

        if !char_file.is_ascii_alphabetic() || !char_rank.is_ascii_digit(){
            return Err("invalid character".to_string());
        }
        Ok(Self {
            rank: char_rank.to_digit(10).unwrap() as i8,
            file: match char_file.to_digit(18) {
                Some(n) => (n - 9) as i8, 
                None => {return Err("Invalid character".to_string());},
            }
        })
    }

    pub fn get_rank(&self) -> usize {
       self.rank as usize
    }
    pub fn get_file(&self) -> usize {
        self.file as usize
    }
}

impl std::ops::Add for Square {
    type Output = Self;
    fn add(self, rhs: Square) -> Self{
        Self {
            rank: self.rank + rhs.rank,
            file: self.file + rhs.file,
        }
    }
}

impl std::ops::Sub for Square {
    type Output = Self;
    fn sub(self, rhs: Square) -> Self{
        Self {
            rank: self.rank - rhs.rank,
            file: self.file - rhs.file,
        }
    }
}

impl std::ops::Add for &Square {
    type Output = Square;
    fn add(self, rhs: &Square) -> Square{
        Square {
            rank: self.rank + rhs.rank,
            file: self.file + rhs.file,
        }
    }
}

impl std::ops::AddAssign::<Self> for Square {
    fn add_assign(&mut self, rhs: Self) {
        self.rank += rhs.rank;
        self.file += rhs.file;
    }
}

#[derive(Clone, Copy)]
pub enum MoveType {
    Capture(piece::Piece),
    Castle(Square),
    EnPassant, //needed to keep track of the piece captured on a diff square
    Move,
    Promotion(piece::Piece),
}

#[derive(Clone)]
pub struct Move {
    pub piece: piece::Piece, 
    pub move_type: MoveType,
    pub old_pos: Square, //location of piece before moving
    pub new_pos: Square, //location of piece after moving
}

pub struct MoveCandidate {
    pub old_pos: Square,
    pub new_pos: Square,
    pub promote_to: Option::<piece::Piece>,
}
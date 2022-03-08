
use termion::{color};

mod board;
mod piece;
mod pos;
mod validator;

#[derive(Clone,Copy)]
pub enum GameEval {
	Checkmate(piece::Player),
	Draw,
	Eval(f64),
}

#[derive(Clone)]
pub struct Game{
	board: board::Board,
	eval: Option<GameEval>,
	legal_moves: Option<std::vec::Vec<pos::Move>>
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
	if trimput.len() < 4 {
		return Err("Invalid Input!".to_owned())
	}
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
			eval: None,
			legal_moves: None
		}
	}
	pub fn from_fen(fen: &str) -> Self {
		let board = board::Board::from_fen(fen);
		Self {
			board,
			eval: None,
			legal_moves: None
		}
	}
	pub fn eval(&mut self) -> GameEval {
		self.eval.unwrap_or(GameEval::Eval(0.0))
	}
	pub fn gen_moves(&mut self) {
		self.legal_moves = Some(validator::get_possible_moves(&mut self.board));
	}
	pub fn get_moves(&self) -> &Vec::<pos::Move>{
		self.legal_moves.as_ref().unwrap()
	}
	pub fn make_move(&mut self, legal_move: &pos::Move) {
		self.board.make_move(legal_move);
	}
	pub fn print(&self) {
		self.board.print(self.board.active_player());
	}
	pub fn print_for(&self, p: piece::Player) {
		self.board.print(p);
	}

	pub fn print_possible_moves(&mut self) {
		for m in self.get_moves() {
			let mut temp = self.clone();
			temp.make_move(m);
			temp.print_for(self.board.active_player());
		}
	}

	pub fn play(&mut self) {
		loop {
			let all_legal_moves = validator::get_possible_moves(&mut self.board);
			if all_legal_moves.len() == 0 {            
				if let (Some(_), check_vec) = self.board.in_check(self.board.active_player()) {
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
					} else {
						self.board.print(self.board.active_player().invert()); 
						println!(
							"{}Draw!", 
							color::Fg(color::Red),
						);
					}
				}
			}
			self.gen_moves();
			println!("-----------------------------");
			self.print_possible_moves();
			println!("-----------------------------");
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
			if let Some(valid_move) = all_legal_moves.iter().find(|m| {
				if let pos::MoveType::Promotion(p) = m.move_type {
					m.old_pos == user_move.old_pos &&
					m.new_pos == user_move.new_pos &&
					p == user_move.promote_to.unwrap_or(piece::Piece::Queen(self.board.active_player()))
				}
				else{
					m.old_pos == user_move.old_pos &&
					m.new_pos == user_move.new_pos 
				}
			})  {
				self.board.make_move(valid_move);
			}
			else {

				println!("{}Invalid Move!", color::Fg(color::Red));
				print!("{}", color::Fg(color::Reset));
			}
		}
	}
}
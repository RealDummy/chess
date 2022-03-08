mod chess;
use std::vec::Vec;

fn perft(base_game: &chess::Game, depth: i32) {
    let mut count = 0u64;
    let mut gamestates = Vec::<chess::Game>::new();
    gamestates.push(base_game.clone());
    for _ in 0..depth {
        let mut next_gamestates = Vec::<chess::Game>::new();
        for game in &mut gamestates {
            game.gen_moves();
            for m in game.get_moves() {
                let mut gc = game.clone();
                gc.make_move(m);
                //gc.print();
                next_gamestates.push(gc);
                count += 1;
            }
        }
        gamestates = next_gamestates;
    }

    println!("I have peered into {} positions with depth {}, and {} final positions", 
        count, 
        depth, 
        gamestates.len()
    );
}

fn main() {
    // default a couple hundred less moves at depth 5!
    // "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - " ~100 more moves depth 3
    // "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - " 2 less moves at depth 3
    let mut game = chess::Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");
    //game.play();
    perft(&game, 4);
    
}

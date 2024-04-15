use bevy::utils::petgraph::algo::Matching;
use darts::{game::Game, player::Player};

mod darts;


fn main() {

    let game = Game::new(vec!(
        Player::new("Bengt".to_string(), true),
        Player::new("Joakim".to_string(), true)
    ));

    let mut game = match game {
        Ok(game) => game,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };


    let mut rounds_played = 0;

    loop{

        if rounds_played > 120 {
            println!("Game took too long, exiting");
            break;
        }

        rounds_played += 1;

        match game.play_round() {
            Ok(_) => {
                let round = game.current_round().unwrap();
                println!("---- Round {} ----", round.round_number);
                println!("---- Player {} ----", game.current_player());


                game.players.iter().for_each(|p| {
                    println!("{}: {}", p, p.score);

                    if p.score == 0 {
                        println!("{} wins!", p);

                        // make a csv file with the results
                        
                        let mut csv = "Round, Player, Round score, Score \n".to_owned();


                        game.players.iter().for_each(|p| {
                            p.rounds.iter().for_each(|r| {
                                csv.push_str( format!("{}, {}, {}, {}\n", r.round_number, p, r.score, r.start_score - r.score).as_str());
                            });
                        });

                        std::fs::write("./results.csv", csv).unwrap();

                        std::process::exit(0);
                    }
                });

            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }

        match game.advance_round() {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }


}


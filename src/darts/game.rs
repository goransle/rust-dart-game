use super::rules::DartRules;
use super::round::DartRound;
use super::player::Player;
use std::str::FromStr;
use std::fs;

pub struct Game {
    pub players: Vec<Player>,
    pub current_player: usize,
    pub rules: DartRules,
}

impl Game {
    pub fn new( 
            mut players: Vec<Player>
        ) -> Result<Self, anyhow::Error> {

        if players.len() < 2 {
            return Err(anyhow::Error::msg("At least two players are required"));
        }

        let rules = DartRules::new(false, false, 301);
        let current_player = 0;

        players.iter_mut().for_each(|p| {
            p.set_score(rules.start_score);
            p.rounds.push(DartRound::new(&rules, rules.start_score, 0).unwrap());
        });

        let game = Self {
            players,
            rules,
            current_player
        }; 

        return Ok(game);
    }

    pub fn current_round(&mut self) -> Option<&mut DartRound> {
        return self.players[self.current_player].rounds.last_mut();
    }

    pub fn current_player(&self) -> &Player {
        return &self.players[self.current_player];
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        return &mut self.players[self.current_player];
    }

    pub fn advance_round(&mut self) -> Result<(), anyhow::Error> {
        let next_player = (self.current_player + 1) % self.players.len();

        self.current_player = next_player;

        let is_first_round = self.current_player().rounds.len() == 0;

        let round = DartRound::new(
                &self.rules, 
                if is_first_round { 
                    301 
                } else {
                    self.players[self.current_player].score
                },
                if is_first_round {
                   0
                } else {
                    let last_round = self.current_player().rounds.last();
                    last_round.unwrap().round_number + 1
                }
            )?;

        self.current_player_mut().rounds.push(round);

        return Ok(());
    }

    pub fn play_round(&mut self) -> Result<(), anyhow::Error> {
        const MAX_THROWS: usize = 3;
        let mut throws = 0;

        while throws < MAX_THROWS {
            throws += 1;

            let player = self.current_player_mut();
            let current_round = player.rounds.last_mut();
            let mut random_hit = super::hit::DartHit::rnd();


            match current_round {
                Some(round) => {

                    let result = round.handle_throw(
                        &mut random_hit
                    );

                    if round.score == 0 {
                        break;
                    }

                    if !result {
                        break;
                    }

                    let score = player.score - round.score;

                    if score >= 0 {
                        player.set_score(score);
                    }
                },
                None => {
                    break;
                }

            }




        }



        Ok(())
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut players = vec!();
        let mut rounds = vec!();

        for line in s.lines() {
            match line.split_once(" ") {
                Some((name, score)) => {
                    // use .parse to call FromStr from Player
                    let player = name.parse::<Player>()?; //Player::from_str(name)?;
                    players.push(player);

                    if score == "301" {
                        rounds.push(DartRound::new(&DartRules::new(false, false, 301), 301, 0)?);
                    }
                },
                None => {
                    return Err(anyhow::Error::msg("Invalid input"));
                }
            }
        }

        return Game::new(players);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ok(result: &Result<(), anyhow::Error>) {
        match result {
            Ok(_) => {},
            Err(e) => {
                assert!(false, "Error: {}", e);
            }
        }
    }

    #[test]
    fn test_game() {
        let player1 = Player::new("Player 1".to_string(), false);
        let player2 = Player::new("Player 2".to_string(), true);

        let mut game = Game::new(vec!(player1, player2)).unwrap();

        assert_eq!(game.players.len(), 2);
        assert_eq!(game.current_player, 0);
        assert_eq!(game.current_player().rounds.len(), 1);
        assert_eq!(game.rules.start_score, 301);
        assert_eq!(game.current_player, 0);

        assert_ok(&game.advance_round());
        assert_eq!(game.current_player, 1);

        // assert_eq!(game.current_player().rounds.len(), 1);

        game.play_round().unwrap();

        assert_eq!(game.current_round().unwrap().throws.len(), 3);
    }

    #[test]
    fn test_game_from_str() {
        let game = Game::from_str("Player 1\nPlayer 2").unwrap();

        assert_eq!(game.players.len(), 2);

        let str_from_file = fs::read_to_string("./game.txt").unwrap();

        let game = str_from_file.parse::<Game>();

        match game {
            Ok(game) => {
                assert_eq!(game.players.len(), 2);
                assert_eq!(
                    game.players
                    .into_iter()
                    .map(|p| p.name)
                    .collect::<Vec<String>>(), 
                    vec!("Joakim", "Bengt")
                    );
            },
            Err(e) => {
                assert!(false, "Error: {}", e);
            }
        }



    }
}

use super::rules::DartRules;
use super::round::DartRound;
use super::player::Player;
use std::str::FromStr;
use std::fs;

pub struct Game {
    pub players: Vec<Player>,
    pub current_player: usize,
    pub rounds: Vec<DartRound>,
    pub rules: DartRules,
}

impl Game {
    pub fn new( 
            players: Vec<Player>
        ) -> Result<Self, anyhow::Error> {

        if players.len() < 2 {
            return Err(anyhow::Error::msg("At least two players are required"));
        }

        let rules = DartRules::new(false, false, 301);

        let mut rounds = vec!();

        let mut current_player = 0;


        let game = Self {
            players,
            rules,
            current_player,
            rounds
        }; 

        return Ok(game);
    }

    pub fn next_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    pub fn current_round(&mut self) -> Option<&mut DartRound> {
        return self.rounds.last_mut()
    }

    pub fn advance_round(&mut self) -> Result<(), anyhow::Error> {
        let round = DartRound::new(&self.rules)?;

        self.rounds.push(round);

        return Ok(());
    }

    pub fn play_round(&mut self) -> Result<(), anyhow::Error> {
        const MAX_THROWS: usize = 3;
        let current_round = self.rounds.last_mut().unwrap();

        for mut player in &self.players {
            println!("Player: {}", player);
            let mut throws = 0;

            while throws < MAX_THROWS {
                println!("Throws: {}", throws);
                throws += 1;

                let mut random_hit = super::hit::DartHit::rnd();
                
                let result = current_round.handle_throw(
                    &mut random_hit
                );

                if !result {
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

        for line in s.lines() {
            let (name, score) = line.split_once(" ")
                .ok_or(anyhow::Error::msg("Invalid input"))?;

            let player = Player::from_str(name)?;

            players.push(player);
        }

        return Game::new(players);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let player1 = Player::new("Player 1".to_string(), false);
        let player2 = Player::new("Player 2".to_string(), true);

        let mut game = Game::new(vec!(player1, player2)).unwrap();

        assert_eq!(game.players.len(), 2);
        assert_eq!(game.current_player, 0);
        assert_eq!(game.rounds.len(), 0);
        assert_eq!(game.rules.start_score, 301);

        game.advance_round();

        assert_eq!(game.rounds.len(), 1);

        game.play_round().unwrap();

        assert_eq!(game.current_round().unwrap().throws.len(), 6);
    }

    #[test]
    fn test_game_from_str() {
        let game = Game::from_str("Player 1\nPlayer 2").unwrap();

        assert_eq!(game.players.len(), 2);

        let str_from_file = fs::read_to_string("./game.txt").unwrap();

        let game = Game::from_str(&str_from_file).unwrap();

        assert_eq!(game.players.len(), 2);
        assert_eq!(
            game.players
                .into_iter()
                .map(|p| p.name)
                .collect::<Vec<String>>(), 
            vec!("Joakim", "Bengt")
        );


    }
}

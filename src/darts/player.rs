use std::{fmt, str::FromStr};

use super::round::DartRound;

pub struct Player {
    pub name: String,
    pub computer: bool,
    pub score: i32,
    pub rounds: Vec<DartRound>
}

impl Player {
    pub fn new(name: String, computer: bool) -> Self {
        Player {
            name,
            computer,
            score: 999,
            rounds: vec!()
        }
    }

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            name: self.name.clone(),
            computer: self.computer,
            score: self.score,
            rounds: vec!()
        }
    }
}

// Lets us print the player name by writing println!("{}", player)
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// Lets us create a player from a string
impl FromStr for Player {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(anyhow::anyhow!("Player name cannot be empty"));
        }

        let player = Player::new(s.to_string(), false);

        return Ok(player);
    }
}

// Test can be added in the same file as the code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player() {
        let player = Player::new("Player 1".to_string(), false);

        assert_eq!(player.name, "Player 1");
        assert_eq!(player.computer, false);

        let player_string = player.to_string();

        println!("Player string: {}", player);

        assert_eq!(player_string, "Player 1");
    }
}

use anyhow::Result;

use super::hit::DartHit;
use super::rules::DartRules;
use super::multipliers::DartMultipliers;

pub struct DartRound {
    pub score: i32,
    pub start_score: i32,
    pub throws: Vec<DartHit>,
    pub rules: DartRules,
    pub round_number: i32,
    pub has_busted: bool
}

impl DartRound {
    pub fn new(rules: &DartRules, score: i32, round_number: i32 ) -> Result<Self, anyhow::Error> {

        if rules.start_score < 0 {
            return Err(anyhow::Error::msg("Start score must be greater than 0"));
        }

        let round = Self {
            round_number,
            score,
            start_score: score,
            throws: vec!(),
            rules: rules.clone(),
            has_busted: false
        }; 

        return Ok(round);
    }

   pub fn handle_throw(&mut self, score: &mut DartHit) -> bool {
        let score_diff = self.score - score.get_score();

       // Always a bust
       if score_diff < 0 {
           self.has_busted = true;
            return false;
       }

       // Double in
       if self.rules.double_in && self.rules.start_score == self.score {
           self.score = match score.multiplier {
                DartMultipliers::Double => score_diff,
                _ => self.score
           };

           return true;
       }

       // Double out
       self.has_busted =
           match self.rules.double_out {
               true => match score_diff {
                   0 => match score.multiplier {
                       DartMultipliers::Double => false, // if 0 and hit is a double, you win
                       _ => true
                   },
                   1 => true, // is a bust
                   _ => false // can still play out
               },
               false => false
           };

       if self.score < 0 {
           self.has_busted = true;
       }

       if self.has_busted {
            println!(".......Bust!");
       }

       if !self.has_busted {
           self.score = self.score - score_diff;  
           self.throws.push(score.clone());

           return true;
       }

       return false;
   } 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dart_round() {

        let rules = DartRules::new(true, true, 301);
        let mut round = DartRound::new(&rules, 301, 0).unwrap();

        // Double in should count
        let mut hit = DartHit::new(20, DartMultipliers::Double);
        assert_eq!(round.handle_throw(&mut hit), true);
        assert_eq!(round.score, 301 - 40);

        let rules = &DartRules::new(true, true, 301);
        let mut round = DartRound::new(rules, 301, 0).unwrap();

        // Not double in should not count
        let mut hit = DartHit::new(20, DartMultipliers::Triple);
        let result = round.handle_throw(&mut hit);

        assert_eq!(result, true, "Throw should be valid");
        assert_eq!(round.score, 301, "Score should not be counted");

    }
}

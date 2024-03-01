use super::multipliers::DartMultipliers;
use rand::prelude::*;

pub struct DartHit {
    pub base_score: i32,
    pub multiplier: DartMultipliers,
    pub calculated_score: i32
}

impl DartHit {
    pub fn new(base_score: i32, multiplier: DartMultipliers) -> Self {
        DartHit {
            base_score,
            multiplier,
            calculated_score: 0
        }
    }

    pub fn rnd() -> Self {
        let mut rng = thread_rng();

        let base_score = rng.gen_range(1..20);

        let multiplier = match rng.gen_range(0..3) {
            0 => DartMultipliers::None,
            1 => DartMultipliers::Double,
            2 => DartMultipliers::Triple,
            _ => DartMultipliers::None
        };

        DartHit {
            base_score,
            multiplier,
            calculated_score: 0
        }
    }

    pub fn get_score (&mut self) -> i32{
        self.calculated_score = match self.multiplier {
            DartMultipliers::None => self.base_score,
            DartMultipliers::Double => self.base_score * 2,
            DartMultipliers::Triple => self.base_score * 3
        };

        return self.calculated_score;
    }
}

impl Clone for DartHit {
    fn clone(&self) -> Self {
        DartHit {
            base_score: self.base_score,
            multiplier: self.multiplier,
            calculated_score: self.calculated_score
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dart_hit() {
        let mut hit = DartHit::new(20, DartMultipliers::Double );

        assert_eq!(hit.get_score(), 40);
    }
}

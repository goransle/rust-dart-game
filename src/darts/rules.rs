use std::str::FromStr;

pub struct DartRules {
    pub double_out: bool,
    pub double_in: bool,
    pub start_score: i32
}

impl DartRules {
    pub fn new(double_out: bool, double_in: bool, start_score: i32) -> Self {
        DartRules {
            double_out,
            double_in,
            start_score
        }
    }
}

impl Clone for DartRules {
    fn clone(&self) -> Self {
        DartRules {
            double_out: self.double_out,
            double_in: self.double_in,
            start_score: self.start_score
        }
    }
}

impl FromStr for DartRules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.split_once(" ") {
            Some((start_score, ruleset)) => {
                Ok(Self {
                    start_score: i32::from_str(start_score)?,
                    double_in: ruleset == "double_in" ||  ruleset == "master",
                    double_out: ruleset == "double_out" || ruleset == "master"
                })
            },
            None => {
                return Err(anyhow::Error::msg("Invalid input"));
            }

        };

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let rules = "301 double_out".parse::<DartRules>().unwrap();
        assert_eq!(rules.start_score, 301);
        assert_eq!(rules.double_out, true);
        assert_eq!(rules.double_in, false);

        let rules = "501 master".parse::<DartRules>().unwrap();
        assert_eq!(rules.start_score, 501);
        assert_eq!(rules.double_out, true);
        assert_eq!(rules.double_in, true);
    }
}

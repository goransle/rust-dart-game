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

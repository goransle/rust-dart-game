pub enum DartMultipliers {
    None,
    Double,
    Triple
}

impl Copy for DartMultipliers {
    
}

impl Clone for DartMultipliers {
    fn clone(&self) -> Self {
        match self {
            DartMultipliers::None => DartMultipliers::None,
            DartMultipliers::Double => DartMultipliers::Double,
            DartMultipliers::Triple => DartMultipliers::Triple
        }
    }
}

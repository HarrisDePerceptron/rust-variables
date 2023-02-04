use crate::car;




pub enum CarMake {
    TOYOTA(String),
    SUZUKI(String),
    HONDA(String),
    
}   


struct ToyotaCar {
  t: car::make::CarMake,
  a: CarMake
}


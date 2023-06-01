use typechain::{Chain, chainlink};


#[chainlink]
pub trait Living {
    fn name(&self) -> String;
}

#[derive(Chain)]
pub struct Farmer {
    #[chain(Living)]
    name: String,
    pub age: u8
}

impl Farmer {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age
        }
    }
}

#[chainlink]
pub trait Animal: Living {
    fn legs(&self) -> u8;
}

#[derive(Chain)]
pub struct FarmAnimal {
    #[chain(Living)]
    name: String,
    #[chain(Animal)]
    legs: u8
}

impl FarmAnimal {
    pub fn new(name: &str, legs: u8) -> Self {
        Self {
            name: name.to_string(),
            legs
        }
    }
}

#[derive(Chain)]
pub struct WildAnimal {
    #[chain(Living)]
    name: String,
    #[chain(Animal)]
    legs: u8
}

impl WildAnimal {
    pub fn new(name: &str, legs: u8) -> Self {
        Self {
            name: name.to_string(),
            legs
        }
    }
}
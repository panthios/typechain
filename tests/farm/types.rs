use typechain::{chain, chainlink};


chainlink!(Living => {
    const name: String;
});

chain!(Farmer => {
    @Living
    const name: String;

    pub const age: u8;
});

impl Farmer {
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age
        }
    }
}

chainlink!(Animal => {
    const legs: u8;
});

chain!(FarmAnimal => {
    @Living
    const name: String;

    @Animal
    const legs: u8;
});

impl FarmAnimal {
    pub fn new(name: &str, legs: u8) -> Self {
        Self {
            name: name.to_string(),
            legs
        }
    }
}

chain!(WildAnimal => {
    @Living
    const name: String;

    @Animal
    const legs: u8;
});

impl WildAnimal {
    pub fn new(name: &str, legs: u8) -> Self {
        Self {
            name: name.to_string(),
            legs
        }
    }
}
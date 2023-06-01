use typechain::use_chains;

mod types;
use types::{Farmer, FarmAnimal, WildAnimal};

use_chains![
    types::Living,
    types::Animal
];

#[test]
fn test_farm() {
    let farmer = Farmer::new("John", 42);

    let animals: Vec<Box<Animal>> = vec![
        Box::new(FarmAnimal::new("Cow", 4)),
        Box::new(WildAnimal::new("Wolf", 4))
    ];

    for animal in animals {
        assert_eq!(animal.legs(), 4);
    }

    assert_eq!(farmer.name(), "John");
    assert_eq!(farmer.age, 42);

    let _ = vec![
        Box::new(farmer) as Box<Living>,
        Box::new(FarmAnimal::new("Cow", 4)) as Box<Living>,
        Box::new(WildAnimal::new("Wolf", 4)) as Box<Living>
    ];
}
use std::rc::Rc;

use typechain::{chain, chainlink};


chainlink!(Person => {
    const name: String;
    const age: u8;
});

chainlink!(Adult => {
    const job: String;
});

chain!(Parent => {
    @Person
    const name: String;

    @Person
    const age: u8;

    @Adult
    const job: String;

    const children: Vec<Rc<Person>>;
});

chain!(Employer => {
    @Person
    const name: String;

    @Person
    const age: u8;

    @Adult
    const job: String;
});

chain!(Child => {
    @Person
    const name: String;

    @Person
    const age: u8;

    const school: String;
});

#[test]
fn test_person() {
    let child = Rc::new(Child {
        name: "John".to_string(),
        age: 10,
        school: "Elementary".to_string(),
    });

    let child2 = Rc::new(Child {
        name: "Jane".to_string(),
        age: 8,
        school: "Elementary".to_string(),
    });

    let employer = Employer {
        name: "Bob".to_string(),
        age: 45,
        job: "Software Engineer".to_string(),
    };

    let parent1 = Rc::new(Parent {
        name: "Dave".to_string(),
        age: 40,
        job: "Software Engineer".to_string(),
        children: vec![child.clone(), child2.clone()]
    });

    let parent2 = Rc::new(Parent {
        name: "Alice".to_string(),
        age: 40,
        job: "Software Engineer".to_string(),
        children: vec![child.clone(), child2.clone()]
    });

    let gparent = Parent {
        name: "George".to_string(),
        age: 70,
        job: "Retired".to_string(),
        children: vec![parent1.clone(), parent2.clone()]
    };

    let people: Vec<Rc<Person>> = vec![Rc::new(gparent), Rc::new(employer), child, child2, parent1, parent2];

    assert_eq!(people.iter().filter(|p| p.name() == "John").count(), 1);
}
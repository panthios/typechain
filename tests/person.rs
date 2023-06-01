use std::rc::Rc;

use typechain::{Chain, chainlink};


#[chainlink]
trait Person {
    fn name(&self) -> String;
    fn age(&self) -> u8;
}

#[chainlink]
trait Adult: Person {
    fn job(&self) -> String;
}

#[derive(Chain)]
struct Parent {
    #[chain(Person)]
    name: String,
    #[chain(Person)]
    age: u8,
    #[chain(Adult)]
    job: String,
    children: Vec<Rc<Person>>
}

#[derive(Chain)]
struct Employer {
    #[chain(Person)]
    name: String,
    #[chain(Person)]
    age: u8,
    #[chain(Adult)]
    job: String
}

#[derive(Chain)]
struct Child {
    #[chain(Person)]
    name: String,
    #[chain(Person)]
    age: u8,
    school: String
}

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
use typechain::{chainlink, impl_chains};


#[chainlink]
pub trait Container {
    fn get(&self) -> u32;
    fn set(&mut self, value: u32);
}

impl_chains!(Box<u32> => {
    fn get(&self) -> u32 {
        **self
    } in Container;

    fn set(&mut self, value: u32) {
        *self = Box::new(value);
    } in Container;
});

impl_chains!(Vec<u32> => {
    fn get(&self) -> u32 {
        self[0]
    } in Container;

    fn set(&mut self, value: u32) {
        if self.is_empty() {
            self.push(value);
            return;
        }

        self[0] = value;
    } in Container;
});

#[test]
fn test_box() {
    let mut my_box = Box::new(0u32);
    let mut my_vec = vec![0u32];

    let containers: Vec<&mut Container> = vec![&mut my_box, &mut my_vec];

    for container in containers {
        container.set(1);
        assert_eq!(container.get(), 1);
    }

    assert_eq!(my_box, Box::new(1));
    assert_eq!(my_vec, vec![1]);
}
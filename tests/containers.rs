use typechain::{chainlink, impl_chains};


chainlink!(Container<T> => {
    fn get(&self) -> T;
    fn set(&mut self, value: T);
});

impl_chains!(Box<T> => {
    fn get(&self) -> T {
        **self
    } in Container<T>;

    fn set(&mut self, value: T) {
        *self = Box::new(value);
    } in Container<T>;
} where <T: Clone + Copy>);

impl_chains!(Vec<T> => {
    fn get(&self) -> T {
        self[0]
    } in Container<T>;

    fn set(&mut self, value: T) {
        if self.is_empty() {
            self.push(value);
            return;
        }

        self[0] = value;
    } in Container<T>;
} where <T: Clone + Copy>);

#[test]
fn test_box() {
    let mut my_box = Box::new(0u32);
    let mut my_vec = vec![0u32];

    let containers: Vec<&mut Container<_>> = vec![&mut my_box, &mut my_vec];

    for container in containers {
        container.set(1);
        assert_eq!(container.get(), 1);
    }

    assert_eq!(my_box, Box::new(1));
    assert_eq!(my_vec, vec![1]);
}
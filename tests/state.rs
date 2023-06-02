use typechain::{chainlink, chain};


chainlink!(State<T> => {
    mut value: T;
});

chain!(BasicState<T> => {
    @State<T>
    mut value: T;
});

impl<T> BasicState<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}

#[test]
fn test_state() {
    let mut state = BasicState::new(0);

    assert_eq!(state.value(), &0);

    *state.value() = 1;

    assert_eq!(state.value(), &1);
}
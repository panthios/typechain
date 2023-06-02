use typechain::{chainlink, chain};


chainlink!(RefBox<'a, T> => {
    const value: &'a T
});

chain!(Basic<'a, T> => {
    @RefBox<'a, T>
    const value: &'a T
});

#[test]
fn test_lifetimes() {
    let val = 5;

    let basic = Basic {
        value: &val
    };

    assert_eq!(*basic.value(), &val);
}
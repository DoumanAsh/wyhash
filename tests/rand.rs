#[test]
fn should_rand() {
    let mut rand = wy::Random::new(0);
    assert_ne!(rand.gen(), 0);

}

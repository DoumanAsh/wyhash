#[test]
fn should_rand() {
    let mut rand = wy::Random::new(0);
    assert_ne!(rand.gen(), 0);

    let rand = wy::AtomicRandom::new(2);
    assert_ne!(rand.gen(), 0);
}

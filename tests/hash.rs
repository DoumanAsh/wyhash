#[test]
fn should_hash32() {
    let mut rand = wy::Random::new(0);
    let seed = rand.gen() as u32;

    assert_ne!(wy::hash32(b"", seed), 0);
    assert_ne!(wy::hash32(b"1", seed), 0);
    assert_ne!(wy::hash32(b"1234", seed), 0);
    assert_ne!(wy::hash32(b"1234", seed), wy::hash32(b"4321", seed));
    assert_ne!(wy::hash32(b"12", seed), wy::hash32(b"21", seed));
    assert_eq!(wy::hash32(b"1", seed), wy::hash32(b"1", seed));
    assert_ne!(wy::hash32(b"12345678", seed), wy::hash32(b"87654321", seed));
    assert_ne!(wy::hash32(b"123456789", seed), wy::hash32(b"987654321", seed));
}

#[test]
fn should_hash() {
    let mut rand = wy::Random::new(0);
    let seed = rand.gen();

    assert_ne!(wy::def_hash(b"", seed), 0);
    assert_ne!(wy::def_hash(b"1", seed), 0);
    assert_ne!(wy::def_hash(b"1234", seed), 0);
    assert_ne!(wy::def_hash(b"1234", seed), wy::def_hash(b"4321", seed));
    assert_ne!(wy::def_hash(b"12", seed), wy::def_hash(b"21", seed));
    assert_eq!(wy::def_hash(b"1", seed), wy::def_hash(b"1", seed));
    assert_eq!(wy::def_hash(b"12345", seed), wy::def_hash(b"12345", seed));
    assert_eq!(wy::def_hash(b"123456", seed), wy::def_hash(b"123456", seed));
    assert_ne!(wy::def_hash(b"1234567", seed), wy::def_hash(b"7654321", seed));
    assert_ne!(wy::def_hash(b"12345678", seed), wy::def_hash(b"87654321", seed));
    assert_ne!(wy::def_hash(b"123456789", seed), wy::def_hash(b"987654321", seed));
    assert_ne!(wy::def_hash(b"123456789ABCD", seed), wy::def_hash(b"DCBA987654321", seed));
    assert_eq!(wy::def_hash(b"123456789ABCD", seed), wy::def_hash(b"123456789ABCD", seed));
    assert_eq!(wy::def_hash(b"123456789ABC", seed), wy::def_hash(b"123456789ABC", seed));
    assert_eq!(wy::def_hash(b"123456789AB", seed), wy::def_hash(b"123456789AB", seed));
    assert_eq!(wy::def_hash(b"123456789A", seed), wy::def_hash(b"123456789A", seed));
}

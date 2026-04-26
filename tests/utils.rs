#[warn(dead_code)]

use rand::RngExt;

pub const ONE_VARA: u128 = 1_000_000_000_000;

pub fn rand_salt() -> Vec<u8> {
    let mut rng = rand::rng();
    (0..8).map(|_| rng.random::<u8>()).collect()
}
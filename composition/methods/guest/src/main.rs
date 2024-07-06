use helloworld_methods::MULTIPLY_ID;
use risc0_zkvm::{guest::env, serde};
fn main() {
    // read the input
    let (n, e, x): (u64, u64, u64) = env::read();

    env::verify(MULTIPLY_ID, &serde::to_vec(&n).unwrap()).unwrap();

    // write public output to the journal
    env::commit(&(n, e, pow_mod(x, e, n)));
}

fn pow_mod(x: u64, mut e:u64, n: u64) -> u64 {
    let mut x = x;
    let mut res = 1;
    while e > 0 {
        if e % 2 == 1 {
            res = res * x % n;
        }
        e >>= 1;
        x = (x * x) % n;
    }
    res as u64
}
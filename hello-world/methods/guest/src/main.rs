#![no_main]
#![no_std]


use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
    
    // read public input from the env
    let a:u64 = env::read();
    let b:u64 = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    if a == 1 || b == 1 {
        panic!("Trivial factors")
    }
    
    let produce = a.checked_mul(b).expect("Integer Overflow");

    // write public output to the journal
    env::commit(&produce);
}

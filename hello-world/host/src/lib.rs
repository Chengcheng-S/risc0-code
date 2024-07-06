use helloworld_methods::MULTIPLY_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// Compute the product a*b inside the zkVM
pub fn multiply(a: u64, b: u64) -> (Receipt, u64) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover.prove(env, MULTIPLY_ELF).unwrap().receipt;

    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    println!("I know the factors of {}, and I can prove it!", c);

    (receipt, c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        const TEST_FACTOR_ONE: u64 = 17;
        const TEST_FACTOR_TWO: u64 = 23;
        let (_, result) = multiply(17, 23);
        assert_eq!(
            result,
            TEST_FACTOR_ONE * TEST_FACTOR_TWO,
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}

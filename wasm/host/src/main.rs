use methods::{WASM_ELF, WASM_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
    wat::parse_str(wat)
}

fn run_guest(iters: i32) -> i32 {
    // reference https://github.com/WebAssembly/wabt/issues/1802
    // `set_local` can not compile to wasm, So change to local.set
    // Similarly `get_local` can not compile to wasm, So change to local.get and local.tee
    let wat = r#"
    (module
        (export "fib" (func $fib))
        (func $fib (; 0 ;) (param $0 i32) (result i32)
        (local $1 i32)
        (local $2 i32)
        (local $3 i32)
        (local $4 i32)
        (local.set $4
         (i32.const 1)
        )
        (block $label$0
         (br_if $label$0
          (i32.lt_s
           (local.get $0)
           (i32.const 1)
          )
         )
         (local.set $3
          (i32.const 0)
         )
         (loop $label$1
          (local.set $1
           (i32.add
            (local.get $3)
            (local.get $4)
           )
          )
          (local.set $2
           (local.get $4)
          )
          (local.set $3
           (local.get $4)
          )
          (local.set $4
           (local.get $1)
          )
          (br_if $label$1
           (local.tee $0
            (i32.add
             (local.get $0)
             (i32.const -1)
            )
           )
          )
         )
         (return
          (local.get $2)
         )
        )
        (i32.const 0)
    )
    )    
    "#;

    let wasm = wat2wasm(wat).expect("Failed to parse_str");

    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .write(&iters)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    let receipt = prover.prove(env, WASM_ELF).unwrap().receipt;

    receipt.verify(WASM_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
    let result: i32 = receipt.journal.decode().unwrap();

    result
}

fn main() {
    let fib_iters: i32 = 100;
    let _ = run_guest(fib_iters);
}

#[cfg(test)]
mod tests {
    fn fibonacci(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            let c = a;
            a = b;
            b += c;
        }
        a
    }

    #[test]
    fn wasm_fib() {
        let fib_iters: i32 = 10;
        let result = super::run_guest(fib_iters);
        assert_eq!(
            result,
            fibonacci(fib_iters),
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}

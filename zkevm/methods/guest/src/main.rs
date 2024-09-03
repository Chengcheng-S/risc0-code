use risc0_zkvm::guest::env;
use zkevm_core::{Env, EvmResult, ExecutionResult, ZkDb, EVM};

fn main() {
    let env: Env = env::read();
    let db: ZkDb = env::read();

    let mut evm = EVM::new();
    evm.database(db);
    evm.env = env;
    let res = evm.transact().unwrap();
    if let ExecutionResult::Success { reason, .. } = res.result {
        env::commit(&EvmResult {
            exit_reason: reason,
            state: res.state,
        });
    } else {
        panic!("Unexpected result");
    }
    env::log("");
}

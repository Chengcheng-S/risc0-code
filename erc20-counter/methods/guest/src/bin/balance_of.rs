
#![allow(unused_doc_comments)]
#![no_main]

use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolValue};
use risc0_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmInput, Contract, SolCommitment};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

/// Specify the function to call using the [`sol!`] macro.
/// This parses the Solidity syntax to generate a struct that implements the `SolCall` trait.
sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
}

/// ABI encodable journal data.
sol! {
    struct Journal {
        SolCommitment commitment;
        address tokenAddress;
    }
}

fn main(){
    // inputs from guest env
    let input:EthEvmInput = env::read();
    let contract:Address = env::read();
    let account:Address = env::read();

    // Converts the input into a `EvmEnv` for execution.
    let env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);


    let call = IERC20::balanceOfCall{account};
    let returns = Contract::new(contract, &env).call_builder(&call).call();

    assert!(returns._0 >= U256::from(1));

    // Commit the block hash and number used when deriving `view_call_env` to the journal.
    let journal = Journal {
        commitment: env.block_commitment(),
        tokenAddress: contract,
    };
    env::commit_slice(&journal.abi_encode());
}
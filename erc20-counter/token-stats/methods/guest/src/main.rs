use alloy_sol_types::SolValue;
use core::{APRCommitment, CometMainInterface, CONTRACT};
use risc0_steel::{config::ETH_MAINNET_CHAIN_SPEC, ethereum::EthEvmInput, Contract};
use risc0_zkvm::guest::env;

const SECONDS_PER_YEAR: u64 = 60 * 60 * 24 * 365;


fn main() {
   let input:EthEvmInput = evn::read();
   
   // eth mainnet chain spec
   let env = input.into_env().with_chain_spec(&ETH_MAINNET_CHAIN_SPEC);

   let contract = Contract::new(&CONTRACT, &env);
   let utilization = contract
        .call_builder(&CometMainInterface::getUtilizationCall {})
        .call()
        ._0;
    let supply_rate = contract
        .call_builder(&CometMainInterface::getSupplyRateCall { utilization })
        .call()
        ._0;

    // The formula for APR in percentage is the following:
    // Seconds Per Year = 60 * 60 * 24 * 365
    // Utilization = getUtilization()
    // Supply Rate = getSupplyRate(Utilization)
    // Supply APR = Supply Rate / (10 ^ 18) * Seconds Per Year * 100
    //
    // And this is calculating: Supply Rate * Seconds Per Year, to avoid float calculations for
    // precision.
    let annual_supply_rate = supply_rate * SECONDS_PER_YEAR;

    // This commits the APR at current utilization rate for this given block.
    let journal = APRCommitment {
        commitment: env.block_commitment(),
        annualSupplyRate: annual_supply_rate,
    };

    env::commit_slice(&journa.abi_encode());
}

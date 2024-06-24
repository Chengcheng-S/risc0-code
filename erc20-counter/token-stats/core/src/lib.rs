use alloy_primitives::{address, Address};
use alloy_sol_types::sol;
use risc0_steel::SolCommitment;

/// Address of Compound USDC (cUSDCv3) token.
pub const CONTRACT: Address = address!("c3d688B66703497DAA19211EEdff47f25384cdc3");

sol!{
    interface CometMainInterface{
        function getSupplyRate(uint utilization) virtual public view returns (uint64);
        function getUtilization() public view returns (uint);
    }
}
sol! {
    struct APRCommitment {
        SolCommitment commitment;
        uint64 annualSupplyRate;
    }
}

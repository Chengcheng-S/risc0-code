use anyhow::Result;
use ethers::prelude::*;
pub struct TxSender{
    chain_id :u64,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    contract:Address,
}

impl TxSender {
    pub fn new(
        chain_id: u64, 
        rpc_url: &str, 
        private_key: &str, 
        contract: &str
    )->Result<Self>{
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let contract = contract.parse::<Address>()?;
        Ok(Self{
            chain_id,
            client,
            contract
        })

    }

    pub async fn send_tx(
        &self,
        calldata:Vec<u8>
    )->Result<Option<TransactionReceipt>>{
        let tx = TransactionRequest::new()
            .from(self.client.address())
            .to(self.contract)
            .data(calldata)
            .chain_id(self.chain_id);
        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;
        
        log::info!("Transaction sent: {:?}", &tx);
        Ok(tx)
    }
}
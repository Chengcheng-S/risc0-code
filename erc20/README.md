## erc20-counter
参照 risc0-ethereum/erc20-counter

##功能概述
使用steel 模块与自定义的 evm合约进行交互。 使用steel链下证明来：
- 根据用户提交的证明增加计数器。
- 在增加之前验证 ERC20 代币所有权（至少需要 1 个代币）。
- 利用 RISC Zero 作为 [协处理器] 来高效生成和验证证明。

## APR proof  token-state
- 展示了如何使用 [Steel] 库来调用合约的多个veiw function。
- 生成了 [Compound] cToken 的 APR（年利率）证明，展示了链上验证复杂财务指标的能力。
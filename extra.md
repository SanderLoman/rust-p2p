FlashBotsUniswapQuery address: 0x5EF1009b9FCD4fec3094a5564047e190D72Bd511 (for simple arbitrage, maybe not needed)
UniswapRouterV2 address: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
UniswapFactory address: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f
PancakeRouterV2 address: 0x10ED43C718714eb63d5aA57B78B54704E256024E
SushiSwap address: 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506

Ethereum address: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2

!!! RANDOM CODE !!!
```rust
    let sub = provider_eth.watch_pending_transactions().await?;

    sub.for_each(|tx| async move {
        println!("New pending transaction: https://etherscan.io/tx/{:?}", tx);
    })
    .await;

    let ethereum_ca: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let read_ethereum_abi: String = fs::read_to_string("abis/ethereum.json")?;
    let abi = Contract::load(read_ethereum_abi.as_bytes())?;

    let ethereum_contract = ethers::contract::Contract::new(ethereum_ca, abi, provider_eth);
```
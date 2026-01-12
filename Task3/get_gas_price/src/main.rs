use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{U256, utils::format_units};
use std::error::Error;

// Arbitrum 基础转账的 Gas Limit（经验值）
const BASE_TRANSFER_GAS_LIMIT: u64 = 21000;

/// 获取当前 Arbitrum Gas 价格（以 Wei 为单位，返回 u128）
async fn get_gas_price(rpc_url: &str) -> Result<u128, Box<dyn Error>> {
    let url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(url);
    let gas_price = provider.get_gas_price().await?;
    Ok(gas_price)
}

/// 计算预估转账 Gas 费用（以 ETH 为单位）
async fn estimate_transfer_gas_fee(rpc_url: &str) -> Result<String, Box<dyn Error>> {
    // 1. 获取 Gas 价格 (u128)
    let gas_price_val = get_gas_price(rpc_url).await?;

    // 2. 关键修改：把 u128 转成 U256
    let gas_price_u256 = U256::from(gas_price_val);
    let gas_limit_u256 = U256::from(BASE_TRANSFER_GAS_LIMIT);

    // 3. 现在两个都是 U256，可以相乘了
    let gas_fee_wei = gas_price_u256 * gas_limit_u256;

    // 4. 将 Gas 费用从 Wei 转换为 ETH
    let gas_fee_eth = format_units(gas_fee_wei, 18)?;

    Ok(gas_fee_eth)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Arbitrum Sepolia 的公共 RPC 节点
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";

    match estimate_transfer_gas_fee(rpc_url).await {
        Ok(gas_fee) => {
            println!("---------------------------------");
            println!("当前预估 Gas 费: {} ETH", gas_fee);
            println!("---------------------------------");
        }
        Err(e) => {
            eprintln!("获取预估 Gas 费失败: {}", e);
        }
    }

    Ok(())
}
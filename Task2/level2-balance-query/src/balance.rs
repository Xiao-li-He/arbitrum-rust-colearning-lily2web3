use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{Address, utils::format_units};
use std::error::Error;
use std::str::FromStr;

/// 查询地址余额并返回可读的 ETH 字符串
/// 
/// # 参数
/// * `rpc_url` - RPC 节点地址
/// * `address_str` - 要查询的钱包或合约地址
async fn get_readable_balance(rpc_url: &str, address_str: &str) -> Result<String, Box<dyn Error>> {
    // 1. 创建 Provider (连接到 RPC)
    let url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(url);

    // 2. 解析地址字符串为 Address 类型
    let address = Address::from_str(address_str)?;

    // 3. 查询余额 (返回的是 U256 类型的 Wei)
    // get_balance 默认查询最新区块
    let balance_wei = provider.get_balance(address).await?;

    // 4. 格式化单位 (Wei -> ETH)
    // ETH 的精度是 18 位
    let balance_eth = format_units(balance_wei, 18)?;

    Ok(balance_eth)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Arbitrum Sepolia 的公共 RPC 节点
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    
    // 替换为你自己的地址
    let target_address = "0xF6FEBd05224397E58CFd604220b31f84c089A2e1"; 

    println!("正在查询地址: {} ...", target_address);

    match get_readable_balance(rpc_url, target_address).await {
        Ok(balance) => {
            println!("---------------------------------");
            println!("余额: {} ETH", balance);
            println!("---------------------------------");
        },
        Err(e) => {
            eprintln!("查询失败: {}", e);
        }
    }

    Ok(())
}

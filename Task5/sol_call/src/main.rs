use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::Address;
use alloy::sol;
use std::error::Error;
use std::str::FromStr;

// 1. 使用 sol! 宏定义合约接口 (ABI)
// 我们不需要知道合约的全部代码，只需要把我们要调用的函数写出来即可
// 这里定义了一个标准的 ERC20 接口
sol! {
    // #[sol(rpc)] 会自动为结构体生成连接 RPC 的方法
    #[sol(rpc)]
    contract MyERC20 {
        // 查询代币名称
        function name() external view returns (string memory);
        // 查询代币符号
        function symbol() external view returns (string memory);
        // 查询精度
        function decimals() external view returns (uint8);
        // 查询总供应量 (可选)
        function totalSupply() external view returns (uint256);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 2. 连接 RPC 节点
    // 因为只是读取数据，不需要私钥，也不需要 .env 文件，直接连公用节点即可
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com";
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    // 3. 设置目标合约地址 (这里使用的是 Arbitrum Sepolia 的 WETH 地址)
    let contract_addr = Address::from_str("0x980B62Da83eFf3D4576C647993b0c1D7faf17c73")?;

    println!("正在连接 Arbitrum Sepolia 网络...");
    println!("目标合约: {}", contract_addr);

    // 4. 创建合约实例
    // MyERC20 是上面 sol! 宏自动生成的结构体
    let contract = MyERC20::new(contract_addr, provider);

    println!("--------------------------------------");
    println!("正在读取合约状态...");

    // 5. 调用只读方法
    // 注意：读取操作使用 .call().await?
    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;
    let total_supply = contract.totalSupply().call().await?; // 返回的是 U256

    // 6. 打印结果
    println!("代币名称 (Name): {}", name);
    println!("代币符号 (Symbol): {}", symbol);
    println!("代币精度 (Decimals): {}", decimals);
    println!("总供应量 (Total Supply): {} (Wei)", total_supply);
    println!("--------------------------------------");

    Ok(())
}
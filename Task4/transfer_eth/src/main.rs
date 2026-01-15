use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::primitives::{Address, utils::parse_units};
use alloy::rpc::types::TransactionRequest;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 加载 .env 文件中的环境变量
    dotenv().ok();

    // 2. 从环境变量读取 RPC URL 和 私钥    
    let rpc_url = env::var("RPC_URL").expect("请在 .env 中设置 RPC_URL");
    let private_key = env::var("PRIVATE_KEY").expect("请在 .env 中设置 PRIVATE_KEY");

    // 3. 设置转账接收方地址 (B 地址)
    // 这里可以替换为你想要转账的目标地址
    let to_address = Address::from_str("0x3019826431baaacc91604A595791a2d84acf5a56")?; 

    // 4. 设置转账金额 (例如 0.0001 ETH)
    let transfer_amount_eth = "0.001";
    let amount_wei = parse_units(transfer_amount_eth, "ether")?.into();

    println!("------------------------------------------------");
    println!("准备开始转账...");
    println!("目标网络: Arbitrum Sepolia");
    println!("接收地址: {}", to_address);
    println!("转账金额: {} ETH", transfer_amount_eth);
    println!("------------------------------------------------");

    // 5. 初始化钱包 (Signer)
    // 解析私钥，创建一个签名者
    let signer: PrivateKeySigner = private_key.parse().expect("私钥格式错误");
    let wallet = EthereumWallet::from(signer);

    // 6. 创建 Provider
    // .with_recommended_fillers() 是关键：
    // 它会自动帮我们完成 Task 3 的工作：查询当前 Gas Price，预估 Gas Limit，并填充 Nonce
    let provider = ProviderBuilder::new()
        // .with_recommended_fillers()
        .wallet(wallet) // 绑定钱包，赋予“写”权限
        .connect_http(rpc_url.parse()?);

    // 7. 构建交易请求
    // 注意：我们不需要手动计算 Gas = Price * Limit，Provider 会自动处理
    let tx = TransactionRequest::default()
        .with_to(to_address)
        .with_value(amount_wei);

    println!("正在发送交易，请稍候...");

    // 8. 发送交易并等待回执
    // send_transaction 发送 -> watch 等待链上确认
    let receipt = provider.send_transaction(tx)
        .await?
        .get_receipt() 
        .await?;

    // 9. 获取交易哈希并验证结果
    if receipt.status() {
        println!("✅ 转账成功!");
        println!("交易哈希 (Tx Hash): {}", receipt.transaction_hash);
        println!("查看浏览器: https://sepolia.arbiscan.io/tx/{}", receipt.transaction_hash);
    } else {
        eprintln!("❌ 交易失败 (Reverted)");
    }

    Ok(())
}
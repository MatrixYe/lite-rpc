use std::env;

use anyhow::Context;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

pub async fn load_identity_keypair(
    identity_keyfile_path: Option<String>,
) -> anyhow::Result<Option<Keypair>> {
    let identity_json_array_str = if let Ok(identity_env_var) = env::var("IDENTITY") {
        identity_env_var
    } else if let Some(identity_path) = identity_keyfile_path {
        tokio::fs::read_to_string(identity_path)
            .await
            .context("加载账户失败: Cannot find the identity file provided")?
    } else {
        return Ok(None);
    };

    let identity_bytes: Vec<u8> = serde_json::from_str(&identity_json_array_str)
        .context("加载账户失败:Invalid identity format expected Vec<u8>")?;
    let keypair = Keypair::from_bytes(identity_bytes.as_slice()).context("Invalid identity")?;
    let pubkey = keypair.pubkey().to_string();
    println!("加载账户地址成功: {:?}", pubkey);
    Ok(Some(keypair))
}


pub async fn monero_create_subaddress(rpc_url: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": "0",
        "method": "create_address",
        "params": {
            "account_index": 0,
            "count": 1,
        }
    });

    let resp = client
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Monero RPC request failed: {e}"))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Monero RPC parse failed: {e}"))?;

    let address = resp["result"]["addresses"][0]["address"]
        .as_str()
        .ok_or_else(|| format!("Unexpected RPC response: {resp}"))?
        .to_string();

    Ok(address)
}
use anyhow::Result; // type of import
use iroh::{Endpoint, SecretKey};

#[tokio::main] // attribute way of modifying code
async fn main() -> Result<()> {
    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    println!("> our secret key: {:?}", secret_key);

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .discovery_n0()
        .bind()
        .await?;

    println!("> our node id: {}", endpoint.node_id());
    Ok(()) // return Ok and ignore semicolon to make it the return value of the function
}
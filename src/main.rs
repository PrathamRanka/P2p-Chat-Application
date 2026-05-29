use anyhow::Result;
use iroh::protocol::Router;
use iroh::{Endpoint, endpoint::presets};
use iroh_gossip::net::Gossip;


#[tokio::main] 
async fn main() -> Result<()> {
    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    println!("> our secret key: {:?}", secret_key);

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .discovery_n0()
        .bind()
        .await?;

    println!("> our node id: {}", endpoint.node_id());

    let gossip = Gossip::builder().spawn(endpoint.clone());


    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    router.shutdown().await?;
    Ok(()) }
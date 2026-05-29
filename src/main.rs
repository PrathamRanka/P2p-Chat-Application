use anyhow::Result;
use iroh::protocol::Router;
use iroh::{Endpoint, endpoint::presets, SecretKey};
use iroh_gossip::net::Gossip;


#[tokio::main] 
async fn main() -> Result<()> {
    let secret_key = SecretKey::generate();
    println!("> our secret key: {:?}", secret_key);

    let endpoint = Endpoint::builder(presets::Empty)
        .secret_key(secret_key)
        .bind()
        .await?;

    println!("> our node id: {}", endpoint.id().fmt_short());

    let gossip = Gossip::builder().spawn(endpoint.clone());


    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    router.shutdown().await?;
    Ok(()) }
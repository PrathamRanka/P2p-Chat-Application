use anyhow::Result;
use iroh::protocol::Router;
use iroh::{Endpoint, endpoint::presets, SecretKey};
use iroh_gossip::net::Gossip;
use std::env;


async fn run_node() -> Result<()> {
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
    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        None => run_node().await,
        Some("smoke-test") => {
            // Non-interactive smoke test: create two endpoints and ensure they initialize.
            println!("Running smoke-test: spinning up two endpoints...");

            // Start node A
            let a = tokio::spawn(async { run_node().await });

            // Start node B
            let b = tokio::spawn(async { run_node().await });

            let ra = a.await.expect("task a panicked")?;
            let rb = b.await.expect("task b panicked")?;
            drop(ra);
            drop(rb);
            println!("smoke-test: OK - two endpoints started and shut down successfully");
            Ok(())
        }
        Some(cmd) => {
            println!("Unknown command: {}", cmd);
            println!("Usage: <no-args> | smoke-test");
            Ok(())
        }
    }
}
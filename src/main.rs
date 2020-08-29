use tokio::runtime::Runtime;
use tokio::time::{delay_for, Duration};

use client::Client;
use error::Result;

mod client;
mod error;
mod request;
mod response;
mod session;
mod torrent;

async fn run() -> Result<()> {
    let uri = dotenv::var("TARGET")?;
    let mut client = Client::new(&uri);
    // let body = client.session_stats().await?;
    // println!("{:?}", body);
    // let body = client.port_test().await?;
    // println!("{:?}", body);
    // let body = client.blocklist_update().await?;
    // println!("{:?}", body);
    // let _ = client.session_close().await?;
    // let body = client.free_space("c:").await?;
    // println!("{:?}", body);
    // let body = client
    //     .torrent_get(torrent::TorrentGetArgs {
    //         fields: vec![
    //             torrent::TorrentFields::Priorities,
    //             torrent::TorrentFields::Pieces,
    //             torrent::TorrentFields::Wanted,
    //         ],
    //         ids: Some(request::IDS::ID(1)),
    //     })
    //     .await?;
    // println!("{:?}", body);

    delay_for(Duration::from_millis(1000)).await;
    Ok(())
}

fn main() {
    dotenv::dotenv().ok().unwrap();
    env_logger::init();

    let mut rt = Runtime::new().unwrap();

    rt.block_on(async { run().await.unwrap() });
}

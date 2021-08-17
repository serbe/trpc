use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

use client::Client;
use error::Error;
use torrent::TorrentAddArgs;

mod client;
mod error;
mod request;
mod response;
mod session;
mod torrent;

async fn run() -> Result<(), Error> {
    // let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");
    // let mut client = Client::new(&uri);
    // let body = client.session_stats().await?;
    // println!("{:?}", body);
    // let body = client.port_test().await?;
    // println!("{:?}", body);
    // let body = client.blocklist_update().await?;
    // println!("{:?}", body);
    // let _ = client.session_close().await?;
    // let body = client.free_space("d:\\Downloads\\").await?;
    // println!("{:?}", body);
    // let body = client
    //     .torrent_get(torrent::TorrentGetArgs {
    //         fields: vec![
    //             torrent::TorrentFields::Priorities,
    //             torrent::TorrentFields::Pieces,
    //             torrent::TorrentFields::Wanted,
    //         ],
    //         ids: Some(request::Ids::Id(1)),
    //     })
    //     .await?;
    // println!("{:?}", body);
    // let body = client
    //     .session_get(Some(session::SessionGetArgs {
    //         fields: vec![session::SessionFields::SessionID],
    //     }))
    //     .await?;
    // println!("{:?}", body);

    let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

    let mut client = Client::new(&uri);
    let args = TorrentAddArgs::from_file(
        "magnet:?xt=urn:btih:6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8&dn=test%20dir",
    )
    .unwrap();
    let body = client.torrent_add(args).await.unwrap();
    dbg!(body);

    sleep(Duration::from_millis(1000)).await;
    Ok(())
}

fn main() {
    dotenv::dotenv().ok().unwrap();
    env_logger::init();

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        if let Err(err) = run().await {
            eprintln!("{:?}", err);
        }
    });
}
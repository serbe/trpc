use std::convert::TryInto;

use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

use client::Client;
use error::Error;
use torrent::{TorrentAddArgs, TorrentGetArgs, TorrentRemoveArgs};

pub mod client;
pub mod error;
pub mod request;
pub mod response;
pub mod session;
pub mod torrent;

async fn run() -> Result<(), Error> {
    // let uri = dotenvy::var("TRPC_TARGET").expect("not set TRPC_TARGET");
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

    let uri = dotenvy::var("TRPC_TARGET").expect("not set TRPC_TARGET");
    let mut client = Client::new(&uri);

    let get_args = TorrentGetArgs {
        ids: Some(vec!["6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8".into()].into()),
        fields: vec!["id".try_into().unwrap()],
    };

    let body = client.torrent_get(get_args).await.unwrap();
    dbg!(body);

    let add_args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();

    let body = client.torrent_add(add_args).await.unwrap();
    dbg!(body);

    // let del_args = TorrentRemoveArgs {
    //     ids: Ids::Array(vec![Id::Hash(
    //         "6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8".to_string(),
    //     )]),
    //     delete_local_data: true,
    // };

    // let body = client.torrent_remove(del_args).await.unwrap();
    // dbg!(body);

    sleep(Duration::from_millis(1000)).await;
    Ok(())
}

fn main() {
    dotenvy::dotenv().ok().unwrap();
    env_logger::init();

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        if let Err(err) = run().await {
            eprintln!("{:?}", err);
        }
    });
}

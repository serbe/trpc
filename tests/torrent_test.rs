use transmission_rpc::{torrent::TorrentAddArgs, Client};

const MAGNET: &str = "magnet:?xt=urn:btih:6a0a9282c65fc6a1324e6e1605fe9bb9746c3aa8&dn=test%20dir";

#[tokio::test]
async fn test_torrent_add_meta() {
    let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

    let mut client = Client::new(&uri);
    let args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();
    let body = client.torrent_add(args).await.unwrap();
    dbg!(body);
}

#[tokio::test]
async fn test_torrent_add_uri() {
    let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

    let mut client = Client::new(&uri);
    let args = TorrentAddArgs::from_file(MAGNET).unwrap();
    let body = client.torrent_add(args).await.unwrap();
    dbg!(body);
}

#[tokio::test]
async fn test_torrent_add_with_file_and_meta() {
    let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

    let mut client = Client::new(&uri);
    let mut args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();
    args.metainfo = Some(MAGNET.to_string());
    let body = client.torrent_add(args).await;
    assert!(body.is_err());
}

#[tokio::test]
async fn test_torrent_add_without_file_and_meta() {
    let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");

    let mut client = Client::new(&uri);
    let mut args = TorrentAddArgs::from_meta("tests\\test dir.torrent").unwrap();
    args.metainfo = None;
    let body = client.torrent_add(args).await;
    assert!(body.is_err());
}

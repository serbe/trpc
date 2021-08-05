use std::env::temp_dir;

use transmission_rpc::Client;

#[tokio::test]
async fn test_torrent() {
    let _dir = temp_dir().to_str().expect("not get TEMP dir from env");

    let uri = dotenv::var("TRPC_TARGET").expect("not set TRPC_TARGET");
    let _client = Client::new(&uri);
}

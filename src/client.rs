use bytes::Bytes;
use nc::StatusCode;

use crate::error::Result;
use crate::request::RpcRequest;
use crate::response::RpcResponse;

pub struct Client {
    uri: String,
    id: String,
}

impl Client {
    pub fn new(uri: &str) -> Client {
        Client {
            uri: uri.to_string(),
            id: String::new(),
        }
    }

    fn set_id(&mut self, id: String) -> &mut Client {
        self.id = id;
        self
    }

    async fn get_response(&self, body: &Bytes) -> Result<nc::Response> {
        let cb = nc::Client::builder();
        let mut client = cb
            .post(&self.uri)
            .header("Content-Type", "application/json")
            .header("X-Transmission-Session-Id", &self.id)
            .body(body.clone())
            .build()
            .await?;
        Ok(client.send().await?)
    }

    pub async fn send_msg(&mut self, input: &RpcRequest) -> Result<RpcResponse> {
        let mut buf = vec![];
        serde_json::to_writer(&mut buf, &input)?;
        let body = buf.into();
        let response = self.get_response(&body).await?;
        let response_body = if Ok(response.status_code()) == StatusCode::from_u16(409u16) {
            if let Some(id) = response.headers().get("X-Transmission-Session-Id") {
                let response = self.set_id(id).get_response(&body).await?;
                response.body()
            } else {
                response.body()
            }
        } else {
            response.body()
        };
        Ok(serde_json::from_reader(response_body.as_ref())?)
    }
}

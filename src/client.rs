use bytes::Bytes;
use netc::StatusCode;

use crate::error::Error;
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

    async fn get_response(&self, body: &Bytes) -> Result<netc::Response, Error> {
        let cb = netc::Client::builder();
        let mut client = cb
            .post(&self.uri)
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Content-Type", "application/json")
            .header("X-Transmission-Session-Id", &self.id)
            .body(body.clone())
            .build()
            .await?;
        Ok(client.send().await?)
    }

    pub async fn send_msg(&mut self, input: &RpcRequest) -> Result<RpcResponse, Error> {
        let mut buf = vec![];
        serde_json::to_writer(&mut buf, &input)?;
        let body = buf.into();
        let response = self.get_response(&body).await?;
        if Ok(response.status_code()) == StatusCode::from_u16(401u16) {
            return Err(Error::NotAuth);
        }
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

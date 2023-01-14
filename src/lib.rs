use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{
    HttpRequest, HttpResponse, HttpServer, HttpServerReceiver,
};
mod logger;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct VerbalcodeActor {}

#[async_trait]
impl HttpServer for VerbalcodeActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> RpcResult<HttpResponse> {
        logger::log(format!("Request = {:?}", req)).await;
        handle_http_request(ctx, req).await
    }
}

use serde_urlencoded as urlencoded;

use key_value_store::KeyValueStore;

mod key_value_store;
mod responder;
mod twilio {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Payload {
        pub body: String,
        pub from: String,
    }
}

async fn handle_http_request(
    ctx: &Context,
    req: &HttpRequest,
) -> RpcResult<HttpResponse> {
    let payload = &req.body.as_slice();
    let payload: twilio::Payload = urlencoded::from_bytes(payload).unwrap();
    let mut store = new_store(ctx);
    let body = respond(payload.body, payload.from, &mut store).await;

    let mut resp = HttpResponse {
        body: body.as_bytes().to_vec(),
        ..Default::default()
    };

    resp.header
        .entry("content-type".to_string())
        .or_insert(vec!["text/plain".to_string()]);

    logger::log(format!("Response = {:?}, Body = {}", resp, body)).await;

    Ok(resp)
}

#[cfg(not(test))]
async fn respond<T: KeyValueStore>(
    prompt: String,
    prompter: String,
    store: &mut T,
) -> String {
    responder::handle(prompt, prompter, store).await
}

#[cfg(test)]
async fn respond<T: KeyValueStore>(
    prompt: String,
    prompter: String,
    _store: &mut T,
) -> String {
    format!("from: {}, body: {}", prompter, prompt)
}

#[cfg(target_arch = "wasm32")]
fn new_store(ctx: &Context) -> key_value_store::Actor {
    key_value_store::Actor::new(ctx)
}

#[cfg(not(target_arch = "wasm32"))]
fn new_store(_ctx: &Context) -> key_value_store::InMemory {
    key_value_store::InMemory::new()
}

#[cfg(test)]
mod test {
    use crate::handle_http_request;
    use serde_json as json;
    use std::fs;
    use wasmbus_rpc::actor::prelude::*;
    use wasmcloud_interface_httpserver::HttpRequest;

    #[tokio::test]
    async fn can_handle_http_request() {
        let req = fs::read_to_string("test/fixtures/request_1.json").unwrap();
        let req: HttpRequest = json::from_str(&req).unwrap();
        let ctx: Context = Default::default();
        let resp = handle_http_request(&ctx, &req).await.unwrap();

        assert_eq!(resp.status_code, 200);
        assert_eq!(
            String::from_utf8(resp.body).unwrap(),
            "from: +14108025604, body: test".to_string()
        );
    }
}

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{
    HttpRequest, HttpResponse, HttpServer, HttpServerReceiver,
};

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
        handle_http_request(ctx, req)
    }
}

#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;

mod code_exchange;
mod responder;
mod twilio {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Payload {
        pub body: String,
        pub from: String,
    }
}

fn handle_http_request(
    ctx: &Context,
    req: &HttpRequest,
) -> RpcResult<HttpResponse> {
    let payload: twilio::Payload = qs::from_bytes(req.body.as_slice()).unwrap();
    let exchange = code_exchange::ActorCodeExchange::new(ctx);
    let body = respond(payload.body, payload.from, exchange);

    Ok(HttpResponse {
        body: body.as_bytes().to_vec(),
        ..Default::default()
    })
}

#[cfg(not(test))]
fn respond<E: code_exchange::CodeExchange>(
    prompt: String,
    prompter: String,
    exchange: E,
) -> String {
    responder::handle(prompt, prompter, exchange)
}

#[cfg(test)]
fn respond<C: code_exchange::CodeExchange>(
    prompt: String,
    prompter: String,
    _ex: C,
) -> String {
    format!("from: {}, body: {}", prompter, prompt)
}

#[cfg(test)]
mod test {
    use crate::handle_http_request;
    use std::fs;
    use wasmbus_rpc::actor::prelude::*;
    use wasmcloud_interface_httpserver::HttpRequest;
    extern crate serde_json as json;

    #[test]
    fn can_handle_request() {
        let req = fs::read_to_string("fixtures/request.json").unwrap();
        let req: HttpRequest = json::from_str(&req).unwrap();
        let ctx: Context = Default::default();
        let resp = handle_http_request(&ctx, &req).unwrap();

        assert_eq!(resp.status_code, 200);
        assert_eq!(
            String::from_utf8(resp.body).unwrap(),
            "from: +14108025604, body: test".to_string()
        );
    }
}

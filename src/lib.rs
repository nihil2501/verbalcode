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
        _ctx: &Context,
        req: &HttpRequest,
    ) -> RpcResult<HttpResponse> {
        handle_http_request(req)
    }
}

#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate serde_qs as qs;

mod responder;
mod twilio {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Payload {
        pub body: String,
        pub from: String,
    }
}

fn handle_http_request(req: &HttpRequest) -> RpcResult<HttpResponse> {
    let payload: twilio::Payload = qs::from_bytes(req.body.as_slice()).unwrap();
    let body = respond(payload.body, payload.from);

    Ok(HttpResponse {
        body: body.as_bytes().to_vec(),
        ..Default::default()
    })
}

#[cfg(not(test))]
fn respond(prompt: String, prompter: String) -> String {
    responder::handle(prompt, prompter)
}

#[cfg(test)]
fn respond(prompt: String, prompter: String) -> String {
    format!("from: {}, body: {}", prompter, prompt)
}

#[cfg(test)]
mod test {
    use crate::handle_http_request;
    use std::fs;
    use wasmcloud_interface_httpserver::HttpRequest;

    #[test]
    fn can_handle_request() {
        let request = fs::read_to_string("fixtures/request.json").unwrap();
        let request: HttpRequest = json::from_str(&request).unwrap();
        let response = handle_http_request(&request).unwrap();

        assert_eq!(response.status_code, 200);
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            "from: +14108025604, body: test".to_string()
        );
    }
}

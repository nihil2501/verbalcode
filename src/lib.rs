use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{
    HttpRequest, HttpResponse, HttpServer, HttpServerReceiver,
};
// use wasmcloud_interface_logging::info;

use std::result;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct VerbalcodeActor {}

/// Implementation of HttpServer trait methods
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

mod twilio {
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Payload {
        pub body: String,
        pub from: String,
    }
}

fn handle_http_request(
    req: &HttpRequest,
) -> result::Result<HttpResponse, RpcError> {
    let payload: twilio::Payload = qs::from_bytes(req.body.as_slice()).unwrap();

    Ok(HttpResponse {
        body: format!("from: {}, body: {}", payload.from, payload.body)
            .as_bytes()
            .to_vec(),
        ..Default::default()
    })
}

#[cfg(test)]
mod test {
    use crate::handle_http_request;
    use std::fs;
    use wasmcloud_interface_httpserver::HttpRequest;

    #[test]
    fn can_handle_request() {
        let request: HttpRequest = json::from_str(
            &fs::read_to_string("fixtures/request.json").unwrap(),
        )
        .unwrap();
        let response = handle_http_request(&request).unwrap();

        assert_eq!(response.status_code, 200);
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            "from: +14108025604, body: test".to_string()
        );
    }
}

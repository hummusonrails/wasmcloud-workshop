use wasmcloud_interface_logging::info;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, Handlers};

struct HelloWorld;

impl Handlers for HelloWorld {
    fn handle_request(req: HttpRequest) -> HttpResponse {
        let response_text = format!("Hello, wasmCloud! You requested: {}", req.path);
        info!("{}", response_text);
        HttpResponse {
            body: response_text.into_bytes(),
            ..Default::default()
        }
    }
}

wasmcloud_interface_httpserver::export!(HelloWorld);

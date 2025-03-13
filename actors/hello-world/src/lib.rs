use wasmcloud_component::http;
use wasmcloud_component::http::ErrorCode;
use wasmcloud_component::wasi::keyvalue::{self, store, atomics};
use wasmcloud_component::{info, error}; 

struct Component;

http::export!(Component);

impl http::Server for Component {
    fn handle(request: http::IncomingRequest) 
        -> http::Result<http::Response<impl http::OutgoingBody>> 
    {
        // Split the request into parts (headers, etc.) and body stream
        let (parts, body_stream) = request.into_parts();
        let path = parts.uri.path();
        let query = parts.uri.query().unwrap_or_default().to_string();

        // Parse a "name" query parameter
        let name = match query.split('=').collect::<Vec<&str>>()[..] {
            ["name", value] => value,
            _ => "World",
        };

        // Log an info message (using wasi:logging under the hood)
        info!("Received request for '{}', responding to {}", path, name);

        // Use key-value store (wasi:keyvalue) to count visits for the name
        let bucket = store::open("default").map_err(|e| {
            // Convert any error to an HTTP 500 InternalError with a message
            ErrorCode::InternalError(Some(format!("KV open error: {:?}", e)))
        })?;
        let count = atomics::increment(&bucket, name, 1).map_err(|e| {
            ErrorCode::InternalError(Some(format!("KV increment error: {:?}", e)))
        })?;

        // Form the response body
        let body_text = format!("Hello x{} from wasmCloud, {}!\n", count, name);

        // Return an OK 200 response with the body text
        Ok(http::Response::new(body_text))
    }
}

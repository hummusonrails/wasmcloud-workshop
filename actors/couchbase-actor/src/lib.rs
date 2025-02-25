use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, Handlers};
use wasmcloud_interface_couchbase::{Couchbase, CouchbaseSender};
use std::env;
use std::str;

struct CouchbaseActor;

impl Handlers for CouchbaseActor {
    fn handle_request(req: HttpRequest) -> HttpResponse {
        let cb = CouchbaseSender::new();

        // Fetch connection details from environment variables
        let host = env::var("COUCHBASE_HOST").unwrap_or_default();
        let username = env::var("COUCHBASE_USERNAME").unwrap_or_default();
        let password = env::var("COUCHBASE_PASSWORD").unwrap_or_default();
        let bucket = env::var("COUCHBASE_BUCKET").unwrap_or_default();

        if host.is_empty() || username.is_empty() || password.is_empty() || bucket.is_empty() {
            return HttpResponse {
                body: "Missing Couchbase Capella credentials!".into_bytes(),
                ..Default::default()
            };
        }

        // Store key-value
        cb.set("workshop-key".to_string(), "Hello from wasmCloud!".as_bytes().to_vec());

        // Retrieve value
        let result = cb.get("workshop-key".to_string());
        let stored_value = str::from_utf8(&result.unwrap()).unwrap_or("No data found");

        HttpResponse {
            body: format!("Retrieved from Couchbase: {}", stored_value).into_bytes(),
            ..Default::default()
        }
    }
}

wasmcloud_interface_httpserver::export!(CouchbaseActor);

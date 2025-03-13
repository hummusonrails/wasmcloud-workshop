use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, Handlers};
use wasmcloud_interface_couchbase::{Couchbase, CouchbaseSender};
use wasmcloud_interface_logging::{info, error, warn};
use std::str;

struct CouchbaseActor;

impl Handlers for CouchbaseActor {
    fn handle_request(req: HttpRequest) -> HttpResponse {
        // The document key is taken from the URL path (everything after the '/')
        let key = req.path.trim_start_matches('/').to_string();
        if key.is_empty() {
            warn!("Received request with empty key");
            return HttpResponse {
                body: b"Key must be provided in the URL path".to_vec(),
                status_code: 400,
                ..Default::default()
            };
        }

        match req.method.as_str() {
            "GET" => handle_get(key),
            "POST" => handle_post(key, req.body),
            "PUT" => handle_put(key, req.body),
            "DELETE" => handle_delete(key),
            _ => HttpResponse {
                body: b"Method Not Allowed".to_vec(),
                status_code: 405,
                ..Default::default()
            },
        }
    }
}

fn handle_get(key: String) -> HttpResponse {
    info!("Handling GET for key: {}", key);
    let cb = CouchbaseSender::new();
    match cb.get_document(key.clone()) {
        Ok(opt) => {
            if let Some(doc) = opt {
                HttpResponse {
                    body: doc,
                    status_code: 200,
                    ..Default::default()
                }
            } else {
                HttpResponse {
                    body: b"Document not found".to_vec(),
                    status_code: 404,
                    ..Default::default()
                }
            }
        }
        Err(e) => {
            error!("Error retrieving document for key {}: {}", key, e);
            HttpResponse {
                body: format!("Error retrieving document: {}", e).into_bytes(),
                status_code: 500,
                ..Default::default()
            }
        }
    }
}

fn handle_post(key: String, body: Vec<u8>) -> HttpResponse {
    info!("Handling POST (create) for key: {}", key);
    let cb = CouchbaseSender::new();
    // Insert document – expected to fail if the key already exists.
    match cb.insert_document(key.clone(), body.clone()) {
        Ok(_) => HttpResponse {
            body,
            status_code: 201, // Created
            ..Default::default()
        },
        Err(e) => {
            if e.to_lowercase().contains("exists") {
                HttpResponse {
                    body: b"Document already exists".to_vec(),
                    status_code: 409,
                    ..Default::default()
                }
            } else {
                error!("Error inserting document for key {}: {}", key, e);
                HttpResponse {
                    body: format!("Error inserting document: {}", e).into_bytes(),
                    status_code: 500,
                    ..Default::default()
                }
            }
        }
    }
}

fn handle_put(key: String, body: Vec<u8>) -> HttpResponse {
    info!("Handling PUT (upsert) for key: {}", key);
    let cb = CouchbaseSender::new();
    // Upsert document regardless of existence
    match cb.upsert_document(key.clone(), body.clone()) {
        Ok(_) => HttpResponse {
            body,
            status_code: 200,
            ..Default::default()
        },
        Err(e) => {
            error!("Error upserting document for key {}: {}", key, e);
            HttpResponse {
                body: format!("Error upserting document: {}", e).into_bytes(),
                status_code: 500,
                ..Default::default()
            }
        }
    }
}

fn handle_delete(key: String) -> HttpResponse {
    info!("Handling DELETE for key: {}", key);
    let cb = CouchbaseSender::new();
    match cb.remove_document(key.clone()) {
        Ok(_) => HttpResponse {
            body: Vec::new(),
            status_code: 204, // No Content
            ..Default::default()
        },
        Err(e) => {
            if e.to_lowercase().contains("not found") {
                HttpResponse {
                    body: b"Document not found".to_vec(),
                    status_code: 404,
                    ..Default::default()
                }
            } else {
                error!("Error deleting document for key {}: {}", key, e);
                HttpResponse {
                    body: format!("Error deleting document: {}", e).into_bytes(),
                    status_code: 500,
                    ..Default::default()
                }
            }
        }
    }
}

wasmcloud_interface_httpserver::export!(CouchbaseActor);

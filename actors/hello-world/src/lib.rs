use wasmcloud_component::http;
use wasmcloud_interface_couchbase::CouchbaseSender;
use serde_json::json; 
use serde_json::Value; 
use std::str;

struct Component;

// Export the HTTP actor component 
http::export!(Component);

pub async fn call_couchbase_actor_get_items() -> Result<Value, String> {
    // Replace with the actual public key (subject) or call alias of your Couchbase actor.
    // The subject is a 56-character string starting with 'M' (e.g., "Mxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
    let couchbase_actor_id = "MXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    // Create a sender handle to the Couchbase actor.
    let sender = CouchbaseSender::for_actor(couchbase_actor_id)
        .map_err(|e| format!("Failed to create Couchbase sender: {}", e))?;

    // Invoke the 'get_items' operation on the Couchbase actor.
    // This method returns a Result with a vector of bytes.
    let result_bytes = sender.get_items().await
        .map_err(|e| format!("Failed to invoke get_items: {}", e))?;

    // Parse the returned bytes as JSON.
    let items: Value = serde_json::from_slice(&result_bytes)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(items)
}

impl http::Server for Component { 
    fn handle(request: http::IncomingRequest) -> http::Result<http::Response<impl http::OutgoingBody>> { 
        // Check if the request method is GET. 
        if request.method.to_uppercase() == "GET" { 
            match call_couchbase_actor_get_items() { 
                Ok(items) => { 
                    // Convert the JSON value to a pretty-printed string. 
                    let body = serde_json::to_string_pretty(&items) .unwrap_or_else(|_| "Error formatting response".to_string()); 
                    Ok(http::Response::new(body)) 
                }, 
                Err(err) => { 
                    Ok(http::Response::new(format!("Failed to fetch items: {}\n", err))) 
                }, 
            } 
        } 
        else 
        { 
            // For non-GET methods, return an error response. 
            Ok(http::Response::new("Unsupported HTTP method\n".to_string())) 
        } 
    } 
}
use std::collections::HashMap;

/// Represents an API request to be sent to vroom
#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub body: String,
}

impl Request {
    pub fn from_hashmap(hashmap: HashMap<String, String>) -> Result<Self, magnus::Error> {
        // Check presence of body key value pair in the hash
        // required to build the vroom request
        let body = match hashmap.get("body") {
            Some(json_string) => json_string.to_string(),
            None => {
                let rb_error = magnus::Error::new(
                    magnus::exception::arg_error(),
                    "expected hashes with url and body key pairs",
                );
                return Err(rb_error);
            }
        };

        // Check presence of the vroom url environment variable
        // required to build the vroom request
        let url = match std::env::var("VROOM_URL") {
            Ok(vroom_url) => vroom_url,
            Err(_) => {
                let rb_error = magnus::Error::new(
                    magnus::exception::arg_error(),
                    "missing environment variable VROOM_URL",
                );
                return Err(rb_error);
            }
        };

        Ok(Self { url, body })
    }
}

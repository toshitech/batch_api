use std::collections::HashMap;

// Sort id is to optionally sort responses
// in the same order they were sent

#[derive(Debug)]
pub struct Response {
    pub sort_key: i32,
    pub body: String,
    pub http_status_code: u16,
}

impl Response {
    /// consumes self and returns a ruby convertable rust type
    pub fn into_hashmap(self) -> HashMap<String, String> {
        let mut rbarray_convertable_hashmap: HashMap<String, String> = HashMap::with_capacity(1);
        println!("dog");
        // Insert hash for all fields on Request
        rbarray_convertable_hashmap.insert(String::from("body"), self.body);
        rbarray_convertable_hashmap.insert(
            String::from("http_status_code"),
            self.http_status_code.to_string(),
        );
        rbarray_convertable_hashmap
    }
}

use std::collections::HashMap;

use magnus::{function, prelude::*, RArray, Ruby};

// No need to implement classes or anything at this point
// just module with functions
// note this https://github.com/matsadler/magnus?tab=readme-ov-file#safety

// Pretty sure Vroom is just POSTs requests
#[derive(Debug)]
struct VroomRequest {
  url: String,
  body: Option<String>
}

// Sort id is to optionally sort responses
// in the same order they were sent
#[derive(Debug)]
struct ApiResponse {
  sort_key: i32,
  body: String
}

/// Wrap for some error handling
/// requires hashes to include url and body key value pairs
/// for the vroom request methods
/// { 'url' => 'value', 'body' => 'value' }
struct VroomRequestArgument(HashMap<String, String>);

impl VroomRequestArgument {
  fn access_hash_with_vroom_checks(&self, key: &str) -> Result<String, String> {
    if let Some(val) = self.0.get(key) {
      Ok(val.clone())
    } else {
      Err(format!("missing hash key {}", key))
    }
  }
}

/// Wrap batch_send_vroom_api_requests() function
fn blocking_batch_send_vroom_requests(requests: Vec<HashMap<String, String>>) -> Result<RArray, magnus::Error> {
  let vroom_requests: Vec<Result<(String, String), String>> = requests.into_iter().map(|rhash| {
    let arg = VroomRequestArgument(rhash);

    let url = arg.access_hash_with_vroom_checks("url")?;
    let body = arg.access_hash_with_vroom_checks("body")?;

    Ok((url, body))
  }).collect();

  // If there were any errors immediately return them to ruby vm
  for result_ref in vroom_requests.iter() {
    if result_ref.is_err() {
      let rb_error = magnus::Error::new(magnus::exception::arg_error(), "expected hashes with url and body key pairs");

      return Err(rb_error);
    }
  }

  let vroom_requests: Vec<VroomRequest> = vroom_requests.into_iter().map(|r|{
    let (url, body) = r.unwrap();
    VroomRequest { url, body: if body.is_empty() { None } else  { Some(body) } }
  }).collect();


  // Execute async API calls on a single threaded runtime
  let rt = tokio::runtime::Builder::new_current_thread().enable_all().max_blocking_threads(1).build().unwrap();
  let async_return_value = rt.block_on(batch_send_vroom_api_requests(vroom_requests));

  Ok(RArray::from_vec(async_return_value))
}

/// Execute API calls asynchronously with tokio
// Only needs to handle Plan -> Vroom format for now
async fn batch_send_vroom_api_requests(requests: Vec<VroomRequest>) -> Vec<String> {
  let mut responses: Vec<ApiResponse> = Vec::with_capacity(requests.len());
  let mut set = tokio::task::JoinSet::new();

  for (sort_key, r) in requests.into_iter().enumerate() {
    set.spawn(async move {
        let mut client = reqwest::Client::new().get(r.url);
        // Add support for different methods
        if let Some(body) = r.body {
          client = client.body(body);
        }

        let res = client.send().await.unwrap();

        ApiResponse { sort_key: sort_key as i32, body: res.text().await.unwrap() }
    });
  }

  // Run the joinset to completion, they return in the order they finish
  // so need to sort them again after
  while let Some(res) = set.join_next().await {
    let api_response = res.unwrap();
    responses.push(api_response);
  }

  // order results in the same order they came in
  responses.sort_by_key(|r| r.sort_key);

  // respond just with the response bodies for now
  responses.into_iter().map(|r| r.body ).collect()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("BatchApi")?;
    module.define_singleton_method("batch_send_vroom_api_requests", function!(blocking_batch_send_vroom_requests, 1))?;
    Ok(())
}

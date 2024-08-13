use std::collections::HashMap;
use std::sync::Arc;

use super::request::Request;
use super::response::Response;

// magnus converts the ruby hash to rust types for us
type RbArrayOfHashes = Vec<HashMap<String, String>>;

/// Sends vroom api requests async using single threaded tokio runtime
/// Raises ruby exceptions if the arguments are not hashes and if the VROOM_URL env var is not present
pub fn rb_batch_send_vroom_requests(rb_array_of_hashes: RbArrayOfHashes) -> Result<magnus::RArray, magnus::Error> {
  // Take ruby argument, converted to rust types
  // then convert them into vroom requests, which will validate them also
  let mut vroom_requests: Vec<Request> = Vec::new();

  for rb_hash_as_rust_type in rb_array_of_hashes.into_iter() {
    let request = Request::from_hashmap(rb_hash_as_rust_type)?;
    vroom_requests.push(request);
  }

  // send the requests and return vroom responses on a single threaded tokio runtime
  let rt = tokio::runtime::Builder::new_current_thread().enable_all().max_blocking_threads(1).build().unwrap();
  let vroom_responses = rt.block_on(batch_send_vroom_api_requests(vroom_requests));

  // convert them from vroom responses types back into rust types convertable to ruby
  let ruby_array_of_hash_responses: RbArrayOfHashes = vroom_responses.into_iter().map(|response| response.into_hashmap() ).collect();

  Ok(magnus::RArray::from_vec(ruby_array_of_hash_responses))
}

/// Execute API calls async with reqwest
async fn batch_send_vroom_api_requests(requests: Vec<Request>) -> Vec<Response> {
  let mut responses: Vec<Response> = Vec::with_capacity(requests.len());
  let mut set = tokio::task::JoinSet::new();

  let client: Arc<reqwest::Client> = Arc::new(reqwest::Client::new());

  for (sort_key, r) in requests.into_iter().enumerate() {
    let client: Arc<reqwest::Client> = Arc::clone(&client);

    set.spawn(async move {
      let reqwest_response = client.post(r.url)
                                   .header("Content-Type", "application/json")
                                   .body(r.body)
                                   .send()
                                   .await
                                   .unwrap();

      let http_status_code = reqwest_response.status().as_u16();
      // consumes self so do it after we get the status code
      let body = reqwest_response.text().await.unwrap_or("".to_string());

      Response { sort_key: sort_key as i32, http_status_code, body }
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
  responses
}

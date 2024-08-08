mod vroom;

use magnus::{function, prelude::*, Ruby};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("BatchApi")?;
    module.define_singleton_method("batch_send_vroom_api_requests", function!(vroom::api::rb_batch_send_vroom_requests, 1))?;
    Ok(())
}

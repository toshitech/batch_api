mod vroom;
mod zipcode_verification;

use magnus::{class, function, method, prelude::*, Object, Ruby};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("BatchApi")?;
    // Vroom stuff
    let vroom = module.define_module("Vroom")?;
    vroom.define_module_function(
        "batch_send_api_requests",
        function!(vroom::api::rb_batch_send_vroom_requests, 1),
    )?;

    // KMZ / KML utilities
    let kml_utilities = module.define_module("KmlUtilities")?;

    kml_utilities.define_singleton_method(
        "uncompress_kmz_to_kml",
        function!(zipcode_verification::zip::rb_uncompress_kmz_to_kml, 2),
    )?;

    kml_utilities.define_singleton_method(
        "compress_kml_to_kmz",
        function!(zipcode_verification::zip::rb_compress_kml_to_kmz, 2),
    )?;

    // Zipcode verification stuff
    let zipcode_verification = module.define_module("ZipcodeVerification")?;
    let memstore_ruby = zipcode_verification.define_class("MemStore", class::object())?;

    memstore_ruby.define_singleton_method(
        "new",
        function!(zipcode_verification::storage::MutMemStore::rb_new, 0),
    )?;

    memstore_ruby.define_method(
        "load_uk_sectors_from_kmz_file",
        method!(
            zipcode_verification::storage::MutMemStore::rb_load_uk_sectors_from_kmz_file,
            1
        ),
    )?;

    memstore_ruby.define_method(
        "load_ny_sectors_from_kmz_file",
        method!(
            zipcode_verification::storage::MutMemStore::rb_load_ny_sectors_from_kmz_file,
            1
        ),
    )?;

    memstore_ruby.define_method(
        "load_ca_sectors_from_kmz_file",
        method!(
            zipcode_verification::storage::MutMemStore::rb_load_ca_sectors_from_kmz_file,
            1
        ),
    )?;

    memstore_ruby.define_method(
        "load_nj_sectors_from_kmz_file",
        method!(
            zipcode_verification::storage::MutMemStore::rb_load_nj_sectors_from_kmz_file,
            1
        ),
    )?;

    memstore_ruby.define_method(
        "query",
        method!(zipcode_verification::storage::MutMemStore::rb_query, 3),
    )?;
    Ok(())
}

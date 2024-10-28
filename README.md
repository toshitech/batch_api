# BatchApi (old name, now PlanUtils)

I'll change the project repo some other time.

Contains the odd routing utility library for TOSHI Plan.

Currently supports:
- Batched async API calls to Vroom bypassing Ruby's GIL.
- Zipcode verification / geocoding using KMZ (compressed KML) files.

## Installing
- Add gem to your gemfile with
```ruby
  gem 'batch_api', github: '/toshitech/batch_api'
```
- set `VROOM_URL` env variable
- `bundle install`

## Usage

### Batching Vroom API calls
```ruby
require 'batch_api'
# Batch send API requests to vroom

request = { 'body' => vroom_request_body_json }
requests = [request] * 10
responses = BatchApi::Vroom.batch_send_api_requests(requests)
# Returns array of hashes containing vroom response details
# {
#   'http_status_code': '200',
#   'body': 'json string'
# }

# Vroom codes
# 0	no error raised
# 1	internal error
# 2	input error
# 3	routing error

successful_responses = []
failed_responses = []

responses.each do |r|
  if r['body'].size > 0 && JSON.parse(r['body'])['code'] == 0
    succesful_responses << r
  else
    failed_responses << r
  end
end
```

### KML Utilities

```ruby
# Compress KML file to KMZ
BatchApi::KmlUtilities.compress_kml_to_kmz("input.kml", "output.kmz")
# Uncompress a KMZ file to KML
BatchApi::KmlUtilities.uncompress_kmz_to_kml("input.kmz", "output.kml")
```

### Zipcode verification

```ruby
require 'batch_api'

mem = BatchApi::ZipcodeVerification::MemStore.new
# Load location based KMZ files. As of right now because the format
# of data you can find is complete different for each location
# these are specific files.
# For our case we want a lot of data loaded into memory here
# so make mem part of app state and reuse it.
mem.load_uk_sectors_from_kmz_file('./uk.kmz')
# mem.load_ny_sectors_from_kmz_file('./ny.kmz')
# mem.load_la_sectors_from_kmz_file('./la.kmz')

# Query a lat lng in a location to return the postcode sector.
mem.query("uk", 53.9591450, -1.0792350)
# Returns string 'YO1 8'

# Full countries
# "uk" united kingdom

# US states
# "ca" california
# "ny" new york
# "nj" new jersey
```

## Troubleshooting
### Compiling locally
bundle install requires rust toolchain to be installed to compile the gem, install rust with:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
then compile with:
```bash
bundle exec rake compile
```

### Compiling in your ruby application's dockerfile
When bundle installing in a dockerfile, add the following to allow compilation at build time:
```dockerfile
# Needed for rb-sys
RUN apt-get install -y libclang-dev
# Get Rust toolchain
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
# To find cargo
ENV PATH="/root/.cargo/bin:$PATH"
# then run your bundle install after
RUN bundle install
```

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

# BatchApi

For sending some service API calls async

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
responses = BatchApi.batch_send_vroom_api_requests(requests)
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

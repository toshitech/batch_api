# frozen_string_literal: true

require 'json'

RSpec.describe BatchApi do
  it "has a version number" do
    expect(BatchApi::VERSION).not_to be nil
  end

  describe '#batch_send_vroom_api_requests' do
    context 'incorrectly formatted argument' do
      let(:requests) do
        [
          { 'not_url' => 'http://somethingrandom.com', 'not_body' => ''},
          { 'not_body' => '' },
          {}
        ]
      end

      it 'raises argument errors' do
        requests.each do |request|
          expect { BatchApi.batch_send_vroom_api_requests([request]) }.to raise_error(ArgumentError)
        end
      end
    end

    context 'with a request containing vroom data' do
      let(:requests) { [{ 'url' => 'http://httpbin.org/delay/1', 'body' => ''}] * 10 }
      xit 'returns the request responses' do
        responses = BatchApi.batch_send_vroom_api_requests(requests)
        expect(responses.size).to be 10
      end
    end

    context 'passing empty array argument' do
      it 'returns an empty array' do
        expect(BatchApi.batch_send_vroom_api_requests([])).to eq([])
      end
    end
  end
end
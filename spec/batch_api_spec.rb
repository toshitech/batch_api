# frozen_string_literal: true

require 'json'

RSpec.describe BatchApi do
  it "has a version number" do
    expect(BatchApi::VERSION).not_to be nil
  end


  describe BatchApi::KmlUtilities do
    describe 'rust interface' do
      it 'defines our methods' do
        expect(BatchApi::KmlUtilities).to respond_to(:uncompress_kmz_to_kml)
        expect(BatchApi::KmlUtilities).to respond_to(:compress_kml_to_kmz)
      end
    end
  end

  describe BatchApi::ZipcodeVerification::MemStore do
    let(:storage) { BatchApi::ZipcodeVerification::MemStore.new }
    describe '#new' do
      it 'correctly initialises the ruby object of the rust struct' do
        # just dont break
        expect(storage).to be_a BatchApi::ZipcodeVerification::MemStore
      end
    end

    describe 'rust interface' do
      it 'defines our methods' do
        expect(storage).to respond_to(:query)

        expect(storage).to respond_to(:load_uk_sectors_from_kmz_file)
        expect(storage).to respond_to(:load_ny_sectors_from_kmz_file)
        expect(storage).to respond_to(:load_ca_sectors_from_kmz_file)
        expect(storage).to respond_to(:load_nj_sectors_from_kmz_file)
      end
    end
  end

  describe BatchApi::Vroom do
    describe '#batch_send_api_requests' do
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
            expect { BatchApi::Vroom.batch_send_api_requests([request]) }.to raise_error(ArgumentError)
          end
        end
      end

      context 'with a request containing vroom data' do
        let(:requests) { [{ 'url' => 'http://httpbin.org/delay/1', 'body' => ''}] * 10 }
        xit 'returns the request responses' do
          responses = BatchApi::Vroom.batch_send_api_requests(requests)
          expect(responses.size).to be 10
        end
      end

      context 'passing empty array argument' do
        it 'returns an empty array' do
          expect(BatchApi::Vroom.batch_send_api_requests([])).to eq([])
        end
      end
    end
  end
end

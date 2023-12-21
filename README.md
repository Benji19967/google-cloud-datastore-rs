# Rust Client for Google Cloud Datastore

There are clients available for Google Cloud Storage, such as https://github.com/yoshidan/google-cloud-rust/tree/main
but none for Datastore. Hence, I will try to write one for Datastore.

## Resources

### gRPC
- https://googleapis.github.io/HowToRPC.html
  - How to call Google RPC APIs
- https://github.com/googleapis/googleapis/blob/master/google/datastore/v1/datastore.proto
  - Proto definitions for Datastore
- https://github.com/hyperium/tonic
  - Rust gRPC crate
- https://github.com/tokio-rs/prost
  - A Protocol Buffers implementation for Rust

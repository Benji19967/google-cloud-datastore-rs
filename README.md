# Rust Client for Google Cloud Datastore

There are clients available for Google Cloud Storage, such as https://github.com/yoshidan/google-cloud-rust/tree/main
but none for Datastore. Hence, I will try to write one for Datastore.

## Google Cloud

### Auth

`gcloud` needs to be installed:

```bash
brew install --cask google-cloud-sdk
```

Authenticate: 

```bash
gcloud auth login
```

Set the project id:

```bash
gcloud config set project <project_id>
```

You're good to go! You can run commands such as:

```bash
gcloud storage buckets list
```


## Proto

To be able to use `tonic`: https://github.com/hyperium/tonic
```bash
brew install protobuf
```

Getting the Google RPC protos:
```bash
cd proto
git submodule add https://github.com/googleapis/googleapis
git submodule update --remote
```

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

### Datastore Client Example using gRPC

- https://github.com/i110/google-cloud-rs/blob/c826356348972c859886f9b439d183a50dcf9047/google-cloud/src/datastore/client.rs

### TLS and SSL certs

- https://jessitron.com/2022/11/02/make-https-work-on-grpc-in-rust-load-a-root-certificate-into-the-tls-config/

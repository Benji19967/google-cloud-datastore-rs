pub mod api {
    tonic::include_proto!("google.datastore.v1");
}
use api::datastore_client::DatastoreClient;
use api::key::path_element::IdType;
use api::key::PathElement;
use api::read_options::ConsistencyType;
use api::read_options::ReadConsistency;
use api::Key;
use api::LookupRequest;
use api::PartitionId;
use api::ReadOptions;
use gcp_auth::AuthenticationManager;
use std::env;
use std::fs::File;
use tonic::metadata::AsciiMetadataValue;
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::IntoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let bxdx_project_id = env::var("BXDX_PROJECT_ID").expect("$BXDX_PROJECT_ID is not set");

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let request = tonic::Request::new(LookupRequest {
        database_id: "".into(),
        project_id: bxdx_project_id.clone(),
        read_options: Some(ReadOptions {
            consistency_type: Some(ConsistencyType::ReadConsistency(
                ReadConsistency::Strong.into(),
            )),
        }),
        keys: vec![Key {
            partition_id: {
                Some(PartitionId {
                    database_id: "".into(),
                    namespace_id: "".into(),
                    project_id: bxdx_project_id.clone(),
                })
            },
            path: vec![PathElement {
                kind: "source".into(),
                id_type: Some(IdType::Id(5634161670881280)),
            }],
        }]
        .into(),
    });

    // let path = env::var("GOOGLE_APPLICATION_CREDENTIALS")?;
    // let file = File::open(path)?;
    // let creds = serde_json::from_reader(file)?;

    const DOMAIN_NAME: &'static str = "datastore.googleapis.com";
    const ENDPOINT: &'static str = "https://datastore.googleapis.com";
    const TLS_CERTS: &[u8] = include_bytes!("/etc/ssl/cert.pem");
    // let TOKEN: &'static str = token.clone().as_str();
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(TLS_CERTS))
        .domain_name(DOMAIN_NAME);
    let channel = Channel::from_static(ENDPOINT)
        .tls_config(tls_config)?
        .connect()
        .await?;
    let mut client = DatastoreClient::new(channel);
    let mut request = request.into_request();
    let metadata = request.metadata_mut();

    // Adding Bearer worked: seems like the gRPC metadata for the request is similar to HTTP headers.
    let tokenfinal: MetadataValue<_> = format!("Bearer {}", token.as_str()).parse().unwrap();
    println!("{:?}", tokenfinal.to_str());
    metadata.insert("authorization", tokenfinal);
    let response = client.lookup(request).await.unwrap();
    let response_message = response.into_inner();

    println!("{:?}", response_message);

    Ok(())
}

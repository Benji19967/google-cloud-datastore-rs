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
use std::env;
use std::fs::File;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::IntoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(LookupRequest {
        database_id: "".into(),
        // TODO: Add project_id
        project_id: "bxdx-dev".into(),
        read_options: Some(ReadOptions {
            consistency_type: Some(ConsistencyType::ReadConsistency(
                ReadConsistency::Strong.into(),
            )),
        }),
        keys: vec![Key {
            partition_id: {
                Some(PartitionId {
                    database_id: "".into(),
                    // TODO: Add namespace_id and project
                    namespace_id: "default".into(),
                    project_id: "bxdx-dev".into(),
                })
            },
            path: vec![PathElement {
                kind: "Source".into(),
                id_type: Some(IdType::Id(79797)),
            }],
        }]
        .into(),
    });

    let path = env::var("GOOGLE_APPLICATION_CREDENTIALS")?;
    let file = File::open(path)?;
    let creds = serde_json::from_reader(file)?;

    const DOMAIN_NAME: &'static str = "datastore.googleapis.com";
    const ENDPOINT: &'static str = "https://datastore.googleapis.com";
    const TLS_CERTS: &[u8] = include_bytes!("/etc/ssl/cert.pem");
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
    metadata.insert("authorization", token);
    let response = client.lookup(request).await.unwrap();
    let response_message = response.into_inner();

    println!("{:?}", response_message);

    Ok(())
}

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
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::IntoRequest;

mod auth;
mod settings;

use auth::AuthManager;
use settings::SETTINGS;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let bxdx_gcp_project_id = env::var("BXDX_PROJECT_ID").expect("$BXDX_PROJECT_ID is not set");
    let bxdx_gcp_project_id = &SETTINGS.gcp_project_id;

    let request = tonic::Request::new(LookupRequest {
        database_id: "".into(),
        project_id: bxdx_gcp_project_id.clone(),
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
                    project_id: bxdx_gcp_project_id.clone(),
                })
            },
            path: vec![PathElement {
                kind: "source".into(),
                id_type: Some(IdType::Id(5634161670881280)),
            }],
        }]
        .into(),
    });

    const TLS_CERTS: &[u8] = include_bytes!("/etc/ssl/cert.pem");
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(TLS_CERTS))
        .domain_name(&SETTINGS.gcp.domain_name);
    let channel = Channel::from_static(&SETTINGS.gcp.endpoint)
        .tls_config(tls_config)?
        .connect()
        .await?;
    let mut client = DatastoreClient::new(channel);
    let mut request = request.into_request();
    let metadata = request.metadata_mut();

    let token = AuthManager::new().await?.get_token().await?;
    // Adding Bearer worked: seems like the gRPC metadata for the request is similar to HTTP headers.
    let tokenfinal: MetadataValue<_> = format!("Bearer {}", token.as_str()).parse().unwrap();
    metadata.insert("authorization", tokenfinal);
    let response = client.lookup(request).await.unwrap();
    let response_message = response.into_inner();

    println!("{:?}", response_message);

    Ok(())
}

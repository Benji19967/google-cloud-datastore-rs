pub mod api {
    tonic::include_proto!("google.datastore.v1");
}
use api::key::path_element::IdType;
use api::key::PathElement;
use api::read_options::ConsistencyType;
use api::read_options::ReadConsistency;
use api::Key;
use api::LookupRequest;
use api::PartitionId;
use api::ReadOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(LookupRequest {
        database_id: "".into(),
        // TODO: Add project_id
        project_id: "".into(),
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
                    project_id: "".into(),
                })
            },
            path: vec![PathElement {
                kind: "Source".into(),
                id_type: Some(IdType::Id(79797)),
            }],
        }]
        .into(),
    });

    // TODO: Send the request
    // let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);

    Ok(())
}

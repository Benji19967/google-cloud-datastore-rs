pub mod api {
    tonic::include_proto!("google.datastore.v1");
}
use api::datastore_client::DatastoreClient;
use std::collections::HashMap;
use tonic::transport::Channel;

pub struct DSutils {
    clients: HashMap<String, DatastoreClient<Channel>>, // one client per GCP project
}

impl DSutils {
    pub fn get_entity() {
        unimplemented!()
    }
    pub fn get_entity_by_key() {
        unimplemented!()
    }
    pub fn get_entities() {
        unimplemented!()
    }
    pub fn get_entities_by_kind() {
        unimplemented!()
    }
    pub fn put() {
        unimplemented!()
    }
    pub fn put_multi() {
        unimplemented!()
    }
    pub fn delete() {
        unimplemented!()
    }
    pub fn delete_multi() {
        unimplemented!()
    }
    pub fn create_complete_keys() {
        unimplemented!()
    }
    pub fn query() {
        unimplemented!()
    }
}

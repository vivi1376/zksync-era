use std::sync::Arc;

use zksync_web3_decl::jsonrpsee::http_client::HttpClient;

use crate::resource::{Resource, ResourceId};

#[derive(Debug, Clone)]
pub struct MainNodeClientResource(pub Arc<HttpClient>);

impl Resource for MainNodeClientResource {
    fn resource_id() -> ResourceId {
        "common/main_node_client".into()
    }
}

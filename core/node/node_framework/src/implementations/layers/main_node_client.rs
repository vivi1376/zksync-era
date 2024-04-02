use std::sync::Arc;

use anyhow::Context;
use zksync_core::sync_layer::MainNodeClient;

use crate::{
    implementations::resources::main_node_client::MainNodeClientResource,
    service::ServiceContext,
    wiring_layer::{WiringError, WiringLayer},
};

#[derive(Debug)]
pub struct MainNodeClientLayer {
    main_node_url: String,
}

impl MainNodeClientLayer {
    pub fn new(main_node_url: String) -> Self {
        Self { main_node_url }
    }
}

#[async_trait::async_trait]
impl WiringLayer for MainNodeClientLayer {
    fn layer_name(&self) -> &'static str {
        "main_node_client_layer"
    }

    async fn wire(self: Box<Self>, mut context: ServiceContext<'_>) -> Result<(), WiringError> {
        let main_node_client = <dyn MainNodeClient>::json_rpc(&self.main_node_url)
            .context("Failed creating JSON-RPC client for main node")?;
        context.insert_resource(MainNodeClientResource(Arc::new(main_node_client)))?;
        Ok(())
    }
}

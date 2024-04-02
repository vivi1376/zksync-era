use std::sync::Arc;

use zksync_core::block_reverter::{BlockReverter, L1ExecutedBatchesRevert, NodeRole};

use crate::{
    implementations::resources::{pools::MasterPoolResource, reverter::BlockReverterResource},
    service::ServiceContext,
    wiring_layer::{WiringError, WiringLayer},
};

#[derive(Debug)]
pub struct BlockReverterLayer {
    state_cache_path: String,
    merkle_tree_path: String,
}

impl BlockReverterLayer {
    pub fn new(state_cache_path: String, merkle_tree_path: String) -> Self {
        Self {
            state_cache_path,
            merkle_tree_path,
        }
    }
}

#[async_trait::async_trait]
impl WiringLayer for BlockReverterLayer {
    fn layer_name(&self) -> &'static str {
        "block_reverter_layer"
    }

    async fn wire(self: Box<Self>, mut context: ServiceContext<'_>) -> Result<(), WiringError> {
        let pool_resource = context.get_resource::<MasterPoolResource>().await?;
        let pool = pool_resource.get().await?;

        let reverter = BlockReverter::new(
            NodeRole::External,
            self.state_cache_path,
            self.merkle_tree_path,
            None,
            pool,
            L1ExecutedBatchesRevert::Allowed,
        );

        context.insert_resource(BlockReverterResource(Arc::new(reverter)))?;

        Ok(())
    }
}

use std::sync::Arc;

use zksync_core::block_reverter::BlockReverter;

use crate::resource::Resource;

/// Wrapper for the  block reverter.
#[derive(Debug, Clone)]
pub struct BlockReverterResource(pub Arc<BlockReverter>);

impl Resource for BlockReverterResource {
    fn resource_id() -> crate::resource::ResourceId {
        "common/block_reverter".into()
    }
}

use codec::Codec;
pub use pallet_posts_runtime_api::PostsApi as PostsRuntimeApi;

use jsonrpc_core::{Error, ErrorCode, Result as RpcResult};

use jsonrpc_derive::rpc;
use pallet_posts::rpc::FlatPost;
use pallet_support::PostId;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
#[rpc(client, server)]
pub trait PostsApi<BlockHash, AccountId, BlockNumber> {
    #[rpc(name = "posts_nextPostId")]
    fn get_next_post_id(&self, at: Option<BlockHash>) -> RpcResult<PostId>;

    #[rpc(name = "posts_getFeed")]

    fn get_feed(
        &self,
        at: Option<BlockHash>,
        account: AccountId,
        offset: u64,
        limit: u16,
    ) -> RpcResult<Vec<FlatPost<AccountId, BlockNumber>>>;
}

pub struct Posts<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> Posts<C, Block> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, BlockNumber> PostsApi<<Block as BlockT>::Hash, AccountId, BlockNumber>
    for Posts<C, Block>
where
    Block: BlockT,
    AccountId: Codec,
    BlockNumber: Codec,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PostsRuntimeApi<Block, AccountId, BlockNumber>,
{
    fn get_next_post_id(&self, _at: Option<<Block as BlockT>::Hash>) -> RpcResult<u128> {
        Ok(1)
    }
    fn get_feed(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        account: AccountId,
        offset: u64,
        limit: u16,
    ) -> RpcResult<Vec<FlatPost<AccountId, BlockNumber>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_feed(&at, account, offset, limit);
        //runtime_api_result
        runtime_api_result.map_err(map_rpc_error)
    }
}

//const RUNTIME_ERROR: i64 = 1;
// Converts a runtime trap into an RPC error.
pub fn map_rpc_error(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(1),
        message: "An RPC error occurred".into(),
        data: Some(format!("{:?}", err).into()),
    }
}

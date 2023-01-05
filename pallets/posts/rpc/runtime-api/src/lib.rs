#![cfg_attr(not(feature = "std"), no_std)]
use codec::Codec;
use pallet_posts::rpc::FlatPost;
use pallet_support::PostId;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait PostsApi<AccountId, BlockNumber> where
        AccountId: Codec,
        BlockNumber: Codec
    {
        fn get_next_post_id() -> PostId;
        fn get_feed(account: AccountId, offset: u64, limit: u16) -> Vec<FlatPost<AccountId, BlockNumber>>;
    }
}

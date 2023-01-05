use super::{Config, PostId};
use derive_more::From;
use frame_support::pallet_prelude::DispatchResult;
use frame_support::sp_runtime::DispatchError;
use frame_support::{
    ensure,
    pallet_prelude::{Decode, Encode, TypeInfo},
    RuntimeDebug,
};
use pallet_support::WhoAndWhen;
#[cfg(feature = "std")]
use serde::Deserialize;
use sp_std::vec::Vec;
/// Data structure representing a post
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Post<T: Config> {
    /// Id of the post in a sequential order i.e
    /// the first post created off chain is numbered 1
    pub id: PostId,

    pub created: WhoAndWhen<T>,
    /// Ideally the current owner should be the one who created the post.
    /// However, ownership of the post could be transferred.
    pub owner: T::AccountId,
    /// Type of post
    pub post_type: PostType,
    /// Get content of post
    pub content: PostContent,
    /// Comments associated with this post/comment
    pub comments_id: Vec<PostId>,
    /// The number of times a given post has been liked.
    pub likes: u32,

    /// The number of times a given post has been disliked.
    pub dislikes: u32,
    // TODO: Add extra features of post such as set_hidden,
    // TODO: likes, dislikes, space it belongs to, etc
}

/// PostTypes provides specific information necessary for different kinds
/// of posts such as
/// Regular posts e.g images, videos, text, etc,
/// Comments eg comment under a post
#[derive(Clone, Copy, Decode, Default, Encode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum PostType {
    #[default]
    RegularPost,
    // `parent_id` refers to post/comment this comment replies to
    Comment {
        parent_id: PostId,
    },
}

/// The content of a post refers text, video, etc
#[derive(Encode, From, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub enum PostContent {
    Content(Vec<u8>),
}
// impl From<PostContent> for Vec<u8> {
//     fn from(content: PostContent) -> Vec<u8> {
//         match content {
//             PostContent::Content(vec_u8) => vec_u8,
//         }
//     }
// }

#[derive(Encode, Decode, RuntimeDebug)]
pub enum ContentError {
    /// Post content is empty.
    EmptyContent,
}

impl From<ContentError> for DispatchError {
    fn from(_: ContentError) -> DispatchError {
        Self::Other("EmptyContent")
        // since there's only one type of content error
    }
}
impl PostContent {
    pub fn ensure_content_is_valid(&self) -> DispatchResult {
        match self {
            PostContent::Content(c) => {
                ensure!(c.len() != 0, ContentError::EmptyContent);
                Ok(())
            }
        }
    }
}

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize, Serializer};
use sp_std::prelude::*;

use frame_support::pallet_prelude::*;

use crate::{Config, Pallet, PostId};

use crate::post_types::{Post, PostContent, PostType};

use pallet_support::FlatWhoAndWhen;

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct FlatPost<AccountId, BlockNumber> {
    pub id: PostId,

    pub who_and_when: FlatWhoAndWhen<AccountId, BlockNumber>,

    pub owner: AccountId,

    /// Type of post
    #[cfg_attr(feature = "std", serde(flatten))]
    pub post_type: FlatPostType,
    /// Get content of post
    #[cfg_attr(feature = "std", serde(flatten))]
    pub content: FlatContent,
    /// Comments associated with this post/comment
    pub comments_id: Vec<PostId>,
    /// The number of times a given post has been liked.
    pub likes: u32,

    /// The number of times a given post has been disliked.
    pub dislikes: u32,
}

#[derive(Eq, PartialEq, Encode, Decode, Default, Debug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
#[derive(Clone)]
pub struct FlatPostType {
    pub is_regular_post: Option<bool>,
    pub is_comment: Option<bool>,
}

impl From<PostType> for FlatPostType {
    fn from(from: PostType) -> Self {
        match from {
            PostType::RegularPost => Self {
                is_regular_post: Some(true),
                is_comment: Some(false),
            },
            PostType::Comment { parent_id: _ } => Self {
                is_regular_post: Some(false),
                is_comment: Some(true),
            },
        }
    }
}

#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Clone, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct FlatContent {
    pub content: PostContent,
}

impl From<PostContent> for FlatContent {
    fn from(content: PostContent) -> Self {
        Self {
            content: content.clone(),
        }
    }
}
#[cfg(feature = "std")]
impl Serialize for PostContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let content_vec: Vec<u8> = self.clone().into();

        // If Bytes slice is invalid, then empty string will be returned
        serializer.serialize_str(std::str::from_utf8(&content_vec).unwrap_or_default())
    }
}
impl<T: Config> From<Post<T>> for FlatPost<T::AccountId, T::BlockNumber> {
    fn from(from: Post<T>) -> Self {
        let Post {
            id,
            created,
            owner,
            content,
            dislikes,
            likes,
            comments_id,
            post_type,
            ..
        } = from;

        Self {
            id,
            who_and_when: created.into(),
            owner,
            post_type: post_type.into(),
            content: content.into(),
            dislikes,
            likes,
            comments_id,
        }
    }
}

impl<T: Config> Pallet<T> {
    fn get_posts_by_ids_with_filter<F: FnMut(&Post<T>) -> bool>(
        all_post_ids: Vec<PostId>,
        offset: u64,
        limit: u16,
        _filter: F,
    ) -> Vec<FlatPost<T::AccountId, T::BlockNumber>> {
        let (_, posts_ids) = all_post_ids.split_at(offset as usize);
        posts_ids
            .iter()
            .filter_map(|id| Self::require_post(*id).ok())
            .take(limit as usize)
            .map(Into::into)
            .collect()
    }

    pub fn get_next_post_id() -> PostId {
        Self::next_post_id()
    }

    pub fn get_feed(
        account: T::AccountId,
        offset: u64,
        limit: u16,
    ) -> Vec<FlatPost<T::AccountId, T::BlockNumber>> {
        let mut post_ids: Vec<PostId> = Pallet::<T>::post_ids_by_account(account);
        post_ids.sort_by(|a, b| b.cmp(a));
        Self::get_posts_by_ids_with_filter(post_ids, offset, limit, |post| !post.is_comment())
    }
}

use {
    crate::{
        post_types::{Post, PostContent, PostType},
        reaction_types::{ReactionData, ReactionKind},
        Config, Error, Pallet, PostById, PostReactionKindByAccount,
    },
    frame_support::{dispatch::DispatchResult, pallet_prelude::DispatchError},
};

use pallet_support::{PostId, WhoAndWhen};

impl<T: Config> Post<T> {
    pub fn new(
        id: PostId,
        creator: T::AccountId,
        post_type: PostType,
        content: PostContent,
    ) -> Post<T> {
        Post {
            id,
            created: WhoAndWhen::<T>::new(creator.clone()),
            owner: creator,
            post_type,
            content,
            comments_id: Default::default(),
            likes: 0,
            dislikes: 0,
        }
    }

    pub fn decrease_reaction(&mut self, kind: ReactionKind) {
        match kind {
            ReactionKind::Like => self.likes = self.likes.saturating_sub(1),
            ReactionKind::Dislike => self.dislikes = self.dislikes.saturating_sub(1),
        };
    }

    pub fn add_reaction(
        &mut self,
        creator: <T as frame_system::Config>::AccountId,
        post_id: PostId,
        kind: ReactionKind,
    ) {
        let data = kind.new_reaction_data::<T>();
        PostReactionKindByAccount::<T>::insert(post_id, creator, data);
        match kind {
            ReactionKind::Like => self.likes = self.likes.saturating_add(1),
            ReactionKind::Dislike => self.dislikes = self.dislikes.saturating_add(1),
        };
    }

    pub fn delete_reaction(
        &mut self,
        creator: <T as frame_system::Config>::AccountId,
        post_id: PostId,
    ) -> Result<ReactionKind, frame_support::dispatch::DispatchError> {
        let reaction = self::Pallet::<T>::take_reaction((post_id, creator))?;
        self.decrease_reaction(reaction.kind);
        Ok(reaction.kind)
    }

    pub fn total_likes(&self) -> u32 {
        self.likes
    }

    pub fn total_dislikes(&self) -> u32 {
        self.dislikes
    }
    pub fn get_root_post(&self) -> Result<Post<T>, DispatchError> {
        match self.post_type {
            PostType::RegularPost => Ok(self.clone()),
            PostType::Comment { parent_id } => Pallet::<T>::require_post(parent_id),
        }
    }

    pub fn is_comment(&self) -> bool {
        matches!(self.post_type, PostType::Comment { parent_id: _ })
    }
    pub fn is_root_post(&self) -> bool {
        !self.is_comment()
    }
}

impl<T: Config> Pallet<T> {
    pub(crate) fn create_comment(comment_id: PostId, post_type: PostType) -> DispatchResult {
        match post_type {
            PostType::Comment { parent_id } => {
                PostById::<T>::try_mutate::<_, _, Error<T>, _>(parent_id, |parent| {
                    parent
                        .as_mut()
                        .map(|parent| parent.comments_id.push(comment_id))
                        .ok_or(Error::<T>::PostNotFound)
                })
            },
            PostType::RegularPost => Err(Error::<T>::NotAComment),
        }
        .map_err(Into::into)
    }

    /// Get `Post` by id from the storage or return `PostNotFound` error.
    pub fn require_post(post_id: PostId) -> Result<Post<T>, DispatchError> {
        Self::post_by_id(post_id).ok_or(Error::<T>::PostNotFound).map_err(Into::into)
    }
    /// Extract `Reaction` by id from the storage or return `ReactionByAccountNotFound` error.
    pub fn take_reaction(key: (PostId, T::AccountId)) -> Result<ReactionData<T>, DispatchError> {
        PostReactionKindByAccount::<T>::take(key.0, key.1)
            .ok_or(Error::<T>::ReactionByAccountNotFound)
            .map_err(Into::into)
    }
}

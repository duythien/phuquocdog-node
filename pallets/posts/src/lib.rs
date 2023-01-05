#![cfg_attr(not(feature = "std"), no_std)]

pub mod functions;
pub mod post_types;
pub mod reaction_types;
pub mod rpc;
pub use pallet::*;
use pallet_support::PostId;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        ensure,
        pallet_prelude::{
            DispatchResult, IsType, StorageDoubleMap, StorageMap, StorageValue, Twox64Concat,
            ValueQuery,
        },
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::{ensure_signed, OriginFor};

    use super::PostId;
    use crate::{
        post_types::{Post, PostContent, PostType},
        reaction_types::{ReactionData, ReactionKind},
    };
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    /// Get the details of a post by its' id.
    #[pallet::storage]
    #[pallet::getter(fn post_by_id)]
    pub type PostById<T: Config> = StorageMap<_, Twox64Concat, PostId, Post<T>>;

    #[pallet::type_value]
    pub fn DefaultForNextPostId() -> PostId {
        1
        // First post id
    }

    /// The next post id.
    #[pallet::storage]
    #[pallet::getter(fn next_post_id)]
    pub type NextPostId<T: Config> = StorageValue<_, PostId, ValueQuery, DefaultForNextPostId>;

    #[pallet::storage]
    #[pallet::getter(fn post_reaction_id_by_account)]
    pub type PostReactionKindByAccount<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        PostId,
        Blake2_128Concat,
        T::AccountId,
        ReactionData<T>,
    >;

    /// Get the ids of all posts in a given by account
    #[pallet::storage]
    #[pallet::getter(fn post_ids_by_account)]
    pub type PostIdsByAccount<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<PostId>, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        PostCreated {
            who: T::AccountId,
            post_id: PostId,
        },
        /// Emit events when a post has been updated e.g removed, edited, etc
        PostUpdated {
            who: T::AccountId,
            post_id: PostId,
        },
        PostReactionCreated {
            account: T::AccountId,
            post_id: PostId,
            reaction_kind: ReactionKind,
        },
        PostReactionUpdated {
            account: T::AccountId,
            post_id: PostId,
            reaction_kind: ReactionKind,
        },
        PostReactionDeleted {
            account: T::AccountId,
            post_id: PostId,
            reaction_kind: ReactionKind,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Post was not found by id.
        PostNotFound,
        // Comment related errors:
        /// Unknown parent comment id.
        UnknownParentComment,
        /// This post's type is not a `Comment`
        NotAComment,
        // Reaction related errors
        /// New reaction kind is the same as old one on this post/comment.
        SameReaction,
        /// There is no reaction by account on this post/comment.
        ReactionByAccountNotFound,

        PostHasNoSpaceId, // TODO: Add more post errors
    }

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_post(
            origin: OriginFor<T>,
            post_type: PostType,
            content: PostContent,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            let new_post_id = Self::next_post_id();
            // make sure an empty post isn't added
            content.ensure_content_is_valid()?;

            let new_post: Post<T> = Post::new(new_post_id, creator.clone(), post_type, content);
            match post_type {
                PostType::RegularPost => {}
                PostType::Comment { .. } => Self::create_comment(new_post_id, post_type)?,
            }
            // TODO: Add more checks. For instance, check if the one commenting
            // has been blocked by post owner
            if new_post.is_root_post() {
                PostIdsByAccount::<T>::mutate(creator.clone(), |ids| ids.push(new_post_id));
            }
            PostById::<T>::insert(new_post_id, new_post);
            NextPostId::<T>::mutate(|n| {
                *n += 1;
            });
            Self::deposit_event(Event::PostCreated {
                who: creator,
                post_id: new_post_id,
            });
            Ok(())
        }

        /// Function to add or update a post reaction
        // With the current implementation, both comments and posts can be liked/disliked
        #[pallet::weight(0)]
        pub fn add_post_reaction(
            origin: OriginFor<T>,
            post_id: PostId,
            kind: ReactionKind,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            let mut post = Self::require_post(post_id)?;

            let maybe_present = PostReactionKindByAccount::<T>::try_get(post_id, &creator);
            if let Ok(val) = &maybe_present {
                let val = val.kind;
                ensure!(val != kind, Error::<T>::SameReaction);
                // reduce count of old reaction
                post.decrease_reaction(val);
            }
            post.add_reaction(creator.clone(), post_id, kind);

            PostById::<T>::insert(post_id, post);
            Self::deposit_event(maybe_present.map_or(
                Event::PostReactionCreated {
                    account: creator.clone(),
                    post_id,
                    reaction_kind: kind,
                },
                |_| Event::PostReactionUpdated {
                    account: creator,
                    post_id,
                    reaction_kind: kind,
                },
            ));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn delete_post_reaction(origin: OriginFor<T>, post_id: PostId) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            let mut post = Self::require_post(post_id)?;
            let kind = post.delete_reaction(creator.clone(), post_id)?;
            PostById::<T>::insert(post_id, post);
            Self::deposit_event(Event::PostReactionDeleted {
                account: creator,
                post_id,
                reaction_kind: kind,
            });
            Ok(())
        }
    }
}

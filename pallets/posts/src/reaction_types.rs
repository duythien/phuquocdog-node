use frame_support::{
    pallet_prelude::{Decode, Encode, TypeInfo},
    RuntimeDebug,
};

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Reaction<T> {
    pub time_created: T,
    pub kind: ReactionKind,
}

pub type ReactionData<T> = Reaction<<T as pallet_timestamp::Config>::Moment>;

pub fn new_reaction_data<T>(kind: ReactionKind) -> Reaction<T::Moment>
where
    T: frame_system::Config + pallet_timestamp::Config,
{
    Reaction { time_created: pallet_timestamp::Pallet::<T>::now(), kind }
}

#[derive(
    Encode,
    Decode,
    Clone,
    Copy,
    Eq,
    PartialEq,
    RuntimeDebug,
    TypeInfo,
    Default,
    strum::IntoStaticStr,
)]
pub enum ReactionKind {
    #[default]
    Like,
    Dislike,
}

impl ReactionKind {
    pub fn new_reaction_data<T>(self) -> Reaction<T::Moment>
    where
        T: frame_system::Config + pallet_timestamp::Config,
    {
        Reaction { time_created: pallet_timestamp::Pallet::<T>::now(), kind: self }
    }
}

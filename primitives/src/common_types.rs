use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
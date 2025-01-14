pub mod batched_range_proof;
pub mod ciphertext_ciphertext_equality;
pub mod ciphertext_commitment_equality;
pub mod pubkey_validity;
pub mod range_proof;
pub mod transfer;
pub mod transfer_with_fee;
pub mod withdraw;
pub mod zero_balance;

use num_derive::{FromPrimitive, ToPrimitive};
#[cfg(not(target_os = "solana"))]
use {
    crate::{
        encryption::{
            elgamal::ElGamalCiphertext,
            pedersen::{PedersenCommitment, PedersenOpening},
        },
        errors::ProofError,
    },
    curve25519_dalek::scalar::Scalar,
};
pub use {
    batched_range_proof::{
        batched_range_proof_u128::BatchedRangeProofU128Data,
        batched_range_proof_u256::BatchedRangeProofU256Data,
        batched_range_proof_u64::BatchedRangeProofU64Data, BatchedRangeProofContext,
    },
    bytemuck::Pod,
    ciphertext_ciphertext_equality::{
        CiphertextCiphertextEqualityProofContext, CiphertextCiphertextEqualityProofData,
    },
    ciphertext_commitment_equality::{
        CiphertextCommitmentEqualityProofContext, CiphertextCommitmentEqualityProofData,
    },
    pubkey_validity::{PubkeyValidityData, PubkeyValidityProofContext},
    range_proof::{RangeProofContext, RangeProofU64Data},
    transfer::{TransferData, TransferProofContext},
    transfer_with_fee::{FeeParameters, TransferWithFeeData, TransferWithFeeProofContext},
    withdraw::{WithdrawData, WithdrawProofContext},
    zero_balance::{ZeroBalanceProofContext, ZeroBalanceProofData},
};

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum ProofType {
    /// Empty proof type used to distinguish if a proof context account is initialized
    Uninitialized,
    ZeroBalance,
    Withdraw,
    CiphertextCiphertextEquality,
    Transfer,
    TransferWithFee,
    PubkeyValidity,
    RangeProofU64,
    BatchedRangeProofU64,
    BatchedRangeProofU128,
    BatchedRangeProofU256,
    CiphertextCommitmentEquality,
}

pub trait ZkProofData<T: Pod> {
    const PROOF_TYPE: ProofType;

    fn context_data(&self) -> &T;

    #[cfg(not(target_os = "solana"))]
    fn verify_proof(&self) -> Result<(), ProofError>;
}

#[cfg(not(target_os = "solana"))]
#[derive(Debug, Copy, Clone)]
pub enum Role {
    Source,
    Destination,
    Auditor,
    WithdrawWithheldAuthority,
}

/// Takes in a 64-bit number `amount` and a bit length `bit_length`. It returns:
///  - the `bit_length` low bits of `amount` interpretted as u64
///  - the (64 - `bit_length`) high bits of `amount` interpretted as u64
#[cfg(not(target_os = "solana"))]
pub fn split_u64(amount: u64, bit_length: usize) -> (u64, u64) {
    if bit_length == 64 {
        (amount, 0)
    } else {
        let lo = amount << (64 - bit_length) >> (64 - bit_length);
        let hi = amount >> bit_length;
        (lo, hi)
    }
}

#[cfg(not(target_os = "solana"))]
pub fn combine_lo_hi_u64(amount_lo: u64, amount_hi: u64, bit_length: usize) -> u64 {
    if bit_length == 64 {
        amount_lo
    } else {
        amount_lo + (amount_hi << bit_length)
    }
}

#[cfg(not(target_os = "solana"))]
fn combine_lo_hi_ciphertexts(
    ciphertext_lo: &ElGamalCiphertext,
    ciphertext_hi: &ElGamalCiphertext,
    bit_length: usize,
) -> ElGamalCiphertext {
    let two_power = (1_u64) << bit_length;
    ciphertext_lo + &(ciphertext_hi * &Scalar::from(two_power))
}

#[cfg(not(target_os = "solana"))]
pub fn combine_lo_hi_commitments(
    comm_lo: &PedersenCommitment,
    comm_hi: &PedersenCommitment,
    bit_length: usize,
) -> PedersenCommitment {
    let two_power = (1_u64) << bit_length;
    comm_lo + comm_hi * &Scalar::from(two_power)
}

#[cfg(not(target_os = "solana"))]
pub fn combine_lo_hi_openings(
    opening_lo: &PedersenOpening,
    opening_hi: &PedersenOpening,
    bit_length: usize,
) -> PedersenOpening {
    let two_power = (1_u64) << bit_length;
    opening_lo + opening_hi * &Scalar::from(two_power)
}

#![deny(missing_docs)]

//! A program for creating and managing pools of stake

pub mod big_vec;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
use {
    crate::state::Fee,
    solana_program::{pubkey::Pubkey, stake::state::Meta},
};

/// Seed for deposit authority seed
const AUTHORITY_DEPOSIT: &[u8] = b"deposit";

/// Seed for withdraw authority seed
const AUTHORITY_WITHDRAW: &[u8] = b"withdraw";

/// Seed for transient stake account
const TRANSIENT_STAKE_SEED_PREFIX: &[u8] = b"transient";

/// Minimum amount of staked SOL required in a validator stake account to allow
/// for merges without a mismatch on credits observed
pub const MINIMUM_ACTIVE_STAKE: u64 = 1_000_000;

/// Minimum amount of SOL in the reserve
/// NOTE: This can be changed to 0 once the `stake_allow_zero_undelegated_amount`
/// feature is enabled on all clusters
pub const MINIMUM_RESERVE_LAMPORTS: u64 = 1;

/// Maximum amount of validator stake accounts to update per
/// `UpdateValidatorListBalance` instruction, based on compute limits
pub const MAX_VALIDATORS_TO_UPDATE: usize = 5;

/// Maximum factor by which a withdrawal fee can be increased per epoch
/// protecting stakers from malicious users.
/// If current fee is 0, WITHDRAWAL_BASELINE_FEE is used as the baseline
pub const MAX_WITHDRAWAL_FEE_INCREASE: Fee = Fee {
    numerator: 3,
    denominator: 2,
};
/// Drop-in baseline fee when evaluating withdrawal fee increases when fee is 0
pub const WITHDRAWAL_BASELINE_FEE: Fee = Fee {
    numerator: 1,
    denominator: 1000,
};

/// The maximum number of transient stake accounts respecting
/// transaction account limits.
pub const MAX_TRANSIENT_STAKE_ACCOUNTS: usize = 10;

/// Get the stake amount under consideration when calculating pool token
/// conversions
#[inline]
pub fn minimum_stake_lamports(meta: &Meta, stake_program_minimum_delegation: u64) -> u64 {
    meta.rent_exempt_reserve
        .saturating_add(minimum_delegation(stake_program_minimum_delegation))
}

/// Get the minimum delegation required by a stake account in a stake pool
#[inline]
pub fn minimum_delegation(stake_program_minimum_delegation: u64) -> u64 {
    std::cmp::max(stake_program_minimum_delegation, MINIMUM_ACTIVE_STAKE)
}

/// Get the stake amount under consideration when calculating pool token
/// conversions
#[inline]
pub fn minimum_reserve_lamports(meta: &Meta) -> u64 {
    meta.rent_exempt_reserve
        .saturating_add(MINIMUM_RESERVE_LAMPORTS)
}

/// Generates the deposit authority program address for the stake pool
pub fn find_deposit_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&stake_pool_address.to_bytes()[..32], AUTHORITY_DEPOSIT],
        program_id,
    )
}

/// Generates the withdraw authority program address for the stake pool
pub fn find_withdraw_authority_program_address(
    program_id: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[&stake_pool_address.to_bytes(), AUTHORITY_WITHDRAW],
        program_id,
    )
}

/// Generates the stake program address for a validator's vote account
pub fn find_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &vote_account_address.to_bytes(),
            &stake_pool_address.to_bytes(),
        ],
        program_id,
    )
}

/// Generates the stake program address for a validator's vote account
pub fn find_transient_stake_program_address(
    program_id: &Pubkey,
    vote_account_address: &Pubkey,
    stake_pool_address: &Pubkey,
    seed: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            TRANSIENT_STAKE_SEED_PREFIX,
            &vote_account_address.to_bytes(),
            &stake_pool_address.to_bytes(),
            &seed.to_le_bytes(),
        ],
        program_id,
    )
}

solana_program::declare_id!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy");

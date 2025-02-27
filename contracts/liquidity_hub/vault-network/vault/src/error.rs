use cosmwasm_std::{OverflowError, Uint128};
use semver::Version;
use thiserror::Error;

pub type StdResult<T> = Result<T, VaultError>;

#[derive(Error, Debug, PartialEq)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Deposits are not enabled")]
    DepositsDisabled {},

    #[error("Flash-loans are not enabled")]
    FlashLoansDisabled {},

    #[error("mismatch of sent {sent} but specified deposit amount of {wanted}")]
    FundsMismatch { sent: Uint128, wanted: Uint128 },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Attempt to call callback function outside contract")]
    ExternalCallback {},

    #[error(
        "Final desired amount of {required_amount} is less than initial balance of {old_balance}"
    )]
    NegativeProfit {
        old_balance: Uint128,
        required_amount: Uint128,
    },

    #[error("Attempt to migrate to version {new_version}, but contract is on a higher version {current_version}")]
    MigrateInvalidVersion {
        new_version: Version,
        current_version: Version,
    },

    #[error("Withdrawals are not enabled")]
    WithdrawsDisabled {},

    #[error("Cannot deposit while flash-loaning")]
    DepositDuringLoan {},
}

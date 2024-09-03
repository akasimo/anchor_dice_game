use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Bet not expired")]
    BetNotExpired,

    #[msg("Signature verification failed")]
    SignatureVerificationFailed,

    #[msg("Ed25519 error")]
    Ed25519,

    #[msg("Ed25519 accounts error")]
    Ed25519Accounts,

    #[msg("signature is not verifiable")]
    Ed25519Signature
}
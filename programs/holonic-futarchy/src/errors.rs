use anchor_lang::error_code;

#[error_code]
pub enum HolonicFutarchyErrors {
    /// Incorrect Owner
    #[msg("The key passed in doesn't match the stored owner data on-chain.")]
    IncorrectOwner,
}

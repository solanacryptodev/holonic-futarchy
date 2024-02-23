use anchor_lang::error_code;

#[error_code]
pub enum HolonicFutarchyErrors {
    /// Incorrect Holarchy
    #[msg("This holarchy doesn't match.")]
    IncorrectHolarchy,
}

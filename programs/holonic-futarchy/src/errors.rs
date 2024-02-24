use anchor_lang::error_code;

#[error_code]
pub enum HolonicFutarchyErrors {
    /// Incorrect Holarchy
    #[msg("This holarchy doesn't match.")]
    IncorrectHolarchy,
    /// Not a Multisig
    #[msg("You need a multisig.")]
    NotMultisig,
    /// Not a futarchy
    #[msg("You need a futarchy.")]
    NotFutarchy,
    /// Incorrect Instruction: Use initializeCreateFutarchyHolon
    #[msg("Incorrect Instruction: Use initializeCreateFutarchyHolon")]
    IncorrectInstruction
}

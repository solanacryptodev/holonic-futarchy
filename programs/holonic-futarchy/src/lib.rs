use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

pub mod instructions;
pub mod state;
mod errors;
mod constants;

pub use instructions::*;
pub use crate::state::state::*;

security_txt! {
    name: "Holonic Futarchy Program",
    project_url: "",
    contacts: "email:solanacryptodev@protonmail.com",
    policy: "",
    source_code: "https://github.com/solanacryptodev/holonic-futarchy/",
    source_release: "v0",
    auditors: "None",
    acknowledgements: ""
}

declare_id!("C1Hdqy8NDbNWHTM8Mhy67pTzTsZbGtwKGpz33vrzReuK");

#[program]
pub mod holonic_futarchy {
    use super::*;

    /// Creates a multisig
    pub fn create_multisig(ctx: Context<CreateHolarchyMultisig>, owners: Vec<Pubkey>, threshold: u64, nonce: u8) -> Result<()> {
        handle_create_holarchy_multisig(ctx, owners, threshold, nonce)?;
        Ok(())
    }

    /// Initializes the root holon
    pub fn initialize_root_holon(ctx: Context<HolarchyRoot>, holarchy_metadata: HolarchyMetadata) -> Result<()> {
        handle_initialize_root_holon(ctx, holarchy_metadata)?;
        Ok(())
    }

    /// Creates a new holon
    pub fn create_futarchy_holon(ctx: Context<CreateFutarchyHolon>, holon_metadata: HolonMetadata) -> Result<()> {
        handle_create_futarchy_holon(ctx, holon_metadata)?;
        Ok(())
    }

    pub fn create__holon(ctx: Context<CreateHolon>, holon_metadata: HolonMetadata) -> Result<()> {
        handle_create_holon(ctx, holon_metadata)?;
        Ok(())
    }
}

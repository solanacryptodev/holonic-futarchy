use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

pub mod instructions;
pub mod state;
mod errors;

pub use instructions::*;
pub use crate::state::state::*;

security_txt! {
    name: "Holonic Futarchy Program",
    project_url: "",
    contacts: "email:---@protonmail.com",
    policy: "",
    source_code: "https://github.com/",
    source_release: "v0",
    auditors: "None",
    acknowledgements: ""
}

declare_id!("C1Hdqy8NDbNWHTM8Mhy67pTzTsZbGtwKGpz33vrzReuK");

#[program]
pub mod holonic_futarchy {
    use super::*;

    /// Initializes the root holon
    pub fn initialize_root_holon(ctx: Context<HolarchyRoot>, holarchy_metadata: HolarchyMetadata) -> Result<()> {
        handle_initialize_root_holon(ctx, holarchy_metadata)?;
        Ok(())
    }

    pub fn initialize_holon(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize {}

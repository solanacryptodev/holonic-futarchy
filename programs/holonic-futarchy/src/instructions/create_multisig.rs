use anchor_lang::prelude::*;
use coral_multisig::program::CoralMultisig;
use coral_multisig::cpi::accounts::CreateMultisig;
use coral_multisig::cpi::create_multisig;
use coral_multisig::{Multisig};
use crate::constants::*;

#[derive(Accounts)]
pub struct CreateHolarchyMultisig<'info> {
    #[account(
    init,
    payer = owner,
    space = 8 + (10 + 32) + 8 + 4 + 1,
    seeds = [HOLARCHY_MULTISIG.as_bytes(), holarchy.key().as_ref()],
    bump
    )]
    pub holarchy: Account<'info, Multisig>,
    #[account(mut)]
    pub coral_multisig_program: Program<'info, CoralMultisig>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_holarchy_multisig(ctx: Context<CreateHolarchyMultisig>, owners: Vec<Pubkey>, threshold: u64, nonce: u8) -> Result<()> {
    let holarchy = &mut ctx.accounts.holarchy;
    let coral_multisig = &mut ctx.accounts.coral_multisig_program;

    let cpi_multisig_program = coral_multisig.to_account_info();
    let cpi_accounts = CreateMultisig {
        multisig: holarchy.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_multisig_program, cpi_accounts);

    create_multisig(cpi_ctx, owners, threshold, nonce)?;

    Ok(())
}

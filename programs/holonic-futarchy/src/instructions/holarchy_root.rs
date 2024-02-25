use anchor_lang::prelude::*;
use crate::Holarchy;
use crate::state::state::HolarchyMetadata;
use coral_multisig::Multisig;

#[derive(Accounts)]
pub struct HolarchyRoot<'info> {
    #[account(
    init,
    payer = owner,
    space = 8 + Holarchy::MAX_SIZE,
    seeds = [Holarchy::SEED.as_bytes()],
    bump
    )]
    pub holarchy: Account<'info, Holarchy>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_root_holon(ctx: Context<HolarchyRoot>, holarchy_metadata: HolarchyMetadata ) -> Result<()> {
    let holarchy = &mut ctx.accounts.holarchy;
    let multisig = &mut ctx.accounts.multisig;

    holarchy.metadata.key = ctx.accounts.multisig.key();
    holarchy.new(multisig.owners.clone(), holarchy_metadata);
    holarchy.assert_multisig(multisig.owners.clone())?;

    Ok(())
}

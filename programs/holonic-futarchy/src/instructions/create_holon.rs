use anchor_lang::prelude::*;
use crate::{Holarchy, Holon, HolonMetadata};

#[derive(Accounts)]
pub struct CreateHolon<'info> {
    #[account(
    init,
    payer = owner,
    space = 8 + Holon::MAX_SIZE,
    seeds = [Holon::SEED.as_bytes()],
    bump
    )]
    pub holon: Account<'info, Holon>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_holon(ctx: Context<CreateHolon>, holon_metadata: HolonMetadata) -> Result<()> {
    let holon = &mut ctx.accounts.holon;
}

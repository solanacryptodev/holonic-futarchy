use anchor_lang::prelude::*;
use crate::{Holarchy, Holon, HolonMetadata};
use crate::errors::HolonicFutarchyErrors;

#[derive(Accounts)]
#[instruction(data: HolonMetadata)]
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
    pub holarchy: Account<'info, Holarchy>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_holon(ctx: Context<CreateHolon>, holon_metadata: HolonMetadata) -> Result<()> {
    let holon = &mut ctx.accounts.holon;
    let holarchy = &mut ctx.accounts.holarchy;
    let holarchy_key = holarchy.key();

    require!(holon_metadata.futarchy == false, HolonicFutarchyErrors::IncorrectInstruction);

    holon.new(holon_metadata);
    holon.assert_from_holarchy(&holarchy_key)?;

    Ok(())
}

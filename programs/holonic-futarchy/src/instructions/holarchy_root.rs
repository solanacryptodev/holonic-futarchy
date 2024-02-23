use anchor_lang::prelude::*;
use crate::DAOHolon;
use crate::state::state::HolarchyMetadata;

#[derive(Accounts)]
pub struct RootHolon<'info> {
    #[account(
    init,
    payer = owner,
    space = 8 + DAOHolon::MAX_SIZE,
    seeds = [GameData::SEED.as_bytes(), owner.key().as_ref()],
    bump
    )]
    pub dao_holon: Account<'info, DAOHolon>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_root_holon(ctx: Context<RootHolon>, holarchy_metadata: HolarchyMetadata ) -> Result<()> {

}

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::{Holarchy, Holon, HolonMetadata};
use autocrat_v0::cpi::initialize_dao;
use autocrat_v0::program::AutocratV0;
use autocrat_v0::cpi::accounts::InitializeDAO;
use crate::errors::HolonicFutarchyErrors;

#[derive(Accounts)]
#[instruction(data: HolonMetadata)]
pub struct CreateFutarchyHolon<'info> {
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
    pub autocrat_program: Program<'info, AutocratV0>,
    #[account(mut, address = data.meta_mint)]
    pub meta_mint: Account<'info, Mint>,
    #[account(mut, address = data.usdc_mint)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_futarchy_holon(ctx: Context<CreateFutarchyHolon>, holon_metadata: HolonMetadata) -> Result<()> {
    let holon = &mut ctx.accounts.holon;
    let holarchy = &mut ctx.accounts.holarchy;
    let autocrat_program = &mut ctx.accounts.autocrat_program;
    let system_program = &mut ctx.accounts.system_program;
    let meta_mint = &mut ctx.accounts.meta_mint;
    let usdc_mint = &mut ctx.accounts.usdc_mint;
    let owner = &mut ctx.accounts.owner;
    let holarchy_key = holarchy.key();

    require!(holon_metadata.futarchy == true, HolonicFutarchyErrors::NotFutarchy);

    holon.new(holon_metadata)?;
    holon.assert_from_holarchy(&holarchy_key)?;

    let cpi_autocrat_program = autocrat_program.to_account_info();
    let cpi_accounts = InitializeDAO {
        dao: holon.to_account_info(),
        payer: owner.to_account_info(),
        system_program: system_program.to_account_info(),
        meta_mint: meta_mint.to_account_info(),
        usdc_mint: usdc_mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_autocrat_program, cpi_accounts);
    initialize_dao(cpi_ctx)?;

    Ok(())
}

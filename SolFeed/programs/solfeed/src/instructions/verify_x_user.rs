use anchor_lang::prelude::*;
use crate::state::{verification::Verification, global_settings::GlobalSettings};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct VerifyXUser<'info> {
    #[account(
        init,
        seeds = [VERIFICATION_SEED, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 8 // Verification struct: user + timestamp
    )]
    pub verification: Account<'info, Verification>,

    #[account(
        seeds = [GLOBAL_SETTINGS_SEED],
        bump,
    )]
    pub global_settings: Account<'info, GlobalSettings>,

    #[account(mut)]
    pub authority: Signer<'info>, 

    #[account(mut)]
    pub treasury_wallet: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<VerifyXUser>) -> Result<()> {
    let fee = ctx.accounts.global_settings.verification_fee;

    invoke(
        &system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &ctx.accounts.treasury_wallet.key(),
            fee,
        ),
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.treasury_wallet.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let verification = &mut ctx.accounts.verification;
    verification.user = ctx.accounts.authority.key();
    verification.timestamp = ctx.accounts.clock.unix_timestamp;

    Ok(())
}


//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

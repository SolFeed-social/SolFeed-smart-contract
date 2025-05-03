use anchor_lang::prelude::*;
use crate::state::{like::Like, global_settings::GlobalSettings};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct LikePost<'info> {
    #[account(
        init,
        seeds = [b"like", authority.key().as_ref(), post.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 32 + 8
    )]
    pub like: Account<'info, Like>,

    pub post: UncheckedAccount<'info>,

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

pub fn handler(ctx: Context<LikePost>) -> Result<()> {
    let fee = ctx.accounts.global_settings.action_fee;

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

    let like = &mut ctx.accounts.like;
    like.liker = ctx.accounts.authority.key();
    like.post = ctx.accounts.post.key();
    like.timestamp = ctx.accounts.clock.unix_timestamp;

    Ok(())
}




//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

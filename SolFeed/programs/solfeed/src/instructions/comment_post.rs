use anchor_lang::prelude::*;
use crate::state::{comment::Comment, global_settings::GlobalSettings};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
#[instruction(comment: String)]
pub struct CommentPost<'info> {
    #[account(
        init,
        seeds = [b"comment", authority.key().as_ref(), post.key().as_ref(), clock.unix_timestamp.to_le_bytes().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 32 + comment.len() + 8
    )]
    pub comment_account: Account<'info, Comment>,

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

pub fn handler(ctx: Context<CommentPost>, comment: String) -> Result<()> {
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

    let comment_account = &mut ctx.accounts.comment_account;
    comment_account.commenter = ctx.accounts.authority.key();
    comment_account.post = ctx.accounts.post.key();
    comment_account.text = comment;
    comment_account.timestamp = ctx.accounts.clock.unix_timestamp;

    Ok(())
}


//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

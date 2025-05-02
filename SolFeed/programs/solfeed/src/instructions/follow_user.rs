use anchor_lang::prelude::*;
use crate::state::{follow::Follow, global_settings::GlobalSettings};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct FollowUser<'info> {
    #[account(
        init,
        seeds = [b"follow", follower.key().as_ref(), followed.key().as_ref()],
        bump,
        payer = follower,
        space = 8 + 32 + 32 + 8
    )]
    pub follow: Account<'info, Follow>,

    #[account(
        seeds = [GLOBAL_SETTINGS_SEED],
        bump,
    )]
    pub global_settings: Account<'info, GlobalSettings>,

    #[account(mut)]
    pub follower: Signer<'info>,

    pub followed: UncheckedAccount<'info>,

    #[account(mut)]
    pub treasury_wallet: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<FollowUser>) -> Result<()> {
    let fee = ctx.accounts.global_settings.action_fee;

    invoke(
        &system_instruction::transfer(
            &ctx.accounts.follower.key(),
            &ctx.accounts.treasury_wallet.key(),
            fee,
        ),
        &[
            ctx.accounts.follower.to_account_info(),
            ctx.accounts.treasury_wallet.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let follow = &mut ctx.accounts.follow;
    follow.follower = ctx.accounts.follower.key();
    follow.followed = ctx.accounts.followed.key();
    follow.timestamp = ctx.accounts.clock.unix_timestamp;

    Ok(())
}



// ╔══════════════════════════════╗  Author: @TheItalianPimp 
// ║         ☠ THE VOID ☠         ║  Name: Luca C.
// ╠══════════════════════════════╣  Date: 04/02/2025
// ║                              ║  Project: SolFeed
// ║          █████████           ║
// ║       ███░░░░░░░░░███        ║
// ║     ██░░░░░░░░░░░░░░░██      ║
// ║    ██░░░░░░░░░░░░░░░░░░██    ║
// ║   ██░░░░░░░░░░░░░░░░░░░░██   ║
// ║   ██░░░░░░░░░░░░░░░░░░░░██   ║
// ║   ██░░░░░░░░░░░░░░░░░░░░██   ║
// ║    ██░░░░░░░░░░░░░░░░░░██    ║
// ║     ██░░░░░░░░░░░░░░░██      ║
// ║       ███░░░░░░░░░███        ║
// ║          █████████           ║
// ║                              ║
// ╚══════════════════════════════╝
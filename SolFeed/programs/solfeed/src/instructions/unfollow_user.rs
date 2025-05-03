use anchor_lang::prelude::*;
use crate::state::follow::Follow;

#[derive(Accounts)]
pub struct UnfollowUser<'info> {
    #[account(
        mut,
        seeds = [b"follow", follower.key().as_ref(), followed.key().as_ref()],
        bump,
        close = receiver,
    )]
    pub follow: Account<'info, Follow>,

    #[account(mut)]
    pub follower: Signer<'info>,

    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub treasury_wallet: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<UnfollowUser>) -> Result<()> {
    let lamports = ctx.accounts.follow.to_account_info().lamports();
    let half = lamports / 2;

    **ctx.accounts.receiver.to_account_info().try_borrow_mut_lamports()? += half;
    **ctx.accounts.treasury_wallet.to_account_info().try_borrow_mut_lamports()? += lamports - half;
    **ctx.accounts.follow.to_account_info().try_borrow_mut_lamports()? = 0;

    Ok(())
}




//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

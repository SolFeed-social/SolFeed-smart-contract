use anchor_lang::prelude::*;
use crate::state::{profile::Profile, global_settings::GlobalSettings};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(
        mut,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub profile: Account<'info, Profile>,

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
}

pub fn handler(
    ctx: Context<UpdateProfile>,
    bio: String,
    image: Vec<u8>,
) -> Result<()> {
    let fee = ctx.accounts.global_settings.profile_update_fee;

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

    let profile = &mut ctx.accounts.profile;
    profile.bio = bio;
    profile.image = image;

    Ok(())
}



//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

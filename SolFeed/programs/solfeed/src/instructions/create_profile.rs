use anchor_lang::prelude::*;
use crate::state::{global_settings::GlobalSettings, profile::Profile};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
#[instruction(username: String)]
pub struct CreateProfile<'info> {
    #[account(
        init,
        seeds = [b"profile", authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + MAX_USERNAME_LENGTH + MAX_BIO_LENGTH + MAX_IMAGE_SIZE,
    )]
    pub profile: Account<'info, Profile>,

    #[account(
        init,
        seeds = [USERNAME_SEED, username.as_bytes()],
        bump,
        payer = authority,
        space = 8 
    )]
    pub username_marker: AccountInfo<'info>, 

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
    ctx: Context<CreateProfile>,
    username: String,
    bio: String,
    image: Vec<u8>,
) -> Result<()> {
    let fee = ctx.accounts.global_settings.profile_fee;

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

    // Write profile data
    let profile = &mut ctx.accounts.profile;
    profile.authority = ctx.accounts.authority.key();
    profile.username = username;
    profile.bio = bio;
    profile.image = image;

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
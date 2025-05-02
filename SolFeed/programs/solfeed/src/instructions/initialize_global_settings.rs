use anchor_lang::prelude::*;
use crate::state::global_settings::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeGlobalSettings<'info> {
    #[account(
        init,
        seeds = [GLOBAL_SETTINGS_SEED],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<GlobalSettings>(),
    )]
    pub global_settings: Account<'info, GlobalSettings>,

    #[account(mut)]
    pub authority: Signer<'info>, 

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeGlobalSettings>, treasury_wallet: Pubkey) -> Result<()> {
    let settings = &mut ctx.accounts.global_settings;

    settings.authority = ctx.accounts.authority.key();
    settings.treasury_wallet = treasury_wallet;

    settings.profile_fee = 2_500_000;         // 0.0025 SOL
    settings.profile_update_fee = 2_000_000;  // 0.0020 SOL
    settings.text_post_fee = 250_000;         // 0.00025 SOL
    settings.image_post_fee = 300_000;        // 0.00030 SOL
    settings.action_fee = 250_000;            // 0.00025 SOL
    settings.verification_fee = 100_000_000;  // 0.1 SOL

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
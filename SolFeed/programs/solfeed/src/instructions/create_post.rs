use anchor_lang::prelude::*;
use crate::state::{post::Post, global_settings::GlobalSettings};
use crate::constants::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [b"post", authority.key().as_ref(), &clock.unix_timestamp.to_le_bytes()],
        bump,
        payer = authority,
        space = 8 + 32 + 280 + MAX_IMAGE_SIZE + 8
    )]
    pub post: Account<'info, Post>,

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

pub fn handler(
    ctx: Context<CreatePost>,
    text: String,
    image: Option<Vec<u8>>,
) -> Result<()> {
    let settings = &ctx.accounts.global_settings;
    let fee = if image.is_some() {
        settings.image_post_fee
    } else {
        settings.text_post_fee
    };

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

    let post = &mut ctx.accounts.post;
    post.author = ctx.accounts.authority.key();
    post.text = text;
    post.image = image;
    post.timestamp = ctx.accounts.clock.unix_timestamp;

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
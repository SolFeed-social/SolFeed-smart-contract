use anchor_lang::prelude::*;

pub mod constants;
pub mod state;
pub mod instructions;
pub mod errors;
pub mod utils;

use instructions::*;

declare_id!("SOLFEED_MAINNET_PROGRAM_ID");

#[program]
pub mod solfeed {
    use super::*;

    pub fn initialize_global_settings(
        ctx: Context<InitializeGlobalSettings>,
        treasury_wallet: Pubkey,
    ) -> Result<()> {
        initialize_global_settings::handler(ctx, treasury_wallet)
    }

    pub fn create_profile(
        ctx: Context<CreateProfile>,
        username: String,
        bio: String,
        image: Vec<u8>,
    ) -> Result<()> {
        create_profile::handler(ctx, username, bio, image)
    }

    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        bio: String,
        image: Vec<u8>,
    ) -> Result<()> {
        update_profile::handler(ctx, bio, image)
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        image: Option<Vec<u8>>,
    ) -> Result<()> {
        create_post::handler(ctx, text, image)
    }

    pub fn like_post(ctx: Context<LikePost>) -> Result<()> {
        like_post::handler(ctx)
    }

    pub fn comment_post(
        ctx: Context<CommentPost>,
        comment: String,
    ) -> Result<()> {
        comment_post::handler(ctx, comment)
    }

    pub fn follow_user(ctx: Context<FollowUser>) -> Result<()> {
        follow_user::handler(ctx)
    }

    pub fn unfollow_user(ctx: Context<UnfollowUser>) -> Result<()> {
        unfollow_user::handler(ctx)
    }
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
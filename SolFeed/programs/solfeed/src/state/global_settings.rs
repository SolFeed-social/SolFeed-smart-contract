use anchor_lang::prelude::*;

#[account]
pub struct GlobalSettings {
    pub authority: Pubkey,         //Admin
    pub treasury_wallet: Pubkey,  

    pub profile_fee: u64,          // 0.0025 SOL
    pub profile_update_fee: u64,   // 0.0020 SOL
    pub text_post_fee: u64,        // 0.00025 SOL
    pub image_post_fee: u64,       // 0.00030 SOL
    pub action_fee: u64,           // 0.00025 SOL (like, comment, follow)
    pub verification_fee: u64,     // 0.1 SOL for X verification one time
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
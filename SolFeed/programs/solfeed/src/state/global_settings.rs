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


//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

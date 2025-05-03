use anchor_lang::prelude::*;

#[account]
pub struct Profile {
    pub authority: Pubkey,     // Wallet that owns this profile
    pub username: String,      // Unique, permanent
    pub bio: String,           // Editable
    pub image: Vec<u8>,        // Profile image (stored on-chain, max 50KB)
}



//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

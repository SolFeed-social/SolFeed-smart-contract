use anchor_lang::prelude::*;

#[account]
pub struct Like {
    pub liker: Pubkey,      // Liker
    pub post: Pubkey,       // Post liked
    pub timestamp: i64,     // When
}



//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

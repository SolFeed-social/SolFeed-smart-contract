use anchor_lang::prelude::*;

#[account]
pub struct Verification {
    pub user: Pubkey,       // The verified user
    pub timestamp: i64,     // When they verified
}



//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

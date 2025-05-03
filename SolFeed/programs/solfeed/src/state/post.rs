use anchor_lang::prelude::*;

#[account]
pub struct Post {
    pub author: Pubkey,             // author
    pub text: String,               // Text content
    pub image: Option<Vec<u8>>,     // Optional on-chain image (up to 50KB)
    pub timestamp: i64,             
}



//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

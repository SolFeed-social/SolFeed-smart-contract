use anchor_lang::prelude::*;

#[account]
pub struct Comment {
    pub commenter: Pubkey,  // Author
    pub post: Pubkey,       // Which post it belongs
    pub text: String,       // The comment itself (capped by MAX_COMMENT_LENGTH)
    pub timestamp: i64,     // When
}



//   ░░░    Author: @TheItalianPimp         lucacavallaro02@proton.me
// ░░ + ░░  Name: Luca C.                   "the last universal common anchestor"
//   ░░░    Project: SolFeed (Solfeed)      solfeed.social@proton.me

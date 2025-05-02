use anchor_lang::prelude::*;

#[account]
pub struct Comment {
    pub commenter: Pubkey,  // Author
    pub post: Pubkey,       // Which post it belongs
    pub text: String,       // The comment itself (capped by MAX_COMMENT_LENGTH)
    pub timestamp: i64,     // When
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
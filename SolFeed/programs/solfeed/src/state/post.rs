use anchor_lang::prelude::*;

#[account]
pub struct Post {
    pub author: Pubkey,             // author
    pub text: String,               // Text content
    pub image: Option<Vec<u8>>,     // Optional on-chain image (up to 50KB)
    pub timestamp: i64,             
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
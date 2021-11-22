//! An ERC20-like Token program for the Solana blockchain

// pub mod error;
// pub mod instruction;
// pub mod native_mint;
// pub mod processor;
pub mod hello_world;
pub mod rps;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

//! # MDB Core — Multidimensional Binary Computing Engine
//!
//! The canonical Rust implementation of the MDB paradigm.
//!
//! MDB redefines the atomic unit of computation from a classical bit (0 or 1)
//! to a **SuperBit** — a binary string that exists as a geometric object in
//! abstract higher-dimensional coordinate space. Binary data is no longer flat;
//! it carries intrinsic dimensional properties (length, density, gravity) that
//! enable O(1) retrieval, non-destructive collapse, lossless geometric folding,
//! and deterministic evolution.
//!
//! ## Modules
//!
//! - [`coordinates`] — Dimensional address computation (D3/D4/D5)
//! - [`superbit`] — The SuperBit atomic unit with full state encoding
//! - [`definitions`] — Immutable anchor positions (DefinitionsList)
//! - [`fold`] — Lossless geometric folding engine
//! - [`unfold`] — Lossless geometric unfolding (inverse of fold)
//! - [`index`] — DimensionalIndex for O(1) guaranteed retrieval
//! - [`network`] — MDBNetwork and EntangledMemory
//! - [`evolution`] — Dimensional and learning evolution rules

pub mod coordinates;
pub mod definitions;
pub mod evolution;
pub mod fold;
pub mod index;
pub mod network;
pub mod superbit;
pub mod unfold;

/// The Golden Ratio — fundamental constant used in D5 gravity computation.
pub const PHI: f64 = 1.618_033_988_749_895;

/// MDB version string.
pub const VERSION: &str = "0.1.0";

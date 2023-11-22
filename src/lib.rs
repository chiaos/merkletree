//! # Merkle Tree Library for Ethereum
//!
//! This library provides a simple implementation of a Merkle Tree.
//!
//! ## Usage
//!
//! ```rust
//! use merkle_tree::MerkleTree;
//! use tiny_keccak::{ Keccak, Hasher};
//!
//!
//! #[derive(Clone, Debug)]
//! pub struct KeccakHasher;
//! impl merkle_tree::Hasher for KeccakHasher {
//!     type Hash = [u8; 32];
//!
//!     fn hash(&self, value: &[u8]) -> Self::Hash {
//!         keccak256(value)
//!     }
//! }
//!
//! fn keccak256(data: &[u8]) -> [u8; 32] {
//!     let mut hasher = Keccak::v256();
//!     let mut hash = [0_u8; 32];
//!     hasher.update(data);
//!     hasher.finalize(&mut hash);
//!     hash
//! }
//!
//! let leaves = vec!["a", "b", "c", "d", "e", "f"].iter().map(|x| keccak256(x.as_bytes())).collect::<Vec<_>>();
//! let mtree = merkle_tree::MerkleTree::<KeccakHasher>::new(KeccakHasher, leaves, None);
//! let root = mtree.root();
//!
//! // verify the proof of the first leaf
//! let leaf = keccak256("a".as_bytes());
//! let proof = mtree.proof(&leaf).unwrap();
//! assert!(mtree.verify(&leaf, &root, &proof));
//! ```
//!

pub mod hash;
pub mod merkle;

pub use hash::Hasher;
pub use merkle::*;

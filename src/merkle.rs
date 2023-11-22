use crate::hash::Hasher;

/// Maximum supported depth of the tree. 32 corresponds to `2^32` elements in the tree, which
/// we unlikely to ever hit.
const MAX_TREE_DEPTH: usize = 32;

pub struct MerkleOptions {
	pub min_tree_size: Option<usize>,
	/// If set to `true`, the leaves will hashed using the set hashing algorithms.
	pub hash_leaves: Option<bool>,
	/// If `true`, the leaves are sorted before building the tree.
	pub sort_leaves: Option<bool>,
	/// If `true`, the pairs of adjacent leaves are sorted before computing the parent hash.
	pub sort_pairs: Option<bool>,

	/// If `true`, sort_pairs is set to `true` and sort_pairs is set to `true`.
	pub sort: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MerkleTree<H>
where
	H: Hasher,
{
	hasher: H,
	hashes: Vec<H::Hash>,
	binary_tree_size: usize,
	hash_leaves: bool,
	sort_leaves: bool,
	sort_pairs: bool,
	sort: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
	Left,
	Right,
}

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Position::Left => write!(f, "left"),
			Position::Right => write!(f, "right"),
		}
	}
}

#[derive(Debug, Clone)]
pub struct MerkleProof<H>
where
	H: Hasher,
{
	pub data: H::Hash,
	pub position: Position,
}

impl<H> MerkleTree<H>
where
	H: Hasher,
{
	pub fn new(hasher: H, leaves: impl IntoIterator<Item = H::Hash>, opts: Option<MerkleOptions>) -> Self {
		let (mut hash_leaves, mut sort_leaves, mut sort_pairs, mut sort) = (false, false, false, false);
		let mut min_tree_size = None;
		if let Some(opts) = opts {
			hash_leaves = opts.hash_leaves.unwrap_or(false);
			sort_leaves = opts.sort_leaves.unwrap_or(false);
			sort_pairs = opts.sort_pairs.unwrap_or(false);
			sort = opts.sort.unwrap_or(false);
			if sort {
				sort_leaves = true;
				sort_pairs = true;
			}
			min_tree_size = opts.min_tree_size;
		}

		let mut hashes: Vec<_> = leaves
			.into_iter()
			.map(|bytes| if hash_leaves { hasher.hash(bytes.as_ref()) } else { bytes })
			.collect();

		if sort_leaves {
			hashes.sort();
		}

		let mut binary_tree_size = hashes.len().next_power_of_two();
		if let Some(min_tree_size) = min_tree_size {
			assert!(min_tree_size.is_power_of_two(), "tree size must be a power of 2");
			binary_tree_size = min_tree_size.max(binary_tree_size);
		}
		assert!(
			tree_depth_by_size(binary_tree_size) <= MAX_TREE_DEPTH,
			"Tree contains more than {} items; this is not supported",
			1 << MAX_TREE_DEPTH
		);

		Self {
			hasher,
			hashes,
			binary_tree_size,
			hash_leaves,
			sort_leaves,
			sort_pairs,
			sort,
		}
	}

	/// Returns the root hash of this tree.
	/// # Panics
	/// Will panic if the constant below is invalid.
	pub fn root(&self) -> H::Hash {
		if self.hashes.is_empty() {
			panic!("merkle root of empty tree is not defined");
		} else {
			self.compute(0, None)
		}
	}

	pub fn proof(&self, leaf: &[u8]) -> Option<Vec<MerkleProof<H>>> {
		let index = self.hashes.iter().position(|x| x.as_ref() == leaf)?;

		let mut merkle_path = vec![];
		let _ = self.compute(index, Some(&mut merkle_path));
		Some(merkle_path)
	}

	pub fn verify(&self, leaf: &[u8], root: &[u8], proof: &[MerkleProof<H>]) -> bool {
		let mut hash = leaf.to_vec();
		for p in proof {
			if self.sort_pairs {
				let mut v = vec![hash.clone(), p.data.as_ref().to_vec()];
				v.sort();
				let mut combine = v[0].clone();
				combine.extend(v[1].iter());
				hash = self.hasher.hash(&combine).as_ref().to_vec();
			} else if p.position == Position::Left {
				let mut combine = p.data.as_ref().to_vec();
				combine.extend(hash);
				hash = self.hasher.hash(combine.as_ref()).as_ref().to_vec();
			} else {
				let mut combine = hash.clone();
				combine.extend(p.data.as_ref());
				hash = self.hasher.hash(combine.as_ref()).as_ref().to_vec();
			}
		}
		hash == root
	}

	/// Returns the root hash and the Merkle proof for a leaf with the specified 0-based `index`.
	#[allow(dead_code)]
	fn merkle_root_and_path(&self, index: usize) -> (H::Hash, Vec<MerkleProof<H>>) {
		let mut merkle_path = vec![];
		let root_hash = self.compute(index, Some(&mut merkle_path));
		(root_hash, merkle_path)
	}

	fn compute(&self, mut index: usize, mut merkle_path: Option<&mut Vec<MerkleProof<H>>>) -> H::Hash {
		assert!(index < self.hashes.len(), "invalid tree leaf index");

		let depth = tree_depth_by_size(self.binary_tree_size);
		if let Some(merkle_path) = merkle_path.as_deref_mut() {
			merkle_path.reserve(depth);
		}

		let mut hashes = self.hashes.clone();
		let mut level_len = hashes.len();
		for _level in 0..depth {
			if let Some(merkle_path) = merkle_path.as_deref_mut() {
				let adjacent_idx = index ^ 1;
				if adjacent_idx < level_len {
					let p = MerkleProof {
						data: hashes[adjacent_idx].clone(),
						position: if adjacent_idx <= index { Position::Left } else { Position::Right },
					};

					merkle_path.push(p);
				}
			}

			for i in 0..(level_len / 2) {
				if self.sort_pairs {
					hashes[2 * i..2 * i + 2].sort();
				}
				// combine hashes[2 * i], &hashes[2 * i + 1]
				let mut combine = hashes[2 * i].as_ref().to_vec();
				combine.extend(hashes[2 * i + 1].as_ref());
				hashes[i] = self.hasher.hash(combine.as_ref());
			}
			if level_len % 2 == 1 {
				hashes[level_len / 2] = hashes[level_len - 1].clone();
			}

			index /= 2;
			level_len = level_len / 2 + level_len % 2;
		}
		hashes[0].clone()
	}
}

fn tree_depth_by_size(tree_size: usize) -> usize {
	debug_assert!(tree_size.is_power_of_two());
	tree_size.trailing_zeros() as usize
}

#[cfg(test)]
mod tests {
	use super::*;

	use hex_literal::hex;

	use tiny_keccak::{Hasher, Keccak};

	fn keccak256(value: &[u8]) -> [u8; 32] {
		let mut hasher = Keccak::v256();
		let mut hash = [0_u8; 32];
		hasher.update(value);
		hasher.finalize(&mut hash);
		hash
	}

	#[derive(Clone)]
	pub struct KeccakHasher;
	impl super::Hasher for KeccakHasher {
		type Hash = [u8; 32];

		fn hash(&self, value: &[u8]) -> Self::Hash {
			keccak256(value)
		}
	}
	// [u8; 32] to hex string
	fn to_hex_string(hash: &[u8]) -> String {
		let mut s = String::new();
		for h in hash {
			s.push_str(&format!("{:02x}", h));
		}
		s
	}

	impl std::fmt::Display for MerkleProof<KeccakHasher> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "data: {}, position: {}", to_hex_string(self.data.as_ref()), self.position)
		}
	}

	#[test]
	fn test_k256() {
		let n = keccak256(&[]);
		println!("n: {:?}", to_hex_string(&n));
	}

	#[test]
	fn test_merkle_root() {
		let leaves = vec![
			hex!("8bc6a1d45d6d4c063a0936e28d702bcc15bd5a2dc8eb65e85006915f191e294c"),
			hex!("f49675d9423d176411d875b32fd9f272269f0b4a44f52b16d40e575a6c1bea61"),
			hex!("84f2c42c72ab408b2a49be2c2ccf3f896b05308a0116bce5e93d073189b250f7"),
			hex!("e6a12952e10e9bd1585df31d4fdf615a41f2a797779bf7c9b55e6facdbde6430"),
			hex!("02582971c6440ea5aab5d02bc054c6ba3c3a6025790543420e0ca186de30f6cf"),
			hex!("b7732e74565cc365a9c58f349f11d94700bcee8fc5ba5ba5a508236ca1d300e4"),
			hex!("79a15a38292be04bf2376a164674d975a7a596afd8d6fe85981a6305c849bb99"),
			hex!("d6fd2aaabaebf50980d2184e9141ce286111b4f7a39b32bd0ecac22871a4ed14"),
			hex!("5b2f452cde6154eeb8227eb91d5fb1f2e31d1e16cac741d4ec6ce18593fd6792"),
			hex!("bbc69bfe1697e473296758de9b9796948a13b6b1ad2b91fcd6118916fdff8dea"),
			hex!("3389eff6964486b49edb5f54cf9a833aee42298ebe250c094ae3a0857944ba5c"),
			hex!("5b920af4c112b8bd1c3ef32c2f24f8f3f891fc3c85b8285bbf21cb9a4573b608"),
		];

		let tree = MerkleTree::new(
			KeccakHasher,
			leaves.clone(),
			Some(MerkleOptions {
				min_tree_size: None,
				hash_leaves: None,
				sort_leaves: None,
				sort_pairs: None,
				sort: Some(true),
			}),
		);

		let root = tree.root();
		let expect_root = hex!("fb2a038476ab757a1c8bd9bf1b329eb3c2a9e5db7415eaee9acdbbe1ae789aeb");
		assert_eq!(
			root,
			expect_root,
			"merkle root mismatch, expect: {:?}, got: {:?}",
			to_hex_string(&expect_root),
			to_hex_string(&root)
		);
		// 3a1c0c3404f3cb834b40c4c477d4da4e40df75a9bf771b0461277e25b8e1db88
		// 74af6d554f807094a752ff496ec0b907822d0775ee823cb92cf6ae01258b30ea
		// 315e78532a270b2954079858c4abfaebc0a49a36c3c1b2684da3847a161d5bd6
		// 15a8528ae2d9f85886bcc8f0ee24f8a6d0c1ec57ef3dc0bbcc964cbe820a7d95
		// 9861cae0002869d2f4c65e4a31500ea55d0a2686d44dbdeeedc29e09196a1e99

		let index = 1;

		let proof = tree.proof(leaves[index].as_ref()).unwrap();
		// for p in proof.iter() {
		// 	println!("proof +: {:?},{}", to_hex_string(p.data.as_ref()), p.position);
		// }

		let v = tree.verify(leaves[index].as_ref(), root.as_ref(), &proof);
		assert!(
			v,
			"verify failed, leaf: {:?}, root: {:?}",
			to_hex_string(leaves[index].as_ref()),
			to_hex_string(root.as_ref())
		);

		for l in leaves.iter() {
			let proof = tree.proof(l.as_ref()).unwrap();

			let v = tree.verify(l.as_ref(), root.as_ref(), &proof);
			assert!(
				v,
				"loop :verify failed, leaf: {:?}, root: {:?}",
				to_hex_string(l.as_ref()),
				to_hex_string(root.as_ref())
			);
		}

		assert_eq!(
			root,
			expect_root,
			"merkle root mismatch, expect: {:?}, got: {:?}",
			to_hex_string(&expect_root),
			to_hex_string(&root)
		);
		// println!("root: {:?}", to_hex_string(root.as_ref()));
	}

	#[test]
	fn test_index2() {
		let l1 =
			hex!("e6a12952e10e9bd1585df31d4fdf615a41f2a797779bf7c9b55e6facdbde6430f49675d9423d176411d875b32fd9f272269f0b4a44f52b16d40e575a6c1bea61");
		let _h1 = keccak256(l1.as_ref());
		let l2 =
			hex!("0db8d09ee11b02e821ef5624b60d658c297fc8142bdd59a94ce22d92d44a5a1e78da1389c734a3d24a3507be4031aa3ab3f703cbe59ed5fe868b04c787b68517")
				.to_vec();

		let h2 = keccak256(l2.as_ref());

		let mut l3 = hex!("a3647b2eff717882dabdf2e0ace205337209cf3d9f4b888810ef18939fc6834f").to_vec();
		l3.extend(h2);
		let h3 = keccak256(l3.as_ref());

		assert_eq!(h3, hex!("fb2a038476ab757a1c8bd9bf1b329eb3c2a9e5db7415eaee9acdbbe1ae789aeb"));
	}
}
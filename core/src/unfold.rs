//! # Geometric Unfolding Engine
//!
//! Unfolding is the exact inverse of folding — it restores the original binary
//! data from its folded geometric representation with **zero information loss**.
//!
//! ```text
//! unfold(fold(data)) == data  ∀ data
//! ```
//!
//! Every unfold operation verifies the SHA-256 hash of the restored data against
//! the hash stored during folding. If they don't match, the data has been
//! corrupted and the unfold fails with an explicit error.

use crate::coordinates::DimensionalAddress;
use crate::fold::{
    decode_folded, inverse_permutation, sha256, FoldError, FoldedData,
};

/// Unfold a FoldedData structure back to the original bytes.
///
/// This is the inverse of `fold()`. It:
/// 1. Reverses fold layers in reverse order using stored layer addresses
/// 2. Verifies SHA-256 hash matches the original
///
/// Returns the original data or an error if verification fails.
pub fn unfold(folded: &FoldedData) -> Result<Vec<u8>, FoldError> {
    let mut data = folded.payload.clone();

    // Unwrap fold layers in reverse order.
    // layer_addresses[0] was used for the first fold, [1] for the second, etc.
    // We reverse them to undo the last fold first.
    for layer_idx in (0..folded.fold_depth as usize).rev() {
        let addr = if layer_idx < folded.layer_addresses.len() {
            &folded.layer_addresses[layer_idx]
        } else {
            &folded.address
        };
        data = geometric_unfold_payload(&data, addr);
    }

    // Trim to original bit length if needed
    let original_byte_length = ((folded.original_bit_length + 7) / 8) as usize;
    if data.len() > original_byte_length {
        data.truncate(original_byte_length);
    }

    // SHA-256 verification — the lossless guarantee
    let hash = sha256(&data);
    if hash != folded.original_hash {
        return Err(FoldError::HashMismatch);
    }

    Ok(data)
}

/// Unfold from encoded binary format.
///
/// Combines decoding and unfolding in one step.
pub fn unfold_from_bytes(encoded: &[u8]) -> Result<Vec<u8>, FoldError> {
    let folded = decode_folded(encoded)?;
    unfold(&folded)
}

/// Reverse the geometric fold transformation for a single layer.
fn geometric_unfold_payload(data: &[u8], address: &DimensionalAddress) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let len = data.len();

    // Step 1: Remove gravity mask (XOR is self-inverse)
    let mask = gravity_mask_for_unfold(len, address.d5);
    let mut unmasked = data.to_vec();
    for (i, m) in unmasked.iter_mut().zip(mask.iter()) {
        *i ^= m;
    }

    // Step 2: Reverse the permutation
    let perm = dimension_derived_permutation_for_unfold(len, address);
    let inv = inverse_permutation(&perm);

    // During fold: folded[i] = data[perm[i]]
    // So to reverse: data[perm[i]] = unmasked[i]
    let mut unfolded = vec![0u8; len];
    for (i, &perm_idx) in perm.iter().enumerate() {
        unfolded[perm_idx] = unmasked[i];
    }

    unfolded
}

/// Reproduce the same permutation used during folding.
/// (Must be identical to fold::dimension_derived_permutation)
fn dimension_derived_permutation_for_unfold(
    len: usize,
    addr: &DimensionalAddress,
) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }

    let seed = addr
        .d3
        .wrapping_mul(2654435761)
        .wrapping_add((addr.d4 * 1_000_000.0) as u64)
        .wrapping_add((addr.d5 * 1_000_000_000.0) as u64);

    let mut indices: Vec<usize> = (0..len).collect();

    let mut rng_state = seed;
    for i in (1..len).rev() {
        rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (rng_state >> 33) as usize % (i + 1);
        indices.swap(i, j);
    }

    indices
}

/// Reproduce the same gravity mask used during folding.
/// (Must be identical to fold::gravity_mask)
fn gravity_mask_for_unfold(len: usize, d5: f64) -> Vec<u8> {
    let mut mask = Vec::with_capacity(len);
    let mut state = (d5 * 1_000_000_000.0) as u64;
    for _ in 0..len {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        mask.push((state >> 40) as u8);
    }
    mask
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fold::{encode_folded, fold, fold_with_depth};

    #[test]
    fn test_fold_unfold_identity_simple() {
        let data = b"Hello, MDB world!";
        let folded = fold(data);
        let restored = unfold(&folded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_fold_unfold_identity_binary() {
        // Test with raw binary data (all byte values)
        let data: Vec<u8> = (0..=255).collect();
        let folded = fold(&data);
        let restored = unfold(&folded).unwrap();
        assert_eq!(restored, data);
    }

    #[test]
    fn test_fold_unfold_empty() {
        let data = b"";
        let folded = fold(data);
        let restored = unfold(&folded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_fold_unfold_single_byte() {
        for b in 0..=255u8 {
            let data = vec![b];
            let folded = fold(&data);
            let restored = unfold(&folded).unwrap();
            assert_eq!(restored, data, "failed for byte {}", b);
        }
    }

    #[test]
    fn test_fold_unfold_depth_2() {
        let data = b"Recursive folding test with depth 2";
        let folded = fold_with_depth(data, 2);
        assert_eq!(folded.fold_depth, 2);
        assert_eq!(folded.layer_addresses.len(), 2);
        let restored = unfold(&folded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_fold_unfold_depth_3() {
        let data = b"Even deeper folding at depth 3!";
        let folded = fold_with_depth(data, 3);
        assert_eq!(folded.fold_depth, 3);
        assert_eq!(folded.layer_addresses.len(), 3);
        let restored = unfold(&folded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_fold_unfold_depth_5() {
        let data = b"Deep recursive folding test at depth 5 with more data to fold";
        let folded = fold_with_depth(data, 5);
        assert_eq!(folded.fold_depth, 5);
        let restored = unfold(&folded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_fold_unfold_large_data() {
        // 64KB of pseudorandom data
        let mut data = Vec::with_capacity(65536);
        let mut state: u64 = 0xDEADBEEF;
        for _ in 0..65536 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            data.push((state >> 33) as u8);
        }
        let folded = fold(&data);
        let restored = unfold(&folded).unwrap();
        assert_eq!(restored, data);
    }

    #[test]
    fn test_fold_unfold_large_data_depth_2() {
        // 64KB of pseudorandom data at depth 2
        let mut data = Vec::with_capacity(65536);
        let mut state: u64 = 0xCAFEBABE;
        for _ in 0..65536 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            data.push((state >> 33) as u8);
        }
        let folded = fold_with_depth(&data, 2);
        let restored = unfold(&folded).unwrap();
        assert_eq!(restored, data);
    }

    #[test]
    fn test_sha256_verification_catches_corruption() {
        let data = b"This data must not be corrupted";
        let mut folded = fold(data);

        // Corrupt one byte in the payload
        if !folded.payload.is_empty() {
            folded.payload[0] ^= 0xFF;
        }

        let result = unfold(&folded);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FoldError::HashMismatch);
    }

    #[test]
    fn test_encode_decode_unfold_pipeline() {
        let data = b"Full pipeline: fold -> encode -> decode -> unfold";
        let folded = fold(data);
        let encoded = encode_folded(&folded);
        let restored = unfold_from_bytes(&encoded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_encode_decode_unfold_pipeline_depth_3() {
        let data = b"Full pipeline with depth 3";
        let folded = fold_with_depth(data, 3);
        let encoded = encode_folded(&folded);
        let restored = unfold_from_bytes(&encoded).unwrap();
        assert_eq!(&restored, data);
    }

    #[test]
    fn test_all_zeros() {
        let data = vec![0u8; 1000];
        let folded = fold(&data);
        let restored = unfold(&folded).unwrap();
        assert_eq!(restored, data);
    }

    #[test]
    fn test_all_ones() {
        let data = vec![0xFFu8; 1000];
        let folded = fold(&data);
        let restored = unfold(&folded).unwrap();
        assert_eq!(restored, data);
    }
}

//! # Dimensional Coordinate System
//!
//! Every binary string in MDB exists as a point in ℝ³ (extendable to ℝ^∞)
//! defined by its intrinsic dimensional properties. These are not metadata —
//! they are active computational dimensions.
//!
//! - **D3 (Temporal)** — The length of the binary string |S| = n
//! - **D4 (Density)** — The ratio of 1-bits to total bits (probability mass)
//! - **D5 (Gravity)** — A unique relational signature computed via the Golden Ratio φ
//!
//! The dimensional address `addr(S) = (D3, D4, D5)` uniquely identifies a binary
//! string's position in abstract coordinate space and enables O(1) retrieval
//! through the DimensionalIndex.

use crate::PHI;

/// A point in MDB's 3D coordinate space.
///
/// Represents the intrinsic dimensional address of a binary string or SuperBit.
#[derive(Debug, Clone, PartialEq)]
pub struct DimensionalAddress {
    /// D3 — Temporal coordinate (string length).
    pub d3: u64,
    /// D4 — Density coordinate (ratio of 1-bits to total bits).
    pub d4: f64,
    /// D5 — Gravity coordinate (Golden Ratio weighted positional signature).
    pub d5: f64,
}

impl DimensionalAddress {
    /// Compute the dimensional address of a raw binary string (sequence of 0/1 bytes).
    ///
    /// The input `bits` is a slice of `u8` where each element is 0 or 1.
    pub fn from_bits(bits: &[u8]) -> Self {
        Self {
            d3: d3_temporal(bits),
            d4: d4_density(bits),
            d5: d5_gravity(bits),
        }
    }

    /// Compute the dimensional address from raw byte data.
    ///
    /// Expands each byte into 8 individual bits before computing coordinates.
    pub fn from_bytes(data: &[u8]) -> Self {
        let bits = bytes_to_bits(data);
        Self::from_bits(&bits)
    }

    /// Return the address as a tuple for indexing/hashing purposes.
    pub fn as_tuple(&self) -> (u64, u64, u64) {
        // Quantize f64 coordinates to u64 for deterministic hashing.
        // D4 is quantized to parts-per-million, D5 to parts-per-billion.
        let d4_quantized = (self.d4 * 1_000_000.0).round() as u64;
        let d5_quantized = (self.d5 * 1_000_000_000.0).round() as u64;
        (self.d3, d4_quantized, d5_quantized)
    }
}

impl std::fmt::Display for DimensionalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "addr(D3={}, D4={:.6}, D5={:.9})", self.d3, self.d4, self.d5)
    }
}

/// **D3 — Temporal coordinate**: the length of the binary string.
///
/// D3(S) = |S| = n
pub fn d3_temporal(bits: &[u8]) -> u64 {
    bits.len() as u64
}

/// **D4 — Density coordinate**: the ratio of 1-bits to total bits.
///
/// D4(S) = |{i : S\[i\] = 1}| / n
pub fn d4_density(bits: &[u8]) -> f64 {
    if bits.is_empty() {
        return 0.0;
    }
    let ones = bits.iter().filter(|&&b| b == 1).count();
    ones as f64 / bits.len() as f64
}

/// **D5 — Gravity coordinate**: Golden Ratio weighted positional signature.
///
/// D5(S) = Σᵢ (S\[i\] · φ · i) mod 1
///
/// where φ = 1.6180339887... (the Golden Ratio)
pub fn d5_gravity(bits: &[u8]) -> f64 {
    let mut sum: f64 = 0.0;
    for (i, &bit) in bits.iter().enumerate() {
        if bit == 1 {
            sum += PHI * (i as f64);
        }
    }
    // mod 1: take fractional part
    sum - sum.floor()
}

/// Compute the full dimensional address for a bit sequence.
pub fn dimensional_address(bits: &[u8]) -> DimensionalAddress {
    DimensionalAddress::from_bits(bits)
}

/// Expand a byte slice into individual bits (MSB first per byte).
pub fn bytes_to_bits(data: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(data.len() * 8);
    for &byte in data {
        for shift in (0..8).rev() {
            bits.push((byte >> shift) & 1);
        }
    }
    bits
}

/// Pack individual bits (0/1 values) back into bytes (MSB first).
///
/// If the number of bits is not a multiple of 8, the final byte is zero-padded.
pub fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity((bits.len() + 7) / 8);
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        bytes.push(byte);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d3_temporal() {
        assert_eq!(d3_temporal(&[0, 1, 1, 0]), 4);
        assert_eq!(d3_temporal(&[]), 0);
        assert_eq!(d3_temporal(&[1; 100]), 100);
    }

    #[test]
    fn test_d4_density() {
        let eps = 1e-10;
        assert!((d4_density(&[0, 1, 1, 0]) - 0.5).abs() < eps);
        assert!((d4_density(&[1, 1, 1, 1]) - 1.0).abs() < eps);
        assert!((d4_density(&[0, 0, 0, 0]) - 0.0).abs() < eps);
        assert_eq!(d4_density(&[]), 0.0);
    }

    #[test]
    fn test_d5_gravity() {
        // D5 for all zeros should be 0.0
        assert_eq!(d5_gravity(&[0, 0, 0, 0]), 0.0);

        // D5 for a known pattern
        // bits = [1, 0, 1, 0] => sum = φ*0 + φ*2 = 2φ = 3.23606...
        // 3.23606... mod 1 = 0.23606...
        let d5 = d5_gravity(&[1, 0, 1, 0]);
        let expected = (PHI * 0.0 + PHI * 2.0) % 1.0;
        assert!((d5 - expected).abs() < 1e-10);
    }

    #[test]
    fn test_dimensional_address_uniqueness() {
        // Different lengths => different D3 => different addresses
        let a1 = dimensional_address(&[1, 0, 1]);
        let a2 = dimensional_address(&[1, 0, 1, 0]);
        assert_ne!(a1.d3, a2.d3);

        // Same length, different density => different D4
        let a3 = dimensional_address(&[1, 1, 0, 0]);
        let a4 = dimensional_address(&[1, 1, 1, 0]);
        assert_eq!(a3.d3, a4.d3);
        assert_ne!(a3.d4, a4.d4);
    }

    #[test]
    fn test_bytes_to_bits_roundtrip() {
        let data = vec![0xAB, 0xCD, 0xEF];
        let bits = bytes_to_bits(&data);
        let back = bits_to_bytes(&bits);
        assert_eq!(data, back);
    }

    #[test]
    fn test_address_display() {
        let addr = dimensional_address(&[1, 0, 1, 1]);
        let s = format!("{}", addr);
        assert!(s.starts_with("addr("));
    }
}

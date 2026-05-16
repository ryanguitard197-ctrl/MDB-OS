//! # Evolution — The Execution Primitive
//!
//! In MDB, the classical CPU cycle is replaced by the **evolution step**.
//! A SuperBit doesn't execute instructions — it *evolves* according to
//! deterministic rules derived from its dimensional coordinates.
//!
//! Two modes of evolution:
//!
//! 1. **Dimensional Evolution** — The raw binary string transforms based on
//!    its D3 (temporal) coordinate. For odd-length strings, the middle bit
//!    flips; for even-length strings, the last bit flips. Protected positions
//!    (anchors from the DefinitionsList) are never modified.
//!
//! 2. **Learning Evolution** — Probabilities (W) are reweighted based on
//!    observed outcomes and rewards. The SuperBit *learns*. The updated state
//!    is re-encoded into the binary string.

use crate::superbit::SuperBit;

/// Result of an evolution step.
#[derive(Debug)]
pub struct EvolutionResult {
    /// The position that was modified (if any).
    pub modified_position: Option<usize>,
    /// Whether the evolution was blocked by an anchor.
    pub blocked_by_anchor: bool,
    /// The new generation number.
    pub generation: u64,
}

/// Perform a **dimensional evolution** step on a SuperBit.
///
/// Rules (from the formal spec):
/// - If D3 (length) is odd: flip the middle bit at position ⌊n/2⌋
/// - If D3 (length) is even: flip the last bit at position n-1
/// - Anchored positions (DefinitionsList) are NEVER modified
///
/// The generation counter increments after each evolution.
pub fn evolve_dimensional(sb: &mut SuperBit) -> EvolutionResult {
    let n = sb.sigma.len();
    if n == 0 {
        sb.generation += 1;
        return EvolutionResult {
            modified_position: None,
            blocked_by_anchor: false,
            generation: sb.generation,
        };
    }

    // Determine target position based on D3 parity
    let target = if n % 2 == 1 { n / 2 } else { n - 1 };

    // Check if the position is anchored
    let blocked = sb.anchors.is_anchored(target);

    if !blocked {
        // Flip the bit
        sb.sigma[target] ^= 1;
    }

    sb.generation += 1;

    EvolutionResult {
        modified_position: if blocked { None } else { Some(target) },
        blocked_by_anchor: blocked,
        generation: sb.generation,
    }
}

/// Perform a **learning evolution** step on a SuperBit.
///
/// Given an observed state index and a reward value, the SuperBit adjusts
/// its probability weights to favor (positive reward) or disfavor (negative
/// reward) that state. The weights are then renormalized.
///
/// ```text
/// W'ⱼ = Wⱼ + r  if j = i  else  Wⱼ
/// W' normalized so Σw'ⱼ = 1
/// G' = G + 1
/// ```
///
/// Returns the old and new weight of the reinforced state.
pub fn evolve_learning(
    sb: &mut SuperBit,
    state_index: usize,
    reward: f64,
) -> Result<(f64, f64), EvolutionError> {
    if state_index >= sb.states.len() {
        return Err(EvolutionError::StateIndexOutOfBounds(
            state_index,
            sb.states.len(),
        ));
    }

    let old_weight = sb.weights[state_index];

    // Apply reward
    sb.weights[state_index] += reward;

    // Clamp to non-negative
    for w in &mut sb.weights {
        if *w < 0.0 {
            *w = 0.0;
        }
    }

    // Renormalize
    sb.normalize_weights();

    sb.generation += 1;

    let new_weight = sb.weights[state_index];
    Ok((old_weight, new_weight))
}

/// Run multiple dimensional evolution steps.
pub fn evolve_dimensional_n(sb: &mut SuperBit, steps: usize) -> Vec<EvolutionResult> {
    (0..steps).map(|_| evolve_dimensional(sb)).collect()
}

/// Errors that can occur during evolution.
#[derive(Debug, Clone, PartialEq)]
pub enum EvolutionError {
    StateIndexOutOfBounds(usize, usize),
}

impl std::fmt::Display for EvolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateIndexOutOfBounds(idx, len) => {
                write!(f, "state index {} out of bounds (state count: {})", idx, len)
            }
        }
    }
}

impl std::error::Error for EvolutionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::DefinitionsList;
    use crate::superbit::{State, SuperBit};

    #[test]
    fn test_dimensional_evolution_odd_length() {
        // Length 5 (odd) → flip middle (position 2)
        let mut sb = SuperBit::from_bits(vec![1, 0, 0, 1, 1]);
        let result = evolve_dimensional(&mut sb);
        assert_eq!(result.modified_position, Some(2));
        assert!(!result.blocked_by_anchor);
        assert_eq!(sb.sigma[2], 1); // 0 → 1
        assert_eq!(sb.generation, 1);
    }

    #[test]
    fn test_dimensional_evolution_even_length() {
        // Length 4 (even) → flip last (position 3)
        let mut sb = SuperBit::from_bits(vec![1, 0, 1, 0]);
        let result = evolve_dimensional(&mut sb);
        assert_eq!(result.modified_position, Some(3));
        assert_eq!(sb.sigma[3], 1); // 0 → 1
    }

    #[test]
    fn test_dimensional_evolution_blocked_by_anchor() {
        // Length 5 (odd) → target position 2, but it's anchored
        let mut sb = SuperBit::from_bits(vec![1, 0, 0, 1, 1]);
        sb.anchors.add(2);
        let result = evolve_dimensional(&mut sb);
        assert_eq!(result.modified_position, None);
        assert!(result.blocked_by_anchor);
        assert_eq!(sb.sigma[2], 0); // unchanged!
        assert_eq!(sb.generation, 1); // generation still increments
    }

    #[test]
    fn test_dimensional_evolution_double_flip_restores() {
        // Two dimensional evolutions on the same position should restore original
        let original = vec![1, 0, 1, 1, 0];
        let mut sb = SuperBit::from_bits(original.clone());
        evolve_dimensional(&mut sb);
        evolve_dimensional(&mut sb);
        assert_eq!(sb.sigma, original);
        assert_eq!(sb.generation, 2);
    }

    #[test]
    fn test_learning_evolution() {
        let states = vec![
            State { label: "a".into(), pattern: vec![0] },
            State { label: "b".into(), pattern: vec![1] },
        ];
        let mut sb = SuperBit::with_states(
            vec![0, 1],
            states,
            vec![0.5, 0.5],
            DefinitionsList::new(),
        ).unwrap();

        // Reward state 0
        let (old, new) = evolve_learning(&mut sb, 0, 0.3).unwrap();
        assert!((old - 0.5).abs() < 1e-10);
        assert!(new > 0.5); // weight increased
        let sum: f64 = sb.weights.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10); // still normalized
        assert_eq!(sb.generation, 1);
    }

    #[test]
    fn test_learning_evolution_negative_reward() {
        let states = vec![
            State { label: "a".into(), pattern: vec![0] },
            State { label: "b".into(), pattern: vec![1] },
        ];
        let mut sb = SuperBit::with_states(
            vec![0, 1],
            states,
            vec![0.5, 0.5],
            DefinitionsList::new(),
        ).unwrap();

        // Penalize state 0
        let (_, new) = evolve_learning(&mut sb, 0, -0.3).unwrap();
        assert!(new < 0.5); // weight decreased
    }

    #[test]
    fn test_learning_evolution_clamps_negative() {
        let states = vec![
            State { label: "a".into(), pattern: vec![0] },
            State { label: "b".into(), pattern: vec![1] },
        ];
        let mut sb = SuperBit::with_states(
            vec![0, 1],
            states,
            vec![0.1, 0.9],
            DefinitionsList::new(),
        ).unwrap();

        // Heavy penalty that would make weight negative
        evolve_learning(&mut sb, 0, -1.0).unwrap();
        assert!(sb.weights[0] >= 0.0); // clamped
        assert!((sb.weights.iter().sum::<f64>() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_learning_evolution_out_of_bounds() {
        let mut sb = SuperBit::from_bits(vec![0, 1]);
        let result = evolve_learning(&mut sb, 5, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_evolve_n() {
        let mut sb = SuperBit::from_bits(vec![1, 0, 1, 0, 1]);
        let results = evolve_dimensional_n(&mut sb, 10);
        assert_eq!(results.len(), 10);
        assert_eq!(sb.generation, 10);
    }
}

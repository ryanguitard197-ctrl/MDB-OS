# MDB OS — Multidimensional Binary Operating System

> *"The bit itself has changed."*

MDB redefines the atomic unit of computation from a classical bit (0 or 1) to a **SuperBit** — a binary string that exists as a geometric object in abstract higher-dimensional coordinate space. Binary data is no longer flat; it carries intrinsic dimensional properties that enable O(1) retrieval, non-destructive collapse, lossless geometric folding, and deterministic evolution.

**Invented by Ryan Guitard.**

## Dimensional Cascade (v0.2)

MDB's coordinate engine is built on a **recursive Fibonacci cascade** where each dimension derives from the two below it. This is not a set of independent features computed on a string — it is a self-building tower where every new dimension *emerges* from the previous ones.

```
D1 → D2 → D3 → D4 = D2+D3 → D5 = D3+D4 → D6 = D4+D5 → …
```

| Dim | Name       | Formula (per position i)                   | What it represents            |
|-----|------------|---------------------------------------------|-------------------------------|
| D1  | Value      | w_i = 0.3 if bit=1, 0.2 if bit=0           | Existence — the raw bit       |
| D2  | Space      | D2_i = w_i × n                              | Length-scaled position weight  |
| D3  | Time       | D3_i = D2_i (1:1 mapping)                  | Simulated time from space     |
| D4  | Spacetime  | D4_i = D2_i + D3_i                         | The first combined dimension  |
| D5  | Momentum   | D5_i = D3_i + D4_i                         | Movement through spacetime    |
| D6  | Energy     | D6_i = D4_i + D5_i                         | Emerges from momentum         |
| D7+ | (recurse)  | D(k)_i = D(k-2)_i + D(k-1)_i              | Infinite recursive extension  |

### Emergent properties

- **Fibonacci coefficients**: D2=n, D3=n, D4=2n, D5=3n, D6=5n, D7=8n, …
- **Golden Ratio convergence**: D(k)/D(k-1) → φ ≈ 1.618 as k → ∞
- **Lossless recovery**: The original binary string is always recoverable from D1 (0.3 → 1, 0.2 → 0)
- **Zero collisions**: Every binary string occupies a unique point in dimensional space

## Architecture

```
mdb-os/
├── core/              # Rust — the canonical MDB engine (v0.2.0)
│   ├── src/
│   │   ├── lib.rs           # Crate root
│   │   ├── coordinates.rs   # Cascade engine & dimensional addressing
│   │   ├── superbit.rs      # The SuperBit — atomic unit of MDB
│   │   ├── definitions.rs   # DefinitionsList (immutable anchors)
│   │   ├── fold.rs          # Lossless geometric folding engine
│   │   ├── unfold.rs        # Lossless geometric unfolding
│   │   ├── index.rs         # DimensionalIndex — O(1) retrieval
│   │   ├── network.rs       # Cascade-aware entanglement fabric
│   │   ├── evolution.rs     # Dimensional, learning & cascade evolution
│   │   ├── gates.rs         # Dimensional gate system (Hadamard, CNOT, Phase, Oracle)
│   │   ├── search.rs        # Superposition search algorithms
│   │   ├── register.rs      # Quantum register (statevector, all standard gates, QFT)
│   │   ├── algorithms.rs    # Quantum algorithms (Shor's, Grover's, Deutsch-Jozsa, teleportation)
│   │   ├── circuit.rs       # Declarative circuit builder with ASCII visualization
│   │   ├── error_correction.rs  # Error correction codes (3-qubit, Shor, Steane)
│   │   ├── phase_estimation.rs  # Quantum phase estimation
│   │   └── variational.rs   # VQE + QAOA variational algorithms
│   ├── Cargo.toml
│   └── tests/
├── mdbfs/             # FUSE filesystem (transparent fold/unfold)
├── desktop/           # Wayland compositor (planned)
├── process/           # SuperBit process model (planned)
├── iso/               # Bootable ISO builder (planned)
└── README.md
```

## Core Concepts

### The SuperBit

The fundamental primitive of MDB — replaces the classical bit:

```
B = (σ, Ψ, W, A, G)
```

- **σ** — Binary string encoding all state information
- **Ψ** — State space (multiple possible states, held simultaneously)
- **W** — Weight vector (collapse probabilities, Σwᵢ = 1)
- **A** — Immutable anchor positions (DefinitionsList)
- **G** — Generation counter (evolution depth)

**Key property**: Collapse is a *read* operation. The binary string σ is never modified. The SuperBit remains in full superposition after collapse — solving the quantum superposition destruction problem on classical hardware.

#### Non-Destructive Superposition (what quantum hardware cannot do)

A real qubit destroys its superposition on measurement. A SuperBit does not:

| Operation | What it does | Quantum equivalent |
|-----------|-------------|-------------------|
| `peek()` | Inspect every state, weight, and dimensional address | **Impossible** — measurement collapses |
| `fork()` | Create independent clone for parallel exploration | **Impossible** — no-cloning theorem |
| `collapse_to(i)` | Choose a specific state without modifying σ | **Impossible** — collapse is random |
| `state_distances()` | Compare states dimensionally | **Impossible** — can't read pre-collapse |
| `evolve_cascade_preview()` | See what evolution *would* do | **Impossible** — evolution is irreversible |

This enables workflows that are fundamentally impossible on quantum hardware: fork a SuperBit, collapse each fork to a different state, compare outcomes dimensionally — while the original sits untouched in full superposition.

### Dimensional Addressing

Every binary string gets a unique address computed from the cascade:

| Component     | Source | Purpose |
|---------------|--------|---------|
| `n`           | Bit length | Separates strings by size |
| `d4_spacetime`| Position-weighted D4 sum | Geometric scalar coordinate |
| `d5_momentum` | FNV-1a fingerprint of D1 | Collision-free identity hash |

The address `addr(S) = (n, d4_spacetime, d5_momentum)` enables **O(1) guaranteed retrieval** — you don't search for data, you compute where it *must* be.

### Geometric Folding

Lossless, deterministic transformation of data into dimensional coordinate space:

- **fold(data)** → compact geometric representation
- **unfold(folded)** → original data, bit-for-bit identical
- SHA-256 verified at every unfold
- Supports recursive multi-depth folding
- Not compression — geometric reorganization

### Evolution

SuperBits evolve rather than execute:

1. **Dimensional evolution** — bit-flip based on D3 parity, respecting anchors
2. **Learning evolution** — probability reweighting based on observed outcomes
3. **Cascade evolution** — φ-driven golden-ratio position selection (non-periodic, visits all positions uniformly via low-discrepancy sequence)

All evolution modes have **non-destructive preview** variants that return the result without modifying the original SuperBit.

### Dimensional Gate System

MDB has its own gate system — analogous to quantum gates but operating on cascade vectors instead of complex amplitudes:

| Gate | Inputs | What it does |
|------|--------|-------------|
| `cascade_hadamard` | 1 | Split into 2^k balanced states via φ-driven position selection |
| `cascade_cnot` | 2 | Entangle: control's D4/D5 coordinates select target flips |
| `cascade_phase` | 1 | Rotate probability weights by φ-derived sine wave |
| `cascade_oracle` | 1 | Amplify weights of states matching a predicate |

Gates compose into **pipelines** (`GatePipeline`) that apply sequentially. After any gate, the full state space is inspectable via `peek()`.

### Superposition Search

Search algorithms that exploit non-destructive superposition:

- **`dimensional_search`** — Find state closest to a target dimensional address
- **`pattern_search`** — Find state closest to a target bit pattern (Hamming)
- **`fitness_search`** — Optimize any custom fitness function over the state space
- **`exhaustive_explore`** — Fork, collapse each fork to a different state, compare all outcomes

All searches leave the original SuperBit in full superposition.

### Quantum Register

Full statevector quantum register — computationally equivalent to a universal quantum computer's qubit register, plus MDB-exclusive operations:

| Gate | Type | What it does |
|------|------|-------------|
| `hadamard(k)` | Single | Superposition: \|0⟩→(|0⟩+|1⟩)/√2 |
| `pauli_x/y/z(k)` | Single | Pauli gates (NOT, Y, Z) |
| `phase_gate(k,θ)` | Single | Phase rotation e^(iθ) on \|1⟩ |
| `s_gate/t_gate(k)` | Single | S=√Z, T=π/8 gates |
| `rx/ry/rz(k,θ)` | Single | Rotation gates |
| `cnot(c,t)` | Two | Controlled-NOT |
| `cz(a,b)` | Two | Controlled-Z |
| `swap(a,b)` | Two | Swap two positions |
| `controlled_phase(c,t,θ)` | Two | Controlled phase rotation |
| `toffoli(c1,c2,t)` | Three | Doubly-controlled NOT (CCX) |
| `fredkin(c,t1,t2)` | Three | Controlled-SWAP |
| `qft(positions)` | Multi | Quantum Fourier Transform |
| `inverse_qft(positions)` | Multi | Inverse QFT |
| `grover_diffusion()` | Multi | Inversion about the mean |
| `apply_oracle(pred)` | Multi | Phase-flip matching states |

**MDB-exclusive operations** (impossible on quantum hardware):
- `peek()` — see all states without collapse
- `fork()` — clone register (violates no-cloning theorem)
- `sample_all()` — non-destructive measurement

### Quantum Algorithms

Standard algorithms proving full quantum computing equivalence:

- **Deutsch–Jozsa** — Constant-vs-balanced in 1 evaluation (vs 2^(n-1)+1 classical)
- **Grover's Search** — Find marked items in O(√N) time
- **Shor's Factoring** — Polynomial-time integer factoring via quantum period-finding + QFT
- **Quantum Teleportation** — State transfer via entanglement + classical bits

### Circuit Model

Declarative circuit builder with ASCII visualization:

```rust
let result = Circuit::new(3, "Bell + GHZ")
    .h(0)
    .cnot(0, 1)
    .cnot(1, 2)
    .barrier()
    .measure_all()
    .execute();

println!("{}", circuit.to_ascii());
//   q0: ─[H]───●───────│ ─[M*]─
//   q1: ───────X───●───│ ─[M*]─
//   q2: ───────────X───│ ─[M*]─
```

Preset circuits: `bell_pair()`, `ghz_state()`, `qft_circuit()`, `grover_iteration()`.

### Error Correction

| Code | Qubits | Corrects |
|------|--------|----------|
| Bit-flip | 3 | Single X error |
| Phase-flip | 3 | Single Z error |
| Shor's | 9 | Any single-qubit error (X, Z, or Y) |
| Steane's | 7 | Any single-qubit error (CSS code) |

MDB advantage: syndrome extraction uses `peek()` — no ancilla qubits needed.

### Quantum Phase Estimation

Estimates eigenvalue phases θ of unitary operators to arbitrary precision:
- `estimate_phase_gate(θ, bits)` — exact for binary fractions
- `estimate_eigenvalues_2x2(matrix, bits)` — general 2×2 unitaries
- `phase_estimation(...)` — fully general QPE subroutine

### Variational Algorithms

**VQE** (Variational Quantum Eigensolver):
- Pauli Hamiltonians (I, X, Y, Z tensor products)
- Ansätze: Ry-ladder, hardware-efficient
- Parameter-shift gradient rule
- Preset: `Hamiltonian::hydrogen_molecule(bond_length)`

**QAOA** (Quantum Approximate Optimization):
- Parameterised cost/mixer layers
- MaxCut solver: `maxcut_cost(edges)`
- Gradient-based parameter optimization

### Cascade Entanglement Network

All SuperBits form an interconnected fabric with **cascade-aware entanglement links**. Unlike simple BFS propagation, entanglement operates at the dimensional cascade level:

- When one SuperBit evolves, the cascade delta (change in D4 Spacetime) propagates
- Entangled partners receive φ-correlated bit flips scaled by coupling strength
- Ripple decays by golden ratio (φ-1 ≈ 0.618) at each depth level
- Changes cascade through chains: A→B→C with diminishing but deterministic effect

## Quick Start

```bash
# Run core engine tests (202 tests)
cd core && cargo test

# Build MDBFS (requires libfuse3-dev)
# Ubuntu/Debian: sudo apt install pkg-config libfuse3-dev fuse3
cd mdbfs && cargo build --release

# Mount an MDB filesystem
./target/release/mdbfs mount /mnt/mdb --store /var/lib/mdbfs --foreground

# Use it like normal — all data is dimensionally folded under the hood
echo "Hello, MDB!" > /mnt/mdb/hello.txt
cat /mnt/mdb/hello.txt   # transparently unfolded

# Check MDB metadata
getfattr -n mdb.address /mnt/mdb/hello.txt
getfattr -n mdb.fold_depth /mnt/mdb/hello.txt
```

## Roadmap

- [x] **Core engine v0.2** — Cascade-based dimensional coordinates (Fibonacci, Golden Ratio convergence, zero collisions)
- [x] **SuperBit** — Non-destructive superposition (peek, fork, selective collapse, state comparison)
- [x] **Fold/Unfold** — Lossless geometric folding with SHA-256 verification (depth 1–5 tested, up to 64KB)
- [x] **DimensionalIndex** — O(1) retrieval via cascade-derived addresses
- [x] **MDBNetwork** — Cascade-aware entanglement with φ-correlated ripple propagation
- [x] **MDBFS** — FUSE filesystem with transparent fold/unfold, xattr metadata, fsck
- [x] **Dimensional Gates** — Hadamard, CNOT, Phase, Oracle gates operating on cascade vectors
- [x] **Superposition Search** — Dimensional, pattern, fitness, and exhaustive search algorithms
- [x] **Quantum Register** — Full statevector simulator with all standard gates + QFT
- [x] **Quantum Algorithms** — Shor's factoring, Grover's search, Deutsch–Jozsa, teleportation
- [x] **Circuit Model** — Declarative circuit builder with ASCII visualization + presets
- [x] **Error Correction** — Bit-flip, phase-flip, Shor's 9-qubit, Steane's 7-qubit codes
- [x] **Phase Estimation** — QPE subroutine with arbitrary precision
- [x] **VQE/QAOA** — Variational hybrid algorithms with gradient optimization
- [ ] **CLI / REPL** — Interactive command-line interface
- [ ] **Persistence Layer** — Save/load SuperBits and entanglement graphs
- [ ] **WASM Build** — Browser-native compilation
- [ ] **D6+ exploration** — Energy dimension and beyond; what emerges at higher cascade depths?
- [ ] **MDB Desktop** — Wayland compositor / desktop environment
- [ ] **MDB Process Model** — SuperBit-based process management
- [ ] **Bootable ISO** — Linux substrate with MDB as the user-facing OS

## Version History

| Version | Description |
|---------|-------------|
| 0.1.0   | Initial implementation (independent D3/D4/D5 dimensions) |
| 0.2.0   | **Cascade rewrite** — sequential Fibonacci cascade where each dimension derives from the previous two. Matches Ryan's original theoretical design. |
| 0.2.1   | **Non-destructive superposition** — peek/fork/collapse_to/state_distances on SuperBit. φ-driven cascade evolution. 76 tests. |
| 0.3.0   | **Computing engine** — Dimensional gates, cascade-aware entanglement, superposition search. Quantum register (all standard gates + QFT). Quantum algorithms (Shor's, Grover's, Deutsch–Jozsa, teleportation). Circuit model with ASCII visualization. Error correction (bit-flip, phase-flip, Shor 9-qubit, Steane 7-qubit). Phase estimation. VQE + QAOA. *202 tests.* |

## License

MIT

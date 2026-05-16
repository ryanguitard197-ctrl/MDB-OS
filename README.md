# MDB OS — Multidimensional Binary Operating System

> *"The bit itself has changed."*

MDB redefines the atomic unit of computation from a classical bit (0 or 1) to a **SuperBit** — a binary string that exists as a geometric object in abstract higher-dimensional coordinate space. Binary data is no longer flat; it carries intrinsic dimensional properties that enable O(1) retrieval, non-destructive collapse, lossless geometric folding, and deterministic evolution.

**Invented by Ryan Guitard.**

## Architecture

```
mdb-os/
├── core/              # Rust — the canonical MDB engine
│   ├── src/
│   │   ├── lib.rs           # Crate root
│   │   ├── coordinates.rs   # D3/D4/D5 dimensional address system
│   │   ├── superbit.rs      # The SuperBit — atomic unit of MDB
│   │   ├── definitions.rs   # DefinitionsList (immutable anchors)
│   │   ├── fold.rs          # Lossless geometric folding engine
│   │   ├── unfold.rs        # Lossless geometric unfolding
│   │   ├── index.rs         # DimensionalIndex — O(1) retrieval
│   │   ├── network.rs       # MDBNetwork & EntangledMemory
│   │   └── evolution.rs     # Dimensional & learning evolution
│   ├── Cargo.toml
│   └── tests/
├── paper/             # Formal papers & documentation
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

### Dimensional Coordinates

Every binary string exists in at least 5 natural dimensions:

| Dimension | Name | Definition |
|-----------|------|------------|
| D1 | Value | The actual bit content |
| D2 | Space | Positional relationships |
| D3 | Time | String length `\|S\| = n` |
| D4 | Density | Ratio of 1s to 0s |
| D5 | Gravity | Golden Ratio weighted signature `Σ(S[i]·φ·i) mod 1` |

The dimensional address `addr(S) = (D3, D4, D5)` enables **O(1) guaranteed retrieval** — you don't search for data, you compute where it *must* be.

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

### MDBNetwork

All SuperBits form an interconnected fabric with entanglement links. Changes propagate through the network via ripple operations — non-local state sharing on classical hardware.

## Quick Start

```bash
cd core
cargo test
```

## Test Results

```
running 57 tests
... all passing ...
test result: ok. 57 passed; 0 failed; 0 ignored
```

## Roadmap

- [x] **Core engine** — SuperBit, coordinates, fold/unfold, index, network, evolution
- [ ] **MDBFS** — FUSE filesystem with fold/unfold transparent storage
- [ ] **MDB Desktop** — Wayland compositor / desktop environment
- [ ] **MDB Process Model** — SuperBit-based process management
- [ ] **Bootable ISO** — Linux substrate with MDB as the user-facing OS

## License

MIT

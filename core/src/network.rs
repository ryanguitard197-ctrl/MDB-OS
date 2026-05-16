//! # MDBNetwork & EntangledMemory
//!
//! The MDBNetwork facilitates interconnected SuperBits, forming a network
//! where each node is a SuperBit and each edge represents a relational
//! gravity link. This structure is the unified fabric for scheduling,
//! IPC, memory, and execution in MDB OS.
//!
//! **EntangledMemory** implements ripple-style propagation:
//! ```text
//! ripple(Sᵢ, Sⱼ) = entangle(Sᵢ, Sⱼ)
//! ```
//!
//! When a SuperBit changes, the change propagates to its entangled partners
//! through the network, enabling non-local state sharing analogous to
//! quantum entanglement — on classical hardware.

use crate::evolution;
use crate::superbit::SuperBit;
use std::collections::{HashMap, HashSet, VecDeque};

/// Unique identifier for a node in the MDB network.
pub type NodeId = u64;

/// An edge (entanglement link) between two SuperBits.
#[derive(Debug, Clone)]
pub struct EntanglementLink {
    /// Source node.
    pub from: NodeId,
    /// Target node.
    pub to: NodeId,
    /// Gravity weight of this link (strength of entanglement).
    pub gravity: f64,
}

/// A node in the MDB network — wraps a SuperBit with network metadata.
#[derive(Debug, Clone)]
pub struct NetworkNode {
    /// Unique node identifier.
    pub id: NodeId,
    /// The SuperBit at this node.
    pub superbit: SuperBit,
    /// IDs of entangled neighbor nodes.
    pub neighbors: HashSet<NodeId>,
}

/// The MDB Network — a unified fabric of interconnected SuperBits.
///
/// All OS primitives (processes, files, memory, IPC) are nodes in this
/// network. The network evolves as a whole through evolution steps,
/// with changes rippling through entanglement links.
#[derive(Debug)]
pub struct MDBNetwork {
    /// All nodes in the network.
    nodes: HashMap<NodeId, NetworkNode>,
    /// All entanglement links.
    links: Vec<EntanglementLink>,
    /// Next available node ID.
    next_id: NodeId,
    /// Global evolution tick counter.
    pub tick: u64,
}

impl MDBNetwork {
    /// Create an empty MDB network.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
            next_id: 1,
            tick: 0,
        }
    }

    /// Add a SuperBit to the network. Returns its node ID.
    pub fn add_node(&mut self, superbit: SuperBit) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        let node = NetworkNode {
            id,
            superbit,
            neighbors: HashSet::new(),
        };
        self.nodes.insert(id, node);
        id
    }

    /// Remove a node from the network.
    pub fn remove_node(&mut self, id: NodeId) -> Option<SuperBit> {
        if let Some(node) = self.nodes.remove(&id) {
            // Remove all links involving this node
            self.links.retain(|l| l.from != id && l.to != id);
            // Remove from neighbor sets
            for &neighbor_id in &node.neighbors {
                if let Some(neighbor) = self.nodes.get_mut(&neighbor_id) {
                    neighbor.neighbors.remove(&id);
                }
            }
            Some(node.superbit)
        } else {
            None
        }
    }

    /// Entangle two SuperBits — create a bidirectional gravity link.
    ///
    /// This is the `ripple(Sᵢ, Sⱼ) = entangle(Sᵢ, Sⱼ)` operation.
    pub fn entangle(&mut self, a: NodeId, b: NodeId, gravity: f64) -> bool {
        if !self.nodes.contains_key(&a) || !self.nodes.contains_key(&b) || a == b {
            return false;
        }

        // Add bidirectional link
        self.links.push(EntanglementLink {
            from: a,
            to: b,
            gravity,
        });

        self.nodes.get_mut(&a).unwrap().neighbors.insert(b);
        self.nodes.get_mut(&b).unwrap().neighbors.insert(a);

        true
    }

    /// Get a reference to a node's SuperBit.
    pub fn get(&self, id: NodeId) -> Option<&SuperBit> {
        self.nodes.get(&id).map(|n| &n.superbit)
    }

    /// Get a mutable reference to a node's SuperBit.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut SuperBit> {
        self.nodes.get_mut(&id).map(|n| &mut n.superbit)
    }

    /// Get the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get the number of entanglement links.
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Get all neighbor IDs for a node.
    pub fn neighbors(&self, id: NodeId) -> Option<&HashSet<NodeId>> {
        self.nodes.get(&id).map(|n| &n.neighbors)
    }

    /// Propagate a change from a source node through the network.
    ///
    /// Uses BFS to ripple dimensional evolution through entangled nodes
    /// up to the specified depth. This implements the `MDBNetwork.propagate(depth)`
    /// operation from the spec.
    pub fn propagate(&mut self, source: NodeId, depth: u32) -> Vec<NodeId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut evolved = Vec::new();

        visited.insert(source);
        queue.push_back((source, 0u32));

        while let Some((current_id, current_depth)) = queue.pop_front() {
            if current_depth > 0 {
                // Evolve this node (the source already initiated the change)
                if let Some(node) = self.nodes.get_mut(&current_id) {
                    evolution::evolve_dimensional(&mut node.superbit);
                    evolved.push(current_id);
                }
            }

            if current_depth < depth {
                // Get neighbors to visit
                let neighbor_ids: Vec<NodeId> = self
                    .nodes
                    .get(&current_id)
                    .map(|n| n.neighbors.iter().copied().collect())
                    .unwrap_or_default();

                for neighbor_id in neighbor_ids {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id);
                        queue.push_back((neighbor_id, current_depth + 1));
                    }
                }
            }
        }

        evolved
    }

    /// Evolve the entire network by one tick.
    ///
    /// Every node undergoes dimensional evolution. This is the
    /// "eternal evolution loop" from the MDB OS spec.
    pub fn evolve_all(&mut self) {
        let ids: Vec<NodeId> = self.nodes.keys().copied().collect();
        for id in ids {
            if let Some(node) = self.nodes.get_mut(&id) {
                evolution::evolve_dimensional(&mut node.superbit);
            }
        }
        self.tick += 1;
    }

    /// Iterate over all node IDs.
    pub fn node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.nodes.keys()
    }
}

impl Default for MDBNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sb(bits: Vec<u8>) -> SuperBit {
        SuperBit::from_bits(bits)
    }

    #[test]
    fn test_add_and_get_nodes() {
        let mut net = MDBNetwork::new();
        let id1 = net.add_node(make_sb(vec![1, 0, 1]));
        let id2 = net.add_node(make_sb(vec![0, 1, 0, 1]));

        assert_eq!(net.node_count(), 2);
        assert_eq!(net.get(id1).unwrap().bit_length(), 3);
        assert_eq!(net.get(id2).unwrap().bit_length(), 4);
    }

    #[test]
    fn test_entanglement() {
        let mut net = MDBNetwork::new();
        let id1 = net.add_node(make_sb(vec![1, 0, 1]));
        let id2 = net.add_node(make_sb(vec![0, 1, 0]));

        assert!(net.entangle(id1, id2, 0.5));
        assert_eq!(net.link_count(), 1);
        assert!(net.neighbors(id1).unwrap().contains(&id2));
        assert!(net.neighbors(id2).unwrap().contains(&id1));
    }

    #[test]
    fn test_entangle_invalid() {
        let mut net = MDBNetwork::new();
        let id1 = net.add_node(make_sb(vec![1]));
        assert!(!net.entangle(id1, id1, 0.5)); // self-link
        assert!(!net.entangle(id1, 999, 0.5)); // non-existent
    }

    #[test]
    fn test_remove_node() {
        let mut net = MDBNetwork::new();
        let id1 = net.add_node(make_sb(vec![1, 0]));
        let id2 = net.add_node(make_sb(vec![0, 1]));
        net.entangle(id1, id2, 0.5);

        net.remove_node(id1);
        assert_eq!(net.node_count(), 1);
        assert!(net.get(id1).is_none());
        // id2 should no longer have id1 as neighbor
        assert!(!net.neighbors(id2).unwrap().contains(&id1));
    }

    #[test]
    fn test_propagate() {
        let mut net = MDBNetwork::new();
        // Create a chain: A - B - C
        let a = net.add_node(make_sb(vec![1, 0, 1, 0, 1]));
        let b = net.add_node(make_sb(vec![0, 1, 0, 1, 0]));
        let c = net.add_node(make_sb(vec![1, 1, 0, 0, 1]));

        net.entangle(a, b, 1.0);
        net.entangle(b, c, 1.0);

        // Propagate from A with depth 1 → should evolve B only
        let evolved = net.propagate(a, 1);
        assert!(evolved.contains(&b));
        assert!(!evolved.contains(&c));
        assert_eq!(net.get(b).unwrap().generation, 1);
        assert_eq!(net.get(c).unwrap().generation, 0);

        // Propagate from A with depth 2 → should evolve B and C
        let evolved = net.propagate(a, 2);
        assert!(evolved.contains(&b));
        assert!(evolved.contains(&c));
    }

    #[test]
    fn test_evolve_all() {
        let mut net = MDBNetwork::new();
        net.add_node(make_sb(vec![1, 0, 1]));
        net.add_node(make_sb(vec![0, 1, 0, 1]));
        net.add_node(make_sb(vec![1, 1, 0]));

        net.evolve_all();
        assert_eq!(net.tick, 1);

        // All nodes should have generation 1
        for id in net.node_ids() {
            assert_eq!(net.get(*id).unwrap().generation, 1);
        }
    }
}

use std::collections::VecDeque;
use crate::{DefaultDerivationHeuristic,DerivationHeuristic,DerivationTree,DerivationTerm};

/// Responsible for exploring the search space of assignments for a
/// given term.
pub struct Derivation<T:DerivationTerm,H:DerivationHeuristic<T> = DefaultDerivationHeuristic<T>> {
    tree: DerivationTree<T>,
    /// Worklist of items remaining to be explored.  The first field
    /// of each element identifies a term within the derivation tree,
    /// whilst the second field identifies the current assignment used
    /// to derive that term.
    worklist: VecDeque<(usize,Vec<usize>)>,
    /// Heuristic responsible for derivationting individual terms.
    heuristic: H
}

impl<T:DerivationTerm,H:Default+DerivationHeuristic<T>> Derivation<T,H> {
    pub fn new(mut term: T) -> Self {
        let n = term.domain();
        // Construct initial assignment
        let mut assignments = vec![0; n];
        // Prevent unused variables from taking part
        for i in 0..n {
            if term.num_uses(i) == 0 {
                // Since this is not a free variable within the term,
                // there is no point exploring possible assignments
                // for it.  Therefore, we just give the maximum
                // possible assignment as an indicator.
                assignments[i] = usize::MAX;
            }
        }
        // Construct proof
        let tree = DerivationTree::new(term);
        // Construct worklist
        let worklist = vec![(0,assignments)];
        // Construct heuristic
        let heuristic = H::default();
        // Begin!
        Derivation{tree, heuristic, worklist: worklist.into()}
    }
}

impl<T:DerivationTerm,H:DerivationHeuristic<T>> Derivation<T,H> {
    /// Return current size of the search.  Observe that, when this is
    /// zero, the search is complete.
    pub fn size(&self) -> usize {
        self.worklist.len()
    }

    /// Continue searching along the current branch.  This assumes
    /// there is at least one item remaining in the worklist.  We
    /// apply the query function to check whether a terminal node is
    /// reached (and, if so, whether we found what we're looking for).
    /// If so, that is returned.  Otherwise, we continue derivationting.
    pub fn derivation<F>(&mut self, query: F) -> Option<usize>
    where F:Copy+Fn(usize,&[usize],&DerivationTree<T>)->Option<bool> {
        // Pull off the next term
        let (next,assignments) = self.worklist.pop_front().unwrap();
        // Run the derivation functions
        match query(next,&assignments,&self.tree) {
            Some(true) => {
                Some(next)
            }
            Some(false) => {
                // Indicates a terminal node which doesn't match the
                // query.  Therefore, it is simply dropped.
                None
            },
            None => {
                // Continue derivationting!
                todo!()
            }
        }
    }
}
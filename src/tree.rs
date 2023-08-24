/// A split tree provides a ledger recording intermediate terms
/// encountered during a derivation.  Each term in the tree is derived
/// from a single parent using the split operator.  A key property is
/// that we can determine whether or not a given term has been
/// encountered previously.
pub struct DerivationTree<T:PartialEq> {
    /// Terms themselves
    terms: Vec<T>,
    /// Parent information for each term, where `MAX` indicates the
    /// root.  Each element identifies the index in `terms` of the
    /// parent.
    parents: Vec<usize>
}

impl<T:PartialEq> DerivationTree<T> {
    /// Construct
    pub fn new(root: T) -> Self {
        DerivationTree {
            terms: vec![root],
            parents: vec![usize::MAX]
        }
    }
    /// Get the number of nodes currently in this tree.
    pub fn len(&self) -> usize {
        self.terms.len()
    }
    /// Get the term at the `ith` index of this split tree.
    pub fn get(&self, index: usize) -> &T {
        &self.terms[index]
    }
    /// Push a new term derived from a given parent onto the tree.
    pub fn push(&mut self, term: T, parent: usize) -> usize {
        let index = self.terms.len();
        self.terms.push(term);
        self.parents.push(parent);
        index
    }
    /// Attempt to determine whether or not the term at a given index
    /// is a duplicate of one of its ancestors.  In principle, this
    /// search can be pruned in various ways.  For example, when a
    /// given parent (and, hence, its ancestors) are "too small" to
    /// match the term in question.
    pub fn is_duplicate(&self, mut index: usize) -> bool {
        let term = &self.terms[index];
        index = self.parents[index];
        //
        while index != usize::MAX {
            if &self.terms[index] == term {
                return true;
            }
            index = self.parents[index];
        }
        false
    }
}

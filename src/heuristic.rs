use std::marker::PhantomData;
use std::ops::Range;
use crate::DerivationTree;

/// A derivation heuristic is responsible for deriving a given term in
/// the derivation tree.  That means it must decide what aspect of the
/// term to derivation on (e.g. the number of occurences of a
/// particular variable) and then apply this to the derivation tree.
pub trait DerivationHeuristic<T:PartialEq> {
    /// Apply this heuristic to a give term (identified by `index)` in
    /// the derivation `tree`.  This returns a range of one or more indices
    /// identifying children added to the tree.
    fn apply(&self,index: usize, tree: &DerivationTree<T>) -> Range<usize>;
}

pub type DefaultDerivationHeuristic<T> = AssignmentHeuristic<T>;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct AssignmentHeuristic<T:PartialEq> {
    dummy: PhantomData<T>
}

impl<T:PartialEq> DerivationHeuristic<T> for AssignmentHeuristic<T> {
    fn apply(&self, index: usize, tree: &DerivationTree<T>) -> Range<usize> {
        todo!()
    }
}

impl<T:PartialEq> Default for AssignmentHeuristic<T> {
    fn default() -> Self {
        Self{dummy: PhantomData}
    }
}

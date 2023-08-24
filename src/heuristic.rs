use std::marker::PhantomData;
use std::ops::Range;
use crate::{DerivationTree,DerivationTerm};

/// A derivation heuristic is responsible for deriving a given term in
/// the derivation tree.  That means it must decide what aspect of the
/// term to derivation on (e.g. the number of occurences of a
/// particular variable) and then apply this to the derivation tree.
pub trait DerivationHeuristic<T:PartialEq> {
    /// Apply this heuristic to a give term (identified by `index)` in
    /// the derivation `tree`.  This returns a range of one or more indices
    /// identifying children added to the tree.
    fn apply(&self,index: usize, tree: &mut DerivationTree<T>) -> Range<usize>;
}

pub type DefaultDerivationHeuristic<T> = SimplestHeuristic<T>;

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct SimplestHeuristic<T:PartialEq> {
    dummy: PhantomData<T>
}

impl<T:PartialEq+DerivationTerm> DerivationHeuristic<T> for SimplestHeuristic<T> {
    /// Split a given term by finding the first variable which is
    /// actively used within.
    fn apply(&self, index: usize, tree: &mut DerivationTree<T>) -> Range<usize> {
        let term : &T = tree.get(index);
        //
        for i in 0..term.domain() {
            if term.num_uses(i) != 0 {
                let index = tree.len();
                // Split on this variable
                let (l,r) = term.split(i);
                // Push items onto tree
                tree.push(l,i);
                tree.push(r,i);
                // Done
                return index..index+2;
            }
        }
        //
        0..0
    }
}

impl<T:PartialEq> Default for SimplestHeuristic<T> {
    fn default() -> Self {
        Self{dummy: PhantomData}
    }
}

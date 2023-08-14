use crate::{Derivation,DerivationHeuristic,DerivationTree,DerivationTerm};

pub struct DerivationIterator<T:DerivationTerm,H:DerivationHeuristic<T>,F>
// Query function
where F:Copy+Fn(usize,&[usize],&DerivationTree<T>)->Option<bool> {
    /// The queryDerivation function
    query: F,
    /// Current Derivation
    derivation: Derivation<T,H>
}

impl<T:DerivationTerm,H:DerivationHeuristic<T>,F> DerivationIterator<T,H,F>
where F:Copy+Fn(usize,&[usize],&DerivationTree<T>)->Option<bool> 
{

    // pub fn new(mut term: bit::Term, context: &'a [bit::Function], query: F) -> Self {
    //     let splitter = BitSplitter::new(term,context);
    //     BitInducter{query,splitter}
    // }
}

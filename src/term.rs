/// Abstracts the notion of a term which can be used within a split
/// tree.
pub trait DerivationTerm : PartialEq+Sized {
    /// Determine the domain (`n`) of variables for this term.  It
    /// assumed that variables are always numbered consecutively
    /// starting from `0..n`.  Every variable used within this term
    /// must be in the domain.  But, not every variable in the domain
    /// is necessarily used within this term.
    fn domain(&self) -> usize;

    /// Determine the number of uses of a given variable within this
    /// term.  This is useful, for example, to determine which
    /// variables are actually used within this term and which are
    /// not.
    fn num_uses(&self, var: usize) -> usize;

    /// Split this term in two by a given variable.
    fn split(&self, var: usize) -> (Self,Self);
}

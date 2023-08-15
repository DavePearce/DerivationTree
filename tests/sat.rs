use derivation_tree::{DefaultDerivationHeuristic,Derivation,DerivationTerm};

/// Represents a sequence of one or more variables disjuncted
/// together.  Each variable can be negated or not.  For example, we
/// can represent `a || !b`.  
#[derive(Clone,Debug,PartialEq)]
struct Clause {
    /// Identifies which are used in the clause, and their status
    /// where negative values represent negated variables.
    terms: Vec<isize>
}

/// Represents a formula in conjuncted normal form.  That is, a set of
/// zero or more _clauses_ which are conjuncted together.  For
/// example, `(a||b) && (!b||c)` is a formula composed of two clauses
/// (i.e. `a||b` and `!b||c`).
#[derive(Clone,Debug,PartialEq)]
struct Formula {
    clauses: Vec<Clause>
}

impl Formula {
    // Use the derivation tree to determine whether or not a formula
    // is satisfiable.
    fn sat(self) -> bool {
        let dt = Derivation::new(Term::Node(self), |t:&Term| t.sat_query());
        // If a single sat instance is found, then we know its sat.
        for (_,_) in dt { return true; }
        // Otherwise, its unsat.
	return false;
    }
}

impl From<Vec<Vec<isize>>> for Formula {
    fn from(clauses: Vec<Vec<isize>>) -> Self {
        let mut res = Vec::new();
        for c in clauses {
            res.push(Clause{terms: c});
        }
        Self{clauses: res}
    }
}

// =============================================================================
// Derivation
// =============================================================================

#[derive(Clone,Debug,PartialEq)]
enum Term {
    // Internal tree node, representing an ongoing derivation.
    Node(Formula),
    // Leaf node of the tree, representing the end of a branch which
    // reduced to either `true` or `false`.
    Leaf(bool)
}

impl Term {
    /// Query looking for satisfiable instances.  Thus, an term which
    /// has reduced to `true` is a match.
    fn sat_query(&self) -> Option<bool> {
        match self {
            Term::Node(_) => None,
            Term::Leaf(b) => Some(*b)
        }
    }
    /// Query looking for unsatisfiable instances.  Thus, an term
    /// which has reduced to `false` is a match.
    fn unsat_query(&self) -> Option<bool> {
        match self {
            Term::Node(_) => None,
            Term::Leaf(b) => Some(!*b)
        }
    }    
}

impl DerivationTerm for Term {
    fn domain(&self) -> usize {
        todo!()
    }
    fn num_uses(&self, var: usize) -> usize {
        todo!()
    }
    fn split(&self, var: usize) -> (Self,Self) {
        todo!()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[test]
fn test_01() {
    // a||b
    let f1 = Formula::from(vec![vec![0,1]]);
    //
    assert!(f1.sat());
}

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
    fn sat(&self) -> bool {
        todo!()
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
// Tests
// =============================================================================

#[test]
fn test_01() {
    // a||b
    let f1 = Formula::from(vec![vec![0,1]]);
    //
    assert!(f1.sat());
}

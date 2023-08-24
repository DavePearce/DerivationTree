use std::cmp;
use derivation_tree::{DefaultDerivationHeuristic,Derivation,DerivationTerm};

#[derive(Clone,Copy,Debug,PartialEq)]
struct Term {
    index: usize
}

impl Term {
    const fn new(var: usize) -> Self { Self{index: var * 2} }
    /// Get the variable represented by this term.
    fn var(self) -> usize { self.index / 2 }
    /// Negate this term.
    fn negate(mut self) -> Self {
        if (self.index%2) == 0 {
            self.index += 1;
        } else {
            self.index -= 1;
        }
        self
    }
}

/// Represents a sequence of one or more variables disjuncted
/// together.  Each variable can be negated or not.  For example, we
/// can represent `a || !b`.  
#[derive(Clone,Debug,PartialEq)]
struct Clause {
    /// Identifies which are used in the clause, and their status
    /// where negative values represent negated variables.
    terms: Vec<Term>
}

const CLAUSE_FALSE : Clause = Clause{terms: Vec::new()};

impl Clause {    
    fn domain(&self) -> usize {
        let mut d = 0;
        for t in &self.terms { d = cmp::max(t.var()+1,d); }
        d
    }
    fn num_uses(&self, var: usize) -> usize {
        let mut r = 0;
        for t in &self.terms {
            if t.var() == var { r += 1; }
        }
        r
    }
    fn substitute(&self, var: Term) -> Option<Self> {
        // Check whether clause evaluates to true
        if self.terms.contains(&var) { return None; }
        let mut terms = self.terms.clone();
        // Remove negated variable var from clause (as this now
        // evaluates to false).
        let nvar = var.negate();
        terms.retain(|v| *v != nvar);
        // Done
        Some(Self{terms})
    }
}

/// Represents a formula in conjuncted normal form.  That is, a set of
/// zero or more _clauses_ which are conjuncted together.  For
/// example, `(a||b) && (!b||c)` is a formula composed of two clauses
/// (i.e. `a||b` and `!b||c`).
#[derive(Clone,Debug,PartialEq)]
struct Formula { clauses: Vec<Clause> }

impl Formula {
    /// Check whether the formula is `true` or `false`.  Observe that
    /// it maybe neither at this stage (in which case `None` is
    /// returned).
    fn eval(&self) -> Option<bool> {
        if self.clauses.len() == 0 {
            Some(true)
        } else if self.clauses.len() == 1 && self.clauses[0] == CLAUSE_FALSE {
            Some(false)
        } else {
            None
        }
    }
    fn domain(&self) -> usize {
        let mut d = 0;
        for c in &self.clauses { d = cmp::max(c.domain(),d); }
        d
    }
    fn num_uses(&self, var: usize) -> usize {
        let mut r = 0;
        for c in &self.clauses { r += c.num_uses(var); }
        r
    }
    fn substitute(&self, var: Term) -> Self {
        let mut clauses = Vec::new();
        //
        for c in &self.clauses {
            match c.substitute(var) {
                Some(n) if n == CLAUSE_FALSE => {
                    // Indicates subsitution reduced clause to false.
                    // Therefore, entire formula is false.
                    return Self{clauses:vec![CLAUSE_FALSE]};
                }
                Some(n) => {
                    // Indicates substitution did not yet reduce
                    // clause to terminal, therefore it continues.
                    clauses.push(n);
                }
                None => {
                    // Indicates substitution reduced clause to true.
                    // Therefore, it can be dropped entirely.
                }
            }            
        }
        //
        Self{clauses}
    }
    // Use the derivation tree to determine whether or not a formula
    // is satisfiable.
    fn sat(self) -> bool {
        let dt = Derivation::new(self, |f:&Formula| f.eval());
        // If a single sat instance is found, then we know its sat.
        for _ in dt { return true; }
        // Otherwise, its unsat.
	return false;
    }
}

impl From<Vec<Vec<Term>>> for Formula {
    fn from(clauses: Vec<Vec<Term>>) -> Self {
        let mut res = Vec::new();
        for c in clauses {
            res.push(Clause{terms: c});
        }
        Self{clauses: res}
    }
}

impl DerivationTerm for Formula {
    fn domain(&self) -> usize {
        let mut d = 0;
        for c in &self.clauses { d = cmp::max(c.domain(),d); }
        d        
    }
    fn num_uses(&self, var: usize) -> usize {
        let mut r = 0;
        for c in &self.clauses { r += c.num_uses(var); }
        r        
    }
    fn split(&self, var: usize) -> (Self,Self) {
        let term = Term::new(var);
        // positive
        let l = self.substitute(term);
        // negative
        let r = self.substitute(term.negate());        
        // Done
        (l,r)
    }
}

// =============================================================================
// Tests
// =============================================================================

const A : Term = Term::new(0);
const B : Term = Term::new(1);
const C : Term = Term::new(2);
const D : Term = Term::new(3);

#[test]
fn test_01() {
    // a || b
    let f1 = Formula::from(vec![vec![A,B]]);
    //
    assert!(f1.sat());
}

#[test]
fn test_02() {
    // a || !a
    let f1 = Formula::from(vec![vec![A,A.negate()]]);
    //
    assert!(f1.sat());
}

#[test]
fn test_03() {
    // a && b
    let f1 = Formula::from(vec![vec![A],vec![B]]);
    //
    assert!(f1.sat());
}

#[test]
fn test_04() {
    // a && !a    
    let f1 = Formula::from(vec![vec![A],vec![A.negate()]]);
    //
    assert!(!f1.sat());
}

#[test]
fn test_05() {
    // !a||b && !a||!b    
    let f1 = Formula::from(vec![vec![A.negate(),B],vec![A.negate(),B.negate()]]);
    //
    assert!(f1.sat());
}

#[test]
fn test_06() {
    // a && !a||b && !a||!b    
    let f1 = Formula::from(vec![vec![A],vec![A.negate(),B],vec![A.negate(),B.negate()]]);
    //
    assert!(!f1.sat());
}

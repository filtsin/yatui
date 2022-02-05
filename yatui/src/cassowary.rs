// Reusing from `cassowary` crate

pub use cassowary::{
    strength::{MEDIUM, REQUIRED, STRONG, WEAK},
    Constraint, Expression, Variable,
    WeightedRelation::{EQ, GE, LE},
};

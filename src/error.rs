use cassowary::AddConstraintError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Bad layout equation: {0}")]
    LayoutEquation(#[from] LayoutEquationProblem),
    #[error("Buffer error: {0}")]
    Buffer(String),
    #[error("Application is not initialized")]
    AppNotInit,
}

#[derive(Error, Debug)]
pub enum LayoutEquationProblem {
    #[error("Duplicate constraint")]
    Duplicate,
    #[error("Unsatisfiable constraint")]
    Unsatisfiable,
    #[error("Internal problem: {0}")]
    Internal(&'static str),
}

impl From<AddConstraintError> for LayoutEquationProblem {
    fn from(v: AddConstraintError) -> Self {
        match v {
            AddConstraintError::DuplicateConstraint => Self::Duplicate,
            AddConstraintError::UnsatisfiableConstraint => Self::Unsatisfiable,
            AddConstraintError::InternalSolverError(v) => Self::Internal(v),
        }
    }
}

impl From<AddConstraintError> for Error {
    fn from(v: AddConstraintError) -> Self {
        Self::LayoutEquation(LayoutEquationProblem::from(v))
    }
}

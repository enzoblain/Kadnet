use crate::routing::id::IdError;

pub enum NodeError {
    IdError(IdError),
}

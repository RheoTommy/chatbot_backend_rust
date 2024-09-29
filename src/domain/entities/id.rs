use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Id<T> {
    id: Uuid,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> Uuid {
        self.id
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

use id_arena::{Arena, Id};

use crate::tensor::TypedShape;

pub type ValueId = Id<Value>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value {
    pub name: Option<String>,
    pub shape: Option<TypedShape>, // TODO: Support dynamic shape.
}

#[derive(Debug, Default, Clone)]
pub struct ValueArena(Arena<Value>);

impl ValueArena {
    pub fn new_val(&mut self) -> ValueId {
        self.0.alloc(Value {
            name: None,
            shape: None,
        })
    }

    pub fn new_val_named(&mut self, name: impl Into<String>) -> ValueId {
        self.0.alloc(Value {
            name: Some(name.into()),
            shape: None,
        })
    }

    pub fn new_val_named_and_shaped(
        &mut self,
        name: impl Into<String>,
        shape: impl Into<TypedShape>,
    ) -> ValueId {
        self.0.alloc(Value {
            name: Some(name.into()),
            shape: Some(shape.into()),
        })
    }

    pub fn inner(&self) -> &Arena<Value> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Arena<Value> {
        &mut self.0
    }
}

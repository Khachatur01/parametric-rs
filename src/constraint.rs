use crate::param::{Param, ParamId};
use dyn_clone::DynClone;
use std::collections::HashMap;

pub trait Constraint: DynClone {}
impl Clone for Box<dyn Constraint> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}

#[derive(Clone)]
pub struct Constraints {
    constraints: Vec<Box<dyn Constraint>>,
}

impl Constraints {
    pub fn empty() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn solve(&self, params: HashMap<ParamId, Param>) {

    }
}

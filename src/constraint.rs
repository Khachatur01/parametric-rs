use crate::param::{Param, ParamId};
use std::collections::HashMap;

pub trait Constraint {}

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

    pub fn solve(&self, params: HashMap<ParamId, Box<dyn Param>>) {

    }
}

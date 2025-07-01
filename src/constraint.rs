use std::collections::HashMap;
use crate::param::{Param, ParamId};

pub enum Constraint {}

pub struct Constraints {
    pub constraints: Vec<Constraint>,
}

impl Constraints {
    pub fn solve(&self, params: HashMap<ParamId, Box<dyn Param>>) {
        
    }
}

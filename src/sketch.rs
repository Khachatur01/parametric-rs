use crate::constraint::{Constraint, Constraints};
use crate::entity::{Entity, EntityConversionError, EntityId};
use crate::param::{Param, ParamId};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

static SKETCH_ID: AtomicUsize = AtomicUsize::new(0);

pub struct SketchId(usize);
impl SketchId {
    pub fn generate() -> Self {
        let id: usize = SKETCH_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        SketchId(id)
    }
}


#[derive(Clone)]
pub struct Sketch {
    params: HashMap<ParamId, Box<dyn Param>>,
    entities: HashMap<EntityId, Box<dyn Entity>>,
    constraints: Constraints,
}
impl Sketch {
    pub fn empty() -> Self {
        Self {
            params: HashMap::new(),
            entities: HashMap::new(),
            constraints: Constraints::empty()
        }
    }

    pub fn params(&self) -> &HashMap<ParamId, Box<dyn Param>> {
        &self.params
    }
    pub fn entities(&self) -> &HashMap<EntityId, Box<dyn Entity>> {
        &self.entities
    }
    pub fn constraints(&self) -> &Constraints {
        &self.constraints
    }

    pub fn add_param(&mut self, param: impl Param + 'static) -> ParamId {
        let param_id = ParamId::generate();

        self.params.insert(param_id, Box::new(param));

        param_id
    }

    pub fn add_entity(&mut self, entity: impl Entity + 'static) -> EntityId {
        let entity_id = EntityId::generate();

        self.entities.insert(entity_id, Box::new(entity));

        entity_id
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) {
        self.constraints.add_constraint(constraint)
    }
}


#[derive(Debug)]
pub enum SketchConversionError {
    EntityDoesNotFound(EntityId),
    EntityConversionError(EntityConversionError),
}

pub trait SketchConverter<T> {
    fn try_into(&self, sketch: &Sketch) -> Result<Vec<T>, SketchConversionError>;
    fn into(&self, sketch: &Sketch) -> Vec<T>;

    fn from(&self, inputs: &[T]) -> Sketch;
}

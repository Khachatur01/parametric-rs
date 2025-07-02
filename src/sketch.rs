use crate::constraint::{Constraint, Constraints};
use crate::entity::{Entity, EntityId};
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
    params: HashMap<ParamId, Param>,
    entities: HashMap<EntityId, Entity>,
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

    pub fn params(&self) -> &HashMap<ParamId, Param> {
        &self.params
    }
    pub fn entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }
    pub fn constraints(&self) -> &Constraints {
        &self.constraints
    }

    pub fn add_param(&mut self, param: Param) -> ParamId {
        let param_id = ParamId::generate();

        self.params.insert(param_id, param);

        param_id
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        let entity_id = EntityId::generate();

        self.entities.insert(entity_id, entity);

        entity_id
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) {
        self.constraints.add_constraint(constraint)
    }
}

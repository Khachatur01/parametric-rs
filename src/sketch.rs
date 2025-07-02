use crate::constraint::Constraint;
use crate::entity::{Entity, EntityId};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

static SKETCH_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct SketchId(usize);
impl SketchId {
    pub fn generate() -> Self {
        let id: usize = SKETCH_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        SketchId(id)
    }
}


#[derive(Clone)]
pub struct Sketch {
    entities: HashMap<EntityId, Entity>,
    constraints: Vec<Constraint>,
}

impl Sketch {
    pub fn empty() -> Self {
        Self {
            entities: HashMap::new(),
            constraints: Vec::new()
        }
    }

    pub fn entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }
    pub fn constraints(&self) -> &Vec<Constraint> {
        &self.constraints
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        let entity_id = EntityId::generate();

        self.entities.insert(entity_id, entity);

        entity_id
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint)
    }
}

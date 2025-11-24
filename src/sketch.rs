use crate::constraint::Constraint;
use crate::entity::EntitySet;
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
    entity_set: EntitySet,
    constraints: Vec<Constraint>,
}

impl Sketch {
    pub fn empty() -> Self {
        Self {
            entity_set: EntitySet::default(),
            constraints: Vec::new()
        }
    }

    pub fn entity_mut(&self) -> &EntitySet {
        &self.entity_set
    }
    pub fn constraints(&self) -> &Vec<Constraint> {
        &self.constraints
    }

    // pub fn add_entity(&mut self, entity: Entity) -> EntityId {
    //     let entity_id = EntityId::generate();
    //
    //     self.entity_set.insert(entity_id, entity);
    //
    //     entity_id
    // }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint)
    }
}

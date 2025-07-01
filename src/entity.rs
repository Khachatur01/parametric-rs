use crate::sketch::Sketch;
use std::any::Any;
use std::sync::atomic::AtomicUsize;

static ENTITY_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct EntityId(usize);
impl EntityId {
    pub fn generate() -> Self {
        let id: usize = ENTITY_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        EntityId(id)
    }
}


pub trait Entity: Any {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub enum EntityConversionError {
    InvalidEntity,
}

pub trait EntityConverter<R> {
    fn try_convert(&self, sketch: &Sketch, entity: &dyn Entity) -> Result<R, EntityConversionError>;
}

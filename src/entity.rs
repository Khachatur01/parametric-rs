use crate::param::ParamId;
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

#[derive(PartialEq, Clone, Debug)]
pub enum Entity {
    Point(PointEntity),
    Segment(SegmentEntity),
    Circle(CircleEntity),
    Mesh(MeshEntity),
}

#[derive(PartialEq, Clone, Debug)]
pub struct PointEntity {
    pub x: ParamId,
    pub y: ParamId,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SegmentEntity {
    pub start: PointEntity,
    pub end: PointEntity,
}

#[derive(PartialEq, Clone, Debug)]
pub struct CircleEntity {
    pub center: PointEntity,
    pub radius: ParamId,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MeshEntity {
    pub edges: Vec<SegmentEntity>,
}

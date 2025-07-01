use crate::param::ParamId;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct EntityId(usize);

pub enum Entity {
    Point { x: ParamId, y: ParamId },
    Line { start: EntityId, end: EntityId },
    Circle { center: EntityId, radius: ParamId },
}

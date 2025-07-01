use crate::entity::EntityId;
use crate::param::ParamId;
use crate::sketch::SketchId;

pub enum Feature {
    Extrude {
        sketch: SketchId,
        height: ParamId,
    },
    Revolve {
        sketch: SketchId,
        axis: (EntityId, EntityId),
        angle: ParamId,
    },
}

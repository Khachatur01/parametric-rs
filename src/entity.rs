use crate::id::Id;
use crate::param::Expression;
use id_derive::GenerateId;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct EntitySet {
    pub points: HashMap<PointId, PointEntity>,
    pub segments: HashMap<SegmentId, SegmentEntity>,
    pub rectangles: HashMap<RectangleId, RectangleEntity>,
    pub circles: HashMap<CircleId, CircleEntity>,
    pub meshes: HashMap<MeshId, MeshEntity>,
}

impl EntitySet {
    pub fn add_point(&mut self, point: PointEntity) -> PointId {
        let id = PointId::generate();
        self.points.insert(id, point);
        id
    }

    pub fn add_segment(&mut self, segment: SegmentEntity) -> SegmentId {
        let id = SegmentId::generate();
        self.segments.insert(id, segment);
        id
    }

    pub fn add_rectangle(&mut self, rectangle: RectangleEntity) -> RectangleId {
        let id = RectangleId::generate();
        self.rectangles.insert(id, rectangle);
        id
    }

    pub fn add_circle(&mut self, circle: CircleEntity) -> CircleId {
        let id = CircleId::generate();
        self.circles.insert(id, circle);
        id
    }

    pub fn add_mesh(&mut self, mesh: MeshEntity) -> MeshId {
        let id = MeshId::generate();
        self.meshes.insert(id, mesh);
        id
    }
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, GenerateId)]
pub struct PointId(Id);
#[derive(PartialEq, Clone, Debug)]
pub struct PointEntity {
    pub x: Expression,
    pub y: Expression,
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, GenerateId)]
pub struct SegmentId(Id);
#[derive(PartialEq, Clone, Debug)]
pub struct SegmentEntity {
    pub start: PointId,
    pub end: PointId,
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, GenerateId)]
pub struct RectangleId(Id);
#[derive(PartialEq, Clone, Debug)]
pub struct RectangleEntity {
    pub top_left: PointId,
    pub width: Expression,
    pub height: Expression,
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, GenerateId)]
pub struct CircleId(Id);
#[derive(PartialEq, Clone, Debug)]
pub struct CircleEntity {
    pub center: PointId,
    pub radius: Expression,
}


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, GenerateId)]
pub struct MeshId(Id);
#[derive(PartialEq, Clone, Debug)]
pub struct MeshEntity {
    pub edges: Vec<SegmentId>,
}

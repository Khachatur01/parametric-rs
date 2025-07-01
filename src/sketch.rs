use std::collections::HashMap;
use geometry::figure::circle::Circle;
use geometry::figure::mesh::Mesh;
use geometry::figure::segment::Segment;
use geometry::point::point_2d::Point2D;
use crate::constraint::Constraints;
use crate::entity::{Entity, EntityId};
use crate::param::{F64Param, Param, ParamId};

pub struct SketchId(u64);
pub struct Sketch {
    pub params: HashMap<ParamId, Box<dyn Param>>,
    pub entities: HashMap<EntityId, Entity>,
    pub constraints: Constraints,
}

pub trait SketchConverter {
    type Result;
    fn convert(sketch: &Sketch) -> Self::Result;
}

pub enum Shape {
    Point2D(Point2D),
    Segment2D(Segment<Point2D>),
    Circle(Circle),
    Mesh2D(Mesh<Point2D>)
}

pub struct SketchToShapeConverter;
impl SketchConverter for SketchToShapeConverter {
    type Result = Vec<Shape>;

    fn convert(sketch: &Sketch) -> Self::Result {
        sketch.entities
            .values()
            .map(|entity| match entity {
                Entity::Point { x, y } => {
                    let x = **sketch.params.get(x).unwrap().value().downcast_ref::<F64Param>().unwrap();
                    let y = **sketch.params.get(y).unwrap().value().downcast_ref::<F64Param>().unwrap();

                    Shape::Point2D(Point2D { x, y })
                }
                Entity::Line { start, end } => {
                    let start = sketch.entities.get(start).unwrap();
                    let end = sketch.entities.get(end).unwrap();
                    let (Entity::Point { x: start_x, y: start_y }, Entity::Point { x: end_x, y: end_y }) = (start, end) else {
                        panic!()
                    };

                    let start_x = **sketch.params.get(start_x).unwrap().value().downcast_ref::<F64Param>().unwrap();
                    let start_y = **sketch.params.get(start_y).unwrap().value().downcast_ref::<F64Param>().unwrap();

                    let end_x = **sketch.params.get(end_x).unwrap().value().downcast_ref::<F64Param>().unwrap();
                    let end_y = **sketch.params.get(end_y).unwrap().value().downcast_ref::<F64Param>().unwrap();

                    Shape::Segment2D(Segment::new(Point2D { x: start_x, y: start_y }, Point2D { x: end_x, y: end_y }))
                }
                Entity::Circle { center, radius } => {
                    Shape::Circle(Circle::new(Point2D { x: 0.0, y: 0.0 }, 0.0))
                }
            })
            .collect()
    }
}

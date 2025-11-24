use crate::model::model_2d::Model2D;
use crate::model::model_3d::Model3D;

pub mod model_2d;
pub mod model_3d;

pub enum Model {
    Model2D(Model2D),
    Model3D(Model3D),
}

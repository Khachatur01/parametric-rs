use crate::feature::Feature;
use crate::param::{Param, ParamId};
use crate::sketch::{Sketch, SketchId};
use std::collections::HashMap;

pub struct Model {
    pub params: HashMap<ParamId, Param>,
    pub sketches: HashMap<SketchId, Sketch>,
    pub features: Vec<Feature>,
}

impl Model {
    pub fn insert_param(&mut self, param: Param) -> ParamId {
        let param_id: ParamId = ParamId::generate();

        self.params.insert(param_id, param);

        param_id
    }

    pub fn insert_sketch(&mut self, sketch: Sketch) -> SketchId {
        let sketch_id: SketchId = SketchId::generate();

        self.sketches.insert(sketch_id, sketch);

        sketch_id
    }

    pub fn insert_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }
}



#[derive(Debug)]
pub enum ModelConversionError {}

pub trait ModelInto<T> {
    fn try_into(&self) -> Result<Vec<T>, ModelConversionError>;
}

pub trait ModelFrom<T> {
    fn try_from(inputs: &[T]) -> Result<Model, ModelConversionError>;
}

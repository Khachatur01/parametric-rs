use crate::feature::Feature;
use crate::sketch::{Sketch, SketchConverter, SketchId};
use std::collections::HashMap;

pub struct Model {
    pub sketches: HashMap<SketchId, Sketch>,
    pub features: Vec<Feature>,
}

impl Model {
    pub fn convert<Result>(&self, sketch_to_shape_converter: &dyn SketchConverter<Result>) -> Vec<Result> {
        self.sketches
            .values()
            .map(|sketch| sketch_to_shape_converter.into(sketch))
            .flatten()
            .collect()
    }
}

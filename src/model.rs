use std::collections::HashMap;
use crate::feature::Feature;
use crate::sketch::{Shape, Sketch, SketchConverter, SketchId, SketchToShapeConverter};

pub struct Model {
    pub sketches: HashMap<SketchId, Sketch>,
    pub features: Vec<Feature>,
}

impl Model {
    pub fn convert(&self) -> Vec<Shape> {
        self.sketches
            .values()
            .map(SketchToShapeConverter::convert)
            .flatten()
            .collect()
    }
}

use crate::feature::Feature;
use crate::sketch::{Sketch, SketchConverter, SketchId};
use std::collections::HashMap;

pub struct Model {
    pub sketches: HashMap<SketchId, Sketch>,
    pub features: Vec<Box<dyn Feature>>,
}

impl Model {
    pub fn add_feature(&mut self, feature: impl Feature + 'static) {
        self.features.push(Box::new(feature));
    }

    pub fn convert<Result>(&self, sketch_to_shape_converter: &dyn SketchConverter<Result>) -> Vec<Result> {
        let mut sketches = self.sketches.values().cloned().collect::<Vec<_>>();

        for feature in &self.features {
            for sketch in &mut sketches {
                feature.apply(sketch);
            }
        }

        sketches
            .iter()
            .map(|sketch| sketch_to_shape_converter.into(sketch))
            .flatten()
            .collect()
    }
}

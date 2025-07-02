use crate::feature::Feature;
use crate::sketch::{Sketch, SketchId};
use std::collections::HashMap;

pub struct Model {
    pub sketches: HashMap<SketchId, Sketch>,
    pub features: Vec<Feature>,
}

impl Model {
    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }

    // pub fn into<Result, C: SketchInto<Result>>(&self) -> Vec<Result> {
    //     let mut sketches = self.sketches.values().cloned().collect::<Vec<_>>();
    // 
    //     for feature in &self.features {
    //         for sketch in &mut sketches {
    //             feature.apply(sketch);
    //         }
    //     }
    // 
    //     sketches
    //         .iter()
    //         .map(C::into)
    //         .flatten()
    //         .collect()
    // }
}



#[derive(Debug)]
pub enum ModelConversionError {}

pub trait ModelInto<T> {
    fn try_into(&self) -> Result<Vec<T>, ModelConversionError>;
}

pub trait ModelFrom<T> {
    fn try_from(inputs: &[T]) -> Result<Model, ModelConversionError>;
}

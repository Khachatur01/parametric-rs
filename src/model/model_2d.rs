use crate::param::ParamSet;
use crate::sketch::Sketch;

pub struct Model2D {
    pub param_set: ParamSet,
    pub sketch: Sketch,
}

impl Model2D {
    // pub fn insert_param(&mut self, param: Param) -> ParamId {
    //     let param_id: ParamId = ParamId::generate();
    //
    //     self.params.insert(param_id, param);
    //
    //     param_id
    // }

    pub fn set_sketch(&mut self, sketch: Sketch) {
        self.sketch = sketch;
    }
}


#[derive(Debug)]
pub enum ModelConversionError {}

pub trait ModelInto<T> {
    fn try_into(&self) -> Result<Vec<T>, ModelConversionError>;
}

pub trait ModelFrom<T> {
    fn try_from(inputs: &[T]) -> Result<Model2D, ModelConversionError>;
}

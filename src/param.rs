use std::any::Any;
use std::ops::Deref;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ParamId(usize);

pub trait Param {
    fn label(&self) -> Option<String>;
    fn value(&self) -> &dyn Any;
}

pub struct F64Param(f64);
impl Param for F64Param {
    fn label(&self) -> Option<String> {
        None
    }

    fn value(&self) -> &dyn Any {
        &self.0
    }
}
impl Deref for F64Param {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

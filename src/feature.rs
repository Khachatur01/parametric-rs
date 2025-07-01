use crate::sketch::Sketch;

pub trait Feature {
    fn apply(&self, sketch: &mut Sketch);
}

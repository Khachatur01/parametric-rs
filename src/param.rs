use dyn_clone::DynClone;
use std::any::Any;
use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;

static PARAM_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ParamId(usize);
impl ParamId {
    pub fn generate() -> Self {
        let id: usize = PARAM_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        ParamId(id)
    }
}


pub trait Param: DynClone {
    fn label(&self) -> Option<String>;
    fn value(&self) -> &dyn Any;
}
impl Clone for Box<dyn Param> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}

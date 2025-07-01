use std::any::Any;
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


pub trait Param {
    fn label(&self) -> Option<String>;
    fn value(&self) -> &dyn Any;
}

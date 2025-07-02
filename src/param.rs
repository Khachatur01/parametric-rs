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

#[derive(PartialEq, Clone, Debug)]
pub enum ParamValue {
    F64(f64)
}

#[derive(PartialEq, Clone, Debug)]
pub struct Param {
    pub name: Option<String>,
    pub value: ParamValue,
}

impl Param {
    pub fn f64(value: f64) -> Self {
        Self {
            name: None,
            value: ParamValue::F64(value),
        }
    }
    pub fn f64_named(value: f64, label: String) -> Self {
        Self {
            name: Some(label),
            value: ParamValue::F64(value),
        }
    }
}

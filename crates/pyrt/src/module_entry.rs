//! The fixed module interface (upstream bot/runtime.py): one entry per module
//! in bot/modules/.
use crate::value::Py;

pub struct ModuleEntry {
    pub name: &'static str,
    pub priority: fn() -> i64,
    pub observe: Option<fn(&[Py]) -> Py>,
    pub respond: Option<fn(&[Py]) -> Py>,
    pub after: Option<fn(&[Py]) -> Py>,
}

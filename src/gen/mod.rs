//! Generated Rust modules transpiled from upstream bot/ (stub until the transpiler runs).
use crate::pyrt::value::*;
use crate::runtime::ModuleEntry;

/// Dynamic import support for transpiled `importlib.import_module` calls.
pub fn import_module(_name: &str, _package: Option<&str>) -> PyResult {
    Err(py_err("ImportError", "no modules registered"))
}

pub fn registry() -> Vec<ModuleEntry> { vec![] }

pub fn arbiter_choose() -> Option<fn(&[Py]) -> Py> { None }

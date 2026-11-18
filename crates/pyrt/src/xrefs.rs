//! Cross-crate registry: fn-pointer indirection for generated-module references
//! that would otherwise create crate dependency cycles. Every generated module
//! registers its functions, globals (getter/setter) and module object at startup
//! (gen_meta::register_all); back-edge references resolve through here.
//! Single-threaded like the rest of the runtime: all tables are thread-local.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::exc::{self, PyExc};
use crate::value::*;

thread_local! {
    static FUNCS: RefCell<HashMap<(String, String), Py>> = RefCell::new(HashMap::new());
    static GETTERS: RefCell<HashMap<(String, String), fn() -> Py>> = RefCell::new(HashMap::new());
    static SETTERS: RefCell<HashMap<(String, String), fn(Py)>> = RefCell::new(HashMap::new());
    static MODULES: RefCell<HashMap<String, fn() -> Py>> = RefCell::new(HashMap::new());
}

pub fn reg_fn(module: &str, name: &str, f: Py) {
    FUNCS.with(|m| m.borrow_mut().insert((module.to_string(), name.to_string()), f));
}

pub fn reg_access(module: &str, name: &str, get: fn() -> Py, set: fn(Py)) {
    GETTERS.with(|m| m.borrow_mut().insert((module.to_string(), name.to_string()), get));
    SETTERS.with(|m| m.borrow_mut().insert((module.to_string(), name.to_string()), set));
}

pub fn reg_module(module: &str, f: fn() -> Py) {
    MODULES.with(|m| m.borrow_mut().insert(module.to_string(), f));
}

fn lookup_func(module: &str, name: &str) -> Option<Py> {
    let direct = FUNCS.with(|m| m.borrow().get(&(module.to_string(), name.to_string())).cloned());
    if direct.is_some() {
        return direct;
    }
    // fall back to a registered global (it may hold a callable)
    let g = GETTERS.with(|m| m.borrow().get(&(module.to_string(), name.to_string())).copied());
    g.map(|f| f())
}

pub fn xcall(module: &str, name: &str, args: &[Py]) -> Py {
    match lookup_func(module, name) {
        Some(f) => crate::builtins::py_call(&f, args),
        None => exc::raise(PyExc::new("AttributeError", &format!("module '{}' has no attribute '{}'", module, name))),
    }
}

pub fn xget(module: &str, name: &str) -> Py {
    let g = GETTERS.with(|m| m.borrow().get(&(module.to_string(), name.to_string())).copied());
    if let Some(g) = g {
        return g();
    }
    match lookup_func(module, name) {
        Some(f) => f,
        None => {
            let factory = MODULES.with(|m| m.borrow().get(module).copied());
            if let Some(factory) = factory {
                if let Py::Module(m) = factory() { if let Some(v) = (m.get)(name) { return v; } }
            }
            exc::raise(PyExc::new("AttributeError", &format!("module '{}' has no attribute '{}'", module, name)))
        }
    }
}

pub fn xset(module: &str, name: &str, v: Py) {
    let s = SETTERS.with(|m| m.borrow().get(&(module.to_string(), name.to_string())).copied());
    match s {
        Some(s) => s(v),
        None => exc::raise(PyExc::new("AttributeError", &format!("module '{}' has no attribute '{}'", module, name))),
    }
}

pub fn xmodule(module: &str) -> Py {
    let factory = MODULES.with(|m| m.borrow().get(module).copied());
    match factory { Some(f) => f(), None => exc::raise(PyExc::new("ImportError", module)) }
}

/// importlib.import_module for transpiled callers: resolves relative names
/// against package and strips the leading "bot.".
pub fn import_module(name: &str, package: Option<&str>) -> Result<Py, PyExc> {
    let joined = if let Some(rest) = name.strip_prefix('.') {
        match package {
            Some(p) => format!("{}.{}", p, rest),
            None => rest.to_string(),
        }
    } else {
        name.to_string()
    };
    let short = joined.strip_prefix("bot.").unwrap_or(&joined).to_string();
    let known = MODULES.with(|m| m.borrow().contains_key(&short));
    if known {
        Ok(xmodule(&short))
    } else {
        Err(PyExc::new("ImportError", &format!("No module named '{}'", name)))
    }
}

pub fn children(package: &str) -> Vec<String> {
    let prefix = format!("{}.", package.trim_start_matches("bot."));
    let mut names = MODULES.with(|m| m.borrow().keys().filter_map(|k| k.strip_prefix(&prefix).filter(|s| !s.contains('.')).map(str::to_string)).collect::<Vec<_>>());
    names.sort(); names
}

#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use pyrt::builtins::*;
use pyrt::pybuiltins::*;
use pyrt::stdlib::*;
use pyrt::value::*;
use pyrt::exc;
use pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


thread_local! { static G_reply: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT __main__: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_reply() -> Py { ensure_init(); G_reply.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_reply(v: Py) { ensure_init(); G_reply.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Fixed interface: one reply per input line. Put bot logic in bot/modules/.");
    s_reply(py_func("reply", |args: &[Py]| pyrt::xrefs::xcall("runtime", "reply", args)));
    if py_truthy(&Py::Bool(py_eq(&py_str("bot.__main__"), &py_str("__main__")))) {
    let _ = main(&[]);
    }
}
pub fn main(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = Py::None;
    let mut line: Py = Py::None;
    let mut memory: Py = Py::None;
    memory = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_attr("sys", "stdin").unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    let __try2 = exc::catch(|| {
    answer = pyrt::xrefs::xcall("runtime", "reply", &[mcall(&line.clone(), "rstrip", &[py_str("\r\n")]), memory.clone()]);
    answer = { let __b = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]); if py_truthy(&__b) { __b } else { py_str("Hello!") } };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["Exception"]) {
                __handled6 = true;
    answer = py_str("Hello!");
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answer.clone()); __kws.extend(vec![("flush".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("print", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.__main__".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "main" => Some(py_func("main", |args: &[Py]| main(args))),
            "reply" => Some(g_reply()),
            "sys" => Some(stdlib_module("sys")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "reply" => s_reply(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("__main__", module_obj);
    pyrt::xrefs::reg_fn("__main__", "main", py_func("main", |args: &[Py]| main(args)));
    pyrt::xrefs::reg_access("__main__", "reply", g_reply, s_reply);
}

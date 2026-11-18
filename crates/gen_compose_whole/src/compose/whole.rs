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


thread_local! { static G__CONTINUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAST_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MOST_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.whole: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g__CONTINUES() -> Py { ensure_init(); G__CONTINUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINUES(v: Py) { ensure_init(); G__CONTINUES.with(|c| *c.borrow_mut() = v); }
pub fn g__LAST_CLAUSE() -> Py { ensure_init(); G__LAST_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAST_CLAUSE(v: Py) { ensure_init(); G__LAST_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g__MOST_AFTER() -> Py { ensure_init(); G__MOST_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MOST_AFTER(v: Py) { ensure_init(); G__MOST_AFTER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A reply cut to length without ending on a condition whose doing was cut away.\n\nHanding back a part of an earlier answer means cutting it to the length of a reply, and the\nusual cut is at the last comma that fits: \"... 48 hours ahead?\" If it keeps happening\" is what\nthat gives, and \"If it keeps happening.\" tells the person nothing to do - the step is in the\nclause the cut took away. The fault is general: any sentence written \"If X, do Y\" loses Y when\nit is cut between X and Y, and the same happens with \"unless\" and \"in case\".\n\nSo the cut is checked before it is used: when the piece that fits would end on one of those\nconditions, the cut moves on to the end of the doing that follows it, or, if that doing is too\nlong to carry, steps back to the clause before the condition. Either way the reply is a whole\nstep rather than a condition with its consequence missing.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.whole._LAST_CLAUSE"); }
    s__LAST_CLAUSE(stdlib_call("re", "compile", &[py_str("(?:^|[,;:]\\s+|[.!?][\\\"'\\u201d]?\\s+)((?:and\\s+)?(?:if|unless|in\\s+case)\\b[^,;:]*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.whole._CONTINUES"); }
    s__CONTINUES(stdlib_call("re", "compile", &[py_str("^['\\\"\\u201d]?(?:or|and|but|which|who|whom|whose|that|so|then|yet|plus|as|if|unless)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.whole._MOST_AFTER"); }
    s__MOST_AFTER(Py::Int(90));
}
pub fn a_whole_cut(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut most: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "most") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: most"))) };
    let mut after: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut end: Py = Py::None;
    let mut hanging: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("``text`` cut to about ``most`` characters, never ending on a hanging condition.\n\n    The caller asks for the last comma or semicolon inside ``most``; when that lands between a\n    condition and its doing, the cut is moved to the end of the doing, or back to before the\n    condition when the doing is too long. A text that already fits is returned unchanged.\n    ");
    whole = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&op_le(&call_builtin("len", &[whole.clone()]).unwrap_or_else(|e| exc::raise(e)), &most.clone())) {
    return whole.clone();
    }
    cut = call_builtin("max", &[mcall(&whole.clone(), "rfind", &[py_str(", "), Py::Int(0), most.clone()]), mcall(&whole.clone(), "rfind", &[py_str("; "), Py::Int(0), most.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&cut.clone(), &Py::Int(80))) {
    cut = mcall(&whole.clone(), "rfind", &[py_str(" "), Py::Int(0), most.clone()]);
    }
    cut = call_builtin("max", &[cut.clone(), Py::Int(80)]).unwrap_or_else(|e| exc::raise(e));
    kept = mcall(&slice_or_raise(&whole.clone(), &Py::None, &cut.clone(), &Py::None), "strip", &[py_str(" ,;:-")]);
    dropped = mcall(&slice_or_raise(&whole.clone(), &cut.clone(), &Py::None, &Py::None), "strip", &[py_str(" ,;:-")]);
    hanging = mcall(&g__LAST_CLAUSE(), "search", &[kept.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&hanging.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&dropped.clone())); if py_truthy(&__b) { __b } else { mcall(&g__CONTINUES(), "match", &[dropped.clone()]) } } } }) {
    return kept.clone();
    }
    after = stdlib_call("re", "search", &[py_str("[,;]\\s"), slice_or_raise(&whole.clone(), &op_add(&cut.clone(), &Py::Int(2)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e));
    end = if py_truthy(&after.clone()) { op_add(&op_add(&cut.clone(), &Py::Int(2)), &mcall(&after.clone(), "start", &[])) } else { call_builtin("len", &[whole.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    doing = slice_or_raise(&whole.clone(), &cut.clone(), &end.clone(), &Py::None);
    if py_truthy(&{ let __b = op_le(&op_sub(&end.clone(), &cut.clone()), &g__MOST_AFTER()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&doing.clone(), &py_str("'"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&doing.clone(), &py_str("\""))) } } } }) {
    return mcall(&slice_or_raise(&whole.clone(), &Py::None, &end.clone(), &Py::None), "strip", &[py_str(" ,;:-")]);
    }
    return mcall(&slice_or_raise(&kept.clone(), &Py::None, &mcall(&hanging.clone(), "start", &[Py::Int(1)]), &Py::None), "strip", &[py_str(" ,;:-")]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.whole".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "_CONTINUES" => Some(g__CONTINUES()),
            "_LAST_CLAUSE" => Some(g__LAST_CLAUSE()),
            "_MOST_AFTER" => Some(g__MOST_AFTER()),
            "a_whole_cut" => Some(py_func("a_whole_cut", |args: &[Py]| a_whole_cut(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "_CONTINUES" => s__CONTINUES(v),
            "_LAST_CLAUSE" => s__LAST_CLAUSE(v),
            "_MOST_AFTER" => s__MOST_AFTER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.whole", module_obj);
    pyrt::xrefs::reg_fn("compose.whole", "a_whole_cut", py_func("a_whole_cut", |args: &[Py]| a_whole_cut(args)));
    pyrt::xrefs::reg_access("compose.whole", "_CONTINUES", g__CONTINUES, s__CONTINUES);
    pyrt::xrefs::reg_access("compose.whole", "_LAST_CLAUSE", g__LAST_CLAUSE, s__LAST_CLAUSE);
    pyrt::xrefs::reg_access("compose.whole", "_MOST_AFTER", g__MOST_AFTER, s__MOST_AFTER);
}

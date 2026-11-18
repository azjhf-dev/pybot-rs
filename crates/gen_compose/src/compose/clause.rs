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


thread_local! { static G_NOTHING_TO_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_TO_LOOK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OPENS_A_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHAPES_FROM_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_NOTHING_TO_A_THING() -> Py { ensure_init(); G_NOTHING_TO_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_TO_A_THING(v: Py) { ensure_init(); G_NOTHING_TO_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_TO_LOOK() -> Py { ensure_init(); G_WHAT_TO_LOOK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_LOOK(v: Py) { ensure_init(); G_WHAT_TO_LOOK.with(|c| *c.borrow_mut() = v); }
pub fn g__OPENS_A_QUESTION() -> Py { ensure_init(); G__OPENS_A_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OPENS_A_QUESTION(v: Py) { ensure_init(); G__OPENS_A_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__SHAPES_FROM_A_CLAUSE() -> Py { ensure_init(); G__SHAPES_FROM_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHAPES_FROM_A_CLAUSE(v: Py) { ensure_init(); G__SHAPES_FROM_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The question is the clause after the comma.\n\nPeople lead with the situation and ask at the end: \"camping, might rain, what to pack\", \"5 foot 2,\ntrousers for work, what do i look for when buying a pair\". The understanding engine already splits\nthe line - `understood[\"focus\"]` is the clause after the last comma - but the shape of answer is\nread from the whole line, and a lead-in can change what that reading looks like: \"might rain\"\ncarries an auxiliary, so the line reads as a yes/no, and the person who asked what to pack is owed\na list rather than a verdict.\n\nTwo things are read here, and only where the whole-line reading is weak:\n\n- the clause after the comma, when it is a question of its own (`what to pack`, `what do i need`,\n  `what do i look for`) and the line is one clause no longer;\n- a thing the person has not got (\"wedding in september, nothing to wear\"), which is the thing they\n  are asking for.\n\nA line that already asks for a shape of its own keeps it: this can only lift a reading that came\nback as nothing or as a yes/no, never overrule a time, an amount or a place. `bot/modules/intent.py`\ncalls `shape()` after `fit.wants()`, and the arbiter reads the result as it always has.\n");
    s_WHAT_TO_LOOK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat\\s+(?:to|(?:do|should|shall|can|could|must)\\s+(?:i|we))\\s+(?:need to\\s+|have to\\s+|ought to\\s+)?(?:look(?!\\s+like\\b)(?:\\s+(?:out\\s+|around\\s+)?for)?|check(?:\\s+for)?|watch\\s+out\\s+for|ask(?:\\s+for)?|avoid)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOTHING_TO_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bnothing (?:to|for) (?:wear|put on|bring|take|pack|cook|make|eat|have|drink|watch|read|give|buy|get|play)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OPENS_A_QUESTION(py_tuple(vec![py_str("what"), py_str("which"), py_str("how")]));
    s__SHAPES_FROM_A_CLAUSE(py_tuple(vec![py_str("pick"), py_str("options"), py_str("steps")]));
}
pub fn _said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn _focus(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut focus: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The clause after the last comma, or None when the line is one clause.");
    focus = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("focus")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); __args.push(py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    focus = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&focus.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    focus = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&focus.clone(), "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&focus.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&focus.clone(), &said.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_mul(&Py::Float(0.85), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    return focus.clone();
    Py::None
}
pub fn the_clause_ask(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut focus: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The shape asked in the clause after the comma, or None when the clause asks for none.\n\n    Only the asks a clause can carry on its own are read - \"what to cook, wear, pack or buy\", \"what\n    to do\", \"what do i need\", \"what do i look for\" - so nothing else is invented from it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    focus = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _focus(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&focus.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__OPENS_A_QUESTION()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&focus.clone(), "startswith", args))(&__args) })) } }) {
    return Py::None;
    }
    focus = op_add(&op_add(&py_str(" "), &focus.clone()), &py_str(" "));
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fit::g_WHAT_TO_PICK(), "search", args))(&__args) }) {
    return py_str("pick");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fit::g_WHAT_I_NEED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fit::g_WHAT_YOU_TAKE(), "search", args))(&__args) } } }) {
    return py_str("options");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fit::g_WHAT_TO_STEPS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::compose::fit::g_WHAT_TO_DO(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(focus.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_TO_LOOK(), "search", args))(&__args) } } } } }) {
    return py_str("steps");
    }
    return Py::None;
    Py::None
}
pub fn shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut read: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "read") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: read"))) };
    let mut want: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "want") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: want"))) };
    let mut said: Py = Py::None;
    let _ = py_str("`want`, lifted to the shape the clause after the comma asks, or left alone.\n\n    A shape the whole line already gave - a time, an amount, a place, or a pick, options or steps\n    of its own - is kept: the clause is only read where the line came back as nothing or as a\n    yes/no a lead-in put there.\n    ");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__SHAPES_FROM_A_CLAUSE(), &want.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("time"), py_str("amount"), py_str("place"), py_str("person"), py_str("meaning"), py_str("reason"), py_str("number"), py_str("clock"), py_str("price")]), &want.clone())) } }) {
    return want.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return want.clone();
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said(args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOTHING_TO_A_THING(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("asks")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&read.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &py_str("list"))) } }) {
    return py_str("pick");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &said.clone()), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_TO_LOOK(), "search", args))(&__args) }) {
    return py_str("steps");
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(read.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_clause_ask(args))(&__args) }; if py_truthy(&__b) { __b } else { want.clone() } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.clause".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "NOTHING_TO_A_THING" => Some(g_NOTHING_TO_A_THING()),
            "WHAT_TO_LOOK" => Some(g_WHAT_TO_LOOK()),
            "_OPENS_A_QUESTION" => Some(g__OPENS_A_QUESTION()),
            "_SHAPES_FROM_A_CLAUSE" => Some(g__SHAPES_FROM_A_CLAUSE()),
            "_focus" => Some(py_func("_focus", |args: &[Py]| _focus(args))),
            "_said" => Some(py_func("_said", |args: &[Py]| _said(args))),
            "shape" => Some(py_func("shape", |args: &[Py]| shape(args))),
            "the_clause_ask" => Some(py_func("the_clause_ask", |args: &[Py]| the_clause_ask(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "NOTHING_TO_A_THING" => s_NOTHING_TO_A_THING(v),
            "WHAT_TO_LOOK" => s_WHAT_TO_LOOK(v),
            "_OPENS_A_QUESTION" => s__OPENS_A_QUESTION(v),
            "_SHAPES_FROM_A_CLAUSE" => s__SHAPES_FROM_A_CLAUSE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.clause", module_obj);
    pyrt::xrefs::reg_fn("compose.clause", "_said", py_func("_said", |args: &[Py]| _said(args)));
    pyrt::xrefs::reg_fn("compose.clause", "_focus", py_func("_focus", |args: &[Py]| _focus(args)));
    pyrt::xrefs::reg_fn("compose.clause", "the_clause_ask", py_func("the_clause_ask", |args: &[Py]| the_clause_ask(args)));
    pyrt::xrefs::reg_fn("compose.clause", "shape", py_func("shape", |args: &[Py]| shape(args)));
    pyrt::xrefs::reg_access("compose.clause", "NOTHING_TO_A_THING", g_NOTHING_TO_A_THING, s_NOTHING_TO_A_THING);
    pyrt::xrefs::reg_access("compose.clause", "WHAT_TO_LOOK", g_WHAT_TO_LOOK, s_WHAT_TO_LOOK);
    pyrt::xrefs::reg_access("compose.clause", "_OPENS_A_QUESTION", g__OPENS_A_QUESTION, s__OPENS_A_QUESTION);
    pyrt::xrefs::reg_access("compose.clause", "_SHAPES_FROM_A_CLAUSE", g__SHAPES_FROM_A_CLAUSE, s__SHAPES_FROM_A_CLAUSE);
}

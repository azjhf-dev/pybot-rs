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


thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_PERSON_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITHOUT_A_THING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_A_THING_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_PERSON() -> Py { ensure_init(); G_SECOND_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_PERSON(v: Py) { ensure_init(); G_SECOND_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_PERSON_RE() -> Py { ensure_init(); G_SECOND_PERSON_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_PERSON_RE(v: Py) { ensure_init(); G_SECOND_PERSON_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_WITHOUT_A_THING_LINES() -> Py { ensure_init(); G_WITHOUT_A_THING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_A_THING_LINES(v: Py) { ensure_init(); G_WITHOUT_A_THING_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_A_THING_LINES() -> Py { ensure_init(); G_WITH_A_THING_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_A_THING_LINES(v: Py) { ensure_init(); G_WITH_A_THING_LINES.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing the person cannot follow is asked about, by name.\n\n\"the instructions are confusing\", \"i cant follow the recipe\", \"im baffled by my payslip\", \"my head\nis spinning with all this\": the person has said that something specific will not go in. What a\nfriend does with that is ask about the thing - which bit, where it lost them - because the thing\nis the only place the answer can come from.\n\nOn main every one of those got a reply that asked them to start again: \"I'm listening - say more\nif it helps\", or \"Thanks for telling me - I'm here if you want to dig into any of that\", and\n\"im baffled by my payslip\" got \"Good to hear. What was the best bit?\" - a congratulation for a\nthing that would not go in.\n\nThe reading is the understanding engine's (`memory[\"understood\"][\"confusion\"]`,\n`bot/understanding/kinds.py::confusion_of`): the shape of the line and the thing it named. This\nmodule is the wording, at priority -900, just above the composer floor and below every module that\ncan answer for itself.\n\nNothing is taken from a real answer. The composer's floor is not only the shrug: it gives a form\nnobody can read the three things to look for, and \"im confused about my medication\" the medication\nrow. So the composer is asked first (`bot/compose/hedge.py`), and this module answers only where\nit has nothing - or where its answer is a congratulation, which a line about a thing that will not\ngo in cannot be.\n");
    s_PRIORITY(op_neg(&Py::Int(900)));
    s_WITH_A_THING_LINES(py_tuple(vec![py_str("Which bit of {thing} stops making sense? Say the first line that does and we'll take it from there."), py_str("Which part of {thing} loses you - the start, or somewhere in the middle?"), py_str("What is it about {thing} that isn't going in - the words, or the order of it?"), py_str("Let's take {thing} one line at a time. Which line is the first one that doesn't land?")]));
    s_WITHOUT_A_THING_LINES(py_tuple(vec![py_str("What is the thing itself - a letter, a form, a set of instructions? Even the name helps."), py_str("Say the last bit that made sense and I'll take it from there."), py_str("Which part is clear and which part is foggy? Start where it stops making sense.")]));
    s_SECOND_PERSON(py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your")), (py_str("me"), py_str("you")), (py_str("us"), py_str("you")), (py_str("mine"), py_str("yours")), (py_str("ours"), py_str("yours"))]));
    s_SECOND_PERSON_RE(stdlib_call("re", "compile", &[py_str("\\b(my|our|me|us|mine|ours)\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _said_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let _ = py_str("The thing as it is said to the person: \"my payslip\" -> \"your payslip\".");
    return mcall(&mcall(&g_SECOND_PERSON_RE(), "sub", &[{ let thing = thing.clone(); py_func("lambda", move |args: &[Py]| { let thing = thing.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); geti(&g_SECOND_PERSON(), &mcall(&m.clone(), "group", &[Py::Int(1)])) }) }, { let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]), "strip", &[]);
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = Py::None;
    let mut confusion: Py = Py::None;
    let mut line: Py = Py::None;
    let mut state: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The reply to a thing the person cannot follow, or None for every other line.");
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    confusion = { let __b = mcall(&understood.clone(), "get", &[py_str("confusion")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&confusion.clone()))) {
    return Py::None;
    }
    clean = pyrt::xrefs::xcall("compose.fallback", "_matchable", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); __kws.extend(vec![("misreads".to_string(), pyrt::xrefs::xget("compose.hedge", "MISREAD_AS_GOOD_NEWS"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose.hedge", "only_a_shrug", args))(&__args) }))) {
    return Py::None;
    }
    thing = _said_back(&[mcall(&confusion.clone(), "get", &[py_str("thing")])]);
    state = stdlib_call("random", "getstate", &[]).unwrap_or_else(|e| exc::raise(e));
    let __try1 = exc::catch(|| {
    if py_truthy(&thing.clone()) {
    return py_tuple(vec![Py::Bool(true), py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WITH_A_THING_LINES()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone(), clean.clone()])]);
    }
    return py_tuple(vec![Py::Bool(true), py_call(&pyrt::xrefs::xget("compose.fallback", "_choose"), &[g_WITHOUT_A_THING_LINES(), memory.clone(), clean.clone()])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    let _ = stdlib_call("random", "setstate", &[state.clone()]).unwrap_or_else(|e| exc::raise(e));
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.confusing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "PRIORITY" => Some(g_PRIORITY()),
            "SECOND_PERSON" => Some(g_SECOND_PERSON()),
            "SECOND_PERSON_RE" => Some(g_SECOND_PERSON_RE()),
            "WITHOUT_A_THING_LINES" => Some(g_WITHOUT_A_THING_LINES()),
            "WITH_A_THING_LINES" => Some(g_WITH_A_THING_LINES()),
            "_said_back" => Some(py_func("_said_back", |args: &[Py]| _said_back(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "PRIORITY" => s_PRIORITY(v),
            "SECOND_PERSON" => s_SECOND_PERSON(v),
            "SECOND_PERSON_RE" => s_SECOND_PERSON_RE(v),
            "WITHOUT_A_THING_LINES" => s_WITHOUT_A_THING_LINES(v),
            "WITH_A_THING_LINES" => s_WITH_A_THING_LINES(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.confusing", module_obj);
    pyrt::xrefs::reg_fn("modules.confusing", "_said_back", py_func("_said_back", |args: &[Py]| _said_back(args)));
    pyrt::xrefs::reg_fn("modules.confusing", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.confusing", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.confusing", "SECOND_PERSON", g_SECOND_PERSON, s_SECOND_PERSON);
    pyrt::xrefs::reg_access("modules.confusing", "SECOND_PERSON_RE", g_SECOND_PERSON_RE, s_SECOND_PERSON_RE);
    pyrt::xrefs::reg_access("modules.confusing", "WITHOUT_A_THING_LINES", g_WITHOUT_A_THING_LINES, s_WITHOUT_A_THING_LINES);
    pyrt::xrefs::reg_access("modules.confusing", "WITH_A_THING_LINES", g_WITH_A_THING_LINES, s_WITH_A_THING_LINES);
}

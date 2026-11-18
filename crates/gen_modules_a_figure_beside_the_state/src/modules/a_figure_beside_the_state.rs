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


thread_local! { static G_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE_WITH_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINUS_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINUS_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_AS_IT_COMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT modules.a_figure_beside_the_state: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_FIGURE() -> Py { ensure_init(); G_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE(v: Py) { ensure_init(); G_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE_WITH_LEAD() -> Py { ensure_init(); G_FIGURE_WITH_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE_WITH_LEAD(v: Py) { ensure_init(); G_FIGURE_WITH_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_MINUS_FIGURE() -> Py { ensure_init(); G_MINUS_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINUS_FIGURE(v: Py) { ensure_init(); G_MINUS_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_MINUS_WORD() -> Py { ensure_init(); G_MINUS_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINUS_WORD(v: Py) { ensure_init(); G_MINUS_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_AS_IT_COMES() -> Py { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_AS_IT_COMES(v: Py) { ensure_init(); G_TAKES_AS_IT_COMES.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAD() -> Py { ensure_init(); G__LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); G__LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER() -> Py { ensure_init(); G__NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER(v: Py) { ensure_init(); G__NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT() -> Py { ensure_init(); G__UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT(v: Py) { ensure_init(); G__UNIT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A bare figure of heat said beside how a person feels.\n\n\"its 20 degrees and im freezing\" holds two halves: a figure of heat with nothing else around it,\nand a person saying how they are. The figure is the weather or the room, and the state is the\nperson's, so what a friend does is one practical thing and one question about the situation.\n\nThe engine already reads the warmth of the person (`understood[\"warmth\"]`), and the composer\nalready has the pools for a body state - but only for a line that is the state and nothing else\n(\"im cold\"). A figure in front of the state is what the two readings disagree about: the knowledge\nbase reads the figure as a subject (\"Water freezes at 0C (32F)\", and with a room named \"A degree\npasses at 40% in most UK universities\"), and it still holds the turn because a card that matched a\nword outranks the composer's pool. So the reading is done here, before the knowledge base: take the\nbare figure out of the line, and ask the composer's own reader whether what is left is a body state.\nThat is the whole of the rule, and it needs no list of the substances a figure can be about:\n\n* \"its 20 degrees and im freezing\" -> \"and im freezing\" -> cold -> the warm pool;\n* \"im freezing, its 20 degrees\" -> \"im freezing\" -> cold;\n* \"its 30 degrees and im boiling\" -> \"and im boiling\" -> hot -> the cool pool;\n* \"its 20 degrees and im hungry\" -> hungry;\n* but \"the office is 20 degrees and im freezing\" leaves \"the office is and im freezing\", and a\n  room named is the heating answer's, so this passes;\n* \"what temperature does water freeze\", \"my pipes are freezing\", \"im roasting a chicken at 180\n  degrees\" and \"is 20 degrees cold enough to freeze the pipes\" have no state left, or no figure,\n  and pass.\n\nThe words of the pools are the composer's: this module never writes a reply of its own.\n");
    s_PRIORITY(Py::Int(1000));
    s__NUMBER(py_str("(?:\\d+(?:\\.\\d+)?|zero|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred)"));
    s__UNIT(py_str("(?:degrees?|deg\\b|°|celsius|centigrade|fahrenheit|c\\b|f\\b)"));
    s__LEAD(py_str("(?:its|it's|its\\s+about|about|around|only|just|currently|still|nearly|almost|minus|negative|plus|and|but|so|then|is|be)"));
    s_FIGURE(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g__NUMBER(), -1, "")); __s.push_str("\\s*"); __s.push_str(&fstr(&g__UNIT(), -1, "")); __s.push_str("\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MINUS_FIGURE(stdlib_call("re", "compile", &[py_str("(?<![\\w-])(?:minus\\s+|[-\\u2212]\\s?)\\d+\\s*(?:degrees?)?(?:\\s*(?:outside|out\\s+there|out|in\\s+here|today|tonight|now))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MINUS_WORD(stdlib_call("re", "compile", &[py_str("(?<!\\w)minus\\s+\\d+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_WITH_LEAD(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:(?:"); __s.push_str(&fstr(&g__LEAD(), -1, "")); __s.push_str(")\\s+)*"); __s.push_str(&fstr(&g__NUMBER(), -1, "")); __s.push_str("\\s*"); __s.push_str(&fstr(&g__UNIT(), -1, "")); __s.push_str("\\b(?:\\s+(?:outside|out\\s+there|in\\s+here|here|today|tonight|now|outside|already|still))?"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_AS_IT_COMES(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
}
pub fn a_figure_of_heat(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line says a temperature: \"20 degrees\", \"20c\", \"minus three degrees\".");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g_FIGURE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MINUS_WORD(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MINUS_FIGURE(), "search", &[line.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn without_the_figure(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The line with a bare figure of heat taken out, and the join left tidy.\n\n    Only the figure and the words that introduce it go, so the rest of the line reads as it did:\n    \"its 20 degrees and im freezing\" is \"and im freezing\", and \"im freezing, its 20 degrees\" is\n    \"im freezing,\". A figure inside a longer phrase (\"at 180 degrees for an hour\") is left alone\n    by the caller's own reading, because what is left of the line is not a body state.\n    ");
    line = mcall(&g_FIGURE_WITH_LEAD(), "sub", &[py_str(" "), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    line = mcall(&g_MINUS_FIGURE(), "sub", &[py_str(" "), line.clone()]);
    line = stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    line = stdlib_call("re", "sub", &[py_str("^[\\s,;:-]*(?:(?:its|it|it's|is|was|there|and|but|so|then|about|around|only|just|still|currently|be)\\b[\\s,;:-]*)+"), py_str(""), mcall(&line.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    line = stdlib_call("re", "sub", &[py_str("[\\s,;:-]+$"), py_str(""), mcall(&line.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    line = stdlib_call("re", "sub", &[py_str("[\\s,;:-]+(?:(?:its|it|it's|is|was|there|about|around|only|just|still|currently|and|but|so|then)\\b[\\s,;:-]*)+$"), py_str(""), mcall(&line.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    return mcall(&line.clone(), "strip", &[]);
    Py::None
}
pub fn a_state_said_with_a_figure(args: &[Py]) -> Py {
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
    let mut _body_state: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("The body state the line says beside a figure (\"cold\", \"hot\"), or None.\n\n    The composer's reader is the authority on what counts as a body state, so it is asked about\n    the line with the figure taken out. A line with no figure at all is not this module's.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&a_figure_of_heat(&[line.clone()])))) {
    return Py::None;
    }
    rest = without_the_figure(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    _body_state = py_func("_body_state", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback_chain", "_body_state", args));
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_call(&_body_state, &[rest.clone()])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
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
    let mut BODY_POOLS: Py = Py::None;
    let mut _choose: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut state: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The body state's own pool for a line that says a figure of heat beside it, or None.");
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } };
    state = a_state_said_with_a_figure(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&state.clone()))) {
    return Py::None;
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    _choose = py_func("_choose", |args: &[Py]| pyrt::xrefs::xcall("compose.fallback_chain", "_choose", args));
    BODY_POOLS = pyrt::xrefs::xget("compose.fallback_data", "BODY_POOLS");
    pool = mcall(&BODY_POOLS.clone(), "get", &[state.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_call(&_choose, &[pool.clone(), memory.clone(), clean.clone()])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    let mut understood: Py = Py::None;
    let _ = py_str("Only on a line that says a figure of heat beside a body state, and nowhere else.");
    understood = { let __b = mcall(&memory.clone(), "get", &[py_str("understood")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[a_state_said_with_a_figure(&[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { text.clone() } }])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.a_figure_beside_the_state".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "FIGURE" => Some(g_FIGURE()),
            "FIGURE_WITH_LEAD" => Some(g_FIGURE_WITH_LEAD()),
            "MINUS_FIGURE" => Some(g_MINUS_FIGURE()),
            "MINUS_WORD" => Some(g_MINUS_WORD()),
            "PRIORITY" => Some(g_PRIORITY()),
            "TAKES_AS_IT_COMES" => Some(g_TAKES_AS_IT_COMES()),
            "_LEAD" => Some(g__LEAD()),
            "_NUMBER" => Some(g__NUMBER()),
            "_UNIT" => Some(g__UNIT()),
            "a_figure_of_heat" => Some(py_func("a_figure_of_heat", |args: &[Py]| a_figure_of_heat(args))),
            "a_state_said_with_a_figure" => Some(py_func("a_state_said_with_a_figure", |args: &[Py]| a_state_said_with_a_figure(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            "without_the_figure" => Some(py_func("without_the_figure", |args: &[Py]| without_the_figure(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "FIGURE" => s_FIGURE(v),
            "FIGURE_WITH_LEAD" => s_FIGURE_WITH_LEAD(v),
            "MINUS_FIGURE" => s_MINUS_FIGURE(v),
            "MINUS_WORD" => s_MINUS_WORD(v),
            "PRIORITY" => s_PRIORITY(v),
            "TAKES_AS_IT_COMES" => s_TAKES_AS_IT_COMES(v),
            "_LEAD" => s__LEAD(v),
            "_NUMBER" => s__NUMBER(v),
            "_UNIT" => s__UNIT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.a_figure_beside_the_state", module_obj);
    pyrt::xrefs::reg_fn("modules.a_figure_beside_the_state", "a_figure_of_heat", py_func("a_figure_of_heat", |args: &[Py]| a_figure_of_heat(args)));
    pyrt::xrefs::reg_fn("modules.a_figure_beside_the_state", "without_the_figure", py_func("without_the_figure", |args: &[Py]| without_the_figure(args)));
    pyrt::xrefs::reg_fn("modules.a_figure_beside_the_state", "a_state_said_with_a_figure", py_func("a_state_said_with_a_figure", |args: &[Py]| a_state_said_with_a_figure(args)));
    pyrt::xrefs::reg_fn("modules.a_figure_beside_the_state", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.a_figure_beside_the_state", "takes_as_it_comes", py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args)));
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "FIGURE", g_FIGURE, s_FIGURE);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "FIGURE_WITH_LEAD", g_FIGURE_WITH_LEAD, s_FIGURE_WITH_LEAD);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "MINUS_FIGURE", g_MINUS_FIGURE, s_MINUS_FIGURE);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "MINUS_WORD", g_MINUS_WORD, s_MINUS_WORD);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "TAKES_AS_IT_COMES", g_TAKES_AS_IT_COMES, s_TAKES_AS_IT_COMES);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "_LEAD", g__LEAD, s__LEAD);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "_NUMBER", g__NUMBER, s__NUMBER);
    pyrt::xrefs::reg_access("modules.a_figure_beside_the_state", "_UNIT", g__UNIT, s__UNIT);
}

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


thread_local! { static G_ANYONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GET_ANY_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITHOUT_A_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.too_late: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANYONE() -> Py { ensure_init(); G_ANYONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANYONE(v: Py) { ensure_init(); G_ANYONE.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT() -> Py { ensure_init(); G_DOUBT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT(v: Py) { ensure_init(); G_DOUBT.with(|c| *c.borrow_mut() = v); }
pub fn g_GET_ANY_GOOD() -> Py { ensure_init(); G_GET_ANY_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GET_ANY_GOOD(v: Py) { ensure_init(); G_GET_ANY_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS() -> Py { ensure_init(); G_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); G_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_IT() -> Py { ensure_init(); G_SAYS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT(v: Py) { ensure_init(); G_SAYS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_WITHOUT_A_SUBJECT() -> Py { ensure_init(); G_WITHOUT_A_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_A_SUBJECT(v: Py) { ensure_init(); G_WITHOUT_A_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Is it too late - answered by the answer already on the screen.\n\n\"i never learned to swim im 45\" is answered with the adult-lessons card, which says in as many\nwords that instructors teach adults who are frightened of the water. The next line, \"is it too\nlate\", is the person asking whether that card still applies to them, and it was answered with a\nsearch hedge - as if nothing on the screen had been about it.\n\nThe line names no thing of its own, so no entry is reached and the knowledge base cannot see it.\nWhat can answer it is the answer itself: the sentence in it that says people like them do the\nthing.\n\nThe rules are narrow on purpose:\n\n* the line is only the doubt and nothing else, so \"is it too late to plant tulips\" - a question\n  with a thing of its own - never comes here;\n* the answer one or two turns back has to hold a sentence that says anybody can (beginners,\n  adults, any age, most people, nobody is watching), so an answer about a deadline, a closing\n  date or a cutting is never told it is not too late: nothing in it says that;\n* a hedge is never answered from itself, and a hand-back already made is not made again.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.too_late.DOUBT"); }
    s_DOUBT(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|but\\s+|ok\\s+|okay\\s+|well\\s+|right\\s+|hmm+\\s+|ah\\s+|oh\\s+)*(?:(?:is|isnt|isn'?t|would|will|was|does)\\s+(?:it|that|this|there)\\s+(?:be\\s+)?(?:too\\s+(?:late|old|young)|still\\s+(?:worth\\s+(?:it|doing|a\\s+go)|possible|an?\\s+option|on|fine|alright|ok|okay|any\\s+good)|worth\\s+(?:it|doing|a\\s+go))|(?:am|are|was|were)\\s+(?:i|we|you)\\s+too\\s+(?:old|late|young|far\\s+gone)|(?:am|are|is|was|were)\\s+(?:i|we|you|it|that|this)\\s+too\\s+(?:old|late|young)\\s+to\\s+(?:get|be|become|do|learn|start)\\s+(?:any\\s+)?(?:good|better|decent|anywhere)(?:\\s+(?:at|with)\\s+(?:it|that|this|[a-z'-]+))?\\b|(?:im|i'?m)\\s+too\\s+(?:old|late|young)\\s+to\\s+(?:get|be|become|do|learn|start)\\s+(?:any\\s+)?(?:good|better|decent|anywhere)(?:\\s+(?:at|with)\\s+(?:it|that|this|[a-z'-]+))?\\b|(?:can|could)\\s+(?:i|we|you|she|he|they)\\s+still\\s+(?:do|learn|start|join|try|manage|cope|go|take\\s+it\\s+up|get\\s+into)\\b(?:\\s+(?:it|that|this))?|(?:is|isnt|isn'?t)\\s+there\\s+(?:still\\s+)?(?:time|a\\s+chance|any\\s+point|a\\s+point)|too\\s+late(?:\\s+(?:for\\s+me|now|isnt\\s+it|right))?|(?:what|whats|what'?s)\\s+the\\s+point(?:\\s+(?:now|of\\s+it|in\\s+(?:it|trying)))?|any\\s+point(?:\\s+(?:now|in\\s+(?:it|trying))))(?:\\s+to\\s+(?:learn|start|begin|try|catch\\s+up|get\\s+any\\s+good|get\\s+going|improve|take\\s+it\\s+up|change|retrain|study|practise|practice|go\\s+back))?\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.too_late.ANYONE"); }
    s_ANYONE(stdlib_call("re", "compile", &[py_str("\\b(?:complete\\s+beginners?|beginners?|any\\s+age|no\\s+age|never\\s+too\\s+late|at\\s+your\\s+own\\s+pace|whatever\\s+your\\s+age|whatever\\s+age|all\\s+ages|people\\s+start|just\\s+as\\s+at\\s+\\w+|the\\s+same\\s+at\\s+\\w+\\s+as\\s+at\\s+\\w+|adults?\\s+(?:who|pick|learn|learns|starting|start|started|take|took|can|could|are|do|in\\s+their|of\\s+any|taught|like)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.too_late.LEADS"); }
    s_LEADS(py_tuple(vec![py_str("Not too late — {said}"), py_str("Never too late: {said}"), py_str("It is not too late — {said}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.too_late.GET_ANY_GOOD"); }
    s_GET_ANY_GOOD(stdlib_call("re", "compile", &[py_str("\\btoo\\s+(?:old|late|young)\\s+to\\s+(?:get|be|become|do|learn|start)\\s+(?:any\\s+)?(?:good|better|decent|anywhere)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.too_late.WITHOUT_A_SUBJECT"); }
    s_WITHOUT_A_SUBJECT(py_tuple(vec![py_str("Not too late — what decides it is a regular slot in the week, not the year you start in"), py_str("Never too late: a short practice most days does more than a long one now and then"), py_str("It is not too late — adults who start later tend to know what they want from it")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.too_late.SAYS_IT"); }
    s_SAYS_IT(stdlib_call("re", "compile", &[py_str("\\b(?:not\\s+too\\s+late|never\\s+too\\s+late|no\\s+age\\s+limit|any\\s+age)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut gist: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence of the answer that says anybody can, cut down to its gist, or None.");
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.meant", "_sentences", &[said.clone()]));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&mcall(&mcall(&sentence.clone(), "strip", &[]), "endswith", &[py_str("?")])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ANYONE(), "search", &[sentence.clone()])))) {
    continue;
    }
    gist = pyrt::xrefs::xcall("memory.meant", "_gist", &[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ANYONE(), "search", &[gist.clone()])))) {
    gist = { let __b = _two_clauses(&[sentence.clone()]); if py_truthy(&__b) { __b } else { gist.clone() } };
    }
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&gist.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    continue;
    }
    return gist.clone();
    }
    return Py::None;
    Py::None
}
pub fn _two_clauses(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut cuts: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("A sentence cut after its second clause, as a sentence: the one that says it, and what\n    it is said about.");
    cuts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.meant", "CLAUSE"), "finditer", &[sentence.clone()])) { let mut match_: Py = __item.clone(); __out.push(mcall(&match_.clone(), "start", &[])); } py_list(__out) })() };
    if py_truthy(&op_ge(&call_builtin("len", &[cuts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return op_add(&mcall(&slice_or_raise(&sentence.clone(), &Py::None, &geti(&cuts.clone(), &Py::Int(1)), &Py::None), "strip", &[py_str(" ,;:-.")]), &py_str("."));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("len", &[cuts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40)) } }) {
    return op_add(&mcall(&mcall(&sentence.clone(), "strip", &[]), "rstrip", &[py_str(" ,;:-.")]), &py_str("."));
    }
    return py_str("");
    Py::None
}
pub fn _the_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut body: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The sentence handed back under a plain lead, or on its own when it says it already.");
    body = mcall(&sentence.clone(), "rstrip", &[py_str(".!?")]);
    if py_truthy(&mcall(&g_SAYS_IT(), "search", &[body.clone()])) {
    return op_add(&op_add(&mcall(&geti(&body.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None)), &{ let __b = slice_or_raise(&sentence.clone(), &call_builtin("len", &[body.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None); if py_truthy(&__b) { __b } else { py_str(".") } });
    }
    said = op_add(&op_add(&mcall(&geti(&body.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&body.clone(), &Py::Int(1), &Py::None, &Py::None)), &{ let __b = slice_or_raise(&sentence.clone(), &call_builtin("len", &[body.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None); if py_truthy(&__b) { __b } else { py_str(".") } });
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), said.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_LEADS(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut _asked: Py = Py::None;
    let mut answers: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence that answers the doubt, or None when this is not that turn.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_DOUBT(), "match", &[clean.clone()])))) {
    return Py::None;
    }
    answers = pyrt::xrefs::xcall("memory.meant", "_answers", &[memory.clone()]);
    if py_truthy(&{ let __b = answers.clone(); if !py_truthy(&__b) { __b } else { mcall(&gattr(&pyrt::xrefs::xget("memory.meant", "repeat"), "HEDGE"), "search", &[geti(&geti(&answers.clone(), &Py::Int(0)), &Py::Int(0))]) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&answers.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    said = __unpk2[0].clone();
    _asked = __unpk2[1].clone();
    if py_truthy(&mcall(&gattr(&pyrt::xrefs::xget("memory.meant", "repeat"), "HEDGE"), "search", &[said.clone()])) {
    continue;
    }
    sentence = _the_sentence(&[said.clone()]);
    if py_truthy(&sentence.clone()) {
    return _the_reply(&[sentence.clone(), memory.clone(), clean.clone()]);
    }
    }
    if py_truthy(&mcall(&g_GET_ANY_GOOD(), "search", &[clean.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[g_WITHOUT_A_SUBJECT(), memory.clone(), clean.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.too_late".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANYONE" => Some(g_ANYONE()),
            "DOUBT" => Some(g_DOUBT()),
            "GET_ANY_GOOD" => Some(g_GET_ANY_GOOD()),
            "LEADS" => Some(g_LEADS()),
            "SAYS_IT" => Some(g_SAYS_IT()),
            "WITHOUT_A_SUBJECT" => Some(g_WITHOUT_A_SUBJECT()),
            "_the_reply" => Some(py_func("_the_reply", |args: &[Py]| _the_reply(args))),
            "_the_sentence" => Some(py_func("_the_sentence", |args: &[Py]| _the_sentence(args))),
            "_two_clauses" => Some(py_func("_two_clauses", |args: &[Py]| _two_clauses(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "pick" => Some(g_pick()),
            "meant" => Some(pyrt::xrefs::xmodule("memory.meant")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANYONE" => s_ANYONE(v),
            "DOUBT" => s_DOUBT(v),
            "GET_ANY_GOOD" => s_GET_ANY_GOOD(v),
            "LEADS" => s_LEADS(v),
            "SAYS_IT" => s_SAYS_IT(v),
            "WITHOUT_A_SUBJECT" => s_WITHOUT_A_SUBJECT(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.too_late", module_obj);
    pyrt::xrefs::reg_fn("memory.too_late", "_the_sentence", py_func("_the_sentence", |args: &[Py]| _the_sentence(args)));
    pyrt::xrefs::reg_fn("memory.too_late", "_two_clauses", py_func("_two_clauses", |args: &[Py]| _two_clauses(args)));
    pyrt::xrefs::reg_fn("memory.too_late", "_the_reply", py_func("_the_reply", |args: &[Py]| _the_reply(args)));
    pyrt::xrefs::reg_fn("memory.too_late", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("memory.too_late", "ANYONE", g_ANYONE, s_ANYONE);
    pyrt::xrefs::reg_access("memory.too_late", "DOUBT", g_DOUBT, s_DOUBT);
    pyrt::xrefs::reg_access("memory.too_late", "GET_ANY_GOOD", g_GET_ANY_GOOD, s_GET_ANY_GOOD);
    pyrt::xrefs::reg_access("memory.too_late", "LEADS", g_LEADS, s_LEADS);
    pyrt::xrefs::reg_access("memory.too_late", "SAYS_IT", g_SAYS_IT, s_SAYS_IT);
    pyrt::xrefs::reg_access("memory.too_late", "WITHOUT_A_SUBJECT", g_WITHOUT_A_SUBJECT, s_WITHOUT_A_SUBJECT);
    pyrt::xrefs::reg_access("memory.too_late", "pick", g_pick, s_pick);
}

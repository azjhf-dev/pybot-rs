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


thread_local! { static G_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MOVE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ITS_OWN_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_GLAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ADVICE() -> Py { ensure_init(); G_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE(v: Py) { ensure_init(); G_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOVE() -> Py { ensure_init(); G_A_MOVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOVE(v: Py) { ensure_init(); G_A_MOVE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MOVE_NOUN() -> Py { ensure_init(); G_A_MOVE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MOVE_NOUN(v: Py) { ensure_init(); G_A_MOVE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_GLAD() -> Py { ensure_init(); G_GLAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLAD(v: Py) { ensure_init(); G_GLAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ITS_OWN_NEWS() -> Py { ensure_init(); G_ITS_OWN_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ITS_OWN_NEWS(v: Py) { ensure_init(); G_ITS_OWN_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_MANY() -> Py { ensure_init(); G_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MANY(v: Py) { ensure_init(); G_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_GLAD() -> Py { ensure_init(); G_NOT_GLAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_GLAD(v: Py) { ensure_init(); G_NOT_GLAD.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A move is news, but it is not good news: the bot asks about it instead of cheering it.\n\n`im moving out next month` was answered \"That's lovely news - and something to look forward to. How\nare the plans going?\" (`NEWS_TO_COME`, reached through `goodnews.reads_as_news_to_come`, which only\nasks whether the thing is ahead). People move out after a breakup, an eviction or a rent rise at\nleast as often as into their first flat, and the reader who is being evicted gets congratulated. The\nmove is the one thing ahead that a friend never cheers blind: they ask where to and how the person\nfeels about it.\n\nWhat is here:\n\n- `reads_as_a_move` - a move said as a thing ahead or just done, without a word that makes it good\n  (\"into our first flat\", \"finally\", \"we got the keys\") and without the lines that are their own\n  kind of news (\"moving in with my girlfriend\" is a step; \"moving abroad\" is a plan with a job in\n  it). It also stands aside for a line that is clearly not glad (`_AHEAD_BAD`, and the mortal\n  `goodnews` guards) because those keep the reading they already have.\n- `reply_for` - the pool that asks, with the two halves of it named (\"are you glad to be going, or\n  is it just what has to happen\"), so either reader gets an answer that fits.\n\nThe good reason keeps the congratulations: `we are moving into our first flat in june` is a thing to\nlook forward to and is answered by the card for it, and this module never takes a line another\nmodule reads better (`greeting` only reaches here when nothing else answered).\n");
    s_A_MOVE(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|they|he|she|im|i'm|we're|were|were)\\s*(?:'m|'re|am|are|is)?\\s*(?:just\\s+|finally\\s+|now\\s+|soon\\s+|actually\\s+|been\\s+|have\\s+|had\\s+)?mov(?:e|ed|ing)\\s+(?:out|houses?|household|flats?|home|abroad|in\\b|into\\b|back\\s+(?:in\\s+)?with|away)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MOVE_NOUN(stdlib_call("re", "compile", &[py_str("\\b(?:our|my|the)\\s+move\\s+(?:is|to|next|soon|coming|happens|happening)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GLAD(stdlib_call("re", "compile", &[py_str("\\b(?:first\\s+(?:flat|house|home|place)|our\\s+own\\s+(?:flat|house|place)|dream|finally|cant\\s+wait|can'?t\\s+wait|so\\s+excited|excited\\s+to|looking\\s+forward|got\\s+the\\s+keys|we\\s+got\\s+it|brand\\s+new|bigger|nicer|better\\s+place|closer\\s+to|new\\s+job|promotion|girlfriend|boyfriend|partner|fiance|fiancee|husband|wife)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_GLAD(stdlib_call("re", "compile", &[py_str("\\b(?:evict\\w*|kicked\\s+out|thrown\\s+out|have\\s+to|havent\\s+got\\s+a\\s+choice|no\\s+choice|break\\s*up|broke\\s+up|divorce|separat\\w*|lost\\s+my\\s+job|rent\\s+(?:rise|went\\s+up)|dreading|cant\\s+afford|can'?t\\s+afford|sad|gutted|hate|awful|nightmare|landlord|selling\\s+up|sold\\s+the\\s+(?:flat|house)|for\\s+work|because\\s+of\\s+work)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE(stdlib_call("re", "compile", &[py_str("\\b(?:should\\s+(?:i|we)|shall\\s+(?:i|we)|do\\s+you\\s+think|is\\s+it\\s+worth|how\\s+do\\s+(?:i|we)|what\\s+should|any\\s+(?:advice|tips))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ITS_OWN_NEWS(stdlib_call("re", "compile", &[py_str("\\b(?:abroad|overseas|another\\s+country|to\\s+(?:spain|france|australia|canada|america|portugal|italy|germany|ireland|scotland|wales)|(?:in|into)\\s+with\\s+(?:my|her|his|our)\\s+(?:girlfriend|boyfriend|partner|fiance|fiancee|husband|wife|mum|mother|dad|father|parents))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MANY(py_tuple(vec![py_str("Big thing to have ahead of you. Where to, and are you glad to be going?"), py_str("That's a big change. Is it a move you wanted, or one that has to happen?"), py_str("Moving is a lot of work. Where are you off to, and how are you feeling about it?"), py_str("A move is never just a move. Where to, and is it a good thing for you?")]));
}
pub fn reads_as_a_move(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("True when the line is a move said without saying it is a glad one.\n\n    The guards are the ones this reading needs to stay out of the way: a reason to be glad takes the\n    line (the news-to-come pool and the card for a first flat both answer it better than a question\n    that asks whether they are glad), and so does a line that is plainly not glad - those readers\n    are already answered by the pools for a bad week, and asking \"are you glad to be going\" of\n    somebody being evicted is the failure this module exists to prevent, said backwards.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_MOVE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_MOVE_NOUN(), "search", &[clean.clone()]) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_GLAD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ITS_OWN_NEWS(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_GLAD(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ADVICE(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn reads_as_a_move_cheered(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("The move the composer would have congratulated: the reading, and the pool behind the fault.\n\n    A move said ahead is read as news to come by `goodnews.reads_as_news_to_come`, and that is what\n    reached the pool which looks forward (\"That's lovely news - and something to look forward to\")\n    at a person who may be leaving a marriage. Only that line is taken here: a move the composer\n    files (\"Noted: you move house on the 20th\") keeps its filing, because the date is the thing to\n    remember and a question about it loses the date; and a move already behind the person (\"i moved\n    house last september\") is not ahead at all, so it never enters the reading.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_move(&[clean.clone()])))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[crate::compose::goodnews::ahead_of_now(&[clean.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
pub fn reply_for(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("One of the pool's lines for a move, or None when the line is something else.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_as_a_move_cheered(&[text.clone()])))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), py_call(&crate::compose::fallback::g__choose(), &[g_MANY(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]);
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
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.moving".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE" => Some(g_ADVICE()),
            "A_MOVE" => Some(g_A_MOVE()),
            "A_MOVE_NOUN" => Some(g_A_MOVE_NOUN()),
            "GLAD" => Some(g_GLAD()),
            "ITS_OWN_NEWS" => Some(g_ITS_OWN_NEWS()),
            "MANY" => Some(g_MANY()),
            "NOT_GLAD" => Some(g_NOT_GLAD()),
            "reads_as_a_move" => Some(py_func("reads_as_a_move", |args: &[Py]| reads_as_a_move(args))),
            "reads_as_a_move_cheered" => Some(py_func("reads_as_a_move_cheered", |args: &[Py]| reads_as_a_move_cheered(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADVICE" => s_ADVICE(v),
            "A_MOVE" => s_A_MOVE(v),
            "A_MOVE_NOUN" => s_A_MOVE_NOUN(v),
            "GLAD" => s_GLAD(v),
            "ITS_OWN_NEWS" => s_ITS_OWN_NEWS(v),
            "MANY" => s_MANY(v),
            "NOT_GLAD" => s_NOT_GLAD(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.moving", module_obj);
    pyrt::xrefs::reg_fn("compose.moving", "reads_as_a_move", py_func("reads_as_a_move", |args: &[Py]| reads_as_a_move(args)));
    pyrt::xrefs::reg_fn("compose.moving", "reads_as_a_move_cheered", py_func("reads_as_a_move_cheered", |args: &[Py]| reads_as_a_move_cheered(args)));
    pyrt::xrefs::reg_fn("compose.moving", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.moving", "ADVICE", g_ADVICE, s_ADVICE);
    pyrt::xrefs::reg_access("compose.moving", "A_MOVE", g_A_MOVE, s_A_MOVE);
    pyrt::xrefs::reg_access("compose.moving", "A_MOVE_NOUN", g_A_MOVE_NOUN, s_A_MOVE_NOUN);
    pyrt::xrefs::reg_access("compose.moving", "GLAD", g_GLAD, s_GLAD);
    pyrt::xrefs::reg_access("compose.moving", "ITS_OWN_NEWS", g_ITS_OWN_NEWS, s_ITS_OWN_NEWS);
    pyrt::xrefs::reg_access("compose.moving", "MANY", g_MANY, s_MANY);
    pyrt::xrefs::reg_access("compose.moving", "NOT_GLAD", g_NOT_GLAD, s_NOT_GLAD);
}

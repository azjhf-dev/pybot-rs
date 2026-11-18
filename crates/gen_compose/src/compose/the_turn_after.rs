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


thread_local! { static G_BARE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_REQUEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_REQUEST_ALONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BARE_DAY() -> Py { ensure_init(); G_BARE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DAY(v: Py) { ensure_init(); G_BARE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_REQUEST() -> Py { ensure_init(); G_CHECK_REQUEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_REQUEST(v: Py) { ensure_init(); G_CHECK_REQUEST.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_REQUEST_ALONE() -> Py { ensure_init(); G_CHECK_REQUEST_ALONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_REQUEST_ALONE(v: Py) { ensure_init(); G_CHECK_REQUEST_ALONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_WORDS() -> Py { ensure_init(); G_SHORT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_WORDS(v: Py) { ensure_init(); G_SHORT_WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The turn after an honest limit: the same question about another day, or \"can you just check\".\n\nThe composer has a pool for the things it cannot see - today's opening hours, the prices in the\nshops, the weather, the roads - and each line in those pools says what the bot cannot do and where\nthe answer is. Both readings here are the *next* line of that exchange, and both reach the search\nhedge today:\n\n    is the library open today      \"I can't check today's opening hours, sorry. The venue's own\n                                    page or its map listing shows them, holidays included\"\n    and tomorrow                   \"I'm not sure about that one, sorry - I'd rather not guess.\n                                    A quick search online will get you a solid answer\"\n\n    whats the weather like today   \"I can't check the forecast from here, sorry. A weather app...\"\n    can you check for me           \"That's beyond what I know well, sorry. Searching for it will\n                                    turn up better sources than I can offer\"\n\nThe person has not changed the subject and has not asked anything new. \"And tomorrow\" is the same\nquestion about another day, and \"can you check for me\" is the same request put straight back. A\nfriend says the same thing again - the times are on the venue's page, and tomorrow's are no\nexception - and does not send somebody off to search when they were just told where to look.\n\nThree guards keep this from taking a turn that has an answer of its own:\n\n  - the reply just given has to be one of the composer's honest limits - a live pool's line, or one\n    of the three plain ones (`CANNOT_ANYTHING`, `CANNOT_SEE_ANYTHING_LINES`, `CANNOT_YOUR_OWN`) -\n    so a bare \"and tomorrow\" after a joke, a card or a piece of advice changes nothing at all;\n  - a bare day has to be the hours limit that was just given, nothing else (`BARE_DAY`);\n  - a request to check either names nothing at all (\"can you just check\") or names the thing the\n    limit just refused, by the same words the composer's own pool uses for it (\"can you check the\n    price for me\" after the price limit). A line naming anything else has a subject of its own and\n    is answered before this reading is asked (`CHECK_REQUEST`, `COVERS`).\n\nNothing here is a new fact and no pool is new: the reply is one of the composer's own lines,\nchosen the way it chooses any other. `bot/modules/greeting.py` asks this only where it would\notherwise have replaced a hedge, so a line with a card of its own never reaches the reading.\n");
    s_BARE_DAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|or|also|what\\s+about|how\\s+about|what\\s+if|and\\s+what\\s+about|so)?\\s*(?:(?:on|for|at)\\s+)?(?:the\\s+)?(?:bank\\s+holiday\\s+)?(?:(?:next|this|that)\\s+)?(?:monday|tuesdays?|wednesdays?|thursdays?|fridays?|saturdays?|sundays?|weekends?|weekly|tomorrow|today|tonight|week|month|christmas|boxing\\s+day|new\\s+year|easter|half\\s+term|bank\\s+holidays?|holidays?)(?:\\s+(?:day|morning|afternoon|evening|lunchtime|night))?[\\s.?!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHECK_REQUEST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:can|could|would|will)\\s+(?:you|u|ya)\\s+(?:just\\s+|please\\s+|maybe\\s+|quickly\\s+|quick\\s+)*(?:check|look\\s+(?:it|that|this)?\\s*up|look|search|google|find\\s+(?:it\\s+)?out|see)\\b|\\bjust\\s+(?:check|look\\s+it\\s+up|look|google\\s+it|google|search)\\b|\\b(?:check|look\\s+it\\s+up|search|google)\\s+(?:it|that|this)?\\s*for\\s+me\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHECK_REQUEST_ALONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:(?:and|so|ok|okay|well|but|then|cool|right|please)\\s+)*(?:(?:can|could|would|will)\\s+(?:you|u|ya)\\s+)?(?:just\\s+|please\\s+|maybe\\s+|quickly\\s+|quick\\s+)*(?:check|look\\s+(?:it|that|this)?\\s*up|look|search|google|find\\s+(?:it\\s+)?out|see)(?:\\s+(?:it|that|this|for\\s+me|then|now|again|please))*[\\s.?!]*$")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SHORT_WORDS(py_tuple(vec![py_str("open"), py_str("shut"), py_str("cost"), py_str("time"), py_str("road"), py_str("news"), py_str("bus"), py_str("car"), py_str("hot"), py_str("cold"), py_str("won"), py_str("jam"), py_str("tube")]));
}
pub fn _the_limit_just_given(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut memory: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut head: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut one: Py = Py::None;
    let mut options: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The lines and phrases of the pool the reply just given came from, or (None, ()).\n\n    Only the pools that say what the bot cannot do: the live ones (times, prices, the roads, the\n    weather) and the three plain ones. Read from the composer's own data, so a pool that grows or\n    is reworded is followed here without a second list to keep in step.\n    ");
    let __try1 = exc::catch(|| {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g_last_bot(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
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
    return py_tuple(vec![Py::None, py_tuple(vec![])]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_tuple(vec![Py::None, py_tuple(vec![])]);
    }
    let __iter6 = py_iter_or_raise(&crate::compose::fallback_data::g_LIVE_KINDS());
    for __item in __iter6 {
    let __unpk7 = py_unpack(&__item.clone(), 3, None);
    kind = __unpk7[0].clone();
    phrases = __unpk7[1].clone();
    options = __unpk7[2].clone();
    let __iter8 = py_iter_or_raise(&options.clone());
    for __item in __iter8 {
    line = __item.clone();
    head = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(40), &Py::None);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "startswith", args))(&__args) } } }) {
    return py_tuple(vec![options.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut one: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    }
    }
    }
    let __iter9 = py_iter_or_raise(&py_tuple(vec![crate::compose::fallback::g_CANNOT_ANYTHING(), crate::compose::fallback::g_CANNOT_SEE_ANYTHING_LINES(), crate::compose::fallback::g_CANNOT_YOUR_OWN()]));
    for __item in __iter9 {
    pool = __item.clone();
    let __iter10 = py_iter_or_raise(&pool.clone());
    for __item in __iter10 {
    line = __item.clone();
    head = slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::Int(40), &Py::None);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(head.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "startswith", args))(&__args) } } }) {
    return py_tuple(vec![pool.clone(), py_tuple(vec![])]);
    }
    }
    }
    return py_tuple(vec![Py::None, py_tuple(vec![])]);
    Py::None
}
pub fn _the_hours_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut _phrases: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut options: Py = Py::None;
    let _ = py_str("The composer's honest lines for a venue's opening hours, or ().");
    let __iter1 = py_iter_or_raise(&crate::compose::fallback_data::g_LIVE_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    kind = __unpk2[0].clone();
    _phrases = __unpk2[1].clone();
    options = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("hours")))) {
    return options.clone();
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _words_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrases: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The words a pool names its subject with, four letters or more, and the short ones too.");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&phrases.clone());
    for __item in __iter1 {
    phrase = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SHORT_WORDS(), &word.clone())) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "add", args))(&__args) };
    }
    }
    }
    return words.clone();
    Py::None
}
pub fn the_day_after(args: &[Py]) -> Py {
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
    let mut _phrases: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut just: Py = Py::None;
    let _ = py_str("\"And tomorrow\" straight after the honest hours line is the same question, another day.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_DAY(), "match", args))(&__args) }))) {
    return Py::None;
    }
    hours = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_hours_pool(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&hours.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_limit_just_given(args))(&__args) }, 2, None);
    just = __unpk1[0].clone();
    _phrases = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&just.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&just.clone(), &hours.clone())) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(hours.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    Py::None
}
pub fn asked_to_check(args: &[Py]) -> Py {
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
    let mut covered: Py = Py::None;
    let mut just: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"Can you just check\" straight after the limit is the same limit, said in other words.");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CHECK_REQUEST(), "search", args))(&__args) }))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_limit_just_given(args))(&__args) }, 2, None);
    just = __unpk1[0].clone();
    phrases = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&just.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CHECK_REQUEST_ALONE(), "match", args))(&__args) }))) {
    covered = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrases.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _words_of(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&covered.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&covered.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z]+")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(just.clone()); __args.push(memory.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
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
    let _ = py_str("The turn after an honest limit, or None when this is not that turn.");
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_day_after(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asked_to_check(args))(&__args) } } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.the_turn_after".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BARE_DAY" => Some(g_BARE_DAY()),
            "CHECK_REQUEST" => Some(g_CHECK_REQUEST()),
            "CHECK_REQUEST_ALONE" => Some(g_CHECK_REQUEST_ALONE()),
            "SHORT_WORDS" => Some(g_SHORT_WORDS()),
            "_the_hours_pool" => Some(py_func("_the_hours_pool", |args: &[Py]| _the_hours_pool(args))),
            "_the_limit_just_given" => Some(py_func("_the_limit_just_given", |args: &[Py]| _the_limit_just_given(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "asked_to_check" => Some(py_func("asked_to_check", |args: &[Py]| asked_to_check(args))),
            "the_day_after" => Some(py_func("the_day_after", |args: &[Py]| the_day_after(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BARE_DAY" => s_BARE_DAY(v),
            "CHECK_REQUEST" => s_CHECK_REQUEST(v),
            "CHECK_REQUEST_ALONE" => s_CHECK_REQUEST_ALONE(v),
            "SHORT_WORDS" => s_SHORT_WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.the_turn_after", module_obj);
    pyrt::xrefs::reg_fn("compose.the_turn_after", "_the_limit_just_given", py_func("_the_limit_just_given", |args: &[Py]| _the_limit_just_given(args)));
    pyrt::xrefs::reg_fn("compose.the_turn_after", "_the_hours_pool", py_func("_the_hours_pool", |args: &[Py]| _the_hours_pool(args)));
    pyrt::xrefs::reg_fn("compose.the_turn_after", "_words_of", py_func("_words_of", |args: &[Py]| _words_of(args)));
    pyrt::xrefs::reg_fn("compose.the_turn_after", "the_day_after", py_func("the_day_after", |args: &[Py]| the_day_after(args)));
    pyrt::xrefs::reg_fn("compose.the_turn_after", "asked_to_check", py_func("asked_to_check", |args: &[Py]| asked_to_check(args)));
    pyrt::xrefs::reg_fn("compose.the_turn_after", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.the_turn_after", "BARE_DAY", g_BARE_DAY, s_BARE_DAY);
    pyrt::xrefs::reg_access("compose.the_turn_after", "CHECK_REQUEST", g_CHECK_REQUEST, s_CHECK_REQUEST);
    pyrt::xrefs::reg_access("compose.the_turn_after", "CHECK_REQUEST_ALONE", g_CHECK_REQUEST_ALONE, s_CHECK_REQUEST_ALONE);
    pyrt::xrefs::reg_access("compose.the_turn_after", "SHORT_WORDS", g_SHORT_WORDS, s_SHORT_WORDS);
}

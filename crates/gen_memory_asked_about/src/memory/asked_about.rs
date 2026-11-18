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


thread_local! { static G_MOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RETURN_UNNAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CONTINUATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.asked_about: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_MOST() -> Py { ensure_init(); G_MOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); G_MOST.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURN() -> Py { ensure_init(); G_RETURN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURN(v: Py) { ensure_init(); G_RETURN.with(|c| *c.borrow_mut() = v); }
pub fn g_RETURN_UNNAMED() -> Py { ensure_init(); G_RETURN_UNNAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RETURN_UNNAMED(v: Py) { ensure_init(); G_RETURN_UNNAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT() -> Py { ensure_init(); G_SHORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT(v: Py) { ensure_init(); G_SHORT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CONTINUATION() -> Py { ensure_init(); G__A_CONTINUATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CONTINUATION(v: Py) { ensure_init(); G__A_CONTINUATION.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_THING() -> Py { ensure_init(); G__NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_THING(v: Py) { ensure_init(); G__NOT_A_THING.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The answer this chat already gave, when a later line names the thing it was about.\n\n    i havent slept much          -> Nights like that wear you down ...\n    i keep waking at 3           -> Waking in the night is normal - sleep runs in cycles ...\n    my head is full              -> The fix is what you do in those minutes ...\n    and how do i get some sleep  -> I'm not sure about that one, sorry - I'd rather not guess.\n\nThe chat had the sleep answer three turns up and then said it did not know enough. Nothing read\nthe line: every reader that hands an earlier answer back is reached only when a module *serves*\nthat answer again, and on this line nothing does - the composer has no card for \"some sleep\", so\nthe search hedge is the only candidate left, and it is the one answer that cannot help when the\nchat itself holds the answer.\n\n`topics.answer_about` already finds that card: it scores the line's own words against every\n`answers` turn in the last six and returns the one they belong to. Nothing asked it on this path.\nSo this is the asking, and the reply is the sentence of that card which carries the words the\nline asked with, said as the return it is - \"Back to sleep: ...\".\n\nNarrow on purpose:\n\n* it is only ever read in place of a hedge (`bot/modules/greeting.py`), so a line with an answer\n  of its own keeps it. That is the same gate the answer-on-screen reader uses;\n* the line has to name a thing, a content word the card also names, so a bare \"and what should i\n  do\" - which belongs to every answer that gives a step - is left to the readers that own it;\n* the sentence said back may not be one the bot has just said, so this cannot be a repeat dressed\n  as a return, and the whole card is never handed over: it is the part that answers the line.\n");
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_RETURN(py_tuple(vec![py_str("Back to {thing}: {reply}"), py_str("Back to {thing} then: {reply}"), py_str("It is the same answer on {thing}: {reply}"), py_str("As before on {thing}: {reply}")]));
    s_RETURN_UNNAMED(py_tuple(vec![py_str("Back to what I said: {reply}"), py_str("It is the same answer as before: {reply}"), py_str("As before: {reply}"), py_str("Back to it: {reply}")]));
    s_SHORT(Py::Int(5));
    s_MOST(Py::Int(45));
    s__NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nwhat which who whom whose how when where why whether whats\nnow then again left else next other same other another anything something nothing everything\nplease thanks ok okay well so and but\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CONTINUATION(stdlib_call("re", "compile", &[py_str("^(?:if|and|or|but|so|then|also|because|when|while|though|although|plus|that said)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _sentences(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut one: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(mcall(&one.clone(), "strip", &[])); } } py_list(__out) })() };
    Py::None
}
pub fn _the_words_asked(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The content words of the line, without the asking: \"how do i get some sleep\" -> sleep.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[text.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("memory.topics", "ASK_ONLY"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&out.clone(), &word.clone())) } } } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _the_part_that_answers(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut wanted: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wanted") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wanted"))) };
    let mut said_before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_before"))) };
    let mut best: Py = Py::None;
    let mut flat_before: Py = Py::None;
    let mut here: Py = Py::None;
    let mut i: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut score: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut stands: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence of the card that carries the words the line asked with, and its neighbour.");
    parts = _sentences(&[answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&parts.clone()))) {
    return py_str("");
    }
    scored = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    part = __unpk2[1].clone();
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pyrt::xrefs::xcall("memory.topics", "keywords", &[part.clone()])) { let mut word: Py = __item.clone(); __out.push(pyrt::xrefs::xcall("memory.topics", "stem", &[word.clone()])); } py_set(__out) })() };
    score = call_builtin("len", &[op_bitand(&here.clone(), &call_builtin("set", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    stands = if py_truthy(&mcall(&g__A_CONTINUATION(), "match", &[part.clone()])) { Py::Int(0) } else { Py::Int(1) };
    let _ = mcall(&scored.clone(), "append", &[py_tuple(vec![score.clone(), stands.clone(), index.clone()])]);
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let answer = answer.clone(); let best = best.clone(); let flat_before = flat_before.clone(); let here = here.clone(); let i = i.clone(); let index = index.clone(); let out = out.clone(); let part = part.clone(); let parts = parts.clone(); let said_before = said_before.clone(); let score = score.clone(); let scored = scored.clone(); let stands = stands.clone(); let wanted = wanted.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let answer = answer.clone(); let best = best.clone(); let flat_before = flat_before.clone(); let here = here.clone(); let i = i.clone(); let index = index.clone(); let out = out.clone(); let part = part.clone(); let parts = parts.clone(); let said_before = said_before.clone(); let score = score.clone(); let scored = scored.clone(); let stands = stands.clone(); let wanted = wanted.clone(); let word = word.clone(); let triple: Py = __kwp(args, 0, "triple").unwrap_or(Py::None); py_tuple(vec![op_neg(&geti(&triple.clone(), &Py::Int(0))), op_neg(&geti(&triple.clone(), &Py::Int(1))), geti(&triple.clone(), &Py::Int(2))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scored.clone(), "sort", args))(&__args) };
    best = geti(&scored.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = geti(&best.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { geti(&best.clone(), &Py::Int(1)) } }) {
    index = geti(&best.clone(), &Py::Int(2));
    } else {
    if py_truthy(&geti(&best.clone(), &Py::Int(0))) {
    index = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8876 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk8876[0].clone(); let mut part: Py = __unpk8876[1].clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_CONTINUATION(), "match", &[part.clone()])))) {  __out.push(i.clone()); } } py_list(__out) })() }, geti(&best.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e));
    } else {
    return py_str("");
    }
    }
    out = geti(&parts.clone(), &index.clone());
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&out.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT()); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    out = op_add(&op_add(&out.clone(), &py_str(" ")), &geti(&parts.clone(), &op_add(&index.clone(), &Py::Int(1))));
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&out.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST())) {
    out = op_add(&mcall(&mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&out.clone(), "split", &[]), &Py::None, &g_MOST(), &Py::None)]), "rstrip", &[py_str(" ,;:")]), &py_str("."));
    }
    flat_before = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = said_before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = flat_before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat_before.clone(), &slice_or_raise(&mcall(&py_str(" "), "join", &[mcall(&mcall(&out.clone(), "lower", &[]), "split", &[])]), &Py::None, &Py::Int(60), &Py::None))) } }) {
    return py_str("");
    }
    return out.clone();
    Py::None
}
pub fn the_answer_this_chat_gave(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut just_said: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The earlier answer of this chat about the thing the line names, as the return it is.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    wanted = _the_words_asked(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    answer = pyrt::xrefs::xcall("memory.topics", "answer_about", &[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    said_before = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    part = _the_part_that_answers(&[answer.clone(), wanted.clone(), said_before.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&part.clone(), "lower", &[]), "split", &[])]);
    just_said = if py_truthy(&turns.clone()) { mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]) } else { py_str("") };
    if py_truthy(&{ let __b = slice_or_raise(&said.clone(), &Py::None, &Py::Int(60), &Py::None); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&just_said.clone(), &slice_or_raise(&said.clone(), &Py::None, &Py::Int(60), &Py::None))) } }) {
    return Py::None;
    }
    frame = if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&g_RETURN(), &op_modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_RETURN()]).unwrap_or_else(|e| exc::raise(e)))) } else { geti(&g_RETURN_UNNAMED(), &op_modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[g_RETURN_UNNAMED()]).unwrap_or_else(|e| exc::raise(e)))) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), geti(&wanted.clone(), &Py::Int(0))), ("reply".to_string(), part.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.asked_about".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "MOST" => Some(g_MOST()),
            "RETURN" => Some(g_RETURN()),
            "RETURN_UNNAMED" => Some(g_RETURN_UNNAMED()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SHORT" => Some(g_SHORT()),
            "_A_CONTINUATION" => Some(g__A_CONTINUATION()),
            "_NOT_A_THING" => Some(g__NOT_A_THING()),
            "_sentences" => Some(py_func("_sentences", |args: &[Py]| _sentences(args))),
            "_the_part_that_answers" => Some(py_func("_the_part_that_answers", |args: &[Py]| _the_part_that_answers(args))),
            "_the_words_asked" => Some(py_func("_the_words_asked", |args: &[Py]| _the_words_asked(args))),
            "the_answer_this_chat_gave" => Some(py_func("the_answer_this_chat_gave", |args: &[Py]| the_answer_this_chat_gave(args))),
            "topics" => Some(pyrt::xrefs::xmodule("memory.topics")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "MOST" => s_MOST(v),
            "RETURN" => s_RETURN(v),
            "RETURN_UNNAMED" => s_RETURN_UNNAMED(v),
            "SENTENCE" => s_SENTENCE(v),
            "SHORT" => s_SHORT(v),
            "_A_CONTINUATION" => s__A_CONTINUATION(v),
            "_NOT_A_THING" => s__NOT_A_THING(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.asked_about", module_obj);
    pyrt::xrefs::reg_fn("memory.asked_about", "_sentences", py_func("_sentences", |args: &[Py]| _sentences(args)));
    pyrt::xrefs::reg_fn("memory.asked_about", "_the_words_asked", py_func("_the_words_asked", |args: &[Py]| _the_words_asked(args)));
    pyrt::xrefs::reg_fn("memory.asked_about", "_the_part_that_answers", py_func("_the_part_that_answers", |args: &[Py]| _the_part_that_answers(args)));
    pyrt::xrefs::reg_fn("memory.asked_about", "the_answer_this_chat_gave", py_func("the_answer_this_chat_gave", |args: &[Py]| the_answer_this_chat_gave(args)));
    pyrt::xrefs::reg_access("memory.asked_about", "MOST", g_MOST, s_MOST);
    pyrt::xrefs::reg_access("memory.asked_about", "RETURN", g_RETURN, s_RETURN);
    pyrt::xrefs::reg_access("memory.asked_about", "RETURN_UNNAMED", g_RETURN_UNNAMED, s_RETURN_UNNAMED);
    pyrt::xrefs::reg_access("memory.asked_about", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.asked_about", "SHORT", g_SHORT, s_SHORT);
    pyrt::xrefs::reg_access("memory.asked_about", "_A_CONTINUATION", g__A_CONTINUATION, s__A_CONTINUATION);
    pyrt::xrefs::reg_access("memory.asked_about", "_NOT_A_THING", g__NOT_A_THING, s__NOT_A_THING);
}

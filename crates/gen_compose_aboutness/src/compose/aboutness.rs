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


thread_local! { static G_CHANGES_THE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RESTATEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.aboutness: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_CHANGES_THE_SUBJECT() -> Py { ensure_init(); G_CHANGES_THE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGES_THE_SUBJECT(v: Py) { ensure_init(); G_CHANGES_THE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_FRAME() -> Py { ensure_init(); G_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FRAME(v: Py) { ensure_init(); G_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_HEDGE() -> Py { ensure_init(); G_HEDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEDGE(v: Py) { ensure_init(); G_HEDGE.with(|c| *c.borrow_mut() = v); }
pub fn g_RESTATEMENT() -> Py { ensure_init(); G_RESTATEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RESTATEMENT(v: Py) { ensure_init(); G_RESTATEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_STEM() -> Py { ensure_init(); G_STEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEM(v: Py) { ensure_init(); G_STEM.with(|c| *c.borrow_mut() = v); }
pub fn g__WORDS() -> Py { ensure_init(); G__WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORDS(v: Py) { ensure_init(); G__WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Is a reply about the thing the line is about? One reading, used by the router and the arbiter.\n\nSTYLE.md asks for a reply that answers what the person asked, and the whole routing lane rests on\none test of that: **the reply says something the line is about**. A stock line (\"I'm listening —\nsay more if it helps.\", \"Nice one. How did the rest of it go?\") says nothing about the line at all,\nso it reads as a reply only while the person happens to be talking about themselves in general.\n\nTwo things are deliberately *not* counted as saying something about the line:\n\n* a **restatement**, where the reply is the line given back (60% of its words or more). STYLE.md\n  calls that a restatement, and it names the subject without answering it.\n* a **hedge** (\"That's beyond what I know well, sorry\", \"I don't know enough to answer that\n  honestly\"), which is about the bot's knowledge, not about the line.\n\nThe words a line is about are its content words with the frame taken out, so \"i went shopping and\nhated every minute\" is about shopping and minutes. Two words are the same word when the first\nfive letters agree, so `plumber`/`plumbers` and `move`/`moving` count as one.\n\nThis is a reading, not a verdict: `unlike_the_line` says the reply names none of the line's words.\nA caller decides what to do with that, and the one-word answer is never enough on its own — see\n`the_better_answer`, which needs two.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.aboutness.FRAME"); }
    s_FRAME(call_builtin("frozenset", &[mcall(&py_str("\nabout after again all also and any anything are around because been before being both but can\ncould did does doing done down each even ever every for from get gets getting give given go goes\ngoing gone got had has have having here him his her how into its just keep kept know knew like\nlong made make many may might more most much must need needed not now off once one only other our\nout over own said same say says she should since some something still such take taken tell than\nthat thats the their them then there these they thing things think this those through time times\ntoday told tomorrow tonight too took under until very want wanted was way well went were what\nwhen where which while who whom why will with wont would yeah yes yet you your yours yourself\nim ive ill id youre dont doesnt didnt cant couldnt wont isnt arent wasnt werent havent hasnt\nhello hi hey thanks thank please sorry ok okay right sure fine good nice bad lot bit put puts\nknows mean means meant seem seems felt feel feels look looks looking sound sounds\nreally stopping\nanyway anyways btw honestly seriously basically literally actually whatever\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.aboutness.HEDGE"); }
    s_HEDGE(stdlib_call("re", "compile", &[py_str("beyond what i know|i don't know enough|not sure about that one|a search should"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.aboutness.STEM"); }
    s_STEM(Py::Int(5));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.aboutness.RESTATEMENT"); }
    s_RESTATEMENT(Py::Float(0.6));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.aboutness._WORDS"); }
    s__WORDS(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.aboutness.CHANGES_THE_SUBJECT"); }
    s_CHANGES_THE_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:anyway|anyways|any\\s+way|back\\s+to|on\\s+a\\s+different\\s+note|on\\s+another\\s+note|changing\\s+the\\s+subject|change\\s+of\\s+subject|changing\\s+tack|while\\s+i'?m\\s+here|unrelated\\s+but|that\\s+aside|moving\\s+on|by\\s+the\\s+way|btw|different\\s+question|other\\s+thing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn after_a_change_of_subject(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("What the line is about after the marker that puts the earlier subject down, or ''.\n\n    \"im fine really, anyway whats a good way to sell her furniture\" is a furniture question: the\n    first half is the thread they are leaving. Returned only when the tail carries words of its\n    own, because a person who says \"anyway\" and nothing else has changed no subject yet.\n    ");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    tail = py_str("");
    let __iter1 = py_iter_or_raise(&mcall(&g_CHANGES_THE_SUBJECT(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    tail = slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None);
    }
    tail = mcall(&tail.clone(), "strip", &[py_str(" \t,.;:!-—-")]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&tail.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    return tail.clone();
    Py::None
}
pub fn subject_words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The words the line is about, as a set: content words minus the frame and the light verbs.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__WORDS(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FRAME(), &word.clone())) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn names_the_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut about: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "about") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: about"))) };
    let mut found: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Which of `about` the reply actually says, by shared stem.");
    said = mcall(&g__WORDS(), "findall", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    found = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&about.clone());
    for __item in __iter1 {
    word = __item.clone();
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    other = __item.clone();
    if py_truthy(&{ let __b = op_ge(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&other.clone(), &Py::None, &g_STEM(), &Py::None), &slice_or_raise(&word.clone(), &Py::None, &g_STEM(), &Py::None))) } }) {
    let _ = mcall(&found.clone(), "add", &[word.clone()]);
    break;
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn restates_the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut mine: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the reply is only the line given back, which names nothing by answering nothing.");
    said = call_builtin("set", &[mcall(&g__WORDS(), "findall", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    mine = call_builtin("set", &[mcall(&g__WORDS(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&op_truediv(&call_builtin("len", &[op_bitand(&said.clone(), &mine.clone())]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))), &g_RESTATEMENT()) } };
    Py::None
}
pub fn is_a_hedge(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let _ = py_str("True when the reply is about the bot's knowledge rather than the line.");
    return call_builtin("bool", &[mcall(&g_HEDGE(), "search", &[call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn unlike_the_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut about: Py = Py::None;
    let _ = py_str("`(about, said)`: the words the line is about, and the ones the reply says.\n\n    An empty `about` (a line with nothing but frame words) has no subject to be unlike, and a\n    restatement or a hedge names nothing that counts, so both come back with an empty `said`.\n    ");
    about = subject_words(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&about.clone()))) {
    return py_tuple(vec![about.clone(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&{ let __b = restates_the_line(&[reply.clone(), text.clone()]); if py_truthy(&__b) { __b } else { is_a_hedge(&[reply.clone()]) } }) {
    return py_tuple(vec![about.clone(), call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return py_tuple(vec![about.clone(), names_the_subject(&[reply.clone(), about.clone()])]);
    Py::None
}
pub fn the_better_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut candidates: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut skip: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { Py::None };
    let mut need: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "need") { v.clone() }
    else { Py::Int(2) };
    let mut about: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_named: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut count: Py = Py::None;
    let mut index: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The candidate that names two or more of the line's words, when `reply` names none.\n\n    `candidates` is the arbiter's list of proposals (`{\"module\", \"priority\", \"reply\"}`), `skip`\n    the index of `reply`'s own candidate. Returns the index of the closer answer, or None. Two\n    words, not one: a single shared word is how a stock line lands on a line about something else\n    in the first place, so one word is not evidence that a reply is about the subject.\n\n    `need` is the count the other reply has to reach. It is 2 for a general comparison, where one\n    shared word is what a coincidence looks like, and 1 for a module that overrides the entries\n    (`ANSWERS_INSTEAD`): that override rests on having read the whole line, so a reply that names\n    nothing the line is about has not shown the reading the claim was made for.\n    ");
    let __unpk1 = py_unpack(&unlike_the_line(&[reply.clone(), text.clone()]), 2, None);
    about = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&about.clone())); if py_truthy(&__b) { __b } else { said.clone() } }) {
    return Py::None;
    }
    best = Py::None;
    best_named = op_sub(&need.clone(), &Py::Int(1));
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[{ let __b = candidates.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    candidate = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &skip.clone()))) {
    continue;
    }
    other = call_builtin("str", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("reply")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("int", &[{ let __b = mcall(&candidate.clone(), "get", &[py_str("priority")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0))) {
    continue;
    }
    if py_truthy(&{ let __b = restates_the_line(&[other.clone(), text.clone()]); if py_truthy(&__b) { __b } else { is_a_hedge(&[other.clone()]) } }) {
    continue;
    }
    count = call_builtin("len", &[names_the_subject(&[other.clone(), about.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&count.clone(), &best_named.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![index.clone(), count.clone()]), 2, None);
    best = __unpk4[0].clone();
    best_named = __unpk4[1].clone();
    }
    }
    return best.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.aboutness".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CHANGES_THE_SUBJECT" => Some(g_CHANGES_THE_SUBJECT()),
            "FRAME" => Some(g_FRAME()),
            "HEDGE" => Some(g_HEDGE()),
            "RESTATEMENT" => Some(g_RESTATEMENT()),
            "STEM" => Some(g_STEM()),
            "_WORDS" => Some(g__WORDS()),
            "after_a_change_of_subject" => Some(py_func("after_a_change_of_subject", |args: &[Py]| after_a_change_of_subject(args))),
            "is_a_hedge" => Some(py_func("is_a_hedge", |args: &[Py]| is_a_hedge(args))),
            "names_the_subject" => Some(py_func("names_the_subject", |args: &[Py]| names_the_subject(args))),
            "restates_the_line" => Some(py_func("restates_the_line", |args: &[Py]| restates_the_line(args))),
            "subject_words" => Some(py_func("subject_words", |args: &[Py]| subject_words(args))),
            "the_better_answer" => Some(py_func("the_better_answer", |args: &[Py]| the_better_answer(args))),
            "unlike_the_line" => Some(py_func("unlike_the_line", |args: &[Py]| unlike_the_line(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CHANGES_THE_SUBJECT" => s_CHANGES_THE_SUBJECT(v),
            "FRAME" => s_FRAME(v),
            "HEDGE" => s_HEDGE(v),
            "RESTATEMENT" => s_RESTATEMENT(v),
            "STEM" => s_STEM(v),
            "_WORDS" => s__WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.aboutness", module_obj);
    pyrt::xrefs::reg_fn("compose.aboutness", "after_a_change_of_subject", py_func("after_a_change_of_subject", |args: &[Py]| after_a_change_of_subject(args)));
    pyrt::xrefs::reg_fn("compose.aboutness", "subject_words", py_func("subject_words", |args: &[Py]| subject_words(args)));
    pyrt::xrefs::reg_fn("compose.aboutness", "names_the_subject", py_func("names_the_subject", |args: &[Py]| names_the_subject(args)));
    pyrt::xrefs::reg_fn("compose.aboutness", "restates_the_line", py_func("restates_the_line", |args: &[Py]| restates_the_line(args)));
    pyrt::xrefs::reg_fn("compose.aboutness", "is_a_hedge", py_func("is_a_hedge", |args: &[Py]| is_a_hedge(args)));
    pyrt::xrefs::reg_fn("compose.aboutness", "unlike_the_line", py_func("unlike_the_line", |args: &[Py]| unlike_the_line(args)));
    pyrt::xrefs::reg_fn("compose.aboutness", "the_better_answer", py_func("the_better_answer", |args: &[Py]| the_better_answer(args)));
    pyrt::xrefs::reg_access("compose.aboutness", "CHANGES_THE_SUBJECT", g_CHANGES_THE_SUBJECT, s_CHANGES_THE_SUBJECT);
    pyrt::xrefs::reg_access("compose.aboutness", "FRAME", g_FRAME, s_FRAME);
    pyrt::xrefs::reg_access("compose.aboutness", "HEDGE", g_HEDGE, s_HEDGE);
    pyrt::xrefs::reg_access("compose.aboutness", "RESTATEMENT", g_RESTATEMENT, s_RESTATEMENT);
    pyrt::xrefs::reg_access("compose.aboutness", "STEM", g_STEM, s_STEM);
    pyrt::xrefs::reg_access("compose.aboutness", "_WORDS", g__WORDS, s__WORDS);
}

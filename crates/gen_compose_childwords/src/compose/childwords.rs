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


thread_local! { static G_A_CHILD_DIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHILD_IN_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEATH_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEATH_IN_THE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUICIDE_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_ABOUT_THE_ONE_WHO_DIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAIL_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_WORDS_FOR_A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON_IN_THE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.childwords: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_A_CHILD_DIED() -> Py { ensure_init(); G_A_CHILD_DIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_DIED(v: Py) { ensure_init(); G_A_CHILD_DIED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHILD_IN_THE_ASK() -> Py { ensure_init(); G_A_CHILD_IN_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHILD_IN_THE_ASK(v: Py) { ensure_init(); G_A_CHILD_IN_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEATH_IN_THE_LINE() -> Py { ensure_init(); G_A_DEATH_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEATH_IN_THE_LINE(v: Py) { ensure_init(); G_A_DEATH_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEATH_IN_THE_TAIL() -> Py { ensure_init(); G_A_DEATH_IN_THE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEATH_IN_THE_TAIL(v: Py) { ensure_init(); G_A_DEATH_IN_THE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUICIDE_IN_THE_LINE() -> Py { ensure_init(); G_A_SUICIDE_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUICIDE_IN_THE_LINE(v: Py) { ensure_init(); G_A_SUICIDE_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_ABOUT_THE_ONE_WHO_DIED() -> Py { ensure_init(); G_TAIL_ABOUT_THE_ONE_WHO_DIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_ABOUT_THE_ONE_WHO_DIED(v: Py) { ensure_init(); G_TAIL_ABOUT_THE_ONE_WHO_DIED.with(|c| *c.borrow_mut() = v); }
pub fn g_TAIL_FILLER() -> Py { ensure_init(); G_TAIL_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAIL_FILLER(v: Py) { ensure_init(); G_TAIL_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_TELL_ASK() -> Py { ensure_init(); G_TELL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELL_ASK(v: Py) { ensure_init(); G_TELL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_WORDS_FOR_A_CHILD() -> Py { ensure_init(); G_THE_WORDS_FOR_A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_WORDS_FOR_A_CHILD(v: Py) { ensure_init(); G_THE_WORDS_FOR_A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON_IN_THE_TAIL() -> Py { ensure_init(); G__A_PERSON_IN_THE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON_IN_THE_TAIL(v: Py) { ensure_init(); G__A_PERSON_IN_THE_TAIL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The words for a child, asked for one turn after the death.\n\n`my husband died this morning and i have to tell the kids` is answered by the composer's pool for\nthe first hour (the condolence, then the one thing worth knowing). The person then asks the\nfollow-up every parent asks - \"how do i tell a 4 year old\", \"what do i say to my son\" - and\nwithout this reading the line goes to whatever else matches the child in it: the card on screen\ntime (\"Match the limit to the age: under two, video calls and the odd photo only\"), the search\nhedge, or the drafter that asks them to send the gist so it can be written up. None of those is\nthe words.\n\nTwo callers need the same reading, which is why it lives here on its own:\n\n* bot/compose/fallback.py answers the ask from `THE_WORDS_FOR_A_CHILD`;\n* bot/modules/arbiter.py gives it the turn over the card that matched the age, because a card\n  about screen time answers how many hours and this turn is \"what do i say\".\n\nThe reading is narrow on purpose: it needs the ask shape, a child named in the ask, and a death\nin the thread the bot has just been condoled about. The same ask in an ordinary conversation\n(\"we are moving in june\" / \"how do i tell my son\") keeps its card, and a line that names its own\nsubject (\"how do i tell my son that i lost my job\") keeps its own answer.\n");
    s_A_CHILD_DIED(pyrt::xrefs::xget("compose.fallback", "A_CHILD_DIED"));
    s_A_DEATH_IN_THE_LINE(pyrt::xrefs::xget("compose.fallback", "A_DEATH_IN_THE_LINE"));
    s_A_SUICIDE_IN_THE_LINE(pyrt::xrefs::xget("compose.fallback", "A_SUICIDE_IN_THE_LINE"));
    s_TELL_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how|what)\\s+(?:do|should|can|would|will)\\s+(?:i|we)\\s+(?:tell|say|word|phrase|put it|put)\\b|\\bhow do i put (?:it|this)\\b|\\bwhat do i tell\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHILD_IN_THE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:kid|kids|child|children|son|daughter|boy|boys|girl|girls|toddler|little one|teenager|teen|baby|niece|nephew|grandson|granddaughter|(?:two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2})[- ](?:year|yr)s?[- ]old)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_FILLER(call_builtin("frozenset", &[mcall(&py_str("please now then too ok okay soon gently first properly exactly tonight\ntoday tomorrow at bedtime the a an this that it and but so just really here you i we my"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_ABOUT_THE_ONE_WHO_DIED(call_builtin("frozenset", &[mcall(&py_str("about his her their them of on with dad daddy mum mummy\nmom mommy mother father gran granny grandma grandad grandpa nana nan brother sister uncle aunt dog\ncat pet name what happened truth"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PERSON_IN_THE_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:dad|daddy|mum|mummy|mom|mommy|mother|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|uncle|aunt|dog|cat|pet)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEATH_IN_THE_TAIL(stdlib_call("re", "compile", &[py_str("\\b(?:died|dead|dying|dies|death|passed|passing|gone|killed|funeral|buried|cremated)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WORDS_FOR_A_CHILD(py_tuple(vec![py_str("Keep it to a sentence or two, with their name and the plain word: they died, and they are not coming back. Then answer whatever the child asks, in the same words, and if they ask again next week - they will - say the same thing again."), py_str("Say it plainly and soon: their name, and that they died. Not asleep, not gone away, and not \"lost\" - a child takes those words literally, and then waits for them to wake up. What follows can come from their questions."), py_str("The words are the ones you would use to an adult, just shorter: their name, that they died, and that they will not be coming back. It is all right for the child to see you cry - that is how they learn it is all right to be sad.")]));
}
pub fn _the_tail_after_the_child(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The words after the last child named in the ask, or None when no child is named.");
    last = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_A_CHILD_IN_THE_ASK(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    last = match_.clone();
    }
    if py_truthy(&Py::Bool(py_is(&last.clone(), &Py::None))) {
    return Py::None;
    }
    return slice_or_raise(&text.clone(), &mcall(&last.clone(), "end", &[]), &Py::None, &Py::None);
    Py::None
}
pub fn _the_tail_is_about_the_death(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when what follows the child leaves the ask about the death, and not about something else.");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), { let __b = tail.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g_A_DEATH_IN_THE_TAIL(), "search", &[tail.clone()])) {
    return Py::Bool(!py_truthy(&mcall(&g__A_PERSON_IN_THE_TAIL(), "search", &[tail.clone()])));
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_TAIL_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TAIL_ABOUT_THE_ONE_WHO_DIED(), &word.clone())) } }) {
    continue;
    }
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn a_death_in_the_thread(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(3) };
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the person said a death in one of their last few lines.\n\n    The bot's own last line is not enough to go on: the condolence for \"my mum died yesterday\"\n    comes from another module's pool (bot/compose/loss.py, LOSS_REPLIES), so a death said one\n    turn ago reads as no thread at all when only the composer's own pools are checked, and the\n    ask after it went to the search hedge. The person's own lines say what the conversation is\n    about, and two asks in a row (\"how do i tell a 4 year old\", then \"what about my son\") both\n    still have the death behind them.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("compose.fallback", "A_DEATH_IN_THE_LINE"), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn asks_the_words_for_a_child(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut tail: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thread: Py = Py::None;
    let _ = py_str("A how-do-I-tell ask about a child, in a thread where somebody has died.\n\n    Narrow on purpose. Both halves are needed: the ask shape and the thread. A line inside a\n    loss thread that is not this ask keeps its own answer, and this ask outside a loss thread is\n    an everyday how-to. A suicide among the people they know is left to the safety readings, and\n    a line whose speaker names the child who died is not the same telling (\"my son died and I\n    have to tell my daughter\" is the condolence's turn, which the readings above already give).\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_TELL_ASK(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_CHILD_IN_THE_ASK(), "search", &[text.clone()]) } }))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&a_death_in_the_thread(&[memory.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    thread = _the_last_few_lines(&[memory.clone()]);
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
    if py_truthy(&{ let __b = mcall(&pyrt::xrefs::xget("compose.fallback", "A_SUICIDE_IN_THE_LINE"), "search", &[thread.clone()]); if py_truthy(&__b) { __b } else { mcall(&pyrt::xrefs::xget("compose.fallback", "A_SUICIDE_IN_THE_LINE"), "search", &[text.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("compose.fallback", "A_CHILD_DIED"), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    tail = _the_tail_after_the_child(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&tail.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_tail_is_about_the_death(&[tail.clone()]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_last_few_lines(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(4) };
    let mut lines: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The recent lines of the conversation, the person's and the bot's, as one string.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&look_back.clone()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return mcall(&mcall(&py_str(" "), "join", &[lines.clone()]), "lower", &[]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.childwords".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_CHILD_DIED" => Some(g_A_CHILD_DIED()),
            "A_CHILD_IN_THE_ASK" => Some(g_A_CHILD_IN_THE_ASK()),
            "A_DEATH_IN_THE_LINE" => Some(g_A_DEATH_IN_THE_LINE()),
            "A_DEATH_IN_THE_TAIL" => Some(g_A_DEATH_IN_THE_TAIL()),
            "A_SUICIDE_IN_THE_LINE" => Some(g_A_SUICIDE_IN_THE_LINE()),
            "TAIL_ABOUT_THE_ONE_WHO_DIED" => Some(g_TAIL_ABOUT_THE_ONE_WHO_DIED()),
            "TAIL_FILLER" => Some(g_TAIL_FILLER()),
            "TELL_ASK" => Some(g_TELL_ASK()),
            "THE_WORDS_FOR_A_CHILD" => Some(g_THE_WORDS_FOR_A_CHILD()),
            "_A_PERSON_IN_THE_TAIL" => Some(g__A_PERSON_IN_THE_TAIL()),
            "_the_last_few_lines" => Some(py_func("_the_last_few_lines", |args: &[Py]| _the_last_few_lines(args))),
            "_the_tail_after_the_child" => Some(py_func("_the_tail_after_the_child", |args: &[Py]| _the_tail_after_the_child(args))),
            "_the_tail_is_about_the_death" => Some(py_func("_the_tail_is_about_the_death", |args: &[Py]| _the_tail_is_about_the_death(args))),
            "a_death_in_the_thread" => Some(py_func("a_death_in_the_thread", |args: &[Py]| a_death_in_the_thread(args))),
            "asks_the_words_for_a_child" => Some(py_func("asks_the_words_for_a_child", |args: &[Py]| asks_the_words_for_a_child(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_CHILD_DIED" => s_A_CHILD_DIED(v),
            "A_CHILD_IN_THE_ASK" => s_A_CHILD_IN_THE_ASK(v),
            "A_DEATH_IN_THE_LINE" => s_A_DEATH_IN_THE_LINE(v),
            "A_DEATH_IN_THE_TAIL" => s_A_DEATH_IN_THE_TAIL(v),
            "A_SUICIDE_IN_THE_LINE" => s_A_SUICIDE_IN_THE_LINE(v),
            "TAIL_ABOUT_THE_ONE_WHO_DIED" => s_TAIL_ABOUT_THE_ONE_WHO_DIED(v),
            "TAIL_FILLER" => s_TAIL_FILLER(v),
            "TELL_ASK" => s_TELL_ASK(v),
            "THE_WORDS_FOR_A_CHILD" => s_THE_WORDS_FOR_A_CHILD(v),
            "_A_PERSON_IN_THE_TAIL" => s__A_PERSON_IN_THE_TAIL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.childwords", module_obj);
    pyrt::xrefs::reg_fn("compose.childwords", "_the_tail_after_the_child", py_func("_the_tail_after_the_child", |args: &[Py]| _the_tail_after_the_child(args)));
    pyrt::xrefs::reg_fn("compose.childwords", "_the_tail_is_about_the_death", py_func("_the_tail_is_about_the_death", |args: &[Py]| _the_tail_is_about_the_death(args)));
    pyrt::xrefs::reg_fn("compose.childwords", "a_death_in_the_thread", py_func("a_death_in_the_thread", |args: &[Py]| a_death_in_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.childwords", "asks_the_words_for_a_child", py_func("asks_the_words_for_a_child", |args: &[Py]| asks_the_words_for_a_child(args)));
    pyrt::xrefs::reg_fn("compose.childwords", "_the_last_few_lines", py_func("_the_last_few_lines", |args: &[Py]| _the_last_few_lines(args)));
    pyrt::xrefs::reg_access("compose.childwords", "A_CHILD_DIED", g_A_CHILD_DIED, s_A_CHILD_DIED);
    pyrt::xrefs::reg_access("compose.childwords", "A_CHILD_IN_THE_ASK", g_A_CHILD_IN_THE_ASK, s_A_CHILD_IN_THE_ASK);
    pyrt::xrefs::reg_access("compose.childwords", "A_DEATH_IN_THE_LINE", g_A_DEATH_IN_THE_LINE, s_A_DEATH_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.childwords", "A_DEATH_IN_THE_TAIL", g_A_DEATH_IN_THE_TAIL, s_A_DEATH_IN_THE_TAIL);
    pyrt::xrefs::reg_access("compose.childwords", "A_SUICIDE_IN_THE_LINE", g_A_SUICIDE_IN_THE_LINE, s_A_SUICIDE_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.childwords", "TAIL_ABOUT_THE_ONE_WHO_DIED", g_TAIL_ABOUT_THE_ONE_WHO_DIED, s_TAIL_ABOUT_THE_ONE_WHO_DIED);
    pyrt::xrefs::reg_access("compose.childwords", "TAIL_FILLER", g_TAIL_FILLER, s_TAIL_FILLER);
    pyrt::xrefs::reg_access("compose.childwords", "TELL_ASK", g_TELL_ASK, s_TELL_ASK);
    pyrt::xrefs::reg_access("compose.childwords", "THE_WORDS_FOR_A_CHILD", g_THE_WORDS_FOR_A_CHILD, s_THE_WORDS_FOR_A_CHILD);
    pyrt::xrefs::reg_access("compose.childwords", "_A_PERSON_IN_THE_TAIL", g__A_PERSON_IN_THE_TAIL, s__A_PERSON_IN_THE_TAIL);
}

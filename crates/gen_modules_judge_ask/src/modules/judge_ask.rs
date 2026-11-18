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


thread_local! { static G_EMPTY_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_A_THING_NOT_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_BY_A_STANDARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_PEOPLE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUDGE_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_EMPTY_HEADS() -> Py { ensure_init(); G_EMPTY_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPTY_HEADS(v: Py) { ensure_init(); G_EMPTY_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_A_THING_NOT_PEOPLE() -> Py { ensure_init(); G_JUDGE_A_THING_NOT_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_A_THING_NOT_PEOPLE(v: Py) { ensure_init(); G_JUDGE_A_THING_NOT_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_BY_A_STANDARD() -> Py { ensure_init(); G_JUDGE_BY_A_STANDARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_BY_A_STANDARD(v: Py) { ensure_init(); G_JUDGE_BY_A_STANDARD.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_PEOPLE_ASK() -> Py { ensure_init(); G_JUDGE_PEOPLE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_PEOPLE_ASK(v: Py) { ensure_init(); G_JUDGE_PEOPLE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_JUDGE_SOMEONE_ELSE() -> Py { ensure_init(); G_JUDGE_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUDGE_SOMEONE_ELSE(v: Py) { ensure_init(); G_JUDGE_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_LINES() -> Py { ensure_init(); G_PEOPLE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_LINES(v: Py) { ensure_init(); G_PEOPLE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line asking whether this bot judges people is answered, not searched.\n\n\"do you judge people\" and \"do you judge people by their past\" were answered \"That's beyond what I\nknow well, sorry. Searching for it will turn up better sources than I can offer\" - an answer about\na search engine for a question about the bot. \"are you judging them\" the same. Nobody owned it:\nthe about-the-bot reader stops short of the judging word, and the composer's own judging pool is\nwritten for the person being told (\"Not judging, no. Say it plainly\"), not for a question about\npeople in general.\n\nThe reading is narrow on purpose, and it is a floor. On today's main the composer already answers\nthe plain plea to the bot (\"dont judge me\", \"will you judge me\", \"please dont judge me for it\") and\nthe person who holds the judgement (\"i feel like everyone is judging me\", \"my mum would judge me\"),\nso this module is not asked about them at all: it reads only the bot being asked what it makes of\npeople, and it asks the composer first, so a line the composer answers whole keeps its turn.\n\nAnything else stands down and goes on to whatever answered it before: a thing being judged (\"dont\njudge the cake by its look\", \"do you judge a book by its cover\"), a court (\"the judge said i have to\ngo to court\"), a standard of judgement with no people in it (\"judging by the weather\"), and somebody\nelse holding the judgement (\"are people judging me\").\n");
    s_PRIORITY(op_neg(&Py::Int(500)));
    s_JUDGE_PEOPLE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:will|would|do|did|are|r|can|could)\\s+(?:you|u)\\s+(?:just\\s+|really\\s+|actually\\s+|even\\s+|ever\\s+|not\\s+)?judg\\w*\\s+(?:(?:a|the|other)\\s+)?(?:people|person|persons|someone|somebody|anyone|anybody|folk|folks|others|them|him|her)\\b|\\b(?:dont|do not|never)\\s+judg\\w*\\s+(?:(?:a|the|other)\\s+)?(?:people|anyone|anybody|someone|somebody|person|folk|folks)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_A_THING_NOT_PEOPLE(stdlib_call("re", "compile", &[py_str("\\bjudg\\w*\\s+(?:the|this|that|it|my|his|its)\\b|\\bjudg\\w*\\s+(?:a|an)\\s+(?!person\\b|people\\b|human\\b|child\\b|kid\\b|man\\b|woman\\b|guy\\b|girl\\b|boy\\b|teen\\w*\\b)\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_BY_A_STANDARD(stdlib_call("re", "compile", &[py_str("\\bjudg(?:ing|ed)\\s+(?:by|on|from)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUDGE_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:everyone|everybody|people|they|them|someone|somebody|nobody|no one|parents|friends|family|colleagues|strangers|relatives|neighbours)\\b(?:\\s+\\w+){0,3}\\s+judg\\w*|\\b(?:my|his|her|their|our|your)\\s+(?:\\w+\\s+){0,2}judg\\w*|\\b(?:she|he|they|it|shell|hell|theyll|theyd|shed|shes|hes)\\b(?:\\s+\\w+){0,3}\\s+judg\\w*|\\b(?:the|a|my|his|her|our|their|your)\\s+(?:\\w+\\s+){0,2}to\\s+judg\\w*\\s+(?:me|us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE_LINES(py_tuple(vec![py_str("I try not to. Nobody gets a verdict off me - what people tell me stays here. What's on your mind?"), py_str("No. I've no interest in scoring people - say whatever you came to say."), py_str("I don't, no. Whatever you tell me I take as it comes, so where are we starting?")]));
    s_EMPTY_HEADS(py_tuple(vec![py_str("got it"), py_str("noted:"), py_str("thanks for telling me"), py_str("i'm listening"), py_str("i am listening"), py_str("that's your call"), py_str("that sounds like a lot"), py_str("sorry, i don't know"), py_str("i don't know enough")]));
}
pub fn _says_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply is one of the pools that tell the person nothing.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.fallback", "_is_a_hedge", &[said.clone()])) {
    return Py::Bool(true);
    }
    return mcall(&mcall(&said.clone(), "lower", &[]), "startswith", &[g_EMPTY_HEADS()]);
    Py::None
}
pub fn _the_composer_says_nothing(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let _ = py_str("True when the composer's own answer for this line is a nod, a receipt, a hedge or an echo.\n\n    The composer reads the thread and answers most judging lines better than a reassurance pool\n    (\"i feel like a bad mum, dont judge me\" gets the card about saying the one thing plainly), so\n    its answer is asked first and this module only fills the turns where it says nothing at all.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![pyrt::xrefs::xcall("modules.greeting", "respond", &[text.clone(), memory.clone()]), py_call(&pyrt::xrefs::xget("compose.fallback", "reply_for"), &[text.clone(), memory.clone()])]));
    for __item in __iter1 {
    reply = __item.clone();
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_nothing(&[reply.clone()]))) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn reads_it(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True for the lines this module reads whole: the bot asked what it makes of people.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_JUDGE_PEOPLE_ASK(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g_JUDGE_A_THING_NOT_PEOPLE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_JUDGE_BY_A_STANDARD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_JUDGE_SOMEONE_ELSE(), "search", &[said.clone()]) } } } }));
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
    let mut ch: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut start: Py = Py::None;
    let mut step: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One of the people lines, never the one the bot has just said, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&reads_it(&[text.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_composer_says_nothing(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    start = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])) { let mut ch: Py = __item.clone(); __out.push(call_builtin("ord", &[ch.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[g_PEOPLE_LINES()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    step = __item.clone();
    line = geti(&g_PEOPLE_LINES(), &op_modulo(&op_add(&start.clone(), &step.clone()), &call_builtin("len", &[g_PEOPLE_LINES()]).unwrap_or_else(|e| exc::raise(e))));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&last.clone(), &slice_or_raise(&line.clone(), &Py::None, &Py::Int(24), &Py::None))) } }) {
    return line.clone();
    }
    }
    return geti(&g_PEOPLE_LINES(), &op_modulo(&start.clone(), &call_builtin("len", &[g_PEOPLE_LINES()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.judge_ask".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "EMPTY_HEADS" => Some(g_EMPTY_HEADS()),
            "JUDGE_A_THING_NOT_PEOPLE" => Some(g_JUDGE_A_THING_NOT_PEOPLE()),
            "JUDGE_BY_A_STANDARD" => Some(g_JUDGE_BY_A_STANDARD()),
            "JUDGE_PEOPLE_ASK" => Some(g_JUDGE_PEOPLE_ASK()),
            "JUDGE_SOMEONE_ELSE" => Some(g_JUDGE_SOMEONE_ELSE()),
            "PEOPLE_LINES" => Some(g_PEOPLE_LINES()),
            "PRIORITY" => Some(g_PRIORITY()),
            "_says_nothing" => Some(py_func("_says_nothing", |args: &[Py]| _says_nothing(args))),
            "_the_composer_says_nothing" => Some(py_func("_the_composer_says_nothing", |args: &[Py]| _the_composer_says_nothing(args))),
            "reads_it" => Some(py_func("reads_it", |args: &[Py]| reads_it(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "EMPTY_HEADS" => s_EMPTY_HEADS(v),
            "JUDGE_A_THING_NOT_PEOPLE" => s_JUDGE_A_THING_NOT_PEOPLE(v),
            "JUDGE_BY_A_STANDARD" => s_JUDGE_BY_A_STANDARD(v),
            "JUDGE_PEOPLE_ASK" => s_JUDGE_PEOPLE_ASK(v),
            "JUDGE_SOMEONE_ELSE" => s_JUDGE_SOMEONE_ELSE(v),
            "PEOPLE_LINES" => s_PEOPLE_LINES(v),
            "PRIORITY" => s_PRIORITY(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.judge_ask", module_obj);
    pyrt::xrefs::reg_fn("modules.judge_ask", "_says_nothing", py_func("_says_nothing", |args: &[Py]| _says_nothing(args)));
    pyrt::xrefs::reg_fn("modules.judge_ask", "_the_composer_says_nothing", py_func("_the_composer_says_nothing", |args: &[Py]| _the_composer_says_nothing(args)));
    pyrt::xrefs::reg_fn("modules.judge_ask", "reads_it", py_func("reads_it", |args: &[Py]| reads_it(args)));
    pyrt::xrefs::reg_fn("modules.judge_ask", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_access("modules.judge_ask", "EMPTY_HEADS", g_EMPTY_HEADS, s_EMPTY_HEADS);
    pyrt::xrefs::reg_access("modules.judge_ask", "JUDGE_A_THING_NOT_PEOPLE", g_JUDGE_A_THING_NOT_PEOPLE, s_JUDGE_A_THING_NOT_PEOPLE);
    pyrt::xrefs::reg_access("modules.judge_ask", "JUDGE_BY_A_STANDARD", g_JUDGE_BY_A_STANDARD, s_JUDGE_BY_A_STANDARD);
    pyrt::xrefs::reg_access("modules.judge_ask", "JUDGE_PEOPLE_ASK", g_JUDGE_PEOPLE_ASK, s_JUDGE_PEOPLE_ASK);
    pyrt::xrefs::reg_access("modules.judge_ask", "JUDGE_SOMEONE_ELSE", g_JUDGE_SOMEONE_ELSE, s_JUDGE_SOMEONE_ELSE);
    pyrt::xrefs::reg_access("modules.judge_ask", "PEOPLE_LINES", g_PEOPLE_LINES, s_PEOPLE_LINES);
    pyrt::xrefs::reg_access("modules.judge_ask", "PRIORITY", g_PRIORITY, s_PRIORITY);
}

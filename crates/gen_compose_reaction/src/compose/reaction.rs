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


thread_local! { static G_AWE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BODY_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARE_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBT_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_STANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHRASES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_REACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.reaction: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AWE_REPLIES() -> Py { ensure_init(); G_AWE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AWE_REPLIES(v: Py) { ensure_init(); G_AWE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BODY_LINE() -> Py { ensure_init(); G_A_BODY_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_LINE(v: Py) { ensure_init(); G_A_BODY_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARE_ANSWER() -> Py { ensure_init(); G_A_CARE_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARE_ANSWER(v: Py) { ensure_init(); G_A_CARE_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_REPLIES() -> Py { ensure_init(); G_CARE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_REPLIES(v: Py) { ensure_init(); G_CARE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK() -> Py { ensure_init(); G_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK(v: Py) { ensure_init(); G_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_REPLIES() -> Py { ensure_init(); G_CHECK_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_REPLIES(v: Py) { ensure_init(); G_CHECK_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBT_REPLIES() -> Py { ensure_init(); G_DOUBT_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBT_REPLIES(v: Py) { ensure_init(); G_DOUBT_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_ANSWER() -> Py { ensure_init(); G_NO_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_ANSWER(v: Py) { ensure_init(); G_NO_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_STANCE() -> Py { ensure_init(); G_NO_STANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_STANCE(v: Py) { ensure_init(); G_NO_STANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PLAN() -> Py { ensure_init(); G_OWN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PLAN(v: Py) { ensure_init(); G_OWN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PHRASES() -> Py { ensure_init(); G_PHRASES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHRASES(v: Py) { ensure_init(); G_PHRASES.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_REACTION() -> Py { ensure_init(); G_THEIR_REACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_REACTION(v: Py) { ensure_init(); G_THEIR_REACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK() -> Py { ensure_init(); G__ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK(v: Py) { ensure_init(); G__ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__TAIL() -> Py { ensure_init(); G__TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAIL(v: Py) { ensure_init(); G__TAIL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that reacts to the answer just given: \"wow\", \"really?\", \"seriously\".\n\nA reaction to something the *person* said has its own pools in the composer, and they are right:\n\"ouch\" after a report of a bad day asks what happened. This is the other reaction - the one aimed\nat the bot's own answer. \"how do i poach an egg\" then \"wow\" got \"Alright then — good luck with\nit\", which closes a conversation the person was opening, and \"how do i stop my back aching\" then\n\"really?\" got the same, which leaves somebody who doubts the answer still doubting it.\n\nBoth are the ack pool's turn today: \"wow\" and \"really\" are in its word list, and the pool has no\nway to see what they were reacting to. So this module reads only that - the line is nothing but\none of these reactions, and the reply above it was an answer and not a line of the bot's own -\nand hands back a reply that answers the answer. It says nothing at all otherwise, which keeps\n\"wow\" after a plan on \"Good plan. What sparked it?\" and \"wow, that is a lot\" on its own reading.\n\nRun it alone:\n\n    python -m bot.compose.reaction \"wow\" --after \"Slide it into barely moving water for three.\"\n");
    fn __data1() -> Py { py_dict(vec![(py_str("wow"), py_str("awe")), (py_str("woah"), py_str("awe")), (py_str("whoa"), py_str("awe")), (py_str("wowsers"), py_str("awe")), (py_str("wowee"), py_str("awe")), (py_str("omg"), py_str("awe")), (py_str("gosh"), py_str("awe")), (py_str("blimey"), py_str("awe")), (py_str("crikey"), py_str("awe")), (py_str("cor"), py_str("awe")), (py_str("geez"), py_str("awe")), (py_str("jeez"), py_str("awe")), (py_str("really"), py_str("doubt")), (py_str("seriously"), py_str("doubt")), (py_str("honestly"), py_str("doubt")), (py_str("truly"), py_str("doubt")), (py_str("surely"), py_str("doubt")), (py_str("no way"), py_str("doubt")), (py_str("for real"), py_str("doubt")), (py_str("get out"), py_str("doubt")), (py_str("you what"), py_str("doubt")), (py_str("come on"), py_str("doubt")), (py_str("say what"), py_str("doubt")), (py_str("my word"), py_str("awe")), (py_str("good god"), py_str("awe")), (py_str("my god"), py_str("awe")), (py_str("oh my god"), py_str("awe")), (py_str("oh my word"), py_str("awe")), (py_str("good heavens"), py_str("awe"))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.PHRASES"); }
    s_PHRASES(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("oh"), py_str("ah"), py_str("aw"), py_str("ooh"), py_str("uh"), py_str("um"), py_str("well"), py_str("so"), py_str("and"), py_str("but"), py_str("my"), py_str("god"), py_str("jesus"), py_str("christ"), py_str("jeez"), py_str("geez"), py_str("good"), py_str("bloody"), py_str("damn"), py_str("just"), py_str("huh"), py_str("hmm"), py_str("ok"), py_str("okay"), py_str("no"), py_str("for"), py_str("get"), py_str("out"), py_str("the"), py_str("hell"), py_str("heck"), py_str("come"), py_str("on"), py_str("you"), py_str("what"), py_str("say"), py_str("yeah"), py_str("yes")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.LEAD"); }
    s_LEAD(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.AWE_REPLIES"); }
    s_AWE_REPLIES(py_tuple(vec![py_str("It sounds like more than it is — the whole thing is usually one step. Which bit do you want to start with?"), py_str("It is easier once you have done it once. Do you want that as a list you can follow?"), py_str("Most of it is one small thing done properly. Shall I say which part matters most?"), py_str("That is the whole trick of it, honestly. Anything in there you want me to go over?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.DOUBT_REPLIES"); }
    s_DOUBT_REPLIES(py_tuple(vec![py_str("Yes, really — and it is the small part that does the work. Which bit do you want me to go over?"), py_str("It is true, honestly. Is there a part of it that sounds off to you?"), py_str("I know it sounds odd — that is the whole answer though. Which bit would you not bet on?"), py_str("It is right, yes. Which part surprised you?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.NO_ANSWER"); }
    s_NO_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|111|ambulance|a&e|casualty|emergency|urgent care|a search|search online|rather not guess|sorry|sad news|condolence\\w*|passed away|passed on|funeral|grief|grieving|bereave\\w*|miss (?:him|her|them)|died|death|dying|hurt yourself|harm yourself|end your life|crisis|samaritans|suicide|116 123)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.A_CARE_ANSWER"); }
    s_A_CARE_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:gp|doctor|dentist|dental|nurse|pharmac\\w*|111|999|a&e|nhs|clinic|ring (?:them|us|the)|get it (?:looked at|checked|seen)|needs? (?:looking at|checking|seeing)|ache|aching|aches|pain|sore|swelling|swollen|lump|rash|inflamed|fever|bleeding|burn|sprain)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.A_BODY_LINE"); }
    s_A_BODY_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|their|our|your|the left|the right)\\s+(?:tooth|teeth|gums?|jaw|back|knee|knees|hip|shoulder|neck|throat|chest|stomach|belly|tummy|arm|arms|leg|legs|foot|feet|ankle|wrist|hand|hands|finger|fingers|thumb|toe|toes|head|headache|migraine|ear|ears|eye|eyes|skin|nose|sinus|lump|rash|itch|itchy|sore|ache|aching|aches|pain|painful|hurt|hurts|hurting|swollen|swelling|bleed|bleeding|fever|cough|burn|burning|blister|sprain|sting|infection|gland|glands)\\b(?!\\s+(?:garden|yard|door|room|wall|seat|gate|fence|window|path|drive|field|lane|page|cover|button|pocket|light|start|of\\b))|\\b(?:toothache|headaches?|migraine|tooth|gums|sore throat|backache|stomach ache)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.CARE_REPLIES"); }
    s_CARE_REPLIES(py_tuple(vec![py_str("It is worth getting that looked at rather than waiting on it. Anything in there you want me to go over?"), py_str("Nothing there is meant to scare you - it is just worth getting it seen. Any part you want me to say again?"), py_str("Yes - that one is a ring-them-today sort of thing. Do you want me to say what to ask when you call?"), py_str("That is the size of it, honestly. Shall I say what to mention when you get it looked at?")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction._ASK"); }
    s__ASK(py_str("(?:ok|okay|alright|fine|sensible|sound|wise|right|fair|make\\s+(?:any\\s+)?sense|worth\\s+it|a\\s+good\\s+idea|a\\s+good\\s+choice|the\\s+right\\s+(?:thing|way))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction._TAIL"); }
    s__TAIL(py_str("(?:\\s+(?:then|though|honestly|yeah|for\\s+me))?\\s*[.?!]*\\s*$"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.CHECK"); }
    s_CHECK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:do|would|could)\\s+(?:you|u)\\s+(?:think|reckon|say|agree)\\b[^.?!]{0,24}?\\b"), &g__ASK()), &g__TAIL()), &py_str("|^\\s*(?:and\\s+|but\\s+|so\\s+)?(?:is|are|was|were|does|did|would)\\s+(?:that|this)\\s+(?:be\\s+|sound\\s+|look\\s+|seem\\s+)?")), &g__ASK()), &g__TAIL()), &py_str("|\\bare\\s+you\\s+sure")), &g__TAIL()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.OWN_PLAN"); }
    s_OWN_PLAN(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+(?:going|gonna|planning|thinking (?:of|about)|intending|about)\\b|\\b(?:i|we)\\s+(?:want|need|have decided|decided)\\s+to\\b|\\b(?:i|we)\\s+(?:might|may)\\s+\\w+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.NO_STANCE"); }
    s_NO_STANCE(stdlib_call("re", "compile", &[py_str("\\b(?:i (?:don't|do not|cannot|can't) (?:know|say)|not know enough|rather not guess|a search|search online|look it up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.CHECK_REPLIES"); }
    s_CHECK_REPLIES(py_tuple(vec![py_str("Yes, as far as what you have told me goes - and if a detail is different, the answer changes with it. Which part are you least sure of?"), py_str("I would stand by it as advice. What I cannot check is the part only you know - which bit is worrying you?"), py_str("It holds up as far as I can tell. Tell me the bit that feels off and I will build the answer around it."), py_str("Yes. It is the answer I would give again, and if one thing in it does not fit your side of it, say which.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.reaction.THEIR_REACTION"); }
    s_THEIR_REACTION(stdlib_call("re", "compile", &[py_str("\\bam i\\b|\\boverreacting\\b|\\bbeing (?:silly|daft|mad)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(py_eq(&py_str("bot.compose.reaction"), &py_str("__main__")))) {
    let _ = main(&[]);
    }
}
pub fn _checking(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks the bot to judge the answer it has just given.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&g_THEIR_REACTION(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_CHECK(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bare(args: &[Py]) -> Py {
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
    let mut kind: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut size: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Which kind of reaction a line that is nothing but a reaction is, or \"\".");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    clean = mcall(&clean.clone(), "strip", &[py_str(" .,!?;:\"'")]);
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(3), Py::Int(2), Py::Int(1)]));
    for __item in __iter1 {
    size = __item.clone();
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[Py::Int(0), op_add(&op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    kind = mcall(&g_PHRASES(), "get", &[mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &start.clone(), &op_add(&start.clone(), &size.clone()), &Py::None)])]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    continue;
    }
    rest = op_add(&slice_or_raise(&words.clone(), &Py::None, &start.clone(), &Py::None), &slice_or_raise(&words.clone(), &op_add(&start.clone(), &size.clone()), &Py::None, &Py::None));
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LEAD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return kind.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _an_answer(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the bot's last line was an answer rather than a nod, a question or a hello.");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&op_lt(&call_builtin("len", &[line.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(60))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&line.clone(), "endswith", &[py_str("?")])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NO_ANSWER(), "search", &[mcall(&line.clone(), "lower", &[])])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _fallback: Py = Py::None;
    let mut above: Py = Py::None;
    let mut above_text: Py = Py::None;
    let mut care: Py = Py::None;
    let mut i: Py = Py::None;
    let mut index: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut last: Py = Py::None;
    let mut letter: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plan_line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The reply to a line that is only a reaction to the answer above, or None.");
    kind = _bare(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    kind = if py_truthy(&_checking(&[text.clone()])) { py_str("check") } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_truthy(&_an_answer(&[mcall(&last.clone(), "get", &[py_str("bot")])])))) {
    return Py::None;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("check"))); if !py_truthy(&__b) { __b } else { mcall(&g_OWN_PLAN(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("check")))) {
    _fallback = pyrt::xrefs::xmodule("compose.fallback");
    let __unpk1 = py_unpack(&mcall(&_fallback.clone(), "_the_plan_and_its_line", &[memory.clone()]), 2, None);
    plan = __unpk1[0].clone();
    plan_line = __unpk1[1].clone();
    if py_truthy(&{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { plan_line.clone() } }) {
    return Py::None;
    }
    }
    above_text = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("check"))); if !py_truthy(&__b) { __b } else { mcall(&g_NO_STANCE(), "search", &[above_text.clone()]) } }) {
    return Py::None;
    }
    care = { let __b = mcall(&g_A_BODY_LINE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_CARE_ANSWER(), "search", &[above_text.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("check"))); if !py_truthy(&__b) { __b } else { care.clone() } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("check")))) {
    pool = g_CHECK_REPLIES();
    } else {
    if py_truthy(&care.clone()) {
    pool = g_CARE_REPLIES();
    } else {
    pool = if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("awe")))) { g_AWE_REPLIES() } else { g_DOUBT_REPLIES() };
    }
    }
    seen = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    above = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    seed = op_add(&op_add(&op_add(&op_add(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &py_str("|")), &call_builtin("str", &[seen.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("|")), &above.clone());
    index = op_modulo(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk708 = py_unpack(&__item.clone(), 2, None); let mut i: Py = __unpk708[0].clone(); let mut letter: Py = __unpk708[1].clone(); __out.push(op_mul(&op_add(&i.clone(), &Py::Int(1)), &call_builtin("ord", &[letter.clone()]).unwrap_or_else(|e| exc::raise(e)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[pool.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return geti(&pool.clone(), &index.clone());
    Py::None
}
pub fn main(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut argv: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "argv") { v.clone() }
    else { Py::None };
    let mut after: Py = Py::None;
    let mut args: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sys: Py = Py::None;
    sys = stdlib_module("sys");
    args = call_builtin("list", &[if py_truthy(&Py::Bool(py_is(&argv.clone(), &Py::None))) { slice_or_raise(&gattr(&sys.clone(), "argv"), &Py::Int(1), &Py::None, &Py::None) } else { argv.clone() }]).unwrap_or_else(|e| exc::raise(e));
    said = if py_truthy(&args.clone()) { geti(&args.clone(), &Py::Int(0)) } else { py_str("wow") };
    after = py_str("");
    if py_truthy(&Py::Bool(py_contains(&args.clone(), &py_str("--after")))) {
    rest = slice_or_raise(&args.clone(), &op_add(&mcall(&args.clone(), "index", &[py_str("--after")]), &Py::Int(1)), &Py::None, &Py::None);
    after = if py_truthy(&rest.clone()) { geti(&rest.clone(), &Py::Int(0)) } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    let _ = call_builtin("print", &[{ let __b = _bare(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("(nothing)") } }]).unwrap_or_else(|e| exc::raise(e));
    return Py::None;
    }
    let _ = call_builtin("print", &[{ let __b = reply_for(&[said.clone(), py_dict(vec![(py_str("turns"), py_list(vec![py_dict(vec![(py_str("user"), py_str("")), (py_str("bot"), after.clone())])]))])]); if py_truthy(&__b) { __b } else { py_str("(nothing)") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.reaction".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AWE_REPLIES" => Some(g_AWE_REPLIES()),
            "A_BODY_LINE" => Some(g_A_BODY_LINE()),
            "A_CARE_ANSWER" => Some(g_A_CARE_ANSWER()),
            "CARE_REPLIES" => Some(g_CARE_REPLIES()),
            "CHECK" => Some(g_CHECK()),
            "CHECK_REPLIES" => Some(g_CHECK_REPLIES()),
            "DOUBT_REPLIES" => Some(g_DOUBT_REPLIES()),
            "LEAD" => Some(g_LEAD()),
            "NO_ANSWER" => Some(g_NO_ANSWER()),
            "NO_STANCE" => Some(g_NO_STANCE()),
            "OWN_PLAN" => Some(g_OWN_PLAN()),
            "PHRASES" => Some(g_PHRASES()),
            "THEIR_REACTION" => Some(g_THEIR_REACTION()),
            "_ASK" => Some(g__ASK()),
            "_TAIL" => Some(g__TAIL()),
            "_an_answer" => Some(py_func("_an_answer", |args: &[Py]| _an_answer(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_checking" => Some(py_func("_checking", |args: &[Py]| _checking(args))),
            "main" => Some(py_func("main", |args: &[Py]| main(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AWE_REPLIES" => s_AWE_REPLIES(v),
            "A_BODY_LINE" => s_A_BODY_LINE(v),
            "A_CARE_ANSWER" => s_A_CARE_ANSWER(v),
            "CARE_REPLIES" => s_CARE_REPLIES(v),
            "CHECK" => s_CHECK(v),
            "CHECK_REPLIES" => s_CHECK_REPLIES(v),
            "DOUBT_REPLIES" => s_DOUBT_REPLIES(v),
            "LEAD" => s_LEAD(v),
            "NO_ANSWER" => s_NO_ANSWER(v),
            "NO_STANCE" => s_NO_STANCE(v),
            "OWN_PLAN" => s_OWN_PLAN(v),
            "PHRASES" => s_PHRASES(v),
            "THEIR_REACTION" => s_THEIR_REACTION(v),
            "_ASK" => s__ASK(v),
            "_TAIL" => s__TAIL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.reaction", module_obj);
    pyrt::xrefs::reg_fn("compose.reaction", "_checking", py_func("_checking", |args: &[Py]| _checking(args)));
    pyrt::xrefs::reg_fn("compose.reaction", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("compose.reaction", "_an_answer", py_func("_an_answer", |args: &[Py]| _an_answer(args)));
    pyrt::xrefs::reg_fn("compose.reaction", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.reaction", "main", py_func("main", |args: &[Py]| main(args)));
    pyrt::xrefs::reg_access("compose.reaction", "AWE_REPLIES", g_AWE_REPLIES, s_AWE_REPLIES);
    pyrt::xrefs::reg_access("compose.reaction", "A_BODY_LINE", g_A_BODY_LINE, s_A_BODY_LINE);
    pyrt::xrefs::reg_access("compose.reaction", "A_CARE_ANSWER", g_A_CARE_ANSWER, s_A_CARE_ANSWER);
    pyrt::xrefs::reg_access("compose.reaction", "CARE_REPLIES", g_CARE_REPLIES, s_CARE_REPLIES);
    pyrt::xrefs::reg_access("compose.reaction", "CHECK", g_CHECK, s_CHECK);
    pyrt::xrefs::reg_access("compose.reaction", "CHECK_REPLIES", g_CHECK_REPLIES, s_CHECK_REPLIES);
    pyrt::xrefs::reg_access("compose.reaction", "DOUBT_REPLIES", g_DOUBT_REPLIES, s_DOUBT_REPLIES);
    pyrt::xrefs::reg_access("compose.reaction", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("compose.reaction", "NO_ANSWER", g_NO_ANSWER, s_NO_ANSWER);
    pyrt::xrefs::reg_access("compose.reaction", "NO_STANCE", g_NO_STANCE, s_NO_STANCE);
    pyrt::xrefs::reg_access("compose.reaction", "OWN_PLAN", g_OWN_PLAN, s_OWN_PLAN);
    pyrt::xrefs::reg_access("compose.reaction", "PHRASES", g_PHRASES, s_PHRASES);
    pyrt::xrefs::reg_access("compose.reaction", "THEIR_REACTION", g_THEIR_REACTION, s_THEIR_REACTION);
    pyrt::xrefs::reg_access("compose.reaction", "_ASK", g__ASK, s__ASK);
    pyrt::xrefs::reg_access("compose.reaction", "_TAIL", g__TAIL, s__TAIL);
}

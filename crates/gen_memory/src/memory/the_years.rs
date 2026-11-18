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


thread_local! { static G_BIRTHDAY_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPARISON_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_AGE_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OLD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_OLD_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTHDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BODY_CLAIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISLIKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FORGETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TREATMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_BIRTHDAY_LINES() -> Py { ensure_init(); G_BIRTHDAY_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIRTHDAY_LINES(v: Py) { ensure_init(); G_BIRTHDAY_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPARISON_LINES() -> Py { ensure_init(); G_COMPARISON_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPARISON_LINES(v: Py) { ensure_init(); G_COMPARISON_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_AGE_LINES() -> Py { ensure_init(); G_MY_AGE_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_AGE_LINES(v: Py) { ensure_init(); G_MY_AGE_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_OLD_LINES() -> Py { ensure_init(); G_OLD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OLD_LINES(v: Py) { ensure_init(); G_OLD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_LINES() -> Py { ensure_init(); G_PAST_IT_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_LINES(v: Py) { ensure_init(); G_PAST_IT_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_OLD_LINES() -> Py { ensure_init(); G_TOO_OLD_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_OLD_LINES(v: Py) { ensure_init(); G_TOO_OLD_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE() -> Py { ensure_init(); G__AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE(v: Py) { ensure_init(); G__AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_START() -> Py { ensure_init(); G__ASK_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_START(v: Py) { ensure_init(); G__ASK_START.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTHDAY() -> Py { ensure_init(); G__BIRTHDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTHDAY(v: Py) { ensure_init(); G__BIRTHDAY.with(|c| *c.borrow_mut() = v); }
pub fn g__BODY_CLAIM() -> Py { ensure_init(); G__BODY_CLAIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BODY_CLAIM(v: Py) { ensure_init(); G__BODY_CLAIM.with(|c| *c.borrow_mut() = v); }
pub fn g__DISLIKE() -> Py { ensure_init(); G__DISLIKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISLIKE(v: Py) { ensure_init(); G__DISLIKE.with(|c| *c.borrow_mut() = v); }
pub fn g__FEELING() -> Py { ensure_init(); G__FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEELING(v: Py) { ensure_init(); G__FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g__FORGETTING() -> Py { ensure_init(); G__FORGETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FORGETTING(v: Py) { ensure_init(); G__FORGETTING.with(|c| *c.borrow_mut() = v); }
pub fn g__LIMIT() -> Py { ensure_init(); G__LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIMIT(v: Py) { ensure_init(); G__LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER() -> Py { ensure_init(); G__OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER(v: Py) { ensure_init(); G__OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_SUBJECT() -> Py { ensure_init(); G__OWN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_SUBJECT(v: Py) { ensure_init(); G__OWN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__SELF() -> Py { ensure_init(); G__SELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SELF(v: Py) { ensure_init(); G__SELF.with(|c| *c.borrow_mut() = v); }
pub fn g__TREATMENT() -> Py { ensure_init(); G__TREATMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TREATMENT(v: Py) { ensure_init(); G__TREATMENT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line about feeling old, answered as the thing it is.\n\nGetting older comes up in ordinary chat and there was nothing to answer it with. On main every\none of these was a shrug, an echo or a line for a disclosure:\n\n```\ni feel old                                -> I'm listening - say more if it helps.\ni feel ancient                            -> Thanks for telling me - I'm here if you want to dig into any of that.\nim past it                                -> Got it. Is there anything you'd like a hand with?\ni feel my age today                       -> Got it. Is there anything you'd like a hand with?\nim too old for this                       -> Got it. Is there anything you'd like a hand with?\nim turning 40 next month and i feel old   -> That's lovely news - and something to look forward to.\n```\n\nand one worse: **\"im getting old\" was answered \"Nice. How long do you reckon it'll take?\"**, the\nreader for a length of time (\"how long does a chicken take\") taking the \"getting\" and the whole\nline with it.\n\nA friend does three things with a line like this: agrees about the body in a word, refuses the\nverdict (feeling old is usually a bad night, a birthday or a thing they cannot do any more rather\nthan an age), and asks what brought it on - because there always is something behind it.\n\nWhat keeps it narrow:\n\n* it is the person's own line. Somebody else's age (\"how old is my mum\") and a body that hurts\n  in the cold (\"my knees are 80 years old\") belong to the readings that own them.\n* a birthday on its own (\"i turn 30 next week\", \"im 67\") is news or a fact, not a feeling, and\n  the pools here are only for the line that says how it feels.\n* the age, when one is said, is answered as an age: \"40 next month and i feel old\" is a birthday\n  coming, so the reply is about that rather than about being old in general.\n");
    s__FORGETTING(stdlib_call("re", "compile", &[py_str("\\b(?:forget|forgets|forgetting|forgetful|forgetfulness|forgot|forgotten)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TREATMENT(stdlib_call("re", "compile", &[py_str("\\b(?:chemo|chemotherapy|radiotherapy|cancer|tumour|tumor|surgery|operation|treatment|diagnosis|diagnosed|recovering|recovery|hospital|illness|condition)\\b|\\bafter my\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OWN_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:hair|grey|gray|greying|graying|teeth|dentures|hearing|glasses|wrinkles?|skin)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__DISLIKE(stdlib_call("re", "compile", &[py_str("\\b(?:hate|hates|hating|can't stand|cant stand|fed up|sick of)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LIMIT(Py::Int(16));
    s__FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:feel(?:ing|s|ing)?\\s+(?:so\\s+|really\\s+|quite\\s+|very\\s+|a bit\\s+|pretty\\s+)?(?:old|ancient|past it|my age|like an old (?:man|woman|person)|like a pensioner|about (?:90|100|eighty|ninety))\\b|feeling (?:old|ancient|my age)\\b|past it\\b|getting old\\b|growing old\\b|getting on a bit\\b|too old for (?:this|that|it|all this)\\b|not (?:young|getting) (?:any more|anymore)\\b|not (?:as|so) (?:young|fit|quick|strong) as (?:i|we)\\b|(?:i'?m|i am|im|i feel|i'?m feeling|i feel like i'?m)\\s+(?:a bit\\s+|a little\\s+)?over the hill\\b|feel(?:ing)?\\s+(?:like\\s+)?(?:a bit\\s+)?(?:past|passed) it\\b|\\b(?:im|i'?m|i am)\\s+getting on\\b(?=\\s*(?:$|[.,!?]|now\\b|these days\\b|in years\\b|a bit\\b))|(?:the years?|age) (?:are |is )?(?:catching up|creeping up)\\b|feel(?:ing)?\\s+(?:\\d\\d|like (?:i'?m |i am )?\\d\\d|ninety|eighty|seventy)\\b|(?:my )?(?:body|knees|back) (?:is|are) (?:letting me down|falling apart)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SELF(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|iam|i am|me|my|mine)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OTHER(stdlib_call("re", "compile", &[py_str("\\b(?:my (?:mum|dad|mother|father|gran|nan|grandad|grandma|grandpa|husband|wife|partner|son|daughter|sister|brother|friend|boss|neighbour)|he|she|they|them|his|her|their)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BODY_CLAIM(stdlib_call("re", "compile", &[py_str("\\b(?:knees?|hips?|back|shoulders?|joints?|wrists?|ankles?|feet|hands?|neck|spine)\\b.{0,24}\\b(?:hurt|hurts|aching|aches|stiff|sore|clicking|giving way)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASK_START(stdlib_call("re", "compile", &[py_str("^\\s*(?:is|are|was|were|will|would|shall|should|can|could|do|does|did|when|what|how|why|where|who|any)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AGE(stdlib_call("re", "compile", &[py_str("\\b(?:turning|turn|turned|hitting|hitting|becoming|40|50|60|70|80|ninety|forty|fifty|sixty|seventy|eighty)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BIRTHDAY(stdlib_call("re", "compile", &[py_str("\\b(?:turning|turn|hitting|(?:am|im|i'm)\\s+(?:going to be)?)\\s+(?:\\d\\d|forty|fifty|sixty|seventy|eighty|ninety)\\b|\\b\\d\\d?\\s+(?:next|this|in|last)\\s+(?:week|month|year|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OLD_LINES(py_tuple(vec![py_str("It is mostly the body sending invoices. What is it that made you feel it today?"), py_str("That is usually one bad night or one bad morning rather than an age. What set it off?"), py_str("Some days the body reports in and everything creaks. What is yours doing at the moment?"), py_str("Old is a number that only feels heavy on some days. What brought it on today?"), py_str("I would not take one bad day as a verdict. What has been hard work lately?"), py_str("Everything hurts and nothing is where you left it. What is the worst of it just now?")]));
    s_PAST_IT_LINES(py_tuple(vec![py_str("Past it at what, though? Say the thing you have decided you cannot do and we will look at it."), py_str("I would want to know which thing has gone past you. What is it?"), py_str("That is a hard thing to decide about yourself. What is it that has started to feel beyond you?"), py_str("Give me the thing it is about - a job, a walk, a hobby - and we can see if it really has.")]));
    s_TOO_OLD_LINES(py_tuple(vec![py_str("Too old for it, or just tired of it? What is the thing that has got too much?"), py_str("There is a difference between too old and had enough. Which is it?"), py_str("Then do it your own way or not at all. What is it that has gone past the point of being fun?")]));
    s_MY_AGE_LINES(py_tuple(vec![py_str("Some days the body reports in and the rest of us just listen. What is doing the reporting?"), py_str("A day that feels its age usually had something in it. What has yours had?"), py_str("Then today is a slower one. What is on it?")]));
    s_BIRTHDAY_LINES(py_tuple(vec![py_str("{turn} - a birthday makes everybody take stock, and that is all this usually is. What are you dreading about it?"), py_str("The birthday is the loud part; the feeling is usually something else. What is behind it?"), py_str("Nothing changes overnight at {turn}. What have you started noticing?"), py_str("Everybody does this around a birthday. What would you want the next year to look like?")]));
    s_COMPARISON_LINES(py_tuple(vec![py_str("That is other people getting younger rather than you getting old. Who were you looking at?"), py_str("The world does look younger from here. What was it that made you notice?"), py_str("Everybody under thirty starts to look like a child at some point. What brought it on?")]));
}
pub fn _the_turn(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("(\"40 next month\", ...) when the line says a birthday is coming, else None.");
    found = mcall(&g__BIRTHDAY(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[])]);
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("A warm, specific answer to a line about feeling old, or None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__LIMIT()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { mcall(&g__ASK_START(), "match", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__FEELING(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    said = { let __b = call_builtin("bool", &[mcall(&g__SELF(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "match", &[py_str("^\\W*(?:the years|it is|its|age)\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__OTHER(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__BODY_CLAIM(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__FORGETTING(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__TREATMENT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__OWN_SUBJECT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__DISLIKE(), "search", &[clean.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&crate::memory::passing_the_time::_the_chat_is_heavy(&[memory.clone()])) {
    return Py::None;
    }
    seed = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    turn = _the_turn(&[clean.clone()]);
    if py_truthy(&turn.clone()) {
    turn = op_add(&mcall(&geti(&turn.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&turn.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&turn.clone()) {
    pool = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BIRTHDAY_LINES()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "replace", &[py_str("{turn}"), turn.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:past|passed) it\\b|\\bover the hill\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PAST_IT_LINES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\btoo old\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_TOO_OLD_LINES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("look at|young people|teenagers|schoolchildren|all these kids"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_COMPARISON_LINES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmy age\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MY_AGE_LINES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OLD_LINES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.the_years".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BIRTHDAY_LINES" => Some(g_BIRTHDAY_LINES()),
            "COMPARISON_LINES" => Some(g_COMPARISON_LINES()),
            "MY_AGE_LINES" => Some(g_MY_AGE_LINES()),
            "OLD_LINES" => Some(g_OLD_LINES()),
            "PAST_IT_LINES" => Some(g_PAST_IT_LINES()),
            "TOO_OLD_LINES" => Some(g_TOO_OLD_LINES()),
            "_AGE" => Some(g__AGE()),
            "_ASK_START" => Some(g__ASK_START()),
            "_BIRTHDAY" => Some(g__BIRTHDAY()),
            "_BODY_CLAIM" => Some(g__BODY_CLAIM()),
            "_DISLIKE" => Some(g__DISLIKE()),
            "_FEELING" => Some(g__FEELING()),
            "_FORGETTING" => Some(g__FORGETTING()),
            "_LIMIT" => Some(g__LIMIT()),
            "_OTHER" => Some(g__OTHER()),
            "_OWN_SUBJECT" => Some(g__OWN_SUBJECT()),
            "_SELF" => Some(g__SELF()),
            "_TREATMENT" => Some(g__TREATMENT()),
            "_the_turn" => Some(py_func("_the_turn", |args: &[Py]| _the_turn(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BIRTHDAY_LINES" => s_BIRTHDAY_LINES(v),
            "COMPARISON_LINES" => s_COMPARISON_LINES(v),
            "MY_AGE_LINES" => s_MY_AGE_LINES(v),
            "OLD_LINES" => s_OLD_LINES(v),
            "PAST_IT_LINES" => s_PAST_IT_LINES(v),
            "TOO_OLD_LINES" => s_TOO_OLD_LINES(v),
            "_AGE" => s__AGE(v),
            "_ASK_START" => s__ASK_START(v),
            "_BIRTHDAY" => s__BIRTHDAY(v),
            "_BODY_CLAIM" => s__BODY_CLAIM(v),
            "_DISLIKE" => s__DISLIKE(v),
            "_FEELING" => s__FEELING(v),
            "_FORGETTING" => s__FORGETTING(v),
            "_LIMIT" => s__LIMIT(v),
            "_OTHER" => s__OTHER(v),
            "_OWN_SUBJECT" => s__OWN_SUBJECT(v),
            "_SELF" => s__SELF(v),
            "_TREATMENT" => s__TREATMENT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.the_years", module_obj);
    pyrt::xrefs::reg_fn("memory.the_years", "_the_turn", py_func("_the_turn", |args: &[Py]| _the_turn(args)));
    pyrt::xrefs::reg_fn("memory.the_years", "reply", py_func("reply", |args: &[Py]| reply(args)));
    pyrt::xrefs::reg_access("memory.the_years", "BIRTHDAY_LINES", g_BIRTHDAY_LINES, s_BIRTHDAY_LINES);
    pyrt::xrefs::reg_access("memory.the_years", "COMPARISON_LINES", g_COMPARISON_LINES, s_COMPARISON_LINES);
    pyrt::xrefs::reg_access("memory.the_years", "MY_AGE_LINES", g_MY_AGE_LINES, s_MY_AGE_LINES);
    pyrt::xrefs::reg_access("memory.the_years", "OLD_LINES", g_OLD_LINES, s_OLD_LINES);
    pyrt::xrefs::reg_access("memory.the_years", "PAST_IT_LINES", g_PAST_IT_LINES, s_PAST_IT_LINES);
    pyrt::xrefs::reg_access("memory.the_years", "TOO_OLD_LINES", g_TOO_OLD_LINES, s_TOO_OLD_LINES);
    pyrt::xrefs::reg_access("memory.the_years", "_AGE", g__AGE, s__AGE);
    pyrt::xrefs::reg_access("memory.the_years", "_ASK_START", g__ASK_START, s__ASK_START);
    pyrt::xrefs::reg_access("memory.the_years", "_BIRTHDAY", g__BIRTHDAY, s__BIRTHDAY);
    pyrt::xrefs::reg_access("memory.the_years", "_BODY_CLAIM", g__BODY_CLAIM, s__BODY_CLAIM);
    pyrt::xrefs::reg_access("memory.the_years", "_DISLIKE", g__DISLIKE, s__DISLIKE);
    pyrt::xrefs::reg_access("memory.the_years", "_FEELING", g__FEELING, s__FEELING);
    pyrt::xrefs::reg_access("memory.the_years", "_FORGETTING", g__FORGETTING, s__FORGETTING);
    pyrt::xrefs::reg_access("memory.the_years", "_LIMIT", g__LIMIT, s__LIMIT);
    pyrt::xrefs::reg_access("memory.the_years", "_OTHER", g__OTHER, s__OTHER);
    pyrt::xrefs::reg_access("memory.the_years", "_OWN_SUBJECT", g__OWN_SUBJECT, s__OWN_SUBJECT);
    pyrt::xrefs::reg_access("memory.the_years", "_SELF", g__SELF, s__SELF);
    pyrt::xrefs::reg_access("memory.the_years", "_TREATMENT", g__TREATMENT, s__TREATMENT);
}

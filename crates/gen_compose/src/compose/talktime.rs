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


thread_local! { static G_A_COMPANY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COURTESY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TALK_AT_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TALK_AT_A_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TALK_AT_A_TIME_HARM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MARKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_COMPANY_ASK() -> Py { ensure_init(); G_A_COMPANY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMPANY_ASK(v: Py) { ensure_init(); G_A_COMPANY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COURTESY() -> Py { ensure_init(); G_A_COURTESY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COURTESY(v: Py) { ensure_init(); G_A_COURTESY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TALK_AT_A_TIME() -> Py { ensure_init(); G_A_TALK_AT_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TALK_AT_A_TIME(v: Py) { ensure_init(); G_A_TALK_AT_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TALK_AT_A_TIME_ASK() -> Py { ensure_init(); G_A_TALK_AT_A_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TALK_AT_A_TIME_ASK(v: Py) { ensure_init(); G_A_TALK_AT_A_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TALK_AT_A_TIME_HARM() -> Py { ensure_init(); G_A_TALK_AT_A_TIME_HARM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TALK_AT_A_TIME_HARM(v: Py) { ensure_init(); G_A_TALK_AT_A_TIME_HARM.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_TIME() -> Py { ensure_init(); G_THE_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_TIME(v: Py) { ensure_init(); G_THE_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MARKER() -> Py { ensure_init(); G__A_MARKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MARKER(v: Py) { ensure_init(); G__A_MARKER.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY() -> Py { ensure_init(); G__DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY(v: Py) { ensure_init(); G__DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__PART() -> Py { ensure_init(); G__PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PART(v: Py) { ensure_init(); G__PART.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A talk asked for at a time: \"can we talk tonight\", \"can we chat later\", \"can you keep me company\".\n\nNothing here keeps a diary and there is no clock on this end, and that is the honest half of the\nanswer. What went wrong on main is the other half: these lines reached the search hedge (\"I don't\nknow enough to answer that honestly. A search should sort you out quickly\"), which hands a person\nwho wants a conversation a page about nothing. A friend offered a time does not say \"look it up\" -\nthey say when they are reachable, and ask what it is about.\n\nThe reading is narrow on purpose:\n\n  - the ask is a talk word (talk, chat, speak, catch up, a word, a chat, a talk) with a *time* on\n    the end and nothing else. The time is what makes it a plan, so `THE_TIME` is a closed list of\n    days, parts of a day and points in one, and the anchor needs it to be all that is left. Without\n    it \"can we talk\" keeps its chat line (\"Yep, right here - go on.\") and \"can we talk properly\"\n    keeps the same one;\n  - a thing named after the talk is the thing's own reading, not this one: \"can we talk about my\n    boiler\" and \"can we talk tomorrow about the rent\" are the boiler and the rent;\n  - \"can you keep me company\" is the same ask with no time on it, and nothing else reads it, so it\n    does not have to carry one;\n  - harm is never a plan: \"can we talk tomorrow, i keep thinking about ending it\" belongs to the\n    safety cards and the care pools, which are asked first anyway - the fence here means this\n    reading can never be the one that answers it.\n\nIt is asked early in `_reply_for` (`bot/compose/fallback_chain.py`), before the hedge and before the\npools that answer a thing, and it stands down for any line that names what the talk is about.\n");
    s__DAY(py_str("(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|today|tomorrow|tonight|christmas|the weekend)"));
    s__PART(py_str("(?:morning|afternoon|evening|night|daytime|lunchtime|teatime|after work|after tea|after dinner|after lunch|after school|after the kids are in bed)"));
    s__A_MARKER(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?:(?:this |next |on |by )(?:"), &g__DAY()), &py_str(")|(?:")), &g__DAY()), &py_str(")(?:\\s+(?:")), &g__PART()), &py_str("))?|(?:this |in the |in |during the |tomorrow )(?:")), &g__PART()), &py_str(")|(?:")), &g__PART()), &py_str(")|later(?:\\s+on)?|in a (?:bit|minute|while|mo)|in the week|during the week|sometime|some time|any time|at some point|at some stage|whenever|next week|this week|next month|this month|at\\s+\\d{1,2}(?:[.:]\\d{2})?\\s*(?:am|pm|a\\.m\\.|p\\.m\\.|o'?clock)?|at\\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|noon|midday|midnight|teatime|lunchtime|half\\s+\\w+|quarter\\s+\\w+)|in\\s+(?:a|one|two|three|half\\s+an?|an?|\\d{1,3})\\s+(?:hour|hours|hr|hrs|minute|minutes|min|mins)|when you(?:'re| are) free|when you have time|when you get a chance|when you can)")));
    s_THE_TIME(op_add(&op_add(&op_add(&op_add(&py_str("(?:"), &g__A_MARKER()), &py_str("(?:\\s+")), &g__A_MARKER()), &py_str("){0,2})")));
    s_A_COURTESY(py_str("(?:\\s+(?:please|thanks|thank you|ta))?"));
    s_A_COMPANY_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^(?:hi|hiya|hey|hello|so|ok|okay|and|but|just|right)?[,!. ]*(?:(?:can|could|will|would|do)\\s+(?:you|u)\\s+(?:please\\s+)?)?(?:(?:keep|stay)\\s+me\\s+company|stay\\s+with\\s+me)(?:"), &g_A_COURTESY()), &py_str("|\\s+(?:")), &g_THE_TIME()), &py_str(")")), &g_A_COURTESY()), &py_str(")?\\s*[?.!]*$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TALK_AT_A_TIME_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("^(?:hi|hiya|hey|hello|so|ok|okay|and|but|just|right|um|er)?[,!. ]*(?:(?:can|could|shall|should|would|will)\\s+(?:we|you|u|i)\\s+(?:please\\s+)?(?:talk|chat|speak|catch up|have a word|have a chat|have a talk)(?:\\s+(?:to|with)\\s+(?:you|u|me))?|i\\s+(?:want|need|would like|wanna|have)\\s+to\\s+(?:talk|chat|speak)\\s+to\\s+(?:you|u))\\s+"), &g_THE_TIME()), &g_A_COURTESY()), &py_str("\\s*[?.!]*$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TALK_AT_A_TIME_HARM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:kill myself|end it|end my life|suicid|self harm|hurt myself|overdose|\\bdie\\b|\\bdying\\b|dead|funeral|died|cancer|hospital|ambulance|999|911)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TALK_AT_A_TIME(py_tuple(vec![py_str("I can't hold a time or remind you - no diary and no clock on this end. The chat doesn't mind when you come back, so start now or whenever suits: what's it about?"), py_str("Nothing here books a slot or rings you, so a time is the one thing I can't keep. What I can do is talk it through now and pick it up again when you come back. What did you want to say?"), py_str("There's no diary on this end, so I can't promise to be there at a set time - but nothing here closes either, so the thread keeps its place. What's on your mind?"), py_str("I can't put anything in a diary or set a reminder, and I would rather say that than nod along. What I can do is start now and keep the thread for whenever you come back. What's up?"), py_str("No clock here, so any hour is all the same to me, and the conversation stays where we leave it. What did you want to talk about?"), py_str("I can't save a slot or bring it up again, sorry - but the chat is here at any hour of the day and keeps its place. What's going on?")]));
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
    else { Py::None };
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The honest line for a talk asked for at a time, or None when the line is something else.");
    said = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { memory.clone() } else { py_dict(vec![]) };
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TALK_AT_A_TIME_ASK(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COMPANY_ASK(), "match", args))(&__args) } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TALK_AT_A_TIME_HARM(), "search", args))(&__args) }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_TALK_AT_A_TIME()); __args.push(said.clone()); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&crate::compose::fallback::g__choose(), args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.talktime".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_COMPANY_ASK" => Some(g_A_COMPANY_ASK()),
            "A_COURTESY" => Some(g_A_COURTESY()),
            "A_TALK_AT_A_TIME" => Some(g_A_TALK_AT_A_TIME()),
            "A_TALK_AT_A_TIME_ASK" => Some(g_A_TALK_AT_A_TIME_ASK()),
            "A_TALK_AT_A_TIME_HARM" => Some(g_A_TALK_AT_A_TIME_HARM()),
            "THE_TIME" => Some(g_THE_TIME()),
            "_A_MARKER" => Some(g__A_MARKER()),
            "_DAY" => Some(g__DAY()),
            "_PART" => Some(g__PART()),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_COMPANY_ASK" => s_A_COMPANY_ASK(v),
            "A_COURTESY" => s_A_COURTESY(v),
            "A_TALK_AT_A_TIME" => s_A_TALK_AT_A_TIME(v),
            "A_TALK_AT_A_TIME_ASK" => s_A_TALK_AT_A_TIME_ASK(v),
            "A_TALK_AT_A_TIME_HARM" => s_A_TALK_AT_A_TIME_HARM(v),
            "THE_TIME" => s_THE_TIME(v),
            "_A_MARKER" => s__A_MARKER(v),
            "_DAY" => s__DAY(v),
            "_PART" => s__PART(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.talktime", module_obj);
    pyrt::xrefs::reg_fn("compose.talktime", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.talktime", "A_COMPANY_ASK", g_A_COMPANY_ASK, s_A_COMPANY_ASK);
    pyrt::xrefs::reg_access("compose.talktime", "A_COURTESY", g_A_COURTESY, s_A_COURTESY);
    pyrt::xrefs::reg_access("compose.talktime", "A_TALK_AT_A_TIME", g_A_TALK_AT_A_TIME, s_A_TALK_AT_A_TIME);
    pyrt::xrefs::reg_access("compose.talktime", "A_TALK_AT_A_TIME_ASK", g_A_TALK_AT_A_TIME_ASK, s_A_TALK_AT_A_TIME_ASK);
    pyrt::xrefs::reg_access("compose.talktime", "A_TALK_AT_A_TIME_HARM", g_A_TALK_AT_A_TIME_HARM, s_A_TALK_AT_A_TIME_HARM);
    pyrt::xrefs::reg_access("compose.talktime", "THE_TIME", g_THE_TIME, s_THE_TIME);
    pyrt::xrefs::reg_access("compose.talktime", "_A_MARKER", g__A_MARKER, s__A_MARKER);
    pyrt::xrefs::reg_access("compose.talktime", "_DAY", g__DAY, s__DAY);
    pyrt::xrefs::reg_access("compose.talktime", "_PART", g__PART, s__PART);
}

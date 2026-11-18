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


thread_local! { static G_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASIER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EASIER_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOURS_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAN_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AWAY() -> Py { ensure_init(); G_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AWAY(v: Py) { ensure_init(); G_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g_EASIER_ASK() -> Py { ensure_init(); G_EASIER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASIER_ASK(v: Py) { ensure_init(); G_EASIER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_EASIER_TAIL() -> Py { ensure_init(); G_EASIER_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EASIER_TAIL(v: Py) { ensure_init(); G_EASIER_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY_NAMES() -> Py { ensure_init(); G_FAMILY_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY_NAMES(v: Py) { ensure_init(); G_FAMILY_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING() -> Py { ensure_init(); G_FEELING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING(v: Py) { ensure_init(); G_FEELING.with(|c| *c.borrow_mut() = v); }
pub fn g_HOURS_KEY() -> Py { ensure_init(); G_HOURS_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOURS_KEY(v: Py) { ensure_init(); G_HOURS_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS() -> Py { ensure_init(); G_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); G_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK() -> Py { ensure_init(); G_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); G_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST() -> Py { ensure_init(); G_MOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST(v: Py) { ensure_init(); G_MOST.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_ASK() -> Py { ensure_init(); G_PLAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_ASK(v: Py) { ensure_init(); G_PLAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAN_TAIL() -> Py { ensure_init(); G_PLAN_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAN_TAIL(v: Py) { ensure_init(); G_PLAN_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP_ASK() -> Py { ensure_init(); G_STOP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP_ASK(v: Py) { ensure_init(); G_STOP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP_TAIL() -> Py { ensure_init(); G_STOP_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP_TAIL(v: Py) { ensure_init(); G_STOP_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The turn after the hours: does that get easier, how do I stop, how do I plan.\n\nThe bot has just answered a line about the week somebody works - \"i start at six so im up at\nfive\", \"i work from home now\" - and the next line asks the thing that follows from it:\n\n```\ni start at six so im up at five / does that get easier\n  before: That's beyond what I know well, sorry. Searching for it will turn up better sources\n\ni work from home now / i never know when to stop\n  before: Got it. Is there anything you'd like a hand with?\n```\n\nBoth are the same failure: the bot forgets the week it was just talking about and answers the\nfollow-up as if the line before had never been said. So the family the hours line named is kept\nfor one turn in `memory[\"hours\"]`, and the follow-up is answered from it - an early start is\nanswered about the going to bed, working from home about the day having no end.\n\nThe rule is narrow on purpose:\n* the hours have to be the turn before this one, so a \"does that get easier\" three turns later,\n  after something else entirely, is not this conversation's question;\n* the line has to be one of the three things people actually ask after saying their hours - will\n  it get easier, how do I switch off, how do I plan around it - and nothing else;\n* a death in the last few turns, a body that hurts, bad news or a feeling all keep their own\n  answers, whatever was said about the hours before them;\n* it never fires on the same line twice, and the pools are chosen off the line, so two people who\n  both say \"i work nights\" do not get the same sentence back.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_HOURS_KEY(py_str("hours"));
    s_FAMILY_NAMES(py_dict(vec![(py_str("nights"), py_str("nights")), (py_str("early"), py_str("an early start like that")), (py_str("late"), py_str("a finish that late")), (py_str("long"), py_str("shifts that long")), (py_str("spread"), py_str("a week laid out like that")), (py_str("home"), py_str("working from home"))]));
    s_EASIER_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|will|would|is|did)\\s+(?:that|it|this|they)\\s+(?:ever\\s+|eventually\\s+)?(?:get|gets|be|become|gets)\\s+(?:any\\s+)?(?:easier|better|any easier)\\b|\\b(?:will|do)\\s+i\\s+(?:ever\\s+)?get (?:used to|over)\\s+(?:it|them|that)\\b|\\b(?:it|that|this|they)\\s+(?:ever\\s+)?gets?\\s+(?:any\\s+)?easier\\b|\\b(?:does|will) (?:that|it|this) (?:ever )?stop\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:never|can'?t|cant|don'?t|dont|just can'?t)\\s+(?:ever\\s+)?(?:know when to stop|switch off|stop|wind down|log off)\\b|\\bhow do i (?:switch off|stop|wind down|log off)\\b|\\b(?:hard|impossible) to switch off\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:its|it is|it'?s|that is)\\s+(?:so\\s+)?hard to plan\\b|\\b(?:i|we)\\s+(?:can'?t|cant|never)\\s+plan\\b|\\bnever know my week\\b|\\bhow do i plan\\b|\\bimpossible to plan\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EASIER_TAIL(py_tuple(vec![py_str("It does, and the change is in the sleep rather than in the work: {hours} only start to sit right once the going to bed moves with them, and that takes a week or two rather than a night."), py_str("Yes, in stages. The first fortnight with {hours} is the worst of it, and what is still hard a month in is worth taking to a doctor, because that part is not the adjustment any more."), py_str("It gets easier. The body is not the slow part - the evenings are, and those only settle once they have a fixed edge at both ends.")]));
    s_STOP_TAIL(py_tuple(vec![py_str("Then put the end on the clock rather than on the work: a time that finishes the day whatever is still undone, because the work never runs out by itself."), py_str("Take the last hour back deliberately. It is easier to protect a slot than a mood, and a day with no end will take every open minute it is given.")]));
    s_PLAN_TAIL(py_tuple(vec![py_str("Plan less and keep it fixed: two or three things a week that cannot move is all a plan should hold, and the rest of the week is better left loose."), py_str("Keep the sleep and one day off in the same place every week. Those two are what the rest of the week hangs on, and they hold even when the week moves under them.")]));
    s_KINDS(py_tuple(vec![py_tuple(vec![py_str("easier"), g_EASIER_ASK(), g_EASIER_TAIL()]), py_tuple(vec![py_str("stop"), g_STOP_ASK(), g_STOP_TAIL()]), py_tuple(vec![py_str("plan"), g_PLAN_ASK(), g_PLAN_TAIL()])]));
    s_AWAY(stdlib_call("re", "compile", &[py_str("\\b(?:die|died|dies|dying|death|dead|kill|killed|suicide|funeral|hospital|ambulance|bleeding|blood|chest|breathless|sore|pain|hurts|aching|ill|sick|diagnosed|cancer|redundant|sacked|fired|dumped|divorce|miss|miss him|miss her)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING(stdlib_call("re", "compile", &[py_str("\\b(?:sad|lonely|alone|awful|terrible|horrible|miserable|hopeless|worthless|hate myself|cant cope|can'?t cope|breaking down|crying|anxious|depressed|scared|worried sick)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST(Py::Int(12));
    s_LOOK_BACK(Py::Int(1));
}
pub fn note(args: &[Py]) -> Py {
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
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Keep the family of week the hours line named, for the turn after it.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&family.clone())) } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    turn = if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    let _ = py_setitem(&memory.clone(), &g_HOURS_KEY(), py_dict(vec![(py_str("family"), call_builtin("str", &[family.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), turn.clone())]));
    Py::None
}
pub fn remembered(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut held: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The family named on the last turn, or None when the hours are older than that.");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[g_HOURS_KEY()]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&held.clone(), "get", &[py_str("turn")]), &op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_LOOK_BACK())))) {
    return Py::None;
    }
    family = call_builtin("str", &[{ let __b = mcall(&held.clone(), "get", &[py_str("family")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(py_contains(&g_FAMILY_NAMES(), &family.clone()))) { family.clone() } else { Py::None };
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
    let mut _kind: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut family: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("A reply to the follow-up about the hours just said, or None when this is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&text.clone(), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_AWAY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_FEELING(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    family = remembered(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&family.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    pool = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[clean.clone()])))) {
    continue;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("hours".to_string(), geti(&g_FAMILY_NAMES(), &family.clone()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), clean.clone()]), "format", args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.hours".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AWAY" => Some(g_AWAY()),
            "EASIER_ASK" => Some(g_EASIER_ASK()),
            "EASIER_TAIL" => Some(g_EASIER_TAIL()),
            "FAMILY_NAMES" => Some(g_FAMILY_NAMES()),
            "FEELING" => Some(g_FEELING()),
            "HOURS_KEY" => Some(g_HOURS_KEY()),
            "KINDS" => Some(g_KINDS()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "MOST" => Some(g_MOST()),
            "PLAN_ASK" => Some(g_PLAN_ASK()),
            "PLAN_TAIL" => Some(g_PLAN_TAIL()),
            "STOP_ASK" => Some(g_STOP_ASK()),
            "STOP_TAIL" => Some(g_STOP_TAIL()),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "note" => Some(py_func("note", |args: &[Py]| note(args))),
            "pick" => Some(g_pick()),
            "remembered" => Some(py_func("remembered", |args: &[Py]| remembered(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AWAY" => s_AWAY(v),
            "EASIER_ASK" => s_EASIER_ASK(v),
            "EASIER_TAIL" => s_EASIER_TAIL(v),
            "FAMILY_NAMES" => s_FAMILY_NAMES(v),
            "FEELING" => s_FEELING(v),
            "HOURS_KEY" => s_HOURS_KEY(v),
            "KINDS" => s_KINDS(v),
            "LOOK_BACK" => s_LOOK_BACK(v),
            "MOST" => s_MOST(v),
            "PLAN_ASK" => s_PLAN_ASK(v),
            "PLAN_TAIL" => s_PLAN_TAIL(v),
            "STOP_ASK" => s_STOP_ASK(v),
            "STOP_TAIL" => s_STOP_TAIL(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.hours", module_obj);
    pyrt::xrefs::reg_fn("memory.hours", "note", py_func("note", |args: &[Py]| note(args)));
    pyrt::xrefs::reg_fn("memory.hours", "remembered", py_func("remembered", |args: &[Py]| remembered(args)));
    pyrt::xrefs::reg_fn("memory.hours", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("memory.hours", "AWAY", g_AWAY, s_AWAY);
    pyrt::xrefs::reg_access("memory.hours", "EASIER_ASK", g_EASIER_ASK, s_EASIER_ASK);
    pyrt::xrefs::reg_access("memory.hours", "EASIER_TAIL", g_EASIER_TAIL, s_EASIER_TAIL);
    pyrt::xrefs::reg_access("memory.hours", "FAMILY_NAMES", g_FAMILY_NAMES, s_FAMILY_NAMES);
    pyrt::xrefs::reg_access("memory.hours", "FEELING", g_FEELING, s_FEELING);
    pyrt::xrefs::reg_access("memory.hours", "HOURS_KEY", g_HOURS_KEY, s_HOURS_KEY);
    pyrt::xrefs::reg_access("memory.hours", "KINDS", g_KINDS, s_KINDS);
    pyrt::xrefs::reg_access("memory.hours", "LOOK_BACK", g_LOOK_BACK, s_LOOK_BACK);
    pyrt::xrefs::reg_access("memory.hours", "MOST", g_MOST, s_MOST);
    pyrt::xrefs::reg_access("memory.hours", "PLAN_ASK", g_PLAN_ASK, s_PLAN_ASK);
    pyrt::xrefs::reg_access("memory.hours", "PLAN_TAIL", g_PLAN_TAIL, s_PLAN_TAIL);
    pyrt::xrefs::reg_access("memory.hours", "STOP_ASK", g_STOP_ASK, s_STOP_ASK);
    pyrt::xrefs::reg_access("memory.hours", "STOP_TAIL", g_STOP_TAIL, s_STOP_TAIL);
    pyrt::xrefs::reg_access("memory.hours", "pick", g_pick, s_pick);
}

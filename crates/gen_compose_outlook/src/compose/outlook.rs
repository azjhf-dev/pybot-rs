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


thread_local! { static G_BODY_PARTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES_IN_AN_EMERGENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES_IN_CRISIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES_WHEN_SOMEONE_IS_HURTING_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LINES_WITH_HELP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUTCOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POINTS_AT_THE_THREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_THING_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.outlook: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_BODY_PARTS() -> Py { ensure_init(); G_BODY_PARTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_PARTS(v: Py) { ensure_init(); G_BODY_PARTS.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_WORDS() -> Py { ensure_init(); G_HEALTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_WORDS(v: Py) { ensure_init(); G_HEALTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES_IN_AN_EMERGENCY() -> Py { ensure_init(); G_LINES_IN_AN_EMERGENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES_IN_AN_EMERGENCY(v: Py) { ensure_init(); G_LINES_IN_AN_EMERGENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES_IN_CRISIS() -> Py { ensure_init(); G_LINES_IN_CRISIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES_IN_CRISIS(v: Py) { ensure_init(); G_LINES_IN_CRISIS.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES_WHEN_SOMEONE_IS_HURTING_YOU() -> Py { ensure_init(); G_LINES_WHEN_SOMEONE_IS_HURTING_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES_WHEN_SOMEONE_IS_HURTING_YOU(v: Py) { ensure_init(); G_LINES_WHEN_SOMEONE_IS_HURTING_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_LINES_WITH_HELP() -> Py { ensure_init(); G_LINES_WITH_HELP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LINES_WITH_HELP(v: Py) { ensure_init(); G_LINES_WITH_HELP.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM_WORDS() -> Py { ensure_init(); G_SYMPTOM_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM_WORDS(v: Py) { ensure_init(); G_SYMPTOM_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__OUTCOME() -> Py { ensure_init(); G__OUTCOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUTCOME(v: Py) { ensure_init(); G__OUTCOME.with(|c| *c.borrow_mut() = v); }
pub fn g__POINTS_AT_THE_THREAD() -> Py { ensure_init(); G__POINTS_AT_THE_THREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POINTS_AT_THE_THREAD(v: Py) { ensure_init(); G__POINTS_AT_THE_THREAD.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_THING_ITSELF() -> Py { ensure_init(); G__THE_THING_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_THING_ITSELF(v: Py) { ensure_init(); G__THE_THING_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g__WORDS() -> Py { ensure_init(); G__WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORDS(v: Py) { ensure_init(); G__WORDS.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that asks how the thing turns out, and the lines that answer it inside the thread.\n\nThe safety answer says what to do now. The line after it is often not \"is that bad\" but \"does it\nget better\", \"will i always feel like this\", \"will things get better\" - the person asking about the\noutcome, with the subject pointed at rather than named. No pool held an answer to that, so the floor\nguessed at what the pronoun meant:\n\n    i cut myself off and i wont be around / does it get better\n      \"Say what you are asking about and I'll answer it properly rather than guess.\"\n    my husband hit me / will things get better\n      \"I'm not sure about that one, sorry - a quick search will get you a long way.\"\n    i cant breathe / will i always feel like this\n      \"I don't know enough to answer that honestly. A search should sort you out quickly.\"\n\nA punt and a search hedge are the worst thing the bot can say to a person who has just said they\nmay not be around. Four of sixty turns of that battery were a punt, four a hedge, and the rest a card\nabout something else entirely: an overdose answered with \"Compare the price per kilo or per 100g on\nthe shelf label\".\n\nThree things are read here, and nothing else:\n\n* `asks_the_outlook` - the line asks how it turns out, with a pronoun for the subject and nothing of\n  its own. A line that names the symptom (\"will it stop bleeding\") is about the symptom, and the\n  safety answer has already answered that.\n* `points_at_the_thread` - the same pronoun shape without the outcome word. The arbiter uses it to\n  say that a follow-up has no subject of its own, so the thread it points at owns the turn whatever\n  the understanding engine's topic label says about it.\n* the pools. Three of them, because the true thing to say is not the same in a crisis, in a crisis\n  with help already on the way, and after first aid.\n\nThe wording keeps to what is true: nobody can promise how it goes, and the line is the part that can\nbe acted on now. A pool line never repeats one already said in this conversation - `_choose` in\n`bot/compose/fallback_chain.py` reads the memory for that, the same as the pools beside it.\n");
    s__WORDS(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_AT_THE_THREAD(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|but\\s+|so\\s+|ok(?:ay)?\\s+|also\\s+)?(?:does|do|did|will|would|can|could|is|are|am|was|were|has|have|should)\\s+(?:it|its|it'?s|that|this|them|they|things|i|we|life|everything)\\b|^\\W*(?:what\\s+(?:happens|happened|now)|where\\s+does\\s+that|how\\s+long\\s+(?:till|until|before|does|will))")]).unwrap_or_else(|e| exc::raise(e)));
    s__OUTCOME(stdlib_call("re", "compile", &[py_str("\\b(?:better|worse|normal|again|ok|okay|alright|fine|same|improve|improves|change|changes|always|ever|forever|go(?:es)?\\s+away|get\\s+over|recover|heals?|happens?|now|tomorrow|feel\\s+like\\s+this)\\b|\\b(?:stop|stops|stopped|end|ends|pass|passes|ease|eases|last|lasts)\\b\\s*(?:soon|now|altogether|completely|on\\s+its\\s+own|eventually|in\\s+a\\s+day\\s+or\\s+two)?\\s*[?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    let __try1 = exc::catch(|| {
    s__THE_THING_ITSELF(call_builtin("frozenset", &[op_bitor(&op_bitor(&pyrt::xrefs::xget("compose.fallback_data", "BODY_PARTS"), &pyrt::xrefs::xget("compose.fallback_data", "HEALTH_WORDS")), &pyrt::xrefs::xget("compose.fallback_data", "SYMPTOM_WORDS"))]).unwrap_or_else(|e| exc::raise(e)));
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
    s__THE_THING_ITSELF(call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)));
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_LINES_IN_CRISIS(py_tuple(vec![py_str("I can't promise how it goes from here, and anyone who says they can is guessing. What is in your hands is the line - people ring it again the same day, and that is what it is for. Ring it if it turns, and tell me what the next hour looks like."), py_str("Nobody can put a shape on that from here, and I won't pretend to. The part you can act on is the call: they have heard all of this before, at three in the morning included, so use it again the moment it gets heavier."), py_str("That is not a thing I can answer for you, and it does not need an answer tonight. The number does not run out - ring it again if it turns, and say what today is like for you.")]));
    s_LINES_IN_AN_EMERGENCY(py_tuple(vec![py_str("I can't promise how it goes, and nobody can from here. The thing that changes it is the call in the answer above - if you have not rung yet, ring now and stay on the line with them; they can hear what I can't."), py_str("Nobody can say how it goes, and the part you can act on is the call you were told to make. If you have made it, do what they say over anything I could add; if not, make it before anything else."), py_str("That is not a thing I can answer for you, and my guess would be worth less than the one in the answer above. The call is what stands between you and how it goes - everything else waits.")]));
    s_LINES_WITH_HELP(py_tuple(vec![py_str("I can't promise how it goes, and nobody honestly can. What they told you on that call is worth more than anything I could add, so stay with it - and ring again if it turns rather than waiting it out."), py_str("Nobody can say how it goes from here, and the part in your hands is what the line told you to do next. Do that, and ring them back if anything changes."), py_str("That is not something I can answer for you, and it does not need an answer tonight. Somebody who can help knows about it now - keep that number open and use it again if it gets heavier.")]));
    s_LINES_WHEN_SOMEONE_IS_HURTING_YOU(py_tuple(vec![py_str("I can't promise how it goes, and nobody honestly can - not from a line of chat. What I can say is that none of it is your fault, and the number in the answer above is for exactly the question you just asked; they hear it all day."), py_str("Nobody can say how it goes, and you should not have to work it out alone. The line above can talk it through with you - what you want, not only what is happening - and that is worth more than my guess."), py_str("I can't say how it goes from here, and I would be making it up if I tried. What is in your hands is the call: they will not tell you to leave or to stay, and asking costs nothing.")]));
}
pub fn points_at_the_thread(args: &[Py]) -> Py {
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
    let _ = py_str("True for a short line whose whole subject is a pronoun standing for the thread above.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__POINTS_AT_THE_THREAD(), "match", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_the_outlook(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { Py::None };
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks how the thing turns out, pointing at it with a pronoun.\n\n    `clean` is the normalised line when the caller has one; the reading is the same either way.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[if py_truthy(&Py::Bool(!py_is(&clean.clone(), &Py::None))) { clean.clone() } else { { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__OUTCOME(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = g__THE_THING_ITSELF(); if !py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[mcall(&g__WORDS(), "findall", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g__THE_THING_ITSELF()) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__POINTS_AT_THE_THREAD(), "match", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut in_crisis: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "in_crisis") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: in_crisis"))) };
    let mut help_is_coming: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "help_is_coming") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: help_is_coming"))) };
    let mut asked_them_to_call: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked_them_to_call") { v.clone() }
    else { Py::Bool(false) };
    let mut abuse: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "abuse") { v.clone() }
    else { Py::Bool(false) };
    let _ = py_str("The pool for the threads a forecast line does not fit, or None.\n\n    Three cases, because in each the forecast pool's \"the ones treating you can see it\" is not\n    true: somebody being hurt (the outcome asked about is the relationship), a crisis line with\n    the call not made (the number stays open), and a crisis with help on the way (what they say\n    comes first). A card that said ring 999 now counts as a crisis for this: the call is the next\n    step, made or not.\n\n    Everything else comes back None, and the caller uses the forecast line the composer already\n    has for a thread it has just answered: that one fits a condition as well as an injury, which\n    \"most of these settle in a day or two\" does not.\n    ");
    if py_truthy(&abuse.clone()) {
    return g_LINES_WHEN_SOMEONE_IS_HURTING_YOU();
    }
    if py_truthy(&in_crisis.clone()) {
    return if py_truthy(&help_is_coming.clone()) { g_LINES_WITH_HELP() } else { g_LINES_IN_CRISIS() };
    }
    if py_truthy(&asked_them_to_call.clone()) {
    return if py_truthy(&help_is_coming.clone()) { g_LINES_WITH_HELP() } else { g_LINES_IN_AN_EMERGENCY() };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.outlook".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BODY_PARTS" => Some(g_BODY_PARTS()),
            "HEALTH_WORDS" => Some(g_HEALTH_WORDS()),
            "LINES_IN_AN_EMERGENCY" => Some(g_LINES_IN_AN_EMERGENCY()),
            "LINES_IN_CRISIS" => Some(g_LINES_IN_CRISIS()),
            "LINES_WHEN_SOMEONE_IS_HURTING_YOU" => Some(g_LINES_WHEN_SOMEONE_IS_HURTING_YOU()),
            "LINES_WITH_HELP" => Some(g_LINES_WITH_HELP()),
            "SYMPTOM_WORDS" => Some(g_SYMPTOM_WORDS()),
            "_OUTCOME" => Some(g__OUTCOME()),
            "_POINTS_AT_THE_THREAD" => Some(g__POINTS_AT_THE_THREAD()),
            "_THE_THING_ITSELF" => Some(g__THE_THING_ITSELF()),
            "_WORDS" => Some(g__WORDS()),
            "asks_the_outlook" => Some(py_func("asks_the_outlook", |args: &[Py]| asks_the_outlook(args))),
            "points_at_the_thread" => Some(py_func("points_at_the_thread", |args: &[Py]| points_at_the_thread(args))),
            "the_pool" => Some(py_func("the_pool", |args: &[Py]| the_pool(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "BODY_PARTS" => s_BODY_PARTS(v),
            "HEALTH_WORDS" => s_HEALTH_WORDS(v),
            "LINES_IN_AN_EMERGENCY" => s_LINES_IN_AN_EMERGENCY(v),
            "LINES_IN_CRISIS" => s_LINES_IN_CRISIS(v),
            "LINES_WHEN_SOMEONE_IS_HURTING_YOU" => s_LINES_WHEN_SOMEONE_IS_HURTING_YOU(v),
            "LINES_WITH_HELP" => s_LINES_WITH_HELP(v),
            "SYMPTOM_WORDS" => s_SYMPTOM_WORDS(v),
            "_OUTCOME" => s__OUTCOME(v),
            "_POINTS_AT_THE_THREAD" => s__POINTS_AT_THE_THREAD(v),
            "_THE_THING_ITSELF" => s__THE_THING_ITSELF(v),
            "_WORDS" => s__WORDS(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.outlook", module_obj);
    pyrt::xrefs::reg_fn("compose.outlook", "points_at_the_thread", py_func("points_at_the_thread", |args: &[Py]| points_at_the_thread(args)));
    pyrt::xrefs::reg_fn("compose.outlook", "asks_the_outlook", py_func("asks_the_outlook", |args: &[Py]| asks_the_outlook(args)));
    pyrt::xrefs::reg_fn("compose.outlook", "the_pool", py_func("the_pool", |args: &[Py]| the_pool(args)));
    pyrt::xrefs::reg_access("compose.outlook", "BODY_PARTS", g_BODY_PARTS, s_BODY_PARTS);
    pyrt::xrefs::reg_access("compose.outlook", "HEALTH_WORDS", g_HEALTH_WORDS, s_HEALTH_WORDS);
    pyrt::xrefs::reg_access("compose.outlook", "LINES_IN_AN_EMERGENCY", g_LINES_IN_AN_EMERGENCY, s_LINES_IN_AN_EMERGENCY);
    pyrt::xrefs::reg_access("compose.outlook", "LINES_IN_CRISIS", g_LINES_IN_CRISIS, s_LINES_IN_CRISIS);
    pyrt::xrefs::reg_access("compose.outlook", "LINES_WHEN_SOMEONE_IS_HURTING_YOU", g_LINES_WHEN_SOMEONE_IS_HURTING_YOU, s_LINES_WHEN_SOMEONE_IS_HURTING_YOU);
    pyrt::xrefs::reg_access("compose.outlook", "LINES_WITH_HELP", g_LINES_WITH_HELP, s_LINES_WITH_HELP);
    pyrt::xrefs::reg_access("compose.outlook", "SYMPTOM_WORDS", g_SYMPTOM_WORDS, s_SYMPTOM_WORDS);
    pyrt::xrefs::reg_access("compose.outlook", "_OUTCOME", g__OUTCOME, s__OUTCOME);
    pyrt::xrefs::reg_access("compose.outlook", "_POINTS_AT_THE_THREAD", g__POINTS_AT_THE_THREAD, s__POINTS_AT_THE_THREAD);
    pyrt::xrefs::reg_access("compose.outlook", "_THE_THING_ITSELF", g__THE_THING_ITSELF, s__THE_THING_ITSELF);
    pyrt::xrefs::reg_access("compose.outlook", "_WORDS", g__WORDS, s__WORDS);
}

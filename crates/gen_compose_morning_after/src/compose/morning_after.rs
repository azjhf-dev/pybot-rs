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


thread_local! { static G_AN_EMERGENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CONSEQUENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HANGOVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MORNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MORNING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUESTION_ABOUT_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STILL_TO_COME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STRONG_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_THAT_HAS_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WEAK_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.morning_after: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AN_EMERGENCY() -> Py { ensure_init(); G_AN_EMERGENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_EMERGENCY(v: Py) { ensure_init(); G_AN_EMERGENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CONSEQUENCE() -> Py { ensure_init(); G_A_CONSEQUENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CONSEQUENCE(v: Py) { ensure_init(); G_A_CONSEQUENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HANGOVER() -> Py { ensure_init(); G_A_HANGOVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HANGOVER(v: Py) { ensure_init(); G_A_HANGOVER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MORNING() -> Py { ensure_init(); G_A_MORNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MORNING(v: Py) { ensure_init(); G_A_MORNING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MORNING_AFTER() -> Py { ensure_init(); G_A_MORNING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MORNING_AFTER(v: Py) { ensure_init(); G_A_MORNING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUESTION_ABOUT_IT() -> Py { ensure_init(); G_A_QUESTION_ABOUT_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUESTION_ABOUT_IT(v: Py) { ensure_init(); G_A_QUESTION_ABOUT_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STILL_TO_COME() -> Py { ensure_init(); G_A_STILL_TO_COME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STILL_TO_COME(v: Py) { ensure_init(); G_A_STILL_TO_COME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STRONG_STATE() -> Py { ensure_init(); G_A_STRONG_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STRONG_STATE(v: Py) { ensure_init(); G_A_STRONG_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_THAT_HAS_GONE() -> Py { ensure_init(); G_A_TIME_THAT_HAS_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_THAT_HAS_GONE(v: Py) { ensure_init(); G_A_TIME_THAT_HAS_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WEAK_STATE() -> Py { ensure_init(); G_A_WEAK_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WEAK_STATE(v: Py) { ensure_init(); G_A_WEAK_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); G_SOMEONE_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); G_SOMEONE_ELSE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A night of drinking, reported the day after.\n\nPeople say this in one breath and mean one of two things - \"i got really drunk last night\" can be\na good story or a regret - and the bot answered both badly:\n\n    i was blind drunk last night      -> \"Good to hear. What was the best bit?\"\n    i got pissed last night           -> \"Good to hear. What was the best bit?\"\n    i had one too many at the pub     -> \"Nice one. How did the rest of it go?\"\n    i went on a bender last night     -> \"Sounds alright. Anything good in it?\"\n    i got wasted at the party         -> \"Noted: you got wasted at the party.\"\n    i was bladdered at the do         -> \"Bladdered is a thing you already know how to do...\"\n    i woke up still drunk             -> \"Thanks for telling me — I'm here if you want to dig in...\"\n\nThe cheers are the day pools reading a past-tense account of an evening as an evening that went\nwell, which is the one reply a person who is not proud of it cannot use; the receipt and the\nskills line read a single word (\"wasted\", \"bladdered\") for its own subject. What a friend says to\neither telling is the same thing: how are you today, and here is the one thing that helps. So\nthis is one reading for the whole family, and it never tells them the night was good or bad -\nthe person already knows which it was.\n\nThe reading turns on which word it is. \"drunk\", \"plastered\", \"bladdered\", \"off my face\", \"on the\nlash\", \"one too many\" are only ever about drink, so they carry the reading on their own. \"wasted\",\n\"smashed\", \"steaming\", \"tipsy\", \"boozy\" and the bare \"drinking\" are ordinary words in other\nlines - \"i wasted my money on that\", \"i smashed my phone\", \"i drink too much coffee in the\nmorning\" - so they need a night that has gone (\"last night\", \"this morning\", \"woke up\") beside\nthem. \"hangover\" and \"hanging\" are the weakest of all and need the morning said out loud, because\n\"hanging the curtains\" is a job, and \"rough\" is left out altogether: \"im rough today\" is as likely\nto be a bug as a night out.\n\nIt stands down for somebody else's drinking (\"my husband was drunk again last night\" is the\nrelationship card's, \"we got drunk at the wedding\" is a wedding), for a question about drinking\n(\"should i stop drinking\" has an answer in the knowledge base), for whatever the night did\nafterwards (\"i was drunk and i said something stupid\" wants the apology, \"i got drunk and drove\nhome\" is the driving, \"i was plastered and fell over\" is the fall), and for the night that ended\nin an ambulance, which the safety readers own.\n\nIt is asked early in the chain, because the six shapes above were answered by six different pools\ndeep inside it - the day pools, the echo, the skills line, the task line and the sober-years pool\nthat read \"plastered\" as \"off the drink\" - and a reading that sits next to the day pools never\nsees the other five. A knowledge card still wins the turn, since the composer is last: \"i have\nsuch a hangover\" keeps the water, sleep and food card.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_STRONG_STATE"); }
    s_A_STRONG_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:drunk|drunken|pissed(?!\\s+off)|plastered|bladdered|trolleyed|mullered|mangled|sozzled|paralytic|off\\s+(?:my|his|her|their|our)\\s+(?:face|head|tits)|on\\s+the\\s+lash|bender|(?:a\\s+)?few\\s+too\\s+many|one\\s+too\\s+many|too\\s+much\\s+(?:to\\s+drink|wine|beer|lager|booze)|too\\s+much\\s+last\\s+night|heavy\\s+night|big\\s+night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_WEAK_STATE"); }
    s_A_WEAK_STATE(stdlib_call("re", "compile", &[py_str("\\b(?:wasted|smashed|steaming|tipsy|boozy|lash|binge|bendered|drank|hammered|leathered|wrecked|half\\s+cut|been\\s+drinking|was\\s+drinking|were\\s+drinking)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_TIME_THAT_HAS_GONE"); }
    s_A_TIME_THAT_HAS_GONE(stdlib_call("re", "compile", &[py_str("\\b(?:last\\s+night|last\\s+evening|yesterday|the\\s+night\\s+before|all\\s+night|overnight|this\\s+morning|woke\\s+up|till\\s+\\d|til\\s+\\d|until\\s+\\d|(?:fri|satur|sun)day\\s+night|at\\s+the\\s+weekend|night\\s+out|at\\s+the\\s+(?:party|pub|club|do|wedding|works\\s+do|leaving\\s+do)|at\\s+a\\s+party|on\\s+a\\s+night\\s+out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_HANGOVER"); }
    s_A_HANGOVER(stdlib_call("re", "compile", &[py_str("\\b(?:hung\\s?over|hangover|hanging|woke\\s+up\\s+rough|rough\\s+as\\s+a\\s+badger)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_MORNING"); }
    s_A_MORNING(stdlib_call("re", "compile", &[py_str("\\b(?:this\\s+morning|today|now|after\\s+last\\s+night)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_CONSEQUENCE"); }
    s_A_CONSEQUENCE(stdlib_call("re", "compile", &[py_str("\\b(?:said|saying|told|texted|messaged|rang|rung|called|drove|drive|drove\\s+home|hit|shouted|argued|arguing|kissed|cheated|spent|ruined|broke|broke\\s+up|fell|fall|lost\\s+my|got\\s+in\\s+a\\s+fight|fight)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_STILL_TO_COME"); }
    s_A_STILL_TO_COME(stdlib_call("re", "compile", &[py_str("\\b(?:tonight|tomorrow|next\\s+(?:week|weekend|month|year)|going\\s+to|gonna|this\\s+weekend|on\\s+friday|on\\s+saturday)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.SOMEONE_ELSE"); }
    s_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:hes|shes|theyre)\\b|\\b(?:my|our|his|her|their|he|she|they|we|us)\\s+(?:\\w+\\s+){0,2}?(?:was|were|got|gets|get|is|are|had|has|keeps?|been)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.AN_EMERGENCY"); }
    s_AN_EMERGENCY(stdlib_call("re", "compile", &[py_str("\\b(?:ambulance|999|911|111|hospital|a\\s?&?\\s?e\\b|passed\\s+out|passing\\s+out|blacked\\s+out|blackout|can'?t\\s+wake|not\\s+breathing|stopped\\s+breathing|alcohol\\s+poisoning|overdose|overdosed|choking|seizure|collapsed|unconscious)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_QUESTION_ABOUT_IT"); }
    s_A_QUESTION_ABOUT_IT(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|could|how\\s+do|how\\s+can|how\\s+to|what\\s+should|any\\s+tips|advice|stop\\s+drinking|give\\s+up|cut\\s+down|cutting\\s+down|quitting|quitting|help)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.morning_after.A_MORNING_AFTER"); }
    s_A_MORNING_AFTER(py_tuple(vec![py_str("Ha - how is your head today?"), py_str("Water, something plain to eat and an early night is most of the cure. How is today going?"), py_str("That is a long night. Are you feeling it today, or did you get away with it?"), py_str("How are you this morning? A pint of water, food and time, in that order."), py_str("How bad is it - banging head, or just tired and skint?"), py_str("That sounds like one of those nights. Are you able to take it easy today?"), py_str("Right. How are you feeling now?")]));
}
pub fn reads_as_a_morning_after(args: &[Py]) -> Py {
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
    let mut gone: Py = Py::None;
    let mut hangover: Py = Py::None;
    let mut said: Py = Py::None;
    let mut strong: Py = Py::None;
    let mut weak: Py = Py::None;
    let _ = py_str("True when the line reports a night of drinking that has already happened.\n\n    `clean` is the understood line: lowercased, with the punctuation tidied.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_AN_EMERGENCY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_QUESTION_ABOUT_IT(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_CONSEQUENCE(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    gone = mcall(&g_A_TIME_THAT_HAS_GONE(), "search", &[clean.clone()]);
    strong = mcall(&g_A_STRONG_STATE(), "search", &[clean.clone()]);
    weak = if py_truthy(&gone.clone()) { mcall(&g_A_WEAK_STATE(), "search", &[clean.clone()]) } else { Py::None };
    hangover = if py_truthy(&mcall(&g_A_MORNING(), "search", &[clean.clone()])) { mcall(&g_A_HANGOVER(), "search", &[clean.clone()]) } else { Py::None };
    said = { let __b = strong.clone(); if py_truthy(&__b) { __b } else { { let __b = weak.clone(); if py_truthy(&__b) { __b } else { hangover.clone() } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_SOMEONE_ELSE(), "search", &[slice_or_raise(&clean.clone(), &Py::None, &mcall(&said.clone(), "start", &[]), &Py::None)])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&gone.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_STILL_TO_COME(), "search", &[clean.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.morning_after".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_EMERGENCY" => Some(g_AN_EMERGENCY()),
            "A_CONSEQUENCE" => Some(g_A_CONSEQUENCE()),
            "A_HANGOVER" => Some(g_A_HANGOVER()),
            "A_MORNING" => Some(g_A_MORNING()),
            "A_MORNING_AFTER" => Some(g_A_MORNING_AFTER()),
            "A_QUESTION_ABOUT_IT" => Some(g_A_QUESTION_ABOUT_IT()),
            "A_STILL_TO_COME" => Some(g_A_STILL_TO_COME()),
            "A_STRONG_STATE" => Some(g_A_STRONG_STATE()),
            "A_TIME_THAT_HAS_GONE" => Some(g_A_TIME_THAT_HAS_GONE()),
            "A_WEAK_STATE" => Some(g_A_WEAK_STATE()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "reads_as_a_morning_after" => Some(py_func("reads_as_a_morning_after", |args: &[Py]| reads_as_a_morning_after(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AN_EMERGENCY" => s_AN_EMERGENCY(v),
            "A_CONSEQUENCE" => s_A_CONSEQUENCE(v),
            "A_HANGOVER" => s_A_HANGOVER(v),
            "A_MORNING" => s_A_MORNING(v),
            "A_MORNING_AFTER" => s_A_MORNING_AFTER(v),
            "A_QUESTION_ABOUT_IT" => s_A_QUESTION_ABOUT_IT(v),
            "A_STILL_TO_COME" => s_A_STILL_TO_COME(v),
            "A_STRONG_STATE" => s_A_STRONG_STATE(v),
            "A_TIME_THAT_HAS_GONE" => s_A_TIME_THAT_HAS_GONE(v),
            "A_WEAK_STATE" => s_A_WEAK_STATE(v),
            "SOMEONE_ELSE" => s_SOMEONE_ELSE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.morning_after", module_obj);
    pyrt::xrefs::reg_fn("compose.morning_after", "reads_as_a_morning_after", py_func("reads_as_a_morning_after", |args: &[Py]| reads_as_a_morning_after(args)));
    pyrt::xrefs::reg_access("compose.morning_after", "AN_EMERGENCY", g_AN_EMERGENCY, s_AN_EMERGENCY);
    pyrt::xrefs::reg_access("compose.morning_after", "A_CONSEQUENCE", g_A_CONSEQUENCE, s_A_CONSEQUENCE);
    pyrt::xrefs::reg_access("compose.morning_after", "A_HANGOVER", g_A_HANGOVER, s_A_HANGOVER);
    pyrt::xrefs::reg_access("compose.morning_after", "A_MORNING", g_A_MORNING, s_A_MORNING);
    pyrt::xrefs::reg_access("compose.morning_after", "A_MORNING_AFTER", g_A_MORNING_AFTER, s_A_MORNING_AFTER);
    pyrt::xrefs::reg_access("compose.morning_after", "A_QUESTION_ABOUT_IT", g_A_QUESTION_ABOUT_IT, s_A_QUESTION_ABOUT_IT);
    pyrt::xrefs::reg_access("compose.morning_after", "A_STILL_TO_COME", g_A_STILL_TO_COME, s_A_STILL_TO_COME);
    pyrt::xrefs::reg_access("compose.morning_after", "A_STRONG_STATE", g_A_STRONG_STATE, s_A_STRONG_STATE);
    pyrt::xrefs::reg_access("compose.morning_after", "A_TIME_THAT_HAS_GONE", g_A_TIME_THAT_HAS_GONE, s_A_TIME_THAT_HAS_GONE);
    pyrt::xrefs::reg_access("compose.morning_after", "A_WEAK_STATE", g_A_WEAK_STATE, s_A_WEAK_STATE);
    pyrt::xrefs::reg_access("compose.morning_after", "SOMEONE_ELSE", g_SOMEONE_ELSE, s_SOMEONE_ELSE);
}

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


thread_local! { static G_A_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BROKEN_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THE_TROUBLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONNECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUPPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEHICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WATER: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_A_ASK() -> Py { ensure_init(); G_A_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ASK(v: Py) { ensure_init(); G_A_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BROKEN_NOW() -> Py { ensure_init(); G_A_BROKEN_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BROKEN_NOW(v: Py) { ensure_init(); G_A_BROKEN_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THE_TROUBLE() -> Py { ensure_init(); G_A_THE_TROUBLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THE_TROUBLE(v: Py) { ensure_init(); G_A_THE_TROUBLE.with(|c| *c.borrow_mut() = v); }
pub fn g_BILL() -> Py { ensure_init(); G_BILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BILL(v: Py) { ensure_init(); G_BILL.with(|c| *c.borrow_mut() = v); }
pub fn g_CONNECTION() -> Py { ensure_init(); G_CONNECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONNECTION(v: Py) { ensure_init(); G_CONNECTION.with(|c| *c.borrow_mut() = v); }
pub fn g_FILLER() -> Py { ensure_init(); G_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FILLER(v: Py) { ensure_init(); G_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_HEATING() -> Py { ensure_init(); G_HEATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEATING(v: Py) { ensure_init(); G_HEATING.with(|c| *c.borrow_mut() = v); }
pub fn g_SAY() -> Py { ensure_init(); G_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAY(v: Py) { ensure_init(); G_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g_SUPPLY() -> Py { ensure_init(); G_SUPPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUPPLY(v: Py) { ensure_init(); G_SUPPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_VEHICLE() -> Py { ensure_init(); G_VEHICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEHICLE(v: Py) { ensure_init(); G_VEHICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_WATER() -> Py { ensure_init(); G_WATER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WATER(v: Py) { ensure_init(); G_WATER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A thing named as the trouble - \"its the electric thats the problem\", \"the problem is the car\".\n\nFive of seven wordings of this shape reached the intake form on main: \"I'm listening - say more if\nit helps\", \"Thanks for telling me - I'm here if you want to dig into any of that\", \"Got it. Is\nthere anything you'd like a hand with?\". A person who names the trouble has handed over the\nsubject of the next few turns; a nod hands it back to them and asks for nothing.\n\nWhat a friend does with it is say the two ways that thing usually goes wrong and ask which one it\nis, because from there the next step is obvious and it is a different next step for each: a bill\nis a payment plan, and a supply that has gone is the supplier's. That is one short question about\nthe thing they named, not a shrug and not a list - the shape STYLE.md shows for \"this spreadsheet\nis killing me\" (\"What's it fighting you on, formulas or formatting?\").\n\nOnly the things where two branches really are the whole of it are here: a supply, the heating,\nwater, a vehicle, a connection, a bill. A body part named as the trouble (\"its my back thats the\nproblem\") is a health line with its own cards, a pet is not a thing that goes wrong in twos, and\nanything else returns None and keeps the reply it had.\n\nAnd the question is only ever better than a *shrug*. The composer is asked what it would have said\nfirst, and unless that is a receipt, a neutral line or a hedge, this stands down: \"the problem is\nthe council\" is a Citizens Advice pointer and \"its the boiler thats the problem\" is a plumber, and\nboth keep their turn. The same trade `where_it_answers` makes, for the same reason.\n");
    s_A_THE_TROUBLE(stdlib_call("re", "compile", &[py_str("^(?:so\\s+|well\\s+|and\\s+|but\\s+|ok\\s+|okay\\s+|yeah\\s+|yeh\\s+|no\\s+|also\\s+|honestly\\s+)*(?:(?:it'?s|its|it\\s+is)\\s+(?:the|my|our|his|her|their|this|that|me)?\\s*(?P<thing>[a-z][a-z' -]{2,40}?)\\s+(?:that'?s|thats|what'?s|which\\s+is|is)\\s+(?:the\\s+|my\\s+|our\\s+)?(?:problem|trouble|issue|worry|wrong|bit\\s+that'?s\\s+wrong)|(?:the|my|our)\\s+(?:problem|trouble|issue)\\s+(?:is|was)\\s+(?:the|my|our|his|her|their)?\\s*(?P<thing2>[a-z][a-z' -]{2,40}))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BROKEN_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:broken|broke|not\\s+working|stopped\\s+working|packed\\s+in|packed\\s+up|dead|leaking|flooding|blown|on\\s+fire|smoking|sparking|no\\s+power|no\\s+hot\\s+water|not\\s+coming\\s+on|not\\s+starting|wont\\s+start)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_ASK(stdlib_call("re", "compile", &[py_str("\\?|\\b(?:can|could|should|how|what|when|why|which|who|shall)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FILLER(py_set(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("his"), py_str("her"), py_str("their"), py_str("this"), py_str("that"), py_str("me"), py_str("it"), py_str("and"), py_str("but"), py_str("so"), py_str("for"), py_str("of"), py_str("with"), py_str("on"), py_str("in"), py_str("at"), py_str("to")]));
    s_SUPPLY(py_tuple(vec![py_str("electric"), py_str("electricity"), py_str("electrics"), py_str("power"), py_str("meter"), py_str("fuse"), py_str("fuses"), py_str("gas")]));
    s_HEATING(py_tuple(vec![py_str("boiler"), py_str("heating"), py_str("radiator"), py_str("radiators"), py_str("hot"), py_str("immersion"), py_str("thermostat")]));
    s_WATER(py_tuple(vec![py_str("water"), py_str("leak"), py_str("leaking"), py_str("drain"), py_str("drains"), py_str("tap"), py_str("taps"), py_str("sink"), py_str("toilet"), py_str("plumbing"), py_str("pipe"), py_str("pipes"), py_str("shower"), py_str("bath")]));
    s_VEHICLE(py_tuple(vec![py_str("car"), py_str("van"), py_str("bike"), py_str("motorbike"), py_str("scooter"), py_str("engine"), py_str("tyre"), py_str("tyres"), py_str("clutch"), py_str("mot"), py_str("gearbox"), py_str("battery"), py_str("brakes")]));
    s_CONNECTION(py_tuple(vec![py_str("wifi"), py_str("internet"), py_str("broadband"), py_str("signal"), py_str("mobile"), py_str("phone"), py_str("router"), py_str("connection"), py_str("landline"), py_str("data")]));
    s_BILL(py_tuple(vec![py_str("rent"), py_str("mortgage"), py_str("council"), py_str("tax"), py_str("bill"), py_str("bills"), py_str("energy"), py_str("debt"), py_str("loan"), py_str("money"), py_str("fees"), py_str("fee"), py_str("service"), py_str("charge"), py_str("charges")]));
    s_SAY(py_dict(vec![(py_str("supply"), py_tuple(vec![py_str("A supply goes wrong in one of two ways - a bill that has got away from you, or the supply itself - and the first step is different for each. Which is it for you?"), py_str("With the electric or the gas it is nearly always the bill or the supply, and one is a phone call to the supplier while the other is a payment plan. Which is it?")])), (py_str("heating"), py_tuple(vec![py_str("Heating trouble is usually either a bill you can't cover or the boiler not doing its job, and the first step is different for each. Which is it for you?"), py_str("The heating has two ways of becoming the problem - what it costs, or it not coming on - and the fix is different for each. Which one is it?")])), (py_str("water"), py_tuple(vec![py_str("Water in the wrong place is a today job and water that has stopped is the supplier's; the first move is different for each. Which is it for you?"), py_str("Two different problems wear the same word: a leak, which is a today job, and a supply that has gone, which is the water company's. Which one is yours?")])), (py_str("vehicle"), py_tuple(vec![py_str("A vehicle goes wrong in two ways that matter - something is not right with it, or what putting it right costs - and the first step is different for each. Which is it?"), py_str("With a car it is usually the thing itself or the bill to fix it, and only one of those is the garage's. Which one is the trouble?")])), (py_str("connection"), py_tuple(vec![py_str("Broadband goes wrong in two ways - the connection dropping, or the bill - and the first check is different for each. Which is it?"), py_str("A connection problem is either the line or the account, and the supplier's own status page answers the first before you ring anybody. Which of the two is it?")])), (py_str("bill"), py_tuple(vec![py_str("A bill becomes the problem in one of two ways - too big for the month, or wrong on the paper - and the first step is different for each. Which is it?"), py_str("It is usually either the amount or the paper, and a bill that is wrong is worth a call before a payment plan. Which one is it for you?")]))]));
}
pub fn _has(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    return call_builtin("bool", &[op_bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), text.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _family(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let _ = py_str("The two-way family the named thing belongs to, or None.");
    if py_truthy(&_has(&[thing.clone(), g_VEHICLE()])) {
    return py_str("vehicle");
    }
    if py_truthy(&_has(&[thing.clone(), g_CONNECTION()])) {
    return py_str("connection");
    }
    if py_truthy(&_has(&[thing.clone(), g_BILL()])) {
    return py_str("bill");
    }
    if py_truthy(&_has(&[thing.clone(), g_SUPPLY()])) {
    return py_str("supply");
    }
    if py_truthy(&_has(&[thing.clone(), g_HEATING()])) {
    return py_str("heating");
    }
    if py_truthy(&_has(&[thing.clone(), g_WATER()])) {
    return py_str("water");
    }
    return Py::None;
    Py::None
}
pub fn _the_wording(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let mut wordings: Py = Py::None;
    let __try1 = exc::catch(|| {
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["AttributeError"]) {
                __handled5 = true;
    turns = py_tuple(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    wordings = geti(&g_SAY(), &family.clone());
    return geti(&wordings.clone(), &op_modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wordings.clone()]).unwrap_or_else(|e| exc::raise(e))));
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
    else { Py::None };
    let mut clean: Py = Py::None;
    let mut family: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A question about the thing they named as the trouble, or None when it is another line.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    match_ = mcall(&g_A_THE_TROUBLE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    thing = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("thing")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("thing2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_ASK(), "search", &[clean.clone()])) {
    return Py::None;
    }
    rest = op_add(&op_add(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None));
    if py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), thing.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_FILLER(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    family = _family(&[thing.clone()]);
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_BROKEN_NOW(), "search", &[clean.clone()])) {
    return Py::None;
    }
    said = py_call(&crate::compose::fallback::g_reply_for(), &[text.clone(), if py_truthy(&Py::Bool(!py_is(&memory.clone(), &Py::None))) { memory.clone() } else { py_dict(vec![]) }]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = crate::compose::fallback::_nothing_to_stand_behind(&[said.clone()]); if py_truthy(&__b) { __b } else { crate::compose::fallback::_is_a_hedge(&[said.clone()]) } })) } }) {
    return Py::None;
    }
    return _the_wording(&[if py_truthy(&Py::Bool(!py_is(&memory.clone(), &Py::None))) { memory.clone() } else { py_dict(vec![]) }, family.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.the_trouble_named".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "A_ASK" => Some(g_A_ASK()),
            "A_BROKEN_NOW" => Some(g_A_BROKEN_NOW()),
            "A_THE_TROUBLE" => Some(g_A_THE_TROUBLE()),
            "BILL" => Some(g_BILL()),
            "CONNECTION" => Some(g_CONNECTION()),
            "FILLER" => Some(g_FILLER()),
            "HEATING" => Some(g_HEATING()),
            "SAY" => Some(g_SAY()),
            "SUPPLY" => Some(g_SUPPLY()),
            "VEHICLE" => Some(g_VEHICLE()),
            "WATER" => Some(g_WATER()),
            "_family" => Some(py_func("_family", |args: &[Py]| _family(args))),
            "_has" => Some(py_func("_has", |args: &[Py]| _has(args))),
            "_the_wording" => Some(py_func("_the_wording", |args: &[Py]| _the_wording(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "A_ASK" => s_A_ASK(v),
            "A_BROKEN_NOW" => s_A_BROKEN_NOW(v),
            "A_THE_TROUBLE" => s_A_THE_TROUBLE(v),
            "BILL" => s_BILL(v),
            "CONNECTION" => s_CONNECTION(v),
            "FILLER" => s_FILLER(v),
            "HEATING" => s_HEATING(v),
            "SAY" => s_SAY(v),
            "SUPPLY" => s_SUPPLY(v),
            "VEHICLE" => s_VEHICLE(v),
            "WATER" => s_WATER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.the_trouble_named", module_obj);
    pyrt::xrefs::reg_fn("compose.the_trouble_named", "_has", py_func("_has", |args: &[Py]| _has(args)));
    pyrt::xrefs::reg_fn("compose.the_trouble_named", "_family", py_func("_family", |args: &[Py]| _family(args)));
    pyrt::xrefs::reg_fn("compose.the_trouble_named", "_the_wording", py_func("_the_wording", |args: &[Py]| _the_wording(args)));
    pyrt::xrefs::reg_fn("compose.the_trouble_named", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.the_trouble_named", "A_ASK", g_A_ASK, s_A_ASK);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "A_BROKEN_NOW", g_A_BROKEN_NOW, s_A_BROKEN_NOW);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "A_THE_TROUBLE", g_A_THE_TROUBLE, s_A_THE_TROUBLE);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "BILL", g_BILL, s_BILL);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "CONNECTION", g_CONNECTION, s_CONNECTION);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "FILLER", g_FILLER, s_FILLER);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "HEATING", g_HEATING, s_HEATING);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "SAY", g_SAY, s_SAY);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "SUPPLY", g_SUPPLY, s_SUPPLY);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "VEHICLE", g_VEHICLE, s_VEHICLE);
    pyrt::xrefs::reg_access("compose.the_trouble_named", "WATER", g_WATER, s_WATER);
}

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


thread_local! { static G_CAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEARNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOKBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT understanding.cars: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_CAR() -> Py { ensure_init(); G_CAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAR(v: Py) { ensure_init(); G_CAR.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_BREAK() -> Py { ensure_init(); G_CLAUSE_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_BREAK(v: Py) { ensure_init(); G_CLAUSE_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g_FAULT() -> Py { ensure_init(); G_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAULT(v: Py) { ensure_init(); G_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEARNING() -> Py { ensure_init(); G_LEARNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEARNING(v: Py) { ensure_init(); G_LEARNING.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOKBACK() -> Py { ensure_init(); G_LOOKBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOKBACK(v: Py) { ensure_init(); G_LOOKBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FAULT() -> Py { ensure_init(); G_NOT_A_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FAULT(v: Py) { ensure_init(); G_NOT_A_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_MACHINE() -> Py { ensure_init(); G_NOT_THE_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_MACHINE(v: Py) { ensure_init(); G_NOT_THE_MACHINE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A car with something wrong with it, which is not a question about learning to drive.\n\nThe two things share a word. \"drive\" is the verb of a lesson and the verb of a broken car, so a\nchat about a car that has gone wrong collects the learning-to-drive rows:\n\n    my punto makes a clicking noise when i turn\n    is it safe to drive\n    what will it cost roughly   -> \"Most people need around 45 hours of lessons...\"\n\nThe price of a repair answered with the price of lessons. Nothing in the third line says which\ncar it is: the fault is two lines back, and a bare \"what will it cost roughly\" is the same\nsentence for a clutch and for a test. So the reading is made from the chat, once per turn: a car\nand something wrong with it in the last few lines, and nothing in this line about learning.\n\nA line that does name the learning keeps its own row - \"how much do driving lessons cost\" after a\ncar fault is still the lessons question - so the learning words here take the reading back.\n");
    s_CAR(stdlib_call("re", "compile", &[py_str("\\b(?:car|cars|van|vans|mot|mot'd|garage|garages|mechanic|mechanics|tyre|tyres|tire|tires|engine|gearbox|clutch|brakes|brake|exhaust|radiator|battery|alternator|starter|windscreen|bonnet|suspension|shock|absorber|wheel|wheels|hub|bearing|bearing's|jump\\s?start|service|serviced|motoring|audi|ford|vauxhall|toyota|honda|nissan|renault|peugeot|citroen|skoda|hyundai|kia|mazda|volvo|suzuki|bmw|mercedes|volkswagen|vw|fiat|tesla|dacia|chevrolet|chrysler|dodge|subaru|mitsubishi|lexus|saab|alfa|jeep|bentley|porsche|ferrari|lamborghini|aston|lada|daewoo|infiniti|acura|punto|corsa|clio|astra|mondeo|passat|micra|yaris|fabia|octavia|ibiza|qashqai|juke|captur|kadjar|sandero|duster|sprinter|picanto|zafira|insignia|vectra|laguna|megane|berlingo|picasso|cupra|leon|toledo|kamiq|karoq|kodiaq|tucson|sportage|camaro|impreza|forester|hilux|auris|avensis|prius|celica|supra|aygo|navara)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:noise|noises|noisy|rattle|rattles|rattling|click|clicking|clicks|knock|knocking|grinding|squeal|squealing|squeak|squeaking|whining|humming|judder|juddering|wont start|won't start|wouldnt start|wouldn't start|not starting|cutting out|cuts out|stalls|stalled|warning light|engine light|light on|leak|leaks|leaking|leaked|smoke|smoking|overheating|overheated|misfire|misfiring|broken|broke|faulty|gone wrong|failed|failing|fail|puncture|punctured|flat|bald|locked up|seized|clunk|clunking|vibration|vibrating|pulling|sticking|jammed|rust|rusted|corroded|seized up|garage (?:says|said|thinks|reckons|quoted|quotes|wants|has had)|they quoted|needs? (?:a )?(?:new|replacing)|worn|knackered|shot)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEARNING(stdlib_call("re", "compile", &[py_str("\\b(?:lesson|lessons|learner|learners|instructor|instructors|provisional|theory|driving test|test booked|passed|pass|learn|learning|beginners?|tuition)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LOOKBACK(Py::Int(3));
    s_CLAUSE_BREAK(stdlib_call("re", "compile", &[py_str("\\b(?:but|and|though|although|while|whereas|so|because|then|except)\\b|[;,.!?]")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:not|no|never|isnt|isn't|arent|wasnt|wasn't|hasnt|hasn't|havent|haven't|dont|don't|cant|can't|fine|ok|okay|sorted|fixed|repaired|stopped|gone|better)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_MACHINE(stdlib_call("re", "compile", &[py_str("\\b(?:bike|cycle|bicycle|motorbike|motorcycle|scooter|mower|lawnmower|tractor|lorry|bus|train|plane|caravan|trailer)s?\\b")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_fault_that_is_not_undone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut after: Py = Py::None;
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("True when the clause says something is wrong, with no word taking it back.");
    let __iter1 = py_iter_or_raise(&mcall(&g_FAULT(), "finditer", &[clause.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "lower", &[]), "startswith", &[py_tuple(vec![py_str("not "), py_str("wont"), py_str("won't"), py_str("wouldnt"), py_str("wouldn't")])])) {
    return Py::Bool(true);
    }
    before = slice_or_raise(&clause.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(24))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "start", &[]), &Py::None);
    after = slice_or_raise(&clause.clone(), &mcall(&match_.clone(), "end", &[]), &op_add(&mcall(&match_.clone(), "end", &[]), &Py::Int(24)), &Py::None);
    if py_truthy(&{ let __b = mcall(&g_NOT_A_FAULT(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_FAULT(), "search", &[after.clone()]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_car_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut before: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("True when the clause names the machine itself, and not a bike's part or a bus.");
    let __iter1 = py_iter_or_raise(&mcall(&g_CAR(), "finditer", &[clause.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    before = slice_or_raise(&clause.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(20))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&mcall(&g_NOT_THE_MACHINE(), "search", &[before.clone()])) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_a_broken_car(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when one clause of the line names the car and says something is wrong with it.");
    low = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE_BREAK(), "split", &[low.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&{ let __b = _the_car_in(&[clause.clone()]); if !py_truthy(&__b) { __b } else { _a_fault_that_is_not_undone(&[clause.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn a_broken_car_is_the_subject(args: &[Py]) -> Py {
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
    let mut earlier: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "earlier") { v.clone() }
    else { py_tuple(vec![]) };
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when a car with something wrong with it is what this turn is about.\n\n    `earlier` is the person's own earlier lines, oldest first. The fault may be in this line or\n    in the last few, because a bare price question names none of it, and the learning words in\n    this line take the reading back: a lesson question is not a repair, however broken the car in\n    the chat is.\n    ");
    low = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&mcall(&g_LEARNING(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_says_a_broken_car(&[low.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[earlier.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&g_LOOKBACK()), &Py::None, &Py::None)) { let mut line: Py = __item.clone(); __out.push(_says_a_broken_car(&[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn no_broken_car(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("A guard for a row about learning or hiring a car: False when a broken one is the subject.\n\n    The row that answers \"how much do driving lessons cost\" answered a bare \"what will it cost\n    roughly\" two lines after a clicking noise, because the subject of the chat was the word\n    \"drive\". A row about lessons stands aside for the repair, and the line itself decides (see\n    `a_broken_car_is_the_subject`): \"how much do driving lessons cost\" is still the lessons.\n    ");
    return Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("car_fault")]) } }));
    Py::None
}
pub fn no_broken_car_or_none(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("A guard for a row that answers because the chat says a car is broken: `car_fault` or no.");
    return call_builtin("bool", &[{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("car_fault")]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.understanding.cars".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "CAR" => Some(g_CAR()),
            "CLAUSE_BREAK" => Some(g_CLAUSE_BREAK()),
            "FAULT" => Some(g_FAULT()),
            "LEARNING" => Some(g_LEARNING()),
            "LOOKBACK" => Some(g_LOOKBACK()),
            "NOT_A_FAULT" => Some(g_NOT_A_FAULT()),
            "NOT_THE_MACHINE" => Some(g_NOT_THE_MACHINE()),
            "_a_fault_that_is_not_undone" => Some(py_func("_a_fault_that_is_not_undone", |args: &[Py]| _a_fault_that_is_not_undone(args))),
            "_says_a_broken_car" => Some(py_func("_says_a_broken_car", |args: &[Py]| _says_a_broken_car(args))),
            "_the_car_in" => Some(py_func("_the_car_in", |args: &[Py]| _the_car_in(args))),
            "a_broken_car_is_the_subject" => Some(py_func("a_broken_car_is_the_subject", |args: &[Py]| a_broken_car_is_the_subject(args))),
            "no_broken_car" => Some(py_func("no_broken_car", |args: &[Py]| no_broken_car(args))),
            "no_broken_car_or_none" => Some(py_func("no_broken_car_or_none", |args: &[Py]| no_broken_car_or_none(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "CAR" => s_CAR(v),
            "CLAUSE_BREAK" => s_CLAUSE_BREAK(v),
            "FAULT" => s_FAULT(v),
            "LEARNING" => s_LEARNING(v),
            "LOOKBACK" => s_LOOKBACK(v),
            "NOT_A_FAULT" => s_NOT_A_FAULT(v),
            "NOT_THE_MACHINE" => s_NOT_THE_MACHINE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("understanding.cars", module_obj);
    pyrt::xrefs::reg_fn("understanding.cars", "_a_fault_that_is_not_undone", py_func("_a_fault_that_is_not_undone", |args: &[Py]| _a_fault_that_is_not_undone(args)));
    pyrt::xrefs::reg_fn("understanding.cars", "_the_car_in", py_func("_the_car_in", |args: &[Py]| _the_car_in(args)));
    pyrt::xrefs::reg_fn("understanding.cars", "_says_a_broken_car", py_func("_says_a_broken_car", |args: &[Py]| _says_a_broken_car(args)));
    pyrt::xrefs::reg_fn("understanding.cars", "a_broken_car_is_the_subject", py_func("a_broken_car_is_the_subject", |args: &[Py]| a_broken_car_is_the_subject(args)));
    pyrt::xrefs::reg_fn("understanding.cars", "no_broken_car", py_func("no_broken_car", |args: &[Py]| no_broken_car(args)));
    pyrt::xrefs::reg_fn("understanding.cars", "no_broken_car_or_none", py_func("no_broken_car_or_none", |args: &[Py]| no_broken_car_or_none(args)));
    pyrt::xrefs::reg_access("understanding.cars", "CAR", g_CAR, s_CAR);
    pyrt::xrefs::reg_access("understanding.cars", "CLAUSE_BREAK", g_CLAUSE_BREAK, s_CLAUSE_BREAK);
    pyrt::xrefs::reg_access("understanding.cars", "FAULT", g_FAULT, s_FAULT);
    pyrt::xrefs::reg_access("understanding.cars", "LEARNING", g_LEARNING, s_LEARNING);
    pyrt::xrefs::reg_access("understanding.cars", "LOOKBACK", g_LOOKBACK, s_LOOKBACK);
    pyrt::xrefs::reg_access("understanding.cars", "NOT_A_FAULT", g_NOT_A_FAULT, s_NOT_A_FAULT);
    pyrt::xrefs::reg_access("understanding.cars", "NOT_THE_MACHINE", g_NOT_THE_MACHINE, s_NOT_THE_MACHINE);
}

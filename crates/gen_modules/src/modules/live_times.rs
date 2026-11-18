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


thread_local! { static G_ADVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOARD_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DELAYED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FERRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLIGHT_LIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_OPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLATFORM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRIORITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SERVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_BOARD_THAT_KEEPS_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEHICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VEHICLE_STEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ADVICE() -> Py { ensure_init(); G_ADVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVICE(v: Py) { ensure_init(); G_ADVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BOARD_ASK() -> Py { ensure_init(); G_BOARD_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOARD_ASK(v: Py) { ensure_init(); G_BOARD_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BUS() -> Py { ensure_init(); G_BUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUS(v: Py) { ensure_init(); G_BUS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOCK() -> Py { ensure_init(); G_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOCK(v: Py) { ensure_init(); G_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_DELAYED() -> Py { ensure_init(); G_DELAYED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DELAYED(v: Py) { ensure_init(); G_DELAYED.with(|c| *c.borrow_mut() = v); }
pub fn g_FERRY() -> Py { ensure_init(); G_FERRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FERRY(v: Py) { ensure_init(); G_FERRY.with(|c| *c.borrow_mut() = v); }
pub fn g_FLIGHT() -> Py { ensure_init(); G_FLIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLIGHT(v: Py) { ensure_init(); G_FLIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_FLIGHT_LIVE() -> Py { ensure_init(); G_FLIGHT_LIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLIGHT_LIVE(v: Py) { ensure_init(); G_FLIGHT_LIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY() -> Py { ensure_init(); G_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY(v: Py) { ensure_init(); G_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS() -> Py { ensure_init(); G_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS(v: Py) { ensure_init(); G_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_OPENING() -> Py { ensure_init(); G_PLACE_OPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_OPENING(v: Py) { ensure_init(); G_PLACE_OPENING.with(|c| *c.borrow_mut() = v); }
pub fn g_PLATFORM() -> Py { ensure_init(); G_PLATFORM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLATFORM(v: Py) { ensure_init(); G_PLATFORM.with(|c| *c.borrow_mut() = v); }
pub fn g_PRIORITY() -> Py { ensure_init(); G_PRIORITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRIORITY(v: Py) { ensure_init(); G_PRIORITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RAIL() -> Py { ensure_init(); G_RAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAIL(v: Py) { ensure_init(); G_RAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SERVICE() -> Py { ensure_init(); G_SERVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SERVICE(v: Py) { ensure_init(); G_SERVICE.with(|c| *c.borrow_mut() = v); }
pub fn g_STATUS() -> Py { ensure_init(); G_STATUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATUS(v: Py) { ensure_init(); G_STATUS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_BOARD_THAT_KEEPS_THEM() -> Py { ensure_init(); G_THE_BOARD_THAT_KEEPS_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_BOARD_THAT_KEEPS_THEM(v: Py) { ensure_init(); G_THE_BOARD_THAT_KEEPS_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME() -> Py { ensure_init(); G_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME(v: Py) { ensure_init(); G_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAIN() -> Py { ensure_init(); G_TRAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAIN(v: Py) { ensure_init(); G_TRAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_VEHICLE() -> Py { ensure_init(); G_VEHICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEHICLE(v: Py) { ensure_init(); G_VEHICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_VEHICLE_STEM() -> Py { ensure_init(); G_VEHICLE_STEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VEHICLE_STEM(v: Py) { ensure_init(); G_VEHICLE_STEM.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_ASK() -> Py { ensure_init(); G_WHEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASK(v: Py) { ensure_init(); G_WHEN_ASK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A bus or train time is live, and the bot should say where the next one is.\n\n\"When is the next bus into town\" and \"has the 2 oclock train been cancelled\" are not plans the\nperson told us about: the answer is a departure board, which the bot cannot see. The memory store\nread the first as a thing coming up and said \"Nothing so far - you haven't mentioned anything\ncoming up\", and the knowledge base read the second as a delay-repay claim. Both are the wrong\nsubject for a question about a live service.\n\nSo this module answers only a line that names a scheduled service and asks for its time or\nstate, and it says the two things a friend can: I cannot see it from here, and the operator's own\napp or the board at the stop has it. A line with \"my\" or \"our\" in it is their own plan and stays\nwith the memory store, and a line about the price or the worth of a ticket is not a time.\n");
    s_PRIORITY(Py::Int(7001));
    s_ANSWERS(py_tuple(vec![py_str("geography-and-travel")]));
    s_VEHICLE(stdlib_call("re", "compile", &[py_str("\\b(?:bus|buses|train|trains|tube|tram|trams|coach|coaches|ferry|ferries|flight|flights)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:when|what\\s+time|how\\s+soon|next|on\\s+time|delayed|cancelled|canceled|running|due|leaves?|departs?|comes?|arrives?|platform|replacement|engineering\\s+works?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STATUS(stdlib_call("re", "compile", &[py_str("\\b(?:on\\s+time|delayed|cancelled|canceled|running|due|replacement|engineering\\s+works?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLATFORM(stdlib_call("re", "compile", &[py_str("\\b(?:platform|station|stop|interchange|departure\\s+board)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_OPENING(stdlib_call("re", "compile", &[py_str("\\b(?:train|bus|coach|tram|tube)\\s+(?:station|stop)\\b[^?.]{0,25}\\b(?:open|opens|opening|close|closes|closed|shut|shuts|hours)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MONEY(stdlib_call("re", "compile", &[py_str("\\b(?:worth|cost|costs|price|prices|fare|fares|cheap|cheaper|expensive|refund|refunds|compensation|money|claim|claims|insurance|voucher|rebook|rebooking|delay\\s+repay|pass|rights?|entitled|entitlement|owed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVICE(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+if|should\\s+(?:i|we)|what\\s+time\\s+should|how\\s+(?:do|can|should)\\s+(?:i|we)|how\\s+early|how\\s+long\\s+before|what\\s+happens\\s+if|do\\s+(?:i|we)\\s+need)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DELAYED(stdlib_call("re", "compile", &[py_str("\\b(?:delayed|delays?|running|on\\s+time|disruption|disrupted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BOARD_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:next|last|first|when|what\\s+time|how\\s+soon|platform|departure\\s+board|station|stop|leaves?|departs?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FLIGHT_LIVE(stdlib_call("re", "compile", &[py_str("\\b(?:next|last|first|running|on\\s+time|delayed|cancelled|canceled|platform|leaves?|departs?|replacement|engineering\\s+works?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK(stdlib_call("re", "compile", &[py_str("^(?:when|what|how|is|are|was|were|has|have|did|does|do|can|could|will|would|any|next|which)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_ASK(stdlib_call("re", "compile", &[py_str("^\\s*when\\s+(?:is|are|was|were|does|do|did|will|would|should|can|could)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEWS(stdlib_call("re", "compile", &[py_str("\\b(?:strike|strikes|striking|walkout|walk\\s*out|industrial\\s+action|union|ballot|announcement|news)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SERVICE(stdlib_call("re", "compile", &[py_str("\\b(?:(?P<clock>[0-9]{1,2}(?::[0-9]{2})?\\s*(?:o'?clock)?)\\s+)?(?:(?P<qual>next|night|last|first|early|late|morning|evening|midnight)\\s+)?(?P<vehicle>bus|train|tube|tram|coach|ferry|flight)\\b(?:\\s+(?P<via>to|into|from)\\s+(?P<dest>[a-z][a-z'-]*(?:\\s+[a-z][a-z'-]*){0,4}))?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOCK(stdlib_call("re", "compile", &[py_str("\\b\\d{1,2}[:.]\\d{2}\\s*(?:am|pm)?\\b|\\b\\d{1,2}\\s*(?:am|pm|o'?clock)\\b|\\b(?:at|by|around|about|before|after|from|until|till)\\s+\\d{1,2}\\b|\\b\\d{1,2}\\s+(?:tomorrow|today|tonight)\\b|\\b(?:this\\s+)?(?:morning|afternoon|evening|noon|midnight)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VEHICLE_STEM(py_dict(vec![(py_str("buses"), py_str("bus")), (py_str("trains"), py_str("train")), (py_str("trams"), py_str("tram")), (py_str("coaches"), py_str("coach")), (py_str("ferries"), py_str("ferry")), (py_str("flights"), py_str("flight"))]));
    s_BUS(py_tuple(vec![py_str("I can't see live bus times from here. The operator's app, or the timetable at the stop, has the next one; where the stop has a code, a text to it brings the times back. A journey planner puts the whole trip together."), py_str("The next one is live, so nothing here can see it. The bus operator's app, or the timetable printed at the stop, has the time - text the stop's code where it has one - and a journey planner joins the trip up.")]));
    s_THE_BOARD_THAT_KEEPS_THEM(py_dict(vec![(py_str("train"), py_str("the board at the station")), (py_str("tube"), py_str("the board on the platform")), (py_str("tram"), py_str("the board at the stop")), (py_str("coach"), py_str("the board at the coach station"))]));
    s_RAIL(py_tuple(vec![py_str("I can't see live {vehicle} times from here. The operator's app, or {where}, has the next one and any delay or cancellation; a journey planner puts the whole trip together."), py_str("That one is a live departure, so the operator's app, or {where}, is where the next one and any delay or cancellation show up. A journey planner puts the whole trip together.")]));
    s_TRAIN(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RAIL()) { let mut text: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("vehicle".to_string(), py_str("train")), ("where".to_string(), py_str("the board at the station"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_FLIGHT(py_tuple(vec![py_str("I can't see live flights from here. The airline's app, or the airport's own departures board, has the gate and any delay; the booking reference brings it up."), py_str("The gate and any delay are live, and I can't see them from here. The airline's app, or the airport's own departures board, has them, and the booking reference brings the booking up.")]));
    s_FERRY(py_tuple(vec![py_str("I can't see live ferry times from here. The operator's app, or the port's own departure board, has the next sailing and any delay; a journey planner puts the whole crossing together."), py_str("That sailing is a live one, and I can't see it from here. The ferry operator's app, or the board at the port, has the next one and any delay; a journey planner joins the crossing up.")]));
}
pub fn _service_named(args: &[Py]) -> Py {
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
    let mut dest: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut part: Py = Py::None;
    let mut words: Py = Py::None;
    match_ = mcall(&g_SERVICE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![mcall(&match_.clone(), "group", &[py_str("clock")]), mcall(&match_.clone(), "group", &[py_str("qual")]), mcall(&match_.clone(), "group", &[py_str("vehicle")])])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() };
    name = mcall(&py_str(" "), "join", &[words.clone()]);
    if py_truthy(&mcall(&match_.clone(), "group", &[py_str("dest")])) {
    dest = geti(&stdlib_call("re", "split", &[py_str("\\b(?:cancelled|canceled|delayed|running|tomorrow|today|tonight|please|leave|leaves|go|goes|departs?|arrives?)\\b"), mcall(&match_.clone(), "group", &[py_str("dest")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    dest = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\s*\\b(?:of|the|and|a|an|to|from|in|on|at|for|by)\\b)+$")); __args.push(py_str("")); __args.push(mcall(&dest.clone(), "strip", &[])); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", &[]);
    if py_truthy(&dest.clone()) {
    let __aug1 = op_add(&name.clone(), &op_add(&op_add(&op_add(&py_str(" "), &mcall(&match_.clone(), "group", &[py_str("via")])), &py_str(" ")), &dest.clone()));
    name = __aug1;
    }
    }
    return name.clone();
    Py::None
}
pub fn _their_own_time(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:next|last|first|night|early|late|platform|station|stop|running|on\\s+time|delayed|cancelled|canceled|replacement)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    found = mcall(&g_VEHICLE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    word = mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    stem = mcall(&g_VEHICLE_STEM(), "get", &[word.clone(), word.clone()]);
    let __try1 = exc::catch(|| {
    turns = call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_CLOCK(), "search", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _clean(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
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
    let mut clean: Py = Py::None;
    let mut flight: Py = Py::None;
    let mut found: Py = Py::None;
    let mut found_plural: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut named: Py = Py::None;
    let mut service: Py = Py::None;
    let mut vehicle: Py = Py::None;
    clean = _clean(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_VEHICLE(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TIME(), "search", &[clean.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MONEY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NEWS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PLACE_OPENING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ADVICE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { _their_own_time(&[clean.clone(), memory.clone()]) } } } } } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_DELAYED(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_BOARD_ASK(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    flight = { let __b = stdlib_call("re", "search", &[py_str("\\b(?:flight|flights|plane)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "search", &[py_str("\\bairport\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_VEHICLE(), "search", &[clean.clone()]))) } } } };
    if py_truthy(&{ let __b = flight.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_FLIGHT_LIVE(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ASK(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "endswith", &[py_str("?")]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bnext\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_STATUS(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_PLATFORM(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bwhat\\s+time\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_WHEN_ASK(), "search", &[clean.clone()]))) } } } } } } } }) {
    return Py::None;
    }
    service = _service_named(&[clean.clone()]);
    lead = if py_truthy(&service.clone()) { op_add(&op_add(&py_str("For the "), &service.clone()), &py_str(": ")) } else { py_str("") };
    found = mcall(&g_SERVICE(), "search", &[clean.clone()]);
    vehicle = if py_truthy(&found.clone()) { mcall(&mcall(&found.clone(), "group", &[py_str("vehicle")]), "lower", &[]) } else { py_str("") };
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:replacement|engineering\\s+works?)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A replacement is the operator's own announcement: their app or site says where it leaves from and when, and the station's board carries it on the day.");
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&vehicle.clone(), &py_str("flight"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&vehicle.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:flight|flights|plane|airport)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return op_add(&lead.clone(), &_the_wording(&[memory.clone(), g_FLIGHT()]));
    }
    if py_truthy(&Py::Bool(py_eq(&vehicle.clone(), &py_str("ferry")))) {
    return op_add(&lead.clone(), &_the_wording(&[memory.clone(), g_FERRY()]));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("train"), py_str("tube"), py_str("tram"), py_str("coach")]), &vehicle.clone())); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:trains|trams|coaches)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    named = mcall(&g_VEHICLE_STEM(), "get", &[vehicle.clone(), vehicle.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    found_plural = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_VEHICLE_STEM()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    named = if py_truthy(&found_plural.clone()) { mcall(&g_VEHICLE_STEM(), "get", &[mcall(&found_plural.clone(), "group", &[Py::Int(0)]), mcall(&found_plural.clone(), "group", &[Py::Int(0)])]) } else { py_str("train") };
    }
    return op_add(&lead.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("vehicle".to_string(), named.clone()), ("where".to_string(), mcall(&g_THE_BOARD_THAT_KEEPS_THEM(), "get", &[named.clone(), py_str("the board at the station")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_the_wording(&[memory.clone(), g_RAIL()]), "format", args))(&__args) });
    }
    return op_add(&lead.clone(), &_the_wording(&[memory.clone(), g_BUS()]));
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
    let mut wordings: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wordings") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: wordings"))) };
    let mut turns: Py = Py::None;
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
    return geti(&wordings.clone(), &op_modulo(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[wordings.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    let _ = py_str("This module read the line whole as a live service, so it takes it as it comes.\n\n    The travel cards are specialists on the same topic and the arbiter lets the specialist win;\n    \"has the 2 oclock train been cancelled\" was answered with the Delay Repay claim. The hook is\n    only true on a line the module already answers.\n    ");
    return Py::Bool(!py_is(&respond(&[text.clone(), memory.clone()]), &Py::None));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.live_times".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADVICE" => Some(g_ADVICE()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK" => Some(g_ASK()),
            "BOARD_ASK" => Some(g_BOARD_ASK()),
            "BUS" => Some(g_BUS()),
            "CLOCK" => Some(g_CLOCK()),
            "DELAYED" => Some(g_DELAYED()),
            "FERRY" => Some(g_FERRY()),
            "FLIGHT" => Some(g_FLIGHT()),
            "FLIGHT_LIVE" => Some(g_FLIGHT_LIVE()),
            "MONEY" => Some(g_MONEY()),
            "NEWS" => Some(g_NEWS()),
            "PLACE_OPENING" => Some(g_PLACE_OPENING()),
            "PLATFORM" => Some(g_PLATFORM()),
            "PRIORITY" => Some(g_PRIORITY()),
            "RAIL" => Some(g_RAIL()),
            "SERVICE" => Some(g_SERVICE()),
            "STATUS" => Some(g_STATUS()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "THE_BOARD_THAT_KEEPS_THEM" => Some(g_THE_BOARD_THAT_KEEPS_THEM()),
            "TIME" => Some(g_TIME()),
            "TRAIN" => Some(g_TRAIN()),
            "VEHICLE" => Some(g_VEHICLE()),
            "VEHICLE_STEM" => Some(g_VEHICLE_STEM()),
            "WHEN_ASK" => Some(g_WHEN_ASK()),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_service_named" => Some(py_func("_service_named", |args: &[Py]| _service_named(args))),
            "_the_wording" => Some(py_func("_the_wording", |args: &[Py]| _the_wording(args))),
            "_their_own_time" => Some(py_func("_their_own_time", |args: &[Py]| _their_own_time(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADVICE" => s_ADVICE(v),
            "ANSWERS" => s_ANSWERS(v),
            "ASK" => s_ASK(v),
            "BOARD_ASK" => s_BOARD_ASK(v),
            "BUS" => s_BUS(v),
            "CLOCK" => s_CLOCK(v),
            "DELAYED" => s_DELAYED(v),
            "FERRY" => s_FERRY(v),
            "FLIGHT" => s_FLIGHT(v),
            "FLIGHT_LIVE" => s_FLIGHT_LIVE(v),
            "MONEY" => s_MONEY(v),
            "NEWS" => s_NEWS(v),
            "PLACE_OPENING" => s_PLACE_OPENING(v),
            "PLATFORM" => s_PLATFORM(v),
            "PRIORITY" => s_PRIORITY(v),
            "RAIL" => s_RAIL(v),
            "SERVICE" => s_SERVICE(v),
            "STATUS" => s_STATUS(v),
            "THE_BOARD_THAT_KEEPS_THEM" => s_THE_BOARD_THAT_KEEPS_THEM(v),
            "TIME" => s_TIME(v),
            "TRAIN" => s_TRAIN(v),
            "VEHICLE" => s_VEHICLE(v),
            "VEHICLE_STEM" => s_VEHICLE_STEM(v),
            "WHEN_ASK" => s_WHEN_ASK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("modules.live_times", module_obj);
    pyrt::xrefs::reg_fn("modules.live_times", "_service_named", py_func("_service_named", |args: &[Py]| _service_named(args)));
    pyrt::xrefs::reg_fn("modules.live_times", "_their_own_time", py_func("_their_own_time", |args: &[Py]| _their_own_time(args)));
    pyrt::xrefs::reg_fn("modules.live_times", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("modules.live_times", "respond", py_func("respond", |args: &[Py]| respond(args)));
    pyrt::xrefs::reg_fn("modules.live_times", "_the_wording", py_func("_the_wording", |args: &[Py]| _the_wording(args)));
    pyrt::xrefs::reg_fn("modules.live_times", "TAKES_AS_IT_COMES", py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args)));
    pyrt::xrefs::reg_access("modules.live_times", "ADVICE", g_ADVICE, s_ADVICE);
    pyrt::xrefs::reg_access("modules.live_times", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("modules.live_times", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("modules.live_times", "BOARD_ASK", g_BOARD_ASK, s_BOARD_ASK);
    pyrt::xrefs::reg_access("modules.live_times", "BUS", g_BUS, s_BUS);
    pyrt::xrefs::reg_access("modules.live_times", "CLOCK", g_CLOCK, s_CLOCK);
    pyrt::xrefs::reg_access("modules.live_times", "DELAYED", g_DELAYED, s_DELAYED);
    pyrt::xrefs::reg_access("modules.live_times", "FERRY", g_FERRY, s_FERRY);
    pyrt::xrefs::reg_access("modules.live_times", "FLIGHT", g_FLIGHT, s_FLIGHT);
    pyrt::xrefs::reg_access("modules.live_times", "FLIGHT_LIVE", g_FLIGHT_LIVE, s_FLIGHT_LIVE);
    pyrt::xrefs::reg_access("modules.live_times", "MONEY", g_MONEY, s_MONEY);
    pyrt::xrefs::reg_access("modules.live_times", "NEWS", g_NEWS, s_NEWS);
    pyrt::xrefs::reg_access("modules.live_times", "PLACE_OPENING", g_PLACE_OPENING, s_PLACE_OPENING);
    pyrt::xrefs::reg_access("modules.live_times", "PLATFORM", g_PLATFORM, s_PLATFORM);
    pyrt::xrefs::reg_access("modules.live_times", "PRIORITY", g_PRIORITY, s_PRIORITY);
    pyrt::xrefs::reg_access("modules.live_times", "RAIL", g_RAIL, s_RAIL);
    pyrt::xrefs::reg_access("modules.live_times", "SERVICE", g_SERVICE, s_SERVICE);
    pyrt::xrefs::reg_access("modules.live_times", "STATUS", g_STATUS, s_STATUS);
    pyrt::xrefs::reg_access("modules.live_times", "THE_BOARD_THAT_KEEPS_THEM", g_THE_BOARD_THAT_KEEPS_THEM, s_THE_BOARD_THAT_KEEPS_THEM);
    pyrt::xrefs::reg_access("modules.live_times", "TIME", g_TIME, s_TIME);
    pyrt::xrefs::reg_access("modules.live_times", "TRAIN", g_TRAIN, s_TRAIN);
    pyrt::xrefs::reg_access("modules.live_times", "VEHICLE", g_VEHICLE, s_VEHICLE);
    pyrt::xrefs::reg_access("modules.live_times", "VEHICLE_STEM", g_VEHICLE_STEM, s_VEHICLE_STEM);
    pyrt::xrefs::reg_access("modules.live_times", "WHEN_ASK", g_WHEN_ASK, s_WHEN_ASK);
}

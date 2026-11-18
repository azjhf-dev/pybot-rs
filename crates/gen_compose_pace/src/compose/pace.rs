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


thread_local! { static G_KEEP_UP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PORTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTINUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DANGLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_ALLOWED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_MARK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEXT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__REST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STOPPED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TESTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.pace: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_KEEP_UP_ASK() -> Py { ensure_init(); G_KEEP_UP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_UP_ASK(v: Py) { ensure_init(); G_KEEP_UP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PACE_ASK() -> Py { ensure_init(); G_PACE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACE_ASK(v: Py) { ensure_init(); G_PACE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLOWED() -> Py { ensure_init(); G__ALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLOWED(v: Py) { ensure_init(); G__ALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_WORDS() -> Py { ensure_init(); G__ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_WORDS(v: Py) { ensure_init(); G__ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_LENGTH() -> Py { ensure_init(); G__A_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_LENGTH(v: Py) { ensure_init(); G__A_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PORTION() -> Py { ensure_init(); G__A_PORTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PORTION(v: Py) { ensure_init(); G__A_PORTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE_LEAD() -> Py { ensure_init(); G__A_RATE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE_LEAD(v: Py) { ensure_init(); G__A_RATE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTINUES() -> Py { ensure_init(); G__CONTINUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTINUES(v: Py) { ensure_init(); G__CONTINUES.with(|c| *c.borrow_mut() = v); }
pub fn g__DANGLES() -> Py { ensure_init(); G__DANGLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DANGLES(v: Py) { ensure_init(); G__DANGLES.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_ALLOWED() -> Py { ensure_init(); G__KEEP_ALLOWED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_ALLOWED(v: Py) { ensure_init(); G__KEEP_ALLOWED.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_LEADS() -> Py { ensure_init(); G__KEEP_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_LEADS(v: Py) { ensure_init(); G__KEEP_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_MARK() -> Py { ensure_init(); G__KEEP_MARK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_MARK(v: Py) { ensure_init(); G__KEEP_MARK.with(|c| *c.borrow_mut() = v); }
pub fn g__LEADS() -> Py { ensure_init(); G__LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEADS(v: Py) { ensure_init(); G__LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g__NEXT() -> Py { ensure_init(); G__NEXT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEXT(v: Py) { ensure_init(); G__NEXT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TAIL() -> Py { ensure_init(); G__NOT_A_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TAIL(v: Py) { ensure_init(); G__NOT_A_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__PACE() -> Py { ensure_init(); G__PACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PACE(v: Py) { ensure_init(); G__PACE.with(|c| *c.borrow_mut() = v); }
pub fn g__REST() -> Py { ensure_init(); G__REST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__REST(v: Py) { ensure_init(); G__REST.with(|c| *c.borrow_mut() = v); }
pub fn g__STEP() -> Py { ensure_init(); G__STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STEP(v: Py) { ensure_init(); G__STEP.with(|c| *c.borrow_mut() = v); }
pub fn g__STOPPED() -> Py { ensure_init(); G__STOPPED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STOPPED(v: Py) { ensure_init(); G__STOPPED.with(|c| *c.borrow_mut() = v); }
pub fn g__TEST() -> Py { ensure_init(); G__TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TEST(v: Py) { ensure_init(); G__TEST.with(|c| *c.borrow_mut() = v); }
pub fn g__TESTED() -> Py { ensure_init(); G__TESTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TESTED(v: Py) { ensure_init(); G__TESTED.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The pace of the plan the bot just gave: \"how slowly should i build up\", \"how fast should i go\".\n\nThe person has a plan in hand - \"run-walk to start, one minute of easy jogging to two of walking,\ntwenty minutes, three times a week, and add a minute of running a week\" - and asks how fast or how\nslowly to go at it. The rate is already in the answer they were given, so the reply hands it back\nrather than looking the question up or saying it cannot be answered:\n\n    how slowly should i build up   \"Slowly, a step at a time: add a minute of running a week for\n                                    six weeks. The rest of the plan is the check on it: one minute\n                                    of easy jogging to two of walking, three times a week.\"\n\nThe same reading answers the length of it - \"how long should i keep that up\", \"how long do i keep\ngoing\" - from the answer's own mark (\"two or three weeks of doing it the same way\"), because the\nperson is asking about the advice in front of them and not about the world.\n\nOnly a line that names nothing of its own qualifies. \"how fast should i run a 5k\" names the 5k and\nhas an answer in the knowledge base, so it is left to that module; a bare \"how fast should i be\ngoing\" has no subject and is asking about the answer in front of it, which is exactly what this\nreads. And a line only gets this when the last answer really does say a pace: with no rate in it\nthere is nothing to hand back, and the composer answers as it did before.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace.PACE_ASK"); }
    s_PACE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:slowly|fast|quickly|soon)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace.KEEP_UP_ASK"); }
    s_KEEP_UP_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long (?:should|shall|do|can|could|would) (?:i|we) (?:keep|carry on|go on|stick|stay|continue|persist)\\b|\\bhow long (?:should|shall|do|can|could|would) (?:i|we) keep \\w+ing\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._A_LENGTH"); }
    s__A_LENGTH(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|eighteen|twenty|thirty|forty|fifty|sixty|a few|a couple of|several|a)\\s+(?:(?:or|to)\\s+(?:\\d+|two|three|four|five|six|seven|eight|ten|twelve|fifteen|twenty)\\s+)?(?:weeks?|months?|days?|hours?|minutes?|fortnights?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._A_RATE_LEAD"); }
    s__A_RATE_LEAD(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|twelve|a|an|once|twice|half)\\s+(?:runs?|times|sessions?|days?|weeks?|months?|hours?|minutes?|miles?|km|kms|kilometres?|reps?|sets?|walks?|workouts?|feeds?|charges?|meals?|nights?)\\s+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._ALLOWED"); }
    s__ALLOWED(call_builtin("frozenset", &[mcall(&py_str("\na an the this that these it i you we me my our\nshould shall can could would will do does did be is was are am\ngo going do doing now then again first next too so much more\nadd build start push increase work step up at on in to of for\nrun runs running walk walks walking jog jogging going train training slower faster easy harder\nslow fast\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._KEEP_ALLOWED"); }
    s__KEEP_ALLOWED(call_builtin("frozenset", &[mcall(&py_str("\nlooking searching waiting trying watching going at it tonight today now longer long for doing\nthis that the same way\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._STEP"); }
    s__STEP(stdlib_call("re", "compile", &[py_str("\\b(?:add|increase|build(?:ing)? up|work(?:ing)? up|step(?:ping)? up|extend|progress)\\b[^.;]{0,70}\\b(?:a|an|each|every|per)\\s+(?:week|day|session|run|month|fortnight)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._PACE"); }
    s__PACE(stdlib_call("re", "compile", &[py_str("\\b(?:add|increase|build(?:ing)? up|work(?:ing)? up|step(?:ping)? up|extend|progress|no more than|only)\\b[^.;]{0,70}\\b(?:a|an|each|every|per)\\s+(?:week|day|session|run|month|fortnight)\\b|\\b(?:one|two|three|four|five|six|\\d+)\\s+times?\\s+a\\s+(?:week|day)\\b|\\bevery other day\\b|\\bweek by week\\b|\\ba (?:minute|day|session|week)\\s+(?:at a time|more|longer)\\b|\\b(?:a|an|one)\\s+\\w+(?:\\s+\\w+)?\\s+at a time\\b|\\ba little at a time\\b|\\bgradually\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._TEST"); }
    s__TEST(stdlib_call("re", "compile", &[py_str("[^.;]{0,60}\\b(?:slow enough to talk|hold a conversation|talk test|conversational|can still talk|able to talk)\\b[^.;]{0,60}"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._LEADS"); }
    s__LEADS(py_dict(vec![(py_str("slowly"), py_tuple(vec![py_str("Slowly, a step at a time"), py_str("Slowly, and only as the plan steps up"), py_str("Slowly - less than you think")])), (py_str("fast"), py_tuple(vec![py_str("No faster than the plan says"), py_str("Only as fast as the answer above says"), py_str("Not fast - by the plan's own step")])), (py_str("quickly"), py_tuple(vec![py_str("Only as quickly as the plan steps up"), py_str("As quickly as the plan says, and no more")])), (py_str("soon"), py_tuple(vec![py_str("No sooner than the answer above says"), py_str("Only when the plan says you are ready")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._REST"); }
    s__REST(py_tuple(vec![py_str("That is the pace the plan sets: {rest}."), py_str("The rest of the plan is the check on it: {rest}."), py_str("The plan above already says the rest of it: {rest}.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._NEXT"); }
    s__NEXT(py_tuple(vec![py_str("The rest of it is as the answer says: {next}."), py_str("Then carry on as the answer says: {next}."), py_str("Nothing else changes after that: {next}.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._TESTED"); }
    s__TESTED(py_tuple(vec![py_str("The test is the plan's own - {test}."), py_str("Judge it by the one the answer already gives - {test}."), py_str("And keep the plan's own check - {test}.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._CONTINUES"); }
    s__CONTINUES(stdlib_call("re", "compile", &[py_str("^(?:for\\s|\\w+ing\\b|until\\b|then\\s|and\\b|so\\b|which\\b|that\\b|it\\b|this\\b|about\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._NOT_A_TAIL"); }
    s__NOT_A_TAIL(stdlib_call("re", "compile", &[py_str("^(?:and|but|so|it|they|them|this|that|the|a|an|after|before|since|until|once|when|while)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._DANGLES"); }
    s__DANGLES(stdlib_call("re", "compile", &[py_str("\\b(?:until|till|because|so that|so|which|that|when|where|if|while|unless|than|and|or|but|to|of|in|on|at|for|with|by|from|as|into|about|after|before)[^,;:.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._A_PORTION"); }
    s__A_PORTION(stdlib_call("re", "compile", &[py_str("\\b(?:for each|for every|a time|per|twice|once a|\\d+ times)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._STOPPED"); }
    s__STOPPED(call_builtin("frozenset", &[mcall(&py_str("\na an the and or but if then so of to in on at for with from by is are was were be been being it\nits this that these those i you we they he she them him her your our my me us do does did done\nnot no as than there here what which who when where how why can could should would will just\none two three keep keeps keeping go going get gets got make makes made say says said\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._ASK_WORDS"); }
    s__ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nkeep keeps keeping going long longer how should shall could would can do does did i we\nmuch many time times more again this that it thing things\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._KEEP_LEADS"); }
    s__KEEP_LEADS(py_tuple(vec![py_str("Keep it up for {length}"), py_str("Stay with it for {length}"), py_str("Give it {length}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.pace._KEEP_MARK"); }
    s__KEEP_MARK(py_tuple(vec![py_str("the answer's own mark for it"), py_str("the answer's own length for it"), py_str("what the answer says to allow")]));
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
pub fn asks_the_pace(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { py_str("") };
    let mut found: Py = Py::None;
    let mut in_the_answer: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks how slowly or how fast and names no subject of its own.\n\n    A subject *the answer above already named* is not a subject of the line's own: \"how slowly\n    should i add the stock\" after a recipe that talks about the stock is asking about the answer\n    in hand. Anything else named - a 5k, a rowing machine, onions - has an entry of its own and is\n    left to the module that has it.\n    ");
    said = mcall(&_clean(&[text.clone()]), "strip", &[py_str(" ?!.,;:'\"")]);
    found = mcall(&g_PACE_ASK(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&mcall(&op_add(&op_add(&slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)), "replace", &[py_str("'s"), py_str(" ")]), "split", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "sub", &[py_str("[^a-z0-9]"), py_str(""), w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    in_the_answer = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__ALLOWED(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&in_the_answer.clone(), &w.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _last_answer(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _clauses(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut out: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The answer cut into the short pieces a pace is said in: on sentence ends and commas.");
    pieces = stdlib_call("re", "split", &[py_str("(?<=[.;!?])\\s+|;\\s*|,\\s*|:\\s+|\\s+-\\s+"), call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pieces.clone());
    for __item in __iter1 {
    piece = __item.clone();
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&piece.clone(), "strip", &[py_str(" ,.;")]), "split", &[])]);
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|but|then|so)\\s+")); __args.push(py_str("")); __args.push(text.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&text.clone()) {
    let _ = mcall(&out.clone(), "append", &[text.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _trim(args: &[Py]) -> Py {
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
    let mut most: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "most") { v.clone() }
    else { Py::Int(96) };
    let mut comma: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut space: Py = Py::None;
    let _ = py_str("A clause short enough to read out, cut at a comma rather than mid-phrase.");
    if py_truthy(&op_le(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &most.clone())) {
    return text.clone();
    }
    cut = slice_or_raise(&text.clone(), &Py::None, &most.clone(), &Py::None);
    comma = mcall(&cut.clone(), "rfind", &[py_str(",")]);
    if py_truthy(&op_gt(&comma.clone(), &op_floordiv(&most.clone(), &Py::Int(2)))) {
    cut = slice_or_raise(&cut.clone(), &Py::None, &comma.clone(), &Py::None);
    } else {
    space = mcall(&cut.clone(), "rfind", &[py_str(" ")]);
    if py_truthy(&op_gt(&space.clone(), &op_floordiv(&most.clone(), &Py::Int(2)))) {
    cut = slice_or_raise(&cut.clone(), &Py::None, &space.clone(), &Py::None);
    }
    }
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&cut.clone(), "split", &[])]), "rstrip", &[py_str(",;:")]);
    Py::None
}
pub fn _parts(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut after: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut index: Py = Py::None;
    let mut pace: Py = Py::None;
    let mut stepped: Py = Py::None;
    let mut test: Py = Py::None;
    let mut timed: Py = Py::None;
    let _ = py_str("The answer's own pace, best first, its own test, and the step that comes after it.");
    clauses = _clauses(&[answer.clone()]);
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![]), py_str(""), py_str("")]), 4, None);
    stepped = __unpk1[0].clone();
    timed = __unpk1[1].clone();
    test = __unpk1[2].clone();
    after = __unpk1[3].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    index = __unpk3[0].clone();
    clause = __unpk3[1].clone();
    if py_truthy(&{ let __b = mcall(&g__STEP(), "search", &[clause.clone()]); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[stepped.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    let _ = mcall(&stepped.clone(), "append", &[_trim(&[clause.clone()])]);
    } else {
    if py_truthy(&{ let __b = mcall(&g__PACE(), "search", &[clause.clone()]); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[timed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    let _ = mcall(&timed.clone(), "append", &[_trim(&[clause.clone()])]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&test.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__TEST(), "search", &[clause.clone()]) } }) {
    test = _trim(&[mcall(&mcall(&mcall(&g__TEST(), "search", &[clause.clone()]), "group", &[Py::Int(0)]), "strip", &[])]);
    }
    }
    }
    if py_truthy(&{ let __b = { let __b = stepped.clone(); if py_truthy(&__b) { __b } else { timed.clone() } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&after.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_lead_in(&[clause.clone()]))) } } } }) {
    after = _the_next_step(&[slice_or_raise(&clauses.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)]);
    }
    }
    pace = op_add(&stepped.clone(), &timed.clone());
    if py_truthy(&Py::Bool(py_contains(&pace.clone(), &after.clone()))) {
    after = py_str("");
    }
    return py_tuple(vec![pace.clone(), test.clone(), after.clone()]);
    Py::None
}
pub fn _the_next_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clauses: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clauses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clauses"))) };
    let mut clause: Py = Py::None;
    let _ = py_str("The first clause after the pace that names something to do, rather than carrying on.");
    let __iter1 = py_iter_or_raise(&clauses.clone());
    for __item in __iter1 {
    clause = __item.clone();
    clause = _trim(&[clause.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clause.clone())); if py_truthy(&__b) { __b } else { { let __b = _a_lead_in(&[clause.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__CONTINUES(), "match", &[clause.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g__PACE(), "search", &[clause.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__STEP(), "search", &[clause.clone()]) } }) {
    continue;
    }
    return clause.clone();
    }
    return py_str("");
    Py::None
}
pub fn _a_lead_in(args: &[Py]) -> Py {
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
    let _ = py_str("A clause that only introduces the one after it (\"to start with\", \"first of all\").");
    return call_builtin("bool", &[stdlib_call("re", "match", &[py_str("^(?:to start with|first|first of all|for a start)\\b"), clause.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_pace_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut lead: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut opening: Py = Py::None;
    let _ = py_str("True when the answer above is this file's own reply, so the rate has been given once.");
    opening = mcall(&slice_or_raise(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), &Py::None, &Py::Int(80), &Py::None), "lower", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&g__LEADS(), "values", &[]));
    for __item in __iter1 {
    leads = __item.clone();
    let __iter2 = py_iter_or_raise(&leads.clone());
    for __item in __iter2 {
    lead = __item.clone();
    if py_truthy(&mcall(&opening.clone(), "startswith", &[slice_or_raise(&mcall(&lead.clone(), "lower", &[]), &Py::None, &Py::Int(24), &Py::None)])) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn asks_the_keep_up(args: &[Py]) -> Py {
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
    let mut answer: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { py_str("") };
    let mut found: Py = Py::None;
    let mut in_the_answer: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks how long to keep going and names no subject of its own.");
    said = mcall(&_clean(&[text.clone()]), "strip", &[py_str(" ?!.,;:'\"")]);
    found = mcall(&g_KEEP_UP_ASK(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&mcall(&op_add(&op_add(&slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)), "replace", &[py_str("'s"), py_str(" ")]), "split", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "sub", &[py_str("[^a-z0-9]"), py_str(""), w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    in_the_answer = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__ALLOWED(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__KEEP_ALLOWED(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&in_the_answer.clone(), &w.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_doing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut cut: Py = Py::None;
    let _ = py_str("The words the answer puts after its length: \"of doing it the same way every single time\".");
    after = stdlib_call("re", "sub", &[py_str("^(?:of|in|on|for|at)\\s+"), py_str(""), mcall(&after.clone(), "strip", &[py_str(" ,.;")])]).unwrap_or_else(|e| exc::raise(e));
    cut = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|was|were|turns?|helps?|makes?|works?|should|will|but)\\b")); __args.push(after.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    cut = mcall(&g__DANGLES(), "sub", &[py_str(""), cut.clone()]);
    if py_truthy(&op_gt(&mcall(&cut.clone(), "count", &[py_str("(")]), &mcall(&cut.clone(), "count", &[py_str(")")]))) {
    cut = slice_or_raise(&cut.clone(), &Py::None, &mcall(&cut.clone(), "rfind", &[py_str("(")]), &Py::None);
    }
    cut = _trim(&[mcall(&mcall(&py_str(" "), "join", &[mcall(&cut.clone(), "split", &[])]), "strip", &[py_str(" ,.;-")]), Py::Int(80)]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { mcall(&g__NOT_A_TAIL(), "match", &[cut.clone()]) } }) {
    return py_str("");
    }
    return cut.clone();
    Py::None
}
pub fn _the_length(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut asked: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)) };
    let mut clause: Py = Py::None;
    let mut doing: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut length: Py = Py::None;
    let _ = py_str("The answer's own length of time, and what it says to do over it.\n\n    An answer can hold more than one length (\"4 to 6 hours with a fold every hour\" and \"a week of\n    feeding\" in the sourdough card), and the one the person asked about is the one in the clause\n    that names what they said: \"how long should i keep feeding it\" wants the week of feeding, not\n    the bulk rise. When no clause names it, the first length in the answer is the answer's own.\n    ");
    first = py_tuple(vec![py_str(""), py_str("")]);
    let __iter1 = py_iter_or_raise(&_clauses(&[answer.clone()]));
    for __item in __iter1 {
    clause = __item.clone();
    found = mcall(&g__A_LENGTH(), "search", &[clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g__A_PORTION(), "search", &[clause.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_RATE_LEAD(), "search", &[slice_or_raise(&clause.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None)]) } }) {
    continue;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]), _the_doing(&[slice_or_raise(&clause.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])]), 2, None);
    length = __unpk2[0].clone();
    doing = __unpk2[1].clone();
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { _names_it(&[clause.clone(), asked.clone()]) } }) {
    return py_tuple(vec![length.clone(), doing.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&geti(&first.clone(), &Py::Int(0))))) {
    first = py_tuple(vec![length.clone(), doing.clone()]);
    }
    }
    return first.clone();
    Py::None
}
pub fn _names_it(args: &[Py]) -> Py {
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
    let mut asked: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: asked"))) };
    let mut words: Py = Py::None;
    let _ = py_str("Whether the clause says the thing the person asked about, near enough by its stem.");
    words = _stems(&[clause.clone()]);
    return call_builtin("bool", &[op_bitand(&words.clone(), &asked.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _stems(args: &[Py]) -> Py {
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
    let mut ending: Py = Py::None;
    let mut out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The content words of a line, cut back so \"feeding\", \"feeds\" and \"feed\" are one word.");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]{4,}"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g__STOPPED(), &word.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter2 {
    ending = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[ending.clone()]) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    let _ = mcall(&out.clone(), "add", &[word.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _content(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut w: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]{4,}"), mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__STOPPED(), &w.clone()))) {  __out.push(w.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _about_the_same_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut other: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other"))) };
    let _ = py_str("Two bot answers on one subject share the words the subject is made of.");
    return op_ge(&call_builtin("len", &[op_bitand(&_content(&[one.clone()]), &_content(&[other.clone()]))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    Py::None
}
pub fn _the_advice_length(args: &[Py]) -> Py {
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
    let mut answers: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answers") { v.clone() }
    else { Py::Int(3) };
    let mut asked: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked") { v.clone() }
    else { call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)) };
    let mut action: Py = Py::None;
    let mut length: Py = Py::None;
    let mut older: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The length of time in the advice in hand - the newest answer, or the one before it.\n\n    \"how long should i keep going in\" comes after the light-on answer, while the length (\"two or\n    three weeks of doing it the same way\") was said in the answer before that; both are about the\n    same bedtime. A length from an older turn is only taken when the two answers are about the\n    same thing, so a figure from an answer on another subject is never handed back here.\n    ");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[_bot_turns(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    let _ = mcall(&said.clone(), "append", &[turn.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &answers.clone())) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    let __unpk2 = py_unpack(&_the_length(&[geti(&said.clone(), &Py::Int(0)), asked.clone()]), 2, None);
    length = __unpk2[0].clone();
    action = __unpk2[1].clone();
    if py_truthy(&length.clone()) {
    return py_tuple(vec![length.clone(), action.clone()]);
    }
    let __iter3 = py_iter_or_raise(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter3 {
    older = __item.clone();
    let __unpk4 = py_unpack(&_the_length(&[older.clone(), asked.clone()]), 2, None);
    length = __unpk4[0].clone();
    action = __unpk4[1].clone();
    if py_truthy(&{ let __b = length.clone(); if !py_truthy(&__b) { __b } else { _about_the_same_thing(&[older.clone(), geti(&said.clone(), &Py::Int(0))]) } }) {
    return py_tuple(vec![length.clone(), action.clone()]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("")]);
    Py::None
}
pub fn _asked_stems(args: &[Py]) -> Py {
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
    let _ = py_str("What the person asked about, from the line itself, as stems.");
    return call_builtin("frozenset", &[op_sub(&_stems(&[text.clone()]), &g__ASK_WORDS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bot_turns(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Every answer the bot has given, oldest first, that is not one of this file's own.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_pace_reply(&[said.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_keep_reply(&[said.clone()]))) } } } }) {
    let _ = mcall(&out.clone(), "append", &[said.clone()]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _a_keep_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut lead: Py = Py::None;
    let mut opening: Py = Py::None;
    let _ = py_str("True when the answer above is this file's own length reply.");
    opening = mcall(&slice_or_raise(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), &Py::None, &Py::Int(60), &Py::None), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KEEP_LEADS()) { let mut lead: Py = __item.clone(); __out.push(mcall(&opening.clone(), "startswith", &[mcall(&mcall(&geti(&mcall(&lead.clone(), "split", &[py_str("{length}")]), &Py::Int(0)), "strip", &[]), "lower", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_keep_up_line(args: &[Py]) -> Py {
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
    let mut _action: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut length: Py = Py::None;
    let _ = py_str("True only when this line is the one the length reply answers.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_KEEP_UP_ASK(), "search", &[_clean(&[text.clone()])])))) {
    return Py::Bool(false);
    }
    answer = _last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { { let __b = _a_pace_reply(&[answer.clone()]); if py_truthy(&__b) { __b } else { _a_keep_reply(&[answer.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&asks_the_keep_up(&[text.clone(), answer.clone()])))) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("asked".to_string(), _asked_stems(&[text.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_advice_length(args))(&__args) }, 2, None);
    length = __unpk1[0].clone();
    _action = __unpk1[1].clone();
    return call_builtin("bool", &[length.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _keep_up_reply_for(args: &[Py]) -> Py {
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
    let mut action: Py = Py::None;
    let mut first: Py = Py::None;
    let mut length: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&the_keep_up_line(&[text.clone(), memory.clone()])))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("asked".to_string(), _asked_stems(&[text.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_advice_length(args))(&__args) }, 2, None);
    length = __unpk1[0].clone();
    action = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&length.clone()))) {
    return Py::None;
    }
    first = op_modulo(&py_str("%s - %s"), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("length".to_string(), _as_clause(&[length.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g__KEEP_LEADS(), memory.clone(), text.clone()]), "format", args))(&__args) }, pyrt::xrefs::xcall("compose", "pick", &[g__KEEP_MARK(), memory.clone(), text.clone()])]));
    if py_truthy(&action.clone()) {
    return op_modulo(&py_str("%s, %s."), &py_tuple(vec![first.clone(), _as_clause(&[action.clone()])]));
    }
    return op_add(&first.clone(), &py_str("."));
    Py::None
}
pub fn the_pace_line(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut pace: Py = Py::None;
    let mut test: Py = Py::None;
    let _ = py_str("True only when this line is the one `reply_for` answers: a rate, and a pace above it.");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PACE_ASK(), "search", &[_clean(&[text.clone()])])))) {
    return Py::Bool(false);
    }
    answer = _last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = _a_pace_reply(&[answer.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_the_pace(&[text.clone(), answer.clone()]))) } }) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&_parts(&[answer.clone()]), 3, None);
    pace = __unpk1[0].clone();
    test = __unpk1[1].clone();
    after = __unpk1[2].clone();
    return { let __b = call_builtin("bool", &[pace.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[{ let __b = test.clone(); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[pace.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { after.clone() } } } }]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let mut after: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut f: Py = Py::None;
    let mut first: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut pace: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut test: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The pace or the length of the answer just given, or None when this is not that line.");
    if py_truthy(&mcall(&g_KEEP_UP_ASK(), "search", &[_clean(&[text.clone()])])) {
    return _keep_up_reply_for(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PACE_ASK(), "search", &[_clean(&[text.clone()])])))) {
    return Py::None;
    }
    answer = _last_answer(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { { let __b = _a_pace_reply(&[answer.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_the_pace(&[text.clone(), answer.clone()]))) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_parts(&[answer.clone()]), 3, None);
    pace = __unpk1[0].clone();
    test = __unpk1[1].clone();
    after = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&pace.clone()))) {
    return Py::None;
    }
    said = _clean(&[text.clone()]);
    word = if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("slowly")))) { py_str("slowly") } else { if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("fast")))) { py_str("fast") } else { if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("soon")))) { py_str("soon") } else { py_str("quickly") } } };
    lead = pyrt::xrefs::xcall("compose", "pick", &[geti(&g__LEADS(), &word.clone()), memory.clone(), text.clone()]);
    first = op_modulo(&py_str("%s: %s."), &py_tuple(vec![lead.clone(), _as_clause(&[geti(&pace.clone(), &Py::Int(0))])]));
    if py_truthy(&test.clone()) {
    opening = slice_or_raise(&mcall(&_as_clause(&[test.clone()]), "split", &[]), &Py::None, &Py::Int(1), &Py::None);
    opening = if py_truthy(&opening.clone()) { mcall(&mcall(&geti(&opening.clone(), &Py::Int(0)), "lower", &[]), "strip", &[py_str(",.;:")]) } else { py_str("") };
    frames = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__TESTED()) { let mut f: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&geti(&mcall(&f.clone(), "split", &[py_str("{test}")]), &Py::Int(0)), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&w.clone(), "lower", &[]), "strip", &[py_str(",.;:")])); } py_list(__out) })() }, &opening.clone()))) {  __out.push(f.clone()); } } py_list(__out) })() };
    second = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("test".to_string(), _as_clause(&[test.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[{ let __b = frames.clone(); if py_truthy(&__b) { __b } else { call_builtin("list", &[g__TESTED()]).unwrap_or_else(|e| exc::raise(e)) } }, memory.clone(), text.clone()]), "format", args))(&__args) };
    } else {
    if py_truthy(&op_gt(&call_builtin("len", &[pace.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    second = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("rest".to_string(), _as_clause(&[geti(&pace.clone(), &Py::Int(1))]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g__REST(), memory.clone(), text.clone()]), "format", args))(&__args) };
    } else {
    if py_truthy(&after.clone()) {
    second = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("next".to_string(), _as_clause(&[after.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g__NEXT(), memory.clone(), text.clone()]), "format", args))(&__args) };
    } else {
    return Py::None;
    }
    }
    }
    return op_add(&op_add(&first.clone(), &py_str(" ")), &second.clone());
    Py::None
}
pub fn _as_clause(args: &[Py]) -> Py {
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
    let _ = py_str("A piece of the answer with its final stop taken off, so it can sit inside a sentence.");
    said = mcall(&mcall(&mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "rstrip", &[py_str(".")]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(2), &Py::None), &py_str("I "))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(2), &Py::None), &py_str("I'"))) } }) {
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return said.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.pace".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "KEEP_UP_ASK" => Some(g_KEEP_UP_ASK()),
            "PACE_ASK" => Some(g_PACE_ASK()),
            "_ALLOWED" => Some(g__ALLOWED()),
            "_ASK_WORDS" => Some(g__ASK_WORDS()),
            "_A_LENGTH" => Some(g__A_LENGTH()),
            "_A_PORTION" => Some(g__A_PORTION()),
            "_A_RATE_LEAD" => Some(g__A_RATE_LEAD()),
            "_CONTINUES" => Some(g__CONTINUES()),
            "_DANGLES" => Some(g__DANGLES()),
            "_KEEP_ALLOWED" => Some(g__KEEP_ALLOWED()),
            "_KEEP_LEADS" => Some(g__KEEP_LEADS()),
            "_KEEP_MARK" => Some(g__KEEP_MARK()),
            "_LEADS" => Some(g__LEADS()),
            "_NEXT" => Some(g__NEXT()),
            "_NOT_A_TAIL" => Some(g__NOT_A_TAIL()),
            "_PACE" => Some(g__PACE()),
            "_REST" => Some(g__REST()),
            "_STEP" => Some(g__STEP()),
            "_STOPPED" => Some(g__STOPPED()),
            "_TEST" => Some(g__TEST()),
            "_TESTED" => Some(g__TESTED()),
            "_a_keep_reply" => Some(py_func("_a_keep_reply", |args: &[Py]| _a_keep_reply(args))),
            "_a_lead_in" => Some(py_func("_a_lead_in", |args: &[Py]| _a_lead_in(args))),
            "_a_pace_reply" => Some(py_func("_a_pace_reply", |args: &[Py]| _a_pace_reply(args))),
            "_about_the_same_thing" => Some(py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args))),
            "_as_clause" => Some(py_func("_as_clause", |args: &[Py]| _as_clause(args))),
            "_asked_stems" => Some(py_func("_asked_stems", |args: &[Py]| _asked_stems(args))),
            "_bot_turns" => Some(py_func("_bot_turns", |args: &[Py]| _bot_turns(args))),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_keep_up_reply_for" => Some(py_func("_keep_up_reply_for", |args: &[Py]| _keep_up_reply_for(args))),
            "_last_answer" => Some(py_func("_last_answer", |args: &[Py]| _last_answer(args))),
            "_names_it" => Some(py_func("_names_it", |args: &[Py]| _names_it(args))),
            "_parts" => Some(py_func("_parts", |args: &[Py]| _parts(args))),
            "_stems" => Some(py_func("_stems", |args: &[Py]| _stems(args))),
            "_the_advice_length" => Some(py_func("_the_advice_length", |args: &[Py]| _the_advice_length(args))),
            "_the_doing" => Some(py_func("_the_doing", |args: &[Py]| _the_doing(args))),
            "_the_length" => Some(py_func("_the_length", |args: &[Py]| _the_length(args))),
            "_the_next_step" => Some(py_func("_the_next_step", |args: &[Py]| _the_next_step(args))),
            "_trim" => Some(py_func("_trim", |args: &[Py]| _trim(args))),
            "asks_the_keep_up" => Some(py_func("asks_the_keep_up", |args: &[Py]| asks_the_keep_up(args))),
            "asks_the_pace" => Some(py_func("asks_the_pace", |args: &[Py]| asks_the_pace(args))),
            "pick" => Some(g_pick()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_keep_up_line" => Some(py_func("the_keep_up_line", |args: &[Py]| the_keep_up_line(args))),
            "the_pace_line" => Some(py_func("the_pace_line", |args: &[Py]| the_pace_line(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "KEEP_UP_ASK" => s_KEEP_UP_ASK(v),
            "PACE_ASK" => s_PACE_ASK(v),
            "_ALLOWED" => s__ALLOWED(v),
            "_ASK_WORDS" => s__ASK_WORDS(v),
            "_A_LENGTH" => s__A_LENGTH(v),
            "_A_PORTION" => s__A_PORTION(v),
            "_A_RATE_LEAD" => s__A_RATE_LEAD(v),
            "_CONTINUES" => s__CONTINUES(v),
            "_DANGLES" => s__DANGLES(v),
            "_KEEP_ALLOWED" => s__KEEP_ALLOWED(v),
            "_KEEP_LEADS" => s__KEEP_LEADS(v),
            "_KEEP_MARK" => s__KEEP_MARK(v),
            "_LEADS" => s__LEADS(v),
            "_NEXT" => s__NEXT(v),
            "_NOT_A_TAIL" => s__NOT_A_TAIL(v),
            "_PACE" => s__PACE(v),
            "_REST" => s__REST(v),
            "_STEP" => s__STEP(v),
            "_STOPPED" => s__STOPPED(v),
            "_TEST" => s__TEST(v),
            "_TESTED" => s__TESTED(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.pace", module_obj);
    pyrt::xrefs::reg_fn("compose.pace", "_clean", py_func("_clean", |args: &[Py]| _clean(args)));
    pyrt::xrefs::reg_fn("compose.pace", "asks_the_pace", py_func("asks_the_pace", |args: &[Py]| asks_the_pace(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_last_answer", py_func("_last_answer", |args: &[Py]| _last_answer(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_clauses", py_func("_clauses", |args: &[Py]| _clauses(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_trim", py_func("_trim", |args: &[Py]| _trim(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_parts", py_func("_parts", |args: &[Py]| _parts(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_the_next_step", py_func("_the_next_step", |args: &[Py]| _the_next_step(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_a_lead_in", py_func("_a_lead_in", |args: &[Py]| _a_lead_in(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_a_pace_reply", py_func("_a_pace_reply", |args: &[Py]| _a_pace_reply(args)));
    pyrt::xrefs::reg_fn("compose.pace", "asks_the_keep_up", py_func("asks_the_keep_up", |args: &[Py]| asks_the_keep_up(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_the_doing", py_func("_the_doing", |args: &[Py]| _the_doing(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_the_length", py_func("_the_length", |args: &[Py]| _the_length(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_names_it", py_func("_names_it", |args: &[Py]| _names_it(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_stems", py_func("_stems", |args: &[Py]| _stems(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_content", py_func("_content", |args: &[Py]| _content(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_about_the_same_thing", py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_the_advice_length", py_func("_the_advice_length", |args: &[Py]| _the_advice_length(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_asked_stems", py_func("_asked_stems", |args: &[Py]| _asked_stems(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_bot_turns", py_func("_bot_turns", |args: &[Py]| _bot_turns(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_a_keep_reply", py_func("_a_keep_reply", |args: &[Py]| _a_keep_reply(args)));
    pyrt::xrefs::reg_fn("compose.pace", "the_keep_up_line", py_func("the_keep_up_line", |args: &[Py]| the_keep_up_line(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_keep_up_reply_for", py_func("_keep_up_reply_for", |args: &[Py]| _keep_up_reply_for(args)));
    pyrt::xrefs::reg_fn("compose.pace", "the_pace_line", py_func("the_pace_line", |args: &[Py]| the_pace_line(args)));
    pyrt::xrefs::reg_fn("compose.pace", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_fn("compose.pace", "_as_clause", py_func("_as_clause", |args: &[Py]| _as_clause(args)));
    pyrt::xrefs::reg_access("compose.pace", "KEEP_UP_ASK", g_KEEP_UP_ASK, s_KEEP_UP_ASK);
    pyrt::xrefs::reg_access("compose.pace", "PACE_ASK", g_PACE_ASK, s_PACE_ASK);
    pyrt::xrefs::reg_access("compose.pace", "_ALLOWED", g__ALLOWED, s__ALLOWED);
    pyrt::xrefs::reg_access("compose.pace", "_ASK_WORDS", g__ASK_WORDS, s__ASK_WORDS);
    pyrt::xrefs::reg_access("compose.pace", "_A_LENGTH", g__A_LENGTH, s__A_LENGTH);
    pyrt::xrefs::reg_access("compose.pace", "_A_PORTION", g__A_PORTION, s__A_PORTION);
    pyrt::xrefs::reg_access("compose.pace", "_A_RATE_LEAD", g__A_RATE_LEAD, s__A_RATE_LEAD);
    pyrt::xrefs::reg_access("compose.pace", "_CONTINUES", g__CONTINUES, s__CONTINUES);
    pyrt::xrefs::reg_access("compose.pace", "_DANGLES", g__DANGLES, s__DANGLES);
    pyrt::xrefs::reg_access("compose.pace", "_KEEP_ALLOWED", g__KEEP_ALLOWED, s__KEEP_ALLOWED);
    pyrt::xrefs::reg_access("compose.pace", "_KEEP_LEADS", g__KEEP_LEADS, s__KEEP_LEADS);
    pyrt::xrefs::reg_access("compose.pace", "_KEEP_MARK", g__KEEP_MARK, s__KEEP_MARK);
    pyrt::xrefs::reg_access("compose.pace", "_LEADS", g__LEADS, s__LEADS);
    pyrt::xrefs::reg_access("compose.pace", "_NEXT", g__NEXT, s__NEXT);
    pyrt::xrefs::reg_access("compose.pace", "_NOT_A_TAIL", g__NOT_A_TAIL, s__NOT_A_TAIL);
    pyrt::xrefs::reg_access("compose.pace", "_PACE", g__PACE, s__PACE);
    pyrt::xrefs::reg_access("compose.pace", "_REST", g__REST, s__REST);
    pyrt::xrefs::reg_access("compose.pace", "_STEP", g__STEP, s__STEP);
    pyrt::xrefs::reg_access("compose.pace", "_STOPPED", g__STOPPED, s__STOPPED);
    pyrt::xrefs::reg_access("compose.pace", "_TEST", g__TEST, s__TEST);
    pyrt::xrefs::reg_access("compose.pace", "_TESTED", g__TESTED, s__TESTED);
    pyrt::xrefs::reg_access("compose.pace", "pick", g_pick, s_pick);
}

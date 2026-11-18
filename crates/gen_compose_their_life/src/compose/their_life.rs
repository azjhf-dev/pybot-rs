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


thread_local! { static G_ABOUT_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ALREADY_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AN_AGE_NAME_OR_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEED_OF_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MISSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_THEY_ARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.their_life: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_THEM() -> Py { ensure_init(); G_ABOUT_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THEM(v: Py) { ensure_init(); G_ABOUT_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_ALREADY_SAID() -> Py { ensure_init(); G_ALREADY_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY_SAID(v: Py) { ensure_init(); G_ALREADY_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_AN_AGE_NAME_OR_PLACE() -> Py { ensure_init(); G_AN_AGE_NAME_OR_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AN_AGE_NAME_OR_PLACE(v: Py) { ensure_init(); G_AN_AGE_NAME_OR_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_TO() -> Py { ensure_init(); G_A_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_TO(v: Py) { ensure_init(); G_A_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEED_OF_THEIRS() -> Py { ensure_init(); G_A_NEED_OF_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEED_OF_THEIRS(v: Py) { ensure_init(); G_A_NEED_OF_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PRONOUN() -> Py { ensure_init(); G_A_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PRONOUN(v: Py) { ensure_init(); G_A_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_MISSING() -> Py { ensure_init(); G_MISSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MISSING(v: Py) { ensure_init(); G_MISSING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THIS() -> Py { ensure_init(); G_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THIS(v: Py) { ensure_init(); G_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_PROSE() -> Py { ensure_init(); G_PROSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROSE(v: Py) { ensure_init(); G_PROSE.with(|c| *c.borrow_mut() = v); }
pub fn g_RECALL() -> Py { ensure_init(); G_RECALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECALL(v: Py) { ensure_init(); G_RECALL.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION() -> Py { ensure_init(); G_RELATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION(v: Py) { ensure_init(); G_RELATION.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_WHO() -> Py { ensure_init(); G_SAID_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_WHO(v: Py) { ensure_init(); G_SAID_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_STAY() -> Py { ensure_init(); G_STAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAY(v: Py) { ensure_init(); G_STAY.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP_ABOUT() -> Py { ensure_init(); G_STOP_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP_ABOUT(v: Py) { ensure_init(); G_STOP_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTE() -> Py { ensure_init(); G_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTE(v: Py) { ensure_init(); G_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEM() -> Py { ensure_init(); G_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEM(v: Py) { ensure_init(); G_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_LONG() -> Py { ensure_init(); G_TOO_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_LONG(v: Py) { ensure_init(); G_TOO_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_THEY_ARE() -> Py { ensure_init(); G_WHERE_THEY_ARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_THEY_ARE(v: Py) { ensure_init(); G_WHERE_THEY_ARE.with(|c| *c.borrow_mut() = v); }
pub fn g_WORK() -> Py { ensure_init(); G_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORK(v: Py) { ensure_init(); G_WORK.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A particular of the person's own life is theirs to say, and no search can find it.\n\n\"I have a sister in Cork\" / \"what does she do\", \"do i like fish\", \"how long is she likely to be\nin\" - main answered all of them with the search hedge (\"I don't know enough to answer that\nhonestly. A search should sort you out quickly\"). A search cannot say what somebody's sister does\nfor a living, whether a person likes fish, or how long a ward will keep somebody in: the only\nplaces those facts live are the person in front of the bot and the people around them. A pointer\nat a search page tells them to go and look for something nobody has written down.\n\nWhat IS honest here is short: the bot has not met them and has no page to read, so it says that\nin a few words and asks for the one thing it needs - and it keeps what it is told for the rest of\nthe chat, which is the half it can really do.\n\nFour kinds, all of them a question with no source but the people in it:\n  - \"taste\"  - their own like or dislike (\"do i like fish\", \"would i like oysters\")\n  - \"work\"   - what somebody they know does (\"what does she do\", \"whats she like\")\n  - \"stay\"   - how long somebody will be in hospital, or away (\"how long is she likely to be in\")\n  - \"theirs\" - any other particular of a person they know (\"who is my friend coming with\")\n\nAn ask for an idea about them is not one of the four either: \"what could she build for the\nproject\", \"what can she take to school\", \"what does she need to get better\" want a suggestion,\nand this reader has one answer - that the person would know better - which is no help at all.\n\nAn age, a name or a place about one of their people is not read here: @scarlet-bat's\n`bot/compose/untold.py` (#8690) answers those three shapes (\"how old is she\", \"whats her name\",\n\"where does he live\"), and two readers on one turn would only fight over it.\n\nIt reads only the bare question, and it is called only where the composer's own reply was the\nsearch hedge, so a card that answers, a thread that remembers, the records a person can only see\nthemselves (bot/compose/fallback.py, the own-record families) and every safety answer keep their\nturn. A fact said earlier in the chat is the memory store's, not this: if the person has already\ntold the bot what they asked about, the thread answers it before the fallback is reached.\n");
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.TOO_LONG"); }
    s_TOO_LONG(Py::Int(12));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.TASTE"); }
    s_TASTE(stdlib_call("re", "compile", &[py_str("^(?:so |and |but |ok |okay )?(?:do|did|would|will|should)\\s+i\\s+(?:actually\\s+|even\\s+|really\\s+|still\\s+|probably\\s+|think\\s+i\\s+)?(?:like|love|enjoy|hate|want|fancy|prefer|get\\s+on\\s+with)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.RELATION"); }
    s_RELATION(py_str("(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.THEM"); }
    s_THEM(op_add(&op_add(&py_str("(?:(?:my|our|his|her|their)\\s+"), &g_RELATION()), &py_str("|she|he|they|him|her|them)")));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.AN_AGE_NAME_OR_PLACE"); }
    s_AN_AGE_NAME_OR_PLACE(stdlib_call("re", "compile", &[py_str("\\bhow\\s+old\\b|\\bname\\b|\\bwhere\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.WORK"); }
    s_WORK(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bwhat\\s+(?:does|do|did)\\s+"), &g_THEM()), &py_str("\\s+(?:do|work\\s+as)\\b|\\bwhats?\\s+")), &g_THEM()), &py_str("\\s+like\\b|\\bwhat\\s+is\\s+")), &g_THEM()), &py_str("\\s+like\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.ABOUT_THEM"); }
    s_ABOUT_THEM(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:who|what|where|when|which|how(?:\\s+\\w+)?)\\s+(?P<about>(?:\\w+\\s+){0,2})(?:is|was|are|were|does|did|do|has|have)\\s+"), &g_THEM()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.ALREADY_SAID"); }
    s_ALREADY_SAID(stdlib_call("re", "compile", &[py_str("\\b[A-Za-z]{4,}\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.STOP_ABOUT"); }
    s_STOP_ABOUT(call_builtin("frozenset", &[mcall(&py_str("what whats which when where who whom whose does did have has about going doing being their there these those"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.A_NEED_OF_THEIRS"); }
    s_A_NEED_OF_THEIRS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &g_THEM()), &py_str("\\s+(?:need|needs|want|wants|take|takes|build|builds|make|makes|cook|cooks|wear|wears|bring|brings|give|gives|try|tries|use|uses|buy|buys|eat|eats|say|says|tell|tells|get|gets|say\\s+to|do\\s+about)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.STAY"); }
    s_STAY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bhow\\s+long\\s+(?:is|are|will|would|might|could|does|do)\\s+"), &g_THEM()), &py_str("\\s+(?:(?:likely|going|expected|probably|meant)\\s+to\\s+)?(?:be\\s+)?(?:in|staying|stay|away|off|be\\s+off|be\\s+in|have\\s+to\\s+stay|need\\s+to\\s+stay)\\b|\\bhow\\s+long\\s+(?:is|are)\\s+(?:the\\s+)?(?:course|treatment|recovery|stay|operation)\\s+(?:going\\s+to\\s+)?(?:take|last)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.MISSING"); }
    s_MISSING(stdlib_call("re", "compile", &[py_str("\\b(?:missing|lost|gone|disappeared|found|ran\\s+off|run\\s+off|not\\s+come\\s+home|havent\\s+come\\s+home|hasnt\\s+come\\s+home)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.WHERE_THEY_ARE"); }
    s_WHERE_THEY_ARE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bwhere(?:'s|\\s+is|\\s+are|\\s+was|\\s+were)\\s+"), &g_THEM()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.A_PRONOUN"); }
    s_A_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:she|he|they|him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.SAID_WHO"); }
    s_SAID_WHO(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:my|our|his|her|their)\\s+"), &g_RELATION()), &py_str("\\b|\\b(?:mum|dad|son|daughter|sister|brother|friend|mate|partner|husband|wife|nan|gran|boss|colleague|neighbour)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.NOT_THIS"); }
    s_NOT_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:excess|booking|reference|policy|premium|prescription|appointment|balance|account|order|invoice|bill|refund|renewal|renew|due\\s+date|expiry|expires|warranty|guarantee|membership|subscription|statement|receipt|tracking|delivery|parcel|form|deadline|opening|closing|open|close|closed|hours|times|timetable|train|bus|flight|what\\s+time|cost|price|how\\s+much)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.RECALL"); }
    s_RECALL(stdlib_call("re", "compile", &[py_str("\\b(?:was\\s+it|which\\s+(?:\\w+\\s+)?was\\s+it|what\\s+did\\s+i\\s+say|again\\b.*\\bwhat|remind\\s+me\\s+what)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.A_HOW_TO"); }
    s_A_HOW_TO(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:do|does|did|can|could|should)\\s+(?:i|you|we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET compose.their_life.PROSE"); }
    s_PROSE(py_dict(vec![(py_str("taste"), py_tuple(vec![py_str("That one's yours, not mine: I can't know what you like from here. What does your gut say?"), py_str("No way for me to know that, sorry - only you can say. What is your instinct telling you?"), py_str("I'd only be guessing, and you'd know in a second. What do you reckon?")])), (py_str("work"), py_tuple(vec![py_str("I've not met them, so I can't say what they do. What do they do?"), py_str("No idea, sorry - nobody has told me. What do they do for a living?"), py_str("Not a thing I could know. Tell me about them and I'll keep it straight from here: what do they do for a living?")])), (py_str("theirs"), py_tuple(vec![py_str("I've never met them, so I've nothing on that one - you'd know better than any page. Tell me and I'll keep it for the rest of our chat."), py_str("That's yours to know and mine to be told. What's the answer?"), py_str("No idea, sorry - nobody has introduced us. Tell me and it stays here with me.")])), (py_str("stay"), py_tuple(vec![py_str("Only the people looking after them can say that - ask the nurse on the ward and they will give you a rough day. How are they in themselves?"), py_str("Nobody here can say how long they'll keep them in. The staff on the ward can, and the desk will ring through for you. How are they doing?"), py_str("That's the ward's call, not mine - the nurse looking after them knows, and it is one question at the desk. How is she in herself?")]))]));
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
    return mcall(&mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .!,")]), "lower", &[]);
    Py::None
}
pub fn _a_thing_already_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut about: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the noun the question turns on was said by the person earlier in the chat.");
    about = mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("about")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_ALREADY_SAID(), "findall", &[mcall(&about.clone(), "lower", &[])])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STOP_ABOUT(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    earlier = mcall(&py_str(" "), "join", &[slice_or_raise(&turns.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&earlier.clone(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _somebody_of_theirs(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("\"she\", \"he\" and \"they\" are theirs when the last few turns named somebody they know.");
    turns = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(6)), &Py::None, &Py::None);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut t: Py = __item.clone(); __out.push(mcall(&g_SAID_WHO(), "search", &[call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The kind of particular this is - \"taste\", \"work\", \"stay\", \"theirs\" - or None.");
    line = _bare(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_TOO_LONG())) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_THIS(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_RECALL(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HOW_TO(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_AN_AGE_NAME_OR_PLACE(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MISSING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WHERE_THEY_ARE(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_NEED_OF_THEIRS(), "search", &[line.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_TASTE(), "search", &[line.clone()])) {
    return py_str("taste");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("work"), g_WORK()]), py_tuple(vec![py_str("stay"), g_STAY()]), py_tuple(vec![py_str("theirs"), g_ABOUT_THEM()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    found = mcall(&pattern.clone(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&gattr(&found.clone(), "re"), &g_ABOUT_THEM())); if !py_truthy(&__b) { __b } else { _a_thing_already_said(&[found.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_PRONOUN(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&py_str("\\b(?:my|our|his|her|their)\\s+"), &g_RELATION()), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_somebody_of_theirs(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }])))) {
    return Py::None;
    }
    }
    return kind.clone();
    }
    return Py::None;
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
    let mut kind: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The honest line and the one question, or None when this is not the ask.");
    kind = reads(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::None;
    }
    lines = geti(&g_PROSE(), &kind.clone());
    turns = call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return geti(&lines.clone(), &op_modulo(&turns.clone(), &call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.their_life".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THEM" => Some(g_ABOUT_THEM()),
            "ALREADY_SAID" => Some(g_ALREADY_SAID()),
            "AN_AGE_NAME_OR_PLACE" => Some(g_AN_AGE_NAME_OR_PLACE()),
            "A_HOW_TO" => Some(g_A_HOW_TO()),
            "A_NEED_OF_THEIRS" => Some(g_A_NEED_OF_THEIRS()),
            "A_PRONOUN" => Some(g_A_PRONOUN()),
            "MISSING" => Some(g_MISSING()),
            "NOT_THIS" => Some(g_NOT_THIS()),
            "PROSE" => Some(g_PROSE()),
            "RECALL" => Some(g_RECALL()),
            "RELATION" => Some(g_RELATION()),
            "SAID_WHO" => Some(g_SAID_WHO()),
            "STAY" => Some(g_STAY()),
            "STOP_ABOUT" => Some(g_STOP_ABOUT()),
            "TASTE" => Some(g_TASTE()),
            "THEM" => Some(g_THEM()),
            "TOO_LONG" => Some(g_TOO_LONG()),
            "WHERE_THEY_ARE" => Some(g_WHERE_THEY_ARE()),
            "WORK" => Some(g_WORK()),
            "_a_thing_already_said" => Some(py_func("_a_thing_already_said", |args: &[Py]| _a_thing_already_said(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_somebody_of_theirs" => Some(py_func("_somebody_of_theirs", |args: &[Py]| _somebody_of_theirs(args))),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THEM" => s_ABOUT_THEM(v),
            "ALREADY_SAID" => s_ALREADY_SAID(v),
            "AN_AGE_NAME_OR_PLACE" => s_AN_AGE_NAME_OR_PLACE(v),
            "A_HOW_TO" => s_A_HOW_TO(v),
            "A_NEED_OF_THEIRS" => s_A_NEED_OF_THEIRS(v),
            "A_PRONOUN" => s_A_PRONOUN(v),
            "MISSING" => s_MISSING(v),
            "NOT_THIS" => s_NOT_THIS(v),
            "PROSE" => s_PROSE(v),
            "RECALL" => s_RECALL(v),
            "RELATION" => s_RELATION(v),
            "SAID_WHO" => s_SAID_WHO(v),
            "STAY" => s_STAY(v),
            "STOP_ABOUT" => s_STOP_ABOUT(v),
            "TASTE" => s_TASTE(v),
            "THEM" => s_THEM(v),
            "TOO_LONG" => s_TOO_LONG(v),
            "WHERE_THEY_ARE" => s_WHERE_THEY_ARE(v),
            "WORK" => s_WORK(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.their_life", module_obj);
    pyrt::xrefs::reg_fn("compose.their_life", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("compose.their_life", "_a_thing_already_said", py_func("_a_thing_already_said", |args: &[Py]| _a_thing_already_said(args)));
    pyrt::xrefs::reg_fn("compose.their_life", "_somebody_of_theirs", py_func("_somebody_of_theirs", |args: &[Py]| _somebody_of_theirs(args)));
    pyrt::xrefs::reg_fn("compose.their_life", "reads", py_func("reads", |args: &[Py]| reads(args)));
    pyrt::xrefs::reg_fn("compose.their_life", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("compose.their_life", "ABOUT_THEM", g_ABOUT_THEM, s_ABOUT_THEM);
    pyrt::xrefs::reg_access("compose.their_life", "ALREADY_SAID", g_ALREADY_SAID, s_ALREADY_SAID);
    pyrt::xrefs::reg_access("compose.their_life", "AN_AGE_NAME_OR_PLACE", g_AN_AGE_NAME_OR_PLACE, s_AN_AGE_NAME_OR_PLACE);
    pyrt::xrefs::reg_access("compose.their_life", "A_HOW_TO", g_A_HOW_TO, s_A_HOW_TO);
    pyrt::xrefs::reg_access("compose.their_life", "A_NEED_OF_THEIRS", g_A_NEED_OF_THEIRS, s_A_NEED_OF_THEIRS);
    pyrt::xrefs::reg_access("compose.their_life", "A_PRONOUN", g_A_PRONOUN, s_A_PRONOUN);
    pyrt::xrefs::reg_access("compose.their_life", "MISSING", g_MISSING, s_MISSING);
    pyrt::xrefs::reg_access("compose.their_life", "NOT_THIS", g_NOT_THIS, s_NOT_THIS);
    pyrt::xrefs::reg_access("compose.their_life", "PROSE", g_PROSE, s_PROSE);
    pyrt::xrefs::reg_access("compose.their_life", "RECALL", g_RECALL, s_RECALL);
    pyrt::xrefs::reg_access("compose.their_life", "RELATION", g_RELATION, s_RELATION);
    pyrt::xrefs::reg_access("compose.their_life", "SAID_WHO", g_SAID_WHO, s_SAID_WHO);
    pyrt::xrefs::reg_access("compose.their_life", "STAY", g_STAY, s_STAY);
    pyrt::xrefs::reg_access("compose.their_life", "STOP_ABOUT", g_STOP_ABOUT, s_STOP_ABOUT);
    pyrt::xrefs::reg_access("compose.their_life", "TASTE", g_TASTE, s_TASTE);
    pyrt::xrefs::reg_access("compose.their_life", "THEM", g_THEM, s_THEM);
    pyrt::xrefs::reg_access("compose.their_life", "TOO_LONG", g_TOO_LONG, s_TOO_LONG);
    pyrt::xrefs::reg_access("compose.their_life", "WHERE_THEY_ARE", g_WHERE_THEY_ARE, s_WHERE_THEY_ARE);
    pyrt::xrefs::reg_access("compose.their_life", "WORK", g_WORK, s_WORK);
}

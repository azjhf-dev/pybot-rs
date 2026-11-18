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


thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ATTRIBUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ATTRIBUTE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_METHOD_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SIZE_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGE_REPLIES_MOVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONFIRMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_REASON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUANTITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUESTION_OPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORTEST_REASON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNIT_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.stands: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_ATTRIBUTES() -> Py { ensure_init(); G_ATTRIBUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATTRIBUTES(v: Py) { ensure_init(); G_ATTRIBUTES.with(|c| *c.borrow_mut() = v); }
pub fn g_ATTRIBUTE_WORDS() -> Py { ensure_init(); G_ATTRIBUTE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ATTRIBUTE_WORDS(v: Py) { ensure_init(); G_ATTRIBUTE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHANGE() -> Py { ensure_init(); G_A_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHANGE(v: Py) { ensure_init(); G_A_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_METHOD_CHANGE() -> Py { ensure_init(); G_A_METHOD_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_METHOD_CHANGE(v: Py) { ensure_init(); G_A_METHOD_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SETTING() -> Py { ensure_init(); G_A_SETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SETTING(v: Py) { ensure_init(); G_A_SETTING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SIZE_CHANGE() -> Py { ensure_init(); G_A_SIZE_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SIZE_CHANGE(v: Py) { ensure_init(); G_A_SIZE_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_ASK() -> Py { ensure_init(); G_CHANGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_ASK(v: Py) { ensure_init(); G_CHANGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_REPLIES() -> Py { ensure_init(); G_CHANGE_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_REPLIES(v: Py) { ensure_init(); G_CHANGE_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGE_REPLIES_MOVED() -> Py { ensure_init(); G_CHANGE_REPLIES_MOVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGE_REPLIES_MOVED(v: Py) { ensure_init(); G_CHANGE_REPLIES_MOVED.with(|c| *c.borrow_mut() = v); }
pub fn g_CONFIRMS() -> Py { ensure_init(); G_CONFIRMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONFIRMS(v: Py) { ensure_init(); G_CONFIRMS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURE() -> Py { ensure_init(); G_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURE(v: Py) { ensure_init(); G_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURED() -> Py { ensure_init(); G_FIGURED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURED(v: Py) { ensure_init(); G_FIGURED.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK() -> Py { ensure_init(); G_LOOK_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK(v: Py) { ensure_init(); G_LOOK_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_REASON_WORDS() -> Py { ensure_init(); G_MOST_REASON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_REASON_WORDS(v: Py) { ensure_init(); G_MOST_REASON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED() -> Py { ensure_init(); G_NOTHING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED(v: Py) { ensure_init(); G_NOTHING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_SUBJECT() -> Py { ensure_init(); G_NOT_THE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_SUBJECT(v: Py) { ensure_init(); G_NOT_THE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBERS() -> Py { ensure_init(); G_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBERS(v: Py) { ensure_init(); G_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUANTITY() -> Py { ensure_init(); G_QUANTITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUANTITY(v: Py) { ensure_init(); G_QUANTITY.with(|c| *c.borrow_mut() = v); }
pub fn g_QUESTION_OPENING() -> Py { ensure_init(); G_QUESTION_OPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUESTION_OPENING(v: Py) { ensure_init(); G_QUESTION_OPENING.with(|c| *c.borrow_mut() = v); }
pub fn g_SENTENCE() -> Py { ensure_init(); G_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); G_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORTEST_REASON() -> Py { ensure_init(); G_SHORTEST_REASON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORTEST_REASON(v: Py) { ensure_init(); G_SHORTEST_REASON.with(|c| *c.borrow_mut() = v); }
pub fn g_STANDS() -> Py { ensure_init(); G_STANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STANDS(v: Py) { ensure_init(); G_STANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS() -> Py { ensure_init(); G_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS(v: Py) { ensure_init(); G_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNIT_FAMILY() -> Py { ensure_init(); G_UNIT_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNIT_FAMILY(v: Py) { ensure_init(); G_UNIT_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOSE() -> Py { ensure_init(); G_WHOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOSE(v: Py) { ensure_init(); G_WHOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A yes/no about a figure the bot itself gave: \"so is eight weeks still right\".\n\nThe bot had said \"Three runs a week gets most people to 5k in about eight weeks\", and then the\nperson asked whether the eight weeks stood. Nothing in the line names a thing of its own - the\nthing it is about is the figure in the bot's own answer - so the knowledge base read the words it\ncould see and answered from a different entry altogether:\n\n    i want to run 5k\n      Three runs a week gets most people to 5k in about eight weeks ...\n    how long will it take\n      Eight weeks.\n    im 52 and a bit overweight\n      Got it. Is there anything you'd like a hand with?\n    so is eight weeks still right\n      Ask in writing and name what you want: a refund, a repair, or money for the trouble. (a\n      consumer-complaint entry, reached by the words \"eight weeks\")\n\nThe figure is the answer's, and the question is whether it still stands, so the answer is the\nfigure with the reason the chat already gave for it: \"Still right: Three runs a week gets most\npeople to 5k in about eight weeks.\" That is also how a friend answers it - they do not go and\nlook something up, they say where the number came from.\n\nNarrow on purpose:\n\n* the line has to ask (a question mark, or \"is/are/does/will\" at the front, or \"still\"/\"right\");\n* the line has to carry a figure - a number and, where it has one, the unit the answer gave it;\n* every other word in the line has to be one that names nothing (\"so is eight weeks still\n  right\"), because \"is eight weeks right for a puppy\" is a question about a puppy;\n* the figure has to be in an earlier answer of this chat, and that answer has to say what the\n  figure rests on, or there is nothing to hand back;\n* a line that takes issue with the answer (\"that cant be right\") is `bot/memory/doubt.py`'s, and\n  a line that is `not` something is a different question, so both are left alone here.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.UNITS"); }
    s_UNITS(call_builtin("frozenset", &[mcall(&py_str("week month day year hour minute second mile km kilo stone pound percent\n    session run portion time glass cup litre pint degree inch cm metre gram ml"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.NUMBERS"); }
    s_NUMBERS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[mcall(&py_str("zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety hundred"), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut value: Py = __unpk1[0].clone(); let mut word: Py = __unpk1[1].clone(); __out.push((word.clone(), value.clone())); } py_dict(__out) })() });
    let _ = py_setitem(&g_NUMBERS(), &py_str("couple"), Py::Int(2));
    let _ = py_setitem(&g_NUMBERS(), &py_str("few"), Py::Int(3));
    let _ = py_setitem(&g_NUMBERS(), &py_str("once"), Py::Int(1));
    let _ = py_setitem(&g_NUMBERS(), &py_str("twice"), Py::Int(2));
    let _ = py_setitem(&g_NUMBERS(), &py_str("thrice"), Py::Int(3));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.FIGURE"); }
    s_FIGURE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(\\d[\\d,]*(?:\\.\\d+)?|"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NUMBERS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")(?:\\s+(")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_UNITS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")s?\\b)?")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.NOTHING_NAMED"); }
    s_NOTHING_NAMED(call_builtin("frozenset", &[mcall(&py_str("so and but or then ok okay right still is are was were am that\n    this it they those these the a an of in on at for to do does did will would can could should\n    my i me you we our yeah yes no anymore any more both quite really actually true correct\n    stand stands work works hold holds apply applies changed change mean means all that's its\n    it's again now today currently nowadays even just also too much very bit little"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.QUESTION_OPENING"); }
    s_QUESTION_OPENING(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:so|and|but|ok|okay|then|right|oh|also|now)\\W+)*(?:is|are|was|were|does|do|did|will|would|should|can|could)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.ASKING"); }
    s_ASKING(stdlib_call("re", "compile", &[py_str("\\b(?:still|right|correct|true|ok|okay|the same|sure)\\b|\\?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.MOST_WORDS"); }
    s_MOST_WORDS(Py::Int(9));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.LOOK_BACK"); }
    s_LOOK_BACK(Py::Int(5));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.SHORTEST_REASON"); }
    s_SHORTEST_REASON(Py::Int(6));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.MOST_REASON_WORDS"); }
    s_MOST_REASON_WORDS(Py::Int(45));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.SENTENCE"); }
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.STANDS"); }
    s_STANDS(py_tuple(vec![py_str("Still right: {reason}"), py_str("That has not changed: {reason}"), py_str("Yes - and that is where it comes from: {reason}")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.CONFIRMS"); }
    s_CONFIRMS(py_tuple(vec![py_str("Still right - {figure}."), py_str("Yes, {figure} is still the number."), py_str("That holds: {figure}, as before.")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.CHANGE_ASK"); }
    s_CHANGE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:does|do|will|would|should|did)\\b[^.?!]{0,24}?\\b(?:change|affect|alter|make (?:a|any) difference to|matter (?:to|for)|impact)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.ATTRIBUTES"); }
    s_ATTRIBUTES(py_dict(vec![(py_str("time"), py_tuple(vec![py_str("minute"), py_str("hour"), py_str("day"), py_str("week"), py_str("month")])), (py_str("long"), py_tuple(vec![py_str("minute"), py_str("hour"), py_str("day"), py_str("week"), py_str("month")])), (py_str("temperature"), py_tuple(vec![py_str("degree")])), (py_str("heat"), py_tuple(vec![py_str("degree")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.ATTRIBUTE_WORDS"); }
    s_ATTRIBUTE_WORDS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:the|my|our|its)\\s+("), &mcall(&py_str("|"), "join", &[g_ATTRIBUTES()])), &py_str("|cooking time|oven time|time)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.A_SETTING"); }
    s_A_SETTING(stdlib_call("re", "compile", &[py_str("\\b(?:boiler|thermostat|programmer|clock|timer|heating|timer\\s+on)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.FIGURED"); }
    s_FIGURED(stdlib_call("re", "compile", &[py_str("\\b(\\d[\\d,.]*(?:\\s*(?:to|or|-)\\s*\\d[\\d,.]*)?)\\s*(hours?|hrs?)\\s+(\\d[\\d,.]*)\\s*(minutes?|mins?)?\\b(?!\\s*(?:g|kg|ml|cl|lb|lbs|oz|cm|degrees?)\\b)|\\b(\\d[\\d,.]*(?:\\s*(?:to|or|-)\\s*\\d[\\d,.]*)?)\\s*(minutes?|mins?|hours?|hrs?|days?|weeks?|months?|degrees?|C\\b|\\u00b0C)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.UNIT_FAMILY"); }
    s_UNIT_FAMILY(py_dict(vec![(py_str("minute"), py_str("time")), (py_str("min"), py_str("time")), (py_str("hour"), py_str("time")), (py_str("hr"), py_str("time")), (py_str("day"), py_str("time")), (py_str("week"), py_str("time")), (py_str("month"), py_str("time")), (py_str("degree"), py_str("temperature")), (py_str("c"), py_str("temperature")), (py_str("°c"), py_str("temperature"))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.CHANGE_REPLIES"); }
    s_CHANGE_REPLIES(py_dict(vec![(py_str("time"), py_tuple(vec![py_str("No - {figure} was the time and it stays that. What changed is what goes in it, not how long it takes."), py_str("That does not move it: {figure}, same as before."), py_str("Same time - {figure}. The swap changes what is in it, not how long it is in for."), py_str("The time holds: {figure}. It is the dish that sets it, not the filling."), py_str("No change there - {figure}, and that is the figure to cook to.")])), (py_str("temperature"), py_tuple(vec![py_str("The heat holds: {figure}, same as before."), py_str("No change to the heat - {figure}, and that is the oven to cook it at."), py_str("Same heat - {figure}. The swap changes what goes in the dish, not the oven."), py_str("That does not move it: {figure}, the heat it was already to be cooked at.")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.CHANGE_REPLIES_MOVED"); }
    s_CHANGE_REPLIES_MOVED(py_dict(vec![(py_str("time"), py_tuple(vec![py_str("That moves it: {figure} now, not the time from before."), py_str("It does - {figure} for this version."), py_str("Yes - {figure}. The swap changes how long it takes."), py_str("It does move it: {figure}, so cook to that instead.")])), (py_str("temperature"), py_tuple(vec![py_str("The heat moves too: {figure} for this version."), py_str("It does - {figure} now, not the oven from before."), py_str("Yes, the heat changes: {figure}.")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.NOT_THE_SUBJECT"); }
    s_NOT_THE_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("minute minutes hour hours second seconds day days week weeks\n    month months degree degrees oven ovens tray tins pan pans tin dish dishes cook cooking cooked\n    bake baked roast roasted about until when then they this that with from your yours each every\n    before after while total same time times heat high medium low fan"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.A_METHOD_CHANGE"); }
    s_A_METHOD_CHANGE(stdlib_call("re", "compile", &[py_str("\\b(?:slow cooker|crock ?pot|pressure cooker|instant pot|air ?fryer|microwave|steam\\w*|sous ?vide|barbecue|bbq|grill\\w*|poach\\w*|deep[- ]?fr(?:y|ied|ying)|boil\\w*|on the hob|in the oven instead|oven instead)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.A_CHANGE"); }
    s_A_CHANGE(stdlib_call("re", "compile", &[py_str("\\b(?:instead|rather than|without|swap\\w*|switch\\w*|replace\\w*|substitut\\w*|veggie|vegetarian|vegan|gluten|dairy|free from|omitted|skipped|cut out|left out|leave out|made it with|no [a-z]+)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.QUANTITY"); }
    s_QUANTITY(py_str("(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|\\d+)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.WHOSE"); }
    s_WHOSE(py_str("(?:of us|people|guests?|kids?|portions?|slices|servings?|adults?|children)"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET memory.stands.A_SIZE_CHANGE"); }
    s_A_SIZE_CHANGE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:bigger|larger|smaller|double|doubled|halve|halved|batch|twice as|half as)\\b|\\b(?:more|extra|less|fewer)\\s+"), &g_WHOSE()), &py_str("\\b|\\bfor\\s+")), &g_QUANTITY()), &py_str("\\b|\\bfor\\s+(?:\\w+\\s+)?")), &g_WHOSE()), &py_str("\\b|\\b(?:feed|feeding|cooking for)\\s+(?:")), &g_QUANTITY()), &py_str("|")), &g_WHOSE()), &py_str(")\\b|\\b")), &g_QUANTITY()), &py_str("\\s*")), &g_WHOSE()), &py_str("\\b|\\b(?:there will be|there are|its|it's|so its|so it's|now)\\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|\\d+)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _number(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut word: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&mcall(&mcall(&word.clone(), "replace", &[py_str(","), py_str("")]), "replace", &[py_str("."), py_str("")]), "isdigit", &[])) {
    return call_builtin("float", &[mcall(&word.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    }
    return if py_truthy(&Py::Bool(py_contains(&g_NUMBERS(), &word.clone()))) { call_builtin("float", &[geti(&g_NUMBERS(), &word.clone())]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    Py::None
}
pub fn _figures(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("[(value, unit or \"\", where it starts, where it ends)] for the figures in a line.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_FIGURE(), "finditer", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    match_ = __item.clone();
    value = _number(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    continue;
    }
    unit = mcall(&mcall(&{ let __b = mcall(&match_.clone(), "group", &[Py::Int(2)]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "rstrip", &[py_str("s")]);
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![value.clone(), unit.clone(), mcall(&match_.clone(), "start", &[]), mcall(&match_.clone(), "end", &[])])]);
    }
    return found.clone();
    Py::None
}
pub fn _names_the_figure(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut _u: Py = Py::None;
    let mut other: Py = Py::None;
    let mut other_unit: Py = Py::None;
    let _ = py_str("True when this text gives the same figure, with the same unit where the line names one.\n\n    A line that says \"nine hours\" is not answered by an answer's bare \"9\" (\"9 to 10 for hard\n    boiled\"): the unit is what a figure means, and matching the number alone would confirm a\n    length the chat never gave.\n    ");
    let __iter1 = py_iter_or_raise(&_figures(&[text.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    other = __unpk2[0].clone();
    other_unit = __unpk2[1].clone();
    _u = __unpk2[2].clone();
    _u = __unpk2[3].clone();
    if py_truthy(&Py::Bool(!py_eq(&other.clone(), &value.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = unit.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&unit.clone(), &other_unit.clone())) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_line_asks_about_a_figure_and_elsewhere_nothing(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut start: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The (value, unit) the line asks about, or None when it names something of its own.");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = mcall(&g_QUESTION_OPENING(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKING(), "search", &[line.clone()]) } })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKING(), "search", &[line.clone()]))) } }) {
    return Py::None;
    }
    figures = _figures(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[figures.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&geti(&figures.clone(), &Py::Int(0)), 4, None);
    value = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    start = __unpk1[2].clone();
    end = __unpk1[3].clone();
    rest = mcall(&op_add(&op_add(&slice_or_raise(&line.clone(), &Py::None, &start.clone(), &Py::None), &py_str(" ")), &slice_or_raise(&line.clone(), &end.clone(), &Py::None, &Py::None)), "split", &[]);
    let __iter2 = py_iter_or_raise(&rest.clone());
    for __item in __iter2 {
    word = __item.clone();
    word = mcall(&word.clone(), "strip", &[py_str(".,!?;:")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOTHING_NAMED(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_UNITS(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("per"), py_str("each"), py_str("every")]), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_NOTHING_NAMED(), &mcall(&word.clone(), "rstrip", &[py_str("'s")])))) {
    continue;
    }
    return Py::None;
    }
    return py_tuple(vec![value.clone(), unit.clone(), mcall(&slice_or_raise(&line.clone(), &start.clone(), &end.clone(), &Py::None), "strip", &[])]);
    Py::None
}
pub fn _the_chat_gave_it(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when one of the bot's own recent answers gives this figure with this unit.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&g_LOOK_BACK()), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(_names_the_figure(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), value.clone(), unit.clone()])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_reason_behind(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The bot's own sentence that gives this figure and says what it rests on, or \"\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&g_LOOK_BACK()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_the_figure(&[said.clone(), value.clone(), unit.clone()]))) } }) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_SENTENCE(), "split", &[said.clone()]));
    for __item in __iter2 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    words = mcall(&sentence.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp31: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&g_SHORTEST_REASON(), &__cmp31)) { op_le(&__cmp31, &g_MOST_REASON_WORDS()) } else { Py::Bool(false) } }))) {
    continue;
    }
    if py_truthy(&_names_the_figure(&[sentence.clone(), value.clone(), unit.clone()])) {
    return sentence.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn the_figure_asked_about(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("(value, unit, phrase, reason) when the line asks whether a figure of the chat stands.\n\n    `reason` is \"\" when the chat gave the figure and never said what it rests on: the answer is\n    then the figure itself, said back as the thing that has not changed.\n    ");
    asked = _the_line_asks_about_a_figure_and_elsewhere_nothing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&asked.clone(), 3, None);
    value = __unpk1[0].clone();
    unit = __unpk1[1].clone();
    phrase = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_the_chat_gave_it(&[memory.clone(), value.clone(), unit.clone()])))) {
    return Py::None;
    }
    return py_tuple(vec![value.clone(), unit.clone(), phrase.clone(), _the_reason_behind(&[memory.clone(), value.clone(), unit.clone()])]);
    Py::None
}
pub fn _in_minutes(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut value: Py = Py::None;
    let _ = py_str("A time figure as a number of minutes, so \"1 hour 40 minutes\" beats \"20 minutes\".");
    value = _number(&[number.clone()]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    return Py::Float(0.0);
    }
    unit = mcall(&mcall(&unit.clone(), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("hour"), py_str("hr")]), &unit.clone()))) {
    return op_mul(&value.clone(), &Py::Int(60));
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("day")))) {
    return op_mul(&value.clone(), &Py::Int(1440));
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("week")))) {
    return op_mul(&value.clone(), &Py::Int(10080));
    }
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("month")))) {
    return op_mul(&value.clone(), &Py::Int(43200));
    }
    return value.clone();
    Py::None
}
pub fn _the_best(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut candidates: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "candidates") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: candidates"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut best: Py = Py::None;
    let _ = py_str("The figure to hand back: the whole time rather than a rate, and the first temperature.");
    if py_truthy(&Py::Bool(!py_truthy(&candidates.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("temperature")))) {
    return geti(&geti(&candidates.clone(), &Py::Int(0)), &Py::Int(2));
    }
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidates.clone()); __kws.extend(vec![("key".to_string(), { let best = best.clone(); let candidates = candidates.clone(); let family = family.clone(); py_func("lambda", move |args: &[Py]| { let best = best.clone(); let candidates = candidates.clone(); let family = family.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); _in_minutes(&[geti(&mcall(&geti(&found.clone(), &Py::Int(0)), "split", &[]), &Py::Int(0)), geti(&found.clone(), &Py::Int(1))]) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return geti(&best.clone(), &Py::Int(2));
    Py::None
}
pub fn _figures_in_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut candidates: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The figures one answer gives for this attribute, as [(value, unit, as said)].");
    candidates = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_FIGURED(), "finditer", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&mcall(&match_.clone(), "group", &[Py::Int(1)])) {
    whole = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "lower", &[]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(3)]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&{ let __b = mcall(&match_.clone(), "group", &[Py::Int(4)]); if py_truthy(&__b) { __b } else { py_str("minutes") } }, "lower", &[]), -1, "")); py_string(__s) };
    minutes = op_add(&_in_minutes(&[mcall(&match_.clone(), "group", &[Py::Int(1)]), py_str("hour")]), &_in_minutes(&[mcall(&match_.clone(), "group", &[Py::Int(3)]), py_str("minute")]));
    if py_truthy(&Py::Bool(!py_eq(&family.clone(), &py_str("time")))) {
    continue;
    }
    let _ = mcall(&candidates.clone(), "append", &[py_tuple(vec![call_builtin("str", &[minutes.clone()]).unwrap_or_else(|e| exc::raise(e)), py_str("minute"), whole.clone()])]);
    continue;
    }
    unit = mcall(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(6)]), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&Py::Bool(!py_eq(&mcall(&g_UNIT_FAMILY(), "get", &[unit.clone()]), &family.clone()))) {
    continue;
    }
    figure = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(5)]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(6)]), "lower", &[]), -1, "")); py_string(__s) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("c"), py_str("°c")]), &unit.clone()))) {
    figure = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[Py::Int(5)]), -1, "")); __s.push_str("C"); py_string(__s) };
    }
    let _ = mcall(&candidates.clone(), "append", &[py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(5)]), mcall(&mcall(&match_.clone(), "group", &[Py::Int(6)]), "lower", &[]), figure.clone()])]);
    }
    return candidates.clone();
    Py::None
}
pub fn _the_same_figure(args: &[Py]) -> Py {
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
    let _ = py_str("True when two figures are the same one: \"12 to 15 minutes\" and \"12-15 minutes\".");
    return Py::Bool(py_eq(&stdlib_call("re", "sub", &[py_str("\\D"), py_str(""), call_builtin("str", &[{ let __b = one.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &stdlib_call("re", "sub", &[py_str("\\D"), py_str(""), call_builtin("str", &[{ let __b = other.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _subjects(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]{4,}"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_NOT_THE_SUBJECT(), &word.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
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
    let _ = py_str("True when two answers are about the same thing, so a figure in the second is this dish's.");
    return call_builtin("bool", &[op_bitand(&_subjects(&[one.clone()]), &_subjects(&[other.clone()]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_figure_for(args: &[Py]) -> Py {
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
    let mut answers: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut swapped: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("(figure, moved) - the figure to hand back, and whether the swap moved it.\n\n    The swap is the last thing the person said, so the newest answer is the answer to it, and the\n    one before is the answer that set the plan going. Where they give the same figure, or the\n    answer to the swap gives none, the plan's figure holds. Where they give different figures about\n    the same thing - \"im making rice\" then \"im using brown rice instead\", 10 to 12 minutes against\n    25 to 30 - the swap did move it, and handing back the new figure while saying \"same as before\"\n    would have a person cook raw rice. Where the second answer is a card about something else (\"im\n    roasting a chicken\" / \"theres no foil\" answered with a dhal) its figure is not this dish's, and\n    the reader says nothing rather than guess.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    answers = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&answers.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    plan = _the_best(&[_figures_in_answer(&[geti(&answers.clone(), &Py::Int(0)), family.clone()]), family.clone()]);
    swapped = if py_truthy(&op_gt(&call_builtin("len", &[answers.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { _the_best(&[_figures_in_answer(&[geti(&answers.clone(), &Py::Int(1)), family.clone()]), family.clone()]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&swapped.clone()))) {
    return py_tuple(vec![plan.clone(), Py::Bool(false)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&plan.clone()))) {
    return py_tuple(vec![swapped.clone(), Py::Bool(true)]);
    }
    if py_truthy(&_the_same_figure(&[swapped.clone(), plan.clone()])) {
    return py_tuple(vec![plan.clone(), Py::Bool(false)]);
    }
    if py_truthy(&_about_the_same_thing(&[geti(&answers.clone(), &Py::Int(0)), geti(&answers.clone(), &Py::Int(1))])) {
    return py_tuple(vec![swapped.clone(), Py::Bool(true)]);
    }
    said = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    if py_truthy(&mcall(&g_A_METHOD_CHANGE(), "search", &[said.clone()])) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    return py_tuple(vec![plan.clone(), Py::Bool(false)]);
    Py::None
}
pub fn _a_change_was_made(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when one of the last few lines of theirs swaps something for something else.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&said.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut line: Py = __item.clone(); __out.push(mcall(&g_A_CHANGE(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_change_of_size(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("True when the change the person just made was a size or a quantity.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&said.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut line: Py = __item.clone(); __out.push(mcall(&g_A_SIZE_CHANGE(), "search", &[line.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_change_asked_about(args: &[Py]) -> Py {
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
    let mut attribute: Py = Py::None;
    let mut family: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut line: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word_: Py = Py::None;
    let _ = py_str("(attribute, figure, moved) when the line asks whether a change moves a figure, else None.\n\n    Narrow on purpose: the line has to ask with does/will/would, has to name the attribute with\n    \"the\"/\"my\" in front of it, has to be about nothing else, and the chat has to hold a figure for\n    that attribute in one of its own answers. A line about a machine's clock is a setting.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &op_add(&g_MOST_WORDS(), &Py::Int(3))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_CHANGE_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_A_SETTING(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _a_change_of_size(&[memory.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_change_was_made(&[memory.clone()]))) } }) {
    return Py::None;
    }
    named = mcall(&g_ATTRIBUTE_WORDS(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    word = mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("time"), py_str("cooking time"), py_str("oven time"), py_str("long")]), &word.clone()))) {
    let __unpk1 = py_unpack(&py_tuple(vec![py_str("time"), py_str("time")]), 2, None);
    attribute = __unpk1[0].clone();
    family = __unpk1[1].clone();
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("temperature"), py_str("heat")]), &word.clone()))) {
    let __unpk2 = py_unpack(&py_tuple(vec![py_str("temperature"), py_str("temperature")]), 2, None);
    attribute = __unpk2[0].clone();
    family = __unpk2[1].clone();
    } else {
    return Py::None;
    }
    }
    rest = op_add(&op_add(&slice_or_raise(&line.clone(), &Py::None, &mcall(&named.clone(), "start", &[Py::Int(1)]), &Py::None), &py_str(" ")), &slice_or_raise(&line.clone(), &mcall(&named.clone(), "end", &[Py::Int(1)]), &Py::None, &Py::None));
    let __iter3 = py_iter_or_raise(&mcall(&rest.clone(), "split", &[]));
    for __item in __iter3 {
    word_ = __item.clone();
    word_ = mcall(&word_.clone(), "strip", &[py_str(".,!?;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word_.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOTHING_NAMED(), &word_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("that"), py_str("this"), py_str("it"), py_str("which")]), &word_.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("change"), py_str("changed"), py_str("changes"), py_str("affect"), py_str("affects"), py_str("alter"), py_str("alters"), py_str("impact"), py_str("matter"), py_str("difference"), py_str("make"), py_str("makes"), py_str("made"), py_str("a"), py_str("an"), py_str("any"), py_str("to"), py_str("for"), py_str("the"), py_str("on"), py_str("of"), py_str("in"), py_str("with"), py_str("much"), py_str("long"), py_str("time"), py_str("cooking"), py_str("oven")]), &word_.clone()))) {
    continue;
    }
    return Py::None;
    }
    let __unpk4 = py_unpack(&_a_figure_for(&[memory.clone(), family.clone()]), 2, None);
    figure = __unpk4[0].clone();
    moved = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&figure.clone()))) {
    return Py::None;
    }
    return py_tuple(vec![attribute.clone(), figure.clone(), moved.clone()]);
    Py::None
}
pub fn change_reply(args: &[Py]) -> Py {
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
    let mut attribute: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut moved: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pools: Py = Py::None;
    let _ = py_str("The reply to \"does that change the time?\" - the chat's own figure, handed back.");
    found = the_change_asked_about(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 3, None);
    attribute = __unpk1[0].clone();
    figure = __unpk1[1].clone();
    moved = __unpk1[2].clone();
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    pools = if py_truthy(&moved.clone()) { g_CHANGE_REPLIES_MOVED() } else { g_CHANGE_REPLIES() };
    pool = { let __b = mcall(&pools.clone(), "get", &[attribute.clone()]); if py_truthy(&__b) { __b } else { geti(&pools.clone(), &py_str("time")) } };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), figure.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[pool.clone(), memory.clone(), line.clone()]), "format", args))(&__args) };
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
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reason: Py = Py::None;
    let _ = py_str("The reply to a yes/no about a figure the bot gave, or None for every other line.");
    found = the_figure_asked_about(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 4, None);
    _u = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    phrase = __unpk1[2].clone();
    reason = __unpk1[3].clone();
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&reason.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("figure".to_string(), phrase.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_CONFIRMS(), memory.clone(), line.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reason".to_string(), reason.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pyrt::xrefs::xcall("compose", "pick", &[g_STANDS(), memory.clone(), line.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.stands".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASKING" => Some(g_ASKING()),
            "ATTRIBUTES" => Some(g_ATTRIBUTES()),
            "ATTRIBUTE_WORDS" => Some(g_ATTRIBUTE_WORDS()),
            "A_CHANGE" => Some(g_A_CHANGE()),
            "A_METHOD_CHANGE" => Some(g_A_METHOD_CHANGE()),
            "A_SETTING" => Some(g_A_SETTING()),
            "A_SIZE_CHANGE" => Some(g_A_SIZE_CHANGE()),
            "CHANGE_ASK" => Some(g_CHANGE_ASK()),
            "CHANGE_REPLIES" => Some(g_CHANGE_REPLIES()),
            "CHANGE_REPLIES_MOVED" => Some(g_CHANGE_REPLIES_MOVED()),
            "CONFIRMS" => Some(g_CONFIRMS()),
            "FIGURE" => Some(g_FIGURE()),
            "FIGURED" => Some(g_FIGURED()),
            "LOOK_BACK" => Some(g_LOOK_BACK()),
            "MOST_REASON_WORDS" => Some(g_MOST_REASON_WORDS()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "NOTHING_NAMED" => Some(g_NOTHING_NAMED()),
            "NOT_THE_SUBJECT" => Some(g_NOT_THE_SUBJECT()),
            "NUMBERS" => Some(g_NUMBERS()),
            "QUANTITY" => Some(g_QUANTITY()),
            "QUESTION_OPENING" => Some(g_QUESTION_OPENING()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SHORTEST_REASON" => Some(g_SHORTEST_REASON()),
            "STANDS" => Some(g_STANDS()),
            "UNITS" => Some(g_UNITS()),
            "UNIT_FAMILY" => Some(g_UNIT_FAMILY()),
            "WHOSE" => Some(g_WHOSE()),
            "_a_change_of_size" => Some(py_func("_a_change_of_size", |args: &[Py]| _a_change_of_size(args))),
            "_a_change_was_made" => Some(py_func("_a_change_was_made", |args: &[Py]| _a_change_was_made(args))),
            "_a_figure_for" => Some(py_func("_a_figure_for", |args: &[Py]| _a_figure_for(args))),
            "_about_the_same_thing" => Some(py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args))),
            "_figures" => Some(py_func("_figures", |args: &[Py]| _figures(args))),
            "_figures_in_answer" => Some(py_func("_figures_in_answer", |args: &[Py]| _figures_in_answer(args))),
            "_in_minutes" => Some(py_func("_in_minutes", |args: &[Py]| _in_minutes(args))),
            "_names_the_figure" => Some(py_func("_names_the_figure", |args: &[Py]| _names_the_figure(args))),
            "_number" => Some(py_func("_number", |args: &[Py]| _number(args))),
            "_subjects" => Some(py_func("_subjects", |args: &[Py]| _subjects(args))),
            "_the_best" => Some(py_func("_the_best", |args: &[Py]| _the_best(args))),
            "_the_chat_gave_it" => Some(py_func("_the_chat_gave_it", |args: &[Py]| _the_chat_gave_it(args))),
            "_the_line_asks_about_a_figure_and_elsewhere_nothing" => Some(py_func("_the_line_asks_about_a_figure_and_elsewhere_nothing", |args: &[Py]| _the_line_asks_about_a_figure_and_elsewhere_nothing(args))),
            "_the_reason_behind" => Some(py_func("_the_reason_behind", |args: &[Py]| _the_reason_behind(args))),
            "_the_same_figure" => Some(py_func("_the_same_figure", |args: &[Py]| _the_same_figure(args))),
            "change_reply" => Some(py_func("change_reply", |args: &[Py]| change_reply(args))),
            "pick" => Some(g_pick()),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "the_change_asked_about" => Some(py_func("the_change_asked_about", |args: &[Py]| the_change_asked_about(args))),
            "the_figure_asked_about" => Some(py_func("the_figure_asked_about", |args: &[Py]| the_figure_asked_about(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASKING" => s_ASKING(v),
            "ATTRIBUTES" => s_ATTRIBUTES(v),
            "ATTRIBUTE_WORDS" => s_ATTRIBUTE_WORDS(v),
            "A_CHANGE" => s_A_CHANGE(v),
            "A_METHOD_CHANGE" => s_A_METHOD_CHANGE(v),
            "A_SETTING" => s_A_SETTING(v),
            "A_SIZE_CHANGE" => s_A_SIZE_CHANGE(v),
            "CHANGE_ASK" => s_CHANGE_ASK(v),
            "CHANGE_REPLIES" => s_CHANGE_REPLIES(v),
            "CHANGE_REPLIES_MOVED" => s_CHANGE_REPLIES_MOVED(v),
            "CONFIRMS" => s_CONFIRMS(v),
            "FIGURE" => s_FIGURE(v),
            "FIGURED" => s_FIGURED(v),
            "LOOK_BACK" => s_LOOK_BACK(v),
            "MOST_REASON_WORDS" => s_MOST_REASON_WORDS(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "NOTHING_NAMED" => s_NOTHING_NAMED(v),
            "NOT_THE_SUBJECT" => s_NOT_THE_SUBJECT(v),
            "NUMBERS" => s_NUMBERS(v),
            "QUANTITY" => s_QUANTITY(v),
            "QUESTION_OPENING" => s_QUESTION_OPENING(v),
            "SENTENCE" => s_SENTENCE(v),
            "SHORTEST_REASON" => s_SHORTEST_REASON(v),
            "STANDS" => s_STANDS(v),
            "UNITS" => s_UNITS(v),
            "UNIT_FAMILY" => s_UNIT_FAMILY(v),
            "WHOSE" => s_WHOSE(v),
            "pick" => s_pick(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.stands", module_obj);
    pyrt::xrefs::reg_fn("memory.stands", "_number", py_func("_number", |args: &[Py]| _number(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_figures", py_func("_figures", |args: &[Py]| _figures(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_names_the_figure", py_func("_names_the_figure", |args: &[Py]| _names_the_figure(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_the_line_asks_about_a_figure_and_elsewhere_nothing", py_func("_the_line_asks_about_a_figure_and_elsewhere_nothing", |args: &[Py]| _the_line_asks_about_a_figure_and_elsewhere_nothing(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_the_chat_gave_it", py_func("_the_chat_gave_it", |args: &[Py]| _the_chat_gave_it(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_the_reason_behind", py_func("_the_reason_behind", |args: &[Py]| _the_reason_behind(args)));
    pyrt::xrefs::reg_fn("memory.stands", "the_figure_asked_about", py_func("the_figure_asked_about", |args: &[Py]| the_figure_asked_about(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_in_minutes", py_func("_in_minutes", |args: &[Py]| _in_minutes(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_the_best", py_func("_the_best", |args: &[Py]| _the_best(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_figures_in_answer", py_func("_figures_in_answer", |args: &[Py]| _figures_in_answer(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_the_same_figure", py_func("_the_same_figure", |args: &[Py]| _the_same_figure(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_subjects", py_func("_subjects", |args: &[Py]| _subjects(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_about_the_same_thing", py_func("_about_the_same_thing", |args: &[Py]| _about_the_same_thing(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_a_figure_for", py_func("_a_figure_for", |args: &[Py]| _a_figure_for(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_a_change_was_made", py_func("_a_change_was_made", |args: &[Py]| _a_change_was_made(args)));
    pyrt::xrefs::reg_fn("memory.stands", "_a_change_of_size", py_func("_a_change_of_size", |args: &[Py]| _a_change_of_size(args)));
    pyrt::xrefs::reg_fn("memory.stands", "the_change_asked_about", py_func("the_change_asked_about", |args: &[Py]| the_change_asked_about(args)));
    pyrt::xrefs::reg_fn("memory.stands", "change_reply", py_func("change_reply", |args: &[Py]| change_reply(args)));
    pyrt::xrefs::reg_fn("memory.stands", "reply_for", py_func("reply_for", |args: &[Py]| reply_for(args)));
    pyrt::xrefs::reg_access("memory.stands", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.stands", "ATTRIBUTES", g_ATTRIBUTES, s_ATTRIBUTES);
    pyrt::xrefs::reg_access("memory.stands", "ATTRIBUTE_WORDS", g_ATTRIBUTE_WORDS, s_ATTRIBUTE_WORDS);
    pyrt::xrefs::reg_access("memory.stands", "A_CHANGE", g_A_CHANGE, s_A_CHANGE);
    pyrt::xrefs::reg_access("memory.stands", "A_METHOD_CHANGE", g_A_METHOD_CHANGE, s_A_METHOD_CHANGE);
    pyrt::xrefs::reg_access("memory.stands", "A_SETTING", g_A_SETTING, s_A_SETTING);
    pyrt::xrefs::reg_access("memory.stands", "A_SIZE_CHANGE", g_A_SIZE_CHANGE, s_A_SIZE_CHANGE);
    pyrt::xrefs::reg_access("memory.stands", "CHANGE_ASK", g_CHANGE_ASK, s_CHANGE_ASK);
    pyrt::xrefs::reg_access("memory.stands", "CHANGE_REPLIES", g_CHANGE_REPLIES, s_CHANGE_REPLIES);
    pyrt::xrefs::reg_access("memory.stands", "CHANGE_REPLIES_MOVED", g_CHANGE_REPLIES_MOVED, s_CHANGE_REPLIES_MOVED);
    pyrt::xrefs::reg_access("memory.stands", "CONFIRMS", g_CONFIRMS, s_CONFIRMS);
    pyrt::xrefs::reg_access("memory.stands", "FIGURE", g_FIGURE, s_FIGURE);
    pyrt::xrefs::reg_access("memory.stands", "FIGURED", g_FIGURED, s_FIGURED);
    pyrt::xrefs::reg_access("memory.stands", "LOOK_BACK", g_LOOK_BACK, s_LOOK_BACK);
    pyrt::xrefs::reg_access("memory.stands", "MOST_REASON_WORDS", g_MOST_REASON_WORDS, s_MOST_REASON_WORDS);
    pyrt::xrefs::reg_access("memory.stands", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.stands", "NOTHING_NAMED", g_NOTHING_NAMED, s_NOTHING_NAMED);
    pyrt::xrefs::reg_access("memory.stands", "NOT_THE_SUBJECT", g_NOT_THE_SUBJECT, s_NOT_THE_SUBJECT);
    pyrt::xrefs::reg_access("memory.stands", "NUMBERS", g_NUMBERS, s_NUMBERS);
    pyrt::xrefs::reg_access("memory.stands", "QUANTITY", g_QUANTITY, s_QUANTITY);
    pyrt::xrefs::reg_access("memory.stands", "QUESTION_OPENING", g_QUESTION_OPENING, s_QUESTION_OPENING);
    pyrt::xrefs::reg_access("memory.stands", "SENTENCE", g_SENTENCE, s_SENTENCE);
    pyrt::xrefs::reg_access("memory.stands", "SHORTEST_REASON", g_SHORTEST_REASON, s_SHORTEST_REASON);
    pyrt::xrefs::reg_access("memory.stands", "STANDS", g_STANDS, s_STANDS);
    pyrt::xrefs::reg_access("memory.stands", "UNITS", g_UNITS, s_UNITS);
    pyrt::xrefs::reg_access("memory.stands", "UNIT_FAMILY", g_UNIT_FAMILY, s_UNIT_FAMILY);
    pyrt::xrefs::reg_access("memory.stands", "WHOSE", g_WHOSE, s_WHOSE);
    pyrt::xrefs::reg_access("memory.stands", "pick", g_pick, s_pick);
}

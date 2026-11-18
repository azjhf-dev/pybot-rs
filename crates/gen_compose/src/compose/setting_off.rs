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


thread_local! { static G_ASK_LATEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DESTINATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_GIVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEEDS_THE_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_JOURNEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTING_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SET_OFF_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEBODY_ELSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_OWN_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIR_TIMETABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HOUR_IS_FIXED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HOUR_IS_KNOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_HOUR_THEY_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ASK_LATEST() -> Py { ensure_init(); G_ASK_LATEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_LATEST(v: Py) { ensure_init(); G_ASK_LATEST.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_SHAPE() -> Py { ensure_init(); G_ASK_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_SHAPE(v: Py) { ensure_init(); G_ASK_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DESTINATION() -> Py { ensure_init(); G_A_DESTINATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DESTINATION(v: Py) { ensure_init(); G_A_DESTINATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_NAMED() -> Py { ensure_init(); G_A_PLACE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_NAMED(v: Py) { ensure_init(); G_A_PLACE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_GIVEN() -> Py { ensure_init(); G_A_TIME_GIVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_GIVEN(v: Py) { ensure_init(); G_A_TIME_GIVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON() -> Py { ensure_init(); G_FIRST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON(v: Py) { ensure_init(); G_FIRST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_NEEDS_THE_HOUR() -> Py { ensure_init(); G_NEEDS_THE_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEEDS_THE_HOUR(v: Py) { ensure_init(); G_NEEDS_THE_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_JOURNEY() -> Py { ensure_init(); G_NOT_A_JOURNEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_JOURNEY(v: Py) { ensure_init(); G_NOT_A_JOURNEY.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTERS() -> Py { ensure_init(); G_POINTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTERS(v: Py) { ensure_init(); G_POINTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTING_OFF() -> Py { ensure_init(); G_SETTING_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTING_OFF(v: Py) { ensure_init(); G_SETTING_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_SET_OFF_A_THING() -> Py { ensure_init(); G_SET_OFF_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SET_OFF_A_THING(v: Py) { ensure_init(); G_SET_OFF_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEBODY_ELSE() -> Py { ensure_init(); G_SOMEBODY_ELSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY_ELSE(v: Py) { ensure_init(); G_SOMEBODY_ELSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_OWN_CHOICE() -> Py { ensure_init(); G_THEIR_OWN_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_CHOICE(v: Py) { ensure_init(); G_THEIR_OWN_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIR_TIMETABLE() -> Py { ensure_init(); G_THEIR_TIMETABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIR_TIMETABLE(v: Py) { ensure_init(); G_THEIR_TIMETABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HOUR_IS_FIXED() -> Py { ensure_init(); G_THE_HOUR_IS_FIXED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HOUR_IS_FIXED(v: Py) { ensure_init(); G_THE_HOUR_IS_FIXED.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HOUR_IS_KNOWN() -> Py { ensure_init(); G_THE_HOUR_IS_KNOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HOUR_IS_KNOWN(v: Py) { ensure_init(); G_THE_HOUR_IS_KNOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_HOUR_THEY_SAID() -> Py { ensure_init(); G_THE_HOUR_THEY_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_HOUR_THEY_SAID(v: Py) { ensure_init(); G_THE_HOUR_THEY_SAID.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("What time to set off: the composer names where to look, and never how the time is worked out.\n\n\"What time should i leave\", \"how early should i get there\" and \"when should i leave for the station\"\nreached the search hedge - \"That's beyond what I know well, sorry. Searching for it will turn up\nbetter sources than I can offer\" - and \"what time do i need to leave\" got the live-timetable line:\n\"I can't check live times, sorry. The operator's app (or the board at the stop) is the one that\nknows\". Both name where to look and neither says how the answer is worked out, which is the part\nthat can be said: the hour they have to be there, minus the journey, minus the finding-the-door.\n\nSo this answers with the method, says in a few words what it cannot see (today's traffic, the\ntimetable) and where that is (a planner, the operator's app, on the morning), and asks for the one\nthing only the person knows when the line does not carry it. Where the hour is already in the line\nit finishes the sum with them instead of asking for it again.\n\nThe fences are the entries that already answer and the lines that are not a journey. An airport, a\nflight, a train, a cruise, an interview and an appointment each have a card with their own figure,\nand those answer the composer before this reader is reached, so they keep their turn. Leaving a\nperson, a job or a country is a different decision (\"when should i leave him\", \"should i leave my\njob\"), leaving the light on or a message is not a departure, and somebody else's timetable (\"what\ntime does the bus leave\") is the operator's. A statement with the same words in it is not an ask\n(\"i will call you when i get there\"), so the question has to be a question: the asking word and\nthen the verb and then them - \"what time should i leave\", \"when do we need to set off\".\n");
    s_ASK_SHAPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:(?:what|which)\\s+time|when|how\\s+early|how\\s+late)\\s+(?:should|shall|do|does|did|will|would|must|can|could|need|have|am|are|is)\\b|(?<![a-z])(?:early|late)\\s+enough\\b|(?<![a-z])(?:best|better|ideal|sensible|wisest)\\s+time\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_LATEST({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:(?:what'?s|what\\s+is|when'?s|which\\s+is)\\s+the\\s+latest|latest\\s+(?:time\\s+)?(?:i|we)\\s+(?:can|could|should|would)\\b)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FIRST_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:i|we|us|im|i'm|ive|i've|ill|i'll|well|we'll)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_OWN_CHOICE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:best|better|ideal|sensible|wisest|reasonable|(?:early|late)\\s+enough)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEBODY_ELSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:he|she|they|him|her|hers|his|them|their|mum|dad|mother|father|son|daughter|brother|sister|partner|husband|wife|friend|boss|colleague)(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SETTING_OFF(py_tuple(vec![py_str("leave"), py_str("leaves"), py_str("leaving"), py_str("set off"), py_str("setting off"), py_str("set out"), py_str("setting out"), py_str("head off"), py_str("heading off"), py_str("get going"), py_str("getting going"), py_str("start out"), py_str("starting out"), py_str("get there"), py_str("getting there"), py_str("be there"), py_str("arrive"), py_str("arriving"), py_str("get to the"), py_str("get to my"), py_str("get to work"), py_str("getting to the"), py_str("getting to my"), py_str("getting to work")]));
    s_NOT_A_JOURNEY(py_tuple(vec![py_str("leave him"), py_str("leave her"), py_str("leave them"), py_str("leave my husband"), py_str("leave my wife"), py_str("leave my partner"), py_str("leave my boyfriend"), py_str("leave my girlfriend"), py_str("leave my job"), py_str("leave my marriage"), py_str("leave the country"), py_str("leave home for good"), py_str("leave work"), py_str("leave school"), py_str("leave college"), py_str("leave uni"), py_str("leave the army"), py_str("leave it"), py_str("leave that"), py_str("leave the light"), py_str("leave the lights"), py_str("leave the heating"), py_str("leave the oven"), py_str("leave the door"), py_str("leave the window"), py_str("leave the keys"), py_str("leave the car"), py_str("leave a message"), py_str("leave a review"), py_str("leave a tip"), py_str("leave feedback"), py_str("leave a comment"), py_str("leave the dishes"), py_str("leave the washing"), py_str("leave for good"), py_str("leave the bin"), py_str("leave the bins"), py_str("leave the rubbish"), py_str("leave the dog"), py_str("leave the cat"), py_str("leave the kids"), py_str("leave the baby"), py_str("leave me"), py_str("leave us"), py_str("leave time"), py_str("leave it to")]));
    s_SET_OFF_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("set(?:ting)?\\s+off\\s+(?:the|a|an|my|our)\\s+(?:alarm|alarms|firework|fireworks|sprinkler|sprinklers|flare|detector|trigger|siren|airbag|extinguisher|smoke|bomb)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEIR_TIMETABLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:does|do|did|will|would)\\s+(?:the|my|our|your|his|her|their|this|that)\\s+(?:bus|buses|train|trains|tram|trams|tube|ferry|boat|ship|plane|flight|flights|coach|coaches|match|game|film|show|class|lesson|appointment|shift|service|services)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TIME_GIVEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z0-9])(?:at|by|for|before|until|till)\\s+\\d{1,2}(?::\\d\\d)?(?:\\s*(?:am|pm|a\\.m|p\\.m))?(?![a-z0-9])|(?<![a-z0-9])\\d{1,2}(?::\\d\\d)?\\s*(?:am|pm|a\\.m|p\\.m)(?![a-z])")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THE_HOUR_IS_FIXED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:have\\s+to\\s+be|need\\s+to\\s+be|got\\s+to\\s+be|due|starts?|begins?|booked|meeting|appointment|interview|flight|ceremony|table|class|shift|lesson|course|visit|delivery|viewing)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DESTINATION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<![a-z])(?:to|for|into|towards|toward)\\s+(?:the\\s+|a\\s+|an\\s+|my\\s+|our\\s+)?(?!be\\b|do\\b|go\\b|get\\b|set\\b|head\\b|leave\\b|arrive\\b|make\\b|see\\b|take\\b|find\\b|know\\b|come\\b|there\\b|here\\b|it\\b|them\\b|him\\b|her\\b|us\\b|me\\b|you\\b)[a-z][a-z'\\-]*(?![a-z])")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTERS(py_tuple(vec![py_str("searching for it will turn up"), py_str("a search should sort you out"), py_str("i don't know enough to answer that"), py_str("that's beyond what i know"), py_str("can't check live times"), py_str("timetables and listings change by the hour"), py_str("the operator's app"), py_str("the operator's or the venue's own app"), py_str("times like that i can't see"), py_str("put the journey into"), py_str("i'd rather not guess"), py_str("a quick search online will get you"), py_str("that was the whole of my answer"), py_str("that is all of what i know on it"), py_str("that is everything i have on that one"), py_str("that's the lot i have on that one"), py_str("that's most of what i know on it"), py_str("no strong view on that one")]));
    s_NEEDS_THE_HOUR(py_tuple(vec![py_str("Work back from the hour you have to be there, not forward from now: a journey planner for that time of day gives the journey, then ten minutes on top for parking and finding the door, and that is when to set off. I can't see today's traffic or the timetables from here - the planner or the operator's app has them, live on the morning. What time do you have to be there?"), py_str("The hour you have to be there is the bit that decides it, and the journey a planner gives for that time of day is the rest, plus ten minutes for parking and getting in. Today's roads and delays are the one thing nothing here can see - check the planner just before you go. What time is it you need to be there?")]));
    s_THE_HOUR_IS_KNOWN(py_tuple(vec![py_str("Work back from that hour: a journey planner for that time of day gives the journey and the traffic so far, and ten minutes on top covers parking and the walk in. Look at the planner again just before you go, because that is when a closure or a cancellation is real."), py_str("Take the hour you have to be there, take the journey a planner gives for that time of day off it, and set off ten minutes earlier than that for parking and finding the door. The live part - an accident, a cancelled train - is the planner's or the operator's app's on the morning.")]));
    s_THE_HOUR_THEY_SAID(py_tuple(vec![py_str("You said %s, so that is the hour to work back from: a journey planner for that time of day gives the journey, ten minutes on top covers parking and the walk in, and that is when to set off. Look at the planner again on the morning, because that is when a closure or a delay is real."), py_str("Work back from %s: the planner's journey for that time of day, plus ten minutes for parking and finding the door, taken off it. The live part - a crash, a cancelled train - is only in the planner or the operator's app on the day.")]));
    s_A_PLACE_NAMED(py_tuple(vec![py_str("I can't see today's roads or trains from here, so the journey part has to come off a planner - but the working out is the same either way: the hour you have to be there, minus the journey for that time of day, minus ten minutes for parking and the walk in. Check the planner on the morning, when the traffic is real."), py_str("Nobody here knows what that journey takes today; a journey planner does, and it knows the traffic with it. Work back from the hour you have to be there, allow ten minutes for parking and getting in, and look at the planner again before you set off.")]));
}
pub fn _the_hour_already_said(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut hour: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The hour the person fixed a line or two back, with the preposition off it: \"9\", \"2.30pm\".");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THE_HOUR_IS_FIXED(), "search", args))(&__args) })) } }) {
    continue;
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME_GIVEN(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    hour = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:at|by|for|before|until|till)\\s+")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return hour.clone();
    }
    return py_str("");
    Py::None
}
pub fn is_a_pointer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reply: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("True when the composer's line names where to look rather than giving an answer.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_POINTERS()) { let mut shape: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &shape.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_it(args: &[Py]) -> Py {
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
    let mut fence: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("True for a question about what time the person should set off.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_SHAPE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_LATEST(), "search", args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FIRST_PERSON(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_OWN_CHOICE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEBODY_ELSE(), "search", args))(&__args) })) } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SETTING_OFF()) { let mut verb: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &verb.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_JOURNEY()) { let mut fence: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &fence.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SET_OFF_A_THING(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_THEIR_TIMETABLE(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _choose(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut turns: Py = Py::None;
    turns = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = py_str("");
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&turns.clone(), &op_neg(&Py::Int(1)))); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    last = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&pool.clone());
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&last.clone(), &slice_or_raise(&line.clone(), &Py::None, &Py::Int(30), &Py::None))) } }) {
    return line.clone();
    }
    }
    return geti(&pool.clone(), &Py::Int(0));
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
    else { Py::None };
    let mut hour: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The method for working out a leaving time, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_it(args))(&__args) }))) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TIME_GIVEN(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_THE_HOUR_IS_KNOWN()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    hour = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_hour_already_said(args))(&__args) };
    if py_truthy(&hour.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THE_HOUR_THEY_SAID()) { let mut line: Py = __item.clone(); __out.push(op_modulo(&line.clone(), &hour.clone())); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_DESTINATION(), "search", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_PLACE_NAMED()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_NEEDS_THE_HOUR()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _choose(args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.setting_off".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK_LATEST" => Some(g_ASK_LATEST()),
            "ASK_SHAPE" => Some(g_ASK_SHAPE()),
            "A_DESTINATION" => Some(g_A_DESTINATION()),
            "A_PLACE_NAMED" => Some(g_A_PLACE_NAMED()),
            "A_TIME_GIVEN" => Some(g_A_TIME_GIVEN()),
            "FIRST_PERSON" => Some(g_FIRST_PERSON()),
            "NEEDS_THE_HOUR" => Some(g_NEEDS_THE_HOUR()),
            "NOT_A_JOURNEY" => Some(g_NOT_A_JOURNEY()),
            "POINTERS" => Some(g_POINTERS()),
            "SETTING_OFF" => Some(g_SETTING_OFF()),
            "SET_OFF_A_THING" => Some(g_SET_OFF_A_THING()),
            "SOMEBODY_ELSE" => Some(g_SOMEBODY_ELSE()),
            "THEIR_OWN_CHOICE" => Some(g_THEIR_OWN_CHOICE()),
            "THEIR_TIMETABLE" => Some(g_THEIR_TIMETABLE()),
            "THE_HOUR_IS_FIXED" => Some(g_THE_HOUR_IS_FIXED()),
            "THE_HOUR_IS_KNOWN" => Some(g_THE_HOUR_IS_KNOWN()),
            "THE_HOUR_THEY_SAID" => Some(g_THE_HOUR_THEY_SAID()),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_the_hour_already_said" => Some(py_func("_the_hour_already_said", |args: &[Py]| _the_hour_already_said(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "is_a_pointer" => Some(py_func("is_a_pointer", |args: &[Py]| is_a_pointer(args))),
            "reads_it" => Some(py_func("reads_it", |args: &[Py]| reads_it(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ASK_LATEST" => s_ASK_LATEST(v),
            "ASK_SHAPE" => s_ASK_SHAPE(v),
            "A_DESTINATION" => s_A_DESTINATION(v),
            "A_PLACE_NAMED" => s_A_PLACE_NAMED(v),
            "A_TIME_GIVEN" => s_A_TIME_GIVEN(v),
            "FIRST_PERSON" => s_FIRST_PERSON(v),
            "NEEDS_THE_HOUR" => s_NEEDS_THE_HOUR(v),
            "NOT_A_JOURNEY" => s_NOT_A_JOURNEY(v),
            "POINTERS" => s_POINTERS(v),
            "SETTING_OFF" => s_SETTING_OFF(v),
            "SET_OFF_A_THING" => s_SET_OFF_A_THING(v),
            "SOMEBODY_ELSE" => s_SOMEBODY_ELSE(v),
            "THEIR_OWN_CHOICE" => s_THEIR_OWN_CHOICE(v),
            "THEIR_TIMETABLE" => s_THEIR_TIMETABLE(v),
            "THE_HOUR_IS_FIXED" => s_THE_HOUR_IS_FIXED(v),
            "THE_HOUR_IS_KNOWN" => s_THE_HOUR_IS_KNOWN(v),
            "THE_HOUR_THEY_SAID" => s_THE_HOUR_THEY_SAID(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.setting_off", module_obj);
    pyrt::xrefs::reg_fn("compose.setting_off", "_the_hour_already_said", py_func("_the_hour_already_said", |args: &[Py]| _the_hour_already_said(args)));
    pyrt::xrefs::reg_fn("compose.setting_off", "is_a_pointer", py_func("is_a_pointer", |args: &[Py]| is_a_pointer(args)));
    pyrt::xrefs::reg_fn("compose.setting_off", "reads_it", py_func("reads_it", |args: &[Py]| reads_it(args)));
    pyrt::xrefs::reg_fn("compose.setting_off", "_choose", py_func("_choose", |args: &[Py]| _choose(args)));
    pyrt::xrefs::reg_fn("compose.setting_off", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_access("compose.setting_off", "ASK_LATEST", g_ASK_LATEST, s_ASK_LATEST);
    pyrt::xrefs::reg_access("compose.setting_off", "ASK_SHAPE", g_ASK_SHAPE, s_ASK_SHAPE);
    pyrt::xrefs::reg_access("compose.setting_off", "A_DESTINATION", g_A_DESTINATION, s_A_DESTINATION);
    pyrt::xrefs::reg_access("compose.setting_off", "A_PLACE_NAMED", g_A_PLACE_NAMED, s_A_PLACE_NAMED);
    pyrt::xrefs::reg_access("compose.setting_off", "A_TIME_GIVEN", g_A_TIME_GIVEN, s_A_TIME_GIVEN);
    pyrt::xrefs::reg_access("compose.setting_off", "FIRST_PERSON", g_FIRST_PERSON, s_FIRST_PERSON);
    pyrt::xrefs::reg_access("compose.setting_off", "NEEDS_THE_HOUR", g_NEEDS_THE_HOUR, s_NEEDS_THE_HOUR);
    pyrt::xrefs::reg_access("compose.setting_off", "NOT_A_JOURNEY", g_NOT_A_JOURNEY, s_NOT_A_JOURNEY);
    pyrt::xrefs::reg_access("compose.setting_off", "POINTERS", g_POINTERS, s_POINTERS);
    pyrt::xrefs::reg_access("compose.setting_off", "SETTING_OFF", g_SETTING_OFF, s_SETTING_OFF);
    pyrt::xrefs::reg_access("compose.setting_off", "SET_OFF_A_THING", g_SET_OFF_A_THING, s_SET_OFF_A_THING);
    pyrt::xrefs::reg_access("compose.setting_off", "SOMEBODY_ELSE", g_SOMEBODY_ELSE, s_SOMEBODY_ELSE);
    pyrt::xrefs::reg_access("compose.setting_off", "THEIR_OWN_CHOICE", g_THEIR_OWN_CHOICE, s_THEIR_OWN_CHOICE);
    pyrt::xrefs::reg_access("compose.setting_off", "THEIR_TIMETABLE", g_THEIR_TIMETABLE, s_THEIR_TIMETABLE);
    pyrt::xrefs::reg_access("compose.setting_off", "THE_HOUR_IS_FIXED", g_THE_HOUR_IS_FIXED, s_THE_HOUR_IS_FIXED);
    pyrt::xrefs::reg_access("compose.setting_off", "THE_HOUR_IS_KNOWN", g_THE_HOUR_IS_KNOWN, s_THE_HOUR_IS_KNOWN);
    pyrt::xrefs::reg_access("compose.setting_off", "THE_HOUR_THEY_SAID", g_THE_HOUR_THEY_SAID, s_THE_HOUR_THEY_SAID);
}

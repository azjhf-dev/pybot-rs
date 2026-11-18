#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static AT_A_DOCTOR_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AT_A_PLACE_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AT_THE_GYM_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AT_WORK_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DAY_TO_COME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STRETCH_OF_TIME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BREAK_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_BED_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_THE_CAR_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVING_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLOW_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOFA_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AT_A_DOCTOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AT_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AT_THE_GYM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AT_WORK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HEAVY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IN_BED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _IN_THE_CAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MOVING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NOT_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ON_A_BREAK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SLOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SOFA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WAITING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AT_A_DOCTOR_REPLIES() -> Py { ensure_init(); AT_A_DOCTOR_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AT_A_DOCTOR_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = AT_A_DOCTOR_REPLIES.write() { *w = v; } }
pub fn g_AT_A_PLACE_REPLIES() -> Py { ensure_init(); AT_A_PLACE_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AT_A_PLACE_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = AT_A_PLACE_REPLIES.write() { *w = v; } }
pub fn g_AT_THE_GYM_REPLIES() -> Py { ensure_init(); AT_THE_GYM_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AT_THE_GYM_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = AT_THE_GYM_REPLIES.write() { *w = v; } }
pub fn g_AT_WORK_REPLIES() -> Py { ensure_init(); AT_WORK_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AT_WORK_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = AT_WORK_REPLIES.write() { *w = v; } }
pub fn g_A_DAY_TO_COME() -> Py { ensure_init(); A_DAY_TO_COME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DAY_TO_COME(v: Py) { ensure_init(); if let Ok(mut w) = A_DAY_TO_COME.write() { *w = v; } }
pub fn g_A_STRETCH_OF_TIME() -> Py { ensure_init(); A_STRETCH_OF_TIME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STRETCH_OF_TIME(v: Py) { ensure_init(); if let Ok(mut w) = A_STRETCH_OF_TIME.write() { *w = v; } }
pub fn g_BREAK_REPLIES() -> Py { ensure_init(); BREAK_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BREAK_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = BREAK_REPLIES.write() { *w = v; } }
pub fn g_IN_BED_REPLIES() -> Py { ensure_init(); IN_BED_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_BED_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = IN_BED_REPLIES.write() { *w = v; } }
pub fn g_IN_THE_CAR_REPLIES() -> Py { ensure_init(); IN_THE_CAR_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_THE_CAR_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = IN_THE_CAR_REPLIES.write() { *w = v; } }
pub fn g_MOVING_REPLIES() -> Py { ensure_init(); MOVING_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVING_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = MOVING_REPLIES.write() { *w = v; } }
pub fn g_SLOW_REPLIES() -> Py { ensure_init(); SLOW_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLOW_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SLOW_REPLIES.write() { *w = v; } }
pub fn g_SOFA_REPLIES() -> Py { ensure_init(); SOFA_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOFA_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = SOFA_REPLIES.write() { *w = v; } }
pub fn g_WAITING_REPLIES() -> Py { ensure_init(); WAITING_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_REPLIES.write() { *w = v; } }
pub fn g__AT_A_DOCTOR() -> Py { ensure_init(); _AT_A_DOCTOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AT_A_DOCTOR(v: Py) { ensure_init(); if let Ok(mut w) = _AT_A_DOCTOR.write() { *w = v; } }
pub fn g__AT_A_PLACE() -> Py { ensure_init(); _AT_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AT_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = _AT_A_PLACE.write() { *w = v; } }
pub fn g__AT_THE_GYM() -> Py { ensure_init(); _AT_THE_GYM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AT_THE_GYM(v: Py) { ensure_init(); if let Ok(mut w) = _AT_THE_GYM.write() { *w = v; } }
pub fn g__AT_WORK() -> Py { ensure_init(); _AT_WORK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AT_WORK(v: Py) { ensure_init(); if let Ok(mut w) = _AT_WORK.write() { *w = v; } }
pub fn g__END() -> Py { ensure_init(); _END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__END(v: Py) { ensure_init(); if let Ok(mut w) = _END.write() { *w = v; } }
pub fn g__HEAVY() -> Py { ensure_init(); _HEAVY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HEAVY(v: Py) { ensure_init(); if let Ok(mut w) = _HEAVY.write() { *w = v; } }
pub fn g__IN_BED() -> Py { ensure_init(); _IN_BED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IN_BED(v: Py) { ensure_init(); if let Ok(mut w) = _IN_BED.write() { *w = v; } }
pub fn g__IN_THE_CAR() -> Py { ensure_init(); _IN_THE_CAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__IN_THE_CAR(v: Py) { ensure_init(); if let Ok(mut w) = _IN_THE_CAR.write() { *w = v; } }
pub fn g__LEAD() -> Py { ensure_init(); _LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEAD(v: Py) { ensure_init(); if let Ok(mut w) = _LEAD.write() { *w = v; } }
pub fn g__MOVING() -> Py { ensure_init(); _MOVING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MOVING(v: Py) { ensure_init(); if let Ok(mut w) = _MOVING.write() { *w = v; } }
pub fn g__NOT_A_PLACE() -> Py { ensure_init(); _NOT_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = _NOT_A_PLACE.write() { *w = v; } }
pub fn g__ON_A_BREAK() -> Py { ensure_init(); _ON_A_BREAK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ON_A_BREAK(v: Py) { ensure_init(); if let Ok(mut w) = _ON_A_BREAK.write() { *w = v; } }
pub fn g__SLOW() -> Py { ensure_init(); _SLOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SLOW(v: Py) { ensure_init(); if let Ok(mut w) = _SLOW.write() { *w = v; } }
pub fn g__SOFA() -> Py { ensure_init(); _SOFA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SOFA(v: Py) { ensure_init(); if let Ok(mut w) = _SOFA.write() { *w = v; } }
pub fn g__TAIL() -> Py { ensure_init(); _TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TAIL(v: Py) { ensure_init(); if let Ok(mut w) = _TAIL.write() { *w = v; } }
pub fn g__WAITING() -> Py { ensure_init(); _WAITING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WAITING(v: Py) { ensure_init(); if let Ok(mut w) = _WAITING.write() { *w = v; } }
fn module_body() {
    let _ = py_str("Where the person is, and what they are up to right now.\n\nA chat is not only questions. People open with a line about the moment they are in - \"im on the\nbus\", \"waiting for the bus\", \"im having a lazy day\" - and there is nothing in it to answer, so\nevery module that answers a request declined it and the turn ended at the fallback: the shrug\n(\"Got it. Is there anything you'd like a hand with?\"), the filing cabinet (\"Noted: quiet day\ntoday.\"), or the wrong card altogether (\"its been a slow day\" got advice about a slow boiler).\n\nThe reply a friend gives is short. It says the line back in a word (\"queues are the worst\"),\nadds one thing that fits that kind of wait or ride or slow day, and leaves the door open for\nwhatever is behind the line, because there usually is something. Four things keep it from being\nthe stock line this file exists to replace:\n\n* only the whole line counts, so \"im on the bus and my stop is coming up, how do i get to the\n  station\" is a question with a place in it and stays with the module that answers it.\n* a line that names where they are and nothing else is the same reading, and on main it was\n  answered with the shrug or the filing cabinet: \"im at work\", \"im at the gym\", \"im at the\n  library\", \"im at nans\", \"im at the hospital\". The reply is about being there with something to\n  ask, never about the place, so it fits every place equally. A job in progress (\"im painting the\n  hall\") is not here: the cards and the pools already answer that better than a line about being\n  somewhere would.\n* the pool is chosen by the shape of the line - a wait, a journey, a bed, a slow day - never by\n  the words of the place in it, so the reply cannot be pasted onto a different kind of line.\n* not every line in a pool ends in a question. A question on the end of every reply turns a chat\n  into an interview, and the person said something rather than asked.\n* the one place where the reply is about what the person should do next is the car: a person\n  typing on the move is driving, and no chat is worth a glance off the road.\n\nThe lines are grouped here rather than spread through the chat module because they are the same\nreading - the person's own moment - and each group needs its own small pool to stay varied.\n");
    s__LEAD(py_str("^\\s*(?:(?:oh|aw+|ah|ha+|hi|hey|tbh|honestly|ok|okay|well|and|but|so|yeah|anyway|still|currently|right now|not much|nm|just|im|i'm|iam|i am)[,\\s]+)*"));
    s__TAIL(py_str("(?:\\s+(?:today|tonight|now|at the moment|right now|lately|this morning|this afternoon|this evening|lol|ha+|haha|tbh|honestly))*"));
    s__END(py_str("\\s*(?:[.!]+|lol|ha+|haha|tbh|honestly)*\\s*$"));
    s__WAITING(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:wait(?:ing)? (?:at|for|in|on) \\w+(?: \\w+){0,3}|wait(?:ing)? in (?:a|the) (?:queue|line|waiting room)|in (?:a|the) (?:queue|line|waiting room)(?: (?:at|for) \\w+(?: \\w+){0,3})?|queu(?:e|ing) (?:at|for) \\w+(?: \\w+){0,2}|on hold(?: with \\w+(?: \\w+)?)?)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__MOVING(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:on (?:the|a|my) (?:bus|train|tube|tram|coach|ferry|plane|flight|boat)|on (?:the|a) (?:bus|train|tube|tram) (?:home|in|back|to \\w+(?: \\w+)?)|on my way (?:home|in|back|there|to \\w+(?: \\w+)?))")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__IN_THE_CAR(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:in (?:the|my) car|driving(?: (?:home|back|up|down|to \\w+(?: \\w+)?))?|on the motorway(?: to \\w+(?: \\w+)?)?|stuck in traffic)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SLOW(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:hav(?:e|ing) (?:a|an) (?:lazy|quiet|slow|cosy|couch|pyjama|relaxing|easy|chill) (?:one|day|morning|afternoon|evening|night|sunday|weekend)|(?:it'?s|its)(?: been)?(?: a)?(?: (?:really|very|proper|nice|lovely))? (?:lazy|quiet|slow|cosy|relaxing|easy) (?:one|day|morning|afternoon|evening|sunday)|(?:lazy|quiet|slow|chilled) day(?: (?:today|here|at work|so far))?|nothing much (?:on|planned|doing)(?: (?:today|here|planned))?|not much (?:on|planned)(?: (?:today|here))?|nothing on (?:today|this afternoon|this morning)|(?:took|taking|having) (?:the|a|my) day off(?: work)?(?: today)?)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__IN_BED(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:(?:still |back |lying |sat |sitting |laying )?in bed|(?:lying|laying|sat|sitting) in bed|back to bed)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__SOFA(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:(?:lying|sat|sitting|slouched|curled up) on the (?:sofa|settee|couch)|curled up (?:with a book|in front of the tv)|having (?:a|another) (?:cup of tea|brew|cuppa|coffee)|chill(?:ing|in)(?: out)?(?: (?:now|at home))?|taking it easy)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ON_A_BREAK(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:on (?:my|a|the) (?:lunch|dinner|tea|coffee) ?(?:break|hour)|on (?:my|a) break|taking (?:a|my) (?:lunch )?break)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AT_WORK(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:(?:at|back at) (?:work|the office)|in (?:the )?office|at (?:my|the) desk|on (?:shift|shifts|nights|days|the clock)|working)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AT_THE_GYM(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:(?:at|in) the gym|working out|lifting|lifting weights|on the treadmill|doing (?:a|my) (?:workout|session|class))")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AT_A_DOCTOR(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:(?:at|in|outside) (?:the |a )?(?:hospital|a&e|ae|casualty|clinic|surgery|doctors|doctor's|gp|dentist|orthodontist|pharmacy|chemist|walk in centre)|at the doctors waiting)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AT_A_PLACE(stdlib_call("re", "compile", &[add(&add(&add(&g__LEAD(), &py_str("(?:(?:at|over at|down at|round at|up at|outside|near) (?:the |a |my |our |me )?[a-z][\\w'&]*(?: [a-z][\\w'&]*){0,2}|at home)")), &g__TAIL()), &g__END()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:limit|limits|loss|a loss|breaking point|end of my tether|wits end|wit's end|fault|risk|peace|war|odds|best|worst|least|most|first|last|standstill|stretch|push|glance|crossroads|disadvantage|loggerheads|pains|ease|large|hand|that age|my age|this|that|it|stage|point|end|end of my rope|loose end|again|once|twice|funeral|wake|christening)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WAITING_REPLIES(py_tuple(vec![py_str("Queues are the worst of it. This is as good a way to spend one as any."), py_str("Nothing to do but wait, then. What are you waiting on?"), py_str("That kind of wait always runs longer than they tell you. Hope it moves."), py_str("Hope it goes quickly. I'm here if you want company while it does.")]));
    s_MOVING_REPLIES(py_tuple(vec![py_str("A journey is a good place for a chat. What's on your mind?"), py_str("Nothing like a ride for thinking things through. Ask away."), py_str("Journeys are mostly waiting with a view. Where are you off to?"), py_str("Mind how you go - I'll be here when you get there.")]));
    s_IN_THE_CAR_REPLIES(py_tuple(vec![py_str("Eyes on the road - I'll be here when you stop."), py_str("No reading and driving. Park up and I'll answer anything you like."), py_str("I'll keep. Get where you're going first, then ask me whatever it is.")]));
    s_SLOW_REPLIES(py_tuple(vec![py_str("Then take it slow. That is what those days are for."), py_str("A lazy day with nothing in it is a good day. Enjoy it."), py_str("A slow one can be a good one. What would make it better?"), py_str("Quiet days are underrated. I'm here if you want something to fill it.")]));
    s_IN_BED_REPLIES(py_tuple(vec![py_str("No rush to get up. What is the rest of the day looking like?"), py_str("Enjoy the duvet. Anything on your mind?"), py_str("A warm bed and nowhere to be. Fair enough."), py_str("Five more minutes is always the right answer. I'm here when you're up.")]));
    s_SOFA_REPLIES(py_tuple(vec![py_str("That sounds like the right place to be. I'm here when you want something."), py_str("Nothing wrong with taking it easy. Ask away if something comes up."), py_str("Comfortable, then. Anything you want a hand with while you're there?"), py_str("A slow afternoon done properly. What are you thinking about?")]));
    s_BREAK_REPLIES(py_tuple(vec![py_str("Break earned. What are you doing with it?"), py_str("Enjoy it — half an hour goes faster than you think."), py_str("A proper break, then. Anything you want a second opinion on?")]));
    s_AT_WORK_REPLIES(py_tuple(vec![py_str("Work can wait for this. What is it?"), py_str("Right, you're on the clock. What do you need?"), py_str("Go on — I'll keep it short.")]));
    s_AT_THE_GYM_REPLIES(py_tuple(vec![py_str("Good stuff. Cardio or the weights?"), py_str("Nice. What are you working on today?"), py_str("Go on then — what do you need between sets?"), py_str("Better than sitting down. What's the question?")]));
    s_AT_A_DOCTOR_REPLIES(py_tuple(vec![py_str("Waiting rooms are the worst of it. What do you need while you're there?"), py_str("That's a long morning whatever it's for. How long have you been there?"), py_str("Hope it goes quickly. What's on your mind?"), py_str("Hospital or not, I'm here. What do you need?")]));
    s_AT_A_PLACE_REPLIES(py_tuple(vec![py_str("What are you up to there?"), py_str("Noted — what do you need while you're there?"), py_str("Right you are. Anything you want a second opinion on?"), py_str("I'll let you get on. What's on your mind?")]));
    s__HEAVY(stdlib_call("re", "compile", &[py_str("\\b(?:died|dying|death|funeral|passed away|kill myself|killing myself|suicide|suicidal|end it all|take my own life|hurt myself|hurting myself|self harm|self-harm|cut myself|overdose|miscarriage|stillborn|terminal|beat me|beats me|hit me|hits me|hurts me|hurting me|threatened me|abused? me|assaulted|not safe|scared of|afraid of|can'?t stop crying|can not stop crying|crying|in tears|so upset|upset about|devastated|heartbroken|at the end of my tether)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STRETCH_OF_TIME(stdlib_call("re", "compile", &[py_str("\\b(?:all|every|each|most|whole) (?:day|days|week|weeks|night|nights|morning|evening|time|summer|winter|year)|these days|most days|all the time|\\d+ (?:days|weeks|hours) a week\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DAY_TO_COME(stdlib_call("re", "compile", &[py_str("\\b(?:tomorrow|next (?:week|month|year|monday|tuesday|wednesday|thursday|friday|saturday|sunday)|on (?:monday|tuesday|wednesday|thursday|friday|saturday|sunday))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_chat_is_heavy(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the turns just before this one were about something heavy.\n\n    The store holds the turns before this one (the line being answered is added after the reply\n    is chosen), so the last two of them decide: a moment line three turns after a disclosure is a\n    change of subject and can be answered.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    earlier = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut turn: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[mcall(&turn.clone(), "get", &[py_str("user"), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&earlier.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&mcall(&g__HEAVY(), "search", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reply(args: &[Py]) -> Py {
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
    let mut seed: Py = Py::None;
    let _ = py_str("A short, warm reply to a line about the moment they are in, or None.");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)) } }) {
    return Py::None;
    }
    if py_truthy(&_the_chat_is_heavy(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_STRETCH_OF_TIME(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DAY_TO_COME(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    seed = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__ON_A_BREAK(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_BREAK_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__WAITING(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_WAITING_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__IN_THE_CAR(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_IN_THE_CAR_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__MOVING(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_MOVING_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__IN_BED(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_IN_BED_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__SOFA(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SOFA_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__SLOW(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_SLOW_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__AT_WORK(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_AT_WORK_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__AT_THE_GYM(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_AT_THE_GYM_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&mcall(&g__AT_A_DOCTOR(), "match", &[clean.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_AT_A_DOCTOR_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    if py_truthy(&{ let __b = mcall(&g__AT_A_PLACE(), "match", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__NOT_A_PLACE(), "search", &[clean.clone()]))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_AT_A_PLACE_REPLIES()); __args.push(memory.clone()); __kws.extend(vec![("seed_text".to_string(), seed.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::compose::pick(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.passing_the_time".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AT_A_DOCTOR_REPLIES" => Some(g_AT_A_DOCTOR_REPLIES()),
            "AT_A_PLACE_REPLIES" => Some(g_AT_A_PLACE_REPLIES()),
            "AT_THE_GYM_REPLIES" => Some(g_AT_THE_GYM_REPLIES()),
            "AT_WORK_REPLIES" => Some(g_AT_WORK_REPLIES()),
            "A_DAY_TO_COME" => Some(g_A_DAY_TO_COME()),
            "A_STRETCH_OF_TIME" => Some(g_A_STRETCH_OF_TIME()),
            "BREAK_REPLIES" => Some(g_BREAK_REPLIES()),
            "IN_BED_REPLIES" => Some(g_IN_BED_REPLIES()),
            "IN_THE_CAR_REPLIES" => Some(g_IN_THE_CAR_REPLIES()),
            "MOVING_REPLIES" => Some(g_MOVING_REPLIES()),
            "SLOW_REPLIES" => Some(g_SLOW_REPLIES()),
            "SOFA_REPLIES" => Some(g_SOFA_REPLIES()),
            "WAITING_REPLIES" => Some(g_WAITING_REPLIES()),
            "_AT_A_DOCTOR" => Some(g__AT_A_DOCTOR()),
            "_AT_A_PLACE" => Some(g__AT_A_PLACE()),
            "_AT_THE_GYM" => Some(g__AT_THE_GYM()),
            "_AT_WORK" => Some(g__AT_WORK()),
            "_END" => Some(g__END()),
            "_HEAVY" => Some(g__HEAVY()),
            "_IN_BED" => Some(g__IN_BED()),
            "_IN_THE_CAR" => Some(g__IN_THE_CAR()),
            "_LEAD" => Some(g__LEAD()),
            "_MOVING" => Some(g__MOVING()),
            "_NOT_A_PLACE" => Some(g__NOT_A_PLACE()),
            "_ON_A_BREAK" => Some(g__ON_A_BREAK()),
            "_SLOW" => Some(g__SLOW()),
            "_SOFA" => Some(g__SOFA()),
            "_TAIL" => Some(g__TAIL()),
            "_WAITING" => Some(g__WAITING()),
            "_the_chat_is_heavy" => Some(py_func("_the_chat_is_heavy", |args: &[Py]| _the_chat_is_heavy(args))),
            "reply" => Some(py_func("reply", |args: &[Py]| reply(args))),
            _ => None,
        }),
        set: None,
    }))
}

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


pub static BEEN_ON_A_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARING_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARING_STRETCH_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EFFORT_STRETCH_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_SPENT_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_STRETCH_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTER_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUEUE_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUEUE_STRETCH_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEBODY_ELSE_IN_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SPENT_ON_A_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRETCH_IS_FINISHED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STRETCH_STARTS_UPPER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAKEN_A_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALKING_STRETCH_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TALKING_TO_A_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static UP_WITH_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VIGIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISITING_SOMEBODY_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISITING_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VISITING_STRETCH_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_AFTER_THE_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_FOR_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_ON_THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_ON_THEIR_OWN_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_STRETCH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_STRETCH_SAID_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAITING_TO_TALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITH_THEIR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_BEEN_ON_A_STRETCH() -> Py { ensure_init(); BEEN_ON_A_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BEEN_ON_A_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = BEEN_ON_A_STRETCH.write() { *w = v; } }
pub fn g_CARING_STRETCH() -> Py { ensure_init(); CARING_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARING_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = CARING_STRETCH.write() { *w = v; } }
pub fn g_CARING_STRETCH_SAID_BACK() -> Py { ensure_init(); CARING_STRETCH_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARING_STRETCH_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = CARING_STRETCH_SAID_BACK.write() { *w = v; } }
pub fn g_EFFORT_STRETCH() -> Py { ensure_init(); EFFORT_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_STRETCH.write() { *w = v; } }
pub fn g_EFFORT_STRETCH_SAID_BACK() -> Py { ensure_init(); EFFORT_STRETCH_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EFFORT_STRETCH_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = EFFORT_STRETCH_SAID_BACK.write() { *w = v; } }
pub fn g_NOT_A_SPENT_JOB() -> Py { ensure_init(); NOT_A_SPENT_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SPENT_JOB(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_SPENT_JOB.write() { *w = v; } }
pub fn g_NOT_A_STRETCH_JOB() -> Py { ensure_init(); NOT_A_STRETCH_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STRETCH_JOB(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_STRETCH_JOB.write() { *w = v; } }
pub fn g_POINTER_STRETCH() -> Py { ensure_init(); POINTER_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTER_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = POINTER_STRETCH.write() { *w = v; } }
pub fn g_QUEUE_STRETCH() -> Py { ensure_init(); QUEUE_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUEUE_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = QUEUE_STRETCH.write() { *w = v; } }
pub fn g_QUEUE_STRETCH_SAID_BACK() -> Py { ensure_init(); QUEUE_STRETCH_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUEUE_STRETCH_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = QUEUE_STRETCH_SAID_BACK.write() { *w = v; } }
pub fn g_SOMEBODY_ELSE_IN_IT() -> Py { ensure_init(); SOMEBODY_ELSE_IN_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEBODY_ELSE_IN_IT(v: Py) { ensure_init(); if let Ok(mut w) = SOMEBODY_ELSE_IN_IT.write() { *w = v; } }
pub fn g_SPENT_ON_A_STRETCH() -> Py { ensure_init(); SPENT_ON_A_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SPENT_ON_A_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = SPENT_ON_A_STRETCH.write() { *w = v; } }
pub fn g_STRETCH_IS_FINISHED() -> Py { ensure_init(); STRETCH_IS_FINISHED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRETCH_IS_FINISHED(v: Py) { ensure_init(); if let Ok(mut w) = STRETCH_IS_FINISHED.write() { *w = v; } }
pub fn g_STRETCH_STARTS_UPPER() -> Py { ensure_init(); STRETCH_STARTS_UPPER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STRETCH_STARTS_UPPER(v: Py) { ensure_init(); if let Ok(mut w) = STRETCH_STARTS_UPPER.write() { *w = v; } }
pub fn g_TAKEN_A_STRETCH() -> Py { ensure_init(); TAKEN_A_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAKEN_A_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = TAKEN_A_STRETCH.write() { *w = v; } }
pub fn g_TALKING_STRETCH_SAID_BACK() -> Py { ensure_init(); TALKING_STRETCH_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALKING_STRETCH_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = TALKING_STRETCH_SAID_BACK.write() { *w = v; } }
pub fn g_TALKING_TO_A_PLACE() -> Py { ensure_init(); TALKING_TO_A_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TALKING_TO_A_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = TALKING_TO_A_PLACE.write() { *w = v; } }
pub fn g_UP_WITH_A_PERSON() -> Py { ensure_init(); UP_WITH_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_UP_WITH_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = UP_WITH_A_PERSON.write() { *w = v; } }
pub fn g_VIGIL() -> Py { ensure_init(); VIGIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VIGIL(v: Py) { ensure_init(); if let Ok(mut w) = VIGIL.write() { *w = v; } }
pub fn g_VISITING_SOMEBODY_SAID_BACK() -> Py { ensure_init(); VISITING_SOMEBODY_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISITING_SOMEBODY_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = VISITING_SOMEBODY_SAID_BACK.write() { *w = v; } }
pub fn g_VISITING_STRETCH() -> Py { ensure_init(); VISITING_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISITING_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = VISITING_STRETCH.write() { *w = v; } }
pub fn g_VISITING_STRETCH_SAID_BACK() -> Py { ensure_init(); VISITING_STRETCH_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VISITING_STRETCH_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = VISITING_STRETCH_SAID_BACK.write() { *w = v; } }
pub fn g_WAITING_AFTER_THE_STRETCH() -> Py { ensure_init(); WAITING_AFTER_THE_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_AFTER_THE_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_AFTER_THE_STRETCH.write() { *w = v; } }
pub fn g_WAITING_FOR_A_PERSON() -> Py { ensure_init(); WAITING_FOR_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_FOR_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_FOR_A_PERSON.write() { *w = v; } }
pub fn g_WAITING_ON_THEIR_OWN() -> Py { ensure_init(); WAITING_ON_THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_ON_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_ON_THEIR_OWN.write() { *w = v; } }
pub fn g_WAITING_ON_THEIR_OWN_SAID_BACK() -> Py { ensure_init(); WAITING_ON_THEIR_OWN_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_ON_THEIR_OWN_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_ON_THEIR_OWN_SAID_BACK.write() { *w = v; } }
pub fn g_WAITING_STRETCH() -> Py { ensure_init(); WAITING_STRETCH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_STRETCH(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_STRETCH.write() { *w = v; } }
pub fn g_WAITING_STRETCH_SAID_BACK() -> Py { ensure_init(); WAITING_STRETCH_SAID_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_STRETCH_SAID_BACK(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_STRETCH_SAID_BACK.write() { *w = v; } }
pub fn g_WAITING_TO_TALK() -> Py { ensure_init(); WAITING_TO_TALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAITING_TO_TALK(v: Py) { ensure_init(); if let Ok(mut w) = WAITING_TO_TALK.write() { *w = v; } }
pub fn g_WITH_THEIR_OWN() -> Py { ensure_init(); WITH_THEIR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITH_THEIR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = WITH_THEIR_OWN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A stretch of the day already spent on one thing, said while it is still going on.\n\n\"ive been waiting for the plumber since nine\" was answered \"Got it. Is there anything you'd like a\nhand with?\", \"ive been chasing the bank for two weeks\" was filed with \"Noted: you've been chasing\nthe bank for two weeks.\", and \"ive been painting the fence all morning\" got the receipt that asks\nfor the one thing the line had just given: \"You've been painting the fence all morning, how long\nhave you been at it?\" (bot/memory/ack.py, GOING_TAIL). None of those is a friend's reply to\nsomebody who has been kept waiting, or who is halfway through a job.\n\nThe reading is the shape of the line rather than its words: first person, `been`, a doing, and a\nstretch of time in the same line. What the doing is decides the reply -- waiting on other people,\na job of their own, or looking after somebody -- and a state of the body or the mood is left to\nthe readers that already own it. The reply names the stretch the person gave and never the\nactivity, so one pool covers a fence, a loft and a pile of marking, and the question it asks is\nthe only one left open: how close to done, whether anyone has answered, or whether anybody gives\nthem a break.\n\nIt is asked from bot/modules/greeting.py, and only ever in place of a placeholder -- the three\nstock lines, a receipt, a search hedge or the person's own words said back -- so a line the\ncomposer has an answer of its own for keeps it. It lives in its own file because\nbot/compose/fallback.py is at the world's size cap.\n");
    s_BEEN_ON_A_STRETCH(stdlib_call("re", "compile", &[py_str("\\b(?:ive|i've|i\\u2019ve|i have|i had|weve|we've|we\\u2019ve|we have|i)\\s+been\\b(?P<doing>[a-z' ]{3,60}?)(?P<when>\\ball (?:day|night|morning|afternoon|evening|week|weekend|month|year|shift|term)\\b|\\b(?:most of the|the whole|the entire)\\s+(?:day|morning|afternoon|evening|week|weekend)\\b|\\bfor (?:ages|ever|hours|days|weeks|months|years|minutes|a while|half an hour|(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|forty|fifteen|\\d+)\\s*(?:hour|hours|minute|minutes|day|days|week|weeks|month|months|year|years))\\b|\\b(?:ever since|since (?:this |last |the )?(?:morning|afternoon|evening|night|week|weekend|month|year|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december|christmas|easter|yesterday|noon|midnight|half (?:past )?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d+(?::\\d+)?\\s*(?:am|pm)?))\\b|\\b(?:hours|days|weeks|months|years|ages)\\s+now\\b\\s*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTER_STRETCH(py_tuple(vec![py_str("at this"), py_str("on this"), py_str("at it"), py_str("on it"), py_str("at that"), py_str("on that"), py_str("at these"), py_str("on these"), py_str("with this"), py_str("with it"), py_str("the thing")]));
    s_SPENT_ON_A_STRETCH(stdlib_call("re", "compile", &[py_str("\\b(?:ive|i've|i\\u2019ve|i have|i|weve|we've|we\\u2019ve|we have|we)\\s+(?:spent|wasted)\\b(?P<doing>[a-z' ]{0,40}?)\\s*(?P<when>\\ball (?:day|night|morning|afternoon|evening|weekend|week)\\b|\\b(?:most of the|the whole|the entire)\\s+(?:day|morning|afternoon|evening|weekend|week)\\b|\\bfor (?:ages|hours|days|weeks|months|years)\\b|\\b(?:hours|days|weeks|months|years)\\s+now\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKEN_A_STRETCH(stdlib_call("re", "compile", &[py_str("\\b(?:this|that|it)(?:'s|s|\\u2019s|\\s+has|\\s+had)\\s+taken\\s+(?:me|us)\\s+(?P<when>\\ball (?:day|night|morning|afternoon|evening|weekend|week)\\b|\\b(?:most of the|the whole|the entire)\\s+(?:day|morning|afternoon|evening|weekend|week)\\b|\\bfor (?:ages|hours|days|weeks|months|years)\\b|\\b(?:hours|days|weeks|months|years)\\s+now\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VISITING_STRETCH(py_tuple(vec![py_str("at the hospital"), py_str("in hospital"), py_str("at hospital"), py_str("in the hospital"), py_str("at a&e"), py_str("in a&e"), py_str("at the a&e"), py_str("in the a&e"), py_str("at the clinic"), py_str("at the doctors"), py_str("at the doctor's"), py_str("at the surgery"), py_str("at the care home"), py_str("at the hospice"), py_str("at the nursing home"), py_str("at the vets"), py_str("at the vet")]));
    s_STRETCH_IS_FINISHED(py_tuple(vec![py_str("finished"), py_str("finished it"), py_str("done it"), py_str("all done"), py_str("got it done"), py_str("sorted it"), py_str("sorted now"), py_str("got there"), py_str("finally done"), py_str("managed it"), py_str("got it finished"), py_str("its done"), py_str("it's done")]));
    s_QUEUE_STRETCH(py_tuple(vec![py_str("queueing"), py_str("queuing"), py_str("queued"), py_str("in a queue"), py_str("sat here"), py_str("standing here"), py_str("sat in the car"), py_str("waiting for the bus"), py_str("waiting for a bus"), py_str("waiting for the train"), py_str("waiting for a taxi"), py_str("waiting at the tip"), py_str("at the tip"), py_str("at the dump"), py_str("recycling centre")]));
    s_WITH_THEIR_OWN(py_tuple(vec![py_str("with my mum"), py_str("with my mom"), py_str("with my mother"), py_str("with my dad"), py_str("with my father"), py_str("with my nan"), py_str("with my gran"), py_str("with my grandma"), py_str("with my grandad"), py_str("with my granny"), py_str("with my son"), py_str("with my daughter"), py_str("with my kids"), py_str("with my child"), py_str("with my husband"), py_str("with my wife"), py_str("with my partner"), py_str("with my sister"), py_str("with my brother"), py_str("with my friend"), py_str("with my mate"), py_str("at my mum's"), py_str("at my mom's"), py_str("at my dad's"), py_str("at my nan's")]));
    s_WAITING_AFTER_THE_STRETCH(py_tuple(vec![py_str("waiting"), py_str("waited"), py_str("on hold"), py_str("in a queue"), py_str("queueing"), py_str("queuing"), py_str("on the phone")]));
    s_VIGIL(stdlib_call("re", "compile", &[py_str("\\bwait(?:ing|ed)?\\s+(?:for|on|by|at|with)\\b[^.]{0,24}?\\b(?:die|dying|death|pass|passing|go|wake|wake up|come round|come back|pull through|make it|breathe|survive)\\b|\\b(?:by|at|beside)\\s+(?:his|her|their|my (?:mum|mom|dad|nan|gran|son|daughter|wife|husband|partner|sister|brother|friend)'?s?)\\s+(?:bed|bedside)\\b|\\b(?:his|her|their)\\s+bedside\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SPENT_JOB(py_tuple(vec![py_str("to get here"), py_str("to get there"), py_str("to drive"), py_str("to come"), py_str("to travel"), py_str("to walk"), py_str("the bus"), py_str("the train"), py_str("the traffic"), py_str("at work"), py_str("at the office"), py_str("in work"), py_str("at school"), py_str("at college"), py_str("at uni"), py_str("on a course")]));
    s_WAITING_STRETCH(py_tuple(vec![py_str("waiting"), py_str("waited"), py_str("chasing"), py_str("chased"), py_str("chase"), py_str("ringing"), py_str("phoning"), py_str("phoned"), py_str("calling"), py_str("emailing"), py_str("emailed"), py_str("messaging"), py_str("queuing"), py_str("queueing"), py_str("queued"), py_str("hounding"), py_str("pestering"), py_str("chasing it"), py_str("on hold"), py_str("on the phone"), py_str("in a queue"), py_str("on a waiting list"), py_str("on a waiting list"), py_str("back and forth"), py_str("trying to get through"), py_str("trying to speak"), py_str("trying to cancel"), py_str("hunting"), py_str("searching"), py_str("applying"), py_str("job hunting")]));
    s_CARING_STRETCH(py_tuple(vec![py_str("looking after"), py_str("caring for"), py_str("sitting with"), py_str("nursing"), py_str("minding"), py_str("cleaning up after"), py_str("running after"), py_str("running round after"), py_str("feeding"), py_str("settling"), py_str("carrying")]));
    s_UP_WITH_A_PERSON(stdlib_call("re", "compile", &[py_str("\\bup with\\s+(?:my|her|his|their|our|the|a)\\s+\\w+|\\bup with\\s+(?:him|her|them|mum|dad|nan|gran|the kids|someone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EFFORT_STRETCH(py_tuple(vec![py_str("painting"), py_str("decorating"), py_str("wallpapering"), py_str("tiling"), py_str("plastering"), py_str("sanding"), py_str("stripping"), py_str("cleaning"), py_str("tidying"), py_str("sorting"), py_str("clearing"), py_str("hoovering"), py_str("vacuuming"), py_str("scrubbing"), py_str("washing"), py_str("ironing"), py_str("dusting"), py_str("unloading"), py_str("loading"), py_str("packing"), py_str("unpacking"), py_str("moving"), py_str("digging"), py_str("weeding"), py_str("mowing"), py_str("pruning"), py_str("building"), py_str("fixing"), py_str("mending"), py_str("repairing"), py_str("renovating"), py_str("assembling"), py_str("filing"), py_str("shredding"), py_str("decluttering"), py_str("boxing"), py_str("shifting"), py_str("polishing"), py_str("hanging"), py_str("hammering")]));
    s_NOT_A_STRETCH_JOB(py_tuple(vec![py_str("feeling"), py_str("felt"), py_str("low"), py_str("flat"), py_str("tired"), py_str("exhausted"), py_str("knackered"), py_str("shattered"), py_str("sleeping"), py_str("slept"), py_str("waking"), py_str("hurting"), py_str("aching"), py_str("sore"), py_str("sick"), py_str("puking"), py_str("vomiting"), py_str("throwing up"), py_str("bleeding"), py_str("coughing"), py_str("sneezing"), py_str("crying"), py_str("shaking"), py_str("dizzy"), py_str("worried"), py_str("anxious"), py_str("stressing"), py_str("stressed"), py_str("panicking"), py_str("missing"), py_str("thinking"), py_str("going over"), py_str("dreaming"), py_str("eating"), py_str("drinking"), py_str("smoking"), py_str("scratching"), py_str("itching"), py_str("out of breath"), py_str("breathless"), py_str("sweating"), py_str("on my feet"), py_str("in bed"), py_str("off work"), py_str("off sick"), py_str("off ill"), py_str("signed off"), py_str("on the sick")]));
    s_WAITING_STRETCH_SAID_BACK(py_tuple(vec![py_str("{When} - that is a long time to be kept waiting. Has anyone given you an answer yet?"), py_str("{When} and still no answer - who is left to try?"), py_str("{When} chasing the same thing. Has it moved at all?"), py_str("{When} and it is all waiting on other people. Is there a name you can ask for?")]));
    s_EFFORT_STRETCH_SAID_BACK(py_tuple(vec![py_str("{When} - that is a lot of it on one job. How close is it to done?"), py_str("{When} and still at it - what is left of it?"), py_str("{When} - jobs like that always take longer than they look. What is left?"), py_str("{When} at the same thing - is the end in sight?")]));
    s_CARING_STRETCH_SAID_BACK(py_tuple(vec![py_str("{When} - that is a lot of looking after someone, on your own. Does anyone give you a break?"), py_str("{When} being needed and nothing else. Who is looking after you in it?"), py_str("{When} of somebody else's week taking yours. Have you got any help with it?")]));
    s_QUEUE_STRETCH_SAID_BACK(py_tuple(vec![py_str("{When} in a queue - did it move at all?"), py_str("{When} of it and no sign of it moving. Is there another way round it, or is this the only one?"), py_str("{When} of it with nothing to show for it. How much more of it is there?")]));
    s_VISITING_STRETCH_SAID_BACK(py_tuple(vec![py_str("{When} in that place - those are mostly waiting about, whichever end of it you are on. How is it looking?"), py_str("{When}, and it is all waiting. How are things there?"), py_str("{When} of it. Nobody plans a day like that - how is it going?")]));
    s_VISITING_SOMEBODY_SAID_BACK(py_tuple(vec![py_str("{When} sitting with somebody. That is a long day for you as well - is anyone spelling you?"), py_str("{When} and no let up - how are they doing, and how are you doing with it?"), py_str("{When} of it. Is there anyone sitting with them so you can get a break?")]));
    s_SOMEBODY_ELSE_IN_IT(py_tuple(vec![py_str("with my mum"), py_str("with my mom"), py_str("with my mother"), py_str("with my dad"), py_str("with my father"), py_str("with my nan"), py_str("with my gran"), py_str("with my grandma"), py_str("with my grandad"), py_str("with my granny"), py_str("with my son"), py_str("with my daughter"), py_str("with my kids"), py_str("with my child"), py_str("with my husband"), py_str("with my wife"), py_str("with my partner"), py_str("with my sister"), py_str("with my brother"), py_str("with my friend"), py_str("with my mate"), py_str("to see my mum"), py_str("to see my mom"), py_str("to see my dad"), py_str("to see my nan"), py_str("to see my gran"), py_str("to see my son"), py_str("to see my daughter"), py_str("to see my wife"), py_str("to see my husband"), py_str("to see my partner"), py_str("to see my sister"), py_str("to see my brother"), py_str("to see my friend"), py_str("visiting my"), py_str("at my mum's"), py_str("at my mom's"), py_str("at my dad's"), py_str("at my nan's"), py_str("my mum is in"), py_str("my dad is in"), py_str("my nan is in"), py_str("my son is in"), py_str("my daughter is in"), py_str("my wife is in"), py_str("my husband is in"), py_str("my sister is in"), py_str("my brother is in"), py_str("my friend is in"), py_str("hes in hospital"), py_str("shes in hospital"), py_str("theyre in hospital"), py_str("with him"), py_str("with her"), py_str("with them")]));
    s_WAITING_ON_THEIR_OWN(py_tuple(vec![py_str("my mum"), py_str("my mom"), py_str("my dad"), py_str("my nan"), py_str("my gran"), py_str("my son"), py_str("my daughter"), py_str("my kids"), py_str("my husband"), py_str("my wife"), py_str("my partner"), py_str("my sister"), py_str("my brother"), py_str("my friend"), py_str("my mate"), py_str("my lad"), py_str("my girl"), py_str("the kids")]));
    s_WAITING_FOR_A_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:for|on)\\s+(?:him|her|them)\\s+(?:to|about)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WAITING_ON_THEIR_OWN_SAID_BACK(py_tuple(vec![py_str("{When} waiting on somebody of your own. Have they been in touch at all?"), py_str("{When} of it, and no word. Is this how they usually are?"), py_str("{When} waiting on them. That is a long time to be kept hanging about.")]));
    s_WAITING_TO_TALK(py_tuple(vec![py_str("to talk to someone"), py_str("to talk to somebody"), py_str("to talk to anyone"), py_str("for someone to talk to"), py_str("for somebody to talk to"), py_str("to have someone to talk to"), py_str("to have somebody to talk to"), py_str("to speak to someone"), py_str("to speak to somebody"), py_str("someone to talk to"), py_str("somebody to talk to")]));
    s_TALKING_TO_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:talk to|speak to|get through to|get hold of)\\s+(?:someone|somebody|a person)\\s+(?:at|from|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TALKING_STRETCH_SAID_BACK(py_tuple(vec![py_str("{When} with nobody to talk to. That is a long stretch to carry on your own - what have you been doing with it?"), py_str("{When} of it, and no company. What is the day like where you are?"), py_str("{When} waiting for someone to talk to - well, you have my attention now. What is on your mind?")]));
    s_STRETCH_STARTS_UPPER(call_builtin("frozenset", &[py_tuple(vec![py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("christmas"), py_str("easter")])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _the_stretch_said_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut lead: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The person's own stretch, written the way the reply says it back.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&phrase.clone(), "split", &[])]), "strip", &[py_str(" .,!?;:")]), "split", &[])) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_list(vec![py_str("for")]), py_list(vec![py_str("since")]), py_list(vec![py_str("all")]), py_list(vec![py_str("most")]), py_list(vec![py_str("the")])]), &slice_or_raise(&words.clone(), &Py::None, &Py::Int(1), &Py::None))); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&words.clone(), &Py::Int(0)), slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]), 2, None);
    lead = __unpk1[0].clone();
    rest = __unpk1[1].clone();
    words = add(&py_list(vec![lead.clone()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut w: Py = __item.clone(); __out.push(if py_truthy(&Py::Bool(py_contains(&g_STRETCH_STARTS_UPPER(), &mcall(&w.clone(), "lower", &[])))) { mcall(&w.clone(), "capitalize", &[]) } else { w.clone() }); } py_list(__out) })() });
    }
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    said = mcall(&py_str(" "), "join", &[words.clone()]);
    return add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _names_one_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doing"))) };
    let mut phrases: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the doing holds one of the phrases, as whole words.");
    words = mcall(&doing.clone(), "split", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push({ let __b = { let __b = Py::Bool(py_contains(&phrase.clone(), &py_str(" "))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&doing.clone(), &phrase.clone())) } }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &phrase.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_shape(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut doing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doing"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("Which pool this stretch belongs in, or None when it is not one of them.\n\n    The doing decides first, because a job is the most specific thing in the line; the words after\n    the stretch come next, which is where \"ive been sat here for two hours waiting\" keeps its\n    wait; and a doing that only points at the job (\"at this\", \"on it\") is a job of their own.\n    ");
    if py_truthy(&_names_one_of(&[doing.clone(), g_NOT_A_STRETCH_JOB()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _names_one_of(&[doing.clone(), g_CARING_STRETCH()]); if py_truthy(&__b) { __b } else { mcall(&g_UP_WITH_A_PERSON(), "search", &[said.clone()]) } }) {
    return py_str("caring");
    }
    if py_truthy(&{ let __b = _names_one_of(&[doing.clone(), g_QUEUE_STRETCH()]); if py_truthy(&__b) { __b } else { _names_one_of(&[said.clone(), g_QUEUE_STRETCH()]) } }) {
    return py_str("queueing");
    }
    if py_truthy(&_names_one_of(&[doing.clone(), g_WAITING_STRETCH()])) {
    if py_truthy(&{ let __b = _names_one_of(&[said.clone(), g_WAITING_TO_TALK()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TALKING_TO_A_PLACE(), "search", &[said.clone()]))) } }) {
    return py_str("talking");
    }
    if py_truthy(&{ let __b = _names_one_of(&[doing.clone(), g_WAITING_ON_THEIR_OWN()]); if py_truthy(&__b) { __b } else { { let __b = _names_one_of(&[after.clone(), g_WAITING_ON_THEIR_OWN()]); if py_truthy(&__b) { __b } else { mcall(&g_WAITING_FOR_A_PERSON(), "search", &[said.clone()]) } } } }) {
    return py_str("waiting_on_their_own");
    }
    return py_str("waiting");
    }
    if py_truthy(&{ let __b = _names_one_of(&[said.clone(), g_VISITING_STRETCH()]); if py_truthy(&__b) { __b } else { _names_one_of(&[said.clone(), g_WITH_THEIR_OWN()]) } }) {
    return py_str("visiting");
    }
    if py_truthy(&_names_one_of(&[doing.clone(), g_EFFORT_STRETCH()])) {
    return py_str("effort");
    }
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { _names_one_of(&[doing.clone(), g_POINTER_STRETCH()]) } }) {
    return py_str("effort");
    }
    if py_truthy(&{ let __b = _names_one_of(&[after.clone(), g_WAITING_AFTER_THE_STRETCH()]); if py_truthy(&__b) { __b } else { _names_one_of(&[said.clone(), g_WAITING_AFTER_THE_STRETCH()]) } }) {
    return if py_truthy(&_names_one_of(&[said.clone(), g_QUEUE_STRETCH()])) { py_str("queueing") } else { py_str("waiting") };
    }
    return Py::None;
    Py::None
}
pub fn reads_as_a_long_stretch(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut met: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut spent: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("(shape, the stretch to say back) for a stretch the person is still inside, or None.\n\n    shape is \"waiting\", \"effort\", \"caring\" or \"visiting\". Only a first-person line with a\"\n    stretch of time in it counts - \"i've been <doing> since nine\", \"ive spent all day on this\n    form\", \"this has taken me all afternoon\" - and only where the doing is one of those kinds:\n    anything else has its own reader above, and a state of the body or the mood is somebody\n    else's question.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_BEEN_ON_A_STRETCH(), Py::Bool(false)]), py_tuple(vec![g_SPENT_ON_A_STRETCH(), Py::Bool(true)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    spent = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[said.clone()]));
    for __item in __iter3 {
    met = __item.clone();
    doing = mcall(&{ let __b = mcall(&met.clone(), "group", &[py_str("doing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[py_str(" ,.;:")]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&doing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&doing.clone(), &py_str(" and "))) } }) {
    continue;
    }
    when = _the_stretch_said_back(&[mcall(&met.clone(), "group", &[py_str("when")])]);
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = _names_one_of(&[said.clone(), g_NOT_A_STRETCH_JOB()]); if py_truthy(&__b) { __b } else { mcall(&g_VIGIL(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&_names_one_of(&[said.clone(), g_STRETCH_IS_FINISHED()])) {
    continue;
    }
    if py_truthy(&{ let __b = spent.clone(); if !py_truthy(&__b) { __b } else { _names_one_of(&[said.clone(), g_NOT_A_SPENT_JOB()]) } }) {
    continue;
    }
    shape = _the_shape(&[doing.clone(), said.clone(), slice_or_raise(&said.clone(), &mcall(&met.clone(), "end", &[]), &Py::None, &Py::None)]);
    if py_truthy(&{ let __b = shape.clone(); if py_truthy(&__b) { __b } else { spent.clone() } }) {
    return py_tuple(vec![{ let __b = shape.clone(); if py_truthy(&__b) { __b } else { py_str("effort") } }, when.clone()]);
    }
    }
    }
    let __iter4 = py_iter_or_raise(&mcall(&g_TAKEN_A_STRETCH(), "finditer", &[said.clone()]));
    for __item in __iter4 {
    met = __item.clone();
    when = _the_stretch_said_back(&[mcall(&met.clone(), "group", &[py_str("when")])]);
    if py_truthy(&Py::Bool(!py_truthy(&when.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = _names_one_of(&[said.clone(), g_NOT_A_STRETCH_JOB()]); if py_truthy(&__b) { __b } else { _names_one_of(&[said.clone(), g_NOT_A_SPENT_JOB()]) } }) {
    continue;
    }
    if py_truthy(&_names_one_of(&[said.clone(), g_STRETCH_IS_FINISHED()])) {
    continue;
    }
    return py_tuple(vec![py_str("effort"), when.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn the_stretch_pool(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut shape: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shape"))) };
    let mut when: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "when") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: when"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let _ = py_str("The three lines for this kind of stretch, with the person's own time in them.\n\n    Two of the shapes are split by who the line is about rather than by what was being done: a day\n    at a hospital with somebody is not a day in the bed, and a wait for your sister to ring is not\n    a wait for a bank. `said` is the person's own line, and it decides which half.\n    ");
    if py_truthy(&Py::Bool(py_eq(&shape.clone(), &py_str("visiting")))) {
    shape = if py_truthy(&_names_one_of(&[said.clone(), g_SOMEBODY_ELSE_IN_IT()])) { py_str("visiting_somebody") } else { py_str("visiting") };
    }
    pool = mcall(&py_dict(vec![(py_str("waiting"), g_WAITING_STRETCH_SAID_BACK()), (py_str("effort"), g_EFFORT_STRETCH_SAID_BACK()), (py_str("caring"), g_CARING_STRETCH_SAID_BACK()), (py_str("queueing"), g_QUEUE_STRETCH_SAID_BACK()), (py_str("visiting"), g_VISITING_STRETCH_SAID_BACK()), (py_str("visiting_somebody"), g_VISITING_SOMEBODY_SAID_BACK()), (py_str("waiting_on_their_own"), g_WAITING_ON_THEIR_OWN_SAID_BACK()), (py_str("talking"), g_TALKING_STRETCH_SAID_BACK())]), "get", &[shape.clone(), py_tuple(vec![])]);
    return call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "replace", &[py_str("{When}"), when.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_a_placeholder(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut mine: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut theirs: Py = Py::None;
    let _ = py_str("True when the reply tells the person nothing of its own about the line.\n\n    Three ways a reply says nothing back: the composer's three stock lines, which fit any message;\n    a search hedge or the shrug; and a receipt -- the person's own line handed back with a frame\n    around it (\"Noted: ...\", \"Got it - you've been painting the fence all morning, how long have\n    you been at it?\"), which asks what the line has already said. The last of those is read by\n    the words the two share, so a frame this module has never seen counts as well.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback::g_NEUTRAL(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = crate::gen::compose::fallback::_is_a_hedge(&[said.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::ack::reads_as_a_receipt(&[said.clone()]) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::compose::fallback_data::g_A_SMALL_DAY(), &said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::compose::fallback_data::g_A_DAY_ASKED_PLAIN(), &said.clone())) } }) {
    return Py::Bool(true);
    }
    mine = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    theirs = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    shared = bitand(&mine.clone(), &theirs.clone());
    return { let __b = ge(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { ge(&truediv(&call_builtin("len", &[shared.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Float(0.6)) } };
    Py::None
}
pub fn the_stretch_said_back(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut clean: Py = Py::None;
    let mut stretch: Py = Py::None;
    let _ = py_str("The reply for a stretch the person is still inside, or \"\" when this is not that line.\n\n    Only a placeholder is replaced: a line the composer answered with something of its own, or\n    that another reader here has already picked up, keeps its reply. The stretch is said back in\n    the person's own time and then one question is asked of it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_only_a_placeholder(&[text.clone(), said.clone()])))) {
    return py_str("");
    }
    stretch = reads_as_a_long_stretch(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&stretch.clone()))) {
    return py_str("");
    }
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = py_call(&crate::gen::compose::fallback::g__choose(), &[the_stretch_pool(&[geti(&stretch.clone(), &Py::Int(0)), geti(&stretch.clone(), &Py::Int(1)), clean.clone()]), memory.clone(), clean.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.stretches".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "BEEN_ON_A_STRETCH" => Some(g_BEEN_ON_A_STRETCH()),
            "CARING_STRETCH" => Some(g_CARING_STRETCH()),
            "CARING_STRETCH_SAID_BACK" => Some(g_CARING_STRETCH_SAID_BACK()),
            "EFFORT_STRETCH" => Some(g_EFFORT_STRETCH()),
            "EFFORT_STRETCH_SAID_BACK" => Some(g_EFFORT_STRETCH_SAID_BACK()),
            "NOT_A_SPENT_JOB" => Some(g_NOT_A_SPENT_JOB()),
            "NOT_A_STRETCH_JOB" => Some(g_NOT_A_STRETCH_JOB()),
            "POINTER_STRETCH" => Some(g_POINTER_STRETCH()),
            "QUEUE_STRETCH" => Some(g_QUEUE_STRETCH()),
            "QUEUE_STRETCH_SAID_BACK" => Some(g_QUEUE_STRETCH_SAID_BACK()),
            "SOMEBODY_ELSE_IN_IT" => Some(g_SOMEBODY_ELSE_IN_IT()),
            "SPENT_ON_A_STRETCH" => Some(g_SPENT_ON_A_STRETCH()),
            "STRETCH_IS_FINISHED" => Some(g_STRETCH_IS_FINISHED()),
            "STRETCH_STARTS_UPPER" => Some(g_STRETCH_STARTS_UPPER()),
            "TAKEN_A_STRETCH" => Some(g_TAKEN_A_STRETCH()),
            "TALKING_STRETCH_SAID_BACK" => Some(g_TALKING_STRETCH_SAID_BACK()),
            "TALKING_TO_A_PLACE" => Some(g_TALKING_TO_A_PLACE()),
            "UP_WITH_A_PERSON" => Some(g_UP_WITH_A_PERSON()),
            "VIGIL" => Some(g_VIGIL()),
            "VISITING_SOMEBODY_SAID_BACK" => Some(g_VISITING_SOMEBODY_SAID_BACK()),
            "VISITING_STRETCH" => Some(g_VISITING_STRETCH()),
            "VISITING_STRETCH_SAID_BACK" => Some(g_VISITING_STRETCH_SAID_BACK()),
            "WAITING_AFTER_THE_STRETCH" => Some(g_WAITING_AFTER_THE_STRETCH()),
            "WAITING_FOR_A_PERSON" => Some(g_WAITING_FOR_A_PERSON()),
            "WAITING_ON_THEIR_OWN" => Some(g_WAITING_ON_THEIR_OWN()),
            "WAITING_ON_THEIR_OWN_SAID_BACK" => Some(g_WAITING_ON_THEIR_OWN_SAID_BACK()),
            "WAITING_STRETCH" => Some(g_WAITING_STRETCH()),
            "WAITING_STRETCH_SAID_BACK" => Some(g_WAITING_STRETCH_SAID_BACK()),
            "WAITING_TO_TALK" => Some(g_WAITING_TO_TALK()),
            "WITH_THEIR_OWN" => Some(g_WITH_THEIR_OWN()),
            "_names_one_of" => Some(py_func("_names_one_of", |args: &[Py]| _names_one_of(args))),
            "_only_a_placeholder" => Some(py_func("_only_a_placeholder", |args: &[Py]| _only_a_placeholder(args))),
            "_the_shape" => Some(py_func("_the_shape", |args: &[Py]| _the_shape(args))),
            "_the_stretch_said_back" => Some(py_func("_the_stretch_said_back", |args: &[Py]| _the_stretch_said_back(args))),
            "reads_as_a_long_stretch" => Some(py_func("reads_as_a_long_stretch", |args: &[Py]| reads_as_a_long_stretch(args))),
            "the_stretch_pool" => Some(py_func("the_stretch_pool", |args: &[Py]| the_stretch_pool(args))),
            "the_stretch_said_back" => Some(py_func("the_stretch_said_back", |args: &[Py]| the_stretch_said_back(args))),
            _ => None,
        }),
        set: None,
    }))
}

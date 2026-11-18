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


pub static ASK_WITHOUT_MY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNT_ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNT_BARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_OF_MONTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAY_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELIDED_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELIDED_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELIDED_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELIDED_NOT_SURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELIDED_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MANY_OF_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEMORY_POINTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MISSED_CATCH_UP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_AS_A_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THAT_DATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TIME_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOPIC_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEEKDAY_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_I_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_END_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_END_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_INVERTED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_INVERTED_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_QUALITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_START_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_THING_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_VAGUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_PLAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ASK_WITHOUT_MY() -> Py { ensure_init(); ASK_WITHOUT_MY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_WITHOUT_MY(v: Py) { ensure_init(); if let Ok(mut w) = ASK_WITHOUT_MY.write() { *w = v; } }
pub fn g_CALENDAR_SAID() -> Py { ensure_init(); CALENDAR_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_SAID(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR_SAID.write() { *w = v; } }
pub fn g_COUNT_ANIMAL() -> Py { ensure_init(); COUNT_ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNT_ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = COUNT_ANIMAL.write() { *w = v; } }
pub fn g_COUNT_BARE() -> Py { ensure_init(); COUNT_BARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNT_BARE(v: Py) { ensure_init(); if let Ok(mut w) = COUNT_BARE.write() { *w = v; } }
pub fn g_DAY_ASKS() -> Py { ensure_init(); DAY_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = DAY_ASKS.write() { *w = v; } }
pub fn g_DAY_IT() -> Py { ensure_init(); DAY_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_IT(v: Py) { ensure_init(); if let Ok(mut w) = DAY_IT.write() { *w = v; } }
pub fn g_DAY_OF_MONTH() -> Py { ensure_init(); DAY_OF_MONTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_OF_MONTH(v: Py) { ensure_init(); if let Ok(mut w) = DAY_OF_MONTH.write() { *w = v; } }
pub fn g_DAY_THING() -> Py { ensure_init(); DAY_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAY_THING(v: Py) { ensure_init(); if let Ok(mut w) = DAY_THING.write() { *w = v; } }
pub fn g_ELIDED_AGAIN() -> Py { ensure_init(); ELIDED_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELIDED_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = ELIDED_AGAIN.write() { *w = v; } }
pub fn g_ELIDED_ASK() -> Py { ensure_init(); ELIDED_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELIDED_ASK(v: Py) { ensure_init(); if let Ok(mut w) = ELIDED_ASK.write() { *w = v; } }
pub fn g_ELIDED_NAMES() -> Py { ensure_init(); ELIDED_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELIDED_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = ELIDED_NAMES.write() { *w = v; } }
pub fn g_ELIDED_NOT_SURE() -> Py { ensure_init(); ELIDED_NOT_SURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELIDED_NOT_SURE(v: Py) { ensure_init(); if let Ok(mut w) = ELIDED_NOT_SURE.write() { *w = v; } }
pub fn g_ELIDED_WORDS() -> Py { ensure_init(); ELIDED_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELIDED_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ELIDED_WORDS.write() { *w = v; } }
pub fn g_HOW_MANY_OF_MINE() -> Py { ensure_init(); HOW_MANY_OF_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_OF_MINE(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MANY_OF_MINE.write() { *w = v; } }
pub fn g_MEMORY_POINTER() -> Py { ensure_init(); MEMORY_POINTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEMORY_POINTER(v: Py) { ensure_init(); if let Ok(mut w) = MEMORY_POINTER.write() { *w = v; } }
pub fn g_MISSED_CATCH_UP() -> Py { ensure_init(); MISSED_CATCH_UP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MISSED_CATCH_UP(v: Py) { ensure_init(); if let Ok(mut w) = MISSED_CATCH_UP.write() { *w = v; } }
pub fn g_PLAN_ASKS() -> Py { ensure_init(); PLAN_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_ASKS.write() { *w = v; } }
pub fn g_PLAN_MINE() -> Py { ensure_init(); PLAN_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_MINE(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_MINE.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_SAID_ASK() -> Py { ensure_init(); SAID_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_ASK(v: Py) { ensure_init(); if let Ok(mut w) = SAID_ASK.write() { *w = v; } }
pub fn g_SAID_AS_A_SENTENCE() -> Py { ensure_init(); SAID_AS_A_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_A_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SAID_AS_A_SENTENCE.write() { *w = v; } }
pub fn g_THAT_DATE() -> Py { ensure_init(); THAT_DATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THAT_DATE(v: Py) { ensure_init(); if let Ok(mut w) = THAT_DATE.write() { *w = v; } }
pub fn g_TIME_ASKS() -> Py { ensure_init(); TIME_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = TIME_ASKS.write() { *w = v; } }
pub fn g_TIME_IT() -> Py { ensure_init(); TIME_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_IT(v: Py) { ensure_init(); if let Ok(mut w) = TIME_IT.write() { *w = v; } }
pub fn g_TIME_THING() -> Py { ensure_init(); TIME_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TIME_THING(v: Py) { ensure_init(); if let Ok(mut w) = TIME_THING.write() { *w = v; } }
pub fn g_TOPIC_ASK() -> Py { ensure_init(); TOPIC_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOPIC_ASK(v: Py) { ensure_init(); if let Ok(mut w) = TOPIC_ASK.write() { *w = v; } }
pub fn g_WEEKDAY_WORD() -> Py { ensure_init(); WEEKDAY_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEEKDAY_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WEEKDAY_WORD.write() { *w = v; } }
pub fn g_WHAT_I_SAID() -> Py { ensure_init(); WHAT_I_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_I_SAID.write() { *w = v; } }
pub fn g_WHEN_ASKS() -> Py { ensure_init(); WHEN_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_ASKS.write() { *w = v; } }
pub fn g_WHEN_END_ASK() -> Py { ensure_init(); WHEN_END_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_END_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_END_ASK.write() { *w = v; } }
pub fn g_WHEN_END_WORD() -> Py { ensure_init(); WHEN_END_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_END_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_END_WORD.write() { *w = v; } }
pub fn g_WHEN_INVERTED() -> Py { ensure_init(); WHEN_INVERTED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_INVERTED(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_INVERTED.write() { *w = v; } }
pub fn g_WHEN_INVERTED_SUBJECT() -> Py { ensure_init(); WHEN_INVERTED_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_INVERTED_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_INVERTED_SUBJECT.write() { *w = v; } }
pub fn g_WHEN_PLAN() -> Py { ensure_init(); WHEN_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_PLAN.write() { *w = v; } }
pub fn g_WHEN_QUALITY() -> Py { ensure_init(); WHEN_QUALITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_QUALITY(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_QUALITY.write() { *w = v; } }
pub fn g_WHEN_START_ASK() -> Py { ensure_init(); WHEN_START_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_START_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_START_ASK.write() { *w = v; } }
pub fn g_WHEN_THING() -> Py { ensure_init(); WHEN_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_THING(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_THING.write() { *w = v; } }
pub fn g_WHEN_THING_VERB() -> Py { ensure_init(); WHEN_THING_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_THING_VERB(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_THING_VERB.write() { *w = v; } }
pub fn g_WHEN_VAGUE() -> Py { ensure_init(); WHEN_VAGUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_VAGUE(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_VAGUE.write() { *w = v; } }
pub fn g_WHEN_VERB() -> Py { ensure_init(); WHEN_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_VERB(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_VERB.write() { *w = v; } }
pub fn g_WHERE_PLAN() -> Py { ensure_init(); WHERE_PLAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_PLAN(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_PLAN.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The constant tables of the fallback composer.\n\nLifted out of `bot/compose/fallback.py`, which is over the Hub's one-mebibyte file\ncap - one file over that cap stops *every* branch in the swarm from being uploaded,\nwhatever it changes. The same lift moved `bot/knowledge/home_diy.py`,\n`bot/modules/crisis_replies.py` and `bot/memory/patterns.py` out of their files.\n\nNothing here is code: `re` patterns, `frozenset`s of words and tuples of reply\nlines, moved as they were written and imported back by name in `fallback.py`.\nGenerated by `tools/lift_fallback_tables.py`, which also refuses to move anything\nwhose value reads a function or a mutable name of the file - run it again rather\nthan editing a table here and leaving the two out of step.\n");
    s_MISSED_CATCH_UP(stdlib_call("re", "compile", &[py_str("\\bwhat (?:have|had|did) i miss(?:ed)?\\b(?=\\s*(?:so far|then|here|yet|today|again)?\\s*[?.,!]*$)|\\b(?:have|did) i miss(?:ed)? anything\\b(?=\\s*[?.,!]*$)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TOPIC_ASK(stdlib_call("re", "compile", &[add(&add(&py_str("\\bwhat (?:were|are|was|did|do|have|had) we\\s+(?:talk|chat|discuss|mention|speak|spoke|say|said|cover)\\w*\\s*(?:about|of|on)\\b|\\bwhat (?:were|was|did|have|had) we\\s+(?:talk|chat|discuss|mention|speak|spoke|say|said|cover|do|done)\\w*(?=\\s*[?.!,]|\\s*$)|\\bwhat we\\s+(?:talk|chat|discuss|mention)\\w*\\s*(?:about|of|on)\\b|\\bwhat (?:were|are|was) (?:we|i|you) (?:talking|chatting|saying|on) about\\b|\\bwhat (?:were|are|was) (?:we|i|you) saying\\b|\\bwhere (?:were we|are we)(?=\\s*[?.!,]|\\s*$)|\\bwhere did we (?:leave|get to|stop)\\b|\\bwhat was i (?:saying|on about|asking|learning|studying|reading|researching|working on|up to)\\b|\\bwhat (?:was|is) (?:my|the|that) question\\b\\s*(?:again)?\\s*[?.!,]*$|\\bwhat question (?:did i (?:just )?ask|(?:was|is) (?:it|that))\\b|\\bwhat (?:was|were) (?:i|we|you) up to\\b|\\bwhat (?:am|was) i (?:learning|studying|reading|researching)\\b|\\bcatch me up\\b|\\brecap\\b|"), &gattr(&g_MISSED_CATCH_UP(), "pattern")), &py_str("|\\bremind me (?:what|where|how)\\b|\\bsum (?:up )?what we\\b|\\bwhat (?:did|have) we (?:talk|chat|say) about\\b|\\bwhat we (?:have|had) (?:covered|talked|discussed|done|said|been on)\\b|\\bwhat we were (?:on|up to)\\b|\\bwhat did i (?:just )?(?:ask|want to know)\\b(?![^.?!]*\\babout\\b(?!\\s+(?:me|it|that|this|them|us)\\b))|\\bwhat did i (?:just )?say\\b(?!\\s+(?:about|to|when|in|on|at|for|during|before|after|since|wrong)\\b)"))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WITHOUT_MY(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:pets?|dogs?|cats?|animals?) (?:do|did|have) i\\b|\\b(?:do|have|did) i (?:have|got) (?:any )?(?:pets?|dogs?|cats?|animals?|allergies|a budget)\\b|\\bam i allergic to\\b|\\b(?:anything|something) i(?:'m| am) allergic to\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_OF_MINE(stdlib_call("re", "compile", &[py_str("\\bhow many (?:pets?|dogs?|pupp(?:y|ies)|cats?|kittens?|animals?|rabbits?|hamsters?|parrots?|fish|horses?|pon(?:y|ies)) (?:do|did) (?:i|we) (?:have|got|own)\\b|\\bhow many (?:pets?|dogs?|pupp(?:y|ies)|cats?|kittens?|animals?|rabbits?|hamsters?|parrots?|fish|horses?|pon(?:y|ies)) (?:have|had) (?:i|we) got\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEMORY_POINTER(stdlib_call("re", "compile", &[py_str("\\b(?:again|remind me|did i (?:say|tell)|for a living|for work)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_ASKS(stdlib_call("re", "compile", &[add(&add(&add(&add(&add(&add(&py_str("\\bwhat (?:am i|are we) (?:planning|up to)\\b|\\bwhat (?:am i|are we) doing\\b(?!\\s+wrong)\\b|\\bwhat(?:'s| is|s)?\\s+(?:coming up|on for (?:me|us))\\b|\\bwhat (?:have|has) (?:i|we) got (?:coming up|on|planned|left)\\b|\\bwhat (?:have|has) (?:i|we) got to (?:do|sort|get|finish|arrange)\\b|\\bwhat (?:have|has) (?:i|we) got (?:in|on) (?:the|my|our) (?:diary|calendar)\\b|\\bwhat (?:have|has) (?:i|we) got\\s*[?.!,]*\\s*$|\\bwhat do (?:i|we) have (?:coming up|on|planned)\\b|\\banything (?:planned|coming up)\\b|\\banything on\\b"), &crate::gen::memory::store_lines::g_NOT_ON_A_MEDIUM()), &py_str("|\\bwhat am i looking forward to\\b|\\bwhat (?:was|were) (?:i|we) (?:planning|saving|saving up)\\b|\\bwhat (?:am i|are we|is) (?:saving|saving up)\\b|\\bwhat i (?:was|am) (?:planning|saving)\\b|\\bwhat(?:'s| is| was) my plan\\b")), &crate::gen::memory::store_lines::g_NOT_THE_PLAN()), &py_str("|\\bwhat was (?:the|my) plan\\b")), &crate::gen::memory::store_lines::g_NOT_THE_PLAN()), &py_str("|\\bwhat was i going to do\\b(?:\\s*then)?\\s*[?.!,]*$|\\bwhere (?:are|were|was|am) (?:we|i) (?:going|off|headed|heading|away)\\b|\\bwhen (?:are|were|was|am) (?:we|i) (?:off|going|away|moving|flying|travelling)\\b|\\b(?:which|what) (?:month|day|week|date) (?:are|am|is) (?:we|i)\\b|\\bremind me where\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_MINE(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\bwhat (?:was|is) my plan\\b|\\bwhat my plan (?:was|is)\\b|\\bwhat (?:did|do) i say (?:my plan|i was planning)\\b|\\bwhat (?:are|were) (?:my|our) plans?\\b"), &crate::gen::memory::store_lines::g_NOT_THE_PLAN()), &py_str("|\\bwhat plans (?:do|did) (?:i|we) have\\b")), &crate::gen::memory::store_lines::g_NOT_THE_PLAN()), &py_str("|\\b(?:do|did) (?:i|we) have (?:any |some )?plans?\\b|\\b(?:do|did) (?:i|we) have anything (?:planned|coming up|on)\\b|\\bhave (?:i|we) got (?:any |some )?(?:plans?|anything (?:planned|coming up|on))\\b|\\b(?:is|are) there anything (?:planned|coming up|on) for (?:me|us)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_ASKS(stdlib_call("re", "compile", &[py_str("\\bwhen (?:do|does|will|am|are) (?:i|we)\\s+(?:going to\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_ASKS(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:day|date) (?:do|does|will|am|are) (?:i|we)\\s+(?:going to\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_PLAN(stdlib_call("re", "compile", &[py_str("\\bwhere (?:are|were|was|am) (?:we|i) (?:going|off|headed|heading|away|moving|flying|travelling|traveling|driving)\\b|\\bwhere (?:we|i) (?:are|were|am|was) (?:going|off|away|moving|flying|travelling|traveling|driving)\\b|\\bwhere to again\\b|\\bwhere (?:are|were) we off to\\b|\\bremind me where\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_AS_A_SENTENCE(stdlib_call("re", "compile", &[py_str("^(?:we|i|we'?ve|i'?ve|we'?re|i'?m|we'?ll|i'?ll|we'?d|i'?d|we are|i am|we were|i was|my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_PLAN(stdlib_call("re", "compile", &[py_str("\\bwhen (?:are|were|was|am) (?:we|i) (?:off|going|away|moving|flying|travelling|traveling)\\b|\\b(?:which|what) (?:month|day|week|date|time of year) (?:are|am|is) (?:we|i)\\b|\\b(?:which|what) (?:month|day|week|date|time of year) (?:is|was) that\\b|\\bwhat date (?:are|am) (?:we|i)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THAT_DATE(stdlib_call("re", "compile", &[py_str("\\b(?:which|what) (?:date|day) (?:is|was) that\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR_SAID(stdlib_call("re", "compile", &[py_str("\\b\\d{1,2} (?:january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEEKDAY_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_OF_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:on the |the )?(\\d{1,2}(?:st|nd|rd|th))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_THING(stdlib_call("re", "compile", &[py_str("\\bwhen(?:'s| is| are| was| were)\\s+(?:the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_INVERTED(stdlib_call("re", "compile", &[py_str("^(?:when|when's)\\s+(the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)\\s+(is|are|was|were)\\b(?:[\\s,]*(?:again|please|then))?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_INVERTED_SUBJECT(stdlib_call("re", "compile", &[py_str("^when\\s+(i|we)\\s+((?:am|are|was|were|do|does|did|will|have|has)\\b.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_THING_VERB(stdlib_call("re", "compile", &[py_str("\\bwhen (?:does|do|did|will|would|can|could)\\s+(?:the|my|our|his|her|their|your)\\s+([a-z][a-z'-]*)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_END_ASK(call_builtin("frozenset", &[mcall(&py_str("end ends finish finishes last lasts leave leaves return returns come\narrive arrives land lands close closes stop stops expire expires wrap"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_START_ASK(call_builtin("frozenset", &[mcall(&py_str("start starts begin begins open opens kick"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_END_WORD(call_builtin("frozenset", &[mcall(&py_str("end ends finish finishes last lasts over expire expires close closes wrap wrapped"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_ASKS(stdlib_call("re", "compile", &[py_str("\\bwhat time (?:do|does|will|am|are) (?:i|we)\\s+(?:going to\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_THING(stdlib_call("re", "compile", &[py_str("\\bwhat time(?:'s| is| are| was| were)\\s+(?:the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_THING(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:day|date) (?:is|was|are|were)\\s+(?:the|my|our|his|her|their|your|a|an)\\s+([a-z][a-z'-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_DAY_IT(stdlib_call("re", "compile", &[py_str("\\b(?:what|which) (?:day|date) (?:is|was) (?:that|this)\\b|\\b(?:what|which) (?:day|date) (?:is|was) (?:it|that)\\s+(?:on|at|for|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_IT(stdlib_call("re", "compile", &[py_str("\\bwhat time (?:is|was|will be) (?:that|this)\\b|\\bwhat time (?:is|was) (?:it|that)\\s+(?:on|at|for|again)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_VAGUE(call_builtin("frozenset", &[mcall(&py_str("next last best first good right usual actual whole thing stuff one time"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_QUALITY(call_builtin("frozenset", &[mcall(&py_str("best good right usual actual"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_VERB(call_builtin("frozenset", &[mcall(&py_str("prep prepare get go leave book buy tell say send start take plant apply ask\ncall visit fly travel do make set put order renew sort pack move pay file check wrap post"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(call_builtin("frozenset", &[mcall(&py_str("it its them there that this these those one ones back again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ELIDED_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:did|do) i (?:say|call|name) (?:it|him|her|them|they|he|she)(?: (?:was|were|is|are))?(?: (?:called|named))?\\b|\\b(?:and )?what (?:was|is) (?:it|that) again\\b|\\b(?:and )?(?:what|which) (?:was|is) (?:its |his |her |their |the )?name(?: again)?\\b|\\b(?:and )?what did i (?:call|name) (?:him|her|them)\\b|\\bwhat (?:was|is) that called again\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ELIDED_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:where|when) (?:was|is) (?:it|that|that one)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ELIDED_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nand so but ok okay oh then well right what which where when was is did do i call called name\nsaid say it that this him her them they he she his hers its their the again to me you my of\nwere was that one\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ELIDED_NOT_SURE(stdlib_call("re", "compile", &[py_str("\\bhaven't (?:told|mentioned)\\b|\\bdon't know (?:it|your)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ELIDED_NAMES(py_tuple(vec![py_tuple(vec![py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:call|called|name|named)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("city"), stdlib_call("re", "compile", &[py_str("\\b(?:live|living|from|town|city|moved)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("job"), stdlib_call("re", "compile", &[py_str("\\b(?:work|working|job)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("age"), stdlib_call("re", "compile", &[py_str("\\b(?:old|age)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("budget"), stdlib_call("re", "compile", &[py_str("\\b(?:budget|spend|money|cost)\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("allergy"), stdlib_call("re", "compile", &[py_str("\\ballerg\\w*\\b")]).unwrap_or_else(|e| exc::raise(e))]), py_tuple(vec![py_str("pets"), stdlib_call("re", "compile", &[py_str("\\b(?:pet|pets|dog|cat|animal)\\b")]).unwrap_or_else(|e| exc::raise(e))])]));
    s_COUNT_ANIMAL(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok |okay |so |and |but |well |oh |quick |just )*how many\\s+(dogs?|cats?|puppies|puppy|kittens?|kitten|rabbits?|bunnies|bunny|hamsters?|fish|horses?|ponies|parrots?|pets?)\\b\\s*(?:do (?:i|we) have|have (?:i|we) got|did (?:i|we) say|do (?:i|we) own|have we|are there|(?:is|was|are|were) (?:that|it|this)(?: again| now)?|again|then|now)?\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_BARE(stdlib_call("re", "compile", &[py_str("^\\s*(?:ok |okay |so |and |but |well |oh |quick |just )*how many\\s+(?:do (?:i|we) have|have (?:i|we) got|do (?:i|we) own|are there|did (?:i|we) say|in total|altogether)?\\s*[?.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_I_SAID(stdlib_call("re", "compile", &[py_str("\\bwhat did i (?:just )?(?:say|tell you|mention)\\b|\\bwhat have i (?:said|mentioned)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat did i (?:just )?(?:say|tell you)(?: just)?(?: then| last| before| earlier| again)?\\s*\\??$|\\bwhat (?:was|is|were) (?:the |my )?(?:last|previous|most recent) (?:thing|bit|line|message|one) (?:that )?i (?:said|told you|mentioned)\\s*\\??$")]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.memory_tables".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ASK_WITHOUT_MY" => Some(g_ASK_WITHOUT_MY()),
            "CALENDAR_SAID" => Some(g_CALENDAR_SAID()),
            "COUNT_ANIMAL" => Some(g_COUNT_ANIMAL()),
            "COUNT_BARE" => Some(g_COUNT_BARE()),
            "DAY_ASKS" => Some(g_DAY_ASKS()),
            "DAY_IT" => Some(g_DAY_IT()),
            "DAY_OF_MONTH" => Some(g_DAY_OF_MONTH()),
            "DAY_THING" => Some(g_DAY_THING()),
            "ELIDED_AGAIN" => Some(g_ELIDED_AGAIN()),
            "ELIDED_ASK" => Some(g_ELIDED_ASK()),
            "ELIDED_NAMES" => Some(g_ELIDED_NAMES()),
            "ELIDED_NOT_SURE" => Some(g_ELIDED_NOT_SURE()),
            "ELIDED_WORDS" => Some(g_ELIDED_WORDS()),
            "HOW_MANY_OF_MINE" => Some(g_HOW_MANY_OF_MINE()),
            "MEMORY_POINTER" => Some(g_MEMORY_POINTER()),
            "MISSED_CATCH_UP" => Some(g_MISSED_CATCH_UP()),
            "PLAN_ASKS" => Some(g_PLAN_ASKS()),
            "PLAN_MINE" => Some(g_PLAN_MINE()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "SAID_ASK" => Some(g_SAID_ASK()),
            "SAID_AS_A_SENTENCE" => Some(g_SAID_AS_A_SENTENCE()),
            "THAT_DATE" => Some(g_THAT_DATE()),
            "TIME_ASKS" => Some(g_TIME_ASKS()),
            "TIME_IT" => Some(g_TIME_IT()),
            "TIME_THING" => Some(g_TIME_THING()),
            "TOPIC_ASK" => Some(g_TOPIC_ASK()),
            "WEEKDAY_WORD" => Some(g_WEEKDAY_WORD()),
            "WHAT_I_SAID" => Some(g_WHAT_I_SAID()),
            "WHEN_ASKS" => Some(g_WHEN_ASKS()),
            "WHEN_END_ASK" => Some(g_WHEN_END_ASK()),
            "WHEN_END_WORD" => Some(g_WHEN_END_WORD()),
            "WHEN_INVERTED" => Some(g_WHEN_INVERTED()),
            "WHEN_INVERTED_SUBJECT" => Some(g_WHEN_INVERTED_SUBJECT()),
            "WHEN_PLAN" => Some(g_WHEN_PLAN()),
            "WHEN_QUALITY" => Some(g_WHEN_QUALITY()),
            "WHEN_START_ASK" => Some(g_WHEN_START_ASK()),
            "WHEN_THING" => Some(g_WHEN_THING()),
            "WHEN_THING_VERB" => Some(g_WHEN_THING_VERB()),
            "WHEN_VAGUE" => Some(g_WHEN_VAGUE()),
            "WHEN_VERB" => Some(g_WHEN_VERB()),
            "WHERE_PLAN" => Some(g_WHERE_PLAN()),
            _ => None,
        }),
        set: None,
    }))
}

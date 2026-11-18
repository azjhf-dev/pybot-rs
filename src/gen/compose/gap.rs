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


pub static AN_AMOUNT_ALREADY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_A_COST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_SOMETHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BROKEN_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHOICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_COUNT_IN_A_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DISTANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DO_I_NEED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_DURATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_EVERY_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOWTO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HOWTO_HAS_A_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LOAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LONGER_SPAN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LOOSE_KIND_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MOVING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAMED_DOCUMENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NEED_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_POINTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PROCESS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_QUEUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPAIR_OR_BUY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_REPEATED_SPEND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RULE_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_STAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_TROUBLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WORRY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WORTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_YEAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_DURATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHOICE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COUNT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISTANCE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DURATION_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXAMPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOWTO_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KINDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOAD_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVING_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_ONLY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEED_TO_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_HOWTO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_PRICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_RULE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_HERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRICE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PROCEDURE_SHAPES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RATE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECORD_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULE_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_PAPER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PAPER_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORRY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORRY_SHAPES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORTH_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_AN_AMOUNT_ALREADY() -> Py { ensure_init(); AN_AMOUNT_ALREADY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AN_AMOUNT_ALREADY(v: Py) { ensure_init(); if let Ok(mut w) = AN_AMOUNT_ALREADY.write() { *w = v; } }
pub fn g_ASKS_A_COST() -> Py { ensure_init(); ASKS_A_COST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_COST(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_A_COST.write() { *w = v; } }
pub fn g_ASKS_SOMETHING() -> Py { ensure_init(); ASKS_SOMETHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_SOMETHING(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_SOMETHING.write() { *w = v; } }
pub fn g_A_BROKEN_THING() -> Py { ensure_init(); A_BROKEN_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BROKEN_THING(v: Py) { ensure_init(); if let Ok(mut w) = A_BROKEN_THING.write() { *w = v; } }
pub fn g_A_CHOICE() -> Py { ensure_init(); A_CHOICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHOICE(v: Py) { ensure_init(); if let Ok(mut w) = A_CHOICE.write() { *w = v; } }
pub fn g_A_COUNT_IN_A_SPAN() -> Py { ensure_init(); A_COUNT_IN_A_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_IN_A_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = A_COUNT_IN_A_SPAN.write() { *w = v; } }
pub fn g_A_DISTANCE() -> Py { ensure_init(); A_DISTANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DISTANCE(v: Py) { ensure_init(); if let Ok(mut w) = A_DISTANCE.write() { *w = v; } }
pub fn g_A_DO_I_NEED() -> Py { ensure_init(); A_DO_I_NEED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DO_I_NEED(v: Py) { ensure_init(); if let Ok(mut w) = A_DO_I_NEED.write() { *w = v; } }
pub fn g_A_DURATION() -> Py { ensure_init(); A_DURATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_DURATION(v: Py) { ensure_init(); if let Ok(mut w) = A_DURATION.write() { *w = v; } }
pub fn g_A_EVERY_SPAN() -> Py { ensure_init(); A_EVERY_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_EVERY_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = A_EVERY_SPAN.write() { *w = v; } }
pub fn g_A_FIT() -> Py { ensure_init(); A_FIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FIT(v: Py) { ensure_init(); if let Ok(mut w) = A_FIT.write() { *w = v; } }
pub fn g_A_HOWTO() -> Py { ensure_init(); A_HOWTO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOWTO(v: Py) { ensure_init(); if let Ok(mut w) = A_HOWTO.write() { *w = v; } }
pub fn g_A_HOWTO_HAS_A_SHAPE() -> Py { ensure_init(); A_HOWTO_HAS_A_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HOWTO_HAS_A_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = A_HOWTO_HAS_A_SHAPE.write() { *w = v; } }
pub fn g_A_LOAD() -> Py { ensure_init(); A_LOAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LOAD(v: Py) { ensure_init(); if let Ok(mut w) = A_LOAD.write() { *w = v; } }
pub fn g_A_LONGER_SPAN() -> Py { ensure_init(); A_LONGER_SPAN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LONGER_SPAN(v: Py) { ensure_init(); if let Ok(mut w) = A_LONGER_SPAN.write() { *w = v; } }
pub fn g_A_LOOSE_KIND_WORD() -> Py { ensure_init(); A_LOOSE_KIND_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LOOSE_KIND_WORD(v: Py) { ensure_init(); if let Ok(mut w) = A_LOOSE_KIND_WORD.write() { *w = v; } }
pub fn g_A_MOVING() -> Py { ensure_init(); A_MOVING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MOVING(v: Py) { ensure_init(); if let Ok(mut w) = A_MOVING.write() { *w = v; } }
pub fn g_A_NAMED_DOCUMENT() -> Py { ensure_init(); A_NAMED_DOCUMENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_DOCUMENT(v: Py) { ensure_init(); if let Ok(mut w) = A_NAMED_DOCUMENT.write() { *w = v; } }
pub fn g_A_NEED_TO() -> Py { ensure_init(); A_NEED_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NEED_TO(v: Py) { ensure_init(); if let Ok(mut w) = A_NEED_TO.write() { *w = v; } }
pub fn g_A_POINTER() -> Py { ensure_init(); A_POINTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_POINTER(v: Py) { ensure_init(); if let Ok(mut w) = A_POINTER.write() { *w = v; } }
pub fn g_A_PRICE() -> Py { ensure_init(); A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE.write() { *w = v; } }
pub fn g_A_PROCESS() -> Py { ensure_init(); A_PROCESS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PROCESS(v: Py) { ensure_init(); if let Ok(mut w) = A_PROCESS.write() { *w = v; } }
pub fn g_A_QUEUE() -> Py { ensure_init(); A_QUEUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_QUEUE(v: Py) { ensure_init(); if let Ok(mut w) = A_QUEUE.write() { *w = v; } }
pub fn g_A_RATE() -> Py { ensure_init(); A_RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RATE(v: Py) { ensure_init(); if let Ok(mut w) = A_RATE.write() { *w = v; } }
pub fn g_A_RECORD() -> Py { ensure_init(); A_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = A_RECORD.write() { *w = v; } }
pub fn g_A_REPAIR_OR_BUY() -> Py { ensure_init(); A_REPAIR_OR_BUY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPAIR_OR_BUY(v: Py) { ensure_init(); if let Ok(mut w) = A_REPAIR_OR_BUY.write() { *w = v; } }
pub fn g_A_REPEATED_SPEND() -> Py { ensure_init(); A_REPEATED_SPEND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_REPEATED_SPEND(v: Py) { ensure_init(); if let Ok(mut w) = A_REPEATED_SPEND.write() { *w = v; } }
pub fn g_A_RULE() -> Py { ensure_init(); A_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RULE(v: Py) { ensure_init(); if let Ok(mut w) = A_RULE.write() { *w = v; } }
pub fn g_A_RULE_SUBJECT() -> Py { ensure_init(); A_RULE_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RULE_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = A_RULE_SUBJECT.write() { *w = v; } }
pub fn g_A_STAT() -> Py { ensure_init(); A_STAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_STAT(v: Py) { ensure_init(); if let Ok(mut w) = A_STAT.write() { *w = v; } }
pub fn g_A_TROUBLE() -> Py { ensure_init(); A_TROUBLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_TROUBLE(v: Py) { ensure_init(); if let Ok(mut w) = A_TROUBLE.write() { *w = v; } }
pub fn g_A_WORRY_ASK() -> Py { ensure_init(); A_WORRY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WORRY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_WORRY_ASK.write() { *w = v; } }
pub fn g_A_WORTH() -> Py { ensure_init(); A_WORTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WORTH(v: Py) { ensure_init(); if let Ok(mut w) = A_WORTH.write() { *w = v; } }
pub fn g_A_YEAR() -> Py { ensure_init(); A_YEAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_YEAR(v: Py) { ensure_init(); if let Ok(mut w) = A_YEAR.write() { *w = v; } }
pub fn g_BARE_DURATION() -> Py { ensure_init(); BARE_DURATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_DURATION(v: Py) { ensure_init(); if let Ok(mut w) = BARE_DURATION.write() { *w = v; } }
pub fn g_CHOICE_LINES() -> Py { ensure_init(); CHOICE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHOICE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = CHOICE_LINES.write() { *w = v; } }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); COUNT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = COUNT_WORDS.write() { *w = v; } }
pub fn g_DISTANCE_LINES() -> Py { ensure_init(); DISTANCE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISTANCE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DISTANCE_LINES.write() { *w = v; } }
pub fn g_DURATION_LINES() -> Py { ensure_init(); DURATION_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DURATION_LINES(v: Py) { ensure_init(); if let Ok(mut w) = DURATION_LINES.write() { *w = v; } }
pub fn g_EXAMPLE() -> Py { ensure_init(); EXAMPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXAMPLE(v: Py) { ensure_init(); if let Ok(mut w) = EXAMPLE.write() { *w = v; } }
pub fn g_FIT_LINES() -> Py { ensure_init(); FIT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FIT_LINES.write() { *w = v; } }
pub fn g_HOWTO_LINES() -> Py { ensure_init(); HOWTO_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOWTO_LINES(v: Py) { ensure_init(); if let Ok(mut w) = HOWTO_LINES.write() { *w = v; } }
pub fn g_KINDS() -> Py { ensure_init(); KINDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); if let Ok(mut w) = KINDS.write() { *w = v; } }
pub fn g_LOAD_LINES() -> Py { ensure_init(); LOAD_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOAD_LINES(v: Py) { ensure_init(); if let Ok(mut w) = LOAD_LINES.write() { *w = v; } }
pub fn g_MOVING_LINES() -> Py { ensure_init(); MOVING_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVING_LINES(v: Py) { ensure_init(); if let Ok(mut w) = MOVING_LINES.write() { *w = v; } }
pub fn g_NAME_ONLY_ASK() -> Py { ensure_init(); NAME_ONLY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_ONLY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NAME_ONLY_ASK.write() { *w = v; } }
pub fn g_NEED_TO_LINES() -> Py { ensure_init(); NEED_TO_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEED_TO_LINES(v: Py) { ensure_init(); if let Ok(mut w) = NEED_TO_LINES.write() { *w = v; } }
pub fn g_NOT_A_HOWTO() -> Py { ensure_init(); NOT_A_HOWTO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_HOWTO(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_HOWTO.write() { *w = v; } }
pub fn g_NOT_A_PRICE() -> Py { ensure_init(); NOT_A_PRICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_PRICE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_PRICE.write() { *w = v; } }
pub fn g_NOT_A_RECORD() -> Py { ensure_init(); NOT_A_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_RECORD.write() { *w = v; } }
pub fn g_NOT_A_RULE() -> Py { ensure_init(); NOT_A_RULE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RULE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_RULE.write() { *w = v; } }
pub fn g_NOT_HERE() -> Py { ensure_init(); NOT_HERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_HERE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_HERE.write() { *w = v; } }
pub fn g_PRICE_LINES() -> Py { ensure_init(); PRICE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRICE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = PRICE_LINES.write() { *w = v; } }
pub fn g_PROCEDURE_SHAPES() -> Py { ensure_init(); PROCEDURE_SHAPES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PROCEDURE_SHAPES(v: Py) { ensure_init(); if let Ok(mut w) = PROCEDURE_SHAPES.write() { *w = v; } }
pub fn g_RATE_LINES() -> Py { ensure_init(); RATE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RATE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = RATE_LINES.write() { *w = v; } }
pub fn g_RECORD_LINES() -> Py { ensure_init(); RECORD_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECORD_LINES(v: Py) { ensure_init(); if let Ok(mut w) = RECORD_LINES.write() { *w = v; } }
pub fn g_RULE_LINES() -> Py { ensure_init(); RULE_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULE_LINES(v: Py) { ensure_init(); if let Ok(mut w) = RULE_LINES.write() { *w = v; } }
pub fn g_STAT_LINES() -> Py { ensure_init(); STAT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = STAT_LINES.write() { *w = v; } }
pub fn g_THEIR_OWN_PAPER() -> Py { ensure_init(); THEIR_OWN_PAPER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_PAPER(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_PAPER.write() { *w = v; } }
pub fn g_THE_PAPER_ITSELF() -> Py { ensure_init(); THE_PAPER_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PAPER_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = THE_PAPER_ITSELF.write() { *w = v; } }
pub fn g_WORRY_LINES() -> Py { ensure_init(); WORRY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORRY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = WORRY_LINES.write() { *w = v; } }
pub fn g_WORRY_SHAPES() -> Py { ensure_init(); WORRY_SHAPES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORRY_SHAPES(v: Py) { ensure_init(); if let Ok(mut w) = WORRY_SHAPES.write() { *w = v; } }
pub fn g_WORTH_LINES() -> Py { ensure_init(); WORTH_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORTH_LINES(v: Py) { ensure_init(); if let Ok(mut w) = WORTH_LINES.write() { *w = v; } }
fn module_body() {
    let _ = py_str("What the bot knows about the kind of question, when it does not know the answer.\n\n`findout.py` reads the asks whose answer lives in one named place - today's hours, the queue, a\nstatus, a form, something near the person - and says the place. It reads them from the thing in\nthe line (\"the tip\", \"my council\", \"near me\"), so it can only answer where a thing is named.\n\nThe everyday asks that carry no such thing got the plain hedge instead:\n\n    **User:** What does a structural engineer cost?\n    **Bot:** I'm not sure about that one, sorry - I'd rather not guess. A quick search online\n    will get you a solid answer.\n\n    **User:** How many people live in Sheffield?\n    **Bot:** No number for that here, sorry, and I'd rather not invent one. The Wikipedia page\n    or the official site will have it.\n\nA pointer is not a reply (STYLE.md, \"Not knowing, honestly\"): the bot does know what that kind\nof question turns on - a job is quoted rather than priced off a list, a population figure moves\nwith the year and the boundary drawn round it - and that half is worth saying before the place\nthat settles the rest. So this module reads the kind of ask from its shape and answers with the\ntwo halves in order: what decides it, then where the rest is.\n\nNothing here is a fact about the world. Not one line holds a price, a date, a figure or a name,\nso nothing can be invented and nothing goes stale. A line that is not one of these kinds gets\nNone and keeps whatever the composer had, and a kind is skipped when the shape is not the\nquestion (\"how much paracetamol can i take\" is a dose, not a price).\n");
    s_A_MOVING(stdlib_call("re", "compile", &[py_str("\\b(?:exchange|interest|mortgage|savings|base|bank) ?rates?\\b|\\brate of exchange\\b|\\bfuel prices?\\b|\\b(?:petrol|diesel) prices?\\b|\\bprice of (?:petrol|diesel|fuel)\\b|\\bhow much is (?:petrol|diesel|fuel)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVING_LINES(py_tuple(vec![py_str("That figure is republished every day and two places rarely quote the same one, because each adds its own margin or terms on top: compare two of them and go with the one whose small print fits what you are actually doing, checking the date printed with the number."), py_str("Nobody keeps that number in their head - it moves daily and the version printed next to it is the only one that counts. Read two offers rather than one, and add on the fee or the terms before you decide which is really cheaper.")]));
    s_A_PRICE(stdlib_call("re", "compile", &[py_str("\\bhow much\\b|\\bwhat(?:'s| is| does| would| will)?\\b[^.?!]*\\bcosts?\\b|\\bcost of\\b|\\bprice of\\b|\\bprices?\\b|\\bcheaper\\b|\\bcheapest\\b|\\bset me back\\b|\\bbudget for\\b|\\bgoing rate\\b|\\bthe rate for\\b|\\bwhat do you pay for\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_ONLY_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:the\\s+)?(?:price|cost)s?\\s+(?:of|for)\\b|\\bgoing rate\\b|\\bthe rate for\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_PRICE(stdlib_call("re", "compile", &[py_str("\\bhow much\\b[^.?!]{0,32}\\b(?:weigh|weighs|weight|sleep|eat|drink|take|give|feed|exercise|water|milk|protein|salt|sugar|fibre|calories|units|grams|ml|litres|paracetamol|ibuprofen|is safe|are safe|is too much|is normal|is enough)\\b|\\bhow much\\b[^.?!]{0,20}\\b(?:can|should|could) i\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPEATED_SPEND(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|twice|three times|four times|every|each)\\b[^.?!]{0,24}?\\b(?:a|per|each|every)\\s+(?:day|week|month)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LONGER_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?:in|over|for|across)\\s+a\\s+year\\b|\\ba\\s+year\\b|\\bper\\s+year\\b|\\bannual(?:ly)?\\b|\\bin\\s+(?:twelve|12)\\s+months\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_COST(stdlib_call("re", "compile", &[py_str("\\bhow much\\b|\\bcosts?\\b|\\bcosting\\b|\\bspend\\b|\\bspent\\b|\\bafford\\b|\\bexpensive\\b|\\bset me back\\b|\\bcome to\\b|\\bcomes to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AN_AMOUNT_ALREADY(stdlib_call("re", "compile", &[py_str("[£$]|\\b\\d+\\s*(?:p|pence|pounds?|quid|bucks|euros?)\\b|\\b(?:fiver|tenner|twenty)\\b|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten)\\s+(?:pounds?|quid)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_IN_A_SPAN(stdlib_call("re", "compile", &[py_str("\\b(\\d+|one|two|three|four|five|six|seven|twice|three times|four times|five times)\\s*(?:days?|times|nights?|lunches|cups?|pints?|cigarettes?|takeaways?|journeys?|trips?|rides?|lessons?|bottles?|coffees?)?\\s*(?:a|per|each|every)\\s+(day|week|month)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_EVERY_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?:every|each)\\s+(day|week|month)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("twice"), Py::Int(2)), (py_str("three times"), Py::Int(3)), (py_str("four times"), Py::Int(4)), (py_str("five times"), Py::Int(5))]));
    s_A_YEAR(py_dict(vec![(py_str("day"), Py::Int(365)), (py_str("week"), Py::Int(52)), (py_str("month"), Py::Int(12))]));
    s_EXAMPLE(Py::Int(5));
    s_PRICE_LINES(py_tuple(vec![py_str("There is no list price for that: what it comes to depends on the size of the job, the state of the thing and when you want it, so ask two firms to price the same description in writing and compare what each one leaves out rather than the total."), py_str("The figure comes from a quote or a listing, not from me: get the same description from two places, in writing, and check what each one includes - the parts, the labour, the call-out and the VAT.")]));
    s_A_RATE(stdlib_call("re", "compile", &[py_str("\\b(?:parking|car ?park|to park|park there|entry|admission|tickets?|fares?|day pass|week pass|season ticket|membership|toll|ferry|crossing|per night|a night|per hour|per day|day rate|hourly rate)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RATE_LINES(py_tuple(vec![py_str("That is a published rate rather than a quote: whoever runs the place sets it, it moves with the day and the season, and a cap or a longer stay usually costs less per hour. Their own page or listing has today's, so ask what the whole stretch comes to rather than the headline figure."), py_str("The headline rate is only half of it: an evening or weekend price, a day cap, a return or a longer booking often costs less than paying by the hour, and the operator's own page lists them all. Check today's figure there before you pay.")]));
    s_A_WORTH(stdlib_call("re", "compile", &[py_str("\\bworth it\\b|\\bworth the money\\b|\\bworth buying\\b|\\bworth the extra\\b|\\bworth paying\\b|\\bworth the price\\b|\\bgood buy\\b|\\bis it worth\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPAIR_OR_BUY(stdlib_call("re", "compile", &[py_str("\\b(?:fix|fixes|fixing|fixed|mend|mends|mending|mended|repair|repairs|repairing|repaired|service|services|servicing|serviced|replace|replaces|replacing|replacement|replaced|new one|second hand|secondhand|refurb(?:ish|ished)?|upgrade|upgrading|claim|claims|claiming|claimed|install|installs|installing|buy|buys|buying|bought|worth the money|worth buying|good buy|worth paying|worth the price|paying extra|pay extra)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BROKEN_THING(stdlib_call("re", "compile", &[py_str("\\b(?:broken|breaking|broke|not working|stopped working|gone wrong|last legs|leaking|faulty|dead|dead as|knackered|worn out|on the blink|cracked|rusting|needs? a repair|needs? fixing|needs? replacing|needs? a service|mend it|fix it|replace it|repair it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORTH_LINES(py_tuple(vec![py_str("What decides it is the condition, what it would cost to put right and what the new one costs: if the repair comes to more than about half of new, it usually is not worth it. Get the repair cost in writing first, then compare that with the price of a new one."), py_str("It earns its price when what it needs doing comes to less than replacing it, and a second one of the same age has its own faults. Ask for the repair cost in writing, and price the new one before you decide rather than after.")]));
    s_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\bwhich is better\\b|\\bwhich one is better\\b|\\bbetter to (?:get|buy|go for)\\b|\\bor should i\\b|\\bshould i (?:get|buy|go for)\\b[^.?!]{0,40}\\bor\\b|\\bvs\\b|\\b(?:pick|choose|decide|select) between\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHOICE_LINES(py_tuple(vec![py_str("Both work for somebody, so it comes down to what you would miss if you had to give the other one back, and which of them fits the week you actually have. Write down what each one is for before you compare what they cost, and the one you have to keep justifying is usually the answer."), py_str("The way to settle it is which one you would still be glad of in a year, and whether either of them brings a running cost with it - so list what you would use each one for, then compare the whole cost rather than the price on the ticket.")]));
    s_A_DISTANCE(stdlib_call("re", "compile", &[py_str("\\bhow far\\b|\\bmiles? from\\b|\\bmiles? to\\b|\\bhow long is the (?:drive|journey|walk)\\b|\\bdistance (?:from|to|between)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DISTANCE_LINES(py_tuple(vec![py_str("The straight-line miles and the road miles are two different numbers, and the time bends with the hour you travel, so put both ends into a map app and read the route it gives: it carries the miles, the time and the way round the slow bit."), py_str("For a journey by road that is a map app, and for one by train it is the operator's own planner: both give the miles or the connections for the hour you would actually go, which is the part a rough figure from anyone else gets wrong.")]));
    s_A_DURATION(stdlib_call("re", "compile", &[py_str("\\bhow long (?:does|do|will|would|should|until|before|after)\\b|\\bhow long until\\b|\\btakes? how long\\b|\\bhow soon\\b|\\bhow many (?:hours|days|weeks|months|years)\\b[^.?!]{0,20}\\b(?:does|do|will|until|before|after|should i wait)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DURATION(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |ok |okay )?(?:how long|how soon)\\b[\\s\\w]{0,24}?\\b(?:it|that|this|they|them|one)\\b[\\s\\w]{0,10}?\\??$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUEUE(stdlib_call("re", "compile", &[py_str("\\b(?:claim|claims|application|apply|form|forms|paperwork|order|delivery|courier|post|posting|post office|queue|backlog|repair|repairs|service|servicing|mot|survey|surveyor|solicitor|conveyanc|completion|exchange of contracts|offer|mortgage|valuation|passport|licence|license|visa|council|benefit|benefits|universal credit|pip|esa|refund|return|returns|dispute|appeal|hearing|decision|assessment|builder|builders|plasterer|electrician|plumber|joiner|installer|install|installation|dispatch|boiler|house move|sale|sell|selling|buying|purchase|probate|will|tenancy|landlord|deposit|dbs|background check|bank|payment|transfer|clearing|money|clears|clearing|wait|waits|waiting)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DURATION_LINES(py_tuple(vec![py_str("How long that takes depends on the queue, how much there is to do and the time of year, so ask the body that runs it for their current wait and whether the date they give includes the slow part at the end - a range from them beats any single figure."), py_str("The honest answer is a range: the size of it and how busy they are move it by weeks, so ask whoever handles it what they are running at now, and treat the earliest date they give as the best case rather than the promise.")]));
    s_A_LOAD(stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*-?\\s*amp\\b|\\bextension lead\\b|\\bplug socket\\b|\\bfuse\\b|\\btripp(?:ed|ing)\\b|\\btrips?\\b\\s+(?:the\\s+|a\\s+|our\\s+|my\\s+|your\\s+)?(?:fuse|circuit|breaker|mcb|rcd|socket|supply|switch|electric\\w*|mains|ring)\\b|\\b(?:off|from|on|into)\\s+(?:one|the same|a single)\\s+(?:socket|plug|circuit|extension)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOAD_LINES(py_tuple(vec![py_str("The labels settle it: the watts are printed on the base of each thing, and a 13-amp socket gives about 3,000 watts in total, so the sum is what counts rather than the number of plugs. Add up the labels and keep the total under that - a kettle or a heater on its own takes most of it."), py_str("Add up the watts on the labels of everything that would be on at once and keep the total under about 3,000 for a 13-amp socket, because a heater or a kettle alone takes most of the headroom. The rating is usually moulded into the plug or the base, not on the box.")]));
    s_A_FIT(stdlib_call("re", "compile", &[py_str("\\b(?:will|would|does|do|can)\\b[^?!]{0,30}\\b(?:fit|fits)\\b|\\b(?:work|works|go) with\\b[^.?!]{0,20}\\b(?:battery|charger|drill|hose|part|filter|attachment|unit|set|kit)\\b|\\bwhich\\s+(?:size|model|part|one)\\b[^?!]{0,20}\\bdo i need\\b|\\bwhat size\\b|\\bis (?:it|this|that) compatible\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIT_LINES(py_tuple(vec![py_str("The size and the model on the label decide it rather than the picture: the model number is usually on a plate underneath, in the manual or on the maker's page for it. Read that before anything is ordered, because the same name comes in more than one size."), py_str("It is the measurements and the model number that settle it, and both are on the thing itself, so note them down and match them against the spec on the maker's own page for the part - that is where the fit is stated rather than guessed at.")]));
    s_A_RULE(stdlib_call("re", "compile", &[py_str("\\bam i allowed\\b|\\bare we allowed\\b|\\bis it legal\\b|\\bis that legal\\b|\\bdo i need (?:a |an |permission|to )\\b|\\bcan i be made to\\b|\\bcan they make me\\b|\\bhave i got the right\\b|\\bwhat are my rights\\b|\\bdo i have to\\b|\\bam i entitled\\b|\\bis it against the law\\b|\\bcan my (?:landlord|employer|boss)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_RULE(stdlib_call("re", "compile", &[py_str("\\b(?:uno|monopoly|chess|poker|scrabble|darts|snooker|board game|bishop|knight|rook|pawn|checkmate|the pieces)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DO_I_NEED(stdlib_call("re", "compile", &[py_str("\\bdo i need (?:a |an |to )\\b|\\bdo i have to\\b|\\bdo we need to\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RULE_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:licen[cs]e|licen[cs]ed|permit|permission|visa|insurance|insurer|mot\\b|tax|taxed|deposit|notice|contract|landlord|tenant|letting agent|employer|boss|\\bhr\\b|council|council tax|hmrc|gov\\.uk|government|court|probate|benefits?|universal credit|housing benefit|claim|claiming|form|forms|declaration|statement|registration|register|passport|inspection|consent|referral|prescription|legal|law|rights|entitled|mortgage|solicitor|conveyanc\\w*|certificate|crime number|vet\\b|policy|warranty|guarantee|refund|return it|duty|obliged|obligation)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RULE_LINES(py_tuple(vec![py_str("Whether that is allowed turns on the rules as they stand this year and on your own case, so the current wording decides it rather than what anyone has heard. GOV.UK has the national version and your council's site the local one, and the office that runs it will put the conditions for your situation in writing - keep that letter."), py_str("It depends on what your own agreement says and on the version of the rules in force, and the gap between those two is usually the whole answer. The issuing office's own page has the current wording, and one call to them settles the case the page does not cover.")]));
    s_A_STAT(stdlib_call("re", "compile", &[py_str("\\bhow many (?:people|residents|households|of them|are there)\\b|\\bwhat percentage\\b|\\bwhat proportion\\b|\\bwhat(?:'s|s| is)?\\s*(?:is\\s+)?the population\\b|\\bpopulation of\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STAT_LINES(py_tuple(vec![py_str("Figures like that come from whoever counts them, and the year and the boundary change the answer: \"the town\" and \"the wider area\" are two different numbers. The official statistics page or the council's own figures have the current one, with the year it is for printed beside it."), py_str("That number moves every year and it depends where the line is drawn, so the body that publishes it is the one to quote: the national statistics site has it with the year, and the local council publishes its own for the area it covers.")]));
    s_A_HOWTO(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|should|could|would) (?:i|you|we)\\b|\\bhow to\\b|\\bbest way to\\b|\\bany (?:tips|advice) (?:for|on)\\b|\\bwhat should i do about\\b|\\bwhere do i start\\b|\\bhow would you go about\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TROUBLE(stdlib_call("re", "compile", &[py_str("\\b(?:stuck|stubborn|tight|seized|jammed|wedged|won'?t|wont|not working|stopped working|broke|broken|leak|drip|damp|mould|mold|smell|stink|stain|crease|creak|squeak|squeal|rattl|wobbl|vibrat|noisy|block|blocked|slow|stiff|rust|scratch|burn|fade|crack|chip|snap|worn|tangled|knotted|fog|cloudy|flat|dead|flicker|trip|overflow|keeps (?:going|coming|stopping|turning))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PROCESS(stdlib_call("re", "compile", &[py_str("\\b(?:will|probate|benefits?|universal credit|council tax|licence|license|permit|passport|visa|tax return|self assessment|pension|redundancy|redundant|apply|application|appeal|complain|complaint|refund|return|cancel|cancelling|mortgage|insurance|insure|claim|registration|register|deregister|loyalty|warranty|guarantee|bill|bills|direct debit|standing order|switch|switching|comparison|quote|quotes|contract|agreement|tenancy|deposit|moving house|house move|move house|solicitor|notary|deed|title|land registry)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOWTO_HAS_A_SHAPE(stdlib_call("re", "compile", &[add(&add(&gattr(&g_A_TROUBLE(), "pattern"), &py_str("|")), &gattr(&g_A_PROCESS(), "pattern")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_HOWTO(stdlib_call("re", "compile", &[py_str("\\b(?:feel|feeling|feelings|anxious|anxiety|depressed|depression|panic|grief|grieving|lonely|loneliness|scared|worried|worry|miscarriage|divorce|separating)\\b|\\b(?:tell|talk to|speak to|say to|ask|apologise to|apologize to|comfort|support|help|explain to|break it to|be there for|deal with|do about)\\b[^?!]{0,24}\\b(?:my|his|her|their)\\s+(?:mum|mom|dad|son|daughter|wife|husband|partner|boyfriend|girlfriend|friend|brother|sister|boss|child|kid|parents|family)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HOWTO_LINES(py_tuple(vec![py_str("The first steps are the same for most of these: work out what is actually causing it before you treat what you can see, change one thing at a time, and give each change a day before the next so you know which one worked. Write down what you tried and when."), py_str("Do it in the order that costs least first: what you already have in the cupboard or the shed, then the cheap part or product, then the trade. Before you buy anything, check the manual or the maker's page for the exact model, because it says what is safe on that one and what is not.")]));
    s_A_RECORD(stdlib_call("re", "compile", &[py_str("\\b(?:deposit|deeds?|title plan|land registry|warrant(?:y|ies)|guarantee|receipt|invoice|policy|insurance|cover note|booking|reservation|tenancy agreement|assured shorthold|certificate|statement|reference number|reference|paperwork|the small print|terms|boundary|boundaries)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_RECORD(stdlib_call("re", "compile", &[py_str("\\b(?:whats|what is|what are|what does|what do|meaning of)\\s+(?:a|an)\\b|\\bwhat\\s+(?:does|do|is|are|did|can)\\s+(?:the|this|that)\\b[^.?!]{0,40}\\b(?:mean|means|meaning|stand for|definition)\\b|\\b(?:how do i|how to|should i (?:take out|buy|get)|worth (?:getting|it)|cheapest|cancel my|switch my|compare)\\b|\\bwhats? the point of\\b|\\bwhat is the point of\\b|\\bsick of\\b|\\bfed up\\b|\\bdrowning in\\b|\\bcan'?t be (?:bothered|arsed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOOSE_KIND_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:policy|policies|statement|statements|terms|paperwork|insurance|boundar(?:y|ies)|contract|agreement)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_OWN_PAPER(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|his|her|their|your|yours|ours|theirs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAMED_DOCUMENT(stdlib_call("re", "compile", &[py_str("\\b(?:terms and conditions|terms of (?:the|my|our|their|his|her|your)\\b|insurance polic(?:y|ies)|policy wording|policy document|policy schedule|bank statement|statement of (?:the|my|our|their)\\b|tenancy agreement|assured shorthold|contract of employment|the small print)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECORD_LINES(py_tuple(vec![py_str("What settles that is the entry itself rather than the general rule - the date on it, the terms that were in force then, and the name it is in. The office or firm that holds it can read it back to you, so ask for it in writing and quote the reference on it: the answer that counts is the one on paper, not the one given over the phone."), py_str("That turns on the wording of the document, not on what is usually done: what it says, what date it came into force and whose name is on it. Whoever issued it keeps the record, and the one thing worth doing first is asking them to confirm in writing what their copy says."), py_str("Nobody can answer that from the description of it, because the answer is in the paperwork - the date, the terms and the name on it. Ask the body holding it for the entry in writing with the reference on it, and keep what they send: a written answer is the one that can be relied on later.")]));
    s_NOT_HERE(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|111|ambulance|a&e|emergency|bleeding|not breathing|chest pain|overdose|suicide|kill myself|end my life|self harm|died|death|funeral|buried|cremation|passed away|grief|grieving)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_SOMETHING(stdlib_call("re", "compile", &[py_str("\\b(?:how|whats?|what's|which|when|where|who|why|can|could|should|would|will|is|are|was|were|do|does|did|am|has|have|had|may|might|shall)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KINDS(py_tuple(vec![py_tuple(vec![py_str("choice"), g_A_CHOICE(), g_CHOICE_LINES(), Py::None]), py_tuple(vec![py_str("howto"), g_A_HOWTO(), g_HOWTO_LINES(), g_NOT_A_HOW_TO()]), py_tuple(vec![py_str("worth"), g_A_WORTH(), g_WORTH_LINES(), g_not_about_one_owned_thing()]), py_tuple(vec![py_str("distance"), g_A_DISTANCE(), g_DISTANCE_LINES(), Py::None]), py_tuple(vec![py_str("load"), g_A_LOAD(), g_LOAD_LINES(), Py::None]), py_tuple(vec![py_str("fit"), g_A_FIT(), g_FIT_LINES(), Py::None]), py_tuple(vec![py_str("rule"), g_A_RULE(), g_RULE_LINES(), g_not_a_rule()]), py_tuple(vec![py_str("figure"), g_A_STAT(), g_STAT_LINES(), Py::None]), py_tuple(vec![py_str("moving"), g_A_MOVING(), g_MOVING_LINES(), Py::None]), py_tuple(vec![py_str("rate"), g_A_RATE(), g_RATE_LINES(), Py::None]), py_tuple(vec![py_str("price"), g_A_PRICE(), g_PRICE_LINES(), py_tuple(vec![g_NOT_A_PRICE(), g_a_sum_through_the_year()])]), py_tuple(vec![py_str("duration"), g_A_DURATION(), g_DURATION_LINES(), g_NOT_A_QUEUE()]), py_tuple(vec![py_str("record"), g_A_RECORD(), g_RECORD_LINES(), g__not_a_record()])]));
    s_A_POINTER(stdlib_call("re", "compile", &[py_str("\\ba quick search\\b|\\bsearch online\\b|\\bsearching for it\\b|\\bquick look\\b|\\ba search\\b|\\blook (?:it|that) up\\b|\\bwikipedia\\b|\\bprice comparison\\b|\\bpaste what it says\\b|\\b(?:official|seller'?s|their|its|the)\\b[^.?!]{0,20}?\\b(?:site|page|listing|app)\\b[^.?!]{0,24}?\\b(?:will|has|have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PAPER_ITSELF(stdlib_call("re", "compile", &[py_str("\\bthe (?:receipt|deed|deeds|wording|document|documents|paperwork|certificate|policy|contract|agreement|statement|invoice|booking|label|manual|terms)\\b|\\b(?:date|name|reference|number) on it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEED_TO(stdlib_call("re", "compile", &[py_str("\\bdo (?:i|we) need (?:to |a |an )\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEED_TO_LINES(py_tuple(vec![py_str("Depends what it is for. Say the thing you are doing and I will tell you what it takes - and where a rule rather than common sense decides it, who to ask."), py_str("It turns on the thing itself and on what it is for - a trip, a form, a job and a pan of something all want different things. Tell me which it is and I will give you the short version, rather than the general one that fits none of them."), py_str("That depends on the thing itself and what you are doing with it. Say which and I will tell you, rather than guess at it.")]));
    s_PROCEDURE_SHAPES(py_tuple(vec![py_tuple(vec![py_str("needto"), g_A_NEED_TO(), g_NEED_TO_LINES(), Py::None])]));
    s_A_WORRY_ASK(stdlib_call("re", "compile", &[py_str("\\bdo (?:i|we) (?:need|have) to\\b[^.?!]{0,24}\\b(?:worry|panick?\\w*|stress\\w*|be anxious|be scared|be (?:worried|afraid)|fret)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WORRY_LINES(py_tuple(vec![py_str("Worry about what? Tell me what is coming up and I will say whether there is anything to do about it, because most of what people brace for turns out to be somebody else's rule."), py_str("Say what it is and I will tell you straight whether there is anything to do about it. If there is, it is usually one form or one call, and if there is not, you can stop carrying it.")]));
    s_WORRY_SHAPES(py_tuple(vec![py_tuple(vec![py_str("worried"), g_A_WORRY_ASK(), g_WORRY_LINES(), Py::None])]));
}
pub fn times_a_year(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let _ = py_str("The count of a repeated spend in a year, and the span it was said in, or (None, None).");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASKS_A_COST(), "search", &[clean.clone()])))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&mcall(&g_AN_AMOUNT_ALREADY(), "search", &[clean.clone()])) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_LONGER_SPAN(), "search", &[clean.clone()])))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    found = mcall(&g_A_COUNT_IN_A_SPAN(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    plain = mcall(&g_A_EVERY_SPAN(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    return py_tuple(vec![geti(&g_A_YEAR(), &mcall(&mcall(&plain.clone(), "group", &[Py::Int(1)]), "lower", &[])), mcall(&py_str(" "), "join", &[mcall(&mcall(&plain.clone(), "group", &[Py::Int(0)]), "split", &[])])]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "lower", &[])]), 2, None);
    said = __unpk1[0].clone();
    span = __unpk1[1].clone();
    count = if py_truthy(&mcall(&said.clone(), "isdigit", &[])) { call_builtin("int", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g_COUNT_WORDS(), "get", &[said.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    return py_tuple(vec![mul(&count.clone(), &geti(&g_A_YEAR(), &span.clone())), mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "split", &[])])]);
    Py::None
}
pub fn sum_lines(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut said: Py = Py::None;
    let mut times: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The lines for a repeated spend asked for by the year, or None.");
    let __unpk1 = py_unpack(&times_a_year(&[clean.clone()]), 2, None);
    times = __unpk1[0].clone();
    said = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&times.clone()))) {
    return Py::None;
    }
    figure = mul(&times.clone(), &g_EXAMPLE());
    written = add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    return py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("w".to_string(), written.clone()), ("n".to_string(), times.clone()), ("fig".to_string(), figure.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{w} is {n:,} a year. Put your own price on one - £5 each is £{fig:,} - and that is the figure to weigh against cutting it back."), "format", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("w".to_string(), written.clone()), ("n".to_string(), times.clone()), ("fig".to_string(), figure.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{w} is {n:,} times a year, so put a price on one: at a fiver each that is £{fig:,}, which is what the habit really costs."), "format", args))(&__args) }]);
    Py::None
}
pub fn a_sum_through_the_year(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line adds up a repeated spend over a longer span, which is not a quote.");
    return call_builtin("bool", &[geti(&times_a_year(&[clean.clone()]), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn not_about_one_owned_thing(args: &[Py]) -> Py {
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
    let _ = py_str("True when a \"worth it\" line is not about a thing that is repaired or replaced.\n\n    The shape is the guard the other kinds take. It reads the line, not the conversation: a line\n    that names neither the repair nor the thing going wrong is a question about a work, a place,\n    a course or a person, and the precept about the cost of putting it right does not fit it.\n    ");
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_REPAIR_OR_BUY(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_BROKEN_THING(), "search", &[clean.clone()]) } }));
    Py::None
}
pub fn NOT_A_QUEUE(args: &[Py]) -> Py {
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
    let _ = py_str("True when the wait is not somebody else's queue: a bare follow-up, a body, a pan.");
    if py_truthy(&mcall(&g_BARE_DURATION(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:how long|how soon) (?:do|should|shall|can|will) (?:i|we) (?:wait|hold|hang|leave|give|keep|sit)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g_A_QUEUE(), "search", &[text.clone()])));
    Py::None
}
pub fn not_a_bare_procedure(args: &[Py]) -> Py {
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
    let _ = py_str("True when a \"do i need to\" line names nothing a rule is about.\n\n    The rest of `A_RULE` stands on its own: \"am i allowed\", \"is it legal\" and \"what are my\n    rights\" are about rules whichever thing follows them. Only the \"do i need\" / \"do i have to\"\n    shapes are the everyday yes-or-no about a procedure, and they need the subject of a rule in\n    the same line before the precept about the wording in force is the right thing to say.\n    ");
    return { let __b = call_builtin("bool", &[mcall(&g_A_DO_I_NEED(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_RULE_SUBJECT(), "search", &[clean.clone()]))) } };
    Py::None
}
pub fn not_a_rule(args: &[Py]) -> Py {
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
    let _ = py_str("Both guards for the rule kind: the board game, and the bare procedure.");
    return { let __b = call_builtin("bool", &[mcall(&g_NOT_A_RULE(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { not_a_bare_procedure(&[clean.clone()]) } };
    Py::None
}
pub fn NOT_A_HOW_TO(args: &[Py]) -> Py {
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
    let _ = py_str("True when the steps above do not fit the ask: a feeling, a person, or no trouble in it.");
    return { let __b = call_builtin("bool", &[mcall(&g_NOT_A_HOWTO(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_HOWTO_HAS_A_SHAPE(), "search", &[text.clone()]))) } };
    Py::None
}
pub fn _not_a_record(args: &[Py]) -> Py {
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
    let _ = py_str("True when this line is not about the entry itself, so the record kind stands down.\n\n    The kinds' guards answer the same way: true means \"not mine\", and the next kind is read.\n    ");
    if py_truthy(&mcall(&g_NOT_A_RECORD(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_A_NAMED_DOCUMENT(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THEIR_OWN_PAPER(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_LOOSE_KIND_WORD(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn names_only_a_page(args: &[Py]) -> Py {
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
    let _ = py_str("True when a reply is a shrug with a search box or a page named as where to look.\n\n    Not the same test as `fallback.is_a_hedge`: the live notes (\"Prices move too fast for me to\n    know. A quick look at the seller's own site\") are the composer's own answer to a price ask\n    rather than a hedge, and they are the same pointer to the person reading them. The greeting\n    module asks both, so a line that already answers keeps its turn.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_POINTER(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn names_the_paper(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply already says the person's own paper is where the answer is.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_THE_PAPER_ITSELF(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
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
    let mut clean: Py = Py::None;
    let mut g: Py = Py::None;
    let mut guard: Py = Py::None;
    let mut guards: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The kind of question this is, and the lines for it, or (None, None).\n\n    Kept public so a test reads the same shapes the reply is built from: a test that re-wrote\n    them would only be testing itself.\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { mcall(&g_NOT_HERE(), "search", &[clean.clone()]) } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_ASKS_SOMETHING(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAME_ONLY_ASK(), "search", &[clean.clone()]) } }))) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    let __iter1 = py_iter_or_raise(&add(&add(&g_WORRY_SHAPES(), &g_KINDS()), &g_PROCEDURE_SHAPES()));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    kind = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    lines = __unpk2[2].clone();
    guard = __unpk2[3].clone();
    if py_truthy(&mcall(&pattern.clone(), "search", &[clean.clone()])) {
    if py_truthy(&guard.clone()) {
    guards = if py_truthy(&call_builtin("isinstance", &[guard.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { guard.clone() } else { py_tuple(vec![guard.clone()]) };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&guards.clone()) { let mut g: Py = __item.clone(); __out.push(if py_truthy(&call_builtin("callable", &[g.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&g, &[clean.clone()]) } else { mcall(&g.clone(), "search", &[clean.clone()]) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    }
    return py_tuple(vec![kind.clone(), lines.clone()]);
    }
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn pool(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut summed: Py = Py::None;
    let _ = py_str("The lines for a gap the bot can still say something useful about, or None.\n\n    The shape is the one `findout.pool` returns, so the caller picks among them the usual way and\n    the last line the bot said is not said again.\n    ");
    summed = sum_lines(&[clean.clone()]);
    if py_truthy(&summed.clone()) {
    return summed.clone();
    }
    return geti(&reads(&[clean.clone()]), &Py::Int(1));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.gap".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AN_AMOUNT_ALREADY" => Some(g_AN_AMOUNT_ALREADY()),
            "ASKS_A_COST" => Some(g_ASKS_A_COST()),
            "ASKS_SOMETHING" => Some(g_ASKS_SOMETHING()),
            "A_BROKEN_THING" => Some(g_A_BROKEN_THING()),
            "A_CHOICE" => Some(g_A_CHOICE()),
            "A_COUNT_IN_A_SPAN" => Some(g_A_COUNT_IN_A_SPAN()),
            "A_DISTANCE" => Some(g_A_DISTANCE()),
            "A_DO_I_NEED" => Some(g_A_DO_I_NEED()),
            "A_DURATION" => Some(g_A_DURATION()),
            "A_EVERY_SPAN" => Some(g_A_EVERY_SPAN()),
            "A_FIT" => Some(g_A_FIT()),
            "A_HOWTO" => Some(g_A_HOWTO()),
            "A_HOWTO_HAS_A_SHAPE" => Some(g_A_HOWTO_HAS_A_SHAPE()),
            "A_LOAD" => Some(g_A_LOAD()),
            "A_LONGER_SPAN" => Some(g_A_LONGER_SPAN()),
            "A_LOOSE_KIND_WORD" => Some(g_A_LOOSE_KIND_WORD()),
            "A_MOVING" => Some(g_A_MOVING()),
            "A_NAMED_DOCUMENT" => Some(g_A_NAMED_DOCUMENT()),
            "A_NEED_TO" => Some(g_A_NEED_TO()),
            "A_POINTER" => Some(g_A_POINTER()),
            "A_PRICE" => Some(g_A_PRICE()),
            "A_PROCESS" => Some(g_A_PROCESS()),
            "A_QUEUE" => Some(g_A_QUEUE()),
            "A_RATE" => Some(g_A_RATE()),
            "A_RECORD" => Some(g_A_RECORD()),
            "A_REPAIR_OR_BUY" => Some(g_A_REPAIR_OR_BUY()),
            "A_REPEATED_SPEND" => Some(g_A_REPEATED_SPEND()),
            "A_RULE" => Some(g_A_RULE()),
            "A_RULE_SUBJECT" => Some(g_A_RULE_SUBJECT()),
            "A_STAT" => Some(g_A_STAT()),
            "A_TROUBLE" => Some(g_A_TROUBLE()),
            "A_WORRY_ASK" => Some(g_A_WORRY_ASK()),
            "A_WORTH" => Some(g_A_WORTH()),
            "A_YEAR" => Some(g_A_YEAR()),
            "BARE_DURATION" => Some(g_BARE_DURATION()),
            "CHOICE_LINES" => Some(g_CHOICE_LINES()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "DISTANCE_LINES" => Some(g_DISTANCE_LINES()),
            "DURATION_LINES" => Some(g_DURATION_LINES()),
            "EXAMPLE" => Some(g_EXAMPLE()),
            "FIT_LINES" => Some(g_FIT_LINES()),
            "HOWTO_LINES" => Some(g_HOWTO_LINES()),
            "KINDS" => Some(g_KINDS()),
            "LOAD_LINES" => Some(g_LOAD_LINES()),
            "MOVING_LINES" => Some(g_MOVING_LINES()),
            "NAME_ONLY_ASK" => Some(g_NAME_ONLY_ASK()),
            "NEED_TO_LINES" => Some(g_NEED_TO_LINES()),
            "NOT_A_HOWTO" => Some(g_NOT_A_HOWTO()),
            "NOT_A_HOW_TO" => Some(py_func("NOT_A_HOW_TO", |args: &[Py]| NOT_A_HOW_TO(args))),
            "NOT_A_PRICE" => Some(g_NOT_A_PRICE()),
            "NOT_A_QUEUE" => Some(py_func("NOT_A_QUEUE", |args: &[Py]| NOT_A_QUEUE(args))),
            "NOT_A_RECORD" => Some(g_NOT_A_RECORD()),
            "NOT_A_RULE" => Some(g_NOT_A_RULE()),
            "NOT_HERE" => Some(g_NOT_HERE()),
            "PRICE_LINES" => Some(g_PRICE_LINES()),
            "PROCEDURE_SHAPES" => Some(g_PROCEDURE_SHAPES()),
            "RATE_LINES" => Some(g_RATE_LINES()),
            "RECORD_LINES" => Some(g_RECORD_LINES()),
            "RULE_LINES" => Some(g_RULE_LINES()),
            "STAT_LINES" => Some(g_STAT_LINES()),
            "THEIR_OWN_PAPER" => Some(g_THEIR_OWN_PAPER()),
            "THE_PAPER_ITSELF" => Some(g_THE_PAPER_ITSELF()),
            "WORRY_LINES" => Some(g_WORRY_LINES()),
            "WORRY_SHAPES" => Some(g_WORRY_SHAPES()),
            "WORTH_LINES" => Some(g_WORTH_LINES()),
            "_not_a_record" => Some(py_func("_not_a_record", |args: &[Py]| _not_a_record(args))),
            "a_sum_through_the_year" => Some(py_func("a_sum_through_the_year", |args: &[Py]| a_sum_through_the_year(args))),
            "names_only_a_page" => Some(py_func("names_only_a_page", |args: &[Py]| names_only_a_page(args))),
            "names_the_paper" => Some(py_func("names_the_paper", |args: &[Py]| names_the_paper(args))),
            "not_a_bare_procedure" => Some(py_func("not_a_bare_procedure", |args: &[Py]| not_a_bare_procedure(args))),
            "not_a_rule" => Some(py_func("not_a_rule", |args: &[Py]| not_a_rule(args))),
            "not_about_one_owned_thing" => Some(py_func("not_about_one_owned_thing", |args: &[Py]| not_about_one_owned_thing(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "reads" => Some(py_func("reads", |args: &[Py]| reads(args))),
            "sum_lines" => Some(py_func("sum_lines", |args: &[Py]| sum_lines(args))),
            "times_a_year" => Some(py_func("times_a_year", |args: &[Py]| times_a_year(args))),
            _ => None,
        }),
        set: None,
    }))
}

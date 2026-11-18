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


pub static ABOUT_THIS_CHAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASKS_TO_DECIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_ONLY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_BODY_IN_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_FOOD_IN_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MACHINE_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MACHINE_OF_THEIRS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MACHINE_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PRICE_THEY_WILL_PAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_USUAL_AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOCUMENT_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_ATTEMPT_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_ATTEMPT_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_READY_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_READY_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_READY_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_READY_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GET_READY_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MUCH_TO_PUT_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO_TEACH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO_TEACH_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO_TEACH_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_OK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_OK_FOOD_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_OK_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_OK_PLAIN_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_OK_PLAIN_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_IT_OK_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVING_SOMEONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVING_SOMEONE_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAVING_SOMEONE_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIVE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIVE_TODAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIVE_TODAY_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIVE_TODAY_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMES_WHERE_TO_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEWS_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEWS_RESULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEWS_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTER_HEDGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUT_IN_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PUT_IN_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTE_IN_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROUTE_IN_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULES_FOR_THEIR_CASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULES_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULES_PLAIN_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RULES_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSES_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSES_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSES_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static USUAL_AMOUNT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static USUAL_AMOUNT_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATCH_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATCH_OUT_BODY_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATCH_OUT_BODY_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATCH_OUT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WATCH_OUT_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_TAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_TAKE_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_TO_TAKE_ROUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_YOU_NEED_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_YOU_NEED_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_TO_START_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_TO_START_LINES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THIS_CHAT_TALK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THIS_CHAT() -> Py { ensure_init(); ABOUT_THIS_CHAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THIS_CHAT(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THIS_CHAT.write() { *w = v; } }
pub fn g_ASKS_TO_DECIDE() -> Py { ensure_init(); ASKS_TO_DECIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASKS_TO_DECIDE(v: Py) { ensure_init(); if let Ok(mut w) = ASKS_TO_DECIDE.write() { *w = v; } }
pub fn g_ASK_ONLY_WORDS() -> Py { ensure_init(); ASK_ONLY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_ONLY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_ONLY_WORDS.write() { *w = v; } }
pub fn g_A_BODY_IN_IT() -> Py { ensure_init(); A_BODY_IN_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_BODY_IN_IT(v: Py) { ensure_init(); if let Ok(mut w) = A_BODY_IN_IT.write() { *w = v; } }
pub fn g_A_FOOD_IN_IT() -> Py { ensure_init(); A_FOOD_IN_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_FOOD_IN_IT(v: Py) { ensure_init(); if let Ok(mut w) = A_FOOD_IN_IT.write() { *w = v; } }
pub fn g_A_MACHINE_LEADS() -> Py { ensure_init(); A_MACHINE_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = A_MACHINE_LEADS.write() { *w = v; } }
pub fn g_A_MACHINE_OF_THEIRS() -> Py { ensure_init(); A_MACHINE_OF_THEIRS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE_OF_THEIRS(v: Py) { ensure_init(); if let Ok(mut w) = A_MACHINE_OF_THEIRS.write() { *w = v; } }
pub fn g_A_MACHINE_ROUTES() -> Py { ensure_init(); A_MACHINE_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = A_MACHINE_ROUTES.write() { *w = v; } }
pub fn g_A_PRICE_LEADS() -> Py { ensure_init(); A_PRICE_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE_LEADS.write() { *w = v; } }
pub fn g_A_PRICE_ROUTES() -> Py { ensure_init(); A_PRICE_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE_ROUTES.write() { *w = v; } }
pub fn g_A_PRICE_THEY_WILL_PAY() -> Py { ensure_init(); A_PRICE_THEY_WILL_PAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PRICE_THEY_WILL_PAY(v: Py) { ensure_init(); if let Ok(mut w) = A_PRICE_THEY_WILL_PAY.write() { *w = v; } }
pub fn g_A_USUAL_AMOUNT() -> Py { ensure_init(); A_USUAL_AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_USUAL_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = A_USUAL_AMOUNT.write() { *w = v; } }
pub fn g_DOCUMENT_NAMED() -> Py { ensure_init(); DOCUMENT_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOCUMENT_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = DOCUMENT_NAMED.write() { *w = v; } }
pub fn g_FIRST_ATTEMPT_ASKS() -> Py { ensure_init(); FIRST_ATTEMPT_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_ATTEMPT_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_ATTEMPT_ASKS.write() { *w = v; } }
pub fn g_FIRST_ATTEMPT_LINES() -> Py { ensure_init(); FIRST_ATTEMPT_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_ATTEMPT_LINES(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_ATTEMPT_LINES.write() { *w = v; } }
pub fn g_GET_READY_ASKS() -> Py { ensure_init(); GET_READY_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_READY_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = GET_READY_ASKS.write() { *w = v; } }
pub fn g_GET_READY_FOR() -> Py { ensure_init(); GET_READY_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_READY_FOR(v: Py) { ensure_init(); if let Ok(mut w) = GET_READY_FOR.write() { *w = v; } }
pub fn g_GET_READY_LEADS() -> Py { ensure_init(); GET_READY_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_READY_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = GET_READY_LEADS.write() { *w = v; } }
pub fn g_GET_READY_LINES() -> Py { ensure_init(); GET_READY_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_READY_LINES(v: Py) { ensure_init(); if let Ok(mut w) = GET_READY_LINES.write() { *w = v; } }
pub fn g_GET_READY_ROUTES() -> Py { ensure_init(); GET_READY_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GET_READY_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = GET_READY_ROUTES.write() { *w = v; } }
pub fn g_HOW_MUCH_TO_PUT_IN() -> Py { ensure_init(); HOW_MUCH_TO_PUT_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MUCH_TO_PUT_IN(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MUCH_TO_PUT_IN.write() { *w = v; } }
pub fn g_HOW_TO_TEACH() -> Py { ensure_init(); HOW_TO_TEACH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_TEACH(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO_TEACH.write() { *w = v; } }
pub fn g_HOW_TO_TEACH_LEADS() -> Py { ensure_init(); HOW_TO_TEACH_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_TEACH_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO_TEACH_LEADS.write() { *w = v; } }
pub fn g_HOW_TO_TEACH_ROUTES() -> Py { ensure_init(); HOW_TO_TEACH_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO_TEACH_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO_TEACH_ROUTES.write() { *w = v; } }
pub fn g_IS_IT_OK() -> Py { ensure_init(); IS_IT_OK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_OK(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_OK.write() { *w = v; } }
pub fn g_IS_IT_OK_FOOD_ROUTES() -> Py { ensure_init(); IS_IT_OK_FOOD_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_OK_FOOD_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_OK_FOOD_ROUTES.write() { *w = v; } }
pub fn g_IS_IT_OK_LEADS() -> Py { ensure_init(); IS_IT_OK_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_OK_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_OK_LEADS.write() { *w = v; } }
pub fn g_IS_IT_OK_PLAIN_LEADS() -> Py { ensure_init(); IS_IT_OK_PLAIN_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_OK_PLAIN_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_OK_PLAIN_LEADS.write() { *w = v; } }
pub fn g_IS_IT_OK_PLAIN_ROUTES() -> Py { ensure_init(); IS_IT_OK_PLAIN_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_OK_PLAIN_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_OK_PLAIN_ROUTES.write() { *w = v; } }
pub fn g_IS_IT_OK_ROUTES() -> Py { ensure_init(); IS_IT_OK_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_IT_OK_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = IS_IT_OK_ROUTES.write() { *w = v; } }
pub fn g_LEAVING_SOMEONE() -> Py { ensure_init(); LEAVING_SOMEONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVING_SOMEONE(v: Py) { ensure_init(); if let Ok(mut w) = LEAVING_SOMEONE.write() { *w = v; } }
pub fn g_LEAVING_SOMEONE_LEADS() -> Py { ensure_init(); LEAVING_SOMEONE_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVING_SOMEONE_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEAVING_SOMEONE_LEADS.write() { *w = v; } }
pub fn g_LEAVING_SOMEONE_ROUTES() -> Py { ensure_init(); LEAVING_SOMEONE_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAVING_SOMEONE_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = LEAVING_SOMEONE_ROUTES.write() { *w = v; } }
pub fn g_LIVE_ASK() -> Py { ensure_init(); LIVE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIVE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = LIVE_ASK.write() { *w = v; } }
pub fn g_LIVE_TODAY() -> Py { ensure_init(); LIVE_TODAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIVE_TODAY(v: Py) { ensure_init(); if let Ok(mut w) = LIVE_TODAY.write() { *w = v; } }
pub fn g_LIVE_TODAY_LEADS() -> Py { ensure_init(); LIVE_TODAY_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIVE_TODAY_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LIVE_TODAY_LEADS.write() { *w = v; } }
pub fn g_LIVE_TODAY_ROUTES() -> Py { ensure_init(); LIVE_TODAY_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIVE_TODAY_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = LIVE_TODAY_ROUTES.write() { *w = v; } }
pub fn g_NAMES_WHERE_TO_ASK() -> Py { ensure_init(); NAMES_WHERE_TO_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMES_WHERE_TO_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NAMES_WHERE_TO_ASK.write() { *w = v; } }
pub fn g_NEWS_LEADS() -> Py { ensure_init(); NEWS_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEWS_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = NEWS_LEADS.write() { *w = v; } }
pub fn g_NEWS_RESULT() -> Py { ensure_init(); NEWS_RESULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEWS_RESULT(v: Py) { ensure_init(); if let Ok(mut w) = NEWS_RESULT.write() { *w = v; } }
pub fn g_NEWS_ROUTES() -> Py { ensure_init(); NEWS_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEWS_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = NEWS_ROUTES.write() { *w = v; } }
pub fn g_POINTER_HEDGE() -> Py { ensure_init(); POINTER_HEDGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTER_HEDGE(v: Py) { ensure_init(); if let Ok(mut w) = POINTER_HEDGE.write() { *w = v; } }
pub fn g_PUT_IN_LEADS() -> Py { ensure_init(); PUT_IN_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUT_IN_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = PUT_IN_LEADS.write() { *w = v; } }
pub fn g_PUT_IN_ROUTES() -> Py { ensure_init(); PUT_IN_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PUT_IN_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = PUT_IN_ROUTES.write() { *w = v; } }
pub fn g_ROUTE_IN_ASKS() -> Py { ensure_init(); ROUTE_IN_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTE_IN_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = ROUTE_IN_ASKS.write() { *w = v; } }
pub fn g_ROUTE_IN_LINES() -> Py { ensure_init(); ROUTE_IN_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROUTE_IN_LINES(v: Py) { ensure_init(); if let Ok(mut w) = ROUTE_IN_LINES.write() { *w = v; } }
pub fn g_RULES_FOR_THEIR_CASE() -> Py { ensure_init(); RULES_FOR_THEIR_CASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULES_FOR_THEIR_CASE(v: Py) { ensure_init(); if let Ok(mut w) = RULES_FOR_THEIR_CASE.write() { *w = v; } }
pub fn g_RULES_LEADS() -> Py { ensure_init(); RULES_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULES_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = RULES_LEADS.write() { *w = v; } }
pub fn g_RULES_PLAIN_ROUTES() -> Py { ensure_init(); RULES_PLAIN_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULES_PLAIN_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = RULES_PLAIN_ROUTES.write() { *w = v; } }
pub fn g_RULES_ROUTES() -> Py { ensure_init(); RULES_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RULES_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = RULES_ROUTES.write() { *w = v; } }
pub fn g_SOMEONE_ELSES_LEADS() -> Py { ensure_init(); SOMEONE_ELSES_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSES_LEADS.write() { *w = v; } }
pub fn g_SOMEONE_ELSES_OWN() -> Py { ensure_init(); SOMEONE_ELSES_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_OWN(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSES_OWN.write() { *w = v; } }
pub fn g_SOMEONE_ELSES_ROUTES() -> Py { ensure_init(); SOMEONE_ELSES_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSES_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSES_ROUTES.write() { *w = v; } }
pub fn g_USUAL_AMOUNT_LEADS() -> Py { ensure_init(); USUAL_AMOUNT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_USUAL_AMOUNT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = USUAL_AMOUNT_LEADS.write() { *w = v; } }
pub fn g_USUAL_AMOUNT_ROUTES() -> Py { ensure_init(); USUAL_AMOUNT_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_USUAL_AMOUNT_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = USUAL_AMOUNT_ROUTES.write() { *w = v; } }
pub fn g_WATCH_OUT() -> Py { ensure_init(); WATCH_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATCH_OUT(v: Py) { ensure_init(); if let Ok(mut w) = WATCH_OUT.write() { *w = v; } }
pub fn g_WATCH_OUT_BODY_LEADS() -> Py { ensure_init(); WATCH_OUT_BODY_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATCH_OUT_BODY_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = WATCH_OUT_BODY_LEADS.write() { *w = v; } }
pub fn g_WATCH_OUT_BODY_ROUTES() -> Py { ensure_init(); WATCH_OUT_BODY_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATCH_OUT_BODY_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = WATCH_OUT_BODY_ROUTES.write() { *w = v; } }
pub fn g_WATCH_OUT_LEADS() -> Py { ensure_init(); WATCH_OUT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATCH_OUT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = WATCH_OUT_LEADS.write() { *w = v; } }
pub fn g_WATCH_OUT_ROUTES() -> Py { ensure_init(); WATCH_OUT_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WATCH_OUT_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = WATCH_OUT_ROUTES.write() { *w = v; } }
pub fn g_WHAT_TO_TAKE() -> Py { ensure_init(); WHAT_TO_TAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_TAKE(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_TAKE.write() { *w = v; } }
pub fn g_WHAT_TO_TAKE_LEADS() -> Py { ensure_init(); WHAT_TO_TAKE_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_TAKE_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_TAKE_LEADS.write() { *w = v; } }
pub fn g_WHAT_TO_TAKE_ROUTES() -> Py { ensure_init(); WHAT_TO_TAKE_ROUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_TAKE_ROUTES(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_TO_TAKE_ROUTES.write() { *w = v; } }
pub fn g_WHAT_YOU_NEED_ASKS() -> Py { ensure_init(); WHAT_YOU_NEED_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_YOU_NEED_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_YOU_NEED_ASKS.write() { *w = v; } }
pub fn g_WHAT_YOU_NEED_LINES() -> Py { ensure_init(); WHAT_YOU_NEED_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_YOU_NEED_LINES(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_YOU_NEED_LINES.write() { *w = v; } }
pub fn g_WHERE_TO_START_ASKS() -> Py { ensure_init(); WHERE_TO_START_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_START_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_TO_START_ASKS.write() { *w = v; } }
pub fn g_WHERE_TO_START_LINES() -> Py { ensure_init(); WHERE_TO_START_LINES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_TO_START_LINES(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_TO_START_LINES.write() { *w = v; } }
pub fn g__THIS_CHAT_TALK() -> Py { ensure_init(); _THIS_CHAT_TALK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THIS_CHAT_TALK(v: Py) { ensure_init(); if let Ok(mut w) = _THIS_CHAT_TALK.write() { *w = v; } }
fn module_body() {
    let _ = py_str("What the fallback can still say when no pool holds the answer.\n\nA search tip on its own is not a reply: it hands the whole question back and tells the person to\ngo somewhere else. What a friend does with the same gap is say the part that is true without the\nanswer - what settles this kind of question, and what to have in front of you - and only then say\nwhere the rest lives. The kinds here are kinds of question, not wordings: what is open today, what\na machine of theirs is doing, what the rules say for their dates, what a thing costs, what only\nanother person can answer, what happened, what people usually give. None of them needs the answer\nto be useful, and none of them invents one.\n\nTwo entry points, both called from the fallback's hedge slot and nowhere else, so every line that\nsome other module can answer keeps its own reply:\n\n    the_lead_and_the_route(clean, memory)      what is known about this kind, then the pointer\n    an_ask_with_no_subject(clean, memory)      \"whats the first step\": the usual first moves\n\n`POINTER_HEDGE` says which replies may be replaced at all: a reply that only points somewhere. A\nreply that weighs a decision, names where money advice comes from, or hands a body to a doctor is\nan answer of its own and is left alone.\n");
    s_LIVE_TODAY(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:open|opens|opening|close|closes|closing|shut|hours|opening times|till|busy|busiest|queue|traffic|parking|stock|in stock|sold out|available|delivery|today|tonight|tomorrow|this weekend|sunday|saturday)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what time|when|is it|are they|is the|is my|do they|does it|can i|will|how busy|how long is the wait|still)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MACHINE_OF_THEIRS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:model|serial|make and model|fault code|error code|manual|spare part|part number|firmware|warranty|(?:the|my|this|that|an) (?:app|settings|instructions)|reset (?:it|the|my)|(?:software|firmware|system) update|boiler|washer|washing machine|dishwasher|tumble dryer|fridge|freezer|oven|hob|kettle|vacuum|hoover|laptop|tablet|printer|router|thermostat|radiator|heat pump|smart ?watch|fitbit|tracker|console|my (?:phone|tv|telly|car|van|bike|scooter|watch|camera|alarm))(?!['a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_READY_FOR(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how\\s+(?:long|many\\s+weeks|many\\s+days|much\\s+time)\\s+(?:should|do|would|will|can)\\s+(?:i|we)\\s+(?:train|practise|practice|study|revise|prepare|get\\s+ready)|how\\s+long\\s+(?:before|until)\\s+(?:i|we)\\s+(?:need|should|have)\\s+to\\s+(?:train|be\\s+ready|get\\s+ready|start))(?!['a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_READY_LEADS(py_tuple(vec![py_str("The number comes from two things: where you can get to now, and how many days a week you can give it. A build-up that climbs and then eases off beats a heroic first week every time."), py_str("It depends on your starting point and on the time you can actually give it, not on a rule - and the shape is the same for most things: steady most days, one hard session, and rest.")]));
    s_GET_READY_ROUTES(py_tuple(vec![py_str("Tell me what you can do now and how many days a week you have, and I will say what I would aim at; the training app or the course's own plan has the standard build-up for your dates."), py_str("Give me those two and I will say where I would start; whoever runs the event or the exam publishes the usual build-up, and that is worth comparing with.")]));
    s_RULES_FOR_THEIR_CASE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:allowed|entitled|entitlement|qualify|eligible|legal|law|rules|regulation|statutory|allowance|benefit|benefits|tax|taxed|taxable|vat|licence|license|permit|visa|tenancy|contract|clause|notice period|policy|terms|insurance|pension|ni|national insurance|council tax|deadline)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MUCH_TO_PUT_IN(stdlib_call("re", "compile", &[py_str("how much (?:should|shall|do|can|would) i (?:be )?(?:put|putting|pay|paying|save|saving|contribute|contributing|invest|investing|stick|stick in|set aside)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PUT_IN_LEADS(py_tuple(vec![py_str("Start at whatever the other side adds to: below the employer's match you are leaving their money on the table, and that is the one part with no argument against it."), py_str("The order is the same for most people: the matched amount first, then whatever you can leave alone until you retire, and only then anything else.")]));
    s_PUT_IN_ROUTES(py_tuple(vec![py_str("The figure that suits you comes from what the scheme pays in and what you can do without now, and their calculator or the free guidance service will work it through with you."), py_str("Take the percentage your scheme's own page shows for the match, then look at what your payslip leaves - that pair gives you the number.")]));
    s_A_PRICE_THEY_WILL_PAY(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how much (?:is|does|do|would|will|are|should)|what does .{1,30} cost|the cost of|price of|prices for|how expensive|cheapest|cheapest|going rate|quote|charge|charges|fee|fees|fare|fares|how much (?:will|would) it cost)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_TAKE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what (?:do|should) i (?:take|bring|pack)|what to (?:take|bring|pack)|what should i put in my (?:bag|case|rucksack|pack)|what do i need to (?:take|bring|pack)|what else (?:do|should) i (?:need to )?(?:take|bring|pack))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_TO_TAKE_LEADS(py_tuple(vec![py_str("Take less than you think, and the three things most people forget: a charger, a bag for anything wet, and your own water."), py_str("The list that works is short: anything you would not want to buy twice, a layer for the cold end of the day, and the paperwork if there is any.")]));
    s_WHAT_TO_TAKE_ROUTES(py_tuple(vec![py_str("The operator's or the venue's page says what is not allowed in through the door, and that is the only part worth checking before you go."), py_str("Tell me where you are going and for how long, and I will cut that list down to what you need.")]));
    s_HOW_TO_TEACH(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how (?:do|can|should) i (?:teach|help|show|explain)|how do i get (?:them|him|her|my \\w+) to|how can i help (?:out|at home|them|him|her))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_TO_TEACH_LEADS(py_tuple(vec![py_str("Break it into the smallest piece they can already do and add one step: ten minutes that ends while it is still going well is worth more than an hour that ends badly."), py_str("Show it once, let them do it badly without being corrected, then give them one thing to change next time. That order is what makes it stay.")]));
    s_HOW_TO_TEACH_ROUTES(py_tuple(vec![py_str("Tell me what they can already do and where it goes wrong, and I will say what the next step is."), py_str("Say how old they are and what they can already manage, and I will make it concrete.")]));
    s_LEAVING_SOMEONE(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:is it ok(?:ay)? to (?:leave|take|send|let) (?:him|her|them|my \\w+)|is it fine to (?:leave|take|send|let) (?:him|her|them)|can i (?:leave|let) (?:him|her|them)|should i let (?:him|her|them))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAVING_SOMEONE_LEADS(py_tuple(vec![py_str("That turns on three things: how old they are, how long it would be, and whether they could reach you - and what they would do if they could not."), py_str("There is no general yes or no to that: it comes down to how old they are, how long for, and who they could get hold of.")]));
    s_LEAVING_SOMEONE_ROUTES(py_tuple(vec![py_str("Tell me the three and I will say how it looks rather than what the rule is."), py_str("Give me those and I will tell you what I would check before deciding.")]));
    s_IS_IT_OK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:is it ok(?:ay)? to|is it alright to|is it fine to|would it be ok(?:ay)? to|am i ok to|are we ok to|is it safe to)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_IS_IT_OK_LEADS(py_tuple(vec![py_str("There is no general yes or no to that: it hangs on the details in front of you - how long it has been, how warm it is, and who it affects - and none of those reach me from here."), py_str("Whether that is fine turns on the specifics rather than on a rule: how long it has been, what the conditions are, and who else it touches.")]));
    s_IS_IT_OK_PLAIN_LEADS(py_tuple(vec![py_str("There is no rule of thumb that answers that one: it turns on your own circumstances more than on the question, and I cannot see yours from here."), py_str("That one is decided case by case rather than by a general rule, and the part that decides it is the part only you can see.")]));
    s_IS_IT_OK_ROUTES(py_tuple(vec![py_str("Tell me those three and I will say how it looks; where a body is involved, a pharmacist or 111 is quicker than me."), py_str("Give me the details and I will tell you which of them decides it.")]));
    s_IS_IT_OK_PLAIN_ROUTES(py_tuple(vec![py_str("Tell me the detail that matters most in your case and I will say how it looks."), py_str("Give me that detail and I will tell you what I would check first.")]));
    s_A_BODY_IN_IT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:swim|swimming|sea|pool|river|lake|beach|sun|sunburn|heat|hot tub|paint|fumes|bleach|mould|mold|asbestos|gas|electrics|electric|wiring|wires|plug|socket|ladder|roof|scaffold|knife|fire|barbecue|bbq|baby|pregnan\\w*|medicines?|tablets?|dose|antibiotics?|symptom|fever|sick|vomit\\w*|bleed\\w*|bruise\\w*|burn|wound|rash|swelling|pain|hurt|give|giving|given|feed|feeding|fed|knee|knees|back|shoulder|shoulders|ankle|ankles|hip|hips|wrist|wrists|elbow|elbows|neck|foot|feet|arm|arms|leg|legs|hand|hands|head|stomach|chest|ear|ears|eye|eyes|tooth|teeth|skin|heel|heels|joint|joints|muscle|muscles)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FOOD_IN_IT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:food|eat|eating|eaten|raw|cook|cooked|cooking|chicken|meat|beef|pork|lamb|fish|egg|eggs|dairy|milk|cheese|rice|pasta|leftovers?|reheat|defrost|fridge|freezer|use by|best before|drink|drinking|water|soup|sauce|mayo|mayonnaise|salad|bread|fruit|tomato(?:es)?|veg|vegetables?)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_IS_IT_OK_FOOD_ROUTES(py_tuple(vec![py_str("Tell me how long it has been and how warm it has been kept, and I will say how it looks; if anyone has eaten it and is unwell, 111 is quicker than me."), py_str("Give me those two - how long, and how warm - and I will tell you which of them decides it.")]));
    s_WATCH_OUT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what should i watch (?:out )?for|anything i should (?:watch|look out) for|what should i look out for|what are the risks|what could go wrong|is there anything to watch|what do i need to watch)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_USUAL_AMOUNT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:how much (?:cash|money)?\\s*(?:do|should|would) (?:you|i|people|we) (?:put|give|spend|tip|leave|pay)|how much (?:cash|money) (?:in|for|on)|what do people usually (?:give|spend|pay)|what.s (?:a )?normal (?:to give|to spend|amount)|how much is normal|what would you (?:give|put in)|is \\d+ (?:too much|enough)|how much do you (?:give|tip))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEWS_RESULT(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:did it (?:go through|pass|happen|work)|what happened|who won|was there|did they (?:win|vote|agree|announce)|has it been (?:announced|decided)|any news|did (?:[a-z]+ ){0,4}(?:get |go |went )?(?:called off|cancelled|approved|decided|accepted|agreed|through|ahead|passed|signed off)|is it still (?:on|going ahead)|what came of|how did it (?:go|turn out)|what was the result|result of the (?:vote|election|match|game)|did it get (?:approved|passed|through))(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSES_OWN(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:does (?:he|she|they|my \\w+) (?:like|want|prefer|think|plan|need)|is (?:he|she|they) (?:happy|ok|okay|up for|coming|free)|what does (?:he|she|they) (?:do|think|want|like)|what is (?:he|she|they) into|how does (?:he|she|they) feel)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_LIVE_TODAY_LEADS(py_tuple(vec![py_str("That changes by the day and by the place, so anything I said would be yesterday's - the day and the place decide it."), py_str("This is one of the ones that turns over daily, and I cannot see today's from here.")]));
    s_LIVE_TODAY_ROUTES(py_tuple(vec![py_str("The place's own page has today's, one phone call confirms it, and a map listing carries the hours underneath the address."), py_str("Their own page or its map listing is the only place that is right today; ring them if it is close to closing time.")]));
    s_A_MACHINE_LEADS(py_tuple(vec![py_str("The model number and the exact code on the display are what settle these - the number is on a sticker on the machine, or in the settings screen under About."), py_str("For a machine, the maker's own sheet is written for your exact model: the sticker on the back or inside the door has the number, and the code on the display is the other half.")]));
    s_A_MACHINE_ROUTES(py_tuple(vec![py_str("Put the model number into the maker's own support page and it gives the manual and what that code means."), py_str("The maker's support page with that model has the manual, the fault codes and the reset sequence.")]));
    s_RULES_LEADS(py_tuple(vec![py_str("Two things decide these: the exact wording of the rule, and your own dates. A general answer from me could be right about the rule and wrong about you."), py_str("These turn on the exact wording and on your own dates rather than on the rule itself, and I cannot see either from here.")]));
    s_RULES_ROUTES(py_tuple(vec![py_str("Whoever runs it - the office named on the letter, the department, or the helpline that owns the rule (GOV.UK for most of these) - is the one that will confirm your own case, and a free advice line will read it with you."), py_str("Ask whoever decided it to confirm it for your dates, and keep the reply: a free advice line will read it with you.")]));
    s_DOCUMENT_NAMED(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:letter|paperwork|papers?|email|e-mail|form|contract|policy|terms|bill|invoice|statement|receipt|quote|quotation|agreement|tenancy|p45|p60)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_RULES_PLAIN_ROUTES(py_tuple(vec![py_str("Whoever runs it - the office, the department, or the helpline that owns the rule (GOV.UK for most of these) - is the one that will confirm your own case, and a free advice line will read it with you."), py_str("Ask whoever decided it, in writing, to confirm it for your dates, and keep the reply: a free advice line will read it with you.")]));
    s_A_PRICE_LEADS(py_tuple(vec![py_str("Prices move week to week and shop to shop, so a number from me would be out of date the moment I said it."), py_str("The price depends on where you buy and when, which is not something I can see.")]));
    s_A_PRICE_ROUTES(py_tuple(vec![py_str("The shop's own page has today's price, and a comparison site shows what the others are charging this week."), py_str("Look at two or three sellers' own pages - the spread is usually bigger than people expect.")]));
    s_SOMEONE_ELSES_LEADS(py_tuple(vec![py_str("Only they can say that - it is their own head, and I have nothing on it from here."), py_str("That one lives with them rather than with me: no one else can answer it for them.")]));
    s_SOMEONE_ELSES_ROUTES(py_tuple(vec![py_str("Ask them straight - a plain question gets a straighter answer than working it out from the outside."), py_str("Say it to them in one line and let them fill in the rest.")]));
    s_USUAL_AMOUNT_LEADS(py_tuple(vec![py_str("There is no right number for this: the occasion and how close you are decide it, and people usually settle on a range they can repeat for everyone in the same circle so nobody is singled out."), py_str("What people give is a range rather than a figure - what the occasion calls for, what you would be comfortable receiving, and what the others in the same group are doing.")]));
    s_WATCH_OUT_LEADS(py_tuple(vec![py_str("The things that catch people out are usually the same few: the total cost rather than the headline price, who is responsible when two people are involved, and the deadline nobody mentions until it has passed."), py_str("What goes wrong is rarely the obvious part - it is the small print, the thing that was somebody else's job, and the timing.")]));
    s_WATCH_OUT_BODY_LEADS(py_tuple(vec![py_str("Three things, and they are the same for most joints: pain that carries on after you stop, swelling that comes up over the next day, and the joint locking or giving way under you."), py_str("What to watch for is not the noise - it is pain that settles in, a swell you can see, and anything that stops you putting weight on it or straightening it.")]));
    s_WATCH_OUT_BODY_ROUTES(py_tuple(vec![py_str("Any of those and it is worth a physio or your GP rather than waiting it out; none of them and it is usually just the joint being noisy."), py_str("If any of the three turns up, get it looked at rather than resting it for weeks; if none does, keep moving it.")]));
    s_WATCH_OUT_ROUTES(py_tuple(vec![py_str("Say what you are about to do and I will go through those three against it."), py_str("Tell me the situation and I will name the one I would check first.")]));
    s_USUAL_AMOUNT_ROUTES(py_tuple(vec![py_str("Ask one person who is going to the same thing what they are giving, and match it; that is how most of these are settled."), py_str("If it is a gift, a card and something small and chosen beats a bigger amount given flatly - tell me who it is for and I will help you pick.")]));
    s_NEWS_LEADS(py_tuple(vec![py_str("I have no feed from here, so I cannot see what happened - the result is the one thing I would be guessing at."), py_str("That is happening where I cannot see it: no papers, no bulletins, nothing that reaches me.")]));
    s_NEWS_ROUTES(py_tuple(vec![py_str("Whoever ran it will have the result - the organiser, the office that sent it, or the news site for it. Tell me what it says and I will help you read it."), py_str("The body's own page or a news report has the outcome; say what it says and we can work out what it means for you.")]));
    s_POINTER_HEDGE(stdlib_call("re", "compile", &[py_str("(?i)(?:a quick search|search online|searching for it|a search should|better sources|look it up|look that up|a map app|a directory|comparison site|its own page has|their own page has|the seller's site|official site|the official finder|i can't look (?:up|this)|no directory here|no maps here)")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_TO_DECIDE(stdlib_call("re", "compile", &[py_str("^\\s*(?:so |and |ok |okay |but |well )*(?:should|shall|ought) i(?![a-z])|(?<![a-z])(?:do i bother|do i go for|is it worth|worth (?:it|fixing|replacing)|better to|which (?:one|is better)|what would you do)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_ONLY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\na an the i we you me us my our your it its that this these those there here what whats how why\nwhen who which should shall can could would will do does did have has had got get need needs\nwant wants any some more most all one ones two first next last step steps start starting go goes\ngoing way ways best good right proper thing things bit part parts now then again still just\nreally very quite also too so and but if or about for with to in on at from by as up down out off\nover back through\neat drink cook make take bring put keep leave buy book phone ring call ask say tell show give\nplan save spend pay wear fix choose pick sort try look think know find use work ready prepared\nmissing miss anything else needed necessary am is are was were be been being prepare preparing\nprepped prep helpful use useful much long far soon expect expecting happen happens day night\nroute routes path paths into in career change switch started starting begun beginning\npeople person someone somebody anyone everyone somebody's others group colleagues\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_TO_START_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:first step|first thing|next step|next thing|where (?:do|would|should) i (?:start|begin)|how do i start|how do i begin|what should i do first|what do i do first|where to start|what first|what now|what next|and then what)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_YOU_NEED_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what should i (?:have|get|buy|bring|take)|what do i need|what will i need|do i need anything|what else do i need|what do i have to get|am i missing anything)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_ROUTE_IN_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what(?:.s|s)? the route|what are the routes|which route|what route|what(?:.s|s)? the path|how do (?:you|people|i) (?:get|go) into|how do i get in|what(?:.s|s)? the way in|how do you break into|how do people get started|what(?:.s|s)? the way into|how do i start out)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_ATTEMPT_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:what should (?:the|my) first .{0,24} (?:look like|be like|be)|what does (?:the|my) first .{0,24} look like|how should (?:the|my) first .{0,24} go|what should i expect from (?:the|my) first|how do i start (?:it|off)|what.s a good first)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_GET_READY_ASKS(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:get ready|getting ready|prepare|preparing|be ready|get set|brush up|what should i expect|what happens on the day)(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_TO_START_LINES(py_tuple(vec![py_str("Start with what you want to end up with and what you cannot go past - money, time, or somebody else's say-so - then take the smallest action that moves it. Where are you with it now?"), py_str("Put the end result into a sentence, then the one thing that has to happen before any of the rest can. What is stopping that one thing?")]));
    s_WHAT_YOU_NEED_LINES(py_tuple(vec![py_str("The list is theirs rather than mine: whoever you are dealing with sets it and publishes it, and it is short. Read it once and tell me what is on it - it is the part people get stuck on that is worth checking before you go."), py_str("Start from what they ask for in writing rather than from memory, and go down it a line at a time. What is on their list?")]));
    s_FIRST_ATTEMPT_LINES(py_tuple(vec![py_str("Shorter and easier than you think: fifteen or twenty minutes, stop while you could still do more, and judge it by how you feel the next day rather than how you feel at the end."), py_str("The first one is a test of the plan rather than the thing itself - small enough that you would do it again, and written down so the second one is easy to repeat. What are you starting with?")]));
    s_ROUTE_IN_LINES(py_tuple(vec![py_str("There are usually three ways in: the qualification, a paid trainee or apprenticeship place where you earn while you learn, and a side door through a related job. Which of those fits depends on whether you can afford to study full time."), py_str("The doors are the same most places: the course, the paid training place, and starting lower down in the same trade and moving up. The one to take depends on what you can afford to do and what is hiring near you - want me to go through them?")]));
    s_GET_READY_LINES(py_tuple(vec![py_str("Split it in two: what will be wanted from you - what gets asked, what to bring - and the practical half like what you wear and how you get there, which is the half people leave until the morning. Which half worries you?"), py_str("Know the order things happen in and what you need with you, then sort the clothes and the journey the night before rather than on the day. What is the part you are unsure about?")]));
    s_NAMES_WHERE_TO_ASK(stdlib_call("re", "compile", &[py_str("(?<![a-z])(?:citizens advice|acas|ombudsman|council|gp|doctor|surgery|pharmacist|dentist|vet|nurse|111|999|bank|broker|adviser|advisor|solicitor|union rep|map app|their site|its site|the site|their page|the page|their own page|gov\\.uk|official site|the label|the packet|the box|the shop|the garage|a dealer|the library|the post office|the school|the landlord|the office|the helpline|the company|the operator|the insurer|the seller|the manufacturer|your employer)(?![a-z])"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THIS_CHAT_TALK(py_str("(?:talk|chat|talking|chatting|keep\\s+going|keep\\s+you\\s+here)"));
    s_ABOUT_THIS_CHAT(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^\\W*(?:(?:and|but|so|oh|ok|okay|well|right|sorry|quick|just|question|one\\s+question|quick\\s+question|first\\s+question|random\\s+question)\\W+)*(?:how\\s+(?:long|much\\s+time)\\s+(?:have|has|did)\\s+(?:we|i|you|you\\s+and\\s+i)\\s+been\\s+"), &g__THIS_CHAT_TALK()), &py_str("|how\\s+long\\s+has\\s+this\\s+been\\s+going\\s+on|(?:is|are)\\s+there\\s+(?:a|any)\\s+(?:time\\s+)?limit(?:\\s+(?:to|on)\\s+(?:(?:how\\s+long|the\\s+time)\\s+)?(?:(?:we|i)\\s+can\\s+)?(?:")), &g__THIS_CHAT_TALK()), &py_str("|this|it|the\\s+chat|our\\s+chat))?|have\\s+(?:we|i)\\s+got\\s+long)(?:\\s+(?:with|to|for)\\s+(?:you|me|this|now)|\\s+so\\s+far|\\s+now|\\s+for\\s+long|\\s+much)?[ ?!.,;]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let _ = py_str("The pool's own chooser, imported when it is called.\n\n    `fallback_chain` imports from `fallback`, so importing it at the top of this module would be a\n    cycle: by the time a line reaches here both are loaded.\n    ");
    let mut pick: Py = py_func("_choose", |args: &[Py]| crate::gen::compose::fallback_chain::_choose(args));
    return py_call(&pick, &[options.clone(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn _the_words_of(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("The words of a line, lowercased, with the punctuation and the apostrophes dropped.\n\n    \"what's the route\" is \"whats the route\" once the apostrophe goes, and a chat line rarely\n    carries one anyway: without this the same line was read one way with the apostrophe and the\n    other way without it.\n    ");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "replace", &[py_str("'"), py_str("")])); } py_list(__out) })() };
    Py::None
}
pub fn _says_nothing_of_its_own(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when every word of the line is an ask or a filler: a follow-up with no subject.");
    words = _the_words_of(&[clean.clone()]);
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ASK_ONLY_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _an_ask_with_no_subject(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("The usual first moves for the kind of help asked for, or \"\" when the ask is not one.");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_says_nothing_of_its_own(&[line.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_WHERE_TO_START_ASKS(), "search", &[line.clone()])) {
    return _choose(&[g_WHERE_TO_START_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_WHAT_YOU_NEED_ASKS(), "search", &[line.clone()])) {
    return _choose(&[g_WHAT_YOU_NEED_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_ROUTE_IN_ASKS(), "search", &[line.clone()])) {
    return _choose(&[g_ROUTE_IN_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_GET_READY_ASKS(), "search", &[line.clone()])) {
    return _choose(&[g_GET_READY_LINES(), memory.clone(), clean.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _the_whole_exchange(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The line and what was said before it, so a follow-up keeps the subject of the turn before.\n\n    \"is there anything i should watch for\" after a knee that grinds has no body in it at all, and\n    the three things to watch for on a body are a different three from the ones on a contract.\n    ");
    said = py_list(vec![line.clone()]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    return mcall(&py_str(" "), "join", &[said.clone()]);
    Py::None
}
pub fn _the_routes_hold_this(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("True when `routes.py` names a page or a rule for this line, so this file must stand down.");
    let __try1 = exc::catch(|| {
    let mut options: Py = py_func("options", |args: &[Py]| crate::gen::compose::routes::options(args));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["Exception"]) {
                __handled5 = true;
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __try6 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&options, &[line.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
    return Py::Bool(false);
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn _the_ok_lead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut route: Py = Py::None;
    let _ = py_str("The lead for a yes-or-no ask: about food and bodies only where one of them is in the line.");
    route = _the_ok_route(&[line.clone()]);
    return if py_truthy(&Py::Bool(py_is(&route.clone(), &g_IS_IT_OK_PLAIN_ROUTES()))) { g_IS_IT_OK_PLAIN_LEADS() } else { g_IS_IT_OK_LEADS() };
    Py::None
}
pub fn _the_ok_route(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut text: Py = Py::None;
    let _ = py_str("The route for a yes-or-no ask: a body first, then the kitchen, then the general one.");
    text = { let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&mcall(&g_A_BODY_IN_IT(), "search", &[text.clone()])) {
    return g_IS_IT_OK_ROUTES();
    }
    if py_truthy(&mcall(&g_A_FOOD_IN_IT(), "search", &[text.clone()])) {
    return g_IS_IT_OK_FOOD_ROUTES();
    }
    return g_IS_IT_OK_PLAIN_ROUTES();
    Py::None
}
pub fn _the_rules_route(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let _ = py_str("The route for a rule: the document one only where the line names a document.");
    return if py_truthy(&mcall(&g_DOCUMENT_NAMED(), "search", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { g_RULES_ROUTES() } else { g_RULES_PLAIN_ROUTES() };
    Py::None
}
pub fn _the_lead_and_the_route(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("What can be said about this kind of question, and where the rest lives, or \"\".");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { mcall(&g_ASKS_TO_DECIDE(), "search", &[line.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&_the_routes_hold_this(&[line.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_NEWS_RESULT(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_NEWS_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_NEWS_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_WHAT_TO_TAKE(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_WHAT_TO_TAKE_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_WHAT_TO_TAKE_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_HOW_TO_TEACH(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_HOW_TO_TEACH_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_HOW_TO_TEACH_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_LEAVING_SOMEONE(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_LEAVING_SOMEONE_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_LEAVING_SOMEONE_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_IS_IT_OK(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[_the_ok_lead(&[line.clone()]), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[_the_ok_route(&[line.clone()]), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_FIRST_ATTEMPT_ASKS(), "search", &[line.clone()])) {
    return _choose(&[g_FIRST_ATTEMPT_LINES(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_WATCH_OUT(), "search", &[line.clone()])) {
    if py_truthy(&mcall(&g_A_BODY_IN_IT(), "search", &[_the_whole_exchange(&[line.clone(), memory.clone()])])) {
    return add(&add(&_choose(&[g_WATCH_OUT_BODY_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_WATCH_OUT_BODY_ROUTES(), memory.clone(), clean.clone()]));
    }
    return add(&add(&_choose(&[g_WATCH_OUT_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_WATCH_OUT_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_A_USUAL_AMOUNT(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_USUAL_AMOUNT_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_USUAL_AMOUNT_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_SOMEONE_ELSES_OWN(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_SOMEONE_ELSES_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_SOMEONE_ELSES_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&{ let __b = mcall(&g_LIVE_TODAY(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_LIVE_ASK(), "search", &[line.clone()]) } }) {
    return add(&add(&_choose(&[g_LIVE_TODAY_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_LIVE_TODAY_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_HOW_MUCH_TO_PUT_IN(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_PUT_IN_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_PUT_IN_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_A_PRICE_THEY_WILL_PAY(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_A_PRICE_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_A_PRICE_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_GET_READY_FOR(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_GET_READY_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_GET_READY_ROUTES(), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_RULES_FOR_THEIR_CASE(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_RULES_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[_the_rules_route(&[line.clone()]), memory.clone(), clean.clone()]));
    }
    if py_truthy(&mcall(&g_A_MACHINE_OF_THEIRS(), "search", &[line.clone()])) {
    return add(&add(&_choose(&[g_A_MACHINE_LEADS(), memory.clone(), clean.clone()]), &py_str(" ")), &_choose(&[g_A_MACHINE_ROUTES(), memory.clone(), clean.clone()]));
    }
    return py_str("");
    Py::None
}
pub fn _asks_about_this_chat(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line asks how long this chat has gone on, or whether there is a limit.");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return call_builtin("bool", &[{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_ABOUT_THIS_CHAT(), "match", &[line.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.not_knowing".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THIS_CHAT" => Some(g_ABOUT_THIS_CHAT()),
            "ASKS_TO_DECIDE" => Some(g_ASKS_TO_DECIDE()),
            "ASK_ONLY_WORDS" => Some(g_ASK_ONLY_WORDS()),
            "A_BODY_IN_IT" => Some(g_A_BODY_IN_IT()),
            "A_FOOD_IN_IT" => Some(g_A_FOOD_IN_IT()),
            "A_MACHINE_LEADS" => Some(g_A_MACHINE_LEADS()),
            "A_MACHINE_OF_THEIRS" => Some(g_A_MACHINE_OF_THEIRS()),
            "A_MACHINE_ROUTES" => Some(g_A_MACHINE_ROUTES()),
            "A_PRICE_LEADS" => Some(g_A_PRICE_LEADS()),
            "A_PRICE_ROUTES" => Some(g_A_PRICE_ROUTES()),
            "A_PRICE_THEY_WILL_PAY" => Some(g_A_PRICE_THEY_WILL_PAY()),
            "A_USUAL_AMOUNT" => Some(g_A_USUAL_AMOUNT()),
            "DOCUMENT_NAMED" => Some(g_DOCUMENT_NAMED()),
            "FIRST_ATTEMPT_ASKS" => Some(g_FIRST_ATTEMPT_ASKS()),
            "FIRST_ATTEMPT_LINES" => Some(g_FIRST_ATTEMPT_LINES()),
            "GET_READY_ASKS" => Some(g_GET_READY_ASKS()),
            "GET_READY_FOR" => Some(g_GET_READY_FOR()),
            "GET_READY_LEADS" => Some(g_GET_READY_LEADS()),
            "GET_READY_LINES" => Some(g_GET_READY_LINES()),
            "GET_READY_ROUTES" => Some(g_GET_READY_ROUTES()),
            "HOW_MUCH_TO_PUT_IN" => Some(g_HOW_MUCH_TO_PUT_IN()),
            "HOW_TO_TEACH" => Some(g_HOW_TO_TEACH()),
            "HOW_TO_TEACH_LEADS" => Some(g_HOW_TO_TEACH_LEADS()),
            "HOW_TO_TEACH_ROUTES" => Some(g_HOW_TO_TEACH_ROUTES()),
            "IS_IT_OK" => Some(g_IS_IT_OK()),
            "IS_IT_OK_FOOD_ROUTES" => Some(g_IS_IT_OK_FOOD_ROUTES()),
            "IS_IT_OK_LEADS" => Some(g_IS_IT_OK_LEADS()),
            "IS_IT_OK_PLAIN_LEADS" => Some(g_IS_IT_OK_PLAIN_LEADS()),
            "IS_IT_OK_PLAIN_ROUTES" => Some(g_IS_IT_OK_PLAIN_ROUTES()),
            "IS_IT_OK_ROUTES" => Some(g_IS_IT_OK_ROUTES()),
            "LEAVING_SOMEONE" => Some(g_LEAVING_SOMEONE()),
            "LEAVING_SOMEONE_LEADS" => Some(g_LEAVING_SOMEONE_LEADS()),
            "LEAVING_SOMEONE_ROUTES" => Some(g_LEAVING_SOMEONE_ROUTES()),
            "LIVE_ASK" => Some(g_LIVE_ASK()),
            "LIVE_TODAY" => Some(g_LIVE_TODAY()),
            "LIVE_TODAY_LEADS" => Some(g_LIVE_TODAY_LEADS()),
            "LIVE_TODAY_ROUTES" => Some(g_LIVE_TODAY_ROUTES()),
            "NAMES_WHERE_TO_ASK" => Some(g_NAMES_WHERE_TO_ASK()),
            "NEWS_LEADS" => Some(g_NEWS_LEADS()),
            "NEWS_RESULT" => Some(g_NEWS_RESULT()),
            "NEWS_ROUTES" => Some(g_NEWS_ROUTES()),
            "POINTER_HEDGE" => Some(g_POINTER_HEDGE()),
            "PUT_IN_LEADS" => Some(g_PUT_IN_LEADS()),
            "PUT_IN_ROUTES" => Some(g_PUT_IN_ROUTES()),
            "ROUTE_IN_ASKS" => Some(g_ROUTE_IN_ASKS()),
            "ROUTE_IN_LINES" => Some(g_ROUTE_IN_LINES()),
            "RULES_FOR_THEIR_CASE" => Some(g_RULES_FOR_THEIR_CASE()),
            "RULES_LEADS" => Some(g_RULES_LEADS()),
            "RULES_PLAIN_ROUTES" => Some(g_RULES_PLAIN_ROUTES()),
            "RULES_ROUTES" => Some(g_RULES_ROUTES()),
            "SOMEONE_ELSES_LEADS" => Some(g_SOMEONE_ELSES_LEADS()),
            "SOMEONE_ELSES_OWN" => Some(g_SOMEONE_ELSES_OWN()),
            "SOMEONE_ELSES_ROUTES" => Some(g_SOMEONE_ELSES_ROUTES()),
            "USUAL_AMOUNT_LEADS" => Some(g_USUAL_AMOUNT_LEADS()),
            "USUAL_AMOUNT_ROUTES" => Some(g_USUAL_AMOUNT_ROUTES()),
            "WATCH_OUT" => Some(g_WATCH_OUT()),
            "WATCH_OUT_BODY_LEADS" => Some(g_WATCH_OUT_BODY_LEADS()),
            "WATCH_OUT_BODY_ROUTES" => Some(g_WATCH_OUT_BODY_ROUTES()),
            "WATCH_OUT_LEADS" => Some(g_WATCH_OUT_LEADS()),
            "WATCH_OUT_ROUTES" => Some(g_WATCH_OUT_ROUTES()),
            "WHAT_TO_TAKE" => Some(g_WHAT_TO_TAKE()),
            "WHAT_TO_TAKE_LEADS" => Some(g_WHAT_TO_TAKE_LEADS()),
            "WHAT_TO_TAKE_ROUTES" => Some(g_WHAT_TO_TAKE_ROUTES()),
            "WHAT_YOU_NEED_ASKS" => Some(g_WHAT_YOU_NEED_ASKS()),
            "WHAT_YOU_NEED_LINES" => Some(g_WHAT_YOU_NEED_LINES()),
            "WHERE_TO_START_ASKS" => Some(g_WHERE_TO_START_ASKS()),
            "WHERE_TO_START_LINES" => Some(g_WHERE_TO_START_LINES()),
            "_THIS_CHAT_TALK" => Some(g__THIS_CHAT_TALK()),
            "_an_ask_with_no_subject" => Some(py_func("_an_ask_with_no_subject", |args: &[Py]| _an_ask_with_no_subject(args))),
            "_asks_about_this_chat" => Some(py_func("_asks_about_this_chat", |args: &[Py]| _asks_about_this_chat(args))),
            "_choose" => Some(py_func("_choose", |args: &[Py]| _choose(args))),
            "_says_nothing_of_its_own" => Some(py_func("_says_nothing_of_its_own", |args: &[Py]| _says_nothing_of_its_own(args))),
            "_the_lead_and_the_route" => Some(py_func("_the_lead_and_the_route", |args: &[Py]| _the_lead_and_the_route(args))),
            "_the_ok_lead" => Some(py_func("_the_ok_lead", |args: &[Py]| _the_ok_lead(args))),
            "_the_ok_route" => Some(py_func("_the_ok_route", |args: &[Py]| _the_ok_route(args))),
            "_the_routes_hold_this" => Some(py_func("_the_routes_hold_this", |args: &[Py]| _the_routes_hold_this(args))),
            "_the_rules_route" => Some(py_func("_the_rules_route", |args: &[Py]| _the_rules_route(args))),
            "_the_whole_exchange" => Some(py_func("_the_whole_exchange", |args: &[Py]| _the_whole_exchange(args))),
            "_the_words_of" => Some(py_func("_the_words_of", |args: &[Py]| _the_words_of(args))),
            _ => None,
        }),
        set: None,
    }))
}

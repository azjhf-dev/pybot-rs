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


thread_local! { static G_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_CLAIM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BOUNDARY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAIM_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAIM_WITHOUT_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE_OF_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CLAUSE_OPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DATE_PROPERTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DENIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KIND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LEADING_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_PROPERTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUOTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STATEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_AGO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRAILING_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERB_IN_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_DENIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_DENIAL_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALLBACK_MISSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALLED_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECKED_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_OF_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIMED_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIMS_TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAIM_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KNOW_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOWERABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOTHING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONLY_A_POINTING_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEMBERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REMEMBER_TELLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AS_IT_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_OF_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_OF_SOMEONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMETHING_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELLING_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_KINDS_KEPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOLD_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_I_LIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_AN_ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOURS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_others: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_pick: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_say_it_back: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_user: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.told: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ALREADY() -> Py { ensure_init(); G_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ALREADY(v: Py) { ensure_init(); G_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL() -> Py { ensure_init(); G_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL(v: Py) { ensure_init(); G_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS() -> Py { ensure_init(); G_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS(v: Py) { ensure_init(); G_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_CLAIM() -> Py { ensure_init(); G_A_BARE_CLAIM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_CLAIM(v: Py) { ensure_init(); G_A_BARE_CLAIM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BOUNDARY() -> Py { ensure_init(); G_A_BOUNDARY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BOUNDARY(v: Py) { ensure_init(); G_A_BOUNDARY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAIM_BEFORE() -> Py { ensure_init(); G_A_CLAIM_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAIM_BEFORE(v: Py) { ensure_init(); G_A_CLAIM_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAIM_WITHOUT_YOU() -> Py { ensure_init(); G_A_CLAIM_WITHOUT_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAIM_WITHOUT_YOU(v: Py) { ensure_init(); G_A_CLAIM_WITHOUT_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE() -> Py { ensure_init(); G_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE(v: Py) { ensure_init(); G_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE_OF_MINE() -> Py { ensure_init(); G_A_CLAUSE_OF_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE_OF_MINE(v: Py) { ensure_init(); G_A_CLAUSE_OF_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CLAUSE_OPENING() -> Py { ensure_init(); G_A_CLAUSE_OPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CLAUSE_OPENING(v: Py) { ensure_init(); G_A_CLAUSE_OPENING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DATE_PROPERTY() -> Py { ensure_init(); G_A_DATE_PROPERTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DATE_PROPERTY(v: Py) { ensure_init(); G_A_DATE_PROPERTY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DENIAL() -> Py { ensure_init(); G_A_DENIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DENIAL(v: Py) { ensure_init(); G_A_DENIAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KIND() -> Py { ensure_init(); G_A_KIND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KIND(v: Py) { ensure_init(); G_A_KIND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LEADING_TIME() -> Py { ensure_init(); G_A_LEADING_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LEADING_TIME(v: Py) { ensure_init(); G_A_LEADING_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME() -> Py { ensure_init(); G_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME(v: Py) { ensure_init(); G_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NUMBER() -> Py { ensure_init(); G_A_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NUMBER(v: Py) { ensure_init(); G_A_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_PROPERTY() -> Py { ensure_init(); G_A_PLACE_PROPERTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_PROPERTY(v: Py) { ensure_init(); G_A_PLACE_PROPERTY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUOTE() -> Py { ensure_init(); G_A_QUOTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUOTE(v: Py) { ensure_init(); G_A_QUOTE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STATEMENT() -> Py { ensure_init(); G_A_STATEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STATEMENT(v: Py) { ensure_init(); G_A_STATEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_AGO() -> Py { ensure_init(); G_A_TIME_AGO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_AGO(v: Py) { ensure_init(); G_A_TIME_AGO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_ONLY() -> Py { ensure_init(); G_A_TIME_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_ONLY(v: Py) { ensure_init(); G_A_TIME_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRAILING_TIME() -> Py { ensure_init(); G_A_TRAILING_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRAILING_TIME(v: Py) { ensure_init(); G_A_TRAILING_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRIP() -> Py { ensure_init(); G_A_TRIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRIP(v: Py) { ensure_init(); G_A_TRIP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERB_IN_A_CLAUSE() -> Py { ensure_init(); G_A_VERB_IN_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERB_IN_A_CLAUSE(v: Py) { ensure_init(); G_A_VERB_IN_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_DENIAL() -> Py { ensure_init(); G_BARE_DENIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DENIAL(v: Py) { ensure_init(); G_BARE_DENIAL.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_DENIAL_PRONOUN() -> Py { ensure_init(); G_BARE_DENIAL_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_DENIAL_PRONOUN(v: Py) { ensure_init(); G_BARE_DENIAL_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_CALLBACK_MISSED() -> Py { ensure_init(); G_CALLBACK_MISSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALLBACK_MISSED(v: Py) { ensure_init(); G_CALLBACK_MISSED.with(|c| *c.borrow_mut() = v); }
pub fn g_CALLED_NAME() -> Py { ensure_init(); G_CALLED_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALLED_NAME(v: Py) { ensure_init(); G_CALLED_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECKED_ELSEWHERE() -> Py { ensure_init(); G_CHECKED_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECKED_ELSEWHERE(v: Py) { ensure_init(); G_CHECKED_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_OF_SOMEONE() -> Py { ensure_init(); G_CHECK_OF_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_OF_SOMEONE(v: Py) { ensure_init(); G_CHECK_OF_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIMED_ABOUT() -> Py { ensure_init(); G_CLAIMED_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIMED_ABOUT(v: Py) { ensure_init(); G_CLAIMED_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIMS_TOLD() -> Py { ensure_init(); G_CLAIMS_TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIMS_TOLD(v: Py) { ensure_init(); G_CLAIMS_TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAIM_FILLER() -> Py { ensure_init(); G_CLAIM_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAIM_FILLER(v: Py) { ensure_init(); G_CLAIM_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRACTIONS() -> Py { ensure_init(); G_CONTRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRACTIONS(v: Py) { ensure_init(); G_CONTRACTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTING() -> Py { ensure_init(); G_CORRECTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTING(v: Py) { ensure_init(); G_CORRECTING.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTION_WORDS() -> Py { ensure_init(); G_CORRECTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTION_WORDS(v: Py) { ensure_init(); G_CORRECTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); G_COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); G_COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_QUESTION() -> Py { ensure_init(); G_IN_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_QUESTION(v: Py) { ensure_init(); G_IN_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g_KNOW_ABOUT() -> Py { ensure_init(); G_KNOW_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KNOW_ABOUT(v: Py) { ensure_init(); G_KNOW_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_THERE() -> Py { ensure_init(); G_LEFT_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_THERE(v: Py) { ensure_init(); G_LEFT_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOWERABLE() -> Py { ensure_init(); G_LOWERABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOWERABLE(v: Py) { ensure_init(); G_LOWERABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_MINE() -> Py { ensure_init(); G_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINE(v: Py) { ensure_init(); G_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_NAMED() -> Py { ensure_init(); G_NOTHING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_NAMED(v: Py) { ensure_init(); G_NOTHING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOTHING_SAID() -> Py { ensure_init(); G_NOTHING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOTHING_SAID(v: Py) { ensure_init(); G_NOTHING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE() -> Py { ensure_init(); G_NOT_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE(v: Py) { ensure_init(); G_NOT_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_ONLY_A_POINTING_TIME() -> Py { ensure_init(); G_ONLY_A_POINTING_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONLY_A_POINTING_TIME(v: Py) { ensure_init(); G_ONLY_A_POINTING_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING() -> Py { ensure_init(); G_POINTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING(v: Py) { ensure_init(); G_POINTING.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_SAID() -> Py { ensure_init(); G_POINTING_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_SAID(v: Py) { ensure_init(); G_POINTING_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_TAIL() -> Py { ensure_init(); G_POINTING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_TAIL(v: Py) { ensure_init(); G_POINTING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_WORDS() -> Py { ensure_init(); G_RELATION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_WORDS(v: Py) { ensure_init(); G_RELATION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEMBERED() -> Py { ensure_init(); G_REMEMBERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEMBERED(v: Py) { ensure_init(); G_REMEMBERED.with(|c| *c.borrow_mut() = v); }
pub fn g_REMEMBER_TELLING() -> Py { ensure_init(); G_REMEMBER_TELLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REMEMBER_TELLING(v: Py) { ensure_init(); G_REMEMBER_TELLING.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED() -> Py { ensure_init(); G_REPORTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED(v: Py) { ensure_init(); G_REPORTED.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AS_IT_IS() -> Py { ensure_init(); G_SAID_AS_IT_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_IT_IS(v: Py) { ensure_init(); G_SAID_AS_IT_IS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_OF_PRONOUN() -> Py { ensure_init(); G_SAID_OF_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_OF_PRONOUN(v: Py) { ensure_init(); G_SAID_OF_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_OF_SOMEONE() -> Py { ensure_init(); G_SAID_OF_SOMEONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_OF_SOMEONE(v: Py) { ensure_init(); G_SAID_OF_SOMEONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMETHING_ABOUT() -> Py { ensure_init(); G_SOMETHING_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMETHING_ABOUT(v: Py) { ensure_init(); G_SOMETHING_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_TELLING_ALREADY() -> Py { ensure_init(); G_TELLING_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELLING_ALREADY(v: Py) { ensure_init(); G_TELLING_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_KINDS_KEPT() -> Py { ensure_init(); G_THE_KINDS_KEPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_KINDS_KEPT(v: Py) { ensure_init(); G_THE_KINDS_KEPT.with(|c| *c.borrow_mut() = v); }
pub fn g_TOLD_ABOUT() -> Py { ensure_init(); G_TOLD_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOLD_ABOUT(v: Py) { ensure_init(); G_TOLD_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_I_LIVE() -> Py { ensure_init(); G_WHERE_I_LIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_I_LIVE(v: Py) { ensure_init(); G_WHERE_I_LIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_AN_ARTICLE() -> Py { ensure_init(); G_WITH_AN_ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_AN_ARTICLE(v: Py) { ensure_init(); G_WITH_AN_ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_YOURS() -> Py { ensure_init(); G_YOURS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOURS(v: Py) { ensure_init(); G_YOURS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_STOP() -> Py { ensure_init(); G__THE_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_STOP(v: Py) { ensure_init(); G__THE_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_others() -> Py { ensure_init(); G_others.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_others(v: Py) { ensure_init(); G_others.with(|c| *c.borrow_mut() = v); }
pub fn g_pick() -> Py { ensure_init(); G_pick.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_pick(v: Py) { ensure_init(); G_pick.with(|c| *c.borrow_mut() = v); }
pub fn g_say_it_back() -> Py { ensure_init(); G_say_it_back.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_say_it_back(v: Py) { ensure_init(); G_say_it_back.with(|c| *c.borrow_mut() = v); }
pub fn g_user() -> Py { ensure_init(); G_user.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_user(v: Py) { ensure_init(); G_user.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Whether they already told the bot about a thing, answered from the chat.\n\n\"have I told you about my sister?\" is a question about the conversation, not about the world,\nand the store is the only part of the bot that can answer it. The shape was reaching the search\nhedge (\"That's beyond what I know well, sorry\"), which is wrong twice over: the thing is in the\nchat, and the person is asking the bot to remember rather than to look anything up.\n\nWhat the store holds is said back in the order that reads best:\n\n1. the fact against the person it is about - \"my sister lives in Cork\" is their sister's line,\n   and the store keeps what it knows of her;\n2. a pet by its kind - \"have I told you about the cat?\" is the cat called Milo;\n3. the line itself, quoted, when no fact was kept: their own words still answer the question,\n   and \"you haven't told me about your sister yet\" to someone who just did is the worst answer\n   available;\n4. the fact of the kind they asked about (\"the trip\", \"my job\") when the ask names a kind\n   rather than a thing, because the plan's words never appear in the ask.\n\nAnything else is left alone (`None`), including an ask about a thing the store has never heard\nof: that is the honest \"nothing yet\", which is a memory answer too, so it is said here rather\nthan pushed onto a module that knows nothing about the person.\n");
    s_pick(py_func("pick", |args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args)));
    s_others(py_func("others", |args: &[Py]| pyrt::xrefs::xcall("memory", "others", args)));
    s_user(py_func("user", |args: &[Py]| pyrt::xrefs::xcall("memory", "user", args)));
    s_say_it_back(py_func("you", |args: &[Py]| pyrt::xrefs::xcall("memory", "you", args)));
    s_TOLD_ABOUT(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+|so\\s+|but\\s+|ok(?:ay)?\\s+|wait\\s+|quick\\s+)?(?:have|has|had|did)\\s+(?:i|we)\\s+(?:ever\\s+|already\\s+)?(?:told|tell|mentioned|mention|said|say|talked|talk)\\s+(?:you\\s+)?(?P<about>about\\s+|of\\s+)?(?P<what>[^?]+?)\\s*\\??$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KNOW_ABOUT(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+|so\\s+|but\\s+|ok(?:ay)?\\s+|wait\\s+|quick\\s+|sorry\\s+)?(?:do|did|can|could)\\s+(?:you|u)\\s+(?:still\\s+|actually\\s+|even\\s+|really\\s+)?(?:know|remember|recall)\\s+(?P<what>(?:anything\\s+|something\\s+|more\\s+)?(?:about\\s+|on\\s+)?(?:my|our)\\s+\\w[^?]*?|(?:what|where|who|which|how\\s+old)\\s+(?:i|we)\\s+[^?]*?)\\s*\\??$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_STATEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|am|will|would|can|could|have|has|had|do|does|did|i|we|you|im|i'm|ive|i've|id|i'd|ill|i'll|we're|we've|youre|you're|dont|don't|didnt|didn't|cant|can't|wont|won't)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_NAMED(stdlib_call("re", "compile", &[py_str("^what\\s+(?:i|we)\\s+(?:told|said)(?:\\s+(?:you|u|me))?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING(call_builtin("frozenset", &[mcall(&py_str("me it that this them us anything something everything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_QUESTION(py_tuple(vec![py_tuple(vec![py_str("^(?:how old|what age)\\s+(?:i|we)\\s+(?:am|are|was|were)\\b.*$"), py_str("my age")]), py_tuple(vec![py_str("^what\\s+(?:i|we)\\s+(?:do|did)(?:\\s+for (?:a living|work))?\\b.*$"), py_str("what i do")]), py_tuple(vec![py_str("^who\\s+(?:i|we)\\s+(?:am|are|was|were)\\b.*$"), py_str("my name")]), py_tuple(vec![py_str("^what\\s+(?:i|we)(?:'m|'re| am|are|was|were)\\s+(?:allergic|intolerant)\\s+to\\b.*$"), py_str("what im allergic to")]), py_tuple(vec![py_str("^where\\s+(?:i|we)\\s+(?:live|lived|come from|was born|grew up|am from|'m from)\\b.*$"), py_str("where i live")])]));
    fn __data1() -> Py { py_dict(vec![(py_str("trip"), py_str("plan")), (py_str("trips"), py_str("plan")), (py_str("holiday"), py_str("plan")), (py_str("holidays"), py_str("plan")), (py_str("break"), py_str("plan")), (py_str("travel"), py_str("plan")), (py_str("flight"), py_str("plan")), (py_str("flights"), py_str("plan")), (py_str("hotel"), py_str("plan")), (py_str("booking"), py_str("plan")), (py_str("plan"), py_str("plan")), (py_str("plans"), py_str("plan")), (py_str("job"), py_str("job")), (py_str("work"), py_str("job")), (py_str("money"), py_str("budget")), (py_str("budget"), py_str("budget")), (py_str("broke"), py_str("budget")), (py_str("skint"), py_str("budget")), (py_str("cash"), py_str("budget")), (py_str("savings"), py_str("budget")), (py_str("car"), py_str("car")), (py_str("vehicle"), py_str("car")), (py_str("van"), py_str("car")), (py_str("drive"), py_str("car")), (py_str("driving"), py_str("car")), (py_str("drove"), py_str("car")), (py_str("mot"), py_str("car")), (py_str("bike"), py_str("bike")), (py_str("bicycle"), py_str("bike")), (py_str("cycling"), py_str("bike")), (py_str("cycle"), py_str("bike")), (py_str("running"), py_str("running")), (py_str("run"), py_str("running")), (py_str("jogging"), py_str("running")), (py_str("jog"), py_str("running")), (py_str("runner"), py_str("running")), (py_str("feelings"), py_str("feelings")), (py_str("feeling"), py_str("feelings")), (py_str("feel"), py_str("feelings")), (py_str("felt"), py_str("feelings")), (py_str("mood"), py_str("feelings")), (py_str("emotions"), py_str("feelings")), (py_str("emotion"), py_str("feelings")), (py_str("flat"), py_str("flat")), (py_str("house"), py_str("flat")), (py_str("home"), py_str("flat")), (py_str("landlord"), py_str("flat")), (py_str("rent"), py_str("flat")), (py_str("boiler"), py_str("flat")), (py_str("heating"), py_str("flat")), (py_str("mortgage"), py_str("flat")), (py_str("age"), py_str("age")), (py_str("name"), py_str("name")), (py_str("diet"), py_str("diet")), (py_str("allergy"), py_str("allergies")), (py_str("allergies"), py_str("allergies")), (py_str("kids"), py_str("kids")), (py_str("kid"), py_str("kids")), (py_str("children"), py_str("kids")), (py_str("child"), py_str("kids"))]) }
    s_A_KIND(__data1());
    s_RELATION_WORDS(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pyrt::xrefs::xget("memory.facts", "RELATIONS")); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_CHECK_OF_SOMEONE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:is|are|was|were|does|do|did)\\s+(?:my|our|your)\\s+(?:"), &g_RELATION_WORDS()), &py_str(")(?:'s|s)?\\s+(?P<claim>[^?.!]{1,44})\\s*[?.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTION_WORDS(py_str("(?:and|also|so|actually|no|sorry|wait|oh|hmm|hold\\s+on|correction|i\\s+mean)"));
    s_CORRECTING(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:"), &g_CORRECTION_WORDS()), &py_str("\\b[,.!;:]?\\s*){1,2}")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_OF_SOMEONE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^\\W*(?:"), &g_CORRECTION_WORDS()), &py_str("\\b[,.!;:]?\\s*){0,2}(?:my|our)\\s+(?P<who>")), &g_RELATION_WORDS()), &py_str(")(?:'s|s)?\\s+(?:(?P<not>isnt|isn't|is not|wasnt|wasn't|was not|arent|aren't|are not|doesnt|doesn't|does not|didnt|didn't|did not|hasnt|hasn't|has not|is n't)\\s+(?P<pred>[^.,;!?]{1,40})|(?P<verb>is|was|are|were|lives|works|has|got|goes|plays|studies|drives|does|did)\\s+(?P<pred2>[^.,;!?]{1,40}))")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_OF_PRONOUN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:"), &g_CORRECTION_WORDS()), &py_str("\\b[,.!;:]?\\s*){0,2}(?P<pron>she|he|they)(?:'s|s)?\\s+(?:(?P<not>isnt|isn't|is not|wasnt|wasn't|was not|arent|aren't|are not|doesnt|doesn't|does not|didnt|didn't|did not|hasnt|hasn't|has not)\\s+(?P<pred>[^.,;!?]{1,40})|(?P<verb>is|was|are|were|lives|works|has|got|goes|plays|studies|drives|does|did)\\s+(?P<pred2>[^.,;!?]{1,40}))")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DENIAL(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^\\W*(?:"), &g_CORRECTION_WORDS()), &py_str("\\b[,.!;:]?\\s*){0,2}(?:my|our)\\s+(?P<who>")), &g_RELATION_WORDS()), &py_str(")?(?:'s|s)?\\s*(?P<not>isnt|isn't|is not|wasnt|wasn't|was not|arent|aren't|are not|doesnt|doesn't|does not|didnt|didn't|did not|hasnt|hasn't|has not)\\s*(?:either|at all|really|actually|any ?more|no)?\\s*[.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_DENIAL_PRONOUN(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:"), &g_CORRECTION_WORDS()), &py_str("\\b[,.!;:]?\\s*){0,2}(?:she|he|they)(?:'s|s)?\\s*(?P<not>isnt|isn't|is not|wasnt|wasn't|was not|arent|aren't|are not|doesnt|doesn't|does not|didnt|didn't|did not|hasnt|hasn't|has not)\\s*(?:either|at all|really|actually|any ?more|no)?\\s*[.!]*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NUMBER(stdlib_call("re", "compile", &[py_str("^\\d+(?:\\s*(?:years?|months?)(?:\\s+old)?)?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME(stdlib_call("re", "compile", &[py_str("^(?:called|named)\\s+\\S+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WITH_AN_ARTICLE(stdlib_call("re", "compile", &[py_str("^(?:a|an)\\s+\\S+$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIM_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the some my our your of to in on at called is was are were be"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECKED_ELSEWHERE(stdlib_call("re", "compile", &[py_str("allerg|\\beat|eats|eating|vegan|vegetarian|pescatarian|diet"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:dog|cat|puppy|kitten|rabbit|hamster|bird|fish|parrot|horse|tortoise|guinea pig|pet)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_I_LIVE(stdlib_call("re", "compile", &[py_str("^\\W*(?:where\\s+(?:i|we)\\s+(?:live|stay)|where\\s+(?:i|we)'?re\\s+based|my\\s+(?:address|house|home|flat|place)|the\\s+place\\s+(?:i|we)\\s+live)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MINE(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:mum|mom|dad|father|mother|sister|brother|son|daughter|husband|wife|partner|friend|boss|niece|nephew|aunt|uncle|grandma|grandad|grandmother|grandfather|cousin)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ALREADY(stdlib_call("re", "compile", &[py_str("\\s+(?:already|before|yet|so far|then|earlier)\\b.*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS(stdlib_call("re", "compile", &[py_str("^\\W*(?:what|why|how|when|where|who|which|whose|did|do|does|is|are|was|were|can|could|should|would|will|have|has|am)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMEMBERED(stdlib_call("re", "compile", &[py_str("^\\W*(?:oh\\s+|and\\s+|so\\s+|but\\s+|hey\\s+|wait\\s+|quick\\s+|ok(?:ay)?\\s+)?remember\\s+(?:when\\s+|that\\s+)?(?:(?:i|we)\\s+(?:said|say|told\\s+you|tell\\s+you|mentioned|mention|talked|spoken|spoke|chatted|went\\s+on|were\\s+on)|me\\s+(?:saying|telling\\s+you|mentioning))\\s*(?:about\\s+|of\\s+)?(?P<what>[^?]+?)\\s*\\??$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CALLBACK_MISSED(py_tuple(vec![py_str("I don't have that in this chat's notes — say it again and I'll keep it."), py_str("Nothing about that in what I've kept here — tell me and it's in."), py_str("I've only got this conversation to go on, and that isn't in it yet.")]));
    s_A_DENIAL(stdlib_call("re", "compile", &[py_str("^\\W*(?:i haven't kept a note|there's nothing on|nothing on)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIMED_ABOUT(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+|so\\s+|but\\s+|ok(?:ay)?\\s+|look\\s+|listen\\s+|hey\\s+)?(?:i|we)\\s+(?:already\\s+|just\\s+|definitely\\s+|did\\s+|have\\s+|had\\s+)?(?:told|tell|mentioned|mention|said|say|talked|talk|spoke)\\s+(?:you|u)\\b\\s*(?:(?:about|of|that)\\s+)?(?P<what>[^?]+?)\\s*[.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAIM_WITHOUT_YOU(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+|so\\s+|but\\s+|ok(?:ay)?\\s+|look\\s+|listen\\s+|hey\\s+)?(?:i|we)\\s+(?:already\\s+|just\\s+|did\\s+|have\\s+|had\\s+)?(?:mentioned|mention|talked|talk|spoke)\\s+(?:(?:about|of|that)\\s+)?(?P<what>[^?]+?)\\s+(?:earlier|before|already|ages\\s+ago|a\\s+minute\\s+ago|just\\s+now|last\\s+(?:time|week|night))\\s*[.?!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_AGO(stdlib_call("re", "compile", &[py_str("\\b(?:earlier|before|already|yesterday|last\\s+(?:time|week|night)|ages\\s+ago|a\\s+minute\\s+ago|just\\s+now|in\\s+the\\s+(?:last|previous)\\s+(?:chat|conversation))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRAILING_TIME(stdlib_call("re", "compile", &[py_str("\\s+(?:earlier|before|already|yesterday|ages\\s+ago|just\\s+now|a\\s+minute\\s+ago|last\\s+(?:time|week|night))\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REMEMBER_TELLING(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+|so\\s+|but\\s+|ok(?:ay)?\\s+|wait\\s+|quick\\s+)?(?:do|did|can|could)\\s+(?:you|u)\\s+(?:remember|recall)\\s+(?:(?:that|when|me|i|us|we)\\s+){0,2}(?:told|telling|tell|talked|talking|talk|mentioned|mentioning|mention|said|saying|say)(?:\\s+(?:you|u))?\\s*(?:about\\s+|of\\s+)?(?P<what>[^?]+?)\\s*\\??$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAUSE(stdlib_call("re", "compile", &[py_str("^(?:i|we|im|i'm|ive|i've|id|i'd|he|she|they|you|it)\\b\\s*\\w*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_STOP(py_str("(?:talk|speak|discuss|go\\s+into|get\\s+into)"));
    s_A_BOUNDARY(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("^\\W*(?:and\\s+|but\\s+|so\\s+|ok(?:ay)?\\s+|well\\s+|look\\s+|honestly\\s+|please\\s+|just\\s+)*(?:(?:i|we)\\s+(?:(?:really|just|honestly|actually|kind\\s+of|sort\\s+of)\\s+)*(?:don'?t|dont|do\\s+not|wouldn'?t|wouldnt|would\\s+not|never)\\s+(?:want|wanna|wish|like)\\s+to\\s+"), &g__THE_STOP()), &py_str("\\s+(?:about|of)\\s+(?:it|that|this|that\\s+one|this\\s+one)|(?:i|we)(?:'d|\\s+would)\\s+rather\\s+not\\s+")), &g__THE_STOP()), &py_str("\\s+(?:about|of)?\\s*(?:it|that|this)?|(?:can\\s+we|can\\s+i|let'?s|lets)\\s+(?:not\\s+")), &g__THE_STOP()), &py_str("\\s+(?:about\\s+)?(?:it|that|this)|drop\\s+(?:it|this\\s+one)|leave\\s+(?:it|that))|(?:leave|drop)\\s+(?:it|that|this))\\s*(?:please|for\\s+now|now|alright|ok|okay|thanks|thank\\s+you)?\\W*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAIM_BEFORE(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|but\\s+|so\\s+|ok(?:ay)?\\s+|look\\s+|listen\\s+|hey\\s+)?(?:i|we)\\s+(?:already\\s+|just\\s+|did\\s+)?(?:told|tell|said|say|mentioned|mention)\\s+(?:(?:you|u)\\b[^,;]*[,;]\\s*|(?=i\\s|we\\s|im\\s|i'm\\s))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEFT_THERE(py_tuple(vec![py_str("Fair enough - I'll leave that one alone."), py_str("Of course. I won't bring it up again."), py_str("Alright. That's the last I'll say on it."), py_str("No problem - let's leave that one."), py_str("Understood. Nothing more on it from me."), py_str("That's fair. I'll not raise it again.")]));
    s_A_LEADING_TIME(stdlib_call("re", "compile", &[py_str("^\\W*(?:already|before|earlier|just|then|so\\s+far)\\b[,;:]?\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_ONLY(stdlib_call("re", "compile", &[py_str("^\\W*(?:already|before|earlier|that|it|this|you\\s+know|you\\s+already\\s+know|yes|yeah|right)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_CLAIM(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|but\\s+|so\\s+|ok(?:ay)?\\s+|look\\s+|listen\\s+|hey\\s+)?(?:i|we)\\s+(?:already\\s+|just\\s+|did\\s+)?(?:told|tell|said|say|mentioned|mention)\\s+(?:you|u)?\\s*(?:about\\s+)?(?:it|that|this|already|before|earlier|so\\s+far|you\\s+already\\s+know)?\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMETHING_ABOUT(stdlib_call("re", "compile", &[py_str("^(?:anything|something|much|any|more|else|stuff)\\s+(?:about|on|regarding|to do with)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_AS_IT_IS(stdlib_call("re", "compile", &[py_str("^(?:yes|no|nothing|you haven't|i haven't|there's nothing|there is nothing|not that i)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LOWERABLE(call_builtin("frozenset", &[mcall(&py_str("you your it its they the there that this we he she i"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUOTE(stdlib_call("re", "compile", &[py_str("[\"“”]")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DATE_PROPERTY(call_builtin("frozenset", &[py_tuple(vec![py_str("when"), py_str("what day"), py_str("what date"), py_str("what time"), py_str("which day"), py_str("the day"), py_str("the date"), py_str("the time")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE_PROPERTY(call_builtin("frozenset", &[py_tuple(vec![py_str("where"), py_str("what place"), py_str("which place"), py_str("the place")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAUSE_OPENING(stdlib_call("re", "compile", &[py_str("^(?:i|we|he|she|they|it|you|there|my|our)\\b|\\b(?:was|were|am|is|are|had|have|has|did|do|does|will|would|can|could|should)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORTED(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|also\\s+)?(?:i|we)\\s+(?:did\\s+)?(?:say|said|told\\s+you|mentioned|was\\s+saying|were\\s+saying)\\b[:,]?\\s+(?P<clause>(?!about\\b|of\\b).+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TELLING_ALREADY(stdlib_call("re", "compile", &[py_str("^you (?:told me|said to me|mentioned to me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MINE(stdlib_call("re", "compile", &[py_str("\\b(my|our|me|mine|ours)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_YOURS(py_dict(vec![(py_str("my"), py_str("your")), (py_str("our"), py_str("your")), (py_str("me"), py_str("you")), (py_str("mine"), py_str("yours")), (py_str("ours"), py_str("yours"))]));
    s_A_VERB_IN_A_CLAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:was|were|am|is|are|had|have|has|did|do|does|will|would|can|could|should|going)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CLAUSE_OF_MINE(stdlib_call("re", "compile", &[py_str("^(my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONTRACTIONS(py_dict(vec![(py_tuple(vec![py_str("isnt")]), py_str("isn't")), (py_tuple(vec![py_str("wasnt")]), py_str("wasn't")), (py_tuple(vec![py_str("doesnt")]), py_str("doesn't")), (py_tuple(vec![py_str("didnt")]), py_str("didn't")), (py_tuple(vec![py_str("hasnt")]), py_str("hasn't")), (py_tuple(vec![py_str("arent")]), py_str("aren't")), (py_tuple(vec![py_str("dont")]), py_str("don't")), (py_tuple(vec![py_str("cant")]), py_str("can't")), (py_tuple(vec![py_str("wont")]), py_str("won't")), (py_tuple(vec![py_str("is"), py_str("n't")]), py_str("isn't"))]));
    s_CALLED_NAME(stdlib_call("re", "compile", &[py_str("\\b(?:called|named)\\s+([a-z][a-z'\\-]*)")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DENIAL(stdlib_call("re", "compile", &[py_str("^\\W*(?:not\\b|n't\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_WORDS(py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten"))]));
    s_ONLY_A_POINTING_TIME(stdlib_call("re", "compile", &[py_str("^\\W*(?:already|before|earlier|yesterday|that|this|it|them|just\\s+now|ages\\s+ago|a\\s+minute\\s+ago|the\\s+other\\s+day|last\\s+(?:time|night|week))\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:already|before|earlier|just\\s+now|ages\\s+ago|a\\s+minute\\s+ago|the\\s+other\\s+day|last\\s+time)\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_SAID(stdlib_call("re", "compile", &[py_str("^\\W*(?:and\\s+|also\\s+|so\\s+|but\\s+|ok(?:ay)?\\s+|look\\s+|listen\\s+|hey\\s+)?(?:i|we)\\s+(?:said|told\\s+you|told\\s+u)\\s+(?:that|it|this|them)\\s+(?:already|before|earlier|just\\s+now|ages\\s+ago|a\\s+minute\\s+ago|the\\s+other\\s+day|last\\s+(?:time|week|night))\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRIP(stdlib_call("re", "compile", &[py_str("\\b(?:went|going|booked|flying|flight|holiday|trip|visiting|staying|travel(?:ling|ing)?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIMS_TOLD(stdlib_call("re", "compile", &[py_str("\\b(?:told|mentioned|said|talked)\\s+(?:you|about|it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOTHING_SAID(stdlib_call("re", "compile", &[py_str("^\\W*(?:you\\s+(?:haven't|have not|don't|do not|'ve not|ve not|didn't)|nothing|no\\b|not\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_KINDS_KEPT(py_tuple(vec![py_str("allergies"), py_str("budget"), py_str("diet"), py_str("dislikes"), py_str("tastes"), py_str("plan"), py_str("places"), py_str("kids")]));
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
    let mut verb: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut pred: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pred") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pred"))) };
    let mut kind: Py = Py::None;
    let _ = py_str("What a fact is about, in the coarsest words that tell one fact from another.\n\n    (the verb, the sort of thing said of them): (\"is\", \"number\") for \"my son is 7\", (\"is\",\n    \"called\") for \"my son is called tom\", (\"is\", \"a\") for \"my dad is a teacher\". A fact with the\n    same shape as the claim is the newest word on the same thing, and no older line may overrule\n    it.\n    ");
    kind = py_str("said");
    if py_truthy(&mcall(&g_A_NUMBER(), "match", &[mcall(&pred.clone(), "strip", &[])])) {
    kind = py_str("number");
    } else {
    if py_truthy(&mcall(&g_A_NAME(), "match", &[mcall(&pred.clone(), "strip", &[])])) {
    kind = py_str("called");
    } else {
    if py_truthy(&mcall(&g_WITH_AN_ARTICLE(), "match", &[mcall(&pred.clone(), "strip", &[])])) {
    kind = py_str("a");
    }
    }
    }
    return py_tuple(vec![verb.clone(), kind.clone()]);
    Py::None
}
pub fn _the_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fact: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fact") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fact"))) };
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("A fact's verb, with a denial read as the thing it denies: \"isnt\" is \"is\".");
    verb = mcall(&fact.clone(), "group", &[py_str("verb")]);
    if py_truthy(&verb.clone()) {
    return mcall(&verb.clone(), "lower", &[]);
    }
    said = mcall(&mcall(&{ let __b = mcall(&fact.clone(), "group", &[py_str("not")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), "replace", &[py_str(" n't"), py_str("nt")]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("is"), py_str("is")]), py_tuple(vec![py_str("was"), py_str("was")]), py_tuple(vec![py_str("are"), py_str("are")]), py_tuple(vec![py_str("does"), py_str("does")]), py_tuple(vec![py_str("did"), py_str("did")]), py_tuple(vec![py_str("has"), py_str("has")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    one = __unpk2[0].clone();
    plain = __unpk2[1].clone();
    if py_truthy(&mcall(&said.clone(), "startswith", &[one.clone()])) {
    return plain.clone();
    }
    }
    return said.clone();
    Py::None
}
pub fn _the_same_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fact: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fact") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fact"))) };
    let mut pred: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pred") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pred"))) };
    let mut claim: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "claim") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: claim"))) };
    let _ = py_str("Whether a fact is about the same thing as the claim, said with a different value.\n\n    \"is my son 7\" against a line that now says \"my son is 8\": the same verb, both bare numbers,\n    so the 8 is the newest word on his age and the 7 is not confirmed by an older line.\n    ");
    return Py::Bool(py_eq(&_the_shape(&[_the_verb(&[fact.clone()]), pred.clone()]), &_the_shape(&[py_str("is"), claim.clone()])));
    Py::None
}
pub fn _tells_nothing(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("True when the line claims the bot was told and names nothing that it was told.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_A_BARE_CLAIM(), "match", &[line.clone()])) {
    return Py::Bool(true);
    }
    match_ = { let __b = mcall(&g_CLAIMED_ABOUT(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_CLAIM_WITHOUT_YOU(), "match", &[line.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    what = mcall(&g_A_TRAILING_TIME(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "split", &[])])]);
    what = mcall(&mcall(&g_A_LEADING_TIME(), "sub", &[py_str(""), what.clone()]), "strip", &[py_str(" .,!")]);
    return { let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_A_TIME_ONLY(), "match", &[what.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _handed_back(args: &[Py]) -> Py {
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
    let mut lines: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("\"So you did - you said your knee hurts.\", from the last thing they said, or None.\n\n    Their own last line is the thing the claim is about, and it goes back in the second person\n    the way every other store reply does. A chat with nothing in it has nothing to hand back, and\n    the claim is left to whichever module answers the news.\n    ");
    mine = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    lines = py_list(vec![]);
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { turns.clone() } else { py_list(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[mcall(&turn.clone(), "get", &[py_str("user"), py_str("")])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    let _ = mcall(&lines.clone(), "append", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    said = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&said.clone(), "lower", &[]), &mine.clone()))) {
    continue;
    }
    quoted = _quote(&[said.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&quoted.clone()))) {
    continue;
    }
    quoted = op_add(&mcall(&geti(&quoted.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&quoted.clone(), &Py::Int(1), &Py::None, &Py::None));
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&py_str("So you did - "), &quoted.clone()), op_add(&py_str("You're right - "), &quoted.clone()), op_add(&py_str("You did - "), &quoted.clone()), op_add(&py_str("Fair enough - "), &quoted.clone())]), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn is_a_boundary(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True when the line is the person closing a subject rather than asking about one.");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_CLAIM_BEFORE(), "match", &[line.clone()])) {
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(line.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CLAIM_BEFORE(), "sub", args))(&__args) };
    }
    return call_builtin("bool", &[mcall(&g_A_BOUNDARY(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn at_a_boundary(args: &[Py]) -> Py {
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
    let _ = py_str("The short answer to a boundary, or None when the line is not one.");
    if py_truthy(&Py::Bool(!py_truthy(&is_a_boundary(&[text.clone()])))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_LEFT_THERE()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    Py::None
}
pub fn _a_boundary_in_a_claim(args: &[Py]) -> Py {
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
    let _ = py_str("True when the telling in front of the boundary tells nothing: it is a preamble.");
    return call_builtin("bool", &[mcall(&g_A_CLAIM_BEFORE(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ask(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The thing they ask about (\"my sister\", \"the trip\"), or None when the line is not this ask.");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&is_a_boundary(&[line.clone()])) {
    return Py::None;
    }
    match_ = { let __b = mcall(&g_TOLD_ABOUT(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_KNOW_ABOUT(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CLAIMED_ABOUT(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_CLAIM_WITHOUT_YOU(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REMEMBER_TELLING(), "match", &[line.clone()]) } } } } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    said = mcall(&mcall(&g_A_TRAILING_TIME(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "split", &[])])]), "strip", &[py_str(" .,!")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![g_CLAIMED_ABOUT(), g_A_CLAIM_WITHOUT_YOU(), g_REMEMBER_TELLING()]), &gattr(&match_.clone(), "re"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTING(), &mcall(&geti(&mcall(&said.clone(), "split", &[]), &Py::Int(0)), "lower", &[]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&gattr(&match_.clone(), "re"), &g_TOLD_ABOUT())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&match_.clone(), "group", &[py_str("about")]))); if !py_truthy(&__b) { __b } else { mcall(&g_A_STATEMENT(), "search", &[said.clone()]) } } } }) {
    return Py::None;
    }
    return said.clone();
    Py::None
}
pub fn ABOUT_THE_CHAT(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("True when the line is this ask, whatever word in it another module would claim.\n\n    A specialist module proposes from one word: \"have we talked about food\" is handed to the\n    answers lane because of \"food\", and \"did we talk about my mum\" because of \"mum\", so the store\n    is never asked about a line that is about the chat itself. The store says which of its own\n    proposals this is (bot/modules/arbiter.py, TAKES_AS_IT_COMES).\n    ");
    if py_truthy(&_tells_nothing(&[text.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&is_a_boundary(&[text.clone()])) {
    return Py::Bool(true);
    }
    said = _the_ask(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    what = mcall(&stdlib_call("re", "sub", &[py_str("^(?:the|my|our|a|an)\\s+"), py_str(""), said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,!")]);
    what = mcall(&g_ALREADY(), "sub", &[py_str(""), what.clone()]);
    return { let __b = call_builtin("bool", &[what.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINTING(), &mcall(&what.clone(), "lower", &[]))) } };
    Py::None
}
pub fn answer(args: &[Py]) -> Py {
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
    let mut facts: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut plain: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plain") { v.clone() }
    else { Py::None };
    let mut claims: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "claims") { v.clone() }
    else { Py::None };
    let mut checked: Py = Py::None;
    let mut handed: Py = Py::None;
    let mut pointed: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The thing they ask about, said back from the chat, or None when the line is not this ask.\n\n    `facts` is the store's own reader for each kind of fact (bot/modules/memory_store.py,\n    `_told_readers`), so an ask about their allergies is answered with the same sentence the\n    store says for \"what am i allergic to\": one set of sentences, and the two cannot drift apart.\n    `plain` is the store's answer to the same ask in its plain words (\"do you know what i do\" ->\n    \"what i do\"), which is how a fact the store keeps under a shape this module has no kind for\n    (the car they drive, the job they do) is answered rather than denied. `claims` says whether\n    the store reads the line as being about what it keeps, which is what makes a denial honest:\n    \"do you know my age\" may be told nothing is kept, \"do you know where i can buy paint\" may not.\n    ");
    if py_truthy(&is_a_boundary(&[text.clone()])) {
    return at_a_boundary(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&_tells_nothing(&[text.clone()])) {
    handed = _handed_back(&[text.clone(), memory.clone()]);
    if py_truthy(&handed.clone()) {
    return handed.clone();
    }
    }
    if py_truthy(&{ let __b = _a_claim_about_the_chat(&[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_earlier_line(&[memory.clone(), text.clone()]))) } }) {
    return Py::None;
    }
    checked = check(&[text.clone(), memory.clone()]);
    if py_truthy(&checked.clone()) {
    return checked.clone();
    }
    pointed = _a_claim_that_points(&[text.clone(), memory.clone()]);
    if py_truthy(&pointed.clone()) {
    return pointed.clone();
    }
    said = _the_answer(&[text.clone(), memory.clone(), facts.clone(), plain.clone(), claims.clone()]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_DENIAL(), "match", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_A_CLAUSE(), "match", &[{ let __b = _the_ask(&[text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]) } } } }) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_CALLBACK_MISSED()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SAID_AS_IT_IS(), "match", &[said.clone()]))) } }) {
    return _with_the_yes(&[said.clone(), text.clone()]);
    }
    return said.clone();
    Py::None
}
pub fn a_denial(args: &[Py]) -> Py {
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
    let _ = py_str("True when the sentence says the chat does not hold the thing - a miss, not a recall.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g_A_DENIAL(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_QUOTE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SAID_AS_IT_IS(), "match", &[said.clone()]) } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _lower_first(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let _ = py_str("The sentence with a common opener lowercased, ready to follow \"Yes — \".");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return said.clone();
    }
    first = geti(&mcall(&mcall(&mcall(&geti(&mcall(&said.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str(".,!?\"'—-")]), "lower", &[]), "split", &[py_str("'")]), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&g_LOWERABLE(), &first.clone()))) {
    return op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return said.clone();
    Py::None
}
pub fn _with_the_yes(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("`said` with \"Yes — \" in front of it when the line is a yes/no question about the chat.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_QUOTE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_TOLD_ABOUT(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } } } }) {
    return said.clone();
    }
    return op_add(&py_str("Yes — "), &_lower_first(&[said.clone()]));
    Py::None
}
pub fn _the_remembered(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("\"my knee hurt\" out of \"remember when i said my knee hurt\", or None when it is not this.");
    match_ = mcall(&g_REMEMBERED(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "split", &[])]), "strip", &[py_str(" .,!")]);
    said = mcall(&mcall(&g_ALREADY(), "sub", &[py_str(""), said.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTING(), &mcall(&said.clone(), "lower", &[]))) } }) {
    return Py::None;
    }
    return said.clone();
    Py::None
}
pub fn A_CALLBACK(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks the bot to remember a thing they said, whatever else it names.\n\n    A callback names something the chat already heard (\"remember when i said my knee hurt\"), so\n    the word in it is also a topic the knowledge base reads: `answers` took the line on \"knee\"\n    and resigned the advice. The store is the only part of the bot that can say what was said, so\n    it says so in front of its reply (bot/modules/arbiter.py, `TAKES_AS_IT_COMES`).\n    ");
    return call_builtin("bool", &[_the_remembered(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn a_callback_the_chat_has(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("True when the line is a callback to something this chat already holds.\n\n    The store files each line as the subject of the conversation (bot/memory/topics.py, `note`),\n    and a callback names a subject the conversation is on already. Kept as a new subject, the\n    word \"remember\" went into the topic and the turn after it was read against remembering.\n    ");
    said = _the_remembered(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    what = mcall(&stdlib_call("re", "sub", &[py_str("^(?:the|my|our|a|an)\\s+"), py_str(""), said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,!")]);
    return call_builtin("bool", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push({ let __b = what.clone(); if py_truthy(&__b) { __b } else { said.clone() } }); __kws.extend(vec![("skip".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_about(args))(&__args) }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_claim_about_the_chat(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line claims something of the chat was already told or mentioned.");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[{ let __b = mcall(&g_CLAIMED_ABOUT(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_CLAIM_WITHOUT_YOU(), "match", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _an_earlier_line(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("Whether the chat holds a line of theirs other than this one.\n\n    One check for the whole bot (bot/memory/history.py): a reader that counts turns for itself\n    here would disagree with the recall readers about what counts as an earlier turn.\n    ");
    return pyrt::xrefs::xcall("memory.history", "an_earlier_turn", &[memory.clone(), text.clone()]);
    Py::None
}
pub fn callback(args: &[Py]) -> Py {
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
    let mut facts: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut plain: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plain") { v.clone() }
    else { Py::None };
    let mut claims: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "claims") { v.clone() }
    else { Py::None };
    let mut recalled: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The thing they said, handed back to them, or None when the line is not this.\n\n    A callback is about the chat, not about the world: the answer is the thing itself in their\n    own words, or the fact the store keeps of it, and never the advice again. A callback naming\n    nothing the store has is answered as a missing note rather than as a correction of what they\n    remember, because the bot cannot see into the conversation the person is thinking of.\n    ");
    said = _the_remembered(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&is_a_boundary(&[text.clone()])) {
    return at_a_boundary(&[text.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = _a_claim_about_the_chat(&[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_earlier_line(&[memory.clone(), text.clone()]))) } }) {
    return Py::None;
    }
    recalled = answer(&[op_add(&py_str("have i told you about "), &said.clone()), memory.clone(), facts.clone(), plain.clone(), claims.clone()]);
    if py_truthy(&{ let __b = recalled.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_DENIAL(), "match", &[call_builtin("str", &[recalled.clone()]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return recalled.clone();
    }
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_CALLBACK_MISSED()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    Py::None
}
pub fn _the_answer(args: &[Py]) -> Py {
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
    let mut facts: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut plain: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plain") { v.clone() }
    else { Py::None };
    let mut claims: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "claims") { v.clone() }
    else { Py::None };
    let mut bare: Py = Py::None;
    let mut dated: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kind_words: Py = Py::None;
    let mut know: Py = Py::None;
    let mut known: Py = Py::None;
    let mut named: Py = Py::None;
    let mut noframe: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut plain_ask: Py = Py::None;
    let mut pointed: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut said: Py = Py::None;
    let mut someone: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("The readers that answer one thing they ask about, in the order that reads best.");
    said = _the_ask(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    bare = mcall(&mcall(&g_SOMETHING_ABOUT(), "sub", &[py_str(""), said.clone()]), "strip", &[]);
    pointed = _the_proof_they_asked_for(&[bare.clone(), text.clone(), memory.clone()]);
    if py_truthy(&pointed.clone()) {
    return pointed.clone();
    }
    noframe = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    know = { let __b = mcall(&g_KNOW_ABOUT(), "match", &[noframe.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOTHING_NAMED(), "match", &[bare.clone()]))) } };
    if py_truthy(&mcall(&g_NOTHING_NAMED(), "match", &[bare.clone()])) {
    return Py::None;
    }
    if py_truthy(&know.clone()) {
    let __iter1 = py_iter_or_raise(&g_IN_QUESTION());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    plain_ask = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "match", &[pattern.clone(), bare.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    bare = plain_ask.clone();
    break;
    }
    }
    }
    if py_truthy(&{ let __b = plain.clone(); if !py_truthy(&__b) { __b } else { { let __b = know.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINTING(), &mcall(&bare.clone(), "lower", &[]))) } } } }) {
    kept = py_call(&plain, &[bare.clone()]);
    if py_truthy(&kept.clone()) {
    return kept.clone();
    }
    }
    what = mcall(&stdlib_call("re", "sub", &[py_str("^(?:the|my|our|a|an)\\s+"), py_str(""), bare.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(" .,!")]);
    what = mcall(&mcall(&g_ALREADY(), "sub", &[py_str(""), what.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&what.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTING(), &mcall(&what.clone(), "lower", &[]))) } }) {
    return Py::None;
    }
    known = pyrt::xrefs::xcall("memory", "user", &[memory.clone()]);
    theirs = _about_a_person(&[memory.clone(), what.clone()]);
    if py_truthy(&Py::Bool(!py_is(&theirs.clone(), &Py::None))) {
    return theirs.clone();
    }
    someone = _the_person_a_pronoun_points_at(&[memory.clone(), what.clone()]);
    if py_truthy(&someone.clone()) {
    return _about_a_person(&[memory.clone(), someone.clone()]);
    }
    pet = _about_a_pet(&[memory.clone(), what.clone(), known.clone(), facts.clone()]);
    if py_truthy(&pet.clone()) {
    return pet.clone();
    }
    if py_truthy(&mcall(&g_WHERE_I_LIVE(), "match", &[what.clone()])) {
    said = py_call(&{ let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { { let bare = bare.clone(); let claims = claims.clone(); let dated = dated.clone(); let facts = facts.clone(); let kept = kept.clone(); let kind_words = kind_words.clone(); let know = know.clone(); let known = known.clone(); let memory = memory.clone(); let named = named.clone(); let noframe = noframe.clone(); let pattern = pattern.clone(); let pet = pet.clone(); let plain = plain.clone(); let plain_ask = plain_ask.clone(); let pointed = pointed.clone(); let quoted = quoted.clone(); let said = said.clone(); let someone = someone.clone(); let text = text.clone(); let theirs = theirs.clone(); let what = what.clone(); py_func("lambda", move |args: &[Py]| { let bare = bare.clone(); let claims = claims.clone(); let dated = dated.clone(); let facts = facts.clone(); let kept = kept.clone(); let kind_words = kind_words.clone(); let know = know.clone(); let known = known.clone(); let memory = memory.clone(); let named = named.clone(); let noframe = noframe.clone(); let pattern = pattern.clone(); let pet = pet.clone(); let plain = plain.clone(); let plain_ask = plain_ask.clone(); let pointed = pointed.clone(); let quoted = quoted.clone(); let said = said.clone(); let someone = someone.clone(); let text = text.clone(); let theirs = theirs.clone(); let what = what.clone(); let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); py_str("") }) } } }, &[memory.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    if py_truthy(&_names_only_a_kind(&[what.clone()])) {
    kept = _the_fact_of_that_kind(&[memory.clone(), what.clone(), known.clone(), facts.clone()]);
    if py_truthy(&kept.clone()) {
    return kept.clone();
    }
    }
    kind_words = _the_words_of_the_kind(&[what.clone()]);
    if py_truthy(&kind_words.clone()) {
    quoted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(mcall(&py_str(" "), "join", &[call_builtin("sorted", &[kind_words.clone()]).unwrap_or_else(|e| exc::raise(e))])); __kws.extend(vec![("whole_words".to_string(), Py::Bool(true)), ("skip".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_about(args))(&__args) };
    if py_truthy(&quoted.clone()) {
    return _quote(&[quoted.clone(), memory.clone()]);
    }
    }
    kept = _the_fact_of_that_kind(&[memory.clone(), what.clone(), known.clone(), facts.clone()]);
    if py_truthy(&kept.clone()) {
    return kept.clone();
    }
    named = _the_fact_that_names(&[memory.clone(), what.clone(), known.clone(), facts.clone()]);
    if py_truthy(&named.clone()) {
    return named.clone();
    }
    quoted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(what.clone()); __kws.extend(vec![("skip".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_about(args))(&__args) };
    if py_truthy(&quoted.clone()) {
    return _quote(&[quoted.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = know.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&claims.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&py_call(&claims, &[]))) } } } }) {
    return Py::None;
    }
    dated = _the_property_of_the_thing(&[memory.clone(), bare.clone(), facts.clone()]);
    if py_truthy(&dated.clone()) {
    return dated.clone();
    }
    return _no_note(&[_yours(&[bare.clone()]), memory.clone()]);
    Py::None
}
pub fn _the_property_of_the_thing(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut facts: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: facts"))) };
    let mut asked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut when_answer: Py = Py::None;
    let _ = py_str("What the chat holds for the property the ask names, when it names the property only.");
    asked = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,!?")]);
    if py_truthy(&Py::Bool(py_contains(&g_A_DATE_PROPERTY(), &asked.clone()))) {
    when_answer = py_func("when_answer", |args: &[Py]| pyrt::xrefs::xcall("memory.when", "when_answer", args));
    return { let __b = py_call(&when_answer, &[py_str("when is it"), memory.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_PLACE_PROPERTY(), &asked.clone()))) {
    said = py_call(&{ let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { { let asked = asked.clone(); let facts = facts.clone(); let memory = memory.clone(); let said = said.clone(); let what = what.clone(); let when_answer = when_answer.clone(); py_func("lambda", move |args: &[Py]| { let asked = asked.clone(); let facts = facts.clone(); let memory = memory.clone(); let said = said.clone(); let what = what.clone(); let when_answer = when_answer.clone(); let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); py_str("") }) } } }, &[memory.clone()]);
    return { let __b = said.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    }
    return Py::None;
    Py::None
}
pub fn _a_clause_is_named(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let _ = py_str("True when a claim about the chat names a sentence as well as the telling.");
    found = { let __b = mcall(&g_CLAIMED_ABOUT(), "match", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if py_truthy(&__b) { __b } else { mcall(&g_A_CLAIM_WITHOUT_YOU(), "match", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_CLAUSE(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&found.clone(), "group", &[py_str("what")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_thing_the_chat_is_on(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let _ = py_str("The phrase the chat is on, from the store's own reader, or \"\" when it names nothing.");
    about = py_func("_the_thing_the_chat_is_on", |args: &[Py]| pyrt::xrefs::xcall("memory.store_replies", "_the_thing_the_chat_is_on", args));
    return { let __b = py_call(&about, &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    Py::None
}
pub fn _no_note(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("The denial, said about the note rather than about the chat.\n\n    \"im saving for a car\" then \"have i told you anything about money\" was answered \"You haven't\n    told me about money yet.\" - a claim about what the person said, and false: they had said\n    something about money, in words the store files as a plan. How people say money, or a\n    friend, is unbounded (\"saving for a car\", \"best mate\"), so no list of words closes it. What\n    the bot does know for certain is its own note, so that is what it speaks for: a note that\n    has nothing under that word is true however they said it.\n    ");
    if py_truthy(&_the_thing_is_a_clause(&[thing.clone()])) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("Nothing in my notes says “"), &thing.clone()), &py_str("” yet — tell me and I'll keep it.")), op_add(&op_add(&py_str("I don't have “"), &thing.clone()), &py_str("” in my notes — say it and I'll hold on to it.")), op_add(&op_add(&py_str("“"), &thing.clone()), &py_str("” isn't in what I keep yet — tell me and I'll remember it."))]), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("I haven't kept a note of "), &thing.clone()), &py_str(" — tell me and I'll remember it.")), op_add(&op_add(&py_str("There's nothing on "), &thing.clone()), &py_str(" in what I keep — say it and I'll hold on to it.")), op_add(&op_add(&py_str("Nothing on "), &thing.clone()), &py_str(" in my notes yet — tell me and I'll keep it."))]), memory.clone()]);
    Py::None
}
pub fn _the_thing_is_a_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut words: Py = Py::None;
    let _ = py_str("True when what they handed back is a sentence they said, not the name of a thing.");
    words = mcall(&call_builtin("str", &[{ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_A_CLAUSE_OPENING(), "search", &[mcall(&py_str(" "), "join", &[words.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _quote(args: &[Py]) -> Py {
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
    let mut opened: Py = Py::None;
    let mut reported: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Their own line, said back to them, when no fact of the store says it better.\n\n    The line is theirs, so it goes back in the second person and without the quote marks:\n    \"You said your kids are 6 and 9\" says what a friend would, where `You said: \"my kids are 6\n    and 9\"` hands the person a stranger's words - \"my\" is the bot talking about somebody else's\n    kids. Every other reply this store gives is in the second person, and a quote was the one\n    place it was not.\n    ");
    reported = mcall(&g_REPORTED(), "match", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&reported.clone()) {
    line = mcall(&reported.clone(), "group", &[py_str("clause")]);
    }
    opened = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s*m\\b")); __args.push(py_str("i'm")); __args.push(call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    opened = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bi\\s*ve\\b")); __args.push(py_str("i've")); __args.push(opened.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    said = mcall(&mcall(&pyrt::xrefs::xcall("memory", "you", &[opened.clone()]), "strip", &[]), "rstrip", &[py_str(".")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![py_str("You told me that already"), py_str("You did say that")]), memory.clone()]);
    }
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "lower", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    let __aug1 = op_add(&said.clone(), &py_str("."));
    said = __aug1;
    if py_truthy(&mcall(&g_TELLING_ALREADY(), "match", &[said.clone()])) {
    return op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&py_str("You said "), &said.clone()), op_add(&py_str("You told me "), &said.clone()), op_add(&py_str("You did say "), &said.clone())]), memory.clone()]);
    Py::None
}
pub fn _the_words_of_the_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut what: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut key: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every word the store files under the kind they asked about, or an empty set.\n\n    \"the job\" is filed as `job`, and the lines people write about it say \"work\", so both words\n    have to find it. A kind is asked about one at a time; \"my money and my job\" is not this.");
    kinds = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), mcall(&call_builtin("str", &[{ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    kind = _the_kind_of(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    let _ = mcall(&kinds.clone(), "add", &[kind.clone()]);
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[kinds.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    kind = mcall(&kinds.clone(), "pop", &[]);
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_KIND(), "items", &[])) { let __unpk8309 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk8309[0].clone(); let mut key: Py = __unpk8309[1].clone(); if py_truthy(&Py::Bool(py_eq(&key.clone(), &kind.clone()))) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn _yours(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The thing asked about, said from the bot's side: \"my back\" -> \"your back\".\n\n    A thing is a name and takes the other person's possessive. A whole clause is not: \"i was\n    quitting my job\" handed back with only the \"my\" turned is neither their words nor the bot's\n    (\"i was quitting your job\"), so a clause is handed back as it was said and quoted.\n    ");
    said = call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_A_VERB_IN_A_CLAUSE(), "search", &[said.clone()])) {
    return said.clone();
    }
    return mcall(&g_MINE(), "sub", &[{ let phrase = phrase.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let phrase = phrase.clone(); let said = said.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); mcall(&g_YOURS(), "get", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), mcall(&found.clone(), "group", &[Py::Int(1)])]) }) }, said.clone()]);
    Py::None
}
pub fn _the_person_a_pronoun_points_at(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut entry: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Who \"her\", \"him\" or \"them\" is: the last person the chat named, or \"\".\n\n    \"my sister lives in cork\" then \"have i told you about her\" is a question about the sister, and\n    the store has their line about her. Answering \"you haven't told me about her yet\" says the\n    opposite of the truth to the person who has just said it.\n    ");
    word = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "rstrip", &[py_str(".,!?")]);
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("her"), py_str("him"), py_str("them"), py_str("they")]), &word.clone()))) {
    return py_str("");
    }
    entry = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[pyrt::xrefs::xcall("memory", "others", &[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    kept = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&kept.clone(), "get", &[py_str("who")]) } }) {
    entry = kept.clone();
    break;
    }
    }
    if py_truthy(&entry.clone()) {
    return mcall(&mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("who"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")]);
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    other = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &mcall(&other.clone(), "rstrip", &[py_str("s")])))) {
    return mcall(&other.clone(), "rstrip", &[py_str("s")]);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _words_of(args: &[Py]) -> Py {
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
    let _ = py_str("The words a claim or a fact is made of: no articles, no punctuation, no filler.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = { let __b = Py::Bool(!py_contains(&g_CLAIM_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }; if py_truthy(&__b) { __b } else { mcall(&word.clone(), "isdigit", &[]) } }) {  __out.push(word.clone()); } } py_set(__out) })() };
    Py::None
}
pub fn check(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut by_pronoun: Py = Py::None;
    let mut claim: Py = Py::None;
    let mut denial: Py = Py::None;
    let mut fact: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut newer: Py = Py::None;
    let mut newer_said: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut pred: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut taken_back: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A yes/no check of a fact the chat gave about someone else, answered from that line.\n\n    \"my mum is coeliac\" then \"is my mum coeliac\" was answered with the search hedge, while \"what\n    do you know about my mum\" said the fact back. The fact is in the chat either way, and a\n    question about it is a memory question.\n\n    Only a **yes** is said here, and a **no** only when the chat said the same thing the other\n    way round (\"my mum isnt coeliac\"), because the claim has to be in the line for the answer to\n    be sound: \"is my mum ok\" after \"my mum is coeliac\" is a different question, and the hedge is\n    a better answer to it than a confident \"no\". Anything that does not match is left to the\n    readers that already answer it (`None`).\n    ");
    found = mcall(&g_CHECK_OF_SOMEONE(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    claim = mcall(&found.clone(), "group", &[py_str("claim")]);
    if py_truthy(&mcall(&g_CHECKED_ELSEWHERE(), "search", &[claim.clone()])) {
    return Py::None;
    }
    asked = _words_of(&[claim.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    who = mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]);
    who = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z]+"), who.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    newer = Py::None;
    newer_said = py_str("");
    taken_back = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[_their_lines(&[memory.clone(), who.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    said = __item.clone();
    fact = mcall(&g_SAID_OF_SOMEONE(), "match", &[said.clone()]);
    by_pronoun = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&fact.clone()))) {
    bare = { let __b = mcall(&g_BARE_DENIAL_PRONOUN(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BARE_DENIAL(), "match", &[said.clone()]) } };
    if py_truthy(&Py::Bool(!py_is(&bare.clone(), &Py::None))) {
    named = mcall(&mcall(&mcall(&{ let __b = mcall(&mcall(&bare.clone(), "groupdict", &[]), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&named.clone(), &who.clone())) } }) {
    taken_back = Py::Bool(true);
    }
    continue;
    }
    fact = mcall(&g_SAID_OF_PRONOUN(), "match", &[said.clone()]);
    by_pronoun = call_builtin("bool", &[fact.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&Py::Bool(!py_truthy(&fact.clone()))) {
    continue;
    }
    pred = { let __b = mcall(&fact.clone(), "group", &[py_str("pred")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&fact.clone(), "group", &[py_str("pred2")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&op_le(&asked.clone(), &_words_of(&[pred.clone()]))))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&newer.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _the_same_thing(&[fact.clone(), pred.clone(), claim.clone()]) } }) {
    newer = mcall(&py_str(" "), "join", &[mcall(&pred.clone(), "split", &[])]);
    newer_said = said.clone();
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&newer.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&newer.clone(), &mcall(&py_str(" "), "join", &[mcall(&pred.clone(), "split", &[])]))) } }) {
    return op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("You've told me your "), &who.clone()), &py_str(" is ")), &newer.clone()), &py_str(", and before that ")), &mcall(&py_str(" "), "join", &[mcall(&pred.clone(), "split", &[])])), &py_str(". Which is right?"));
    }
    denial = Py::Bool(!py_eq(&call_builtin("bool", &[mcall(&fact.clone(), "group", &[py_str("not")])]).unwrap_or_else(|e| exc::raise(e)), &taken_back.clone()));
    if py_truthy(&by_pronoun.clone()) {
    said_back = _names_in_capitals(&[mcall(&_plain_contractions(&[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &py_str(" ")), &{ let __b = mcall(&fact.clone(), "group", &[py_str("not")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&fact.clone(), "group", &[py_str("verb")]); if py_truthy(&__b) { __b } else { py_str("") } } } }), &py_str(" ")), &pred.clone())]), "strip", &[])]);
    } else {
    if py_truthy(&taken_back.clone()) {
    said_back = _the_denial(&[who.clone(), _the_verb(&[fact.clone()]), pred.clone()]);
    } else {
    said_back = _names_in_capitals(&[_plain_contractions(&[mcall(&_your_own_clause(&[said.clone()]), "strip", &[py_str(" .,!")])])]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said_back.clone()))) {
    continue;
    }
    opening = if py_truthy(&Py::Bool(!py_eq(&denial.clone(), &_claim_is_a_denial(&[claim.clone()])))) { py_str("No — ") } else { py_str("Yes — ") };
    return op_add(&op_add(&op_add(&opening.clone(), &mcall(&geti(&said_back.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&said_back.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    if py_truthy(&{ let __b = newer.clone(); if !py_truthy(&__b) { __b } else { { let __b = newer_said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&_the_shape(&[py_str("is"), newer.clone()]), &Py::Int(1)), &py_str("said"))) } } } }) {
    said_back = _names_in_capitals(&[_plain_contractions(&[mcall(&_your_own_clause(&[newer_said.clone()]), "strip", &[py_str(" .,!")])])]);
    if py_truthy(&said_back.clone()) {
    return op_add(&op_add(&mcall(&geti(&said_back.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said_back.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    }
    return Py::None;
    Py::None
}
pub fn _your_own_clause(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("Their line about their own person, said from the bot's side: \"your mum is 71\".");
    line = mcall(&g_CORRECTING(), "sub", &[py_str(""), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]);
    found = mcall(&g_A_CLAUSE_OF_MINE(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return line.clone();
    }
    return op_add(&mcall(&g_YOURS(), "get", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), py_str("your")]), &slice_or_raise(&line.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    Py::None
}
pub fn _plain_contractions(args: &[Py]) -> Py {
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
    let mut joined: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line with the words a person writes as one word put back together.");
    let __iter1 = py_iter_or_raise(&mcall(&g_CONTRACTIONS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    joined = __unpk2[1].clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &mcall(&py_str("\\s+"), "join", &[words.clone()])), &py_str("\\b"))); __args.push(joined.clone()); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return said.clone();
    Py::None
}
pub fn _names_in_capitals(args: &[Py]) -> Py {
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
    return mcall(&g_CALLED_NAME(), "sub", &[{ let said = said.clone(); py_func("lambda", move |args: &[Py]| { let said = said.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); op_add(&op_add(&py_str("called "), &mcall(&slice_or_raise(&mcall(&found.clone(), "group", &[Py::Int(1)]), &Py::None, &Py::Int(1), &Py::None), "upper", &[])), &slice_or_raise(&mcall(&found.clone(), "group", &[Py::Int(1)]), &Py::Int(1), &Py::None, &Py::None)) }) }, said.clone()]);
    Py::None
}
pub fn _claim_is_a_denial(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut claim: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "claim") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: claim"))) };
    return call_builtin("bool", &[mcall(&g_A_DENIAL(), "match", &[call_builtin("str", &[{ let __b = claim.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _their_lines(args: &[Py]) -> Py {
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
    let mut who: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut kept: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Every line of theirs about that person, oldest first, and the ones said of her by pronoun.\n\n    \"my mum is coeliac\" then \"sorry she isnt\" is two lines about the mum, and the second one does\n    not say her name: the pronoun points at the person the chat last named, which is what makes\n    it the newest word on her fact.\n    ");
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&lines.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("limit".to_string(), Py::Int(80))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _trim(args))(&__args) }]);
    } else {
    if py_truthy(&{ let __b = lines.clone(); if !py_truthy(&__b) { __b } else { mcall(&_a_pronoun_for(&[who.clone()]), "search", &[said.clone()]) } }) {
    let _ = mcall(&lines.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("limit".to_string(), Py::Int(80))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _trim(args))(&__args) }]);
    }
    }
    }
    kept = mcall(&pyrt::xrefs::xcall("memory", "user", &[memory.clone()]), "get", &[py_str("person")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&kept.clone(), "get", &[py_str("said")]), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&lines.clone(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&kept.clone(), &py_str("said"))); __kws.extend(vec![("limit".to_string(), Py::Int(80))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _trim(args))(&__args) }]);
    }
    return lines.clone();
    Py::None
}
pub fn _about_a_person(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut entry: Py = Py::None;
    let mut fact: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What the store holds about a relation of theirs, or None when the ask names no one.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&what.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    who = mcall(&mcall(&word.clone(), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&Py::Bool(!py_contains(&pyrt::xrefs::xget("memory.facts", "RELATIONS"), &who.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&pyrt::xrefs::xcall("memory", "others", &[memory.clone()]));
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")]), &who.clone()))) {
    continue;
    }
    fact = _their_fact(&[who.clone(), entry.clone()]);
    if py_truthy(&fact.clone()) {
    return fact.clone();
    }
    }
    quoted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(who.clone()); __kws.extend(vec![("whole_words".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_about(args))(&__args) };
    if py_truthy(&quoted.clone()) {
    return _quote(&[quoted.clone(), memory.clone()]);
    }
    return _no_note(&[op_add(&py_str("your "), &who.clone()), memory.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _their_fact(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut entry: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut allergies: Py = Py::None;
    let mut item: Py = Py::None;
    let mut name: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("One fact about someone else, said with the relation in it.");
    if py_truthy(&mcall(&entry.clone(), "get", &[py_str("allergies")])) {
    allergies = geti(&entry.clone(), &py_str("allergies"));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[allergies.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))))) {
    allergies = py_list(vec![allergies.clone()]);
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("You said your "), &who.clone()), &py_str(" is allergic to ")), &mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&allergies.clone()) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str("."));
    }
    if py_truthy(&mcall(&entry.clone(), "get", &[py_str("diet")])) {
    return op_add(&op_add(&op_add(&op_add(&py_str("You said your "), &who.clone()), &py_str(" is ")), &call_builtin("str", &[geti(&entry.clone(), &py_str("diet"))]).unwrap_or_else(|e| exc::raise(e))), &py_str("."));
    }
    if py_truthy(&mcall(&entry.clone(), "get", &[py_str("no_meat")])) {
    return op_add(&op_add(&py_str("You said your "), &who.clone()), &py_str(" doesn't eat meat."));
    }
    if py_truthy(&mcall(&entry.clone(), "get", &[py_str("name")])) {
    name = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(op_add(&mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&word.clone(), &Py::Int(1), &Py::None, &Py::None))); } py_list(__out) })() }]);
    return op_add(&op_add(&op_add(&op_add(&py_str("Your "), &who.clone()), &py_str(" is called ")), &name.clone()), &py_str("."));
    }
    if py_truthy(&mcall(&entry.clone(), "get", &[py_str("condition")])) {
    return op_add(&op_add(&op_add(&op_add(&py_str("You said your "), &who.clone()), &py_str(" has ")), &call_builtin("str", &[geti(&entry.clone(), &py_str("condition"))]).unwrap_or_else(|e| exc::raise(e))), &py_str("."));
    }
    return py_str("");
    Py::None
}
pub fn _about_a_pet(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut known: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut facts: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut animal: Py = Py::None;
    let mut breed: Py = Py::None;
    let mut family: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A pet they ask about by its kind: \"have I told you about the dog?\".");
    if py_truthy(&mcall(&g_NOT_MINE(), "search", &[op_add(&what.clone(), &py_str(" "))])) {
    return Py::None;
    }
    kind = mcall(&g_ANIMAL(), "search", &[what.clone()]);
    family = { let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("family")]); if py_truthy(&__b) { __b } else { { let animal = animal.clone(); let breed = breed.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let known = known.clone(); let memory = memory.clone(); let mine = mine.clone(); let name = name.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); let what = what.clone(); py_func("lambda", move |args: &[Py]| { let animal = animal.clone(); let breed = breed.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let known = known.clone(); let memory = memory.clone(); let mine = mine.clone(); let name = name.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); let what = what.clone(); let word: Py = __kwp(args, 0, "word").unwrap_or(Py::None); word.clone() }) } } };
    if py_truthy(&kind.clone()) {
    name = py_call(&family, &[mcall(&mcall(&mcall(&kind.clone(), "group", &[Py::Int(0)]), "lower", &[]), "rstrip", &[py_str("s")])]);
    } else {
    breed = mcall(&pyrt::xrefs::xget("understanding.breeds", "BREED_RE"), "search", &[what.clone()]);
    name = if py_truthy(&breed.clone()) { mcall(&pyrt::xrefs::xget("understanding.breeds", "BREED_KIND"), "get", &[pyrt::xrefs::xcall("understanding.breeds", "the_breed_named", &[mcall(&breed.clone(), "group", &[Py::Int(0)])]), py_str("")]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pets.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("pet"), py_str("animal")]), &name.clone()))) {
    return _the_pets(&[memory.clone(), pets.clone(), facts.clone()]);
    }
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&py_call(&family, &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")])]), &name.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return _no_note(&[op_add(&py_str("a "), &name.clone()), memory.clone()]);
    }
    said = py_call(&{ let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { { let animal = animal.clone(); let breed = breed.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let known = known.clone(); let memory = memory.clone(); let mine = mine.clone(); let name = name.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); let what = what.clone(); py_func("lambda", move |args: &[Py]| { let animal = animal.clone(); let breed = breed.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let known = known.clone(); let memory = memory.clone(); let mine = mine.clone(); let name = name.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); let what = what.clone(); let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let animal: Py = __kwp(args, 1, "animal").unwrap_or(Py::None); py_str("") }) } } }, &[memory.clone(), name.clone()]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&pet.clone(), "get", &[py_str("name")])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return said.clone();
    }
    pet = geti(&mine.clone(), &op_neg(&Py::Int(1)));
    animal = call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { name.clone() } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&pet.clone(), "get", &[py_str("coming")])) {
    return op_add(&op_add(&py_str("You're getting "), &_many(&[pet.clone()])), &py_str("."));
    }
    return op_add(&op_add(&py_str("You've got "), &_many(&[pet.clone()])), &py_str("."));
    Py::None
}
pub fn _the_pets(args: &[Py]) -> Py {
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
    let mut pets: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut facts: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut coming: Py = Py::None;
    let mut here: Py = Py::None;
    let mut one: Py = Py::None;
    let mut owed: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("All the pets the store holds, in the store's own sentence when they are one kind.");
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("coming")])))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    coming = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&mcall(&pet.clone(), "get", &[py_str("coming")])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    said = _the_pets_here(&[memory.clone(), here.clone(), facts.clone()]);
    owed = _the_pets_here(&[memory.clone(), coming.clone(), facts.clone()]);
    if py_truthy(&owed.clone()) {
    owed = mcall(&owed.clone(), "replace", &[py_str("You've got "), py_str("You're getting "), Py::Int(1)]);
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![said.clone(), owed.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    return { let __b = mcall(&py_str(" "), "join", &[parts.clone()]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _the_pets_here(args: &[Py]) -> Py {
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
    let mut pets: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut facts: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut animal: Py = Py::None;
    let mut family: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut kinds: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    family = { let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("family")]); if py_truthy(&__b) { __b } else { { let animal = animal.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let kinds = kinds.clone(); let memory = memory.clone(); let one = one.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let animal = animal.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let kinds = kinds.clone(); let memory = memory.clone(); let one = one.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); let word: Py = __kwp(args, 0, "word").unwrap_or(Py::None); word.clone() }) } } };
    kinds = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pets.clone());
    for __item in __iter1 {
    pet = __item.clone();
    animal = py_call(&family, &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")])]);
    if py_truthy(&{ let __b = animal.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&kinds.clone(), &animal.clone())) } }) {
    let _ = mcall(&kinds.clone(), "append", &[animal.clone()]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[kinds.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    said = py_call(&{ let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { { let animal = animal.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let kinds = kinds.clone(); let memory = memory.clone(); let one = one.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); py_func("lambda", move |args: &[Py]| { let animal = animal.clone(); let facts = facts.clone(); let family = family.clone(); let kind = kind.clone(); let kinds = kinds.clone(); let memory = memory.clone(); let one = one.clone(); let pet = pet.clone(); let pets = pets.clone(); let said = said.clone(); let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let animal: Py = __kwp(args, 1, "animal").unwrap_or(Py::None); py_str("") }) } } }, &[memory.clone(), geti(&kinds.clone(), &Py::Int(0))]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kinds.clone()) { let mut kind: Py = __item.clone(); __out.push(_many(&[geti(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![kind.clone(), op_add(&kind.clone(), &py_str("s"))]), &mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")])))) {  __out.push(pet.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(1)))])); } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return op_add(&op_add(&py_str("You've got "), &geti(&said.clone(), &Py::Int(0))), &py_str("."));
    }
    return op_add(&op_add(&op_add(&op_add(&py_str("You've got "), &mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)])), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1)))), &py_str("."));
    Py::None
}
pub fn _a(args: &[Py]) -> Py {
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
    return op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &mcall(&slice_or_raise(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(1), &Py::None), "lower", &[])))) { py_str("an ") } else { py_str("a ") }, &call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _many(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut animal: Py = Py::None;
    let mut count: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut many: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("A pet said with its name or its count: \"a cat called Milo\", \"two cats\", \"a dog\".");
    kind = call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }]).unwrap_or_else(|e| exc::raise(e));
    animal = if py_truthy(&mcall(&pet.clone(), "get", &[py_str("breed_of_one")])) { kind.clone() } else { call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("breed")]); if py_truthy(&__b) { __b } else { kind.clone() } }]).unwrap_or_else(|e| exc::raise(e)) };
    many = if py_truthy(&mcall(&kind.clone(), "endswith", &[py_tuple(vec![py_str("fish"), py_str("sheep"), py_str("deer")])])) { kind.clone() } else { op_add(&kind.clone(), &py_str("s")) };
    if py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")])) {
    name = call_builtin("str", &[geti(&pet.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e));
    return op_add(&op_add(&op_add(&_a(&[animal.clone()]), &py_str(" called ")), &mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[])), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    let __try1 = exc::catch(|| {
    count = call_builtin("int", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled5 = true;
    count = Py::Int(1);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&op_gt(&count.clone(), &Py::Int(1))) {
    return op_add(&op_add(&mcall(&g_COUNT_WORDS(), "get", &[count.clone(), call_builtin("str", &[count.clone()]).unwrap_or_else(|e| exc::raise(e))]), &py_str(" ")), &many.clone());
    }
    return _a(&[animal.clone()]);
    Py::None
}
pub fn _the_newest_stated_line(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { py_str("") };
    let mut a_question_ok: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a_question_ok") { v.clone() }
    else { Py::Bool(false) };
    let mut asked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The last line of theirs that states something, or \"\" when the chat is all questions.\n\n    `a_question_ok` is the last resort for a chat that holds nothing but an ask: it is still what\n    they said, and it is said as the question it was rather than denied.\n    ");
    asked = _flat(&[skip.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&_flat(&[said.clone()]), &asked.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&a_question_ok.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_ASKS(), "match", &[said.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&_a_claim_about_the_chat(&[said.clone()])) {
    continue;
    }
    return mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])]);
    }
    return py_str("");
    Py::None
}
pub fn _a_claim_that_points(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The proof for a claim that names the chat rather than a thing (\"i said that already\").");
    line = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_POINTING_SAID(), "match", &[line.clone()])) {
    return _the_proof_they_asked_for(&[py_str(""), text.clone(), memory.clone()]);
    }
    match_ = { let __b = mcall(&g_CLAIMED_ABOUT(), "match", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_CLAIM_WITHOUT_YOU(), "match", &[line.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    said = mcall(&mcall(&g_A_TRAILING_TIME(), "sub", &[py_str(""), mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("what")]), "split", &[])])]), "strip", &[py_str(" .,!")]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINTING(), &mcall(&said.clone(), "lower", &[]))) } }) {
    return Py::None;
    }
    return _the_proof_they_asked_for(&[said.clone(), text.clone(), memory.clone()]);
    Py::None
}
pub fn _the_proof_they_asked_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut asked_line: Py = Py::None;
    let mut quoted: Py = Py::None;
    let mut said: Py = Py::None;
    let mut told_about: Py = Py::None;
    let _ = py_str("The line they are asking for, when the claim points at the chat instead of naming a thing.\n\n    \"i told you already\", \"i mentioned it before\" name no thing at all: they point at the thread\n    and ask whether it was kept. Read as the thing themselves, the store denied a note of the word\n    \"already\" (\"I haven't kept a note of already - tell me and I'll remember it.\") while it held\n    the boiler the whole time. What was told is the newest line of theirs about the thing the chat\n    is on, said back as the proof they are asking for.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = named.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ONLY_A_POINTING_TIME(), "match", &[mcall(&mcall(&g_POINTING_TAIL(), "sub", &[py_str(""), said.clone()]), "strip", &[])]))) } }) {
    return Py::None;
    }
    told_about = _the_thing_the_chat_is_on(&[memory.clone()]);
    quoted = if py_truthy(&told_about.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(told_about.clone()); __kws.extend(vec![("skip".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _line_about(args))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&quoted.clone()))) {
    quoted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("skip".to_string(), text.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_newest_stated_line(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&quoted.clone()))) {
    asked_line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("skip".to_string(), text.clone()), ("a_question_ok".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_newest_stated_line(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&asked_line.clone()))) {
    return Py::None;
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("All I have from you so far is \""), &asked_line.clone()), &py_str("\" — tell me the rest and I'll keep it.")), op_add(&op_add(&py_str("What I have is \""), &asked_line.clone()), &py_str("\". Say it in your own words and it is in."))]), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[py_list(vec![op_add(&op_add(&py_str("You did — \""), &quoted.clone()), &py_str("\" is what I kept.")), op_add(&op_add(&py_str("\""), &quoted.clone()), &py_str("\" — that one I have.")), op_add(&op_add(&py_str("I have it: \""), &quoted.clone()), &py_str("\"."))]), memory.clone()]);
    Py::None
}
pub fn _line_about(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut whole_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "whole_words") { v.clone() }
    else { Py::Bool(false) };
    let mut skip: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { py_str("") };
    let mut asked: Py = Py::None;
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The last line of theirs about the thing, trimmed, or \"\" when they never said one.\n\n    `whole_words` matches the thing as a word (\"son\" is not in \"song\"), which a relation needs:\n    the word is short and sits inside other words all the time.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), mcall(&what.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_str("");
    }
    asked = _flat(&[skip.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&_flat(&[said.clone()]), &asked.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { mcall(&g_ASKS(), "match", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = _a_claim_about_the_chat(&[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_clause_is_named(&[said.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&pyrt::xrefs::xcall("memory.recall", "ask", &[said.clone()]), &py_str("me")))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_TOLD_ABOUT(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CLAIMS_TOLD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_REMEMBERED(), "match", &[said.clone()]) } } } }) {
    continue;
    }
    low = mcall(&said.clone(), "lower", &[]);
    if py_truthy(&whole_words.clone()) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return _trim(&[said.clone()]);
    }
    } else {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return _trim(&[said.clone()]);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_trip_line(args: &[Py]) -> Py {
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
    let _ = py_str("The last line of theirs about going somewhere, trimmed, or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&said.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_TRIP(), "search", &[said.clone()])) {
    return _trim(&[said.clone()]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,!")]);
    Py::None
}
pub fn _the_denial(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut verb: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut pred: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pred") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pred"))) };
    let mut word: Py = Py::None;
    let _ = py_str("\"your mum isn't coeliac\": the fact said the other way round, with the relation named.");
    word = mcall(&py_dict(vec![(py_str("is"), py_str("isn't")), (py_str("was"), py_str("wasn't")), (py_str("are"), py_str("aren't")), (py_str("does"), py_str("doesn't")), (py_str("did"), py_str("didn't")), (py_str("has"), py_str("hasn't"))]), "get", &[verb.clone(), py_str("isn't")]);
    return op_add(&op_add(&op_add(&op_add(&op_add(&py_str("your "), &who.clone()), &py_str(" ")), &word.clone()), &py_str(" ")), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = pred.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]));
    Py::None
}
pub fn _a_pronoun_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut fits: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The pronouns that can point at that person: \"she\" for a mum, and \"they\" for anybody.");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = who.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    fits = py_list(vec![py_str("they")]);
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "SHE"), &word.clone()))) {
    let _ = mcall(&fits.clone(), "append", &[py_str("she")]);
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("memory.facts", "HE"), &word.clone()))) {
    let _ = mcall(&fits.clone(), "append", &[py_str("he")]);
    }
    return stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^\\W*(?:"), &g_CORRECTION_WORDS()), &py_str("\\b[,.!;:]?\\s*){0,2}(?:")), &mcall(&py_str("|"), "join", &[fits.clone()])), &py_str(")(?:'s|s)?\\s+\\S")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(100) };
    let mut cut: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Their line as a quote: one line, short enough to read inside a reply.");
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .,!")]);
    if py_truthy(&op_gt(&call_builtin("len", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
    cut = geti(&mcall(&slice_or_raise(&text.clone(), &Py::None, &limit.clone(), &Py::None), "rsplit", &[py_str(" "), Py::Int(1)]), &Py::Int(0));
    text = op_add(&cut.clone(), &py_str("…"));
    }
    return text.clone();
    Py::None
}
pub fn _stem(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut ending: Py = Py::None;
    let _ = py_str("A word without its endings, so \"savings\", \"saving\" and \"save\" are one thing to match on.\n\n    The endings come off one after another, because English stacks them: \"savings\" is \"save\" with\n    \"ing\" and then \"s\" on it, and one pass left \"saving\" beside \"save\".\n    ");
    word = stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("ings"), py_str("ing"), py_str("ies"), py_str("es"), py_str("ed"), py_str("s"), py_str("e"), py_str("y")]));
    let mut __else3 = false;
    for __item in __iter2 {
    ending = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[ending.clone()]) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    break;
    }
    return word.clone();
    Py::None
}
pub fn _kept_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut into: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "into") { v.clone() }
    else { Py::None };
    let mut item: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Every word of a kept fact, so a plan or a list of allergies can be searched for one.");
    into = if py_truthy(&Py::Bool(py_is(&into.clone(), &Py::None))) { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) } else { into.clone() };
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&into.clone(), "update", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z]+"), mcall(&value.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&mcall(&value.clone(), "values", &[]));
    for __item in __iter1 {
    item = __item.clone();
    let _ = _kept_words(&[item.clone(), into.clone()]);
    }
    } else {
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter2 = py_iter_or_raise(&value.clone());
    for __item in __iter2 {
    item = __item.clone();
    let _ = _kept_words(&[item.clone(), into.clone()]);
    }
    }
    }
    }
    return into.clone();
    Py::None
}
pub fn _the_fact_that_names(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut known: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut facts: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: facts"))) };
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut reader: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stems: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The fact the ask names by one of its words, said the store's way, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&facts.clone()))) {
    return Py::None;
    }
    stems = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-zA-Z]+"), call_builtin("str", &[{ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {  __out.push(_stem(&[word.clone()])); } } py_set(__out) })() };
    let _ = mcall(&stems.clone(), "discard", &[py_str("")]);
    if py_truthy(&Py::Bool(!py_truthy(&stems.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_THE_KINDS_KEPT());
    for __item in __iter1 {
    key = __item.clone();
    reader = mcall(&facts.clone(), "get", &[key.clone()]);
    value = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reader.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&value.clone())) } }) {
    continue;
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_kept_words(&[value.clone()])) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&stems.clone(), &kept.clone())))) {
    continue;
    }
    said = py_call(&reader, &[memory.clone()]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOTHING_SAID(), "match", &[said.clone()]))) } }) {
    return said.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_kind_of(args: &[Py]) -> Py {
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
    let mut candidate: Py = Py::None;
    let mut key: Py = Py::None;
    let _ = py_str("\"plans\" and \"plan\" are the same kind of fact, and so are \"jobs\" and \"job\".");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![word.clone(), slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("es")])) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None) } else { word.clone() }]));
    for __item in __iter1 {
    candidate = __item.clone();
    key = mcall(&g_A_KIND(), "get", &[candidate.clone()]);
    if py_truthy(&key.clone()) {
    return key.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _names_only_a_kind(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut what: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when every word of the thing asked about names a kind of fact (\"my name\", \"the money\").");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), mcall(&call_builtin("str", &[{ let __b = what.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(mcall(&word.clone(), "strip", &[py_str(".,!?")])); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_the_kind_of(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_fact_of_that_kind(args: &[Py]) -> Py {
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
    let mut what: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut known: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut facts: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "facts") { v.clone() }
    else { Py::None };
    let mut key: Py = Py::None;
    let mut p: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut v: Py = Py::None;
    let mut value: Py = Py::None;
    let mut values: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The fact a kind-naming ask asks about: \"have i told you about the trip\" -> the plan.\n\n    The sentence is the store's own (\"You're Sam.\" for the name, \"You told me you're allergic to\n    peanuts.\" for the allergy), handed in as `facts`: a kind of ask should not be a second set of\n    sentences for the same fact, or the two answers drift apart and one of them reads like a form.\n    ");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), mcall(&what.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    key = _the_kind_of(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&key.clone()))) {
    continue;
    }
    value = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut p: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[p.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(p.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    place = __item.clone();
    said = _line_about(&[memory.clone(), place.clone()]);
    if py_truthy(&said.clone()) {
    return _quote(&[said.clone(), memory.clone()]);
    }
    }
    said = _a_trip_line(&[memory.clone()]);
    if py_truthy(&said.clone()) {
    return _quote(&[said.clone(), memory.clone()]);
    }
    }
    return py_str("");
    }
    said = py_call(&{ let __b = mcall(&{ let __b = facts.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { { let facts = facts.clone(); let key = key.clone(); let known = known.clone(); let memory = memory.clone(); let p = p.clone(); let place = place.clone(); let said = said.clone(); let v = v.clone(); let value = value.clone(); let values = values.clone(); let what = what.clone(); let word = word.clone(); py_func("lambda", move |args: &[Py]| { let facts = facts.clone(); let key = key.clone(); let known = known.clone(); let memory = memory.clone(); let p = p.clone(); let place = place.clone(); let said = said.clone(); let v = v.clone(); let value = value.clone(); let values = values.clone(); let what = what.clone(); let word = word.clone(); let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); py_str("") }) } } }, &[memory.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plan")))) {
    return op_add(&op_add(&py_str("You said: \""), &_trim(&[value.clone()])), &py_str("\""));
    }
    values = if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { py_list(vec![value.clone()]) };
    return op_add(&op_add(&py_str("You told me: "), &mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&values.clone()) { let mut v: Py = __item.clone(); __out.push(call_builtin("str", &[v.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), &py_str("."));
    }
    return py_str("");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.told".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_CHAT" => Some(py_func("ABOUT_THE_CHAT", |args: &[Py]| ABOUT_THE_CHAT(args))),
            "ALREADY" => Some(g_ALREADY()),
            "ANIMAL" => Some(g_ANIMAL()),
            "ASKS" => Some(g_ASKS()),
            "A_BARE_CLAIM" => Some(g_A_BARE_CLAIM()),
            "A_BOUNDARY" => Some(g_A_BOUNDARY()),
            "A_CALLBACK" => Some(py_func("A_CALLBACK", |args: &[Py]| A_CALLBACK(args))),
            "A_CLAIM_BEFORE" => Some(g_A_CLAIM_BEFORE()),
            "A_CLAIM_WITHOUT_YOU" => Some(g_A_CLAIM_WITHOUT_YOU()),
            "A_CLAUSE" => Some(g_A_CLAUSE()),
            "A_CLAUSE_OF_MINE" => Some(g_A_CLAUSE_OF_MINE()),
            "A_CLAUSE_OPENING" => Some(g_A_CLAUSE_OPENING()),
            "A_DATE_PROPERTY" => Some(g_A_DATE_PROPERTY()),
            "A_DENIAL" => Some(g_A_DENIAL()),
            "A_KIND" => Some(g_A_KIND()),
            "A_LEADING_TIME" => Some(g_A_LEADING_TIME()),
            "A_NAME" => Some(g_A_NAME()),
            "A_NUMBER" => Some(g_A_NUMBER()),
            "A_PLACE_PROPERTY" => Some(g_A_PLACE_PROPERTY()),
            "A_QUOTE" => Some(g_A_QUOTE()),
            "A_STATEMENT" => Some(g_A_STATEMENT()),
            "A_TIME_AGO" => Some(g_A_TIME_AGO()),
            "A_TIME_ONLY" => Some(g_A_TIME_ONLY()),
            "A_TRAILING_TIME" => Some(g_A_TRAILING_TIME()),
            "A_TRIP" => Some(g_A_TRIP()),
            "A_VERB_IN_A_CLAUSE" => Some(g_A_VERB_IN_A_CLAUSE()),
            "BARE_DENIAL" => Some(g_BARE_DENIAL()),
            "BARE_DENIAL_PRONOUN" => Some(g_BARE_DENIAL_PRONOUN()),
            "CALLBACK_MISSED" => Some(g_CALLBACK_MISSED()),
            "CALLED_NAME" => Some(g_CALLED_NAME()),
            "CHECKED_ELSEWHERE" => Some(g_CHECKED_ELSEWHERE()),
            "CHECK_OF_SOMEONE" => Some(g_CHECK_OF_SOMEONE()),
            "CLAIMED_ABOUT" => Some(g_CLAIMED_ABOUT()),
            "CLAIMS_TOLD" => Some(g_CLAIMS_TOLD()),
            "CLAIM_FILLER" => Some(g_CLAIM_FILLER()),
            "CONTRACTIONS" => Some(g_CONTRACTIONS()),
            "CORRECTING" => Some(g_CORRECTING()),
            "CORRECTION_WORDS" => Some(g_CORRECTION_WORDS()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "IN_QUESTION" => Some(g_IN_QUESTION()),
            "KNOW_ABOUT" => Some(g_KNOW_ABOUT()),
            "LEFT_THERE" => Some(g_LEFT_THERE()),
            "LOWERABLE" => Some(g_LOWERABLE()),
            "MINE" => Some(g_MINE()),
            "NOTHING_NAMED" => Some(g_NOTHING_NAMED()),
            "NOTHING_SAID" => Some(g_NOTHING_SAID()),
            "NOT_MINE" => Some(g_NOT_MINE()),
            "ONLY_A_POINTING_TIME" => Some(g_ONLY_A_POINTING_TIME()),
            "POINTING" => Some(g_POINTING()),
            "POINTING_SAID" => Some(g_POINTING_SAID()),
            "POINTING_TAIL" => Some(g_POINTING_TAIL()),
            "RELATION_WORDS" => Some(g_RELATION_WORDS()),
            "REMEMBERED" => Some(g_REMEMBERED()),
            "REMEMBER_TELLING" => Some(g_REMEMBER_TELLING()),
            "REPORTED" => Some(g_REPORTED()),
            "SAID_AS_IT_IS" => Some(g_SAID_AS_IT_IS()),
            "SAID_OF_PRONOUN" => Some(g_SAID_OF_PRONOUN()),
            "SAID_OF_SOMEONE" => Some(g_SAID_OF_SOMEONE()),
            "SOMETHING_ABOUT" => Some(g_SOMETHING_ABOUT()),
            "TELLING_ALREADY" => Some(g_TELLING_ALREADY()),
            "THE_KINDS_KEPT" => Some(g_THE_KINDS_KEPT()),
            "TOLD_ABOUT" => Some(g_TOLD_ABOUT()),
            "WHERE_I_LIVE" => Some(g_WHERE_I_LIVE()),
            "WITH_AN_ARTICLE" => Some(g_WITH_AN_ARTICLE()),
            "YOURS" => Some(g_YOURS()),
            "_THE_STOP" => Some(g__THE_STOP()),
            "_a" => Some(py_func("_a", |args: &[Py]| _a(args))),
            "_a_boundary_in_a_claim" => Some(py_func("_a_boundary_in_a_claim", |args: &[Py]| _a_boundary_in_a_claim(args))),
            "_a_claim_about_the_chat" => Some(py_func("_a_claim_about_the_chat", |args: &[Py]| _a_claim_about_the_chat(args))),
            "_a_claim_that_points" => Some(py_func("_a_claim_that_points", |args: &[Py]| _a_claim_that_points(args))),
            "_a_clause_is_named" => Some(py_func("_a_clause_is_named", |args: &[Py]| _a_clause_is_named(args))),
            "_a_pronoun_for" => Some(py_func("_a_pronoun_for", |args: &[Py]| _a_pronoun_for(args))),
            "_a_trip_line" => Some(py_func("_a_trip_line", |args: &[Py]| _a_trip_line(args))),
            "_about_a_person" => Some(py_func("_about_a_person", |args: &[Py]| _about_a_person(args))),
            "_about_a_pet" => Some(py_func("_about_a_pet", |args: &[Py]| _about_a_pet(args))),
            "_an_earlier_line" => Some(py_func("_an_earlier_line", |args: &[Py]| _an_earlier_line(args))),
            "_claim_is_a_denial" => Some(py_func("_claim_is_a_denial", |args: &[Py]| _claim_is_a_denial(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_handed_back" => Some(py_func("_handed_back", |args: &[Py]| _handed_back(args))),
            "_kept_words" => Some(py_func("_kept_words", |args: &[Py]| _kept_words(args))),
            "_line_about" => Some(py_func("_line_about", |args: &[Py]| _line_about(args))),
            "_lower_first" => Some(py_func("_lower_first", |args: &[Py]| _lower_first(args))),
            "_many" => Some(py_func("_many", |args: &[Py]| _many(args))),
            "_names_in_capitals" => Some(py_func("_names_in_capitals", |args: &[Py]| _names_in_capitals(args))),
            "_names_only_a_kind" => Some(py_func("_names_only_a_kind", |args: &[Py]| _names_only_a_kind(args))),
            "_no_note" => Some(py_func("_no_note", |args: &[Py]| _no_note(args))),
            "_plain_contractions" => Some(py_func("_plain_contractions", |args: &[Py]| _plain_contractions(args))),
            "_quote" => Some(py_func("_quote", |args: &[Py]| _quote(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_tells_nothing" => Some(py_func("_tells_nothing", |args: &[Py]| _tells_nothing(args))),
            "_the_answer" => Some(py_func("_the_answer", |args: &[Py]| _the_answer(args))),
            "_the_ask" => Some(py_func("_the_ask", |args: &[Py]| _the_ask(args))),
            "_the_denial" => Some(py_func("_the_denial", |args: &[Py]| _the_denial(args))),
            "_the_fact_of_that_kind" => Some(py_func("_the_fact_of_that_kind", |args: &[Py]| _the_fact_of_that_kind(args))),
            "_the_fact_that_names" => Some(py_func("_the_fact_that_names", |args: &[Py]| _the_fact_that_names(args))),
            "_the_kind_of" => Some(py_func("_the_kind_of", |args: &[Py]| _the_kind_of(args))),
            "_the_newest_stated_line" => Some(py_func("_the_newest_stated_line", |args: &[Py]| _the_newest_stated_line(args))),
            "_the_person_a_pronoun_points_at" => Some(py_func("_the_person_a_pronoun_points_at", |args: &[Py]| _the_person_a_pronoun_points_at(args))),
            "_the_pets" => Some(py_func("_the_pets", |args: &[Py]| _the_pets(args))),
            "_the_pets_here" => Some(py_func("_the_pets_here", |args: &[Py]| _the_pets_here(args))),
            "_the_proof_they_asked_for" => Some(py_func("_the_proof_they_asked_for", |args: &[Py]| _the_proof_they_asked_for(args))),
            "_the_property_of_the_thing" => Some(py_func("_the_property_of_the_thing", |args: &[Py]| _the_property_of_the_thing(args))),
            "_the_remembered" => Some(py_func("_the_remembered", |args: &[Py]| _the_remembered(args))),
            "_the_same_thing" => Some(py_func("_the_same_thing", |args: &[Py]| _the_same_thing(args))),
            "_the_shape" => Some(py_func("_the_shape", |args: &[Py]| _the_shape(args))),
            "_the_thing_is_a_clause" => Some(py_func("_the_thing_is_a_clause", |args: &[Py]| _the_thing_is_a_clause(args))),
            "_the_thing_the_chat_is_on" => Some(py_func("_the_thing_the_chat_is_on", |args: &[Py]| _the_thing_the_chat_is_on(args))),
            "_the_verb" => Some(py_func("_the_verb", |args: &[Py]| _the_verb(args))),
            "_the_words_of_the_kind" => Some(py_func("_the_words_of_the_kind", |args: &[Py]| _the_words_of_the_kind(args))),
            "_their_fact" => Some(py_func("_their_fact", |args: &[Py]| _their_fact(args))),
            "_their_lines" => Some(py_func("_their_lines", |args: &[Py]| _their_lines(args))),
            "_trim" => Some(py_func("_trim", |args: &[Py]| _trim(args))),
            "_with_the_yes" => Some(py_func("_with_the_yes", |args: &[Py]| _with_the_yes(args))),
            "_words_of" => Some(py_func("_words_of", |args: &[Py]| _words_of(args))),
            "_your_own_clause" => Some(py_func("_your_own_clause", |args: &[Py]| _your_own_clause(args))),
            "_yours" => Some(py_func("_yours", |args: &[Py]| _yours(args))),
            "a_callback_the_chat_has" => Some(py_func("a_callback_the_chat_has", |args: &[Py]| a_callback_the_chat_has(args))),
            "a_denial" => Some(py_func("a_denial", |args: &[Py]| a_denial(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "at_a_boundary" => Some(py_func("at_a_boundary", |args: &[Py]| at_a_boundary(args))),
            "callback" => Some(py_func("callback", |args: &[Py]| callback(args))),
            "check" => Some(py_func("check", |args: &[Py]| check(args))),
            "is_a_boundary" => Some(py_func("is_a_boundary", |args: &[Py]| is_a_boundary(args))),
            "others" => Some(g_others()),
            "pick" => Some(g_pick()),
            "say_it_back" => Some(g_say_it_back()),
            "user" => Some(g_user()),
            "breed_words" => Some(pyrt::xrefs::xmodule("understanding.breeds")),
            "history" => Some(pyrt::xrefs::xmodule("memory.history")),
            "extract" => Some(pyrt::xrefs::xmodule("memory.facts")),
            "recall" => Some(pyrt::xrefs::xmodule("memory.recall")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ALREADY" => s_ALREADY(v),
            "ANIMAL" => s_ANIMAL(v),
            "ASKS" => s_ASKS(v),
            "A_BARE_CLAIM" => s_A_BARE_CLAIM(v),
            "A_BOUNDARY" => s_A_BOUNDARY(v),
            "A_CLAIM_BEFORE" => s_A_CLAIM_BEFORE(v),
            "A_CLAIM_WITHOUT_YOU" => s_A_CLAIM_WITHOUT_YOU(v),
            "A_CLAUSE" => s_A_CLAUSE(v),
            "A_CLAUSE_OF_MINE" => s_A_CLAUSE_OF_MINE(v),
            "A_CLAUSE_OPENING" => s_A_CLAUSE_OPENING(v),
            "A_DATE_PROPERTY" => s_A_DATE_PROPERTY(v),
            "A_DENIAL" => s_A_DENIAL(v),
            "A_KIND" => s_A_KIND(v),
            "A_LEADING_TIME" => s_A_LEADING_TIME(v),
            "A_NAME" => s_A_NAME(v),
            "A_NUMBER" => s_A_NUMBER(v),
            "A_PLACE_PROPERTY" => s_A_PLACE_PROPERTY(v),
            "A_QUOTE" => s_A_QUOTE(v),
            "A_STATEMENT" => s_A_STATEMENT(v),
            "A_TIME_AGO" => s_A_TIME_AGO(v),
            "A_TIME_ONLY" => s_A_TIME_ONLY(v),
            "A_TRAILING_TIME" => s_A_TRAILING_TIME(v),
            "A_TRIP" => s_A_TRIP(v),
            "A_VERB_IN_A_CLAUSE" => s_A_VERB_IN_A_CLAUSE(v),
            "BARE_DENIAL" => s_BARE_DENIAL(v),
            "BARE_DENIAL_PRONOUN" => s_BARE_DENIAL_PRONOUN(v),
            "CALLBACK_MISSED" => s_CALLBACK_MISSED(v),
            "CALLED_NAME" => s_CALLED_NAME(v),
            "CHECKED_ELSEWHERE" => s_CHECKED_ELSEWHERE(v),
            "CHECK_OF_SOMEONE" => s_CHECK_OF_SOMEONE(v),
            "CLAIMED_ABOUT" => s_CLAIMED_ABOUT(v),
            "CLAIMS_TOLD" => s_CLAIMS_TOLD(v),
            "CLAIM_FILLER" => s_CLAIM_FILLER(v),
            "CONTRACTIONS" => s_CONTRACTIONS(v),
            "CORRECTING" => s_CORRECTING(v),
            "CORRECTION_WORDS" => s_CORRECTION_WORDS(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "IN_QUESTION" => s_IN_QUESTION(v),
            "KNOW_ABOUT" => s_KNOW_ABOUT(v),
            "LEFT_THERE" => s_LEFT_THERE(v),
            "LOWERABLE" => s_LOWERABLE(v),
            "MINE" => s_MINE(v),
            "NOTHING_NAMED" => s_NOTHING_NAMED(v),
            "NOTHING_SAID" => s_NOTHING_SAID(v),
            "NOT_MINE" => s_NOT_MINE(v),
            "ONLY_A_POINTING_TIME" => s_ONLY_A_POINTING_TIME(v),
            "POINTING" => s_POINTING(v),
            "POINTING_SAID" => s_POINTING_SAID(v),
            "POINTING_TAIL" => s_POINTING_TAIL(v),
            "RELATION_WORDS" => s_RELATION_WORDS(v),
            "REMEMBERED" => s_REMEMBERED(v),
            "REMEMBER_TELLING" => s_REMEMBER_TELLING(v),
            "REPORTED" => s_REPORTED(v),
            "SAID_AS_IT_IS" => s_SAID_AS_IT_IS(v),
            "SAID_OF_PRONOUN" => s_SAID_OF_PRONOUN(v),
            "SAID_OF_SOMEONE" => s_SAID_OF_SOMEONE(v),
            "SOMETHING_ABOUT" => s_SOMETHING_ABOUT(v),
            "TELLING_ALREADY" => s_TELLING_ALREADY(v),
            "THE_KINDS_KEPT" => s_THE_KINDS_KEPT(v),
            "TOLD_ABOUT" => s_TOLD_ABOUT(v),
            "WHERE_I_LIVE" => s_WHERE_I_LIVE(v),
            "WITH_AN_ARTICLE" => s_WITH_AN_ARTICLE(v),
            "YOURS" => s_YOURS(v),
            "_THE_STOP" => s__THE_STOP(v),
            "others" => s_others(v),
            "pick" => s_pick(v),
            "say_it_back" => s_say_it_back(v),
            "user" => s_user(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.told", module_obj);
    pyrt::xrefs::reg_fn("memory.told", "_the_shape", py_func("_the_shape", |args: &[Py]| _the_shape(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_verb", py_func("_the_verb", |args: &[Py]| _the_verb(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_same_thing", py_func("_the_same_thing", |args: &[Py]| _the_same_thing(args)));
    pyrt::xrefs::reg_fn("memory.told", "_tells_nothing", py_func("_tells_nothing", |args: &[Py]| _tells_nothing(args)));
    pyrt::xrefs::reg_fn("memory.told", "_handed_back", py_func("_handed_back", |args: &[Py]| _handed_back(args)));
    pyrt::xrefs::reg_fn("memory.told", "is_a_boundary", py_func("is_a_boundary", |args: &[Py]| is_a_boundary(args)));
    pyrt::xrefs::reg_fn("memory.told", "at_a_boundary", py_func("at_a_boundary", |args: &[Py]| at_a_boundary(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a_boundary_in_a_claim", py_func("_a_boundary_in_a_claim", |args: &[Py]| _a_boundary_in_a_claim(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_ask", py_func("_the_ask", |args: &[Py]| _the_ask(args)));
    pyrt::xrefs::reg_fn("memory.told", "ABOUT_THE_CHAT", py_func("ABOUT_THE_CHAT", |args: &[Py]| ABOUT_THE_CHAT(args)));
    pyrt::xrefs::reg_fn("memory.told", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_fn("memory.told", "a_denial", py_func("a_denial", |args: &[Py]| a_denial(args)));
    pyrt::xrefs::reg_fn("memory.told", "_lower_first", py_func("_lower_first", |args: &[Py]| _lower_first(args)));
    pyrt::xrefs::reg_fn("memory.told", "_with_the_yes", py_func("_with_the_yes", |args: &[Py]| _with_the_yes(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_remembered", py_func("_the_remembered", |args: &[Py]| _the_remembered(args)));
    pyrt::xrefs::reg_fn("memory.told", "A_CALLBACK", py_func("A_CALLBACK", |args: &[Py]| A_CALLBACK(args)));
    pyrt::xrefs::reg_fn("memory.told", "a_callback_the_chat_has", py_func("a_callback_the_chat_has", |args: &[Py]| a_callback_the_chat_has(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a_claim_about_the_chat", py_func("_a_claim_about_the_chat", |args: &[Py]| _a_claim_about_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.told", "_an_earlier_line", py_func("_an_earlier_line", |args: &[Py]| _an_earlier_line(args)));
    pyrt::xrefs::reg_fn("memory.told", "callback", py_func("callback", |args: &[Py]| callback(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_answer", py_func("_the_answer", |args: &[Py]| _the_answer(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_property_of_the_thing", py_func("_the_property_of_the_thing", |args: &[Py]| _the_property_of_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a_clause_is_named", py_func("_a_clause_is_named", |args: &[Py]| _a_clause_is_named(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_thing_the_chat_is_on", py_func("_the_thing_the_chat_is_on", |args: &[Py]| _the_thing_the_chat_is_on(args)));
    pyrt::xrefs::reg_fn("memory.told", "_no_note", py_func("_no_note", |args: &[Py]| _no_note(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_thing_is_a_clause", py_func("_the_thing_is_a_clause", |args: &[Py]| _the_thing_is_a_clause(args)));
    pyrt::xrefs::reg_fn("memory.told", "_quote", py_func("_quote", |args: &[Py]| _quote(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_words_of_the_kind", py_func("_the_words_of_the_kind", |args: &[Py]| _the_words_of_the_kind(args)));
    pyrt::xrefs::reg_fn("memory.told", "_yours", py_func("_yours", |args: &[Py]| _yours(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_person_a_pronoun_points_at", py_func("_the_person_a_pronoun_points_at", |args: &[Py]| _the_person_a_pronoun_points_at(args)));
    pyrt::xrefs::reg_fn("memory.told", "_words_of", py_func("_words_of", |args: &[Py]| _words_of(args)));
    pyrt::xrefs::reg_fn("memory.told", "check", py_func("check", |args: &[Py]| check(args)));
    pyrt::xrefs::reg_fn("memory.told", "_your_own_clause", py_func("_your_own_clause", |args: &[Py]| _your_own_clause(args)));
    pyrt::xrefs::reg_fn("memory.told", "_plain_contractions", py_func("_plain_contractions", |args: &[Py]| _plain_contractions(args)));
    pyrt::xrefs::reg_fn("memory.told", "_names_in_capitals", py_func("_names_in_capitals", |args: &[Py]| _names_in_capitals(args)));
    pyrt::xrefs::reg_fn("memory.told", "_claim_is_a_denial", py_func("_claim_is_a_denial", |args: &[Py]| _claim_is_a_denial(args)));
    pyrt::xrefs::reg_fn("memory.told", "_their_lines", py_func("_their_lines", |args: &[Py]| _their_lines(args)));
    pyrt::xrefs::reg_fn("memory.told", "_about_a_person", py_func("_about_a_person", |args: &[Py]| _about_a_person(args)));
    pyrt::xrefs::reg_fn("memory.told", "_their_fact", py_func("_their_fact", |args: &[Py]| _their_fact(args)));
    pyrt::xrefs::reg_fn("memory.told", "_about_a_pet", py_func("_about_a_pet", |args: &[Py]| _about_a_pet(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_pets", py_func("_the_pets", |args: &[Py]| _the_pets(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_pets_here", py_func("_the_pets_here", |args: &[Py]| _the_pets_here(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a", py_func("_a", |args: &[Py]| _a(args)));
    pyrt::xrefs::reg_fn("memory.told", "_many", py_func("_many", |args: &[Py]| _many(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_newest_stated_line", py_func("_the_newest_stated_line", |args: &[Py]| _the_newest_stated_line(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a_claim_that_points", py_func("_a_claim_that_points", |args: &[Py]| _a_claim_that_points(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_proof_they_asked_for", py_func("_the_proof_they_asked_for", |args: &[Py]| _the_proof_they_asked_for(args)));
    pyrt::xrefs::reg_fn("memory.told", "_line_about", py_func("_line_about", |args: &[Py]| _line_about(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a_trip_line", py_func("_a_trip_line", |args: &[Py]| _a_trip_line(args)));
    pyrt::xrefs::reg_fn("memory.told", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_denial", py_func("_the_denial", |args: &[Py]| _the_denial(args)));
    pyrt::xrefs::reg_fn("memory.told", "_a_pronoun_for", py_func("_a_pronoun_for", |args: &[Py]| _a_pronoun_for(args)));
    pyrt::xrefs::reg_fn("memory.told", "_trim", py_func("_trim", |args: &[Py]| _trim(args)));
    pyrt::xrefs::reg_fn("memory.told", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("memory.told", "_kept_words", py_func("_kept_words", |args: &[Py]| _kept_words(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_fact_that_names", py_func("_the_fact_that_names", |args: &[Py]| _the_fact_that_names(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_kind_of", py_func("_the_kind_of", |args: &[Py]| _the_kind_of(args)));
    pyrt::xrefs::reg_fn("memory.told", "_names_only_a_kind", py_func("_names_only_a_kind", |args: &[Py]| _names_only_a_kind(args)));
    pyrt::xrefs::reg_fn("memory.told", "_the_fact_of_that_kind", py_func("_the_fact_of_that_kind", |args: &[Py]| _the_fact_of_that_kind(args)));
    pyrt::xrefs::reg_access("memory.told", "ALREADY", g_ALREADY, s_ALREADY);
    pyrt::xrefs::reg_access("memory.told", "ANIMAL", g_ANIMAL, s_ANIMAL);
    pyrt::xrefs::reg_access("memory.told", "ASKS", g_ASKS, s_ASKS);
    pyrt::xrefs::reg_access("memory.told", "A_BARE_CLAIM", g_A_BARE_CLAIM, s_A_BARE_CLAIM);
    pyrt::xrefs::reg_access("memory.told", "A_BOUNDARY", g_A_BOUNDARY, s_A_BOUNDARY);
    pyrt::xrefs::reg_access("memory.told", "A_CLAIM_BEFORE", g_A_CLAIM_BEFORE, s_A_CLAIM_BEFORE);
    pyrt::xrefs::reg_access("memory.told", "A_CLAIM_WITHOUT_YOU", g_A_CLAIM_WITHOUT_YOU, s_A_CLAIM_WITHOUT_YOU);
    pyrt::xrefs::reg_access("memory.told", "A_CLAUSE", g_A_CLAUSE, s_A_CLAUSE);
    pyrt::xrefs::reg_access("memory.told", "A_CLAUSE_OF_MINE", g_A_CLAUSE_OF_MINE, s_A_CLAUSE_OF_MINE);
    pyrt::xrefs::reg_access("memory.told", "A_CLAUSE_OPENING", g_A_CLAUSE_OPENING, s_A_CLAUSE_OPENING);
    pyrt::xrefs::reg_access("memory.told", "A_DATE_PROPERTY", g_A_DATE_PROPERTY, s_A_DATE_PROPERTY);
    pyrt::xrefs::reg_access("memory.told", "A_DENIAL", g_A_DENIAL, s_A_DENIAL);
    pyrt::xrefs::reg_access("memory.told", "A_KIND", g_A_KIND, s_A_KIND);
    pyrt::xrefs::reg_access("memory.told", "A_LEADING_TIME", g_A_LEADING_TIME, s_A_LEADING_TIME);
    pyrt::xrefs::reg_access("memory.told", "A_NAME", g_A_NAME, s_A_NAME);
    pyrt::xrefs::reg_access("memory.told", "A_NUMBER", g_A_NUMBER, s_A_NUMBER);
    pyrt::xrefs::reg_access("memory.told", "A_PLACE_PROPERTY", g_A_PLACE_PROPERTY, s_A_PLACE_PROPERTY);
    pyrt::xrefs::reg_access("memory.told", "A_QUOTE", g_A_QUOTE, s_A_QUOTE);
    pyrt::xrefs::reg_access("memory.told", "A_STATEMENT", g_A_STATEMENT, s_A_STATEMENT);
    pyrt::xrefs::reg_access("memory.told", "A_TIME_AGO", g_A_TIME_AGO, s_A_TIME_AGO);
    pyrt::xrefs::reg_access("memory.told", "A_TIME_ONLY", g_A_TIME_ONLY, s_A_TIME_ONLY);
    pyrt::xrefs::reg_access("memory.told", "A_TRAILING_TIME", g_A_TRAILING_TIME, s_A_TRAILING_TIME);
    pyrt::xrefs::reg_access("memory.told", "A_TRIP", g_A_TRIP, s_A_TRIP);
    pyrt::xrefs::reg_access("memory.told", "A_VERB_IN_A_CLAUSE", g_A_VERB_IN_A_CLAUSE, s_A_VERB_IN_A_CLAUSE);
    pyrt::xrefs::reg_access("memory.told", "BARE_DENIAL", g_BARE_DENIAL, s_BARE_DENIAL);
    pyrt::xrefs::reg_access("memory.told", "BARE_DENIAL_PRONOUN", g_BARE_DENIAL_PRONOUN, s_BARE_DENIAL_PRONOUN);
    pyrt::xrefs::reg_access("memory.told", "CALLBACK_MISSED", g_CALLBACK_MISSED, s_CALLBACK_MISSED);
    pyrt::xrefs::reg_access("memory.told", "CALLED_NAME", g_CALLED_NAME, s_CALLED_NAME);
    pyrt::xrefs::reg_access("memory.told", "CHECKED_ELSEWHERE", g_CHECKED_ELSEWHERE, s_CHECKED_ELSEWHERE);
    pyrt::xrefs::reg_access("memory.told", "CHECK_OF_SOMEONE", g_CHECK_OF_SOMEONE, s_CHECK_OF_SOMEONE);
    pyrt::xrefs::reg_access("memory.told", "CLAIMED_ABOUT", g_CLAIMED_ABOUT, s_CLAIMED_ABOUT);
    pyrt::xrefs::reg_access("memory.told", "CLAIMS_TOLD", g_CLAIMS_TOLD, s_CLAIMS_TOLD);
    pyrt::xrefs::reg_access("memory.told", "CLAIM_FILLER", g_CLAIM_FILLER, s_CLAIM_FILLER);
    pyrt::xrefs::reg_access("memory.told", "CONTRACTIONS", g_CONTRACTIONS, s_CONTRACTIONS);
    pyrt::xrefs::reg_access("memory.told", "CORRECTING", g_CORRECTING, s_CORRECTING);
    pyrt::xrefs::reg_access("memory.told", "CORRECTION_WORDS", g_CORRECTION_WORDS, s_CORRECTION_WORDS);
    pyrt::xrefs::reg_access("memory.told", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("memory.told", "IN_QUESTION", g_IN_QUESTION, s_IN_QUESTION);
    pyrt::xrefs::reg_access("memory.told", "KNOW_ABOUT", g_KNOW_ABOUT, s_KNOW_ABOUT);
    pyrt::xrefs::reg_access("memory.told", "LEFT_THERE", g_LEFT_THERE, s_LEFT_THERE);
    pyrt::xrefs::reg_access("memory.told", "LOWERABLE", g_LOWERABLE, s_LOWERABLE);
    pyrt::xrefs::reg_access("memory.told", "MINE", g_MINE, s_MINE);
    pyrt::xrefs::reg_access("memory.told", "NOTHING_NAMED", g_NOTHING_NAMED, s_NOTHING_NAMED);
    pyrt::xrefs::reg_access("memory.told", "NOTHING_SAID", g_NOTHING_SAID, s_NOTHING_SAID);
    pyrt::xrefs::reg_access("memory.told", "NOT_MINE", g_NOT_MINE, s_NOT_MINE);
    pyrt::xrefs::reg_access("memory.told", "ONLY_A_POINTING_TIME", g_ONLY_A_POINTING_TIME, s_ONLY_A_POINTING_TIME);
    pyrt::xrefs::reg_access("memory.told", "POINTING", g_POINTING, s_POINTING);
    pyrt::xrefs::reg_access("memory.told", "POINTING_SAID", g_POINTING_SAID, s_POINTING_SAID);
    pyrt::xrefs::reg_access("memory.told", "POINTING_TAIL", g_POINTING_TAIL, s_POINTING_TAIL);
    pyrt::xrefs::reg_access("memory.told", "RELATION_WORDS", g_RELATION_WORDS, s_RELATION_WORDS);
    pyrt::xrefs::reg_access("memory.told", "REMEMBERED", g_REMEMBERED, s_REMEMBERED);
    pyrt::xrefs::reg_access("memory.told", "REMEMBER_TELLING", g_REMEMBER_TELLING, s_REMEMBER_TELLING);
    pyrt::xrefs::reg_access("memory.told", "REPORTED", g_REPORTED, s_REPORTED);
    pyrt::xrefs::reg_access("memory.told", "SAID_AS_IT_IS", g_SAID_AS_IT_IS, s_SAID_AS_IT_IS);
    pyrt::xrefs::reg_access("memory.told", "SAID_OF_PRONOUN", g_SAID_OF_PRONOUN, s_SAID_OF_PRONOUN);
    pyrt::xrefs::reg_access("memory.told", "SAID_OF_SOMEONE", g_SAID_OF_SOMEONE, s_SAID_OF_SOMEONE);
    pyrt::xrefs::reg_access("memory.told", "SOMETHING_ABOUT", g_SOMETHING_ABOUT, s_SOMETHING_ABOUT);
    pyrt::xrefs::reg_access("memory.told", "TELLING_ALREADY", g_TELLING_ALREADY, s_TELLING_ALREADY);
    pyrt::xrefs::reg_access("memory.told", "THE_KINDS_KEPT", g_THE_KINDS_KEPT, s_THE_KINDS_KEPT);
    pyrt::xrefs::reg_access("memory.told", "TOLD_ABOUT", g_TOLD_ABOUT, s_TOLD_ABOUT);
    pyrt::xrefs::reg_access("memory.told", "WHERE_I_LIVE", g_WHERE_I_LIVE, s_WHERE_I_LIVE);
    pyrt::xrefs::reg_access("memory.told", "WITH_AN_ARTICLE", g_WITH_AN_ARTICLE, s_WITH_AN_ARTICLE);
    pyrt::xrefs::reg_access("memory.told", "YOURS", g_YOURS, s_YOURS);
    pyrt::xrefs::reg_access("memory.told", "_THE_STOP", g__THE_STOP, s__THE_STOP);
    pyrt::xrefs::reg_access("memory.told", "others", g_others, s_others);
    pyrt::xrefs::reg_access("memory.told", "pick", g_pick, s_pick);
    pyrt::xrefs::reg_access("memory.told", "say_it_back", g_say_it_back, s_say_it_back);
    pyrt::xrefs::reg_access("memory.told", "user", g_user, s_user);
}

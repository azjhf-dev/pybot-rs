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


thread_local! { static G_ABOUT_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AND_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_A_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COUNT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DANGLING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DURATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GREETING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PART_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUALIFIER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_QUANTITY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REFERRAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SMALL_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_STEP_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_OF_ITS_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TIME_IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COPULA_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTED_DETAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETAIL_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_SENTENCE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FALTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIGURED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREQUENCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEADING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEADING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOIN_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOIN_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS_FOR_A_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEADS_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD_AND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIKE_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMES_NOTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENS_MID_THOUGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLAIN_LEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREPOSITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREPOSITION_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUANTITY_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_BEHAVIOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_NEGATES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_OPENS_ON_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_SENTENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHORT_STEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOWS_FROM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOWS_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_WORDS_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STEP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_OF_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VAGUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.detail: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ABOUT_OPEN() -> Py { ensure_init(); G_ABOUT_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_OPEN(v: Py) { ensure_init(); G_ABOUT_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_AND_A_CLAUSE() -> Py { ensure_init(); G_AND_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AND_A_CLAUSE(v: Py) { ensure_init(); G_AND_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_ME() -> Py { ensure_init(); G_ASKING_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_ME(v: Py) { ensure_init(); G_ASKING_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_A_PART() -> Py { ensure_init(); G_ASKS_A_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_A_PART(v: Py) { ensure_init(); G_ASKS_A_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_ONLY() -> Py { ensure_init(); G_ASK_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_ONLY(v: Py) { ensure_init(); G_ASK_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COUNT_WORD() -> Py { ensure_init(); G_A_COUNT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COUNT_WORD(v: Py) { ensure_init(); G_A_COUNT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DANGLING() -> Py { ensure_init(); G_A_DANGLING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DANGLING(v: Py) { ensure_init(); G_A_DANGLING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DURATION() -> Py { ensure_init(); G_A_DURATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DURATION(v: Py) { ensure_init(); G_A_DURATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GREETING() -> Py { ensure_init(); G_A_GREETING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GREETING(v: Py) { ensure_init(); G_A_GREETING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PART_WORDS() -> Py { ensure_init(); G_A_PART_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PART_WORDS(v: Py) { ensure_init(); G_A_PART_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_BODY() -> Py { ensure_init(); G_A_PERSON_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_BODY(v: Py) { ensure_init(); G_A_PERSON_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUALIFIER() -> Py { ensure_init(); G_A_QUALIFIER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUALIFIER(v: Py) { ensure_init(); G_A_QUALIFIER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_QUANTITY_ASK() -> Py { ensure_init(); G_A_QUANTITY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_QUANTITY_ASK(v: Py) { ensure_init(); G_A_QUANTITY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REFERRAL() -> Py { ensure_init(); G_A_REFERRAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REFERRAL(v: Py) { ensure_init(); G_A_REFERRAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SMALL_WORD() -> Py { ensure_init(); G_A_SMALL_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SMALL_WORD(v: Py) { ensure_init(); G_A_SMALL_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_STEP_ANSWER() -> Py { ensure_init(); G_A_STEP_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_STEP_ANSWER(v: Py) { ensure_init(); G_A_STEP_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_OF_ITS_OWN() -> Py { ensure_init(); G_A_SUBJECT_OF_ITS_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_OF_ITS_OWN(v: Py) { ensure_init(); G_A_SUBJECT_OF_ITS_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TIME_IDIOM() -> Py { ensure_init(); G_A_TIME_IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TIME_IDIOM(v: Py) { ensure_init(); G_A_TIME_IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WHEN() -> Py { ensure_init(); G_A_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WHEN(v: Py) { ensure_init(); G_A_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE() -> Py { ensure_init(); G_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); G_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_WORDS() -> Py { ensure_init(); G_CLAUSE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_WORDS(v: Py) { ensure_init(); G_CLAUSE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COPULA_OPEN() -> Py { ensure_init(); G_COPULA_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COPULA_OPEN(v: Py) { ensure_init(); G_COPULA_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT() -> Py { ensure_init(); G_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT(v: Py) { ensure_init(); G_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTED_DETAIL() -> Py { ensure_init(); G_COUNTED_DETAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTED_DETAIL(v: Py) { ensure_init(); G_COUNTED_DETAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_DETAIL_BODY() -> Py { ensure_init(); G_DETAIL_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETAIL_BODY(v: Py) { ensure_init(); G_DETAIL_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINER() -> Py { ensure_init(); G_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINER(v: Py) { ensure_init(); G_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_SENTENCE_WORDS() -> Py { ensure_init(); G_FACT_SENTENCE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_SENTENCE_WORDS(v: Py) { ensure_init(); G_FACT_SENTENCE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FALTER() -> Py { ensure_init(); G_FALTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FALTER(v: Py) { ensure_init(); G_FALTER.with(|c| *c.borrow_mut() = v); }
pub fn g_FIGURED() -> Py { ensure_init(); G_FIGURED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIGURED(v: Py) { ensure_init(); G_FIGURED.with(|c| *c.borrow_mut() = v); }
pub fn g_FREQUENCY() -> Py { ensure_init(); G_FREQUENCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY(v: Py) { ensure_init(); G_FREQUENCY.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_COUNT() -> Py { ensure_init(); G_HAVE_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_COUNT(v: Py) { ensure_init(); G_HAVE_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_HEADING() -> Py { ensure_init(); G_HEADING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEADING(v: Py) { ensure_init(); G_HEADING.with(|c| *c.borrow_mut() = v); }
pub fn g_HEADING_WORDS() -> Py { ensure_init(); G_HEADING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEADING_WORDS(v: Py) { ensure_init(); G_HEADING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOIN_LEAD() -> Py { ensure_init(); G_JOIN_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOIN_LEAD(v: Py) { ensure_init(); G_JOIN_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_JOIN_OPEN() -> Py { ensure_init(); G_JOIN_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOIN_OPEN(v: Py) { ensure_init(); G_JOIN_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS_FOR() -> Py { ensure_init(); G_LEADS_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS_FOR(v: Py) { ensure_init(); G_LEADS_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS_FOR_A_TIME() -> Py { ensure_init(); G_LEADS_FOR_A_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS_FOR_A_TIME(v: Py) { ensure_init(); G_LEADS_FOR_A_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_LEADS_LEAD() -> Py { ensure_init(); G_LEADS_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEADS_LEAD(v: Py) { ensure_init(); G_LEADS_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD_AND() -> Py { ensure_init(); G_LEAD_AND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD_AND(v: Py) { ensure_init(); G_LEAD_AND.with(|c| *c.borrow_mut() = v); }
pub fn g_LIKE_OPEN() -> Py { ensure_init(); G_LIKE_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIKE_OPEN(v: Py) { ensure_init(); G_LIKE_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURE_BODY() -> Py { ensure_init(); G_MEASURE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURE_BODY(v: Py) { ensure_init(); G_MEASURE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMES_NOTHING() -> Py { ensure_init(); G_NAMES_NOTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMES_NOTHING(v: Py) { ensure_init(); G_NAMES_NOTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_ONLY() -> Py { ensure_init(); G_NAME_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_ONLY(v: Py) { ensure_init(); G_NAME_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_ANSWER() -> Py { ensure_init(); G_NOT_AN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_ANSWER(v: Py) { ensure_init(); G_NOT_AN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORDS() -> Py { ensure_init(); G_NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORDS(v: Py) { ensure_init(); G_NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENS_MID_THOUGHT() -> Py { ensure_init(); G_OPENS_MID_THOUGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENS_MID_THOUGHT(v: Py) { ensure_init(); G_OPENS_MID_THOUGHT.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_ASK() -> Py { ensure_init(); G_PLAIN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_ASK(v: Py) { ensure_init(); G_PLAIN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PLAIN_LEADS() -> Py { ensure_init(); G_PLAIN_LEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLAIN_LEADS(v: Py) { ensure_init(); G_PLAIN_LEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_PREPOSITION() -> Py { ensure_init(); G_PREPOSITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREPOSITION(v: Py) { ensure_init(); G_PREPOSITION.with(|c| *c.borrow_mut() = v); }
pub fn g_PREPOSITION_OPEN() -> Py { ensure_init(); G_PREPOSITION_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREPOSITION_OPEN(v: Py) { ensure_init(); G_PREPOSITION_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN_OPEN() -> Py { ensure_init(); G_PRONOUN_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_OPEN(v: Py) { ensure_init(); G_PRONOUN_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_QUANTITY_BODY() -> Py { ensure_init(); G_QUANTITY_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUANTITY_BODY(v: Py) { ensure_init(); G_QUANTITY_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_ASKS() -> Py { ensure_init(); G_REPORT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_ASKS(v: Py) { ensure_init(); G_REPORT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_BEHAVIOUR() -> Py { ensure_init(); G_REPORT_BEHAVIOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_BEHAVIOUR(v: Py) { ensure_init(); G_REPORT_BEHAVIOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_FILLER() -> Py { ensure_init(); G_REPORT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_FILLER(v: Py) { ensure_init(); G_REPORT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_NEGATES() -> Py { ensure_init(); G_REPORT_NEGATES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_NEGATES(v: Py) { ensure_init(); G_REPORT_NEGATES.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_OPENS_ON_THE_THING() -> Py { ensure_init(); G_REPORT_OPENS_ON_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_OPENS_ON_THE_THING(v: Py) { ensure_init(); G_REPORT_OPENS_ON_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_SENTENCE() -> Py { ensure_init(); G_REPORT_SENTENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_SENTENCE(v: Py) { ensure_init(); G_REPORT_SENTENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORT_WORDS() -> Py { ensure_init(); G_REPORT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORT_WORDS(v: Py) { ensure_init(); G_REPORT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SHORT_STEP() -> Py { ensure_init(); G_SHORT_STEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHORT_STEP(v: Py) { ensure_init(); G_SHORT_STEP.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOWS_FROM() -> Py { ensure_init(); G_SHOWS_FROM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOWS_FROM(v: Py) { ensure_init(); G_SHOWS_FROM.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOWS_OUT() -> Py { ensure_init(); G_SHOWS_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOWS_OUT(v: Py) { ensure_init(); G_SHOWS_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_ONLY() -> Py { ensure_init(); G_SPAN_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_ONLY(v: Py) { ensure_init(); G_SPAN_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_WORDS() -> Py { ensure_init(); G_SPAN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_WORDS(v: Py) { ensure_init(); G_SPAN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_WORDS_ABOUT() -> Py { ensure_init(); G_SPAN_WORDS_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_WORDS_ABOUT(v: Py) { ensure_init(); G_SPAN_WORDS_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE() -> Py { ensure_init(); G_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE(v: Py) { ensure_init(); G_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_ANSWERS() -> Py { ensure_init(); G_STEP_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_ANSWERS(v: Py) { ensure_init(); G_STEP_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_STEP_WORDS() -> Py { ensure_init(); G_STEP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STEP_WORDS(v: Py) { ensure_init(); G_STEP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_PERSON() -> Py { ensure_init(); G_THE_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_PERSON(v: Py) { ensure_init(); G_THE_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_OF_DAY() -> Py { ensure_init(); G_TIME_OF_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_OF_DAY(v: Py) { ensure_init(); G_TIME_OF_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_TIME_WORDS() -> Py { ensure_init(); G_TIME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIME_WORDS(v: Py) { ensure_init(); G_TIME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VAGUE() -> Py { ensure_init(); G_VAGUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VAGUE(v: Py) { ensure_init(); G_VAGUE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A line that adds a detail to the thing the last answer was about.\n\n\"how do i stop condensation on my windows\" gets the paragraph, and the next line is \"its on the\nbedroom ones mainly\". That line is not an ask and not small talk: it says which of the things the\nadvice was about is the one they mean. The store already keeps it - the word \"bedroom\" goes into\n`memory[\"topic\"]` and `memory[\"entry_answer\"]` holds the paragraph - but nothing answered it, so\nthe composer echoed the person's own words back (\"Got it - it's on the bedroom ones mainly\"),\nwhich adds nothing and reads like a form being filled in. A search hedge is the other answer it\ngot (\"what about the bathroom\" after advice about the bathroom's own window).\n\nSo this finds that turn and answers it with the one thing to do out of the answer just given,\nwith the detail named in front: \"On the bedroom ones mainly - use the extractor fan, and open a\nwindow for ten minutes a day.\" That is the step they were already given, so it is honest without\nany new knowledge, and it is about the thing they just narrowed to.\n\nThe rule is narrow on purpose:\n* the line has to open on a back-reference - a copula or a preposition in front of the thing\n  (\"its on the ...\", \"they are in the ...\", \"with two kids\", \"what about the bathroom\") - so a\n  new subject (\"its my birthday\") is not a detail of anything;\n* it has to have no verb of its own: \"we already open a window\" and \"they ate my lettuce\" are\n  reports about the thing, and a report is the composer's to answer;\n* the reply just given has to be a knowledge answer with a step in it, so there is something to\n  hand back;\n* a detail the same shape has already been handed back in the last few turns is not repeated.\n");
    s_JOIN_OPEN(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|oh|also|plus|then|well|yeah|yep|ok|okay|right|alright)\\b[\\s,]+)+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COPULA_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:it'?s|it\\s+is|its|they'?re|they\\s+are|there'?s|there\\s+is|there\\s+are|that'?s|this\\s+is)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:what|how)\\s+about\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:i\\s*m|im|i\\s+am|we\\s*re|were|we\\s+are)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PREPOSITION_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:with|for|on|in|at|by|near|from|like)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIKE_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*like\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TIME_WORDS(call_builtin("frozenset", &[mcall(&py_str("every each once twice thrice all most couple few several\n    time times day days daily night nights nightly morning mornings afternoon afternoon\n    evening evenings week weeks weekly month months monthly year years yearly hour hours\n    minute minutes second seconds yesterday today tomorrow tonight lunch lunchtime teatime\n    bedtime weekend weekends sometimes often always usually rarely never again ago since\n    hourly sort of kind"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERSON_BODY(stdlib_call("re", "compile", &[py_str("^(?:my|our|his|her|their|your|the)\\s+(?:mum|mom|dad|mother|father|sister|brother|son|daughter|wife|husband|partner|boyfriend|girlfriend|friend|mate|colleague|boss|neighbour|neighbor|nan|gran|grandma|grandad|grandpa|aunt|uncle|cousin|kid|kids|child|children|baby|boy|girl|dog|cat|puppy|kitten|parents|family)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AND_A_CLAUSE(stdlib_call("re", "compile", &[py_str("\\band\\s+(?:i|we|he|she|they|it|you|there)\\s+\\w"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DETAIL_BODY(stdlib_call("re", "compile", &[py_str("^(?:on|in|at|near|by|from|for|with|about|the|a|an|this|that|these|those|both|just)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT(py_str("(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|a\\s+few|couple\\s+of|several)"));
    s_QUANTITY_BODY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:"), &g_COUNT()), &py_str(")\\s+(?:of\\s+(?:us|them|you)|days?|weeks?|months?|nights?|hours?|minutes?|people|adults|kids?|children|guests)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURE_BODY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^(?:"), &g_COUNT()), &py_str(")\\s*(?:kg|kgs|kilo|kilos|kilogram|kilograms|g|gram|grams|lb|lbs|pound|pounds|oz|ounce|ounces|stone|stones|litre|litres|liter|liters|ml|cl|cm|mm|metre|metres|meter|meters|inch|inches|feet|foot|ft|miles?|km|degrees?|hours?|hrs?|minutes?|mins?|seconds?|days?|weeks?|months?)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVE_COUNT(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:we|i|weve|ive)\\s+(?:(?:only|just)\\s+)*(?:have|got|ve\\s+got)\\s+(?P<count>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VAGUE(call_builtin("frozenset", &[mcall(&py_str("it its that this these those one ones thing things stuff\n    the a an and but so oh also well yeah yep ok okay right alright\n    really very quite just mainly mostly only also too as well now today later soon again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE(call_builtin("frozenset", &[mcall(&py_str("better worse ok okay fine sore stiff swollen tender aching painful easier\n    harder bigger smaller good bad rubbish awful great healing improving recovered the same\n    bit lot little much slightly more less worse off"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS(Py::Int(9));
    s_STEP_WORDS(Py::Int(14));
    s_SHORT_STEP(Py::Int(8));
    s_JOIN_LEAD(stdlib_call("re", "compile", &[py_str("^(?:so|and|but|then|also|or|plus|because)\\b[\\s,]+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADS_LEAD(py_tuple(vec![py_str("{detail_cap} - same answer: {step}"), py_str("{detail_cap}, then. {step}"), py_str("{detail_cap}: {step}")]));
    s_LEADS_FOR(py_tuple(vec![py_str("Same answer for {thing}: {step}"), py_str("Same for {thing}: {step}"), py_str("For {thing}, the same: {step}")]));
    s_PREPOSITION(stdlib_call("re", "compile", &[py_str("^(?:on|in|at|near|by|from|for|with|about|along|across|round|around|under|down)\\b\\s*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUBJECT_OF_ITS_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|ive|i've|id|i'd|he|hes|he's|she|shes|she's|they|theyre|they're|we|you|it|its|it's)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMES_NOTHING(stdlib_call("re", "compile", &[py_str("^(?:(?:the|a|an|this|that|these|those)\\s+)?(?:end|ends|beginning|start|meantime|meanwhile|moment|whole|same|fact|facts|case|cases|point|first|last|least|most|main|long|short|past|future|addition|total|average|balance|purpose|way|ways|sense|practice|theory|hindsight|conclusion|contrast|order|mind|own|once|now|ages|while|time|times)\\b|(?:my|your|his|her|their|our|its)\\s+(?:case|part|opinion|view|mind|experience|defence|book|own|shoes|honour)\\b|(?:general|particular|large|all|once|now|ages)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WHEN(stdlib_call("re", "compile", &[py_str("^(?:in|on|at|by|before|after|until|till|within|during)\\s+(?:the\\s+|a\\s+|an\\s+)?(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|few|couple|monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekday|weekend|january|february|march|april|may|june|july|august|september|october|november|december|morning|afternoon|evening|night|week|fortnight|month|year|day|hour|minute|tomorrow|tonight|today)\\w*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE(py_str("(?:front|back|rear|bottom|top|inside|outside|underneath|under|side|corner|edge|middle|floor|door|seal|hose|filter|drum|pump|tank|pipe|valve|tap|wheel|tyre|tire|chimney|gutter|roof|celling|wall|window|sill|radiator|boiler|toilet|basin|shower|socket|switch|handle|lid|drawer|join|connection)"));
    s_SHOWS_OUT(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:com(?:e|es|ing)|comin|leak(?:s|ing|ed)?|drip(?:s|ping|ped)?|seep(?:s|ing|ed)?|run(?:s|ning)?|pour(?:s|ing|ed)?|spray(?:s|ing|ed)?|blow(?:s|ing)?)\\s+out(?:\\s+of)?\\s+(?=(?:the\\s+)?"), &g_A_PLACE()), &py_str("\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHOWS_FROM(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:com(?:e|es|ing)|comin|leak(?:s|ing|ed)?|drip(?:s|ping|ped)?|seep(?:s|ing|ed)?|run(?:s|ning)?|pour(?:s|ing|ed)?|spray(?:s|ing|ed)?|blow(?:s|ing)?)\\s+(?=from\\s+(?:the\\s+)?"), &g_A_PLACE()), &py_str("\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FALTER(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:it|its|it'?s|they|they'?re|he|she|that|this|the\\s+\\w+|my\\s+\\w+|our\\s+\\w+)\\s+[a-z']+\\s+(?=(?P<place>(?:at|on|in|along|across|round|around|near|by|under|down)\\s+(?:(?:the|my|its|their|your)\\s+)?"), &g_A_PLACE()), &py_str("\\b))")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DETERMINER(call_builtin("frozenset", &[mcall(&py_str("a an the my our your their his her its some this that these those\n    two three few both"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPAN_ONLY(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:it'?s|it\\s+is|its|it\\s+has|thats|that'?s|that\\s+is|ive|i\\s+have)\\s+(?:been\\s+)?(?P<span>(?:(?:about|around|nearly|almost|over|just\\s+over)\\s+)?(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+few|couple\\s+of|a|several)\\s+(?:day|week|month|year|fortnight|hour|minute)s?(?:\\s+(?:now|or\\s+so|or\\s+more|already))?)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b\\d+(?:[.,]\\d+)?\\s*(?:k|km|kms|kilometres?|kilometers?|miles?|mi|metres?|meters?|m|mins?|minutes?|hours?|hrs?|h|kg|kilos?|lb|lbs|stone|st|litres?|liters?|degrees?|secs?|seconds?|days?|weeks?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PERSON(stdlib_call("re", "compile", &[py_str("\\b(?:i|im|i'm|ive|i've|we|we're|we've|my|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_ME(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:can|should|could|would|do|does|did|is|are|will|shall)\\s+(?:i|we)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_SENTENCE_WORDS(Py::Int(5));
    s_TIME_OF_DAY(call_builtin("frozenset", &[mcall(&py_str("morning mornings afternoon afternoons evening evenings night nights\n    night-time nighttime daytime bedtime lunchtime teatime weekend weekends weekday weekdays\n    monday tuesday wednesday thursday friday saturday sunday"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FREQUENCY(call_builtin("frozenset", &[mcall(&py_str("every each once twice thrice couple few several most usually often\n    sometimes always daily nightly weekly monthly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUALIFIER(call_builtin("frozenset", &[mcall(&py_str("mostly mainly usually generally especially worse worst better\n    particularly right first thing late early"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GREETING(stdlib_call("re", "compile", &[py_str("^(?:good\\s+)?(?:morning|afternoon|evening|night|night night|gnight|nighty night)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DANGLING(call_builtin("frozenset", &[mcall(&py_str("in on at by of to from for with about the a an and or than as"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COUNT_WORD(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven twelve\n    thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty\n    hundred thousand"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TIME_IDIOM(py_tuple(vec![py_str("first thing"), py_str("first thing in the morning"), py_str("middle of the night"), py_str("all day"), py_str("all night"), py_str("an odd night"), py_str("every so often")]));
    s_A_SMALL_WORD(call_builtin("frozenset", &[mcall(&py_str("in on at by of to from and or the a an this that these those\n    thing things time times"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADS_FOR_A_TIME(py_tuple(vec![py_str("{detail_cap}, then. {step}"), py_str("{detail_cap} - the same steps: {step_low}")]));
    s_COUNTED_DETAIL(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&py_str("^(?:"), &g_COUNT()), &py_str(")")), &py_str("\\s+(?:of\\s+(?:us|them|you)|people|adults|kids?|children|guests|portions|slices|days|weeks?|months?|nights?|hours?|minutes?)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURED(stdlib_call("re", "compile", &[py_str("\\d")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_A_PART(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|also|oh|well|yeah|yep|ok|okay|right|alright)\\s+)*(?:what|which|how much|how many|how long|how often|when|where|why|do|does|did|should|shall|can|could|would|will|is|are|was|were|the|that|those|these)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(call_builtin("frozenset", &[mcall(&py_str("what which how much many long often when where why who whom whose do does\n    did should shall can could would will is are was were i me my we our us you your they them\n    their it its that this these those the a an and but so also too oh well yeah yep ok okay\n    right alright now then just really very please\n    need needs needed want wants like get gets got have has had there here one any some\n    again more"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PART_WORDS(Py::Int(26));
    s_PLAIN_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:ok|okay|so|and|but|then|right|yeah|yes|hmm)[\\s,]+)*(?:what|whats|what's|where|wheres|which|how|hows|how's)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_ONLY(call_builtin("frozenset", &[mcall(&py_str("what whats what's where wheres which how hows how's do does did should would could can shall will i we you\nme us it that this these those them they he she one ones the a an my our your their his her its\nbest good right next first thing things bit bits way ways time put use add get buy need want take\nclean start make there in on for to with of about at from into more else other any some all\nnow tonight today tomorrow evening weekend afternoon morning week month please just tell again\nhonestly actually really still also even lets let go on ok okay so right yeah yes hmm then but\nand though well"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_QUANTITY_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:much|many|long|often)|much|many|long|often)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_WHEN(stdlib_call("re", "compile", &[py_str("\\b(tonight|today|tomorrow|now|this evening|this weekend|this afternoon|first|next)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAIN_LEADS(py_tuple(vec![py_str("For that, {step}"), py_str("That's the bit that matters: {step}"), py_str("What I would start with: {step}"), py_str("For that one, {step}")]));
    s_STEP_ANSWERS(py_tuple(vec![py_str("answers"), py_str("memory_store"), py_str("safety")]));
    s_A_STEP_ANSWER(Py::Int(50));
    s_CLAUSE(stdlib_call("re", "compile", &[py_str("[,;:]\\s+|\\s+-\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_ONLY(call_builtin("frozenset", &[mcall(&py_str("a an the my our your their his her its some this that these those and\n    or with without for of in on at to it them they both two three few"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING(call_builtin("frozenset", &[mcall(&py_str("\nthink thinks thinking thought know knows knowing say says saying tell tells telling ask asks\nasking wonder wonders wondering guess guesses guessing consider considers considering decide\n              decides deciding choose chooses choosing help helps helping do does doing work works\nwant wants wanting need needs needing like likes liking get gets getting make makes making\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPAN_WORDS(call_builtin("frozenset", &[mcall(&py_str("day days week weeks month months year years fortnight fortnights\n                          hour hours minute minutes"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPAN_WORDS_ABOUT(call_builtin("frozenset", &[mcall(&py_str("for in after since over within about now already past last next\n                               been it its thats that and but so oh well yeah yep ok okay right\n                               like that"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORDS(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven twelve\n                            couple few several half"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(0), Py::Int(100)]).unwrap_or_else(|e| exc::raise(e))) { let mut digit: Py = __item.clone(); __out.push(call_builtin("str", &[digit.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s_LEAD_AND(stdlib_call("re", "compile", &[py_str("^(?:and|then|but|or|also|so|plus)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEADING(stdlib_call("re", "compile", &[py_str("^(?!(?:the|a|an|my|your|our|their|his|her|it|they)\\b)[^:!?.;]{1,24}:\\s+(?=\\S)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HEADING_WORDS(Py::Int(2));
    s_CLAUSE_WORDS(call_builtin("frozenset", &[mcall(&py_str("is are was were be been am not no yes for and or but so of in on at\n    with to from by as that which who what when how why"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DURATION(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+few|couple\\s+of|several|a)\\s+(?:day|week|month|year|fortnight)s?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_OPENS_ON_THE_THING(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|then|also|now|well|ok|okay|right)\\b[\\s,]*)*(?:it'?s|its|it|they'?re|they|there'?s|there|that'?s|that|this)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_ASKS(stdlib_call("re", "compile", &[py_str("\\?|^\\W*(?:(?:and|but|so|then|also|now|well|ok|okay|right)\\b[\\s,]*)*(?:what|why|how|when|where|who|which|is|are|was|were|do|does|did|can|could|should|would|will|shall|may|might)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_BEHAVIOUR(stdlib_call("re", "compile", &[py_str("\\b(?:if|when|whenever|unless|until|keeps?|kept|usually|often|always|every|tends?|starts?|stops?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_NEGATES(stdlib_call("re", "compile", &[py_str("\\b(?:not|never|nothing|none|without|hardly|isn'?t|isnt|aren'?t|arent|doesn'?t|doesnt|didn'?t|didnt|don'?t|dont|won'?t|wont|can'?t|cant|couldn'?t|couldnt)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REFERRAL(stdlib_call("re", "compile", &[py_str("\\b(?:ring|call|phone|contact|see|visit|go\\s+to|get\\s+to|get\\s+seen)\\b[^.!?]*\\b(?:111|999|911|112|999|nhs|gp|doctor|nurse|a\\s*&\\s*e|casualty|hospital|pharmacy|midwife|health\\s+visitor|emergency)\\b|\\b(?:should|needs?|ought)\\s+(?:to\\s+)?be\\s+(?:looked\\s+at|seen|checked|assessed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORT_WORDS(Py::Int(14));
    s_REPORT_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the and or but if then so to of in on at by for with from about as is are was were be been\nbeing am do does did doing have has had having it its they them their this that these those there\nhere i me my we us our you your he she his her him one ones very really quite just now still\nagain too also only even not no yes yeah yep ok okay well right then than\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_ANSWER(py_tuple(vec![py_str("i don't know enough"), py_str("i dont know enough"), py_str("beyond what i know"), py_str("no live "), py_str("i'm not sure what you mean"), py_str("can't help with that"), py_str("searching for it"), py_str("a search should"), py_str("better sources")]));
    s_OPENS_MID_THOUGHT(stdlib_call("re", "compile", &[py_str("^\\W*(?:and|but|so|then|if|unless|when|whenever|while|because|though|although|which|also|plus|otherwise|either|instead|or|yet|nor|whereas|even so|in that case|as well|a search|searching for it|the search|(?:a|the) quick search)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_verb(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { py_str("") };
    let _ = py_str("True for a word that makes the line a report rather than a detail.\n\n    The verb forms the reading already keeps are reused, so \"ate\", \"open\" and \"tried\" are caught\n    the same way they are in a question. A verb form behind a determiner is the thing's adjective\n    and not a verb, which is how \"a rented flat\" stays a detail and \"they ate my lettuce\" does\n    not.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_DETERMINER(), &before.clone()))) {
    return Py::Bool(false);
    }
    return { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "VERBISH"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "VERB_FORMS"), &word.clone())) } };
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn _the_same_answer_as_the_one_it_narrows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut here: Py = Py::None;
    let mut there: Py = Py::None;
    let _ = py_str("True when the whole reply found for the detail is the answer it narrows, word for word.\n\n    The first 60 characters are enough: an entry is one paragraph, and two entries that open with\n    the same sixty characters are the same paragraph to the person reading it.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![slice_or_raise(&mcall(&_flat(&[step.clone()]), "lower", &[]), &Py::None, &Py::Int(60), &Py::None), slice_or_raise(&mcall(&_flat(&[said.clone()]), "lower", &[]), &Py::None, &Py::Int(60), &Py::None)]), 2, None);
    here = __unpk1[0].clone();
    there = __unpk1[1].clone();
    return { let __b = call_builtin("bool", &[here.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&here.clone(), &there.clone())) } };
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&_flat(&[text.clone()]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_detail(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut body_words: Py = Py::None;
    let mut content: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut counted_rest: Py = Py::None;
    let mut falter: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut like_head: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut span: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The detail as a phrase to put in front of the step (\"on the bedroom ones mainly\"), or None.\n\n    None means the line is not a detail of the thing the answer was about: a question with a\n    thing of its own, a new subject, or a report with a verb in it.\n    ");
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&g_SHOWS_OUT(), "sub", &[py_str("from "), mcall(&g_SHOWS_FROM(), "sub", &[py_str(""), said.clone()])]), "split", &[])]);
    joined = mcall(&mcall(&g_JOIN_OPEN(), "sub", &[py_str(""), said.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = joined.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&joined.clone(), &said.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_PREPOSITION(), "match", &[joined.clone()]) } } } }) {
    said = joined.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_PREPOSITION(), "match", &[said.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _names_nothing(&[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_SUBJECT_OF_ITS_OWN(), "search", &[said.clone()]) } } } }) {
    return Py::None;
    }
    words = mcall(&said.clone(), "split", &[]);
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS())) {
    return Py::None;
    }
    span = mcall(&g_SPAN_ONLY(), "match", &[said.clone()]);
    if py_truthy(&span.clone()) {
    return op_add(&py_str("for "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&span.clone(), "group", &[py_str("span")]), "split", &[])]));
    }
    counted = mcall(&g_HAVE_COUNT(), "match", &[said.clone()]);
    counted_rest = Py::None;
    if py_truthy(&{ let __b = counted.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_QUANTITY_BODY(), "match", &[mcall(&counted.clone(), "group", &[py_str("count")])]) } }) {
    counted_rest = mcall(&mcall(&counted.clone(), "group", &[py_str("count")]), "strip", &[]);
    }
    falter = mcall(&g_FALTER(), "match", &[said.clone()]);
    if py_truthy(&falter.clone()) {
    return mcall(&mcall(&falter.clone(), "group", &[py_str("place")]), "strip", &[]);
    }
    body_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[said.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    like_head = mcall(&g_LIKE_OPEN(), "match", &[said.clone()]);
    if py_truthy(&like_head.clone()) {
    body_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[slice_or_raise(&said.clone(), &mcall(&like_head.clone(), "end", &[]), &Py::None, &Py::None)])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&counted_rest.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[op_add(&py_list(vec![py_str("")]), &body_words.clone()), body_words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8492 = py_unpack(&__item.clone(), 2, None); let mut before: Py = __unpk8492[0].clone(); let mut w: Py = __unpk8492[1].clone(); __out.push(_a_verb(&[w.clone(), before.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    opener = Py::None;
    rest = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_ABOUT_OPEN(), g_LIKE_OPEN(), g_COPULA_OPEN(), g_PRONOUN_OPEN()]));
    for __item in __iter1 {
    opener = __item.clone();
    stripped = mcall(&mcall(&opener.clone(), "sub", &[py_str(""), said.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stripped.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&stripped.clone(), &said.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&opener.clone(), &g_PRONOUN_OPEN())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&opener.clone(), &g_LIKE_OPEN())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_DETAIL_BODY(), "match", &[stripped.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&opener.clone(), &g_COPULA_OPEN())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_QUANTITY_BODY(), "match", &[stripped.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MEASURE_BODY(), "match", &[stripped.clone()]) } } } } } })) } } } }) {
    return Py::None;
    }
    rest = stripped.clone();
    break;
    }
    if py_truthy(&Py::Bool(py_is(&rest.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_is(&counted_rest.clone(), &Py::None))) {
    opener = g_HAVE_COUNT();
    rest = counted_rest.clone();
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PREPOSITION_OPEN(), "match", &[said.clone()])))) {
    return Py::None;
    } else {
    rest = said.clone();
    if py_truthy(&mcall(&g_LIKE_OPEN(), "match", &[rest.clone()])) {
    rest = mcall(&mcall(&g_LIKE_OPEN(), "sub", &[py_str(""), rest.clone()]), "strip", &[py_str(" .,;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    }
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&opener.clone(), &g_PRONOUN_OPEN())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PREPOSITION(), "match", &[rest.clone()]))) } }) {
    return Py::None;
    }
    rest = mcall(&rest.clone(), "strip", &[py_str(" .,!;:")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&rest.clone())); if py_truthy(&__b) { __b } else { mcall(&g_AND_A_CLAUSE(), "search", &[rest.clone()]) } }) {
    return Py::None;
    }
    content = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[mcall(&g_PREPOSITION(), "sub", &[py_str(""), rest.clone()])])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_VAGUE(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&content.clone()))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TIME_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_PERSON_BODY(), "match", &[rest.clone()])) {
    return Py::None;
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STATE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    return rest.clone();
    Py::None
}
pub fn _a_figure(args: &[Py]) -> Py {
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
    let _ = py_str("The figure as it can be compared: the number and the unit without the space (\"5k\").");
    return mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(""), mcall(&word.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "rstrip", &[py_str(".,")]);
    Py::None
}
pub fn the_fact_the_answer_has(args: &[Py]) -> Py {
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
    let mut _asked: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut back: Py = Py::None;
    let mut found: Py = Py::None;
    let mut part: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut there: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The sentence of the answer just given that speaks of a figure the person names, or None.\n\n    \"im 34 and i want to run a 10k\" then \"i can do 5k now\" was answered \"I'm listening — say more\n    if it helps\", although the answer on the screen already says what to do about exactly that:\n    \"If you can run 5k comfortably now, eight to ten weeks is a realistic build.\" Nothing new is\n    known here - the sentence is the answer's own - so it can be handed over honestly, and the\n    whole paragraph is not given again.\n\n    The reading is narrow on purpose: the line has to be the person's own (\"i can\", \"we've got\"),\n    has to name a figure with a unit, and the answer has to have more than one sentence with that\n    same figure in one of them. A bare number (\"im 34\") is an age or a time and nothing here.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return Py::None;
    }
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THE_PERSON(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_ASKING_ME(), "match", &[said.clone()]) } }) {
    return Py::None;
    }
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_FIGURE(), "finditer", &[said.clone()])) { let mut found: Py = __item.clone(); __out.push(_a_figure(&[mcall(&found.clone(), "group", &[Py::Int(0)])])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    reply = py_str("");
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.repeat", "_recent_answers", &[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    _asked = __unpk2[1].clone();
    answer = __unpk2[2].clone();
    if py_truthy(&op_gt(&back.clone(), &Py::Int(1))) {
    break;
    }
    if py_truthy(&answer.clone()) {
    reply = call_builtin("str", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&reply.clone()))) {
    return Py::None;
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&sentences.clone());
    for __item in __iter3 {
    sentence = __item.clone();
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_FACT_SENTENCE_WORDS())) {
    continue;
    }
    there = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_FIGURE(), "finditer", &[sentence.clone()])) { let mut found: Py = __item.clone(); __out.push(_a_figure(&[mcall(&found.clone(), "group", &[Py::Int(0)])])); } py_set(__out) })() };
    if py_truthy(&op_bitand(&there.clone(), &wanted.clone())) {
    said_back = if py_truthy(&mcall(&sentence.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { sentence.clone() } else { op_add(&sentence.clone(), &py_str(".")) };
    return op_add(&mcall(&slice_or_raise(&said_back.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said_back.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    }
    return Py::None;
    Py::None
}
pub fn the_step_for(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut own: Py = Py::None;
    let mut own_read: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut trouble: Py = Py::None;
    let mut worth: Py = Py::None;
    let _ = py_str("The detail, the answer it narrows and the step to hand back, or None.\n\n    The whole of the reading `answer_for` answers from, kept on its own so that another part of\n    the bot can ask whether this line belongs to the answer just given before it answers it\n    itself: a follow-up that the last answer already has a step for is that answer's, and a\n    reading written twice would let the two disagree. Nothing is picked and nothing is written\n    here, so asking costs the conversation nothing.\n\n    The fourth item is true when what came back is a whole reply rather than a step: a thing\n    named beside the chat's subject has an answer of its own in the knowledge base, and that\n    answer beats the answer's step handed over under its name. It is already worded, so it is\n    handed back as it stands.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return Py::None;
    }
    detail = the_detail(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&detail.clone()))) {
    return Py::None;
    }
    trouble = pyrt::xrefs::xmodule("compose.trouble");
    if py_truthy(&mcall(&trouble.clone(), "reads_as_trouble", &[_flat(&[text.clone()])])) {
    return Py::None;
    }
    own = Py::None;
    own_read = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.repeat", "_recent_answers", &[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&op_gt(&back.clone(), &Py::Int(2))) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&back.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_about_the_same_thing", &[asked.clone(), said.clone(), memory.clone()]))) } }) {
    continue;
    }
    plain = _without_branches(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&own_read.clone()))) {
    own = _the_thing_itself(&[detail.clone(), memory.clone()]);
    own_read = Py::Bool(true);
    }
    if py_truthy(&own.clone()) {
    return py_tuple(vec![detail.clone(), said.clone(), own.clone(), Py::Bool(true)]);
    }
    if py_truthy(&_another_thing(&[detail.clone(), said.clone()])) {
    worth = _what_it_is_worth(&[detail.clone(), memory.clone()]);
    if py_truthy(&worth.clone()) {
    return py_tuple(vec![detail.clone(), said.clone(), worth.clone(), Py::Bool(true)]);
    }
    }
    continue;
    }
    step = _step_about(&[plain.clone(), detail.clone()]);
    if py_truthy(&_a_count(&[detail.clone()])) {
    step = { let __b = _the_figure(&[plain.clone()]); if py_truthy(&__b) { __b } else { step.clone() } };
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&own_read.clone()))) {
    own = _the_thing_itself(&[detail.clone(), memory.clone()]);
    own_read = Py::Bool(true);
    }
    if py_truthy(&own.clone()) {
    return py_tuple(vec![detail.clone(), said.clone(), own.clone(), Py::Bool(true)]);
    }
    if py_truthy(&_another_thing(&[detail.clone(), said.clone()])) {
    worth = _what_it_is_worth(&[detail.clone(), memory.clone()]);
    if py_truthy(&worth.clone()) {
    return py_tuple(vec![detail.clone(), said.clone(), worth.clone(), Py::Bool(true)]);
    }
    }
    if py_truthy(&_asks_about_a_new_thing(&[text.clone(), detail.clone(), said.clone()])) {
    continue;
    }
    if py_truthy(&_is_a_span(&[detail.clone()])) {
    step = { let __b = _a_span_step(&[plain.clone(), detail.clone()]); if py_truthy(&__b) { __b } else { { let __b = _first_instruction(&[plain.clone()]); if py_truthy(&__b) { __b } else { _opening_said(&[plain.clone()]) } } } };
    } else {
    step = _a_step(&[plain.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if !py_truthy(&__b) { __b } else { _says_where(&[detail.clone()]) } }) {
    step = { let __b = _first_instruction(&[plain.clone()]); if py_truthy(&__b) { __b } else { _opening_said(&[plain.clone()]) } };
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[step.clone(), memory.clone()])) {
    step = _first_clause(&[step.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[step.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    }
    return py_tuple(vec![detail.clone(), said.clone(), step.clone(), Py::Bool(false)]);
    }
    return Py::None;
    Py::None
}
pub fn the_time_in_the_line(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut body: Py = Py::None;
    let mut content: Py = Py::None;
    let mut idiom: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The time or frequency the line names and no thing, or None.\n\n    \"mostly in the morning\", \"worse at night\", \"its most evenings\", \"like every couple of\n    months\", \"a couple of times a week\": the whole line is when the thing happens. A line that\n    names a thing as well is that thing's question and not this one.\n    ");
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } } } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&g_SHOWS_OUT(), "sub", &[py_str("from "), mcall(&g_SHOWS_FROM(), "sub", &[py_str(""), said.clone()])]), "split", &[])]);
    body = said.clone();
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_ABOUT_OPEN(), g_LIKE_OPEN(), g_COPULA_OPEN(), g_PRONOUN_OPEN()]));
    for __item in __iter1 {
    opener = __item.clone();
    stripped = mcall(&mcall(&opener.clone(), "sub", &[py_str(""), body.clone()]), "strip", &[]);
    if py_truthy(&{ let __b = stripped.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&stripped.clone(), &body.clone())) } }) {
    body = stripped.clone();
    break;
    }
    }
    body = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:but|and|so|oh|also|well|yeah|yep|ok|okay|right|alright)[\\s,]+")); __args.push(py_str("")); __args.push(body.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", &[]);
    words = _words(&[body.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[op_add(&py_list(vec![py_str("")]), &words.clone()), words.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk8807 = py_unpack(&__item.clone(), 2, None); let mut before: Py = __unpk8807[0].clone(); let mut word: Py = __unpk8807[1].clone(); __out.push(_a_verb(&[word.clone(), before.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_GREETING(), "match", &[mcall(&body.clone(), "strip", &[py_str(" .,!;:")])])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_DANGLING(), &geti(&words.clone(), &op_neg(&Py::Int(1)))))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_A_COUNT_WORD(), &word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "isdigit", &[]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&g_A_TIME_IDIOM());
    for __item in __iter2 {
    idiom = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&body.clone(), "strip", &[]), "lower", &[]), &idiom.clone()))) {
    return mcall(&body.clone(), "strip", &[py_str(" .,!;:")]);
    }
    }
    content = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_VAGUE(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_STATE(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_A_QUALIFIER(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_A_SMALL_WORD(), &word.clone())) } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&content.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_TIME_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&content.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_TIME_OF_DAY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FREQUENCY(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    return mcall(&body.clone(), "strip", &[py_str(" .,!;:")]);
    Py::None
}
pub fn _a_time_step(args: &[Py]) -> Py {
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
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut here: Py = Py::None;
    let mut named: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence of the answer that speaks of the same time, or None.\n\n    It is the better reply when there is one (\"a cough that is worse at night ... that wakes you\n    at night\"), because it answers the time with something about the time rather than with the\n    step that would have been given anyway.\n    ");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[detail.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_TIME_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    here = _words(&[sentence.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&here.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { op_ge(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } }) {
    return sentence.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_answer_before(args: &[Py]) -> Py {
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
    let mut here: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the bot said to the turn before this one, or \"\" when it has said nothing yet.\n\n    The line being answered is skipped by its own words, because the store writes the turn down\n    as it goes and a reader asking about the chat runs while that turn is in progress. A turn with\n    no line of the person's in it is not an exchange, so it is skipped too.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    here = mcall(&mcall(&_flat(&[text.clone()]), "lower", &[]), "rstrip", &[py_str(".!?, ")]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&mcall(&_flat(&[mcall(&turn.clone(), "get", &[py_str("user")])]), "lower", &[]), "rstrip", &[py_str(".!?, ")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = here.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &here.clone())) } } } }) {
    continue;
    }
    reply = _flat(&[mcall(&turn.clone(), "get", &[py_str("bot")])]);
    if py_truthy(&reply.clone()) {
    return reply.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn the_step_for_a_time(args: &[Py]) -> Py {
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
    let mut detail: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut trouble: Py = Py::None;
    let _ = py_str("The time named, the answer it belongs to and the step to hand back, or None.\n\n    Only the answer one turn back counts, for the same reason as `the_step_for`: an older answer\n    is a different thing they may have moved on from. An answer with nothing to do in it hands\n    nothing back, and the line is left to the composer rather than answered with the paragraph\n    it already had.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return Py::None;
    }
    detail = the_time_in_the_line(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&detail.clone()))) {
    return Py::None;
    }
    trouble = pyrt::xrefs::xmodule("compose.trouble");
    if py_truthy(&mcall(&trouble.clone(), "reads_as_trouble", &[_flat(&[text.clone()])])) {
    return Py::None;
    }
    said = _the_answer_before(&[memory.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&pyrt::xrefs::xget("memory.repeat", "NEXT_STEP_EMERGENCY"), "search", &[said.clone()])) {
    return Py::None;
    }
    plain = { let __b = _without_branches(&[said.clone()]); if py_truthy(&__b) { __b } else { _flat(&[said.clone()]) } };
    step = { let __b = _a_time_step(&[plain.clone(), detail.clone()]); if py_truthy(&__b) { __b } else { { let __b = _first_instruction(&[plain.clone()]); if py_truthy(&__b) { __b } else { _a_step(&[plain.clone()]) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[step.clone(), memory.clone()])) {
    step = _first_clause(&[step.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[step.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    }
    return py_tuple(vec![detail.clone(), said.clone(), step.clone(), Py::Bool(false)]);
    Py::None
}
pub fn _a_count(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let _ = py_str("Whether the detail is a number of people or portions, not a part of the thing.");
    return call_builtin("bool", &[mcall(&g_COUNTED_DETAIL(), "match", &[_flat(&[detail.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_figure(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let _ = py_str("The sentence of the answer that carries the figure, or None.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&{ let __b = sentence.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_FIGURED(), "search", &[sentence.clone()]) } }) {
    return sentence.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_carrying_words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The words of the line that name something, with a plural ending taken off.\n\n    A line made only of short words (\"what should i put out\") names no part of the answer: \"put\"\n    and \"out\" are how a thing is done, and the clause they land on is about one item of a list\n    rather than the part asked about - the bird answer came back \"Put out water too\", which\n    answers a question about water and not the one asked. So a word of four letters is needed\n    before any of this counts, and such a line is left to answer for itself.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASKING(), &word.clone()))) {  __out.push(op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASKING(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } } py_list(__out) })() };
    Py::None
}
pub fn _the_clause_that_holds(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut words: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut best: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut held: Py = Py::None;
    let mut here: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The sentence, or the clause of it, that holds the words asked about, or \"\".");
    wanted = call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[sentence.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&wanted.clone(), &here.clone())))) {
    continue;
    }
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_clauses(&[sentence.clone()])) { let mut piece: Py = __item.clone(); if py_truthy(&mcall(&piece.clone(), "strip", &[py_str(" ,;:-")])) {  __out.push(mcall(&piece.clone(), "strip", &[py_str(" ,;:-")])); } } py_list(__out) })() };
    best = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clauses.clone()); __kws.extend(vec![("key".to_string(), { let best = best.clone(); let clauses = clauses.clone(); let held = held.clone(); let here = here.clone(); let piece = piece.clone(); let said = said.clone(); let sentence = sentence.clone(); let wanted = wanted.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let best = best.clone(); let clauses = clauses.clone(); let held = held.clone(); let here = here.clone(); let piece = piece.clone(); let said = said.clone(); let sentence = sentence.clone(); let wanted = wanted.clone(); let word = word.clone(); let words = words.clone(); let piece: Py = __kwp(args, 0, "piece").unwrap_or(Py::None); call_builtin("len", &[op_bitand(&wanted.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[piece.clone()])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() })]).unwrap_or_else(|e| exc::raise(e)) }) }), ("default".to_string(), py_str(""))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    held = op_bitand(&wanted.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[best.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() });
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&held.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&best.clone())) } }) {
    continue;
    }
    best = mcall(&mcall(&g_JOIN_LEAD(), "sub", &[py_str(""), best.clone()]), "strip", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&best.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    best = mcall(&mcall(&g_JOIN_LEAD(), "sub", &[py_str(""), _flat(&[sentence.clone()])]), "strip", &[py_str(" ,;:-")]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&best.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_PART_WORDS()) } }) {
    continue;
    }
    return op_add(&mcall(&slice_or_raise(&best.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&best.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    Py::None
}
pub fn the_part_of_the_answer(args: &[Py]) -> Py {
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
    let mut _asked: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut back: Py = Py::None;
    let mut carrying: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_before: Py = Py::None;
    let mut there: Py = Py::None;
    let mut trouble: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The reply when the line asks about a part of the answer one turn back, or \"\".\n\n    The line has to ask, has to name no thing of its own - every word of it that carries meaning\n    is a word of that answer - and the answer has to hold a sentence or a clause with those words\n    in it. Anything else is a new question, and this reader passes.\n    ");
    said = _flat(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKS_A_PART(), "match", &[said.clone()]))) } } } }) {
    return py_str("");
    }
    carrying = _the_carrying_words(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&carrying.clone()))) {
    return py_str("");
    }
    trouble = pyrt::xrefs::xmodule("compose.trouble");
    if py_truthy(&mcall(&trouble.clone(), "reads_as_trouble", &[said.clone()])) {
    return py_str("");
    }
    answer = py_str("");
    let __iter1 = py_iter_or_raise(&pyrt::xrefs::xcall("memory.repeat", "_recent_answers", &[memory.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    back = __unpk2[0].clone();
    _asked = __unpk2[1].clone();
    said_before = __unpk2[2].clone();
    if py_truthy(&op_gt(&back.clone(), &Py::Int(1))) {
    break;
    }
    if py_truthy(&said_before.clone()) {
    answer = call_builtin("str", &[said_before.clone()]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return py_str("");
    }
    there = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[answer.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&carrying.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(!py_contains(&there.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    clause = _the_clause_that_holds(&[carrying.clone(), answer.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return py_str("");
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[clause.clone(), memory.clone()])) {
    return py_str("");
    }
    return if py_truthy(&mcall(&clause.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { clause.clone() } else { op_add(&clause.clone(), &py_str(".")) };
    Py::None
}
pub fn answers_the_line(args: &[Py]) -> Py {
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
    let _ = py_str("True when `answer_for` will answer this line: the answer just given has its step.");
    return { let __b = Py::Bool(!py_is(&the_step_for(&[text.clone(), memory.clone()]), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_is(&the_step_for_a_time(&[text.clone(), memory.clone()]), &Py::None)) } };
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut a_part: Py = Py::None;
    let mut a_time: Py = Py::None;
    let mut detail: Py = Py::None;
    let mut found: Py = Py::None;
    let mut framed: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The reply for a detail added to the answer just given, or None.\n\n    Only the answer one turn back counts: an older answer is a different thing they may have\n    moved on from, which is `repeat`'s call, not this one's.\n    ");
    found = the_step_for(&[text.clone(), memory.clone()]);
    a_time = Py::Bool(false);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    found = the_step_for_a_time(&[text.clone(), memory.clone()]);
    a_time = Py::Bool(!py_is(&found.clone(), &Py::None));
    }
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    a_part = the_part_of_the_answer(&[text.clone(), memory.clone()]);
    if py_truthy(&a_part.clone()) {
    return pyrt::xrefs::xcall("memory.repeat", "_handed_back", &[memory.clone(), a_part.clone()]);
    }
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 4, None);
    detail = __unpk1[0].clone();
    said = __unpk1[1].clone();
    step = __unpk1[2].clone();
    whole = __unpk1[3].clone();
    if py_truthy(&{ let __b = whole.clone(); if !py_truthy(&__b) { __b } else { _the_same_answer_as_the_one_it_narrows(&[step.clone(), said.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&whole.clone()) {
    framed = pyrt::xrefs::xcall("memory.again", "a_thing_of_the_answer", &[step.clone(), text.clone(), memory.clone()]);
    let _ = pyrt::xrefs::xcall("memory.repeat", "_handed_back", &[memory.clone(), step.clone()]);
    return { let __b = framed.clone(); if py_truthy(&__b) { __b } else { step.clone() } };
    }
    thing = { let __b = mcall(&mcall(&g_PREPOSITION(), "sub", &[py_str(""), detail.clone()]), "strip", &[]); if py_truthy(&__b) { __b } else { detail.clone() } };
    if py_truthy(&a_time.clone()) {
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("detail_cap".to_string(), _capital(&[detail.clone()])), ("thing".to_string(), thing.clone()), ("step".to_string(), _capital(&[step.clone()])), ("step_low".to_string(), op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&pyrt::xrefs::xget("memory.repeat", "pick"), &[g_LEADS_FOR_A_TIME(), memory.clone(), text.clone()]), "format", args))(&__args) };
    return pyrt::xrefs::xcall("memory.repeat", "_handed_back", &[memory.clone(), reply.clone()]);
    }
    leads = if py_truthy(&Py::Bool(!py_eq(&thing.clone(), &detail.clone()))) { g_LEADS_LEAD() } else { g_LEADS_FOR() };
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("detail_cap".to_string(), _capital(&[detail.clone()])), ("thing".to_string(), thing.clone()), ("step".to_string(), _capital(&[step.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&pyrt::xrefs::xget("memory.repeat", "pick"), &[leads.clone(), memory.clone(), text.clone()]), "format", args))(&__args) };
    return pyrt::xrefs::xcall("memory.repeat", "_handed_back", &[memory.clone(), reply.clone()]);
    Py::None
}
pub fn _the_answer_just_given(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The bot's last reply when it was an answer with a step in it, or \"\".");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_contains(&g_STEP_ANSWERS(), &mcall(&last.clone(), "get", &[py_str("module")])))) {
    return py_str("");
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_STEP_ANSWER()); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn _bare_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let _ = py_str("The step with its capital taken off where it carries on a sentence: \"Start with ...\" ->\n    \"start with ...\". An opening that is all capitals is a name and stays (\"CO2\", \"RG\"), so it is\n    only ever the first letter and only when the first two are not both capitals.\n\n    Every lead here continues into the step - \"For that one, start with a plunger\" - and a step\n    left with its capital in it reads as a new sentence inside the sentence: \"For that one, Start\n    with a plunger over the plughole with a little water in the bowl.\"\n    ");
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[step.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(2), &Py::None), &mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(2), &Py::None), "upper", &[]))) } }) {
    return step.clone();
    }
    return op_add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn the_plain_ask(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut step: Py = Py::None;
    let mut when: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The step of the answer just given, when the line asks about it and names no thing, or None.\n\n    Returned framed rather than bare, so it reads as the answer it is and not as an answer given\n    twice: \"For that, run a hot 60C or 90C service wash with a washing-machine cleaner.\"\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return Py::None;
    }
    said = _flat(&[text.clone()]);
    words = mcall(&mcall(&said.clone(), "lower", &[]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PLAIN_ASK(), "match", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_QUANTITY_ASK(), "search", &[said.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&g_ASK_ONLY(), &mcall(&word.clone(), "strip", &[py_str("?,.!;:")])))) {
    return Py::None;
    }
    }
    answer = _the_answer_just_given(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&answer.clone()))) {
    return Py::None;
    }
    step = { let __b = _a_step(&[answer.clone()]); if py_truthy(&__b) { __b } else { _first_instruction(&[answer.clone()]) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[mcall(&step.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_handed_back_already", &[step.clone(), memory.clone()])) {
    return Py::None;
    }
    when = mcall(&g_A_WHEN(), "search", &[said.clone()]);
    if py_truthy(&when.clone()) {
    reply = op_add(&op_add(&_capital(&[mcall(&when.clone(), "group", &[Py::Int(1)])]), &py_str(" - ")), &_bare_step(&[step.clone()]));
    } else {
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("step".to_string(), _bare_step(&[step.clone()]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_call(&pyrt::xrefs::xget("memory.repeat", "pick"), &[g_PLAIN_LEADS(), memory.clone(), text.clone()]), "format", args))(&__args) };
    }
    return pyrt::xrefs::xcall("memory.repeat", "_handed_back", &[memory.clone(), reply.clone()]);
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
    let mut move_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "move") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: move"))) };
    let mut chunk: Py = Py::None;
    let mut depth: Py = Py::None;
    let mut out: Py = Py::None;
    let mut piece: Py = Py::None;
    let _ = py_str("The move cut where a clause ends, ignoring a comma inside brackets.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![]), Py::Int(0)]), 3, None);
    out = __unpk1[0].clone();
    piece = __unpk1[1].clone();
    depth = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_CLAUSE(), "split", &[move_.clone()]));
    for __item in __iter2 {
    chunk = __item.clone();
    let __aug3 = op_add(&depth.clone(), &op_sub(&mcall(&chunk.clone(), "count", &[py_str("(")]), &mcall(&chunk.clone(), "count", &[py_str(")")])));
    depth = __aug3;
    if py_truthy(&{ let __b = piece.clone(); if !py_truthy(&__b) { __b } else { depth.clone() } }) {
    let _ = mcall(&piece.clone(), "append", &[chunk.clone()]);
    continue;
    }
    if py_truthy(&piece.clone()) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(", "), "join", &[piece.clone()])]);
    piece = py_list(vec![]);
    }
    piece = py_list(vec![chunk.clone()]);
    }
    if py_truthy(&piece.clone()) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(", "), "join", &[piece.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _first_clause(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut step: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut p: Py = Py::None;
    let mut pieces: Py = Py::None;
    let _ = py_str("The shortest form of a step: its first clause, with a full stop.");
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_clauses(&[mcall(&step.clone(), "rstrip", &[py_str(".")])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[py_str(" ,;:-.")])) {  __out.push(mcall(&p.clone(), "strip", &[py_str(" ,;:-.")])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pieces.clone()))) {
    return py_str("");
    }
    return op_add(&geti(&pieces.clone(), &Py::Int(0)), &py_str("."));
    Py::None
}
pub fn _one_of_a_list(args: &[Py]) -> Py {
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
    let mut groups: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "groups") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: groups"))) };
    let mut at: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "at") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: at"))) };
    let _ = py_str("Whether a clause is one item of a list of things, or a thing a dash-group qualifies.\n\n    The tent answer lists the kit (\"Tent with its pegs and a mallet, a sleeping bag and a mat -\n    a foam or inflatable one\"), so the clause that names the sleeping bag is one item of a list\n    and the qualifier after the dash is what to look for. The condensation answer names the\n    bedroom in a sentence of its own (\"A dehumidifier helps in a bedroom that gets it badly\"),\n    which says what helps, not what to do first - and a detail gets the move, not a description.\n    ");
    return { let __b = op_gt(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[groups.clone()]).unwrap_or_else(|e| exc::raise(e))) } };
    Py::None
}
pub fn _head_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word the detail names its thing with: \"a helmet\" -> \"helmet\", \"in a rented flat\" ->\n    \"flat\" (the last word that says something, since English puts the thing last).");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[mcall(&g_PREPOSITION(), "sub", &[py_str(""), detail.clone()])])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_VAGUE(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    return if py_truthy(&words.clone()) { geti(&words.clone(), &op_neg(&Py::Int(1))) } else { py_str("") };
    Py::None
}
pub fn _subject_words(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut both: Py = Py::None;
    let mut out: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut source: Py = Py::None;
    let mut t: Py = Py::None;
    let mut topics: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words the chat's thing can be named by, best first.\n\n    The subject the store keeps is the best of them (\"used bike\" -> bike), but it is not always a\n    thing: \"what should i look for in a used bike\" files the subject as \"look\", so the words of\n    the last ask and of the answer it got are read too, and a word in both of those is the thing\n    the chat is about before any other.\n    ");
    topics = pyrt::xrefs::xmodule("memory.topics");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    asked = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    said = if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![{ let __b = mcall(&topics.clone(), "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }, asked.clone()]));
    for __item in __iter1 {
    source = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&source.clone(), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    word = stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&word.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_VAGUE(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&gattr(&topics.clone(), "STOPWORDS"), &word.clone())) } } } }) {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    }
    let __unpk3 = py_unpack(&py_tuple(vec![call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)), py_list(vec![]), py_list(vec![])]), 3, None);
    seen = __unpk3[0].clone();
    both = __unpk3[1].clone();
    rest = __unpk3[2].clone();
    let __iter4 = py_iter_or_raise(&out.clone());
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&seen.clone(), &word.clone()))) {
    continue;
    }
    let _ = mcall(&seen.clone(), "add", &[word.clone()]);
    let _ = mcall(&if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { both.clone() } else { rest.clone() }, "append", &[word.clone()]);
    }
    return slice_or_raise(&op_add(&both.clone(), &rest.clone()), &Py::None, &Py::Int(6), &Py::None);
    Py::None
}
pub fn _the_entry_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("The text of an entry, or \"\" when there is none to say (a callable is not read here).");
    reply = if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) { call_builtin("getattr", &[found.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    return if py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reply.clone() } else { py_str("") };
    Py::None
}
pub fn _another_thing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut have: Py = Py::None;
    let mut want: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the detail names something the answer in hand never speaks of.\n\n    The answer given is asked as well as the thing named, because a detail about a part of the\n    answer can be said in another way (\"the bedroom ones\" after a paragraph about a cold bedroom)\n    and that is still the same answer's detail. What is left is a thing of its own.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_head_word(&[detail.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &_head_word(&[detail.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_PREPOSITION(), "match", &[_flat(&[detail.clone()])])) {
    return Py::Bool(false);
    }
    want = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[detail.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_VAGUE(), &word.clone()))) {  __out.push({ let __b = mcall(&word.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { word.clone() } }); } } py_set(__out) })() };
    have = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[said.clone()])) { let mut word: Py = __item.clone(); __out.push({ let __b = mcall(&word.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { word.clone() } }); } py_set(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&want.clone())); if py_truthy(&__b) { __b } else { op_bitand(&want.clone(), &have.clone()) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _what_it_is_worth(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answers: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut lookup_mod: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut understand: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("\"What about a dehumidifier\" is the question \"is one worth it\", when the base keeps a card so.\n\n    A thing named on its own is asked about the way people ask about a thing they might buy or\n    take up, and the base keeps those cards under that ask (\"should i get a dehumidifier\" reaches\n    the appliance card, \"a dehumidifier\" on its own reaches nothing). Only an answer that opens by\n    naming the thing is taken, so a card about something else cannot be handed over this way.\n    ");
    head = _head_word(&[detail.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    lookup_mod = pyrt::xrefs::xmodule("knowledge.lookup");
    answers = pyrt::xrefs::xmodule("modules.answers");
    understood = { let __b = py_call(&understand, &[op_add(&py_str("should i get "), &detail.clone()), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    found = mcall(&lookup_mod.clone(), "find", &[understood.clone()]);
    reply = mcall(&answers.clone(), "_payload", &[found.clone(), memory.clone(), understood.clone()]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&_says_about(&[reply.clone(), head.clone()])))) {
    return Py::None;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[reply.clone(), memory.clone()])) {
    return Py::None;
    }
    return reply.clone();
    Py::None
}
pub fn _says_about(args: &[Py]) -> Py {
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
    let mut head: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut first: Py = Py::None;
    let _ = py_str("True when an entry's answer is about the thing itself, and says so as it opens.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40)) } }) {
    return Py::Bool(false);
    }
    first = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&reply.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    return Py::Bool(py_contains(&mcall(&first.clone(), "lower", &[]), &head.clone()));
    Py::None
}
pub fn _the_thing_itself(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut first: Py = Py::None;
    let mut head: Py = Py::None;
    let mut lookup_mod: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut understand: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The knowledge base's answer for the thing the detail names, asked beside the subject.\n\n    A follow-up can name a thing of its own rather than a part of the answer, and the answer's\n    step is then not its answer: \"ive got about 400 pounds for a used bike\" / \"what should i look\n    for\" / \"and what about a helmet\" was replied \"Same answer for a helmet: Check the frame for\n    cracks and dents around the welds\", which is the answer to a different purchase under the\n    helmet's name. A friend answers about the helmet. The knowledge base keeps an entry for the\n    two things together (\"bike helmet\"), so the thing is asked about with a word of the subject\n    joined to it, and only an answer that names the thing itself is taken: \"puppy biting\" reaches\n    the chewing entry, which never says \"biting\", and that is the answer already on the screen.\n    ");
    head = _head_word(&[detail.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_THING(), &head.clone())) } }) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    lookup_mod = pyrt::xrefs::xmodule("knowledge.lookup");
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&_subject_words(&[memory.clone()]));
    for __item in __iter6 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &head.clone()))) {
    continue;
    }
    let __try7 = exc::catch(|| {
    understood = { let __b = py_call(&understand, &[op_add(&op_add(&word.clone(), &py_str(" ")), &head.clone()), py_dict(vec![])]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    reply = _the_entry_reply(&[mcall(&lookup_mod.clone(), "find", &[understood.clone()])]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret8: Py = Py::Bool(false);
    let mut __val9: Py = Py::None;
    let mut __unhandled10: Option<exc::PyExc> = None;
    match __try7 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret8 = __p[0].clone(); __val9 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled11 = false;
            if !__handled11 && exc::matches_any(&__e, &["Exception"]) {
                __handled11 = true;
    continue;
            }
            if !__handled11 { __unhandled10 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled10 { exc::raise(__e); }
    if py_truthy(&__ret8) { return __val9; }
    first = geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&reply.clone(), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mcall(&first.clone(), "lower", &[]), &head.clone())) } }) {
    continue;
    }
    if py_truthy(&pyrt::xrefs::xcall("memory.repeat", "_already_repeated", &[reply.clone(), memory.clone()])) {
    continue;
    }
    return reply.clone();
    }
    return Py::None;
    Py::None
}
pub fn _asks_about_a_new_thing(args: &[Py]) -> Py {
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
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut have: Py = Py::None;
    let mut opened: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut want: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for \"what about a case\" whose thing the answer never speaks of.\n\n    \"im buying a new laptop\" is answered with what to look for in the laptop, and the next line,\n    \"what about a case\", was handed that same answer back - \"Same answer for a case: Get 16GB of\n    RAM...\" - which is not about a case at all. A thing named after \"what about\" is only a\n    narrowing of the answer when the answer speaks of it; when it does not, the line is a\n    question about that thing and the honest answer is no answer here, not the last one again.\n    ");
    opened = mcall(&g_ABOUT_OPEN(), "match", &[_flat(&[text.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&opened.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("(?:a|an|some)\\b"), slice_or_raise(&_flat(&[text.clone()]), &mcall(&opened.clone(), "end", &[]), &Py::None, &Py::None), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    stem = py_func("stem", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "stem", args));
    want = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[detail.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_VAGUE(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NAME_ONLY(), &word.clone())) } }) {  __out.push(py_call(&stem, &[word.clone()])); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return Py::Bool(true);
    }
    have = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[said.clone()])) { let mut word: Py = __item.clone(); __out.push(py_call(&stem, &[word.clone()])); } py_set(__out) })() };
    return Py::Bool(!py_truthy(&op_bitand(&want.clone(), &have.clone())));
    Py::None
}
pub fn _step_about(args: &[Py]) -> Py {
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
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut at: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut group: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut have: Py = Py::None;
    let mut met: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pos: Py = Py::None;
    let mut qualifier: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut w: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("The part of the answer that is about the thing the detail names, or None.\n\n    \"i need to buy a tent\" / \"what about a sleeping bag\" was answered from the last instruction\n    in the answer - \"Book the pitch ahead and check the site's rules on fires\" - which is advice\n    about the pitch, not about a sleeping bag. When the answer names the thing the detail names,\n    that is where the answer is, however far down the paragraph it sits: the clause that names it,\n    and the qualifier the sentence puts after it (\"a foam or inflatable one, not just the tent\n    floor\").\n    ");
    want = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[detail.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_VAGUE(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NAME_ONLY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return Py::None;
    }
    if py_truthy(&_is_a_span(&[detail.clone()])) {
    return Py::None;
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(mcall(&one.clone(), "strip", &[])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    groups = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("\\s+-\\s+|\\s+\\u2014\\s+"), sentence.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "strip", &[])); } py_list(__out) })() };
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[groups.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    at = __unpk3[0].clone();
    group = __unpk3[1].clone();
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&group.clone(), "split", &[py_str(",")])) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(mcall(&one.clone(), "strip", &[])); } } py_list(__out) })() };
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    pos = __unpk5[0].clone();
    clause = __unpk5[1].clone();
    have = call_builtin("set", &[_words(&[clause.clone()])]).unwrap_or_else(|e| exc::raise(e));
    met = op_bitand(&have.clone(), &want.clone());
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = op_le(&want.clone(), &have.clone()); if py_truthy(&__b) { __b } else { op_gt(&op_mul(&call_builtin("len", &[met.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[want.clone()]).unwrap_or_else(|e| exc::raise(e))) } }))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_one_of_a_list(&[clauses.clone(), groups.clone(), at.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&pos.clone(), &op_sub(&call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[groups.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    qualifier = mcall(&geti(&groups.clone(), &op_add(&at.clone(), &Py::Int(1))), "strip", &[py_str(" .,;:-")]);
    if py_truthy(&{ let __cmp61: Py = call_builtin("len", &[mcall(&qualifier.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp61)) { op_le(&__cmp61, &g_STEP_WORDS()) } else { Py::Bool(false) } }) {
    return op_add(&qualifier.clone(), &py_str("."));
    }
    }
    if py_truthy(&{ let __b = op_le(&have.clone(), &op_bitor(&want.clone(), &g_NAME_ONLY())); if !py_truthy(&__b) { __b } else { op_lt(&op_add(&pos.clone(), &Py::Int(1)), &call_builtin("len", &[clauses.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    clause = op_add(&op_add(&geti(&clauses.clone(), &pos.clone()), &py_str(", ")), &geti(&clauses.clone(), &op_add(&pos.clone(), &Py::Int(1))));
    }
    clause = mcall(&g_JOIN_LEAD(), "sub", &[py_str(""), mcall(&clause.clone(), "strip", &[py_str(" .,;:-")])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clause.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS()) } }) {
    return Py::None;
    }
    return op_add(&clause.clone(), &py_str("."));
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_branch(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let _ = py_str("Whether the sentence is a case and what happens in it, rather than a move.\n\n    \"A cat is the kinder choice if you're out all day\" is a branch of the answer: it says what\n    to do in a particular case. Handed over as a step it answers the person's detail with a\n    condition, and to whoever just said the other case it is not even theirs: \"im at home all\n    day\" came back as \"At home all day: A cat is the kinder choice if you're out all day\".\n    ");
    return call_builtin("bool", &[pyrt::xrefs::xcall("memory.condition", "conditions", &[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _without_branches(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut sentences: Py = Py::None;
    let _ = py_str("The answer with the sentences that are cases of it taken out.");
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&one.clone(), "strip", &[])) {  __out.push(mcall(&one.clone(), "strip", &[])); } } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_a_branch(&[one.clone()])))) {  __out.push(one.clone()); } } py_list(__out) })() }]);
    Py::None
}
pub fn _the_move_itself(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut move_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "move") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: move"))) };
    let mut head: Py = Py::None;
    let _ = py_str("The move without the half a colon puts after it.\n\n    A colon says the clause in front of it is complete: what follows is a list or a second\n    sentence, not the rest of the same one. Cut at a comma like any other clause boundary and the\n    two halves were joined straight back together, so \"ask early and in writing: unused holiday\n    can usually be taken inside the notice\" came back as \"Ask early and in writing, unused\n    holiday can usually be taken inside the notice\" - a run-on that is not a sentence, handed to\n    somebody who asked about the holiday they are owed. The half after the colon is never part of\n    the step: they have had the answer once.\n    ");
    head = mcall(&geti(&mcall(&move_.clone(), "split", &[py_str(":"), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str(" ,;.-")]);
    return if py_truthy(&op_ge(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { head.clone() } else { move_.clone() };
    Py::None
}
pub fn _the_heading(args: &[Py]) -> Py {
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
    let mut label: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The label a card opens with (\"Air fryer:\", \"Slow cooker:\"), or \"\" when it has none.");
    text = mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    found = mcall(&g_HEADING(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    label = mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "rstrip", &[]), "rstrip", &[py_str(":")]);
    words = mcall(&label.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_HEADING_WORDS()) } }) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CLAUSE_WORDS(), &mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str("*_\"',")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    return label.clone();
    Py::None
}
pub fn _is_the_label_itself(args: &[Py]) -> Py {
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
    let mut label: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "label") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: label"))) };
    let _ = py_str("True when the words are the label and nothing else, whatever the punctuation.");
    return Py::Bool(py_eq(&mcall(&_flat(&[text.clone()]), "strip", &[py_str(" .,;:")]), &mcall(&_flat(&[label.clone()]), "strip", &[py_str(" .,;:")])));
    Py::None
}
pub fn _without_the_heading(args: &[Py]) -> Py {
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
    let mut label: Py = Py::None;
    let _ = py_str("The answer without the label in front of it, or the answer itself when it has none.");
    label = _the_heading(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&label.clone()))) {
    return call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&mcall(&mcall(&slice_or_raise(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), &call_builtin("len", &[label.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "lstrip", &[]), "lstrip", &[py_str(":")]), "lstrip", &[]);
    Py::None
}
pub fn _first_instruction(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut first: Py = Py::None;
    let mut one: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer's first sentence that opens on something to do, cut to its first clause, or \"\".\n\n    The instruction is often not the opening sentence (\"Waking in the night is normal\" comes\n    first, \"Keep the wake-up time the same\" comes later), and the person who has just said how\n    long it has been going wants the instruction.\n    ");
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_clauses(&[sentence.clone()])) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "strip", &[py_str(" ,;:.-")])); } py_list(__out) })() };
    first = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&IMPERATIVES.clone(), &mcall(&geti(&mcall(&first.clone(), "split", &[]), &Py::Int(0)), "lower", &[]))) } }) {
    continue;
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    bare = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("\\s*\\([^)]*\\)"), py_str(""), first.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    first = { let __b = bare.clone(); if py_truthy(&__b) { __b } else { first.clone() } };
    }
    if py_truthy(&{ let __cmp21: Py = call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &g_STEP_WORDS()) } else { Py::Bool(false) } }) {
    return op_add(&first.clone(), &py_str("."));
    }
    }
    return py_str("");
    Py::None
}
pub fn _opening_said(args: &[Py]) -> Py {
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
    let _ = py_str("The answer's first sentence, or its first clause when that is long.");
    first = mcall(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&first.clone()))) {
    return py_str("");
    }
    if py_truthy(&op_le(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    return if py_truthy(&mcall(&first.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { first.clone() } else { op_add(&first.clone(), &py_str(".")) };
    }
    return _first_clause(&[first.clone()]);
    Py::None
}
pub fn _is_a_span(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut w: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("True when the detail is only how long a thing has been going on, with no name in it.");
    want = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[detail.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_VAGUE(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NAME_ONLY(), &w.clone())) } }) {  __out.push(w.clone()); } } py_set(__out) })() };
    return { let __b = call_builtin("bool", &[want.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_sub(&op_sub(&op_sub(&want.clone(), &g_SPAN_WORDS()), &g_NUMBER_WORDS()), &g_SPAN_WORDS_ABOUT()))) } };
    Py::None
}
pub fn _a_span_step(args: &[Py]) -> Py {
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
    let mut detail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut piece: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The move in the answer that speaks of a span, when the detail is a span, or \"\".\n\n    \"i want to start running\" / \"its been about two weeks now\" is somebody two weeks into it, so\n    the sentence of the answer that speaks of a length of time is the one for them (\"Each week add\n    a minute to the jog\") rather than the opening sentence they have already read. The last such\n    sentence wins, because an answer gives the first step and then how it goes from there.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[_words(&[detail.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_SPAN_WORDS())))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), _flat(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_DURATION(), "search", &[sentence.clone()])))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&_clauses(&[sentence.clone()]));
    for __item in __iter2 {
    piece = __item.clone();
    piece = mcall(&mcall(&g_LEAD_AND(), "sub", &[py_str(""), mcall(&piece.clone(), "strip", &[py_str(" ,;:.-")])]), "strip", &[py_str(" ,;:.-")]);
    if py_truthy(&{ let __cmp31: Py = call_builtin("len", &[mcall(&piece.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp31)) { op_le(&__cmp31, &g_STEP_WORDS()) } else { Py::Bool(false) } }) {
    return op_add(&piece.clone(), &py_str("."));
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _opening_of(args: &[Py]) -> Py {
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
    let mut IMPERATIVES: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut clauses: Py = Py::None;
    let mut first: Py = Py::None;
    let mut one: Py = Py::None;
    let mut p: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The first clause of the answer's opening sentence, when that sentence says what to do.");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pyrt::xrefs::xget("memory.repeat", "SENTENCE"), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&op_lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    clauses = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_clauses(&[geti(&parts.clone(), &Py::Int(0))])) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "strip", &[py_str(" ,;:.-")])); } py_list(__out) })() };
    first = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clauses.clone()) { let mut one: Py = __item.clone(); if py_truthy(&one.clone()) {  __out.push(one.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = first.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS()) } }) {
    bare = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("\\s*\\([^)]*\\)"), py_str(""), first.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&bare.clone()) {
    first = bare.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&first.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS()) } } } }) {
    return py_str("");
    }
    IMPERATIVES = pyrt::xrefs::xget("compose.verdict", "IMPERATIVES");
    if py_truthy(&Py::Bool(!py_contains(&IMPERATIVES.clone(), &mcall(&geti(&mcall(&first.clone(), "split", &[]), &Py::Int(0)), "lower", &[])))) {
    return py_str("");
    }
    return if py_truthy(&Py::Bool(!py_truthy(&mcall(&first.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])))) { op_add(&first.clone(), &py_str(".")) } else { first.clone() };
    Py::None
}
pub fn _names_nothing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut flat: Py = Py::None;
    let mut opening: Py = Py::None;
    let _ = py_str("True when the phrase opens on a preposition and names no place and no use.\n\n    \"in the end\", \"at first\", \"in my case\", \"for now\" all open on a preposition and none of them\n    says where the thing is or what it is for, so they are a remark rather than a narrowing of the\n    answer. One answer to \"is this a place?\" is shared by the join cut and by `_says_where`, which\n    is what decides whether a detail gets the answer's own step: a phrase that names nothing gets\n    neither.\n    ");
    flat = _flat(&[detail.clone()]);
    opening = mcall(&g_PREPOSITION(), "match", &[flat.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_NAMES_NOTHING(), "match", &[mcall(&slice_or_raise(&flat.clone(), &mcall(&opening.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _says_where(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut detail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "detail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: detail"))) };
    let mut flat: Py = Py::None;
    let _ = py_str("True when the detail narrows the answer by where or by what for, and not by when.\n\n    One answer to \"is this a place?\" for both readers: a phrase that opens on a preposition narrows\n    the answer, unless it is a fixed phrase that names nothing (\"in the end\", \"at least\") or a\n    clause of the person's own with a subject in it (\"at home he refuses\"). Those are remarks, and\n    the answer's step is not the reply to one.\n    ");
    flat = _flat(&[detail.clone()]);
    return { let __b = call_builtin("bool", &[mcall(&g_PREPOSITION(), "match", &[flat.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_WHEN(), "match", &[flat.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_names_nothing(&[flat.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_SUBJECT_OF_ITS_OWN(), "search", &[flat.clone()]))) } } } } } };
    Py::None
}
pub fn _a_step(args: &[Py]) -> Py {
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
    let mut questions_are_steps: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "questions_are_steps") { v.clone() }
    else { Py::Bool(false) };
    let mut body: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut label: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut p: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut trial: Py = Py::None;
    let _ = py_str("The one move out of the answer, cut at a clause boundary so it is not the answer again.\n\n    A knowledge answer's move is often three instructions in one sentence (\"use the extractor\n    fan, open a window for ten minutes a day, dry clothes outside\"): the detail of the thing is\n    answered with the first of them, not with all three, because they have already had the\n    paragraph once.\n    ");
    move_ = mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("questions_are_steps".to_string(), questions_are_steps.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("memory.repeat", "_a_move", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[py_str(" ,;:.-")]);
    if py_truthy(&Py::Bool(!py_truthy(&move_.clone()))) {
    return py_str("");
    }
    label = _the_heading(&[said.clone()]);
    opening = _opening_of(&[said.clone()]);
    if py_truthy(&{ let __b = opening.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = label.clone(); if !py_truthy(&__b) { __b } else { _is_the_label_itself(&[opening.clone(), label.clone()]) } })) } }) {
    move_ = opening.clone();
    }
    if py_truthy(&label.clone()) {
    move_ = { let __b = _without_the_heading(&[move_.clone()]); if py_truthy(&__b) { __b } else { move_.clone() } };
    if py_truthy(&_is_the_label_itself(&[move_.clone(), label.clone()])) {
    move_ = { let __b = _the_move_itself(&[_without_the_heading(&[said.clone()])]); if py_truthy(&__b) { __b } else { move_.clone() } };
    }
    }
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_clauses(&[_the_move_itself(&[move_.clone()])])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[])) {  __out.push(mcall(&p.clone(), "strip", &[])); } } py_list(__out) })() };
    kept = if py_truthy(&pieces.clone()) { geti(&pieces.clone(), &Py::Int(0)) } else { move_.clone() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&pieces.clone(), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    piece = __item.clone();
    if py_truthy(&op_ge(&call_builtin("len", &[mcall(&kept.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SHORT_STEP())) {
    break;
    }
    trial = op_add(&op_add(&kept.clone(), &py_str(", ")), &piece.clone());
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&trial.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    break;
    }
    kept = trial.clone();
    }
    kept = mcall(&kept.clone(), "strip", &[py_str(" ,;:-.")]);
    label = _the_heading(&[said.clone()]);
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { { let __b = label.clone(); if !py_truthy(&__b) { __b } else { _is_the_label_itself(&[kept.clone(), label.clone()]) } } } }) {
    body = _without_the_heading(&[said.clone()]);
    if py_truthy(&{ let __b = body.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&body.clone(), &said.clone())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(body.clone()); __kws.extend(vec![("questions_are_steps".to_string(), questions_are_steps.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_step(args))(&__args) };
    }
    }
    return if py_truthy(&kept.clone()) { op_add(&kept.clone(), &py_str(".")) } else { py_str("") };
    Py::None
}
pub fn _capital(args: &[Py]) -> Py {
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
    said = mcall(&said.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return said.clone();
    }
    return op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn stands_alone(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut said: Py = Py::None;
    let _ = py_str("Whether a sentence can be read as the reply, with nothing before it on the screen.");
    said = mcall(&call_builtin("str", &[{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_OPENS_MID_THOUGHT(), "match", &[said.clone()]))) } };
    Py::None
}
pub fn _report_stem(args: &[Py]) -> Py {
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
    let mut ending: Py = Py::None;
    let _ = py_str("The word without the ending a verb or a plural wears, so \"drops\" meets \"dropping\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&word.clone(), &op_neg(&Py::Int(1))), &geti(&word.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _report_content(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_REPORT_FILLER(), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(_report_stem(&[w.clone()])); } } py_list(__out) })() };
    Py::None
}
pub fn a_report_about_the_thing(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_shared: Py = Py::None;
    let mut hedge: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The sentence of the last answer that speaks to this report about its thing, or \"\".");
    said = mcall(&_flat(&[text.clone()]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_REPORT_WORDS()) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_REPORT_ASKS(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REPORT_OPENS_ON_THE_THING(), "match", &[said.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_REPORT_NEGATES(), "search", &[said.clone()])) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    answer = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("len", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(60))) {
    return py_str("");
    }
    lowered = mcall(&answer.clone(), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_AN_ANSWER()) { let mut hedge: Py = __item.clone(); __out.push(Py::Bool(py_contains(&lowered.clone(), &hedge.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    about = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_report_content(&[said.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(w.clone()); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&about.clone()))) {
    return py_str("");
    }
    best = py_str("");
    best_shared = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g_REPORT_SENTENCE(), "split", &[answer.clone()]));
    for __item in __iter1 {
    sentence = __item.clone();
    sentence = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[mcall(&sentence.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&sentence.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_REPORT_BEHAVIOUR(), "search", &[sentence.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stands_alone(&[sentence.clone()])))) {
    continue;
    }
    if py_truthy(&mcall(&g_A_REFERRAL(), "search", &[sentence.clone()])) {
    continue;
    }
    shared = call_builtin("len", &[op_bitand(&about.clone(), &call_builtin("set", &[_report_content(&[sentence.clone()])]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_gt(&shared.clone(), &best_shared.clone())) {
    let __unpk2 = py_unpack(&py_tuple(vec![sentence.clone(), shared.clone()]), 2, None);
    best = __unpk2[0].clone();
    best_shared = __unpk2[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&best.clone())); if py_truthy(&__b) { __b } else { op_lt(&best_shared.clone(), &Py::Int(1)) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); __out.push(mcall(&_flat(&[mcall(&t.clone(), "get", &[py_str("bot")])]), "lower", &[])); } py_set(__out) })() }, &mcall(&_flat(&[best.clone()]), "lower", &[])))) {
    return py_str("");
    }
    return _capital(&[best.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.detail".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_OPEN" => Some(g_ABOUT_OPEN()),
            "AND_A_CLAUSE" => Some(g_AND_A_CLAUSE()),
            "ASKING" => Some(g_ASKING()),
            "ASKING_ME" => Some(g_ASKING_ME()),
            "ASKS_A_PART" => Some(g_ASKS_A_PART()),
            "ASK_ONLY" => Some(g_ASK_ONLY()),
            "A_COUNT_WORD" => Some(g_A_COUNT_WORD()),
            "A_DANGLING" => Some(g_A_DANGLING()),
            "A_DURATION" => Some(g_A_DURATION()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_GREETING" => Some(g_A_GREETING()),
            "A_PART_WORDS" => Some(g_A_PART_WORDS()),
            "A_PERSON_BODY" => Some(g_A_PERSON_BODY()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_QUALIFIER" => Some(g_A_QUALIFIER()),
            "A_QUANTITY_ASK" => Some(g_A_QUANTITY_ASK()),
            "A_REFERRAL" => Some(g_A_REFERRAL()),
            "A_SMALL_WORD" => Some(g_A_SMALL_WORD()),
            "A_STEP_ANSWER" => Some(g_A_STEP_ANSWER()),
            "A_SUBJECT_OF_ITS_OWN" => Some(g_A_SUBJECT_OF_ITS_OWN()),
            "A_TIME_IDIOM" => Some(g_A_TIME_IDIOM()),
            "A_WHEN" => Some(g_A_WHEN()),
            "CLAUSE" => Some(g_CLAUSE()),
            "CLAUSE_WORDS" => Some(g_CLAUSE_WORDS()),
            "COPULA_OPEN" => Some(g_COPULA_OPEN()),
            "COUNT" => Some(g_COUNT()),
            "COUNTED_DETAIL" => Some(g_COUNTED_DETAIL()),
            "DETAIL_BODY" => Some(g_DETAIL_BODY()),
            "DETERMINER" => Some(g_DETERMINER()),
            "FACT_SENTENCE_WORDS" => Some(g_FACT_SENTENCE_WORDS()),
            "FALTER" => Some(g_FALTER()),
            "FIGURED" => Some(g_FIGURED()),
            "FREQUENCY" => Some(g_FREQUENCY()),
            "HAVE_COUNT" => Some(g_HAVE_COUNT()),
            "HEADING" => Some(g_HEADING()),
            "HEADING_WORDS" => Some(g_HEADING_WORDS()),
            "JOIN_LEAD" => Some(g_JOIN_LEAD()),
            "JOIN_OPEN" => Some(g_JOIN_OPEN()),
            "LEADS_FOR" => Some(g_LEADS_FOR()),
            "LEADS_FOR_A_TIME" => Some(g_LEADS_FOR_A_TIME()),
            "LEADS_LEAD" => Some(g_LEADS_LEAD()),
            "LEAD_AND" => Some(g_LEAD_AND()),
            "LIKE_OPEN" => Some(g_LIKE_OPEN()),
            "MEASURE_BODY" => Some(g_MEASURE_BODY()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "NAMES_NOTHING" => Some(g_NAMES_NOTHING()),
            "NAME_ONLY" => Some(g_NAME_ONLY()),
            "NOT_AN_ANSWER" => Some(g_NOT_AN_ANSWER()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NUMBER_WORDS" => Some(g_NUMBER_WORDS()),
            "OPENS_MID_THOUGHT" => Some(g_OPENS_MID_THOUGHT()),
            "PLAIN_ASK" => Some(g_PLAIN_ASK()),
            "PLAIN_LEADS" => Some(g_PLAIN_LEADS()),
            "PREPOSITION" => Some(g_PREPOSITION()),
            "PREPOSITION_OPEN" => Some(g_PREPOSITION_OPEN()),
            "PRONOUN_OPEN" => Some(g_PRONOUN_OPEN()),
            "QUANTITY_BODY" => Some(g_QUANTITY_BODY()),
            "REPORT_ASKS" => Some(g_REPORT_ASKS()),
            "REPORT_BEHAVIOUR" => Some(g_REPORT_BEHAVIOUR()),
            "REPORT_FILLER" => Some(g_REPORT_FILLER()),
            "REPORT_NEGATES" => Some(g_REPORT_NEGATES()),
            "REPORT_OPENS_ON_THE_THING" => Some(g_REPORT_OPENS_ON_THE_THING()),
            "REPORT_SENTENCE" => Some(g_REPORT_SENTENCE()),
            "REPORT_WORDS" => Some(g_REPORT_WORDS()),
            "SHORT_STEP" => Some(g_SHORT_STEP()),
            "SHOWS_FROM" => Some(g_SHOWS_FROM()),
            "SHOWS_OUT" => Some(g_SHOWS_OUT()),
            "SPAN_ONLY" => Some(g_SPAN_ONLY()),
            "SPAN_WORDS" => Some(g_SPAN_WORDS()),
            "SPAN_WORDS_ABOUT" => Some(g_SPAN_WORDS_ABOUT()),
            "STATE" => Some(g_STATE()),
            "STEP_ANSWERS" => Some(g_STEP_ANSWERS()),
            "STEP_WORDS" => Some(g_STEP_WORDS()),
            "THE_PERSON" => Some(g_THE_PERSON()),
            "TIME_OF_DAY" => Some(g_TIME_OF_DAY()),
            "TIME_WORDS" => Some(g_TIME_WORDS()),
            "VAGUE" => Some(g_VAGUE()),
            "_a_branch" => Some(py_func("_a_branch", |args: &[Py]| _a_branch(args))),
            "_a_count" => Some(py_func("_a_count", |args: &[Py]| _a_count(args))),
            "_a_figure" => Some(py_func("_a_figure", |args: &[Py]| _a_figure(args))),
            "_a_span_step" => Some(py_func("_a_span_step", |args: &[Py]| _a_span_step(args))),
            "_a_step" => Some(py_func("_a_step", |args: &[Py]| _a_step(args))),
            "_a_time_step" => Some(py_func("_a_time_step", |args: &[Py]| _a_time_step(args))),
            "_a_verb" => Some(py_func("_a_verb", |args: &[Py]| _a_verb(args))),
            "_another_thing" => Some(py_func("_another_thing", |args: &[Py]| _another_thing(args))),
            "_asks_about_a_new_thing" => Some(py_func("_asks_about_a_new_thing", |args: &[Py]| _asks_about_a_new_thing(args))),
            "_bare_step" => Some(py_func("_bare_step", |args: &[Py]| _bare_step(args))),
            "_capital" => Some(py_func("_capital", |args: &[Py]| _capital(args))),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_first_clause" => Some(py_func("_first_clause", |args: &[Py]| _first_clause(args))),
            "_first_instruction" => Some(py_func("_first_instruction", |args: &[Py]| _first_instruction(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_head_word" => Some(py_func("_head_word", |args: &[Py]| _head_word(args))),
            "_is_a_span" => Some(py_func("_is_a_span", |args: &[Py]| _is_a_span(args))),
            "_is_the_label_itself" => Some(py_func("_is_the_label_itself", |args: &[Py]| _is_the_label_itself(args))),
            "_names_nothing" => Some(py_func("_names_nothing", |args: &[Py]| _names_nothing(args))),
            "_one_of_a_list" => Some(py_func("_one_of_a_list", |args: &[Py]| _one_of_a_list(args))),
            "_opening_of" => Some(py_func("_opening_of", |args: &[Py]| _opening_of(args))),
            "_opening_said" => Some(py_func("_opening_said", |args: &[Py]| _opening_said(args))),
            "_report_content" => Some(py_func("_report_content", |args: &[Py]| _report_content(args))),
            "_report_stem" => Some(py_func("_report_stem", |args: &[Py]| _report_stem(args))),
            "_says_about" => Some(py_func("_says_about", |args: &[Py]| _says_about(args))),
            "_says_where" => Some(py_func("_says_where", |args: &[Py]| _says_where(args))),
            "_step_about" => Some(py_func("_step_about", |args: &[Py]| _step_about(args))),
            "_subject_words" => Some(py_func("_subject_words", |args: &[Py]| _subject_words(args))),
            "_the_answer_before" => Some(py_func("_the_answer_before", |args: &[Py]| _the_answer_before(args))),
            "_the_answer_just_given" => Some(py_func("_the_answer_just_given", |args: &[Py]| _the_answer_just_given(args))),
            "_the_carrying_words" => Some(py_func("_the_carrying_words", |args: &[Py]| _the_carrying_words(args))),
            "_the_clause_that_holds" => Some(py_func("_the_clause_that_holds", |args: &[Py]| _the_clause_that_holds(args))),
            "_the_entry_reply" => Some(py_func("_the_entry_reply", |args: &[Py]| _the_entry_reply(args))),
            "_the_figure" => Some(py_func("_the_figure", |args: &[Py]| _the_figure(args))),
            "_the_heading" => Some(py_func("_the_heading", |args: &[Py]| _the_heading(args))),
            "_the_move_itself" => Some(py_func("_the_move_itself", |args: &[Py]| _the_move_itself(args))),
            "_the_same_answer_as_the_one_it_narrows" => Some(py_func("_the_same_answer_as_the_one_it_narrows", |args: &[Py]| _the_same_answer_as_the_one_it_narrows(args))),
            "_the_thing_itself" => Some(py_func("_the_thing_itself", |args: &[Py]| _the_thing_itself(args))),
            "_what_it_is_worth" => Some(py_func("_what_it_is_worth", |args: &[Py]| _what_it_is_worth(args))),
            "_without_branches" => Some(py_func("_without_branches", |args: &[Py]| _without_branches(args))),
            "_without_the_heading" => Some(py_func("_without_the_heading", |args: &[Py]| _without_the_heading(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "a_report_about_the_thing" => Some(py_func("a_report_about_the_thing", |args: &[Py]| a_report_about_the_thing(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "answers_the_line" => Some(py_func("answers_the_line", |args: &[Py]| answers_the_line(args))),
            "stands_alone" => Some(py_func("stands_alone", |args: &[Py]| stands_alone(args))),
            "the_detail" => Some(py_func("the_detail", |args: &[Py]| the_detail(args))),
            "the_fact_the_answer_has" => Some(py_func("the_fact_the_answer_has", |args: &[Py]| the_fact_the_answer_has(args))),
            "the_part_of_the_answer" => Some(py_func("the_part_of_the_answer", |args: &[Py]| the_part_of_the_answer(args))),
            "the_plain_ask" => Some(py_func("the_plain_ask", |args: &[Py]| the_plain_ask(args))),
            "the_step_for" => Some(py_func("the_step_for", |args: &[Py]| the_step_for(args))),
            "the_step_for_a_time" => Some(py_func("the_step_for_a_time", |args: &[Py]| the_step_for_a_time(args))),
            "the_time_in_the_line" => Some(py_func("the_time_in_the_line", |args: &[Py]| the_time_in_the_line(args))),
            "kinds" => Some(pyrt::xrefs::xmodule("understanding.kinds")),
            "again" => Some(pyrt::xrefs::xmodule("memory.again")),
            "condition" => Some(pyrt::xrefs::xmodule("memory.condition")),
            "repeat" => Some(pyrt::xrefs::xmodule("memory.repeat")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_OPEN" => s_ABOUT_OPEN(v),
            "AND_A_CLAUSE" => s_AND_A_CLAUSE(v),
            "ASKING" => s_ASKING(v),
            "ASKING_ME" => s_ASKING_ME(v),
            "ASKS_A_PART" => s_ASKS_A_PART(v),
            "ASK_ONLY" => s_ASK_ONLY(v),
            "A_COUNT_WORD" => s_A_COUNT_WORD(v),
            "A_DANGLING" => s_A_DANGLING(v),
            "A_DURATION" => s_A_DURATION(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_GREETING" => s_A_GREETING(v),
            "A_PART_WORDS" => s_A_PART_WORDS(v),
            "A_PERSON_BODY" => s_A_PERSON_BODY(v),
            "A_PLACE" => s_A_PLACE(v),
            "A_QUALIFIER" => s_A_QUALIFIER(v),
            "A_QUANTITY_ASK" => s_A_QUANTITY_ASK(v),
            "A_REFERRAL" => s_A_REFERRAL(v),
            "A_SMALL_WORD" => s_A_SMALL_WORD(v),
            "A_STEP_ANSWER" => s_A_STEP_ANSWER(v),
            "A_SUBJECT_OF_ITS_OWN" => s_A_SUBJECT_OF_ITS_OWN(v),
            "A_TIME_IDIOM" => s_A_TIME_IDIOM(v),
            "A_WHEN" => s_A_WHEN(v),
            "CLAUSE" => s_CLAUSE(v),
            "CLAUSE_WORDS" => s_CLAUSE_WORDS(v),
            "COPULA_OPEN" => s_COPULA_OPEN(v),
            "COUNT" => s_COUNT(v),
            "COUNTED_DETAIL" => s_COUNTED_DETAIL(v),
            "DETAIL_BODY" => s_DETAIL_BODY(v),
            "DETERMINER" => s_DETERMINER(v),
            "FACT_SENTENCE_WORDS" => s_FACT_SENTENCE_WORDS(v),
            "FALTER" => s_FALTER(v),
            "FIGURED" => s_FIGURED(v),
            "FREQUENCY" => s_FREQUENCY(v),
            "HAVE_COUNT" => s_HAVE_COUNT(v),
            "HEADING" => s_HEADING(v),
            "HEADING_WORDS" => s_HEADING_WORDS(v),
            "JOIN_LEAD" => s_JOIN_LEAD(v),
            "JOIN_OPEN" => s_JOIN_OPEN(v),
            "LEADS_FOR" => s_LEADS_FOR(v),
            "LEADS_FOR_A_TIME" => s_LEADS_FOR_A_TIME(v),
            "LEADS_LEAD" => s_LEADS_LEAD(v),
            "LEAD_AND" => s_LEAD_AND(v),
            "LIKE_OPEN" => s_LIKE_OPEN(v),
            "MEASURE_BODY" => s_MEASURE_BODY(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "NAMES_NOTHING" => s_NAMES_NOTHING(v),
            "NAME_ONLY" => s_NAME_ONLY(v),
            "NOT_AN_ANSWER" => s_NOT_AN_ANSWER(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NUMBER_WORDS" => s_NUMBER_WORDS(v),
            "OPENS_MID_THOUGHT" => s_OPENS_MID_THOUGHT(v),
            "PLAIN_ASK" => s_PLAIN_ASK(v),
            "PLAIN_LEADS" => s_PLAIN_LEADS(v),
            "PREPOSITION" => s_PREPOSITION(v),
            "PREPOSITION_OPEN" => s_PREPOSITION_OPEN(v),
            "PRONOUN_OPEN" => s_PRONOUN_OPEN(v),
            "QUANTITY_BODY" => s_QUANTITY_BODY(v),
            "REPORT_ASKS" => s_REPORT_ASKS(v),
            "REPORT_BEHAVIOUR" => s_REPORT_BEHAVIOUR(v),
            "REPORT_FILLER" => s_REPORT_FILLER(v),
            "REPORT_NEGATES" => s_REPORT_NEGATES(v),
            "REPORT_OPENS_ON_THE_THING" => s_REPORT_OPENS_ON_THE_THING(v),
            "REPORT_SENTENCE" => s_REPORT_SENTENCE(v),
            "REPORT_WORDS" => s_REPORT_WORDS(v),
            "SHORT_STEP" => s_SHORT_STEP(v),
            "SHOWS_FROM" => s_SHOWS_FROM(v),
            "SHOWS_OUT" => s_SHOWS_OUT(v),
            "SPAN_ONLY" => s_SPAN_ONLY(v),
            "SPAN_WORDS" => s_SPAN_WORDS(v),
            "SPAN_WORDS_ABOUT" => s_SPAN_WORDS_ABOUT(v),
            "STATE" => s_STATE(v),
            "STEP_ANSWERS" => s_STEP_ANSWERS(v),
            "STEP_WORDS" => s_STEP_WORDS(v),
            "THE_PERSON" => s_THE_PERSON(v),
            "TIME_OF_DAY" => s_TIME_OF_DAY(v),
            "TIME_WORDS" => s_TIME_WORDS(v),
            "VAGUE" => s_VAGUE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.detail", module_obj);
    pyrt::xrefs::reg_fn("memory.detail", "_a_verb", py_func("_a_verb", |args: &[Py]| _a_verb(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_flat", py_func("_flat", |args: &[Py]| _flat(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_same_answer_as_the_one_it_narrows", py_func("_the_same_answer_as_the_one_it_narrows", |args: &[Py]| _the_same_answer_as_the_one_it_narrows(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_detail", py_func("the_detail", |args: &[Py]| the_detail(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_a_figure", py_func("_a_figure", |args: &[Py]| _a_figure(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_fact_the_answer_has", py_func("the_fact_the_answer_has", |args: &[Py]| the_fact_the_answer_has(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_step_for", py_func("the_step_for", |args: &[Py]| the_step_for(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_time_in_the_line", py_func("the_time_in_the_line", |args: &[Py]| the_time_in_the_line(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_a_time_step", py_func("_a_time_step", |args: &[Py]| _a_time_step(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_answer_before", py_func("_the_answer_before", |args: &[Py]| _the_answer_before(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_step_for_a_time", py_func("the_step_for_a_time", |args: &[Py]| the_step_for_a_time(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_a_count", py_func("_a_count", |args: &[Py]| _a_count(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_figure", py_func("_the_figure", |args: &[Py]| _the_figure(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_carrying_words", py_func("_the_carrying_words", |args: &[Py]| _the_carrying_words(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_clause_that_holds", py_func("_the_clause_that_holds", |args: &[Py]| _the_clause_that_holds(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_part_of_the_answer", py_func("the_part_of_the_answer", |args: &[Py]| the_part_of_the_answer(args)));
    pyrt::xrefs::reg_fn("memory.detail", "answers_the_line", py_func("answers_the_line", |args: &[Py]| answers_the_line(args)));
    pyrt::xrefs::reg_fn("memory.detail", "answer_for", py_func("answer_for", |args: &[Py]| answer_for(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_answer_just_given", py_func("_the_answer_just_given", |args: &[Py]| _the_answer_just_given(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_bare_step", py_func("_bare_step", |args: &[Py]| _bare_step(args)));
    pyrt::xrefs::reg_fn("memory.detail", "the_plain_ask", py_func("the_plain_ask", |args: &[Py]| the_plain_ask(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_clauses", py_func("_clauses", |args: &[Py]| _clauses(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_first_clause", py_func("_first_clause", |args: &[Py]| _first_clause(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_one_of_a_list", py_func("_one_of_a_list", |args: &[Py]| _one_of_a_list(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_head_word", py_func("_head_word", |args: &[Py]| _head_word(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_subject_words", py_func("_subject_words", |args: &[Py]| _subject_words(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_entry_reply", py_func("_the_entry_reply", |args: &[Py]| _the_entry_reply(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_another_thing", py_func("_another_thing", |args: &[Py]| _another_thing(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_what_it_is_worth", py_func("_what_it_is_worth", |args: &[Py]| _what_it_is_worth(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_says_about", py_func("_says_about", |args: &[Py]| _says_about(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_thing_itself", py_func("_the_thing_itself", |args: &[Py]| _the_thing_itself(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_asks_about_a_new_thing", py_func("_asks_about_a_new_thing", |args: &[Py]| _asks_about_a_new_thing(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_step_about", py_func("_step_about", |args: &[Py]| _step_about(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_a_branch", py_func("_a_branch", |args: &[Py]| _a_branch(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_without_branches", py_func("_without_branches", |args: &[Py]| _without_branches(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_move_itself", py_func("_the_move_itself", |args: &[Py]| _the_move_itself(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_the_heading", py_func("_the_heading", |args: &[Py]| _the_heading(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_is_the_label_itself", py_func("_is_the_label_itself", |args: &[Py]| _is_the_label_itself(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_without_the_heading", py_func("_without_the_heading", |args: &[Py]| _without_the_heading(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_first_instruction", py_func("_first_instruction", |args: &[Py]| _first_instruction(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_opening_said", py_func("_opening_said", |args: &[Py]| _opening_said(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_is_a_span", py_func("_is_a_span", |args: &[Py]| _is_a_span(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_a_span_step", py_func("_a_span_step", |args: &[Py]| _a_span_step(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_opening_of", py_func("_opening_of", |args: &[Py]| _opening_of(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_names_nothing", py_func("_names_nothing", |args: &[Py]| _names_nothing(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_says_where", py_func("_says_where", |args: &[Py]| _says_where(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_a_step", py_func("_a_step", |args: &[Py]| _a_step(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_capital", py_func("_capital", |args: &[Py]| _capital(args)));
    pyrt::xrefs::reg_fn("memory.detail", "stands_alone", py_func("stands_alone", |args: &[Py]| stands_alone(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_report_stem", py_func("_report_stem", |args: &[Py]| _report_stem(args)));
    pyrt::xrefs::reg_fn("memory.detail", "_report_content", py_func("_report_content", |args: &[Py]| _report_content(args)));
    pyrt::xrefs::reg_fn("memory.detail", "a_report_about_the_thing", py_func("a_report_about_the_thing", |args: &[Py]| a_report_about_the_thing(args)));
    pyrt::xrefs::reg_access("memory.detail", "ABOUT_OPEN", g_ABOUT_OPEN, s_ABOUT_OPEN);
    pyrt::xrefs::reg_access("memory.detail", "AND_A_CLAUSE", g_AND_A_CLAUSE, s_AND_A_CLAUSE);
    pyrt::xrefs::reg_access("memory.detail", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.detail", "ASKING_ME", g_ASKING_ME, s_ASKING_ME);
    pyrt::xrefs::reg_access("memory.detail", "ASKS_A_PART", g_ASKS_A_PART, s_ASKS_A_PART);
    pyrt::xrefs::reg_access("memory.detail", "ASK_ONLY", g_ASK_ONLY, s_ASK_ONLY);
    pyrt::xrefs::reg_access("memory.detail", "A_COUNT_WORD", g_A_COUNT_WORD, s_A_COUNT_WORD);
    pyrt::xrefs::reg_access("memory.detail", "A_DANGLING", g_A_DANGLING, s_A_DANGLING);
    pyrt::xrefs::reg_access("memory.detail", "A_DURATION", g_A_DURATION, s_A_DURATION);
    pyrt::xrefs::reg_access("memory.detail", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.detail", "A_GREETING", g_A_GREETING, s_A_GREETING);
    pyrt::xrefs::reg_access("memory.detail", "A_PART_WORDS", g_A_PART_WORDS, s_A_PART_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "A_PERSON_BODY", g_A_PERSON_BODY, s_A_PERSON_BODY);
    pyrt::xrefs::reg_access("memory.detail", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("memory.detail", "A_QUALIFIER", g_A_QUALIFIER, s_A_QUALIFIER);
    pyrt::xrefs::reg_access("memory.detail", "A_QUANTITY_ASK", g_A_QUANTITY_ASK, s_A_QUANTITY_ASK);
    pyrt::xrefs::reg_access("memory.detail", "A_REFERRAL", g_A_REFERRAL, s_A_REFERRAL);
    pyrt::xrefs::reg_access("memory.detail", "A_SMALL_WORD", g_A_SMALL_WORD, s_A_SMALL_WORD);
    pyrt::xrefs::reg_access("memory.detail", "A_STEP_ANSWER", g_A_STEP_ANSWER, s_A_STEP_ANSWER);
    pyrt::xrefs::reg_access("memory.detail", "A_SUBJECT_OF_ITS_OWN", g_A_SUBJECT_OF_ITS_OWN, s_A_SUBJECT_OF_ITS_OWN);
    pyrt::xrefs::reg_access("memory.detail", "A_TIME_IDIOM", g_A_TIME_IDIOM, s_A_TIME_IDIOM);
    pyrt::xrefs::reg_access("memory.detail", "A_WHEN", g_A_WHEN, s_A_WHEN);
    pyrt::xrefs::reg_access("memory.detail", "CLAUSE", g_CLAUSE, s_CLAUSE);
    pyrt::xrefs::reg_access("memory.detail", "CLAUSE_WORDS", g_CLAUSE_WORDS, s_CLAUSE_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "COPULA_OPEN", g_COPULA_OPEN, s_COPULA_OPEN);
    pyrt::xrefs::reg_access("memory.detail", "COUNT", g_COUNT, s_COUNT);
    pyrt::xrefs::reg_access("memory.detail", "COUNTED_DETAIL", g_COUNTED_DETAIL, s_COUNTED_DETAIL);
    pyrt::xrefs::reg_access("memory.detail", "DETAIL_BODY", g_DETAIL_BODY, s_DETAIL_BODY);
    pyrt::xrefs::reg_access("memory.detail", "DETERMINER", g_DETERMINER, s_DETERMINER);
    pyrt::xrefs::reg_access("memory.detail", "FACT_SENTENCE_WORDS", g_FACT_SENTENCE_WORDS, s_FACT_SENTENCE_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "FALTER", g_FALTER, s_FALTER);
    pyrt::xrefs::reg_access("memory.detail", "FIGURED", g_FIGURED, s_FIGURED);
    pyrt::xrefs::reg_access("memory.detail", "FREQUENCY", g_FREQUENCY, s_FREQUENCY);
    pyrt::xrefs::reg_access("memory.detail", "HAVE_COUNT", g_HAVE_COUNT, s_HAVE_COUNT);
    pyrt::xrefs::reg_access("memory.detail", "HEADING", g_HEADING, s_HEADING);
    pyrt::xrefs::reg_access("memory.detail", "HEADING_WORDS", g_HEADING_WORDS, s_HEADING_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "JOIN_LEAD", g_JOIN_LEAD, s_JOIN_LEAD);
    pyrt::xrefs::reg_access("memory.detail", "JOIN_OPEN", g_JOIN_OPEN, s_JOIN_OPEN);
    pyrt::xrefs::reg_access("memory.detail", "LEADS_FOR", g_LEADS_FOR, s_LEADS_FOR);
    pyrt::xrefs::reg_access("memory.detail", "LEADS_FOR_A_TIME", g_LEADS_FOR_A_TIME, s_LEADS_FOR_A_TIME);
    pyrt::xrefs::reg_access("memory.detail", "LEADS_LEAD", g_LEADS_LEAD, s_LEADS_LEAD);
    pyrt::xrefs::reg_access("memory.detail", "LEAD_AND", g_LEAD_AND, s_LEAD_AND);
    pyrt::xrefs::reg_access("memory.detail", "LIKE_OPEN", g_LIKE_OPEN, s_LIKE_OPEN);
    pyrt::xrefs::reg_access("memory.detail", "MEASURE_BODY", g_MEASURE_BODY, s_MEASURE_BODY);
    pyrt::xrefs::reg_access("memory.detail", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "NAMES_NOTHING", g_NAMES_NOTHING, s_NAMES_NOTHING);
    pyrt::xrefs::reg_access("memory.detail", "NAME_ONLY", g_NAME_ONLY, s_NAME_ONLY);
    pyrt::xrefs::reg_access("memory.detail", "NOT_AN_ANSWER", g_NOT_AN_ANSWER, s_NOT_AN_ANSWER);
    pyrt::xrefs::reg_access("memory.detail", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.detail", "NUMBER_WORDS", g_NUMBER_WORDS, s_NUMBER_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "OPENS_MID_THOUGHT", g_OPENS_MID_THOUGHT, s_OPENS_MID_THOUGHT);
    pyrt::xrefs::reg_access("memory.detail", "PLAIN_ASK", g_PLAIN_ASK, s_PLAIN_ASK);
    pyrt::xrefs::reg_access("memory.detail", "PLAIN_LEADS", g_PLAIN_LEADS, s_PLAIN_LEADS);
    pyrt::xrefs::reg_access("memory.detail", "PREPOSITION", g_PREPOSITION, s_PREPOSITION);
    pyrt::xrefs::reg_access("memory.detail", "PREPOSITION_OPEN", g_PREPOSITION_OPEN, s_PREPOSITION_OPEN);
    pyrt::xrefs::reg_access("memory.detail", "PRONOUN_OPEN", g_PRONOUN_OPEN, s_PRONOUN_OPEN);
    pyrt::xrefs::reg_access("memory.detail", "QUANTITY_BODY", g_QUANTITY_BODY, s_QUANTITY_BODY);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_ASKS", g_REPORT_ASKS, s_REPORT_ASKS);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_BEHAVIOUR", g_REPORT_BEHAVIOUR, s_REPORT_BEHAVIOUR);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_FILLER", g_REPORT_FILLER, s_REPORT_FILLER);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_NEGATES", g_REPORT_NEGATES, s_REPORT_NEGATES);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_OPENS_ON_THE_THING", g_REPORT_OPENS_ON_THE_THING, s_REPORT_OPENS_ON_THE_THING);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_SENTENCE", g_REPORT_SENTENCE, s_REPORT_SENTENCE);
    pyrt::xrefs::reg_access("memory.detail", "REPORT_WORDS", g_REPORT_WORDS, s_REPORT_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "SHORT_STEP", g_SHORT_STEP, s_SHORT_STEP);
    pyrt::xrefs::reg_access("memory.detail", "SHOWS_FROM", g_SHOWS_FROM, s_SHOWS_FROM);
    pyrt::xrefs::reg_access("memory.detail", "SHOWS_OUT", g_SHOWS_OUT, s_SHOWS_OUT);
    pyrt::xrefs::reg_access("memory.detail", "SPAN_ONLY", g_SPAN_ONLY, s_SPAN_ONLY);
    pyrt::xrefs::reg_access("memory.detail", "SPAN_WORDS", g_SPAN_WORDS, s_SPAN_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "SPAN_WORDS_ABOUT", g_SPAN_WORDS_ABOUT, s_SPAN_WORDS_ABOUT);
    pyrt::xrefs::reg_access("memory.detail", "STATE", g_STATE, s_STATE);
    pyrt::xrefs::reg_access("memory.detail", "STEP_ANSWERS", g_STEP_ANSWERS, s_STEP_ANSWERS);
    pyrt::xrefs::reg_access("memory.detail", "STEP_WORDS", g_STEP_WORDS, s_STEP_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "THE_PERSON", g_THE_PERSON, s_THE_PERSON);
    pyrt::xrefs::reg_access("memory.detail", "TIME_OF_DAY", g_TIME_OF_DAY, s_TIME_OF_DAY);
    pyrt::xrefs::reg_access("memory.detail", "TIME_WORDS", g_TIME_WORDS, s_TIME_WORDS);
    pyrt::xrefs::reg_access("memory.detail", "VAGUE", g_VAGUE, s_VAGUE);
}

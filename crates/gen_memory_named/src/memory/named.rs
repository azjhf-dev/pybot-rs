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


thread_local! { static G_ACTIVITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AFTER_THE_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AIM_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_FOR_IDEAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKED_THE_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKS_FOR_AN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASPECT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASPECT_FILLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASPECT_NOUNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BARE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DEADLINE_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DOING_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE_ON_THE_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FITTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GERUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GETTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HOW_OFTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LENGTH_STATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLAN_TO_THE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PURPOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THANK_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TRIP_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VAGUE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ANOTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BASE_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEST_TIME_TO_DO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTINUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISTRESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOING_BASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FED_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEELING_SAID_AS_A_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOAL_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_FRONT_OF_THE_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_PROGRESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOINING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOURNEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_FILLERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_KEY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LENGTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIGHT_DOING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOST_WORDS_ASKED_ABOUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAMED_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NAME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_SHELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_GOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_THING_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_THANKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUTCOME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PART_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PEOPLE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_ARTICLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_PAIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_PREPOSITIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTED_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_AT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROGRESS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROGRESS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_NOT_A_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPAN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKE_BACK_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKE_BACK_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASKING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_ASKING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THE_LENGTH_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIMES_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_MANNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VIEW_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WANTS_A_NEW_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_IT_STANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_X_IS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AND_AT_THE_START: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASK_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CONNECTIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_IN_A_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GOING_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LIVING_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_PRACTICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_LIFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_is_ack: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_reads_as_bad_news: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_carried_topic: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_subject: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT memory.named: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ACTIVITY_WORDS() -> Py { ensure_init(); G_ACTIVITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACTIVITY_WORDS(v: Py) { ensure_init(); G_ACTIVITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AFTER_THE_NAME() -> Py { ensure_init(); G_AFTER_THE_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_NAME(v: Py) { ensure_init(); G_AFTER_THE_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_AIM_VERB() -> Py { ensure_init(); G_AIM_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AIM_VERB(v: Py) { ensure_init(); G_AIM_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_FOR_IDEAS() -> Py { ensure_init(); G_ASKED_FOR_IDEAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_FOR_IDEAS(v: Py) { ensure_init(); G_ASKED_FOR_IDEAS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKED_THE_MEASURE() -> Py { ensure_init(); G_ASKED_THE_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKED_THE_MEASURE(v: Py) { ensure_init(); G_ASKED_THE_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_FOR_AN_ANSWER() -> Py { ensure_init(); G_ASKS_FOR_AN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_FOR_AN_ANSWER(v: Py) { ensure_init(); G_ASKS_FOR_AN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_ASPECT_ASKS() -> Py { ensure_init(); G_ASPECT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASPECT_ASKS(v: Py) { ensure_init(); G_ASPECT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASPECT_FILLERS() -> Py { ensure_init(); G_ASPECT_FILLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASPECT_FILLERS(v: Py) { ensure_init(); G_ASPECT_FILLERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASPECT_NOUNS() -> Py { ensure_init(); G_ASPECT_NOUNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASPECT_NOUNS(v: Py) { ensure_init(); G_ASPECT_NOUNS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BARE_JOB() -> Py { ensure_init(); G_A_BARE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BARE_JOB(v: Py) { ensure_init(); G_A_BARE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHOICE() -> Py { ensure_init(); G_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHOICE(v: Py) { ensure_init(); G_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DEADLINE_CLAUSE() -> Py { ensure_init(); G_A_DEADLINE_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DEADLINE_CLAUSE(v: Py) { ensure_init(); G_A_DEADLINE_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DOING_NAMED() -> Py { ensure_init(); G_A_DOING_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DOING_NAMED(v: Py) { ensure_init(); G_A_DOING_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FAULT() -> Py { ensure_init(); G_A_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FAULT(v: Py) { ensure_init(); G_A_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE_ON_THE_END() -> Py { ensure_init(); G_A_FIGURE_ON_THE_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE_ON_THE_END(v: Py) { ensure_init(); G_A_FIGURE_ON_THE_END.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FITTING() -> Py { ensure_init(); G_A_FITTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FITTING(v: Py) { ensure_init(); G_A_FITTING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GERUND() -> Py { ensure_init(); G_A_GERUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GERUND(v: Py) { ensure_init(); G_A_GERUND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GETTING() -> Py { ensure_init(); G_A_GETTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GETTING(v: Py) { ensure_init(); G_A_GETTING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOAL() -> Py { ensure_init(); G_A_GOAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOAL(v: Py) { ensure_init(); G_A_GOAL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_GOOD() -> Py { ensure_init(); G_A_HOW_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_GOOD(v: Py) { ensure_init(); G_A_HOW_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HOW_OFTEN() -> Py { ensure_init(); G_A_HOW_OFTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HOW_OFTEN(v: Py) { ensure_init(); G_A_HOW_OFTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LENGTH_STATED() -> Py { ensure_init(); G_A_LENGTH_STATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LENGTH_STATED(v: Py) { ensure_init(); G_A_LENGTH_STATED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE_AGAIN() -> Py { ensure_init(); G_A_MEASURE_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE_AGAIN(v: Py) { ensure_init(); G_A_MEASURE_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAME_ASK() -> Py { ensure_init(); G_A_NAME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAME_ASK(v: Py) { ensure_init(); G_A_NAME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PACE() -> Py { ensure_init(); G_A_PACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PACE(v: Py) { ensure_init(); G_A_PACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE() -> Py { ensure_init(); G_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE(v: Py) { ensure_init(); G_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLAN_TO_THE_DOING() -> Py { ensure_init(); G_A_PLAN_TO_THE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLAN_TO_THE_DOING(v: Py) { ensure_init(); G_A_PLAN_TO_THE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PURPOSE() -> Py { ensure_init(); G_A_PURPOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PURPOSE(v: Py) { ensure_init(); G_A_PURPOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THANK_YOU() -> Py { ensure_init(); G_A_THANK_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THANK_YOU(v: Py) { ensure_init(); G_A_THANK_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TRIP_PART() -> Py { ensure_init(); G_A_TRIP_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TRIP_PART(v: Py) { ensure_init(); G_A_TRIP_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VAGUE_JOB() -> Py { ensure_init(); G_A_VAGUE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VAGUE_JOB(v: Py) { ensure_init(); G_A_VAGUE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ANOTHER() -> Py { ensure_init(); G_BARE_ANOTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ANOTHER(v: Py) { ensure_init(); G_BARE_ANOTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_BASE_DOING() -> Py { ensure_init(); G_BASE_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BASE_DOING(v: Py) { ensure_init(); G_BASE_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_BEST_TIME_TO_DO() -> Py { ensure_init(); G_BEST_TIME_TO_DO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEST_TIME_TO_DO(v: Py) { ensure_init(); G_BEST_TIME_TO_DO.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_NOT_THE_THING() -> Py { ensure_init(); G_CARE_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_NOT_THE_THING(v: Py) { ensure_init(); G_CARE_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_CARE_VERBS() -> Py { ensure_init(); G_CARE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARE_VERBS(v: Py) { ensure_init(); G_CARE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTINUES() -> Py { ensure_init(); G_CONTINUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTINUES(v: Py) { ensure_init(); G_CONTINUES.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); G_COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); G_COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISTRESS() -> Py { ensure_init(); G_DISTRESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISTRESS(v: Py) { ensure_init(); G_DISTRESS.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING() -> Py { ensure_init(); G_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); G_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_DOING_BASE() -> Py { ensure_init(); G_DOING_BASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOING_BASE(v: Py) { ensure_init(); G_DOING_BASE.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_FILLER() -> Py { ensure_init(); G_FACT_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_FILLER(v: Py) { ensure_init(); G_FACT_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_MOST_WORDS() -> Py { ensure_init(); G_FACT_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_MOST_WORDS(v: Py) { ensure_init(); G_FACT_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_OPEN() -> Py { ensure_init(); G_FACT_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_OPEN(v: Py) { ensure_init(); G_FACT_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_FED_THINGS() -> Py { ensure_init(); G_FED_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FED_THINGS(v: Py) { ensure_init(); G_FED_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING_FILLER() -> Py { ensure_init(); G_FEELING_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING_FILLER(v: Py) { ensure_init(); G_FEELING_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_FEELING_SAID_AS_A_PHRASE() -> Py { ensure_init(); G_FEELING_SAID_AS_A_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEELING_SAID_AS_A_PHRASE(v: Py) { ensure_init(); G_FEELING_SAID_AS_A_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_GOAL_VERB() -> Py { ensure_init(); G_GOAL_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOAL_VERB(v: Py) { ensure_init(); G_GOAL_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_FRONT_OF_THE_NAME() -> Py { ensure_init(); G_IN_FRONT_OF_THE_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_FRONT_OF_THE_NAME(v: Py) { ensure_init(); G_IN_FRONT_OF_THE_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_PROGRESS() -> Py { ensure_init(); G_IN_PROGRESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_PROGRESS(v: Py) { ensure_init(); G_IN_PROGRESS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOINING() -> Py { ensure_init(); G_JOINING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOINING(v: Py) { ensure_init(); G_JOINING.with(|c| *c.borrow_mut() = v); }
pub fn g_JOURNEY() -> Py { ensure_init(); G_JOURNEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOURNEY(v: Py) { ensure_init(); G_JOURNEY.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_A_NAME() -> Py { ensure_init(); G_JUST_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_A_NAME(v: Py) { ensure_init(); G_JUST_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_FILLERS() -> Py { ensure_init(); G_LENGTH_FILLERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_FILLERS(v: Py) { ensure_init(); G_LENGTH_FILLERS.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_KEY_WORDS() -> Py { ensure_init(); G_LENGTH_KEY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_KEY_WORDS(v: Py) { ensure_init(); G_LENGTH_KEY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_NOT() -> Py { ensure_init(); G_LENGTH_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_NOT(v: Py) { ensure_init(); G_LENGTH_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_LENGTH_WORDS() -> Py { ensure_init(); G_LENGTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LENGTH_WORDS(v: Py) { ensure_init(); G_LENGTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIGHT_DOING() -> Py { ensure_init(); G_LIGHT_DOING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIGHT_DOING(v: Py) { ensure_init(); G_LIGHT_DOING.with(|c| *c.borrow_mut() = v); }
pub fn g_LIMIT() -> Py { ensure_init(); G_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIMIT(v: Py) { ensure_init(); G_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURE_ASK() -> Py { ensure_init(); G_MEASURE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURE_ASK(v: Py) { ensure_init(); G_MEASURE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURE_VERBS() -> Py { ensure_init(); G_MEASURE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURE_VERBS(v: Py) { ensure_init(); G_MEASURE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURE_WORDS() -> Py { ensure_init(); G_MEASURE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURE_WORDS(v: Py) { ensure_init(); G_MEASURE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS() -> Py { ensure_init(); G_MOST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS(v: Py) { ensure_init(); G_MOST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOST_WORDS_ASKED_ABOUT() -> Py { ensure_init(); G_MOST_WORDS_ASKED_ABOUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOST_WORDS_ASKED_ABOUT(v: Py) { ensure_init(); G_MOST_WORDS_ASKED_ABOUT.with(|c| *c.borrow_mut() = v); }
pub fn g_NAMED_A_THING() -> Py { ensure_init(); G_NAMED_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAMED_A_THING(v: Py) { ensure_init(); G_NAMED_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NAME_WORD() -> Py { ensure_init(); G_NAME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NAME_WORD(v: Py) { ensure_init(); G_NAME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_NAME() -> Py { ensure_init(); G_NOT_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME(v: Py) { ensure_init(); G_NOT_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_SHELF() -> Py { ensure_init(); G_NOT_A_SHELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_SHELF(v: Py) { ensure_init(); G_NOT_A_SHELF.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING_END() -> Py { ensure_init(); G_NOT_A_THING_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING_END(v: Py) { ensure_init(); G_NOT_A_THING_END.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_GOT() -> Py { ensure_init(); G_NOT_GOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_GOT(v: Py) { ensure_init(); G_NOT_GOT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_JOB() -> Py { ensure_init(); G_NOT_THE_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_JOB(v: Py) { ensure_init(); G_NOT_THE_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_SUBJECT() -> Py { ensure_init(); G_NOT_THE_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_SUBJECT(v: Py) { ensure_init(); G_NOT_THE_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); G_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); G_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_THING_ITSELF() -> Py { ensure_init(); G_NOT_THE_THING_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING_ITSELF(v: Py) { ensure_init(); G_NOT_THE_THING_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_THANKS() -> Py { ensure_init(); G_NO_THANKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_THANKS(v: Py) { ensure_init(); G_NO_THANKS.with(|c| *c.borrow_mut() = v); }
pub fn g_OUTCOME_ASK() -> Py { ensure_init(); G_OUTCOME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUTCOME_ASK(v: Py) { ensure_init(); G_OUTCOME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_LEAD() -> Py { ensure_init(); G_PART_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_LEAD(v: Py) { ensure_init(); G_PART_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_PART_WORDS() -> Py { ensure_init(); G_PART_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PART_WORDS(v: Py) { ensure_init(); G_PART_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PEOPLE_WORDS() -> Py { ensure_init(); G_PEOPLE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_WORDS(v: Py) { ensure_init(); G_PEOPLE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_OPEN() -> Py { ensure_init(); G_PERSON_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_OPEN(v: Py) { ensure_init(); G_PERSON_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_ARTICLES() -> Py { ensure_init(); G_PLACE_ARTICLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_ARTICLES(v: Py) { ensure_init(); G_PLACE_ARTICLES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_PAIRS() -> Py { ensure_init(); G_PLACE_PAIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_PAIRS(v: Py) { ensure_init(); G_PLACE_PAIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_PREPOSITIONS() -> Py { ensure_init(); G_PLACE_PREPOSITIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_PREPOSITIONS(v: Py) { ensure_init(); G_PLACE_PREPOSITIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_WORDS() -> Py { ensure_init(); G_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORDS(v: Py) { ensure_init(); G_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTED_AT() -> Py { ensure_init(); G_POINTED_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTED_AT(v: Py) { ensure_init(); G_POINTED_AT.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_AT_A_THING() -> Py { ensure_init(); G_POINTS_AT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT_A_THING(v: Py) { ensure_init(); G_POINTS_AT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_PREP() -> Py { ensure_init(); G_PREP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREP(v: Py) { ensure_init(); G_PREP.with(|c| *c.borrow_mut() = v); }
pub fn g_PROGRESS_ASK() -> Py { ensure_init(); G_PROGRESS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROGRESS_ASK(v: Py) { ensure_init(); G_PROGRESS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PROGRESS_WORDS() -> Py { ensure_init(); G_PROGRESS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROGRESS_WORDS(v: Py) { ensure_init(); G_PROGRESS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_QUANT() -> Py { ensure_init(); G_QUANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUANT(v: Py) { ensure_init(); G_QUANT.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_NOT_A_FOOD() -> Py { ensure_init(); G_SAID_NOT_A_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_NOT_A_FOOD(v: Py) { ensure_init(); G_SAID_NOT_A_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_SPAN_WORDS() -> Py { ensure_init(); G_SPAN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPAN_WORDS(v: Py) { ensure_init(); G_SPAN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_WORDS() -> Py { ensure_init(); G_STATE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_WORDS(v: Py) { ensure_init(); G_STATE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_BACK() -> Py { ensure_init(); G_TAKES_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_BACK(v: Py) { ensure_init(); G_TAKES_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKE_BACK_LEAD() -> Py { ensure_init(); G_TAKE_BACK_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKE_BACK_LEAD(v: Py) { ensure_init(); G_TAKE_BACK_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKE_BACK_WHOLE() -> Py { ensure_init(); G_TAKE_BACK_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKE_BACK_WHOLE(v: Py) { ensure_init(); G_TAKE_BACK_WHOLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASKING() -> Py { ensure_init(); G_THE_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASKING(v: Py) { ensure_init(); G_THE_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASKING_TAIL() -> Py { ensure_init(); G_THE_ASKING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASKING_TAIL(v: Py) { ensure_init(); G_THE_ASKING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_ASKING_VERB() -> Py { ensure_init(); G_THE_ASKING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_ASKING_VERB(v: Py) { ensure_init(); G_THE_ASKING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_THE_LENGTH_FOR_IT() -> Py { ensure_init(); G_THE_LENGTH_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THE_LENGTH_FOR_IT(v: Py) { ensure_init(); G_THE_LENGTH_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_TIMES_ONLY() -> Py { ensure_init(); G_TIMES_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIMES_ONLY(v: Py) { ensure_init(); G_TIMES_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_VERBS() -> Py { ensure_init(); G_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERBS(v: Py) { ensure_init(); G_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_FRAME() -> Py { ensure_init(); G_VIEW_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_FRAME(v: Py) { ensure_init(); G_VIEW_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_MANNER() -> Py { ensure_init(); G_VIEW_MANNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_MANNER(v: Py) { ensure_init(); G_VIEW_MANNER.with(|c| *c.borrow_mut() = v); }
pub fn g_VIEW_POSSESSIVE() -> Py { ensure_init(); G_VIEW_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VIEW_POSSESSIVE(v: Py) { ensure_init(); G_VIEW_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_WANTS_A_NEW_ONE() -> Py { ensure_init(); G_WANTS_A_NEW_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WANTS_A_NEW_ONE(v: Py) { ensure_init(); G_WANTS_A_NEW_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_IT_STANDS() -> Py { ensure_init(); G_WHERE_IT_STANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_IT_STANDS(v: Py) { ensure_init(); G_WHERE_IT_STANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_X_IS() -> Py { ensure_init(); G_WHO_X_IS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_X_IS(v: Py) { ensure_init(); G_WHO_X_IS.with(|c| *c.borrow_mut() = v); }
pub fn g__AND_AT_THE_START() -> Py { ensure_init(); G__AND_AT_THE_START.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AND_AT_THE_START(v: Py) { ensure_init(); G__AND_AT_THE_START.with(|c| *c.borrow_mut() = v); }
pub fn g__ASK_FILLER() -> Py { ensure_init(); G__ASK_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASK_FILLER(v: Py) { ensure_init(); G__ASK_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CONNECTIVE() -> Py { ensure_init(); G__A_CONNECTIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CONNECTIVE(v: Py) { ensure_init(); G__A_CONNECTIVE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_IN_A_REPLY() -> Py { ensure_init(); G__A_TIME_IN_A_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_IN_A_REPLY(v: Py) { ensure_init(); G__A_TIME_IN_A_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_WORD() -> Py { ensure_init(); G__A_TIME_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_WORD(v: Py) { ensure_init(); G__A_TIME_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__GOING_AWAY() -> Py { ensure_init(); G__GOING_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GOING_AWAY(v: Py) { ensure_init(); G__GOING_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__LIVING_THERE() -> Py { ensure_init(); G__LIVING_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LIVING_THERE(v: Py) { ensure_init(); G__LIVING_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_PRONOUN() -> Py { ensure_init(); G__NOT_A_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_PRONOUN(v: Py) { ensure_init(); G__NOT_A_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_PRACTICE() -> Py { ensure_init(); G__NOT_THE_PRACTICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_PRACTICE(v: Py) { ensure_init(); G__NOT_THE_PRACTICE.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_LIFE() -> Py { ensure_init(); G__THE_LIFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_LIFE(v: Py) { ensure_init(); G__THE_LIFE.with(|c| *c.borrow_mut() = v); }
pub fn g_is_ack() -> Py { ensure_init(); G_is_ack.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_is_ack(v: Py) { ensure_init(); G_is_ack.with(|c| *c.borrow_mut() = v); }
pub fn g_reads_as_bad_news() -> Py { ensure_init(); G_reads_as_bad_news.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_reads_as_bad_news(v: Py) { ensure_init(); G_reads_as_bad_news.with(|c| *c.borrow_mut() = v); }
pub fn g_the_carried_topic() -> Py { ensure_init(); G_the_carried_topic.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_carried_topic(v: Py) { ensure_init(); G_the_carried_topic.with(|c| *c.borrow_mut() = v); }
pub fn g_the_subject() -> Py { ensure_init(); G_the_subject.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_subject(v: Py) { ensure_init(); G_the_subject.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A follow-up that names one thing takes the ask before it with it.\n\n\"my mum is coming to stay\" / \"what should i cook\" / \"and a pudding\" is a person carrying on the\nsame request about a new thing: what they want is pudding ideas. The knowledge base has the\nanswer (\"ideas for pudding\" gives the crumble), but the line on its own names a pudding and asks\nnothing, so nothing in it is answerable and the turn fell to the small talk: \"Thanks for telling\nme - I'm here if you want to dig into any of that\", which is the one reply a follow-up must not\nget.\n\nThis reads the line as the ask it carries: the thing it names, put back into the request the\nprevious line made. The rule is narrow on purpose:\n\n* the line must be a thing and nothing else - an optional \"and\"/\"or\"/\"also\"/\"plus\" or\n  \"what about\"/\"how about\" in front, then a short noun phrase, with no verb, no question of its\n  own and no word pointing back at the last subject (\"and what about it\", \"and how long\");\n* the line just before it must have asked for suggestions (\"what should i cook\", \"what can i\n  make\", \"any ideas\"), because that is the ask a thing can be carried into;\n* the phrase it hands back is \"ideas for <thing>\", and the caller uses it only when the line\n  itself has no answer of its own - so a line that answers today still answers tomorrow.\n");
    s_the_carried_topic(py_func("carried_topic", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "carried_topic", args)));
    s_is_ack(py_func("is_ack", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "is_ack", args)));
    s_the_subject(py_func("subject", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "subject", args)));
    s_reads_as_bad_news(py_func("reads_as_bad_news", |args: &[Py]| pyrt::xrefs::xcall("compose.badnews", "reads_as_bad_news", args)));
    s_CONTINUES(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|or|plus|also|but|then)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|or|also|plus|ok|okay|then|so)\\b[\\s,]*)*(?:(?:what|how)\\s+about\\s+|and\\s+what\\s+about\\s+|what\\s+of\\s+)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_QUANT(stdlib_call("re", "compile", &[py_str("^\\s*(?:a|an|the|some|any|another|one|two|three|more|other)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:what|how|where|when|which|who|why)(?:'s|s)?\\b|(?:can|could|should|do|does|did|is|are|was|were|have|has|will|would|shall|may|might)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_BACK(call_builtin("frozenset", &[mcall(&py_str("it its it's them they that this these those there one ones\nhim her his hers their theirs he she"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKS_FOR_AN_ANSWER(call_builtin("frozenset", &[mcall(&py_str("tip tips advice suggestion suggestions pointer pointers\nrecommendation recommendations trick tricks guidance idea ideas hint hints"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERBS(call_builtin("frozenset", &[mcall(&py_str("is are was were am be been being have has had do does did done\nwill would shall should can could may might must\ngo going goes went come coming comes came get getting got make making made\ntake taking took need needs needed want wants wanted like likes liked\nknow knows knew think thinks thought feel feels felt hurt hurts hurting\neat eats eating ate drink drinks drinking work works working play plays playing\nhappen happens happened start starts started stop stops stopped try tries tried\ngive gives gave put puts keep keeps kept buy buys bought sell sells sold\nlive lives lived move moves moved stay stays stayed visit visits visited\nrain rains raining snow snows snowing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKES_BACK(call_builtin("frozenset", &[mcall(&py_str("forget forgets forgot forgetting drop drops dropped dropping\nscratch scratches scratched strike strikes struck remove removes removed delete deletes deleted\ncancel cancels cancelled canceling bin bins binned ignore ignores ignored discard discards\ndiscarded"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED_FOR_IDEAS(stdlib_call("re", "compile", &[py_str("\\b(?:what|which)\\s+(?:should|could|can|shall|would)\\s+(?:i|we)\\s+(?:cook|make|do|have|eat|get|buy|watch|read|wear|give|bring|take|serve|bake|plan)\\b|\\b(?:what|which)\\s+(?:do|did)\\s+(?:i|we)\\s+(?:cook|make|have|eat)\\b|\\bwhat\\s+(?:can|could)\\s+(?:i|we)\\s+(?:cook|make|have|eat|do|get)\\b|\\bany\\s+(?:ideas?|suggestions?)\\b|\\bideas?\\s+for\\b|\\bwhat\\s+would\\s+you\\s+(?:suggest|recommend|cook|make|do)\\b|\\bsuggest(?:ions?)?\\s+(?:for|something)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS(Py::Int(4));
    s_SAID_NOT_A_FOOD(call_builtin("frozenset", &[mcall(&py_str("i me we us you he she they it its said says say saying meant\nmeans mean asking asked told tell telling sorry actually really"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THANK_YOU(call_builtin("frozenset", &[mcall(&py_str("thanks thank thankyou thank-you thx ta cheers mate love hun"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PREP(call_builtin("frozenset", &[py_tuple(vec![py_str("of"), py_str("in"), py_str("on"), py_str("at"), py_str("for"), py_str("from"), py_str("with"), py_str("about"), py_str("to"), py_str("by")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKE_BACK_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|or|so|ok|okay|now|then)\\b[\\s,]*)*(?:(?:please|just|actually)\\s+)*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKE_BACK_WHOLE(py_tuple(vec![py_str("take that back"), py_str("takes that back"), py_str("never mind"), py_str("scratch that"), py_str("forget it"), py_str("forget that"), py_str("forget about it"), py_str("drop it"), py_str("drop that")]));
    s_THE_ASKING(py_tuple(vec![stdlib_call("re", "compile", &[py_str("^\\s*(?:and|also|so|ok|okay|but|then|well|oh)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:can|could|would)\\s+you\\s+(?:please\\s+)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:please\\s+)?(?:tell|remind|show)\\s+me\\s+(?:again\\s+)?(?:about|of|who|what)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:tell|remind)\\s+me\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:anything|something|more|a\\s+bit)\\s+about\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:what|how)\\s+about\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*same\\s+(?:for|as)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*what\\s+of\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:do|did)\\s+you\\s+(?:remember|know|recall)\\s+(?:about\\s+|the\\s+)?"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*(?:what\\s+is|what's|whats|what)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\s*who(?:'s|s| is| was)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_THE_ASKING_TAIL(stdlib_call("re", "compile", &[py_str("\\s+(?:again|then|though|please|now|too|already|anyway|honestly|is|was|are|were)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_X_IS(stdlib_call("re", "compile", &[py_str("^(?:(?:and|also|so|ok|okay|then)\\s+)?who\\s+(?P<thing>.+?)\\s+(?:is|was)\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MOST_WORDS_ASKED_ABOUT(Py::Int(3));
    s_POINTED_AT(call_builtin("frozenset", &[py_tuple(vec![py_str("him"), py_str("her"), py_str("them"), py_str("they"), py_str("that one"), py_str("this one"), py_str("those two"), py_str("the two of them"), py_str("both of them"), py_str("the pair of them")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_ASKING_VERB(stdlib_call("re", "compile", &[py_str("^\\s*(?:is|are|was|were|am|be|been|being|do|does|did|should|could|would|will|shall|can|may|might|must|have|has|had)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:called|named|names?)\\b|name'?s"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_FRONT_OF_THE_NAME(call_builtin("frozenset", &[mcall(&py_str("is not was isnt aint it its really actually"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_THE_NAME(call_builtin("frozenset", &[mcall(&py_str("instead now then actually please ok okay"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s|s| is| are| was)?\\s*(?:it|its|he|she|they|them|the|your|his|her|their)?\\s*(?:called|named)\\b|\\bwhat(?:'s|s| is)?\\s+(?:its|his|her|their|your)\\s+names?\\b|\\bwhat\\s+do\\s+(?:you|they)\\s+call\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_JUST_A_NAME(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:its|it's|hes|he's|shes|she's|theyre|they're|just|actually|his|her)\\s+)*(?P<name>[A-Za-z][A-Za-z'-]{1,20})(?:\\s+(?:i think|we think|for now|so far|anyway|probably))*\\W*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME(call_builtin("frozenset", &[mcall(&py_str("\nno yes yeah nah not none nothing unknown undecided idea clue sure maybe dont dontnot doesnt\nstill deciding thinking undecided wait later him her them they it he she that this one\npuppy dog cat kitten rabbit hamster boy girl friend mate son daughter baby kid\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SPAN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("in"), py_str("for"), py_str("since"), py_str("over"), py_str("all")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_GOT(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|but|also|plus|oh|sorry|actually)\\b[\\s,]*)?(?:(?:(?:i|we)\\s+)?(?:dont|don't|do not|didnt|didn't|did not|havent|haven't|have not|hasnt|hasn't|has not)\\s+(?:(?P<holding>got|get|have|had|buy|bought)\\s+)?|(?:there\\s+is\\s+|theres\\s+|there's\\s+|i\\s+have\\s+|we\\s+have\\s+)?no\\s+)(?:any\\s+|a\\s+|the\\s+|some\\s+)?(?P<food>[a-z][a-z '-]{1,24}?)\\s*(?:left|in|at all)?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_SHELF(call_builtin("frozenset", &[mcall(&py_str("\neat eats eating ate eaten like likes liked liking love loves loved fancy fancies enjoy enjoys\nenjoyed want wants wanted need needs needed use uses used drink drinks drinking stomach stomachs\ntouch touches much many"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PART_WORDS(Py::Int(4));
    s_PART_LEAD(stdlib_call("re", "compile", &[py_str("^\\s*(?:the|its|it's|their|his|her|a|an|any|more|some|this|that|these|those)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MEASURE_AGAIN(stdlib_call("re", "compile", &[py_str("^\\s*(?:and|or|also|plus|what\\s+about|how\\s+about)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FITTING(stdlib_call("re", "compile", &[py_str("\\b(?:fit|fits|fitting|fitted|swap|swaps|swapping|installs?|installed|installation|replac(?:e|es|ed|ing)|replacement)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WANTS_A_NEW_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|the)\\s+new\\s+(?:one|boiler|machine|shower|radiator|oven|fridge|dishwasher|washer|dryer|cooker|hob|combi|heating|system)\\b|\\bnew\\s+one\\b|\\breplac\\w+\\b|\\b(?:fit|fits|fitting|fitted|swap|swaps|swapping|install|installs|installed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKED_THE_MEASURE(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__THE_LIFE(stdlib_call("re", "compile", &[py_str("\\b(?:last|lasts|lasting|lasted|years?|old|lifespan|life\\s*span|expectancy|cost|costs|price|much)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_A_THING(stdlib_call("re", "compile", &[py_str("\\b(?:my|our|the|his|her|this)\\s+((?:[a-z][a-z-]*\\s+){0,2}[a-z][a-z-]*)\\s+(?:is|was|are|were|keeps|kept|has|had|stopped|broke|breaks|failed|went|wont|won't|isnt|doesnt|doesn't|needs|makes|sounds|smells)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_THING_ITSELF(call_builtin("frozenset", &[mcall(&py_str("\nbroken broke dead leaking leak faulty failed failing cracked blocked jammed stuck gone kaput\ndodgy unreliable stopped working old new bad worse better fine ok okay good rubbish\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GOAL(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:so|ok|okay|and|also|well|right|yeah|hi|hello)[, ]+)*(?:i'?m|im|i am|we'?re|we are|i'?ve been|ive been|i have been)\\s+(?P<how>trying|aiming|hoping|learning)\\s+(?P<what>(?:to\\s+|for\\s+)?[a-z0-9][a-z0-9 '\\-]{2,60}?)\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_GOAL_VERB(py_dict(vec![(py_str("trying"), py_str("")), (py_str("aiming"), py_str("")), (py_str("hoping"), py_str("")), (py_str("learning"), py_str("learn to "))]));
    s_OUTCOME_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\s+(?:until|till|before)\\s+(?:i'?m|i\\s+am)\\s+(?P<state>[a-z][a-z '\\-]{2,30})\\s*[.!?]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AIM_VERB(call_builtin("frozenset", &[mcall(&py_str("get getting be been become becoming feel feeling look looking stay"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DEADLINE_CLAUSE(stdlib_call("re", "compile", &[py_str("\\s+(?:before|by|until|till|ahead\\s+of|in\\s+time\\s+for)\\s+.*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nhow long soon many much weeks week months month days day years year hours hour will would does do\ndid it that this take takes taken be is till until before from now to see notice seeing a an the\ndifference differences result results normally usually roughly about actually and so but ok okay\nthen expect should could might start any go going i ill we\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DISTRESS(call_builtin("frozenset", &[mcall(&py_str("\ncry crying cried sob sobbing weep weeping tears sad sadder unhappiest unhappy depressed\ndepression anxious anxiety anxiousness panic panicking grieve grieving grief bereaved bereavement\ndie dying died death dead funeral bury mourning mourn miss missing hurt hurting pain ache aching\nsuicide self lonely alone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTIVITY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ntraining practise practice practices lesson lessons session sessions class classes study studying\nrevision revising prep preparation coaching drills workout workouts\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_KEY_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nlong soon weeks week months month years year hours hour days day minutes minute\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LENGTH_STATED(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,4}|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty)\\s+(?:to\\s+\\w+\\s+)?(?:seconds?|minutes?|hours?|days?|weeks?|months?|years?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PACE(stdlib_call("re", "compile", &[py_str("\\b(?:a|an|one|two|three|four|five|half\\s+a|\\d[\\d,]*(?:\\s*to\\s*\\d[\\d,]*)?)\\s+[a-z][\\w-]*(?:\\s+[a-z][\\w-]*){0,2}?\\s+(?:a|per|each)\\s+(?:week|month|day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_THANKS(stdlib_call("re", "compile", &[py_str("^\\s*(?:no|nope|nah|not)\\b[\\s,]*(?:th(?:anks|ank\\s+you|anks\\s+you|x)|ta|cheers|mate|love)\\b[\\s,]*(?:(?:though|then|but|anyway|really|at\\s+all|so|ok|okay|very\\s+much|any)\\b[\\s,]*)*[.!]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_ANOTHER(stdlib_call("re", "compile", &[py_str("^\\s*(?:(?:and|or|so|ok|okay|right|well|but|then|now|hmm|ha)\\b[\\s,]*)*(?:(?:what|anything|something)\\s+else|any\\s+others?|another\\s+(?:one|idea|option|film|book|recipe|suggestion|pick|show))(?:\\s+(?:(?:is|are)\\s+on|on|works?|then|tonight|now|please|for\\s+me|(?:would\\s+)?you(?:'d)?\\s*(?:would\\s+)?(?:suggest|recommend|have|got)|do\\s+you\\s+have))*\\s*[.!?]*\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VIEW_FRAME(call_builtin("frozenset", &[mcall(&py_str("view opinion take takes thinking thoughts tuppence"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VIEW_MANNER(call_builtin("frozenset", &[mcall(&py_str("honest real quick true actual blunt personal own full proper same\nplain straight"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VIEW_POSSESSIVE(call_builtin("frozenset", &[mcall(&py_str("your my our the a an and also plus too then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the just only bit little so really very quite too still today tonight now thanks thank\nthankyou you im i m am is are was were be been feeling feel feels felt and but yeah well of\nabout kind sort\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FEELING_SAID_AS_A_PHRASE(py_tuple(vec![py_str("wiped out"), py_str("done in"), py_str("dead on my feet"), py_str("run down"), py_str("no energy"), py_str("knackered out")]));
    s_MEASURE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+(?:long|much|many|often|big|far|heavy|hot|warm|expensive)|what\\s+temperature)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PROGRESS_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+long\\b(?=[^.!?]*\\b(?:until|till|before)\\b)|\\bhow\\s+long\\s+(?:does\\s+it\\s+take\\s+)?(?:to\\s+)?(?:get|be|become)\\s+(?:good|any\\s+good|decent|ok|okay|fluent|better)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GERUND(stdlib_call("re", "compile", &[py_str("^[a-z]{4,}ing$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DOING_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:learn|learning|learnt)\\s+to\\s+(?P<doing>[a-z]{3,})\\b|\\b(?:start|starting|started|try|trying|tried|practise|practising|practice|practicing|how\\s+to)\\s+(?P<also>[a-z]{3,})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE_ON_THE_END(stdlib_call("re", "compile", &[py_str("^\\d+[a-z]{0,2}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLAN_TO_THE_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|wanting|wanted|hope|hoping|hoped|plan|planning|planned|aim|aiming|aimed|intend|intending|learn|learning|learnt|try|trying|tried|would\\s+like|would\\s+love)\\s+to\\s+%s\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOW_GOOD(stdlib_call("re", "compile", &[py_str("\\b(?:good|better|best|fluent|confident|comfortable|competent|proficient|capable|skilled|decent|mastered|able\\s+to|any\\s+good)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PROGRESS_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ngood any decent ok okay fine fluent confident comfortable competent proficient proficiently\nbetter well able capable mastered skilled basics basic conversational conversation\nplay playing speak speaking read reading write writing hold manage manage use using\nget getting become becoming at it to be can could will would until till before by\nim i'm i am then actually really genuinely properly roughly about\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURE_VERBS(call_builtin("frozenset", &[mcall(&py_str("\ntake takes taking took last lasts lasting cost costs costing\nbe being been get gets getting got happen happens happening"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nhow what long much many often big far heavy hot warm expensive temperature time times\nsecond seconds minute minutes hour hours day days week weeks month months year years\ncost costs costing price prices charge charges worth pay pays spend spends need needs\na an the per\ni me my mine we our us you your should would will shall can could do does did be is are was were\nhas have had having\nit its them they that this these those there one ones\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FAULT(call_builtin("frozenset", &[mcall(&py_str("\nbroken broke bust busted stopped dead leaking leaky faulty failed failing cracked blocked\njammed stuck gone kaput dodgy unreliable"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\nworried worry worries worrying anxious anxiety scared afraid nervous stressed stress upset angry\nannoyed sad unhappy happy excited tired exhausted confused unsure lost stuck glad pleased\nthinking thought considering planning hoping wanting looking interested keen trying dreaming\nweighing wondering\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GETTING(call_builtin("frozenset", &[mcall(&py_str("\nbuy buys buying get gets getting purchase purchases purchasing pick picks picking choose chooses\nchoosing order orders ordering\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASPECT_ASKS(py_tuple(vec![py_tuple(vec![call_builtin("frozenset", &[mcall(&py_str("cost costs price prices expense expenses upkeep costing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), py_str("how much does the {thing} cost")]), py_tuple(vec![call_builtin("frozenset", &[mcall(&py_str("food feeding feed diet"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), py_str("what do i feed the {thing}")])]));
    s_FED_THINGS(call_builtin("frozenset", &[mcall(&py_str("\ndog dogs puppy puppies cat cats kitten kittens rabbit rabbits bunny bunnies guinea pig pigs hamster\nhamsters gerbil gerbils rat rats mouse mice ferret ferrets parrot parrots budgie budgies bird birds\nfish goldfish chicken chickens hen hens cockerel horse horses pony ponies tortoise tortoises\nsnake snakes lizard lizards gecko geckos tortoise degu degus chinchilla chinchillas\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASPECT_FILLERS(call_builtin("frozenset", &[mcall(&py_str("\nand so or but also then ok okay well what about how the a an any more of\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASPECT_NOUNS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.extend(py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASPECT_ASKS()) { let __unpk3418 = py_unpack(&__item.clone(), 2, None); let mut nouns: Py = __unpk3418[0].clone(); let mut _u: Py = __unpk3418[1].clone(); __out.push(nouns.clone()); } py_list(__out) })() })); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&call_builtin("frozenset", &[]).unwrap_or_else(|e| exc::raise(e)), "union", args))(&__args) });
    s_IN_PROGRESS(stdlib_call("re", "compile", &[py_str("\\b(?:im|i'm|i am|we're|we are|hes|he's|shes|she's|theyre|they're|ive|i've)\\b(?:\\s+(?:still|also|just|been|busy|now))*\\s+((?:[a-z']+\\s+){0,6}?[a-z]{2,}ing\\b[^.?!,]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_LENGTH_FOR_IT(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|so\\s+|ok\\s+|okay\\s+|then\\s+|also\\s+|roughly\\s+|about\\s+)?(?:how\\s+long|how\\s+much\\s+time|how\\s+many\\s+(?:hours|minutes|days|weeks|months))[^.?!]{0,24}$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:until|till|before|after|ago|since|between|every|each|\\d+\\s*(?:h|hr|hrs|hours?|minutes?|days?|weeks?))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LENGTH_FILLERS(call_builtin("frozenset", &[mcall(&py_str("\nsupposed going go went spent took long should will would does do did i you we they it that this\nis are be last for about of\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JOURNEY(stdlib_call("re", "compile", &[py_str("^(?:going|go|goes|heading|off|flying|travelling|traveling|visiting|moving|coming|leaving|driving|walking|cycling)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BEST_TIME_TO_DO(stdlib_call("re", "compile", &[py_str("^(?:when|what)(?:'s|s| is| was)?\\s+(?:the\\s+)?(?:best|right|good|ideal|safest|cheapest|easiest)\\s+(?:time|month|week|season|day|part)\\s+(?:to|for)\\s+(?P<rest>.+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_AT_A_THING(call_builtin("frozenset", &[mcall(&py_str("it its them that this these those one ones"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_VAGUE_JOB(call_builtin("frozenset", &[mcall(&py_str("do doing get getting go going make making start starting sort sorting\n    prep prepare preparing deal dealing have"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BARE_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:should|could|would|can|shall|will|to)\\s+(?:i\\s+|we\\s+|you\\s+)?([a-z]{3,})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_JOB(call_builtin("frozenset", &[mcall(&py_str("i me my we you your the a an and but so if of for in on at it its\n    that this them they he she there here not no yes please thanks need needs want wants have\n    has had do did does am is are was were be been got get going want trying"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__AND_AT_THE_START(stdlib_call("re", "compile", &[py_str("^(?:and|then|also|so)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\noven hob microwave grill grillpan airfryer fryer pan saucepan pot steamer slowcooker cooker kettle\ntoaster barbecue bbq fridge freezer dishwasher washingmachine dryer radiator sink bath\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_PAIRS(call_builtin("frozenset", &[py_set(vec![py_tuple(vec![py_str("slow"), py_str("cooker")]), py_tuple(vec![py_str("air"), py_str("fryer")]), py_tuple(vec![py_str("grill"), py_str("pan")]), py_tuple(vec![py_str("pressure"), py_str("cooker")]), py_tuple(vec![py_str("rice"), py_str("cooker")]), py_tuple(vec![py_str("tumble"), py_str("dryer")])])]).unwrap_or_else(|e| exc::raise(e)));
    s_LIGHT_DOING(call_builtin("frozenset", &[mcall(&py_str("go goes going do does doing get take use try keep start"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_PREPOSITIONS(call_builtin("frozenset", &[mcall(&py_str("in on at into under over"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLACE_ARTICLES(call_builtin("frozenset", &[mcall(&py_str("the a an my our your their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("cook"), py_str("bake"), py_str("roast"), py_str("boil"), py_str("fry"), py_str("grill"), py_str("steam"), py_str("make"), py_str("prepare"), py_str("reheat"), py_str("heat"), py_str("store"), py_str("keep"), py_str("freeze"), py_str("defrost"), py_str("warm")]) }
    s_BASE_DOING(__data1());
    s_DOING_BASE({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g_BASE_DOING()) { let mut base: Py = __item.clone(); for __item in py_iter_or_raise(&_the_forms(&[base.clone()])) { let mut form: Py = __item.clone(); __out.push((form.clone(), base.clone())); } } py_dict(__out) })() });
    s_DOING(call_builtin("frozenset", &[g_DOING_BASE()]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_WORDS(call_builtin("frozenset", &[mcall(&py_str("\nstill now already again left gone flat empty full broken dead stuck worse better okay ok\nsmells smell stinks stinking works working not never\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+(?:\\.\\d+)?\\s*(?:%|percent|hours?|minutes?|days?|weeks?|litres?|liters?|kilos?|kg|metres?|miles?|pounds?))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:at|in|on|under|behind)\\s+(?:work|home|the office|the car|the garage|the kitchen|the shed|the loft|the shop|the hotel|my bag|my pocket|my desk)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_VERBS(call_builtin("frozenset", &[mcall(&py_str("\nfeed feeds feeding fed water waters watering watered repot repotting repotted prune prunes\npruning pruned deadhead deadheading deadheaded stake staking staked mulch mulching mulched\nmist misting misted thin thinning thinned aerate aerating\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HOW_OFTEN(stdlib_call("re", "compile", &[py_str("\\b(?:every|each|once|twice|three times|a|per|daily|weekly|fortnightly|monthly|yearly|\\\nannually|regularly|sometimes|usually|often|january|february|march|april|may|june|july|august|\\\nseptember|october|november|december|spring|summer|autumn|winter)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_NOT_THE_THING(call_builtin("frozenset", &[mcall(&py_str("\nbaby babies child children kid kids toddler toddlers mum mom dad daddy mummy son sons daughter\nfamily dog dogs cat cats puppy puppies kitten kittens fish rabbit hamster guinea bird birds\nparrot chickens hen hens horse horses me myself us everyone everybody\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_IT_STANDS(call_builtin("frozenset", &[mcall(&py_str("\nwindowsill window sill shelf table bench desk worktop counter sideboard floor carpet rug patio\nbalcony garden greenhouse conservatory kitchen bathroom bedroom lounge hall hallway garage shed\nattic loft cellar porch utility laundry nursery study room house home flat pot pots tub tray\nbasket fridge freezer oven hob grill cupboard cupboard drawer desk\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_THE_PRACTICE(op_bitor(&call_builtin("frozenset", &[mcall(&py_str("\nevery each once twice three times week weeks day days month months year years morning\nmornings evening evenings night nights daily weekly fortnightly monthly regularly sometimes\nusually often spring summer autumn winter january february march april may june july august\nseptember october november december about around roughly usually also still just really\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_CARE_VERBS()));
    s_PERSON_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\\W+)*(?:i|im|i'?m|i'?ve|ive|i'?d|id|we|we'?re|were|we'?ve|me|us|she|he|they|there|it|its|it'?s|they're)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_OPEN(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|nothing|just)\\W+)*(?:for|with|on|in|at|by|near|from|about|only|mostly|usually|mainly|every|each)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_FILLER(call_builtin("frozenset", &[mcall(&py_str("the and but for with about only just even also very really quite\n    still now then than that this these those there their them they she her his him its it\n    was were been being have has had does did doing get got going make made more most some any\n    im ive id you your my our me us we i am is are not no yes like want need know think"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TIMES_ONLY(call_builtin("frozenset", &[mcall(&py_str("morning mornings evening evenings night nights afternoon afternoons\n    weekend weekends weekday weekdays day days week weeks month months winter winters summer\n    summers autumn spring springtime sometimes always usually mostly mainly normally lately\n    recently today tonight tomorrow"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_JOINING(call_builtin("frozenset", &[mcall(&py_str("before after until till while when whenever since once if because cause\n    though although whereas unless during without within between through across along around\n    behind below beside beyond onto upon than"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_MOST_WORDS(Py::Int(14));
    s_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\bwhether\\b|\\bwhat\\s+to\\b|\\bhow\\s+to\\b|\\bwhat\\s+i\\s+should\\b|\\bif\\s+i\\s+should\\b|\\bnot\\s+sure\\s+(?:if|whether|what|how)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNT_WORDS(call_builtin("frozenset", &[mcall(&py_str("one two three four five six seven eight nine ten eleven twelve\n    couple few several many lot lots"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PEOPLE_WORDS(call_builtin("frozenset", &[mcall(&py_str("kid kids child children boy boys girl girls son sons daughter\n    daughters baby babies toddler toddlers teen teens teenager teenagers grandchild\n    grandchildren person people adult adults flatmate flatmates housemate housemates"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_SUBJECT(call_builtin("frozenset", &[mcall(&py_str("buy buying bought choose choosing chose pick picking want wanted\n    wanted need needed get getting got looking look looked find finding"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_PRONOUN(call_builtin("frozenset", &[mcall(&py_str("\nhe she him her his hers them they their theirs himself herself themselves\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CONNECTIVE(call_builtin("frozenset", &[mcall(&py_str("and so but also then ok okay please"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TIME_IN_A_REPLY(stdlib_call("re", "compile", &[py_str("\\b(?:\\d+|an?|one|two|three|four|five|ten|fifteen|twenty|thirty|forty|forty-five|fifty|sixty|ninety|half an|a couple of|a few)\\s*(?:second|seconds|minute|minutes|hour|hours|day|days|week|weeks|month|months|year|years)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PURPOSE(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|this|they|them|he|she)\\s+(?:would\\s+be|is|are|was|were)\\s+(?:for|to\\s+be)\\b|\\b(?:for|with)\\s+(?:the|my|our|his|her|their|a|an)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_THING_END(call_builtin("frozenset", &[mcall(&py_str("\nnot never no yes ok okay thanks thank please too also very really much many more less\ngood bad fine same sure well now then again already still just today tomorrow yesterday\nweek month year morning evening night day weekend ever yet either\none two three four five six seven eight nine ten eleven twelve\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TRIP_PART(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:stay|staying|hotel|hotels|accommodation|sleep|sleeping|hostel|hostels)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("where should i stay in {place}")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:eat|eats|eating|food|restaurant|restaurants|dinner|lunch|breakfast|drink|drinks|bar|bars|cafe|cafes|coffee|pizza|gelato)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("where should i eat in {place}")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:see|sights|sightseeing|visit|visiting|museum|museums|gallery|galleries|ruins|tour|tours)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("what should i see in {place}")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:days?|long enough|enough time|how long)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("how many days in {place}")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:when|month|months|season|weather|climate|temperature|january|february|april|june|july|august|september|october|november|december|spring|summer|autumn|winter)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("when should i go to {place}")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:expensive|pricey|cost|costs|costing|afford|budget|spend|how much)\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("is {place} expensive")])]));
    s__ASK_FILLER(call_builtin("frozenset", &[mcall(&py_str("\na an the and but or so also then too now ok okay please about for to in on at of it that this\ni we you me us my our your there here any some much many more most best good worth should could\nwould can will do does did is are was were be been get got have has had going go stay stay\nsome anywhere somewhere place places area areas neighbourhood neighborhood part parts spot spots\nnice pleasant decent good best spot\nwhat where which how when why who whats like tell me info advice recommend recommends\nrecommendation recommendations suggest suggestion suggestions\nneed needs needed want wants wanted enough require required take takes give gives\none two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen\nsixteen seventeen eighteen nineteen twenty couple few several\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__GOING_AWAY(stdlib_call("re", "compile", &[py_str("\\b(?:going|go|goes|off|heading|trip|trips|holiday|holidays|travel|travelling|traveling|visit|visiting|fly|flying|flight|flights|weekend|weekends|break|away|abroad|booked|booking|planning|thinking about|considering|looking at)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LIVING_THERE(stdlib_call("re", "compile", &[py_str("\\b(?:live|lives|lived|living|moved|moves|moving|relocate|relocated|relocating|grew up|born|home|hometown|my flat|my house)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("spring"), py_str("summer"), py_str("autumn"), py_str("winter"), py_str("the morning"), py_str("the evening"), py_str("the afternoon"), py_str("the end"), py_str("the meantime"), py_str("general"), py_str("fact"), py_str("short"), py_str("total"), py_str("terms"), py_str("the"), py_str("a"), py_str("an"), py_str("its"), py_str("their"), py_str("addition"), py_str("practice"), py_str("reality")]) }
    s__A_TIME_WORD(call_builtin("frozenset", &[__data2()]).unwrap_or_else(|e| exc::raise(e)));
    s_LIMIT(stdlib_call("re", "compile", &[py_str("\\b(?:no|not|dont|doesnt|didnt|havent|isnt|without|ran out|run out|only got|only have|short of|lacking|instead|left)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _answers_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a short reply to the bot's last turn, not a continuation of it.");
    return { let __b = call_builtin("bool", &[pyrt::xrefs::xcall("memory.topics", "is_ack", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CONTINUES(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } };
    Py::None
}
pub fn _previous_ask(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line the person put before this one, when it asked for suggestions.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    said = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&mcall(&g_ASKED_FOR_IDEAS(), "search", &[said.clone()])) { said.clone() } else { py_str("") };
    Py::None
}
pub fn takes_back(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line is the person taking something back off the record.\n\n    \"forget the chicken\", \"drop the whole trip\", \"take that back\", \"never mind\": the line gives no\n    fact and asks nothing, so its reply is an acknowledgement rather than an answer. A reader that\n    hands the last thing said back - \"say that again\", \"repeat that\" - has nothing to repeat in it,\n    and handing it back twice (\"Sure - Got it - forget the chicken.\") is worse than the answer\n    before it.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    rest = mcall(&mcall(&g_TAKE_BACK_LEAD(), "sub", &[py_str(""), said.clone()]), "strip", &[py_str(" .,;:!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&rest.clone(), "startswith", &[g_TAKE_BACK_WHOLE()])) {
    return Py::Bool(true);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&rest.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) {
    return Py::Bool(false);
    }
    first = mcall(&geti(&stdlib_call("re", "split", &[py_str("[\\s,]+"), rest.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(".,;:!?")]);
    return Py::Bool(py_contains(&g_TAKES_BACK(), &first.clone()));
    Py::None
}
pub fn the_one_asked_about(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut last_verb: Py = Py::None;
    let mut new: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trimmed: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The one thing a framed line asks about: \"tell me about emma\" -> \"emma\".\n\n    The asking is taken off the front, one frame at a time, and what is left has to be a thing\n    and nothing else: \"who is meg on eastenders\" leaves \"meg on eastenders\", which is a programme\n    and not a name the chat holds, so it is not handed back. This only says what the line is\n    about; whether the chat holds it is for the caller to check.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" ?!.,;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    last_verb = mcall(&g_WHO_X_IS(), "match", &[said.clone()]);
    if py_truthy(&last_verb.clone()) {
    said = mcall(&mcall(&last_verb.clone(), "group", &[py_str("thing")]), "strip", &[py_str(" ?!.,;:")]);
    }
    changed = Py::Bool(true);
    while py_truthy(&changed.clone()) {
    changed = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&g_THE_ASKING());
    for __item in __iter1 {
    frame = __item.clone();
    new = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "sub", args))(&__args) }, "strip", &[py_str(" ?!.,;:")]);
    if py_truthy(&{ let __b = new.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&new.clone(), &said.clone())) } }) {
    said = new.clone();
    changed = Py::Bool(true);
    }
    }
    trimmed = mcall(&mcall(&g_THE_ASKING_TAIL(), "sub", &[py_str(""), said.clone()]), "strip", &[py_str(" ?!.,;:")]);
    if py_truthy(&{ let __b = trimmed.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&trimmed.clone(), &said.clone())) } }) {
    said = trimmed.clone();
    changed = Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_POINTED_AT(), &mcall(&said.clone(), "lower", &[])))) {
    return said.clone();
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS_ASKED_ABOUT())) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_THE_ASKING_VERB(), "match", &[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_VERBS(), &mcall(&word.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_looks_like_a_thing(&[said.clone()])))) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn thing(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line names on its own (\"pudding\", \"a starter\"), or \"\".\n\n    The line has to be that and nothing else: an ask of its own, a verb, a word pointing back at\n    the last subject, or a word that asks for advice all mean the line is not a bare name.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &op_add(&g_MOST_WORDS(), &Py::Int(4))) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_ASKING(), "match", &[said.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("^\\s*(?:what|how)\\s+about\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    }
    rest = mcall(&g_LEAD(), "sub", &[py_str(""), said.clone()]);
    rest = mcall(&g_QUANT(), "sub", &[py_str(""), mcall(&rest.clone(), "strip", &[py_str(" .,;:!?")])]);
    rest = mcall(&rest.clone(), "strip", &[py_str(" .,;:!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return py_str("");
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS())) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_POINTS_BACK(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_VERBS(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_TAKES_BACK(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ASKS_FOR_AN_ANSWER(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("what"), py_str("how"), py_str("why"), py_str("when")]), &bare.clone())) } } } } } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[a-z]"), bare.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&_looks_like_a_thing(&[mcall(&py_str(" "), "join", &[words.clone()])])))) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[words.clone()]);
    Py::None
}
pub fn _looks_like_a_thing(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A thing, not a sentence: it must not be a bare pronoun or a word with a digit in it, and it\n    must have at least one word of real length (a name or a noun, not \"a bit\").");
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[py_str("\\d"), word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(op_ge(&call_builtin("len", &[stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&word.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn carries_a_name(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line, or a value from it, is said with the naming word in it.");
    return call_builtin("bool", &[mcall(&g_NAME_WORD(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_name_in(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The name a value carries, or \"\" when the value is not said with the naming word.\n\n    A correction says the name inside the frame that names it - \"no its called tim\" hands over\n    \"called tim\" - and the frame is not part of the name: kept whole, the cat was filed as\n    \"Called tim\" and said back as \"I'll call the cat Called tim\". Only a value with the naming\n    word in it is touched, so an ordinary value (\"tim\", \"tree nuts\") is handed back untouched\n    by the caller, which is the turn's own value and not this reader's.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&carries_a_name(&[text.clone()])))) {
    return py_str("");
    }
    words = mcall(&text.clone(), "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    bare = mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("called"), py_str("named"), py_str("name"), py_str("names"), py_str("name's")]), &bare.clone()))) {
    continue;
    }
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_IN_FRONT_OF_THE_NAME(), &mcall(&mcall(&one.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[])))) {  __out.push(one.clone()); } } py_list(__out) })() };
    while py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_AFTER_THE_NAME(), &mcall(&mcall(&geti(&rest.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?;:")]), "lower", &[]))) } }) {
    let _ = mcall(&rest.clone(), "pop", &[]);
    }
    if py_truthy(&rest.clone()) {
    return mcall(&mcall(&py_str(" "), "join", &[rest.clone()]), "strip", &[py_str(" .,!?;:")]);
    }
    return py_str("");
    }
    return py_str("");
    Py::None
}
pub fn asked_for_a_name(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the bot's own last line asked what the new animal or person is called.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]) } }) {
    return call_builtin("bool", &[mcall(&g_A_NAME_ASK(), "search", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_name_given(args: &[Py]) -> Py {
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
    let mut framed: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The name this line hands over in answer to the bot's own \"what's it called\", or \"\".\n\n    Only when the bot asked. \"biscuit\" on its own is the food in the kitchen in any other\n    conversation, and \"hes called biscuit\" a turn after \"What's its name?\" is the answer to that\n    question: the reply has to take the name up, not open a new subject with it. Read from the\n    bot's own line in memory[\"turns\"], so a bare name said unprompted is left alone.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("’"), py_str("'")]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asked_for_a_name(&[memory.clone()]))) } }) {
    return py_str("");
    }
    framed = the_name_in(&[said.clone()]);
    if py_truthy(&framed.clone()) {
    return framed.clone();
    }
    found = mcall(&g_JUST_A_NAME(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    name = mcall(&found.clone(), "group", &[py_str("name")]);
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_NAME(), &mcall(&name.clone(), "lower", &[])))) {
    return py_str("");
    }
    return name.clone();
    Py::None
}
pub fn _not_got(args: &[Py]) -> Py {
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
    let mut IRREGULAR_PARTICIPLES: Py = Py::None;
    let mut after: Py = Py::None;
    let mut food: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The food this line says they have not got, in the line's own words.\n\n    \"i dont have any cheese\" a turn after a frittata said \"cheese on top is worth it\" is a limit\n    on that idea, and the answer to it is the same dish without the cheese. Said with no idea on\n    the table at all, it is still a question about the kitchen: the dish table answers \"what can\n    i make without cheese\", and a line it has no dish for falls through to the acknowledgement.\n    A line about eating a food rather than about having it is not this reader's (NOT_A_SHELF).\n    ");
    match_ = mcall(&g_NOT_GOT(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    food = mcall(&mcall(&match_.clone(), "group", &[py_str("food")]), "strip", &[]);
    if py_truthy(&carries_a_name(&[food.clone()])) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_looks_like_a_thing(&[food.clone()])))) {
    return py_str("");
    }
    IRREGULAR_PARTICIPLES = pyrt::xrefs::xget("understanding.normalize", "IRREGULAR_PARTICIPLES");
    words = mcall(&mcall(&food.clone(), "lower", &[]), "split", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_SHELF(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    after = if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&words.clone(), &Py::Int(1)) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&{ let __b = mcall(&mcall(&match_.clone(), "groupdict", &[]), "get", &[py_str("holding")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&IRREGULAR_PARTICIPLES.clone(), &geti(&words.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&after.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SPAN_WORDS(), &after.clone())) } } } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&g_ASKING(), "match", &[food.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&food.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SAID_NOT_A_FOOD(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&food.clone(), "lower", &[]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_THANK_YOU(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    if py_truthy(&op_gt(&call_builtin("len", &[mcall(&food.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_MOST_WORDS())) {
    return py_str("");
    }
    return food.clone();
    Py::None
}
pub fn _said_the_food(args: &[Py]) -> Py {
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
    let mut food: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "food") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: food"))) };
    let mut said: Py = Py::None;
    let _ = py_str("True when the answer on the table is one this food was part of.");
    said = mcall(&_last_reply(&[memory.clone()]), "lower", &[]);
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&food.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("")), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _last_reply(args: &[Py]) -> Py {
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
    let _ = py_str("What the bot said on the turn before this one, or \"\".");
    return pyrt::xrefs::xcall("memory.holds", "last_reply", &[memory.clone()]);
    Py::None
}
pub fn the_thing_named(args: &[Py]) -> Py {
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
    let _ = py_str("The thing the person named as the chat's subject (\"washing machine\"), or \"\".\n\n    `the_subject` reads the newest turn's words, and a turn about a part of the thing is not the\n    thing: after \"my washing machine smells\" / \"i clean the filter\" the subject reads \"filter\",\n    and a line asking about another part of the machine has nobody to belong to. What the person\n    named is still the machine, so a reader that has to own a part asks with this.\n    ");
    return _the_thing_they_named(&[memory.clone()]);
    Py::None
}
pub fn about_the_thing(args: &[Py]) -> Py {
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
    let mut owner: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "owner") { v.clone() }
    else { py_str("") };
    let mut bare: Py = Py::None;
    let mut part: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut thing_said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The part of the subject this line asks about, written as a question, or \"\".\n\n    `owner` writes the part under that thing instead of the newest turn's subject: a caller that\n    finds the part reaches nothing under the last turn's words tries the thing the person named.\n    The subject itself is never owned by a part, so a line naming it still reads as no part.\n\n    \"and its population\" with the chat on Spain -> \"population of spain\"; \"what about the food\"\n    in Rome -> \"food of rome\". Empty for anything that is not that shape: a question of its own,\n    a verb, a thing carrying the previous ask (that is `follow_up`'s job), a part with no\n    subject to belong to, or a line that names the subject itself.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &op_add(&g_PART_WORDS(), &Py::Int(5))) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_ASKING(), "match", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "match", &[py_str("^\\s*(?:what|how)\\s+about\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    if py_truthy(&follow_up(&[text.clone(), memory.clone()])) {
    return py_str("");
    }
    rest = mcall(&mcall(&g_LEAD(), "sub", &[py_str(""), said.clone()]), "strip", &[py_str(" .,;:!?")]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_PART_LEAD(), "match", &[rest.clone()])))) {
    return py_str("");
    }
    rest = mcall(&mcall(&g_QUANT(), "sub", &[py_str(""), rest.clone()]), "strip", &[py_str(" .,;:!?")]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[\\s,]+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&mcall(&word.clone(), "strip", &[py_str(".,;:!?")])) {  __out.push(mcall(&word.clone(), "strip", &[py_str(".,;:!?")])); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_PART_WORDS()) } }) {
    return py_str("");
    }
    part = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&word.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&g_POINTS_BACK(), &bare.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VERBS(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("what"), py_str("how"), py_str("why"), py_str("when"), py_str("where"), py_str("which"), py_str("who")]), &bare.clone())) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PREP(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("another"), py_str("more"), py_str("other")]), &bare.clone())) } } } }) {
    return py_str("");
    }
    let _ = mcall(&part.clone(), "append", &[word.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&part.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_looks_like_a_thing(&[mcall(&py_str(" "), "join", &[part.clone()])]))) } }) {
    return py_str("");
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&mcall(&geti(&part.clone(), &Py::Int(0)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("what"), py_str("how")]))) {
    return py_str("");
    }
    thing_said = { let __b = mcall(&call_builtin("str", &[{ let __b = owner.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&thing_said.clone()))) {
    return py_str("");
    }
    said_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&py_str(" "), "join", &[part.clone()]), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&op_bitand(&said_words.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&thing_said.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?")])); } py_set(__out) })() })) {
    return py_str("");
    }
    return op_add(&op_add(&mcall(&py_str(" "), "join", &[part.clone()]), &py_str(" of ")), &thing_said.clone());
    Py::None
}
pub fn _the_thing_they_named(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing on the table as the person named it (\"boiler\", \"washing machine\"), or \"\".");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut row: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[row.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(row.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None), &Py::None, &Py::None, &op_neg(&Py::Int(1))));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter2 = py_iter_or_raise(&mcall(&g_NAMED_A_THING(), "finditer", &[said.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_THE_THING_ITSELF(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&words.clone()) {
    return mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)]);
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn a_measure_asked_again(args: &[Py]) -> Py {
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
    let mut adds_on: Py = Py::None;
    let mut before: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The measure question this line asks again about a new one, or \"\".\n\n    Two shapes, and both are questions about a job with a time in it:\n\n    - the line only adds to the turn before, and that turn asked how long: \"my boiler is broken\" /\n      \"how long will it take\" / \"and a new one fitted?\". The measure is the one from the turn\n      before and the thing is the one on the table.\n    - the line asks it with the words of its own: \"my boiler is 15 years old\" / \"how long does it\n      take to fit a new one\". \"one\" points back at the boiler.\n\n    Only the shape is read here. Whether the knowledge base has a fitting row for that thing is\n    the caller's question, and the caller keeps the answer only where it beats the line on its\n    own. A line that asks how long the thing LASTS, or what it costs, is that other question and\n    is left alone.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_A_MEASURE_AGAIN(), "match", &[said.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g__THE_LIFE(), "search", &[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_WANTS_A_NEW_ONE(), "search", &[said.clone()])))) {
    return py_str("");
    }
    adds_on = call_builtin("bool", &[mcall(&g_A_MEASURE_AGAIN(), "match", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = adds_on.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_FITTING(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_ASKED_THE_MEASURE(), "search", &[said.clone()]) } })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&adds_on.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_ASKED_THE_MEASURE(), "search", &[said.clone()]))) } }) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut row: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[row.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    if py_truthy(&adds_on.clone()) {
    before = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASKED_THE_MEASURE(), "search", &[before.clone()])))) {
    return py_str("");
    }
    }
    thing = _the_thing_they_named(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return py_str("");
    }
    return op_add(&py_str("how long does it take to fit a new "), &thing.clone());
    Py::None
}
pub fn _a_length_ask(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is a length question and nothing else.");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow\\s+(?:long|soon|many\\s+(?:weeks|months|days|hours|years))\\b"), mcall(&py_str(" "), "join", &[words.clone()])]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_LENGTH_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_length_ask_of_the_aim(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("A length question about the aim on the line before, however the activity is named.\n\n    Wider than `_a_length_ask` by the activity words above and by \"of\", which is how the activity\n    is joined to the length (\"how many weeks of training\"). It is only ever asked when the turn\n    before is an aim in the person's own words, so the width costs nothing on a line with no aim\n    behind it.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow\\s+(?:long|soon|many\\s+(?:weeks|months|days|hours|years))\\b"), mcall(&py_str(" "), "join", &[words.clone()])]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_LENGTH_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_ACTIVITY_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &py_str("of"))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _answers_a_pace(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let _ = py_str("True when the rewritten ask reaches an entry that answers with a rate of progress.");
    found = _the_entry(&[ask.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_PACE(), "search", &[call_builtin("str", &[{ let __b = call_builtin("getattr", &[found.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_entry(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut find: Py = Py::None;
    let mut understand: Py = Py::None;
    let _ = py_str("The knowledge entry the rewritten ask reaches, or None.");
    let __try1 = exc::catch(|| {
    find = py_func("find", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "find", args));
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    return py_tuple(vec![Py::Bool(true), py_call(&find, &[py_call(&understand, &[ask.clone(), memory.clone()])])]);
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
    Py::None
}
pub fn _answers_a_length(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut found: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("True when the rewritten ask reaches an entry that answers a length.");
    found = _the_entry(&[ask.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    reply = call_builtin("str", &[{ let __b = call_builtin("getattr", &[found.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_LENGTH_STATED(), "search", &[reply.clone()])))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[found.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    phrase = __item.clone();
    if py_truthy(&op_bitand(&call_builtin("set", &[mcall(&mcall(&call_builtin("str", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_LENGTH_KEY_WORDS())) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_length_entry_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut aim_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "aim_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: aim_words"))) };
    let mut found: Py = Py::None;
    let mut keyed: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when the ask reaches an entry that states a length and is about the aim itself.\n\n    `_answers_a_length` also asks that the entry be *keyed* on a length word, which is how a\n    question about the length of a job is told from one about the job. An entry can answer a\n    length without a length word in its key - the 10k training entry is keyed on \"10k\" and\n    \"training\" and opens \"a 10k takes about ten weeks\" - so the entry is matched to the aim by a\n    word they share instead.\n    ");
    found = _the_entry(&[ask.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_LENGTH_STATED(), "search", &[call_builtin("str", &[{ let __b = call_builtin("getattr", &[found.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])))) {
    return Py::None;
    }
    keyed = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = call_builtin("getattr", &[found.clone(), py_str("key"), py_tuple(vec![])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    part = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { part.clone() } else { py_tuple(vec![part.clone()]) });
    for __item in __iter2 {
    phrase = __item.clone();
    let __aug3 = op_bitor(&keyed.clone(), &call_builtin("set", &[mcall(&mcall(&call_builtin("str", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    keyed = __aug3;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&keyed.clone(), &call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&aim_words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))))) {
    return Py::None;
    }
    return found.clone();
    Py::None
}
pub fn _answers_a_length_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ask: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut aim_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "aim_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: aim_words"))) };
    let _ = py_str("True when the ask reaches an entry that states a length about the aim itself.");
    return Py::Bool(!py_is(&_the_length_entry_about(&[ask.clone(), memory.clone(), aim_words.clone()]), &Py::None));
    Py::None
}
pub fn _an_aim_before_the_side_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut turns: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut r: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The last aim in the chat, when the turn before was not one, or \"\".\n\n    Only read when the turn before this one answered with no length and no rate. \"my boiler is\n    broken\" / \"how long will it take\" is \"Ten minutes\" and a recipe answer is \"40 minutes\": there\n    the thing on the table has a length of its own, so the question is its and the aim is left\n    alone. A lunch suggestion says nothing about how long a stone takes to lose, so the question\n    goes back to the aim the person set - three turns is as far as it looks, because a plan that\n    old is another conversation.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("last_answer")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&g_A_LENGTH_STATED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PACE(), "search", &[said.clone()]) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut r: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[r.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(r.clone()); } } py_list(__out) })() }, &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    row = __item.clone();
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&row.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_GOAL(), "match", &[line.clone()]) } }) {
    return line.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_aim_and_its_length_question(args: &[Py]) -> Py {
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
    let mut activity: Py = Py::None;
    let mut aim: Py = Py::None;
    let mut as_it_stands: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut before: Py = Py::None;
    let mut deadline: Py = Py::None;
    let mut found: Py = Py::None;
    let mut goal: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut outcome: Py = Py::None;
    let mut paced: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut state: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut w: Py = Py::None;
    let mut what: Py = Py::None;
    let mut with_the_activity: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The length question this line asks about the aim on the line before, and whether the aim\n    carried a deadline: (ask, deadline), or (\"\", False).\n\n    The aim is the turn before this one, said as their own (\"im trying to get fit before my\n    wedding\"), with the deadline clause taken off it. The question handed back is that aim put\n    into the length ask the knowledge base answers - \"how long does it take to get fit\". An aim\n    on a deadline is often a change of pace rather than a job to finish, so the question is tried\n    the other way round too (\"how fast can i lose weight\"), and that is the form that reaches an\n    answer stating a rate.\n    ");
    outcome = mcall(&g_OUTCOME_ASK(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_length_ask(&[text.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_a_length_ask_of_the_aim(&[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&outcome.clone())) } } } }) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut row: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[row.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    before = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    match_ = mcall(&g_A_GOAL(), "match", &[before.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    before = _an_aim_before_the_side_question(&[turns.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    match_ = mcall(&g_A_GOAL(), "match", &[before.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow\\s+(?:long|soon|many)\\b"), before.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    verb = mcall(&g_GOAL_VERB(), "get", &[mcall(&mcall(&match_.clone(), "group", &[py_str("how")]), "lower", &[]), py_str("")]);
    what = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^to\\s+")); __args.push(py_str("")); __args.push(mcall(&match_.clone(), "group", &[py_str("what")])); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    deadline = call_builtin("bool", &[mcall(&g_A_DEADLINE_CLAUSE(), "search", &[what.clone()])]).unwrap_or_else(|e| exc::raise(e));
    goal = mcall(&mcall(&g_A_DEADLINE_CLAUSE(), "sub", &[py_str(""), what.clone()]), "strip", &[py_str(" .,;:-")]);
    words = mcall(&goal.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp11: Py = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(1), &__cmp11)) { op_le(&__cmp11, &Py::Int(6)) } else { Py::Bool(false) } }))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[a-z]{3}"), goal.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&goal.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_DISTRESS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    if py_truthy(&outcome.clone()) {
    state = mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), mcall(&outcome.clone(), "group", &[py_str("state")])]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_eq(&state.clone(), &stdlib_call("re", "sub", &[op_add(&op_add(&py_str("^(?:"), &mcall(&py_str("|"), "join", &[call_builtin("sorted", &[g_AIM_VERB()]).unwrap_or_else(|e| exc::raise(e))])), &py_str(")\\s+")), py_str(""), mcall(&goal.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_tuple(vec![py_str(""), Py::Bool(false)]);
    }
    }
    aim = mcall(&mcall(&op_add(&verb.clone(), &goal.clone()), "lower", &[]), "strip", &[]);
    activity = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e))) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_ACTIVITY_WORDS(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&activity.clone()) {
    as_it_stands = mcall(&py_str(" "), "join", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&_answers_a_length_about(&[as_it_stands.clone(), memory.clone(), mcall(&mcall(&goal.clone(), "lower", &[]), "split", &[])])) {
    return py_tuple(vec![py_str(""), deadline.clone()]);
    }
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    let __iter2 = py_iter_or_raise(&py_tuple(vec![op_add(&op_add(&op_add(&op_add(&py_str("how long does "), &geti(&activity.clone(), &Py::Int(0))), &py_str(" for ")), &aim.clone()), &py_str(" take")), op_add(&op_add(&op_add(&op_add(&py_str("how long does "), &geti(&activity.clone(), &Py::Int(0))), &py_str(" for ")), &stdlib_call("re", "sub", &[py_str("^[a-z]+\\s+"), py_str(""), aim.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str(" take"))]));
    for __item in __iter2 {
    with_the_activity = __item.clone();
    found = _the_length_entry_about(&[with_the_activity.clone(), memory.clone(), mcall(&mcall(&goal.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    continue;
    }
    reply = mcall(&mcall(&call_builtin("str", &[{ let __b = call_builtin("getattr", &[found.clone(), py_str("reply"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = reply.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &slice_or_raise(&reply.clone(), &Py::None, &Py::Int(70), &Py::None))) } }) {
    continue;
    }
    return py_tuple(vec![with_the_activity.clone(), deadline.clone()]);
    }
    }
    asked = op_add(&py_str("how long does it take to "), &aim.clone());
    if py_truthy(&_answers_a_length(&[asked.clone(), memory.clone()])) {
    return py_tuple(vec![asked.clone(), deadline.clone()]);
    }
    paced = op_add(&py_str("how fast can i "), &mcall(&op_add(&verb.clone(), &goal.clone()), "lower", &[]));
    if py_truthy(&_answers_a_pace(&[paced.clone(), memory.clone()])) {
    return py_tuple(vec![paced.clone(), deadline.clone()]);
    }
    if py_truthy(&_answers_a_pace(&[asked.clone(), memory.clone()])) {
    return py_tuple(vec![asked.clone(), deadline.clone()]);
    }
    return py_tuple(vec![py_str(""), deadline.clone()]);
    Py::None
}
pub fn a_goal_measured(args: &[Py]) -> Py {
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
    let _ = py_str("The length question this line asks about the aim on the line before, or \"\".\n\n    The caller keeps the answer only where it beats the line on its own, so a bare \"how long will\n    it take\" that today reaches an entry still reaches it.\n    ");
    return geti(&_the_aim_and_its_length_question(&[text.clone(), memory.clone()]), &Py::Int(0));
    Py::None
}
pub fn a_length_about_an_aim_on_a_deadline(args: &[Py]) -> Py {
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
    let mut deadline: Py = Py::None;
    let _ = py_str("The length question about the aim, when the aim named a deadline, or \"\".\n\n    A bare \"how long will it take\" names nothing of its own, so the knowledge base answers about\n    whatever subject the chat left in memory - and with an aim on a deadline that subject is the\n    deadline: \"im aiming to get fit before the holiday\" makes \"holiday\" the subject, and the\n    length question then gets the package-travel rules. The aim is the thing they are asking\n    about, so where it carried a deadline this question is taken before the line's own read, and\n    not only where the line reaches nothing.\n    ");
    let __unpk1 = py_unpack(&_the_aim_and_its_length_question(&[text.clone(), memory.clone()]), 2, None);
    asked = __unpk1[0].clone();
    deadline = __unpk1[1].clone();
    return if py_truthy(&deadline.clone()) { asked.clone() } else { py_str("") };
    Py::None
}
pub fn _no_thanks(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line turns the answer down and names nothing of its own.");
    return call_builtin("bool", &[mcall(&g_NO_THANKS(), "match", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn follow_up(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut aim: Py = Py::None;
    let mut ask: Py = Py::None;
    let mut missing: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The ask this line carries (\"ideas for pudding\"), or \"\" when it carries none.");
    aim = a_goal_measured(&[text.clone(), memory.clone()]);
    if py_truthy(&aim.clone()) {
    return aim.clone();
    }
    again = a_measure_asked_again(&[text.clone(), memory.clone()]);
    if py_truthy(&again.clone()) {
    return again.clone();
    }
    missing = _not_got(&[text.clone(), memory.clone()]);
    if py_truthy(&missing.clone()) {
    ask = _previous_ask(&[memory.clone()]);
    if py_truthy(&ask.clone()) {
    if py_truthy(&_said_the_food(&[memory.clone(), missing.clone()])) {
    return op_add(&op_add(&mcall(&ask.clone(), "rstrip", &[py_str(" .,!?")]), &py_str(" without ")), &missing.clone());
    }
    return py_str("");
    }
    return op_add(&py_str("what can i make without "), &missing.clone());
    }
    if py_truthy(&_answers_the_bot(&[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&_no_thanks(&[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.trouble", "reads_as_trouble", &[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_BARE_ANOTHER(), "match", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    return if py_truthy(&_previous_ask(&[memory.clone()])) { py_str("ideas for anything else") } else { py_str("") };
    }
    said = thing(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_previous_ask(&[memory.clone()])))) {
    return py_str("");
    }
    return op_add(&py_str("ideas for "), &said.clone());
    Py::None
}
pub fn names_nothing_to_be_about(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing a follow-up names is only the person asking for a view.\n\n    \"honest view\" and \"your thoughts\" name the ask, not a subject: every word of them is doing\n    that and nothing else is left, so there is no thing for the earlier turn to be about.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_VIEW_POSSESSIVE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_VIEW_FRAME(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VIEW_MANNER(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _only_how_they_feel(args: &[Py]) -> Py {
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
    let mut FEELING_PHRASES: Py = Py::None;
    let mut feeling_words: Py = Py::None;
    let mut low: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut read: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a line whose whole content is how the person says they feel.\n\n    \"just tired\", \"im so shattered\", \"feeling low\": the reading says the feeling (its `emotion`)\n    and every other word in the line is a filler or the feeling's own adjective, so there is no\n    thing in it to ask about. A question, or a feeling with anything else in the line, is not\n    this: \"why am i so tired\" and \"im tired all the time\" are the entry's own questions.\n\n    A feeling said straight after a knowledge answer is not small talk either: it is about the\n    answer's subject (\"the baby woke up five times last night\" / \"im shattered\"), and the\n    answer's own follow-up (\"any tips\") is read from that answer. Standing down there read the\n    card's step as gone and the next ask came back as \"what have you tried so far\".\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    read = mcall(&memory.clone(), "get", &[py_str("understood")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&read.clone(), "get", &[py_str("emotion")]))) } }) {
    return Py::Bool(false);
    }
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    if py_truthy(&{ let __b = turns.clone(); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("module")]), &py_str("answers"))) } } } }) {
    return Py::Bool(false);
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&low.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("question"))) } } } }) {
    return Py::Bool(false);
    }
    feeling_words = pyrt::xrefs::xget("compose.draft", "FEELING");
    FEELING_PHRASES = pyrt::xrefs::xget("compose.draft", "FEELING_PHRASES");
    let __iter1 = py_iter_or_raise(&op_add(&call_builtin("tuple", &[FEELING_PHRASES.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_FEELING_SAID_AS_A_PHRASE()));
    for __item in __iter1 {
    phrase = __item.clone();
    low = mcall(&low.clone(), "replace", &[phrase.clone(), py_str(" ")]);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_FEELING_FILLER(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&feeling_words.clone(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn about_it(args: &[Py]) -> Py {
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
    let mut corrected: Py = Py::None;
    let mut facts: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("The question a bare thing carries after a turn about something (\"what about frosting\").\n\n    \"im making a cake\" / \"frosting\" is a person asking about the frosting of the cake they are\n    making, and the knowledge base has an entry for it. On its own the line names a thing and\n    asks nothing, so `follow_up` above stands aside - its previous turn was not a request for\n    ideas - and nothing else reads it as a question: the lookup finds nothing, and the turn comes\n    back as \"I'm listening - say more if it helps.\" The thing they named is what the turn is\n    about, so that is the question asked. Only where the lookup answered nothing at all, so no\n    line that answers today moves.\n    ");
    if py_truthy(&_answers_the_bot(&[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&pyrt::xrefs::xcall("compose.trouble", "reads_as_trouble", &[text.clone()])) {
    return py_str("");
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&mcall(&fallback.clone(), "reads_as_a_judgement", &[text.clone()])) {
    return py_str("");
    }
    if py_truthy(&_only_how_they_feel(&[text.clone(), memory.clone()])) {
    return py_str("");
    }
    facts = pyrt::xrefs::xmodule("memory.facts");
    corrected = { let __b = mcall(&facts.clone(), "without_the_correction_opener", &[text.clone()]); if py_truthy(&__b) { __b } else { text.clone() } };
    said = thing(&[corrected.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    was = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&was.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&was.clone(), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))) } }) {
    return py_str("");
    }
    if py_truthy(&names_nothing_to_be_about(&[said.clone()])) {
    return py_str("");
    }
    return op_add(&py_str("what about "), &said.clone());
    Py::None
}
pub fn a_figure_on_the_doing(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut index: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The figure the plan put on the doing (\"run 5k\"), or \"\" when the plan named none.");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return py_str("");
    }
    said = _the_plan_said(&[memory.clone(), word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    one = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&one.clone(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE_ON_THE_END(), "match", &[geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1)))]) } }) {
    return geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1)));
    }
    }
    return py_str("");
    Py::None
}
pub fn is_a_progress_ask(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks how good, how fluent or how far along the doing is.\n\n    Read from the asking alone, so the answer side can tell a progress ask (which the subject has\n    already been written into as a doing) from the other measures that point back.\n    ");
    return call_builtin("bool", &[mcall(&g_PROGRESS_ASK(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_doing(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut doing: Py = Py::None;
    let mut found: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The doing the plan named, written as the competence is said with it, or \"\".\n\n    \"im learning to cycle\" / \"how long until im good at it\" points at cycling, and the point of\n    writing it in is that the entry for cycling is reached. Written behind an article it is another\n    word entirely (\"the cycle\"), and left bare (\"the run\") the same. Only a word the plan named as\n    a doing is turned this way: a thing stays as it is, because \"good at the guitar\" is right and\n    \"good at guitaring\" is not.\n    ");
    said = _the_plan_said(&[memory.clone(), word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&word.clone())) } }) {
    return py_str("");
    }
    word = mcall(&word.clone(), "lower", &[]);
    named = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_DOING_NAMED(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    doing = { let __b = mcall(&found.clone(), "group", &[py_str("doing")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("also")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(py_eq(&doing.clone(), &word.clone()))) {
    named = Py::Bool(true);
    break;
    }
    if py_truthy(&{ let __b = mcall(&doing.clone(), "endswith", &[py_str("ing")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&slice_or_raise(&doing.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), "rstrip", &[py_str("n")]), &mcall(&word.clone(), "rstrip", &[py_str("n")]))) } }) {
    named = Py::Bool(true);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    named = the_doing_the_plan_named(&[word.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_str("");
    }
    return _the_ing(&[word.clone()]);
    Py::None
}
pub fn the_doing_the_plan_named(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut said: Py = Py::None;
    let _ = py_str("True when the plan named this word as something to do (\"want to run\") and not a thing.");
    said = _the_plan_said(&[memory.clone(), word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&word.clone())) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[op_modulo(&gattr(&g_A_PLAN_TO_THE_DOING(), "pattern"), &stdlib_call("re", "escape", &[mcall(&word.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_plan_said(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut line: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("What the person said the plan in, looking back a few turns, or \"\".\n\n    The topic has already been narrowed to the thing by the time a follow-up is read (\"run\"), so\n    the words that say it was a doing (\"learn to run 5k\") are in the turn before: the newest user\n    line that names the word is the plan.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut row: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[row.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(row.clone()); } } py_list(__out) })() };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    row = __item.clone();
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&row.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e)), &mcall(&word.clone(), "lower", &[]))) } }) {
    let _ = mcall(&said.clone(), "append", &[line.clone()]);
    }
    }
    topic = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&topic.clone()) {
    let _ = mcall(&said.clone(), "append", &[topic.clone()]);
    }
    return mcall(&py_str(" | "), "join", &[said.clone()]);
    Py::None
}
pub fn _the_ing(args: &[Py]) -> Py {
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
    let _ = py_str("The -ing form of a bare verb: \"run\" -> \"running\", \"cycle\" -> \"cycling\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("ing")]) } }) {
    return word.clone();
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("e")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&word.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing"));
    }
    if py_truthy(&{ let __b = op_le(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("[^aeiou][aeiou][bdgklmnprst]$"), word.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return op_add(&op_add(&word.clone(), &geti(&word.clone(), &op_neg(&Py::Int(1)))), &py_str("ing"));
    }
    return op_add(&word.clone(), &py_str("ing"));
    Py::None
}
pub fn _the_doing_in_hand(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The doing in progress in the line before this one: \"learning to drive\", or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    found = mcall(&g_IN_PROGRESS(), "search", &[said.clone()]);
    return if py_truthy(&found.clone()) { mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[py_str(" ,;:-")]) } else { py_str("") };
    }
    return py_str("");
    Py::None
}
pub fn the_duration_for_the_doing(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The length ask written about the doing in progress, or \"\" when the line is not that ask.\n\n    \"im cooking pasta\" / \"how long on the hob\" is one question: what the pasta needs on the heat.\n    Written with the doing in it (\"how long does it take to cooking pasta\") it reaches the entry\n    that knows - dried pasta is 8 to 12 minutes - where the line on its own reaches the row about\n    pasta left out of the fridge. Only the shape is read here; whether the doing's entry then\n    answers it is the caller's question, and a place word (\"on the hob\") is dropped rather than\n    kept, because \"cooking pasta on the hob\" is what reaches the left-out row. The \"to ... -ing\"\n    is not English, and it is never shown to anybody: it is tested by which entry it reaches, and\n    it is the wording that takes both pasta (8 to 12 minutes) and rice (1 part to 1.5, 12\n    minutes) to the cooking rows.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!?,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_THE_LENGTH_FOR_IT(), "match", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_LENGTH_NOT(), "search", &[said.clone()]) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&said.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MEASURE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MEASURE_VERBS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_LENGTH_FILLERS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_WORDS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_PREPOSITIONS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_ARTICLES(), &word.clone())) } }) {
    continue;
    }
    return py_str("");
    }
    doing = _the_doing_in_hand(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&doing.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&doing.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)); if py_truthy(&__b) { __b } else { mcall(&g_JOURNEY(), "match", &[doing.clone()]) } } } }) {
    return py_str("");
    }
    if py_truthy(&_the_place_belongs_to_the_doing(&[said.clone(), doing.clone(), memory.clone()])) {
    return py_str("");
    }
    return op_add(&py_str("how long does it take to "), &doing.clone());
    Py::None
}
pub fn _the_place_belongs_to_the_doing(args: &[Py]) -> Py {
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
    let mut doing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "doing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: doing"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _entry: Py = Py::None;
    let mut by_topic: Py = Py::None;
    let mut domain: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut place: Py = Py::None;
    let mut understand: Py = Py::None;
    let _ = py_str("True when the line's place is not the doing's place, so the doing cannot answer it.\n\n    The place is dropped from the written ask, which is safe only where the doing is the thing\n    that is done in it: \"how long on the hob\" after \"im cooking pasta\" is the pasta's time on the\n    heat. A place ask is written about whatever the doing in hand is, though, and the doing can be\n    something else entirely: after \"im learning to drive\", \"how long in the oven\" was written out\n    as \"how long does it take to learning to drive\" and the person asking about an oven was told\n    how many lessons it takes to pass a driving test.\n\n    The understanding engine reads the kind of thing a place is, and every knowledge domain\n    declares the kinds of thing it answers (`bot.knowledge.by_topic`), so an entry whose domain\n    does not answer the place's kind of thing is an answer to a different question: cooking is a\n    kind the pasta entry answers, so \"on the hob\" is the pasta's place, and a kitchen is not a\n    kind the driving entry answers.\n\n    The reading is of the place the line names, not of the line: the engine's `topic` for a length\n    ask carries the doing in hand along with the line (\"how long does that take\" after \"im\n    planning to retrain as a plumber\" reads `maths-and-numbers` here and `home-and-diy` in\n    another chat), so asking it which kind of thing the line is about refused a rewrite that is\n    right - the plumber's own length answer - on every chat whose doing scored a house or a DIY\n    topic (bronze-camera, #9963). A line that names no place of its own is not asking about one,\n    so nothing is refused. Where either reading is missing nothing is refused either, which\n    leaves the caller's own checks as they are.\n    ");
    by_topic = py_func("by_topic", |args: &[Py]| pyrt::xrefs::xcall("knowledge", "by_topic", args));
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    _entry = py_func("_entry", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "_entry", args));
    place = call_builtin("str", &[{ let __b = mcall(&{ let __b = py_call(&understand, &[said.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::Bool(false);
    }
    item = py_call(&_entry, &[op_add(&py_str("how long does it take to "), &doing.clone())]);
    if py_truthy(&Py::Bool(py_is(&item.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    domain = call_builtin("str", &[{ let __b = call_builtin("getattr", &[item.clone(), py_str("domain"), py_str("")]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&domain.clone()))) {
    return Py::Bool(false);
    }
    kind = call_builtin("str", &[{ let __b = mcall(&{ let __b = py_call(&understand, &[place.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&kind.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&mcall(&py_call(&by_topic, &[]), "get", &[domain.clone(), py_tuple(vec![])]), &kind.clone()));
    Py::None
}
pub fn _the_aspect_asked(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut nouns: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The question shape for a line that names only an aspect (\"the cost\"), or \"\".");
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_ASPECT_FILLERS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&g_ASPECT_ASKS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    nouns = __unpk2[0].clone();
    shape = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&nouns.clone(), &geti(&rest.clone(), &Py::Int(0))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&shape.clone(), &py_str("{thing}"))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&shape.clone(), "startswith", &[py_str("what do i feed")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FED_THINGS(), &thing.clone())) } } } }) {
    continue;
    }
    return shape.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_job_before(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The job the line before named, as a bare verb: \"the hedge needs cutting back\" -> \"cut\".");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut row: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[row.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(row.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_THE_JOB(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("ing")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("ed")]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } } } }) {
    stem = if py_truthy(&mcall(&word.clone(), "endswith", &[py_str("ing")])) { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None) } else { slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None) };
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&stem.clone(), &op_neg(&Py::Int(1))), &geti(&stem.clone(), &op_neg(&Py::Int(2))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("lsz"), &geti(&stem.clone(), &op_neg(&Py::Int(1))))) } } } }) {
    stem = slice_or_raise(&stem.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THE_JOB(), &stem.clone())) } }) {
    return stem.clone();
    }
    }
    }
    found = mcall(&g_A_BARE_JOB(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THE_JOB(), &mcall(&found.clone(), "group", &[Py::Int(1)]))) } }) {
    return mcall(&found.clone(), "group", &[Py::Int(1)]);
    }
    return py_str("");
    Py::None
}
pub fn the_best_time_to_do_it(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut head: Py = Py::None;
    let mut job: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_target: Py = Py::None;
    let mut target: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The ask written out with the thing it points at, or \"\".\n\n    Only the ask that names nothing but a job and a word pointing back: \"when is the best time to\n    do it\", \"what is the best day to plant them\". The thing is the one the reading resolved the\n    pointing word to (\"courgette plants\"), so the line becomes the question about it. A line that\n    names a thing of its own (\"when is the best time to plant tomatoes\") is already that question\n    and is left alone, and so is one that points at nothing.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return py_str("");
    }
    found = mcall(&g_BEST_TIME_TO_DO(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_AT_A_THING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(!py_contains(&g_POINTS_AT_A_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_A_VAGUE_JOB(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    ref_ = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("ref")]) } else { Py::None };
    target = py_str("");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("it"), py_str("they"), py_str("them"), py_str("that"), py_str("this"), py_str("those"), py_str("these")]));
    for __item in __iter1 {
    word = __item.clone();
    said_target = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = ref_.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[word.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = said_target.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_POINTS_AT_A_THING(), &mcall(&said_target.clone(), "lower", &[]))) } }) {
    target = said_target.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&target.clone()))) {
    return py_str("");
    }
    rest = mcall(&found.clone(), "group", &[py_str("rest")]);
    job = stdlib_call("re", "match", &[py_str("(\\w+)"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = job.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_VAGUE_JOB(), &mcall(&mcall(&job.clone(), "group", &[Py::Int(1)]), "lower", &[]))) } }) {
    before = _the_job_before(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return py_str("");
    }
    if py_truthy(&before.clone()) {
    rest = mcall(&slice_or_raise(&rest.clone(), &mcall(&job.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|its|them|that|this|those|these|one|ones)\\b")); __args.push(target.clone()); __args.push(rest.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return op_add(&op_add(&op_add(&py_str("when is the best time to "), &before.clone()), &py_str(" ")), &rest.clone());
    }
    }
    head = slice_or_raise(&said.clone(), &Py::None, &mcall(&found.clone(), "start", &[py_str("rest")]), &Py::None);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|its|them|that|this|those|these|one|ones)\\b")); __args.push(target.clone()); __args.push(rest.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    return op_add(&head.clone(), &rest.clone());
    Py::None
}
pub fn _only_the_measure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether a line is nothing but the asking, the measure and a bare doing, so it points back.\n\n    A line with a noun of its own says what it is about and is somebody else's job; these are the\n    words that leave it with nothing of its own (\"how many times a week\", \"how often should i go\").\n    ");
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk4178 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk4178[0].clone(); let mut word: Py = __unpk4178[1].clone(); __out.push({ let __b = Py::Bool(py_contains(&g_MEASURE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_LIGHT_DOING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_PREPOSITIONS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_ARTICLES(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_PAIRS(), &py_tuple(vec![word.clone(), geti(&low.clone(), &op_add(&index.clone(), &Py::Int(1)))]))) } } } } } } } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn about_the_subject(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut faulty: Py = Py::None;
    let mut in_hand: Py = Py::None;
    let mut index: Py = Py::None;
    let mut low: Py = Py::None;
    let mut place: Py = Py::None;
    let mut progress: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_the_thing: Py = Py::None;
    let mut timed: Py = Py::None;
    let mut word: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The measure ask written about the subject of the chat, or \"\".\n\n    \"how long should it be\" and \"how much would that cost\" name nothing: they point back at what\n    the chat is on, and the entry matcher has only the topic's newest words to go on. Written with\n    the subject in, the same line reaches the entry that knows it - \"how much would the dog cost\",\n    \"how long should the speech be\". Only the shape is read here (a measure ask, a word pointing\n    back, and nothing else of its own); whether the line then reaches an entry is the caller's\n    question, and the caller uses the answer only where it beats the line on its own.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    said = mcall(&said.clone(), "strip", &[py_str(" .!?")]);
    said = _without_and_at_the_start(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return py_str("");
    }
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?")])); } py_list(__out) })() };
    timed = the_duration_for_the_doing(&[said.clone(), memory.clone()]);
    if py_truthy(&timed.clone()) {
    return timed.clone();
    }
    said_the_thing = subject_word(&[memory.clone()]);
    asked = _the_aspect_asked(&[low.clone(), said_the_thing.clone()]);
    if py_truthy(&asked.clone()) {
    return if py_truthy(&said_the_thing.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), said_the_thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "format", args))(&__args) } } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_MEASURE_ASK(), "search", &[said.clone()])))) {
    return py_str("");
    }
    place = _the_place_phrase(&[low.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&place.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_only_the_measure(&[low.clone()]))) } } } }) {
    return py_str("");
    }
    faulty = a_faulty_thing(&[memory.clone()]);
    while py_truthy(&{ let __b = faulty.clone(); if !py_truthy(&__b) { __b } else { { let __b = low.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_CONNECTIVE(), &geti(&low.clone(), &Py::Int(0)))) } } } }) {
    low = slice_or_raise(&low.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    bare = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    progress = call_builtin("bool", &[mcall(&g_PROGRESS_ASK(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MEASURE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_LIGHT_DOING(), &word.clone())) } } } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_PLACE_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = faulty.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MEASURE_VERBS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = progress.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PROGRESS_WORDS(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_PREPOSITIONS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_ARTICLES(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_PAIRS(), &py_tuple(vec![word.clone(), geti(&low.clone(), &op_add(&index.clone(), &Py::Int(1)))]))) } }) {
    continue;
    }
    return py_str("");
    }
    said_the_thing = subject_word(&[memory.clone()]);
    if py_truthy(&place.clone()) {
    in_hand = { let __b = _the_thing_in_hand(&[memory.clone(), said.clone()]); if py_truthy(&__b) { __b } else { said_the_thing.clone() } };
    if py_truthy(&Py::Bool(!py_truthy(&in_hand.clone()))) {
    return py_str("");
    }
    return _about_the_thing_in_the_place(&[low.clone(), place.clone(), in_hand.clone(), said.clone(), memory.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&said_the_thing.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = low.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    said = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&said.clone(), "split", &[]), &op_neg(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None, &Py::None)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return op_add(&op_add(&said.clone(), &py_str(" ")), &said_the_thing.clone());
    }
    written = op_add(&py_str("the "), &said_the_thing.clone());
    doing = if py_truthy(&progress.clone()) { _the_doing(&[said_the_thing.clone(), memory.clone()]) } else { py_str("") };
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_HOW_GOOD(), "search", &[said.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = a_figure_on_the_doing(&[memory.clone(), said_the_thing.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow\\s+long\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = doing.clone(); if py_truthy(&__b) { __b } else { { let __b = progress.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_GERUND(), "match", &[said_the_thing.clone()]) } } } }) {
    written = { let __b = doing.clone(); if py_truthy(&__b) { __b } else { said_the_thing.clone() } };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|this|they|them|those|these|one|ones)\\b")); __args.push({ let asked = asked.clone(); let bare = bare.clone(); let doing = doing.clone(); let faulty = faulty.clone(); let in_hand = in_hand.clone(); let index = index.clone(); let low = low.clone(); let memory = memory.clone(); let place = place.clone(); let progress = progress.clone(); let said = said.clone(); let said_the_thing = said_the_thing.clone(); let text = text.clone(); let timed = timed.clone(); let word = word.clone(); let written = written.clone(); py_func("lambda", move |args: &[Py]| { let asked = asked.clone(); let bare = bare.clone(); let doing = doing.clone(); let faulty = faulty.clone(); let in_hand = in_hand.clone(); let index = index.clone(); let low = low.clone(); let memory = memory.clone(); let place = place.clone(); let progress = progress.clone(); let said = said.clone(); let said_the_thing = said_the_thing.clone(); let text = text.clone(); let timed = timed.clone(); let word = word.clone(); let written = written.clone(); let found: Py = __kwp(args, 0, "found").unwrap_or(Py::None); written.clone() }) }); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _without_and_at_the_start(args: &[Py]) -> Py {
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
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AND_AT_THE_START(), "sub", args))(&__args) };
    Py::None
}
pub fn _the_forms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut base: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "base") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: base"))) };
    let mut forms: Py = Py::None;
    forms = py_set(vec![base.clone(), op_add(&base.clone(), &py_str("s")), op_add(&base.clone(), &py_str("ing")), op_add(&base.clone(), &py_str("ed"))]);
    if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &geti(&base.clone(), &op_neg(&Py::Int(1)))))) {
    let _ = mcall(&forms.clone(), "add", &[op_add(&slice_or_raise(&base.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ing"))]);
    } else {
    if py_truthy(&mcall(&base.clone(), "endswith", &[py_str("y")])) {
    let __aug1 = op_bitor(&forms.clone(), &py_set(vec![op_add(&slice_or_raise(&base.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")), op_add(&slice_or_raise(&base.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ied"))]));
    forms = __aug1;
    } else {
    if py_truthy(&Py::Bool(!py_contains(&py_str("aeiouwxy"), &geti(&base.clone(), &op_neg(&Py::Int(1)))))) {
    let __aug2 = op_bitor(&forms.clone(), &py_set(vec![op_add(&op_add(&base.clone(), &geti(&base.clone(), &op_neg(&Py::Int(1)))), &py_str("ing")), op_add(&op_add(&base.clone(), &geti(&base.clone(), &op_neg(&Py::Int(1)))), &py_str("ed"))]));
    forms = __aug2;
    }
    }
    }
    return forms.clone();
    Py::None
}
pub fn _the_doing_word(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The doing in the line, in its plain form (\"reheating\" -> \"reheat\"), or \"\".");
    let __iter1 = py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    bare = mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?")]);
    if py_truthy(&Py::Bool(py_contains(&g_DOING_BASE(), &bare.clone()))) {
    return geti(&g_DOING_BASE(), &bare.clone());
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_doing_word(args: &[Py]) -> Py {
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
    return call_builtin("bool", &[_the_doing_word(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _what_the_chat_is_on(args: &[Py]) -> Py {
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
    let mut this_line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "this_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: this_line"))) };
    let mut mine: Py = Py::None;
    let mut record: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("The line before this one: what the person said, or the record the chat was left on.");
    mine = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = this_line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    was = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&was.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&was.clone(), "lower", &[]), "split", &[])]), &mine.clone())) } }) {
    continue;
    }
    return was.clone();
    }
    record = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&record.clone(), "get", &[py_str("text")]) } }) {
    return call_builtin("str", &[geti(&record.clone(), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e));
    }
    return py_str("");
    Py::None
}
pub fn _the_thing_in_hand(args: &[Py]) -> Py {
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
    let mut this_line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "this_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: this_line"))) };
    let mut part: Py = Py::None;
    let mut was: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the line before this one is about: \"rice\" out of \"how do i cook rice\".");
    was = _what_the_chat_is_on(&[memory.clone(), this_line.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z']+"), mcall(&was.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MEASURE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_WORDS(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DOING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_PREPOSITIONS(), &word.clone())) } } } }) {
    continue;
    }
    return word.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_place_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut first: Py = Py::None;
    let mut index: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place the line names with its preposition and article (\"in the microwave\"), or \"\".");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_PLACE_WORDS(), &word.clone()))) {
    continue;
    }
    first = index.clone();
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_PAIRS(), &py_tuple(vec![geti(&low.clone(), &op_sub(&index.clone(), &Py::Int(1))), word.clone()]))) } }) {
    first = op_sub(&index.clone(), &Py::Int(1));
    }
    start = first.clone();
    while py_truthy(&{ let __b = start.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_PREPOSITIONS(), &geti(&low.clone(), &op_sub(&start.clone(), &Py::Int(1))))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_ARTICLES(), &geti(&low.clone(), &op_sub(&start.clone(), &Py::Int(1))))) } } } }) {
    let __aug3 = op_sub(&start.clone(), &Py::Int(1));
    start = __aug3;
    }
    return mcall(&py_str(" "), "join", &[slice_or_raise(&low.clone(), &start.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None)]);
    }
    return py_str("");
    Py::None
}
pub fn _about_the_thing_in_the_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut place: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "place") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: place"))) };
    let mut said_the_thing: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_the_thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_the_thing"))) };
    let mut this_line: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "this_line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: this_line"))) };
    let mut memory: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut doing: Py = Py::None;
    let mut measure: Py = Py::None;
    let mut word: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The ask about the place, written about the thing it is done to, or \"\".\n\n    \"how do i cook rice\" then \"how long in the microwave\" is a question about the rice, in the\n    microwave. Read on its own words the place is the only thing the line names, so it reaches the\n    entry about the appliance and the person asking about their rice is told how long a microwave\n    lasts. Written about the thing - \"how long does the rice take in the microwave\" - it reaches the\n    entry that knows, and the answer is the ten minutes on full power they wanted.\n    ");
    if py_truthy(&Py::Bool(py_contains(&g_PLACE_WORDS(), &said_the_thing.clone()))) {
    return py_str("");
    }
    doing = _the_doing_word(&[_what_the_chat_is_on(&[memory.clone(), this_line.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&doing.clone()))) {
    return py_str("");
    }
    measure = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&low.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PLACE_PREPOSITIONS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_ARTICLES(), &word.clone())) } } } }) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))) {
    break;
    }
    let _ = mcall(&measure.clone(), "append", &[word.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&measure.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&measure.clone(), &py_str("how"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&measure.clone(), &py_str("what"))) } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_eq(&measure.clone(), &py_list(vec![py_str("how"), py_str("long")])))) {
    return py_str("");
    }
    written = op_add(&op_add(&op_add(&op_add(&op_add(&py_str("how long does it take to "), &doing.clone()), &py_str(" ")), &said_the_thing.clone()), &py_str(" ")), &place.clone());
    if py_truthy(&Py::Bool(!py_truthy(&_the_rewrite_earns_its_place(&[mcall(&py_str(" "), "join", &[low.clone()]), written.clone(), memory.clone()])))) {
    return py_str("");
    }
    return written.clone();
    Py::None
}
pub fn _the_rewrite_earns_its_place(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut alone: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "alone") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: alone"))) };
    let mut written: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "written") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: written"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _entry: Py = Py::None;
    let mut became: Py = Py::None;
    let mut last: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("Whether writing the thing into the place ask beats asking the place on its own.\n\n    The two asks have to reach different cards, and the card the thing finds must not be the one\n    that has just answered. \"how long in the oven\" after the roasting method reaches that method\n    again, and the person gets \"Nothing separate for the oven - it is the same advice: 30 to 40\n    minutes\" where today they get the figure itself. A card already on the screen is never worth\n    reaching a second time.\n    ");
    _entry = py_func("_entry", |args: &[Py]| pyrt::xrefs::xcall("memory.topics", "_entry", args));
    was = py_call(&_entry, &[alone.clone()]);
    became = py_call(&_entry, &[written.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&was.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&became.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&became.clone(), &was.clone())) } } } }) {
    return Py::Bool(false);
    }
    last = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("last_answer")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&last.clone(), "get", &[py_str("id")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&last.clone(), "get", &[py_str("id")]), &call_builtin("getattr", &[became.clone(), py_str("id"), Py::None]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _thing_in_hand(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut only: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject_keys: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the last answer was about, in one word (\"phone\", \"carpet\"), or \"\".\n\n    `subject_word` reads the noun a measure ask is about, and it takes the last content word of the\n    topic phrase: after \"my phone battery drains fast\" it gives \"fast\", which is what the line\n    says about the phone rather than the phone. A report is about the thing the answer on the\n    screen was about, and that answer's own key words say it: the topic's words in the order the\n    person said them, kept where the answer is keyed on one. Words are compared by their opening,\n    because a key holds one form of a word (\"drains\") and the line another (\"drain\").\n    ");
    topic = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("topic")]) } else { Py::None };
    said = if py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = topic.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return subject_word(&[memory.clone()]);
    }
    answer = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("last_answer")]) } else { Py::None };
    keys = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("keys")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    subject_keys = _the_answers_own_subject(&[answer.clone()]);
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![subject_keys.clone(), keys.clone()]));
    for __item in __iter1 {
    wanted = __item.clone();
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    bare = mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str(".,;:!?")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[bare.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_WORDS(), &bare.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&found.clone(), &bare.clone()))) {
    continue;
    }
    if py_truthy(&_only_a_particle(&[bare.clone(), { let __b = mcall(&{ let __b = topic.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }])) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&wanted.clone()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_eq(&slice_or_raise(&key.clone(), &Py::None, &Py::Int(4), &Py::None), &slice_or_raise(&bare.clone(), &Py::None, &Py::Int(4), &Py::None)))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[bare.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    only = subject_word(&[memory.clone()]);
    return if py_truthy(&only.clone()) { py_list(vec![only.clone()]) } else { py_list(vec![]) };
    }
    return found.clone();
    Py::None
}
pub fn _only_a_particle(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when every time the text says the word, it is the particle of a phrasal verb.\n\n    \"i keep waking up at 3am and cant get back to sleep\" is about the sleep, and the \"back\" in it\n    follows \"get\". Read as the thing in hand it wrote \"my back is been about three weeks now\" for\n    the follow-up \"its been about three weeks now\", and the sore-back-for-a-week card answered a\n    line about sleep with a paragraph about a body nobody had mentioned. A word the person says as\n    a thing of its own (\"my back is sore\") is not touched: only a word that never appears without\n    a verb in front of it is left out.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    found = call_builtin("list", &[stdlib_call("re", "finditer", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&found.clone());
    for __item in __iter1 {
    match_ = __item.clone();
    before = mcall(&slice_or_raise(&said.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Bool(false);
    }
    last = mcall(&geti(&before.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,;:!?()")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "VERBISH"), &last.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.kinds", "VERB_FORMS"), &last.clone())); if py_truthy(&__b) { __b } else { mcall(&last.clone(), "endswith", &[py_tuple(vec![py_str("ing"), py_str("ed")])]) } } } }))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_answers_own_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut all_entries: Py = Py::None;
    let mut entry_id: Py = Py::None;
    let mut first: Py = Py::None;
    let mut item: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The subject words of the entry the answer came from (\"battery\", \"charge\"), or ().");
    entry_id = if py_truthy(&call_builtin("isinstance", &[answer.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = answer.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("id")]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&entry_id.clone()))) {
    return py_tuple(vec![]);
    }
    let __try1 = exc::catch(|| {
    all_entries = py_func("entries", |args: &[Py]| pyrt::xrefs::xcall("knowledge", "entries", args));
    let __iter6 = py_iter_or_raise(&py_call(&all_entries, &[]));
    for __item in __iter6 {
    item = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("getattr", &[item.clone(), py_str("id"), py_str("")]).unwrap_or_else(|e| exc::raise(e)), &entry_id.clone())); if !py_truthy(&__b) { __b } else { gattr(&item.clone(), "key") } }) {
    first = geti(&gattr(&item.clone(), "key"), &Py::Int(0));
    parts = if py_truthy(&call_builtin("isinstance", &[first.clone(), py_tuple(vec![py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { first.clone() } else { py_tuple(vec![first.clone()]) };
    return py_tuple(vec![Py::Bool(true), call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[part.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
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
    return py_tuple(vec![]);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return py_tuple(vec![]);
    Py::None
}
pub fn reported_states(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut reads: Py = Py::None;
    let mut reported: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut things: Py = Py::None;
    let mut word: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The line with the thing in hand written in where it points back, and its neighbours.\n\n    \"its on 4% and the charger is at work\" is about the phone the chat is on, and the person is\n    telling the bot where it stands rather than asking anything. The line names its state, not its\n    subject, so the knowledge base cannot reach an answer; with the subject in it the same line\n    does. Empty unless three things hold, so a line that says what it is about is left alone:\n\n    - it points back (\"it\", \"its\", \"they\") rather than naming a subject;\n    - it reports something (a figure, a state word, or where the thing is);\n    - the chat is on a thing, and the line does not name that thing itself.\n\n    A line can hold more than one thing for the \"it\" to be (\"red wine out of a carpet\" is the\n    carpet when something still smells, and the wine when something has stained), and nothing here\n    can tell which the person means: the read is only a rewrite, and the caller takes the first\n    one that reaches an answer the conversation has not already had.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(16)) } }) {
    return py_list(vec![]);
    }
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?()")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTS_BACK(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    if py_truthy(&{ let __b = mcall(&g_ASKING(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MEASURE_ASK(), "search", &[said.clone()]) } }) {
    return py_list(vec![]);
    }
    things = _thing_in_hand(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&things.clone()))) {
    return py_list(vec![]);
    }
    reported = call_builtin("bool", &[{ let __b = mcall(&g_A_FIGURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_PLACE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { op_bitand(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } py_set(__out) })() }, &g_STATE_WORDS()) } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&reported.clone()))) {
    return py_list(vec![]);
    }
    reads = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&things.clone());
    for __item in __iter1 {
    thing = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &thing.clone()))) {
    continue;
    }
    written = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:its|it's)\\b")); __args.push(op_add(&op_add(&py_str("my "), &thing.clone()), &py_str(" is"))); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    written = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|they|them|their)\\b")); __args.push(op_add(&py_str("my "), &thing.clone())); __args.push(written.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_contains(&reads.clone(), &written.clone()))) {
    let _ = mcall(&reads.clone(), "append", &[written.clone()]);
    }
    }
    return reads.clone();
    Py::None
}
pub fn _the_persons_own_words(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut found: Py = Py::None;
    let mut key: Py = Py::None;
    let mut own: Py = Py::None;
    let mut source: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The words the person used for the thing in hand, as they spelled them, best first.\n\n    `_thing_in_hand` hands back the topic's words, and a topic is kept as stems: the thing after\n    \"im growing tomatoes in the greenhouse\" is \"tomatoe\", which no entry is keyed on, and by the\n    time the practice is read the topic is the practice line itself (\"feed every week\"), which\n    names no thing at all. So the thing is read from the answer on the screen - its own subject\n    words first, then its keys - and the spelling is taken from the person's own earlier line, kept\n    only where it matches that thing by its opening. Words that are the practice rather than the\n    thing (\"feed\", \"every\", \"week\") are dropped: written in front of a practice, a verb reaches\n    whatever entry fits it (\"growing feed every week\" reached the houseplant-feed card).\n    ");
    answer = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("last_answer")]) } else { Py::None };
    wanted = call_builtin("list", &[_the_answers_own_subject(&[answer.clone()])]).unwrap_or_else(|e| exc::raise(e));
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]) } else { Py::None };
    own = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    let __iter2 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^A-Za-z']+"), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&own.clone(), &word.clone())) } }) {
    let _ = mcall(&own.clone(), "append", &[word.clone()]);
    }
    }
    }
    found = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&py_tuple(vec![wanted.clone(), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_thing_in_hand(&[memory.clone()])) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]));
    for __item in __iter3 {
    source = __item.clone();
    let __iter4 = py_iter_or_raise(&source.clone());
    for __item in __iter4 {
    key = __item.clone();
    let __iter5 = py_iter_or_raise(&own.clone());
    for __item in __iter5 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&found.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__NOT_THE_PRACTICE(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MEASURE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_STATE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WHERE_IT_STANDS(), &word.clone())) } } } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&key.clone(), &Py::None, &Py::Int(4), &Py::None), &slice_or_raise(&word.clone(), &Py::None, &Py::Int(4), &Py::None)))) {
    let _ = mcall(&found.clone(), "append", &[word.clone()]);
    }
    }
    }
    if py_truthy(&found.clone()) {
    break;
    }
    }
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_thing_in_hand(&[memory.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g__NOT_THE_PRACTICE(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_WHERE_IT_STANDS(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } } } }) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    Py::None
}
pub fn stated_about_the_person(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut named: Py = Py::None;
    let mut reads: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut written: Py = Py::None;
    let _ = py_str("The line rewritten with the subject of the chat in front of it: [\"pension im 34 ...\"].\n\n    Empty unless four things hold, so a line that says what it is about is left alone:\n\n    - it is short and asks nothing (\"how long will it take\" is the ask readers' line);\n    - it states a fact rather than a feeling: it opens on the person (\"im ...\", \"we ...\", \"my\n      ...\") or on a preposition (\"for a ...\", \"with ...\"), or it says a figure;\n    - the chat is on a subject, and the line does not name that subject itself;\n    - the subject is a thing the knowledge base keeps an entry about, which is the caller's\n      question: the read is only a rewrite.\n\n    The subject goes in front (\"pension im 34 and self employed\") because the entry is keyed on\n    the subject and the fact, in that order of importance, and the line's own wording is kept:\n    the person's words are what the entry for that pairing is keyed on.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_FACT_MOST_WORDS()) } }) {
    return py_list(vec![]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_ASKING(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MEASURE_ASK(), "search", &[said.clone()]) } } } }) {
    return py_list(vec![]);
    }
    if py_truthy(&mcall(&g_A_CHOICE(), "search", &[said.clone()])) {
    return py_list(vec![]);
    }
    if py_truthy(&_a_place_of_its_own(&[memory.clone()])) {
    return py_list(vec![]);
    }
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?()")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_PERSON_OPEN(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_FACT_OPEN(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_FIGURE(), "search", &[said.clone()]) } } } }))) {
    return py_list(vec![]);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_TIMES_ONLY(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FACT_FILLER(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_list(vec![]);
    }
    if py_truthy(&call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_PEOPLE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COUNT_WORDS(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_list(vec![]);
    }
    reads = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&subject_words(&[memory.clone(), text.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &word.clone()))) {
    continue;
    }
    written = mcall(&py_str(" "), "join", &[mcall(&op_add(&op_add(&word.clone(), &py_str(" ")), &said.clone()), "split", &[])]);
    if py_truthy(&Py::Bool(!py_contains(&reads.clone(), &written.clone()))) {
    let _ = mcall(&reads.clone(), "append", &[written.clone()]);
    }
    }
    return reads.clone();
    Py::None
}
pub fn _a_place_of_its_own(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut read: Py = Py::None;
    let _ = py_str("True when the reading found a place named in this line.");
    read = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[read.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&read.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[found.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&found.clone(), "get", &[py_str("type")]), &py_str("place"))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn subject_words(args: &[Py]) -> Py {
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
    else { py_str("") };
    let mut kinds: Py = Py::None;
    let mut out: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut was: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The nouns the chat is on, best first: [\"boiler\"], [\"pension\"], [\"cat\", \"mat\"].\n\n    `the_subject` is the phrase the conversation is about, which on a statement is the sentence\n    itself (\"my boiler is banging\"). On a line the store has already noted as the topic (\"usually\n    in the morning\" after a boiler) that phrase is this very line, so the subject before it is\n    read instead: `the_carried_topic` is the record of what the chat was on before this line.\n    The words a knowledge entry is keyed on are the nouns in it, so the verbs (\"banging\",\n    \"weeing\"), the pointing words and the words that say nothing (\"my\", \"is\") are dropped, and\n    what is left is tried in the order the chat said it.\n    ");
    kinds = pyrt::xrefs::xmodule("understanding.kinds");
    was = py_str("");
    if py_truthy(&text.clone()) {
    record = pyrt::xrefs::xcall("memory.topics", "carried_topic", &[memory.clone(), text.clone()]);
    was = if py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&{ let __b = record.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = was.clone(); if py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z0-9]+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&out.clone(), &word.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_MEASURE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_WORDS(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g__NOT_A_PRONOUN(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&gattr(&kinds.clone(), "VERBISH"), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&gattr(&kinds.clone(), "VERB_FORMS"), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_THE_SUBJECT(), &word.clone())) } } } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    return slice_or_raise(&out.clone(), &Py::None, &Py::Int(4), &Py::None);
    Py::None
}
pub fn care_reports(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut things: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The line rewritten with the thing in hand in front of it: [\"tomatoes feed every week\"].\n\n    One read only, for the best word for the thing. Every word that might be the thing also\n    reaches entries of its own, and a second read is a second chance to reach a word-mate:\n    \"chillies feed weekly\" was answered from \"growing feed weekly\", because the houseplant-feed\n    card is keyed on \"growing\" and \"feed\" and says nothing about chillies. The first word is the\n    one the answer on the screen is about, so it is the only one written in.\n\n    Empty unless four things hold, so a line that says what it is about is left alone:\n\n    - it names a practice (a care verb) and how often, and is short;\n    - it points back (\"them\", \"it\") or says nothing else that names an object;\n    - it names no one of its own (\"i feed the kids every day\" is about the kids);\n    - the chat is on a thing, and the line does not name that thing itself.\n\n    The rewrite puts the thing first (\"tomatoes feed every week\") rather than in the subject's\n    place (\"i feed my tomatoes every week\"), because \"feed\" with \"my\" in front of the object\n    reads as meals for the household and the meal-plan entry takes it. Which of the reads answers\n    is the caller's question, and the caller's guard.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12)) } }) {
    return py_list(vec![]);
    }
    low = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&said.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,;:!?()")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CARE_VERBS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_list(vec![]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_HOW_OFTEN(), "search", &[said.clone()])))) {
    return py_list(vec![]);
    }
    if py_truthy(&{ let __b = mcall(&g_ASKING(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MEASURE_ASK(), "search", &[said.clone()]) } }) {
    return py_list(vec![]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&low.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CARE_NOT_THE_THING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_list(vec![]);
    }
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\s*(?:i|we|im|i'm|ive|i've)\\s+")); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_POINTS_BACK()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b"))); __args.push(py_str("")); __args.push(rest.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    rest = mcall(&py_str(" "), "join", &[mcall(&rest.clone(), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return py_list(vec![]);
    }
    things = _the_persons_own_words(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&things.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &geti(&things.clone(), &Py::Int(0)))) } }) {
    return py_list(vec![]);
    }
    return py_list(vec![mcall(&py_str(" "), "join", &[mcall(&op_add(&op_add(&geti(&things.clone(), &Py::Int(0)), &py_str(" ")), &rest.clone()), "split", &[])])]);
    Py::None
}
pub fn a_faulty_thing(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when the chat is on a thing that has stopped working, not on a plan about one.\n\n    \"my boiler is broken\" / \"how long will it take\" is a question about the repair, and the entry\n    that knows the answer is filed under the boiler *and* the repair, so a measure ask carrying the\n    thing alone reaches nothing. Only the line's own words decide: a fault said of the thing, with\n    nothing being planned about it (\"im thinking of getting a new boiler\" is a plan, and the price\n    of a new one is what that person wants).\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_the_answer_gave_a_time(&[memory.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z]+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_FAULT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_answer_gave_a_time(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the reply just before this line said how long something takes.");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__A_TIME_IN_A_REPLY(), "search", &[call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_thing(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The phrase naming the thing the chat is on, without the words for how it is, or \"\".\n\n    \"my washing machine has stopped\" is on the subject \"washing machine stopped\", and the thing is\n    the washing machine. A measure ask written with one word of it (\"how long will the machine\n    take\") reaches the row about how long machines last, where the phrase reaches the row filed for\n    the washing machine's repair: an entry keyed on a phrase is only reachable with the phrase.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z0-9]+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_A_FAULT(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_THE_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_MEASURE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("and"), py_str("for"), py_str("with")]), &word.clone())) } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    return mcall(&py_str(" "), "join", &[slice_or_raise(&kept.clone(), &Py::None, &Py::Int(4), &Py::None)]);
    Py::None
}
pub fn subject_word(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let mut newest: Py = Py::None;
    let mut own: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing the chat is on, in one word (\"speech\", \"dog\"), or \"\".\n\n    `topics.subject` is the phrase the conversation is about (\"worried speech\", \"getting a dog\"),\n    and the noun in it is the last content word that is not a feeling and not the verb of getting\n    it. The thing is what a measure ask is asked about, so the noun is what is needed, not the\n    phrase.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = pyrt::xrefs::xcall("memory.topics", "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    found = _the_noun_in(&[said.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_GETTING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    newest = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]) } else { Py::None };
    own = if py_truthy(&call_builtin("isinstance", &[newest.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("str", &[{ let __b = mcall(&{ let __b = newest.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    found = _the_noun_in(&[mcall(&mcall(&py_str(" "), "join", &[mcall(&own.clone(), "split", &[])]), "lower", &[])]);
    }
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACE_WORDS(), &geti(&found.clone(), &op_neg(&Py::Int(1))))) } }) {
    before = _the_record_before(&[memory.clone()]);
    if py_truthy(&before.clone()) {
    found = { let __b = _the_noun_in(&[mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&before.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[])]); if py_truthy(&__b) { __b } else { found.clone() } };
    }
    }
    return if py_truthy(&found.clone()) { geti(&found.clone(), &op_neg(&Py::Int(1))) } else { py_str("") };
    Py::None
}
pub fn _the_noun_in(args: &[Py]) -> Py {
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
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_list(vec![]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z]+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_THE_THING(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_MEASURE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_A_FAULT(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("the"), py_str("and"), py_str("for"), py_str("with"), py_str("some"), py_str("any"), py_str("new")]), &word.clone())) } } } } } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _the_record_before(args: &[Py]) -> Py {
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
    let mut newest: Py = Py::None;
    let mut now: Py = Py::None;
    let mut record: Py = Py::None;
    let mut records: Py = Py::None;
    let _ = py_str("The record the chat was on before the newest one, while it is the same conversation.");
    records = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut record: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&record.clone(), "get", &[py_str("text")]) } }) {  __out.push(record.clone()); } } py_list(__out) })() };
    newest = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]);
    now = call_builtin("str", &[{ let __b = mcall(&{ let __b = newest.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[records.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &now.clone()))) {
    return record.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn a_newer_thing(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut last: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The thing the line just before the ask said they need, or \"\".\n\n    \"my car failed its mot\" / \"it needs a new spring\" / \"how much would that cost\" is a question\n    about the spring: the subject the chat opened with is the MOT, and the price of the test is not\n    what is being asked. An English line puts the thing it is about at its end, so the last word of\n    the line before the ask is the newer thing, when it is a noun of its own and not what the\n    subject is for (\"it would be for the kids\" leaves the subject standing).\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&word.clone())) } }) {
    return py_str("");
    }
    last = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&last.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_PURPOSE(), "search", &[last.clone()]) } }) {
    return py_str("");
    }
    tail = mcall(&geti(&mcall(&last.clone(), "split", &[]), &op_neg(&Py::Int(1))), "strip", &[py_str(".,;:!?")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_VERBS(), &tail.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING_END(), &tail.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_POINTS_BACK(), &tail.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_FAULT(), &tail.clone())) } } } } } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_ASPECT_NOUNS(), &tail.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&tail.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&tail.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&word.clone(), &tail.clone())) } } } }) {
    return py_str("");
    }
    return tail.clone();
    Py::None
}
pub fn _the_trip_place(args: &[Py]) -> Py {
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
    let mut card: Py = Py::None;
    let mut card_at: Py = Py::None;
    let mut index: Py = Py::None;
    let mut known: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named_at: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The place the chat is going to, in the person's own spelling, or \"\".");
    known = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut place: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[place.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&places.clone()))) {
    return _the_place_the_bot_just_named(&[memory.clone()]);
    }
    place = geti(&places.clone(), &op_neg(&Py::Int(1)));
    said = mcall(&place.clone(), "lower", &[]);
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    named_at = op_neg(&Py::Int(1));
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &index.clone()), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])")), line.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__GOING_AWAY(), "search", &[line.clone()])))) {
    return py_str("");
    }
    named_at = index.clone();
    break;
    }
    if py_truthy(&op_lt(&named_at.clone(), &Py::Int(0))) {
    plan = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&plan.clone(), &said.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__GOING_AWAY(), "search", &[plan.clone()]) } }))) {
    return py_str("");
    }
    }
    let __unpk2 = py_unpack(&_the_opening_place_at(&[memory.clone()]), 2, None);
    card = __unpk2[0].clone();
    card_at = __unpk2[1].clone();
    if py_truthy(&{ let __b = card.clone(); if !py_truthy(&__b) { __b } else { op_gt(&card_at.clone(), &named_at.clone()) } }) {
    return card.clone();
    }
    return place.clone();
    card = _the_place_the_bot_just_named(&[memory.clone()]);
    if py_truthy(&{ let __b = card.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&mcall(&card.clone(), "lower", &[]), &said.clone())) } }) {
    return place.clone();
    }
    return py_str("");
    Py::None
}
pub fn _the_opening_place(args: &[Py]) -> Py {
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
    let _ = py_str("The place a card on the screen opens on, or \"\".\n\n    A card about one place says so in its first words: \"In Lisbon: Alfama on foot...\", \"In Rome\n    the cheap meal is pizza al taglio...\". That is the bot naming the place the whole answer is\n    about, which is as good as the person naming it for reading the next line.\n    ");
    return geti(&_the_opening_place_at(&[memory.clone()]), &Py::Int(0));
    Py::None
}
pub fn _the_opening_place_at(args: &[Py]) -> Py {
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
    let mut back: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut place: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("(the place a card on the screen opens on, the turn it came on), or (\"\", -1).");
    turns = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_tuple(vec![py_str(""), op_neg(&Py::Int(1))]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    opening = stdlib_call("re", "match", &[py_str("\\s*in\\s+(.{2,40}?)(?=[,:.!?]|\\s(?:the|a|an|is|it|you|we)\\b)"), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&opening.clone()))) {
    continue;
    }
    place = mcall(&mcall(&opening.clone(), "group", &[Py::Int(1)]), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&g__A_TIME_WORD(), &mcall(&place.clone(), "lower", &[])))) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("(?i)(?:the|a|an|your|my|his|her|their|our|this|that)\\b"), place.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&slice_or_raise(&place.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])))) {
    continue;
    }
    return py_tuple(vec![place.clone(), op_sub(&op_sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), &back.clone())]);
    }
    return py_tuple(vec![py_str(""), op_neg(&Py::Int(1))]);
    Py::None
}
pub fn _the_place_the_bot_just_named(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The place on the card the bot just gave, when no line in the chat named one.\n\n    \"tell me about lisbon\" names no trip - there is no going, no week, no booking - so the store\n    files no place and the food-and-cost follow-ups after the card lost the city: \"and the food\n    like there\" / \"is it expensive\" got the household shopping plan. The card itself says which\n    city the chat is on. A line that puts the person at home keeps this out: \"i live in rome\"\n    followed by \"is it expensive\" is not a holiday, and the card is what answers \"what's it like\n    there\" anyway.\n    ");
    place = _the_opening_place(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g__LIVING_THERE(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    return py_str("");
    }
    }
    return place.clone();
    Py::None
}
pub fn a_part_of_the_trip(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut earliest: Py = Py::None;
    let mut found: Py = Py::None;
    let mut found_: Py = Py::None;
    let mut matched: Py = Py::None;
    let mut part: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line as the question it asks about the place the chat is on, or \"\".\n\n    See A_TRIP_PART: \"and where should i eat\" after a trip to Rome is \"where should i eat in\n    rome\", and the card filed for that question is the one that answers it.\n    ");
    place = _the_trip_place(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_str("");
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return py_str("");
    }
    part = py_str("");
    words = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    earliest = Py::None;
    let __iter1 = py_iter_or_raise(&g_A_TRIP_PART());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    asked = __unpk2[1].clone();
    found = call_builtin("set", &[mcall(&pattern.clone(), "findall", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    at = call_builtin("min", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[said.clone()])) { let mut found_: Py = __item.clone(); __out.push(mcall(&found_.clone(), "start", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_is(&earliest.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_lt(&at.clone(), &earliest.clone()) } }) {
    part = asked.clone();
    earliest = at.clone();
    words = op_bitor(&call_builtin("set", &[stdlib_call("re", "split", &[py_str("[^a-z]+"), mcall(&py_str(" "), "join", &[found.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &found.clone());
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    return py_str("");
    }
    let __iter3 = py_iter_or_raise(&words.clone());
    for __item in __iter3 {
    matched = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b(?:a|an)\\s+"), &stdlib_call("re", "escape", &[matched.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    }
    let __iter4 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    word = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__ASK_FILLER(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &word.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("day"), py_str("days")]), &word.clone())); if !py_truthy(&__b) { __b } else { mcall(&part.clone(), "startswith", &[py_str("how many days")]) } }) {
    continue;
    }
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("place".to_string(), if py_truthy(&mcall(&place.clone(), "isupper", &[])) { mcall(&place.clone(), "lower", &[]) } else { place.clone() })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&part.clone(), "format", args))(&__args) };
    Py::None
}
pub fn asks_a_part_of_the_trip(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut place: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks about a part of the trip, whether it names the place or not.\n\n    \"how many days do i need\" after the card for a city is the days question for that city, and\n    the trip reader writes it out (\"how many days in berlin\") for the entry that answers it. The\n    same question with the city already in it - \"how many days in berlin\" as the person first\n    typed it, or as the reader wrote it - is the same ask, so the place can also come out of the\n    line before the rest is read as the part.\n    ");
    if py_truthy(&a_part_of_the_trip(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    place = _the_trip_place(&[memory.clone()]);
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    name = mcall(&place.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &name.clone())) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[a_part_of_the_trip(&[mcall(&said.clone(), "replace", &[name.clone(), py_str(" ")]), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_food_it_is_about(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut food: Py = Py::None;
    let mut line: Py = Py::None;
    let mut raised: Py = Py::None;
    let mut record: Py = Py::None;
    let _ = py_str("The food the conversation has moved to, when a bare line's \"it\" is that food.\n\n    \"i burnt the rice\" then \"can i still eat it\" is a question about the rice, and the store\n    holds it: the understanding engine has already read what the bare line leans on, and that\n    is `leans_on.subject`. The knowledge base's own scan for a dish does not know it - a food\n    with no plan of its own (rice, pasta, potatoes) is skipped over - so it hands back the last\n    dish it heard of, which after this chat is a cake two turns back, and the cake's \"3 days in\n    a tin\" is a keeping fact about the wrong food.\n\n    Two things stop it. The line naming the food itself means the entries can read it as it\n    stands, so nothing has to be put back in. And the line the food came from ruling it out of\n    the dish (\"i have no eggs\", \"no butter left for the cake\") is a limit on the dish, not a\n    move to another one: the dish is still in hand.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    record = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    food = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("subject")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&food.clone()))) {
    return py_str("");
    }
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &food.clone())) } }) {
    return py_str("");
    }
    raised = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    if py_truthy(&{ let __b = raised.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_LIMIT(), "search", &[raised.clone()]) } }) {
    return py_str("");
    }
    return food.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.named".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACTIVITY_WORDS" => Some(g_ACTIVITY_WORDS()),
            "AFTER_THE_NAME" => Some(g_AFTER_THE_NAME()),
            "AIM_VERB" => Some(g_AIM_VERB()),
            "ASKED_FOR_IDEAS" => Some(g_ASKED_FOR_IDEAS()),
            "ASKED_THE_MEASURE" => Some(g_ASKED_THE_MEASURE()),
            "ASKING" => Some(g_ASKING()),
            "ASKS_FOR_AN_ANSWER" => Some(g_ASKS_FOR_AN_ANSWER()),
            "ASPECT_ASKS" => Some(g_ASPECT_ASKS()),
            "ASPECT_FILLERS" => Some(g_ASPECT_FILLERS()),
            "ASPECT_NOUNS" => Some(g_ASPECT_NOUNS()),
            "A_BARE_JOB" => Some(g_A_BARE_JOB()),
            "A_CHOICE" => Some(g_A_CHOICE()),
            "A_DEADLINE_CLAUSE" => Some(g_A_DEADLINE_CLAUSE()),
            "A_DOING_NAMED" => Some(g_A_DOING_NAMED()),
            "A_FAULT" => Some(g_A_FAULT()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_FIGURE_ON_THE_END" => Some(g_A_FIGURE_ON_THE_END()),
            "A_FITTING" => Some(g_A_FITTING()),
            "A_GERUND" => Some(g_A_GERUND()),
            "A_GETTING" => Some(g_A_GETTING()),
            "A_GOAL" => Some(g_A_GOAL()),
            "A_HOW_GOOD" => Some(g_A_HOW_GOOD()),
            "A_HOW_OFTEN" => Some(g_A_HOW_OFTEN()),
            "A_LENGTH_STATED" => Some(g_A_LENGTH_STATED()),
            "A_MEASURE_AGAIN" => Some(g_A_MEASURE_AGAIN()),
            "A_NAME_ASK" => Some(g_A_NAME_ASK()),
            "A_PACE" => Some(g_A_PACE()),
            "A_PLACE" => Some(g_A_PLACE()),
            "A_PLAN_TO_THE_DOING" => Some(g_A_PLAN_TO_THE_DOING()),
            "A_PURPOSE" => Some(g_A_PURPOSE()),
            "A_THANK_YOU" => Some(g_A_THANK_YOU()),
            "A_TRIP_PART" => Some(g_A_TRIP_PART()),
            "A_VAGUE_JOB" => Some(g_A_VAGUE_JOB()),
            "BARE_ANOTHER" => Some(g_BARE_ANOTHER()),
            "BASE_DOING" => Some(g_BASE_DOING()),
            "BEST_TIME_TO_DO" => Some(g_BEST_TIME_TO_DO()),
            "CARE_NOT_THE_THING" => Some(g_CARE_NOT_THE_THING()),
            "CARE_VERBS" => Some(g_CARE_VERBS()),
            "CONTINUES" => Some(g_CONTINUES()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "DISTRESS" => Some(g_DISTRESS()),
            "DOING" => Some(g_DOING()),
            "DOING_BASE" => Some(g_DOING_BASE()),
            "FACT_FILLER" => Some(g_FACT_FILLER()),
            "FACT_MOST_WORDS" => Some(g_FACT_MOST_WORDS()),
            "FACT_OPEN" => Some(g_FACT_OPEN()),
            "FED_THINGS" => Some(g_FED_THINGS()),
            "FEELING_FILLER" => Some(g_FEELING_FILLER()),
            "FEELING_SAID_AS_A_PHRASE" => Some(g_FEELING_SAID_AS_A_PHRASE()),
            "GOAL_VERB" => Some(g_GOAL_VERB()),
            "IN_FRONT_OF_THE_NAME" => Some(g_IN_FRONT_OF_THE_NAME()),
            "IN_PROGRESS" => Some(g_IN_PROGRESS()),
            "JOINING" => Some(g_JOINING()),
            "JOURNEY" => Some(g_JOURNEY()),
            "JUST_A_NAME" => Some(g_JUST_A_NAME()),
            "LEAD" => Some(g_LEAD()),
            "LENGTH_FILLERS" => Some(g_LENGTH_FILLERS()),
            "LENGTH_KEY_WORDS" => Some(g_LENGTH_KEY_WORDS()),
            "LENGTH_NOT" => Some(g_LENGTH_NOT()),
            "LENGTH_WORDS" => Some(g_LENGTH_WORDS()),
            "LIGHT_DOING" => Some(g_LIGHT_DOING()),
            "LIMIT" => Some(g_LIMIT()),
            "MEASURE_ASK" => Some(g_MEASURE_ASK()),
            "MEASURE_VERBS" => Some(g_MEASURE_VERBS()),
            "MEASURE_WORDS" => Some(g_MEASURE_WORDS()),
            "MOST_WORDS" => Some(g_MOST_WORDS()),
            "MOST_WORDS_ASKED_ABOUT" => Some(g_MOST_WORDS_ASKED_ABOUT()),
            "NAMED_A_THING" => Some(g_NAMED_A_THING()),
            "NAME_WORD" => Some(g_NAME_WORD()),
            "NOT_A_NAME" => Some(g_NOT_A_NAME()),
            "NOT_A_SHELF" => Some(g_NOT_A_SHELF()),
            "NOT_A_THING_END" => Some(g_NOT_A_THING_END()),
            "NOT_GOT" => Some(g_NOT_GOT()),
            "NOT_THE_JOB" => Some(g_NOT_THE_JOB()),
            "NOT_THE_SUBJECT" => Some(g_NOT_THE_SUBJECT()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "NOT_THE_THING_ITSELF" => Some(g_NOT_THE_THING_ITSELF()),
            "NO_THANKS" => Some(g_NO_THANKS()),
            "OUTCOME_ASK" => Some(g_OUTCOME_ASK()),
            "PART_LEAD" => Some(g_PART_LEAD()),
            "PART_WORDS" => Some(g_PART_WORDS()),
            "PEOPLE_WORDS" => Some(g_PEOPLE_WORDS()),
            "PERSON_OPEN" => Some(g_PERSON_OPEN()),
            "PLACE_ARTICLES" => Some(g_PLACE_ARTICLES()),
            "PLACE_PAIRS" => Some(g_PLACE_PAIRS()),
            "PLACE_PREPOSITIONS" => Some(g_PLACE_PREPOSITIONS()),
            "PLACE_WORDS" => Some(g_PLACE_WORDS()),
            "POINTED_AT" => Some(g_POINTED_AT()),
            "POINTS_AT_A_THING" => Some(g_POINTS_AT_A_THING()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "PREP" => Some(g_PREP()),
            "PROGRESS_ASK" => Some(g_PROGRESS_ASK()),
            "PROGRESS_WORDS" => Some(g_PROGRESS_WORDS()),
            "QUANT" => Some(g_QUANT()),
            "SAID_NOT_A_FOOD" => Some(g_SAID_NOT_A_FOOD()),
            "SPAN_WORDS" => Some(g_SPAN_WORDS()),
            "STATE_WORDS" => Some(g_STATE_WORDS()),
            "TAKES_BACK" => Some(g_TAKES_BACK()),
            "TAKE_BACK_LEAD" => Some(g_TAKE_BACK_LEAD()),
            "TAKE_BACK_WHOLE" => Some(g_TAKE_BACK_WHOLE()),
            "THE_ASKING" => Some(g_THE_ASKING()),
            "THE_ASKING_TAIL" => Some(g_THE_ASKING_TAIL()),
            "THE_ASKING_VERB" => Some(g_THE_ASKING_VERB()),
            "THE_LENGTH_FOR_IT" => Some(g_THE_LENGTH_FOR_IT()),
            "TIMES_ONLY" => Some(g_TIMES_ONLY()),
            "VERBS" => Some(g_VERBS()),
            "VIEW_FRAME" => Some(g_VIEW_FRAME()),
            "VIEW_MANNER" => Some(g_VIEW_MANNER()),
            "VIEW_POSSESSIVE" => Some(g_VIEW_POSSESSIVE()),
            "WANTS_A_NEW_ONE" => Some(g_WANTS_A_NEW_ONE()),
            "WHERE_IT_STANDS" => Some(g_WHERE_IT_STANDS()),
            "WHO_X_IS" => Some(g_WHO_X_IS()),
            "_AND_AT_THE_START" => Some(g__AND_AT_THE_START()),
            "_ASK_FILLER" => Some(g__ASK_FILLER()),
            "_A_CONNECTIVE" => Some(g__A_CONNECTIVE()),
            "_A_TIME_IN_A_REPLY" => Some(g__A_TIME_IN_A_REPLY()),
            "_A_TIME_WORD" => Some(g__A_TIME_WORD()),
            "_GOING_AWAY" => Some(g__GOING_AWAY()),
            "_LIVING_THERE" => Some(g__LIVING_THERE()),
            "_NOT_A_PRONOUN" => Some(g__NOT_A_PRONOUN()),
            "_NOT_THE_PRACTICE" => Some(g__NOT_THE_PRACTICE()),
            "_THE_LIFE" => Some(g__THE_LIFE()),
            "_a_doing_word" => Some(py_func("_a_doing_word", |args: &[Py]| _a_doing_word(args))),
            "_a_length_ask" => Some(py_func("_a_length_ask", |args: &[Py]| _a_length_ask(args))),
            "_a_length_ask_of_the_aim" => Some(py_func("_a_length_ask_of_the_aim", |args: &[Py]| _a_length_ask_of_the_aim(args))),
            "_a_place_of_its_own" => Some(py_func("_a_place_of_its_own", |args: &[Py]| _a_place_of_its_own(args))),
            "_about_the_thing_in_the_place" => Some(py_func("_about_the_thing_in_the_place", |args: &[Py]| _about_the_thing_in_the_place(args))),
            "_an_aim_before_the_side_question" => Some(py_func("_an_aim_before_the_side_question", |args: &[Py]| _an_aim_before_the_side_question(args))),
            "_answers_a_length" => Some(py_func("_answers_a_length", |args: &[Py]| _answers_a_length(args))),
            "_answers_a_length_about" => Some(py_func("_answers_a_length_about", |args: &[Py]| _answers_a_length_about(args))),
            "_answers_a_pace" => Some(py_func("_answers_a_pace", |args: &[Py]| _answers_a_pace(args))),
            "_answers_the_bot" => Some(py_func("_answers_the_bot", |args: &[Py]| _answers_the_bot(args))),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "_looks_like_a_thing" => Some(py_func("_looks_like_a_thing", |args: &[Py]| _looks_like_a_thing(args))),
            "_no_thanks" => Some(py_func("_no_thanks", |args: &[Py]| _no_thanks(args))),
            "_not_got" => Some(py_func("_not_got", |args: &[Py]| _not_got(args))),
            "_only_a_particle" => Some(py_func("_only_a_particle", |args: &[Py]| _only_a_particle(args))),
            "_only_how_they_feel" => Some(py_func("_only_how_they_feel", |args: &[Py]| _only_how_they_feel(args))),
            "_only_the_measure" => Some(py_func("_only_the_measure", |args: &[Py]| _only_the_measure(args))),
            "_previous_ask" => Some(py_func("_previous_ask", |args: &[Py]| _previous_ask(args))),
            "_said_the_food" => Some(py_func("_said_the_food", |args: &[Py]| _said_the_food(args))),
            "_the_aim_and_its_length_question" => Some(py_func("_the_aim_and_its_length_question", |args: &[Py]| _the_aim_and_its_length_question(args))),
            "_the_answer_gave_a_time" => Some(py_func("_the_answer_gave_a_time", |args: &[Py]| _the_answer_gave_a_time(args))),
            "_the_answers_own_subject" => Some(py_func("_the_answers_own_subject", |args: &[Py]| _the_answers_own_subject(args))),
            "_the_aspect_asked" => Some(py_func("_the_aspect_asked", |args: &[Py]| _the_aspect_asked(args))),
            "_the_doing" => Some(py_func("_the_doing", |args: &[Py]| _the_doing(args))),
            "_the_doing_in_hand" => Some(py_func("_the_doing_in_hand", |args: &[Py]| _the_doing_in_hand(args))),
            "_the_doing_word" => Some(py_func("_the_doing_word", |args: &[Py]| _the_doing_word(args))),
            "_the_entry" => Some(py_func("_the_entry", |args: &[Py]| _the_entry(args))),
            "_the_forms" => Some(py_func("_the_forms", |args: &[Py]| _the_forms(args))),
            "_the_ing" => Some(py_func("_the_ing", |args: &[Py]| _the_ing(args))),
            "_the_job_before" => Some(py_func("_the_job_before", |args: &[Py]| _the_job_before(args))),
            "_the_length_entry_about" => Some(py_func("_the_length_entry_about", |args: &[Py]| _the_length_entry_about(args))),
            "_the_noun_in" => Some(py_func("_the_noun_in", |args: &[Py]| _the_noun_in(args))),
            "_the_opening_place" => Some(py_func("_the_opening_place", |args: &[Py]| _the_opening_place(args))),
            "_the_opening_place_at" => Some(py_func("_the_opening_place_at", |args: &[Py]| _the_opening_place_at(args))),
            "_the_persons_own_words" => Some(py_func("_the_persons_own_words", |args: &[Py]| _the_persons_own_words(args))),
            "_the_place_belongs_to_the_doing" => Some(py_func("_the_place_belongs_to_the_doing", |args: &[Py]| _the_place_belongs_to_the_doing(args))),
            "_the_place_phrase" => Some(py_func("_the_place_phrase", |args: &[Py]| _the_place_phrase(args))),
            "_the_place_the_bot_just_named" => Some(py_func("_the_place_the_bot_just_named", |args: &[Py]| _the_place_the_bot_just_named(args))),
            "_the_plan_said" => Some(py_func("_the_plan_said", |args: &[Py]| _the_plan_said(args))),
            "_the_record_before" => Some(py_func("_the_record_before", |args: &[Py]| _the_record_before(args))),
            "_the_rewrite_earns_its_place" => Some(py_func("_the_rewrite_earns_its_place", |args: &[Py]| _the_rewrite_earns_its_place(args))),
            "_the_thing_in_hand" => Some(py_func("_the_thing_in_hand", |args: &[Py]| _the_thing_in_hand(args))),
            "_the_thing_they_named" => Some(py_func("_the_thing_they_named", |args: &[Py]| _the_thing_they_named(args))),
            "_the_trip_place" => Some(py_func("_the_trip_place", |args: &[Py]| _the_trip_place(args))),
            "_thing_in_hand" => Some(py_func("_thing_in_hand", |args: &[Py]| _thing_in_hand(args))),
            "_what_the_chat_is_on" => Some(py_func("_what_the_chat_is_on", |args: &[Py]| _what_the_chat_is_on(args))),
            "_without_and_at_the_start" => Some(py_func("_without_and_at_the_start", |args: &[Py]| _without_and_at_the_start(args))),
            "a_faulty_thing" => Some(py_func("a_faulty_thing", |args: &[Py]| a_faulty_thing(args))),
            "a_figure_on_the_doing" => Some(py_func("a_figure_on_the_doing", |args: &[Py]| a_figure_on_the_doing(args))),
            "a_goal_measured" => Some(py_func("a_goal_measured", |args: &[Py]| a_goal_measured(args))),
            "a_length_about_an_aim_on_a_deadline" => Some(py_func("a_length_about_an_aim_on_a_deadline", |args: &[Py]| a_length_about_an_aim_on_a_deadline(args))),
            "a_measure_asked_again" => Some(py_func("a_measure_asked_again", |args: &[Py]| a_measure_asked_again(args))),
            "a_newer_thing" => Some(py_func("a_newer_thing", |args: &[Py]| a_newer_thing(args))),
            "a_part_of_the_trip" => Some(py_func("a_part_of_the_trip", |args: &[Py]| a_part_of_the_trip(args))),
            "about_it" => Some(py_func("about_it", |args: &[Py]| about_it(args))),
            "about_the_subject" => Some(py_func("about_the_subject", |args: &[Py]| about_the_subject(args))),
            "about_the_thing" => Some(py_func("about_the_thing", |args: &[Py]| about_the_thing(args))),
            "asked_for_a_name" => Some(py_func("asked_for_a_name", |args: &[Py]| asked_for_a_name(args))),
            "asks_a_part_of_the_trip" => Some(py_func("asks_a_part_of_the_trip", |args: &[Py]| asks_a_part_of_the_trip(args))),
            "care_reports" => Some(py_func("care_reports", |args: &[Py]| care_reports(args))),
            "carries_a_name" => Some(py_func("carries_a_name", |args: &[Py]| carries_a_name(args))),
            "follow_up" => Some(py_func("follow_up", |args: &[Py]| follow_up(args))),
            "is_a_progress_ask" => Some(py_func("is_a_progress_ask", |args: &[Py]| is_a_progress_ask(args))),
            "is_ack" => Some(g_is_ack()),
            "names_nothing_to_be_about" => Some(py_func("names_nothing_to_be_about", |args: &[Py]| names_nothing_to_be_about(args))),
            "reads_as_bad_news" => Some(g_reads_as_bad_news()),
            "reported_states" => Some(py_func("reported_states", |args: &[Py]| reported_states(args))),
            "stated_about_the_person" => Some(py_func("stated_about_the_person", |args: &[Py]| stated_about_the_person(args))),
            "subject_word" => Some(py_func("subject_word", |args: &[Py]| subject_word(args))),
            "subject_words" => Some(py_func("subject_words", |args: &[Py]| subject_words(args))),
            "takes_back" => Some(py_func("takes_back", |args: &[Py]| takes_back(args))),
            "the_best_time_to_do_it" => Some(py_func("the_best_time_to_do_it", |args: &[Py]| the_best_time_to_do_it(args))),
            "the_carried_topic" => Some(g_the_carried_topic()),
            "the_doing_the_plan_named" => Some(py_func("the_doing_the_plan_named", |args: &[Py]| the_doing_the_plan_named(args))),
            "the_duration_for_the_doing" => Some(py_func("the_duration_for_the_doing", |args: &[Py]| the_duration_for_the_doing(args))),
            "the_food_it_is_about" => Some(py_func("the_food_it_is_about", |args: &[Py]| the_food_it_is_about(args))),
            "the_name_given" => Some(py_func("the_name_given", |args: &[Py]| the_name_given(args))),
            "the_name_in" => Some(py_func("the_name_in", |args: &[Py]| the_name_in(args))),
            "the_one_asked_about" => Some(py_func("the_one_asked_about", |args: &[Py]| the_one_asked_about(args))),
            "the_subject" => Some(g_the_subject()),
            "the_thing" => Some(py_func("the_thing", |args: &[Py]| the_thing(args))),
            "the_thing_named" => Some(py_func("the_thing_named", |args: &[Py]| the_thing_named(args))),
            "thing" => Some(py_func("thing", |args: &[Py]| thing(args))),
            "earlier" => Some(pyrt::xrefs::xmodule("memory.holds")),
            "trouble" => Some(pyrt::xrefs::xmodule("compose.trouble")),
            "kinds" => Some(pyrt::xrefs::xmodule("understanding.kinds")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACTIVITY_WORDS" => s_ACTIVITY_WORDS(v),
            "AFTER_THE_NAME" => s_AFTER_THE_NAME(v),
            "AIM_VERB" => s_AIM_VERB(v),
            "ASKED_FOR_IDEAS" => s_ASKED_FOR_IDEAS(v),
            "ASKED_THE_MEASURE" => s_ASKED_THE_MEASURE(v),
            "ASKING" => s_ASKING(v),
            "ASKS_FOR_AN_ANSWER" => s_ASKS_FOR_AN_ANSWER(v),
            "ASPECT_ASKS" => s_ASPECT_ASKS(v),
            "ASPECT_FILLERS" => s_ASPECT_FILLERS(v),
            "ASPECT_NOUNS" => s_ASPECT_NOUNS(v),
            "A_BARE_JOB" => s_A_BARE_JOB(v),
            "A_CHOICE" => s_A_CHOICE(v),
            "A_DEADLINE_CLAUSE" => s_A_DEADLINE_CLAUSE(v),
            "A_DOING_NAMED" => s_A_DOING_NAMED(v),
            "A_FAULT" => s_A_FAULT(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_FIGURE_ON_THE_END" => s_A_FIGURE_ON_THE_END(v),
            "A_FITTING" => s_A_FITTING(v),
            "A_GERUND" => s_A_GERUND(v),
            "A_GETTING" => s_A_GETTING(v),
            "A_GOAL" => s_A_GOAL(v),
            "A_HOW_GOOD" => s_A_HOW_GOOD(v),
            "A_HOW_OFTEN" => s_A_HOW_OFTEN(v),
            "A_LENGTH_STATED" => s_A_LENGTH_STATED(v),
            "A_MEASURE_AGAIN" => s_A_MEASURE_AGAIN(v),
            "A_NAME_ASK" => s_A_NAME_ASK(v),
            "A_PACE" => s_A_PACE(v),
            "A_PLACE" => s_A_PLACE(v),
            "A_PLAN_TO_THE_DOING" => s_A_PLAN_TO_THE_DOING(v),
            "A_PURPOSE" => s_A_PURPOSE(v),
            "A_THANK_YOU" => s_A_THANK_YOU(v),
            "A_TRIP_PART" => s_A_TRIP_PART(v),
            "A_VAGUE_JOB" => s_A_VAGUE_JOB(v),
            "BARE_ANOTHER" => s_BARE_ANOTHER(v),
            "BASE_DOING" => s_BASE_DOING(v),
            "BEST_TIME_TO_DO" => s_BEST_TIME_TO_DO(v),
            "CARE_NOT_THE_THING" => s_CARE_NOT_THE_THING(v),
            "CARE_VERBS" => s_CARE_VERBS(v),
            "CONTINUES" => s_CONTINUES(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "DISTRESS" => s_DISTRESS(v),
            "DOING" => s_DOING(v),
            "DOING_BASE" => s_DOING_BASE(v),
            "FACT_FILLER" => s_FACT_FILLER(v),
            "FACT_MOST_WORDS" => s_FACT_MOST_WORDS(v),
            "FACT_OPEN" => s_FACT_OPEN(v),
            "FED_THINGS" => s_FED_THINGS(v),
            "FEELING_FILLER" => s_FEELING_FILLER(v),
            "FEELING_SAID_AS_A_PHRASE" => s_FEELING_SAID_AS_A_PHRASE(v),
            "GOAL_VERB" => s_GOAL_VERB(v),
            "IN_FRONT_OF_THE_NAME" => s_IN_FRONT_OF_THE_NAME(v),
            "IN_PROGRESS" => s_IN_PROGRESS(v),
            "JOINING" => s_JOINING(v),
            "JOURNEY" => s_JOURNEY(v),
            "JUST_A_NAME" => s_JUST_A_NAME(v),
            "LEAD" => s_LEAD(v),
            "LENGTH_FILLERS" => s_LENGTH_FILLERS(v),
            "LENGTH_KEY_WORDS" => s_LENGTH_KEY_WORDS(v),
            "LENGTH_NOT" => s_LENGTH_NOT(v),
            "LENGTH_WORDS" => s_LENGTH_WORDS(v),
            "LIGHT_DOING" => s_LIGHT_DOING(v),
            "LIMIT" => s_LIMIT(v),
            "MEASURE_ASK" => s_MEASURE_ASK(v),
            "MEASURE_VERBS" => s_MEASURE_VERBS(v),
            "MEASURE_WORDS" => s_MEASURE_WORDS(v),
            "MOST_WORDS" => s_MOST_WORDS(v),
            "MOST_WORDS_ASKED_ABOUT" => s_MOST_WORDS_ASKED_ABOUT(v),
            "NAMED_A_THING" => s_NAMED_A_THING(v),
            "NAME_WORD" => s_NAME_WORD(v),
            "NOT_A_NAME" => s_NOT_A_NAME(v),
            "NOT_A_SHELF" => s_NOT_A_SHELF(v),
            "NOT_A_THING_END" => s_NOT_A_THING_END(v),
            "NOT_GOT" => s_NOT_GOT(v),
            "NOT_THE_JOB" => s_NOT_THE_JOB(v),
            "NOT_THE_SUBJECT" => s_NOT_THE_SUBJECT(v),
            "NOT_THE_THING" => s_NOT_THE_THING(v),
            "NOT_THE_THING_ITSELF" => s_NOT_THE_THING_ITSELF(v),
            "NO_THANKS" => s_NO_THANKS(v),
            "OUTCOME_ASK" => s_OUTCOME_ASK(v),
            "PART_LEAD" => s_PART_LEAD(v),
            "PART_WORDS" => s_PART_WORDS(v),
            "PEOPLE_WORDS" => s_PEOPLE_WORDS(v),
            "PERSON_OPEN" => s_PERSON_OPEN(v),
            "PLACE_ARTICLES" => s_PLACE_ARTICLES(v),
            "PLACE_PAIRS" => s_PLACE_PAIRS(v),
            "PLACE_PREPOSITIONS" => s_PLACE_PREPOSITIONS(v),
            "PLACE_WORDS" => s_PLACE_WORDS(v),
            "POINTED_AT" => s_POINTED_AT(v),
            "POINTS_AT_A_THING" => s_POINTS_AT_A_THING(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "PREP" => s_PREP(v),
            "PROGRESS_ASK" => s_PROGRESS_ASK(v),
            "PROGRESS_WORDS" => s_PROGRESS_WORDS(v),
            "QUANT" => s_QUANT(v),
            "SAID_NOT_A_FOOD" => s_SAID_NOT_A_FOOD(v),
            "SPAN_WORDS" => s_SPAN_WORDS(v),
            "STATE_WORDS" => s_STATE_WORDS(v),
            "TAKES_BACK" => s_TAKES_BACK(v),
            "TAKE_BACK_LEAD" => s_TAKE_BACK_LEAD(v),
            "TAKE_BACK_WHOLE" => s_TAKE_BACK_WHOLE(v),
            "THE_ASKING" => s_THE_ASKING(v),
            "THE_ASKING_TAIL" => s_THE_ASKING_TAIL(v),
            "THE_ASKING_VERB" => s_THE_ASKING_VERB(v),
            "THE_LENGTH_FOR_IT" => s_THE_LENGTH_FOR_IT(v),
            "TIMES_ONLY" => s_TIMES_ONLY(v),
            "VERBS" => s_VERBS(v),
            "VIEW_FRAME" => s_VIEW_FRAME(v),
            "VIEW_MANNER" => s_VIEW_MANNER(v),
            "VIEW_POSSESSIVE" => s_VIEW_POSSESSIVE(v),
            "WANTS_A_NEW_ONE" => s_WANTS_A_NEW_ONE(v),
            "WHERE_IT_STANDS" => s_WHERE_IT_STANDS(v),
            "WHO_X_IS" => s_WHO_X_IS(v),
            "_AND_AT_THE_START" => s__AND_AT_THE_START(v),
            "_ASK_FILLER" => s__ASK_FILLER(v),
            "_A_CONNECTIVE" => s__A_CONNECTIVE(v),
            "_A_TIME_IN_A_REPLY" => s__A_TIME_IN_A_REPLY(v),
            "_A_TIME_WORD" => s__A_TIME_WORD(v),
            "_GOING_AWAY" => s__GOING_AWAY(v),
            "_LIVING_THERE" => s__LIVING_THERE(v),
            "_NOT_A_PRONOUN" => s__NOT_A_PRONOUN(v),
            "_NOT_THE_PRACTICE" => s__NOT_THE_PRACTICE(v),
            "_THE_LIFE" => s__THE_LIFE(v),
            "is_ack" => s_is_ack(v),
            "reads_as_bad_news" => s_reads_as_bad_news(v),
            "the_carried_topic" => s_the_carried_topic(v),
            "the_subject" => s_the_subject(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.named", module_obj);
    pyrt::xrefs::reg_fn("memory.named", "_answers_the_bot", py_func("_answers_the_bot", |args: &[Py]| _answers_the_bot(args)));
    pyrt::xrefs::reg_fn("memory.named", "_previous_ask", py_func("_previous_ask", |args: &[Py]| _previous_ask(args)));
    pyrt::xrefs::reg_fn("memory.named", "takes_back", py_func("takes_back", |args: &[Py]| takes_back(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_one_asked_about", py_func("the_one_asked_about", |args: &[Py]| the_one_asked_about(args)));
    pyrt::xrefs::reg_fn("memory.named", "thing", py_func("thing", |args: &[Py]| thing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_looks_like_a_thing", py_func("_looks_like_a_thing", |args: &[Py]| _looks_like_a_thing(args)));
    pyrt::xrefs::reg_fn("memory.named", "carries_a_name", py_func("carries_a_name", |args: &[Py]| carries_a_name(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_name_in", py_func("the_name_in", |args: &[Py]| the_name_in(args)));
    pyrt::xrefs::reg_fn("memory.named", "asked_for_a_name", py_func("asked_for_a_name", |args: &[Py]| asked_for_a_name(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_name_given", py_func("the_name_given", |args: &[Py]| the_name_given(args)));
    pyrt::xrefs::reg_fn("memory.named", "_not_got", py_func("_not_got", |args: &[Py]| _not_got(args)));
    pyrt::xrefs::reg_fn("memory.named", "_said_the_food", py_func("_said_the_food", |args: &[Py]| _said_the_food(args)));
    pyrt::xrefs::reg_fn("memory.named", "_last_reply", py_func("_last_reply", |args: &[Py]| _last_reply(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_thing_named", py_func("the_thing_named", |args: &[Py]| the_thing_named(args)));
    pyrt::xrefs::reg_fn("memory.named", "about_the_thing", py_func("about_the_thing", |args: &[Py]| about_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_thing_they_named", py_func("_the_thing_they_named", |args: &[Py]| _the_thing_they_named(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_measure_asked_again", py_func("a_measure_asked_again", |args: &[Py]| a_measure_asked_again(args)));
    pyrt::xrefs::reg_fn("memory.named", "_a_length_ask", py_func("_a_length_ask", |args: &[Py]| _a_length_ask(args)));
    pyrt::xrefs::reg_fn("memory.named", "_a_length_ask_of_the_aim", py_func("_a_length_ask_of_the_aim", |args: &[Py]| _a_length_ask_of_the_aim(args)));
    pyrt::xrefs::reg_fn("memory.named", "_answers_a_pace", py_func("_answers_a_pace", |args: &[Py]| _answers_a_pace(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_entry", py_func("_the_entry", |args: &[Py]| _the_entry(args)));
    pyrt::xrefs::reg_fn("memory.named", "_answers_a_length", py_func("_answers_a_length", |args: &[Py]| _answers_a_length(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_length_entry_about", py_func("_the_length_entry_about", |args: &[Py]| _the_length_entry_about(args)));
    pyrt::xrefs::reg_fn("memory.named", "_answers_a_length_about", py_func("_answers_a_length_about", |args: &[Py]| _answers_a_length_about(args)));
    pyrt::xrefs::reg_fn("memory.named", "_an_aim_before_the_side_question", py_func("_an_aim_before_the_side_question", |args: &[Py]| _an_aim_before_the_side_question(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_aim_and_its_length_question", py_func("_the_aim_and_its_length_question", |args: &[Py]| _the_aim_and_its_length_question(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_goal_measured", py_func("a_goal_measured", |args: &[Py]| a_goal_measured(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_length_about_an_aim_on_a_deadline", py_func("a_length_about_an_aim_on_a_deadline", |args: &[Py]| a_length_about_an_aim_on_a_deadline(args)));
    pyrt::xrefs::reg_fn("memory.named", "_no_thanks", py_func("_no_thanks", |args: &[Py]| _no_thanks(args)));
    pyrt::xrefs::reg_fn("memory.named", "follow_up", py_func("follow_up", |args: &[Py]| follow_up(args)));
    pyrt::xrefs::reg_fn("memory.named", "names_nothing_to_be_about", py_func("names_nothing_to_be_about", |args: &[Py]| names_nothing_to_be_about(args)));
    pyrt::xrefs::reg_fn("memory.named", "_only_how_they_feel", py_func("_only_how_they_feel", |args: &[Py]| _only_how_they_feel(args)));
    pyrt::xrefs::reg_fn("memory.named", "about_it", py_func("about_it", |args: &[Py]| about_it(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_figure_on_the_doing", py_func("a_figure_on_the_doing", |args: &[Py]| a_figure_on_the_doing(args)));
    pyrt::xrefs::reg_fn("memory.named", "is_a_progress_ask", py_func("is_a_progress_ask", |args: &[Py]| is_a_progress_ask(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_doing", py_func("_the_doing", |args: &[Py]| _the_doing(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_doing_the_plan_named", py_func("the_doing_the_plan_named", |args: &[Py]| the_doing_the_plan_named(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_plan_said", py_func("_the_plan_said", |args: &[Py]| _the_plan_said(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_ing", py_func("_the_ing", |args: &[Py]| _the_ing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_doing_in_hand", py_func("_the_doing_in_hand", |args: &[Py]| _the_doing_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_duration_for_the_doing", py_func("the_duration_for_the_doing", |args: &[Py]| the_duration_for_the_doing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_place_belongs_to_the_doing", py_func("_the_place_belongs_to_the_doing", |args: &[Py]| _the_place_belongs_to_the_doing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_aspect_asked", py_func("_the_aspect_asked", |args: &[Py]| _the_aspect_asked(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_job_before", py_func("_the_job_before", |args: &[Py]| _the_job_before(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_best_time_to_do_it", py_func("the_best_time_to_do_it", |args: &[Py]| the_best_time_to_do_it(args)));
    pyrt::xrefs::reg_fn("memory.named", "_only_the_measure", py_func("_only_the_measure", |args: &[Py]| _only_the_measure(args)));
    pyrt::xrefs::reg_fn("memory.named", "about_the_subject", py_func("about_the_subject", |args: &[Py]| about_the_subject(args)));
    pyrt::xrefs::reg_fn("memory.named", "_without_and_at_the_start", py_func("_without_and_at_the_start", |args: &[Py]| _without_and_at_the_start(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_forms", py_func("_the_forms", |args: &[Py]| _the_forms(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_doing_word", py_func("_the_doing_word", |args: &[Py]| _the_doing_word(args)));
    pyrt::xrefs::reg_fn("memory.named", "_a_doing_word", py_func("_a_doing_word", |args: &[Py]| _a_doing_word(args)));
    pyrt::xrefs::reg_fn("memory.named", "_what_the_chat_is_on", py_func("_what_the_chat_is_on", |args: &[Py]| _what_the_chat_is_on(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_thing_in_hand", py_func("_the_thing_in_hand", |args: &[Py]| _the_thing_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_place_phrase", py_func("_the_place_phrase", |args: &[Py]| _the_place_phrase(args)));
    pyrt::xrefs::reg_fn("memory.named", "_about_the_thing_in_the_place", py_func("_about_the_thing_in_the_place", |args: &[Py]| _about_the_thing_in_the_place(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_rewrite_earns_its_place", py_func("_the_rewrite_earns_its_place", |args: &[Py]| _the_rewrite_earns_its_place(args)));
    pyrt::xrefs::reg_fn("memory.named", "_thing_in_hand", py_func("_thing_in_hand", |args: &[Py]| _thing_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.named", "_only_a_particle", py_func("_only_a_particle", |args: &[Py]| _only_a_particle(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_answers_own_subject", py_func("_the_answers_own_subject", |args: &[Py]| _the_answers_own_subject(args)));
    pyrt::xrefs::reg_fn("memory.named", "reported_states", py_func("reported_states", |args: &[Py]| reported_states(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_persons_own_words", py_func("_the_persons_own_words", |args: &[Py]| _the_persons_own_words(args)));
    pyrt::xrefs::reg_fn("memory.named", "stated_about_the_person", py_func("stated_about_the_person", |args: &[Py]| stated_about_the_person(args)));
    pyrt::xrefs::reg_fn("memory.named", "_a_place_of_its_own", py_func("_a_place_of_its_own", |args: &[Py]| _a_place_of_its_own(args)));
    pyrt::xrefs::reg_fn("memory.named", "subject_words", py_func("subject_words", |args: &[Py]| subject_words(args)));
    pyrt::xrefs::reg_fn("memory.named", "care_reports", py_func("care_reports", |args: &[Py]| care_reports(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_faulty_thing", py_func("a_faulty_thing", |args: &[Py]| a_faulty_thing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_answer_gave_a_time", py_func("_the_answer_gave_a_time", |args: &[Py]| _the_answer_gave_a_time(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_thing", py_func("the_thing", |args: &[Py]| the_thing(args)));
    pyrt::xrefs::reg_fn("memory.named", "subject_word", py_func("subject_word", |args: &[Py]| subject_word(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_noun_in", py_func("_the_noun_in", |args: &[Py]| _the_noun_in(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_record_before", py_func("_the_record_before", |args: &[Py]| _the_record_before(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_newer_thing", py_func("a_newer_thing", |args: &[Py]| a_newer_thing(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_trip_place", py_func("_the_trip_place", |args: &[Py]| _the_trip_place(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_opening_place", py_func("_the_opening_place", |args: &[Py]| _the_opening_place(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_opening_place_at", py_func("_the_opening_place_at", |args: &[Py]| _the_opening_place_at(args)));
    pyrt::xrefs::reg_fn("memory.named", "_the_place_the_bot_just_named", py_func("_the_place_the_bot_just_named", |args: &[Py]| _the_place_the_bot_just_named(args)));
    pyrt::xrefs::reg_fn("memory.named", "a_part_of_the_trip", py_func("a_part_of_the_trip", |args: &[Py]| a_part_of_the_trip(args)));
    pyrt::xrefs::reg_fn("memory.named", "asks_a_part_of_the_trip", py_func("asks_a_part_of_the_trip", |args: &[Py]| asks_a_part_of_the_trip(args)));
    pyrt::xrefs::reg_fn("memory.named", "the_food_it_is_about", py_func("the_food_it_is_about", |args: &[Py]| the_food_it_is_about(args)));
    pyrt::xrefs::reg_access("memory.named", "ACTIVITY_WORDS", g_ACTIVITY_WORDS, s_ACTIVITY_WORDS);
    pyrt::xrefs::reg_access("memory.named", "AFTER_THE_NAME", g_AFTER_THE_NAME, s_AFTER_THE_NAME);
    pyrt::xrefs::reg_access("memory.named", "AIM_VERB", g_AIM_VERB, s_AIM_VERB);
    pyrt::xrefs::reg_access("memory.named", "ASKED_FOR_IDEAS", g_ASKED_FOR_IDEAS, s_ASKED_FOR_IDEAS);
    pyrt::xrefs::reg_access("memory.named", "ASKED_THE_MEASURE", g_ASKED_THE_MEASURE, s_ASKED_THE_MEASURE);
    pyrt::xrefs::reg_access("memory.named", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.named", "ASKS_FOR_AN_ANSWER", g_ASKS_FOR_AN_ANSWER, s_ASKS_FOR_AN_ANSWER);
    pyrt::xrefs::reg_access("memory.named", "ASPECT_ASKS", g_ASPECT_ASKS, s_ASPECT_ASKS);
    pyrt::xrefs::reg_access("memory.named", "ASPECT_FILLERS", g_ASPECT_FILLERS, s_ASPECT_FILLERS);
    pyrt::xrefs::reg_access("memory.named", "ASPECT_NOUNS", g_ASPECT_NOUNS, s_ASPECT_NOUNS);
    pyrt::xrefs::reg_access("memory.named", "A_BARE_JOB", g_A_BARE_JOB, s_A_BARE_JOB);
    pyrt::xrefs::reg_access("memory.named", "A_CHOICE", g_A_CHOICE, s_A_CHOICE);
    pyrt::xrefs::reg_access("memory.named", "A_DEADLINE_CLAUSE", g_A_DEADLINE_CLAUSE, s_A_DEADLINE_CLAUSE);
    pyrt::xrefs::reg_access("memory.named", "A_DOING_NAMED", g_A_DOING_NAMED, s_A_DOING_NAMED);
    pyrt::xrefs::reg_access("memory.named", "A_FAULT", g_A_FAULT, s_A_FAULT);
    pyrt::xrefs::reg_access("memory.named", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.named", "A_FIGURE_ON_THE_END", g_A_FIGURE_ON_THE_END, s_A_FIGURE_ON_THE_END);
    pyrt::xrefs::reg_access("memory.named", "A_FITTING", g_A_FITTING, s_A_FITTING);
    pyrt::xrefs::reg_access("memory.named", "A_GERUND", g_A_GERUND, s_A_GERUND);
    pyrt::xrefs::reg_access("memory.named", "A_GETTING", g_A_GETTING, s_A_GETTING);
    pyrt::xrefs::reg_access("memory.named", "A_GOAL", g_A_GOAL, s_A_GOAL);
    pyrt::xrefs::reg_access("memory.named", "A_HOW_GOOD", g_A_HOW_GOOD, s_A_HOW_GOOD);
    pyrt::xrefs::reg_access("memory.named", "A_HOW_OFTEN", g_A_HOW_OFTEN, s_A_HOW_OFTEN);
    pyrt::xrefs::reg_access("memory.named", "A_LENGTH_STATED", g_A_LENGTH_STATED, s_A_LENGTH_STATED);
    pyrt::xrefs::reg_access("memory.named", "A_MEASURE_AGAIN", g_A_MEASURE_AGAIN, s_A_MEASURE_AGAIN);
    pyrt::xrefs::reg_access("memory.named", "A_NAME_ASK", g_A_NAME_ASK, s_A_NAME_ASK);
    pyrt::xrefs::reg_access("memory.named", "A_PACE", g_A_PACE, s_A_PACE);
    pyrt::xrefs::reg_access("memory.named", "A_PLACE", g_A_PLACE, s_A_PLACE);
    pyrt::xrefs::reg_access("memory.named", "A_PLAN_TO_THE_DOING", g_A_PLAN_TO_THE_DOING, s_A_PLAN_TO_THE_DOING);
    pyrt::xrefs::reg_access("memory.named", "A_PURPOSE", g_A_PURPOSE, s_A_PURPOSE);
    pyrt::xrefs::reg_access("memory.named", "A_THANK_YOU", g_A_THANK_YOU, s_A_THANK_YOU);
    pyrt::xrefs::reg_access("memory.named", "A_TRIP_PART", g_A_TRIP_PART, s_A_TRIP_PART);
    pyrt::xrefs::reg_access("memory.named", "A_VAGUE_JOB", g_A_VAGUE_JOB, s_A_VAGUE_JOB);
    pyrt::xrefs::reg_access("memory.named", "BARE_ANOTHER", g_BARE_ANOTHER, s_BARE_ANOTHER);
    pyrt::xrefs::reg_access("memory.named", "BASE_DOING", g_BASE_DOING, s_BASE_DOING);
    pyrt::xrefs::reg_access("memory.named", "BEST_TIME_TO_DO", g_BEST_TIME_TO_DO, s_BEST_TIME_TO_DO);
    pyrt::xrefs::reg_access("memory.named", "CARE_NOT_THE_THING", g_CARE_NOT_THE_THING, s_CARE_NOT_THE_THING);
    pyrt::xrefs::reg_access("memory.named", "CARE_VERBS", g_CARE_VERBS, s_CARE_VERBS);
    pyrt::xrefs::reg_access("memory.named", "CONTINUES", g_CONTINUES, s_CONTINUES);
    pyrt::xrefs::reg_access("memory.named", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("memory.named", "DISTRESS", g_DISTRESS, s_DISTRESS);
    pyrt::xrefs::reg_access("memory.named", "DOING", g_DOING, s_DOING);
    pyrt::xrefs::reg_access("memory.named", "DOING_BASE", g_DOING_BASE, s_DOING_BASE);
    pyrt::xrefs::reg_access("memory.named", "FACT_FILLER", g_FACT_FILLER, s_FACT_FILLER);
    pyrt::xrefs::reg_access("memory.named", "FACT_MOST_WORDS", g_FACT_MOST_WORDS, s_FACT_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.named", "FACT_OPEN", g_FACT_OPEN, s_FACT_OPEN);
    pyrt::xrefs::reg_access("memory.named", "FED_THINGS", g_FED_THINGS, s_FED_THINGS);
    pyrt::xrefs::reg_access("memory.named", "FEELING_FILLER", g_FEELING_FILLER, s_FEELING_FILLER);
    pyrt::xrefs::reg_access("memory.named", "FEELING_SAID_AS_A_PHRASE", g_FEELING_SAID_AS_A_PHRASE, s_FEELING_SAID_AS_A_PHRASE);
    pyrt::xrefs::reg_access("memory.named", "GOAL_VERB", g_GOAL_VERB, s_GOAL_VERB);
    pyrt::xrefs::reg_access("memory.named", "IN_FRONT_OF_THE_NAME", g_IN_FRONT_OF_THE_NAME, s_IN_FRONT_OF_THE_NAME);
    pyrt::xrefs::reg_access("memory.named", "IN_PROGRESS", g_IN_PROGRESS, s_IN_PROGRESS);
    pyrt::xrefs::reg_access("memory.named", "JOINING", g_JOINING, s_JOINING);
    pyrt::xrefs::reg_access("memory.named", "JOURNEY", g_JOURNEY, s_JOURNEY);
    pyrt::xrefs::reg_access("memory.named", "JUST_A_NAME", g_JUST_A_NAME, s_JUST_A_NAME);
    pyrt::xrefs::reg_access("memory.named", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.named", "LENGTH_FILLERS", g_LENGTH_FILLERS, s_LENGTH_FILLERS);
    pyrt::xrefs::reg_access("memory.named", "LENGTH_KEY_WORDS", g_LENGTH_KEY_WORDS, s_LENGTH_KEY_WORDS);
    pyrt::xrefs::reg_access("memory.named", "LENGTH_NOT", g_LENGTH_NOT, s_LENGTH_NOT);
    pyrt::xrefs::reg_access("memory.named", "LENGTH_WORDS", g_LENGTH_WORDS, s_LENGTH_WORDS);
    pyrt::xrefs::reg_access("memory.named", "LIGHT_DOING", g_LIGHT_DOING, s_LIGHT_DOING);
    pyrt::xrefs::reg_access("memory.named", "LIMIT", g_LIMIT, s_LIMIT);
    pyrt::xrefs::reg_access("memory.named", "MEASURE_ASK", g_MEASURE_ASK, s_MEASURE_ASK);
    pyrt::xrefs::reg_access("memory.named", "MEASURE_VERBS", g_MEASURE_VERBS, s_MEASURE_VERBS);
    pyrt::xrefs::reg_access("memory.named", "MEASURE_WORDS", g_MEASURE_WORDS, s_MEASURE_WORDS);
    pyrt::xrefs::reg_access("memory.named", "MOST_WORDS", g_MOST_WORDS, s_MOST_WORDS);
    pyrt::xrefs::reg_access("memory.named", "MOST_WORDS_ASKED_ABOUT", g_MOST_WORDS_ASKED_ABOUT, s_MOST_WORDS_ASKED_ABOUT);
    pyrt::xrefs::reg_access("memory.named", "NAMED_A_THING", g_NAMED_A_THING, s_NAMED_A_THING);
    pyrt::xrefs::reg_access("memory.named", "NAME_WORD", g_NAME_WORD, s_NAME_WORD);
    pyrt::xrefs::reg_access("memory.named", "NOT_A_NAME", g_NOT_A_NAME, s_NOT_A_NAME);
    pyrt::xrefs::reg_access("memory.named", "NOT_A_SHELF", g_NOT_A_SHELF, s_NOT_A_SHELF);
    pyrt::xrefs::reg_access("memory.named", "NOT_A_THING_END", g_NOT_A_THING_END, s_NOT_A_THING_END);
    pyrt::xrefs::reg_access("memory.named", "NOT_GOT", g_NOT_GOT, s_NOT_GOT);
    pyrt::xrefs::reg_access("memory.named", "NOT_THE_JOB", g_NOT_THE_JOB, s_NOT_THE_JOB);
    pyrt::xrefs::reg_access("memory.named", "NOT_THE_SUBJECT", g_NOT_THE_SUBJECT, s_NOT_THE_SUBJECT);
    pyrt::xrefs::reg_access("memory.named", "NOT_THE_THING", g_NOT_THE_THING, s_NOT_THE_THING);
    pyrt::xrefs::reg_access("memory.named", "NOT_THE_THING_ITSELF", g_NOT_THE_THING_ITSELF, s_NOT_THE_THING_ITSELF);
    pyrt::xrefs::reg_access("memory.named", "NO_THANKS", g_NO_THANKS, s_NO_THANKS);
    pyrt::xrefs::reg_access("memory.named", "OUTCOME_ASK", g_OUTCOME_ASK, s_OUTCOME_ASK);
    pyrt::xrefs::reg_access("memory.named", "PART_LEAD", g_PART_LEAD, s_PART_LEAD);
    pyrt::xrefs::reg_access("memory.named", "PART_WORDS", g_PART_WORDS, s_PART_WORDS);
    pyrt::xrefs::reg_access("memory.named", "PEOPLE_WORDS", g_PEOPLE_WORDS, s_PEOPLE_WORDS);
    pyrt::xrefs::reg_access("memory.named", "PERSON_OPEN", g_PERSON_OPEN, s_PERSON_OPEN);
    pyrt::xrefs::reg_access("memory.named", "PLACE_ARTICLES", g_PLACE_ARTICLES, s_PLACE_ARTICLES);
    pyrt::xrefs::reg_access("memory.named", "PLACE_PAIRS", g_PLACE_PAIRS, s_PLACE_PAIRS);
    pyrt::xrefs::reg_access("memory.named", "PLACE_PREPOSITIONS", g_PLACE_PREPOSITIONS, s_PLACE_PREPOSITIONS);
    pyrt::xrefs::reg_access("memory.named", "PLACE_WORDS", g_PLACE_WORDS, s_PLACE_WORDS);
    pyrt::xrefs::reg_access("memory.named", "POINTED_AT", g_POINTED_AT, s_POINTED_AT);
    pyrt::xrefs::reg_access("memory.named", "POINTS_AT_A_THING", g_POINTS_AT_A_THING, s_POINTS_AT_A_THING);
    pyrt::xrefs::reg_access("memory.named", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.named", "PREP", g_PREP, s_PREP);
    pyrt::xrefs::reg_access("memory.named", "PROGRESS_ASK", g_PROGRESS_ASK, s_PROGRESS_ASK);
    pyrt::xrefs::reg_access("memory.named", "PROGRESS_WORDS", g_PROGRESS_WORDS, s_PROGRESS_WORDS);
    pyrt::xrefs::reg_access("memory.named", "QUANT", g_QUANT, s_QUANT);
    pyrt::xrefs::reg_access("memory.named", "SAID_NOT_A_FOOD", g_SAID_NOT_A_FOOD, s_SAID_NOT_A_FOOD);
    pyrt::xrefs::reg_access("memory.named", "SPAN_WORDS", g_SPAN_WORDS, s_SPAN_WORDS);
    pyrt::xrefs::reg_access("memory.named", "STATE_WORDS", g_STATE_WORDS, s_STATE_WORDS);
    pyrt::xrefs::reg_access("memory.named", "TAKES_BACK", g_TAKES_BACK, s_TAKES_BACK);
    pyrt::xrefs::reg_access("memory.named", "TAKE_BACK_LEAD", g_TAKE_BACK_LEAD, s_TAKE_BACK_LEAD);
    pyrt::xrefs::reg_access("memory.named", "TAKE_BACK_WHOLE", g_TAKE_BACK_WHOLE, s_TAKE_BACK_WHOLE);
    pyrt::xrefs::reg_access("memory.named", "THE_ASKING", g_THE_ASKING, s_THE_ASKING);
    pyrt::xrefs::reg_access("memory.named", "THE_ASKING_TAIL", g_THE_ASKING_TAIL, s_THE_ASKING_TAIL);
    pyrt::xrefs::reg_access("memory.named", "THE_ASKING_VERB", g_THE_ASKING_VERB, s_THE_ASKING_VERB);
    pyrt::xrefs::reg_access("memory.named", "THE_LENGTH_FOR_IT", g_THE_LENGTH_FOR_IT, s_THE_LENGTH_FOR_IT);
    pyrt::xrefs::reg_access("memory.named", "TIMES_ONLY", g_TIMES_ONLY, s_TIMES_ONLY);
    pyrt::xrefs::reg_access("memory.named", "VERBS", g_VERBS, s_VERBS);
    pyrt::xrefs::reg_access("memory.named", "VIEW_FRAME", g_VIEW_FRAME, s_VIEW_FRAME);
    pyrt::xrefs::reg_access("memory.named", "VIEW_MANNER", g_VIEW_MANNER, s_VIEW_MANNER);
    pyrt::xrefs::reg_access("memory.named", "VIEW_POSSESSIVE", g_VIEW_POSSESSIVE, s_VIEW_POSSESSIVE);
    pyrt::xrefs::reg_access("memory.named", "WANTS_A_NEW_ONE", g_WANTS_A_NEW_ONE, s_WANTS_A_NEW_ONE);
    pyrt::xrefs::reg_access("memory.named", "WHERE_IT_STANDS", g_WHERE_IT_STANDS, s_WHERE_IT_STANDS);
    pyrt::xrefs::reg_access("memory.named", "WHO_X_IS", g_WHO_X_IS, s_WHO_X_IS);
    pyrt::xrefs::reg_access("memory.named", "_AND_AT_THE_START", g__AND_AT_THE_START, s__AND_AT_THE_START);
    pyrt::xrefs::reg_access("memory.named", "_ASK_FILLER", g__ASK_FILLER, s__ASK_FILLER);
    pyrt::xrefs::reg_access("memory.named", "_A_CONNECTIVE", g__A_CONNECTIVE, s__A_CONNECTIVE);
    pyrt::xrefs::reg_access("memory.named", "_A_TIME_IN_A_REPLY", g__A_TIME_IN_A_REPLY, s__A_TIME_IN_A_REPLY);
    pyrt::xrefs::reg_access("memory.named", "_A_TIME_WORD", g__A_TIME_WORD, s__A_TIME_WORD);
    pyrt::xrefs::reg_access("memory.named", "_GOING_AWAY", g__GOING_AWAY, s__GOING_AWAY);
    pyrt::xrefs::reg_access("memory.named", "_LIVING_THERE", g__LIVING_THERE, s__LIVING_THERE);
    pyrt::xrefs::reg_access("memory.named", "_NOT_A_PRONOUN", g__NOT_A_PRONOUN, s__NOT_A_PRONOUN);
    pyrt::xrefs::reg_access("memory.named", "_NOT_THE_PRACTICE", g__NOT_THE_PRACTICE, s__NOT_THE_PRACTICE);
    pyrt::xrefs::reg_access("memory.named", "_THE_LIFE", g__THE_LIFE, s__THE_LIFE);
    pyrt::xrefs::reg_access("memory.named", "is_ack", g_is_ack, s_is_ack);
    pyrt::xrefs::reg_access("memory.named", "reads_as_bad_news", g_reads_as_bad_news, s_reads_as_bad_news);
    pyrt::xrefs::reg_access("memory.named", "the_carried_topic", g_the_carried_topic, s_the_carried_topic);
    pyrt::xrefs::reg_access("memory.named", "the_subject", g_the_subject, s_the_subject);
}

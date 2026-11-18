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


thread_local! { static G_ABOUT_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANIMAL_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ARRIVED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_OPEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GAP_WITHOUT_A_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_GOOD_ENDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POINTER_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SETBACK_AFTER_THE_WIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TURN_IN_THE_TALE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_YEAR_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY_GAVE_WAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLAUSE_STOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLEAR_RESULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLEAR_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITIONAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DENIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAMILY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HANDED_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAVE_GOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOME_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JUST_HAPPENED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LET_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_OF_THE_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_OF_THE_NEWS_ARRIVAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MORE_OF_THE_NEWS_SCORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVED_IN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_FRAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_IDIOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_IDIOM_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_OF_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEW_ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEW_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEW_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEW_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOBODY_SAID_ANYTHING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_ARRIVAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_CLEAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_QUITE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OCCASION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ORDINAL_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASSED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASS_PARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASS_TAKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRAISED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECOVERY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_BEFORE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_IT_IN_THE_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SETTLED_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SILENCE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOMEONE_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPEAKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKES_A_CLAUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_STOPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_HAPPENED_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORLD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORRY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YEARS_BACK_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ABOUT_THE_WIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AHEAD_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_AGE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ATTEMPT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_HOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AROUND_THE_FIGURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_FOR_SOMETHING_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_OR_THANKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AWARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHEER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOCK_MINUTES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COURSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEDIUM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MESSAGE_THAT_CLAIMS_A_WIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RELATIVE_TOLD_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SCORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WIN_THEN_A_MESSAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARELY_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLAUSE_BREAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COMING_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DONE_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DREAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ENTERED_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__I_PRAISED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MORE_OF_THE_NEWS_MARKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MY_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEVER_ENTERED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_HAPPENING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OR_A_SCORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON_DUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRAISING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SOON_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STANDIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKDAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN_DUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHO_DUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_low: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_THEM() -> Py { ensure_init(); G_ABOUT_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THEM(v: Py) { ensure_init(); G_ABOUT_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g_ANIMAL_THING() -> Py { ensure_init(); G_ANIMAL_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANIMAL_THING(v: Py) { ensure_init(); G_ANIMAL_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_ARRIVED() -> Py { ensure_init(); G_ARRIVED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ARRIVED(v: Py) { ensure_init(); G_ARRIVED.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_OPEN() -> Py { ensure_init(); G_ASK_OPEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_OPEN(v: Py) { ensure_init(); G_ASK_OPEN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GAP_WITHOUT_A_NO() -> Py { ensure_init(); G_A_GAP_WITHOUT_A_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GAP_WITHOUT_A_NO(v: Py) { ensure_init(); G_A_GAP_WITHOUT_A_NO.with(|c| *c.borrow_mut() = v); }
pub fn g_A_GOOD_ENDING() -> Py { ensure_init(); G_A_GOOD_ENDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_GOOD_ENDING(v: Py) { ensure_init(); G_A_GOOD_ENDING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POINTER_IN_FRONT() -> Py { ensure_init(); G_A_POINTER_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POINTER_IN_FRONT(v: Py) { ensure_init(); G_A_POINTER_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SETBACK_AFTER_THE_WIN() -> Py { ensure_init(); G_A_SETBACK_AFTER_THE_WIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SETBACK_AFTER_THE_WIN(v: Py) { ensure_init(); G_A_SETBACK_AFTER_THE_WIN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TURN_IN_THE_TALE() -> Py { ensure_init(); G_A_TURN_IN_THE_TALE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TURN_IN_THE_TALE(v: Py) { ensure_init(); G_A_TURN_IN_THE_TALE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_YEAR_AFTER() -> Py { ensure_init(); G_A_YEAR_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_YEAR_AFTER(v: Py) { ensure_init(); G_A_YEAR_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY() -> Py { ensure_init(); G_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY(v: Py) { ensure_init(); G_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD() -> Py { ensure_init(); G_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD(v: Py) { ensure_init(); G_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_GONE() -> Py { ensure_init(); G_BARE_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_GONE(v: Py) { ensure_init(); G_BARE_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY_GAVE_WAY() -> Py { ensure_init(); G_BODY_GAVE_WAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY_GAVE_WAY(v: Py) { ensure_init(); G_BODY_GAVE_WAY.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE() -> Py { ensure_init(); G_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); G_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLAUSE_STOPS() -> Py { ensure_init(); G_CLAUSE_STOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_STOPS(v: Py) { ensure_init(); G_CLAUSE_STOPS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLEAR_RESULT() -> Py { ensure_init(); G_CLEAR_RESULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLEAR_RESULT(v: Py) { ensure_init(); G_CLEAR_RESULT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLEAR_TAIL() -> Py { ensure_init(); G_CLEAR_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLEAR_TAIL(v: Py) { ensure_init(); G_CLEAR_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITIONAL() -> Py { ensure_init(); G_CONDITIONAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL(v: Py) { ensure_init(); G_CONDITIONAL.with(|c| *c.borrow_mut() = v); }
pub fn g_DENIED() -> Py { ensure_init(); G_DENIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DENIED(v: Py) { ensure_init(); G_DENIED.with(|c| *c.borrow_mut() = v); }
pub fn g_FAMILY() -> Py { ensure_init(); G_FAMILY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAMILY(v: Py) { ensure_init(); G_FAMILY.with(|c| *c.borrow_mut() = v); }
pub fn g_FAULT() -> Py { ensure_init(); G_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAULT(v: Py) { ensure_init(); G_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_GOOD() -> Py { ensure_init(); G_GOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GOOD(v: Py) { ensure_init(); G_GOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_HANDED_OVER() -> Py { ensure_init(); G_HANDED_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HANDED_OVER(v: Py) { ensure_init(); G_HANDED_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_HAVE_GOT() -> Py { ensure_init(); G_HAVE_GOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAVE_GOT(v: Py) { ensure_init(); G_HAVE_GOT.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_THING() -> Py { ensure_init(); G_HOME_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_THING(v: Py) { ensure_init(); G_HOME_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_HOME_WORK() -> Py { ensure_init(); G_HOME_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOME_WORK(v: Py) { ensure_init(); G_HOME_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_JUST_HAPPENED() -> Py { ensure_init(); G_JUST_HAPPENED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JUST_HAPPENED(v: Py) { ensure_init(); G_JUST_HAPPENED.with(|c| *c.borrow_mut() = v); }
pub fn g_LET_DOWN() -> Py { ensure_init(); G_LET_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LET_DOWN(v: Py) { ensure_init(); G_LET_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_OF_THE_NEWS() -> Py { ensure_init(); G_MORE_OF_THE_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_OF_THE_NEWS(v: Py) { ensure_init(); G_MORE_OF_THE_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_OF_THE_NEWS_ARRIVAL() -> Py { ensure_init(); G_MORE_OF_THE_NEWS_ARRIVAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_OF_THE_NEWS_ARRIVAL(v: Py) { ensure_init(); G_MORE_OF_THE_NEWS_ARRIVAL.with(|c| *c.borrow_mut() = v); }
pub fn g_MORE_OF_THE_NEWS_SCORE() -> Py { ensure_init(); G_MORE_OF_THE_NEWS_SCORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MORE_OF_THE_NEWS_SCORE(v: Py) { ensure_init(); G_MORE_OF_THE_NEWS_SCORE.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVED_IN() -> Py { ensure_init(); G_MOVED_IN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVED_IN(v: Py) { ensure_init(); G_MOVED_IN.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARLY() -> Py { ensure_init(); G_NEARLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARLY(v: Py) { ensure_init(); G_NEARLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATED() -> Py { ensure_init(); G_NEGATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATED(v: Py) { ensure_init(); G_NEGATED.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_ASK() -> Py { ensure_init(); G_NEWS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_ASK(v: Py) { ensure_init(); G_NEWS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_FRAME() -> Py { ensure_init(); G_NEWS_FRAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_FRAME(v: Py) { ensure_init(); G_NEWS_FRAME.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_IDIOM() -> Py { ensure_init(); G_NEWS_IDIOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_IDIOM(v: Py) { ensure_init(); G_NEWS_IDIOM.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_IDIOM_TAIL() -> Py { ensure_init(); G_NEWS_IDIOM_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_IDIOM_TAIL(v: Py) { ensure_init(); G_NEWS_IDIOM_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_OF_IT() -> Py { ensure_init(); G_NEWS_OF_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_OF_IT(v: Py) { ensure_init(); G_NEWS_OF_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_WORD() -> Py { ensure_init(); G_NEWS_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_WORD(v: Py) { ensure_init(); G_NEWS_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_NEW_ANIMAL() -> Py { ensure_init(); G_NEW_ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEW_ANIMAL(v: Py) { ensure_init(); G_NEW_ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NEW_HOME() -> Py { ensure_init(); G_NEW_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEW_HOME(v: Py) { ensure_init(); G_NEW_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_NEW_PLACE() -> Py { ensure_init(); G_NEW_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEW_PLACE(v: Py) { ensure_init(); G_NEW_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEW_THING() -> Py { ensure_init(); G_NEW_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEW_THING(v: Py) { ensure_init(); G_NEW_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY() -> Py { ensure_init(); G_NOBODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY(v: Py) { ensure_init(); G_NOBODY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOBODY_SAID_ANYTHING() -> Py { ensure_init(); G_NOBODY_SAID_ANYTHING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOBODY_SAID_ANYTHING(v: Py) { ensure_init(); G_NOBODY_SAID_ANYTHING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_ARRIVAL() -> Py { ensure_init(); G_NOT_AN_ARRIVAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_ARRIVAL(v: Py) { ensure_init(); G_NOT_AN_ARRIVAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_OCCASION() -> Py { ensure_init(); G_NOT_AN_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_OCCASION(v: Py) { ensure_init(); G_NOT_AN_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_CLEAR() -> Py { ensure_init(); G_NOT_CLEAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_CLEAR(v: Py) { ensure_init(); G_NOT_CLEAR.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_NEWS() -> Py { ensure_init(); G_NOT_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_NEWS(v: Py) { ensure_init(); G_NOT_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_QUITE() -> Py { ensure_init(); G_NOT_QUITE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_QUITE(v: Py) { ensure_init(); G_NOT_QUITE.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT_THING() -> Py { ensure_init(); G_OBJECT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT_THING(v: Py) { ensure_init(); G_OBJECT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OCCASION() -> Py { ensure_init(); G_OCCASION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OCCASION(v: Py) { ensure_init(); G_OCCASION.with(|c| *c.borrow_mut() = v); }
pub fn g_ORDINAL_DAY() -> Py { ensure_init(); G_ORDINAL_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ORDINAL_DAY(v: Py) { ensure_init(); G_ORDINAL_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_PASSED() -> Py { ensure_init(); G_PASSED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASSED(v: Py) { ensure_init(); G_PASSED.with(|c| *c.borrow_mut() = v); }
pub fn g_PASS_PARTICLE() -> Py { ensure_init(); G_PASS_PARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASS_PARTICLE(v: Py) { ensure_init(); G_PASS_PARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_PASS_TAKEN() -> Py { ensure_init(); G_PASS_TAKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASS_TAKEN(v: Py) { ensure_init(); G_PASS_TAKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_PRAISED() -> Py { ensure_init(); G_PRAISED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRAISED(v: Py) { ensure_init(); G_PRAISED.with(|c| *c.borrow_mut() = v); }
pub fn g_RECOVERY() -> Py { ensure_init(); G_RECOVERY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECOVERY(v: Py) { ensure_init(); G_RECOVERY.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_BEFORE_DAY() -> Py { ensure_init(); G_RELATION_BEFORE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_BEFORE_DAY(v: Py) { ensure_init(); G_RELATION_BEFORE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_IT_IN_THE_PRESENT() -> Py { ensure_init(); G_SAYS_IT_IN_THE_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_IT_IN_THE_PRESENT(v: Py) { ensure_init(); G_SAYS_IT_IN_THE_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED() -> Py { ensure_init(); G_SETTLED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED(v: Py) { ensure_init(); G_SETTLED.with(|c| *c.borrow_mut() = v); }
pub fn g_SETTLED_TAIL() -> Py { ensure_init(); G_SETTLED_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SETTLED_TAIL(v: Py) { ensure_init(); G_SETTLED_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_SILENCE_WORDS() -> Py { ensure_init(); G_SILENCE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SILENCE_WORDS(v: Py) { ensure_init(); G_SILENCE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SOMEONE_LEFT() -> Py { ensure_init(); G_SOMEONE_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_LEFT(v: Py) { ensure_init(); G_SOMEONE_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_SPEAKER() -> Py { ensure_init(); G_SPEAKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPEAKER(v: Py) { ensure_init(); G_SPEAKER.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKES_A_CLAUSE() -> Py { ensure_init(); G_TAKES_A_CLAUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKES_A_CLAUSE(v: Py) { ensure_init(); G_TAKES_A_CLAUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_STOPS() -> Py { ensure_init(); G_THING_STOPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_STOPS(v: Py) { ensure_init(); G_THING_STOPS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_HAPPENED_ASK() -> Py { ensure_init(); G_WHAT_HAPPENED_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_HAPPENED_ASK(v: Py) { ensure_init(); G_WHAT_HAPPENED_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WIN() -> Py { ensure_init(); G_WIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WIN(v: Py) { ensure_init(); G_WIN.with(|c| *c.borrow_mut() = v); }
pub fn g_WORLD_NEWS() -> Py { ensure_init(); G_WORLD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORLD_NEWS(v: Py) { ensure_init(); G_WORLD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_WORRY_WORDS() -> Py { ensure_init(); G_WORRY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORRY_WORDS(v: Py) { ensure_init(); G_WORRY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_YEARS_BACK_LINES() -> Py { ensure_init(); G_YEARS_BACK_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YEARS_BACK_LINES(v: Py) { ensure_init(); G_YEARS_BACK_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g__ABOUT_THE_WIN() -> Py { ensure_init(); G__ABOUT_THE_WIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ABOUT_THE_WIN(v: Py) { ensure_init(); G__ABOUT_THE_WIN.with(|c| *c.borrow_mut() = v); }
pub fn g__AHEAD() -> Py { ensure_init(); G__AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AHEAD(v: Py) { ensure_init(); G__AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__AHEAD_SPAN() -> Py { ensure_init(); G__AHEAD_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AHEAD_SPAN(v: Py) { ensure_init(); G__AHEAD_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g__ALREADY() -> Py { ensure_init(); G__ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALREADY(v: Py) { ensure_init(); G__ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_AGE_SAID() -> Py { ensure_init(); G__AN_AGE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_SAID(v: Py) { ensure_init(); G__AN_AGE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ATTEMPT() -> Py { ensure_init(); G__AN_ATTEMPT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ATTEMPT(v: Py) { ensure_init(); G__AN_ATTEMPT.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_HOUR() -> Py { ensure_init(); G__AN_HOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_HOUR(v: Py) { ensure_init(); G__AN_HOUR.with(|c| *c.borrow_mut() = v); }
pub fn g__AROUND_THE_FIGURES() -> Py { ensure_init(); G__AROUND_THE_FIGURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AROUND_THE_FIGURES(v: Py) { ensure_init(); G__AROUND_THE_FIGURES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_FOR_SOMETHING_BACK() -> Py { ensure_init(); G__ASKS_FOR_SOMETHING_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_SOMETHING_BACK(v: Py) { ensure_init(); G__ASKS_FOR_SOMETHING_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_OR_THANKS() -> Py { ensure_init(); G__ASKS_OR_THANKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_OR_THANKS(v: Py) { ensure_init(); G__ASKS_OR_THANKS.with(|c| *c.borrow_mut() = v); }
pub fn g__AWARD() -> Py { ensure_init(); G__AWARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AWARD(v: Py) { ensure_init(); G__AWARD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHEER() -> Py { ensure_init(); G__A_CHEER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHEER(v: Py) { ensure_init(); G__A_CHEER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLASS() -> Py { ensure_init(); G__A_CLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLASS(v: Py) { ensure_init(); G__A_CLASS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOCK() -> Py { ensure_init(); G__A_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOCK(v: Py) { ensure_init(); G__A_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOCK_MINUTES() -> Py { ensure_init(); G__A_CLOCK_MINUTES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOCK_MINUTES(v: Py) { ensure_init(); G__A_CLOCK_MINUTES.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COUNT() -> Py { ensure_init(); G__A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNT(v: Py) { ensure_init(); G__A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COURSE() -> Py { ensure_init(); G__A_COURSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COURSE(v: Py) { ensure_init(); G__A_COURSE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DATE() -> Py { ensure_init(); G__A_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DATE(v: Py) { ensure_init(); G__A_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE_IN_THE_LINE() -> Py { ensure_init(); G__A_FIGURE_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE_IN_THE_LINE(v: Py) { ensure_init(); G__A_FIGURE_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEDIUM() -> Py { ensure_init(); G__A_MEDIUM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEDIUM(v: Py) { ensure_init(); G__A_MEDIUM.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MESSAGE_THAT_CLAIMS_A_WIN() -> Py { ensure_init(); G__A_MESSAGE_THAT_CLAIMS_A_WIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MESSAGE_THAT_CLAIMS_A_WIN(v: Py) { ensure_init(); G__A_MESSAGE_THAT_CLAIMS_A_WIN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NUMBER() -> Py { ensure_init(); G__A_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NUMBER(v: Py) { ensure_init(); G__A_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERSON() -> Py { ensure_init(); G__A_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERSON(v: Py) { ensure_init(); G__A_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RELATIVE_TOLD_ME() -> Py { ensure_init(); G__A_RELATIVE_TOLD_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RELATIVE_TOLD_ME(v: Py) { ensure_init(); G__A_RELATIVE_TOLD_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SCORE() -> Py { ensure_init(); G__A_SCORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SCORE(v: Py) { ensure_init(); G__A_SCORE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WEIGHT() -> Py { ensure_init(); G__A_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WEIGHT(v: Py) { ensure_init(); G__A_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WIN() -> Py { ensure_init(); G__A_WIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WIN(v: Py) { ensure_init(); G__A_WIN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WIN_THEN_A_MESSAGE() -> Py { ensure_init(); G__A_WIN_THEN_A_MESSAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WIN_THEN_A_MESSAGE(v: Py) { ensure_init(); G__A_WIN_THEN_A_MESSAGE.with(|c| *c.borrow_mut() = v); }
pub fn g__BARELY_ASKED() -> Py { ensure_init(); G__BARELY_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARELY_ASKED(v: Py) { ensure_init(); G__BARELY_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__CLAUSE_BREAK() -> Py { ensure_init(); G__CLAUSE_BREAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLAUSE_BREAK(v: Py) { ensure_init(); G__CLAUSE_BREAK.with(|c| *c.borrow_mut() = v); }
pub fn g__COMING_UP() -> Py { ensure_init(); G__COMING_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COMING_UP(v: Py) { ensure_init(); G__COMING_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED_UP() -> Py { ensure_init(); G__COUNTED_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED_UP(v: Py) { ensure_init(); G__COUNTED_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__DONE_ALREADY() -> Py { ensure_init(); G__DONE_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DONE_ALREADY(v: Py) { ensure_init(); G__DONE_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g__DREAD() -> Py { ensure_init(); G__DREAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DREAD(v: Py) { ensure_init(); G__DREAD.with(|c| *c.borrow_mut() = v); }
pub fn g__DUE() -> Py { ensure_init(); G__DUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DUE(v: Py) { ensure_init(); G__DUE.with(|c| *c.borrow_mut() = v); }
pub fn g__ENTERED_IT() -> Py { ensure_init(); G__ENTERED_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ENTERED_IT(v: Py) { ensure_init(); G__ENTERED_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__I_PRAISED() -> Py { ensure_init(); G__I_PRAISED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__I_PRAISED(v: Py) { ensure_init(); G__I_PRAISED.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY() -> Py { ensure_init(); G__MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY(v: Py) { ensure_init(); G__MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH() -> Py { ensure_init(); G__MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH(v: Py) { ensure_init(); G__MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g__MORE_OF_THE_NEWS_MARKS() -> Py { ensure_init(); G__MORE_OF_THE_NEWS_MARKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MORE_OF_THE_NEWS_MARKS(v: Py) { ensure_init(); G__MORE_OF_THE_NEWS_MARKS.with(|c| *c.borrow_mut() = v); }
pub fn g__MY_OWN() -> Py { ensure_init(); G__MY_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MY_OWN(v: Py) { ensure_init(); G__MY_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__NEVER_ENTERED() -> Py { ensure_init(); G__NEVER_ENTERED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEVER_ENTERED(v: Py) { ensure_init(); G__NEVER_ENTERED.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_HAPPENING() -> Py { ensure_init(); G__NOT_HAPPENING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_HAPPENING(v: Py) { ensure_init(); G__NOT_HAPPENING.with(|c| *c.borrow_mut() = v); }
pub fn g__OR_A_SCORE() -> Py { ensure_init(); G__OR_A_SCORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OR_A_SCORE(v: Py) { ensure_init(); G__OR_A_SCORE.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON_DUE() -> Py { ensure_init(); G__PERSON_DUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON_DUE(v: Py) { ensure_init(); G__PERSON_DUE.with(|c| *c.borrow_mut() = v); }
pub fn g__PRAISING() -> Py { ensure_init(); G__PRAISING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRAISING(v: Py) { ensure_init(); G__PRAISING.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID() -> Py { ensure_init(); G__SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID(v: Py) { ensure_init(); G__SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__SOON_ITSELF() -> Py { ensure_init(); G__SOON_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SOON_ITSELF(v: Py) { ensure_init(); G__SOON_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g__STANDIN() -> Py { ensure_init(); G__STANDIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STANDIN(v: Py) { ensure_init(); G__STANDIN.with(|c| *c.borrow_mut() = v); }
pub fn g__TOLD() -> Py { ensure_init(); G__TOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TOLD(v: Py) { ensure_init(); G__TOLD.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKDAY() -> Py { ensure_init(); G__WEEKDAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKDAY(v: Py) { ensure_init(); G__WEEKDAY.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN_DUE() -> Py { ensure_init(); G__WHEN_DUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN_DUE(v: Py) { ensure_init(); G__WHEN_DUE.with(|c| *c.borrow_mut() = v); }
pub fn g__WHO_DUE() -> Py { ensure_init(); G__WHO_DUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHO_DUE(v: Py) { ensure_init(); G__WHO_DUE.with(|c| *c.borrow_mut() = v); }
pub fn g_line() -> Py { ensure_init(); G_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_line(v: Py) { ensure_init(); G_line.with(|c| *c.borrow_mut() = v); }
pub fn g_low() -> Py { ensure_init(); G_low.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_low(v: Py) { ensure_init(); G_low.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Good news said as an event, not as a feeling.\n\n`POSITIVE_CUES` is a list of words, and it catches \"i passed\", \"we won\", \"we're pregnant\". What it\nmisses is the way people actually tell you about a good thing: the event is in a phrase and every\nword of it is plain. \"my sister had her baby yesterday\", \"we got the keys on friday\", \"he got into\nhis first choice\" are all good news with nothing in `POSITIVE_CUES` to notice it, so the line fell\nthrough to the neutral pool - and, since the acknowledgement landed, to the fact read back:\n\"Noted: your sister had her baby yesterday\" is a filing cabinet, not a friend.\n\nThis module notices those events. It only says \"this is good news\"; the words of the reply belong\nto the composer, so the congrats line can change without touching this file. Run it alone:\n\n    python -m bot.compose.goodnews \"my sister had her baby yesterday\"\n");
    s_NEARLY(crate::compose::g_NEARLY());
    s__MONTH(py_str("(?:jan(?:uary)?|feb(?:ruary)?|mar(?:ch)?|apr(?:il)?|may|jun(?:e)?|jul(?:y)?|aug(?:ust)?|sep(?:t|tember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)"));
    s__WEEKDAY(py_str("(?:mon(?:day)?|tue(?:sday)?|wed(?:nesday)?|thu(?:rsday)?|fri(?:day)?|sat(?:urday)?|sun(?:day)?)"));
    s__A_DATE(op_add(&op_add(&op_add(&op_add(&py_str("\\s+(?:the\\s+)?(?:early\\s+|late\\s+|mid-?)?(?:\\d|"), &g__MONTH()), &py_str("|")), &g__WEEKDAY()), &py_str("|christmas|easter|spring|summer|autumn|winter|month|week|day|year|a week|a month|two weeks|three weeks)\\b")));
    s__DUE(py_str("\\s+due\\b"));
    s__WHEN_DUE(py_str("\\s+(?:in|on|next|by|at|early|late|mid)\\b"));
    s__WHO_DUE(op_add(&op_add(&py_str("\\b(?:i'm|im|i am|we're|we re|we are|she's|shes|she is|he's|hes|he is|they're|theyre|they are)"), &g__DUE()), &g__WHEN_DUE()));
    s__PERSON_DUE(py_str("\\b(?:the|our|my)?\\s*(?:baby|babies|twins?|wife|partner|girlfriend|fiancee?|daughter|sister|brother|friend|surrogate)"));
    s_BABY(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:had|have|has|have just had|had just had)\\s+(?:her|his|the|a|our)\\s+(?:baby|little (?:girl|boy|one)|girl|boy)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:baby|girl|boy|twins|daughter|son)\\s+(?:was |were |is )?(?:born)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bgave birth\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|she|he|they)(?:'re| are|re)?\\s+(?:now\\s+)?(?:expecting|pregnant)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bexpecting\\s+(?:a|our|twins)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it'?s|its)\\s+(?:a\\s+)?(?:little\\s+)?(?:girl|boy|healthy|twins)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|well|and|ok|okay|ah|oh)?[\\s,]*a\\s+(?:little\\s+)?(?:girl|boy|twins|healthy\\s+baby)\\s*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|well|and|ok|okay|ah|oh)?[\\s,]*a\\s+(?:little\\s+)?(?:girl|boy|twins)\\b[^.!?]{0,20}?\\b\\d+(?:\\.\\d+)?\\s*(?:lb|lbs|pounds?|kg|kilos?|oz|ounces?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the|our|my)\\s+baby\\s+(?:is|was|'s)\\s+(?:a\\s+)?(?:little\\s+)?(?:girl|boy|twins|healthy)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|i|she|he|they|my\\s+\\w+|his\\s+\\w+|her\\s+\\w+|their\\s+\\w+)(?:'re|re|\\s+are|\\s+am|\\s+is)?\\s+(?:now\\s+)?having\\s+(?:a\\s+|our\\s+)?(?:little\\s+)?(?:boy|girl|twins)\\b(?!\\s+(?:over|round|to\\s+stay|for\\s+(?:dinner|lunch|tea|supper|christmas)))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&g__WHO_DUE(), &g__A_DATE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&g__PERSON_DUE(), &py_str("(?:'s|s'|s)?\\s+(?:is\\s+|are\\s+)?due\\b")), &g__WHEN_DUE()), &g__A_DATE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:my|her|his|their|the)\\s+due date\\b(?:\\s+(?:is|was))?"), &g__WHEN_DUE()), &g__A_DATE())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s__A_CLASS(py_str("(?:first|2[:. ]?1|2[:. ]?2|1st|2nd|upper\\s+second|third|distinction|merit)"));
    s__MY_OWN(py_str("\\b(?:i|we)\\b[^.!?]{0,30}?"));
    s__A_COUNT(py_str("(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+"));
    s__A_COURSE(py_str("(?:degree|degrees|honours|graduat\\w+|classification|results?|finals|dissertation|thesis|course|uni|university|masters?|phd|doctorate|btec|diploma|hnd|a\\s+levels?|gcses?|qualification|exams?)"));
    s_PASSED(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bpassed\\b(?:\\s+\\w+){0,3}?\\s+(?:test|exams?|course|interview|boards?|driving|theory|assessment|audition|viva|dissertation|grade|gcses?|a level|finals|mocks?|check|inspection|audit|probation|review)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bpassed\\s+(?:it\\b(?!\\s+(?:to|on|up|over|round|around|back)\\b)|with\\b|first time|everything|all of)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we) (?:just |finally )?passed\\b(?=$|[.!]| (?:now|today|yay|at last)\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwe passed (?:the |an? )?\\w+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_WIN(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bgot\\s+(?:the|a|my|her|his|our|their)\\s+(?:job|role|keys|offer)\\b(?!\\s+(?:interview|application|centre|center))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bgot\\s+into\\b(?:\\s+\\w+){0,4}?\\s+(?:uni|university|college|school|course|team|squad|finals?|semis|shortlist|first choice|apprenticeship|placement|nursing|med school|medical school|the show|the exhibition|the choir)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:got|getting|had)\\s+(?:a|the|my|her|his)\\s+(?:promotion|pay rise|payrise|raise|new job)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bpromoted\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|his|her|their)\\s+(?:\\w+\\s+){0,2}?(?:got|was|were|has\\s+been|have\\s+been|had\\s+been|been)\\s+(?:approved|granted)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:just\\s+|finally\\s+)?got\\s+(?:\\w+\\s+){0,3}?(?:approved|granted)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:won|win|wins)\\s+(?:\\w+\\s+){0,3}?(?:cup|medal|prize|award|race|match|competition|league|final|round|gold|silver|bronze)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcame\\s+(?:first|top|second)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|his|her|their)\\s+(?:\\w+\\s+){0,2}?(?:leave|holiday|time\\s+off|visa|application|offer|mortgage|transfer|grant|funding|booking|planning\\s+permission|request)\\b[^.!?]{0,24}\\b(?:got|was|were|is|been|just|now|finally)\\s+(?:\\w+\\s+){0,2}?(?:approved|accepted|granted|signed\\s+off|came\\s+through|went\\s+through)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|his|her|their)\\s+(?:\\w+\\s+){0,2}?(?:leave|visa|application|offer|mortgage|transfer|grant|funding|booking)\\b[^.!?]{0,24}\\b(?:came\\s+through|went\\s+through|signed\\s+off)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our|his|her|their)\\s+(?:\\w+\\s+){0,2}?(?:leave|visa|application|offer|mortgage|transfer|grant|funding|booking|request|day\\s+off|time\\s+off)\\s+(?:(?:got|was|were|is|been|now|just|finally|all)\\s+){0,2}(?:approved|accepted|granted|signed\\s+off)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bgraduated\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&g__MY_OWN(), &py_str("(?:got|gotten|achieved|came\\s+away\\s+with|graduated\\s+with|finished\\s+with)\\s+(?:a\\s+|an\\s+|my\\s+|our\\s+)?")), &g__A_CLASS()), &py_str("(?!\\s+of\\b)(?=[^.!?]{0,60}?\\b")), &g__A_COURSE()), &py_str("\\b)"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&g__MY_OWN(), &py_str("(?:got|gotten|achieved)\\s+(?:a|an)\\s+")), &g__A_CLASS()), &py_str("(?!\\s+of\\b)(?=\\s*(?:[.!?]|$|,|\\s+(?:in|for|overall|class)\\b))"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&g__MY_OWN(), &py_str("(?:got|gotten|achieved|came\\s+away\\s+with|ended\\s+up\\s+with)\\s+(?:a\\s+|an\\s+|my\\s+|our\\s+|")), &g__A_COUNT()), &py_str(")?(?:a[\\s*-]?stars?\\b(?![\\s-]*(?:wars|trek|gazer|gazing|sign|bucks|fish|city|chart))|(?:all|straight)\\s+(?:9s|8s)\\b)"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:got|gotten|now)\\s+engaged\\b|\\b(?:i'm|im|i\\s+am|we're|were|we\\s+are|weve|we've|she's|shes|she\\s+is|he's|hes|he\\s+is)\\s+(?:now\\s+|just\\s+|finally\\s+)?engaged\\b(?!\\s+(?:in|on|with|to\\s+(?:a|an|the|some)\\s+\\w))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she)\\s+(?:only\\s+|just\\s+|finally\\s+|only\\s+went\\s+and\\s+)?proposed\\b(?=\\s*(?:[.!]|,|$|\\bto\\s+me\\b|\\byesterday\\b|\\blast\\b|\\bat\\b|\\bon\\b|\\band\\b|\\bso\\b|\\bnow\\b|\\btoday\\b))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:moving|moved|move)\\s+in\\s+together\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:had|got|have|having)\\s+(?:an?\\s+|our\\s+|the\\s+|my\\s+|their\\s+)?offer\\s+accepted\\b|\\boffer\\s+(?:got|was|is|has\\s+been|have\\s+been)\\s+accepted\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:been|being|was|were|got)\\s+(?:just\\s+)?offered\\s+(?:the|a|my|this)\\s+(?:job|role|position|place|post)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:they|he|she|the\\s+\\w+)\\s+offered\\s+me\\s+(?:the|a)\\s+(?:job|role|position|post)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfinished\\s+(?:my|the|our)\\s+(?:degree|course|exams?|dissertation|thesis|apprenticeship|training|masters?|phd|nursing|gcses?|a\\s+levels?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:going|gonna)\\s+to\\s+be\\s+(?:(?:a|an)\\s+(?:grandma|grandmother|nan|nana|granny|gran|grandad|grandfather|grandpa|auntie|aunty|uncle)\\b|grandparents\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bought|got)\\s+(?:a|our own|our first)\\s+(?:house|flat|place)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:got|signed)\\s+the\\s+keys\\b|\\bcompletion\\s+(?:date|day)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:all clear|got the all clear|out of hospital|rang the bell)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we're|we are|were|we've\\s+started|we\\s+have\\s+started)\\s+(?:in\\s+the\\s+middle\\s+of\\s+)?adopting\\b(?![^.!?]*\\b(?:policy|policies|approach|stance|attitude|method|tactic)\\b)|\\b(?:the\\s+)?adoption\\s+(?:went\\s+through|is\\s+going\\s+through)\\b|\\b(?:started|starting|began)\\s+(?:the|an|our)\\s+adoption\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:came|come)\\s+(?:in\\s*to|into)\\s+(?:some\\s+|a\\s+bit\\s+of\\s+|a\\s+little\\s+)?(?:money|an\\s+inheritance|cash)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:book|novel)\\s+(?:got|was|is|has\\s+been|have\\s+been)\\s+published\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s__SAID(py_str("(?:said|says|saying|told\\s+me|tells\\s+me|texted|texts|wrote|emailed|emails|rang|phoned|reckons|thinks)"));
    s__PRAISING(py_str("(?:doing\\s+(?:really\\s+|very\\s+)?well|done\\s+(?:really\\s+|very\\s+)?well|did\\s+(?:really\\s+|very\\s+)?well|doing\\s+(?:really\\s+|very\\s+)?great|did\\s+great|a\\s+(?:really\\s+|very\\s+)?(?:good|great|smashing)\\s+job|(?:really\\s+|very\\s+)?(?:proud|pleased|impressed)\\s+(?:of|with)\\s+(?:me|us|him|her|them)|well\\s+done|star\\s+of\\s+the\\s+week|top\\s+of\\s+the\\s+class)"));
    s__AWARD(py_str("(?:merit|gold\\s+star|star\\s+of\\s+the\\s+week|head\\s+teachers?\\s+award|player\\s+of\\s+the\\s+match)"));
    s__I_PRAISED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:said|say|says|told|tell|tells|texted|wrote|emailed)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PRAISED(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g__SAID()), &py_str("\\b(?:\\s+\\w+){0,5}?\\s+")), &g__PRAISING()), &py_str("\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:got|came\\s+home\\s+with|won|awarded)\\s+(?:(?:a|an|the)\\s+)?"), &g__AWARD()), &py_str("\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_GOOD(op_add(&op_add(&op_add(&g_BABY(), &g_PASSED()), &g_WIN()), &g_PRAISED()));
    s_ARRIVED(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|i)(?:\\s+(?:just|finally|only))?\\s+(?:got|gotten|adopted|rescued|rehomed|picked up|bought|chose)\\s+(?:a|an|our|my|his|her|their|another|two|some|one|a few|a couple|a pair|three|four|five|six)\\s+(?P<thing>[a-z][a-z' ]{1,30})")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|i)(?:'ve|ve)\\s+(?:just|finally|only)\\s+(?:got|gotten|adopted|rescued|rehomed|picked up|bought|chose)\\s+(?:a|an|our|my|his|her|their|another|two|some|one|a few|a couple|a pair|three|four|five|six)\\s+(?P<thing>[a-z][a-z' ]{1,30})")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|i)\\s*(?:'re|re|are)?\\s+(?:getting|having)\\s+(?:a|an|our|another)\\s+(?P<thing>[a-z][a-z' ]{1,30}?)\\s+(?:put in|fitted|installed|done|built|delivered)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_HAVE_GOT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?P<who>we|i)(?:\\s+(?:have|has|had)|'ve|ve)\\s+got\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_OF_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:new|brand[- ]new|another|just|finally|only|now)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MOVED_IN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:we|i)(?:'ve|ve)?\\s+(?:just\\s+|finally\\s+|now\\s+)?(?:moved|moving)\\s+(?:house|flat|home|in|into)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ANIMAL_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    dog puppy pup cat kitten greyhound lurcher whippet terrier spaniel retriever labrador\n    collie poodle rabbit bunny bunnies hamster gerbil guinea pig rat mice mouse horse pony\n    donkey parrot budgie cockatiel bird chicken chickens hen hens duck ducks fish goldfish\n    tortoise snake lizard ferret chinchilla koi\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOME_WORK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    kitchen bathroom extension loft garden shed decking conservatory roof windows drive\n    driveway fence flooring tiles tiling wallpaper fireplace plastering rewiring insulation\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOME_THING(op_bitor(&g_HOME_WORK(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    boiler carpet carpets sofa bed fridge freezer oven cooker hob washer washing machine\n    dishwasher door doors window\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    s_OBJECT_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    bike bicycle trike motorbike scooter car van caravan camper moped boat kayak canoe\n    phone iphone mobile tablet ipad laptop computer pc desktop macbook monitor keyboard\n    printer camera headphone headphones speaker speakers radio tv television kindle watch\n    guitar piano violin drums ukulele keyboard sofa couch armchair chair table desk bed\n    mattress wardrobe cupboard drawers bookcase shelf shelves rug lamp light curtains\n    blinds mirror picture painting fridge freezer kettle toaster microwave oven cooker hob\n    dishwasher washer washing machine dryer vacuum hoover iron blender airfryer\n    coat jacket dress skirt trousers jeans shirt dress suit shoes boots trainers sandals\n    hat cap scarf gloves bag handbag rucksack suitcase pram pushchair buggy cot crib\n    toy toys lego doll dolls console playstation xbox switch game games book books\n    ring necklace bracelet earring tattoo plant plants tree seeds greenhouse pond\n    mower lawnmower bbq barbecue grill tent sleeping bag\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_AN_ARRIVAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    ticket fine bill bills letter letters cold cough bug virus parking quote estimate\n    refund diagnosis invoice debt summons notice warning complaint infection\n    job role promotion offer keys tenancy\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THING_STOPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    for from in on at with by to of last next this yesterday today tomorrow and but so because\n    that which who when while if then than as about after before over under into onto\n    fitted installed delivered built done put made changed fixed coming arriving called\n    named\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FAULT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:no|not|isnt|isn't|wont|won't|doesnt|doesn't|dont|don't|broken|leaking|leak|damaged|fault|faulty|problem|issues?|wrong|blocked|flooded|stuck|cracked|smell|smells|nightmare|hated|hates?|regret\\w*|disappoint\\w*|rip[- ]off|useless|rubbish|waste of|went wrong|fell apart|dead on arrival|crash\\w*|dud|lemon)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEW_ANIMAL(py_tuple(vec![py_str("A {thing} — how are they settling in?"), py_str("Lovely. What's the {thing} like so far?"), py_str("How's the {thing} getting on?")]));
    s_NEW_HOME(py_tuple(vec![py_str("A {thing} is a proper job — how far along is it?"), py_str("How's the {thing} coming along?"), py_str("Good news. What's the part of the {thing} you're dreading?")]));
    s_NEW_PLACE(py_tuple(vec![py_str("How's the new place?"), py_str("What's the best thing about the new place so far?"), py_str("How's it feeling, being somewhere new?")]));
    s_NEW_THING(py_tuple(vec![py_str("Tell me about the {thing} — what's it like?"), py_str("Nice. What made you go for the {thing}?"), py_str("How are you finding the {thing}?")]));
    s_A_POINTER_IN_FRONT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^of\\s+(?:those|these|them|the|my|our|his|her|their)\\s+")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ORDINAL_DAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:(?P<whose>[\\w'-]+)\\s+)?(?P<day>\\d+(?:st|nd|rd|th))\\b(?:\\s*,?\\s*(?:is|was|are)?\\s*(?:in|on|next|this|tomorrow|tonight|soon|coming|at\\s+the\\s+end|early|late|this\\s+weekend|the\\s+weekend)\\b|[\\s.!]*$)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OCCASION(py_tuple(vec![py_tuple(vec![py_str("birthday"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:its|it's|it is)\\s+(?:\\d+(?:st|nd|rd|th)\\s+)?birthday\\b|\\b(?:my|our)\\s+(?:[\\w'-]+\\s+)?(?:\\d+(?:st|nd|rd|th)\\s+)?birthday\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("anniversary"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:its|it's|it is|my|our)\\s+(?:\\d+(?:st|nd|rd|th)\\s+)?(?:\\w+\\s+)?anniversary\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("wedding"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:im|i'm|i am|we're|we are|we re)\\s+(?:getting\\s+)?married\\b|\\b(?:my|our)\\s+wedding\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), py_tuple(vec![py_str("wish"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a\\s+|very\\s+|so\\s+|aw+\\s+|oh\\s+|many\\s+)*(?:happy\\s+(?:birthday|bday|b-day)|many\\s+happy\\s+returns)\\s*[\\s!.,]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])]));
    s_NOT_AN_OCCASION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\byou\\b|\\byour\\b|\\bwould(?:'ve| have|ve)?\\s+been\\b|\\b(?:what|which|when|whens|when's|how|where)\\b|\\b(?:present|presents|gift|gifts|idea|ideas|nothing|need|needs)\\b|\\b(?:was|were)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SOMEONE_LEFT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:he|she|they)\\s*(?:'s|'d|s|d)?\\s+(?:has\\s+|had\\s+|just\\s+|then\\s+|already\\s+)?(?:left|leaves|leaving|walked\\s+out|walking\\s+out|moved\\s+out|dumped|split\\s+up)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LET_DOWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:forgot|forgets|forgetting|forgotten|fell\\s+apart|cancell?ed|ruined|messed|mucked|hate|wrong|disaster|horrible|nobody|no\\s?body|no\\s?one|none\\s+of\\s+them|annoyed|annoying|upset|arguing|argued|row|fight|fighting|didn't|didnt|did not|won't|wont|worried|worry|scared|terrified|dreading|stressed|anxious)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RELATION_BEFORE_DAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?:(?P<whose>[\\w'-]+(?:\\s+[\\w'-]+)?)\\s+)?(?:\\d+(?:st|nd|rd|th)\\s+)?(?:birthday|anniversary)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_FAMILY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("kid"), py_str("child"), py_str("baby"), py_str("twins"), py_str("girl"), py_str("boy"), py_str("lad"), py_str("teenager"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("ex"), py_str("fiance"), py_str("fiancee"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parent"), py_str("parents"), py_str("stepmum"), py_str("stepdad"), py_str("stepson"), py_str("stepdaughter"), py_str("sister"), py_str("brother"), py_str("sibling"), py_str("sisterinlaw"), py_str("brotherinlaw"), py_str("motherinlaw"), py_str("fatherinlaw"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("niece"), py_str("nephew"), py_str("grandchild"), py_str("gran"), py_str("granny"), py_str("nan"), py_str("nana"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandparent"), py_str("friend"), py_str("mate"), py_str("boss"), py_str("colleague"), py_str("coworker"), py_str("neighbour"), py_str("neighbor"), py_str("flatmate"), py_str("housemate"), py_str("landlord"), py_str("carer"), py_str("client"), py_str("student"), py_str("teacher")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_NEWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:did|do|does|will|would|are|is|was|were|can|could|shall|should|have|has)\\s+(?:i|you|we|she|he|they)\\b|\\b(?:hope|hoping|wish|wishing|wonder|wondering|praying|fingers crossed|fingers are crossed)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_QUITE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::compose::g_NEARLY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TURN_IN_THE_TALE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\band\\s+now\\s+(?:every\\w+|nobody|no\\s+one|no-one|they|he|she)\\b|\\band\\s+[^.!?]{0,40}\\b(?:hates?|hate\\b|disapproves?|refuses?|won'?t|wont|doesn'?t\\s+(?:like|approve|want)|doesnt\\s+(?:like|approve|want)|is\\s+against|objects?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_SETBACK_AFTER_THE_WIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\band\\s+(?:then\\s+|now\\s+)?[^.!?]{0,60}\\b(?:broke|broken|breaks|packed\\s+in|stopped\\s+working|gone\\s+wrong|went\\s+wrong|on\\s+the\\s+blink|is\\s+a\\s+(?:state|mess|tip)|ruined|leaking|leaks?\\b|flooded|cracked|worse|awful|terrible|horrible|nightmare|regrets?|cant\\s+afford|cant\\s+cope|i\\s+cant\\s+do\\b|just\\s+my\\s+luck|died|dead)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GOOD_ENDING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\band\\s+(?:now\\s+|at\\s+last\\s+)?(?:we|i)\\s+(?:can|could|get\\s+to|are\\s+able)\\b|\\band\\s+(?:now\\s+)?(?:that\\s+means\\s+)?(?:we|i)\\s+can\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:lost|miscarried|miscarriage|died|death|stillborn|failed|failing|not going|didn't|didnt|did not|haven't|havent|hasn't|hasnt|not yet|never|gave up|turned down|rejected|but|except|however|fight|fights|trouble|accident|argument|row|debt|hospital|intensive care|poorly|poorly|not well)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_GAP_WITHOUT_A_NO(py_str("(?:(?!\\b(?:not|no|never|hardly|barely|without|nothing|none)\\b)[^.!?])"));
    s_RECOVERY(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:feel|feeling|felt)\\s+(?:a\\s+(?:bit|lot|whole\\s+lot)\\s+|much\\s+|so\\s+much\\s+|loads\\s+|really\\s+|a\\s+lot\\s+)?better\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:im|i'm|i\\s+am)\\s+(?:much\\s+|a\\s+lot\\s+|so\\s+much\\s+|loads\\s+)?better\\b(?!\\s+at\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:pain|headache|migraine|cough|cold|sickness|nausea|rash|swelling|soreness|knee|back|shoulder|ankle|wrist|hip|neck|throat|stomach|ear|tooth|chest)\\b"), &g_A_GAP_WITHOUT_A_NO()), &py_str("{0,8}?\\b(?:is|are|feels?)\\s+(?:much\\s+|a\\s+lot\\s+|so\\s+much\\s+|loads\\s+)?better\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bbetter\\s+than\\s+(?:yesterday|this\\s+morning|last\\s+(?:night|week|month)|before|earlier|it\\s+was|i\\s+was)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:pain|headache|migraine|cough|cold|sickness|nausea|temperature|fever|rash|swelling|soreness|ache)\\b"), &g_A_GAP_WITHOUT_A_NO()), &py_str("{0,12}?\\b(?:has|have|is|are)?\\s*(?:all\\s+)?(?:gone|eased|easing|cleared\\s+up|stopped|settled)\\b(?!\\s+(?:on|to|into|in|down|through|round|around|for|up))\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:over|past|through)\\s+the\\s+worst|on\\s+the\\s+mend|back\\s+on\\s+my\\s+feet|back\\s+to\\s+normal)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)(?:'m|m|\\s+am|\\s+are|'re)?\\s+(?:finally\\s+|just\\s+|mostly\\s+|nearly\\s+)?over\\s+(?:the|my|that|this)\\s+(?:flu|cold|cough|bug|virus|infection|chest\\s+infection|sickness|norovirus|migraine|sore\\s+throat|tonsillitis|bronchitis|pneumonia|stomach\\s+bug|hangover)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:got|get|getting)\\s+over\\s+(?:the|my|that|this)\\s+(?:flu|cold|cough|bug|virus|infection|sickness|norovirus|migraine|sore\\s+throat|tonsillitis|bronchitis|pneumonia|stomach\\s+bug|hangover)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|this|my\\s+\\w+|the\\s+\\w+|the\\s+\\w+\\s+\\w+|wound|cut|burn|scar|sore)\\s+(?:has|have|is|are|'s|s)?\\s*(?:all\\s+)?healed\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_ABOUT_THEM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|well|ok|okay|and|but|tbh|honestly|ah|oh|right|good)?[\\s,]*(?:i|im|i'm|ive|i've|we|we're|weve|we've|my|the|feeling|feel|over|on|back|still)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CONDITIONAL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bbetter\\b[^.!?]{0,24}?\\b(?:when|whenever|if|unless|as\\s+long\\s+as)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PASS_PARTICLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("on"), py_str("up"), py_str("over"), py_str("away"), py_str("round"), py_str("around"), py_str("out"), py_str("by"), py_str("through"), py_str("for"), py_str("to")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PASS_TAKEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("salt"), py_str("ball"), py_str("butter"), py_str("plate"), py_str("bottle"), py_str("note"), py_str("message"), py_str("book"), py_str("pen"), py_str("phone"), py_str("remote"), py_str("keys"), py_str("sugar"), py_str("jug"), py_str("envelope")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:so|ok|okay|well|and|but|oh|right|hi|hey|hello|morning|now|then)\\s*,?\\s+)*(?:got|get|any|anything|whats|what's|what\\s+is|what\\s+about|how\\s+about|do\\s+you\\s+have|have\\s+you\\s+got|have\\s+you|has|have|did|you|u|is\\s+there|are\\s+there|tell\\s+me|give\\s+me|hear\\s+any|heard\\s+any|share|spill)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:news|anything|something)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WORLD_NEWS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:the|todays|today's)\\s+(?:(?:latest|current|world|local|national|main|evening|morning|morning's)\\s+)?news\\b|\\bnews\\s+today\\b|\\bheadlines?\\b|\\bcurrent events\\b|\\bwhat happened in the\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__STANDIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:anything|something)\\s+(?:good|nice|great|happy|brilliant|lovely|exciting)\\b(?:\\s+(?:has\\s+|have\\s+)?(?:happen|happened|happens|happening|been\\s+happening))?(?:\\s+(?:today|lately|recently|this\\s+week|this\\s+weekend|at\\s+work|with\\s+you|for\\s+you))*\\s*$|\\b(?:anything|something)\\s+(?:happen|happened|happens|happening)\\b(?:\\s+(?:today|lately|recently|this\\s+week|this\\s+morning|at\\s+work|at\\s+school|then|so\\s+far|with\\s+you))*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_FRAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("got get any anything whats what's what is about how do you u have it\n    there tell give us hear heard share spill some good nice happy great brilliant amazing\n    wonderful excellent fantastic news today lately now then so well and but ok okay oh hi hey\n    hello morning hows how's been new happen happened happening anything the a an i we me my\n    week weekend day on going with for at"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HANDED_OVER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfor\\s+(?:you|u|ya|you\\s+two|us\\s+all)\\b|\\b(?:to|i'?ll)\\s+tell\\s+you\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_IDIOM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:so|ok|okay|well|and|but|oh|right|hi|hey|hello|morning|now|then)\\s*,?\\s+)*(?:whats|what's|what\\s+is|what\\s+was|tell\\s+me|give\\s+me|spill)\\s+(?:(?:the|any|all\\s+the)\\s+)?(?:script|crack|craic|story|latest|word|buzz|goss|gossip|happening|occurring|been\\s+happening)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEWS_IDIOM_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("then now today lately mate pal folks everyone guys all so\n    far at the moment please thanks"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WHAT_HAPPENED_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:so|ok|okay|well|and|but|oh|right|hi|hey|hello|morning|now|then)\\s*,?\\s+)*what(?:'s|s|\\s+has|\\s+have)?\\s+happened(?:\\s+(?:today|lately|recently|this\\s+week|this\\s+morning|tonight|then|so\\s+far|at\\s+work|at\\s+school|in\\s+your\\s+day|with\\s+you))*\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AHEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:in\\s+(?:the\\s+)?(?:\\d+\\s+)?(?:days?|weeks?|months?|years?|spring|summer|autumn|winter|"), &g__MONTH()), &py_str(")|next\\s+(?:week|month|year|spring|summer|autumn|winter|weekend|")), &g__WEEKDAY()), &py_str(")|on\\s+")), &g__WEEKDAY()), &py_str("|starting\\s+(?:on\\s+)?")), &g__WEEKDAY()), &py_str("|tomorrow|tonight|this\\s+(?:weekend|week|month|spring|summer|autumn|winter)|soon|coming\\s+up|later\\s+(?:this|in)\\b)"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COUNTED_UP({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:turning|turns?|is|am|im|i'm|are|be|will\\s+be|'ll\\s+be)\\s+\\d{1,3}\\b(?!\\s*(?:pounds?|quid|dollars?|euros?|grand|thousand|percent|%|hours?|minutes?|mins?|miles?|kg|kilos?|litres?|liters?)\\b)(?!\\s+a\\s+(?:month|week|year|hour|day)\\b)(?!\\s+per\\s+\\w+)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PERSON({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|im|i'm|me|we|us|she|he|they)\\b|\\b(?:my|our|his|her|their|the|a)\\s+(?:\\w+\\s+)?(?:son|daughter|kid|child|baby|boy|girl|toddler|teen|teenager|nipper|youngster|unborn|mum|mom|mother|dad|father|nan|nana|gran|granny|grandma|grandad|grandpa|grandmother|grandfather|sister|brother|niece|nephew|cousin|aunt|auntie|uncle|twins?|friend|mate|wife|husband|partner|girlfriend|boyfriend|fiance|fiancee|boss|colleague|dog|puppy|pup|cat|kitten|rabbit|hamster|guinea\\s+pig|horse|pony|pet)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_AGE_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:years?|yrs?)\\s+old\\b|\\bage[sd]?\\b|\\bturning\\b|\\bturns?\\b|\\bbirthday\\b|\\bbday\\b|\\bdue\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COMING_UP(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:getting|get)\\s+(?:married|wed)\\b|\\b(?:our|the|my)\\s+wedding\\b|\\bset\\s+(?:a|the)\\s+date\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:having|expecting)\\s+(?:a\\s+|our\\s+|twins?\\b|a\\s+little\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bbaby\\s+(?:is\\s+|'s\\s+)?due\\b|\\b(?:i'm|im|i am|we're|we are|she's|she is|hes|he is)\\s+due\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bstart(?:s|ing)?\\s+(?:my\\s+|a\\s+|the\\s+|our\\s+)?(?:[\\w-]+\\s+){0,2}?(?:job|role|course|college|uni|university|school|placement|apprenticeship|training|shift|work)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:new\\s+)?(?:job|role|course|shift)\\s+(?:starts|starting|begins|beginning)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bmov(?:e|ing)\\s+(?:houses?|home|flats?|place|into|in|out|to|away|abroad|back)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, g__COUNTED_UP(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bretir(?:e|es|ing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:half\\s+)?marathon\\b|\\b10k\\b|\\btriathlon\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s__SOON_ITSELF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bset\\s+(?:a|the)\\s+date\\b|\\b(?:having|expecting)\\s+a\\s+baby\\b|\\b(?:a|our|the|my)?\\s*baby\\s+(?:is\\s+|'s\\s+)?due\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ALREADY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:yesterday|last\\s+(?:week|month|year|night|weekend)|a\\s+few\\s+(?:days|weeks)\\s+ago|already|used\\s+to)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DREAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dread(?:ing|ed)?|not\\s+looking\\s+forward|worr(?:y|ied|ies|ying)|scared|terrified|nervous|anxious|panic(?:king)?|stress(?:ed|ful)?|afford|terrible|awful|hate|hopeless|no\\s+idea\\s+how|upset|angry|furious|livid|annoyed|not\\s+speaking|not\\s+talking|stopped\\s+speaking|not\\s+happy)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_HAPPENING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:not|never|no\\s+longer|stopped|cancelled?|canceled?|called\\s+off)\\s+(?:\\w+\\s+){0,3}(?:get(?:ting)?\\s+(?:married|wed)|having|expecting|turning|turns?|mov(?:e|ing)|retir(?:e|es|ing))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BARELY_ASKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:is|are|am|was|were|do|does|did|have|has|had|can|could|would|should|shall|may|might|what|when|where|who|why|how)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MONEY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:rent|pay|wage|wages|salary|bill|bills|price|prices|cost|costs|deposit|savings?|income|mortgage|overdraft|loan|loans|debt|debts|arrears|fine|fines|invoice|quote|quotes|tax|council\\s+tax)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__CLAUSE_BREAK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;]|\\b(?:but|and|so|because|though|although)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AHEAD_SPAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\bin\\s+(?:the\\s+)?(?:about\\s+|around\\s+|roughly\\s+|nearly\\s+|almost\\s+|just\\s+over\\s+|over\\s+)?(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a\\s+few|a\\s+couple\\s+of|a|an)?\\s*(?:hours?|days?|weeks?|fortnights?|months?|years?|spring|summer|autumn|winter|"), &g__MONTH()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DONE_ALREADY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:moved|have\\s+moved|had\\s+moved|comes\\s+home|we\\s+did\\s+move)\\b|\\b(?:yesterday|last\\s+(?:week|month|year|weekend))\\b|\\balready\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WIN(py_str("(?:won|win|winner|winnings|prize|prizes|jackpot|lottery|competition|sweepstake)"));
    s__A_MEDIUM(py_str("(?:text|texts|texted|message|messages|email|emails|letter|letters|call|phone call|voicemail|whatsapp|dm|notification)"));
    s__ENTERED_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)\\s+(?:entered|entered it|took part|did the|bought|played|signed up|applied)\\b|\\bscratchcard\\b|\\braffle ticket\\b|\\bticket\\b|\\bmy number came up\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_MESSAGE_THAT_CLAIMS_A_WIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g__A_MEDIUM()), &py_str("\\b(?:(?!")), &g__A_WIN()), &py_str(").){0,60}?\\b")), &g__A_WIN()), &py_str("\\b"))); __args.push(op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WIN_THEN_A_MESSAGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b"), &g__A_WIN()), &py_str("\\b(?:(?!")), &g__A_MEDIUM()), &py_str(").){0,60}?\\b")), &g__A_MEDIUM()), &py_str("\\b"))); __args.push(op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS_FOR_SOMETHING_BACK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fee|fees|pay|paid|send|transfer|wire|release|admin|processing|clearance|customs|bank details|account details|card details|sort code|final notice|get in touch|contact us|text them|text back|call them|ring them|reply|respond|confirm your)\\b|£")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NEVER_ENTERED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:never|did\\s*nt|did\\s+not|do\\s*nt|do\\s+not)\\s+(?:even\\s+)?(?:enter|entered|applied|signed up|took part)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ABOUT_THE_WIN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:about|over)\\s+it\\b|\\bto\\s+(?:tell|let)\\s+me\\b|\\bto\\s+say\\b|\\bconfirming\\b|\\bwell\\s+done\\b|\\bcame\\s+after\\b|\\bafterwards\\b|\\bthe\\s+details\\b|\\bcongratulat\\w*\\b|\\bto\\s+let\\s+us\\s+know\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_RELATIVE_TOLD_ME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:my|our)\\s+)?(?:sister|brother|son|daughter|mum|mam|mother|dad|father|friend|mate|partner|husband|wife|colleague|girlfriend|boyfriend|cousin|nan|gran|granny|aunt|uncle|niece|nephew|neighbour|neighbor)\\b.{0,40}?\\b(?:texted|texts|messaged|messages|told|tells|called|calls|rang|rings|phoned|phones|wrote|whatsapped|emailed|emails)\\b.{0,20}?\\b(?:me|us)\\b")); __args.push(op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_YEAR_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:in|since|back\\s+in|during|of|from)\\s+((?:19|20)\\d\\d)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_JUST_HAPPENED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:just|only\\s+just|this\\s+(?:week|month|year)|today|yesterday|last\\s+(?:night|week)|at\\s+the\\s+weekend|finally)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_YEARS_BACK_LINES(py_tuple(vec![py_str("Good to hear. What made you think of it now?"), py_str("Nice one - what brings it up today?"), py_str("Good to hear. Is it something you still think about?")]));
    s_SAYS_IT_IN_THE_PRESENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:im|i'm|i\\s+am|we\\s+are|we're)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CHEER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:congratulations|well\\s+done|brilliant|lovely(?:\\s+news)?|wonderful|fantastic|that(?:'s|s)?\\s+(?:great|lovely|wonderful|brilliant|fantastic|really\\s+lovely)\\b|good\\s+for\\s+you|so\\s+pleased|really\\s+pleased|lovely\\s+to\\s+hear)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AN_ATTEMPT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:first|second|third|fourth|fifth|sixth|1st|2nd|3rd|4th|5th|6th|last)\\s+(?:time|go|try|attempt|sit)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_NUMBER(py_str("(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten)"));
    s__A_SCORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&g__A_NUMBER(), &py_str("\\s*(?:-|to)\\s*")), &g__A_NUMBER()), &py_str("|")), &g__A_NUMBER()), &py_str("\\s*nil"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OR_A_SCORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^\\s*"), &g__A_NUMBER()), &py_str("\\s*(?:-|to)?\\s*(?:")), &g__A_NUMBER()), &py_str("|nil)\\s*$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_CLOCK_MINUTES(py_str("(?:5|10|15|20|25|30|35|40|45|50|five|ten|quarter|fifteen|twenty|twenty[- ]five|thirty|thirty[- ]five|forty|forty[- ]five|fifty)"));
    s__AN_HOUR(py_str("(?:1|2|3|4|5|6|7|8|9|10|11|12|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)"));
    s__A_CLOCK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("^\\s*"), &g__A_CLOCK_MINUTES()), &py_str("\\s*(?:past|to)\\s*")), &g__AN_HOUR()), &py_str("\\s*$"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_WEIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b\\d+(?:\\.\\d+)?\\s*(?:lb|lbs|pounds?|kg|kilos?|oz|ounces?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__AROUND_THE_FIGURES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    a an the its it's hes shes he she they theyre were was is are both and so just about exactly\n    weighing weighed weigh born baby twins boy girl little pound pounds lb lbs kg kilo kilos oz\n    ounce ounces gram grams g of on in at with each apiece together\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FIGURE_IN_THE_LINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d+(?:\\.\\d+)?\\s*(?:lb|lbs|pounds?|kg|kilos?|oz|ounces?|g|grams?)?")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ASKS_OR_THANKS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\?|\\b(?:what|how|why|when|where|who|which|can|could|should|would|will|shall|do|does|did|is|are|was|were|thanks|thank|thx|ty|ta|cheers|bye|goodbye|night|please|sorry)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MORE_OF_THE_NEWS(py_tuple(vec![py_str("However many goes it took, you have it now."), py_str("The win is the whole of it, not which go it was."), py_str("{X} — the way to have it.")]));
    s_MORE_OF_THE_NEWS_SCORE(py_tuple(vec![py_str("{X} — better than a plain win."), py_str("A score like that makes it a better story."), py_str("Even better than it sounded.")]));
    s_MORE_OF_THE_NEWS_ARRIVAL(py_tuple(vec![py_str("{X} — lovely news. Really pleased for you."), py_str("{X}. Really pleased for you."), py_str("{X} — that is the best kind of news.")]));
    s__MORE_OF_THE_NEWS_MARKS(py_tuple(vec![py_str("however many goes"), py_str("the win is the whole of it"), py_str("the way to have it"), py_str("better than a plain win"), py_str("makes it a better story"), py_str("even better than it sounded"), py_str("the best kind of news")]));
    s_NOBODY(py_str("(?:nobody|no\\s+one|no-one|not\\s+(?:a\\s+)?single\\s+(?:one|person|soul)|none\\s+of\\s+(?:them|us|my\\s+\\w+|the\\s+\\w+))"));
    s__TOLD(py_str("(?:said|says|saying|told\\s+me|tells\\s+me|congratulated|congratulates|noticed|notices|mentioned|mentions|asked|asks|texted|texts|called|calls|rang|replied|replies|phoned|wrote|written|thanked|thanks|clapped|acknowledged|commented|reached\\s+out)"));
    s_NOBODY_SAID_ANYTHING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?P<first>\\b"), &g_NOBODY()), &py_str("\\s+(?:even\\s+|ever\\s+|actually\\s+|\\w+\\s+){0,2}?(?P<verb>")), &g__TOLD()), &py_str(")\\b)|(?P<object>\\b")), &g__TOLD()), &py_str("\\s+(?:me\\s+)?nothing\\b)|\\b(?:didnt|didn't|did\\s+not|doesnt|doesn't|never|not)\\s+(?:even\\s+)?(?:say|said|text|texted|call|called|ask|asked|notice|noticed|mention|mentioned|reply|replied|congratulate|congratulated|thank|thanked|bother|bothered|ring|rang|write|wrote)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;.!?]|\\b(?:and|but|so|yet|cos|because|while|when|if|though|although|then|till|until)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SPEAKER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("i"), py_str("im"), py_str("ive"), py_str("id"), py_str("ill"), py_str("myself")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TAKES_A_CLAUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("noticed"), py_str("notices"), py_str("noticing"), py_str("mentioned"), py_str("mentions"), py_str("mentioning")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SILENCE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("nothing"), py_str("anything"), py_str("a"), py_str("word"), py_str("well"), py_str("done"), py_str("congrats"), py_str("congratulation"), py_str("congratulations"), py_str("nice"), py_str("hello"), py_str("hi"), py_str("hooray"), py_str("bravo"), py_str("much"), py_str("all"), py_str("good"), py_str("luck"), py_str("me"), py_str("us"), py_str("her"), py_str("him"), py_str("them"), py_str("everybody"), py_str("anybody"), py_str("anyone"), py_str("on"), py_str("for"), py_str("about"), py_str("at"), py_str("with"), py_str("to"), py_str("either"), py_str("it"), py_str("that")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLAUSE_STOPS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("and"), py_str("but"), py_str("so"), py_str("yet"), py_str("though"), py_str("although"), py_str("while"), py_str("which"), py_str("when"), py_str("because"), py_str("cos"), py_str("then"), py_str("if")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NEGATED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:dont|don't|doesnt|doesn't|didnt|didn't|did\\s+not|do\\s+not|isnt|isn't|wasnt|wasn't|not|never|no\\s+longer|no)\\s+(?:(?:really|very|much|so|still|any|getting|feel|feels|feeling|felt|seem|seems|be|is|was|am|a|bit|lot|longer)\\s+){0,3}better\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_DENIED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?:\\bnot\\b|\\bnever\\b|\\bno\\s+(?:better|good|improvement)\\b|\\bstill\\b|\\bn't\\b|\\bain'?t\\b|\\b(?:do|does|did|is|are|was|were|has|have|had|could|would|should)n'?t\\b|\\bworse\\b)")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SETTLED(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:is|are|seems?|looks?|feels?)\\s+(?:ok|okay|alright|fine|clear|normal)\\s+(?:now|again)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:ok|okay|alright|fine)\\s+now\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:(?:she|he|it|they)(?:'s|s)?\\s+(?:is|are|was|were)?|(?:is|are|seems?|looks?))\\s*(?:much\\s+|a\\s+lot\\s+|so\\s+much\\s+|loads\\s+)?better\\s+(?:now|today|again|thanks)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:has|have|had)\\s+(?:gone|eased|cleared(?:\\s+up)?|settled(?:\\s+down)?|stopped|come\\s+down)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:went|goes)\\s+(?:away|down)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gone|eased\\s+off|cleared\\s+up|settled\\s+down|died\\s+down)\\s+(?:now|again|overnight|since)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gone|come|went)\\s+(?:down|away|off)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:eased|calmed|settled|cleared)\\s+(?:off|down|up)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:results?|tests?|scans?)\\s+(?:came|come|are|is|were)\\s+(?:back\\s+)?(?:clear|fine|normal|ok|okay|all\\s+clear)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|they)\\s+came\\s+back\\s+(?:clear|fine|normal|ok|okay)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bit'?s\\s+(?:all\\s+)?gone\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:she|he|they)(?:'s|s)?\\s+(?:over\\s+(?:it|the\\s+worst)|back\\s+to\\s+(?:herself|himself|themselves|normal))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:she|he|they)(?:'s|s|\\s+is|\\s+are)?\\s+out\\s+(?:now|today|again)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:all\\s+clear|out\\s+of\\s+(?:hospital|the\\s+woods)|back\\s+to\\s+normal|back\\s+on\\s+(?:his|her|their)\\s+feet|over\\s+the\\s+worst)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:has|have|had|is|are|was|were|s|'s)\\s+(?:all\\s+|now\\s+|finally\\s+)?healed\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhealed\\s+(?:up|over|now|nicely|well)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:it|that|this|it's|its|thats|that's|the\\s+\\w+)\\s*(?:'s|s|\\s+is|\\s+has|\\s+have)?\\s*stopped\\s+(?:now|again|completely|properly|finally)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bnothing\\s+(?:hurts|aches|is\\s+sore|\\w+\\s+hurts)\\s+(?:now|any\\s?more|today|at\\s+all)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_WORRY_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nsick ill poorly unwell fever temperature cough cold flu covid chest throat tummy stomach\nheadache migraine pain ache aches sore hurting swollen swelling rash lump dizzy faint\nvomit vomiting sick diarrhoea infection antibiotics hospital a&e operation surgery scan\nxray x-ray test results test radiography chemo chemotherapy treatment dose medicine\nallergy allergic asthma inhaler epilepsy seizure seizure fit fits bleed bleeding\nvirus bug tonsils ear ears tooth teeth back knee hip ankle wrist shoulder neck leg arm\ntemperature poorly worried scared nervous accident fall fell collapsed plaster cast\nburnt burn burns scald scalded cut cuts wound wounds blood bled bleeding nosebleed nosebleeds\nblister blisters sting stung bite bitten bruise bruised sprain sprained splint fracture\nbroken finger fingers thumb hand hands foot feet toe toes eye eyes gum gums toothache\ndressing bandage stitches stitch plastering\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ASK_OPEN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:so|well|ok|okay|and|but|ah|oh|right|and)?[\\s,]*(?:is|are|was|were|do|does|did|has|have|will|would|can|could|should|any)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_GONE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:gone|went)\\b(?!\\s*(?:down|away|off|all))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BODY_GAVE_WAY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|his|her|their|your|the)\\s+(?:\\w+\\s+)?(?:back|knee|knees|leg|legs|arm|arms|shoulder|shoulders|hip|hips|ankle|ankles|wrist|wrists|neck|foot|feet|hand|hands|elbow|elbows|jaw|stomach|tummy|chest|side)\\b(?!\\s*(?:pain|ache|aches|hurt|hurts|trouble|problem|injury|stiffness|muscle))")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SETTLED_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:(?:now|again|today|tonight|thanks|completely|properly|finally|at\\s+last|anyway|so\\s+far|for\\s+good|down|off|this\\s+week|this\\s+morning|this\\s+afternoon|this\\s+evening|these\\s+days|the\\s+last\\s+few\\s+days|a\\s+few\\s+days|recently|lately|nowadays|for\\s+now|for\\s+a\\s+while)\\s*)*[.!]?$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CLEAR_RESULT(py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:in|out\\s+of)\\s+(?:full\\s+)?remission\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:scan|scans|results?|tests?|biopsy|mri|ct|x-?ray|bloods?|screening|checks?|check\\s?up|swabs?)\\b[^.!?]{0,24}?\\b(?:came\\s+back|come\\s+back|was|were|is|are|looks?|looked)\\s+(?:all\\s+)?(?:clear|normal|fine|negative)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:biopsy|lump|mole|tumou?r|cyst|polyp|growth|cells?|cells)\\b[^.!?]{0,20}?\\b(?:was|were|is|are|turned\\s+out|come\\s+back|came\\s+back)\\s+(?:benign|harmless|nothing\\s+to\\s+worry\\s+about)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:hasnt|hasn't|hadnt|hadn't|has|had)\\s+not\\s+spread\\b|\\bit\\s+(?:hasnt|hasn't|hadnt|hadn't)\\s+spread\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bno\\s+(?:sign|trace|evidence)\\s+of\\s+(?:it|any\\s+cancer|the\\s+cancer|the\\s+disease)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:treatment|chemo|chemotherapy|radiotherapy|surgery|operation|transplant)\\b[^.!?]{0,24}?\\b(?:is\\s+working|worked|went\\s+(?:well|fine)|was\\s+a\\s+success|has\\s+worked)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:tumou?r|lump|cancer|growth)s?\\b[^.!?]{0,16}?\\b(?:has|have)\\s+(?:gone|shrunk|shrinking|stopped\\s+growing)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:its|it\\s+is|it\\s+was|hes|she's|theyre|they're)\\s+(?:shrinking|stopped\\s+growing|gone)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_CLEAR_TAIL({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s*(?:(?:now|again|today|thanks|thank\\s+god|thankfully|thank\\s+fully|finally|at\\s+last|anyway|so\\s+far|great|brilliant|so\\s+(?:relieved|happy|pleased|glad)|relieved|what\\s+a\\s+relief)\\s*)*[.!]?$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_CLEAR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:not|isnt|isn't|wasnt|wasn't|arent|aren't|werent|weren't|never|no)\\s+(?:\\w+\\s+){0,2}?(?:\\w+\\s+)?(?:clear|benign|remission|normal|negative|gone|harmless)\\b|\\b(?:has|had)\\s+spread\\b|\\bhasnt\\s+(?:worked|helped)\\b|\\bdidnt\\s+work\\b|\\bnot\\s+(?:worked|working)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    if py_truthy(&Py::Bool(py_eq(&g___name__(), &py_str("__main__")))) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&gattr(&g_sys(), "argv"), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter1 {
    s_line(__item.clone());
    s_low({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_line(), "lower", args))(&__args) });
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __s = String::new(); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_low()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_good_news(args))(&__args) }, 115, &"5")); __s.push_str(" "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_low()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_pass(args))(&__args) }, 115, &"5")); __s.push_str(" "); __s.push_str(&fstr(&g_line(), -1, "")); py_string(__s) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("print", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
}
pub fn _bare(args: &[Py]) -> Py {
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
    let _ = py_str("The word without a plural ending, so \"two puppies\" names a puppy.\n\n    The thing lists are all written in the singular, and a line about two of something says the\n    plural - \"we got two crates\", \"one of those baby monitors\" - so a plural was never read as a\n    thing that arrived, and the news got a receipt instead of a question about it.\n    ");
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ies")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("es")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("sxz"), &geti(&word.clone(), &op_neg(&Py::Int(3))))) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } } } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _thing_words(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut match_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut bare: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing that arrived, cut at the first word that is not part of its name.");
    kept = py_list(vec![]);
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_POINTER_IN_FRONT(), "sub", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THING_STOPS(), &bare.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_AN_ARRIVAL(), &bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3))) } } } } } }) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    Py::None
}
pub fn new_thing(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The kind and name of a thing that has just arrived, or None.\n\n    Returns `{\"kind\": \"animal\"|\"home\"|\"place\"|\"thing\", \"thing\": \"rescue greyhound\"}`, so the\n    caller can ask a question about the thing without reading the line again.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_FAULT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) } } } } } } }) {
    return Py::None;
    }
    said = clean.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HAVE_GOT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEWS_OF_IT(), "search", args))(&__args) } } }) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\g<who> got")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HAVE_GOT(), "sub", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_MOVED_IN(), "search", args))(&__args) }) {
    return py_dict(vec![(py_str("kind"), py_str("place")), (py_str("thing"), py_str(""))]);
    }
    let __iter1 = py_iter_or_raise(&g_ARRIVED());
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing_words(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("new"), py_str("another"), py_str("first"), py_str("second")]), &thing.clone())) } }) {
    continue;
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __aug2 = op_bitor(&words.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bare(args))(&__args) }); } py_set(__out) })() });
    words = __aug2;
    if py_truthy(&op_bitand(&words.clone(), &g_NOT_AN_ARRIVAL())) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&words.clone(), &op_bitor(&op_bitor(&g_ANIMAL_THING(), &g_HOME_THING()), &g_OBJECT_THING()))))) {
    continue;
    }
    if py_truthy(&op_bitand(&words.clone(), &g_ANIMAL_THING())) {
    return py_dict(vec![(py_str("kind"), py_str("animal")), (py_str("thing"), thing.clone())]);
    }
    if py_truthy(&op_bitand(&words.clone(), &g_HOME_WORK())) {
    return py_dict(vec![(py_str("kind"), py_str("home")), (py_str("thing"), thing.clone())]);
    }
    if py_truthy(&op_bitand(&words.clone(), &g_HOME_THING())) {
    return py_dict(vec![(py_str("kind"), py_str("thing")), (py_str("thing"), thing.clone())]);
    }
    return py_dict(vec![(py_str("kind"), py_str("thing")), (py_str("thing"), thing.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn new_thing_replies(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut arrived: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "arrived") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: arrived"))) };
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The lines an arrival is answered with, the thing's name said back once.");
    kind = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&arrived.clone(), "get", args))(&__args) };
    thing = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("thing")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&arrived.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("place")))) {
    return g_NEW_PLACE();
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kind.clone()); __args.push(g_NEW_THING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_dict(vec![(py_str("animal"), g_NEW_ANIMAL()), (py_str("home"), g_NEW_HOME())]), "get", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_person_word(args: &[Py]) -> Py {
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
    let _ = py_str("The relation word inside a possessive - \"mums\", \"wifes\", \"son's\" - or \"\".");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) }, "strip", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn someone_elses_occasion(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the day named belongs to somebody other than the person writing.\n\n    Only a relation in front of the day makes it somebody else's: \"my sons birthday\", \"my best\n    friends anniversary\". A day with nothing in front of it is theirs, and so is one described\n    rather than owned - \"my 30th birthday\", \"my big birthday\".");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_RELATION_BEFORE_DAY(), "finditer", args))(&__args) });
    for __item in __iter1 {
    found = __item.clone();
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("whose")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "split", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_FAMILY(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_person_word(args))(&__args) }))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ORDINAL_DAY(), "finditer", args))(&__args) });
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FAMILY(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("whose")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_person_word(args))(&__args) }))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_an_occasion(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The occasion a line says is coming up - \"birthday\", \"anniversary\", \"wedding\" - or None.\n\n    The name is for the composer to pick the greeting with; this reader only says which occasion\n    the line is about, so the wording can change without touching this file.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_AN_OCCASION(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LET_DOWN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SOMEONE_LEFT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAD(), "search", args))(&__args) } } } } } } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_OCCASION());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    pattern = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {
    return name.clone();
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ORDINAL_DAY(), "search", args))(&__args) }) {
    return py_str("birthday");
    }
    return Py::None;
    Py::None
}
pub fn _a_hand_over(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut i: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a \"passed\" in these words hands something over rather than reports a result.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("enumerate", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("passed"), py_str("pass")]), &word.clone()))) {
    continue;
    }
    after = slice_or_raise(&words.clone(), &op_add(&i.clone(), &Py::Int(1)), &op_add(&i.clone(), &Py::Int(4)), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&after.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_PASS_PARTICLE(), &geti(&after.clone(), &Py::Int(0))))) {
    return Py::Bool(true);
    }
    thing = if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &geti(&after.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)) } }) { geti(&after.clone(), &Py::Int(1)) } else { geti(&after.clone(), &Py::Int(0)) };
    if py_truthy(&Py::Bool(py_contains(&g_PASS_TAKEN(), &thing.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&after.clone(), &Py::Int(0)), &py_str("it"))); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(after.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PASS_PARTICLE(), &geti(&after.clone(), &Py::Int(1)))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_a_pass(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line reports a pass: \"i passed my driving test\", \"i passed\".\n\n    A bare \"passed\" with a thing after it is a hand-over, not a result, and a question is a\n    question (\"did you pass\"), so neither is one.");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_QUITE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_win_nobody_noticed(args))(&__args) }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_hand_over(args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PASSED()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _an_idiom_standing_alone(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut tail: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when nothing after the idiom names a thing of its own.");
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&low.clone(), &end.clone(), &Py::None, &Py::None), "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(Py::Bool(py_contains(&g_NEWS_IDIOM_TAIL(), &w.clone()))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn asks_for_news(args: &[Py]) -> Py {
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
    let mut idiom: Py = Py::None;
    let mut low: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line asks for the news rather than telling it.");
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("?!.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "strip", args))(&__args) }, "rstrip", args))(&__args) }, "lower", args))(&__args) };
    idiom = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEWS_IDIOM(), "match", args))(&__args) };
    if py_truthy(&{ let __b = idiom.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&idiom.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_idiom_standing_alone(args))(&__args) } } }) {
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HANDED_OVER(), "search", args))(&__args) }));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_HAPPENED_ASK(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEWS_ASK(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEWS_WORD(), "search", args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &py_str("news")))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WORLD_NEWS(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STANDIN(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HANDED_OVER(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&low.clone(), "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NEWS_FRAME(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_contains(&py_tuple(vec![py_str("tell"), py_str("give"), py_str("share"), py_str("spill")]), &geti(&words.clone(), &Py::Int(0))));
    Py::None
}
pub fn _a_money_clause(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let _ = py_str("True when a clause of the line counts something in money rather than in years.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CLAUSE_BREAK(), "split", args))(&__args) });
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MONEY(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\d")); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn ahead_of_now(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line puts its thing ahead of now, by a date or by a span of time.\n\n    Only the date half of `reads_as_news_to_come`: no view on whether the thing is good news, and\n    no view on whether it has already happened - every caller asks its own second question.\n    ");
    clean = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__DONE_ALREADY(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AHEAD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AHEAD_SPAN(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_news_to_come(args: &[Py]) -> Py {
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
    let mut hits: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether the line puts a happy event ahead of now: a date and a thing still to happen.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_QUITE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::badnews::reads_as_someones_death(args))(&__args) } } } } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ALREADY(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__DREAD(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_HAPPENING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BARELY_ASKED(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AHEAD(), "search", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SOON_ITSELF(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we|my|our|im|i'm|we're|were)\\b|\\bthe\\s+baby\\b")); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    hits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COMING_UP()) { let mut pattern: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }) {  __out.push(pattern.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&hits.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hits.clone()) { let mut pattern: Py = __item.clone(); __out.push(Py::Bool(py_is(&pattern.clone(), &g__COUNTED_UP()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_money_clause(args))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(hits.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number_counts_a_life(args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _the_number_counts_a_life(args: &[Py]) -> Py {
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
    let mut hits: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "hits") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: hits"))) };
    let mut clause: Py = Py::None;
    let _ = py_str("True when the counted number is somebody's age, not a rota or a timetable.\n\n    Read on the clause that carries the number, so an age beside another event still reads\n    (\"my son is 5 next week and we are moving house\"), and then on the whole line for a line that\n    says it is an age without a person in the same clause (\"84 years old next tuesday\").\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_AGE_SAID(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CLAUSE_BREAK(), "split", args))(&__args) });
    for __item in __iter1 {
    clause = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COUNTED_UP(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clause.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERSON(), "search", args))(&__args) } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_a_prize_in_a_message(args: &[Py]) -> Py {
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
    let _ = py_str("True when the win in the line came to the person in a message they did not ask for.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ENTERED_IT(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ABOUT_THE_WIN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_RELATIVE_TOLD_ME(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NEVER_ENTERED(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_MESSAGE_THAT_CLAIMS_A_WIN(), "search", args))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WIN_THEN_A_MESSAGE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_FOR_SOMETHING_BACK(), "search", args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn a_year_behind(args: &[Py]) -> Py {
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
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { Py::None };
    let mut match_: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("Whether the line puts the thing in a year that has gone: \"answered in 1998\", \"the class of 2015\".");
    when = { let __b = today.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "date.today", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_YEAR_AFTER(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    let __try2 = exc::catch(|| {
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &gattr(&when.clone(), "year"))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["TypeError", "ValueError"]) {
                __handled6 = true;
    continue;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reply_for_a_year_ago(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let _ = py_str("The line for a win the person puts in a year that has gone, or None for another turn.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_congratulation(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_cheer_was_this_pool(args))(&__args) } } }))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reply_for_a_win_from_a_past_year(args))(&__args) };
    Py::None
}
pub fn reply_for_a_win_from_a_past_year(args: &[Py]) -> Py {
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
    let _ = py_str("The line for a win the person puts in a year that has gone, or None for another line.\n\n    Whether the reply it would have got is a cheer is the caller's question, and the two callers\n    know it differently: `chat.py` has just picked the congratulations pool, and `greeting.py` has\n    the composer's reply in front of it. So this reads the line and nothing else.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_year_behind(args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_JUST_HAPPENED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAYS_IT_IN_THE_PRESENT(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    return geti(&g_YEARS_BACK_LINES(), &op_modulo(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_func("ord", |args: &[Py]| call_builtin("ord", args).unwrap_or_else(|e| exc::raise(e)))); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("map", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sum", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_YEARS_BACK_LINES()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn the_cheer_was_this_pool(args: &[Py]) -> Py {
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
    let mut note: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the reply the person just got came from the good-news pool, whatever it said.\n\n    A wording list over the composer's prose is always a month behind the prose, and the pool has\n    lines that no congratulations pattern reads (\"Love that - well done. How are you celebrating?\",\n    \"Good stuff. That must be a weight off.\"). `chat._say` notes the pool where the reply is\n    chosen, and the arbiter keeps the notes of the module whose reply the person got, so the note\n    cannot be there for a reply another module gave.\n    ");
    note = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("said_kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(note.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&note.clone(), "get", args))(&__args) }, &py_str("good_news"))) } }) {
    return Py::Bool(false);
    }
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    return Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&note.clone(), "get", args))(&__args) }, &if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { Py::Int(0) }));
    Py::None
}
pub fn reads_as_a_congratulation(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the composer's line is a cheer for something that has just happened.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CHEER(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _only_the_arrivals_figures(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether every word of the line is the sex, a figure, or the words that join them.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bare.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__AROUND_THE_FIGURES(), &bare.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FIGURE_IN_THE_LINE(), "fullmatch", args))(&__args) }) {
    continue;
    }
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn reads_as_the_detail_line(args: &[Py]) -> Py {
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
    let mut mark: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether a reply is this module's own wording for a detail added to the news.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MORE_OF_THE_NEWS_MARKS()) { let mut mark: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &mark.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_reply_just_cheered(args: &[Py]) -> Py {
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
    let mut note: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the reply the person just got came from the good-news pool.\n\n    `the_cheer_was_this_pool` compares the note the composer makes with the number of turns, and a\n    note made while the reply is being chosen counts the turns before it, so on the turn after the\n    cheer it is one short. This reads the note the way the next turn finds it: one back from the\n    turn just finished, and from this module, so a note left by an older cheer cannot answer for\n    the reply in between.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    note = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("said_kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(note.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kind")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&note.clone(), "get", args))(&__args) }, &py_str("good_news"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = geti(&turns.clone(), &op_neg(&Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("module")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&last.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &py_str("chat"))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turn")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&note.clone(), "get", args))(&__args) }, &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))));
    Py::None
}
pub fn a_detail_of_the_news(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("The shape of a detail added to good news, or \"\" for another line.\n\n    \"attempt\" for which go it was (\"first time\", \"third go\"), \"score\" for the result of the win\n    (\"2-0\", \"three nil\"), \"arrival\" for a new baby's figures (\"a boy, 8 pounds\", \"7lb 2oz\"). It\n    reads the line and the turn before it, nothing else: whether the cheer is the bot's own is\n    `the_cheer_was_this_pool`, which the caller knows.\n    ");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(6)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_WEIGHT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&said.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_the_arrivals_figures(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_birth_just_announced(args))(&__args) } } } } } } }) {
    return py_str("arrival");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_cheer_was_this_pool(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_reply_just_cheered(args))(&__args) } } })) } }) {
    return py_str("");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_ATTEMPT(), "search", args))(&__args) }) {
    return py_str("attempt");
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OR_A_SCORE(), "match", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_SCORE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CLOCK(), "match", args))(&__args) })) } } } }) {
    return py_str("score");
    }
    return py_str("");
    Py::None
}
pub fn a_birth_just_announced(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the person announced a birth in one of the two lines before this one.\n\n    The weight is what makes a birth detail a birth detail, and a weight on its own says nothing:\n    \"its 8 pounds\" is a wage or a parcel. The chat is what says which - the birth was the news, and\n    the figures are the detail of it.\n    ");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BABY()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_a_detail_of_the_news(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line adds a detail to news the bot has just cheered, with nothing asked in it.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ASKS_OR_THANKS(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_detail_of_the_news(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_good_news(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line reports a happy event as a plain phrase.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_news_to_come(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_QUITE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::badnews::reads_as_someones_death(args))(&__args) } } } } } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| asks_for_news(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_win_nobody_noticed(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| reads_as_a_prize_in_a_message(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TURN_IN_THE_TALE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SETBACK_AFTER_THE_WIN(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&clean.clone(), "split", args))(&__args) }) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_hand_over(args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__I_PRAISED(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&g_BABY(), &g_PASSED()), &g_WIN())) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GOOD()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_speaker_is_the_quiet_one(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the words in front of the match name the person talking as the one who said nothing.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLAUSE(), "split", args))(&__args) }, &op_neg(&Py::Int(1))), "split", args))(&__args) }, &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&w.clone(), "strip", args))(&__args) }); } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SPEAKER(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_silence_is_the_whole_of_it(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut bare: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether nothing but the silence follows the verb in a silence match.\n\n    It holds for both arms that end on a verb of speaking, not only the `nobody said ...` one:\n    \"i got the job and they never asked me for id\" and \"i got the job but they never mentioned the\n    hours\" wear the same words as a silence and are about people who said plenty. Only the arm\n    that puts `nothing` after the verb is the silence whatever follows it.");
    if py_truthy(&Py::Bool(!py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("object")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, &Py::None))) {
    return Py::Bool(true);
    }
    verb = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("verb")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "split", args))(&__args) }, &op_neg(&Py::Int(1)), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g_TAKES_A_CLAUSE(), &verb.clone()))) {
    return Py::Bool(true);
    }
    rest = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,.;!?]")); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&rest.clone(), "split", args))(&__args) });
    for __item in __iter1 {
    word = __item.clone();
    bare = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(".,!?;:\"'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&g_CLAUSE_STOPS(), &bare.clone()))) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(bare.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&kept.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut bare: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_SILENCE_WORDS(), &bare.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_a_win_a_cost_came_with(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a line reports a win and the cost of it in the same breath: \"we finally got the\n    keys and the place is a state\".\n\n    The sibling of `reads_as_a_win_nobody_noticed` and read the same way: both halves are needed,\n    so a setback on its own is somebody's trouble and a win on its own is just good news. The\n    difference is what the second half says - a silence around the win, or the price of it.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_QUITE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GOOD()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TURN_IN_THE_TALE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_SETBACK_AFTER_THE_WIN(), "search", args))(&__args) } } }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_GOOD_ENDING(), "search", args))(&__args) }));
    Py::None
}
pub fn reads_as_a_win_nobody_noticed(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a line reports a win and a silence around it: \"i got promoted and my mum said\n    nothing\". Both halves are needed - a silence on its own is somebody letting them down, which\n    is answered elsewhere, and a win on its own is just good news - and the silence has to be\n    somebody else's, not the speaker's own.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_QUITE(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::compose::doubted(args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GOOD()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOBODY_SAID_ANYTHING(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_speaker_is_the_quiet_one(args))(&__args) }) {
    continue;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_silence_is_the_whole_of_it(args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _denied(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { Py::None };
    let _ = py_str("Whether a negator sits in the phrase or just before it, which turns news to no news.\n\n    The whole match is searched and not only the run-up: \"my knee doesnt feel better\" starts its\n    match at \"knee\", so a check on the run-up alone would read it as the good news it denies.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&start.clone(), &Py::Int(26))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &if py_truthy(&Py::Bool(!py_is(&end.clone(), &Py::None))) { end.clone() } else { start.clone() }, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_DENIED(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn reads_as_recovery(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line reports that something has got better.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_THEM(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BAD(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEGATED(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RECOVERY()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn worry_on_the_table(args: &[Py]) -> Py {
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
    let mut look_back: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "look_back") { v.clone() }
    else { Py::Int(4) };
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last few things the person said named a body, an illness or a worry.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(t.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(t.clone()); } } py_list(__out) })() };
    seen = Py::Int(0);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &g_WORRY_WORDS())) {
    return Py::Bool(true);
    }
    let __aug2 = op_add(&seen.clone(), &Py::Int(1));
    seen = __aug2;
    if py_truthy(&op_ge(&seen.clone(), &look_back.clone())) {
    break;
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn the_tail_adds_nothing(args: &[Py]) -> Py {
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
    let mut end: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut tail: Py = Py::None;
    let _ = py_str("Whether what follows a settled phrase is only the words that add nothing (SETTLED_TAIL).");
    tail = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[,;:!?]")); __args.push(py_str(" ")); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &end.clone(), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "strip", args))(&__args) };
    return { let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str(" "), &tail.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SETTLED_TAIL(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn _a_bare_gone_is_not_the_news(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let _ = py_str("Whether a match that turns on \"gone\"/\"went\" is still the news, once the line is read.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_GONE(), "search", args))(&__args) }))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bgone\\s+again\\b")); __args.push(text.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BODY_GAVE_WAY(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_tail_adds_nothing(args))(&__args) };
    Py::None
}
pub fn the_news_is_the_whole_of_it(args: &[Py]) -> Py {
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
    let mut end: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the recovery or settled phrase in the line closes the clause.\n\n    \"the rash has cleared up\" and \"its stopped now\" are the end of the trouble; \"my cough has gone\n    green\", \"the pain has gone down my leg\" and \"hes gone off his food\" carry the same words and\n    report something else. `reads_as_recovery` reads the words, so the reader that *stands an\n    answer down* has to check what follows them: a cough that has gone green wants the\n    chesty-cough card, and \"That's a relief\" at it is the wrong reply at a symptom that changed.\n    The tail is only the words that add nothing (see `SETTLED_TAIL`), so a line with anything else\n    after the phrase is a report of its own and keeps its own answer.\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    end = Py::Int(0);
    let __iter1 = py_iter_or_raise(&op_add(&g_RECOVERY(), &g_SETTLED()));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    end = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(end.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(end.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &end.clone(), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SETTLED_TAIL(), "match", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    Py::None
}
pub fn reads_as_a_clear_result(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line says a test or a treatment came back well.\n\n    Said about anyone: \"my mums cancer is in remission\", \"the biopsy was benign\", \"his treatment\n    worked\". The speaker is not the subject in half of these, so this does not use `ABOUT_THEM`,\n    which is the guard for the person's own body (\"i feel better\").\n    ");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_OPEN(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_CLEAR(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NEGATED(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_NEWS(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_QUITE(), "search", args))(&__args) } } } } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_CLEAR_RESULT());
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&text.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CLEAR_TAIL(), "match", args))(&__args) }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_settled(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a line says the thing they were worried about has passed, about anyone.");
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASK_OPEN(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CONDITIONAL(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SETTLED()) { let mut pattern: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "search", args))(&__args) }); } py_list(__out) })() }) { let mut match_: Py = __item.clone(); __out.push({ let __b = match_.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _denied(args))(&__args) })); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(match_.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_bare_gone_is_not_the_news(args))(&__args) } } } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.goodnews".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THEM" => Some(g_ABOUT_THEM()),
            "ANIMAL_THING" => Some(g_ANIMAL_THING()),
            "ARRIVED" => Some(g_ARRIVED()),
            "ASK_OPEN" => Some(g_ASK_OPEN()),
            "A_GAP_WITHOUT_A_NO" => Some(g_A_GAP_WITHOUT_A_NO()),
            "A_GOOD_ENDING" => Some(g_A_GOOD_ENDING()),
            "A_POINTER_IN_FRONT" => Some(g_A_POINTER_IN_FRONT()),
            "A_SETBACK_AFTER_THE_WIN" => Some(g_A_SETBACK_AFTER_THE_WIN()),
            "A_TURN_IN_THE_TALE" => Some(g_A_TURN_IN_THE_TALE()),
            "A_YEAR_AFTER" => Some(g_A_YEAR_AFTER()),
            "BABY" => Some(g_BABY()),
            "BAD" => Some(g_BAD()),
            "BARE_GONE" => Some(g_BARE_GONE()),
            "BODY_GAVE_WAY" => Some(g_BODY_GAVE_WAY()),
            "CLAUSE" => Some(g_CLAUSE()),
            "CLAUSE_STOPS" => Some(g_CLAUSE_STOPS()),
            "CLEAR_RESULT" => Some(g_CLEAR_RESULT()),
            "CLEAR_TAIL" => Some(g_CLEAR_TAIL()),
            "CONDITIONAL" => Some(g_CONDITIONAL()),
            "DENIED" => Some(g_DENIED()),
            "FAMILY" => Some(g_FAMILY()),
            "FAULT" => Some(g_FAULT()),
            "GOOD" => Some(g_GOOD()),
            "HANDED_OVER" => Some(g_HANDED_OVER()),
            "HAVE_GOT" => Some(g_HAVE_GOT()),
            "HOME_THING" => Some(g_HOME_THING()),
            "HOME_WORK" => Some(g_HOME_WORK()),
            "JUST_HAPPENED" => Some(g_JUST_HAPPENED()),
            "LET_DOWN" => Some(g_LET_DOWN()),
            "MORE_OF_THE_NEWS" => Some(g_MORE_OF_THE_NEWS()),
            "MORE_OF_THE_NEWS_ARRIVAL" => Some(g_MORE_OF_THE_NEWS_ARRIVAL()),
            "MORE_OF_THE_NEWS_SCORE" => Some(g_MORE_OF_THE_NEWS_SCORE()),
            "MOVED_IN" => Some(g_MOVED_IN()),
            "NEARLY" => Some(g_NEARLY()),
            "NEGATED" => Some(g_NEGATED()),
            "NEWS_ASK" => Some(g_NEWS_ASK()),
            "NEWS_FRAME" => Some(g_NEWS_FRAME()),
            "NEWS_IDIOM" => Some(g_NEWS_IDIOM()),
            "NEWS_IDIOM_TAIL" => Some(g_NEWS_IDIOM_TAIL()),
            "NEWS_OF_IT" => Some(g_NEWS_OF_IT()),
            "NEWS_WORD" => Some(g_NEWS_WORD()),
            "NEW_ANIMAL" => Some(g_NEW_ANIMAL()),
            "NEW_HOME" => Some(g_NEW_HOME()),
            "NEW_PLACE" => Some(g_NEW_PLACE()),
            "NEW_THING" => Some(g_NEW_THING()),
            "NOBODY" => Some(g_NOBODY()),
            "NOBODY_SAID_ANYTHING" => Some(g_NOBODY_SAID_ANYTHING()),
            "NOT_AN_ARRIVAL" => Some(g_NOT_AN_ARRIVAL()),
            "NOT_AN_OCCASION" => Some(g_NOT_AN_OCCASION()),
            "NOT_CLEAR" => Some(g_NOT_CLEAR()),
            "NOT_NEWS" => Some(g_NOT_NEWS()),
            "NOT_QUITE" => Some(g_NOT_QUITE()),
            "OBJECT_THING" => Some(g_OBJECT_THING()),
            "OCCASION" => Some(g_OCCASION()),
            "ORDINAL_DAY" => Some(g_ORDINAL_DAY()),
            "PASSED" => Some(g_PASSED()),
            "PASS_PARTICLE" => Some(g_PASS_PARTICLE()),
            "PASS_TAKEN" => Some(g_PASS_TAKEN()),
            "PRAISED" => Some(g_PRAISED()),
            "RECOVERY" => Some(g_RECOVERY()),
            "RELATION_BEFORE_DAY" => Some(g_RELATION_BEFORE_DAY()),
            "SAYS_IT_IN_THE_PRESENT" => Some(g_SAYS_IT_IN_THE_PRESENT()),
            "SETTLED" => Some(g_SETTLED()),
            "SETTLED_TAIL" => Some(g_SETTLED_TAIL()),
            "SILENCE_WORDS" => Some(g_SILENCE_WORDS()),
            "SOMEONE_LEFT" => Some(g_SOMEONE_LEFT()),
            "SPEAKER" => Some(g_SPEAKER()),
            "TAKES_A_CLAUSE" => Some(g_TAKES_A_CLAUSE()),
            "THING_STOPS" => Some(g_THING_STOPS()),
            "WHAT_HAPPENED_ASK" => Some(g_WHAT_HAPPENED_ASK()),
            "WIN" => Some(g_WIN()),
            "WORLD_NEWS" => Some(g_WORLD_NEWS()),
            "WORRY_WORDS" => Some(g_WORRY_WORDS()),
            "YEARS_BACK_LINES" => Some(g_YEARS_BACK_LINES()),
            "_ABOUT_THE_WIN" => Some(g__ABOUT_THE_WIN()),
            "_AHEAD" => Some(g__AHEAD()),
            "_AHEAD_SPAN" => Some(g__AHEAD_SPAN()),
            "_ALREADY" => Some(g__ALREADY()),
            "_AN_AGE_SAID" => Some(g__AN_AGE_SAID()),
            "_AN_ATTEMPT" => Some(g__AN_ATTEMPT()),
            "_AN_HOUR" => Some(g__AN_HOUR()),
            "_AROUND_THE_FIGURES" => Some(g__AROUND_THE_FIGURES()),
            "_ASKS_FOR_SOMETHING_BACK" => Some(g__ASKS_FOR_SOMETHING_BACK()),
            "_ASKS_OR_THANKS" => Some(g__ASKS_OR_THANKS()),
            "_AWARD" => Some(g__AWARD()),
            "_A_CHEER" => Some(g__A_CHEER()),
            "_A_CLASS" => Some(g__A_CLASS()),
            "_A_CLOCK" => Some(g__A_CLOCK()),
            "_A_CLOCK_MINUTES" => Some(g__A_CLOCK_MINUTES()),
            "_A_COUNT" => Some(g__A_COUNT()),
            "_A_COURSE" => Some(g__A_COURSE()),
            "_A_DATE" => Some(g__A_DATE()),
            "_A_FIGURE_IN_THE_LINE" => Some(g__A_FIGURE_IN_THE_LINE()),
            "_A_MEDIUM" => Some(g__A_MEDIUM()),
            "_A_MESSAGE_THAT_CLAIMS_A_WIN" => Some(g__A_MESSAGE_THAT_CLAIMS_A_WIN()),
            "_A_NUMBER" => Some(g__A_NUMBER()),
            "_A_PERSON" => Some(g__A_PERSON()),
            "_A_RELATIVE_TOLD_ME" => Some(g__A_RELATIVE_TOLD_ME()),
            "_A_SCORE" => Some(g__A_SCORE()),
            "_A_WEIGHT" => Some(g__A_WEIGHT()),
            "_A_WIN" => Some(g__A_WIN()),
            "_A_WIN_THEN_A_MESSAGE" => Some(g__A_WIN_THEN_A_MESSAGE()),
            "_BARELY_ASKED" => Some(g__BARELY_ASKED()),
            "_CLAUSE_BREAK" => Some(g__CLAUSE_BREAK()),
            "_COMING_UP" => Some(g__COMING_UP()),
            "_COUNTED_UP" => Some(g__COUNTED_UP()),
            "_DONE_ALREADY" => Some(g__DONE_ALREADY()),
            "_DREAD" => Some(g__DREAD()),
            "_DUE" => Some(g__DUE()),
            "_ENTERED_IT" => Some(g__ENTERED_IT()),
            "_I_PRAISED" => Some(g__I_PRAISED()),
            "_MONEY" => Some(g__MONEY()),
            "_MONTH" => Some(g__MONTH()),
            "_MORE_OF_THE_NEWS_MARKS" => Some(g__MORE_OF_THE_NEWS_MARKS()),
            "_MY_OWN" => Some(g__MY_OWN()),
            "_NEVER_ENTERED" => Some(g__NEVER_ENTERED()),
            "_NOT_HAPPENING" => Some(g__NOT_HAPPENING()),
            "_OR_A_SCORE" => Some(g__OR_A_SCORE()),
            "_PERSON_DUE" => Some(g__PERSON_DUE()),
            "_PRAISING" => Some(g__PRAISING()),
            "_SAID" => Some(g__SAID()),
            "_SOON_ITSELF" => Some(g__SOON_ITSELF()),
            "_STANDIN" => Some(g__STANDIN()),
            "_TOLD" => Some(g__TOLD()),
            "_WEEKDAY" => Some(g__WEEKDAY()),
            "_WHEN_DUE" => Some(g__WHEN_DUE()),
            "_WHO_DUE" => Some(g__WHO_DUE()),
            "_a_bare_gone_is_not_the_news" => Some(py_func("_a_bare_gone_is_not_the_news", |args: &[Py]| _a_bare_gone_is_not_the_news(args))),
            "_a_hand_over" => Some(py_func("_a_hand_over", |args: &[Py]| _a_hand_over(args))),
            "_a_money_clause" => Some(py_func("_a_money_clause", |args: &[Py]| _a_money_clause(args))),
            "_an_idiom_standing_alone" => Some(py_func("_an_idiom_standing_alone", |args: &[Py]| _an_idiom_standing_alone(args))),
            "_bare" => Some(py_func("_bare", |args: &[Py]| _bare(args))),
            "_denied" => Some(py_func("_denied", |args: &[Py]| _denied(args))),
            "_only_the_arrivals_figures" => Some(py_func("_only_the_arrivals_figures", |args: &[Py]| _only_the_arrivals_figures(args))),
            "_the_number_counts_a_life" => Some(py_func("_the_number_counts_a_life", |args: &[Py]| _the_number_counts_a_life(args))),
            "_the_person_word" => Some(py_func("_the_person_word", |args: &[Py]| _the_person_word(args))),
            "_the_silence_is_the_whole_of_it" => Some(py_func("_the_silence_is_the_whole_of_it", |args: &[Py]| _the_silence_is_the_whole_of_it(args))),
            "_the_speaker_is_the_quiet_one" => Some(py_func("_the_speaker_is_the_quiet_one", |args: &[Py]| _the_speaker_is_the_quiet_one(args))),
            "_thing_words" => Some(py_func("_thing_words", |args: &[Py]| _thing_words(args))),
            "a_birth_just_announced" => Some(py_func("a_birth_just_announced", |args: &[Py]| a_birth_just_announced(args))),
            "a_detail_of_the_news" => Some(py_func("a_detail_of_the_news", |args: &[Py]| a_detail_of_the_news(args))),
            "a_year_behind" => Some(py_func("a_year_behind", |args: &[Py]| a_year_behind(args))),
            "ahead_of_now" => Some(py_func("ahead_of_now", |args: &[Py]| ahead_of_now(args))),
            "asks_for_news" => Some(py_func("asks_for_news", |args: &[Py]| asks_for_news(args))),
            "line" => Some(g_line()),
            "low" => Some(g_low()),
            "new_thing" => Some(py_func("new_thing", |args: &[Py]| new_thing(args))),
            "new_thing_replies" => Some(py_func("new_thing_replies", |args: &[Py]| new_thing_replies(args))),
            "reads_as_a_clear_result" => Some(py_func("reads_as_a_clear_result", |args: &[Py]| reads_as_a_clear_result(args))),
            "reads_as_a_congratulation" => Some(py_func("reads_as_a_congratulation", |args: &[Py]| reads_as_a_congratulation(args))),
            "reads_as_a_detail_of_the_news" => Some(py_func("reads_as_a_detail_of_the_news", |args: &[Py]| reads_as_a_detail_of_the_news(args))),
            "reads_as_a_pass" => Some(py_func("reads_as_a_pass", |args: &[Py]| reads_as_a_pass(args))),
            "reads_as_a_prize_in_a_message" => Some(py_func("reads_as_a_prize_in_a_message", |args: &[Py]| reads_as_a_prize_in_a_message(args))),
            "reads_as_a_win_a_cost_came_with" => Some(py_func("reads_as_a_win_a_cost_came_with", |args: &[Py]| reads_as_a_win_a_cost_came_with(args))),
            "reads_as_a_win_nobody_noticed" => Some(py_func("reads_as_a_win_nobody_noticed", |args: &[Py]| reads_as_a_win_nobody_noticed(args))),
            "reads_as_an_occasion" => Some(py_func("reads_as_an_occasion", |args: &[Py]| reads_as_an_occasion(args))),
            "reads_as_good_news" => Some(py_func("reads_as_good_news", |args: &[Py]| reads_as_good_news(args))),
            "reads_as_news_to_come" => Some(py_func("reads_as_news_to_come", |args: &[Py]| reads_as_news_to_come(args))),
            "reads_as_recovery" => Some(py_func("reads_as_recovery", |args: &[Py]| reads_as_recovery(args))),
            "reads_as_settled" => Some(py_func("reads_as_settled", |args: &[Py]| reads_as_settled(args))),
            "reads_as_the_detail_line" => Some(py_func("reads_as_the_detail_line", |args: &[Py]| reads_as_the_detail_line(args))),
            "reply_for_a_win_from_a_past_year" => Some(py_func("reply_for_a_win_from_a_past_year", |args: &[Py]| reply_for_a_win_from_a_past_year(args))),
            "reply_for_a_year_ago" => Some(py_func("reply_for_a_year_ago", |args: &[Py]| reply_for_a_year_ago(args))),
            "someone_elses_occasion" => Some(py_func("someone_elses_occasion", |args: &[Py]| someone_elses_occasion(args))),
            "the_cheer_was_this_pool" => Some(py_func("the_cheer_was_this_pool", |args: &[Py]| the_cheer_was_this_pool(args))),
            "the_news_is_the_whole_of_it" => Some(py_func("the_news_is_the_whole_of_it", |args: &[Py]| the_news_is_the_whole_of_it(args))),
            "the_reply_just_cheered" => Some(py_func("the_reply_just_cheered", |args: &[Py]| the_reply_just_cheered(args))),
            "the_tail_adds_nothing" => Some(py_func("the_tail_adds_nothing", |args: &[Py]| the_tail_adds_nothing(args))),
            "worry_on_the_table" => Some(py_func("worry_on_the_table", |args: &[Py]| worry_on_the_table(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THEM" => s_ABOUT_THEM(v),
            "ANIMAL_THING" => s_ANIMAL_THING(v),
            "ARRIVED" => s_ARRIVED(v),
            "ASK_OPEN" => s_ASK_OPEN(v),
            "A_GAP_WITHOUT_A_NO" => s_A_GAP_WITHOUT_A_NO(v),
            "A_GOOD_ENDING" => s_A_GOOD_ENDING(v),
            "A_POINTER_IN_FRONT" => s_A_POINTER_IN_FRONT(v),
            "A_SETBACK_AFTER_THE_WIN" => s_A_SETBACK_AFTER_THE_WIN(v),
            "A_TURN_IN_THE_TALE" => s_A_TURN_IN_THE_TALE(v),
            "A_YEAR_AFTER" => s_A_YEAR_AFTER(v),
            "BABY" => s_BABY(v),
            "BAD" => s_BAD(v),
            "BARE_GONE" => s_BARE_GONE(v),
            "BODY_GAVE_WAY" => s_BODY_GAVE_WAY(v),
            "CLAUSE" => s_CLAUSE(v),
            "CLAUSE_STOPS" => s_CLAUSE_STOPS(v),
            "CLEAR_RESULT" => s_CLEAR_RESULT(v),
            "CLEAR_TAIL" => s_CLEAR_TAIL(v),
            "CONDITIONAL" => s_CONDITIONAL(v),
            "DENIED" => s_DENIED(v),
            "FAMILY" => s_FAMILY(v),
            "FAULT" => s_FAULT(v),
            "GOOD" => s_GOOD(v),
            "HANDED_OVER" => s_HANDED_OVER(v),
            "HAVE_GOT" => s_HAVE_GOT(v),
            "HOME_THING" => s_HOME_THING(v),
            "HOME_WORK" => s_HOME_WORK(v),
            "JUST_HAPPENED" => s_JUST_HAPPENED(v),
            "LET_DOWN" => s_LET_DOWN(v),
            "MORE_OF_THE_NEWS" => s_MORE_OF_THE_NEWS(v),
            "MORE_OF_THE_NEWS_ARRIVAL" => s_MORE_OF_THE_NEWS_ARRIVAL(v),
            "MORE_OF_THE_NEWS_SCORE" => s_MORE_OF_THE_NEWS_SCORE(v),
            "MOVED_IN" => s_MOVED_IN(v),
            "NEARLY" => s_NEARLY(v),
            "NEGATED" => s_NEGATED(v),
            "NEWS_ASK" => s_NEWS_ASK(v),
            "NEWS_FRAME" => s_NEWS_FRAME(v),
            "NEWS_IDIOM" => s_NEWS_IDIOM(v),
            "NEWS_IDIOM_TAIL" => s_NEWS_IDIOM_TAIL(v),
            "NEWS_OF_IT" => s_NEWS_OF_IT(v),
            "NEWS_WORD" => s_NEWS_WORD(v),
            "NEW_ANIMAL" => s_NEW_ANIMAL(v),
            "NEW_HOME" => s_NEW_HOME(v),
            "NEW_PLACE" => s_NEW_PLACE(v),
            "NEW_THING" => s_NEW_THING(v),
            "NOBODY" => s_NOBODY(v),
            "NOBODY_SAID_ANYTHING" => s_NOBODY_SAID_ANYTHING(v),
            "NOT_AN_ARRIVAL" => s_NOT_AN_ARRIVAL(v),
            "NOT_AN_OCCASION" => s_NOT_AN_OCCASION(v),
            "NOT_CLEAR" => s_NOT_CLEAR(v),
            "NOT_NEWS" => s_NOT_NEWS(v),
            "NOT_QUITE" => s_NOT_QUITE(v),
            "OBJECT_THING" => s_OBJECT_THING(v),
            "OCCASION" => s_OCCASION(v),
            "ORDINAL_DAY" => s_ORDINAL_DAY(v),
            "PASSED" => s_PASSED(v),
            "PASS_PARTICLE" => s_PASS_PARTICLE(v),
            "PASS_TAKEN" => s_PASS_TAKEN(v),
            "PRAISED" => s_PRAISED(v),
            "RECOVERY" => s_RECOVERY(v),
            "RELATION_BEFORE_DAY" => s_RELATION_BEFORE_DAY(v),
            "SAYS_IT_IN_THE_PRESENT" => s_SAYS_IT_IN_THE_PRESENT(v),
            "SETTLED" => s_SETTLED(v),
            "SETTLED_TAIL" => s_SETTLED_TAIL(v),
            "SILENCE_WORDS" => s_SILENCE_WORDS(v),
            "SOMEONE_LEFT" => s_SOMEONE_LEFT(v),
            "SPEAKER" => s_SPEAKER(v),
            "TAKES_A_CLAUSE" => s_TAKES_A_CLAUSE(v),
            "THING_STOPS" => s_THING_STOPS(v),
            "WHAT_HAPPENED_ASK" => s_WHAT_HAPPENED_ASK(v),
            "WIN" => s_WIN(v),
            "WORLD_NEWS" => s_WORLD_NEWS(v),
            "WORRY_WORDS" => s_WORRY_WORDS(v),
            "YEARS_BACK_LINES" => s_YEARS_BACK_LINES(v),
            "_ABOUT_THE_WIN" => s__ABOUT_THE_WIN(v),
            "_AHEAD" => s__AHEAD(v),
            "_AHEAD_SPAN" => s__AHEAD_SPAN(v),
            "_ALREADY" => s__ALREADY(v),
            "_AN_AGE_SAID" => s__AN_AGE_SAID(v),
            "_AN_ATTEMPT" => s__AN_ATTEMPT(v),
            "_AN_HOUR" => s__AN_HOUR(v),
            "_AROUND_THE_FIGURES" => s__AROUND_THE_FIGURES(v),
            "_ASKS_FOR_SOMETHING_BACK" => s__ASKS_FOR_SOMETHING_BACK(v),
            "_ASKS_OR_THANKS" => s__ASKS_OR_THANKS(v),
            "_AWARD" => s__AWARD(v),
            "_A_CHEER" => s__A_CHEER(v),
            "_A_CLASS" => s__A_CLASS(v),
            "_A_CLOCK" => s__A_CLOCK(v),
            "_A_CLOCK_MINUTES" => s__A_CLOCK_MINUTES(v),
            "_A_COUNT" => s__A_COUNT(v),
            "_A_COURSE" => s__A_COURSE(v),
            "_A_DATE" => s__A_DATE(v),
            "_A_FIGURE_IN_THE_LINE" => s__A_FIGURE_IN_THE_LINE(v),
            "_A_MEDIUM" => s__A_MEDIUM(v),
            "_A_MESSAGE_THAT_CLAIMS_A_WIN" => s__A_MESSAGE_THAT_CLAIMS_A_WIN(v),
            "_A_NUMBER" => s__A_NUMBER(v),
            "_A_PERSON" => s__A_PERSON(v),
            "_A_RELATIVE_TOLD_ME" => s__A_RELATIVE_TOLD_ME(v),
            "_A_SCORE" => s__A_SCORE(v),
            "_A_WEIGHT" => s__A_WEIGHT(v),
            "_A_WIN" => s__A_WIN(v),
            "_A_WIN_THEN_A_MESSAGE" => s__A_WIN_THEN_A_MESSAGE(v),
            "_BARELY_ASKED" => s__BARELY_ASKED(v),
            "_CLAUSE_BREAK" => s__CLAUSE_BREAK(v),
            "_COMING_UP" => s__COMING_UP(v),
            "_COUNTED_UP" => s__COUNTED_UP(v),
            "_DONE_ALREADY" => s__DONE_ALREADY(v),
            "_DREAD" => s__DREAD(v),
            "_DUE" => s__DUE(v),
            "_ENTERED_IT" => s__ENTERED_IT(v),
            "_I_PRAISED" => s__I_PRAISED(v),
            "_MONEY" => s__MONEY(v),
            "_MONTH" => s__MONTH(v),
            "_MORE_OF_THE_NEWS_MARKS" => s__MORE_OF_THE_NEWS_MARKS(v),
            "_MY_OWN" => s__MY_OWN(v),
            "_NEVER_ENTERED" => s__NEVER_ENTERED(v),
            "_NOT_HAPPENING" => s__NOT_HAPPENING(v),
            "_OR_A_SCORE" => s__OR_A_SCORE(v),
            "_PERSON_DUE" => s__PERSON_DUE(v),
            "_PRAISING" => s__PRAISING(v),
            "_SAID" => s__SAID(v),
            "_SOON_ITSELF" => s__SOON_ITSELF(v),
            "_STANDIN" => s__STANDIN(v),
            "_TOLD" => s__TOLD(v),
            "_WEEKDAY" => s__WEEKDAY(v),
            "_WHEN_DUE" => s__WHEN_DUE(v),
            "_WHO_DUE" => s__WHO_DUE(v),
            "line" => s_line(v),
            "low" => s_low(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.goodnews", module_obj);
    pyrt::xrefs::reg_fn("compose.goodnews", "_bare", py_func("_bare", |args: &[Py]| _bare(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_thing_words", py_func("_thing_words", |args: &[Py]| _thing_words(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "new_thing", py_func("new_thing", |args: &[Py]| new_thing(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "new_thing_replies", py_func("new_thing_replies", |args: &[Py]| new_thing_replies(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_the_person_word", py_func("_the_person_word", |args: &[Py]| _the_person_word(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "someone_elses_occasion", py_func("someone_elses_occasion", |args: &[Py]| someone_elses_occasion(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_an_occasion", py_func("reads_as_an_occasion", |args: &[Py]| reads_as_an_occasion(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_a_hand_over", py_func("_a_hand_over", |args: &[Py]| _a_hand_over(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_pass", py_func("reads_as_a_pass", |args: &[Py]| reads_as_a_pass(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_an_idiom_standing_alone", py_func("_an_idiom_standing_alone", |args: &[Py]| _an_idiom_standing_alone(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "asks_for_news", py_func("asks_for_news", |args: &[Py]| asks_for_news(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_a_money_clause", py_func("_a_money_clause", |args: &[Py]| _a_money_clause(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "ahead_of_now", py_func("ahead_of_now", |args: &[Py]| ahead_of_now(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_news_to_come", py_func("reads_as_news_to_come", |args: &[Py]| reads_as_news_to_come(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_the_number_counts_a_life", py_func("_the_number_counts_a_life", |args: &[Py]| _the_number_counts_a_life(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_prize_in_a_message", py_func("reads_as_a_prize_in_a_message", |args: &[Py]| reads_as_a_prize_in_a_message(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "a_year_behind", py_func("a_year_behind", |args: &[Py]| a_year_behind(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reply_for_a_year_ago", py_func("reply_for_a_year_ago", |args: &[Py]| reply_for_a_year_ago(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reply_for_a_win_from_a_past_year", py_func("reply_for_a_win_from_a_past_year", |args: &[Py]| reply_for_a_win_from_a_past_year(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "the_cheer_was_this_pool", py_func("the_cheer_was_this_pool", |args: &[Py]| the_cheer_was_this_pool(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_congratulation", py_func("reads_as_a_congratulation", |args: &[Py]| reads_as_a_congratulation(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_only_the_arrivals_figures", py_func("_only_the_arrivals_figures", |args: &[Py]| _only_the_arrivals_figures(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_the_detail_line", py_func("reads_as_the_detail_line", |args: &[Py]| reads_as_the_detail_line(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "the_reply_just_cheered", py_func("the_reply_just_cheered", |args: &[Py]| the_reply_just_cheered(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "a_detail_of_the_news", py_func("a_detail_of_the_news", |args: &[Py]| a_detail_of_the_news(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "a_birth_just_announced", py_func("a_birth_just_announced", |args: &[Py]| a_birth_just_announced(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_detail_of_the_news", py_func("reads_as_a_detail_of_the_news", |args: &[Py]| reads_as_a_detail_of_the_news(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_good_news", py_func("reads_as_good_news", |args: &[Py]| reads_as_good_news(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_the_speaker_is_the_quiet_one", py_func("_the_speaker_is_the_quiet_one", |args: &[Py]| _the_speaker_is_the_quiet_one(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_the_silence_is_the_whole_of_it", py_func("_the_silence_is_the_whole_of_it", |args: &[Py]| _the_silence_is_the_whole_of_it(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_win_a_cost_came_with", py_func("reads_as_a_win_a_cost_came_with", |args: &[Py]| reads_as_a_win_a_cost_came_with(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_win_nobody_noticed", py_func("reads_as_a_win_nobody_noticed", |args: &[Py]| reads_as_a_win_nobody_noticed(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_denied", py_func("_denied", |args: &[Py]| _denied(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_recovery", py_func("reads_as_recovery", |args: &[Py]| reads_as_recovery(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "worry_on_the_table", py_func("worry_on_the_table", |args: &[Py]| worry_on_the_table(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "the_tail_adds_nothing", py_func("the_tail_adds_nothing", |args: &[Py]| the_tail_adds_nothing(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "_a_bare_gone_is_not_the_news", py_func("_a_bare_gone_is_not_the_news", |args: &[Py]| _a_bare_gone_is_not_the_news(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "the_news_is_the_whole_of_it", py_func("the_news_is_the_whole_of_it", |args: &[Py]| the_news_is_the_whole_of_it(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_a_clear_result", py_func("reads_as_a_clear_result", |args: &[Py]| reads_as_a_clear_result(args)));
    pyrt::xrefs::reg_fn("compose.goodnews", "reads_as_settled", py_func("reads_as_settled", |args: &[Py]| reads_as_settled(args)));
    pyrt::xrefs::reg_access("compose.goodnews", "ABOUT_THEM", g_ABOUT_THEM, s_ABOUT_THEM);
    pyrt::xrefs::reg_access("compose.goodnews", "ANIMAL_THING", g_ANIMAL_THING, s_ANIMAL_THING);
    pyrt::xrefs::reg_access("compose.goodnews", "ARRIVED", g_ARRIVED, s_ARRIVED);
    pyrt::xrefs::reg_access("compose.goodnews", "ASK_OPEN", g_ASK_OPEN, s_ASK_OPEN);
    pyrt::xrefs::reg_access("compose.goodnews", "A_GAP_WITHOUT_A_NO", g_A_GAP_WITHOUT_A_NO, s_A_GAP_WITHOUT_A_NO);
    pyrt::xrefs::reg_access("compose.goodnews", "A_GOOD_ENDING", g_A_GOOD_ENDING, s_A_GOOD_ENDING);
    pyrt::xrefs::reg_access("compose.goodnews", "A_POINTER_IN_FRONT", g_A_POINTER_IN_FRONT, s_A_POINTER_IN_FRONT);
    pyrt::xrefs::reg_access("compose.goodnews", "A_SETBACK_AFTER_THE_WIN", g_A_SETBACK_AFTER_THE_WIN, s_A_SETBACK_AFTER_THE_WIN);
    pyrt::xrefs::reg_access("compose.goodnews", "A_TURN_IN_THE_TALE", g_A_TURN_IN_THE_TALE, s_A_TURN_IN_THE_TALE);
    pyrt::xrefs::reg_access("compose.goodnews", "A_YEAR_AFTER", g_A_YEAR_AFTER, s_A_YEAR_AFTER);
    pyrt::xrefs::reg_access("compose.goodnews", "BABY", g_BABY, s_BABY);
    pyrt::xrefs::reg_access("compose.goodnews", "BAD", g_BAD, s_BAD);
    pyrt::xrefs::reg_access("compose.goodnews", "BARE_GONE", g_BARE_GONE, s_BARE_GONE);
    pyrt::xrefs::reg_access("compose.goodnews", "BODY_GAVE_WAY", g_BODY_GAVE_WAY, s_BODY_GAVE_WAY);
    pyrt::xrefs::reg_access("compose.goodnews", "CLAUSE", g_CLAUSE, s_CLAUSE);
    pyrt::xrefs::reg_access("compose.goodnews", "CLAUSE_STOPS", g_CLAUSE_STOPS, s_CLAUSE_STOPS);
    pyrt::xrefs::reg_access("compose.goodnews", "CLEAR_RESULT", g_CLEAR_RESULT, s_CLEAR_RESULT);
    pyrt::xrefs::reg_access("compose.goodnews", "CLEAR_TAIL", g_CLEAR_TAIL, s_CLEAR_TAIL);
    pyrt::xrefs::reg_access("compose.goodnews", "CONDITIONAL", g_CONDITIONAL, s_CONDITIONAL);
    pyrt::xrefs::reg_access("compose.goodnews", "DENIED", g_DENIED, s_DENIED);
    pyrt::xrefs::reg_access("compose.goodnews", "FAMILY", g_FAMILY, s_FAMILY);
    pyrt::xrefs::reg_access("compose.goodnews", "FAULT", g_FAULT, s_FAULT);
    pyrt::xrefs::reg_access("compose.goodnews", "GOOD", g_GOOD, s_GOOD);
    pyrt::xrefs::reg_access("compose.goodnews", "HANDED_OVER", g_HANDED_OVER, s_HANDED_OVER);
    pyrt::xrefs::reg_access("compose.goodnews", "HAVE_GOT", g_HAVE_GOT, s_HAVE_GOT);
    pyrt::xrefs::reg_access("compose.goodnews", "HOME_THING", g_HOME_THING, s_HOME_THING);
    pyrt::xrefs::reg_access("compose.goodnews", "HOME_WORK", g_HOME_WORK, s_HOME_WORK);
    pyrt::xrefs::reg_access("compose.goodnews", "JUST_HAPPENED", g_JUST_HAPPENED, s_JUST_HAPPENED);
    pyrt::xrefs::reg_access("compose.goodnews", "LET_DOWN", g_LET_DOWN, s_LET_DOWN);
    pyrt::xrefs::reg_access("compose.goodnews", "MORE_OF_THE_NEWS", g_MORE_OF_THE_NEWS, s_MORE_OF_THE_NEWS);
    pyrt::xrefs::reg_access("compose.goodnews", "MORE_OF_THE_NEWS_ARRIVAL", g_MORE_OF_THE_NEWS_ARRIVAL, s_MORE_OF_THE_NEWS_ARRIVAL);
    pyrt::xrefs::reg_access("compose.goodnews", "MORE_OF_THE_NEWS_SCORE", g_MORE_OF_THE_NEWS_SCORE, s_MORE_OF_THE_NEWS_SCORE);
    pyrt::xrefs::reg_access("compose.goodnews", "MOVED_IN", g_MOVED_IN, s_MOVED_IN);
    pyrt::xrefs::reg_access("compose.goodnews", "NEARLY", g_NEARLY, s_NEARLY);
    pyrt::xrefs::reg_access("compose.goodnews", "NEGATED", g_NEGATED, s_NEGATED);
    pyrt::xrefs::reg_access("compose.goodnews", "NEWS_ASK", g_NEWS_ASK, s_NEWS_ASK);
    pyrt::xrefs::reg_access("compose.goodnews", "NEWS_FRAME", g_NEWS_FRAME, s_NEWS_FRAME);
    pyrt::xrefs::reg_access("compose.goodnews", "NEWS_IDIOM", g_NEWS_IDIOM, s_NEWS_IDIOM);
    pyrt::xrefs::reg_access("compose.goodnews", "NEWS_IDIOM_TAIL", g_NEWS_IDIOM_TAIL, s_NEWS_IDIOM_TAIL);
    pyrt::xrefs::reg_access("compose.goodnews", "NEWS_OF_IT", g_NEWS_OF_IT, s_NEWS_OF_IT);
    pyrt::xrefs::reg_access("compose.goodnews", "NEWS_WORD", g_NEWS_WORD, s_NEWS_WORD);
    pyrt::xrefs::reg_access("compose.goodnews", "NEW_ANIMAL", g_NEW_ANIMAL, s_NEW_ANIMAL);
    pyrt::xrefs::reg_access("compose.goodnews", "NEW_HOME", g_NEW_HOME, s_NEW_HOME);
    pyrt::xrefs::reg_access("compose.goodnews", "NEW_PLACE", g_NEW_PLACE, s_NEW_PLACE);
    pyrt::xrefs::reg_access("compose.goodnews", "NEW_THING", g_NEW_THING, s_NEW_THING);
    pyrt::xrefs::reg_access("compose.goodnews", "NOBODY", g_NOBODY, s_NOBODY);
    pyrt::xrefs::reg_access("compose.goodnews", "NOBODY_SAID_ANYTHING", g_NOBODY_SAID_ANYTHING, s_NOBODY_SAID_ANYTHING);
    pyrt::xrefs::reg_access("compose.goodnews", "NOT_AN_ARRIVAL", g_NOT_AN_ARRIVAL, s_NOT_AN_ARRIVAL);
    pyrt::xrefs::reg_access("compose.goodnews", "NOT_AN_OCCASION", g_NOT_AN_OCCASION, s_NOT_AN_OCCASION);
    pyrt::xrefs::reg_access("compose.goodnews", "NOT_CLEAR", g_NOT_CLEAR, s_NOT_CLEAR);
    pyrt::xrefs::reg_access("compose.goodnews", "NOT_NEWS", g_NOT_NEWS, s_NOT_NEWS);
    pyrt::xrefs::reg_access("compose.goodnews", "NOT_QUITE", g_NOT_QUITE, s_NOT_QUITE);
    pyrt::xrefs::reg_access("compose.goodnews", "OBJECT_THING", g_OBJECT_THING, s_OBJECT_THING);
    pyrt::xrefs::reg_access("compose.goodnews", "OCCASION", g_OCCASION, s_OCCASION);
    pyrt::xrefs::reg_access("compose.goodnews", "ORDINAL_DAY", g_ORDINAL_DAY, s_ORDINAL_DAY);
    pyrt::xrefs::reg_access("compose.goodnews", "PASSED", g_PASSED, s_PASSED);
    pyrt::xrefs::reg_access("compose.goodnews", "PASS_PARTICLE", g_PASS_PARTICLE, s_PASS_PARTICLE);
    pyrt::xrefs::reg_access("compose.goodnews", "PASS_TAKEN", g_PASS_TAKEN, s_PASS_TAKEN);
    pyrt::xrefs::reg_access("compose.goodnews", "PRAISED", g_PRAISED, s_PRAISED);
    pyrt::xrefs::reg_access("compose.goodnews", "RECOVERY", g_RECOVERY, s_RECOVERY);
    pyrt::xrefs::reg_access("compose.goodnews", "RELATION_BEFORE_DAY", g_RELATION_BEFORE_DAY, s_RELATION_BEFORE_DAY);
    pyrt::xrefs::reg_access("compose.goodnews", "SAYS_IT_IN_THE_PRESENT", g_SAYS_IT_IN_THE_PRESENT, s_SAYS_IT_IN_THE_PRESENT);
    pyrt::xrefs::reg_access("compose.goodnews", "SETTLED", g_SETTLED, s_SETTLED);
    pyrt::xrefs::reg_access("compose.goodnews", "SETTLED_TAIL", g_SETTLED_TAIL, s_SETTLED_TAIL);
    pyrt::xrefs::reg_access("compose.goodnews", "SILENCE_WORDS", g_SILENCE_WORDS, s_SILENCE_WORDS);
    pyrt::xrefs::reg_access("compose.goodnews", "SOMEONE_LEFT", g_SOMEONE_LEFT, s_SOMEONE_LEFT);
    pyrt::xrefs::reg_access("compose.goodnews", "SPEAKER", g_SPEAKER, s_SPEAKER);
    pyrt::xrefs::reg_access("compose.goodnews", "TAKES_A_CLAUSE", g_TAKES_A_CLAUSE, s_TAKES_A_CLAUSE);
    pyrt::xrefs::reg_access("compose.goodnews", "THING_STOPS", g_THING_STOPS, s_THING_STOPS);
    pyrt::xrefs::reg_access("compose.goodnews", "WHAT_HAPPENED_ASK", g_WHAT_HAPPENED_ASK, s_WHAT_HAPPENED_ASK);
    pyrt::xrefs::reg_access("compose.goodnews", "WIN", g_WIN, s_WIN);
    pyrt::xrefs::reg_access("compose.goodnews", "WORLD_NEWS", g_WORLD_NEWS, s_WORLD_NEWS);
    pyrt::xrefs::reg_access("compose.goodnews", "WORRY_WORDS", g_WORRY_WORDS, s_WORRY_WORDS);
    pyrt::xrefs::reg_access("compose.goodnews", "YEARS_BACK_LINES", g_YEARS_BACK_LINES, s_YEARS_BACK_LINES);
    pyrt::xrefs::reg_access("compose.goodnews", "_ABOUT_THE_WIN", g__ABOUT_THE_WIN, s__ABOUT_THE_WIN);
    pyrt::xrefs::reg_access("compose.goodnews", "_AHEAD", g__AHEAD, s__AHEAD);
    pyrt::xrefs::reg_access("compose.goodnews", "_AHEAD_SPAN", g__AHEAD_SPAN, s__AHEAD_SPAN);
    pyrt::xrefs::reg_access("compose.goodnews", "_ALREADY", g__ALREADY, s__ALREADY);
    pyrt::xrefs::reg_access("compose.goodnews", "_AN_AGE_SAID", g__AN_AGE_SAID, s__AN_AGE_SAID);
    pyrt::xrefs::reg_access("compose.goodnews", "_AN_ATTEMPT", g__AN_ATTEMPT, s__AN_ATTEMPT);
    pyrt::xrefs::reg_access("compose.goodnews", "_AN_HOUR", g__AN_HOUR, s__AN_HOUR);
    pyrt::xrefs::reg_access("compose.goodnews", "_AROUND_THE_FIGURES", g__AROUND_THE_FIGURES, s__AROUND_THE_FIGURES);
    pyrt::xrefs::reg_access("compose.goodnews", "_ASKS_FOR_SOMETHING_BACK", g__ASKS_FOR_SOMETHING_BACK, s__ASKS_FOR_SOMETHING_BACK);
    pyrt::xrefs::reg_access("compose.goodnews", "_ASKS_OR_THANKS", g__ASKS_OR_THANKS, s__ASKS_OR_THANKS);
    pyrt::xrefs::reg_access("compose.goodnews", "_AWARD", g__AWARD, s__AWARD);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_CHEER", g__A_CHEER, s__A_CHEER);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_CLASS", g__A_CLASS, s__A_CLASS);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_CLOCK", g__A_CLOCK, s__A_CLOCK);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_CLOCK_MINUTES", g__A_CLOCK_MINUTES, s__A_CLOCK_MINUTES);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_COUNT", g__A_COUNT, s__A_COUNT);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_COURSE", g__A_COURSE, s__A_COURSE);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_DATE", g__A_DATE, s__A_DATE);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_FIGURE_IN_THE_LINE", g__A_FIGURE_IN_THE_LINE, s__A_FIGURE_IN_THE_LINE);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_MEDIUM", g__A_MEDIUM, s__A_MEDIUM);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_MESSAGE_THAT_CLAIMS_A_WIN", g__A_MESSAGE_THAT_CLAIMS_A_WIN, s__A_MESSAGE_THAT_CLAIMS_A_WIN);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_NUMBER", g__A_NUMBER, s__A_NUMBER);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_PERSON", g__A_PERSON, s__A_PERSON);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_RELATIVE_TOLD_ME", g__A_RELATIVE_TOLD_ME, s__A_RELATIVE_TOLD_ME);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_SCORE", g__A_SCORE, s__A_SCORE);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_WEIGHT", g__A_WEIGHT, s__A_WEIGHT);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_WIN", g__A_WIN, s__A_WIN);
    pyrt::xrefs::reg_access("compose.goodnews", "_A_WIN_THEN_A_MESSAGE", g__A_WIN_THEN_A_MESSAGE, s__A_WIN_THEN_A_MESSAGE);
    pyrt::xrefs::reg_access("compose.goodnews", "_BARELY_ASKED", g__BARELY_ASKED, s__BARELY_ASKED);
    pyrt::xrefs::reg_access("compose.goodnews", "_CLAUSE_BREAK", g__CLAUSE_BREAK, s__CLAUSE_BREAK);
    pyrt::xrefs::reg_access("compose.goodnews", "_COMING_UP", g__COMING_UP, s__COMING_UP);
    pyrt::xrefs::reg_access("compose.goodnews", "_COUNTED_UP", g__COUNTED_UP, s__COUNTED_UP);
    pyrt::xrefs::reg_access("compose.goodnews", "_DONE_ALREADY", g__DONE_ALREADY, s__DONE_ALREADY);
    pyrt::xrefs::reg_access("compose.goodnews", "_DREAD", g__DREAD, s__DREAD);
    pyrt::xrefs::reg_access("compose.goodnews", "_DUE", g__DUE, s__DUE);
    pyrt::xrefs::reg_access("compose.goodnews", "_ENTERED_IT", g__ENTERED_IT, s__ENTERED_IT);
    pyrt::xrefs::reg_access("compose.goodnews", "_I_PRAISED", g__I_PRAISED, s__I_PRAISED);
    pyrt::xrefs::reg_access("compose.goodnews", "_MONEY", g__MONEY, s__MONEY);
    pyrt::xrefs::reg_access("compose.goodnews", "_MONTH", g__MONTH, s__MONTH);
    pyrt::xrefs::reg_access("compose.goodnews", "_MORE_OF_THE_NEWS_MARKS", g__MORE_OF_THE_NEWS_MARKS, s__MORE_OF_THE_NEWS_MARKS);
    pyrt::xrefs::reg_access("compose.goodnews", "_MY_OWN", g__MY_OWN, s__MY_OWN);
    pyrt::xrefs::reg_access("compose.goodnews", "_NEVER_ENTERED", g__NEVER_ENTERED, s__NEVER_ENTERED);
    pyrt::xrefs::reg_access("compose.goodnews", "_NOT_HAPPENING", g__NOT_HAPPENING, s__NOT_HAPPENING);
    pyrt::xrefs::reg_access("compose.goodnews", "_OR_A_SCORE", g__OR_A_SCORE, s__OR_A_SCORE);
    pyrt::xrefs::reg_access("compose.goodnews", "_PERSON_DUE", g__PERSON_DUE, s__PERSON_DUE);
    pyrt::xrefs::reg_access("compose.goodnews", "_PRAISING", g__PRAISING, s__PRAISING);
    pyrt::xrefs::reg_access("compose.goodnews", "_SAID", g__SAID, s__SAID);
    pyrt::xrefs::reg_access("compose.goodnews", "_SOON_ITSELF", g__SOON_ITSELF, s__SOON_ITSELF);
    pyrt::xrefs::reg_access("compose.goodnews", "_STANDIN", g__STANDIN, s__STANDIN);
    pyrt::xrefs::reg_access("compose.goodnews", "_TOLD", g__TOLD, s__TOLD);
    pyrt::xrefs::reg_access("compose.goodnews", "_WEEKDAY", g__WEEKDAY, s__WEEKDAY);
    pyrt::xrefs::reg_access("compose.goodnews", "_WHEN_DUE", g__WHEN_DUE, s__WHEN_DUE);
    pyrt::xrefs::reg_access("compose.goodnews", "_WHO_DUE", g__WHO_DUE, s__WHO_DUE);
    pyrt::xrefs::reg_access("compose.goodnews", "line", g_line, s_line);
    pyrt::xrefs::reg_access("compose.goodnews", "low", g_low, s_low);
}

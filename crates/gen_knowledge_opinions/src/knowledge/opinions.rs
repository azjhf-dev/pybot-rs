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


thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_MARKERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JOB_ON_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEW_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CANT_DECIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAR_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_EQUIPMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_ON_A_DIMENSION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_VERDICT_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_WINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CAT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_IN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COURSE_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECIDE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DECISION_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISPOSAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_ON_A_DIMENSION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOG_WINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRUMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPLAINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GUINEA_PIG_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAMSTER_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSTRUMENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSTRUMENT_GENERIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSTRUMENT_LOOKBACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INSTRUMENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MADE_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOVE_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_BUYING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_INSTRUMENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_ALL_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_BUYING_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_OPINION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALIFIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RABBIT_TRADE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STUCK_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOO_LATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRADES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAIN_COMPARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TRAIN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TWO_WAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANOTHER_CATS_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_JOB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MANNERS_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SHARED_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TABLE_BILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BUYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GADGET_TIPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NORMAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NORMAL_SUBJECTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NORMAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TWO_OF_THEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHAT_TO_BUY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__rule_of_thumb: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_asks_where_a_shop_answers: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_the_book_is_a_reservation: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.opinions: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_MARKERS() -> Py { ensure_init(); G_ASK_MARKERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_MARKERS(v: Py) { ensure_init(); G_ASK_MARKERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JOB_ON_IT() -> Py { ensure_init(); G_A_JOB_ON_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JOB_ON_IT(v: Py) { ensure_init(); G_A_JOB_ON_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEW_ONE() -> Py { ensure_init(); G_A_NEW_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEW_ONE(v: Py) { ensure_init(); G_A_NEW_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_CANT_DECIDE() -> Py { ensure_init(); G_CANT_DECIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CANT_DECIDE(v: Py) { ensure_init(); G_CANT_DECIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAR_TRADE() -> Py { ensure_init(); G_CAR_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAR_TRADE(v: Py) { ensure_init(); G_CAR_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_EQUIPMENT() -> Py { ensure_init(); G_CAT_EQUIPMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_EQUIPMENT(v: Py) { ensure_init(); G_CAT_EQUIPMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_ON_A_DIMENSION() -> Py { ensure_init(); G_CAT_ON_A_DIMENSION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_ON_A_DIMENSION(v: Py) { ensure_init(); G_CAT_ON_A_DIMENSION.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_TRADE() -> Py { ensure_init(); G_CAT_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_TRADE(v: Py) { ensure_init(); G_CAT_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_VERDICT_SHAPES() -> Py { ensure_init(); G_CAT_VERDICT_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_VERDICT_SHAPES(v: Py) { ensure_init(); G_CAT_VERDICT_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_WINS() -> Py { ensure_init(); G_CAT_WINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_WINS(v: Py) { ensure_init(); G_CAT_WINS.with(|c| *c.borrow_mut() = v); }
pub fn g_CAT_WORDS() -> Py { ensure_init(); G_CAT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CAT_WORDS(v: Py) { ensure_init(); G_CAT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE() -> Py { ensure_init(); G_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE(v: Py) { ensure_init(); G_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_ASKS() -> Py { ensure_init(); G_CHOICE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_ASKS(v: Py) { ensure_init(); G_CHOICE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_ASK_WORDS() -> Py { ensure_init(); G_CHOICE_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_ASK_WORDS(v: Py) { ensure_init(); G_CHOICE_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_IN_WORDS() -> Py { ensure_init(); G_CHOICE_IN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_IN_WORDS(v: Py) { ensure_init(); G_CHOICE_IN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COURSE_TRADE() -> Py { ensure_init(); G_COURSE_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COURSE_TRADE(v: Py) { ensure_init(); G_COURSE_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_DECIDE_WORDS() -> Py { ensure_init(); G_DECIDE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECIDE_WORDS(v: Py) { ensure_init(); G_DECIDE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DECISION_SUBJECTS() -> Py { ensure_init(); G_DECISION_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DECISION_SUBJECTS(v: Py) { ensure_init(); G_DECISION_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISPOSAL() -> Py { ensure_init(); G_DISPOSAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISPOSAL(v: Py) { ensure_init(); G_DISPOSAL.with(|c| *c.borrow_mut() = v); }
pub fn g_DOG_ON_A_DIMENSION() -> Py { ensure_init(); G_DOG_ON_A_DIMENSION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOG_ON_A_DIMENSION(v: Py) { ensure_init(); G_DOG_ON_A_DIMENSION.with(|c| *c.borrow_mut() = v); }
pub fn g_DOG_TRADE() -> Py { ensure_init(); G_DOG_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOG_TRADE(v: Py) { ensure_init(); G_DOG_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_DOG_WINS() -> Py { ensure_init(); G_DOG_WINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOG_WINS(v: Py) { ensure_init(); G_DOG_WINS.with(|c| *c.borrow_mut() = v); }
pub fn g_DRUMS() -> Py { ensure_init(); G_DRUMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRUMS(v: Py) { ensure_init(); G_DRUMS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPLAINS() -> Py { ensure_init(); G_EXPLAINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPLAINS(v: Py) { ensure_init(); G_EXPLAINS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAULT() -> Py { ensure_init(); G_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAULT(v: Py) { ensure_init(); G_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON() -> Py { ensure_init(); G_FIRST_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON(v: Py) { ensure_init(); G_FIRST_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_GUINEA_PIG_TRADE() -> Py { ensure_init(); G_GUINEA_PIG_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GUINEA_PIG_TRADE(v: Py) { ensure_init(); G_GUINEA_PIG_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_HAMSTER_TRADE() -> Py { ensure_init(); G_HAMSTER_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAMSTER_TRADE(v: Py) { ensure_init(); G_HAMSTER_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH_ASK() -> Py { ensure_init(); G_HEALTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH_ASK(v: Py) { ensure_init(); G_HEALTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_INSTRUMENTS() -> Py { ensure_init(); G_INSTRUMENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSTRUMENTS(v: Py) { ensure_init(); G_INSTRUMENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_INSTRUMENT_GENERIC() -> Py { ensure_init(); G_INSTRUMENT_GENERIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSTRUMENT_GENERIC(v: Py) { ensure_init(); G_INSTRUMENT_GENERIC.with(|c| *c.borrow_mut() = v); }
pub fn g_INSTRUMENT_LOOKBACK() -> Py { ensure_init(); G_INSTRUMENT_LOOKBACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSTRUMENT_LOOKBACK(v: Py) { ensure_init(); G_INSTRUMENT_LOOKBACK.with(|c| *c.borrow_mut() = v); }
pub fn g_INSTRUMENT_WORDS() -> Py { ensure_init(); G_INSTRUMENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INSTRUMENT_WORDS(v: Py) { ensure_init(); G_INSTRUMENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_INTENT() -> Py { ensure_init(); G_INTENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTENT(v: Py) { ensure_init(); G_INTENT.with(|c| *c.borrow_mut() = v); }
pub fn g_MADE_UP() -> Py { ensure_init(); G_MADE_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MADE_UP(v: Py) { ensure_init(); G_MADE_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_MOVE_TRADE() -> Py { ensure_init(); G_MOVE_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOVE_TRADE(v: Py) { ensure_init(); G_MOVE_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_BUYING_ASK() -> Py { ensure_init(); G_NOT_A_BUYING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BUYING_ASK(v: Py) { ensure_init(); G_NOT_A_BUYING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_INSTRUMENTS() -> Py { ensure_init(); G_OTHER_INSTRUMENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_INSTRUMENTS(v: Py) { ensure_init(); G_OTHER_INSTRUMENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_ALL_DAY() -> Py { ensure_init(); G_OUT_ALL_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_ALL_DAY(v: Py) { ensure_init(); G_OUT_ALL_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_BUYING_ANSWER() -> Py { ensure_init(); G_OWN_BUYING_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_BUYING_ANSWER(v: Py) { ensure_init(); G_OWN_BUYING_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_OPINION() -> Py { ensure_init(); G_OWN_OPINION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_OPINION(v: Py) { ensure_init(); G_OWN_OPINION.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALIFIED() -> Py { ensure_init(); G_QUALIFIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALIFIED(v: Py) { ensure_init(); G_QUALIFIED.with(|c| *c.borrow_mut() = v); }
pub fn g_RABBIT_TRADE() -> Py { ensure_init(); G_RABBIT_TRADE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RABBIT_TRADE(v: Py) { ensure_init(); G_RABBIT_TRADE.with(|c| *c.borrow_mut() = v); }
pub fn g_STUCK_SHAPES() -> Py { ensure_init(); G_STUCK_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STUCK_SHAPES(v: Py) { ensure_init(); G_STUCK_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTE() -> Py { ensure_init(); G_TASTE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTE(v: Py) { ensure_init(); G_TASTE.with(|c| *c.borrow_mut() = v); }
pub fn g_TOO_LATE() -> Py { ensure_init(); G_TOO_LATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOO_LATE(v: Py) { ensure_init(); G_TOO_LATE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRADES() -> Py { ensure_init(); G_TRADES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRADES(v: Py) { ensure_init(); G_TRADES.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAIN_COMPARE() -> Py { ensure_init(); G_TRAIN_COMPARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAIN_COMPARE(v: Py) { ensure_init(); G_TRAIN_COMPARE.with(|c| *c.borrow_mut() = v); }
pub fn g_TRAIN_WORDS() -> Py { ensure_init(); G_TRAIN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TRAIN_WORDS(v: Py) { ensure_init(); G_TRAIN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TWO_WAYS() -> Py { ensure_init(); G_TWO_WAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TWO_WAYS(v: Py) { ensure_init(); G_TWO_WAYS.with(|c| *c.borrow_mut() = v); }
pub fn g__ANOTHER_CATS_QUESTION() -> Py { ensure_init(); G__ANOTHER_CATS_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANOTHER_CATS_QUESTION(v: Py) { ensure_init(); G__ANOTHER_CATS_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_JOB() -> Py { ensure_init(); G__A_JOB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_JOB(v: Py) { ensure_init(); G__A_JOB.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MANNERS_WORD() -> Py { ensure_init(); G__A_MANNERS_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MANNERS_WORD(v: Py) { ensure_init(); G__A_MANNERS_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PART() -> Py { ensure_init(); G__A_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PART(v: Py) { ensure_init(); G__A_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SHARED_HOME() -> Py { ensure_init(); G__A_SHARED_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SHARED_HOME(v: Py) { ensure_init(); G__A_SHARED_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TABLE_BILL() -> Py { ensure_init(); G__A_TABLE_BILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TABLE_BILL(v: Py) { ensure_init(); G__A_TABLE_BILL.with(|c| *c.borrow_mut() = v); }
pub fn g__BUYING() -> Py { ensure_init(); G__BUYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BUYING(v: Py) { ensure_init(); G__BUYING.with(|c| *c.borrow_mut() = v); }
pub fn g__GADGET_TIPS() -> Py { ensure_init(); G__GADGET_TIPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GADGET_TIPS(v: Py) { ensure_init(); G__GADGET_TIPS.with(|c| *c.borrow_mut() = v); }
pub fn g__NORMAL_ASK() -> Py { ensure_init(); G__NORMAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NORMAL_ASK(v: Py) { ensure_init(); G__NORMAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__NORMAL_SUBJECTS() -> Py { ensure_init(); G__NORMAL_SUBJECTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NORMAL_SUBJECTS(v: Py) { ensure_init(); G__NORMAL_SUBJECTS.with(|c| *c.borrow_mut() = v); }
pub fn g__NORMAL_WORDS() -> Py { ensure_init(); G__NORMAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NORMAL_WORDS(v: Py) { ensure_init(); G__NORMAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_VERDICT() -> Py { ensure_init(); G__THE_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_VERDICT(v: Py) { ensure_init(); G__THE_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g__TWO_OF_THEM() -> Py { ensure_init(); G__TWO_OF_THEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TWO_OF_THEM(v: Py) { ensure_init(); G__TWO_OF_THEM.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT_ASK() -> Py { ensure_init(); G__VERDICT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT_ASK(v: Py) { ensure_init(); G__VERDICT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__WHAT_TO_BUY() -> Py { ensure_init(); G__WHAT_TO_BUY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHAT_TO_BUY(v: Py) { ensure_init(); G__WHAT_TO_BUY.with(|c| *c.borrow_mut() = v); }
pub fn g__rule_of_thumb() -> Py { ensure_init(); G__rule_of_thumb.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__rule_of_thumb(v: Py) { ensure_init(); G__rule_of_thumb.with(|c| *c.borrow_mut() = v); }
pub fn g_asks_where_a_shop_answers() -> Py { ensure_init(); G_asks_where_a_shop_answers.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_asks_where_a_shop_answers(v: Py) { ensure_init(); G_asks_where_a_shop_answers.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_the_book_is_a_reservation() -> Py { ensure_init(); G_the_book_is_a_reservation.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_the_book_is_a_reservation(v: Py) { ensure_init(); G_the_book_is_a_reservation.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Opinions and small choices: the bot's own taste, given briefly and with a reason.\n\nPeople ask the bot what it thinks more often than they ask it for a fact: \"cats or dogs?\",\n\"what do you think is the best season\", \"is it worth learning an instrument at 45\". STYLE.md\nallows the bot to have tastes, so this domain keeps them as data, one Entry per subject, and\nanswers only when the line really is a question or a choice: a statement that merely mentions\nthe subject (\"my favourite season is autumn\", \"we split the bill\") is left to the modules that\ncan help with it.\n\nTwo shapes of entry:\n* a plain subject (\"season\", \"pizza topping\") answers an opinion question about it;\n* `_either` pairs two everyday things and answers only when the line is a choice between them,\n  in either order (\"tea or coffee\", \"which is better, paper or an e-reader\").\n");
    s_the_book_is_a_reservation(py_func("the_book_is_a_reservation", |args: &[Py]| pyrt::xrefs::xcall("knowledge.entertainment", "the_book_is_a_reservation", args)));
    s_asks_where_a_shop_answers(py_func("asks_where_a_shop_answers", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "asks_where_a_shop_answers", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_ANSWERS(py_tuple(vec![py_str("opinions")]));
    s_ASK_MARKERS(call_builtin("frozenset", &[mcall(&py_str("\n    you your better rather which opinion pick recommend worth should would\n    ok okay rude fine wrong right good bad\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_OPINION(stdlib_call("re", "compile", &[py_str("\\b(i think|i thought|i prefer|i'd prefer|id prefer|i believe|i reckon|i feel|i'd say|id say|i say|in my opinion|personally|for me|to me)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_PERSON(call_builtin("frozenset", &[mcall(&py_str("i i'm im me my mine we we're were our ours us myself"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data1() -> Py { py_tuple(vec![py_str("want to"), py_str("wanted to"), py_str("wants to"), py_str("thinking of"), py_str("thinking about"), py_str("planning to"), py_str("plan to"), py_str("tempted to"), py_str("considering"), py_str("keen to"), py_str("hoping to"), py_str("too late"), py_str("at my age"), py_str("old age")]) }
    s_INTENT(__data1());
    s_EXPLAINS(stdlib_call("re", "compile", &[py_str("^\\s*(why|how come|what makes|what causes|explain|what's the reason)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("filter"), py_str("filters"), py_str("seal"), py_str("gasket"), py_str("drawer"), py_str("drum"), py_str("hose"), py_str("latch"), py_str("pump"), py_str("door"), py_str("lid"), py_str("tray"), py_str("paddle"), py_str("blade"), py_str("element"), py_str("shelf"), py_str("shelves"), py_str("basket"), py_str("spray arm"), py_str("cutting blade"), py_str("jug"), py_str("carafe"), py_str("pump filter")]) }
    s__A_PART(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("gunk"), py_str("blocked"), py_str("blocking"), py_str("smell"), py_str("smells"), py_str("mould"), py_str("mold"), py_str("broke"), py_str("broken"), py_str("torn"), py_str("split"), py_str("cracked"), py_str("leaking"), py_str("jam"), py_str("jammed"), py_str("will not"), py_str("wont"), py_str("won't"), py_str("fix"), py_str("replace"), py_str("stuck"), py_str("clogged"), py_str("descale"), py_str("scrub"), py_str("empty it"), py_str("get it out"), py_str("get the gunk out")]) }
    s__A_JOB(__data3());
    s__BUYING(py_tuple(vec![py_str("worth it"), py_str("worth buying"), py_str("should i buy"), py_str("is it worth"), py_str("buy a"), py_str("buying a")]));
    s__GADGET_TIPS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("slow cooker"), py_str("slow cookers"), py_str("crock pot"), py_str("crockpot")]), py_str("Brown the meat in a pan first and leave the lid on, because every lift of the lid is twenty minutes of heat gone: six to eight hours on low does what four on high cannot, and shin, shoulder and chicken thighs come out better than anything you would roast.")]), py_tuple(vec![py_tuple(vec![py_str("air fryer"), py_str("air fryers")]), py_str("Fill the basket no more than two-thirds and shake it halfway, because a crowded basket steams instead of crisping. Treat it as a fan oven twenty degrees cooler and a fifth less time: chips at 200C for eighteen minutes with one shake, chicken thighs at 190C for twenty-five.")]), py_tuple(vec![py_tuple(vec![py_str("bread maker"), py_str("bread makers"), py_str("bread machine"), py_str("bread machines")]), py_str("Use it for the dough and bake the loaf in the oven, because the machine's own bake gives a squat loaf with a pale top and a hard crust. Three-quarters of a teaspoon of yeast for a 500g loaf is plenty when it proves slowly.")]), py_tuple(vec![py_tuple(vec![py_str("blender"), py_str("blenders"), py_str("smoothie maker"), py_str("smoothie makers"), py_str("nutribullet")]), py_str("Liquid in first, then the soft things, then anything frozen, and never more than half full of solids: start on the lowest speed and pulse, because full speed from cold throws the lot into the lid.")]), py_tuple(vec![py_tuple(vec![py_str("food processor"), py_str("food processors"), py_str("magimix")]), py_str("Pulse rather than hold, because pastry, mince and pesto are a series of short bursts and a long run turns them to paste. Add anything wet through the tube while it is running and it mixes in evenly instead of pooling at the bottom.")]), py_tuple(vec![py_tuple(vec![py_str("stand mixer"), py_str("stand mixers"), py_str("kitchenaid"), py_str("kenwood")]), py_str("Paddle for cakes, dough hook on the slowest speed for bread, whisk only for eggs and cream. Stop at the time in the recipe rather than watching the bowl, because an over-mixed cake comes out flat and tough.")]), py_tuple(vec![py_tuple(vec![py_str("pressure cooker"), py_str("pressure cookers"), py_str("instant pot")]), py_str("Never fill past two-thirds, and let meat drop from pressure on its own for ten minutes, because a quick release pulls the steam out of the middle and leaves it stringy. Soaked beans and stewing steak are where it beats the slow cooker on a weekday.")]), py_tuple(vec![py_tuple(vec![py_str("juicer"), py_str("juicers")]), py_str("Feed soft fruit in with the hard, and wrap leaves in a lettuce leaf so they go down instead of clogging the mesh. Rinse the basket straight away, before the pulp dries onto it.")]), py_tuple(vec![py_tuple(vec![py_str("coffee machine"), py_str("coffee machines"), py_str("bean to cup"), py_str("espresso machine"), py_str("sage barista")]), py_str("Weigh the beans rather than scooping - about 18g in for a double - and descale every couple of months, because scale is what drops the temperature and makes every cup taste flat.")]), py_tuple(vec![py_tuple(vec![py_str("sandwich toaster"), py_str("sandwich toasters"), py_str("toastie maker"), py_str("panini press")]), py_str("Butter the outside of the bread, not the inside, and keep the filling an inch from the edges: cheese in the hinge is what gets these thrown away.")]), py_tuple(vec![py_tuple(vec![py_str("rice cooker"), py_str("rice cookers")]), py_str("Rinse the rice until the water runs clear, then one part rice to one and a half parts water. Leave it on warm for an hour at most, because past that the bottom sets and it starts to smell.")]), py_tuple(vec![py_tuple(vec![py_str("ice cream maker"), py_str("ice cream makers")]), py_str("Chill the bowl overnight and the mix in the fridge as well, because a warm mix churns to slush. Take it out while it is still soft, since it sets hard once it is in the freezer.")])]));
    s__VERDICT_ASK(py_tuple(vec![py_str("think"), py_str("reckon"), py_str("opinion"), py_str("worth"), py_str("any good"), py_str("recommend"), py_str("like my"), py_str("regret"), py_str("good buy"), py_str("waste"), py_str("rate my"), py_str("should i keep")]));
    s__THE_VERDICT(py_str("The honest test is whether it earns its counter space: three uses a week and it was a good buy, once a month and it lives in a cupboard. Give it a month of proper use before you decide, and keep one recipe you like so it is not a project every time."));
    s_CHOICE(call_builtin("frozenset", &[mcall(&py_str("or vs versus better prefer rather which either easier harder cheaper simpler quicker cleaner quieter calmer friendlier healthier more work less work more expensive less expensive"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data4() -> Py { py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("buy"), py_str("buying"), py_str("get"), py_str("getting"), py_str("need"), py_str("recommend"), py_str("any good"), py_str("good"), py_str("think"), py_str("opinion"), py_str("reckon"), py_str("rate"), py_str("prefer"), py_str("better")]) }
    s_TASTE(__data4());
    s_CHOICE_ASKS(py_tuple(vec![py_str("should i"), py_str("shall i"), py_str("should we"), py_str("is it worth"), py_str("would i be better")]));
    s_CHOICE_IN_WORDS(stdlib_call("re", "compile", &[py_str("(?:^|[^a-z])(?:or|vs|versus|rather than|instead of)(?:[^a-z]|$)")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHOICE_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("should shall would which better rather worth recommend pick choose prefer"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_BUYING_ANSWER(py_tuple(vec![py_str("new washing machine"), py_str("buying a washing machine"), py_str("washing machines"), py_str("new dishwasher"), py_str("buying a dishwasher")]));
    s_NOT_A_BUYING_ASK(py_tuple(vec![py_str("which"), py_str("my new"), py_str("my old"), py_str("i bought"), py_str("i've bought"), py_str("i got"), py_str("i've got"), py_str("we bought"), py_str("we got"), py_str("i own"), py_str("we own")]));
    fn __data5() -> Py { py_tuple(vec![py_str("get rid of"), py_str("getting rid of"), py_str("got rid of"), py_str("throw away"), py_str("throwing away"), py_str("throw it out"), py_str("throwing it out"), py_str("throw out"), py_str("tip"), py_str("the tip"), py_str("recycling centre"), py_str("dump"), py_str("dumped"), py_str("dumping"), py_str("dispose"), py_str("disposal"), py_str("scrap"), py_str("scrapping"), py_str("scrapped"), py_str("recycle"), py_str("recycling"), py_str("bulky waste"), py_str("bulky collection"), py_str("collection"), py_str("collect it"), py_str("take it away"), py_str("taking it away"), py_str("unwanted"), py_str("no longer want"), py_str("dont want it"), py_str("don't want it"), py_str("getting a new one"), py_str("replace it")]) }
    s_DISPOSAL(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("how do i get"), py_str("how to get"), py_str("how do i clean"), py_str("how to clean"), py_str("how do i remove"), py_str("how to remove"), py_str("how do i clear"), py_str("how to clear"), py_str("how do i stop the"), py_str("hair out of"), py_str("lint out of"), py_str("fluff out of"), py_str("clean out"), py_str("cleaned out"), py_str("unblock"), py_str("clear the"), py_str("get the smell out"), py_str("get rid of the smell")]) }
    s_A_JOB_ON_IT(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("plumber"), py_str("electrician"), py_str("gas safe"), py_str("have a go"), py_str("have a bash"), py_str("pay someone"), py_str("call someone"), py_str("call a man"), py_str("get someone in"), py_str("tradesman"), py_str("handyman"), py_str("diy"), py_str("myself"), py_str("or should i have a"), py_str("worth paying"), py_str("paying someone")]) }
    s_TRADES(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("wont"), py_str("won't"), py_str("isnt"), py_str("isn't"), py_str("doesnt"), py_str("doesn't"), py_str("cant"), py_str("can't"), py_str("cannot"), py_str("drain"), py_str("draining"), py_str("leak"), py_str("leaking"), py_str("leaked"), py_str("broken"), py_str("broke"), py_str("fault"), py_str("faulty"), py_str("error"), py_str("flashing"), py_str("not working"), py_str("stopped"), py_str("stop"), py_str("flood"), py_str("flooded"), py_str("water everywhere"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("repair"), py_str("repairs"), py_str("repairing"), py_str("fix"), py_str("fixing"), py_str("stuck"), py_str("noise"), py_str("noisy"), py_str("loud"), py_str("rattling"), py_str("grinding"), py_str("dead"), py_str("warranty"), py_str("guarantee"), py_str("not heating"), py_str("isnt heating"), py_str("wont heat"), py_str("no heat"), py_str("heat up"), py_str("not spinning"), py_str("wont spin"), py_str("not turning"), py_str("wont turn"), py_str("not drying"), py_str("wont dry"), py_str("takes ages"), py_str("still wet"), py_str("still damp"), py_str("engineer"), py_str("engineers"), py_str("call someone out")]) }
    s_FAULT(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("too late"), py_str("too old"), py_str("am i too old"), py_str("at my age"), py_str("my age"), py_str("old age"), py_str("start at"), py_str("adult"), py_str("as an adult"), py_str("grown up"), py_str("40"), py_str("45"), py_str("50"), py_str("60"), py_str("70")]) }
    s_TOO_LATE(__data9());
    s_DRUMS(py_tuple(vec![py_str("drum"), py_str("drums"), py_str("drum kit"), py_str("drumkit"), py_str("drumming"), py_str("percussion"), py_str("practice pad")]));
    fn __data10() -> Py { py_tuple(vec![py_str("guitar"), py_str("piano"), py_str("ukulele"), py_str("violin"), py_str("keyboard"), py_str("bass"), py_str("flute"), py_str("sax"), py_str("saxophone"), py_str("cello"), py_str("banjo"), py_str("mandolin"), py_str("harmonica"), py_str("trumpet"), py_str("clarinet"), py_str("choir"), py_str("singing"), py_str("practise singing")]) }
    s_OTHER_INSTRUMENTS(__data10());
    s_HEALTH_ASK(py_tuple(vec![py_str("healthier"), py_str("healthiest"), py_str("better for you"), py_str("better for me"), py_str("good for you"), py_str("caffeine"), py_str("healthy"), py_str("health")]));
    s_INSTRUMENTS(py_dict(vec![(py_str("guitar"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("guitar"), py_str("guitars"), py_str("ukulele"), py_str("uke"), py_str("banjo"), py_str("mandolin")])), (py_str("hint"), py_str("four chords")), (py_str("start"), py_str("Start with four chords that fit a hundred songs — G, C, D and E minor — and the strumming hand before the fretting one. Twenty minutes a day gets you a simple song in two or three months, and six lessons at the start mean nobody has to unlearn your hand position.")), (py_str("practise"), py_str("One song you actually like, in its easiest key: take the chord changes slowly, in a loop, then put them in order. Speed comes last — clean and slow beats fast and sloppy every time.")), (py_str("how_long"), py_str("With 15–20 minutes a day, four chords and a simple song come together in about a month, and something recognisable with a few more chords in three to six months.")), (py_str("how_often"), py_str("Fifteen to twenty minutes a day, five or six days a week, beats one long session at the weekend: the chord changes are what need repeating, and your fingertips need the odd rest day more than the extra minutes. Ten minutes is enough on a busy day - the streak matters more than the length."))])), (py_str("piano"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("piano"), py_str("pianos"), py_str("keyboard"), py_str("keyboards"), py_str("electric piano")])), (py_str("hint"), py_str("right hand")), (py_str("start"), py_str("Start with the right hand alone on a song you already know, one note at a time, and the left hand on two or three chords to go with it: hands separately for the first few weeks is not cheating, it is how everyone does it. Twenty minutes a day, and a teacher for the first month so your hand shape is right.")), (py_str("practise"), py_str("Right hand alone, slowly, on a tune you already know by ear — then the same bars with the left hand, then both together much slower than either felt. Five minutes of that daily beats an hour on Sunday.")), (py_str("how_long"), py_str("With 15–20 minutes a day, a simple tune with the right hand comes together in a few weeks, and both hands in three to six months — faster than most people expect, because the notes are laid out in front of you.")), (py_str("how_often"), py_str("Twenty minutes a day, five or six days a week, and shorter on a bad day rather than skipping it: ten focused minutes of scales and one bar done slowly is worth more than an hour on Sunday evening. Keep a day off."))])), (py_str("violin"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("violin"), py_str("violins"), py_str("viola"), py_str("fiddle"), py_str("cello")])), (py_str("hint"), py_str("open strings")), (py_str("start"), py_str("Start with the bow and the open strings: long, slow bow strokes on one string are the first month, because a straight bow and a steady sound come before any tune. Ten minutes a day, and a teacher from the first week — this is the instrument where a bad habit is hardest to undo.")), (py_str("practise"), py_str("Open strings with a long, even bow, then one finger at a time on the D and A strings: it sounds dreadful for a few weeks and that is the normal sound of learning. A teacher's ear in the room is worth more here than on any other instrument.")), (py_str("how_long"), py_str("Expect a few months of short tunes on one or two strings, and about a year before something sounds like the tune you set out to play: the bow is the slow part, not the fingers.")), (py_str("how_often"), py_str("Ten to fifteen minutes a day, every day if you can, because it is the bow arm and the intonation that have to repeat - a week off costs you more on the violin than on any other instrument. Stop while it still sounds good rather than practising tired."))])), (py_str("flute"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("flute"), py_str("flutes"), py_str("recorder"), py_str("piccolo")])), (py_str("hint"), py_str("head joint")), (py_str("start"), py_str("Start with the mouth, not the tune: getting a clean sound out of the head joint is the first week, then long notes on a few fingerings. Ten minutes a day, and a teacher early, because the embouchure is the thing that is hard to fix yourself.")), (py_str("practise"), py_str("Long notes on each fingering, from the easiest note upwards, listening for a steady sound: five minutes of that a day before any tune. Then a short tune in the easy keys, slowly.")), (py_str("how_long"), py_str("A clean sound takes a few weeks, a simple tune two or three months, and something you would play for someone else six months to a year.")), (py_str("how_often"), py_str("Ten to fifteen minutes a day is plenty, most days: five of long notes and the awkward fingering, then a short tune. Daily is what settles the embouchure, and a day off now and then does no harm - a tired mouth is where squeaks come from."))])), (py_str("reed"), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("clarinet"), py_str("sax"), py_str("saxophone"), py_str("trumpet"), py_str("oboe"), py_str("bassoon"), py_str("trombone"), py_str("cornet"), py_str("horn"), py_str("french horn"), py_str("bugle")])), (py_str("hint"), py_str("mouthpiece")), (py_str("start"), py_str("Start with the mouth: a clean, steady note on the mouthpiece alone is the first week, and long notes on a few fingerings after that. Ten minutes a day, and a teacher early, because the embouchure is the thing that is hard to fix yourself.")), (py_str("practise"), py_str("Long notes, from the easiest note upwards, listening for a steady sound and a clean start to each one: five minutes of that a day before any tune. Then a short tune in the easy keys, slowly.")), (py_str("how_long"), py_str("A clean sound takes a few weeks, a simple tune two or three months, and something you would play for someone else six months to a year.")), (py_str("how_often"), py_str("Ten to fifteen minutes a day, most days: long notes and a clean start to each one first, then a tune. Daily repetition is what builds the embouchure, and the mouth is what gives out first if you push it."))]))]));
    s_INSTRUMENT_GENERIC(py_dict(vec![(py_str("start"), py_str("Start with a teacher or a good app for the first month and ten or fifteen minutes a day on the basics: every instrument has its own awkward first fortnight, and one lesson early saves months of undoing. Pick one short piece you like as the target.")), (py_str("practise"), py_str("Whatever is awkward on it — the first clean note, the hand position, the breathing — ten minutes a day, then a short piece slowly. Speed comes last.")), (py_str("how_long"), py_str("With 15–20 minutes a day, a simple tune is usually a few months and something recognisable six months to a year, depending on the instrument.")), (py_str("how_often"), py_str("Short and often: ten to twenty minutes a day, most days of the week, beats one long session at the weekend. Whatever is awkward in it gets the first five minutes, and stopping while it is still going well is what keeps you coming back to it."))]));
    s_INSTRUMENT_WORDS(op_add(&op_add(&g_DRUMS(), &g_OTHER_INSTRUMENTS()), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_INSTRUMENTS(), "values", &[])) { let mut table: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&table.clone(), &py_str("names"))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    s_INSTRUMENT_LOOKBACK(Py::Int(6));
    s_CANT_DECIDE(py_tuple(vec![py_str("cant decide"), py_str("can't decide"), py_str("cannot decide"), py_str("undecided"), py_str("help me decide"), py_str("which should i"), py_str("what would you pick"), py_str("dont know which"), py_str("don't know which"), py_str("no idea which")]));
    fn __data11() -> Py { py_tuple(vec![py_str("out all day"), py_str("out at work"), py_str("at work all day"), py_str("work all day"), py_str("out for work"), py_str("away all day"), py_str("long hours"), py_str("night shifts"), py_str("night shift"), py_str("travel for work"), py_str("always out"), py_str("rarely home"), py_str("small flat"), py_str("no garden"), py_str("in a flat"), py_str("renting"), py_str("landlord"), py_str("allergic"), py_str("allergy"), py_str("allergies")]) }
    s_OUT_ALL_DAY(__data11());
    fn __data12() -> Py { py_tuple(vec![py_str("easier"), py_str("easy"), py_str("less work"), py_str("less effort"), py_str("less hassle"), py_str("independent"), py_str("quiet"), py_str("quieter"), py_str("silent"), py_str("noise"), py_str("noisy"), py_str("noisier"), py_str("loud"), py_str("louder"), py_str("clean"), py_str("cleaner"), py_str("tidy"), py_str("smaller"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("low maintenance"), py_str("self sufficient")]) }
    s_CAT_WINS(__data12());
    fn __data13() -> Py { py_tuple(vec![py_str("affectionate"), py_str("loving"), py_str("loyal"), py_str("friendly"), py_str("playful"), py_str("fun"), py_str("company"), py_str("companionship"), py_str("clingy"), py_str("cuddly"), py_str("cuddles"), py_str("trainable"), py_str("obedient"), py_str("protective"), py_str("energetic"), py_str("good with kids"), py_str("good with children"), py_str("interactive")]) }
    s_DOG_WINS(__data13());
    s_TRAIN_WORDS(py_tuple(vec![py_str("train"), py_str("trains"), py_str("trained"), py_str("training"), py_str("obedience"), py_str("obedient"), py_str("commands"), py_str("tricks")]));
    s_TRAIN_COMPARE(py_str("A dog, on that one: dogs work for food and praise and will sit, stay and come in a few weeks, where a cat learns a few tricks on its own terms and will not be told. If training the animal is part of the fun for you, get the dog."));
    s_CAT_ON_A_DIMENSION(py_str("A cat, on that one: it sleeps most of the day, does not bark at the postman and costs about £700 to £1,000 a year once insurance and a vet fund are in, where a dog is £1,500 and up before any vet bill. Pick the cat only because you want a cat, though, not just because it is the cheaper one."));
    s_DOG_ON_A_DIMENSION(py_str("Dogs, by a whisker: daft, straightforward and pleased to see you, and they show it plainly - a dog greets you at the door where a cat may not. The trade is the walk every morning and £1,500 a year before any vet bill, and an adult rescue is far easier than a puppy."));
    s_DOG_TRADE(py_str("Want one only if the person who wants it is also the one who will walk it at seven on a wet Tuesday for the next twelve years. A dog costs £1,500 to £2,000 a year before any vet bill and needs someone home at lunchtime or a walker; an adult rescue is far easier than a puppy. If that sounds fine rather than like a sacrifice, get one."));
    s_RABBIT_TRADE(py_str("A rabbit is an eight to ten year commitment and not a hutch pet: it needs a run it can stand up and stretch in, hay every day, another rabbit for company, and a vet who treats rabbits, because they hide illness until they are very ill. The daily work is the cleaning out and the company, and a rabbit kept alone in a small hutch is a miserable animal. A child can enjoy a rabbit but cannot be the one who cleans it out or notices it has stopped eating, so that job is an adult's. Get one because you want a rabbit for that long."));
    s_GUINEA_PIG_TRADE(py_str("A guinea pig lives five to seven years and should never be kept alone - two of them, with a hutch and a run, hay all day because their teeth grow constantly, and vegetables with vitamin C in them because they cannot make their own. They are the friendliest of the small pets if they are handled from the start, and the daily work is hay, water and cleaning out."));
    s_HAMSTER_TRADE(py_str("A hamster lives two to three years, sleeps through your day and is awake in the evening, so it is a pet to watch and handle gently rather than carry about. It wants the biggest cage you can give it with deep bedding to dig in, a wheel that does not bend its back, and a clean-out every week or so. Cheap to buy, not cheap to set up, and not a pet for a child on their own."));
    s_CAT_TRADE(py_str("A cat costs about £700 to £1,000 a year once insurance and a vet fund are in, and unlike a dog it does not need you home at lunchtime: an adult cat sleeps through a working day, which is why people who cannot have a dog often do well with a cat. Get an adult rather than a kitten if you are out, two if the house is empty all day, a litter tray each plus one, and settle the landlord and who looks after it when you go away before you fall for one."));
    s_CAT_WORDS(py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens")]));
    fn __data14() -> Py { py_tuple(vec![py_str("flap"), py_str("litter"), py_str("tray"), py_str("trays"), py_str("scratching"), py_str("tree"), py_str("carrier"), py_str("food"), py_str("foods"), py_str("treat"), py_str("treats"), py_str("toy"), py_str("toys"), py_str("brush"), py_str("claws"), py_str("collar"), py_str("microchip"), py_str("harness"), py_str("bed"), py_str("bowl"), py_str("bowls"), py_str("catnip")]) }
    s_CAT_EQUIPMENT(__data14());
    s_MOVE_TRADE(py_str("A move is decided by what you cannot change later: the commute, the noise, the light, and what you can walk to. Write the one thing each place gives you that the other cannot, then take the one you would miss on a bad day rather than the one that looks better on a good one. Move when the reason is still there in February."));
    s_CAR_TRADE(py_str("A car is decided by the trips you actually make, not the ones you imagine: mostly short local miles with charging at home, electric; a long motorway run every month, petrol or a hybrid. Put insurance, tax and a service into the first-year budget, then only look at cars under that. The reliable boring one beats the fun one you cannot afford to fix."));
    s_COURSE_TRADE(py_str("Pick the course by where you want to be at the end of it: a job, a licence, or the habit of studying. Ask two people who finished it what they do now, and count the hours a week it really takes. If the point is interest rather than a qualification, a short evening course tells you whether you like it before you pay for three years."));
    fn __data15() -> Py { py_tuple(vec![py_str("which"), py_str("what model"), py_str("what kind"), py_str("what sort"), py_str("what breed"), py_str("what size"), py_str("how much"), py_str("how big"), py_str("how old"), py_str("how long"), py_str("used"), py_str("second hand"), py_str("secondhand"), py_str("lead"), py_str("leads"), py_str("harness"), py_str("breed"), py_str("breeds"), py_str("price"), py_str("prices"), py_str("cost"), py_str("cheap"), py_str("cheapest"), py_str("best"), py_str("insurance"), py_str("tax"), py_str("mot"), py_str("vet"), py_str("food"), py_str("train"), py_str("training"), py_str("name"), py_str("names"), py_str("age"), py_str("size"), py_str("survey"), py_str("surveys"), py_str("surveyor"), py_str("surveyors"), py_str("valuation"), py_str("snagging"), py_str("dealer"), py_str("dealership"), py_str("auction"), py_str("private seller")]) }
    s_QUALIFIED(__data15());
    s_TWO_WAYS(py_tuple(vec![py_tuple(vec![py_str("buy"), py_str("buying"), py_str("bought"), py_str("purchase"), py_str("purchasing"), py_str("mortgage"), py_str("owner")]), py_tuple(vec![py_str("rent"), py_str("renting"), py_str("rented"), py_str("rents"), py_str("tenant"), py_str("lease"), py_str("leasing")])]));
    fn __data16() -> Py { py_tuple(vec![py_str("flat"), py_str("flats"), py_str("house"), py_str("houses"), py_str("moving house"), py_str("move house"), py_str("moving in together"), py_str("rent"), py_str("renting"), py_str("landlord"), py_str("area"), py_str("neighbourhood"), py_str("commute")]) }
    s_DECISION_SUBJECTS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("rescue dog")]), g_DOG_TRADE()]), py_tuple(vec![py_tuple(vec![py_str("rabbit"), py_str("rabbits"), py_str("bunny")]), g_RABBIT_TRADE()]), py_tuple(vec![py_tuple(vec![py_str("guinea pig"), py_str("guinea pigs"), py_str("guinea-pig"), py_str("cavy")]), g_GUINEA_PIG_TRADE()]), py_tuple(vec![py_tuple(vec![py_str("hamster"), py_str("hamsters"), py_str("gerbil"), py_str("gerbils"), py_str("degus"), py_str("chinchilla"), py_str("chinchillas")]), g_HAMSTER_TRADE()]), py_tuple(vec![__data16(), g_MOVE_TRADE()]), py_tuple(vec![py_tuple(vec![py_str("car"), py_str("cars"), py_str("van"), py_str("second car")]), g_CAR_TRADE()]), py_tuple(vec![py_tuple(vec![py_str("course"), py_str("courses"), py_str("degree"), py_str("degrees"), py_str("masters"), py_str("college"), py_str("university"), py_str("uni"), py_str("apprenticeship"), py_str("night class")]), g_COURSE_TRADE()])]));
    fn __data17() -> Py { py_tuple(vec![py_str("cant decide"), py_str("can't decide"), py_str("cannot decide"), py_str("cant choose"), py_str("can't choose"), py_str("torn between"), py_str("not sure which"), py_str("not sure whether"), py_str("not sure if"), py_str("unsure which"), py_str("unsure whether"), py_str("going back and forth"), py_str("back and forth"), py_str("two minds"), py_str("on the fence"), py_str("help me decide"), py_str("decide between"), py_str("deciding between"), py_str("should we get"), py_str("should i get"), py_str("should we buy"), py_str("should i buy"), py_str("thinking of getting"), py_str("thinking about getting"), py_str("thinking of moving"), py_str("thinking about moving"), py_str("thinking of buying"), py_str("whether to"), py_str("thinking of doing"), py_str("thinking about doing"), py_str("thinking of studying"), py_str("thinking about studying"), py_str("thinking of going back"), py_str("worth getting"), py_str("worth having")]) }
    s_STUCK_SHAPES(__data17());
    s_DECIDE_WORDS(py_tuple(vec![py_str("between"), py_str("whether"), py_str("torn"), py_str("unsure"), py_str("decide"), py_str("deciding"), py_str("decision"), py_str("choose"), py_str("choice"), py_str("option"), py_str("options"), py_str("stuck")]));
    s_MADE_UP(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|they|she|he)\\s+(?:have\\s+|had\\s+)?(?:decided|chose|picked|went with|bought|got|ordered|took)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEW_ONE(stdlib_call("re", "compile", &[py_str("\\b(?:new|another|bigger|smaller|different|better|second|cheaper)\\s+one\\b|\\bone\\s+instead\\b|\\bone\\s+of\\s+(?:those|them)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WHAT_TO_BUY(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|will|would) (?:i|we) (?:need|buy|get)\\b|\\bwhat (?:do|should|shall) (?:i|we) (?:need|buy|get) (?:to|for)\\b|\\b(?:need|needs) to (?:buy|get)\\b|\\bshopping list\\b|\\bchecklist\\b|\\bwhat to (?:buy|get)\\b|\\bwhich (?:kit|supplies)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__TWO_OF_THEM(stdlib_call("re", "compile", &[py_str("\\btwo (?:cats|kittens)\\b|\\bsecond (?:cat|kitten)\\b|\\bpair of (?:cats|kittens)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAT_VERDICT_SHAPES(py_tuple(vec![py_str("good idea"), py_str("worth it"), py_str("sensible"), py_str("wise"), py_str("a mistake"), py_str("right for me"), py_str("right for us"), py_str("suit me"), py_str("suit us")]));
    s__ANOTHER_CATS_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|toddler|toddlers|child|children|kid|kids|newborn|pram|flats?|rent|rents|rented|renting|landlord|rescue|rescued|garden|outdoor|indoors?|dog|dogs|puppy|puppies|cost|costs|price|prices|afford|how much)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data18() -> Py { py_tuple(vec![py_str("is it normal"), py_str("is that normal"), py_str("is this normal"), py_str("isnt normal"), py_str("isn't normal"), py_str("not normal"), py_str("normal to"), py_str("normal for"), py_str("is it weird"), py_str("is that weird"), py_str("is this weird"), py_str("weird to"), py_str("weird that"), py_str("weird if"), py_str("am i weird"), py_str("am i strange"), py_str("is it strange"), py_str("is that strange"), py_str("is it odd"), py_str("is that odd"), py_str("odd that"), py_str("odd to"), py_str("is it unusual"), py_str("is that unusual"), py_str("is it ok that"), py_str("is it okay that"), py_str("is there something wrong with me")]) }
    s__NORMAL_ASK(__data18());
    fn __data19() -> Py { py_tuple(vec![py_str("live at home"), py_str("living at home"), py_str("live with my parents"), py_str("living with my parents"), py_str("live with your parents"), py_str("living with your parents"), py_str("moved back home"), py_str("moved back in"), py_str("still at home"), py_str("still live with my mum"), py_str("still live with my dad"), py_str("live with my mum"), py_str("live with my dad"), py_str("back with my parents")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("not want children"), py_str("not want kids"), py_str("not wanting children"), py_str("not wanting kids"), py_str("not wanting a baby"), py_str("not want a baby"), py_str("childfree"), py_str("child free"), py_str("not having children"), py_str("not having kids"), py_str("no children by choice"), py_str("never want kids"), py_str("childless by choice")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("still single"), py_str("single at 30"), py_str("single at 40"), py_str("never had a girlfriend"), py_str("never had a boyfriend"), py_str("never have had a girlfriend"), py_str("never have had a boyfriend"), py_str("never had a partner"), py_str("never have had a partner"), py_str("never been in a relationship"), py_str("never dated"), py_str("not had a relationship"), py_str("never had a relationship"), py_str("no relationship at 30")]) }
    s__NORMAL_SUBJECTS(py_tuple(vec![py_tuple(vec![__data19(), py_str("Living at home as an adult is common: about a quarter of 25 to 34 year olds in the UK still do, and more do wherever rents are high. What makes it work is agreed terms - what you pay, what you do round the house, and a date to look at it again - not how long it has been.")]), py_tuple(vec![py_tuple(vec![py_str("talk to myself"), py_str("talking to myself"), py_str("talk to yourself"), py_str("talking to yourself"), py_str("talk out loud"), py_str("talking out loud"), py_str("answer myself"), py_str("argue with myself")]), py_str("Nearly everyone does - surveys put it around 9 in 10 adults - and talking out loud on purpose is how people hold a task in mind, which is why athletes and surgeons do it. It is only worth a thought if the voice is not yours or it tells you what to do.")]), py_tuple(vec![py_tuple(vec![py_str("sleep with a teddy"), py_str("sleep with my teddy"), py_str("still have a teddy"), py_str("sleep with a soft toy"), py_str("teddy"), py_str("teddies"), py_str("comfort blanket"), py_str("blankie"), py_str("cuddly toy"), py_str("stuffed toy")]), py_str("About a third of adults in surveys still have a childhood soft toy, and plenty of them sleep with it: it is a sleep cue, and a sleep cue that works is worth keeping. It only matters if it is the only way you can settle at all.")]), py_tuple(vec![__data20(), py_str("About one in five women in the UK reaches 45 without children, and choosing that is common enough that nobody owes anyone a reason. The thing worth doing is saying it out loud to a partner early, because the assumptions people make are where the trouble is.")]), py_tuple(vec![py_tuple(vec![py_str("cry at films"), py_str("cry at a film"), py_str("cry at movies"), py_str("cry at a movie"), py_str("crying at films"), py_str("cry at the cinema"), py_str("cry at adverts"), py_str("cry at tv"), py_str("cry at everything")]), py_str("That is being moved, and it is what a story is built to do: films pace the music, the close-ups and the pauses to make you feel for someone. Crying easily at a screen goes with empathy rather than fragility, and most people find it settles as the mood does.")]), py_tuple(vec![py_tuple(vec![py_str("broke up"), py_str("break up"), py_str("breakup"), py_str("break-up"), py_str("split up"), py_str("got dumped"), py_str("being dumped"), py_str("divorce"), py_str("divorced"), py_str("separation"), py_str("she left me"), py_str("he left me")]), py_str("Sad after a break-up is grief, not an illness: it comes in waves with good hours in between, and it eases over weeks rather than days. The line where it is worth a GP is low mood most of the day, most days, for two weeks, with nothing giving you pleasure any more.")]), py_tuple(vec![py_tuple(vec![py_str("dont drink"), py_str("don't drink"), py_str("not drinking"), py_str("i dont drink alcohol"), py_str("teetotal"), py_str("sober"), py_str("not a drinker"), py_str("stopped drinking"), py_str("gave up alcohol"), py_str("gave up drinking")]), py_str("About one in five adults in the UK does not drink at all, and the under-30s drink less than their parents did, so you are much less of an odd one out than a round of drinks makes it feel. 'Not for me, thanks' is a whole answer, and asking them something after it ends the conversation about your glass.")]), py_tuple(vec![__data21(), py_str("Plenty of people get to 30 without a relationship, and the number is rising: the timing of a first relationship stretches over a decade, and none of it is a verdict on you. What makes the difference is meeting people in a place you would go anyway, rather than waiting to be picked.")])]));
    s__NORMAL_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NORMAL_SUBJECTS()) { let __unpk894 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk894[0].clone(); let mut _u: Py = __unpk894[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MANNERS_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:rude|ruder|politest|polite|impolite|cheeky|stingy|stingey|tight|mean|weird|strange|normal|ok|okay|fine|wrong|right|fair|acceptable|unacceptable|awkward|etiquette|manners|bad\\s+form|selfish|generous|expected|allowed|supposed|unfair|annoying|greedy|cheapskate)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TABLE_BILL(stdlib_call("re", "compile", &[py_str("\\b(?:bill|tab|check|cheque|dinner|lunch|brunch|meal|restaurant|drinks|round|takeaway|takeout|coffee|starter|dessert|menu)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SHARED_HOME(stdlib_call("re", "compile", &[py_str("\\b(?:rent|flats?|houses?|housemates?|flatmates?|roommates?|household|landlord|tenancy|mortgage|council\\s+tax|utilit(?:y|ies)|electricity|energy|heating|broadband|wifi|internet|partner|husband|wife|spouse|boyfriend|girlfriend|couple|income|earns?|salary|pays?\\s+(?:half|halves)|joint\\s+account|bills?\\s+between\\s+us)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data23() -> Py { py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("toddler"), py_str("baby"), py_str("teen"), py_str("family"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("watch"), py_str("watching"), py_str("something to watch"), py_str("book"), py_str("books"), py_str("read"), py_str("reading"), py_str("recipe"), py_str("cook"), py_str("make"), py_str("save"), py_str("saving"), py_str("savings"), py_str("money"), py_str("fund"), py_str("afford"), py_str("pay"), py_str("pounds"), py_str("ideas"), py_str("idea"), py_str("things to do"), py_str("something to do"), py_str("what can i do"), py_str("what can we do"), py_str("what should i do"), py_str("what to do"), py_str("bored"), py_str("step"), py_str("steps"), py_str("music"), py_str("album"), py_str("albums"), py_str("playlist"), py_str("playlists"), py_str("podcast"), py_str("podcasts"), py_str("listen"), py_str("listening"), py_str("spotify"), py_str("tunes")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("my new"), py_str("my old"), py_str("my own"), py_str("do you like my"), py_str("think of my"), py_str("reckon of my"), py_str("i bought"), py_str("i've bought"), py_str("i just bought"), py_str("i got"), py_str("i've got"), py_str("we bought"), py_str("we got"), py_str("our new"), py_str("i have got"), py_str("ive got"), py_str("i have"), py_str("we have"), py_str("we have got"), py_str("i was given"), py_str("got given"), py_str("someone gave me"), py_str("i have been given"), py_str("was given a"), py_str("i have just bought"), py_str("have just got"), py_str("just got a"), py_str("i use my"), py_str("i use the")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("how long does"), py_str("how long do"), py_str("how long will"), py_str("how long to"), py_str("how long before"), py_str("take to get cold"), py_str("get cold"), py_str("heat up"), py_str("heats up"), py_str("preheat"), py_str("service"), py_str("services"), py_str("servicing"), py_str("serviced")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("need a vent"), py_str("needs a vent"), py_str("need venting"), py_str("needs venting"), py_str("need to vent"), py_str("needs to vent"), py_str("vent for"), py_str("vent a tumble"), py_str("vent the dryer"), py_str("vent my dryer"), py_str("vent a dryer"), py_str("venting a"), py_str("outside vent"), py_str("wall vent"), py_str("vent hose"), py_str("hose keeps"), py_str("where can i put"), py_str("can i put my dryer"), py_str("can a tumble dryer go"), py_str("can a dryer go"), py_str("can i put a dryer")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("scared"), py_str("scary"), py_str("afraid"), py_str("frightened"), py_str("frightens"), py_str("terrified"), py_str("terrifies"), py_str("panics"), py_str("hates"), py_str("hate"), py_str("hiding"), py_str("hides"), py_str("cowers"), py_str("trembles"), py_str("shakes"), py_str("shaking"), py_str("barks"), py_str("barking")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("should i"), py_str("is it worth"), py_str("get"), py_str("getting"), py_str("buy"), py_str("buying"), py_str("any good"), py_str("reckon"), py_str("pay for itself"), py_str("payback"), py_str("pay back"), py_str("save money"), py_str("recommend"), py_str("recommended"), py_str("recommendation"), py_str("recommendations"), py_str("any point in"), py_str("point in solar"), py_str("waste of money"), py_str("do i need"), py_str("do i need solar"), py_str("savings"), py_str("roof"), py_str("battery"), py_str("install"), py_str("installing"), py_str("installed"), py_str("quote"), py_str("south facing"), py_str("shade"), py_str("kwh"), py_str("export"), py_str("feed in tariff")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("how do"), py_str("how does"), py_str("how is"), py_str("what is"), py_str("what are"), py_str("explain"), py_str("work"), py_str("works"), py_str("working"), py_str("broken"), py_str("not working"), py_str("fault"), py_str("faulty"), py_str("panel is"), py_str("panels are"), py_str("clean"), py_str("cleaning"), py_str("moss"), py_str("bird"), py_str("installer has"), py_str("already")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("blood test"), py_str("blood tests"), py_str("period"), py_str("periods"), py_str("spotting"), py_str("discharge"), py_str("lump"), py_str("lumps"), py_str("pregnant"), py_str("pregnancy"), py_str("miscarriage"), py_str("hiv"), py_str("std"), py_str("stis"), py_str("pain"), py_str("pains"), py_str("hurts"), py_str("bleeding"), py_str("temperature"), py_str("rash"), py_str("itch"), py_str("itchy")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("cinema"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("gig"), py_str("gigs"), py_str("concert"), py_str("theatre"), py_str("show"), py_str("museum"), py_str("gallery"), py_str("exhibition"), py_str("festival"), py_str("pub"), py_str("restaurant"), py_str("cafe"), py_str("meal"), py_str("brunch"), py_str("lunch"), py_str("dinner"), py_str("coffee"), py_str("eat"), py_str("eating"), py_str("class"), py_str("course"), py_str("gym"), py_str("swim"), py_str("match"), py_str("talk"), py_str("things"), py_str("going out"), py_str("go out"), py_str("night out"), py_str("day out"), py_str("weekend")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("travel"), py_str("travelling"), py_str("traveling"), py_str("trip"), py_str("holiday"), py_str("holidays"), py_str("abroad"), py_str("flight"), py_str("flights"), py_str("hostel"), py_str("backpacking"), py_str("swim"), py_str("swimming"), py_str("lengths"), py_str("wedding"), py_str("lonely"), py_str("loneliness"), py_str("lonely"), py_str("what can i eat"), py_str("what should i eat"), py_str("what to eat"), py_str("what can i have"), py_str("meal ideas"), py_str("meal plan"), py_str("recipes"), py_str("recipe"), py_str("cooking"), py_str("cook"), py_str("food shop"), py_str("batch"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("pup"), py_str("rabbit"), py_str("hamster"), py_str("budgie"), py_str("parrot"), py_str("pet"), py_str("pets"), py_str("tortoise"), py_str("ferret"), py_str("guinea pig"), py_str("goldfish"), py_str("fish"), py_str("horse"), py_str("pony"), py_str("snake"), py_str("lizard")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("holiday"), py_str("holidays"), py_str("flight"), py_str("flights"), py_str("hotel"), py_str("hotels"), py_str("trip"), py_str("travel"), py_str("accommodation"), py_str("airbnb"), py_str("villa"), py_str("restaurant"), py_str("table"), py_str("train"), py_str("trains"), py_str("tickets"), py_str("ferry"), py_str("car hire"), py_str("hire car"), py_str("check in"), py_str("check-in"), py_str("abroad")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("split the bill"), py_str("splitting the bill"), py_str("split the cost"), py_str("go halves"), py_str("go halfs"), py_str("splitting a bill"), py_str("split the tab"), py_str("split tabs"), py_str("split the check"), py_str("split bills"), py_str("splitting bills"), py_str("divide the bill"), py_str("dividing the bill"), py_str("divvy up the bill"), py_str("share the bill"), py_str("sharing the bill")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("fair"), py_str("ok"), py_str("rude"), py_str("should"), py_str("right"), py_str("better"), py_str("how"), py_str("best"), py_str("friends"), py_str("even"), py_str("fine"), py_str("okay"), py_str("etiquette"), py_str("manners"), py_str("awkward"), py_str("impolite"), py_str("polite"), py_str("cheeky"), py_str("stingy"), py_str("tight"), py_str("mean"), py_str("rude"), py_str("weird"), py_str("strange"), py_str("wrong"), py_str("acceptable"), py_str("normal"), py_str("expected"), py_str("bad form"), py_str("selfish"), py_str("generous"), py_str("allowed"), py_str("supposed"), py_str("unfair"), py_str("annoying"), py_str("greedy"), py_str("stingey"), py_str("cheapskate")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("housemate"), py_str("housemates"), py_str("flatmate"), py_str("flatmates"), py_str("roommate"), py_str("roommates"), py_str("household"), py_str("flat share"), py_str("house share"), py_str("shared flat"), py_str("shared house"), py_str("with my flat"), py_str("council tax"), py_str("electricity"), py_str("broadband"), py_str("partner"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("fiance"), py_str("fiancee"), py_str("other half"), py_str("as a couple"), py_str("joint account"), py_str("our bills"), py_str("the rent"), py_str("taxi"), py_str("taxis"), py_str("cab"), py_str("cabs"), py_str("uber"), py_str("ride"), py_str("fare"), py_str("fares"), py_str("petrol"), py_str("diesel"), py_str("fuel"), py_str("parking"), py_str("toll"), py_str("tolls"), py_str("hotel"), py_str("hotel room"), py_str("a room"), py_str("the room"), py_str("rooms"), py_str("cottage"), py_str("cottages"), py_str("airbnb"), py_str("villa"), py_str("caravan"), py_str("holiday house"), py_str("holiday home"), py_str("apartment"), py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("collection"), py_str("whip round"), py_str("kitty"), py_str("secret santa"), py_str("leaving present"), py_str("wedding present"), py_str("holiday"), py_str("holidays"), py_str("vacation"), py_str("flight"), py_str("flights"), py_str("trip"), py_str("weekend away"), py_str("week away"), py_str("weekend break"), py_str("city break"), py_str("abroad"), py_str("group holiday")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("how many"), py_str("how long does it take"), py_str("how long until"), py_str("how often"), py_str("fail"), py_str("failed"), py_str("failing"), py_str("fails"), py_str("flunked"), py_str("renew"), py_str("renews"), py_str("renewing"), py_str("renewal"), py_str("expired"), py_str("expires"), py_str("runs out"), py_str("replace"), py_str("replacement"), py_str("lost"), py_str("photocard")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("instrument"), py_str("piano"), py_str("guitar"), py_str("ukulele"), py_str("violin"), py_str("drums"), py_str("keyboard"), py_str("flute"), py_str("cello"), py_str("sax"), py_str("saxophone"), py_str("trumpet"), py_str("clarinet"), py_str("harmonica"), py_str("singing"), py_str("sing"), py_str("choir"), py_str("harp")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("adult"), py_str("learn"), py_str("learning"), py_str("learned"), py_str("learnt"), py_str("start"), py_str("begin"), py_str("wanted"), py_str("always"), py_str("worth"), py_str("45"), py_str("40"), py_str("50"), py_str("60"), py_str("age"), py_str("old age"), py_str("my age")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("daughter"), py_str("son"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("grandson"), py_str("granddaughter"), py_str("nephew"), py_str("niece"), py_str("teenager"), py_str("toddler"), py_str("washing"), py_str("laundry"), py_str("machine"), py_str("dryer"), py_str("tumble"), py_str("ear"), py_str("ears"), py_str("eardrum"), py_str("boiler"), py_str("loud"), py_str("noisy"), py_str("computer"), py_str("laptop"), py_str("typing"), py_str("champagne"), py_str("prosecco"), py_str("wine"), py_str("sparkling")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("song"), py_str("tune"), py_str("piano"), py_str("guitar"), py_str("ukulele"), py_str("violin"), py_str("instrument"), py_str("music"), py_str("chords"), py_str("scales"), py_str("beat"), py_str("drum"), py_str("drums"), py_str("flute"), py_str("cello"), py_str("sax"), py_str("trumpet"), py_str("singing")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("ear"), py_str("ears"), py_str("eardrum"), py_str("washing"), py_str("machine"), py_str("laundry"), py_str("brake"), py_str("brakes"), py_str("cardrum"), py_str("tumble"), py_str("dryer"), py_str("open mic"), py_str("open mike"), py_str("gig"), py_str("gigs"), py_str("concert"), py_str("recital"), py_str("audition"), py_str("performance"), py_str("performing"), py_str("on stage"), py_str("nervous"), py_str("mc")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("driving"), py_str("exam"), py_str("test"), py_str("interview"), py_str("speech"), py_str("presentation"), py_str("football"), py_str("singing"), py_str("language"), py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("amp"), py_str("buy"), py_str("purchase"), py_str("shop"), py_str("pedal"), py_str("strings"), py_str("course"), py_str("app"), py_str("tutor"), py_str("teacher")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("driving"), py_str("exam"), py_str("test"), py_str("interview"), py_str("speech"), py_str("presentation"), py_str("football"), py_str("tennis"), py_str("golf"), py_str("singing"), py_str("language"), py_str("spanish"), py_str("french"), py_str("italian"), py_str("german"), py_str("amp"), py_str("buy"), py_str("purchase"), py_str("shop"), py_str("pedal"), py_str("strings"), py_str("course"), py_str("app"), py_str("tutor"), py_str("teacher"), py_str("rest"), py_str("stop"), py_str("hurt"), py_str("hurts"), py_str("pain"), py_str("sore"), py_str("injury"), py_str("injured"), py_str("serve"), py_str("serves"), py_str("toss"), py_str("tosses"), py_str("swimming"), py_str("swim"), py_str("running"), py_str("run"), py_str("cycling"), py_str("cycle"), py_str("gym"), py_str("weights"), py_str("yoga"), py_str("dancing"), py_str("dance"), py_str("singing"), py_str("boxing"), py_str("climbing"), py_str("skating"), py_str("skiing"), py_str("batting"), py_str("bowling"), py_str("cricket"), py_str("golf"), py_str("putting"), py_str("netball"), py_str("hockey"), py_str("basketball"), py_str("rugby"), py_str("football"), py_str("tennis"), py_str("badminton"), py_str("squash"), py_str("rowing"), py_str("sailing")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("cheaper"), py_str("cheap"), py_str("cheapest"), py_str("cost"), py_str("costs"), py_str("costing"), py_str("afford"), py_str("affordable"), py_str("money"), py_str("price"), py_str("prices"), py_str("expensive"), py_str("per year"), py_str("a year"), py_str("a month"), py_str("insurance")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("than"), py_str("because"), py_str("degrees"), py_str("radiator"), py_str("radiators"), py_str("boiler"), py_str("thermostat"), py_str("bleed"), py_str("bleeding"), py_str("at the top"), py_str("at the bottom"), py_str("toes"), py_str("hands"), py_str("bath"), py_str("shower"), py_str("showers"), py_str("swim"), py_str("swimming"), py_str("pool"), py_str("sauna"), py_str("compress"), py_str("ice pack"), py_str("heat pack"), py_str("hot water bottle"), py_str("sore"), py_str("muscle"), py_str("muscles"), py_str("back"), py_str("sprain"), py_str("sprained"), py_str("stiff"), py_str("stiffness"), py_str("injury"), py_str("injured"), py_str("cramp"), py_str("ache"), py_str("aches"), py_str("fever"), py_str("chills")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("step"), py_str("steps"), py_str("wont"), py_str("won't"), py_str("isnt"), py_str("isn't"), py_str("broken"), py_str("fault"), py_str("leak"), py_str("leaking"), py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("drain"), py_str("not working"), py_str("sick"), py_str("ill"), py_str("vet"), py_str("insurance"), py_str("tax"), py_str("mot"), py_str("service"), py_str("repair"), py_str("error"), py_str("flashing"), py_str("or"), py_str("vs"), py_str("versus"), py_str("either"), py_str("name"), py_str("names"), py_str("naming"), py_str("called"), py_str("call it"), py_str("what to call"), py_str("difference"), py_str("differences"), py_str("neuter"), py_str("neutered"), py_str("neutering"), py_str("neuters"), py_str("spay"), py_str("spayed"), py_str("spaying"), py_str("castrate"), py_str("castrated"), py_str("the snip"), py_str("watch"), py_str("watching"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("series"), py_str("tv"), py_str("netflix"), py_str("read"), py_str("reading"), py_str("book"), py_str("books"), py_str("play"), py_str("game"), py_str("games")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("need"), py_str("needs"), py_str("need to"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("shop"), py_str("shopping"), py_str("list"), py_str("kit"), py_str("litter"), py_str("tray"), py_str("food"), py_str("feed"), py_str("name"), py_str("called"), py_str("call it"), py_str("spay"), py_str("spayed"), py_str("spaying"), py_str("neuter"), py_str("neutered"), py_str("neutering"), py_str("the snip"), py_str("scratch"), py_str("scratching"), py_str("hiss"), py_str("hissing"), py_str("vet"), py_str("ill"), py_str("sick"), py_str("unwell"), py_str("wont"), py_str("won't"), py_str("isn't"), py_str("isnt"), py_str("broken"), py_str("difference"), py_str("vs"), py_str("versus"), py_str("two"), py_str("second"), py_str("pair"), py_str("better than one"), py_str("another")]) }
    fn __data22() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("best-season")); __args.push(py_tuple(vec![py_str("season"), py_tuple(vec![py_str("best"), py_str("favourite"), py_str("favorite"), py_str("think"), py_str("pick"), py_str("prefer"), py_str("like"), py_str("love"), py_str("better"), py_str("decide"), py_str("undecided")])])); __args.push(_asked(&[py_func("_best_season", |args: &[Py]| _best_season(args))])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("visit"), py_str("travel"), py_str("holiday"), py_str("trip"), py_str("plant"), py_str("sow"), py_str("grow")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, _either(&[py_str("air-fryer-or-oven"), py_tuple(vec![py_str("air fryer"), py_str("air fryers")]), py_tuple(vec![py_str("oven")]), _asked(&[py_str("Oven for a big tray and anything wet; air fryer for a small, crisp batch with less heat in the kitchen. If you already cook for one or two, the air fryer earns its space.")])]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("pizza-topping"), py_tuple(vec![py_str("pizza"), py_tuple(vec![py_str("topping"), py_str("best"), py_str("favourite"), py_str("favorite"), py_str("think"), py_str("pick")])]), _asked(&[py_str("Pepperoni, hot out of the oven, is the one I'd pick. Mushroom and ham is the better pizza when you want the whole thing rather than a slice.")])]), pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("pineapple-pizza"), py_tuple(vec![py_str("pineapple"), py_str("pizza")]), _asked(&[py_str("For it, honestly. Sweet with salty ham and hot cheese works; people mostly enjoy being angry about it.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rainy-sunday")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rainy"), py_str("raining"), py_str("wet weekend")]), py_tuple(vec![py_str("sunday"), py_str("day"), py_str("weekend"), py_str("afternoon"), py_str("think"), py_str("like")])])); __args.push(_asked(&[py_str("I like them: a rainy Sunday is a good excuse to do something that needs a whole afternoon — a long film, a proper cook, or a book with the phone in another room.")])); __kws.extend(vec![("avoid".to_string(), __data23())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("cereal-soup"), py_tuple(vec![py_str("cereal"), py_tuple(vec![py_str("soup"), py_str("salad"), py_str("think")])]), _asked(&[py_str("Not a soup. Soup is cooked and served hot; cereal is assembled cold at the last second, which makes it closer to a salad with milk.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("air-fryer")); __args.push(py_tuple(vec![py_tuple(vec![py_str("air fryer"), py_str("air fryers")]), g_TASTE()])); __args.push(_asked(&[py_str("Worth it for one or two people who like crisp things: chips, roast vegetables and chicken thighs in half the time. Skip it if you cook for four or already have a good fan oven.")])); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_NOT_A_BUYING_ASK(), &g_DISPOSAL()), &g_FAULT()), &g_TRADES()), &py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("cleaner"), py_str("wash"), py_str("washing"), py_str("grease"), py_str("greasy"), py_str("crumbs"), py_str("scrub"), py_str("dirty"), py_str("stain"), py_str("stains")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("opinion-on-my-gadget")); __args.push(py_tuple(vec![py_tuple(vec![py_str("blender"), py_str("food processor"), py_str("slow cooker"), py_str("bread maker"), py_str("smoothie maker"), py_str("juicer"), py_str("coffee machine"), py_str("stand mixer"), py_str("pressure cooker"), py_str("sandwich toaster"), py_str("ice cream maker"), py_str("air fryer")]), __data24()])); __args.push(_asked(&[py_func("_gadget_view", |args: &[Py]| _gadget_view(args))])); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_FAULT(), &g_TRADES()), &py_tuple(vec![py_str("broken"), py_str("wont"), py_str("won't"), py_str("stinks"), py_str("smells"), py_str("leaking")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("worth-an-appliance")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dishwasher"), py_str("tumble dryer"), py_str("dryer"), py_str("washing machine"), py_str("slow cooker"), py_str("bread maker"), py_str("blender"), py_str("food processor"), py_str("pressure cooker"), py_str("vacuum"), py_str("hoover"), py_str("freezer")]), g_TASTE()])); __args.push(py_func("_worth_an_appliance", |args: &[Py]| _worth_an_appliance(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_A_BUYING_ASK(), &g_DISPOSAL()), &g_FAULT()), &g_TRADES()), &g_OWN_BUYING_ANSWER()), &g_A_JOB_ON_IT()), &__data25()), &__data26()), &__data27())), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.lookup", "asks_where_a_shop_answers", &[understood.clone()]))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("worth-solar")); __args.push(py_tuple(vec![py_tuple(vec![py_str("solar panel"), py_str("solar panels"), py_str("solar pv"), py_str("solar tiles"), py_str("pv panels"), py_str("solar")]), __data28()])); __args.push(py_str("Worth it if the roof is roughly south facing, unshaded, and somebody is in during the day: a typical 4kW system costs about £5,000 to £7,000 fitted and pays back in 10 to 14 years on the avoided bill plus the export payment. A battery rarely pays for itself. Get two quotes with the yearly kWh in writing, and the MCS certificate.")); __kws.extend(vec![("avoid".to_string(), __data29())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("is-that-normal")); __args.push(py_tuple(vec![g__NORMAL_WORDS(), g__NORMAL_ASK()])); __args.push(py_func("_the_normal_answer", |args: &[Py]| _the_normal_answer(args))); __kws.extend(vec![("avoid".to_string(), __data30()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("eat-alone"), py_tuple(vec![py_tuple(vec![py_str("eat alone"), py_str("eating alone"), py_str("by myself"), py_str("by yourself"), py_str("on my own"), py_str("on your own"), py_str("eating on your own"), py_str("eat on my own")]), py_tuple(vec![py_str("restaurant"), py_str("cafe"), py_str("pub")])]), _asked(&[py_str("Not rude at all, and it gets easier with a book or a seat at the bar. Nobody in there is thinking about you half as much as you fear.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("out-on-your-own")); __args.push(py_tuple(vec![__data31(), py_tuple(vec![py_str("on my own"), py_str("on your own"), py_str("alone"), py_str("by myself"), py_str("by yourself"), py_str("solo"), py_str("single ticket"), py_str("no one to go with"), py_str("nobody to go with"), py_str("without anyone")])])); __args.push(_asked(&[py_str("Go, and go once without talking yourself out of it: the hard part is the ten minutes before you sit down, not the two hours after. Book one seat at the end of a row for the cinema or the theatre so you are not climbing over people, take a bar stool rather than a table in a pub or a restaurant, and arrive five minutes late rather than half an hour early. Take something to do with your hands the first time and pick a weekday evening - the second trip is easy.")])); __kws.extend(vec![("avoid".to_string(), __data32()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("phone-at-dinner"), py_tuple(vec![py_str("phone"), py_tuple(vec![py_str("dinner table"), py_str("at dinner"), py_str("at the table"), py_str("during dinner"), py_str("meal")])]), _asked(&[py_str("Face down and out of reach, at least while you're eating. A phone on the table pulls attention even when it's silent; taking it out to show a photo is fine, it's the checking that annoys people.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("read-book-first")); __args.push(py_tuple(vec![py_str("book"), py_tuple(vec![py_str("film"), py_str("movie")]), py_tuple(vec![py_str("before"), py_str("first"), py_str("adaptation"), py_str("better"), py_str("worse"), py_str("which"), py_str("as good as"), py_str("good as")])])); __args.push(_asked(&[py_str("Read the book first if you care about the story, watch the film first if you might not finish the book. An adaptation cuts, so the film rarely ruins a good novel — the other way round happens more.")])); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&pyrt::xrefs::xcall("knowledge.entertainment", "the_book_is_a_reservation", &[understood.clone()]))) })), ("avoid".to_string(), __data33())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("split-bill-etiquette")); __args.push(py_tuple(vec![__data34(), __data35()])); __args.push(_asked(&[py_str("Even split is fine when everyone ordered roughly the same; ask for separate bills when one person had the steak and the wine. Say it before the bill arrives and nobody has to be awkward.")])); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_a_table_bills_line", |args: &[Py]| _a_table_bills_line(args))), ("avoid".to_string(), __data36())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("regift"), py_tuple(vec![py_tuple(vec![py_str("regift"), py_str("re-gift"), py_str("re gift"), py_str("give it away")]), py_tuple(vec![py_str("ok"), py_str("rude"), py_str("fine"), py_str("should"), py_str("wrong")])]), _asked(&[py_str("Fine, with two rules: the person who gave it to you never finds out, and the next person would actually use it. Take the original card out first.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-to-drive")); __args.push(py_tuple(vec![py_tuple(vec![py_str("learn to drive"), py_str("driving licence"), py_str("driving license"), py_str("driving test"), py_str("driving lesson"), py_str("driving lessons")]), py_tuple(vec![py_str("should"), py_str("worth"), py_str("need"), py_str("old"), py_str("late"), py_str("adult"), py_str("any point")])])); __args.push(_asked(&[py_str("Yes, if you'll use it: the licence is much easier to get at 20 than at 40, and it opens up jobs and weekends. In a city with good buses and trains the cost rarely adds up, so decide on the next five years, not on the test.")])); __kws.extend(vec![("avoid".to_string(), __data37())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, pyrt::xrefs::xcall("knowledge.lookup", "entry", &[py_str("keep-a-diary"), py_tuple(vec![py_tuple(vec![py_str("keep a diary"), py_str("write a diary"), py_str("start a diary"), py_str("keeping a diary"), py_str("journal"), py_str("journaling")]), py_tuple(vec![py_str("worth"), py_str("should"), py_str("start"), py_str("keep"), py_str("help"), py_str("good"), py_str("why")])]), _asked(&[py_str("Worth two weeks, then decide. Three lines at night is enough to show you patterns you'd otherwise miss; if it starts to feel like homework, stop — a diary you dread is worse than none.")])]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-adult")); __args.push(py_tuple(vec![__data38(), __data39()])); __args.push(_asked(&[py_func("_start_instrument", |args: &[Py]| _start_instrument(args))])); __kws.extend(vec![("avoid".to_string(), __data40()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-how-long")); __args.push(py_tuple(vec![__data41(), py_tuple(vec![py_str("how long"), py_str("long"), py_str("months"), py_str("month"), py_str("weeks"), py_str("soon")])])); __args.push(py_func("_how_long_song", |args: &[Py]| _how_long_song(args))); __kws.extend(vec![("avoid".to_string(), __data42()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-practise")); __args.push(py_tuple(vec![py_tuple(vec![py_str("practise"), py_str("practice")]), py_tuple(vec![py_str("first"), py_str("start"), py_str("begin"), py_str("what")])])); __args.push(py_func("_practise_first", |args: &[Py]| _practise_first(args))); __kws.extend(vec![("avoid".to_string(), __data43())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("instrument-how-often")); __args.push(py_tuple(vec![py_tuple(vec![py_str("practise"), py_str("practice"), py_str("practising"), py_str("practicing")]), py_tuple(vec![py_str("how often"), py_str("often"), py_str("how many times"), py_str("times a week"), py_str("a day"), py_str("per day"), py_str("how much should i practise"), py_str("how much practice"), py_str("every day"), py_str("daily"), py_str("minutes")])])); __args.push(py_func("_how_often_at_it", |args: &[Py]| _how_often_at_it(args))); __kws.extend(vec![("avoid".to_string(), __data44())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cats-dogs")); __args.push(py_tuple(vec![py_str("cat"), py_str("cats")])); __args.push(py_tuple(vec![py_str("dog"), py_str("dogs")])); __args.push(py_func("_cats_or_dogs", |args: &[Py]| _cats_or_dogs(args))); __kws.extend(vec![("avoid".to_string(), __data45())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _either(args))(&__args) }, _either(&[py_str("tea-coffee"), py_tuple(vec![py_str("tea")]), py_tuple(vec![py_str("coffee")]), py_func("_tea_or_coffee", |args: &[Py]| _tea_or_coffee(args))]), _either(&[py_str("paper-ebook"), py_tuple(vec![py_str("paper"), py_str("paperback"), py_str("book"), py_str("books")]), py_tuple(vec![py_str("kindle"), py_str("ereader"), py_str("e-reader"), py_str("ebook"), py_str("ebooks")]), py_str("Paper for anything you want to remember, an e-reader for holidays and long queues. The real question is whether you're carrying it.")]), _either(&[py_str("tabs-spaces"), py_tuple(vec![py_str("tab"), py_str("tabs")]), py_tuple(vec![py_str("space"), py_str("spaces")]), py_str("Spaces: every editor shows them the same way, and nobody argues about how wide a tab is.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("android-iphone")); __args.push(py_tuple(vec![py_str("android")])); __args.push(py_tuple(vec![py_str("iphone"), py_str("apple")])); __args.push(py_str("iPhone if you want the simplest five years of updates; Android if you want a cheaper phone or more choice of hardware. Neither is a mistake.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("android"), py_str("iphone")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _either(args))(&__args) }, _either(&[py_str("city-country"), py_tuple(vec![py_str("city"), py_str("town")]), py_tuple(vec![py_str("countryside"), py_str("country"), py_str("village"), py_str("rural")]), py_str("City for a week, countryside for a month. If I had to choose one for good it would be the countryside, with a town within reach.")]), _either(&[py_str("beach-mountains"), py_tuple(vec![py_str("beach"), py_str("coast"), py_str("sea")]), py_tuple(vec![py_str("mountain"), py_str("mountains"), py_str("hills")]), py_str("Mountains: you can always find shade and a view, and the walk earns the dinner. Beaches win with small children, though — sand keeps them busy for hours.")]), _either(&[py_str("sweet-savoury"), py_tuple(vec![py_str("sweet"), py_str("sugary")]), py_tuple(vec![py_str("savoury"), py_str("salty")]), py_str("Savoury on a normal day — eggs, or toast with something salty — because it keeps you going until lunch. Sweet is for weekends and holidays.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("hot-cold-weather")); __args.push(py_tuple(vec![py_str("hot"), py_str("hot weather"), py_str("summer"), py_str("warm"), py_str("sunshine")])); __args.push(py_tuple(vec![py_str("cold"), py_str("cold weather"), py_str("winter"), py_str("freezing"), py_str("snow")])); __args.push(py_str("Cold, if I can wrap up: you can always add a layer, and a cold walk ends with a hot drink. Heat is harder to escape from.")); __kws.extend(vec![("avoid".to_string(), __data46())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _either(args))(&__args) }, _either(&[py_str("cinema-home"), py_tuple(vec![py_str("cinema"), py_str("big screen")]), py_tuple(vec![py_str("home"), py_str("streaming"), py_str("sofa"), py_str("netflix")]), py_str("Cinema for anything with scale or a big sound; home for comedies and anything you might pause. The cinema's real advantage is that nothing else asks for your attention for two hours.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rent-buy")); __args.push(py_tuple(vec![py_str("rent"), py_str("renting"), py_str("rented")])); __args.push(py_tuple(vec![py_str("buy"), py_str("buying"), py_str("mortgage"), py_str("owning")])); __args.push(py_str("Buy once you know where you want to be for five years or more and the mortgage is no more than a third of your take-home pay; rent until then, and keep the deposit growing. Renting is also the honest answer when the job might move.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("car"), py_str("van"), py_str("bike"), py_str("flat for the weekend"), py_str("holiday")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _either(args))(&__args) }, _either(&[py_str("run-walk"), py_tuple(vec![py_str("run"), py_str("running"), py_str("jog"), py_str("jogging")]), py_tuple(vec![py_str("walk"), py_str("walking")]), py_str("Walk if you are starting out or carrying an injury, run if you want the same fitness in half the time. Walking is easier to keep up for a year, and a year is what changes anything.")]), _either(&[py_str("walk-cycle"), py_tuple(vec![py_str("walk"), py_str("walking"), py_str("on foot")]), py_tuple(vec![py_str("cycle"), py_str("cycling"), py_str("bike"), py_str("bicycle")]), py_str("Cycle if the trip is more than about two miles and there is somewhere to lock a bike and change; walk if it is shorter and you want the thinking time. Under twenty minutes either way it comes down to which one you will still do in the rain.")]), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("electric-petrol")); __args.push(py_tuple(vec![py_str("electric"), py_str("electric car"), py_str("ev")])); __args.push(py_tuple(vec![py_str("petrol"), py_str("diesel"), py_str("hybrid")])); __args.push(py_str("Electric for the everyday car: cheaper to run, quieter, and charging at home beats a forecourt. Petrol still wins if you drive long distances weekly or cannot charge at home; a hybrid is the compromise that suits most people who cannot.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("won't"), py_str("wont"), py_str("charging point"), py_str("not charging"), py_str("battery"), py_str("repair"), py_str("garage"), py_str("lease")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _either(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("remote-work")); __args.push(py_tuple(vec![py_tuple(vec![py_str("remote work"), py_str("remote working"), py_str("working from home"), py_str("wfh"), py_str("hybrid working")]), py_tuple(vec![py_str("think"), py_str("good"), py_str("better"), py_str("worth"), py_str("opinion"), py_str("prefer"), py_str("like")])])); __args.push(_asked(&[py_str("Remote for the work that needs quiet, the office for the work that needs people. Two or three days in is the arrangement most people end up preferring, and it keeps you in the room when decisions get made.")])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("focus"), py_str("distracted"), py_str("routine"), py_str("procrastinating"), py_str("lonely"), py_str("isolated")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("decide-a-choice")); __args.push(py_tuple(vec![op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DECISION_SUBJECTS()) { let __unpk6879 = py_unpack(&__item.clone(), 2, None); let mut options: Py = __unpk6879[0].clone(); let mut _u: Py = __unpk6879[1].clone(); for __item in py_iter_or_raise(&options.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g_CAT_WORDS()), g_STUCK_SHAPES(), g_DECIDE_WORDS()])); __args.push(py_func("_deciding", |args: &[Py]| _deciding(args))); __kws.extend(vec![("ask".to_string(), op_add(&g_STUCK_SHAPES(), &py_tuple(vec![py_str("decide"), py_str("deciding"), py_str("decision"), py_str("whether"), py_str("choice")]))), ("avoid".to_string(), __data47())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cat-decide")); __args.push(py_tuple(vec![g_CAT_WORDS(), g_CAT_VERDICT_SHAPES()])); __args.push(py_func("_deciding", |args: &[Py]| _deciding(args))); __kws.extend(vec![("ask".to_string(), g_CAT_VERDICT_SHAPES()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data48()), ("guard".to_string(), py_func("_not_another_cats_question", |args: &[Py]| _not_another_cats_question(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    s_ENTRIES(__data22());
    let __iter49 = py_iter_or_raise(&g_ENTRIES());
    for __item in __iter49 {
    s__rule_of_thumb(__item.clone());
    if py_truthy(&Py::Bool(py_eq(&gattr(&g__rule_of_thumb(), "id"), &py_str("worth-an-appliance")))) {
    let _ = py_setattr(&g__rule_of_thumb(), "avoid", op_add(&gattr(&g__rule_of_thumb(), "avoid"), &py_tuple(vec![py_str("sealer"), py_str("sealers"), py_str("vacuum sealer"), py_str("vacuum sealers"), py_str("food sealer"), py_str("food sealers")])));
    }
    }
}
pub fn _worth_an_appliance(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut article: Py = Py::None;
    let mut buy: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut job: Py = Py::None;
    let mut names: Py = Py::None;
    let mut other: Py = Py::None;
    let mut part: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The buy-it-or-not rule for a machine the table has no row for, worded about its name.\n\n    The generic sentence was the answer to \"is a dishwasher worth it\", and a rule of thumb that\n    opens \"Worth it if you would use it three times a week\" never says which machine it is\n    talking about, so the reader has to check it is their question. The name in their line is\n    the cheapest way to say yes, this one.\n    ");
    names = py_tuple(vec![py_tuple(vec![py_str("robot vacuum"), py_str("robot vacuums"), py_str("robot hoover")]), py_tuple(vec![py_str("dishwasher"), py_str("dishwashers")]), py_tuple(vec![py_str("tumble dryer"), py_str("tumble dryers"), py_str("dryer")]), py_tuple(vec![py_str("washing machine"), py_str("washing machines")]), py_tuple(vec![py_str("slow cooker"), py_str("slow cookers")]), py_tuple(vec![py_str("bread maker"), py_str("bread makers")]), py_tuple(vec![py_str("blender"), py_str("blenders")]), py_tuple(vec![py_str("food processor"), py_str("food processors")]), py_tuple(vec![py_str("pressure cooker"), py_str("pressure cookers")]), py_tuple(vec![py_str("vacuum"), py_str("vacuum cleaner")]), py_tuple(vec![py_str("hoover"), py_str("hoovers")]), py_tuple(vec![py_str("freezer"), py_str("freezers")])]);
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_PART()) { let mut part: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &part.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_JOB()) { let mut job: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &job.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BUYING()) { let mut buy: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &buy.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("heat pump"))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = Py::Bool(py_contains(&said.clone(), &py_str("dryer"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("drier"))) } }; if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g__BUYING(), &py_tuple(vec![py_str("any good"), py_str("good"), py_str("worth"), py_str("worth it"), py_str("should i get"), py_str("should i buy"), py_str("get one")]))) { let mut buy: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &buy.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("hair dryer"), py_str("hair drier"), py_str("hairdryer"), py_str("nail dryer"), py_str("vacuum flask"), py_str("flask"), py_str("thermos")]));
    for __item in __iter1 {
    other = __item.clone();
    said = mcall(&said.clone(), "replace", &[other.clone(), py_str(" ")]);
    }
    thing = Py::None;
    let __iter2 = py_iter_or_raise(&names.clone());
    for __item in __iter2 {
    forms = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &form.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    thing = geti(&forms.clone(), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("vacuum"), py_str("hoover")]), &geti(&forms.clone(), &Py::Int(0))))) {
    thing = if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("vacuum")))) { py_str("vacuum cleaner") } else { py_str("hoover") };
    }
    break;
    }
    }
    article = if py_truthy(&{ let __b = thing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_str("aeiou"), &geti(&thing.clone(), &Py::Int(0)))) } }) { py_str("an") } else { py_str("a") };
    subject = if py_truthy(&thing.clone()) { { let mut __s = String::new(); __s.push_str(&fstr(&article.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); py_string(__s) } } else { py_str("it") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&if py_truthy(&thing.clone()) { mcall(&subject.clone(), "capitalize", &[]) } else { py_str("It") }, -1, "")); __s.push_str(" is worth it if you would use it three times a week or more and it saves a job you dislike; skip it otherwise, because it takes up a cupboard. Buy the cheap one first, because most of these are replaced by a better model you only learn you wanted after a year of using the first."); py_string(__s) };
    Py::None
}
pub fn _gadget_view(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut ask: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tip: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A tip for the machine they already have, or a verdict when it is the machine being judged.\n\n    \"what do you think of my new blender\" asks for a view, so it keeps the counter-space test.\n    \"i've got an air fryer\" asks nothing and is asking what to do with it, so it gets the one tip\n    that machine needs. A machine the table does not name gets the verdict, which is still true\n    of anything.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__VERDICT_ASK()) { let mut ask: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &ask.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__THE_VERDICT();
    }
    let __iter1 = py_iter_or_raise(&g__GADGET_TIPS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    forms = __unpk2[0].clone();
    tip = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &form.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return tip.clone();
    }
    }
    return g__THE_VERDICT();
    Py::None
}
pub fn _asked(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let _ = py_str("A reply, but only to a line that asks: a statement about the same subject passes.");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut full: Py = Py::None;
    let mut line: Py = Py::None;
    let mut marker: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    words = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = mcall(&g_EXPLAINS(), "match", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("your"), py_str("yourself")])))) } }) {
    return Py::None;
    }
pub fn __nested_said_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut full: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut line: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut marker: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    return if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&reply, &[memory.clone(), understood.clone()]) } else { reply.clone() };
    Py::None
}
    said = {{ let answer = answer.clone(); let full = full.clone(); let line = line.clone(); let marker = marker.clone(); let memory = memory.clone(); let reply = reply.clone(); let said = said.clone(); let text = text.clone(); let understood = understood.clone(); let words = words.clone(); py_func("said", move |args: &[Py]| { let mut full = vec![answer.clone(), full.clone(), line.clone(), marker.clone(), memory.clone(), reply.clone(), said.clone(), text.clone(), understood.clone(), words.clone()]; full.extend_from_slice(args); __nested_said_0(&full) }) }};
    if py_truthy(&Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question")))) {
    return py_call(&said, &[]);
    }
    words = { let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    line = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    full = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_INTENT()) { let mut marker: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &marker.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_INTENT()) { let mut marker: Py = __item.clone(); __out.push(Py::Bool(py_contains(&full.clone(), &marker.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_call(&said, &[]);
    }
    if py_truthy(&{ let __b = mcall(&g_OWN_OPINION(), "search", &[full.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("you"), py_str("your"), py_str("yourself"), py_str("yours")])))) } }) {
    return Py::None;
    }
    if py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_ASK_MARKERS())) {
    return py_call(&said, &[]);
    }
    if py_truthy(&{ let __b = op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_FIRST_PERSON()))) } }) {
    return py_call(&said, &[]);
    }
    return Py::None;
    Py::None
}
    answer = {{ let answer = answer.clone(); let reply = reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return answer.clone();
    Py::None
}
pub fn _is_a_choice(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line compares two things: a choice word, or a question that picks one.\n\n    \"im 34 and still renting, should i be buying\" names both halves and asks which, without any\n    \"or\" in it.\n    ");
    if py_truthy(&op_bitand(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &g_CHOICE())) {
    return Py::Bool(true);
    }
    text = op_add(&op_add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]), "lower", &[])), &py_str(" "));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHOICE_ASKS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
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
    let mut options: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut option: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the line names one of these things, in any of the ways people write it.");
    text = { let __b = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } };
    text = op_add(&op_add(&py_str(" "), &mcall(&text.clone(), "lower", &[])), &py_str(" "));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut option: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &option.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _either(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut id: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "id") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: id"))) };
    let mut a: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "a") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: a"))) };
    let mut b: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "b") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: b"))) };
    let mut reply: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut avoid: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "avoid") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    let mut answer: Py = Py::None;
    let _ = py_str("A pick between two everyday things, in either order, only when the line is a choice.\n\n    Both things have to be named and a word that makes it a comparison has to be there too\n    (\"tea or coffee\", \"coffee vs tea\", \"which is better\"), so a line that merely mentions both\n    (\"my cat keeps chasing the dog\", \"hotter in summer than in winter\") is not a question.\n\n    A choice outranks a plain entry that only noticed one half of it (\"should i get a cat or a\n    dog\" beats the kitten shopping list) through one point of bias, not by putting the choice\n    words in the key: \"or\" and \"which\" are in half the language, and an entry that matches them\n    is a match the answers lane then has to pass on.\n    ");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut a: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut answer: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut avoid: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut b: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut bias: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut id: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(7).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_is_a_choice(&[understood.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _names(&[understood.clone(), a.clone()]); if !py_truthy(&__b) { __b } else { _names(&[understood.clone(), b.clone()]) } }))) {
    return Py::None;
    }
    said = py_call(&_asked(&[reply.clone()]), &[memory.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&said.clone(), &Py::None))) {
    return said.clone();
    }
    words = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[g_CHOICE_IN_WORDS(), text.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_FIRST_PERSON()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_CHOICE_ASK_WORDS()))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_OWN_OPINION(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("you"), py_str("your"), py_str("yourself"), py_str("yours")])))) } }) {
    return Py::None;
    }
    return if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&reply, &[memory.clone(), understood.clone()]) } else { reply.clone() };
    Py::None
}
    answer = {{ let a = a.clone(); let answer = answer.clone(); let avoid = avoid.clone(); let b = b.clone(); let bias = bias.clone(); let id = id.clone(); let reply = reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![a.clone(), answer.clone(), avoid.clone(), b.clone(), bias.clone(), id.clone(), reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(id.clone()); __args.push(py_tuple(vec![a.clone(), b.clone()])); __args.push(answer.clone()); __kws.extend(vec![("avoid".to_string(), avoid.clone()), ("bias".to_string(), op_add(&bias.clone(), &Py::Int(1))), ("ask".to_string(), op_add(&call_builtin("tuple", &[call_builtin("sorted", &[g_CHOICE()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_CHOICE_ASKS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) };
    Py::None
}
pub fn _already_on_instruments(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last turn was already about learning an instrument.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::Bool(false);
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return { let __b = Py::Bool(py_contains(&said.clone(), &py_str("15–20 minutes"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("learning an instrument"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("four chords"))) } } } };
    Py::None
}
pub fn _on_drums(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when this line, or a line or two back, is about drums rather than a guitar.");
    if py_truthy(&_names(&[understood.clone(), g_DRUMS()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_names(&[understood.clone(), g_OTHER_INSTRUMENTS()])) {
    return Py::Bool(false);
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&turns.clone(), &op_neg(&g_INSTRUMENT_LOOKBACK()), &Py::None, &Py::None) } else { py_list(vec![]) }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str("practice pad")))) {
    return Py::Bool(true);
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("basic beat")))) {
    return Py::Bool(true);
    }
    if py_truthy(&_names(&[py_dict(vec![(py_str("text"), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]), g_DRUMS()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_names(&[py_dict(vec![(py_str("text"), call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)))]), g_OTHER_INSTRUMENTS()])) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _tea_or_coffee(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("Tea or coffee: a taste, unless the line asks which one is better for you.");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _names(&[understood.clone(), g_HEALTH_ASK()]) } }) {
    return py_str("Neither is bad for you, and the health difference is small: tea has about half the caffeine of coffee, both count towards your fluids, and green tea and filter coffee have the most evidence behind them. What matters far more is how much sugar and milk goes in.");
    }
    return py_str("Tea, most of the time: it's a better excuse to stop for five minutes. Coffee when you actually need to be awake in a hurry.");
    Py::None
}
pub fn _named_instrument(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut table: Py = Py::None;
    let _ = py_str("Which of the instrument answers this line is about, or None for one we do not cover.");
    let __iter1 = py_iter_or_raise(&mcall(&g_INSTRUMENTS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    table = __unpk2[1].clone();
    if py_truthy(&_names(&[understood.clone(), geti(&table.clone(), &py_str("names"))])) {
    return table.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _instrument_table(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut asked: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut table: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The instrument this conversation is about, the named one first, then the last answer's.");
    named = _named_instrument(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&named.clone(), &Py::None))) {
    return named.clone();
    }
    if py_truthy(&_names(&[understood.clone(), g_OTHER_INSTRUMENTS()])) {
    return g_INSTRUMENT_GENERIC();
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { slice_or_raise(&turns.clone(), &op_neg(&g_INSTRUMENT_LOOKBACK()), &Py::None, &Py::None) } else { py_list(vec![]) }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter2 = py_iter_or_raise(&mcall(&g_INSTRUMENTS(), "values", &[]));
    for __item in __iter2 {
    table = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &geti(&table.clone(), &py_str("hint"))))) {
    return table.clone();
    }
    }
    asked = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&asked.clone()) {
    let __iter3 = py_iter_or_raise(&mcall(&g_INSTRUMENTS(), "values", &[]));
    for __item in __iter3 {
    table = __item.clone();
    if py_truthy(&_names(&[py_dict(vec![(py_str("text"), asked.clone())]), geti(&table.clone(), &py_str("names"))])) {
    return table.clone();
    }
    }
    }
    }
    return g_INSTRUMENT_GENERIC();
    Py::None
}
pub fn _an_instrument_is_in_play(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this line, or something said a line or two back, names an instrument.\n\n    \"practice\" and \"practise\" are ordinary words, and the practise answer is about an\n    instrument — the embouchure, the bow, the hand position. Without this the answer went to\n    \"what can i cook to practice\" and to \"what should i practice first\" with no subject at\n    all, in a conversation about nothing musical. Three ways the instrument can be in play:\n    named here (\"what should i practise first on the violin\"), named in the answer just given\n    (the guitar answer says \"four chords\"), or named in a line just before it (\"im learning\n    the violin\", then \"what should i practice first\").\n    ");
    if py_truthy(&_names(&[understood.clone(), g_INSTRUMENT_WORDS()])) {
    return Py::Bool(true);
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = py_str(" ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&g_INSTRUMENT_LOOKBACK()), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug2 = op_add(&said.clone(), &op_add(&op_add(&op_add(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str(" ")), &call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), &py_str(" ")));
    said = __aug2;
    }
    }
    said = mcall(&said.clone(), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_INSTRUMENT_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _practise_first(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut table: Py = Py::None;
    let _ = py_str("What to practise in the first week, for the instrument this line (or the last one) named.");
    if py_truthy(&_on_drums(&[memory.clone(), understood.clone()])) {
    return py_str("Single strokes first, slow and even, ten minutes a day on a practice pad or a cushion: full strokes up to about a foot, then back down. Get that steady, then a basic beat — hi-hat on the eighths, snare on two and four, kick on one and three — and play along to a song you like. Speed comes last.");
    }
    table = _instrument_table(&[memory.clone(), understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&table.clone(), &g_INSTRUMENT_GENERIC())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_instrument_is_in_play(&[memory.clone(), understood.clone()]))) } }) {
    return Py::None;
    }
    return geti(&table.clone(), &py_str("practise"));
    Py::None
}
pub fn _drums_start(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut too_late: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "too_late") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: too_late"))) };
    let mut first: Py = Py::None;
    let _ = py_str("The drums version of the adult-beginner take: a beat in weeks, no kit to start.");
    first = if py_truthy(&too_late.clone()) { py_str("Not too late at all. ") } else { py_str("") };
    return op_add(&first.clone(), &py_str("Ten minutes a day on a practice pad gets you a steady basic beat in about six weeks: single strokes slow and even, then hi-hat on the eighths, snare on two and four, kick on one and three. Play along to songs you like before you buy a kit, and take one lesson at the start so your grip is right — a pad, or a cushion and a pair of sticks, is enough to begin."));
    Py::None
}
pub fn _start_instrument(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut start: Py = Py::None;
    let mut table: Py = Py::None;
    let _ = py_str("An adult who always wanted to learn: the first month, and the reassurance when it is asked.");
    if py_truthy(&_on_drums(&[memory.clone(), understood.clone()])) {
    if py_truthy(&_already_on_instruments(&[memory.clone()])) {
    return py_str("Then start this week: pad and sticks, ten minutes a day, one lesson at the start, and one song you like to play along to. Speed comes last — slow and even beats fast and ragged every time.");
    }
    return _drums_start(&[_names(&[understood.clone(), g_TOO_LATE()])]);
    }
    table = _instrument_table(&[memory.clone(), understood.clone()]);
    start = geti(&table.clone(), &py_str("start"));
    if py_truthy(&{ let __b = _already_on_instruments(&[memory.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names(&[understood.clone(), g_TOO_LATE()]))) } }) {
    return op_add(&mcall(&geti(&start.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&start.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return op_add(&op_add(&py_str("Not too late at all: adults pick up the theory faster than children, they just need shorter and more regular practice. "), &mcall(&geti(&start.clone(), &Py::Int(0)), "upper", &[])), &slice_or_raise(&start.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _how_long_song(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let _ = py_str("The follow-up after the instrument: how long until something recognisable.");
    if py_truthy(&_on_drums(&[memory.clone(), understood.clone()])) {
    return py_str("About six weeks of ten minutes a day for a steady basic beat, and three or four months before it holds together with a song at full speed. Play along to songs you like before you buy a kit.");
    }
    return geti(&_instrument_table(&[memory.clone(), understood.clone()]), &py_str("how_long"));
    Py::None
}
pub fn _how_often_at_it(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut table: Py = Py::None;
    let _ = py_str("How often to practise, for the instrument this line (or the last one) named.\n\n    A frequency question is not the practice-first question and not the how-long-until-a-song\n    question: it wants a rate, and the answer that carries the rate is the one thing the\n    instrument tables did not hold. \"how often should i practise the piano\" was the shrug and\n    \"how often should i practise guitar\" was the buy-a-guitar row, because \"guitar\" plus an ask\n    word was all that row needed.\n    ");
    if py_truthy(&_on_drums(&[memory.clone(), understood.clone()])) {
    return py_str("Ten minutes a day, five or six days a week, on a practice pad: single strokes slow and even first, then the basic beat played along to a song. The hands want the day off more than they want the extra twenty minutes.");
    }
    table = _instrument_table(&[memory.clone(), understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&table.clone(), &g_INSTRUMENT_GENERIC())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_an_instrument_is_in_play(&[memory.clone(), understood.clone()]))) } }) {
    return geti(&g_INSTRUMENT_GENERIC(), &py_str("how_often"));
    }
    return geti(&table.clone(), &py_str("how_often"));
    Py::None
}
pub fn _best_season(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut taste: Py = Py::None;
    let _ = py_str("A taste on the seasons, plus the way through it when the line says it cannot decide.");
    taste = py_str("Late spring for me: long evenings, everything green, and not hot enough to make excuses.");
    if py_truthy(&_names(&[understood.clone(), g_CANT_DECIDE()])) {
    return op_add(&taste.clone(), &py_str(" Since you cannot settle on one, pick by what you would do in it: autumn for walking, summer for eating outside, winter for staying in."));
    }
    return taste.clone();
    Py::None
}
pub fn _cats_or_dogs(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    if py_truthy(&_names(&[understood.clone(), g_OUT_ALL_DAY()])) {
    return py_str("A cat, if you're out all day: it sleeps through your shift and does not need a walk at seven in the morning. Get an adult cat rather than a kitten, and two if you are away a lot, so they have company.");
    }
    if py_truthy(&_names(&[understood.clone(), g_TRAIN_WORDS()])) {
    return g_TRAIN_COMPARE();
    }
    if py_truthy(&_names(&[understood.clone(), g_CAT_WINS()])) {
    return g_CAT_ON_A_DIMENSION();
    }
    if py_truthy(&_names(&[understood.clone(), g_DOG_WINS()])) {
    return g_DOG_ON_A_DIMENSION();
    }
    return py_str("Dogs, by a whisker: daft, straightforward and pleased to see you. A cat is the kinder choice if you're out all day, though.");
    Py::None
}
pub fn _says_one_of(args: &[Py]) -> Py {
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
    let mut options: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut option: Py = Py::None;
    let _ = py_str("True when the line holds one of these as a whole word.\n\n    `_names` matches a substring, which is right for a phrase like \"out all day\" and wrong for a\n    table of short words: \"apprenticeship\" holds \"rent\", so a line about an apprenticeship read\n    as a line about paying a landlord and got the move trade.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&options.clone()) { let mut option: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[option.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _not_another_cats_question(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line does not name another row's cat question.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__ANOTHER_CATS_QUESTION(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _deciding(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut options: Py = Py::None;
    let mut text: Py = Py::None;
    let mut trade: Py = Py::None;
    let _ = py_str("The trade that settles the choice the line names, or the general frame if it names none.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    text = { let __b = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]); if py_truthy(&__b) { __b } else { call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } };
    text = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    if py_truthy(&mcall(&g_A_NEW_ONE(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__WHAT_TO_BUY(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { _says_one_of(&[text.clone(), g_CAT_WORDS()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__TWO_OF_THEM(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bbetween\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_MADE_UP(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(cant|can't|cannot|whether|which|torn|unsure)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = _says_one_of(&[text.clone(), py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens")])]); if !py_truthy(&__b) { __b } else { _says_one_of(&[text.clone(), py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies")])]) } }) {
    return _cats_or_dogs(&[memory.clone(), understood.clone()]);
    }
    if py_truthy(&{ let __b = _says_one_of(&[text.clone(), geti(&g_TWO_WAYS(), &Py::Int(0))]); if !py_truthy(&__b) { __b } else { _says_one_of(&[text.clone(), geti(&g_TWO_WAYS(), &Py::Int(1))]) } }) {
    return Py::None;
    }
    if py_truthy(&_says_one_of(&[text.clone(), g_CAT_WORDS()])) {
    if py_truthy(&_says_one_of(&[text.clone(), g_CAT_EQUIPMENT()])) {
    return Py::None;
    }
    return g_CAT_TRADE();
    }
    if py_truthy(&_says_one_of(&[text.clone(), g_QUALIFIED()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_DECISION_SUBJECTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    options = __unpk2[0].clone();
    trade = __unpk2[1].clone();
    if py_truthy(&_says_one_of(&[text.clone(), options.clone()])) {
    return trade.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_normal_answer(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The sentence for the thing the line asked about, and nothing when it names no subject.");
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g__NORMAL_SUBJECTS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_table_bills_line(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the split in the line is a bill at a table, or asks whether it is rude.\n\n    A dinner's bill and a household's bills are the same words: \"split bills\" is one and\n    \"how do we split the bills\" is the other, and the plural is what people type for both. The\n    row this guards is the manners answer - even split, ask for separate bills - and it holds a\n    table's bill however the person asks it, including the how-to wording with no judgement word\n    in it (\"should i split the bill\", \"how do i split the bill\").\n\n    What it must not hold is every other thing that gets split. Its key is a split phrase and a\n    how or should, so once the phrase list grew to \"split bills\" and \"split the cost\" it\n    answered \"how do i split bills with my partner\", \"how should we split the cost of a villa\"\n    and \"how do i split the cost of a present\" with the even-split rule for a dinner, over the\n    couple's answer and the chip-in answer those lines have. A home word, or a person the money\n    is shared with, keeps those rows' own answers.\n    ");
    text = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    }
    if py_truthy(&mcall(&g__A_SHARED_HOME(), "search", &[text.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g__A_MANNERS_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_TABLE_BILL(), "search", &[text.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.opinions".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK_MARKERS" => Some(g_ASK_MARKERS()),
            "A_JOB_ON_IT" => Some(g_A_JOB_ON_IT()),
            "A_NEW_ONE" => Some(g_A_NEW_ONE()),
            "CANT_DECIDE" => Some(g_CANT_DECIDE()),
            "CAR_TRADE" => Some(g_CAR_TRADE()),
            "CAT_EQUIPMENT" => Some(g_CAT_EQUIPMENT()),
            "CAT_ON_A_DIMENSION" => Some(g_CAT_ON_A_DIMENSION()),
            "CAT_TRADE" => Some(g_CAT_TRADE()),
            "CAT_VERDICT_SHAPES" => Some(g_CAT_VERDICT_SHAPES()),
            "CAT_WINS" => Some(g_CAT_WINS()),
            "CAT_WORDS" => Some(g_CAT_WORDS()),
            "CHOICE" => Some(g_CHOICE()),
            "CHOICE_ASKS" => Some(g_CHOICE_ASKS()),
            "CHOICE_ASK_WORDS" => Some(g_CHOICE_ASK_WORDS()),
            "CHOICE_IN_WORDS" => Some(g_CHOICE_IN_WORDS()),
            "COURSE_TRADE" => Some(g_COURSE_TRADE()),
            "DECIDE_WORDS" => Some(g_DECIDE_WORDS()),
            "DECISION_SUBJECTS" => Some(g_DECISION_SUBJECTS()),
            "DISPOSAL" => Some(g_DISPOSAL()),
            "DOG_ON_A_DIMENSION" => Some(g_DOG_ON_A_DIMENSION()),
            "DOG_TRADE" => Some(g_DOG_TRADE()),
            "DOG_WINS" => Some(g_DOG_WINS()),
            "DRUMS" => Some(g_DRUMS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EXPLAINS" => Some(g_EXPLAINS()),
            "FAULT" => Some(g_FAULT()),
            "FIRST_PERSON" => Some(g_FIRST_PERSON()),
            "GUINEA_PIG_TRADE" => Some(g_GUINEA_PIG_TRADE()),
            "HAMSTER_TRADE" => Some(g_HAMSTER_TRADE()),
            "HEALTH_ASK" => Some(g_HEALTH_ASK()),
            "INSTRUMENTS" => Some(g_INSTRUMENTS()),
            "INSTRUMENT_GENERIC" => Some(g_INSTRUMENT_GENERIC()),
            "INSTRUMENT_LOOKBACK" => Some(g_INSTRUMENT_LOOKBACK()),
            "INSTRUMENT_WORDS" => Some(g_INSTRUMENT_WORDS()),
            "INTENT" => Some(g_INTENT()),
            "MADE_UP" => Some(g_MADE_UP()),
            "MOVE_TRADE" => Some(g_MOVE_TRADE()),
            "NOT_A_BUYING_ASK" => Some(g_NOT_A_BUYING_ASK()),
            "OTHER_INSTRUMENTS" => Some(g_OTHER_INSTRUMENTS()),
            "OUT_ALL_DAY" => Some(g_OUT_ALL_DAY()),
            "OWN_BUYING_ANSWER" => Some(g_OWN_BUYING_ANSWER()),
            "OWN_OPINION" => Some(g_OWN_OPINION()),
            "QUALIFIED" => Some(g_QUALIFIED()),
            "RABBIT_TRADE" => Some(g_RABBIT_TRADE()),
            "STUCK_SHAPES" => Some(g_STUCK_SHAPES()),
            "TASTE" => Some(g_TASTE()),
            "TOO_LATE" => Some(g_TOO_LATE()),
            "TRADES" => Some(g_TRADES()),
            "TRAIN_COMPARE" => Some(g_TRAIN_COMPARE()),
            "TRAIN_WORDS" => Some(g_TRAIN_WORDS()),
            "TWO_WAYS" => Some(g_TWO_WAYS()),
            "_ANOTHER_CATS_QUESTION" => Some(g__ANOTHER_CATS_QUESTION()),
            "_A_JOB" => Some(g__A_JOB()),
            "_A_MANNERS_WORD" => Some(g__A_MANNERS_WORD()),
            "_A_PART" => Some(g__A_PART()),
            "_A_SHARED_HOME" => Some(g__A_SHARED_HOME()),
            "_A_TABLE_BILL" => Some(g__A_TABLE_BILL()),
            "_BUYING" => Some(g__BUYING()),
            "_GADGET_TIPS" => Some(g__GADGET_TIPS()),
            "_NORMAL_ASK" => Some(g__NORMAL_ASK()),
            "_NORMAL_SUBJECTS" => Some(g__NORMAL_SUBJECTS()),
            "_NORMAL_WORDS" => Some(g__NORMAL_WORDS()),
            "_THE_VERDICT" => Some(g__THE_VERDICT()),
            "_TWO_OF_THEM" => Some(g__TWO_OF_THEM()),
            "_VERDICT_ASK" => Some(g__VERDICT_ASK()),
            "_WHAT_TO_BUY" => Some(g__WHAT_TO_BUY()),
            "_a_table_bills_line" => Some(py_func("_a_table_bills_line", |args: &[Py]| _a_table_bills_line(args))),
            "_already_on_instruments" => Some(py_func("_already_on_instruments", |args: &[Py]| _already_on_instruments(args))),
            "_an_instrument_is_in_play" => Some(py_func("_an_instrument_is_in_play", |args: &[Py]| _an_instrument_is_in_play(args))),
            "_asked" => Some(py_func("_asked", |args: &[Py]| _asked(args))),
            "_best_season" => Some(py_func("_best_season", |args: &[Py]| _best_season(args))),
            "_cats_or_dogs" => Some(py_func("_cats_or_dogs", |args: &[Py]| _cats_or_dogs(args))),
            "_deciding" => Some(py_func("_deciding", |args: &[Py]| _deciding(args))),
            "_drums_start" => Some(py_func("_drums_start", |args: &[Py]| _drums_start(args))),
            "_either" => Some(py_func("_either", |args: &[Py]| _either(args))),
            "_gadget_view" => Some(py_func("_gadget_view", |args: &[Py]| _gadget_view(args))),
            "_how_long_song" => Some(py_func("_how_long_song", |args: &[Py]| _how_long_song(args))),
            "_how_often_at_it" => Some(py_func("_how_often_at_it", |args: &[Py]| _how_often_at_it(args))),
            "_instrument_table" => Some(py_func("_instrument_table", |args: &[Py]| _instrument_table(args))),
            "_is_a_choice" => Some(py_func("_is_a_choice", |args: &[Py]| _is_a_choice(args))),
            "_named_instrument" => Some(py_func("_named_instrument", |args: &[Py]| _named_instrument(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_not_another_cats_question" => Some(py_func("_not_another_cats_question", |args: &[Py]| _not_another_cats_question(args))),
            "_on_drums" => Some(py_func("_on_drums", |args: &[Py]| _on_drums(args))),
            "_practise_first" => Some(py_func("_practise_first", |args: &[Py]| _practise_first(args))),
            "_rule_of_thumb" => Some(g__rule_of_thumb()),
            "_says_one_of" => Some(py_func("_says_one_of", |args: &[Py]| _says_one_of(args))),
            "_start_instrument" => Some(py_func("_start_instrument", |args: &[Py]| _start_instrument(args))),
            "_tea_or_coffee" => Some(py_func("_tea_or_coffee", |args: &[Py]| _tea_or_coffee(args))),
            "_the_normal_answer" => Some(py_func("_the_normal_answer", |args: &[Py]| _the_normal_answer(args))),
            "_worth_an_appliance" => Some(py_func("_worth_an_appliance", |args: &[Py]| _worth_an_appliance(args))),
            "asks_where_a_shop_answers" => Some(g_asks_where_a_shop_answers()),
            "entry" => Some(g_entry()),
            "the_book_is_a_reservation" => Some(g_the_book_is_a_reservation()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASK_MARKERS" => s_ASK_MARKERS(v),
            "A_JOB_ON_IT" => s_A_JOB_ON_IT(v),
            "A_NEW_ONE" => s_A_NEW_ONE(v),
            "CANT_DECIDE" => s_CANT_DECIDE(v),
            "CAR_TRADE" => s_CAR_TRADE(v),
            "CAT_EQUIPMENT" => s_CAT_EQUIPMENT(v),
            "CAT_ON_A_DIMENSION" => s_CAT_ON_A_DIMENSION(v),
            "CAT_TRADE" => s_CAT_TRADE(v),
            "CAT_VERDICT_SHAPES" => s_CAT_VERDICT_SHAPES(v),
            "CAT_WINS" => s_CAT_WINS(v),
            "CAT_WORDS" => s_CAT_WORDS(v),
            "CHOICE" => s_CHOICE(v),
            "CHOICE_ASKS" => s_CHOICE_ASKS(v),
            "CHOICE_ASK_WORDS" => s_CHOICE_ASK_WORDS(v),
            "CHOICE_IN_WORDS" => s_CHOICE_IN_WORDS(v),
            "COURSE_TRADE" => s_COURSE_TRADE(v),
            "DECIDE_WORDS" => s_DECIDE_WORDS(v),
            "DECISION_SUBJECTS" => s_DECISION_SUBJECTS(v),
            "DISPOSAL" => s_DISPOSAL(v),
            "DOG_ON_A_DIMENSION" => s_DOG_ON_A_DIMENSION(v),
            "DOG_TRADE" => s_DOG_TRADE(v),
            "DOG_WINS" => s_DOG_WINS(v),
            "DRUMS" => s_DRUMS(v),
            "ENTRIES" => s_ENTRIES(v),
            "EXPLAINS" => s_EXPLAINS(v),
            "FAULT" => s_FAULT(v),
            "FIRST_PERSON" => s_FIRST_PERSON(v),
            "GUINEA_PIG_TRADE" => s_GUINEA_PIG_TRADE(v),
            "HAMSTER_TRADE" => s_HAMSTER_TRADE(v),
            "HEALTH_ASK" => s_HEALTH_ASK(v),
            "INSTRUMENTS" => s_INSTRUMENTS(v),
            "INSTRUMENT_GENERIC" => s_INSTRUMENT_GENERIC(v),
            "INSTRUMENT_LOOKBACK" => s_INSTRUMENT_LOOKBACK(v),
            "INSTRUMENT_WORDS" => s_INSTRUMENT_WORDS(v),
            "INTENT" => s_INTENT(v),
            "MADE_UP" => s_MADE_UP(v),
            "MOVE_TRADE" => s_MOVE_TRADE(v),
            "NOT_A_BUYING_ASK" => s_NOT_A_BUYING_ASK(v),
            "OTHER_INSTRUMENTS" => s_OTHER_INSTRUMENTS(v),
            "OUT_ALL_DAY" => s_OUT_ALL_DAY(v),
            "OWN_BUYING_ANSWER" => s_OWN_BUYING_ANSWER(v),
            "OWN_OPINION" => s_OWN_OPINION(v),
            "QUALIFIED" => s_QUALIFIED(v),
            "RABBIT_TRADE" => s_RABBIT_TRADE(v),
            "STUCK_SHAPES" => s_STUCK_SHAPES(v),
            "TASTE" => s_TASTE(v),
            "TOO_LATE" => s_TOO_LATE(v),
            "TRADES" => s_TRADES(v),
            "TRAIN_COMPARE" => s_TRAIN_COMPARE(v),
            "TRAIN_WORDS" => s_TRAIN_WORDS(v),
            "TWO_WAYS" => s_TWO_WAYS(v),
            "_ANOTHER_CATS_QUESTION" => s__ANOTHER_CATS_QUESTION(v),
            "_A_JOB" => s__A_JOB(v),
            "_A_MANNERS_WORD" => s__A_MANNERS_WORD(v),
            "_A_PART" => s__A_PART(v),
            "_A_SHARED_HOME" => s__A_SHARED_HOME(v),
            "_A_TABLE_BILL" => s__A_TABLE_BILL(v),
            "_BUYING" => s__BUYING(v),
            "_GADGET_TIPS" => s__GADGET_TIPS(v),
            "_NORMAL_ASK" => s__NORMAL_ASK(v),
            "_NORMAL_SUBJECTS" => s__NORMAL_SUBJECTS(v),
            "_NORMAL_WORDS" => s__NORMAL_WORDS(v),
            "_THE_VERDICT" => s__THE_VERDICT(v),
            "_TWO_OF_THEM" => s__TWO_OF_THEM(v),
            "_VERDICT_ASK" => s__VERDICT_ASK(v),
            "_WHAT_TO_BUY" => s__WHAT_TO_BUY(v),
            "_rule_of_thumb" => s__rule_of_thumb(v),
            "asks_where_a_shop_answers" => s_asks_where_a_shop_answers(v),
            "entry" => s_entry(v),
            "the_book_is_a_reservation" => s_the_book_is_a_reservation(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.opinions", module_obj);
    pyrt::xrefs::reg_fn("knowledge.opinions", "_worth_an_appliance", py_func("_worth_an_appliance", |args: &[Py]| _worth_an_appliance(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_gadget_view", py_func("_gadget_view", |args: &[Py]| _gadget_view(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_asked", py_func("_asked", |args: &[Py]| _asked(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_is_a_choice", py_func("_is_a_choice", |args: &[Py]| _is_a_choice(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_names", py_func("_names", |args: &[Py]| _names(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_either", py_func("_either", |args: &[Py]| _either(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_already_on_instruments", py_func("_already_on_instruments", |args: &[Py]| _already_on_instruments(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_on_drums", py_func("_on_drums", |args: &[Py]| _on_drums(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_tea_or_coffee", py_func("_tea_or_coffee", |args: &[Py]| _tea_or_coffee(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_named_instrument", py_func("_named_instrument", |args: &[Py]| _named_instrument(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_instrument_table", py_func("_instrument_table", |args: &[Py]| _instrument_table(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_an_instrument_is_in_play", py_func("_an_instrument_is_in_play", |args: &[Py]| _an_instrument_is_in_play(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_practise_first", py_func("_practise_first", |args: &[Py]| _practise_first(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_drums_start", py_func("_drums_start", |args: &[Py]| _drums_start(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_start_instrument", py_func("_start_instrument", |args: &[Py]| _start_instrument(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_how_long_song", py_func("_how_long_song", |args: &[Py]| _how_long_song(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_how_often_at_it", py_func("_how_often_at_it", |args: &[Py]| _how_often_at_it(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_best_season", py_func("_best_season", |args: &[Py]| _best_season(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_cats_or_dogs", py_func("_cats_or_dogs", |args: &[Py]| _cats_or_dogs(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_says_one_of", py_func("_says_one_of", |args: &[Py]| _says_one_of(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_not_another_cats_question", py_func("_not_another_cats_question", |args: &[Py]| _not_another_cats_question(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_deciding", py_func("_deciding", |args: &[Py]| _deciding(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_the_normal_answer", py_func("_the_normal_answer", |args: &[Py]| _the_normal_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.opinions", "_a_table_bills_line", py_func("_a_table_bills_line", |args: &[Py]| _a_table_bills_line(args)));
    pyrt::xrefs::reg_access("knowledge.opinions", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.opinions", "ASK_MARKERS", g_ASK_MARKERS, s_ASK_MARKERS);
    pyrt::xrefs::reg_access("knowledge.opinions", "A_JOB_ON_IT", g_A_JOB_ON_IT, s_A_JOB_ON_IT);
    pyrt::xrefs::reg_access("knowledge.opinions", "A_NEW_ONE", g_A_NEW_ONE, s_A_NEW_ONE);
    pyrt::xrefs::reg_access("knowledge.opinions", "CANT_DECIDE", g_CANT_DECIDE, s_CANT_DECIDE);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAR_TRADE", g_CAR_TRADE, s_CAR_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAT_EQUIPMENT", g_CAT_EQUIPMENT, s_CAT_EQUIPMENT);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAT_ON_A_DIMENSION", g_CAT_ON_A_DIMENSION, s_CAT_ON_A_DIMENSION);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAT_TRADE", g_CAT_TRADE, s_CAT_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAT_VERDICT_SHAPES", g_CAT_VERDICT_SHAPES, s_CAT_VERDICT_SHAPES);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAT_WINS", g_CAT_WINS, s_CAT_WINS);
    pyrt::xrefs::reg_access("knowledge.opinions", "CAT_WORDS", g_CAT_WORDS, s_CAT_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "CHOICE", g_CHOICE, s_CHOICE);
    pyrt::xrefs::reg_access("knowledge.opinions", "CHOICE_ASKS", g_CHOICE_ASKS, s_CHOICE_ASKS);
    pyrt::xrefs::reg_access("knowledge.opinions", "CHOICE_ASK_WORDS", g_CHOICE_ASK_WORDS, s_CHOICE_ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "CHOICE_IN_WORDS", g_CHOICE_IN_WORDS, s_CHOICE_IN_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "COURSE_TRADE", g_COURSE_TRADE, s_COURSE_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "DECIDE_WORDS", g_DECIDE_WORDS, s_DECIDE_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "DECISION_SUBJECTS", g_DECISION_SUBJECTS, s_DECISION_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.opinions", "DISPOSAL", g_DISPOSAL, s_DISPOSAL);
    pyrt::xrefs::reg_access("knowledge.opinions", "DOG_ON_A_DIMENSION", g_DOG_ON_A_DIMENSION, s_DOG_ON_A_DIMENSION);
    pyrt::xrefs::reg_access("knowledge.opinions", "DOG_TRADE", g_DOG_TRADE, s_DOG_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "DOG_WINS", g_DOG_WINS, s_DOG_WINS);
    pyrt::xrefs::reg_access("knowledge.opinions", "DRUMS", g_DRUMS, s_DRUMS);
    pyrt::xrefs::reg_access("knowledge.opinions", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.opinions", "EXPLAINS", g_EXPLAINS, s_EXPLAINS);
    pyrt::xrefs::reg_access("knowledge.opinions", "FAULT", g_FAULT, s_FAULT);
    pyrt::xrefs::reg_access("knowledge.opinions", "FIRST_PERSON", g_FIRST_PERSON, s_FIRST_PERSON);
    pyrt::xrefs::reg_access("knowledge.opinions", "GUINEA_PIG_TRADE", g_GUINEA_PIG_TRADE, s_GUINEA_PIG_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "HAMSTER_TRADE", g_HAMSTER_TRADE, s_HAMSTER_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "HEALTH_ASK", g_HEALTH_ASK, s_HEALTH_ASK);
    pyrt::xrefs::reg_access("knowledge.opinions", "INSTRUMENTS", g_INSTRUMENTS, s_INSTRUMENTS);
    pyrt::xrefs::reg_access("knowledge.opinions", "INSTRUMENT_GENERIC", g_INSTRUMENT_GENERIC, s_INSTRUMENT_GENERIC);
    pyrt::xrefs::reg_access("knowledge.opinions", "INSTRUMENT_LOOKBACK", g_INSTRUMENT_LOOKBACK, s_INSTRUMENT_LOOKBACK);
    pyrt::xrefs::reg_access("knowledge.opinions", "INSTRUMENT_WORDS", g_INSTRUMENT_WORDS, s_INSTRUMENT_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "INTENT", g_INTENT, s_INTENT);
    pyrt::xrefs::reg_access("knowledge.opinions", "MADE_UP", g_MADE_UP, s_MADE_UP);
    pyrt::xrefs::reg_access("knowledge.opinions", "MOVE_TRADE", g_MOVE_TRADE, s_MOVE_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "NOT_A_BUYING_ASK", g_NOT_A_BUYING_ASK, s_NOT_A_BUYING_ASK);
    pyrt::xrefs::reg_access("knowledge.opinions", "OTHER_INSTRUMENTS", g_OTHER_INSTRUMENTS, s_OTHER_INSTRUMENTS);
    pyrt::xrefs::reg_access("knowledge.opinions", "OUT_ALL_DAY", g_OUT_ALL_DAY, s_OUT_ALL_DAY);
    pyrt::xrefs::reg_access("knowledge.opinions", "OWN_BUYING_ANSWER", g_OWN_BUYING_ANSWER, s_OWN_BUYING_ANSWER);
    pyrt::xrefs::reg_access("knowledge.opinions", "OWN_OPINION", g_OWN_OPINION, s_OWN_OPINION);
    pyrt::xrefs::reg_access("knowledge.opinions", "QUALIFIED", g_QUALIFIED, s_QUALIFIED);
    pyrt::xrefs::reg_access("knowledge.opinions", "RABBIT_TRADE", g_RABBIT_TRADE, s_RABBIT_TRADE);
    pyrt::xrefs::reg_access("knowledge.opinions", "STUCK_SHAPES", g_STUCK_SHAPES, s_STUCK_SHAPES);
    pyrt::xrefs::reg_access("knowledge.opinions", "TASTE", g_TASTE, s_TASTE);
    pyrt::xrefs::reg_access("knowledge.opinions", "TOO_LATE", g_TOO_LATE, s_TOO_LATE);
    pyrt::xrefs::reg_access("knowledge.opinions", "TRADES", g_TRADES, s_TRADES);
    pyrt::xrefs::reg_access("knowledge.opinions", "TRAIN_COMPARE", g_TRAIN_COMPARE, s_TRAIN_COMPARE);
    pyrt::xrefs::reg_access("knowledge.opinions", "TRAIN_WORDS", g_TRAIN_WORDS, s_TRAIN_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "TWO_WAYS", g_TWO_WAYS, s_TWO_WAYS);
    pyrt::xrefs::reg_access("knowledge.opinions", "_ANOTHER_CATS_QUESTION", g__ANOTHER_CATS_QUESTION, s__ANOTHER_CATS_QUESTION);
    pyrt::xrefs::reg_access("knowledge.opinions", "_A_JOB", g__A_JOB, s__A_JOB);
    pyrt::xrefs::reg_access("knowledge.opinions", "_A_MANNERS_WORD", g__A_MANNERS_WORD, s__A_MANNERS_WORD);
    pyrt::xrefs::reg_access("knowledge.opinions", "_A_PART", g__A_PART, s__A_PART);
    pyrt::xrefs::reg_access("knowledge.opinions", "_A_SHARED_HOME", g__A_SHARED_HOME, s__A_SHARED_HOME);
    pyrt::xrefs::reg_access("knowledge.opinions", "_A_TABLE_BILL", g__A_TABLE_BILL, s__A_TABLE_BILL);
    pyrt::xrefs::reg_access("knowledge.opinions", "_BUYING", g__BUYING, s__BUYING);
    pyrt::xrefs::reg_access("knowledge.opinions", "_GADGET_TIPS", g__GADGET_TIPS, s__GADGET_TIPS);
    pyrt::xrefs::reg_access("knowledge.opinions", "_NORMAL_ASK", g__NORMAL_ASK, s__NORMAL_ASK);
    pyrt::xrefs::reg_access("knowledge.opinions", "_NORMAL_SUBJECTS", g__NORMAL_SUBJECTS, s__NORMAL_SUBJECTS);
    pyrt::xrefs::reg_access("knowledge.opinions", "_NORMAL_WORDS", g__NORMAL_WORDS, s__NORMAL_WORDS);
    pyrt::xrefs::reg_access("knowledge.opinions", "_THE_VERDICT", g__THE_VERDICT, s__THE_VERDICT);
    pyrt::xrefs::reg_access("knowledge.opinions", "_TWO_OF_THEM", g__TWO_OF_THEM, s__TWO_OF_THEM);
    pyrt::xrefs::reg_access("knowledge.opinions", "_VERDICT_ASK", g__VERDICT_ASK, s__VERDICT_ASK);
    pyrt::xrefs::reg_access("knowledge.opinions", "_WHAT_TO_BUY", g__WHAT_TO_BUY, s__WHAT_TO_BUY);
    pyrt::xrefs::reg_access("knowledge.opinions", "_rule_of_thumb", g__rule_of_thumb, s__rule_of_thumb);
    pyrt::xrefs::reg_access("knowledge.opinions", "asks_where_a_shop_answers", g_asks_where_a_shop_answers, s_asks_where_a_shop_answers);
    pyrt::xrefs::reg_access("knowledge.opinions", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.opinions", "the_book_is_a_reservation", g_the_book_is_a_reservation, s_the_book_is_a_reservation);
}

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


thread_local! { static G_ABOUT_THE_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DUMMY_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KIND_OF_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LABEL_IN_FRONT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEAL_IN_THE_CHAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MEASURE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PERSON_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RANGE_AROUND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TABLE_BOOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TABLE_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_COUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_GROUP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHANGED_BARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHAT_JOINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTS_THE_THING_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_ASK_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_CORRECTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_OF_FOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROUP_OF_US: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOK_BACK_FOR_A_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAX_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MY_THREE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOW_THE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_LESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ONE_MORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTY_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHRASE_ENDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_AT_A_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_AT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RELATION_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AS_A_CHANGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TABLE_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THEY_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_IN_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WRONG_NUMBER_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIGGER_NUMBERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUP_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUP_POINTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUP_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUM_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SUBJECT_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ABOUT_THE_HOUSE() -> Py { ensure_init(); G_ABOUT_THE_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_HOUSE(v: Py) { ensure_init(); G_ABOUT_THE_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING() -> Py { ensure_init(); G_ASKING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING(v: Py) { ensure_init(); G_ASKING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COPULA() -> Py { ensure_init(); G_A_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COPULA(v: Py) { ensure_init(); G_A_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DUMMY_SUBJECT() -> Py { ensure_init(); G_A_DUMMY_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DUMMY_SUBJECT(v: Py) { ensure_init(); G_A_DUMMY_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FIGURE() -> Py { ensure_init(); G_A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FIGURE(v: Py) { ensure_init(); G_A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KIND_OF_PLACE() -> Py { ensure_init(); G_A_KIND_OF_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KIND_OF_PLACE(v: Py) { ensure_init(); G_A_KIND_OF_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LABEL_IN_FRONT() -> Py { ensure_init(); G_A_LABEL_IN_FRONT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LABEL_IN_FRONT(v: Py) { ensure_init(); G_A_LABEL_IN_FRONT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEAL_IN_THE_CHAT() -> Py { ensure_init(); G_A_MEAL_IN_THE_CHAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEAL_IN_THE_CHAT(v: Py) { ensure_init(); G_A_MEAL_IN_THE_CHAT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MEASURE_WORD() -> Py { ensure_init(); G_A_MEASURE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MEASURE_WORD(v: Py) { ensure_init(); G_A_MEASURE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PERSON_TAIL() -> Py { ensure_init(); G_A_PERSON_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PERSON_TAIL(v: Py) { ensure_init(); G_A_PERSON_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RANGE_AROUND() -> Py { ensure_init(); G_A_RANGE_AROUND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RANGE_AROUND(v: Py) { ensure_init(); G_A_RANGE_AROUND.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TABLE_BOOKED() -> Py { ensure_init(); G_A_TABLE_BOOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TABLE_BOOKED(v: Py) { ensure_init(); G_A_TABLE_BOOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TABLE_CUE() -> Py { ensure_init(); G_A_TABLE_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TABLE_CUE(v: Py) { ensure_init(); G_A_TABLE_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_COUNT_ASK() -> Py { ensure_init(); G_BARE_COUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_COUNT_ASK(v: Py) { ensure_init(); G_BARE_COUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_GROUP_ASK() -> Py { ensure_init(); G_BARE_GROUP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_GROUP_ASK(v: Py) { ensure_init(); G_BARE_GROUP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_NUMBER() -> Py { ensure_init(); G_BARE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_NUMBER(v: Py) { ensure_init(); G_BARE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_PEOPLE() -> Py { ensure_init(); G_BARE_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_PEOPLE(v: Py) { ensure_init(); G_BARE_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_BEDS() -> Py { ensure_init(); G_BEDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEDS(v: Py) { ensure_init(); G_BEDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED() -> Py { ensure_init(); G_CHANGED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED(v: Py) { ensure_init(); G_CHANGED.with(|c| *c.borrow_mut() = v); }
pub fn g_CHANGED_BARE() -> Py { ensure_init(); G_CHANGED_BARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHANGED_BARE(v: Py) { ensure_init(); G_CHANGED_BARE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHAT_JOINERS() -> Py { ensure_init(); G_CHAT_JOINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHAT_JOINERS(v: Py) { ensure_init(); G_CHAT_JOINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTED() -> Py { ensure_init(); G_CORRECTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTED(v: Py) { ensure_init(); G_CORRECTED.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTS_THE_THING_AFTER() -> Py { ensure_init(); G_COUNTS_THE_THING_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTS_THE_THING_AFTER(v: Py) { ensure_init(); G_COUNTS_THE_THING_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP() -> Py { ensure_init(); G_GROUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP(v: Py) { ensure_init(); G_GROUP.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_ASK() -> Py { ensure_init(); G_GROUP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_ASK(v: Py) { ensure_init(); G_GROUP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_ASK_YOU() -> Py { ensure_init(); G_GROUP_ASK_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_ASK_YOU(v: Py) { ensure_init(); G_GROUP_ASK_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_CORRECTED() -> Py { ensure_init(); G_GROUP_CORRECTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_CORRECTED(v: Py) { ensure_init(); G_GROUP_CORRECTED.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_NUMBER() -> Py { ensure_init(); G_GROUP_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_NUMBER(v: Py) { ensure_init(); G_GROUP_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_OF_FOUR() -> Py { ensure_init(); G_GROUP_OF_FOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_OF_FOUR(v: Py) { ensure_init(); G_GROUP_OF_FOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_GROUP_OF_US() -> Py { ensure_init(); G_GROUP_OF_US.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROUP_OF_US(v: Py) { ensure_init(); G_GROUP_OF_US.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MANY() -> Py { ensure_init(); G_HOW_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY(v: Py) { ensure_init(); G_HOW_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR() -> Py { ensure_init(); G_IRREGULAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR(v: Py) { ensure_init(); G_IRREGULAR.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOK_BACK_FOR_A_MEAL() -> Py { ensure_init(); G_LOOK_BACK_FOR_A_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOK_BACK_FOR_A_MEAL(v: Py) { ensure_init(); G_LOOK_BACK_FOR_A_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_MAX_PHRASE() -> Py { ensure_init(); G_MAX_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAX_PHRASE(v: Py) { ensure_init(); G_MAX_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_MY_THREE() -> Py { ensure_init(); G_MY_THREE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MY_THREE(v: Py) { ensure_init(); G_MY_THREE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_COUNT() -> Py { ensure_init(); G_NOT_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_COUNT(v: Py) { ensure_init(); G_NOT_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_COUNT() -> Py { ensure_init(); G_NOT_THE_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_COUNT(v: Py) { ensure_init(); G_NOT_THE_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOW_THE_NUMBER() -> Py { ensure_init(); G_NOW_THE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOW_THE_NUMBER(v: Py) { ensure_init(); G_NOW_THE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_AHEAD() -> Py { ensure_init(); G_NUMBER_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_AHEAD(v: Py) { ensure_init(); G_NUMBER_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_LESS() -> Py { ensure_init(); G_ONE_LESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_LESS(v: Py) { ensure_init(); G_ONE_LESS.with(|c| *c.borrow_mut() = v); }
pub fn g_ONE_MORE() -> Py { ensure_init(); G_ONE_MORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ONE_MORE(v: Py) { ensure_init(); G_ONE_MORE.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTY() -> Py { ensure_init(); G_PARTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTY(v: Py) { ensure_init(); G_PARTY.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTY_ASK() -> Py { ensure_init(); G_PARTY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTY_ASK(v: Py) { ensure_init(); G_PARTY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTY_THING() -> Py { ensure_init(); G_PARTY_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTY_THING(v: Py) { ensure_init(); G_PARTY_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PHRASE_ENDS() -> Py { ensure_init(); G_PHRASE_ENDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHRASE_ENDS(v: Py) { ensure_init(); G_PHRASE_ENDS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_AT_A_NUMBER() -> Py { ensure_init(); G_POINTS_AT_A_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT_A_NUMBER(v: Py) { ensure_init(); G_POINTS_AT_A_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_AT_THE_THING() -> Py { ensure_init(); G_POINTS_AT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT_THE_THING(v: Py) { ensure_init(); G_POINTS_AT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_RELATION_AHEAD() -> Py { ensure_init(); G_RELATION_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RELATION_AHEAD(v: Py) { ensure_init(); G_RELATION_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AS_A_CHANGE() -> Py { ensure_init(); G_SAID_AS_A_CHANGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AS_A_CHANGE(v: Py) { ensure_init(); G_SAID_AS_A_CHANGE.with(|c| *c.borrow_mut() = v); }
pub fn g_TABLE_MEAL() -> Py { ensure_init(); G_TABLE_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TABLE_MEAL(v: Py) { ensure_init(); G_TABLE_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_THEIRS() -> Py { ensure_init(); G_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEIRS(v: Py) { ensure_init(); G_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_THEY_HAVE() -> Py { ensure_init(); G_THEY_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THEY_HAVE(v: Py) { ensure_init(); G_THEY_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_IN_PHRASE() -> Py { ensure_init(); G_VERB_IN_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_IN_PHRASE(v: Py) { ensure_init(); G_VERB_IN_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_WRONG_NUMBER_AFTER() -> Py { ensure_init(); G_WRONG_NUMBER_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WRONG_NUMBER_AFTER(v: Py) { ensure_init(); G_WRONG_NUMBER_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DAY() -> Py { ensure_init(); G__A_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DAY(v: Py) { ensure_init(); G__A_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__BIGGER_NUMBERS() -> Py { ensure_init(); G__BIGGER_NUMBERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIGGER_NUMBERS(v: Py) { ensure_init(); G__BIGGER_NUMBERS.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUP_ASK_WORDS() -> Py { ensure_init(); G__GROUP_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUP_ASK_WORDS(v: Py) { ensure_init(); G__GROUP_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUP_POINTER() -> Py { ensure_init(); G__GROUP_POINTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUP_POINTER(v: Py) { ensure_init(); G__GROUP_POINTER.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUP_TAIL() -> Py { ensure_init(); G__GROUP_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUP_TAIL(v: Py) { ensure_init(); G__GROUP_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__NUM_WORD() -> Py { ensure_init(); G__NUM_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUM_WORD(v: Py) { ensure_init(); G__NUM_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__SUBJECT_AFTER() -> Py { ensure_init(); G__SUBJECT_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SUBJECT_AFTER(v: Py) { ensure_init(); G__SUBJECT_AFTER.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("How many of a thing the person has, kept as a number and given back when they ask.\n\n\"i have three brothers\" was filed only as one person (\"people\": [\"brother\"]) with the number\ndropped, so \"how many brothers do i have\" answered \"I don't know enough to answer that honestly\"\none turn after being told. Animals and children have readers of their own; this is every other\nthing a person counts - brothers, sisters, cars, grandkids, cats they are about to get - and the\nnoun is whatever they said, not a list, so a count nobody has thought about yet is kept too.\n\nNothing is answered unless the ask names a count the store holds, so a \"how many\" about the\nworld (\"how many hours should i study\") is left to the modules that know about it.\n");
    s_NUMBER_AHEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?P<n>a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2})\\s+(?:(?:more|other|elder|older|younger|little|lovely|grown|grown-up|step|half|foster|adopted|healthy|beautiful|new|old)\\s+){0,2}(?P<who>[a-z][a-z-]{2,})\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_THEY_HAVE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:i|we)(?:\\s+have|'ve|ve|\\s+got|\\s+have got|\\s+own)\\s+(?P<rest>[^.!?]{0,48})")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MY_THREE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\s+(?P<rest>(?:a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2})\\b[^.!?]{0,40})")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CORRECTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:no|nope|sorry|actually|wait|i mean|make that|oops|correction|sorry, no)\\b[\\s,]*(?:(?:make|change|put)\\s+(?:that|it)(?:\\s+to)?|its|it's|to)?\\s*((?:a couple of|a pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2})\\b[^.!?]{0,32})$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SUBJECT_AFTER(py_str("(?:i|you|we|they|he|she|it|there|that|this|these|those|any|some|my|your|our|their|his|her|its)\\b"));
    s_ASKING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\W*(?:(?:do|did|does|have|has|are|is|was|were|can|could|would|should|will)\\s+"), &g__SUBJECT_AFTER()), &py_str("|how|what|which|who|when|where|why)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_IRREGULAR(py_dict(vec![(py_str("children"), py_str("child")), (py_str("people"), py_str("person")), (py_str("men"), py_str("man")), (py_str("women"), py_str("woman")), (py_str("grandchildren"), py_str("grandchild")), (py_str("grandkids"), py_str("grandkid")), (py_str("nieces"), py_str("niece")), (py_str("nephews"), py_str("nephew")), (py_str("mice"), py_str("mouse")), (py_str("geese"), py_str("goose")), (py_str("feet"), py_str("foot")), (py_str("teeth"), py_str("tooth")), (py_str("oxen"), py_str("ox")), (py_str("wives"), py_str("wife")), (py_str("lives"), py_str("life")), (py_str("knives"), py_str("knife")), (py_str("shelves"), py_str("shelf")), (py_str("wolves"), py_str("wolf")), (py_str("halves"), py_str("half"))]));
    s_THEIRS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("cat cats dog dogs puppy puppies kitten kittens rabbit rabbits bunny\nbunnies hamster hamsters fish horse horses pony ponies parrot parrots pet pets animal animals\nkid kids child children baby babies son sons daughter daughters"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_A_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("minutes minute hours hour days day weeks week months month years year\no'clock pounds pound quid euros euro dollars dollar pence pennies percent times time\nyears-old oclock weeks-old months-old miles mile kilometres km kilos kilo pounds-weight\nstone stones stones-weight left right quarter quarters third fourth fifth half halves dozen\nlitres litre pints pint ounces ounce grams kilo kilos"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GROUP_NUMBER(py_str("a couple of|a pair of|couple of|pair of|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2}"));
    s__GROUP_TAIL(py_str("(?=\\s*(?:$|[,.!?;:]|\\b(?:here|now|at home|in the house|in the flat|in the home|in total|altogether|total|and|but|so)\\b))"));
    s_GROUP_OF_US({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?:there|it)\\s*(?:'s|s|'re|re|are|is|'ll|ll|will|would)|we\\s*(?:'re|re|are|'ll|ll|will))\\s+(?:(?:going to be|gonna be|will be|would be|should be|be)\\s+)?(?:(?:just|only|now|still|currently|actually)\\s+){0,2}(?:the\\s+)?(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\s+of us\\b")), &g__GROUP_TAIL()), &py_str("|\\b(?:we\\s*(?:'re|re|are)|there\\s*(?:'s|s|'re|re|are|is))\\s+(?:(?:just|only|now|still|currently|actually)\\s+){0,2}(?P<n2>")), &g_GROUP_NUMBER()), &py_str(")\\s+(?:people|persons?)\\b")), &g__GROUP_TAIL()), &py_str("|^\\W*(?:(?:and|so|well|ok|okay|oh|hmm)\\b[,.]?\\s+)*(?:(?:just|only)\\s+)?(?:the\\s+)?(?P<n3>")), &g_GROUP_NUMBER()), &py_str(")\\s+of us\\b")), &g__GROUP_TAIL())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GROUP_CORRECTED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("^\\W*(?:no|nope|sorry|actually|wait|i mean|make it|make that|hold on|hang on|correction)\\b[^.!?]{0,12}?(?:the\\s+)?(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\s+of us\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GROUP_OF_FOUR({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:family|household)\\s+of\\s+(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\b(?!\\s+[a-z])"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__GROUP_ASK_WORDS(py_str("(?:are|is|were|was|there|here|now|altogether|total|in all|at home|it|that|again|then|did i say|did we say|do you remember|remind me|in the house|in the flat|in the home|in my (?:house|flat|home|household)|in our (?:house|flat|home|household)|live here|living here|live in the (?:house|flat|home)|living in the (?:house|flat|home))"));
    s__GROUP_POINTER(py_str("(?:is it|was it|were they|is that|was that|again|then|did i say|did we say|i say)"));
    s__GROUP_TAIL(op_add(&op_add(&g__GROUP_ASK_WORDS(), &py_str("|")), &g__GROUP_POINTER()));
    s_GROUP_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\bhow many of us(?:\\s+(?:"), &g__GROUP_TAIL()), &py_str("))*\\s*[?.!]*$|\\bhow many people(?:\\s+(?:")), &g__GROUP_TAIL()), &py_str("))*\\s*[?.!]*$|\\bhow (?:big|large) (?:is|are) (?:my|our|the) (?:family|household)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GROUP_ASK_YOU({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("\\bhow many of (?:you|u)(?:\\s+(?:"), &g__GROUP_ASK_WORDS()), &py_str("|")), &g__GROUP_POINTER()), &py_str("))*\\s*[?.!]*$|\\bhow many of (?:you|u) (?:live|living|are living)(?:\\s+(?:there|here|at home|now|altogether))?\\s*[?.!]*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_GROUP_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many (?:are|is|were|was) (?:coming|going|expected|there)(?:\\s+(?:then|now|again|altogether|in total))?\\s*[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BARE_COUNT_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many\\s+(?:was|is|were|are)\\s+(?:it|that|there)(?:\\s+(?:again|then|now|altogether|in total))?\\s*[?.!]*$|\\bhow many again\\b\\s*[?.!]*$|\\A\\W*how many\\W*[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GROUP(py_str("us"));
    s_TABLE_MEAL(py_str("roast|dinner|supper|tea|lunch|breakfast|meal|barbecue|bbq|barbeque|party|spread|buffet|feast|brunch|carvery"));
    s__A_DAY(py_str("monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|tues|tue|weds|wed|thurs|thur|thu|fri|sat|sun|weekend|tonight|tomorrow|today"));
    s_BARE_NUMBER(op_add(&g_GROUP_NUMBER(), &py_str("|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|")));
    let __aug1 = op_add(&g_BARE_NUMBER(), &py_str("twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|a hundred|one hundred|\\d{1,3}"));
    s_BARE_NUMBER(__aug1);
    s_PARTY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:for|having|expecting|feeding|cooking for|catering for|a table for|table for|booking for|booked for|party for|dinner for|lunch for|tea for|supper for|seats? for|invited|inviting)\\s+(?:the\\s+|just\\s+|only\\s+)?(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\s+(?:people|persons?|guests?|adults?|heads?|folk|friends|kids|children)\\b|\\b(?P<n2>")), &g_GROUP_NUMBER()), &py_str(")\\s+(?:people|persons?|guests?|adults?|heads?|friends)\\s+(?:coming|over|round|for|at|on)\\b|\\b(?P<n4>")), &g_GROUP_NUMBER()), &py_str(")\\s+of us\\s+(?:(?:coming|over|round)\\b|for\\s+(?!(?:a|an|the|good|now|once|ages|keeps|ever)\\b)[a-z])|\\b(?P<n7>")), &g_GROUP_NUMBER()), &py_str(")\\s+of us\\s+(?:on|this|next)\\s+(?:the\\s+)?(?:")), &g__A_DAY()), &py_str(")\\b|\\b(?:a table for|table for|booking for|booked for|reservation for)\\s+(?P<n3>")), &g_GROUP_NUMBER()), &py_str(")\\b(?!(?:\\s*(?:minutes?|hours?|days?|weeks?|months?|years?|courses?)\\b))|\\b(?:cooking for|catering for|feeding|having over|having round)\\s+(?P<n6>")), &g_GROUP_NUMBER()), &py_str(")\\b(?!(?:\\s*(?:minutes?|hours?|days?|weeks?|months?|years?|courses?|of them)\\b))|\\b(?:")), &g_TABLE_MEAL()), &py_str(")\\b[^.?!]{0,24}?\\bfor\\s+(?:the\\s+|just\\s+|only\\s+)?(?P<n5>")), &g_BARE_NUMBER()), &py_str(")\\b(?!(?:\\s*(?:minutes?|hours?|days?|weeks?|months?|years?|courses?|more|less|pounds?|quid|euros?|dollars?|kg|kilos?|grams?|litres?|pints?|miles?|metres?|o'clock)\\b))"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PARTY_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many (?:people|persons?|guests?|heads?|adults?)\\b\\s*(?:$|[,.!?;:]|\\b(?:again|then|though|is it|is that|are coming|coming|was it|was that|you think|do you think|do i need|do i cook|am i cooking|we having|did i say|did we say)\\b)|\\bhow many (?:are|is) (?:coming|it for|we having|we cooking for|we expecting)\\b|\\bhow many did i say\\b|\\bhow many (?:did|have) (?:i|we) (?:book|booked|reserve|reserved|ask for|asked for)\\b|\\bhow many is (?:the|that|this|my|our) (?:table|booking|reservation) for\\b|\\bhow many (?:am|are|was|were) (?:i|we) (?:cooking|baking|making|catering|feeding|preparing|planning|doing)\\b[^.?!]{0,18}?\\bfor\\b|\\bhow many (?:do|did|will) (?:i|we) (?:cook|bake|make|cater|feed|prepare|plan) for\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHANGED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:actually|instead|now|sorry|hang on|one more|another|extra|make that|no)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHANGED_BARE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:make it|make that|put it at|change it to|change to|actually|sorry|no|instead|lets say|let's say|now)\\b[\\s,]*(?:it\\s+)?(?:to\\s+|at\\s+)?(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\b(?:\\s*,?\\s*(?:people|persons?|guests?|heads?|adults?|of us|now|instead|please|thanks|after all))*\\s*[.!]*$"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOW_THE_NUMBER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:we|there)\\s*(?:'s|'re|re|are|is|s)?\\s+(?:now\\s+)?(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\s+(?:of us\\s+)?now\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ONE_MORE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:and\\s+|oh\\s+|plus\\s+)?(?:one|1)\\s+more(?:\\s+(?:is\\s+coming|is\\s+coming\\s+now|coming|now|person|guest|guest\\s+is\\s+coming|of\\s+us|of\\s+them))?\\s*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ONE_LESS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^\\W*(?:and\\s+|oh\\s+)?(?:one|1)(?:\\s+of\\s+(?:them|the\\s+guests?|the\\s+group))?\\s+(?:less|cant\\s+come|cannot\\s+come|can't\\s+come|isnt\\s+coming|isn't\\s+coming|is\\s+not\\s+coming|has\\s+dropped\\s+out|dropped\\s+out|cant\\s+make\\s+it|can't\\s+make\\s+it)(?:\\s+(?:now|after\\s+all|anymore|any\\s+more))?\\s*[.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ABOUT_THE_HOUSE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:at home|in the house|in the flat|in the home|live|lives|living|living here|household|family|house)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTS_AT_A_NUMBER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:again|was it|was that|is it for|did i say|did we say|do you remember|remind me)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PARTY_THING(py_str("person"));
    s_LOOK_BACK_FOR_A_MEAL(Py::Int(6));
    s_BARE_PEOPLE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:(?:about|around|roughly|maybe|probably|just|only|some|like)\\s+)?(?P<n>"), &g_BARE_NUMBER()), &py_str(")\\s+(?:people|persons?|guests?|adults?|heads?|folk)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MEAL_IN_THE_CHAT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &g_TABLE_MEAL()), &py_str("|cake|cakes|bake|baking|cooking|cook|menu|scones?|birthday|buffet|canapes|finger food)\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PHRASE_ENDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("called named of and but with for to in at on from by near who that\nwhich because so then when if about today tomorrow yesterday tonight now ago already still here\nthere not no only just instead rather than"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHAT_JOINERS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("n nd"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_MAX_PHRASE(Py::Int(3));
    s_VERB_IN_PHRASE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("live lives are is was were be been am come comes go goes went\nhave has had do does did get gets got make makes made take takes need needs want wants eat eats\nate cost costs work works stays stay move moves play plays run runs see saw know knows think\nthinks feel feels look looks seem seems sit sits speak speaks grow grows cost"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_RELATION_AHEAD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("grand"), py_str("step"), py_str("half"), py_str("foster"), py_str("adopted"), py_str("adoptive"), py_str("ex"), py_str("surrogate")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_PERSON_TAIL(py_tuple(vec![py_str("child"), py_str("kid"), py_str("son"), py_str("daughter"), py_str("aunt"), py_str("uncle"), py_str("niece"), py_str("nephew"), py_str("parent"), py_str("mother"), py_str("father"), py_str("grandma"), py_str("grandad"), py_str("granddaughter"), py_str("grandson"), py_str("gran"), py_str("nan"), py_str("dad"), py_str("mum")]));
    s_BEDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\s*(?P<join>-?)\\s*bed(?:room)?s?\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_KIND_OF_PLACE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("house home flat place bungalow maisonette semi terrace terraced\ncottage apartment rental rented bedsit studio pad maisonette maisonettes"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_MEASURE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("bed beds bedroom bedrooms"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NUM_WORD(py_str("(?:a couple of|a pair of|one|two|three|four|five|six|seven|eight|nine|ten|\\d{1,3})"));
    s_NOT_A_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("degrees degree percent miles mile km kms kilometres kilometers metres\nmeters cm mm kg kgs lbs lb oz litres liters ml celsius fahrenheit mph kph"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_RANGE_AROUND({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\bbetween\\b[^.!?]{0,14}?"), &g__NUM_WORD()), &py_str("[^.!?]{0,10}?\\band\\b|")), &g__NUM_WORD()), &py_str("\\s*(?:-|\\u2013|to|or)\\s*")), &g__NUM_WORD())); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BIGGER_NUMBERS(py_dict(vec![(py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50)), (py_str("sixty"), Py::Int(60)), (py_str("seventy"), Py::Int(70)), (py_str("eighty"), Py::Int(80)), (py_str("ninety"), Py::Int(90)), (py_str("a hundred"), Py::Int(100)), (py_str("one hundred"), Py::Int(100)), (py_str("a dozen"), Py::Int(12)), (py_str("dozen"), Py::Int(12))]));
    s_A_TABLE_BOOKED({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:book(?:ed|ing|ings)?|reserv(?:ed|ing|ation|ations))\\b[^.]{0,24}\\btables?\\b|\\btables?\\b[^.]{0,12}\\bfor\\s+(?:\\d+|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b|\\b(?:book(?:ed|ing)?|reserved)\\b\\s+(?:for\\s+)?(?:\\d+|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_TABLE_CUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:people|persons?|guests?|adults?|heads?|folk|friends|kids|children)\\b[^.]{0,16}\\b(?:coming|over|round)\\b|\\b(?:for|having|expecting|feeding|catering for|cooking for|invited|inviting)\\s+(?:the\\s+|just\\s+|only\\s+)?(?:"), &g_GROUP_NUMBER()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_FIGURE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?P<n>"), &g_GROUP_NUMBER()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_SAID_AS_A_CHANGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:actually|instead|now|sorry|wait|hold on|make (?:it|that)|change|put it at|no|nope|correction|then|after all|not|still|only)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_POINTS_AT_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bof (?:them|those|these|it)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_LABEL_IN_FRONT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("number no nos num bus train flight room flat gate platform\nseat table lane door channel code pin model version series episode chapter page line half\nquarter past o'clock oclock platform route size step floor grade level class year"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COUNTS_THE_THING_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("of now then instead too as well please thanks again\nstill only just left to go more another"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_DUMMY_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("its it's it thats that's theres there's"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_COPULA({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?P<subj>[a-z][a-z'-]*)\\s+(?:is|are|was|were|be|being)\\s*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_WRONG_NUMBER_AFTER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\W*\\b(?:not|instead of|rather than|no longer|as opposed to)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_HOW_MANY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bhow many (?P<what>[a-z-]+(?:\\s+[a-z-]+){0,2}?)(?:\\s+(?:and|n|nd)\\s+(?P<also>[a-z-]+(?:\\s+[a-z-]+){0,2}?))?\\s*(?:(?:do|did) (?:i|we) (?:have|got|own)|(?:have|had) (?:i|we) got|(?:do|did) (?:i|we) own|are there|have we|again|then|now)?\\s*[?.!]*$")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_THE_COUNT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:should|could|would|might|may|can|will)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
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
    let mut n: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "n") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: n"))) };
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The number as a word up to twelve, as digits after that.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::facts::g_PET_COUNT_WORDS(), "items", args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&value.clone(), &n.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&word.clone(), &py_str(" "))) } }) {
    return word.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(n.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _head(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The thing a count is about: the last word of the phrase before it stops.\n\n    \"two flat tyres\" -> \"tyres\", \"two spare bedrooms\" -> \"bedrooms\", \"5 missed calls\" -> \"calls\",\n    but \"two cats called milo and tom\" -> \"cats\" and \"three jars of jam\" -> \"jars\". An adjective\n    nobody listed (\"spare\", \"missed\", \"flat\") used to become the thing counted, so \"i have two\n    flat tyres\" filed a count of flats and \"how many flats do i have\" answered \"You've got two\n    flats.\" Reading the head of the phrase fixes that without a longer list of adjectives.\n    ");
    phrase = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }, &Py::None, &g_MAX_PHRASE(), &Py::None));
    for __item in __iter1 {
    word = __item.clone();
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str("’"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "isalpha", args))(&__args) })); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PHRASE_ENDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CHAT_JOINERS(), &word.clone())) } } } }) {
    break;
    }
    if py_truthy(&Py::Bool(py_contains(&crate::memory::facts::g_PET_COUNT_WORDS(), &word.clone()))) {
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_VERB_IN_PHRASE(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ly"), py_str("ing")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) } } }) {
    break;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "append", args))(&__args) };
    }
    return phrase.clone();
    Py::None
}
pub fn _the_relation(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The singular thing a phrase counts, with the word that says which relation it is.");
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&phrase.clone(), &op_neg(&Py::Int(1)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)) } }) {
    return thing.clone();
    }
    before = geti(&phrase.clone(), &op_neg(&Py::Int(2)));
    if py_truthy(&Py::Bool(py_contains(&g_RELATION_AHEAD(), &before.clone()))) {
    return op_add(&op_add(&before.clone(), &py_str(" ")), &thing.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&before.clone(), &py_str("great"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_A_PERSON_TAIL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "endswith", args))(&__args) } } }) {
    return op_add(&py_str("great "), &thing.clone());
    }
    return thing.clone();
    Py::None
}
pub fn _thing_of(args: &[Py]) -> Py {
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
    let _ = py_str("The singular of the thing the phrase asks about: its head, read the way it is filed.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _head(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relation(args))(&__args) };
    Py::None
}
pub fn _thing(args: &[Py]) -> Py {
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
    let _ = py_str("The singular of a counted noun, or \"\" when the word cannot be a count of things.");
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("-")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(3)) } } } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&g_IRREGULAR(), &word.clone()))) {
    return geti(&g_IRREGULAR(), &word.clone());
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ies")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(4)) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ches"), py_str("shes"), py_str("sses"), py_str("xes"), py_str("zes")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("ss"), py_str("us"), py_str("is")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) })) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _beds(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut after: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How many bedrooms a phrase says the place has: \"a 3 bed house\" -> 3, or 0.\n\n    \"3 bed\", \"three-bed\" and \"3 bedroom\" all say how many bedrooms the place has. A line that\n    only says the number of bedrooms (\"we have three bedrooms\") is a count of bedrooms already,\n    so filing it here as well changes nothing.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }, "split", args))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BEDS(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    number = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "isdigit", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::facts::g_PET_COUNT_WORDS(), "get", args))(&__args) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp21: Py = number.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp21)) { op_le(&__cmp21, &Py::Int(20)) } else { Py::Bool(false) } }))) {
    continue;
    }
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&line.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) }, "split", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("join")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "strip", args))(&__args) }, &py_str("-"))); if py_truthy(&__b) { __b } else { { let __b = after.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_A_KIND_OF_PLACE(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }) {
    return number.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _counted(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut window: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("{thing: number} for the counts in one possession phrase.");
    found = py_dict(vec![]);
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str(" "), &rest.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NUMBER_AHEAD(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    window = slice_or_raise(&rest.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(0)); __args.push(op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::Int(16))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::Int(12)), &Py::None);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(window.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_RANGE_AROUND(), "search", args))(&__args) }) {
    continue;
    }
    tail = slice_or_raise(&rest.clone(), &op_sub(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("who")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_contains(&tail.clone(), &py_str(",")))) {
    tail = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(",")); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tail.clone(), "split", args))(&__args) }, &Py::Int(0));
    }
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(tail.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _head(args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrase.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    continue;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_relation(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THEIRS(), &thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THEIRS(), &geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1))))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_MEASURE_WORD(), &geti(&phrase.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }, "lower", args))(&__args) };
    number = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "isdigit", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::facts::g_PET_COUNT_WORDS(), "get", args))(&__args) } };
    if py_truthy(&op_gt(&number.clone(), &Py::Int(0))) {
    let _ = py_setitem(&found.clone(), &thing.clone(), number.clone());
    }
    }
    return found.clone();
    Py::None
}
pub fn _said_number(args: &[Py]) -> Py {
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
    let _ = py_str("The number a phrase names: \"8\", \"eight\", \"twenty\", \"a couple of\". 0 when it names none.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "strip", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "isdigit", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::memory::facts::g_PET_COUNT_WORDS(), "get", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(found.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return found.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(Py::Int(0)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BIGGER_NUMBERS(), "get", args))(&__args) };
    Py::None
}
pub fn _the_group(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("How many people the line says are in their group, or 0 when it says nothing of the sort.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_GROUP_OF_US(), g_GROUP_OF_FOUR(), g_GROUP_CORRECTED()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n2")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n3")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_number(args))(&__args) };
    if py_truthy(&op_gt(&number.clone(), &Py::Int(0))) {
    return number.clone();
    }
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _the_party(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let _ = py_str("How many people the line says are coming, or 0 when it says nothing of the sort.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PARTY(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n2")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n3")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n4")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n5")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n6")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n7")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) } } } } } } } } } } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_number(args))(&__args) };
    if py_truthy(&op_gt(&number.clone(), &Py::Int(0))) {
    return number.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _the_chat_lines(args: &[Py]) -> Py {
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
    let mut lines: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The lines of this chat a count is read from: the plan it holds and its last turns.");
    lines = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]);
    let __aug1 = op_add(&lines.clone(), &slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }, &op_neg(&g_LOOK_BACK_FOR_A_MEAL()), &Py::None, &Py::None));
    lines = __aug1;
    return lines.clone();
    Py::None
}
pub fn _the_chat_feeds_people(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this chat has said what the people are for: a meal, a cake, a party being given.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_chat_lines(args))(&__args) }) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MEAL_IN_THE_CHAT(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_chat_sets_a_table(args: &[Py]) -> Py {
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
    let mut lines: Py = Py::None;
    let _ = py_str("Whether this chat has a table for people at all: a meal said, a booking, or people coming.");
    lines = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_chat_lines(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_MEAL_IN_THE_CHAT(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TABLE_BOOKED(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_TABLE_CUE(), "search", args))(&__args) } } } } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_bare_people(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let _ = py_str("How many people a line says with no word for what they are for, or 0.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_PEOPLE(), "finditer", args))(&__args) });
    for __item in __iter1 {
    match_ = __item.clone();
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_number(args))(&__args) };
    if py_truthy(&op_gt(&number.clone(), &Py::Int(0))) {
    return number.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _the_number_put_right(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut held: Py = Py::None;
    let mut held_party: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("The number a line puts right, when the chat already holds the number at the table.\n\n    \"we booked a table for 6 on friday\" then \"make it 8\": the second line has no noun for the\n    count to hang on, so it is only a correction because the chat already has a number to\n    correct. Nothing is read when the chat holds none - \"make it 8\" on its own is not a\n    headcount - and nothing is read for the house they live in, which is a different number.\n    ");
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how_many")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    held_party = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PARTY_THING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held_party.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { op_le(&held_party.clone(), &Py::Int(0)) } }) {
    return Py::Int(0);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_THE_HOUSE(), "search", args))(&__args) }) {
    return Py::Int(0);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ONE_MORE(), "match", args))(&__args) }) {
    return op_add(&held_party.clone(), &Py::Int(1));
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ONE_LESS(), "match", args))(&__args) }) {
    return if py_truthy(&op_gt(&held_party.clone(), &Py::Int(1))) { op_sub(&held_party.clone(), &Py::Int(1)) } else { Py::Int(0) };
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_CHANGED_BARE(), g_NOW_THE_NUMBER()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_number(args))(&__args) };
    if py_truthy(&op_gt(&number.clone(), &Py::Int(0))) {
    return number.clone();
    }
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _forms_of(args: &[Py]) -> Py {
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
    let mut forms: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut single: Py = Py::None;
    let _ = py_str("The words the person may say the thing with: its singular and its plural.");
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = thing.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    forms = py_set(vec![thing.clone(), op_add(&thing.clone(), &py_str("s")), op_add(&thing.clone(), &py_str("es"))]);
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IRREGULAR(), "items", args))(&__args) });
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    plural = __unpk2[0].clone();
    single = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&single.clone(), &thing.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plural.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&forms.clone(), "add", args))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(forms.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_figure_counts_the_thing(args: &[Py]) -> Py {
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
    let mut match_: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "match") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: match"))) };
    let mut after_word: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after_word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after_word"))) };
    let mut thing: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut before: Py = Py::None;
    let mut first: Py = Py::None;
    let mut front: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("Whether the line says the figure is the count of `thing`, and not of something else.\n\n    A figure in a line is the count of the thing in hand only when the line puts the two\n    together. Everything else is a figure of its own: a name (\"number 9\"), a measurement (\"20\n    degrees\"), a time (\"its 2 now\", \"half 5\"), a price. One rule, read here, because every\n    shape of it was moving the count: the nights of a Rome trip became 9, then 5, then 2, and the\n    ask that follows (\"how many nights was it\") gave back the bus number.\n    ");
    before = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    front = if py_truthy(&before.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&before.clone(), "split", args))(&__args) }, &op_neg(&Py::Int(1))), "lower", args))(&__args) }, "strip", args))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_A_LABEL_IN_FRONT(), &front.clone()))) {
    return Py::Bool(false);
    }
    subject = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(before.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_COPULA(), "search", args))(&__args) };
    if py_truthy(&{ let __b = subject.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _forms_of(args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subj")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "group", args))(&__args) }, "lower", args))(&__args) })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("it"), py_str("there"), py_str("that"), py_str("this")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("subj")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&subject.clone(), "group", args))(&__args) }, "lower", args))(&__args) })) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _forms_of(args))(&__args) }, &after_word.clone()))) {
    first = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) }, "strip", args))(&__args) } } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_A_DUMMY_SUBJECT(), &first.clone()))) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_COUNTS_THE_THING_AFTER(), &after_word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&after_word.clone())) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _forms_of(args))(&__args) }, &after_word.clone()));
    Py::None
}
pub fn _the_lines(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own lines, oldest first, as the store keeps them.");
    turns = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) } } else { Py::None };
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { turns.clone() } else { py_tuple(vec![]) });
    for __item in __iter1 {
    turn = __item.clone();
    line = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    return said.clone();
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
    let mut held: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut line: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The thing the last numbered line named, when the chat keeps a count of it.");
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_lines(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    line = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counted(args))(&__args) });
    for __item in __iter2 {
    thing = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return thing.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _the_figure_that_stands(args: &[Py]) -> Py {
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
    let mut kept: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The line up to the word that pushes a number aside, so the number that stands is read.");
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WRONG_NUMBER_AFTER(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return said.clone();
    }
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &Py::None, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "start", args))(&__args) }, &Py::None), "strip", args))(&__args) };
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(kept.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE(), "search", args))(&__args) }) { kept.clone() } else { said.clone() };
    Py::None
}
pub fn _a_figure_for_a_thing_the_chat_counts(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut already: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "already") { v.clone() }
    else { py_tuple(vec![]) };
    let mut after: Py = Py::None;
    let mut figures: Py = Py::None;
    let mut held: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut number: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("{thing: number} for a figure said again, with the thing this chat already counts.\n\n    `already` is the counts the readers above found in this line. When they found one, the bare\n    figure is not read again: \"no, the 5 of us\" is a group of five, and reading its figure a second\n    time by way of the thing in hand filed it over the nights the chat was planning.\n    ");
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how_many")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&held.clone())) } }) {
    return py_dict(vec![]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKING(), "match", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_THE_COUNT(), "search", args))(&__args) } } } } }) {
    return py_dict(vec![]);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_figure_that_stands(args))(&__args) };
    figures = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE(), "finditer", args))(&__args) }) { let mut match_: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(figures.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return py_dict(vec![]);
    }
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&figures.clone(), &Py::Int(0))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_number(args))(&__args) };
    if py_truthy(&op_le(&number.clone(), &Py::Int(0))) {
    return py_dict(vec![]);
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counted(args))(&__args) }) { let mut thing: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&held.clone(), &thing.clone()))) {  __out.push(thing.clone()); } } py_list(__out) })() };
    if py_truthy(&named.clone()) {
    thing = geti(&named.clone(), &Py::Int(0));
    } else {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POINTS_AT_THE_THING(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_SAID_AS_A_CHANGE(), "search", args))(&__args) } } }) {
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_FIGURE(), "search", args))(&__args) };
    after = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e)), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&said.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("n")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "end", args))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    word = if py_truthy(&after.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&after.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) } } else { py_str("") };
    word = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(stdlib_attr("string", "punctuation").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "strip", args))(&__args) };
    if py_truthy(&already.clone()) {
    return py_dict(vec![]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_THING(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_COUNT(), &word.clone())) } }) {
    return py_dict(vec![]);
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_in_hand(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(match_.clone()); __args.push(word.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_figure_counts_the_thing(args))(&__args) }))) {
    return py_dict(vec![]);
    }
    } else {
    return py_dict(vec![]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }, &number.clone())) } }) {
    return py_dict(vec![]);
    }
    return py_dict(vec![(thing.clone(), number.clone())]);
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut beds: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut group: Py = Py::None;
    let mut held: Py = Py::None;
    let mut held_party: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut party: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("Keep how many of a thing the line says they have.\n\n    \"i have three brothers\" -> memory[\"user\"][\"how_many\"][\"brother\"] = 3. A count is a fact like\n    any other, so it is filed where the store files facts, and it is the number that is new: a\n    later line that changes it (\"no, four brothers\") files the new one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&text.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ASKING(), "match", args))(&__args) } } }) {
    return Py::None;
    }
    counts = py_dict(vec![]);
    beds = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _beds(args))(&__args) };
    if py_truthy(&beds.clone()) {
    let _ = py_setitem(&counts.clone(), &py_str("bedroom"), beds.clone());
    }
    group = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_group(args))(&__args) };
    if py_truthy(&group.clone()) {
    let _ = py_setitem(&counts.clone(), &g_GROUP(), group.clone());
    }
    party = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_party(args))(&__args) };
    if py_truthy(&party.clone()) {
    let _ = py_setitem(&counts.clone(), &g_PARTY_THING(), party.clone());
    } else {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number_put_right(args))(&__args) }) {
    let _ = py_setitem(&counts.clone(), &g_PARTY_THING(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_number_put_right(args))(&__args) });
    } else {
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_chat_feeds_people(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_bare_people(args))(&__args) } } }) {
    let _ = py_setitem(&counts.clone(), &g_PARTY_THING(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_bare_people(args))(&__args) });
    } else {
    if py_truthy(&{ let __b = group.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CHANGED(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ABOUT_THE_HOUSE(), "search", args))(&__args) })) } } } }) {
    held_party = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PARTY_THING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how_many")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held_party.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = py_setitem(&counts.clone(), &g_PARTY_THING(), group.clone());
    }
    }
    }
    }
    }
    put_right = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_CORRECTED(), "match", args))(&__args) };
    if py_truthy(&put_right.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&put_right.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counted(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&counts.clone(), "update", args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_THEY_HAVE(), g_MY_THREE()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pattern.clone(), "finditer", args))(&__args) });
    for __item in __iter2 {
    match_ = __item.clone();
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rest")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) };
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _counted(args))(&__args) }, "items", args))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk4[0].clone();
    number = __unpk4[1].clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&counts.clone(), "setdefault", args))(&__args) };
    }
    }
    }
    let __iter5 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(memory.clone()); __args.push(counts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_figure_for_a_thing_the_chat_counts(args))(&__args) }, "items", args))(&__args) });
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk6[0].clone();
    number = __unpk6[1].clone();
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&counts.clone(), "setdefault", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&counts.clone()))) {
    return Py::None;
    }
    user = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(user.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    user = py_dict(vec![]);
    let _ = py_setitem(&memory.clone(), &py_str("user"), py_dict(vec![]));
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how_many")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    held = py_dict(vec![]);
    }
    let __iter7 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&counts.clone(), "items", args))(&__args) });
    for __item in __iter7 {
    let __unpk8 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk8[0].clone();
    number = __unpk8[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }, &number.clone()))) {
    let _ = py_setitem(&held.clone(), &thing.clone(), number.clone());
    }
    }
    let _ = py_setitem(&user.clone(), &py_str("how_many"), held.clone());
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
    let mut asked: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut held: Py = Py::None;
    let mut low: Py = Py::None;
    let mut many: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut missing: Py = Py::None;
    let mut number: Py = Py::None;
    let mut of_the_group: Py = Py::None;
    let mut of_you: Py = Py::None;
    let mut one: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The number of a thing they asked the count of, or None when the store holds no count.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&memory.clone(), "get", args))(&__args) })) } }) {
    return Py::None;
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how_many")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&memory.clone(), &py_str("user")), "get", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&held.clone())) } }) {
    return Py::None;
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_THE_COUNT(), "search", args))(&__args) }) {
    return Py::None;
    }
    of_you = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GROUP_ASK_YOU(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_GROUP_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_GROUP_ASK(), "search", args))(&__args) }; if py_truthy(&__b) { __b } else { of_you.clone() } } } }) {
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GROUP()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&number.clone(), &Py::Int(0)) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_group_said_back(args))(&__args) };
    }
    if py_truthy(&of_you.clone()) {
    return Py::None;
    }
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PARTY_THING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&number.clone(), &Py::Int(0)) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(low.clone()); __args.push(memory.clone()); __kws.extend(vec![("of_the_group".to_string(), Py::Bool(py_contains(&low.clone(), &py_str("of us"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_party_said_back(args))(&__args) };
    }
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PARTY_ASK(), "search", args))(&__args) }) {
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PARTY_THING()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = op_gt(&number.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_chat_sets_a_table(args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(low.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_party_said_back(args))(&__args) };
    }
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GROUP()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = op_gt(&number.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_POINTS_AT_A_NUMBER(), "search", args))(&__args) } } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(low.clone()); __args.push(memory.clone()); __kws.extend(vec![("of_the_group".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_party_said_back(args))(&__args) };
    }
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_BARE_COUNT_ASK(), "search", args))(&__args) }) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![g_PARTY_THING(), Py::Bool(false)]), py_tuple(vec![g_GROUP(), Py::Bool(true)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk2[0].clone();
    of_the_group = __unpk2[1].clone();
    number = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&number.clone(), &Py::Int(0)) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); __args.push(low.clone()); __args.push(memory.clone()); __kws.extend(vec![("of_the_group".to_string(), of_the_group.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_party_said_back(args))(&__args) };
    }
    }
    return Py::None;
    }
    match_ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str(" "), &low.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_HOW_MANY(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    asked = py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing_of(args))(&__args) }]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("also")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("also")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&match_.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _thing_of(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&asked.clone(), "append", args))(&__args) };
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut thing: Py = __item.clone(); if py_truthy(&thing.clone()) {  __out.push(thing.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::None;
    }
    counted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut thing: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(py_tuple(vec![thing.clone(), geti(&held.clone(), &thing.clone())])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&counted.clone()))) {
    return Py::None;
    }
    parts = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&counted.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    thing = __unpk4[0].clone();
    number = __unpk4[1].clone();
    many = if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(1)))) { thing.clone() } else { op_add(&thing.clone(), &py_str("s")) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IRREGULAR(), "values", args))(&__args) }, &thing.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&number.clone(), &Py::Int(1))) } }) {
    many = geti(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_IRREGULAR(), "items", args))(&__args) }) { let __unpk4180 = py_unpack(&__item.clone(), 2, None); let mut plural: Py = __unpk4180[0].clone(); let mut one: Py = __unpk4180[1].clone(); if py_truthy(&Py::Bool(py_eq(&one.clone(), &thing.clone()))) {  __out.push(plural.clone()); } } py_list(__out) })() }, &Py::Int(0));
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _many(args))(&__args) }, &py_str(" ")), &many.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&parts.clone(), "append", args))(&__args) };
    }
    said = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(parts.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) { geti(&parts.clone(), &Py::Int(0)) } else { op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&parts.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" and "), "join", args))(&__args) }, &py_str(" and ")), &geti(&parts.clone(), &op_neg(&Py::Int(1)))) };
    if py_truthy(&op_lt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(counted.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(asked.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {
    missing = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut thing: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&held.clone(), "get", args))(&__args) }); __args.push(py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {  __out.push(thing.clone()); } } py_list(__out) })() };
    return op_add(&op_add(&op_add(&op_add(&py_str("You've got "), &said.clone()), &py_str(" - I've not been told how many ")), &geti(&missing.clone(), &Py::Int(0))), &py_str("s you have."));
    }
    return op_add(&op_add(&py_str("You've got "), &said.clone()), &py_str("."));
    Py::None
}
pub fn _the_chat_books(args: &[Py]) -> Py {
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
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("Whether the plan the chat is on is a booking, so a table is the thing to say back.");
    user = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    plans = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plans")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plan")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&user.clone(), "get", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&plans.clone(), "append", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&plans.clone()) { let mut plan: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:book|booked|booking|reserv\\w*|table)\\b")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plan.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_party_said_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut of_the_group: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "of_the_group") { v.clone() }
    else { Py::Bool(false) };
    let mut many: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The number of people they are cooking for, said back in their own terms.\n\n    \"and how many people\" eight turns into a roast is answered \"Eight people, you said.\" - the\n    number belongs to the plan the chat is on, so nothing is asked for again. The noun follows the\n    ask (\"how many guests\" gets \"Eight guests\"), and a chat that asks twice gets a second wording\n    rather than the same sentence twice.\n    ");
    many = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _many(args))(&__args) };
    if py_truthy(&of_the_group.clone()) {
    plain = op_add(&op_add(&py_str("You said "), &many.clone()), &py_str(" of you."));
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_set(__out) })() };
    return if py_truthy(&Py::Bool(py_contains(&said.clone(), &plain.clone()))) { op_add(&op_add(&py_str("Still "), &many.clone()), &py_str(" of you.")) } else { plain.clone() };
    }
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &py_str("guest")))) {
    noun = if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(1)))) { py_str("guest") } else { py_str("guests") };
    } else {
    if py_truthy(&Py::Bool(py_contains(&low.clone(), &py_str("adult")))) {
    noun = if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(1)))) { py_str("adult") } else { py_str("adults") };
    } else {
    if py_truthy(&{ let __b = { let __b = Py::Bool(py_contains(&low.clone(), &py_str("book"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&low.clone(), &py_str("table"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("reserv"))) } } } }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_chat_books(args))(&__args) } } }) {
    return op_add(&op_add(&py_str("The table is for "), &many.clone()), &py_str("."));
    } else {
    noun = if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(1)))) { py_str("person") } else { py_str("people") };
    }
    }
    }
    plain = op_add(&op_add(&op_add(&op_add(&py_str("You said "), &many.clone()), &py_str(" ")), &noun.clone()), &py_str("."));
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bot")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_set(__out) })() };
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &plain.clone()))) {
    return op_add(&op_add(&op_add(&op_add(&py_str("Still "), &many.clone()), &py_str(" ")), &noun.clone()), &py_str("."));
    }
    return plain.clone();
    Py::None
}
pub fn _the_group_said_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut number: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let _ = py_str("The size of the group, said the way a person would say it back.");
    if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(1)))) {
    return py_str("Just you, then.");
    }
    if py_truthy(&Py::Bool(py_eq(&number.clone(), &Py::Int(2)))) {
    return py_str("The two of you, then.");
    }
    return op_add(&op_add(&py_str("There are "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(number.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _many(args))(&__args) }), &py_str(" of you."));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.counts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_HOUSE" => Some(g_ABOUT_THE_HOUSE()),
            "ASKING" => Some(g_ASKING()),
            "A_COPULA" => Some(g_A_COPULA()),
            "A_DUMMY_SUBJECT" => Some(g_A_DUMMY_SUBJECT()),
            "A_FIGURE" => Some(g_A_FIGURE()),
            "A_KIND_OF_PLACE" => Some(g_A_KIND_OF_PLACE()),
            "A_LABEL_IN_FRONT" => Some(g_A_LABEL_IN_FRONT()),
            "A_MEAL_IN_THE_CHAT" => Some(g_A_MEAL_IN_THE_CHAT()),
            "A_MEASURE_WORD" => Some(g_A_MEASURE_WORD()),
            "A_PERSON_TAIL" => Some(g_A_PERSON_TAIL()),
            "A_RANGE_AROUND" => Some(g_A_RANGE_AROUND()),
            "A_TABLE_BOOKED" => Some(g_A_TABLE_BOOKED()),
            "A_TABLE_CUE" => Some(g_A_TABLE_CUE()),
            "BARE_COUNT_ASK" => Some(g_BARE_COUNT_ASK()),
            "BARE_GROUP_ASK" => Some(g_BARE_GROUP_ASK()),
            "BARE_NUMBER" => Some(g_BARE_NUMBER()),
            "BARE_PEOPLE" => Some(g_BARE_PEOPLE()),
            "BEDS" => Some(g_BEDS()),
            "CHANGED" => Some(g_CHANGED()),
            "CHANGED_BARE" => Some(g_CHANGED_BARE()),
            "CHAT_JOINERS" => Some(g_CHAT_JOINERS()),
            "CORRECTED" => Some(g_CORRECTED()),
            "COUNTS_THE_THING_AFTER" => Some(g_COUNTS_THE_THING_AFTER()),
            "GROUP" => Some(g_GROUP()),
            "GROUP_ASK" => Some(g_GROUP_ASK()),
            "GROUP_ASK_YOU" => Some(g_GROUP_ASK_YOU()),
            "GROUP_CORRECTED" => Some(g_GROUP_CORRECTED()),
            "GROUP_NUMBER" => Some(g_GROUP_NUMBER()),
            "GROUP_OF_FOUR" => Some(g_GROUP_OF_FOUR()),
            "GROUP_OF_US" => Some(g_GROUP_OF_US()),
            "HOW_MANY" => Some(g_HOW_MANY()),
            "IRREGULAR" => Some(g_IRREGULAR()),
            "LOOK_BACK_FOR_A_MEAL" => Some(g_LOOK_BACK_FOR_A_MEAL()),
            "MAX_PHRASE" => Some(g_MAX_PHRASE()),
            "MY_THREE" => Some(g_MY_THREE()),
            "NOT_A_COUNT" => Some(g_NOT_A_COUNT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_THE_COUNT" => Some(g_NOT_THE_COUNT()),
            "NOW_THE_NUMBER" => Some(g_NOW_THE_NUMBER()),
            "NUMBER_AHEAD" => Some(g_NUMBER_AHEAD()),
            "ONE_LESS" => Some(g_ONE_LESS()),
            "ONE_MORE" => Some(g_ONE_MORE()),
            "PARTY" => Some(g_PARTY()),
            "PARTY_ASK" => Some(g_PARTY_ASK()),
            "PARTY_THING" => Some(g_PARTY_THING()),
            "PHRASE_ENDS" => Some(g_PHRASE_ENDS()),
            "POINTS_AT_A_NUMBER" => Some(g_POINTS_AT_A_NUMBER()),
            "POINTS_AT_THE_THING" => Some(g_POINTS_AT_THE_THING()),
            "RELATION_AHEAD" => Some(g_RELATION_AHEAD()),
            "SAID_AS_A_CHANGE" => Some(g_SAID_AS_A_CHANGE()),
            "TABLE_MEAL" => Some(g_TABLE_MEAL()),
            "THEIRS" => Some(g_THEIRS()),
            "THEY_HAVE" => Some(g_THEY_HAVE()),
            "VERB_IN_PHRASE" => Some(g_VERB_IN_PHRASE()),
            "WRONG_NUMBER_AFTER" => Some(g_WRONG_NUMBER_AFTER()),
            "_A_DAY" => Some(g__A_DAY()),
            "_BIGGER_NUMBERS" => Some(g__BIGGER_NUMBERS()),
            "_GROUP_ASK_WORDS" => Some(g__GROUP_ASK_WORDS()),
            "_GROUP_POINTER" => Some(g__GROUP_POINTER()),
            "_GROUP_TAIL" => Some(g__GROUP_TAIL()),
            "_NUM_WORD" => Some(g__NUM_WORD()),
            "_SUBJECT_AFTER" => Some(g__SUBJECT_AFTER()),
            "_a_figure_for_a_thing_the_chat_counts" => Some(py_func("_a_figure_for_a_thing_the_chat_counts", |args: &[Py]| _a_figure_for_a_thing_the_chat_counts(args))),
            "_beds" => Some(py_func("_beds", |args: &[Py]| _beds(args))),
            "_counted" => Some(py_func("_counted", |args: &[Py]| _counted(args))),
            "_forms_of" => Some(py_func("_forms_of", |args: &[Py]| _forms_of(args))),
            "_head" => Some(py_func("_head", |args: &[Py]| _head(args))),
            "_many" => Some(py_func("_many", |args: &[Py]| _many(args))),
            "_said_number" => Some(py_func("_said_number", |args: &[Py]| _said_number(args))),
            "_the_bare_people" => Some(py_func("_the_bare_people", |args: &[Py]| _the_bare_people(args))),
            "_the_chat_books" => Some(py_func("_the_chat_books", |args: &[Py]| _the_chat_books(args))),
            "_the_chat_feeds_people" => Some(py_func("_the_chat_feeds_people", |args: &[Py]| _the_chat_feeds_people(args))),
            "_the_chat_lines" => Some(py_func("_the_chat_lines", |args: &[Py]| _the_chat_lines(args))),
            "_the_chat_sets_a_table" => Some(py_func("_the_chat_sets_a_table", |args: &[Py]| _the_chat_sets_a_table(args))),
            "_the_figure_counts_the_thing" => Some(py_func("_the_figure_counts_the_thing", |args: &[Py]| _the_figure_counts_the_thing(args))),
            "_the_figure_that_stands" => Some(py_func("_the_figure_that_stands", |args: &[Py]| _the_figure_that_stands(args))),
            "_the_group" => Some(py_func("_the_group", |args: &[Py]| _the_group(args))),
            "_the_group_said_back" => Some(py_func("_the_group_said_back", |args: &[Py]| _the_group_said_back(args))),
            "_the_lines" => Some(py_func("_the_lines", |args: &[Py]| _the_lines(args))),
            "_the_number_put_right" => Some(py_func("_the_number_put_right", |args: &[Py]| _the_number_put_right(args))),
            "_the_party" => Some(py_func("_the_party", |args: &[Py]| _the_party(args))),
            "_the_party_said_back" => Some(py_func("_the_party_said_back", |args: &[Py]| _the_party_said_back(args))),
            "_the_relation" => Some(py_func("_the_relation", |args: &[Py]| _the_relation(args))),
            "_the_thing_in_hand" => Some(py_func("_the_thing_in_hand", |args: &[Py]| _the_thing_in_hand(args))),
            "_thing" => Some(py_func("_thing", |args: &[Py]| _thing(args))),
            "_thing_of" => Some(py_func("_thing_of", |args: &[Py]| _thing_of(args))),
            "answer" => Some(py_func("answer", |args: &[Py]| answer(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ABOUT_THE_HOUSE" => s_ABOUT_THE_HOUSE(v),
            "ASKING" => s_ASKING(v),
            "A_COPULA" => s_A_COPULA(v),
            "A_DUMMY_SUBJECT" => s_A_DUMMY_SUBJECT(v),
            "A_FIGURE" => s_A_FIGURE(v),
            "A_KIND_OF_PLACE" => s_A_KIND_OF_PLACE(v),
            "A_LABEL_IN_FRONT" => s_A_LABEL_IN_FRONT(v),
            "A_MEAL_IN_THE_CHAT" => s_A_MEAL_IN_THE_CHAT(v),
            "A_MEASURE_WORD" => s_A_MEASURE_WORD(v),
            "A_PERSON_TAIL" => s_A_PERSON_TAIL(v),
            "A_RANGE_AROUND" => s_A_RANGE_AROUND(v),
            "A_TABLE_BOOKED" => s_A_TABLE_BOOKED(v),
            "A_TABLE_CUE" => s_A_TABLE_CUE(v),
            "BARE_COUNT_ASK" => s_BARE_COUNT_ASK(v),
            "BARE_GROUP_ASK" => s_BARE_GROUP_ASK(v),
            "BARE_NUMBER" => s_BARE_NUMBER(v),
            "BARE_PEOPLE" => s_BARE_PEOPLE(v),
            "BEDS" => s_BEDS(v),
            "CHANGED" => s_CHANGED(v),
            "CHANGED_BARE" => s_CHANGED_BARE(v),
            "CHAT_JOINERS" => s_CHAT_JOINERS(v),
            "CORRECTED" => s_CORRECTED(v),
            "COUNTS_THE_THING_AFTER" => s_COUNTS_THE_THING_AFTER(v),
            "GROUP" => s_GROUP(v),
            "GROUP_ASK" => s_GROUP_ASK(v),
            "GROUP_ASK_YOU" => s_GROUP_ASK_YOU(v),
            "GROUP_CORRECTED" => s_GROUP_CORRECTED(v),
            "GROUP_NUMBER" => s_GROUP_NUMBER(v),
            "GROUP_OF_FOUR" => s_GROUP_OF_FOUR(v),
            "GROUP_OF_US" => s_GROUP_OF_US(v),
            "HOW_MANY" => s_HOW_MANY(v),
            "IRREGULAR" => s_IRREGULAR(v),
            "LOOK_BACK_FOR_A_MEAL" => s_LOOK_BACK_FOR_A_MEAL(v),
            "MAX_PHRASE" => s_MAX_PHRASE(v),
            "MY_THREE" => s_MY_THREE(v),
            "NOT_A_COUNT" => s_NOT_A_COUNT(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_THE_COUNT" => s_NOT_THE_COUNT(v),
            "NOW_THE_NUMBER" => s_NOW_THE_NUMBER(v),
            "NUMBER_AHEAD" => s_NUMBER_AHEAD(v),
            "ONE_LESS" => s_ONE_LESS(v),
            "ONE_MORE" => s_ONE_MORE(v),
            "PARTY" => s_PARTY(v),
            "PARTY_ASK" => s_PARTY_ASK(v),
            "PARTY_THING" => s_PARTY_THING(v),
            "PHRASE_ENDS" => s_PHRASE_ENDS(v),
            "POINTS_AT_A_NUMBER" => s_POINTS_AT_A_NUMBER(v),
            "POINTS_AT_THE_THING" => s_POINTS_AT_THE_THING(v),
            "RELATION_AHEAD" => s_RELATION_AHEAD(v),
            "SAID_AS_A_CHANGE" => s_SAID_AS_A_CHANGE(v),
            "TABLE_MEAL" => s_TABLE_MEAL(v),
            "THEIRS" => s_THEIRS(v),
            "THEY_HAVE" => s_THEY_HAVE(v),
            "VERB_IN_PHRASE" => s_VERB_IN_PHRASE(v),
            "WRONG_NUMBER_AFTER" => s_WRONG_NUMBER_AFTER(v),
            "_A_DAY" => s__A_DAY(v),
            "_BIGGER_NUMBERS" => s__BIGGER_NUMBERS(v),
            "_GROUP_ASK_WORDS" => s__GROUP_ASK_WORDS(v),
            "_GROUP_POINTER" => s__GROUP_POINTER(v),
            "_GROUP_TAIL" => s__GROUP_TAIL(v),
            "_NUM_WORD" => s__NUM_WORD(v),
            "_SUBJECT_AFTER" => s__SUBJECT_AFTER(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.counts", module_obj);
    pyrt::xrefs::reg_fn("memory.counts", "_many", py_func("_many", |args: &[Py]| _many(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_head", py_func("_head", |args: &[Py]| _head(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_relation", py_func("_the_relation", |args: &[Py]| _the_relation(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_thing_of", py_func("_thing_of", |args: &[Py]| _thing_of(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_thing", py_func("_thing", |args: &[Py]| _thing(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_beds", py_func("_beds", |args: &[Py]| _beds(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_counted", py_func("_counted", |args: &[Py]| _counted(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_said_number", py_func("_said_number", |args: &[Py]| _said_number(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_group", py_func("_the_group", |args: &[Py]| _the_group(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_party", py_func("_the_party", |args: &[Py]| _the_party(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_chat_lines", py_func("_the_chat_lines", |args: &[Py]| _the_chat_lines(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_chat_feeds_people", py_func("_the_chat_feeds_people", |args: &[Py]| _the_chat_feeds_people(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_chat_sets_a_table", py_func("_the_chat_sets_a_table", |args: &[Py]| _the_chat_sets_a_table(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_bare_people", py_func("_the_bare_people", |args: &[Py]| _the_bare_people(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_number_put_right", py_func("_the_number_put_right", |args: &[Py]| _the_number_put_right(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_forms_of", py_func("_forms_of", |args: &[Py]| _forms_of(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_figure_counts_the_thing", py_func("_the_figure_counts_the_thing", |args: &[Py]| _the_figure_counts_the_thing(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_lines", py_func("_the_lines", |args: &[Py]| _the_lines(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_thing_in_hand", py_func("_the_thing_in_hand", |args: &[Py]| _the_thing_in_hand(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_figure_that_stands", py_func("_the_figure_that_stands", |args: &[Py]| _the_figure_that_stands(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_a_figure_for_a_thing_the_chat_counts", py_func("_a_figure_for_a_thing_the_chat_counts", |args: &[Py]| _a_figure_for_a_thing_the_chat_counts(args)));
    pyrt::xrefs::reg_fn("memory.counts", "observe", py_func("observe", |args: &[Py]| observe(args)));
    pyrt::xrefs::reg_fn("memory.counts", "answer", py_func("answer", |args: &[Py]| answer(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_chat_books", py_func("_the_chat_books", |args: &[Py]| _the_chat_books(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_party_said_back", py_func("_the_party_said_back", |args: &[Py]| _the_party_said_back(args)));
    pyrt::xrefs::reg_fn("memory.counts", "_the_group_said_back", py_func("_the_group_said_back", |args: &[Py]| _the_group_said_back(args)));
    pyrt::xrefs::reg_access("memory.counts", "ABOUT_THE_HOUSE", g_ABOUT_THE_HOUSE, s_ABOUT_THE_HOUSE);
    pyrt::xrefs::reg_access("memory.counts", "ASKING", g_ASKING, s_ASKING);
    pyrt::xrefs::reg_access("memory.counts", "A_COPULA", g_A_COPULA, s_A_COPULA);
    pyrt::xrefs::reg_access("memory.counts", "A_DUMMY_SUBJECT", g_A_DUMMY_SUBJECT, s_A_DUMMY_SUBJECT);
    pyrt::xrefs::reg_access("memory.counts", "A_FIGURE", g_A_FIGURE, s_A_FIGURE);
    pyrt::xrefs::reg_access("memory.counts", "A_KIND_OF_PLACE", g_A_KIND_OF_PLACE, s_A_KIND_OF_PLACE);
    pyrt::xrefs::reg_access("memory.counts", "A_LABEL_IN_FRONT", g_A_LABEL_IN_FRONT, s_A_LABEL_IN_FRONT);
    pyrt::xrefs::reg_access("memory.counts", "A_MEAL_IN_THE_CHAT", g_A_MEAL_IN_THE_CHAT, s_A_MEAL_IN_THE_CHAT);
    pyrt::xrefs::reg_access("memory.counts", "A_MEASURE_WORD", g_A_MEASURE_WORD, s_A_MEASURE_WORD);
    pyrt::xrefs::reg_access("memory.counts", "A_PERSON_TAIL", g_A_PERSON_TAIL, s_A_PERSON_TAIL);
    pyrt::xrefs::reg_access("memory.counts", "A_RANGE_AROUND", g_A_RANGE_AROUND, s_A_RANGE_AROUND);
    pyrt::xrefs::reg_access("memory.counts", "A_TABLE_BOOKED", g_A_TABLE_BOOKED, s_A_TABLE_BOOKED);
    pyrt::xrefs::reg_access("memory.counts", "A_TABLE_CUE", g_A_TABLE_CUE, s_A_TABLE_CUE);
    pyrt::xrefs::reg_access("memory.counts", "BARE_COUNT_ASK", g_BARE_COUNT_ASK, s_BARE_COUNT_ASK);
    pyrt::xrefs::reg_access("memory.counts", "BARE_GROUP_ASK", g_BARE_GROUP_ASK, s_BARE_GROUP_ASK);
    pyrt::xrefs::reg_access("memory.counts", "BARE_NUMBER", g_BARE_NUMBER, s_BARE_NUMBER);
    pyrt::xrefs::reg_access("memory.counts", "BARE_PEOPLE", g_BARE_PEOPLE, s_BARE_PEOPLE);
    pyrt::xrefs::reg_access("memory.counts", "BEDS", g_BEDS, s_BEDS);
    pyrt::xrefs::reg_access("memory.counts", "CHANGED", g_CHANGED, s_CHANGED);
    pyrt::xrefs::reg_access("memory.counts", "CHANGED_BARE", g_CHANGED_BARE, s_CHANGED_BARE);
    pyrt::xrefs::reg_access("memory.counts", "CHAT_JOINERS", g_CHAT_JOINERS, s_CHAT_JOINERS);
    pyrt::xrefs::reg_access("memory.counts", "CORRECTED", g_CORRECTED, s_CORRECTED);
    pyrt::xrefs::reg_access("memory.counts", "COUNTS_THE_THING_AFTER", g_COUNTS_THE_THING_AFTER, s_COUNTS_THE_THING_AFTER);
    pyrt::xrefs::reg_access("memory.counts", "GROUP", g_GROUP, s_GROUP);
    pyrt::xrefs::reg_access("memory.counts", "GROUP_ASK", g_GROUP_ASK, s_GROUP_ASK);
    pyrt::xrefs::reg_access("memory.counts", "GROUP_ASK_YOU", g_GROUP_ASK_YOU, s_GROUP_ASK_YOU);
    pyrt::xrefs::reg_access("memory.counts", "GROUP_CORRECTED", g_GROUP_CORRECTED, s_GROUP_CORRECTED);
    pyrt::xrefs::reg_access("memory.counts", "GROUP_NUMBER", g_GROUP_NUMBER, s_GROUP_NUMBER);
    pyrt::xrefs::reg_access("memory.counts", "GROUP_OF_FOUR", g_GROUP_OF_FOUR, s_GROUP_OF_FOUR);
    pyrt::xrefs::reg_access("memory.counts", "GROUP_OF_US", g_GROUP_OF_US, s_GROUP_OF_US);
    pyrt::xrefs::reg_access("memory.counts", "HOW_MANY", g_HOW_MANY, s_HOW_MANY);
    pyrt::xrefs::reg_access("memory.counts", "IRREGULAR", g_IRREGULAR, s_IRREGULAR);
    pyrt::xrefs::reg_access("memory.counts", "LOOK_BACK_FOR_A_MEAL", g_LOOK_BACK_FOR_A_MEAL, s_LOOK_BACK_FOR_A_MEAL);
    pyrt::xrefs::reg_access("memory.counts", "MAX_PHRASE", g_MAX_PHRASE, s_MAX_PHRASE);
    pyrt::xrefs::reg_access("memory.counts", "MY_THREE", g_MY_THREE, s_MY_THREE);
    pyrt::xrefs::reg_access("memory.counts", "NOT_A_COUNT", g_NOT_A_COUNT, s_NOT_A_COUNT);
    pyrt::xrefs::reg_access("memory.counts", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.counts", "NOT_THE_COUNT", g_NOT_THE_COUNT, s_NOT_THE_COUNT);
    pyrt::xrefs::reg_access("memory.counts", "NOW_THE_NUMBER", g_NOW_THE_NUMBER, s_NOW_THE_NUMBER);
    pyrt::xrefs::reg_access("memory.counts", "NUMBER_AHEAD", g_NUMBER_AHEAD, s_NUMBER_AHEAD);
    pyrt::xrefs::reg_access("memory.counts", "ONE_LESS", g_ONE_LESS, s_ONE_LESS);
    pyrt::xrefs::reg_access("memory.counts", "ONE_MORE", g_ONE_MORE, s_ONE_MORE);
    pyrt::xrefs::reg_access("memory.counts", "PARTY", g_PARTY, s_PARTY);
    pyrt::xrefs::reg_access("memory.counts", "PARTY_ASK", g_PARTY_ASK, s_PARTY_ASK);
    pyrt::xrefs::reg_access("memory.counts", "PARTY_THING", g_PARTY_THING, s_PARTY_THING);
    pyrt::xrefs::reg_access("memory.counts", "PHRASE_ENDS", g_PHRASE_ENDS, s_PHRASE_ENDS);
    pyrt::xrefs::reg_access("memory.counts", "POINTS_AT_A_NUMBER", g_POINTS_AT_A_NUMBER, s_POINTS_AT_A_NUMBER);
    pyrt::xrefs::reg_access("memory.counts", "POINTS_AT_THE_THING", g_POINTS_AT_THE_THING, s_POINTS_AT_THE_THING);
    pyrt::xrefs::reg_access("memory.counts", "RELATION_AHEAD", g_RELATION_AHEAD, s_RELATION_AHEAD);
    pyrt::xrefs::reg_access("memory.counts", "SAID_AS_A_CHANGE", g_SAID_AS_A_CHANGE, s_SAID_AS_A_CHANGE);
    pyrt::xrefs::reg_access("memory.counts", "TABLE_MEAL", g_TABLE_MEAL, s_TABLE_MEAL);
    pyrt::xrefs::reg_access("memory.counts", "THEIRS", g_THEIRS, s_THEIRS);
    pyrt::xrefs::reg_access("memory.counts", "THEY_HAVE", g_THEY_HAVE, s_THEY_HAVE);
    pyrt::xrefs::reg_access("memory.counts", "VERB_IN_PHRASE", g_VERB_IN_PHRASE, s_VERB_IN_PHRASE);
    pyrt::xrefs::reg_access("memory.counts", "WRONG_NUMBER_AFTER", g_WRONG_NUMBER_AFTER, s_WRONG_NUMBER_AFTER);
    pyrt::xrefs::reg_access("memory.counts", "_A_DAY", g__A_DAY, s__A_DAY);
    pyrt::xrefs::reg_access("memory.counts", "_BIGGER_NUMBERS", g__BIGGER_NUMBERS, s__BIGGER_NUMBERS);
    pyrt::xrefs::reg_access("memory.counts", "_GROUP_ASK_WORDS", g__GROUP_ASK_WORDS, s__GROUP_ASK_WORDS);
    pyrt::xrefs::reg_access("memory.counts", "_GROUP_POINTER", g__GROUP_POINTER, s__GROUP_POINTER);
    pyrt::xrefs::reg_access("memory.counts", "_GROUP_TAIL", g__GROUP_TAIL, s__GROUP_TAIL);
    pyrt::xrefs::reg_access("memory.counts", "_NUM_WORD", g__NUM_WORD, s__NUM_WORD);
    pyrt::xrefs::reg_access("memory.counts", "_SUBJECT_AFTER", g__SUBJECT_AFTER, s__SUBJECT_AFTER);
}

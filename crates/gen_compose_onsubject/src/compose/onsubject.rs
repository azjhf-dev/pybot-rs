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


thread_local! { static G_AMOUNT_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ASK_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_A_QUESTION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BACK_REFERENCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BOILING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COMPARISON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COOKING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_COOKING_TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DETERMINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_DVLA_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_EMPTY_ASK_LENGTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_EMPTY_ASK_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_EMPTY_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_EXISTENTIAL_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_EXPLANATION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_FOOD_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_HELP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ITS_OWN_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JUDGEMENT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_JUDGEMENT_OF_AMOUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_KNOWS_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LIGHT_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_LOST_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_MACHINE_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMED_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMED_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NAMING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NEEDS_A_NEIGHBOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_NOT_A_PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_OPEN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_OTHER_METHOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PASSPORT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_PREP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_PLACE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_POINTER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_RECALL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_REPEAT_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_ROASTING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SHOWN_AS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_SUBJECT_WORD_LIMIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_THING_VERB_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_TURNS_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_WORDS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_TIME_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MACHINE_BEFORE_MACHINE_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_CAR_GARAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_PASSPORT: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.onsubject: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AMOUNT_KINDS() -> Py { ensure_init(); G_AMOUNT_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AMOUNT_KINDS(v: Py) { ensure_init(); G_AMOUNT_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ASK_OPENER() -> Py { ensure_init(); G_A_ASK_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ASK_OPENER(v: Py) { ensure_init(); G_A_ASK_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_A_QUESTION_ASK() -> Py { ensure_init(); G_A_A_QUESTION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_A_QUESTION_ASK(v: Py) { ensure_init(); G_A_A_QUESTION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BACK_REFERENCE() -> Py { ensure_init(); G_A_BACK_REFERENCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BACK_REFERENCE(v: Py) { ensure_init(); G_A_BACK_REFERENCE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BOILING_WORD() -> Py { ensure_init(); G_A_BOILING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BOILING_WORD(v: Py) { ensure_init(); G_A_BOILING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COMPARISON_ASK() -> Py { ensure_init(); G_A_COMPARISON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COMPARISON_ASK(v: Py) { ensure_init(); G_A_COMPARISON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COOKING_ASK() -> Py { ensure_init(); G_A_COOKING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COOKING_ASK(v: Py) { ensure_init(); G_A_COOKING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_COOKING_TIME_ASK() -> Py { ensure_init(); G_A_COOKING_TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_COOKING_TIME_ASK(v: Py) { ensure_init(); G_A_COOKING_TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DETERMINER() -> Py { ensure_init(); G_A_DETERMINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DETERMINER(v: Py) { ensure_init(); G_A_DETERMINER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_DVLA_WORD() -> Py { ensure_init(); G_A_DVLA_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_DVLA_WORD(v: Py) { ensure_init(); G_A_DVLA_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_EMPTY_ASK_LENGTH() -> Py { ensure_init(); G_A_EMPTY_ASK_LENGTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_EMPTY_ASK_LENGTH(v: Py) { ensure_init(); G_A_EMPTY_ASK_LENGTH.with(|c| *c.borrow_mut() = v); }
pub fn g_A_EMPTY_ASK_OPENER() -> Py { ensure_init(); G_A_EMPTY_ASK_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_EMPTY_ASK_OPENER(v: Py) { ensure_init(); G_A_EMPTY_ASK_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_A_EMPTY_ASK_WORDS() -> Py { ensure_init(); G_A_EMPTY_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_EMPTY_ASK_WORDS(v: Py) { ensure_init(); G_A_EMPTY_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_EXISTENTIAL_THERE() -> Py { ensure_init(); G_A_EXISTENTIAL_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_EXISTENTIAL_THERE(v: Py) { ensure_init(); G_A_EXISTENTIAL_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_EXPLANATION() -> Py { ensure_init(); G_A_EXPLANATION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_EXPLANATION(v: Py) { ensure_init(); G_A_EXPLANATION.with(|c| *c.borrow_mut() = v); }
pub fn g_A_FOOD_PRONOUN() -> Py { ensure_init(); G_A_FOOD_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_FOOD_PRONOUN(v: Py) { ensure_init(); G_A_FOOD_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_A_HELP_ASK() -> Py { ensure_init(); G_A_HELP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_HELP_ASK(v: Py) { ensure_init(); G_A_HELP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ITS_OWN_SUBJECT() -> Py { ensure_init(); G_A_ITS_OWN_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ITS_OWN_SUBJECT(v: Py) { ensure_init(); G_A_ITS_OWN_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JUDGEMENT_ASK() -> Py { ensure_init(); G_A_JUDGEMENT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JUDGEMENT_ASK(v: Py) { ensure_init(); G_A_JUDGEMENT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_JUDGEMENT_OF_AMOUNT() -> Py { ensure_init(); G_A_JUDGEMENT_OF_AMOUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_JUDGEMENT_OF_AMOUNT(v: Py) { ensure_init(); G_A_JUDGEMENT_OF_AMOUNT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_KNOWS_IT() -> Py { ensure_init(); G_A_KNOWS_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_KNOWS_IT(v: Py) { ensure_init(); G_A_KNOWS_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LIGHT_KEY() -> Py { ensure_init(); G_A_LIGHT_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LIGHT_KEY(v: Py) { ensure_init(); G_A_LIGHT_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_LOST_THING() -> Py { ensure_init(); G_A_LOST_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_LOST_THING(v: Py) { ensure_init(); G_A_LOST_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_MACHINE_CUE() -> Py { ensure_init(); G_A_MACHINE_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_MACHINE_CUE(v: Py) { ensure_init(); G_A_MACHINE_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMED_FOOD() -> Py { ensure_init(); G_A_NAMED_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_FOOD(v: Py) { ensure_init(); G_A_NAMED_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMED_PLACE() -> Py { ensure_init(); G_A_NAMED_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMED_PLACE(v: Py) { ensure_init(); G_A_NAMED_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NAMING_ASK() -> Py { ensure_init(); G_A_NAMING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NAMING_ASK(v: Py) { ensure_init(); G_A_NAMING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NEEDS_A_NEIGHBOUR() -> Py { ensure_init(); G_A_NEEDS_A_NEIGHBOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NEEDS_A_NEIGHBOUR(v: Py) { ensure_init(); G_A_NEEDS_A_NEIGHBOUR.with(|c| *c.borrow_mut() = v); }
pub fn g_A_NOT_A_PLACE() -> Py { ensure_init(); G_A_NOT_A_PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_NOT_A_PLACE(v: Py) { ensure_init(); G_A_NOT_A_PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_OPEN_ASK() -> Py { ensure_init(); G_A_OPEN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_OPEN_ASK(v: Py) { ensure_init(); G_A_OPEN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_OTHER_METHOD() -> Py { ensure_init(); G_A_OTHER_METHOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_OTHER_METHOD(v: Py) { ensure_init(); G_A_OTHER_METHOD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PASSPORT_WORD() -> Py { ensure_init(); G_A_PASSPORT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PASSPORT_WORD(v: Py) { ensure_init(); G_A_PASSPORT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_PREP() -> Py { ensure_init(); G_A_PLACE_PREP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_PREP(v: Py) { ensure_init(); G_A_PLACE_PREP.with(|c| *c.borrow_mut() = v); }
pub fn g_A_PLACE_WORD() -> Py { ensure_init(); G_A_PLACE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_PLACE_WORD(v: Py) { ensure_init(); G_A_PLACE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_POINTER_ASK() -> Py { ensure_init(); G_A_POINTER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_POINTER_ASK(v: Py) { ensure_init(); G_A_POINTER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_RECALL_ASK() -> Py { ensure_init(); G_A_RECALL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_RECALL_ASK(v: Py) { ensure_init(); G_A_RECALL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_REPEAT_LEAD() -> Py { ensure_init(); G_A_REPEAT_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_REPEAT_LEAD(v: Py) { ensure_init(); G_A_REPEAT_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_ROASTING_WORD() -> Py { ensure_init(); G_A_ROASTING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_ROASTING_WORD(v: Py) { ensure_init(); G_A_ROASTING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SHOWN_AS() -> Py { ensure_init(); G_A_SHOWN_AS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SHOWN_AS(v: Py) { ensure_init(); G_A_SHOWN_AS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_SUBJECT_WORD_LIMIT() -> Py { ensure_init(); G_A_SUBJECT_WORD_LIMIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_SUBJECT_WORD_LIMIT(v: Py) { ensure_init(); G_A_SUBJECT_WORD_LIMIT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THERE() -> Py { ensure_init(); G_A_THERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THERE(v: Py) { ensure_init(); G_A_THERE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_THING_VERB_ASK() -> Py { ensure_init(); G_A_THING_VERB_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_THING_VERB_ASK(v: Py) { ensure_init(); G_A_THING_VERB_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_TURNS_ON() -> Py { ensure_init(); G_A_TURNS_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_TURNS_ON(v: Py) { ensure_init(); G_A_TURNS_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORD() -> Py { ensure_init(); G_A_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORD(v: Py) { ensure_init(); G_A_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_A_WORDS_ASK() -> Py { ensure_init(); G_A_WORDS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_WORDS_ASK(v: Py) { ensure_init(); G_A_WORDS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_TIME_SHAPES() -> Py { ensure_init(); G_COOKING_TIME_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_TIME_SHAPES(v: Py) { ensure_init(); G_COOKING_TIME_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_KINDS() -> Py { ensure_init(); G_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KINDS(v: Py) { ensure_init(); G_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MACHINE_BEFORE_MACHINE_KINDS() -> Py { ensure_init(); G_MACHINE_BEFORE_MACHINE_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MACHINE_BEFORE_MACHINE_KINDS(v: Py) { ensure_init(); G_MACHINE_BEFORE_MACHINE_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACES() -> Py { ensure_init(); G_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACES(v: Py) { ensure_init(); G_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_SHAPES() -> Py { ensure_init(); G_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHAPES(v: Py) { ensure_init(); G_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_STOP() -> Py { ensure_init(); G_STOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STOP(v: Py) { ensure_init(); G_STOP.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO() -> Py { ensure_init(); G_WHO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO(v: Py) { ensure_init(); G_WHO.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_CAR_GARAGE() -> Py { ensure_init(); G_WHO_CAR_GARAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_CAR_GARAGE(v: Py) { ensure_init(); G_WHO_CAR_GARAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_PASSPORT() -> Py { ensure_init(); G_WHO_PASSPORT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_PASSPORT(v: Py) { ensure_init(); G_WHO_PASSPORT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("A short follow-up with the subject left in the chat gets the subject named, not a search.\n\nThe decks are full of lines like \"can he do that\", \"do i need an adapter there\", \"how long will\nthat take to clear\", and every one of them reached the same honest hedge: \"That's beyond what I\nknow well, sorry. Searching for it will turn up better sources than I can offer\". The sentence is\ntrue and it is the worst reply in the bot here, because the person has just told the bot what they\nare talking about and the bot answers as if it had not been listening. A friend says the thing\nback - \"The rent, then\" - and then says who does know: the tenancy team, the vet, the garage, the\noperator's own site.\n\nSo this reads a follow-up that names nothing itself (short, no wh-word of its own subject, a\npronoun or a \"that\" for the thing), finds the word the chat is on from what the person said on the\nearlier turns, and answers with that word and the one place that settles it. It is a hedge still -\nthe bot does not know the answer and says so - but it is a hedge that shows it followed the chat\nand tells the person where to go, which is what STYLE.md asks for when the bot cannot know.\n\nNothing here is a fact about the world: the routes are places and people that exist for that kind\nof question (a pharmacist, ACAS, the DVLA's own page), never a number, an hour or a rule.\n");
    s_A_ASK_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:can|could|do|does|did|is|are|was|were|would|will|should|am|shall|have|has|had|what|when|where|which|who|why|how|if)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BACK_REFERENCE(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|this|there|he|she|they|him|her|them|one|those|these|hers|his|theirs)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUBJECT_WORD_LIMIT(Py::Int(2));
    s_A_EMPTY_ASK_WORDS(Py::Int(2));
    s_A_EMPTY_ASK_LENGTH(Py::Int(7));
    s_A_EMPTY_ASK_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:how|when)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_POINTER_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:which|who)\\b|^\\s*how\\s+come\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HELP_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|anything)\\s+(?:would\\s+|will\\s+|might\\s+)?helps?\\b|^\\s*what\\s+(?:can|should|could)\\s+(?:i|we)\\s+do\\b|^\\s*how\\s+(?:do|can|should)\\s+i\\s+(?:help|calm|support|comfort|reassure)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RECALL_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:how many|how much)\\b(?:(?!\\bhow much (?:is|does|do|will|would)\\b).)*?\\b(?:do i have|have i got|do we have|did i|did we|did he|did she)\\b|^\\s*how many\\b(?:\\s+\\w+){0,2}\\s*$|^\\s*how many\\b.*\\bagain\\s*$|^\\s*what (?:day|date|time)\\b|^\\s*whats the (?:day|date|time)\\b|^\\s*when (?:did|was that|were that|does my|is my|do i|are we)\\b|^\\s*did you (?:get|catch|hear|note|remember|write)\\b|\\b(?:what|how old|how long ago) (?:did|do|was) i (?:say|said|tell|told|mention|planning|planning to do|doing)\\b|\\bdid i (?:say|tell|mention)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COMPARISON_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:too (?:far|much|long|late|early|expensive|cheap|old|big|small)|worth it|a lot|any good|good value|cheaper|expensive|good for|reasonable|enough)\\b|\\b(?:is|was|would) (?:that|it|this) (?:good|bad|fair|ok|okay|decent)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_COOKING_TIME_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_FOOD_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:it|them|those|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_ROASTING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:roast|roasting|roasted|bake|baking|baked|oven)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_BOILING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:boil|boiling|boiled|simmer|simmering)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_OTHER_METHOD(stdlib_call("re", "compile", &[py_str("\\b(?:grill|grilling|grilled|fry|frying|fried|rest|resting|steam|steaming|poach|poaching|barbecue|barbeque|bbq|microwave|air fryer|toast|toasting)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAMED_FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:chicken|turkey|beef|lamb|pork|ham|gammon|duck|goose|joint|steak|sausage|sausages|fish|salmon|potato|potatoes|egg|eggs|rice|pasta|bread|cake|pastry|pie|casserole|stew|chops|chop|brisket|shoulder|leg|breast|thigh|fillet|kettle)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COOKING_TIME_SHAPES(py_dict(vec![(py_str("oven"), py_str("Time depends on what it is and how heavy: a chicken is about 20 minutes per 500g plus 20 minutes at 190C, and a joint of beef 15 to 25 minutes per 500g once it has browned. Tell me which it is and how much it weighs, and I'll do the sum.")), (py_str("boil"), py_str("Boiling time depends on what they are: eggs are 6 minutes from boiling water for a runny yolk and 9 to 10 for a hard one. Tell me what it is and I'll narrow it down.")), (py_str("either"), py_str("Time depends on the food and how it is cooked: a chicken is about 20 minutes per 500g plus 20 minutes at 190C in the oven, and eggs are 6 minutes from boiling water for a runny yolk. Tell me what it is and how you are cooking it."))]));
    s_A_COOKING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:cook|cooking|cooked|roast|roasting|bake|baking|boil|boiling|fry|frying|grill|oven|freezer|fridge|thaw|defrost|stuffing|gravy|rice|pasta|potato|potatoes|recipe|rest it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LOST_THING(stdlib_call("re", "compile", &[py_str("\\b(?:lost|mislaid|misplaced|can'?t find|cannot find)\\b|\\bleft (?:it|them|my|the|his|her)\\b[^.?!]*\\b(?:in|on|at|by)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MACHINE_CUE(stdlib_call("re", "compile", &[py_str("\\b(?:laptop|computer|phone|tablet|charger|charging|screen|windows|macbook|pc)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_A_QUESTION_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|whats|when|whens|where|wheres|which|who|whos|why|whys|how|hows|is|are|was|were|do|does|did|can|could|should|would|will|shall|have|has|am|any|anything)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_JUDGEMENT_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:stupid|silly|daft|mad|unreasonable|out of order|selfish|allowed)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_JUDGEMENT_OF_AMOUNT(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|would) (?:that|it|this) (?:good|bad|ok|okay|enough|fair|normal|reasonable|a lot|too much|too little|decent)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AMOUNT_KINDS(py_tuple(vec![py_str("money"), py_str("bills")]));
    s_MACHINE_BEFORE_MACHINE_KINDS(py_tuple(vec![py_str("car")]));
    s_A_WORDS_ASK(stdlib_call("re", "compile", &[py_str("^\\s*(?:what|whats|how|hows|should|shall|can you|could you|would you|do i need to|give me|help me)\\b[^?]*\\b(?:write|writing|word|words|say|saying|tell|telling|ask|asking|reply|answer|email|letter|phone|ring|call|complain|phrase)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAMING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|will|would|can|could|do|to) (?:i|we|you) (?:call|name)\\b|\\b(?:call|name) (?:it|him|her|them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("work"), py_str("school"), py_str("college"), py_str("hospital"), py_str("bank"), py_str("town"), py_str("city"), py_str("flat"), py_str("flats"), py_str("home"), py_str("house"), py_str("garage")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DETERMINER(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("this")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACE_PREP(call_builtin("frozenset", &[py_tuple(vec![py_str("at"), py_str("to"), py_str("from"), py_str("in"), py_str("into"), py_str("near"), py_str("off"), py_str("for"), py_str("of"), py_str("on")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_EXPLANATION(stdlib_call("re", "compile", &[py_str("\\b(?:thats why|that is why|because|so that|the reason)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NEEDS_A_NEIGHBOUR(py_dict(vec![(py_str("washing"), call_builtin("frozenset", &[py_tuple(vec![py_str("machine"), py_str("machines"), py_str("powder"), py_str("liquid"), py_str("detergent"), py_str("basket"), py_str("line"), py_str("drum"), py_str("load"), py_str("cycle")])]).unwrap_or_else(|e| exc::raise(e)))]));
    s_A_SHOWN_AS(py_dict(vec![(py_str("washing"), py_str("washing machine")), (py_str("sacked"), py_str("job"))]));
    s_A_ITS_OWN_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:little one|little man|youngest|eldest|oldest|the baby)\\b|\\b(?:he|she|they|him|her|them|son|daughter|child|kid|boy|girl)\\b[^?]{0,12}\\b\\d+\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_DVLA_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:mot|dvla|tax|sorn|licence|license|log ?book|v5|clamped)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_CAR_GARAGE(py_str("a garage will tell you over the phone"));
    fn __data1() -> Py { py_tuple(vec![py_str("leave"), py_str("pay"), py_str("order"), py_str("booking"), py_str("paint"), py_str("drill"), py_str("work"), py_str("delivery"), py_str("bid"), py_str("invoice"), py_str("credit"), py_str("deal"), py_str("cover"), py_str("fill"), py_str("charge"), py_str("term")]) }
    s_A_LIGHT_KEY(call_builtin("frozenset", &[__data1()]).unwrap_or_else(|e| exc::raise(e)));
    s_STOP(call_builtin("frozenset", &[mcall(&py_str("\na an the and or but if so then than that this those these i me my mine you your yours we us our he\nshe it they them him her his hers their there here what which who whom whose when where why how do\ndoes did done doing is are was were be been being am have has had having will would can could\nshould shall may might must not no nor yes ok okay just also too very really quite about for with\nwithout on in into at by from to of as up down out off over under again more most much many any\nsome all both each either neither other another same such own only ever never always still yet\nalready even though although because while since until before after during against between among\nright well thing things stuff get got give take make made say said tell told want wanted need\nneeded like liked know knew think thought feel felt go going went come came back put let try tried\nuse used find found work works working day days time times week weeks month months year years one\ntwo three four five six seven eight nine ten bit lot lots sort kind way ways now today tomorrow\nyesterday please thanks thank hi hello hey yeah yep nope um uh oh ah eh due anyway gonna wanna dont\ndoesnt didnt isnt wasnt cant couldnt wouldnt shouldnt havent hasnt im ive id ill youre youve theyre\nthats whats heres theres whose wont\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data2() -> Py { py_tuple(vec![py_str("vet"), py_str("puppy"), py_str("puppies"), py_str("kitten"), py_str("kittens"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("rabbit"), py_str("rabbits"), py_str("hamster"), py_str("guinea"), py_str("parrot"), py_str("fleas"), py_str("flea"), py_str("litter")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("mot"), py_str("tyre"), py_str("tyres"), py_str("brake"), py_str("brakes"), py_str("clutch"), py_str("gearbox"), py_str("engine"), py_str("battery"), py_str("batteries"), py_str("hybrid"), py_str("mileage"), py_str("garage"), py_str("mechanic"), py_str("dvla"), py_str("car"), py_str("cars")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("prescription"), py_str("pharmacist"), py_str("pharmacy"), py_str("symptom"), py_str("symptoms"), py_str("migraine"), py_str("infection"), py_str("swelling"), py_str("swollen"), py_str("rash"), py_str("piles"), py_str("pain"), py_str("dose"), py_str("tablets"), py_str("tablet"), py_str("medicine"), py_str("doctor"), py_str("hospital"), py_str("nurse"), py_str("clinic"), py_str("surgery")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("sacked"), py_str("redundancy"), py_str("disciplinary"), py_str("warning"), py_str("rota"), py_str("shift"), py_str("shifts"), py_str("overtime"), py_str("payslip"), py_str("employer"), py_str("manager"), py_str("boss"), py_str("hr"), py_str("union"), py_str("interview"), py_str("contract"), py_str("job"), py_str("workplace"), py_str("work"), py_str("leave")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("gcse"), py_str("gcses"), py_str("a-level"), py_str("a-levels"), py_str("exam"), py_str("exams"), py_str("revision"), py_str("homework"), py_str("teacher"), py_str("lesson"), py_str("lessons"), py_str("school"), py_str("college"), py_str("university"), py_str("ucas"), py_str("course"), py_str("term"), py_str("paper"), py_str("papers")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("boiler"), py_str("radiator"), py_str("radiators"), py_str("thermostat"), py_str("dishwasher"), py_str("washer"), py_str("washing"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("hob"), py_str("toilet"), py_str("shower"), py_str("leak"), py_str("leaking"), py_str("damp"), py_str("mould"), py_str("ceiling"), py_str("roof"), py_str("gutter"), py_str("gutters"), py_str("socket"), py_str("wiring"), py_str("plumber"), py_str("electrician"), py_str("limescale"), py_str("drain"), py_str("drains"), py_str("filter"), py_str("pump"), py_str("tap"), py_str("taps")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("curtain"), py_str("curtains"), py_str("rail"), py_str("drill"), py_str("shelf"), py_str("shelves"), py_str("paint"), py_str("wall"), py_str("walls"), py_str("screw"), py_str("screws"), py_str("rawplug"), py_str("rawplugs"), py_str("tile"), py_str("tiles"), py_str("tiling"), py_str("grout"), py_str("sanding"), py_str("filler"), py_str("sealant"), py_str("flatpack")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("passport"), py_str("visa"), py_str("boarding"), py_str("luggage"), py_str("suitcase"), py_str("adapter"), py_str("plug"), py_str("flight"), py_str("flights"), py_str("airport"), py_str("hotel"), py_str("train"), py_str("booking"), py_str("holiday"), py_str("abroad"), py_str("japan"), py_str("spain"), py_str("france"), py_str("italy"), py_str("greece"), py_str("portugal"), py_str("germany"), py_str("holland"), py_str("turkey"), py_str("egypt"), py_str("mexico"), py_str("canada"), py_str("america"), py_str("australia"), py_str("ireland"), py_str("scotland"), py_str("wales"), py_str("england"), py_str("london"), py_str("paris"), py_str("rome"), py_str("madrid"), py_str("lisbon")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("overdraft"), py_str("mortgage"), py_str("loan"), py_str("loan"), py_str("benefits"), py_str("universal"), py_str("pension"), py_str("savings"), py_str("debt"), py_str("invoice"), py_str("bill"), py_str("bills"), py_str("tax"), py_str("salary"), py_str("wages"), py_str("wage"), py_str("pay"), py_str("bank"), py_str("credit"), py_str("budget"), py_str("money")]) }
    s_KINDS(py_tuple(vec![py_tuple(vec![py_str("tenancy"), py_tuple(vec![py_str("landlord"), py_str("tenancy"), py_str("tenant"), py_str("tenants"), py_str("deposit"), py_str("eviction"), py_str("lease"), py_str("letting"), py_str("lettings"), py_str("rent"), py_str("flat"), py_str("flats")])]), py_tuple(vec![py_str("pet"), __data2()]), py_tuple(vec![py_str("car"), __data3()]), py_tuple(vec![py_str("bills"), py_tuple(vec![py_str("bill"), py_str("bills"), py_str("electric"), py_str("electricity"), py_str("energy"), py_str("supplier"), py_str("meter"), py_str("standing"), py_str("tariff"), py_str("direct"), py_str("debit"), py_str("arrears")])]), py_tuple(vec![py_str("health"), __data4()]), py_tuple(vec![py_str("work"), __data5()]), py_tuple(vec![py_str("school"), __data6()]), py_tuple(vec![py_str("home"), __data7()]), py_tuple(vec![py_str("diy"), __data8()]), py_tuple(vec![py_str("travel"), __data9()]), py_tuple(vec![py_str("shopping"), py_tuple(vec![py_str("receipt"), py_str("refund"), py_str("returns"), py_str("warranty"), py_str("guarantee"), py_str("seller"), py_str("order"), py_str("delivery"), py_str("shop"), py_str("store")])]), py_tuple(vec![py_str("money"), __data10()])]));
    fn __data11() -> Py { py_tuple(vec![py_str("japan"), py_str("spain"), py_str("france"), py_str("italy"), py_str("greece"), py_str("portugal"), py_str("germany"), py_str("holland"), py_str("turkey"), py_str("egypt"), py_str("mexico"), py_str("canada"), py_str("america"), py_str("australia"), py_str("ireland"), py_str("scotland"), py_str("wales"), py_str("england"), py_str("london"), py_str("manchester"), py_str("birmingham"), py_str("edinburgh"), py_str("cardiff"), py_str("belfast"), py_str("dublin"), py_str("paris"), py_str("rome"), py_str("madrid"), py_str("lisbon"), py_str("new"), py_str("york"), py_str("florida"), py_str("california")]) }
    s_PLACES(call_builtin("frozenset", &[__data11()]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PASSPORT_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("passport"), py_str("passports"), py_str("visa"), py_str("visas")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO_PASSPORT(py_str("gov.uk's own page states it, and the passport office will confirm it on the phone"));
    s_A_THING_VERB_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:renew|renewing|cancel|cancelling|canceled|refund|rebook|upgrade|print|return)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHO(py_dict(vec![(py_str("tenancy"), py_str("Shelter's advice line or your council's tenancy team will tell you what the rule is")), (py_str("pet"), py_str("your vet's reception will answer that in one call, and the breeder or rescue you got them from knows the rest")), (py_str("car"), py_str("a garage will tell you over the phone, and the DVLA's own page has the rule")), (py_str("bills"), py_str("your supplier will say what their own rules are, and Citizens Advice has the free script for asking them")), (py_str("health"), py_str("a pharmacist will settle it across the counter, and 111 if it cannot wait")), (py_str("work"), py_str("ACAS will tell you where you stand in one free call, and your contract says what was agreed")), (py_str("school"), py_str("the school office will tell you, and the exam board's specification says it in writing")), (py_str("home"), py_str("the manual for that model says what it does, and a plumber will confirm it on the phone")), (py_str("diy"), py_str("the instructions that came with it say, and anyone behind the counter at a builders' merchant will tell you")), (py_str("travel"), py_str("the operator's own site states it, and the airline or the hotel will confirm it in a message")), (py_str("shopping"), py_str("the shop's own page states it, and their chat will confirm it in writing")), (py_str("money"), py_str("your bank or the office that runs it has the exact rule in writing"))]));
    s_SHAPES(py_dict(vec![(py_str("howlong"), py_str("{subject}, then. {on} I can't put a time on that from here; {who}.")), (py_str("yesno"), py_str("{subject}, then. {on} I can't say yes or no from here; {who}.")), (py_str("how"), py_str("{subject}, then. {on} I can't talk you through it without seeing it; {who}.")), (py_str("what"), py_str("{subject}, then. {on} That one is past what I know well; {who}.")), (py_str("words"), py_str("{subject}, then. Keep it to three lines, say what you want and by when; {who}.")), (py_str("howmuch"), py_str("I haven't got a figure for {subject}, and a guessed one is worse than none. {on} {who}."))]));
    s_A_OPEN_ASK(stdlib_call("re", "compile", &[py_str("\\bwhat (?:do|should|shall|can|could|would) (?:i|we|you|they) do\\b|\\bwhat (?:happens|now|next)\\b|\\bwhat does (?:that|it|this) mean\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_KNOWS_IT(Py::Float(0.6));
    s_A_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_TURNS_ON(py_dict(vec![(py_str("tenancy"), py_str("what settles it is the agreement and what is in writing")), (py_str("pet"), py_str("what settles it is the age and the breed")), (py_str("car"), py_str("what settles it is the age, the mileage and the last MOT")), (py_str("bills"), py_str("what settles it is the account and what the supplier has already been told")), (py_str("health"), py_str("what settles it is how long it has gone on and what has already been tried")), (py_str("work"), py_str("what settles it is the dates and what was put in writing")), (py_str("school"), py_str("what settles it is the subject, the board and the dates")), (py_str("home"), py_str("what settles it is the model and when it started")), (py_str("diy"), py_str("what settles it is the size and what it is fixed to")), (py_str("travel"), py_str("what settles it is the booking reference and the dates")), (py_str("shopping"), py_str("what settles it is the receipt and the date you bought it")), (py_str("money"), py_str("what settles it is the account and the figures"))]));
    s_A_THERE(stdlib_call("re", "compile", &[py_str("\\bthere\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data12() -> Py { py_tuple(vec![py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("jan"), py_str("feb"), py_str("mar"), py_str("apr"), py_str("jun"), py_str("jul"), py_str("aug"), py_str("sep"), py_str("sept"), py_str("oct"), py_str("nov"), py_str("dec"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("week"), py_str("weeks"), py_str("weekend"), py_str("month"), py_str("months"), py_str("year"), py_str("years"), py_str("day"), py_str("days"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("night"), py_str("spring"), py_str("summer"), py_str("autumn"), py_str("winter"), py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("it"), py_str("them"), py_str("all"), py_str("some"), py_str("any"), py_str("no"), py_str("one"), py_str("two"), py_str("three"), py_str("ten"), py_str("twenty"), py_str("future"), py_str("past"), py_str("total"), py_str("general"), py_str("fact"), py_str("particular"), py_str("touch"), py_str("writing"), py_str("person"), py_str("which"), py_str("what"), py_str("order"), py_str("case"), py_str("time"), py_str("front"), py_str("bed"), py_str("hospital"), py_str("work"), py_str("school")]) }
    s_A_NOT_A_PLACE(call_builtin("frozenset", &[__data12()]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAMED_PLACE(stdlib_call("re", "compile", &[py_str("\\b(?:in|to|at|from|visiting|visit)\\s+([a-z][\\w'-]*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_REPEAT_LEAD(stdlib_call("re", "compile", &[py_str("^same (?:as before|answer)\\b[^:]{0,40}:"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_EXISTENTIAL_THERE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|isn't|aren't|wasn't|weren't)\\s+there\\b|\\bthere(?:'s| is| are| was| were)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
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
    let mut line: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    return stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _bare_follow_up(args: &[Py]) -> Py {
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
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The line when it is a short ask about something said earlier, else None.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[_words(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_ASK_OPENER(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&mcall(&text.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } }))) {
    return Py::None;
    }
    subject = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[text.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STOP(), &w.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&op_gt(&call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_SUBJECT_WORD_LIMIT())) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_BACK_REFERENCE(), "search", &[text.clone()])))) {
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_A_EMPTY_ASK_WORDS()); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[_words(&[text.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_A_EMPTY_ASK_LENGTH()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_EMPTY_ASK_OPENER(), "match", &[text.clone()]))) } } } }) {
    return Py::None;
    }
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
    let mut suffix: Py = Py::None;
    let _ = py_str("The part of a word that stays the same when it is inflected.\n\n    A KINDS key and a word in the line have to meet by stem or \"sack\" misses \"sacked\", which is\n    the shape that sent a person to a garage: \"the car passed its mot\" / \"can they sack me now\"\n    answered \"The mot, then ... the DVLA's own page has the rule\". So the key \"sacked\" and the\n    word \"sack\" are the same word here, and so are \"tyre\"/\"tyres\", \"bug\"/\"bugs\".\n    ");
    word = mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "strip", &[py_str("'")]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ies"), py_str("ied"), py_str("es"), py_str("ed"), py_str("s")]));
    for __item in __iter1 {
    suffix = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &op_add(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[suffix.clone()]) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&call_builtin("len", &[suffix.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    break;
    }
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[py_str("e")]) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return word.clone();
    Py::None
}
pub fn _only_a_place(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut before: Py = Py::None;
    let _ = py_str("True when the word at index only ever appears as the place: \"at work\", \"in a flat\".");
    before = if py_truthy(&op_gt(&index.clone(), &Py::Int(0))) { geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&g_A_PLACE_PREP(), &before.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_DETERMINER(), &before.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_PLACE_PREP(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(2))))) } } } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _beside(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut allowed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let _ = py_str("True when the word either side of this one is in allowed: \"washing machine\", \"a drum wash\".");
    if py_truthy(&{ let __b = op_gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&allowed.clone(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    return { let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&allowed.clone(), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))))) } };
    Py::None
}
pub fn _named(args: &[Py]) -> Py {
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
    let mut skip_place_words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip_place_words") { v.clone() }
    else { Py::Bool(false) };
    let mut light: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "light") { v.clone() }
    else { Py::Bool(true) };
    let mut found: Py = Py::None;
    let mut index: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut neighbour: Py = Py::None;
    let mut places: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("[(the word from KINDS that the line says, the kind)] for one line, in KINDS order.");
    words = _words(&[line.clone()]);
    said = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    let _ = mcall(&mcall(&said.clone(), "setdefault", &[_stem(&[word.clone()]), py_list(vec![])]), "append", &[index.clone()]);
    }
    found = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&g_KINDS());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk4[0].clone();
    keys = __unpk4[1].clone();
    let __iter5 = py_iter_or_raise(&keys.clone());
    for __item in __iter5 {
    key = __item.clone();
    places = mcall(&said.clone(), "get", &[_stem(&[key.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&places.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = light.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_A_LIGHT_KEY(), &key.clone())) } }) {
    continue;
    }
    neighbour = mcall(&g_A_NEEDS_A_NEIGHBOUR(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = neighbour.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut index: Py = __item.clone(); __out.push(_beside(&[words.clone(), index.clone(), neighbour.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = skip_place_words.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_PLACE_WORD(), &key.clone())); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut index: Py = __item.clone(); __out.push(_only_a_place(&[words.clone(), index.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![key.clone(), kind.clone()])]);
    break;
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_subject(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut _word: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_kind: Py = Py::None;
    let mut newest: Py = Py::None;
    let mut now: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_things: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("(the word the chat is on, the kind of thing it is, the line it came from) or blanks.");
    now = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    lines = py_list(vec![]);
    topic = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("topic")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&topic.clone(), "get", &[py_str("text")]) } }) {
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[geti(&topic.clone(), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&said.clone(), &now.clone())) } }) {
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[geti(&topic.clone(), &py_str("text"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_list(vec![]) };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(5)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&turn.clone(), "get", &[py_str("user")]))) } }) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&said.clone(), &now.clone())) } }) {
    let _ = mcall(&lines.clone(), "append", &[call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    said_things = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&turn.clone(), "get", &[py_str("user")]))) } }) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&said.clone(), &now.clone())); if py_truthy(&__b) { __b } else { mcall(&g_A_A_QUESTION_ASK(), "match", &[said.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_A_EXPLANATION(), "search", &[said.clone()])) {
    continue;
    }
    let _ = mcall(&said_things.clone(), "append", &[call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[said_things.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said_things.clone()))) {
    return py_tuple(vec![py_str(""), py_str(""), py_str("")]);
    }
    newest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&said_things.clone(), &Py::Int(0))); __kws.extend(vec![("skip_place_words".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named(args))(&__args) };
    if py_truthy(&op_gt(&call_builtin("len", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&newest.clone()) { let __unpk5429 = py_unpack(&__item.clone(), 2, None); let mut _word: Py = __unpk5429[0].clone(); let mut kind: Py = __unpk5429[1].clone(); __out.push(kind.clone()); } py_set(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    let __iter3 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(now.clone()); __kws.extend(vec![("skip_place_words".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named(args))(&__args) });
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    word = __unpk4[0].clone();
    kind = __unpk4[1].clone();
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&newest.clone()) { let __unpk3542 = py_unpack(&__item.clone(), 2, None); let mut _word: Py = __unpk3542[0].clone(); let mut named_kind: Py = __unpk3542[1].clone(); __out.push(named_kind.clone()); } py_set(__out) })() }, &kind.clone()))) {
    return py_tuple(vec![word.clone(), kind.clone(), now.clone()]);
    }
    }
    }
    let __iter5 = py_iter_or_raise(&said_things.clone());
    for __item in __iter5 {
    line = __item.clone();
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __kws.extend(vec![("skip_place_words".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named(args))(&__args) };
    if py_truthy(&named.clone()) {
    let __unpk6 = py_unpack(&geti(&named.clone(), &Py::Int(0)), 2, None);
    word = __unpk6[0].clone();
    kind = __unpk6[1].clone();
    return py_tuple(vec![word.clone(), kind.clone(), line.clone()]);
    }
    }
    return py_tuple(vec![py_str(""), py_str(""), py_str("")]);
    Py::None
}
pub fn _already_the_who(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut each: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("True when the reply's own place is this word, so saying it back would repeat it.\n\n    \"the pharmacist gave me tablets for the rash\" / \"how often do i take them\" said back as \"The\n    pharmacist, then ... a pharmacist will settle it across the counter\" repeats the place inside\n    one breath. The thing beside it in the same line is the better subject.\n    ");
    stem = _stem(&[word.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_words(&[{ let __b = mcall(&g_WHO(), "get", &[kind.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut each: Py = __item.clone(); __out.push(Py::Bool(py_eq(&_stem(&[each.clone()]), &stem.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _beside_the_who(args: &[Py]) -> Py {
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
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut subject: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "subject") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: subject"))) };
    let mut by_stem: Py = Py::None;
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Another word of the same kind in the line, when the subject is already the reply's place.\n\n    Only a thing will do: a place name is the other half of the sentence (\"we booked a hotel in\n    rome\" is not a figure for Rome), and a light key is a word the line says in passing.\n    ");
    keys = { let __b = mcall(&call_builtin("dict", &[g_KINDS()]).unwrap_or_else(|e| exc::raise(e)), "get", &[kind.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    by_stem = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&keys.clone());
    for __item in __iter1 {
    key = __item.clone();
    let _ = mcall(&by_stem.clone(), "setdefault", &[_stem(&[key.clone()]), key.clone()]);
    }
    let __iter2 = py_iter_or_raise(&_words(&[line.clone()]));
    for __item in __iter2 {
    word = __item.clone();
    key = mcall(&by_stem.clone(), "get", &[_stem(&[word.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&key.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&key.clone(), &subject.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_A_LIGHT_KEY(), &key.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PLACES(), &key.clone())) } } } } } }) {
    continue;
    }
    if py_truthy(&_already_the_who(&[key.clone(), kind.clone()])) {
    continue;
    }
    return key.clone();
    }
    return py_str("");
    Py::None
}
pub fn _shape(args: &[Py]) -> Py {
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
    if py_truthy(&mcall(&g_A_WORDS_ASK(), "match", &[text.clone()])) {
    return py_str("words");
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bhow long\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow soon\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("howlong");
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bhow much\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow many\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("howmuch");
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("^\\s*how\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("how");
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("^\\s*(?:what|which|who|when|where)\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("what");
    }
    return py_str("yesno");
    Py::None
}
pub fn _content(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a sentence that carry its meaning, for telling it from one already said.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_A_WORD(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])])) { let mut word: Py = __item.clone(); if py_truthy(&op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _already_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut mine: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the bot has said most of this sentence already, in its last few replies.");
    mine = _content(&[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mine.clone()))) {
    return Py::Bool(true);
    }
    seen = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&call_builtin("list", &[{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&seen.clone(), "update", &[_content(&[mcall(&turn.clone(), "get", &[py_str("bot")])])]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&seen.clone()))) {
    return Py::Bool(false);
    }
    return op_ge(&call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mine.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&seen.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &op_mul(&g_A_KNOWS_IT(), &call_builtin("len", &[mine.clone()]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _one_sentence(args: &[Py]) -> Py {
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
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(250) };
    let mut at: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stop: Py = Py::None;
    let _ = py_str("The first sentence of an answer, cut at a clause if it runs long, for a short reply.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    stop = mcall(&said.clone(), "find", &[py_str(". ")]);
    if py_truthy(&{ let __cmp11: Py = stop.clone(); if py_truthy(&op_lt(&Py::Int(40), &__cmp11)) { op_lt(&__cmp11, &limit.clone()) } else { Py::Bool(false) } }) {
    said = slice_or_raise(&said.clone(), &Py::None, &op_add(&stop.clone(), &Py::Int(1)), &Py::None);
    }
    if py_truthy(&op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
    cut = mcall(&said.clone(), "rfind", &[py_str(", "), Py::Int(0), op_sub(&limit.clone(), &Py::Int(3))]);
    if py_truthy(&op_lt(&cut.clone(), &Py::Int(100))) {
    cut = mcall(&said.clone(), "rfind", &[py_str(" "), Py::Int(0), op_sub(&limit.clone(), &Py::Int(3))]);
    }
    said = op_add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &call_builtin("max", &[cut.clone(), Py::Int(100)]).unwrap_or_else(|e| exc::raise(e)), &Py::None), "rstrip", &[py_str(" ,;:-")]), &py_str(", and"));
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("\""), py_str("("), py_str("[")]));
    for __item in __iter2 {
    opener = __item.clone();
    if py_truthy(&op_modulo(&mcall(&said.clone(), "count", &[opener.clone()]), &Py::Int(2))) {
    at = mcall(&said.clone(), "rfind", &[opener.clone()]);
    if py_truthy(&op_gt(&at.clone(), &Py::Int(40))) {
    said = mcall(&slice_or_raise(&said.clone(), &Py::None, &at.clone(), &Py::None), "rstrip", &[py_str(" ,;:-")]);
    }
    }
    }
    return said.clone();
    Py::None
}
pub fn _cap(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let _ = py_str("A clause that starts a sentence gets a capital; one used mid-sentence does not.");
    clause = call_builtin("str", &[{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&clause.clone()) { op_add(&mcall(&geti(&clause.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { clause.clone() };
    Py::None
}
pub fn _the_place(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The place the subject's line names, when it names one: \"we are in spain\" -> \"spain\".");
    let __iter1 = py_iter_or_raise(&mcall(&g_A_NAMED_PLACE(), "finditer", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter1 {
    match_ = __item.clone();
    word = mcall(&match_.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_A_NOT_A_PLACE(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_STOP(), &word.clone())); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } }) {
    return word.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_answer_about_the_place(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: line"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut answer: Py = Py::None;
    let mut answers: Py = Py::None;
    let mut chat: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut first: Py = Py::None;
    let mut place: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut spliced: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What the bot knows about the ask with the place put back into it, or \"\".\n\n    \"do i need an adapter there\" is asked of the answers lane as \"do i need an adapter in spain\".\n    Only a real answer is taken: the search hedge back means no entry claims that question, and\n    the reply below is the better one after all.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_A_THERE(), "search", &[text.clone()])))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_A_EXISTENTIAL_THERE(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&_shape(&[text.clone()]), &py_str("howlong"))) } }) {
    return py_str("");
    }
    place = _the_place(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return py_str("");
    }
    spliced = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("in "), &place.clone())); __args.push(text.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_THERE(), "sub", args))(&__args) };
    chat = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    clean = py_dict(vec![(py_str("turns"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&chat.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() })]);
    let __try1 = exc::catch(|| {
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    answers = pyrt::xrefs::xmodule("modules.answers");
    answer = mcall(&answers.clone(), "respond", &[spliced.clone(), clean.clone()]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))) } }) {
    return py_str("");
    }
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&mcall(&fallback.clone(), "_in_a_pool", &[call_builtin("str", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![gattr(&fallback.clone(), "UNKNOWN")])])) {
    return py_str("");
    }
    first = _one_sentence(&[call_builtin("str", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    rest = mcall(&slice_or_raise(&call_builtin("str", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    second = _one_sentence(&[rest.clone()]);
    said = mcall(&op_add(&first.clone(), &if py_truthy(&second.clone()) { op_add(&py_str(" "), &second.clone()) } else { py_str("") }), "strip", &[]);
    if py_truthy(&mcall(&g_A_REPEAT_LEAD(), "match", &[said.clone()])) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_contains(&mcall(&said.clone(), "lower", &[]), &mcall(&place.clone(), "lower", &[])))) {
    return py_str("");
    }
    return said.clone();
    Py::None
}
pub fn _what_it_knows(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut lookup: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut understand: Py = Py::None;
    let _ = py_str("One sentence the knowledge base has about the thing the chat is on, or \"\".\n\n    `line` is the line the subject came from, as the person said it. The entry is looked up by\n    that line's own reading, so the words stay in the knowledge base and nothing about the world\n    is written here. A sentence the bot has already said is no good to the person twice, so the\n    second one is used when the first is on their screen, and \"\" when the whole answer is.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return py_str("");
    }
    let __try1 = exc::catch(|| {
    lookup = pyrt::xrefs::xmodule("knowledge.lookup");
    understand = py_func("understand", |args: &[Py]| pyrt::xrefs::xcall("understanding", "understand", args));
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    reading = py_call(&understand, &[line.clone()]);
    entry = mcall(&lookup.clone(), "find", &[reading.clone()]);
    if py_truthy(&Py::Bool(py_is(&entry.clone(), &Py::None))) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    answer = gattr(&entry.clone(), "reply");
    if py_truthy(&call_builtin("callable", &[answer.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    answer = py_call(&answer, &[py_dict(vec![]), reading.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[answer.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&answer.clone(), "strip", &[]))) } }) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
    }
    if py_truthy(&mcall(&fallback.clone(), "_in_a_pool", &[answer.clone(), py_tuple(vec![gattr(&fallback.clone(), "UNKNOWN")])])) {
    return py_tuple(vec![Py::Bool(true), py_str("")]);
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let __iter6 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&py_str(" "), "join", &[mcall(&answer.clone(), "split", &[])])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[sentence.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(40)); if py_truthy(&__b) { __b } else { _already_said(&[sentence.clone(), memory.clone()]) } }) {
    continue;
    }
    return _one_sentence(&[sentence.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _the_line_before(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut chat: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "chat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: chat"))) };
    let mut now: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "now") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: now"))) };
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The newest line the person said that names a place, and is not the ask itself.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&chat.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&turn.clone(), "get", &[py_str("user")]))) } }) {
    continue;
    }
    said = call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]), &now.clone()))) {
    continue;
    }
    if py_truthy(&_the_place(&[said.clone()])) {
    return said.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn place_pool(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut answer: Py = Py::None;
    let mut chat: Py = Py::None;
    let mut fallback: Py = Py::None;
    let mut line: Py = Py::None;
    let mut place: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A there-ask about the place the chat named, where the line names no subject at all.\n\n    \"were off to orlando in october\" / \"do i need an adapter there\" reaches no key: the reading\n    knows no subject word in Orlando, so `pool` passes and the seed fallback gives the search\n    hedge. The place is the whole of what that question needs, so the place goes back into the\n    ask and the answers lane answers it. It answers only where the seed would have hedged, so\n    this is still a last resort, and nothing is said that the knowledge base does not say.\n    ");
    text = _bare_follow_up(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&text.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_THERE(), "search", &[text.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_A_EXISTENTIAL_THERE(), "search", &[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&_shape(&[text.clone()]), &py_str("howlong")))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_POINTER_ASK(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HELP_ASK(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_JUDGEMENT_ASK(), "search", &[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_RECALL_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_COMPARISON_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_COOKING_ASK(), "search", &[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_NAMING_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_ITS_OWN_SUBJECT(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    chat = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { memory.clone() } else { py_dict(vec![]) };
    let __try1 = exc::catch(|| {
    fallback = pyrt::xrefs::xmodule("compose.fallback");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&fallback.clone(), "_in_a_pool", &[mcall(&fallback.clone(), "reply_for", &[clean.clone(), chat.clone()]), py_tuple(vec![gattr(&fallback.clone(), "UNKNOWN")])])))) {
    return py_tuple(vec![Py::Bool(true), Py::None]);
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    line = _the_line_before(&[chat.clone(), text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    answer = _the_answer_about_the_place(&[text.clone(), line.clone(), chat.clone()]);
    place = _the_place(&[line.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&place.clone())) } }) {
    return Py::None;
    }
    return op_modulo(&py_str("%s, then. %s"), &py_tuple(vec![_cap(&[place.clone()]), answer.clone()]));
    Py::None
}
pub fn _food_above(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("True when a recent line is already about food, so the chat can answer this one.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(6)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_A_COOKING_ASK(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_NAMED_FOOD(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_ROASTING_WORD(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut _word: Py = Py::None;
    let mut ends: Py = Py::None;
    let mut join: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut known: Py = Py::None;
    let mut line: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut named_kind: Py = Py::None;
    let mut on: Py = Py::None;
    let mut place: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut shape_kind: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turns_on: Py = Py::None;
    let mut who: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The honest line for a follow-up whose subject is only in the chat, or None.");
    text = _bare_follow_up(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&text.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_A_POINTER_ASK(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_HELP_ASK(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_JUDGEMENT_ASK(), "search", &[text.clone()]) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_NAMING_ASK(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_ITS_OWN_SUBJECT(), "search", &[text.clone()])) {
    return Py::None;
    }
    subject = geti(&_the_subject(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, text.clone()]), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&subject.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_COOKING_TIME_ASK(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_FOOD_PRONOUN(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_A_NAMED_FOOD(), "search", &[text.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_food_above(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]))) } } } } } } } }) {
    if py_truthy(&mcall(&g_A_ROASTING_WORD(), "search", &[text.clone()])) {
    return geti(&g_COOKING_TIME_SHAPES(), &py_str("oven"));
    }
    if py_truthy(&mcall(&g_A_BOILING_WORD(), "search", &[text.clone()])) {
    return geti(&g_COOKING_TIME_SHAPES(), &py_str("boil"));
    }
    if py_truthy(&{ let __b = mcall(&g_A_COOKING_ASK(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_OTHER_METHOD(), "search", &[text.clone()]))) } }) {
    return geti(&g_COOKING_TIME_SHAPES(), &py_str("either"));
    }
    }
    if py_truthy(&{ let __b = mcall(&g_A_RECALL_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_COMPARISON_ASK(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_COOKING_ASK(), "search", &[text.clone()]) } } } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_the_subject(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, text.clone()]), 3, None);
    subject = __unpk1[0].clone();
    kind = __unpk1[1].clone();
    line = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&subject.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&line.clone(), &text.clone()))) {
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("skip_place_words".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named(args))(&__args) }) { let __unpk2772 = py_unpack(&__item.clone(), 2, None); let mut _word: Py = __unpk2772[0].clone(); let mut named_kind: Py = __unpk2772[1].clone(); __out.push(named_kind.clone()); } py_list(__out) })() };
    if py_truthy(&{ let __b = mine.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mine.clone(), &kind.clone())) } }) {
    return Py::None;
    }
    }
    who = mcall(&g_WHO(), "get", &[kind.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AMOUNT_KINDS(), &kind.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_JUDGEMENT_OF_AMOUNT(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MACHINE_BEFORE_MACHINE_KINDS(), &kind.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_MACHINE_CUE(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_LOST_THING(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    if py_truthy(&_already_the_who(&[subject.clone(), kind.clone()])) {
    subject = { let __b = _beside_the_who(&[line.clone(), kind.clone(), subject.clone()]); if py_truthy(&__b) { __b } else { subject.clone() } };
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACES(), &subject.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_A_THING_VERB_ASK(), "search", &[text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_A_PASSPORT_WORD(), &subject.clone()))) {
    who = g_WHO_PASSPORT();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("car"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_A_DVLA_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_DVLA_WORD(), "search", &[call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } })) } }) {
    who = g_WHO_CAR_GARAGE();
    }
    said = if py_truthy(&Py::Bool(py_contains(&g_PLACES(), &subject.clone()))) { subject.clone() } else { op_add(&py_str("the "), &mcall(&g_A_SHOWN_AS(), "get", &[subject.clone(), subject.clone()])) };
    shape_kind = _shape(&[text.clone()]);
    shape = geti(&g_SHAPES(), &shape_kind.clone());
    if py_truthy(&mcall(&shape.clone(), "startswith", &[py_str("{subject}")])) {
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    known = py_str("");
    if py_truthy(&{ let __b = mcall(&g_A_OPEN_ASK(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("skip_place_words".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _named(args))(&__args) })) } }) {
    known = _what_it_knows(&[line.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    }
    whole = py_str("");
    if py_truthy(&Py::Bool(!py_truthy(&known.clone()))) {
    whole = _the_answer_about_the_place(&[text.clone(), line.clone(), { let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    }
    if py_truthy(&whole.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", then. "); __s.push_str(&fstr(&whole.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&known.clone()) {
    ends = Py::Bool(py_contains(&py_str(".!?"), &slice_or_raise(&known.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)));
    join = if py_truthy(&ends.clone()) { py_str(" ") } else { py_str("; ") };
    place = if py_truthy(&ends.clone()) { _cap(&[who.clone()]) } else { who.clone() };
    if py_truthy(&Py::Bool(py_eq(&shape_kind.clone(), &py_str("howmuch")))) {
    return { let mut __s = String::new(); __s.push_str("I haven't got a figure for "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", and a guessed one is worse than none - "); __s.push_str(&fstr(&known.clone(), -1, "")); __s.push_str(&fstr(&join.clone(), -1, "")); __s.push_str(&fstr(&place.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", then. "); __s.push_str(&fstr(&known.clone(), -1, "")); __s.push_str(&fstr(&join.clone(), -1, "")); __s.push_str(&fstr(&place.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    turns_on = mcall(&g_A_TURNS_ON(), "get", &[kind.clone()]);
    on = py_str("");
    if py_truthy(&{ let __b = turns_on.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&shape_kind.clone(), &py_str("words"))) } }) {
    on = op_add(&op_add(&mcall(&geti(&turns_on.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&turns_on.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("subject".to_string(), said.clone()), ("on".to_string(), on.clone()), ("who".to_string(), who.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&shape.clone(), "format", args))(&__args) };
    return mcall(&mcall(&stdlib_call("re", "sub", &[py_str("\\s+"), py_str(" "), reply.clone()]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str(" ."), py_str(".")]), "strip", &[]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.onsubject".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AMOUNT_KINDS" => Some(g_AMOUNT_KINDS()),
            "A_ASK_OPENER" => Some(g_A_ASK_OPENER()),
            "A_A_QUESTION_ASK" => Some(g_A_A_QUESTION_ASK()),
            "A_BACK_REFERENCE" => Some(g_A_BACK_REFERENCE()),
            "A_BOILING_WORD" => Some(g_A_BOILING_WORD()),
            "A_COMPARISON_ASK" => Some(g_A_COMPARISON_ASK()),
            "A_COOKING_ASK" => Some(g_A_COOKING_ASK()),
            "A_COOKING_TIME_ASK" => Some(g_A_COOKING_TIME_ASK()),
            "A_DETERMINER" => Some(g_A_DETERMINER()),
            "A_DVLA_WORD" => Some(g_A_DVLA_WORD()),
            "A_EMPTY_ASK_LENGTH" => Some(g_A_EMPTY_ASK_LENGTH()),
            "A_EMPTY_ASK_OPENER" => Some(g_A_EMPTY_ASK_OPENER()),
            "A_EMPTY_ASK_WORDS" => Some(g_A_EMPTY_ASK_WORDS()),
            "A_EXISTENTIAL_THERE" => Some(g_A_EXISTENTIAL_THERE()),
            "A_EXPLANATION" => Some(g_A_EXPLANATION()),
            "A_FOOD_PRONOUN" => Some(g_A_FOOD_PRONOUN()),
            "A_HELP_ASK" => Some(g_A_HELP_ASK()),
            "A_ITS_OWN_SUBJECT" => Some(g_A_ITS_OWN_SUBJECT()),
            "A_JUDGEMENT_ASK" => Some(g_A_JUDGEMENT_ASK()),
            "A_JUDGEMENT_OF_AMOUNT" => Some(g_A_JUDGEMENT_OF_AMOUNT()),
            "A_KNOWS_IT" => Some(g_A_KNOWS_IT()),
            "A_LIGHT_KEY" => Some(g_A_LIGHT_KEY()),
            "A_LOST_THING" => Some(g_A_LOST_THING()),
            "A_MACHINE_CUE" => Some(g_A_MACHINE_CUE()),
            "A_NAMED_FOOD" => Some(g_A_NAMED_FOOD()),
            "A_NAMED_PLACE" => Some(g_A_NAMED_PLACE()),
            "A_NAMING_ASK" => Some(g_A_NAMING_ASK()),
            "A_NEEDS_A_NEIGHBOUR" => Some(g_A_NEEDS_A_NEIGHBOUR()),
            "A_NOT_A_PLACE" => Some(g_A_NOT_A_PLACE()),
            "A_OPEN_ASK" => Some(g_A_OPEN_ASK()),
            "A_OTHER_METHOD" => Some(g_A_OTHER_METHOD()),
            "A_PASSPORT_WORD" => Some(g_A_PASSPORT_WORD()),
            "A_PLACE_PREP" => Some(g_A_PLACE_PREP()),
            "A_PLACE_WORD" => Some(g_A_PLACE_WORD()),
            "A_POINTER_ASK" => Some(g_A_POINTER_ASK()),
            "A_RECALL_ASK" => Some(g_A_RECALL_ASK()),
            "A_REPEAT_LEAD" => Some(g_A_REPEAT_LEAD()),
            "A_ROASTING_WORD" => Some(g_A_ROASTING_WORD()),
            "A_SHOWN_AS" => Some(g_A_SHOWN_AS()),
            "A_SUBJECT_WORD_LIMIT" => Some(g_A_SUBJECT_WORD_LIMIT()),
            "A_THERE" => Some(g_A_THERE()),
            "A_THING_VERB_ASK" => Some(g_A_THING_VERB_ASK()),
            "A_TURNS_ON" => Some(g_A_TURNS_ON()),
            "A_WORD" => Some(g_A_WORD()),
            "A_WORDS_ASK" => Some(g_A_WORDS_ASK()),
            "COOKING_TIME_SHAPES" => Some(g_COOKING_TIME_SHAPES()),
            "KINDS" => Some(g_KINDS()),
            "MACHINE_BEFORE_MACHINE_KINDS" => Some(g_MACHINE_BEFORE_MACHINE_KINDS()),
            "PLACES" => Some(g_PLACES()),
            "SHAPES" => Some(g_SHAPES()),
            "STOP" => Some(g_STOP()),
            "WHO" => Some(g_WHO()),
            "WHO_CAR_GARAGE" => Some(g_WHO_CAR_GARAGE()),
            "WHO_PASSPORT" => Some(g_WHO_PASSPORT()),
            "_already_said" => Some(py_func("_already_said", |args: &[Py]| _already_said(args))),
            "_already_the_who" => Some(py_func("_already_the_who", |args: &[Py]| _already_the_who(args))),
            "_bare_follow_up" => Some(py_func("_bare_follow_up", |args: &[Py]| _bare_follow_up(args))),
            "_beside" => Some(py_func("_beside", |args: &[Py]| _beside(args))),
            "_beside_the_who" => Some(py_func("_beside_the_who", |args: &[Py]| _beside_the_who(args))),
            "_cap" => Some(py_func("_cap", |args: &[Py]| _cap(args))),
            "_content" => Some(py_func("_content", |args: &[Py]| _content(args))),
            "_food_above" => Some(py_func("_food_above", |args: &[Py]| _food_above(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_one_sentence" => Some(py_func("_one_sentence", |args: &[Py]| _one_sentence(args))),
            "_only_a_place" => Some(py_func("_only_a_place", |args: &[Py]| _only_a_place(args))),
            "_shape" => Some(py_func("_shape", |args: &[Py]| _shape(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_the_answer_about_the_place" => Some(py_func("_the_answer_about_the_place", |args: &[Py]| _the_answer_about_the_place(args))),
            "_the_line_before" => Some(py_func("_the_line_before", |args: &[Py]| _the_line_before(args))),
            "_the_place" => Some(py_func("_the_place", |args: &[Py]| _the_place(args))),
            "_the_subject" => Some(py_func("_the_subject", |args: &[Py]| _the_subject(args))),
            "_what_it_knows" => Some(py_func("_what_it_knows", |args: &[Py]| _what_it_knows(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "place_pool" => Some(py_func("place_pool", |args: &[Py]| place_pool(args))),
            "pool" => Some(py_func("pool", |args: &[Py]| pool(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AMOUNT_KINDS" => s_AMOUNT_KINDS(v),
            "A_ASK_OPENER" => s_A_ASK_OPENER(v),
            "A_A_QUESTION_ASK" => s_A_A_QUESTION_ASK(v),
            "A_BACK_REFERENCE" => s_A_BACK_REFERENCE(v),
            "A_BOILING_WORD" => s_A_BOILING_WORD(v),
            "A_COMPARISON_ASK" => s_A_COMPARISON_ASK(v),
            "A_COOKING_ASK" => s_A_COOKING_ASK(v),
            "A_COOKING_TIME_ASK" => s_A_COOKING_TIME_ASK(v),
            "A_DETERMINER" => s_A_DETERMINER(v),
            "A_DVLA_WORD" => s_A_DVLA_WORD(v),
            "A_EMPTY_ASK_LENGTH" => s_A_EMPTY_ASK_LENGTH(v),
            "A_EMPTY_ASK_OPENER" => s_A_EMPTY_ASK_OPENER(v),
            "A_EMPTY_ASK_WORDS" => s_A_EMPTY_ASK_WORDS(v),
            "A_EXISTENTIAL_THERE" => s_A_EXISTENTIAL_THERE(v),
            "A_EXPLANATION" => s_A_EXPLANATION(v),
            "A_FOOD_PRONOUN" => s_A_FOOD_PRONOUN(v),
            "A_HELP_ASK" => s_A_HELP_ASK(v),
            "A_ITS_OWN_SUBJECT" => s_A_ITS_OWN_SUBJECT(v),
            "A_JUDGEMENT_ASK" => s_A_JUDGEMENT_ASK(v),
            "A_JUDGEMENT_OF_AMOUNT" => s_A_JUDGEMENT_OF_AMOUNT(v),
            "A_KNOWS_IT" => s_A_KNOWS_IT(v),
            "A_LIGHT_KEY" => s_A_LIGHT_KEY(v),
            "A_LOST_THING" => s_A_LOST_THING(v),
            "A_MACHINE_CUE" => s_A_MACHINE_CUE(v),
            "A_NAMED_FOOD" => s_A_NAMED_FOOD(v),
            "A_NAMED_PLACE" => s_A_NAMED_PLACE(v),
            "A_NAMING_ASK" => s_A_NAMING_ASK(v),
            "A_NEEDS_A_NEIGHBOUR" => s_A_NEEDS_A_NEIGHBOUR(v),
            "A_NOT_A_PLACE" => s_A_NOT_A_PLACE(v),
            "A_OPEN_ASK" => s_A_OPEN_ASK(v),
            "A_OTHER_METHOD" => s_A_OTHER_METHOD(v),
            "A_PASSPORT_WORD" => s_A_PASSPORT_WORD(v),
            "A_PLACE_PREP" => s_A_PLACE_PREP(v),
            "A_PLACE_WORD" => s_A_PLACE_WORD(v),
            "A_POINTER_ASK" => s_A_POINTER_ASK(v),
            "A_RECALL_ASK" => s_A_RECALL_ASK(v),
            "A_REPEAT_LEAD" => s_A_REPEAT_LEAD(v),
            "A_ROASTING_WORD" => s_A_ROASTING_WORD(v),
            "A_SHOWN_AS" => s_A_SHOWN_AS(v),
            "A_SUBJECT_WORD_LIMIT" => s_A_SUBJECT_WORD_LIMIT(v),
            "A_THERE" => s_A_THERE(v),
            "A_THING_VERB_ASK" => s_A_THING_VERB_ASK(v),
            "A_TURNS_ON" => s_A_TURNS_ON(v),
            "A_WORD" => s_A_WORD(v),
            "A_WORDS_ASK" => s_A_WORDS_ASK(v),
            "COOKING_TIME_SHAPES" => s_COOKING_TIME_SHAPES(v),
            "KINDS" => s_KINDS(v),
            "MACHINE_BEFORE_MACHINE_KINDS" => s_MACHINE_BEFORE_MACHINE_KINDS(v),
            "PLACES" => s_PLACES(v),
            "SHAPES" => s_SHAPES(v),
            "STOP" => s_STOP(v),
            "WHO" => s_WHO(v),
            "WHO_CAR_GARAGE" => s_WHO_CAR_GARAGE(v),
            "WHO_PASSPORT" => s_WHO_PASSPORT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.onsubject", module_obj);
    pyrt::xrefs::reg_fn("compose.onsubject", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_bare_follow_up", py_func("_bare_follow_up", |args: &[Py]| _bare_follow_up(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_stem", py_func("_stem", |args: &[Py]| _stem(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_only_a_place", py_func("_only_a_place", |args: &[Py]| _only_a_place(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_beside", py_func("_beside", |args: &[Py]| _beside(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_the_subject", py_func("_the_subject", |args: &[Py]| _the_subject(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_already_the_who", py_func("_already_the_who", |args: &[Py]| _already_the_who(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_beside_the_who", py_func("_beside_the_who", |args: &[Py]| _beside_the_who(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_shape", py_func("_shape", |args: &[Py]| _shape(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_content", py_func("_content", |args: &[Py]| _content(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_already_said", py_func("_already_said", |args: &[Py]| _already_said(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_one_sentence", py_func("_one_sentence", |args: &[Py]| _one_sentence(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_cap", py_func("_cap", |args: &[Py]| _cap(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_the_place", py_func("_the_place", |args: &[Py]| _the_place(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_the_answer_about_the_place", py_func("_the_answer_about_the_place", |args: &[Py]| _the_answer_about_the_place(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_what_it_knows", py_func("_what_it_knows", |args: &[Py]| _what_it_knows(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_the_line_before", py_func("_the_line_before", |args: &[Py]| _the_line_before(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "place_pool", py_func("place_pool", |args: &[Py]| place_pool(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "_food_above", py_func("_food_above", |args: &[Py]| _food_above(args)));
    pyrt::xrefs::reg_fn("compose.onsubject", "pool", py_func("pool", |args: &[Py]| pool(args)));
    pyrt::xrefs::reg_access("compose.onsubject", "AMOUNT_KINDS", g_AMOUNT_KINDS, s_AMOUNT_KINDS);
    pyrt::xrefs::reg_access("compose.onsubject", "A_ASK_OPENER", g_A_ASK_OPENER, s_A_ASK_OPENER);
    pyrt::xrefs::reg_access("compose.onsubject", "A_A_QUESTION_ASK", g_A_A_QUESTION_ASK, s_A_A_QUESTION_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_BACK_REFERENCE", g_A_BACK_REFERENCE, s_A_BACK_REFERENCE);
    pyrt::xrefs::reg_access("compose.onsubject", "A_BOILING_WORD", g_A_BOILING_WORD, s_A_BOILING_WORD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_COMPARISON_ASK", g_A_COMPARISON_ASK, s_A_COMPARISON_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_COOKING_ASK", g_A_COOKING_ASK, s_A_COOKING_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_COOKING_TIME_ASK", g_A_COOKING_TIME_ASK, s_A_COOKING_TIME_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_DETERMINER", g_A_DETERMINER, s_A_DETERMINER);
    pyrt::xrefs::reg_access("compose.onsubject", "A_DVLA_WORD", g_A_DVLA_WORD, s_A_DVLA_WORD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_EMPTY_ASK_LENGTH", g_A_EMPTY_ASK_LENGTH, s_A_EMPTY_ASK_LENGTH);
    pyrt::xrefs::reg_access("compose.onsubject", "A_EMPTY_ASK_OPENER", g_A_EMPTY_ASK_OPENER, s_A_EMPTY_ASK_OPENER);
    pyrt::xrefs::reg_access("compose.onsubject", "A_EMPTY_ASK_WORDS", g_A_EMPTY_ASK_WORDS, s_A_EMPTY_ASK_WORDS);
    pyrt::xrefs::reg_access("compose.onsubject", "A_EXISTENTIAL_THERE", g_A_EXISTENTIAL_THERE, s_A_EXISTENTIAL_THERE);
    pyrt::xrefs::reg_access("compose.onsubject", "A_EXPLANATION", g_A_EXPLANATION, s_A_EXPLANATION);
    pyrt::xrefs::reg_access("compose.onsubject", "A_FOOD_PRONOUN", g_A_FOOD_PRONOUN, s_A_FOOD_PRONOUN);
    pyrt::xrefs::reg_access("compose.onsubject", "A_HELP_ASK", g_A_HELP_ASK, s_A_HELP_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_ITS_OWN_SUBJECT", g_A_ITS_OWN_SUBJECT, s_A_ITS_OWN_SUBJECT);
    pyrt::xrefs::reg_access("compose.onsubject", "A_JUDGEMENT_ASK", g_A_JUDGEMENT_ASK, s_A_JUDGEMENT_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_JUDGEMENT_OF_AMOUNT", g_A_JUDGEMENT_OF_AMOUNT, s_A_JUDGEMENT_OF_AMOUNT);
    pyrt::xrefs::reg_access("compose.onsubject", "A_KNOWS_IT", g_A_KNOWS_IT, s_A_KNOWS_IT);
    pyrt::xrefs::reg_access("compose.onsubject", "A_LIGHT_KEY", g_A_LIGHT_KEY, s_A_LIGHT_KEY);
    pyrt::xrefs::reg_access("compose.onsubject", "A_LOST_THING", g_A_LOST_THING, s_A_LOST_THING);
    pyrt::xrefs::reg_access("compose.onsubject", "A_MACHINE_CUE", g_A_MACHINE_CUE, s_A_MACHINE_CUE);
    pyrt::xrefs::reg_access("compose.onsubject", "A_NAMED_FOOD", g_A_NAMED_FOOD, s_A_NAMED_FOOD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_NAMED_PLACE", g_A_NAMED_PLACE, s_A_NAMED_PLACE);
    pyrt::xrefs::reg_access("compose.onsubject", "A_NAMING_ASK", g_A_NAMING_ASK, s_A_NAMING_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_NEEDS_A_NEIGHBOUR", g_A_NEEDS_A_NEIGHBOUR, s_A_NEEDS_A_NEIGHBOUR);
    pyrt::xrefs::reg_access("compose.onsubject", "A_NOT_A_PLACE", g_A_NOT_A_PLACE, s_A_NOT_A_PLACE);
    pyrt::xrefs::reg_access("compose.onsubject", "A_OPEN_ASK", g_A_OPEN_ASK, s_A_OPEN_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_OTHER_METHOD", g_A_OTHER_METHOD, s_A_OTHER_METHOD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_PASSPORT_WORD", g_A_PASSPORT_WORD, s_A_PASSPORT_WORD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_PLACE_PREP", g_A_PLACE_PREP, s_A_PLACE_PREP);
    pyrt::xrefs::reg_access("compose.onsubject", "A_PLACE_WORD", g_A_PLACE_WORD, s_A_PLACE_WORD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_POINTER_ASK", g_A_POINTER_ASK, s_A_POINTER_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_RECALL_ASK", g_A_RECALL_ASK, s_A_RECALL_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_REPEAT_LEAD", g_A_REPEAT_LEAD, s_A_REPEAT_LEAD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_ROASTING_WORD", g_A_ROASTING_WORD, s_A_ROASTING_WORD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_SHOWN_AS", g_A_SHOWN_AS, s_A_SHOWN_AS);
    pyrt::xrefs::reg_access("compose.onsubject", "A_SUBJECT_WORD_LIMIT", g_A_SUBJECT_WORD_LIMIT, s_A_SUBJECT_WORD_LIMIT);
    pyrt::xrefs::reg_access("compose.onsubject", "A_THERE", g_A_THERE, s_A_THERE);
    pyrt::xrefs::reg_access("compose.onsubject", "A_THING_VERB_ASK", g_A_THING_VERB_ASK, s_A_THING_VERB_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "A_TURNS_ON", g_A_TURNS_ON, s_A_TURNS_ON);
    pyrt::xrefs::reg_access("compose.onsubject", "A_WORD", g_A_WORD, s_A_WORD);
    pyrt::xrefs::reg_access("compose.onsubject", "A_WORDS_ASK", g_A_WORDS_ASK, s_A_WORDS_ASK);
    pyrt::xrefs::reg_access("compose.onsubject", "COOKING_TIME_SHAPES", g_COOKING_TIME_SHAPES, s_COOKING_TIME_SHAPES);
    pyrt::xrefs::reg_access("compose.onsubject", "KINDS", g_KINDS, s_KINDS);
    pyrt::xrefs::reg_access("compose.onsubject", "MACHINE_BEFORE_MACHINE_KINDS", g_MACHINE_BEFORE_MACHINE_KINDS, s_MACHINE_BEFORE_MACHINE_KINDS);
    pyrt::xrefs::reg_access("compose.onsubject", "PLACES", g_PLACES, s_PLACES);
    pyrt::xrefs::reg_access("compose.onsubject", "SHAPES", g_SHAPES, s_SHAPES);
    pyrt::xrefs::reg_access("compose.onsubject", "STOP", g_STOP, s_STOP);
    pyrt::xrefs::reg_access("compose.onsubject", "WHO", g_WHO, s_WHO);
    pyrt::xrefs::reg_access("compose.onsubject", "WHO_CAR_GARAGE", g_WHO_CAR_GARAGE, s_WHO_CAR_GARAGE);
    pyrt::xrefs::reg_access("compose.onsubject", "WHO_PASSPORT", g_WHO_PASSPORT, s_WHO_PASSPORT);
}

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


thread_local! { static G_AM_I: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CYCLE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CYCLE_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRINKS_WITH_THEIR_OWN_ANSWER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EARLY_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FERTILITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEAD_PREGNANCY_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LATE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATIVE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSITIVE_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_ALREADY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANT_MOVEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREG_ANY_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREG_WORRY_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEST_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEST_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_THE_BIRTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AMBIGUOUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_WEEKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BIRTH_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MILESTONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PREGNANCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTH_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTH_WORD_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRTH_WORD_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_THEN_MONTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DO_THEY_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DUE_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DUE_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEEDING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GIVES_THE_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_FAR_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONTH_THEN_DAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MY_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BIRTH_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_BIRTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OTHER_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERIOD_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERIOD_WITH_A_PREGNANCY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POSTNATAL_BIRTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POSTNATAL_BLEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREG_DENIED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREG_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREG_SOFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PREG_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SECOND_BIRTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHORT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THEY_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEEKS_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHEN_TO_CALL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.pregnancy: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AM_I() -> Py { ensure_init(); G_AM_I.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AM_I(v: Py) { ensure_init(); G_AM_I.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD() -> Py { ensure_init(); G_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD(v: Py) { ensure_init(); G_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_CYCLE_REPLY() -> Py { ensure_init(); G_CYCLE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CYCLE_REPLY(v: Py) { ensure_init(); G_CYCLE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_CYCLE_SHAPE() -> Py { ensure_init(); G_CYCLE_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CYCLE_SHAPE(v: Py) { ensure_init(); G_CYCLE_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_DRINKS_WITH_THEIR_OWN_ANSWER() -> Py { ensure_init(); G_DRINKS_WITH_THEIR_OWN_ANSWER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRINKS_WITH_THEIR_OWN_ANSWER(v: Py) { ensure_init(); G_DRINKS_WITH_THEIR_OWN_ANSWER.with(|c| *c.borrow_mut() = v); }
pub fn g_EARLY_REPLY() -> Py { ensure_init(); G_EARLY_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EARLY_REPLY(v: Py) { ensure_init(); G_EARLY_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FERTILITY() -> Py { ensure_init(); G_FERTILITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FERTILITY(v: Py) { ensure_init(); G_FERTILITY.with(|c| *c.borrow_mut() = v); }
pub fn g_HEAD_PREGNANCY_REPLY() -> Py { ensure_init(); G_HEAD_PREGNANCY_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEAD_PREGNANCY_REPLY(v: Py) { ensure_init(); G_HEAD_PREGNANCY_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_LATE_REPLY() -> Py { ensure_init(); G_LATE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LATE_REPLY(v: Py) { ensure_init(); G_LATE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_LIST() -> Py { ensure_init(); G_LIST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIST(v: Py) { ensure_init(); G_LIST.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATIVE_REPLY() -> Py { ensure_init(); G_NEGATIVE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE_REPLY(v: Py) { ensure_init(); G_NEGATIVE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TEST() -> Py { ensure_init(); G_NOT_A_TEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TEST(v: Py) { ensure_init(); G_NOT_A_TEST.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FOOD() -> Py { ensure_init(); G_NOT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FOOD(v: Py) { ensure_init(); G_NOT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_POSITIVE_REPLY() -> Py { ensure_init(); G_POSITIVE_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSITIVE_REPLY(v: Py) { ensure_init(); G_POSITIVE_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT() -> Py { ensure_init(); G_PREGNANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT(v: Py) { ensure_init(); G_PREGNANT.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_ALREADY() -> Py { ensure_init(); G_PREGNANT_ALREADY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_ALREADY(v: Py) { ensure_init(); G_PREGNANT_ALREADY.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANT_MOVEMENT() -> Py { ensure_init(); G_PREGNANT_MOVEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANT_MOVEMENT(v: Py) { ensure_init(); G_PREGNANT_MOVEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g_PREG_ANY_FOOD() -> Py { ensure_init(); G_PREG_ANY_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREG_ANY_FOOD(v: Py) { ensure_init(); G_PREG_ANY_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_PREG_WORRY_FOODS() -> Py { ensure_init(); G_PREG_WORRY_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREG_WORRY_FOODS(v: Py) { ensure_init(); G_PREG_WORRY_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_TEST_REPLY() -> Py { ensure_init(); G_TEST_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEST_REPLY(v: Py) { ensure_init(); G_TEST_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_TEST_WORDS() -> Py { ensure_init(); G_TEST_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEST_WORDS(v: Py) { ensure_init(); G_TEST_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN() -> Py { ensure_init(); G_WHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN(v: Py) { ensure_init(); G_WHEN.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_THE_BIRTH() -> Py { ensure_init(); G__AFTER_THE_BIRTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_THE_BIRTH(v: Py) { ensure_init(); G__AFTER_THE_BIRTH.with(|c| *c.borrow_mut() = v); }
pub fn g__AMBIGUOUS() -> Py { ensure_init(); G__AMBIGUOUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AMBIGUOUS(v: Py) { ensure_init(); G__AMBIGUOUS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED() -> Py { ensure_init(); G__ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED(v: Py) { ensure_init(); G__ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_WEEKS() -> Py { ensure_init(); G__ASKS_WEEKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_WEEKS(v: Py) { ensure_init(); G__ASKS_WEEKS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BIRTH_LINE() -> Py { ensure_init(); G__A_BIRTH_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BIRTH_LINE(v: Py) { ensure_init(); G__A_BIRTH_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MILESTONE() -> Py { ensure_init(); G__A_MILESTONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MILESTONE(v: Py) { ensure_init(); G__A_MILESTONE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MONTH() -> Py { ensure_init(); G__A_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MONTH(v: Py) { ensure_init(); G__A_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PREGNANCY() -> Py { ensure_init(); G__A_PREGNANCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PREGNANCY(v: Py) { ensure_init(); G__A_PREGNANCY.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTH_ASK() -> Py { ensure_init(); G__BIRTH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTH_ASK(v: Py) { ensure_init(); G__BIRTH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTH_WORD() -> Py { ensure_init(); G__BIRTH_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTH_WORD(v: Py) { ensure_init(); G__BIRTH_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTH_WORDS() -> Py { ensure_init(); G__BIRTH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTH_WORDS(v: Py) { ensure_init(); G__BIRTH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTH_WORD_KEY() -> Py { ensure_init(); G__BIRTH_WORD_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTH_WORD_KEY(v: Py) { ensure_init(); G__BIRTH_WORD_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRTH_WORD_NAMES() -> Py { ensure_init(); G__BIRTH_WORD_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRTH_WORD_NAMES(v: Py) { ensure_init(); G__BIRTH_WORD_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_THEN_MONTH() -> Py { ensure_init(); G__DAY_THEN_MONTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_THEN_MONTH(v: Py) { ensure_init(); G__DAY_THEN_MONTH.with(|c| *c.borrow_mut() = v); }
pub fn g__DO_THEY_HURT() -> Py { ensure_init(); G__DO_THEY_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DO_THEY_HURT(v: Py) { ensure_init(); G__DO_THEY_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g__DUE_ELSEWHERE() -> Py { ensure_init(); G__DUE_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DUE_ELSEWHERE(v: Py) { ensure_init(); G__DUE_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__DUE_SHAPE() -> Py { ensure_init(); G__DUE_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DUE_SHAPE(v: Py) { ensure_init(); G__DUE_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__FEEDING_WORDS() -> Py { ensure_init(); G__FEEDING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEEDING_WORDS(v: Py) { ensure_init(); G__FEEDING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__GIVES_THE_DAY() -> Py { ensure_init(); G__GIVES_THE_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GIVES_THE_DAY(v: Py) { ensure_init(); G__GIVES_THE_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_FAR_APART() -> Py { ensure_init(); G__HOW_FAR_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_FAR_APART(v: Py) { ensure_init(); G__HOW_FAR_APART.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTHS() -> Py { ensure_init(); G__MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTHS(v: Py) { ensure_init(); G__MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__MONTH_THEN_DAY() -> Py { ensure_init(); G__MONTH_THEN_DAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONTH_THEN_DAY(v: Py) { ensure_init(); G__MONTH_THEN_DAY.with(|c| *c.borrow_mut() = v); }
pub fn g__MY_OWN() -> Py { ensure_init(); G__MY_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MY_OWN(v: Py) { ensure_init(); G__MY_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BIRTH_LINE() -> Py { ensure_init(); G__NOT_A_BIRTH_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BIRTH_LINE(v: Py) { ensure_init(); G__NOT_A_BIRTH_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_BIRTH() -> Py { ensure_init(); G__NOT_THE_BIRTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_BIRTH(v: Py) { ensure_init(); G__NOT_THE_BIRTH.with(|c| *c.borrow_mut() = v); }
pub fn g__OTHER_SUBJECT() -> Py { ensure_init(); G__OTHER_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OTHER_SUBJECT(v: Py) { ensure_init(); G__OTHER_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__PERIOD_ONLY() -> Py { ensure_init(); G__PERIOD_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERIOD_ONLY(v: Py) { ensure_init(); G__PERIOD_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g__PERIOD_WITH_A_PREGNANCY() -> Py { ensure_init(); G__PERIOD_WITH_A_PREGNANCY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERIOD_WITH_A_PREGNANCY(v: Py) { ensure_init(); G__PERIOD_WITH_A_PREGNANCY.with(|c| *c.borrow_mut() = v); }
pub fn g__POSTNATAL_BIRTH() -> Py { ensure_init(); G__POSTNATAL_BIRTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POSTNATAL_BIRTH(v: Py) { ensure_init(); G__POSTNATAL_BIRTH.with(|c| *c.borrow_mut() = v); }
pub fn g__POSTNATAL_BLEED() -> Py { ensure_init(); G__POSTNATAL_BLEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POSTNATAL_BLEED(v: Py) { ensure_init(); G__POSTNATAL_BLEED.with(|c| *c.borrow_mut() = v); }
pub fn g__PREG_DENIED() -> Py { ensure_init(); G__PREG_DENIED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREG_DENIED(v: Py) { ensure_init(); G__PREG_DENIED.with(|c| *c.borrow_mut() = v); }
pub fn g__PREG_SAID() -> Py { ensure_init(); G__PREG_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREG_SAID(v: Py) { ensure_init(); G__PREG_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__PREG_SOFT() -> Py { ensure_init(); G__PREG_SOFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREG_SOFT(v: Py) { ensure_init(); G__PREG_SOFT.with(|c| *c.borrow_mut() = v); }
pub fn g__PREG_WORD() -> Py { ensure_init(); G__PREG_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PREG_WORD(v: Py) { ensure_init(); G__PREG_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__SECOND_BIRTH() -> Py { ensure_init(); G__SECOND_BIRTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SECOND_BIRTH(v: Py) { ensure_init(); G__SECOND_BIRTH.with(|c| *c.borrow_mut() = v); }
pub fn g__SHORT_ASK() -> Py { ensure_init(); G__SHORT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHORT_ASK(v: Py) { ensure_init(); G__SHORT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__THEY_HURT() -> Py { ensure_init(); G__THEY_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THEY_HURT(v: Py) { ensure_init(); G__THEY_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_ASK() -> Py { ensure_init(); G__TIME_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_ASK(v: Py) { ensure_init(); G__TIME_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__WEEKS_NOW() -> Py { ensure_init(); G__WEEKS_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEEKS_NOW(v: Py) { ensure_init(); G__WEEKS_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g__WHEN_TO_CALL() -> Py { ensure_init(); G__WHEN_TO_CALL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHEN_TO_CALL(v: Py) { ensure_init(); G__WHEN_TO_CALL.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Pregnancy: what is off the menu, and what is not.\n\n\"Can I eat brie when pregnant\" is one of the most asked questions in the world, and the answer\nis not a lecture about a healthy diet: it is a short list of foods a pregnant woman should skip,\neach with the reason, and the reassurance that most of the kitchen is still fine. So the table is\nkeyed on the food and on the pregnancy, the same way keeping.py keys a food that has gone off on\nthe food and the sign, and one row is a food or a group of foods that share a verdict.\n\nThe advice is the NHS line as it stands in the UK, because the details people ask about are UK\ndetails: British Lion eggs are fine runny, hard cheese is fine, and the caffeine figure is 200mg\na day. Where a line has no food in it at all (\"what can I eat\", \"how much coffee\") the row is\nkeyed on the general words, so \"what can't i eat when pregnant\" still gets the list.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("health"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("pregnancies"), py_str("expecting a baby"), py_str("expecting our baby"), py_str("trimester"), py_str("weeks pregnant"), py_str("months pregnant"), py_str("im having a baby"), py_str("i'm having a baby"), py_str("baby on the way"), py_str("antenatal"), py_str("first baby"), py_str("second baby"), py_str("third baby")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.PREGNANT"); }
    s_PREGNANT(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("exercise"), py_str("exercising"), py_str("workout"), py_str("work out"), py_str("workouts"), py_str("gym"), py_str("run"), py_str("running"), py_str("jog"), py_str("jogging"), py_str("swim"), py_str("swimming"), py_str("lengths"), py_str("weights"), py_str("lift"), py_str("lifting"), py_str("weight training"), py_str("strength training"), py_str("yoga"), py_str("pilates"), py_str("spin"), py_str("spinning"), py_str("cycling"), py_str("cycle"), py_str("bike"), py_str("walking"), py_str("walks"), py_str("hike"), py_str("hiking"), py_str("dance"), py_str("dancing"), py_str("zumba"), py_str("tennis"), py_str("badminton"), py_str("netball"), py_str("football"), py_str("hockey"), py_str("climbing"), py_str("horse riding"), py_str("riding"), py_str("skiing"), py_str("ski"), py_str("crossfit"), py_str("fitness class"), py_str("aerobics"), py_str("training"), py_str("keep fit"), py_str("stay active"), py_str("active")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.PREGNANT_MOVEMENT"); }
    s_PREGNANT_MOVEMENT(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("morning sickness"), py_str("nausea"), py_str("sick every"), py_str("bleeding"), py_str("bleed"), py_str("spotting"), py_str("cramps"), py_str("cramping"), py_str("scan"), py_str("scans"), py_str("midwife"), py_str("labour"), py_str("labor"), py_str("birth"), py_str("c-section"), py_str("induction"), py_str("glucose test"), py_str("gestational"), py_str("pre-eclampsia"), py_str("blood pressure"), py_str("kick"), py_str("kicks"), py_str("movement"), py_str("movements"), py_str("heartbeat"), py_str("twins"), py_str("miscarriage"), py_str("ectopic"), py_str("test"), py_str("tests"), py_str("folic acid"), py_str("vitamin d"), py_str("vitamin a"), py_str("supplement"), py_str("supplements"), py_str("paracetamol"), py_str("ibuprofen"), py_str("medicine"), py_str("medication"), py_str("tablet"), py_str("tablets"), py_str("antibiotic"), py_str("vaccine"), py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("migraines"), py_str("pain"), py_str("painful"), py_str("sore"), py_str("soreness"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("hurts"), py_str("hurting"), py_str("toothache"), py_str("travel"), py_str("flight"), py_str("fly"), py_str("flying"), py_str("sleep"), py_str("sleeping"), py_str("sex"), py_str("exercise"), py_str("gym"), py_str("swim"), py_str("swimming"), py_str("bump"), py_str("back"), py_str("backache"), py_str("heartburn"), py_str("constipation"), py_str("swollen"), py_str("stretch marks"), py_str("weight gain"), py_str("how much weight"), py_str("mood"), py_str("anxious"), py_str("anxiety"), py_str("sick of"), py_str("not pregnant"), py_str("no longer pregnant"), py_str("was pregnant"), py_str("weren't pregnant"), py_str("breastfeeding"), py_str("breastfeed"), py_str("nursing"), py_str("formula"), py_str("weaning"), py_str("my wife is not"), py_str("trying to get pregnant"), py_str("trying for a baby"), py_str("ivf"), py_str("fertility"), py_str("baby shower"), py_str("baby names"), py_str("nursery"), py_str("pram"), py_str("pushchair"), py_str("cot"), py_str("nappy"), py_str("nappies"), py_str("egg"), py_str("eggs"), py_str("runny egg"), py_str("raw egg"), py_str("mayonnaise"), py_str("mayo"), py_str("dippy")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.NOT_FOOD"); }
    s_NOT_FOOD(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.LIST"); }
    s_LIST(py_str("The list is short: no soft mould-ripened or blue cheese unless it is cooked until steaming, no pate or liver, no raw or undercooked meat, fish or shellfish, and nothing made with unpasteurised milk. Everything else is fine - hard cheese, cooked eggs, cooked fish, peanuts, fruit and veg - though wash the veg and heat leftovers until they are steaming hot, because listeria is the reason for the whole list and it is killed by heat."));
    fn __data4() -> Py { py_tuple(vec![py_str("mango"), py_str("mangoes"), py_str("mangos"), py_str("melon"), py_str("melons"), py_str("watermelon"), py_str("grapes"), py_str("grape"), py_str("kiwi"), py_str("kiwis"), py_str("berries"), py_str("strawberries"), py_str("blueberries"), py_str("raspberries"), py_str("blackberries"), py_str("peach"), py_str("peaches"), py_str("pear"), py_str("pears"), py_str("plum"), py_str("plums"), py_str("banana"), py_str("bananas"), py_str("orange"), py_str("oranges"), py_str("apple"), py_str("apples"), py_str("pineapple chunks"), py_str("satsuma"), py_str("satsumas"), py_str("clementine"), py_str("clementines")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.PREG_WORRY_FOODS"); }
    s_PREG_WORRY_FOODS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("pineapple"), py_str("pineapples"), py_str("pineapple juice")]), py_str("Fine. Pineapple does not bring on labour: the enzyme people blame for it is in the stem, not in the fruit you eat, and a portion has nowhere near enough of it to do anything. So have it fresh, tinned or juiced - the myth is the only thing to worry about.")]), py_tuple(vec![py_tuple(vec![py_str("juice"), py_str("juices"), py_str("orange juice"), py_str("apple juice"), py_str("fruit juice"), py_str("grapefruit juice"), py_str("cranberry juice"), py_str("tomato juice"), py_str("a glass of juice")]), py_str("Juice is fine: the cartons and bottles in a shop are pasteurised already, so nothing on the pregnancy list applies to them, and juice you squeeze yourself is fine if you wash the fruit and drink it the same day. It is mostly sugar without the fibre that slows it down, so one glass with a meal is the habit to aim for rather than sipping it through the day, and whole fruit is the better version of the same vitamins.")]), py_tuple(vec![__data4(), py_str("Fine, and worth eating: fruit is where most of the vitamin C comes from. Rinse it under the tap first, because listeria and toxoplasmosis are on the outside of fruit and veg rather than in it, and a melon wants washing before it is cut, not after. Fruit juice is fine too, but it is sugary - whole fruit is the better habit.")]), py_tuple(vec![py_tuple(vec![py_str("papaya"), py_str("papayas"), py_str("pawpaw"), py_str("pawpaws"), py_str("green papaya"), py_str("unripe papaya")]), py_str("Ripe papaya is fine. The one to leave is the unripe green papaya, and papaya skin and seeds: unripe papaya carries a latex-like substance that is said to act on the womb, so a green papaya salad is one to skip for nine months. A ripe orange one is a good breakfast.")]), py_tuple(vec![py_tuple(vec![py_str("honey")]), py_str("Fine for you. The warning about honey is for babies under one, whose gut can let botulism spores grow - it does not apply to you or to the baby before birth. It is still sugar, so a spoon in a drink rather than a jar a day.")]), py_tuple(vec![py_tuple(vec![py_str("hummus"), py_str("houmous"), py_str("humous"), py_str("tahini"), py_str("houmus")]), py_str("Fine, including the tubs from the supermarket: shop hummus is made with pasteurised sesame and chickpeas, and sesame is not one of the allergens to avoid in pregnancy. Keep an opened tub in the fridge and eat it within a couple of days, because it is a cold food and the listeria advice is about cold food.")]), py_tuple(vec![py_tuple(vec![py_str("spicy food"), py_str("spicy"), py_str("curry"), py_str("curries"), py_str("chilli"), py_str("chili"), py_str("chillies"), py_str("chilies"), py_str("hot sauce"), py_str("spices"), py_str("spice")]), py_str("Fine. Spicy food does not bring on labour or harm the baby, and the old advice to avoid it was about the mother's comfort. If you get heartburn, which is common later on, it is worth eating smaller amounts more often rather than avoiding spice altogether.")]), py_tuple(vec![py_tuple(vec![py_str("ice cream"), py_str("icecream"), py_str("soft serve"), py_str("soft-serve"), py_str("soft whip"), py_str("milkshake"), py_str("milkshakes")]), py_str("Fine from a tub: ice cream is made with pasteurised milk. The one to skip is soft-serve from a machine, the sort that sits in a hopper all day - that is the cold-served food listeria has been found in, and a machine that is not cleaned often is the risk. A scoop from a shop tub is no riskier than a supermarket one.")]), py_tuple(vec![py_tuple(vec![py_str("coleslaw"), py_str("slaw"), py_str("potato salad"), py_str("deli salad")]), py_str("Fine from a packet or a supermarket tub, which is made with pasteurised mayonnaise. The one to skip is salad that has been sitting out on a deli counter, and anything you are unsure has been kept cold: listeria grows in food that is kept cold for a while, not in food that is freshly made.")]), py_tuple(vec![py_tuple(vec![py_str("bean sprouts"), py_str("beansprouts"), py_str("sprouts"), py_str("sprouted seeds"), py_str("alfalfa")]), py_str("Cook them through, or leave them: raw bean sprouts and sprouted seeds are one of the foods listeria and salmonella have been found in, and washing does not reliably clear them. Sprouts cooked into a stir fry, a curry or a soup are fine.")]), py_tuple(vec![py_tuple(vec![py_str("olives"), py_str("pickles"), py_str("gherkins"), py_str("sauerkraut")]), py_str("Fine, and the vinegar or brine they are in is not a problem. They are salty, so if your ankles or blood pressure are being watched, keep them to a few rather than a bowl.")]), py_tuple(vec![py_tuple(vec![py_str("dates"), py_str("dates fruit")]), py_str("Fine, and a good snack: they are sweet, so they are the thing to reach for instead of a biscuit at four in the afternoon. There is no evidence they bring on labour, whatever you read.")])]));
    fn __data5() -> Py { py_tuple(vec![py_str("alcohol"), py_str("wine"), py_str("beer"), py_str("lager"), py_str("cider"), py_str("stout"), py_str("ale"), py_str("prosecco"), py_str("champagne"), py_str("gin"), py_str("vodka"), py_str("whisky"), py_str("whiskey"), py_str("rum"), py_str("brandy"), py_str("tequila"), py_str("booze"), py_str("cocktail"), py_str("cocktails"), py_str("coffee"), py_str("caffeine"), py_str("decaf"), py_str("tea"), py_str("espresso"), py_str("latte"), py_str("cappuccino"), py_str("energy drink"), py_str("energy drinks"), py_str("cola"), py_str("diet coke"), py_str("hot chocolate"), py_str("mulled wine"), py_str("alcopop")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.DRINKS_WITH_THEIR_OWN_ANSWER"); }
    s_DRINKS_WITH_THEIR_OWN_ANSWER(__data5());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.PREG_ANY_FOOD"); }
    s_PREG_ANY_FOOD(py_str("That food is not on the list of things to avoid, so it is fine. The list is short: no mould-ripened or blue cheese unless it is cooked until steaming, no pate or liver, no raw meat, fish or shellfish, and nothing made with unpasteurised milk. Wash fruit and veg and heat leftovers until steaming - listeria is why the list exists, and heat kills it."));
    fn __data6() -> Py { py_tuple(vec![py_str("am i"), py_str("am i not"), py_str("could i be"), py_str("could i"), py_str("might be"), py_str("might i"), py_str("maybe"), py_str("perhaps"), py_str("possibly"), py_str("possible"), py_str("think"), py_str("think im"), py_str("think i'm"), py_str("suspect"), py_str("wondering"), py_str("not sure"), py_str("unsure"), py_str("no idea"), py_str("or not"), py_str("when will i know"), py_str("how do i know"), py_str("how can i tell"), py_str("signs"), py_str("symptoms"), py_str("early"), py_str("missed")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.AM_I"); }
    s_AM_I(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("period"), py_str("periods"), py_str("missed period"), py_str("late period"), py_str("period late"), py_str("no period"), py_str("periods late"), py_str("period hasnt come"), py_str("period hasn't come"), py_str("period not come"), py_str("late"), py_str("late by"), py_str("days late"), py_str("weeks late"), py_str("due date"), py_str("baby due"), py_str("when am i due"), py_str("due date is"), py_str("missed"), py_str("irregular")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.WHEN"); }
    s_WHEN(__data7());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.TEST_WORDS"); }
    s_TEST_WORDS(py_tuple(vec![py_str("pregnancy test"), py_str("test"), py_str("tests"), py_str("tested"), py_str("faint line"), py_str("clearblue"), py_str("test kit"), py_str("test strip"), py_str("test strips"), py_str("positive"), py_str("negative")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.PREGNANT_ALREADY"); }
    s_PREGNANT_ALREADY(py_tuple(vec![py_str("im pregnant what"), py_str("i am pregnant what"), py_str("i'm pregnant what"), py_str("im pregnant now what"), py_str("i am pregnant now what"), py_str("im pregnant so what"), py_str("i am pregnant so what"), py_str("found out im pregnant what"), py_str("found out i am pregnant what")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.CYCLE_SHAPE"); }
    s_CYCLE_SHAPE(py_tuple(vec![py_str("normal"), py_str("how long"), py_str("how often"), py_str("how many days"), py_str("days between"), py_str("lasts"), py_str("started"), py_str("when do periods"), py_str("first period"), py_str("regular")]));
    fn __data8() -> Py { py_tuple(vec![py_str("get pregnant on"), py_str("get pregnant after"), py_str("get pregnant if"), py_str("get pregnant while"), py_str("get pregnant from"), py_str("pregnant on my period"), py_str("conceive"), py_str("conceiving"), py_str("conception"), py_str("ovulate"), py_str("ovulating"), py_str("ovulation"), py_str("fertile"), py_str("fertility"), py_str("chance of"), py_str("chances of"), py_str("how likely")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.FERTILITY"); }
    s_FERTILITY(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("covid"), py_str("coronavirus"), py_str("lateral flow"), py_str("pcr"), py_str("mot"), py_str("theory"), py_str("driving test"), py_str("eye test"), py_str("blood test"), py_str("urine test"), py_str("hearing test"), py_str("smear"), py_str("cervical"), py_str("chlamydia"), py_str("sti"), py_str("std"), py_str("thyroid"), py_str("diabetes"), py_str("allergy"), py_str("intolerance"), py_str("paternity"), py_str("dna"), py_str("quiz"), py_str("exam"), py_str("revision"), py_str("mock"), py_str("interview"), py_str("coding"), py_str("swab"), py_str("skin prick")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.NOT_A_TEST"); }
    s_NOT_A_TEST(__data9());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.TEST_REPLY"); }
    s_TEST_REPLY(py_str("A test is the answer, and the timing is the part people get wrong: from the day your period is due, on the first pee of the morning, which has the most hormone in it. A faint second line is still a line. If it is negative and nothing comes in three days, test again, because the hormone doubles every couple of days. Early signs - sore breasts, going off food, tiredness, feeling sick - are not reliable enough to go on either way. If you would rather not be pregnant, a GP or a sexual health clinic will go through the options without pushing you either way, and the sooner you ring the more there is to choose from."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.POSITIVE_REPLY"); }
    s_POSITIVE_REPLY(py_str("Ring the GP surgery and say you have had a positive test: the booking-in appointment is usually between 8 and 10 weeks, and they will sort the midwife, the scans and the bloods. Start 400 micrograms of folic acid and 10 micrograms of vitamin D every day, and stop alcohol - most of what you eat is still fine, and the list of what is not is short. If the line was faint or early, test again in three days. A negative test with no period in three days is worth testing again; a positive one with pain or bleeding is a call to the GP or 111 rather than something to sit on."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.NEGATIVE_REPLY"); }
    s_NEGATIVE_REPLY(py_str("A negative test this early is worth repeating rather than believing: test again in three days, first pee of the morning, because the hormone doubles every couple of days and an early test can simply be too soon. Plenty else shifts a period - stress, a bug, a bad month of sleep, a change in weight, travel, the pill - so one late period with a negative test is common and usually settles. Ring the GP if nothing comes for three months, if the tests stay negative but you feel pregnant, or if there is pain one side at all."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.EARLY_REPLY"); }
    s_EARLY_REPLY(py_str("A period that comes early is common and usually just a short cycle: stress, a bug, a change in sleep or travel, a big drop in weight, or the pill can all bring one on, and the spotting some people get at ovulation a couple of weeks before is often taken for one. Write the dates down for the next two or three - if they stay under three weeks apart, if you bleed between periods or after sex, or if the bleeding is very heavy, that is a GP appointment. If there is any chance you could be pregnant, test anyway: a bleed can happen early on."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.CYCLE_REPLY"); }
    s_CYCLE_REPLY(py_str("The usual cycle is 21 to 35 days from the first day of one period to the first day of the next, and the bleeding itself is usually two to seven days - up to eight is still normal. What is worth a GP appointment: cycles closer than 21 days or longer than 35, bleeding that lasts more than eight days, bleeding between periods or after sex, soaking a pad or tampon an hour for several hours, or three months with no period at all. Write the dates down for three cycles first, because that is what the GP will ask for, and the pill can change the pattern while you are on it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.LATE_REPLY"); }
    s_LATE_REPLY(py_str("Plenty shifts a period by a few days - stress, a bug, a bad month of sleep, a change in weight, travel, the pill - so the useful next step is a test rather than a wait: from the day it was due, first pee of the morning, and again in three days if it is negative and nothing comes. Three months without a period, bleeding between periods or after sex, or a positive test with pain or bleeding is a GP appointment. If your cycles were always irregular, a few weeks late on its own is not alarming."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._FEEDING_WORDS"); }
    s__FEEDING_WORDS(py_tuple(vec![py_str("breastfeed"), py_str("breastfeeding"), py_str("breast feeding"), py_str("breast fed"), py_str("breastfed"), py_str("nursing"), py_str("nurse"), py_str("lactating"), py_str("chestfeeding"), py_str("expressing"), py_str("pumping"), py_str("feeding")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._DUE_SHAPE"); }
    s__DUE_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:due date|due dates|baby due|baby is due|due in|when am i due|when are you due|weeks pregnant|weeks along|weeks am i|weeks gone|how far along|how far gone|what week am i|which week am i|how many (?:weeks?|wks?|months) (?:pregnant|preg|am i|along|into|gone|left|to go)|how pregnant am i|pregnancy weeks|how many weeks (?:is|does|are) (?:a |the |an )?pregnan\\w*|how many months (?:pregnant|along)|how long (?:is|does|are|do|can) (?:a |the |an )?pregnan\\w*|how long (?:are you|is a woman|is she|is he) pregnant|last period|first day of (?:my|her|your|the) last period|lmp\\b|(?:when|what date|which date)\\b[^.?!]{0,30}\\bdue\\b|when will (?:the baby|my baby|she|he|it) (?:arrive|come|be born|get here)|when (?:is|are) (?:the baby|my baby|she|he|it) born|when (?:is|are) (?:the baby|my baby|she|he|it) (?:due|coming))"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._PREG_WORD"); }
    s__PREG_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:pregnan\\w*|preg|expecting|baby|babies|due date|lmp|last period|weeks (?:pregnant|preg|along|gone|am i)|birth|born|midwife)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._PREG_SOFT"); }
    s__PREG_SOFT(stdlib_call("re", "compile", &[py_str("\\b(?:how far along|how far gone|weeks am i|wks am i|what week am i|which week am i|when am i due|when are you due|when (?:is|are) (?:she|he) due|due (?:in|on) (?:january|february|march|april|may|june|july|august|september|october|november|december))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._OTHER_SUBJECT"); }
    s__OTHER_SUBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:essay|project|thesis|dissertation|book|build|renovation|course|game|sprint|assignment|paper|novel|training|app|website)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._DUE_ELSEWHERE"); }
    s__DUE_ELSEWHERE(stdlib_call("re", "compile", &[py_str("\\bdue\\s+(?:back|at work|for work|in work|to (?:start|finish|go|leave|renew|pay|return|hand|come back)|(?:a|an|any|some|my|the)?\\s*(?:tax\\s+)?(?:refund|rebate|payment|bill|rise|bonus|money|fee|invoice))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._A_MILESTONE"); }
    s__A_MILESTONE(stdlib_call("re", "compile", &[py_str("\\bwhen (?:will|do|should|can)\\b[^?]{0,40}?\\b(?:feel (?:the )?(?:baby|it|them|kicks?|movement|movements)|kicks? (?:start|begin)|start (?:to )?(?:show|showing)|be showing|bump (?:show|appear|grow)|feel (?:him|her) move|movement)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._PERIOD_ONLY"); }
    s__PERIOD_ONLY(stdlib_call("re", "compile", &[py_str("\\bperiods?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._PERIOD_WITH_A_PREGNANCY"); }
    s__PERIOD_WITH_A_PREGNANCY(stdlib_call("re", "compile", &[py_str("\\b(?:pregnan\\w*|expecting|baby|babies|last period|lmp|weeks (?:pregnant|along|gone|am i)|due date|when am i due|when are you due)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data10() -> Py { py_dict(vec![(py_str("january"), Py::Int(1)), (py_str("jan"), Py::Int(1)), (py_str("february"), Py::Int(2)), (py_str("feb"), Py::Int(2)), (py_str("march"), Py::Int(3)), (py_str("mar"), Py::Int(3)), (py_str("april"), Py::Int(4)), (py_str("apr"), Py::Int(4)), (py_str("may"), Py::Int(5)), (py_str("june"), Py::Int(6)), (py_str("jun"), Py::Int(6)), (py_str("july"), Py::Int(7)), (py_str("jul"), Py::Int(7)), (py_str("august"), Py::Int(8)), (py_str("aug"), Py::Int(8)), (py_str("september"), Py::Int(9)), (py_str("sep"), Py::Int(9)), (py_str("sept"), Py::Int(9)), (py_str("october"), Py::Int(10)), (py_str("oct"), Py::Int(10)), (py_str("november"), Py::Int(11)), (py_str("nov"), Py::Int(11)), (py_str("december"), Py::Int(12)), (py_str("dec"), Py::Int(12))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._MONTHS"); }
    s__MONTHS(__data10());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._DAY_THEN_MONTH"); }
    s__DAY_THEN_MONTH(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})(?:st|nd|rd|th)?\\s+(?:of\\s+)?([a-z]{3,9})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._MONTH_THEN_DAY"); }
    s__MONTH_THEN_DAY(stdlib_call("re", "compile", &[py_str("\\b([a-z]{3,9})\\s+(?:the\\s+)?(\\d{1,2})(?:st|nd|rd|th)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._A_PREGNANCY"); }
    s__A_PREGNANCY(stdlib_call("re", "compile", &[py_str("\\b(?:a|the|an) pregnanc(?:y|ies)|\\bpregnancies\\b|\\bpregnancy last(?:s|ing)?\\b|\\bpregnancy norm(?:al|ally)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._MY_OWN"); }
    s__MY_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:am i|i am|i'?m|im|my|me|we|us|our)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._ASKS_WEEKS"); }
    s__ASKS_WEEKS(stdlib_call("re", "compile", &[py_str("\\b(?:how many (?:weeks?|wks?|months)|how far along|how far gone|weeks am i|wks am i|weeks pregnant|weeks along|weeks gone)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._WEEKS_NOW"); }
    s__WEEKS_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:im|i'?m|i am|am|is|are|at|now|about|nearly|almost|just|only)\\s+(\\d{1,2})\\s*(?:weeks?|wks?)\\b|\\b(\\d{1,2})\\s*(?:weeks?|wks?)\\s+(?:pregnant|along|gone|in)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._SHORT_ASK"); }
    s__SHORT_ASK(stdlib_call("re", "compile", &[py_str("\\bdue date\\b|\\bhow far along\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._ASKED"); }
    s__ASKED(stdlib_call("re", "compile", &[py_str("\\bwhens?\\b|\\bwhen's\\b|\\bhow (?:many|far|long|soon|pregnant)\\b|\\bwhich date\\b|\\bwhat date\\b|\\bwhat week\\b|\\bwhich week\\b|\\bpregnancy weeks\\b|\\bcalculat\\w* (?:my|the) due date\\b|\\bdue date calculator\\b|\\bwhat(?:'s|s)? (?:is |was )?(?:my|the|your) (?:due date|date thing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._GIVES_THE_DAY"); }
    s__GIVES_THE_DAY(stdlib_call("re", "compile", &[py_str("\\b(?:last period|period started|lmp|first day)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._A_MONTH"); }
    s__A_MONTH(stdlib_call("re", "compile", &[py_str("\\b(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.CARD"); }
    s_CARD(py_str("Forty weeks from the first day of your last period, so count 280 days from it (nine months and a week): a midwife writes that date in your notes at your booking-in appointment, and the dating scan can move it by a few days. If you have neither to hand, the date on your scan report or in your maternity notes is the one they will go by. Weeks gone are counted from that same first day of your last period, not from the day you conceived."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._TIME_ASK"); }
    s__TIME_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long\\b|\\bhow many hours\\b|\\bhow many days\\b|\\bhow soon\\b|\\bhow early\\b|\\bwhen should i\\b|\\bwhen do i\\b|\\bwhen to\\b|\\bwhat week\\b|\\bhow far along\\b|\\bwhat (?:should|shall|can|do) i (?:take|bring|pack|put)\\b|\\bwhat do i (?:need|want|have) to (?:pack|take|bring)\\b|\\bwhat to (?:take|bring|pack)\\b|\\blonger\\b|\\bshorter\\b|\\bquicker\\b|\\bfaster\\b|\\blongest\\b|\\btoo long\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._BIRTH_WORD"); }
    s__BIRTH_WORD(stdlib_call("re", "compile", &[py_str("\\bin labour\\b|\\bin labor\\b|\\blabour\\b|\\blabor\\b|\\bgiv(?:e|ing) birth\\b|\\bthe birth\\b|\\bbirth\\b|\\bcontractions?\\b|\\bwaters\\b|\\bc-section\\b|\\bc section\\b|\\bcaesarean\\b|\\bcesarean\\b|\\bepidural\\b|\\bpregnan\\w*\\b|\\bbaby\\b|\\bdelivery\\b|\\bhospital bag\\b|\\bmaternity bag\\b|\\bmaternity\\b|\\b(?:first|second|third) stage\\b|\\bstage of (?:labour|labor|birth)\\b|\\bpushing stage\\b|\\bhow long (?:is|does) pushing\\b|\\bplacenta\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data11() -> Py { py_tuple(vec![py_str("car"), py_str("cars"), py_str("garage"), py_str("mot"), py_str("bill"), py_str("bills"), py_str("quote"), py_str("wage"), py_str("wages"), py_str("hourly"), py_str("party"), py_str("mp"), py_str("mps"), py_str("election"), py_str("vote"), py_str("voted"), py_str("government"), py_str("council"), py_str("worker"), py_str("workers"), py_str("job"), py_str("jobs"), py_str("engine"), py_str("clutch"), py_str("cambelt"), py_str("servicing"), py_str("price"), py_str("cost"), py_str("union"), py_str("manifesto"), py_str("poll"), py_str("brexit"), py_str("tax"), py_str("budget"), py_str("praevia"), py_str("previa")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._NOT_THE_BIRTH"); }
    s__NOT_THE_BIRTH(__data11());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._SECOND_BIRTH"); }
    s__SECOND_BIRTH(stdlib_call("re", "compile", &[py_str("\\bsecond\\b|\\bthird\\b|\\bfourth\\b|\\bnot my first\\b|\\banother baby\\b|\\b2nd\\b|\\b3rd\\b|\\bnot the first\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._AFTER_THE_BIRTH"); }
    s__AFTER_THE_BIRTH(stdlib_call("re", "compile", &[py_str("\\bdriv(?:e|ing)\\b|\\bexercise\\b|\\bexercising\\b|\\bgym\\b|\\bswim(?:ming)?\\b|\\bbath\\b|\\bbath(?:e|ing)\\b|\\bback to work\\b|\\bto work\\b|\\blift(?:ing)?\\b|\\bsex\\b|\\bstitches\\b|\\bbleed(?:ing)?\\b|\\bperiod\\b|\\bwash my hair\\b|\\balcohol\\b|\\bdrink\\b|\\brun(?:ning)?\\b|\\bwalk(?:ing)?\\b|\\bhoover\\b|\\bhousework\\b|\\bpelvic floor\\b|\\bscar\\b|\\brecover\\w*\\b|\\bheal\\w*\\b|\\bnumb\\b|\\bcontinence\\b|\\bpostnatal\\b|\\bpost natal\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._PREG_SAID"); }
    s__PREG_SAID(stdlib_call("re", "compile", &[py_str("\\b(?:pregnan\\w*|expecting a baby|expecting our baby|with child|trimester|\\d\\d? weeks (?:pregnant|gone))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._PREG_DENIED"); }
    s__PREG_DENIED(stdlib_call("re", "compile", &[py_str("\\b(?:not|isn't|isnt|aren't|arent|no longer|stopped being|never)\\s+(?:pregnan\\w*)")]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.HEAD_PREGNANCY_REPLY"); }
    s_HEAD_PREGNANCY_REPLY(py_str("Paracetamol is the one to take for a headache in pregnancy, at the lowest dose that works and for the shortest time - not ibuprofen or aspirin unless a doctor or midwife has told you to. Water, food and sleep do a lot, and a skipped meal, dehydration and caffeine withdrawal are the usual causes. Mention headaches that keep coming back to your midwife, and go in the same day if a bad headache comes with blurred vision or flashing lights, pain under the ribs, swelling of the face or hands, or vomiting after 20 weeks: that combination is the one that has to be checked for pre-eclampsia."));
    fn __data13() -> Py { py_tuple(vec![py_str("brie"), py_str("camembert"), py_str("soft cheese"), py_str("soft cheeses"), py_str("blue cheese"), py_str("stilton"), py_str("gorgonzola"), py_str("roquefort"), py_str("chevre"), py_str("goats cheese"), py_str("mould ripened"), py_str("mold ripened"), py_str("mexicana"), py_str("mould ripened cheese"), py_str("mold ripened cheese"), py_str("baked camembert")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("rare"), py_str("raw meat"), py_str("undercooked"), py_str("pink"), py_str("medium rare"), py_str("steak"), py_str("salami"), py_str("chorizo"), py_str("prosciutto"), py_str("parma ham"), py_str("salami"), py_str("pepperoni"), py_str("cured meat"), py_str("cured meats"), py_str("sausage"), py_str("sausages"), py_str("hot dog"), py_str("hot dogs"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("chicken"), py_str("turkey"), py_str("mince"), py_str("burgers"), py_str("meat")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("sushi"), py_str("sashimi"), py_str("raw fish"), py_str("raw salmon"), py_str("smoked salmon"), py_str("gravadlax"), py_str("oysters"), py_str("oyster"), py_str("raw prawns"), py_str("shellfish"), py_str("mussels"), py_str("prawns"), py_str("fish"), py_str("salmon"), py_str("cod"), py_str("ceviche"), py_str("anchovies"), py_str("sardines"), py_str("mackerel"), py_str("sashimi")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("what can i eat"), py_str("what can't i eat"), py_str("what cant i eat"), py_str("what can i not eat"), py_str("what should i avoid"), py_str("what to avoid"), py_str("what foods"), py_str("which foods"), py_str("what shouldn't i eat"), py_str("what is off limits"), py_str("can i eat"), py_str("avoid"), py_str("anything i shouldn't"), py_str("what should i eat"), py_str("what should i be eating"), py_str("what to eat"), py_str("what do i eat"), py_str("what not to eat"), py_str("what i shouldn't eat"), py_str("what i shouldnt eat"), py_str("shouldn't eat"), py_str("shouldnt eat"), py_str("shouldn't i eat"), py_str("shouldnt i eat"), py_str("not eat"), py_str("not to eat"), py_str("can't i eat"), py_str("cant i eat"), py_str("avoid eating"), py_str("what's safe to eat"), py_str("whats safe to eat"), py_str("safe to eat")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("fly"), py_str("flying"), py_str("flight"), py_str("flights"), py_str("plane"), py_str("airplane"), py_str("aeroplane"), py_str("air travel"), py_str("long haul"), py_str("long-haul"), py_str("holiday abroad"), py_str("travel"), py_str("travelling"), py_str("traveling"), py_str("a trip"), py_str("trip abroad"), py_str("go abroad"), py_str("going abroad"), py_str("flying abroad"), py_str("holiday"), py_str("holidays"), py_str("on holiday"), py_str("vacation"), py_str("a break abroad")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("allowed"), py_str("can i"), py_str("should i"), py_str("is it"), py_str("when can i"), py_str("how late"), py_str("how far"), py_str("abroad"), py_str("can you"), py_str("can we"), py_str("could i"), py_str("is it ok to"), py_str("still ok"), py_str("still fine")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("allowed"), py_str("can i"), py_str("should i"), py_str("is it"), py_str("still"), py_str("while"), py_str("when"), py_str("how much"), py_str("how often"), py_str("how long"), py_str("continue"), py_str("keep")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("bring on labour"), py_str("bring on labor"), py_str("start labour"), py_str("start labor"), py_str("induce"), py_str("kick start"), py_str("weight gain"), py_str("how much weight"), py_str("put on"), py_str("gained"), py_str("back pain"), py_str("backache"), py_str("pelvic"), py_str("sciatica"), py_str("cramp"), py_str("sick"), py_str("nausea"), py_str("bleeding"), py_str("bleed"), py_str("discharge"), py_str("waters"), py_str("contractions")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting a baby"), py_str("expecting our baby"), py_str("trimester"), py_str("weeks pregnant"), py_str("weeks gone"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("can i"), py_str("could i"), py_str("is it"), py_str("while pregnant"), py_str("in pregnancy"), py_str("take")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("folic acid"), py_str("vitamin"), py_str("supplement"), py_str("scan"), py_str("scans"), py_str("midwife appointment"), py_str("bp"), py_str("blood pressure"), py_str("glucose test"), py_str("sugar test"), py_str("kick"), py_str("kicks"), py_str("movement"), py_str("movements"), py_str("bleeding"), py_str("bleed"), py_str("spotting"), py_str("waters"), py_str("contractions"), py_str("swollen feet"), py_str("swollen ankles"), py_str("sickness"), py_str("nausea"), py_str("vomiting"), py_str("heartburn"), py_str("constipation"), py_str("piles"), py_str("stretch marks"), py_str("sleep"), py_str("travel"), py_str("flight"), py_str("flying"), py_str("food"), py_str("eat"), py_str("eating"), py_str("diet"), py_str("sex"), py_str("exercise"), py_str("gym"), py_str("swim"), py_str("nursery"), py_str("pram"), py_str("cot"), py_str("nappy"), py_str("nappies")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("safe"), py_str("safely"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("allowed"), py_str("allow"), py_str("bad"), py_str("harm"), py_str("harmful"), py_str("danger"), py_str("dangerous"), py_str("risky"), py_str("risk"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("juice"), py_str("squash"), py_str("squashes"), py_str("smoothie")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("allowed"), py_str("allow"), py_str("bad"), py_str("harm"), py_str("harmful"), py_str("danger"), py_str("dangerous"), py_str("risky"), py_str("risk"), py_str("can i"), py_str("should i"), py_str("is it"), py_str("are they"), py_str("is"), py_str("are")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("bath"), py_str("baths"), py_str("sauna"), py_str("jacuzzi"), py_str("hot tub"), py_str("steam room"), py_str("hair dye"), py_str("dye"), py_str("dying my hair"), py_str("highlights"), py_str("hair colour"), py_str("hair color"), py_str("nails"), py_str("massage"), py_str("tattoo"), py_str("tattoos"), py_str("fake tan"), py_str("sunbed"), py_str("sunbathing"), py_str("cat litter"), py_str("litter"), py_str("cleaning products"), py_str("bleach"), py_str("paint"), py_str("x-ray"), py_str("xray"), py_str("dentist"), py_str("dental"), py_str("white spirit"), py_str("turps"), py_str("turpentine"), py_str("paint thinner"), py_str("thinner"), py_str("varnish"), py_str("wood stain"), py_str("primer"), py_str("gloss paint"), py_str("spray paint"), py_str("aerosol"), py_str("adhesive"), py_str("wood glue"), py_str("superglue"), py_str("solvent"), py_str("solvents"), py_str("fumes"), py_str("fumigate"), py_str("weedkiller"), py_str("weed killer"), py_str("glyphosate"), py_str("pesticide"), py_str("insecticide"), py_str("antifreeze"), py_str("oven cleaner"), py_str("drain cleaner"), py_str("descaler"), py_str("disinfectant"), py_str("ammonia"), py_str("mould spray"), py_str("ant powder"), py_str("slug pellets"), py_str("rat poison"), py_str("rodenticide"), py_str("flea treatment"), py_str("flea bomb"), py_str("creosote"), py_str("wood preserver"), py_str("decking oil"), py_str("paint stripper"), py_str("nitromors"), py_str("caustic"), py_str("swimming pool chemicals"), py_str("jab"), py_str("jabs"), py_str("jabbed"), py_str("booster jab"), py_str("booster"), py_str("vaccination"), py_str("immunisation"), py_str("immunization"), py_str("covid jab"), py_str("flu jab"), py_str("covid vaccine"), py_str("flu vaccine"), py_str("what can i eat"), py_str("what cant i eat"), py_str("what can't i eat"), py_str("what can i not eat"), py_str("what should i avoid"), py_str("what to avoid"), py_str("what foods"), py_str("which foods"), py_str("diet"), py_str("menu"), py_str("foods to avoid"), py_str("eat when pregnant"), py_str("eat while pregnant")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("drink"), py_str("drinking"), py_str("is it"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("can i"), py_str("should i"), py_str("much")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("breastfeeding"), py_str("nursing"), py_str("formula"), py_str("weaning"), py_str("my baby"), py_str("a baby"), py_str("the baby"), py_str("baby have"), py_str("baby need"), py_str("how much milk"), py_str("ounces"), py_str("oz"), py_str("expressing"), py_str("colic"), py_str("reflux"), py_str("allergy"), py_str("allergic"), py_str("lactose"), py_str("cows milk protein"), py_str("cheese"), py_str("brie"), py_str("camembert"), py_str("feta"), py_str("stilton"), py_str("yoghurt"), py_str("yogurt"), py_str("ice cream"), py_str("goat"), py_str("soya"), py_str("oat milk"), py_str("almond milk"), py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("frozen"), py_str("freezer"), py_str("thaw"), py_str("thawed"), py_str("defrost"), py_str("defrosted"), py_str("use by"), py_str("out of date"), py_str("gone off"), py_str("goes off"), py_str("how long does milk"), py_str("how long can i keep"), py_str("storage")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("how many weeks"), py_str("weeks pregnant"), py_str("weeks am i"), py_str("due date"), py_str("trimester"), py_str("maternity"), py_str("birth control"), py_str("contraception"), py_str("condom"), py_str("morning after"), py_str("test was negative"), py_str("test is negative"), py_str("test was positive"), py_str("test is positive"), py_str("tests were negative"), py_str("tests are negative")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("mean"), py_str("means"), py_str("meant"), py_str("pregnant"), py_str("pregnancy"), py_str("test"), py_str("tests"), py_str("positive"), py_str("got"), py_str("have"), py_str("see"), py_str("showing"), py_str("shows")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("late"), py_str("missed"), py_str("days"), py_str("weeks"), py_str("hasnt"), py_str("not come"), py_str("where"), py_str("why"), py_str("when"), py_str("worry"), py_str("normal"), py_str("pregnant"), py_str("pregnancy"), py_str("test")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("period pain"), py_str("period pains"), py_str("cramps"), py_str("heavy period"), py_str("period pads"), py_str("menopause"), py_str("hot flushes"), py_str("contraception"), py_str("condom"), py_str("morning after"), py_str("early"), py_str("days early"), py_str("week early"), py_str("weeks early"), py_str("sooner"), py_str("shorter"), py_str("driving test"), py_str("theory test"), py_str("theory"), py_str("mot"), py_str("exam"), py_str("exams"), py_str("missed my test"), py_str("missed the test"), py_str("missed a test"), py_str("interview"), py_str("deadline"), py_str("payment"), py_str("train"), py_str("bus"), py_str("flight"), py_str("due date"), py_str("weeks pregnant"), py_str("weeks am i"), py_str("trimester"), py_str("maternity"), py_str("due back"), py_str("due to be"), py_str("back at work"), py_str("back to work"), py_str("refund"), py_str("tax return"), py_str("parcel"), py_str("library"), py_str("renewal"), py_str("invoice"), py_str("normal"), py_str("how long"), py_str("how often"), py_str("how many days"), py_str("days between"), py_str("lasts"), py_str("started"), py_str("when do periods"), py_str("first period"), py_str("heavy"), py_str("light"), py_str("boss"), py_str("manager"), py_str("rota"), py_str("my job"), py_str("work late"), py_str("working late"), py_str("work every"), py_str("overtime"), py_str("be paid for"), py_str("paid for it"), py_str("get paid"), py_str("wages"), py_str("salary"), py_str("hourly"), py_str("hours i work"), py_str("for the hours")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("heavy"), py_str("light"), py_str("late"), py_str("missed"), py_str("pregnant"), py_str("pregnancy"), py_str("test"), py_str("tests"), py_str("period pain"), py_str("cramps"), py_str("tank"), py_str("tanks"), py_str("aquarium"), py_str("fish tank"), py_str("nitrogen cycle"), py_str("ammonia"), py_str("nitrite"), py_str("nitrate"), py_str("probation"), py_str("probationary"), py_str("on probation"), py_str("trial period"), py_str("trial shift"), py_str("contract"), py_str("contracts"), py_str("employment"), py_str("employer"), py_str("job offer"), py_str("new job"), py_str("at work"), py_str("sentence"), py_str("court"), py_str("parole"), py_str("dishwasher"), py_str("dishwashers"), py_str("washing machine"), py_str("washer"), py_str("washer dryer"), py_str("tumble dryer"), py_str("dryer"), py_str("spin cycle"), py_str("wash cycle"), py_str("bike"), py_str("bikes"), py_str("bicycle"), py_str("bicycles"), py_str("cycle to work"), py_str("cycling to work"), py_str("learn to cycle"), py_str("learning to cycle"), py_str("pedal"), py_str("pedals"), py_str("pedalling"), py_str("saddle"), py_str("ride a bike"), py_str("riding a bike"), py_str("learn to ride"), py_str("wash"), py_str("washing"), py_str("washed"), py_str("laundry"), py_str("programme"), py_str("program"), py_str("quick wash"), py_str("eco"), py_str("cycle ride"), py_str("cycle lane"), py_str("cycle path"), py_str("cycle route")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("due date"), py_str("due date is"), py_str("when am i due"), py_str("when is my due"), py_str("whens my due"), py_str("when's my due"), py_str("baby due"), py_str("when is the baby due"), py_str("how far along"), py_str("weeks pregnant am i"), py_str("weeks am i"), py_str("how many weeks pregnant"), py_str("how pregnant am i"), py_str("how many weeks am i"), py_str("pregnancy weeks"), py_str("when will the baby"), py_str("when will my baby"), py_str("when is the baby born"), py_str("how long left of my pregnancy"), py_str("how many weeks left of my pregnancy"), py_str("weeks pregnant"), py_str("how many wks am i"), py_str("how many weeks along"), py_str("how many weeks into"), py_str("how many weeks gone"), py_str("what week am i"), py_str("which week am i"), py_str("how many weeks is a pregnancy"), py_str("how long is a pregnancy"), py_str("when are you due"), py_str("when is she due"), py_str("when is he due"), py_str("baby is due"), py_str("last period"), py_str("lmp"), py_str("first day of my last period"), py_str("period started"), py_str("my period started"), py_str("how many months")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("when"), py_str("what"), py_str("how many"), py_str("how far"), py_str("date"), py_str("weeks"), py_str("days"), py_str("am i"), py_str("due"), py_str("work out"), py_str("worked out"), py_str("calculate"), py_str("count"), py_str("left"), py_str("born"), py_str("is it"), py_str("will it"), py_str("how long")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("when"), py_str("whens"), py_str("how many"), py_str("how far"), py_str("how long"), py_str("which date"), py_str("what date"), py_str("due date"), py_str("weeks"), py_str("week"), py_str("months"), py_str("calculate"), py_str("worked out"), py_str("left"), py_str("am i"), py_str("is"), py_str("are"), py_str("will"), py_str("last period"), py_str("first day"), py_str("period started"), py_str("lmp")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("am i pregnant"), py_str("could i be pregnant"), py_str("test was"), py_str("test is"), py_str("two lines"), py_str("one line"), py_str("negative"), py_str("positive"), py_str("missed period"), py_str("late period"), py_str("period late"), py_str("contraction"), py_str("contractions"), py_str("induction"), py_str("c section"), py_str("c-section"), py_str("epidural"), py_str("maternity pay"), py_str("maternity leave"), py_str("paternity"), py_str("birth certificate"), py_str("names"), py_str("name"), py_str("nursery"), py_str("cot"), py_str("rent"), py_str("bill"), py_str("bills"), py_str("payment"), py_str("pay"), py_str("invoice"), py_str("loan"), py_str("library"), py_str("essay"), py_str("assignment"), py_str("coursework"), py_str("homework"), py_str("deadline"), py_str("tax"), py_str("mot"), py_str("insurance"), py_str("subscription")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("labour"), py_str("labor"), py_str("birth"), py_str("give birth"), py_str("in labour"), py_str("contractions"), py_str("waters"), py_str("first stage"), py_str("second stage"), py_str("third stage"), py_str("pushing"), py_str("pushing stage"), py_str("placenta"), py_str("stage of labour")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("how long"), py_str("how many hours"), py_str("hours"), py_str("long"), py_str("longer"), py_str("shorter"), py_str("quicker"), py_str("faster"), py_str("slow"), py_str("take"), py_str("last"), py_str("how soon"), py_str("longest"), py_str("too long"), py_str("stalled"), py_str("stalling"), py_str("go on")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("bin"), py_str("bin bag"), py_str("tea"), py_str("teabag"), py_str("teabags"), py_str("shopping"), py_str("dog"), py_str("school"), py_str("rucksack"), py_str("backpack"), py_str("handbag"), py_str("luggage"), py_str("suitcase"), py_str("sleeping bag")]) }
    fn __data12() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-soft-cheese")); __args.push(py_tuple(vec![__data13(), g_PREGNANT()])); __args.push(py_str("Skip it unless it is cooked until steaming hot: brie, camembert, soft blue cheeses and any mould-ripened cheese can carry listeria, and listeria can reach the baby. Baked camembert or brie in a hot dish is fine because the heat kills it. Hard cheese is a different story - cheddar, parmesan, and the tubs of cream cheese, cottage cheese and mozzarella are all fine, because they are made with pasteurised milk.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("should"), py_str("what about")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-cheese-list")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("cheeses"), py_str("dairy")]), g_PREGNANT()])); __args.push(py_str("Hard cheese is all fine - cheddar, parmesan, gruyere - and so are feta, halloumi, mozzarella, cottage cheese and cream cheese, because they are made with pasteurised milk. The ones to cook until steaming or leave alone are the mould-ripened and blue ones: brie, camembert, stilton, gorgonzola, and any soft cheese with a rind. Skip anything labelled unpasteurised.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("which"), py_str("what"), py_str("can i"), py_str("is it"), py_str("safe"), py_str("ok"), py_str("avoid"), py_str("eat"), py_str("list")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-feta-halloumi")); __args.push(py_tuple(vec![py_tuple(vec![py_str("feta"), py_str("halloumi"), py_str("goats cheese"), py_str("goat cheese"), py_str("cottage cheese"), py_str("cream cheese"), py_str("mozzarella"), py_str("cheddar"), py_str("parmesan"), py_str("hard cheese"), py_str("paneer")]), g_PREGNANT()])); __args.push(py_str("Fine, and this is where people cut out more than they need to: feta, halloumi, mozzarella, cottage cheese, cream cheese and all the hard cheeses - cheddar, parmesan, paneer - are made with pasteurised milk, and listeria is the only reason soft cheese is a worry. Check the label says pasteurised if it is a farm shop one; the supermarket tubs always are.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("have")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-raw-milk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("unpasteurised"), py_str("unpasteurized"), py_str("raw milk"), py_str("raw cheese"), py_str("farmhouse cheese"), py_str("goats milk"), py_str("sheeps milk"), py_str("raw dairy")]), g_PREGNANT()])); __args.push(py_str("No: anything made with unpasteurised milk can carry listeria and campylobacter, so leave the raw-milk cheese and the milk straight from the farm. Pasteurised milk and cheese are fine, and most goat and sheep cheese sold in supermarkets is pasteurised - the label will say so.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("drink")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-pate-liver")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pate"), py_str("pates"), py_str("liver"), py_str("liver sausage"), py_str("liver pate"), py_str("chicken liver"), py_str("foie gras"), py_str("haggis"), py_str("liver and onions"), py_str("cod liver oil"), py_str("vitamin a")]), g_PREGNANT()])); __args.push(py_str("No pate and no liver, and that includes vegetable pate: liver is so high in vitamin A that too much of it can harm the baby, and pate can carry listeria. The same goes for cod liver oil and any multivitamin with vitamin A in it - take folic acid and vitamin D instead, and ask the midwife about anything else.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("take")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-raw-meat")); __args.push(py_tuple(vec![__data14(), g_PREGNANT()])); __args.push(py_str("Cook meat right through: no rare steak, no pink lamb, and nothing raw or cured unless it is cooked into a hot dish. The worry is toxoplasmosis, which is in raw meat, and salami, chorizo, Parma ham and prosciutto are cured rather than cooked. Well-done steak, cooked chicken, ham from the deli counter and hot dogs heated until steaming are all fine.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("have")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-raw-fish")); __args.push(py_tuple(vec![__data15(), g_PREGNANT()])); __args.push(py_str("Yes, with one rule. Raw is the part to skip: cooked prawns, mussels that have opened in the pan, tinned tuna and smoked salmon are all fine in the UK, and so is sushi made with cooked or frozen fish. What to leave alone is raw fish and raw shellfish - no oysters, no raw prawns - because of listeria and norovirus. Cooked fish is good for you and the baby too: salmon, mackerel and sardines are the ones to have, twice a week.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("have")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-big-fish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("swordfish"), py_str("shark"), py_str("marlin"), py_str("tuna"), py_str("tinned tuna"), py_str("tuna steak"), py_str("oily fish"), py_str("mercury")]), g_PREGNANT()])); __args.push(py_str("Skip swordfish, shark and marlin completely, and keep tuna to two fresh steaks or four medium tins a week: those three are the fish with the most mercury, and tuna is the one people eat most of. Oily fish like salmon, mackerel and sardines is good for the baby's brain - two portions a week, and no more than one of them tuna.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("how much"), py_str("how many")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-peanuts")); __args.push(py_tuple(vec![py_tuple(vec![py_str("peanuts"), py_str("peanut"), py_str("nuts"), py_str("nut"), py_str("almonds"), py_str("peanut butter"), py_str("tree nuts")]), g_PREGNANT()])); __args.push(py_str("Peanuts and nuts are fine in pregnancy, and avoiding them does not prevent an allergy - the old advice to cut them out has been dropped. The exception is if you are allergic yourself, or the midwife or GP has told you to avoid them. Nuts are a good snack: a handful with an apple beats a biscuit at three in the afternoon.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("eat"), py_str("avoid"), py_str("have")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-herbal")); __args.push(py_tuple(vec![py_tuple(vec![py_str("herbal tea"), py_str("herbal teas"), py_str("green tea"), py_str("fennel"), py_str("liquorice"), py_str("raspberry leaf"), py_str("camomile"), py_str("chamomile"), py_str("peppermint tea")]), g_PREGNANT()])); __args.push(py_str("Keep herbal teas to a couple of cups a day and don't treat any of them as medicine: liquorice is the one to avoid (it can raise blood pressure), and raspberry leaf is usually left until the last weeks and only with the midwife's agreement. Ordinary tea, decaf tea and water are the easy answers.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("can i"), py_str("is it"), py_str("ok"), py_str("safe"), py_str("drink"), py_str("have")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-listeria")); __args.push(py_tuple(vec![py_tuple(vec![py_str("listeria"), py_str("listeriosis"), py_str("food poisoning"), py_str("food hygiene"), py_str("toxoplasmosis")]), g_PREGNANT()])); __args.push(py_str("Listeria is a bug that is rare but dangerous in pregnancy because it can cross to the baby, and it grows in food kept cold rather than in food left out - soft cheese, pate, smoked fish and cooked meats from a deli are the usual culprits. That is why the advice is to heat leftovers until steaming, wash the veg, and skip those foods. It usually shows as a mild flu-like illness, so mention any fever to the midwife.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("is"), py_str("what"), py_str("can i"), py_str("should"), py_str("worried"), py_str("dangerous"), py_str("risk"), py_str("how")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-what-can-i-eat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("eat"), py_str("eating"), py_str("food"), py_str("foods"), py_str("diet"), py_str("avoid"), py_str("anything"), py_str("menu")]), g_PREGNANT()])); __args.push(g_LIST()); __kws.extend(vec![("ask".to_string(), __data16()), ("avoid".to_string(), g_NOT_FOOD())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-hot-bath")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hot bath"), py_str("bath"), py_str("baths"), py_str("bathwater"), py_str("hot tub"), py_str("jacuzzi"), py_str("sauna"), py_str("steam room"), py_str("spa"), py_str("warm bath")]), g_PREGNANT()])); __args.push(py_str("Warm is fine, hot is the part to avoid: keep the water comfortable rather than steaming, and get out if you feel light-headed. Hot tubs, saunas and steam rooms are best skipped for the first 12 weeks, because raising your core temperature early on has been linked to problems with the baby's spine. A warm bath, a shower and a swim are all fine right through.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("allowed"), py_str("can i"), py_str("should i"), py_str("is it"), py_str("is a"), py_str("how hot"), py_str("how warm")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_NOT_FOOD(), &py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("toddler"), py_str("child"), py_str("clean the"), py_str("cleaning"), py_str("limescale"), py_str("mould"), py_str("drain")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-flying")); __args.push(py_tuple(vec![__data17(), g_PREGNANT()])); __args.push(py_str("Flying is usually fine: most airlines take you up to 36 weeks for one baby, and 32 weeks for twins, and some ask for a letter from the midwife from 28 weeks, so check your airline before you book. Drink water, get up and walk every hour or so and wear compression socks on a long flight. Ask the midwife first if you have had bleeding, high blood pressure, pre-eclampsia or twins.")); __kws.extend(vec![("ask".to_string(), __data18()), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_FOOD()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("fly"), py_str("flying"), py_str("flight"), py_str("travel")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("book a flight"), py_str("price"), py_str("fare"), py_str("delayed"), py_str("cancelled"), py_str("luggage"), py_str("legroom"), py_str("jet lag")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-hair-dye")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hair dye"), py_str("dye my hair"), py_str("dye your hair"), py_str("dyeing my hair"), py_str("colour my hair"), py_str("color my hair"), py_str("hair colour"), py_str("hair color"), py_str("highlights"), py_str("bleach my hair"), py_str("hair treatment"), py_str("salon")]), op_add(&g_PREGNANT(), &py_tuple(vec![py_str("breastfeeding"), py_str("breastfeed"), py_str("breast feeding"), py_str("nursing")]))])); __args.push(py_func("_preg_hair_dye", |args: &[Py]| _preg_hair_dye(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("allowed"), py_str("can i"), py_str("should i"), py_str("is it"), py_str("when can i"), py_str("still")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_FOOD()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("breastfeeding"), py_str("breastfeed"), py_str("nursing")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("cut"), py_str("cuts"), py_str("trim"), py_str("hairdresser"), py_str("fringe"), py_str("grey"), py_str("thinning"), py_str("hair loss"), py_str("hair falling"), py_str("shampoo"), py_str("conditioner"), py_str("dandruff")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-exercise")); __args.push(py_tuple(vec![g_PREGNANT_MOVEMENT(), g_PREGNANT()])); __args.push(py_str("Keep doing what you already do, at a pace where you can still hold a conversation: 150 minutes a week of walking, swimming, yoga, pilates or light weights is what the guidance recommends, and it is good for you and the baby. Leave out contact sports and anything with a fall risk such as riding, skiing or cycling on the road, and in a class do not lie flat on your back after 16 weeks. Stop and ring your midwife if you bleed, leak fluid, feel dizzy or very breathless, and ask her first if you have a condition or were not active before.")); __kws.extend(vec![("ask".to_string(), __data19()), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data20())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-headache")); __args.push(py_tuple(vec![py_tuple(vec![py_str("headache"), py_str("headaches"), py_str("migraine"), py_str("migraines"), py_str("head ache"), py_str("head aches"), py_str("my head hurts"), py_str("my head is banging"), py_str("head is pounding"), py_str("head pounding"), py_str("splitting headache"), py_str("bad head")]), __data21()])); __args.push(g_HEAD_PREGNANCY_REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("_said_pregnant", |args: &[Py]| _said_pregnant(args))), ("avoid".to_string(), __data22())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-any-food")); __args.push(py_tuple(vec![op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PREG_WORRY_FOODS()) { let __unpk6944 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk6944[0].clone(); let mut _u: Py = __unpk6944[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &__data23()), g_PREGNANT()])); __args.push(py_func("_preg_food", |args: &[Py]| _preg_food(args))); __kws.extend(vec![("ask".to_string(), __data24()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_NOT_FOOD(), &g_PREGNANT_MOVEMENT()), &__data25()), &g_DRINKS_WITH_THEIR_OWN_ANSWER()), &py_tuple(vec![py_str("milk"), py_str("a pint of milk"), py_str("dairy"), py_str("milkshake"), py_str("milkshakes"), py_str("hot chocolate")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-milk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("milk"), py_str("a pint of milk"), py_str("dairy"), py_str("milkshake"), py_str("milkshakes"), py_str("hot chocolate")]), __data26()])); __args.push(py_str("Pasteurised milk is fine, and worth drinking: about three servings of dairy a day covers the calcium and iodine a baby needs. The one to skip is unpasteurised milk and anything made with it - some farm-shop milk, and raw-milk cheese - because of listeria. If milk turns your stomach early on, yoghurt, cheese and a fortified plant milk cover the same ground.")); __kws.extend(vec![("avoid".to_string(), __data27()), ("guard".to_string(), py_func("_said_pregnant", |args: &[Py]| _said_pregnant(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-am-i")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pregnant"), py_str("pregnancy")]), op_add(&op_add(&g_AM_I(), &g_TEST_WORDS()), &g_WHEN())])); __args.push(g_TEST_REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_FOOD()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("test"), py_str("tests")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_TEST()), &g_PREGNANT_ALREADY()), &g_FERTILITY()), &__data28()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-two-lines")); __args.push(py_tuple(vec![py_tuple(vec![py_str("two lines"), py_str("both lines")]), __data29()])); __args.push(g_POSITIVE_REPLY()); __kws.extend(vec![("avoid".to_string(), op_add(&g_NOT_A_TEST(), &py_tuple(vec![py_str("negative"), py_str("not pregnant")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-test-positive")); __args.push(py_tuple(vec![py_tuple(vec![py_str("positive"), py_str("two lines"), py_str("both lines"), py_str("faint line"), py_str("faint"), py_str("it says"), py_str("says positive")]), g_TEST_WORDS()])); __args.push(g_POSITIVE_REPLY()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("should"), py_str("do i"), py_str("is"), py_str("does"), py_str("test"), py_str("result"), py_str("now"), py_str("next"), py_str("so")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_NOT_A_TEST(), &py_tuple(vec![py_str("negative"), py_str("not pregnant")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-test-negative")); __args.push(py_tuple(vec![py_tuple(vec![py_str("negative"), py_str("not pregnant"), py_str("one line"), py_str("no line"), py_str("no lines"), py_str("nothing there")]), g_TEST_WORDS()])); __args.push(g_NEGATIVE_REPLY()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("what"), py_str("should"), py_str("do i"), py_str("is"), py_str("does"), py_str("test"), py_str("result"), py_str("now"), py_str("next"), py_str("so"), py_str("can i"), py_str("how")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_NOT_A_TEST(), &py_tuple(vec![py_str("positive")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-late-period")); __args.push(py_tuple(vec![g_WHEN(), __data30()])); __args.push(g_LATE_REPLY()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("my period"), py_str("period"), py_str("late"), py_str("missed"), py_str("why"), py_str("when"), py_str("should"), py_str("is it"), py_str("how long"), py_str("what")])), ("guard".to_string(), py_func("_a_late_that_is_not_a_period", |args: &[Py]| _a_late_that_is_not_a_period(args))), ("avoid".to_string(), op_add(&__data31(), &g_FERTILITY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-cycle-normal")); __args.push(py_tuple(vec![py_tuple(vec![py_str("period"), py_str("periods"), py_str("cycle"), py_str("cycles")]), op_add(&g_CYCLE_SHAPE(), &py_tuple(vec![py_str("how often"), py_str("typical"), py_str("average"), py_str("length"), py_str("between")]))])); __args.push(g_CYCLE_REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_a_late_that_is_not_a_period", |args: &[Py]| _a_late_that_is_not_a_period(args))), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_TEST(), &g_NOT_FOOD()), &__data32()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-pms-days")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pms"), py_str("pmt"), py_str("premenstrual"), py_str("pre menstrual"), py_str("pre-menstrual")]), py_tuple(vec![py_str("how many days"), py_str("how long"), py_str("when"), py_str("before"), py_str("last"), py_str("lasts"), py_str("start"), py_str("starts"), py_str("days"), py_str("week"), py_str("weeks"), py_str("early")])])); __args.push(py_str("Premenstrual symptoms usually start a week or two before a period and are at their worst in the five days before it, then ease within a day or two of bleeding starting. Track them for two or three cycles - the pattern is what tells you it is the cycle and not the week. If it is bad enough to wreck two weeks in four, or the mood change is severe, that is worth the GP: PMDD is real and treatable.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&g_NOT_FOOD(), &g_NOT_A_TEST()), &py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("test"), py_str("tests"), py_str("missed"), py_str("late"), py_str("period pain"), py_str("cramps")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-early-period")); __args.push(py_tuple(vec![py_tuple(vec![py_str("period"), py_str("periods"), py_str("cycle")]), py_tuple(vec![py_str("early"), py_str("days early"), py_str("week early"), py_str("weeks early"), py_str("sooner"), py_str("shorter"), py_str("before i expected"), py_str("earlier than usual"), py_str("two weeks early")])])); __args.push(g_EARLY_REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&g_NOT_FOOD(), &g_NOT_A_TEST()), &py_tuple(vec![py_str("late"), py_str("missed"), py_str("period pain"), py_str("cramps"), py_str("heavy")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-due-date")); __args.push(py_tuple(vec![__data33(), __data34()])); __args.push(py_func("_preg_due_date", |args: &[Py]| _preg_due_date(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), __data35()), ("shape".to_string(), py_tuple(vec![py_func("_gives_a_month", |args: &[Py]| _gives_a_month(args))])), ("guard".to_string(), py_func("_asks_when_due", |args: &[Py]| _asks_when_due(args))), ("avoid".to_string(), op_add(&op_add(&g_NOT_FOOD(), &g_NOT_A_TEST()), &__data36()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-labour-length")); __args.push(py_tuple(vec![__data37(), __data38()])); __args.push(py_func("_preg_labour_length", |args: &[Py]| _preg_labour_length(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_asks_a_birth_time", |args: &[Py]| _asks_a_birth_time(args))), ("avoid".to_string(), g__NOT_THE_BIRTH())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("preg-hospital-bag")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hospital bag"), py_str("hospital bags"), py_str("maternity bag"), py_str("bag for hospital"), py_str("bag for the hospital"), py_str("bags for hospital"), py_str("hospital")]), py_tuple(vec![py_str("pack"), py_str("packed"), py_str("packing"), py_str("when"), py_str("what to take"), py_str("what to bring")])])); __args.push(py_func("_preg_bag", |args: &[Py]| _preg_bag(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("_asks_about_the_bag", |args: &[Py]| _asks_about_the_bag(args))), ("avoid".to_string(), __data39())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy.ENTRIES"); }
    s_ENTRIES(__data12());
    fn __data40() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("c section"), py_str("c-section"), py_str("caesarean"), py_str("caesarian"), py_str("cesarean"), py_str("cesarian"), py_str("caesarean section"), py_str("cesarean section")]), py_str("A caesarean is the baby delivered through a cut in the tummy and womb, usually because labour is not progressing, the baby is breech or either of you is unwell. It is a major operation: about 45 minutes, a spinal so you are awake, and roughly four to six weeks of careful lifting afterwards. An emergency one is done for a reason found in labour, not because anything went wrong on your part.")]), py_tuple(vec![py_tuple(vec![py_str("doula"), py_str("doulas")]), py_str("A doula is a person who supports you through pregnancy, birth and the first weeks - practical help and someone who stays with you - but she is not a midwife and does no clinical care. Doulas are hired privately, usually GBP 300 to 800, and there are free volunteer doulas in many areas; ask what she does at a hospital birth, as some trusts limit how many people are in the room.")]), py_tuple(vec![py_tuple(vec![py_str("midwife"), py_str("midwives")]), py_str("A midwife is the trained professional who looks after a straightforward pregnancy, birth and the first weeks, and she is who you see at the antenatal appointments. She is a nurse or a direct-entry midwife with a degree, and she can order scans and blood tests, deliver the baby and stitch a tear. A doctor takes over only if something needs one.")]), py_tuple(vec![py_tuple(vec![py_str("epidural"), py_str("epidurals")]), py_str("An epidural is pain relief given through a fine tube in the back, numbing from the waist down while you stay awake. It takes about 20 minutes to set up and 15 to 20 to work, and it can make your legs heavy and slow labour, so it is usually offered once labour is established. You can still push with one; ask early if you want one, because it needs an anaesthetist.")]), py_tuple(vec![py_tuple(vec![py_str("birth plan"), py_str("birth plans")]), py_str("A birth plan is a page saying what you would prefer - who is with you, how you want pain relief, water, feeding, and what to do if things change - written so a midwife can read it in a minute. Keep it to one side of A4 and put your first two wishes at the top: the plan that helps is a short list of preferences, not a script, because birth often goes its own way.")]), py_tuple(vec![py_tuple(vec![py_str("induced"), py_str("induction"), py_str("inductions"), py_str("being induced"), py_str("induced labour")]), py_str("Induction is starting labour with medicine or a balloon instead of waiting for it to start itself, usually offered if you go past about 41 weeks or there is a reason the baby is better out. It is a prostaglandin tablet or gel first, then a drip if needed, and it can take two or three days from the first dose to the baby. You can ask what the reason is and what happens if you wait.")]), py_tuple(vec![py_tuple(vec![py_str("water birth"), py_str("water births"), py_str("birthing pool"), py_str("birth pool")]), py_str("A water birth is labouring, and sometimes delivering, in a deep pool of warm water, which takes the weight off and often eases the pain of contractions. Many birth centres and some labour wards have pools; ask early, as there may be one and it may be taken. You can get out at any point, and being in water is not offered if there are complications.")]), py_tuple(vec![py_tuple(vec![py_str("colostrum"), py_str("colostrum harvesting")]), py_str("Colostrum is the first milk: thick, yellow and made in small amounts in the last weeks and the first days, and it is all a newborn needs - a teaspoon at a time, full of antibodies. It acts as a laxative that clears the meconium, which is why the first dirty nappies come. Hand expressing a little into a syringe before the birth is worth asking about if your baby may need extra.")]), py_tuple(vec![py_tuple(vec![py_str("membrane sweep"), py_str("stretch and sweep"), py_str("sweep"), py_str("sweeps")]), py_str("A membrane sweep is a midwife's gloved finger passing over the cervix to separate the membranes, which releases hormones and can start labour - offered from about 40 weeks if you are keen to avoid induction. It takes a minute, feels like a smear test and a period cramp, and works for roughly one in eight. You can say no and ask again in a week.")]), py_tuple(vec![py_tuple(vec![py_str("ventouse"), py_str("ventouse delivery"), py_str("vacuum delivery"), py_str("vacuum extraction")]), py_str("A ventouse is a soft cup on the baby's head that a doctor pulls with, to help in the last stage when pushing is not moving the baby or either of you is tiring. It leaves a bruise and a cone-shaped head that settles in a few days. Forceps do the same job with metal tongs; the doctor will explain which they want and why.")]), py_tuple(vec![py_tuple(vec![py_str("episiotomy"), py_str("episiotomies")]), py_str("An episiotomy is a small cut made at the bottom of the vagina during birth to make room, done with local anaesthetic when the baby needs to come quickly or the tissue is about to tear badly. It is stitched and heals in two to three weeks, and it is not routine - a tear that happens on its own is often smaller and heals better, so not every birth needs one.")]), py_tuple(vec![py_tuple(vec![py_str("nicu"), py_str("scbu"), py_str("special care baby unit"), py_str("neonatal unit")]), py_str("NICU is the neonatal intensive care unit, and SCBU the special care baby unit - where a baby goes if they are early, small or need help breathing or feeding. You can visit almost all the time, ask to do the nappy changes and feeds, and ask the nurse to explain every tube and number: babies on these units are usually there for warmth, feeding and time.")]), py_tuple(vec![py_tuple(vec![py_str("labour"), py_str("in labour"), py_str("going into labour")]), py_str("Labour is the womb contracting to open the cervix and push the baby out: for a first baby it usually takes 8 to 18 hours from regular contractions, and it comes in three stages - the cervix opening to 10 cm, the pushing, and the placenta. Call the midwife when contractions are regular, getting longer and stronger, about every five minutes.")]), py_tuple(vec![py_tuple(vec![py_str("placenta"), py_str("the afterbirth"), py_str("afterbirth")]), py_str("The placenta is the organ on the womb wall that feeds the baby through the cord, and it comes out as the third stage of labour, usually within 15 to 30 minutes of the birth. You can have an injection to speed that up or wait for it to happen on its own. It is checked whole afterwards, which is the one reason to look at it if you want to.")]), py_tuple(vec![py_tuple(vec![py_str("birth centre"), py_str("birth centres"), py_str("midwife led unit"), py_str("midwifery led unit")]), py_str("A birth centre is a midwife-led unit for a straightforward birth - a room with a pool, your own space, no doctors on site and usually a shorter stay, so it suits a pregnancy with no complications. If something needs a doctor you are transferred to the labour ward, which is usually minutes away. Ask for a look round at about 30 weeks.")]), py_tuple(vec![py_tuple(vec![py_str("home birth"), py_str("home births"), py_str("birth at home")]), py_str("A home birth means two midwives come to you, with gas and air and a pool if you have one, and you are transferred to hospital by ambulance if it is needed. It is offered for a straightforward second or later pregnancy, and it is your choice to ask for it - about 2 in 100 births in the UK happen at home.")]), py_tuple(vec![py_tuple(vec![py_str("perineal tear"), py_str("tear"), py_str("tears"), py_str("second degree tear"), py_str("third degree tear"), py_str("a tear")]), py_str("A tear is the skin at the bottom of the vagina stretching and splitting as the baby comes: most are small, most need a few stitches with local anaesthetic, and they heal in two to three weeks. A third or fourth degree tear reaches the muscle and the back passage and is repaired in theatre. Tell your midwife about the pain rather than sitting on it.")]), py_tuple(vec![py_tuple(vec![py_str("lochia"), py_str("postnatal bleeding"), py_str("bleeding after birth")]), py_str("Lochia is the bleeding after birth: heavy red blood for three to five days, then browner and lighter for up to six weeks. Use pads rather than tampons, and get help the same day if you soak a pad in an hour, pass clots bigger than a 50p coin, or the blood smells bad or you feel feverish - that is a sign to have checked.")]), py_tuple(vec![py_tuple(vec![py_str("heel prick test"), py_str("newborn blood spot"), py_str("blood spot test"), py_str("heel prick")]), py_str("The heel prick test is the newborn blood spot test, done at five days old: a few drops of blood from the heel onto a card, checked for nine rare conditions including cystic fibrosis, sickle cell and an underactive thyroid. It takes a minute and the baby cries briefly. It is offered to every baby and you can ask what each result means.")]), py_tuple(vec![py_tuple(vec![py_str("mucus plug"), py_str("bloody show"), py_str("show in pregnancy"), py_str("a show in pregnancy"), py_str("show")]), py_str("The mucus plug is the plug of mucus that seals the cervix, and it comes away as a thick jelly, sometimes streaked with blood - the 'show'. It can be days or a fortnight before labour and means nothing on its own; call the midwife if the blood is more than a streak or comes with regular contractions.")]), py_tuple(vec![py_tuple(vec![py_str("nuchal scan"), py_str("nuchal translucency"), py_str("dating scan"), py_str("20 week scan"), py_str("anomaly scan")]), py_str("A nuchal scan is the 12-week scan: it measures the fluid at the back of the baby's neck and, with a blood test, gives a risk figure for Down's, Edwards' and Patau's syndromes - a screening test, not a diagnosis. The 20-week scan checks how the baby is growing and developing. Both are offered, and you can decline or ask for time to think.")]), py_tuple(vec![py_tuple(vec![py_str("skin to skin"), py_str("skin to skin contact"), py_str("kangaroo care")]), py_str("Skin to skin is the naked baby laid on your bare chest, covered with a blanket, in the first hour and as often as you like after. It keeps them warm, steadies their breathing and blood sugar, and is the strongest start to feeding - and it works just as well for a dad or a partner, which is useful when the mother needs stitching or a rest.")]), py_tuple(vec![py_tuple(vec![py_str("forceps"), py_str("forceps delivery"), py_str("kiwi cup")]), py_str("Forceps are the metal tongs a doctor uses to guide the baby out in the last stage, when pushing is not moving them or one of you is tiring. They leave marks on the head that fade in a few days, and you are given pain relief first. They are used in about one birth in fourteen, and a ventouse is the alternative when a cup is enough.")]), py_tuple(vec![py_tuple(vec![py_str("tongue tie"), py_str("tongue-tie"), py_str("tongue tied"), py_str("tongue-tied")]), py_str("A tongue tie is the strip of tissue under the tongue being tight or short, so the tongue cannot lift or reach forward as far as it should - which can make feeding sore and slow. It is common, it is checked at the newborn examination, and a simple snip in the first weeks is quick and needs no anaesthetic. Ask a midwife or health visitor to look if feeding hurts.")]), py_tuple(vec![py_tuple(vec![py_str("gestational diabetes"), py_str("gestational diabetes test"), py_str("glucose tolerance test")]), py_str("Gestational diabetes is high blood sugar that starts in pregnancy, from the placenta's hormones making insulin work less well - it is tested for with a glucose drink and blood tests at 24 to 28 weeks if you are at risk. It is managed with diet, monitoring and sometimes tablets or insulin, and it usually goes after the birth. Nothing you did caused it.")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._BIRTH_WORDS"); }
    s__BIRTH_WORDS(__data40());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._BIRTH_WORD_KEY"); }
    s__BIRTH_WORD_KEY(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BIRTH_WORDS()) { let __unpk7177 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk7177[0].clone(); let mut _u: Py = __unpk7177[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._BIRTH_WORD_NAMES"); }
    s__BIRTH_WORD_NAMES(call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BIRTH_WORDS()) { let __unpk8458 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk8458[0].clone(); let mut reply: Py = __unpk8458[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(py_tuple(vec![word.clone(), reply.clone()])); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| {  let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); call_builtin("len", &[geti(&pair.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)) })), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data41() -> Py { py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what's"), py_str("what are"), py_str("mean"), py_str("means"), py_str("meaning"), py_str("explain"), py_str("definition"), py_str("define"), py_str("stand for"), py_str("difference"), py_str("what does"), py_str("what happens"), py_str("what happens during"), py_str("what happens at"), py_str("what happens in"), py_str("what goes on"), py_str("what goes on during"), py_str("whats it like"), py_str("what is it like")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._BIRTH_ASK"); }
    s__BIRTH_ASK(__data41());
    fn __data42() -> Py { py_tuple(vec![py_str("section"), py_str("show"), py_str("a show"), py_str("tear"), py_str("tears"), py_str("a tear"), py_str("sweep"), py_str("sweeps"), py_str("labour"), py_str("in labour"), py_str("going into labour"), py_str("induction"), py_str("contraction"), py_str("contractions"), py_str("bump")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._AMBIGUOUS"); }
    s__AMBIGUOUS(call_builtin("frozenset", &[__data42()]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._NOT_A_BIRTH_LINE"); }
    s__NOT_A_BIRTH_LINE(stdlib_call("re", "compile", &[py_str("\\blabou?r day\\b|\\bpublic holiday\\b|\\bday off\\b|\\bbank holiday\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._A_BIRTH_LINE"); }
    s__A_BIRTH_LINE(stdlib_call("re", "compile", &[py_str("\\b(?:pregnan\\w*|labour|labor|birth|births|midwife|midwives|due date|weeks|bump|baby|babies|newborn|waters|dilat\\w*|hospital|epidural|cervix)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._WHEN_TO_CALL"); }
    s__WHEN_TO_CALL(stdlib_call("re", "compile", &[py_str("\\b(?:how far apart|how often|how close|close together|when should i (?:go|call)|how long (?:do|should) (?:they|contractions) last|timing|time them)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._DO_THEY_HURT"); }
    s__DO_THEY_HURT(stdlib_call("re", "compile", &[py_str("\\b(?:hurt|hurts|painful|pain|supposed to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._HOW_FAR_APART"); }
    s__HOW_FAR_APART(py_str("Time them from the start of one to the start of the next, and call the midwife when they are about five minutes apart, each one lasting 45 to 60 seconds, for an hour - or straight away if your waters break, you bleed, the baby stops moving or you feel frightened. A first labour is usually 8 to 18 hours, so early ones at ten or fifteen minutes apart are the time to rest and eat rather than go in."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._THEY_HURT"); }
    s__THEY_HURT(py_str("Yes, contractions hurt - it is a strong muscle working, and for most people it is the hardest pain they meet. It peaks and then stops completely between contractions, which is what makes it bearable, and the relief you can ask for is real: gas and air, a pool, diamorphine or an epidural. Say what you want early rather than waiting to see how bad it gets."));
    fn __data43() -> Py { py_tuple(vec![py_str("bleeding"), py_str("bled"), py_str("waters broke"), py_str("my waters"), py_str("pain"), py_str("pains"), py_str("hurt"), py_str("hurts"), py_str("cramp"), py_str("cramps"), py_str("contractions are"), py_str("how much"), py_str("cost"), py_str("costs"), py_str("price"), py_str("pay"), py_str("should i"), py_str("shall i"), py_str("or not"), py_str("worth it"), py_str("safe"), py_str("dangerous"), py_str("risk"), py_str("how do i"), py_str("how to"), py_str("how long"), py_str("when do i"), py_str("when should i"), py_str("appointment"), py_str("results"), py_str("not offered"), py_str("decline"), py_str("choose"), py_str("decide"), py_str("my friend"), py_str("my sister"), py_str("my wife"), py_str("my partner"), py_str("for my")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("birth-words")); __args.push(py_tuple(vec![g__BIRTH_WORD_KEY(), g__BIRTH_ASK()])); __args.push(py_func("_a_birth_word", |args: &[Py]| _a_birth_word(args))); __kws.extend(vec![("ask".to_string(), g__BIRTH_ASK()), ("avoid".to_string(), __data43()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-contraction-has-a-birth-in-it")); __args.push(py_tuple(vec![py_tuple(vec![py_str("contraction"), py_str("contractions")]), op_add(&g__BIRTH_ASK(), &py_tuple(vec![py_str("how far apart"), py_str("how often"), py_str("how long"), py_str("hurt"), py_str("hurts"), py_str("painful"), py_str("supposed to"), py_str("close together")]))])); __args.push(py_func("_a_labour_contraction", |args: &[Py]| _a_labour_contraction(args))); __kws.extend(vec![("ask".to_string(), op_add(&g__BIRTH_ASK(), &py_tuple(vec![py_str("how far apart"), py_str("how often"), py_str("hurt"), py_str("painful")]))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data44() -> Py { py_tuple(vec![py_str("bleed"), py_str("bleeds"), py_str("bled"), py_str("bleeding"), py_str("blood"), py_str("bloods"), py_str("clot"), py_str("clots"), py_str("pad"), py_str("pads"), py_str("lochia"), py_str("postnatal"), py_str("post natal"), py_str("postpartum")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._POSTNATAL_BLEED"); }
    s__POSTNATAL_BLEED(__data44());
    fn __data45() -> Py { py_tuple(vec![py_str("birth"), py_str("births"), py_str("after birth"), py_str("baby"), py_str("babies"), py_str("the baby"), py_str("my baby"), py_str("section"), py_str("c section"), py_str("c-section"), py_str("caesarean"), py_str("cesarean"), py_str("caesarian"), py_str("postnatal"), py_str("post natal"), py_str("postpartum"), py_str("lochia"), py_str("maternity"), py_str("midwife"), py_str("give birth"), py_str("gave birth"), py_str("giving birth"), py_str("delivered"), py_str("weeks after"), py_str("days after"), py_str("born"), py_str("was born"), py_str("having my son"), py_str("having my daughter"), py_str("having a baby"), py_str("had my son"), py_str("had my daughter"), py_str("had a baby"), py_str("months after"), py_str("month after")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.pregnancy._POSTNATAL_BIRTH"); }
    s__POSTNATAL_BIRTH(__data45());
    fn __data46() -> Py { py_tuple(vec![py_str("period"), py_str("periods"), py_str("menstrual"), py_str("between periods"), py_str("miscarriage"), py_str("missed"), py_str("ectopic"), py_str("implantation"), py_str("pregnant"), py_str("pregnancy test"), py_str("after sex"), py_str("sex"), py_str("nappy"), py_str("nappies"), py_str("poo"), py_str("stool"), py_str("stools"), py_str("cord"), py_str("circumcision"), py_str("nose"), py_str("nosebleed"), py_str("gums"), py_str("tooth"), py_str("teeth"), py_str("urine"), py_str("wee"), py_str("cut"), py_str("wound"), py_str("gash"), py_str("stitches in my hand"), py_str("dog"), py_str("cat"), py_str("scar"), py_str("scars"), py_str("scar tissue"), py_str("incision"), py_str("stitches"), py_str("dressing"), py_str("blood test"), py_str("blood tests"), py_str("blood test tomorrow"), py_str("heel prick"), py_str("screening"), py_str("jab"), py_str("jabs"), py_str("vaccine"), py_str("vaccination"), py_str("vaccinated"), py_str("injection"), py_str("immunisation"), py_str("group b strep"), py_str("newborn screening"), py_str("birth control"), py_str("contraception"), py_str("pill"), py_str("pills"), py_str("implant"), py_str("coil"), py_str("iud"), py_str("morning after"), py_str("brain"), py_str("head injury"), py_str("hit my head")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("postnatal-bleeding")); __args.push(py_tuple(vec![g__POSTNATAL_BLEED(), g__POSTNATAL_BIRTH()])); __args.push(py_str("Lochia is the bleeding after birth: heavy red blood for three to five days, then browner and lighter for up to six weeks, coming back a little when you do more - so a bleed still going past six weeks is a GP visit, not lochia. Use pads, not tampons. Ring your maternity unit the same day if you soak a pad in an hour, pass clots bigger than a 50p coin, or the blood smells bad or you are feverish - and 999 or 911 if you are faint, dizzy or flooding.")); __kws.extend(vec![("avoid".to_string(), __data46()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _a_late_that_is_not_a_period(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("False when \"late\" in this line is lateness, and the cycle rows should stand aside.\n\n    \"im on lates this week\" is a week of two-till-ten, not a period that is late: \"lates\" was\n    read as \"late\" and reached the cycle card. A line that names a period as well keeps the\n    entry: \"im on lates and my period is late\" is still a late period.\n\n    The second shape is the same fault without a roster in it: \"they fired me for being late\n    every day\" and \"im late for work again\" were answered with the advice for a period that has\n    not come, because \"late\" is on the key and nothing said which late it was. The reading says\n    it now - `understood[\"sense\"][\"late\"]` is `time` when the line names something a person is\n    late for (a boss, a bus, an interview, a deadline) and `cycle` when it names the cycle - and\n    only `time` changes anything here: a line that says neither keeps the card it had.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&{ let __b = mcall(&understood.clone(), "get", &[py_str("sense")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("late")]), &py_str("time")))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&understood.clone(), "get", &[py_str("shift")])))) {
    return Py::Bool(true);
    }
    text = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:periods?|cycles?|pregnan\\w*|tests?)\\b"), text.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _preg_food(args: &[Py]) -> Py {
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
    let mut joined: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The worry food the line names, by meaning; otherwise the general safe-food answer.");
    words = call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    joined = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    let __iter1 = py_iter_or_raise(&g_PREG_WORRY_FOODS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&name.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&joined.clone(), &name.clone()))) {
    return reply.clone();
    }
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&words.clone(), &name.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&name.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } } } }) {
    return reply.clone();
    }
    }
    }
    return g_PREG_ANY_FOOD();
    Py::None
}
pub fn _gives_a_month(args: &[Py]) -> Py {
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
    let _ = py_str("A month in the line is a day being given, which is the answer to the bot's own question.\n\n    The bot asks for the first day of the last period, and the reply to that names a day and\n    asks nothing: \"my last period was 3 may\", \"my last period started on 3 may\", \"lmp was 3 may\".\n    None of those carries an ask word, so the row scored 0 and the cycle card, the puberty card\n    and the greeting took them. A month is the figure this row counts from, so it is a signal in\n    its own right - the same weight as a key word, so the subject still has to be in the line -\n    and `_asks_when_due` is what keeps a mention in passing (\"baby is due in march\") off it.\n    ");
    return call_builtin("bool", &[mcall(&g__A_MONTH(), "search", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_when_due(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True for a line that asks when a baby is due, and for nothing that only looks like one.");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__DUE_SHAPE(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__ASKED(), "search", &[line.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__GIVES_THE_DAY(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__A_MONTH(), "search", &[line.clone()]) } })) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__SHORT_ASK(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)) } }))) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&{ let __b = mcall(&g__DUE_ELSEWHERE(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_MILESTONE(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__PERIOD_ONLY(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PERIOD_WITH_A_PREGNANCY(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__PREG_WORD(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g__PREG_SOFT(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__OTHER_SUBJECT(), "search", &[line.clone()]))) } };
    Py::None
}
pub fn _the_day(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut date: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "date") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: date"))) };
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut said: Py = Py::None;
    let _ = py_str("A date as a person says it: \"7 February\", with the year only when it is not this one.");
    said = { let mut __s = String::new(); __s.push_str(&fstr(&gattr(&date.clone(), "day"), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&date.clone(), "strftime", &[py_str("%B")]), -1, "")); py_string(__s) };
    return if py_truthy(&Py::Bool(py_eq(&gattr(&date.clone(), "year"), &gattr(&today.clone(), "year")))) { said.clone() } else { { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&date.clone(), "year"), -1, "")); py_string(__s) } };
    Py::None
}
pub fn _the_date_named(args: &[Py]) -> Py {
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
    let mut today: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut date: Py = Py::None;
    let mut day: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut month: Py = Py::None;
    let _ = py_str("The day the line names, or None. A last period is always in the past, so a date still to\n    come in this year is last year's: \"my last period was 3 may\" said in November is 3 May.");
    let __iter1 = py_iter_or_raise(&mcall(&g__DAY_THEN_MONTH(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), mcall(&g__MONTHS(), "get", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "lower", &[])])]), 2, None);
    day = __unpk2[0].clone();
    month = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_is(&month.clone(), &Py::None))) {
    date = _the_date_in_this_year(&[today.clone(), month.clone(), day.clone()]);
    if py_truthy(&date.clone()) {
    return date.clone();
    }
    }
    }
    let __iter3 = py_iter_or_raise(&mcall(&g__MONTH_THEN_DAY(), "finditer", &[line.clone()]));
    for __item in __iter3 {
    match_ = __item.clone();
    let __unpk4 = py_unpack(&py_tuple(vec![mcall(&g__MONTHS(), "get", &[mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[])]), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    month = __unpk4[0].clone();
    day = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_is(&month.clone(), &Py::None))) {
    date = _the_date_in_this_year(&[today.clone(), month.clone(), day.clone()]);
    if py_truthy(&date.clone()) {
    return date.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_date_in_this_year(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut today: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "today") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: today"))) };
    let mut month: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "month") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: month"))) };
    let mut day: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "day") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: day"))) };
    let mut date: Py = Py::None;
    let mut year: Py = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![gattr(&today.clone(), "year"), op_sub(&gattr(&today.clone(), "year"), &Py::Int(1))]));
    for __item in __iter1 {
    year = __item.clone();
    let __try2 = exc::catch(|| {
    date = stdlib_call("datetime", "date", &[year.clone(), month.clone(), day.clone()]).unwrap_or_else(|e| exc::raise(e));
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret3: Py = Py::Bool(false);
    let mut __val4: Py = Py::None;
    let mut __unhandled5: Option<exc::PyExc> = None;
    match __try2 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret3 = __p[0].clone(); __val4 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled6 = false;
            if !__handled6 && exc::matches_any(&__e, &["ValueError"]) {
                __handled6 = true;
    return Py::None;
            }
            if !__handled6 { __unhandled5 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled5 { exc::raise(__e); }
    if py_truthy(&__ret3) { return __val4; }
    if py_truthy(&op_le(&date.clone(), &op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return date.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_weeks_named(args: &[Py]) -> Py {
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
    let mut figure: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("How many weeks along the line says the person is, or None.");
    let __iter1 = py_iter_or_raise(&mcall(&g__WEEKS_NOW(), "finditer", &[line.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    figure = { let __b = mcall(&match_.clone(), "group", &[Py::Int(1)]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[Py::Int(2)]) } };
    if py_truthy(&figure.clone()) {
    weeks = call_builtin("int", &[figure.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __cmp21: Py = weeks.clone(); if py_truthy(&op_le(&Py::Int(1), &__cmp21)) { op_le(&__cmp21, &Py::Int(45)) } else { Py::Bool(false) } }) {
    return weeks.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _preg_due_date(args: &[Py]) -> Py {
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
    let mut date: Py = Py::None;
    let mut days: Py = Py::None;
    let mut left: Py = Py::None;
    let mut line: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut today: Py = Py::None;
    let mut weeks: Py = Py::None;
    let _ = py_str("When the baby is due: the date from the figures the line gives, else the rule and an ask.\n\n    A due date is 40 weeks - 280 days - from the first day of the last period, so a line that\n    gives that day gets the date worked out, and a line that gives how many weeks the person is\n    now gets the weeks that are left counted on from today. A line that gives neither gets the\n    rule itself and one question, because the bot cannot see a date the person has not said.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    today = stdlib_call("datetime", "date.today", &[]).unwrap_or_else(|e| exc::raise(e));
    weeks = _the_weeks_named(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_is(&weeks.clone(), &Py::None))) {
    left = op_sub(&Py::Int(40), &weeks.clone());
    if py_truthy(&op_lt(&left.clone(), &Py::Int(0))) {
    return { let mut __s = String::new(); __s.push_str("At "); __s.push_str(&fstr(&weeks.clone(), -1, "")); __s.push_str(" weeks you are past the 40-week due date, so the date on your notes and your midwife are the ones to go by now. Most places offer an induction by 41 or 42 weeks, and she will talk you through what happens next - worth asking her at your next appointment rather than waiting."); py_string(__s) };
    }
    date = op_add(&today.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), op_mul(&left.clone(), &Py::Int(7)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    return { let mut __s = String::new(); __s.push_str("Week "); __s.push_str(&fstr(&weeks.clone(), -1, "")); __s.push_str(" of 40 leaves "); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str(" weeks to go, which puts the due date at about "); __s.push_str(&fstr(&_the_day(&[date.clone(), today.clone()]), -1, "")); __s.push_str(". That is the 40-week count, but the dating scan at 11 to 14 weeks is the date the midwife goes by, so go by what is on the notes if the scan moved it."); py_string(__s) };
    }
    named = _the_date_named(&[line.clone(), today.clone()]);
    if py_truthy(&Py::Bool(!py_is(&named.clone(), &Py::None))) {
    date = op_add(&named.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("days".to_string(), Py::Int(280))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("datetime", "timedelta", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    said = { let mut __s = String::new(); __s.push_str("Counted from the first day of the last period, "); __s.push_str(&fstr(&_the_day(&[named.clone(), today.clone()]), -1, "")); __s.push_str(", the due date is about "); __s.push_str(&fstr(&_the_day(&[date.clone(), today.clone()]), -1, "")); __s.push_str(" - 40 weeks, or 280 days, later. "); py_string(__s) };
    days = gattr(&op_sub(&today.clone(), &named.clone()), "days");
    if py_truthy(&{ let __cmp11: Py = days.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp11)) { op_le(&__cmp11, &Py::Int(300)) } else { Py::Bool(false) } }) {
    return op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str("That makes you "); __s.push_str(&fstr(&op_floordiv(&days.clone(), &Py::Int(7)), -1, "")); __s.push_str(" weeks now. The dating scan at 11 to 14 weeks is the date the midwife goes by, so if it moved the date, the scan is the one that counts."); py_string(__s) });
    }
    return op_add(&said.clone(), &py_str("If that is not the day you meant, tell me the first day of the last period as a date and I will work it out again."));
    }
    return g_CARD();
    Py::None
}
pub fn _preg_hair_dye(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Dyeing in pregnancy, or while breastfeeding - two different questions.\n\n    The row keys both, and its one card opened on pregnancy: a mother asking about feeding was\n    told it was fine in pregnancy and to wait twelve weeks, which answers a question she did not\n    ask. The feeding answer is shorter and has no twelve-week line in it, so the line decides.\n    ");
    line = op_add(&op_add(&py_str(" "), &call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FEEDING_WORDS()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&line.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A colour is fine while you are breastfeeding: the dye does not get into your milk, so nothing reaches the baby through a feed. Keep the room aired, wear the gloves that come with the kit and do the 48-hour patch test first, because skin can react differently after a baby. Highlights or a semi-permanent keep the dye off your scalp if you would rather be careful, and a salon can do the mixing for you.");
    }
    return py_str("Fine, and there is no evidence hair dye harms a baby: the amount that reaches your bloodstream through the scalp is tiny. Plenty of women wait until after 12 weeks before colouring, which is a reasonable choice rather than a rule. Have the patch test as usual, keep the dye off a cut or an eczema patch, and make sure the room is ventilated if you are doing it at home.");
    Py::None
}
pub fn _the_line(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    Py::None
}
pub fn _asks_a_birth_time(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a question about how long the birth takes, and for nothing that reads like one.");
    line = _the_line(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__TIME_ASK(), "search", &[line.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__BIRTH_WORD(), "search", &[line.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__AFTER_THE_BIRTH(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&line.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g__NOT_THE_BIRTH()]).unwrap_or_else(|e| exc::raise(e)))));
    Py::None
}
pub fn _asks_about_the_bag(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True for \"when do I pack the bag\", so a labour question about going in does not get it.");
    line = _the_line(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__TIME_ASK(), "search", &[line.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bpack\\b|\\bpacked\\b|\\bpacking\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bhospital\\b|\\bmaternity\\b|\\bbag for\\b|\\bdelivery\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _preg_labour_length(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("How long a labour lasts: the figure people ask for, and what \"early labour\" really means.\n\n    Three questions hide behind the same words, and each has its own figure: the whole labour,\n    one contraction, and the stay in hospital afterwards. \"how long do contractions last\" was\n    answered with the 8 to 18 hours of a labour, which is a different question.\n    ");
    line = _the_line(&[understood.clone()]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bin hospital\\b|\\bstay in\\b|\\bstay at\\b|\\bcome home\\b|\\bgoing home\\b|\\bhow long do i stay\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("After a straightforward birth most women are home within a day: a few hours if everything is well and it is not your first, overnight or a day or two with a first baby. A caesarean is two to four days, and any concern about you or the baby adds a day, so plan for the longer end when you arrange a lift home.");
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bcontractions?\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\blabour\\b|\\blabor\\b|\\bthe birth\\b|\\bgiv(?:e|ing) birth\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("Each contraction usually lasts 30 to 60 seconds early on, then 60 to 90 seconds once labour is established, with a few minutes between them. They get longer, stronger and closer together as it goes on - that pattern is what tells you it is real labour rather than the practice tightenings. Ring the midwife if one lasts much longer than 90 seconds, or there is no gap between them.");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\blongest\\b|\\btoo long\\b|\\bstall(?:ed|ing)?\\b|\\bhow long can (?:a |the |my )?(?:labour|labor|it|this) (?:go|last|be)\\b|\\bgo on for\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("There is no fixed limit: a first labour is usually under 18 hours once it is established, but slow is not the same as dangerous, and they go by how it is progressing, how you are and how the baby is rather than the clock. If it stalls they offer to break the waters or a hormone drip, and if it is not moving at all they talk to you about a caesarean. Ask what has changed each time they examine you.");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:first|second|third|1st|2nd|3rd) stage\\b|\\bpushing\\b|\\bplacenta (?:take|takes|come|comes|coming|out)\\b|\\bdelivery of the placenta\\b|\\bskin to skin\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bfirst\\b|\\b1st\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("The first stage is the long part: the cervix opening to 10cm, which takes most of the 8 to 18 hours of a first labour and is much quicker with a second. It starts with contractions you can talk through and ends with strong ones a few minutes apart - that middle stretch, 4 to 6cm, is usually the hardest, so ask for pain relief before you need it.");
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bthird\\b|\\b3rd\\b|\\bplacenta\\b|\\bskin to skin\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("The third stage is the placenta: usually 5 to 15 minutes, and up to half an hour if you wait for it to come away by itself rather than having the injection that speeds it up. They check it is all out, and it is also when any tear is stitched, so it can feel longer than it is.");
    }
    return py_str("The second stage is the pushing: about an hour on average for a first baby, sometimes two, and often 10 or 20 minutes with a second or later one. The urge to push is hard to resist and that is normal - go with it, take the breaks between contractions, and expect the midwife to guide you to slow down at the end so you tear less.");
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bwaters?\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bbreak|broke|broken|gone|went|went off\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("Once your waters break, ring the midwife and go in to be checked - she will want to know the time, the colour and the smell. If labour has not started, most units offer to induce it within about 24 hours, because the risk of infection to you and the baby starts to rise after that. Do not use tampons or have sex once the waters have gone, and ring straight away if the water is green, brown or smelly, or if you feel unwell or the baby is not moving as usual.");
    }
    if py_truthy(&mcall(&g__SECOND_BIRTH(), "search", &[line.clone()])) {
    return py_str("A second or later labour is usually quicker than the first - often 5 to 12 hours from regular contractions to the birth, and some are much faster than that. Because it can move quickly, ring the midwife early rather than timing it at home, and go in when the contractions are strong and regular.");
    }
    return py_str("A first labour usually takes 8 to 18 hours from the first regular contractions to the birth, and a second or later baby is usually quicker. Early labour at home can run much longer than that - a day or more of stop-start contractions is normal - so it is the pattern that matters: contractions getting stronger, longer and closer together, or your waters breaking, is the point to ring the midwife.");
    Py::None
}
pub fn _preg_bag(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("When the hospital bag gets packed, and what goes in it - the ask decides which.");
    line = _the_line(&[understood.clone()]);
    when = py_str("Pack it by about 36 weeks - roughly a month before the due date - so it is ready if you go early. Two bags: yours with your maternity notes, a nightdress or two, big pads, toiletries, a phone charger and snacks; the baby's with nappies, vests, sleepsuits, a hat and a blanket. Keep it by the door, and add the notes last because you need them at appointments before then.");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bwhat (?:should|do) i (?:pack|take|bring)\\b|\\bwhat to (?:pack|take|bring)\\b|\\bwhat goes in\\b"), line.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("For you: maternity notes, a nightdress or two, big pads, toiletries, a phone charger and snacks. For the baby: nappies, vests, sleepsuits, a hat and a blanket. Pack it by about 36 weeks so it is ready if you go early, keep it by the door, and leave the notes on top because you need them at appointments before then.");
    }
    return when.clone();
    Py::None
}
pub fn _said_pregnant(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line itself says the person is pregnant, used as an entry guard.\n\n    The key holds the symptoms, so without this the row took every headache, every ibuprofen\n    question and a dog's bad leg: \"is ibuprofen bad for my kidneys\" was answered about\n    pre-eclampsia.\n    ");
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("text"), py_str("norm_text")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&mcall(&g__PREG_DENIED(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__PREG_SAID(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_birth_word(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The term the line names, defined; the longest name first so \"birth plan\" beats \"birth\".");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    if py_truthy(&mcall(&g__NOT_A_BIRTH_LINE(), "search", &[said.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__BIRTH_WORD_NAMES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__AMBIGUOUS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_BIRTH_LINE(), "search", &[said.clone()]))) } }) {
    continue;
    }
    return reply.clone();
    }
    return Py::None;
    Py::None
}
pub fn _a_labour_contraction(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The contraction of labour, when the line has a birth in it, or asks about the timing.");
    reading = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    said = call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("plain")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g__WHEN_TO_CALL(), "search", &[said.clone()])) {
    return g__HOW_FAR_APART();
    }
    if py_truthy(&mcall(&g__DO_THEY_HURT(), "search", &[said.clone()])) {
    return g__THEY_HURT();
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_BIRTH_LINE(), "search", &[said.clone()])))) {
    return Py::None;
    }
    return py_str("A contraction is the womb muscle tightening and then relaxing, which is what opens the cervix: it builds, holds for 45 to 60 seconds and fades, and the first ones feel like a bad period pain. They get longer, stronger and closer together as labour goes on. Time them from the start of one to the start of the next, and call the midwife when they are about five minutes apart.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.pregnancy".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AM_I" => Some(g_AM_I()),
            "ANSWERS" => Some(g_ANSWERS()),
            "CARD" => Some(g_CARD()),
            "CYCLE_REPLY" => Some(g_CYCLE_REPLY()),
            "CYCLE_SHAPE" => Some(g_CYCLE_SHAPE()),
            "DRINKS_WITH_THEIR_OWN_ANSWER" => Some(g_DRINKS_WITH_THEIR_OWN_ANSWER()),
            "EARLY_REPLY" => Some(g_EARLY_REPLY()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FERTILITY" => Some(g_FERTILITY()),
            "HEAD_PREGNANCY_REPLY" => Some(g_HEAD_PREGNANCY_REPLY()),
            "LATE_REPLY" => Some(g_LATE_REPLY()),
            "LIST" => Some(g_LIST()),
            "NEGATIVE_REPLY" => Some(g_NEGATIVE_REPLY()),
            "NOT_A_TEST" => Some(g_NOT_A_TEST()),
            "NOT_FOOD" => Some(g_NOT_FOOD()),
            "POSITIVE_REPLY" => Some(g_POSITIVE_REPLY()),
            "PREGNANT" => Some(g_PREGNANT()),
            "PREGNANT_ALREADY" => Some(g_PREGNANT_ALREADY()),
            "PREGNANT_MOVEMENT" => Some(g_PREGNANT_MOVEMENT()),
            "PREG_ANY_FOOD" => Some(g_PREG_ANY_FOOD()),
            "PREG_WORRY_FOODS" => Some(g_PREG_WORRY_FOODS()),
            "TEST_REPLY" => Some(g_TEST_REPLY()),
            "TEST_WORDS" => Some(g_TEST_WORDS()),
            "WHEN" => Some(g_WHEN()),
            "_AFTER_THE_BIRTH" => Some(g__AFTER_THE_BIRTH()),
            "_AMBIGUOUS" => Some(g__AMBIGUOUS()),
            "_ASKED" => Some(g__ASKED()),
            "_ASKS_WEEKS" => Some(g__ASKS_WEEKS()),
            "_A_BIRTH_LINE" => Some(g__A_BIRTH_LINE()),
            "_A_MILESTONE" => Some(g__A_MILESTONE()),
            "_A_MONTH" => Some(g__A_MONTH()),
            "_A_PREGNANCY" => Some(g__A_PREGNANCY()),
            "_BIRTH_ASK" => Some(g__BIRTH_ASK()),
            "_BIRTH_WORD" => Some(g__BIRTH_WORD()),
            "_BIRTH_WORDS" => Some(g__BIRTH_WORDS()),
            "_BIRTH_WORD_KEY" => Some(g__BIRTH_WORD_KEY()),
            "_BIRTH_WORD_NAMES" => Some(g__BIRTH_WORD_NAMES()),
            "_DAY_THEN_MONTH" => Some(g__DAY_THEN_MONTH()),
            "_DO_THEY_HURT" => Some(g__DO_THEY_HURT()),
            "_DUE_ELSEWHERE" => Some(g__DUE_ELSEWHERE()),
            "_DUE_SHAPE" => Some(g__DUE_SHAPE()),
            "_FEEDING_WORDS" => Some(g__FEEDING_WORDS()),
            "_GIVES_THE_DAY" => Some(g__GIVES_THE_DAY()),
            "_HOW_FAR_APART" => Some(g__HOW_FAR_APART()),
            "_MONTHS" => Some(g__MONTHS()),
            "_MONTH_THEN_DAY" => Some(g__MONTH_THEN_DAY()),
            "_MY_OWN" => Some(g__MY_OWN()),
            "_NOT_A_BIRTH_LINE" => Some(g__NOT_A_BIRTH_LINE()),
            "_NOT_THE_BIRTH" => Some(g__NOT_THE_BIRTH()),
            "_OTHER_SUBJECT" => Some(g__OTHER_SUBJECT()),
            "_PERIOD_ONLY" => Some(g__PERIOD_ONLY()),
            "_PERIOD_WITH_A_PREGNANCY" => Some(g__PERIOD_WITH_A_PREGNANCY()),
            "_POSTNATAL_BIRTH" => Some(g__POSTNATAL_BIRTH()),
            "_POSTNATAL_BLEED" => Some(g__POSTNATAL_BLEED()),
            "_PREG_DENIED" => Some(g__PREG_DENIED()),
            "_PREG_SAID" => Some(g__PREG_SAID()),
            "_PREG_SOFT" => Some(g__PREG_SOFT()),
            "_PREG_WORD" => Some(g__PREG_WORD()),
            "_SECOND_BIRTH" => Some(g__SECOND_BIRTH()),
            "_SHORT_ASK" => Some(g__SHORT_ASK()),
            "_THEY_HURT" => Some(g__THEY_HURT()),
            "_TIME_ASK" => Some(g__TIME_ASK()),
            "_WEEKS_NOW" => Some(g__WEEKS_NOW()),
            "_WHEN_TO_CALL" => Some(g__WHEN_TO_CALL()),
            "_a_birth_word" => Some(py_func("_a_birth_word", |args: &[Py]| _a_birth_word(args))),
            "_a_labour_contraction" => Some(py_func("_a_labour_contraction", |args: &[Py]| _a_labour_contraction(args))),
            "_a_late_that_is_not_a_period" => Some(py_func("_a_late_that_is_not_a_period", |args: &[Py]| _a_late_that_is_not_a_period(args))),
            "_asks_a_birth_time" => Some(py_func("_asks_a_birth_time", |args: &[Py]| _asks_a_birth_time(args))),
            "_asks_about_the_bag" => Some(py_func("_asks_about_the_bag", |args: &[Py]| _asks_about_the_bag(args))),
            "_asks_when_due" => Some(py_func("_asks_when_due", |args: &[Py]| _asks_when_due(args))),
            "_gives_a_month" => Some(py_func("_gives_a_month", |args: &[Py]| _gives_a_month(args))),
            "_preg_bag" => Some(py_func("_preg_bag", |args: &[Py]| _preg_bag(args))),
            "_preg_due_date" => Some(py_func("_preg_due_date", |args: &[Py]| _preg_due_date(args))),
            "_preg_food" => Some(py_func("_preg_food", |args: &[Py]| _preg_food(args))),
            "_preg_hair_dye" => Some(py_func("_preg_hair_dye", |args: &[Py]| _preg_hair_dye(args))),
            "_preg_labour_length" => Some(py_func("_preg_labour_length", |args: &[Py]| _preg_labour_length(args))),
            "_said_pregnant" => Some(py_func("_said_pregnant", |args: &[Py]| _said_pregnant(args))),
            "_the_date_in_this_year" => Some(py_func("_the_date_in_this_year", |args: &[Py]| _the_date_in_this_year(args))),
            "_the_date_named" => Some(py_func("_the_date_named", |args: &[Py]| _the_date_named(args))),
            "_the_day" => Some(py_func("_the_day", |args: &[Py]| _the_day(args))),
            "_the_line" => Some(py_func("_the_line", |args: &[Py]| _the_line(args))),
            "_the_weeks_named" => Some(py_func("_the_weeks_named", |args: &[Py]| _the_weeks_named(args))),
            "entry" => Some(g_entry()),
            "datetime" => Some(stdlib_module("datetime")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AM_I" => s_AM_I(v),
            "ANSWERS" => s_ANSWERS(v),
            "CARD" => s_CARD(v),
            "CYCLE_REPLY" => s_CYCLE_REPLY(v),
            "CYCLE_SHAPE" => s_CYCLE_SHAPE(v),
            "DRINKS_WITH_THEIR_OWN_ANSWER" => s_DRINKS_WITH_THEIR_OWN_ANSWER(v),
            "EARLY_REPLY" => s_EARLY_REPLY(v),
            "ENTRIES" => s_ENTRIES(v),
            "FERTILITY" => s_FERTILITY(v),
            "HEAD_PREGNANCY_REPLY" => s_HEAD_PREGNANCY_REPLY(v),
            "LATE_REPLY" => s_LATE_REPLY(v),
            "LIST" => s_LIST(v),
            "NEGATIVE_REPLY" => s_NEGATIVE_REPLY(v),
            "NOT_A_TEST" => s_NOT_A_TEST(v),
            "NOT_FOOD" => s_NOT_FOOD(v),
            "POSITIVE_REPLY" => s_POSITIVE_REPLY(v),
            "PREGNANT" => s_PREGNANT(v),
            "PREGNANT_ALREADY" => s_PREGNANT_ALREADY(v),
            "PREGNANT_MOVEMENT" => s_PREGNANT_MOVEMENT(v),
            "PREG_ANY_FOOD" => s_PREG_ANY_FOOD(v),
            "PREG_WORRY_FOODS" => s_PREG_WORRY_FOODS(v),
            "TEST_REPLY" => s_TEST_REPLY(v),
            "TEST_WORDS" => s_TEST_WORDS(v),
            "WHEN" => s_WHEN(v),
            "_AFTER_THE_BIRTH" => s__AFTER_THE_BIRTH(v),
            "_AMBIGUOUS" => s__AMBIGUOUS(v),
            "_ASKED" => s__ASKED(v),
            "_ASKS_WEEKS" => s__ASKS_WEEKS(v),
            "_A_BIRTH_LINE" => s__A_BIRTH_LINE(v),
            "_A_MILESTONE" => s__A_MILESTONE(v),
            "_A_MONTH" => s__A_MONTH(v),
            "_A_PREGNANCY" => s__A_PREGNANCY(v),
            "_BIRTH_ASK" => s__BIRTH_ASK(v),
            "_BIRTH_WORD" => s__BIRTH_WORD(v),
            "_BIRTH_WORDS" => s__BIRTH_WORDS(v),
            "_BIRTH_WORD_KEY" => s__BIRTH_WORD_KEY(v),
            "_BIRTH_WORD_NAMES" => s__BIRTH_WORD_NAMES(v),
            "_DAY_THEN_MONTH" => s__DAY_THEN_MONTH(v),
            "_DO_THEY_HURT" => s__DO_THEY_HURT(v),
            "_DUE_ELSEWHERE" => s__DUE_ELSEWHERE(v),
            "_DUE_SHAPE" => s__DUE_SHAPE(v),
            "_FEEDING_WORDS" => s__FEEDING_WORDS(v),
            "_GIVES_THE_DAY" => s__GIVES_THE_DAY(v),
            "_HOW_FAR_APART" => s__HOW_FAR_APART(v),
            "_MONTHS" => s__MONTHS(v),
            "_MONTH_THEN_DAY" => s__MONTH_THEN_DAY(v),
            "_MY_OWN" => s__MY_OWN(v),
            "_NOT_A_BIRTH_LINE" => s__NOT_A_BIRTH_LINE(v),
            "_NOT_THE_BIRTH" => s__NOT_THE_BIRTH(v),
            "_OTHER_SUBJECT" => s__OTHER_SUBJECT(v),
            "_PERIOD_ONLY" => s__PERIOD_ONLY(v),
            "_PERIOD_WITH_A_PREGNANCY" => s__PERIOD_WITH_A_PREGNANCY(v),
            "_POSTNATAL_BIRTH" => s__POSTNATAL_BIRTH(v),
            "_POSTNATAL_BLEED" => s__POSTNATAL_BLEED(v),
            "_PREG_DENIED" => s__PREG_DENIED(v),
            "_PREG_SAID" => s__PREG_SAID(v),
            "_PREG_SOFT" => s__PREG_SOFT(v),
            "_PREG_WORD" => s__PREG_WORD(v),
            "_SECOND_BIRTH" => s__SECOND_BIRTH(v),
            "_SHORT_ASK" => s__SHORT_ASK(v),
            "_THEY_HURT" => s__THEY_HURT(v),
            "_TIME_ASK" => s__TIME_ASK(v),
            "_WEEKS_NOW" => s__WEEKS_NOW(v),
            "_WHEN_TO_CALL" => s__WHEN_TO_CALL(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.pregnancy", module_obj);
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_a_late_that_is_not_a_period", py_func("_a_late_that_is_not_a_period", |args: &[Py]| _a_late_that_is_not_a_period(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_preg_food", py_func("_preg_food", |args: &[Py]| _preg_food(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_gives_a_month", py_func("_gives_a_month", |args: &[Py]| _gives_a_month(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_asks_when_due", py_func("_asks_when_due", |args: &[Py]| _asks_when_due(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_the_day", py_func("_the_day", |args: &[Py]| _the_day(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_the_date_named", py_func("_the_date_named", |args: &[Py]| _the_date_named(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_the_date_in_this_year", py_func("_the_date_in_this_year", |args: &[Py]| _the_date_in_this_year(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_the_weeks_named", py_func("_the_weeks_named", |args: &[Py]| _the_weeks_named(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_preg_due_date", py_func("_preg_due_date", |args: &[Py]| _preg_due_date(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_preg_hair_dye", py_func("_preg_hair_dye", |args: &[Py]| _preg_hair_dye(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_the_line", py_func("_the_line", |args: &[Py]| _the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_asks_a_birth_time", py_func("_asks_a_birth_time", |args: &[Py]| _asks_a_birth_time(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_asks_about_the_bag", py_func("_asks_about_the_bag", |args: &[Py]| _asks_about_the_bag(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_preg_labour_length", py_func("_preg_labour_length", |args: &[Py]| _preg_labour_length(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_preg_bag", py_func("_preg_bag", |args: &[Py]| _preg_bag(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_said_pregnant", py_func("_said_pregnant", |args: &[Py]| _said_pregnant(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_a_birth_word", py_func("_a_birth_word", |args: &[Py]| _a_birth_word(args)));
    pyrt::xrefs::reg_fn("knowledge.pregnancy", "_a_labour_contraction", py_func("_a_labour_contraction", |args: &[Py]| _a_labour_contraction(args)));
    pyrt::xrefs::reg_access("knowledge.pregnancy", "AM_I", g_AM_I, s_AM_I);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "CARD", g_CARD, s_CARD);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "CYCLE_REPLY", g_CYCLE_REPLY, s_CYCLE_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "CYCLE_SHAPE", g_CYCLE_SHAPE, s_CYCLE_SHAPE);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "DRINKS_WITH_THEIR_OWN_ANSWER", g_DRINKS_WITH_THEIR_OWN_ANSWER, s_DRINKS_WITH_THEIR_OWN_ANSWER);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "EARLY_REPLY", g_EARLY_REPLY, s_EARLY_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "FERTILITY", g_FERTILITY, s_FERTILITY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "HEAD_PREGNANCY_REPLY", g_HEAD_PREGNANCY_REPLY, s_HEAD_PREGNANCY_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "LATE_REPLY", g_LATE_REPLY, s_LATE_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "LIST", g_LIST, s_LIST);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "NEGATIVE_REPLY", g_NEGATIVE_REPLY, s_NEGATIVE_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "NOT_A_TEST", g_NOT_A_TEST, s_NOT_A_TEST);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "NOT_FOOD", g_NOT_FOOD, s_NOT_FOOD);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "POSITIVE_REPLY", g_POSITIVE_REPLY, s_POSITIVE_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "PREGNANT", g_PREGNANT, s_PREGNANT);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "PREGNANT_ALREADY", g_PREGNANT_ALREADY, s_PREGNANT_ALREADY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "PREGNANT_MOVEMENT", g_PREGNANT_MOVEMENT, s_PREGNANT_MOVEMENT);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "PREG_ANY_FOOD", g_PREG_ANY_FOOD, s_PREG_ANY_FOOD);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "PREG_WORRY_FOODS", g_PREG_WORRY_FOODS, s_PREG_WORRY_FOODS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "TEST_REPLY", g_TEST_REPLY, s_TEST_REPLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "TEST_WORDS", g_TEST_WORDS, s_TEST_WORDS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "WHEN", g_WHEN, s_WHEN);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_AFTER_THE_BIRTH", g__AFTER_THE_BIRTH, s__AFTER_THE_BIRTH);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_AMBIGUOUS", g__AMBIGUOUS, s__AMBIGUOUS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_ASKED", g__ASKED, s__ASKED);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_ASKS_WEEKS", g__ASKS_WEEKS, s__ASKS_WEEKS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_A_BIRTH_LINE", g__A_BIRTH_LINE, s__A_BIRTH_LINE);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_A_MILESTONE", g__A_MILESTONE, s__A_MILESTONE);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_A_MONTH", g__A_MONTH, s__A_MONTH);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_A_PREGNANCY", g__A_PREGNANCY, s__A_PREGNANCY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_BIRTH_ASK", g__BIRTH_ASK, s__BIRTH_ASK);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_BIRTH_WORD", g__BIRTH_WORD, s__BIRTH_WORD);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_BIRTH_WORDS", g__BIRTH_WORDS, s__BIRTH_WORDS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_BIRTH_WORD_KEY", g__BIRTH_WORD_KEY, s__BIRTH_WORD_KEY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_BIRTH_WORD_NAMES", g__BIRTH_WORD_NAMES, s__BIRTH_WORD_NAMES);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_DAY_THEN_MONTH", g__DAY_THEN_MONTH, s__DAY_THEN_MONTH);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_DO_THEY_HURT", g__DO_THEY_HURT, s__DO_THEY_HURT);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_DUE_ELSEWHERE", g__DUE_ELSEWHERE, s__DUE_ELSEWHERE);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_DUE_SHAPE", g__DUE_SHAPE, s__DUE_SHAPE);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_FEEDING_WORDS", g__FEEDING_WORDS, s__FEEDING_WORDS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_GIVES_THE_DAY", g__GIVES_THE_DAY, s__GIVES_THE_DAY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_HOW_FAR_APART", g__HOW_FAR_APART, s__HOW_FAR_APART);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_MONTHS", g__MONTHS, s__MONTHS);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_MONTH_THEN_DAY", g__MONTH_THEN_DAY, s__MONTH_THEN_DAY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_MY_OWN", g__MY_OWN, s__MY_OWN);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_NOT_A_BIRTH_LINE", g__NOT_A_BIRTH_LINE, s__NOT_A_BIRTH_LINE);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_NOT_THE_BIRTH", g__NOT_THE_BIRTH, s__NOT_THE_BIRTH);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_OTHER_SUBJECT", g__OTHER_SUBJECT, s__OTHER_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_PERIOD_ONLY", g__PERIOD_ONLY, s__PERIOD_ONLY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_PERIOD_WITH_A_PREGNANCY", g__PERIOD_WITH_A_PREGNANCY, s__PERIOD_WITH_A_PREGNANCY);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_POSTNATAL_BIRTH", g__POSTNATAL_BIRTH, s__POSTNATAL_BIRTH);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_POSTNATAL_BLEED", g__POSTNATAL_BLEED, s__POSTNATAL_BLEED);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_PREG_DENIED", g__PREG_DENIED, s__PREG_DENIED);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_PREG_SAID", g__PREG_SAID, s__PREG_SAID);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_PREG_SOFT", g__PREG_SOFT, s__PREG_SOFT);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_PREG_WORD", g__PREG_WORD, s__PREG_WORD);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_SECOND_BIRTH", g__SECOND_BIRTH, s__SECOND_BIRTH);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_SHORT_ASK", g__SHORT_ASK, s__SHORT_ASK);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_THEY_HURT", g__THEY_HURT, s__THEY_HURT);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_TIME_ASK", g__TIME_ASK, s__TIME_ASK);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_WEEKS_NOW", g__WEEKS_NOW, s__WEEKS_NOW);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "_WHEN_TO_CALL", g__WHEN_TO_CALL, s__WHEN_TO_CALL);
    pyrt::xrefs::reg_access("knowledge.pregnancy", "entry", g_entry, s_entry);
}

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
thread_local! { static G_BURNT_VESSEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURNT_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_RESULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EGG_COOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_IT_IS_KEEPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_BOTTLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_DAIRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_EGG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_MEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_RICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_OUT_SOFT_CHEESE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OUT_OF_THE_FRIDGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_BIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_GENERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_PRODUCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST_IT_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYS_BIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_OK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_TO_DO_WITH_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BIRD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COOLING_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FAULT_IN_THE_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_PERIOD_OF_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BREAD_SPOILT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHICKEN_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COLD_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRY_LOAF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FREEZER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GADGETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_TO_KEEP_IT_FRESH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_TO_MAKE_IT_SO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_THE_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVING_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVING_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEFTOVER_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEFT_OUT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_WANTING_TO_EAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUT_OF_PLACE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OUT_OF_THE_FREEZER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPENT_ON_THE_SIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TAKING_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_BURNT_VESSEL() -> Py { ensure_init(); G_BURNT_VESSEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURNT_VESSEL(v: Py) { ensure_init(); G_BURNT_VESSEL.with(|c| *c.borrow_mut() = v); }
pub fn g_BURNT_WORK() -> Py { ensure_init(); G_BURNT_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURNT_WORK(v: Py) { ensure_init(); G_BURNT_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_RESULT() -> Py { ensure_init(); G_COOKING_RESULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_RESULT(v: Py) { ensure_init(); G_COOKING_RESULT.with(|c| *c.borrow_mut() = v); }
pub fn g_EGG_COOKED() -> Py { ensure_init(); G_EGG_COOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EGG_COOKED(v: Py) { ensure_init(); G_EGG_COOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_IT_IS_KEEPING() -> Py { ensure_init(); G_HOW_IT_IS_KEEPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_IT_IS_KEEPING(v: Py) { ensure_init(); G_HOW_IT_IS_KEEPING.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_ASK() -> Py { ensure_init(); G_LEFT_OUT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_ASK(v: Py) { ensure_init(); G_LEFT_OUT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_BOTTLE() -> Py { ensure_init(); G_LEFT_OUT_BOTTLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_BOTTLE(v: Py) { ensure_init(); G_LEFT_OUT_BOTTLE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_DAIRY() -> Py { ensure_init(); G_LEFT_OUT_DAIRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_DAIRY(v: Py) { ensure_init(); G_LEFT_OUT_DAIRY.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_EGG() -> Py { ensure_init(); G_LEFT_OUT_EGG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_EGG(v: Py) { ensure_init(); G_LEFT_OUT_EGG.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_FOODS() -> Py { ensure_init(); G_LEFT_OUT_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_FOODS(v: Py) { ensure_init(); G_LEFT_OUT_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_KEEP() -> Py { ensure_init(); G_LEFT_OUT_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_KEEP(v: Py) { ensure_init(); G_LEFT_OUT_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_MEAT() -> Py { ensure_init(); G_LEFT_OUT_MEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_MEAT(v: Py) { ensure_init(); G_LEFT_OUT_MEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_PHRASE() -> Py { ensure_init(); G_LEFT_OUT_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_PHRASE(v: Py) { ensure_init(); G_LEFT_OUT_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_RICE() -> Py { ensure_init(); G_LEFT_OUT_RICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_RICE(v: Py) { ensure_init(); G_LEFT_OUT_RICE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_OUT_SOFT_CHEESE() -> Py { ensure_init(); G_LEFT_OUT_SOFT_CHEESE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_OUT_SOFT_CHEESE(v: Py) { ensure_init(); G_LEFT_OUT_SOFT_CHEESE.with(|c| *c.borrow_mut() = v); }
pub fn g_OUT_OF_THE_FRIDGE() -> Py { ensure_init(); G_OUT_OF_THE_FRIDGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OUT_OF_THE_FRIDGE(v: Py) { ensure_init(); G_OUT_OF_THE_FRIDGE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_ASK() -> Py { ensure_init(); G_PAST_IT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_ASK(v: Py) { ensure_init(); G_PAST_IT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_BIN() -> Py { ensure_init(); G_PAST_IT_BIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_BIN(v: Py) { ensure_init(); G_PAST_IT_BIN.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_GENERAL() -> Py { ensure_init(); G_PAST_IT_GENERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_GENERAL(v: Py) { ensure_init(); G_PAST_IT_GENERAL.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_KINDS() -> Py { ensure_init(); G_PAST_IT_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_KINDS(v: Py) { ensure_init(); G_PAST_IT_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_PRODUCE() -> Py { ensure_init(); G_PAST_IT_PRODUCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_PRODUCE(v: Py) { ensure_init(); G_PAST_IT_PRODUCE.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST_IT_STATE() -> Py { ensure_init(); G_PAST_IT_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST_IT_STATE(v: Py) { ensure_init(); G_PAST_IT_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYS_BIN() -> Py { ensure_init(); G_SAYS_BIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYS_BIN(v: Py) { ensure_init(); G_SAYS_BIN.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_OK_ASK() -> Py { ensure_init(); G_STILL_OK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_OK_ASK(v: Py) { ensure_init(); G_STILL_OK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_TO_DO_WITH_IT() -> Py { ensure_init(); G_WHAT_TO_DO_WITH_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_TO_DO_WITH_IT(v: Py) { ensure_init(); G_WHAT_TO_DO_WITH_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BIRD() -> Py { ensure_init(); G__A_BIRD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BIRD(v: Py) { ensure_init(); G__A_BIRD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COOLING_SUBJECT() -> Py { ensure_init(); G__A_COOLING_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COOLING_SUBJECT(v: Py) { ensure_init(); G__A_COOLING_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FAULT_IN_THE_FOOD() -> Py { ensure_init(); G__A_FAULT_IN_THE_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FAULT_IN_THE_FOOD(v: Py) { ensure_init(); G__A_FAULT_IN_THE_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_PERIOD_OF_TIME() -> Py { ensure_init(); G__A_PERIOD_OF_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_PERIOD_OF_TIME(v: Py) { ensure_init(); G__A_PERIOD_OF_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__BREAD_SPOILT() -> Py { ensure_init(); G__BREAD_SPOILT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BREAD_SPOILT(v: Py) { ensure_init(); G__BREAD_SPOILT.with(|c| *c.borrow_mut() = v); }
pub fn g__CHICKEN_OUT() -> Py { ensure_init(); G__CHICKEN_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHICKEN_OUT(v: Py) { ensure_init(); G__CHICKEN_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__COLD_MACHINE() -> Py { ensure_init(); G__COLD_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COLD_MACHINE(v: Py) { ensure_init(); G__COLD_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORDS() -> Py { ensure_init(); G__COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORDS(v: Py) { ensure_init(); G__COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DRY_LOAF() -> Py { ensure_init(); G__DRY_LOAF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRY_LOAF(v: Py) { ensure_init(); G__DRY_LOAF.with(|c| *c.borrow_mut() = v); }
pub fn g__FREEZER_WORD() -> Py { ensure_init(); G__FREEZER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FREEZER_WORD(v: Py) { ensure_init(); G__FREEZER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__GADGETS() -> Py { ensure_init(); G__GADGETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GADGETS(v: Py) { ensure_init(); G__GADGETS.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_TO_KEEP_IT_FRESH() -> Py { ensure_init(); G__HOW_TO_KEEP_IT_FRESH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_TO_KEEP_IT_FRESH(v: Py) { ensure_init(); G__HOW_TO_KEEP_IT_FRESH.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_TO_MAKE_IT_SO() -> Py { ensure_init(); G__HOW_TO_MAKE_IT_SO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_TO_MAKE_IT_SO(v: Py) { ensure_init(); G__HOW_TO_MAKE_IT_SO.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_THE_DISH() -> Py { ensure_init(); G__IN_THE_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_THE_DISH(v: Py) { ensure_init(); G__IN_THE_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVING_OUT() -> Py { ensure_init(); G__LEAVING_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVING_OUT(v: Py) { ensure_init(); G__LEAVING_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVING_VERB() -> Py { ensure_init(); G__LEAVING_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVING_VERB(v: Py) { ensure_init(); G__LEAVING_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g__LEFTOVER_WORD() -> Py { ensure_init(); G__LEFTOVER_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEFTOVER_WORD(v: Py) { ensure_init(); G__LEFTOVER_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__LEFT_OUT_WORD() -> Py { ensure_init(); G__LEFT_OUT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEFT_OUT_WORD(v: Py) { ensure_init(); G__LEFT_OUT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_WANTING_TO_EAT() -> Py { ensure_init(); G__NOT_WANTING_TO_EAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_WANTING_TO_EAT(v: Py) { ensure_init(); G__NOT_WANTING_TO_EAT.with(|c| *c.borrow_mut() = v); }
pub fn g__OUT_OF_PLACE_WORD() -> Py { ensure_init(); G__OUT_OF_PLACE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUT_OF_PLACE_WORD(v: Py) { ensure_init(); G__OUT_OF_PLACE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__OUT_OF_THE_FREEZER() -> Py { ensure_init(); G__OUT_OF_THE_FREEZER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OUT_OF_THE_FREEZER(v: Py) { ensure_init(); G__OUT_OF_THE_FREEZER.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_OUT() -> Py { ensure_init(); G__SAID_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_OUT(v: Py) { ensure_init(); G__SAID_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__SPAN() -> Py { ensure_init(); G__SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPAN(v: Py) { ensure_init(); G__SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g__SPENT_ON_THE_SIDE() -> Py { ensure_init(); G__SPENT_ON_THE_SIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPENT_ON_THE_SIDE(v: Py) { ensure_init(); G__SPENT_ON_THE_SIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_WORD() -> Py { ensure_init(); G__STATE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_WORD(v: Py) { ensure_init(); G__STATE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__TAKING_OUT() -> Py { ensure_init(); G__TAKING_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TAKING_OUT(v: Py) { ensure_init(); G__TAKING_OUT.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Freshness: has that food been out too long to eat.\n\n\"How long can cooked rice sit out\", \"is it ok to eat food left out overnight\" and \"i left the\npizza out overnight\" are one question, and the answer depends on the food, so this is one table\nkeyed on the food the line names: rice is the strictest, cooked meat, fish, gravy and anything\nwith egg or mayonnaise next, milk and cream after that, and bread, cake, butter, hard cheese, jam\nand fruit come to no harm because they were never a food-poisoning risk.\n\nIt lives here rather than in cooking.py because that file is past the size the world accepts for\nan upload, so no change to it can be submitted. Ripeness (\"is this fruit ripe\", \"how do i ripen a\nmango\") is a different question and belongs with shelf life in keeping.py.\n");
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_LEFT_OUT_KEEP(py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("cake"), py_str("butter"), py_str("cheese"), py_str("cheddar"), py_str("parmesan"), py_str("jam"), py_str("marmalade"), py_str("pickle"), py_str("chutney"), py_str("fruit"), py_str("banana"), py_str("apple"), py_str("orange"), py_str("biscuit"), py_str("crisps"), py_str("chocolate"), py_str("honey"), py_str("cereal"), py_str("flour"), py_str("sugar"), py_str("nuts"), py_str("olives"), py_str("capers")]));
    s_LEFT_OUT_SOFT_CHEESE(py_tuple(vec![py_str("soft cheese"), py_str("brie"), py_str("camembert"), py_str("mozzarella"), py_str("feta"), py_str("cream cheese"), py_str("blue cheese"), py_str("stilton"), py_str("gorgonzola"), py_str("ricotta"), py_str("cottage cheese"), py_str("mascarpone"), py_str("goats cheese"), py_str("goat cheese"), py_str("halloumi"), py_str("paneer")]));
    s_LEFT_OUT_DAIRY(py_tuple(vec![py_str("milk"), py_str("cream"), py_str("yoghurt"), py_str("yogurt"), py_str("custard"), py_str("ice cream"), py_str("ice-cream")]));
    s_LEFT_OUT_BOTTLE(py_tuple(vec![py_str("formula"), py_str("breast milk"), py_str("baby milk"), py_str("expressed milk"), py_str("bottle of milk"), py_str("baby bottle"), py_str("baby's bottle"), py_str("bottles of milk")]));
    s_LEFT_OUT_RICE(py_tuple(vec![py_str("rice"), py_str("risotto")]));
    s_LEFT_OUT_MEAT(py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("ham"), py_str("bacon"), py_str("sausage"), py_str("meat"), py_str("steak"), py_str("steaks"), py_str("chops"), py_str("mince"), py_str("burger"), py_str("burgers"), py_str("meatballs"), py_str("gammon"), py_str("venison"), py_str("duck"), py_str("sausages"), py_str("fish"), py_str("salmon"), py_str("tuna"), py_str("stew"), py_str("curry"), py_str("soup"), py_str("gravy"), py_str("pasta"), py_str("spaghetti"), py_str("quiche"), py_str("mayonnaise"), py_str("mayo"), py_str("potato salad"), py_str("pie"), py_str("pizza"), py_str("mince"), py_str("bolognese"), py_str("chilli"), py_str("lasagne"), py_str("shepherds"), py_str("peas"), py_str("beans"), py_str("sweetcorn"), py_str("couscous"), py_str("stuffing"), py_str("potatoes"), py_str("mashed potato"), py_str("sandwich"), py_str("sandwiches"), py_str("wrap"), py_str("wraps"), py_str("coleslaw"), py_str("sushi"), py_str("prawn"), py_str("prawns"), py_str("tofu"), py_str("sauce"), py_str("sauces")]));
    s_LEFT_OUT_EGG(py_tuple(vec![py_str("egg"), py_str("eggs")]));
    s_EGG_COOKED(py_tuple(vec![py_str("cooked"), py_str("boiled"), py_str("scrambled"), py_str("fried"), py_str("poached"), py_str("omelette"), py_str("frittata"), py_str("hard boiled"), py_str("soft boiled"), py_str("quiche"), py_str("mayonnaise"), py_str("custard"), py_str("raw egg"), py_str("cracked"), py_str("broken egg"), py_str("shell"), py_str("smashed"), py_str("batter")]));
    s_LEFT_OUT_ASK(py_tuple(vec![py_str("left out"), py_str("left it out"), py_str("left them out"), py_str("left over out"), py_str("out overnight"), py_str("overnight"), py_str("all night"), py_str("sat out"), py_str("sit out"), py_str("sitting out"), py_str("stood out"), py_str("stay out"), py_str("stays out"), py_str("staying out"), py_str("out on the counter"), py_str("on the counter"), py_str("on the side"), py_str("on the hob"), py_str("out of the fridge"), py_str("forgot to put"), py_str("forgot it"), py_str("didnt put it"), py_str("did not put it"), py_str("room temperature"), py_str("went cold"), py_str("still out"), py_str("was left out"), py_str("that was left out"), py_str("left out overnight"), py_str("sat out overnight"), py_str("was sat out"), py_str("left"), py_str("out")]));
    s_STILL_OK_ASK(py_tuple(vec![py_str("still cook it"), py_str("still eat it"), py_str("can i cook it"), py_str("can i eat it"), py_str("can i still"), py_str("is it still ok"), py_str("still ok to eat"), py_str("still safe"), py_str("safe to eat"), py_str("still use it"), py_str("should i bin it"), py_str("should i throw it"), py_str("should i chuck it"), py_str("still good"), py_str("is it still safe"), py_str("can we still eat"), py_str("will it still be ok")]));
    s_LEFT_OUT_PHRASE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LEFT_OUT_ASK()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("left"), py_str("out")]), &phrase.clone()))) {  __out.push(phrase.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_OUT_OF_THE_FRIDGE(py_tuple(vec![py_str("left out"), py_str("left it out"), py_str("left them out"), py_str("left this out"), py_str("sat out"), py_str("sit out"), py_str("sitting out"), py_str("stood out"), py_str("standing out"), py_str("still out"), py_str("been out"), py_str("out on the"), py_str("out all night"), py_str("out overnight"), py_str("out since"), py_str("on the side"), py_str("on the counter"), py_str("on the worktop"), py_str("on the hob"), py_str("on the table"), py_str("on top of the"), py_str("overnight on the")]));
    s__FREEZER_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bfreez(?:e|es|er|ers|ing|ed)?\\b|\\bfrozen\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OUT_OF_THE_FREEZER({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bout\\s+(?:of\\s+)?(?:the\\s+|my\\s+|a\\s+|your\\s+|its\\s+|their\\s+)?freez(?:er|ers)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEAVING_VERB(py_str("leave|leaves|leaving|left|forget|forgets|forgot|forgotten"));
    s__LEAVING_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&py_str("(?:\\b(?:"), &g__LEAVING_VERB()), &py_str("|sit|sits|sitting|sat|stand|stands|standing|stood|stay|stays|stayed|keep|keeps|kept|been|being|is|are|was|were|still)\\b(?:\\W+\\w+){0,6}?\\W+out\\b(?!\\s+of\\b)|\\bout\\b(?:\\W+\\w+){0,6}?\\W+(?:")), &g__LEAVING_VERB()), &py_str(")\\b|\\bout\\s+of\\s+(?:the\\s+|my\\s+|a\\s+|your\\s+|its\\s+)?(?:freez(?:er|ers)|fridge)\\b)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_LEFT_OUT_FOODS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&op_bitor(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_MEAT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_RICE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_DAIRY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_KEEP()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_SOFT_CHEESE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_EGG()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_LEFT_OUT_BOTTLE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_set(vec![py_str("food"), py_str("leftovers"), py_str("leftover"), py_str("takeaway"), py_str("sauce"), py_str("sandwich"), py_str("wrap"), py_str("curry"), py_str("soup"), py_str("stew")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COUNT_WORDS(py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("half"), Py::Float(0.5)), (py_str("couple"), Py::Int(2)), (py_str("couple of"), Py::Int(2)), (py_str("few"), Py::Int(3)), (py_str("a few"), Py::Int(3)), (py_str("several"), Py::Int(3)), (py_str("twelve"), Py::Int(12))]));
    s__SPAN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(\\d+(?:\\.\\d+)?|a few|couple of|half|a|an|one|two|three|four|five|six|several|twelve|few|couple)\\s*(hours?|hrs?|minutes?|mins?)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__CHICKEN_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bchicken(?:s|ed|ing)?\\s+out\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEFTOVER_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:left ?over|leftovers?|spare)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__LEFT_OUT_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:left|leave|leaves|leaving|forgot|forget|forgotten|sat|sit|sits|sitting|stood|stand|standing|still|overnight|nite|tonight|night|hours?|counter|side|worktop|hob|fridge|freezer|frozen|defrost|out of)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__OUT_OF_PLACE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:out|overnight|nite|tonight|night|hours?|counter|side|worktop|hob|fridge|freezer|frozen|defrost|room temperature|sat|sit|sits|sitting|stood|standing|still)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_COOKING_RESULT(py_tuple(vec![py_str("came out"), py_str("come out"), py_str("comes out"), py_str("came up"), py_str("turned out"), py_str("turn out"), py_str("turns out"), py_str("boiled over"), py_str("boils over"), py_str("boiling over"), py_str("boil over"), py_str("boiled dry"), py_str("boils dry"), py_str("boiling dry"), py_str("caught on the bottom"), py_str("catches on the bottom"), py_str("stuck to the pan"), py_str("sticks to the pan"), py_str("stuck to the bottom"), py_str("went wrong"), py_str("goes wrong"), py_str("gone wrong")]));
    s__SAID_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:left|leave|leaves|leaving|sat|sitting|stood|standing|kept|been)\\b[^.]{0,20}?\\bout\\b|\\bout\\s+(?:overnight|all\\s+night|all\\s+day|since|for)\\b|\\b(?:overnight|all\\s+night|all\\s+day|this\\s+morning|last\\s+night|yesterday)\\b|\\bfor\\s+(?:\\d+|a\\s+few|a\\s+couple\\s+of|a|an)\\s*(?:minutes?|mins?|hours?|hrs?)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TAKING_OUT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:take|takes|taking|took|taken|get|gets|getting|got)\\b[^.;]{0,24}\\bout\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SPENT_ON_THE_SIDE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:overnight|nites?|nights?|tonight|hours?|counter|worktop|hob|side|room temperature|since|left it|leaving it)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_BIRD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:chicken|chickens|poultry)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_BURNT_WORK(py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("burning"), py_str("burn"), py_str("scorched"), py_str("blackened"), py_str("charred")]));
    s_BURNT_VESSEL(py_tuple(vec![py_str("pan"), py_str("pans"), py_str("pot"), py_str("pots"), py_str("tin"), py_str("tins"), py_str("tray"), py_str("trays"), py_str("shelf"), py_str("shelves"), py_str("oven"), py_str("hob"), py_str("wok"), py_str("saucepan"), py_str("saucepans"), py_str("skillet"), py_str("griddle"), py_str("baking tray"), py_str("cake tin"), py_str("roasting tin"), py_str("baking tin"), py_str("baking sheet"), py_str("casserole")]));
    s_SAYS_BIN(py_tuple(vec![py_str("bin it"), py_str("bin them"), py_str("into the bin"), py_str("in the bin"), py_str("throw it away"), py_str("throw it out"), py_str("throw them away"), py_str("chuck it"), py_str("dispose of it")]));
    s_HOW_IT_IS_KEEPING(py_tuple(vec![py_str("going off"), py_str("gone off"), py_str("smells off"), py_str("smell off"), py_str("smells bad"), py_str("smells funny"), py_str("is it ok"), py_str("still ok"), py_str("still good"), py_str("is it safe"), py_str("is it off"), py_str("gone bad"), py_str("has gone bad"), py_str("use by"), py_str("out of date"), py_str("past its date"), py_str("off yet"), py_str("gone slimy"), py_str("is it still")]));
    s_WHAT_TO_DO_WITH_IT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:what|what can i|what can we|what should i|what shall i|what do i|what will i)\\b[^.?!]*\\b(?:do|make|cook|use|eat)\\b[^.?!]*\\b(?:with (?:it|them|that|this))\\b|\\b(?:can|could|should|shall|will|is it ok to|is it safe to|is it fine to)\\b[^.?!]*\\b(?:still )?(?:cook|use|eat|keep|freeze|save)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FAULT_IN_THE_FOOD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:too (?:spicy|hot to eat|hot for|salty|salt\\w*|sweet|sour|bitter|thick|thin|dry|bland)|way too (?:spicy|hot|salty)|far too (?:spicy|hot|salty)|cool it down|make it milder|less spicy|too much (?:chilli|chili|salt|spice))\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_COOLING_SUBJECT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("fridge"), py_str("refrigerator"), py_str("cool"), py_str("cooling"), py_str("cooled"), py_str("warm"), py_str("warmish"), py_str("hot"), py_str("lukewarm"), py_str("steaming"), py_str("leftover"), py_str("leftovers"), py_str("food"), py_str("stew"), py_str("curry"), py_str("soup"), py_str("rice"), py_str("pasta"), py_str("chilli"), py_str("chili"), py_str("casserole"), py_str("chicken"), py_str("meat"), py_str("pot"), py_str("pots"), py_str("pan"), py_str("saucepan"), py_str("milk"), py_str("bottle"), py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("batch"), py_str("bolognese"), py_str("sauce"), py_str("stock"), py_str("tub"), py_str("tupperware"), py_str("containers"), py_str("container"), py_str("boxes"), py_str("box")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__GADGETS(py_tuple(vec![py_str("phone"), py_str("phones"), py_str("mobile"), py_str("iphone"), py_str("android"), py_str("smartphone"), py_str("camera"), py_str("watch"), py_str("tablet"), py_str("ipad"), py_str("laptop"), py_str("airpods"), py_str("headphones"), py_str("earbuds"), py_str("keyboard"), py_str("remote"), py_str("sim card"), py_str("memory card"), py_str("screen"), py_str("battery"), py_str("charger"), py_str("charging port")]));
    s_PAST_IT_PRODUCE(py_tuple(vec![py_str("banana"), py_str("bananas"), py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("grape"), py_str("grapes"), py_str("berries"), py_str("strawberries"), py_str("raspberries"), py_str("blueberries"), py_str("peach"), py_str("peaches"), py_str("plum"), py_str("plums"), py_str("mango"), py_str("avocado"), py_str("avocados"), py_str("tomato"), py_str("tomatoes"), py_str("spinach"), py_str("lettuce"), py_str("salad"), py_str("salad leaves"), py_str("rocket"), py_str("watercress"), py_str("kale"), py_str("cabbage"), py_str("leafy"), py_str("greens"), py_str("carrot"), py_str("carrots"), py_str("cucumber"), py_str("pepper"), py_str("peppers"), py_str("courgette"), py_str("courgettes"), py_str("broccoli"), py_str("cauliflower"), py_str("onion"), py_str("onions"), py_str("fruit"), py_str("vegs"), py_str("vegetables"), py_str("veg"), py_str("produce")]));
    s_PAST_IT_STATE(py_tuple(vec![py_str("gone soft"), py_str("went soft"), py_str("gone a bit soft"), py_str("a bit soft"), py_str("soft"), py_str("softened"), py_str("wrinkly"), py_str("wrinkled"), py_str("shrivelled"), py_str("shrunken"), py_str("limp"), py_str("wilting"), py_str("wilted"), py_str("floppy"), py_str("rubbery"), py_str("dry"), py_str("dryish"), py_str("stale"), py_str("gone a bit"), py_str("old"), py_str("slimy"), py_str("slimey"), py_str("sticky"), py_str("smelling"), py_str("smells"), py_str("smelly"), py_str("sour"), py_str("leaking"), py_str("mouldy"), py_str("mould"), py_str("black spots"), py_str("spotty")]));
    s_PAST_IT_ASK(py_tuple(vec![py_str("can i eat"), py_str("can i still"), py_str("is it ok"), py_str("is it still"), py_str("still ok"), py_str("still good"), py_str("still safe"), py_str("safe to eat"), py_str("eat it"), py_str("eat them"), py_str("use it"), py_str("use them"), py_str("should i bin"), py_str("throw it"), py_str("throw them"), py_str("chuck it"), py_str("ruined"), py_str("bin it"), py_str("what can i do with"), py_str("why is my"), py_str("why has my"), py_str("why are my"), py_str("why have my"), py_str("are they ok"), py_str("is it fine"), py_str("ok to eat"), py_str("gone a bit"), py_str("old"), py_str("been there a while"), py_str("has gone"), py_str("have gone"), py_str("went soft"), py_str("gone soft"), py_str("smells"), py_str("smelly"), py_str("slimy")]));
    s_PAST_IT_BIN(py_tuple(vec![py_str("slimy"), py_str("slimey"), py_str("sticky"), py_str("sour"), py_str("smelly"), py_str("smells"), py_str("stinks"), py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("furry"), py_str("fuzz"), py_str("leaking"), py_str("gone off"), py_str("went off"), py_str("rotten")]));
    s_PAST_IT_KINDS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("spinach"), py_str("lettuce"), py_str("salad"), py_str("salad leaves"), py_str("rocket"), py_str("watercress"), py_str("kale"), py_str("cabbage"), py_str("leafy"), py_str("greens")]), py_str("Leaves that have gone a bit yellow or brown are older, not unsafe: the colour is the leaf giving up its green, and it does not make anybody ill. Pick out any that are slimy or smelly - those are the ones to bin - and eat or cook the rest, because a yellowed leaf is still good wilted in a pan or stirred into soup.")]), py_tuple(vec![py_tuple(vec![py_str("banana"), py_str("bananas")]), py_str("A soft banana is a ripe one: the starch has turned to sugar, so it is sweeter and easier to mash - eat it, or put it in a smoothie or banana bread. A banana is the fruit that keeps best in the fruit bowl, never the fridge.")]), py_tuple(vec![py_tuple(vec![py_str("grape"), py_str("grapes")]), py_str("A wrinkly grape has lost water and is sweeter for it, so eat them - and pick out any that are leaking or furred, because one going over spoils the rest in the punnet.")]), py_tuple(vec![py_tuple(vec![py_str("berries"), py_str("strawberries"), py_str("raspberries"), py_str("blueberries"), py_str("blackberries")]), py_str("A soft berry has only lost water, so it is still fine to eat - pick out any that are leaking or furred, because one going over spoils the rest in the punnet, and eat them sooner rather than later or cook them down with a little sugar.")]), py_tuple(vec![py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("peach"), py_str("peaches"), py_str("plum"), py_str("plums"), py_str("mango")]), py_str("A soft or shrivelled apple or pear has lost water: it is fine to eat, and it is at its best stewed, baked or in a crumble.")]), py_tuple(vec![py_tuple(vec![py_str("carrot"), py_str("carrots")]), py_str("A floppy carrot has lost water, not gone off: peel it and it is fine to eat, and standing it in a glass of cold water for an hour firms it up again.")]), py_tuple(vec![py_tuple(vec![py_str("cucumber"), py_str("courgette"), py_str("courgettes"), py_str("pepper"), py_str("peppers"), py_str("broccoli"), py_str("cauliflower")]), py_str("A soft cucumber, pepper or courgette has lost water and is fine, just not crisp: cook it rather than eat it raw, and cut away any soft spots.")]), py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes")]), py_str("A soft tomato is fine and sweeter: cook it into a sauce rather than slice it raw, and use it before the ones around it go too.")])]));
    s_PAST_IT_GENERAL(py_str("It has lost water rather than gone off, so it is still fine to eat and better cooked than raw."));
    s__STATE_WORD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PAST_IT_STATE()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("|"), "join", args))(&__args) }), &py_str(")\\b"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__NOT_WANTING_TO_EAT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:wont eat|won't eat|will not eat|wouldn't eat|would not eat|doesn't eat|does not eat|refuses|refusing|refused|fussy|picky|won't touch|wont touch|will not touch|year old|toddler|my son|my daughter|my child|my kid|my kids|children|kids|nursery|school dinner|fed up of|sick of eating|only eats|ate nothing|eats nothing|barely eats)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HOW_TO_MAKE_IT_SO({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:how do i|how do you|how can i|how to|how would i|how should i|whats the best way to|what's the best way to|best way to|quickest way to|easiest way to|any way to|way of)(?:\\W+\\w+){0,2}?\\W+(?:dry|drying|dry them|dry it|dehydrate|pickle|pickling|freeze|freezing|ferment|fermenting|cure|curing|smoke|smoking|bottle|bottling|can|canning|preserve|preserving|keep|keeping|store|storing|revive|reviving|crisp up|crisp it up)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_PERIOD_OF_TIME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a|an|another|few|several|couple(?=\\s+of))\\s+(?:day|days|week|weeks|fortnight|fortnights|month|months|night|nights)\\b|\\bsince\\s+(?:yesterday|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\\b|\\b(?:been|sat|sitting|stood|stored|kept|left|made|cooked|bought)\\b(?:\\W+\\w+){0,4}?\\W+(?:in|inside|out)\\s+of\\s*the\\s+(?:fridge|refrigerator)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COLD_MACHINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|the|our|your|his|her|their|this|that|a|an)\\s+(?:fridge freezer|fridge|freezer)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__IN_THE_DISH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:pot|pots|pan|pans|saucepan|saucepans|wok|bowl|bowls|tub|tubs|tray|trays|stew|stews|casserole|casseroles|curry|curries|soup|soups|rice|pasta|chilli|chili|stock|leftover|leftovers|dish|dishes|pie|pies|roast|chicken|meat|joint|batch|bolognese|sauce|beans|lentils|porridge|pudding|cake|cakes|bread|bottle|bottles|milk|formula|yoghurt|yogurt|cream|muffin|muffins|loaf|loaves)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__DRY_LOAF({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:hard|hardened|dry|dried|stale|stalish|tough|crusty|brittle)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__BREAD_SPOILT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:mould|mouldy|mold|moldy|green|blue|furry|fuzz|fluff|spores|spots|speckled|specked|smell|smells|smelly|smelt|slimy|damp)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__HOW_TO_KEEP_IT_FRESH({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:how do i stop|how to stop|how do i keep|how to keep|how can i keep|how can you keep|stop it going|stop them going|prevent|keep it fresh|keep them fresh|keep bread|keep a loaf|make it last|make bread last|last longer|stay fresh|keep fresh|fresh for longer)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cool-before-fridge")); __args.push(py_tuple(vec![py_tuple(vec![py_str("warm"), py_str("warmish"), py_str("hot"), py_str("still warm"), py_str("lukewarm"), py_str("cool"), py_str("cooled"), py_str("cooling"), py_str("room temperature"), py_str("steaming"), py_str("left to cool"), py_str("leftover"), py_str("leftovers"), py_str("stew"), py_str("curry"), py_str("soup"), py_str("rice"), py_str("chilli"), py_str("casserole"), py_str("pasta"), py_str("meat"), py_str("chicken"), py_str("food")]), py_tuple(vec![py_str("fridge"), py_str("refrigerator"), py_str("put it in"), py_str("put them in"), py_str("put in the fridge"), py_str("put it in the fridge"), py_str("in the fridge"), py_str("store"), py_str("keep it"), py_str("put away"), py_str("how do i cool"), py_str("how to cool"), py_str("cool it quickly"), py_str("cool them quickly"), py_str("cool down quickly"), py_str("quickest way to cool"), py_str("fastest way to cool"), py_str("cool quickly"), py_str("cool it down")])])); __args.push(g__cool_before_fridge()); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| a_cooling_question_not_a_keeping_one(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_cold_machine_and_not_the_dish(args))(&__args) } } } })), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("defrost"), py_str("refreeze"), py_str("reheat"), py_str("oven"), py_str("microwave"), py_str("how long"), py_str("how many"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("how do i store"), py_str("how to store"), py_str("how should i store"), py_str("storing"), py_str("storage"), py_str("keep for"), py_str("keeps for"), py_str("how long do leftovers keep"), py_str("what can i cook"), py_str("what can i make"), py_str("what shall i cook"), py_str("what shall i make"), py_str("what to cook"), py_str("what to make"), py_str("what can i do with"), py_str("meal ideas"), py_str("dinner ideas"), py_str("claim"), py_str("claims"), py_str("refund"), py_str("money back"), py_str("compensat"), py_str("who pays"), py_str("insurance"), py_str("ruined"), py_str("wasted"), py_str("spoiled"), py_str("spoilt"), py_str("go warm"), py_str("goes warm"), py_str("getting warm"), py_str("gets warm"), py_str("fridge is warm"), py_str("fridge is cold"), py_str("fridge feels warm"), py_str("fridge feels cold"), py_str("partially warm"), py_str("slightly warm"), py_str("half warm"), py_str("barely cold"), py_str("fridge too warm"), py_str("fridge is not cold"), py_str("fridge isnt cold"), py_str("stopped cooling"), py_str("not cooling"), py_str("stopped getting cold"), py_str("not getting cold"), py_str("temperature"), py_str("thermometer"), py_str("fridge smells"), py_str("warm in the fridge"), py_str("not cold"), py_str("isn't cold"), py_str("isnt cold"), py_str("stopped working"), py_str("power cut"), py_str("power went off"), py_str("thermometer"), py_str("drinks"), py_str("beer"), py_str("wine"), py_str("when i take it out"), py_str("out of the fridge"), py_str("day"), py_str("days"), py_str("tomorrow"), py_str("next day"), py_str("how long"), py_str("keeps"), py_str("keeping"), py_str("will it keep"), py_str("does it keep"), py_str("raw"), py_str("phone"), py_str("phones"), py_str("mobile"), py_str("iphone"), py_str("camera"), py_str("watch"), py_str("tablet"), py_str("laptop"), py_str("airpods"), py_str("headphones"), py_str("keyboard"), py_str("remote"), py_str("sim"), py_str("battery"), py_str("charger"), py_str("wont turn on"), py_str("wont turn off"), py_str("wont charge"), py_str("wont switch on"), py_str("wet"), py_str("all night"), py_str("overnight"), py_str("still ok"), py_str("still okay"), py_str("still safe"), py_str("still good"), py_str("still fine"), py_str("safe to eat"), py_str("can i eat it"), py_str("can i eat that"), py_str("can i eat them"), py_str("is it ok to eat"), py_str("do i have to bin"), py_str("throw it away"), py_str("too warm"), py_str("warm fridge"), py_str("fridge is too"), py_str("fridge so warm"), py_str("flat"), py_str("room"), py_str("rooms"), py_str("house"), py_str("bedroom"), py_str("the heat"), py_str("sun"), py_str("sunburn"), py_str("burn"), py_str("burnt"), py_str("scald"), py_str("fever"), py_str("temperature of"), py_str("baby"), py_str("babies"), py_str("child"), py_str("toddler"), py_str("after a run"), py_str("after exercise"), py_str("after a workout"), py_str("gym"), py_str("run"), py_str("body"), py_str("drink"), py_str("tea"), py_str("coffee"), py_str("wash"), py_str("washing up"), py_str("clean the pot")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-left-out")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("pizza"), py_str("milk"), py_str("cream"), py_str("yoghurt"), py_str("yogurt"), py_str("soup"), py_str("stew"), py_str("turkey"), py_str("chicken"), py_str("pasta"), py_str("potato salad"), py_str("mayonnaise"), py_str("mayo"), py_str("butter"), py_str("cheese"), py_str("bread"), py_str("cake"), py_str("curry"), py_str("gravy"), py_str("sausages"), py_str("eggs"), py_str("quiche"), py_str("fish"), py_str("ham"), py_str("bacon"), py_str("food"), py_str("leftovers"), py_str("leftover"), py_str("risotto"), py_str("pie"), py_str("chilli"), py_str("lasagne"), py_str("meat"), py_str("steak"), py_str("steaks"), py_str("chops"), py_str("mince"), py_str("burger"), py_str("burgers"), py_str("meatballs"), py_str("gammon"), py_str("duck"), py_str("joint"), py_str("crisps"), py_str("biscuits"), py_str("chocolate"), py_str("banana"), py_str("apples"), py_str("orange"), py_str("honey"), py_str("cereal"), py_str("peas"), py_str("beans"), py_str("sweetcorn"), py_str("potatoes"), py_str("flour"), py_str("jam"), py_str("marmalade"), py_str("chutney"), py_str("loaf"), py_str("fruit"), py_str("brie"), py_str("camembert"), py_str("mozzarella"), py_str("feta"), py_str("stilton"), py_str("cheddar"), py_str("parmesan"), py_str("halloumi"), py_str("paneer"), py_str("takeaway"), py_str("takeaways"), py_str("take out food"), py_str("chinese"), py_str("kebab"), py_str("curry house"), py_str("mascarpone"), py_str("ricotta"), py_str("sandwich"), py_str("sandwiches"), py_str("wrap"), py_str("wraps"), py_str("coleslaw"), py_str("sushi"), py_str("prawn"), py_str("prawns"), py_str("tofu"), py_str("sauce"), py_str("formula"), py_str("baby bottle"), py_str("bottle of milk")]), g_LEFT_OUT_ASK()])); __args.push(g__left_out()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("goes with"), py_str("go with"), py_str("going with"), py_str("goes on the side"), py_str("goes nicely"), py_str("what side"), py_str("which side"), py_str("side dish"), py_str("side dishes"), py_str("sides with"), py_str("serve with"), py_str("serving with"), py_str("to serve with"), py_str("to go with"), py_str("alongside"), py_str("reheat"), py_str("reheating"), py_str("warm it up"), py_str("warm up"), py_str("in the fridge"), py_str("how many days"), py_str("keep for"), py_str("recipe"), py_str("how do i"), py_str("how to make"), py_str("make a"), py_str("order"), py_str("delivery"), py_str("dough"), py_str("toppings"), py_str("defrost"), py_str("come to room temperature"), py_str("come up to room temperature"), py_str("to room temperature before"), py_str("room temperature before"), py_str("let it come to"), py_str("bring it to room temperature"), py_str("get it to room temperature"), py_str("comes out"), py_str("come out"), py_str("came out"), py_str("coming out"), py_str("turned out"), py_str("turns out"), py_str("turning out"), py_str("works out"), py_str("worked out"), py_str("figure out"), py_str("find out"), py_str("check out"), py_str("point out"), py_str("sort out"), py_str("spits it out"), py_str("spitting it out"), py_str("spits the food out"), py_str("spitting the food out"), py_str("spits food out"), py_str("spitting food out"), py_str("spits his food out"), py_str("spits her food out"), py_str("spat the food out"), py_str("spat it out"), py_str("night out"), py_str("meal out"), py_str("out for drinks"), py_str("out for a drink"), py_str("took me out"), py_str("taking me out"), py_str("took us out"), py_str("eat out"), py_str("eating out"), py_str("take out the"), py_str("ordered from"), py_str("tooth"), py_str("teeth"), py_str("milk tooth"), py_str("fell out"), py_str("wobbly"), py_str("out of milk"), py_str("out of bread"), py_str("out of butter"), py_str("out of eggs"), py_str("out of cheese"), py_str("run out"), py_str("ran out"), py_str("running out"), py_str("we are out"), py_str("weve run"), py_str("run out of"), py_str("shopping list"), py_str("need to buy"), py_str("need to get"), py_str("forgot to buy"), py_str("at home"), py_str("at work"), py_str("in the car"), py_str("on the train"), py_str("in the office"), py_str("in the oven"), py_str("in the pan"), py_str("on the hob cooking"), py_str("my lunch at"), py_str("left my job"), py_str("left my husband"), py_str("left my partner"), py_str("left the country"), py_str("left school"), py_str("left it too late"), py_str("left in the tin"), py_str("left the lid"), py_str("left over"), py_str("marinate"), py_str("marinade"), py_str("marinating"), py_str("marinated"), py_str("left on"), py_str("left the oven"), py_str("left the light"), py_str("left it running"), py_str("out of date"), py_str("use by"), py_str("best before"), py_str("gone off"), py_str("gone bad"), py_str("expired"), py_str("went off"), py_str("smells off"), py_str("how long does it last"), py_str("past its date"), py_str("went white"), py_str("white"), py_str("bloom"), py_str("bloomed"), py_str("bloomy"), py_str("grey"), py_str("greyed"), py_str("turned pale"), py_str("carpet"), py_str("carpets"), py_str("rug"), py_str("upholstery"), py_str("curtain"), py_str("curtains"), py_str("mattress"), py_str("shirt"), py_str("shirts"), py_str("jumper"), py_str("jumpers"), py_str("blouse"), py_str("trousers"), py_str("fabric"), py_str("sleeve"), py_str("sleeves"), py_str("cushion"), py_str("cushions"), py_str("red wine"), py_str("white wine"), py_str("coffee on"), py_str("on my shirt"), py_str("on the carpet"), py_str("out of the carpet"), py_str("the sofa"), py_str("my sofa"), py_str("cream sofa"), py_str("on the sofa"), py_str("out of the sofa"), py_str("ask out"), py_str("asking out"), py_str("asked out"), py_str("ask her out"), py_str("ask him out"), py_str("ask them out"), py_str("ask someone out"), py_str("ask a girl out"), py_str("ask a guy out"), py_str("crush"), py_str("chickened out of"), py_str("chickening out of"), py_str("chicken out of"), py_str("chickened out about"), py_str("chickening out about"), py_str("i chickened out"), py_str("i chicken out"), py_str("i chickening out"), py_str("i keep chickening"), py_str("i always chicken"), py_str("i usually chicken"), py_str("ive chickened out"), py_str("i've chickened"), py_str("on a date"), py_str("first date")])), ("guard".to_string(), g_soak_but_not_left_out_or_in_the_freezer())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("food-in-the-car")); __args.push(py_tuple(vec![py_tuple(vec![py_str("milk"), py_str("cream"), py_str("yoghurt"), py_str("yogurt"), py_str("formula"), py_str("bottle"), py_str("cheese"), py_str("butter"), py_str("chicken"), py_str("turkey"), py_str("ham"), py_str("sausages"), py_str("sausage"), py_str("bacon"), py_str("sandwich"), py_str("sandwiches"), py_str("wrap"), py_str("prawns"), py_str("fish"), py_str("mince"), py_str("steak"), py_str("eggs"), py_str("mayonnaise"), py_str("mayo"), py_str("cake"), py_str("chocolate"), py_str("food"), py_str("foods"), py_str("shopping"), py_str("groceries"), py_str("leftovers"), py_str("lunch"), py_str("snacks"), py_str("takeaway")]), py_tuple(vec![py_str("in the car"), py_str("in my car"), py_str("in the boot"), py_str("back seat"), py_str("on the back seat"), py_str("car boot"), py_str("in a hot car"), py_str("hot car"), py_str("warm car"), py_str("got warm"), py_str("went warm"), py_str("keep it cold"), py_str("keep food cold"), py_str("keep it cool"), py_str("keep cold"), py_str("cold in the car"), py_str("on the drive"), py_str("walked home"), py_str("on the way home"), py_str("in the van"), py_str("picnic")])])); __args.push(g__in_the_car()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("cool bag"), py_str("cool box"), py_str("cooler bag"), py_str("cooler box"), py_str("cool bag"), py_str("ice pack"), py_str("ice packs"), py_str("ice blocks"), py_str("freezer blocks"), py_str("picnic"), py_str("picnic box"), py_str("fridge"), py_str("refrigerator"), py_str("freezer"), py_str("how does"), py_str("how do fridges"), py_str("how it works"), py_str("why does a fridge"), py_str("compressor")])), ("ask".to_string(), py_tuple(vec![py_str("left"), py_str("leave"), py_str("leaves"), py_str("leaving"), py_str("forgot"), py_str("sat"), py_str("sitting"), py_str("kept"), py_str("stored"), py_str("packed"), py_str("bought"), py_str("brought"), py_str("how long"), py_str("how many hours"), py_str("still safe"), py_str("safe to eat"), py_str("got"), py_str("went"), py_str("get"), py_str("keep"), py_str("keeping"), py_str("cold"), py_str("warm")])), ("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pup"), py_str("puppies"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("pet"), py_str("pets")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }, "split", args))(&__args) }, &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("marinate-the-meat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("marinate"), py_str("marinade"), py_str("marinating"), py_str("marinated"), py_str("marinade for"), py_str("marinade in")]), py_tuple(vec![py_str("chicken"), py_str("meat"), py_str("steak"), py_str("lamb"), py_str("pork"), py_str("fish"), py_str("salmon"), py_str("tofu"), py_str("beef"), py_str("prawns"), py_str("turkey"), py_str("lamb chops"), py_str("chops")])])); __args.push(g__marinate()); __kws.extend(vec![("bias".to_string(), Py::Int(8))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fridge-order")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fridge"), py_str("fridge freezer"), py_str("fridges"), py_str("shelf"), py_str("shelves"), py_str("drawer"), py_str("crisper")]), py_tuple(vec![py_str("what goes off first"), py_str("goes off first"), py_str("go off first"), py_str("what should i eat first"), py_str("what should i use first"), py_str("what to eat first"), py_str("what to use first"), py_str("eat first"), py_str("use first"), py_str("use up first"), py_str("what goes where"), py_str("where in the fridge"), py_str("fridge order"), py_str("which shelf"), py_str("top shelf"), py_str("bottom shelf"), py_str("middle shelf"), py_str("salad drawer"), py_str("crisper"), py_str("how should i organise the fridge"), py_str("how do i organise my fridge"), py_str("how do i organise the fridge"), py_str("how do i stack the fridge"), py_str("what goes on which shelf")])])); __args.push(py_str("Eat in date order first, and after that the food that is on borrowed time: fresh fish and mince within a day or two, then soft fruit, berries and salad leaves, then milk, yoghurt and cream, then hard cheese, butter and cooked leftovers. Where it sits matters as much as the order - cooked and ready-to-eat food on the top shelves, raw meat and fish on the bottom shelf in a box so nothing drips, and vegetables in the drawer. The door is the warmest part, so keep milk out of it, and the top of the door is fine for butter and condiments.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("gone off"), py_str("is it ok"), py_str("safe to eat"), py_str("still ok"), py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("waste"), py_str("throw"), py_str("bin"), py_str("buy too much"), py_str("how long does"), py_str("how long can i keep"), py_str("use by"), py_str("best before"), py_str("smell")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("past-its-best")); __args.push(py_tuple(vec![g_PAST_IT_PRODUCE(), g_PAST_IT_STATE(), g_PAST_IT_ASK()])); __args.push(g__past_its_best()); __kws.extend(vec![("guard".to_string(), g__says_the_state()), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), py_tuple(vec![py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("green potato"), py_str("sprouting"), py_str("sprouted"), py_str("sprouts"), py_str("bread"), py_str("loaf"), py_str("baguette"), py_str("rolls"), py_str("toast"), py_str("croutons"), py_str("basil"), py_str("parsley"), py_str("mint"), py_str("coriander"), py_str("cilantro"), py_str("chives"), py_str("thyme"), py_str("rosemary"), py_str("herb"), py_str("herbs"), py_str("houseplant"), py_str("keep for"), py_str("keeps for"), py_str("how long do"), py_str("how long does"), py_str("how long will"), py_str("store"), py_str("storing"), py_str("storage"), py_str("keep them"), py_str("keep it"), py_str("last"), py_str("lasts"), py_str("garden"), py_str("plant"), py_str("plants"), py_str("tree"), py_str("grow"), py_str("growing"), py_str("fertiliser"), py_str("compost"), py_str("water them"), py_str("water it"), py_str("leaves on"), py_str("pot it"), py_str("repot"), py_str("from going slimy"), py_str("from going soft"), py_str("from going brown"), py_str("from going limp"), py_str("how do i keep"), py_str("how to keep"), py_str("how can i keep"), py_str("keep a bag"), py_str("keep them fresh"), py_str("keep it fresh"), py_str("stop it going"), py_str("stop them going"), py_str("prevent"), py_str("last longer"), py_str("stay fresh"), py_str("keep fresh"), py_str("make it last"), py_str("keep them from"), py_str("keep it from"), py_str("why does"), py_str("why do"), py_str("what makes"), py_str("what causes"), py_str("kitchen smells"), py_str("kitchen smell"), py_str("smells of"), py_str("smell of"), py_str("smells like"), py_str("smell like"), py_str("stinks of"), py_str("stink of"), py_str("the house smells"), py_str("the fridge smells"), py_str("too sour"), py_str("too sweet"), py_str("too salty"), py_str("too bitter"), py_str("a bit sour"), py_str("sauce"), py_str("soup"), py_str("stew"), py_str("curry"), py_str("casserole"), py_str("dough"), py_str("batter"), py_str("cake"), py_str("recipe"), py_str("recipes"), py_str("meal idea"), py_str("meal ideas"), py_str("limp"), py_str("limper"), py_str("wilted"), py_str("wilting"), py_str("wilt"), py_str("floppy leaf"), py_str("floppy leaves"), py_str("revive"), py_str("reviving"), py_str("bring it back"), py_str("bring them back"), py_str("crisp it up"), py_str("crisp them up"), py_str("make it crisp"), py_str("make them crisp"), py_str("what can i do with"), py_str("what to do with"), py_str("use it up"), py_str("use them up"), py_str("use up"), py_str("how long"), py_str("how many minutes"), py_str("soak"), py_str("soaking"), py_str("ice water"), py_str("iced water")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
}
pub fn soak_but_not_left_out(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True unless the line is a soak that also does not say the food was left out.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bsoak")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OUT_OF_THE_FRIDGE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn soak_but_not_left_out_or_in_the_freezer(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line is a soak with nothing out, or the food is still in the freezer.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| soak_but_not_left_out(args))(&__args) }))) {
    return Py::Bool(false);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__FREEZER_WORD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OUT_OF_THE_FREEZER(), "search", args))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _asks_if_it_is_still_all_right(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("True when the line asks whether the thing in hand can still be used.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STILL_OK_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_line_it_leans_on(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut earlier: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("The line, with the leaving line before it in front when the line asks about that food.\n\n    \"i left the chicken out for a few hours\" then \"can i still cook it\" is one question asked\n    twice: the leaving, the food and the hours are all in the first line, and the second only\n    says what the person wants to know. The reply reads the leaving off the first line, because\n    the food-safety rule is the answer to the second one - a recipe is not. Three things have to\n    hold, so an ordinary follow-up is never answered by this row: the line asks whether it is\n    still all right, the line before it was itself a leaving line, and both are about the same\n    food (\"i left the chicken out overnight\" then \"can i still cook the mince\" is not).\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) }) {
    return py_tuple(vec![text.clone(), Py::Bool(false)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_if_it_is_still_all_right(args))(&__args) }))) {
    return py_tuple(vec![text.clone(), Py::Bool(false)]);
    }
    earlier = py_str("");
    leans = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(reading.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leans_on")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) } } else { Py::None };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(leans.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&leans.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&earlier.clone()))) {
    let __try1 = exc::catch(|| {
    let mut last_user: Py = py_func("last_user", |args: &[Py]| gen_memory::memory::last_user(args));
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&last_user, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
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
    earlier = py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    earlier = op_add(&op_add(&py_str(" "), &earlier.clone()), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&earlier.clone(), "strip", args))(&__args) }))) {
    return py_tuple(vec![text.clone(), Py::Bool(false)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) } } }))) {
    return py_tuple(vec![text.clone(), Py::Bool(false)]);
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) } }) {
    return py_tuple(vec![text.clone(), Py::Bool(false)]);
    }
    return py_tuple(vec![op_add(&earlier.clone(), &text.clone()), Py::Bool(true)]);
    Py::None
}
pub fn says_a_food_was_left_out(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says a food was left out on the side, and so is a food-safety line.\n\n    The dish follow-ups read any line after a meal as being about the meal, and a person typing\n    \"i left the mince out all night\" three turns into a cooking talk got \"Mince and tomatoes make\n    a ragu\" - a recipe for food the bot had just told them to bin. The leaving is the subject of\n    this module, so it is asked here rather than spelled out again in bot/modules/answers.py.\n    ");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) };
    Py::None
}
pub fn _leaving_out(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let _ = py_str("Does the line say a food was left out, rather than that a dish came out a certain way?");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LEFT_OUT_PHRASE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEAVING_OUT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _hours_out(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("The hours the line says the food was out, or None if it does not say.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SPAN(), "search", args))(&__args) };
    if py_truthy(&found.clone()) {
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COUNT_WORDS(), "get", args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&count.clone(), &Py::None))) {
    let __try1 = exc::catch(|| {
    count = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("min")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "startswith", args))(&__args) }) { op_truediv(&count.clone(), &Py::Float(60.0)) } else { count.clone() };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(overnight|all night|the night|night|next morning|in the morning)\\b")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Float(8.0);
    }
    return Py::None;
    Py::None
}
pub fn _names_any(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut word: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _agree(args: &[Py]) -> Py {
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
    let mut many: Py = Py::None;
    let _ = py_str("Eggs are plural and a pizza is not, and the reply says which.");
    many = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("s")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ss")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "endswith", args))(&__args) })) } };
    return py_dict(vec![(py_str("be"), if py_truthy(&many.clone()) { py_str("are") } else { py_str("is") }), (py_str("were"), if py_truthy(&many.clone()) { py_str("were") } else { py_str("was") }), (py_str("them"), if py_truthy(&many.clone()) { py_str("them") } else { py_str("it") }), (py_str("they"), if py_truthy(&many.clone()) { py_str("they") } else { py_str("it") }), (py_str("look"), if py_truthy(&many.clone()) { py_str("look") } else { py_str("looks") }), (py_str("smell"), if py_truthy(&many.clone()) { py_str("smell") } else { py_str("smells") })]);
    Py::None
}
pub fn _how_long(args: &[Py]) -> Py {
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
    let mut soft: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "soft") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: soft"))) };
    let mut kept: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kept") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kept"))) };
    let mut meat: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "meat") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: meat"))) };
    let mut rice: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rice") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rice"))) };
    let mut dairy: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "dairy") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: dairy"))) };
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"How long can it sit out\" wants the rule, not a verdict on food nobody said was out.\n\n    The same table, told as a limit rather than as a bin notice: a person asking before it\n    happens must not be told to throw away a dish they are still cooking.\n    ");
    if py_truthy(&soft.clone()) {
    return py_str("Two hours on the side is the line for soft cheese: past that, bin it, because brie, camembert, mozzarella, feta and cream cheese can carry listeria, which grows in the fridge and changes neither the taste nor the smell. Hard cheese is the exception and can stay out a lot longer.");
    }
    if py_truthy(&rice.clone()) {
    return py_str("Two hours is the line for cooked rice, and it is the strictest food there is: cool it fast, get it into the fridge within the hour and eat it within a day, because the toxin it can grow survives reheating.");
    }
    if py_truthy(&meat.clone()) {
    return py_str("Two hours is the line for cooked meat, fish, gravy, pasta and anything with egg or mayonnaise in it - past that it is a bin job even if it looks and smells fine. Rice is the same. Bread, cake, butter, hard cheese, jam and fruit are the exceptions and come to no harm at all.");
    }
    if py_truthy(&dairy.clone()) {
    return py_str("Two hours for milk, cream and yoghurt, and a whole night out is the bin job: they sour and can grow bacteria you cannot see or smell. Go by the smell if it is borderline.");
    }
    if py_truthy(&kept.clone()) {
    word = geti(&kept.clone(), &Py::Int(0));
    verb = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _agree(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "capitalize", args))(&__args) }), ("be".to_string(), geti(&verb.clone(), &py_str("be"))), ("them".to_string(), geti(&verb.clone(), &py_str("them")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{thing} {be} one of the ones with no time limit: there was never a food-poisoning risk at room temperature, so a night out only makes {them} stale, dry or soft. The two hours and the bin apply to cooked meat, rice, milk and anything with mayonnaise."), "format", args))(&__args) };
    }
    return py_str("Two hours out of the fridge is the line for anything cooked, and after a whole night it is a bin job - meat, fish, rice, milk and anything with egg or mayonnaise especially. Bread, cake, butter, hard cheese, jam and fruit are the exceptions and keep for a long time on the side.");
    Py::None
}
pub fn _left_out(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut borrowed: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("What happens to a food that spent the night on the side depends on the food.\n\n    A follow-up that asks whether the food in hand can still be used (\"can i still cook it\") is\n    the same question as the line before it, which is where the food and the hours are, so the\n    verdict is said again as an answer about time rather than the whole card over again.\n    ");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }), &py_str(" "));
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(reading.clone()); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_line_it_leans_on(args))(&__args) }, 2, None);
    text = __unpk1[0].clone();
    borrowed = __unpk1[1].clone();
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _left_out_card(args))(&__args) };
    return if py_truthy(&{ let __b = borrowed.clone(); if !py_truthy(&__b) { __b } else { reply.clone() } }) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(reply.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_follow_up(args))(&__args) } } else { reply.clone() };
    Py::None
}
pub fn _left_out_card(args: &[Py]) -> Py {
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
    let mut bottle: Py = Py::None;
    let mut cooked_egg: Py = Py::None;
    let mut dairy: Py = Py::None;
    let mut egg: Py = Py::None;
    let mut hours: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut meat: Py = Py::None;
    let mut rice: Py = Py::None;
    let mut short: Py = Py::None;
    let mut sitting: Py = Py::None;
    let mut soft: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The left-out card for a line that names the leaving and the food.");
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__CHICKEN_OUT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEFT_OUT_WORD(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__TAKING_OUT(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_BIRD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SPENT_ON_THE_SIDE(), "search", args))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_COOKING_RESULT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hours_out(args))(&__args) }, &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SAID_OUT(), "search", args))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__LEFTOVER_WORD(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__OUT_OF_PLACE_WORD(), "search", args))(&__args) })) } }) {
    return Py::None;
    }
    hours = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hours_out(args))(&__args) };
    short = { let __b = Py::Bool(!py_is(&hours.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_le(&hours.clone(), &Py::Int(2)) } };
    soft = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_SOFT_CHEESE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    kept = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_KEEP()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    meat = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_MEAT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    rice = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_RICE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    dairy = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_DAIRY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    egg = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_EGG()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    cooked_egg = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(egg.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_EGG_COOKED()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_BURNT_WORK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_BURNT_VESSEL()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) } } }) {
    return Py::None;
    }
    sitting = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" sit"), py_str("sits"), py_str("sitting"), py_str(" leave"), py_str("leaves"), py_str("out"), py_str("keep"), py_str("keeps")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("how long"))); if !py_truthy(&__b) { __b } else { { let __b = sitting.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&cooked_egg.clone())) } } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(soft.clone()); __args.push(kept.clone()); __args.push(meat.clone()); __args.push(rice.clone()); __args.push(dairy.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _how_long(args))(&__args) };
    }
    if py_truthy(&{ let __b = cooked_egg.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&meat.clone())) } }) {
    meat = py_list(vec![py_str("egg")]);
    }
    if py_truthy(&{ let __b = egg.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&cooked_egg.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&short.clone())) } } } }) {
    return py_str("Whole eggs are fine: they are sold and kept on a shelf at room temperature and keep for weeks, so a night on the side changes nothing. It is a cracked egg, or a cooked one - scrambled, boiled, fried, an omelette - that follows the two-hour rule and goes in the bin after a night out.");
    }
    if py_truthy(&soft.clone()) {
    if py_truthy(&short.clone()) {
    return py_str("Inside the two hours, so it is fine even for this one: get it back in the fridge now and eat it within a couple of days. Soft cheese is the risky end of cheese, so it does not get the long stay a hard cheddar does.");
    }
    return py_str("Bin it, I'm afraid. Soft cheese is the risky end of cheese: brie, camembert, mozzarella, feta, cream cheese and blue cheese can carry listeria, which grows in the fridge and does not change the taste or the smell. Anything more than two hours out is a bin job, and a hard cheese is the only one you can leave.");
    }
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = meat.clone(); if py_truthy(&__b) { __b } else { { let __b = rice.clone(); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_DAIRY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) } } } } })) } }) {
    word = geti(&kept.clone(), &Py::Int(0));
    verb = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _agree(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "capitalize", args))(&__args) }), ("be".to_string(), geti(&verb.clone(), &py_str("be"))), ("them".to_string(), geti(&verb.clone(), &py_str("them")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{thing} {be} one of the safe ones: there was never a food-poisoning risk at room temperature, so a night on the side only makes {them} stale, dry or soft. Put {them} back and carry on - it is cooked meat, rice, milk and anything with mayonnaise that you cannot leave out."), "format", args))(&__args) };
    }
    bottle = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_BOTTLE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&bottle.clone()) {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&hours.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_ge(&hours.clone(), &Py::Int(3)) } }) {
    return py_str("Bin it. A made-up bottle is two hours from the moment it is made, and an hour once the baby has started it, whatever the room is like: bacteria from the baby's mouth get into the milk and warm milk grows them fast. It is not one of the things you can judge by smell.");
    }
    return py_str("Made-up formula or expressed milk is good for two hours at room temperature and a bottle the baby has started for one hour, so this one is inside the window: use it now rather than later, and past two hours it is a bin job. It is not dairy you can test by smell - two hours is the number, not the sniff.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_DAIRY()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }) {
    if py_truthy(&short.clone()) {
    return py_str("A couple of hours on the table is fine for milk, cream and yoghurt: get it back in the fridge now and it will be good to the date on the pack. A whole night out is the bin job, because they sour and can grow bacteria you cannot smell.");
    }
    return py_str("Milk, cream and yoghurt are fine on the table for a couple of hours at a meal, and a whole night out is a bin job: they sour and can grow bacteria you cannot see or smell. Keep it at the back of the fridge, not in the door, and go by the smell if you are not sure.");
    }
    if py_truthy(&rice.clone()) {
    if py_truthy(&short.clone()) {
    return py_str("Inside the two hours, so it is fine: get it into the fridge now in a shallow tub so it cools fast, and eat it within a day. Rice is the one to be careful with, because the toxin it can grow survives reheating.");
    }
    return py_str("Bin it after 2 hours out, and this one is the strictest of the lot: cooked rice can grow a toxin that reheating does not destroy, and it looks and smells fine while it does it. Next time cool it fast, fridge it within the hour, and eat it within a day.");
    }
    if py_truthy(&meat.clone()) {
    word = geti(&meat.clone(), &Py::Int(0));
    verb = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _agree(args))(&__args) };
    if py_truthy(&short.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "capitalize", args))(&__args) }), ("be".to_string(), geti(&verb.clone(), &py_str("be"))), ("them".to_string(), geti(&verb.clone(), &py_str("them")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{thing} inside the two-hour limit {be} fine: get {them} into the fridge now, where cooked food keeps 3 days. It is only past two hours that this becomes a bin job."), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "capitalize", args))(&__args) }), ("be".to_string(), geti(&verb.clone(), &py_str("be"))), ("they".to_string(), geti(&verb.clone(), &py_str("they"))), ("look".to_string(), geti(&verb.clone(), &py_str("look"))), ("smell".to_string(), geti(&verb.clone(), &py_str("smell")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Bin it, I'm afraid. {thing} left out for more than 2 hours {be} not safe to eat, even if {they} {look} and {smell} fine. Meat, fish, gravy and anything with egg or mayonnaise in it all follow that rule, so next time get it into the fridge within the hour."), "format", args))(&__args) };
    }
    if py_truthy(&short.clone()) {
    return py_str("Inside the two hours, so it is fine: get it covered into the fridge now and eat it within a day or two. Two hours out is the limit, and it is a night out that makes it a bin job.");
    }
    return py_str("Two hours out of the fridge is the line, and after a whole night it is a bin job for anything cooked, especially meat, fish, rice, milk or a dish with mayonnaise or egg in it. Bread, cake, butter, hard cheese, jam and fruit are the exceptions and come to no harm.");
    Py::None
}
pub fn the_still_ok_follow_up(args: &[Py]) -> Py {
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
    let mut here: Py = Py::None;
    let mut verdict: Py = Py::None;
    let _ = py_str("The verdict for \"can i still cook it\" after a line that had food left out on the side.\n\n    Returned as the turn's answer, before the dish follow-ups, because the line names no food of\n    its own: the leaving, the food and the hours are all in the line before it. Nothing else can\n    see that - the lookup reads the line alone, the dish follow-ups read it against the meal the\n    talk was on - so on main a person asked to confirm the verdict got a recipe for the very food\n    the bot had told them to bin. None when the two lines do not make that pair.\n    ");
    here = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_if_it_is_still_all_right(args))(&__args) } } }) {
    verdict = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_left_out_follow_up(args))(&__args) };
    if py_truthy(&verdict.clone()) {
    return verdict.clone();
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_food_it_just_said_to_bin(args))(&__args) };
    Py::None
}
pub fn _the_left_out_follow_up(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut here: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "here") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: here"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut card: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut named: Py = Py::None;
    let _ = py_str("The verdict for a line asking whether a food that sat out can still be cooked.");
    earlier = py_str("");
    let __try1 = exc::catch(|| {
    let mut last_user: Py = py_func("last_user", |args: &[Py]| gen_memory::memory::last_user(args));
    earlier = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&last_user, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) } } }))) {
    return Py::None;
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) } }) {
    return Py::None;
    }
    card = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&earlier.clone(), &here.clone())); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _left_out_card(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&card.clone()))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&earlier.clone(), &here.clone())); __args.push(card.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_follow_up(args))(&__args) };
    Py::None
}
pub fn _the_food_it_just_said_to_bin(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut here: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "here") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: here"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut agree: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut food: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What to do with a food the answer before this line told the person to bin.\n\n    Only that pair: the line has to ask what to do with the thing, and the answer before it has\n    to be a bin verdict - the food is read off the line before that, because the line asking the\n    question does not name it. A line naming a food of its own is a different question.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_WHAT_TO_DO_WITH_IT(), "search", args))(&__args) }))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    let mut last_bot: Py = py_func("last_bot", |args: &[Py]| gen_memory::memory::last_bot(args));
    let mut last_user: Py = py_func("last_user", |args: &[Py]| gen_memory::memory::last_user(args));
    said = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&last_bot, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
    earlier = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&last_user, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
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
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SAYS_BIN()) { let mut words: Py = __item.clone(); __out.push(Py::Bool(py_contains(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(". ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }, &Py::Int(0)), &words.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _asks_if_it_is_still_all_right(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _leaving_out(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HOW_IT_IS_KEEPING()) { let mut words: Py = __item.clone(); __out.push(Py::Bool(py_contains(&earlier.clone(), &words.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } } } }))) {
    return Py::None;
    }
    food = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&food.clone()))) {
    return Py::None;
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); __args.push(g_LEFT_OUT_FOODS()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(food.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) } }) {
    return Py::None;
    }
    thing = geti(&food.clone(), &Py::Int(0));
    agree = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _agree(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("look".to_string(), geti(&agree.clone(), &py_str("look"))), ("smell".to_string(), geti(&agree.clone(), &py_str("smell"))), ("be".to_string(), geti(&agree.clone(), &py_str("be"))), ("they".to_string(), geti(&agree.clone(), &py_str("they"))), ("them".to_string(), geti(&agree.clone(), &py_str("them")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Nothing to cook, I'm afraid: {thing} that {look} or {smell} like that {be} a bin job, and no recipe makes {them} safe - heat kills the bacteria growing on {them} but not the toxins they have already left behind. If the only thing wrong with {them} is the date on the packet, use {them} tonight rather than keep {them} for another day."), "format", args))(&__args) };
    Py::None
}
pub fn _the_follow_up(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut hours: Py = Py::None;
    let _ = py_str("The verdict as the answer to \"can I still cook it\", instead of the card over again.\n\n    Cooking does not undo the time on the side: heat kills the bacteria that grew while it sat\n    out, but not the toxins some of them have already left behind, so it can still make a person\n    ill. That is the answer to the follow-up, said about the rule rather than about the food. The\n    safe things - bread, cake, butter, hard cheese - keep the card, which is the right answer for\n    them, and so does a food this function cannot place.\n    ");
    first = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str(". ")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reply.clone(), "split", args))(&__args) }, &Py::Int(0)), "lower", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&first.clone(), &py_str("bin"))); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_LEFT_OUT_BOTTLE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) } } }) {
    return reply.clone();
    }
    hours = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _hours_out(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&hours.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_le(&hours.clone(), &Py::Int(2)) } }) {
    return py_str("Inside the two hours, so yes: use it now rather than later, and do not put it back in the fridge to keep. Two hours out is the line - past that it is a bin job even when it looks and smells fine.");
    }
    return py_str("Bin it, I'm afraid: past two hours out it is not safe, and cooking does not undo that - heat kills the bacteria that grew while it sat out, but not the toxins some of them have already left behind, so it can still make you ill. Two hours is the line for meat, fish, rice, milk and anything with egg or mayonnaise in it.");
    Py::None
}
pub fn _in_the_car(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut found: Py = Py::None;
    let mut hot: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A food carried in a warm car: the same two hours, less on a hot day.\n\n    The milk that sat on the back seat on the way home from the shop is the ordinary case, and it\n    is fine - the fridge is what it needs once it is in. It is the whole afternoon in the sun that\n    ends it, and that is the one thing the reply has to say, because it is the part a person\n    cannot work out from \"two hours out of the fridge\". The reply names the food it read, so a\n    line about sausages is not answered about milk.\n    ");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }), &py_str(" "));
    thing = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_LEFT_OUT_BOTTLE(), g_LEFT_OUT_DAIRY(), g_LEFT_OUT_MEAT(), g_LEFT_OUT_KEEP()]));
    for __item in __iter1 {
    words = __item.clone();
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(words.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&found.clone()) {
    thing = geti(&found.clone(), &Py::Int(0));
    break;
    }
    }
    hot = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(hot day|heatwave|in the sun|all afternoon|all day|sunny|summer|hot car|warm car)\\b")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_is(&thing.clone(), &Py::None))) {
    thing = py_str("food");
    }
    if py_truthy(&hot.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Thing".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "capitalize", args))(&__args) }), ("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("{Thing} in a hot car is the worst place it can be: a warm car gets well past fridge temperature, so count it as more than two hours out and bin anything cooked, dairy or made with mayonnaise that spent the afternoon there. On a hot day a cool bag, or buying the {thing} last, saves the whole question."), "format", args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(car|cars|boot|back seat|van|drive)\\b")); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    where_ = py_str("A car is a warm room with no fridge in it, so the two-hour rule applies to the {thing} as it does anywhere else: fine for the drive home, then straight into the fridge.");
    } else {
    where_ = py_str("Out of the fridge with no cool bag is the same as the table, so the two-hour rule applies to the {thing} as it does anywhere else: fine for the journey, then into the fridge as soon as you are in.");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_add(&where_.clone(), &py_str(" Anything cooked, dairy or made with mayonnaise that spent the afternoon warm is a bin job - warmth is what the bacteria that change neither taste nor smell need.")), "format", args))(&__args) };
    Py::None
}
pub fn _marinate(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut meat: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A marinade is meat in the fridge, not meat left on the side.\n\n    \"I want to marinate chicken overnight\" carried the words chicken and overnight and was\n    answered with the bin notice for chicken that sat out, which is the opposite of the question:\n    a marinade is meant to sit, and it sits in the fridge.\n    ");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&reading.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, "lower", args))(&__args) }), &py_str(" "));
    meat = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(py_tuple(vec![py_str("chicken"), py_str("fish"), py_str("salmon"), py_str("prawns"), py_str("turkey")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&meat.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), geti(&meat.clone(), &Py::Int(0)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("Overnight is fine for a marinade, in the fridge: {thing} takes 30 minutes to 2 hours at room temperature and up to a day in the fridge, and the fridge is the safer way to do it. Whisk 3 parts oil to 1 part acid - lemon, wine or vinegar - with salt, then garlic, honey, soy or herbs."), "format", args))(&__args) };
    }
    return py_str("Overnight is what a marinade is for, and it belongs in the fridge: tougher cuts take 4 hours or a full night, steak and lamb chops a few hours, and fish no more than an hour because the acid starts to cure it. Whisk 3 parts oil to 1 part acid - lemon, wine or vinegar - with salt, then garlic, honey, soy or herbs. More than a day in acid or sugar turns the meat mushy.");
    Py::None
}
pub fn _a_gadget_in_the_talk(args: &[Py]) -> Py {
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
    let mut one: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a gadget is what the conversation is about, not the food in the line.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GADGETS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(true);
    }
    turns = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    recent = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None)) { let mut one: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(one.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&one.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } } py_list(__out) })() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&recent.clone()) { let mut line: Py = __item.clone(); for __item in py_iter_or_raise(&g__GADGETS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _cool_before_fridge(args: &[Py]) -> Py {
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
    let mut quick: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How to get something hot into the fridge, said for the thing in the person's hand.\n\n    A pot of stew, a tub of leftovers and a bottle of warm milk cool at different speeds and are\n    eaten on different clocks, so the reply follows the object: an ask for the quick way gets the\n    sink of cold water first, cooked rice has its own hour and its own reason, milk is stood in a\n    jug of water, and a plate or a tub is spread shallow.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&words.clone(), "add", args))(&__args) };
    }
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&words.clone(), &g__A_COOLING_SUBJECT())))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_FAULT_IN_THE_FOOD(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwhat time\\b|\\bwhen should i\\b|\\bwhen do i\\b|\\bwhat time do i\\b")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:fridge|freez\\w*|refrigerat\\w*|cool\\w*|cold|ice|chill\\w*|worktop|defrost\\w*)\\b")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_gadget_in_the_talk(args))(&__args) }) {
    return Py::None;
    }
    quick = op_bitand(&words.clone(), &py_set(vec![py_str("quickly"), py_str("quick"), py_str("fast"), py_str("faster"), py_str("fastest"), py_str("quickest"), py_str("rapidly"), py_str("speed"), py_str("sooner"), py_str("hurry")]));
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("rice")))) {
    return py_str("Cool cooked rice fast, not slowly: spread it in a shallow tray and leave it no more than an hour, then cover it and put it in the fridge and eat it within a day, reheated until it is steaming hot. Rice left at room temperature grows a toxin that reheating does not destroy, which is why it gets an hour where a stew gets two.");
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("milk"), py_str("bottle"), py_str("bottles"), py_str("formula"), py_str("bottle of milk"), py_str("breast milk")]))) {
    return py_str("Stand the bottle in a jug of cold water for ten minutes, or hold it under the cold tap, then put it straight back in the fridge. Milk that has been warmed keeps about two days, so use it before the rest of the bottle and do not warm it twice - and never keep what a baby has already drunk from.");
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("brownie"), py_str("brownies"), py_str("muffin"), py_str("muffins"), py_str("cupcake"), py_str("cupcakes"), py_str("loaf"), py_str("loaves"), py_str("traybake"), py_str("bread"), py_str("buns")]))) {
    return py_str("Leave it in the tin for ten minutes, then turn it out onto a rack and let it go cold before anything goes on it: a warm cake steams itself soggy and the icing slides off. Do not put it in the fridge to speed it up - a chilled sponge goes heavy and stale faster than one kept in a tin.");
    }
    if py_truthy(&quick.clone()) {
    return py_str("Stand the pot in a sink of cold water, with a cold pack or a tray of ice in it, and stir every few minutes: a big pot comes down to warm in about 20 minutes rather than two hours. Then cover it and put it in the fridge - it does not need to be stone cold, just not steaming. Past two hours out of the fridge, bin it.");
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("pot"), py_str("pots"), py_str("pan"), py_str("saucepan"), py_str("stew"), py_str("soup"), py_str("curry"), py_str("chilli"), py_str("chili"), py_str("casserole"), py_str("stock"), py_str("batch"), py_str("bolognese"), py_str("sauce")]))) {
    return py_str("Let it cool on the worktop for up to two hours, then cover it and put it in the fridge. A big pot straight off the hob warms the whole fridge, so split it into shallow tubs first - it does not need to be stone cold, just not steaming. Past two hours out, bin it.");
    }
    return py_str("Spread it out in a shallow tub so it cools faster, leave it on the worktop for up to two hours, then cover it and put it in the fridge - it does not need to be stone cold, just not steaming. Eat leftovers within two days and reheat them until they are steaming hot all through. Past two hours out of the fridge, bin it.");
    Py::None
}
pub fn _says_the_state(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line says what state the food is in, not only what state it is.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NOT_WANTING_TO_EAT(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOW_TO_MAKE_IT_SO(), "search", args))(&__args) }) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__STATE_WORD(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _past_its_best(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut names: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The verdict on food that has gone a bit past its best, said about the food in hand.");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(g_PAST_IT_PRODUCE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _names_any(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PAST_IT_BIN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Bin it. The feel and the smell are what the stomach reacts to, not the colour, and cooking does not make it safe again: the bacteria that grew leave toxins behind that heat does not touch. Compost it and wash the drawer it was in.");
    }
    let __iter1 = py_iter_or_raise(&g_PAST_IT_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return said.clone();
    }
    }
    return g_PAST_IT_GENERAL();
    Py::None
}
pub fn the_cold_machine_and_not_the_dish(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("False when the line is the fridge or freezer being slow rather than a dish to cool.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    }
    return Py::Bool(!py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__COLD_MACHINE(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__IN_THE_DISH(), "search", args))(&__args) })) } }));
    Py::None
}
pub fn a_cooling_question_not_a_keeping_one(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True unless the line says how long the dish has been in the fridge already.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_PERIOD_OF_TIME(), "search", args))(&__args) }));
    Py::None
}
pub fn _not_just_a_hard_loaf(args: &[Py]) -> Py {
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
    let _ = py_str("False for a loaf that is dry or hard and says nothing about being spoilt.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__DRY_LOAF(), "search", args))(&__args) }))) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__BREAD_SPOILT(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _not_how_to_keep_it_fresh(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line asks how to stop a loaf going stale, not what to do with a stale one.");
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HOW_TO_KEEP_IT_FRESH(), "search", args))(&__args) }));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.freshness".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "BURNT_VESSEL" => Some(g_BURNT_VESSEL()),
            "BURNT_WORK" => Some(g_BURNT_WORK()),
            "COOKING_RESULT" => Some(g_COOKING_RESULT()),
            "EGG_COOKED" => Some(g_EGG_COOKED()),
            "ENTRIES" => Some(g_ENTRIES()),
            "HOW_IT_IS_KEEPING" => Some(g_HOW_IT_IS_KEEPING()),
            "LEFT_OUT_ASK" => Some(g_LEFT_OUT_ASK()),
            "LEFT_OUT_BOTTLE" => Some(g_LEFT_OUT_BOTTLE()),
            "LEFT_OUT_DAIRY" => Some(g_LEFT_OUT_DAIRY()),
            "LEFT_OUT_EGG" => Some(g_LEFT_OUT_EGG()),
            "LEFT_OUT_FOODS" => Some(g_LEFT_OUT_FOODS()),
            "LEFT_OUT_KEEP" => Some(g_LEFT_OUT_KEEP()),
            "LEFT_OUT_MEAT" => Some(g_LEFT_OUT_MEAT()),
            "LEFT_OUT_PHRASE" => Some(g_LEFT_OUT_PHRASE()),
            "LEFT_OUT_RICE" => Some(g_LEFT_OUT_RICE()),
            "LEFT_OUT_SOFT_CHEESE" => Some(g_LEFT_OUT_SOFT_CHEESE()),
            "OUT_OF_THE_FRIDGE" => Some(g_OUT_OF_THE_FRIDGE()),
            "PAST_IT_ASK" => Some(g_PAST_IT_ASK()),
            "PAST_IT_BIN" => Some(g_PAST_IT_BIN()),
            "PAST_IT_GENERAL" => Some(g_PAST_IT_GENERAL()),
            "PAST_IT_KINDS" => Some(g_PAST_IT_KINDS()),
            "PAST_IT_PRODUCE" => Some(g_PAST_IT_PRODUCE()),
            "PAST_IT_STATE" => Some(g_PAST_IT_STATE()),
            "SAYS_BIN" => Some(g_SAYS_BIN()),
            "STILL_OK_ASK" => Some(g_STILL_OK_ASK()),
            "WHAT_TO_DO_WITH_IT" => Some(g_WHAT_TO_DO_WITH_IT()),
            "_A_BIRD" => Some(g__A_BIRD()),
            "_A_COOLING_SUBJECT" => Some(g__A_COOLING_SUBJECT()),
            "_A_FAULT_IN_THE_FOOD" => Some(g__A_FAULT_IN_THE_FOOD()),
            "_A_PERIOD_OF_TIME" => Some(g__A_PERIOD_OF_TIME()),
            "_BREAD_SPOILT" => Some(g__BREAD_SPOILT()),
            "_CHICKEN_OUT" => Some(g__CHICKEN_OUT()),
            "_COLD_MACHINE" => Some(g__COLD_MACHINE()),
            "_COUNT_WORDS" => Some(g__COUNT_WORDS()),
            "_DRY_LOAF" => Some(g__DRY_LOAF()),
            "_FREEZER_WORD" => Some(g__FREEZER_WORD()),
            "_GADGETS" => Some(g__GADGETS()),
            "_HOW_TO_KEEP_IT_FRESH" => Some(g__HOW_TO_KEEP_IT_FRESH()),
            "_HOW_TO_MAKE_IT_SO" => Some(g__HOW_TO_MAKE_IT_SO()),
            "_IN_THE_DISH" => Some(g__IN_THE_DISH()),
            "_LEAVING_OUT" => Some(g__LEAVING_OUT()),
            "_LEAVING_VERB" => Some(g__LEAVING_VERB()),
            "_LEFTOVER_WORD" => Some(g__LEFTOVER_WORD()),
            "_LEFT_OUT_WORD" => Some(g__LEFT_OUT_WORD()),
            "_NOT_WANTING_TO_EAT" => Some(g__NOT_WANTING_TO_EAT()),
            "_OUT_OF_PLACE_WORD" => Some(g__OUT_OF_PLACE_WORD()),
            "_OUT_OF_THE_FREEZER" => Some(g__OUT_OF_THE_FREEZER()),
            "_SAID_OUT" => Some(g__SAID_OUT()),
            "_SPAN" => Some(g__SPAN()),
            "_SPENT_ON_THE_SIDE" => Some(g__SPENT_ON_THE_SIDE()),
            "_STATE_WORD" => Some(g__STATE_WORD()),
            "_TAKING_OUT" => Some(g__TAKING_OUT()),
            "_a_gadget_in_the_talk" => Some(py_func("_a_gadget_in_the_talk", |args: &[Py]| _a_gadget_in_the_talk(args))),
            "_agree" => Some(py_func("_agree", |args: &[Py]| _agree(args))),
            "_asks_if_it_is_still_all_right" => Some(py_func("_asks_if_it_is_still_all_right", |args: &[Py]| _asks_if_it_is_still_all_right(args))),
            "_cool_before_fridge" => Some(py_func("_cool_before_fridge", |args: &[Py]| _cool_before_fridge(args))),
            "_hours_out" => Some(py_func("_hours_out", |args: &[Py]| _hours_out(args))),
            "_how_long" => Some(py_func("_how_long", |args: &[Py]| _how_long(args))),
            "_in_the_car" => Some(py_func("_in_the_car", |args: &[Py]| _in_the_car(args))),
            "_leaving_out" => Some(py_func("_leaving_out", |args: &[Py]| _leaving_out(args))),
            "_left_out" => Some(py_func("_left_out", |args: &[Py]| _left_out(args))),
            "_left_out_card" => Some(py_func("_left_out_card", |args: &[Py]| _left_out_card(args))),
            "_marinate" => Some(py_func("_marinate", |args: &[Py]| _marinate(args))),
            "_names_any" => Some(py_func("_names_any", |args: &[Py]| _names_any(args))),
            "_not_how_to_keep_it_fresh" => Some(py_func("_not_how_to_keep_it_fresh", |args: &[Py]| _not_how_to_keep_it_fresh(args))),
            "_not_just_a_hard_loaf" => Some(py_func("_not_just_a_hard_loaf", |args: &[Py]| _not_just_a_hard_loaf(args))),
            "_past_its_best" => Some(py_func("_past_its_best", |args: &[Py]| _past_its_best(args))),
            "_says_the_state" => Some(py_func("_says_the_state", |args: &[Py]| _says_the_state(args))),
            "_the_follow_up" => Some(py_func("_the_follow_up", |args: &[Py]| _the_follow_up(args))),
            "_the_food_it_just_said_to_bin" => Some(py_func("_the_food_it_just_said_to_bin", |args: &[Py]| _the_food_it_just_said_to_bin(args))),
            "_the_left_out_follow_up" => Some(py_func("_the_left_out_follow_up", |args: &[Py]| _the_left_out_follow_up(args))),
            "_the_line_it_leans_on" => Some(py_func("_the_line_it_leans_on", |args: &[Py]| _the_line_it_leans_on(args))),
            "a_cooling_question_not_a_keeping_one" => Some(py_func("a_cooling_question_not_a_keeping_one", |args: &[Py]| a_cooling_question_not_a_keeping_one(args))),
            "says_a_food_was_left_out" => Some(py_func("says_a_food_was_left_out", |args: &[Py]| says_a_food_was_left_out(args))),
            "soak_but_not_left_out" => Some(py_func("soak_but_not_left_out", |args: &[Py]| soak_but_not_left_out(args))),
            "soak_but_not_left_out_or_in_the_freezer" => Some(py_func("soak_but_not_left_out_or_in_the_freezer", |args: &[Py]| soak_but_not_left_out_or_in_the_freezer(args))),
            "the_cold_machine_and_not_the_dish" => Some(py_func("the_cold_machine_and_not_the_dish", |args: &[Py]| the_cold_machine_and_not_the_dish(args))),
            "the_still_ok_follow_up" => Some(py_func("the_still_ok_follow_up", |args: &[Py]| the_still_ok_follow_up(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "BURNT_VESSEL" => s_BURNT_VESSEL(v),
            "BURNT_WORK" => s_BURNT_WORK(v),
            "COOKING_RESULT" => s_COOKING_RESULT(v),
            "EGG_COOKED" => s_EGG_COOKED(v),
            "ENTRIES" => s_ENTRIES(v),
            "HOW_IT_IS_KEEPING" => s_HOW_IT_IS_KEEPING(v),
            "LEFT_OUT_ASK" => s_LEFT_OUT_ASK(v),
            "LEFT_OUT_BOTTLE" => s_LEFT_OUT_BOTTLE(v),
            "LEFT_OUT_DAIRY" => s_LEFT_OUT_DAIRY(v),
            "LEFT_OUT_EGG" => s_LEFT_OUT_EGG(v),
            "LEFT_OUT_FOODS" => s_LEFT_OUT_FOODS(v),
            "LEFT_OUT_KEEP" => s_LEFT_OUT_KEEP(v),
            "LEFT_OUT_MEAT" => s_LEFT_OUT_MEAT(v),
            "LEFT_OUT_PHRASE" => s_LEFT_OUT_PHRASE(v),
            "LEFT_OUT_RICE" => s_LEFT_OUT_RICE(v),
            "LEFT_OUT_SOFT_CHEESE" => s_LEFT_OUT_SOFT_CHEESE(v),
            "OUT_OF_THE_FRIDGE" => s_OUT_OF_THE_FRIDGE(v),
            "PAST_IT_ASK" => s_PAST_IT_ASK(v),
            "PAST_IT_BIN" => s_PAST_IT_BIN(v),
            "PAST_IT_GENERAL" => s_PAST_IT_GENERAL(v),
            "PAST_IT_KINDS" => s_PAST_IT_KINDS(v),
            "PAST_IT_PRODUCE" => s_PAST_IT_PRODUCE(v),
            "PAST_IT_STATE" => s_PAST_IT_STATE(v),
            "SAYS_BIN" => s_SAYS_BIN(v),
            "STILL_OK_ASK" => s_STILL_OK_ASK(v),
            "WHAT_TO_DO_WITH_IT" => s_WHAT_TO_DO_WITH_IT(v),
            "_A_BIRD" => s__A_BIRD(v),
            "_A_COOLING_SUBJECT" => s__A_COOLING_SUBJECT(v),
            "_A_FAULT_IN_THE_FOOD" => s__A_FAULT_IN_THE_FOOD(v),
            "_A_PERIOD_OF_TIME" => s__A_PERIOD_OF_TIME(v),
            "_BREAD_SPOILT" => s__BREAD_SPOILT(v),
            "_CHICKEN_OUT" => s__CHICKEN_OUT(v),
            "_COLD_MACHINE" => s__COLD_MACHINE(v),
            "_COUNT_WORDS" => s__COUNT_WORDS(v),
            "_DRY_LOAF" => s__DRY_LOAF(v),
            "_FREEZER_WORD" => s__FREEZER_WORD(v),
            "_GADGETS" => s__GADGETS(v),
            "_HOW_TO_KEEP_IT_FRESH" => s__HOW_TO_KEEP_IT_FRESH(v),
            "_HOW_TO_MAKE_IT_SO" => s__HOW_TO_MAKE_IT_SO(v),
            "_IN_THE_DISH" => s__IN_THE_DISH(v),
            "_LEAVING_OUT" => s__LEAVING_OUT(v),
            "_LEAVING_VERB" => s__LEAVING_VERB(v),
            "_LEFTOVER_WORD" => s__LEFTOVER_WORD(v),
            "_LEFT_OUT_WORD" => s__LEFT_OUT_WORD(v),
            "_NOT_WANTING_TO_EAT" => s__NOT_WANTING_TO_EAT(v),
            "_OUT_OF_PLACE_WORD" => s__OUT_OF_PLACE_WORD(v),
            "_OUT_OF_THE_FREEZER" => s__OUT_OF_THE_FREEZER(v),
            "_SAID_OUT" => s__SAID_OUT(v),
            "_SPAN" => s__SPAN(v),
            "_SPENT_ON_THE_SIDE" => s__SPENT_ON_THE_SIDE(v),
            "_STATE_WORD" => s__STATE_WORD(v),
            "_TAKING_OUT" => s__TAKING_OUT(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.freshness", module_obj);
    pyrt::xrefs::reg_fn("knowledge.freshness", "soak_but_not_left_out", py_func("soak_but_not_left_out", |args: &[Py]| soak_but_not_left_out(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "soak_but_not_left_out_or_in_the_freezer", py_func("soak_but_not_left_out_or_in_the_freezer", |args: &[Py]| soak_but_not_left_out_or_in_the_freezer(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_asks_if_it_is_still_all_right", py_func("_asks_if_it_is_still_all_right", |args: &[Py]| _asks_if_it_is_still_all_right(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_the_line_it_leans_on", py_func("_the_line_it_leans_on", |args: &[Py]| _the_line_it_leans_on(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "says_a_food_was_left_out", py_func("says_a_food_was_left_out", |args: &[Py]| says_a_food_was_left_out(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_leaving_out", py_func("_leaving_out", |args: &[Py]| _leaving_out(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_hours_out", py_func("_hours_out", |args: &[Py]| _hours_out(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_names_any", py_func("_names_any", |args: &[Py]| _names_any(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_agree", py_func("_agree", |args: &[Py]| _agree(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_how_long", py_func("_how_long", |args: &[Py]| _how_long(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_left_out", py_func("_left_out", |args: &[Py]| _left_out(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_left_out_card", py_func("_left_out_card", |args: &[Py]| _left_out_card(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "the_still_ok_follow_up", py_func("the_still_ok_follow_up", |args: &[Py]| the_still_ok_follow_up(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_the_left_out_follow_up", py_func("_the_left_out_follow_up", |args: &[Py]| _the_left_out_follow_up(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_the_food_it_just_said_to_bin", py_func("_the_food_it_just_said_to_bin", |args: &[Py]| _the_food_it_just_said_to_bin(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_the_follow_up", py_func("_the_follow_up", |args: &[Py]| _the_follow_up(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_in_the_car", py_func("_in_the_car", |args: &[Py]| _in_the_car(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_marinate", py_func("_marinate", |args: &[Py]| _marinate(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_a_gadget_in_the_talk", py_func("_a_gadget_in_the_talk", |args: &[Py]| _a_gadget_in_the_talk(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_cool_before_fridge", py_func("_cool_before_fridge", |args: &[Py]| _cool_before_fridge(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_says_the_state", py_func("_says_the_state", |args: &[Py]| _says_the_state(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_past_its_best", py_func("_past_its_best", |args: &[Py]| _past_its_best(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "the_cold_machine_and_not_the_dish", py_func("the_cold_machine_and_not_the_dish", |args: &[Py]| the_cold_machine_and_not_the_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "a_cooling_question_not_a_keeping_one", py_func("a_cooling_question_not_a_keeping_one", |args: &[Py]| a_cooling_question_not_a_keeping_one(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_not_just_a_hard_loaf", py_func("_not_just_a_hard_loaf", |args: &[Py]| _not_just_a_hard_loaf(args)));
    pyrt::xrefs::reg_fn("knowledge.freshness", "_not_how_to_keep_it_fresh", py_func("_not_how_to_keep_it_fresh", |args: &[Py]| _not_how_to_keep_it_fresh(args)));
    pyrt::xrefs::reg_access("knowledge.freshness", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.freshness", "BURNT_VESSEL", g_BURNT_VESSEL, s_BURNT_VESSEL);
    pyrt::xrefs::reg_access("knowledge.freshness", "BURNT_WORK", g_BURNT_WORK, s_BURNT_WORK);
    pyrt::xrefs::reg_access("knowledge.freshness", "COOKING_RESULT", g_COOKING_RESULT, s_COOKING_RESULT);
    pyrt::xrefs::reg_access("knowledge.freshness", "EGG_COOKED", g_EGG_COOKED, s_EGG_COOKED);
    pyrt::xrefs::reg_access("knowledge.freshness", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.freshness", "HOW_IT_IS_KEEPING", g_HOW_IT_IS_KEEPING, s_HOW_IT_IS_KEEPING);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_ASK", g_LEFT_OUT_ASK, s_LEFT_OUT_ASK);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_BOTTLE", g_LEFT_OUT_BOTTLE, s_LEFT_OUT_BOTTLE);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_DAIRY", g_LEFT_OUT_DAIRY, s_LEFT_OUT_DAIRY);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_EGG", g_LEFT_OUT_EGG, s_LEFT_OUT_EGG);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_FOODS", g_LEFT_OUT_FOODS, s_LEFT_OUT_FOODS);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_KEEP", g_LEFT_OUT_KEEP, s_LEFT_OUT_KEEP);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_MEAT", g_LEFT_OUT_MEAT, s_LEFT_OUT_MEAT);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_PHRASE", g_LEFT_OUT_PHRASE, s_LEFT_OUT_PHRASE);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_RICE", g_LEFT_OUT_RICE, s_LEFT_OUT_RICE);
    pyrt::xrefs::reg_access("knowledge.freshness", "LEFT_OUT_SOFT_CHEESE", g_LEFT_OUT_SOFT_CHEESE, s_LEFT_OUT_SOFT_CHEESE);
    pyrt::xrefs::reg_access("knowledge.freshness", "OUT_OF_THE_FRIDGE", g_OUT_OF_THE_FRIDGE, s_OUT_OF_THE_FRIDGE);
    pyrt::xrefs::reg_access("knowledge.freshness", "PAST_IT_ASK", g_PAST_IT_ASK, s_PAST_IT_ASK);
    pyrt::xrefs::reg_access("knowledge.freshness", "PAST_IT_BIN", g_PAST_IT_BIN, s_PAST_IT_BIN);
    pyrt::xrefs::reg_access("knowledge.freshness", "PAST_IT_GENERAL", g_PAST_IT_GENERAL, s_PAST_IT_GENERAL);
    pyrt::xrefs::reg_access("knowledge.freshness", "PAST_IT_KINDS", g_PAST_IT_KINDS, s_PAST_IT_KINDS);
    pyrt::xrefs::reg_access("knowledge.freshness", "PAST_IT_PRODUCE", g_PAST_IT_PRODUCE, s_PAST_IT_PRODUCE);
    pyrt::xrefs::reg_access("knowledge.freshness", "PAST_IT_STATE", g_PAST_IT_STATE, s_PAST_IT_STATE);
    pyrt::xrefs::reg_access("knowledge.freshness", "SAYS_BIN", g_SAYS_BIN, s_SAYS_BIN);
    pyrt::xrefs::reg_access("knowledge.freshness", "STILL_OK_ASK", g_STILL_OK_ASK, s_STILL_OK_ASK);
    pyrt::xrefs::reg_access("knowledge.freshness", "WHAT_TO_DO_WITH_IT", g_WHAT_TO_DO_WITH_IT, s_WHAT_TO_DO_WITH_IT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_A_BIRD", g__A_BIRD, s__A_BIRD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_A_COOLING_SUBJECT", g__A_COOLING_SUBJECT, s__A_COOLING_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_A_FAULT_IN_THE_FOOD", g__A_FAULT_IN_THE_FOOD, s__A_FAULT_IN_THE_FOOD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_A_PERIOD_OF_TIME", g__A_PERIOD_OF_TIME, s__A_PERIOD_OF_TIME);
    pyrt::xrefs::reg_access("knowledge.freshness", "_BREAD_SPOILT", g__BREAD_SPOILT, s__BREAD_SPOILT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_CHICKEN_OUT", g__CHICKEN_OUT, s__CHICKEN_OUT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_COLD_MACHINE", g__COLD_MACHINE, s__COLD_MACHINE);
    pyrt::xrefs::reg_access("knowledge.freshness", "_COUNT_WORDS", g__COUNT_WORDS, s__COUNT_WORDS);
    pyrt::xrefs::reg_access("knowledge.freshness", "_DRY_LOAF", g__DRY_LOAF, s__DRY_LOAF);
    pyrt::xrefs::reg_access("knowledge.freshness", "_FREEZER_WORD", g__FREEZER_WORD, s__FREEZER_WORD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_GADGETS", g__GADGETS, s__GADGETS);
    pyrt::xrefs::reg_access("knowledge.freshness", "_HOW_TO_KEEP_IT_FRESH", g__HOW_TO_KEEP_IT_FRESH, s__HOW_TO_KEEP_IT_FRESH);
    pyrt::xrefs::reg_access("knowledge.freshness", "_HOW_TO_MAKE_IT_SO", g__HOW_TO_MAKE_IT_SO, s__HOW_TO_MAKE_IT_SO);
    pyrt::xrefs::reg_access("knowledge.freshness", "_IN_THE_DISH", g__IN_THE_DISH, s__IN_THE_DISH);
    pyrt::xrefs::reg_access("knowledge.freshness", "_LEAVING_OUT", g__LEAVING_OUT, s__LEAVING_OUT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_LEAVING_VERB", g__LEAVING_VERB, s__LEAVING_VERB);
    pyrt::xrefs::reg_access("knowledge.freshness", "_LEFTOVER_WORD", g__LEFTOVER_WORD, s__LEFTOVER_WORD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_LEFT_OUT_WORD", g__LEFT_OUT_WORD, s__LEFT_OUT_WORD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_NOT_WANTING_TO_EAT", g__NOT_WANTING_TO_EAT, s__NOT_WANTING_TO_EAT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_OUT_OF_PLACE_WORD", g__OUT_OF_PLACE_WORD, s__OUT_OF_PLACE_WORD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_OUT_OF_THE_FREEZER", g__OUT_OF_THE_FREEZER, s__OUT_OF_THE_FREEZER);
    pyrt::xrefs::reg_access("knowledge.freshness", "_SAID_OUT", g__SAID_OUT, s__SAID_OUT);
    pyrt::xrefs::reg_access("knowledge.freshness", "_SPAN", g__SPAN, s__SPAN);
    pyrt::xrefs::reg_access("knowledge.freshness", "_SPENT_ON_THE_SIDE", g__SPENT_ON_THE_SIDE, s__SPENT_ON_THE_SIDE);
    pyrt::xrefs::reg_access("knowledge.freshness", "_STATE_WORD", g__STATE_WORD, s__STATE_WORD);
    pyrt::xrefs::reg_access("knowledge.freshness", "_TAKING_OUT", g__TAKING_OUT, s__TAKING_OUT);
}

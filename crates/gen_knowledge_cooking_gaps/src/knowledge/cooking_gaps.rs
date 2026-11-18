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
thread_local! { static G_BURGER_APART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BURGER_APART_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIABETES_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIABETIC_BAKE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JACKET_FILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAKE_AT_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_SITUATIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_SITUATION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SWEET_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLOWED_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEAL_IS_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TABLE_OF_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TABLE_PUT_RIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAKE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILD_DRANK_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CUP_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EATING_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HERBS_GROWING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HERB_OWNED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HERB_PLANT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HERB_SURPLUS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HERB_USE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEASURE_AWAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEASURING_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THE_CROWD_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ORDER_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_IS_HOT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAN_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POWDER_FOR_SODA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RECIPE_TERM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STARTER_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TABLE_AHEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TABLE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_MEAL_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_TABLE_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIN_AREA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WOODY_HERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__crowd: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__crowd_meal: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__loaf_collapse_text: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__occasion: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_a_plan_that_might_be_ruined: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_a_table_of_people: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_bake_nothing_messy: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_bake_went_wrong: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_bake_with_children: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_batter_went_wrong: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_bored_of_chicken: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_bread_did_not_rise: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_croutons: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_dish_went_wrong: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_joint_weight: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_loaf_collapse: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_make_mayonnaise: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_no_eggs: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_potato_toppings: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_powder_for_soda: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_rice_burnt: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_rises_then_drops: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_roast_for_a_crowd: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_score_the_loaf: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_soup_curdle: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_starter_dying: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_starter_keeping: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_starter_mould: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry_starter_smell: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_not_ruled_out: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.cooking_gaps: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGER_APART() -> Py { ensure_init(); G_BURGER_APART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGER_APART(v: Py) { ensure_init(); G_BURGER_APART.with(|c| *c.borrow_mut() = v); }
pub fn g_BURGER_APART_REPLY() -> Py { ensure_init(); G_BURGER_APART_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BURGER_APART_REPLY(v: Py) { ensure_init(); G_BURGER_APART_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_DIABETES_WORDS() -> Py { ensure_init(); G_DIABETES_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIABETES_WORDS(v: Py) { ensure_init(); G_DIABETES_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIABETIC_BAKE() -> Py { ensure_init(); G_DIABETIC_BAKE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIABETIC_BAKE(v: Py) { ensure_init(); G_DIABETIC_BAKE.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_JACKET_FILL() -> Py { ensure_init(); G_JACKET_FILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JACKET_FILL(v: Py) { ensure_init(); G_JACKET_FILL.with(|c| *c.borrow_mut() = v); }
pub fn g_MAKE_AT_HOME() -> Py { ensure_init(); G_MAKE_AT_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAKE_AT_HOME(v: Py) { ensure_init(); G_MAKE_AT_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_ASK() -> Py { ensure_init(); G_MEAL_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_ASK(v: Py) { ensure_init(); G_MEAL_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_ASK_WORDS() -> Py { ensure_init(); G_MEAL_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_ASK_WORDS(v: Py) { ensure_init(); G_MEAL_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_SITUATIONS() -> Py { ensure_init(); G_MEAL_SITUATIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_SITUATIONS(v: Py) { ensure_init(); G_MEAL_SITUATIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_SITUATION_WORDS() -> Py { ensure_init(); G_MEAL_SITUATION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_SITUATION_WORDS(v: Py) { ensure_init(); G_MEAL_SITUATION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SWEET_ASK() -> Py { ensure_init(); G_SWEET_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SWEET_ASK(v: Py) { ensure_init(); G_SWEET_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLOWED_ASKS() -> Py { ensure_init(); G__ALLOWED_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLOWED_ASKS(v: Py) { ensure_init(); G__ALLOWED_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DIET() -> Py { ensure_init(); G__A_DIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DIET(v: Py) { ensure_init(); G__A_DIET.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEAL_IS_NAMED() -> Py { ensure_init(); G__A_MEAL_IS_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEAL_IS_NAMED(v: Py) { ensure_init(); G__A_MEAL_IS_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TABLE_OF_PEOPLE() -> Py { ensure_init(); G__A_TABLE_OF_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TABLE_OF_PEOPLE(v: Py) { ensure_init(); G__A_TABLE_OF_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TABLE_PUT_RIGHT() -> Py { ensure_init(); G__A_TABLE_PUT_RIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TABLE_PUT_RIGHT(v: Py) { ensure_init(); G__A_TABLE_PUT_RIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__BAKE_NAMED() -> Py { ensure_init(); G__BAKE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKE_NAMED(v: Py) { ensure_init(); G__BAKE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILD_DRANK_IT() -> Py { ensure_init(); G__CHILD_DRANK_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILD_DRANK_IT(v: Py) { ensure_init(); G__CHILD_DRANK_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORDS() -> Py { ensure_init(); G__COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORDS(v: Py) { ensure_init(); G__COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CUP_OF() -> Py { ensure_init(); G__CUP_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CUP_OF(v: Py) { ensure_init(); G__CUP_OF.with(|c| *c.borrow_mut() = v); }
pub fn g__EATING_VERBS() -> Py { ensure_init(); G__EATING_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EATING_VERBS(v: Py) { ensure_init(); G__EATING_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__FAULT() -> Py { ensure_init(); G__FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FAULT(v: Py) { ensure_init(); G__FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__HERBS_GROWING() -> Py { ensure_init(); G__HERBS_GROWING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HERBS_GROWING(v: Py) { ensure_init(); G__HERBS_GROWING.with(|c| *c.borrow_mut() = v); }
pub fn g__HERB_OWNED() -> Py { ensure_init(); G__HERB_OWNED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HERB_OWNED(v: Py) { ensure_init(); G__HERB_OWNED.with(|c| *c.borrow_mut() = v); }
pub fn g__HERB_PLANT_WORDS() -> Py { ensure_init(); G__HERB_PLANT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HERB_PLANT_WORDS(v: Py) { ensure_init(); G__HERB_PLANT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__HERB_SURPLUS_WORDS() -> Py { ensure_init(); G__HERB_SURPLUS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HERB_SURPLUS_WORDS(v: Py) { ensure_init(); G__HERB_SURPLUS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__HERB_USE_ASK() -> Py { ensure_init(); G__HERB_USE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HERB_USE_ASK(v: Py) { ensure_init(); G__HERB_USE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__MEASURE_AWAY() -> Py { ensure_init(); G__MEASURE_AWAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEASURE_AWAY(v: Py) { ensure_init(); G__MEASURE_AWAY.with(|c| *c.borrow_mut() = v); }
pub fn g__MEASURING_TAIL() -> Py { ensure_init(); G__MEASURING_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEASURING_TAIL(v: Py) { ensure_init(); G__MEASURING_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TABLE() -> Py { ensure_init(); G__NOT_A_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TABLE(v: Py) { ensure_init(); G__NOT_A_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THE_CROWD_CARD() -> Py { ensure_init(); G__NOT_THE_CROWD_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THE_CROWD_CARD(v: Py) { ensure_init(); G__NOT_THE_CROWD_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__ORDER_ASK() -> Py { ensure_init(); G__ORDER_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ORDER_ASK(v: Py) { ensure_init(); G__ORDER_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_IS_HOT_ASK() -> Py { ensure_init(); G__PAN_IS_HOT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_IS_HOT_ASK(v: Py) { ensure_init(); G__PAN_IS_HOT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PAN_THING() -> Py { ensure_init(); G__PAN_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAN_THING(v: Py) { ensure_init(); G__PAN_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__POWDER_FOR_SODA() -> Py { ensure_init(); G__POWDER_FOR_SODA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POWDER_FOR_SODA(v: Py) { ensure_init(); G__POWDER_FOR_SODA.with(|c| *c.borrow_mut() = v); }
pub fn g__RECIPE_TERM() -> Py { ensure_init(); G__RECIPE_TERM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RECIPE_TERM(v: Py) { ensure_init(); G__RECIPE_TERM.with(|c| *c.borrow_mut() = v); }
pub fn g__STARTER_NAMES() -> Py { ensure_init(); G__STARTER_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STARTER_NAMES(v: Py) { ensure_init(); G__STARTER_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__TABLE_AHEAD() -> Py { ensure_init(); G__TABLE_AHEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TABLE_AHEAD(v: Py) { ensure_init(); G__TABLE_AHEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__TABLE_WORDS() -> Py { ensure_init(); G__TABLE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TABLE_WORDS(v: Py) { ensure_init(); G__TABLE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_MEAL_NAMED() -> Py { ensure_init(); G__THE_MEAL_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_MEAL_NAMED(v: Py) { ensure_init(); G__THE_MEAL_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_TABLE_NAMED() -> Py { ensure_init(); G__THE_TABLE_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_TABLE_NAMED(v: Py) { ensure_init(); G__THE_TABLE_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g__TIN_AREA() -> Py { ensure_init(); G__TIN_AREA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIN_AREA(v: Py) { ensure_init(); G__TIN_AREA.with(|c| *c.borrow_mut() = v); }
pub fn g__WOODY_HERB() -> Py { ensure_init(); G__WOODY_HERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WOODY_HERB(v: Py) { ensure_init(); G__WOODY_HERB.with(|c| *c.borrow_mut() = v); }
pub fn g__crowd() -> Py { ensure_init(); G__crowd.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__crowd(v: Py) { ensure_init(); G__crowd.with(|c| *c.borrow_mut() = v); }
pub fn g__crowd_meal() -> Py { ensure_init(); G__crowd_meal.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__crowd_meal(v: Py) { ensure_init(); G__crowd_meal.with(|c| *c.borrow_mut() = v); }
pub fn g__loaf_collapse_text() -> Py { ensure_init(); G__loaf_collapse_text.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__loaf_collapse_text(v: Py) { ensure_init(); G__loaf_collapse_text.with(|c| *c.borrow_mut() = v); }
pub fn g__occasion() -> Py { ensure_init(); G__occasion.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__occasion(v: Py) { ensure_init(); G__occasion.with(|c| *c.borrow_mut() = v); }
pub fn g_a_plan_that_might_be_ruined() -> Py { ensure_init(); G_a_plan_that_might_be_ruined.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_a_plan_that_might_be_ruined(v: Py) { ensure_init(); G_a_plan_that_might_be_ruined.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_a_table_of_people() -> Py { ensure_init(); G_entry_a_table_of_people.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_a_table_of_people(v: Py) { ensure_init(); G_entry_a_table_of_people.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_bake_nothing_messy() -> Py { ensure_init(); G_entry_bake_nothing_messy.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_bake_nothing_messy(v: Py) { ensure_init(); G_entry_bake_nothing_messy.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_bake_went_wrong() -> Py { ensure_init(); G_entry_bake_went_wrong.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_bake_went_wrong(v: Py) { ensure_init(); G_entry_bake_went_wrong.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_bake_with_children() -> Py { ensure_init(); G_entry_bake_with_children.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_bake_with_children(v: Py) { ensure_init(); G_entry_bake_with_children.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_batter_went_wrong() -> Py { ensure_init(); G_entry_batter_went_wrong.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_batter_went_wrong(v: Py) { ensure_init(); G_entry_batter_went_wrong.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_bored_of_chicken() -> Py { ensure_init(); G_entry_bored_of_chicken.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_bored_of_chicken(v: Py) { ensure_init(); G_entry_bored_of_chicken.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_bread_did_not_rise() -> Py { ensure_init(); G_entry_bread_did_not_rise.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_bread_did_not_rise(v: Py) { ensure_init(); G_entry_bread_did_not_rise.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_croutons() -> Py { ensure_init(); G_entry_croutons.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_croutons(v: Py) { ensure_init(); G_entry_croutons.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_dish_went_wrong() -> Py { ensure_init(); G_entry_dish_went_wrong.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_dish_went_wrong(v: Py) { ensure_init(); G_entry_dish_went_wrong.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_joint_weight() -> Py { ensure_init(); G_entry_joint_weight.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_joint_weight(v: Py) { ensure_init(); G_entry_joint_weight.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_loaf_collapse() -> Py { ensure_init(); G_entry_loaf_collapse.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_loaf_collapse(v: Py) { ensure_init(); G_entry_loaf_collapse.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_make_mayonnaise() -> Py { ensure_init(); G_entry_make_mayonnaise.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_make_mayonnaise(v: Py) { ensure_init(); G_entry_make_mayonnaise.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_no_eggs() -> Py { ensure_init(); G_entry_no_eggs.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_no_eggs(v: Py) { ensure_init(); G_entry_no_eggs.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_potato_toppings() -> Py { ensure_init(); G_entry_potato_toppings.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_potato_toppings(v: Py) { ensure_init(); G_entry_potato_toppings.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_powder_for_soda() -> Py { ensure_init(); G_entry_powder_for_soda.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_powder_for_soda(v: Py) { ensure_init(); G_entry_powder_for_soda.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_rice_burnt() -> Py { ensure_init(); G_entry_rice_burnt.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_rice_burnt(v: Py) { ensure_init(); G_entry_rice_burnt.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_rises_then_drops() -> Py { ensure_init(); G_entry_rises_then_drops.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_rises_then_drops(v: Py) { ensure_init(); G_entry_rises_then_drops.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_roast_for_a_crowd() -> Py { ensure_init(); G_entry_roast_for_a_crowd.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_roast_for_a_crowd(v: Py) { ensure_init(); G_entry_roast_for_a_crowd.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_score_the_loaf() -> Py { ensure_init(); G_entry_score_the_loaf.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_score_the_loaf(v: Py) { ensure_init(); G_entry_score_the_loaf.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_soup_curdle() -> Py { ensure_init(); G_entry_soup_curdle.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_soup_curdle(v: Py) { ensure_init(); G_entry_soup_curdle.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_starter_dying() -> Py { ensure_init(); G_entry_starter_dying.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_starter_dying(v: Py) { ensure_init(); G_entry_starter_dying.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_starter_keeping() -> Py { ensure_init(); G_entry_starter_keeping.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_starter_keeping(v: Py) { ensure_init(); G_entry_starter_keeping.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_starter_mould() -> Py { ensure_init(); G_entry_starter_mould.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_starter_mould(v: Py) { ensure_init(); G_entry_starter_mould.with(|c| *c.borrow_mut() = v); }
pub fn g_entry_starter_smell() -> Py { ensure_init(); G_entry_starter_smell.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry_starter_smell(v: Py) { ensure_init(); G_entry_starter_smell.with(|c| *c.borrow_mut() = v); }
pub fn g_not_ruled_out() -> Py { ensure_init(); G_not_ruled_out.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_not_ruled_out(v: Py) { ensure_init(); G_not_ruled_out.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Three everyday kitchen questions the cooking table had no answer for.\n\nCooking itself is too big to add to: bot/knowledge/cooking.py is at the hub's one-megabyte\nlimit, so a change to it cannot be uploaded at all. The rows here are new answers rather than\nedits to an existing one, which makes this the cheap place to put them - they are ordinary\ncooking entries with the same key, reply and avoid shape as the rest.\n\nEach of these was measured on the whole bot before it was written:\n\n  why is my bread crumbly            \"I don't know enough to answer that honestly. A search...\"\n  what is a roux                     \"I'm not sure about that one, sorry - I'd rather not guess\"\n  my freezer is full what can i do   \"A dead appliance is usually the plug, the fuse or a door switch\"\n\nA crumbly loaf is a baking fault (water, flour, kneading, a warm cut) and not a stale or a flat\none, so those rows keep their lines. \"What is a roux\" is a definition and the sauce row's reply\nopens with the pan, so the definition has its own row and the sauce row stands down for it. And\na freezer with no room in it is not a broken appliance: `appliance-broken` in home.py keeps\n\"what can i do\" in its key for its own lines, and now stands down for a full or packed freezer.\n\nFour sentences at most for a kitchen answer, cause first: that is what a person wants back.\n\nThe last three are the other half of the baking and sauce faults. A loaf that collapses, a\nsourdough starter that stops working and a soup that curdles each name a fault the table had no\nrow for, so all eight ways of saying them were a shrug or an echo (\"why does my bread collapse\"\nwas a search hedge, \"my bread keeps collapsing\" was \"Thanks for telling me\"). They are apart from\nthe neighbours on purpose: the dense-bread answer owns \"sank in the middle\" and \"dense\", the flat\ncake answer owns \"my cake sank\", and the split-sauce answer owns \"my sauce split\" - a soup is a\npot of liquid with dairy or egg in it and wants the prevention, not the rescue.\n\nThe two rows at the end of the file came later. \"My curry went wrong\", with no word for what it\ndid, reached no row at all: every fault has one of its own - split, watery, salty, lumpy, mushy,\nsank, did not rise - so the line that names none of them got \"I'm listening - say more if it\nhelps\", and \"i ruined the curry\" got the roast meat answer about resting a joint. A dish that\nwent wrong is a short list of faults, and that list is the answer: four for a pot, three for a\nbake. Once the line names the fault the row for that fault is more specific and keeps it.\n");
    s_a_plan_that_might_be_ruined(py_func("a_plan_that_might_be_ruined", |args: &[Py]| pyrt::xrefs::xcall("knowledge.dishes", "a_plan_that_might_be_ruined", args)));
    s_not_ruled_out(py_func("not_ruled_out", |args: &[Py]| pyrt::xrefs::xcall("knowledge.dishes", "not_ruled_out", args)));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_JACKET_FILL(pyrt::xrefs::xget("knowledge.cooking", "JACKET_FILL"));
    s__crowd(py_func("_crowd", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_crowd", args)));
    s__crowd_meal(py_func("_crowd_meal", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_crowd_meal", args)));
    s__occasion(py_func("_occasion", |args: &[Py]| pyrt::xrefs::xcall("knowledge.cooking", "_occasion", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking")]));
    fn __data2() -> Py { py_tuple(vec![py_str("what can i bake"), py_str("what should i bake"), py_str("what shall i bake"), py_str("what can we bake"), py_str("what should we bake"), py_str("what to bake"), py_str("what can i bake today"), py_str("bake something"), py_str("something to bake"), py_str("baking ideas"), py_str("ideas for baking"), py_str("want to bake"), py_str("want to do some baking"), py_str("feel like baking"), py_str("fancy baking"), py_str("start baking"), py_str("get into baking"), py_str("getting into baking"), py_str("learn to bake"), py_str("what could i bake")]) }
    fn __data3() -> Py { py_tuple(vec![py_str("bake"), py_str("baking"), py_str("baked"), py_str("oven"), py_str("flour"), py_str("cake"), py_str("cakes"), py_str("biscuits"), py_str("cookies"), py_str("bread"), py_str("scones"), py_str("brownies"), py_str("easy"), py_str("quick"), py_str("simple"), py_str("today"), py_str("tonight"), py_str("weekend"), py_str("kids"), py_str("children"), py_str("beginner"), py_str("first time"), py_str("sweet"), py_str("something"), py_str("nothing"), py_str("banana"), py_str("bananas"), py_str("chocolate"), py_str("lemon")]) }
    fn __data4() -> Py { py_tuple(vec![py_str("cake sale"), py_str("bake sale"), py_str("sale"), py_str("stall"), py_str("school fair"), py_str("fete"), py_str("fayre"), py_str("birthday"), py_str("wedding"), py_str("christening"), py_str("party"), py_str("celebration"), py_str("how many people"), py_str("tin size"), py_str("which tin"), py_str("how long"), py_str("temperature"), py_str("gas mark"), py_str("fan oven"), py_str("bake blind"), py_str("blind"), py_str("pastry"), py_str("shortcrust"), py_str("baked potato"), py_str("baked potatoes"), py_str("jacket potato"), py_str("baked beans"), py_str("bake beans"), py_str("no oven"), py_str("microwave"), py_str("air fryer"), py_str("sourdough"), py_str("yeast"), py_str("why did my"), py_str("why is my"), py_str("sank"), py_str("soggy"), py_str("burnt"), py_str("crumbly"), py_str("stodgy"), py_str("goes flat"), py_str("went flat"), py_str("pizza"), py_str("casserole"), py_str("one pot"), py_str("traybake chicken"), py_str("fish"), py_str("salmon"), py_str("chicken")]) }
    fn __data5() -> Py { py_tuple(vec![py_str("crumbly"), py_str("crumbles"), py_str("crumbling"), py_str("falls apart"), py_str("falling apart"), py_str("fall apart"), py_str("breaks up"), py_str("breaking up"), py_str("falls to pieces"), py_str("sandy"), py_str("stodgy and dry"), py_str("dry and crumbly"), py_str("wont hold together"), py_str("won't hold together")]) }
    fn __data6() -> Py { py_tuple(vec![py_str("stale"), py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("green"), py_str("rise"), py_str("rising"), py_str("flat"), py_str("yeast"), py_str("prove"), py_str("proof"), py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("sponges"), py_str("muffin"), py_str("muffins"), py_str("cupcake"), py_str("cupcakes"), py_str("traybake"), py_str("tray bake"), py_str("brownie"), py_str("brownies"), py_str("scone"), py_str("scones")]) }
    fn __data7() -> Py { py_tuple(vec![py_str("defrost"), py_str("not working"), py_str("broken"), py_str("dead"), py_str("died"), py_str("faulty"), py_str("ice"), py_str("frost"), py_str("smell"), py_str("leak"), py_str("cold"), py_str("warm"), py_str("temperature")]) }
    fn __data8() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("sponges"), py_str("muffin"), py_str("muffins"), py_str("cupcake"), py_str("cupcakes"), py_str("loaf cake"), py_str("traybake"), py_str("tray bake"), py_str("brownies"), py_str("brownie"), py_str("victoria sponge")]) }
    fn __data9() -> Py { py_tuple(vec![py_str("rise"), py_str("rises"), py_str("risen"), py_str("rising"), py_str("didnt rise"), py_str("not risen"), py_str("sinking"), py_str("sank"), py_str("sunken"), py_str("icing"), py_str("frosting"), py_str("buttercream"), py_str("tin size"), py_str("how long in the oven"), py_str("dry"), py_str("dried out"), py_str("overbaked"), py_str("over baked"), py_str("how do i make"), py_str("recipe"), py_str("bake a cake"), py_str("make a cake"), py_str("freeze"), py_str("freezing"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("last"), py_str("lasts"), py_str("how long"), py_str("fridge"), py_str("refrigerate"), py_str("store"), py_str("storing"), py_str("stored"), py_str("stays fresh"), py_str("still good")]) }
    fn __data10() -> Py { py_tuple(vec![py_str("beer"), py_str("lager"), py_str("ale"), py_str("wine"), py_str("juice"), py_str("cider"), py_str("gravy"), py_str("sauce"), py_str("soup"), py_str("cloudy day"), py_str("weather"), py_str("sky"), py_str("rain"), py_str("urine"), py_str("water"), py_str("aquarium")]) }
    fn __data11() -> Py { py_tuple(vec![py_str("instead of"), py_str("swap"), py_str("swapping"), py_str("replace"), py_str("replacing"), py_str("substitute"), py_str("recipe"), py_str("make a cake"), py_str("make the cake"), py_str("make my cake"), py_str("bake a cake"), py_str("icing"), py_str("frosting"), py_str("buttercream"), py_str("tin size"), py_str("how long in the oven"), py_str("store"), py_str("storing"), py_str("keep"), py_str("keeping"), py_str("go off"), py_str("goes off"), py_str("gone off"), py_str("last"), py_str("healthiest"), py_str("healthy"), py_str("calorie"), py_str("calories"), py_str("reuse"), py_str("reusing"), py_str("deep fat"), py_str("fry"), py_str("frying"), py_str("oil paint"), py_str("hair"), py_str("skin"), py_str("drizzle"), py_str("dressing"), py_str("butter or"), py_str("or butter"), py_str("cake tin"), py_str("cake tins"), py_str("loaf tin"), py_str("tin"), py_str("tins"), py_str("grease"), py_str("greasing"), py_str("greased"), py_str("line the tin"), py_str("lining the tin"), py_str("loaf of bread"), py_str("bread dough"), py_str("dough")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("fresh"), py_str("stale"), py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("keep"), py_str("store"), py_str("last"), py_str("rise"), py_str("rising"), py_str("prove"), py_str("proof"), py_str("bake"), py_str("recipe"), py_str("crumbs"), py_str("sauce"), py_str("butter"), py_str("dough"), py_str("batter")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("ideas"), py_str("menu"), py_str("what else"), py_str("what should i have"), py_str("what should i make"), py_str("what to serve"), py_str("what can i serve"), py_str("what goes with"), py_str("what to have with"), py_str("sides"), py_str("side dishes"), py_str("serve with"), py_str("have with"), py_str("cook with it")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("lamb"), py_str("beef"), py_str("chicken"), py_str("pork"), py_str("turkey"), py_str("gammon"), py_str("duck"), py_str("ham"), py_str("goose"), py_str("potato"), py_str("potatoes"), py_str("parsnip"), py_str("parsnips"), py_str("carrot"), py_str("carrots"), py_str("yorkshire"), py_str("how long"), py_str("temperature"), py_str("temp"), py_str("gas mark"), py_str("defrost"), py_str("frozen"), py_str("freeze"), py_str("leftover"), py_str("leftovers"), py_str("veg"), py_str("vegetables"), py_str("vegan"), py_str("vegetarian"), py_str("nut roast"), py_str("casserole"), py_str("slow cooker"), py_str("air fryer"), py_str("roast veg"), py_str("roast vegetables"), py_str("roasted veg"), py_str("roasted vegetables")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("flatbread"), py_str("flatbreads"), py_str("soda bread"), py_str("naan"), py_str("pitta"), py_str("pita"), py_str("rolls"), py_str("buns"), py_str("bake"), py_str("baking"), py_str("make"), py_str("recipe"), py_str("recipes")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("crumbly"), py_str("stale"), py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("flat"), py_str("yeast"), py_str("sourdough"), py_str("cake"), py_str("cakes"), py_str("sauce"), py_str("gravy"), py_str("thicken"), py_str("roux"), py_str("batter"), py_str("pancake"), py_str("pancakes"), py_str("gluten free"), py_str("gluten-free"), py_str("bread machine"), py_str("pastry"), py_str("shortcrust"), py_str("short crust"), py_str("pie"), py_str("pies"), py_str("tart"), py_str("tarts"), py_str("quiche")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("steak"), py_str("roast"), py_str("brisket"), py_str("meat"), py_str("duck"), py_str("gammon"), py_str("ham"), py_str("chops"), py_str("chop"), py_str("whole bird"), py_str("crown"), py_str("belly pork"), py_str("loin"), py_str("joint of beef"), py_str("joint of lamb"), py_str("joint of pork"), py_str("joint of meat"), py_str("roasting joint"), py_str("beef joint"), py_str("lamb joint"), py_str("pork joint"), py_str("shoulder of lamb"), py_str("shoulder of pork"), py_str("lamb shoulder"), py_str("pork shoulder"), py_str("rib of beef"), py_str("beef ribs"), py_str("pork ribs"), py_str("spare ribs"), py_str("rack of ribs"), py_str("lamb ribs"), py_str("chump"), py_str("rack of lamb")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("juicy"), py_str("juicier"), py_str("juiciest"), py_str("moist"), py_str("moister"), py_str("tender"), py_str("tenderer"), py_str("succulent"), py_str("moisture"), py_str("not dry"), py_str("isnt dry"), py_str("isn't dry"), py_str("not go dry"), py_str("not dry out"), py_str("doesnt dry"), py_str("doesn't dry"), py_str("wont dry"), py_str("won't dry"), py_str("drying out"), py_str("dry out"), py_str("dries out"), py_str("not dried out"), py_str("from drying"), py_str("juices in"), py_str("stays moist"), py_str("comes out dry"), py_str("come out dry"), py_str("coming out dry"), py_str("turns out dry"), py_str("comes out tough"), py_str("comes out rubbery")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("chopping board"), py_str("cutting board"), py_str("wooden board"), py_str("wooden spoon"), py_str("spatula"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("frozen"), py_str("leftovers"), py_str("leftover"), py_str("how long"), py_str("days"), py_str("reheat"), py_str("reheating"), py_str("safe"), py_str("safely"), py_str("bacteria"), py_str("poisoning"), py_str("left out"), py_str("overnight"), py_str("thaw"), py_str("defrost"), py_str("marinade"), py_str("marinading"), py_str("marinating"), py_str("brine"), py_str("brining"), py_str("rub"), py_str("calories"), py_str("protein"), py_str("healthy"), py_str("healthier"), py_str("cheap"), py_str("price"), py_str("cost"), py_str("recipe"), py_str("air fryer"), py_str("slow cooker"), py_str("sous vide"), py_str("deep fry"), py_str("temperature to cook"), py_str("how long to cook"), py_str("what temperature"), py_str("breast"), py_str("breasts"), py_str("thigh"), py_str("thighs"), py_str("burgers"), py_str("mince"), py_str("meatballs"), py_str("stew"), py_str("casserole"), py_str("curry"), py_str("soup"), py_str("sandwich"), py_str("ribeye"), py_str("how do i cook"), py_str("how to cook"), py_str("how do i roast"), py_str("how to roast"), py_str("rest"), py_str("slow cook"), py_str("slow cooking"), py_str("slow-cooked"), py_str("slow roast"), py_str("rubbery"), py_str("rubber"), py_str("tough"), py_str("tougher"), py_str("toughest"), py_str("resting"), py_str("rest it"), py_str("how long in the oven"), py_str("roasting time"), py_str("cooking time"), py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("achy"), py_str("pain"), py_str("pains"), py_str("painful"), py_str("sore"), py_str("soreness"), py_str("stiff"), py_str("stiffness"), py_str("injury"), py_str("injured"), py_str("sprain"), py_str("sprained"), py_str("arthritis"), py_str("physio"), py_str("physiotherapy"), py_str("muscle"), py_str("muscles"), py_str("sciatica"), py_str("slipped disc"), py_str("trapped nerve"), py_str("hamstring"), py_str("elbow"), py_str("wrist"), py_str("ankle"), py_str("neck"), py_str("my back"), py_str("bad back"), py_str("back pain"), py_str("shoulder pain"), py_str("neck pain"), py_str("joint pain"), py_str("my knee"), py_str("my shoulder"), py_str("my ribs"), py_str("my hip"), py_str("my elbow"), py_str("my wrist"), py_str("my ankle"), py_str("my knees"), py_str("my joints"), py_str("shoulder ache"), py_str("sore shoulder"), py_str("cod"), py_str("haddock"), py_str("trout"), py_str("mackerel"), py_str("tuna"), py_str("prawns"), py_str("seafood"), py_str("fishcakes"), py_str("fish cake"), py_str("fish cakes")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("fish"), py_str("salmon"), py_str("cod"), py_str("haddock"), py_str("trout"), py_str("mackerel"), py_str("seafood"), py_str("fishcakes"), py_str("fish cake"), py_str("fish cakes"), py_str("sea bass"), py_str("bass"), py_str("hake"), py_str("pollock"), py_str("sardines"), py_str("kippers"), py_str("monkfish"), py_str("fish fillet"), py_str("fish fillets"), py_str("tuna steak"), py_str("tuna steaks")]) }
    fn __data21() -> Py { py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("leftovers"), py_str("how long"), py_str("days"), py_str("reheat"), py_str("safe"), py_str("safely"), py_str("bacteria"), py_str("poisoning"), py_str("left out"), py_str("overnight"), py_str("thaw"), py_str("defrost"), py_str("recipe"), py_str("healthy"), py_str("healthier"), py_str("protein"), py_str("calories"), py_str("cheap"), py_str("price"), py_str("cost"), py_str("buy"), py_str("shop"), py_str("mercury"), py_str("pregnant"), py_str("allergy"), py_str("allergic"), py_str("how do i cook"), py_str("how to cook"), py_str("how do i pan fry"), py_str("how to pan fry"), py_str("bake"), py_str("baking"), py_str("batter"), py_str("fish and chips"), py_str("breadcrumbs"), py_str("smoked"), py_str("smoking"), py_str("sushi"), py_str("raw"), py_str("shellfish"), py_str("prawn cocktail")]) }
    fn __data22() -> Py { py_tuple(vec![py_str("bored"), py_str("bored of"), py_str("bored with"), py_str("same dinners"), py_str("same meals"), py_str("same old dinners"), py_str("dinner rut"), py_str("stuck in a rut"), py_str("run out of ideas"), py_str("running out of ideas"), py_str("tired of cooking"), py_str("tired of the same"), py_str("sick of the same"), py_str("sick of cooking"), py_str("fed up of cooking"), py_str("fed up with cooking"), py_str("cooking the same")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("dinner"), py_str("dinners"), py_str("tea"), py_str("supper"), py_str("meals"), py_str("meal"), py_str("cooking"), py_str("cook"), py_str("food"), py_str("recipes"), py_str("ideas"), py_str("what can i try"), py_str("what can i make"), py_str("suggest"), py_str("weeknight")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("mince"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("fish"), py_str("salmon"), py_str("pasta"), py_str("rice"), py_str("potato"), py_str("potatoes"), py_str("eggs"), py_str("vegetarian"), py_str("vegan"), py_str("gluten"), py_str("dairy"), py_str("calorie"), py_str("calories"), py_str("healthy"), py_str("budget"), py_str("cheap"), py_str("student"), py_str("kids"), py_str("family"), py_str("party"), py_str("how long"), py_str("minutes"), py_str("freeze"), py_str("leftover"), py_str("leftovers"), py_str("goes with"), py_str("meal prep"), py_str("batch")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("home"), py_str("at home"), py_str("homemade"), py_str("myself"), py_str("starter"), py_str("culture"), py_str("ferment"), py_str("fermented"), py_str("set it"), py_str("from milk"), py_str("own yoghurt"), py_str("machine"), py_str("without a machine"), py_str("no machine")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("how long"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("gone off"), py_str("off"), py_str("bin"), py_str("use by"), py_str("healthy"), py_str("good for you"), py_str("bad for you"), py_str("calories"), py_str("sugar"), py_str("protein"), py_str("calcium"), py_str("dairy free"), py_str("lactose"), py_str("freeze"), py_str("frozen"), py_str("thick"), py_str("greek"), py_str("skin on")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("cheaper"), py_str("cheapest"), py_str("cheap"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("worth it"), py_str("worth buying"), py_str("worth the money"), py_str("better value"), py_str("value"), py_str("more expensive"), py_str("expensive"), py_str("save"), py_str("saves"), py_str("saving"), py_str("save money"), py_str("money"), py_str("vs"), py_str("versus"), py_str("instead of breasts"), py_str("instead of fillets"), py_str("or breasts"), py_str("or fillets")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("cheaper"), py_str("cheapest"), py_str("cheap"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("worth it"), py_str("worth buying"), py_str("worth the money"), py_str("better value"), py_str("value"), py_str("more expensive"), py_str("expensive"), py_str("save"), py_str("saves"), py_str("saving"), py_str("save money"), py_str("money"), py_str("vs"), py_str("versus"), py_str("instead of breasts"), py_str("instead of fillets"), py_str("or breasts"), py_str("or fillets")]) }
    fn __data1() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bake-ideas")); __args.push(py_tuple(vec![__data2(), __data3()])); __args.push(py_str("Scones take 15 minutes, flapjacks 25, a Victoria sponge 40 and banana bread an hour, so start with whichever time you have, and ripe bananas are the reason to make the last one. Weigh everything out before you start, cream the butter and sugar until the mixture is pale, and keep the oven door shut for the first two thirds of the time, because the cold air that gets in is what makes a cake sink.")); __kws.extend(vec![("avoid".to_string(), __data4())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("what-a-roux")); __args.push(py_tuple(vec![py_tuple(vec![py_str("roux"), py_str("beurre manie"), py_str("beurre manié")]), py_tuple(vec![py_str("what is"), py_str("whats a"), py_str("what's a"), py_str("what is a"), py_str("explain"), py_str("mean"), py_str("means"), py_str("definition"), py_str("difference"), py_str("used for"), py_str("for")])])); __args.push(py_str("A roux is equal weights of butter or oil and plain flour cooked together in a pan, and it is what thickens a white sauce, a gravy or a soup: melt 25g of butter, stir in 25g of plain flour and cook it a minute to take the raw flour taste off, then add the milk or stock a splash at a time, whisking. A pale roux thickens most, and the darker you cook it the less it thickens, which is why a brown roux for gravy needs more flour.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("without flour"), py_str("cornflour"), py_str("corn flour"), py_str("gluten free"), py_str("gluten-free"), py_str("cheese sauce"), py_str("white sauce"), py_str("bechamel"), py_str("béchamel")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bread-crumbly")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("sourdough"), py_str("buns"), py_str("roll"), py_str("rolls"), py_str("focaccia")]), __data5()])); __args.push(py_str("Crumbly bread is usually too little water or too much flour: the dough should be tacky and slightly sticky rather than dry enough to handle, so hold back a little of the flour and weigh it rather than scooping with a cup. Knead until a piece stretches thin enough to see light through it, which is what holds the crumb together, and let the loaf cool on a rack for an hour before cutting, because warm bread tears. If it is also pale and heavy, the oven was too cool or the loaf was under-proved.")); __kws.extend(vec![("avoid".to_string(), __data6())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("freezer-full")); __args.push(py_tuple(vec![py_tuple(vec![py_str("freezer"), py_str("freezers"), py_str("freezer drawer"), py_str("chest freezer")]), py_tuple(vec![py_str("full"), py_str("full up"), py_str("too full"), py_str("packed"), py_str("overfull"), py_str("no room"), py_str("stuffed"), py_str("no space"), py_str("run out of room"), py_str("run out of space"), py_str("bursting")])])); __args.push(py_str("A freezer works best about three-quarters full: the frozen food holds the cold, but packed to the door it cannot move air and the food at the front starts to thaw. Freeze flat in bags with the air pressed out and stand them on edge like files, keep the oldest at the front so it goes first, and use the crush to cook from it - a full freezer is a week of dinners if you eat what is in it. A drawer that will not close means it is time for a clear-out, not more squeezing.")); __kws.extend(vec![("avoid".to_string(), __data7())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cake-crumbly")); __args.push(py_tuple(vec![__data8(), py_tuple(vec![py_str("crumbly"), py_str("crumbles"), py_str("crumbling"), py_str("falls apart"), py_str("falling apart"), py_str("fall apart"), py_str("falls to pieces"), py_str("sandy"), py_str("wont hold together"), py_str("won't hold together"), py_str("breaks up"), py_str("breaking up")])])); __args.push(py_str("A crumbly cake usually has too much flour or too little fat and egg, or it was beaten on after the flour went in: weigh the flour rather than scooping it with a cup, and stop mixing the moment the last streaks of flour disappear. Take it out a shade early rather than late, because five extra minutes dries a sponge and dry is what crumbles, and cool it in the tin for ten minutes before turning it out. Cut it cold: a cake cut warm breaks the crumb as the knife goes through.")); __kws.extend(vec![("avoid".to_string(), __data9())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("stock-cloudy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("stock"), py_str("broth"), py_str("bouillon"), py_str("consomme"), py_str("consommé"), py_str("chicken stock"), py_str("beef stock"), py_str("vegetable stock"), py_str("bone broth")]), py_tuple(vec![py_str("cloudy"), py_str("clouded"), py_str("murky"), py_str("not clear"), py_str("unclear"), py_str("bits in it"), py_str("sediment"), py_str("how do i clear"), py_str("clear it"), py_str("make it clear"), py_str("looks cloudy")])])); __args.push(py_str("A cloudy stock boiled: once it comes to the boil the fat and protein break up into droplets that nothing will settle again, so keep it at the barest simmer with the lid off and skim the grey foam off the top in the first twenty minutes. To clear one that has already gone cloudy, whisk a lightly beaten egg white into the cold stock, bring it slowly to a simmer and let the white rise and set as a raft, then lift it out with the bits trapped in it. Strain through a sieve lined with kitchen paper, and do not stir it while it simmers or the raft breaks up.")); __kws.extend(vec![("avoid".to_string(), __data10())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("oil-in-a-cake")); __args.push(py_tuple(vec![py_tuple(vec![py_str("oil"), py_str("olive oil"), py_str("vegetable oil"), py_str("sunflower oil"), py_str("rapeseed oil"), py_str("cooking oil")]), py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("muffin"), py_str("muffins"), py_str("brownies"), py_str("brownie"), py_str("traybake"), py_str("tray bake"), py_str("carrot cake"), py_str("chocolate cake"), py_str("banana bread")])])); __args.push(py_str("Use 80g of oil for every 100g of butter the recipe asks for - about 4 to 5 tablespoons in a 20cm cake - and expect a slightly denser, moister crumb. Sunflower, vegetable or rapeseed gives a light plain cake; keep olive oil for a cake with a flavour of its own, like carrot or chocolate, where a good extra virgin is an asset rather than a taste you notice. Skip it where the method creams butter and sugar for air, and in pastry.")); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("ask".to_string(), py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how"), py_str("which"), py_str("what"), py_str("can i"), py_str("could i"), py_str("do i"), py_str("should i")])), ("avoid".to_string(), __data11())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("freeze-cooked-rice")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("long grain rice"), py_str("white rice"), py_str("cooked rice"), py_str("leftover rice")]), py_tuple(vec![py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("frozen"), py_str("freezer"), py_str("can i freeze"), py_str("freeze it")])])); __args.push(py_str("Yes: cool it within the hour, box it while it is still warm rather than hot, and freeze it the same day - cooked rice keeps about a month in the freezer. Reheat it once until it is piping hot all the way through, and do not freeze it again after that.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("freeze"), py_str("freezer"), py_str("frozen"), py_str("freezing")])), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("fry"), py_str("fried"), py_str("stir fry"), py_str("bake"), py_str("boil"), py_str("cook it"), py_str("how do i cook"), py_str("how long does it take"), py_str("reheat"), py_str("microwave"), py_str("pudding"), py_str("risotto")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("freeze-bread")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("sliced loaf"), py_str("sourdough")]), py_tuple(vec![py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("frozen"), py_str("freezer"), py_str("can i freeze"), py_str("freeze it")])])); __args.push(py_str("Slice it first, then bag it: bread keeps about three months in the freezer and the slices go straight into the toaster from frozen. Freeze it on the day you buy it, because a loaf frozen when it is already going stale thaws out stale.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("freeze"), py_str("freezer"), py_str("frozen"), py_str("freezing")])), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), __data12())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("roast-dinner-menu")); __args.push(py_tuple(vec![py_tuple(vec![py_str("roast"), py_str("roast dinner"), py_str("sunday roast"), py_str("roasting joint"), py_str("roast lunch")]), __data13()])); __args.push(py_str("A roast is one joint, potatoes, two veg and gravy, and the veg is where you ring the changes: carrots and parsnips roasted round the meat for the last 45 minutes, greens or cabbage steamed, and peas with mint if it is lamb. Add Yorkshire puddings for beef, stuffing and apple sauce for pork, bread sauce for chicken or turkey. Work back from the meat, because the joint rests 20 minutes and that is when the potatoes go up to full heat.")); __kws.extend(vec![("avoid".to_string(), __data14()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), pyrt::xrefs::xcall("knowledge.dishes", "not_ruled_out", &[py_str("sunday-roast")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bread-without-flour")); __args.push(py_tuple(vec![__data15(), py_tuple(vec![py_str("without flour"), py_str("with no flour"), py_str("no flour"), py_str("flourless"), py_str("out of flour"), py_str("ran out of flour"), py_str("run out of flour"), py_str("no bread flour"), py_str("none left of the flour"), py_str("no flour none left"), py_str("flour none left")])])); __args.push(py_str("With no flour at all, the quickest loaf is an oat one: blitz 300g porridge oats in a blender until they look like flour, then mix in 2 teaspoons of baking powder, 1 teaspoon of salt and 300g of thick yogurt or buttermilk, pour it into a lined loaf tin and bake at 180C for 40 to 45 minutes. For something in ten minutes, mash cold potato into a dough with 2 tablespoons of cornflour, salt and a knob of butter, pat it into flat rounds and fry them dry for three minutes a side. Ground almonds or a gluten-free bread flour blend also work cup for cup in a soda bread: 400g of either, 1 teaspoon of bicarbonate of soda, 350ml of buttermilk, 20 minutes at 200C.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("without flour"), py_str("with no flour"), py_str("no flour"), py_str("flourless"), py_str("out of flour"), py_str("ran out of flour"), py_str("run out of flour"), py_str("no bread flour"), py_str("none left of the flour"), py_str("no flour none left"), py_str("flour none left")])), ("avoid".to_string(), __data16()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-meat-juicy")); __args.push(py_tuple(vec![__data17(), __data18()])); __args.push(py_str("Dry meat is overcooked meat, so keep it juicy with a thermometer and a rest rather than basting: chicken and pork come out of the oven at 74C, beef at 55C for medium rare or 65C for well done, and then sit on a warm board under foil for 10 to 20 minutes, which lets the juice settle back in instead of running onto the plate. A 2kg chicken wants 20 minutes per 500g plus 20 at 190C, started breast down so the white meat sits in the fat. Basting mostly costs oven heat and changes little.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data19())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-fish-moist")); __args.push(py_tuple(vec![__data20(), py_tuple(vec![py_str("juicy"), py_str("juicier"), py_str("moist"), py_str("moister"), py_str("flakes"), py_str("rubbery"), py_str("stays moist"), py_str("keep it moist")])])); __args.push(py_str("Fish is done at 50 to 55C, or the moment a fork twists a flake and it just separates, and it keeps cooking off the heat, so take it out a touch early and rest it two minutes. Cook it through to 63C for anyone pregnant, elderly or with a weak immune system. A salmon fillet wants 4 to 6 minutes a side in a hot pan or 8 to 12 minutes at 200C, cod and haddock a minute or two more, a tuna steak only 1 to 2 minutes a side so the middle stays pink, and a whole fish 10 minutes per 2.5cm of thickness. Salt it 15 minutes ahead or not at all and keep lemon off it until it is off the heat: both draw water out. Flakes that stay in soft pieces mean it was in long enough, so lift it then rather than waiting for it to look white all through.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data21())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dinner-rut")); __args.push(py_tuple(vec![__data22(), __data23()])); __args.push(py_str("Try a traybake, a stir-fry and a pasta this week: change one of three things and the same dinner comes out new - the shape (traybake, stir-fry, pasta, curry, soup), the carb under it (rice, potatoes, couscous, bread, noodles) or the sauce that finishes it (tomato, yoghurt and lemon, a spice paste, gravy, pesto). Mince is a ragu, a chilli with rice, or a cottage pie. Keep five dinners you can cook without thinking and swap one of those each week.")); __kws.extend(vec![("avoid".to_string(), __data24()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-yoghurt")); __args.push(py_tuple(vec![py_tuple(vec![py_str("yoghurt"), py_str("yogurt"), py_str("yoghurt maker"), py_str("yogurt maker"), py_str("natural yoghurt"), py_str("live yoghurt")]), __data25()])); __args.push(py_str("Warm a litre of milk to just below boiling - 85C, or small bubbles round the edge of the pan - then let it cool to about 43C, which is a finger in it feeling warm rather than hot. Stir in 3 tablespoons of live natural yoghurt, keep it at that warmth for 8 to 12 hours in a flask, a yoghurt maker or an airing cupboard, and chill it once it has set; keep 3 spoons back as the starter for the next lot. Long or warm and it turns sour and grainy, cool or short and it stays runny.")); __kws.extend(vec![("avoid".to_string(), __data26())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("whole-chicken-value")); __args.push(py_tuple(vec![py_tuple(vec![py_str("whole chicken"), py_str("whole chickens"), py_str("a whole bird"), py_str("whole birds"), py_str("whole chook"), py_str("buy a whole chicken"), py_str("buying a whole chicken"), py_str("chicken whole"), py_str("whole chicken or breast"), py_str("whole chicken or breasts"), py_str("whole chicken vs breasts")]), __data27()])); __args.push(py_str("A whole chicken is nearly always the cheaper way to buy chicken: a 1.5kg bird gives about 700g of meat plus a carcass for stock, which is four portions for £4 to £5, where the same weight in breast fillets is £7 to £9. The catch is a third of the weight is bone and it takes 15 minutes to joint. Roast it whole for a Sunday, joint it for a curry or a traybake, and simmer the carcass for stock; buy fillets only when dinner has to be quick.")); __kws.extend(vec![("ask".to_string(), __data28()), ("avoid".to_string(), py_tuple(vec![py_str("turkey"), py_str("turkeys"), py_str("stuffing"), py_str("defrost"), py_str("freeze"), py_str("freezing"), py_str("frozen")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.ENTRIES"); }
    s_ENTRIES(__data1());
    fn __data29() -> Py { py_tuple(vec![py_str("went wrong"), py_str("gone wrong"), py_str("went out wrong"), py_str("went badly"), py_str("gone badly"), py_str("turned out wrong"), py_str("messed up"), py_str("screwed up"), py_str("ruined"), py_str("is a disaster"), py_str("was a disaster"), py_str("didnt work out"), py_str("didn't work out"), py_str("not gone right"), py_str("didnt turn out"), py_str("didn't turn out"), py_str("did not turn out"), py_str("doesnt turn out"), py_str("turned out badly"), py_str("turned out wrong"), py_str("was a flop"), py_str("flopped"), py_str("failed")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._FAULT"); }
    s__FAULT(__data29());
    fn __data30() -> Py { py_tuple(vec![py_str("curry"), py_str("stew"), py_str("casserole"), py_str("gravy"), py_str("soup"), py_str("broth"), py_str("sauce"), py_str("chilli"), py_str("chili"), py_str("risotto"), py_str("pasta"), py_str("lasagne"), py_str("lasagna"), py_str("bolognese"), py_str("ragu"), py_str("tagine"), py_str("dal"), py_str("daal"), py_str("stir fry"), py_str("stir-fry"), py_str("roast"), py_str("pie"), py_str("pudding"), py_str("porridge"), py_str("rice dish"), py_str("chicken"), py_str("rice"), py_str("fish"), py_str("salmon"), py_str("potatoes"), py_str("mash"), py_str("pizza"), py_str("noodles"), py_str("meatballs"), py_str("meatloaf"), py_str("takeaway"), py_str("food"), py_str("dinner"), py_str("dish"), py_str("meal"), py_str("supper")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("bread"), py_str("loaf"), py_str("scone"), py_str("scones"), py_str("pastry"), py_str("biscuit"), py_str("biscuits"), py_str("cookie"), py_str("cookies"), py_str("brownie"), py_str("brownies"), py_str("muffin"), py_str("muffins"), py_str("oven"), py_str("fridge"), py_str("freezer"), py_str("dishwasher"), py_str("washing machine"), py_str("washer"), py_str("boiler"), py_str("microwave"), py_str("kettle"), py_str("blender"), py_str("toaster"), py_str("hob"), py_str("cooker"), py_str("pan"), py_str("evening"), py_str("date"), py_str("day"), py_str("weekend"), py_str("holiday"), py_str("meal out"), py_str("dinner out"), py_str("party"), py_str("parties"), py_str("guests"), py_str("hosting"), py_str("eggs"), py_str("egg"), py_str("scrambled"), py_str("poached"), py_str("boiled"), py_str("pizza"), py_str("pizzas")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_dish_went_wrong"); }
    s_entry_dish_went_wrong({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("dish-went-wrong")); __args.push(py_tuple(vec![g__FAULT(), __data30()])); __args.push(py_str("Nearly always one of four things: too thin (simmer it uncovered for ten minutes), too salty (bulk it out with more of what is already in it, not water), caught on the bottom (into a clean pan, without scraping the base), or bland (salt a little, then a squeeze of lemon or a splash of vinegar). Tell me which it is and I will give you the fix for that dish.")); __kws.extend(vec![("avoid".to_string(), __data31()), ("guard".to_string(), g_a_plan_that_might_be_ruined())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data32() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("scone"), py_str("scones"), py_str("pastry"), py_str("biscuit"), py_str("biscuits"), py_str("cookie"), py_str("cookies"), py_str("brownie"), py_str("brownies"), py_str("muffin"), py_str("muffins"), py_str("bun"), py_str("buns"), py_str("yorkshire"), py_str("bake"), py_str("bakes"), py_str("baking"), py_str("sourdough"), py_str("dough"), py_str("pizza"), py_str("pizzas"), py_str("starter")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("oven"), py_str("fridge"), py_str("freezer"), py_str("dishwasher"), py_str("washing machine"), py_str("washer"), py_str("boiler"), py_str("microwave"), py_str("kettle"), py_str("blender"), py_str("toaster"), py_str("hob"), py_str("cooker"), py_str("pan"), py_str("pans"), py_str("evening"), py_str("date"), py_str("day"), py_str("weekend"), py_str("holiday"), py_str("recipe"), py_str("how do i make"), py_str("make a cake"), py_str("bake a cake"), py_str("without flour"), py_str("no flour"), py_str("freeze"), py_str("frozen"), py_str("keep"), py_str("keeping"), py_str("stale"), py_str("mouldy"), py_str("mould"), py_str("mold")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_bake_went_wrong"); }
    s_entry_bake_went_wrong({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bake-went-wrong")); __args.push(py_tuple(vec![g__FAULT(), __data32()])); __args.push(py_str("Nearly always one of three: an oven running 20C cooler than it says (an oven thermometer pays for itself), too much mixing once the flour went in (stop as soon as no dry flour shows), or old raising agent - bicarbonate and baking powder lose their fizz about six months after opening. Tell me what it did - sank, dry, heavy, burnt - and I will give you the fix for it.")); __kws.extend(vec![("avoid".to_string(), __data33()), ("guard".to_string(), g_a_plan_that_might_be_ruined())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data34() -> Py { py_tuple(vec![py_str("recipe"), py_str("how do i make"), py_str("how to make"), py_str("how do you make"), py_str("shrove"), py_str("day"), py_str("fish"), py_str("chicken"), py_str("shop"), py_str("supermarket"), py_str("freeze"), py_str("frozen"), py_str("bought"), py_str("oven"), py_str("fridge"), py_str("freezer"), py_str("dishwasher"), py_str("washing machine"), py_str("microwave")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_batter_went_wrong"); }
    s_entry_batter_went_wrong({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("batter-went-wrong")); __args.push(py_tuple(vec![g__FAULT(), py_tuple(vec![py_str("pancake"), py_str("pancakes"), py_str("crepe"), py_str("crepes"), py_str("waffle"), py_str("waffles"), py_str("batter"), py_str("drop scones"), py_str("blini"), py_str("blinis"), py_str("yorkshire pudding batter")])])); __args.push(py_str("A batter that goes wrong is nearly always one of three: too thick (a splash more milk, until it runs off the whisk in a ribbon), the pan not hot enough (a drop should sizzle the moment it lands, so heat it a minute longer than feels right), or turned too early (wait for bubbles across the top and a dry-looking edge). Resting the batter fifteen minutes fixes half of them, and a pinch of baking powder lifts the thick American ones.")); __kws.extend(vec![("avoid".to_string(), __data34())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_dish_went_wrong()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_bake_went_wrong()]);
    fn __data35() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("made"), py_str("own"), py_str("homemade"), py_str("recipe"), py_str("how do i"), py_str("how to"), py_str("what goes in"), py_str("from scratch"), py_str("without egg"), py_str("no egg"), py_str("egg free"), py_str("vegan")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("buy"), py_str("brand"), py_str("shop"), py_str("price"), py_str("cost"), py_str("gift"), py_str("healthy"), py_str("calories"), py_str("calorie"), py_str("diet"), py_str("jar"), py_str("jars"), py_str("bottle"), py_str("bottles"), py_str("recycling"), py_str("bin"), py_str("keep"), py_str("keeping"), py_str("store"), py_str("fridge"), py_str("freeze"), py_str("gone off"), py_str("off"), py_str("split"), py_str("splits"), py_str("split sauce"), py_str("curdled"), py_str("sandwich"), py_str("burger"), py_str("chips")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_make_mayonnaise"); }
    s_entry_make_mayonnaise({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-mayonnaise")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mayonnaise"), py_str("mayo"), py_str("aioli"), py_str("tartare sauce"), py_str("tartar sauce"), py_str("salad cream"), py_str("garlic mayo"), py_str("remoulade"), py_str("garlic aioli")]), __data35()])); __args.push(py_str("Whisk an egg yolk with a teaspoon of mustard and a pinch of salt, then add 250ml of oil in a thin thread to start with - drop by drop - and only pour faster once it has thickened; finish with a squeeze of lemon. If it splits, put a fresh yolk in a clean bowl and whisk the split batch in a spoonful at a time. Tartare sauce is mayonnaise with chopped capers, gherkins and parsley, aioli is the same with crushed garlic, and salad cream is mayonnaise loosened with vinegar and a pinch of sugar.")); __kws.extend(vec![("avoid".to_string(), __data36()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data37() -> Py { py_tuple(vec![py_str("revive"), py_str("soft again"), py_str("fresh again"), py_str("gone hard"), py_str("stuffing"), py_str("bread sauce"), py_str("pudding"), py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("keep"), py_str("keeping"), py_str("freeze"), py_str("frozen"), py_str("toast"), py_str("toasted cheese")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_croutons"); }
    s_entry_croutons({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("croutons-and-breadcrumbs")); __args.push(py_tuple(vec![py_tuple(vec![py_str("croutons"), py_str("crouton"), py_str("breadcrumbs"), py_str("bread crumbs"), py_str("panko"), py_str("toasted bread cubes")]), py_tuple(vec![py_str("make"), py_str("making"), py_str("own"), py_str("homemade"), py_str("from"), py_str("stale"), py_str("leftover"), py_str("left over"), py_str("how do i"), py_str("how to"), py_str("what can i do"), py_str("use up")])])); __args.push(py_str("Cut stale bread into cubes, toss them in olive oil with a little salt, and bake at 180C for 8 to 10 minutes until they rattle in the tin - a halved garlic clove in the oil is the whole trick. For breadcrumbs, tear the bread up, blitz it or grate it on the coarse side, dry them in the same oven for 5 minutes, and they keep a month in a jar.")); __kws.extend(vec![("avoid".to_string(), __data37()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._COUNT_WORDS"); }
    s__COUNT_WORDS(py_tuple(vec![py_tuple(vec![py_str("a dozen"), Py::Int(12)]), py_tuple(vec![py_str("four"), Py::Int(4)]), py_tuple(vec![py_str("five"), Py::Int(5)]), py_tuple(vec![py_str("six"), Py::Int(6)]), py_tuple(vec![py_str("seven"), Py::Int(7)]), py_tuple(vec![py_str("eight"), Py::Int(8)]), py_tuple(vec![py_str("nine"), Py::Int(9)]), py_tuple(vec![py_str("ten"), Py::Int(10)]), py_tuple(vec![py_str("eleven"), Py::Int(11)]), py_tuple(vec![py_str("twelve"), Py::Int(12)]), py_tuple(vec![py_str("sixteen"), Py::Int(16)]), py_tuple(vec![py_str("twenty"), Py::Int(20)])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._THE_MEAL_NAMED"); }
    s__THE_MEAL_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:roast|sunday (?:lunch|dinner)|christmas dinner|xmas dinner|turkey dinner|cooking (?:for|a meal)|doing the dinner)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._THE_TABLE_NAMED"); }
    s__THE_TABLE_NAMED(stdlib_call("re", "compile", &[py_str("\\b(?:for\\s+)?(?P<n>\\d{1,2}|a dozen|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|sixteen|twenty)\\s*(?:people|persons?|adults?|kids|children|guests?|of us|coming|eating)\\b|\\broast(?:\\s+is)?\\s+for\\s+(?P<only>\\d{1,2}|a dozen|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|sixteen|twenty)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data38() -> Py { py_dict(vec![(py_str("a dozen"), Py::Int(12)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("sixteen"), Py::Int(16)), (py_str("twenty"), Py::Int(20))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._TABLE_WORDS"); }
    s__TABLE_WORDS(__data38());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._A_TABLE_PUT_RIGHT"); }
    s__A_TABLE_PUT_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:actually|no|nope|sorry|make it|change it|instead)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data39() -> Py { py_tuple(vec![py_str("chicken"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("turkey"), py_str("duck"), py_str("goose"), py_str("gammon"), py_str("ham"), py_str("joint"), py_str("potato"), py_str("potatoes"), py_str("carrot"), py_str("carrots"), py_str("parsnip"), py_str("parsnips"), py_str("yorkshire"), py_str("stuffing"), py_str("gravy"), py_str("temperature"), py_str("temp"), py_str("gas mark"), py_str("defrost"), py_str("frozen"), py_str("freeze"), py_str("leftover"), py_str("leftovers"), py_str("vegan"), py_str("vegetarian"), py_str("veggie"), py_str("plant based"), py_str("nut roast"), py_str("slow cooker"), py_str("air fryer"), py_str("roast veg"), py_str("roast vegetables"), py_str("roasted veg"), py_str("roasted vegetables"), py_str("soggy"), py_str("dry"), py_str("burnt"), py_str("bland"), py_str("recipe"), py_str("ideas"), py_str("what goes with"), py_str("menu"), py_str("sides")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._NOT_THE_CROWD_CARD"); }
    s__NOT_THE_CROWD_CARD(__data39());
    fn __data40() -> Py { py_tuple(vec![py_str("timings"), py_str("timing"), py_str("timetable"), py_str("what time"), py_str("when do i start"), py_str("when should i start"), py_str("work back"), py_str("work backwards"), py_str("for four"), py_str("for 4"), py_str("for five"), py_str("for 5"), py_str("for six"), py_str("for 6"), py_str("for seven"), py_str("for 7"), py_str("for eight"), py_str("for 8"), py_str("for nine"), py_str("for 9"), py_str("for ten"), py_str("for 10"), py_str("for eleven"), py_str("for 11"), py_str("for twelve"), py_str("for 12"), py_str("for six people"), py_str("for eight people"), py_str("for ten people"), py_str("how much meat"), py_str("how much should i buy"), py_str("quantity"), py_str("per head"), py_str("per person"), py_str("how many"), py_str("plan"), py_str("schedule"), py_str("order of things")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("timing"), py_str("timings"), py_str("time"), py_str("timetable"), py_str("plan"), py_str("schedule"), py_str("when"), py_str("start"), py_str("how long"), py_str("much"), py_str("many"), py_str("order"), py_str("quantity"), py_str("per head"), py_str("per person"), py_str("for four"), py_str("for 4"), py_str("for five"), py_str("for 5"), py_str("for six"), py_str("for 6"), py_str("for seven"), py_str("for 7"), py_str("for eight"), py_str("for 8"), py_str("for nine"), py_str("for 9"), py_str("for ten"), py_str("for 10"), py_str("for eleven"), py_str("for 11"), py_str("for twelve"), py_str("for 12")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_roast_for_a_crowd"); }
    s_entry_roast_for_a_crowd({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("roast-for-a-crowd")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sunday roast"), py_str("roast dinner"), py_str("roast lunch"), py_str("sunday lunch"), py_str("sunday dinner"), py_str("roast"), py_str("roasting")]), __data40()])); __args.push(py_func("_roast_for_a_crowd", |args: &[Py]| _roast_for_a_crowd(args))); __kws.extend(vec![("ask".to_string(), __data41()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_make_mayonnaise()]);
    fn __data42() -> Py { py_tuple(vec![py_str("how many meals"), py_str("how many people"), py_str("how many portions"), py_str("feed"), py_str("feeds"), py_str("serves"), py_str("go round"), py_str("goes round"), py_str("go far"), py_str("goes far"), py_str("how far"), py_str("far"), py_str("enough for"), py_str("last"), py_str("stretch")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("how long"), py_str("oven"), py_str("roast it"), py_str("temperature"), py_str("defrost"), py_str("frozen"), py_str("freeze"), py_str("cheaper"), py_str("cost"), py_str("price"), py_str("value"), py_str("worth"), py_str("per person"), py_str("grams"), py_str("kg"), py_str("how much chicken"), py_str("stock cube"), py_str("curry recipe"), py_str("leftover chicken"), py_str("breast"), py_str("breasts"), py_str("fillet"), py_str("fillets"), py_str("thigh"), py_str("thighs"), py_str("wing"), py_str("wings"), py_str("leg"), py_str("legs"), py_str("drumstick"), py_str("drumsticks"), py_str("mince"), py_str("diced"), py_str("strips"), py_str("in advance"), py_str("prepare"), py_str("prep"), py_str("keep"), py_str("fridge"), py_str("freezer"), py_str("cold"), py_str("raw"), py_str("how long"), py_str("safe"), py_str("reheat"), py_str("defrost")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("whole-chicken-meals")); __args.push(py_tuple(vec![py_tuple(vec![py_str("whole chicken"), py_str("whole bird"), py_str("whole roast chicken"), py_str("a chicken")]), __data42()])); __args.push(py_str("Two meals from a 1.5kg bird, three from a 2kg one: four roast portions, then enough stripped off the frame for two more in a curry, a stir-fry or sandwiches - about 600g of cooked meat from 1.5kg, because a third of the raw weight is bone, skin and water. The frame then gives roughly 2 litres of stock, which is a soup or a risotto, and the fat roasts the potatoes.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("chicken"), py_str("bird")])), ("avoid".to_string(), __data43())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_croutons()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_batter_went_wrong()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_roast_for_a_crowd()]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._loaf_collapse_text"); }
    s__loaf_collapse_text(py_str("A loaf that collapses rose and then fell back, which is a prove that ran too long: the yeast has eaten its sugar and the gluten that held the bubbles gives way when the heat hits it. Shape it and prove it only until a floured finger pressed in leaves a dent that springs back slowly - if the dent stays, it is ready, and if it fills at once it is not. Bake it at 220C (200C fan) and leave the door shut for the first 20 minutes, and cut the water and yeast slightly next time; a loaf that also smells of alcohol went well past ready."));
    fn __data44() -> Py { py_tuple(vec![py_str("collapse"), py_str("collapses"), py_str("collapsed"), py_str("collapsing"), py_str("caved in"), py_str("caves in"), py_str("fell in"), py_str("falls in"), py_str("fell"), py_str("deflated"), py_str("deflates"), py_str("shrunk"), py_str("shrinks")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("dense"), py_str("sank"), py_str("sunken"), py_str("sinking"), py_str("flat"), py_str("didnt rise"), py_str("did not rise"), py_str("not rising"), py_str("wont rise"), py_str("won't rise"), py_str("crumbly"), py_str("stale"), py_str("mouldy"), py_str("mould"), py_str("mold"), py_str("freeze"), py_str("frozen"), py_str("recipe"), py_str("how do i make"), py_str("how long"), py_str("shelf life"), py_str("how to make"), py_str("oven temperature"), py_str("starter"), py_str("cake"), py_str("cakes"), py_str("banana"), py_str("malt"), py_str("soda")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_loaf_collapse"); }
    s_entry_loaf_collapse({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("loaf-collapse")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("sourdough"), py_str("sourdough loaf"), py_str("buns"), py_str("roll"), py_str("rolls"), py_str("focaccia"), py_str("tin loaf"), py_str("white loaf"), py_str("wholemeal loaf")]), __data44()])); __args.push(g__loaf_collapse_text()); __kws.extend(vec![("avoid".to_string(), __data45())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_loaf_collapse()]);
    fn __data46() -> Py { py_tuple(vec![py_str("rise"), py_str("rises"), py_str("risen"), py_str("rising"), py_str("goes up"), py_str("went up"), py_str("puffs up"), py_str("puffed up"), py_str("comes up"), py_str("came up"), py_str("springs up"), py_str("sprung up"), py_str("proves up"), py_str("proved up")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("drops"), py_str("dropped"), py_str("drops back"), py_str("dropped back"), py_str("falls"), py_str("fall"), py_str("fell"), py_str("fallen"), py_str("falls back"), py_str("fell back"), py_str("sinks"), py_str("sink"), py_str("sinking"), py_str("sank"), py_str("sunk"), py_str("collapses"), py_str("collapse"), py_str("collapsed"), py_str("collapsing"), py_str("deflates"), py_str("deflate"), py_str("deflated"), py_str("caves in"), py_str("cave in"), py_str("caved in"), py_str("flops"), py_str("flop"), py_str("flopped"), py_str("deflated")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("souffle"), py_str("soufflé"), py_str("yorkshire"), py_str("pudding"), py_str("muffin"), py_str("muffins"), py_str("scone"), py_str("scones"), py_str("meringue"), py_str("how long"), py_str("keep"), py_str("keeps"), py_str("shelf life"), py_str("recipe"), py_str("freeze"), py_str("frozen"), py_str("starter"), py_str("flat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_rises_then_drops"); }
    s_entry_rises_then_drops({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rises-then-drops")); __args.push(py_tuple(vec![__data46(), __data47()])); __args.push(g__loaf_collapse_text()); __kws.extend(vec![("avoid".to_string(), __data48())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_rises_then_drops()]);
    fn __data49() -> Py { py_tuple(vec![py_str("dying"), py_str("died"), py_str("dies"), py_str("dead"), py_str("killed"), py_str("killing"), py_str("not rising"), py_str("isnt rising"), py_str("isn't rising"), py_str("stopped rising"), py_str("stopped growing"), py_str("wont rise"), py_str("won't rise"), py_str("not bubbling"), py_str("stopped bubbling"), py_str("no bubbles"), py_str("flat and lifeless")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("how do i make"), py_str("how to make"), py_str("start from scratch"), py_str("from scratch"), py_str("name"), py_str("call it"), py_str("recipe"), py_str("buy"), py_str("shop"), py_str("freeze"), py_str("frozen"), py_str("fridge"), py_str("mould"), py_str("mold"), py_str("pink"), py_str("black"), py_str("bread"), py_str("breads"), py_str("loaf"), py_str("loaves"), py_str("dough"), py_str("roll"), py_str("rolls"), py_str("boule"), py_str("batard")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_starter_dying"); }
    s_entry_starter_dying({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("starter-dying")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sourdough starter"), py_str("sour dough starter"), py_str("starter"), py_str("starters"), py_str("levain"), py_str("sourdough")]), __data49()])); __args.push(py_str("A starter that has stopped rising is usually hungry, cold or both: give it equal weights of flour and water, about its own weight, at room temperature (20 to 25C) and it should double in four to eight hours. If it has gone quiet, feed it twice a day for three or four days without putting it back in the fridge, and use wholemeal or rye flour for one of those feeds, which wakes it faster than white. A dark liquid on top is alcohol, not death: pour it off and feed. Mould - pink, black or furry - is the only one to throw away.")); __kws.extend(vec![("avoid".to_string(), __data50())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_starter_dying()]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._STARTER_NAMES"); }
    s__STARTER_NAMES(py_tuple(vec![py_str("sourdough starter"), py_str("sour dough starter"), py_str("starter"), py_str("starters"), py_str("levain"), py_str("sourdough"), py_str("sour dough")]));
    fn __data51() -> Py { py_tuple(vec![py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("fur"), py_str("furry"), py_str("fuzz"), py_str("fuzzy"), py_str("green"), py_str("black"), py_str("pink"), py_str("orange"), py_str("white film"), py_str("white layer"), py_str("skin on top"), py_str("gone off"), py_str("ruined")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("bread"), py_str("breads"), py_str("loaf"), py_str("loaves"), py_str("cake"), py_str("cakes"), py_str("muffin"), py_str("muffins"), py_str("bun"), py_str("buns"), py_str("jam"), py_str("cheese"), py_str("yoghurt"), py_str("yogurt"), py_str("fruit"), py_str("freeze"), py_str("frozen"), py_str("name"), py_str("call it"), py_str("recipe"), py_str("how do i make"), py_str("how to make"), py_str("ratio")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_starter_mould"); }
    s_entry_starter_mould({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("starter-mould")); __args.push(py_tuple(vec![g__STARTER_NAMES(), __data51()])); __args.push(py_str("Look at the colour before you do anything. A white or creamy film on top, with no fur on it, is kahm yeast and the starter is fine: skim it off, move the jar to a clean one and feed it. Anything furry, or pink, orange, green or black, means mould has gone through the jar - throw it out and start again, because the spores are in the flour and not just on the surface. Wash the jar with hot soapy water before the next one, and keep the sides scraped down so nothing grows on the rim.")); __kws.extend(vec![("avoid".to_string(), __data52())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data53() -> Py { py_tuple(vec![py_str("keep"), py_str("keeping"), py_str("store"), py_str("storing"), py_str("fridge"), py_str("refrigerator"), py_str("how long"), py_str("last"), py_str("leave it"), py_str("leave my"), py_str("holiday"), py_str("holidays"), py_str("away"), py_str("two weeks"), py_str("a week"), py_str("month"), py_str("months"), py_str("travel"), py_str("forget"), py_str("forgot"), py_str("neglect")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("how do i feed"), py_str("how to feed"), py_str("feed it"), py_str("feeding"), py_str("ratio"), py_str("flour and water"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("name"), py_str("call it"), py_str("recipe"), py_str("how do i make"), py_str("how to make"), py_str("not rising"), py_str("wont rise"), py_str("died"), py_str("dead"), py_str("milk"), py_str("cheese"), py_str("meat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_starter_keeping"); }
    s_entry_starter_keeping({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("starter-keeping")); __args.push(py_tuple(vec![g__STARTER_NAMES(), __data53()])); __args.push(py_str("A starter in the fridge will sit for weeks without much harm and months if you feed it now and then: once a week is plenty in the cold, and a grey liquid on top is alcohol, which you pour off. Before you bake, take it out, feed it and let it double at room temperature, and feed it twice a day for two days if it has been in there a while. Going away for a month is fine - feed it thick, spread a spoonful on baking parchment, dry it and it can be woken up afterwards.")); __kws.extend(vec![("avoid".to_string(), __data54())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data55() -> Py { py_tuple(vec![py_str("smell"), py_str("smells"), py_str("smelling"), py_str("smelly"), py_str("stinks"), py_str("stink"), py_str("whiff"), py_str("acetone"), py_str("nail polish"), py_str("nail varnish"), py_str("vinegar"), py_str("vinegary"), py_str("sharp"), py_str("sickly"), py_str("alcohol"), py_str("beery")]) }
    fn __data56() -> Py { py_tuple(vec![py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("fur"), py_str("fuzzy"), py_str("pink"), py_str("orange"), py_str("green film"), py_str("not rising"), py_str("wont rise"), py_str("stopped rising"), py_str("died"), py_str("dead"), py_str("how do i feed"), py_str("how to feed"), py_str("feeding ratio"), py_str("how do i make"), py_str("how to make"), py_str("name"), py_str("call it"), py_str("recipe"), py_str("milk"), py_str("cheese"), py_str("yoghurt pot"), py_str("yogurt pot"), py_str("bread"), py_str("loaf"), py_str("loaves")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_starter_smell"); }
    s_entry_starter_smell({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("starter-smell")); __args.push(py_tuple(vec![g__STARTER_NAMES(), __data55()])); __args.push(py_str("Sniff it and match the smell: yoghurt or beer is a healthy starter at work and nothing needs doing; acetone or nail polish means it has run out of food and is making alcohol, so pour off any liquid on top and feed it twice a day until it smells of yoghurt again; vinegar or a sharp sourness is the same thing further along, so feed it more often than once a day and use a splash of it in a bake rather than a whole loaf.")); __kws.extend(vec![("avoid".to_string(), __data56()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_starter_mould()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_starter_keeping()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_starter_smell()]);
    fn __data57() -> Py { py_tuple(vec![py_str("sauce"), py_str("sauces"), py_str("custard"), py_str("hollandaise"), py_str("mayonnaise"), py_str("cheesecake"), py_str("cake"), py_str("cream sauce"), py_str("gravy"), py_str("curry"), py_str("recipe"), py_str("how do i make"), py_str("how to make"), py_str("freeze"), py_str("frozen"), py_str("fridge"), py_str("how long"), py_str("shelf life")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_soup_curdle"); }
    s_entry_soup_curdle({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("soup-curdle")); __args.push(py_tuple(vec![py_tuple(vec![py_str("soup"), py_str("soups"), py_str("broth"), py_str("bisque"), py_str("chowder"), py_str("minestrone"), py_str("soup pot"), py_str("pot of soup")]), py_tuple(vec![py_str("curdle"), py_str("curdles"), py_str("curdled"), py_str("curdling"), py_str("split"), py_str("splits"), py_str("splitting"), py_str("separated"), py_str("separates"), py_str("separating"), py_str("grainy"), py_str("grainy bits")])])); __args.push(py_str("A creamy or egg-thickened soup curdles when it gets too hot or too sharp: keep it under a simmer - small bubbles at the edge, never a rolling boil - and stir the cream, yogurt or egg in off the heat at the very end. Acid does the same, so add wine, lemon or tomatoes early and let them cook down before the dairy goes in, and a pinch of bicarbonate of soda in a tomato soup stops the split. If it has already gone, take it off the heat and blend it smooth with a little cold cream, or whisk in a cornflour slurry and warm it gently.")); __kws.extend(vec![("avoid".to_string(), __data57())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_soup_curdle()]);
    fn __data58() -> Py { py_tuple(vec![py_str("bake"), py_str("baking"), py_str("cake"), py_str("cakes"), py_str("biscuits"), py_str("cookies"), py_str("muffins"), py_str("scones"), py_str("sweet"), py_str("dessert"), py_str("pudding"), py_str("traybake"), py_str("recipe"), py_str("what can i bake")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("instead"), py_str("replace"), py_str("substitute"), py_str("swap"), py_str("swap out"), py_str("egg free cake"), py_str("dinner"), py_str("tea"), py_str("supper"), py_str("buy"), py_str("shop"), py_str("sainsbury"), py_str("tesco"), py_str("price"), py_str("cost"), py_str("calories"), py_str("vegan"), py_str("gluten"), py_str("sank"), py_str("sinking"), py_str("sunny border"), py_str("plant"), py_str("freeze"), py_str("keep"), py_str("cook"), py_str("dinner"), py_str("tea")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_no_eggs"); }
    s_entry_no_eggs({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("baking-without-eggs")); __args.push(py_tuple(vec![py_tuple(vec![py_str("no eggs"), py_str("without eggs"), py_str("out of eggs"), py_str("no egg"), py_str("egg free"), py_str("eggless"), py_str("eggless")]), __data58()])); __args.push(py_str("Plenty bakes without eggs: shortbread, flapjack, a fruit crumble, icebox biscuits and most traybakes need none at all. If a recipe wants an egg for binding, a tablespoon of ground flaxseed in three of water stands in for one, and a tablespoon of yogurt or half a mashed banana does the same job in a cake. Add a splash more milk, because egg is liquid too.")); __kws.extend(vec![("avoid".to_string(), __data59()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data60() -> Py { py_tuple(vec![py_str("bored"), py_str("boring"), py_str("bored of"), py_str("fed up"), py_str("sick of"), py_str("tired of"), py_str("every night"), py_str("every day"), py_str("same thing"), py_str("different"), py_str("something else"), py_str("ideas"), py_str("other ideas"), py_str("another way"), py_str("change"), py_str("not a roast"), py_str("no roast"), py_str("isnt a roast"), py_str("instead of a roast"), py_str("apart from a roast"), py_str("other than a roast"), py_str("besides a roast"), py_str("what can i make"), py_str("what can i cook"), py_str("what can i do"), py_str("what else can i make"), py_str("what else can i do"), py_str("something with chicken")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("was lovely"), py_str("was delicious"), py_str("came out"), py_str("how long"), py_str("temperature"), py_str("calories"), py_str("healthy"), py_str("buy"), py_str("price"), py_str("raw"), py_str("frozen"), py_str("defrost"), py_str("freeze"), py_str("bone"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("cooked"), py_str("cold chicken"), py_str("sandwich"), py_str("sandwiches"), py_str("salad"), py_str("salads"), py_str("shred"), py_str("shredded"), py_str("use up"), py_str("using up"), py_str("can i eat"), py_str("ok to eat"), py_str("okay to eat"), py_str("safe to eat"), py_str("good for"), py_str("is chicken"), py_str("how often"), py_str("how much"), py_str("too much"), py_str("should i eat"), py_str("is it bad")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_bored_of_chicken"); }
    s_entry_bored_of_chicken({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bored-of-chicken")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("chicken breast"), py_str("chicken breasts"), py_str("chicken thigh"), py_str("chicken thighs"), py_str("chicken wing"), py_str("chicken wings"), py_str("chicken drumsticks"), py_str("roast chicken"), py_str("whole chicken"), py_str("chicken pieces")]), __data60()])); __args.push(py_str("The fastest change is the cut and the heat rather than a new recipe: thighs roasted hot with paprika and lemon (200C for 35 to 40 minutes), breasts bashed flat and griddled in four minutes, wings salted and roasted at 200C for 45, or the whole bird pulled into tacos with lime and yogurt. After that it is the sauce that stops it being the same dinner - a traybake with chorizo and chickpeas, a satay with peanut butter and soy, or a tray of thighs marinated in yogurt and curry powder.")); __kws.extend(vec![("avoid".to_string(), __data61()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data62() -> Py { py_tuple(vec![py_str("how do i make bread rise"), py_str("how long to prove"), py_str("how long do i prove"), py_str("recipe"), py_str("how do i make bread"), py_str("stale"), py_str("mouldy"), py_str("keep"), py_str("freeze"), py_str("buy"), py_str("price"), py_str("gluten free"), py_str("no yeast"), py_str("without yeast"), py_str("sourdough"), py_str("sticky"), py_str("crumbly"), py_str("dense"), py_str("sank"), py_str("sunken"), py_str("pastry"), py_str("soggy in the middle"), py_str("wet in the middle"), py_str("doughy"), py_str("raw in the middle"), py_str("gummy"), py_str("wet inside"), py_str("stodgy"), py_str("not cooked in the middle"), py_str("not bake in the middle"), py_str("not cook in the middle"), py_str("collapse"), py_str("collapses"), py_str("collapsed"), py_str("collapsing"), py_str("caved"), py_str("deflated"), py_str("fell")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_bread_did_not_rise"); }
    s_entry_bread_did_not_rise({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bread-did-not-rise")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("dough"), py_str("bread dough"), py_str("sourdough")]), py_tuple(vec![py_str("not rise"), py_str("not risen"), py_str("did not rise"), py_str("hasnt risen"), py_str("hasn't risen"), py_str("hasnt proved"), py_str("hasn't proved"), py_str("why did"), py_str("why has"), py_str("why is"), py_str("what went wrong"), py_str("what happened")])])); __args.push(py_str("Four things stop a loaf rising, in the order worth checking. The yeast is dead - test it in warm water with a pinch of sugar and it should foam in ten minutes; water over about 45C, or a tin past its date, kills it. The water was too hot or too cold, so the dough never got going. The salt went in on top of the yeast and stalled it - keep them apart until the flour is in. Or it simply had not had long enough somewhere warm: doubling takes 1 to 2 hours at 25C and much longer in a cold kitchen. A dough that rose and then fell was left too long.")); __kws.extend(vec![("avoid".to_string(), __data62()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_no_eggs()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_bored_of_chicken()]);
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_bread_did_not_rise()]);
    fn __data63() -> Py { py_tuple(vec![py_str("score"), py_str("scored"), py_str("scoring"), py_str("scores"), py_str("slash"), py_str("slashed"), py_str("slashing"), py_str("slashes"), py_str("cut the top"), py_str("cut into the top"), py_str("snip the top"), py_str("top of the loaf"), py_str("top of the bread"), py_str("split at the side"), py_str("splits at the side"), py_str("burst at the side"), py_str("bursts at the side"), py_str("split down the side"), py_str("burst open"), py_str("bursts open"), py_str("split open"), py_str("splits open"), py_str("cracked open"), py_str("cracks open"), py_str("crack on top"), py_str("cracked on top"), py_str("split on top")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("did not rise"), py_str("not risen"), py_str("not rise"), py_str("dense"), py_str("crumbly"), py_str("sank"), py_str("collapsed"), py_str("stale"), py_str("mouldy"), py_str("freeze"), py_str("recipe"), py_str("buy"), py_str("price"), py_str("calories"), py_str("sourdough starter"), py_str("cake"), py_str("cakes"), py_str("pastry"), py_str("score a goal"), py_str("score points"), py_str("score in"), py_str("test score"), py_str("credit score"), py_str("music"), py_str("sheet music"), py_str("film"), py_str("game"), py_str("match")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_score_the_loaf"); }
    s_entry_score_the_loaf({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("score-the-loaf")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("loaves"), py_str("sourdough"), py_str("dough"), py_str("boule"), py_str("batard"), py_str("baguette"), py_str("rolls"), py_str("buns"), py_str("focaccia")]), __data63()])); __args.push(py_str("Yes - one slash about 1cm deep at a shallow angle is what decides where the loaf opens, so it rises where you cut instead of tearing wherever the crust is thinnest. Do it with a wet blade just before the loaf goes in, not before the final prove, or the cut closes up again. A tin loaf needs no slash, because the tin holds the sides and the top is free to rise; a round loaf or a long one on a tray wants one slash along it or a cross on top.")); __kws.extend(vec![("avoid".to_string(), __data64())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_score_the_loaf()]);
    fn __data65() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("made"), py_str("home"), py_str("at home"), py_str("homemade"), py_str("home made"), py_str("myself"), py_str("own"), py_str("from scratch"), py_str("recipe"), py_str("churn"), py_str("churning"), py_str("ferment"), py_str("fermenting"), py_str("starter"), py_str("culture")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.MAKE_AT_HOME"); }
    s_MAKE_AT_HOME(__data65());
    fn __data66() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("gone off"), py_str("rancid"), py_str("freeze"), py_str("frozen"), py_str("peanut"), py_str("peanuts"), py_str("peanut butter"), py_str("almond butter"), py_str("cashew"), py_str("nut butter"), py_str("substitute"), py_str("instead"), py_str("replace"), py_str("swap"), py_str("healthy"), py_str("healthier"), py_str("bad for you"), py_str("calories"), py_str("cholesterol"), py_str("dairy free"), py_str("vegan"), py_str("how much"), py_str("how many"), py_str("grams"), py_str("price"), py_str("cost"), py_str("cheap"), py_str("buy"), py_str("salted or unsalted"), py_str("lurpak"), py_str("brand"), py_str("shortbread"), py_str("cake"), py_str("pastry"), py_str("toast"), py_str("sandwich")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-butter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("butter"), py_str("buttermilk")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("cream"), py_str("double cream"), py_str("churn")]))])); __args.push(py_str("Churn double cream past whipped cream and it splits into butter and buttermilk: 300ml of double cream at room temperature, beaten in a food processor or shaken in a jar for 10 minutes, first goes thick, then grainy, then sloshes. Pour the buttermilk off, rinse the butter in ice-cold water and squeeze it dry with the back of a spoon, work in salt to taste, and it keeps 2 weeks in the fridge. The buttermilk is for scones or pancakes.")); __kws.extend(vec![("avoid".to_string(), __data66()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data67() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("still good"), py_str("use by"), py_str("go off"), py_str("gone off"), py_str("out of date"), py_str("melted"), py_str("melting"), py_str("soft"), py_str("scoop"), py_str("shop"), py_str("icy"), py_str("ice crystals"), py_str("crystals"), py_str("crystallised"), py_str("crystalised"), py_str("freezer burn"), py_str("grainy"), py_str("hard as a rock"), py_str("stops it going icy"), py_str("going icy"), py_str("went icy"), py_str("buy"), py_str("brand"), py_str("ben and jerrys"), py_str("healthy"), py_str("calories"), py_str("sugar"), py_str("dairy free"), py_str("vegan"), py_str("lactose"), py_str("milk free"), py_str("price"), py_str("cost"), py_str("vanilla ice cream"), py_str("strawberry ice cream"), py_str("which ice cream"), py_str("best ice cream")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-ice-cream")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ice cream"), py_str("icecream"), py_str("gelato"), py_str("sorbet")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("machine"), py_str("churn"), py_str("freezer"), py_str("without a machine"), py_str("no churn")]))])); __args.push(py_str("No machine needed: whip 300ml of double cream to soft peaks, fold in a 397g tin of condensed milk and whatever flavouring you like, and freeze it in a loaf tin for 4 hours, stirring it twice in the first hour. The creamier way is a custard - 4 yolks, 100g sugar and 500ml of milk and cream cooked to 80C - frozen in a tub and beaten every 30 minutes three times. Home-made ice cream wants eating within a fortnight.")); __kws.extend(vec![("avoid".to_string(), __data67()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data68() -> Py { py_tuple(vec![py_str("how long"), py_str("cook"), py_str("cooking"), py_str("boil"), py_str("al dente"), py_str("sauce"), py_str("bake"), py_str("baked"), py_str("lasagne"), py_str("shape"), py_str("shapes"), py_str("gluten free"), py_str("instead"), py_str("substitute"), py_str("keep"), py_str("leftover"), py_str("calories"), py_str("healthy"), py_str("dried"), py_str("packet"), py_str("brand"), py_str("barilla"), py_str("price"), py_str("cost"), py_str("how much pasta"), py_str("portion"), py_str("grams per person"), py_str("too much"), py_str("made too much"), py_str("make too much"), py_str("left over"), py_str("left over pasta")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-pasta")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("tagliatelle"), py_str("fettuccine"), py_str("ravioli"), py_str("pappardelle")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("dough"), py_str("flour"), py_str("egg"), py_str("eggs"), py_str("semolina"), py_str("fresh")]))])); __args.push(py_str("Fresh pasta is 100g of tipo 00 flour and one large egg per person, mixed to a stiff dough, kneaded 5 minutes and rested 30 minutes under a bowl, then rolled as thin as you can and cut. It cooks in 2 to 3 minutes in salted water, not the 10 a dried pasta takes. Semolina and water with no egg is the southern way, and it wants a rolling pin or a machine.")); __kws.extend(vec![("avoid".to_string(), __data68()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data69() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("gone off"), py_str("healthy"), py_str("calories"), py_str("protein"), py_str("instead"), py_str("substitute"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("philadelphia"), py_str("price"), py_str("cost"), py_str("cake"), py_str("cheesecake"), py_str("on toast"), py_str("sauce"), py_str("melt"), py_str("melted"), py_str("grated"), py_str("toastie"), py_str("sandwich"), py_str("board"), py_str("baked"), py_str("macaroni"), py_str("pizza"), py_str("topping")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-soft-cheese")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cottage cheese"), py_str("cream cheese"), py_str("soft cheese"), py_str("ricotta"), py_str("mascarpone"), py_str("quark"), py_str("paneer"), py_str("halloumi"), py_str("cheese")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("milk"), py_str("vinegar"), py_str("lemon"), py_str("rennet"), py_str("curdle"), py_str("curds")]))])); __args.push(py_str("Warm 2 litres of whole milk to 80C - steaming, not boiling - stir in 3 tablespoons of lemon juice or white vinegar, and it separates into curds and whey. Strain it through a muslin for 20 minutes for a soft curd to eat that day, four hours for a cottage cheese, or overnight with a weight on it for a paneer you can fry. Add a pinch of salt and a splash of the whey back at the end, and keep it 5 days.")); __kws.extend(vec![("avoid".to_string(), __data69()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data70() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("healthy"), py_str("good for you"), py_str("gut"), py_str("probiotic"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("price"), py_str("cost"), py_str("dairy free"), py_str("lactose"), py_str("calories"), py_str("where can i buy")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-kefir")); __args.push(py_tuple(vec![py_tuple(vec![py_str("kefir"), py_str("water kefir"), py_str("milk kefir")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("grains"), py_str("milk"), py_str("culture"), py_str("starter"), py_str("ferment")]))])); __args.push(py_str("Milk kefir is 1 to 2 tablespoons of live kefir grains in a litre of whole milk in a jar with a loose lid, left out of the fridge for 24 hours and then strained: the grains go back in fresh milk and the kefir goes in the fridge. The grains grow, so give the extra away. With no grains, 2 tablespoons of live yoghurt or buttermilk starts it off, though it stops after a few lots. It keeps a week.")); __kws.extend(vec![("avoid".to_string(), __data70()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data71() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("healthy"), py_str("sugar"), py_str("calories"), py_str("instead"), py_str("substitute"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("heinz"), py_str("price"), py_str("cost"), py_str("on chips"), py_str("with chips"), py_str("how much")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-ketchup")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ketchup"), py_str("tomato ketchup"), py_str("catsup")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("tomatoes"), py_str("tomato"), py_str("sauce"), py_str("sugar"), py_str("vinegar")]))])); __args.push(py_str("Simmer 1kg of tomatoes with an onion, 100g of sugar, 150ml of vinegar, a teaspoon of salt and a spice bag of cloves, peppercorns and a bay leaf for 45 minutes, then blend it and simmer it down until a spoon dragged across the pan leaves a line. Push it through a sieve and bottle it hot: it keeps a month in the fridge once opened. Tinned tomatoes and half the sugar make a thinner, sharper ketchup.")); __kws.extend(vec![("avoid".to_string(), __data71()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data72() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("healthy"), py_str("calories"), py_str("protein"), py_str("instead"), py_str("substitute"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("price"), py_str("cost"), py_str("sandwich"), py_str("toast"), py_str("allergy"), py_str("allergic"), py_str("reaction"), py_str("dogs"), py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("every day"), py_str("a day"), py_str("portion"), py_str("serving"), py_str("good for you"), py_str("good for me"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("toddlers"), py_str("my son"), py_str("my daughter"), py_str("weaning"), py_str("first foods"), py_str("first tastes"), py_str("highchair"), py_str("high chair")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-peanut-butter")); __args.push(py_tuple(vec![py_tuple(vec![py_str("peanut butter"), py_str("nut butter"), py_str("almond butter"), py_str("cashew butter"), py_str("peanuts")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("peanut"), py_str("peanuts"), py_str("nuts"), py_str("blitz"), py_str("roast")]))])); __args.push(py_str("Roast 300g of shelled peanuts at 180C (160C fan) for 10 minutes until they smell nutty, then blitz them in a food processor for 3 to 5 minutes, scraping down the sides: first it is crumbs, then a ball, then suddenly a smooth butter. Add a pinch of salt and a teaspoon of honey, and a spoon of oil if you want it looser. It keeps a month in a jar.")); __kws.extend(vec![("avoid".to_string(), __data72()), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_MAKE_AT_HOME())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data73() -> Py { py_tuple(vec![py_str("separat"), py_str("separates"), py_str("separating"), py_str("separated"), py_str("oily"), py_str("oil on top"), py_str("stir"), py_str("stirring"), py_str("runny"), py_str("liquid"), py_str("grainy"), py_str("hard"), py_str("stiff"), py_str("fridge"), py_str("upside down"), py_str("store"), py_str("storing"), py_str("storage"), py_str("keep"), py_str("keeping"), py_str("keeps")]) }
    fn __data74() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("blitz"), py_str("roast"), py_str("roasted"), py_str("homemade"), py_str("calories"), py_str("healthy"), py_str("allergy"), py_str("allergic"), py_str("baby"), py_str("sandwich"), py_str("toast"), py_str("brand"), py_str("best"), py_str("buy"), py_str("shop"), py_str("cost"), py_str("price"), py_str("cookie"), py_str("cake"), py_str("biscuit")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("nut-butter-separates")); __args.push(py_tuple(vec![py_tuple(vec![py_str("peanut butter"), py_str("nut butter"), py_str("almond butter"), py_str("cashew butter")]), __data73()])); __args.push(py_str("Natural nut butter separates because there is nothing in it but nuts: the oil rises and the solids sink. Stir it back in with a knife, from the bottom of the jar upwards, and it comes together in a minute or two. A jar that has set hard mixes more easily if it stands upside down for a day first, and standing it in a bowl of warm water for five minutes softens it without cooking it. An opened jar keeps a few months in the cupboard and longer in the fridge, though cold makes it stiffer to spread.")); __kws.extend(vec![("avoid".to_string(), __data74()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data75() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("melted"), py_str("melting"), py_str("shop"), py_str("buy"), py_str("brand"), py_str("healthy"), py_str("sugar"), py_str("calories"), py_str("price"), py_str("cost"), py_str("toddler"), py_str("baby"), py_str("choking")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-ice-lollies")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ice lollies"), py_str("ice lolly"), py_str("popsicles"), py_str("popsicle"), py_str("ice pops"), py_str("lolly"), py_str("lollies")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("juice"), py_str("mould"), py_str("moulds"), py_str("molds"), py_str("sticks"), py_str("freezer"), py_str("fruit")]))])); __args.push(py_str("Blend or mash soft fruit with a little sugar and lemon - strawberries, mango, berries, or just orange juice - and pour it into moulds or paper cups with a stick in, leaving a centimetre at the top because water expands. Four to six hours in the freezer. A splash of yoghurt or milk makes them creamier, and a spoon of vodka stops them going rock hard.")); __kws.extend(vec![("avoid".to_string(), __data75()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data76() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("healthy"), py_str("calories"), py_str("sugar"), py_str("instead"), py_str("substitute"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("heinz"), py_str("price"), py_str("cost"), py_str("on toast"), py_str("how long do i cook"), py_str("tin of")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-baked-beans")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baked beans"), py_str("beans on toast"), py_str("haricot beans"), py_str("cannellini beans")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("sugar"), py_str("haricot"), py_str("dried")]))])); __args.push(py_str("Soak 250g of dried haricot beans overnight, boil them hard for 10 minutes then simmer for an hour until soft (or start from two tins). Simmer a tin of tomatoes with an onion, a tablespoon of black treacle or brown sugar, a teaspoon of mustard, a splash of vinegar and a pinch of salt for 20 minutes, stir the beans in and bake at 160C (140C fan) for 40 minutes. It keeps 4 days and freezes.")); __kws.extend(vec![("avoid".to_string(), __data76()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data77() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("healthy"), py_str("calories"), py_str("protein"), py_str("instead"), py_str("substitute"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("price"), py_str("cost"), py_str("how long do i cook"), py_str("how do i cook"), py_str("in the oven"), py_str("air fryer"), py_str("barbecue"), py_str("bangers")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-sausages")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sausages"), py_str("sausage meat"), py_str("sausagemeat")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("mince"), py_str("meat"), py_str("pork"), py_str("casings"), py_str("skins"), py_str("seasoning")]))])); __args.push(py_str("Sausages are 70/30 lean pork to fat, minced coarse and kept cold: for 1kg of shoulder add 18g of salt, a teaspoon of pepper, a teaspoon of sage and 100ml of iced water, mix it until it is sticky, and stuff it into soaked hog casings - or shape it into patties, which needs no casing at all. Fry a small piece to taste for salt before you stuff the rest. Eat them within 3 days or freeze them.")); __kws.extend(vec![("avoid".to_string(), __data77()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data78() -> Py { py_tuple(vec![py_str("ferment"), py_str("fermenting"), py_str("fermented"), py_str("incubate"), py_str("incubating"), py_str("incubator"), py_str("culture"), py_str("starter"), py_str("warm spot"), py_str("airing cupboard"), py_str("what temperature"), py_str("how warm"), py_str("set it"), py_str("how long do i leave"), py_str("how long to ferment"), py_str("didnt set"), py_str("did not set"), py_str("wont set")]) }
    fn __data79() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("use by"), py_str("past the date"), py_str("out of date"), py_str("gone off"), py_str("mould"), py_str("cheap"), py_str("buy"), py_str("brand"), py_str("calories"), py_str("protein"), py_str("dairy free"), py_str("lactose"), py_str("my starter"), py_str("the starter"), py_str("starter smells"), py_str("starter has"), py_str("starter is"), py_str("starter went"), py_str("starter gone")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("yoghurt-fermenting")); __args.push(py_tuple(vec![py_tuple(vec![py_str("yoghurt"), py_str("yogurt"), py_str("yoghurt maker"), py_str("yoghurt pot")]), __data78()])); __args.push(py_str("Keep it at 43C for 8 to 12 hours: a yoghurt maker does it, an airing cupboard or an oven switched off with the light on is close enough, and a flask of it in a warm water bath works too. Below 40C it takes longer and tastes sourer, above 46C the culture dies and it stays milk. It is set when it pulls away from the side of the jar - then chill it for 4 hours before eating, which is when it firms up properly.")); __kws.extend(vec![("avoid".to_string(), __data79()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data80() -> Py { py_tuple(vec![py_str("roast"), py_str("roasts"), py_str("roasted"), py_str("roasting"), py_str("parboil"), py_str("parboiled"), py_str("parboiling"), py_str("sunday lunch"), py_str("yorkshire"), py_str("buy"), py_str("buying"), py_str("shop"), py_str("brand"), py_str("packet"), py_str("walkers"), py_str("which"), py_str("best"), py_str("healthy"), py_str("healthier"), py_str("bad for you"), py_str("calories"), py_str("sugar"), py_str("salt in"), py_str("how much salt"), py_str("baby"), py_str("toddler"), py_str("kids"), py_str("child"), py_str("keto"), py_str("air fryer"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("pastry"), py_str("crumble"), py_str("apple"), py_str("shop bought"), py_str("multipack"), py_str("price"), py_str("cost"), py_str("cheap"), py_str("chip"), py_str("chips"), py_str("fries"), py_str("french fries"), py_str("oven chips"), py_str("chip shop"), py_str("roast"), py_str("roasted"), py_str("roasting"), py_str("roasties"), py_str("parboil"), py_str("parboiled"), py_str("sunday dinner")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-crisps")); __args.push(py_tuple(vec![py_tuple(vec![py_str("crisps"), py_str("potato crisps"), py_str("crisp")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("sliced"), py_str("slices"), py_str("oven"), py_str("fry"), py_str("frying"), py_str("salted"), py_str("salt"), py_str("crunchy"), py_str("crispy")]))])); __args.push(py_str("Slice floury potatoes about 2mm thin - a mandoline, or a peeler does one at a time - rinse the starch off in cold water, then dry the slices properly on a tea towel: a wet slice steams instead of crisping. Fry at 160C for 3 minutes, until the bubbling dies down, or toss them in a tablespoon of oil and bake at 200C for 15 to 20 minutes, turning once. Salt the moment they come out and cool them on kitchen paper. Dry slices and a steady temperature are the whole trick.")); __kws.extend(vec![("avoid".to_string(), __data80()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data81() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("healthy"), py_str("calories"), py_str("sugar"), py_str("protein"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("price"), py_str("cost"), py_str("which"), py_str("best"), py_str("toddler"), py_str("kids")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-granola")); __args.push(py_tuple(vec![py_tuple(vec![py_str("granola"), py_str("muesli"), py_str("toasted oats"), py_str("breakfast cereal")]), op_add(&g_MAKE_AT_HOME(), &py_tuple(vec![py_str("oats"), py_str("honey"), py_str("syrup"), py_str("nuts"), py_str("oven"), py_str("bake"), py_str("toast")]))])); __args.push(py_str("Toss 300g of oats with 100g of nuts and seeds, then stir in 3 tablespoons of oil, 3 of honey or maple syrup and a pinch of salt, and spread it thin on a lined tray. Bake at 160C (140C fan) for 20 to 25 minutes, stirring twice, until it is golden - it crisps as it cools. Add dried fruit after baking, or it burns. It keeps a month in a jar; muesli is the same mix untoasted.")); __kws.extend(vec![("avoid".to_string(), __data81()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data82() -> Py { py_tuple(vec![py_str("calories"), py_str("how many calories"), py_str("healthy"), py_str("diet"), py_str("buy"), py_str("shop"), py_str("supermarket"), py_str("brand"), py_str("price"), py_str("cost"), py_str("gluten"), py_str("vegan"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("thermomix"), py_str("air fryer"), py_str("biscuit base"), py_str("cheesecake"), py_str("crumbly"), py_str("crumbs"), py_str("store"), py_str("storing"), py_str("keep"), py_str("keeps"), py_str("sealed"), py_str("freeze"), py_str("frozen"), py_str("food processor"), py_str("processor"), py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how long"), py_str("oven")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("shortbread-amounts")); __args.push(py_tuple(vec![py_tuple(vec![py_str("shortbread"), py_str("shortbreads"), py_str("shortbread fingers"), py_str("millionaires shortbread")]), py_tuple(vec![py_str("butter"), py_str("how much"), py_str("how many"), py_str("much"), py_str("ratio"), py_str("ratios"), py_str("flour"), py_str("sugar"), py_str("margarine")])])); __args.push(py_str("Shortbread is one part sugar, two parts butter and three parts flour: for a 20cm round that is 60g caster sugar, 120g butter and 180g plain flour. Cream the butter and sugar, work in the flour without kneading it, press it into the tin, prick it, and bake at 150C (130C fan) for 35 to 40 minutes until it is pale gold rather than brown. It firms as it cools, so cut it into fingers while it is still warm.")); __kws.extend(vec![("avoid".to_string(), __data82()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data83() -> Py { py_tuple(vec![py_str("apple jelly"), py_str("mint jelly"), py_str("redcurrant jelly"), py_str("crab apple jelly"), py_str("bramble jelly"), py_str("blackberry jelly"), py_str("quince jelly"), py_str("rowan jelly"), py_str("rosemary jelly"), py_str("damson jelly"), py_str("fruit jelly"), py_str("jelly"), py_str("jellies")]) }
    fn __data84() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("homemade"), py_str("from scratch"), py_str("how do i"), py_str("how to"), py_str("recipe"), py_str("apples"), py_str("apple"), py_str("mint"), py_str("redcurrants"), py_str("crab apples"), py_str("brambles"), py_str("blackberries"), py_str("quinces"), py_str("damsons"), py_str("strain"), py_str("muslin"), py_str("juice")]) }
    fn __data85() -> Py { py_tuple(vec![py_str("didnt set"), py_str("did not set"), py_str("wont set"), py_str("won't set"), py_str("not set"), py_str("runny"), py_str("liquid"), py_str("how do i set"), py_str("how to set"), py_str("set a jelly"), py_str("set the jelly"), py_str("set it"), py_str("setting point"), py_str("when is it set"), py_str("is it set"), py_str("has it set"), py_str("how do i know when"), py_str("know when a jelly"), py_str("wobbly"), py_str("how long does"), py_str("keeps"), py_str("keeping"), py_str("out of date"), py_str("gone off"), py_str("mould"), py_str("calories"), py_str("sugar"), py_str("healthy"), py_str("buy"), py_str("shop"), py_str("brand"), py_str("price"), py_str("cost"), py_str("rowntrees"), py_str("hartley"), py_str("jam"), py_str("marmalade"), py_str("chutney"), py_str("crumble"), py_str("pie"), py_str("cake")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("make-fruit-jelly")); __args.push(py_tuple(vec![__data83(), __data84()])); __args.push(py_str("Cook the fruit with just enough water to stop it catching - apples and blackberries 20 minutes, redcurrants 10, mint as a strong vinegar infusion - then strain it through a muslin or a clean tea towel hung over a bowl and leave it overnight: squeeze it and the jelly goes cloudy. For every 600ml of juice weigh out 450g of sugar, warm them together until the sugar has gone, then boil hard to 104C (or until a drop on a cold plate wrinkles) and pot it hot. It sets in a day.")); __kws.extend(vec![("avoid".to_string(), __data85()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._CHILD_DRANK_IT"); }
    s__CHILD_DRANK_IT(stdlib_call("re", "compile", &[py_str("\\b(?:baby|babies|infant|toddler|toddlers|child|children|childs|kid|kids|boy|girl|son|daughter|newborn|little one|grandchild|grandson|granddaughter)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data86() -> Py { py_tuple(vec![py_str("drank"), py_str("drunk"), py_str("drank some"), py_str("swallowed"), py_str("sipped"), py_str("had some"), py_str("has had some"), py_str("gave him"), py_str("gave her"), py_str("gave my"), py_str("gave the"), py_str("off milk"), py_str("sour milk"), py_str("turned milk"), py_str("milk that was off"), py_str("milk that had gone off"), py_str("milk was off"), py_str("milk had gone off"), py_str("bad milk"), py_str("milk that turned"), py_str("curdled milk")]) }
    fn __data87() -> Py { py_tuple(vec![py_str("how do i know"), py_str("how can i tell"), py_str("how long"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("use by"), py_str("date on the"), py_str("freeze"), py_str("frozen"), py_str("buy"), py_str("brand"), py_str("price"), py_str("cost"), py_str("calories"), py_str("protein"), py_str("lactose"), py_str("allergy"), py_str("allergic"), py_str("recipe"), py_str("sauce"), py_str("cheese"), py_str("yoghurt"), py_str("yogurt"), py_str("kefir"), py_str("breastfeeding"), py_str("formula"), py_str("cows milk"), py_str("which milk"), py_str("what milk"), py_str("oat milk"), py_str("soya milk"), py_str("almond milk")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("milk-that-turned-was-drunk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("milk")]), __data86()])); __args.push(py_str("Sour milk is curdled and unpleasant, not poisonous: it will not give a child the food poisoning that undercooked meat does, and a mouthful or a cup is very unlikely to do anything. Offer water, and watch for vomiting or diarrhoea over the next few hours. Ring 111 if the child is under one, cannot keep water down, or seems unwell rather than just cross about it. Then bin the bottle - the date is about quality, not safety.")); __kws.extend(vec![("avoid".to_string(), __data87()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g__CHILD_DRANK_IT(), "search", &[op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "))])]).unwrap_or_else(|e| exc::raise(e)) })), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data88() -> Py { py_tuple(vec![py_str("already burnt"), py_str("what can i do"), py_str("what do i do"), py_str("do with"), py_str("can i still eat"), py_str("still eat"), py_str("throw"), py_str("bin"), py_str("rescue"), py_str("save it"), py_str("burnt bits"), py_str("stick"), py_str("sticks"), py_str("sticking"), py_str("sticky"), py_str("stick to the pan"), py_str("catch"), py_str("catches"), py_str("crunchy"), py_str("crunch"), py_str("hard"), py_str("undercooked"), py_str("underdone"), py_str("raw"), py_str("mushy"), py_str("stodgy"), py_str("soggy"), py_str("wet"), py_str("dry"), py_str("slimy"), py_str("gluey"), py_str("rice pudding"), py_str("fried rice"), py_str("risotto"), py_str("paella"), py_str("sushi"), py_str("rice cooker"), py_str("how long"), py_str("calories"), py_str("healthy"), py_str("freeze"), py_str("reheat"), py_str("leftover rice"), py_str("buy"), py_str("price"), py_str("grow")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_rice_burnt"); }
    s_entry_rice_burnt({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rice-keeps-burning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("long grain"), py_str("pilau"), py_str("risotto rice"), py_str("brown rice")]), py_tuple(vec![py_str("why"), py_str("keep burning"), py_str("keeps burning"), py_str("always burn"), py_str("always burning"), py_str("burning"), py_str("burns"), py_str("burn"), py_str("stop it burning"), py_str("stop burning"), py_str("prevent"), py_str("what am i doing wrong")])])); __args.push(py_str("Rice burns when there is no water left under it: the heat is too high, so the pan dries out before the grain is cooked and the bottom scorches. One mug of rice to one and a half of water, bring it to the boil with the lid on, then turn it down to the lowest ring for 10 to 12 minutes without stirring - stirring rubs the starch off and makes it catch. A heavy pan and a lid that fits are most of the fix, and 5 minutes off the heat at the end finishes it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("why"), py_str("keep"), py_str("keeps"), py_str("always"), py_str("stop"), py_str("prevent"), py_str("wrong")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data88())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_rice_burnt()]);
    fn __data89() -> Py { py_tuple(vec![py_str("refreeze"), py_str("re freeze"), py_str("re-freeze"), py_str("again"), py_str("twice"), py_str("already"), py_str("open"), py_str("opened"), py_str("bag"), py_str("packet"), py_str("pack"), py_str("opened bag"), py_str("half used"), py_str("half bag"), py_str("left out")]) }
    fn __data90() -> Py { py_tuple(vec![py_str("mashed"), py_str("milk"), py_str("breast milk"), py_str("cakes"), py_str("cake"), py_str("bread"), py_str("meat"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("fish"), py_str("rice"), py_str("soup"), py_str("stock"), py_str("banana"), py_str("bananas"), py_str("avocado"), py_str("eggs"), py_str("potatoes"), py_str("cooked"), py_str("leftover"), py_str("leftovers"), py_str("put the heating"), py_str("pipes"), py_str("burst")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("freeze-an-open-bag")); __args.push(py_tuple(vec![py_tuple(vec![py_str("freeze"), py_str("freezing"), py_str("frozen"), py_str("freezer"), py_str("thaw"), py_str("thawed"), py_str("thawing"), py_str("defrost"), py_str("defrosted"), py_str("defrosting")]), __data89()])); __args.push(py_str("An open bag of frozen veg goes straight back in the freezer: press the air out, clip it, and use it within a month or two before it dries out or picks up ice. Re-freezing is the one with a rule: food that has fully thawed can be cooked and then frozen again, but not frozen raw a second time, because the thaw is when bacteria multiply and freezing does not kill them. Frozen peas that have half thawed in a warm kitchen are still peas - cook them now rather than put them back.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("refreeze"), py_str("re freeze"), py_str("re-freeze"), py_str("again"), py_str("twice"), py_str("already"), py_str("open"), py_str("opened"), py_str("bag"), py_str("packet")])), ("avoid".to_string(), __data90()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data91() -> Py { py_tuple(vec![py_str("how long"), py_str("oven"), py_str("temperature"), py_str("calories"), py_str("healthy"), py_str("buy"), py_str("price"), py_str("keep"), py_str("leftover"), py_str("leftovers"), py_str("freeze"), py_str("frozen"), py_str("reheat"), py_str("mash"), py_str("mashed"), py_str("chips"), py_str("wedges"), py_str("soup"), py_str("peel"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("plants"), py_str("dog"), py_str("dogs"), py_str("sprout"), py_str("sprouts"), py_str("raw"), py_str("boil"), py_str("boiling"), py_str("microwave"), py_str("salad"), py_str("herb"), py_str("herbs"), py_str("spice"), py_str("spices"), py_str("seasoning"), py_str("rosemary"), py_str("thyme"), py_str("paprika"), py_str("cumin"), py_str("salt and pepper")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_potato_toppings"); }
    s_entry_potato_toppings({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("baked-potato-toppings")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baked potato"), py_str("baked potatoes"), py_str("jacket potato"), py_str("jacket potatoes"), py_str("potato"), py_str("potatoes")]), py_tuple(vec![py_str("put on"), py_str("topping"), py_str("toppings"), py_str("top it"), py_str("fill"), py_str("filling"), py_str("fillings"), py_str("what can i put"), py_str("what do you put"), py_str("what should i put"), py_str("what goes on")])])); __args.push(g_JACKET_FILL()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("put on"), py_str("topping"), py_str("toppings"), py_str("fill"), py_str("filling"), py_str("fillings"), py_str("goes on"), py_str("what can i put"), py_str("what do you put"), py_str("what should i put")])), ("avoid".to_string(), __data91())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_potato_toppings()]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._TIN_AREA"); }
    s__TIN_AREA(py_dict(vec![(py_str("15cm"), py_str("about half")), (py_str("18cm"), py_str("about a third less than a 20cm")), (py_str("20cm"), py_str("the recipe's own")), (py_str("23cm"), py_str("about a third more")), (py_str("25cm"), py_str("about half again"))]));
    fn __data92() -> Py { py_tuple(vec![py_str("double the recipe"), py_str("double a recipe"), py_str("double this recipe"), py_str("doubling the recipe"), py_str("doubling a recipe"), py_str("doubling"), py_str("doubled the recipe"), py_str("double batch"), py_str("double the batch"), py_str("two batches"), py_str("twice as much"), py_str("make twice"), py_str("twice the recipe"), py_str("halve"), py_str("halving"), py_str("half the recipe"), py_str("half a recipe"), py_str("half this recipe"), py_str("half batch"), py_str("smaller batch"), py_str("bigger tin"), py_str("larger tin"), py_str("smaller tin"), py_str("different tin"), py_str("tin size"), py_str("bigger cake"), py_str("bigger dish"), py_str("smaller dish"), py_str("scale up"), py_str("scale down"), py_str("scale the recipe"), py_str("two at once"), py_str("cook two at once"), py_str("two at the same time"), py_str("cook two at the same time"), py_str("make two at once"), py_str("bake two at once"), py_str("two in the oven at once")]) }
    fn __data93() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("batch"), py_str("mixture"), py_str("batter"), py_str("dough"), py_str("ingredients"), py_str("quantities"), py_str("amount"), py_str("amounts"), py_str("tin"), py_str("tins"), py_str("cook"), py_str("cooking"), py_str("bake"), py_str("baking"), py_str("oven"), py_str("time"), py_str("longer"), py_str("freeze"), py_str("portion"), py_str("portions")]) }
    fn __data94() -> Py { py_tuple(vec![py_str("does"), py_str("do i"), py_str("can i"), py_str("should i"), py_str("how long"), py_str("how much"), py_str("will"), py_str("if i"), py_str("is it"), py_str("what"), py_str("longer"), py_str("time"), py_str("quantities"), py_str("i have"), py_str("ive got"), py_str("i've got"), py_str("i want"), py_str("i need"), py_str("i only"), py_str("im making"), py_str("i'm making")]) }
    fn __data95() -> Py { py_tuple(vec![py_str("glazing"), py_str("glazed"), py_str("bed"), py_str("garage"), py_str("vision"), py_str("check"), py_str("checked"), py_str("dose"), py_str("doses"), py_str("tablets"), py_str("medicine"), py_str("paracetamol"), py_str("ibuprofen"), py_str("antibiotic"), py_str("insulin"), py_str("breast"), py_str("formula"), py_str("baby"), py_str("toddler"), py_str("dog"), py_str("cat"), py_str("air fryer"), py_str("slow cooker"), py_str("microwave"), py_str("yorkshire"), py_str("bread"), py_str("sourdough"), py_str("starter"), py_str("pizza"), py_str("roast potatoes"), py_str("turkey"), py_str("chicken"), py_str("chickens"), py_str("bird"), py_str("birds"), py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("tray"), py_str("trays"), py_str("tin of"), py_str("dishes"), py_str("things"), py_str("christmas"), py_str("calories"), py_str("how many calories"), py_str("protein"), py_str("price"), py_str("cheaper"), py_str("buy"), py_str("supermarket"), py_str("pack"), py_str("keep"), py_str("keeps"), py_str("reheat"), py_str("leftover"), py_str("leftovers"), py_str("double cream"), py_str("single cream"), py_str("soured cream"), py_str("creme fraiche"), py_str("whipping cream"), py_str("extra thick cream"), py_str("double cream in"), py_str("a pot of cream"), py_str("cream cheese")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("recipe-amount")); __args.push(py_tuple(vec![__data92(), __data93()])); __args.push(py_func("_recipe_amount", |args: &[Py]| _recipe_amount(args))); __kws.extend(vec![("ask".to_string(), __data94()), ("avoid".to_string(), __data95()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data96() -> Py { py_tuple(vec![py_str("how long"), py_str("time"), py_str("timing"), py_str("timetable"), py_str("minutes"), py_str("oven"), py_str("temperature"), py_str("gas mark"), py_str("cook"), py_str("cooking"), py_str("done"), py_str("kilo"), py_str("kilos"), py_str("kg"), py_str("pound"), py_str("pounds"), py_str("lbs"), py_str("chicken"), py_str("turkey"), py_str("duck"), py_str("goose"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("gammon"), py_str("ham"), py_str("brisket"), py_str("shoulder"), py_str("leg of"), py_str("joint of"), py_str("mince"), py_str("slow cooker"), py_str("air fryer"), py_str("my joint"), py_str("joints hurt"), py_str("joint pain"), py_str("knee"), py_str("knees"), py_str("hip"), py_str("hips"), py_str("lift"), py_str("lifting"), py_str("dumbbell"), py_str("dumbbells"), py_str("barbell"), py_str("gym"), py_str("workout"), py_str("exercise"), py_str("squat"), py_str("squats"), py_str("press")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_joint_weight"); }
    s_entry_joint_weight({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("joint-weight")); __args.push(py_tuple(vec![py_tuple(vec![py_str("joint"), py_str("joints"), py_str("roast")]), py_tuple(vec![py_str("what weight"), py_str("weight"), py_str("how heavy"), py_str("how big"), py_str("how much"), py_str("size"), py_str("per person"), py_str("each"), py_str("each person"), py_str("per head")])])); __args.push(py_func("_roast_for_a_crowd", |args: &[Py]| _roast_for_a_crowd(args))); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("weight"), py_str("what weight"), py_str("how heavy"), py_str("how big"), py_str("how much"), py_str("size"), py_str("per person"), py_str("per head"), py_str("each")])), ("avoid".to_string(), __data96())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_joint_weight()]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._PAN_THING"); }
    s__PAN_THING(py_tuple(vec![py_str("pan"), py_str("pans"), py_str("griddle"), py_str("griddle pan"), py_str("skillet"), py_str("frying pan"), py_str("wok"), py_str("saucepan"), py_str("non stick pan"), py_str("pan base"), py_str("flat pan"), py_str("cast iron pan")]));
    fn __data97() -> Py { py_tuple(vec![py_str("hot"), py_str("hot enough"), py_str("properly hot"), py_str("up to heat"), py_str("ready"), py_str("heated"), py_str("how do i know"), py_str("how can i tell"), py_str("how do you know"), py_str("when is it"), py_str("when is the"), py_str("how long to heat"), py_str("how long does it take to heat"), py_str("warm enough"), py_str("smoking hot")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._PAN_IS_HOT_ASK"); }
    s__PAN_IS_HOT_ASK(__data97());
    fn __data98() -> Py { py_tuple(vec![py_str("oil"), py_str("oils"), py_str("butter"), py_str("fat"), py_str("lard"), py_str("ghee"), py_str("dripping"), py_str("smoke"), py_str("smoking"), py_str("smoke point"), py_str("season"), py_str("seasoning"), py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washed"), py_str("burnt"), py_str("stuck"), py_str("stick"), py_str("rust"), py_str("oven"), py_str("temperature of the oven"), py_str("gas mark"), py_str("air fryer"), py_str("chicken"), py_str("steak"), py_str("fish"), py_str("egg"), py_str("eggs"), py_str("pancake"), py_str("bacon"), py_str("onion"), py_str("bread"), py_str("buy"), py_str("price"), py_str("which pan"), py_str("what pan"), py_str("best pan"), py_str("induction"), py_str("hob"), py_str("hot plate"), py_str("hotplate"), py_str("barbecue"), py_str("bbq"), py_str("grill"), py_str("pan is hot"), py_str("wok is hot"), py_str("test the pan heat"), py_str("worktop"), py_str("worktops"), py_str("counter"), py_str("countertop"), py_str("work surface"), py_str("worksurface"), py_str("benchtop"), py_str("the table"), py_str("wooden table"), py_str("wood table"), py_str("the side"), py_str("on the side"), py_str("chopping board"), py_str("wooden board"), py_str("bread board"), py_str("trivet"), py_str("granite"), py_str("quartz"), py_str("marble"), py_str("laminate"), py_str("glass table"), py_str("stone worktop"), py_str("mark"), py_str("marks"), py_str("marked"), py_str("scorch"), py_str("scorched"), py_str("scorching"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("white mark"), py_str("burn mark"), py_str("burnt the"), py_str("melted the"), py_str("cold water"), py_str("in water"), py_str("in the sink"), py_str("dunk"), py_str("dunking"), py_str("quench"), py_str("warp"), py_str("warped"), py_str("warping"), py_str("crack"), py_str("cracks"), py_str("cracked"), py_str("cool down"), py_str("cool the pan"), py_str("soak"), py_str("soaking")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pan-heat-without-a-fat")); __args.push(py_tuple(vec![g__PAN_THING(), g__PAN_IS_HOT_ASK()])); __args.push(py_str("Three things, and any one of them is enough. Hold your hand an inch above the surface: it should feel properly hot after about two seconds, not straight away. Flick a few drops of water in and they should bead and skate across the pan rather than sit there and hiss. And the whole base should look faintly shimmering rather than wet. Give a heavy pan three or four minutes on a medium heat and a thin one two, and let it heat before you turn it up.")); __kws.extend(vec![("avoid".to_string(), __data98()), ("bias".to_string(), Py::Int(5))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._ORDER_ASK"); }
    s__ORDER_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:pan|pans|wok|griddle|skillet|frying pan)\\b[^.?!]{0,24}\\bfirst\\b|\\bfirst\\b[^.?!]{0,24}\\b(?:pan|oil|butter|fat)\\b|\\bwhich\\s+(?:one\\s+)?first\\b|\\bpan\\s+first\\b|\\boil\\s+first\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data99() -> Py { py_tuple(vec![py_str("smoke"), py_str("smoking"), py_str("smoke point"), py_str("keep"), py_str("keeps"), py_str("store"), py_str("storing"), py_str("reuse"), py_str("reusing"), py_str("throw away"), py_str("get rid"), py_str("dispose"), py_str("drain"), py_str("oven"), py_str("air fryer"), py_str("first time"), py_str("my first"), py_str("first go"), py_str("first aid"), py_str("first birthday"), py_str("panini")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pan-then-oil")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pan"), py_str("pans"), py_str("wok"), py_str("griddle"), py_str("skillet"), py_str("frying pan"), py_str("hot pan"), py_str("cold pan")]), py_tuple(vec![py_str("first"), py_str("which first"), py_str("pan first"), py_str("oil first"), py_str("butter first"), py_str("one first"), py_str("heat the pan"), py_str("heat the oil"), py_str("hot pan")])])); __args.push(py_str("Pan first, then the oil. A dry pan heats evenly and you can judge its heat with your hand or a drop of water, and oil added to a pan that is already hot shimmers and moves at once instead of sitting in a cold pool. The exception is a marinade, a rub or a cold steak straight from the fridge, which goes into the hot oil rather than the bare pan.")); __kws.extend(vec![("avoid".to_string(), __data99()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); call_builtin("bool", &[mcall(&g__ORDER_ASK(), "search", &[op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "))])]).unwrap_or_else(|e| exc::raise(e)) })), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data100() -> Py { py_tuple(vec![py_str("rosemary"), py_str("thyme"), py_str("sage"), py_str("bay"), py_str("bay leaf"), py_str("bay leaves"), py_str("oregano"), py_str("mint"), py_str("basil"), py_str("coriander"), py_str("cilantro"), py_str("parsley"), py_str("dill"), py_str("chives"), py_str("tarragon"), py_str("herb"), py_str("herbs")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._HERBS_GROWING"); }
    s__HERBS_GROWING(__data100());
    fn __data101() -> Py { py_tuple(vec![py_str("plant"), py_str("plants"), py_str("bush"), py_str("bushes"), py_str("tree"), py_str("pot"), py_str("pots"), py_str("flowerpot"), py_str("herb pot"), py_str("patio"), py_str("window box"), py_str("garden"), py_str("allotment"), py_str("sprouting"), py_str("come back"), py_str("coming back"), py_str("gone mad"), py_str("running riot"), py_str("taken over")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._HERB_PLANT_WORDS"); }
    s__HERB_PLANT_WORDS(__data101());
    fn __data102() -> Py { py_tuple(vec![py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("use"), py_str("uses"), py_str("using"), py_str("do with"), py_str("make"), py_str("made"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("eat"), py_str("eating"), py_str("salad"), py_str("sauce"), py_str("pesto"), py_str("stuffing"), py_str("roast"), py_str("roasting"), py_str("grill"), py_str("grilling"), py_str("bake"), py_str("baking")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._HERB_USE_ASK"); }
    s__HERB_USE_ASK(__data102());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._HERB_SURPLUS_WORDS"); }
    s__HERB_SURPLUS_WORDS(py_tuple(vec![py_str("too much"), py_str("too many"), py_str("lots of"), py_str("loads of"), py_str("a lot of"), py_str("so much"), py_str("plenty of"), py_str("glut"), py_str("glut of"), py_str("overgrown"), py_str("surplus")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._WOODY_HERB"); }
    s__WOODY_HERB(call_builtin("frozenset", &[py_tuple(vec![py_str("rosemary"), py_str("thyme"), py_str("sage"), py_str("bay"), py_str("oregano")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._HERB_OWNED"); }
    s__HERB_OWNED(stdlib_call("re", "compile", &[py_str("\\b(?:have|has|having|got|there's|there is|bought|given|gave)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data103() -> Py { py_tuple(vec![py_str("grow"), py_str("grows"), py_str("growing"), py_str("grew"), py_str("propagate"), py_str("propagating"), py_str("seed"), py_str("seeds"), py_str("sow"), py_str("sowing"), py_str("cutting"), py_str("cuttings"), py_str("repot"), py_str("repotting"), py_str("potting"), py_str("wilt"), py_str("wilting"), py_str("wilts"), py_str("yellow"), py_str("yellowing"), py_str("brown"), py_str("dying"), py_str("died"), py_str("dead"), py_str("die"), py_str("prune"), py_str("pruning"), py_str("fertiliser"), py_str("fertilizer"), py_str("feed"), py_str("pests"), py_str("pest"), py_str("aphids"), py_str("frost"), py_str("winter"), py_str("indoor"), py_str("indoors"), py_str("sunlight"), py_str("shade"), py_str("soil"), py_str("compost"), py_str("buy"), py_str("where can i buy"), py_str("how do i keep"), py_str("keep it alive"), py_str("overwinter"), py_str("plugs"), py_str("seeds"), py_str("taken over"), py_str("take over"), py_str("taking over"), py_str("come back"), py_str("comes back"), py_str("coming back"), py_str("gone mad"), py_str("going mad"), py_str("running riot"), py_str("overgrown"), py_str("spreading"), py_str("spreads"), py_str("spread"), py_str("invasive"), py_str("taking hold"), py_str("safe"), py_str("unsafe"), py_str("can i eat"), py_str("is it safe"), py_str("dogs"), py_str("dog"), py_str("cats"), py_str("cat"), py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("pesticide"), py_str("pesticides"), py_str("kids"), py_str("toddler")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("herb-plant-to-cook")); __args.push(py_tuple(vec![g__HERBS_GROWING(), op_add(&g__HERB_PLANT_WORDS(), &g__HERB_SURPLUS_WORDS())])); __args.push(py_func("_herb_plant_reply", |args: &[Py]| _herb_plant_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("_herb_use_asked", |args: &[Py]| _herb_use_asked(args))), ("avoid".to_string(), __data103())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data104() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("prepare"), py_str("clean"), py_str("cleaning"), py_str("steam"), py_str("steaming"), py_str("grill"), py_str("grilling"), py_str("how do i"), py_str("how to"), py_str("what do i do"), py_str("eat"), py_str("serve"), py_str("recipe"), py_str("best way"), py_str("open"), py_str("shut"), py_str("sand"), py_str("grit")]) }
    fn __data105() -> Py { py_tuple(vec![py_str("shave"), py_str("shaving"), py_str("shaved"), py_str("blade"), py_str("blades"), py_str("burn"), py_str("bumps"), py_str("razor burn"), py_str("beard"), py_str("beard trimmer"), py_str("hair removal"), py_str("wax"), py_str("waxing"), py_str("razor blade")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("razor-clams")); __args.push(py_tuple(vec![py_tuple(vec![py_str("razor clam"), py_str("razor clams"), py_str("razor shell"), py_str("razor shells"), py_str("razor fish"), py_str("razorfish")]), __data104()])); __args.push(py_str("Rinse them under cold water and leave them an hour in salty water so they spit out the sand, then steam or grill them for about four minutes, until the shells open. They have no beard to pull like a mussel, and any that will not open go in the bin. Garlic, chilli, lemon and parsley is all they need.")); __kws.extend(vec![("avoid".to_string(), __data105())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._CUP_OF"); }
    s__CUP_OF(py_tuple(vec![py_tuple(vec![py_str("flour"), py_str("plain flour"), py_str("self raising flour"), py_str("self-raising flour"), py_str("bread flour"), py_str("spelt"), py_str("wholemeal"), py_str("Spoon the flour into the cup and level it off with a knife - a scoop straight from the bag packs in up to a fifth more, which is the difference between a light cake and a heavy one. A cup of plain flour is about 120g, and a level tablespoon about 8g.")]), py_tuple(vec![py_str("sugar"), py_str("caster sugar"), py_str("granulated"), py_str("brown sugar"), py_str("icing sugar"), py_str("muscovado"), py_str("Spoon white sugar into the cup and level it, and pack brown sugar down hard. A cup of caster or granulated sugar is about 200g, a cup of icing sugar about 120g, and a level tablespoon of caster sugar about 12g.")]), py_tuple(vec![py_str("butter"), py_str("margarine"), py_str("lard"), py_str("shortening"), py_str("Butter is easiest by weight: a cup is about 225g, a stick about 113g, a level tablespoon about 14g. If you have a block in its wrapper, the paper is marked in 25g lines - cut on them and there is nothing to wash up. Soften it first, or it will not cream evenly.")]), py_tuple(vec![py_str("oats"), py_str("porridge oats"), py_str("rolled oats"), py_str("A cup of rolled oats is about 90g, and 50g a person is a portion of porridge. Spoon them in and level rather than pressing them down, because oats settle and a packed cup can weigh half again as much.")]), py_tuple(vec![py_str("cocoa"), py_str("cocoa powder"), py_str("A cup of cocoa powder is about 85g, and it is the one ingredient that really does want spooning and levelling: cocoa compacts badly and a scooped cup can be a third heavier.")]), py_tuple(vec![py_str("honey"), py_str("syrup"), py_str("golden syrup"), py_str("treacle"), py_str("molasses"), py_str("Honey and syrup are measured in a jug at eye level, not a cup, because they do not settle flat - a cup of honey is about 340g. Warm the spoon or the jug in hot water first and it will run out clean.")]), py_tuple(vec![py_str("milk"), py_str("water"), py_str("oil"), py_str("buttermilk"), py_str("Liquids go in a jug on the worktop, read at eye level with the jug standing still - a cup of milk or water is 240ml and a cup of oil about 225ml.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._MEASURE_AWAY"); }
    s__MEASURE_AWAY(py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("risotto"), py_str("pasta"), py_str("spaghetti"), py_str("noodles"), py_str("couscous"), py_str("quinoa"), py_str("porridge"), py_str("soup"), py_str("gravy"), py_str("stock")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._MEASURING_TAIL"); }
    s__MEASURING_TAIL(py_str("Spoon it into the cup and level it rather than scooping from the bag, because a cup is a volume and how much it holds depends on how the ingredient went in. With no scales at all, use one mug for everything in the recipe - the proportions matter more than the exact weight."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._RECIPE_TERM"); }
    s__RECIPE_TERM(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("divided"), py_str("divide"), py_str("half in the sauce"), py_str("reserve")]), py_str("\\bdivid(?:ed|e)\\b"), py_str("'Divided' means the amount is split between stages: half the cheese in the sauce and half on top, or the butter half in the cake and half in the icing. Weigh or measure the whole lot out and set the second part aside in a bowl before you start, or it is easy to tip it all in at once.")]), py_tuple(vec![py_tuple(vec![py_str("until"), py_str("cue"), py_str("how do i know when it is done")]), py_str("\\buntil\\b"), py_str("'Until' is a cue, not a time: until golden, until it comes away from the pan, until a skewer comes out clean. The recipe gives you the sign to look for because the clock cannot - ovens and tins differ - so trust your eyes and nose first and the timer second.")]), py_tuple(vec![py_tuple(vec![py_str("mise en place"), py_str("prep"), py_str("prepped"), py_str("prep ahead")]), py_str("\\bmise en place\\b|\\bprep\\b"), py_str("Mise en place is everything weighed, chopped and set out in bowls before you turn the heat on - the French for 'in place'. It is the whole trick of the last five minutes of a recipe: most dishes go wrong there because the onion was not chopped yet. Read the ingredients as a shopping-and-chopping list and do all of it first.")]), py_tuple(vec![py_tuple(vec![py_str("cup"), py_str("cups"), py_str("what is a cup")]), py_str("\\bcups?\\b"), py_str("A cup in a recipe is a volume: 240ml, or 8 US fluid ounces, which is a bit over half a pint. Most British recipes weigh in grams instead and a British teacup is not a US cup, so a recipe that says cups is written in American measures - and a cup of flour is about 120g, a cup of sugar about 200g.")]), py_tuple(vec![py_tuple(vec![py_str("serves"), py_str("makes"), py_str("how many does it serve")]), py_str("\\bserves\\b|\\bmakes\\b|\\bhow many\\b"), py_str("'Serves four' usually means four as a main with something alongside, and the number is a guide rather than a promise - a hungry four will finish a dish written for six. Halve or double the weights if your table is smaller or bigger, and keep the tin size and the time in mind when you do.")])]));
    fn __data106() -> Py { py_tuple(vec![py_str("measure flour"), py_str("measuring flour"), py_str("weigh flour"), py_str("measure out flour"), py_str("measure sugar"), py_str("measuring sugar"), py_str("measure butter"), py_str("weigh butter"), py_str("measure with cups"), py_str("measuring cups"), py_str("cup measure"), py_str("cups to grams"), py_str("grams to cups"), py_str("measure ingredients"), py_str("measuring ingredients"), py_str("weigh ingredients"), py_str("weighing ingredients"), py_str("kitchen scales"), py_str("digital scales"), py_str("measure oil"), py_str("measure milk"), py_str("measure oats"), py_str("measure cocoa"), py_str("measure syrup"), py_str("measure honey"), py_str("ounces to grams"), py_str("oz to grams")]) }
    fn __data107() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("how much"), py_str("what is"), py_str("weigh"), py_str("weighing"), py_str("measure"), py_str("measuring"), py_str("convert"), py_str("grams"), py_str("cups"), py_str("ounces"), py_str("oz"), py_str("accurate"), py_str("properly"), py_str("right"), py_str("best way"), py_str("scoop")]) }
    fn __data108() -> Py { py_tuple(vec![py_str("without scales"), py_str("no scales"), py_str("no weighing"), py_str("mug"), py_str("mugs"), py_str("rice"), py_str("basmati"), py_str("risotto"), py_str("pasta"), py_str("spaghetti"), py_str("noodles"), py_str("couscous"), py_str("quinoa"), py_str("porridge"), py_str("blood sugar"), py_str("diabetes"), py_str("diabetic"), py_str("insulin"), py_str("sugar intake"), py_str("how much sugar"), py_str("added sugar"), py_str("cut down on sugar"), py_str("sugar free"), py_str("portion"), py_str("portion size"), py_str("how many calories"), py_str("calories in"), py_str("protein"), py_str("carbs"), py_str("weight loss"), py_str("lose weight"), py_str("diet"), py_str("weigh myself"), py_str("bathroom scales"), py_str("scale says")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("measure-ingredients")); __args.push(py_tuple(vec![__data106(), __data107()])); __args.push(py_func("_measuring", |args: &[Py]| _measuring(args))); __kws.extend(vec![("avoid".to_string(), __data108())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data109() -> Py { py_tuple(vec![py_str("read a recipe"), py_str("read the recipe"), py_str("follow a recipe"), py_str("following a recipe"), py_str("reading a recipe"), py_str("recipe card"), py_str("recipe says"), py_str("recipe mean"), py_str("recipe means"), py_str("recipe tells"), py_str("in a recipe"), py_str("on a recipe"), py_str("understand a recipe"), py_str("how do recipes work"), py_str("recipe instructions"), py_str("mise en place"), py_str("prep ahead")]) }
    fn __data110() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("understand"), py_str("read"), py_str("reading"), py_str("follow"), py_str("following"), py_str("mean"), py_str("means"), py_str("divided"), py_str("until"), py_str("prep"), py_str("prep time"), py_str("serves"), py_str("makes"), py_str("what does"), py_str("what is"), py_str("whats"), py_str("what's"), py_str("explain"), py_str("trick"), py_str("tips"), py_str("first")]) }
    fn __data111() -> Py { py_tuple(vec![py_str("divide"), py_str("division"), py_str("divided by"), py_str("vitamin"), py_str("nutrition"), py_str("calories"), py_str("slimming"), py_str("restaurant"), py_str("takeaway"), py_str("meal plan"), py_str("meal idea"), py_str("what should i cook"), py_str("dinner idea")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("read-a-recipe")); __args.push(py_tuple(vec![__data109(), __data110()])); __args.push(py_func("lambda", move |args: &[Py]| {  let memory: Py = __kwp(args, 0, "memory").unwrap_or(Py::None); let understood: Py = __kwp(args, 1, "understood").unwrap_or(Py::None); { let __b = _the_recipe_term(&[memory.clone(), understood.clone()]); if py_truthy(&__b) { __b } else { py_str("Read it twice, all the way through, before you touch anything: what it serves, the prep time and the cooking time separately, the oven setting, and whether anything has to chill, soak or rest. Then get every ingredient out and cut up, which is what the French call mise en place - most recipes go wrong in the last five minutes because the onion was not chopped yet. Two phrases to know: 'divided' means the amount is split between stages (half the cheese in the sauce, half on top), and 'until' is a cue rather than a time - until golden, until it comes away from the pan. Times in a recipe are for that oven and that tin; trust your eyes and nose first. Check the tin size too, because a wrong-sized tin changes the depth and with it the time.") } } })); __kws.extend(vec![("avoid".to_string(), __data111())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data112() -> Py { py_tuple(vec![py_str("halve a recipe"), py_str("halving a recipe"), py_str("half the recipe"), py_str("half a recipe"), py_str("double a recipe"), py_str("doubling a recipe"), py_str("scale a recipe"), py_str("scaling a recipe"), py_str("smaller batch"), py_str("bigger batch"), py_str("recipe for two"), py_str("recipe for one"), py_str("make half"), py_str("make double"), py_str("times the recipe")]) }
    fn __data113() -> Py { py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("can i"), py_str("is it"), py_str("do i"), py_str("times"), py_str("oven"), py_str("tin"), py_str("pan"), py_str("same"), py_str("longer"), py_str("shorter"), py_str("easier")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("halve-a-batch-keep-the-tin")); __args.push(py_tuple(vec![__data112(), __data113()])); __args.push(py_str("Halve the weights but not quite everything: salt, spices and strong flavours go a little under half, because they do not scale with the rest, and a raising agent like baking powder goes to about three quarters of half, or the bake tastes of it. An egg is the awkward one, so weigh it beaten and use half by weight - about 25g of a large egg. The tin matters more than the time: half the batter in the same tin is a shallow cake that bakes fast and dries, so use a smaller tin and check it early rather than turning the oven down. Doubling is the same in reverse: double the tin's volume, not its width, or bake in two tins.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("half a portion"), py_str("half the price"), py_str("half price"), py_str("half of it"), py_str("freeze"), py_str("leftovers"), py_str("keep"), py_str("store"), py_str("reheat"), py_str("calories"), py_str("guests"), py_str("party")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data114() -> Py { py_tuple(vec![py_str("bake"), py_str("baking"), py_str("what can i bake"), py_str("what should i bake"), py_str("what to bake"), py_str("what can we bake"), py_str("things to bake"), py_str("bake with kids"), py_str("bake with my kids"), py_str("bake with children"), py_str("bake with the kids"), py_str("baking with kids"), py_str("baking with children"), py_str("easy bake")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_bake_with_children"); }
    s_entry_bake_with_children({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bake-with-children")); __args.push(py_tuple(vec![__data114(), py_tuple(vec![py_str("kids"), py_str("kid"), py_str("children"), py_str("child"), py_str("toddler"), py_str("grandchildren"), py_str("family"), py_str("my son"), py_str("my daughter"), py_str("with my kids"), py_str("with children"), py_str("with kids")])])); __args.push(py_str("Three that work with children, easiest first: flapjack (no oven hands, just stirring and a flat tin), fairy cakes (they can fill the cases and ice them once cool) and shortbread (three ingredients, and cutting shapes is the fun part). Give the little ones the measuring and the stirring and keep the oven and the hob to yourself, and expect it to take twice as long as it would alone.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("birthday"), py_str("went wrong"), py_str("sank"), py_str("flat"), py_str("burnt"), py_str("burned"), py_str("raw"), py_str("soggy"), py_str("vegan"), py_str("gluten"), py_str("dairy"), py_str("calories")])), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_bake_with_children()]);
    fn __data115() -> Py { py_tuple(vec![py_str("mess"), py_str("messy"), py_str("messing"), py_str("mess-free"), py_str("no mess"), py_str("less mess"), py_str("much mess"), py_str("too much mess"), py_str("messiest"), py_str("minimal mess"), py_str("cleaning up"), py_str("clean up"), py_str("clear up"), py_str("clearing up"), py_str("washing up"), py_str("wash up"), py_str("one bowl"), py_str("fuss"), py_str("faff")]) }
    fn __data116() -> Py { py_tuple(vec![py_str("bake"), py_str("baking"), py_str("baked"), py_str("kids"), py_str("kid"), py_str("children"), py_str("child"), py_str("toddler"), py_str("grandchildren"), py_str("with the kids"), py_str("with kids"), py_str("with children"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("recipes"), py_str("ingredients")]) }
    fn __data117() -> Py { py_tuple(vec![py_str("floor"), py_str("kitchen floor"), py_str("paint"), py_str("painting"), py_str("glitter"), py_str("playdough"), py_str("play dough"), py_str("sand"), py_str("mud"), py_str("garden"), py_str("house"), py_str("room"), py_str("bedroom"), py_str("car"), py_str("iron"), py_str("hoover"), py_str("washing machine"), py_str("laundry"), py_str("dishes"), py_str("grease"), py_str("burnt on"), py_str("after baking"), py_str("mess after"), py_str("clean the kitchen"), py_str("cleaning the kitchen"), py_str("tidy the kitchen"), py_str("tidy up"), py_str("wipe"), py_str("clear the kitchen"), py_str("clear away"), py_str("sink full")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_bake_nothing_messy"); }
    s_entry_bake_nothing_messy({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bake-nothing-messy")); __args.push(py_tuple(vec![__data115(), __data116()])); __args.push(py_str("The least messy bakes are the ones with one bowl and no icing: a no-bake fridge cake (melted chocolate and butter, crushed biscuits, into a lined tin, an hour in the fridge), flapjack (one pan on the hob, one lined tin) and rocky road, which is the same thing with marshmallows. Shortbread and gingerbread shapes are tidy because the dough is stiff, and drop scones or pancakes make the mess in one jug. Three things that keep it small: weigh the ingredients out before anyone is called in, stand the bowl on a tray with the bin beside it, and line the tin with a sheet that overhangs so the whole bake lifts out.")); __kws.extend(vec![("avoid".to_string(), __data117()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_bake_nothing_messy()]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._POWDER_FOR_SODA"); }
    s__POWDER_FOR_SODA(stdlib_call("re", "compile", &[py_str("\\bbaking powder\\b[^.?!]{0,24}?\\b(?:instead of|in place of|for|substitute for|replace|swap for|swap with)\\b[^.?!]{0,16}?\\b(?:baking soda|bicarbonate|bicarb|soda)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_powder_for_soda"); }
    s_entry_powder_for_soda({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("powder-in-place-of-soda")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baking powder"), py_str("powder")]), py_tuple(vec![py_str("bicarbonate of soda"), py_str("bicarb"), py_str("baking soda"), py_str("soda")]), py_tuple(vec![py_str("instead of"), py_str("in place of"), py_str("for"), py_str("substitute"), py_str("replacement"), py_str("replace"), py_str("swap"), py_str("swapping"), py_str("use"), py_str("using"), py_str("can i"), py_str("could i")])])); __args.push(py_str("Use about three teaspoons of baking powder for each teaspoon of baking soda, and put the acid the recipe counted on - buttermilk, yoghurt, lemon juice - in some other way, or the rise goes flat. That is the easy direction, because baking powder already carries its own acid. The other way round is not a straight swap: bicarbonate is about four times stronger and needs an acid to work, so a quarter of a teaspoon with a teaspoon of lemon juice or vinegar stands in for each teaspoon of powder.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("difference"), py_str("different"), py_str("same"), py_str("vs"), py_str("versus"), py_str("what is baking powder"), py_str("what is bicarbonate"), py_str("soapy"), py_str("tastes of soap"), py_str("what does it do"), py_str("why do i need")])), ("guard".to_string(), py_func("_powder_goes_in_for_soda", |args: &[Py]| _powder_goes_in_for_soda(args))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_powder_for_soda()]);
    fn __data118() -> Py { py_tuple(vec![py_str("recipe"), py_str("how do i make dough"), py_str("how do i make pizza dough"), py_str("dough recipe"), py_str("flour"), py_str("yeast"), py_str("takeaway"), py_str("delivery"), py_str("order"), py_str("frozen"), py_str("calories"), py_str("healthy"), py_str("how long does it"), py_str("freeze")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pizza-base-soggy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pizza"), py_str("pizza base"), py_str("pizza bases"), py_str("pizza dough"), py_str("margherita")]), py_tuple(vec![py_str("soggy"), py_str("soggy bottom"), py_str("wet"), py_str("limp"), py_str("not crisp"), py_str("not crispy"), py_str("doughy in the middle"), py_str("raw in the middle"), py_str("went soggy"), py_str("goes soggy"), py_str("stodgy")])])); __args.push(py_str("A soggy pizza is three things at once: a cool oven, a thin tray and a wet topping. Heat the oven and the tray to its top setting for 20 minutes, roll the base thin and put it onto the hot tray, drain the mozzarella and pat any vegetables dry, and keep the sauce to a thin layer. Bake it high for 10 to 12 minutes, then finish it under the grill for a minute.")); __kws.extend(vec![("avoid".to_string(), __data118()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data119() -> Py { py_tuple(vec![py_str("sauce"), py_str("gravy"), py_str("soup"), py_str("stew"), py_str("curry"), py_str("roux"), py_str("white sauce"), py_str("cheese sauce"), py_str("casserole"), py_str("pie filling"), py_str("get rid of"), py_str("floury taste"), py_str("why does"), py_str("why is my sauce"), py_str("why has my sauce")]) }
    fn __data120() -> Py { py_tuple(vec![py_str("bread"), py_str("bake"), py_str("cake"), py_str("batter"), py_str("pancake"), py_str("flourless"), py_str("which flour"), py_str("what flour"), py_str("how much flour"), py_str("grams"), py_str("cup"), py_str("self raising"), py_str("plain flour"), py_str("potato"), py_str("potatoes"), py_str("spud"), py_str("spuds"), py_str("old"), py_str("gone off"), py_str("off"), py_str("smell"), py_str("smells"), py_str("smelling"), py_str("musty"), py_str("rancid"), py_str("stale"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("raw-flour-taste")); __args.push(py_tuple(vec![py_tuple(vec![py_str("floury"), py_str("tastes of flour"), py_str("tastes floury"), py_str("raw flour"), py_str("raw flour taste"), py_str("chalky"), py_str("tastes like flour"), py_str("flour taste"), py_str("powdery taste")]), __data119()])); __args.push(py_str("A floury taste means the flour is still raw, and the fix is time on the heat, not more seasoning: simmer it 5 to 10 minutes longer, stirring, so the starch cooks through and loses the raw edge. A roux wants 2 or 3 minutes of bubbling before any liquid goes in, and if the sauce is already made, a splash of stock and a slow simmer takes it out. Milk, butter or cream rounds off what is left.")); __kws.extend(vec![("avoid".to_string(), __data120())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data121() -> Py { py_tuple(vec![py_str("rice"), py_str("soup"), py_str("sauce"), py_str("stew"), py_str("pasta"), py_str("teapot"), py_str("tea"), py_str("water"), py_str("food"), py_str("dish"), py_str("curry"), py_str("pan"), py_str("kettle"), py_str("cook"), py_str("cooked"), py_str("boiled")]) }
    fn __data122() -> Py { py_tuple(vec![py_str("mouth"), py_str("in my mouth"), py_str("gum"), py_str("gums"), py_str("dentist"), py_str("medicine"), py_str("pregnant"), py_str("pregnancy"), py_str("sinus"), py_str("blood"), py_str("iron tablet"), py_str("coins"), py_str("coin"), py_str("pipe"), py_str("my kettle"), py_str("the kettle"), py_str("kettle tastes"), py_str("kettle smells"), py_str("from the kettle")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("metallic-taste-in-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("metallic"), py_str("metal"), py_str("tastes of metal"), py_str("taste of metal"), py_str("tastes metallic")]), __data121()])); __args.push(py_str("A metal taste in food is usually the pan or the water rather than the food: an unlined aluminium or cast-iron pan with anything acidic, a metal spoon left in a tomato sauce, or a furred-up kettle. Cook acidic dishes in stainless steel, enamel or non-stick, move leftovers out of the pan into glass or plastic, and descale the kettle with a 50/50 mix of water and white vinegar.")); __kws.extend(vec![("avoid".to_string(), __data122()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data123() -> Py { py_tuple(vec![py_str("fell apart"), py_str("falling apart"), py_str("falls apart"), py_str("fall apart"), py_str("broke up"), py_str("breaking up"), py_str("breaks up"), py_str("crumbled"), py_str("crumbling"), py_str("crumbles"), py_str("wont hold"), py_str("won't hold"), py_str("not holding"), py_str("holds together"), py_str("hold together"), py_str("went to pieces"), py_str("slid apart"), py_str("disintegrated")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.BURGER_APART"); }
    s_BURGER_APART(__data123());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.BURGER_APART_REPLY"); }
    s_BURGER_APART_REPLY(mcall(&py_str("It falls apart for three reasons: the mince is too lean, it was handled too much, or it was turned before it had a crust. Use mince with some fat in it (20 per cent is right), break it up with a fork rather than squeezing it, and add an egg and a couple of spoons of breadcrumbs per 500g if the meat is lean - then shape it loosely, press a shallow dip in the middle and chill it 20 minutes so it sets. Salt it just before it goes in the pan, cook it in a hot pan and leave it alone for 3 minutes before the first turn, and it will hold."), "replace", &[py_str("a egg"), py_str("an egg")]));
    fn __data124() -> Py { py_tuple(vec![py_str("how do i make"), py_str("recipe"), py_str("best burger"), py_str("veggie"), py_str("vegan"), py_str("bean burger"), py_str("tofu"), py_str("bun"), py_str("buns"), py_str("sauce"), py_str("cheese slice"), py_str("restaurant"), py_str("takeaway")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("burger-fell-apart")); __args.push(py_tuple(vec![py_tuple(vec![py_str("burger"), py_str("patty"), py_str("patties"), py_str("beefburger"), py_str("meatball"), py_str("meatballs"), py_str("meatloaf"), py_str("kofta"), py_str("koftas"), py_str("frikadellen")]), g_BURGER_APART()])); __args.push(g_BURGER_APART_REPLY()); __kws.extend(vec![("avoid".to_string(), __data124()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data125() -> Py { py_tuple(vec![py_str("football match"), py_str("before a match"), py_str("before the match"), py_str("playing sport"), py_str("five a side"), py_str("swimming gala"), py_str("cross country"), py_str("sports day"), py_str("before training"), py_str("game tomorrow"), py_str("before football"), py_str("before rugby"), py_str("before a run"), py_str("before swimming"), py_str("rugby match"), py_str("netball match"), py_str("hockey match"), py_str("match tomorrow"), py_str("before the game"), py_str("before his match"), py_str("before her match"), py_str("match on saturday"), py_str("match on sunday"), py_str("football tomorrow"), py_str("swimming gala"), py_str("sports day")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.MEAL_SITUATIONS"); }
    s_MEAL_SITUATIONS(py_tuple(vec![py_tuple(vec![__data125(), py_str("Feed them plain carbohydrate about three hours before, not straight before: a bowl of pasta with a little butter or cheese, a jacket potato, or toast and banana, plus water. Keep the fat and the fibre light so it is out of the stomach by kick-off, and have a banana or a flask of squash for after. A cold day wants a warm drink in a flask more than a bigger meal.")]), py_tuple(vec![py_tuple(vec![py_str("coeliac"), py_str("celiac"), py_str("gluten free"), py_str("gluten-free"), py_str("no gluten"), py_str("wheat allergy")]), py_str("Cook one dish that is gluten-free by nature rather than making a second version: chicken thighs roasted with potatoes and lemon, a chilli with rice, or a fish pie with a mashed top. Check the stock cube, the soy sauce and the sausages, because that is where the wheat hides, use a clean board, and put the bread in its own basket so nothing is cross-contaminated.")]), py_tuple(vec![py_tuple(vec![py_str("no oven"), py_str("oven is broken"), py_str("oven broke"), py_str("without an oven"), py_str("hob only"), py_str("just a hob"), py_str("oven is out"), py_str("no cooker"), py_str("microwave only"), py_str("moving house")]), py_str("Everything on the hob: pasta with a sauce made in the same pan, rice with a stir fry, tinned tomatoes with beans and eggs poached in them, or soup with bread. One pan and one hob ring is plenty if you add the pasta to the sauce with a ladle of its water, and a kettle fills in for hot water.")]), py_tuple(vec![py_tuple(vec![py_str("tight budget"), py_str("on a budget"), py_str("skint"), py_str("no money"), py_str("cheap meals"), py_str("cheap dinners"), py_str("money is tight"), py_str("cutting back"), py_str("cost of living")]), py_str("Build the week on eggs, lentils, beans, oats, frozen vegetables and chicken thighs: a lentil bolognese, a bean chilli, egg fried rice, a frittata and a soup covers five dinners for about the price of one takeaway. Frozen veg and tinned tomatoes cost less and waste less than fresh, and a whole chicken gives a roast plus a second meal from the carcass.")]), py_tuple(vec![py_tuple(vec![py_str("pregnant"), py_str("cant stand the smell"), py_str("can't stand the smell"), py_str("smell of cooking"), py_str("smells make me"), py_str("cooking makes me")]), py_str("Eat cold or barely cooked while the smell turns your stomach: sandwiches, cereal, yoghurt, fruit, cheese and crackers, or a plate of the dinner you cooked yesterday eaten cold. Cook once in the morning when the kitchen is fresher, open a window and use the extractor, and let somebody else do the frying - it is the smell of hot fat that carries.")]), py_tuple(vec![py_tuple(vec![py_str("chest infection"), py_str("no appetite"), py_str("lost my appetite"), py_str("recovering from flu"), py_str("got the flu"), py_str("been ill"), py_str("feeling ill"), py_str("off my food"), py_str("cant eat"), py_str("can't eat")]), py_str("Small and plain, eaten often: soup, toast, rice pudding, yoghurt, a banana, scrambled egg on a good day, and a drink beside you all day. Do not worry about a proper dinner while you are ill - calories and fluid are the job, and the appetite comes back with them.")]), py_tuple(vec![py_tuple(vec![py_str("my back"), py_str("back is bad"), py_str("bad back"), py_str("cant stand long"), py_str("can't stand for long"), py_str("standing hurts"), py_str("on my feet all day"), py_str("knee is bad"), py_str("cant stand up")]), py_str("One tray in the oven and one pot on the hob, done in about ten minutes of standing: chicken thighs or sausages with chopped veg and oil at 200C for 30 minutes, or a chilli you leave to simmer. Sit at the table to prep with everything within reach, use a stool at the hob, and cook double so tomorrow needs no standing at all.")]), py_tuple(vec![py_tuple(vec![py_str("camping"), py_str("in a tent"), py_str("in the tent"), py_str("no fridge"), py_str("campervan"), py_str("camper van"), py_str("festival"), py_str("no electricity")]), py_str("One pot, no fridge: pasta with tinned tomatoes, tinned beans and a chopped chorizo or a tin of tuna, or a curry from a jar with rice. Take food that keeps without cooling - tins, dried pasta, noodles, porridge oats, bread, hard cheese, apples - and a small gas ring plus matches in a plastic bag, because a wet night in a tent is when the cooking has to be simple.")]), py_tuple(vec![py_tuple(vec![py_str("packed lunch"), py_str("packed lunches"), py_str("take to work"), py_str("take for lunch"), py_str("lunch for work"), py_str("lunchbox"), py_str("lunch box"), py_str("back at work"), py_str("back to work"), py_str("school lunch"), py_str("dinner lady")]), py_str("Cook one extra portion of dinner and box it: rice, pasta or a grain with a protein and something sharp keeps a day in a cool bag and needs no microwave. If there is nothing left over, a flask of soup with bread, or a tub of couscous made with boiling water from a kettle and stirred with feta, tomato and herbs, takes five minutes the night before.")]), py_tuple(vec![py_tuple(vec![py_str("night shift"), py_str("night shifts"), py_str("eats at 3am"), py_str("at 3am"), py_str("works nights"), py_str("working nights"), py_str("night worker")]), py_str("Treat the shift as two meals: a proper one before you go in - rice, pasta or potato with protein and vegetables - then something light about two thirds of the way through, like a sandwich, yoghurt, fruit or cereal, rather than a full dinner at 3am. Heavy fat at that hour sits on the stomach and makes the last hours worse, and a flask of something warm helps more than coffee at the end of the shift.")]), py_tuple(vec![py_tuple(vec![py_str("pudding"), py_str("dessert"), py_str("to bring a cake"), py_str("bring a cake"), py_str("bring a dessert"), py_str("bbq"), py_str("barbecue"), py_str("pot luck"), py_str("bring something")]), py_str("Bring something that travels and needs no fridge: a lemon drizzle traybake, brownies, a fruit crumble in its dish, or an Eton mess carried as meringue, cream and fruit in three tubs to layer at the table. Cut it in the tin so it arrives whole, and take the serving spoon with you.")]), py_tuple(vec![py_tuple(vec![py_str("tiny kitchen"), py_str("no worktop"), py_str("small kitchen"), py_str("no space"), py_str("little space"), py_str("one hob ring"), py_str("no surface")]), py_str("One tray or one pan a night, prepped on a board balanced on the hob or the table: a traybake of chicken, potatoes and veg, a one-pot pasta where everything goes in together, or a stir fry chopped straight into the pan. Clear as you go so the board has a home, and cook double on the nights you have room, because the second meal needs no worktop at all.")])]));
    fn __data126() -> Py { py_tuple(vec![py_str("what can i cook"), py_str("what can i make"), py_str("what should i cook"), py_str("what should i make"), py_str("what can we eat"), py_str("what shall i cook"), py_str("what shall i make"), py_str("what should i feed"), py_str("what can i feed"), py_str("what can i bring"), py_str("what should i bring"), py_str("what do i feed"), py_str("what do i cook"), py_str("what do i make"), py_str("what is quick"), py_str("whats quick"), py_str("what's quick"), py_str("what can i give"), py_str("what should i give"), py_str("what to cook"), py_str("what to make"), py_str("meal ideas"), py_str("dinner ideas"), py_str("what can i do for dinner"), py_str("what can i do for tea"), py_str("what should i take for lunch"), py_str("what do i take for lunch"), py_str("what to take for lunch"), py_str("what should i buy"), py_str("what do i buy"), py_str("what cake can i make"), py_str("what can i bake"), py_str("how do i prep"), py_str("prep a meal"), py_str("what should i eat"), py_str("what can i eat"), py_str("what should he eat"), py_str("what should she eat"), py_str("what should they eat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.MEAL_ASK"); }
    s_MEAL_ASK(__data126());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.DIABETES_WORDS"); }
    s_DIABETES_WORDS(py_tuple(vec![py_str("diabetic"), py_str("diabetics"), py_str("diabetes"), py_str("type 1"), py_str("type 2"), py_str("blood sugar"), py_str("watching my sugar"), py_str("sugar free"), py_str("low sugar"), py_str("no sugar"), py_str("sugar diabetes")]));
    fn __data127() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("bake"), py_str("baking"), py_str("biscuit"), py_str("biscuits"), py_str("pudding"), py_str("puddings"), py_str("dessert"), py_str("desserts"), py_str("something sweet"), py_str("sweet treat"), py_str("sweet things"), py_str("sweets"), py_str("sweetener"), py_str("sweeteners"), py_str("sugar substitute")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.SWEET_ASK"); }
    s_SWEET_ASK(__data127());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.DIABETIC_BAKE"); }
    s_DIABETIC_BAKE(py_str("Bake something that is low in sugar by nature rather than swapping in sweeteners: an almond and olive oil cake, a carrot cake made with less sugar and more spice, or berries with cream and a little dark chocolate. Keep the portion small and have it after a meal rather than on its own, and put the fruit and the cream on the plate so the cake slice can be thinner."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._A_MEAL_IS_NAMED"); }
    s__A_MEAL_IS_NAMED(py_tuple(vec![py_str("dinner"), py_str("dinners"), py_str("supper"), py_str("lunch"), py_str("main meal"), py_str("main course")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._ALLOWED_ASKS"); }
    s__ALLOWED_ASKS(py_tuple(vec![py_str("is it ok"), py_str("is it okay"), py_str("is it alright"), py_str("allowed"), py_str("ok for"), py_str("okay for"), py_str("bad for"), py_str("good for"), py_str("fine for"), py_str("suitable for")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._EATING_VERBS"); }
    s__EATING_VERBS(py_tuple(vec![py_str(" eat "), py_str(" have "), py_str(" drink ")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-bake-for-a-diabetic")); __args.push(py_tuple(vec![g_DIABETES_WORDS(), g_SWEET_ASK()])); __args.push(g_DIABETIC_BAKE()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("_a_bake_is_the_ask", |args: &[Py]| _a_bake_is_the_ask(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.MEAL_SITUATION_WORDS"); }
    s_MEAL_SITUATION_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAL_SITUATIONS()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut words: Py = __unpk1[0].clone(); let mut _reply: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.MEAL_ASK_WORDS"); }
    s_MEAL_ASK_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAL_ASK()) { let mut phrase: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&phrase.clone(), "split", &[])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data128() -> Py { py_tuple(vec![py_str("gone off"), py_str("is it safe"), py_str("still ok"), py_str("eat the"), py_str("freezer broke"), py_str("nothing to eat"), py_str("should i eat"), py_str("how long does"), py_str("thaw"), py_str("defrost"), py_str("allergic"), py_str("allergy"), py_str("intolerant"), py_str("poisoning"), py_str("recipe for"), py_str("how do i make a")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-for-the-situation")); __args.push(py_tuple(vec![g_MEAL_SITUATION_WORDS(), g_MEAL_ASK()])); __args.push(py_func("_meal_for_the_situation", |args: &[Py]| _meal_for_the_situation(args))); __kws.extend(vec![("avoid".to_string(), __data128()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data129() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("biscuit"), py_str("biscuits"), py_str("cookie"), py_str("cookies"), py_str("bun"), py_str("buns"), py_str("muffin"), py_str("muffins"), py_str("loaf"), py_str("loaves"), py_str("bread"), py_str("sponge"), py_str("brownie"), py_str("brownies"), py_str("flapjack"), py_str("flapjacks"), py_str("traybake"), py_str("tray bake"), py_str("pie"), py_str("crumble"), py_str("pudding"), py_str("frosting"), py_str("icing"), py_str("buttercream"), py_str("recipe"), py_str("recipes"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("batter"), py_str("dough"), py_str("meringue"), py_str("scone"), py_str("scones"), py_str("shortbread"), py_str("cheesecake"), py_str("swiss roll"), py_str("madeira"), py_str("victoria sandwich"), py_str("tin"), py_str("tins"), py_str("oven"), py_str("frosting"), py_str("ganache"), py_str("dessert")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._BAKE_NAMED"); }
    s__BAKE_NAMED(__data129());
    fn __data130() -> Py { py_tuple(vec![py_str("less"), py_str("leave"), py_str("leaving"), py_str("leave out"), py_str("leaving out"), py_str("left out"), py_str("left"), py_str("take"), py_str("taking"), py_str("take out"), py_str("taking out"), py_str("cut"), py_str("cutting"), py_str("cut down"), py_str("cut the"), py_str("cut it"), py_str("reduce"), py_str("reducing"), py_str("halve"), py_str("halving"), py_str("half the"), py_str("omit"), py_str("skip"), py_str("skipping"), py_str("no sugar"), py_str("without sugar"), py_str("sugar free"), py_str("sugar-free"), py_str("too sweet"), py_str("less sweet"), py_str("not so sweet"), py_str("healthier"), py_str("healthy"), py_str("diet"), py_str("diabetic"), py_str("diabetes"), py_str("calories"), py_str("how little"), py_str("how much less")]) }
    fn __data131() -> Py { py_tuple(vec![py_str("how much sugar"), py_str("how many calories"), py_str("calories in"), py_str("coke"), py_str("cola"), py_str("fizzy"), py_str("cereal"), py_str("yoghurt"), py_str("yogurt"), py_str("juice"), py_str("squash"), py_str("energy drink"), py_str("bad for"), py_str("addictive"), py_str("teeth"), py_str("blood sugar"), py_str("insulin"), py_str("type 2"), py_str("type 1"), py_str("double"), py_str("doubling"), py_str("half the recipe"), py_str("for two tins"), py_str("two tins"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("gone off"), py_str("fridge"), py_str("freeze")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("less-sugar-in-a-bake")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sugar"), py_str("caster sugar"), py_str("granulated"), py_str("icing sugar"), py_str("brown sugar"), py_str("sweetener"), py_str("sweet")]), __data130()])); __args.push(py_str("You can cut the sugar in most cakes and traybakes by a third to a half and still get a good bake, but not all of it: sugar keeps the crumb soft, browns the crust and holds the structure, so a cake with none bakes pale, dry and crumbly. Leave at least a quarter of what the recipe asks, add a splash of milk if the batter looks stiff, and eat it within a couple of days. Bread needs its teaspoon to get the yeast going, and jam and meringue are set by their sugar, so those cannot be cut.")); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let said: Py = __kwp(args, 0, "said").unwrap_or(Py::None); _a_bake_is_named(&[said.clone()]) })), ("avoid".to_string(), __data131()), ("bias".to_string(), Py::Int(5))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._TABLE_AHEAD"); }
    s__TABLE_AHEAD(py_str("(?=[^.?!]{0,30}?\\b(?:coming|come|round|over|due|arriving|invited|tonight|tomorrow|this\\s+weekend|next\\s+week|for\\s+(?:dinner|lunch|tea|supper|brunch|christmas|a\\s+meal|the\\s+meal|a\\s+party|a\\s+buffet|a\\s+barbecue|a\\s+bbq|a\\s+birthday|sat|sun|fri|mon|tue|wed|thu)|at\\s+(?:my|our|a|the)\\s+(?:party|do|barbecue|bbq|dinner|table|wedding|reception)|on\\s+(?:sat|sun|fri|mon|tue|wed|thu)\\w*))"));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._A_TABLE_OF_PEOPLE"); }
    s__A_TABLE_OF_PEOPLE(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:having|have\\s+got|feeding|catering\\s+for|invited|inviting|cooking\\s+for|making\\s+food\\s+for|supper\\s+for|dinner\\s+for|lunch\\s+for|tea\\s+for|table\\s+for|seats?\\s+for)\\s+(?:\\d{1,3}|a\\s+dozen|twenty|thirty|forty|fifty)\\b"), &g__TABLE_AHEAD()), &py_str("|\\b(?:\\d{1,3}|a\\s+dozen|twenty|thirty|forty|fifty)\\s+(?:people|guests|adults|folk|of\\s+us|grown\\s*ups)\\b")), &g__TABLE_AHEAD()), &py_str("|\\b(?:people|guests|adults)\\s+(?:coming|round|over|due|arriving)\\b")), &g__TABLE_AHEAD()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data132() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("per head"), py_str("per person"), py_str("per guest"), py_str("amount"), py_str("quantity"), py_str("kilos"), py_str(" kg"), py_str("grams"), py_str("portions"), py_str("portion"), py_str("how big a"), py_str("how large a"), py_str("poison"), py_str("poisoning"), py_str("hygiene"), py_str("allergy"), py_str("allergic"), py_str("reaction"), py_str("ill"), py_str("sick"), py_str("waste"), py_str("bin"), py_str("bins"), py_str("food bank"), py_str("dog food"), py_str("cat food"), py_str("pet food"), py_str("baby food"), py_str("fast food"), py_str("menu"), py_str("seating"), py_str("seat"), py_str("plates"), py_str("cutlery"), py_str("chairs"), py_str("parking"), py_str("bbq"), py_str("barbecue"), py_str("barbeque"), py_str("buffet"), py_str("finger food"), py_str("party food"), py_str("canape"), py_str("nibbles"), py_str("crisps"), py_str("sausage rolls"), py_str("sandwiches"), py_str("picnic"), py_str("how many people does"), py_str("feeds"), py_str("feeding a"), py_str("plan"), py_str("planning"), py_str("meal plan"), py_str("the week"), py_str("cant cook"), py_str("can't cook"), py_str("cannot cook"), py_str("never cooked"), py_str("dont know how to cook"), py_str("don't know how to cook"), py_str("beginner")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._NOT_A_TABLE"); }
    s__NOT_A_TABLE(__data132());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps._A_DIET"); }
    s__A_DIET(py_tuple(vec![py_tuple(vec![py_str("vegetarian"), py_str("meat")]), py_tuple(vec![py_str("vegan"), py_str("meat")]), py_tuple(vec![py_str("no meat"), py_str("meat")]), py_tuple(vec![py_str("nut allergy"), py_str("nuts")]), py_tuple(vec![py_str("gluten free"), py_str("gluten")]), py_tuple(vec![py_str("dairy free"), py_str("dairy")]), py_tuple(vec![py_str("lactose"), py_str("dairy")])]));
    fn __data133() -> Py { py_tuple(vec![py_str("coming"), py_str("round"), py_str("over"), py_str("for"), py_str("having"), py_str("feeding"), py_str("catering"), py_str("invited"), py_str("due"), py_str("arriving"), py_str("on saturday"), py_str("on sunday"), py_str("on friday"), py_str("on monday"), py_str("tonight"), py_str("tomorrow")]) }
    fn __data134() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("per head"), py_str("per person"), py_str("per guest"), py_str("portion"), py_str("poison"), py_str("hygiene"), py_str("allergy"), py_str("allergic"), py_str("food bank"), py_str("menu"), py_str("seating"), py_str("chairs"), py_str("plates"), py_str("cutlery"), py_str("feed the dog"), py_str("feeding a baby")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.cooking_gaps.entry_a_table_of_people"); }
    s_entry_a_table_of_people({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-table-of-people")); __args.push(py_tuple(vec![py_tuple(vec![py_str("people"), py_str("guests"), py_str("adults"), py_str("folk"), py_str("of us"), py_str("feeding"), py_str("cooking for"), py_str("catering")]), __data133()])); __args.push(py_func("the_table_of_people", |args: &[Py]| the_table_of_people(args))); __kws.extend(vec![("avoid".to_string(), __data134()), ("guard".to_string(), py_func("_a_table_announced", |args: &[Py]| _a_table_announced(args))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    let _ = mcall(&g_ENTRIES(), "append", &[g_entry_a_table_of_people()]);
}
pub fn _spoken(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    return Py::Bool(!py_is(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<![a-z0-9])"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z0-9])")), line.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    Py::None
}
pub fn _the_table_the_chat_set(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The number the chat said this meal is for, or 0 when no earlier line set one.\n\n    \"im doing a roast on sunday for 8 people\" then \"how many people is it for\" is the same\n    question about the same table: the first line set it, and answering the second with the\n    six-person example contradicts what the person said a turn earlier. Only the last few of\n    their own lines are read, and only one that names the meal and a count of people, so an\n    age, a weight or a cooking time is never taken as the number at the table.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Int(0);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &op_neg(&Py::Int(8)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    found = mcall(&g__THE_TABLE_NAMED(), "search", &[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__THE_MEAL_NAMED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__A_TABLE_PUT_RIGHT(), "search", &[said.clone()]) } })) } }) {
    continue;
    }
    raw = mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("n")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&found.clone(), "group", &[py_str("only")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    count = if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) { call_builtin("int", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__TABLE_WORDS(), "get", &[raw.clone(), Py::Int(0)]) };
    if py_truthy(&{ let __cmp21: Py = count.clone(); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &Py::Int(30)) } else { Py::Bool(false) } }) {
    return count.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _roast_head_count(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut found: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The number of people the line is cooking for, or 0 when it names none.");
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("(?<![a-z0-9])(\\d{1,2})(?![a-z0-9])"), line.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    found = __item.clone();
    count = call_builtin("int", &[found.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __cmp21: Py = count.clone(); if py_truthy(&op_le(&Py::Int(4), &__cmp21)) { op_le(&__cmp21, &Py::Int(30)) } else { Py::Bool(false) } }) {
    return count.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&g__COUNT_WORDS());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    word = __unpk4[0].clone();
    count = __unpk4[1].clone();
    if py_truthy(&_spoken(&[line.clone(), word.clone()])) {
    return count.clone();
    }
    }
    return Py::Int(0);
    Py::None
}
pub fn _kilos(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut count: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "count") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: count"))) };
    let mut grams: Py = Py::None;
    let _ = py_str("The raw weight to buy, said the way a butcher would say it.");
    grams = op_mul(&count.clone(), &Py::Int(250));
    if py_truthy(&Py::Bool(py_eq(&op_modulo(&grams.clone(), &Py::Int(1000)), &Py::Int(0)))) {
    return op_modulo(&py_str("%dkg"), &op_floordiv(&grams.clone(), &Py::Int(1000)));
    }
    if py_truthy(&Py::Bool(py_eq(&op_modulo(&grams.clone(), &Py::Int(500)), &Py::Int(0)))) {
    return op_modulo(&py_str("%d.5kg"), &op_floordiv(&grams.clone(), &Py::Int(1000)));
    }
    return op_modulo(&py_str("%.2fkg"), &op_truediv(&grams.clone(), &Py::Float(1000.0)));
    Py::None
}
pub fn _the_time_a_joint_needs(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut grams: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "grams") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: grams"))) };
    let mut hours: Py = Py::None;
    let mut minutes: Py = Py::None;
    let _ = py_str("The time this card's own rule gives a joint of this weight: 20 minutes per 500g plus 20.\n\n    Worked out from the grams rather than from the whole kilos in the buy weight. A weight that\n    is not a whole number of kilos was read as the kilos below it, so six people - who want\n    1.5kg, three lots of 500g - were told \"an hour and 0 minutes\" by the same card that says 20\n    minutes per 500g plus 20, and five, seven, nine and eleven were as short.\n    ");
    minutes = op_add(&call_builtin("int", &[op_truediv(&grams.clone(), &Py::Int(25))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20));
    let __unpk1 = py_unpack(&call_builtin("divmod", &[minutes.clone(), Py::Int(60)]).unwrap_or_else(|e| exc::raise(e)), 2, None);
    hours = __unpk1[0].clone();
    minutes = __unpk1[1].clone();
    if py_truthy(&op_gt(&hours.clone(), &Py::Int(1))) {
    return if py_truthy(&minutes.clone()) { op_modulo(&py_str("%d hours %d minutes"), &py_tuple(vec![hours.clone(), minutes.clone()])) } else { op_modulo(&py_str("%d hours"), &hours.clone()) };
    }
    if py_truthy(&Py::Bool(py_eq(&hours.clone(), &Py::Int(1)))) {
    return if py_truthy(&minutes.clone()) { op_modulo(&py_str("an hour and %d minutes"), &minutes.clone()) } else { py_str("an hour") };
    }
    return op_modulo(&py_str("%d minutes"), &minutes.clone());
    Py::None
}
pub fn _the_dish_the_line_names(args: &[Py]) -> Py {
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
    let mut dishes_ruled_out: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line itself names a dish, a joint or a diet the crowd card is not for.");
    let __iter1 = py_iter_or_raise(&g__NOT_THE_CROWD_CARD());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<!\\w)"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?!\\w)")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    dishes_ruled_out = py_func("dishes_ruled_out", |args: &[Py]| pyrt::xrefs::xcall("knowledge.dishes", "dishes_ruled_out", args));
    return Py::Bool(py_contains(&py_call(&dishes_ruled_out, &[line.clone()]), &py_str("sunday-roast")));
    Py::None
}
pub fn _roast_for_a_crowd(args: &[Py]) -> Py {
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
    let mut count: Py = Py::None;
    let mut kilos: Py = Py::None;
    let mut length: Py = Py::None;
    let mut line: Py = Py::None;
    line = op_add(&op_add(&py_str(" "), &stdlib_call("re", "sub", &[py_str("[^a-z0-9' ]+"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str(" "));
    if py_truthy(&_the_dish_the_line_names(&[line.clone()])) {
    return Py::None;
    }
    count = { let __b = _roast_head_count(&[line.clone()]); if py_truthy(&__b) { __b } else { _the_table_the_chat_set(&[memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&count.clone()))) {
    return op_add(&op_add(&py_str("Allow about 250g of raw meat each with the bone in, or 150g boneless: six people want a 2kg chicken or a 1.5kg joint. Work backwards from the time you want to eat: meat in at 190C (170C fan) "), &_the_time_a_joint_needs(&[Py::Int(2000)])), &py_str(" before, because a 2kg bird is 20 minutes per 500g plus 20. Potatoes in an hour before, parboiled and rough in hot fat at 200C for 45 minutes. Stuffing and pigs in blankets 25 minutes before the end, greens 10. The meat comes out to rest 15 minutes under foil, and the gravy is made in the tin while it does."));
    }
    kilos = _kilos(&[count.clone()]);
    length = _the_time_a_joint_needs(&[op_mul(&count.clone(), &Py::Int(250))]);
    return op_modulo(&py_str("For %d people allow about 250g of raw meat each with the bone in, or 150g boneless, so buy about %s in total: one joint that size, or two good-sized chickens. Work backwards from the time you want to eat: meat in at 190C (170C fan) %s before, because a joint is 20 minutes per 500g plus 20. Potatoes in an hour before, parboiled and rough in hot fat at 200C for 45 minutes. Stuffing and pigs in blankets 25 minutes before the end, greens 10. The meat comes out to rest 15 minutes under foil, and the gravy is made in the tin while it does."), &py_tuple(vec![count.clone(), kilos.clone(), length.clone()]));
    Py::None
}
pub fn _recipe_amount(args: &[Py]) -> Py {
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
    let mut halving: Py = Py::None;
    let mut other_size: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tin: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The answer for a doubled, halved or differently tinned recipe, chosen by the words said.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    tin = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" tin "), py_str(" tins "), py_str(" sandwich tin"), py_str(" cake tin")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    other_size = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("bigger"), py_str("larger"), py_str("smaller"), py_str("different size"), py_str("different tin"), py_str("other tin"), py_str("wrong size"), py_str("too big"), py_str("too small"), py_str("deeper")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = tin.clone(); if !py_truthy(&__b) { __b } else { other_size.clone() } }) {
    return py_str("Go by the area of the tin rather than the numbers on the page: a 23cm round holds about a third more than a 20cm, so the same batter fills it shallower and bakes 5 to 10 minutes faster, and scaling the batter up by that third fills it to the same depth in the same time. Keep the oven temperature, fill to two-thirds, and check with a skewer 5 minutes before the recipe's time either way.");
    }
    halving = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("halve"), py_str("halving"), py_str("half the recipe"), py_str("half a recipe"), py_str("half this recipe"), py_str("half batch"), py_str("smaller batch"), py_str("only two of us"), py_str("just for one"), py_str("just me"), py_str("for two")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&halving.clone()) {
    return py_str("Halve everything except the egg and the raising agent: beat the egg and weigh half of it, and keep the baking powder or yeast nearer two-thirds, because it does not scale down in a straight line. Use a smaller tin or pan so the depth stays the same, keep the temperature, and start checking 5 minutes earlier - a smaller batch comes up to heat and browns sooner.");
    }
    return py_str("No - the time barely changes, the quantity does. Keep the oven or hob where it was and add 10 to 20 minutes for the extra volume to heat through and reduce, checking early rather than trusting the clock. Use a wider pot or two tins rather than a deeper one, swap two trays on their shelves halfway, and cut the liquid by about a tenth because less of it evaporates. Salt, chilli, stock and herbs do not double exactly: add three-quarters, taste, then top up.");
    Py::None
}
pub fn _herb_use_asked(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line asks for a use, or says the plant is theirs: \"i have a sage bush\" is\n    a person holding a herb they do not know what to do with, and \"Noted: you have a sage bush\"\n    is not an answer. A line about growing one or a fault in it is the garden's, and those\n    words are in this row's avoid list, so they never arrive here.");
    clean = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    words = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g__HERB_USE_ASK()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HERB_USE_ASK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__HERB_PLANT_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__HERB_OWNED(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _herb_plant_reply(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut herb: Py = Py::None;
    let mut pick: Py = Py::None;
    let mut use_: Py = Py::None;
    let mut woody: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The picking and the herb's own use - woody at the start of the cooking, soft at the end.\n    When the line asked what to cook, the cooking comes first.");
    words = call_builtin("set", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    clean = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    woody = { let __b = call_builtin("bool", &[op_bitand(&words.clone(), &g__WOODY_HERB())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WOODY_HERB()) { let mut herb: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &op_add(&op_add(&py_str(" "), &herb.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&woody.clone()) {
    use_ = py_str("Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew.");
    pick = py_str("Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there. ");
    } else {
    use_ = py_str("Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served.");
    pick = py_str("Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts. ");
    }
    if py_truthy(&_herb_use_asked(&[understood.clone()])) {
    return op_add(&op_add(&use_.clone(), &py_str(" ")), &mcall(&pick.clone(), "strip", &[]));
    }
    return op_add(&pick.clone(), &use_.clone());
    Py::None
}
pub fn _lined(args: &[Py]) -> Py {
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
    let _ = py_str("The words the person typed, cleaned, with a space at each end.");
    return op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), &py_str(" "));
    Py::None
}
pub fn _measuring(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    line = _lined(&[memory.clone(), understood.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__MEASURE_AWAY()) { let mut w: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__CUP_OF());
    for __item in __iter1 {
    words = __item.clone();
    let __iter2 = py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&op_add(&geti(&words.clone(), &op_neg(&Py::Int(1))), &py_str(" ")), &g__MEASURING_TAIL());
    }
    }
    }
    return g__MEASURING_TAIL();
    Py::None
}
pub fn _the_recipe_term(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    line = _lined(&[memory.clone(), understood.clone()]);
    let __iter1 = py_iter_or_raise(&g__RECIPE_TERM());
    for __item in __iter1 {
    words = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[geti(&words.clone(), &Py::Int(1)), line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&words.clone(), &Py::Int(2));
    }
    }
    return Py::None;
    Py::None
}
pub fn _powder_goes_in_for_soda(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    read = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    return call_builtin("bool", &[mcall(&g__POWDER_FOR_SODA(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_what_is_allowed(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks whether a food is allowed rather than asking for a recipe.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALLOWED_ASKS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EATING_VERBS()) { let mut verb: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &verb.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" can "), py_str(" could "), py_str(" should "), py_str(" is "), py_str(" are ")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_bake_is_the_ask(args: &[Py]) -> Py {
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
    let _ = py_str("The row answers a sweet that is asked for, and stands aside for a meal or a permission.\n\n    A guard rather than a reply that returns None: a row that matches and then declines takes the\n    turn down with it, where a guard lets the row behind it answer - \"can a diabetic eat cake\" is\n    the plate rule's, and it was the plate rule that answered it before.\n    ");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_MEAL_IS_NAMED()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&_asks_what_is_allowed(&[said.clone()])));
    Py::None
}
pub fn _meal_for_the_situation(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The dinner the situation wants, or nothing when the line names no situation.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAL_ASK_WORDS()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&g_MEAL_SITUATIONS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_bake_is_named(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names something being baked, so the sugar is in a recipe.\n\n    \"Can i leave the sugar out of a cake\" is this row's line; \"im trying to cut down on sugar\",\n    \"should i give up sugar\" and \"how much sugar is in a can of coke\" are the diet's and the\n    drink's, and none of them names a bake at all.\n    ");
    said = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("clean")]) } else { understood.clone() };
    words = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BAKE_NAMED()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_table_announced(args: &[Py]) -> Py {
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
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether the line states a table of people, with no meal and no cooking verb in it.");
    read = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_TABLE_OF_PEOPLE(), "search", &[text.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_A_TABLE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn the_table_of_people(args: &[Py]) -> Py {
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
    let mut banned: Py = Py::None;
    let mut food: Py = Py::None;
    let mut line: Py = Py::None;
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The one-dish answer for a stated table, sized by the number in the line.\n\n    The reading is the one cooking.py's own crowd row does, over the line plus the last few\n    turns: an occasion named one turn back (\"a bit of a do for my mums birthday\", then \"im\n    cooking for 12 on saturday\") belongs to the table being cooked for, and a table of twelve\n    told the plain version while cooking's row tells the fuller one is the same line answered two\n    ways. Where both rows fit, both now say the same thing, so the person sees no difference.\n    ");
    read = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    line = call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    text = line.clone();
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&text.clone(), &op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(4)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }])));
    text = __aug1;
    }
    text = mcall(&text.clone(), "lower", &[]);
    banned = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&g__A_DIET());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    word = __unpk3[0].clone();
    food = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &word.clone()))) {
    let _ = mcall(&banned.clone(), "add", &[food.clone()]);
    }
    }
    return pyrt::xrefs::xcall("knowledge.cooking", "_crowd_meal", &[banned.clone(), pyrt::xrefs::xcall("knowledge.cooking", "_occasion", &[text.clone()]), pyrt::xrefs::xcall("knowledge.cooking", "_crowd", &[text.clone()])]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.cooking_gaps".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "BURGER_APART" => Some(g_BURGER_APART()),
            "BURGER_APART_REPLY" => Some(g_BURGER_APART_REPLY()),
            "DIABETES_WORDS" => Some(g_DIABETES_WORDS()),
            "DIABETIC_BAKE" => Some(g_DIABETIC_BAKE()),
            "ENTRIES" => Some(g_ENTRIES()),
            "JACKET_FILL" => Some(g_JACKET_FILL()),
            "MAKE_AT_HOME" => Some(g_MAKE_AT_HOME()),
            "MEAL_ASK" => Some(g_MEAL_ASK()),
            "MEAL_ASK_WORDS" => Some(g_MEAL_ASK_WORDS()),
            "MEAL_SITUATIONS" => Some(g_MEAL_SITUATIONS()),
            "MEAL_SITUATION_WORDS" => Some(g_MEAL_SITUATION_WORDS()),
            "SWEET_ASK" => Some(g_SWEET_ASK()),
            "_ALLOWED_ASKS" => Some(g__ALLOWED_ASKS()),
            "_A_DIET" => Some(g__A_DIET()),
            "_A_MEAL_IS_NAMED" => Some(g__A_MEAL_IS_NAMED()),
            "_A_TABLE_OF_PEOPLE" => Some(g__A_TABLE_OF_PEOPLE()),
            "_A_TABLE_PUT_RIGHT" => Some(g__A_TABLE_PUT_RIGHT()),
            "_BAKE_NAMED" => Some(g__BAKE_NAMED()),
            "_CHILD_DRANK_IT" => Some(g__CHILD_DRANK_IT()),
            "_COUNT_WORDS" => Some(g__COUNT_WORDS()),
            "_CUP_OF" => Some(g__CUP_OF()),
            "_EATING_VERBS" => Some(g__EATING_VERBS()),
            "_FAULT" => Some(g__FAULT()),
            "_HERBS_GROWING" => Some(g__HERBS_GROWING()),
            "_HERB_OWNED" => Some(g__HERB_OWNED()),
            "_HERB_PLANT_WORDS" => Some(g__HERB_PLANT_WORDS()),
            "_HERB_SURPLUS_WORDS" => Some(g__HERB_SURPLUS_WORDS()),
            "_HERB_USE_ASK" => Some(g__HERB_USE_ASK()),
            "_MEASURE_AWAY" => Some(g__MEASURE_AWAY()),
            "_MEASURING_TAIL" => Some(g__MEASURING_TAIL()),
            "_NOT_A_TABLE" => Some(g__NOT_A_TABLE()),
            "_NOT_THE_CROWD_CARD" => Some(g__NOT_THE_CROWD_CARD()),
            "_ORDER_ASK" => Some(g__ORDER_ASK()),
            "_PAN_IS_HOT_ASK" => Some(g__PAN_IS_HOT_ASK()),
            "_PAN_THING" => Some(g__PAN_THING()),
            "_POWDER_FOR_SODA" => Some(g__POWDER_FOR_SODA()),
            "_RECIPE_TERM" => Some(g__RECIPE_TERM()),
            "_STARTER_NAMES" => Some(g__STARTER_NAMES()),
            "_TABLE_AHEAD" => Some(g__TABLE_AHEAD()),
            "_TABLE_WORDS" => Some(g__TABLE_WORDS()),
            "_THE_MEAL_NAMED" => Some(g__THE_MEAL_NAMED()),
            "_THE_TABLE_NAMED" => Some(g__THE_TABLE_NAMED()),
            "_TIN_AREA" => Some(g__TIN_AREA()),
            "_WOODY_HERB" => Some(g__WOODY_HERB()),
            "_a_bake_is_named" => Some(py_func("_a_bake_is_named", |args: &[Py]| _a_bake_is_named(args))),
            "_a_bake_is_the_ask" => Some(py_func("_a_bake_is_the_ask", |args: &[Py]| _a_bake_is_the_ask(args))),
            "_a_table_announced" => Some(py_func("_a_table_announced", |args: &[Py]| _a_table_announced(args))),
            "_asks_what_is_allowed" => Some(py_func("_asks_what_is_allowed", |args: &[Py]| _asks_what_is_allowed(args))),
            "_crowd" => Some(g__crowd()),
            "_crowd_meal" => Some(g__crowd_meal()),
            "_herb_plant_reply" => Some(py_func("_herb_plant_reply", |args: &[Py]| _herb_plant_reply(args))),
            "_herb_use_asked" => Some(py_func("_herb_use_asked", |args: &[Py]| _herb_use_asked(args))),
            "_kilos" => Some(py_func("_kilos", |args: &[Py]| _kilos(args))),
            "_lined" => Some(py_func("_lined", |args: &[Py]| _lined(args))),
            "_loaf_collapse_text" => Some(g__loaf_collapse_text()),
            "_meal_for_the_situation" => Some(py_func("_meal_for_the_situation", |args: &[Py]| _meal_for_the_situation(args))),
            "_measuring" => Some(py_func("_measuring", |args: &[Py]| _measuring(args))),
            "_occasion" => Some(g__occasion()),
            "_powder_goes_in_for_soda" => Some(py_func("_powder_goes_in_for_soda", |args: &[Py]| _powder_goes_in_for_soda(args))),
            "_recipe_amount" => Some(py_func("_recipe_amount", |args: &[Py]| _recipe_amount(args))),
            "_roast_for_a_crowd" => Some(py_func("_roast_for_a_crowd", |args: &[Py]| _roast_for_a_crowd(args))),
            "_roast_head_count" => Some(py_func("_roast_head_count", |args: &[Py]| _roast_head_count(args))),
            "_spoken" => Some(py_func("_spoken", |args: &[Py]| _spoken(args))),
            "_the_dish_the_line_names" => Some(py_func("_the_dish_the_line_names", |args: &[Py]| _the_dish_the_line_names(args))),
            "_the_recipe_term" => Some(py_func("_the_recipe_term", |args: &[Py]| _the_recipe_term(args))),
            "_the_table_the_chat_set" => Some(py_func("_the_table_the_chat_set", |args: &[Py]| _the_table_the_chat_set(args))),
            "_the_time_a_joint_needs" => Some(py_func("_the_time_a_joint_needs", |args: &[Py]| _the_time_a_joint_needs(args))),
            "a_plan_that_might_be_ruined" => Some(g_a_plan_that_might_be_ruined()),
            "entry" => Some(g_entry()),
            "entry_a_table_of_people" => Some(g_entry_a_table_of_people()),
            "entry_bake_nothing_messy" => Some(g_entry_bake_nothing_messy()),
            "entry_bake_went_wrong" => Some(g_entry_bake_went_wrong()),
            "entry_bake_with_children" => Some(g_entry_bake_with_children()),
            "entry_batter_went_wrong" => Some(g_entry_batter_went_wrong()),
            "entry_bored_of_chicken" => Some(g_entry_bored_of_chicken()),
            "entry_bread_did_not_rise" => Some(g_entry_bread_did_not_rise()),
            "entry_croutons" => Some(g_entry_croutons()),
            "entry_dish_went_wrong" => Some(g_entry_dish_went_wrong()),
            "entry_joint_weight" => Some(g_entry_joint_weight()),
            "entry_loaf_collapse" => Some(g_entry_loaf_collapse()),
            "entry_make_mayonnaise" => Some(g_entry_make_mayonnaise()),
            "entry_no_eggs" => Some(g_entry_no_eggs()),
            "entry_potato_toppings" => Some(g_entry_potato_toppings()),
            "entry_powder_for_soda" => Some(g_entry_powder_for_soda()),
            "entry_rice_burnt" => Some(g_entry_rice_burnt()),
            "entry_rises_then_drops" => Some(g_entry_rises_then_drops()),
            "entry_roast_for_a_crowd" => Some(g_entry_roast_for_a_crowd()),
            "entry_score_the_loaf" => Some(g_entry_score_the_loaf()),
            "entry_soup_curdle" => Some(g_entry_soup_curdle()),
            "entry_starter_dying" => Some(g_entry_starter_dying()),
            "entry_starter_keeping" => Some(g_entry_starter_keeping()),
            "entry_starter_mould" => Some(g_entry_starter_mould()),
            "entry_starter_smell" => Some(g_entry_starter_smell()),
            "not_ruled_out" => Some(g_not_ruled_out()),
            "the_table_of_people" => Some(py_func("the_table_of_people", |args: &[Py]| the_table_of_people(args))),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "BURGER_APART" => s_BURGER_APART(v),
            "BURGER_APART_REPLY" => s_BURGER_APART_REPLY(v),
            "DIABETES_WORDS" => s_DIABETES_WORDS(v),
            "DIABETIC_BAKE" => s_DIABETIC_BAKE(v),
            "ENTRIES" => s_ENTRIES(v),
            "JACKET_FILL" => s_JACKET_FILL(v),
            "MAKE_AT_HOME" => s_MAKE_AT_HOME(v),
            "MEAL_ASK" => s_MEAL_ASK(v),
            "MEAL_ASK_WORDS" => s_MEAL_ASK_WORDS(v),
            "MEAL_SITUATIONS" => s_MEAL_SITUATIONS(v),
            "MEAL_SITUATION_WORDS" => s_MEAL_SITUATION_WORDS(v),
            "SWEET_ASK" => s_SWEET_ASK(v),
            "_ALLOWED_ASKS" => s__ALLOWED_ASKS(v),
            "_A_DIET" => s__A_DIET(v),
            "_A_MEAL_IS_NAMED" => s__A_MEAL_IS_NAMED(v),
            "_A_TABLE_OF_PEOPLE" => s__A_TABLE_OF_PEOPLE(v),
            "_A_TABLE_PUT_RIGHT" => s__A_TABLE_PUT_RIGHT(v),
            "_BAKE_NAMED" => s__BAKE_NAMED(v),
            "_CHILD_DRANK_IT" => s__CHILD_DRANK_IT(v),
            "_COUNT_WORDS" => s__COUNT_WORDS(v),
            "_CUP_OF" => s__CUP_OF(v),
            "_EATING_VERBS" => s__EATING_VERBS(v),
            "_FAULT" => s__FAULT(v),
            "_HERBS_GROWING" => s__HERBS_GROWING(v),
            "_HERB_OWNED" => s__HERB_OWNED(v),
            "_HERB_PLANT_WORDS" => s__HERB_PLANT_WORDS(v),
            "_HERB_SURPLUS_WORDS" => s__HERB_SURPLUS_WORDS(v),
            "_HERB_USE_ASK" => s__HERB_USE_ASK(v),
            "_MEASURE_AWAY" => s__MEASURE_AWAY(v),
            "_MEASURING_TAIL" => s__MEASURING_TAIL(v),
            "_NOT_A_TABLE" => s__NOT_A_TABLE(v),
            "_NOT_THE_CROWD_CARD" => s__NOT_THE_CROWD_CARD(v),
            "_ORDER_ASK" => s__ORDER_ASK(v),
            "_PAN_IS_HOT_ASK" => s__PAN_IS_HOT_ASK(v),
            "_PAN_THING" => s__PAN_THING(v),
            "_POWDER_FOR_SODA" => s__POWDER_FOR_SODA(v),
            "_RECIPE_TERM" => s__RECIPE_TERM(v),
            "_STARTER_NAMES" => s__STARTER_NAMES(v),
            "_TABLE_AHEAD" => s__TABLE_AHEAD(v),
            "_TABLE_WORDS" => s__TABLE_WORDS(v),
            "_THE_MEAL_NAMED" => s__THE_MEAL_NAMED(v),
            "_THE_TABLE_NAMED" => s__THE_TABLE_NAMED(v),
            "_TIN_AREA" => s__TIN_AREA(v),
            "_WOODY_HERB" => s__WOODY_HERB(v),
            "_crowd" => s__crowd(v),
            "_crowd_meal" => s__crowd_meal(v),
            "_loaf_collapse_text" => s__loaf_collapse_text(v),
            "_occasion" => s__occasion(v),
            "a_plan_that_might_be_ruined" => s_a_plan_that_might_be_ruined(v),
            "entry" => s_entry(v),
            "entry_a_table_of_people" => s_entry_a_table_of_people(v),
            "entry_bake_nothing_messy" => s_entry_bake_nothing_messy(v),
            "entry_bake_went_wrong" => s_entry_bake_went_wrong(v),
            "entry_bake_with_children" => s_entry_bake_with_children(v),
            "entry_batter_went_wrong" => s_entry_batter_went_wrong(v),
            "entry_bored_of_chicken" => s_entry_bored_of_chicken(v),
            "entry_bread_did_not_rise" => s_entry_bread_did_not_rise(v),
            "entry_croutons" => s_entry_croutons(v),
            "entry_dish_went_wrong" => s_entry_dish_went_wrong(v),
            "entry_joint_weight" => s_entry_joint_weight(v),
            "entry_loaf_collapse" => s_entry_loaf_collapse(v),
            "entry_make_mayonnaise" => s_entry_make_mayonnaise(v),
            "entry_no_eggs" => s_entry_no_eggs(v),
            "entry_potato_toppings" => s_entry_potato_toppings(v),
            "entry_powder_for_soda" => s_entry_powder_for_soda(v),
            "entry_rice_burnt" => s_entry_rice_burnt(v),
            "entry_rises_then_drops" => s_entry_rises_then_drops(v),
            "entry_roast_for_a_crowd" => s_entry_roast_for_a_crowd(v),
            "entry_score_the_loaf" => s_entry_score_the_loaf(v),
            "entry_soup_curdle" => s_entry_soup_curdle(v),
            "entry_starter_dying" => s_entry_starter_dying(v),
            "entry_starter_keeping" => s_entry_starter_keeping(v),
            "entry_starter_mould" => s_entry_starter_mould(v),
            "entry_starter_smell" => s_entry_starter_smell(v),
            "not_ruled_out" => s_not_ruled_out(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.cooking_gaps", module_obj);
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_spoken", py_func("_spoken", |args: &[Py]| _spoken(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_the_table_the_chat_set", py_func("_the_table_the_chat_set", |args: &[Py]| _the_table_the_chat_set(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_roast_head_count", py_func("_roast_head_count", |args: &[Py]| _roast_head_count(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_kilos", py_func("_kilos", |args: &[Py]| _kilos(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_the_time_a_joint_needs", py_func("_the_time_a_joint_needs", |args: &[Py]| _the_time_a_joint_needs(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_the_dish_the_line_names", py_func("_the_dish_the_line_names", |args: &[Py]| _the_dish_the_line_names(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_roast_for_a_crowd", py_func("_roast_for_a_crowd", |args: &[Py]| _roast_for_a_crowd(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_recipe_amount", py_func("_recipe_amount", |args: &[Py]| _recipe_amount(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_herb_use_asked", py_func("_herb_use_asked", |args: &[Py]| _herb_use_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_herb_plant_reply", py_func("_herb_plant_reply", |args: &[Py]| _herb_plant_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_lined", py_func("_lined", |args: &[Py]| _lined(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_measuring", py_func("_measuring", |args: &[Py]| _measuring(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_the_recipe_term", py_func("_the_recipe_term", |args: &[Py]| _the_recipe_term(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_powder_goes_in_for_soda", py_func("_powder_goes_in_for_soda", |args: &[Py]| _powder_goes_in_for_soda(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_asks_what_is_allowed", py_func("_asks_what_is_allowed", |args: &[Py]| _asks_what_is_allowed(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_a_bake_is_the_ask", py_func("_a_bake_is_the_ask", |args: &[Py]| _a_bake_is_the_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_meal_for_the_situation", py_func("_meal_for_the_situation", |args: &[Py]| _meal_for_the_situation(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_a_bake_is_named", py_func("_a_bake_is_named", |args: &[Py]| _a_bake_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "_a_table_announced", py_func("_a_table_announced", |args: &[Py]| _a_table_announced(args)));
    pyrt::xrefs::reg_fn("knowledge.cooking_gaps", "the_table_of_people", py_func("the_table_of_people", |args: &[Py]| the_table_of_people(args)));
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "BURGER_APART", g_BURGER_APART, s_BURGER_APART);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "BURGER_APART_REPLY", g_BURGER_APART_REPLY, s_BURGER_APART_REPLY);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "DIABETES_WORDS", g_DIABETES_WORDS, s_DIABETES_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "DIABETIC_BAKE", g_DIABETIC_BAKE, s_DIABETIC_BAKE);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "JACKET_FILL", g_JACKET_FILL, s_JACKET_FILL);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "MAKE_AT_HOME", g_MAKE_AT_HOME, s_MAKE_AT_HOME);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "MEAL_ASK", g_MEAL_ASK, s_MEAL_ASK);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "MEAL_ASK_WORDS", g_MEAL_ASK_WORDS, s_MEAL_ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "MEAL_SITUATIONS", g_MEAL_SITUATIONS, s_MEAL_SITUATIONS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "MEAL_SITUATION_WORDS", g_MEAL_SITUATION_WORDS, s_MEAL_SITUATION_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "SWEET_ASK", g_SWEET_ASK, s_SWEET_ASK);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_ALLOWED_ASKS", g__ALLOWED_ASKS, s__ALLOWED_ASKS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_A_DIET", g__A_DIET, s__A_DIET);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_A_MEAL_IS_NAMED", g__A_MEAL_IS_NAMED, s__A_MEAL_IS_NAMED);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_A_TABLE_OF_PEOPLE", g__A_TABLE_OF_PEOPLE, s__A_TABLE_OF_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_A_TABLE_PUT_RIGHT", g__A_TABLE_PUT_RIGHT, s__A_TABLE_PUT_RIGHT);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_BAKE_NAMED", g__BAKE_NAMED, s__BAKE_NAMED);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_CHILD_DRANK_IT", g__CHILD_DRANK_IT, s__CHILD_DRANK_IT);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_COUNT_WORDS", g__COUNT_WORDS, s__COUNT_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_CUP_OF", g__CUP_OF, s__CUP_OF);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_EATING_VERBS", g__EATING_VERBS, s__EATING_VERBS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_FAULT", g__FAULT, s__FAULT);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_HERBS_GROWING", g__HERBS_GROWING, s__HERBS_GROWING);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_HERB_OWNED", g__HERB_OWNED, s__HERB_OWNED);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_HERB_PLANT_WORDS", g__HERB_PLANT_WORDS, s__HERB_PLANT_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_HERB_SURPLUS_WORDS", g__HERB_SURPLUS_WORDS, s__HERB_SURPLUS_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_HERB_USE_ASK", g__HERB_USE_ASK, s__HERB_USE_ASK);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_MEASURE_AWAY", g__MEASURE_AWAY, s__MEASURE_AWAY);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_MEASURING_TAIL", g__MEASURING_TAIL, s__MEASURING_TAIL);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_NOT_A_TABLE", g__NOT_A_TABLE, s__NOT_A_TABLE);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_NOT_THE_CROWD_CARD", g__NOT_THE_CROWD_CARD, s__NOT_THE_CROWD_CARD);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_ORDER_ASK", g__ORDER_ASK, s__ORDER_ASK);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_PAN_IS_HOT_ASK", g__PAN_IS_HOT_ASK, s__PAN_IS_HOT_ASK);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_PAN_THING", g__PAN_THING, s__PAN_THING);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_POWDER_FOR_SODA", g__POWDER_FOR_SODA, s__POWDER_FOR_SODA);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_RECIPE_TERM", g__RECIPE_TERM, s__RECIPE_TERM);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_STARTER_NAMES", g__STARTER_NAMES, s__STARTER_NAMES);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_TABLE_AHEAD", g__TABLE_AHEAD, s__TABLE_AHEAD);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_TABLE_WORDS", g__TABLE_WORDS, s__TABLE_WORDS);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_THE_MEAL_NAMED", g__THE_MEAL_NAMED, s__THE_MEAL_NAMED);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_THE_TABLE_NAMED", g__THE_TABLE_NAMED, s__THE_TABLE_NAMED);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_TIN_AREA", g__TIN_AREA, s__TIN_AREA);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_WOODY_HERB", g__WOODY_HERB, s__WOODY_HERB);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_crowd", g__crowd, s__crowd);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_crowd_meal", g__crowd_meal, s__crowd_meal);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_loaf_collapse_text", g__loaf_collapse_text, s__loaf_collapse_text);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "_occasion", g__occasion, s__occasion);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "a_plan_that_might_be_ruined", g_a_plan_that_might_be_ruined, s_a_plan_that_might_be_ruined);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_a_table_of_people", g_entry_a_table_of_people, s_entry_a_table_of_people);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_bake_nothing_messy", g_entry_bake_nothing_messy, s_entry_bake_nothing_messy);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_bake_went_wrong", g_entry_bake_went_wrong, s_entry_bake_went_wrong);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_bake_with_children", g_entry_bake_with_children, s_entry_bake_with_children);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_batter_went_wrong", g_entry_batter_went_wrong, s_entry_batter_went_wrong);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_bored_of_chicken", g_entry_bored_of_chicken, s_entry_bored_of_chicken);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_bread_did_not_rise", g_entry_bread_did_not_rise, s_entry_bread_did_not_rise);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_croutons", g_entry_croutons, s_entry_croutons);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_dish_went_wrong", g_entry_dish_went_wrong, s_entry_dish_went_wrong);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_joint_weight", g_entry_joint_weight, s_entry_joint_weight);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_loaf_collapse", g_entry_loaf_collapse, s_entry_loaf_collapse);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_make_mayonnaise", g_entry_make_mayonnaise, s_entry_make_mayonnaise);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_no_eggs", g_entry_no_eggs, s_entry_no_eggs);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_potato_toppings", g_entry_potato_toppings, s_entry_potato_toppings);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_powder_for_soda", g_entry_powder_for_soda, s_entry_powder_for_soda);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_rice_burnt", g_entry_rice_burnt, s_entry_rice_burnt);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_rises_then_drops", g_entry_rises_then_drops, s_entry_rises_then_drops);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_roast_for_a_crowd", g_entry_roast_for_a_crowd, s_entry_roast_for_a_crowd);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_score_the_loaf", g_entry_score_the_loaf, s_entry_score_the_loaf);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_soup_curdle", g_entry_soup_curdle, s_entry_soup_curdle);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_starter_dying", g_entry_starter_dying, s_entry_starter_dying);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_starter_keeping", g_entry_starter_keeping, s_entry_starter_keeping);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_starter_mould", g_entry_starter_mould, s_entry_starter_mould);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "entry_starter_smell", g_entry_starter_smell, s_entry_starter_smell);
    pyrt::xrefs::reg_access("knowledge.cooking_gaps", "not_ruled_out", g_not_ruled_out, s_not_ruled_out);
}

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


thread_local! { static G_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_BODY_LUMP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOOK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOB_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_ABOUT_WATER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MENU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPICE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAND_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEANING_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLERGEN_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ALLERGEN_ROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ANIMAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ADULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BABY_TILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_BARE_YEAR_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHILD_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DRINK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FEEDING_PHRASE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_ORGAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_RATE_IN_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TABLE_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TABLE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BABY_ACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRINK_OVER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRINK_SIX_TO_TWELVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRINK_UNDER_SIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FIRST_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOR_A_GROWN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOR_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GIVING_WATER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GIVING_WATER_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GIVING_WATER_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROWN_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HER_OWN_EATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_MUCH_MILK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INTRODUCES_A_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INTRODUCING_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_HOW_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_OVER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_SIX_TO_TWELVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_UNDER_SIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MILK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMES_A_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BABY_MILK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_BABY_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_SNACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_MY_DIET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER_WORDS_AT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OLDER_CHILD_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OWN_EATING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANT_MILK_BABY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANT_MILK_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLANT_MILK_RICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SEASON_OF_A_SHOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SELF_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SNACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SNACK_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SNACK_OVER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SNACK_SIX_TO_TWELVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SNACK_UNDER_SIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_REFUSAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_WITHOUT_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAKE_TO_FEED_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WAKE_TO_FEED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATER_JUICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATER_NO_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATER_OVER_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATER_UNDER_SIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHAT_TO_FEED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHILE_SHE_FEEDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_not_the_heat_of_an_argument: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.babyfood: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AGE() -> Py { ensure_init(); G_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE(v: Py) { ensure_init(); G_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_A_BODY_LUMP() -> Py { ensure_init(); G_A_BODY_LUMP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_BODY_LUMP(v: Py) { ensure_init(); G_A_BODY_LUMP.with(|c| *c.borrow_mut() = v); }
pub fn g_BABY() -> Py { ensure_init(); G_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BABY(v: Py) { ensure_init(); G_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_BOOK_WORDS() -> Py { ensure_init(); G_BOOK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOOK_WORDS(v: Py) { ensure_init(); G_BOOK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_EATING() -> Py { ensure_init(); G_EATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EATING(v: Py) { ensure_init(); G_EATING.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FOODS() -> Py { ensure_init(); G_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOODS(v: Py) { ensure_init(); G_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_JOB_WORDS() -> Py { ensure_init(); G_JOB_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOB_WORDS(v: Py) { ensure_init(); G_JOB_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_ABOUT_WATER() -> Py { ensure_init(); G_NOT_ABOUT_WATER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_ABOUT_WATER(v: Py) { ensure_init(); G_NOT_ABOUT_WATER.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_AGE() -> Py { ensure_init(); G_NOT_AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_AGE(v: Py) { ensure_init(); G_NOT_AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_BABY() -> Py { ensure_init(); G_NOT_A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_BABY(v: Py) { ensure_init(); G_NOT_A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MENU() -> Py { ensure_init(); G_NOT_A_MENU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MENU(v: Py) { ensure_init(); G_NOT_A_MENU.with(|c| *c.borrow_mut() = v); }
pub fn g_SPICE_NAMES() -> Py { ensure_init(); G_SPICE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPICE_NAMES(v: Py) { ensure_init(); G_SPICE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_STAGES() -> Py { ensure_init(); G_STAGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAGES(v: Py) { ensure_init(); G_STAGES.with(|c| *c.borrow_mut() = v); }
pub fn g_STAND_DOWN() -> Py { ensure_init(); G_STAND_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAND_DOWN(v: Py) { ensure_init(); G_STAND_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_WEANING_ELSEWHERE() -> Py { ensure_init(); G_WEANING_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEANING_ELSEWHERE(v: Py) { ensure_init(); G_WEANING_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_FIGURE() -> Py { ensure_init(); G__AGE_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_FIGURE(v: Py) { ensure_init(); G__AGE_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLERGEN_FOOD() -> Py { ensure_init(); G__ALLERGEN_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLERGEN_FOOD(v: Py) { ensure_init(); G__ALLERGEN_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__ALLERGEN_ROW() -> Py { ensure_init(); G__ALLERGEN_ROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ALLERGEN_ROW(v: Py) { ensure_init(); G__ALLERGEN_ROW.with(|c| *c.borrow_mut() = v); }
pub fn g__ANIMAL() -> Py { ensure_init(); G__ANIMAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ANIMAL(v: Py) { ensure_init(); G__ANIMAL.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ADULT() -> Py { ensure_init(); G__AN_ADULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ADULT(v: Py) { ensure_init(); G__AN_ADULT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BABY_TILL() -> Py { ensure_init(); G__A_BABY_TILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BABY_TILL(v: Py) { ensure_init(); G__A_BABY_TILL.with(|c| *c.borrow_mut() = v); }
pub fn g__A_BARE_YEAR_FIGURE() -> Py { ensure_init(); G__A_BARE_YEAR_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_BARE_YEAR_FIGURE(v: Py) { ensure_init(); G__A_BARE_YEAR_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHILD() -> Py { ensure_init(); G__A_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHILD(v: Py) { ensure_init(); G__A_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHILD_PRONOUN() -> Py { ensure_init(); G__A_CHILD_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHILD_PRONOUN(v: Py) { ensure_init(); G__A_CHILD_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CONDITION() -> Py { ensure_init(); G__A_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CONDITION(v: Py) { ensure_init(); G__A_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COUNT() -> Py { ensure_init(); G__A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNT(v: Py) { ensure_init(); G__A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DRINK() -> Py { ensure_init(); G__A_DRINK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DRINK(v: Py) { ensure_init(); G__A_DRINK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FEEDING_PHRASE() -> Py { ensure_init(); G__A_FEEDING_PHRASE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FEEDING_PHRASE(v: Py) { ensure_init(); G__A_FEEDING_PHRASE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_ORGAN() -> Py { ensure_init(); G__A_ORGAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_ORGAN(v: Py) { ensure_init(); G__A_ORGAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_RATE_IN_MONTHS() -> Py { ensure_init(); G__A_RATE_IN_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_RATE_IN_MONTHS(v: Py) { ensure_init(); G__A_RATE_IN_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TABLE_FOOD() -> Py { ensure_init(); G__A_TABLE_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TABLE_FOOD(v: Py) { ensure_init(); G__A_TABLE_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TABLE_WORD() -> Py { ensure_init(); G__A_TABLE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TABLE_WORD(v: Py) { ensure_init(); G__A_TABLE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__BABY() -> Py { ensure_init(); G__BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BABY(v: Py) { ensure_init(); G__BABY.with(|c| *c.borrow_mut() = v); }
pub fn g__BABY_ACT() -> Py { ensure_init(); G__BABY_ACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BABY_ACT(v: Py) { ensure_init(); G__BABY_ACT.with(|c| *c.borrow_mut() = v); }
pub fn g__DRINK_OVER_ONE() -> Py { ensure_init(); G__DRINK_OVER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRINK_OVER_ONE(v: Py) { ensure_init(); G__DRINK_OVER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__DRINK_SIX_TO_TWELVE() -> Py { ensure_init(); G__DRINK_SIX_TO_TWELVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRINK_SIX_TO_TWELVE(v: Py) { ensure_init(); G__DRINK_SIX_TO_TWELVE.with(|c| *c.borrow_mut() = v); }
pub fn g__DRINK_UNDER_SIX() -> Py { ensure_init(); G__DRINK_UNDER_SIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRINK_UNDER_SIX(v: Py) { ensure_init(); G__DRINK_UNDER_SIX.with(|c| *c.borrow_mut() = v); }
pub fn g__FIRST_FOODS() -> Py { ensure_init(); G__FIRST_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FIRST_FOODS(v: Py) { ensure_init(); G__FIRST_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g__FOR_A_GROWN_UP() -> Py { ensure_init(); G__FOR_A_GROWN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOR_A_GROWN_UP(v: Py) { ensure_init(); G__FOR_A_GROWN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__FOR_ME() -> Py { ensure_init(); G__FOR_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOR_ME(v: Py) { ensure_init(); G__FOR_ME.with(|c| *c.borrow_mut() = v); }
pub fn g__GIVING_WATER() -> Py { ensure_init(); G__GIVING_WATER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GIVING_WATER(v: Py) { ensure_init(); G__GIVING_WATER.with(|c| *c.borrow_mut() = v); }
pub fn g__GIVING_WATER_ASKS() -> Py { ensure_init(); G__GIVING_WATER_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GIVING_WATER_ASKS(v: Py) { ensure_init(); G__GIVING_WATER_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__GIVING_WATER_OUT() -> Py { ensure_init(); G__GIVING_WATER_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GIVING_WATER_OUT(v: Py) { ensure_init(); G__GIVING_WATER_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g__GROWN_UP() -> Py { ensure_init(); G__GROWN_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROWN_UP(v: Py) { ensure_init(); G__GROWN_UP.with(|c| *c.borrow_mut() = v); }
pub fn g__HER_OWN_EATING() -> Py { ensure_init(); G__HER_OWN_EATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HER_OWN_EATING(v: Py) { ensure_init(); G__HER_OWN_EATING.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_MUCH_MILK() -> Py { ensure_init(); G__HOW_MUCH_MILK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_MUCH_MILK(v: Py) { ensure_init(); G__HOW_MUCH_MILK.with(|c| *c.borrow_mut() = v); }
pub fn g__INTRODUCES_A_FOOD() -> Py { ensure_init(); G__INTRODUCES_A_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INTRODUCES_A_FOOD(v: Py) { ensure_init(); G__INTRODUCES_A_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__INTRODUCING_PEOPLE() -> Py { ensure_init(); G__INTRODUCING_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INTRODUCING_PEOPLE(v: Py) { ensure_init(); G__INTRODUCING_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_HOW_MUCH() -> Py { ensure_init(); G__MILK_HOW_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_HOW_MUCH(v: Py) { ensure_init(); G__MILK_HOW_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_OVER_ONE() -> Py { ensure_init(); G__MILK_OVER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_OVER_ONE(v: Py) { ensure_init(); G__MILK_OVER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_SIX_TO_TWELVE() -> Py { ensure_init(); G__MILK_SIX_TO_TWELVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_SIX_TO_TWELVE(v: Py) { ensure_init(); G__MILK_SIX_TO_TWELVE.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_UNDER_SIX() -> Py { ensure_init(); G__MILK_UNDER_SIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_UNDER_SIX(v: Py) { ensure_init(); G__MILK_UNDER_SIX.with(|c| *c.borrow_mut() = v); }
pub fn g__MILK_WORDS() -> Py { ensure_init(); G__MILK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MILK_WORDS(v: Py) { ensure_init(); G__MILK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMES_A_BABY() -> Py { ensure_init(); G__NAMES_A_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMES_A_BABY(v: Py) { ensure_init(); G__NAMES_A_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BABY_MILK() -> Py { ensure_init(); G__NOT_A_BABY_MILK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BABY_MILK(v: Py) { ensure_init(); G__NOT_A_BABY_MILK.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_BABY_NAME() -> Py { ensure_init(); G__NOT_A_BABY_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_BABY_NAME(v: Py) { ensure_init(); G__NOT_A_BABY_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_SNACK() -> Py { ensure_init(); G__NOT_A_SNACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_SNACK(v: Py) { ensure_init(); G__NOT_A_SNACK.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_MY_DIET() -> Py { ensure_init(); G__NOT_MY_DIET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_MY_DIET(v: Py) { ensure_init(); G__NOT_MY_DIET.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORDS() -> Py { ensure_init(); G__NUMBER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORDS(v: Py) { ensure_init(); G__NUMBER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER_WORDS_AT() -> Py { ensure_init(); G__NUMBER_WORDS_AT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER_WORDS_AT(v: Py) { ensure_init(); G__NUMBER_WORDS_AT.with(|c| *c.borrow_mut() = v); }
pub fn g__OLDER_CHILD_WORD() -> Py { ensure_init(); G__OLDER_CHILD_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OLDER_CHILD_WORD(v: Py) { ensure_init(); G__OLDER_CHILD_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__OWN_EATING() -> Py { ensure_init(); G__OWN_EATING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OWN_EATING(v: Py) { ensure_init(); G__OWN_EATING.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANT_MILK_BABY() -> Py { ensure_init(); G__PLANT_MILK_BABY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANT_MILK_BABY(v: Py) { ensure_init(); G__PLANT_MILK_BABY.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANT_MILK_CHILD() -> Py { ensure_init(); G__PLANT_MILK_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANT_MILK_CHILD(v: Py) { ensure_init(); G__PLANT_MILK_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__PLANT_MILK_RICE() -> Py { ensure_init(); G__PLANT_MILK_RICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLANT_MILK_RICE(v: Py) { ensure_init(); G__PLANT_MILK_RICE.with(|c| *c.borrow_mut() = v); }
pub fn g__SEASON_OF_A_SHOW() -> Py { ensure_init(); G__SEASON_OF_A_SHOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SEASON_OF_A_SHOW(v: Py) { ensure_init(); G__SEASON_OF_A_SHOW.with(|c| *c.borrow_mut() = v); }
pub fn g__SELF_PRONOUN() -> Py { ensure_init(); G__SELF_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SELF_PRONOUN(v: Py) { ensure_init(); G__SELF_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__SNACK() -> Py { ensure_init(); G__SNACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SNACK(v: Py) { ensure_init(); G__SNACK.with(|c| *c.borrow_mut() = v); }
pub fn g__SNACK_CHILD() -> Py { ensure_init(); G__SNACK_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SNACK_CHILD(v: Py) { ensure_init(); G__SNACK_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__SNACK_OVER_ONE() -> Py { ensure_init(); G__SNACK_OVER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SNACK_OVER_ONE(v: Py) { ensure_init(); G__SNACK_OVER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__SNACK_SIX_TO_TWELVE() -> Py { ensure_init(); G__SNACK_SIX_TO_TWELVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SNACK_SIX_TO_TWELVE(v: Py) { ensure_init(); G__SNACK_SIX_TO_TWELVE.with(|c| *c.borrow_mut() = v); }
pub fn g__SNACK_UNDER_SIX() -> Py { ensure_init(); G__SNACK_UNDER_SIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SNACK_UNDER_SIX(v: Py) { ensure_init(); G__SNACK_UNDER_SIX.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_CHILD() -> Py { ensure_init(); G__THE_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_CHILD(v: Py) { ensure_init(); G__THE_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_REFUSAL_WORDS() -> Py { ensure_init(); G__THE_REFUSAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_REFUSAL_WORDS(v: Py) { ensure_init(); G__THE_REFUSAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_WITHOUT_FIGURE() -> Py { ensure_init(); G__UNIT_WITHOUT_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_WITHOUT_FIGURE(v: Py) { ensure_init(); G__UNIT_WITHOUT_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT_ASK() -> Py { ensure_init(); G__VERDICT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT_ASK(v: Py) { ensure_init(); G__VERDICT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__WAKE_TO_FEED_KEY() -> Py { ensure_init(); G__WAKE_TO_FEED_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAKE_TO_FEED_KEY(v: Py) { ensure_init(); G__WAKE_TO_FEED_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g__WAKE_TO_FEED_WORDS() -> Py { ensure_init(); G__WAKE_TO_FEED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WAKE_TO_FEED_WORDS(v: Py) { ensure_init(); G__WAKE_TO_FEED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WATER_JUICE() -> Py { ensure_init(); G__WATER_JUICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATER_JUICE(v: Py) { ensure_init(); G__WATER_JUICE.with(|c| *c.borrow_mut() = v); }
pub fn g__WATER_NO_AGE() -> Py { ensure_init(); G__WATER_NO_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATER_NO_AGE(v: Py) { ensure_init(); G__WATER_NO_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__WATER_OVER_ONE() -> Py { ensure_init(); G__WATER_OVER_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATER_OVER_ONE(v: Py) { ensure_init(); G__WATER_OVER_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__WATER_UNDER_SIX() -> Py { ensure_init(); G__WATER_UNDER_SIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATER_UNDER_SIX(v: Py) { ensure_init(); G__WATER_UNDER_SIX.with(|c| *c.borrow_mut() = v); }
pub fn g__WHAT_TO_FEED() -> Py { ensure_init(); G__WHAT_TO_FEED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHAT_TO_FEED(v: Py) { ensure_init(); G__WHAT_TO_FEED.with(|c| *c.borrow_mut() = v); }
pub fn g__WHILE_SHE_FEEDS() -> Py { ensure_init(); G__WHILE_SHE_FEEDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHILE_SHE_FEEDS(v: Py) { ensure_init(); G__WHILE_SHE_FEEDS.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
pub fn g_not_the_heat_of_an_argument() -> Py { ensure_init(); G_not_the_heat_of_an_argument.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_not_the_heat_of_an_argument(v: Py) { ensure_init(); G_not_the_heat_of_an_argument.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Baby feeding: weaning, first foods, and what a baby can have.\n\nThe answers lane owns this domain. This is the food end of parenting: the questions people ask\nin the first two years, over and over and usually in a hurry - what do I start with, what can\nthey eat at this age, can my baby have this thing, how do I bring in an allergen. The facts are\nmainstream weaning guidance, kept as data so one fact answers every way of asking for it: \"can\nmy baby have yogurt\", \"is yoghurt ok for a 7 month old\" and \"can i give my baby yoghurt\" all\nreach the same row.\n\nThree tables carry the domain:\n\n- `STAGES` is the age table: what a baby eats and how much of it, from before weaning to a year\n  old. One row answers \"what can they eat\", \"how many meals a day\" and \"how much\" alike,\n  because all three are the same stage, and the age is read off the line when the answer is\n  built rather than keyed three times.\n- `FOODS` is the verdict table: one row per food people ask about, each with the answer as it\n  would be said - the verdict first, then the age, then the one thing that matters. A food with\n  no row is not answered here at all.\n- `NOT_A_MENU` and `NOT_A_BABY` are the stand-downs. A baby with a cold, a cot, a car seat or a\n  baby shower is not a baby with a menu, and a puppy is not a baby at all. Every row here stands\n  down for those, because a food table that answers \"my baby has petechiae on her legs\" is worse\n  than no table.\n\nThe other half of the same care is in the keys: a row needs a word of *eating*, not just a baby\nand an ask. \"what can i give a neighbour who just had a baby\" and \"how deep should the water be\nfor a baby bath\" have a baby and a question in them and nothing to do with food, and they are\nleft to the rows that answer them.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    s_not_the_heat_of_an_argument(py_func("not_the_heat_of_an_argument", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "not_the_heat_of_an_argument", args)));
    s_ANSWERS(py_tuple(vec![py_str("family-and-parenting"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("baby"), py_str("baby's"), py_str("babys"), py_str("babies"), py_str("infant"), py_str("infants"), py_str("little one"), py_str("little ones"), py_str("newborn"), py_str("newborns"), py_str("breastfed baby"), py_str("bottle fed baby"), py_str("weaning"), py_str("weaned")]) }
    s__BABY(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("month old"), py_str("months old"), py_str("week old"), py_str("weeks old"), py_str("newborn"), py_str("newborns"), py_str("1 year old"), py_str("one year old"), py_str("a year old"), py_str("eighteen month"), py_str("18 month"), py_str("two year old"), py_str("three year old"), py_str("four year old"), py_str("five year old"), py_str("six year old"), py_str("seven year old"), py_str("eight year old"), py_str("nine year old"), py_str("ten year old"), py_str("toddler"), py_str("toddlers"), py_str("month"), py_str("months"), py_str("baby is")]) }
    s_AGE(__data2());
    s_BABY(op_add(&g__BABY(), &g_AGE()));
    s__UNIT_WITHOUT_FIGURE(py_tuple(vec![py_str("month"), py_str("months")]));
    fn __data3() -> Py { py_tuple(vec![py_str("eat"), py_str("eats"), py_str("eating"), py_str("feed"), py_str("feeds"), py_str("feeding"), py_str("food"), py_str("foods"), py_str("solids"), py_str("meals"), py_str("meal"), py_str("wean"), py_str("weaning"), py_str("weaned"), py_str("diet"), py_str("portion"), py_str("portions"), py_str("puree"), py_str("purees"), py_str("finger food"), py_str("finger foods"), py_str("breakfast"), py_str("lunch"), py_str("tea"), py_str("supper"), py_str("snack"), py_str("snacks"), py_str("drink"), py_str("drinks"), py_str("milk"), py_str("menu")]) }
    s_EATING(__data3());
    s__THE_REFUSAL_WORDS(py_tuple(vec![py_str("spits"), py_str("spit"), py_str("spat")]));
    fn __data4() -> Py { py_tuple(vec![py_str("not breathing"), py_str("isnt breathing"), py_str("isn't breathing"), py_str("stopped breathing"), py_str("choking"), py_str("choked"), py_str("limp"), py_str("floppy"), py_str("unresponsive"), py_str("wont wake"), py_str("won't wake"), py_str("blue"), py_str("pale"), py_str("breathing"), py_str("breathe"), py_str("breathes"), py_str("breathless"), py_str("wheezing"), py_str("wheeze"), py_str("grunting"), py_str("grunt"), py_str("sucking in"), py_str("under the ribs"), py_str("fever"), py_str("temperature"), py_str("rash"), py_str("petechiae"), py_str("hives"), py_str("swollen"), py_str("swelling"), py_str("vomiting"), py_str("throwing up"), py_str("sick"), py_str("diarrhoea"), py_str("diarrhea"), py_str("constipation"), py_str("constipated"), py_str("blood"), py_str("reaction"), py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("off her food"), py_str("off his food"), py_str("off their food"), py_str("off food"), py_str("allergic reaction"), py_str("anaphylaxis"), py_str("a&e"), py_str("ambulance"), py_str("999"), py_str("111"), py_str("hospital"), py_str("doctor"), py_str("gp"), py_str("health visitor"), py_str("refusing feeds"), py_str("not feeding"), py_str("isnt feeding"), py_str("isn't feeding"), py_str("losing weight"), py_str("not putting on weight"), py_str("how often"), py_str("how many times"), py_str("wont eat"), py_str("won't eat"), py_str("not eating"), py_str("isnt eating"), py_str("isn't eating"), py_str("refuses"), py_str("refusing"), py_str("fussy"), py_str("picky"), py_str("hates"), py_str("spits"), py_str("throwing food"), py_str("throws food"), py_str("only eats"), py_str("cold"), py_str("cough"), py_str("blocked nose"), py_str("ear infection"), py_str("colic"), py_str("colicky"), py_str("windy"), py_str("wind"), py_str("gripe"), py_str("reflux"), py_str("possetting"), py_str("thrush"), py_str("eczema"), py_str("teething"), py_str("teeth"), py_str("jab"), py_str("jabs"), py_str("immunisation"), py_str("immunisations"), py_str("vaccination"), py_str("vaccinations"), py_str("calpol"), py_str("paracetamol"), py_str("ibuprofen"), py_str("dose"), py_str("doses"), py_str("medicine"), py_str("medicines"), py_str("tongue tie"), py_str("cradle cap"), py_str("bath"), py_str("bathing"), py_str("bathwater"), py_str("swim"), py_str("swimming"), py_str("pool"), py_str("cot"), py_str("crib"), py_str("moses"), py_str("pram"), py_str("buggy"), py_str("pushchair"), py_str("car seat"), py_str("carseat"), py_str("highchair"), py_str("high chair"), py_str("bib"), py_str("sterilise"), py_str("steriliser"), py_str("formula"), py_str("dummy"), py_str("nappy"), py_str("nappies"), py_str("wipes"), py_str("nap"), py_str("naps"), py_str("sleep"), py_str("sleeping"), py_str("crying"), py_str("unsettled"), py_str("routine"), py_str("nursery"), py_str("childminder"), py_str("creche"), py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("labour"), py_str("birth"), py_str("born"), py_str("shower"), py_str("christening"), py_str("gift"), py_str("present"), py_str("neighbour"), py_str("neighbor"), py_str("my sister"), py_str("my brother"), py_str("my cousin"), py_str("my friend"), py_str("my niece"), py_str("my nephew"), py_str("grandchild"), py_str("grandson"), py_str("granddaughter"), py_str("wont drink"), py_str("won't drink"), py_str("refuses to drink"), py_str("temp"), py_str("in a bottle"), py_str("in the bottle"), py_str("how much water"), py_str("how much juice")]) }
    s_NOT_A_MENU(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("how many feeds"), py_str("how many feed"), py_str("how many bottles"), py_str("how many ounces"), py_str("how many oz"), py_str("how many ml"), py_str("how many mls"), py_str("feeds a day"), py_str("bottles a day"), py_str("ounces of milk"), py_str("ml of milk"), py_str("how much formula"), py_str("how much expressed"), py_str("how much should i feed"), py_str("how much to feed"), py_str("how much should i be feeding"), py_str("how much should i give my baby"), py_str("how much milk"), py_str("how much breastmilk"), py_str("how much breast milk")]) }
    s__HOW_MUCH_MILK(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("puppy"), py_str("pup"), py_str("puppies"), py_str("dog"), py_str("dogs"), py_str("kitten"), py_str("kittens"), py_str("cat"), py_str("cats"), py_str("vet"), py_str("hamster"), py_str("guinea"), py_str("rabbit"), py_str("rabbits"), py_str("litter"), py_str("crate"), py_str("animal"), py_str("pets"), py_str("lamb"), py_str("calves"), py_str("foal"), py_str("chick"), py_str("parrot"), py_str("fish tank")]) }
    s_NOT_A_BABY(__data6());
    s_WEANING_ELSEWHERE(py_tuple(vec![py_str("wean"), py_str("weans"), py_str("weaned"), py_str("weaning"), py_str("finger food"), py_str("finger foods")]));
    fn __data7() -> Py { py_tuple(vec![py_str("a lump"), py_str("has a lump"), py_str("had a lump"), py_str("found a lump"), py_str("under the skin"), py_str("armpit"), py_str("armpits"), py_str("neck"), py_str("groin"), py_str("head"), py_str("breast"), py_str("testicle"), py_str("testicles"), py_str("scrotum"), py_str("gum"), py_str("gums"), py_str("cheek"), py_str("swollen"), py_str("swelling"), py_str("bump"), py_str("bumps"), py_str("back"), py_str("bottom"), py_str("tummy"), py_str("leg"), py_str("thigh"), py_str("shoulder"), py_str("chest"), py_str("forehead"), py_str("under the arm"), py_str("in the nappy area"), py_str("skin")]) }
    s_A_BODY_LUMP(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("contract"), py_str("contracts"), py_str("membership"), py_str("memberships"), py_str("subscription"), py_str("subscriptions"), py_str("tenancy"), py_str("lease"), py_str("warranty"), py_str("insurance"), py_str("loan"), py_str("loans"), py_str("instalment"), py_str("instalments"), py_str("repayments"), py_str("gym"), py_str("phone contract"), py_str("mobile contract"), py_str("broadband"), py_str("cancellation"), py_str("notice period"), py_str("without an r"), py_str("with an r"), py_str("time of year")]) }
    s_NOT_AN_AGE(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("cinnamon"), py_str("cumin"), py_str("turmeric"), py_str("paprika"), py_str("nutmeg"), py_str("coriander"), py_str("cardamom"), py_str("saffron"), py_str("curry powder"), py_str("chilli powder"), py_str("chili powder"), py_str("black pepper"), py_str("white pepper"), py_str("mixed spice"), py_str("allspice"), py_str("garam masala"), py_str("spices"), py_str("spice"), py_str("herbs"), py_str("herb"), py_str("mild curry")]) }
    s_SPICE_NAMES(__data9());
    s_NOT_ABOUT_WATER(py_tuple(vec![py_str("dehydrate"), py_str("dehydrates"), py_str("dehydrated"), py_str("dehydration"), py_str("dehydrating"), py_str("hydration"), py_str("diuretic"), py_str("electrolytes")]));
    fn __data10() -> Py { py_tuple(vec![py_str("job"), py_str("jobs"), py_str("my job"), py_str("new job"), py_str("job offer"), py_str("leave a job"), py_str("leaving a job"), py_str("left a job"), py_str("quit my job"), py_str("quitting my job"), py_str("boss"), py_str("employer"), py_str("employee"), py_str("career"), py_str("resign"), py_str("resigning"), py_str("resignation"), py_str("hand in my notice"), py_str("handing in my notice"), py_str("notice period"), py_str("interview"), py_str("salary"), py_str("my salary")]) }
    s_JOB_WORDS(__data10());
    s_STAND_DOWN(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_A_MENU(), &g__HOW_MUCH_MILK()), &g_NOT_A_BABY()), &g_WEANING_ELSEWHERE()), &g_NOT_AN_AGE()), &g_SPICE_NAMES()), &g_NOT_ABOUT_WATER()), &g_JOB_WORDS()));
    s_BOOK_WORDS(py_tuple(vec![py_str("book"), py_str("books"), py_str("novel"), py_str("picture book"), py_str("story"), py_str("stories"), py_str("read to"), py_str("reading to"), py_str("read aloud")]));
    s_STAGES(py_tuple(vec![py_dict(vec![(py_str("from"), Py::Int(0)), (py_str("until"), Py::Int(5)), (py_str("reply"), py_str("Milk is all a baby needs before about six months, and food before four months is not recommended at all. If they seem hungrier, more milk is the answer - ask your health visitor before starting anything early."))]), py_dict(vec![(py_str("from"), Py::Int(6)), (py_str("until"), Py::Int(8)), (py_str("reply"), py_str("At this age milk is still the main food, so treat eating as practice: two or three small goes a day of soft food they can hold - cooked carrot or broccoli, toast fingers, banana, plain yoghurt, mashed lentils. One new food at a time, a few days apart, so a reaction is easy to spot."))]), py_dict(vec![(py_str("from"), Py::Int(9)), (py_str("until"), Py::Int(11)), (py_str("reply"), py_str("By now it is three meals a day plus milk, and what they eat starts to matter: chopped versions of your own meal, soft meat or fish with the bones out, pasta, beans and lentils, fruit, full-fat dairy. No added salt or sugar, and no honey before the first birthday."))]), py_dict(vec![(py_str("from"), Py::Int(12)), (py_str("until"), Py::None), (py_str("reply"), py_str("From one they can eat what the rest of the family eats: three meals and two snacks, whole cows milk as a drink at about 350 to 400ml a day, and no added salt. Honey is fine from now on."))])]));
    s__NUMBER_WORDS(py_tuple(vec![py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("six"), py_str("seven"), py_str("eight"), py_str("nine"), py_str("ten"), py_str("eleven"), py_str("twelve")]));
    fn __data12() -> Py { py_tuple(vec![py_str("cheese"), py_str("cheddar"), py_str("cheeses"), py_str("parmesan"), py_str("mozzarella"), py_str("cream cheese"), py_str("brie"), py_str("camembert"), py_str("blue cheese"), py_str("stilton"), py_str("gorgonzola"), py_str("roquefort"), py_str("mould ripened"), py_str("mold ripened"), py_str("soft cheese"), py_str("goats cheese"), py_str("feta")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("tea"), py_str("teas"), py_str("green tea"), py_str("herbal tea"), py_str("camomile tea"), py_str("chamomile tea"), py_str("peppermint tea"), py_str("fruit tea"), py_str("coffee"), py_str("cola"), py_str("coke"), py_str("energy drink"), py_str("energy drinks"), py_str("caffeine"), py_str("fizzy drink"), py_str("fizzy drinks"), py_str("diet coke")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers"), py_str("teen"), py_str("teens"), py_str("teenager"), py_str("teenagers"), py_str("school"), py_str("year old"), py_str("12"), py_str("under 12"), py_str("10"), py_str("14"), py_str("dehydrat"), py_str("dehydrate"), py_str("dehydrates"), py_str("dehydrated"), py_str("dehydrating"), py_str("dehydration"), py_str("hydrate"), py_str("hydrated"), py_str("hydration")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("litre"), py_str("litres"), py_str("2 year"), py_str("3 year"), py_str("4 year"), py_str("5 year"), py_str("child"), py_str("children"), py_str("kids"), py_str("i drink"), py_str("should i drink"), py_str("how much should i")]) }
    fn __data16() -> Py { py_tuple(vec![py_str("salt"), py_str("salty"), py_str("stock cube"), py_str("stock cubes"), py_str("gravy"), py_str("bacon"), py_str("ham"), py_str("sausages"), py_str("sausage"), py_str("salted"), py_str("marmite"), py_str("yeast extract"), py_str("bovril"), py_str("soy sauce"), py_str("baked beans"), py_str("beans on toast"), py_str("tomato ketchup"), py_str("ketchup"), py_str("brown sauce"), py_str("mayonnaise"), py_str("mayo"), py_str("soup"), py_str("soups"), py_str("gravy granules"), py_str("stock"), py_str("salted butter")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("cucumber"), py_str("cucumbers"), py_str("tomato"), py_str("tomatoes"), py_str("bell pepper"), py_str("bell peppers"), py_str("red pepper"), py_str("green pepper"), py_str("sweet pepper"), py_str("lettuce"), py_str("celery"), py_str("radish"), py_str("radishes"), py_str("spring onion"), py_str("spring onions"), py_str("beetroot"), py_str("raw carrot"), py_str("carrot sticks")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("toast"), py_str("bread"), py_str("crumpet"), py_str("pitta"), py_str("cracker"), py_str("crackers"), py_str("chapati"), py_str("chapatis"), py_str("naan"), py_str("roti"), py_str("wrap"), py_str("wraps"), py_str("bagel"), py_str("bagels"), py_str("scone"), py_str("scones"), py_str("teacake")]) }
    fn __data19() -> Py { py_tuple(vec![py_str("broccoli"), py_str("cauliflower"), py_str("courgette"), py_str("carrot"), py_str("carrots"), py_str("sweet potato"), py_str("butternut squash"), py_str("parsnip"), py_str("peas"), py_str("green beans"), py_str("spinach"), py_str("kale"), py_str("leeks"), py_str("swede"), py_str("turnip"), py_str("asparagus"), py_str("plantain"), py_str("mushroom"), py_str("mushrooms"), py_str("mange tout"), py_str("sugar snap peas"), py_str("runner beans"), py_str("cabbage"), py_str("brussels sprouts"), py_str("cauliflower cheese")]) }
    fn __data20() -> Py { py_tuple(vec![py_str("mango"), py_str("melon"), py_str("watermelon"), py_str("peach"), py_str("pear"), py_str("plum"), py_str("nectarine"), py_str("blueberries"), py_str("raspberries"), py_str("kiwi"), py_str("kiwis"), py_str("pineapple"), py_str("coconut"), py_str("grated coconut"), py_str("strawberry"), py_str("strawberries"), py_str("blackberries"), py_str("cherries"), py_str("apricot"), py_str("apricots"), py_str("papaya"), py_str("fig"), py_str("figs"), py_str("orange"), py_str("satsuma"), py_str("mandarin"), py_str("clementine")]) }
    fn __data11() -> Py { py_tuple(vec![py_dict(vec![(py_str("names"), py_tuple(vec![py_str("yogurt"), py_str("yoghurt"), py_str("yoghurts"), py_str("yogurts"), py_str("fromage frais"), py_str("greek yogurt"), py_str("greek yoghurt"), py_str("kefir"), py_str("skyr")])), (py_str("reply"), py_str("Yes, plain full-fat yoghurt is fine from six months. Buy natural or Greek rather than the children's fruit pots, which are mostly sugar - stir in mashed fruit instead - and give it with a meal rather than as a drink."))]), py_dict(vec![(py_str("names"), __data12()), (py_str("reply"), py_str("Yes, from six months, in small amounts, because cheese is salty: about 25g a day is plenty for a little one. Stick to pasteurised cheese - brie, camembert and blue cheese are the ones to leave raw, and they are fine once cooked until steaming hot."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("scrambled egg"), py_str("boiled egg"), py_str("omelette")])), (py_str("reply"), py_str("Yes, well cooked, from six months: scrambled egg, a hard-boiled egg mashed with a fork, or omelette cut into fingers. An egg with the red lion stamp can be given runny because those hens are vaccinated; without the stamp, cook it until white and yolk are both solid."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("strawberry"), py_str("strawberries"), py_str("raspberry"), py_str("raspberries"), py_str("blueberry"), py_str("blueberries"), py_str("berries"), py_str("blackberries")])), (py_str("reply"), py_str("Yes, from six months - cut them in half or quarters, because a round berry is a choking shape. Berries are acidic, so they can bring on a red nappy rash, and that is not an allergy; a rash round the mouth that fades on its own is usually the same thing."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("peanut butter"), py_str("peanut"), py_str("peanuts"), py_str("nut butter"), py_str("almond butter"), py_str("nut"), py_str("nuts"), py_str("almonds"), py_str("cashews"), py_str("almond")])), (py_str("reply"), py_str("Yes, and early is better than late: from six months, a thin smear of smooth peanut butter on toast or stirred into porridge. Never a spoonful on its own and never a whole nut - those are choking shapes, so use ground or butter only. Offer it on a day they are well, in the morning, then keep it in their week."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("raisins"), py_str("raisin"), py_str("sultanas"), py_str("sultana"), py_str("dried fruit"), py_str("dried apricots"), py_str("prunes"), py_str("dates"), py_str("currants"), py_str("cranberries"), py_str("dried cranberries")])), (py_str("reply"), py_str("Yes, from six months, but chop them: a whole raisin is the shape that blocks a small throat and it is sticky enough to sit on the teeth, so cut each one in half, give them with a meal rather than as a snack on their own, and keep them to a small handful a day. Dried fruit counts as one of the five a day, so it is not a substitute for fresh fruit - and never dried fruit with added sugar or yoghurt coating."))]), py_dict(vec![(py_str("names"), __data13()), (py_str("extra"), __data14()), (py_str("bias"), Py::Int(3)), (py_str("reply"), py_str("No, not as a drink: tea and coffee both carry caffeine a small body does not need, and the tannins in tea stop iron being absorbed, which matters most in the first two years. Cola, energy drinks and fizzy drinks are the same answer with sugar on top, and herbal teas are no better for a baby. Milk and water are the drinks: breast milk or formula before one, whole cows milk from one, and water in an open cup with meals from six months."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("liver"), py_str("lamb's liver"), py_str("chicken liver"), py_str("kidney"), py_str("pate"), py_str("liver pate"), py_str("paté")])), (py_str("reply"), py_str("Liver is fine from six months, but once a week at most: it is packed with vitamin A and iron, and too much vitamin A builds up. Cook it through, mash or blend it into something they already eat, and count liver, liver pate and cod liver oil as the same thing in that week. Pate itself is salty, so leave that until after the first birthday and then keep it to a thin scrape."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("crisps"), py_str("crisp"), py_str("chips"), py_str("wotsits"), py_str("quavers"), py_str("cheese puffs")])), (py_str("reply"), py_str("No, not before one, and rarely after: crisps are salty and they are a choking shape, and the same goes for chips with salt on. A handful of soft veg sticks, toast fingers or a bit of cheese is the same idea without the salt."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("wheat"), py_str("soya"), py_str("soy"), py_str("sesame"), py_str("gluten")])), (py_str("reply"), py_str("Yes, from six months, and early is the point: wheat, soya and sesame are three of the allergens to give in the first year, one at a time, on a day they are well and in the morning. Bread, pasta, cereal, tofu and tahini all count, and once a food is in, keep giving it a couple of times a week rather than stopping."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("cows milk"), py_str("cow's milk"), py_str("whole milk"), py_str("full fat milk"), py_str("semi skimmed milk")])), (py_str("reply"), py_str("Not as a drink before twelve months: it has too little iron and too much salt and protein for a baby. You can cook with it, or put a splash on cereal, from six months. From one, give whole milk, about 350 to 400ml a day."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("water"), py_str("juice"), py_str("fruit juice"), py_str("squash"), py_str("cordial"), py_str("orange juice"), py_str("apple juice"), py_str("smoothie"), py_str("smoothies"), py_str("milkshake"), py_str("milkshakes")])), (py_str("own_ask"), py_func("_the_water_ask", |args: &[Py]| _the_water_ask(args))), (py_str("extra"), __data15()), (py_str("reply"), py_str("Not before six months - milk is all they need, and extra water can upset their salts. From six months, water is the drink to offer in an open cup with meals, and juice or squash is not needed at any age: if you give it, wait until twelve months, dilute it, and keep it to a meal in a cup rather than a bottle."))]), py_dict(vec![(py_str("names"), __data16()), (py_str("reply"), py_str("No salt at all in their food before one, and very little after: a baby's kidneys cannot cope with it. That rules out stock cubes, gravy granules, bacon, ham, sausages and most sauces - cook their portion separately and season yours."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("sugar"), py_str("sweets"), py_str("chocolate"), py_str("biscuits"), py_str("biscuit"), py_str("cake"), py_str("cakes"), py_str("jelly"), py_str("jam"), py_str("marmalade"), py_str("rice pudding"), py_str("custard")])), (py_str("reply"), py_str("There is no need for sugar at all before two, and it is worth keeping it that way while you can, because it is what makes them prefer sweet food later. Fruit is the sweet thing to give, and cheese or toast makes a better snack than a biscuit."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("grapes"), py_str("grape"), py_str("cherry tomatoes"), py_str("cherries"), py_str("olives"), py_str("hot dog"), py_str("hot dogs")])), (py_str("reply"), py_str("Yes, but cut them: grapes, cherry tomatoes and olives go lengthways into quarters and sausages lengthways into strips, because the round shape is what blocks a small throat. Whole grapes are the food toddlers choke on most, and popcorn is not one to give before about four."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("popcorn"), py_str("pop corn"), py_str("puffed corn"), py_str("corn snacks")])), (py_str("reply"), py_str("No, not before about four: popcorn is light, hard and the exact shape that lodges in a small airway, and it is the food toddlers choke on most after grapes. Puffed corn snacks are the same shape and salty as well, so leave those too, and keep crisps for an occasional treat after one, sitting down."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rice cake"), py_str("rice cakes"), py_str("corn cake"), py_str("corn cakes"), py_str("oat cake"), py_str("oat cakes"), py_str("crackers"), py_str("cracker"), py_str("breadsticks"), py_str("bread sticks"), py_str("rusks"), py_str("rusk")])), (py_str("reply"), py_str("Yes, from six months, and they are good for the pincer grip - but read the packet first, because they are often the saltiest thing in the baby aisle. Unsalted rice cakes, plain oatcakes and no-salt breadsticks are the ones to pick; a rusk is mostly sugar, so treat it as a biscuit rather than a food."))]), py_dict(vec![(py_str("names"), __data17()), (py_str("reply"), py_str("Yes, from six months, raw or cooked: cucumber and tomato cut into finger-length batons or quarters are two of the easiest first finger foods, because a baby can hold them and gum them. Cherry tomatoes and olives go lengthways into quarters, and celery strings are worth peeling off before you hand it over."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("tofu"), py_str("bean curd"), py_str("edamame"), py_str("soya bean"), py_str("soy bean"), py_str("tempeh")])), (py_str("reply"), py_str("Yes, from six months, cooked through: tofu cut into strips is soft enough to gum, and it takes on whatever it is cooked in. It is soya, so introduce it the way you would peanut - a little on its own the first time, then watch for two hours - and keep it plain rather than in a salty marinade."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("quorn"), py_str("mycoprotein"), py_str("quorn mince"), py_str("quorn pieces"), py_str("meat free mince"), py_str("veggie mince"), py_str("vegetarian mince")])), (py_str("reply"), py_str("Yes, from six months, cooked through and chopped small: Quorn is mycoprotein, a fungal protein rather than soya, and the plain mince and pieces are low in salt and fine for a baby. It is a protein swap rather than a whole food, so give it with something iron-rich - lentils, beans or a little meat - and read the packet on the breaded and ready-made versions, which are salty."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("sushi"), py_str("sashimi"), py_str("raw fish"), py_str("raw prawns"), py_str("smoked salmon"), py_str("smoked mackerel"), py_str("gravadlax"), py_str("gravalax"), py_str("cured fish"), py_str("oysters")])), (py_str("reply"), py_str("Sushi is fine once the fish in it is cooked: a cooked prawn or salmon roll, or a vegetable one, is a good finger food from six months. Raw fish and raw shellfish wait until they are five, because of the parasites and the bones. Smoked salmon is cured rather than cooked and it is salty, so a flake now and then is the most of it.")), (py_str("bias"), Py::Int(2))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("fish"), py_str("salmon"), py_str("tuna"), py_str("cod"), py_str("prawns"), py_str("shellfish"), py_str("prawn"), py_str("mussels")])), (py_str("reply"), py_str("Yes, well cooked and with every bone out - flake it and check between your fingers first. Oily fish like salmon once or twice a week is worth having. Leave out shark, swordfish and marlin, and keep tuna to about one small tin a week."))]), py_dict(vec![(py_str("names"), __data18()), (py_str("reply"), py_str("Yes, from six months - toast fingers are one of the easiest first foods because a baby can hold them. White, wholemeal or a mix is fine; bread has salt in it already, which is a reason not to add more, not a reason to wait.")), (py_str("pronoun"), Py::Bool(true)), (py_str("extra"), py_tuple(vec![py_str("a toast"), py_str("raise a glass"), py_str("raise the glass"), py_str("speech"), py_str("speeches"), py_str("wedding"), py_str("best man"), py_str("eulogy"), py_str("few words"), py_str("say a few words")]))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("banana"), py_str("bananas")])), (py_str("reply"), py_str("Yes, from six months, and it is one of the easiest first foods: mash it or cut it into finger-length pieces a baby can hold. It is sweet and soft, so it goes down on a day when nothing else does."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("avocado"), py_str("avocados"), py_str("guacamole"), py_str("avocado pear")])), (py_str("reply"), py_str("Yes, from six months: ripe avocado mashed on toast fingers, or in a wedge they can hold with the skin as a handle. The fat in it is the kind a growing baby needs, and its blandness sits well next to a stronger taste."))]), py_dict(vec![(py_str("names"), __data19()), (py_str("reply"), py_str("Yes, from six months, cooked soft enough to squash between your fingers - a whole floret or a stick of carrot is a handle a baby can hold and gum. Keep offering the ones they refuse: it can take ten tries before a vegetable is eaten."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("mash")])), (py_str("reply"), py_str("Yes, from six months: mashed potato with a little of their usual milk or unsalted butter, or soft roast potato in strips. No salt in the water or on top, and skip chips and crisps, which are salty and hard to manage."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("pasta"), py_str("fusilli"), py_str("penne"), py_str("macaroni"), py_str("spaghetti"), py_str("noodles")])), (py_str("reply"), py_str("Yes, from about six months once they can pick things up: shapes like fusilli or penne are easier to hold than spaghetti, and cook them a minute past the packet so they are soft. Cut long pasta, and keep the sauce low in salt."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("rice"), py_str("couscous"), py_str("quinoa"), py_str("bulgur")])), (py_str("reply"), py_str("Yes, from six months, cooked soft - as a mash, or pressed into a sticky ball they can pick up. No salt in the water. Cooked rice is the one to be careful with: cool it fast, fridge it within the hour, and reheat it once, hot all through."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("hummus"), py_str("houmous"), py_str("chickpeas")])), (py_str("reply"), py_str("Yes, from six months: spread it thin on toast fingers or let them dip a finger of pitta in it, and it counts as a portion of protein. Shop hummus can be salty, so a plain one or your own is better."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("lentils"), py_str("dhal"), py_str("daal"), py_str("beans"), py_str("kidney beans"), py_str("chick pea"), py_str("pulses")])), (py_str("reply"), py_str("Yes, from six months: red lentils cook down into a thick dhal they can eat from a spoon or with bread, and mashed beans go into a sauce the same way. Pulses are a good source of iron, and a squeeze of lemon or some tomato helps the iron go in."))]), py_dict(vec![(py_str("names"), __data20()), (py_str("reply"), py_str("Yes, from six months: soft ripe fruit in strips or wedges they can hold, and a wedge with the skin on makes a handle to gnaw. Round fruit is the one to cut - berries in half, grapes lengthways - and stay with them while they eat it."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("chicken"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("mince"), py_str("meat")])), (py_str("reply"), py_str("Yes, from six months: soft cooked meat in strips they can hold, or minced into a sauce, and it is the best source of the iron a baby uses up at about six months. A strip of well-cooked chicken or a lamb chop bone is a good thing to gnaw, with the gristle and small bones taken out."))]), py_dict(vec![(py_str("names"), py_tuple(vec![py_str("porridge"), py_str("oats"), py_str("cereal"), py_str("baby rice")])), (py_str("extra"), py_tuple(vec![py_str("bottle"), py_str("bottles"), py_str("in a bottle"), py_str("in the bottle"), py_str("bottle feed")])), (py_str("reply"), py_str("Yes, from six months, and ordinary porridge oats made with their usual milk are as good as any baby cereal, with no added sugar or salt. Baby rice is almost all starch, so it fills them up without feeding them much."))])]) }
    s_FOODS(__data11());
    s__FIRST_FOODS(py_str("Start with soft food they can hold themselves rather than something on a spoon: cooked carrot or broccoli, toast fingers, ripe pear, banana, avocado, or a spoon of plain yoghurt. One food at a time, a few days apart, so a reaction is easy to spot, and milk first so they are not too hungry to try. How much is a few spoonfuls or a few sticks at the start, half a fist by nine months and about a fist at a year. Around six months is the age, unless a health visitor has said otherwise."));
    fn __data21() -> Py { py_tuple(vec![py_str("what can"), py_str("what should"), py_str("what do"), py_str("what to"), py_str("what shall"), py_str("what could"), py_str("what will"), py_str("what food"), py_str("what foods"), py_str("what food do i"), py_str("how much"), py_str("how many"), py_str("how often"), py_str("when can"), py_str("when should"), py_str("when do"), py_str("is it ok to"), py_str("can i give"), py_str("can i feed"), py_str("can i start"), py_str("can they have"), py_str("can she have"), py_str("can he have"), py_str("where do i start")]) }
    s__WHAT_TO_FEED(__data21());
    s__AGE_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2})\\s*-?\\s*(month|months|mth|mths|year|years|yr|yrs)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NUMBER_WORDS_AT({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("enumerate", &[g__NUMBER_WORDS()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk7442 = py_unpack(&__item.clone(), 2, None); let mut index: Py = __unpk7442[0].clone(); let mut word: Py = __unpk7442[1].clone(); __out.push((word.clone(), op_add(&index.clone(), &Py::Int(1)))); } py_dict(__out) })() });
    s__MILK_WORDS(py_tuple(vec![py_str("milk"), py_str("breastmilk"), py_str("breast milk"), py_str("expressed milk"), py_str("expressed")]));
    fn __data22() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how often"), py_str("much milk"), py_str("much breastmilk"), py_str("many ounces"), py_str("how many oz"), py_str("what milk"), py_str("which milk"), py_str("what kind of milk"), py_str("what type of milk"), py_str("best milk"), py_str("right milk")]) }
    s__MILK_HOW_MUCH(__data22());
    fn __data23() -> Py { py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("2 year"), py_str("two year"), py_str("3 year"), py_str("three year"), py_str("4 year"), py_str("four year"), py_str("5 year"), py_str("five year"), py_str("child"), py_str("children"), py_str("kids"), py_str("should i drink"), py_str("i drink"), py_str("i have"), py_str("can i have"), py_str("adult"), py_str("pump"), py_str("pumping"), py_str("recipe"), py_str("latte"), py_str("cappuccino"), py_str("protein"), py_str("calcium")]) }
    s__NOT_A_BABY_MILK(__data23());
    s__A_DRINK(py_tuple(vec![py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("have"), py_str("has"), py_str("need"), py_str("needs"), py_str("take"), py_str("takes")]));
    s__A_COUNT(py_tuple(vec![py_str("how many ounces"), py_str("how many oz"), py_str("how many ml"), py_str("how many mls"), py_str("ounces"), py_str("oz"), py_str("ml"), py_str("mls")]));
    s__THE_CHILD(py_tuple(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("they"), py_str("them")]));
    s__MILK_UNDER_SIX(py_str("Milk is the whole diet before six months, so there is no set amount: eight to twelve feeds in 24 hours, on demand, day and night. Six or more wet nappies a day and a baby who settles after a feed mean they are getting enough, and the scales settle it better than the ounces. Nothing else goes in the bottle - no cereal, no water unless it is hot."));
    s__MILK_SIX_TO_TWELVE(py_str("About 500 to 600ml (17 to 20oz) of breast milk or formula a day from six to twelve months, in four or five feeds, and food goes alongside it rather than instead of it: milk is still the main drink until the first birthday, and the wet nappies still say whether it is enough."));
    s__MILK_OVER_ONE(py_str("About 350 to 400ml (12oz) a day of whole cows milk from one, in a cup rather than a bottle: that much covers the calcium without filling them up before meals, and more than about 500ml crowds out the iron in food."));
    s__WATER_UNDER_SIX(py_str("Under six months, none: milk is all they need and extra water can upset the salts in a small body. From six months, offer a little water in an open cup with meals - a few sips is enough, and they will take what they need rather than a set amount. From one, water with every meal and between meals as they ask for it, and that is when a cup measuring roughly a litre a day starts to make sense."));
    s__WATER_OVER_ONE(py_str("Water with every meal and between meals whenever they ask for it: a cup measuring roughly a litre a day is the toddler figure, and the milk they drink counts towards it. Leave the cup where they can reach it and refill it without a fuss. Under six months it was none, because extra water upsets the salts in a small body, so do not go back to that unless a doctor says so."));
    s__WATER_NO_AGE(py_str("Under six months, none: milk is all they need and extra water can upset the salts in a small body. From six months, offer a little water in an open cup with meals - a few sips is enough, and they will take what they need rather than a set amount. From one, water with every meal and between meals as they ask for it, and that is when a cup measuring roughly a litre a day starts to make sense."));
    s__WATER_JUICE(py_str(" Juice and squash are not needed at any age: if you give juice at all, wait until twelve months, dilute it, and keep it to a meal in a cup."));
    s__DRINK_UNDER_SIX(py_str("Milk is the only drink before six months - breast milk or formula, on demand, and nothing else in the bottle, not even water."));
    s__DRINK_SIX_TO_TWELVE(py_str("Milk is still the main drink at this age: breast milk or formula on demand, and water in an open cup with meals from six months. A free-flow cup or a straw cup is better than a bottle from six months, since the sucking is what makes the fuss. Juice and squash are not needed at any age; if you give juice at all, wait until twelve months, dilute it, and keep it to a meal."));
    s__DRINK_OVER_ONE(py_str("About a litre a day for a one-year-old, and the milk they drink counts towards it: whole cows milk at roughly 350 to 400ml of that, in a cup rather than a bottle, and water with every meal and whenever they ask. Leave the cup where they can reach it and refill it without a fuss. Juice and squash are not needed; if you give juice, dilute it and keep it to a meal in a cup."));
    s__SNACK(py_tuple(vec![py_str("snack"), py_str("snacks"), py_str("snacking"), py_str("a snack"), py_str("between meals"), py_str("in between meals"), py_str("little and often")]));
    s__SNACK_CHILD(op_add(&op_add(&g__BABY(), &g_AGE()), &py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("1 year old"), py_str("2 year old"), py_str("3 year old"), py_str("4 year old"), py_str("school age")])));
    fn __data24() -> Py { py_tuple(vec![py_str("party"), py_str("parties"), py_str("buffet"), py_str("guests"), py_str("film"), py_str("films"), py_str("movie"), py_str("match"), py_str("half time"), py_str("picnic"), py_str("me"), py_str("myself"), py_str("adult"), py_str("adults"), py_str("for the adults"), py_str("recipe"), py_str("make ahead"), py_str("christmas"), py_str("birthday"), py_str("wedding"), py_str("work")]) }
    s__NOT_A_SNACK(__data24());
    s__SNACK_UNDER_SIX(py_str("No snacks before six months: milk is the whole of it, and a baby that young does not need anything between feeds, only more milk."));
    s__SNACK_SIX_TO_TWELVE(py_str("From six months, two or three small goes a day at set times rather than all day long, and the best snacks are the ones they can hold: toast fingers, soft fruit, cooked veg sticks, plain yoghurt, a little cheese, oatcakes. Keep away from the choking shapes - whole grapes, whole nuts, popcorn - and sit them down while they eat."));
    s__SNACK_OVER_ONE(py_str("Two snacks a day between meals is right from one: fruit, plain yoghurt, toast, oatcakes, cheese, veg sticks. Keep them at set times rather than in the hand all afternoon, because a child who grazes does not sit down for the meal, and keep the choking shapes - whole grapes, whole nuts, popcorn - for later."));
    s__PLANT_MILK_BABY(py_str("Not as their main drink, no. A baby under a year needs breast milk or first infant formula, and a plant milk does not have the protein, fat, calcium or vitamins to stand in for it. In cereal, porridge or cooking from six months it is fine - choose one with no sugar and with calcium and vitamin D added."));
    s__PLANT_MILK_RICE(py_str("Rice milk is the one to leave out completely before five years: it has arsenic in it, which a small body clears slowly, and a child drinking it daily can take in too much. Oat, soya or coconut milk, unsweetened and fortified, are the better swaps over one."));
    s__PLANT_MILK_CHILD(py_str("Yes, as a drink now - unsweetened and with calcium and vitamin D added, and it does not replace their food. Soya is the only one with protein close to cows milk, so it is the usual swap; rice milk still waits until five."));
    fn __data25() -> Py { py_tuple(vec![py_str("have"), py_str("has"), py_str("give"), py_str("gives"), py_str("giving"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("feed"), py_str("drink"), py_str("drinks"), py_str("try"), py_str("ok"), py_str("safe"), py_str("allowed"), py_str("much"), py_str("many"), py_str("any"), py_str("can i give"), py_str("can i feed"), py_str("can i put"), py_str("can i try"), py_str("am i allowed"), py_str("is it ok to"), py_str("should i give"), py_str("ok to give"), py_str("safe to give"), py_str("when can"), py_str("when should")]) }
    s__VERDICT_ASK(__data25());
    fn __data26() -> Py { py_tuple(vec![py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("boy"), py_str("boys"), py_str("girl"), py_str("girls"), py_str("little boy"), py_str("little girl"), py_str("grandchild"), py_str("grandchildren"), py_str("grandson"), py_str("granddaughter"), py_str("stepson"), py_str("stepdaughter")]) }
    s__A_CHILD(__data26());
    s__NAMES_A_BABY(op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BABY()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__UNIT_WITHOUT_FIGURE(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g__A_CHILD()));
    s__FOR_A_GROWN_UP(py_tuple(vec![py_str("adult"), py_str("adults"), py_str("grown up"), py_str("grown ups"), py_str("grownup"), py_str("grownups"), py_str("for a grown up"), py_str("for adults"), py_str("is it ok for me"), py_str("safe for me")]));
    s__NOT_A_BABY_NAME(py_tuple(vec![py_str("baby rice"), py_str("baby porridge"), py_str("baby cereal"), py_str("baby food"), py_str("baby foods"), py_str("baby jar"), py_str("baby jars"), py_str("baby pouch"), py_str("baby pouches"), py_str("baby porridge pot")]));
    s__A_RATE_IN_MONTHS(stdlib_call("re", "compile", &[py_str("\\b(?:a|per|each|every|the)\\s+month\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_BABY_TILL(Py::Int(36));
    s__A_CHILD_PRONOUN(py_tuple(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("his"), py_str("hers")]));
    s__A_ORGAN(py_str("(?:liver|kidneys?|hearts?)"));
    s__A_CONDITION(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b("), &g__A_ORGAN()), &py_str(")\\b(?:\\W+\\w+){0,2}?\\W+(?:problems?|conditions?|disease|failure|infections?|damage|stones?|transplants?|surgery|operation|disorder|dialysis|reflux)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data27() -> Py { py_tuple(vec![py_str("mum"), py_str("mummy"), py_str("mom"), py_str("mommy"), py_str("mother"), py_str("dad"), py_str("daddy"), py_str("father"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandmother"), py_str("grandad"), py_str("granddad"), py_str("grandpa"), py_str("grandfather"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("carer"), py_str("neighbour"), py_str("neighbor"), py_str("friend"), py_str("mam"), py_str("sister"), py_str("brother"), py_str("boss"), py_str("colleague")]) }
    s__AN_ADULT(__data27());
    fn __data28() -> Py { py_tuple(vec![py_str("lump"), py_str("lumps"), py_str("lumpy"), py_str("wean"), py_str("weans"), py_str("weaning"), py_str("weaned"), py_str("puree"), py_str("purees"), py_str("mash"), py_str("mashes"), py_str("mashed"), py_str("spoon"), py_str("spoons"), py_str("spoonful"), py_str("bottle"), py_str("bottles"), py_str("sippy"), py_str("finger food"), py_str("solids"), py_str("texture"), py_str("textures")]) }
    s__A_TABLE_WORD(__data28());
    s__A_TABLE_FOOD(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOODS()) { let mut food: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&food.clone(), &py_str("names"))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data29() -> Py { py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("son"), py_str("daughter"), py_str("boy"), py_str("girl"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("lad"), py_str("lass")]) }
    s__OLDER_CHILD_WORD(__data29());
    s__A_BARE_YEAR_FIGURE(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|aged|at)\\s+(?:two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|\\d{1,2})\\b(?!\\s*(?:month|months|week|weeks|wk|wks|mth|mths|day|days))")]).unwrap_or_else(|e| exc::raise(e)));
    s__SELF_PRONOUN(py_tuple(vec![py_str("i"), py_str("i'm"), py_str("im"), py_str("me"), py_str("we"), py_str("we're"), py_str("you"), py_str("you're"), py_str("my"), py_str("our")]));
    fn __data30() -> Py { py_tuple(vec![py_str("eat"), py_str("eats"), py_str("eating"), py_str("ate"), py_str("have"), py_str("has"), py_str("having"), py_str("had"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("drank"), py_str("cook"), py_str("cooks"), py_str("cooking"), py_str("make"), py_str("makes"), py_str("making"), py_str("try"), py_str("tries"), py_str("trying"), py_str("buy"), py_str("buying")]) }
    s__OWN_EATING(__data30());
    s__FOR_ME(py_tuple(vec![py_str("bad for you"), py_str("good for you"), py_str("healthy for you"), py_str("unhealthy for you"), py_str("for me"), py_str("my lunch"), py_str("my dinner"), py_str("my breakfast"), py_str("my tea"), py_str("on my plate")]));
    fn __data31() -> Py { py_tuple(vec![py_str("breastfeed"), py_str("breastfeeds"), py_str("breastfeeding"), py_str("breast feed"), py_str("breast feeding"), py_str("formula"), py_str("wean"), py_str("weans"), py_str("weaning"), py_str("weaned"), py_str("milk feed"), py_str("milk feeds"), py_str("drop a feed"), py_str("drop a milk feed"), py_str("a cup"), py_str("open cup"), py_str("sippy"), py_str("solids"), py_str("dummy"), py_str("teat"), py_str("sterilise"), py_str("sterilising"), py_str("puree"), py_str("baby rice"), py_str("first foods"), py_str("finger food"), py_str("finger foods"), py_str("introduce water"), py_str("introduce a cup"), py_str("introducing water"), py_str("introduce solids"), py_str("introduce allergens"), py_str("first tastes"), py_str("the first taste"), py_str("mash"), py_str("mashed"), py_str("mashing"), py_str("mashed up"), py_str("squash it"), py_str("fork mash")]) }
    s__BABY_ACT(__data31());
    s__INTRODUCES_A_FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:introduc(?:e|ing|ed)|first\\s+(?:taste|try|time\\s+giving)|start(?:ing)?\\s+giving)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_MY_DIET(py_tuple(vec![py_str("diet"), py_str("into my"), py_str("to my diet"), py_str("for me"), py_str("myself"), py_str("my own")]));
    fn __data32() -> Py { py_tuple(vec![py_str("jars or pouches"), py_str("jars and pouches"), py_str("jars as good"), py_str("pouches as good"), py_str("jars or homemade"), py_str("pouches or homemade"), py_str("own purees"), py_str("purees or"), py_str("puree or jars"), py_str("buy jars"), py_str("shop bought"), py_str("shop-bought"), py_str("ready made pots"), py_str("ready-made pots"), py_str("rusk"), py_str("rusks"), py_str("how much milk should i give"), py_str("jar"), py_str("jars"), py_str("pouch"), py_str("pouches"), py_str("puree"), py_str("purees"), py_str("pureed")]) }
    s__A_FEEDING_PHRASE(__data32());
    s__GIVING_WATER(py_tuple(vec![py_str("give water"), py_str("giving water"), py_str("give them water"), py_str("give him water"), py_str("give her water"), py_str("start giving water")]));
    s__GIVING_WATER_ASKS(call_builtin("frozenset", &[mcall(&py_str("when can should how howmuch age what"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__GIVING_WATER_OUT(call_builtin("frozenset", &[mcall(&py_str("plant plants cactus cacti garden gardens flower flowers tree trees\n    grass lawn shrub shrubs seedling seedlings houseplant houseplants dog dogs puppy puppies cat\n    cats kitten kittens pet pets vet hamster rabbit bird birds fish pond"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__WHILE_SHE_FEEDS(stdlib_call("re", "compile", &[py_str("\\bwhile (?:breastfeeding|breast feeding|nursing|lactating|chestfeeding)\\b|\\bwhen (?:breastfeeding|breast feeding|nursing|lactating)\\b|\\bbreastfeeding (?:can|should|could|may|is|and)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HER_OWN_EATING(stdlib_call("re", "compile", &[py_str("\\b(?:can|should|could|may|do) i (?:eat|drink|have|avoid|skip|cut out)\\b|\\bis it (?:ok|okay|safe|fine) to (?:eat|drink|have)\\b|\\banything i (?:cant|can't|shouldnt|shouldn't) (?:eat|drink|have)\\b|\\bwhat can i (?:eat|drink|have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ALLERGEN_ROW(py_str("Give the common allergens early and often rather than late: from six months, one at a time, on a day they are well and in the morning - peanut, egg, milk, wheat, fish, shellfish, sesame and soya. Once a food is in, keep giving it a couple of times a week, because stopping is what raises the risk. With severe eczema or a reaction already, ask a GP before starting."));
    fn __data33() -> Py { py_tuple(vec![py_str("allergen"), py_str("allergens"), py_str("allergy"), py_str("allergies"), py_str("peanut"), py_str("peanuts"), py_str("peanut butter"), py_str("egg"), py_str("eggs"), py_str("milk"), py_str("cows milk"), py_str("wheat"), py_str("fish"), py_str("shellfish"), py_str("sesame"), py_str("soya"), py_str("soy"), py_str("nuts"), py_str("nut butter"), py_str("dairy"), py_str("cheese"), py_str("yoghurt"), py_str("yogurt"), py_str("strawberry"), py_str("strawberries"), py_str("kiwi"), py_str("tomato"), py_str("food"), py_str("foods"), py_str("finger food"), py_str("finger foods"), py_str("solids"), py_str("weaning"), py_str("wean"), py_str("puree"), py_str("purees"), py_str("reaction"), py_str("rash"), py_str("hives"), py_str("eczema"), py_str("reflux"), py_str("formula"), py_str("meal"), py_str("meals"), py_str("breakfast"), py_str("lunch"), py_str("dinner"), py_str("spoon"), py_str("lumps")]) }
    s__ALLERGEN_FOOD(call_builtin("frozenset", &[__data33()]).unwrap_or_else(|e| exc::raise(e)));
    s__INTRODUCING_PEOPLE(stdlib_call("re", "compile", &[py_str("\\bintroduc(?:e|ing|ed)\\b.*?\\bto (?:a|an|the|my|our|her|his|their|each)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data34() -> Py { py_tuple(vec![py_str("mum"), py_str("mums"), py_str("mom"), py_str("moms"), py_str("mother"), py_str("mam"), py_str("dad"), py_str("dads"), py_str("father"), py_str("stepdad"), py_str("stepmum"), py_str("stepmom"), py_str("gran"), py_str("grans"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("nana"), py_str("nan"), py_str("pop"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("brother"), py_str("brothers"), py_str("sister"), py_str("sisters"), py_str("friend"), py_str("friends"), py_str("neighbour"), py_str("neighbor"), py_str("colleague"), py_str("boss"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("cousin"), py_str("housemate"), py_str("flatmate"), py_str("roommate"), py_str("carer"), py_str("client"), py_str("customer"), py_str("patient"), py_str("man"), py_str("woman"), py_str("lady"), py_str("gentleman"), py_str("fella"), py_str("guy"), py_str("bloke"), py_str("she"), py_str("he"), py_str("her"), py_str("him")]) }
    s__GROWN_UP(__data34());
    fn __data35() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("pup"), py_str("pups"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("rabbit"), py_str("rabbits"), py_str("hamster"), py_str("hamsters"), py_str("guinea"), py_str("pig"), py_str("pigs"), py_str("parrot"), py_str("bird"), py_str("birds"), py_str("horse"), py_str("horses"), py_str("pet"), py_str("pets"), py_str("vet")]) }
    s__ANIMAL(__data35());
    s__SEASON_OF_A_SHOW(stdlib_call("re", "compile", &[py_str("\\b(?:season\\s*(?:\\d+|one|two|three|four|five|six)|this season|next season|last season|new season|the season|final season|first season|episodes?|series|box set|binge|football|cricket|rugby|premier league|the league|league|fixtures?|match|matches|team|teams|gym|the show|on tv|netflix)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data36() -> Py { py_tuple(vec![py_str("should i wake"), py_str("shall i wake"), py_str("do i need to wake"), py_str("do i have to wake"), py_str("can i wake"), py_str("could i wake"), py_str("how do i wake"), py_str("how often should i wake"), py_str("how long should i let"), py_str("wake my baby"), py_str("wake my newborn"), py_str("wake the baby"), py_str("wake a baby"), py_str("wake up for a feed"), py_str("wake a sleepy baby"), py_str("wake a sleepy newborn"), py_str("wake a sleepy"), py_str("sleepy for a feed"), py_str("too sleepy to feed")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("feed"), py_str("feeds"), py_str("feeding"), py_str("night"), py_str("nights"), py_str("sleep"), py_str("sleeping"), py_str("slept"), py_str("through the night"), py_str("should i"), py_str("do i need to"), py_str("how often"), py_str("how long")]) }
    s__WAKE_TO_FEED_KEY(py_tuple(vec![__data36(), op_add(&g_BABY(), &__data37())]));
    s__WAKE_TO_FEED_WORDS(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WAKE_TO_FEED_KEY()) { let mut part: Py = __item.clone(); for __item in py_iter_or_raise(&part.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data39() -> Py { py_tuple(vec![py_str("what we eat"), py_str("what you eat"), py_str("our food"), py_str("our dinner"), py_str("our meals"), py_str("family food"), py_str("family meal"), py_str("family meals"), py_str("the same as us"), py_str("same as us"), py_str("adult food"), py_str("grown up food"), py_str("eat what we eat"), py_str("eat what i eat"), py_str("eat the same")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("spoon"), py_str("spoons"), py_str("loaded spoon"), py_str("cutlery"), py_str("fork"), py_str("forks"), py_str("her own spoon"), py_str("his own spoon"), py_str("feed herself"), py_str("feed himself"), py_str("feed themselves"), py_str("feeding herself"), py_str("feeding himself")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("give"), py_str("gives"), py_str("giving"), py_str("hold"), py_str("holds"), py_str("use"), py_str("using"), py_str("when"), py_str("start"), py_str("started"), py_str("first"), py_str("should"), py_str("eat"), py_str("eating")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("wont drink from a cup"), py_str("won't drink from a cup"), py_str("wont take a cup"), py_str("won't take a cup"), py_str("refuses a cup"), py_str("refuses the cup"), py_str("refuses a beaker"), py_str("will not drink from a cup"), py_str("wont drink from a beaker"), py_str("wont use a cup"), py_str("hates the cup"), py_str("spits out the cup"), py_str("not drinking from a cup"), py_str("refuses to drink"), py_str("wont drink"), py_str("won't drink"), py_str("refuses milk"), py_str("wont drink milk"), py_str("will not drink")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("cup"), py_str("beaker"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("water"), py_str("milk"), py_str("wont"), py_str("won't"), py_str("refuses"), py_str("should"), py_str("do"), py_str("i")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("bottle"), py_str("bottles"), py_str("teat"), py_str("teats"), py_str("formula"), py_str("breast"), py_str("breastfeeding"), py_str("food"), py_str("foods"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("solids"), py_str("meals")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("stop formula"), py_str("stop having formula"), py_str("stop giving formula"), py_str("stop the formula"), py_str("come off formula"), py_str("off formula"), py_str("switch to cows milk"), py_str("move to cows milk"), py_str("change to cows milk"), py_str("when do i stop formula"), py_str("follow on milk"), py_str("follow-on milk"), py_str("stage 2 milk"), py_str("hungrier baby milk"), py_str("growing up milk")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("formula"), py_str("formula milk"), py_str("baby formula"), py_str("infant formula"), py_str("first infant milk"), py_str("infant milk"), py_str("hungry baby milk"), py_str("hungry baby"), py_str("follow on milk"), py_str("follow-on milk"), py_str("stage 2 milk"), py_str("stage 3 milk"), py_str("growing up milk"), py_str("brand of formula"), py_str("formula brand"), py_str("formula brands")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("which"), py_str("what"), py_str("best"), py_str("better"), py_str("switch"), py_str("switching"), py_str("switched"), py_str("change"), py_str("changing"), py_str("changed"), py_str("try"), py_str("trying"), py_str("brand"), py_str("brands"), py_str("use"), py_str("using"), py_str("buy"), py_str("buying"), py_str("choose"), py_str("choosing"), py_str("recommend"), py_str("worth"), py_str("need"), py_str("mix"), py_str("mixing"), py_str("suit"), py_str("suits"), py_str("baby"), py_str("babies"), py_str("baby's"), py_str("babys"), py_str("newborn"), py_str("infant"), py_str("bottle"), py_str("bottles"), py_str("feed"), py_str("feeds"), py_str("feeding"), py_str("milk"), py_str("colic"), py_str("colicky"), py_str("reflux")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("how much"), py_str("how many"), py_str("how often"), py_str("ml"), py_str("oz"), py_str("ounces"), py_str("scoop"), py_str("scoops"), py_str("make up"), py_str("making up"), py_str("made up"), py_str("prepare"), py_str("preparing"), py_str("sterilise"), py_str("sterilising"), py_str("steriliser"), py_str("kettle"), py_str("70c"), py_str("70 c"), py_str("cool it"), py_str("left out"), py_str("left over"), py_str("two hours"), py_str("reheat"), py_str("warm up"), py_str("bottle warmer"), py_str("how long"), py_str("how do i make"), py_str("cows milk"), py_str("whole milk"), py_str("temperature"), py_str("how hot"), py_str("hot water"), py_str("boiling"), py_str("boil the kettle"), py_str("warm it"), py_str("the formula for"), py_str("formula for the"), py_str("area"), py_str("circle"), py_str("triangle"), py_str("equation"), py_str("quadratic"), py_str("maths"), py_str("math"), py_str("algebra"), py_str("percentage"), py_str("spreadsheet"), py_str("sheet"), py_str("excel"), py_str("cell"), py_str("cells"), py_str("calculate"), py_str("gcse"), py_str("homework"), py_str("column"), py_str("columns"), py_str("row"), py_str("rows"), py_str("numbers"), py_str("number"), py_str("sum"), py_str("autosum"), py_str("breast"), py_str("breastfeeding"), py_str("breastfeed"), py_str("breastfed"), py_str("breast milk"), py_str("breastmilk"), py_str("expressed"), py_str("expressing")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("which"), py_str("what"), py_str("buy"), py_str("buying"), py_str("brand"), py_str("best"), py_str("better"), py_str("switch"), py_str("change"), py_str("use"), py_str("try"), py_str("mix"), py_str("mixing"), py_str("worth"), py_str("recommend"), py_str("need"), py_str("needs"), py_str("does"), py_str("do"), py_str("is"), py_str("are"), py_str("should"), py_str("suit"), py_str("suits"), py_str("right"), py_str("good")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("introduce water"), py_str("introduce a cup"), py_str("introduce the cup"), py_str("introduce a beaker"), py_str("introduce a sippy cup"), py_str("introduce an open cup"), py_str("first cup"), py_str("first water"), py_str("start water"), py_str("start a cup"), py_str("offer water"), py_str("offer a cup"), py_str("offer a beaker")]) }
    fn __data51() -> Py { py_tuple(vec![py_str("have"), py_str("has"), py_str("eat"), py_str("give"), py_str("cook"), py_str("cooking"), py_str("add"), py_str("put"), py_str("use"), py_str("ok"), py_str("safe"), py_str("can"), py_str("should"), py_str("in"), py_str("on"), py_str("for")]) }
    fn __data52() -> Py { py_tuple(vec![py_str("frying"), py_str("deep fry"), py_str("fryer"), py_str("chip pan"), py_str("oven"), py_str("car"), py_str("engine"), py_str("bike"), py_str("chain"), py_str("central heating"), py_str("radiator"), py_str("boiler"), py_str("hair"), py_str("skin"), py_str("essential oil"), py_str("drain"), py_str("drains"), py_str("sink"), py_str("plughole"), py_str("toilet"), py_str("loo"), py_str("pipes"), py_str("grease"), py_str("freeze"), py_str("freezer"), py_str("frozen"), py_str("thaw"), py_str("shelf life"), py_str("use by"), py_str("go off"), py_str("gone off"), py_str("peanut butter"), py_str("peanut"), py_str("peanuts"), py_str("nut butter"), py_str("almond butter"), py_str("nuts")]) }
    fn __data53() -> Py { py_tuple(vec![py_str("grow"), py_str("growing"), py_str("plant"), py_str("sow"), py_str("garden"), py_str("windowsill"), py_str("recipe"), py_str("bake"), py_str("spice rack"), py_str("where to buy"), py_str("grind"), py_str("in the eye"), py_str("in her eye"), py_str("in his eye"), py_str("in their eye"), py_str("in my eye"), py_str("eye is"), py_str("eyes are"), py_str("eye hurts"), py_str("eyes hurt"), py_str("eyeball"), py_str("screaming"), py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("nursing"), py_str("lactating"), py_str("while im feeding"), py_str("while breastfeeding")]) }
    fn __data54() -> Py { py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("peanut"), py_str("peanuts"), py_str("peanut butter"), py_str("milk"), py_str("dairy"), py_str("wheat"), py_str("soya"), py_str("soy"), py_str("sesame"), py_str("fish"), py_str("nut"), py_str("nuts"), py_str("allergen"), py_str("allergens"), py_str("allergy"), py_str("shellfish"), py_str("give"), py_str("start"), py_str("try"), py_str("introduce")]) }
    fn __data55() -> Py { py_tuple(vec![py_str("my eczema"), py_str("i have eczema"), py_str("my skin"), py_str("itchy"), py_str("flare"), py_str("cream"), py_str("emollient"), py_str("steroid"), py_str("bad"), py_str("worse"), py_str("infected"), py_str("weeping"), py_str("oozing")]) }
    fn __data56() -> Py { py_tuple(vec![py_str("rice milk"), py_str("rice drink"), py_str("oat milk"), py_str("oat drink"), py_str("almond milk"), py_str("soya milk"), py_str("soy milk"), py_str("soya drink"), py_str("coconut milk"), py_str("coconut drink"), py_str("plant milk"), py_str("plant based milk"), py_str("hemp milk"), py_str("cashew milk"), py_str("pea milk"), py_str("milk alternative")]) }
    fn __data57() -> Py { py_tuple(vec![py_str("have"), py_str("has"), py_str("give"), py_str("giving"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("ok"), py_str("safe"), py_str("use"), py_str("cook"), py_str("cooking"), py_str("instead"), py_str("can"), py_str("should")]) }
    fn __data58() -> Py { py_tuple(vec![py_str("have"), py_str("has"), py_str("give"), py_str("giving"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("ok"), py_str("safe"), py_str("use"), py_str("cook"), py_str("cooking"), py_str("instead"), py_str("can"), py_str("should")]) }
    fn __data59() -> Py { py_tuple(vec![py_str("what can my baby drink"), py_str("what can a baby drink"), py_str("what can babies drink"), py_str("what should my baby drink"), py_str("what can my baby have to drink"), py_str("how much should my baby drink"), py_str("how much should a baby drink"), py_str("how much can my baby drink"), py_str("how much should babies drink"), py_str("how much can a baby drink"), py_str("straw"), py_str("straws"), py_str("sippy cup"), py_str("open cup"), py_str("drink"), py_str("drinks"), py_str("drinking")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("how much"), py_str("iron"), py_str("vitamin"), py_str("vitamins"), py_str("vitamin d"), py_str("calcium"), py_str("protein"), py_str("calories"), py_str("fibre"), py_str("fiber"), py_str("sleep"), py_str("sleeping"), py_str("i put on"), py_str("i have put on"), py_str("ive put on"), py_str("i've put on"), py_str("i gained"), py_str("lose weight"), py_str("losing weight"), py_str("lose the baby weight"), py_str("weight loss"), py_str("baby weight"), py_str("my own weight"), py_str("back to my old weight"), py_str("slimming")]) }
    fn __data61() -> Py { py_tuple(vec![py_str("ready"), py_str("readiness"), py_str("start solids"), py_str("starting solids"), py_str("start on solids"), py_str("starting on solids"), py_str("first tastes"), py_str("the signs"), py_str("signs of"), py_str("any signs"), py_str("any sign"), py_str("a sign"), py_str("show signs"), py_str("shows signs"), py_str("showing signs"), py_str("signs that"), py_str("signs should"), py_str("signs to"), py_str("introduce solids"), py_str("introducing solids"), py_str("introduce food"), py_str("introducing food"), py_str("give solids"), py_str("when to wean"), py_str("solids"), py_str("solid food"), py_str("solid foods"), py_str("too young"), py_str("too early"), py_str("old enough"), py_str("young for solids"), py_str("young for food"), py_str("early for solids"), py_str("start giving"), py_str("start food"), py_str("starting food"), py_str("start on food"), py_str("give food"), py_str("giving food"), py_str("ready for food"), py_str("food ready"), py_str("give her food"), py_str("give him food"), py_str("first food")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("breakfast"), py_str("lunch"), py_str("dinner"), py_str("supper"), py_str("snack"), py_str("snacks"), py_str("bottle"), py_str("bottles"), py_str("teat"), py_str("teats"), py_str("cup"), py_str("cups"), py_str("sippy"), py_str("night feed"), py_str("night feeds"), py_str("night feeding"), py_str("off the bottle"), py_str("sterilise"), py_str("sterilising"), py_str("formula")]) }
    fn __data63() -> Py { py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("milk"), py_str("cow's milk"), py_str("cows milk"), py_str("peanut"), py_str("peanuts"), py_str("peanut butter"), py_str("nut butter"), py_str("wheat"), py_str("fish"), py_str("sesame"), py_str("soya"), py_str("nuts"), py_str("dairy"), py_str("cheese"), py_str("yoghurt"), py_str("yogurt"), py_str("strawberry"), py_str("kiwi"), py_str("food"), py_str("foods"), py_str("finger food"), py_str("solids"), py_str("allergen"), py_str("allergens"), py_str("puree"), py_str("lumps")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("ok"), py_str("fine"), py_str("good"), py_str("safe"), py_str("healthy"), py_str("buy"), py_str("worth"), py_str("reheat"), py_str("warm"), py_str("use"), py_str("best"), py_str("better"), py_str("any good"), py_str("have"), py_str("give"), py_str("can")]) }
    fn __data65() -> Py { py_tuple(vec![py_str("only wants milk"), py_str("only wants Breast milk"), py_str("only drinks milk"), py_str("no interest in food"), py_str("not interested in food"), py_str("won't eat"), py_str("wont eat"), py_str("will not eat"), py_str("refuses food"), py_str("refusing food"), py_str("won't take food"), py_str("wont take food"), py_str("spits it out"), py_str("pushed the spoon away"), py_str("pushes the spoon away"), py_str("pushed the spoon"), py_str("spoon away"), py_str("pushed it away"), py_str("pushes it away"), py_str("pushes the food away"), py_str("pushed the food away"), py_str("spat it out"), py_str("spat the food out"), py_str("spat out"), py_str("turns her head away"), py_str("turns his head away"), py_str("turns away from the spoon"), py_str("turns away from food"), py_str("closes her mouth"), py_str("closes his mouth"), py_str("shakes her head"), py_str("shakes his head"), py_str("wont take the spoon"), py_str("won't take the spoon"), py_str("wont open her mouth"), py_str("won't open his mouth"), py_str("spits the food out"), py_str("spitting the food out"), py_str("spits food out"), py_str("spitting food out"), py_str("keeps spitting the food out"), py_str("keeps spitting food out"), py_str("spitting it out"), py_str("keeps spitting it out"), py_str("spits it all out"), py_str("spitting it all out"), py_str("spits his food out"), py_str("spits her food out"), py_str("spat the food out")]) }
    fn __data66() -> Py { py_tuple(vec![py_str("refuses lumpy food"), py_str("refusing lumpy food"), py_str("won't eat lumpy food"), py_str("lumpy food"), py_str("lumpy foods"), py_str("lumpy"), py_str("refuses lumps"), py_str("hates lumps"), py_str("lumps"), py_str("lumpier"), py_str("texture"), py_str("textures"), py_str("chewy"), py_str("bits of food"), py_str("bits of it"), py_str("the bits"), py_str("spits the bits"), py_str("chunks")]) }
    fn __data67() -> Py { py_tuple(vec![py_str("refuse"), py_str("refuses"), py_str("refusing"), py_str("refused"), py_str("hates"), py_str("hate"), py_str("won't"), py_str("wont"), py_str("will not"), py_str("spits"), py_str("gags"), py_str("struggles"), py_str("too early"), py_str("have"), py_str("has"), py_str("try"), py_str("start"), py_str("move on"), py_str("when"), py_str("can")]) }
    fn __data68() -> Py { py_tuple(vec![py_str("nails"), py_str("nail"), py_str("fingernails"), py_str("fingernail"), py_str("nail biting"), py_str("cuticles"), py_str("choking"), py_str("choke"), py_str("first aid"), py_str("fever"), py_str("temperature"), py_str("rash"), py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("not feeding"), py_str("isnt feeding"), py_str("breathing"), py_str("hospital"), py_str("999"), py_str("111")]) }
    fn __data69() -> Py { py_tuple(vec![py_str("lump in his mouth"), py_str("lump in her mouth"), py_str("lump in their mouth"), py_str("lump in the mouth"), py_str("lump in his gums"), py_str("lump on his gums"), py_str("lump on her gums"), py_str("lump on the gum"), py_str("lump on his gum"), py_str("lump on her gum"), py_str("lump in his cheek"), py_str("lump on his cheek"), py_str("lump on her cheek"), py_str("lump on the cheek"), py_str("lump inside his cheek"), py_str("lump under his tongue"), py_str("lump on his tongue"), py_str("lump on his lip"), py_str("lump on her lip"), py_str("lump on the roof of his mouth"), py_str("bump in his mouth"), py_str("bump on his gum"), py_str("bump on his cheek"), py_str("white lump in his mouth"), py_str("lump inside his mouth")]) }
    fn __data70() -> Py { py_tuple(vec![py_str("baby"), py_str("babies"), py_str("newborn"), py_str("month old"), py_str("months old"), py_str("year old"), py_str("toddler"), py_str("child"), py_str("son"), py_str("daughter"), py_str("kid"), py_str("what"), py_str("why"), py_str("should"), py_str("worried"), py_str("is it"), py_str("normal"), py_str("check"), py_str("how"), py_str("have"), py_str("has"), py_str("got"), py_str("noticed"), py_str("found"), py_str("there is")]) }
    fn __data71() -> Py { py_tuple(vec![py_str("tooth"), py_str("teeth"), py_str("teething"), py_str("toothache"), py_str("dentist appointment"), py_str("brace"), py_str("braces"), py_str("ulcer"), py_str("ulcers"), py_str("cold sore"), py_str("blister"), py_str("blisters"), py_str("hand foot and mouth"), py_str("chickenpox"), py_str("measles"), py_str("mumps"), py_str("tonsil"), py_str("tonsils"), py_str("tongue tie")]) }
    fn __data72() -> Py { py_tuple(vec![py_str("breathing"), py_str("wheeze"), py_str("floppy"), py_str("limp"), py_str("unresponsive"), py_str("999"), py_str("ambulance"), py_str("epipen"), py_str("epi pen"), py_str("tongue"), py_str("hay fever"), py_str("hayfever"), py_str("vaccine"), py_str("vaccination"), py_str("jab"), py_str("jabs"), py_str("mmr"), py_str("immunisation"), py_str("nappy rash"), py_str("teething"), py_str("covid")]) }
    fn __data73() -> Py { py_tuple(vec![py_str("drop a milk feed"), py_str("drop a feed"), py_str("drop feeds"), py_str("dropping feeds"), py_str("stop breastfeeding"), py_str("stop breast feeding"), py_str("stop nursing"), py_str("stop feeding"), py_str("wean off the breast"), py_str("give up breastfeeding"), py_str("finish breastfeeding"), py_str("dropping milk feeds"), py_str("drop a bottle"), py_str("drop the bottle"), py_str("dropping bottles"), py_str("cut down feeds"), py_str("cut down bottles"), py_str("cut down on bottles"), py_str("fewer feeds"), py_str("fewer bottles"), py_str("give up a feed"), py_str("stop a feed"), py_str("stopping feeds"), py_str("reduce feeds"), py_str("reduce milk"), py_str("less interested in milk"), py_str("feeding less")]) }
    fn __data74() -> Py { py_tuple(vec![py_str("milk"), py_str("bottles"), py_str("bottle"), py_str("feeds"), py_str("feed"), py_str("bf"), py_str("breastfeeding"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers")]) }
    fn __data75() -> Py { py_tuple(vec![py_str("newborn"), py_str("newborns"), py_str("neonate"), py_str("weeks old"), py_str("week old"), py_str("1 month old"), py_str("2 month old"), py_str("3 month old"), py_str("one month old"), py_str("two month old"), py_str("three month old"), py_str("not feeding"), py_str("isnt feeding"), py_str("isn't feeding"), py_str("refusing feeds"), py_str("not taking feeds"), py_str("too sleepy")]) }
    fn __data76() -> Py { py_tuple(vec![py_str("what foods to avoid"), py_str("foods to avoid"), py_str("what food to avoid"), py_str("what not to give"), py_str("foods not to give"), py_str("what not to feed"), py_str("foods not to feed"), py_str("what should i not give"), py_str("what should i not feed"), py_str("what shouldnt i give"), py_str("what shouldn't i give"), py_str("what should i avoid"), py_str("foods should i avoid"), py_str("avoid giving"), py_str("foods to avoid giving"), py_str("what to avoid giving"), py_str("not feed my baby"), py_str("not feed a baby"), py_str("not give my baby"), py_str("should i not feed"), py_str("foods are unsafe"), py_str("unsafe foods"), py_str("what is unsafe to give"), py_str("what is unsafe to feed"), py_str("what cant my baby eat"), py_str("what can't my baby eat"), py_str("what can a baby not eat"), py_str("what cant a baby eat"), py_str("what can't a baby eat")]) }
    fn __data77() -> Py { py_tuple(vec![py_str("avoid"), py_str("unsafe"), py_str("not"), py_str("give"), py_str("giving"), py_str("feed"), py_str("feeding"), py_str("eat"), py_str("eating"), py_str("food"), py_str("foods"), py_str("should"), py_str("what"), py_str("can")]) }
    fn __data78() -> Py { py_tuple(vec![py_str("water"), py_str("juice"), py_str("squash"), py_str("milk"), py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("thirsty"), py_str("bottle"), py_str("bottles"), py_str("cup"), py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("nursing"), py_str("lactating"), py_str("chestfeeding"), py_str("while im feeding")]) }
    fn __data79() -> Py { py_tuple(vec![py_str("alarm"), py_str("alarm clock"), py_str("wake up early"), py_str("wake up tired"), py_str("wake up at night"), py_str("cant sleep"), py_str("can't sleep"), py_str("insomnia"), py_str("tired all the time"), py_str("wake up"), py_str("woken up by"), py_str("wake me up"), py_str("for the wake"), py_str("funeral")]) }
    fn __data38() -> Py { py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-by-age")); __args.push(py_tuple(vec![g__WHAT_TO_FEED(), g_AGE(), g_EATING()])); __args.push(py_func("_by_age", |args: &[Py]| _by_age(args))); __kws.extend(vec![("ask".to_string(), g_AGE()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_a_baby_or_a_pronoun", |args: &[Py]| _a_baby_or_a_pronoun(args))), ("avoid".to_string(), op_add(&g_STAND_DOWN(), &py_tuple(vec![py_str("drink"), py_str("drinks"), py_str("drinking"), py_str("drank"), py_str("thirsty"), py_str("fluids"), py_str("hydrate"), py_str("hydrated")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-family-food")); __args.push(py_tuple(vec![op_add(&g__BABY(), &py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers"), py_str("1 year old"), py_str("one year old")])), __data39()])); __args.push(py_str("From about a year they can have what the rest of the family eats, with two changes: take their portion out before the salt goes in, and cut it small - no whole grapes, whole nuts or popcorn before about four. Before one the food is practice, and milk is still the meal, so the family dinner is offered alongside it rather than instead of it.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-finger-foods")); __args.push(py_tuple(vec![py_tuple(vec![py_str("finger food"), py_str("finger foods"), py_str("fingerfoods"), py_str("finger foods for")]), py_tuple(vec![py_str("what"), py_str("which"), py_str("ideas"), py_str("list"), py_str("good"), py_str("best"), py_str("options"), py_str("suggest"), py_str("suggestions")])])); __args.push(py_str("Sticks and strips they can pick up and hold: cooked carrot, broccoli or green beans, toast fingers with a thin smear of peanut butter, ripe pear or banana, soft cheese, scrambled egg, well-cooked chicken or pasta shapes. Cut grapes and cherry tomatoes lengthways into quarters, cook hard vegetables until they squash between your fingers, and always have them sitting down while they eat.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &py_tuple(vec![py_str("when"), py_str("how old"), py_str("age"), py_str("choking"), py_str("gagging")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-a-spoon")); __args.push(py_tuple(vec![__data40(), op_add(&g_BABY(), &__data41())])); __args.push(py_str("From the very first meals, at six months: put a loaded spoon in their hand and let them get it to their mouth, however badly, while you load another. They will grip it in a fist at first and get more on the floor than in, and that is the learning, not a failure. Until they have the pincer grip, around eight or nine months, the spoon works best with thick things - porridge, mash, yoghurt - and finger food does the rest. Let them decide when the meal is over, and never shovel a spoon in while they are crying, because that is how a baby learns to dread the highchair. A children's fork with rounded prongs is fine too from around nine months, and it does the one job a spoon cannot - stabbing the piece they have decided they want.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &py_tuple(vec![py_str("cutlery set"), py_str("wedding"), py_str("christening"), py_str("gift"), py_str("present"), py_str("silver"), py_str("antique"), py_str("my dinner"), py_str("i eat"), py_str("restaurant")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-refusing-a-cup")); __args.push(py_tuple(vec![__data42(), op_add(&g_BABY(), &__data43())])); __args.push(py_str("Keep offering it and don't make it a fight: a little water in an open cup at every meal, your own cup in your hand at the same time, and no bottle as a fallback in between. Free-flow beakers and straw cups are worth trying, because a valve bottle teaches a different suck. Milk is still the main drink until a year, so a baby who takes only a few sips from a cup is not going short - the cup is practice, and most get there between nine months and a year.")); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_BABY(), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_MENU()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("wont drink"), py_str("won't drink"), py_str("refuses to drink"), py_str("refuses"), py_str("refusing"), py_str("refused"), py_str("refuse")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))), &__data44()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-stopping-formula")); __args.push(py_tuple(vec![__data45(), op_add(&g_BABY(), &py_tuple(vec![py_str("formula"), py_str("milk"), py_str("when"), py_str("stop"), py_str("stopped"), py_str("switch"), py_str("change"), py_str("give"), py_str("giving"), py_str("should"), py_str("do i")]))])); __args.push(py_str("First infant milk until the first birthday, then whole cows milk in a cup. There is no need to move to a follow-on or hungry-baby milk: they are the same kind of thing with a different label, and the labels are marketing rather than a stage. Do not switch to cows milk as the main drink before twelve months, because it has too little iron. If you are stopping breastfeeding rather than formula, drop one feed at a time, about a week apart, and leave the bedtime one until last.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&g_NOT_A_BABY(), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_MENU()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("formula")))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-which-formula")); __args.push(py_tuple(vec![__data46(), __data47()])); __args.push(py_str("Every first infant milk sold has to meet the same standard, so the brand matters much less than the price and the tin you can get hold of: pick one first infant milk and stay with it. Hungry-baby and follow-on milks are the same kind of thing under a different label, and moving to them is marketing rather than a stage, so there is no need to change when a baby seems hungrier - offer a little more of the same milk first, because it is usually volume and not the powder. If you do want a different brand, change straight over rather than mixing two powders in one bottle, and give it a few days: a bit of wind, looser nappies or more posseting on a new milk usually settles, while blood in the nappy, forceful vomiting, a rash or eczema that will not settle is worth a word with the health visitor about a milk-free trial.")); __kws.extend(vec![("avoid".to_string(), __data48()), ("ask".to_string(), __data49()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-introducing-a-cup")); __args.push(py_tuple(vec![__data50(), py_tuple(vec![py_str("do i"), py_str("i"), py_str("we"), py_str("should"), py_str("when"), py_str("how"), py_str("baby"), py_str("cup"), py_str("beaker"), py_str("water"), py_str("drink"), py_str("sips")])])); __args.push(py_str("Water in an open cup, from six months and with meals: a few sips is the whole point, and an open cup or a free-flow beaker teaches the sip that a valve bottle never does. Hold the cup with them at first, or use a small one they can get both hands round, and expect it down the front for a week or two. Milk stays the main drink until a year, so water is offered rather than pushed, and juice or squash is not needed at all.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &py_tuple(vec![py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("my son"), py_str("my daughter"), py_str("new man"), py_str("new woman"), py_str("family"), py_str("child to"), py_str("to my child"), py_str("to my kids")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-butter-and-oil")); __args.push(py_tuple(vec![py_tuple(vec![py_str("butter"), py_str("oil"), py_str("spread"), py_str("margarine"), py_str("ghee"), py_str("lard"), py_str("fats"), py_str("fat")]), op_add(&g_BABY(), &__data51())])); __args.push(py_str("Yes, in the amounts you would use yourself: a little unsalted butter or a teaspoon of rapeseed or olive oil in their mash, on toast, or to cook their vegetables in. Fat is not the enemy at this age - it carries the vitamins and it is most of what makes a small stomach feel fed - so do not go for low-fat spreads or skimmed anything before two. Salt is the thing to watch, and butter is salty, so keep it to a scrape.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), op_add(&g_BABY(), &py_tuple(vec![py_str("puree"), py_str("purees"), py_str("mash"), py_str("mashed"), py_str("solids"), py_str("spoon"), py_str("bowl"), py_str("baby food"), py_str("food"), py_str("foods")]))), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &__data52()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-spices-and-herbs")); __args.push(py_tuple(vec![op_add(&g_SPICE_NAMES(), &py_tuple(vec![py_str("spice rack"), py_str("spicy"), py_str("seasoning"), py_str("pepper"), py_str("chilli"), py_str("chili"), py_str("garlic"), py_str("ginger"), py_str("curry"), py_str("curries"), py_str("mild curry")])), g_BABY()])); __args.push(py_str("Yes, and earlier is better than later: mild spices and herbs - cumin, cinnamon, turmeric, garlic, ginger, parsley, coriander - from six months, in the food you cook for the family. A baby who meets flavour early is usually the one who eats more widely at two. What to leave out is the heat and the salt: no chilli heat, no salt, no stock cubes. Black pepper in cooking is fine; a hot dish is not, because it is painful rather than dangerous.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &__data53())), ("guard".to_string(), py_func("_a_child_is_in_the_line", |args: &[Py]| _a_child_is_in_the_line(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-eczema-and-egg")); __args.push(py_tuple(vec![py_tuple(vec![py_str("eczema"), py_str("dry skin"), py_str("flaky skin"), py_str("hives"), py_str("my baby has eczema"), py_str("baby eczema"), py_str("eczema and egg"), py_str("egg and eczema")]), __data54()])); __args.push(py_str("Yes, and don't put it off because of the eczema: a baby with eczema is the one most likely to react, and early, regular tastes of egg and peanut lower the chance of a real allergy rather than raising it. Give a small amount in the morning, at home, on a day you are not going out, and keep it in their diet a couple of times a week once it is fine. If the eczema is bad or already infected, ask the GP or health visitor before you start, and if there is a reaction - hives, swelling, vomiting, wheeze - stop and call 999 if the breathing changes.")); __kws.extend(vec![("bias".to_string(), Py::Int(8)), ("ask".to_string(), py_tuple(vec![py_str("give"), py_str("start"), py_str("try"), py_str("introduce"), py_str("have"), py_str("eat"), py_str("can i")])), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_BABY(), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_MENU()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("eczema")))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))), &__data55()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-which-milk")); __args.push(py_tuple(vec![py_tuple(vec![py_str("what milk"), py_str("which milk"), py_str("what kind of milk"), py_str("what type of milk"), py_str("best milk")]), op_add(&op_add(&g_BABY(), &g_AGE()), &py_tuple(vec![py_str("give"), py_str("giving"), py_str("should"), py_str("for"), py_str("to"), py_str("use"), py_str("buy")]))])); __args.push(py_func("_milk_volume", |args: &[Py]| _milk_volume(args))); __kws.extend(vec![("ask".to_string(), op_add(&op_add(&g_BABY(), &g_AGE()), &py_tuple(vec![py_str("give"), py_str("giving"), py_str("should"), py_str("use")]))), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g_WEANING_ELSEWHERE()), &g__NOT_A_BABY_MILK()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-plant-milks")); __args.push(py_tuple(vec![__data56(), op_add(&op_add(&g_BABY(), &g_AGE()), &__data57())])); __args.push(py_func("_plant_milk", |args: &[Py]| _plant_milk(args))); __kws.extend(vec![("ask".to_string(), __data58()), ("bias".to_string(), Py::Int(4)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g_WEANING_ELSEWHERE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-how-much-milk")); __args.push(py_tuple(vec![g__MILK_WORDS(), g__MILK_HOW_MUCH(), g__A_DRINK()])); __args.push(py_func("_milk_volume", |args: &[Py]| _milk_volume(args))); __kws.extend(vec![("ask".to_string(), op_add(&op_add(&g_BABY(), &g_AGE()), &g__THE_CHILD())), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g_WEANING_ELSEWHERE()), &g__NOT_A_BABY_MILK()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-what-a-baby-drinks")); __args.push(py_tuple(vec![g_BABY(), __data59()])); __args.push(py_func("_what_a_baby_drinks", |args: &[Py]| _what_a_baby_drinks(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), op_add(&g_BABY(), &g_AGE())), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g__NOT_A_BABY_MILK()), &g__A_COUNT()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-getting-enough")); __args.push(py_tuple(vec![g_BABY(), py_tuple(vec![py_str("enough"), py_str("getting enough"), py_str("fed"), py_str("feeding well"), py_str("putting on weight")])])); __args.push(py_str("The signs are the answer, not an amount: six or more wet nappies a day, dirty nappies that turn from black to yellow in the first week, steady weight along their own line, and a baby who settles after a feed and is alert when awake. A baby who is drowsy, has fewer wet nappies or is not back to birth weight by two weeks needs to be seen.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &__data60()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-ready-for-solids")); __args.push(py_tuple(vec![op_add(&g_BABY(), &g__A_CHILD_PRONOUN()), __data61()])); __args.push(py_str("Three signs, all at once, and around six months: sitting up with little or no support, steady head control, and swallowing food rather than pushing it back out with the tongue. Reaching for your plate or waking at night is not a sign on its own. Before four months is too early, and by six months it is time - waiting longer makes the iron and the chewing harder.")); __kws.extend(vec![("guard".to_string(), py_func("_a_baby_or_a_pronoun", |args: &[Py]| _a_baby_or_a_pronoun(args))), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g_STAND_DOWN(), &g_BOOK_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-what-to-feed")); __args.push(py_tuple(vec![g__WHAT_TO_FEED(), op_add(&g_BABY(), &g__A_CHILD_PRONOUN())])); __args.push(g__FIRST_FOODS()); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let u: Py = __kwp(args, 0, "u").unwrap_or(Py::None); { let __b = _a_baby_or_a_pronoun(&[u.clone()]); if !py_truthy(&__b) { __b } else { _of_weaning_age(&[u.clone()]) } } })), ("ask".to_string(), op_add(&g_EATING(), &py_tuple(vec![py_str("give"), py_str("gives"), py_str("giving"), py_str("first"), py_str("start"), py_str("starting")]))), ("avoid".to_string(), op_add(&g_STAND_DOWN(), &__data62()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-baby-led-weaning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("baby led weaning"), py_str("baby-led weaning"), py_str("blw"), py_str("babyled weaning")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("is"), py_str("explain"), py_str("mean"), py_str("means"), py_str("safe"), py_str("better"), py_str("start"), py_str("started"), py_str("starting")])])); __args.push(py_str("Baby-led weaning means skipping spoon-fed purees and letting them pick up soft finger food themselves from about six months. It is safe at that age and no more of a choking risk than spoon feeding, and you do not have to choose: most people end up doing a mix of both.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-puree")); __args.push(py_tuple(vec![g_BABY(), py_tuple(vec![py_str("puree"), py_str("purees"), py_str("pureed"), py_str("blend"), py_str("blended"), py_str("lumps"), py_str("lumpy"), py_str("smooth")])])); __args.push(py_str("You do not have to: soft finger food is safe from six months and teaches chewing sooner. If you do puree, keep it thick rather than watery and move on to lumps and finger food by about eight or nine months - a baby still on smooth food at one is the one who struggles with texture later.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_STAND_DOWN(), &g_A_BODY_LUMP()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-allergens")); __args.push(py_tuple(vec![g_BABY(), py_tuple(vec![py_str("allergen"), py_str("allergens"), py_str("allergy"), py_str("allergies"), py_str("sensitive"), py_str("sensitivities"), py_str("introduce"), py_str("introducing")])])); __args.push(py_func("_the_allergen_row", |args: &[Py]| _the_allergen_row(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), g_STAND_DOWN())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-first-taste")); __args.push(py_tuple(vec![py_tuple(vec![py_str("introduce"), py_str("introducing"), py_str("introduced"), py_str("first taste"), py_str("first tastes"), py_str("first time giving"), py_str("start giving"), py_str("start on")]), __data63()])); __args.push(py_str("One new food at a time, in the morning on a day they are well: a teaspoon is a first taste, then watch for twenty minutes, and a rash, swelling or sickness means stop and ring 111. Once a food is in, keep giving it a couple of times a week, because stopping is what raises the risk. Peanut and egg are the ones worth doing early rather than late, and with severe eczema or a reaction already, ask a GP before starting.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_STAND_DOWN(), &py_tuple(vec![py_str("bottle"), py_str("bottles"), py_str("bottle feed"), py_str("formula"), py_str("dummy")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-snacks")); __args.push(py_tuple(vec![g__SNACK(), g__SNACK_CHILD()])); __args.push(py_func("_snacks", |args: &[Py]| _snacks(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g__NOT_A_SNACK()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-shop-bought")); __args.push(py_tuple(vec![py_tuple(vec![py_str("jar"), py_str("jars"), py_str("pouch"), py_str("pouches"), py_str("shop bought"), py_str("shop-bought"), py_str("ready made"), py_str("ready-made"), py_str("baby food"), py_str("commercial"), py_str("supermarket")]), __data64()])); __args.push(py_str("Jars, pouches and ready-made pots are fine, and the rules on them are the same as on anything else: no salt, no sugar, no honey before one. Read the stage on the front against what they are managing now, decant it into a bowl rather than feeding from the pouch, and throw away what the spoon has been in. Warm it by standing the bowl in hot water, never in the pouch or the jar, and check it is not hot in the middle.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("keep"), py_str("last"), py_str("left out"), py_str("defrost"), py_str("batch")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-ice-cream")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ice cream"), py_str("icecream"), py_str("ice lolly"), py_str("ice lollies"), py_str("lolly"), py_str("lollies"), py_str("gelato")]), op_add(&g_BABY(), &py_tuple(vec![py_str("toddler"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("1 year old"), py_str("one year old")]))])); __args.push(py_str("A taste of plain ice cream from six months is fine, and it is the sugar rather than the cold that is the reason not to make it a habit: the shop stuff is mostly sugar, and the cones are a choking shape for a baby. Frozen yoghurt, a frozen banana or a lolly made from fruit puree gives the same thing with less sugar in it.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-only-milk")); __args.push(py_tuple(vec![__data65(), op_add(&op_add(&g_BABY(), &g__A_CHILD_PRONOUN()), &py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers")]))])); __args.push(py_str("That is normal for the first weeks: milk is still their food, and eating is practice. Keep offering one or two teaspoons once a day without coaxing, put them in the chair with you eating at the same time, and let them see the food rather than putting it in their mouth. Go to the health visitor if they are past nine months and still taking almost nothing, if they are dropping weight, or if they gag and cough on every texture.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let u: Py = __kwp(args, 0, "u").unwrap_or(Py::None); { let __b = _a_baby_is_named(&[u.clone()]); if py_truthy(&__b) { __b } else { _a_pronoun_is_in_the_line(&[u.clone()]) } } })), ("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &py_tuple(vec![py_str("ill"), py_str("unwell"), py_str("poorly"), py_str("temperature"), py_str("fever"), py_str("sick")]))) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__THE_REFUSAL_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-lumpy-food")); __args.push(py_tuple(vec![__data66(), op_add(&op_add(&g__BABY(), &g__THE_CHILD()), &__data67())])); __args.push(py_str("Do not blend it back: the window for lumps is about seven to nine months, and a baby who stays on smooth food gets crosser about texture later. Mash rather than puree, leave soft lumps you can squash with a fork, and give finger food next to the spoon so they are practising chewing whatever the spoon does. Gagging while they learn is normal and it is not choking - stay calm, sit them up, and let them bring it forward.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let u: Py = __kwp(args, 0, "u").unwrap_or(Py::None); { let __b = { let __b = _a_baby_is_named(&[u.clone()]); if py_truthy(&__b) { __b } else { _a_pronoun_is_in_the_line(&[u.clone()]) } }; if !py_truthy(&__b) { __b } else { pyrt::xrefs::xcall("knowledge.lookup", "not_the_heat_of_an_argument", &[u.clone()]) } } })), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_NOT_A_BABY(), &g_WEANING_ELSEWHERE()), &g__HOW_MUCH_MILK()), &__data68()), &g_A_BODY_LUMP()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-lump-in-mouth")); __args.push(py_tuple(vec![__data69(), __data70()])); __args.push(py_str("Most lumps in a baby's mouth are harmless: the little white pearls on the gums of a newborn, a mucocele from a bite on the lip, or a firm bony bump on the roof of the mouth. Leave it alone, do not squeeze it, and keep the mouth clean. Show it to the health visitor, a dentist or a GP within two weeks if it is still there after three weeks, or if it is hard, bleeding, growing, or comes with a fever or with a baby who will not feed - that is a check, not an emergency. A lump on the outside of the cheek is a skin lump and is usually harmless too; the ones that need a look are the same ones, plus anything that is hot and spreading.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_BABY(), &g_WEANING_ELSEWHERE()), &g__HOW_MUCH_MILK()), &__data71()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-food-reaction")); __args.push(py_tuple(vec![op_add(&g_BABY(), &py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers"), py_str("1 year old"), py_str("one year old")])), py_tuple(vec![py_str("reaction"), py_str("reacted"), py_str("reacts"), py_str("reacting"), py_str("allergic"), py_str("allergy"), py_str("hives"), py_str("swollen"), py_str("swelling"), py_str("rash round the mouth"), py_str("sick after eating")])])); __args.push(py_str("Stop giving that food and watch them for the next two hours. A rash, hives, swelling round the mouth, vomiting or a runny tummy is worth a same-day call to the GP or 111, and write down what they ate, how much and how long it took. Swelling of the lips or tongue, a hoarse cry, noisy breathing, or going pale and floppy is 999 - that is anaphylaxis and it needs an ambulance, not a wait. Do not try that food again until you have been told it is safe to.")); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("avoid".to_string(), op_add(&g_NOT_A_BABY(), &__data72()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-dropping-feeds")); __args.push(py_tuple(vec![__data73(), op_add(&g_BABY(), &__data74())])); __args.push(py_str("One feed at a time, about a week apart, and start with the one they are least bothered about - usually the mid-morning or mid-afternoon bottle rather than the first or the bedtime one. Offer a cup and a snack at the time you dropped, expect a few grumpy days, and keep the bedtime feed longest because it is doing the most work. Milk falling as food rises is the plan, not a problem: by a year three meals, two snacks and about 350 to 400ml of milk a day is enough.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g__HOW_MUCH_MILK()), &__data75()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-what-to-avoid")); __args.push(py_tuple(vec![__data76(), op_add(&op_add(&g_BABY(), &g_AGE()), &g__THE_CHILD()), __data77()])); __args.push(py_str("No salt and no sugar, and no honey at all before the first birthday. No whole nuts, whole grapes, popcorn or anything hard and round, because those are the shapes that block a small throat - cut grapes and cherry tomatoes into quarters lengthways. Skip low-fat and diet foods, unpasteurised cheese, raw or runny egg without the red lion stamp, and shark, swordfish and marlin. Cow's milk as a drink waits until twelve months, though in cooking it is fine from six; tea, coffee, cola and juice are not needed at any age.")); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("avoid".to_string(), op_add(&op_add(&op_add(&g_NOT_A_MENU(), &g_NOT_A_BABY()), &g_WEANING_ELSEWHERE()), &__data78())), ("ask".to_string(), py_tuple(vec![py_str("avoid"), py_str("not"), py_str("unsafe"), py_str("give"), py_str("giving"), py_str("feed"), py_str("feeding"), py_str("eat"), py_str("eating")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-how-much-water")); __args.push(py_tuple(vec![py_tuple(vec![py_str("how much water"), py_str("how much water should"), py_str("how much water can"), py_str("how much juice"), py_str("how much squash")]), op_add(&g_BABY(), &py_tuple(vec![py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("drink"), py_str("drinks"), py_str("day")]))])); __args.push(py_func("_how_much_water", |args: &[Py]| _how_much_water(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let u: Py = __kwp(args, 0, "u").unwrap_or(Py::None); { let __b = _a_baby_is_named(&[u.clone()]); if py_truthy(&__b) { __b } else { _a_pronoun_is_in_the_line(&[u.clone()]) } } })), ("avoid".to_string(), op_add(&op_add(&g_NOT_A_BABY(), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_MENU()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("how much water"), py_str("how much juice")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))), &py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("child"), py_str("children"), py_str("kids"), py_str("2 year"), py_str("3 year"), py_str("4 year"), py_str("5 year"), py_str("i drink"), py_str("should i drink")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("babyfood-wake-to-feed")); __args.push(g__WAKE_TO_FEED_KEY()); __args.push(py_str("No, not once they are back to their birth weight and gaining: a baby who sleeps is allowed to sleep, and night waking is normal for the first months. Wake them for a feed only if they are under two weeks old, were early or small, are jaundiced, are not yet back to their birth weight or are too sleepy to take a full feed - then offer every two to three hours, and make it worth their while: change the nappy first, undress them to the vest, and stroke a cool flannel over the hands and face.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_MENU()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__WAKE_TO_FEED_WORDS(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &__data79()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    s_ENTRIES(op_add(&__data38(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOODS()) { let mut row: Py = __item.clone(); __out.push(_verdict(&[geti(&row.clone(), &py_str("names")), geti(&row.clone(), &py_str("reply")), mcall(&row.clone(), "get", &[py_str("extra"), py_tuple(vec![])]), mcall(&row.clone(), "get", &[py_str("bias"), Py::Int(0)]), mcall(&row.clone(), "get", &[py_str("own_ask")]), mcall(&row.clone(), "get", &[py_str("pronoun"), Py::Bool(false)])])); } py_list(__out) })() }));
    s_ENTRIES({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ENTRIES()) { let mut row: Py = __item.clone(); __out.push(_only_for_a_baby(&[row.clone()])); } py_list(__out) })() });
}
pub fn _the_water_ask(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("\"when can i give water\": water, being given, and no drink of the person's own.\n\n    The table answers a food by name once a child is named, and this line names none - the water\n    is the whole question - so this row brings its own evidence rather than standing down. The\n    adult's own drink is somebody else's row: \"how much water should i drink a day\" names no\n    one to give it to, so it is left alone here.\n    ");
    text = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&text.clone(), &py_str(" water "))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "endswith", &[py_str(" water ")]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("i drink"), py_str("should i drink"), py_str(" a day "), py_str("litre"), py_str("liter"), py_str("pint"), py_str("eight glasses"), py_str("2 litres"), py_str("2 liters")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("give"), py_str("giving"), py_str("offer"), py_str("when can"), py_str("how much"), py_str("how often"), py_str("sips")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _text(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line as lower-case words, digits and all.");
    words = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("words")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[words.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { words.clone() } }) {
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    }
    return mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    Py::None
}
pub fn _age_in_the_chat(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut numbers: Py = Py::None;
    let mut said_now: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The age the conversation gave a turn or two ago, for a line that only says \"he\".\n\n    \"my little one is 7 months old\" then \"how much milk should he have now\" is one question in\n    two halves, and the second half names the baby with a pronoun and no age. The bands are only\n    useful if the age travels, so a pronoun line reads back through what the person typed before\n    it. Nothing else does: a line that names an age of its own is never second-guessed, and a\n    line with no pronoun is not read as one about the baby of three turns ago.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&op_bitand(&words.clone(), &call_builtin("set", &[g__THE_CHILD()]).unwrap_or_else(|e| exc::raise(e)))))) {
    return Py::None;
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said_now = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&text.clone(), &said_now.clone())) } }) {
    continue;
    }
    numbers = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__AGE_FIGURE(), "finditer", &[text.clone()])) { let mut match_: Py = __item.clone(); __out.push(py_dict(vec![(py_str("value"), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e))), (py_str("unit"), mcall(&match_.clone(), "group", &[Py::Int(2)]))])); } py_list(__out) })() };
    age = _age_in_months(&[py_dict(vec![(py_str("clean"), text.clone()), (py_str("words"), mcall(&text.clone(), "split", &[])), (py_str("numbers"), numbers.clone())])]);
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    return age.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _age_in_months(args: &[Py]) -> Py {
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
    let mut bare: Py = Py::None;
    let mut number: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The age the line gives, in months, or None when it names none.\n\n    A figure with a unit (\"9 month old\", \"13 months\", \"1 year\") is read from the engine's own\n    numbers, so a line written in digits and a line written in words read the same. A bare\n    figure after \"baby is\" is only read when it is a 1, which is how a person says a first\n    birthday; any other bare figure is too ambiguous to guess at, and no age is better than a\n    wrong stage.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("numbers")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[number.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    value = mcall(&number.clone(), "get", &[py_str("value")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { op_lt(&value.clone(), &Py::Int(0)) } }) {
    continue;
    }
    unit = mcall(&call_builtin("str", &[{ let __b = mcall(&number.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) {
    return value.clone();
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("year")])) {
    return op_mul(&Py::Int(12), &value.clone());
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) {
    return Py::Int(0);
    }
    }
    said = _text(&[understood.clone()]);
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bbaby is (one|1)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(one|1) year old\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Int(12);
    }
    let __iter2 = py_iter_or_raise(&g__NUMBER_WORDS());
    for __item in __iter2 {
    words = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_modulo(&py_str("\\b%s\\s+months?\\b"), &words.clone()), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__NUMBER_WORDS_AT(), &words.clone());
    }
    if py_truthy(&stdlib_call("re", "search", &[op_modulo(&py_str("\\b%s\\s+years?[\\s-]*old\\b"), &words.clone()), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return op_mul(&Py::Int(12), &geti(&g__NUMBER_WORDS_AT(), &words.clone()));
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__OLDER_CHILD_WORD()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[op_modulo(&py_str("\\b%s\\b"), &word.clone()), said.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    bare = stdlib_call("re", "search", &[op_modulo(&py_str("\\b(?:%s)\\s+(?:is\\s+|are\\s+|aged\\s+|at\\s+|of\\s+)?(\\d{1,2})\\b(?!\\s*(?:month|months|mth|mths|week|weeks|wk|wks|day|days|stone|kilos|kilo|kg|lbs|pounds)\\b)"), &mcall(&py_str("|"), "join", &[g__OLDER_CHILD_WORD()])), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { { let __cmp31: Py = call_builtin("int", &[mcall(&bare.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(2), &__cmp31)) { op_le(&__cmp31, &Py::Int(19)) } else { Py::Bool(false) } } } }) {
    return op_mul(&Py::Int(12), &call_builtin("int", &[mcall(&bare.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)));
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(toddler|toddlers)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Int(12);
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(newborn|newborns)\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Int(0);
    }
    return Py::None;
    Py::None
}
pub fn _stage(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut stage: Py = Py::None;
    let mut until: Py = Py::None;
    let _ = py_str("The reply for the stage the line's age lands on, or None when it names no age.");
    age = _age_in_months(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_STAGES());
    for __item in __iter1 {
    stage = __item.clone();
    until = geti(&stage.clone(), &py_str("until"));
    if py_truthy(&{ let __b = op_ge(&age.clone(), &geti(&stage.clone(), &py_str("from"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&until.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_le(&age.clone(), &until.clone()) } } } }) {
    return geti(&stage.clone(), &py_str("reply"));
    }
    }
    return Py::None;
    Py::None
}
pub fn _how_much_water(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let _ = py_str("How much water, for the age the line gives or the chat already knows.");
    age = _age_in_months(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(6)) } }) {
    return op_add(&g__WATER_UNDER_SIX(), &g__WATER_JUICE());
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_ge(&age.clone(), &Py::Int(12)) } }) {
    return op_add(&g__WATER_OVER_ONE(), &g__WATER_JUICE());
    }
    return op_add(&g__WATER_NO_AGE(), &g__WATER_JUICE());
    Py::None
}
pub fn _milk_volume(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let _ = py_str("How much milk, for the age the line gives - and for all three bands when it gives none.");
    age = _age_in_months(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(6)) } }) {
    return g__MILK_UNDER_SIX();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(12)) } }) {
    return g__MILK_SIX_TO_TWELVE();
    }
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    return g__MILK_OVER_ONE();
    }
    return op_add(&op_add(&op_add(&op_add(&g__MILK_UNDER_SIX(), &py_str(" ")), &g__MILK_SIX_TO_TWELVE()), &py_str(" ")), &g__MILK_OVER_ONE());
    Py::None
}
pub fn _what_a_baby_drinks(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let _ = py_str("What goes in the cup, for the age the line gives or the chat already knows.");
    age = _age_in_months(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(6)) } }) {
    return g__DRINK_UNDER_SIX();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(12)) } }) {
    return g__DRINK_SIX_TO_TWELVE();
    }
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    return g__DRINK_OVER_ONE();
    }
    return op_add(&op_add(&op_add(&op_add(&g__DRINK_UNDER_SIX(), &py_str(" ")), &g__DRINK_SIX_TO_TWELVE()), &py_str(" ")), &g__DRINK_OVER_ONE());
    Py::None
}
pub fn _snacks(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let _ = py_str("What to give between meals, banded by the age the line names.");
    age = _age_in_months(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(6)) } }) {
    return g__SNACK_UNDER_SIX();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&age.clone(), &Py::Int(12)) } }) {
    return g__SNACK_SIX_TO_TWELVE();
    }
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    return g__SNACK_OVER_ONE();
    }
    return op_add(&op_add(&g__SNACK_SIX_TO_TWELVE(), &py_str(" ")), &g__SNACK_OVER_ONE());
    Py::None
}
pub fn _says_what_they_eat(args: &[Py]) -> Py {
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
    let mut padded: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line is about eating at all, or only has a baby and a question in it.\n\n    The key can only insist on one part after the first two, and an age and an ask are both\n    reachable without a word of eating: \"my baby is breathing fast what should i do\" and \"what do\n    i need for a newborn\" each name the age and the ask and nothing to do with food. So the eating\n    word is checked here, in the reply itself, where both can be required at once.\n    ");
    said = _text(&[understood.clone()]);
    padded = op_modulo(&py_str(" %s "), &said.clone());
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EATING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _plant_milk(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut rice: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A plant milk by the age of the child, because the answer changes twice.\n\n    Under a year it is not a drink at all; rice milk is out until five whatever the age; over a\n    year an unsweetened fortified one is a fair swap. The words are read the same way the milk\n    bands are - the age in the line first, then the age the chat gave - so a follow-up gets the\n    same answer as the question it follows.\n    ");
    age = _age_in_months(&[understood.clone()]);
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    }
    said = _text(&[understood.clone()]);
    rice = Py::Bool(py_contains(&said.clone(), &py_str("rice")));
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    return op_add(&op_add(&g__PLANT_MILK_BABY(), &py_str(" ")), &g__PLANT_MILK_RICE());
    }
    if py_truthy(&op_lt(&age.clone(), &Py::Int(12))) {
    return op_add(&g__PLANT_MILK_BABY(), &if py_truthy(&rice.clone()) { op_add(&py_str(" "), &g__PLANT_MILK_RICE()) } else { py_str("") });
    }
    if py_truthy(&rice.clone()) {
    return g__PLANT_MILK_RICE();
    }
    return g__PLANT_MILK_CHILD();
    Py::None
}
pub fn _by_age(args: &[Py]) -> Py {
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
    let _ = py_str("The stage the line names, and the first foods for a line that names no age.\n\n    A line with no word of eating in it is not answered from this table at all: returning None\n    lets it go on to the row that does answer it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_says_what_they_eat(&[understood.clone()])))) {
    return Py::None;
    }
    return { let __b = _stage(&[understood.clone()]); if py_truthy(&__b) { __b } else { g__FIRST_FOODS() } };
    Py::None
}
pub fn _names_no_baby(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The line with the foods that call themselves baby taken out.");
    low = op_modulo(&py_str(" %s "), &text.clone());
    let __iter1 = py_iter_or_raise(&g__NOT_A_BABY_NAME());
    for __item in __iter1 {
    name = __item.clone();
    low = mcall(&low.clone(), "replace", &[op_modulo(&py_str(" %s "), &name.clone()), py_str(" thing ")]);
    }
    return low.clone();
    Py::None
}
pub fn _a_baby_is_named(args: &[Py]) -> Py {
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
    let mut hit: Py = Py::None;
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line itself says whose food this is: a baby, an age, a little one, a child.");
    low = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&mcall(&g__AGE_FIGURE(), "search", &[low.clone()])) {
    return Py::Bool(true);
    }
    hit = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NAMES_A_BABY()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = hit.clone(); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&hit.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("month"), py_str("months")]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g__A_RATE_IN_MONTHS(), "search", &[low.clone()]) } } } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[hit.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_condition_of_this_food(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut organ: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Is the line's condition about this row's food, and is that the whole question?\n\n    A condition is not a food, but only the row whose food is the organ has to stand down.\n    `my baby has kidney problems` scores on this table's kidney row, and that row is the offal\n    card, so it has to go; `my toddler has a heart condition, is cheese ok` scores on the cheese\n    row and the person is asking about cheese, so the cheese card is the answer.\n\n    The line also has to name no other food of the row: the offal card is keyed on pate as well as\n    liver, and `my daughter had a liver transplant, can she have pate` is a question about pate -\n    the transplant is the reason the question is being asked, not a different question.\n    ");
    text = _text(&[understood.clone()]);
    match_ = mcall(&g__A_CONDITION(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(py_is(&match_.clone(), &Py::None))) {
    return Py::Bool(false);
    }
    organ = mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[]);
    if py_truthy(&mcall(&organ.clone(), "endswith", &[py_str("s")])) {
    organ = slice_or_raise(&organ.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    low = op_add(&op_add(&py_str(" "), &mcall(&text.clone(), "lower", &[])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &organ.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    rest = _text(&[py_dict(vec![(py_str("clean"), op_add(&op_add(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&text.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)))])]);
    if py_truthy(&mcall(&g__A_CONDITION(), "search", &[rest.clone()])) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    word = mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("(?<!\\w)"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?!\\w)")), mcall(&rest.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_table_word_is_named(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut name: Py = Py::None;
    let mut word: Py = Py::None;
    low = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_TABLE_WORD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_TABLE_FOOD()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &name.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_pronoun_is_in_the_line(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    low = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_CHILD_PRONOUN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_table_word_is_named(&[understood.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AN_ADULT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _a_baby_or_a_pronoun(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The feeding rows' guard: the line has to be about a young child, not an adult.\n\n    A pronoun is the child only when the line names nobody grown-up. \"what can i give him for\n    lunch\" is a baby the chat has already named; \"my nan is diabetic, what can i take to her for\n    tea\" is a nan, and the first-foods paragraph answered it because the key takes a pronoun and\n    an eating verb and asks nothing else.\n\n    Stricter than `_a_pronoun_is_in_the_line` on purpose, and only the feeding rows use it: the\n    rows that answer a texture (\"my mum thinks she should have lumps by now\") or a food by name\n    (\"my nan says give her a bit of porridge\") read the adult's name as an opinion about the\n    baby's meal, which is right for them and wrong for this table's oldest question.\n    ");
    if py_truthy(&_a_baby_is_named(&[understood.clone()])) {
    return Py::Bool(true);
    }
    low = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_CHILD_PRONOUN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__AN_ADULT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _an_age_of_a_year_or_more(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let _ = py_str("True when the line gives the child an age of a year or more.\n\n    One reader for every shape people write an age in, so the guard and the stage table cannot\n    disagree about the same line: a figure with a unit (\"18 months\", \"4 years\"), a spelled age\n    with \"old\" (\"my two year old\"), and a bare figure beside a word for an older child (\"my kid\n    4\", \"my daughter 6\"), because nobody says \"my kid 4\" of a baby of four months. Beside \"baby\"\n    a bare figure is months, and `_age_in_months` leaves it unread, exactly as before.\n    ");
    age = _age_in_months(&[understood.clone()]);
    return { let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_ge(&age.clone(), &Py::Int(12)) } };
    Py::None
}
pub fn _of_weaning_age(args: &[Py]) -> Py {
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
    let _ = py_str("The feeding rows' other guard: a first food is for a child young enough to need one.");
    return Py::Bool(!py_truthy(&_an_age_of_a_year_or_more(&[understood.clone()])));
    Py::None
}
pub fn _says_the_speaker_is_eating(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut later: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line is somebody asking about their own food rather than a baby's.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FOR_ME()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g__SELF_PRONOUN(), &word.clone()))) {
    if py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_add(&g__A_CHILD(), &g_BABY()), &geti(&words.clone(), &op_add(&index.clone(), &Py::Int(1))))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &op_add(&index.clone(), &Py::Int(1)), &op_add(&index.clone(), &Py::Int(4)), &Py::None)) { let mut later: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__OWN_EATING(), &later.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_line_feeds_somebody(args: &[Py]) -> Py {
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
    let mut act: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut earlier: Py = Py::None;
    let mut feeds: Py = Py::None;
    let mut names_a_baby: Py = Py::None;
    let mut padded: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_now: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word_list: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether this line is about feeding a baby, the ages they are, or the child just named.\n\n    A food and a word of asking are not a baby. \"how many calories in a slice of white bread\",\n    \"how much sugar is in a can of coke\", \"is full fat milk bad for you\", \"can i eat peanut\n    butter every day\" and \"i have a nut allergy can i eat at a curry house\" all named a food and\n    asked about it, and this table answered every one of them with the baby version - the last\n    to somebody who had just said they were allergic to nuts. So the person being fed is\n    required, and there are only three ways to give them:\n\n    - the line names a baby or an age: \"can my baby have cheese\", \"what can a 7 month old eat\";\n    - the line says \"she\" or \"him\", and the chat has a baby in it a turn or two back:\n      \"can she have peanut butter every day\" after \"my baby is 7 months old\";\n    - the line names nobody at all - \"what about cheese\", \"how much milk should i give\" - and the\n      chat has a baby in it. A line that says \"i\" or \"my\" with no baby word in it is the\n      person's own lunch, whatever the chat was about.\n    ");
    word_list = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    words = call_builtin("set", &[word_list.clone()]).unwrap_or_else(|e| exc::raise(e));
    said = _text(&[understood.clone()]);
    padded = op_modulo(&py_str(" %s "), &said.clone());
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("topic")]), &py_str("pets"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("topic_own")]), &py_str("pets"))) } }) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_a_baby_is_named(&[understood.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&_age_in_months(&[understood.clone()]), &Py::None)) } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FOR_A_GROWN_UP()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    stripped = _names_no_baby(&[said.clone()]);
    names_a_baby = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NAMES_A_BABY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&stripped.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__AGE_FIGURE(), "search", &[stripped.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    feeds = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_FEEDING_PHRASE()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&feeds.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&call_builtin("len", &[word_list.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g__GIVING_WATER_ASKS()) } } } }) {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GIVING_WATER()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    feeds = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__GIVING_WATER_OUT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&feeds.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__INTRODUCES_A_FOOD(), "search", &[padded.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NOT_MY_DIET()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_sub(&g__ALLERGEN_FOOD(), &py_set(vec![py_str("bottle"), py_str("formula")]))) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    feeds = Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names_a_baby.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&feeds.clone())); if !py_truthy(&__b) { __b } else { _says_the_speaker_is_eating(&[word_list.clone(), said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names_a_baby.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__WHILE_SHE_FEEDS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__HER_OWN_EATING(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&_a_baby_is_named(&[understood.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_is(&_age_in_months(&[understood.clone()]), &Py::None))) {
    return Py::Bool(true);
    }
    if py_truthy(&feeds.clone()) {
    return Py::Bool(true);
    }
    if py_truthy(&op_bitand(&words.clone(), &py_set(vec![py_str("he"), py_str("she"), py_str("him"), py_str("her"), py_str("his"), py_str("hers"), py_str("they")]))) {
    return Py::Bool(!py_truthy(&_the_pronoun_is_an_animal(&[understood.clone()])));
    }
    if py_truthy(&_says_the_speaker_is_eating(&[word_list.clone(), said.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BABY_ACT()) { let mut act: Py = __item.clone(); __out.push(Py::Bool(py_contains(&padded.clone(), &op_modulo(&py_str(" %s "), &act.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said_now = mcall(&mcall(&said.clone(), "strip", &[]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&text.clone(), &said_now.clone())) } }) {
    continue;
    }
    earlier = op_modulo(&py_str(" %s "), &text.clone());
    if py_truthy(&mcall(&g__AGE_FIGURE(), "search", &[earlier.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NAMES_A_BABY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&earlier.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_allergen_row(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The allergen answer, unless the line is introducing two people to each other.");
    text = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&{ let __b = mcall(&g__INTRODUCING_PEOPLE(), "search", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ALLERGEN_FOOD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("");
    }
    return g__ALLERGEN_ROW();
    Py::None
}
pub fn _the_pronoun_is_an_animal(args: &[Py]) -> Py {
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
    let mut leans: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the \"he\" or \"she\" of the line points at an animal the chat named.");
    places = py_list(vec![]);
    ref_ = mcall(&understood.clone(), "get", &[py_str("ref")]);
    if py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&places.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&ref_.clone(), "items", &[])) { let __unpk2261 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk2261[0].clone(); let mut value: Py = __unpk2261[1].clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her")]), &key.clone())); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(value.clone()); } } py_list(__out) })() }]);
    }
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&places.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("subject"), py_str("text")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    let __iter1 = py_iter_or_raise(&places.clone());
    for __item in __iter1 {
    place = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z']+"), mcall(&place.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__ANIMAL(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_pronoun_is_a_grown_up(args: &[Py]) -> Py {
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
    let mut place: Py = Py::None;
    let mut ref_: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the \"she\" or \"he\" of the line points at a grown-up the chat named.\n\n    \"my mum\" behind the pronoun is a person, and a person with no age this table can read is not\n    a baby: the food rules here are for a child under three. A referent that carries a baby word\n    (\"my baby\", \"my little one\", \"my son\" while the chat gives him months) is not this.\n    ");
    ref_ = mcall(&understood.clone(), "get", &[py_str("ref")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[ref_.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("he"), py_str("she"), py_str("they"), py_str("him"), py_str("her")]));
    for __item in __iter1 {
    key = __item.clone();
    place = mcall(&ref_.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[place.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&place.clone(), "strip", &[]))) } }) {
    continue;
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z']+"), mcall(&place.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g__BABY(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g__A_CHILD_PRONOUN(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("one"), py_str("baby"), py_str("child")]), &word.clone())) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__GROWN_UP(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _verdict(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut extra: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "extra") { v.clone() }
    else { py_tuple(vec![]) };
    let mut bias: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "bias") { v.clone() }
    else { Py::Int(0) };
    let mut own_ask: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "own_ask") { v.clone() }
    else { Py::None };
    let mut pronoun: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pronoun") { v.clone() }
    else { Py::Bool(false) };
    let mut answer: Py = Py::None;
    let mut child: Py = Py::None;
    let _ = py_str("One verdict row: the food by name, the baby, and the word that asks for a verdict.\n\n    The asking words are in the key and the baby is in `ask`. Score counts two of a key's parts\n    and the subject has to be said, so food plus asking word is already two: with the asking\n    words in `ask` instead, \"can i eat bananas\" and \"should i give up sugar\" reached rows about\n    babies. The baby is the part that makes the row this table's, so it is the part that is\n    required, and the asking words stay in the key where they rank the row against the others.\n\n    The baby is often not in the line at all: \"my little one is 9 months old\" then \"can he have\n    strawberries\", or \"my baby has reflux\" then \"is it ok to give water\". So the asking words are\n    the reach (`ask=_VERDICT_ASK`) and this reply is what decides, by reading the line and the\n    chat behind it: a baby, an age or a child in the line; an age the chat is about, read with the\n    reader the age rows use (`_age_in_the_chat`); a person pronoun in the line the follow-up\n    reader rewrote (\"and egg\" -> \"can i give her egg\"); or a baby word in the line it leans on.\n    An empty reply is how an entry says \"not mine\", so the answer the line had before the table\n    gets the turn.\n    ");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut bias: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut child: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut extra: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut names: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut own_ask: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut pronoun: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(8).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(9).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut age: Py = Py::None;
    let mut before: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    if py_truthy(&_a_condition_of_this_food(&[names.clone(), understood.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&own_ask.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { py_call(&own_ask, &[understood.clone()]) } }) {
    return reply.clone();
    }
    if py_truthy(&_a_baby_is_named(&[understood.clone()])) {
    return reply.clone();
    }
    if py_truthy(&_a_pronoun_is_in_the_line(&[understood.clone()])) {
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    return if py_truthy(&op_le(&age.clone(), &g__A_BABY_TILL())) { reply.clone() } else { py_str("") };
    }
    if py_truthy(&{ let __b = _the_pronoun_is_a_grown_up(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_table_word_is_named(&[understood.clone()]))) } }) {
    return py_str("");
    }
    return reply.clone();
    }
    age = _age_in_the_chat(&[memory.clone(), understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_le(&age.clone(), &g__A_BABY_TILL()) } }) {
    return reply.clone();
    }
    leans = mcall(&understood.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&leans.clone(), "get", &[py_str("words")]), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    before = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&leans.clone(), &py_str("words"))) { let mut w: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[w.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(w.clone()); } } py_list(__out) })() }])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g__BABY(), &g__A_CHILD_PRONOUN())) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&before.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return py_str("");
    Py::None
}
    answer = {{ let answer = answer.clone(); let bias = bias.clone(); let child = child.clone(); let extra = extra.clone(); let names = names.clone(); let own_ask = own_ask.clone(); let pronoun = pronoun.clone(); let reply = reply.clone(); py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone(), bias.clone(), child.clone(), extra.clone(), names.clone(), own_ask.clone(), pronoun.clone(), reply.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) }) }};
    child = if py_truthy(&pronoun.clone()) { op_add(&g_BABY(), &g__A_CHILD_PRONOUN()) } else { g_BABY() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("babyfood-"), &stdlib_call("re", "sub", &[py_str("[^a-z0-9]+"), py_str("-"), geti(&names.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)))); __args.push(py_tuple(vec![names.clone(), child.clone(), g__VERDICT_ASK()])); __args.push(answer.clone()); __kws.extend(vec![("ask".to_string(), g__VERDICT_ASK()), ("bias".to_string(), bias.clone()), ("avoid".to_string(), op_add(&g_STAND_DOWN(), &extra.clone()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) };
    Py::None
}
pub fn _not_a_season_of_a_show(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True unless \"season\" here is episodes or a league: then the spice row is not the answer.");
    low = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g__BABY(), &g__A_CHILD())) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&mcall(&g__SEASON_OF_A_SHOW(), "search", &[low.clone()])));
    Py::None
}
pub fn _a_child_is_in_the_line(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line names the child whose food this is, so the spice row may answer.\n\n    A pronoun counts, because the follow-up reader puts one in front of a lean (\"and cinnamon\" ->\n    \"can i give her cinnamon\"), but only while nothing in the line is a season of a show or a\n    league: \"he only started this season\" has a pronoun and is about a footballer.\n    ");
    low = op_add(&op_add(&py_str(" "), &_text(&[understood.clone()])), &py_str(" "));
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_BABY(), &g__A_CHILD())) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { mcall(&g__AGE_FIGURE(), "search", &[low.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__SEASON_OF_A_SHOW(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_CHILD_PRONOUN()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_line_that_feeds_a_baby(args: &[Py]) -> Py {
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
    let mut carried: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether this line, or the line it leans on, names somebody being fed.\n\n    The same test the reply makes, made before the best entry is chosen rather than after. An\n    empty reply is how a row says \"not mine\", but a row that says it has already taken the\n    turn: the topic the next line leans on is read from the entry that won, and the rows either\n    side lose a line they would have answered.\n    ");
    if py_truthy(&_the_line_feeds_somebody(&[understood.clone(), py_dict(vec![])])) {
    return Py::Bool(true);
    }
    leans = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("leans_on")]) } else { Py::None };
    words = if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&leans.clone(), "get", &[py_str("words")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[words.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    carried = op_modulo(&py_str(" %s "), &mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]));
    if py_truthy(&mcall(&g__AGE_FIGURE(), "search", &[carried.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NAMES_A_BABY()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&carried.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _only_for_a_baby(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut row: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "row") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: row"))) };
    let mut gated: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("One gate for the whole table: nobody who is not feeding a baby is answered from it.\n\n    The check is done here, once, rather than in every row: a row that names a food is one\n    sentence away from the same question an adult asks about their own lunch, and the two are\n    told apart by who is named, not by the food. A row's own reply callable still runs inside.\n    ");
    reply = gattr(&row.clone(), "reply");
pub fn __nested_gated_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut gated: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut row: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&_the_line_feeds_somebody(&[understood.clone(), memory.clone()])))) {
    return Py::None;
    }
    return if py_truthy(&call_builtin("callable", &[reply.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_call(&reply, &[memory.clone(), understood.clone()]) } else { reply.clone() };
    Py::None
}
    gated = {{ let gated = gated.clone(); let reply = reply.clone(); let row = row.clone(); py_func("gated", move |args: &[Py]| { let mut full = vec![gated.clone(), reply.clone(), row.clone()]; full.extend_from_slice(args); __nested_gated_0(&full) }) }};
    let _ = py_setattr(&row.clone(), "reply", gated.clone());
    if py_truthy(&Py::Bool(py_is(&call_builtin("getattr", &[row.clone(), py_str("guard"), Py::None]).unwrap_or_else(|e| exc::raise(e)), &Py::None))) {
    let _ = py_setattr(&row.clone(), "guard", py_func("_a_line_that_feeds_a_baby", |args: &[Py]| _a_line_that_feeds_a_baby(args)));
    }
    return row.clone();
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.babyfood".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGE" => Some(g_AGE()),
            "ANSWERS" => Some(g_ANSWERS()),
            "A_BODY_LUMP" => Some(g_A_BODY_LUMP()),
            "BABY" => Some(g_BABY()),
            "BOOK_WORDS" => Some(g_BOOK_WORDS()),
            "EATING" => Some(g_EATING()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FOODS" => Some(g_FOODS()),
            "JOB_WORDS" => Some(g_JOB_WORDS()),
            "NOT_ABOUT_WATER" => Some(g_NOT_ABOUT_WATER()),
            "NOT_AN_AGE" => Some(g_NOT_AN_AGE()),
            "NOT_A_BABY" => Some(g_NOT_A_BABY()),
            "NOT_A_MENU" => Some(g_NOT_A_MENU()),
            "SPICE_NAMES" => Some(g_SPICE_NAMES()),
            "STAGES" => Some(g_STAGES()),
            "STAND_DOWN" => Some(g_STAND_DOWN()),
            "WEANING_ELSEWHERE" => Some(g_WEANING_ELSEWHERE()),
            "_AGE_FIGURE" => Some(g__AGE_FIGURE()),
            "_ALLERGEN_FOOD" => Some(g__ALLERGEN_FOOD()),
            "_ALLERGEN_ROW" => Some(g__ALLERGEN_ROW()),
            "_ANIMAL" => Some(g__ANIMAL()),
            "_AN_ADULT" => Some(g__AN_ADULT()),
            "_A_BABY_TILL" => Some(g__A_BABY_TILL()),
            "_A_BARE_YEAR_FIGURE" => Some(g__A_BARE_YEAR_FIGURE()),
            "_A_CHILD" => Some(g__A_CHILD()),
            "_A_CHILD_PRONOUN" => Some(g__A_CHILD_PRONOUN()),
            "_A_CONDITION" => Some(g__A_CONDITION()),
            "_A_COUNT" => Some(g__A_COUNT()),
            "_A_DRINK" => Some(g__A_DRINK()),
            "_A_FEEDING_PHRASE" => Some(g__A_FEEDING_PHRASE()),
            "_A_ORGAN" => Some(g__A_ORGAN()),
            "_A_RATE_IN_MONTHS" => Some(g__A_RATE_IN_MONTHS()),
            "_A_TABLE_FOOD" => Some(g__A_TABLE_FOOD()),
            "_A_TABLE_WORD" => Some(g__A_TABLE_WORD()),
            "_BABY" => Some(g__BABY()),
            "_BABY_ACT" => Some(g__BABY_ACT()),
            "_DRINK_OVER_ONE" => Some(g__DRINK_OVER_ONE()),
            "_DRINK_SIX_TO_TWELVE" => Some(g__DRINK_SIX_TO_TWELVE()),
            "_DRINK_UNDER_SIX" => Some(g__DRINK_UNDER_SIX()),
            "_FIRST_FOODS" => Some(g__FIRST_FOODS()),
            "_FOR_A_GROWN_UP" => Some(g__FOR_A_GROWN_UP()),
            "_FOR_ME" => Some(g__FOR_ME()),
            "_GIVING_WATER" => Some(g__GIVING_WATER()),
            "_GIVING_WATER_ASKS" => Some(g__GIVING_WATER_ASKS()),
            "_GIVING_WATER_OUT" => Some(g__GIVING_WATER_OUT()),
            "_GROWN_UP" => Some(g__GROWN_UP()),
            "_HER_OWN_EATING" => Some(g__HER_OWN_EATING()),
            "_HOW_MUCH_MILK" => Some(g__HOW_MUCH_MILK()),
            "_INTRODUCES_A_FOOD" => Some(g__INTRODUCES_A_FOOD()),
            "_INTRODUCING_PEOPLE" => Some(g__INTRODUCING_PEOPLE()),
            "_MILK_HOW_MUCH" => Some(g__MILK_HOW_MUCH()),
            "_MILK_OVER_ONE" => Some(g__MILK_OVER_ONE()),
            "_MILK_SIX_TO_TWELVE" => Some(g__MILK_SIX_TO_TWELVE()),
            "_MILK_UNDER_SIX" => Some(g__MILK_UNDER_SIX()),
            "_MILK_WORDS" => Some(g__MILK_WORDS()),
            "_NAMES_A_BABY" => Some(g__NAMES_A_BABY()),
            "_NOT_A_BABY_MILK" => Some(g__NOT_A_BABY_MILK()),
            "_NOT_A_BABY_NAME" => Some(g__NOT_A_BABY_NAME()),
            "_NOT_A_SNACK" => Some(g__NOT_A_SNACK()),
            "_NOT_MY_DIET" => Some(g__NOT_MY_DIET()),
            "_NUMBER_WORDS" => Some(g__NUMBER_WORDS()),
            "_NUMBER_WORDS_AT" => Some(g__NUMBER_WORDS_AT()),
            "_OLDER_CHILD_WORD" => Some(g__OLDER_CHILD_WORD()),
            "_OWN_EATING" => Some(g__OWN_EATING()),
            "_PLANT_MILK_BABY" => Some(g__PLANT_MILK_BABY()),
            "_PLANT_MILK_CHILD" => Some(g__PLANT_MILK_CHILD()),
            "_PLANT_MILK_RICE" => Some(g__PLANT_MILK_RICE()),
            "_SEASON_OF_A_SHOW" => Some(g__SEASON_OF_A_SHOW()),
            "_SELF_PRONOUN" => Some(g__SELF_PRONOUN()),
            "_SNACK" => Some(g__SNACK()),
            "_SNACK_CHILD" => Some(g__SNACK_CHILD()),
            "_SNACK_OVER_ONE" => Some(g__SNACK_OVER_ONE()),
            "_SNACK_SIX_TO_TWELVE" => Some(g__SNACK_SIX_TO_TWELVE()),
            "_SNACK_UNDER_SIX" => Some(g__SNACK_UNDER_SIX()),
            "_THE_CHILD" => Some(g__THE_CHILD()),
            "_THE_REFUSAL_WORDS" => Some(g__THE_REFUSAL_WORDS()),
            "_UNIT_WITHOUT_FIGURE" => Some(g__UNIT_WITHOUT_FIGURE()),
            "_VERDICT_ASK" => Some(g__VERDICT_ASK()),
            "_WAKE_TO_FEED_KEY" => Some(g__WAKE_TO_FEED_KEY()),
            "_WAKE_TO_FEED_WORDS" => Some(g__WAKE_TO_FEED_WORDS()),
            "_WATER_JUICE" => Some(g__WATER_JUICE()),
            "_WATER_NO_AGE" => Some(g__WATER_NO_AGE()),
            "_WATER_OVER_ONE" => Some(g__WATER_OVER_ONE()),
            "_WATER_UNDER_SIX" => Some(g__WATER_UNDER_SIX()),
            "_WHAT_TO_FEED" => Some(g__WHAT_TO_FEED()),
            "_WHILE_SHE_FEEDS" => Some(g__WHILE_SHE_FEEDS()),
            "_a_baby_is_named" => Some(py_func("_a_baby_is_named", |args: &[Py]| _a_baby_is_named(args))),
            "_a_baby_or_a_pronoun" => Some(py_func("_a_baby_or_a_pronoun", |args: &[Py]| _a_baby_or_a_pronoun(args))),
            "_a_child_is_in_the_line" => Some(py_func("_a_child_is_in_the_line", |args: &[Py]| _a_child_is_in_the_line(args))),
            "_a_condition_of_this_food" => Some(py_func("_a_condition_of_this_food", |args: &[Py]| _a_condition_of_this_food(args))),
            "_a_line_that_feeds_a_baby" => Some(py_func("_a_line_that_feeds_a_baby", |args: &[Py]| _a_line_that_feeds_a_baby(args))),
            "_a_pronoun_is_in_the_line" => Some(py_func("_a_pronoun_is_in_the_line", |args: &[Py]| _a_pronoun_is_in_the_line(args))),
            "_a_table_word_is_named" => Some(py_func("_a_table_word_is_named", |args: &[Py]| _a_table_word_is_named(args))),
            "_age_in_months" => Some(py_func("_age_in_months", |args: &[Py]| _age_in_months(args))),
            "_age_in_the_chat" => Some(py_func("_age_in_the_chat", |args: &[Py]| _age_in_the_chat(args))),
            "_an_age_of_a_year_or_more" => Some(py_func("_an_age_of_a_year_or_more", |args: &[Py]| _an_age_of_a_year_or_more(args))),
            "_by_age" => Some(py_func("_by_age", |args: &[Py]| _by_age(args))),
            "_how_much_water" => Some(py_func("_how_much_water", |args: &[Py]| _how_much_water(args))),
            "_milk_volume" => Some(py_func("_milk_volume", |args: &[Py]| _milk_volume(args))),
            "_names_no_baby" => Some(py_func("_names_no_baby", |args: &[Py]| _names_no_baby(args))),
            "_not_a_season_of_a_show" => Some(py_func("_not_a_season_of_a_show", |args: &[Py]| _not_a_season_of_a_show(args))),
            "_of_weaning_age" => Some(py_func("_of_weaning_age", |args: &[Py]| _of_weaning_age(args))),
            "_only_for_a_baby" => Some(py_func("_only_for_a_baby", |args: &[Py]| _only_for_a_baby(args))),
            "_plant_milk" => Some(py_func("_plant_milk", |args: &[Py]| _plant_milk(args))),
            "_says_the_speaker_is_eating" => Some(py_func("_says_the_speaker_is_eating", |args: &[Py]| _says_the_speaker_is_eating(args))),
            "_says_what_they_eat" => Some(py_func("_says_what_they_eat", |args: &[Py]| _says_what_they_eat(args))),
            "_snacks" => Some(py_func("_snacks", |args: &[Py]| _snacks(args))),
            "_stage" => Some(py_func("_stage", |args: &[Py]| _stage(args))),
            "_text" => Some(py_func("_text", |args: &[Py]| _text(args))),
            "_the_allergen_row" => Some(py_func("_the_allergen_row", |args: &[Py]| _the_allergen_row(args))),
            "_the_line_feeds_somebody" => Some(py_func("_the_line_feeds_somebody", |args: &[Py]| _the_line_feeds_somebody(args))),
            "_the_pronoun_is_a_grown_up" => Some(py_func("_the_pronoun_is_a_grown_up", |args: &[Py]| _the_pronoun_is_a_grown_up(args))),
            "_the_pronoun_is_an_animal" => Some(py_func("_the_pronoun_is_an_animal", |args: &[Py]| _the_pronoun_is_an_animal(args))),
            "_the_water_ask" => Some(py_func("_the_water_ask", |args: &[Py]| _the_water_ask(args))),
            "_verdict" => Some(py_func("_verdict", |args: &[Py]| _verdict(args))),
            "_what_a_baby_drinks" => Some(py_func("_what_a_baby_drinks", |args: &[Py]| _what_a_baby_drinks(args))),
            "entry" => Some(g_entry()),
            "not_the_heat_of_an_argument" => Some(g_not_the_heat_of_an_argument()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGE" => s_AGE(v),
            "ANSWERS" => s_ANSWERS(v),
            "A_BODY_LUMP" => s_A_BODY_LUMP(v),
            "BABY" => s_BABY(v),
            "BOOK_WORDS" => s_BOOK_WORDS(v),
            "EATING" => s_EATING(v),
            "ENTRIES" => s_ENTRIES(v),
            "FOODS" => s_FOODS(v),
            "JOB_WORDS" => s_JOB_WORDS(v),
            "NOT_ABOUT_WATER" => s_NOT_ABOUT_WATER(v),
            "NOT_AN_AGE" => s_NOT_AN_AGE(v),
            "NOT_A_BABY" => s_NOT_A_BABY(v),
            "NOT_A_MENU" => s_NOT_A_MENU(v),
            "SPICE_NAMES" => s_SPICE_NAMES(v),
            "STAGES" => s_STAGES(v),
            "STAND_DOWN" => s_STAND_DOWN(v),
            "WEANING_ELSEWHERE" => s_WEANING_ELSEWHERE(v),
            "_AGE_FIGURE" => s__AGE_FIGURE(v),
            "_ALLERGEN_FOOD" => s__ALLERGEN_FOOD(v),
            "_ALLERGEN_ROW" => s__ALLERGEN_ROW(v),
            "_ANIMAL" => s__ANIMAL(v),
            "_AN_ADULT" => s__AN_ADULT(v),
            "_A_BABY_TILL" => s__A_BABY_TILL(v),
            "_A_BARE_YEAR_FIGURE" => s__A_BARE_YEAR_FIGURE(v),
            "_A_CHILD" => s__A_CHILD(v),
            "_A_CHILD_PRONOUN" => s__A_CHILD_PRONOUN(v),
            "_A_CONDITION" => s__A_CONDITION(v),
            "_A_COUNT" => s__A_COUNT(v),
            "_A_DRINK" => s__A_DRINK(v),
            "_A_FEEDING_PHRASE" => s__A_FEEDING_PHRASE(v),
            "_A_ORGAN" => s__A_ORGAN(v),
            "_A_RATE_IN_MONTHS" => s__A_RATE_IN_MONTHS(v),
            "_A_TABLE_FOOD" => s__A_TABLE_FOOD(v),
            "_A_TABLE_WORD" => s__A_TABLE_WORD(v),
            "_BABY" => s__BABY(v),
            "_BABY_ACT" => s__BABY_ACT(v),
            "_DRINK_OVER_ONE" => s__DRINK_OVER_ONE(v),
            "_DRINK_SIX_TO_TWELVE" => s__DRINK_SIX_TO_TWELVE(v),
            "_DRINK_UNDER_SIX" => s__DRINK_UNDER_SIX(v),
            "_FIRST_FOODS" => s__FIRST_FOODS(v),
            "_FOR_A_GROWN_UP" => s__FOR_A_GROWN_UP(v),
            "_FOR_ME" => s__FOR_ME(v),
            "_GIVING_WATER" => s__GIVING_WATER(v),
            "_GIVING_WATER_ASKS" => s__GIVING_WATER_ASKS(v),
            "_GIVING_WATER_OUT" => s__GIVING_WATER_OUT(v),
            "_GROWN_UP" => s__GROWN_UP(v),
            "_HER_OWN_EATING" => s__HER_OWN_EATING(v),
            "_HOW_MUCH_MILK" => s__HOW_MUCH_MILK(v),
            "_INTRODUCES_A_FOOD" => s__INTRODUCES_A_FOOD(v),
            "_INTRODUCING_PEOPLE" => s__INTRODUCING_PEOPLE(v),
            "_MILK_HOW_MUCH" => s__MILK_HOW_MUCH(v),
            "_MILK_OVER_ONE" => s__MILK_OVER_ONE(v),
            "_MILK_SIX_TO_TWELVE" => s__MILK_SIX_TO_TWELVE(v),
            "_MILK_UNDER_SIX" => s__MILK_UNDER_SIX(v),
            "_MILK_WORDS" => s__MILK_WORDS(v),
            "_NAMES_A_BABY" => s__NAMES_A_BABY(v),
            "_NOT_A_BABY_MILK" => s__NOT_A_BABY_MILK(v),
            "_NOT_A_BABY_NAME" => s__NOT_A_BABY_NAME(v),
            "_NOT_A_SNACK" => s__NOT_A_SNACK(v),
            "_NOT_MY_DIET" => s__NOT_MY_DIET(v),
            "_NUMBER_WORDS" => s__NUMBER_WORDS(v),
            "_NUMBER_WORDS_AT" => s__NUMBER_WORDS_AT(v),
            "_OLDER_CHILD_WORD" => s__OLDER_CHILD_WORD(v),
            "_OWN_EATING" => s__OWN_EATING(v),
            "_PLANT_MILK_BABY" => s__PLANT_MILK_BABY(v),
            "_PLANT_MILK_CHILD" => s__PLANT_MILK_CHILD(v),
            "_PLANT_MILK_RICE" => s__PLANT_MILK_RICE(v),
            "_SEASON_OF_A_SHOW" => s__SEASON_OF_A_SHOW(v),
            "_SELF_PRONOUN" => s__SELF_PRONOUN(v),
            "_SNACK" => s__SNACK(v),
            "_SNACK_CHILD" => s__SNACK_CHILD(v),
            "_SNACK_OVER_ONE" => s__SNACK_OVER_ONE(v),
            "_SNACK_SIX_TO_TWELVE" => s__SNACK_SIX_TO_TWELVE(v),
            "_SNACK_UNDER_SIX" => s__SNACK_UNDER_SIX(v),
            "_THE_CHILD" => s__THE_CHILD(v),
            "_THE_REFUSAL_WORDS" => s__THE_REFUSAL_WORDS(v),
            "_UNIT_WITHOUT_FIGURE" => s__UNIT_WITHOUT_FIGURE(v),
            "_VERDICT_ASK" => s__VERDICT_ASK(v),
            "_WAKE_TO_FEED_KEY" => s__WAKE_TO_FEED_KEY(v),
            "_WAKE_TO_FEED_WORDS" => s__WAKE_TO_FEED_WORDS(v),
            "_WATER_JUICE" => s__WATER_JUICE(v),
            "_WATER_NO_AGE" => s__WATER_NO_AGE(v),
            "_WATER_OVER_ONE" => s__WATER_OVER_ONE(v),
            "_WATER_UNDER_SIX" => s__WATER_UNDER_SIX(v),
            "_WHAT_TO_FEED" => s__WHAT_TO_FEED(v),
            "_WHILE_SHE_FEEDS" => s__WHILE_SHE_FEEDS(v),
            "entry" => s_entry(v),
            "not_the_heat_of_an_argument" => s_not_the_heat_of_an_argument(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.babyfood", module_obj);
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_the_water_ask", py_func("_the_water_ask", |args: &[Py]| _the_water_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_text", py_func("_text", |args: &[Py]| _text(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_age_in_the_chat", py_func("_age_in_the_chat", |args: &[Py]| _age_in_the_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_age_in_months", py_func("_age_in_months", |args: &[Py]| _age_in_months(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_stage", py_func("_stage", |args: &[Py]| _stage(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_how_much_water", py_func("_how_much_water", |args: &[Py]| _how_much_water(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_milk_volume", py_func("_milk_volume", |args: &[Py]| _milk_volume(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_what_a_baby_drinks", py_func("_what_a_baby_drinks", |args: &[Py]| _what_a_baby_drinks(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_snacks", py_func("_snacks", |args: &[Py]| _snacks(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_says_what_they_eat", py_func("_says_what_they_eat", |args: &[Py]| _says_what_they_eat(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_plant_milk", py_func("_plant_milk", |args: &[Py]| _plant_milk(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_by_age", py_func("_by_age", |args: &[Py]| _by_age(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_names_no_baby", py_func("_names_no_baby", |args: &[Py]| _names_no_baby(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_baby_is_named", py_func("_a_baby_is_named", |args: &[Py]| _a_baby_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_condition_of_this_food", py_func("_a_condition_of_this_food", |args: &[Py]| _a_condition_of_this_food(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_table_word_is_named", py_func("_a_table_word_is_named", |args: &[Py]| _a_table_word_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_pronoun_is_in_the_line", py_func("_a_pronoun_is_in_the_line", |args: &[Py]| _a_pronoun_is_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_baby_or_a_pronoun", py_func("_a_baby_or_a_pronoun", |args: &[Py]| _a_baby_or_a_pronoun(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_an_age_of_a_year_or_more", py_func("_an_age_of_a_year_or_more", |args: &[Py]| _an_age_of_a_year_or_more(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_of_weaning_age", py_func("_of_weaning_age", |args: &[Py]| _of_weaning_age(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_says_the_speaker_is_eating", py_func("_says_the_speaker_is_eating", |args: &[Py]| _says_the_speaker_is_eating(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_the_line_feeds_somebody", py_func("_the_line_feeds_somebody", |args: &[Py]| _the_line_feeds_somebody(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_the_allergen_row", py_func("_the_allergen_row", |args: &[Py]| _the_allergen_row(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_the_pronoun_is_an_animal", py_func("_the_pronoun_is_an_animal", |args: &[Py]| _the_pronoun_is_an_animal(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_the_pronoun_is_a_grown_up", py_func("_the_pronoun_is_a_grown_up", |args: &[Py]| _the_pronoun_is_a_grown_up(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_verdict", py_func("_verdict", |args: &[Py]| _verdict(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_not_a_season_of_a_show", py_func("_not_a_season_of_a_show", |args: &[Py]| _not_a_season_of_a_show(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_child_is_in_the_line", py_func("_a_child_is_in_the_line", |args: &[Py]| _a_child_is_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_a_line_that_feeds_a_baby", py_func("_a_line_that_feeds_a_baby", |args: &[Py]| _a_line_that_feeds_a_baby(args)));
    pyrt::xrefs::reg_fn("knowledge.babyfood", "_only_for_a_baby", py_func("_only_for_a_baby", |args: &[Py]| _only_for_a_baby(args)));
    pyrt::xrefs::reg_access("knowledge.babyfood", "AGE", g_AGE, s_AGE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "A_BODY_LUMP", g_A_BODY_LUMP, s_A_BODY_LUMP);
    pyrt::xrefs::reg_access("knowledge.babyfood", "BABY", g_BABY, s_BABY);
    pyrt::xrefs::reg_access("knowledge.babyfood", "BOOK_WORDS", g_BOOK_WORDS, s_BOOK_WORDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "EATING", g_EATING, s_EATING);
    pyrt::xrefs::reg_access("knowledge.babyfood", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.babyfood", "FOODS", g_FOODS, s_FOODS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "JOB_WORDS", g_JOB_WORDS, s_JOB_WORDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "NOT_ABOUT_WATER", g_NOT_ABOUT_WATER, s_NOT_ABOUT_WATER);
    pyrt::xrefs::reg_access("knowledge.babyfood", "NOT_AN_AGE", g_NOT_AN_AGE, s_NOT_AN_AGE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "NOT_A_BABY", g_NOT_A_BABY, s_NOT_A_BABY);
    pyrt::xrefs::reg_access("knowledge.babyfood", "NOT_A_MENU", g_NOT_A_MENU, s_NOT_A_MENU);
    pyrt::xrefs::reg_access("knowledge.babyfood", "SPICE_NAMES", g_SPICE_NAMES, s_SPICE_NAMES);
    pyrt::xrefs::reg_access("knowledge.babyfood", "STAGES", g_STAGES, s_STAGES);
    pyrt::xrefs::reg_access("knowledge.babyfood", "STAND_DOWN", g_STAND_DOWN, s_STAND_DOWN);
    pyrt::xrefs::reg_access("knowledge.babyfood", "WEANING_ELSEWHERE", g_WEANING_ELSEWHERE, s_WEANING_ELSEWHERE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_AGE_FIGURE", g__AGE_FIGURE, s__AGE_FIGURE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_ALLERGEN_FOOD", g__ALLERGEN_FOOD, s__ALLERGEN_FOOD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_ALLERGEN_ROW", g__ALLERGEN_ROW, s__ALLERGEN_ROW);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_ANIMAL", g__ANIMAL, s__ANIMAL);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_AN_ADULT", g__AN_ADULT, s__AN_ADULT);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_BABY_TILL", g__A_BABY_TILL, s__A_BABY_TILL);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_BARE_YEAR_FIGURE", g__A_BARE_YEAR_FIGURE, s__A_BARE_YEAR_FIGURE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_CHILD", g__A_CHILD, s__A_CHILD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_CHILD_PRONOUN", g__A_CHILD_PRONOUN, s__A_CHILD_PRONOUN);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_CONDITION", g__A_CONDITION, s__A_CONDITION);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_COUNT", g__A_COUNT, s__A_COUNT);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_DRINK", g__A_DRINK, s__A_DRINK);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_FEEDING_PHRASE", g__A_FEEDING_PHRASE, s__A_FEEDING_PHRASE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_ORGAN", g__A_ORGAN, s__A_ORGAN);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_RATE_IN_MONTHS", g__A_RATE_IN_MONTHS, s__A_RATE_IN_MONTHS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_TABLE_FOOD", g__A_TABLE_FOOD, s__A_TABLE_FOOD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_A_TABLE_WORD", g__A_TABLE_WORD, s__A_TABLE_WORD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_BABY", g__BABY, s__BABY);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_BABY_ACT", g__BABY_ACT, s__BABY_ACT);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_DRINK_OVER_ONE", g__DRINK_OVER_ONE, s__DRINK_OVER_ONE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_DRINK_SIX_TO_TWELVE", g__DRINK_SIX_TO_TWELVE, s__DRINK_SIX_TO_TWELVE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_DRINK_UNDER_SIX", g__DRINK_UNDER_SIX, s__DRINK_UNDER_SIX);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_FIRST_FOODS", g__FIRST_FOODS, s__FIRST_FOODS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_FOR_A_GROWN_UP", g__FOR_A_GROWN_UP, s__FOR_A_GROWN_UP);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_FOR_ME", g__FOR_ME, s__FOR_ME);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_GIVING_WATER", g__GIVING_WATER, s__GIVING_WATER);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_GIVING_WATER_ASKS", g__GIVING_WATER_ASKS, s__GIVING_WATER_ASKS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_GIVING_WATER_OUT", g__GIVING_WATER_OUT, s__GIVING_WATER_OUT);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_GROWN_UP", g__GROWN_UP, s__GROWN_UP);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_HER_OWN_EATING", g__HER_OWN_EATING, s__HER_OWN_EATING);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_HOW_MUCH_MILK", g__HOW_MUCH_MILK, s__HOW_MUCH_MILK);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_INTRODUCES_A_FOOD", g__INTRODUCES_A_FOOD, s__INTRODUCES_A_FOOD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_INTRODUCING_PEOPLE", g__INTRODUCING_PEOPLE, s__INTRODUCING_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_MILK_HOW_MUCH", g__MILK_HOW_MUCH, s__MILK_HOW_MUCH);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_MILK_OVER_ONE", g__MILK_OVER_ONE, s__MILK_OVER_ONE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_MILK_SIX_TO_TWELVE", g__MILK_SIX_TO_TWELVE, s__MILK_SIX_TO_TWELVE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_MILK_UNDER_SIX", g__MILK_UNDER_SIX, s__MILK_UNDER_SIX);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_MILK_WORDS", g__MILK_WORDS, s__MILK_WORDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NAMES_A_BABY", g__NAMES_A_BABY, s__NAMES_A_BABY);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NOT_A_BABY_MILK", g__NOT_A_BABY_MILK, s__NOT_A_BABY_MILK);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NOT_A_BABY_NAME", g__NOT_A_BABY_NAME, s__NOT_A_BABY_NAME);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NOT_A_SNACK", g__NOT_A_SNACK, s__NOT_A_SNACK);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NOT_MY_DIET", g__NOT_MY_DIET, s__NOT_MY_DIET);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NUMBER_WORDS", g__NUMBER_WORDS, s__NUMBER_WORDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_NUMBER_WORDS_AT", g__NUMBER_WORDS_AT, s__NUMBER_WORDS_AT);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_OLDER_CHILD_WORD", g__OLDER_CHILD_WORD, s__OLDER_CHILD_WORD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_OWN_EATING", g__OWN_EATING, s__OWN_EATING);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_PLANT_MILK_BABY", g__PLANT_MILK_BABY, s__PLANT_MILK_BABY);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_PLANT_MILK_CHILD", g__PLANT_MILK_CHILD, s__PLANT_MILK_CHILD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_PLANT_MILK_RICE", g__PLANT_MILK_RICE, s__PLANT_MILK_RICE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SEASON_OF_A_SHOW", g__SEASON_OF_A_SHOW, s__SEASON_OF_A_SHOW);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SELF_PRONOUN", g__SELF_PRONOUN, s__SELF_PRONOUN);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SNACK", g__SNACK, s__SNACK);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SNACK_CHILD", g__SNACK_CHILD, s__SNACK_CHILD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SNACK_OVER_ONE", g__SNACK_OVER_ONE, s__SNACK_OVER_ONE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SNACK_SIX_TO_TWELVE", g__SNACK_SIX_TO_TWELVE, s__SNACK_SIX_TO_TWELVE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_SNACK_UNDER_SIX", g__SNACK_UNDER_SIX, s__SNACK_UNDER_SIX);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_THE_CHILD", g__THE_CHILD, s__THE_CHILD);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_THE_REFUSAL_WORDS", g__THE_REFUSAL_WORDS, s__THE_REFUSAL_WORDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_UNIT_WITHOUT_FIGURE", g__UNIT_WITHOUT_FIGURE, s__UNIT_WITHOUT_FIGURE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_VERDICT_ASK", g__VERDICT_ASK, s__VERDICT_ASK);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WAKE_TO_FEED_KEY", g__WAKE_TO_FEED_KEY, s__WAKE_TO_FEED_KEY);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WAKE_TO_FEED_WORDS", g__WAKE_TO_FEED_WORDS, s__WAKE_TO_FEED_WORDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WATER_JUICE", g__WATER_JUICE, s__WATER_JUICE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WATER_NO_AGE", g__WATER_NO_AGE, s__WATER_NO_AGE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WATER_OVER_ONE", g__WATER_OVER_ONE, s__WATER_OVER_ONE);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WATER_UNDER_SIX", g__WATER_UNDER_SIX, s__WATER_UNDER_SIX);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WHAT_TO_FEED", g__WHAT_TO_FEED, s__WHAT_TO_FEED);
    pyrt::xrefs::reg_access("knowledge.babyfood", "_WHILE_SHE_FEEDS", g__WHILE_SHE_FEEDS, s__WHILE_SHE_FEEDS);
    pyrt::xrefs::reg_access("knowledge.babyfood", "entry", g_entry, s_entry);
    pyrt::xrefs::reg_access("knowledge.babyfood", "not_the_heat_of_an_argument", g_not_the_heat_of_an_argument, s_not_the_heat_of_an_argument);
}

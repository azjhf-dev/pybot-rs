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


thread_local! { static G_ACK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AILING_PLANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANOTHER_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_IN_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEDTIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BEFORE_EVENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BRING_NOT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BROKEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARRY_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP_SET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_AT_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COMPANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOK_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROWD_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CROWD_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DAIRY_EXTRAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEFINITION_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DESSERT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DIET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISHES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_SHAPES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_STYLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DOUBLE_NO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DRINK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EMPTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVENT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAVOURITES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FISH_INGREDIENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_ON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_ON_NO_CHEESE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOOD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAME_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLUTEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HARD_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HEALTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HUNGRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IDEA_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPRESS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPRESS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPRESS_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IMPRESS_WORTH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INGREDIENTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IN_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIDS_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KITCHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIGHT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIGHT_SET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LUNCH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAKING_AT_HOME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEAL_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_MATTERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_IDEA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_AN_INGREDIENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_DISH_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_MEAL_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FOOD_LIMITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_FOR_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE_CHEAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE_ENTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MINE_OUT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_MY_KITCHEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_CHEESE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_CUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_DAIRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_GAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_HAVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_HAVE_GAP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_MEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OPENERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_MEAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECIPE_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPAIR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_AS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAYING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHOW_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SICK_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SLEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SMALL_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOUP_NAMED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPOILT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STAPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STYLE_CUES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STYLE_WANT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEOUT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEOUT_PICKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TAKEOUT_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TECHNIQUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TECHNIQUE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TOTAL_TIMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNCOUNTABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNWELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_ALLIUMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_CLASSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_FISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_FRUIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_GRAINS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_GREENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_MEAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_ROOTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UP_SOFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_DISH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_UP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_UP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_UP_TECHNIQUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_GOES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITHOUT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WITH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_MEAT_BLOCKER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UP_FOOD_SET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__row: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__no_have: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ACK_WORDS() -> Py { ensure_init(); G_ACK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ACK_WORDS(v: Py) { ensure_init(); G_ACK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AILING_PLANT() -> Py { ensure_init(); G_AILING_PLANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AILING_PLANT(v: Py) { ensure_init(); G_AILING_PLANT.with(|c| *c.borrow_mut() = v); }
pub fn g_ANOTHER_MEAL() -> Py { ensure_init(); G_ANOTHER_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_MEAL(v: Py) { ensure_init(); G_ANOTHER_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKING_WORDS() -> Py { ensure_init(); G_ASKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKING_WORDS(v: Py) { ensure_init(); G_ASKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_CUES() -> Py { ensure_init(); G_ASK_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_CUES(v: Py) { ensure_init(); G_ASK_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_IN_NEWS() -> Py { ensure_init(); G_ASK_IN_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_IN_NEWS(v: Py) { ensure_init(); G_ASK_IN_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_BEDTIME() -> Py { ensure_init(); G_BEDTIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEDTIME(v: Py) { ensure_init(); G_BEDTIME.with(|c| *c.borrow_mut() = v); }
pub fn g_BEFORE_EVENT() -> Py { ensure_init(); G_BEFORE_EVENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BEFORE_EVENT(v: Py) { ensure_init(); G_BEFORE_EVENT.with(|c| *c.borrow_mut() = v); }
pub fn g_BRING_NOT_FOOD() -> Py { ensure_init(); G_BRING_NOT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BRING_NOT_FOOD(v: Py) { ensure_init(); G_BRING_NOT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_BROKEN() -> Py { ensure_init(); G_BROKEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BROKEN(v: Py) { ensure_init(); G_BROKEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY_ASK() -> Py { ensure_init(); G_CARRY_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY_ASK(v: Py) { ensure_init(); G_CARRY_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CARRY_REPLIES() -> Py { ensure_init(); G_CARRY_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARRY_REPLIES(v: Py) { ensure_init(); G_CARRY_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP() -> Py { ensure_init(); G_CHEAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP(v: Py) { ensure_init(); G_CHEAP.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP_ASK() -> Py { ensure_init(); G_CHEAP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP_ASK(v: Py) { ensure_init(); G_CHEAP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP_SET() -> Py { ensure_init(); G_CHEAP_SET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP_SET(v: Py) { ensure_init(); G_CHEAP_SET.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_AT_TABLE() -> Py { ensure_init(); G_CHILD_AT_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_AT_TABLE(v: Py) { ensure_init(); G_CHILD_AT_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_COMPANY() -> Py { ensure_init(); G_COMPANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COMPANY(v: Py) { ensure_init(); G_COMPANY.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION() -> Py { ensure_init(); G_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION(v: Py) { ensure_init(); G_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_COOK_TIME() -> Py { ensure_init(); G_COOK_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOK_TIME(v: Py) { ensure_init(); G_COOK_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNT_WORDS() -> Py { ensure_init(); G_COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNT_WORDS(v: Py) { ensure_init(); G_COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CROWD_ASK() -> Py { ensure_init(); G_CROWD_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROWD_ASK(v: Py) { ensure_init(); G_CROWD_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_CROWD_SAID() -> Py { ensure_init(); G_CROWD_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CROWD_SAID(v: Py) { ensure_init(); G_CROWD_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_DAIRY_EXTRAS() -> Py { ensure_init(); G_DAIRY_EXTRAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DAIRY_EXTRAS(v: Py) { ensure_init(); G_DAIRY_EXTRAS.with(|c| *c.borrow_mut() = v); }
pub fn g_DEFINITION_SHAPE() -> Py { ensure_init(); G_DEFINITION_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEFINITION_SHAPE(v: Py) { ensure_init(); G_DEFINITION_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g_DESSERT_ASK() -> Py { ensure_init(); G_DESSERT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DESSERT_ASK(v: Py) { ensure_init(); G_DESSERT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_SAID() -> Py { ensure_init(); G_DIET_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_SAID(v: Py) { ensure_init(); G_DIET_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_WORDS() -> Py { ensure_init(); G_DIET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_WORDS(v: Py) { ensure_init(); G_DIET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISHES() -> Py { ensure_init(); G_DISHES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISHES(v: Py) { ensure_init(); G_DISHES.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_SHAPES() -> Py { ensure_init(); G_DISH_SHAPES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_SHAPES(v: Py) { ensure_init(); G_DISH_SHAPES.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_STYLE() -> Py { ensure_init(); G_DISH_STYLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_STYLE(v: Py) { ensure_init(); G_DISH_STYLE.with(|c| *c.borrow_mut() = v); }
pub fn g_DOUBLE_NO() -> Py { ensure_init(); G_DOUBLE_NO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DOUBLE_NO(v: Py) { ensure_init(); G_DOUBLE_NO.with(|c| *c.borrow_mut() = v); }
pub fn g_DRINK() -> Py { ensure_init(); G_DRINK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DRINK(v: Py) { ensure_init(); G_DRINK.with(|c| *c.borrow_mut() = v); }
pub fn g_EMPTY() -> Py { ensure_init(); G_EMPTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EMPTY(v: Py) { ensure_init(); G_EMPTY.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EVENT_WORDS() -> Py { ensure_init(); G_EVENT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVENT_WORDS(v: Py) { ensure_init(); G_EVENT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FAVOURITES() -> Py { ensure_init(); G_FAVOURITES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAVOURITES(v: Py) { ensure_init(); G_FAVOURITES.with(|c| *c.borrow_mut() = v); }
pub fn g_FISH_INGREDIENTS() -> Py { ensure_init(); G_FISH_INGREDIENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FISH_INGREDIENTS(v: Py) { ensure_init(); G_FISH_INGREDIENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_ON() -> Py { ensure_init(); G_FOLLOW_ON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_ON(v: Py) { ensure_init(); G_FOLLOW_ON.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_ON_NO_CHEESE() -> Py { ensure_init(); G_FOLLOW_ON_NO_CHEESE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_ON_NO_CHEESE(v: Py) { ensure_init(); G_FOLLOW_ON_NO_CHEESE.with(|c| *c.borrow_mut() = v); }
pub fn g_FOOD_WORDS() -> Py { ensure_init(); G_FOOD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOOD_WORDS(v: Py) { ensure_init(); G_FOOD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GAME_WORDS() -> Py { ensure_init(); G_GAME_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAME_WORDS(v: Py) { ensure_init(); G_GAME_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GLUTEN() -> Py { ensure_init(); G_GLUTEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLUTEN(v: Py) { ensure_init(); G_GLUTEN.with(|c| *c.borrow_mut() = v); }
pub fn g_GRACE() -> Py { ensure_init(); G_GRACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRACE(v: Py) { ensure_init(); G_GRACE.with(|c| *c.borrow_mut() = v); }
pub fn g_HARD_CLOCK() -> Py { ensure_init(); G_HARD_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HARD_CLOCK(v: Py) { ensure_init(); G_HARD_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g_HEALTH() -> Py { ensure_init(); G_HEALTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HEALTH(v: Py) { ensure_init(); G_HEALTH.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_TO() -> Py { ensure_init(); G_HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); G_HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g_HUNGRY() -> Py { ensure_init(); G_HUNGRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HUNGRY(v: Py) { ensure_init(); G_HUNGRY.with(|c| *c.borrow_mut() = v); }
pub fn g_IDEA_ASK() -> Py { ensure_init(); G_IDEA_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IDEA_ASK(v: Py) { ensure_init(); G_IDEA_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPRESS() -> Py { ensure_init(); G_IMPRESS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPRESS(v: Py) { ensure_init(); G_IMPRESS.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPRESS_ASK() -> Py { ensure_init(); G_IMPRESS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPRESS_ASK(v: Py) { ensure_init(); G_IMPRESS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPRESS_FOR() -> Py { ensure_init(); G_IMPRESS_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPRESS_FOR(v: Py) { ensure_init(); G_IMPRESS_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_IMPRESS_WORTH() -> Py { ensure_init(); G_IMPRESS_WORTH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IMPRESS_WORTH(v: Py) { ensure_init(); G_IMPRESS_WORTH.with(|c| *c.borrow_mut() = v); }
pub fn g_INGREDIENTS() -> Py { ensure_init(); G_INGREDIENTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INGREDIENTS(v: Py) { ensure_init(); G_INGREDIENTS.with(|c| *c.borrow_mut() = v); }
pub fn g_IN_A_DISH() -> Py { ensure_init(); G_IN_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IN_A_DISH(v: Py) { ensure_init(); G_IN_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_KIDS_IDEA() -> Py { ensure_init(); G_KIDS_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIDS_IDEA(v: Py) { ensure_init(); G_KIDS_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_KITCHEN_WORDS() -> Py { ensure_init(); G_KITCHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KITCHEN_WORDS(v: Py) { ensure_init(); G_KITCHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LIGHT_ASK() -> Py { ensure_init(); G_LIGHT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIGHT_ASK(v: Py) { ensure_init(); G_LIGHT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_LIGHT_SET() -> Py { ensure_init(); G_LIGHT_SET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIGHT_SET(v: Py) { ensure_init(); G_LIGHT_SET.with(|c| *c.borrow_mut() = v); }
pub fn g_LUNCH_ASK() -> Py { ensure_init(); G_LUNCH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LUNCH_ASK(v: Py) { ensure_init(); G_LUNCH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_MAKING_AT_HOME() -> Py { ensure_init(); G_MAKING_AT_HOME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAKING_AT_HOME(v: Py) { ensure_init(); G_MAKING_AT_HOME.with(|c| *c.borrow_mut() = v); }
pub fn g_MEAL_WORD() -> Py { ensure_init(); G_MEAL_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEAL_WORD(v: Py) { ensure_init(); G_MEAL_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURING() -> Py { ensure_init(); G_MEASURING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURING(v: Py) { ensure_init(); G_MEASURING.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_MATTERS() -> Py { ensure_init(); G_MONEY_MATTERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_MATTERS(v: Py) { ensure_init(); G_MONEY_MATTERS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_IDEA() -> Py { ensure_init(); G_NOT_AN_IDEA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_IDEA(v: Py) { ensure_init(); G_NOT_AN_IDEA.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_AN_INGREDIENT() -> Py { ensure_init(); G_NOT_AN_INGREDIENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_AN_INGREDIENT(v: Py) { ensure_init(); G_NOT_AN_INGREDIENT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DISH() -> Py { ensure_init(); G_NOT_A_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DISH(v: Py) { ensure_init(); G_NOT_A_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_DISH_NAME() -> Py { ensure_init(); G_NOT_A_DISH_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DISH_NAME(v: Py) { ensure_init(); G_NOT_A_DISH_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MEAL() -> Py { ensure_init(); G_NOT_A_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MEAL(v: Py) { ensure_init(); G_NOT_A_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_MEAL_SUBJECT() -> Py { ensure_init(); G_NOT_A_MEAL_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_MEAL_SUBJECT(v: Py) { ensure_init(); G_NOT_A_MEAL_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FOOD_LIMITS() -> Py { ensure_init(); G_NOT_FOOD_LIMITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FOOD_LIMITS(v: Py) { ensure_init(); G_NOT_FOOD_LIMITS.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_FOR_ME() -> Py { ensure_init(); G_NOT_FOR_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_FOR_ME(v: Py) { ensure_init(); G_NOT_FOR_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE() -> Py { ensure_init(); G_NOT_MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE(v: Py) { ensure_init(); G_NOT_MINE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE_ASK() -> Py { ensure_init(); G_NOT_MINE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE_ASK(v: Py) { ensure_init(); G_NOT_MINE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE_CHEAP() -> Py { ensure_init(); G_NOT_MINE_CHEAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE_CHEAP(v: Py) { ensure_init(); G_NOT_MINE_CHEAP.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE_ENTRY() -> Py { ensure_init(); G_NOT_MINE_ENTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE_ENTRY(v: Py) { ensure_init(); G_NOT_MINE_ENTRY.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MINE_OUT() -> Py { ensure_init(); G_NOT_MINE_OUT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MINE_OUT(v: Py) { ensure_init(); G_NOT_MINE_OUT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_MY_KITCHEN() -> Py { ensure_init(); G_NOT_MY_KITCHEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_MY_KITCHEN(v: Py) { ensure_init(); G_NOT_MY_KITCHEN.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_CHEESE() -> Py { ensure_init(); G_NO_CHEESE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_CHEESE(v: Py) { ensure_init(); G_NO_CHEESE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_CUE() -> Py { ensure_init(); G_NO_CUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_CUE(v: Py) { ensure_init(); G_NO_CUE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_DAIRY() -> Py { ensure_init(); G_NO_DAIRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_DAIRY(v: Py) { ensure_init(); G_NO_DAIRY.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_GAP() -> Py { ensure_init(); G_NO_GAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_GAP(v: Py) { ensure_init(); G_NO_GAP.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_HAVE() -> Py { ensure_init(); G_NO_HAVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_HAVE(v: Py) { ensure_init(); G_NO_HAVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_HAVE_GAP() -> Py { ensure_init(); G_NO_HAVE_GAP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_HAVE_GAP(v: Py) { ensure_init(); G_NO_HAVE_GAP.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_MEAT() -> Py { ensure_init(); G_NO_MEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_MEAT(v: Py) { ensure_init(); G_NO_MEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_OPENERS() -> Py { ensure_init(); G_OPENERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OPENERS(v: Py) { ensure_init(); G_OPENERS.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER_PEOPLE() -> Py { ensure_init(); G_OTHER_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER_PEOPLE(v: Py) { ensure_init(); G_OTHER_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_MEAL() -> Py { ensure_init(); G_OWN_MEAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_MEAL(v: Py) { ensure_init(); G_OWN_MEAL.with(|c| *c.borrow_mut() = v); }
pub fn g_QUICK() -> Py { ensure_init(); G_QUICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUICK(v: Py) { ensure_init(); G_QUICK.with(|c| *c.borrow_mut() = v); }
pub fn g_RECIPE_FOR() -> Py { ensure_init(); G_RECIPE_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECIPE_FOR(v: Py) { ensure_init(); G_RECIPE_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_REPAIR() -> Py { ensure_init(); G_REPAIR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPAIR(v: Py) { ensure_init(); G_REPAIR.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_AS() -> Py { ensure_init(); G_SAID_AS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_AS(v: Py) { ensure_init(); G_SAID_AS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAYING() -> Py { ensure_init(); G_SAYING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAYING(v: Py) { ensure_init(); G_SAYING.with(|c| *c.borrow_mut() = v); }
pub fn g_SHOW_OFF() -> Py { ensure_init(); G_SHOW_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHOW_OFF(v: Py) { ensure_init(); G_SHOW_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_SICK_OF() -> Py { ensure_init(); G_SICK_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SICK_OF(v: Py) { ensure_init(); G_SICK_OF.with(|c| *c.borrow_mut() = v); }
pub fn g_SLEEP() -> Py { ensure_init(); G_SLEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SLEEP(v: Py) { ensure_init(); G_SLEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_SMALL_PEOPLE() -> Py { ensure_init(); G_SMALL_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SMALL_PEOPLE(v: Py) { ensure_init(); G_SMALL_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_SOUP_NAMED() -> Py { ensure_init(); G_SOUP_NAMED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOUP_NAMED(v: Py) { ensure_init(); G_SOUP_NAMED.with(|c| *c.borrow_mut() = v); }
pub fn g_SPOILT_FOOD() -> Py { ensure_init(); G_SPOILT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPOILT_FOOD(v: Py) { ensure_init(); G_SPOILT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g_STAPLE() -> Py { ensure_init(); G_STAPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STAPLE(v: Py) { ensure_init(); G_STAPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_STYLE_CUES() -> Py { ensure_init(); G_STYLE_CUES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STYLE_CUES(v: Py) { ensure_init(); G_STYLE_CUES.with(|c| *c.borrow_mut() = v); }
pub fn g_STYLE_WANT() -> Py { ensure_init(); G_STYLE_WANT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STYLE_WANT(v: Py) { ensure_init(); G_STYLE_WANT.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEOUT_ASK() -> Py { ensure_init(); G_TAKEOUT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEOUT_ASK(v: Py) { ensure_init(); G_TAKEOUT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEOUT_PICKS() -> Py { ensure_init(); G_TAKEOUT_PICKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEOUT_PICKS(v: Py) { ensure_init(); G_TAKEOUT_PICKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TAKEOUT_PLACES() -> Py { ensure_init(); G_TAKEOUT_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TAKEOUT_PLACES(v: Py) { ensure_init(); G_TAKEOUT_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g_TECHNIQUE() -> Py { ensure_init(); G_TECHNIQUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TECHNIQUE(v: Py) { ensure_init(); G_TECHNIQUE.with(|c| *c.borrow_mut() = v); }
pub fn g_TECHNIQUE_ASK() -> Py { ensure_init(); G_TECHNIQUE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TECHNIQUE_ASK(v: Py) { ensure_init(); G_TECHNIQUE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_TIMES() -> Py { ensure_init(); G_TIMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIMES(v: Py) { ensure_init(); G_TIMES.with(|c| *c.borrow_mut() = v); }
pub fn g_TOTAL_TIMES() -> Py { ensure_init(); G_TOTAL_TIMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TOTAL_TIMES(v: Py) { ensure_init(); G_TOTAL_TIMES.with(|c| *c.borrow_mut() = v); }
pub fn g_UNCOUNTABLE() -> Py { ensure_init(); G_UNCOUNTABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNCOUNTABLE(v: Py) { ensure_init(); G_UNCOUNTABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNWELL() -> Py { ensure_init(); G_UNWELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNWELL(v: Py) { ensure_init(); G_UNWELL.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_ALLIUMS() -> Py { ensure_init(); G_UP_ALLIUMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_ALLIUMS(v: Py) { ensure_init(); G_UP_ALLIUMS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_CLASSES() -> Py { ensure_init(); G_UP_CLASSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_CLASSES(v: Py) { ensure_init(); G_UP_CLASSES.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_FISH() -> Py { ensure_init(); G_UP_FISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_FISH(v: Py) { ensure_init(); G_UP_FISH.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_FOODS() -> Py { ensure_init(); G_UP_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_FOODS(v: Py) { ensure_init(); G_UP_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_FRUIT() -> Py { ensure_init(); G_UP_FRUIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_FRUIT(v: Py) { ensure_init(); G_UP_FRUIT.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_GRAINS() -> Py { ensure_init(); G_UP_GRAINS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_GRAINS(v: Py) { ensure_init(); G_UP_GRAINS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_GREENS() -> Py { ensure_init(); G_UP_GREENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_GREENS(v: Py) { ensure_init(); G_UP_GREENS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_MEAT() -> Py { ensure_init(); G_UP_MEAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_MEAT(v: Py) { ensure_init(); G_UP_MEAT.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_ROOTS() -> Py { ensure_init(); G_UP_ROOTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_ROOTS(v: Py) { ensure_init(); G_UP_ROOTS.with(|c| *c.borrow_mut() = v); }
pub fn g_UP_SOFT() -> Py { ensure_init(); G_UP_SOFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UP_SOFT(v: Py) { ensure_init(); G_UP_SOFT.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_DISH_WORDS() -> Py { ensure_init(); G_USE_DISH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_DISH_WORDS(v: Py) { ensure_init(); G_USE_DISH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_UP() -> Py { ensure_init(); G_USE_UP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_UP(v: Py) { ensure_init(); G_USE_UP.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_UP_ASK() -> Py { ensure_init(); G_USE_UP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_UP_ASK(v: Py) { ensure_init(); G_USE_UP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_UP_TECHNIQUE() -> Py { ensure_init(); G_USE_UP_TECHNIQUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_UP_TECHNIQUE(v: Py) { ensure_init(); G_USE_UP_TECHNIQUE.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_GOES() -> Py { ensure_init(); G_WHAT_GOES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_GOES(v: Py) { ensure_init(); G_WHAT_GOES.with(|c| *c.borrow_mut() = v); }
pub fn g_WITHOUT_WORD() -> Py { ensure_init(); G_WITHOUT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITHOUT_WORD(v: Py) { ensure_init(); G_WITHOUT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_WITH_ASK() -> Py { ensure_init(); G_WITH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WITH_ASK(v: Py) { ensure_init(); G_WITH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WORD() -> Py { ensure_init(); G_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); G_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_SHAPE() -> Py { ensure_init(); G__AGE_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_SHAPE(v: Py) { ensure_init(); G__AGE_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_MEAT_BLOCKER() -> Py { ensure_init(); G__NO_MEAT_BLOCKER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_MEAT_BLOCKER(v: Py) { ensure_init(); G__NO_MEAT_BLOCKER.with(|c| *c.borrow_mut() = v); }
pub fn g__UP_FOOD_SET() -> Py { ensure_init(); G__UP_FOOD_SET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UP_FOOD_SET(v: Py) { ensure_init(); G__UP_FOOD_SET.with(|c| *c.borrow_mut() = v); }
pub fn g__no_have() -> Py { ensure_init(); G__no_have.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__no_have(v: Py) { ensure_init(); G__no_have.with(|c| *c.borrow_mut() = v); }
pub fn g__row() -> Py { ensure_init(); G__row.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__row(v: Py) { ensure_init(); G__row.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    s__no_have(py_call(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("maxsize".to_string(), Py::None)]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("functools", "lru_cache", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &[py_func("_no_have", |args: &[Py]| _no_have__wrapped(args))]));
    let _ = py_str("Meal ideas: what to cook tonight, and what to make from what is already in the kitchen.\n\nThe answers lane owns this domain. bot/knowledge/cooking.py holds techniques and timings\n(how long to roast a chicken, why rice goes sticky); this file holds dishes, keyed by the\ningredients they use and how long they take. One row therefore answers every way of asking\nfor it: \"what should I cook tonight\", \"anything quick with eggs\", \"I have spinach and\npotatoes, what can I make\", \"something without meat\".\n\nThe reply is composed rather than canned: the picker reads the ingredients the line names\n(memory[\"understood\"]), drops dishes that clash with a limit the person set (no meat, no\ncheese, vegan, a time limit), prefers the dish that uses most of what they have, and does\nnot offer the dish it just offered. So a new ingredient or a new dish is one row of data,\nnot a new rule.\n");
    s_KIDS_IDEA(crate::knowledge::dishes::g_KIDS_IDEA());
    s_BRING_NOT_FOOD(crate::knowledge::occasions::g_BRING_NOT_FOOD());
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    s_SAID_AS(py_dict(vec![(py_str("dhal"), py_str("a lentil dhal")), (py_str("chilli"), py_str("a chilli"))]));
    register_class_method("Dish", "__init__", py_func("Dish.__init__", |args: &[Py]| Dish____init__(args)));
    register_class_method("Dish", "__repr__", py_func("Dish.__repr__", |args: &[Py]| Dish____repr__(args)));
    s_FAVOURITES(py_tuple(vec![py_str("a frittata"), py_str("tomato pasta"), py_str("chickpea curry"), py_str("dhal"), py_str("a bean stew"), py_str("a chicken traybake"), py_str("macaroni cheese"), py_str("a sausage traybake"), py_str("tuna pasta"), py_str("vegetable soup"), py_str("a potato curry"), py_str("a salmon traybake")]));
    s_SHOW_OFF(py_tuple(vec![py_str("a salmon traybake"), py_str("a chicken traybake"), py_str("mushroom risotto"), py_str("aubergine parmigiana"), py_str("a sausage and mash"), py_str("a cod traybake"), py_str("chilli"), py_str("spaghetti bolognese"), py_str("a halloumi traybake"), py_str("a turkey traybake"), py_str("a squash and chickpea traybake"), py_str("chicken curry")]));
    s_DISHES(py_tuple(vec![Dish(&[py_str("a frittata"), py_tuple(vec![py_str("egg"), py_str("spinach")]), Py::Int(20), py_str("vegetarian"), py_str("beat six eggs, stir in whatever vegetables you have (spinach wilted first if it is raw), cook it in an oiled ovenproof pan for 5 minutes and finish it under the grill for 8")]), Dish(&[py_str("shakshuka"), py_tuple(vec![py_str("egg"), py_str("tomato"), py_str("onion")]), Py::Int(25), py_str("vegetarian"), py_str("soften an onion with a tin of tomatoes and a pinch of cumin for 10 minutes, crack the eggs into the sauce, cover and cook 5 minutes more")]), Dish(&[py_str("egg fried rice"), py_tuple(vec![py_str("egg"), py_str("rice"), py_str("pea")]), Py::Int(15), py_str("vegetarian"), py_str("fry cold cooked rice in hot oil for 5 minutes, push it to one side, scramble two eggs in the gap, then stir it all together with soy sauce and peas")]), Dish(&[py_str("an omelette"), py_tuple(vec![py_str("egg"), py_str("cheese")]), Py::Int(10), py_str("vegetarian"), py_str("whisk two or three eggs, cook them gently in butter for 2 minutes, add grated cheese and fold it over")]), Dish(&[py_str("a Spanish tortilla"), py_tuple(vec![py_str("egg"), py_str("potato"), py_str("onion")]), Py::Int(30), py_str("vegetarian"), py_str("fry sliced potato and onion softly for 15 minutes, mix them into five beaten eggs and set it in the pan for another 10")]), Dish(&[py_str("carbonara"), py_tuple(vec![py_str("pasta"), py_str("egg"), py_str("bacon"), py_str("cheese")]), Py::Int(20), py_str("meat"), py_str("fry the bacon, cook the pasta, then toss it off the heat with beaten eggs, grated cheese and plenty of black pepper")]), Dish(&[py_str("tomato pasta"), py_tuple(vec![py_str("pasta"), py_str("tomato"), py_str("garlic")]), Py::Int(20), py_str("vegan"), py_str("fry sliced garlic in olive oil for a minute, add a tin of tomatoes and simmer 10 minutes while the pasta cooks")]), Dish(&[py_str("tuna pasta"), py_tuple(vec![py_str("pasta"), py_str("tuna")]), Py::Int(15), py_str("fish"), py_str("stir a tin of tuna through hot pasta with olive oil, lemon and capers if you have them")]), Dish(&[py_str("pesto pasta"), py_tuple(vec![py_str("pasta"), py_str("pesto")]), Py::Int(12), py_str("vegetarian"), py_str("stir pesto through hot pasta with a spoonful of the cooking water to loosen it")]), Dish(&[py_str("macaroni cheese"), py_tuple(vec![py_str("pasta"), py_str("cheese"), py_str("milk")]), Py::Int(25), py_str("vegetarian"), py_str("make a quick white sauce from butter, flour and milk, melt the cheese into it and stir it through cooked pasta")]), Dish(&[py_str("courgette pasta"), py_tuple(vec![py_str("pasta"), py_str("courgette"), py_str("garlic")]), Py::Int(20), py_str("vegan"), py_str("fry thin courgette slices with garlic for 8 minutes and toss them with pasta, lemon and black pepper")]), Dish(&[py_str("broccoli pasta"), py_tuple(vec![py_str("pasta"), py_str("broccoli"), py_str("garlic")]), Py::Int(20), py_str("vegetarian"), py_str("boil the broccoli with the pasta for the last 5 minutes, then toss it all with garlic fried in oil and a lot of pepper")]), Dish(&[py_str("mushroom risotto"), py_tuple(vec![py_str("rice"), py_str("mushroom"), py_str("onion")]), Py::Int(30), py_str("vegetarian"), py_str("soften the onion, add the rice and mushrooms, then stir in hot stock a ladle at a time for 20 minutes")]), Dish(&[py_str("mushrooms on toast"), py_tuple(vec![py_str("mushroom"), py_str("bread"), py_str("garlic")]), Py::Int(15), py_str("vegetarian"), py_str("fry the mushrooms hard in butter so they colour, add garlic at the end and pile them onto toast")]), Dish(&[py_str("chickpea curry"), py_tuple(vec![py_str("chickpea"), py_str("tomato"), py_str("onion")]), Py::Int(25), py_str("vegan"), py_str("fry an onion with curry powder, add a tin of chickpeas and a tin of tomatoes and simmer for 15 minutes")]), Dish(&[py_str("dhal"), py_tuple(vec![py_str("lentil"), py_str("onion"), py_str("garlic")]), Py::Int(30), py_str("vegan"), py_str("simmer red lentils with onion, garlic and cumin in plenty of water for 25 minutes, until soft, then salt it")]), Dish(&[py_str("a bean stew"), py_tuple(vec![py_str("bean"), py_str("tomato"), py_str("onion")]), Py::Int(25), py_str("vegan"), py_str("fry the onion, add a tin of beans, a tin of tomatoes and paprika and simmer 15 minutes")]), Dish(&[py_str("chilli"), py_tuple(vec![py_str("mince"), py_str("bean"), py_str("tomato"), py_str("onion")]), Py::Int(40), py_str("meat"), py_str("brown the mince, add onion, a tin of beans and a tin of tomatoes with chilli and cumin, and simmer it 25 minutes")]), Dish(&[py_str("a veggie chilli"), py_tuple(vec![py_str("bean"), py_str("pepper"), py_str("tomato"), py_str("onion")]), Py::Int(25), py_str("vegan"), py_str("fry a chopped pepper and onion, add beans, tomatoes and chilli powder and simmer 20 minutes; rice or a baked potato alongside")]), Dish(&[py_str("a sausage traybake"), py_tuple(vec![py_str("sausage"), py_str("potato"), py_str("onion")]), Py::Int(45), py_str("meat"), py_str("toss sausages, halved potatoes and onion wedges with oil on a tray at 200C (180C fan) for 40 minutes, turning once")]), Dish(&[py_str("a chicken traybake"), py_tuple(vec![py_str("chicken"), py_str("potato"), py_str("lemon")]), Py::Int(40), py_str("meat"), py_str("toss chicken thighs, potato chunks and lemon slices with oil at 200C (180C fan) for 40 minutes, skin side up")]), Dish(&[py_str("a chicken stir-fry"), py_tuple(vec![py_str("chicken"), py_str("pepper"), py_str("noodle")]), Py::Int(20), py_str("meat"), py_str("slice the chicken thin, fry it hot for 5 minutes with sliced peppers, then add cooked noodles and soy sauce")]), Dish(&[py_str("chicken curry"), py_tuple(vec![py_str("chicken"), py_str("onion"), py_str("tomato")]), Py::Int(35), py_str("meat"), py_str("brown the chicken, add onion and curry paste, then a tin of tomatoes, and simmer 20 minutes")]), Dish(&[py_str("a salmon traybake"), py_tuple(vec![py_str("salmon"), py_str("potato"), py_str("broccoli")]), Py::Int(30), py_str("fish"), py_str("roast the potatoes at 200C for 20 minutes, then add the salmon and broccoli with oil and lemon and give it 12 minutes more")]), Dish(&[py_str("a salmon rice bowl"), py_tuple(vec![py_str("salmon"), py_str("rice"), py_str("cucumber")]), Py::Int(25), py_str("fish"), py_str("pan-fry the salmon 4 minutes a side and flake it over rice with sliced cucumber and soy")]), Dish(&[py_str("a smoked haddock fish pie"), py_tuple(vec![py_str("smoked haddock"), py_str("potato"), py_str("milk")]), Py::Int(40), py_str("fish"), py_str("poach the haddock in milk for 5 minutes, flake it into a dish with that milk and a spoon of flour, top it with mashed potato and bake at 200C for 25 minutes")]), Dish(&[py_str("a tuna salad"), py_tuple(vec![py_str("tuna"), py_str("tomato"), py_str("cucumber")]), Py::Int(10), py_str("fish"), py_str("drain a tin of tuna into a bowl with chopped tomato and cucumber, dress it with oil, lemon and salt, and add a handful of leaves or cold pasta if you want it filling")]), Dish(&[py_str("a chickpea salad"), py_tuple(vec![py_str("chickpea"), py_str("tomato"), py_str("cucumber"), py_str("lemon")]), Py::Int(10), py_str("vegan"), py_str("toss a drained tin of chickpeas with chopped tomato and cucumber, dress it with lemon juice, olive oil and salt, and crumble cheese over it if you eat it")]), Dish(&[py_str("a chicken salad"), py_tuple(vec![py_str("chicken"), py_str("lettuce"), py_str("tomato")]), Py::Int(15), py_str("meat"), py_str("fry or griddle a chicken breast 5 minutes a side, rest it 5 minutes, then slice it over leaves and tomato with oil and lemon")]), Dish(&[py_str("a prawn stir-fry"), py_tuple(vec![py_str("prawn"), py_str("noodle"), py_str("pepper")]), Py::Int(15), py_str("fish"), py_str("fry the prawns 2 minutes, add peppers, cooked noodles and soy sauce and toss until hot")]), Dish(&[py_str("a tofu stir-fry"), py_tuple(vec![py_str("tofu"), py_str("pepper"), py_str("rice")]), Py::Int(20), py_str("vegan"), py_str("pat the tofu dry, fry it until golden, add peppers, soy and ginger, and serve it on rice")]), Dish(&[py_str("a baked potato with beans"), py_tuple(vec![py_str("potato"), py_str("bean"), py_str("cheese")]), Py::Int(25), py_str("vegetarian"), py_str("microwave the potato 8 minutes, rub it with oil and salt and crisp it in a hot oven for 15, then top it with hot beans and cheese")]), Dish(&[py_str("a potato curry"), py_tuple(vec![py_str("potato"), py_str("onion"), py_str("tomato")]), Py::Int(30), py_str("vegan"), py_str("fry onion with spices, add cubed potato and tomatoes with a splash of water and simmer 20 minutes")]), Dish(&[py_str("a bacon and egg hash"), py_tuple(vec![py_str("potato"), py_str("bacon"), py_str("egg")]), Py::Int(25), py_str("meat"), py_str("fry cubed cooked potato until crisp, add chopped bacon, then crack an egg in and cover the pan for 3 minutes")]), Dish(&[py_str("a cheese toastie"), py_tuple(vec![py_str("bread"), py_str("cheese"), py_str("butter")]), Py::Int(10), py_str("vegetarian"), py_str("butter the outside, fill it with cheese and cook it in a dry pan for 3 minutes a side with something heavy on top")]), Dish(&[py_str("beans on toast"), py_tuple(vec![py_str("bread"), py_str("bean")]), Py::Int(8), py_str("vegan"), py_str("warm the beans in a pan with a little oil and plenty of pepper and pour them over thick toast")]), Dish(&[py_str("a quesadilla"), py_tuple(vec![py_str("tortilla"), py_str("cheese"), py_str("bean")]), Py::Int(10), py_str("vegetarian"), py_str("fill a tortilla with cheese and beans, fold it and dry-fry it 2 minutes a side")]), Dish(&[py_str("vegetable soup"), py_tuple(vec![py_str("carrot"), py_str("onion"), py_str("potato")]), Py::Int(30), py_str("vegan"), py_str("fry the onion and carrot 5 minutes, add potato and enough stock to cover, simmer 20 minutes and blend it")]), Dish(&[py_str("a leek and potato soup"), py_tuple(vec![py_str("leek"), py_str("potato"), py_str("onion")]), Py::Int(30), py_str("vegetarian"), py_str("sweat sliced leeks and onion in butter for 8 minutes without colouring them, add cubed potato and stock to cover, simmer 20 minutes and mash it roughly")]), Dish(&[py_str("lentil soup"), py_tuple(vec![py_str("lentil"), py_str("carrot"), py_str("onion")]), Py::Int(35), py_str("vegan"), py_str("fry onion and carrot, add lentils and stock and simmer for 25 minutes")]), Dish(&[py_str("pea and mint soup"), py_tuple(vec![py_str("pea"), py_str("onion")]), Py::Int(15), py_str("vegan"), py_str("simmer frozen peas and a chopped onion in stock for 10 minutes with a handful of mint, then blend it")]), Dish(&[py_str("spinach and chickpea curry"), py_tuple(vec![py_str("spinach"), py_str("chickpea"), py_str("onion"), py_str("tomato")]), Py::Int(25), py_str("vegan"), py_str("fry an onion with curry powder, add a tin of chickpeas and a tin of tomatoes and simmer 15 minutes, then stir the spinach in at the end until it wilts")]), Dish(&[py_str("a halloumi traybake"), py_tuple(vec![py_str("halloumi"), py_str("potato"), py_str("pepper"), py_str("onion")]), Py::Int(35), py_str("vegetarian"), py_str("toss cubed potato and sliced pepper in oil and roast 20 minutes at 200C (180C fan), then add thick slices of halloumi for the last 12 minutes and squeeze lemon over it")]), Dish(&[py_str("cabbage with bacon"), py_tuple(vec![py_str("cabbage"), py_str("bacon")]), Py::Int(20), py_str("meat"), py_str("fry chopped bacon until crisp, add shredded cabbage and a splash of water and cook it 8 minutes")]), Dish(&[py_str("spaghetti bolognese"), py_tuple(vec![py_str("mince"), py_str("tomato"), py_str("onion"), py_str("pasta")]), Py::Int(30), py_str("meat"), py_str("brown the mince with a chopped onion, add a tin of tomatoes and a stock cube and simmer it 20 minutes while the spaghetti cooks")]), Dish(&[py_str("a sausage and mash"), py_tuple(vec![py_str("sausage"), py_str("potato"), py_str("onion")]), Py::Int(30), py_str("meat"), py_str("boil the potatoes 15 minutes, fry or roast the sausages, then mash the potatoes with butter and make a quick onion gravy in the pan")]), Dish(&[py_str("a chicken and rice"), py_tuple(vec![py_str("chicken"), py_str("rice"), py_str("onion")]), Py::Int(30), py_str("meat"), py_str("fry strips of chicken with an onion, add a tin of tomatoes or a spoon of curry paste, then stir in cooked rice for the last 2 minutes")]), Dish(&[py_str("roast veg pasta"), py_tuple(vec![py_str("pasta"), py_str("veg"), py_str("carrot"), py_str("pepper"), py_str("onion")]), Py::Int(30), py_str("vegan"), py_str("roast chopped veg at 200C (180C fan) for 25 minutes with oil and salt and stir it through cooked pasta")]), Dish(&[py_str("a bacon pasta"), py_tuple(vec![py_str("pasta"), py_str("bacon"), py_str("tomato"), py_str("onion")]), Py::Int(20), py_str("meat"), py_str("fry the bacon with an onion in a dry pan, add a tin of tomatoes, and toss it through pasta with a splash of the cooking water")]), Dish(&[py_str("mince and cabbage"), py_tuple(vec![py_str("mince"), py_str("cabbage"), py_str("onion")]), Py::Int(20), py_str("meat"), py_str("brown the mince with a chopped onion, add shredded cabbage and a splash of water and cook it 8 minutes with a lid on")]), Dish(&[py_str("a cheesy potato hash"), py_tuple(vec![py_str("potato"), py_str("egg"), py_str("cheese"), py_str("onion")]), Py::Int(25), py_str("vegetarian"), py_str("boil the diced potato 12 minutes, fry it with an onion until it colours, then crack eggs over it and scatter cheese on top")]), Dish(&[py_str("a cod traybake"), py_tuple(vec![py_str("white fish"), py_str("potato"), py_str("tomato")]), Py::Int(35), py_str("fish"), py_str("roast potato chunks and cherry tomatoes at 200C (180C fan) for 20 minutes, then lay the cod on top with oil and lemon and give it 12 minutes more")]), Dish(&[py_str("a turkey traybake"), py_tuple(vec![py_str("turkey"), py_str("potato"), py_str("onion")]), Py::Int(40), py_str("meat"), py_str("toss turkey pieces, potato chunks and onion wedges with oil and paprika at 200C (180C fan) for 35 minutes")]), Dish(&[py_str("a sweet potato curry"), py_tuple(vec![py_str("sweet potato"), py_str("chickpea"), py_str("tomato"), py_str("onion")]), Py::Int(30), py_str("vegan"), py_str("fry onion with curry powder, add cubed sweet potato and a tin of tomatoes, simmer 20 minutes and stir in a tin of chickpeas")]), Dish(&[py_str("aubergine parmigiana"), py_tuple(vec![py_str("aubergine"), py_str("tomato"), py_str("cheese")]), Py::Int(45), py_str("vegetarian"), py_str("fry sliced aubergine in plenty of oil until soft, layer it in a dish with tomato sauce and grated cheese, and bake at 190C (170C fan) for 25 minutes")]), Dish(&[py_str("a squash and chickpea traybake"), py_tuple(vec![py_str("squash"), py_str("chickpea"), py_str("onion")]), Py::Int(35), py_str("vegan"), py_str("toss squash chunks, onion wedges and cumin with oil at 200C (180C fan) for 25 minutes, then stir in a tin of chickpeas and a squeeze of lemon")]), Dish(&[py_str("a ham and potato chowder"), py_tuple(vec![py_str("bacon"), py_str("potato"), py_str("onion"), py_str("milk")]), Py::Int(30), py_str("meat"), py_str("fry chopped ham and onion 5 minutes, add diced potato and stock to cover, simmer 15 minutes and stir in a splash of milk")]), Dish(&[py_str("a baked feta pasta"), py_tuple(vec![py_str("feta"), py_str("pasta"), py_str("tomato")]), Py::Int(25), py_str("vegetarian"), py_str("put a block of feta in a dish with halved cherry tomatoes, oil, oregano and a pinch of chilli, bake at 200C (180C fan) for 20 minutes, then mash it into the tomatoes and stir through cooked pasta")]), Dish(&[py_str("a fish foil bake"), py_tuple(vec![py_str("salmon"), py_str("lemon"), py_str("potato")]), Py::Int(25), py_str("fish"), py_str("wrap the fish in foil with lemon and butter and bake at 200C (180C fan) for 12 to 15 minutes, with boiled potatoes on the side")])]));
    s_INGREDIENTS(py_dict(vec![(py_str("egg"), py_tuple(vec![py_str("egg"), py_str("eggs")])), (py_str("cheese"), py_tuple(vec![py_str("cheese"), py_str("cheddar"), py_str("mozzarella"), py_str("parmesan")])), (py_str("feta"), py_tuple(vec![py_str("feta"), py_str("feta cheese")])), (py_str("milk"), py_tuple(vec![py_str("milk"), py_str("cream")])), (py_str("coconut milk"), py_tuple(vec![py_str("coconut milk"), py_str("tinned coconut milk"), py_str("coconut cream")])), (py_str("oat milk"), py_tuple(vec![py_str("oat milk")])), (py_str("soy milk"), py_tuple(vec![py_str("soy milk"), py_str("soya milk")])), (py_str("almond milk"), py_tuple(vec![py_str("almond milk")])), (py_str("rice milk"), py_tuple(vec![py_str("rice milk")])), (py_str("plant milk"), py_tuple(vec![py_str("plant milk"), py_str("plant milks")])), (py_str("condensed milk"), py_tuple(vec![py_str("condensed milk")])), (py_str("evaporated milk"), py_tuple(vec![py_str("evaporated milk")])), (py_str("butter bean"), py_tuple(vec![py_str("butter beans"), py_str("butter bean")])), (py_str("butter"), py_tuple(vec![py_str("butter")])), (py_str("pasta"), py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("macaroni")])), (py_str("noodle"), py_tuple(vec![py_str("noodle"), py_str("noodles"), py_str("ramen")])), (py_str("rice noodle"), py_tuple(vec![py_str("rice noodles"), py_str("rice noodle")])), (py_str("rice"), py_tuple(vec![py_str("rice"), py_str("risotto")])), (py_str("bread"), py_tuple(vec![py_str("bread"), py_str("toast"), py_str("baguette"), py_str("rolls")])), (py_str("tortilla"), py_tuple(vec![py_str("tortilla"), py_str("tortillas"), py_str("wrap"), py_str("wraps")])), (py_str("potato"), py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("spud"), py_str("spuds")])), (py_str("sweet potato"), py_tuple(vec![py_str("sweet potato"), py_str("sweet potatoes")])), (py_str("squash"), py_tuple(vec![py_str("butternut"), py_str("butternut squash"), py_str("squash"), py_str("pumpkin")])), (py_str("tomato"), py_tuple(vec![py_str("tomato"), py_str("tomatoes"), py_str("toms"), py_str("passata"), py_str("tinned tomatoes"), py_str("chopped tomatoes")])), (py_str("onion"), py_tuple(vec![py_str("onion"), py_str("onions"), py_str("shallot"), py_str("shallots")])), (py_str("leek"), py_tuple(vec![py_str("leek"), py_str("leeks")])), (py_str("garlic"), py_tuple(vec![py_str("garlic")])), (py_str("pepper"), py_tuple(vec![py_str("pepper"), py_str("peppers"), py_str("capsicum")])), (py_str("carrot"), py_tuple(vec![py_str("carrot"), py_str("carrots")])), (py_str("courgette"), py_tuple(vec![py_str("courgette"), py_str("courgettes"), py_str("zucchini")])), (py_str("aubergine"), py_tuple(vec![py_str("aubergine"), py_str("eggplant")])), (py_str("broccoli"), py_tuple(vec![py_str("broccoli")])), (py_str("cabbage"), py_tuple(vec![py_str("cabbage")])), (py_str("pea"), py_tuple(vec![py_str("pea"), py_str("peas")])), (py_str("bean"), py_tuple(vec![py_str("bean"), py_str("beans"), py_str("baked beans"), py_str("kidney beans"), py_str("black beans")])), (py_str("chickpea"), py_tuple(vec![py_str("chickpea"), py_str("chickpeas"), py_str("hummus")])), (py_str("lentil"), py_tuple(vec![py_str("lentil"), py_str("lentils"), py_str("dhal"), py_str("dal")])), (py_str("spinach"), py_tuple(vec![py_str("spinach"), py_str("kale"), py_str("chard")])), (py_str("mushroom"), py_tuple(vec![py_str("mushroom"), py_str("mushrooms")])), (py_str("chicken"), py_tuple(vec![py_str("chicken")])), (py_str("sausage"), py_tuple(vec![py_str("sausage"), py_str("sausages")])), (py_str("mince"), py_tuple(vec![py_str("mince"), py_str("minced beef"), py_str("ground beef"), py_str("beef")])), (py_str("bacon"), py_tuple(vec![py_str("bacon"), py_str("ham"), py_str("pancetta"), py_str("chorizo")])), (py_str("tuna"), py_tuple(vec![py_str("tuna")])), (py_str("salmon"), py_tuple(vec![py_str("salmon")])), (py_str("white fish"), py_tuple(vec![py_str("cod"), py_str("haddock"), py_str("pollock"), py_str("white fish"), py_str("fish fillets"), py_str("fish")])), (py_str("smoked haddock"), py_tuple(vec![py_str("smoked haddock"), py_str("smoked fish")])), (py_str("turkey"), py_tuple(vec![py_str("turkey")])), (py_str("prawn"), py_tuple(vec![py_str("prawn"), py_str("prawns"), py_str("shrimp")])), (py_str("tofu"), py_tuple(vec![py_str("tofu")])), (py_str("quorn"), py_tuple(vec![py_str("quorn"), py_str("quorn mince"), py_str("quorn pieces"), py_str("quorn fillets"), py_str("mycoprotein")])), (py_str("lemon"), py_tuple(vec![py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes")])), (py_str("pesto"), py_tuple(vec![py_str("pesto")])), (py_str("maple syrup"), py_tuple(vec![py_str("maple syrup")])), (py_str("chocolate"), py_tuple(vec![py_str("chocolate"), py_str("dark chocolate"), py_str("milk chocolate"), py_str("cooking chocolate"), py_str("chocolate chips")])), (py_str("bagel"), py_tuple(vec![py_str("bagel"), py_str("bagels")])), (py_str("cucumber"), py_tuple(vec![py_str("cucumber")])), (py_str("lettuce"), py_tuple(vec![py_str("lettuce"), py_str("salad leaves"), py_str("leaves"), py_str("rocket")])), (py_str("spring greens"), py_tuple(vec![py_str("spring greens"), py_str("spring green"), py_str("collard greens"), py_str("collard"), py_str("greens")])), (py_str("halloumi"), py_tuple(vec![py_str("halloumi"), py_str("paneer")])), (py_str("veg"), py_tuple(vec![py_str("veg"), py_str("vegetables"), py_str("vegetable"), py_str("frozen veg"), py_str("frozen vegetables"), py_str("mixed veg"), py_str("mixed vegetables"), py_str("frozen mixed veg")]))]));
    s_IN_A_DISH(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut _dish: Py = __item.clone(); for __item in py_iter_or_raise(&gattr(&_dish.clone(), "uses")) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_FISH_INGREDIENTS(call_builtin("frozenset", &[py_tuple(vec![py_str("white fish"), py_str("salmon"), py_str("tuna"), py_str("prawn")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9]+(?:'[a-z]+)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH_STYLE(py_tuple(vec![py_str("curry"), py_str("soup"), py_str("stew"), py_str("stir fry"), py_str("stir-fry"), py_str("traybake"), py_str("pie"), py_str("salad"), py_str("roast"), py_str("risotto"), py_str("noodles"), py_str("casserole"), py_str("bake")]));
    s_STYLE_WANT(py_tuple(vec![py_str("fancy"), py_str("mood for"), py_str("feel like"), py_str("up for"), py_str("craving"), py_str("i want a"), py_str("i want some")]));
    s_STYLE_CUES(py_dict(vec![(py_str("spicy"), py_tuple(vec![py_str("curry"), py_str("chilli"), py_str("stir fry")])), (py_str("hot"), py_tuple(vec![py_str("curry"), py_str("chilli"), py_str("stir fry")])), (py_str("creamy"), py_tuple(vec![py_str("risotto"), py_str("pasta"), py_str("curry")])), (py_str("cosy"), py_tuple(vec![py_str("soup"), py_str("stew"), py_str("casserole")])), (py_str("cosy night"), py_tuple(vec![py_str("soup"), py_str("stew")])), (py_str("comfort"), py_tuple(vec![py_str("soup"), py_str("stew"), py_str("pie"), py_str("macaroni")])), (py_str("warming"), py_tuple(vec![py_str("soup"), py_str("stew"), py_str("casserole")])), (py_str("fresh"), py_tuple(vec![py_str("salad"), py_str("stir fry")])), (py_str("light"), py_tuple(vec![py_str("salad"), py_str("soup")]))]));
    s_CARRY_ASK(py_tuple(vec![py_str("picnic"), py_str("picnics"), py_str("beach"), py_str("day out"), py_str("day trip"), py_str("road trip"), py_str("train journey"), py_str("long journey"), py_str("flight"), py_str("cool box"), py_str("coolbox"), py_str("cooler box"), py_str("flask")]));
    s_CARRY_REPLIES(py_tuple(vec![py_str("Things that travel well: a frittata or a quiche cut in wedges, cold sausages or sausage rolls, cheese and oatcakes, cherry tomatoes, and fruit that survives a bag. Pack anything wet in its own box and take a bag for the rubbish."), py_str("Cold and easy to carry: a frittata cut in wedges, sausage rolls, cheese with oatcakes or bread, tomatoes and a bag of grapes. Freeze a bottle of water the night before - it keeps the food cold and then it is your drink.")]));
    s_CHEAP_ASK(py_tuple(vec![py_str("cheaper"), py_str("cheap"), py_str("cheapest"), py_str("budget"), py_str("afford"), py_str("save money"), py_str("less money"), py_str("tight month"), py_str("low on money"), py_str("money is tight"), py_str("skint"), py_str("payday"), py_str("hard up"), py_str("no money"), py_str("short of money"), py_str("on a tight budget"), py_str("student"), py_str("students"), py_str("uni"), py_str("university"), py_str("fresher"), py_str("freshers"), py_str("halls"), py_str("shared kitchen"), py_str("one pan")]));
    s_LIGHT_ASK(py_tuple(vec![py_str("healthy"), py_str("healthier"), py_str("healthily"), py_str("light"), py_str("lighter"), py_str("slimming"), py_str("low calorie"), py_str("low fat"), py_str("nutritious"), py_str("fresh"), py_str("clean eating"), py_str("balanced"), py_str("not too heavy"), py_str("something light")]));
    s_LIGHT_SET(py_tuple(vec![py_str("a salmon traybake"), py_str("a salmon rice bowl"), py_str("a tofu stir-fry"), py_str("a chickpea salad"), py_str("a chicken salad"), py_str("vegetable soup"), py_str("a leek and potato soup"), py_str("lentil soup"), py_str("pea and mint soup"), py_str("spinach and chickpea curry"), py_str("a chicken stir-fry"), py_str("a prawn stir-fry"), py_str("a fish foil bake"), py_str("a cod traybake"), py_str("roast veg pasta"), py_str("a squash and chickpea traybake"), py_str("a bean stew"), py_str("a chicken and rice")]));
    s_SMALL_PEOPLE(py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("baby"), py_str("babies"), py_str("kid"), py_str("kids"), py_str("children"), py_str("child"), py_str("nursery"), py_str("grandchild"), py_str("grandchildren"), py_str("easter"), py_str("halloween"), py_str("christmas"), py_str("birthday"), py_str("bonfire night")]));
    s_CHILD_AT_TABLE(py_tuple(vec![py_str("toddler"), py_str("toddlers"), py_str("baby"), py_str("babies"), py_str("kid"), py_str("kids"), py_str("children"), py_str("child"), py_str("little ones"), py_str("little one"), py_str("grandchild"), py_str("grandchildren"), py_str("family"), py_str("families"), py_str("my son"), py_str("my daughter")]));
    s_EVENT_WORDS(py_tuple(vec![py_str("race"), py_str("match"), py_str("game"), py_str("training"), py_str("workout"), py_str("gym"), py_str("run"), py_str("10k"), py_str("5k"), py_str("marathon"), py_str("football"), py_str("rugby"), py_str("cricket"), py_str("netball"), py_str("swim"), py_str("competition"), py_str("tournament"), py_str("triathlon"), py_str("fight"), py_str("climb")]));
    s_BEFORE_EVENT(py_tuple(vec![py_str("before a"), py_str("before the"), py_str("before my"), py_str("before i"), py_str("morning of"), py_str("day of"), py_str("pre workout"), py_str("pre-workout"), py_str("fuel for"), py_str("fuelling"), py_str("fueling")]));
    s_MEAL_WORD(py_tuple(vec![py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("meals"), py_str("cook"), py_str("cooking"), py_str("eat"), py_str("eating"), py_str("food"), py_str("hungry"), py_str("starving"), py_str("bake"), py_str("baking")]));
    s_CHEAP_SET(py_tuple(vec![py_str("egg"), py_str("potato"), py_str("bean"), py_str("chickpea"), py_str("lentil"), py_str("pasta"), py_str("rice"), py_str("onion"), py_str("tomato"), py_str("cheese"), py_str("cabbage"), py_str("carrot"), py_str("bread"), py_str("pea"), py_str("spinach"), py_str("cucumber"), py_str("lemon"), py_str("tofu"), py_str("noodle"), py_str("mushroom"), py_str("courgette"), py_str("aubergine"), py_str("squash"), py_str("pepper"), py_str("broccoli")]));
    s_CHEAP(call_builtin("set", &[g_CHEAP_SET()]).unwrap_or_else(|e| exc::raise(e)));
    s_FOOD_WORDS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_INGREDIENTS(), "values", &[])) { let mut forms: Py = __item.clone(); for __item in py_iter_or_raise(&forms.clone()) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    let __aug1 = op_add(&g_FOOD_WORDS(), &py_tuple(vec![py_str("meal"), py_str("meals"), py_str("dinner"), py_str("supper"), py_str("teatime"), py_str("lunch"), py_str("food"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("eat"), py_str("eat something"), py_str("hungry"), py_str("kitchen"), py_str("fridge"), py_str("cupboard"), py_str("starving"), py_str("starved"), py_str("famished"), py_str("something quick"), py_str("something fast"), py_str("something easy"), py_str("something simple"), py_str("something filling"), py_str("something hot")]));
    s_FOOD_WORDS(__aug1);
    let __aug2 = op_add(&g_FOOD_WORDS(), &op_add(&call_builtin("tuple", &[g_DISH_STYLE()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[g_STYLE_CUES()]).unwrap_or_else(|e| exc::raise(e))));
    s_FOOD_WORDS(__aug2);
    s_NO_MEAT(py_tuple(vec![py_str("vegetarian"), py_str("veggie"), py_str("vegan"), py_str("meatless"), py_str("no meat"), py_str("without meat"), py_str("less meat"), py_str("cut out meat"), py_str("cut down on meat"), py_str("no meat or fish"), py_str("meat free"), py_str("meat-free")]));
    s_NO_CHEESE(py_tuple(vec![py_str("dairy"), py_str("lactose"), py_str("no cheese"), py_str("without cheese"), py_str("cheeseless")]));
    let __aug3 = op_add(&g_FOOD_WORDS(), &op_add(&call_builtin("tuple", &[g_NO_MEAT()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("tuple", &[g_NO_CHEESE()]).unwrap_or_else(|e| exc::raise(e))));
    s_FOOD_WORDS(__aug3);
    s_NO_CUE(py_tuple(vec![py_str("no"), py_str("without"), py_str("arent"), py_str("isnt"), py_str("dont want"), py_str("dont eat"), py_str("dont like"), py_str("doesnt eat"), py_str("doesnt like"), py_str("sick of"), py_str("bored of"), py_str("tired of"), py_str("had enough"), py_str("not in the mood for"), py_str("cant eat"), py_str("cant have"), py_str("allergic to"), py_str("instead of"), py_str("hate"), py_str("hates"), py_str("hated"), py_str("dislike"), py_str("dislikes"), py_str("disliked"), py_str("loathe"), py_str("cant stand"), py_str("can't stand"), py_str("not a fan"), py_str("not a fan of"), py_str("not fussed"), py_str("not keen"), py_str("not keen on"), py_str("fed up of"), py_str("fed up with"), py_str("had it with"), py_str("wont eat"), py_str("won't eat"), py_str("will not eat"), py_str("wont touch"), py_str("won't touch"), py_str("out of"), py_str("ran out of"), py_str("run out of"), py_str("none left"), py_str("no more"), py_str("dont have"), py_str("do not have"), py_str("havent got"), py_str("have not got"), py_str("got no"), py_str("short of")]));
    s_NO_GAP(py_tuple(vec![py_str(""), py_str(" any"), py_str(" the"), py_str(" a"), py_str(" an"), py_str(" some"), py_str(" more"), py_str(" any more"), py_str(" of the"), py_str(" of any"), py_str(" any of the"), py_str(" of"), py_str(" on")]));
    s_GAME_WORDS(py_tuple(vec![py_str("game"), py_str("games"), py_str("board game"), py_str("board games"), py_str("boardgame"), py_str("boardgames"), py_str("puzzle"), py_str("puzzles"), py_str("jigsaw"), py_str("jigsaws"), py_str("chess"), py_str("cards"), py_str("quiz")]));
    s_TECHNIQUE(py_tuple(vec![py_str("how"), py_str("long"), py_str("safe"), py_str("time"), py_str("temperature"), py_str("temp"), py_str("defrost"), py_str("thaw"), py_str("store"), py_str("keep"), py_str("best way"), py_str("way to cook"), py_str("way to make"), py_str("how to"), py_str("reheat"), py_str("burning"), py_str("burnt"), py_str("smell"), py_str("rubbery"), py_str("sticky"), py_str("soggy"), py_str("lumpy"), py_str("raw"), py_str("pink"), py_str("undercooked"), py_str("ruined"), py_str("freeze"), py_str("why")]));
    s_USE_UP_TECHNIQUE(op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TECHNIQUE()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("how")))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &py_tuple(vec![py_str("stale")])));
    s_USE_UP(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("meat"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("ham"), py_str("joint")]), py_str("Shred it into fried rice, or fold it through a white sauce with pasta and bake it 20 minutes at 200C; cooked meat only needs heating through")]), py_tuple(vec![py_tuple(vec![py_str("potato"), py_str("potatoes")]), py_str("Fry cold chopped potatoes with an onion until crisp, or fold them into a frittata and finish it under the grill; both take about 20 minutes")]), py_tuple(vec![py_tuple(vec![py_str("bread"), py_str("loaf")]), py_str("Tear it into a salad with tomatoes, oil and salt, or blitz it into breadcrumbs and freeze them for topping a bake")]), py_tuple(vec![py_tuple(vec![py_str("cheese")]), py_str("Grate it into a macaroni cheese, or make a toastie with it and a little mustard")])]));
    s_UP_GREENS(py_tuple(vec![py_str("spinach"), py_str("cabbage"), py_str("kale"), py_str("chard"), py_str("greens"), py_str("collard"), py_str("collards"), py_str("savoy"), py_str("sprout"), py_str("sprouts"), py_str("broccoli"), py_str("broccolini"), py_str("cauliflower"), py_str("romanesco"), py_str("kalettes"), py_str("spring green"), py_str("spring greens"), py_str("lettuce"), py_str("rocket"), py_str("cress"), py_str("watercress"), py_str("pea"), py_str("peas"), py_str("green bean"), py_str("green beans"), py_str("mangetout"), py_str("asparagus")]));
    s_UP_ALLIUMS(py_tuple(vec![py_str("onion"), py_str("onions"), py_str("leek"), py_str("leeks"), py_str("shallot"), py_str("shallots"), py_str("garlic"), py_str("spring onion"), py_str("spring onions"), py_str("red onion"), py_str("red onions")]));
    s_UP_ROOTS(py_tuple(vec![py_str("beetroot"), py_str("parsnip"), py_str("parsnips"), py_str("carrot"), py_str("carrots"), py_str("turnip"), py_str("turnips"), py_str("swede"), py_str("celeriac"), py_str("squash"), py_str("pumpkin"), py_str("sweet potato"), py_str("sweet potatoes"), py_str("potato"), py_str("potatoes"), py_str("artichoke"), py_str("artichokes"), py_str("fennel"), py_str("jerusalem artichoke"), py_str("jerusalem artichokes")]));
    s_UP_SOFT(py_tuple(vec![py_str("courgette"), py_str("courgettes"), py_str("zucchini"), py_str("aubergine"), py_str("aubergines"), py_str("pepper"), py_str("peppers"), py_str("mushroom"), py_str("mushrooms"), py_str("tomato"), py_str("tomatoes"), py_str("corn"), py_str("sweetcorn"), py_str("butternut"), py_str("okra"), py_str("bean"), py_str("beans")]));
    s_UP_GRAINS(py_tuple(vec![py_str("rice"), py_str("quinoa"), py_str("couscous"), py_str("bulgur"), py_str("barley"), py_str("pasta"), py_str("noodle"), py_str("noodles"), py_str("spaghetti"), py_str("macaroni"), py_str("lentil"), py_str("lentils"), py_str("chickpea"), py_str("chickpeas"), py_str("porridge"), py_str("oat"), py_str("oats")]));
    s_UP_FRUIT(py_tuple(vec![py_str("cranberry"), py_str("cranberries"), py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("banana"), py_str("bananas"), py_str("berry"), py_str("berries"), py_str("raspberry"), py_str("raspberries"), py_str("strawberry"), py_str("strawberries"), py_str("rhubarb"), py_str("plum"), py_str("plums"), py_str("peach"), py_str("peaches"), py_str("nectarine"), py_str("nectarines"), py_str("mango"), py_str("pineapple"), py_str("orange"), py_str("oranges"), py_str("grapes"), py_str("cherry"), py_str("cherries"), py_str("apricot"), py_str("apricots"), py_str("fig"), py_str("figs"), py_str("melon")]));
    s_UP_MEAT(py_tuple(vec![py_str("chicken"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("ham"), py_str("bacon"), py_str("sausage"), py_str("sausages"), py_str("turkey"), py_str("duck"), py_str("mince"), py_str("steak"), py_str("liver"), py_str("chorizo"), py_str("gammon"), py_str("meat"), py_str("meatballs")]));
    s_UP_FISH(py_tuple(vec![py_str("salmon"), py_str("cod"), py_str("haddock"), py_str("tuna"), py_str("prawn"), py_str("prawns"), py_str("mackerel"), py_str("trout"), py_str("fish"), py_str("fishcake"), py_str("fishcakes"), py_str("sardine"), py_str("sardines"), py_str("crab"), py_str("anchovy"), py_str("anchovies")]));
    s_UP_CLASSES(py_tuple(vec![py_tuple(vec![g_UP_GREENS(), py_str("butter or oil in a hot pan for 3 or 4 minutes is enough, or stir through a soup, an omelette or a pasta sauce just before you serve. Shred raw into a salad with oil and lemon if you would rather have it cold.")]), py_tuple(vec![g_UP_ALLIUMS(), py_str("cook slowly in butter until sweet, which takes 15 minutes and is worth it, then stir through a soup, a gravy or a tray of roasted veg. On a pizza, a tart or a piece of toast it is a topping on its own.")]), py_tuple(vec![g_UP_ROOTS(), py_str("cut into cubes and fry in a little oil until the edges crisp, or mash with potato for a bubble and squeak, or simmer into a soup. Cold in a salad with a sharp dressing is just as good.")]), py_tuple(vec![g_UP_SOFT(), py_str("fry in a hot pan with garlic and oil until the water is gone, stir into a pasta or a sauce, or bake under cheese.")]), py_tuple(vec![g_UP_GRAINS(), py_str("fry hard with an onion and whatever veg you have, loosen into a soup or a stew, or fold into a salad with oil and lemon. Heat through until piping hot, and bin it if it has sat out.")]), py_tuple(vec![g_UP_FRUIT(), py_str("simmer 10 minutes with a spoon of sugar and a splash of water for a compote, fold into a crumble, or spoon over yoghurt or porridge.")]), py_tuple(vec![g_UP_MEAT(), py_str("shred or slice into a sandwich, a salad, a fried rice or a curry, and add at the end so it only heats through. Covered in the fridge it keeps 3 days.")]), py_tuple(vec![g_UP_FISH(), py_str("flake cold through pasta or a salad with lemon and yoghurt, or mix into a fishcake, and don't reheat it hard or it dries out.")])]));
    s_UP_FOODS(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_UP_GREENS(), &g_UP_ALLIUMS()), &g_UP_ROOTS()), &g_UP_SOFT()), &g_UP_GRAINS()), &g_UP_FRUIT()), &g_UP_MEAT()), &g_UP_FISH()));
    s__UP_FOOD_SET(call_builtin("frozenset", &[g_UP_FOODS()]).unwrap_or_else(|e| exc::raise(e)));
    s_USE_DISH_WORDS(py_tuple(vec![py_str("soup"), py_str("broth"), py_str("stew"), py_str("curry"), py_str("casserole"), py_str("salad"), py_str("pasta"), py_str("risotto"), py_str("traybake"), py_str("pie"), py_str("gratin"), py_str("mash"), py_str("remoulade"), py_str("pancake"), py_str("fritter"), py_str("dhal"), py_str("dal"), py_str("sauce"), py_str("compote"), py_str("crumble"), py_str("hummus"), py_str("dip"), py_str("fajita"), py_str("cake"), py_str("bread"), py_str("toast"), py_str("sandwich"), py_str("omelette"), py_str("frittata"), py_str("pizza"), py_str("burger"), py_str("pilaf"), py_str("snack"), py_str("crisp"), py_str("chip"), py_str("relish"), py_str("chutney"), py_str("slaw")]));
    s_QUICK(py_tuple(vec![py_str("quick"), py_str("quicker"), py_str("quickly"), py_str("fast"), py_str("faster")]));
    s_BEDTIME(py_tuple(vec![py_str("off to bed"), py_str("to bed soon"), py_str("in bed soon"), py_str("early night"), py_str("bed early"), py_str("knackered"), py_str("shattered"), py_str("exhausted"), py_str("up early"), py_str("early start")]));
    s_HUNGRY(py_tuple(vec![py_str("starving"), py_str("starved"), py_str("famished"), py_str("hungry"), py_str("hank marvin"), py_str("need to eat"), py_str("need something to eat"), py_str("something to eat"), py_str("skip lunch"), py_str("skipped lunch"), py_str("havent eaten"), py_str("have not eaten")]));
    s_IMPRESS(py_tuple(vec![py_str("nice"), py_str("fancy"), py_str("special"), py_str("impress"), py_str("impressive"), py_str("anniversary"), py_str("birthday"), py_str("date night"), py_str("romantic"), py_str("treat"), py_str("trying to cook something")]));
    s_IMPRESS_ASK(py_tuple(vec![py_str("impress"), py_str("impressing"), py_str("to impress"), py_str("make an effort"), py_str("pull out all the stops"), py_str("special occasion"), py_str("something special"), py_str("trying to cook something")]));
    s_IMPRESS_FOR(py_tuple(vec![py_str("for my girlfriend"), py_str("for my boyfriend"), py_str("for my partner"), py_str("for my wife"), py_str("for my husband"), py_str("for my mum"), py_str("for my mom"), py_str("for my dad"), py_str("for my in laws"), py_str("for my in-laws"), py_str("for her"), py_str("for him")]));
    s_IMPRESS_WORTH(py_tuple(vec![py_str("nice"), py_str("fancy"), py_str("special"), py_str("impressive"), py_str("impress"), py_str("treat"), py_str("birthday"), py_str("anniversary"), py_str("proper"), py_str("best"), py_str("good")]));
    s_NOT_A_MEAL(py_tuple(vec![py_str("exercise"), py_str("workout"), py_str("workouts"), py_str("run"), py_str("runs"), py_str("walk"), py_str("meditate"), py_str("shower"), py_str("study"), py_str("clean"), py_str("sleep"), py_str("nap"), py_str("train"), py_str("commute"), py_str("boil"), py_str("bake")]));
    s_ANOTHER_MEAL(py_tuple(vec![py_str("breakfast"), py_str("brunch"), py_str("snack"), py_str("snacks"), py_str("lunch"), py_str("luncheon"), py_str("pudding"), py_str("dessert"), py_str("afters"), py_str("afternoon tea"), py_str("high tea"), py_str("elevenses")]));
    s_NOT_FOR_ME(py_tuple(vec![py_str("allergy"), py_str("allergic"), py_str("allergies"), py_str("nut"), py_str("nuts"), py_str("peanut"), py_str("peanuts"), py_str("intolerance"), py_str("intolerant"), py_str("coeliac"), py_str("celiac"), py_str("temp"), py_str("temperature")]));
    s_HEALTH(py_tuple(vec![py_str("pox"), py_str("fever"), py_str("flu"), py_str("vomiting"), py_str("diarrhoea"), py_str("diarrhea"), py_str("migraine"), py_str("nausea"), py_str("nauseous"), py_str("throwing up"), py_str("sore throat"), py_str("cant keep anything down"), py_str("sick bug")]));
    s_NO_DAIRY(py_tuple(vec![py_str("no dairy"), py_str("without dairy"), py_str("dairy free"), py_str("dairy-free"), py_str("lactose"), py_str("no milk"), py_str("without milk"), py_str("milk free"), py_str("milk-free"), py_str("cant have dairy"), py_str("cannot have dairy")]));
    s_UNWELL(py_tuple(vec![py_str("stomach"), py_str("tummy"), py_str("dodgy stomach"), py_str("bad stomach"), py_str("upset stomach"), py_str("sick"), py_str("sickness"), py_str("being sick"), py_str("feel sick"), py_str("feeling sick"), py_str("throwing up"), py_str("nausea"), py_str("nauseous"), py_str("diarrhoea"), py_str("diarrhea"), py_str("constipated"), py_str("constipation"), py_str("food poisoning"), py_str("have a cold"), py_str("got a cold"), py_str("a bad cold"), py_str("chest infection"), py_str("tooth"), py_str("teeth"), py_str("dentist"), py_str("wisdom tooth"), py_str("tooth out"), py_str("toothache"), py_str("root canal"), py_str("filling"), py_str("gums"), py_str("sore mouth"), py_str("mouth ulcer"), py_str("tonsils"), py_str("out of date"), py_str("use by"), py_str("past their date"), py_str("past its date"), py_str("past the date"), py_str("gone off"), py_str("off eggs"), py_str("still ok to eat"), py_str("still safe to eat")]));
    s_SICK_OF(py_tuple(vec![py_str("sick of"), py_str("tired of"), py_str("bored of"), py_str("bored with"), py_str("fed up with"), py_str("had enough of"), py_str("sick and tired")]));
    s_CONDITION(py_tuple(vec![py_str("diabetes"), py_str("diabetic"), py_str("type 1"), py_str("type 2"), py_str("blood sugar"), py_str("ibs"), py_str("irritable bowel"), py_str("crohns"), py_str("crohn's"), py_str("colitis"), py_str("gout"), py_str("kidney"), py_str("kidneys"), py_str("cholesterol"), py_str("coeliac"), py_str("celiac"), py_str("anaphyla"), py_str("allergy"), py_str("allergic"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding")]));
    s_EMPTY(py_tuple(vec![py_str("no food"), py_str("nothing in"), py_str("empty fridge"), py_str("empty cupboards"), py_str("bare cupboard"), py_str("nothing at home"), py_str("no shopping"), py_str("run out of food")]));
    s_NOT_AN_IDEA(py_tuple(vec![py_str("leftover"), py_str("left over"), py_str("leftovers"), py_str("spare"), py_str("use up"), py_str("using up")]));
    s_HOW_TO(py_tuple(vec![py_str("need to"), py_str("do i need"), py_str("is it safe"), py_str("still use"), py_str("can i use"), py_str("wash"), py_str("washing"), py_str("wrong"), py_str("normal"), py_str("matter"), py_str("cheaper"), py_str("worth"), py_str("difference"), py_str("better to"), py_str("how do i"), py_str("why does"), py_str("why do"), py_str("is that ok"), py_str("is that ok"), py_str("is it ok"), py_str("from frozen"), py_str("still frozen"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing")]));
    s_SOUP_NAMED(py_tuple(vec![py_str("soup"), py_str("soups"), py_str("broth"), py_str("broths")]));
    s_OWN_MEAL(py_tuple(vec![py_str("im cooking"), py_str("i'm cooking"), py_str("i am cooking"), py_str("cooking a"), py_str("cooking some"), py_str("already cooking"), py_str("on the hob"), py_str("in the oven"), py_str("making a"), py_str("im making"), py_str("i'm making"), py_str("just made")]));
    s_OTHER_PEOPLE(py_tuple(vec![py_str("only eat"), py_str("only eats"), py_str("only ate"), py_str("wont eat"), py_str("won't eat"), py_str("doesnt eat"), py_str("doesn't eat"), py_str("refuses to eat"), py_str("refuses")]));
    s_BROKEN(py_tuple(vec![py_str("noise"), py_str("noisy"), py_str("rattling"), py_str("broken"), py_str("broke down"), py_str("not working"), py_str("stopped working"), py_str("leaking"), py_str("leak"), py_str("repair"), py_str("repairs"), py_str("warranty"), py_str("engineer"), py_str("fuse"), py_str("socket"), py_str("smells"), py_str("smelly"), py_str("won't close"), py_str("wont close")]));
    s_WHAT_GOES(py_tuple(vec![py_str("goes with"), py_str("go with"), py_str("serve with"), py_str("goes nicely")]));
    s_NOT_MY_KITCHEN(py_tuple(vec![py_str("cat"), py_str("cats"), py_str("dog"), py_str("dogs"), py_str("kitten"), py_str("puppy"), py_str("pets"), py_str("hamster"), py_str("rabbit"), py_str("called"), py_str("named"), py_str("nickname")]));
    s_MEASURING(py_tuple(vec![py_str("scales"), py_str("weigh"), py_str("weighing"), py_str("weighed"), py_str("convert"), py_str("converting"), py_str("conversion"), py_str("grammes"), py_str("ounces"), py_str("tablespoon"), py_str("tablespoons"), py_str("teaspoon"), py_str("teaspoons"), py_str("ml"), py_str("litres"), py_str("millilitres")]));
    s_SLEEP(py_tuple(vec![py_str("bed"), py_str("before bed"), py_str("bedtime"), py_str("sleep"), py_str("asleep"), py_str("awake"), py_str("waking"), py_str("insomnia")]));
    s_DRINK(op_add(&py_tuple(vec![py_str("tea"), py_str("brewing"), py_str("brewed"), py_str("tea bag"), py_str("tea bags"), py_str("kettle"), py_str("mug"), py_str("drinking"), py_str("drinks")]), &g_SLEEP()));
    s_AILING_PLANT(py_tuple(vec![py_str("blight"), py_str("mildew"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("rot"), py_str("rots"), py_str("rotting"), py_str("rotted"), py_str("rust"), py_str("wilt"), py_str("wilting"), py_str("yellowing"), py_str("yellow leaves"), py_str("yellowed"), py_str("drooping"), py_str("dying"), py_str("disease"), py_str("diseased"), py_str("infected"), py_str("infection"), py_str("aphids"), py_str("blackfly"), py_str("whitefly"), py_str("slugs"), py_str("snails"), py_str("caterpillars"), py_str("not growing"), py_str("stopped growing"), py_str("flowering"), py_str("fruiting"), py_str("prune"), py_str("pruning"), py_str("compost"), py_str("allotment"), py_str("greenhouse"), py_str("polytunnel"), py_str("raised bed"), py_str("sowed"), py_str("sowing"), py_str("planted"), py_str("planting"), py_str("growing"), py_str("grow"), py_str("harvest"), py_str("crop"), py_str("crops")]));
    s_NOT_A_DISH(op_add(&g_NOT_A_MEAL(), &py_tuple(vec![py_str("packed lunch"), py_str("packed lunches"), py_str("pack lunch"), py_str("packing lunch"), py_str("lunchbox"), py_str("lunchboxes"), py_str("lunch box"), py_str("lunch boxes"), py_str("school lunch"), py_str("bento"), py_str("picnic"), py_str("buffet"), py_str("canape"), py_str("finger food"), py_str("party food")])));
    s_REPAIR(py_tuple(vec![py_str("too much salt"), py_str("much salt"), py_str("much spice"), py_str("much chilli"), py_str("much pepper"), py_str("much sugar"), py_str("too salty"), py_str("over salted"), py_str("oversalted"), py_str("too spicy"), py_str("too hot to eat"), py_str("too thin"), py_str("too thick"), py_str("too sweet"), py_str("too sour"), py_str("burnt"), py_str("burning"), py_str("caught on the"), py_str("sticking to the pan"), py_str("stuck to the pan"), py_str("soggy"), py_str("curdled"), py_str("split"), py_str("went wrong"), py_str("messed up"), py_str("ruined"), py_str("dried out"), py_str("overcooked"), py_str("undercooked"), py_str("not cooked"), py_str("still raw"), py_str("raw in the middle"), py_str("tastes off"), py_str("smells off"), py_str("went off")]));
    s_MONEY_MATTERS(py_tuple(vec![py_str("refund"), py_str("refunds"), py_str("refunded"), py_str("money back"), py_str("get my money"), py_str("reimbursed"), py_str("reimbursement"), py_str("compensation"), py_str("compensated"), py_str("claim"), py_str("claimed"), py_str("claiming"), py_str("charge"), py_str("charges"), py_str("charged"), py_str("fee"), py_str("fees"), py_str("voucher"), py_str("insurance"), py_str("rebook"), py_str("rebooking"), py_str("no show"), py_str("no-show"), py_str("missed"), py_str("delay"), py_str("delayed"), py_str("cancelled"), py_str("canceled"), py_str("cancellation"), py_str("didnt take"), py_str("didn't take"), py_str("did not take"), py_str("didnt go"), py_str("couldnt go"), py_str("could not go")]));
    s_SAYING(py_tuple(vec![py_str("what do i say"), py_str("what should i say"), py_str("what can i say"), py_str("what shall i say"), py_str("what do i tell"), py_str("what should i tell"), py_str("what can i tell"), py_str("what shall i tell"), py_str("what to say"), py_str("what to tell"), py_str("how do i say"), py_str("how should i say"), py_str("what do i reply"), py_str("what should i reply"), py_str("what to reply")]));
    s_NOT_MINE(call_builtin("tuple", &[mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_A_DISH(), &g_NOT_AN_IDEA()), &g_OWN_MEAL()), &g_OTHER_PEOPLE()), &g_HOW_TO()), &g_WHAT_GOES()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY()), &g_MEASURING()), &g_REPAIR()), &g_UNWELL()), &g_SAYING())])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MINE_ENTRY(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_MINE()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("buffet"), py_str("canape"), py_str("canapes"), py_str("finger food"), py_str("party food"), py_str("how do i"), py_str("how to")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MINE_OUT(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_MINE()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("run"), py_str("runs"), py_str("out of"), py_str("ran out of")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MINE_ASK(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_MINE()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("im cooking"), py_str("i'm cooking"), py_str("i am cooking"), py_str("cooking a"), py_str("cooking some"), py_str("already cooking"), py_str("cheaper"), py_str("worth"), py_str("afford")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_MINE_CHEAP(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_MINE_ASK()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("cheaper")]), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENERS(py_tuple(vec![py_str("Try"), py_str("I'd do"), py_str("Go for"), py_str("How about"), py_str("Go with")]));
    s_TIMES(py_tuple(vec![py_str("About {n} minutes."), py_str("Roughly {n} minutes in all."), py_str("{n} minutes, give or take.")]));
    s_TOTAL_TIMES(py_tuple(vec![py_str("Roughly {n} minutes in all."), py_str("About {n} minutes in all."), py_str("{n} minutes from start to finish."), py_str("Roughly {n} minutes, start to finish.")]));
    s_COOK_TIME(stdlib_call("re", "compile", &[py_str("\\d+\\s*(?:to\\s*\\d+\\s*)?(?:min|minute|hour|hr)")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOLLOW_ON(py_tuple(vec![py_str("If you'd rather have something else, {alt} takes {n} minutes."), py_str("Something else that works is {alt}, at {n} minutes.")]));
    s_FOLLOW_ON_NO_CHEESE(py_str("Without the cheese, there is {alt}, at {n} minutes."));
    s_NOT_A_MEAL_SUBJECT(py_tuple(vec![py_str("what to wear"), py_str("wear to"), py_str("to wear"), py_str("dress code"), py_str("outfit"), py_str("dinner roll"), py_str("dinner lady"), py_str("dinner jacket"), py_str("what to bring"), py_str("what should i bring"), py_str("bring to a"), py_str("take to a dinner party")]));
    s_DESSERT_ASK(py_tuple(vec![py_str("dessert"), py_str("desserts"), py_str("pudding"), py_str("puddings"), py_str("afters"), py_str("something sweet"), py_str("sweet thing"), py_str("sweet treat"), py_str("ice cream")]));
    s_MAKING_AT_HOME(py_tuple(vec![py_str("make my own"), py_str("make our own"), py_str("making my own"), py_str("making our own"), py_str("made my own"), py_str("made our own"), py_str("make it myself"), py_str("make them myself"), py_str("make it ourselves"), py_str("make them ourselves"), py_str("make your own"), py_str("making your own"), py_str("from scratch")]));
    s_TECHNIQUE_ASK(py_tuple(vec![py_str("how long"), py_str("how do i"), py_str("how do you"), py_str("how to"), py_str("how many minutes"), py_str("what temperature"), py_str("what temp"), py_str("how much water"), py_str("how much rice"), py_str("how much pasta"), py_str("should i boil"), py_str("do i need to soak")]));
    s_USE_UP_ASK(py_tuple(vec![py_str("use up"), py_str("using up"), py_str("used up"), py_str("eat up"), py_str("eating up")]));
    s_NO_HAVE(py_tuple(vec![py_str("dont have"), py_str("don't have"), py_str("didnt have"), py_str("didn't have"), py_str("havent got"), py_str("haven't got"), py_str("havent any"), py_str("haven't any"), py_str("have no"), py_str("got no"), py_str("there is no"), py_str("theres no"), py_str("there's no")]));
    s_NO_HAVE_GAP(py_str("(?: (?:any|a|an|the|some|more|of|that|this)){0,2}"));
    s_DOUBLE_NO(py_tuple(vec![py_str("dont dislike"), py_str("do not dislike"), py_str("doesnt dislike"), py_str("does not dislike"), py_str("dont hate"), py_str("do not hate"), py_str("doesnt hate"), py_str("does not hate"), py_str("dont mind"), py_str("do not mind"), py_str("doesnt mind"), py_str("does not mind")]));
    s__NO_MEAT_BLOCKER(py_tuple(vec![py_str("time"), py_str("times"), py_str("rush"), py_str("money"), py_str("energy"), py_str("idea"), py_str("clue"), py_str("chance"), py_str("reason"), py_str("need"), py_str("way"), py_str("point"), py_str("room"), py_str("doubt"), py_str("harm"), py_str("but")]));
    s_LUNCH_ASK(py_tuple(vec![py_str("lunch"), py_str("lunchbox"), py_str("lunchboxes"), py_str("lunch box"), py_str("lunch boxes"), py_str("packed lunch"), py_str("school lunch"), py_str("lunchtime"), py_str("lunch time"), py_str("brunch")]));
    s_IDEA_ASK(py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("suggestion"), py_str("recommend"), py_str("options"), py_str("option"), py_str("recipes"), py_str("inspiration")]));
    s_GRACE(Py::Int(10));
    s_HARD_CLOCK(py_tuple(vec![py_str("under"), py_str("only"), py_str("max"), py_str("maximum"), py_str("less"), py_str("tops"), py_str("just")]));
    s_STAPLE(call_builtin("frozenset", &[mcall(&py_str("\nonion garlic butter milk egg cheese pasta rice bread tomato potato bean pea carrot lemon flour\noil stock herb spice pepper salt noodle tortilla butter\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH_SHAPES(py_tuple(vec![py_str("soup"), py_str("stew"), py_str("curry"), py_str("pasta"), py_str("traybake"), py_str("salad"), py_str("stir-fry"), py_str("risotto"), py_str("hash"), py_str("bake"), py_str("pie"), py_str("toast"), py_str("sandwich"), py_str("omelette"), py_str("tortilla"), py_str("frittata"), py_str("potato"), py_str("rice"), py_str("noodles"), py_str("bolognese"), py_str("chowder"), py_str("parmigiana"), py_str("mash")]));
    s_UNCOUNTABLE(call_builtin("frozenset", &[mcall(&py_str("\nmince beef pork lamb chicken turkey bacon ham fish salmon tuna rice pasta bread cheese milk\nbutter spinach broccoli cabbage kale leek garlic onion sugar flour yoghurt courgette aubergine\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DAIRY_EXTRAS(py_tuple(vec![py_str(", and crumble cheese over it if you eat it")]));
    s_ASK_WORDS(py_tuple(vec![py_str("what"), py_str("whats"), py_str("any"), py_str("dinner"), py_str("dinners"), py_str("tea"), py_str("supper"), py_str("supper"), py_str("lunch"), py_str("lunches"), py_str("breakfast"), py_str("meal"), py_str("meals"), py_str("food"), py_str("hungry"), py_str("starving"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("suggestion"), py_str("recommend"), py_str("recommendation"), py_str("something"), py_str("anything"), py_str("tonight"), py_str("recipe"), py_str("recipes"), py_str("leftover"), py_str("leftovers"), py_str("use up"), py_str("eat up"), py_str("cook up"), py_str("fridge"), py_str("freezer"), py_str("cupboard"), py_str("pantry"), py_str("hungry"), py_str("inspiration"), py_str("options")]));
    s_DIET_SAID(py_tuple(vec![py_tuple(vec![py_str("dairy free"), py_str("dairy free")]), py_tuple(vec![py_str("lactose"), py_str("lactose intolerant")]), py_tuple(vec![py_str("vegan"), py_str("vegan")]), py_tuple(vec![py_str("vegetarian"), py_str("vegetarian")]), py_tuple(vec![py_str("gluten free"), py_str("gluten free")]), py_tuple(vec![py_str("no meat"), py_str("not eating meat")]), py_tuple(vec![py_str("no fish"), py_str("not eating fish")]), py_tuple(vec![py_str("no dairy"), py_str("dairy free")]), py_tuple(vec![py_str("cutting out"), py_str("cutting things out")])]));
    s_ACK_WORDS(py_tuple(vec![py_str("thanks"), py_str("thank you"), py_str("thx"), py_str("cheers"), py_str("nice one"), py_str("will do"), py_str("ill try that"), py_str("i'll try that"), py_str("sounds good"), py_str("good idea"), py_str("great idea"), py_str("lovely"), py_str("perfect")]));
    s_DIET_WORDS(py_tuple(vec![py_str("vegetarian"), py_str("vegan"), py_str("veggie"), py_str("dairy free"), py_str("lactose"), py_str("gluten free"), py_str("no meat"), py_str("meat free"), py_str("meatless"), py_str("pescatarian"), py_str("pescetarian"), py_str("allergic"), py_str("allergy"), py_str("nut free"), py_str("nut allergy"), py_str("halal"), py_str("kosher")]));
    s_ASK_CUES(py_tuple(vec![py_str("what"), py_str("which"), py_str("anything"), py_str("any"), py_str("need"), py_str("want"), py_str("feed"), py_str("party"), py_str("guests"), py_str("everyone"), py_str("ideas"), py_str("idea"), py_str("suggest"), py_str("recommend"), py_str("decide"), py_str("menu"), py_str("plan"), py_str("help me")]));
    s_ASK_IN_NEWS(py_tuple(vec![py_str("what to cook"), py_str("what to make"), py_str("what to have"), py_str("what to eat"), py_str("what should i"), py_str("what can i"), py_str("ideas for"), py_str("idea for"), py_str("help me"), py_str("need something"), py_str("need a"), py_str("trying to work out"), py_str("trying to decide"), py_str("trying to plan"), py_str("cant decide"), py_str("can't decide"), py_str("any suggestions"), py_str("suggest something")]));
    s_GLUTEN(py_tuple(vec![py_str("coeliac"), py_str("celiac"), py_str("gluten"), py_str("wheat free"), py_str("no wheat"), py_str("no gluten")]));
    s_CROWD_SAID(py_dict(vec![(py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30))]));
    s_CROWD_ASK(py_tuple(vec![py_str("party"), py_str("buffet"), py_str("birthday"), py_str("wedding"), py_str("christening"), py_str("guests"), py_str("everyone")]));
    s_COUNT_WORDS(py_dict(vec![(Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]));
    s__AGE_SHAPE(stdlib_call("re", "compile", &[py_str("\\b(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)(?:\\s*(?:,|and|&)\\s*(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))?\\s*(?:years?|yrs?)\\s*(?:-\\s*)?olds?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_KITCHEN_WORDS(call_builtin("set", &[py_list(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("our"), py_str("some"), py_str("any"), py_str("half"), py_str("leftover"), py_str("left"), py_str("over"), py_str("frozen"), py_str("fresh"), py_str("tin"), py_str("tins"), py_str("can"), py_str("cans"), py_str("jar"), py_str("jars"), py_str("packet"), py_str("packets"), py_str("pack"), py_str("packs"), py_str("bag"), py_str("bags"), py_str("bunch"), py_str("few"), py_str("bit"), py_str("bits"), py_str("little"), py_str("of"), py_str("and"), py_str("or"), py_str("in"), py_str("on"), py_str("fridge"), py_str("freezer"), py_str("cupboard"), py_str("what"), py_str("can"), py_str("could"), py_str("i"), py_str("we"), py_str("you"), py_str("make"), py_str("made"), py_str("cook"), py_str("cooking"), py_str("do"), py_str("done"), py_str("with"), py_str("from"), py_str("use"), py_str("up"), py_str("using"), py_str("recipe"), py_str("recipes"), py_str("ideas"), py_str("idea"), py_str("please"), py_str("tonight"), py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("meals"), py_str("food"), py_str("eat"), py_str("have"), py_str("got"), py_str("there"), py_str("is"), py_str("are"), py_str("only"), py_str("just"), py_str("lot"), py_str("lots"), py_str("loads"), py_str("plenty"), py_str("still"), py_str("also"), py_str("for"), py_str("to"), py_str("it"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("one"), py_str("two"), py_str("three"), py_str("four"), py_str("five"), py_str("kg"), py_str("g"), py_str("grams"), py_str("oz"), py_str("lb"), py_str("big"), py_str("small"), py_str("old"), py_str("new"), py_str("cooked"), py_str("raw"), py_str("tinned"), py_str("dried"), py_str("spare"), py_str("extra"), py_str("again"), py_str("rest"), py_str("leftovers")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WITH_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:with|using|from)\\s+(.{2,60})")]).unwrap_or_else(|e| exc::raise(e)));
    s_RECIPE_FOR(stdlib_call("re", "compile", &[py_str("\\b(?:recipe|recipes)\\b(?:\\s+(?:for|of|to make))?\\s+(.{2,60})")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_DISH_NAME(py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("meal"), py_str("meals"), py_str("dinner"), py_str("dinners"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("tonight"), py_str("please"), py_str("pls"), py_str("plz"), py_str("quick"), py_str("easy"), py_str("cheap")]));
    s_COMPANY(call_builtin("frozenset", &[py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("toddler"), py_str("toddlers"), py_str("baby"), py_str("babies"), py_str("teen"), py_str("teens"), py_str("teenager"), py_str("teenagers"), py_str("family"), py_str("families"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("mates"), py_str("flatmate"), py_str("flatmates"), py_str("housemate"), py_str("housemates"), py_str("partner"), py_str("partners"), py_str("husband"), py_str("wife"), py_str("boyfriend"), py_str("girlfriend"), py_str("mum"), py_str("mums"), py_str("dad"), py_str("dads"), py_str("mother"), py_str("mothers"), py_str("father"), py_str("fathers"), py_str("parent"), py_str("parents"), py_str("sister"), py_str("sisters"), py_str("brother"), py_str("brothers"), py_str("gran"), py_str("grans"), py_str("grandma"), py_str("grandmas"), py_str("grandad"), py_str("grandads"), py_str("nan"), py_str("nans"), py_str("grandparent"), py_str("grandparents"), py_str("grandchild"), py_str("grandchildren"), py_str("nephew"), py_str("nephews"), py_str("niece"), py_str("nieces"), py_str("cousin"), py_str("cousins"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("in law"), py_str("in laws"), py_str("in-laws"), py_str("inlaw"), py_str("inlaws"), py_str("law"), py_str("laws"), py_str("guest"), py_str("guests"), py_str("people"), py_str("one"), py_str("ones"), py_str("everyone"), py_str("somebody"), py_str("someone"), py_str("workmate"), py_str("workmates"), py_str("colleague"), py_str("colleagues"), py_str("crowd"), py_str("household"), py_str("neighbours"), py_str("neighbors"), py_str("visitors"), py_str("company"), py_str("others"), py_str("them"), py_str("us"), py_str("we"), py_str("me"), py_str("you"), py_str("ourselves"), py_str("myself"), py_str("yourself"), py_str("anyone"), py_str("nobody")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_AN_INGREDIENT(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("your"), py_str("his"), py_str("her"), py_str("its"), py_str("our"), py_str("their"), py_str("mine"), py_str("yours"), py_str("hers"), py_str("ours"), py_str("theirs"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("be"), py_str("been"), py_str("being"), py_str("am"), py_str("do"), py_str("does"), py_str("did"), py_str("done"), py_str("should"), py_str("shall"), py_str("would"), py_str("could"), py_str("can"), py_str("will"), py_str("year"), py_str("years"), py_str("old"), py_str("month"), py_str("months"), py_str("week"), py_str("weeks"), py_str("day"), py_str("days"), py_str("tonight"), py_str("today"), py_str("tomorrow"), py_str("it"), py_str("them"), py_str("him"), py_str("we"), py_str("us"), py_str("they"), py_str("these"), py_str("those"), py_str("again"), py_str("still"), py_str("coming"), py_str("coming"), py_str("staying"), py_str("stay"), py_str("stays"), py_str("visiting"), py_str("visit"), py_str("over"), py_str("coming"), py_str("around"), py_str("here"), py_str("there"), py_str("soon"), py_str("later"), py_str("too"), py_str("also"), py_str("just"), py_str("now"), py_str("quite"), py_str("really"), py_str("time"), py_str("times"), py_str("hour"), py_str("hours"), py_str("minute"), py_str("minutes"), py_str("mins"), py_str("moment"), py_str("rush"), py_str("hurry"), py_str("while"), py_str("effort"), py_str("no"), py_str("not"), py_str("any"), py_str("some"), py_str("much"), py_str("many"), py_str("more"), py_str("less"), py_str("enough"), py_str("half"), py_str("little"), py_str("bit"), py_str("proper"), py_str("decent"), py_str("nice"), py_str("simple"), py_str("whole"), py_str("quick"), py_str("quickly"), py_str("fast"), py_str("easy"), py_str("something"), py_str("anything"), py_str("thing"), py_str("things")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_FOOD_LIMITS(py_tuple(vec![py_str("meat"), py_str("dairy"), py_str("gluten"), py_str("wheat"), py_str("lactose"), py_str("sugar"), py_str("salt"), py_str("fat"), py_str("nuts"), py_str("nut"), py_str("fish"), py_str("seafood"), py_str("shellfish"), py_str("eggs"), py_str("carbs"), py_str("carb"), py_str("calories"), py_str("oil"), py_str("butter"), py_str("flour"), py_str("pasta"), py_str("rice"), py_str("potato"), py_str("potatoes")]));
    s_WITHOUT_WORD(stdlib_call("re", "compile", &[py_str("\\bwithout\\s+(?:the\\s+|any\\s+|a\\s+|much\\s+|so\\s+much\\s+)?(?P<food>[a-z][a-z'-]{1,24})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKEOUT_PICKS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("pizza place"), py_str("pizza shop"), py_str("pizza parlour"), py_str("pizza parlor"), py_str("pizza"), py_str("pizzas"), py_str("dominos"), py_str("domino's")]), py_str("One pizza each and a garlic bread to share: a 12-inch is one hungry person and a 9-inch a light one, so a 12-inch between two plus a side is the cheaper way to feed a pair. A plain margherita and one with a topping beats two fully loaded ones, which are the ones left half eaten.")]), py_tuple(vec![py_tuple(vec![py_str("bakery"), py_str("bakers"), py_str("baker"), py_str("patisserie"), py_str("bakery shop")]), py_str("Buy the loaf that is still warm and one pastry to eat before you get home: a sourdough or a tin loaf keeps two days on the board and a week sliced in the freezer, and a cheese twist or a croissant is what a bakery is for. Ask which loaf is today's rather than yesterday's - it is the one question they are always happy to answer.")]), py_tuple(vec![py_tuple(vec![py_str("sandwich shop"), py_str("deli"), py_str("deli counter"), py_str("subway"), py_str("baguette shop"), py_str("sandwich bar")]), py_str("The filling that is made fresh, in a baguette or a sub rather than sliced bread, and ask for the salad on the side if it has to travel back to a desk: wet salad in the sandwich is what makes it collapse. A hot filling - meatballs, a toastie, a chicken escalope - is the one worth ordering out, because it is the one that will not be as good made at home.")])]));
    s_TAKEOUT_PLACES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TAKEOUT_PICKS()) { let __unpk4786 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk4786[0].clone(); let mut _u: Py = __unpk4786[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_TAKEOUT_ASK(py_tuple(vec![py_str("what"), py_str("whats"), py_str("what is"), py_str("what are"), py_str("nice"), py_str("good"), py_str("best"), py_str("better"), py_str("recommend"), py_str("order"), py_str("get"), py_str("have"), py_str("should i"), py_str("should we"), py_str("do i"), py_str("do we"), py_str("worth"), py_str("idea"), py_str("ideas"), py_str("how many")]));
    s_DEFINITION_SHAPE(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'s| is| are|s)?\\s+(?:a|an|the)\\s+([a-z']+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_ASKING_WORDS(py_set(vec![py_str("good"), py_str("great"), py_str("nice"), py_str("best"), py_str("better"), py_str("cheap"), py_str("cheapest"), py_str("decent"), py_str("safe"), py_str("healthy"), py_str("worth"), py_str("solid"), py_str("vegetarian"), py_str("vegan")]));
    s_SPOILT_FOOD(py_tuple(vec![py_str("mouldy"), py_str("mould"), py_str("moldy"), py_str("mold"), py_str("blue spots"), py_str("green spots"), py_str("white spots"), py_str("black spots"), py_str("spots"), py_str("speckled"), py_str("fluff"), py_str("fluffy"), py_str("fuzzy"), py_str("furry"), py_str("fur"), py_str("green bits"), py_str("white bits"), py_str("blue bits"), py_str("smells"), py_str("smelly"), py_str("smelt"), py_str("stinks"), py_str("gone off"), py_str("went off"), py_str("going off"), py_str("slimy"), py_str("sour"), py_str("rancid"), py_str("stale"), py_str("fizzy"), py_str("curdled"), py_str("separated"), py_str("went bad"), py_str("gone bad"), py_str("rotten"), py_str("off it")]));
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-hurry")); __args.push(py_tuple(vec![py_tuple(vec![py_str("need something"), py_str("something on the table"), py_str("on the table"), py_str("get dinner on"), py_str("dinner on the table"), py_str("feed the kids"), py_str("something fast")]), py_tuple(vec![py_str("minutes"), py_str("minute"), py_str("mins"), py_str("min"), py_str("quick"), py_str("quickly"), py_str("fast"), py_str("hurry"), py_str("now"), py_str("late"), py_str("knackered"), py_str("shattered"), py_str("starving"), py_str("hungry")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&py_tuple(vec![py_str("sleep"), py_str("bed"), py_str("tired out"), py_str("go to bed"), py_str("nap"), py_str("breath"), py_str("chest"), py_str("pain"), py_str("hurt"), py_str("doctor"), py_str("gp"), py_str("hospital"), py_str("panic"), py_str("anxious"), py_str("worried")]), &g_NOT_A_DISH()), &g_HOW_TO()), &g_MEASURING()), &g_DRINK()), &g_NOT_MY_KITCHEN()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-tonight")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tonight"), py_str("supper"), py_str("teatime"), py_str("what should i cook"), py_str("what can i cook"), py_str("what should i make"), py_str("what can i make"), py_str("what can i eat"), py_str("what should i eat"), py_str("what to eat"), py_str("whats for"), py_str("what can i have"), py_str("what should i have"), py_str("what to have"), py_str("what can we have"), py_str("what shall i have"), py_str("what can i make for"), py_str("whats to eat"), py_str("what is for"), py_str("what would you cook"), py_str("what do i eat"), py_str("what do i cook"), py_str("what shall i eat"), py_str("impress"), py_str("impressing"), py_str("to impress"), py_str("in laws"), py_str("in-laws"), py_str("guests"), py_str("guest"), py_str("dinner party"), py_str("special dinner")]), py_tuple(vec![py_str("impress"), py_str("impressing"), py_str("impressive"), py_str("in laws"), py_str("in-laws"), py_str("guests"), py_str("guest"), py_str("special dinner"), py_str("made an effort"), py_str("looks impressive"), py_str("look impressive"), py_str("looks like effort")]), op_add(&g_FOOD_WORDS(), &py_tuple(vec![py_str("impress"), py_str("impressing"), py_str("in laws"), py_str("in-laws"), py_str("guests"), py_str("guest"), py_str("fancy"), py_str("special")]))])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_tuple(vec![py_str("long"), py_str("time"), py_str("temperature"), py_str("oven"), py_str("safe"), py_str("defrost"), py_str("thaw"), py_str("reheat"), py_str("store"), py_str("keep"), py_str("burnt"), py_str("burning"), py_str("smell"), py_str("gone"), py_str("off"), py_str("dairy"), py_str("gluten"), py_str("wheat"), py_str("lactose"), py_str("vegan"), py_str("vegetarian"), py_str("vegetarians"), py_str("allergic"), py_str("allergy"), py_str("coeliac"), py_str("celiac"), py_str("intolerant"), py_str("doesnt eat"), py_str("does not eat"), py_str("can not eat"), py_str("cant eat"), py_str("can not have"), py_str("cant have"), py_str("hardly eats"), py_str("hardly eat"), py_str("barely eats"), py_str("barely eat"), py_str("wont eat"), py_str("won't eat"), py_str("stopped eating"), py_str("not eating"), py_str("isnt eating"), py_str("isn't eating"), py_str("no appetite"), py_str("lost his appetite"), py_str("lost her appetite"), py_str("off her food"), py_str("off his food"), py_str("gone off her food"), py_str("gone off his food"), py_str("not hungry")]), &g_UNWELL()), &g_OWN_MEAL()), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_ANOTHER_MEAL()), &g_SAYING()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-ideas-anything")); __args.push(py_tuple(vec![py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("suggestion"), py_str("suggestions"), py_str("inspiration"), py_str("recommend"), py_str("recommendation"), py_str("options"), py_str("something"), py_str("quick")]), g_FOOD_WORDS()])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_tuple(vec![py_str("book"), py_str("books"), py_str("film"), py_str("films"), py_str("movie"), py_str("podcast"), py_str("album"), py_str("gift"), py_str("present"), py_str("name"), py_str("poison"), py_str("read"), py_str("watch"), py_str("tv"), py_str("show"), py_str("series"), py_str("surname"), py_str("word"), py_str("card"), py_str("cards"), py_str("tub"), py_str("bottle"), py_str("bath"), py_str("yoga"), py_str("tap"), py_str("weather"), py_str("radiator"), py_str("shower")]), &g_OWN_MEAL()), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY())), ("ask".to_string(), py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("recommend"), py_str("option"), py_str("options"), py_str("inspiration"), py_str("for dinner"), py_str("for tea"), py_str("for lunch"), py_str("for supper"), py_str("to cook"), py_str("to make"), py_str("to eat"), py_str("something with"), py_str("something quick"), py_str("fancy"), py_str("hungry"), py_str("starving"), py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("cook"), py_str("make")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-with")); __args.push(py_tuple(vec![g_FOOD_WORDS(), py_tuple(vec![py_str("make"), py_str("cook"), py_str("should i cook"), py_str("should i make"), py_str("can i cook"), py_str("can i make"), py_str("shall i cook"), py_str("shall i make")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_TECHNIQUE(), &g_OWN_MEAL()), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_FOR_ME()), &g_HEALTH()), &g_EMPTY()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_WHAT_GOES()), &g_BROKEN()), &g_SOUP_NAMED()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-haves")); __args.push(py_tuple(vec![op_add(&g_FOOD_WORDS(), &py_tuple(vec![py_str("what else can i do with"), py_str("what else can i make with"), py_str("what else can i cook with"), py_str("what else with"), py_str("something else to do with"), py_str("other ideas for"), py_str("any other ideas")])), py_tuple(vec![py_str("have"), py_str("has"), py_str("got"), py_str("only"), py_str("pack"), py_str("packs"), py_str("packet"), py_str("tin"), py_str("tins"), py_str("bag"), py_str("bags"), py_str("bunch"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("half a"), py_str("few"), py_str("else"), py_str("other")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_OWN_MEAL(), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_FOR_ME()), &g_HEALTH()), &g_EMPTY()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_WHAT_GOES()), &g_BROKEN()), &g_AILING_PLANT()), &g_SPOILT_FOOD()), &g_SOUP_NAMED()), &py_tuple(vec![py_str("how")]))), ("guard".to_string(), g__not_a_mark_and_not_a_row())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-limit")); __args.push(py_tuple(vec![py_tuple(vec![py_str("without"), py_str("no"), py_str("minus"), py_str("skip"), py_str("vegetarian"), py_str("veggie"), py_str("vegan")]), py_tuple(vec![py_str("meat"), py_str("cheese"), py_str("dairy"), py_str("fish"), py_str("pork"), py_str("chicken"), py_str("milk"), py_str("gluten")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&g_HEALTH(), &g_EMPTY()), &g_NOT_MY_KITCHEN()), &g_MEASURING()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-quick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quick"), py_str("fast")]), py_tuple(vec![py_str("minute"), py_str("minutes"), py_str("min"), py_str("mins"), py_str("time")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_OWN_MEAL(), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-clock")); __args.push(py_tuple(vec![py_tuple(vec![py_str("minute"), py_str("minutes"), py_str("min"), py_str("mins"), py_str("hour"), py_str("hours"), py_str("half an hour")]), py_tuple(vec![py_str("dinner"), py_str("supper"), py_str("tea"), py_str("eat"), py_str("eating"), py_str("cook"), py_str("cooking"), py_str("table"), py_str("food"), py_str("meal"), py_str("lunch"), py_str("breakfast"), py_str("hungry")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_OWN_MEAL(), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY()), &py_tuple(vec![py_str("been"), py_str("how"), py_str("long"), py_str("oven"), py_str("boil"), py_str("bake")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-hurry-quick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quick"), py_str("quicker"), py_str("fast"), py_str("hurry"), py_str("hurry up"), py_str("starving")]), py_tuple(vec![py_str("please"), py_str("something"), py_str("idea"), py_str("one"), py_str("dinner"), py_str("food"), py_str("meal")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_OWN_MEAL(), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY()), &py_tuple(vec![py_str("walk"), py_str("chat")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-more")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quicker"), py_str("faster"), py_str("different"), py_str("another"), py_str("other"), py_str("instead"), py_str("else")]), py_tuple(vec![py_str("something"), py_str("idea"), py_str("ideas"), py_str("option"), py_str("options"), py_str("meal"), py_str("dinner"), py_str("dish")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_OWN_MEAL(), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_MEASURING()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY())), ("ask".to_string(), op_add(&g_FOOD_WORDS(), &py_tuple(vec![py_str("dinner"), py_str("tea"), py_str("meal"), py_str("supper"), py_str("food"), py_str("eat"), py_str("tonight"), py_str("cook"), py_str("cooking"), py_str("quick"), py_str("quicker"), py_str("faster"), py_str("hungry")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-use-up")); __args.push(py_tuple(vec![py_tuple(vec![py_str("leftover"), py_str("left over"), py_str("leftovers"), py_str("spare"), py_str("use up"), py_str("using up"), py_str("rest of")]), py_tuple(vec![py_str("turkey"), py_str("meat"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("ham"), py_str("potato"), py_str("potatoes"), py_str("vegetables"), py_str("veg"), py_str("greens"), py_str("cabbage"), py_str("broccoli"), py_str("carrots"), py_str("bread"), py_str("loaf"), py_str("cheese"), py_str("joint")])])); __args.push(g_use_up()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), g_USE_UP_TECHNIQUE())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-tea")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tea"), py_str("teatime"), py_str("tea time")]), py_tuple(vec![py_str("for tea"), py_str("make"), py_str("cook"), py_str("shall"), py_str("should"), py_str("whats"), py_str("what can i"), py_str("ideas"), py_str("idea"), py_str("quick"), py_str("something"), py_str("tonight"), py_str("supper"), py_str("cheap"), py_str("easy"), py_str("hurry"), py_str("starving")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_TECHNIQUE(), &g_OWN_MEAL()), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_NOT_AN_IDEA()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY()), &py_tuple(vec![py_str("brew"), py_str("brewing"), py_str("steep"), py_str("tea bag"), py_str("tea bags"), py_str("green tea"), py_str("black tea"), py_str("cup of tea"), py_str("cup of"), py_str("mug"), py_str("kettle"), py_str("herbal"), py_str("chamomile"), py_str("peppermint"), py_str("drink"), py_str("drinking"), py_str("cold"), py_str("desk"), py_str("caffeine"), py_str("sugar")])), &g_SLEEP()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-diet-again")); __args.push(py_tuple(vec![py_tuple(vec![py_str("something"), py_str("anything"), py_str("and"), py_str("make it"), py_str("make"), py_str("instead")]), py_tuple(vec![py_str("vegetarian"), py_str("vegan"), py_str("meat free"), py_str("meat-free"), py_str("meatless")])])); __args.push(g__later()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_FOR_ME(), &g_HEALTH()), &g_EMPTY()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_WHAT_GOES()), &g_NOT_AN_IDEA()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-cheaper")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cheaper"), py_str("cheap"), py_str("cheapest"), py_str("budget"), py_str("afford"), py_str("save money"), py_str("less money"), py_str("money is tight")]), py_tuple(vec![py_str("something"), py_str("and"), py_str("dinner"), py_str("tonight"), py_str("meal"), py_str("idea"), py_str("ideas"), py_str("make"), py_str("cook"), py_str("what"), py_str("any"), py_str("else")])])); __args.push(g__later()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_MINE_CHEAP(), &g_TECHNIQUE()), &g_HEALTH()), &g_WHAT_GOES()), &g_MEASURING()), &py_tuple(vec![py_str("insure"), py_str("insurance"), py_str("insured"), py_str("car insurance"), py_str("policy"), py_str("premium"), py_str("third party"), py_str("no claims"), py_str("mot")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-out-of")); __args.push(py_tuple(vec![py_tuple(vec![py_str("out of"), py_str("ran out of"), py_str("run out of"), py_str("none left"), py_str("no more")]), g_FOOD_WORDS()])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_MINE_OUT(), &g_TECHNIQUE()), &g_HOW_TO()), &g_HEALTH()), &g_WHAT_GOES()), &g_MEASURING()), &py_tuple(vec![py_str("left out"), py_str("out of the fridge"), py_str("out of the freezer"), py_str("all night"), py_str("overnight"), py_str("two hours"), py_str("safe to eat"), py_str("gone off")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-style")); __args.push(py_tuple(vec![py_tuple(vec![py_str("what"), py_str("whats"), py_str("any"), py_str("good"), py_str("nice"), py_str("fancy"), py_str("fancy a"), py_str("i fancy"), py_str("feel like"), py_str("up for"), py_str("want"), py_str("mood"), py_str("the mood"), py_str("craving"), py_str("do"), py_str("ideas"), py_str("idea"), py_str("suggest"), py_str("quick"), py_str("easy"), py_str("best"), py_str("make"), py_str("cook"), py_str("having")]), g_DISH_STYLE()])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_MINE(), &g_TECHNIQUE()), &g_HOW_TO()), &g_HEALTH()), &g_DRINK()), &g_SLEEP()), &g_WHAT_GOES()), &g_MEASURING()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("takeaway-choice")); __args.push(py_tuple(vec![py_tuple(vec![py_str("takeaway"), py_str("takeaways"), py_str("take away"), py_str("chippy"), py_str("chip shop"), py_str("chinese"), py_str("indian"), py_str("kebab"), py_str("kebabs"), py_str("thai"), py_str("pizza night"), py_str("delivery"), py_str("deliveroo"), py_str("just eat"), py_str("curry house"), py_str("chicken shop")]), py_tuple(vec![py_str("should we get"), py_str("shall we get"), py_str("what should we get"), py_str("what shall we get"), py_str("what should i get"), py_str("what shall i get"), py_str("which"), py_str("which one"), py_str("cant decide"), py_str("can't decide"), py_str("decide"), py_str("choose"), py_str("any ideas"), py_str("ideas"), py_str("idea"), py_str("recommend"), py_str("what do you think"), py_str("or"), py_str("we always get"), py_str("same takeaway"), py_str("same old"), py_str("bored of"), py_str("sick of"), py_str("the kids"), py_str("kids want"), py_str("something different"), py_str("not the usual")])])); __args.push(py_str("Fish and chips feeds a group for the least money and nobody argues about it. If tastes differ, Indian beats Chinese: more vegetarian mains, and the sides stretch, so two mains with one rice and one bread feeds three. With a fussy child or someone veggie, a Chinese with plain rice and a separate veg dish keeps everyone fed. And if it is the same order every week, add one dish you have never had rather than swapping the whole thing - that is how you find the next usual.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("calorie"), py_str("calories"), py_str("healthy"), py_str("healthier"), py_str("diet"), py_str("slimming"), py_str("lose weight"), py_str("how do i make"), py_str("make at home"), py_str("recipe"), py_str("cook it"), py_str("how do i cook"), py_str("freeze"), py_str("freezing"), py_str("reheat"), py_str("left over"), py_str("leftover"), py_str("food poisoning"), py_str("hygiene"), py_str("arrived cold"), py_str("wrong order"), py_str("forgot"), py_str("missing"), py_str("refund"), py_str("complain"), py_str("complaint"), py_str("allergy"), py_str("allergic"), py_str("gluten"), py_str("price"), py_str("cheaper"), py_str("cost"), py_str("spicy"), py_str("mild"), py_str("heat"), py_str("paste"), py_str("powder"), py_str("sauce"), py_str("spice"), py_str("spices"), py_str("ingredient"), py_str("ingredients"), py_str("marinade")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-for-one")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cooking"), py_str("cook"), py_str("dinner"), py_str("tonight"), py_str("meal"), py_str("supper")]), py_tuple(vec![py_str("for one"), py_str("for myself"), py_str("just me"), py_str("on my own"), py_str("by myself")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_MINE_ASK(), &g_TECHNIQUE()), &g_DRINK()), &g_SLEEP()), &g_HEALTH()), &g_HOW_TO()), &g_GAME_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-for-two")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cooking"), py_str("cook"), py_str("dinner"), py_str("tonight"), py_str("meal"), py_str("supper"), py_str("date night"), py_str("date-night"), py_str("anniversary"), py_str("nice"), py_str("fancy"), py_str("special"), py_str("treat"), py_str("impress"), py_str("valentines"), py_str("valentine"), py_str("valentine's"), py_str("valentines day")]), py_tuple(vec![py_str("for two"), py_str("for 2"), py_str("for three"), py_str("for 3"), py_str("for four"), py_str("for 4"), py_str("for five"), py_str("for 5"), py_str("for the two of us"), py_str("date night"), py_str("date-night"), py_str("anniversary"), py_str("romantic"), py_str("looks nice"), py_str("look nice"), py_str("to impress"), py_str("for my girlfriend"), py_str("for my boyfriend"), py_str("for my partner"), py_str("for my wife"), py_str("for my husband"), py_str("for her"), py_str("for him"), py_str("for my mum"), py_str("for my mom"), py_str("for my dad"), py_str("to say sorry"), py_str("valentines"), py_str("valentine"), py_str("valentine's"), py_str("valentines day"), py_str("thing to cook"), py_str("something to cook"), py_str("nice thing to cook"), py_str("something nice to cook"), py_str("nice meal"), py_str("nice dinner")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_MINE_ASK(), &g_TECHNIQUE()), &g_DRINK()), &g_SLEEP()), &g_HEALTH()), &g_HOW_TO()), &g_GAME_WORDS()), &py_tuple(vec![py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("voucher"), py_str("buy him"), py_str("buy her"), py_str("get him"), py_str("get her"), py_str("what should i get"), py_str("what can i get"), py_str("what to get")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-carry")); __args.push(py_tuple(vec![g_CARRY_ASK(), py_tuple(vec![py_str("something"), py_str("anything"), py_str("make"), py_str("take"), py_str("bring"), py_str("pack"), py_str("packing"), py_str("food"), py_str("eat"), py_str("eating"), py_str("lunch"), py_str("ideas"), py_str("idea"), py_str("snack"), py_str("snacks"), py_str("family"), py_str("friends"), py_str("kids"), py_str("children"), py_str("toddler"), py_str("veggie"), py_str("vegetarian"), py_str("vegan")])])); __args.push(g__carry()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("ask".to_string(), py_tuple(vec![py_str("something"), py_str("anything"), py_str("make"), py_str("take"), py_str("bring"), py_str("pack"), py_str("packing"), py_str("food"), py_str("eat"), py_str("eating"), py_str("eaten"), py_str("lunch"), py_str("lunches"), py_str("lunchbox"), py_str("lunchboxes"), py_str("picnic"), py_str("picnics"), py_str("flask"), py_str("cool box"), py_str("coolbox"), py_str("cooler box"), py_str("drink"), py_str("drinks"), py_str("snack"), py_str("snacks"), py_str("ideas"), py_str("idea"), py_str("breakfast"), py_str("supper"), py_str("dinner"), py_str("tea"), py_str("hungry")])), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_MINE()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("picnic"), py_str("picnics")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g_TECHNIQUE()), &g_HOW_TO()), &g_HEALTH()), &g_SLEEP()), &g_DRINK()), &g_WHAT_GOES()), &g_MEASURING()), &g_MONEY_MATTERS()), &py_tuple(vec![py_str("eat out"), py_str("eating out"), py_str("anywhere to eat"), py_str("anywhere good to eat"), py_str("anywhere nice to eat"), py_str("somewhere to eat"), py_str("place to eat"), py_str("places to eat"), py_str("good place to eat"), py_str("restaurant"), py_str("restaurants"), py_str("cafe"), py_str("pub lunch")])), &py_tuple(vec![py_str("airport"), py_str("airports"), py_str("security"), py_str("hand luggage"), py_str("carry on"), py_str("carry-on"), py_str("cabin bag"), py_str("100ml"), py_str("liquids")])), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::knowledge::occasions::g_BRING_NOT_FOOD()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&word.clone(), &py_str("dog"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&word.clone(), &py_str("cat"))) } }) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-faster")); __args.push(py_tuple(vec![py_tuple(vec![py_str("quick"), py_str("quicker"), py_str("faster"), py_str("fast"), py_str("soon")]), py_tuple(vec![py_str("and"), py_str("please"), py_str("make it"), py_str("make"), py_str("something")])])); __args.push(g__later()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_NOT_FOR_ME(), &g_HEALTH()), &g_EMPTY()), &g_NOT_MY_KITCHEN()), &g_MEASURING()), &g_DRINK()), &g_HOW_TO()), &g_WHAT_GOES()), &g_NOT_AN_IDEA()), &g_SLEEP()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-limit-only")); __args.push(py_tuple(vec![py_tuple(vec![py_str("less"), py_str("no"), py_str("without"), py_str("only"), py_str("instead"), py_str("cut down"), py_str("cut out"), py_str("cutting down"), py_str("cutting out"), py_str("reduce"), py_str("minus"), py_str("skip"), py_str("quick"), py_str("faster"), py_str("half an hour")]), py_tuple(vec![py_str("meat"), py_str("dairy"), py_str("cheese"), py_str("fish"), py_str("pasta"), py_str("rice"), py_str("minutes"), py_str("minute"), py_str("hour"), py_str("gluten"), py_str("sugar")])])); __args.push(g_suggest()); __kws.extend(vec![("bias".to_string(), op_neg(&Py::Int(1))), ("ask".to_string(), py_tuple(vec![py_str("what"), py_str("shall"), py_str("should"), py_str("can i"), py_str("could i"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("recommend"), py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("cook"), py_str("make"), py_str("tonight"), py_str("quick"), py_str("fast"), py_str("hurry"), py_str("something"), py_str("instead"), py_str("please"), py_str("starving"), py_str("hungry")])), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&g_OWN_MEAL(), &g_OTHER_PEOPLE()), &g_NOT_A_DISH()), &g_MEASURING()), &g_HOW_TO()), &g_BROKEN()), &g_HEALTH()), &g_EMPTY()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pick-at-the-counter")); __args.push(py_tuple(vec![g_TAKEOUT_PLACES(), g_TAKEOUT_ASK()])); __args.push(g__takeout_pick()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("guard".to_string(), g__one_counter()), ("avoid".to_string(), py_tuple(vec![py_str("calorie"), py_str("calories"), py_str("healthy"), py_str("healthier"), py_str("diet"), py_str("slimming"), py_str("lose weight"), py_str("nutrition"), py_str("how many calories"), py_str("how many kcal"), py_str("how many carbs"), py_str("how much"), py_str("price"), py_str("cheaper"), py_str("cost"), py_str("expensive"), py_str("cheapest"), py_str("deal"), py_str("voucher"), py_str("discount"), py_str("open"), py_str("opening"), py_str("closing"), py_str("closed"), py_str("late"), py_str("near me"), py_str("nearby"), py_str("where is"), py_str("nearest"), py_str("deliver to"), py_str("allergy"), py_str("allergic"), py_str("gluten"), py_str("vegetarian"), py_str("vegan"), py_str("halal"), py_str("hygiene"), py_str("food poisoning"), py_str("refund"), py_str("wrong order"), py_str("missing"), py_str("arrived cold"), py_str("complain"), py_str("complaint"), py_str("how do i make"), py_str("make at home"), py_str("at home"), py_str("recipe"), py_str("cook it"), py_str("how do i cook"), py_str("spicy"), py_str("mild"), py_str("heat"), py_str("paste"), py_str("powder"), py_str("sauce"), py_str("spice"), py_str("spices"), py_str("marinade"), py_str("ingredient"), py_str("ingredients"), py_str("freeze"), py_str("freezing"), py_str("reheat"), py_str("left over"), py_str("leftover"), py_str("work"), py_str("job"), py_str("career"), py_str("wholesale"), py_str("bulk"), py_str("in spain"), py_str("in france"), py_str("in italy"), py_str("in germany"), py_str("topping"), py_str("toppings"), py_str("pizza oven"), py_str("pizza stone"), py_str("best pizza"), py_str("dough"), py_str("base from scratch"), py_str("how do i make"), py_str("make a pizza"), py_str("cook a pizza"), py_str("should i make"), py_str("can i make"), py_str("to make"), py_str("how long"), py_str("how hot"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("oven"), py_str("temperature"), py_str("temp"), py_str("degrees"), py_str("gas mark"), py_str("minutes"), py_str("takes to cook"), py_str("take to cook"), py_str("until crisp"), py_str("until golden"), py_str("bad for you"), py_str("is pizza"), py_str("bin"), py_str("bins"), py_str("recycling"), py_str("recycle"), py_str("cardboard"), py_str("compost"), py_str("bin it"), py_str("throw away"), py_str("get rid"), py_str("box"), py_str("boxes"), py_str("crispy"), py_str("crisp"), py_str("crust"), py_str("soggy"), py_str("dough"), py_str("flour"), py_str("yeast"), py_str("rise"), py_str("prove"), py_str("proving"), py_str("proofing"), py_str("stretch"), py_str("stretching"), py_str("homemade"), py_str("from scratch"), py_str("my pizza"), py_str("the base"), py_str("sticking"), py_str("sticks"), py_str("stuck"), py_str("tray"), py_str("trays"), py_str("thin crust"), py_str("thick crust")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    let __iter4 = py_iter_or_raise(&g_ENTRIES());
    for __item in __iter4 {
    s__row(__item.clone());
    let _ = py_setattr(&g__row(), "avoid", op_add(&call_builtin("tuple", &[gattr(&g__row(), "avoid")]).unwrap_or_else(|e| exc::raise(e)), &g_NOT_A_MEAL_SUBJECT()));
    }
}
pub fn cheap(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut use_: Py = Py::None;
    let _ = py_str("A dish a tight month can carry: nothing bought fresh and skippable in it.");
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&item.clone(), "uses")) { let mut use_: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_CHEAP(), &use_.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _up_food(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The first food in the line this table answers, as the line spells it.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&g__UP_FOOD_SET(), &word.clone()))) {
    return word.clone();
    }
    if py_truthy(&{ let __b = op_lt(&op_add(&index.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__UP_FOOD_SET(), &mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &index.clone(), &op_add(&index.clone(), &Py::Int(2)), &Py::None)]))) } }) {
    return mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &index.clone(), &op_add(&index.clone(), &Py::Int(2)), &Py::None)]);
    }
    }
    return Py::None;
    Py::None
}
pub fn idea_is_a_dish(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut idea: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "idea") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: idea"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when a pantry idea is a way to use the food up rather than how to cook it raw.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_USE_DISH_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&call_builtin("str", &[{ let __b = idea.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn named_food(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut named: Py = Py::None;
    let _ = py_str("The food a line names, preferring the kind this table knows, or None.");
    named = { let __b = _up_food(&[words.clone()]); if py_truthy(&__b) { __b } else { _named(&[{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]) } };
    if py_truthy(&call_builtin("isinstance", &[named.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    named = if py_truthy(&named.clone()) { geti(&named.clone(), &Py::Int(0)) } else { Py::None };
    }
    return { let __b = named.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn use_it_up(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut folded: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How to use up a spare or cooked food, told by the kind of food it is, naming it.");
    said = mcall(&call_builtin("str", &[{ let __b = name.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    folded = mcall(&said.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&g_UP_CLASSES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    words = __unpk2[0].clone();
    move_ = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &folded.clone()))) {
    return op_modulo(&py_str("Leftover %s: %s"), &py_tuple(vec![said.clone(), move_.clone()]));
    }
    }
    let __iter3 = py_iter_or_raise(&g_UP_CLASSES());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    words = __unpk4[0].clone();
    move_ = __unpk4[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = mcall(&folded.clone(), "endswith", &[op_add(&py_str(" "), &word.clone())]); if py_truthy(&__b) { __b } else { mcall(&folded.clone(), "startswith", &[op_add(&word.clone(), &py_str(" "))]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_modulo(&py_str("Leftover %s: %s"), &py_tuple(vec![said.clone(), move_.clone()]));
    }
    }
    return op_modulo(&py_str("Leftover %s: already cooked, so treat it as an ingredient - stir through a soup or a curry, fry with an onion and an egg, or eat cold in a salad with oil and vinegar."), &said.clone());
    Py::None
}
pub fn _variant(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut options: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "options") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: options"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut c: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("One of options, varied by the line itself rather than by the turn count alone.");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    clean = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } else { py_str("") };
    seed = op_add(&op_mul(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&clean.clone()) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    return mcall(&stdlib_call("random", "Random", &[seed.clone()]).unwrap_or_else(|e| exc::raise(e)), "choice", &[call_builtin("list", &[options.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _no_have__wrapped(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cue: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cue") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cue"))) };
    let mut shape: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "shape") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: shape"))) };
    let _ = py_str("The \"no X\" pattern for one cue and one ingredient, compiled once.\n\n    `_ruled_out` asks for one pattern per cue and per ingredient form - about 13k of them for a\n    single meal line - and `re` caches only 512 compiled patterns, so every one of those was\n    compiled again on every turn: 5-7 s of a reply that reads a line and picks a dish. The\n    pattern depends on nothing but these two words, so it is built once and kept.\n    ");
    return stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&op_add(&py_str("(?<![a-z])"), &stdlib_call("re", "escape", &[cue.clone()]).unwrap_or_else(|e| exc::raise(e))), &g_NO_HAVE_GAP()), &py_str(" ")), &stdlib_call("re", "escape", &[shape.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![a-z])"))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _ruled_out(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut gap: Py = Py::None;
    let mut ingredient: Py = Py::None;
    let mut out: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut shape: Py = Py::None;
    let _ = py_str("The ingredients the line rules out: \"anything but pasta\", \"ive had enough rice\".");
    plain = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&g_WORD(), "findall", &[mcall(&text.clone(), "replace", &[py_str("'"), py_str("")])])])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_DOUBLE_NO());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&plain.clone(), &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&phrase.clone(), -1, "")); __s.push_str(" "); py_string(__s) }))) {
    plain = mcall(&plain.clone(), "replace", &[{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&phrase.clone(), -1, "")); __s.push_str(" "); py_string(__s) }, py_str(" ")]);
    }
    }
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&mcall(&g_INGREDIENTS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    ingredient = __unpk3[0].clone();
    forms = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&forms.clone());
    for __item in __iter4 {
    form = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&form.clone(), "isalpha", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&form.clone(), &py_str(" "))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&form.clone(), "isalpha", &[])))) {
    shape = mcall(&form.clone(), "replace", &[py_str("'"), py_str("")]);
    } else {
    shape = form.clone();
    }
    if py_truthy(&Py::Bool(!py_contains(&plain.clone(), &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&shape.clone(), -1, "")); __s.push_str(" "); py_string(__s) }))) {
    continue;
    }
    let __iter5 = py_iter_or_raise(&g_NO_CUE());
    let mut __else6 = false;
    for __item in __iter5 {
    cue = __item.clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NO_GAP()) { let mut gap: Py = __item.clone(); __out.push(Py::Bool(py_contains(&plain.clone(), &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&cue.clone(), -1, "")); __s.push_str(&fstr(&gap.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&shape.clone(), -1, "")); __s.push_str(" "); py_string(__s) }))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&out.clone(), "add", &[ingredient.clone()]);
    break;
    }
    }
    let __iter7 = py_iter_or_raise(&g_NO_HAVE());
    for __item in __iter7 {
    cue = __item.clone();
    if py_truthy(&mcall(&py_call(&g__no_have(), &[cue.clone(), shape.clone()]), "search", &[plain.clone()])) {
    let _ = mcall(&out.clone(), "add", &[ingredient.clone()]);
    break;
    }
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn _the_fish_are_out(args: &[Py]) -> Py {
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
    let mut diet: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "diet") { v.clone() }
    else { py_str("") };
    let mut cue: Py = Py::None;
    let mut family: Py = Py::None;
    let mut plain: Py = Py::None;
    let _ = py_str("True when it is fish itself that is out, not one fish named by itself.\n\n    \"i dont eat fish\", \"no seafood\" and \"allergic to fish\" are about the family, so every fish in\n    FISH_INGREDIENTS goes; \"i dont eat salmon\" is about the salmon, and the other fish stay.\n    ");
    plain = op_add(&op_add(&py_str(" "), &mcall(&py_str(" "), "join", &[mcall(&g_WORD(), "findall", &[mcall(&clean.clone(), "replace", &[py_str("'"), py_str("")])])])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_NO_CUE());
    for __item in __iter1 {
    cue = __item.clone();
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("fish"), py_str("seafood")]));
    for __item in __iter2 {
    family = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&plain.clone(), &{ let mut __s = String::new(); __s.push_str(" "); __s.push_str(&fstr(&cue.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&family.clone(), -1, "")); __s.push_str(" "); py_string(__s) }))) {
    return Py::Bool(true);
    }
    }
    }
    diet = mcall(&{ let __b = diet.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    return { let __b = Py::Bool(py_contains(&diet.clone(), &py_str("fish"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&diet.clone(), &py_str("seafood"))) } };
    Py::None
}
pub fn _is_the_ruled_out_food(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let mut ingredient: Py = Py::None;
    let _ = py_str("Whether a food word in the line is one of the foods the line rules out.\n\n    `_ruled_out` answers with the pantry's name for the food, and a line says one of its forms:\n    \"i cant stand fish what can i cook\" rules out `white fish`, and \"fish\" is a form of it. The\n    caller has the word from the line and needs to know whether it is the same food, which is not\n    the same question as whether the two spellings match.\n    ");
    word = mcall(&mcall(&word.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&_ruled_out(&[text.clone()]));
    for __item in __iter1 {
    ingredient = __item.clone();
    let __iter2 = py_iter_or_raise(&op_add(&mcall(&g_INGREDIENTS(), "get", &[ingredient.clone(), py_tuple(vec![])]), &py_tuple(vec![ingredient.clone()])));
    for __item in __iter2 {
    form = __item.clone();
    form = mcall(&mcall(&form.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&form.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&form.clone(), "startswith", &[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&word.clone(), "startswith", &[form.clone()]) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut found: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut ingredient: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut text: Py = Py::None;
    let mut tidied: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The canonical ingredients the line names, in the order they are mentioned.\n\n    It reads the words as typed and the tidied line the understanding engine keeps in\n    \"norm\", so a slip it has already fixed (\"tomartoes\", \"potatos\") still finds the thing.\n    ");
    words = call_builtin("list", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    tidied = mcall(&understood.clone(), "get", &[py_str("norm_text")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[tidied.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    tidied = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_WORD(), "findall", &[tidied.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&words.clone(), &word.clone()))) {
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    }
    text = mcall(&py_str(" "), "join", &[words.clone()]);
    found = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&mcall(&g_INGREDIENTS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    ingredient = __unpk3[0].clone();
    forms = __unpk3[1].clone();
    phrases = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&form.clone(), "isalpha", &[])))) {  __out.push(form.clone()); } } py_list(__out) })() };
    hit = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&text.clone(), &phrase.clone()))) {  __out.push(phrase.clone()); } } py_list(__out) })() };
    if py_truthy(&hit.clone()) {
    let _ = mcall(&found.clone(), "append", &[ingredient.clone()]);
    let __iter4 = py_iter_or_raise(&hit.clone());
    for __item in __iter4 {
    phrase = __item.clone();
    text = mcall(&text.clone(), "replace", &[phrase.clone(), py_str(" ")]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&phrase.clone(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    }
    }
    }
    let __iter5 = py_iter_or_raise(&mcall(&g_INGREDIENTS(), "items", &[]));
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    ingredient = __unpk6[0].clone();
    forms = __unpk6[1].clone();
    if py_truthy(&Py::Bool(py_contains(&found.clone(), &ingredient.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&mcall(&form.clone(), "isalpha", &[])) {  __out.push(Py::Bool(py_contains(&words.clone(), &form.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&found.clone(), "append", &[ingredient.clone()]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _clock(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut numbers: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "numbers") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: numbers"))) };
    let mut number: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The minutes a line allows, from \"20 minutes\" or \"half an hour\", else None.");
    let __iter1 = py_iter_or_raise(&{ let __b = numbers.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[number.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("minutes"), py_str("minute"), py_str("min"), py_str("mins")]), &mcall(&number.clone(), "get", &[py_str("unit")])))) {
    continue;
    }
    let __try2 = exc::catch(|| {
    value = call_builtin("int", &[mcall(&number.clone(), "get", &[py_str("value")])]).unwrap_or_else(|e| exc::raise(e));
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
    if py_truthy(&op_gt(&value.clone(), &Py::Int(0))) {
    return value.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _no_meat(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut cue_words: Py = Py::None;
    let mut same: Py = Py::None;
    let mut start: Py = Py::None;
    let mut width: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A line that rules meat out, in whatever order the words come.\n\n    \"im vegetarian\" and \"no meat\" are the plain forms. \"she doesnt eat meat\" puts the meat\n    after the negative, so the words just after a \"no\", \"without\" or \"doesnt eat\" are read\n    too. Only whole words count and only a few words of slack, so \"i know meat is pricey\" and\n    \"i have no time to cook chicken\" are not a diet.\n    ");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NO_MEAT()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&text.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "strip", &[py_str(",.!?;:")])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&g_NO_CUE());
    for __item in __iter1 {
    cue = __item.clone();
    cue_words = mcall(&cue.clone(), "split", &[]);
    width = call_builtin("len", &[cue_words.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &width.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    same = Py::Bool(py_eq(&slice_or_raise(&words.clone(), &start.clone(), &op_add(&start.clone(), &width.clone()), &Py::None), &cue_words.clone()));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&same.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&width.clone(), &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("nothing"), py_str("none"), py_str("nobody")]), &geti(&words.clone(), &start.clone()))) } } } }) {
    same = Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&same.clone()))) {
    continue;
    }
    after = slice_or_raise(&words.clone(), &op_add(&start.clone(), &width.clone()), &op_add(&op_add(&start.clone(), &width.clone()), &Py::Int(5)), &Py::None);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__NO_MEAT_BLOCKER(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&after.clone(), &py_str("meat"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&after.clone(), &py_str("chicken"))) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _later(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A limit said after an idea (\"and quick\", \"and something vegetarian\") means something\n    only when an idea is on the table or the line is about food at all.\n\n    Without this, \"whats a quick and easy way to learn excel\" reads as a request for a\n    quicker dinner, because \"quick\" and \"and\" are enough to match.\n    ");
    plan = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal_plan")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("meal_plan")]) } else { py_dict(vec![]) };
    if py_truthy(&mcall(&plan.clone(), "get", &[py_str("offered")])) {
    return suggest(&[memory.clone(), understood.clone()]);
    }
    text = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOOD_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("recipe"), py_str("cook"), py_str("eating"), py_str("eat"), py_str("food")])) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return suggest(&[memory.clone(), understood.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _limits(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut allergy: Py = Py::None;
    let mut allowed: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut diet: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut item: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut name: Py = Py::None;
    let mut order: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plan_now: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_now: Py = Py::None;
    let mut text: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut was: Py = Py::None;
    let mut without: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What this line rules out: (allowed diets or None, ingredients to avoid, max minutes).\n\n    An earlier turn of the same request keeps its limits (\"dinner?\" -> \"without meat\" ->\n    \"and quick\"), for a few turns, and only while the line adds no ingredients of its own:\n    naming new food starts a fresh request.\n    ");
    clean = { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    text = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    allowed = __unpk1[0].clone();
    without = __unpk1[1].clone();
    said_now = call_builtin("set", &[_named(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&_no_meat(&[clean.clone()])) {
    allowed = py_set(vec![py_str("vegan"), py_str("vegetarian")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &py_str("vegan"))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NO_DAIRY()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    allowed = py_set(vec![py_str("vegan")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&clean.clone(), &py_str("no fish"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("without fish"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("no seafood"))) } } } }) {
    allowed = op_sub(&call_builtin("set", &[{ let __b = allowed.clone(); if py_truthy(&__b) { __b } else { py_set(vec![py_str("vegan"), py_str("vegetarian"), py_str("fish")]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("fish")]));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NO_CHEESE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&without.clone(), "add", &[py_str("cheese")]);
    }
    let _ = mcall(&without.clone(), "update", &[_ruled_out(&[clean.clone()])]);
    minutes = _clock(&[mcall(&understood.clone(), "get", &[py_str("numbers")])]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("quicker"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("faster"))) } } } }) {
    plan_now = mcall(&memory.clone(), "get", &[py_str("meal_plan")]);
    was = if py_truthy(&call_builtin("isinstance", &[plan_now.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&{ let __b = plan_now.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pick")]) } else { py_str("") };
    order = op_add(&if py_truthy(&was.clone()) { py_list(vec![was.clone()]) } else { py_list(vec![]) }, &call_builtin("list", &[call_builtin("reversed", &[if py_truthy(&call_builtin("isinstance", &[plan_now.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { { let __b = mcall(&{ let __b = plan_now.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } } } else { py_tuple(vec![]) }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let __iter2 = py_iter_or_raise(&order.clone());
    for __item in __iter2 {
    name = __item.clone();
    item = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut dish: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&gattr(&dish.clone(), "name"), &name.clone()))) {  __out.push(dish.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_is(&item.clone(), &Py::None))) {
    minutes = op_sub(&gattr(&item.clone(), "minutes"), &Py::Int(1));
    break;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_QUICK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&memory.clone(), "get", &[py_str("meal_plan")]); if py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FOOD_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("recipe"), py_str("eating"), py_str("cook")])) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } } } }) {
    minutes = Py::Int(20);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&clean.clone(), &py_str("half an hour"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("30 minutes"))) } } } }) {
    minutes = Py::Int(30);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BEDTIME()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    minutes = call_builtin("min", &[{ let __b = minutes.clone(); if py_truthy(&__b) { __b } else { Py::Int(15) } }, Py::Int(15)]).unwrap_or_else(|e| exc::raise(e));
    }
    about = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    diet = mcall(&call_builtin("str", &[{ let __b = mcall(&about.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { diet.clone() } }) {
    if py_truthy(&Py::Bool(py_contains(&diet.clone(), &py_str("vegan")))) {
    allowed = py_set(vec![py_str("vegan")]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&diet.clone(), &py_str("vegetarian")))) {
    allowed = py_set(vec![py_str("vegan"), py_str("vegetarian")]);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&diet.clone(), &py_str("dairy"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&diet.clone(), &py_str("lactose"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&diet.clone(), &py_str("cheese"))) } } } }) {
    let _ = mcall(&without.clone(), "add", &[py_str("cheese")]);
    }
    if py_truthy(&_the_fish_are_out(&[clean.clone(), diet.clone()])) {
    let _ = mcall(&without.clone(), "update", &[g_FISH_INGREDIENTS()]);
    }
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&about.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    allergy = __item.clone();
    name = mcall(&mcall(&call_builtin("str", &[allergy.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "rstrip", &[py_str("s")]);
    if py_truthy(&Py::Bool(py_contains(&g_INGREDIENTS(), &name.clone()))) {
    let _ = mcall(&without.clone(), "add", &[name.clone()]);
    }
    }
    let __iter4 = py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter4 {
    person = __item.clone();
    theirs = if py_truthy(&call_builtin("isinstance", &[person.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = person.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&theirs.clone(), &py_str("vegan"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&theirs.clone(), &py_str("plant"))) } }) {
    allowed = py_set(vec![py_str("vegan")]);
    } else {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&theirs.clone(), &py_str("vegetarian"))); if py_truthy(&__b) { __b } else { _no_meat(&[theirs.clone()]) } }) {
    if py_truthy(&Py::Bool(py_is(&allowed.clone(), &Py::None))) {
    allowed = py_set(vec![py_str("vegan"), py_str("vegetarian")]);
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&theirs.clone(), &py_str("dairy"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&theirs.clone(), &py_str("cheese"))) } }) {
    let _ = mcall(&without.clone(), "add", &[py_str("cheese")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&theirs.clone(), &py_str("fish"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&theirs.clone(), &py_str("seafood"))) } }) {
    let _ = mcall(&without.clone(), "update", &[py_tuple(vec![py_str("fish"), py_str("tuna"), py_str("salmon"), py_str("prawn"), py_str("cod")])]);
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&allowed.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&without.clone())) } }) {
    let __iter5 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }, &op_neg(&Py::Int(6)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    turn = __item.clone();
    said = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = _no_meat(&[said.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("vegan"))) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IDEA_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { _no_meat(&[said.clone()]) } }) {
    allowed = py_set(vec![py_str("vegan"), py_str("vegetarian")]);
    }
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &py_str("vegan")))) {
    allowed = py_set(vec![py_str("vegan")]);
    }
    }
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NO_CHEESE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("no dairy"))) } }) {
    let _ = mcall(&without.clone(), "add", &[py_str("cheese")]);
    }
    let __iter6 = py_iter_or_raise(&_ruled_out(&[said.clone()]));
    for __item in __iter6 {
    gone = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&said_now.clone(), &gone.clone()))) {
    let _ = mcall(&without.clone(), "add", &[gone.clone()]);
    }
    }
    }
    }
    plan = mcall(&memory.clone(), "get", &[py_str("meal_plan")]);
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_named(&[understood.clone()])) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&without.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[plan.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&wanted.clone())); if !py_truthy(&__b) { __b } else { op_le(&op_sub(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &mcall(&plan.clone(), "get", &[py_str("turn"), Py::Int(0)])), &Py::Int(3)) } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&plan.clone(), "get", &[py_str("allowed")]) } }) {
    allowed = call_builtin("set", &[geti(&plan.clone(), &py_str("allowed"))]).unwrap_or_else(|e| exc::raise(e));
    }
    let _ = mcall(&without.clone(), "update", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("without")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    minutes = if py_truthy(&Py::Bool(!py_is(&minutes.clone(), &Py::None))) { minutes.clone() } else { mcall(&plan.clone(), "get", &[py_str("minutes")]) };
    }
    return py_tuple(vec![allowed.clone(), without.clone(), minutes.clone()]);
    Py::None
}
pub fn _fits(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut allowed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let mut without: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: without"))) };
    let mut minutes: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "minutes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: minutes"))) };
    let mut grace: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "grace") { v.clone() }
    else { Py::Int(0) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&allowed.clone(), &gattr(&item.clone(), "diet"))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = without.clone(); if !py_truthy(&__b) { __b } else { op_bitand(&call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)), &without.clone()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&gattr(&item.clone(), "minutes"), &op_add(&minutes.clone(), &grace.clone())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _rotate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut items: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "items") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: items"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut c: Py = Py::None;
    let mut seed: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("The list turned to start at a place that follows the line, so the same ask varies.");
    if py_truthy(&Py::Bool(!py_truthy(&items.clone()))) {
    return items.clone();
    }
    seed = op_add(&op_mul(&call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7919)), &call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&understood.clone(), "get", &[py_str("clean"), py_str("")])) { let mut c: Py = __item.clone(); __out.push(call_builtin("ord", &[c.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    start = op_modulo(&seed.clone(), &call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return op_add(&call_builtin("list", &[slice_or_raise(&items.clone(), &start.clone(), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[slice_or_raise(&items.clone(), &Py::None, &start.clone(), &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _spread(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ranked: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ranked") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ranked"))) };
    let mut chosen: Py = Py::None;
    let mut distinct: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut high: Py = Py::None;
    let mut item: Py = Py::None;
    let mut low: Py = Py::None;
    let mut picks: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut shapes: Py = Py::None;
    let _ = py_str("Three dishes, quickest first: the ten-minute one, the twenty-minute one, the slower one.\n\n    Three is what a person says when a friend asks what to cook: enough to choose between,\n    short enough to read. Two of them are in the quick half of the table, so the list is not\n    three variations on the same slow supper.\n    ");
    picks = py_list(vec![]);
    shapes = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![Py::Int(0), Py::Int(15)]), py_tuple(vec![Py::Int(16), Py::Int(25)]), py_tuple(vec![Py::Int(26), Py::Int(95)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    fits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut item: Py = __item.clone(); if py_truthy(&{ let __cmp11: Py = gattr(&item.clone(), "minutes"); if py_truthy(&op_le(&low.clone(), &__cmp11)) { op_le(&__cmp11, &high.clone()) } else { Py::Bool(false) } }) {  __out.push(item.clone()); } } py_list(__out) })() };
    distinct = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fits.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&shapes.clone(), &_shape(&[item.clone()])))) {  __out.push(item.clone()); } } py_list(__out) })() };
    let __iter3 = py_iter_or_raise(&py_tuple(vec![distinct.clone(), fits.clone()]));
    for __item in __iter3 {
    pool = __item.clone();
    if py_truthy(&pool.clone()) {
    chosen = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&gattr(&item.clone(), "favourite")) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { pool.clone() } };
    let _ = mcall(&picks.clone(), "append", &[geti(&chosen.clone(), &Py::Int(0))]);
    let _ = mcall(&shapes.clone(), "add", &[_shape(&[geti(&chosen.clone(), &Py::Int(0))])]);
    break;
    }
    }
    }
    return picks.clone();
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
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut shape: Py = Py::None;
    let mut words: Py = Py::None;
    words = op_add(&op_add(&mcall(&gattr(&item.clone(), "name"), "replace", &[py_str("-"), py_str(" ")]), &py_str(" ")), &mcall(&mcall(&gattr(&item.clone(), "name"), "replace", &[py_str("-"), py_str(" ")]), "replace", &[py_str(" "), py_str("")]));
    let __iter1 = py_iter_or_raise(&g_DISH_SHAPES());
    for __item in __iter1 {
    shape = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&mcall(&mcall(&gattr(&item.clone(), "name"), "replace", &[py_str("-"), py_str(" ")]), "replace", &[py_str(" "), py_str("")]), &mcall(&shape.clone(), "replace", &[py_str(" "), py_str("")])))) {
    return shape.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _best(args: &[Py]) -> Py {
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
    let mut allowed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let mut without: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: without"))) };
    let mut minutes: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "minutes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: minutes"))) };
    let mut memory: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut understood: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut across: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut favourites: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut grace: Py = Py::None;
    let mut hard: Py = Py::None;
    let mut item: Py = Py::None;
    let mut light: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_shape: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut order: Py = Py::None;
    let mut others: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut rank: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut shapes: Py = Py::None;
    let mut still: Py = Py::None;
    let mut style: Py = Py::None;
    let mut styled: Py = Py::None;
    let mut thrifty: Py = Py::None;
    let mut using: Py = Py::None;
    let mut wider: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The dish that uses most of what they named, fewest extra ingredients, then quickest.\n\n    With nothing named, a favourite first, in the order a friend would offer them; a dish\n    already offered in this request goes to the back.\n    ");
    plan = mcall(&memory.clone(), "get", &[py_str("meal_plan")]);
    offered = if py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("set", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e)) };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[plan.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { op_ge(&mcall(&plan.clone(), "get", &[py_str("turn"), op_neg(&Py::Int(1))]), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))) } }) {
    offered = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    style = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_STYLE()) { let mut cue: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&said.clone(), &cue.clone()))) {  __out.push(cue.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&mcall(&g_STYLE_CUES(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    shapes = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &word.clone()))) {
    let __aug3 = op_add(&style.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&shapes.clone()) { let mut shape: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&style.clone(), &shape.clone()))) {  __out.push(shape.clone()); } } py_list(__out) })() });
    style = __aug3;
    style = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&style.clone()) { let mut shape: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DISH_STYLE(), &shape.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_contains(&gattr(&item.clone(), "name"), &shape.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(shape.clone()); } } py_list(__out) })() };
    }
    }
    named = call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e));
    words = call_builtin("set", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    hard = call_builtin("bool", &[op_bitand(&words.clone(), &call_builtin("set", &[g_HARD_CLOCK()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    grace = if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&hard.clone())) } }) { g_GRACE() } else { Py::Int(0) };
    pool = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); if py_truthy(&_fits(&[item.clone(), allowed.clone(), without.clone(), minutes.clone(), grace.clone()])) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    pool = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); if py_truthy(&_fits(&[item.clone(), allowed.clone(), without.clone(), Py::None])) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[g_DISHES()]).unwrap_or_else(|e| exc::raise(e)) } };
    }
    across = _said_about(&[memory.clone(), said.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHEAP_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&across.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    thrifty = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&cheap(&[item.clone()])) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); if py_truthy(&cheap(&[item.clone()])) {  __out.push(item.clone()); } } py_list(__out) })() } } };
    if py_truthy(&thrifty.clone()) {
    pool = thrifty.clone();
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LIGHT_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&across.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    light = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_LIGHT_SET(), &gattr(&item.clone(), "name")))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&light.clone()) {
    pool = light.clone();
    }
    }
    if py_truthy(&style.clone()) {
    named_shape = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_STYLE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    styled = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&style.clone()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&gattr(&item.clone(), "name"), "replace", &[py_str("-"), py_str(" ")]), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _fits(&[item.clone(), allowed.clone(), without.clone(), if py_truthy(&named_shape.clone()) { Py::None } else { minutes.clone() }, grace.clone()]) } }) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&styled.clone()) {
    pool = styled.clone();
    }
    }
    if py_truthy(&named.clone()) {
    using = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&using.clone()))) {
    using = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); if py_truthy(&{ let __b = op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _fits(&[item.clone(), allowed.clone(), without.clone(), Py::None]) } }) {  __out.push(item.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&using.clone()) {
    pool = using.clone();
    }
    }
    if py_truthy(&{ let __b = offered.clone(); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); __out.push(Py::Bool(py_contains(&offered.clone(), &gattr(&item.clone(), "name")))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&offered.clone(), &gattr(&item.clone(), "name")))) {  __out.push(item.clone()); } } py_list(__out) })() };
    still = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = fresh.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&still.clone())) } } } }) {
    pool = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { pool.clone() } };
    } else {
    wider = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut item: Py = __item.clone(); if py_truthy(&{ let __b = op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _fits(&[item.clone(), allowed.clone(), without.clone(), minutes.clone()]) } }) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut item: Py = __item.clone(); if py_truthy(&op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)))) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fresh.clone()) { let mut item: Py = __item.clone(); if py_truthy(&_fits(&[item.clone(), allowed.clone(), without.clone(), minutes.clone()])) {  __out.push(item.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { fresh.clone() } } } } } };
    if py_truthy(&wider.clone()) {
    pool = wider.clone();
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    favourites = _rotate(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&gattr(&item.clone(), "favourite")) {  __out.push(item.clone()); } } py_list(__out) })() }, understood.clone(), memory.clone()]);
    others = _rotate(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pool.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&gattr(&item.clone(), "favourite")))) {  __out.push(item.clone()); } } py_list(__out) })() }, understood.clone(), memory.clone()]);
    order = op_add(&favourites.clone(), &others.clone());
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let across = across.clone(); let allowed = allowed.clone(); let cue = cue.clone(); let favourites = favourites.clone(); let fresh = fresh.clone(); let grace = grace.clone(); let hard = hard.clone(); let item = item.clone(); let light = light.clone(); let memory = memory.clone(); let minutes = minutes.clone(); let named = named.clone(); let named_shape = named_shape.clone(); let names = names.clone(); let offered = offered.clone(); let order = order.clone(); let others = others.clone(); let plan = plan.clone(); let pool = pool.clone(); let rank = rank.clone(); let said = said.clone(); let shape = shape.clone(); let shapes = shapes.clone(); let still = still.clone(); let style = style.clone(); let styled = styled.clone(); let thrifty = thrifty.clone(); let understood = understood.clone(); let using = using.clone(); let wider = wider.clone(); let without = without.clone(); let word = word.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let across = across.clone(); let allowed = allowed.clone(); let cue = cue.clone(); let favourites = favourites.clone(); let fresh = fresh.clone(); let grace = grace.clone(); let hard = hard.clone(); let item = item.clone(); let light = light.clone(); let memory = memory.clone(); let minutes = minutes.clone(); let named = named.clone(); let named_shape = named_shape.clone(); let names = names.clone(); let offered = offered.clone(); let order = order.clone(); let others = others.clone(); let plan = plan.clone(); let pool = pool.clone(); let rank = rank.clone(); let said = said.clone(); let shape = shape.clone(); let shapes = shapes.clone(); let still = still.clone(); let style = style.clone(); let styled = styled.clone(); let thrifty = thrifty.clone(); let understood = understood.clone(); let using = using.clone(); let wider = wider.clone(); let without = without.clone(); let word = word.clone(); let words = words.clone(); let item: Py = __kwp(args, 0, "item").unwrap_or(Py::None); Py::Bool(py_contains(&offered.clone(), &gattr(&item.clone(), "name"))) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&order.clone(), "sort", args))(&__args) };
    return order.clone();
    }
pub fn __nested_rank_3(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut across: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut allowed: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut cue: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut favourites: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut fresh: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut grace: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut hard: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut light: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut minutes: Py = __pos.get(10).cloned().unwrap_or(Py::None);
    let mut named: Py = __pos.get(11).cloned().unwrap_or(Py::None);
    let mut named_shape: Py = __pos.get(12).cloned().unwrap_or(Py::None);
    let mut names: Py = __pos.get(13).cloned().unwrap_or(Py::None);
    let mut offered: Py = __pos.get(14).cloned().unwrap_or(Py::None);
    let mut order: Py = __pos.get(15).cloned().unwrap_or(Py::None);
    let mut others: Py = __pos.get(16).cloned().unwrap_or(Py::None);
    let mut plan: Py = __pos.get(17).cloned().unwrap_or(Py::None);
    let mut pool: Py = __pos.get(18).cloned().unwrap_or(Py::None);
    let mut rank: Py = __pos.get(19).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(20).cloned().unwrap_or(Py::None);
    let mut shape: Py = __pos.get(21).cloned().unwrap_or(Py::None);
    let mut shapes: Py = __pos.get(22).cloned().unwrap_or(Py::None);
    let mut still: Py = __pos.get(23).cloned().unwrap_or(Py::None);
    let mut style: Py = __pos.get(24).cloned().unwrap_or(Py::None);
    let mut styled: Py = __pos.get(25).cloned().unwrap_or(Py::None);
    let mut thrifty: Py = __pos.get(26).cloned().unwrap_or(Py::None);
    let mut understood: Py = __pos.get(27).cloned().unwrap_or(Py::None);
    let mut using: Py = __pos.get(28).cloned().unwrap_or(Py::None);
    let mut wider: Py = __pos.get(29).cloned().unwrap_or(Py::None);
    let mut without: Py = __pos.get(30).cloned().unwrap_or(Py::None);
    let mut word: Py = __pos.get(31).cloned().unwrap_or(Py::None);
    let mut words: Py = __pos.get(32).cloned().unwrap_or(Py::None);
    let mut item: Py = __pos.get(33).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "item").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut extra: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut titled: Py = Py::None;
    let mut used: Py = Py::None;
    used = call_builtin("len", &[op_bitand(&named.clone(), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e));
    extra = op_sub(&call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)), &named.clone());
    lead = { let __b = call_builtin("bool", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gattr(&item.clone(), "uses"), &geti(&names.clone(), &Py::Int(0)))) } };
    titled = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&gattr(&item.clone(), "name"), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return py_tuple(vec![Py::Bool(py_contains(&offered.clone(), &gattr(&item.clone(), "name"))), op_neg(&used.clone()), Py::Bool(!py_truthy(&lead.clone())), call_builtin("len", &[op_sub(&extra.clone(), &g_STAPLE())]).unwrap_or_else(|e| exc::raise(e)), call_builtin("len", &[extra.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Bool(!py_truthy(&titled.clone())), gattr(&item.clone(), "minutes")]);
    Py::None
}
    rank = {{ let across = across.clone(); let allowed = allowed.clone(); let cue = cue.clone(); let favourites = favourites.clone(); let fresh = fresh.clone(); let grace = grace.clone(); let hard = hard.clone(); let item = item.clone(); let light = light.clone(); let memory = memory.clone(); let minutes = minutes.clone(); let named = named.clone(); let named_shape = named_shape.clone(); let names = names.clone(); let offered = offered.clone(); let order = order.clone(); let others = others.clone(); let plan = plan.clone(); let pool = pool.clone(); let rank = rank.clone(); let said = said.clone(); let shape = shape.clone(); let shapes = shapes.clone(); let still = still.clone(); let style = style.clone(); let styled = styled.clone(); let thrifty = thrifty.clone(); let understood = understood.clone(); let using = using.clone(); let wider = wider.clone(); let without = without.clone(); let word = word.clone(); let words = words.clone(); py_func("rank", move |args: &[Py]| { let mut full = vec![across.clone(), allowed.clone(), cue.clone(), favourites.clone(), fresh.clone(), grace.clone(), hard.clone(), item.clone(), light.clone(), memory.clone(), minutes.clone(), named.clone(), named_shape.clone(), names.clone(), offered.clone(), order.clone(), others.clone(), plan.clone(), pool.clone(), rank.clone(), said.clone(), shape.clone(), shapes.clone(), still.clone(), style.clone(), styled.clone(), thrifty.clone(), understood.clone(), using.clone(), wider.clone(), without.clone(), word.clone(), words.clone()]; full.extend_from_slice(args); __nested_rank_3(&full) }) }};
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __kws.extend(vec![("key".to_string(), rank.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _prose(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The ingredients as a person would say them: \"chickpeas and tomatoes\", not \"chickpea\".");
    said = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_UNCOUNTABLE(), &name.clone()))) {
    let _ = mcall(&said.clone(), "append", &[name.clone()]);
    } else {
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("potato"), py_str("tomato")]), &name.clone()))) {
    let _ = mcall(&said.clone(), "append", &[op_add(&name.clone(), &py_str("es"))]);
    } else {
    let _ = mcall(&said.clone(), "append", &[op_add(&name.clone(), &py_str("s"))]);
    }
    }
    }
    if py_truthy(&op_gt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&said.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&said.clone(), &op_neg(&Py::Int(1))));
    }
    return if py_truthy(&said.clone()) { geti(&said.clone(), &Py::Int(0)) } else { py_str("") };
    Py::None
}
pub fn _stick(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut allowed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let mut without: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: without"))) };
    let mut dish: Py = Py::None;
    let mut item: Py = Py::None;
    let mut name: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("The dish just offered, if it is what uses the ingredient they still have.");
    plan = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal_plan")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("meal_plan")]) } else { py_dict(vec![]) };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    name = __item.clone();
    item = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut dish: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&gattr(&dish.clone(), "name"), &name.clone()))) {  __out.push(dish.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_is(&item.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = op_bitand(&call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[gattr(&item.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { _fits(&[item.clone(), allowed.clone(), without.clone(), Py::None]) } } } }) {
    return item.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _without_dairy(args: &[Py]) -> Py {
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
    let mut without: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: without"))) };
    let mut extra: Py = Py::None;
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&without.clone(), &py_str("cheese"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&without.clone(), &py_str("milk"))) } }) {
    return text.clone();
    }
    let __iter1 = py_iter_or_raise(&g_DAIRY_EXTRAS());
    for __item in __iter1 {
    extra = __item.clone();
    text = mcall(&text.clone(), "replace", &[extra.clone(), py_str("")]);
    }
    return text.clone();
    Py::None
}
pub fn _times(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut how: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how"))) };
    let _ = py_str("The tail that fits the method: a total when the method already named its own cook time.");
    return if py_truthy(&mcall(&g_COOK_TIME(), "search", &[{ let __b = how.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { g_TOTAL_TIMES() } else { g_TIMES() };
    Py::None
}
pub fn _describe(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut item: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut without: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { py_tuple(vec![]) };
    let mut how: Py = Py::None;
    let mut last: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut openers: Py = Py::None;
    let mut option: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut when: Py = Py::None;
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = { let __b = if py_truthy(&turns.clone()) { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]) } else { py_str("") }; if py_truthy(&__b) { __b } else { py_str("") } };
    openers = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OPENERS()) { let mut option: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&mcall(&last.clone(), "startswith", &[option.clone()])))) {  __out.push(option.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { call_builtin("list", &[g_OPENERS()]).unwrap_or_else(|e| exc::raise(e)) } };
    opener = _variant(&[openers.clone(), understood.clone(), memory.clone()]);
    when = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("n".to_string(), gattr(&item.clone(), "minutes"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_variant(&[_times(&[gattr(&item.clone(), "how")]), understood.clone(), memory.clone()]), "format", args))(&__args) };
    how = _without_dairy(&[gattr(&item.clone(), "how"), without.clone()]);
    how = op_add(&mcall(&geti(&how.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&how.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __s = String::new(); __s.push_str(&fstr(&opener.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&gattr(&item.clone(), "said"), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&how.clone(), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&when.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn use_up(args: &[Py]) -> Py {
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
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut hit: Py = Py::None;
    let mut how: Py = Py::None;
    let mut idea: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut up: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What to do with an ingredient they already have cooked: \"leftover roast chicken\".\n\n    The written rows above say the most about the foods people ask about most; every other food\n    the kitchen knows is answered from its kind, with its own name in the first words, because a\n    use-up question that comes back as how to cook the thing from raw is the wrong question\n    answered well (\"leftover beetroot\" was an hour in the oven).\n    ");
    words = call_builtin("list", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&py_str(" "), "join", &[words.clone()]);
    said = Py::None;
    up = _up_food(&[words.clone()]);
    let __iter1 = py_iter_or_raise(&g_USE_UP());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    forms = __unpk2[0].clone();
    how = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&up.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&forms.clone(), &up.clone())) } }) {
    continue;
    }
    hit = { let __b = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&words.clone(), &form.clone()))) {  __out.push(form.clone()); } } py_list(__out) })() }; if py_truthy(&__b) { __b } else { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&text.clone(), &form.clone()))) {  __out.push(form.clone()); } } py_list(__out) })() } } };
    if py_truthy(&hit.clone()) {
    said = geti(&hit.clone(), &Py::Int(0));
    let _ = py_setitem(&memory.clone(), &py_str("meal_plan"), py_dict(vec![(py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("ingredients"), py_list(vec![])), (py_str("allowed"), Py::None), (py_str("without"), py_list(vec![])), (py_str("minutes"), Py::None), (py_str("offered"), py_list(vec![said.clone()]))]));
    return { let mut __s = String::new(); __s.push_str("Leftover "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&mcall(&geti(&how.clone(), &Py::Int(0)), "lower", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&how.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str("."); py_string(__s) };
    }
    }
    named = { let __b = up.clone(); if py_truthy(&__b) { __b } else { _named(&[understood.clone()]) } };
    if py_truthy(&named.clone()) {
    said = if py_truthy(&call_builtin("isinstance", &[named.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&named.clone(), &Py::Int(0)) } else { named.clone() };
    let mut cooking: Py = crate::knowledge::cooking::module_obj();
    idea = { let __b = mcall(&cooking.clone(), "pantry_idea", &[said.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_USE_DISH_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&idea.clone(), "lower", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    let _ = py_setitem(&memory.clone(), &py_str("meal_plan"), py_dict(vec![(py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("ingredients"), py_list(vec![])), (py_str("allowed"), Py::None), (py_str("without"), py_list(vec![])), (py_str("minutes"), Py::None), (py_str("offered"), py_list(vec![said.clone()]))]));
    return use_it_up(&[said.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _plan(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut allowed: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let mut without: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: without"))) };
    let mut minutes: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "minutes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: minutes"))) };
    let mut offered: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "offered") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: offered"))) };
    let mut earlier: Py = Py::None;
    let _ = py_str("Remember what was offered and the limits set, so the next turn can build on it.");
    earlier = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal_plan")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("meal_plan")]) } else { py_dict(vec![]) };
    let _ = py_setitem(&memory.clone(), &py_str("meal_plan"), py_dict(vec![(py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("ingredients"), { let __b = call_builtin("list", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("list", &[{ let __b = mcall(&earlier.clone(), "get", &[py_str("ingredients")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)) } }), (py_str("allowed"), if py_truthy(&Py::Bool(!py_is(&allowed.clone(), &Py::None))) { call_builtin("sorted", &[allowed.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&earlier.clone(), "get", &[py_str("allowed")]) }), (py_str("without"), call_builtin("sorted", &[without.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("minutes"), minutes.clone()), (py_str("offered"), slice_or_raise(&op_add(&call_builtin("list", &[{ let __b = mcall(&earlier.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[offered.clone()]).unwrap_or_else(|e| exc::raise(e))), &op_neg(&Py::Int(4)), &Py::None, &Py::None)), (py_str("pick"), geti(&{ let __b = call_builtin("list", &[offered.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_list(vec![{ let __b = mcall(&earlier.clone(), "get", &[py_str("pick")]); if py_truthy(&__b) { __b } else { py_str("") } }]) } }, &Py::Int(0)))]));
    Py::None
}
pub fn reads_as_news(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut cue: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when the line is news about a meal rather than a request for one.\n\n    \"maybe i will cook her favourite thing\", said about someone who has died, is news. A line\n    that asks what to cook is a request even when the reading files it as news, because the\n    asking words are in it (\"im trying to work out what to cook for six people\").\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("intent")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("news")))) {
    return Py::Bool(false);
    }
    low = mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_IN_NEWS()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn asks_for_a_meal(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut cue: Py = Py::None;
    let mut intent: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when this line is a request for a meal, not news about one.");
    understood = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    intent = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("intent")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    kind = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&reads_as_news(&[understood.clone(), clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&kind.clone(), &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("question"), py_str("request")]), &intent.clone())) } }) {
    return Py::Bool(true);
    }
    low = mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_CUES()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&low.clone(), "split", &[]), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn says_a_diet(args: &[Py]) -> Py {
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
    let mut cue: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when the line itself names a diet or an allergy.");
    low = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DIET_WORDS()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _carry(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A picnic or a day out: what to pack, not what to cook.");
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("make"), py_str("take"), py_str("bring"), py_str("pack"), py_str("packing"), py_str("food"), py_str("eat"), py_str("eating"), py_str("lunch"), py_str("idea"), py_str("ideas"), py_str("need"), py_str("snack"), py_str("snacks"), py_str("something"), py_str("anything"), py_str("for a picnic"), py_str("on a picnic"), py_str("day out"), py_str("veggie"), py_str("vegetarian"), py_str("vegan"), py_str("no meat"), py_str("gluten"), py_str("kid"), py_str("kids"), py_str("children"), py_str("child"), py_str("toddler"), py_str("people"), py_str("family"), py_str("friends")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("went to"), py_str("was at"), py_str("we had"), py_str("yesterday"), py_str("last week")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("flight"), py_str("flights"), py_str("plane"), py_str("airport"), py_str("airline"), py_str("cancelled"), py_str("canceled"), py_str("delayed"), py_str("delay"), py_str("refund"), py_str("owed"), py_str("compensation"), py_str("lighter"), py_str("luggage"), py_str("baggage"), py_str("security"), py_str("passport"), py_str("check in"), py_str("check-in")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("food"), py_str("eat"), py_str("eating"), py_str("lunch"), py_str("snack"), py_str("snacks"), py_str("picnic"), py_str("sandwich"), py_str("meal"), py_str("meals"), py_str("drink"), py_str("drinks"), py_str("breakfast"), py_str("dinner"), py_str("packed")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("packed lunch"), py_str("lunchbox"), py_str("lunch box"), py_str("school lunch"), py_str("bento")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("flight"), py_str("flights"), py_str("plane"), py_str("airport"), py_str("airline"), py_str("security")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("can i take"), py_str("can i bring"), py_str("can we take"), py_str("can we bring"), py_str("could i take"), py_str("could i bring"), py_str("am i allowed"), py_str("are we allowed"), py_str("allowed to take"), py_str("allowed to bring"), py_str("allowed on"), py_str("100ml"), py_str("liquids")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    reply = _variant(&[g_CARRY_REPLIES(), understood.clone(), memory.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("veggie"), py_str("vegetarian"), py_str("vegan"), py_str("no meat")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    reply = mcall(&mcall(&reply.clone(), "replace", &[py_str("cold sausages or sausage rolls, "), py_str("")]), "replace", &[py_str("sausage rolls, "), py_str("")]);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("kids"), py_str("children"), py_str("child"), py_str("toddler"), py_str("little ones")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug1 = op_add(&reply.clone(), &py_str(" For small people, add something they will actually eat: breadsticks, cheese."));
    reply = __aug1;
    }
    return reply.clone();
    Py::None
}
pub fn _kitchen_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let _ = py_str("cooking.py's answer for a kitchen with no oven, hob or microwave, or None.");
    let mut cooking: Py = crate::knowledge::cooking::module_obj();
    answer = mcall(&cooking.clone(), "without", &[understood.clone()]);
    return if py_truthy(&{ let __b = call_builtin("isinstance", &[answer.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&answer.clone(), "strip", &[]) } }) { answer.clone() } else { Py::None };
    Py::None
}
pub fn _said_about(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The line and the last few turns, lowercased, for cues that hold across turns.");
    said = py_list(vec![call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    let _ = mcall(&said.clone(), "append", &[mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    }
    }
    return mcall(&py_str(" "), "join", &[said.clone()]);
    Py::None
}
pub fn _diet_said(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut about: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut reads: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The diet word the person used about themselves, if they used one.");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &op_neg(&Py::Int(6)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&understood.clone(), "get", &[py_str("clean")]) } }) {
    let __aug1 = op_add(&said.clone(), &op_add(&py_str(" "), &mcall(&call_builtin("str", &[geti(&understood.clone(), &py_str("clean"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])));
    said = __aug1;
    }
    about = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("user")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("user")]) } else { py_dict(vec![]) };
    said = mcall(&op_add(&op_add(&mcall(&call_builtin("str", &[{ let __b = mcall(&about.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &py_str(" ")), &said.clone()), "strip", &[]);
    let __iter2 = py_iter_or_raise(&g_DIET_SAID());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    cue = __unpk3[0].clone();
    reads = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&said.clone(), &cue.clone()))) {
    return reads.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _fresh(args: &[Py]) -> Py {
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
    let mut item: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "item") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: item"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut head: Py = Py::None;
    let mut last: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("A sentence they were told a turn ago is not an answer.\n\n    A follow-up can land back on the idea just given (\"and how long does that take?\"), and\n    saying the same sentence again is no help; the clock is what is still unknown then.\n    ");
    turns = { let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    last = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("bot")]) } }) {
    last = call_builtin("str", &[geti(&turn.clone(), &py_str("bot"))]).unwrap_or_else(|e| exc::raise(e));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return reply.clone();
    }
    head = geti(&stdlib_call("re", "split", &[py_str("(?<=\\.)\\s+"), reply.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&last.clone(), &head.clone())) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&gattr(&item.clone(), "said"), "capitalize", &[]), -1, "")); __s.push_str(" takes about "); __s.push_str(&fstr(&gattr(&item.clone(), "minutes"), -1, "")); __s.push_str(" minutes."); py_string(__s) };
    }
    return reply.clone();
    Py::None
}
pub fn _crowd_answer(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("What cooking.py says for a meal for a table, or None when it has nothing.");
    let mut cooking: Py = crate::knowledge::cooking::module_obj();
    said = py_list(vec![call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    let _ = mcall(&said.clone(), "append", &[call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    text = mcall(&mcall(&py_str(" "), "join", &[said.clone()]), "lower", &[]);
    text = stdlib_call("re", "sub", &[py_str("\\bfor\\s+\\d{1,3}\\s*(?:min|mins|minute|minutes|hour|hours|sec|seconds)\\b"), py_str(" "), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    answer = mcall(&cooking.clone(), "_crowd_or_any", &[text.clone(), mcall(&cooking.clone(), "_excluded", &[text.clone()])]);
    return if py_truthy(&call_builtin("isinstance", &[answer.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { answer.clone() } else { Py::None };
    Py::None
}
pub fn _count_word(args: &[Py]) -> Py {
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
    let _ = py_str("The word for how many are eating, so the reply says \"four\" the way the line did.");
    return mcall(&g_COUNT_WORDS(), "get", &[call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), call_builtin("str", &[call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _for_a_table(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut counts: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut kids_named: Py = Py::None;
    let mut n: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How many people are eating, when the line or the last few turns said so.");
    said = py_list(vec![mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    let __iter1 = py_iter_or_raise(&slice_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &op_neg(&Py::Int(4)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("user")]) } }) {
    let _ = mcall(&said.clone(), "append", &[mcall(&call_builtin("str", &[geti(&turn.clone(), &py_str("user"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    }
    }
    joined = mcall(&py_str(" "), "join", &[said.clone()]);
    kids_named = call_builtin("bool", &[mcall(&g__AGE_SHAPE(), "search", &[joined.clone()])]).unwrap_or_else(|e| exc::raise(e));
    text = mcall(&g__AGE_SHAPE(), "sub", &[py_str(" "), joined.clone()]);
    counts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("\\bfor\\s+(\\d{1,3})\\b(?!\\s*(?:min|mins|minute|minutes|hour|hours|sec|seconds)\\b)"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut n: Py = __item.clone(); __out.push(call_builtin("int", &[n.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    let __iter2 = py_iter_or_raise(&mcall(&g_CROWD_SAID(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    word = __unpk3[0].clone();
    value = __unpk3[1].clone();
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\bfor\\s+"), &word.clone()), &py_str("\\b(?!\\s*(?:min|minute|hour))")), text.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &word.clone()), &py_str("\\s+(?:people|of us|adults|guests|coming)\\b")), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&counts.clone(), "append", &[value.clone()]);
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CROWD_ASK()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&counts.clone(), "append", &[Py::Int(6)]);
    }
    if py_truthy(&counts.clone()) {
    best = call_builtin("max", &[counts.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = kids_named.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\badults?\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return op_add(&best.clone(), &call_builtin("len", &[mcall(&g__AGE_SHAPE(), "findall", &[joined.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    }
    return best.clone();
    }
    return if py_truthy(&kids_named.clone()) { Py::Int(0) } else { Py::Int(0) };
    Py::None
}
pub fn _company(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut tail: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: tail"))) };
    let mut word: Py = Py::None;
    let _ = py_str("True when the words after \"with\" name the people eating, not ingredients.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&g_COMPANY(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COMPANY(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _unknown_ingredient(args: &[Py]) -> Py {
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
    let mut known: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line asks what to make with a food the table has never heard of.");
    let __iter1 = py_iter_or_raise(&mcall(&g_WITH_ASK(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    rest = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&match_.clone(), "group", &[Py::Int(1)])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_KITCHEN_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_AN_INGREDIENT(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&_company(&[rest.clone()])) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&known.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&known.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &mcall(&word.clone(), "rstrip", &[py_str("es")]))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _unknown_dish_recipe(args: &[Py]) -> Py {
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
    let mut known: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "known") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: known"))) };
    let mut match_: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True for \"a recipe for <dish>\" where the dish is not food the tables know.");
    match_ = mcall(&g_RECIPE_FOR(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    tail = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&match_.clone(), "group", &[Py::Int(1)])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_KITCHEN_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_A_DISH_NAME(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&tail.clone()))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&tail.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&known.clone(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&known.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &mcall(&word.clone(), "rstrip", &[py_str("es")]))) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn unknown_food(args: &[Py]) -> Py {
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
    let mut known_food: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names food the dish table has no row for.\n\n    The line is read as the understanding engine read it, repairs and all. Both readers below\n    match the line rather than its words (\"a recipe for <dish>\"), and read as typed the shape\n    slipped past every line with the slip inside the word \"recipe\" itself: \"i need a recipie for\n    shortbread\" was answered with ten-minute dinners while the shortbread card was waiting.\n    ");
    clean = mcall(&gen_understanding::understanding::line_as_read(&[understood.clone()]), "lower", &[]);
    known_food = op_bitor(&call_builtin("set", &[g_FOOD_WORDS()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[g_INGREDIENTS()]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&mcall(&g_INGREDIENTS(), "values", &[]));
    for __item in __iter1 {
    words = __item.clone();
    let _ = mcall(&known_food.clone(), "update", &[words.clone()]);
    }
    if py_truthy(&_unknown_ingredient(&[clean.clone(), known_food.clone()])) {
    return Py::Bool(true);
    }
    return _unknown_dish_recipe(&[clean.clone(), known_food.clone()]);
    Py::None
}
pub fn _something_it_has_not_got(args: &[Py]) -> Py {
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
    let mut allowed: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allowed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allowed"))) };
    let mut without: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "without") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: without"))) };
    let mut clean: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line's \"without X\" is a thing this table cannot read as a food or a diet.\n\n    The dishes know foods and diets. Anything else is the rest of the bot's business, and this\n    table answering it is how \"i dont have any money\" came back with a curry.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_is(&allowed.clone(), &Py::None)); if py_truthy(&__b) { __b } else { without.clone() } }) {
    return Py::Bool(false);
    }
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    match_ = mcall(&g_WITHOUT_WORD(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::Bool(false);
    }
    word = mcall(&mcall(&match_.clone(), "group", &[py_str("food")]), "lower", &[]);
    return { let __b = Py::Bool(!py_contains(&g_NOT_FOOD_LIMITS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FOOD_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_FOOD_WORDS(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } } } };
    Py::None
}
pub fn suggest(args: &[Py]) -> Py {
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
    let mut allowed: Py = Py::None;
    let mut alt: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut asked_for: Py = Py::None;
    let mut best: Py = Py::None;
    let mut carried: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut crowd: Py = Py::None;
    let mut cue: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut eaters: Py = Py::None;
    let mut fancying_a_shape: Py = Py::None;
    let mut first: Py = Py::None;
    let mut form: Py = Py::None;
    let mut forms: Py = Py::None;
    let mut fresh: Py = Py::None;
    let mut how: Py = Py::None;
    let mut hungry: Py = Py::None;
    let mut impress: Py = Py::None;
    let mut impress_ask: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kitchen: Py = Py::None;
    let mut known_here: Py = Py::None;
    let mut last: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut limit_follow: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut names_offered: Py = Py::None;
    let mut news: Py = Py::None;
    let mut off_limits: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut one: Py = Py::None;
    let mut opener: Py = Py::None;
    let mut options: Py = Py::None;
    let mut others: Py = Py::None;
    let mut out_of: Py = Py::None;
    let mut pick: Py = Py::None;
    let mut picks: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut ranked: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut ruled_now: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_words: Py = Py::None;
    let mut second: Py = Py::None;
    let mut shared: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut slower: Py = Py::None;
    let mut stick: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut thrifty: Py = Py::None;
    let mut wanting: Py = Py::None;
    let mut when: Py = Py::None;
    let mut without: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A dinner idea for this line: from what they named, within the limits they set.");
    if py_truthy(&crate::knowledge::lookup::says_it_has_gone_off(&[mcall(&understood.clone(), "get", &[py_str("clean")])])) {
    return Py::None;
    }
    if py_truthy(&crate::knowledge::dishes::a_dish_is_already_the_plan(&[call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return Py::None;
    }
    plan = if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal_plan")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("meal_plan")]) } else { py_dict(vec![]) };
    let __unpk1 = py_unpack(&_limits(&[understood.clone(), memory.clone()]), 3, None);
    allowed = __unpk1[0].clone();
    without = __unpk1[1].clone();
    minutes = __unpk1[2].clone();
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_named(&[understood.clone()])) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&without.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&_something_it_has_not_got(&[understood.clone(), allowed.clone(), without.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = names.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_IN_A_DISH(), &name.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_for_a_table(&[memory.clone(), understood.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAL_WORD()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHILD_AT_TABLE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    return crate::knowledge::dishes::g_KIDS_IDEA();
    }
    kitchen = _kitchen_answer(&[understood.clone()]);
    if py_truthy(&kitchen.clone()) {
    return kitchen.clone();
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BEFORE_EVENT()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EVENT_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SMALL_PEOPLE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAL_WORD()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NOT_A_MEAL_SUBJECT()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DESSERT_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TECHNIQUE_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_USE_UP_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_REPAIR()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MAKING_AT_HOME()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OTHER_PEOPLE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_UNWELL()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SICK_OF()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&unknown_food(&[understood.clone()])) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_GLUTEN(), &g_CONDITION())) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_said_about(&[memory.clone(), clean.clone()]), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    asked = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ASK_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    impress = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IMPRESS()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    impress_ask = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IMPRESS_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IMPRESS_FOR()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IMPRESS_WORTH()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    news = reads_as_news(&[understood.clone(), clean.clone()]);
    hungry = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HUNGRY()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    fancying_a_shape = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_STYLE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STYLE_CUES()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    wanting = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STYLE_WANT()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { fancying_a_shape.clone() } };
    out_of = { let __b = call_builtin("bool", &[_ruled_out(&[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&plan.clone(), "get", &[py_str("offered")])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = news.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&plan.clone(), "get", &[py_str("ingredients")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&hungry.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&wanting.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&out_of.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&impress_ask.clone())) } } } } } } } } } } } } } }) {
    return Py::None;
    }
    limit_follow = { let __b = Py::Bool(!py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&plan.clone(), "get", &[py_str("offered")])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = names.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&plan.clone(), "get", &[py_str("ingredients")]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&minutes.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = asked.clone(); if py_truthy(&__b) { __b } else { { let __b = limit_follow.clone(); if py_truthy(&__b) { __b } else { { let __b = wanting.clone(); if py_truthy(&__b) { __b } else { { let __b = out_of.clone(); if py_truthy(&__b) { __b } else { impress_ask.clone() } } } } } } } } } } } } } }))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("kind")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("thanks"))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = op_ge(&_for_a_table(&[memory.clone(), understood.clone()]), &Py::Int(6)); if !py_truthy(&__b) { __b } else { { let __b = asks_for_a_meal(&[understood.clone(), clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&says_a_diet(&[clean.clone()]))) } } } }) {
    crowd = _crowd_answer(&[memory.clone(), understood.clone()]);
    if py_truthy(&crowd.clone()) {
    return crowd.clone();
    }
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LUNCH_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_WITH_ASK(), "search", &[clean.clone()]) } }) {
    asked_for = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_WORD(), "findall", &[mcall(&mcall(&g_WITH_ASK(), "search", &[clean.clone()]), "group", &[Py::Int(1)])])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_KITCHEN_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NOT_AN_INGREDIENT(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&word.clone(), "isdigit", &[]))) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    known_here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_INGREDIENTS(), "values", &[])) { let mut forms: Py = __item.clone(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push(form.clone()); } } py_set(__out) })() };
    if py_truthy(&{ let __b = asked_for.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_company(&[asked_for.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked_for.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&known_here.clone(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known_here.clone(), &mcall(&word.clone(), "rstrip", &[py_str("s")]))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    }
    carried = Py::Bool(!py_truthy(&names.clone()));
    names = { let __b = names.clone(); if py_truthy(&__b) { __b } else { { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&plan.clone(), "get", &[py_str("ingredients")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&without.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() } } };
    if py_truthy(&{ let __b = carried.clone(); if !py_truthy(&__b) { __b } else { { let __b = names.clone(); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_STYLE()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STYLE_CUES()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&clean.clone(), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } }) {
    names = py_list(vec![]);
    }
    ruled_now = _ruled_out(&[call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    off_limits = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_named(&[understood.clone()])) { let mut name: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&ruled_now.clone(), &name.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&without.clone(), &name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISHES()) { let mut dish: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&gattr(&dish.clone(), "uses"), &name.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&allowed.clone(), &gattr(&dish.clone(), "diet"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } } } } } }) {  __out.push(name.clone()); } } py_list(__out) })() };
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&off_limits.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = off_limits.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&names.clone())) } }) {
    pick = geti(&_best(&[py_list(vec![]), allowed.clone(), without.clone(), minutes.clone(), memory.clone(), understood.clone()]), &Py::Int(0));
    let _ = _plan(&[memory.clone(), py_list(vec![]), allowed.clone(), without.clone(), minutes.clone(), py_list(vec![gattr(&pick.clone(), "name")])]);
    reason = _diet_said(&[memory.clone()]);
    lead = if py_truthy(&reason.clone()) { { let mut __s = String::new(); __s.push_str("Since you are "); __s.push_str(&fstr(&reason.clone(), -1, "")); __s.push_str(", I'd leave that out"); py_string(__s) } } else { py_str("I'd leave that out") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&lead.clone(), -1, "")); __s.push_str(" and do "); __s.push_str(&fstr(&gattr(&pick.clone(), "said"), -1, "")); __s.push_str(" ("); __s.push_str(&fstr(&gattr(&pick.clone(), "minutes"), -1, "")); __s.push_str(" minutes): "); __s.push_str(&fstr(&gattr(&pick.clone(), "how"), -1, "")); __s.push_str("."); py_string(__s) };
    }
    ranked = _best(&[names.clone(), allowed.clone(), without.clone(), minutes.clone(), memory.clone(), understood.clone()]);
    offered = call_builtin("list", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    fresh = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut dish: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&offered.clone(), &gattr(&dish.clone(), "name")))) {  __out.push(dish.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&fresh.clone())); if !py_truthy(&__b) { __b } else { { let __b = names.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&minutes.clone(), &Py::None)) } } } }) {
    best = geti(&ranked.clone(), &Py::Int(0));
    let _ = _plan(&[memory.clone(), names.clone(), allowed.clone(), without.clone(), minutes.clone(), py_list(vec![gattr(&best.clone(), "name")])]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&gattr(&best.clone(), "said"), "capitalize", &[]), -1, "")); __s.push_str(" is the quickest I know with "); __s.push_str(&fstr(&_prose(&[names.clone()]), -1, "")); __s.push_str(", at "); __s.push_str(&fstr(&gattr(&best.clone(), "minutes"), -1, "")); __s.push_str(" minutes; that is every dish I have for them."); py_string(__s) };
    }
    ranked = { let __b = fresh.clone(); if py_truthy(&__b) { __b } else { ranked.clone() } };
    if py_truthy(&{ let __b = impress.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&names.clone())) } }) {
    slower = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut dish: Py = __item.clone(); if py_truthy(&op_ge(&gattr(&dish.clone(), "minutes"), &Py::Int(30))) {  __out.push(dish.clone()); } } py_list(__out) })() };
    ranked = { let __b = slower.clone(); if py_truthy(&__b) { __b } else { ranked.clone() } };
    shown = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ranked.clone()) { let mut dish: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_SHOW_OFF(), &gattr(&dish.clone(), "name")))) {  __out.push(dish.clone()); } } py_list(__out) })() };
    ranked = { let __b = shown.clone(); if py_truthy(&__b) { __b } else { ranked.clone() } };
    }
    first = geti(&ranked.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = names.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&op_bitand(&call_builtin("set", &[gattr(&first.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e))))); if py_truthy(&__b) { __b } else { op_gt(&gattr(&first.clone(), "minutes"), &op_add(&minutes.clone(), &g_GRACE())) } } } } } }) {
    stick = _stick(&[memory.clone(), names.clone(), allowed.clone(), without.clone()]);
    if py_truthy(&Py::Bool(!py_is(&stick.clone(), &Py::None))) {
    how = op_add(&mcall(&geti(&gattr(&stick.clone(), "how"), &Py::Int(0)), "upper", &[]), &slice_or_raise(&gattr(&stick.clone(), "how"), &Py::Int(1), &Py::None, &Py::None));
    when = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("n".to_string(), gattr(&stick.clone(), "minutes"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_variant(&[_times(&[gattr(&stick.clone(), "how")]), understood.clone(), memory.clone()]), "format", args))(&__args) };
    let _ = _plan(&[memory.clone(), names.clone(), allowed.clone(), without.clone(), minutes.clone(), py_list(vec![gattr(&stick.clone(), "name")])]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&gattr(&stick.clone(), "said"), "capitalize", &[]), -1, "")); __s.push_str(" is still the quickest I know with "); __s.push_str(&fstr(&_prose(&[names.clone()]), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&gattr(&stick.clone(), "how"), -1, "")); __s.push_str(". "); __s.push_str(&fstr(&when.clone(), -1, "")); py_string(__s) };
    }
    }
    second = if py_truthy(&op_gt(&call_builtin("len", &[ranked.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) { geti(&ranked.clone(), &Py::Int(1)) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&without.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&allowed.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&minutes.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&impress.clone())) } } } } } } } }) {
    picks = _spread(&[ranked.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[picks.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    names_offered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&picks.clone()) { let mut item: Py = __item.clone(); __out.push(gattr(&item.clone(), "name")); } py_list(__out) })() };
    let _ = _plan(&[memory.clone(), names.clone(), allowed.clone(), without.clone(), minutes.clone(), names_offered.clone()]);
    said = mcall(&py_str(", "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&picks.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut item: Py = __item.clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&gattr(&item.clone(), "said"), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&gattr(&item.clone(), "minutes"), -1, "")); py_string(__s) }); } py_list(__out) })() }]);
    last = geti(&picks.clone(), &op_neg(&Py::Int(1)));
    said_words = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    thrifty = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHEAP_ASK()) { let mut cue: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_said_about(&[memory.clone(), said_words.clone()]), &cue.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    eaters = _for_a_table(&[memory.clone(), understood.clone()]);
    if py_truthy(&{ let __b = thrifty.clone(); if !py_truthy(&__b) { __b } else { { let __cmp21: Py = eaters.clone(); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &Py::Int(12)) } else { Py::Bool(false) } } } }) {
    opener = { let mut __s = String::new(); __s.push_str("Cheap, and enough for "); __s.push_str(&fstr(&_count_word(&[eaters.clone()]), -1, "")); __s.push_str(":"); py_string(__s) };
    } else {
    if py_truthy(&thrifty.clone()) {
    opener = py_str("Cheap:");
    } else {
    opener = py_str("Quick:");
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&pyrt::xrefs::xcall("compose.fit", "wanted", &[memory.clone()]), &py_str("pick"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&thrifty.clone())) } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&picks.clone(), &Py::Int(0)), slice_or_raise(&picks.clone(), &Py::Int(1), &Py::None, &Py::None)]), 2, None);
    one = __unpk3[0].clone();
    rest = __unpk3[1].clone();
    others = mcall(&py_str(""), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&rest.clone()) { let mut item: Py = __item.clone(); __out.push({ let mut __s = String::new(); __s.push_str(", or "); __s.push_str(&fstr(&gattr(&item.clone(), "said"), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&gattr(&item.clone(), "minutes"), -1, "")); py_string(__s) }); } py_list(__out) })() }]);
    return { let mut __s = String::new(); __s.push_str("Here's my pick: "); __s.push_str(&fstr(&gattr(&one.clone(), "said"), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&gattr(&one.clone(), "minutes"), -1, "")); __s.push_str(" minutes"); __s.push_str(&fstr(&others.clone(), -1, "")); __s.push_str(". Tell me what is in the fridge and I will narrow it down."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&opener.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(", or "); __s.push_str(&fstr(&gattr(&last.clone(), "said"), -1, "")); __s.push_str(" if you have "); __s.push_str(&fstr(&gattr(&last.clone(), "minutes"), -1, "")); __s.push_str(" minutes. Tell me what is in the fridge and I will pick properly."); py_string(__s) };
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&names.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&second.clone(), &Py::None)) } }) {
    let _ = _plan(&[memory.clone(), names.clone(), allowed.clone(), without.clone(), minutes.clone(), py_list(vec![gattr(&first.clone(), "name"), gattr(&second.clone(), "name")])]);
    if py_truthy(&op_lt(&gattr(&second.clone(), "minutes"), &op_sub(&gattr(&first.clone(), "minutes"), &g_GRACE()))) {
    tail = { let mut __s = String::new(); __s.push_str("If you want something quicker, "); __s.push_str(&fstr(&gattr(&second.clone(), "said"), -1, "")); __s.push_str(" takes "); __s.push_str(&fstr(&gattr(&second.clone(), "minutes"), -1, "")); __s.push_str(" minutes."); py_string(__s) };
    } else {
    if py_truthy(&op_le(&call_builtin("abs", &[op_sub(&gattr(&second.clone(), "minutes"), &gattr(&first.clone(), "minutes"))]).unwrap_or_else(|e| exc::raise(e)), &g_GRACE())) {
    tail = { let mut __s = String::new(); __s.push_str("Something else that works is "); __s.push_str(&fstr(&gattr(&second.clone(), "said"), -1, "")); __s.push_str("."); py_string(__s) };
    } else {
    tail = { let mut __s = String::new(); __s.push_str("Something else that works is "); __s.push_str(&fstr(&gattr(&second.clone(), "said"), -1, "")); __s.push_str(", at "); __s.push_str(&fstr(&gattr(&second.clone(), "minutes"), -1, "")); __s.push_str(" minutes."); py_string(__s) };
    }
    }
    return op_add(&{ let mut __s = String::new(); __s.push_str("I'd do "); __s.push_str(&fstr(&gattr(&first.clone(), "said"), -1, "")); __s.push_str(" ("); __s.push_str(&fstr(&gattr(&first.clone(), "minutes"), -1, "")); __s.push_str(" minutes): "); __s.push_str(&fstr(&gattr(&first.clone(), "how"), -1, "")); __s.push_str(". "); py_string(__s) }, &tail.clone());
    }
    reply = _describe(&[first.clone(), understood.clone(), memory.clone(), without.clone()]);
    shared = if py_truthy(&Py::Bool(!py_is(&second.clone(), &Py::None))) { call_builtin("len", &[op_bitand(&call_builtin("set", &[gattr(&second.clone(), "uses")]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } else { Py::Int(0) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&second.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = names.clone(); if !py_truthy(&__b) { __b } else { { let __b = op_ge(&shared.clone(), &call_builtin("min", &[Py::Int(2), call_builtin("len", &[names.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { op_le(&gattr(&second.clone(), "minutes"), &call_builtin("max", &[Py::Int(25), op_add(&gattr(&first.clone(), "minutes"), &Py::Int(10))]).unwrap_or_else(|e| exc::raise(e))) } } } } } }) {
    options = g_FOLLOW_ON();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&without.clone(), &py_str("cheese"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&gattr(&second.clone(), "uses"), &py_str("cheese"))) } }) {
    options = op_add(&g_FOLLOW_ON(), &py_tuple(vec![g_FOLLOW_ON_NO_CHEESE()]));
    }
    if py_truthy(&op_le(&call_builtin("abs", &[op_sub(&gattr(&second.clone(), "minutes"), &gattr(&first.clone(), "minutes"))]).unwrap_or_else(|e| exc::raise(e)), &g_GRACE())) {
    alt = _variant(&[py_tuple(vec![py_str("Something else that works is {alt}."), py_str("There is {alt} as well.")]), understood.clone(), memory.clone()]);
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("alt".to_string(), gattr(&second.clone(), "said"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&alt.clone(), "format", args))(&__args) };
    } else {
    alt = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("alt".to_string(), gattr(&second.clone(), "said")), ("n".to_string(), gattr(&second.clone(), "minutes"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_variant(&[options.clone(), understood.clone(), memory.clone()]), "format", args))(&__args) };
    }
    reply = _fresh(&[{ let mut __s = String::new(); __s.push_str(&fstr(&reply.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&alt.clone(), -1, "")); py_string(__s) }, first.clone(), memory.clone()]);
    }
    let _ = _plan(&[memory.clone(), names.clone(), allowed.clone(), without.clone(), minutes.clone(), op_add(&py_list(vec![gattr(&first.clone(), "name")]), &if py_truthy(&second.clone()) { py_list(vec![gattr(&second.clone(), "name")]) } else { py_list(vec![]) })]);
    return _fresh(&[reply.clone(), first.clone(), memory.clone()]);
    Py::None
}
pub fn _takeout_pick(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("What to order at the counter the line names: the most specific name wins.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, op_neg(&Py::Int(1))]), 2, None);
    best = __unpk1[0].clone();
    longest = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g_TAKEOUT_PICKS());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    names = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &longest.clone()); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![reply.clone(), call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    best = __unpk5[0].clone();
    longest = __unpk5[1].clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _counters_named(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut index: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut rows: Py = Py::None;
    let _ = py_str("Which rows of the table above the line names, by row: \"bakery\" and \"baker\" are one row.");
    rows = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[g_TAKEOUT_PICKS()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    names = __unpk3[0].clone();
    _u = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&rows.clone(), "add", &[index.clone()]);
    break;
    }
    }
    }
    return rows.clone();
    Py::None
}
pub fn _one_counter(args: &[Py]) -> Py {
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
    let mut shape: Py = Py::None;
    let _ = py_str("True when the line asks what to order at one counter named in it.");
    clean = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "match", &[py_str("^(?:is|are|was|were)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    shape = mcall(&g_DEFINITION_SHAPE(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = shape.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ASKING_WORDS(), &mcall(&shape.clone(), "group", &[Py::Int(1)]))) } }) {
    return Py::Bool(false);
    }
    return op_le(&call_builtin("len", &[_counters_named(&[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _not_a_mark_and_not_a_row(args: &[Py]) -> Py {
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
    let _ = py_str("A mark on something and a row that got heated are both not food in the kitchen.\n\n    \"i got egg on my shirt\" is not an ingredient (the guard below), and \"things got heated at\n    dinner\" is an argument, not a dinner: it was answered \"Quick: tuna pasta in 15\".\n    ");
    return { let __b = crate::knowledge::cleaning::not_a_mark_on_a_thing(&[understood.clone()]); if !py_truthy(&__b) { __b } else { crate::knowledge::lookup::not_the_heat_of_an_argument(&[understood.clone()]) } };
    Py::None
}
pub fn Dish____init__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut uses: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "uses") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: uses"))) };
    let mut minutes: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "minutes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: minutes"))) };
    let mut diet: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "diet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: diet"))) };
    let mut how: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "how") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: how"))) };
    let _ = py_setattr(&self_.clone(), "name", name.clone());
    let _ = py_setattr(&self_.clone(), "said", mcall(&g_SAID_AS(), "get", &[name.clone(), name.clone()]));
    let _ = py_setattr(&self_.clone(), "uses", call_builtin("tuple", &[uses.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setattr(&self_.clone(), "minutes", minutes.clone());
    let _ = py_setattr(&self_.clone(), "diet", diet.clone());
    let _ = py_setattr(&self_.clone(), "how", how.clone());
    let _ = py_setattr(&self_.clone(), "favourite", Py::Bool(py_contains(&g_FAVOURITES(), &name.clone())));
    Py::None
}
pub fn Dish____repr__(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut self_: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "self") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: self"))) };
    return { let mut __s = String::new(); __s.push_str("<Dish "); __s.push_str(&fstr(&gattr(&self_.clone(), "name"), -1, "")); __s.push_str(">"); py_string(__s) };
    Py::None
}
pub fn Dish(args: &[Py]) -> Py {
    ensure_init();
    let inst = Py::Instance(Rc::new(PyInstance { class_name: "Dish".to_string(), attrs: Rc::new(RefCell::new(IndexMap::new())) }));
    let mut full = vec![inst.clone()]; full.extend_from_slice(args);
    Dish____init__(&full);
    inst
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.meals".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACK_WORDS" => Some(g_ACK_WORDS()),
            "AILING_PLANT" => Some(g_AILING_PLANT()),
            "ANOTHER_MEAL" => Some(g_ANOTHER_MEAL()),
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKING_WORDS" => Some(g_ASKING_WORDS()),
            "ASK_CUES" => Some(g_ASK_CUES()),
            "ASK_IN_NEWS" => Some(g_ASK_IN_NEWS()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "BEDTIME" => Some(g_BEDTIME()),
            "BEFORE_EVENT" => Some(g_BEFORE_EVENT()),
            "BRING_NOT_FOOD" => Some(g_BRING_NOT_FOOD()),
            "BROKEN" => Some(g_BROKEN()),
            "CARRY_ASK" => Some(g_CARRY_ASK()),
            "CARRY_REPLIES" => Some(g_CARRY_REPLIES()),
            "CHEAP" => Some(g_CHEAP()),
            "CHEAP_ASK" => Some(g_CHEAP_ASK()),
            "CHEAP_SET" => Some(g_CHEAP_SET()),
            "CHILD_AT_TABLE" => Some(g_CHILD_AT_TABLE()),
            "COMPANY" => Some(g_COMPANY()),
            "CONDITION" => Some(g_CONDITION()),
            "COOK_TIME" => Some(g_COOK_TIME()),
            "COUNT_WORDS" => Some(g_COUNT_WORDS()),
            "CROWD_ASK" => Some(g_CROWD_ASK()),
            "CROWD_SAID" => Some(g_CROWD_SAID()),
            "DAIRY_EXTRAS" => Some(g_DAIRY_EXTRAS()),
            "DEFINITION_SHAPE" => Some(g_DEFINITION_SHAPE()),
            "DESSERT_ASK" => Some(g_DESSERT_ASK()),
            "DIET_SAID" => Some(g_DIET_SAID()),
            "DIET_WORDS" => Some(g_DIET_WORDS()),
            "DISHES" => Some(g_DISHES()),
            "DISH_SHAPES" => Some(g_DISH_SHAPES()),
            "DISH_STYLE" => Some(g_DISH_STYLE()),
            "DOUBLE_NO" => Some(g_DOUBLE_NO()),
            "DRINK" => Some(g_DRINK()),
            "Dish" => Some(py_func("Dish", |args: &[Py]| Dish(args))),
            "EMPTY" => Some(g_EMPTY()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EVENT_WORDS" => Some(g_EVENT_WORDS()),
            "FAVOURITES" => Some(g_FAVOURITES()),
            "FISH_INGREDIENTS" => Some(g_FISH_INGREDIENTS()),
            "FOLLOW_ON" => Some(g_FOLLOW_ON()),
            "FOLLOW_ON_NO_CHEESE" => Some(g_FOLLOW_ON_NO_CHEESE()),
            "FOOD_WORDS" => Some(g_FOOD_WORDS()),
            "GAME_WORDS" => Some(g_GAME_WORDS()),
            "GLUTEN" => Some(g_GLUTEN()),
            "GRACE" => Some(g_GRACE()),
            "HARD_CLOCK" => Some(g_HARD_CLOCK()),
            "HEALTH" => Some(g_HEALTH()),
            "HOW_TO" => Some(g_HOW_TO()),
            "HUNGRY" => Some(g_HUNGRY()),
            "IDEA_ASK" => Some(g_IDEA_ASK()),
            "IMPRESS" => Some(g_IMPRESS()),
            "IMPRESS_ASK" => Some(g_IMPRESS_ASK()),
            "IMPRESS_FOR" => Some(g_IMPRESS_FOR()),
            "IMPRESS_WORTH" => Some(g_IMPRESS_WORTH()),
            "INGREDIENTS" => Some(g_INGREDIENTS()),
            "IN_A_DISH" => Some(g_IN_A_DISH()),
            "KIDS_IDEA" => Some(g_KIDS_IDEA()),
            "KITCHEN_WORDS" => Some(g_KITCHEN_WORDS()),
            "LIGHT_ASK" => Some(g_LIGHT_ASK()),
            "LIGHT_SET" => Some(g_LIGHT_SET()),
            "LUNCH_ASK" => Some(g_LUNCH_ASK()),
            "MAKING_AT_HOME" => Some(g_MAKING_AT_HOME()),
            "MEAL_WORD" => Some(g_MEAL_WORD()),
            "MEASURING" => Some(g_MEASURING()),
            "MONEY_MATTERS" => Some(g_MONEY_MATTERS()),
            "NOT_AN_IDEA" => Some(g_NOT_AN_IDEA()),
            "NOT_AN_INGREDIENT" => Some(g_NOT_AN_INGREDIENT()),
            "NOT_A_DISH" => Some(g_NOT_A_DISH()),
            "NOT_A_DISH_NAME" => Some(g_NOT_A_DISH_NAME()),
            "NOT_A_MEAL" => Some(g_NOT_A_MEAL()),
            "NOT_A_MEAL_SUBJECT" => Some(g_NOT_A_MEAL_SUBJECT()),
            "NOT_FOOD_LIMITS" => Some(g_NOT_FOOD_LIMITS()),
            "NOT_FOR_ME" => Some(g_NOT_FOR_ME()),
            "NOT_MINE" => Some(g_NOT_MINE()),
            "NOT_MINE_ASK" => Some(g_NOT_MINE_ASK()),
            "NOT_MINE_CHEAP" => Some(g_NOT_MINE_CHEAP()),
            "NOT_MINE_ENTRY" => Some(g_NOT_MINE_ENTRY()),
            "NOT_MINE_OUT" => Some(g_NOT_MINE_OUT()),
            "NOT_MY_KITCHEN" => Some(g_NOT_MY_KITCHEN()),
            "NO_CHEESE" => Some(g_NO_CHEESE()),
            "NO_CUE" => Some(g_NO_CUE()),
            "NO_DAIRY" => Some(g_NO_DAIRY()),
            "NO_GAP" => Some(g_NO_GAP()),
            "NO_HAVE" => Some(g_NO_HAVE()),
            "NO_HAVE_GAP" => Some(g_NO_HAVE_GAP()),
            "NO_MEAT" => Some(g_NO_MEAT()),
            "OPENERS" => Some(g_OPENERS()),
            "OTHER_PEOPLE" => Some(g_OTHER_PEOPLE()),
            "OWN_MEAL" => Some(g_OWN_MEAL()),
            "QUICK" => Some(g_QUICK()),
            "RECIPE_FOR" => Some(g_RECIPE_FOR()),
            "REPAIR" => Some(g_REPAIR()),
            "SAID_AS" => Some(g_SAID_AS()),
            "SAYING" => Some(g_SAYING()),
            "SHOW_OFF" => Some(g_SHOW_OFF()),
            "SICK_OF" => Some(g_SICK_OF()),
            "SLEEP" => Some(g_SLEEP()),
            "SMALL_PEOPLE" => Some(g_SMALL_PEOPLE()),
            "SOUP_NAMED" => Some(g_SOUP_NAMED()),
            "SPOILT_FOOD" => Some(g_SPOILT_FOOD()),
            "STAPLE" => Some(g_STAPLE()),
            "STYLE_CUES" => Some(g_STYLE_CUES()),
            "STYLE_WANT" => Some(g_STYLE_WANT()),
            "TAKEOUT_ASK" => Some(g_TAKEOUT_ASK()),
            "TAKEOUT_PICKS" => Some(g_TAKEOUT_PICKS()),
            "TAKEOUT_PLACES" => Some(g_TAKEOUT_PLACES()),
            "TECHNIQUE" => Some(g_TECHNIQUE()),
            "TECHNIQUE_ASK" => Some(g_TECHNIQUE_ASK()),
            "TIMES" => Some(g_TIMES()),
            "TOTAL_TIMES" => Some(g_TOTAL_TIMES()),
            "UNCOUNTABLE" => Some(g_UNCOUNTABLE()),
            "UNWELL" => Some(g_UNWELL()),
            "UP_ALLIUMS" => Some(g_UP_ALLIUMS()),
            "UP_CLASSES" => Some(g_UP_CLASSES()),
            "UP_FISH" => Some(g_UP_FISH()),
            "UP_FOODS" => Some(g_UP_FOODS()),
            "UP_FRUIT" => Some(g_UP_FRUIT()),
            "UP_GRAINS" => Some(g_UP_GRAINS()),
            "UP_GREENS" => Some(g_UP_GREENS()),
            "UP_MEAT" => Some(g_UP_MEAT()),
            "UP_ROOTS" => Some(g_UP_ROOTS()),
            "UP_SOFT" => Some(g_UP_SOFT()),
            "USE_DISH_WORDS" => Some(g_USE_DISH_WORDS()),
            "USE_UP" => Some(g_USE_UP()),
            "USE_UP_ASK" => Some(g_USE_UP_ASK()),
            "USE_UP_TECHNIQUE" => Some(g_USE_UP_TECHNIQUE()),
            "WHAT_GOES" => Some(g_WHAT_GOES()),
            "WITHOUT_WORD" => Some(g_WITHOUT_WORD()),
            "WITH_ASK" => Some(g_WITH_ASK()),
            "WORD" => Some(g_WORD()),
            "_AGE_SHAPE" => Some(g__AGE_SHAPE()),
            "_NO_MEAT_BLOCKER" => Some(g__NO_MEAT_BLOCKER()),
            "_UP_FOOD_SET" => Some(g__UP_FOOD_SET()),
            "_best" => Some(py_func("_best", |args: &[Py]| _best(args))),
            "_carry" => Some(py_func("_carry", |args: &[Py]| _carry(args))),
            "_clock" => Some(py_func("_clock", |args: &[Py]| _clock(args))),
            "_company" => Some(py_func("_company", |args: &[Py]| _company(args))),
            "_count_word" => Some(py_func("_count_word", |args: &[Py]| _count_word(args))),
            "_counters_named" => Some(py_func("_counters_named", |args: &[Py]| _counters_named(args))),
            "_crowd_answer" => Some(py_func("_crowd_answer", |args: &[Py]| _crowd_answer(args))),
            "_describe" => Some(py_func("_describe", |args: &[Py]| _describe(args))),
            "_diet_said" => Some(py_func("_diet_said", |args: &[Py]| _diet_said(args))),
            "_fits" => Some(py_func("_fits", |args: &[Py]| _fits(args))),
            "_for_a_table" => Some(py_func("_for_a_table", |args: &[Py]| _for_a_table(args))),
            "_fresh" => Some(py_func("_fresh", |args: &[Py]| _fresh(args))),
            "_is_the_ruled_out_food" => Some(py_func("_is_the_ruled_out_food", |args: &[Py]| _is_the_ruled_out_food(args))),
            "_kitchen_answer" => Some(py_func("_kitchen_answer", |args: &[Py]| _kitchen_answer(args))),
            "_later" => Some(py_func("_later", |args: &[Py]| _later(args))),
            "_limits" => Some(py_func("_limits", |args: &[Py]| _limits(args))),
            "_named" => Some(py_func("_named", |args: &[Py]| _named(args))),
            "_no_have" => Some(g__no_have()),
            "_no_meat" => Some(py_func("_no_meat", |args: &[Py]| _no_meat(args))),
            "_not_a_mark_and_not_a_row" => Some(py_func("_not_a_mark_and_not_a_row", |args: &[Py]| _not_a_mark_and_not_a_row(args))),
            "_one_counter" => Some(py_func("_one_counter", |args: &[Py]| _one_counter(args))),
            "_plan" => Some(py_func("_plan", |args: &[Py]| _plan(args))),
            "_prose" => Some(py_func("_prose", |args: &[Py]| _prose(args))),
            "_rotate" => Some(py_func("_rotate", |args: &[Py]| _rotate(args))),
            "_row" => Some(g__row()),
            "_ruled_out" => Some(py_func("_ruled_out", |args: &[Py]| _ruled_out(args))),
            "_said_about" => Some(py_func("_said_about", |args: &[Py]| _said_about(args))),
            "_shape" => Some(py_func("_shape", |args: &[Py]| _shape(args))),
            "_something_it_has_not_got" => Some(py_func("_something_it_has_not_got", |args: &[Py]| _something_it_has_not_got(args))),
            "_spread" => Some(py_func("_spread", |args: &[Py]| _spread(args))),
            "_stick" => Some(py_func("_stick", |args: &[Py]| _stick(args))),
            "_takeout_pick" => Some(py_func("_takeout_pick", |args: &[Py]| _takeout_pick(args))),
            "_the_fish_are_out" => Some(py_func("_the_fish_are_out", |args: &[Py]| _the_fish_are_out(args))),
            "_times" => Some(py_func("_times", |args: &[Py]| _times(args))),
            "_unknown_dish_recipe" => Some(py_func("_unknown_dish_recipe", |args: &[Py]| _unknown_dish_recipe(args))),
            "_unknown_ingredient" => Some(py_func("_unknown_ingredient", |args: &[Py]| _unknown_ingredient(args))),
            "_up_food" => Some(py_func("_up_food", |args: &[Py]| _up_food(args))),
            "_variant" => Some(py_func("_variant", |args: &[Py]| _variant(args))),
            "_without_dairy" => Some(py_func("_without_dairy", |args: &[Py]| _without_dairy(args))),
            "asks_for_a_meal" => Some(py_func("asks_for_a_meal", |args: &[Py]| asks_for_a_meal(args))),
            "cheap" => Some(py_func("cheap", |args: &[Py]| cheap(args))),
            "idea_is_a_dish" => Some(py_func("idea_is_a_dish", |args: &[Py]| idea_is_a_dish(args))),
            "named_food" => Some(py_func("named_food", |args: &[Py]| named_food(args))),
            "reads_as_news" => Some(py_func("reads_as_news", |args: &[Py]| reads_as_news(args))),
            "says_a_diet" => Some(py_func("says_a_diet", |args: &[Py]| says_a_diet(args))),
            "suggest" => Some(py_func("suggest", |args: &[Py]| suggest(args))),
            "unknown_food" => Some(py_func("unknown_food", |args: &[Py]| unknown_food(args))),
            "use_it_up" => Some(py_func("use_it_up", |args: &[Py]| use_it_up(args))),
            "use_up" => Some(py_func("use_up", |args: &[Py]| use_up(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ACK_WORDS" => s_ACK_WORDS(v),
            "AILING_PLANT" => s_AILING_PLANT(v),
            "ANOTHER_MEAL" => s_ANOTHER_MEAL(v),
            "ANSWERS" => s_ANSWERS(v),
            "ASKING_WORDS" => s_ASKING_WORDS(v),
            "ASK_CUES" => s_ASK_CUES(v),
            "ASK_IN_NEWS" => s_ASK_IN_NEWS(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "BEDTIME" => s_BEDTIME(v),
            "BEFORE_EVENT" => s_BEFORE_EVENT(v),
            "BRING_NOT_FOOD" => s_BRING_NOT_FOOD(v),
            "BROKEN" => s_BROKEN(v),
            "CARRY_ASK" => s_CARRY_ASK(v),
            "CARRY_REPLIES" => s_CARRY_REPLIES(v),
            "CHEAP" => s_CHEAP(v),
            "CHEAP_ASK" => s_CHEAP_ASK(v),
            "CHEAP_SET" => s_CHEAP_SET(v),
            "CHILD_AT_TABLE" => s_CHILD_AT_TABLE(v),
            "COMPANY" => s_COMPANY(v),
            "CONDITION" => s_CONDITION(v),
            "COOK_TIME" => s_COOK_TIME(v),
            "COUNT_WORDS" => s_COUNT_WORDS(v),
            "CROWD_ASK" => s_CROWD_ASK(v),
            "CROWD_SAID" => s_CROWD_SAID(v),
            "DAIRY_EXTRAS" => s_DAIRY_EXTRAS(v),
            "DEFINITION_SHAPE" => s_DEFINITION_SHAPE(v),
            "DESSERT_ASK" => s_DESSERT_ASK(v),
            "DIET_SAID" => s_DIET_SAID(v),
            "DIET_WORDS" => s_DIET_WORDS(v),
            "DISHES" => s_DISHES(v),
            "DISH_SHAPES" => s_DISH_SHAPES(v),
            "DISH_STYLE" => s_DISH_STYLE(v),
            "DOUBLE_NO" => s_DOUBLE_NO(v),
            "DRINK" => s_DRINK(v),
            "EMPTY" => s_EMPTY(v),
            "ENTRIES" => s_ENTRIES(v),
            "EVENT_WORDS" => s_EVENT_WORDS(v),
            "FAVOURITES" => s_FAVOURITES(v),
            "FISH_INGREDIENTS" => s_FISH_INGREDIENTS(v),
            "FOLLOW_ON" => s_FOLLOW_ON(v),
            "FOLLOW_ON_NO_CHEESE" => s_FOLLOW_ON_NO_CHEESE(v),
            "FOOD_WORDS" => s_FOOD_WORDS(v),
            "GAME_WORDS" => s_GAME_WORDS(v),
            "GLUTEN" => s_GLUTEN(v),
            "GRACE" => s_GRACE(v),
            "HARD_CLOCK" => s_HARD_CLOCK(v),
            "HEALTH" => s_HEALTH(v),
            "HOW_TO" => s_HOW_TO(v),
            "HUNGRY" => s_HUNGRY(v),
            "IDEA_ASK" => s_IDEA_ASK(v),
            "IMPRESS" => s_IMPRESS(v),
            "IMPRESS_ASK" => s_IMPRESS_ASK(v),
            "IMPRESS_FOR" => s_IMPRESS_FOR(v),
            "IMPRESS_WORTH" => s_IMPRESS_WORTH(v),
            "INGREDIENTS" => s_INGREDIENTS(v),
            "IN_A_DISH" => s_IN_A_DISH(v),
            "KIDS_IDEA" => s_KIDS_IDEA(v),
            "KITCHEN_WORDS" => s_KITCHEN_WORDS(v),
            "LIGHT_ASK" => s_LIGHT_ASK(v),
            "LIGHT_SET" => s_LIGHT_SET(v),
            "LUNCH_ASK" => s_LUNCH_ASK(v),
            "MAKING_AT_HOME" => s_MAKING_AT_HOME(v),
            "MEAL_WORD" => s_MEAL_WORD(v),
            "MEASURING" => s_MEASURING(v),
            "MONEY_MATTERS" => s_MONEY_MATTERS(v),
            "NOT_AN_IDEA" => s_NOT_AN_IDEA(v),
            "NOT_AN_INGREDIENT" => s_NOT_AN_INGREDIENT(v),
            "NOT_A_DISH" => s_NOT_A_DISH(v),
            "NOT_A_DISH_NAME" => s_NOT_A_DISH_NAME(v),
            "NOT_A_MEAL" => s_NOT_A_MEAL(v),
            "NOT_A_MEAL_SUBJECT" => s_NOT_A_MEAL_SUBJECT(v),
            "NOT_FOOD_LIMITS" => s_NOT_FOOD_LIMITS(v),
            "NOT_FOR_ME" => s_NOT_FOR_ME(v),
            "NOT_MINE" => s_NOT_MINE(v),
            "NOT_MINE_ASK" => s_NOT_MINE_ASK(v),
            "NOT_MINE_CHEAP" => s_NOT_MINE_CHEAP(v),
            "NOT_MINE_ENTRY" => s_NOT_MINE_ENTRY(v),
            "NOT_MINE_OUT" => s_NOT_MINE_OUT(v),
            "NOT_MY_KITCHEN" => s_NOT_MY_KITCHEN(v),
            "NO_CHEESE" => s_NO_CHEESE(v),
            "NO_CUE" => s_NO_CUE(v),
            "NO_DAIRY" => s_NO_DAIRY(v),
            "NO_GAP" => s_NO_GAP(v),
            "NO_HAVE" => s_NO_HAVE(v),
            "NO_HAVE_GAP" => s_NO_HAVE_GAP(v),
            "NO_MEAT" => s_NO_MEAT(v),
            "OPENERS" => s_OPENERS(v),
            "OTHER_PEOPLE" => s_OTHER_PEOPLE(v),
            "OWN_MEAL" => s_OWN_MEAL(v),
            "QUICK" => s_QUICK(v),
            "RECIPE_FOR" => s_RECIPE_FOR(v),
            "REPAIR" => s_REPAIR(v),
            "SAID_AS" => s_SAID_AS(v),
            "SAYING" => s_SAYING(v),
            "SHOW_OFF" => s_SHOW_OFF(v),
            "SICK_OF" => s_SICK_OF(v),
            "SLEEP" => s_SLEEP(v),
            "SMALL_PEOPLE" => s_SMALL_PEOPLE(v),
            "SOUP_NAMED" => s_SOUP_NAMED(v),
            "SPOILT_FOOD" => s_SPOILT_FOOD(v),
            "STAPLE" => s_STAPLE(v),
            "STYLE_CUES" => s_STYLE_CUES(v),
            "STYLE_WANT" => s_STYLE_WANT(v),
            "TAKEOUT_ASK" => s_TAKEOUT_ASK(v),
            "TAKEOUT_PICKS" => s_TAKEOUT_PICKS(v),
            "TAKEOUT_PLACES" => s_TAKEOUT_PLACES(v),
            "TECHNIQUE" => s_TECHNIQUE(v),
            "TECHNIQUE_ASK" => s_TECHNIQUE_ASK(v),
            "TIMES" => s_TIMES(v),
            "TOTAL_TIMES" => s_TOTAL_TIMES(v),
            "UNCOUNTABLE" => s_UNCOUNTABLE(v),
            "UNWELL" => s_UNWELL(v),
            "UP_ALLIUMS" => s_UP_ALLIUMS(v),
            "UP_CLASSES" => s_UP_CLASSES(v),
            "UP_FISH" => s_UP_FISH(v),
            "UP_FOODS" => s_UP_FOODS(v),
            "UP_FRUIT" => s_UP_FRUIT(v),
            "UP_GRAINS" => s_UP_GRAINS(v),
            "UP_GREENS" => s_UP_GREENS(v),
            "UP_MEAT" => s_UP_MEAT(v),
            "UP_ROOTS" => s_UP_ROOTS(v),
            "UP_SOFT" => s_UP_SOFT(v),
            "USE_DISH_WORDS" => s_USE_DISH_WORDS(v),
            "USE_UP" => s_USE_UP(v),
            "USE_UP_ASK" => s_USE_UP_ASK(v),
            "USE_UP_TECHNIQUE" => s_USE_UP_TECHNIQUE(v),
            "WHAT_GOES" => s_WHAT_GOES(v),
            "WITHOUT_WORD" => s_WITHOUT_WORD(v),
            "WITH_ASK" => s_WITH_ASK(v),
            "WORD" => s_WORD(v),
            "_AGE_SHAPE" => s__AGE_SHAPE(v),
            "_NO_MEAT_BLOCKER" => s__NO_MEAT_BLOCKER(v),
            "_UP_FOOD_SET" => s__UP_FOOD_SET(v),
            "_no_have" => s__no_have(v),
            "_row" => s__row(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.meals", module_obj);
    pyrt::xrefs::reg_fn("knowledge.meals", "cheap", py_func("cheap", |args: &[Py]| cheap(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_up_food", py_func("_up_food", |args: &[Py]| _up_food(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "idea_is_a_dish", py_func("idea_is_a_dish", |args: &[Py]| idea_is_a_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "named_food", py_func("named_food", |args: &[Py]| named_food(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "use_it_up", py_func("use_it_up", |args: &[Py]| use_it_up(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_variant", py_func("_variant", |args: &[Py]| _variant(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_no_have", py_func("_no_have", |args: &[Py]| py_call(&g__no_have(), args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_ruled_out", py_func("_ruled_out", |args: &[Py]| _ruled_out(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_the_fish_are_out", py_func("_the_fish_are_out", |args: &[Py]| _the_fish_are_out(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_is_the_ruled_out_food", py_func("_is_the_ruled_out_food", |args: &[Py]| _is_the_ruled_out_food(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_named", py_func("_named", |args: &[Py]| _named(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_clock", py_func("_clock", |args: &[Py]| _clock(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_no_meat", py_func("_no_meat", |args: &[Py]| _no_meat(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_later", py_func("_later", |args: &[Py]| _later(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_limits", py_func("_limits", |args: &[Py]| _limits(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_fits", py_func("_fits", |args: &[Py]| _fits(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_rotate", py_func("_rotate", |args: &[Py]| _rotate(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_spread", py_func("_spread", |args: &[Py]| _spread(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_shape", py_func("_shape", |args: &[Py]| _shape(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_best", py_func("_best", |args: &[Py]| _best(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_prose", py_func("_prose", |args: &[Py]| _prose(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_stick", py_func("_stick", |args: &[Py]| _stick(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_without_dairy", py_func("_without_dairy", |args: &[Py]| _without_dairy(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_times", py_func("_times", |args: &[Py]| _times(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_describe", py_func("_describe", |args: &[Py]| _describe(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "use_up", py_func("use_up", |args: &[Py]| use_up(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_plan", py_func("_plan", |args: &[Py]| _plan(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "reads_as_news", py_func("reads_as_news", |args: &[Py]| reads_as_news(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "asks_for_a_meal", py_func("asks_for_a_meal", |args: &[Py]| asks_for_a_meal(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "says_a_diet", py_func("says_a_diet", |args: &[Py]| says_a_diet(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_carry", py_func("_carry", |args: &[Py]| _carry(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_kitchen_answer", py_func("_kitchen_answer", |args: &[Py]| _kitchen_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_said_about", py_func("_said_about", |args: &[Py]| _said_about(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_diet_said", py_func("_diet_said", |args: &[Py]| _diet_said(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_fresh", py_func("_fresh", |args: &[Py]| _fresh(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_crowd_answer", py_func("_crowd_answer", |args: &[Py]| _crowd_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_count_word", py_func("_count_word", |args: &[Py]| _count_word(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_for_a_table", py_func("_for_a_table", |args: &[Py]| _for_a_table(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_company", py_func("_company", |args: &[Py]| _company(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_unknown_ingredient", py_func("_unknown_ingredient", |args: &[Py]| _unknown_ingredient(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_unknown_dish_recipe", py_func("_unknown_dish_recipe", |args: &[Py]| _unknown_dish_recipe(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "unknown_food", py_func("unknown_food", |args: &[Py]| unknown_food(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_something_it_has_not_got", py_func("_something_it_has_not_got", |args: &[Py]| _something_it_has_not_got(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "suggest", py_func("suggest", |args: &[Py]| suggest(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_takeout_pick", py_func("_takeout_pick", |args: &[Py]| _takeout_pick(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_counters_named", py_func("_counters_named", |args: &[Py]| _counters_named(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_one_counter", py_func("_one_counter", |args: &[Py]| _one_counter(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "_not_a_mark_and_not_a_row", py_func("_not_a_mark_and_not_a_row", |args: &[Py]| _not_a_mark_and_not_a_row(args)));
    pyrt::xrefs::reg_fn("knowledge.meals", "Dish", py_func("Dish", |args: &[Py]| Dish(args)));
    pyrt::xrefs::reg_access("knowledge.meals", "ACK_WORDS", g_ACK_WORDS, s_ACK_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "AILING_PLANT", g_AILING_PLANT, s_AILING_PLANT);
    pyrt::xrefs::reg_access("knowledge.meals", "ANOTHER_MEAL", g_ANOTHER_MEAL, s_ANOTHER_MEAL);
    pyrt::xrefs::reg_access("knowledge.meals", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.meals", "ASKING_WORDS", g_ASKING_WORDS, s_ASKING_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "ASK_CUES", g_ASK_CUES, s_ASK_CUES);
    pyrt::xrefs::reg_access("knowledge.meals", "ASK_IN_NEWS", g_ASK_IN_NEWS, s_ASK_IN_NEWS);
    pyrt::xrefs::reg_access("knowledge.meals", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "BEDTIME", g_BEDTIME, s_BEDTIME);
    pyrt::xrefs::reg_access("knowledge.meals", "BEFORE_EVENT", g_BEFORE_EVENT, s_BEFORE_EVENT);
    pyrt::xrefs::reg_access("knowledge.meals", "BRING_NOT_FOOD", g_BRING_NOT_FOOD, s_BRING_NOT_FOOD);
    pyrt::xrefs::reg_access("knowledge.meals", "BROKEN", g_BROKEN, s_BROKEN);
    pyrt::xrefs::reg_access("knowledge.meals", "CARRY_ASK", g_CARRY_ASK, s_CARRY_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "CARRY_REPLIES", g_CARRY_REPLIES, s_CARRY_REPLIES);
    pyrt::xrefs::reg_access("knowledge.meals", "CHEAP", g_CHEAP, s_CHEAP);
    pyrt::xrefs::reg_access("knowledge.meals", "CHEAP_ASK", g_CHEAP_ASK, s_CHEAP_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "CHEAP_SET", g_CHEAP_SET, s_CHEAP_SET);
    pyrt::xrefs::reg_access("knowledge.meals", "CHILD_AT_TABLE", g_CHILD_AT_TABLE, s_CHILD_AT_TABLE);
    pyrt::xrefs::reg_access("knowledge.meals", "COMPANY", g_COMPANY, s_COMPANY);
    pyrt::xrefs::reg_access("knowledge.meals", "CONDITION", g_CONDITION, s_CONDITION);
    pyrt::xrefs::reg_access("knowledge.meals", "COOK_TIME", g_COOK_TIME, s_COOK_TIME);
    pyrt::xrefs::reg_access("knowledge.meals", "COUNT_WORDS", g_COUNT_WORDS, s_COUNT_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "CROWD_ASK", g_CROWD_ASK, s_CROWD_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "CROWD_SAID", g_CROWD_SAID, s_CROWD_SAID);
    pyrt::xrefs::reg_access("knowledge.meals", "DAIRY_EXTRAS", g_DAIRY_EXTRAS, s_DAIRY_EXTRAS);
    pyrt::xrefs::reg_access("knowledge.meals", "DEFINITION_SHAPE", g_DEFINITION_SHAPE, s_DEFINITION_SHAPE);
    pyrt::xrefs::reg_access("knowledge.meals", "DESSERT_ASK", g_DESSERT_ASK, s_DESSERT_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "DIET_SAID", g_DIET_SAID, s_DIET_SAID);
    pyrt::xrefs::reg_access("knowledge.meals", "DIET_WORDS", g_DIET_WORDS, s_DIET_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "DISHES", g_DISHES, s_DISHES);
    pyrt::xrefs::reg_access("knowledge.meals", "DISH_SHAPES", g_DISH_SHAPES, s_DISH_SHAPES);
    pyrt::xrefs::reg_access("knowledge.meals", "DISH_STYLE", g_DISH_STYLE, s_DISH_STYLE);
    pyrt::xrefs::reg_access("knowledge.meals", "DOUBLE_NO", g_DOUBLE_NO, s_DOUBLE_NO);
    pyrt::xrefs::reg_access("knowledge.meals", "DRINK", g_DRINK, s_DRINK);
    pyrt::xrefs::reg_access("knowledge.meals", "EMPTY", g_EMPTY, s_EMPTY);
    pyrt::xrefs::reg_access("knowledge.meals", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.meals", "EVENT_WORDS", g_EVENT_WORDS, s_EVENT_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "FAVOURITES", g_FAVOURITES, s_FAVOURITES);
    pyrt::xrefs::reg_access("knowledge.meals", "FISH_INGREDIENTS", g_FISH_INGREDIENTS, s_FISH_INGREDIENTS);
    pyrt::xrefs::reg_access("knowledge.meals", "FOLLOW_ON", g_FOLLOW_ON, s_FOLLOW_ON);
    pyrt::xrefs::reg_access("knowledge.meals", "FOLLOW_ON_NO_CHEESE", g_FOLLOW_ON_NO_CHEESE, s_FOLLOW_ON_NO_CHEESE);
    pyrt::xrefs::reg_access("knowledge.meals", "FOOD_WORDS", g_FOOD_WORDS, s_FOOD_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "GAME_WORDS", g_GAME_WORDS, s_GAME_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "GLUTEN", g_GLUTEN, s_GLUTEN);
    pyrt::xrefs::reg_access("knowledge.meals", "GRACE", g_GRACE, s_GRACE);
    pyrt::xrefs::reg_access("knowledge.meals", "HARD_CLOCK", g_HARD_CLOCK, s_HARD_CLOCK);
    pyrt::xrefs::reg_access("knowledge.meals", "HEALTH", g_HEALTH, s_HEALTH);
    pyrt::xrefs::reg_access("knowledge.meals", "HOW_TO", g_HOW_TO, s_HOW_TO);
    pyrt::xrefs::reg_access("knowledge.meals", "HUNGRY", g_HUNGRY, s_HUNGRY);
    pyrt::xrefs::reg_access("knowledge.meals", "IDEA_ASK", g_IDEA_ASK, s_IDEA_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "IMPRESS", g_IMPRESS, s_IMPRESS);
    pyrt::xrefs::reg_access("knowledge.meals", "IMPRESS_ASK", g_IMPRESS_ASK, s_IMPRESS_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "IMPRESS_FOR", g_IMPRESS_FOR, s_IMPRESS_FOR);
    pyrt::xrefs::reg_access("knowledge.meals", "IMPRESS_WORTH", g_IMPRESS_WORTH, s_IMPRESS_WORTH);
    pyrt::xrefs::reg_access("knowledge.meals", "INGREDIENTS", g_INGREDIENTS, s_INGREDIENTS);
    pyrt::xrefs::reg_access("knowledge.meals", "IN_A_DISH", g_IN_A_DISH, s_IN_A_DISH);
    pyrt::xrefs::reg_access("knowledge.meals", "KIDS_IDEA", g_KIDS_IDEA, s_KIDS_IDEA);
    pyrt::xrefs::reg_access("knowledge.meals", "KITCHEN_WORDS", g_KITCHEN_WORDS, s_KITCHEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "LIGHT_ASK", g_LIGHT_ASK, s_LIGHT_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "LIGHT_SET", g_LIGHT_SET, s_LIGHT_SET);
    pyrt::xrefs::reg_access("knowledge.meals", "LUNCH_ASK", g_LUNCH_ASK, s_LUNCH_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "MAKING_AT_HOME", g_MAKING_AT_HOME, s_MAKING_AT_HOME);
    pyrt::xrefs::reg_access("knowledge.meals", "MEAL_WORD", g_MEAL_WORD, s_MEAL_WORD);
    pyrt::xrefs::reg_access("knowledge.meals", "MEASURING", g_MEASURING, s_MEASURING);
    pyrt::xrefs::reg_access("knowledge.meals", "MONEY_MATTERS", g_MONEY_MATTERS, s_MONEY_MATTERS);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_AN_IDEA", g_NOT_AN_IDEA, s_NOT_AN_IDEA);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_AN_INGREDIENT", g_NOT_AN_INGREDIENT, s_NOT_AN_INGREDIENT);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_A_DISH", g_NOT_A_DISH, s_NOT_A_DISH);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_A_DISH_NAME", g_NOT_A_DISH_NAME, s_NOT_A_DISH_NAME);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_A_MEAL", g_NOT_A_MEAL, s_NOT_A_MEAL);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_A_MEAL_SUBJECT", g_NOT_A_MEAL_SUBJECT, s_NOT_A_MEAL_SUBJECT);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_FOOD_LIMITS", g_NOT_FOOD_LIMITS, s_NOT_FOOD_LIMITS);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_FOR_ME", g_NOT_FOR_ME, s_NOT_FOR_ME);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_MINE", g_NOT_MINE, s_NOT_MINE);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_MINE_ASK", g_NOT_MINE_ASK, s_NOT_MINE_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_MINE_CHEAP", g_NOT_MINE_CHEAP, s_NOT_MINE_CHEAP);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_MINE_ENTRY", g_NOT_MINE_ENTRY, s_NOT_MINE_ENTRY);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_MINE_OUT", g_NOT_MINE_OUT, s_NOT_MINE_OUT);
    pyrt::xrefs::reg_access("knowledge.meals", "NOT_MY_KITCHEN", g_NOT_MY_KITCHEN, s_NOT_MY_KITCHEN);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_CHEESE", g_NO_CHEESE, s_NO_CHEESE);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_CUE", g_NO_CUE, s_NO_CUE);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_DAIRY", g_NO_DAIRY, s_NO_DAIRY);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_GAP", g_NO_GAP, s_NO_GAP);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_HAVE", g_NO_HAVE, s_NO_HAVE);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_HAVE_GAP", g_NO_HAVE_GAP, s_NO_HAVE_GAP);
    pyrt::xrefs::reg_access("knowledge.meals", "NO_MEAT", g_NO_MEAT, s_NO_MEAT);
    pyrt::xrefs::reg_access("knowledge.meals", "OPENERS", g_OPENERS, s_OPENERS);
    pyrt::xrefs::reg_access("knowledge.meals", "OTHER_PEOPLE", g_OTHER_PEOPLE, s_OTHER_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.meals", "OWN_MEAL", g_OWN_MEAL, s_OWN_MEAL);
    pyrt::xrefs::reg_access("knowledge.meals", "QUICK", g_QUICK, s_QUICK);
    pyrt::xrefs::reg_access("knowledge.meals", "RECIPE_FOR", g_RECIPE_FOR, s_RECIPE_FOR);
    pyrt::xrefs::reg_access("knowledge.meals", "REPAIR", g_REPAIR, s_REPAIR);
    pyrt::xrefs::reg_access("knowledge.meals", "SAID_AS", g_SAID_AS, s_SAID_AS);
    pyrt::xrefs::reg_access("knowledge.meals", "SAYING", g_SAYING, s_SAYING);
    pyrt::xrefs::reg_access("knowledge.meals", "SHOW_OFF", g_SHOW_OFF, s_SHOW_OFF);
    pyrt::xrefs::reg_access("knowledge.meals", "SICK_OF", g_SICK_OF, s_SICK_OF);
    pyrt::xrefs::reg_access("knowledge.meals", "SLEEP", g_SLEEP, s_SLEEP);
    pyrt::xrefs::reg_access("knowledge.meals", "SMALL_PEOPLE", g_SMALL_PEOPLE, s_SMALL_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.meals", "SOUP_NAMED", g_SOUP_NAMED, s_SOUP_NAMED);
    pyrt::xrefs::reg_access("knowledge.meals", "SPOILT_FOOD", g_SPOILT_FOOD, s_SPOILT_FOOD);
    pyrt::xrefs::reg_access("knowledge.meals", "STAPLE", g_STAPLE, s_STAPLE);
    pyrt::xrefs::reg_access("knowledge.meals", "STYLE_CUES", g_STYLE_CUES, s_STYLE_CUES);
    pyrt::xrefs::reg_access("knowledge.meals", "STYLE_WANT", g_STYLE_WANT, s_STYLE_WANT);
    pyrt::xrefs::reg_access("knowledge.meals", "TAKEOUT_ASK", g_TAKEOUT_ASK, s_TAKEOUT_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "TAKEOUT_PICKS", g_TAKEOUT_PICKS, s_TAKEOUT_PICKS);
    pyrt::xrefs::reg_access("knowledge.meals", "TAKEOUT_PLACES", g_TAKEOUT_PLACES, s_TAKEOUT_PLACES);
    pyrt::xrefs::reg_access("knowledge.meals", "TECHNIQUE", g_TECHNIQUE, s_TECHNIQUE);
    pyrt::xrefs::reg_access("knowledge.meals", "TECHNIQUE_ASK", g_TECHNIQUE_ASK, s_TECHNIQUE_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "TIMES", g_TIMES, s_TIMES);
    pyrt::xrefs::reg_access("knowledge.meals", "TOTAL_TIMES", g_TOTAL_TIMES, s_TOTAL_TIMES);
    pyrt::xrefs::reg_access("knowledge.meals", "UNCOUNTABLE", g_UNCOUNTABLE, s_UNCOUNTABLE);
    pyrt::xrefs::reg_access("knowledge.meals", "UNWELL", g_UNWELL, s_UNWELL);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_ALLIUMS", g_UP_ALLIUMS, s_UP_ALLIUMS);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_CLASSES", g_UP_CLASSES, s_UP_CLASSES);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_FISH", g_UP_FISH, s_UP_FISH);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_FOODS", g_UP_FOODS, s_UP_FOODS);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_FRUIT", g_UP_FRUIT, s_UP_FRUIT);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_GRAINS", g_UP_GRAINS, s_UP_GRAINS);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_GREENS", g_UP_GREENS, s_UP_GREENS);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_MEAT", g_UP_MEAT, s_UP_MEAT);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_ROOTS", g_UP_ROOTS, s_UP_ROOTS);
    pyrt::xrefs::reg_access("knowledge.meals", "UP_SOFT", g_UP_SOFT, s_UP_SOFT);
    pyrt::xrefs::reg_access("knowledge.meals", "USE_DISH_WORDS", g_USE_DISH_WORDS, s_USE_DISH_WORDS);
    pyrt::xrefs::reg_access("knowledge.meals", "USE_UP", g_USE_UP, s_USE_UP);
    pyrt::xrefs::reg_access("knowledge.meals", "USE_UP_ASK", g_USE_UP_ASK, s_USE_UP_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "USE_UP_TECHNIQUE", g_USE_UP_TECHNIQUE, s_USE_UP_TECHNIQUE);
    pyrt::xrefs::reg_access("knowledge.meals", "WHAT_GOES", g_WHAT_GOES, s_WHAT_GOES);
    pyrt::xrefs::reg_access("knowledge.meals", "WITHOUT_WORD", g_WITHOUT_WORD, s_WITHOUT_WORD);
    pyrt::xrefs::reg_access("knowledge.meals", "WITH_ASK", g_WITH_ASK, s_WITH_ASK);
    pyrt::xrefs::reg_access("knowledge.meals", "WORD", g_WORD, s_WORD);
    pyrt::xrefs::reg_access("knowledge.meals", "_AGE_SHAPE", g__AGE_SHAPE, s__AGE_SHAPE);
    pyrt::xrefs::reg_access("knowledge.meals", "_NO_MEAT_BLOCKER", g__NO_MEAT_BLOCKER, s__NO_MEAT_BLOCKER);
    pyrt::xrefs::reg_access("knowledge.meals", "_UP_FOOD_SET", g__UP_FOOD_SET, s__UP_FOOD_SET);
    pyrt::xrefs::reg_access("knowledge.meals", "_no_have", g__no_have, s__no_have);
    pyrt::xrefs::reg_access("knowledge.meals", "_row", g__row, s__row);
}

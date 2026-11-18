#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_mut, unused_variables, unused_assignments, unused_parens, unused_imports, clippy::all)]
use crate::pyrt::builtins::*;
use crate::pyrt::pybuiltins::*;
use crate::pyrt::stdlib::*;
use crate::pyrt::value::*;
use crate::pyrt::exc;
use crate::pyrt::{pystr, pyre, pyrandom, pydatetime};
use std::sync::{LazyLock as Lazy, RwLock, Once};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use indexmap::IndexMap;


pub static ABSENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALLERGY_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALLERGY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AVOID_TEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SIDE_WITH_NO_DISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_SUM_OF_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_WISH_FOR_A_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BAKING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_MEAL_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_MEAL_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_MEAL_OBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BIG_DAYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHATTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHILLI_FRUIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COMING_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COOKED_THROUGH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COOKING_FOR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COOK_REQUEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CROWD_DISHES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CROWD_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAIRY_FOODS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DAIRY_FREE_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DINNER_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISH_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DISLIKE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENTRIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVENT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FATTENING_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOOD_PROPERTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FOR_ONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREEN_IN_TEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREEN_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GREEN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAT_INGREDIENTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAT_IN_TEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEAT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOLD_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIMIT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEAL_FRAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEAT_FREE_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEAT_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MENU_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static METHOD_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MINUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEW_MEAL_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_DONENESS_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_TOPPING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ONLY_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_TOPPINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PANTRY_MINUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PEOPLE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PICK_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_KEYS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLAN_LAPSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUICK_MINUTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REFUSAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SEED_FRUIT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIMPLE_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLOT_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLOT_IDEAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SOMEONE_ELSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STAPLES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOCK_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWAP_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWEET_DISHES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWEET_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SWEET_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TASTE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TECHNIQUE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_FOOD_IN_HAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_WAY_TO_COOK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THIN_CUPBOARD_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEGAN_BAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VEG_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WAKE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WITH_PHRASES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ABOUT_A_FOOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _AHEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _APPLE_BROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PRONOUN_HELD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_REACTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BROWN_KINDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BROWN_SAFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _BROWN_SAFE_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CELERY_BROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CHILLI_SEEDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _COLD_HELD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _CUCUMBER_SEEDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _DRY_BROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLATBREAD_KINDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FLATBREAD_RECIPES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FOOD_IN_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _FOOD_JOB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _GROWING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOT_FOOD_RULED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _HOT_HELD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MELON_SEEDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MUSHROOM_BROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MUSHROOM_SAFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _MUSHROOM_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OUR_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POMEGRANATE_SEEDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _PRODUCE_BROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SQUASH_SEEDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TASTE_RULED_OUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _TOMATO_SEEDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WET_BROWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _reply: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _staple: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _words: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABSENCE() -> Py { ensure_init(); ABSENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABSENCE(v: Py) { ensure_init(); if let Ok(mut w) = ABSENCE.write() { *w = v; } }
pub fn g_ACK_WORDS() -> Py { ensure_init(); ACK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ACK_WORDS.write() { *w = v; } }
pub fn g_ALLERGY_PHRASES() -> Py { ensure_init(); ALLERGY_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = ALLERGY_PHRASES.write() { *w = v; } }
pub fn g_ALLERGY_WORDS() -> Py { ensure_init(); ALLERGY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALLERGY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ALLERGY_WORDS.write() { *w = v; } }
pub fn g_AVOID_TEXT() -> Py { ensure_init(); AVOID_TEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AVOID_TEXT(v: Py) { ensure_init(); if let Ok(mut w) = AVOID_TEXT.write() { *w = v; } }
pub fn g_A_SIDE_WITH_NO_DISH() -> Py { ensure_init(); A_SIDE_WITH_NO_DISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SIDE_WITH_NO_DISH(v: Py) { ensure_init(); if let Ok(mut w) = A_SIDE_WITH_NO_DISH.write() { *w = v; } }
pub fn g_A_SUM_OF_MONEY() -> Py { ensure_init(); A_SUM_OF_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_SUM_OF_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = A_SUM_OF_MONEY.write() { *w = v; } }
pub fn g_A_WISH_FOR_A_FOOD() -> Py { ensure_init(); A_WISH_FOR_A_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_WISH_FOR_A_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = A_WISH_FOR_A_FOOD.write() { *w = v; } }
pub fn g_BAKING_WORDS() -> Py { ensure_init(); BAKING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BAKING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = BAKING_WORDS.write() { *w = v; } }
pub fn g_BARE_MEAL_ASK_WORDS() -> Py { ensure_init(); BARE_MEAL_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_MEAL_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = BARE_MEAL_ASK_WORDS.write() { *w = v; } }
pub fn g_BARE_MEAL_NOUNS() -> Py { ensure_init(); BARE_MEAL_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_MEAL_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = BARE_MEAL_NOUNS.write() { *w = v; } }
pub fn g_BARE_MEAL_OBJECT() -> Py { ensure_init(); BARE_MEAL_OBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_MEAL_OBJECT(v: Py) { ensure_init(); if let Ok(mut w) = BARE_MEAL_OBJECT.write() { *w = v; } }
pub fn g_BIG_DAYS() -> Py { ensure_init(); BIG_DAYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BIG_DAYS(v: Py) { ensure_init(); if let Ok(mut w) = BIG_DAYS.write() { *w = v; } }
pub fn g_CHATTER() -> Py { ensure_init(); CHATTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHATTER(v: Py) { ensure_init(); if let Ok(mut w) = CHATTER.write() { *w = v; } }
pub fn g_CHILLI_FRUIT() -> Py { ensure_init(); CHILLI_FRUIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHILLI_FRUIT(v: Py) { ensure_init(); if let Ok(mut w) = CHILLI_FRUIT.write() { *w = v; } }
pub fn g_COMING_FOR() -> Py { ensure_init(); COMING_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COMING_FOR(v: Py) { ensure_init(); if let Ok(mut w) = COMING_FOR.write() { *w = v; } }
pub fn g_COOKED_THROUGH_WORDS() -> Py { ensure_init(); COOKED_THROUGH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COOKED_THROUGH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = COOKED_THROUGH_WORDS.write() { *w = v; } }
pub fn g_COOKING_FOR() -> Py { ensure_init(); COOKING_FOR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COOKING_FOR(v: Py) { ensure_init(); if let Ok(mut w) = COOKING_FOR.write() { *w = v; } }
pub fn g_COOK_REQUEST() -> Py { ensure_init(); COOK_REQUEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COOK_REQUEST(v: Py) { ensure_init(); if let Ok(mut w) = COOK_REQUEST.write() { *w = v; } }
pub fn g_CROWD_DISHES() -> Py { ensure_init(); CROWD_DISHES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CROWD_DISHES(v: Py) { ensure_init(); if let Ok(mut w) = CROWD_DISHES.write() { *w = v; } }
pub fn g_CROWD_PHRASE() -> Py { ensure_init(); CROWD_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CROWD_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = CROWD_PHRASE.write() { *w = v; } }
pub fn g_DAIRY_FOODS() -> Py { ensure_init(); DAIRY_FOODS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAIRY_FOODS(v: Py) { ensure_init(); if let Ok(mut w) = DAIRY_FOODS.write() { *w = v; } }
pub fn g_DAIRY_FREE_PHRASES() -> Py { ensure_init(); DAIRY_FREE_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DAIRY_FREE_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = DAIRY_FREE_PHRASES.write() { *w = v; } }
pub fn g_DINNER_WORDS() -> Py { ensure_init(); DINNER_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DINNER_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DINNER_WORDS.write() { *w = v; } }
pub fn g_DISH_WORDS() -> Py { ensure_init(); DISH_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISH_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DISH_WORDS.write() { *w = v; } }
pub fn g_DISLIKE_WORDS() -> Py { ensure_init(); DISLIKE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DISLIKE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DISLIKE_WORDS.write() { *w = v; } }
pub fn g_ENTRIES() -> Py { ensure_init(); ENTRIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); if let Ok(mut w) = ENTRIES.write() { *w = v; } }
pub fn g_EVENT_WORDS() -> Py { ensure_init(); EVENT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVENT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = EVENT_WORDS.write() { *w = v; } }
pub fn g_FATTENING_WORDS() -> Py { ensure_init(); FATTENING_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FATTENING_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FATTENING_WORDS.write() { *w = v; } }
pub fn g_FOOD_PROPERTY() -> Py { ensure_init(); FOOD_PROPERTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOOD_PROPERTY(v: Py) { ensure_init(); if let Ok(mut w) = FOOD_PROPERTY.write() { *w = v; } }
pub fn g_FOR_ONE() -> Py { ensure_init(); FOR_ONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FOR_ONE(v: Py) { ensure_init(); if let Ok(mut w) = FOR_ONE.write() { *w = v; } }
pub fn g_GREEN_IN_TEXT() -> Py { ensure_init(); GREEN_IN_TEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREEN_IN_TEXT(v: Py) { ensure_init(); if let Ok(mut w) = GREEN_IN_TEXT.write() { *w = v; } }
pub fn g_GREEN_NAMES() -> Py { ensure_init(); GREEN_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREEN_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = GREEN_NAMES.write() { *w = v; } }
pub fn g_GREEN_WORDS() -> Py { ensure_init(); GREEN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GREEN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = GREEN_WORDS.write() { *w = v; } }
pub fn g_HEAT_INGREDIENTS() -> Py { ensure_init(); HEAT_INGREDIENTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAT_INGREDIENTS(v: Py) { ensure_init(); if let Ok(mut w) = HEAT_INGREDIENTS.write() { *w = v; } }
pub fn g_HEAT_IN_TEXT() -> Py { ensure_init(); HEAT_IN_TEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAT_IN_TEXT(v: Py) { ensure_init(); if let Ok(mut w) = HEAT_IN_TEXT.write() { *w = v; } }
pub fn g_HEAT_WORDS() -> Py { ensure_init(); HEAT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEAT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HEAT_WORDS.write() { *w = v; } }
pub fn g_HOLD_WORDS() -> Py { ensure_init(); HOLD_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOLD_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = HOLD_WORDS.write() { *w = v; } }
pub fn g_LIMIT_WORDS() -> Py { ensure_init(); LIMIT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIMIT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = LIMIT_WORDS.write() { *w = v; } }
pub fn g_MEAL_FRAME() -> Py { ensure_init(); MEAL_FRAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEAL_FRAME(v: Py) { ensure_init(); if let Ok(mut w) = MEAL_FRAME.write() { *w = v; } }
pub fn g_MEAT_FREE_PHRASES() -> Py { ensure_init(); MEAT_FREE_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEAT_FREE_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = MEAT_FREE_PHRASES.write() { *w = v; } }
pub fn g_MEAT_NAMES() -> Py { ensure_init(); MEAT_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEAT_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = MEAT_NAMES.write() { *w = v; } }
pub fn g_MENU_PHRASES() -> Py { ensure_init(); MENU_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MENU_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = MENU_PHRASES.write() { *w = v; } }
pub fn g_METHOD_PHRASES() -> Py { ensure_init(); METHOD_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_METHOD_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = METHOD_PHRASES.write() { *w = v; } }
pub fn g_MINUTES() -> Py { ensure_init(); MINUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MINUTES(v: Py) { ensure_init(); if let Ok(mut w) = MINUTES.write() { *w = v; } }
pub fn g_NEW_MEAL_PHRASES() -> Py { ensure_init(); NEW_MEAL_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEW_MEAL_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = NEW_MEAL_PHRASES.write() { *w = v; } }
pub fn g_NOT_A_DONENESS_QUESTION() -> Py { ensure_init(); NOT_A_DONENESS_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_DONENESS_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_DONENESS_QUESTION.write() { *w = v; } }
pub fn g_NOT_A_TOPPING_ASK() -> Py { ensure_init(); NOT_A_TOPPING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TOPPING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_TOPPING_ASK.write() { *w = v; } }
pub fn g_ONLY_PHRASES() -> Py { ensure_init(); ONLY_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ONLY_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = ONLY_PHRASES.write() { *w = v; } }
pub fn g_ON_TOPPINGS() -> Py { ensure_init(); ON_TOPPINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_TOPPINGS(v: Py) { ensure_init(); if let Ok(mut w) = ON_TOPPINGS.write() { *w = v; } }
pub fn g_ON_WORDS() -> Py { ensure_init(); ON_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ON_WORDS.write() { *w = v; } }
pub fn g_PANTRY_MINUTES() -> Py { ensure_init(); PANTRY_MINUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PANTRY_MINUTES(v: Py) { ensure_init(); if let Ok(mut w) = PANTRY_MINUTES.write() { *w = v; } }
pub fn g_PEOPLE_WORDS() -> Py { ensure_init(); PEOPLE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PEOPLE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PEOPLE_WORDS.write() { *w = v; } }
pub fn g_PERSON_WORDS() -> Py { ensure_init(); PERSON_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_WORDS.write() { *w = v; } }
pub fn g_PICK_PHRASES() -> Py { ensure_init(); PICK_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PICK_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = PICK_PHRASES.write() { *w = v; } }
pub fn g_PLAN_KEYS() -> Py { ensure_init(); PLAN_KEYS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_KEYS(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_KEYS.write() { *w = v; } }
pub fn g_PLAN_LAPSE() -> Py { ensure_init(); PLAN_LAPSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLAN_LAPSE(v: Py) { ensure_init(); if let Ok(mut w) = PLAN_LAPSE.write() { *w = v; } }
pub fn g_QUICK_MINUTES() -> Py { ensure_init(); QUICK_MINUTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUICK_MINUTES(v: Py) { ensure_init(); if let Ok(mut w) = QUICK_MINUTES.write() { *w = v; } }
pub fn g_REFUSAL() -> Py { ensure_init(); REFUSAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REFUSAL(v: Py) { ensure_init(); if let Ok(mut w) = REFUSAL.write() { *w = v; } }
pub fn g_SAID_PHRASES() -> Py { ensure_init(); SAID_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = SAID_PHRASES.write() { *w = v; } }
pub fn g_SEED_FRUIT() -> Py { ensure_init(); SEED_FRUIT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SEED_FRUIT(v: Py) { ensure_init(); if let Ok(mut w) = SEED_FRUIT.write() { *w = v; } }
pub fn g_SIMPLE_PHRASES() -> Py { ensure_init(); SIMPLE_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIMPLE_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = SIMPLE_PHRASES.write() { *w = v; } }
pub fn g_SLOT_ASKS() -> Py { ensure_init(); SLOT_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLOT_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = SLOT_ASKS.write() { *w = v; } }
pub fn g_SLOT_IDEAS() -> Py { ensure_init(); SLOT_IDEAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLOT_IDEAS(v: Py) { ensure_init(); if let Ok(mut w) = SLOT_IDEAS.write() { *w = v; } }
pub fn g_SOMEONE_ELSE() -> Py { ensure_init(); SOMEONE_ELSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SOMEONE_ELSE(v: Py) { ensure_init(); if let Ok(mut w) = SOMEONE_ELSE.write() { *w = v; } }
pub fn g_STAPLES() -> Py { ensure_init(); STAPLES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STAPLES(v: Py) { ensure_init(); if let Ok(mut w) = STAPLES.write() { *w = v; } }
pub fn g_STOCK_PHRASES() -> Py { ensure_init(); STOCK_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOCK_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = STOCK_PHRASES.write() { *w = v; } }
pub fn g_SWAP_PHRASES() -> Py { ensure_init(); SWAP_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWAP_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = SWAP_PHRASES.write() { *w = v; } }
pub fn g_SWEET_DISHES() -> Py { ensure_init(); SWEET_DISHES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWEET_DISHES(v: Py) { ensure_init(); if let Ok(mut w) = SWEET_DISHES.write() { *w = v; } }
pub fn g_SWEET_PHRASES() -> Py { ensure_init(); SWEET_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWEET_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = SWEET_PHRASES.write() { *w = v; } }
pub fn g_SWEET_WORDS() -> Py { ensure_init(); SWEET_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SWEET_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SWEET_WORDS.write() { *w = v; } }
pub fn g_TASTE_WORDS() -> Py { ensure_init(); TASTE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TASTE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TASTE_WORDS.write() { *w = v; } }
pub fn g_TECHNIQUE_WORDS() -> Py { ensure_init(); TECHNIQUE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TECHNIQUE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = TECHNIQUE_WORDS.write() { *w = v; } }
pub fn g_THE_FOOD_IN_HAND() -> Py { ensure_init(); THE_FOOD_IN_HAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_FOOD_IN_HAND(v: Py) { ensure_init(); if let Ok(mut w) = THE_FOOD_IN_HAND.write() { *w = v; } }
pub fn g_THE_WAY_TO_COOK() -> Py { ensure_init(); THE_WAY_TO_COOK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_WAY_TO_COOK(v: Py) { ensure_init(); if let Ok(mut w) = THE_WAY_TO_COOK.write() { *w = v; } }
pub fn g_THIN() -> Py { ensure_init(); THIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THIN(v: Py) { ensure_init(); if let Ok(mut w) = THIN.write() { *w = v; } }
pub fn g_THIN_CUPBOARD_PHRASES() -> Py { ensure_init(); THIN_CUPBOARD_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THIN_CUPBOARD_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = THIN_CUPBOARD_PHRASES.write() { *w = v; } }
pub fn g_VEGAN_BAD() -> Py { ensure_init(); VEGAN_BAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEGAN_BAD(v: Py) { ensure_init(); if let Ok(mut w) = VEGAN_BAD.write() { *w = v; } }
pub fn g_VEG_WORDS() -> Py { ensure_init(); VEG_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VEG_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = VEG_WORDS.write() { *w = v; } }
pub fn g_WAKE_WORDS() -> Py { ensure_init(); WAKE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WAKE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = WAKE_WORDS.write() { *w = v; } }
pub fn g_WITH_PHRASES() -> Py { ensure_init(); WITH_PHRASES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WITH_PHRASES(v: Py) { ensure_init(); if let Ok(mut w) = WITH_PHRASES.write() { *w = v; } }
pub fn g__ABOUT_A_FOOD() -> Py { ensure_init(); _ABOUT_A_FOOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ABOUT_A_FOOD(v: Py) { ensure_init(); if let Ok(mut w) = _ABOUT_A_FOOD.write() { *w = v; } }
pub fn g__AHEAD() -> Py { ensure_init(); _AHEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__AHEAD(v: Py) { ensure_init(); if let Ok(mut w) = _AHEAD.write() { *w = v; } }
pub fn g__APPLE_BROWN() -> Py { ensure_init(); _APPLE_BROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__APPLE_BROWN(v: Py) { ensure_init(); if let Ok(mut w) = _APPLE_BROWN.write() { *w = v; } }
pub fn g__A_PRONOUN_HELD() -> Py { ensure_init(); _A_PRONOUN_HELD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PRONOUN_HELD(v: Py) { ensure_init(); if let Ok(mut w) = _A_PRONOUN_HELD.write() { *w = v; } }
pub fn g__A_REACTION() -> Py { ensure_init(); _A_REACTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_REACTION(v: Py) { ensure_init(); if let Ok(mut w) = _A_REACTION.write() { *w = v; } }
pub fn g__BROWN_KINDS() -> Py { ensure_init(); _BROWN_KINDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BROWN_KINDS(v: Py) { ensure_init(); if let Ok(mut w) = _BROWN_KINDS.write() { *w = v; } }
pub fn g__BROWN_SAFE() -> Py { ensure_init(); _BROWN_SAFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BROWN_SAFE(v: Py) { ensure_init(); if let Ok(mut w) = _BROWN_SAFE.write() { *w = v; } }
pub fn g__BROWN_SAFE_ASKS() -> Py { ensure_init(); _BROWN_SAFE_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__BROWN_SAFE_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = _BROWN_SAFE_ASKS.write() { *w = v; } }
pub fn g__CELERY_BROWN() -> Py { ensure_init(); _CELERY_BROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CELERY_BROWN(v: Py) { ensure_init(); if let Ok(mut w) = _CELERY_BROWN.write() { *w = v; } }
pub fn g__CHILLI_SEEDS() -> Py { ensure_init(); _CHILLI_SEEDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CHILLI_SEEDS(v: Py) { ensure_init(); if let Ok(mut w) = _CHILLI_SEEDS.write() { *w = v; } }
pub fn g__COLD_HELD() -> Py { ensure_init(); _COLD_HELD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__COLD_HELD(v: Py) { ensure_init(); if let Ok(mut w) = _COLD_HELD.write() { *w = v; } }
pub fn g__CUCUMBER_SEEDS() -> Py { ensure_init(); _CUCUMBER_SEEDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__CUCUMBER_SEEDS(v: Py) { ensure_init(); if let Ok(mut w) = _CUCUMBER_SEEDS.write() { *w = v; } }
pub fn g__DRY_BROWN() -> Py { ensure_init(); _DRY_BROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__DRY_BROWN(v: Py) { ensure_init(); if let Ok(mut w) = _DRY_BROWN.write() { *w = v; } }
pub fn g__FLATBREAD_KINDS() -> Py { ensure_init(); _FLATBREAD_KINDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLATBREAD_KINDS(v: Py) { ensure_init(); if let Ok(mut w) = _FLATBREAD_KINDS.write() { *w = v; } }
pub fn g__FLATBREAD_RECIPES() -> Py { ensure_init(); _FLATBREAD_RECIPES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FLATBREAD_RECIPES(v: Py) { ensure_init(); if let Ok(mut w) = _FLATBREAD_RECIPES.write() { *w = v; } }
pub fn g__FOOD_IN_LINE() -> Py { ensure_init(); _FOOD_IN_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FOOD_IN_LINE(v: Py) { ensure_init(); if let Ok(mut w) = _FOOD_IN_LINE.write() { *w = v; } }
pub fn g__FOOD_JOB() -> Py { ensure_init(); _FOOD_JOB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__FOOD_JOB(v: Py) { ensure_init(); if let Ok(mut w) = _FOOD_JOB.write() { *w = v; } }
pub fn g__GROWING_WORD() -> Py { ensure_init(); _GROWING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__GROWING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = _GROWING_WORD.write() { *w = v; } }
pub fn g__HOT_FOOD_RULED_OUT() -> Py { ensure_init(); _HOT_FOOD_RULED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOT_FOOD_RULED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = _HOT_FOOD_RULED_OUT.write() { *w = v; } }
pub fn g__HOT_HELD() -> Py { ensure_init(); _HOT_HELD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__HOT_HELD(v: Py) { ensure_init(); if let Ok(mut w) = _HOT_HELD.write() { *w = v; } }
pub fn g__MELON_SEEDS() -> Py { ensure_init(); _MELON_SEEDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MELON_SEEDS(v: Py) { ensure_init(); if let Ok(mut w) = _MELON_SEEDS.write() { *w = v; } }
pub fn g__MUSHROOM_BROWN() -> Py { ensure_init(); _MUSHROOM_BROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MUSHROOM_BROWN(v: Py) { ensure_init(); if let Ok(mut w) = _MUSHROOM_BROWN.write() { *w = v; } }
pub fn g__MUSHROOM_SAFE() -> Py { ensure_init(); _MUSHROOM_SAFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MUSHROOM_SAFE(v: Py) { ensure_init(); if let Ok(mut w) = _MUSHROOM_SAFE.write() { *w = v; } }
pub fn g__MUSHROOM_WORDS() -> Py { ensure_init(); _MUSHROOM_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__MUSHROOM_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = _MUSHROOM_WORDS.write() { *w = v; } }
pub fn g__OUR_OWN() -> Py { ensure_init(); _OUR_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OUR_OWN(v: Py) { ensure_init(); if let Ok(mut w) = _OUR_OWN.write() { *w = v; } }
pub fn g__POMEGRANATE_SEEDS() -> Py { ensure_init(); _POMEGRANATE_SEEDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POMEGRANATE_SEEDS(v: Py) { ensure_init(); if let Ok(mut w) = _POMEGRANATE_SEEDS.write() { *w = v; } }
pub fn g__PRODUCE_BROWN() -> Py { ensure_init(); _PRODUCE_BROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__PRODUCE_BROWN(v: Py) { ensure_init(); if let Ok(mut w) = _PRODUCE_BROWN.write() { *w = v; } }
pub fn g__SQUASH_SEEDS() -> Py { ensure_init(); _SQUASH_SEEDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SQUASH_SEEDS(v: Py) { ensure_init(); if let Ok(mut w) = _SQUASH_SEEDS.write() { *w = v; } }
pub fn g__TASTE_RULED_OUT() -> Py { ensure_init(); _TASTE_RULED_OUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TASTE_RULED_OUT(v: Py) { ensure_init(); if let Ok(mut w) = _TASTE_RULED_OUT.write() { *w = v; } }
pub fn g__TOMATO_SEEDS() -> Py { ensure_init(); _TOMATO_SEEDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__TOMATO_SEEDS(v: Py) { ensure_init(); if let Ok(mut w) = _TOMATO_SEEDS.write() { *w = v; } }
pub fn g__WET_BROWN() -> Py { ensure_init(); _WET_BROWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WET_BROWN(v: Py) { ensure_init(); if let Ok(mut w) = _WET_BROWN.write() { *w = v; } }
pub fn g__reply() -> Py { ensure_init(); _reply.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__reply(v: Py) { ensure_init(); if let Ok(mut w) = _reply.write() { *w = v; } }
pub fn g__staple() -> Py { ensure_init(); _staple.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__staple(v: Py) { ensure_init(); if let Ok(mut w) = _staple.write() { *w = v; } }
pub fn g__words() -> Py { ensure_init(); _words.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__words(v: Py) { ensure_init(); if let Ok(mut w) = _words.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The cooking rows that answer \"is it done, why did it do that, what is the plan\".\n\nThe sections of `cooking.py` from \"how you tell it is cooked\" to the end of that file, in a file\nof their own: `cooking.py` is over the publisher's 1 MiB upload limit (1,082,093 bytes) and the\nworld refuses any PR whose tree carries a file that big, so every lane that touches cooking food\nis blocked. The rows are data rather than code and they are the end of the list, so they move as\none block, the same way `cooking_entries.py` and `home_craft.py` hold rows for their files.\n\nNothing about the rows changed. `cooking.py` imports this file at the end, where these rows used\nto stand, and extends its own list with this one, so the entries are the same entries in the same\norder. The helpers this block defines (`follow_up`, `first_plan`, `minutes_said`, ...) come back\nwith it: other modules ask `cooking.follow_up(...)`, so `cooking.py` puts them in its namespace.\n\nThis file is imported by `cooking.py`, never the other way round, so the import of `.cooking` is\nat module level only for the tables the rows are built from, and every name it asks for is\ndefined above the point where `cooking.py` imports this file.\n");
    s_ENTRIES(py_list(vec![]));
    s_A_SIDE_WITH_NO_DISH(py_str("Two of these three, and the plate balances itself: something starchy (new potatoes, rice, bread), a vegetable cooked hot and quick (greens with butter, roasted carrots, a sharp salad), and something cool or sharp to cut through it (yoghurt, pickles, a squeeze of lemon). So a roast takes potatoes and greens, anything in a sauce takes rice or bread and a salad, and a barbecue takes coleslaw and pickles."));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-side-dish")); __args.push(py_tuple(vec![py_tuple(vec![py_str("side dish"), py_str("side dishes"), py_str("sides for"), py_str("sides to"), py_str("sides with"), py_str("a side for")]), py_tuple(vec![py_str("what"), py_str("which"), py_str("any"), py_str("idea"), py_str("ideas"), py_str("options"), py_str("suggest"), py_str("suggestion"), py_str("suggestions"), py_str("recommend"), py_str("recommendation"), py_str("give"), py_str("something"), py_str("serve"), py_str("make"), py_str("need"), py_str("want"), py_str("help"), py_str("pick"), py_str("choose")])])); __args.push(g__a_side_or_the_three_jobs()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("side effect"), py_str("side effects"), py_str("pain"), py_str("pain in my side"), py_str("stitch"), py_str("left side"), py_str("right side"), py_str("whose side"), py_str("side of the road"), py_str("sleep on")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s_COOKED_THROUGH_WORDS(py_tuple(vec![py_str("know"), py_str("knowing"), py_str("tell"), py_str("check"), py_str("checking"), py_str("done"), py_str("cooked"), py_str("cooked through"), py_str("ready"), py_str("how can i tell"), py_str("how do i tell"), py_str("how do you tell"), py_str("when is it done"), py_str("when its done"), py_str("internal"), py_str("temperature"), py_str("temp"), py_str("thermometer")]));
    s_NOT_A_DONENESS_QUESTION(py_tuple(vec![py_str("how long"), py_str("minutes"), py_str("kg"), py_str("per 500g"), py_str("recipe"), py_str("oven temperature"), py_str("gas mark"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("defrost"), py_str("thaw"), py_str("reheat"), py_str("left out"), py_str("overnight"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("already"), py_str("pregnant"), py_str("pregnancy"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("child"), py_str("children"), py_str("kids"), py_str("food poisoning"), py_str("sick"), py_str("ill"), py_str("off"), py_str("gone off"), py_str("fresh"), py_str("smell"), py_str("smells"), py_str("burnt"), py_str("burnt to"), py_str("overcooked"), py_str("dry"), py_str("tough"), py_str("rubbery"), py_str("recipe"), py_str("what can i make"), py_str("how do i cook"), py_str("how do you cook"), py_str("how to cook"), py_str("how do i bake"), py_str("how to bake"), py_str("how do i fry"), py_str("how to fry"), py_str("how do i roast"), py_str("how to roast"), py_str("how do i grill"), py_str("how to grill"), py_str("how should i cook"), py_str("best way to cook")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fish-done")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("cod"), py_str("haddock"), py_str("hake"), py_str("pollock"), py_str("sea bass"), py_str("tilapia")]), g_COOKED_THROUGH_WORDS()])); __args.push(py_str("Fish is done when it flakes at its thickest point and has turned from glassy to opaque all the way through, and a thermometer in the thickest part reads about 60C. Cod or haddock loins take 10 to 12 minutes at 200C, and a thin fillet 6 to 8. A fork should slide in with no resistance - if the middle still looks see-through, give it two more minutes rather than serving it early.")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_DONENESS_QUESTION())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pork-done")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pork"), py_str("pork chop"), py_str("pork chops"), py_str("chops"), py_str("loin"), py_str("gammon"), py_str("tenderloin")]), g_COOKED_THROUGH_WORDS()])); __args.push(py_str("Pork is done when the juices run clear where you pierce the thickest part and a thermometer reads 71C: a chop can keep the faintest blush of pink in the middle and still be safe, but not wet, jelly-like meat. A 2.5cm chop takes 4 minutes a side in a hot pan and 5 minutes resting, and a loin joint roasts 25 minutes per 500g at 200C (180C fan).")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_DONENESS_QUESTION())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lamb-done")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lamb"), py_str("leg of lamb"), py_str("lamb chops"), py_str("shoulder of lamb"), py_str("rack of lamb")]), g_COOKED_THROUGH_WORDS()])); __args.push(py_str("Lamb is done by temperature in the thickest part: 55C is pink, 60C is medium, 70C is well done, and it is juiciest if you take it out 5C under and rest it 10 minutes. A leg roasts 25 minutes per 500g at 180C (160C fan) plus 25 minutes on top, and a chop takes 3 minutes a side for pink.")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_DONENESS_QUESTION())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("beef-done")); __args.push(py_tuple(vec![py_tuple(vec![py_str("beef"), py_str("rib of beef"), py_str("rib roast"), py_str("roast beef"), py_str("beef joint")]), g_COOKED_THROUGH_WORDS()])); __args.push(py_str("Roast beef is 50C rare, 55C medium rare, 60C medium and 70C well done inside, read in the thickest part; it climbs about 5C while it rests, so pull it out 5C under what you want and rest it 15 minutes before carving, and at 220C (200C fan) allow 12 minutes per 500g. A steak is a pan question, not a roast one.")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_DONENESS_QUESTION())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turkey-done")); __args.push(py_tuple(vec![py_tuple(vec![py_str("turkey"), py_str("turkey crown")]), g_COOKED_THROUGH_WORDS()])); __args.push(py_str("A turkey is done when the thickest part of the thigh reads 74C and the juices run clear, not pink, where you pierce it; the breast reads about 70C by then and carries on cooking as it rests. Without a thermometer, the leg should move freely in its socket and the skin pull away easily - then rest it 30 minutes under foil.")); __kws.extend(vec![("avoid".to_string(), g_NOT_A_DONENESS_QUESTION())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("swap-dairy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dairy"), py_str("dairy free"), py_str("dairy-free"), py_str("no dairy"), py_str("milk free"), py_str("milk-free"), py_str("without dairy"), py_str("lactose"), py_str("lactose free"), py_str("lactose-free"), py_str("plant milk"), py_str("oat milk"), py_str("soya milk"), py_str("soy milk"), py_str("almond milk")]), add(&crate::gen::knowledge::cooking::g__SWAP_ASK(), &py_tuple(vec![py_str("how do i make"), py_str("how to make"), py_str("how can i make"), py_str("what can i eat"), py_str("cut out"), py_str("cutting out")]))])); __args.push(py_str("Every dairy ingredient has a straight swap: unsweetened soya or oat milk in place of milk, one for one, olive oil or a dairy-free spread for butter, and soya yoghurt for yoghurt. Where a recipe wants buttermilk, put a tablespoon of lemon juice in 250ml of plant milk and leave it ten minutes. Soya is the one with as much protein as milk, so pancakes, porridge and coffee taste least different with it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("formula"), py_str("baby"), py_str("babies"), py_str("newborn"), py_str("breastfeeding"), py_str("calcium"), py_str("cheese board"), py_str("how much calcium")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s_FATTENING_WORDS(py_tuple(vec![py_str("fattening"), py_str("make me fat"), py_str("makes me fat"), py_str("make you fat"), py_str("makes you fat"), py_str("put on weight"), py_str("gain weight"), py_str("fatten"), py_str("too fattening"), py_str("fatten me")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fattening-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("banana"), py_str("bananas"), py_str("potato"), py_str("potatoes"), py_str("bread"), py_str("pasta"), py_str("rice"), py_str("nuts"), py_str("cheese"), py_str("eggs"), py_str("egg"), py_str("avocado"), py_str("chocolate"), py_str("honey"), py_str("fruit"), py_str("oats"), py_str("porridge"), py_str("yoghurt"), py_str("yogurt"), py_str("olive oil"), py_str("butter"), py_str("milk"), py_str("smoothie"), py_str("smoothies"), py_str("juice")]), g_FATTENING_WORDS()])); __args.push(py_str("No single food is fattening - what decides your weight is the total across the day, so a banana at about 100 calories is a snack, not a cause. The foods blamed most often are the ones that go down fast and leave you hungry: bread, pasta, rice and juice. Eat them with protein and vegetables, or in one sitting rather than all day, and they stop being a problem.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("lose weight"), py_str("losing weight"), py_str("weight loss"), py_str("how much should i weigh"), py_str("bmi"), py_str("diabetes"), py_str("diabetic"), py_str("keto"), py_str("fasting"), py_str("calories in"), py_str("how many calories"), py_str("allergy"), py_str("allergic"), py_str("intolerant"), py_str("pregnant"), py_str("toddler"), py_str("baby"), py_str("dog"), py_str("cat")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("starter-name")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sourdough"), py_str("starter"), py_str("sourdough starter"), py_str("levain")]), py_tuple(vec![py_str("name"), py_str("named"), py_str("naming"), py_str("call it"), py_str("call him"), py_str("call her"), py_str("call my")])])); __args.push(py_str("Bread people go straight for the pun - Doughlene, Clint Yeastwood, Bread Pitt, Jane Dough - and everyone who asks about it wants the joke, so any name with dough, yeast or rise in it works. The starter does not care, and saying the name out loud once a day is a good reminder to feed it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("should i name"), py_str("should we name"), py_str("should i call"), py_str("should we call"), py_str("what should i name"), py_str("what should i call"), py_str("what to name"), py_str("what could i name"), py_str("suggest"), py_str("ideas"), py_str("idea"), py_str("any name"), py_str("good name"), py_str("nice name"), py_str("name for my"), py_str("name my"), py_str("help me name"), py_str("help me pick"), py_str("recommend"), py_str("come up with"), py_str("think of a name"), py_str("how do i name"), py_str("rename"), py_str("what would you call")])), ("avoid".to_string(), py_tuple(vec![py_str("feed"), py_str("feeding"), py_str("not rising"), py_str("didnt rise"), py_str("died"), py_str("hooch"), py_str("grey"), py_str("mould"), py_str("mold"), py_str("smell"), py_str("start"), py_str("begin"), py_str("first"), py_str("day"), py_str("wifi"), py_str("band"), py_str("cat"), py_str("dog"), py_str("discard"), py_str("flour"), py_str("water ratio")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pan-boil-over")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pan"), py_str("pans"), py_str("pot"), py_str("pots"), py_str("saucepan"), py_str("water"), py_str("milk"), py_str("pasta"), py_str("rice"), py_str("soup"), py_str("sauce"), py_str("stock"), py_str("porridge"), py_str("oats"), py_str("stew"), py_str("custard")]), py_tuple(vec![py_str("boil over"), py_str("boils over"), py_str("boiling over"), py_str("boiled over"), py_str("overflow"), py_str("overflows"), py_str("spill"), py_str("spills"), py_str("spilling"), py_str("spilled"), py_str("spilt"), py_str("stop it"), py_str("keep it from")])])); __args.push(py_str("Turn the heat down so it is a steady simmer rather than a rolling boil, and leave the lid half on. A wooden spoon laid across the top of the pan breaks the bubbles before they reach the rim, and a splash of oil or a knob of butter in starchy water (pasta, rice) stops the foam building. Fill the pan no more than two thirds and stir once at the start. Milk and tomato sauces catch on the base and then spatter, so stir them and keep them low.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("recipe"), py_str("how long"), py_str("burnt"), py_str("stuck"), py_str("sticks"), py_str("soggy"), py_str("kettle"), py_str("boiler"), py_str("central heating"), py_str("radiator"), py_str("water tank"), py_str("egg"), py_str("eggs"), py_str("floor"), py_str("floors"), py_str("sofa"), py_str("carpet"), py_str("rug"), py_str("cushion"), py_str("highchair"), py_str("bib"), py_str("remote"), py_str("remote control"), py_str("phone"), py_str("mobile"), py_str("iphone"), py_str("keyboard"), py_str("laptop"), py_str("tablet"), py_str("ipad"), py_str("kindle"), py_str("earbuds"), py_str("headphones"), py_str("controller"), py_str("console"), py_str("camera"), py_str("speaker"), py_str("charger"), py_str("tv")])), ("ask".to_string(), py_tuple(vec![py_str("boil"), py_str("boils"), py_str("boiling"), py_str("boiled"), py_str("overflow"), py_str("spill"), py_str("spills"), py_str("spilling"), py_str("spilled"), py_str("spilt"), py_str("stop"), py_str("stops"), py_str("stopping"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("prevent")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cream-whip")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cream"), py_str("double cream"), py_str("whipping cream"), py_str("single cream")]), py_tuple(vec![py_str("whip"), py_str("whipping"), py_str("whipped"), py_str("stiff"), py_str("peaks"), py_str("hand"), py_str("by hand"), py_str("overwhipped"), py_str("split"), py_str("grainy")])])); __args.push(py_str("Cold cream, a cold bowl and a big balloon whisk: whip in a steady circle, keeping the whisk under the surface so you beat air in rather than flick it over the side. Double cream takes 2 to 3 minutes by hand and stiffens just as the trails stop disappearing. Stop a moment before it looks ready, because it goes grainy fast; a splash of milk whisked in rescues one that has gone too far.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("ice cream"), py_str("sunburn"), py_str("eczema"), py_str("skin"), py_str("soup"), py_str("sauce"), py_str("coffee"), py_str("tea"), py_str("freeze"), py_str("frozen")])), ("ask".to_string(), py_tuple(vec![py_str("whip"), py_str("whipping"), py_str("whipped"), py_str("peaks"), py_str("stiff")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chicken-joint")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("whole chicken"), py_str("bird")]), py_tuple(vec![py_str("joint"), py_str("jointing"), py_str("portion"), py_str("portions"), py_str("cut up"), py_str("butcher"), py_str("eight pieces"), py_str("legs"), py_str("thighs"), py_str("breast off")])])); __args.push(py_str("Take the legs off first: cut through the skin between leg and body, bend the leg back until the hip joint pops, then cut through the middle of the joint rather than through bone. Then take the wings off at the shoulder, and cut each breast away from the backbone, following the ridge with the knife tip. A sharp knife and a pair of kitchen scissors for the joints do most of the work, and the carcass goes in the pot for stock.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("raw"), py_str("undercooked"), py_str("pink"), py_str("defrost"), py_str("freeze"), py_str("worth"), py_str("buy"), py_str("cheapest"), py_str("welfare")])), ("ask".to_string(), py_tuple(vec![py_str("joint"), py_str("jointing"), py_str("cut up"), py_str("eight pieces"), py_str("portions")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cake-tin-line")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cake tin"), py_str("cake tins"), py_str("muffin tin"), py_str("loaf tin"), py_str("sandwich tin"), py_str("tin"), py_str("muffins"), py_str("springform"), py_str("springform tin"), py_str("loose bottom tin")]), py_tuple(vec![py_str("grease"), py_str("greasing"), py_str("line"), py_str("lining"), py_str("parchment"), py_str("greaseproof"), py_str("stick"), py_str("sticking"), py_str("comes out"), py_str("release"), py_str("loosen"), py_str("flour"), py_str("floured"), py_str("dust"), py_str("dusting"), py_str("prepare"), py_str("preparing")])])); __args.push(py_str("Rub the tin with butter or a flavourless oil, then cut a circle of baking parchment for the base - a square folded into quarters and trimmed to the corner is the quick way - and grease that too. For muffins and small tins, grease every cup and dust with flour, or use paper cases. Run a thin knife round the edge the moment it comes out and leave the cake in the tin for 10 minutes before turning it out.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("roast"), py_str("roasting"), py_str("hob"), py_str("pan"), py_str("frying"), py_str("wok"), py_str("baking tray"), py_str("traybake"), py_str("bread"), py_str("loaf of bread"), py_str("fruit cake"), py_str("christmas cake"), py_str("rich fruit"), py_str("wedding cake"), py_str("simnel"), py_str("dark fruit"), py_str("bundt"), py_str("bundt tin"), py_str("dry"), py_str("dried out"), py_str("overcooked"), py_str("over-cooked"), py_str("crumbly"), py_str("flat"), py_str("didnt rise"), py_str("didn't rise"), py_str("sank"), py_str("soggy"), py_str("raw"), py_str("uncooked"), py_str("burnt"), py_str("burned")])), ("ask".to_string(), py_tuple(vec![py_str("grease"), py_str("greasing"), py_str("line"), py_str("lining"), py_str("parchment"), py_str("greaseproof"), py_str("stick"), py_str("sticking"), py_str("comes out"), py_str("release"), py_str("flour"), py_str("dust"), py_str("prepare"), py_str("should i"), py_str("do i need"), py_str("how do i")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("grease-a-tin-or-pan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pan"), py_str("pans"), py_str("tin"), py_str("tins"), py_str("tray"), py_str("trays"), py_str("mould"), py_str("moulds"), py_str("ramekin"), py_str("ramekins"), py_str("cake tin"), py_str("loaf tin"), py_str("muffin tin"), py_str("baking tin"), py_str("baking tray"), py_str("roasting tin"), py_str("frying pan"), py_str("grill pan"), py_str("wok"), py_str("dish"), py_str("loaf")]), py_tuple(vec![py_str("grease"), py_str("greased"), py_str("greasing"), py_str("greaseproof"), py_str("parchment"), py_str("baking paper")])])); __args.push(py_str("Rub a thin film of butter or flavourless oil all round the inside with a piece of kitchen paper. A cake tin also wants a dusting of flour, tapped round and the extra tipped out, or a lining of baking parchment on the base; a pan or a roasting tin wants only the oil, wiped on while it is cold.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washing"), py_str("scrub"), py_str("soak"), py_str("burnt"), py_str("stuck"), py_str("get off"), py_str("get the fat"), py_str("grease off"), py_str("remove"), py_str("degrease"), py_str("season"), py_str("how long"), py_str("oven temperature"), py_str("temperature"), py_str("recipe"), py_str("buy"), py_str("non stick"), py_str("non-stick"), py_str("scratched"), py_str("scratches"), py_str("scratches on"), py_str("safe"), py_str("unsafe"), py_str("replace"), py_str("coating"), py_str("flaking"), py_str("peeling"), py_str("still ok"), py_str("toxic")])), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("grease"), py_str("greasing"), py_str("non stick"), py_str("parchment")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("frittata")); __args.push(py_tuple(vec![py_tuple(vec![py_str("frittata"), py_str("Spanish omelette"), py_str("spanish tortilla"), py_str("omelette for a crowd"), py_str("tortilla espanola"), py_str("tortilla española"), py_str("potato tortilla")]), py_tuple(vec![py_str("make"), py_str("how"), py_str("oven"), py_str("bake"), py_str("pan"), py_str("eggs"), py_str("filling"), py_str("how many eggs"), py_str("serve")])])); __args.push(py_str("Cook the vegetables or potato first in an ovenproof frying pan until soft, then pour over 6 beaten eggs for a 20cm pan, seasoned and with a handful of cheese if you like. Keep it on a low heat for 3 to 4 minutes until the edges set, then finish it under the grill or in a 180C oven for 8 to 10 minutes until the middle is just set. It takes 6 eggs to feed four, and it is good cold the next day.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("frying pan"), py_str("non stick"), py_str("non-stick"), py_str("buy"), py_str("best"), py_str("pan set")])), ("ask".to_string(), py_tuple(vec![py_str("frittata"), py_str("spanish omelette"), py_str("spanish tortilla"), py_str("how many eggs"), py_str("tortilla espanola"), py_str("espanola"), py_str("espanol"), py_str("potato tortilla")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s__FLATBREAD_KINDS(py_tuple(vec![py_tuple(vec![py_str("tortilla"), py_tuple(vec![py_str("tortilla"), py_str("tortillas"), py_str("wrap"), py_str("wraps"), py_str("fajita"), py_str("burrito")])]), py_tuple(vec![py_str("chapati"), py_tuple(vec![py_str("chapati"), py_str("chapatis"), py_str("chapatti"), py_str("roti"), py_str("rotis")])]), py_tuple(vec![py_str("pitta"), py_tuple(vec![py_str("pitta"), py_str("pitas"), py_str("pita"), py_str("pittas")])])]));
    s__FLATBREAD_RECIPES(py_dict(vec![(py_str("tortilla"), py_str("Plain flour, salt, a little oil or lard and warm water, nothing else: about 250g of flour to 150ml of warm water and a tablespoon of oil. Knead for two minutes, rest the dough under a bowl for 20, then roll each one out as thin as you can. Cook in a dry, very hot pan for about 30 seconds a side - they want speckles, not browning, or they go crisp instead of foldable. Press the bubble down with a spatula as it puffs and stack them under a tea towel.")), (py_str("chapati"), py_str("Wholemeal flour and water, no raising agent and no yoghurt: about 200g of wholemeal or atta to 120ml of warm water, a pinch of salt and a smear of oil. Knead for five minutes until smooth, rest 20 minutes, roll thin, then cook in a dry pan for a minute a side until it speckles. Hold it over a flame with tongs for a couple of seconds at the end and it puffs up.")), (py_str("pitta"), py_str("Pitta needs yeast, because the pocket is steam trapped inside the dough: 250g of strong white flour, 150ml of warm water, 5g of instant yeast, 5g of salt and a teaspoon of oil. Knead, prove an hour until doubled, then roll out about 1cm thick - too thin and there is no steam to open it. Bake on a tray as hot as the oven goes, 250C, for about five minutes, and they puff and hollow as they cool.")), (py_str("quick"), py_str("Self-raising flour, a pinch of salt and enough yoghurt or water to make a soft dough: about 250g of flour to 150g of yoghurt, kneaded for a minute and rested for 10. Roll them thin, then cook each one in a dry, very hot frying pan for a minute a side until it blisters and puffs. Keep them under a tea towel while you do the rest, and eat them the same day; they freeze well between sheets of paper."))]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("flatbreads")); __args.push(py_tuple(vec![py_tuple(vec![py_str("flatbread"), py_str("flatbreads"), py_str("flat bread"), py_str("pitta"), py_str("pita"), py_str("tortilla wraps"), py_str("chapati"), py_str("naan"), py_str("tortilla"), py_str("tortillas"), py_str("corn tortilla"), py_str("flour tortilla")]), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("how"), py_str("dough"), py_str("pan"), py_str("quick"), py_str("no yeast"), py_str("no oven")])])); __args.push(_flatbread_answer(&[])); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("pizza"), py_str("shop"), py_str("buy"), py_str("buying"), py_str("freezer aisle"), py_str("gluten"), py_str("sourdough"), py_str("bread maker"), py_str("breadmaker")])), ("ask".to_string(), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("dough"), py_str("pan"), py_str("quick"), py_str("no yeast"), py_str("no oven")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lunchbox-cold")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lunchbox"), py_str("lunch box"), py_str("packed lunch"), py_str("lunch"), py_str("lunches"), py_str("sandwiches"), py_str("salad"), py_str("bento"), py_str("keep my food"), py_str("keep the food"), py_str("food cold"), py_str("food cool"), py_str("food at work")]), py_tuple(vec![py_str("cold"), py_str("cool"), py_str("cooler"), py_str("ice pack"), py_str("warm"), py_str("hot day"), py_str("stay fresh"), py_str("safe"), py_str("keep")])])); __args.push(py_str("Freeze a bottle of water or a small ice pack overnight and put it in the bag with the food, or freeze the sandwich fillings themselves (hummus, cooked chicken, cheese) and let them thaw by lunchtime. A bag with a lining holds the cold far better than a paper one, and the food wants to sit against the cold pack rather than next to it. Rice, cooked meat and dairy are the things to be strict about: over 5C for more than a couple of hours and they are not worth eating.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("school bag"), py_str("uniform"), py_str("homework"), py_str("kids won't"), py_str("refuse to eat"), py_str("soggy"), py_str("went soggy"), py_str("gone soggy"), py_str("going soggy"), py_str("dressing"), py_str("dressed"), py_str("wet"), py_str("food warm"), py_str("lunch warm"), py_str("keep it warm"), py_str("keep them warm"), py_str("stay warm"), py_str("still warm"), py_str("food hot"), py_str("lunch hot"), py_str("keep it hot"), py_str("hot lunch")])), ("ask".to_string(), py_tuple(vec![py_str("cold"), py_str("cool"), py_str("cooler"), py_str("ice pack"), py_str("warm"), py_str("hot day"), py_str("stay fresh"), py_str("safe"), py_str("keep")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("lunchbox-hot")); __args.push(py_tuple(vec![py_tuple(vec![py_str("lunchbox"), py_str("lunch box"), py_str("packed lunch"), py_str("lunch"), py_str("lunches"), py_str("food"), py_str("soup"), py_str("stew"), py_str("curry"), py_str("leftovers"), py_str("flask"), py_str("thermos"), py_str("pie"), py_str("pasta"), py_str("rice")]), py_tuple(vec![py_str("warm"), py_str("hot"), py_str("still warm"), py_str("steaming")])])); __args.push(py_str("Fill a wide-mouth flask with boiling water for five minutes, empty it, then put the food in steaming hot and it will still be warm at lunchtime; a tea towel round the flask adds an hour or two. The middle ground is the risky bit, because food between 5C and 63C grows bacteria, so reheat it until it is steaming just before it goes in and throw away what is left at the end of the day.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("cold"), py_str("cool"), py_str("ice"), py_str("freezer"), py_str("fridge"), py_str("hot day"), py_str("hot weather"), py_str("sunburn"), py_str("chilli"), py_str("chili"), py_str("spicy"), py_str("hot sauce"), py_str("fever"), py_str("temperature"), py_str("oven"), py_str("bake"), py_str("baking"), py_str("roast"), py_str("raw"), py_str("defrost"), py_str("reheat")])), ("ask".to_string(), py_tuple(vec![py_str("warm"), py_str("hot"), py_str("still warm"), py_str("steaming")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s__APPLE_BROWN(py_str("Toss the cut pieces in lemon or lime juice, or dip them in lightly salted water for a minute and pat them dry, and press cling film right onto the cut faces. It is an enzyme in the flesh meeting the air, nothing to do with going off, so a browned apple is safe - cut the marks off and eat it."));
    s__WET_BROWN(py_str("Drop them into a bowl of cold water as you cut them, with a squeeze of lemon if you have one, and they will stay pale until they are cooked. It is an enzyme in the flesh meeting the air rather than spoilage, so browned potato or aubergine is safe - trim the marks and carry on."));
    s__DRY_BROWN(py_str("Keep the cut face covered with cling film or a drop of lemon juice until you cook it, and wipe rather than wash if it has to wait, as water makes it slimy. The brown marks are only air on the flesh, so a browned avocado is safe - cut them away and use the rest."));
    s__MUSHROOM_BROWN(py_str("A mushroom darkens as it ages, and that on its own is not spoiling - brush it rather than wash it, keep it in a paper bag in the fridge, and cook it soon. Slime, a sour or fishy smell or dark wet patches mean it has gone, so bin those."));
    s__MUSHROOM_SAFE(py_str("Look at the texture, not the colour: a mushroom is fine while it is firm and dry with a pleasant smell, brown cap or not. Slimy skin, a sour or fishy smell, or dark wet patches mean bin it, and cooked mushrooms left out for more than two hours go too."));
    s__CELERY_BROWN(py_str("Stand cut sticks in a jar of cold water in the fridge, or wrap them tight, and they keep their colour for a week. Brown edges are just air and water loss, so trim them off and the rest is fine."));
    s__PRODUCE_BROWN(py_str("It is an enzyme in the cut flesh meeting the air, so the fix is to keep the air off or make it acid: lemon juice, a bowl of cold salted water, or cling film pressed onto the cut face. Brown patches are not spoilage - trim them and eat the rest."));
    s__BROWN_SAFE_ASKS(stdlib_call("re", "compile", &[py_str("\\bsafe\\b|\\bok\\b|\\bokay\\b|\\bstill good\\b|\\bcan i eat\\b|\\bcan you eat\\b|\\bthrow (it|them|away)\\b|\\bstill fine\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BROWN_SAFE(py_str("It is safe: browning is an enzyme in the cut flesh meeting the air, not spoilage, so trim the marks off and eat the rest. To stop it next time, keep the air off the cut face - lemon or lime juice, a bowl of cold salted water, or cling film pressed right onto it."));
    s__MUSHROOM_WORDS(stdlib_call("re", "compile", &[py_str("\\bmushrooms?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__BROWN_KINDS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears")]), g__APPLE_BROWN()]), py_tuple(vec![py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("aubergine"), py_str("aubergines"), py_str("eggplant"), py_str("parsnip"), py_str("parsnips"), py_str("artichoke"), py_str("artichokes"), py_str("salsify"), py_str("jerusalem artichoke")]), g__WET_BROWN()]), py_tuple(vec![py_tuple(vec![py_str("mushroom"), py_str("mushrooms"), py_str("chestnut mushrooms"), py_str("button mushrooms")]), g__MUSHROOM_BROWN()]), py_tuple(vec![py_tuple(vec![py_str("avocado"), py_str("avocados")]), g__DRY_BROWN()]), py_tuple(vec![py_tuple(vec![py_str("celery"), py_str("celeriac")]), g__CELERY_BROWN()])]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("produce-brown")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("potato"), py_str("potatoes"), py_str("aubergine"), py_str("aubergines"), py_str("eggplant"), py_str("mushroom"), py_str("mushrooms"), py_str("celery"), py_str("parsnip"), py_str("parsnips"), py_str("artichoke"), py_str("artichokes"), py_str("fruit"), py_str("fruits")]), py_tuple(vec![py_str("brown"), py_str("browning"), py_str("going brown"), py_str("went brown"), py_str("gone brown"), py_str("gone black"), py_str("gone dark"), py_str("going black"), py_str("going grey"), py_str("going gray"), py_str("gone grey"), py_str("gone gray"), py_str("turns brown"), py_str("turn brown"), py_str("turning brown"), py_str("go black"), py_str("goes black"), py_str("went black"), py_str("go dark"), py_str("oxidise"), py_str("oxidize"), py_str("oxidation"), py_str("discolour"), py_str("discolor"), py_str("keeps browning"), py_str("stop them going brown")])])); __args.push(g__produce_brown()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("tree"), py_str("trees"), py_str("plant"), py_str("plants"), py_str("garden"), py_str("growing"), py_str("grow"), py_str("leaf"), py_str("leaves"), py_str("soil"), py_str("basil"), py_str("lawn"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("roast"), py_str("roasted"), py_str("fry"), py_str("fried"), py_str("frying"), py_str("sear"), py_str("seared"), py_str("maillard"), py_str("pan"), py_str("hot oven"), py_str("oven"), py_str("meat"), py_str("chicken"), py_str("beef"), py_str("bread"), py_str("toast"), py_str("ripen"), py_str("ripe"), py_str("ripening"), py_str("banana"), py_str("bananas"), py_str("chips"), py_str("crisps")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s_WAKE_WORDS(py_tuple(vec![py_str("for the wake"), py_str("at the wake"), py_str("the wake"), py_str("funeral tea"), py_str("after the funeral"), py_str("funeral reception"), py_str("wake food"), py_str("wakes")]));
    s_HOLD_WORDS(py_tuple(vec![py_str("buffet"), py_str("party"), py_str("finger food"), py_str("sandwiches"), py_str("quiche"), py_str("sausage rolls"), py_str("plates"), py_str("dishes")]));
    s__HOT_HELD(stdlib_call("re", "compile", &[py_str("\\bwarm\\b|\\bhot\\b|\\bheat\\b|\\bheating\\b|\\bstay hot\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__COLD_HELD(stdlib_call("re", "compile", &[py_str("\\bcold\\b|\\bchill\\b|\\bfresh\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AHEAD(stdlib_call("re", "compile", &[py_str("\\badvance\\b|\\bahead\\b|\\bnight before\\b|\\bday before\\b|\\bmake ahead\\b|\\bprepare\\b|\\bprep\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__FOOD_IN_LINE(bitor(&bitor(&call_builtin("frozenset", &[crate::gen::knowledge::cooking::g_PANTRY()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[crate::gen::knowledge::cooking::g__INGREDIENT_WORDS()]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("frozenset", &[py_tuple(vec![py_str("food"), py_str("foods"), py_str("meal"), py_str("meals"), py_str("dish"), py_str("dishes"), py_str("cook"), py_str("cooking"), py_str("eat"), py_str("eaten"), py_str("buffet"), py_str("sandwiches"), py_str("sandwich"), py_str("quiche"), py_str("sausage roll"), py_str("sausage rolls"), py_str("finger food"), py_str("nibbles"), py_str("canapes"), py_str("cake"), py_str("soup"), py_str("plates"), py_str("platter"), py_str("spread"), py_str("catering"), py_str("menu"), py_str("recipe"), py_str("recipes"), py_str("bake"), py_str("baked"), py_str("roast"), py_str("salad"), py_str("cheese"), py_str("crisps"), py_str("dips"), py_str("vegetarian"), py_str("vegan"), py_str("picnic"), py_str("barbecue"), py_str("bbq"), py_str("pudding"), py_str("dessert"), py_str("starter")])]).unwrap_or_else(|e| exc::raise(e))));
    s__FOOD_JOB(stdlib_call("re", "compile", &[py_str("\\b(?:warm|hot|cold|chill|fresh|safe|soggy|sitting out|sat out|left out|stand out)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__OUR_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:im|i am|i'm|we are|we're|were|my|our|me and|my wife and i)\\b[^.]{0,24}\\b(?:having|throwing|hosting|organis|organiz|putting on|holding|planning)\\b|\\b(?:my|our|the) (?:party|barbecue|bbq|buffet|wake|christening|wedding) (?:food|spread)\\b|\\bfor \\d+ (?:people|guests|heads)\\b|\\bfor \\w+ (?:people|guests)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PRONOUN_HELD(stdlib_call("re", "compile", &[py_str("\\b(?:keep|keeps|keeping|kept|hold|holds|holding|stay|stays|staying|stand|stands|standing|serve|serves|serving|served|leave|leaves|leaving|left|put|puts|putting)\\s+(?:[a-z']+\\s+){0,2}(?:it|them|that|those|these|things|everything)\\b[^.]{0,24}\\b(?:warm|hot|cold|chill|chilled|fresh|safe)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wake-food")); __args.push(py_tuple(vec![g_WAKE_WORDS(), py_tuple(vec![py_str("cook"), py_str("food"), py_str("make"), py_str("serve"), py_str("bring"), py_str("sandwiches"), py_str("tea"), py_str("catering"), py_str("buffet"), py_str("much"), py_str("how many"), py_str("feed"), py_str("after"), py_str("reception")])])); __args.push(g__wake_food()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__about_the_food()), ("avoid".to_string(), py_tuple(vec![py_str("died"), py_str("death"), py_str("passed away"), py_str("grief"), py_str("grieving"), py_str("miss her"), py_str("miss him"), py_str("sympathy"), py_str("condolences"), py_str("what do i say"), py_str("what to say"), py_str("eulogy"), py_str("speech"), py_str("baby"), py_str("babies"), py_str("newborn"), py_str("newborns"), py_str("infant"), py_str("infants"), py_str("toddler"), py_str("toddlers"), py_str("my baby"), py_str("night feed"), py_str("night feeds"), py_str("waking for feeds"), py_str("weaning"), py_str("month old"), py_str("months old"), py_str("week old"), py_str("weeks old"), py_str("year old"), py_str("years old"), py_str("wake for a feed"), py_str("wakes for a feed"), py_str("wake for feeds")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("buffet-holding")); __args.push(py_tuple(vec![g_HOLD_WORDS(), py_tuple(vec![py_str("keep"), py_str("warm"), py_str("hot"), py_str("cold"), py_str("chill"), py_str("advance"), py_str("ahead"), py_str("night before"), py_str("day before"), py_str("make ahead"), py_str("prepare"), py_str("prep"), py_str("safe"), py_str("left out"), py_str("sit out"), py_str("stay fresh")])])); __args.push(g__buffet_holding()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("guard".to_string(), g__about_the_food()), ("avoid".to_string(), py_tuple(vec![py_str("ideas"), py_str("idea"), py_str("suggest"), py_str("suggestions"), py_str("what can i make"), py_str("what should i make"), py_str("menu"), py_str("cheap"), py_str("recipe"), py_str("recipes"), py_str("how much food"), py_str("how many sandwiches"), py_str("how much do i need"), py_str("how many do i need"), py_str("soggy"), py_str("going soggy"), py_str("goes soggy"), py_str("went soggy"), py_str("gone soggy"), py_str("soggy bottom"), py_str("wet"), py_str("dressing"), py_str("dressed"), py_str("kids"), py_str("kid"), py_str("children"), py_str("child"), py_str("toddlers"), py_str("toddler"), py_str("games"), py_str("game"), py_str("play"), py_str("playing"), py_str("activities"), py_str("activity"), py_str("busy"), py_str("entertain"), py_str("bored"), py_str("occupy"), py_str("neighbour"), py_str("neighbours"), py_str("neighbor"), py_str("neighbors"), py_str("next door"), py_str("next-door"), py_str("upstairs"), py_str("downstairs"), py_str("flat above"), py_str("flat below"), py_str("loud music"), py_str("noise"), py_str("noisy"), py_str("keeps having"), py_str("keeps throwing"), py_str("every weekend"), py_str("all night"), py_str("all hours")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s_STAPLES(call_builtin("frozenset", &[mcall(&py_str("garlic onion salt pepper oil butter flour milk stock lemon tomato\ncheese bread"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEAT_NAMES(call_builtin("frozenset", &[mcall(&py_str("chicken mince sausage bacon ham tuna salmon prawn"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREEN_NAMES(call_builtin("frozenset", &[mcall(&py_str("spinach pea broccoli courgette cabbage pepper"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_GREEN_IN_TEXT(py_tuple(vec![py_str("spinach"), py_str("peas"), py_str("pea "), py_str("broccoli"), py_str("courgette"), py_str("cabbage"), py_str("kale"), py_str("green bean"), py_str("mangetout"), py_str("greens")]));
    s_LIMIT_WORDS(call_builtin("frozenset", &[mcall(&py_str("quick quickly fast rush rushed hurry quickish simple easy easier\ncheaper cheap basic plain lighter light lightly"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SIMPLE_PHRASES(py_tuple(vec![py_str("nothing too fancy"), py_str("nothing fancy"), py_str("not too fancy"), py_str("nothing complicated"), py_str("nothing too complicated"), py_str("no fuss"), py_str("keep it simple"), py_str("plain and simple"), py_str("nothing special"), py_str("nothing flashy"), py_str("something plain"), py_str("plain food")]));
    s_HEAT_WORDS(call_builtin("frozenset", &[mcall(&py_str("spicy spice spicier spicy-hot hotter hottest chilli chillies chili\nchillis kick fiery heat"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HEAT_IN_TEXT(py_tuple(vec![py_str("chilli"), py_str("chili"), py_str("curry"), py_str("cajun"), py_str("paprika"), py_str("harissa"), py_str("hot sauce"), py_str("spice"), py_str("spiced"), py_str("curried"), py_str("spicy"), py_str("ginger"), py_str("cumin")]));
    s_HEAT_INGREDIENTS(py_tuple(vec![py_str("chilli"), py_str("chili"), py_str("chillies"), py_str("chillis"), py_str("cayenne"), py_str("paprika"), py_str("harissa"), py_str("sriracha"), py_str("jalapeno"), py_str("tabasco"), py_str("chilli powder"), py_str("curry powder"), py_str("hot sauce"), py_str("chilli flakes"), py_str("chilli sauce"), py_str("cajun")]));
    s__TASTE_RULED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:wont|won't|wouldnt|wouldn't|doesnt|doesn't|dont|don't|didnt|didn't|never|no|hates?|hated|dislikes?|disliked|refuses?|refused|not)\\b(?:\\s+(?:eat|eats|eating|to|anything|something|food|foods|stuff|things|thing|meals?|dishes|very|too|much|that|this|these|those|so|really|any|all|of|or|and|the|a|an|it|them|sort|kind|hot|fan|keen|into|like|likes|at))*\\s+(?:spicy|spiced|spicier|spice|chilli|chili|chillies|chillis|curry|curried|fiery)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__HOT_FOOD_RULED_OUT(stdlib_call("re", "compile", &[py_str("\\b(?:wont|won't|wouldnt|wouldn't|doesnt|doesn't|dont|don't|didnt|didn't|never|hates?|hated|dislikes?|refuses?|refused)\\b(?:\\s+(?:eat|eats|eating|anything|something|very|too|much|that|so|really|any))*\\s+hot\\s+(?:food|foods|stuff|things|sauce|curry|dishes?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_VEG_WORDS(call_builtin("frozenset", &[mcall(&py_str("vegetarian vegan veggie meatless"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEAT_FREE_PHRASES(py_tuple(vec![py_str("no meat"), py_str("without meat"), py_str("not eat meat"), py_str("doesnt eat meat"), py_str("doesn't eat meat"), py_str("dont eat meat"), py_str("don't eat meat"), py_str("off meat"), py_str("no fish"), py_str("no chicken")]));
    s_DAIRY_FREE_PHRASES(py_tuple(vec![py_str("no dairy"), py_str("dairy free"), py_str("dairy-free"), py_str("without dairy"), py_str("milk free"), py_str("milk-free"), py_str("no milk products"), py_str("off dairy"), py_str("lactose free"), py_str("lactose-free")]));
    s_DAIRY_FOODS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&crate::gen::knowledge::cooking::g_FOOD_CLASSES(), "items", &[])) { let __unpk5471 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk5471[0].clone(); let mut kinds: Py = __unpk5471[1].clone(); if py_truthy(&Py::Bool(py_contains(&kinds.clone(), &py_str("dairy")))) {  __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_COMING_FOR(py_tuple(vec![py_str("for dinner"), py_str("for tea"), py_str("for supper"), py_str("for lunch"), py_str("coming for"), py_str("coming over"), py_str("coming round"), py_str("coming to"), py_str("round for"), py_str("over for"), py_str("having people"), py_str("having friends"), py_str("having guests"), py_str("to cook for"), py_str("cooking for")]));
    s_EVENT_WORDS(call_builtin("frozenset", &[mcall(&py_str("guests guest party parties buffet crowd birthday anniversary\ncelebrate celebration"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MENU_PHRASES(py_tuple(vec![py_str("no idea what to cook"), py_str("what to cook"), py_str("what should i cook"), py_str("what to make"), py_str("what shall i cook"), py_str("cook for"), py_str("cooking for"), py_str("make for")]));
    s_BIG_DAYS(call_builtin("frozenset", &[mcall(&py_str("christmas xmas thanksgiving easter hanukkah diwali ramadan eid"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_COOKING_FOR(py_tuple(vec![py_str("having a"), py_str("having some"), py_str("im making"), py_str("i'm making"), py_str("were making"), py_str("we're making"), py_str("im cooking"), py_str("i'm cooking"), py_str("for a"), py_str("for the"), py_str("for my"), py_str("party food"), py_str("buffet"), py_str("celebrat"), py_str("hosting"), py_str("come over"), py_str("coming over")]));
    s_PEOPLE_WORDS(call_builtin("frozenset", &[mcall(&py_str("mum mom dad family friends visitors mother father parents sister\nbrother inlaws in-laws boyfriend girlfriend"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CROWD_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:people|friends|family|guests?|company)\\s+(?:coming\\s+)?over\\b|\\b(?:for|feeding|there are|there's|theres|we are|we're)\\s+(?:\\d+|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fourteen|fifteen|twenty|thirty|a dozen)\\b|\\b\\d+\\s+of\\s+us\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_DISLIKE_WORDS(call_builtin("frozenset", &[mcall(&py_str("hate hates hated dont don't doesnt doesn't wont won't wouldnt\nwouldn't dislike dislikes picky refuse refuses"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SOMEONE_ELSE(stdlib_call("re", "compile", &[py_str("\\b(?:my|his|her|our|their|the|one)\\s+(?:daughter|son|kid|kids|child|children|boy|girl|teenager|husband|wife|partner|mum|mom|dad|father|mother|brother|sister|friend|friends|nan|gran|granny|grandma|grandad|grandpa|guest|guests|inlaws|in-laws|other half|mother in law|father in law)\\b|\\b(?:he|she|they)\\s+(?:wont|won't|doesnt|doesn't|refuses|hates|dislikes|isnt|isn't)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GREEN_WORDS(call_builtin("frozenset", &[mcall(&py_str("green greens"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PLAN_LAPSE(Py::Int(2));
    s_MINUTES(py_dict(vec![(py_str("potato-egg-hash"), Py::Int(15)), (py_str("courgette-mushroom-pasta"), Py::Int(20)), (py_str("chicken-spinach-cream"), Py::Int(20)), (py_str("frittata"), Py::Int(20)), (py_str("egg-fried-rice"), Py::Int(10)), (py_str("cheese-omelette"), Py::Int(8)), (py_str("mushroom-omelette"), Py::Int(10)), (py_str("courgette-frittata"), Py::Int(20)), (py_str("ham-omelette"), Py::Int(8)), (py_str("tomato-pasta"), Py::Int(15)), (py_str("tuna-pasta"), Py::Int(15)), (py_str("courgette-pasta"), Py::Int(15)), (py_str("mushroom-pasta"), Py::Int(15)), (py_str("bacon-pasta"), Py::Int(15)), (py_str("mac-cheese"), Py::Int(20)), (py_str("pea-pasta"), Py::Int(12)), (py_str("garlic-spaghetti"), Py::Int(10)), (py_str("chicken-rice"), Py::Int(30)), (py_str("leftover-chicken-rice"), Py::Int(10)), (py_str("chicken-potato-tray"), Py::Int(45)), (py_str("chickpea-curry"), Py::Int(20)), (py_str("chickpea-spinach"), Py::Int(15)), (py_str("lentil-soup"), Py::Int(30)), (py_str("dhal"), Py::Int(25)), (py_str("sausage-potato"), Py::Int(40)), (py_str("sausage-bean"), Py::Int(30)), (py_str("bolognese"), Py::Int(35)), (py_str("cottage-pie"), Py::Int(50)), (py_str("mince-pasta"), Py::Int(25)), (py_str("salmon-potato"), Py::Int(40)), (py_str("prawn-pasta"), Py::Int(15)), (py_str("tuna-potato"), Py::Int(20)), (py_str("bean-quesadilla"), Py::Int(5)), (py_str("cheese-quesadilla"), Py::Int(5)), (py_str("toastie"), Py::Int(5)), (py_str("eggy-bread"), Py::Int(5)), (py_str("pancakes"), Py::Int(20)), (py_str("shortbread"), Py::Int(30)), (py_str("banana-pancakes"), Py::Int(10)), (py_str("banana-smoothie"), Py::Int(2)), (py_str("porridge"), Py::Int(5)), (py_str("overnight-oats"), Py::Int(480)), (py_str("apple-crumble"), Py::Int(45)), (py_str("veg-soup"), Py::Int(30)), (py_str("veg-stirfry"), Py::Int(10)), (py_str("cheesy-mash"), Py::Int(20)), (py_str("mince-onion"), Py::Int(25)), (py_str("onion-omelette"), Py::Int(12)), (py_str("fried-potato-onion"), Py::Int(20)), (py_str("chicken-onion-curry"), Py::Int(35)), (py_str("loaded-jacket"), Py::Int(30)), (py_str("shakshuka"), Py::Int(25)), (py_str("spinach-pasta"), Py::Int(15)), (py_str("bean-tomato"), Py::Int(20)), (py_str("tuna-rice"), Py::Int(10)), (py_str("prawn-rice"), Py::Int(15)), (py_str("lemon-chicken"), Py::Int(45)), (py_str("mash-cheese"), Py::Int(10)), (py_str("scrambled-eggs"), Py::Int(5)), (py_str("mushroom-cream"), Py::Int(10))]));
    s_PANTRY_MINUTES(py_dict(vec![(py_str("egg"), Py::Int(5)), (py_str("pasta"), Py::Int(10)), (py_str("rice"), Py::Int(10)), (py_str("potato"), Py::Int(50)), (py_str("chicken"), Py::Int(40)), (py_str("mince"), Py::Int(25)), (py_str("sausage"), Py::Int(25)), (py_str("bacon"), Py::Int(15)), (py_str("ham"), Py::Int(5)), (py_str("tuna"), Py::Int(15)), (py_str("salmon"), Py::Int(15)), (py_str("prawn"), Py::Int(10)), (py_str("chickpea"), Py::Int(30)), (py_str("bean"), Py::Int(5)), (py_str("lentil"), Py::Int(25)), (py_str("cheese"), Py::Int(5)), (py_str("tomato"), Py::Int(15)), (py_str("onion"), Py::Int(25)), (py_str("garlic"), Py::Int(5)), (py_str("mushroom"), Py::Int(10)), (py_str("spinach"), Py::Int(5)), (py_str("courgette"), Py::Int(15)), (py_str("pepper"), Py::Int(10)), (py_str("carrot"), Py::Int(25)), (py_str("broccoli"), Py::Int(15)), (py_str("cabbage"), Py::Int(10)), (py_str("pea"), Py::Int(15)), (py_str("sweetcorn"), Py::Int(10)), (py_str("bread"), Py::Int(5)), (py_str("tortilla"), Py::Int(5)), (py_str("flour"), Py::Int(25)), (py_str("milk"), Py::Int(10)), (py_str("cream"), Py::Int(10)), (py_str("butter"), Py::Int(30)), (py_str("yoghurt"), Py::Int(2)), (py_str("lemon"), Py::Int(5)), (py_str("banana"), Py::Int(10)), (py_str("apple"), Py::Int(15)), (py_str("oats"), Py::Int(5))]));
    s_CROWD_DISHES(call_builtin("frozenset", &[py_tuple(vec![py_str("bolognese"), py_str("cottage-pie"), py_str("chicken-potato-tray"), py_str("chickpea-curry"), py_str("dhal"), py_str("sausage-bean"), py_str("veg-soup"), py_str("mac-cheese"), py_str("salmon-potato"), py_str("lemon-chicken"), py_str("apple-crumble"), py_str("frittata")])]).unwrap_or_else(|e| exc::raise(e)));
    s_FOR_ONE(call_builtin("frozenset", &[py_tuple(vec![py_str("frittata"), py_str("courgette-frittata"), py_str("cheese-omelette"), py_str("mushroom-omelette"), py_str("onion-omelette"), py_str("ham-omelette"), py_str("toastie"), py_str("scrambled-eggs"), py_str("egg-fried-rice"), py_str("bean-quesadilla"), py_str("cheese-quesadilla"), py_str("eggy-bread"), py_str("banana-pancakes"), py_str("pancakes"), py_str("porridge"), py_str("overnight-oats")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THIN(call_builtin("frozenset", &[py_tuple(vec![py_str("veg-soup"), py_str("lentil-soup")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SWEET_WORDS(call_builtin("frozenset", &[mcall(&py_str("pudding puddings dessert desserts afters sweet-tooth"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SWEET_PHRASES(py_tuple(vec![py_str("something sweet"), py_str("something warm for after"), py_str("something for after"), py_str("for after"), py_str("to follow"), py_str("after that"), py_str("sweet course"), py_str("for pudding"), py_str("for dessert"), py_str("for afters")]));
    s_SWEET_DISHES(call_builtin("frozenset", &[py_tuple(vec![py_str("apple-crumble"), py_str("pancakes"), py_str("shortbread"), py_str("banana-pancakes"), py_str("banana-smoothie"), py_str("porridge"), py_str("overnight-oats")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DINNER_WORDS(call_builtin("frozenset", &[mcall(&py_str("dinner tea supper lunch brunch meal meals food cook cooking make made\ntonight eat eating"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_CHATTER(call_builtin("frozenset", &[mcall(&py_str("thanks thank ok okay lol haha yes yeah no nope sure great nice cool\nhi hello bye morning night fine"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TECHNIQUE_WORDS(call_builtin("frozenset", &[mcall(&py_str("time timing temperature oven degrees gas grill boil boiling roast\nroasting fry frying bake baking rest resting cover covered lid stir seasoning"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BAKING_WORDS(call_builtin("frozenset", &[mcall(&py_str("cake cakes bake baking baked sponge icing buttercream biscuit\nbiscuits cookie cookies muffin muffins cupcake cupcakes scone scones pastry loaf bread brownie\nbrownies tart drizzle frosting"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DISH_WORDS(call_builtin("frozenset", &[mcall(&py_str("curry curries pasta soup soups pie pies stew roast salad sandwich\nsandwiches risotto casserole burger pizza traybake omelette frittata macaroni lasagne"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEAL_FRAME(call_builtin("frozenset", &[mcall(&py_str("dinner tea supper lunch brunch meal meals food cook cooking cooked eat\n eating tonight make pudding dessert\n something anything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_PHRASES(py_tuple(vec![py_str("what did i say"), py_str("what did i tell"), py_str("did i tell you"), py_str("what did you say"), py_str("remind me what"), py_str("what have i said"), py_str("did i say")]));
    s_PLAN_KEYS(py_tuple(vec![py_str("have"), py_str("avoid"), py_str("limit"), py_str("veg"), py_str("vegan"), py_str("occasion"), py_str("simple"), py_str("no_green"), py_str("sweet"), py_str("crowd"), py_str("heat"), py_str("no_heat")]));
    s_AVOID_TEXT(py_dict(vec![(py_str("cheese"), py_tuple(vec![py_str("cheese"), py_str("parmesan"), py_str("cheddar"), py_str("mozzarella"), py_str("feta"), py_str("halloumi"), py_str("pecorino"), py_str("gruyere"), py_str("blue cheese")])), (py_str("dairy"), call_builtin("tuple", &[call_builtin("sorted", &[crate::gen::knowledge::cooking::g_DAIRY()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("milk"), py_tuple(vec![py_str("milk"), py_str("cream"), py_str("butter"), py_str("yoghurt"), py_str("yogurt"), py_str("creme fraiche")])), (py_str("butter"), py_tuple(vec![py_str("butter")])), (py_str("egg"), py_tuple(vec![py_str("egg"), py_str("eggs")])), (py_str("meat"), call_builtin("tuple", &[call_builtin("sorted", &[g_MEAT_NAMES()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("fish"), py_tuple(vec![py_str("fish"), py_str("tuna"), py_str("salmon"), py_str("prawn"), py_str("anchovy"), py_str("cod")]))]));
    s_VEGAN_BAD(py_tuple(vec![py_str("cheese"), py_str("parmesan"), py_str("cheddar"), py_str("mozzarella"), py_str("feta"), py_str("halloumi"), py_str("milk"), py_str("cream"), py_str("butter"), py_str("yoghurt"), py_str("yogurt"), py_str("egg"), py_str("eggs"), py_str("honey"), py_str("creme fraiche")]));
    s_PERSON_WORDS(call_builtin("frozenset", &[mcall(&py_str("she he her his him they their them son daughter boy girl child\nchildren kid kids toddler baby mum mom mummy daddy husband wife partner friend nan nana grandma\ngranny grandad niece nephew"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TASTE_WORDS(call_builtin("frozenset", &[mcall(&py_str("likes like loves love hates hate hated prefers prefer adores adore\nfavourite favorite keen fussy refuses refused"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SWAP_PHRASES(py_tuple(vec![py_str("instead"), py_str("substitute"), py_str("stand in"), py_str("replace"), py_str("swap")]));
    s_METHOD_PHRASES(py_tuple(vec![py_str("how do i"), py_str("how do you"), py_str("how should i"), py_str("how long"), py_str("how much"), py_str("what temperature"), py_str("how to")]));
    s_NEW_MEAL_PHRASES(py_tuple(vec![py_str("what can i cook"), py_str("what can i make"), py_str("what should i cook"), py_str("what should i make"), py_str("what can i do with"), py_str("meal ideas"), py_str("dinner ideas"), py_str("any ideas"), py_str("what shall i cook")]));
    s__GROWING_WORD(call_builtin("frozenset", &[mcall(&py_str("plant plants planted planting seedling seedlings seed seeds\n    tree trees bush bushes shrub shrubs crop crops allotment allotments garden greenhouse\n    growing grow grows grown sow sowed sowing"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STOCK_PHRASES(py_tuple(vec![py_str("have"), py_str("has"), py_str("got"), py_str("there is"), py_str("theres"), py_str("there are"), py_str("left over"), py_str("leftover"), py_str("in the fridge"), py_str("in the freezer"), py_str("in the cupboard"), py_str("in the pantry"), py_str("in the house"), py_str("in my kitchen"), py_str("at home")]));
    s_ALLERGY_WORDS(call_builtin("frozenset", &[mcall(&py_str("allergic allergy allergies intolerant intolerance anaphylactic anaphylaxis"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ALLERGY_PHRASES(py_tuple(vec![py_str("cant eat"), py_str("can't eat"), py_str("cannot eat"), py_str("must avoid"), py_str("have to avoid"), py_str("no nuts at all"), py_str("nut free diet")]));
    s_A_WISH_FOR_A_FOOD(stdlib_call("re", "compile", &[py_str("\\b(?:eat|eating|ate|get|getting|want|wants|wanted|need|needs|like|likes|try to eat|trying to eat|cut down on|cut back on)\\s+(more|less|fewer)\\s+(?:of\\s+)?([a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    s_QUICK_MINUTES(Py::Int(20));
    s__ABOUT_A_FOOD(py_tuple(vec![py_str("what about"), py_str("how about"), py_str("what can i do with"), py_str("what else with"), py_str("instead of"), py_str("without")]));
    s_ABSENCE(stdlib_call("re", "compile", &[py_str("\\b(?:havent|haven't|dont have|don't have|didnt have|didn't have|ran out|run out|out of|got any|no)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_MEAL_NOUNS(py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("recipe"), py_str("recipes")]));
    s_BARE_MEAL_OBJECT(stdlib_call("re", "compile", &[py_str("\\b(?:ideas?|recipes?)\\s+(?:for|with|using|about|on)\\s+(.*)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_FOOD_IN_HAND(call_builtin("frozenset", &[mcall(&py_str("it them those these this rest leftover leftovers other left using use"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ONLY_PHRASES(py_tuple(vec![py_str("only have"), py_str("only got"), py_str("only have got"), py_str("nothing but"), py_str("all i have"), py_str("all i've got"), py_str("all ive got"), py_str("just have"), py_str("thats all i"), py_str("that's all i")]));
    s_WITH_PHRASES(py_tuple(vec![py_str("have with it"), py_str("have with that"), py_str("eat with it"), py_str("goes with"), py_str("go with it"), py_str("serve with it"), py_str("serve it with"), py_str("served with"), py_str("on the side"), py_str("alongside"), py_str("what can i have"), py_str("what should i have"), py_str("with that meal")]));
    s_PICK_PHRASES(py_tuple(vec![py_str("you pick"), py_str("you choose"), py_str("you decide"), py_str("your call"), py_str("your choice"), py_str("up to you"), py_str("whatever you think"), py_str("whatever you like"), py_str("i don't mind"), py_str("i dont mind"), py_str("either is fine"), py_str("pick one"), py_str("you tell me")]));
    s__A_REACTION(py_tuple(vec![py_str("nut free diet"), py_str("no nuts at all"), py_str("no dairy at all"), py_str("carry an epipen")]));
    s_ACK_WORDS(call_builtin("frozenset", &[mcall(&py_str("ok okay k cool nice great good fine fair sure right true exactly yeah yep\nyes no nah thanks thank cheers ta please sorry oh ah mm hmm mmm got get gotcha noted understood\nmakes sense sounds sound will do try trying then so and but that this it its low high heat slow\nquickly gently careful huh righto alright perfect ace brill lovely accepted"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_SUM_OF_MONEY(stdlib_call("re", "compile", &[py_str("£\\s*[0-9]|\\b[0-9][0-9,]*(?:\\.[0-9]+)?\\s*(?:pounds?|quid|bucks)\\b|\\b(?:tenner|fiver|score)\\b|\\b(?:a|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred)\\s+(?:pounds?|quid|bucks)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_MEAL_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("\n    what shall should i we you can could do to for now then tonight tonights have eat make cook\n    cooking dinner tea supper lunch meal meals something anything a an the some quick easy simple\n    "), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_REFUSAL(stdlib_call("re", "compile", &[py_str("\\b(?:won['\\u2019]?t|wouldn['\\u2019]?t|don['\\u2019]?t|doesn['\\u2019]?t|didn['\\u2019]?t|can['\\u2019]?t stand|hates?|hated|dislikes?|refuses?|picky|not eat|no thanks)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_WAY_TO_COOK(stdlib_call("re", "compile", &[py_str("\\b(?:best|quickest|fastest|easiest|simplest|cheapest|healthiest|healthier|safest|right|proper|good|nice) way (?:to|of)\\b|\\bthe way (?:to|of) (?:cook|cooking|make|making|bake|baking|roast|boil|fry|steam)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FOOD_PROPERTY(stdlib_call("re", "compile", &[py_str("\\b(?:gluten|wheat|dairy|lactose|calories|calorie|carbs|carb|protein|fat|fattening|sugar|salt|healthy|healthier|safe|vegan|vegetarian|allergy|allergic|coeliac|ok|okay|fine|alright|good|bad|worth it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_COOK_REQUEST(stdlib_call("re", "compile", &[py_str("\\b(?:cook|cooks|cooking|cooked|make|makes|making|bake|recipe|recipes|dinner|teatime|supper|lunch|meal|meals|feed|serve)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THIN_CUPBOARD_PHRASES(py_tuple(vec![py_str("with what i have"), py_str("with what i've got"), py_str("with what ive got"), py_str("what i have in"), py_str("in the cupboard"), py_str("in the fridge"), py_str("in the pantry"), py_str("left in the")]));
    s_CHILLI_FRUIT(py_tuple(vec![py_str("chilli"), py_str("chillies"), py_str("chili"), py_str("chilies"), py_str("chile"), py_str("jalapeno"), py_str("jalapenos"), py_str("serrano"), py_str("birdseye"), py_str("scotch bonnet"), py_str("habanero"), py_str("red chilli"), py_str("green chilli"), py_str("pepper"), py_str("peppers"), py_str("bell pepper"), py_str("bell peppers"), py_str("red pepper"), py_str("green pepper"), py_str("sweet pepper"), py_str("chilli pepper"), py_str("chili pepper")]));
    s_SEED_FRUIT(add(&g_CHILLI_FRUIT(), &py_tuple(vec![py_str("pomegranate"), py_str("pomegranates"), py_str("tomato"), py_str("tomatoes"), py_str("cucumber"), py_str("cucumbers"), py_str("courgette"), py_str("courgettes"), py_str("marrow"), py_str("pumpkin"), py_str("squash"), py_str("butternut"), py_str("melon"), py_str("melons"), py_str("watermelon"), py_str("aubergine"), py_str("eggplant")])));
    s__CHILLI_SEEDS(py_str("Cut the stalk off and halve it lengthways, then scrape the seeds and the pale pith out with the tip of a knife or a teaspoon - that pith is where most of the heat sits, so take it out too if you want the chilli mild. Slice it on a board, not in your hand, and keep your hands away from your eyes and your mouth. Wash the board, the knife and your hands in cold water and soap straight after: the oil that burns is not washed off by rinsing. If your hands do burn, rub them with oil then wash again, or soak them in milk, not water."));
    s__POMEGRANATE_SEEDS(py_str("Cut the crown off and score the skin down the ridges, then break it open in a bowl of water and knock the seeds out with a wooden spoon: the seeds sink, the pith and the membrane float and you can skim them off. Doing it under water is what stops the juice reaching your shirt, and it is much quicker than picking them out."));
    s__TOMATO_SEEDS(py_str("Halve it across the middle, not top to bottom, and squeeze or scoop the seeds out with a finger or a teaspoon. The jelly round them is the flavour, so keep it if you are cooking; for a sandwich or a salad, a sieve catches the juice and you can salt the halves and leave them upside down for ten minutes instead."));
    s__CUCUMBER_SEEDS(py_str("Halve it lengthways and run a teaspoon down the middle to scoop the watery seeds out - a grapefruit spoon has the right edge for it. Salt the halves and leave them face down for fifteen minutes afterwards and the water comes out too, which is what a soggy sandwich is."));
    s__MELON_SEEDS(py_str("Halve it and scoop the seeds out with a spoon, then rinse the last of the strings off under the tap. A watermelon is easier cut into wedges and the seeds flicked out with the tip of a knife, and if you want it neat, cut it in slices and stamp the seeds out with a small round cutter."));
    s__SQUASH_SEEDS(py_str("Halve it and scoop the strings and seeds out with a metal spoon, working round the edge towards the middle. Keep the seeds: rinse them, toss in a little oil and salt and roast them at 180C for 10 to 15 minutes."));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("deseed-chilli")); __args.push(py_tuple(vec![g_SEED_FRUIT(), py_tuple(vec![py_str("deseed"), py_str("de-seed"), py_str("deseeding"), py_str("seeds"), py_str("seed"), py_str("seedless"), py_str("membrane"), py_str("pith"), py_str("core"), py_str("coring"), py_str("take out"), py_str("take the seeds out"), py_str("remove the seeds"), py_str("remove seeds"), py_str("get rid of the seeds"), py_str("less hot"), py_str("not so hot"), py_str("milder"), py_str("mild"), py_str("heat")])])); __args.push(g__deseed()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("chilli con carne"), py_str("con carne"), py_str("recipe"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("plants"), py_str("seedling"), py_str("sow"), py_str("sowing"), py_str("germinate"), py_str("sauce"), py_str("oil"), py_str("powder"), py_str("flakes"), py_str("paste"), py_str("jam"), py_str("pickle"), py_str("pickled"), py_str("dry"), py_str("dried"), py_str("freeze"), py_str("freezer"), py_str("keep"), py_str("store"), py_str("storage"), py_str("shelf life"), py_str("go off"), py_str("best variety"), py_str("which chilli"), py_str("scoville"), py_str("how hot is"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("toast"), py_str("toasted"), py_str("salt"), py_str("salted"), py_str("snack"), py_str("saving"), py_str("save the seeds"), py_str("harvest"), py_str("collect the seeds")])), ("ask".to_string(), py_tuple(vec![py_str("how do i"), py_str("how to"), py_str("how can i"), py_str("should i"), py_str("do i need to"), py_str("is it better"), py_str("deseed"), py_str("de-seed"), py_str("seeds"), py_str("membrane"), py_str("pith")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("umami")); __args.push(py_tuple(vec![py_tuple(vec![py_str("umami"), py_str("the fifth taste"), py_str("savoury taste"), py_str("savory taste")]), py_tuple(vec![py_str("what"), py_str("how"), py_str("explain"), py_str("mean"), py_str("get"), py_str("add"), py_str("more"), py_str("boost"), py_str("taste"), py_str("tasteless"), py_str("food"), py_str("dish"), py_str("cook"), py_str("put"), py_str("sauce"), py_str("good"), py_str("source"), py_str("sources")])])); __args.push(py_str("Umami is the savoury taste that makes food taste deep and round rather than just salty: it comes from glutamates, and the everyday sources are soy sauce, fish sauce, miso, tomato paste, mushrooms, hard cheese, anchovies, seaweed, stock cubes and slow-cooked meat. It is what a dish is missing when it is seasoned but still flat, so add a spoon of one of those rather than more salt, and give it a squeeze of lemon or a splash of vinegar to make it show. Parmesan, tomato and mushroom together are why a good ragu tastes bigger than its ingredients.")); __kws.extend(vec![("explain".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s_SLOT_IDEAS(py_tuple(vec![py_tuple(vec![py_str("brunch"), py_str("Brunch is one thing on the table people help themselves to: scrambled eggs on toast with tomatoes and something green, a frittata cut into wedges, or pancakes with fruit and yoghurt. For more than four, baked eggs in one dish with bread beside it - fried eggs for eight are cold before the last one is done.")]), py_tuple(vec![py_str("lunch"), py_str("Two shapes that work: a sandwich with something wet in it - hummus, tuna mayo, leftover roast - plus fruit, or soup with bread. The real answer is cooking extra at dinner: cold roast chicken, or lentils with feta, is a five-minute lunch the next day.")]), py_tuple(vec![py_str("snack"), py_str("One thing with protein and one with fibre holds you until the next meal: apple with peanut butter, yoghurt with fruit, toast with cheese, or a handful of nuts.")])]));
    s_SLOT_ASKS(py_tuple(vec![py_str("what should i"), py_str("what can i"), py_str("what shall i"), py_str("what do i"), py_str("ideas"), py_str("any ideas"), py_str("suggest"), py_str("suggestions"), py_str("make"), py_str("have"), py_str("eat"), py_str("cook"), py_str("fix"), py_str("quick"), py_str("easy")]));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("meal-idea-for-a-slot")); __args.push(py_tuple(vec![py_tuple(vec![py_str("brunch"), py_str("brunches"), py_str("snack"), py_str("snacks"), py_str("elevenses"), py_str("what should i make for lunch"), py_str("what can i make for lunch"), py_str("what shall i make for lunch"), py_str("what do i make for lunch")]), g_SLOT_ASKS()])); __args.push(g__a_meal_for_the_slot()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("dinner"), py_str("tea"), py_str("tonight"), py_str("roast"), py_str("chicken"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("fish"), py_str("salmon"), py_str("pasta"), py_str("rice"), py_str("egg"), py_str("eggs"), py_str("potato"), py_str("potatoes"), py_str("bread"), py_str("cheese"), py_str("soup"), py_str("salad"), py_str("cake"), py_str("biscuit"), py_str("pancake"), py_str("porridge"), py_str("toast"), py_str("hummus"), py_str("sandwich"), py_str("curry"), py_str("chilli"), py_str("stew"), py_str("pie"), py_str("pizza"), py_str("noodles"), py_str("quinoa"), py_str("how long"), py_str("how do i cook"), py_str("how do i make"), py_str("recipe"), py_str("freeze"), py_str("leftover"), py_str("leftovers"), py_str("calories"), py_str("healthy"), py_str("diet"), py_str("batch"), py_str("pudding"), py_str("dessert"), py_str("birthday"), py_str("kids"), py_str("toddler"), py_str("baby"), py_str("weaning"), py_str("school lunch"), py_str("lunchbox"), py_str("restaurant"), py_str("takeaway"), py_str("book"), py_str("table for")])), ("guard".to_string(), g__one_person_at_this_meal()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    s_ON_WORDS(py_tuple(vec![py_str("put on"), py_str("to put on"), py_str("put on it"), py_str("put on them"), py_str("what can i put"), py_str("what to put"), py_str("goes on"), py_str("go on"), py_str("have on"), py_str("add to"), py_str("add on"), py_str("add in"), py_str("topping"), py_str("toppings"), py_str("topping ideas"), py_str("on top")]));
    s_NOT_A_TOPPING_ASK(py_tuple(vec![py_str("how long"), py_str("how do i cook"), py_str("how do you cook"), py_str("how to cook"), py_str("how do i make"), py_str("how do you make"), py_str("recipe"), py_str("from scratch"), py_str("calories"), py_str("freeze"), py_str("frozen"), py_str("defrost"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("fridge"), py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("buy"), py_str("cheaper"), py_str("price"), py_str("cost"), py_str("substitute"), py_str("instead of"), py_str("how much"), py_str("healthy"), py_str("diet"), py_str("make with"), py_str("make from"), py_str("batch"), py_str("bake"), py_str("roast"), py_str("boil"), py_str("fry"), py_str("grill"), py_str("shop"), py_str("supermarket"), py_str("brand"), py_str("best buy"), py_str("how many")]));
    s_ON_TOPPINGS(py_tuple(vec![py_tuple(vec![py_str("toast"), py_tuple(vec![py_str("toast")]), py_str("Butter first, then something sharp or salty on top: peanut butter and sliced banana, mashed avocado with salt and lemon, beans with a dash of Worcestershire, or a tin of sardines on hot toast. Cheese on toast under the grill for 3 minutes is the fastest good one, and a sliced tomato under the cheese stops it going dry.")]), py_tuple(vec![py_str("pasta"), py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("macaroni"), py_str("fusilli"), py_str("tagliatelle"), py_str("linguine")]), py_str("Something you can stir through in the time the pasta boils: garlic and chilli warmed in olive oil, a tin of tomatoes simmered 10 minutes with a pinch of sugar, or cream cheese with lemon zest and black pepper. Keep a mug of the pasta water - a splash of it makes any of them cling instead of sitting in the bottom of the bowl.")]), py_tuple(vec![py_str("rice"), py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("long grain"), py_str("boiled rice")]), py_str("Cold cooked rice is best fried: hot oil, an egg, soy sauce and whatever veg is in the drawer, 4 minutes. On hot rice, a curry or a chilli ladled over it is dinner, and cold rice with tuna, sweetcorn and mayo is a lunch - the rice wants a day old for either, because fresh rice steams instead of frying.")]), py_tuple(vec![py_str("chips"), py_tuple(vec![py_str("chips"), py_str("fries"), py_str("french fries")]), py_str("Salt and vinegar, gravy, curry sauce, garlic mayo, or cheese and chilli: chip-shop curry sauce is the one to try if you want the bag to feel like a treat. A fried egg on top makes it a meal, and mushy peas or a pickled onion cut through the fat.")]), py_tuple(vec![py_str("porridge"), py_tuple(vec![py_str("porridge"), py_str("oats"), py_str("oatmeal")]), py_str("Sweet: honey or jam with a spoon of peanut butter, stewed apple with cinnamon, or frozen berries stirred in while it is hot. Savoury: butter, salt and a poached egg, or grated cheese and black pepper - porridge is only oats cooked in water or milk, so it takes either, and the salt is what makes the sweet version taste of something.")]), py_tuple(vec![py_str("noodles"), py_tuple(vec![py_str("noodles"), py_str("noodle"), py_str("ramen")]), py_str("A sauce in the time they boil: a spoon of peanut butter, soy sauce, a squeeze of lime and some chilli; or fry them with an egg and whatever veg you have plus soy sauce. Sesame oil and sliced spring onion at the end is what makes them taste like the takeaway.")]), py_tuple(vec![py_str("couscous"), py_tuple(vec![py_str("couscous")]), py_str("Couscous takes a dressing better than it takes a sauce: lemon juice, olive oil, chopped tomato, cucumber, parsley and feta is a lunch in 10 minutes. It also drinks up the juices of a stew or roast veg, which is why it is served under them.")]), py_tuple(vec![py_str("salad"), py_tuple(vec![py_str("salad"), py_str("leaves"), py_str("lettuce"), py_str("rocket")]), py_str("Something sharp, something salty, something crunchy: olive oil and lemon or vinegar, then salt, then nuts, seeds or croutons. A dressing of 2 parts oil to 1 part vinegar with a spoon of mustard, shaken in a jar, keeps a week in the fridge.")]), py_tuple(vec![py_str("crackers"), py_tuple(vec![py_str("crackers"), py_str("cracker"), py_str("crispbread"), py_str("oatcakes")]), py_str("Cheese, chutney and an apple; cream cheese with smoked salmon or cucumber; hummus and something crunchy; butter and a sliced boiled egg with salt and pepper. Anything soft wants something crisp on it, and anything dry wants something wet.")]), py_tuple(vec![py_str("soup"), py_tuple(vec![py_str("soup")]), py_str("A swirl of cream or yogurt loosens a thick soup, and something crisp on top - croutons, toasted seeds, a handful of crushed crisps - makes it feel finished. A squeeze of lemon or a splash of vinegar at the end lifts a soup that tastes flat, which is usually what it is missing rather than salt.")]), py_tuple(vec![py_str("pancakes"), py_tuple(vec![py_str("pancakes"), py_str("pancake"), py_str("crepes"), py_str("crepe")]), py_str("Lemon and sugar is the classic, and butter with golden syrup or jam works as well without the fuss. For savoury, fold cheese and ham inside, or put a fried egg on top. Keep them warm in a low oven and add the topping at the table, so nothing goes soggy.")]), py_tuple(vec![py_str("ice-cream"), py_tuple(vec![py_str("ice cream"), py_str("icecream"), py_str("gelato")]), py_str("Hot chocolate sauce, tinned peaches or raspberries with a splash of their juice, crushed biscuits or toasted nuts, and a pinch of salt on anything caramel. A shot of espresso over vanilla is affogato, which is the best 30-second pudding there is.")]), py_tuple(vec![py_str("yoghurt"), py_tuple(vec![py_str("yoghurt"), py_str("yogurt")]), py_str("Fruit and a spoon of honey, granola or toasted oats for crunch, and nuts or seeds for texture; a spoon of peanut butter and a little cocoa makes it taste like dessert. Plain yoghurt with chopped cucumber, mint, salt and garlic is a dip instead, and it wants straining first if it is watery.")])]));
    let __iter1 = py_iter_or_raise(&g_ON_TOPPINGS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    s__staple(__unpk2[0].clone());
    s__words(__unpk2[1].clone());
    s__reply(__unpk2[2].clone());
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(add(&py_str("topping-on-"), &g__staple())); __args.push(py_tuple(vec![g__words(), g_ON_WORDS()])); __args.push(g__reply()); __kws.extend(vec![("avoid".to_string(), g_NOT_A_TOPPING_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::knowledge::lookup::entry(args))(&__args) }]);
    }
    s__staple(Py::None);
    s__words(Py::None);
    s__reply(Py::None);
}
pub fn _a_side_or_the_three_jobs(args: &[Py]) -> Py {
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
    let _ = py_str("The side for the dish the line names, or for the dish the conversation is cooking.\n\n    \"any ideas for a side dish\" said after \"i'm doing a roast chicken on sunday\" is asked about\n    that bird: the dish is in the conversation rather than in this line, so the sides table is\n    asked about the conversation too (`side_for`), and only a line with no dish anywhere gets the\n    jobs a side can do.\n    ");
    return { let __b = crate::gen::knowledge::cooking::_sides(&[memory.clone(), understood.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::knowledge::cooking::side_for(&[memory.clone(), { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]); if py_truthy(&__b) { __b } else { g_A_SIDE_WITH_NO_DISH() } } } };
    Py::None
}
pub fn _flatbread_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut answer: Py = Py::None;
    let _ = py_str("The flatbread the line actually names, or the quick yoghurt one when it names none.");
pub fn __nested_answer_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut answer: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(1).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    said = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("spanish"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("espanola"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("espanol"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("omelette"))) } } } } } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g__FLATBREAD_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&said.clone(), &add(&add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "strip", &[]), "endswith", &[add(&py_str(" "), &word.clone())]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__FLATBREAD_RECIPES(), &name.clone());
    }
    }
    return geti(&g__FLATBREAD_RECIPES(), &py_str("quick"));
    Py::None
}
    answer = py_func("answer", move |args: &[Py]| { let mut full = vec![answer.clone()]; full.extend_from_slice(args); __nested_answer_0(&full) });
    return answer.clone();
    Py::None
}
pub fn _produce_brown(args: &[Py]) -> Py {
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
    let mut mushrooms: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The cut-and-brown answer for the fruit or vegetable the line names.");
    said = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    mushrooms = mcall(&g__MUSHROOM_WORDS(), "search", &[said.clone()]);
    if py_truthy(&mcall(&g__BROWN_SAFE_ASKS(), "search", &[said.clone()])) {
    return if py_truthy(&mushrooms.clone()) { g__MUSHROOM_SAFE() } else { g__BROWN_SAFE() };
    }
    if py_truthy(&mushrooms.clone()) {
    return g__MUSHROOM_BROWN();
    }
    let __iter1 = py_iter_or_raise(&g__BROWN_KINDS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &add(&add(&py_str(" "), &name.clone()), &py_str(" "))))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return g__PRODUCE_BROWN();
    Py::None
}
pub fn _about_the_food(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a crowd line is about the food: what it is, or what the food has to do.");
    line = add(&add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[])), &py_str(" "));
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&bitand(&words.clone(), &g__FOOD_IN_LINE())) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__OUR_OWN(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_PRONOUN_HELD(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g__FOOD_JOB(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:people|guests|heads|crowd|everyone)\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _wake_food(args: &[Py]) -> Py {
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
    let _ = py_str("What to put out at a wake: food people can take with one hand.");
    return py_str("Food that needs no carving and no serving: sandwiches cut small, sausage rolls, a quiche, a pot of soup kept hot, cut cake, and plenty of tea. Make it the day before, allow three or four sandwich pieces a head, and ask two people to keep the teapot going and the plates moving, because that is the job nobody thinks to do.");
    Py::None
}
pub fn _buffet_holding(args: &[Py]) -> Py {
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
    let _ = py_str("Keeping a buffet hot or cold, and how far ahead it can be made.");
    line = add(&add(&py_str(" "), &mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[])), &py_str(" "));
    if py_truthy(&mcall(&g__AHEAD(), "search", &[line.clone()])) {
    return py_str("Make sandwiches the night before, wrapped in the fridge, and leave the salad or tomato out until the morning because it turns the bread wet. Egg, cheese and ham fillings keep the best; nothing already filled should sit out for more than two hours.");
    }
    if py_truthy(&{ let __b = mcall(&g__COLD_HELD(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__HOT_HELD(), "search", &[line.clone()]))) } }) {
    return py_str("Cold food wants to stay under 5C: keep the bowls in the fridge until people arrive, stand them in a bigger bowl of ice, and put out a little at a time. Rice, cooked meat and dairy are the things to be strict about, and anything out for more than two hours goes in the bin.");
    }
    return py_str("Hot food wants to stay above 63C: a slow cooker on low, an oven on its lowest with the door ajar, or a lidded pan on a trivet, served in small batches so what is out gets eaten. Cold food wants to stay under 5C. Anything that has sat between the two for more than two hours is the food that makes people ill, so bin it.");
    Py::None
}
pub fn _about_someone_else(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line rules the food out for somebody who is not the person typing.");
    return call_builtin("bool", &[mcall(&g_SOMEONE_ELSE(), "search", &[_clean(&[understood.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_crop_not_a_larder(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("True when the food the line names is one it is growing rather than one it has in.");
    words = call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g__GROWING_WORD())))) {
    return Py::Bool(false);
    }
    if py_truthy(&_meal_asked(&[understood.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _clean(args: &[Py]) -> Py {
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
    return mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    Py::None
}
pub fn _said_words(args: &[Py]) -> Py {
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
    words = { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(crate::gen::knowledge::cooking::_fold(&[word.clone()])); } py_list(__out) })() };
    Py::None
}
pub fn minutes_said(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The time a line gives (\"20 mins\", \"half an hour\", \"an hour and a half\"), in minutes.");
    text = add(&add(&py_str(" "), &_clean(&[understood.clone()])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("(\\d+)\\s*(?:mins|min|minutes|minute)\\b"), Py::Int(1)]), py_tuple(vec![py_str("(\\d+)\\s*(?:hrs|hr|hours|hour)\\b"), Py::Int(60)]), py_tuple(vec![py_str("(\\d+)\\s*and\\s*a\\s*half\\s*(?:hours|hour|hrs|hr)\\b"), Py::Int(90)])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    found = stdlib_call("re", "search", &[pattern.clone(), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    return mul(&call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &unit.clone());
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bhalf an hour\\b"), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Int(30);
    }
    return Py::None;
    Py::None
}
pub fn _the_foods_wanted_more_or_less(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said_clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_clean"))) };
    let mut named: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut ending: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut wished: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The named foods this line only wants more or less of, and so is not stock of.");
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    wished = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_A_WISH_FOR_A_FOOD(), "finditer", &[{ let __b = said_clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    found = __item.clone();
    word = mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "strip", &[py_str(",.")]);
    name = mcall(&crate::gen::knowledge::cooking::g__INGREDIENT_WORDS(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(py_is(&name.clone(), &Py::None))) {
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str(""), py_str("s"), py_str("es")]));
    for __item in __iter2 {
    ending = __item.clone();
    name = mcall(&crate::gen::knowledge::cooking::g__INGREDIENT_WORDS(), "get", &[add(&crate::gen::knowledge::cooking::_fold(&[word.clone()]), &ending.clone())]);
    if py_truthy(&Py::Bool(!py_is(&name.clone(), &Py::None))) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&named.clone(), &name.clone()))) {
    let _ = mcall(&wished.clone(), "add", &[name.clone()]);
    }
    }
    return wished.clone();
    Py::None
}
pub fn _the_foods_wanted_more(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut said_clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said_clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said_clean"))) };
    let mut named: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut ending: Py = Py::None;
    let mut found: Py = Py::None;
    let mut more: Py = Py::None;
    let mut name: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The named foods this line says it wants more of, which a later dish is picked from.\n\n    Only \"more\": \"i want to eat less bread\" is a food to keep out of the dish, and a toastie for\n    someone cutting down on bread is the wish answered backwards.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    }
    more = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&g_A_WISH_FOR_A_FOOD(), "finditer", &[{ let __b = said_clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("more")]), &mcall(&found.clone(), "group", &[Py::Int(1)])))) {
    continue;
    }
    word = mcall(&mcall(&found.clone(), "group", &[Py::Int(2)]), "strip", &[py_str(",.")]);
    name = mcall(&crate::gen::knowledge::cooking::g__INGREDIENT_WORDS(), "get", &[word.clone()]);
    if py_truthy(&Py::Bool(py_is(&name.clone(), &Py::None))) {
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str(""), py_str("s"), py_str("es")]));
    for __item in __iter2 {
    ending = __item.clone();
    name = mcall(&crate::gen::knowledge::cooking::g__INGREDIENT_WORDS(), "get", &[add(&crate::gen::knowledge::cooking::_fold(&[word.clone()]), &ending.clone())]);
    if py_truthy(&Py::Bool(!py_is(&name.clone(), &Py::None))) {
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&named.clone(), &name.clone()))) {
    let _ = mcall(&more.clone(), "add", &[name.clone()]);
    }
    }
    return more.clone();
    Py::None
}
pub fn wants(args: &[Py]) -> Py {
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
    let mut old: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "old") { v.clone() }
    else { Py::None };
    let mut avoid: Py = Py::None;
    let mut context: Py = Py::None;
    let mut fancy: Py = Py::None;
    let mut foodish: Py = Py::None;
    let mut have: Py = Py::None;
    let mut key: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut meat: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut refused: Py = Py::None;
    let mut said_clean: Py = Py::None;
    let mut sweet: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wished: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What this line says about the meal, read on top of the plan already in `old`.\n\n    Only the things a cook would keep: the food named and ruled out, the time there is, whether\n    it has to be meat-free, who is coming, and whether it has to be simple. Everything else in\n    the line belongs to the answer and not to the plan.\n    ");
    context = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&{ let __b = old.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "items", &[])) { let __unpk4153 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk4153[0].clone(); let mut value: Py = __unpk4153[1].clone(); __out.push((key.clone(), if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { call_builtin("list", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { value.clone() })); } py_dict(__out) })() };
    words = call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    said_clean = _clean(&[understood.clone()]);
    named = call_builtin("list", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    refused = call_builtin("list", &[crate::gen::knowledge::cooking::negated_ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_ALLERGY_WORDS()); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ALLERGY_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __iter1 = py_iter_or_raise(&named.clone());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&refused.clone(), &name.clone()))) {
    let _ = mcall(&refused.clone(), "append", &[name.clone()]);
    }
    }
    }
    if py_truthy(&_refuses(&[understood.clone()])) {
    let __iter2 = py_iter_or_raise(&named.clone());
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&refused.clone(), &name.clone()))) {
    let _ = mcall(&refused.clone(), "append", &[name.clone()]);
    }
    }
    }
    wished = _the_foods_wanted_more_or_less(&[said_clean.clone(), named.clone()]);
    if py_truthy(&wished.clone()) {
    fancy = Py::None;
    let __iter3 = py_iter_or_raise(&call_builtin("sorted", &[_the_foods_wanted_more(&[said_clean.clone(), named.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    name = __item.clone();
    fancy = mcall(&context.clone(), "setdefault", &[py_str("fancy"), py_list(vec![])]);
    if py_truthy(&Py::Bool(!py_contains(&fancy.clone(), &name.clone()))) {
    let _ = mcall(&fancy.clone(), "append", &[name.clone()]);
    }
    }
    }
    let __iter4 = py_iter_or_raise(&named.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&refused.clone(), &name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&wished.clone(), &name.clone())) } }) {
    continue;
    }
    have = mcall(&context.clone(), "setdefault", &[py_str("have"), py_list(vec![])]);
    if py_truthy(&Py::Bool(!py_contains(&have.clone(), &name.clone()))) {
    let _ = mcall(&have.clone(), "append", &[name.clone()]);
    }
    }
    let __iter5 = py_iter_or_raise(&refused.clone());
    for __item in __iter5 {
    name = __item.clone();
    avoid = mcall(&context.clone(), "setdefault", &[py_str("avoid"), py_list(vec![])]);
    if py_truthy(&Py::Bool(!py_contains(&avoid.clone(), &name.clone()))) {
    let _ = mcall(&avoid.clone(), "append", &[name.clone()]);
    }
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("have")]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ONLY_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&context.clone(), &py_str("only"), Py::Bool(true));
    }
    limit = minutes_said(&[understood.clone()]);
    if py_truthy(&Py::Bool(!py_is(&limit.clone(), &Py::None))) {
    let _ = py_setitem(&context.clone(), &py_str("limit"), limit.clone());
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_VEG_WORDS()); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MEAT_FREE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_clean(&[understood.clone()]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&context.clone(), &py_str("veg"), Py::Bool(true));
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DAIRY_FREE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    avoid = mcall(&context.clone(), "setdefault", &[py_str("avoid"), py_list(vec![])]);
    let __iter6 = py_iter_or_raise(&g_DAIRY_FOODS());
    for __item in __iter6 {
    name = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&avoid.clone(), &name.clone()))) {
    let _ = mcall(&avoid.clone(), "append", &[name.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &py_str("vegan")))) {
    let _ = py_setitem(&context.clone(), &py_str("vegan"), Py::Bool(true));
    }
    if py_truthy(&mcall(&g_CROWD_PHRASE(), "search", &[said_clean.clone()])) {
    let _ = py_setitem(&context.clone(), &py_str("crowd"), Py::Bool(true));
    }
    if py_truthy(&{ let __b = { let __b = mcall(&context.clone(), "get", &[py_str("veg")]); if py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("vegan")]) } }; if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("have")]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("crowd")]))); if py_truthy(&__b) { __b } else { _meal_asked(&[understood.clone()]) } } } } } }) {
    let _ = py_setitem(&context.clone(), &py_str("diet"), Py::Bool(true));
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("veg")]); if !py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("have")]) } }) {
    meat = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&context.clone(), &py_str("have"))) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&g_MEAT_NAMES(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&meat.clone()) {
    let _ = py_setitem(&context.clone(), &py_str("have"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&context.clone(), &py_str("have"))) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_MEAT_NAMES(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() });
    if py_truthy(&Py::Bool(!py_truthy(&geti(&context.clone(), &py_str("have"))))) {
    let _ = mcall(&context.clone(), "pop", &[py_str("have"), Py::None]);
    let _ = py_setitem(&context.clone(), &py_str("diet"), Py::Bool(true));
    }
    }
    }
    foodish = { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_DINNER_WORDS())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COMING_FOR()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_clean(&[understood.clone()]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    said_clean = _clean(&[understood.clone()]);
    if py_truthy(&{ let __b = foodish.clone(); if !py_truthy(&__b) { __b } else { { let __b = { let __b = crate::gen::knowledge::cooking::_occasion(&[said_clean.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MENU_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CROWD_PHRASE(), "search", &[said_clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_BIG_DAYS()); if py_truthy(&__b) { __b } else { { let __b = { let __b = bitand(&words.clone(), &g_PEOPLE_WORDS()); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COMING_FOR()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }; if py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_EVENT_WORDS()); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_COOKING_FOR()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } } } } } } } }) {
    let _ = py_setitem(&context.clone(), &py_str("occasion"), Py::Bool(true));
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_LIMIT_WORDS()); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SIMPLE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said_clean.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&context.clone(), &py_str("simple"), Py::Bool(true));
    }
    if py_truthy(&bitand(&words.clone(), &g_HEAT_WORDS())) {
    if py_truthy(&_a_taste_ruled_out(&[said_clean.clone()])) {
    let _ = py_setitem(&context.clone(), &py_str("no_heat"), Py::Bool(true));
    avoid = mcall(&context.clone(), "setdefault", &[py_str("avoid"), py_list(vec![])]);
    let __iter7 = py_iter_or_raise(&g_HEAT_INGREDIENTS());
    for __item in __iter7 {
    name = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&avoid.clone(), &name.clone()))) {
    let _ = mcall(&avoid.clone(), "append", &[name.clone()]);
    }
    }
    } else {
    let _ = py_setitem(&context.clone(), &py_str("heat"), Py::Bool(true));
    }
    }
    sweet = { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_SWEET_WORDS())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SWEET_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_clean(&[understood.clone()]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = sweet.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::knowledge::cooking::ingredients(&[understood.clone()]))) } }) {
    let _ = py_setitem(&context.clone(), &py_str("sweet"), Py::Bool(true));
    let _ = mcall(&context.clone(), "pop", &[py_str("have"), Py::None]);
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_DISLIKE_WORDS()); if !py_truthy(&__b) { __b } else { bitand(&words.clone(), &bitor(&g_GREEN_WORDS(), &py_set(vec![py_str("veg"), py_str("vegetables"), py_str("veggies")]))) } }) {
    let _ = py_setitem(&context.clone(), &py_str("no_green"), Py::Bool(true));
    }
    return context.clone();
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
    let mut names: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: names"))) };
    let mut context: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let _ = py_str("Whether a dish or an idea is allowed by the plan's refusals.");
    names = call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&bitand(&names.clone(), &call_builtin("set", &[{ let __b = mcall(&context.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("veg")]); if !py_truthy(&__b) { __b } else { bitand(&names.clone(), &g_MEAT_NAMES()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("no_green")]); if !py_truthy(&__b) { __b } else { bitand(&names.clone(), &g_GREEN_NAMES()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("no_heat")]); if !py_truthy(&__b) { __b } else { bitand(&names.clone(), &call_builtin("set", &[g_HEAT_INGREDIENTS()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _text_conflict(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut dish: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "dish") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: dish"))) };
    let mut context: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut name: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the dish's own words break a refusal: pasta with parmesan is not \"no cheese\".");
    text = mcall(&{ let __b = mcall(&dish.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&context.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    name = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_AVOID_TEXT(), "get", &[name.clone(), py_tuple(vec![name.clone()])]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("vegan")]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_VEGAN_BAD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _quick_limit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut context: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let _ = py_str("The longest a dish may take for this plan: the number they said, or \"quick\" as twenty.");
    if py_truthy(&mcall(&context.clone(), "get", &[py_str("limit")])) {
    return geti(&context.clone(), &py_str("limit"));
    }
    return if py_truthy(&mcall(&context.clone(), "get", &[py_str("simple")])) { g_QUICK_MINUTES() } else { Py::None };
    Py::None
}
pub fn _quick_enough(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut minutes: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "minutes") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: minutes"))) };
    let mut context: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut limit: Py = Py::None;
    limit = _quick_limit(&[context.clone()]);
    return Py::Bool(!py_truthy(&{ let __b = limit.clone(); if !py_truthy(&__b) { __b } else { { let __b = minutes.clone(); if !py_truthy(&__b) { __b } else { gt(&minutes.clone(), &limit.clone()) } } } }));
    Py::None
}
pub fn _score(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut dish: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "dish") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: dish"))) };
    let mut context: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut have: Py = Py::None;
    let mut limit: Py = Py::None;
    let mut minutes: Py = Py::None;
    let mut score: Py = Py::None;
    let mut text: Py = Py::None;
    let mut uses: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How well a dish answers the plan: what it uses, what it leaves over, and the time.");
    have = call_builtin("set", &[{ let __b = mcall(&context.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    uses = call_builtin("set", &[geti(&dish.clone(), &py_str("uses"))]).unwrap_or_else(|e| exc::raise(e));
    minutes = mcall(&g_MINUTES(), "get", &[geti(&dish.clone(), &py_str("id"))]);
    score = sub(&mul(&call_builtin("len", &[bitand(&uses.clone(), &have.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)), &mul(&call_builtin("len", &[sub(&have.clone(), &uses.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)));
    if py_truthy(&Py::Bool(!py_is(&minutes.clone(), &Py::None))) {
    let __aug1 = sub(&score.clone(), &floordiv(&minutes.clone(), &Py::Int(25)));
    score = __aug1;
    limit = _quick_limit(&[context.clone()]);
    if py_truthy(&{ let __b = limit.clone(); if !py_truthy(&__b) { __b } else { gt(&minutes.clone(), &limit.clone()) } }) {
    let __aug2 = sub(&score.clone(), &Py::Int(6));
    score = __aug2;
    }
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("simple")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&minutes.clone(), &Py::None)) } }) {
    let __aug3 = sub(&score.clone(), &floordiv(&call_builtin("max", &[Py::Int(0), sub(&minutes.clone(), &Py::Int(20))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)));
    score = __aug3;
    }
    if py_truthy(&{ let __b = have.clone(); if !py_truthy(&__b) { __b } else { le(&have.clone(), &uses.clone()) } }) {
    let __aug4 = add(&score.clone(), &Py::Int(4));
    score = __aug4;
    }
    if py_truthy(&have.clone()) {
    let __aug5 = sub(&score.clone(), &call_builtin("len", &[sub(&sub(&uses.clone(), &have.clone()), &g_STAPLES())]).unwrap_or_else(|e| exc::raise(e)));
    score = __aug5;
    } else {
    let __aug6 = sub(&score.clone(), &mul(&call_builtin("len", &[uses.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)));
    score = __aug6;
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("occasion")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THIN(), &geti(&dish.clone(), &py_str("id")))) } }) {
    let __aug7 = sub(&score.clone(), &Py::Int(6));
    score = __aug7;
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("occasion")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_FOR_ONE(), &geti(&dish.clone(), &py_str("id")))) } }) {
    let __aug8 = sub(&score.clone(), &Py::Int(10));
    score = __aug8;
    }
    if py_truthy(&mcall(&context.clone(), "get", &[py_str("heat")])) {
    text = mcall(&{ let __b = mcall(&dish.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    let __aug9 = add(&score.clone(), &if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEAT_IN_TEXT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) { Py::Int(6) } else { neg(&Py::Int(2)) });
    score = __aug9;
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("occasion")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CROWD_DISHES(), &geti(&dish.clone(), &py_str("id")))) } }) {
    let __aug10 = add(&score.clone(), &Py::Int(8));
    score = __aug10;
    if py_truthy(&ge(&{ let __b = minutes.clone(); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &Py::Int(30))) {
    let __aug11 = add(&score.clone(), &Py::Int(4));
    score = __aug11;
    } else {
    if py_truthy(&Py::Bool(py_is(&mcall(&context.clone(), "get", &[py_str("limit")]), &Py::None))) {
    let __aug12 = sub(&score.clone(), &Py::Int(2));
    score = __aug12;
    }
    }
    }
    return score.clone();
    Py::None
}
pub fn _meal_idea_for(args: &[Py]) -> Py {
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
    let mut idea: Py = Py::None;
    let mut names: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The meal idea for the same ingredient, for a plan that has to be quick.\n\n    The pantry line is the shortest thing the table can say about an ingredient and it is what\n    most lines want, but a roast takes forty minutes: when the line asked for something quick\n    and the pantry line is slower than that, the meal idea for the same food is the twenty-\n    minute one (\"cut it into strips and fry it\").\n    ");
    if py_truthy(&Py::Bool(!py_contains(&crate::gen::knowledge::cooking::g_PANTRY(), &name.clone()))) {
    return Py::None;
    }
    words = bitor(&call_builtin("set", &[geti(&geti(&crate::gen::knowledge::cooking::g_PANTRY(), &name.clone()), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![name.clone()]));
    let __iter1 = py_iter_or_raise(&crate::gen::knowledge::cooking::g_MEAL_IDEAS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    idea = __unpk2[1].clone();
    if py_truthy(&bitand(&words.clone(), &call_builtin("set", &[names.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return idea.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _ideas(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut context: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Bool(true) };
    let mut faster: Py = Py::None;
    let mut have: Py = Py::None;
    let mut idea: Py = Py::None;
    let mut name: Py = Py::None;
    let mut out: Py = Py::None;
    let mut quick: Py = Py::None;
    let _ = py_str("One-ingredient ideas for what is in the plan, in the order the plan mentions it.");
    out = py_list(vec![]);
    have = call_builtin("list", &[{ let __b = mcall(&context.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    quick = if py_truthy(&limit.clone()) { _quick_limit(&[context.clone()]) } else { Py::None };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[have.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&crate::gen::knowledge::cooking::g_PANTRY(), &name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_fits(&[py_tuple(vec![name.clone()]), context.clone()]))) } }) {
    continue;
    }
    idea = geti(&geti(&crate::gen::knowledge::cooking::g_PANTRY(), &name.clone()), &Py::Int(1));
    if py_truthy(&{ let __b = quick.clone(); if !py_truthy(&__b) { __b } else { gt(&{ let __b = mcall(&g_PANTRY_MINUTES(), "get", &[name.clone()]); if py_truthy(&__b) { __b } else { Py::Int(0) } }, &quick.clone()) } }) {
    faster = _meal_idea_for(&[name.clone()]);
    if py_truthy(&faster.clone()) {
    idea = faster.clone();
    } else {
    if py_truthy(&limit.clone()) {
    continue;
    }
    }
    }
    if py_truthy(&{ let __b = crate::gen::knowledge::cooking::_mentions(&[idea.clone(), { let __b = mcall(&context.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]); if py_truthy(&__b) { __b } else { _text_conflict(&[py_dict(vec![(py_str("text"), idea.clone())]), context.clone()]) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![name.clone(), idea.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn plan_reply(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut context: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut skip: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { py_tuple(vec![]) };
    let mut _u: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut have: Py = Py::None;
    let mut idea: Py = Py::None;
    let mut ideas: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut scored: Py = Py::None;
    let mut skip_ids: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("An answer for the plan, or None when the plan has nothing to work with.\n\n    A dish that uses most of what is in the kitchen comes first; with nothing named, or nothing\n    the dishes cover, the one-ingredient ideas for what is in the plan come next. A dish or idea\n    the bot has already offered is passed over so the next line gets something new.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&context.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("have")]))); if !py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("fancy")]) } }) {
    context = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(context.clone()); __kws.extend(vec![("have".to_string(), call_builtin("list", &[geti(&context.clone(), &py_str("fancy"))]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    if py_truthy(&mcall(&context.clone(), "get", &[py_str("only")])) {
    ideas = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(context.clone()); __kws.extend(vec![("limit".to_string(), call_builtin("bool", &[{ let __b = mcall(&context.clone(), "get", &[py_str("limit")]); if py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("simple")]) } }]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ideas(args))(&__args) };
    if py_truthy(&ideas.clone()) {
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&ideas.clone(), &Py::None, &Py::Int(2), &Py::None)) { let __unpk8657 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk8657[0].clone(); let mut idea: Py = __unpk8657[1].clone(); __out.push(idea.clone()); } py_list(__out) })() }]);
    }
    }
    offered = { let __b = mcall(&context.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } };
    skip_ids = bitor(&call_builtin("set", &[skip.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[if py_truthy(&call_builtin("isinstance", &[offered.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) { offered.clone() } else { py_tuple(vec![]) }]).unwrap_or_else(|e| exc::raise(e)));
    scored = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&crate::gen::knowledge::cooking::g_DISHES());
    for __item in __iter1 {
    dish = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&skip_ids.clone(), &geti(&dish.clone(), &py_str("id")))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_fits(&[geti(&dish.clone(), &py_str("uses")), context.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_is(&Py::Bool(py_contains(&g_SWEET_DISHES(), &geti(&dish.clone(), &py_str("id")))), &call_builtin("bool", &[mcall(&context.clone(), "get", &[py_str("sweet")])]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    have = call_builtin("set", &[{ let __b = mcall(&context.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&have.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("sweet")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("diet")]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = have.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&call_builtin("set", &[geti(&dish.clone(), &py_str("uses"))]).unwrap_or_else(|e| exc::raise(e)), &have.clone()))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = have.clone(); if !py_truthy(&__b) { __b } else { sub(&sub(&call_builtin("set", &[geti(&dish.clone(), &py_str("uses"))]).unwrap_or_else(|e| exc::raise(e)), &have.clone()), &g_STAPLES()) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("limit")]); if py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("simple")]) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&_quick_enough(&[mcall(&g_MINUTES(), "get", &[geti(&dish.clone(), &py_str("id"))]), context.clone()])))) {
    continue;
    }
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("no_green")]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GREEN_IN_TEXT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&geti(&dish.clone(), &py_str("text")), "lower", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("no_heat")]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEAT_IN_TEXT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&mcall(&geti(&dish.clone(), &py_str("text")), "lower", &[]), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&_text_conflict(&[dish.clone(), context.clone()])) {
    continue;
    }
    let _ = mcall(&scored.clone(), "append", &[py_tuple(vec![_score(&[dish.clone(), context.clone()]), dish.clone()])]);
    }
    if py_truthy(&scored.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let context = context.clone(); let dish = dish.clone(); let have = have.clone(); let idea = idea.clone(); let ideas = ideas.clone(); let memory = memory.clone(); let offered = offered.clone(); let scored = scored.clone(); let skip = skip.clone(); let skip_ids = skip_ids.clone(); let word = word.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); neg(&geti(&pair.clone(), &Py::Int(0))) }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&scored.clone(), "sort", args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(!py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = mcall(&mcall(&geti(&memory.clone(), &py_str("meal")), "setdefault", &[py_str("offered"), py_list(vec![])]), "append", &[geti(&geti(&geti(&scored.clone(), &Py::Int(0)), &Py::Int(1)), &py_str("id"))]);
    }
    return _with_heat(&[geti(&geti(&geti(&scored.clone(), &Py::Int(0)), &Py::Int(1)), &py_str("text")), context.clone()]);
    }
    ideas = _ideas(&[context.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ideas.clone()))) {
    ideas = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(context.clone()); __kws.extend(vec![("limit".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ideas(args))(&__args) };
    }
    if py_truthy(&ideas.clone()) {
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&ideas.clone(), &Py::None, &Py::Int(2), &Py::None)) { let __unpk9636 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk9636[0].clone(); let mut idea: Py = __unpk9636[1].clone(); __out.push(idea.clone()); } py_list(__out) })() }]);
    }
    return Py::None;
    Py::None
}
pub fn _a_taste_ruled_out(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut found: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the line refuses the heat rather than asking for it.\n\n    \"and the kids wont eat anything spicy\" is the same words as \"something spicy\" to the heat\n    flag, and the dish on the table came back with chilli put into it. \"its not spicy enough\" is\n    the other way round - the line wants more - so a taste word followed by \"enough\" is a wish\n    even with \"not\" in front of it.\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g__TASTE_RULED_OUT(), g__HOT_FOOD_RULED_OUT()]));
    for __item in __iter1 {
    pattern = __item.clone();
    let __iter2 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]));
    for __item in __iter2 {
    found = __item.clone();
    after = mcall(&slice_or_raise(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, &mcall(&found.clone(), "end", &[]), &add(&mcall(&found.clone(), "end", &[]), &Py::Int(12)), &Py::None), "lower", &[]);
    if py_truthy(&mcall(&mcall(&after.clone(), "strip", &[]), "startswith", &[py_str("enough")])) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _with_heat(args: &[Py]) -> Py {
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
    let mut context: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut lowered: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The dish, with the heat asked for put into it when its own words have none.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("heat")]))); if py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("no_heat")]) } }) {
    return text.clone();
    }
    lowered = mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_HEAT_IN_TEXT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&lowered.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return text.clone();
    }
    return add(&mcall(&text.clone(), "rstrip", &[]), &py_str(" Add chopped chilli or a pinch of chilli flakes with the onion for the heat - taste at the end and add more if it needs it."));
    Py::None
}
pub fn plan_recent(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The plan still in force, or None once the conversation has moved on.");
    plan = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("meal")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&gt(&sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e))), &g_PLAN_LAPSE())) {
    return Py::None;
    }
    return plan.clone();
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
    let mut turns: Py = Py::None;
    let _ = py_str("What the bot said last, so a dish already on the table is not put there twice.");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn remembered(args: &[Py]) -> Py {
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
    let mut allergies: Py = Py::None;
    let mut asking_how: Py = Py::None;
    let mut asking_with_food: Py = Py::None;
    let mut avoid: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut food: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut last_ask: Py = Py::None;
    let mut left: Py = Py::None;
    let mut merged: Py = Py::None;
    let mut name: Py = Py::None;
    let mut new: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_last: Py = Py::None;
    let mut stocked: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut wish: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What of this line to keep for the next short reply to narrow, or None.\n\n    The plan is written from any line about cooking that says something a cook would keep, and\n    it is forgotten after a couple of turns about something else, so \"how long do I roast a\n    chicken\" two turns after a dinner plan is not answered with a sausage traybake.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("empty"), py_str("greeting")]), &mcall(&understood.clone(), "get", &[py_str("kind")])))) {
    return Py::None;
    }
    if py_truthy(&crate::gen::knowledge::lookup::says_it_has_gone_off(&[mcall(&understood.clone(), "get", &[py_str("clean")])])) {
    return Py::None;
    }
    plan = plan_recent(&[memory.clone()]);
    words = call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    said = _clean(&[understood.clone()]);
    topic = mcall(&understood.clone(), "get", &[py_str("topic_own")]);
    if py_truthy(&{ let __b = bitand(&words.clone(), &crate::gen::knowledge::cooking::g_PET_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&plan.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&plan.clone())); if !py_truthy(&__b) { __b } else { _a_crop_not_a_larder(&[understood.clone()]) } }) {
    return Py::None;
    }
    food = { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]), &topic.clone())); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_DINNER_WORDS())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[crate::gen::knowledge::cooking::negated_ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&food.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&plan.clone())) } }) {
    return Py::None;
    }
    stocked = { let __b = call_builtin("bool", &[{ let __b = crate::gen::knowledge::cooking::ingredients(&[understood.clone()]); if py_truthy(&__b) { __b } else { crate::gen::knowledge::cooking::negated_ingredients(&[understood.clone()]) } }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("statement"))); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STOCK_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    wish = call_builtin("bool", &[_the_foods_wanted_more(&[said.clone(), call_builtin("list", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    asking_with_food = { let __b = call_builtin("bool", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NEW_MEAL_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&_clean(&[understood.clone()]), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    asking_how = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_METHOD_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&call_builtin("len", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&bitand(&words.clone(), &g_EVENT_WORDS()))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CROWD_PHRASE(), "search", &[said.clone()]))) } } } } } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&plan.clone())); if !py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_TECHNIQUE_WORDS()); if py_truthy(&__b) { __b } else { { let __b = asking_how.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_meal_asked(&[understood.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&stocked.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&asking_with_food.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&wish.clone())) } } } } } } } } } } } }) {
    return Py::None;
    }
    new = wants(&[understood.clone()]);
    kept = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_KEYS()) { let mut key: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&new.clone(), &key.clone()))) {  __out.push((key.clone(), geti(&new.clone(), &key.clone()))); } } py_dict(__out) })() };
    if py_truthy(&mcall(&new.clone(), "get", &[py_str("fancy")])) {
    let _ = py_setitem(&kept.clone(), &py_str("fancy"), geti(&new.clone(), &py_str("fancy")));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&food.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&plan.clone())); if !py_truthy(&__b) { __b } else { { let __b = kept.clone(); if !py_truthy(&__b) { __b } else { le(&call_builtin("set", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("veg"), py_str("vegan")])) } } } }) {
    last_ask = call_builtin("set", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![py_dict(vec![])]) } }, &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&last_ask.clone(), &g_DINNER_WORDS())))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&plan.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = plan.clone(); if !py_truthy(&__b) { __b } else { { let __b = mcall(&plan.clone(), "get", &[py_str("have")]); if !py_truthy(&__b) { __b } else { _asks_about_a_food_it_has_not_got(&[understood.clone(), plan.clone()]) } } } }) {
    return plan.clone();
    }
    merged = wants(&[understood.clone(), plan.clone()]);
    let _ = mcall(&merged.clone(), "setdefault", &[py_str("started"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]);
    let _ = py_setitem(&merged.clone(), &py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    offered = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&merged.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(name.clone()); } } py_list(__out) })() };
    said_last = _last_reply(&[memory.clone()]);
    if py_truthy(&said_last.clone()) {
    let __iter1 = py_iter_or_raise(&crate::gen::knowledge::cooking::g_DISHES());
    for __item in __iter1 {
    dish = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&offered.clone(), &geti(&dish.clone(), &py_str("id")))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said_last.clone(), &geti(&dish.clone(), &py_str("text")))) } }) {
    let _ = mcall(&offered.clone(), "append", &[geti(&dish.clone(), &py_str("id"))]);
    }
    }
    }
    if py_truthy(&offered.clone()) {
    let _ = py_setitem(&merged.clone(), &py_str("offered"), slice_or_raise(&offered.clone(), &neg(&Py::Int(4)), &Py::None, &Py::None));
    }
    allergies = _allergy_names(&[memory.clone()]);
    if py_truthy(&allergies.clone()) {
    gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&merged.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&_is_allergy_food(&[name.clone(), allergies.clone()])) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&gone.clone()) {
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&merged.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&gone.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&left.clone()) {
    let _ = py_setitem(&merged.clone(), &py_str("have"), left.clone());
    } else {
    let _ = mcall(&merged.clone(), "pop", &[py_str("have"), Py::None]);
    }
    avoid = call_builtin("list", &[{ let __b = mcall(&merged.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    let _ = mcall(&avoid.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&avoid.clone(), &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() }]);
    let _ = py_setitem(&merged.clone(), &py_str("avoid"), avoid.clone());
    }
    }
    return merged.clone();
    Py::None
}
pub fn _allergy_names(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut user: Py = Py::None;
    let _ = py_str("The foods the person has said they cannot eat, as the memory lane keeps them.");
    user = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&user.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() };
    Py::None
}
pub fn _is_allergy_food(args: &[Py]) -> Py {
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
    let mut allergies: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "allergies") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: allergies"))) };
    let mut allergy: Py = Py::None;
    let mut folded: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("Whether a food is one of the allergies, allowing for \"peanut\" and \"peanuts\".");
    folded = crate::gen::knowledge::cooking::_fold(&[call_builtin("str", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    stem = slice_or_raise(&folded.clone(), &Py::None, &Py::Int(4), &Py::None);
    return { let __b = call_builtin("bool", &[stem.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&allergies.clone()) { let mut allergy: Py = __item.clone(); if py_truthy(&ge(&call_builtin("len", &[allergy.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {  __out.push({ let __b = Py::Bool(py_contains(&slice_or_raise(&allergy.clone(), &Py::None, &Py::Int(4), &Py::None), &stem.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&folded.clone(), &slice_or_raise(&allergy.clone(), &Py::None, &Py::Int(4), &Py::None))) } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _asks_about_a_food_it_has_not_got(args: &[Py]) -> Py {
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
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether this line asks about a food of its own rather than narrowing the dish.");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::knowledge::cooking::ingredients(&[understood.clone()])) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ let __b = mcall(&plan.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_DINNER_WORDS())) {
    return Py::Bool(false);
    }
    said = _clean(&[understood.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__ABOUT_A_FOOD()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_ABSENCE(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _narrowing(args: &[Py]) -> Py {
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
    let mut plan: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { Py::None };
    let mut longest: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "longest") { v.clone() }
    else { Py::Int(10) };
    let mut asked: Py = Py::None;
    let mut asking: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut narrowed: Py = Py::None;
    let mut new: Py = Py::None;
    let mut own: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut pointed: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The plan a short line adds to, or None when the line is not narrowing one.\n\n    Everything a line has to be to continue a plan is checked here once, so the first line of a\n    conversation (\"whats a quick dinner for two kids who wont eat anything green\") and the fifth\n    (\"and quick\") are read the same way. A question about how to cook something, a swap of one\n    ingredient for another, a yes-or-no about the pan and a meal question of its own are all\n    somebody else's answer, never a narrowing of this one.\n    ");
    words = call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &longest.clone()) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("empty"), py_str("greeting"), py_str("thanks")]), &mcall(&understood.clone(), "get", &[py_str("kind")])))) {
    return Py::None;
    }
    said = _clean(&[understood.clone()]);
    own = mcall(&understood.clone(), "get", &[py_str("topic_own")]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&py_tuple(vec![Py::None, py_str("cooking"), py_str("food-and-drink")]), &own.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::knowledge::cooking::ingredients(&[understood.clone()]))) } }) {
    narrowed = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_KEYS()) { let mut key: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("simple")))) {  __out.push(Py::Bool(py_contains(&wants(&[understood.clone()]), &key.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = plan.clone(); if !py_truthy(&__b) { __b } else { { let __b = narrowed.clone(); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&wants(&[understood.clone()]), &py_str("limit"))) } } } }))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = le(&words.clone(), &g_CHATTER()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &g_DINNER_WORDS()))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SWAP_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WITH_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&bitand(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("how"), py_str("how_long"), py_str("how_much"), py_str("where"), py_str("how_often"), py_str("why")]))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_TECHNIQUE_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&minutes_said(&[understood.clone()]), &Py::None)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("yesno")))) {
    if py_truthy(&{ let __b = crate::gen::knowledge::cooking::ingredients(&[understood.clone()]); if py_truthy(&__b) { __b } else { bitand(&words.clone(), &py_set(vec![py_str("it"), py_str("its"), py_str("that"), py_str("this"), py_str("them"), py_str("those")])) } }) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = crate::gen::knowledge::cooking::ingredients(&[understood.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NEW_MEAL_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_BAKING_WORDS()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal")])))) } }) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SAID_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_PERSON_WORDS()); if !py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &g_TASTE_WORDS()); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&bitand(&words.clone(), &g_DINNER_WORDS()))); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&minutes_said(&[understood.clone()]), &Py::None)) } } } } } }) {
    return Py::None;
    }
    new = wants(&[understood.clone()]);
    if py_truthy(&{ let __b = mcall(&new.clone(), "get", &[py_str("heat")]); if !py_truthy(&__b) { __b } else { mcall(&{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("sweet")]) } }) {
    return Py::None;
    }
    kept = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_KEYS()) { let mut key: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("simple")))) {  __out.push(Py::Bool(py_contains(&new.clone(), &key.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    kept = { let __b = kept.clone(); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&new.clone(), &py_str("limit"))); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_LIMIT_WORDS())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&new.clone(), &py_str("heat"))); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SIMPLE_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } } } } } };
    asked = { let __b = call_builtin("bool", &[bitand(&words.clone(), &g_DINNER_WORDS())]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::knowledge::cooking::g_MEAL_PHRASES()) { let mut phrase: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_BARE_MEAL_NOUNS(), &phrase.clone()))) {  __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { _the_bare_ideas_are_about_food(&[understood.clone(), said.clone()]) } } } };
    pointed = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_KEYS()) { let mut key: Py = __item.clone(); __out.push(mcall(&{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    asking = { let __b = Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&understood.clone(), "get", &[py_str("asks")])]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { { let __b = pointed.clone(); if !py_truthy(&__b) { __b } else { asking.clone() } } } })) } }) {
    return Py::None;
    }
    return wants(&[understood.clone(), plan.clone()]);
    Py::None
}
pub fn _the_bare_ideas_are_about_food(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut named: Py = Py::None;
    let mut noun: Py = Py::None;
    let _ = py_str("Whether a line asking for \"ideas\" without a meal word is about the food on the table.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BARE_MEAL_NOUNS()) { let mut noun: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &noun.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&crate::gen::knowledge::cooking::ingredients(&[understood.clone()])) {
    return Py::Bool(true);
    }
    named = mcall(&g_BARE_MEAL_OBJECT(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(py_is(&named.clone(), &Py::None))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[bitand(&call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &g_THE_FOOD_IN_HAND())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _idea_on_the_plate(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut alias: Py = Py::None;
    let mut lowered: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The meal idea the last answer gave, when it was built from an ingredient and not a dish.");
    said = _last_reply(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    lowered = mcall(&said.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&plan.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    name = __item.clone();
    let __iter2 = py_iter_or_raise(&geti(&{ let __b = mcall(&crate::gen::knowledge::cooking::g_PANTRY(), "get", &[name.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![py_tuple(vec![])]) } }, &Py::Int(0)));
    for __item in __iter2 {
    alias = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&lowered.clone(), &alias.clone()))) {
    return said.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _chosen(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plan: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plan") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plan"))) };
    let mut dish: Py = Py::None;
    let mut dishes: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The dish to put forward when they leave the choice to the bot: the last one offered.");
    dishes = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::knowledge::cooking::g_DISHES()) { let mut dish: Py = __item.clone(); __out.push((geti(&dish.clone(), &py_str("id")), dish.clone())); } py_dict(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    name = __item.clone();
    dish = mcall(&dishes.clone(), "get", &[name.clone()]);
    if py_truthy(&{ let __b = dish.clone(); if !py_truthy(&__b) { __b } else { { let __b = _fits(&[geti(&dish.clone(), &py_str("uses")), plan.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_text_conflict(&[dish.clone(), plan.clone()]))) } } } }) {
    return geti(&dish.clone(), &py_str("text"));
    }
    }
    return Py::None;
    Py::None
}
pub fn _an_allergy(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line names an allergy, in any shape the kitchen already reads as one.\n\n    \"im allergic to nuts\", \"she has a nut allergy\", \"lactose intolerant\" and \"they cant eat\n    dairy\" all rule the food out, and the allergy answer is the one that speaks to them: \"That\n    one has no nut in it anyway, so you're fine\" is not a thing to say to somebody who has just\n    told you about a reaction.\n    ");
    said = _clean(&[understood.clone()]);
    if py_truthy(&bitand(&call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_ALLERGY_WORDS())) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__A_REACTION()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _nothing_to_change(args: &[Py]) -> Py {
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
    let mut banned: Py = Py::None;
    let mut in_dish: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut left_out: Py = Py::None;
    let mut mine: Py = Py::None;
    let mut names: Py = Py::None;
    let mut new: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut said: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut where_: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("An answer for a line that only rules something out of the dish already on the table.\n\n    \"and no cheese either\" after a chickpea curry does not need a new dish, and answering it from\n    the plain entry says the first answer over again as if none of it had been kept. The line has\n    to add nothing but the rule-out: a new food, a time or a pudding is a different meal.\n    ");
    new = wants(&[understood.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PLAN_KEYS()) { let mut key: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&key.clone(), &py_str("avoid")))) {  __out.push(Py::Bool(py_contains(&new.clone(), &key.clone()))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    plan = { let __b = plan_recent(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&plan.clone(), "get", &[py_str("offered")])))) {
    return Py::None;
    }
    kept = call_builtin("list", &[{ let __b = mcall(&new.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    said = _last_reply(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&kept.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::None;
    }
    names = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&py_str(" or "), "join", &[slice_or_raise(&kept.clone(), &Py::None, &Py::Int(2), &Py::None)]), -1, "")); py_string(__s) };
    if py_truthy(&_an_allergy(&[understood.clone()])) {
    banned = call_builtin("sorted", &[crate::gen::knowledge::cooking::_excluded(&[_clean(&[understood.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    mine = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_said_words(&[understood.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&banned.clone(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    theirs = _about_someone_else(&[understood.clone()]);
    in_dish = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&banned.clone()) { let mut word: Py = __item.clone(); if py_truthy(&crate::gen::knowledge::cooking::_mentions(&[said.clone(), py_tuple(vec![word.clone()])])) {  __out.push(word.clone()); } } py_list(__out) })() };
    who = if py_truthy(&theirs.clone()) { py_str("theirs") } else { py_str("that one") };
    if py_truthy(&in_dish.clone()) {
    left_out = geti(&in_dish.clone(), &Py::Int(0));
    where_ = { let __b = mcall(&py_str(" or "), "join", &[slice_or_raise(&mine.clone(), &Py::None, &Py::Int(2), &Py::None)]); if py_truthy(&__b) { __b } else { left_out.clone() } };
    return { let mut __s = String::new(); __s.push_str("The "); __s.push_str(&fstr(&left_out.clone(), -1, "")); __s.push_str(" is where the "); __s.push_str(&fstr(&where_.clone(), -1, "")); __s.push_str(" is, so leave it out of "); __s.push_str(&fstr(&who.clone(), -1, "")); __s.push_str(" and keep the rest the same - and read the label on anything that came in a packet before you use it."); py_string(__s) };
    }
    if py_truthy(&mine.clone()) {
    return { let mut __s = String::new(); __s.push_str("There is no "); __s.push_str(&fstr(&mcall(&py_str(" or "), "join", &[slice_or_raise(&mine.clone(), &Py::None, &Py::Int(2), &Py::None)]), -1, "")); __s.push_str(" in that dish as it stands, so it is safe"); __s.push_str(&fstr(&if py_truthy(&theirs.clone()) { py_str(" for them") } else { py_str("") }, -1, "")); __s.push_str(" - just keep the boards and knives clean, and read the label on anything that came in a packet before you use it."); py_string(__s) };
    }
    return Py::None;
    }
    if py_truthy(&_about_someone_else(&[understood.clone()])) {
    if py_truthy(&crate::gen::knowledge::cooking::_mentions(&[said.clone(), kept.clone()])) {
    return { let mut __s = String::new(); __s.push_str("Then make theirs without the "); __s.push_str(&fstr(&names.clone(), -1, "")); __s.push_str(" and keep the rest the same - it works."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("That one has no "); __s.push_str(&fstr(&names.clone(), -1, "")); __s.push_str(" in it anyway, so theirs is fine."); py_string(__s) };
    }
    if py_truthy(&crate::gen::knowledge::cooking::_mentions(&[said.clone(), kept.clone()])) {
    return { let mut __s = String::new(); __s.push_str("Leave the "); __s.push_str(&fstr(&names.clone(), -1, "")); __s.push_str(" out of that one and it still works."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("That one has no "); __s.push_str(&fstr(&names.clone(), -1, "")); __s.push_str(" in it anyway, so you're fine."); py_string(__s) };
    Py::None
}
pub fn _only_an_acknowledgement(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a yes, a thanks or an echo of advice, and asks for nothing.");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)) } }) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ACK_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut understood: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _u: Py = Py::None;
    let mut added: Py = Py::None;
    let mut answering: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut context: Py = Py::None;
    let mut idea: Py = Py::None;
    let mut name: Py = Py::None;
    let mut offered: Py = Py::None;
    let mut others: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut pointed_food: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The next line of a cooking conversation that only narrows the dish already on the table.");
    if py_truthy(&crate::gen::knowledge::lookup::says_it_has_gone_off(&[mcall(&understood.clone(), "get", &[py_str("clean")])])) {
    return Py::None;
    }
    pointed_food = crate::gen::knowledge::cooking::pointed_at_a_food(&[understood.clone(), memory.clone()]);
    if py_truthy(&pointed_food.clone()) {
    return pointed_food.clone();
    }
    plan = plan_recent(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&plan.clone()))) {
    return Py::None;
    }
    said = _clean(&[understood.clone()]);
    if py_truthy(&crate::gen::knowledge::lookup::asks_the_kinds(&[understood.clone()])) {
    return Py::None;
    }
    if py_truthy(&_only_an_acknowledgement(&[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_A_SUM_OF_MONEY(), "search", &[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&crate::gen::knowledge::cooking::_a_brew_not_a_dinner(&[said.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = le(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PICK_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    chosen = { let __b = _chosen(&[plan.clone()]); if py_truthy(&__b) { __b } else { _idea_on_the_plate(&[plan.clone(), memory.clone()]) } };
    if py_truthy(&chosen.clone()) {
    return chosen.clone();
    }
    return Py::None;
    }
    if py_truthy(&ge(&call_builtin("int", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("started")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::None;
    }
    context = _narrowing(&[understood.clone(), plan.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&context.clone()))) {
    if py_truthy(&{ let __b = mcall(&plan.clone(), "get", &[py_str("fancy")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&plan.clone(), "get", &[py_str("have")]))); if !py_truthy(&__b) { __b } else { _a_bare_meal_ask(&[understood.clone()]) } } } }) {
    return plan_reply(&[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plan.clone()); __kws.extend(vec![("have".to_string(), call_builtin("list", &[geti(&plan.clone(), &py_str("fancy"))]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, memory.clone()]);
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("sweet")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&plan.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { mcall(&plan.clone(), "get", &[py_str("offered")]) } })) } }) {
    return Py::None;
    }
    added = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&context.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ let __b = mcall(&plan.clone(), "get", &[py_str("have")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &name.clone()))) {  __out.push(name.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = added.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&context.clone(), "get", &[py_str("avoid")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&context.clone(), "get", &[py_str("veg")]); if py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("vegan")]) } } } })) } }) {
    return Py::None;
    }
    answering = context.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&context.clone(), "get", &[py_str("have")]))); if !py_truthy(&__b) { __b } else { mcall(&context.clone(), "get", &[py_str("fancy")]) } }) {
    answering = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(context.clone()); __kws.extend(vec![("have".to_string(), call_builtin("list", &[geti(&context.clone(), &py_str("fancy"))]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    reply = { let __b = _nothing_to_change(&[understood.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { plan_reply(&[answering.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(py_is(&reply.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&reply.clone(), "strip", &[]), &mcall(&call_builtin("str", &[{ let __b = _last_reply(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])))) {
    others = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(answering.clone()); __kws.extend(vec![("limit".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _ideas(args))(&__args) }) { let __unpk8632 = py_unpack(&__item.clone(), 2, None); let mut _u: Py = __unpk8632[0].clone(); let mut idea: Py = __unpk8632[1].clone(); if py_truthy(&Py::Bool(!py_eq(&idea.clone(), &reply.clone()))) {  __out.push(idea.clone()); } } py_list(__out) })() };
    if py_truthy(&others.clone()) {
    reply = mcall(&py_str(" "), "join", &[slice_or_raise(&others.clone(), &Py::None, &Py::Int(2), &Py::None)]);
    }
    }
    if py_truthy(&call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("meal")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    offered = { let __b = mcall(&context.clone(), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&geti(&memory.clone(), &py_str("meal")), "get", &[py_str("offered")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } } } };
    let _ = py_setitem(&context.clone(), &py_str("offered"), slice_or_raise(&call_builtin("list", &[offered.clone()]).unwrap_or_else(|e| exc::raise(e)), &neg(&Py::Int(4)), &Py::None, &Py::None));
    let _ = py_setitem(&memory.clone(), &py_str("meal"), context.clone());
    }
    return reply.clone();
    Py::None
}
pub fn _a_bare_meal_ask(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("Whether the line asks for a dinner and names nothing else: \"what should i cook tonight\".\n\n    A subject in the line makes it a question of its own - \"what should i eat for protein\" names\n    the protein - and a plan built from a wish is not what it is asking about.\n    ");
    if py_truthy(&{ let __b = crate::gen::knowledge::cooking::ingredients(&[understood.clone()]); if py_truthy(&__b) { __b } else { bitand(&call_builtin("set", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("how"), py_str("how_long"), py_str("how_much"), py_str("why")])) } }) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { sub(&words.clone(), &g_BARE_MEAL_ASK_WORDS()) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[bitand(&words.clone(), &py_set(vec![py_str("cook"), py_str("make"), py_str("cooking"), py_str("dinner"), py_str("tea"), py_str("supper"), py_str("lunch"), py_str("meal"), py_str("eat")]))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _refuses(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line rules a food out: \"they won't eat mushrooms\", \"she doesnt like onions\".");
    if py_truthy(&bitand(&call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)), &g_DISLIKE_WORDS())) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_REFUSAL(), "search", &[_clean(&[understood.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _meal_asked(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("Is this line a meal being asked for, rather than a dish, a bake or two foods?\n\n    A line that names a dish (\"an easy curry\"), a bake (\"a lemon drizzle\") or two foods asks a\n    question the entries and the dish table answer better, so it starts no plan. So does a line\n    that asks the way to cook the one thing it names: that is the method's question.\n    ");
    said = _clean(&[understood.clone()]);
    if py_truthy(&mcall(&g_THE_WAY_TO_COOK(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[_said_words(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&words.clone(), &g_MEAL_FRAME())))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &g_DISH_WORDS()); if py_truthy(&__b) { __b } else { bitand(&words.clone(), &g_BAKING_WORDS()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&bitand(&words.clone(), &py_set(vec![py_str("breakfast"), py_str("lunch"), py_str("brunch")]))) {
    return Py::Bool(false);
    }
    if py_truthy(&gt(&call_builtin("len", &[crate::gen::knowledge::cooking::ingredients(&[understood.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    said = _clean(&[understood.clone()]);
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SAID_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn asks_a_food_fact(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line asks a fact about the food it names instead of asking what to cook.\n\n    \"is oat milk gluten free\" names two things from the kitchen (oat milk and milk), so the\n    pantry table read it as \"what can I make with these\" and answered with a bowl of porridge;\n    the person asked a yes or no, and no dinner idea answers it. Only a question qualifies, and\n    only one that names no cooking of its own: \"what can i make with chicken and rice\" and\n    \"is chicken and rice a good dinner\" keep their meal.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    said = _clean(&[understood.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { mcall(&g_COOK_REQUEST(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&understood.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&understood.clone(), "get", &[py_str("kind")]), &py_str("question"))) } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_FOOD_PROPERTY(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn first_plan(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut context: Py = Py::None;
    let mut key: Py = Py::None;
    let mut plan: Py = Py::None;
    let _ = py_str("A meal for the first line of a conversation when it says what the meal has to be.\n\n    \"whats a quick dinner for two kids who wont eat anything green\" names nothing in the kitchen\n    and asks no dish by name, so nothing else in the base can answer it: the limits it gives\n    (quick, no greens) are enough to pick a dish, and a dish beats a list of ideas.\n    ");
    plan = plan_recent(&[memory.clone()]);
    if py_truthy(&{ let __b = plan.clone(); if !py_truthy(&__b) { __b } else { lt(&call_builtin("int", &[{ let __b = mcall(&plan.clone(), "get", &[py_str("started")]); if py_truthy(&__b) { __b } else { Py::Int(0) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return Py::None;
    }
    context = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __kws.extend(vec![("longest".to_string(), Py::Int(14))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _narrowing(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&context.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_meal_asked(&[understood.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("limit"), py_str("veg"), py_str("vegan"), py_str("no_green"), py_str("occasion"), py_str("simple"), py_str("sweet")])) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_contains(&context.clone(), &key.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&le(&sub(&call_builtin("set", &[context.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("started"), py_str("turn")])), &py_set(vec![py_str("veg"), py_str("vegan"), py_str("diet"), py_str("simple"), py_str("limit")]))) {
    return Py::None;
    }
    if py_truthy(&_left_to_the_meal_entry(&[understood.clone(), context.clone()])) {
    return Py::None;
    }
    return plan_reply(&[context.clone()]);
    Py::None
}
pub fn _left_to_the_meal_entry(args: &[Py]) -> Py {
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
    let mut context: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "context") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: context"))) };
    let mut phrase: Py = Py::None;
    let mut text: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("True when this module's meal entries already answer the line well.\n\n    A table of people or a birthday gets one big oven dish, and a plain \"what should i cook\n    tonight\" gets the ten-minute list: both are in the base, and a plan should only answer when\n    the line says something the entries cannot see.\n    ");
    text = call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() }]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&ge(&crate::gen::knowledge::cooking::_crowd(&[text.clone()]), &Py::Int(6))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = crate::gen::knowledge::cooking::_occasion(&[text.clone()]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("cook"), py_str("dinner"), py_str("meal"), py_str("make"), py_str("eat")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&le(&sub(&call_builtin("set", &[context.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("started"), py_str("turn")])), &py_set(vec![py_str("sweet")]))) {
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_is(&crate::gen::knowledge::lookup::find(&[understood.clone()]), &Py::None))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
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
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    }
    if py_truthy(&{ let __b = le(&call_builtin("set", &[context.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("sweet")])); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_THIN_CUPBOARD_PHRASES()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&le(&call_builtin("set", &[context.clone()]).unwrap_or_else(|e| exc::raise(e)), &py_set(vec![py_str("simple"), py_str("started"), py_str("turn")]))) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _deseed(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The seeds out, by the fruit's own method.");
    words = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("pomegranate"), py_str("pomegranates")]), g__POMEGRANATE_SEEDS()]), py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes")]), g__TOMATO_SEEDS()]), py_tuple(vec![py_tuple(vec![py_str("cucumber"), py_str("cucumbers"), py_str("courgette"), py_str("courgettes"), py_str("marrow")]), g__CUCUMBER_SEEDS()]), py_tuple(vec![py_tuple(vec![py_str("melon"), py_str("melons"), py_str("watermelon")]), g__MELON_SEEDS()]), py_tuple(vec![py_tuple(vec![py_str("pumpkin"), py_str("squash"), py_str("butternut")]), g__SQUASH_SEEDS()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    family = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&family.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return reply.clone();
    }
    }
    return g__CHILLI_SEEDS();
    Py::None
}
pub fn _one_person_at_this_meal(args: &[Py]) -> Py {
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
    let mut field: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("Whether the line is one person's meal rather than a table to cook for.\n\n    \"what shall i make for lunch for four of us\" is a table of four, and the shape for one\n    person's lunch - a sandwich and a piece of fruit - is not what four people cook. The crowd\n    row answers a counted table, so this row stands aside for it. Both the line as typed and the\n    reading are asked: the reading puts the first person in (\"for four of me\"), and a table can\n    only be counted off the words that were typed.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&understood.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean")]));
    for __item in __iter1 {
    field = __item.clone();
    line = mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[field.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&{ let __b = line.clone(); if !py_truthy(&__b) { __b } else { crate::gen::knowledge::cooking::_table_text(&[line.clone()]) } }) {
    return Py::Bool(false);
    }
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_meal_for_the_slot(args: &[Py]) -> Py {
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
    let mut slot: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A shape for the meal the line names, when it names no food to cook.");
    text = add(&add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_SLOT_IDEAS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    slot = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&text.clone(), &add(&py_str(" "), &slot.clone()))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &add(&slot.clone(), &py_str("s ")))) } }) {
    return reply.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.cooking_more".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABSENCE" => Some(g_ABSENCE()),
            "ACK_WORDS" => Some(g_ACK_WORDS()),
            "ALLERGY_PHRASES" => Some(g_ALLERGY_PHRASES()),
            "ALLERGY_WORDS" => Some(g_ALLERGY_WORDS()),
            "AVOID_TEXT" => Some(g_AVOID_TEXT()),
            "A_SIDE_WITH_NO_DISH" => Some(g_A_SIDE_WITH_NO_DISH()),
            "A_SUM_OF_MONEY" => Some(g_A_SUM_OF_MONEY()),
            "A_WISH_FOR_A_FOOD" => Some(g_A_WISH_FOR_A_FOOD()),
            "BAKING_WORDS" => Some(g_BAKING_WORDS()),
            "BARE_MEAL_ASK_WORDS" => Some(g_BARE_MEAL_ASK_WORDS()),
            "BARE_MEAL_NOUNS" => Some(g_BARE_MEAL_NOUNS()),
            "BARE_MEAL_OBJECT" => Some(g_BARE_MEAL_OBJECT()),
            "BIG_DAYS" => Some(g_BIG_DAYS()),
            "CHATTER" => Some(g_CHATTER()),
            "CHILLI_FRUIT" => Some(g_CHILLI_FRUIT()),
            "COMING_FOR" => Some(g_COMING_FOR()),
            "COOKED_THROUGH_WORDS" => Some(g_COOKED_THROUGH_WORDS()),
            "COOKING_FOR" => Some(g_COOKING_FOR()),
            "COOK_REQUEST" => Some(g_COOK_REQUEST()),
            "CROWD_DISHES" => Some(g_CROWD_DISHES()),
            "CROWD_PHRASE" => Some(g_CROWD_PHRASE()),
            "DAIRY_FOODS" => Some(g_DAIRY_FOODS()),
            "DAIRY_FREE_PHRASES" => Some(g_DAIRY_FREE_PHRASES()),
            "DINNER_WORDS" => Some(g_DINNER_WORDS()),
            "DISH_WORDS" => Some(g_DISH_WORDS()),
            "DISLIKE_WORDS" => Some(g_DISLIKE_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EVENT_WORDS" => Some(g_EVENT_WORDS()),
            "FATTENING_WORDS" => Some(g_FATTENING_WORDS()),
            "FOOD_PROPERTY" => Some(g_FOOD_PROPERTY()),
            "FOR_ONE" => Some(g_FOR_ONE()),
            "GREEN_IN_TEXT" => Some(g_GREEN_IN_TEXT()),
            "GREEN_NAMES" => Some(g_GREEN_NAMES()),
            "GREEN_WORDS" => Some(g_GREEN_WORDS()),
            "HEAT_INGREDIENTS" => Some(g_HEAT_INGREDIENTS()),
            "HEAT_IN_TEXT" => Some(g_HEAT_IN_TEXT()),
            "HEAT_WORDS" => Some(g_HEAT_WORDS()),
            "HOLD_WORDS" => Some(g_HOLD_WORDS()),
            "LIMIT_WORDS" => Some(g_LIMIT_WORDS()),
            "MEAL_FRAME" => Some(g_MEAL_FRAME()),
            "MEAT_FREE_PHRASES" => Some(g_MEAT_FREE_PHRASES()),
            "MEAT_NAMES" => Some(g_MEAT_NAMES()),
            "MENU_PHRASES" => Some(g_MENU_PHRASES()),
            "METHOD_PHRASES" => Some(g_METHOD_PHRASES()),
            "MINUTES" => Some(g_MINUTES()),
            "NEW_MEAL_PHRASES" => Some(g_NEW_MEAL_PHRASES()),
            "NOT_A_DONENESS_QUESTION" => Some(g_NOT_A_DONENESS_QUESTION()),
            "NOT_A_TOPPING_ASK" => Some(g_NOT_A_TOPPING_ASK()),
            "ONLY_PHRASES" => Some(g_ONLY_PHRASES()),
            "ON_TOPPINGS" => Some(g_ON_TOPPINGS()),
            "ON_WORDS" => Some(g_ON_WORDS()),
            "PANTRY_MINUTES" => Some(g_PANTRY_MINUTES()),
            "PEOPLE_WORDS" => Some(g_PEOPLE_WORDS()),
            "PERSON_WORDS" => Some(g_PERSON_WORDS()),
            "PICK_PHRASES" => Some(g_PICK_PHRASES()),
            "PLAN_KEYS" => Some(g_PLAN_KEYS()),
            "PLAN_LAPSE" => Some(g_PLAN_LAPSE()),
            "QUICK_MINUTES" => Some(g_QUICK_MINUTES()),
            "REFUSAL" => Some(g_REFUSAL()),
            "SAID_PHRASES" => Some(g_SAID_PHRASES()),
            "SEED_FRUIT" => Some(g_SEED_FRUIT()),
            "SIMPLE_PHRASES" => Some(g_SIMPLE_PHRASES()),
            "SLOT_ASKS" => Some(g_SLOT_ASKS()),
            "SLOT_IDEAS" => Some(g_SLOT_IDEAS()),
            "SOMEONE_ELSE" => Some(g_SOMEONE_ELSE()),
            "STAPLES" => Some(g_STAPLES()),
            "STOCK_PHRASES" => Some(g_STOCK_PHRASES()),
            "SWAP_PHRASES" => Some(g_SWAP_PHRASES()),
            "SWEET_DISHES" => Some(g_SWEET_DISHES()),
            "SWEET_PHRASES" => Some(g_SWEET_PHRASES()),
            "SWEET_WORDS" => Some(g_SWEET_WORDS()),
            "TASTE_WORDS" => Some(g_TASTE_WORDS()),
            "TECHNIQUE_WORDS" => Some(g_TECHNIQUE_WORDS()),
            "THE_FOOD_IN_HAND" => Some(g_THE_FOOD_IN_HAND()),
            "THE_WAY_TO_COOK" => Some(g_THE_WAY_TO_COOK()),
            "THIN" => Some(g_THIN()),
            "THIN_CUPBOARD_PHRASES" => Some(g_THIN_CUPBOARD_PHRASES()),
            "VEGAN_BAD" => Some(g_VEGAN_BAD()),
            "VEG_WORDS" => Some(g_VEG_WORDS()),
            "WAKE_WORDS" => Some(g_WAKE_WORDS()),
            "WITH_PHRASES" => Some(g_WITH_PHRASES()),
            "_ABOUT_A_FOOD" => Some(g__ABOUT_A_FOOD()),
            "_AHEAD" => Some(g__AHEAD()),
            "_APPLE_BROWN" => Some(g__APPLE_BROWN()),
            "_A_PRONOUN_HELD" => Some(g__A_PRONOUN_HELD()),
            "_A_REACTION" => Some(g__A_REACTION()),
            "_BROWN_KINDS" => Some(g__BROWN_KINDS()),
            "_BROWN_SAFE" => Some(g__BROWN_SAFE()),
            "_BROWN_SAFE_ASKS" => Some(g__BROWN_SAFE_ASKS()),
            "_CELERY_BROWN" => Some(g__CELERY_BROWN()),
            "_CHILLI_SEEDS" => Some(g__CHILLI_SEEDS()),
            "_COLD_HELD" => Some(g__COLD_HELD()),
            "_CUCUMBER_SEEDS" => Some(g__CUCUMBER_SEEDS()),
            "_DRY_BROWN" => Some(g__DRY_BROWN()),
            "_FLATBREAD_KINDS" => Some(g__FLATBREAD_KINDS()),
            "_FLATBREAD_RECIPES" => Some(g__FLATBREAD_RECIPES()),
            "_FOOD_IN_LINE" => Some(g__FOOD_IN_LINE()),
            "_FOOD_JOB" => Some(g__FOOD_JOB()),
            "_GROWING_WORD" => Some(g__GROWING_WORD()),
            "_HOT_FOOD_RULED_OUT" => Some(g__HOT_FOOD_RULED_OUT()),
            "_HOT_HELD" => Some(g__HOT_HELD()),
            "_MELON_SEEDS" => Some(g__MELON_SEEDS()),
            "_MUSHROOM_BROWN" => Some(g__MUSHROOM_BROWN()),
            "_MUSHROOM_SAFE" => Some(g__MUSHROOM_SAFE()),
            "_MUSHROOM_WORDS" => Some(g__MUSHROOM_WORDS()),
            "_OUR_OWN" => Some(g__OUR_OWN()),
            "_POMEGRANATE_SEEDS" => Some(g__POMEGRANATE_SEEDS()),
            "_PRODUCE_BROWN" => Some(g__PRODUCE_BROWN()),
            "_SQUASH_SEEDS" => Some(g__SQUASH_SEEDS()),
            "_TASTE_RULED_OUT" => Some(g__TASTE_RULED_OUT()),
            "_TOMATO_SEEDS" => Some(g__TOMATO_SEEDS()),
            "_WET_BROWN" => Some(g__WET_BROWN()),
            "_a_bare_meal_ask" => Some(py_func("_a_bare_meal_ask", |args: &[Py]| _a_bare_meal_ask(args))),
            "_a_crop_not_a_larder" => Some(py_func("_a_crop_not_a_larder", |args: &[Py]| _a_crop_not_a_larder(args))),
            "_a_meal_for_the_slot" => Some(py_func("_a_meal_for_the_slot", |args: &[Py]| _a_meal_for_the_slot(args))),
            "_a_side_or_the_three_jobs" => Some(py_func("_a_side_or_the_three_jobs", |args: &[Py]| _a_side_or_the_three_jobs(args))),
            "_a_taste_ruled_out" => Some(py_func("_a_taste_ruled_out", |args: &[Py]| _a_taste_ruled_out(args))),
            "_about_someone_else" => Some(py_func("_about_someone_else", |args: &[Py]| _about_someone_else(args))),
            "_about_the_food" => Some(py_func("_about_the_food", |args: &[Py]| _about_the_food(args))),
            "_allergy_names" => Some(py_func("_allergy_names", |args: &[Py]| _allergy_names(args))),
            "_an_allergy" => Some(py_func("_an_allergy", |args: &[Py]| _an_allergy(args))),
            "_asks_about_a_food_it_has_not_got" => Some(py_func("_asks_about_a_food_it_has_not_got", |args: &[Py]| _asks_about_a_food_it_has_not_got(args))),
            "_buffet_holding" => Some(py_func("_buffet_holding", |args: &[Py]| _buffet_holding(args))),
            "_chosen" => Some(py_func("_chosen", |args: &[Py]| _chosen(args))),
            "_clean" => Some(py_func("_clean", |args: &[Py]| _clean(args))),
            "_deseed" => Some(py_func("_deseed", |args: &[Py]| _deseed(args))),
            "_fits" => Some(py_func("_fits", |args: &[Py]| _fits(args))),
            "_flatbread_answer" => Some(py_func("_flatbread_answer", |args: &[Py]| _flatbread_answer(args))),
            "_idea_on_the_plate" => Some(py_func("_idea_on_the_plate", |args: &[Py]| _idea_on_the_plate(args))),
            "_ideas" => Some(py_func("_ideas", |args: &[Py]| _ideas(args))),
            "_is_allergy_food" => Some(py_func("_is_allergy_food", |args: &[Py]| _is_allergy_food(args))),
            "_last_reply" => Some(py_func("_last_reply", |args: &[Py]| _last_reply(args))),
            "_left_to_the_meal_entry" => Some(py_func("_left_to_the_meal_entry", |args: &[Py]| _left_to_the_meal_entry(args))),
            "_meal_asked" => Some(py_func("_meal_asked", |args: &[Py]| _meal_asked(args))),
            "_meal_idea_for" => Some(py_func("_meal_idea_for", |args: &[Py]| _meal_idea_for(args))),
            "_narrowing" => Some(py_func("_narrowing", |args: &[Py]| _narrowing(args))),
            "_nothing_to_change" => Some(py_func("_nothing_to_change", |args: &[Py]| _nothing_to_change(args))),
            "_one_person_at_this_meal" => Some(py_func("_one_person_at_this_meal", |args: &[Py]| _one_person_at_this_meal(args))),
            "_only_an_acknowledgement" => Some(py_func("_only_an_acknowledgement", |args: &[Py]| _only_an_acknowledgement(args))),
            "_produce_brown" => Some(py_func("_produce_brown", |args: &[Py]| _produce_brown(args))),
            "_quick_enough" => Some(py_func("_quick_enough", |args: &[Py]| _quick_enough(args))),
            "_quick_limit" => Some(py_func("_quick_limit", |args: &[Py]| _quick_limit(args))),
            "_refuses" => Some(py_func("_refuses", |args: &[Py]| _refuses(args))),
            "_reply" => Some(g__reply()),
            "_said_words" => Some(py_func("_said_words", |args: &[Py]| _said_words(args))),
            "_score" => Some(py_func("_score", |args: &[Py]| _score(args))),
            "_staple" => Some(g__staple()),
            "_text_conflict" => Some(py_func("_text_conflict", |args: &[Py]| _text_conflict(args))),
            "_the_bare_ideas_are_about_food" => Some(py_func("_the_bare_ideas_are_about_food", |args: &[Py]| _the_bare_ideas_are_about_food(args))),
            "_the_foods_wanted_more" => Some(py_func("_the_foods_wanted_more", |args: &[Py]| _the_foods_wanted_more(args))),
            "_the_foods_wanted_more_or_less" => Some(py_func("_the_foods_wanted_more_or_less", |args: &[Py]| _the_foods_wanted_more_or_less(args))),
            "_wake_food" => Some(py_func("_wake_food", |args: &[Py]| _wake_food(args))),
            "_with_heat" => Some(py_func("_with_heat", |args: &[Py]| _with_heat(args))),
            "_words" => Some(g__words()),
            "asks_a_food_fact" => Some(py_func("asks_a_food_fact", |args: &[Py]| asks_a_food_fact(args))),
            "first_plan" => Some(py_func("first_plan", |args: &[Py]| first_plan(args))),
            "follow_up" => Some(py_func("follow_up", |args: &[Py]| follow_up(args))),
            "minutes_said" => Some(py_func("minutes_said", |args: &[Py]| minutes_said(args))),
            "plan_recent" => Some(py_func("plan_recent", |args: &[Py]| plan_recent(args))),
            "plan_reply" => Some(py_func("plan_reply", |args: &[Py]| plan_reply(args))),
            "remembered" => Some(py_func("remembered", |args: &[Py]| remembered(args))),
            "wants" => Some(py_func("wants", |args: &[Py]| wants(args))),
            _ => None,
        }),
        set: None,
    }))
}

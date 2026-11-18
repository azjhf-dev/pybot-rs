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


thread_local! { static G_AS_KITCHEN_SAYS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BIRD_GRAMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CORRECTING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COUNTED_PACKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISHES_FOR_A_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FAN_COOLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GAS_OVEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRAMS_PER_CUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GRAMS_PER_TSP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LAND_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIQUID_GRAMS_LITRE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOOSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MADE_NOT_MEASURED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OBJECT_GRAMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACK_DRAINED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACK_GRAMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACK_IN_ML: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PER_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PER_PERSON_WHEN_COUNTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PIECE_GRAMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAID_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYSTEM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNITS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHOLE_THING_SERVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AND_A_HALF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AREA_PICTURES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKED_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKING_AFTER_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_FEET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_HOW_MANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CLOCK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COMPLAINT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CUT_NOT_A_BIRD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DISH_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FIGURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FRACTION_AS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FRACTION_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FRACTION_OF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FRACTION_OF_A_MIXED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_HALF_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEASURE_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MEASURE_SAID_ANOTHER_WAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_MIXED_SPAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SIZE_JUDGEMENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_SPAN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TARGET_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_IS_THE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_TIME_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_UNIT_ASKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_WHOLE_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BAKING_PORTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_FOR_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BARE_TEMPERATURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BIRDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARRIED_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARRIED_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARRIED_NOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARRIED_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHILLI_THE_VEGETABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COMPONENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COMPONENT_MEASURED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CONTAINER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CORRECTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNTED_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_CONTENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_OF_BIRDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__C_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DENOMINATOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISH_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRY_IN_A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FEEDS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__F_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GROUP_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEIGHT_CM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEIGHT_FEET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEIGHT_FEET_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEIGHT_METRES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_TO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__IN_A_VESSEL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__JOINED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEPING_HENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAND_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOOSE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LOOSE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MEASURE_AFTER_PIECE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MIXED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEW_WEIGHT_THEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_ABOUT_A_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_HEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TABLE_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_TABLE_QUESTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NUMERAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PACK_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PACK_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PEOPLE_WORD_TO_TWENTY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PER_CUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLURAL_BIRDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PORTIONS_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PORTION_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PORTION_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PORTION_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__POUNDS_IS_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_COUNT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_WHOLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAYS_OWN_HEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAY_K: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_OF_CUP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_THAT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHARE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SLASH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPECIFIC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SPLIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TABLE_GROUPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TARGET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_ARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_PLURAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_SECONDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_SMALLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIME_UNIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TIN_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRAILING_JOIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNCOUNTED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_AND_A_HALF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__UNIT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VESSEL_ML: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEIGH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WEIGHT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHOLE_PIECE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHOLE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WHOLE_WEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_FRACTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.units: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_AS_KITCHEN_SAYS() -> Py { ensure_init(); G_AS_KITCHEN_SAYS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AS_KITCHEN_SAYS(v: Py) { ensure_init(); G_AS_KITCHEN_SAYS.with(|c| *c.borrow_mut() = v); }
pub fn g_BIRD_GRAMS() -> Py { ensure_init(); G_BIRD_GRAMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BIRD_GRAMS(v: Py) { ensure_init(); G_BIRD_GRAMS.with(|c| *c.borrow_mut() = v); }
pub fn g_CORRECTING() -> Py { ensure_init(); G_CORRECTING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CORRECTING(v: Py) { ensure_init(); G_CORRECTING.with(|c| *c.borrow_mut() = v); }
pub fn g_COUNTED_PACKS() -> Py { ensure_init(); G_COUNTED_PACKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COUNTED_PACKS(v: Py) { ensure_init(); G_COUNTED_PACKS.with(|c| *c.borrow_mut() = v); }
pub fn g_DISHES_FOR_A_TABLE() -> Py { ensure_init(); G_DISHES_FOR_A_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISHES_FOR_A_TABLE(v: Py) { ensure_init(); G_DISHES_FOR_A_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_FAN_COOLER() -> Py { ensure_init(); G_FAN_COOLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FAN_COOLER(v: Py) { ensure_init(); G_FAN_COOLER.with(|c| *c.borrow_mut() = v); }
pub fn g_GAS_OVEN() -> Py { ensure_init(); G_GAS_OVEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GAS_OVEN(v: Py) { ensure_init(); G_GAS_OVEN.with(|c| *c.borrow_mut() = v); }
pub fn g_GRAMS_PER_CUP() -> Py { ensure_init(); G_GRAMS_PER_CUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRAMS_PER_CUP(v: Py) { ensure_init(); G_GRAMS_PER_CUP.with(|c| *c.borrow_mut() = v); }
pub fn g_GRAMS_PER_TSP() -> Py { ensure_init(); G_GRAMS_PER_TSP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GRAMS_PER_TSP(v: Py) { ensure_init(); G_GRAMS_PER_TSP.with(|c| *c.borrow_mut() = v); }
pub fn g_LAND_PLAIN() -> Py { ensure_init(); G_LAND_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LAND_PLAIN(v: Py) { ensure_init(); G_LAND_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_LIQUID_GRAMS_LITRE() -> Py { ensure_init(); G_LIQUID_GRAMS_LITRE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIQUID_GRAMS_LITRE(v: Py) { ensure_init(); G_LIQUID_GRAMS_LITRE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOOSE() -> Py { ensure_init(); G_LOOSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOOSE(v: Py) { ensure_init(); G_LOOSE.with(|c| *c.borrow_mut() = v); }
pub fn g_MADE_NOT_MEASURED() -> Py { ensure_init(); G_MADE_NOT_MEASURED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MADE_NOT_MEASURED(v: Py) { ensure_init(); G_MADE_NOT_MEASURED.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TABLE() -> Py { ensure_init(); G_NOT_A_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TABLE(v: Py) { ensure_init(); G_NOT_A_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_OBJECT_GRAMS() -> Py { ensure_init(); G_OBJECT_GRAMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OBJECT_GRAMS(v: Py) { ensure_init(); G_OBJECT_GRAMS.with(|c| *c.borrow_mut() = v); }
pub fn g_PACK_DRAINED() -> Py { ensure_init(); G_PACK_DRAINED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACK_DRAINED(v: Py) { ensure_init(); G_PACK_DRAINED.with(|c| *c.borrow_mut() = v); }
pub fn g_PACK_GRAMS() -> Py { ensure_init(); G_PACK_GRAMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACK_GRAMS(v: Py) { ensure_init(); G_PACK_GRAMS.with(|c| *c.borrow_mut() = v); }
pub fn g_PACK_IN_ML() -> Py { ensure_init(); G_PACK_IN_ML.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACK_IN_ML(v: Py) { ensure_init(); G_PACK_IN_ML.with(|c| *c.borrow_mut() = v); }
pub fn g_PER_PERSON() -> Py { ensure_init(); G_PER_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PER_PERSON(v: Py) { ensure_init(); G_PER_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_PER_PERSON_WHEN_COUNTED() -> Py { ensure_init(); G_PER_PERSON_WHEN_COUNTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PER_PERSON_WHEN_COUNTED(v: Py) { ensure_init(); G_PER_PERSON_WHEN_COUNTED.with(|c| *c.borrow_mut() = v); }
pub fn g_PIECE_GRAMS() -> Py { ensure_init(); G_PIECE_GRAMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PIECE_GRAMS(v: Py) { ensure_init(); G_PIECE_GRAMS.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID() -> Py { ensure_init(); G_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID(v: Py) { ensure_init(); G_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_SAID_ONE() -> Py { ensure_init(); G_SAID_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAID_ONE(v: Py) { ensure_init(); G_SAID_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g_SYSTEM() -> Py { ensure_init(); G_SYSTEM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYSTEM(v: Py) { ensure_init(); G_SYSTEM.with(|c| *c.borrow_mut() = v); }
pub fn g_UNITS() -> Py { ensure_init(); G_UNITS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNITS(v: Py) { ensure_init(); G_UNITS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHOLE_THING_SERVES() -> Py { ensure_init(); G_WHOLE_THING_SERVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHOLE_THING_SERVES(v: Py) { ensure_init(); G_WHOLE_THING_SERVES.with(|c| *c.borrow_mut() = v); }
pub fn g__AND_A_HALF() -> Py { ensure_init(); G__AND_A_HALF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AND_A_HALF(v: Py) { ensure_init(); G__AND_A_HALF.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_ASK() -> Py { ensure_init(); G__AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_ASK(v: Py) { ensure_init(); G__AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__AREA_PICTURES() -> Py { ensure_init(); G__AREA_PICTURES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AREA_PICTURES(v: Py) { ensure_init(); G__AREA_PICTURES.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKED_MEASURE() -> Py { ensure_init(); G__ASKED_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKED_MEASURE(v: Py) { ensure_init(); G__ASKED_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKING_AFTER_MEASURE() -> Py { ensure_init(); G__ASKING_AFTER_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKING_AFTER_MEASURE(v: Py) { ensure_init(); G__ASKING_AFTER_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_ELSEWHERE() -> Py { ensure_init(); G__ASKS_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_ELSEWHERE(v: Py) { ensure_init(); G__ASKS_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_FEET() -> Py { ensure_init(); G__ASKS_FEET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_FEET(v: Py) { ensure_init(); G__ASKS_FEET.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_HOW_MANY() -> Py { ensure_init(); G__ASKS_HOW_MANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_HOW_MANY(v: Py) { ensure_init(); G__ASKS_HOW_MANY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CLOCK() -> Py { ensure_init(); G__A_CLOCK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CLOCK(v: Py) { ensure_init(); G__A_CLOCK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COMPLAINT() -> Py { ensure_init(); G__A_COMPLAINT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COMPLAINT(v: Py) { ensure_init(); G__A_COMPLAINT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_COUNT() -> Py { ensure_init(); G__A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_COUNT(v: Py) { ensure_init(); G__A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CUT_NOT_A_BIRD() -> Py { ensure_init(); G__A_CUT_NOT_A_BIRD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CUT_NOT_A_BIRD(v: Py) { ensure_init(); G__A_CUT_NOT_A_BIRD.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DISH_PART() -> Py { ensure_init(); G__A_DISH_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DISH_PART(v: Py) { ensure_init(); G__A_DISH_PART.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FIGURE() -> Py { ensure_init(); G__A_FIGURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); G__A_FIGURE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FRACTION() -> Py { ensure_init(); G__A_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FRACTION(v: Py) { ensure_init(); G__A_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FRACTION_AS() -> Py { ensure_init(); G__A_FRACTION_AS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FRACTION_AS(v: Py) { ensure_init(); G__A_FRACTION_AS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FRACTION_ASK() -> Py { ensure_init(); G__A_FRACTION_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FRACTION_ASK(v: Py) { ensure_init(); G__A_FRACTION_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FRACTION_OF() -> Py { ensure_init(); G__A_FRACTION_OF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FRACTION_OF(v: Py) { ensure_init(); G__A_FRACTION_OF.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FRACTION_OF_A_MIXED() -> Py { ensure_init(); G__A_FRACTION_OF_A_MIXED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FRACTION_OF_A_MIXED(v: Py) { ensure_init(); G__A_FRACTION_OF_A_MIXED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_HALF_SPAN() -> Py { ensure_init(); G__A_HALF_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_HALF_SPAN(v: Py) { ensure_init(); G__A_HALF_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEASURE_QUESTION() -> Py { ensure_init(); G__A_MEASURE_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEASURE_QUESTION(v: Py) { ensure_init(); G__A_MEASURE_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MEASURE_SAID_ANOTHER_WAY() -> Py { ensure_init(); G__A_MEASURE_SAID_ANOTHER_WAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MEASURE_SAID_ANOTHER_WAY(v: Py) { ensure_init(); G__A_MEASURE_SAID_ANOTHER_WAY.with(|c| *c.borrow_mut() = v); }
pub fn g__A_MIXED_SPAN() -> Py { ensure_init(); G__A_MIXED_SPAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_MIXED_SPAN(v: Py) { ensure_init(); G__A_MIXED_SPAN.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SIZE_JUDGEMENT() -> Py { ensure_init(); G__A_SIZE_JUDGEMENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SIZE_JUDGEMENT(v: Py) { ensure_init(); G__A_SIZE_JUDGEMENT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_SPAN_ASK() -> Py { ensure_init(); G__A_SPAN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_SPAN_ASK(v: Py) { ensure_init(); G__A_SPAN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TARGET_UNIT() -> Py { ensure_init(); G__A_TARGET_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TARGET_UNIT(v: Py) { ensure_init(); G__A_TARGET_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_IS_THE_ASK() -> Py { ensure_init(); G__A_TIME_IS_THE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_IS_THE_ASK(v: Py) { ensure_init(); G__A_TIME_IS_THE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_TIME_SHARE() -> Py { ensure_init(); G__A_TIME_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_TIME_SHARE(v: Py) { ensure_init(); G__A_TIME_SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_UNIT_ASKED() -> Py { ensure_init(); G__A_UNIT_ASKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_UNIT_ASKED(v: Py) { ensure_init(); G__A_UNIT_ASKED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_WHOLE_UNIT() -> Py { ensure_init(); G__A_WHOLE_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_WHOLE_UNIT(v: Py) { ensure_init(); G__A_WHOLE_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__BAKING_PORTION() -> Py { ensure_init(); G__BAKING_PORTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BAKING_PORTION(v: Py) { ensure_init(); G__BAKING_PORTION.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_FOR_COUNT() -> Py { ensure_init(); G__BARE_FOR_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_FOR_COUNT(v: Py) { ensure_init(); G__BARE_FOR_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_JOIN() -> Py { ensure_init(); G__BARE_JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_JOIN(v: Py) { ensure_init(); G__BARE_JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g__BARE_TEMPERATURE() -> Py { ensure_init(); G__BARE_TEMPERATURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BARE_TEMPERATURE(v: Py) { ensure_init(); G__BARE_TEMPERATURE.with(|c| *c.borrow_mut() = v); }
pub fn g__BIRDS() -> Py { ensure_init(); G__BIRDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BIRDS(v: Py) { ensure_init(); G__BIRDS.with(|c| *c.borrow_mut() = v); }
pub fn g__CARRIED_FILLER() -> Py { ensure_init(); G__CARRIED_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARRIED_FILLER(v: Py) { ensure_init(); G__CARRIED_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g__CARRIED_LEAD() -> Py { ensure_init(); G__CARRIED_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARRIED_LEAD(v: Py) { ensure_init(); G__CARRIED_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__CARRIED_NOW() -> Py { ensure_init(); G__CARRIED_NOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARRIED_NOW(v: Py) { ensure_init(); G__CARRIED_NOW.with(|c| *c.borrow_mut() = v); }
pub fn g__CARRIED_PEOPLE() -> Py { ensure_init(); G__CARRIED_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARRIED_PEOPLE(v: Py) { ensure_init(); G__CARRIED_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__CHILLI_THE_VEGETABLE() -> Py { ensure_init(); G__CHILLI_THE_VEGETABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHILLI_THE_VEGETABLE(v: Py) { ensure_init(); G__CHILLI_THE_VEGETABLE.with(|c| *c.borrow_mut() = v); }
pub fn g__COMPONENT() -> Py { ensure_init(); G__COMPONENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COMPONENT(v: Py) { ensure_init(); G__COMPONENT.with(|c| *c.borrow_mut() = v); }
pub fn g__COMPONENT_MEASURED() -> Py { ensure_init(); G__COMPONENT_MEASURED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COMPONENT_MEASURED(v: Py) { ensure_init(); G__COMPONENT_MEASURED.with(|c| *c.borrow_mut() = v); }
pub fn g__CONTAINER() -> Py { ensure_init(); G__CONTAINER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CONTAINER(v: Py) { ensure_init(); G__CONTAINER.with(|c| *c.borrow_mut() = v); }
pub fn g__CORRECTION() -> Py { ensure_init(); G__CORRECTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CORRECTION(v: Py) { ensure_init(); G__CORRECTION.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT() -> Py { ensure_init(); G__COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT(v: Py) { ensure_init(); G__COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNTED_THING() -> Py { ensure_init(); G__COUNTED_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNTED_THING(v: Py) { ensure_init(); G__COUNTED_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_ASK() -> Py { ensure_init(); G__COUNT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_ASK(v: Py) { ensure_init(); G__COUNT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_CONTENT() -> Py { ensure_init(); G__COUNT_CONTENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_CONTENT(v: Py) { ensure_init(); G__COUNT_CONTENT.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_OF_BIRDS() -> Py { ensure_init(); G__COUNT_OF_BIRDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_OF_BIRDS(v: Py) { ensure_init(); G__COUNT_OF_BIRDS.with(|c| *c.borrow_mut() = v); }
pub fn g__C_ASKS() -> Py { ensure_init(); G__C_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__C_ASKS(v: Py) { ensure_init(); G__C_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__DENOMINATOR() -> Py { ensure_init(); G__DENOMINATOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DENOMINATOR(v: Py) { ensure_init(); G__DENOMINATOR.with(|c| *c.borrow_mut() = v); }
pub fn g__DISH_AFTER() -> Py { ensure_init(); G__DISH_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISH_AFTER(v: Py) { ensure_init(); G__DISH_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__DRY_IN_A_MEASURE() -> Py { ensure_init(); G__DRY_IN_A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRY_IN_A_MEASURE(v: Py) { ensure_init(); G__DRY_IN_A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g__FEEDS_ASK() -> Py { ensure_init(); G__FEEDS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FEEDS_ASK(v: Py) { ensure_init(); G__FEEDS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTION() -> Py { ensure_init(); G__FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTION(v: Py) { ensure_init(); G__FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__FRACTIONS() -> Py { ensure_init(); G__FRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FRACTIONS(v: Py) { ensure_init(); G__FRACTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g__F_ASKS() -> Py { ensure_init(); G__F_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__F_ASKS(v: Py) { ensure_init(); G__F_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__GROUP_VALUE() -> Py { ensure_init(); G__GROUP_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GROUP_VALUE(v: Py) { ensure_init(); G__GROUP_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g__HEIGHT_CM() -> Py { ensure_init(); G__HEIGHT_CM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEIGHT_CM(v: Py) { ensure_init(); G__HEIGHT_CM.with(|c| *c.borrow_mut() = v); }
pub fn g__HEIGHT_FEET() -> Py { ensure_init(); G__HEIGHT_FEET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEIGHT_FEET(v: Py) { ensure_init(); G__HEIGHT_FEET.with(|c| *c.borrow_mut() = v); }
pub fn g__HEIGHT_FEET_WORD() -> Py { ensure_init(); G__HEIGHT_FEET_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEIGHT_FEET_WORD(v: Py) { ensure_init(); G__HEIGHT_FEET_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__HEIGHT_METRES() -> Py { ensure_init(); G__HEIGHT_METRES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEIGHT_METRES(v: Py) { ensure_init(); G__HEIGHT_METRES.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_TO() -> Py { ensure_init(); G__HOW_TO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_TO(v: Py) { ensure_init(); G__HOW_TO.with(|c| *c.borrow_mut() = v); }
pub fn g__IN_A_VESSEL() -> Py { ensure_init(); G__IN_A_VESSEL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__IN_A_VESSEL(v: Py) { ensure_init(); G__IN_A_VESSEL.with(|c| *c.borrow_mut() = v); }
pub fn g__JOINED() -> Py { ensure_init(); G__JOINED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__JOINED(v: Py) { ensure_init(); G__JOINED.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEPING_HENS() -> Py { ensure_init(); G__KEEPING_HENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEPING_HENS(v: Py) { ensure_init(); G__KEEPING_HENS.with(|c| *c.borrow_mut() = v); }
pub fn g__LAND_ASK() -> Py { ensure_init(); G__LAND_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAND_ASK(v: Py) { ensure_init(); G__LAND_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__LOOSE_ASK() -> Py { ensure_init(); G__LOOSE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOOSE_ASK(v: Py) { ensure_init(); G__LOOSE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__LOOSE_NOT() -> Py { ensure_init(); G__LOOSE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LOOSE_NOT(v: Py) { ensure_init(); G__LOOSE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g__MEASURE_AFTER_PIECE() -> Py { ensure_init(); G__MEASURE_AFTER_PIECE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MEASURE_AFTER_PIECE(v: Py) { ensure_init(); G__MEASURE_AFTER_PIECE.with(|c| *c.borrow_mut() = v); }
pub fn g__MIXED() -> Py { ensure_init(); G__MIXED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MIXED(v: Py) { ensure_init(); G__MIXED.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMES() -> Py { ensure_init(); G__NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMES(v: Py) { ensure_init(); G__NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__NEW_WEIGHT_THEN() -> Py { ensure_init(); G__NEW_WEIGHT_THEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEW_WEIGHT_THEN(v: Py) { ensure_init(); G__NEW_WEIGHT_THEN.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_ABOUT_A_MEASURE() -> Py { ensure_init(); G__NOT_ABOUT_A_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_ABOUT_A_MEASURE(v: Py) { ensure_init(); G__NOT_ABOUT_A_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_COUNT() -> Py { ensure_init(); G__NOT_A_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_COUNT(v: Py) { ensure_init(); G__NOT_A_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_FOOD() -> Py { ensure_init(); G__NOT_A_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_FOOD(v: Py) { ensure_init(); G__NOT_A_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_HEIGHT() -> Py { ensure_init(); G__NOT_A_HEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_HEIGHT(v: Py) { ensure_init(); G__NOT_A_HEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TABLE_BEFORE() -> Py { ensure_init(); G__NOT_A_TABLE_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TABLE_BEFORE(v: Py) { ensure_init(); G__NOT_A_TABLE_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_TABLE_QUESTION() -> Py { ensure_init(); G__NOT_A_TABLE_QUESTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_TABLE_QUESTION(v: Py) { ensure_init(); G__NOT_A_TABLE_QUESTION.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_PLURAL() -> Py { ensure_init(); G__NO_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_PLURAL(v: Py) { ensure_init(); G__NO_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMBER() -> Py { ensure_init(); G__NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMBER(v: Py) { ensure_init(); G__NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g__NUMERAL() -> Py { ensure_init(); G__NUMERAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NUMERAL(v: Py) { ensure_init(); G__NUMERAL.with(|c| *c.borrow_mut() = v); }
pub fn g__PACK_ASK() -> Py { ensure_init(); G__PACK_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PACK_ASK(v: Py) { ensure_init(); G__PACK_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PACK_WORD() -> Py { ensure_init(); G__PACK_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PACK_WORD(v: Py) { ensure_init(); G__PACK_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE_COUNT() -> Py { ensure_init(); G__PEOPLE_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_COUNT(v: Py) { ensure_init(); G__PEOPLE_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE_VALUE() -> Py { ensure_init(); G__PEOPLE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_VALUE(v: Py) { ensure_init(); G__PEOPLE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE_WORD() -> Py { ensure_init(); G__PEOPLE_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_WORD(v: Py) { ensure_init(); G__PEOPLE_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__PEOPLE_WORD_TO_TWENTY() -> Py { ensure_init(); G__PEOPLE_WORD_TO_TWENTY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PEOPLE_WORD_TO_TWENTY(v: Py) { ensure_init(); G__PEOPLE_WORD_TO_TWENTY.with(|c| *c.borrow_mut() = v); }
pub fn g__PER_CUP() -> Py { ensure_init(); G__PER_CUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PER_CUP(v: Py) { ensure_init(); G__PER_CUP.with(|c| *c.borrow_mut() = v); }
pub fn g__PLURAL_BIRDS() -> Py { ensure_init(); G__PLURAL_BIRDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLURAL_BIRDS(v: Py) { ensure_init(); G__PLURAL_BIRDS.with(|c| *c.borrow_mut() = v); }
pub fn g__PORTIONS_ASK() -> Py { ensure_init(); G__PORTIONS_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PORTIONS_ASK(v: Py) { ensure_init(); G__PORTIONS_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__PORTION_AVOID() -> Py { ensure_init(); G__PORTION_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PORTION_AVOID(v: Py) { ensure_init(); G__PORTION_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__PORTION_COUNT() -> Py { ensure_init(); G__PORTION_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PORTION_COUNT(v: Py) { ensure_init(); G__PORTION_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__PORTION_WORD() -> Py { ensure_init(); G__PORTION_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PORTION_WORD(v: Py) { ensure_init(); G__PORTION_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__POUNDS_IS_MONEY() -> Py { ensure_init(); G__POUNDS_IS_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__POUNDS_IS_MONEY(v: Py) { ensure_init(); G__POUNDS_IS_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_COUNT() -> Py { ensure_init(); G__SAID_COUNT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_COUNT(v: Py) { ensure_init(); G__SAID_COUNT.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_PEOPLE() -> Py { ensure_init(); G__SAID_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_PEOPLE(v: Py) { ensure_init(); G__SAID_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_WHOLE() -> Py { ensure_init(); G__SAID_WHOLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_WHOLE(v: Py) { ensure_init(); G__SAID_WHOLE.with(|c| *c.borrow_mut() = v); }
pub fn g__SAYS_OWN_HEIGHT() -> Py { ensure_init(); G__SAYS_OWN_HEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAYS_OWN_HEIGHT(v: Py) { ensure_init(); G__SAYS_OWN_HEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__SAY_K() -> Py { ensure_init(); G__SAY_K.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAY_K(v: Py) { ensure_init(); G__SAY_K.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE() -> Py { ensure_init(); G__SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE(v: Py) { ensure_init(); G__SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_ASK() -> Py { ensure_init(); G__SHARE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_ASK(v: Py) { ensure_init(); G__SHARE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_FRACTION() -> Py { ensure_init(); G__SHARE_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_FRACTION(v: Py) { ensure_init(); G__SHARE_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_LEAD() -> Py { ensure_init(); G__SHARE_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_LEAD(v: Py) { ensure_init(); G__SHARE_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_OF_CUP() -> Py { ensure_init(); G__SHARE_OF_CUP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_OF_CUP(v: Py) { ensure_init(); G__SHARE_OF_CUP.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_SAID() -> Py { ensure_init(); G__SHARE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_SAID(v: Py) { ensure_init(); G__SHARE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_THAT() -> Py { ensure_init(); G__SHARE_THAT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_THAT(v: Py) { ensure_init(); G__SHARE_THAT.with(|c| *c.borrow_mut() = v); }
pub fn g__SHARE_VALUE() -> Py { ensure_init(); G__SHARE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHARE_VALUE(v: Py) { ensure_init(); G__SHARE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g__SLASH() -> Py { ensure_init(); G__SLASH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SLASH(v: Py) { ensure_init(); G__SLASH.with(|c| *c.borrow_mut() = v); }
pub fn g__SPECIFIC() -> Py { ensure_init(); G__SPECIFIC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPECIFIC(v: Py) { ensure_init(); G__SPECIFIC.with(|c| *c.borrow_mut() = v); }
pub fn g__SPLIT() -> Py { ensure_init(); G__SPLIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SPLIT(v: Py) { ensure_init(); G__SPLIT.with(|c| *c.borrow_mut() = v); }
pub fn g__TABLE_GROUPS() -> Py { ensure_init(); G__TABLE_GROUPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TABLE_GROUPS(v: Py) { ensure_init(); G__TABLE_GROUPS.with(|c| *c.borrow_mut() = v); }
pub fn g__TARGET() -> Py { ensure_init(); G__TARGET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TARGET(v: Py) { ensure_init(); G__TARGET.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_ARTICLE() -> Py { ensure_init(); G__TIME_ARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_ARTICLE(v: Py) { ensure_init(); G__TIME_ARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_PLURAL() -> Py { ensure_init(); G__TIME_PLURAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_PLURAL(v: Py) { ensure_init(); G__TIME_PLURAL.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_SECONDS() -> Py { ensure_init(); G__TIME_SECONDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_SECONDS(v: Py) { ensure_init(); G__TIME_SECONDS.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_SMALLER() -> Py { ensure_init(); G__TIME_SMALLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_SMALLER(v: Py) { ensure_init(); G__TIME_SMALLER.with(|c| *c.borrow_mut() = v); }
pub fn g__TIME_UNIT() -> Py { ensure_init(); G__TIME_UNIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIME_UNIT(v: Py) { ensure_init(); G__TIME_UNIT.with(|c| *c.borrow_mut() = v); }
pub fn g__TIN_FOR() -> Py { ensure_init(); G__TIN_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TIN_FOR(v: Py) { ensure_init(); G__TIN_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g__TRAILING_JOIN() -> Py { ensure_init(); G__TRAILING_JOIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRAILING_JOIN(v: Py) { ensure_init(); G__TRAILING_JOIN.with(|c| *c.borrow_mut() = v); }
pub fn g__UNCOUNTED() -> Py { ensure_init(); G__UNCOUNTED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNCOUNTED(v: Py) { ensure_init(); G__UNCOUNTED.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_AND_A_HALF() -> Py { ensure_init(); G__UNIT_AND_A_HALF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_AND_A_HALF(v: Py) { ensure_init(); G__UNIT_AND_A_HALF.with(|c| *c.borrow_mut() = v); }
pub fn g__UNIT_WORD() -> Py { ensure_init(); G__UNIT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__UNIT_WORD(v: Py) { ensure_init(); G__UNIT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__VESSEL_ML() -> Py { ensure_init(); G__VESSEL_ML.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VESSEL_ML(v: Py) { ensure_init(); G__VESSEL_ML.with(|c| *c.borrow_mut() = v); }
pub fn g__WEIGH() -> Py { ensure_init(); G__WEIGH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEIGH(v: Py) { ensure_init(); G__WEIGH.with(|c| *c.borrow_mut() = v); }
pub fn g__WEIGHT_SAID() -> Py { ensure_init(); G__WEIGHT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WEIGHT_SAID(v: Py) { ensure_init(); G__WEIGHT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__WHOLE_PIECE() -> Py { ensure_init(); G__WHOLE_PIECE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHOLE_PIECE(v: Py) { ensure_init(); G__WHOLE_PIECE.with(|c| *c.borrow_mut() = v); }
pub fn g__WHOLE_THING() -> Py { ensure_init(); G__WHOLE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHOLE_THING(v: Py) { ensure_init(); G__WHOLE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g__WHOLE_WEIGHT() -> Py { ensure_init(); G__WHOLE_WEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WHOLE_WEIGHT(v: Py) { ensure_init(); G__WHOLE_WEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_FRACTION() -> Py { ensure_init(); G__WORD_FRACTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_FRACTION(v: Py) { ensure_init(); G__WORD_FRACTION.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD_SHARE() -> Py { ensure_init(); G__WORD_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD_SHARE(v: Py) { ensure_init(); G__WORD_SHARE.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The one table of units: every measure people ask about, converted by meaning.\n\nThis is shared knowledge, not a domain: it declares no ENTRIES of its own and\nbot/knowledge/__init__.py leaves it out of the domain registry. bot/knowledge/maths.py's\nsolve() asks it first, so \"how many grams in a pound\", \"20 miles in km\", \"12 stone in kg\",\n\"how many cups in a litre\" and \"180 c to f\" are all answered by the same table instead of\none regex per question, and every new unit helps every question about it.\n\nA unit is a name, the words people type for it, and how much of the family's base unit it is\n(the metre, the gram, the millilitre, the metre per second). A line is answered only when it\nnames a measure and the unit wanted, in the same family; anything else returns None, so an\nordinary line with a number in it (\"I bought 2 pounds of mince\") is left to the other\nmodules. Answers are rounded the way a person says them: \"20 miles is about 32.2 km\", never\n\"32.1869\".\n");
    s_UNITS(py_dict(vec![(py_str("length"), py_list(vec![py_tuple(vec![py_str("mm"), py_tuple(vec![py_str("mm"), py_str("millimetre"), py_str("millimetres"), py_str("millimeter"), py_str("millimeters")]), Py::Float(0.001)]), py_tuple(vec![py_str("cm"), py_tuple(vec![py_str("cm"), py_str("centimetre"), py_str("centimetres"), py_str("centimeter"), py_str("centimeters")]), Py::Float(0.01)]), py_tuple(vec![py_str("m"), py_tuple(vec![py_str("m"), py_str("metre"), py_str("metres"), py_str("meter"), py_str("meters")]), Py::Float(1.0)]), py_tuple(vec![py_str("km"), py_tuple(vec![py_str("km"), py_str("kilometre"), py_str("kilometres"), py_str("kilometer"), py_str("kilometers")]), Py::Float(1000.0)]), py_tuple(vec![py_str("inch"), py_tuple(vec![py_str("in"), py_str("inch"), py_str("inches")]), Py::Float(0.0254)]), py_tuple(vec![py_str("foot"), py_tuple(vec![py_str("ft"), py_str("foot"), py_str("feet")]), Py::Float(0.3048)]), py_tuple(vec![py_str("yard"), py_tuple(vec![py_str("yd"), py_str("yard"), py_str("yards")]), Py::Float(0.9144)]), py_tuple(vec![py_str("mile"), py_tuple(vec![py_str("mi"), py_str("mile"), py_str("miles")]), Py::Float(1609.344)])])), (py_str("mass"), py_list(vec![py_tuple(vec![py_str("mg"), py_tuple(vec![py_str("mg"), py_str("milligram"), py_str("milligrams")]), Py::Float(0.001)]), py_tuple(vec![py_str("g"), py_tuple(vec![py_str("g"), py_str("gram"), py_str("grams"), py_str("gramme"), py_str("grammes")]), Py::Float(1.0)]), py_tuple(vec![py_str("kg"), py_tuple(vec![py_str("kg"), py_str("kilogram"), py_str("kilograms"), py_str("kilo"), py_str("kilos")]), Py::Float(1000.0)]), py_tuple(vec![py_str("ounce"), py_tuple(vec![py_str("oz"), py_str("ounce"), py_str("ounces")]), Py::Float(28.349523125)]), py_tuple(vec![py_str("pound"), py_tuple(vec![py_str("lb"), py_str("lbs"), py_str("pound"), py_str("pounds")]), Py::Float(453.59237)]), py_tuple(vec![py_str("stone"), py_tuple(vec![py_str("st"), py_str("stone"), py_str("stones")]), Py::Float(6350.29318)])])), (py_str("volume"), py_list(vec![py_tuple(vec![py_str("ml"), py_tuple(vec![py_str("ml"), py_str("millilitre"), py_str("millilitres"), py_str("milliliter"), py_str("milliliters")]), Py::Float(1.0)]), py_tuple(vec![py_str("l"), py_tuple(vec![py_str("l"), py_str("litre"), py_str("litres"), py_str("liter"), py_str("liters")]), Py::Float(1000.0)]), py_tuple(vec![py_str("tsp"), py_tuple(vec![py_str("tsp"), py_str("teaspoon"), py_str("teaspoons")]), Py::Float(5.0)]), py_tuple(vec![py_str("tbsp"), py_tuple(vec![py_str("tbsp"), py_str("tablespoon"), py_str("tablespoons")]), Py::Float(15.0)]), py_tuple(vec![py_str("dessertspoon"), py_tuple(vec![py_str("dsp"), py_str("dessertspoon"), py_str("dessertspoons")]), Py::Float(10.0)]), py_tuple(vec![py_str("cup"), py_tuple(vec![py_str("cup"), py_str("cups")]), Py::Float(240.0)]), py_tuple(vec![py_str("mug"), py_tuple(vec![py_str("mug"), py_str("mugs")]), Py::Float(250.0)]), py_tuple(vec![py_str("fl oz"), py_tuple(vec![py_str("fl oz"), py_str("floz"), py_str("fluid ounce"), py_str("fluid ounces")]), Py::Float(28.4131)]), py_tuple(vec![py_str("pint"), py_tuple(vec![py_str("pt"), py_str("pint"), py_str("pints")]), Py::Float(568.261)]), py_tuple(vec![py_str("gallon"), py_tuple(vec![py_str("gal"), py_str("gallon"), py_str("gallons")]), Py::Float(4546.09)])])), (py_str("speed"), py_list(vec![py_tuple(vec![py_str("km/h"), py_tuple(vec![py_str("kmh"), py_str("km/h"), py_str("kph"), py_str("kilometres an hour"), py_str("kilometers an hour"), py_str("kilometres per hour"), py_str("kilometers per hour")]), Py::Float(1.0)]), py_tuple(vec![py_str("mph"), py_tuple(vec![py_str("mph"), py_str("miles an hour"), py_str("miles per hour")]), Py::Float(1.609344)])])), (py_str("time"), py_list(vec![py_tuple(vec![py_str("second"), py_tuple(vec![py_str("s"), py_str("sec"), py_str("secs"), py_str("second"), py_str("seconds")]), Py::Float(1.0)]), py_tuple(vec![py_str("minute"), py_tuple(vec![py_str("min"), py_str("mins"), py_str("minute"), py_str("minutes")]), Py::Float(60.0)]), py_tuple(vec![py_str("hour"), py_tuple(vec![py_str("hr"), py_str("hrs"), py_str("hour"), py_str("hours")]), Py::Float(3600.0)]), py_tuple(vec![py_str("day"), py_tuple(vec![py_str("day"), py_str("days")]), Py::Float(86400.0)]), py_tuple(vec![py_str("week"), py_tuple(vec![py_str("week"), py_str("weeks")]), Py::Float(604800.0)]), py_tuple(vec![py_str("fortnight"), py_tuple(vec![py_str("fortnight"), py_str("fortnights")]), Py::Float(1209600.0)])])), (py_str("area"), py_list(vec![py_tuple(vec![py_str("sq m"), py_tuple(vec![py_str("sq m"), py_str("square metre"), py_str("square metres"), py_str("square meter"), py_str("square meters"), py_str("m2"), py_str("sqm")]), Py::Float(1.0)]), py_tuple(vec![py_str("hectare"), py_tuple(vec![py_str("hectare"), py_str("hectares")]), Py::Float(10000.0)]), py_tuple(vec![py_str("sq km"), py_tuple(vec![py_str("sq km"), py_str("square kilometre"), py_str("square kilometres"), py_str("square kilometer"), py_str("square kilometers"), py_str("km2"), py_str("sqkm")]), Py::Float(1000000.0)]), py_tuple(vec![py_str("sq ft"), py_tuple(vec![py_str("sq ft"), py_str("square foot"), py_str("square feet"), py_str("sqft"), py_str("ft2")]), Py::Float(0.09290304)]), py_tuple(vec![py_str("sq yd"), py_tuple(vec![py_str("sq yd"), py_str("square yard"), py_str("square yards"), py_str("sqyd"), py_str("yd2")]), Py::Float(0.83612736)]), py_tuple(vec![py_str("acre"), py_tuple(vec![py_str("acre"), py_str("acres")]), Py::Float(4046.8564224)]), py_tuple(vec![py_str("sq mile"), py_tuple(vec![py_str("sq mile"), py_str("square mile"), py_str("square miles"), py_str("sqmi"), py_str("mi2")]), Py::Float(2589988.110336)])]))]));
    fn __data1() -> Py { py_dict(vec![(py_str("mm"), py_str("metric")), (py_str("cm"), py_str("metric")), (py_str("m"), py_str("metric")), (py_str("km"), py_str("metric")), (py_str("mg"), py_str("metric")), (py_str("second"), py_str("metric")), (py_str("minute"), py_str("metric")), (py_str("hour"), py_str("metric")), (py_str("day"), py_str("metric")), (py_str("week"), py_str("metric")), (py_str("fortnight"), py_str("metric")), (py_str("g"), py_str("metric")), (py_str("kg"), py_str("metric")), (py_str("ml"), py_str("metric")), (py_str("l"), py_str("metric")), (py_str("tsp"), py_str("metric")), (py_str("tbsp"), py_str("metric")), (py_str("km/h"), py_str("metric")), (py_str("inch"), py_str("imperial")), (py_str("foot"), py_str("imperial")), (py_str("yard"), py_str("imperial")), (py_str("mile"), py_str("imperial")), (py_str("ounce"), py_str("imperial")), (py_str("pound"), py_str("imperial")), (py_str("stone"), py_str("imperial")), (py_str("mph"), py_str("imperial")), (py_str("pint"), py_str("imperial")), (py_str("gallon"), py_str("imperial")), (py_str("fl oz"), py_str("imperial")), (py_str("cup"), py_str("us")), (py_str("sq m"), py_str("metric")), (py_str("hectare"), py_str("metric")), (py_str("sq km"), py_str("metric")), (py_str("sq ft"), py_str("imperial")), (py_str("sq yd"), py_str("imperial")), (py_str("acre"), py_str("imperial")), (py_str("sq mile"), py_str("imperial"))]) }
    s_SYSTEM(__data1());
    fn __data2() -> Py { py_dict(vec![(py_str("second"), py_str("seconds")), (py_str("minute"), py_str("minutes")), (py_str("hour"), py_str("hours")), (py_str("day"), py_str("days")), (py_str("week"), py_str("weeks")), (py_str("fortnight"), py_str("fortnights")), (py_str("mm"), py_str("mm")), (py_str("cm"), py_str("cm")), (py_str("m"), py_str("metres")), (py_str("km"), py_str("km")), (py_str("inch"), py_str("inches")), (py_str("foot"), py_str("feet")), (py_str("yard"), py_str("yards")), (py_str("mile"), py_str("miles")), (py_str("mg"), py_str("mg")), (py_str("g"), py_str("grams")), (py_str("kg"), py_str("kg")), (py_str("ounce"), py_str("ounces")), (py_str("pound"), py_str("pounds")), (py_str("stone"), py_str("stone")), (py_str("ml"), py_str("ml")), (py_str("l"), py_str("litres")), (py_str("tsp"), py_str("teaspoons")), (py_str("tbsp"), py_str("tablespoons")), (py_str("dessertspoon"), py_str("dessertspoons")), (py_str("cup"), py_str("cups")), (py_str("pint"), py_str("pints")), (py_str("gallon"), py_str("gallons")), (py_str("km/h"), py_str("km/h")), (py_str("mph"), py_str("mph")), (py_str("fl oz"), py_str("fl oz")), (py_str("sq m"), py_str("square metres")), (py_str("hectare"), py_str("hectares")), (py_str("sq km"), py_str("square kilometres")), (py_str("sq ft"), py_str("square feet")), (py_str("sq yd"), py_str("square yards")), (py_str("acre"), py_str("acres")), (py_str("sq mile"), py_str("square miles"))]) }
    s_SAID(__data2());
    fn __data3() -> Py { py_dict(vec![(py_str("second"), py_str("second")), (py_str("minute"), py_str("minute")), (py_str("hour"), py_str("hour")), (py_str("day"), py_str("day")), (py_str("week"), py_str("week")), (py_str("fortnight"), py_str("fortnight")), (py_str("inch"), py_str("inch")), (py_str("foot"), py_str("foot")), (py_str("yard"), py_str("yard")), (py_str("mile"), py_str("mile")), (py_str("ounce"), py_str("ounce")), (py_str("pound"), py_str("pound")), (py_str("stone"), py_str("stone")), (py_str("cup"), py_str("cup")), (py_str("pint"), py_str("pint")), (py_str("gallon"), py_str("gallon")), (py_str("tsp"), py_str("teaspoon")), (py_str("tbsp"), py_str("tablespoon")), (py_str("dessertspoon"), py_str("dessertspoon")), (py_str("m"), py_str("metre")), (py_str("fl oz"), py_str("fluid ounce")), (py_str("l"), py_str("litre")), (py_str("g"), py_str("gram")), (py_str("km"), py_str("km")), (py_str("kg"), py_str("kilogram")), (py_str("ml"), py_str("ml")), (py_str("mm"), py_str("mm")), (py_str("mg"), py_str("mg")), (py_str("cm"), py_str("cm")), (py_str("km/h"), py_str("km/h")), (py_str("mph"), py_str("mph")), (py_str("sq m"), py_str("square metre")), (py_str("hectare"), py_str("hectare")), (py_str("sq km"), py_str("square kilometre")), (py_str("sq ft"), py_str("square foot")), (py_str("sq yd"), py_str("square yard")), (py_str("acre"), py_str("acre")), (py_str("sq mile"), py_str("square mile"))]) }
    s_SAID_ONE(__data3());
    fn __data4() -> Py { py_tuple(vec![py_tuple(vec![py_str("icing sugar"), Py::Int(120)]), py_tuple(vec![py_str("brown sugar"), Py::Int(220)]), py_tuple(vec![py_str("sugar"), Py::Int(200)]), py_tuple(vec![py_str("flour"), Py::Int(120)]), py_tuple(vec![py_str("cornflour"), Py::Int(120)]), py_tuple(vec![py_str("oats"), Py::Int(90)]), py_tuple(vec![py_str("butter"), Py::Int(227)]), py_tuple(vec![py_str("margarine"), Py::Int(227)]), py_tuple(vec![py_str("rice"), Py::Int(185)]), py_tuple(vec![py_str("couscous"), Py::Int(175)]), py_tuple(vec![py_str("lentils"), Py::Int(190)]), py_tuple(vec![py_str("breadcrumbs"), Py::Int(110)]), py_tuple(vec![py_str("milk"), Py::Int(240)]), py_tuple(vec![py_str("water"), Py::Int(240)]), py_tuple(vec![py_str("cream"), Py::Int(240)]), py_tuple(vec![py_str("yogurt"), Py::Int(245)]), py_tuple(vec![py_str("yoghurt"), Py::Int(245)]), py_tuple(vec![py_str("honey"), Py::Int(340)]), py_tuple(vec![py_str("oil"), Py::Int(220)]), py_tuple(vec![py_str("jam"), Py::Int(320)]), py_tuple(vec![py_str("peanut butter"), Py::Int(260)]), py_tuple(vec![py_str("grated cheese"), Py::Int(100)]), py_tuple(vec![py_str("cheese"), Py::Int(100)]), py_tuple(vec![py_str("chocolate chips"), Py::Int(170)]), py_tuple(vec![py_str("chocolate"), Py::Int(170)]), py_tuple(vec![py_str("cocoa"), Py::Int(85)]), py_tuple(vec![py_str("nuts"), Py::Int(140)]), py_tuple(vec![py_str("almonds"), Py::Int(140)]), py_tuple(vec![py_str("cashews"), Py::Int(140)]), py_tuple(vec![py_str("peanuts"), Py::Int(145)]), py_tuple(vec![py_str("walnuts"), Py::Int(100)]), py_tuple(vec![py_str("raisins"), Py::Int(150)]), py_tuple(vec![py_str("pumpkin seeds"), Py::Int(130)]), py_tuple(vec![py_str("salt"), Py::Int(288)]), py_tuple(vec![py_str("baking powder"), Py::Int(190)])]) }
    s_GRAMS_PER_CUP(__data4());
    fn __data5() -> Py { py_tuple(vec![py_tuple(vec![py_str("baking powder"), Py::Int(4)]), py_tuple(vec![py_str("bicarbonate"), Py::Int(4)]), py_tuple(vec![py_str("bicarb"), Py::Int(4)]), py_tuple(vec![py_str("baking soda"), Py::Int(4)]), py_tuple(vec![py_str("yeast"), Py::Int(3)]), py_tuple(vec![py_str("salt"), Py::Int(6)]), py_tuple(vec![py_str("cinnamon"), Py::Float(2.6)]), py_tuple(vec![py_str("paprika"), Py::Float(2.4)]), py_tuple(vec![py_str("cumin"), Py::Float(2.1)]), py_tuple(vec![py_str("turmeric"), Py::Float(2.4)]), py_tuple(vec![py_str("cayenne"), Py::Float(1.8)]), py_tuple(vec![py_str("spice"), Py::Float(2.5)]), py_tuple(vec![py_str("pepper"), Py::Float(2.3)]), py_tuple(vec![py_str("sugar"), Py::Float(4.2)]), py_tuple(vec![py_str("cocoa"), Py::Float(2.5)]), py_tuple(vec![py_str("flour"), Py::Float(2.6)]), py_tuple(vec![py_str("cornflour"), Py::Float(2.5)]), py_tuple(vec![py_str("butter"), Py::Float(4.7)]), py_tuple(vec![py_str("oil"), Py::Float(4.5)]), py_tuple(vec![py_str("honey"), Py::Int(7)]), py_tuple(vec![py_str("milk"), Py::Int(5)]), py_tuple(vec![py_str("water"), Py::Int(5)]), py_tuple(vec![py_str("oats"), Py::Int(3)])]) }
    s_GRAMS_PER_TSP(__data5());
    fn __data6() -> Py { py_tuple(vec![py_tuple(vec![py_str("clove of garlic"), Py::Int(5)]), py_tuple(vec![py_str("garlic clove"), Py::Int(5)]), py_tuple(vec![py_str("bulb of garlic"), Py::Int(45)]), py_tuple(vec![py_str("garlic bulb"), Py::Int(45)]), py_tuple(vec![py_str("slice of bread"), Py::Int(36)]), py_tuple(vec![py_str("slice of ham"), Py::Int(20)]), py_tuple(vec![py_str("slice of cheese"), Py::Int(20)]), py_tuple(vec![py_str("slice of bacon"), Py::Int(25)]), py_tuple(vec![py_str("rasher"), Py::Int(25)]), py_tuple(vec![py_str("slice of cake"), Py::Int(60)]), py_tuple(vec![py_str("slice of pizza"), Py::Int(100)]), py_tuple(vec![py_str("slice of toast"), Py::Int(30)]), py_tuple(vec![py_str("bread roll"), Py::Int(60)]), py_tuple(vec![py_str("crumpet"), Py::Int(60)]), py_tuple(vec![py_str("pancake"), Py::Int(40)]), py_tuple(vec![py_str("chicken breast"), Py::Int(170)]), py_tuple(vec![py_str("chicken thigh"), Py::Int(100)]), py_tuple(vec![py_str("drumstick"), Py::Int(100)]), py_tuple(vec![py_str("egg"), Py::Int(55)]), py_tuple(vec![py_str("banana"), Py::Int(120)]), py_tuple(vec![py_str("apple"), Py::Int(150)]), py_tuple(vec![py_str("orange"), Py::Int(140)]), py_tuple(vec![py_str("satsuma"), Py::Int(70)]), py_tuple(vec![py_str("pear"), Py::Int(170)]), py_tuple(vec![py_str("peach"), Py::Int(150)]), py_tuple(vec![py_str("plum"), Py::Int(70)]), py_tuple(vec![py_str("lemon"), Py::Int(100)]), py_tuple(vec![py_str("lime"), Py::Int(70)]), py_tuple(vec![py_str("avocado"), Py::Int(150)]), py_tuple(vec![py_str("potato"), Py::Int(170)]), py_tuple(vec![py_str("onion"), Py::Int(150)]), py_tuple(vec![py_str("carrot"), Py::Int(80)]), py_tuple(vec![py_str("tomato"), Py::Int(120)]), py_tuple(vec![py_str("courgette"), Py::Int(200)]), py_tuple(vec![py_str("pepper"), Py::Int(160)]), py_tuple(vec![py_str("mushroom"), Py::Int(20)]), py_tuple(vec![py_str("aubergine"), Py::Int(250)]), py_tuple(vec![py_str("cauliflower"), Py::Int(700)]), py_tuple(vec![py_str("broccoli"), Py::Int(400)]), py_tuple(vec![py_str("swede"), Py::Int(900)]), py_tuple(vec![py_str("cucumber"), Py::Int(300)]), py_tuple(vec![py_str("butternut squash"), Py::Int(1000)]), py_tuple(vec![py_str("sweet potato"), Py::Int(200)]), py_tuple(vec![py_str("cabbage"), Py::Int(900)]), py_tuple(vec![py_str("leek"), Py::Int(200)]), py_tuple(vec![py_str("parsnip"), Py::Int(150)]), py_tuple(vec![py_str("beetroot"), Py::Int(150)]), py_tuple(vec![py_str("turnip"), Py::Int(250)]), py_tuple(vec![py_str("head of celery"), Py::Int(400)]), py_tuple(vec![py_str("head of broccoli"), Py::Int(400)]), py_tuple(vec![py_str("head of cauliflower"), Py::Int(700)]), py_tuple(vec![py_str("bunch of grapes"), Py::Int(400)]), py_tuple(vec![py_str("punnet of strawberries"), Py::Int(300)]), py_tuple(vec![py_str("bunch of spring onions"), Py::Int(100)]), py_tuple(vec![py_str("bunch of spring onion"), Py::Int(100)]), py_tuple(vec![py_str("bunch of parsley"), Py::Int(30)]), py_tuple(vec![py_str("bunch of coriander"), Py::Int(30)]), py_tuple(vec![py_str("bunch of mint"), Py::Int(30)]), py_tuple(vec![py_str("bunch of basil"), Py::Int(30)]), py_tuple(vec![py_str("bunch of dill"), Py::Int(20)]), py_tuple(vec![py_str("bunch of thyme"), Py::Int(20)]), py_tuple(vec![py_str("bunch of rosemary"), Py::Int(20)]), py_tuple(vec![py_str("bunch of chives"), Py::Int(20)]), py_tuple(vec![py_str("bunch of sage"), Py::Int(20)]), py_tuple(vec![py_str("punnet of cherry tomatoes"), Py::Int(250)]), py_tuple(vec![py_str("punnet of raspberries"), Py::Int(300)]), py_tuple(vec![py_str("lettuce"), Py::Int(300)]), py_tuple(vec![py_str("head of lettuce"), Py::Int(300)]), py_tuple(vec![py_str("iceberg lettuce"), Py::Int(600)]), py_tuple(vec![py_str("round lettuce"), Py::Int(200)]), py_tuple(vec![py_str("punnet of blueberries"), Py::Int(200)]), py_tuple(vec![py_str("fennel bulb"), Py::Int(250)]), py_tuple(vec![py_str("red cabbage"), Py::Int(900)]), py_tuple(vec![py_str("sausage"), Py::Int(60)]), py_tuple(vec![py_str("strawberry"), Py::Int(12)]), py_tuple(vec![py_str("sprout"), Py::Int(10)]), py_tuple(vec![py_str("raspberry"), Py::Int(4)]), py_tuple(vec![py_str("biscuit"), Py::Int(10)]), py_tuple(vec![py_str("square of chocolate"), Py::Int(10)])]) }
    s_PIECE_GRAMS(__data6());
    s__WHOLE_PIECE(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PIECE_GRAMS()) { let __unpk1385 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk1385[0].clone(); let mut _grams: Py = __unpk1385[1].clone(); if py_truthy(&Py::Bool(py_contains(&name.clone(), &py_str(" of ")))) {  __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__DISH_AFTER(stdlib_call("re", "compile", &[py_str("\\s+(?:pie|pies|bread|soup|soups|sandwich|sandwiches|curry|curries|salad|salads|sauce|sauces|cake|cakes|tart|tarts|crumble|smoothie|smoothies|juice|juices|jam|chutney|toast|stew|stews|casserole|risotto|pizza|pizzas|burger|burgers|wrap|wraps|muffin|muffins|pancake|pancakes|batter|filling|topping|chips|mash|puree|dip|dressing|mayo|stock|broth|crisps|pickle|relish|ketchup|crust|pastry|dough|flour|powder|extract|oil|wine|cordial|squash|tea|coffee|milkshake|yoghurt|yogurt)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MEASURE_AFTER_PIECE(stdlib_call("re", "compile", &[py_str("\\b(?:in|into)\\s+(?:cups?|ml|mls|millilitres?|milliliters?|litres?|liters?|pints?|fl\\s*oz|fluid ounces?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__WHOLE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:heads?|bunch(?:es)?|punnets?)\\s+of\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s__UNCOUNTED(stdlib_call("re", "compile", &[py_str("\\b(?:tin|tins|can|cans|jar|jars|packet|packets|pack|packs|box|boxes|bag|bags|punnet|punnets|bunch|bunches|tub|tubs|carton|cartons|loaf|loaves|head|heads|handful|handfuls|scoop|scoops|bottle|bottles)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__COUNT(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(three[\\s-]quarters?|two[\\s-]thirds?|half|a\\s+half|quarter|a\\s+quarter|a\\s+third|(?:one|two|three|four)\\s+and\\s+a\\s+half|\\d+(?:\\.\\d+)?|a|an|one|two|three|four|five|six|seven|eight|nine|ten)(?:\\s+of)?(?:\\s+an?)?\\s+(?:(?:small|large|medium|big|whole|ripe|fresh|cooked|raw)\\s+)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_AS_KITCHEN_SAYS(py_dict(vec![(py_str("cheese"), py_str("grated cheese")), (py_str("chocolate"), py_str("grated chocolate")), (py_str("cocoa"), py_str("cocoa powder"))]));
    s__COMPONENT(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\s+(?:of\\s+)?(?:grams?\\s+of\\s+)?(?:sugar|salt|fat|protein|fibre|fiber|carbs?|carbohydrates?|calories|kcal|alcohol|caffeine|sodium|calcium|iron|vitamins?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__WEIGH(stdlib_call("re", "compile", &[py_str("\\b(?:weigh|weighs|weight|heavy|heavier)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__COMPONENT_MEASURED(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\s+(?:of\\s+)?(?:grams?\\s+of\\s+)?(?P<component>sugar|salt|fat|protein|fibre|fiber|carbs?|carbohydrates?|calories|kcal|alcohol|caffeine|sodium|calcium|iron|vitamins?)(?:\\s+(?:is|are)(?:\\s+there)?)?\\s+(?:in|per)\\s+(?:a|one|1|an)\\s+(?:level\\s+|heaped\\s+)?(?P<measure>teaspoons?|tablespoons?|tbsp|tsp|dessertspoons?|spoons?|spoonfuls?|cups?)(?:\\s+of\\s+(?P<again>[a-z]+))?\\s*[?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__DRY_IN_A_MEASURE(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many)\\s+(?:of\\s+)?(?:grams?\\s+of\\s+)?(?P<food>icing sugar|brown sugar|cocoa powder|cocoa|cornflour|breadcrumbs|peanut butter|flour|jam|cream|yoghurt|yogurt|oats|rice|cheese|chocolate)(?:\\s+(?:is|are)(?:\\s+there)?)?\\s+(?:in|per)\\s+(?:a|one|1|an)\\s+(?:level\\s+|heaped\\s+)?(?P<measure>teaspoons?|tablespoons?|tbsp|tsp|dessertspoons?|spoons?|spoonfuls?|cups?)(?:\\s+of\\s+(?P<again>[a-z]+))?\\s*[?.!]*$")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data7() -> Py { py_tuple(vec![py_tuple(vec![py_str("suitcase"), Py::Int(4000)]), py_tuple(vec![py_str("cabin bag"), Py::Int(2500)]), py_tuple(vec![py_str("holdall"), Py::Int(1500)]), py_tuple(vec![py_str("backpack"), Py::Int(1200)]), py_tuple(vec![py_str("rucksack"), Py::Int(1200)]), py_tuple(vec![py_str("handbag"), Py::Int(800)]), py_tuple(vec![py_str("briefcase"), Py::Int(1500)]), py_tuple(vec![py_str("microwave"), Py::Int(12000)]), py_tuple(vec![py_str("kettle"), Py::Int(1000)]), py_tuple(vec![py_str("toaster"), Py::Int(1500)]), py_tuple(vec![py_str("air fryer"), Py::Int(5000)]), py_tuple(vec![py_str("slow cooker"), Py::Int(4000)]), py_tuple(vec![py_str("washing machine"), Py::Int(70000)]), py_tuple(vec![py_str("tumble dryer"), Py::Int(40000)]), py_tuple(vec![py_str("dishwasher"), Py::Int(45000)]), py_tuple(vec![py_str("fridge freezer"), Py::Int(80000)]), py_tuple(vec![py_str("fridge"), Py::Int(70000)]), py_tuple(vec![py_str("freezer"), Py::Int(40000)]), py_tuple(vec![py_str("oven"), Py::Int(50000)]), py_tuple(vec![py_str("hob"), Py::Int(10000)]), py_tuple(vec![py_str("vacuum cleaner"), Py::Int(6000)]), py_tuple(vec![py_str("hoover"), Py::Int(6000)]), py_tuple(vec![py_str("iron"), Py::Int(1200)]), py_tuple(vec![py_str("sewing machine"), Py::Int(8000)]), py_tuple(vec![py_str("laptop"), Py::Int(1500)]), py_tuple(vec![py_str("chromebook"), Py::Int(1300)]), py_tuple(vec![py_str("tablet"), Py::Int(500)]), py_tuple(vec![py_str("ipad"), Py::Int(500)]), py_tuple(vec![py_str("phone"), Py::Int(200)]), py_tuple(vec![py_str("iphone"), Py::Int(200)]), py_tuple(vec![py_str("desktop"), Py::Int(8000)]), py_tuple(vec![py_str("printer"), Py::Int(8000)]), py_tuple(vec![py_str("monitor"), Py::Int(5000)]), py_tuple(vec![py_str("television"), Py::Int(15000)]), py_tuple(vec![py_str("games console"), Py::Int(4000)]), py_tuple(vec![py_str("playstation"), Py::Int(4500)]), py_tuple(vec![py_str("xbox"), Py::Int(4500)]), py_tuple(vec![py_str("smart speaker"), Py::Int(700)]), py_tuple(vec![py_str("router"), Py::Int(500)]), py_tuple(vec![py_str("camera"), Py::Int(600)]), py_tuple(vec![py_str("bicycle"), Py::Int(12000)]), py_tuple(vec![py_str("bike"), Py::Int(12000)]), py_tuple(vec![py_str("e-bike"), Py::Int(22000)]), py_tuple(vec![py_str("scooter"), Py::Int(12000)]), py_tuple(vec![py_str("mobility scooter"), Py::Int(40000)]), py_tuple(vec![py_str("pram"), Py::Int(10000)]), py_tuple(vec![py_str("pushchair"), Py::Int(8000)]), py_tuple(vec![py_str("wheelchair"), Py::Int(15000)]), py_tuple(vec![py_str("car seat"), Py::Int(7000)]), py_tuple(vec![py_str("car battery"), Py::Int(18000)]), py_tuple(vec![py_str("tyre"), Py::Int(8000)]), py_tuple(vec![py_str("roof box"), Py::Int(15000)]), py_tuple(vec![py_str("caravan"), Py::Int(1200000)]), py_tuple(vec![py_str("motorhome"), Py::Int(2800000)]), py_tuple(vec![py_str("trailer"), Py::Int(300000)]), py_tuple(vec![py_str("piano"), Py::Int(200000)]), py_tuple(vec![py_str("sofa"), Py::Int(50000)]), py_tuple(vec![py_str("armchair"), Py::Int(25000)]), py_tuple(vec![py_str("mattress"), Py::Int(25000)]), py_tuple(vec![py_str("wardrobe"), Py::Int(80000)]), py_tuple(vec![py_str("desk"), Py::Int(30000)]), py_tuple(vec![py_str("dining table"), Py::Int(40000)]), py_tuple(vec![py_str("bookshelf"), Py::Int(25000)]), py_tuple(vec![py_str("chest of drawers"), Py::Int(30000)]), py_tuple(vec![py_str("bed"), Py::Int(40000)]), py_tuple(vec![py_str("cot"), Py::Int(20000)]), py_tuple(vec![py_str("bunk bed"), Py::Int(50000)]), py_tuple(vec![py_str("coffee table"), Py::Int(20000)]), py_tuple(vec![py_str("laundry basket"), Py::Int(3000)]), py_tuple(vec![py_str("trampoline"), Py::Int(40000)]), py_tuple(vec![py_str("greenhouse"), Py::Int(50000)]), py_tuple(vec![py_str("boiler"), Py::Int(30000)]), py_tuple(vec![py_str("radiator"), Py::Int(10000)]), py_tuple(vec![py_str("water butt"), Py::Int(20000)]), py_tuple(vec![py_str("wheelie bin"), Py::Int(15000)])]) }
    s_OBJECT_GRAMS(__data7());
    s__COUNTED_THING(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(?:\\d+|a|an|one|two|three|four|five|six|my|your|our|his|her|their|the)\\s+$")]).unwrap_or_else(|e| exc::raise(e)));
    s__SHARE_OF_CUP(py_dict(vec![(py_str("cup"), Py::Float(1.0)), (py_str("tbsp"), op_truediv(&Py::Int(1), &Py::Int(16))), (py_str("tsp"), op_truediv(&Py::Int(1), &Py::Int(48)))]));
    s_GAS_OVEN(py_tuple(vec![py_tuple(vec![Py::Int(1), Py::Int(140), Py::Int(275)]), py_tuple(vec![Py::Int(2), Py::Int(150), Py::Int(300)]), py_tuple(vec![Py::Int(3), Py::Int(170), Py::Int(325)]), py_tuple(vec![Py::Int(4), Py::Int(180), Py::Int(350)]), py_tuple(vec![Py::Int(5), Py::Int(190), Py::Int(375)]), py_tuple(vec![Py::Int(6), Py::Int(200), Py::Int(400)]), py_tuple(vec![Py::Int(7), Py::Int(220), Py::Int(425)]), py_tuple(vec![Py::Int(8), Py::Int(230), Py::Int(450)]), py_tuple(vec![Py::Int(9), Py::Int(240), Py::Int(475)])]));
    s_FAN_COOLER(Py::Int(20));
    s__NO_PLURAL(py_tuple(vec![py_str("in"), py_str("pt"), py_str("mi")]));
    let __unpk8 = py_unpack(&_build(&[]), 2, None);
    s__NAMES(__unpk8[0].clone());
    s__UNIT_WORD(__unpk8[1].clone());
    s__NUMBER(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d[\\d,]*(?:\\.\\d+)?|a|an|one)(?![\\w])")]).unwrap_or_else(|e| exc::raise(e)));
    s__TARGET(stdlib_call("re", "compile", &[py_str("(?:to|in|into|as|->|=)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__SAY_K(stdlib_call("re", "compile", &[py_str("(\\d)\\s*k\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__JOINED(stdlib_call("re", "compile", &[py_str("(?<=\\d)(?=\\s*(?:kg|mg|ml|cm|mm|km|oz|lbs|lb|ft|in|st|pt|g|l|m|metres?|meters?)\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__FRACTION(py_dict(vec![(py_tuple(vec![py_str("1"), py_str("4")]), Py::Float(0.25)), (py_tuple(vec![py_str("1"), py_str("2")]), Py::Float(0.5)), (py_tuple(vec![py_str("3"), py_str("4")]), Py::Float(0.75)), (py_tuple(vec![py_str("1"), py_str("3")]), op_truediv(&Py::Int(1), &Py::Int(3))), (py_tuple(vec![py_str("2"), py_str("3")]), op_truediv(&Py::Int(2), &Py::Int(3)))]));
    s__SLASH(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d)\\s*/\\s*(\\d)(?![\\d/])")]).unwrap_or_else(|e| exc::raise(e)));
    s__AND_A_HALF(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d|one|two|three|four|five)\\s+and\\s+a\\s+half\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_WHOLE(py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5))]));
    s__UNIT_AND_A_HALF(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(?:(?P<count>\\d+|an?|one|two|three|four)\\s+)?(?P<unit>cups?|tablespoons?|tbsp|teaspoons?|tsp|ounces?|oz|pints?|quarts?|litres?|liters?|sticks?)\\s+and\\s+a\\s+half\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__MIXED(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d+)\\s+(\\d)\\s*/\\s*(\\d)(?![\\d/])")]).unwrap_or_else(|e| exc::raise(e)));
    s__WORD_FRACTION(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bthree[\\s-]quarters?(?:\\s+of)?(?:\\s+an?)?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("0.75")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\btwo[\\s-]thirds?(?:\\s+of)?(?:\\s+an?)?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("0.67")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:one|an?)[\\s-]third(?:\\s+of)?(?:\\s+an?)?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("0.33")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\bhalf(?:\\s+of)?(?:\\s+an?)?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("0.5")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\ban?\\s+quarter(?:\\s+of)?(?:\\s+an?)?\\b")]).unwrap_or_else(|e| exc::raise(e)), py_str("0.25")])]));
    s__SPLIT(py_dict(vec![(py_str("week"), py_tuple(vec![py_str("day"), Py::Float(7.0)])), (py_str("day"), py_tuple(vec![py_str("hour"), Py::Float(24.0)]))]));
    s__C_ASKS(stdlib_call("re", "compile", &[py_str("\\bcelsius|\\bcentigrade|\\bdegrees? c\\b|°c\\b|\\bc\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__F_ASKS(stdlib_call("re", "compile", &[py_str("\\bfahrenheit|\\bdegrees? f\\b|°f\\b|\\bf\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_TEMPERATURE(stdlib_call("re", "compile", &[py_str("\\s*(?:convert\\s+)?(?:degrees?\\s+)?(?:c|celsius|centigrade|f|fahrenheit)\\s+(?:to|in|into)\\s*(?:degrees?\\s+)?(?:c|celsius|centigrade|f|fahrenheit)\\s*")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data9() -> Py { py_tuple(vec![py_tuple(vec![py_str("chopped tomatoes"), Py::Int(400)]), py_tuple(vec![py_str("plum tomatoes"), Py::Int(400)]), py_tuple(vec![py_str("tomatoes"), Py::Int(400)]), py_tuple(vec![py_str("baked beans"), Py::Int(415)]), py_tuple(vec![py_str("kidney beans"), Py::Int(400)]), py_tuple(vec![py_str("black beans"), Py::Int(400)]), py_tuple(vec![py_str("cannellini beans"), Py::Int(400)]), py_tuple(vec![py_str("beans"), Py::Int(400)]), py_tuple(vec![py_str("chickpeas"), Py::Int(400)]), py_tuple(vec![py_str("chick peas"), Py::Int(400)]), py_tuple(vec![py_str("lentils"), Py::Int(400)]), py_tuple(vec![py_str("sweetcorn"), Py::Int(340)]), py_tuple(vec![py_str("tuna"), Py::Int(145)]), py_tuple(vec![py_str("sardines"), Py::Int(120)]), py_tuple(vec![py_str("coconut milk"), Py::Int(400)]), py_tuple(vec![py_str("passata"), Py::Int(680)]), py_tuple(vec![py_str("condensed milk"), Py::Int(397)]), py_tuple(vec![py_str("butter"), Py::Int(250)]), py_tuple(vec![py_str("cream"), Py::Int(300)]), py_tuple(vec![py_str("honey"), Py::Int(340)]), py_tuple(vec![py_str("jam"), Py::Int(340)]), py_tuple(vec![py_str("marmalade"), Py::Int(340)]), py_tuple(vec![py_str("peanut butter"), Py::Int(340)]), py_tuple(vec![py_str("ice cream"), Py::Int(500)]), py_tuple(vec![py_str("cheese"), Py::Int(400)]), py_tuple(vec![py_str("cheddar"), Py::Int(400)]), py_tuple(vec![py_str("feta"), Py::Int(200)]), py_tuple(vec![py_str("soup"), Py::Int(400)]), py_tuple(vec![py_str("tomato soup"), Py::Int(400)]), py_tuple(vec![py_str("peaches"), Py::Int(410)]), py_tuple(vec![py_str("pears"), Py::Int(410)]), py_tuple(vec![py_str("pineapple"), Py::Int(435)]), py_tuple(vec![py_str("peas"), Py::Int(400)]), py_tuple(vec![py_str("corned beef"), Py::Int(340)]), py_tuple(vec![py_str("yogurt"), Py::Int(500)]), py_tuple(vec![py_str("yoghurt"), Py::Int(500)]), py_tuple(vec![py_str("chicken breasts"), Py::Int(500)]), py_tuple(vec![py_str("chicken breast"), Py::Int(500)]), py_tuple(vec![py_str("mince"), Py::Int(500)]), py_tuple(vec![py_str("bacon"), Py::Int(250)]), py_tuple(vec![py_str("pasta"), Py::Int(500)]), py_tuple(vec![py_str("spaghetti"), Py::Int(500)]), py_tuple(vec![py_str("penne"), Py::Int(500)]), py_tuple(vec![py_str("fusilli"), Py::Int(500)]), py_tuple(vec![py_str("macaroni"), Py::Int(500)]), py_tuple(vec![py_str("sausages"), Py::Int(400)]), py_tuple(vec![py_str("flour"), Py::Int(1500)]), py_tuple(vec![py_str("sugar"), Py::Int(1000)]), py_tuple(vec![py_str("rice"), Py::Int(1000)]), py_tuple(vec![py_str("crisps"), Py::Int(25)]), py_tuple(vec![py_str("popcorn"), Py::Int(70)]), py_tuple(vec![py_str("biscuits"), Py::Int(200)]), py_tuple(vec![py_str("cereal"), Py::Int(375)]), py_tuple(vec![py_str("salad leaves"), Py::Int(200)]), py_tuple(vec![py_str("spinach"), Py::Int(200)]), py_tuple(vec![py_str("salad"), Py::Int(200)]), py_tuple(vec![py_str("carrots"), Py::Int(1000)]), py_tuple(vec![py_str("chocolate"), Py::Int(100)]), py_tuple(vec![py_str("bread"), Py::Int(800)]), py_tuple(vec![py_str("potatoes"), Py::Int(2000)]), py_tuple(vec![py_str("carrots"), Py::Int(1000)]), py_tuple(vec![py_str("onions"), Py::Int(1000)]), py_tuple(vec![py_str("cola"), Py::Int(330)]), py_tuple(vec![py_str("coke"), Py::Int(330)]), py_tuple(vec![py_str("lemonade"), Py::Int(330)]), py_tuple(vec![py_str("toothpaste"), Py::Int(100)]), py_tuple(vec![py_str("shampoo"), Py::Int(300)]), py_tuple(vec![py_str("conditioner"), Py::Int(250)]), py_tuple(vec![py_str("coffee"), Py::Int(200)]), py_tuple(vec![py_str("grapes"), Py::Int(400)]), py_tuple(vec![py_str("olive oil"), Py::Int(500)]), py_tuple(vec![py_str("washing up liquid"), Py::Int(500)]), py_tuple(vec![py_str("ketchup"), Py::Int(460)]), py_tuple(vec![py_str("wine"), Py::Int(750)]), py_tuple(vec![py_str("beer"), Py::Int(330)]), py_tuple(vec![py_str("vinegar"), Py::Int(500)])]) }
    s_PACK_GRAMS(__data9());
    s_PACK_DRAINED(py_dict(vec![(py_str("tuna"), Py::Int(112)), (py_str("sweetcorn"), Py::Int(285)), (py_str("sardines"), Py::Int(90)), (py_str("peas"), Py::Int(285)), (py_str("peaches"), Py::Int(220)), (py_str("pears"), Py::Int(220)), (py_str("pineapple"), Py::Int(250))]));
    fn __data10() -> Py { py_tuple(vec![py_str("coconut milk"), py_str("cream"), py_str("ice cream"), py_str("cola"), py_str("coke"), py_str("lemonade"), py_str("shampoo"), py_str("conditioner"), py_str("olive oil"), py_str("washing up liquid"), py_str("wine"), py_str("beer"), py_str("vinegar")]) }
    s_PACK_IN_ML(__data10());
    s__PACK_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:tin|tins|can|cans|jar|jars|packet|packets|pack|packs|bag|bags|block|blocks|carton|cartons|tub|tubs|box|boxes|bar|bars|sleeve|sleeves|tube|tubes|loaf|loaves|punnet|punnets|bottle|bottles)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PACK_ASK(stdlib_call("re", "compile", &[py_str("\\bgrams?\\b|\\bg\\b|\\bml\\b|\\bmls\\b|weigh|weighs|weight|ounces?|\\boz\\b|how\\s+(?:big|heavy)|what\\s+size|how\\s+many\\s+(?:grams?|ml|ounces?|oz)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__COUNT_ASK(stdlib_call("re", "compile", &[py_str("\\bhow many\\s+(?P<piece>[a-z][a-z' -]{1,28}?)\\s+(?:are\\s+there\\s+|are\\s+|do\\s+you\\s+get\\s+|can\\s+you\\s+get\\s+|would\\s+you\\s+get\\s+|come\\s+|come in|go\\s+)?(?:in|into|per|to)\\s+(?:a|an|one|1)\\s+(?P<box>packet|pack|bag|box|tin|can|jar|punnet|tub|block|bar|loaf|sleeve|tube|carton)\\b(?:\\s+of\\s+(?P<of>[a-z][a-z' ]{1,45}?))?(?=[,.?!]|$)")]).unwrap_or_else(|e| exc::raise(e)));
    s_COUNTED_PACKS(py_dict(vec![(py_str("egg"), py_str("A box is six or twelve - the box says which, and the twelve is a dozen.")), (py_str("teabag"), py_str("A box is usually 80 or 160 tea bags, and a small box of 40 is common too.")), (py_str("tea bag"), py_str("A box is usually 80 or 160 tea bags, and a small box of 40 is common too.")), (py_str("gelatine"), py_str("A packet is usually 12 sheets, and one sheet sets about 100ml of liquid.")), (py_str("stock cube"), py_str("A box is usually 10 or 12 cubes, and one cube makes about 450ml of stock."))]));
    s__COUNT_CONTENT(py_dict(vec![(py_str("slice"), py_tuple(vec![py_str("bread"), py_str("slice of bread")])), (py_str("slices"), py_tuple(vec![py_str("bread"), py_str("slice of bread")])), (py_str("squares"), py_tuple(vec![py_str("chocolate"), py_str("square of chocolate")])), (py_str("square"), py_tuple(vec![py_str("chocolate"), py_str("square of chocolate")]))]));
    s__NOT_A_COUNT(stdlib_call("re", "compile", &[py_str("^(?:grams?|g|mls?|kilos?|kgs?|litres?|liters?|pints?|ounces?|oz|calories?|kcal|carbs?|sugars?|units?|prices?|costs?|servings?|portions?|people|persons?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data11() -> Py { py_tuple(vec![py_tuple(vec![py_str("knob of butter"), py_str("A knob of butter is about 10 g, a scant tablespoon.")]), py_tuple(vec![py_str("squeeze of lemon"), py_str("A squeeze of lemon is about a teaspoon, 5 ml.")]), py_tuple(vec![py_str("squeeze of lime"), py_str("A squeeze of lime is about a teaspoon, 5 ml.")]), py_tuple(vec![py_str("squeeze of honey"), py_str("A squeeze of honey is about a teaspoon, 7 g.")]), py_tuple(vec![py_str("shot of espresso"), py_str("A single shot of espresso is about 25 ml, a double 50 ml.")]), py_tuple(vec![py_str("shot"), py_str("A single shot is 25 ml and a double 50 ml, 25 g and 50 g of a thin liquid.")]), py_tuple(vec![py_str("glass of wine"), py_str("A small glass of wine is 125 ml, a medium 175 ml and a large 250 ml.")]), py_tuple(vec![py_str("glass of milk"), py_str("A glass of milk is about 200 ml, which is roughly 205 g.")]), py_tuple(vec![py_str("glass of water"), py_str("A glass of water is about 200 ml, which is 200 g, and a big one 250 ml.")]), py_tuple(vec![py_str("mug of tea"), py_str("A mug of tea is about 250 ml.")]), py_tuple(vec![py_str("wedge of lemon"), py_str("A wedge of lemon is about 15 g, and gives a teaspoon or two of juice.")]), py_tuple(vec![py_str("wedge of lime"), py_str("A wedge of lime is about 15 g, and gives a teaspoon or two of juice.")]), py_tuple(vec![py_str("pot of yogurt"), py_str("A small pot of yogurt is 150 g and a big one 500 g.")]), py_tuple(vec![py_str("pot of cream"), py_str("A pot of cream is 300 ml, and a small one 150 ml.")]), py_tuple(vec![py_str("glass of orange juice"), py_str("A glass of orange juice is about 200 ml, which takes two or three oranges.")]), py_tuple(vec![py_str("glass of apple juice"), py_str("A glass of apple juice is about 200 ml.")]), py_tuple(vec![py_str("glass of juice"), py_str("A glass of juice is about 200 ml.")]), py_tuple(vec![py_str("glass of squash"), py_str("A glass of squash is about 200 ml, made up with water.")]), py_tuple(vec![py_str("glass of cola"), py_str("A glass of cola is about 200 ml, which is most of a 330 ml can.")])]) }
    s_LOOSE(__data11());
    s__LOOSE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:much|many|big)\\b|\\bgrams?\\b|\\bg\\b|\\bml\\b|\\bmls\\b|weigh|weighs|weight|ounces?\\b|\\boz\\b|what\\s+size")]).unwrap_or_else(|e| exc::raise(e)));
    s__LOOSE_NOT(stdlib_call("re", "compile", &[py_str("\\b(?:unit|units|alcohol|drink|drinking|pregnan\\w*|calorie\\w*|cost|costs|price|prices|cheap|expensive|drunk|drive|driving)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__IN_A_VESSEL(stdlib_call("re", "compile", &[py_str("\\b(?P<liquid>water|milk|juice|squash|cola|wine)\\s+in\\s+(?:a|one)\\s+(?:(?P<size>small|medium|big|large|tall)\\s+)?(?P<vessel>glass|cup|mug|pint)(?!\\s+of\\b)")]).unwrap_or_else(|e| exc::raise(e)));
    s__VESSEL_ML(py_dict(vec![(py_str("glass"), Py::Int(200)), (py_str("cup"), Py::Int(250)), (py_str("mug"), Py::Int(250)), (py_str("pint"), Py::Int(568))]));
    s_LIQUID_GRAMS_LITRE(py_tuple(vec![py_tuple(vec![py_str("milk"), Py::Int(1030)]), py_tuple(vec![py_str("cream"), Py::Int(1010)]), py_tuple(vec![py_str("honey"), Py::Int(1420)]), py_tuple(vec![py_str("oil"), Py::Int(920)]), py_tuple(vec![py_str("water"), Py::Int(1000)])]));
    s__PER_CUP(stdlib_call("re", "compile", &[py_str("\\bper\\s+(?:a\\s+|one\\s+|each\\s+)?(?:cups?|mugs?)\\b|\\bfor\\s+(?:a|one|each|every)\\s+(?:cups?|mugs?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_BIRD_GRAMS(Py::Int(500));
    s__BIRDS(py_tuple(vec![py_str("turkey"), py_str("turkeys"), py_str("chicken"), py_str("chickens"), py_str("duck"), py_str("ducks"), py_str("goose"), py_str("geese")]));
    s_MADE_NOT_MEASURED(call_builtin("frozenset", &[py_tuple(vec![py_str("the dish"), py_str("cake")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data12() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("new potatoes"), py_str("mash")]), Py::Int(250), py_str("g"), py_str("potatoes")]), py_tuple(vec![py_tuple(vec![py_str("chicken breast"), py_str("chicken breasts"), py_str("chicken thigh"), py_str("chicken thighs"), py_str("chicken")]), Py::Int(200), py_str("g"), py_str("chicken")]), py_tuple(vec![py_tuple(vec![py_str("mince"), py_str("beef mince"), py_str("lamb mince")]), Py::Int(150), py_str("g"), py_str("mince")]), py_tuple(vec![py_tuple(vec![py_str("steak"), py_str("steaks")]), Py::Int(200), py_str("g"), py_str("steak")]), py_tuple(vec![py_tuple(vec![py_str("salmon"), py_str("salmon fillet"), py_str("salmon fillets")]), Py::Int(150), py_str("g"), py_str("salmon")]), py_tuple(vec![py_tuple(vec![py_str("fish"), py_str("white fish"), py_str("cod"), py_str("haddock")]), Py::Int(150), py_str("g"), py_str("fish")]), py_tuple(vec![py_tuple(vec![py_str("meat"), py_str("joint"), py_str("roast beef"), py_str("beef"), py_str("lamb"), py_str("pork")]), Py::Int(250), py_str("g"), py_str("raw meat")]), py_tuple(vec![py_tuple(vec![py_str("turkey"), py_str("turkeys")]), g_BIRD_GRAMS(), py_str("g"), py_str("turkey")]), py_tuple(vec![py_tuple(vec![py_str("duck"), py_str("ducks")]), g_BIRD_GRAMS(), py_str("g"), py_str("duck")]), py_tuple(vec![py_tuple(vec![py_str("goose"), py_str("geese")]), g_BIRD_GRAMS(), py_str("g"), py_str("goose")]), py_tuple(vec![py_tuple(vec![py_str("ham"), py_str("gammon"), py_str("gammon joint")]), Py::Int(250), py_str("g"), py_str("ham")]), py_tuple(vec![py_tuple(vec![py_str("prawns"), py_str("prawn"), py_str("king prawns")]), Py::Int(150), py_str("g"), py_str("prawns")]), py_tuple(vec![py_tuple(vec![py_str("cheese"), py_str("cheddar")]), Py::Int(50), py_str("g"), py_str("cheese")]), py_tuple(vec![py_tuple(vec![py_str("sausage"), py_str("sausages")]), Py::Int(2), py_str("each"), py_str("sausages")]), py_tuple(vec![py_tuple(vec![py_str("egg"), py_str("eggs")]), Py::Int(2), py_str("each"), py_str("eggs")]), py_tuple(vec![py_tuple(vec![py_str("salad"), py_str("salad leaves"), py_str("lettuce"), py_str("rocket"), py_str("spinach")]), Py::Int(50), py_str("g"), py_str("salad leaves")]), py_tuple(vec![py_tuple(vec![py_str("couscous"), py_str("bulgur"), py_str("bulghur")]), Py::Int(60), py_str("g"), py_str("couscous")]), py_tuple(vec![py_tuple(vec![py_str("quinoa")]), Py::Int(60), py_str("g"), py_str("quinoa")]), py_tuple(vec![py_tuple(vec![py_str("lentils"), py_str("red lentils"), py_str("green lentils"), py_str("dal"), py_str("dhal")]), Py::Int(75), py_str("g"), py_str("dried lentils")]), py_tuple(vec![py_tuple(vec![py_str("prawns"), py_str("shrimp")]), Py::Int(150), py_str("g"), py_str("prawns")]), py_tuple(vec![py_tuple(vec![py_str("wine"), py_str("red wine"), py_str("white wine"), py_str("rose"), py_str("prosecco"), py_str("fizz"), py_str("champagne")]), Py::Float(0.5), py_str("bottle"), py_str("wine")]), py_tuple(vec![py_tuple(vec![py_str("chips"), py_str("oven chips"), py_str("fries"), py_str("french fries")]), Py::Int(150), py_str("g"), py_str("chips")]), py_tuple(vec![py_tuple(vec![py_str("carrot"), py_str("carrots")]), Py::Int(1), py_str("each"), py_str("carrot")]), py_tuple(vec![py_tuple(vec![py_str("sandwich"), py_str("sandwiches"), py_str("finger sandwiches"), py_str("rounds of sandwiches")]), Py::Int(3), py_str("each"), py_str("sandwiches")]), py_tuple(vec![py_tuple(vec![py_str("sausage roll"), py_str("sausage rolls")]), Py::Int(3), py_str("each"), py_str("sausage rolls")]), py_tuple(vec![py_tuple(vec![py_str("quiche"), py_str("quiches")]), Py::Int(150), py_str("g"), py_str("quiche")]), py_tuple(vec![py_tuple(vec![py_str("food"), py_str("dish"), py_str("casserole"), py_str("pasta bake"), py_str("curry")]), Py::Int(300), py_str("g"), py_str("the dish")]), py_tuple(vec![py_tuple(vec![py_str("cake")]), Py::Int(60), py_str("g"), py_str("cake")]), py_tuple(vec![py_tuple(vec![py_str("stuffing")]), Py::Int(50), py_str("g"), py_str("stuffing")]), py_tuple(vec![py_tuple(vec![py_str("gravy")]), Py::Int(150), py_str("ml"), py_str("gravy")]), py_tuple(vec![py_tuple(vec![py_str("pigs in blankets"), py_str("pig in blankets")]), Py::Int(3), py_str("each"), py_str("pigs in blankets")]), py_tuple(vec![py_tuple(vec![py_str("scone"), py_str("scones")]), Py::Int(2), py_str("each"), py_str("scones")]), py_tuple(vec![py_tuple(vec![py_str("hummus"), py_str("houmous")]), Py::Int(50), py_str("g"), py_str("hummus")]), py_tuple(vec![py_tuple(vec![py_str("porridge oats"), py_str("porridge"), py_str("oats")]), Py::Int(50), py_str("g"), py_str("porridge oats")]), py_tuple(vec![py_tuple(vec![py_str("soup")]), Py::Int(300), py_str("ml"), py_str("soup")]), py_tuple(vec![py_tuple(vec![py_str("meatballs"), py_str("meatball")]), Py::Int(4), py_str("each"), py_str("meatballs")]), py_tuple(vec![py_tuple(vec![py_str("slices of bread"), py_str("slice of bread"), py_str("bread")]), Py::Int(2), py_str("each"), py_str("slices of bread")]), py_tuple(vec![py_tuple(vec![py_str("sprouts"), py_str("brussels sprouts"), py_str("brussel sprouts")]), Py::Int(5), py_str("each"), py_str("sprouts")]), py_tuple(vec![py_tuple(vec![py_str("yorkshire puddings"), py_str("yorkshire pudding"), py_str("yorkshires")]), Py::Int(2), py_str("each"), py_str("yorkshire puddings")]), py_tuple(vec![py_tuple(vec![py_str("broccoli")]), Py::Int(80), py_str("g"), py_str("broccoli")]), py_tuple(vec![py_tuple(vec![py_str("baked beans"), py_str("beans")]), Py::Int(150), py_str("g"), py_str("baked beans")]), py_tuple(vec![py_tuple(vec![py_str("custard")]), Py::Int(100), py_str("ml"), py_str("custard")]), py_tuple(vec![py_tuple(vec![py_str("coleslaw")]), Py::Int(75), py_str("g"), py_str("coleslaw")]), py_tuple(vec![py_tuple(vec![py_str("pizza"), py_str("pizzas")]), Py::Int(3), py_str("each"), py_str("slices of pizza")]), py_tuple(vec![py_tuple(vec![py_str("burger"), py_str("burgers")]), Py::Int(2), py_str("each"), py_str("burgers")]), py_tuple(vec![py_tuple(vec![py_str("mince pie"), py_str("mince pies")]), Py::Int(2), py_str("each"), py_str("mince pies")])]) }
    s_PER_PERSON(__data12());
    s_PER_PERSON_WHEN_COUNTED(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("basmati"), py_str("long grain rice")]), Py::Int(75), py_str("g"), py_str("dry rice")]), py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("spaghetti"), py_str("penne"), py_str("fusilli"), py_str("macaroni"), py_str("tagliatelle")]), Py::Int(100), py_str("g"), py_str("dry pasta")])]));
    s_DISHES_FOR_A_TABLE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("chilli con carne"), py_str("chili con carne"), py_str("chilli"), py_str("chili"), py_str("chilli beef"), py_str("chilli mince")]), py_tuple(vec![py_str("mince")]), py_tuple(vec![py_str("rice")]), Py::Bool(true), Py::Bool(true)]), py_tuple(vec![py_tuple(vec![py_str("spaghetti bolognese"), py_str("bolognaise"), py_str("bolognese"), py_str("spag bol")]), py_tuple(vec![py_str("mince")]), py_tuple(vec![py_str("spaghetti"), py_str("pasta")]), Py::Bool(true), Py::Bool(false)]), py_tuple(vec![py_tuple(vec![py_str("lasagne"), py_str("lasagna")]), py_tuple(vec![py_str("mince")]), Py::None, Py::Bool(true), Py::Bool(false)]), py_tuple(vec![py_tuple(vec![py_str("shepherds pie"), py_str("shepherd's pie"), py_str("cottage pie")]), py_tuple(vec![py_str("mince")]), py_tuple(vec![py_str("potato")]), Py::Bool(false), Py::Bool(false)]), py_tuple(vec![py_tuple(vec![py_str("tarka dal"), py_str("dhal"), py_str("dal")]), py_tuple(vec![py_str("lentils")]), py_tuple(vec![py_str("rice")]), Py::Bool(false), Py::Bool(true)]), py_tuple(vec![py_tuple(vec![py_str("curry"), py_str("curries"), py_str("tikka masala"), py_str("korma"), py_str("bhuna"), py_str("jalfrezi")]), py_tuple(vec![py_str("chicken")]), py_tuple(vec![py_str("rice")]), Py::Bool(false), Py::Bool(true)]), py_tuple(vec![py_tuple(vec![py_str("casserole"), py_str("stew")]), py_tuple(vec![py_str("meat")]), py_tuple(vec![py_str("potato")]), Py::Bool(false), Py::Bool(false)])]));
    s__TIN_FOR(Py::Int(4));
    s__CHILLI_THE_VEGETABLE(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many) (?:red |green |small |big |hot )?chilli(?:es)?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DISH_PART(stdlib_call("re", "compile", &[py_str("\\s+(?:powder|powders|sauce|sauces|paste|pastes|oil|flakes|seeds|seasoning|spice|spices|mix|puree|ketchup|granules|extract|essence|leaf|leaves|plant|pepper|peppers)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TIME_IS_THE_ASK(stdlib_call("re", "compile", &[py_str("\\bhow long\\b|\\bhow soon\\b|\\bwhat time\\b|\\bwhen\\b|\\bhow many (?:hours?|minutes?|mins?|days?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__CONTAINER(stdlib_call("re", "compile", &[py_str("\\b(?:bags?|packs?|packets?|tins?|cans?|jars?|boxes?|cartons?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AN_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:much|many|big|large)\\b|\\bwhat size\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTING(stdlib_call("re", "compile", &[py_str("\\b(?:actually|no|nah|sorry|instead|make\\s+it|make\\s+that|change\\s+it|change\\s+that|now)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__PEOPLE_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:per\\s+(?:person|head)|an?\\s+person|an?\\s+head(?!\\s+of\\b)|people|persons?|of\\s+us|of\\s+you|guests|adults?|between\\s+us|party|table\\s+for|we(?:'re|\\s+are)\\s+\\d{1,3}\\b|there\\s+are\\s+\\d{1,3}\\b|expecting\\s+\\d{1,3}\\b|each\\s+(?:person|head|guest|adult|of\\s+us|one)|family\\s+of)\\b|\\beach\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BARE_FOR_COUNT(stdlib_call("re", "compile", &[py_str("\\bfor\\s+(?:\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_TABLE_BEFORE(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("them"), py_str("that"), py_str("this"), py_str("one"), py_str("some"), py_str("oven")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_TABLE(py_str("(?!\\s*(?:minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?|slices?|meals?|dinners?|lunches?|kg|kilos?|g|grams?|litres?|liters?|ml|pints?|cans?|tins?|packs?|batches?|portions?|servings?|kms?|kilometres?|kilometers?|k|miles?|metres?|meters?|m|ft|feet|foot|yards?|steps?|laps?|lengths?|rounds?|times?|degrees?))"));
    s__PEOPLE_COUNT(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:for|between)\\s+(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?:people|persons?|guests|adults|of\\s+us|of\\s+you|heads)\\b|\\b(\\d{1,2})\\s+(?:people|persons?|guests|adults|heads)\\b|\\bfamily\\s+of\\s+(\\d|one|two|three|four|five|six|seven|eight|nine|ten)\\b|\\bfor\\s+(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2})\\b"), &g_NOT_A_TABLE()), &py_str("|\\b(?:we(?:'re|\\s+are)|there(?:'s|\\s+are))\\s+(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b")), &g_NOT_A_TABLE()), &py_str("|\\b(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+of\\s+us\\b|\\b(?:party|table|group|booking|buffet)\\s+(?:of|for)\\s+(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b|\\b(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a dozen)\\s+coming\\b|\\b(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\s+(?:of\\s+us\\s+)?(?:for|at)\\s+(?:dinner|lunch|supper|christmas|the\\s+weekend)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__PORTION_AVOID(stdlib_call("re", "compile", &[py_str("\\b(?:calories|kcal|carbs?|protein|sugar|fat|fibre|fiber|cost|costs|price|prices|worth|budget|alcohol|units|feed|feeds|plant|planting|grow|growing|seeds?|leftover|spend|spending|money|pay|paid|shop|shopping|dressing|sauces?|mayo|mayonnaise|stock|dips?|cream\\s+sauces?|salad\\s+cream|chutney|left\\s+over|weigh|weighs|weight|heavy|boards?|cheeseboards?|platters?|powders?|spices?|seasoning|ground|flakes|granules|extracts?|essence|herbs?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__BAKING_PORTION(stdlib_call("re", "compile", &[py_str("\\b(?:cake|cakes|sponge|cupcakes?|batter|meringues?|pavlova|yorkshire|pancakes?|crepes?|pastry|dough|loaf|loaves|bread\\s+and\\s+butter)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PORTION_COUNT(stdlib_call("re", "compile", &[py_str("\\bhow many (?:portions|servings|slices|pieces|bits|helpings)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PORTION_WORD(stdlib_call("re", "compile", &[py_str("\\bportions?\\b|\\bservings?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_PEOPLE(py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]));
    s__PEOPLE_VALUE(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12))]));
    s__TABLE_GROUPS(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a dozen)\\s+(?:adults?|grown[\\s-]?ups?|pensioners?|teenagers?|kids?|children|child|toddlers?|babies|baby|guests?|people|persons?|heads|of\\s+us)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__GROUP_VALUE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__PEOPLE_VALUE()); for (k, v) in kw_pairs(&py_dict(vec![(py_str("a dozen"), Py::Int(12))])) { __kws.push((k, v)); } if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__A_FIGURE(stdlib_call("re", "compile", &[op_add(&py_str("\\b(\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b"), &g_NOT_A_TABLE())]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CLOCK(stdlib_call("re", "compile", &[py_str("\\bin the (?:morning|afternoon|evening)\\b|\\bat night\\b|\\bo ?clock\\b|\\b[ap]\\.?m\\.?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LAND_PLAIN(py_dict(vec![(py_str("acre"), py_str("An acre is 4,047 square metres, or 4,840 square yards: a square about 64 metres on each side, a little under half a hectare, and roughly half a football pitch.")), (py_str("hectare"), py_str("A hectare is 10,000 square metres: a square 100 metres on each side, two and a half acres, or about one and a half football pitches.")), (py_str("sq mile"), py_str("A square mile is 2.59 square kilometres, or 640 acres.")), (py_str("sq km"), py_str("A square kilometre is a square 1 kilometre on each side: 100 hectares, or about 0.39 of a square mile.")), (py_str("sq m"), py_str("A square metre is a square 1 metre on each side, about 10.8 square feet.")), (py_str("sq ft"), py_str("A square foot is 144 square inches, or about 0.09 square metres: a square 30cm on each side.")), (py_str("sq yd"), py_str("A square yard is 9 square feet, or about 0.84 square metres: a square 91cm on each side."))]));
    s__LAND_ASK(stdlib_call("re", "compile", &[py_str("\\bhow (?:big|large|long|wide)\\b|\\bsize of\\b|\\bwhat size\\b|\\bwhat(?:'s| is)\\b|\\bhow much (?:land|ground|space)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__AREA_PICTURES(py_tuple(vec![py_tuple(vec![py_str("parking space"), py_str("parking spaces"), Py::Float(12.5)]), py_tuple(vec![py_str("doubles tennis court"), py_str("doubles tennis courts"), Py::Float(260.0)]), py_tuple(vec![py_str("football pitch"), py_str("football pitches"), Py::Float(7140.0)])]));
    s__WORD_SHARE(py_tuple(vec![py_tuple(vec![py_str("three quarters"), Py::Float(0.75)]), py_tuple(vec![py_str("two thirds"), op_truediv(&Py::Int(2), &Py::Float(3.0))]), py_tuple(vec![py_str("three quarters of"), Py::Float(0.75)]), py_tuple(vec![py_str("two thirds of"), op_truediv(&Py::Int(2), &Py::Float(3.0))]), py_tuple(vec![py_str("a third"), op_truediv(&Py::Int(1), &Py::Float(3.0))]), py_tuple(vec![py_str("a quarter"), Py::Float(0.25)]), py_tuple(vec![py_str("a half"), Py::Float(0.5)]), py_tuple(vec![py_str("half"), Py::Float(0.5)]), py_tuple(vec![py_str("quarter"), Py::Float(0.25)]), py_tuple(vec![py_str("third"), op_truediv(&Py::Int(1), &Py::Float(3.0))])]));
    s__A_SIZE_JUDGEMENT(stdlib_call("re", "compile", &[py_str("\\b(?:big|large|small|huge|tiny|sizeable|decent|reasonable|enough)\\b|\\bhow much (?:land|ground|space)\\b|\\bis that a lot\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_FEET(stdlib_call("re", "compile", &[py_str("\\b(?:feet|foot|ft|tall)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SAYS_OWN_HEIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m|i am|im|my height is|my height's|my height)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HEIGHT_METRES(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d(?:\\.\\d{1,2})?)\\s*(?:m|metres?|meters?)(?!\\w)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HEIGHT_CM(stdlib_call("re", "compile", &[py_str("(?<![\\w.])(\\d{2,3}(?:\\.\\d)?)\\s*(?:cm|centimetres?|centimeters?)(?!\\w)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HEIGHT_FEET(stdlib_call("re", "compile", &[py_str("(?<![\\w.])([2-7])\\s*(?:'|\\u2019|ft|foot|feet)\\s*(\\d{1,2})\\s*(?:\\\"|''|in|inch|inches)?(?!\\w)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HEIGHT_FEET_WORD(stdlib_call("re", "compile", &[py_str("(?<![\\w.])([2-7])\\s*(?:ft|foot|feet)\\s+(\\d{1,2})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_HEIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:away|from|apart|long|wide|deep|of rope|of wood|of cable|of string|of pipe|of cord)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_ELSEWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|where|who|whose|should|shall|could|would|can|need|needs|looking for|look for|recommend|suggest|ideas|advice|buy|buying|wear|choose|pick|worth|fit|fits)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MEASURE_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:cm|centimetres?|centimeters?|m|metres?|meters?|feet|foot|ft|inches?|inch|tall|height|that|this|it|in|convert|converted)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_COMPLAINT(stdlib_call("re", "compile", &[py_str("\\b(?:hurt|hurts|hurting|ache|aches|aching|achy|sore|pain|painful|kills?|killing|blister|blisters|swollen|numb|too (?:small|tight|big|long|short|loose)|doesn'?t fit|don'?t fit|does not fit|do not fit|rubbed|rubbing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKS_HOW_MANY(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:many|much)\\s+(?P<thing>[a-z]+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MEASURE_SAID_ANOTHER_WAY(call_builtin("frozenset", &[py_tuple(vec![py_str("hand"), py_str("hands"), py_str("pace"), py_str("paces"), py_str("span"), py_str("spans")])]).unwrap_or_else(|e| exc::raise(e)));
    fn __data13() -> Py { py_tuple(vec![py_str("in"), py_str("is"), py_str("are"), py_str("of"), py_str("for"), py_str("grams"), py_str("gram"), py_str("g"), py_str("kg"), py_str("oz"), py_str("ounce"), py_str("ounces"), py_str("lb"), py_str("lbs"), py_str("cups"), py_str("cup"), py_str("ml"), py_str("litre"), py_str("litres"), py_str("per"), py_str("weight"), py_str("weigh"), py_str("weighs"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("each"), py_str("person"), py_str("people"), py_str("portion"), py_str("serving"), py_str("worth")]) }
    s__NOT_A_FOOD(call_builtin("frozenset", &[__data13()]).unwrap_or_else(|e| exc::raise(e)));
    s__POUNDS_IS_MONEY(stdlib_call("re", "compile", &[py_str("(?:£\\s*\\d|\\b\\d+(?:\\.\\d+)?\\s*(?:pounds?|quid|pence|p\\b|gbp))[^.?!]*?\\b(?:a lot|lot for|expensive|cheap|cheaper|cheapest|dear|dearer|overpriced|rip.?off|worth it|worth the|too much|reasonable|fair|good price|bad price|a rip|what price)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__HOW_TO(stdlib_call("re", "compile", &[py_str("(?:how\\s+do\\s+i|how\\s+to|how\\s+can\\s+i|how\\s+do\\s+you|convert|change|work\\s+out|turn)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data14() -> Py { py_tuple(vec![py_str("to"), py_str("in"), py_str("into"), py_str("as"), py_str("convert"), py_str("converting"), py_str("how"), py_str("do"), py_str("i"), py_str("many"), py_str("much"), py_str("is"), py_str("are"), py_str("there"), py_str("and"), py_str("the"), py_str("a"), py_str("an"), py_str("what"), py_str("s")]) }
    s__BARE_JOIN(call_builtin("frozenset", &[__data14()]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKING_AFTER_MEASURE(stdlib_call("re", "compile", &[py_str("\\b(should|shall|can|could|will|would|may|might|must|need|needs|needed|do|does|did|don't|doesn't|didn't|have to|has to|had to|ought|supposed to|allowed|is it|is that|are there|enough|normal|healthy|safe|too much|too many|until|till)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SHARE(stdlib_call("re", "compile", &[py_str("\\b(?P<word>three[\\s-]quarters?|two[\\s-]thirds?|one[\\s-](?:third|quarter|half)|a[\\s-](?:third|quarter|half|fifth|sixth)|half|\\d\\s*/\\s*\\d)\\s*(?:of\\s+)?(?:an?\\s+)?(?P<unit>cups?|tablespoons?|tbsp|teaspoons?|tsp)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data15() -> Py { py_dict(vec![(py_str("three quarters"), Py::Float(0.75)), (py_str("three quarter"), Py::Float(0.75)), (py_str("three-quarters"), Py::Float(0.75)), (py_str("three-quarter"), Py::Float(0.75)), (py_str("two thirds"), op_truediv(&Py::Int(2), &Py::Int(3))), (py_str("two third"), op_truediv(&Py::Int(2), &Py::Int(3))), (py_str("two-thirds"), op_truediv(&Py::Int(2), &Py::Int(3))), (py_str("two-third"), op_truediv(&Py::Int(2), &Py::Int(3))), (py_str("one third"), op_truediv(&Py::Int(1), &Py::Int(3))), (py_str("one-third"), op_truediv(&Py::Int(1), &Py::Int(3))), (py_str("one quarter"), Py::Float(0.25)), (py_str("one-quarter"), Py::Float(0.25)), (py_str("one half"), Py::Float(0.5)), (py_str("one-half"), Py::Float(0.5)), (py_str("a third"), op_truediv(&Py::Int(1), &Py::Int(3))), (py_str("a-third"), op_truediv(&Py::Int(1), &Py::Int(3))), (py_str("a quarter"), Py::Float(0.25)), (py_str("a-quarter"), Py::Float(0.25)), (py_str("a half"), Py::Float(0.5)), (py_str("a-half"), Py::Float(0.5)), (py_str("a fifth"), Py::Float(0.2)), (py_str("a-fifth"), Py::Float(0.2)), (py_str("a sixth"), op_truediv(&Py::Int(1), &Py::Int(6))), (py_str("a-sixth"), op_truediv(&Py::Int(1), &Py::Int(6))), (py_str("half"), Py::Float(0.5))]) }
    s__SHARE_VALUE(__data15());
    s__SHARE_FRACTION(stdlib_call("re", "compile", &[py_str("\\b(?:three[\\s-]quarters?|two[\\s-]thirds?|one[\\s-](?:third|quarter|half)|a[\\s-](?:third|quarter|half|fifth|sixth)|half|\\d\\s*/\\s*\\d)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SHARE_SAID(py_dict(vec![(py_str("three quarters"), py_str("three quarters")), (py_str("three-quarter"), py_str("three quarters")), (py_str("two thirds"), py_str("two thirds")), (py_str("two-third"), py_str("two thirds")), (py_str("one third"), py_str("a third")), (py_str("one-quarter"), py_str("a quarter")), (py_str("one half"), py_str("half")), (py_str("half"), py_str("half"))]));
    s__SHARE_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+is|whats|what's|how\\s+much\\s+is|how\\s+much|how\\s+many|how\\s+big\\s+is)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_ABOUT_A_MEASURE(stdlib_call("re", "compile", &[py_str("\\bgrams?\\b|\\bg\\b|\\bml\\b|\\bmillilitres?\\b|\\bmilliliters?\\b|\\blitres?\\b|\\bliters?\\b|\\bounces?\\b|\\boz\\b|\\bpounds?\\b|\\blbs?\\b|\\bpints?\\b|\\bquarts?\\b|\\bcelsius\\b|\\bfahrenheit\\b|\\bdegrees?\\b|\\bcalories?\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__TIME_SMALLER(py_dict(vec![(py_str("hour"), py_tuple(vec![Py::Int(60), py_str("minutes")])), (py_str("minute"), py_tuple(vec![Py::Int(60), py_str("seconds")])), (py_str("day"), py_tuple(vec![Py::Int(24), py_str("hours")])), (py_str("week"), py_tuple(vec![Py::Int(7), py_str("days")]))]));
    s__TIME_ARTICLE(py_dict(vec![(py_str("hour"), py_str("an hour")), (py_str("minute"), py_str("a minute")), (py_str("day"), py_str("a day")), (py_str("week"), py_str("a week"))]));
    fn __data16() -> Py { py_dict(vec![(py_str("hour"), py_str("hour")), (py_str("hours"), py_str("hour")), (py_str("hr"), py_str("hour")), (py_str("hrs"), py_str("hour")), (py_str("second"), py_str("second")), (py_str("seconds"), py_str("second")), (py_str("sec"), py_str("second")), (py_str("secs"), py_str("second")), (py_str("minute"), py_str("minute")), (py_str("minutes"), py_str("minute")), (py_str("min"), py_str("minute")), (py_str("mins"), py_str("minute")), (py_str("day"), py_str("day")), (py_str("days"), py_str("day")), (py_str("week"), py_str("week")), (py_str("weeks"), py_str("week"))]) }
    s__TIME_UNIT(__data16());
    fn __data17() -> Py { py_dict(vec![(py_str("a"), Py::Int(1)), (py_str("an"), Py::Int(1)), (py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fifteen"), Py::Int(15)), (py_str("twenty"), Py::Int(20)), (py_str("thirty"), Py::Int(30)), (py_str("forty"), Py::Int(40)), (py_str("fifty"), Py::Int(50))]) }
    s__NUMERAL(__data17());
    fn __data18() -> Py { py_dict(vec![(py_str("half"), Py::Int(2)), (py_str("halves"), Py::Int(2)), (py_str("third"), Py::Int(3)), (py_str("thirds"), Py::Int(3)), (py_str("quarter"), Py::Int(4)), (py_str("quarters"), Py::Int(4)), (py_str("fifth"), Py::Int(5)), (py_str("fifths"), Py::Int(5)), (py_str("sixth"), Py::Int(6)), (py_str("sixths"), Py::Int(6)), (py_str("eighth"), Py::Int(8)), (py_str("eighths"), Py::Int(8)), (py_str("tenth"), Py::Int(10)), (py_str("tenths"), Py::Int(10)), (py_str("twelfth"), Py::Int(12)), (py_str("twelfths"), Py::Int(12)), (py_str("twentieth"), Py::Int(20)), (py_str("twentieths"), Py::Int(20)), (py_str("twenty-fourth"), Py::Int(24)), (py_str("twenty-fourths"), Py::Int(24))]) }
    s__DENOMINATOR(__data18());
    s__A_FRACTION(py_str("(?:(?:a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d+)[\\s-]+(?:halves?|thirds?|quarters?|fifths?|sixths?|eighths?|tenths?|twelfths?|twentieths?|twenty[\\s-]fourths?)|half|\\d+\\s*/\\s*\\d+|\\d*\\.\\d+)"));
    s__A_TIME_SHARE(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![\\w.])(?P<word>"), &g__A_FRACTION()), &py_str(")\\s*(?:of\\s+)?(?:an?\\s+|one\\s+)?(?P<unit>hours?|hrs?|minutes?|mins?|days?|weeks?)\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_MIXED_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?P<whole>a|an|one|two|three|four|five|six|seven|eight|nine|ten|\\d+(?:\\.\\d+)?)\\s+(?P<unit>hours?|hrs?|days?|weeks?)\\s+(?:and\\s+)?(?P<rest>a\\s+half|half|(?P<n>\\d+|one|two|three|four|five|six|seven|eight|nine|ten|fifteen|twenty|thirty|forty|forty five|fifty)\\s+(?P<small>minutes?|mins?))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FRACTION_OF_A_MIXED(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![\\w.])(?P<word>"), &g__A_FRACTION()), &py_str(")\\s*(?:of|out\\s+of)\\s+(?P<span>.+)$"))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_HALF_SPAN(stdlib_call("re", "compile", &[py_str("\\b(?P<whole>a|an|one|two|three|four|five|\\d+)\\s+and\\s+a\\s+half\\s+(?P<unit>hours?|hrs?|days?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FRACTION_OF(stdlib_call("re", "compile", &[py_str("\\b(?:what|which|how\\s+much)\\s+(?:a\\s+)?fraction\\s+of\\s+(?:an?\\s+|one\\s+)?(?P<unit>hour|day|week|minute)\\s+(?:is|of)\\s+(?P<amount>\\d+(?:\\.\\d+)?|a|an|one)\\s*(?P<small>minutes?|mins?|hours?|hrs?|seconds?|secs?|days?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FRACTION_AS(stdlib_call("re", "compile", &[py_str("\\b(?P<amount>\\d+(?:\\.\\d+)?|a|an|one)\\s*(?P<small>minutes?|mins?|hours?|hrs?|seconds?|secs?|days?)\\s+(?:as\\s+)?(?:a\\s+)?fraction\\s+of\\s+(?:an?\\s+|one\\s+)?(?P<unit>hour|day|week|minute)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SPAN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what\\s+is|whats|what's|how\\s+much|how\\s+many|how\\s+long|how\\s+big)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_FRACTION_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:what(?:s|'s)?|which|how\\s+much|how\\s+many)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_TARGET_UNIT(stdlib_call("re", "compile", &[py_str("\\b(?:in|to|into)\\s+(?:minutes?|mins?|seconds?|secs?|hours?|hrs?|days?|weeks?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_UNIT_ASKED(stdlib_call("re", "compile", &[py_str("\\b(?:how\\s+many|how\\s+much|in|to|into)\\s+(?:a\\s+|an\\s+|one\\s+)?(minutes?|mins?|seconds?|secs?|hours?|hrs?|days?|weeks?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__TIME_SECONDS(py_dict(vec![(py_str("second"), Py::Int(1)), (py_str("minute"), Py::Int(60)), (py_str("hour"), Py::Int(3600)), (py_str("day"), Py::Int(86400)), (py_str("week"), Py::Int(604800))]));
    s__TIME_PLURAL(py_dict(vec![(py_str("second"), py_str("seconds")), (py_str("minute"), py_str("minutes")), (py_str("hour"), py_str("hours")), (py_str("day"), py_str("days")), (py_str("week"), py_str("weeks"))]));
    s__A_WHOLE_UNIT(stdlib_call("re", "compile", &[py_str("^\\s*(?:how\\s+long\\s+is|how\\s+long's|what\\s+is|what's|whats)\\s+(?:an?|one|1)\\s+(?P<unit>hours?|days?|weeks?|minutes?)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__SHARE_THAT(stdlib_call("re", "compile", &[py_str("\\b(?P<word>three[\\s-]quarters?|two[\\s-]thirds?|one[\\s-](?:third|quarter|half)|a[\\s-](?:third|quarter|half|fifth|sixth)|half)\\s+(?:of\\s+)?(?:that|it|the same)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SHARE_LEAD(stdlib_call("re", "compile", &[py_str("^(?:and|so|then|ok|okay|what|how|about|much|is|whats|what's)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__WEIGHT_SAID(stdlib_call("re", "compile", &[py_str("\\b(\\d+(?:\\.\\d+)?)\\s*(kg|kilo|kilos|kilogram|kilograms|g|gram|grams|gramme|grammes)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PORTIONS_ASK(stdlib_call("re", "compile", &[py_str("\\bportions?\\b|\\bservings?\\b|\\bserves?\\b|\\bfeed(?:s|ing)?\\b|\\benough for\\b|\\bi need\\b|\\bi've got\\b|\\bi have\\b(?!\\s+to\\b)|\\bi bought\\b|\\bhow many\\b|\\bfor how many\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_COUNT(stdlib_call("re", "compile", &[py_str("\\b(\\d+)\\s+(?:\\w+\\s+){0,2}?(?P<food>[a-z][a-z' ]*)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHOLE_THING_SERVES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("chickens")]), py_str("about 1.8 to 2kg"), Py::Int(4), Py::Int(6), py_str("two 1.5 to 1.8kg birds rather than one big one: they cook evenly and fit a normal tin"), Py::Int(500), py_str("500g a person on the bone"), py_str(", and the picked frame gives another two meals' worth")]), py_tuple(vec![py_tuple(vec![py_str("duck"), py_str("ducks")]), py_str("about 2kg"), Py::Int(4), Py::None, py_str("two birds"), Py::Int(500), py_str("500g a person on the bone"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("leg of lamb"), py_str("lamb leg"), py_str("leg of mutton")]), py_str("about 2kg"), Py::Int(6), Py::Int(8), py_str("a leg and a shoulder"), Py::Int(300), py_str("300g a person on the bone"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("shoulder of lamb"), py_str("lamb shoulder")]), py_str("about 1.75kg"), Py::Int(6), Py::Int(8), py_str("a shoulder and a leg"), Py::Int(250), py_str("250g a person on the bone"), py_str("")]), py_tuple(vec![py_tuple(vec![py_str("gammon joint"), py_str("ham joint"), py_str("gammon"), py_str("whole ham")]), py_str("about 1.5kg"), Py::Int(8), Py::Int(10), py_str("two 1kg joints, or a bigger one for sandwiches after"), Py::Int(200), py_str("200g a person"), py_str(", and the end of it sandwiches the week")]), py_tuple(vec![py_tuple(vec![py_str("whole salmon"), py_str("side of salmon")]), py_str("about 1.5kg"), Py::Int(8), Py::None, py_str("two sides, or a side and a couple of fillets"), Py::Int(180), py_str("180g a person"), py_str("")])]));
    s__A_CUT_NOT_A_BIRD(stdlib_call("re", "compile", &[py_str("\\b(?:breasts?|thighs?|wings?|drumsticks?|fillets?|nuggets?|strips?|tenders?|mince|sausages?|joints?\\s+of\\s+meat|chops?|steaks?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__FEEDS_ASK(stdlib_call("re", "compile", &[py_str("\\benough\\b|\\bfeed(?:s|ing)?\\b|\\bserves?\\b|\\bgo(?:es)?\\s+round\\b|\\bhow\\s+far\\b|\\bdo\\s+for\\b|\\bhow\\s+many\\s+(?:people|persons?|of\\s+us|adults?|guests?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__NOT_A_TABLE_QUESTION(stdlib_call("re", "compile", &[py_str("\\b(?:how long|how do i|how to|temperature|oven|roast it|bake|reheat|fridge|freezer|freeze|keep|keeps|lasts?|calories|kcal|protein|carbs?|fat|healthy|cost|costs|price|cheap|expensive|buy|shopping|shopping list|defrost|thaw|stuffing|recipe|meals?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__KEEPING_HENS(stdlib_call("re", "compile", &[py_str("\\b(?:hens?|coop|run|pellets|layers?|laying|eggs?|broody|cockerel|chicks?|feed my|keeping chickens?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__WHOLE_WEIGHT(stdlib_call("re", "compile", &[py_str("\\b(\\d+(?:\\.\\d+)?)\\s*(kg|kilo|kilos|kilogram|kilograms)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__PLURAL_BIRDS(mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHOLE_THING_SERVES()) { let __unpk5413 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk5413[0].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); if py_truthy(&mcall(&name.clone(), "endswith", &[py_str("s")])) {  __out.push(name.clone()); } } } py_list(__out) })() }]));
    s__PEOPLE_WORD_TO_TWENTY(py_dict(vec![(Py::Int(13), py_str("thirteen")), (Py::Int(14), py_str("fourteen")), (Py::Int(15), py_str("fifteen")), (Py::Int(16), py_str("sixteen")), (Py::Int(17), py_str("seventeen")), (Py::Int(18), py_str("eighteen")), (Py::Int(19), py_str("nineteen")), (Py::Int(20), py_str("twenty"))]));
    s__COUNT_OF_BIRDS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(how many|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\\d{1,2})\\s+(?:whole\\s+)?("), &g__PLURAL_BIRDS()), &py_str(")\\b"))]).unwrap_or_else(|e| exc::raise(e)));
    s__TRAILING_JOIN(stdlib_call("re", "compile", &[py_str("\\s+(?:then|please)\\s*$")]).unwrap_or_else(|e| exc::raise(e)));
    s__CARRIED_LEAD(stdlib_call("re", "compile", &[py_str("^(?:and\\s+(?:then\\s+|what\\s+about\\s+|how\\s+about\\s+)?|what\\s+about\\s+|how\\s+about\\s+|or\\s+|then\\s+|also\\s+|how\\s+much\\s+|how\\s+many\\s+)")]).unwrap_or_else(|e| exc::raise(e)));
    s__ASKED_MEASURE(stdlib_call("re", "compile", &[py_str("how\\s+(?:many|much)\\s+(?P<want>[a-z]+)")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data19() -> Py { py_tuple(vec![py_str("in"), py_str("to"), py_str("of"), py_str("a"), py_str("an"), py_str("the"), py_str("one"), py_str("and"), py_str("so"), py_str("what"), py_str("about"), py_str("how"), py_str("many"), py_str("much"), py_str("then"), py_str("big"), py_str("size"), py_str("is"), py_str("that"), py_str("it"), py_str("same"), py_str("for"), py_str("per"), py_str("please"), py_str("s"), py_str("gram"), py_str("grams"), py_str("g"), py_str("ml"), py_str("mls"), py_str("litre"), py_str("litres"), py_str("ounce"), py_str("ounces"), py_str("oz"), py_str("pound"), py_str("pounds"), py_str("lb"), py_str("lbs"), py_str("kilo"), py_str("kilos"), py_str("kilogram"), py_str("kilograms"), py_str("kg"), py_str("cup"), py_str("cups"), py_str("mug"), py_str("mugs"), py_str("tablespoon"), py_str("tablespoons"), py_str("tbsp"), py_str("teaspoon"), py_str("teaspoons"), py_str("tsp"), py_str("dessertspoon"), py_str("dessertspoons"), py_str("dsp"), py_str("pint"), py_str("pints"), py_str("fl"), py_str("fluid")]) }
    s__CARRIED_FILLER(call_builtin("frozenset", &[__data19()]).unwrap_or_else(|e| exc::raise(e)));
    s__CARRIED_PEOPLE(stdlib_call("re", "compile", &[py_str("^(?:for\\s+)?(?P<count>\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)(?:\\s+(?:people|persons?|guests|adults|heads|of\\s+us))?$")]).unwrap_or_else(|e| exc::raise(e)));
    s__CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:i meant|i mean|sorry|actually|no wait|make that|on second thoughts|i said|not\\s+\\d)")]).unwrap_or_else(|e| exc::raise(e)));
    s__NEW_WEIGHT_THEN(stdlib_call("re", "compile", &[py_str("^(?:and\\s+|ok\\s+|okay\\s+|so\\s+|but\\s+)?(?:(?:can|could)\\s+you\\s+)?(?:(?:make|change|do|set|use)\\s+(?:it|that|them|this)\\s+(?:to\\s+|at\\s+|as\\s+)?|what\\s+about\\s+|how\\s+about\\s+|and\\s+)?(?P<amount>(?:\\d+(?:\\.\\d+)?)\\s*(?:kg|kilo|kilos|kilogram|kilograms|g|gram|grams|gramme|grammes)|half\\s+a\\s+(?:kilo|kg)|a\\s+(?:kilo|kg)|one\\s+(?:kilo|kg))(?:\\s+(?:instead|then|please|now|of\\s+that))?$")]).unwrap_or_else(|e| exc::raise(e)));
    s__CARRIED_NOW(stdlib_call("re", "compile", &[py_str("^(?:so |ok |okay |and |but |then )*(?:how much|how many|what about|and)(?: of (?:it|that|them|those|these))?(?: now| then| again)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s__FRACTIONS(py_dict(vec![(Py::Float(0.25), py_str("a quarter cup")), (Py::Float(0.5), py_str("half a cup")), (Py::Float(0.75), py_str("three quarters of a cup")), (op_truediv(&Py::Int(1), &Py::Int(3)), py_str("a third of a cup")), (op_truediv(&Py::Int(2), &Py::Int(3)), py_str("two thirds of a cup")), (Py::Float(1.5), py_str("one and a half cups")), (Py::Float(2.5), py_str("two and a half cups"))]));
    s__SPECIFIC(stdlib_call("re", "compile", &[py_str("\\b(gas mark|fan oven|fan|cooking|recipe|stick of butter|sticks? of butter)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__SAID_COUNT(py_dict(vec![(Py::Int(2), py_str("Two")), (Py::Int(3), py_str("Three")), (Py::Int(4), py_str("Four")), (Py::Int(5), py_str("Five")), (Py::Int(6), py_str("Six"))]));
}
pub fn _the_component_is_the_whole_measure(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut component: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("True when the component word is all the measure holds: \"a teaspoon of sugar\".");
    found = mcall(&g__COMPONENT_MEASURED(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    again = mcall(&found.clone(), "group", &[py_str("again")]);
    if py_truthy(&Py::Bool(!py_truthy(&again.clone()))) {
    return Py::Bool(true);
    }
    component = mcall(&found.clone(), "group", &[py_str("component")]);
    return Py::Bool(py_contains(&py_tuple(vec![component.clone(), op_add(&component.clone(), &py_str("s"))]), &again.clone()));
    Py::None
}
pub fn _the_thing_measured_by_itself(args: &[Py]) -> Py {
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
    let mut again: Py = Py::None;
    let mut food: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let _ = py_str("The same question written the way the weight tables read it, or None.\n\n    \"how much flour in a cup\" names no weight, so the tables had no line to read and the answer\n    was a shrug while \"how many grams in a cup of flour\" was 120 g. The food is the whole of what\n    the measure holds, so the line is rewritten as a weight question and answered there.\n    ");
    found = mcall(&g__DRY_IN_A_MEASURE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    held = mcall(&g__COMPONENT_MEASURED(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&held.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&mcall(&held.clone(), "group", &[py_str("measure")]), "startswith", &[py_str("cup")]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_the_component_is_the_whole_measure(&[clean.clone()])))) {
    return Py::None;
    }
    found = held.clone();
    }
    food = { let __b = mcall(&mcall(&found.clone(), "groupdict", &[]), "get", &[py_str("food")]); if py_truthy(&__b) { __b } else { mcall(&found.clone(), "group", &[py_str("component")]) } };
    again = mcall(&found.clone(), "group", &[py_str("again")]);
    if py_truthy(&{ let __b = again.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![food.clone(), op_add(&food.clone(), &py_str("s")), geti(&mcall(&food.clone(), "split", &[]), &op_neg(&Py::Int(1)))]), &again.clone())) } }) {
    return Py::None;
    }
    return { let mut __s = String::new(); __s.push_str("how many grams in a "); __s.push_str(&fstr(&mcall(&found.clone(), "group", &[py_str("measure")]), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&food.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn _object_weight(args: &[Py]) -> Py {
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
    let mut article: Py = Py::None;
    let mut best: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What a whole thing weighs: \"how much does a suitcase weigh\", \"how heavy is a laptop\".");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__WEIGH(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\btoo\\s+(?:heavy|much)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow\\s+(?:much|heavy|many|big)\\b|\\bwhat(?:'s| is)?\\s+(?:the\\s+)?weight\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:cost|costs|price|prices|worth|sell|selling|buy|buying|allowance|allowed|limit|limits|maximum|max|can)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_OBJECT_GRAMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    grams = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&py_tuple(vec![name.clone(), _many(&[name.clone()])]));
    for __item in __iter3 {
    words = __item.clone();
    match_ = stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])(?=\\s*(?:$|[,?]|weigh|weighs|heavy|about|is\\b))"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__COUNTED_THING(), "search", &[slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), grams.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return Py::None;
    }
    let __unpk4 = py_unpack(&best.clone(), 2, None);
    name = __unpk4[0].clone();
    grams = __unpk4[1].clone();
    article = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &geti(&name.clone(), &Py::Int(0))))) { py_str("an") } else { py_str("a") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&article.clone(), "capitalize", &[]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_kilos(&[grams.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
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
    let mut name: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let _ = py_str("The words for more than one: \"bikes\", \"washing machines\".");
    if py_truthy(&{ let __b = mcall(&name.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&name.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    }
    if py_truthy(&mcall(&name.clone(), "endswith", &[py_tuple(vec![py_str("o"), py_str("ch"), py_str("sh"), py_str("s"), py_str("x")])])) {
    return op_add(&name.clone(), &py_str("es"));
    }
    return op_add(&name.clone(), &py_str("s"));
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
    let mut grams: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "grams") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: grams"))) };
    let _ = py_str("A weight the way someone says a big thing: 12 kg, 1.5 kg, 200 g.");
    if py_truthy(&op_ge(&grams.clone(), &Py::Int(1000))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[op_truediv(&grams.clone(), &Py::Int(1000))]), -1, "")); __s.push_str(" kg"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[grams.clone()]), -1, "")); __s.push_str(" g"); py_string(__s) };
    Py::None
}
pub fn _build(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut _u: Py = Py::None;
    let mut family: Py = Py::None;
    let mut items: Py = Py::None;
    let mut names: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("{word people type: (family, unit)}, and one regex of every word, longest first.");
    names = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_UNITS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    family = __unpk2[0].clone();
    items = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&items.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 3, None);
    unit = __unpk4[0].clone();
    words = __unpk4[1].clone();
    _u = __unpk4[2].clone();
    let __iter5 = py_iter_or_raise(&words.clone());
    for __item in __iter5 {
    word = __item.clone();
    let _ = py_setitem(&names.clone(), &word.clone(), py_tuple(vec![family.clone(), unit.clone()]));
    if py_truthy(&{ let __b = op_le(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "isalpha", &[]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("s")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__NO_PLURAL(), &word.clone())) } } } } } }) {
    let _ = mcall(&names.clone(), "setdefault", &[op_add(&word.clone(), &py_str("s")), py_tuple(vec![family.clone(), unit.clone()])]);
    }
    }
    }
    }
    pattern = mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(names.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    return py_tuple(vec![names.clone(), stdlib_call("re", "compile", &[op_add(&op_add(&py_str("(?<![\\w.])(?:"), &pattern.clone()), &py_str(")(?![\\w])"))]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _factor(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut family: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut _u: Py = Py::None;
    let mut factor: Py = Py::None;
    let mut name: Py = Py::None;
    let __iter1 = py_iter_or_raise(&geti(&g_UNITS(), &family.clone()));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    _u = __unpk2[1].clone();
    factor = __unpk2[2].clone();
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &unit.clone()))) {
    return factor.clone();
    }
    }
    return Py::None;
    Py::None
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
    let mut unit: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut _u: Py = Py::None;
    let mut family: Py = Py::None;
    let mut items: Py = Py::None;
    let mut name: Py = Py::None;
    let mut words: Py = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g_UNITS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    family = __unpk2[0].clone();
    items = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&items.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 3, None);
    name = __unpk4[0].clone();
    words = __unpk4[1].clone();
    _u = __unpk4[2].clone();
    if py_truthy(&Py::Bool(py_eq(&name.clone(), &unit.clone()))) {
    return words.clone();
    }
    }
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _grams(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut rounded: Py = Py::None;
    let _ = py_str("A weight the way a cook says it: whole grams once there are ten or more.");
    rounded = op_add(&value.clone(), &Py::Float(1e-09));
    return if py_truthy(&op_ge(&call_builtin("abs", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10))) { { let mut __s = String::new(); __s.push_str(&fstr(&rounded.clone(), -1, &",.0f")); py_string(__s) } } else { mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".1f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]) };
    Py::None
}
pub fn _fmt(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut size: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A number the way someone would say it: no long tails of decimals.");
    size = call_builtin("abs", &[value.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_ge(&size.clone(), &Py::Int(1000))) {
    text = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &",.0f")); py_string(__s) };
    } else {
    if py_truthy(&op_ge(&size.clone(), &Py::Int(100))) {
    text = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".0f")); py_string(__s) };
    } else {
    if py_truthy(&op_ge(&size.clone(), &Py::Int(1))) {
    text = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".1f")); py_string(__s) };
    } else {
    text = { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".2f")); py_string(__s) };
    }
    }
    }
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &py_str(".")))) {
    text = mcall(&mcall(&text.clone(), "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    }
    return text.clone();
    Py::None
}
pub fn _value(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut raw: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "raw") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: raw"))) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("one")]), &raw.clone()))) {
    return Py::Float(1.0);
    }
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("float", &[mcall(&raw.clone(), "replace", &[py_str(","), py_str("")])]).unwrap_or_else(|e| exc::raise(e))]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ValueError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _mentions(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("Every unit the line names: [(start, end, family, unit)], in the order typed.");
    found = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g__UNIT_WORD(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    let __unpk2 = py_unpack(&geti(&g__NAMES(), &mcall(&match_.clone(), "group", &[Py::Int(0)])), 2, None);
    family = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&match_.clone(), "group", &[Py::Int(0)]), &py_str("in"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&_number_before(&[clean.clone(), mcall(&match_.clone(), "start", &[])]), &Py::None)) } }) {
    continue;
    }
    let _ = mcall(&found.clone(), "append", &[py_tuple(vec![mcall(&match_.clone(), "start", &[]), mcall(&match_.clone(), "end", &[]), family.clone(), unit.clone()])]);
    }
    return found.clone();
    Py::None
}
pub fn _number_before(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut found: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut window: Py = Py::None;
    let _ = py_str("The number just before a unit (\"20 miles\", \"a pound\"), or None.");
    window = slice_or_raise(&clean.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&start.clone(), &Py::Int(24))]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    found = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g__NUMBER(), "finditer", &[window.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&slice_or_raise(&window.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[])))) {
    found = _value(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    }
    }
    return found.clone();
    Py::None
}
pub fn _say(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut one: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { Py::None };
    let _ = py_str("The unit as it is said: singular only for one.");
    if py_truthy(&if py_truthy(&Py::Bool(!py_is(&one.clone(), &Py::None))) { one.clone() } else { Py::Bool(py_eq(&value.clone(), &Py::Int(1))) }) {
    return mcall(&g_SAID_ONE(), "get", &[unit.clone(), mcall(&g_SAID(), "get", &[unit.clone(), unit.clone()])]);
    }
    return mcall(&g_SAID(), "get", &[unit.clone(), unit.clone()]);
    Py::None
}
pub fn _split_time(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut result: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "result") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: result"))) };
    let mut to_unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "to_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: to_unit"))) };
    let mut per: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut small_unit: Py = Py::None;
    let mut smaller: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A decimal count of weeks or days as whole units and the rest (\"12 weeks and 6 days\").");
    smaller = mcall(&g__SPLIT(), "get", &[to_unit.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&smaller.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("abs", &[op_sub(&result.clone(), &call_builtin("round", &[result.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09)) } }) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&smaller.clone(), 2, None);
    small_unit = __unpk1[0].clone();
    per = __unpk1[1].clone();
    whole = call_builtin("int", &[result.clone()]).unwrap_or_else(|e| exc::raise(e));
    rest = call_builtin("round", &[op_mul(&op_sub(&result.clone(), &whole.clone()), &per.clone())]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_eq(&rest.clone(), &Py::Int(0))); if py_truthy(&__b) { __b } else { op_ge(&rest.clone(), &per.clone()) } }) {
    return Py::None;
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_say(&[whole.clone(), to_unit.clone()]), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&rest.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_say(&[rest.clone(), small_unit.clone()]), -1, "")); py_string(__s) };
    Py::None
}
pub fn _sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut family: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut from_unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "from_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: from_unit"))) };
    let mut to_unit: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "to_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: to_unit"))) };
    let mut in_phrase: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "in_phrase") { v.clone() }
    else { Py::Bool(false) };
    let mut about: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut factor: Py = Py::None;
    let mut left: Py = Py::None;
    let mut other: Py = Py::None;
    let mut result: Py = Py::None;
    let mut said: Py = Py::None;
    let mut split: Py = Py::None;
    let mut verb: Py = Py::None;
    let __unpk1 = py_unpack(&py_tuple(vec![_factor(&[family.clone(), from_unit.clone()]), _factor(&[family.clone(), to_unit.clone()])]), 2, None);
    factor = __unpk1[0].clone();
    other = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&factor.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&other.clone())) } }) {
    return Py::None;
    }
    result = op_truediv(&op_mul(&value.clone(), &factor.clone()), &other.clone());
    about = if py_truthy(&Py::Bool(py_eq(&mcall(&g_SYSTEM(), "get", &[from_unit.clone()]), &mcall(&g_SYSTEM(), "get", &[to_unit.clone()])))) { py_str("") } else { py_str("about ") };
    left = if py_truthy(&Py::Bool(py_eq(&value.clone(), &Py::Int(1)))) { op_add(&py_str("one "), &_say(&[Py::Int(1), from_unit.clone()])) } else { _amount_phrase(&[value.clone(), from_unit.clone()]) };
    split = if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("time")))) { _split_time(&[result.clone(), to_unit.clone()]) } else { Py::None };
    if py_truthy(&split.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&left.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&left.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&about.clone(), -1, "")); __s.push_str(&fstr(&split.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(result.clone()); __args.push(to_unit.clone()); __kws.extend(vec![("spoons".to_string(), { let __b = Py::Bool(py_eq(&family.clone(), &py_str("mass"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&to_unit.clone(), &py_str("cup"))) } })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cups_phrase(args))(&__args) };
    amount = if py_truthy(&Py::Bool(!py_eq(&said.clone(), &{ let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[result.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(result.clone()); __args.push(to_unit.clone()); __kws.extend(vec![("one".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) }, -1, "")); py_string(__s) }))) { { let mut __s = String::new(); __s.push_str(&fstr(&about.clone(), -1, "")); __s.push_str(&fstr(&said.clone(), -1, "")); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str(&fstr(&about.clone(), -1, "")); __s.push_str(&fstr(&_fmt(&[result.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(result.clone()); __args.push(to_unit.clone()); __kws.extend(vec![("one".to_string(), Py::Bool(py_eq(&_fmt(&[result.clone()]), &py_str("1"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) }, -1, "")); py_string(__s) } };
    if py_truthy(&{ let __b = in_phrase.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&value.clone(), &Py::Int(1))) } }) {
    verb = if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&result.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.06))) { py_str("is") } else { py_str("are") };
    return { let mut __s = String::new(); __s.push_str("There "); __s.push_str(&fstr(&verb.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&left.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&left.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&left.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _temperature(args: &[Py]) -> Py {
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
    let mut celsius: Py = Py::None;
    let mut convert: Py = Py::None;
    let mut fahrenheit: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut source: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("(-?\\d+(?:\\.\\d+)?)\\s*(?:degrees?\\s*|°\\s*)?(?:c|celsius|centigrade)\\b.*?\\b(?:in|to|into|as)\\s*(?:degrees?\\s*)?(?:f|fahrenheit)\\b"), py_str("F"), { let celsius = celsius.clone(); let clean = clean.clone(); let convert = convert.clone(); let fahrenheit = fahrenheit.clone(); let mark = mark.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let source = source.clone(); let value = value.clone(); let wanted = wanted.clone(); py_func("lambda", move |args: &[Py]| { let celsius = celsius.clone(); let clean = clean.clone(); let convert = convert.clone(); let fahrenheit = fahrenheit.clone(); let mark = mark.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let source = source.clone(); let value = value.clone(); let wanted = wanted.clone(); let c: Py = __kwp(args, 0, "c").unwrap_or(Py::None); op_add(&op_truediv(&op_mul(&c.clone(), &Py::Int(9)), &Py::Int(5)), &Py::Int(32)) }) }]), py_tuple(vec![py_str("(-?\\d+(?:\\.\\d+)?)\\s*(?:degrees?\\s*|°\\s*)?(?:f|fahrenheit)\\b.*?\\b(?:in|to|into|as)\\s*(?:degrees?\\s*)?(?:c|celsius|centigrade)\\b"), py_str("C"), { let celsius = celsius.clone(); let clean = clean.clone(); let convert = convert.clone(); let fahrenheit = fahrenheit.clone(); let mark = mark.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let source = source.clone(); let value = value.clone(); let wanted = wanted.clone(); py_func("lambda", move |args: &[Py]| { let celsius = celsius.clone(); let clean = clean.clone(); let convert = convert.clone(); let fahrenheit = fahrenheit.clone(); let mark = mark.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let source = source.clone(); let value = value.clone(); let wanted = wanted.clone(); let f: Py = __kwp(args, 0, "f").unwrap_or(Py::None); op_truediv(&op_mul(&op_sub(&f.clone(), &Py::Int(32)), &Py::Int(5)), &Py::Int(9)) }) }])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    pattern = __unpk2[0].clone();
    wanted = __unpk2[1].clone();
    convert = __unpk2[2].clone();
    match_ = stdlib_call("re", "search", &[pattern.clone(), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&match_.clone()) {
    value = call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    source = if py_truthy(&Py::Bool(py_eq(&wanted.clone(), &py_str("F")))) { py_str("°C") } else { py_str("°F") };
    if py_truthy(&Py::Bool(py_eq(&wanted.clone(), &py_str("C")))) {
    let __unpk3 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GAS_OVEN()); __kws.extend(vec![("key".to_string(), { let celsius = celsius.clone(); let clean = clean.clone(); let convert = convert.clone(); let fahrenheit = fahrenheit.clone(); let mark = mark.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let source = source.clone(); let value = value.clone(); let wanted = wanted.clone(); py_func("lambda", move |args: &[Py]| { let celsius = celsius.clone(); let clean = clean.clone(); let convert = convert.clone(); let fahrenheit = fahrenheit.clone(); let mark = mark.clone(); let match_ = match_.clone(); let pattern = pattern.clone(); let source = source.clone(); let value = value.clone(); let wanted = wanted.clone(); let row: Py = __kwp(args, 0, "row").unwrap_or(Py::None); call_builtin("abs", &[op_sub(&geti(&row.clone(), &Py::Int(2)), &value.clone())]).unwrap_or_else(|e| exc::raise(e)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 3, None);
    mark = __unpk3[0].clone();
    celsius = __unpk3[1].clone();
    fahrenheit = __unpk3[2].clone();
    if py_truthy(&op_le(&call_builtin("abs", &[op_sub(&fahrenheit.clone(), &value.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(12))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[value.clone()]), -1, "")); __s.push_str(&fstr(&source.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_fmt(&[py_call(&convert, &[value.clone()])]), -1, "")); __s.push_str("°C - gas mark "); __s.push_str(&fstr(&mark.clone(), -1, "")); __s.push_str(", which most recipes call "); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C."); py_string(__s) };
    }
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[value.clone()]), -1, "")); __s.push_str(&fstr(&source.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_fmt(&[py_call(&convert, &[value.clone()])]), -1, "")); __s.push_str("°"); __s.push_str(&fstr(&wanted.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _temperature_rule(args: &[Py]) -> Py {
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
    let mut celsius: Py = Py::None;
    let mut fahrenheit: Py = Py::None;
    let _ = py_str("\"how do i convert celsius to fahrenheit\": the formula, since no number is given.");
    if py_truthy(&mcall(&g__NUMBER(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g__HOW_TO(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__BARE_TEMPERATURE(), "fullmatch", &[clean.clone()]) } }))) {
    return Py::None;
    }
    celsius = mcall(&g__C_ASKS(), "search", &[clean.clone()]);
    fahrenheit = mcall(&g__F_ASKS(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = celsius.clone(); if !py_truthy(&__b) { __b } else { fahrenheit.clone() } }) {
    if py_truthy(&op_lt(&mcall(&celsius.clone(), "start", &[]), &mcall(&fahrenheit.clone(), "start", &[]))) {
    return py_str("Multiply the Celsius by 9, divide by 5 and add 32: 20°C is 68°F. The quick way in your head is to double it and add 30, which is close enough for the weather.");
    }
    return py_str("Take away 32, then multiply by 5 and divide by 9: 68°F is 20°C. The quick way in your head is to take away 30 and halve it.");
    }
    return Py::None;
    Py::None
}
pub fn _mark_for(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut celsius: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "celsius") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: celsius"))) };
    let _ = py_str("The gas mark nearest a temperature in °C, the way a recipe would round it.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_GAS_OVEN()); __kws.extend(vec![("key".to_string(), { let celsius = celsius.clone(); py_func("lambda", move |args: &[Py]| { let celsius = celsius.clone(); let row: Py = __kwp(args, 0, "row").unwrap_or(Py::None); call_builtin("abs", &[op_sub(&geti(&row.clone(), &Py::Int(1)), &celsius.clone())]).unwrap_or_else(|e| exc::raise(e)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("min", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _oven(args: &[Py]) -> Py {
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
    let mut asking: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut celsius: Py = Py::None;
    let mut fahrenheit: Py = Py::None;
    let mut fan_asked: Py = Py::None;
    let mut gas_asked: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut mark_said: Py = Py::None;
    let mut number: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Gas marks and fan ovens, so one table answers every way an oven heat is asked for.\n\n    \"what gas mark is 200c\", \"180c in a fan oven\" and \"gas mark 6 in celsius\" all name the\n    same oven; plain °C to °F stays in _temperature.\n    ");
    mark_said = stdlib_call("re", "search", &[py_str("gas\\s*mark\\s*(\\d)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    fan_asked = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bfan\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    gas_asked = call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bgas\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    number = stdlib_call("re", "search", &[py_str("(-?\\d{2,3})\\s*(?:degrees?\\s*|°\\s*)?(c|celsius|centigrade|fan|f|fahrenheit)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mark_said.clone()) {
    mark = call_builtin("int", &[mcall(&mark_said.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    row = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GAS_OVEN()) { let mut row: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&geti(&row.clone(), &Py::Int(0)), &mark.clone()))) {  __out.push(row.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&row.clone()))) {
    return Py::None;
    }
    return { let mut __s = String::new(); __s.push_str("Gas mark "); __s.push_str(&fstr(&mark.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&geti(&row.clone(), &Py::Int(1)), -1, "")); __s.push_str("°C ("); __s.push_str(&fstr(&geti(&row.clone(), &Py::Int(2)), -1, "")); __s.push_str("°F), or "); __s.push_str(&fstr(&op_sub(&geti(&row.clone(), &Py::Int(1)), &g_FAN_COOLER()), -1, "")); __s.push_str("°C in a fan oven."); py_string(__s) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&number.clone())); if !py_truthy(&__b) { __b } else { { let __b = fan_asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(how long|minutes?|hours?|mins)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    bare = stdlib_call("re", "search", &[py_str("(?<![\\d.])(\\d{2,3})(?![\\d.])"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    asking = stdlib_call("re", "search", &[py_str("\\b(what|which|how|convert|equivalent|set|change|turn|use)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { { let __b = asking.clone(); if !py_truthy(&__b) { __b } else { { let __cmp11: Py = call_builtin("float", &[mcall(&bare.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&op_le(&Py::Int(90), &__cmp11)) { op_le(&__cmp11, &Py::Int(280)) } else { Py::Bool(false) } } } } } }) {
    celsius = call_builtin("float", &[mcall(&bare.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C is "); __s.push_str(&fstr(&_fmt(&[op_sub(&celsius.clone(), &g_FAN_COOLER())]), -1, "")); __s.push_str("°C in a fan oven, gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[celsius.clone()]), &Py::Int(0)), -1, "")); __s.push_str("."); py_string(__s) };
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    return Py::None;
    }
    celsius = call_builtin("float", &[mcall(&number.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    fahrenheit = Py::Bool(py_contains(&py_tuple(vec![py_str("f"), py_str("fahrenheit")]), &mcall(&number.clone(), "group", &[Py::Int(2)])));
    if py_truthy(&{ let __b = fahrenheit.clone(); if !py_truthy(&__b) { __b } else { { let __b = fan_asked.clone(); if py_truthy(&__b) { __b } else { gas_asked.clone() } } } }) {
    celsius = op_truediv(&op_mul(&op_sub(&celsius.clone(), &Py::Int(32)), &Py::Int(5)), &Py::Int(9));
    said = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&number.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str("°F is "); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C"); py_string(__s) };
    if py_truthy(&fan_asked.clone()) {
    let __aug2 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(", or "); __s.push_str(&fstr(&_fmt(&[op_sub(&celsius.clone(), &g_FAN_COOLER())]), -1, "")); __s.push_str("°C in a fan oven"); py_string(__s) });
    said = __aug2;
    }
    let __aug3 = op_add(&said.clone(), &{ let mut __s = String::new(); __s.push_str(", gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[celsius.clone()]), &Py::Int(0)), -1, "")); py_string(__s) });
    said = __aug3;
    return op_add(&said.clone(), &py_str("."));
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&number.clone(), "group", &[Py::Int(2)]), &py_str("fan"))); if !py_truthy(&__b) { __b } else { gas_asked.clone() } }) {
    plain = op_add(&celsius.clone(), &g_FAN_COOLER());
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C on a fan setting is "); __s.push_str(&fstr(&_fmt(&[plain.clone()]), -1, "")); __s.push_str("°C in a plain oven, gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[plain.clone()]), &Py::Int(0)), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&gas_asked.clone()) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\d\\s*(?:degrees?\\s*|°\\s*)?(f|fahrenheit)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    celsius = op_truediv(&op_mul(&op_sub(&celsius.clone(), &Py::Int(32)), &Py::Int(5)), &Py::Int(9));
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&number.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str("°F is gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[celsius.clone()]), &Py::Int(0)), -1, "")); __s.push_str(", and "); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C in a plain oven."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C is gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[celsius.clone()]), &Py::Int(0)), -1, "")); __s.push_str(", and "); __s.push_str(&fstr(&_fmt(&[op_sub(&celsius.clone(), &g_FAN_COOLER())]), -1, "")); __s.push_str("°C in a fan oven."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&number.clone(), "group", &[Py::Int(2)]), &py_str("fan")))) {
    plain = op_add(&celsius.clone(), &g_FAN_COOLER());
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C on a fan setting is "); __s.push_str(&fstr(&_fmt(&[plain.clone()]), -1, "")); __s.push_str("°C in a plain oven, gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[plain.clone()]), &Py::Int(0)), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&fan_asked.clone()) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[celsius.clone()]), -1, "")); __s.push_str("°C is "); __s.push_str(&fstr(&_fmt(&[op_sub(&celsius.clone(), &g_FAN_COOLER())]), -1, "")); __s.push_str("°C in a fan oven, gas mark "); __s.push_str(&fstr(&geti(&_mark_for(&[celsius.clone()]), &Py::Int(0)), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _butter(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut count: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("A stick of butter: a weight cooks name instead of giving grams.");
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&clean.clone(), &py_str("butter"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bsticks?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(grams?|g|oz|ounces?|cups?|tablespoons?|tbsp|teaspoons?)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:how (?:much|many)|what is|whats|what's|weigh\\w*)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    count = stdlib_call("re", "search", &[py_str("(half an?|a|an|one|\\d+(?:\\.\\d+)?)\\s+sticks?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bpounds?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bcups?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("There are four sticks of butter in a pound, 450 g, so a stick is a quarter of one.");
    }
    if py_truthy(&Py::Bool(py_contains(&clean.clone(), &py_str("block")))) {
    return py_str("A stick is 113 g, so a 250 g block is about two and a quarter sticks; butter sold in sticks is usually four to a 450 g block.");
    }
    said = if py_truthy(&count.clone()) { mcall(&count.clone(), "group", &[Py::Int(1)]) } else { py_str("a") };
    amount = if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("half")])) { Py::Float(0.5) } else { _value(&[said.clone()]) };
    if py_truthy(&Py::Bool(py_is(&amount.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bcups?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bgrams?\\b|\\bg\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_str("One stick of butter is half a cup, which is 8 tablespoons or about 113 g.");
    }
    if py_truthy(&Py::Bool(py_eq(&amount.clone(), &Py::Int(1)))) {
    return py_str("A stick of butter is about 113 g, which is half a cup or 8 tablespoons.");
    }
    if py_truthy(&Py::Bool(py_eq(&amount.clone(), &Py::Float(0.5)))) {
    return py_str("Half a stick of butter is about 57 g, which is 4 tablespoons.");
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[amount.clone()]), -1, "")); __s.push_str(" sticks of butter is about "); __s.push_str(&fstr(&_fmt(&[op_mul(&amount.clone(), &Py::Float(113.4))]), -1, "")); __s.push_str(" g."); py_string(__s) };
    Py::None
}
pub fn _pack(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut best: Py = Py::None;
    let mut container: Py = Py::None;
    let mut drained: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut name: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("A tin, block or carton: the amount people buy instead of weighing it out.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__PACK_WORD(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PACK_ASK(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:cost|costs|costing|price|prices|cheap|cheaper|expensive)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_PACK_GRAMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    grams = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&name.clone(), &py_str("cream"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("ice cream"))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), grams.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&best.clone(), 2, None);
    name = __unpk3[0].clone();
    grams = __unpk3[1].clone();
    container = mcall(&mcall(&g__PACK_WORD(), "search", &[clean.clone()]), "group", &[Py::Int(0)]);
    if py_truthy(&mcall(&container.clone(), "endswith", &[py_str("s")])) {
    container = slice_or_raise(&container.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    amount = if py_truthy(&Py::Bool(py_contains(&g_PACK_IN_ML(), &name.clone()))) { { let mut __s = String::new(); __s.push_str(&fstr(&grams.clone(), -1, "")); __s.push_str(" ml"); py_string(__s) } } else { _kilos(&[grams.clone()]) };
    reply = { let mut __s = String::new(); __s.push_str("A standard "); __s.push_str(&fstr(&container.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&amount.clone(), -1, "")); py_string(__s) };
    drained = mcall(&g_PACK_DRAINED(), "get", &[name.clone()]);
    if py_truthy(&drained.clone()) {
    let __aug4 = op_add(&reply.clone(), &{ let mut __s = String::new(); __s.push_str(", about "); __s.push_str(&fstr(&drained.clone(), -1, "")); __s.push_str(" g once drained"); py_string(__s) });
    reply = __aug4;
    }
    return op_add(&reply.clone(), &py_str("."));
    Py::None
}
pub fn _singular(args: &[Py]) -> Py {
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
    let mut _g: Py = Py::None;
    let mut ending: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut key: Py = Py::None;
    let mut swap: Py = Py::None;
    let _ = py_str("The one of a food the weight tables are keyed by: \"potatoes\" -> \"potato\".");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("ies"), py_str("y")]), py_tuple(vec![py_str("oes"), py_str("o")]), py_tuple(vec![py_str("ches"), py_str("ch")]), py_tuple(vec![py_str("shes"), py_str("sh")]), py_tuple(vec![py_str("ses"), py_str("s")]), py_tuple(vec![py_str("es"), py_str("")]), py_tuple(vec![py_str("s"), py_str("")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    ending = __unpk2[0].clone();
    swap = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&name.clone(), "endswith", &[ending.clone()]); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    gone = op_add(&slice_or_raise(&name.clone(), &Py::None, &op_neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None), &swap.clone());
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PIECE_GRAMS()) { let __unpk1639 = py_unpack(&__item.clone(), 2, None); let mut key: Py = __unpk1639[0].clone(); let mut _g: Py = __unpk1639[1].clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![key.clone(), _plural_of(&[key.clone()])]), &gone.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return gone.clone();
    }
    }
    }
    return { let __b = mcall(&name.clone(), "rstrip", &[py_str("s")]); if py_truthy(&__b) { __b } else { name.clone() } };
    Py::None
}
pub fn _table_grams(args: &[Py]) -> Py {
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
    let mut table: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: table"))) };
    let mut best: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut key: Py = Py::None;
    let _ = py_str("The row for this food in a weight table, singular or plural, or None.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&table.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    grams = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![key.clone(), _plural_of(&[key.clone()])]), &name.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[key.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![key.clone(), grams.clone()]);
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _pack_count(args: &[Py]) -> Py {
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
    let mut box_: Py = Py::None;
    let mut content: Py = Py::None;
    let mut count: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut packed: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut said: Py = Py::None;
    let mut singular: Py = Py::None;
    let mut size: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("How many pieces the packet holds, from the packet's weight and one piece's weight.");
    found = mcall(&g__COUNT_ASK(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    piece = mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("piece")]), "split", &[])]);
    box_ = mcall(&found.clone(), "group", &[py_str("box")]);
    if py_truthy(&mcall(&g__NOT_A_COUNT(), "match", &[piece.clone()])) {
    return Py::None;
    }
    counted = { let __b = mcall(&g_COUNTED_PACKS(), "get", &[piece.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_COUNTED_PACKS(), "get", &[mcall(&piece.clone(), "rstrip", &[py_str("s")])]) } };
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    words = mcall(&{ let __b = mcall(&found.clone(), "group", &[py_str("of")]); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    size = __item.clone();
    said = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &size.clone(), &Py::None)]);
    counted = { let __b = mcall(&g_COUNTED_PACKS(), "get", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_COUNTED_PACKS(), "get", &[_singular(&[said.clone()])]) } };
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    }
    let __unpk2 = py_unpack(&mcall(&g__COUNT_CONTENT(), "get", &[piece.clone(), py_tuple(vec![piece.clone(), piece.clone()])]), 2, None);
    content = __unpk2[0].clone();
    name = __unpk2[1].clone();
    if py_truthy(&mcall(&g__COUNT_CONTENT(), "get", &[piece.clone()])) {
    singular = name.clone();
    } else {
    singular = _singular(&[name.clone()]);
    }
    packed = _table_grams(&[content.clone(), g_PACK_GRAMS()]);
    one = { let __b = _table_grams(&[singular.clone(), g_PIECE_GRAMS()]); if py_truthy(&__b) { __b } else { _table_grams(&[name.clone(), g_PIECE_GRAMS()]) } };
    if py_truthy(&{ let __b = { let __b = Py::Bool(!py_truthy(&packed.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&one.clone())) } }; if !py_truthy(&__b) { __b } else { words.clone() } }) {
    let __iter3 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter3 {
    size = __item.clone();
    other = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &size.clone(), &Py::None)]);
    if py_truthy(&_table_grams(&[_singular(&[other.clone()]), g_PIECE_GRAMS()])) {
    packed = { let __b = _table_grams(&[_singular(&[other.clone()]), g_PACK_GRAMS()]); if py_truthy(&__b) { __b } else { packed.clone() } };
    one = _table_grams(&[_singular(&[other.clone()]), g_PIECE_GRAMS()]);
    break;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&packed.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&one.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&geti(&one.clone(), &Py::Int(1)))) } } } }) {
    return Py::None;
    }
    count = call_builtin("int", &[call_builtin("round", &[op_truediv(&geti(&packed.clone(), &Py::Int(1)), &geti(&one.clone(), &Py::Int(1)))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&count.clone(), &Py::Int(2))) {
    return Py::None;
    }
    said = _plural_of(&[geti(&one.clone(), &Py::Int(0))]);
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" in a "); __s.push_str(&fstr(&box_.clone(), -1, "")); __s.push_str(": a "); __s.push_str(&fstr(&box_.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_kilos(&[geti(&packed.clone(), &Py::Int(1))]), -1, "")); __s.push_str(" and one "); __s.push_str(&fstr(&geti(&one.clone(), &Py::Int(0)), -1, "")); __s.push_str(" about "); __s.push_str(&fstr(&_fmt(&[geti(&one.clone(), &Py::Int(1))]), -1, "")); __s.push_str("g, so the count moves with the size of both."); py_string(__s) };
    Py::None
}
pub fn _in_a_vessel(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut found: Py = Py::None;
    let mut liquid: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut size: Py = Py::None;
    let mut vessel: Py = Py::None;
    found = mcall(&g__IN_A_VESSEL(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[py_str("liquid")]), mcall(&found.clone(), "group", &[py_str("vessel")])]), 2, None);
    liquid = __unpk1[0].clone();
    vessel = __unpk1[1].clone();
    size = mcall(&found.clone(), "group", &[py_str("size")]);
    if py_truthy(&Py::Bool(py_eq(&vessel.clone(), &py_str("glass")))) {
    let __iter2 = py_iter_or_raise(&g_LOOSE());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&phrase.clone(), &{ let mut __s = String::new(); __s.push_str("glass of "); __s.push_str(&fstr(&liquid.clone(), -1, "")); py_string(__s) }))) {
    return reply.clone();
    }
    }
    }
    amount = geti(&g__VESSEL_ML(), &vessel.clone());
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("big"), py_str("large"), py_str("tall")]), &size.clone()))) {
    amount = Py::Int(250);
    }
    return op_add(&op_add(&{ let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&vessel.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&liquid.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" ml"); py_string(__s) }, &if py_truthy(&{ let __b = Py::Bool(py_eq(&liquid.clone(), &py_str("water"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&vessel.clone(), &py_str("glass"))) } }) { py_str(", which is 200 g") } else { py_str("") }), &py_str("."));
    Py::None
}
pub fn _loose(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut in_vessel: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("A knob, a squeeze, a shot or a glass: the amount the word stands for.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__LOOSE_ASK(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g__LOOSE_NOT(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    in_vessel = _in_a_vessel(&[clean.clone()]);
    if py_truthy(&in_vessel.clone()) {
    return in_vessel.clone();
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_LOOSE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![phrase.clone(), reply.clone()]);
    }
    }
    }
    return if py_truthy(&best.clone()) { geti(&best.clone(), &Py::Int(1)) } else { Py::None };
    Py::None
}
pub fn _liquid_weight(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut liquid: Py = Py::None;
    let mut millilitres: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("What a volume of a named liquid weighs: \"a pint of milk\", \"a litre of water\".");
    if py_truthy(&mcall(&g__PER_CUP(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_asks_for_weight(&[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:cost|costs|price|prices|worth|sell|buy|buying|calories|protein)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    liquid = Py::None;
    let __iter1 = py_iter_or_raise(&g_LIQUID_GRAMS_LITRE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    grams = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    liquid = py_tuple(vec![name.clone(), grams.clone()]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&liquid.clone()))) {
    return Py::None;
    }
    let __unpk3 = py_unpack(&py_tuple(vec![Py::None, py_str("")]), 2, None);
    millilitres = __unpk3[0].clone();
    said = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&_mentions(&[clean.clone()]));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 4, None);
    start = __unpk5[0].clone();
    end = __unpk5[1].clone();
    family = __unpk5[2].clone();
    unit = __unpk5[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&family.clone(), &py_str("volume"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("cup"), py_str("tbsp"), py_str("tsp")]), &unit.clone())) } }) {
    continue;
    }
    value = _number_before(&[clean.clone(), start.clone()]);
    if py_truthy(&Py::Bool(py_is(&value.clone(), &Py::None))) {
    continue;
    }
    millilitres = op_mul(&value.clone(), &_factor(&[py_str("volume"), unit.clone()]));
    said = if py_truthy(&Py::Bool(py_eq(&value.clone(), &Py::Int(1)))) { { let mut __s = String::new(); __s.push_str("a "); __s.push_str(&fstr(&mcall(&g_SAID_ONE(), "get", &[unit.clone(), unit.clone()]), -1, "")); py_string(__s) } } else { _amount_phrase(&[value.clone(), unit.clone()]) };
    break;
    }
    if py_truthy(&Py::Bool(py_is(&millilitres.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bmugs?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __unpk6 = py_unpack(&py_tuple(vec![Py::Float(250.0), py_str("a mug")]), 2, None);
    millilitres = __unpk6[0].clone();
    said = __unpk6[1].clone();
    }
    grams = op_truediv(&op_mul(&millilitres.clone(), &geti(&liquid.clone(), &Py::Int(1))), &Py::Float(1000.0));
    return { let mut __s = String::new(); __s.push_str(&fstr(&op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&geti(&liquid.clone(), &Py::Int(0)), -1, "")); __s.push_str(" weighs "); __s.push_str(&fstr(&_mass_said(&[grams.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _mass_said(args: &[Py]) -> Py {
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
    let mut kilos: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A weight said the way someone would: 1 kg, 1.03 kg, 585 g.");
    if py_truthy(&op_lt(&grams.clone(), &Py::Int(1000))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_grams(&[grams.clone()]), -1, "")); __s.push_str(" g"); py_string(__s) };
    }
    kilos = op_truediv(&grams.clone(), &Py::Float(1000.0));
    text = if py_truthy(&op_lt(&kilos.clone(), &Py::Int(2))) { { let mut __s = String::new(); __s.push_str(&fstr(&kilos.clone(), -1, &".2f")); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str(&fstr(&kilos.clone(), -1, &",.1f")); py_string(__s) } };
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&mcall(&text.clone(), "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]), -1, "")); __s.push_str(" kg"); py_string(__s) };
    Py::None
}
pub fn _a_dish_for_a_table(args: &[Py]) -> Py {
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
    let mut _name: Py = Py::None;
    let mut all_of_it: Py = Py::None;
    let mut allowance: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut each: Py = Py::None;
    let mut found: Py = Py::None;
    let mut main: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut part: Py = Py::None;
    let mut people: Py = Py::None;
    let mut per_head: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut side: Py = Py::None;
    let mut spice: Py = Py::None;
    let mut tin: Py = Py::None;
    let mut tins: Py = Py::None;
    let mut tomatoes: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("How much of a named dish to cook for the number of people the line names.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clean.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_TIME_IS_THE_ASK(), "search", &[clean.clone()])) {
    return Py::None;
    }
    people = _people_asked(&[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&people.clone())); if py_truthy(&__b) { __b } else { { let __b = op_lt(&people.clone(), &Py::Int(2)); if py_truthy(&__b) { __b } else { op_gt(&people.clone(), &Py::Int(60)) } } } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&clean.clone(), "lower", &[]), "split", &[])]);
    named = Py::None;
    let __iter1 = py_iter_or_raise(&g_DISHES_FOR_A_TABLE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 5, None);
    names = __unpk2[0].clone();
    main = __unpk2[1].clone();
    side = __unpk2[2].clone();
    tomatoes = __unpk2[3].clone();
    spice = __unpk2[4].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    found = stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])"); py_string(__s) }, said.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    if py_truthy(&mcall(&g__A_DISH_PART(), "match", &[slice_or_raise(&said.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&named.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&named.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    named = py_tuple(vec![name.clone(), main.clone(), side.clone(), tomatoes.clone(), spice.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&named.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk4 = py_unpack(&named.clone(), 5, None);
    _name = __unpk4[0].clone();
    main = __unpk4[1].clone();
    side = __unpk4[2].clone();
    tomatoes = __unpk4[3].clone();
    spice = __unpk4[4].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("chilli"), py_str("chili")]), &_name.clone())); if !py_truthy(&__b) { __b } else { mcall(&g__CHILLI_THE_VEGETABLE(), "search", &[said.clone()]) } }) {
    return Py::None;
    }
    found = py_list(vec![]);
    let __iter5 = py_iter_or_raise(&py_tuple(vec![main.clone(), side.clone()]));
    for __item in __iter5 {
    part = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&part.clone()))) {
    continue;
    }
    allowance = _an_allowance(&[part.clone()]);
    if py_truthy(&allowance.clone()) {
    let _ = mcall(&found.clone(), "append", &[allowance.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    each = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk8157 = py_unpack(&__item.clone(), 2, None); let mut amount: Py = __unpk8157[0].clone(); let mut noun: Py = __unpk8157[1].clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g of "); __s.push_str(&fstr(&noun.clone(), -1, "")); py_string(__s) }); } py_list(__out) })() };
    total = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk7197 = py_unpack(&__item.clone(), 2, None); let mut amount: Py = __unpk7197[0].clone(); let mut noun: Py = __unpk7197[1].clone(); __out.push({ let mut __s = String::new(); __s.push_str(&fstr(&_a_weight(&[op_mul(&amount.clone(), &people.clone())]), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); py_string(__s) }); } py_list(__out) })() };
    dish = _name.clone();
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[each.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    per_head = geti(&each.clone(), &Py::Int(0));
    all_of_it = geti(&total.clone(), &Py::Int(0));
    } else {
    per_head = mcall(&py_str(" and "), "join", &[each.clone()]);
    all_of_it = mcall(&py_str(" and "), "join", &[total.clone()]);
    }
    reply = { let mut __s = String::new(); __s.push_str("For "); __s.push_str(&fstr(&people.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&dish.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&per_head.clone(), -1, "")); __s.push_str(" each: "); __s.push_str(&fstr(&all_of_it.clone(), -1, "")); __s.push_str(" in all."); py_string(__s) };
    if py_truthy(&tomatoes.clone()) {
    tins = call_builtin("max", &[Py::Int(1), call_builtin("round", &[op_truediv(&people.clone(), &g__TIN_FOR())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    tin = if py_truthy(&Py::Bool(py_eq(&tins.clone(), &Py::Int(1)))) { py_str("tin") } else { py_str("tins") };
    let __aug6 = op_add(&reply.clone(), &{ let mut __s = String::new(); __s.push_str(" One 400 g tin of tomatoes does four, so "); __s.push_str(&fstr(&tins.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&tin.clone(), -1, "")); __s.push_str("."); py_string(__s) });
    reply = __aug6;
    }
    if py_truthy(&spice.clone()) {
    let __aug7 = op_add(&reply.clone(), &py_str(" Use a little more spice than the recipe says - a big batch tastes milder."));
    reply = __aug7;
    }
    return reply.clone();
    Py::None
}
pub fn _an_allowance(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut foods: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut noun: Py = Py::None;
    let _ = py_str("The per-person allowance for one part of a dish, from the one table of allowances.");
    let __iter1 = py_iter_or_raise(&op_add(&g_PER_PERSON(), &g_PER_PERSON_WHEN_COUNTED()));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    foods = __unpk2[0].clone();
    amount = __unpk2[1].clone();
    kind = __unpk2[2].clone();
    noun = __unpk2[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("g"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&amount.clone())) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&foods.clone(), &name.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_tuple(vec![amount.clone(), noun.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_weight(args: &[Py]) -> Py {
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
    let mut kilos: Py = Py::None;
    let _ = py_str("A weight the way a cook says it: kilos past a thousand, with halves kept.");
    if py_truthy(&op_ge(&grams.clone(), &Py::Int(1000))) {
    kilos = op_truediv(&call_builtin("round", &[op_truediv(&grams.clone(), &Py::Float(100.0))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10));
    return { let mut __s = String::new(); __s.push_str(&fstr(&kilos.clone(), -1, &"g")); __s.push_str(" kg"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&grams.clone(), -1, &"g")); __s.push_str(" g"); py_string(__s) };
    Py::None
}
pub fn people_in_groups(args: &[Py]) -> Py {
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
    let mut raw: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("How many people the groups named in the line add up to, or None when it names none.\n\n    Every group counts: \"4 adults and 3 kids\" is seven, \"3 kids and 2 adults\" is five, and a\n    line with one group (\"for 4 people\", \"6 kids\") is that group. A child is a person at the\n    table too, so the total is what the shopping and the pan are for - the note that children eat\n    a little less belongs in the answer, not in a count that quietly leaves them out.\n    ");
    total = Py::Int(0);
    let __iter1 = py_iter_or_raise(&mcall(&g__TABLE_GROUPS(), "findall", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]));
    for __item in __iter1 {
    raw = __item.clone();
    count = if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) { call_builtin("int", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__GROUP_VALUE(), "get", &[raw.clone()]) };
    if py_truthy(&count.clone()) {
    let __aug2 = op_add(&total.clone(), &count.clone());
    total = __aug2;
    }
    }
    return { let __b = total.clone(); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _people_asked(args: &[Py]) -> Py {
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
    let mut grouped: Py = Py::None;
    let mut part: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("How many people the line cooks for, or None when it only says \"a person\".");
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bbetween\\s+us\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Int(2);
    }
    grouped = people_in_groups(&[clean.clone()]);
    found = mcall(&g__PEOPLE_COUNT(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return grouped.clone();
    }
    raw = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "groups", &[])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }, py_str("")]).unwrap_or_else(|e| exc::raise(e));
    said = if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) { call_builtin("int", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__PEOPLE_VALUE(), "get", &[raw.clone()]) };
    if py_truthy(&{ let __b = grouped.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { op_gt(&grouped.clone(), &said.clone()) } } } }) {
    return grouped.clone();
    }
    return said.clone();
    Py::None
}
pub fn _people_before(args: &[Py]) -> Py {
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
    let mut statements_only: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "statements_only") { v.clone() }
    else { Py::Bool(false) };
    let mut asked_for_an_amount: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "asked_for_an_amount") { v.clone() }
    else { Py::Bool(false) };
    let mut earlier: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut mem: Py = Py::None;
    let mut older: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("A headcount from the last turn or two: \"we are 6 on saturday\", then \"how much mince\".\n\n    The number came one line earlier and the portion question is the same question with the food\n    swapped in, so the count comes with it. Only lines that talk about feeding people count:\n    \"3 pans on the go\" and \"a bit for 10 minutes\" are not a table.\n\n    `statements_only` leaves out a count that came inside an earlier portion question: the\n    answer to \"how much mince for 4 people\" does not make the next line a table of four, and\n    \"how much rice per person\", asked after it, is still the question about one person.\n\n    A correction of the count is read as the count, even though it has no word for the people in\n    it: \"i want to cook a curry for 4 people\" then \"actually make it for 8\" left the table at\n    four, and the portion answer said \"About 75 g of dry rice each, so 300 g for four.\" to\n    somebody who had just said eight. The words that make a line a table can sit on either of\n    the two lines - the one that named the people or the one that corrected the figure - so the\n    test is made over both, and then the newest figure in them is the count.\n\n    A figure with no people word next to it is only a table when it is said as a correction\n    (`CORRECTING`): \"put it in the oven for 8\" and \"i need it for 8\" are a time and a date, and\n    a table of eight they never gave is the one thing a portion must not do.\n\n    A count said as a statement about the table stands for a few turns, not one: \"there will be\n    nine of us\", \"how long should it cook for\", \"my oven is fan assisted\", \"so its eleven now\",\n    \"should i change the time\", \"and how much meat should i buy\" - and the meat question got a\n    table of two, because eleven was three lines back and the two lines read held no count. So\n    when the last two lines say nothing, the turns before them are read for a count that still\n    stands, and a line that names people with no number ends the search: the subject has moved\n    on, and an older table is the one thing a portion must not do. Only a line that asks how much\n    or how many reaches back at all - \"what size chicken do i need\" is a question about the bird,\n    not about how many are eating it.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __try1 = exc::catch(|| {
    mem = pyrt::xrefs::xmodule("memory");
    lines = mcall(&mem.clone(), "last_user", &[memory.clone(), Py::Int(2)]);
    earlier = mcall(&mem.clone(), "last_user", &[memory.clone(), Py::Int(6)]);
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
    if py_truthy(&call_builtin("isinstance", &[lines.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    lines = py_list(vec![lines.clone()]);
    }
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = lines.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut line: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])); } py_list(__out) })() };
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut text: Py = __item.clone(); if py_truthy(&{ let __b = text.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = statements_only.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__AN_ASK(), "search", &[text.clone()]) } })) } }) {  __out.push(text.clone()); } } py_list(__out) })() };
    let __iter6 = py_iter_or_raise(&call_builtin("reversed", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter6 {
    text = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = mcall(&g__PEOPLE_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PEOPLE_COUNT(), "search", &[text.clone()]) } })); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[text.clone()]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_a_table_without_people(&[text.clone()])))) {
    continue;
    }
    found = _people_asked(&[text.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_CORRECTING(), "search", &[text.clone()]) } }) {
    found = _a_corrected_figure(&[text.clone()]);
    }
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    }
    if py_truthy(&{ let __b = statements_only.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asked_for_an_amount.clone())) } }) {
    return Py::None;
    }
    older = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = earlier.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut line: Py = __item.clone(); __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])); } py_list(__out) })() }, &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    return _a_count_that_still_stands(&[older.clone()]);
    Py::None
}
pub fn _a_count_that_still_stands(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut older: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "older") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: older"))) };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The newest headcount in the turns before the last two, while it still holds.\n\n    People state a table once and then ask two or three things about the plan around it - the\n    time, the oven, the meat - and the count said between those questions is still the table.\n    Only a statement is read: a count inside an earlier portion question is an ask, and \"how much\n    mince for 4 people\" three lines back is not a table for the rice that follows it. A line that\n    names the people and gives no number ends it, because that is the subject moving on.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[older.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    text = __item.clone();
    if py_truthy(&mcall(&g__AN_ASK(), "search", &[text.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g__PEOPLE_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PEOPLE_COUNT(), "search", &[text.clone()]) } }) {
    found = _people_asked(&[text.clone()]);
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    return Py::None;
    }
    if py_truthy(&mcall(&g_CORRECTING(), "search", &[text.clone()])) {
    found = { let __b = _people_asked(&[text.clone()]); if py_truthy(&__b) { __b } else { _a_corrected_figure(&[text.clone()]) } };
    if py_truthy(&found.clone()) {
    return found.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_corrected_figure(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut raw: Py = Py::None;
    let _ = py_str("A bare figure in a line that corrects something: \"so its eleven now\".\n\n    A correction of the count says the number without a word for the people in it, which is how\n    people say it out loud: \"oh actually my sister is coming with her two kids so its eleven now\".\n    The line has to be a correction and the figure has to be a count, so a weight or a time after\n    it is not one (`NOT_A_TABLE`): \"put it in the oven for 8\" is still a time, and \"its 5 in the\n    afternoon\" is a clock.\n    ");
    if py_truthy(&mcall(&g__A_CLOCK(), "search", &[text.clone()])) {
    return Py::None;
    }
    found = mcall(&g__A_FIGURE(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    raw = mcall(&found.clone(), "group", &[Py::Int(1)]);
    return if py_truthy(&mcall(&raw.clone(), "isdigit", &[])) { call_builtin("int", &[raw.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { mcall(&g__PEOPLE_VALUE(), "get", &[raw.clone()]) };
    Py::None
}
pub fn _a_table_without_people(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut found: Py = Py::None;
    let _ = py_str("True when a line with no word for people still says how many it cooks for.\n\n    \"a lasagne for six\" is a table; \"i need it for 8\" and \"put it in the oven for 8\" are a date\n    and a time, and the proof is the word before \"for\": a dish, not a pronoun or an oven.\n    ");
    if py_truthy(&{ let __b = mcall(&g__PEOPLE_WORD(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CORRECTING(), "search", &[text.clone()]) } }) {
    return Py::Bool(true);
    }
    found = mcall(&g__BARE_FOR_COUNT(), "search", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::Bool(false);
    }
    before = mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&before.clone()))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_contains(&g__NOT_A_TABLE_BEFORE(), &mcall(&mcall(&geti(&before.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?;:")]), "lower", &[])));
    Py::None
}
pub fn _per_person(args: &[Py]) -> Py {
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
    let mut _name: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut best: Py = Py::None;
    let mut bottles: Py = Py::None;
    let mut bunch: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut people: Py = Py::None;
    let mut per: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("How much of a food a person gets, when the line asks to cook for a number of people.\n\n    \"how many potatoes per person\" is not a conversion and holds no measure to convert, so it\n    used to get the boiling method or a shrug. The reply gives the portion for one, and the total\n    only for a headcount the person actually said: a handed a number they never gave is the one\n    thing a portion must not do.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow (?:much|many|big|large)\\b|\\bwhat size\\b|\\bi (?:need|want)\\b|\\bi(?:'?m| am) (?:cooking|making)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:minutes?|mins?|hours?|hrs?|seconds?)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bi (?:need|want)\\s+(?:a|an|one|some)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__COMPONENT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PORTION_AVOID(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__BAKING_PORTION(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\begg"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    people = _people_asked(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&people.clone(), &Py::None))) {
    people = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("asked_for_an_amount".to_string(), call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\bhow (?:much|many)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _people_before(args))(&__args) };
    stated = { let __b = Py::Bool(!py_is(&people.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __kws.extend(vec![("statements_only".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _people_before(args))(&__args) }, &people.clone())) } };
    } else {
    stated = Py::Bool(true);
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&people.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PORTION_WORD(), "search", &[clean.clone()]))) } } } }) {
    let __iter1 = py_iter_or_raise(&g__WHOLE_PIECE());
    for __item in __iter1 {
    phrase = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&people.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PORTION_WORD(), "search", &[clean.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__PORTION_COUNT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&people.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()]))) } } } }) {
    return Py::None;
    }
    best = Py::None;
    counted = if py_truthy(&{ let __b = stated.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__CONTAINER(), "search", &[clean.clone()]))) } }) { g_PER_PERSON_WHEN_COUNTED() } else { py_tuple(vec![]) };
    let __iter2 = py_iter_or_raise(&op_add(&g_PER_PERSON(), &counted.clone()));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 4, None);
    names = __unpk3[0].clone();
    amount = __unpk3[1].clone();
    kind = __unpk3[2].clone();
    noun = __unpk3[3].clone();
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])(?!\\s+(?:dressing|sauce|dip|oil|mayo|mayonnaise|vinaigrette|juice))"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), amount.clone(), kind.clone(), noun.clone()]);
    }
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(!py_is(&people.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = stated.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__PORTION_WORD(), "search", &[clean.clone()]) } } } }) {
    return { let mut __s = String::new(); __s.push_str("A main is about 300 g a head, a plate of pasta or rice 75 to 100 g dry and a buffet six to eight pieces a guest, so for "); __s.push_str(&fstr(&_said_people(&[people.clone()]), -1, "")); __s.push_str(" plan on "); __s.push_str(&fstr(&_mass_of(&[op_mul(&Py::Int(300), &people.clone())]), -1, "")); __s.push_str(" of the main dish. Tell me what you are cooking and I will do the sum for the shopping."); py_string(__s) };
    }
    return Py::None;
    }
    let __unpk5 = py_unpack(&best.clone(), 4, None);
    _name = __unpk5[0].clone();
    amount = __unpk5[1].clone();
    kind = __unpk5[2].clone();
    noun = __unpk5[3].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MADE_NOT_MEASURED(), &noun.clone())); if !py_truthy(&__b) { __b } else { { let __b = stdlib_call("re", "search", &[py_str("\\bi (?:need|want)\\s+to\\b|\\bi(?:'?m| am) (?:cooking|making|baking|roasting|prepping)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhow (?:much|many|big|large)\\b|\\bwhat size\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__BIRDS(), &_name.clone())); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow (?:big|large)\\b|\\bwhat size\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let __unpk6 = py_unpack(&py_tuple(vec![g_BIRD_GRAMS(), _name.clone()]), 2, None);
    amount = __unpk6[0].clone();
    noun = __unpk6[1].clone();
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("each")))) {
    bunch = if py_truthy(&{ let __b = mcall(&noun.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&noun.clone(), &py_str(" "))) } }) { noun.clone() } else { op_add(&noun.clone(), &py_str("s")) };
    said = if py_truthy(&op_gt(&amount.clone(), &Py::Int(1))) { { let mut __s = String::new(); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&bunch.clone(), -1, "")); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str("1 "); __s.push_str(&fstr(&noun.clone(), -1, "")); py_string(__s) } };
    if py_truthy(&people.clone()) {
    total = if py_truthy(&op_gt(&amount.clone(), &Py::Int(1))) { { let mut __s = String::new(); __s.push_str(&fstr(&op_mul(&amount.clone(), &people.clone()), -1, &"g")); __s.push_str(" "); __s.push_str(&fstr(&bunch.clone(), -1, "")); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&g__SAID_PEOPLE(), "get", &[people.clone(), people.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&bunch.clone(), -1, "")); py_string(__s) } };
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" each, so "); __s.push_str(&fstr(&total.clone(), -1, "")); __s.push_str(" for "); __s.push_str(&fstr(&_said_people(&[people.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()])))) {
    return { let mut __s = String::new(); __s.push_str("A portion is about "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" each."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("bottle")))) {
    per = if py_truthy(&Py::Bool(py_eq(&amount.clone(), &Py::Float(0.5)))) { py_str("half a bottle") } else { { let mut __s = String::new(); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" bottles"); py_string(__s) } };
    if py_truthy(&Py::Bool(!py_truthy(&people.clone()))) {
    return { let mut __s = String::new(); __s.push_str("Allow about "); __s.push_str(&fstr(&per.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" a guest."); py_string(__s) };
    }
    bottles = call_builtin("int", &[stdlib_call("math", "ceil", &[op_mul(&amount.clone(), &people.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str("Allow about "); __s.push_str(&fstr(&per.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" a guest, so "); __s.push_str(&fstr(&bottles.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&if py_truthy(&Py::Bool(py_eq(&bottles.clone(), &Py::Int(1)))) { py_str("bottle") } else { py_str("bottles") }, -1, "")); __s.push_str(" for "); __s.push_str(&fstr(&_said_people(&[people.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("ml")))) {
    if py_truthy(&people.clone()) {
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" ml of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" each, so "); __s.push_str(&fstr(&_litres_of(&[op_mul(&amount.clone(), &people.clone())]), -1, "")); __s.push_str(" for "); __s.push_str(&fstr(&_said_people(&[people.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()])))) {
    return { let mut __s = String::new(); __s.push_str("A portion of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" ml."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str(" ml of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" each."); py_string(__s) };
    }
    if py_truthy(&people.clone()) {
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_grams(&[amount.clone()]), -1, "")); __s.push_str(" g of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" each, so "); __s.push_str(&fstr(&_mass_of(&[op_mul(&amount.clone(), &people.clone())]), -1, "")); __s.push_str(" for "); __s.push_str(&fstr(&_said_people(&[people.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()])))) {
    return { let mut __s = String::new(); __s.push_str("A portion of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_grams(&[amount.clone()]), -1, "")); __s.push_str(" g."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("About "); __s.push_str(&fstr(&_grams(&[amount.clone()]), -1, "")); __s.push_str(" g of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" each."); py_string(__s) };
    Py::None
}
pub fn _said_people(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut people: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "people") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: people"))) };
    let _ = py_str("A headcount the way it is said: \"for one\", \"for eight\", never \"for 1\".");
    return mcall(&g__SAID_PEOPLE(), "get", &[people.clone(), call_builtin("str", &[people.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _mass_of(args: &[Py]) -> Py {
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
    let mut kilos: Py = Py::None;
    let _ = py_str("A scaled-up portion the way a shopper reads it: 1 kg, not 1,000 g.");
    if py_truthy(&op_ge(&grams.clone(), &Py::Int(1000))) {
    kilos = mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&op_truediv(&grams.clone(), &Py::Int(1000)), -1, &".1f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&kilos.clone(), -1, "")); __s.push_str(" kg"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_grams(&[grams.clone()]), -1, "")); __s.push_str(" g"); py_string(__s) };
    Py::None
}
pub fn _litres_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ml: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ml") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ml"))) };
    let mut litres: Py = Py::None;
    let _ = py_str("A scaled-up pour the way a jug reads it: 2.4 litres, not 2,400 ml.");
    if py_truthy(&op_ge(&ml.clone(), &Py::Int(1000))) {
    litres = mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&op_truediv(&ml.clone(), &Py::Int(1000)), -1, &".1f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]);
    return { let mut __s = String::new(); __s.push_str(&fstr(&litres.clone(), -1, "")); __s.push_str(" litres"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&ml.clone(), -1, "")); __s.push_str(" ml"); py_string(__s) };
    Py::None
}
pub fn _count_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut ratio: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ratio") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ratio"))) };
    let mut part: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A count of a picture, said the way a person says it: \"two\", \"two and a half\".\n\n    Halves are as precise as a picture gets - \"2 and a quarter doubles tennis courts\" is a\n    number nobody says - and the word \"about\" in front of it covers the rounding.\n    ");
    whole = call_builtin("int", &[ratio.clone()]).unwrap_or_else(|e| exc::raise(e));
    part = op_sub(&ratio.clone(), &whole.clone());
    if py_truthy(&op_lt(&part.clone(), &Py::Float(0.25))) {
    return op_modulo(&py_str("%d"), &whole.clone());
    }
    if py_truthy(&op_lt(&part.clone(), &Py::Float(0.75))) {
    return if py_truthy(&whole.clone()) { op_modulo(&py_str("%d and a half"), &whole.clone()) } else { py_str("half") };
    }
    return op_modulo(&py_str("%d"), &op_add(&whole.clone(), &Py::Int(1)));
    Py::None
}
pub fn _area_pictures(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut square_metres: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "square_metres") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: square_metres"))) };
    let mut name: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut ratio: Py = Py::None;
    let mut size: Py = Py::None;
    let _ = py_str("The nearest thing a person has stood on, said as a count: \"about two tennis courts\".");
    let __iter1 = py_iter_or_raise(&g__AREA_PICTURES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    name = __unpk2[0].clone();
    plural = __unpk2[1].clone();
    size = __unpk2[2].clone();
    ratio = op_truediv(&square_metres.clone(), &size.clone());
    if py_truthy(&{ let __b = op_lt(&ratio.clone(), &Py::Int(5)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&size.clone(), &geti(&geti(&g__AREA_PICTURES(), &op_neg(&Py::Int(1))), &Py::Int(2)))) } }) {
    break;
    }
    }
    if py_truthy(&op_lt(&ratio.clone(), &Py::Float(0.4))) {
    return op_add(&py_str("less than half a "), &name.clone());
    }
    if py_truthy(&op_lt(&ratio.clone(), &Py::Float(0.75))) {
    return op_add(&py_str("about two thirds of a "), &name.clone());
    }
    if py_truthy(&op_lt(&ratio.clone(), &Py::Float(1.15))) {
    return op_add(&py_str("about a "), &name.clone());
    }
    if py_truthy(&op_lt(&ratio.clone(), &Py::Float(1.85))) {
    return op_add(&py_str("a bit more than a "), &name.clone());
    }
    return op_modulo(&py_str("about %s %s"), &py_tuple(vec![_count_said(&[ratio.clone()]), plural.clone()]));
    Py::None
}
pub fn _share_before(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut found: Py = Py::None;
    let mut text: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The count or share of a land measure said before it: 0.15, half, a quarter, 2.");
    text = slice_or_raise(&clean.clone(), &Py::None, &start.clone(), &Py::None);
    found = stdlib_call("re", "search", &[py_str("(\\d+(?:\\.\\d+)?)\\s*(?:of\\s+)?(?:an?\\s+)?$"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    return call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter1 = py_iter_or_raise(&g__WORD_SHARE());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    word = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[op_add(&op_add(&py_str("\\b"), &stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\s*(?:of\\s+)?(?:an?\\s+)?$")), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return value.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _land_in_pictures(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { py_str("") };
    let mut _u: Py = Py::None;
    let mut body: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut size: Py = Py::None;
    let mut square_metres: Py = Py::None;
    let mut table: Py = Py::None;
    let _ = py_str("A number of acres asked *whether it is big*: the size, then the nearest picture of it.");
    table = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&geti(&g_UNITS(), &py_str("area"))) { let __unpk8091 = py_unpack(&__item.clone(), 3, None); let mut name: Py = __unpk8091[0].clone(); let mut _u: Py = __unpk8091[1].clone(); let mut size: Py = __unpk8091[2].clone(); __out.push((name.clone(), size.clone())); } py_dict(__out) })() };
    square_metres = op_mul(&value.clone(), &geti(&table.clone(), &unit.clone()));
    if py_truthy(&op_lt(&square_metres.clone(), &Py::Int(1))) {
    return Py::None;
    }
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    said = op_add(&py_str("One "), &geti(&g_SAID_ONE(), &unit.clone()));
    } else {
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    said = op_modulo(&py_str("%d %s"), &py_tuple(vec![call_builtin("round", &[value.clone()]).unwrap_or_else(|e| exc::raise(e)), geti(&g_SAID(), &unit.clone())]));
    } else {
    said = op_modulo(&py_str("%s %s"), &py_tuple(vec![_fmt(&[value.clone()]), geti(&g_SAID(), &unit.clone())]));
    }
    }
    body = op_modulo(&py_str("%s is about %s square metres - %s"), &py_tuple(vec![said.clone(), _fmt(&[call_builtin("round", &[square_metres.clone()]).unwrap_or_else(|e| exc::raise(e))]), _area_pictures(&[square_metres.clone()])]));
    if py_truthy(&{ let __b = op_ge(&square_metres.clone(), &Py::Int(400)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("garden"))) } }) {
    let __aug1 = op_add(&body.clone(), &py_str(", which is a generous garden for a house"));
    body = __aug1;
    }
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09))) {
    return op_add(&body.clone(), &py_str("."));
    }
    return op_add(&op_add(&body.clone(), &py_str(". ")), &geti(&g_LAND_PLAIN(), &unit.clone()));
    Py::None
}
pub fn _land_size(args: &[Py]) -> Py {
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
    let mut mentions: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mentions") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mentions"))) };
    let mut before: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut end: Py = Py::None;
    let mut family: Py = Py::None;
    let mut pictured: Py = Py::None;
    let mut start: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("\"how big is an acre\": one land measure named, with no number and no second unit.");
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[mentions.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&geti(&mentions.clone(), &Py::Int(0)), 4, None);
    start = __unpk1[0].clone();
    end = __unpk1[1].clone();
    family = __unpk1[2].clone();
    unit = __unpk1[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&family.clone(), &py_str("area"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_LAND_PLAIN(), &unit.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_SIZE_JUDGEMENT(), "search", &[clean.clone()])) {
    counted = _share_before(&[clean.clone(), start.clone()]);
    if py_truthy(&Py::Bool(!py_is(&counted.clone(), &Py::None))) {
    pictured = _land_in_pictures(&[counted.clone(), unit.clone(), clean.clone()]);
    if py_truthy(&pictured.clone()) {
    return pictured.clone();
    }
    }
    }
    before = slice_or_raise(&clean.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&start.clone(), &Py::Int(14))]).unwrap_or_else(|e| exc::raise(e)), &start.clone(), &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__LAND_ASK(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\d\\s*$"), before.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:two|three|four|five|ten|twenty)\\s+$"), before.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__UNIT_WORD(), "search", &[slice_or_raise(&clean.clone(), &end.clone(), &Py::None, &Py::None)]); if py_truthy(&__b) { __b } else { mcall(&g__TARGET(), "search", &[mcall(&slice_or_raise(&clean.clone(), &Py::None, &start.clone(), &Py::None), "strip", &[])]) } }) {
    return Py::None;
    }
    return geti(&g_LAND_PLAIN(), &unit.clone());
    Py::None
}
pub fn _height(args: &[Py]) -> Py {
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
    let mut mentions: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mentions") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mentions"))) };
    let mut _u: Py = Py::None;
    let mut feet: Py = Py::None;
    let mut inches: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut total: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("A height typed the way people type it: 5'6, 5ft 6, or 1.8m asked for in feet and inches.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mentions.clone()) { let __unpk3666 = py_unpack(&__item.clone(), 4, None); let mut _u: Py = __unpk3666[0].clone(); let mut _u: Py = __unpk3666[1].clone(); let mut _u: Py = __unpk3666[2].clone(); let mut unit: Py = __unpk3666[3].clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("cm"), py_str("m")]), &unit.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    match_ = { let __b = stdlib_call("re", "search", &[py_str("(\\d)\\s*(?:'|’|ft|foot|feet)\\s*(\\d{1,2})\\s*(?:\\\"|''|in|inch|inches)?"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("(\\d)\\s*(?:ft|foot|feet)\\s+(\\d{1,2})\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return _metres_in_feet(&[clean.clone()]);
    }
    let __unpk1 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    feet = __unpk1[0].clone();
    inches = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __cmp21: Py = feet.clone(); if py_truthy(&op_le(&Py::Int(2), &__cmp21)) { op_le(&__cmp21, &Py::Int(8)) } else { Py::Bool(false) } })); if py_truthy(&__b) { __b } else { op_gt(&inches.clone(), &Py::Int(11)) } }) {
    return Py::None;
    }
    total = op_add(&op_mul(&feet.clone(), &Py::Int(12)), &inches.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&feet.clone(), -1, "")); __s.push_str(" feet "); __s.push_str(&fstr(&inches.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&call_builtin("round", &[op_mul(&total.clone(), &Py::Float(2.54))]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(" cm, or "); __s.push_str(&fstr(&op_mul(&total.clone(), &Py::Float(0.0254)), -1, &".2f")); __s.push_str(" m."); py_string(__s) };
    Py::None
}
pub fn _is_a_distance(args: &[Py]) -> Py {
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
    let _ = py_str("True when the number is how far or how long, not how tall.");
    return { let __b = call_builtin("bool", &[mcall(&g__NOT_A_HEIGHT(), "search", &[clean.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\btall\\b"), clean.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn _asks_something_else(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("True when a stated height comes with a question about something that is not a measure.");
    if py_truthy(&mcall(&g__A_COMPLAINT(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g__ASKS_HOW_MANY(), "finditer", &[clean.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    thing = mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__UNIT_WORD(), "fullmatch", &[thing.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__A_MEASURE_SAID_ANOTHER_WAY(), &thing.clone())) } }) {
    return Py::Bool(true);
    }
    }
    let __iter2 = py_iter_or_raise(&mcall(&g__ASKS_ELSEWHERE(), "finditer", &[clean.clone()]));
    for __item in __iter2 {
    match_ = __item.clone();
    tail = mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "split", &[]), &Py::None, &Py::Int(5), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__A_MEASURE_QUESTION(), "search", &[tail.clone()])))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _own_height(args: &[Py]) -> Py {
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
    let mut cm: Py = Py::None;
    let mut feet: Py = Py::None;
    let mut found: Py = Py::None;
    let mut inches: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut metres: Py = Py::None;
    let mut said: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("The other number for a height the person states about themselves, or None.\n\n    \"im 1.75 metres\" -> \"1.75 m is about 5 foot 9 (5.7 feet).\"\n    \"im 5 foot 9\" -> \"5 foot 9 is about 175 cm, or 1.75 m.\"\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__SAYS_OWN_HEIGHT(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_is_a_distance(&[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&_asks_something_else(&[clean.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, py_str("")]), 2, None);
    metres = __unpk1[0].clone();
    said = __unpk1[1].clone();
    found = mcall(&g__HEIGHT_METRES(), "search", &[clean.clone()]);
    if py_truthy(&found.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&found.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str(" m"); py_string(__s) }]), 2, None);
    metres = __unpk2[0].clone();
    said = __unpk2[1].clone();
    }
    if py_truthy(&Py::Bool(py_is(&metres.clone(), &Py::None))) {
    found = mcall(&g__HEIGHT_CM(), "search", &[clean.clone()]);
    if py_truthy(&found.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![op_truediv(&call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(100)), { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&found.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str(" cm"); py_string(__s) }]), 2, None);
    metres = __unpk3[0].clone();
    said = __unpk3[1].clone();
    }
    }
    if py_truthy(&Py::Bool(!py_is(&metres.clone(), &Py::None))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __cmp41: Py = metres.clone(); if py_truthy(&op_le(&Py::Float(1.2), &__cmp41)) { op_le(&__cmp41, &Py::Float(2.2)) } else { Py::Bool(false) } }))) {
    return Py::None;
    }
    total = op_truediv(&metres.clone(), &Py::Float(0.0254));
    feet = call_builtin("int", &[op_floordiv(&total.clone(), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e));
    inches = call_builtin("round", &[op_sub(&total.clone(), &op_mul(&feet.clone(), &Py::Int(12)))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&inches.clone(), &Py::Int(12)))) {
    let __unpk5 = py_unpack(&py_tuple(vec![op_add(&feet.clone(), &Py::Int(1)), Py::Int(0)]), 2, None);
    feet = __unpk5[0].clone();
    inches = __unpk5[1].clone();
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&feet.clone(), -1, "")); __s.push_str(" foot "); __s.push_str(&fstr(&inches.clone(), -1, "")); __s.push_str(" ("); __s.push_str(&fstr(&op_truediv(&total.clone(), &Py::Int(12)), -1, &".1f")); __s.push_str(" feet)."); py_string(__s) };
    }
    match_ = { let __b = mcall(&g__HEIGHT_FEET(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__HEIGHT_FEET_WORD(), "search", &[clean.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    let __unpk6 = py_unpack(&py_tuple(vec![call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e))]), 2, None);
    feet = __unpk6[0].clone();
    inches = __unpk6[1].clone();
    if py_truthy(&op_gt(&inches.clone(), &Py::Int(11))) {
    return Py::None;
    }
    cm = call_builtin("round", &[op_mul(&op_add(&op_mul(&feet.clone(), &Py::Int(12)), &inches.clone()), &Py::Float(2.54))]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&feet.clone(), -1, "")); __s.push_str(" foot "); __s.push_str(&fstr(&inches.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&cm.clone(), -1, "")); __s.push_str(" cm, or "); __s.push_str(&fstr(&op_truediv(&cm.clone(), &Py::Int(100)), -1, &".2f")); __s.push_str(" m."); py_string(__s) };
    Py::None
}
pub fn _metres_in_feet(args: &[Py]) -> Py {
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
    let mut feet: Py = Py::None;
    let mut found: Py = Py::None;
    let mut inches: Py = Py::None;
    let mut metres: Py = Py::None;
    let mut said: Py = Py::None;
    let mut total: Py = Py::None;
    let _ = py_str("A height given in metres or centimetres, asked for in feet and inches.\n\n    Only a height: 1.2 m to 2.2 m covers a child to a tall adult, so a room, a run or a\n    length of cable is left to the plain conversion. \"1.8m\" reaches here as \"1.8 m\" because\n    _JOINED puts the space in, which is why the metre spelling is worth its own branch.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__ASKS_FEET(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, py_str("")]), 2, None);
    metres = __unpk1[0].clone();
    said = __unpk1[1].clone();
    found = stdlib_call("re", "search", &[py_str("(?<![\\w.])(\\d(?:\\.\\d{1,2})?)\\s*(?:m|metres?|meters?)(?!\\w)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    let __unpk2 = py_unpack(&py_tuple(vec![call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&found.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str(" m"); py_string(__s) }]), 2, None);
    metres = __unpk2[0].clone();
    said = __unpk2[1].clone();
    } else {
    found = stdlib_call("re", "search", &[py_str("(?<![\\w.])(\\d{2,3}(?:\\.\\d)?)\\s*(?:cm|centimetres?|centimeters?)(?!\\w)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    let __unpk3 = py_unpack(&py_tuple(vec![op_truediv(&call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(100)), { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&found.clone(), "group", &[Py::Int(1)]), -1, "")); __s.push_str(" cm"); py_string(__s) }]), 2, None);
    metres = __unpk3[0].clone();
    said = __unpk3[1].clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&metres.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __cmp41: Py = metres.clone(); if py_truthy(&op_le(&Py::Float(1.2), &__cmp41)) { op_le(&__cmp41, &Py::Float(2.2)) } else { Py::Bool(false) } })) } }) {
    return Py::None;
    }
    total = op_truediv(&metres.clone(), &Py::Float(0.0254));
    feet = call_builtin("int", &[op_floordiv(&total.clone(), &Py::Int(12))]).unwrap_or_else(|e| exc::raise(e));
    inches = call_builtin("round", &[op_sub(&total.clone(), &op_mul(&feet.clone(), &Py::Int(12)))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&inches.clone(), &Py::Int(12)))) {
    let __unpk5 = py_unpack(&py_tuple(vec![op_add(&feet.clone(), &Py::Int(1)), Py::Int(0)]), 2, None);
    feet = __unpk5[0].clone();
    inches = __unpk5[1].clone();
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&feet.clone(), -1, "")); __s.push_str(" foot "); __s.push_str(&fstr(&inches.clone(), -1, "")); __s.push_str(" ("); __s.push_str(&fstr(&op_truediv(&total.clone(), &Py::Int(12)), -1, &".1f")); __s.push_str(" feet)."); py_string(__s) };
    Py::None
}
pub fn _cup_of_something(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut food: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut measure: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut share: Py = Py::None;
    let mut spoon: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Grams in a cup, mug, tablespoon or teaspoon of a dry thing, asked about by name.\n\n    Cups and spoons measure volume, so a dry ingredient's weight depends on what it is;\n    GRAMS_PER_CUP holds the weights cooks use, scaled down for the smaller spoons.\n    ");
    if py_truthy(&mcall(&g__PER_CUP(), "search", &[clean.clone()])) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![py_str("cup"), Py::Float(1.0)]), 2, None);
    measure = __unpk1[0].clone();
    share = __unpk1[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmugs?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let __unpk2 = py_unpack(&py_tuple(vec![py_str("mug"), op_truediv(&Py::Int(240), &Py::Int(250))]), 2, None);
    measure = __unpk2[0].clone();
    share = __unpk2[1].clone();
    } else {
    if py_truthy(&_names_unit(&[clean.clone(), py_str("tbsp")])) {
    let __unpk3 = py_unpack(&py_tuple(vec![py_str("tablespoon"), Py::Float(16.0)]), 2, None);
    measure = __unpk3[0].clone();
    share = __unpk3[1].clone();
    } else {
    if py_truthy(&_names_unit(&[clean.clone(), py_str("tsp")])) {
    let __unpk4 = py_unpack(&py_tuple(vec![py_str("teaspoon"), Py::Float(48.0)]), 2, None);
    measure = __unpk4[0].clone();
    share = __unpk4[1].clone();
    } else {
    if py_truthy(&_names_unit(&[clean.clone(), py_str("dessertspoon")])) {
    let __unpk5 = py_unpack(&py_tuple(vec![py_str("dessertspoon"), Py::Float(24.0)]), 2, None);
    measure = __unpk5[0].clone();
    share = __unpk5[1].clone();
    }
    }
    }
    }
    text = op_add(&py_str("a "), &measure.clone());
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("tablespoon"), py_str("teaspoon"), py_str("dessertspoon")]), &measure.clone()))) {
    spoon = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_GRAMS_PER_TSP()) { let __unpk2261 = py_unpack(&__item.clone(), 2, None); let mut food: Py = __unpk2261[0].clone(); let mut grams: Py = __unpk2261[1].clone(); if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[food.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(py_tuple(vec![food.clone(), grams.clone()])); } } py_list(__out) })() };
    if py_truthy(&spoon.clone()) {
    let __unpk6 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(spoon.clone()); __kws.extend(vec![("key".to_string(), { let amount = amount.clone(); let clean = clean.clone(); let food = food.clone(); let grams = grams.clone(); let measure = measure.clone(); let named = named.clone(); let said = said.clone(); let share = share.clone(); let spoon = spoon.clone(); let text = text.clone(); py_func("lambda", move |args: &[Py]| { let amount = amount.clone(); let clean = clean.clone(); let food = food.clone(); let grams = grams.clone(); let measure = measure.clone(); let named = named.clone(); let said = said.clone(); let share = share.clone(); let spoon = spoon.clone(); let text = text.clone(); let row: Py = __kwp(args, 0, "row").unwrap_or(Py::None); call_builtin("len", &[geti(&row.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("max", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, 2, None);
    food = __unpk6[0].clone();
    grams = __unpk6[1].clone();
    amount = op_truediv(&op_mul(&grams.clone(), &op_truediv(&Py::Float(240.0), &share.clone())), &Py::Float(5.0));
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&measure.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&food.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_fmt(&[amount.clone()]), -1, "")); __s.push_str(" g."); py_string(__s) };
    }
    }
    named = _food_named(&[clean.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { _asks_for_weight(&[clean.clone()]) } }) {
    let __unpk7 = py_unpack(&named.clone(), 2, None);
    food = __unpk7[0].clone();
    grams = __unpk7[1].clone();
    amount = if py_truthy(&Py::Bool(py_eq(&share.clone(), &Py::Float(1.0)))) { grams.clone() } else { call_builtin("max", &[Py::Int(1), call_builtin("round", &[op_truediv(&grams.clone(), &share.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) };
    said = mcall(&g_AS_KITCHEN_SAYS(), "get", &[food.clone(), food.clone()]);
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&measure.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_grams(&[amount.clone()]), -1, "")); __s.push_str(" g."); py_string(__s) };
    }
    if py_truthy(&{ let __b = mcall(&g__WEIGH(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(grams?|g|oz|ounces?|kg)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return op_add(&op_add(&py_str("It depends what is in "), &text.clone()), &py_str(": flour is about 120 g, sugar about 200 g, butter 227 g. By volume a cup is 240 ml, which is 8 fluid ounces."));
    }
    return Py::None;
    Py::None
}
pub fn _handful(args: &[Py]) -> Py {
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
    let mut food: Py = Py::None;
    let mut grams_per_cup: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_food: Py = Py::None;
    let mut stuff: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A handful, and how many grams of the food in it.\n\n    \"How many grams in a handful of almonds\" is the everyday version of a serving, and the\n    answer is a quarter of a cup of whatever it is: the cup weight this table already holds\n    gives the grams. Without a named food there is still a useful figure to give, so the\n    reply gives the quarter cup and three weights that show the spread.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\bhandfuls?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _asks_for_weight(&[clean.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow much\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:cost|costs|costing|price|prices|priced|cheap|cheaper|expensive)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    food = _food_named(&[clean.clone()]);
    if py_truthy(&food.clone()) {
    let __unpk1 = py_unpack(&food.clone(), 2, None);
    name = __unpk1[0].clone();
    grams_per_cup = __unpk1[1].clone();
    said = mcall(&g_AS_KITCHEN_SAYS(), "get", &[name.clone(), name.clone()]);
    return { let mut __s = String::new(); __s.push_str("A handful of "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&_grams(&[call_builtin("round", &[op_truediv(&grams_per_cup.clone(), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" g, since a handful is roughly a quarter of a cup."); py_string(__s) };
    }
    stuff = py_str("");
    said_food = stdlib_call("re", "search", &[py_str("\\bhandfuls?\\s+(?:of\\s+)?(?!of\\b|it\\b|them\\b|these\\b|those\\b|that\\b)([a-z]+(?:\\s+[a-z]+)?)"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said_food.clone()) {
    stuff = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&mcall(&said_food.clone(), "group", &[Py::Int(1)]), "split", &[])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g__NOT_A_FOOD(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]);
    }
    if py_truthy(&stuff.clone()) {
    return { let mut __s = String::new(); __s.push_str("A handful of "); __s.push_str(&fstr(&stuff.clone(), -1, "")); __s.push_str(" is about a quarter of a cup, but the weight depends on what is in it: 35 g of nuts against 8 g of spinach."); py_string(__s) };
    }
    return py_str("A handful is about a quarter of a cup, which comes to roughly 35 g of nuts, 30 g of raisins or 25 g of crisps.");
    Py::None
}
pub fn _asks_for_weight(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line wants a weight: \"how many grams\", \"what does a cup weigh\", \"in oz\".\n\n    \"How much water for two cups of tea\" names a food and a cup but wants a pour, not grams,\n    so a food's cup weight is only given when the line itself asks for a weight. A figure in\n    pounds that the line is judging the price of is money, not a weight, so it asks nothing about\n    grams: \"is 4 pounds a lot for a pint of milk\" wants the price the milk sells at.\n    ");
    if py_truthy(&mcall(&g__POUNDS_IS_MONEY(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__WEIGH(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:grams?|g|kg|kilograms?|oz|ounces?|pounds?|lbs?)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_unit(args: &[Py]) -> Py {
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
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut _u: Py = Py::None;
    let mut items: Py = Py::None;
    let mut name: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names this unit, in any of its words.");
    let __iter1 = py_iter_or_raise(&mcall(&g_UNITS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    _u = __unpk2[0].clone();
    items = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&items.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 3, None);
    name = __unpk4[0].clone();
    words = __unpk4[1].clone();
    _u = __unpk4[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&name.clone(), &unit.clone())); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _frac(args: &[Py]) -> Py {
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
    let mut mixed: Py = Py::None;
    let mut slash: Py = Py::None;
    let mut unit_and_a_half: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word_fraction: Py = Py::None;
    let _ = py_str("A recipe's fractions as decimals, so \"3/4 cup\" reads as three quarters of a cup.");
pub fn __nested_unit_and_a_half_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut mixed: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut slash: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut unit_and_a_half: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut value: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut word_fraction: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut count: Py = Py::None;
    let mut whole: Py = Py::None;
    count = { let __b = mcall(&match_.clone(), "group", &[py_str("count")]); if py_truthy(&__b) { __b } else { py_str("one") } };
    whole = if py_truthy(&mcall(&count.clone(), "isdigit", &[])) { { let __b = mcall(&g__SAID_WHOLE(), "get", &[count.clone()]); if py_truthy(&__b) { __b } else { call_builtin("int", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } else { geti(&g__SAID_WHOLE(), &count.clone()) };
    return { let mut __s = String::new(); __s.push_str(&fstr(&op_add(&whole.clone(), &Py::Float(0.5)), -1, &"g")); __s.push_str(" "); __s.push_str(&fstr(&mcall(&match_.clone(), "group", &[py_str("unit")]), -1, "")); py_string(__s) };
    Py::None
}
    unit_and_a_half = {{ let clean = clean.clone(); let mixed = mixed.clone(); let slash = slash.clone(); let unit_and_a_half = unit_and_a_half.clone(); let value = value.clone(); let word_fraction = word_fraction.clone(); py_func("unit_and_a_half", move |args: &[Py]| { let mut full = vec![clean.clone(), mixed.clone(), slash.clone(), unit_and_a_half.clone(), value.clone(), word_fraction.clone()]; full.extend_from_slice(args); __nested_unit_and_a_half_0(&full) }) }};
    clean = mcall(&g__UNIT_AND_A_HALF(), "sub", &[unit_and_a_half.clone(), clean.clone()]);
pub fn __nested_mixed_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut mixed: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut slash: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut unit_and_a_half: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut value: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut word_fraction: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    value = mcall(&g__FRACTION(), "get", &[py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(2)]), mcall(&match_.clone(), "group", &[Py::Int(3)])])]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&op_add(&call_builtin("int", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &value.clone()), -1, &"g")); py_string(__s) };
    Py::None
}
    mixed = {{ let clean = clean.clone(); let mixed = mixed.clone(); let slash = slash.clone(); let unit_and_a_half = unit_and_a_half.clone(); let value = value.clone(); let word_fraction = word_fraction.clone(); py_func("mixed", move |args: &[Py]| { let mut full = vec![clean.clone(), mixed.clone(), slash.clone(), unit_and_a_half.clone(), value.clone(), word_fraction.clone()]; full.extend_from_slice(args); __nested_mixed_1(&full) }) }};
    clean = mcall(&g__MIXED(), "sub", &[mixed.clone(), clean.clone()]);
    clean = mcall(&g__AND_A_HALF(), "sub", &[{ let clean = clean.clone(); let mixed = mixed.clone(); let slash = slash.clone(); let unit_and_a_half = unit_and_a_half.clone(); let value = value.clone(); let word_fraction = word_fraction.clone(); py_func("lambda", move |args: &[Py]| { let clean = clean.clone(); let mixed = mixed.clone(); let slash = slash.clone(); let unit_and_a_half = unit_and_a_half.clone(); let value = value.clone(); let word_fraction = word_fraction.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); { let mut __s = String::new(); __s.push_str(&fstr(&op_add(&{ let __b = mcall(&g__SAID_WHOLE(), "get", &[mcall(&m.clone(), "group", &[Py::Int(1)])]); if py_truthy(&__b) { __b } else { call_builtin("int", &[mcall(&m.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)) } }, &Py::Float(0.5)), -1, &"g")); py_string(__s) } }) }, clean.clone()]);
    let __iter3 = py_iter_or_raise(&g__WORD_FRACTION());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    word_fraction = __unpk4[0].clone();
    value = __unpk4[1].clone();
    clean = mcall(&word_fraction.clone(), "sub", &[value.clone(), clean.clone()]);
    }
pub fn __nested_slash_4(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clean: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut mixed: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut slash: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut unit_and_a_half: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut value: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut word_fraction: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    value = mcall(&g__FRACTION(), "get", &[py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(1)]), mcall(&match_.clone(), "group", &[Py::Int(2)])])]);
    return if py_truthy(&value.clone()) { mcall(&mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &".4f")); py_string(__s) }, "rstrip", &[py_str("0")]), "rstrip", &[py_str(".")]) } else { mcall(&match_.clone(), "group", &[Py::Int(0)]) };
    Py::None
}
    slash = {{ let clean = clean.clone(); let mixed = mixed.clone(); let slash = slash.clone(); let unit_and_a_half = unit_and_a_half.clone(); let value = value.clone(); let word_fraction = word_fraction.clone(); py_func("slash", move |args: &[Py]| { let mut full = vec![clean.clone(), mixed.clone(), slash.clone(), unit_and_a_half.clone(), value.clone(), word_fraction.clone()]; full.extend_from_slice(args); __nested_slash_4(&full) }) }};
    return mcall(&g__SLASH(), "sub", &[slash.clone(), clean.clone()]);
    Py::None
}
pub fn _rule_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut family: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut from_unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "from_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: from_unit"))) };
    let mut to_unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "to_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: to_unit"))) };
    let mut amount: Py = Py::None;
    let mut candidate: Py = Py::None;
    let mut example: Py = Py::None;
    let mut factor: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut said_from: Py = Py::None;
    let mut said_to: Py = Py::None;
    let mut source: Py = Py::None;
    let mut target: Py = Py::None;
    let _ = py_str("The rule for a conversion asked as a rule: no numbers, so the factor and one example.");
    source = _factor(&[family.clone(), from_unit.clone()]);
    target = _factor(&[family.clone(), to_unit.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&source.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&target.clone())); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("abs", &[op_sub(&source.clone(), &target.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(1e-09)) } } } }) {
    return Py::None;
    }
    factor = op_truediv(&source.clone(), &target.clone());
    said_from = mcall(&g_SAID(), "get", &[from_unit.clone(), from_unit.clone()]);
    said_to = mcall(&g_SAID(), "get", &[to_unit.clone(), to_unit.clone()]);
    if py_truthy(&op_ge(&factor.clone(), &Py::Int(1))) {
    rule = { let mut __s = String::new(); __s.push_str("multiply the "); __s.push_str(&fstr(&said_from.clone(), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&_fmt(&[factor.clone()]), -1, "")); py_string(__s) };
    } else {
    rule = { let mut __s = String::new(); __s.push_str("divide the "); __s.push_str(&fstr(&said_from.clone(), -1, "")); __s.push_str(" by "); __s.push_str(&fstr(&_fmt(&[op_truediv(&Py::Int(1), &factor.clone())]), -1, "")); py_string(__s) };
    }
    amount = Py::Int(1);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![Py::Int(1), Py::Int(10), Py::Int(100), Py::Int(1000), Py::Int(10000)]));
    for __item in __iter1 {
    candidate = __item.clone();
    amount = candidate.clone();
    if py_truthy(&op_ge(&op_mul(&candidate.clone(), &factor.clone()), &Py::Int(1))) {
    break;
    }
    }
    example = { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[amount.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); __args.push(from_unit.clone()); __kws.extend(vec![("one".to_string(), Py::Bool(py_eq(&amount.clone(), &Py::Int(1))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) }, -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_fmt(&[op_mul(&amount.clone(), &factor.clone())]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_say(&[op_mul(&amount.clone(), &factor.clone()), to_unit.clone()]), -1, "")); py_string(__s) };
    return { let mut __s = String::new(); __s.push_str("To go from "); __s.push_str(&fstr(&said_from.clone(), -1, "")); __s.push_str(" to "); __s.push_str(&fstr(&said_to.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&rule.clone(), -1, "")); __s.push_str(": "); __s.push_str(&fstr(&example.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _is_bare_pair(args: &[Py]) -> Py {
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
    let mut mentions: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mentions") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mentions"))) };
    let mut _family: Py = Py::None;
    let mut _unit: Py = Py::None;
    let mut end: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut start: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line is nothing but two units and the word between them: \"km to miles\".\n\n    The units are taken out by where they were found, not word by word: a unit can be two words\n    (\"ml to fl oz\", \"kin to fluid ounces\") and the old word check read \"fl\" as a stray, so every\n    line naming a fluid ounce as one of a bare pair returned None.\n    ");
    if py_truthy(&op_lt(&call_builtin("len", &[mentions.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::Bool(false);
    }
    rest = clean.clone();
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[mentions.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    start = __unpk2[0].clone();
    end = __unpk2[1].clone();
    _family = __unpk2[2].clone();
    _unit = __unpk2[3].clone();
    rest = op_add(&op_add(&slice_or_raise(&rest.clone(), &Py::None, &start.clone(), &Py::None), &py_str(" ")), &slice_or_raise(&rest.clone(), &end.clone(), &Py::None, &Py::None));
    }
    rest = stdlib_call("re", "sub", &[py_str("\\d+(?:\\.\\d+)?"), py_str(" "), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    words = stdlib_call("re", "findall", &[py_str("[a-z]+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__BARE_JOIN(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_ounce_pair(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut family: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: family"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut other_family: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other_family") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other_family"))) };
    let mut other_unit: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "other_unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: other_unit"))) };
    let _ = py_str("Both sides of a conversion, with a bare ounce read in the other side's family.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&unit.clone(), &py_str("ounce"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&other_family.clone(), &py_str("volume"))) } }) {
    return py_tuple(vec![py_str("volume"), py_str("fl oz"), other_family.clone(), other_unit.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other_unit.clone(), &py_str("ounce"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&family.clone(), &py_str("volume"))) } }) {
    return py_tuple(vec![family.clone(), unit.clone(), py_str("volume"), py_str("fl oz")]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&unit.clone(), &py_str("fl oz"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&other_family.clone(), &py_str("mass"))) } }) {
    return py_tuple(vec![py_str("mass"), py_str("ounce"), other_family.clone(), other_unit.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&other_unit.clone(), &py_str("fl oz"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&family.clone(), &py_str("mass"))) } }) {
    return py_tuple(vec![family.clone(), unit.clone(), py_str("mass"), py_str("ounce")]);
    }
    return py_tuple(vec![family.clone(), unit.clone(), other_family.clone(), other_unit.clone()]);
    Py::None
}
pub fn _how_to_convert(args: &[Py]) -> Py {
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
    let mut mentions: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mentions") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mentions"))) };
    let mut _end: Py = Py::None;
    let mut _f: Py = Py::None;
    let mut _oend: Py = Py::None;
    let mut _u: Py = Py::None;
    let mut family: Py = Py::None;
    let mut other_family: Py = Py::None;
    let mut other_start: Py = Py::None;
    let mut other_unit: Py = Py::None;
    let mut position: Py = Py::None;
    let mut start: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("\"how do i convert km to miles\" names the two units and no number: give the rule.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__HOW_TO(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_bare_pair(&[clean.clone(), mentions.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__NUMBER(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mentions.clone()) { let __unpk376 = py_unpack(&__item.clone(), 4, None); let mut start: Py = __unpk376[0].clone(); let mut _end: Py = __unpk376[1].clone(); let mut _f: Py = __unpk376[2].clone(); let mut _u: Py = __unpk376[3].clone(); __out.push(Py::Bool(!py_is(&_number_before(&[clean.clone(), start.clone()]), &Py::None))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[mentions.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    position = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 4, None);
    start = __unpk3[0].clone();
    _end = __unpk3[1].clone();
    family = __unpk3[2].clone();
    unit = __unpk3[3].clone();
    let __iter4 = py_iter_or_raise(&slice_or_raise(&mentions.clone(), &op_add(&position.clone(), &Py::Int(1)), &Py::None, &Py::None));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 4, None);
    other_start = __unpk5[0].clone();
    _oend = __unpk5[1].clone();
    other_family = __unpk5[2].clone();
    other_unit = __unpk5[3].clone();
    let __unpk6 = py_unpack(&_the_ounce_pair(&[family.clone(), unit.clone(), other_family.clone(), other_unit.clone()]), 4, None);
    family = __unpk6[0].clone();
    unit = __unpk6[1].clone();
    other_family = __unpk6[2].clone();
    other_unit = __unpk6[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&other_family.clone(), &family.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&other_unit.clone(), &unit.clone())) } }) {
    continue;
    }
    return _rule_sentence(&[family.clone(), unit.clone(), other_unit.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _tail_is_about_the_measure(args: &[Py]) -> Py {
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
    let _ = py_str("True when the measure is what the line asks about, rather than the object of a plan.\n\n    A measure is nearly always taken out of something - a pound of mince, a metre of fabric,\n    a cup of flour, ten minutes of stretching, three weeks off work - so the words after it are\n    how a person says what is measured. What does change the question is an asker after the\n    measure: \"how many days in a week should I run\" is a training plan, not a conversion of a\n    week into days, so it is left to the modules that read it.\n    ");
    return Py::Bool(!py_truthy(&mcall(&g__ASKING_AFTER_MEASURE(), "search", &[mcall(&tail.clone(), "lower", &[])])));
    Py::None
}
pub fn _share_value(args: &[Py]) -> Py {
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
    let mut slash: Py = Py::None;
    let _ = py_str("The number a fraction word is, or None when it is not a fraction this table knows.");
    if py_truthy(&Py::Bool(py_contains(&g__SHARE_VALUE(), &word.clone()))) {
    return geti(&g__SHARE_VALUE(), &word.clone());
    }
    slash = stdlib_call("re", "fullmatch", &[py_str("(\\d)\\s*/\\s*(\\d)"), word.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = slash.clone(); if !py_truthy(&__b) { __b } else { call_builtin("int", &[mcall(&slash.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return op_truediv(&call_builtin("int", &[mcall(&slash.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[mcall(&slash.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)));
    }
    return Py::None;
    Py::None
}
pub fn _share_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The name a cook would give this fraction, for the lines that write it in digits.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![Py::Float(0.25), py_str("a quarter")]), py_tuple(vec![Py::Float(0.5), py_str("half")]), py_tuple(vec![Py::Float(0.75), py_str("three quarters")]), py_tuple(vec![op_truediv(&Py::Int(1), &Py::Int(3)), py_str("a third")]), py_tuple(vec![op_truediv(&Py::Int(2), &Py::Int(3)), py_str("two thirds")]), py_tuple(vec![Py::Float(0.2), py_str("a fifth")]), py_tuple(vec![op_truediv(&Py::Int(1), &Py::Int(6)), py_str("a sixth")]), py_tuple(vec![op_truediv(&Py::Int(1), &Py::Int(8)), py_str("an eighth")]), py_tuple(vec![Py::Float(0.125), py_str("an eighth")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    known = __unpk2[0].clone();
    name = __unpk2[1].clone();
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &known.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.001))) {
    return name.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _share_spoons(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut per: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "per") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: per"))) };
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spoons: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The spoons a fraction of a unit is: whole ones, then the teaspoons left over.");
    spoons = op_mul(&value.clone(), &per.clone());
    whole = call_builtin("int", &[spoons.clone()]).unwrap_or_else(|e| exc::raise(e));
    rest = call_builtin("round", &[op_mul(&op_sub(&spoons.clone(), &whole.clone()), &Py::Int(3))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_ge(&rest.clone(), &Py::Int(3))) {
    let __unpk1 = py_unpack(&py_tuple(vec![op_add(&whole.clone(), &Py::Int(1)), Py::Int(0)]), 2, None);
    whole = __unpk1[0].clone();
    rest = __unpk1[1].clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&whole.clone()))) {
    return if py_truthy(&Py::Bool(py_eq(&rest.clone(), &Py::Int(1)))) { py_str("a teaspoon") } else { { let mut __s = String::new(); __s.push_str(&fstr(&rest.clone(), -1, "")); __s.push_str(" teaspoons"); py_string(__s) } };
    }
    said = if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) { { let mut __s = String::new(); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(" tablespoon"); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(" tablespoons"); py_string(__s) } };
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return said.clone();
    }
    return op_add(&said.clone(), &if py_truthy(&Py::Bool(py_eq(&rest.clone(), &Py::Int(1)))) { py_str(" and a teaspoon") } else { { let mut __s = String::new(); __s.push_str(" and "); __s.push_str(&fstr(&rest.clone(), -1, "")); __s.push_str(" teaspoons"); py_string(__s) } });
    Py::None
}
pub fn _share_of_a_measure(args: &[Py]) -> Py {
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
    let mut tail: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "tail") { v.clone() }
    else { Py::Bool(true) };
    let mut advice: Py = Py::None;
    let mut before: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut millilitres: Py = Py::None;
    let mut outer: Py = Py::None;
    let mut outer_word: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A fraction of a cup, a tablespoon or a teaspoon, answered in the smaller units.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__SHARE_ASK(), "search", &[clean.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g__NOT_ABOUT_A_MEASURE(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    match_ = mcall(&g__SHARE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    word = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("word")]), "split", &[])]);
    value = _share_value(&[word.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    return Py::None;
    }
    said = { let __b = _share_name(&[value.clone()]); if py_truthy(&__b) { __b } else { word.clone() } };
    before = if py_truthy(&stdlib_call("re", "search", &[py_str("\\bof\\s*$"), slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { mcall(&g__SHARE_FRACTION(), "findall", &[slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]) } else { py_list(vec![]) };
    if py_truthy(&before.clone()) {
    outer_word = mcall(&py_str(" "), "join", &[mcall(&geti(&before.clone(), &op_neg(&Py::Int(1))), "split", &[])]);
    outer = _share_value(&[outer_word.clone()]);
    if py_truthy(&outer.clone()) {
    let __aug1 = op_mul(&value.clone(), &outer.clone());
    value = __aug1;
    said = { let mut __s = String::new(); __s.push_str(&fstr(&{ let __b = _share_name(&[outer.clone()]); if py_truthy(&__b) { __b } else { outer_word.clone() } }, -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&said.clone(), -1, "")); py_string(__s) };
    }
    }
    unit = mcall(&match_.clone(), "group", &[py_str("unit")]);
    rest = op_add(&op_add(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None));
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:cups?|tablespoons?|tbsp|teaspoons?|tsp)\\b"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("cup")])])) {
    millilitres = call_builtin("round", &[op_mul(&value.clone(), &Py::Int(240))]).unwrap_or_else(|e| exc::raise(e));
    lead = if py_truthy(&Py::Bool(py_eq(&said.clone(), &py_str("half")))) { py_str("Half a cup") } else { { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" of a cup"); py_string(__s) } };
    advice = if py_truthy(&tail.clone()) { py_str(": spoons are the easy way to cut a cup down.") } else { py_str(".") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&lead.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_share_spoons(&[value.clone(), Py::Int(16)]), -1, "")); __s.push_str(", about "); __s.push_str(&fstr(&millilitres.clone(), -1, "")); __s.push_str(" ml"); __s.push_str(&fstr(&advice.clone(), -1, "")); py_string(__s) };
    }
    said = { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); py_string(__s) };
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("tablespoon"), py_str("tbsp")])])) {
    millilitres = call_builtin("round", &[op_mul(&value.clone(), &Py::Int(15))]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" of a tablespoon is "); __s.push_str(&fstr(&_tsp_phrase(&[op_mul(&value.clone(), &Py::Int(3))]), -1, "")); __s.push_str(", about "); __s.push_str(&fstr(&millilitres.clone(), -1, "")); __s.push_str(" ml."); py_string(__s) };
    }
    millilitres = call_builtin("round", &[op_mul(&value.clone(), &Py::Int(5))]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" of a teaspoon is about "); __s.push_str(&fstr(&millilitres.clone(), -1, "")); __s.push_str(" ml."); py_string(__s) };
    Py::None
}
pub fn _tsp_phrase(args: &[Py]) -> Py {
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
    let mut whole: Py = Py::None;
    let _ = py_str("A count of teaspoons, rounded the way a cook says it.");
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&count.clone(), &call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.05))) {
    whole = call_builtin("round", &[count.clone()]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(!py_eq(&whole.clone(), &Py::Int(1)))) { { let mut __s = String::new(); __s.push_str(&fstr(&whole.clone(), -1, "")); __s.push_str(" teaspoons"); py_string(__s) } } else { py_str("a teaspoon") };
    }
    return { let mut __s = String::new(); __s.push_str("about "); __s.push_str(&fstr(&call_builtin("round", &[count.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(" teaspoons"); py_string(__s) };
    Py::None
}
pub fn _a_fraction_value(args: &[Py]) -> Py {
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
    let mut denominator: Py = Py::None;
    let mut numeral: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut slash: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("What a fraction word is worth: \"a quarter\" 0.25, \"three quarters\" 0.75, \".75\" 0.75.");
    word = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("-"), py_str(" ")]), "split", &[])]), "lower", &[]);
    slash = stdlib_call("re", "fullmatch", &[py_str("(\\d+)\\s*/\\s*(\\d+)"), mcall(&word.clone(), "replace", &[py_str(" "), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = slash.clone(); if !py_truthy(&__b) { __b } else { call_builtin("int", &[mcall(&slash.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)) } }) {
    value = op_truediv(&call_builtin("int", &[mcall(&slash.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("int", &[mcall(&slash.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)));
    return if py_truthy(&op_lt(&value.clone(), &Py::Int(1))) { value.clone() } else { Py::None };
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d*\\.\\d+"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    value = call_builtin("float", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&op_lt(&value.clone(), &Py::Int(1))) { value.clone() } else { Py::None };
    }
    parts = mcall(&word.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&parts.clone(), &py_list(vec![py_str("half")])))) {
    return Py::Float(0.5);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)))) {
    numeral = mcall(&g__NUMERAL(), "get", &[geti(&parts.clone(), &Py::Int(0))]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&numeral.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&geti(&parts.clone(), &Py::Int(0)), "isdigit", &[]) } }) {
    numeral = call_builtin("int", &[geti(&parts.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e));
    }
    denominator = mcall(&g__DENOMINATOR(), "get", &[geti(&parts.clone(), &Py::Int(1))]);
    if py_truthy(&{ let __b = numeral.clone(); if !py_truthy(&__b) { __b } else { denominator.clone() } }) {
    value = op_truediv(&numeral.clone(), &denominator.clone());
    return if py_truthy(&op_lt(&value.clone(), &Py::Int(1))) { value.clone() } else { Py::None };
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_fraction_name(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut denominator: Py = Py::None;
    let mut name: Py = Py::None;
    let mut numerator: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The name a person gives a fraction: 1/3 is \"a third\", 3/4 \"three quarters\".");
    name = _share_name(&[value.clone()]);
    if py_truthy(&name.clone()) {
    return name.clone();
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![Py::Int(1), Py::Int(7), py_str("a seventh")]), py_tuple(vec![Py::Int(1), Py::Int(9), py_str("a ninth")]), py_tuple(vec![Py::Int(2), Py::Int(7), py_str("two sevenths")]), py_tuple(vec![Py::Int(1), Py::Int(10), py_str("a tenth")]), py_tuple(vec![Py::Int(1), Py::Int(12), py_str("a twelfth")]), py_tuple(vec![Py::Int(1), Py::Int(24), py_str("a twenty-fourth")]), py_tuple(vec![Py::Int(5), Py::Int(6), py_str("five sixths")]), py_tuple(vec![Py::Int(5), Py::Int(8), py_str("five eighths")]), py_tuple(vec![Py::Int(3), Py::Int(8), py_str("three eighths")]), py_tuple(vec![Py::Int(7), Py::Int(8), py_str("seven eighths")]), py_tuple(vec![Py::Int(1), Py::Int(20), py_str("a twentieth")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    numerator = __unpk2[0].clone();
    denominator = __unpk2[1].clone();
    said = __unpk2[2].clone();
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &op_truediv(&numerator.clone(), &denominator.clone()))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.001))) {
    return said.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_span_amount(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut amount: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "amount") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: amount"))) };
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut whole: Py = Py::None;
    let _ = py_str("15 minutes, one minute, 7 and a half minutes.");
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&amount.clone(), &call_builtin("round", &[amount.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.0001))) {
    whole = call_builtin("int", &[call_builtin("round", &[amount.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return if py_truthy(&Py::Bool(!py_eq(&whole.clone(), &Py::Int(1)))) { { let mut __s = String::new(); __s.push_str(&fstr(&whole.clone(), -1, &",")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str("one "); __s.push_str(&fstr(&slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), -1, "")); py_string(__s) } };
    }
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&op_modulo(&amount.clone(), &Py::Int(1)), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.0001))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&call_builtin("int", &[amount.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(" and a half "); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn _a_number_word(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("Two, three, four: the way a span of time is said out loud.");
    words = py_dict(vec![(Py::Int(1), py_str("one")), (Py::Int(2), py_str("two")), (Py::Int(3), py_str("three")), (Py::Int(4), py_str("four")), (Py::Int(5), py_str("five")), (Py::Int(6), py_str("six")), (Py::Int(7), py_str("seven")), (Py::Int(8), py_str("eight")), (Py::Int(9), py_str("nine")), (Py::Int(10), py_str("ten")), (Py::Int(11), py_str("eleven")), (Py::Int(12), py_str("twelve"))]);
    return mcall(&words.clone(), "get", &[call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), { let mut __s = String::new(); __s.push_str(&fstr(&call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); py_string(__s) }]);
    Py::None
}
pub fn _a_share_of_a_unit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut article: Py = Py::None;
    let mut name: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut rest_name: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("A quarter of an hour, half an hour, an hour and a half, two and a half hours.");
    article = geti(&g__TIME_ARTICLE(), &unit.clone());
    plural = op_add(&geti(&mcall(&article.clone(), "split", &[]), &op_neg(&Py::Int(1))), &py_str("s"));
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&value.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.0001))) {
    return { let mut __s = String::new(); __s.push_str("Half "); __s.push_str(&fstr(&article.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&op_gt(&value.clone(), &op_sub(&Py::Int(1), &Py::Float(0.0001)))) {
    whole = call_builtin("int", &[value.clone()]).unwrap_or_else(|e| exc::raise(e));
    rest = call_builtin("round", &[op_sub(&value.clone(), &whole.clone()), Py::Int(4)]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&op_lt(&call_builtin("abs", &[rest.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.0001))) {
    if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) {
    return op_add(&mcall(&geti(&article.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&article.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_a_number_word(&[whole.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&plural.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&rest.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.0001))) {
    if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&article.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&article.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" and a half"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_a_number_word(&[whole.clone()]), -1, "")); __s.push_str(" and a half "); __s.push_str(&fstr(&plural.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) {
    rest_name = { let __b = _a_fraction_name(&[rest.clone()]); if py_truthy(&__b) { __b } else { { let mut __s = String::new(); __s.push_str(&fstr(&rest.clone(), -1, &"g")); py_string(__s) } } };
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&article.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&article.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&rest_name.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_a_number_word(&[whole.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&plural.clone(), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&mcall(&_a_share_of_a_unit(&[rest.clone(), unit.clone()]), "lower", &[]), -1, "")); py_string(__s) };
    }
    name = { let __b = _a_fraction_name(&[value.clone()]); if py_truthy(&__b) { __b } else { { let mut __s = String::new(); __s.push_str(&fstr(&value.clone(), -1, &"g")); py_string(__s) } } };
    if py_truthy(&mcall(&name.clone(), "startswith", &[py_str("a ")])) {
    return { let mut __s = String::new(); __s.push_str("A "); __s.push_str(&fstr(&slice_or_raise(&name.clone(), &Py::Int(2), &Py::None, &Py::None), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&article.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&mcall(&name.clone(), "startswith", &[py_str("an ")])) {
    return { let mut __s = String::new(); __s.push_str("An "); __s.push_str(&fstr(&slice_or_raise(&name.clone(), &Py::Int(3), &Py::None, &Py::None), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&article.clone(), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&name.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&article.clone(), -1, "")); py_string(__s) };
    Py::None
}
pub fn _a_fraction_of_a_time_unit(args: &[Py]) -> Py {
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
    let mut amount: Py = Py::None;
    let mut count: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut per_small: Py = Py::None;
    let mut percent: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let mut small: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The fraction one span is of another: 20 minutes is a third of an hour.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g__A_FRACTION_OF(), g__A_FRACTION_AS()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    unit = mcall(&match_.clone(), "group", &[py_str("unit")]);
    small = mcall(&g__TIME_UNIT(), "get", &[mcall(&match_.clone(), "group", &[py_str("small")])]);
    amount = mcall(&match_.clone(), "group", &[py_str("amount")]);
    count = mcall(&g__NUMERAL(), "get", &[amount.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_is(&count.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "fullmatch", &[py_str("\\d+(?:\\.\\d+)?"), amount.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    count = call_builtin("float", &[amount.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    per_small = mcall(&mcall(&py_dict(vec![(py_str("hour"), py_dict(vec![(py_str("minute"), Py::Int(60))])), (py_str("day"), py_dict(vec![(py_str("hour"), Py::Int(24))])), (py_str("week"), py_dict(vec![(py_str("day"), Py::Int(7))])), (py_str("minute"), py_dict(vec![(py_str("second"), Py::Int(60))]))]), "get", &[unit.clone(), py_dict(vec![])]), "get", &[small.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&count.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&per_small.clone())) } }) {
    continue;
    }
    plural = geti(&py_dict(vec![(py_str("minute"), py_str("minutes")), (py_str("hour"), py_str("hours")), (py_str("second"), py_str("seconds")), (py_str("day"), py_str("days"))]), &small.clone());
    said = _a_span_amount(&[count.clone(), plural.clone()]);
    name = _a_fraction_name(&[op_truediv(&count.clone(), &per_small.clone())]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    percent = call_builtin("round", &[op_truediv(&op_mul(&Py::Int(100), &count.clone()), &per_small.clone())]).unwrap_or_else(|e| exc::raise(e));
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&percent.clone(), -1, "")); __s.push_str("% of "); __s.push_str(&fstr(&geti(&g__TIME_ARTICLE(), &unit.clone()), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), -1, "")); __s.push_str(&fstr(&slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&geti(&g__TIME_ARTICLE(), &unit.clone()), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return Py::None;
    Py::None
}
pub fn _the_span_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut span_seconds: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "span_seconds") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: span_seconds"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut clean: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut asked: Py = Py::None;
    let mut factor: Py = Py::None;
    let mut name: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("The span in the unit the line asked for, or in the next unit down.");
    let __unpk1 = py_unpack(&geti(&g__TIME_SMALLER(), &unit.clone()), 2, None);
    factor = __unpk1[0].clone();
    name = __unpk1[1].clone();
    asked = mcall(&g__A_UNIT_ASKED(), "search", &[clean.clone()]);
    if py_truthy(&asked.clone()) {
    wanted = mcall(&g__TIME_UNIT(), "get", &[mcall(&asked.clone(), "group", &[Py::Int(1)])]);
    if py_truthy(&{ let __b = wanted.clone(); if !py_truthy(&__b) { __b } else { op_lt(&geti(&g__TIME_SECONDS(), &wanted.clone()), &geti(&g__TIME_SECONDS(), &unit.clone())) } }) {
    return _a_span_amount(&[op_truediv(&span_seconds.clone(), &geti(&g__TIME_SECONDS(), &wanted.clone())), geti(&g__TIME_PLURAL(), &wanted.clone())]);
    }
    }
    return _a_span_amount(&[op_truediv(&span_seconds.clone(), &op_truediv(&geti(&g__TIME_SECONDS(), &unit.clone()), &factor.clone())), name.clone()]);
    Py::None
}
pub fn _a_mixed_parts(args: &[Py]) -> Py {
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
    let mut said_n: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The whole hours, the leftover as a share of one, and the unit, of a mixed span.");
    unit = geti(&g__TIME_UNIT(), &mcall(&match_.clone(), "group", &[py_str("unit")]));
    whole = mcall(&g__NUMERAL(), "get", &[mcall(&match_.clone(), "group", &[py_str("whole")])]);
    if py_truthy(&Py::Bool(py_is(&whole.clone(), &Py::None))) {
    whole = call_builtin("float", &[mcall(&match_.clone(), "group", &[py_str("whole")])]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&mcall(&match_.clone(), "groupdict", &[]), &py_str("rest"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("a half"), py_str("half")]), &mcall(&mcall(&match_.clone(), "group", &[py_str("rest")]), "strip", &[]))) } }) {
    return py_tuple(vec![unit.clone(), whole.clone(), Py::Float(0.5)]);
    }
    said_n = mcall(&g__NUMERAL(), "get", &[mcall(&match_.clone(), "group", &[py_str("n")])]);
    return py_tuple(vec![unit.clone(), whole.clone(), op_truediv(&if py_truthy(&Py::Bool(!py_is(&said_n.clone(), &Py::None))) { said_n.clone() } else { call_builtin("float", &[mcall(&match_.clone(), "group", &[py_str("n")])]).unwrap_or_else(|e| exc::raise(e)) }, &Py::Int(60))]);
    Py::None
}
pub fn _a_span_of_time(args: &[Py]) -> Py {
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
    let mut asks: Py = Py::None;
    let mut extra: Py = Py::None;
    let mut half_first: Py = Py::None;
    let mut head: Py = Py::None;
    let mut late: Py = Py::None;
    let mut late_extra: Py = Py::None;
    let mut late_unit: Py = Py::None;
    let mut late_whole: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut mixed: Py = Py::None;
    let mut part: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let mut share: Py = Py::None;
    let mut target: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The span of time the line asks for, in the smaller unit, or the fraction of the unit.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(200)) } } } }) {
    return Py::None;
    }
    asks = mcall(&g__A_SPAN_ASK(), "search", &[clean.clone()]);
    target = mcall(&g__A_TARGET_UNIT(), "search", &[clean.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asks.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&target.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g__A_FRACTION_ASK(), "search", &[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g__A_FRACTION_AS(), "search", &[clean.clone()]))) } } } } } }) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bfractions?\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    said = _a_fraction_of_a_time_unit(&[clean.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    share = mcall(&g__A_FRACTION_OF_A_MIXED(), "search", &[clean.clone()]);
    if py_truthy(&share.clone()) {
    late = { let __b = mcall(&g__A_MIXED_SPAN(), "search", &[mcall(&share.clone(), "group", &[py_str("span")])]); if py_truthy(&__b) { __b } else { mcall(&g__A_HALF_SPAN(), "search", &[mcall(&share.clone(), "group", &[py_str("span")])]) } };
    value = _a_fraction_value(&[mcall(&share.clone(), "group", &[py_str("word")])]);
    if py_truthy(&{ let __b = late.clone(); if !py_truthy(&__b) { __b } else { value.clone() } }) {
    let __unpk1 = py_unpack(&_a_mixed_parts(&[late.clone()]), 3, None);
    late_unit = __unpk1[0].clone();
    late_whole = __unpk1[1].clone();
    late_extra = __unpk1[2].clone();
    part = op_mul(&op_add(&late_whole.clone(), &late_extra.clone()), &value.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&_a_share_of_a_unit(&[part.clone(), late_unit.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_the_span_said(&[op_mul(&part.clone(), &geti(&g__TIME_SECONDS(), &late_unit.clone())), late_unit.clone(), clean.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    }
    mixed = mcall(&g__A_MIXED_SPAN(), "search", &[clean.clone()]);
    half_first = if py_truthy(&Py::Bool(!py_truthy(&mixed.clone()))) { mcall(&g__A_HALF_SPAN(), "search", &[clean.clone()]) } else { Py::None };
    if py_truthy(&{ let __b = mixed.clone(); if py_truthy(&__b) { __b } else { half_first.clone() } }) {
    let __unpk2 = py_unpack(&_a_mixed_parts(&[{ let __b = mixed.clone(); if py_truthy(&__b) { __b } else { half_first.clone() } }]), 3, None);
    unit = __unpk2[0].clone();
    whole = __unpk2[1].clone();
    extra = __unpk2[2].clone();
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&extra.clone(), &Py::Float(0.5))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.0001))) {
    said = _a_share_of_a_unit(&[op_add(&whole.clone(), &Py::Float(0.5)), unit.clone()]);
    } else {
    plural = op_add(&geti(&mcall(&geti(&g__TIME_ARTICLE(), &unit.clone()), "split", &[]), &op_neg(&Py::Int(1))), &py_str("s"));
    head = if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) { op_add(&mcall(&geti(&geti(&g__TIME_ARTICLE(), &unit.clone()), &Py::Int(0)), "upper", &[]), &slice_or_raise(&geti(&g__TIME_ARTICLE(), &unit.clone()), &Py::Int(1), &Py::None, &Py::None)) } else { { let mut __s = String::new(); __s.push_str(&fstr(&_a_number_word(&[whole.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&plural.clone(), -1, "")); py_string(__s) } };
    said = { let mut __s = String::new(); __s.push_str(&fstr(&head.clone(), -1, "")); __s.push_str(" and "); __s.push_str(&fstr(&_a_span_amount(&[op_mul(&extra.clone(), &Py::Int(60)), py_str("minutes")]), -1, "")); py_string(__s) };
    }
    said = op_add(&mcall(&geti(&said.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_the_span_said(&[op_mul(&op_add(&whole.clone(), &extra.clone()), &geti(&g__TIME_SECONDS(), &unit.clone())), unit.clone(), clean.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&target.clone()) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&asks.clone()))) {
    return Py::None;
    }
    whole = mcall(&g__A_WHOLE_UNIT(), "match", &[clean.clone()]);
    if py_truthy(&whole.clone()) {
    unit = geti(&g__TIME_UNIT(), &mcall(&whole.clone(), "group", &[py_str("unit")]));
    return { let mut __s = String::new(); __s.push_str(&fstr(&_a_share_of_a_unit(&[Py::Int(1), unit.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_the_span_said(&[geti(&g__TIME_SECONDS(), &unit.clone()), unit.clone(), clean.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    }
    match_ = mcall(&g__A_TIME_SHARE(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    unit = geti(&g__TIME_UNIT(), &mcall(&match_.clone(), "group", &[py_str("unit")]));
    value = _a_fraction_value(&[mcall(&match_.clone(), "group", &[py_str("word")])]);
    if py_truthy(&Py::Bool(!py_truthy(&value.clone()))) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "match", &[py_str("\\s*and\\s+a\\s+half\\b"), slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &op_add(&mcall(&match_.clone(), "end", &[]), &Py::Int(14)), &Py::None)]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug3 = op_add(&value.clone(), &Py::Float(0.5));
    value = __aug3;
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_a_share_of_a_unit(&[value.clone(), unit.clone()]), -1, "")); __s.push_str(" is "); __s.push_str(&fstr(&_the_span_said(&[op_mul(&value.clone(), &geti(&g__TIME_SECONDS(), &unit.clone())), unit.clone(), clean.clone()]), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _share_carried(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut share: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("A fraction of the measure the last turn was about: \"and half of that\".");
    match_ = mcall(&g__SHARE_THAT(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    rest = op_add(&op_add(&slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), &py_str(" ")), &slice_or_raise(&clean.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None));
    rest = mcall(&rest.clone(), "strip", &[py_str(" ?.!,andso")]);
    while py_truthy(&mcall(&g__SHARE_LEAD(), "match", &[rest.clone()])) {
    rest = mcall(&mcall(&g__SHARE_LEAD(), "sub", &[py_str(""), rest.clone()]), "strip", &[py_str(" ?.!,andso")]);
    }
    if py_truthy(&rest.clone()) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    share = mcall(&g__SHARE(), "search", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&share.clone()))) {
    if py_truthy(&line.clone()) {
    return Py::None;
    }
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&match_.clone(), "group", &[py_str("word")]), "split", &[])]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&py_str("what is "), &said.clone()), &py_str(" of ")), &slice_or_raise(&line.clone(), &mcall(&share.clone(), "start", &[]), &Py::None, &Py::None))); __kws.extend(vec![("tail".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _share_of_a_measure(args))(&__args) };
    }
    return Py::None;
    Py::None
}
pub fn convert(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let _ = py_str("A converted measure as a sentence, or None when the line is not a conversion.");
    reply = _answer(&[clean.clone(), memory.clone()]);
    if py_truthy(&reply.clone()) {
    return reply.clone();
    }
    if py_truthy(&memory.clone()) {
    return { let __b = _share_carried(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _carried_weight(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _carried_measure(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _carried_people(&[clean.clone(), memory.clone()]) } } } } } };
    }
    return Py::None;
    Py::None
}
pub fn _count_in_a_line(args: &[Py]) -> Py {
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
    let mut _name: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut best: Py = Py::None;
    let mut count: Py = Py::None;
    let mut each: Py = Py::None;
    let mut found: Py = Py::None;
    let mut halves: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut portions: Py = Py::None;
    let _ = py_str("How many portions a counted number of pieces makes: \"6 eggs\", \"12 sausages\".");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__PORTIONS_ASK(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_PER_PERSON());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    names = __unpk2[0].clone();
    amount = __unpk2[1].clone();
    kind = __unpk2[2].clone();
    noun = __unpk2[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("each"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&amount.clone())) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    found = stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b(\\d+)\\s+(?:[a-z]+\\s+)?"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    best = py_tuple(vec![name.clone(), call_builtin("int", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e)), amount.clone(), noun.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return Py::None;
    }
    let __unpk4 = py_unpack(&best.clone(), 4, None);
    _name = __unpk4[0].clone();
    count = __unpk4[1].clone();
    amount = __unpk4[2].clone();
    noun = __unpk4[3].clone();
    if py_truthy(&{ let __b = op_le(&count.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_gt(&count.clone(), &Py::Int(200)) } }) {
    return Py::None;
    }
    portions = op_truediv(&count.clone(), &amount.clone());
    halves = op_truediv(&call_builtin("round", &[op_mul(&portions.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    piece = if py_truthy(&{ let __b = mcall(&noun.clone(), "endswith", &[py_str("s")]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&noun.clone(), &py_str(" "))) } }) { noun.clone() } else { op_add(&noun.clone(), &py_str("s")) };
    each = if py_truthy(&Py::Bool(py_eq(&amount.clone(), &Py::Int(1)))) { py_str("one") } else { { let mut __s = String::new(); __s.push_str(&fstr(&amount.clone(), -1, &"g")); py_string(__s) } };
    if py_truthy(&op_le(&halves.clone(), &Py::Int(1))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&piece.clone(), -1, "")); __s.push_str(" is about one portion: "); __s.push_str(&fstr(&each.clone(), -1, "")); __s.push_str(" a person is the usual allowance."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&piece.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&halves.clone(), -1, &"g")); __s.push_str(" portions: "); __s.push_str(&fstr(&each.clone(), -1, "")); __s.push_str(" a person is the usual allowance."); py_string(__s) };
    Py::None
}
pub fn _portions_in_a_weight(args: &[Py]) -> Py {
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
    let mut _name: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut best: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut halves: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut portions: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut weight: Py = Py::None;
    let _ = py_str("How many people a stated weight of a known food feeds.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clean.clone())) } }) {
    return Py::None;
    }
    match_ = mcall(&g__WEIGHT_SAID(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    counted = _count_in_a_line(&[clean.clone()]);
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    }
    if py_truthy(&match_.clone()) {
    grams = call_builtin("float", &[mcall(&match_.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    unit = mcall(&mcall(&match_.clone(), "group", &[Py::Int(2)]), "lower", &[]);
    } else {
    bare = stdlib_call("re", "search", &[py_str("\\b(half\\s+)?(?:a|an|one)?\\s*(kg|kilo|kilos|kilogram|kilograms|gram|grams|gramme|grammes)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::None;
    }
    grams = if py_truthy(&mcall(&bare.clone(), "group", &[Py::Int(1)])) { Py::Float(0.5) } else { Py::Float(1.0) };
    unit = mcall(&mcall(&bare.clone(), "group", &[Py::Int(2)]), "lower", &[]);
    }
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("k"), py_str("kilo")])])) {
    let __aug1 = op_mul(&grams.clone(), &Py::Int(1000));
    grams = __aug1;
    }
    if py_truthy(&{ let __b = op_lt(&grams.clone(), &Py::Int(20)); if py_truthy(&__b) { __b } else { op_gt(&grams.clone(), &Py::Int(20000)) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__PORTIONS_ASK(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__PORTION_WORD(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    best = Py::None;
    let __iter2 = py_iter_or_raise(&g_PER_PERSON());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 4, None);
    names = __unpk3[0].clone();
    amount = __unpk3[1].clone();
    kind = __unpk3[2].clone();
    noun = __unpk3[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("g"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&amount.clone())) } }) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), amount.clone(), noun.clone()]);
    }
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return Py::None;
    }
    let __unpk5 = py_unpack(&best.clone(), 3, None);
    _name = __unpk5[0].clone();
    amount = __unpk5[1].clone();
    noun = __unpk5[2].clone();
    weight = if py_truthy(&{ let __b = mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("k"), py_str("kilo")])]); if !py_truthy(&__b) { __b } else { op_ge(&grams.clone(), &Py::Int(1000)) } }) { { let mut __s = String::new(); __s.push_str(&fstr(&op_truediv(&grams.clone(), &Py::Int(1000)), -1, &"g")); __s.push_str(" kg"); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str(&fstr(&grams.clone(), -1, &"g")); __s.push_str(" g"); py_string(__s) } };
    portions = op_truediv(&grams.clone(), &amount.clone());
    if py_truthy(&op_lt(&portions.clone(), &Py::Float(0.75))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is not a full portion on its own: allow about "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person."); py_string(__s) };
    }
    halves = op_truediv(&call_builtin("round", &[op_mul(&portions.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    said = mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&halves.clone(), -1, &"g")); py_string(__s) }, "replace", &[py_str(".5"), py_str(" and a half")]);
    if py_truthy(&op_le(&halves.clone(), &Py::Int(1))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about one portion: the usual allowance is "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" portions: the usual allowance is "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person, so it feeds "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _a_table_word(args: &[Py]) -> Py {
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
    let _ = py_str("The number of people as words, as far as the table's own words go.");
    return { let __b = mcall(&g__SAID_PEOPLE(), "get", &[count.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g__PEOPLE_WORD_TO_TWENTY(), "get", &[count.clone()]); if py_truthy(&__b) { __b } else { call_builtin("str", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _birds_for_a_table(args: &[Py]) -> Py {
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
    let mut allowance: Py = Py::None;
    let mut feeds: Py = Py::None;
    let mut feeds_had: Py = Py::None;
    let mut found: Py = Py::None;
    let mut grams_each: Py = Py::None;
    let mut had: Py = Py::None;
    let mut have: Py = Py::None;
    let mut keeps: Py = Py::None;
    let mut kilos: Py = Py::None;
    let mut many: Py = Py::None;
    let mut meat: Py = Py::None;
    let mut more: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut needs: Py = Py::None;
    let mut one: Py = Py::None;
    let mut said: Py = Py::None;
    let mut size: Py = Py::None;
    let mut stretch: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How many whole birds a table needs, or whether the number in the line covers it.\n\n    \"how many chickens for 20 people\" was a punt and \"do i need two chickens for 8\" was a hedge.\n    Both are one sum with the bird's own weight in it, so they are answered by the same row that\n    says what one feeds.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(200)) } } } }) {
    return Py::None;
    }
    found = mcall(&g__COUNT_OF_BIRDS(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__NOT_A_TABLE_QUESTION(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__KEEPING_HENS(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    wanted = _people_asked(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&wanted.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&py_tuple(vec![mcall(&found.clone(), "group", &[Py::Int(1)]), mcall(&found.clone(), "group", &[Py::Int(2)])]), 2, None);
    word = __unpk1[0].clone();
    name = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&g_WHOLE_THING_SERVES());
    let mut __else3 = false;
    for __item in __iter2 {
    let __unpk4 = py_unpack(&__item.clone(), 8, None);
    names = __unpk4[0].clone();
    size = __unpk4[1].clone();
    feeds = __unpk4[2].clone();
    stretch = __unpk4[3].clone();
    more = __unpk4[4].clone();
    grams_each = __unpk4[5].clone();
    allowance = __unpk4[6].clone();
    keeps = __unpk4[7].clone();
    if py_truthy(&Py::Bool(py_contains(&names.clone(), &name.clone()))) {
    break;
    }
    }
    return Py::None;
    meat = geti(&names.clone(), &Py::Int(0));
    one = op_mul(&grams_each.clone(), &feeds.clone());
    said = _a_table_word(&[wanted.clone()]);
    needs = op_neg(&op_floordiv(&op_mul(&op_neg(&wanted.clone()), &grams_each.clone()), &one.clone()));
    if py_truthy(&Py::Bool(py_eq(&word.clone(), &py_str("how many")))) {
    kilos = op_truediv(&op_mul(&wanted.clone(), &grams_each.clone()), &Py::Float(1000.0));
    if py_truthy(&Py::Bool(py_eq(&needs.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str("For "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" people you want about "); __s.push_str(&fstr(&kilos.clone(), -1, &"g")); __s.push_str("kg of "); __s.push_str(&fstr(&meat.clone(), -1, "")); __s.push_str(" on the bone, and one bird of "); __s.push_str(&fstr(&size.clone(), -1, "")); __s.push_str(" is enough."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("For "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" people you want about "); __s.push_str(&fstr(&kilos.clone(), -1, &"g")); __s.push_str("kg of "); __s.push_str(&fstr(&meat.clone(), -1, "")); __s.push_str(" on the bone: "); __s.push_str(&fstr(&_a_table_word(&[needs.clone()]), -1, "")); __s.push_str(" birds of "); __s.push_str(&fstr(&size.clone(), -1, "")); __s.push_str(" each."); py_string(__s) };
    }
    have = _people_in_a_count(&[word.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&have.clone())); if py_truthy(&__b) { __b } else { op_lt(&have.clone(), &Py::Int(2)) } }) {
    return Py::None;
    }
    feeds_had = call_builtin("int", &[op_floordiv(&op_mul(&have.clone(), &one.clone()), &grams_each.clone())]).unwrap_or_else(|e| exc::raise(e));
    many = _a_table_word(&[have.clone()]);
    had = if py_truthy(&Py::Bool(py_eq(&have.clone(), &Py::Int(1)))) { py_str("is") } else { py_str("are") };
    kilos = op_truediv(&op_mul(&have.clone(), &one.clone()), &Py::Float(1000.0));
    if py_truthy(&Py::Bool(py_eq(&feeds_had.clone(), &wanted.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&many.clone(), "capitalize", &[]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&had.clone(), -1, "")); __s.push_str(" about "); __s.push_str(&fstr(&kilos.clone(), -1, &"g")); __s.push_str("kg on the bone, which is exactly enough for "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&op_gt(&feeds_had.clone(), &wanted.clone())) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&many.clone(), "capitalize", &[]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&had.clone(), -1, "")); __s.push_str(" about "); __s.push_str(&fstr(&kilos.clone(), -1, &"g")); __s.push_str("kg on the bone, enough for "); __s.push_str(&fstr(&mcall(&g__SAID_PEOPLE(), "get", &[feeds_had.clone(), call_builtin("str", &[feeds_had.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" - so yes for "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&many.clone(), "capitalize", &[]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(" feed about "); __s.push_str(&fstr(&mcall(&g__SAID_PEOPLE(), "get", &[feeds_had.clone(), call_builtin("str", &[feeds_had.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(", so "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" wants "); __s.push_str(&fstr(&mcall(&g__SAID_PEOPLE(), "get", &[needs.clone(), call_builtin("str", &[needs.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" birds."); py_string(__s) };
    Py::None
}
pub fn _people_in_a_count(args: &[Py]) -> Py {
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
    let _ = py_str("The number a word or a figure says, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&word.clone(), "isdigit", &[])) {
    return call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    return mcall(&g__PEOPLE_VALUE(), "get", &[word.clone()]);
    Py::None
}
pub fn _a_whole_thing_feeds(args: &[Py]) -> Py {
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
    let mut allowance: Py = Py::None;
    let mut best: Py = Py::None;
    let mut feeds: Py = Py::None;
    let mut found: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut grams_each: Py = Py::None;
    let mut keeps: Py = Py::None;
    let mut kilos: Py = Py::None;
    let mut more: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut reached: Py = Py::None;
    let mut said: Py = Py::None;
    let mut size: Py = Py::None;
    let mut smaller: Py = Py::None;
    let mut stretch: Py = Py::None;
    let mut stretched: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut weighed: Py = Py::None;
    let _ = py_str("How many people one whole bird, leg or joint feeds.\n\n    \"is one chicken enough for 8\" was a punt on main, and \"does one chicken feed four\" was\n    the hen-keeping card. One bird is one size, so the answer is a fact with the line's own\n    headcount turned into a yes, a yes-with-sides or a \"cook two\".\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(200)) } } } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g__FEEDS_ASK(), "search", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__NOT_A_TABLE_QUESTION(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__KEEPING_HENS(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__A_CUT_NOT_A_BIRD(), "search", &[clean.clone()])) {
    return Py::None;
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_WHOLE_THING_SERVES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 8, None);
    names = __unpk2[0].clone();
    size = __unpk2[1].clone();
    feeds = __unpk2[2].clone();
    stretch = __unpk2[3].clone();
    more = __unpk2[4].clone();
    grams_each = __unpk2[5].clone();
    allowance = __unpk2[6].clone();
    keeps = __unpk2[7].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    found = stdlib_call("re", "search", &[op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:one|1|a|an|the|single)\\s+(?:whole\\s+)?(?:\\d+(?:\\.\\d+)?\\s*kg\\s+)?"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b|\\bwhole\\s+(?:\\d+(?:\\.\\d+)?\\s*kg\\s+)?")), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    best = py_tuple(vec![name.clone(), size.clone(), feeds.clone(), stretch.clone(), more.clone(), grams_each.clone(), allowance.clone(), keeps.clone()]);
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return Py::None;
    }
    let __unpk4 = py_unpack(&best.clone(), 8, None);
    name = __unpk4[0].clone();
    size = __unpk4[1].clone();
    feeds = __unpk4[2].clone();
    stretch = __unpk4[3].clone();
    more = __unpk4[4].clone();
    grams_each = __unpk4[5].clone();
    allowance = __unpk4[6].clone();
    keeps = __unpk4[7].clone();
    noun = if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &geti(&name.clone(), &Py::Int(0))))) { op_add(&py_str("an "), &name.clone()) } else { op_add(&py_str("a "), &name.clone()) };
    weighed = mcall(&g__WHOLE_WEIGHT(), "search", &[clean.clone()]);
    if py_truthy(&weighed.clone()) {
    kilos = call_builtin("float", &[mcall(&weighed.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    grams = op_mul(&kilos.clone(), &Py::Int(1000));
    feeds = call_builtin("max", &[Py::Int(1), call_builtin("round", &[op_truediv(&grams.clone(), &grams_each.clone())]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    size = { let mut __s = String::new(); __s.push_str(&fstr(&kilos.clone(), -1, &"g")); __s.push_str("kg"); py_string(__s) };
    stretch = if py_truthy(&stretch.clone()) { call_builtin("round", &[op_mul(&feeds.clone(), &Py::Float(1.5))]).unwrap_or_else(|e| exc::raise(e)) } else { Py::None };
    }
    smaller = mcall(&g__SAID_PEOPLE(), "get", &[feeds.clone(), call_builtin("str", &[feeds.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    wanted = _people_asked(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&wanted.clone(), &Py::None))) {
    found = stdlib_call("re", "search", &[py_str("\\b(?:feed|feeds|serve|serves|enough for)\\s+(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    wanted = if py_truthy(&found.clone()) { _people_in_a_count(&[mcall(&found.clone(), "group", &[Py::Int(1)])]) } else { Py::None };
    }
    if py_truthy(&Py::Bool(py_is(&wanted.clone(), &Py::None))) {
    stretched = if py_truthy(&stretch.clone()) { { let mut __s = String::new(); __s.push_str(", "); __s.push_str(&fstr(&mcall(&g__SAID_PEOPLE(), "get", &[stretch.clone(), call_builtin("str", &[stretch.clone()]).unwrap_or_else(|e| exc::raise(e))]), -1, "")); __s.push_str(" at a stretch with plenty of sides"); py_string(__s) } } else { py_str("") };
    return { let mut __s = String::new(); __s.push_str("One "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&size.clone(), -1, "")); __s.push_str(", feeds "); __s.push_str(&fstr(&smaller.clone(), -1, "")); __s.push_str(" people"); __s.push_str(&fstr(&stretched.clone(), -1, "")); __s.push_str(&fstr(&keeps.clone(), -1, "")); __s.push_str(". A bigger table is "); __s.push_str(&fstr(&more.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    said = mcall(&g__SAID_PEOPLE(), "get", &[wanted.clone(), call_builtin("str", &[wanted.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&op_le(&wanted.clone(), &feeds.clone())) {
    return { let mut __s = String::new(); __s.push_str("One "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&size.clone(), -1, "")); __s.push_str(", feeds "); __s.push_str(&fstr(&smaller.clone(), -1, "")); __s.push_str(" people as a roast - so one is enough for "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(". Allow "); __s.push_str(&fstr(&allowance.clone(), -1, "")); __s.push_str(" if you are buying another."); py_string(__s) };
    }
    if py_truthy(&{ let __b = stretch.clone(); if !py_truthy(&__b) { __b } else { op_le(&wanted.clone(), &stretch.clone()) } }) {
    reached = mcall(&g__SAID_PEOPLE(), "get", &[stretch.clone(), call_builtin("str", &[stretch.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    return { let mut __s = String::new(); __s.push_str("One "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&size.clone(), -1, "")); __s.push_str(", feeds "); __s.push_str(&fstr(&smaller.clone(), -1, "")); __s.push_str(" comfortably and "); __s.push_str(&fstr(&reached.clone(), -1, "")); __s.push_str(" at a stretch with plenty of sides - so one is enough for "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" if you lay on the sides. Otherwise "); __s.push_str(&fstr(&more.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("One "); __s.push_str(&fstr(&name.clone(), -1, "")); __s.push_str(", "); __s.push_str(&fstr(&size.clone(), -1, "")); __s.push_str(", feeds "); __s.push_str(&fstr(&smaller.clone(), -1, "")); __s.push_str(", so for "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" you want "); __s.push_str(&fstr(&more.clone(), -1, "")); __s.push_str(". That is the "); __s.push_str(&fstr(&allowance.clone(), -1, "")); __s.push_str(" figure."); py_string(__s) };
    Py::None
}
pub fn _answer(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut butter: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut dish: Py = Py::None;
    let mut end: Py = Py::None;
    let mut family: Py = Py::None;
    let mut handful: Py = Py::None;
    let mut height: Py = Py::None;
    let mut land: Py = Py::None;
    let mut liquid: Py = Py::None;
    let mut loose: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut measured: Py = Py::None;
    let mut mentions: Py = Py::None;
    let mut own: Py = Py::None;
    let mut pack: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut portion: Py = Py::None;
    let mut portions: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut rule: Py = Py::None;
    let mut share: Py = Py::None;
    let mut source: Py = Py::None;
    let mut source_family: Py = Py::None;
    let mut source_unit: Py = Py::None;
    let mut span: Py = Py::None;
    let mut start: Py = Py::None;
    let mut temperature: Py = Py::None;
    let mut text: Py = Py::None;
    let mut to_family: Py = Py::None;
    let mut to_unit: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut want: Py = Py::None;
    let mut want_family: Py = Py::None;
    let mut want_unit: Py = Py::None;
    let mut wanted_match: Py = Py::None;
    let mut weighed: Py = Py::None;
    let mut weight: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The table's own answer to a line that carries its whole question.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[clean.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(200)) } } } }) {
    return Py::None;
    }
    whole = { let __b = _a_whole_thing_feeds(&[clean.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { _birds_for_a_table(&[clean.clone()]) } };
    if py_truthy(&whole.clone()) {
    return whole.clone();
    }
    measured = _the_thing_measured_by_itself(&[clean.clone()]);
    if py_truthy(&measured.clone()) {
    return _answer(&[measured.clone(), memory.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g__COMPONENT(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_the_component_is_the_whole_measure(&[clean.clone()]))) } }) {
    return Py::None;
    }
    share = _share_of_a_measure(&[clean.clone()]);
    if py_truthy(&share.clone()) {
    return share.clone();
    }
    span = _a_span_of_time(&[clean.clone()]);
    if py_truthy(&span.clone()) {
    return span.clone();
    }
    portions = _portions_in_a_weight(&[clean.clone()]);
    if py_truthy(&portions.clone()) {
    return portions.clone();
    }
    temperature = { let __b = _oven(&[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = _temperature(&[clean.clone()]); if py_truthy(&__b) { __b } else { _temperature_rule(&[clean.clone()]) } } } };
    if py_truthy(&temperature.clone()) {
    return temperature.clone();
    }
    butter = _butter(&[clean.clone()]);
    if py_truthy(&butter.clone()) {
    return butter.clone();
    }
    if py_truthy(&{ let __b = mcall(&g__PEOPLE_COUNT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__PEOPLE_WORD(), "search", &[clean.clone()]) } }) {
    dish = _a_dish_for_a_table(&[clean.clone()]);
    if py_truthy(&dish.clone()) {
    return dish.clone();
    }
    portion = _per_person(&[clean.clone(), memory.clone()]);
    if py_truthy(&portion.clone()) {
    return portion.clone();
    }
    }
    pack = _pack(&[clean.clone()]);
    if py_truthy(&pack.clone()) {
    return pack.clone();
    }
    counted = _pack_count(&[clean.clone()]);
    if py_truthy(&counted.clone()) {
    return counted.clone();
    }
    loose = _loose(&[clean.clone()]);
    if py_truthy(&loose.clone()) {
    return loose.clone();
    }
    weight = _object_weight(&[clean.clone()]);
    if py_truthy(&weight.clone()) {
    return weight.clone();
    }
    piece = _piece_weight(&[clean.clone()]);
    if py_truthy(&piece.clone()) {
    return piece.clone();
    }
    liquid = _liquid_weight(&[clean.clone()]);
    if py_truthy(&liquid.clone()) {
    return liquid.clone();
    }
    portion = _per_person(&[clean.clone(), memory.clone()]);
    if py_truthy(&portion.clone()) {
    return portion.clone();
    }
    clean = mcall(&g__SAY_K(), "sub", &[py_str("\\1 km"), clean.clone()]);
    clean = _frac(&[clean.clone()]);
    clean = mcall(&g__JOINED(), "sub", &[py_str(" "), clean.clone()]);
    handful = _handful(&[clean.clone()]);
    if py_truthy(&handful.clone()) {
    return handful.clone();
    }
    own = _own_height(&[clean.clone()]);
    if py_truthy(&own.clone()) {
    return own.clone();
    }
    mentions = _mentions(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mentions.clone()))) {
    return Py::None;
    }
    rule = _how_to_convert(&[clean.clone(), mentions.clone()]);
    if py_truthy(&rule.clone()) {
    return rule.clone();
    }
    height = _height(&[clean.clone(), mentions.clone()]);
    if py_truthy(&height.clone()) {
    return height.clone();
    }
    land = _land_size(&[clean.clone(), mentions.clone()]);
    if py_truthy(&land.clone()) {
    return land.clone();
    }
    measured = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mentions.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    start = __unpk2[0].clone();
    end = __unpk2[1].clone();
    family = __unpk2[2].clone();
    unit = __unpk2[3].clone();
    value = _number_before(&[clean.clone(), start.clone()]);
    if py_truthy(&Py::Bool(!py_is(&value.clone(), &Py::None))) {
    let _ = mcall(&measured.clone(), "append", &[py_tuple(vec![value.clone(), family.clone(), unit.clone(), end.clone()])]);
    }
    }
    weighed = _named_ingredient(&[clean.clone(), mentions.clone(), measured.clone()]);
    if py_truthy(&weighed.clone()) {
    return weighed.clone();
    }
    let __iter3 = py_iter_or_raise(&measured.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 4, None);
    value = __unpk4[0].clone();
    family = __unpk4[1].clone();
    unit = __unpk4[2].clone();
    end = __unpk4[3].clone();
    rest = slice_or_raise(&clean.clone(), &end.clone(), &Py::None, &Py::None);
    let __iter5 = py_iter_or_raise(&mcall(&g__UNIT_WORD(), "finditer", &[rest.clone()]));
    for __item in __iter5 {
    match_ = __item.clone();
    text = mcall(&match_.clone(), "group", &[Py::Int(0)]);
    before = slice_or_raise(&rest.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&text.clone(), &py_str("in"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g__TARGET(), "search", &[before.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("how (?:many|much)\\s+$"), before.clone()]).unwrap_or_else(|e| exc::raise(e)) } })) } }) {
    continue;
    }
    let __unpk6 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(family.clone()); __args.push(unit.clone()); __args.extend(py_iter_or_raise(&geti(&g__NAMES(), &text.clone()))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_ounce_pair(args))(&__args) }, 4, None);
    family = __unpk6[0].clone();
    unit = __unpk6[1].clone();
    to_family = __unpk6[2].clone();
    to_unit = __unpk6[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&to_family.clone(), &family.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&to_unit.clone(), &unit.clone())) } }) {
    continue;
    }
    if py_truthy(&mcall(&g__UNIT_WORD(), "search", &[slice_or_raise(&rest.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])) {
    break;
    }
    return _sentence(&[value.clone(), family.clone(), unit.clone(), to_unit.clone()]);
    }
    }
    wanted_match = stdlib_call("re", "search", &[py_str("how (?:many|much)\\s+(?P<want>[a-z/'’ ]+?)\\s+(?:are there\\s+)?(?:in|to|into|is|as)\\s+(?P<rest>.+)$"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&wanted_match.clone()) {
    want = _named_unit(&[mcall(&wanted_match.clone(), "group", &[py_str("want")])]);
    source = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&measured.clone()) { let __unpk950 = py_unpack(&__item.clone(), 4, None); let mut value: Py = __unpk950[0].clone(); let mut family: Py = __unpk950[1].clone(); let mut unit: Py = __unpk950[2].clone(); let mut end: Py = __unpk950[3].clone(); if py_truthy(&op_gt(&end.clone(), &mcall(&wanted_match.clone(), "start", &[py_str("rest")]))) {  __out.push(py_tuple(vec![value.clone(), family.clone(), unit.clone(), end.clone()])); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = source.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_tail_is_about_the_measure(&[slice_or_raise(&clean.clone(), &geti(&source.clone(), &Py::Int(3)), &Py::None, &Py::None)]))) } }) {
    source = Py::None;
    }
    if py_truthy(&{ let __b = want.clone(); if !py_truthy(&__b) { __b } else { source.clone() } }) {
    let __unpk7 = py_unpack(&_the_ounce_pair(&[geti(&want.clone(), &Py::Int(0)), geti(&want.clone(), &Py::Int(1)), geti(&source.clone(), &Py::Int(1)), geti(&source.clone(), &Py::Int(2))]), 4, None);
    want_family = __unpk7[0].clone();
    want_unit = __unpk7[1].clone();
    source_family = __unpk7[2].clone();
    source_unit = __unpk7[3].clone();
    want = py_tuple(vec![want_family.clone(), want_unit.clone()]);
    source = py_tuple(vec![geti(&source.clone(), &Py::Int(0)), source_family.clone(), source_unit.clone(), geti(&source.clone(), &Py::Int(3))]);
    }
    if py_truthy(&{ let __b = want.clone(); if !py_truthy(&__b) { __b } else { { let __b = source.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&want.clone(), &Py::Int(0)), &geti(&source.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&want.clone(), &Py::Int(1)), &geti(&source.clone(), &Py::Int(2)))) } } } } } }) {
    if py_truthy(&{ let __b = Py::Bool(py_eq(&geti(&source.clone(), &Py::Int(1)), &py_str("mass"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("cup"), py_str("tbsp"), py_str("tsp")]), &geti(&source.clone(), &Py::Int(2)))) } }) {
    return _cup_of_something(&[clean.clone()]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&source.clone(), &Py::Int(0))); __args.push(geti(&source.clone(), &Py::Int(1))); __args.push(geti(&source.clone(), &Py::Int(2))); __args.push(geti(&want.clone(), &Py::Int(1))); __kws.extend(vec![("in_phrase".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _sentence(args))(&__args) };
    }
    if py_truthy(&{ let __b = want.clone(); if !py_truthy(&__b) { __b } else { { let __b = source.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("cup"), py_str("tbsp"), py_str("tsp"), py_str("dessertspoon")]), &geti(&want.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&source.clone(), &Py::Int(1)), &py_str("volume"))) } } } } } }) {
    return _cup_of_something(&[clean.clone()]);
    }
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(cups?|mugs?|tablespoons?|tbsp|teaspoons?|tsp|dessertspoons?|dsp)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return _cup_of_something(&[clean.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _unit_words(args: &[Py]) -> Py {
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
    let mut match_: Py = Py::None;
    let _ = py_str("Every unit word a line names, in the order written.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g__UNIT_WORD(), "finditer", &[text.clone()])) { let mut match_: Py = __item.clone(); __out.push(py_tuple(vec![mcall(&match_.clone(), "group", &[Py::Int(0)]), geti(&g__NAMES(), &mcall(&match_.clone(), "group", &[Py::Int(0)]))])); } py_list(__out) })() };
    Py::None
}
pub fn _bare_measure(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut food: Py = Py::None;
    let mut said: Py = Py::None;
    let mut spare: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The unit and food a bare measure phrase names, or None when it says anything more.\n\n    A follow-up is the unit, the food and the grammar between them (\"and in a tablespoon\",\n    \"what about a cup of sugar\"). A line with a verb or a subject of its own is a new question\n    (\"and how long does it keep?\"), so it is not one.\n    ");
    text = mcall(&mcall(&g__CARRIED_LEAD(), "sub", &[py_str(""), mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), text.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    said = _unit_words(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    spare = call_builtin("set", &[mcall(&mcall(&text.clone(), "replace", &[py_str(","), py_str(" ")]), "split", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __aug1 = op_sub(&spare.clone(), &g__CARRIED_FILLER());
    spare = __aug1;
    let __iter2 = py_iter_or_raise(&said.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    word = __unpk3[0].clone();
    _u = __unpk3[1].clone();
    let __aug4 = op_sub(&spare.clone(), &call_builtin("set", &[mcall(&word.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    spare = __aug4;
    }
    food = _food_named(&[text.clone()]);
    if py_truthy(&food.clone()) {
    let __aug5 = op_sub(&spare.clone(), &call_builtin("set", &[mcall(&geti(&food.clone(), &Py::Int(0)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    spare = __aug5;
    }
    if py_truthy(&spare.clone()) {
    return Py::None;
    }
    return py_tuple(vec![said.clone(), food.clone()]);
    Py::None
}
pub fn _previous_measure(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let _ = py_str("The measure question the chat is still on, from the turns the memory keeps.\n\n    A chain is the same question asked again (\"... and in a tablespoon\", \"what about a cup\"),\n    so a bare measure phrase is walked past to reach the question it belongs to. A turn that is\n    neither ends the chain: \"i need to lose weight\" followed by \"in grams\" is not a conversion.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    if py_truthy(&_answer(&[line.clone()])) {
    return line.clone();
    }
    if py_truthy(&Py::Bool(py_is(&_bare_measure(&[line.clone()]), &Py::None))) {
    return Py::None;
    }
    }
    return Py::None;
    Py::None
}
pub fn _previous_portion(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let _ = py_str("The last portion question in the chat, from the turns the memory keeps.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g__PEOPLE_COUNT(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { _per_person(&[line.clone()]) } }) {
    return line.clone();
    }
    if py_truthy(&Py::Bool(py_is(&_bare_measure(&[line.clone()]), &Py::None))) {
    return Py::None;
    }
    }
    return Py::None;
    Py::None
}
pub fn _carried_people(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _u: Py = Py::None;
    let mut found: Py = Py::None;
    let mut group: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut old: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut rebuilt: Py = Py::None;
    let mut stripped: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The portion question the chat is still on, asked for a different number of people.");
    text = mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&mcall(&g__CARRIED_NOW(), "match", &[mcall(&text.clone(), "strip", &[py_str(" ?.!,")])])) {
    return _portion_after_correction(&[memory.clone()]);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(3)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    stripped = mcall(&mcall(&g__TRAILING_JOIN(), "sub", &[py_str(""), mcall(&g__CARRIED_LEAD(), "sub", &[py_str(""), text.clone()])]), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&Py::Bool(py_eq(&stripped.clone(), &mcall(&text.clone(), "strip", &[py_str(" ?.!,")])))) {
    break;
    }
    text = stripped.clone();
    }
    text = mcall(&text.clone(), "strip", &[py_str(" ?.!,")]);
    match_ = mcall(&g__CARRIED_PEOPLE(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    previous = _previous_portion(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&previous.clone()))) {
    return Py::None;
    }
    found = mcall(&g__PEOPLE_COUNT(), "search", &[previous.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    old = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "groups", &[])) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&old.clone(), &Py::None))) {
    return Py::None;
    }
    rebuilt = op_add(&op_add(&slice_or_raise(&previous.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "replace", &[old.clone(), mcall(&match_.clone(), "group", &[py_str("count")]), Py::Int(1)])), &slice_or_raise(&previous.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    return _answer(&[rebuilt.clone()]);
    Py::None
}
pub fn _carried_weight(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut _name: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut halves: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut noun: Py = Py::None;
    let mut people: Py = Py::None;
    let mut portions: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut said: Py = Py::None;
    let mut side: Py = Py::None;
    let mut text: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut weight: Py = Py::None;
    let _ = py_str("The portion question the chat is still on, asked with a different weight.");
    text = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    match_ = mcall(&g__NEW_WEIGHT_THEN(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    previous = _previous_portion(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&previous.clone()))) {
    return Py::None;
    }
    found = stdlib_call("re", "search", &[py_str("\\b(\\d+(?:\\.\\d+)?)\\s*(kg|kilo|kilos|kilogram|kilograms|g|gram|grams|gramme|grammes)\\b"), mcall(&match_.clone(), "group", &[py_str("amount")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&found.clone()) {
    grams = call_builtin("float", &[mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e));
    unit = mcall(&found.clone(), "group", &[Py::Int(2)]);
    if py_truthy(&mcall(&unit.clone(), "startswith", &[py_tuple(vec![py_str("k"), py_str("kilo")])])) {
    let __aug1 = op_mul(&grams.clone(), &Py::Int(1000));
    grams = __aug1;
    }
    } else {
    grams = if py_truthy(&mcall(&mcall(&match_.clone(), "group", &[py_str("amount")]), "startswith", &[py_str("half")])) { Py::Float(500.0) } else { Py::Float(1000.0) };
    }
    best = Py::None;
    let __iter2 = py_iter_or_raise(&op_add(&g_PER_PERSON(), &g_PER_PERSON_WHEN_COUNTED()));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 4, None);
    names = __unpk3[0].clone();
    amount = __unpk3[1].clone();
    kind = __unpk3[2].clone();
    noun = __unpk3[3].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&kind.clone(), &py_str("g"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&amount.clone())) } }) {
    continue;
    }
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])"); py_string(__s) }, previous.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), amount.clone(), noun.clone()]);
    }
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&best.clone()))) {
    return Py::None;
    }
    let __unpk5 = py_unpack(&best.clone(), 3, None);
    _name = __unpk5[0].clone();
    amount = __unpk5[1].clone();
    noun = __unpk5[2].clone();
    weight = if py_truthy(&{ let __b = op_lt(&grams.clone(), &Py::Int(1000)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&mcall(&match_.clone(), "group", &[py_str("amount")]), &py_str("kg"))) } }) { { let mut __s = String::new(); __s.push_str(&fstr(&grams.clone(), -1, &"g")); __s.push_str(" g"); py_string(__s) } } else { { let mut __s = String::new(); __s.push_str(&fstr(&op_truediv(&grams.clone(), &Py::Int(1000)), -1, &"g")); __s.push_str(" kg"); py_string(__s) } };
    portions = op_truediv(&grams.clone(), &amount.clone());
    people = _people_asked(&[previous.clone()]);
    halves = op_truediv(&call_builtin("round", &[op_mul(&portions.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    said = mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&halves.clone(), -1, &"g")); py_string(__s) }, "replace", &[py_str(".5"), py_str(" and a half")]);
    if py_truthy(&{ let __b = people.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("abs", &[op_sub(&halves.clone(), &people.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.75)) } }) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about right for "); __s.push_str(&fstr(&people.clone(), -1, "")); __s.push_str(": the usual allowance is "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person."); py_string(__s) };
    }
    if py_truthy(&people.clone()) {
    side = if py_truthy(&op_gt(&halves.clone(), &people.clone())) { py_str("a little more") } else { py_str("a little less") };
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" portions, "); __s.push_str(&fstr(&side.clone(), -1, "")); __s.push_str(" than the "); __s.push_str(&fstr(&people.clone(), -1, "")); __s.push_str(" you asked about: the usual allowance is "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person."); py_string(__s) };
    }
    if py_truthy(&op_le(&halves.clone(), &Py::Int(1))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about one portion: the usual allowance is "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&weight.clone(), -1, "")); __s.push_str(" of "); __s.push_str(&fstr(&noun.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" portions: the usual allowance is "); __s.push_str(&fstr(&amount.clone(), -1, &"g")); __s.push_str(" g a person, so it feeds "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _names_a_portion_food(args: &[Py]) -> Py {
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
    let mut _amount: Py = Py::None;
    let mut _kind: Py = Py::None;
    let mut _noun: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let _ = py_str("True when the line names a food the portion table knows, so it is a portion question.");
    let __iter1 = py_iter_or_raise(&op_add(&g_PER_PERSON(), &g_PER_PERSON_WHEN_COUNTED()));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    names = __unpk2[0].clone();
    _amount = __unpk2[1].clone();
    _kind = __unpk2[2].clone();
    _noun = __unpk2[3].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])"); py_string(__s) }, line.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _portion_after_correction(args: &[Py]) -> Py {
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
    let mut group: Py = Py::None;
    let mut line: Py = Py::None;
    let mut old: Py = Py::None;
    let mut portion: Py = Py::None;
    let mut rebuilt: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The portion question the chat was on, asked again for the count the person corrected.\n\n    \"how much mince do i need\" answers for six, then \"oh sorry i meant 8 people not 6\" is the\n    new headcount, and \"so how much now\" is the first question with the second number. The food\n    comes from the question and the number from the correction, which is why this reads the\n    turns rather than the line.\n    ");
    portion = Py::None;
    count = Py::None;
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&call_builtin("list", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &op_neg(&Py::Int(4)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&portion.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g__AN_ASK(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { _names_a_portion_food(&[line.clone()]) } } } }) {
    portion = line.clone();
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&portion.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { mcall(&g__CORRECTION(), "search", &[line.clone()]) } }) {
    found = _people_asked(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_is(&found.clone(), &Py::None))) {
    count = found.clone();
    continue;
    }
    }
    break;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&portion.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&count.clone(), &Py::None)) } }) {
    return Py::None;
    }
    found = mcall(&g__PEOPLE_COUNT(), "search", &[portion.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return _answer(&[{ let mut __s = String::new(); __s.push_str(&fstr(&portion.clone(), -1, "")); __s.push_str(" for "); __s.push_str(&fstr(&count.clone(), -1, "")); __s.push_str(" people"); py_string(__s) }]);
    }
    old = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&found.clone(), "groups", &[])) { let mut group: Py = __item.clone(); if py_truthy(&group.clone()) {  __out.push(group.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&old.clone(), &Py::None))) {
    return Py::None;
    }
    rebuilt = op_add(&op_add(&slice_or_raise(&portion.clone(), &Py::None, &mcall(&found.clone(), "start", &[]), &Py::None), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "replace", &[old.clone(), call_builtin("str", &[count.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)])), &slice_or_raise(&portion.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None));
    return _answer(&[rebuilt.clone()]);
    Py::None
}
pub fn _carried_measure(args: &[Py]) -> Py {
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut bare: Py = Py::None;
    let mut food: Py = Py::None;
    let mut named: Py = Py::None;
    let mut of_food: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut previous_units: Py = Py::None;
    let mut rebuilt: Py = Py::None;
    let mut said: Py = Py::None;
    let mut source: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut unit_word: Py = Py::None;
    let mut want: Py = Py::None;
    let mut want_family: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The measure question the turn before asked, asked again with this line's unit.");
    bare = _bare_measure(&[clean.clone()]);
    if py_truthy(&Py::Bool(py_is(&bare.clone(), &Py::None))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&bare.clone(), 2, None);
    said = __unpk1[0].clone();
    food = __unpk1[1].clone();
    previous = _previous_measure(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&previous.clone()))) {
    return Py::None;
    }
    want = mcall(&g__ASKED_MEASURE(), "search", &[previous.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return Py::None;
    }
    want_family = _named_unit(&[mcall(&want.clone(), "group", &[py_str("want")])]);
    let __unpk2 = py_unpack(&geti(&said.clone(), &op_neg(&Py::Int(1))), 2, None);
    unit_word = __unpk2[0].clone();
    unit = __unpk2[1].clone();
    previous_units = _unit_words(&[previous.clone()]);
    source = Py::None;
    let __iter3 = py_iter_or_raise(&previous_units.clone());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    word = __unpk4[0].clone();
    named = __unpk4[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&want_family.clone(), &Py::None)); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&geti(&named.clone(), &Py::Int(1)), &geti(&want_family.clone(), &Py::Int(1)))) } }) {
    source = py_tuple(vec![word.clone(), named.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&food.clone()))) {
    food = _food_named(&[previous.clone()]);
    }
    of_food = if py_truthy(&food.clone()) { op_add(&py_str(" of "), &geti(&food.clone(), &Py::Int(0))) } else { py_str("") };
    if py_truthy(&{ let __b = want_family.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&geti(&unit.clone(), &Py::Int(1)), &geti(&want_family.clone(), &Py::Int(1)))); if !py_truthy(&__b) { __b } else { source.clone() } } } }) {
    rebuilt = { let mut __s = String::new(); __s.push_str("how many "); __s.push_str(&fstr(&unit_word.clone(), -1, "")); __s.push_str(" in a "); __s.push_str(&fstr(&geti(&source.clone(), &Py::Int(0)), -1, "")); __s.push_str(&fstr(&of_food.clone(), -1, "")); py_string(__s) };
    } else {
    rebuilt = { let mut __s = String::new(); __s.push_str("how many "); __s.push_str(&fstr(&mcall(&want.clone(), "group", &[py_str("want")]), -1, "")); __s.push_str(" in a "); __s.push_str(&fstr(&unit_word.clone(), -1, "")); __s.push_str(&fstr(&of_food.clone(), -1, "")); py_string(__s) };
    }
    return _answer(&[rebuilt.clone()]);
    Py::None
}
pub fn _a_or_an(args: &[Py]) -> Py {
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
    let _ = py_str("The article a word takes: \"an hour\", \"a minute\". Half an hour, not half a hour.");
    word = call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("aeiou"), &mcall(&slice_or_raise(&word.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]))); if py_truthy(&__b) { __b } else { mcall(&mcall(&word.clone(), "lower", &[]), "startswith", &[py_str("hour")]) } }) {
    return op_add(&py_str("an "), &word.clone());
    }
    return op_add(&py_str("a "), &word.clone());
    Py::None
}
pub fn _amount_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut value: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("An amount as a cook says it: half a cup, three quarters of a cup, 200 grams.");
    if py_truthy(&Py::Bool(py_contains(&g__SHARE_OF_CUP(), &unit.clone()))) {
    said = _cups_phrase(&[value.clone(), unit.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&said.clone(), &{ let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[value.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(value.clone()); __args.push(unit.clone()); __kws.extend(vec![("one".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) }, -1, "")); py_string(__s) }))) {
    return said.clone();
    }
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![Py::Float(0.25), Py::Float(0.5), Py::Float(0.75)]), &value.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAID_ONE(), &unit.clone())) } }) {
    word = geti(&py_dict(vec![(Py::Float(0.25), py_str("a quarter of")), (Py::Float(0.5), py_str("half")), (Py::Float(0.75), py_str("three quarters of"))]), &value.clone());
    return { let mut __s = String::new(); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_a_or_an(&[geti(&g_SAID_ONE(), &unit.clone())]), -1, "")); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[value.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_say(&[value.clone(), unit.clone()]), -1, "")); py_string(__s) };
    Py::None
}
pub fn _cups_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut amount: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "amount") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: amount"))) };
    let mut unit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut spoons: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "spoons") { v.clone() }
    else { Py::Bool(false) };
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A cup amount as a cook says it: half a cup, not 0.5 cups.\n\n    Under half a cup and off the fractions, a cook weighing a food reaches for a spoon, not\n    for a decimal: 50 g of flour is \"about 6 1/2 tablespoons\", and nobody measures 0.4 of a\n    cup. A plain pair of units stays in cups, so \"how many cups in a tablespoon\"\n    does not answer in teaspoons.\n    ");
    if py_truthy(&Py::Bool(py_eq(&unit.clone(), &py_str("cup")))) {
    let __iter1 = py_iter_or_raise(&mcall(&g__FRACTIONS(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    value = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&amount.clone(), &value.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.06))) {
    return said.clone();
    }
    }
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&amount.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.06))) {
    return py_str("1 cup");
    }
    if py_truthy(&{ let __b = spoons.clone(); if !py_truthy(&__b) { __b } else { { let __cmp31: Py = amount.clone(); if py_truthy(&op_lt(&Py::Int(0), &__cmp31)) { op_lt(&__cmp31, &Py::Float(0.5)) } else { Py::Bool(false) } } } }) {
    spoons = op_mul(&amount.clone(), &Py::Int(16));
    if py_truthy(&op_lt(&spoons.clone(), &Py::Int(2))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_spoons(&[op_mul(&spoons.clone(), &Py::Int(3))]), -1, "")); __s.push_str(" teaspoons"); py_string(__s) };
    }
    if py_truthy(&op_lt(&spoons.clone(), &Py::Int(10))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&_spoons(&[spoons.clone()]), -1, "")); __s.push_str(" tablespoons"); py_string(__s) };
    }
    }
    if py_truthy(&op_ge(&amount.clone(), &Py::Float(0.1))) {
    amount = call_builtin("round", &[amount.clone(), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&_fmt(&[amount.clone()]), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); __args.push(unit.clone()); __kws.extend(vec![("one".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _say(args))(&__args) }, -1, "")); py_string(__s) };
    Py::None
}
pub fn _spoons(args: &[Py]) -> Py {
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
    let mut nearest: Py = Py::None;
    let _ = py_str("A number of spoons as a recipe says it: 4, 6, or 1 1/2 when half a spoon matters.");
    nearest = op_truediv(&call_builtin("round", &[op_mul(&count.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2));
    if py_truthy(&Py::Bool(py_eq(&nearest.clone(), &call_builtin("int", &[nearest.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return call_builtin("str", &[call_builtin("int", &[nearest.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&call_builtin("int", &[nearest.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str(" 1/2"); py_string(__s) };
    Py::None
}
pub fn answers_more_than_a_pair(args: &[Py]) -> Py {
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
    let _ = py_str("True when the reply says more than how many of one unit make another.\n\n    A plain pair (\"how many grams in an ounce\") may already have a hand-written fact with\n    a second useful measure in it; a named food or an oven has no such fact and the table\n    is the only place that knows the answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__SPECIFIC(), "search", &[clean.clone()])) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_is(&_food_named(&[clean.clone()]), &Py::None));
    Py::None
}
pub fn _food_named(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The food this line names, and what a cup of it weighs, the longest name winning.\n\n    \"brown sugar\" and \"sugar\" are each a cup of something, and the longer name is the one\n    meant, so the table can hold both without one swallowing the other.\n    ");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_GRAMS_PER_CUP());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    grams = __unpk2[1].clone();
    if py_truthy(&stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("\\b"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), grams.clone()]);
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _named_ingredient(args: &[Py]) -> Py {
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
    let mut mentions: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "mentions") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: mentions"))) };
    let mut measured: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "measured") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: measured"))) };
    let mut _u: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut fam: Py = Py::None;
    let mut family: Py = Py::None;
    let mut food: Py = Py::None;
    let mut food_name: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut grams_per_cup: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_food: Py = Py::None;
    let mut source: Py = Py::None;
    let mut target: Py = Py::None;
    let mut u: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut value: Py = Py::None;
    let mut wanted: Py = Py::None;
    let mut wanted_mass: Py = Py::None;
    let _ = py_str("A cook's weight and cups of the same ingredient, converted either way.\n\n    \"how many cups is 200 g of flour\" and \"1.5 cups of flour in grams\" both need the\n    ingredient's weight per cup, which is the one thing a plain volume cannot give.\n    ");
    food = _food_named(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&food.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&food.clone(), 2, None);
    food_name = __unpk1[0].clone();
    grams_per_cup = __unpk1[1].clone();
    said_food = mcall(&g_AS_KITCHEN_SAYS(), "get", &[food_name.clone(), food_name.clone()]);
    wanted = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mentions.clone()) { let __unpk2736 = py_unpack(&__item.clone(), 4, None); let mut _u: Py = __unpk2736[0].clone(); let mut _u: Py = __unpk2736[1].clone(); let mut family: Py = __unpk2736[2].clone(); let mut unit: Py = __unpk2736[3].clone(); if py_truthy(&Py::Bool(py_contains(&g__SHARE_OF_CUP(), &unit.clone()))) {  __out.push(unit.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&wanted.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&measured.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 4, None);
    value = __unpk3[0].clone();
    family = __unpk3[1].clone();
    unit = __unpk3[2].clone();
    _u = __unpk3[3].clone();
    if py_truthy(&Py::Bool(py_contains(&g__SHARE_OF_CUP(), &unit.clone()))) {
    grams = op_mul(&op_mul(&value.clone(), &grams_per_cup.clone()), &geti(&g__SHARE_OF_CUP(), &unit.clone()));
    source = op_add(&_amount_phrase(&[value.clone(), unit.clone()]), &{ let mut __s = String::new(); __s.push_str(" of "); __s.push_str(&fstr(&said_food.clone(), -1, "")); py_string(__s) });
    source = op_add(&mcall(&geti(&source.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&source.clone(), &Py::Int(1), &Py::None, &Py::None));
    wanted_mass = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mentions.clone()) { let __unpk3149 = py_unpack(&__item.clone(), 4, None); let mut _u: Py = __unpk3149[0].clone(); let mut _u: Py = __unpk3149[1].clone(); let mut fam: Py = __unpk3149[2].clone(); let mut u: Py = __unpk3149[3].clone(); if py_truthy(&Py::Bool(py_eq(&fam.clone(), &py_str("mass")))) {  __out.push(u.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&wanted_mass.clone()))) {
    continue;
    }
    target = geti(&wanted_mass.clone(), &Py::Int(0));
    if py_truthy(&Py::Bool(py_contains(&g__SHARE_OF_CUP(), &target.clone()))) {
    continue;
    }
    amount = op_truediv(&grams.clone(), &_factor(&[py_str("mass"), target.clone()]));
    said = if py_truthy(&Py::Bool(py_eq(&target.clone(), &py_str("g")))) { _grams(&[amount.clone()]) } else { _fmt(&[amount.clone()]) };
    return { let mut __s = String::new(); __s.push_str(&fstr(&source.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&geti(&g_SAID(), &target.clone()), -1, "")); __s.push_str("."); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_eq(&family.clone(), &py_str("mass")))) {
    grams = op_mul(&value.clone(), &_factor(&[py_str("mass"), unit.clone()]));
    target = geti(&wanted.clone(), &Py::Int(0));
    amount = op_truediv(&grams.clone(), &op_mul(&grams_per_cup.clone(), &geti(&g__SHARE_OF_CUP(), &target.clone())));
    source = op_add(&_amount_phrase(&[value.clone(), unit.clone()]), &{ let mut __s = String::new(); __s.push_str(" of "); __s.push_str(&fstr(&said_food.clone(), -1, "")); py_string(__s) });
    source = op_add(&mcall(&geti(&source.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&source.clone(), &Py::Int(1), &Py::None, &Py::None));
    return { let mut __s = String::new(); __s.push_str(&fstr(&source.clone(), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); __args.push(target.clone()); __kws.extend(vec![("spoons".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _cups_phrase(args))(&__args) }, -1, "")); __s.push_str("."); py_string(__s) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _plural_of(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut head: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The words a cook counts more than one of: \"onions\", \"cloves of garlic\".");
    let __unpk1 = py_unpack(&mcall(&name.clone(), "partition", &[py_str(" ")]), 3, None);
    head = __unpk1[0].clone();
    _u = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = mcall(&head.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&head.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    head = op_add(&slice_or_raise(&head.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies"));
    } else {
    if py_truthy(&mcall(&head.clone(), "endswith", &[py_tuple(vec![py_str("o"), py_str("ch"), py_str("sh"), py_str("s"), py_str("x")])])) {
    let __aug2 = op_add(&head.clone(), &py_str("es"));
    head = __aug2;
    } else {
    let __aug3 = op_add(&head.clone(), &py_str("s"));
    head = __aug3;
    }
    }
    return op_add(&head.clone(), &if py_truthy(&tail.clone()) { op_add(&py_str(" "), &tail.clone()) } else { py_str("") });
    Py::None
}
pub fn _a(args: &[Py]) -> Py {
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
    let _ = py_str("The piece with its article: \"an egg\", \"a banana\".");
    return op_add(&if py_truthy(&Py::Bool(py_contains(&py_str("aeiou"), &geti(&name.clone(), &Py::Int(0))))) { py_str("an ") } else { py_str("a ") }, &name.clone());
    Py::None
}
pub fn _counted_as(args: &[Py]) -> Py {
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
    let mut fractions: Py = Py::None;
    let mut whole_and_a_half: Py = Py::None;
    let _ = py_str("How many the line counts: \"an\" is one, \"half a\" is a half, \"a quarter of\" a quarter.");
    word = mcall(&py_str(" "), "join", &[mcall(&mcall(&word.clone(), "replace", &[py_str("-"), py_str(" ")]), "split", &[])]);
    fractions = py_dict(vec![(py_str("half"), Py::Float(0.5)), (py_str("a half"), Py::Float(0.5)), (py_str("quarter"), Py::Float(0.25)), (py_str("a quarter"), Py::Float(0.25)), (py_str("three quarters"), Py::Float(0.75)), (py_str("a third"), op_truediv(&Py::Int(1), &Py::Int(3))), (py_str("two thirds"), op_truediv(&Py::Int(2), &Py::Int(3)))]);
    if py_truthy(&Py::Bool(py_contains(&fractions.clone(), &word.clone()))) {
    return geti(&fractions.clone(), &word.clone());
    }
    whole_and_a_half = stdlib_call("re", "fullmatch", &[py_str("(one|two|three|four) and a half"), word.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&whole_and_a_half.clone()) {
    return op_add(&geti(&g__SAID_WHOLE(), &mcall(&whole_and_a_half.clone(), "group", &[Py::Int(1)])), &Py::Float(0.5));
    }
    if py_truthy(&stdlib_call("re", "fullmatch", &[py_str("\\d+(?:\\.\\d+)?"), word.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(".")))) { call_builtin("float", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("int", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)) };
    }
    return geti(&g__SAID_WHOLE(), &word.clone());
    Py::None
}
pub fn _piece_named(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The piece this line counts, and what one of it weighs: (name, grams, count) or None.\n\n    Only a counted piece is a piece. \"how many grams of pepper\" is a spoonful of pepper, not\n    a pepper, so the word has to follow a count (\"a pepper\", \"2 onions\"); and a tin, a jar or\n    a loaf is not the thing loose, so those lines are left alone.\n    ");
    if py_truthy(&mcall(&g__UNCOUNTED(), "search", &[mcall(&g__WHOLE_THING(), "sub", &[py_str(""), clean.clone()])])) {
    return Py::None;
    }
    if py_truthy(&mcall(&g__MEASURE_AFTER_PIECE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_PIECE_GRAMS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    name = __unpk2[0].clone();
    grams = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&py_tuple(vec![name.clone(), _plural_of(&[name.clone()])]));
    for __item in __iter3 {
    words = __item.clone();
    match_ = stdlib_call("re", "search", &[{ let mut __s = String::new(); __s.push_str("(?<![\\w.])"); __s.push_str(&fstr(&stdlib_call("re", "escape", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), -1, "")); __s.push_str("(?![\\w])(?=\\s*(?:$|[,?]|of\\b|weigh|weighs|weighed|about|in grams?|in g\\b|to grams|is\\b))"); py_string(__s) }, clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    counted = mcall(&g__COUNT(), "search", &[slice_or_raise(&clean.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None)]);
    if py_truthy(&Py::Bool(!py_truthy(&counted.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[geti(&best.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    best = py_tuple(vec![name.clone(), grams.clone(), _counted_as(&[mcall(&counted.clone(), "group", &[Py::Int(1)])])]);
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _piece_weight(args: &[Py]) -> Py {
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
    let mut _end: Py = Py::None;
    let mut _family: Py = Py::None;
    let mut _unit: Py = Py::None;
    let mut amount: Py = Py::None;
    let mut count: Py = Py::None;
    let mut grams: Py = Py::None;
    let mut name: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let _ = py_str("What one of a food weighs, when the line counts pieces instead of naming a measure.\n\n    \"how many grams in an egg\", \"how much does an onion weigh\", \"2 onions in grams\": the\n    weight is the food's own, so it comes from the same kind of table as a cup's weight.\n    A line that already converts a measure (\"how many grams in a pound of bananas\") is a\n    conversion, and is left to the code below.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&_asks_for_weight(&[clean.clone()])))) {
    return Py::None;
    }
    piece = _piece_named(&[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&piece.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&_mentions(&[clean.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 4, None);
    start = __unpk2[0].clone();
    _end = __unpk2[1].clone();
    _family = __unpk2[2].clone();
    _unit = __unpk2[3].clone();
    if py_truthy(&Py::Bool(!py_is(&_number_before(&[clean.clone(), start.clone()]), &Py::None))) {
    return Py::None;
    }
    }
    let __unpk3 = py_unpack(&piece.clone(), 3, None);
    name = __unpk3[0].clone();
    grams = __unpk3[1].clone();
    count = __unpk3[2].clone();
    amount = _mass_of(&[op_mul(&grams.clone(), &count.clone())]);
    if py_truthy(&Py::Bool(py_eq(&count.clone(), &Py::Int(1)))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&mcall(&_a(&[name.clone()]), "capitalize", &[]), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    let __unpk4 = py_unpack(&_count_phrase(&[count.clone()]), 2, None);
    said = __unpk4[0].clone();
    plural = __unpk4[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&plural.clone()))) {
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_a(&[name.clone()]), -1, "")); __s.push_str(" is about "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str(&fstr(&said.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&_plural_of(&[name.clone()]), -1, "")); __s.push_str(" weigh about "); __s.push_str(&fstr(&amount.clone(), -1, "")); __s.push_str("."); py_string(__s) };
    Py::None
}
pub fn _count_phrase(args: &[Py]) -> Py {
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
    let mut fractions: Py = Py::None;
    let mut said: Py = Py::None;
    let mut share: Py = Py::None;
    let mut whole: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("How the amount is said, and whether the food is plural: (\"Half\", False), (\"Two\", True).");
    fractions = py_dict(vec![(Py::Float(0.25), py_str("A quarter of")), (op_truediv(&Py::Int(1), &Py::Int(3)), py_str("A third of")), (Py::Float(0.5), py_str("Half")), (op_truediv(&Py::Int(2), &Py::Int(3)), py_str("Two thirds of")), (Py::Float(0.75), py_str("Three quarters of"))]);
    let __iter1 = py_iter_or_raise(&mcall(&fractions.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    share = __unpk2[0].clone();
    said = __unpk2[1].clone();
    if py_truthy(&op_lt(&call_builtin("abs", &[op_sub(&count.clone(), &share.clone())]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.01))) {
    return py_tuple(vec![said.clone(), Py::Bool(false)]);
    }
    }
    if py_truthy(&Py::Bool(!py_eq(&count.clone(), &call_builtin("int", &[count.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    whole = call_builtin("int", &[count.clone()]).unwrap_or_else(|e| exc::raise(e));
    word = if py_truthy(&Py::Bool(py_eq(&whole.clone(), &Py::Int(1)))) { py_str("One") } else { mcall(&g__SAID_COUNT(), "get", &[whole.clone(), _fmt(&[whole.clone()])]) };
    return py_tuple(vec![{ let mut __s = String::new(); __s.push_str(&fstr(&word.clone(), -1, "")); __s.push_str(" and a half"); py_string(__s) }, Py::Bool(true)]);
    }
    if py_truthy(&Py::Bool(py_contains(&g__SAID_COUNT(), &count.clone()))) {
    return py_tuple(vec![geti(&g__SAID_COUNT(), &count.clone()), Py::Bool(true)]);
    }
    return py_tuple(vec![_fmt(&[count.clone()]), Py::Bool(true)]);
    Py::None
}
pub fn _named_unit(args: &[Py]) -> Py {
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
    let mut best: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The unit a short phrase names (\"grams\", \"cups of flour\"), or None.");
    best = Py::None;
    let __iter1 = py_iter_or_raise(&mcall(&g__UNIT_WORD(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[mcall(&match_.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e)), &geti(&best.clone(), &Py::Int(2))) } }) {
    best = op_add(&geti(&g__NAMES(), &mcall(&match_.clone(), "group", &[Py::Int(0)])), &py_tuple(vec![call_builtin("len", &[mcall(&match_.clone(), "group", &[Py::Int(0)])]).unwrap_or_else(|e| exc::raise(e))]));
    }
    }
    return if py_truthy(&best.clone()) { py_tuple(vec![geti(&best.clone(), &Py::Int(0)), geti(&best.clone(), &Py::Int(1))]) } else { Py::None };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.units".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AS_KITCHEN_SAYS" => Some(g_AS_KITCHEN_SAYS()),
            "BIRD_GRAMS" => Some(g_BIRD_GRAMS()),
            "CORRECTING" => Some(g_CORRECTING()),
            "COUNTED_PACKS" => Some(g_COUNTED_PACKS()),
            "DISHES_FOR_A_TABLE" => Some(g_DISHES_FOR_A_TABLE()),
            "FAN_COOLER" => Some(g_FAN_COOLER()),
            "GAS_OVEN" => Some(g_GAS_OVEN()),
            "GRAMS_PER_CUP" => Some(g_GRAMS_PER_CUP()),
            "GRAMS_PER_TSP" => Some(g_GRAMS_PER_TSP()),
            "LAND_PLAIN" => Some(g_LAND_PLAIN()),
            "LIQUID_GRAMS_LITRE" => Some(g_LIQUID_GRAMS_LITRE()),
            "LOOSE" => Some(g_LOOSE()),
            "MADE_NOT_MEASURED" => Some(g_MADE_NOT_MEASURED()),
            "NOT_A_TABLE" => Some(g_NOT_A_TABLE()),
            "OBJECT_GRAMS" => Some(g_OBJECT_GRAMS()),
            "PACK_DRAINED" => Some(g_PACK_DRAINED()),
            "PACK_GRAMS" => Some(g_PACK_GRAMS()),
            "PACK_IN_ML" => Some(g_PACK_IN_ML()),
            "PER_PERSON" => Some(g_PER_PERSON()),
            "PER_PERSON_WHEN_COUNTED" => Some(g_PER_PERSON_WHEN_COUNTED()),
            "PIECE_GRAMS" => Some(g_PIECE_GRAMS()),
            "SAID" => Some(g_SAID()),
            "SAID_ONE" => Some(g_SAID_ONE()),
            "SYSTEM" => Some(g_SYSTEM()),
            "UNITS" => Some(g_UNITS()),
            "WHOLE_THING_SERVES" => Some(g_WHOLE_THING_SERVES()),
            "_AND_A_HALF" => Some(g__AND_A_HALF()),
            "_AN_ASK" => Some(g__AN_ASK()),
            "_AREA_PICTURES" => Some(g__AREA_PICTURES()),
            "_ASKED_MEASURE" => Some(g__ASKED_MEASURE()),
            "_ASKING_AFTER_MEASURE" => Some(g__ASKING_AFTER_MEASURE()),
            "_ASKS_ELSEWHERE" => Some(g__ASKS_ELSEWHERE()),
            "_ASKS_FEET" => Some(g__ASKS_FEET()),
            "_ASKS_HOW_MANY" => Some(g__ASKS_HOW_MANY()),
            "_A_CLOCK" => Some(g__A_CLOCK()),
            "_A_COMPLAINT" => Some(g__A_COMPLAINT()),
            "_A_COUNT" => Some(g__A_COUNT()),
            "_A_CUT_NOT_A_BIRD" => Some(g__A_CUT_NOT_A_BIRD()),
            "_A_DISH_PART" => Some(g__A_DISH_PART()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_FRACTION" => Some(g__A_FRACTION()),
            "_A_FRACTION_AS" => Some(g__A_FRACTION_AS()),
            "_A_FRACTION_ASK" => Some(g__A_FRACTION_ASK()),
            "_A_FRACTION_OF" => Some(g__A_FRACTION_OF()),
            "_A_FRACTION_OF_A_MIXED" => Some(g__A_FRACTION_OF_A_MIXED()),
            "_A_HALF_SPAN" => Some(g__A_HALF_SPAN()),
            "_A_MEASURE_QUESTION" => Some(g__A_MEASURE_QUESTION()),
            "_A_MEASURE_SAID_ANOTHER_WAY" => Some(g__A_MEASURE_SAID_ANOTHER_WAY()),
            "_A_MIXED_SPAN" => Some(g__A_MIXED_SPAN()),
            "_A_SIZE_JUDGEMENT" => Some(g__A_SIZE_JUDGEMENT()),
            "_A_SPAN_ASK" => Some(g__A_SPAN_ASK()),
            "_A_TARGET_UNIT" => Some(g__A_TARGET_UNIT()),
            "_A_TIME_IS_THE_ASK" => Some(g__A_TIME_IS_THE_ASK()),
            "_A_TIME_SHARE" => Some(g__A_TIME_SHARE()),
            "_A_UNIT_ASKED" => Some(g__A_UNIT_ASKED()),
            "_A_WHOLE_UNIT" => Some(g__A_WHOLE_UNIT()),
            "_BAKING_PORTION" => Some(g__BAKING_PORTION()),
            "_BARE_FOR_COUNT" => Some(g__BARE_FOR_COUNT()),
            "_BARE_JOIN" => Some(g__BARE_JOIN()),
            "_BARE_TEMPERATURE" => Some(g__BARE_TEMPERATURE()),
            "_BIRDS" => Some(g__BIRDS()),
            "_CARRIED_FILLER" => Some(g__CARRIED_FILLER()),
            "_CARRIED_LEAD" => Some(g__CARRIED_LEAD()),
            "_CARRIED_NOW" => Some(g__CARRIED_NOW()),
            "_CARRIED_PEOPLE" => Some(g__CARRIED_PEOPLE()),
            "_CHILLI_THE_VEGETABLE" => Some(g__CHILLI_THE_VEGETABLE()),
            "_COMPONENT" => Some(g__COMPONENT()),
            "_COMPONENT_MEASURED" => Some(g__COMPONENT_MEASURED()),
            "_CONTAINER" => Some(g__CONTAINER()),
            "_CORRECTION" => Some(g__CORRECTION()),
            "_COUNT" => Some(g__COUNT()),
            "_COUNTED_THING" => Some(g__COUNTED_THING()),
            "_COUNT_ASK" => Some(g__COUNT_ASK()),
            "_COUNT_CONTENT" => Some(g__COUNT_CONTENT()),
            "_COUNT_OF_BIRDS" => Some(g__COUNT_OF_BIRDS()),
            "_C_ASKS" => Some(g__C_ASKS()),
            "_DENOMINATOR" => Some(g__DENOMINATOR()),
            "_DISH_AFTER" => Some(g__DISH_AFTER()),
            "_DRY_IN_A_MEASURE" => Some(g__DRY_IN_A_MEASURE()),
            "_FEEDS_ASK" => Some(g__FEEDS_ASK()),
            "_FRACTION" => Some(g__FRACTION()),
            "_FRACTIONS" => Some(g__FRACTIONS()),
            "_F_ASKS" => Some(g__F_ASKS()),
            "_GROUP_VALUE" => Some(g__GROUP_VALUE()),
            "_HEIGHT_CM" => Some(g__HEIGHT_CM()),
            "_HEIGHT_FEET" => Some(g__HEIGHT_FEET()),
            "_HEIGHT_FEET_WORD" => Some(g__HEIGHT_FEET_WORD()),
            "_HEIGHT_METRES" => Some(g__HEIGHT_METRES()),
            "_HOW_TO" => Some(g__HOW_TO()),
            "_IN_A_VESSEL" => Some(g__IN_A_VESSEL()),
            "_JOINED" => Some(g__JOINED()),
            "_KEEPING_HENS" => Some(g__KEEPING_HENS()),
            "_LAND_ASK" => Some(g__LAND_ASK()),
            "_LOOSE_ASK" => Some(g__LOOSE_ASK()),
            "_LOOSE_NOT" => Some(g__LOOSE_NOT()),
            "_MEASURE_AFTER_PIECE" => Some(g__MEASURE_AFTER_PIECE()),
            "_MIXED" => Some(g__MIXED()),
            "_NAMES" => Some(g__NAMES()),
            "_NEW_WEIGHT_THEN" => Some(g__NEW_WEIGHT_THEN()),
            "_NOT_ABOUT_A_MEASURE" => Some(g__NOT_ABOUT_A_MEASURE()),
            "_NOT_A_COUNT" => Some(g__NOT_A_COUNT()),
            "_NOT_A_FOOD" => Some(g__NOT_A_FOOD()),
            "_NOT_A_HEIGHT" => Some(g__NOT_A_HEIGHT()),
            "_NOT_A_TABLE_BEFORE" => Some(g__NOT_A_TABLE_BEFORE()),
            "_NOT_A_TABLE_QUESTION" => Some(g__NOT_A_TABLE_QUESTION()),
            "_NO_PLURAL" => Some(g__NO_PLURAL()),
            "_NUMBER" => Some(g__NUMBER()),
            "_NUMERAL" => Some(g__NUMERAL()),
            "_PACK_ASK" => Some(g__PACK_ASK()),
            "_PACK_WORD" => Some(g__PACK_WORD()),
            "_PEOPLE_COUNT" => Some(g__PEOPLE_COUNT()),
            "_PEOPLE_VALUE" => Some(g__PEOPLE_VALUE()),
            "_PEOPLE_WORD" => Some(g__PEOPLE_WORD()),
            "_PEOPLE_WORD_TO_TWENTY" => Some(g__PEOPLE_WORD_TO_TWENTY()),
            "_PER_CUP" => Some(g__PER_CUP()),
            "_PLURAL_BIRDS" => Some(g__PLURAL_BIRDS()),
            "_PORTIONS_ASK" => Some(g__PORTIONS_ASK()),
            "_PORTION_AVOID" => Some(g__PORTION_AVOID()),
            "_PORTION_COUNT" => Some(g__PORTION_COUNT()),
            "_PORTION_WORD" => Some(g__PORTION_WORD()),
            "_POUNDS_IS_MONEY" => Some(g__POUNDS_IS_MONEY()),
            "_SAID_COUNT" => Some(g__SAID_COUNT()),
            "_SAID_PEOPLE" => Some(g__SAID_PEOPLE()),
            "_SAID_WHOLE" => Some(g__SAID_WHOLE()),
            "_SAYS_OWN_HEIGHT" => Some(g__SAYS_OWN_HEIGHT()),
            "_SAY_K" => Some(g__SAY_K()),
            "_SHARE" => Some(g__SHARE()),
            "_SHARE_ASK" => Some(g__SHARE_ASK()),
            "_SHARE_FRACTION" => Some(g__SHARE_FRACTION()),
            "_SHARE_LEAD" => Some(g__SHARE_LEAD()),
            "_SHARE_OF_CUP" => Some(g__SHARE_OF_CUP()),
            "_SHARE_SAID" => Some(g__SHARE_SAID()),
            "_SHARE_THAT" => Some(g__SHARE_THAT()),
            "_SHARE_VALUE" => Some(g__SHARE_VALUE()),
            "_SLASH" => Some(g__SLASH()),
            "_SPECIFIC" => Some(g__SPECIFIC()),
            "_SPLIT" => Some(g__SPLIT()),
            "_TABLE_GROUPS" => Some(g__TABLE_GROUPS()),
            "_TARGET" => Some(g__TARGET()),
            "_TIME_ARTICLE" => Some(g__TIME_ARTICLE()),
            "_TIME_PLURAL" => Some(g__TIME_PLURAL()),
            "_TIME_SECONDS" => Some(g__TIME_SECONDS()),
            "_TIME_SMALLER" => Some(g__TIME_SMALLER()),
            "_TIME_UNIT" => Some(g__TIME_UNIT()),
            "_TIN_FOR" => Some(g__TIN_FOR()),
            "_TRAILING_JOIN" => Some(g__TRAILING_JOIN()),
            "_UNCOUNTED" => Some(g__UNCOUNTED()),
            "_UNIT_AND_A_HALF" => Some(g__UNIT_AND_A_HALF()),
            "_UNIT_WORD" => Some(g__UNIT_WORD()),
            "_VESSEL_ML" => Some(g__VESSEL_ML()),
            "_WEIGH" => Some(g__WEIGH()),
            "_WEIGHT_SAID" => Some(g__WEIGHT_SAID()),
            "_WHOLE_PIECE" => Some(g__WHOLE_PIECE()),
            "_WHOLE_THING" => Some(g__WHOLE_THING()),
            "_WHOLE_WEIGHT" => Some(g__WHOLE_WEIGHT()),
            "_WORD_FRACTION" => Some(g__WORD_FRACTION()),
            "_WORD_SHARE" => Some(g__WORD_SHARE()),
            "_a" => Some(py_func("_a", |args: &[Py]| _a(args))),
            "_a_corrected_figure" => Some(py_func("_a_corrected_figure", |args: &[Py]| _a_corrected_figure(args))),
            "_a_count_that_still_stands" => Some(py_func("_a_count_that_still_stands", |args: &[Py]| _a_count_that_still_stands(args))),
            "_a_dish_for_a_table" => Some(py_func("_a_dish_for_a_table", |args: &[Py]| _a_dish_for_a_table(args))),
            "_a_fraction_name" => Some(py_func("_a_fraction_name", |args: &[Py]| _a_fraction_name(args))),
            "_a_fraction_of_a_time_unit" => Some(py_func("_a_fraction_of_a_time_unit", |args: &[Py]| _a_fraction_of_a_time_unit(args))),
            "_a_fraction_value" => Some(py_func("_a_fraction_value", |args: &[Py]| _a_fraction_value(args))),
            "_a_mixed_parts" => Some(py_func("_a_mixed_parts", |args: &[Py]| _a_mixed_parts(args))),
            "_a_number_word" => Some(py_func("_a_number_word", |args: &[Py]| _a_number_word(args))),
            "_a_or_an" => Some(py_func("_a_or_an", |args: &[Py]| _a_or_an(args))),
            "_a_share_of_a_unit" => Some(py_func("_a_share_of_a_unit", |args: &[Py]| _a_share_of_a_unit(args))),
            "_a_span_amount" => Some(py_func("_a_span_amount", |args: &[Py]| _a_span_amount(args))),
            "_a_span_of_time" => Some(py_func("_a_span_of_time", |args: &[Py]| _a_span_of_time(args))),
            "_a_table_without_people" => Some(py_func("_a_table_without_people", |args: &[Py]| _a_table_without_people(args))),
            "_a_table_word" => Some(py_func("_a_table_word", |args: &[Py]| _a_table_word(args))),
            "_a_weight" => Some(py_func("_a_weight", |args: &[Py]| _a_weight(args))),
            "_a_whole_thing_feeds" => Some(py_func("_a_whole_thing_feeds", |args: &[Py]| _a_whole_thing_feeds(args))),
            "_amount_phrase" => Some(py_func("_amount_phrase", |args: &[Py]| _amount_phrase(args))),
            "_an_allowance" => Some(py_func("_an_allowance", |args: &[Py]| _an_allowance(args))),
            "_answer" => Some(py_func("_answer", |args: &[Py]| _answer(args))),
            "_area_pictures" => Some(py_func("_area_pictures", |args: &[Py]| _area_pictures(args))),
            "_asks_for_weight" => Some(py_func("_asks_for_weight", |args: &[Py]| _asks_for_weight(args))),
            "_asks_something_else" => Some(py_func("_asks_something_else", |args: &[Py]| _asks_something_else(args))),
            "_bare_measure" => Some(py_func("_bare_measure", |args: &[Py]| _bare_measure(args))),
            "_birds_for_a_table" => Some(py_func("_birds_for_a_table", |args: &[Py]| _birds_for_a_table(args))),
            "_build" => Some(py_func("_build", |args: &[Py]| _build(args))),
            "_butter" => Some(py_func("_butter", |args: &[Py]| _butter(args))),
            "_carried_measure" => Some(py_func("_carried_measure", |args: &[Py]| _carried_measure(args))),
            "_carried_people" => Some(py_func("_carried_people", |args: &[Py]| _carried_people(args))),
            "_carried_weight" => Some(py_func("_carried_weight", |args: &[Py]| _carried_weight(args))),
            "_count_in_a_line" => Some(py_func("_count_in_a_line", |args: &[Py]| _count_in_a_line(args))),
            "_count_phrase" => Some(py_func("_count_phrase", |args: &[Py]| _count_phrase(args))),
            "_count_said" => Some(py_func("_count_said", |args: &[Py]| _count_said(args))),
            "_counted_as" => Some(py_func("_counted_as", |args: &[Py]| _counted_as(args))),
            "_cup_of_something" => Some(py_func("_cup_of_something", |args: &[Py]| _cup_of_something(args))),
            "_cups_phrase" => Some(py_func("_cups_phrase", |args: &[Py]| _cups_phrase(args))),
            "_factor" => Some(py_func("_factor", |args: &[Py]| _factor(args))),
            "_fmt" => Some(py_func("_fmt", |args: &[Py]| _fmt(args))),
            "_food_named" => Some(py_func("_food_named", |args: &[Py]| _food_named(args))),
            "_frac" => Some(py_func("_frac", |args: &[Py]| _frac(args))),
            "_grams" => Some(py_func("_grams", |args: &[Py]| _grams(args))),
            "_handful" => Some(py_func("_handful", |args: &[Py]| _handful(args))),
            "_height" => Some(py_func("_height", |args: &[Py]| _height(args))),
            "_how_to_convert" => Some(py_func("_how_to_convert", |args: &[Py]| _how_to_convert(args))),
            "_in_a_vessel" => Some(py_func("_in_a_vessel", |args: &[Py]| _in_a_vessel(args))),
            "_is_a_distance" => Some(py_func("_is_a_distance", |args: &[Py]| _is_a_distance(args))),
            "_is_bare_pair" => Some(py_func("_is_bare_pair", |args: &[Py]| _is_bare_pair(args))),
            "_kilos" => Some(py_func("_kilos", |args: &[Py]| _kilos(args))),
            "_land_in_pictures" => Some(py_func("_land_in_pictures", |args: &[Py]| _land_in_pictures(args))),
            "_land_size" => Some(py_func("_land_size", |args: &[Py]| _land_size(args))),
            "_liquid_weight" => Some(py_func("_liquid_weight", |args: &[Py]| _liquid_weight(args))),
            "_litres_of" => Some(py_func("_litres_of", |args: &[Py]| _litres_of(args))),
            "_loose" => Some(py_func("_loose", |args: &[Py]| _loose(args))),
            "_many" => Some(py_func("_many", |args: &[Py]| _many(args))),
            "_mark_for" => Some(py_func("_mark_for", |args: &[Py]| _mark_for(args))),
            "_mass_of" => Some(py_func("_mass_of", |args: &[Py]| _mass_of(args))),
            "_mass_said" => Some(py_func("_mass_said", |args: &[Py]| _mass_said(args))),
            "_mentions" => Some(py_func("_mentions", |args: &[Py]| _mentions(args))),
            "_metres_in_feet" => Some(py_func("_metres_in_feet", |args: &[Py]| _metres_in_feet(args))),
            "_named_ingredient" => Some(py_func("_named_ingredient", |args: &[Py]| _named_ingredient(args))),
            "_named_unit" => Some(py_func("_named_unit", |args: &[Py]| _named_unit(args))),
            "_names_a_portion_food" => Some(py_func("_names_a_portion_food", |args: &[Py]| _names_a_portion_food(args))),
            "_names_unit" => Some(py_func("_names_unit", |args: &[Py]| _names_unit(args))),
            "_number_before" => Some(py_func("_number_before", |args: &[Py]| _number_before(args))),
            "_object_weight" => Some(py_func("_object_weight", |args: &[Py]| _object_weight(args))),
            "_oven" => Some(py_func("_oven", |args: &[Py]| _oven(args))),
            "_own_height" => Some(py_func("_own_height", |args: &[Py]| _own_height(args))),
            "_pack" => Some(py_func("_pack", |args: &[Py]| _pack(args))),
            "_pack_count" => Some(py_func("_pack_count", |args: &[Py]| _pack_count(args))),
            "_people_asked" => Some(py_func("_people_asked", |args: &[Py]| _people_asked(args))),
            "_people_before" => Some(py_func("_people_before", |args: &[Py]| _people_before(args))),
            "_people_in_a_count" => Some(py_func("_people_in_a_count", |args: &[Py]| _people_in_a_count(args))),
            "_per_person" => Some(py_func("_per_person", |args: &[Py]| _per_person(args))),
            "_piece_named" => Some(py_func("_piece_named", |args: &[Py]| _piece_named(args))),
            "_piece_weight" => Some(py_func("_piece_weight", |args: &[Py]| _piece_weight(args))),
            "_plural_of" => Some(py_func("_plural_of", |args: &[Py]| _plural_of(args))),
            "_portion_after_correction" => Some(py_func("_portion_after_correction", |args: &[Py]| _portion_after_correction(args))),
            "_portions_in_a_weight" => Some(py_func("_portions_in_a_weight", |args: &[Py]| _portions_in_a_weight(args))),
            "_previous_measure" => Some(py_func("_previous_measure", |args: &[Py]| _previous_measure(args))),
            "_previous_portion" => Some(py_func("_previous_portion", |args: &[Py]| _previous_portion(args))),
            "_rule_sentence" => Some(py_func("_rule_sentence", |args: &[Py]| _rule_sentence(args))),
            "_said_people" => Some(py_func("_said_people", |args: &[Py]| _said_people(args))),
            "_say" => Some(py_func("_say", |args: &[Py]| _say(args))),
            "_sentence" => Some(py_func("_sentence", |args: &[Py]| _sentence(args))),
            "_share_before" => Some(py_func("_share_before", |args: &[Py]| _share_before(args))),
            "_share_carried" => Some(py_func("_share_carried", |args: &[Py]| _share_carried(args))),
            "_share_name" => Some(py_func("_share_name", |args: &[Py]| _share_name(args))),
            "_share_of_a_measure" => Some(py_func("_share_of_a_measure", |args: &[Py]| _share_of_a_measure(args))),
            "_share_spoons" => Some(py_func("_share_spoons", |args: &[Py]| _share_spoons(args))),
            "_share_value" => Some(py_func("_share_value", |args: &[Py]| _share_value(args))),
            "_singular" => Some(py_func("_singular", |args: &[Py]| _singular(args))),
            "_split_time" => Some(py_func("_split_time", |args: &[Py]| _split_time(args))),
            "_spoons" => Some(py_func("_spoons", |args: &[Py]| _spoons(args))),
            "_table_grams" => Some(py_func("_table_grams", |args: &[Py]| _table_grams(args))),
            "_tail_is_about_the_measure" => Some(py_func("_tail_is_about_the_measure", |args: &[Py]| _tail_is_about_the_measure(args))),
            "_temperature" => Some(py_func("_temperature", |args: &[Py]| _temperature(args))),
            "_temperature_rule" => Some(py_func("_temperature_rule", |args: &[Py]| _temperature_rule(args))),
            "_the_component_is_the_whole_measure" => Some(py_func("_the_component_is_the_whole_measure", |args: &[Py]| _the_component_is_the_whole_measure(args))),
            "_the_ounce_pair" => Some(py_func("_the_ounce_pair", |args: &[Py]| _the_ounce_pair(args))),
            "_the_span_said" => Some(py_func("_the_span_said", |args: &[Py]| _the_span_said(args))),
            "_the_thing_measured_by_itself" => Some(py_func("_the_thing_measured_by_itself", |args: &[Py]| _the_thing_measured_by_itself(args))),
            "_tsp_phrase" => Some(py_func("_tsp_phrase", |args: &[Py]| _tsp_phrase(args))),
            "_unit_words" => Some(py_func("_unit_words", |args: &[Py]| _unit_words(args))),
            "_value" => Some(py_func("_value", |args: &[Py]| _value(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "answers_more_than_a_pair" => Some(py_func("answers_more_than_a_pair", |args: &[Py]| answers_more_than_a_pair(args))),
            "convert" => Some(py_func("convert", |args: &[Py]| convert(args))),
            "people_in_groups" => Some(py_func("people_in_groups", |args: &[Py]| people_in_groups(args))),
            "math" => Some(stdlib_module("math")),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AS_KITCHEN_SAYS" => s_AS_KITCHEN_SAYS(v),
            "BIRD_GRAMS" => s_BIRD_GRAMS(v),
            "CORRECTING" => s_CORRECTING(v),
            "COUNTED_PACKS" => s_COUNTED_PACKS(v),
            "DISHES_FOR_A_TABLE" => s_DISHES_FOR_A_TABLE(v),
            "FAN_COOLER" => s_FAN_COOLER(v),
            "GAS_OVEN" => s_GAS_OVEN(v),
            "GRAMS_PER_CUP" => s_GRAMS_PER_CUP(v),
            "GRAMS_PER_TSP" => s_GRAMS_PER_TSP(v),
            "LAND_PLAIN" => s_LAND_PLAIN(v),
            "LIQUID_GRAMS_LITRE" => s_LIQUID_GRAMS_LITRE(v),
            "LOOSE" => s_LOOSE(v),
            "MADE_NOT_MEASURED" => s_MADE_NOT_MEASURED(v),
            "NOT_A_TABLE" => s_NOT_A_TABLE(v),
            "OBJECT_GRAMS" => s_OBJECT_GRAMS(v),
            "PACK_DRAINED" => s_PACK_DRAINED(v),
            "PACK_GRAMS" => s_PACK_GRAMS(v),
            "PACK_IN_ML" => s_PACK_IN_ML(v),
            "PER_PERSON" => s_PER_PERSON(v),
            "PER_PERSON_WHEN_COUNTED" => s_PER_PERSON_WHEN_COUNTED(v),
            "PIECE_GRAMS" => s_PIECE_GRAMS(v),
            "SAID" => s_SAID(v),
            "SAID_ONE" => s_SAID_ONE(v),
            "SYSTEM" => s_SYSTEM(v),
            "UNITS" => s_UNITS(v),
            "WHOLE_THING_SERVES" => s_WHOLE_THING_SERVES(v),
            "_AND_A_HALF" => s__AND_A_HALF(v),
            "_AN_ASK" => s__AN_ASK(v),
            "_AREA_PICTURES" => s__AREA_PICTURES(v),
            "_ASKED_MEASURE" => s__ASKED_MEASURE(v),
            "_ASKING_AFTER_MEASURE" => s__ASKING_AFTER_MEASURE(v),
            "_ASKS_ELSEWHERE" => s__ASKS_ELSEWHERE(v),
            "_ASKS_FEET" => s__ASKS_FEET(v),
            "_ASKS_HOW_MANY" => s__ASKS_HOW_MANY(v),
            "_A_CLOCK" => s__A_CLOCK(v),
            "_A_COMPLAINT" => s__A_COMPLAINT(v),
            "_A_COUNT" => s__A_COUNT(v),
            "_A_CUT_NOT_A_BIRD" => s__A_CUT_NOT_A_BIRD(v),
            "_A_DISH_PART" => s__A_DISH_PART(v),
            "_A_FIGURE" => s__A_FIGURE(v),
            "_A_FRACTION" => s__A_FRACTION(v),
            "_A_FRACTION_AS" => s__A_FRACTION_AS(v),
            "_A_FRACTION_ASK" => s__A_FRACTION_ASK(v),
            "_A_FRACTION_OF" => s__A_FRACTION_OF(v),
            "_A_FRACTION_OF_A_MIXED" => s__A_FRACTION_OF_A_MIXED(v),
            "_A_HALF_SPAN" => s__A_HALF_SPAN(v),
            "_A_MEASURE_QUESTION" => s__A_MEASURE_QUESTION(v),
            "_A_MEASURE_SAID_ANOTHER_WAY" => s__A_MEASURE_SAID_ANOTHER_WAY(v),
            "_A_MIXED_SPAN" => s__A_MIXED_SPAN(v),
            "_A_SIZE_JUDGEMENT" => s__A_SIZE_JUDGEMENT(v),
            "_A_SPAN_ASK" => s__A_SPAN_ASK(v),
            "_A_TARGET_UNIT" => s__A_TARGET_UNIT(v),
            "_A_TIME_IS_THE_ASK" => s__A_TIME_IS_THE_ASK(v),
            "_A_TIME_SHARE" => s__A_TIME_SHARE(v),
            "_A_UNIT_ASKED" => s__A_UNIT_ASKED(v),
            "_A_WHOLE_UNIT" => s__A_WHOLE_UNIT(v),
            "_BAKING_PORTION" => s__BAKING_PORTION(v),
            "_BARE_FOR_COUNT" => s__BARE_FOR_COUNT(v),
            "_BARE_JOIN" => s__BARE_JOIN(v),
            "_BARE_TEMPERATURE" => s__BARE_TEMPERATURE(v),
            "_BIRDS" => s__BIRDS(v),
            "_CARRIED_FILLER" => s__CARRIED_FILLER(v),
            "_CARRIED_LEAD" => s__CARRIED_LEAD(v),
            "_CARRIED_NOW" => s__CARRIED_NOW(v),
            "_CARRIED_PEOPLE" => s__CARRIED_PEOPLE(v),
            "_CHILLI_THE_VEGETABLE" => s__CHILLI_THE_VEGETABLE(v),
            "_COMPONENT" => s__COMPONENT(v),
            "_COMPONENT_MEASURED" => s__COMPONENT_MEASURED(v),
            "_CONTAINER" => s__CONTAINER(v),
            "_CORRECTION" => s__CORRECTION(v),
            "_COUNT" => s__COUNT(v),
            "_COUNTED_THING" => s__COUNTED_THING(v),
            "_COUNT_ASK" => s__COUNT_ASK(v),
            "_COUNT_CONTENT" => s__COUNT_CONTENT(v),
            "_COUNT_OF_BIRDS" => s__COUNT_OF_BIRDS(v),
            "_C_ASKS" => s__C_ASKS(v),
            "_DENOMINATOR" => s__DENOMINATOR(v),
            "_DISH_AFTER" => s__DISH_AFTER(v),
            "_DRY_IN_A_MEASURE" => s__DRY_IN_A_MEASURE(v),
            "_FEEDS_ASK" => s__FEEDS_ASK(v),
            "_FRACTION" => s__FRACTION(v),
            "_FRACTIONS" => s__FRACTIONS(v),
            "_F_ASKS" => s__F_ASKS(v),
            "_GROUP_VALUE" => s__GROUP_VALUE(v),
            "_HEIGHT_CM" => s__HEIGHT_CM(v),
            "_HEIGHT_FEET" => s__HEIGHT_FEET(v),
            "_HEIGHT_FEET_WORD" => s__HEIGHT_FEET_WORD(v),
            "_HEIGHT_METRES" => s__HEIGHT_METRES(v),
            "_HOW_TO" => s__HOW_TO(v),
            "_IN_A_VESSEL" => s__IN_A_VESSEL(v),
            "_JOINED" => s__JOINED(v),
            "_KEEPING_HENS" => s__KEEPING_HENS(v),
            "_LAND_ASK" => s__LAND_ASK(v),
            "_LOOSE_ASK" => s__LOOSE_ASK(v),
            "_LOOSE_NOT" => s__LOOSE_NOT(v),
            "_MEASURE_AFTER_PIECE" => s__MEASURE_AFTER_PIECE(v),
            "_MIXED" => s__MIXED(v),
            "_NAMES" => s__NAMES(v),
            "_NEW_WEIGHT_THEN" => s__NEW_WEIGHT_THEN(v),
            "_NOT_ABOUT_A_MEASURE" => s__NOT_ABOUT_A_MEASURE(v),
            "_NOT_A_COUNT" => s__NOT_A_COUNT(v),
            "_NOT_A_FOOD" => s__NOT_A_FOOD(v),
            "_NOT_A_HEIGHT" => s__NOT_A_HEIGHT(v),
            "_NOT_A_TABLE_BEFORE" => s__NOT_A_TABLE_BEFORE(v),
            "_NOT_A_TABLE_QUESTION" => s__NOT_A_TABLE_QUESTION(v),
            "_NO_PLURAL" => s__NO_PLURAL(v),
            "_NUMBER" => s__NUMBER(v),
            "_NUMERAL" => s__NUMERAL(v),
            "_PACK_ASK" => s__PACK_ASK(v),
            "_PACK_WORD" => s__PACK_WORD(v),
            "_PEOPLE_COUNT" => s__PEOPLE_COUNT(v),
            "_PEOPLE_VALUE" => s__PEOPLE_VALUE(v),
            "_PEOPLE_WORD" => s__PEOPLE_WORD(v),
            "_PEOPLE_WORD_TO_TWENTY" => s__PEOPLE_WORD_TO_TWENTY(v),
            "_PER_CUP" => s__PER_CUP(v),
            "_PLURAL_BIRDS" => s__PLURAL_BIRDS(v),
            "_PORTIONS_ASK" => s__PORTIONS_ASK(v),
            "_PORTION_AVOID" => s__PORTION_AVOID(v),
            "_PORTION_COUNT" => s__PORTION_COUNT(v),
            "_PORTION_WORD" => s__PORTION_WORD(v),
            "_POUNDS_IS_MONEY" => s__POUNDS_IS_MONEY(v),
            "_SAID_COUNT" => s__SAID_COUNT(v),
            "_SAID_PEOPLE" => s__SAID_PEOPLE(v),
            "_SAID_WHOLE" => s__SAID_WHOLE(v),
            "_SAYS_OWN_HEIGHT" => s__SAYS_OWN_HEIGHT(v),
            "_SAY_K" => s__SAY_K(v),
            "_SHARE" => s__SHARE(v),
            "_SHARE_ASK" => s__SHARE_ASK(v),
            "_SHARE_FRACTION" => s__SHARE_FRACTION(v),
            "_SHARE_LEAD" => s__SHARE_LEAD(v),
            "_SHARE_OF_CUP" => s__SHARE_OF_CUP(v),
            "_SHARE_SAID" => s__SHARE_SAID(v),
            "_SHARE_THAT" => s__SHARE_THAT(v),
            "_SHARE_VALUE" => s__SHARE_VALUE(v),
            "_SLASH" => s__SLASH(v),
            "_SPECIFIC" => s__SPECIFIC(v),
            "_SPLIT" => s__SPLIT(v),
            "_TABLE_GROUPS" => s__TABLE_GROUPS(v),
            "_TARGET" => s__TARGET(v),
            "_TIME_ARTICLE" => s__TIME_ARTICLE(v),
            "_TIME_PLURAL" => s__TIME_PLURAL(v),
            "_TIME_SECONDS" => s__TIME_SECONDS(v),
            "_TIME_SMALLER" => s__TIME_SMALLER(v),
            "_TIME_UNIT" => s__TIME_UNIT(v),
            "_TIN_FOR" => s__TIN_FOR(v),
            "_TRAILING_JOIN" => s__TRAILING_JOIN(v),
            "_UNCOUNTED" => s__UNCOUNTED(v),
            "_UNIT_AND_A_HALF" => s__UNIT_AND_A_HALF(v),
            "_UNIT_WORD" => s__UNIT_WORD(v),
            "_VESSEL_ML" => s__VESSEL_ML(v),
            "_WEIGH" => s__WEIGH(v),
            "_WEIGHT_SAID" => s__WEIGHT_SAID(v),
            "_WHOLE_PIECE" => s__WHOLE_PIECE(v),
            "_WHOLE_THING" => s__WHOLE_THING(v),
            "_WHOLE_WEIGHT" => s__WHOLE_WEIGHT(v),
            "_WORD_FRACTION" => s__WORD_FRACTION(v),
            "_WORD_SHARE" => s__WORD_SHARE(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.units", module_obj);
    pyrt::xrefs::reg_fn("knowledge.units", "_the_component_is_the_whole_measure", py_func("_the_component_is_the_whole_measure", |args: &[Py]| _the_component_is_the_whole_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_the_thing_measured_by_itself", py_func("_the_thing_measured_by_itself", |args: &[Py]| _the_thing_measured_by_itself(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_object_weight", py_func("_object_weight", |args: &[Py]| _object_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_many", py_func("_many", |args: &[Py]| _many(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_kilos", py_func("_kilos", |args: &[Py]| _kilos(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_build", py_func("_build", |args: &[Py]| _build(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_factor", py_func("_factor", |args: &[Py]| _factor(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_words", py_func("_words", |args: &[Py]| _words(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_grams", py_func("_grams", |args: &[Py]| _grams(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_fmt", py_func("_fmt", |args: &[Py]| _fmt(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_value", py_func("_value", |args: &[Py]| _value(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_mentions", py_func("_mentions", |args: &[Py]| _mentions(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_number_before", py_func("_number_before", |args: &[Py]| _number_before(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_say", py_func("_say", |args: &[Py]| _say(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_split_time", py_func("_split_time", |args: &[Py]| _split_time(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_sentence", py_func("_sentence", |args: &[Py]| _sentence(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_temperature", py_func("_temperature", |args: &[Py]| _temperature(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_temperature_rule", py_func("_temperature_rule", |args: &[Py]| _temperature_rule(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_mark_for", py_func("_mark_for", |args: &[Py]| _mark_for(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_oven", py_func("_oven", |args: &[Py]| _oven(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_butter", py_func("_butter", |args: &[Py]| _butter(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_pack", py_func("_pack", |args: &[Py]| _pack(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_singular", py_func("_singular", |args: &[Py]| _singular(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_table_grams", py_func("_table_grams", |args: &[Py]| _table_grams(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_pack_count", py_func("_pack_count", |args: &[Py]| _pack_count(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_in_a_vessel", py_func("_in_a_vessel", |args: &[Py]| _in_a_vessel(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_loose", py_func("_loose", |args: &[Py]| _loose(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_liquid_weight", py_func("_liquid_weight", |args: &[Py]| _liquid_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_mass_said", py_func("_mass_said", |args: &[Py]| _mass_said(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_dish_for_a_table", py_func("_a_dish_for_a_table", |args: &[Py]| _a_dish_for_a_table(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_an_allowance", py_func("_an_allowance", |args: &[Py]| _an_allowance(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_weight", py_func("_a_weight", |args: &[Py]| _a_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "people_in_groups", py_func("people_in_groups", |args: &[Py]| people_in_groups(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_people_asked", py_func("_people_asked", |args: &[Py]| _people_asked(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_people_before", py_func("_people_before", |args: &[Py]| _people_before(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_count_that_still_stands", py_func("_a_count_that_still_stands", |args: &[Py]| _a_count_that_still_stands(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_corrected_figure", py_func("_a_corrected_figure", |args: &[Py]| _a_corrected_figure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_table_without_people", py_func("_a_table_without_people", |args: &[Py]| _a_table_without_people(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_per_person", py_func("_per_person", |args: &[Py]| _per_person(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_said_people", py_func("_said_people", |args: &[Py]| _said_people(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_mass_of", py_func("_mass_of", |args: &[Py]| _mass_of(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_litres_of", py_func("_litres_of", |args: &[Py]| _litres_of(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_count_said", py_func("_count_said", |args: &[Py]| _count_said(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_area_pictures", py_func("_area_pictures", |args: &[Py]| _area_pictures(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_share_before", py_func("_share_before", |args: &[Py]| _share_before(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_land_in_pictures", py_func("_land_in_pictures", |args: &[Py]| _land_in_pictures(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_land_size", py_func("_land_size", |args: &[Py]| _land_size(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_height", py_func("_height", |args: &[Py]| _height(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_is_a_distance", py_func("_is_a_distance", |args: &[Py]| _is_a_distance(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_asks_something_else", py_func("_asks_something_else", |args: &[Py]| _asks_something_else(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_own_height", py_func("_own_height", |args: &[Py]| _own_height(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_metres_in_feet", py_func("_metres_in_feet", |args: &[Py]| _metres_in_feet(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_cup_of_something", py_func("_cup_of_something", |args: &[Py]| _cup_of_something(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_handful", py_func("_handful", |args: &[Py]| _handful(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_asks_for_weight", py_func("_asks_for_weight", |args: &[Py]| _asks_for_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_names_unit", py_func("_names_unit", |args: &[Py]| _names_unit(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_frac", py_func("_frac", |args: &[Py]| _frac(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_rule_sentence", py_func("_rule_sentence", |args: &[Py]| _rule_sentence(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_is_bare_pair", py_func("_is_bare_pair", |args: &[Py]| _is_bare_pair(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_the_ounce_pair", py_func("_the_ounce_pair", |args: &[Py]| _the_ounce_pair(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_how_to_convert", py_func("_how_to_convert", |args: &[Py]| _how_to_convert(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_tail_is_about_the_measure", py_func("_tail_is_about_the_measure", |args: &[Py]| _tail_is_about_the_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_share_value", py_func("_share_value", |args: &[Py]| _share_value(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_share_name", py_func("_share_name", |args: &[Py]| _share_name(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_share_spoons", py_func("_share_spoons", |args: &[Py]| _share_spoons(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_share_of_a_measure", py_func("_share_of_a_measure", |args: &[Py]| _share_of_a_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_tsp_phrase", py_func("_tsp_phrase", |args: &[Py]| _tsp_phrase(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_fraction_value", py_func("_a_fraction_value", |args: &[Py]| _a_fraction_value(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_fraction_name", py_func("_a_fraction_name", |args: &[Py]| _a_fraction_name(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_span_amount", py_func("_a_span_amount", |args: &[Py]| _a_span_amount(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_number_word", py_func("_a_number_word", |args: &[Py]| _a_number_word(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_share_of_a_unit", py_func("_a_share_of_a_unit", |args: &[Py]| _a_share_of_a_unit(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_fraction_of_a_time_unit", py_func("_a_fraction_of_a_time_unit", |args: &[Py]| _a_fraction_of_a_time_unit(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_the_span_said", py_func("_the_span_said", |args: &[Py]| _the_span_said(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_mixed_parts", py_func("_a_mixed_parts", |args: &[Py]| _a_mixed_parts(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_span_of_time", py_func("_a_span_of_time", |args: &[Py]| _a_span_of_time(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_share_carried", py_func("_share_carried", |args: &[Py]| _share_carried(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "convert", py_func("convert", |args: &[Py]| convert(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_count_in_a_line", py_func("_count_in_a_line", |args: &[Py]| _count_in_a_line(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_portions_in_a_weight", py_func("_portions_in_a_weight", |args: &[Py]| _portions_in_a_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_table_word", py_func("_a_table_word", |args: &[Py]| _a_table_word(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_birds_for_a_table", py_func("_birds_for_a_table", |args: &[Py]| _birds_for_a_table(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_people_in_a_count", py_func("_people_in_a_count", |args: &[Py]| _people_in_a_count(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_whole_thing_feeds", py_func("_a_whole_thing_feeds", |args: &[Py]| _a_whole_thing_feeds(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_answer", py_func("_answer", |args: &[Py]| _answer(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_unit_words", py_func("_unit_words", |args: &[Py]| _unit_words(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_bare_measure", py_func("_bare_measure", |args: &[Py]| _bare_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_previous_measure", py_func("_previous_measure", |args: &[Py]| _previous_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_previous_portion", py_func("_previous_portion", |args: &[Py]| _previous_portion(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_carried_people", py_func("_carried_people", |args: &[Py]| _carried_people(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_carried_weight", py_func("_carried_weight", |args: &[Py]| _carried_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_names_a_portion_food", py_func("_names_a_portion_food", |args: &[Py]| _names_a_portion_food(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_portion_after_correction", py_func("_portion_after_correction", |args: &[Py]| _portion_after_correction(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_carried_measure", py_func("_carried_measure", |args: &[Py]| _carried_measure(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a_or_an", py_func("_a_or_an", |args: &[Py]| _a_or_an(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_amount_phrase", py_func("_amount_phrase", |args: &[Py]| _amount_phrase(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_cups_phrase", py_func("_cups_phrase", |args: &[Py]| _cups_phrase(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_spoons", py_func("_spoons", |args: &[Py]| _spoons(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "answers_more_than_a_pair", py_func("answers_more_than_a_pair", |args: &[Py]| answers_more_than_a_pair(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_food_named", py_func("_food_named", |args: &[Py]| _food_named(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_named_ingredient", py_func("_named_ingredient", |args: &[Py]| _named_ingredient(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_plural_of", py_func("_plural_of", |args: &[Py]| _plural_of(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_a", py_func("_a", |args: &[Py]| _a(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_counted_as", py_func("_counted_as", |args: &[Py]| _counted_as(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_piece_named", py_func("_piece_named", |args: &[Py]| _piece_named(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_piece_weight", py_func("_piece_weight", |args: &[Py]| _piece_weight(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_count_phrase", py_func("_count_phrase", |args: &[Py]| _count_phrase(args)));
    pyrt::xrefs::reg_fn("knowledge.units", "_named_unit", py_func("_named_unit", |args: &[Py]| _named_unit(args)));
    pyrt::xrefs::reg_access("knowledge.units", "AS_KITCHEN_SAYS", g_AS_KITCHEN_SAYS, s_AS_KITCHEN_SAYS);
    pyrt::xrefs::reg_access("knowledge.units", "BIRD_GRAMS", g_BIRD_GRAMS, s_BIRD_GRAMS);
    pyrt::xrefs::reg_access("knowledge.units", "CORRECTING", g_CORRECTING, s_CORRECTING);
    pyrt::xrefs::reg_access("knowledge.units", "COUNTED_PACKS", g_COUNTED_PACKS, s_COUNTED_PACKS);
    pyrt::xrefs::reg_access("knowledge.units", "DISHES_FOR_A_TABLE", g_DISHES_FOR_A_TABLE, s_DISHES_FOR_A_TABLE);
    pyrt::xrefs::reg_access("knowledge.units", "FAN_COOLER", g_FAN_COOLER, s_FAN_COOLER);
    pyrt::xrefs::reg_access("knowledge.units", "GAS_OVEN", g_GAS_OVEN, s_GAS_OVEN);
    pyrt::xrefs::reg_access("knowledge.units", "GRAMS_PER_CUP", g_GRAMS_PER_CUP, s_GRAMS_PER_CUP);
    pyrt::xrefs::reg_access("knowledge.units", "GRAMS_PER_TSP", g_GRAMS_PER_TSP, s_GRAMS_PER_TSP);
    pyrt::xrefs::reg_access("knowledge.units", "LAND_PLAIN", g_LAND_PLAIN, s_LAND_PLAIN);
    pyrt::xrefs::reg_access("knowledge.units", "LIQUID_GRAMS_LITRE", g_LIQUID_GRAMS_LITRE, s_LIQUID_GRAMS_LITRE);
    pyrt::xrefs::reg_access("knowledge.units", "LOOSE", g_LOOSE, s_LOOSE);
    pyrt::xrefs::reg_access("knowledge.units", "MADE_NOT_MEASURED", g_MADE_NOT_MEASURED, s_MADE_NOT_MEASURED);
    pyrt::xrefs::reg_access("knowledge.units", "NOT_A_TABLE", g_NOT_A_TABLE, s_NOT_A_TABLE);
    pyrt::xrefs::reg_access("knowledge.units", "OBJECT_GRAMS", g_OBJECT_GRAMS, s_OBJECT_GRAMS);
    pyrt::xrefs::reg_access("knowledge.units", "PACK_DRAINED", g_PACK_DRAINED, s_PACK_DRAINED);
    pyrt::xrefs::reg_access("knowledge.units", "PACK_GRAMS", g_PACK_GRAMS, s_PACK_GRAMS);
    pyrt::xrefs::reg_access("knowledge.units", "PACK_IN_ML", g_PACK_IN_ML, s_PACK_IN_ML);
    pyrt::xrefs::reg_access("knowledge.units", "PER_PERSON", g_PER_PERSON, s_PER_PERSON);
    pyrt::xrefs::reg_access("knowledge.units", "PER_PERSON_WHEN_COUNTED", g_PER_PERSON_WHEN_COUNTED, s_PER_PERSON_WHEN_COUNTED);
    pyrt::xrefs::reg_access("knowledge.units", "PIECE_GRAMS", g_PIECE_GRAMS, s_PIECE_GRAMS);
    pyrt::xrefs::reg_access("knowledge.units", "SAID", g_SAID, s_SAID);
    pyrt::xrefs::reg_access("knowledge.units", "SAID_ONE", g_SAID_ONE, s_SAID_ONE);
    pyrt::xrefs::reg_access("knowledge.units", "SYSTEM", g_SYSTEM, s_SYSTEM);
    pyrt::xrefs::reg_access("knowledge.units", "UNITS", g_UNITS, s_UNITS);
    pyrt::xrefs::reg_access("knowledge.units", "WHOLE_THING_SERVES", g_WHOLE_THING_SERVES, s_WHOLE_THING_SERVES);
    pyrt::xrefs::reg_access("knowledge.units", "_AND_A_HALF", g__AND_A_HALF, s__AND_A_HALF);
    pyrt::xrefs::reg_access("knowledge.units", "_AN_ASK", g__AN_ASK, s__AN_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_AREA_PICTURES", g__AREA_PICTURES, s__AREA_PICTURES);
    pyrt::xrefs::reg_access("knowledge.units", "_ASKED_MEASURE", g__ASKED_MEASURE, s__ASKED_MEASURE);
    pyrt::xrefs::reg_access("knowledge.units", "_ASKING_AFTER_MEASURE", g__ASKING_AFTER_MEASURE, s__ASKING_AFTER_MEASURE);
    pyrt::xrefs::reg_access("knowledge.units", "_ASKS_ELSEWHERE", g__ASKS_ELSEWHERE, s__ASKS_ELSEWHERE);
    pyrt::xrefs::reg_access("knowledge.units", "_ASKS_FEET", g__ASKS_FEET, s__ASKS_FEET);
    pyrt::xrefs::reg_access("knowledge.units", "_ASKS_HOW_MANY", g__ASKS_HOW_MANY, s__ASKS_HOW_MANY);
    pyrt::xrefs::reg_access("knowledge.units", "_A_CLOCK", g__A_CLOCK, s__A_CLOCK);
    pyrt::xrefs::reg_access("knowledge.units", "_A_COMPLAINT", g__A_COMPLAINT, s__A_COMPLAINT);
    pyrt::xrefs::reg_access("knowledge.units", "_A_COUNT", g__A_COUNT, s__A_COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_A_CUT_NOT_A_BIRD", g__A_CUT_NOT_A_BIRD, s__A_CUT_NOT_A_BIRD);
    pyrt::xrefs::reg_access("knowledge.units", "_A_DISH_PART", g__A_DISH_PART, s__A_DISH_PART);
    pyrt::xrefs::reg_access("knowledge.units", "_A_FIGURE", g__A_FIGURE, s__A_FIGURE);
    pyrt::xrefs::reg_access("knowledge.units", "_A_FRACTION", g__A_FRACTION, s__A_FRACTION);
    pyrt::xrefs::reg_access("knowledge.units", "_A_FRACTION_AS", g__A_FRACTION_AS, s__A_FRACTION_AS);
    pyrt::xrefs::reg_access("knowledge.units", "_A_FRACTION_ASK", g__A_FRACTION_ASK, s__A_FRACTION_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_A_FRACTION_OF", g__A_FRACTION_OF, s__A_FRACTION_OF);
    pyrt::xrefs::reg_access("knowledge.units", "_A_FRACTION_OF_A_MIXED", g__A_FRACTION_OF_A_MIXED, s__A_FRACTION_OF_A_MIXED);
    pyrt::xrefs::reg_access("knowledge.units", "_A_HALF_SPAN", g__A_HALF_SPAN, s__A_HALF_SPAN);
    pyrt::xrefs::reg_access("knowledge.units", "_A_MEASURE_QUESTION", g__A_MEASURE_QUESTION, s__A_MEASURE_QUESTION);
    pyrt::xrefs::reg_access("knowledge.units", "_A_MEASURE_SAID_ANOTHER_WAY", g__A_MEASURE_SAID_ANOTHER_WAY, s__A_MEASURE_SAID_ANOTHER_WAY);
    pyrt::xrefs::reg_access("knowledge.units", "_A_MIXED_SPAN", g__A_MIXED_SPAN, s__A_MIXED_SPAN);
    pyrt::xrefs::reg_access("knowledge.units", "_A_SIZE_JUDGEMENT", g__A_SIZE_JUDGEMENT, s__A_SIZE_JUDGEMENT);
    pyrt::xrefs::reg_access("knowledge.units", "_A_SPAN_ASK", g__A_SPAN_ASK, s__A_SPAN_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_A_TARGET_UNIT", g__A_TARGET_UNIT, s__A_TARGET_UNIT);
    pyrt::xrefs::reg_access("knowledge.units", "_A_TIME_IS_THE_ASK", g__A_TIME_IS_THE_ASK, s__A_TIME_IS_THE_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_A_TIME_SHARE", g__A_TIME_SHARE, s__A_TIME_SHARE);
    pyrt::xrefs::reg_access("knowledge.units", "_A_UNIT_ASKED", g__A_UNIT_ASKED, s__A_UNIT_ASKED);
    pyrt::xrefs::reg_access("knowledge.units", "_A_WHOLE_UNIT", g__A_WHOLE_UNIT, s__A_WHOLE_UNIT);
    pyrt::xrefs::reg_access("knowledge.units", "_BAKING_PORTION", g__BAKING_PORTION, s__BAKING_PORTION);
    pyrt::xrefs::reg_access("knowledge.units", "_BARE_FOR_COUNT", g__BARE_FOR_COUNT, s__BARE_FOR_COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_BARE_JOIN", g__BARE_JOIN, s__BARE_JOIN);
    pyrt::xrefs::reg_access("knowledge.units", "_BARE_TEMPERATURE", g__BARE_TEMPERATURE, s__BARE_TEMPERATURE);
    pyrt::xrefs::reg_access("knowledge.units", "_BIRDS", g__BIRDS, s__BIRDS);
    pyrt::xrefs::reg_access("knowledge.units", "_CARRIED_FILLER", g__CARRIED_FILLER, s__CARRIED_FILLER);
    pyrt::xrefs::reg_access("knowledge.units", "_CARRIED_LEAD", g__CARRIED_LEAD, s__CARRIED_LEAD);
    pyrt::xrefs::reg_access("knowledge.units", "_CARRIED_NOW", g__CARRIED_NOW, s__CARRIED_NOW);
    pyrt::xrefs::reg_access("knowledge.units", "_CARRIED_PEOPLE", g__CARRIED_PEOPLE, s__CARRIED_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.units", "_CHILLI_THE_VEGETABLE", g__CHILLI_THE_VEGETABLE, s__CHILLI_THE_VEGETABLE);
    pyrt::xrefs::reg_access("knowledge.units", "_COMPONENT", g__COMPONENT, s__COMPONENT);
    pyrt::xrefs::reg_access("knowledge.units", "_COMPONENT_MEASURED", g__COMPONENT_MEASURED, s__COMPONENT_MEASURED);
    pyrt::xrefs::reg_access("knowledge.units", "_CONTAINER", g__CONTAINER, s__CONTAINER);
    pyrt::xrefs::reg_access("knowledge.units", "_CORRECTION", g__CORRECTION, s__CORRECTION);
    pyrt::xrefs::reg_access("knowledge.units", "_COUNT", g__COUNT, s__COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_COUNTED_THING", g__COUNTED_THING, s__COUNTED_THING);
    pyrt::xrefs::reg_access("knowledge.units", "_COUNT_ASK", g__COUNT_ASK, s__COUNT_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_COUNT_CONTENT", g__COUNT_CONTENT, s__COUNT_CONTENT);
    pyrt::xrefs::reg_access("knowledge.units", "_COUNT_OF_BIRDS", g__COUNT_OF_BIRDS, s__COUNT_OF_BIRDS);
    pyrt::xrefs::reg_access("knowledge.units", "_C_ASKS", g__C_ASKS, s__C_ASKS);
    pyrt::xrefs::reg_access("knowledge.units", "_DENOMINATOR", g__DENOMINATOR, s__DENOMINATOR);
    pyrt::xrefs::reg_access("knowledge.units", "_DISH_AFTER", g__DISH_AFTER, s__DISH_AFTER);
    pyrt::xrefs::reg_access("knowledge.units", "_DRY_IN_A_MEASURE", g__DRY_IN_A_MEASURE, s__DRY_IN_A_MEASURE);
    pyrt::xrefs::reg_access("knowledge.units", "_FEEDS_ASK", g__FEEDS_ASK, s__FEEDS_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_FRACTION", g__FRACTION, s__FRACTION);
    pyrt::xrefs::reg_access("knowledge.units", "_FRACTIONS", g__FRACTIONS, s__FRACTIONS);
    pyrt::xrefs::reg_access("knowledge.units", "_F_ASKS", g__F_ASKS, s__F_ASKS);
    pyrt::xrefs::reg_access("knowledge.units", "_GROUP_VALUE", g__GROUP_VALUE, s__GROUP_VALUE);
    pyrt::xrefs::reg_access("knowledge.units", "_HEIGHT_CM", g__HEIGHT_CM, s__HEIGHT_CM);
    pyrt::xrefs::reg_access("knowledge.units", "_HEIGHT_FEET", g__HEIGHT_FEET, s__HEIGHT_FEET);
    pyrt::xrefs::reg_access("knowledge.units", "_HEIGHT_FEET_WORD", g__HEIGHT_FEET_WORD, s__HEIGHT_FEET_WORD);
    pyrt::xrefs::reg_access("knowledge.units", "_HEIGHT_METRES", g__HEIGHT_METRES, s__HEIGHT_METRES);
    pyrt::xrefs::reg_access("knowledge.units", "_HOW_TO", g__HOW_TO, s__HOW_TO);
    pyrt::xrefs::reg_access("knowledge.units", "_IN_A_VESSEL", g__IN_A_VESSEL, s__IN_A_VESSEL);
    pyrt::xrefs::reg_access("knowledge.units", "_JOINED", g__JOINED, s__JOINED);
    pyrt::xrefs::reg_access("knowledge.units", "_KEEPING_HENS", g__KEEPING_HENS, s__KEEPING_HENS);
    pyrt::xrefs::reg_access("knowledge.units", "_LAND_ASK", g__LAND_ASK, s__LAND_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_LOOSE_ASK", g__LOOSE_ASK, s__LOOSE_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_LOOSE_NOT", g__LOOSE_NOT, s__LOOSE_NOT);
    pyrt::xrefs::reg_access("knowledge.units", "_MEASURE_AFTER_PIECE", g__MEASURE_AFTER_PIECE, s__MEASURE_AFTER_PIECE);
    pyrt::xrefs::reg_access("knowledge.units", "_MIXED", g__MIXED, s__MIXED);
    pyrt::xrefs::reg_access("knowledge.units", "_NAMES", g__NAMES, s__NAMES);
    pyrt::xrefs::reg_access("knowledge.units", "_NEW_WEIGHT_THEN", g__NEW_WEIGHT_THEN, s__NEW_WEIGHT_THEN);
    pyrt::xrefs::reg_access("knowledge.units", "_NOT_ABOUT_A_MEASURE", g__NOT_ABOUT_A_MEASURE, s__NOT_ABOUT_A_MEASURE);
    pyrt::xrefs::reg_access("knowledge.units", "_NOT_A_COUNT", g__NOT_A_COUNT, s__NOT_A_COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_NOT_A_FOOD", g__NOT_A_FOOD, s__NOT_A_FOOD);
    pyrt::xrefs::reg_access("knowledge.units", "_NOT_A_HEIGHT", g__NOT_A_HEIGHT, s__NOT_A_HEIGHT);
    pyrt::xrefs::reg_access("knowledge.units", "_NOT_A_TABLE_BEFORE", g__NOT_A_TABLE_BEFORE, s__NOT_A_TABLE_BEFORE);
    pyrt::xrefs::reg_access("knowledge.units", "_NOT_A_TABLE_QUESTION", g__NOT_A_TABLE_QUESTION, s__NOT_A_TABLE_QUESTION);
    pyrt::xrefs::reg_access("knowledge.units", "_NO_PLURAL", g__NO_PLURAL, s__NO_PLURAL);
    pyrt::xrefs::reg_access("knowledge.units", "_NUMBER", g__NUMBER, s__NUMBER);
    pyrt::xrefs::reg_access("knowledge.units", "_NUMERAL", g__NUMERAL, s__NUMERAL);
    pyrt::xrefs::reg_access("knowledge.units", "_PACK_ASK", g__PACK_ASK, s__PACK_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_PACK_WORD", g__PACK_WORD, s__PACK_WORD);
    pyrt::xrefs::reg_access("knowledge.units", "_PEOPLE_COUNT", g__PEOPLE_COUNT, s__PEOPLE_COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_PEOPLE_VALUE", g__PEOPLE_VALUE, s__PEOPLE_VALUE);
    pyrt::xrefs::reg_access("knowledge.units", "_PEOPLE_WORD", g__PEOPLE_WORD, s__PEOPLE_WORD);
    pyrt::xrefs::reg_access("knowledge.units", "_PEOPLE_WORD_TO_TWENTY", g__PEOPLE_WORD_TO_TWENTY, s__PEOPLE_WORD_TO_TWENTY);
    pyrt::xrefs::reg_access("knowledge.units", "_PER_CUP", g__PER_CUP, s__PER_CUP);
    pyrt::xrefs::reg_access("knowledge.units", "_PLURAL_BIRDS", g__PLURAL_BIRDS, s__PLURAL_BIRDS);
    pyrt::xrefs::reg_access("knowledge.units", "_PORTIONS_ASK", g__PORTIONS_ASK, s__PORTIONS_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_PORTION_AVOID", g__PORTION_AVOID, s__PORTION_AVOID);
    pyrt::xrefs::reg_access("knowledge.units", "_PORTION_COUNT", g__PORTION_COUNT, s__PORTION_COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_PORTION_WORD", g__PORTION_WORD, s__PORTION_WORD);
    pyrt::xrefs::reg_access("knowledge.units", "_POUNDS_IS_MONEY", g__POUNDS_IS_MONEY, s__POUNDS_IS_MONEY);
    pyrt::xrefs::reg_access("knowledge.units", "_SAID_COUNT", g__SAID_COUNT, s__SAID_COUNT);
    pyrt::xrefs::reg_access("knowledge.units", "_SAID_PEOPLE", g__SAID_PEOPLE, s__SAID_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.units", "_SAID_WHOLE", g__SAID_WHOLE, s__SAID_WHOLE);
    pyrt::xrefs::reg_access("knowledge.units", "_SAYS_OWN_HEIGHT", g__SAYS_OWN_HEIGHT, s__SAYS_OWN_HEIGHT);
    pyrt::xrefs::reg_access("knowledge.units", "_SAY_K", g__SAY_K, s__SAY_K);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE", g__SHARE, s__SHARE);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_ASK", g__SHARE_ASK, s__SHARE_ASK);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_FRACTION", g__SHARE_FRACTION, s__SHARE_FRACTION);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_LEAD", g__SHARE_LEAD, s__SHARE_LEAD);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_OF_CUP", g__SHARE_OF_CUP, s__SHARE_OF_CUP);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_SAID", g__SHARE_SAID, s__SHARE_SAID);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_THAT", g__SHARE_THAT, s__SHARE_THAT);
    pyrt::xrefs::reg_access("knowledge.units", "_SHARE_VALUE", g__SHARE_VALUE, s__SHARE_VALUE);
    pyrt::xrefs::reg_access("knowledge.units", "_SLASH", g__SLASH, s__SLASH);
    pyrt::xrefs::reg_access("knowledge.units", "_SPECIFIC", g__SPECIFIC, s__SPECIFIC);
    pyrt::xrefs::reg_access("knowledge.units", "_SPLIT", g__SPLIT, s__SPLIT);
    pyrt::xrefs::reg_access("knowledge.units", "_TABLE_GROUPS", g__TABLE_GROUPS, s__TABLE_GROUPS);
    pyrt::xrefs::reg_access("knowledge.units", "_TARGET", g__TARGET, s__TARGET);
    pyrt::xrefs::reg_access("knowledge.units", "_TIME_ARTICLE", g__TIME_ARTICLE, s__TIME_ARTICLE);
    pyrt::xrefs::reg_access("knowledge.units", "_TIME_PLURAL", g__TIME_PLURAL, s__TIME_PLURAL);
    pyrt::xrefs::reg_access("knowledge.units", "_TIME_SECONDS", g__TIME_SECONDS, s__TIME_SECONDS);
    pyrt::xrefs::reg_access("knowledge.units", "_TIME_SMALLER", g__TIME_SMALLER, s__TIME_SMALLER);
    pyrt::xrefs::reg_access("knowledge.units", "_TIME_UNIT", g__TIME_UNIT, s__TIME_UNIT);
    pyrt::xrefs::reg_access("knowledge.units", "_TIN_FOR", g__TIN_FOR, s__TIN_FOR);
    pyrt::xrefs::reg_access("knowledge.units", "_TRAILING_JOIN", g__TRAILING_JOIN, s__TRAILING_JOIN);
    pyrt::xrefs::reg_access("knowledge.units", "_UNCOUNTED", g__UNCOUNTED, s__UNCOUNTED);
    pyrt::xrefs::reg_access("knowledge.units", "_UNIT_AND_A_HALF", g__UNIT_AND_A_HALF, s__UNIT_AND_A_HALF);
    pyrt::xrefs::reg_access("knowledge.units", "_UNIT_WORD", g__UNIT_WORD, s__UNIT_WORD);
    pyrt::xrefs::reg_access("knowledge.units", "_VESSEL_ML", g__VESSEL_ML, s__VESSEL_ML);
    pyrt::xrefs::reg_access("knowledge.units", "_WEIGH", g__WEIGH, s__WEIGH);
    pyrt::xrefs::reg_access("knowledge.units", "_WEIGHT_SAID", g__WEIGHT_SAID, s__WEIGHT_SAID);
    pyrt::xrefs::reg_access("knowledge.units", "_WHOLE_PIECE", g__WHOLE_PIECE, s__WHOLE_PIECE);
    pyrt::xrefs::reg_access("knowledge.units", "_WHOLE_THING", g__WHOLE_THING, s__WHOLE_THING);
    pyrt::xrefs::reg_access("knowledge.units", "_WHOLE_WEIGHT", g__WHOLE_WEIGHT, s__WHOLE_WEIGHT);
    pyrt::xrefs::reg_access("knowledge.units", "_WORD_FRACTION", g__WORD_FRACTION, s__WORD_FRACTION);
    pyrt::xrefs::reg_access("knowledge.units", "_WORD_SHARE", g__WORD_SHARE, s__WORD_SHARE);
}

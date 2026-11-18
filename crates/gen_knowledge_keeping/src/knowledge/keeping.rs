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
thread_local! { static G_ASKS_HOW_LONG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BLUE_CHEESES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEESE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COLD_REPLIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_IN_THE_ROW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_VEG_COOK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_VEG_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_VEG_REPLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_VEG_STAND_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKED_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOKING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COOK_LINE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_OUT_FOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CUT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATED_STAND_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATES_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATE_ROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DATING_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DISH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FEEDING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIX_FIRST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLOP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FOLLOW_UP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREEZE_SWEETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREEZE_SWEET_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREEZE_SWEET_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FREEZE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROWING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEPING_KEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_NOT_KEEPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_NOT_MEDICINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_NOT_SLEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_QUALITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_TECHNIQUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KEEP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LONG_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MOULD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OATS_BAG: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OATS_COOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_ANY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_BERRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_CITRUS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_FIRM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_LEAVES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_SOFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACK_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PASTA_SAUCE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PROBLEM_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REFREEZE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RIPEN_ASIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RIPEN_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RIPEN_TABLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RIPEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROW_COOK_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROW_STAND_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SAUCE_KEEPING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SC_KEEP_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SC_KEEP_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SC_KEEP_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOFT_CHEESES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STORECUPBOARD_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STORE_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TELL_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THAW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TIMES_A_THAW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNDER_RIPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNRIPE_STATED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_USE_UP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHEN_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YOGHURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CLEANING_WITH_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_KEEP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_SAFE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_STATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CRUNCH_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_AFTER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_AFTER_AVOID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_AFTER_ENTRY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAY_AFTER_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__EATING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FLOUR_LUMPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FLOUR_MITES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LAST_TIME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSH_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MUSH_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_A_FRUIT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__OFF_GROUP_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RIPEN_ASIDE_KEEPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SALT_CLUMPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAND_DOWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STAND_DOWN_HARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__STATE_NOT_FOOD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOGHURT_FIX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__YOGHURT_WHEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.keeping: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASKS_HOW_LONG() -> Py { ensure_init(); G_ASKS_HOW_LONG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASKS_HOW_LONG(v: Py) { ensure_init(); G_ASKS_HOW_LONG.with(|c| *c.borrow_mut() = v); }
pub fn g_BLUE_CHEESES() -> Py { ensure_init(); G_BLUE_CHEESES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BLUE_CHEESES(v: Py) { ensure_init(); G_BLUE_CHEESES.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR_DATE() -> Py { ensure_init(); G_CALENDAR_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR_DATE(v: Py) { ensure_init(); G_CALENDAR_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEESE_NAMES() -> Py { ensure_init(); G_CHEESE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEESE_NAMES(v: Py) { ensure_init(); G_CHEESE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_NOT() -> Py { ensure_init(); G_COLD_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_NOT(v: Py) { ensure_init(); G_COLD_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_COLD_REPLIES() -> Py { ensure_init(); G_COLD_REPLIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COLD_REPLIES(v: Py) { ensure_init(); G_COLD_REPLIES.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_IN_THE_ROW() -> Py { ensure_init(); G_COOKED_IN_THE_ROW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_IN_THE_ROW(v: Py) { ensure_init(); G_COOKED_IN_THE_ROW.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_VEG_COOK_WORDS() -> Py { ensure_init(); G_COOKED_VEG_COOK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_VEG_COOK_WORDS(v: Py) { ensure_init(); G_COOKED_VEG_COOK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_VEG_NAMES() -> Py { ensure_init(); G_COOKED_VEG_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_VEG_NAMES(v: Py) { ensure_init(); G_COOKED_VEG_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_VEG_REPLY() -> Py { ensure_init(); G_COOKED_VEG_REPLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_VEG_REPLY(v: Py) { ensure_init(); G_COOKED_VEG_REPLY.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_VEG_STAND_DOWN() -> Py { ensure_init(); G_COOKED_VEG_STAND_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_VEG_STAND_DOWN(v: Py) { ensure_init(); G_COOKED_VEG_STAND_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKED_WORDS() -> Py { ensure_init(); G_COOKED_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKED_WORDS(v: Py) { ensure_init(); G_COOKED_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOKING_WORDS() -> Py { ensure_init(); G_COOKING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOKING_WORDS(v: Py) { ensure_init(); G_COOKING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_COOK_LINE_WORDS() -> Py { ensure_init(); G_COOK_LINE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COOK_LINE_WORDS(v: Py) { ensure_init(); G_COOK_LINE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_OUT_FOR() -> Py { ensure_init(); G_CUT_OUT_FOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_OUT_FOR(v: Py) { ensure_init(); G_CUT_OUT_FOR.with(|c| *c.borrow_mut() = v); }
pub fn g_CUT_WORDS() -> Py { ensure_init(); G_CUT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CUT_WORDS(v: Py) { ensure_init(); G_CUT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DATED_STAND_DOWN() -> Py { ensure_init(); G_DATED_STAND_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATED_STAND_DOWN(v: Py) { ensure_init(); G_DATED_STAND_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_DATES_OWN() -> Py { ensure_init(); G_DATES_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATES_OWN(v: Py) { ensure_init(); G_DATES_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_ASK() -> Py { ensure_init(); G_DATE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_ASK(v: Py) { ensure_init(); G_DATE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DATE_ROWS() -> Py { ensure_init(); G_DATE_ROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATE_ROWS(v: Py) { ensure_init(); G_DATE_ROWS.with(|c| *c.borrow_mut() = v); }
pub fn g_DATING_DATE() -> Py { ensure_init(); G_DATING_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DATING_DATE(v: Py) { ensure_init(); G_DATING_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_ASK() -> Py { ensure_init(); G_DISH_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_ASK(v: Py) { ensure_init(); G_DISH_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_DISH_WORDS() -> Py { ensure_init(); G_DISH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DISH_WORDS(v: Py) { ensure_init(); G_DISH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_FEEDING_WORDS() -> Py { ensure_init(); G_FEEDING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FEEDING_WORDS(v: Py) { ensure_init(); G_FEEDING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FIX_FIRST() -> Py { ensure_init(); G_FIX_FIRST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIX_FIRST(v: Py) { ensure_init(); G_FIX_FIRST.with(|c| *c.borrow_mut() = v); }
pub fn g_FLOP_WORDS() -> Py { ensure_init(); G_FLOP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLOP_WORDS(v: Py) { ensure_init(); G_FLOP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FOLLOW_UP_WORDS() -> Py { ensure_init(); G_FOLLOW_UP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FOLLOW_UP_WORDS(v: Py) { ensure_init(); G_FOLLOW_UP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_FREEZE_SWEETS() -> Py { ensure_init(); G_FREEZE_SWEETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREEZE_SWEETS(v: Py) { ensure_init(); G_FREEZE_SWEETS.with(|c| *c.borrow_mut() = v); }
pub fn g_FREEZE_SWEET_AVOID() -> Py { ensure_init(); G_FREEZE_SWEET_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREEZE_SWEET_AVOID(v: Py) { ensure_init(); G_FREEZE_SWEET_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_FREEZE_SWEET_NAMES() -> Py { ensure_init(); G_FREEZE_SWEET_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREEZE_SWEET_NAMES(v: Py) { ensure_init(); G_FREEZE_SWEET_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_FREEZE_WORDS() -> Py { ensure_init(); G_FREEZE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FREEZE_WORDS(v: Py) { ensure_init(); G_FREEZE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_GROWING_WORDS() -> Py { ensure_init(); G_GROWING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROWING_WORDS(v: Py) { ensure_init(); G_GROWING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEPING_KEY() -> Py { ensure_init(); G_KEEPING_KEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEPING_KEY(v: Py) { ensure_init(); G_KEEPING_KEY.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_ASKS() -> Py { ensure_init(); G_KEEP_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_ASKS(v: Py) { ensure_init(); G_KEEP_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_NOT_KEEPING() -> Py { ensure_init(); G_KEEP_NOT_KEEPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_NOT_KEEPING(v: Py) { ensure_init(); G_KEEP_NOT_KEEPING.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_NOT_MEDICINE() -> Py { ensure_init(); G_KEEP_NOT_MEDICINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_NOT_MEDICINE(v: Py) { ensure_init(); G_KEEP_NOT_MEDICINE.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_NOT_SLEEP() -> Py { ensure_init(); G_KEEP_NOT_SLEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_NOT_SLEEP(v: Py) { ensure_init(); G_KEEP_NOT_SLEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_QUALITY_WORDS() -> Py { ensure_init(); G_KEEP_QUALITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_QUALITY_WORDS(v: Py) { ensure_init(); G_KEEP_QUALITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_TECHNIQUE() -> Py { ensure_init(); G_KEEP_TECHNIQUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_TECHNIQUE(v: Py) { ensure_init(); G_KEEP_TECHNIQUE.with(|c| *c.borrow_mut() = v); }
pub fn g_KEEP_WORDS() -> Py { ensure_init(); G_KEEP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KEEP_WORDS(v: Py) { ensure_init(); G_KEEP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_LONG_WORDS() -> Py { ensure_init(); G_LONG_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LONG_WORDS(v: Py) { ensure_init(); G_LONG_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MOULD_WORDS() -> Py { ensure_init(); G_MOULD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MOULD_WORDS(v: Py) { ensure_init(); G_MOULD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OATS_BAG() -> Py { ensure_init(); G_OATS_BAG.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OATS_BAG(v: Py) { ensure_init(); G_OATS_BAG.with(|c| *c.borrow_mut() = v); }
pub fn g_OATS_COOKED() -> Py { ensure_init(); G_OATS_COOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OATS_COOKED(v: Py) { ensure_init(); G_OATS_COOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_ANY() -> Py { ensure_init(); G_OFF_ANY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_ANY(v: Py) { ensure_init(); G_OFF_ANY.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_AVOID() -> Py { ensure_init(); G_OFF_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_AVOID(v: Py) { ensure_init(); G_OFF_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_BERRIES() -> Py { ensure_init(); G_OFF_BERRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_BERRIES(v: Py) { ensure_init(); G_OFF_BERRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_CITRUS() -> Py { ensure_init(); G_OFF_CITRUS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_CITRUS(v: Py) { ensure_init(); G_OFF_CITRUS.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_FIRM() -> Py { ensure_init(); G_OFF_FIRM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_FIRM(v: Py) { ensure_init(); G_OFF_FIRM.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_LEAVES() -> Py { ensure_init(); G_OFF_LEAVES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_LEAVES(v: Py) { ensure_init(); G_OFF_LEAVES.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_NAMES() -> Py { ensure_init(); G_OFF_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_NAMES(v: Py) { ensure_init(); G_OFF_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_SOFT() -> Py { ensure_init(); G_OFF_SOFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_SOFT(v: Py) { ensure_init(); G_OFF_SOFT.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_WORDS() -> Py { ensure_init(); G_OFF_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_WORDS(v: Py) { ensure_init(); G_OFF_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PACK_DATE() -> Py { ensure_init(); G_PACK_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACK_DATE(v: Py) { ensure_init(); G_PACK_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PASTA_SAUCE_WORDS() -> Py { ensure_init(); G_PASTA_SAUCE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PASTA_SAUCE_WORDS(v: Py) { ensure_init(); G_PASTA_SAUCE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PROBLEM_ONLY() -> Py { ensure_init(); G_PROBLEM_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PROBLEM_ONLY(v: Py) { ensure_init(); G_PROBLEM_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_REFREEZE_WORDS() -> Py { ensure_init(); G_REFREEZE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REFREEZE_WORDS(v: Py) { ensure_init(); G_REFREEZE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RIPEN_ASIDE() -> Py { ensure_init(); G_RIPEN_ASIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RIPEN_ASIDE(v: Py) { ensure_init(); G_RIPEN_ASIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_RIPEN_ASKS() -> Py { ensure_init(); G_RIPEN_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RIPEN_ASKS(v: Py) { ensure_init(); G_RIPEN_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_RIPEN_TABLE() -> Py { ensure_init(); G_RIPEN_TABLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RIPEN_TABLE(v: Py) { ensure_init(); G_RIPEN_TABLE.with(|c| *c.borrow_mut() = v); }
pub fn g_RIPEN_WORDS() -> Py { ensure_init(); G_RIPEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RIPEN_WORDS(v: Py) { ensure_init(); G_RIPEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ROWS() -> Py { ensure_init(); G_ROWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROWS(v: Py) { ensure_init(); G_ROWS.with(|c| *c.borrow_mut() = v); }
pub fn g_ROW_COOK_FORMS() -> Py { ensure_init(); G_ROW_COOK_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROW_COOK_FORMS(v: Py) { ensure_init(); G_ROW_COOK_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_ROW_STAND_DOWN() -> Py { ensure_init(); G_ROW_STAND_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROW_STAND_DOWN(v: Py) { ensure_init(); G_ROW_STAND_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g_SAUCE_KEEPING_WORDS() -> Py { ensure_init(); G_SAUCE_KEEPING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SAUCE_KEEPING_WORDS(v: Py) { ensure_init(); G_SAUCE_KEEPING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SC_KEEP_ASKS() -> Py { ensure_init(); G_SC_KEEP_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SC_KEEP_ASKS(v: Py) { ensure_init(); G_SC_KEEP_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_SC_KEEP_AVOID() -> Py { ensure_init(); G_SC_KEEP_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SC_KEEP_AVOID(v: Py) { ensure_init(); G_SC_KEEP_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g_SC_KEEP_NAMES() -> Py { ensure_init(); G_SC_KEEP_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SC_KEEP_NAMES(v: Py) { ensure_init(); G_SC_KEEP_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_SOFT_CHEESES() -> Py { ensure_init(); G_SOFT_CHEESES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOFT_CHEESES(v: Py) { ensure_init(); G_SOFT_CHEESES.with(|c| *c.borrow_mut() = v); }
pub fn g_STORECUPBOARD_KEEP() -> Py { ensure_init(); G_STORECUPBOARD_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STORECUPBOARD_KEEP(v: Py) { ensure_init(); G_STORECUPBOARD_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g_STORE_ASKS() -> Py { ensure_init(); G_STORE_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STORE_ASKS(v: Py) { ensure_init(); G_STORE_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_TELL_ASKS() -> Py { ensure_init(); G_TELL_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TELL_ASKS(v: Py) { ensure_init(); G_TELL_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_THAW_WORDS() -> Py { ensure_init(); G_THAW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THAW_WORDS(v: Py) { ensure_init(); G_THAW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TIMES_A_THAW() -> Py { ensure_init(); G_TIMES_A_THAW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TIMES_A_THAW(v: Py) { ensure_init(); G_TIMES_A_THAW.with(|c| *c.borrow_mut() = v); }
pub fn g_UNDER_RIPE() -> Py { ensure_init(); G_UNDER_RIPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNDER_RIPE(v: Py) { ensure_init(); G_UNDER_RIPE.with(|c| *c.borrow_mut() = v); }
pub fn g_UNRIPE_STATED() -> Py { ensure_init(); G_UNRIPE_STATED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNRIPE_STATED(v: Py) { ensure_init(); G_UNRIPE_STATED.with(|c| *c.borrow_mut() = v); }
pub fn g_USE_UP_WORDS() -> Py { ensure_init(); G_USE_UP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_USE_UP_WORDS(v: Py) { ensure_init(); G_USE_UP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_WHEN_WORDS() -> Py { ensure_init(); G_WHEN_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHEN_WORDS(v: Py) { ensure_init(); G_WHEN_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_YOGHURT() -> Py { ensure_init(); G_YOGHURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YOGHURT(v: Py) { ensure_init(); G_YOGHURT.with(|c| *c.borrow_mut() = v); }
pub fn g__CLEANING_WITH_IT() -> Py { ensure_init(); G__CLEANING_WITH_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CLEANING_WITH_IT(v: Py) { ensure_init(); G__CLEANING_WITH_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_FIX() -> Py { ensure_init(); G__CRUNCH_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_FIX(v: Py) { ensure_init(); G__CRUNCH_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_FOODS() -> Py { ensure_init(); G__CRUNCH_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_FOODS(v: Py) { ensure_init(); G__CRUNCH_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_KEEP() -> Py { ensure_init(); G__CRUNCH_KEEP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_KEEP(v: Py) { ensure_init(); G__CRUNCH_KEEP.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_OWN() -> Py { ensure_init(); G__CRUNCH_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_OWN(v: Py) { ensure_init(); G__CRUNCH_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_SAFE() -> Py { ensure_init(); G__CRUNCH_SAFE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_SAFE(v: Py) { ensure_init(); G__CRUNCH_SAFE.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_STATE() -> Py { ensure_init(); G__CRUNCH_STATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_STATE(v: Py) { ensure_init(); G__CRUNCH_STATE.with(|c| *c.borrow_mut() = v); }
pub fn g__CRUNCH_VERDICT() -> Py { ensure_init(); G__CRUNCH_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CRUNCH_VERDICT(v: Py) { ensure_init(); G__CRUNCH_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_AFTER() -> Py { ensure_init(); G__DAY_AFTER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_AFTER(v: Py) { ensure_init(); G__DAY_AFTER.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_AFTER_AVOID() -> Py { ensure_init(); G__DAY_AFTER_AVOID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_AFTER_AVOID(v: Py) { ensure_init(); G__DAY_AFTER_AVOID.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_AFTER_ENTRY() -> Py { ensure_init(); G__DAY_AFTER_ENTRY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_AFTER_ENTRY(v: Py) { ensure_init(); G__DAY_AFTER_ENTRY.with(|c| *c.borrow_mut() = v); }
pub fn g__DAY_AFTER_FOODS() -> Py { ensure_init(); G__DAY_AFTER_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAY_AFTER_FOODS(v: Py) { ensure_init(); G__DAY_AFTER_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g__EATING_WORDS() -> Py { ensure_init(); G__EATING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__EATING_WORDS(v: Py) { ensure_init(); G__EATING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FLOUR_LUMPS() -> Py { ensure_init(); G__FLOUR_LUMPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FLOUR_LUMPS(v: Py) { ensure_init(); G__FLOUR_LUMPS.with(|c| *c.borrow_mut() = v); }
pub fn g__FLOUR_MITES() -> Py { ensure_init(); G__FLOUR_MITES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FLOUR_MITES(v: Py) { ensure_init(); G__FLOUR_MITES.with(|c| *c.borrow_mut() = v); }
pub fn g__LAST_TIME() -> Py { ensure_init(); G__LAST_TIME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LAST_TIME(v: Py) { ensure_init(); G__LAST_TIME.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSH_FIX() -> Py { ensure_init(); G__MUSH_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSH_FIX(v: Py) { ensure_init(); G__MUSH_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g__MUSH_VERDICT() -> Py { ensure_init(); G__MUSH_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MUSH_VERDICT(v: Py) { ensure_init(); G__MUSH_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_A_FRUIT() -> Py { ensure_init(); G__NOT_A_FRUIT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_A_FRUIT(v: Py) { ensure_init(); G__NOT_A_FRUIT.with(|c| *c.borrow_mut() = v); }
pub fn g__OFF_GROUP_WORDS() -> Py { ensure_init(); G__OFF_GROUP_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__OFF_GROUP_WORDS(v: Py) { ensure_init(); G__OFF_GROUP_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__RIPEN_ASIDE_KEEPING() -> Py { ensure_init(); G__RIPEN_ASIDE_KEEPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RIPEN_ASIDE_KEEPING(v: Py) { ensure_init(); G__RIPEN_ASIDE_KEEPING.with(|c| *c.borrow_mut() = v); }
pub fn g__SALT_CLUMPS() -> Py { ensure_init(); G__SALT_CLUMPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SALT_CLUMPS(v: Py) { ensure_init(); G__SALT_CLUMPS.with(|c| *c.borrow_mut() = v); }
pub fn g__STAND_DOWN() -> Py { ensure_init(); G__STAND_DOWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAND_DOWN(v: Py) { ensure_init(); G__STAND_DOWN.with(|c| *c.borrow_mut() = v); }
pub fn g__STAND_DOWN_HARD() -> Py { ensure_init(); G__STAND_DOWN_HARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STAND_DOWN_HARD(v: Py) { ensure_init(); G__STAND_DOWN_HARD.with(|c| *c.borrow_mut() = v); }
pub fn g__STATE_NOT_FOOD() -> Py { ensure_init(); G__STATE_NOT_FOOD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__STATE_NOT_FOOD(v: Py) { ensure_init(); G__STATE_NOT_FOOD.with(|c| *c.borrow_mut() = v); }
pub fn g__YOGHURT_FIX() -> Py { ensure_init(); G__YOGHURT_FIX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOGHURT_FIX(v: Py) { ensure_init(); G__YOGHURT_FIX.with(|c| *c.borrow_mut() = v); }
pub fn g__YOGHURT_WHEY() -> Py { ensure_init(); G__YOGHURT_WHEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__YOGHURT_WHEY(v: Py) { ensure_init(); G__YOGHURT_WHEY.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Shelf life: the store cupboard, the salad drawer and the salad.\n\n\"What can I do with celery\" and \"how long does celery keep\" are the same food and two different\nquestions. `bot/knowledge/cooking.py` answers the first (one dish per ingredient) and its `KEEPS`\ntable answers the second for the fresh things in the fridge; this domain is the same kind of\ntable for the foods that table does not name yet. It is a table, not a mechanism: the key words,\nthe stand-downs and the `entry` shape are imported from `cooking.py`, so there is one set of\nkeeping words in the bot and a row here is data.\n\nThe rule for a row is the one the table has: reply is concrete, leads with the answer, and gives\nthe number (how long, how cold, how soon). A line that is not a shelf-life question passes, which\nhere means the row names the food and a keeping word - \"is celery healthy\" is a nutrition\nquestion with a different answer, so those words take the line away from the table.\n");
    s_COOKED_WORDS(pyrt::xrefs::xget("knowledge.cooking", "COOKED_WORDS"));
    s_COOKING_WORDS(pyrt::xrefs::xget("knowledge.cooking", "COOKING_WORDS"));
    s_KEEP_WORDS(pyrt::xrefs::xget("knowledge.cooking", "KEEP_WORDS"));
    s_USE_UP_WORDS(pyrt::xrefs::xget("knowledge.cooking", "USE_UP_WORDS"));
    s_KEEP_QUALITY_WORDS(pyrt::xrefs::xget("knowledge.dishes", "KEEP_QUALITY_WORDS"));
    s_DISH_ASK(pyrt::xrefs::xget("knowledge.lookup", "DISH_ASK"));
    s_PACK_DATE(pyrt::xrefs::xget("knowledge.lookup", "PACK_DATE"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("healthy"), py_str("unhealthy"), py_str("good for you"), py_str("bad for you"), py_str("fattening"), py_str("calories"), py_str("carbs"), py_str("carbohydrates"), py_str("protein"), py_str("vitamins"), py_str("nutrition"), py_str("nutritious"), py_str("diet"), py_str("dieting"), py_str("superfood"), py_str("keep me up"), py_str("keeps me up"), py_str("keep you up"), py_str("keeps you up"), py_str("keep me awake"), py_str("keeps me awake"), py_str("keep you awake"), py_str("keeps you awake"), py_str("awake at night"), py_str("stay awake"), py_str("cant sleep"), py_str("can't sleep")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.KEEP_NOT_KEEPING"); }
    s_KEEP_NOT_KEEPING(__data1());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OATS_BAG"); }
    s_OATS_BAG(py_str("Porridge oats keep about 6 months in an airtight jar in a cool, dry cupboard, and a sealed bag that has stayed dry is still fine a year past the date on it. Ground oats and oat bran go off sooner, about 3 months, because the oils in them turn: smell the bag, and oats that smell dusty or taste bitter have gone. Cooked porridge is a keep-two-days food, cooled quickly and reheated with a splash of milk."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OATS_COOKED"); }
    s_OATS_COOKED(py_str("Cooked porridge keeps 2 days in the fridge in a covered bowl, and reheat it once with a splash of milk or water over a low heat, because it sets solid as it cools - the same two days cooked rice gets, and for the same reason. Dry oats in the bag keep about 6 months in an airtight jar in a cool, dry cupboard, and longer if the bag stays sealed."));
    fn __data2() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("flour"), py_str("plain flour"), py_str("self raising flour"), py_str("self-raising flour"), py_str("bread flour")]), py_str("Flour keeps 6 months in a sealed jar in a cool cupboard and a year in the freezer; wholemeal and brown flour go off sooner, 3 months, and smell of old paint when they do. Put a bay leaf in the jar and the weevils leave it alone.")]), py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("ground coffee"), py_str("coffee beans"), py_str("instant coffee"), py_str("coffee granules")]), py_str("Ground coffee is at its best in the 2 weeks after opening: keep it in an airtight tin in a dark cupboard rather than the fridge, which is damp and smells of everything else. Beans keep a month, and more than a month's worth is better frozen and ground from frozen.")]), py_tuple(vec![py_tuple(vec![py_str("spices"), py_str("spice"), py_str("ground spices"), py_str("paprika"), py_str("cumin"), py_str("turmeric"), py_str("cinnamon")]), py_str("Ground spices fade in about 6 months and whole ones last 2 to 3 years: small airtight jars, away from the hob and the light. Sniff a spoonful - if it smells of nothing it will taste of nothing, and toasting what is left brings it back a little.")]), py_tuple(vec![py_tuple(vec![py_str("oil"), py_str("olive oil"), py_str("cooking oil"), py_str("vegetable oil"), py_str("sunflower oil"), py_str("rapeseed oil")]), py_str("Oil keeps about a year once opened, often past the date on the bottle if it stays cool and dark: the cupboard rather than beside the hob, with the cap on. Rancid oil smells of crayons or old nuts and tastes of them too, so bin it.")]), py_tuple(vec![py_tuple(vec![py_str("fruit"), py_str("fruits")]), py_str("Most fruit keeps 3 to 7 days in the salad drawer, and it is worth knowing the spread: berries and ripe stone fruit a day or two, apples, oranges and pears a week or more, and anything already cut 3 days in a sealed tub. Bananas, whole melons and tomatoes keep their flavour better out of the fridge.")]), py_tuple(vec![py_tuple(vec![py_str("celery"), py_str("celery sticks"), py_str("head of celery"), py_str("stick of celery")]), py_str("A head of celery keeps 2 weeks wrapped in foil in the fridge, which keeps it crisp where a plastic bag makes it slimy. Limp sticks come back in 20 minutes in iced water, so shave the dry bottom off first.")]), py_tuple(vec![py_tuple(vec![py_str("lettuce"), py_str("lettuces"), py_str("iceberg"), py_str("little gem"), py_str("romaine")]), py_str("Lettuce keeps about a week in the fridge: wash and dry the leaves, wrap them in kitchen paper inside a bag and dress them only at the table, because wet leaves and early dressing are what wilt a salad. Keep the bag in the salad drawer rather than at the back, where it freezes; limp leaves come back in iced water for 10 minutes.")]), py_tuple(vec![py_tuple(vec![py_str("cucumber"), py_str("cucumbers")]), py_str("Cucumber keeps about a week in the fridge, wrapped in a tea towel or a paper bag in the salad drawer: cling film traps the water and that is what makes it slimy. A soft one is still fine cooked, and the cut end dries first, so shave that off.")]), py_tuple(vec![py_tuple(vec![py_str("nuts"), py_str("walnuts"), py_str("almonds"), py_str("cashews"), py_str("peanuts"), py_str("pine nuts"), py_str("pecans")]), py_str("Nuts keep 3 to 6 months in an airtight jar in a cool cupboard and a year in the freezer; shelled or chopped ones go rancid sooner, and walnuts and pine nuts fastest. Taste one: a bitter or soapy taste means the oils have turned, and toasting hides it but does not fix it.")]), py_tuple(vec![py_tuple(vec![py_str("chocolate"), py_str("dark chocolate"), py_str("milk chocolate"), py_str("chocolate bar")]), py_str("Chocolate keeps about a year in a cool, dry cupboard, not the fridge, where condensation brings out a white bloom. The bloom is sugar or fat rather than mould, so it is still fine to eat; a bar that has been warm goes grey and grainy and is only good for melting.")]), py_tuple(vec![py_tuple(vec![py_str("bacon"), py_str("bacon rashers"), py_str("streaky bacon"), py_str("smoked bacon")]), py_str("Bacon keeps about a week in the fridge once the pack is open, sealed in a bag, and 3 months in the freezer, rashers separated with paper so you can take one at a time. It goes slimy and smells sharp when it is off; the cure keeps it pink long after that.")]), py_tuple(vec![py_tuple(vec![py_str("pasta"), py_str("dried pasta"), py_str("spaghetti"), py_str("penne"), py_str("macaroni"), py_str("fusilli")]), py_str("Dried pasta keeps about 2 years past the date on the packet in a sealed jar in a cupboard, and longer if it stays dry. Cooked pasta keeps 3 days in the fridge, tossed with a little oil so it does not clump, and reheats once in the sauce.")]), py_tuple(vec![py_tuple(vec![py_str("oats"), py_str("oat"), py_str("porridge oats"), py_str("rolled oats"), py_str("oat groats"), py_str("jumbo oats"), py_str("oatmeal"), py_str("oat bran"), py_str("porridge")]), py_func("_oats_kept", |args: &[Py]| _oats_kept(args))]), py_tuple(vec![py_tuple(vec![py_str("overnight oats"), py_str("soaked oats"), py_str("bircher"), py_str("bircher muesli"), py_str("overnight porridge")]), py_str("Overnight oats keep 3 days in the fridge in a lidded jar, and they are better after the first night than they are after an hour: 40g of oats to about 100ml of milk or yoghurt, left covered overnight, keeps in the fridge for three breakfasts.")]), py_tuple(vec![py_tuple(vec![py_str("porridge"), py_str("cooked porridge"), py_str("oatmeal"), py_str("ready brek")]), py_str("Cooked porridge keeps 2 days in a lidded tub in the fridge: cool it quickly, then reheat it with a splash of milk or water and a stir, because cold porridge sets solid. Porridge left out on the side overnight goes in the bin.")]), py_tuple(vec![py_tuple(vec![py_str("cereal"), py_str("breakfast cereal"), py_str("cornflakes"), py_str("corn flakes"), py_str("muesli"), py_str("granola"), py_str("rice krispies"), py_str("weetabix")]), py_str("An opened box keeps 2 to 3 months in an airtight container and goes soft before it goes off, so taste rather than sniff it. The date on the box is for the sealed bag inside; stale cereal comes back crisp in a 150C oven for 5 minutes, spread out on a tray.")]), py_tuple(vec![py_tuple(vec![py_str("dried fruit"), py_str("sultanas"), py_str("raisins"), py_str("dried apricots"), py_str("dates"), py_str("prunes"), py_str("cranberries")]), py_str("Dried fruit keeps 6 months in a sealed jar and a year in the fridge, and past that it hardens rather than spoils. Bring it back in 10 minutes in a bowl of hot water, or leave a slice of bread in the jar overnight. White crystals on the outside are sugar, not mould.")]), py_tuple(vec![py_tuple(vec![py_str("guacamole"), py_str("guac"), py_str("avocado dip"), py_str("mashed avocado")]), py_str("Guacamole keeps 1 to 2 days in the fridge in a tub with cling film pressed right onto the surface, and the lime in it is what slows the browning. Brown on top is oxygen, not rot: stir it in and it is fine. After two days it tastes flat and watery, so eat it first.")]), py_tuple(vec![py_tuple(vec![py_str("mashed potato"), py_str("mashed potatoes"), py_str("mash"), py_str("leftover mash")]), py_str("Mashed potato keeps 2 days in the fridge, less than the potatoes it came from, because the butter and milk in it turn first. Warm it in a pan with a splash of milk over a low heat and stir, rather than in the microwave on high, which makes it gluey. It freezes for a month and comes back a little looser, so beat a knob of butter through it.")]), py_tuple(vec![py_tuple(vec![py_str("batter"), py_str("pancake batter"), py_str("crepe batter"), py_str("yorkshire pudding batter"), py_str("waffle batter")]), py_str("Batter keeps 1 to 2 days in the fridge in a jug with a lid, and stir it before it goes in the pan because the flour sinks. One with baking powder in it loses its lift after a day, so use it the same day for pancakes. Crepe batter is better after half an hour's rest, and a batter that smells sour or has grey on top goes down the sink.")]), py_tuple(vec![py_tuple(vec![py_str("quiche"), py_str("quiches"), py_str("a quiche"), py_str("leftover quiche"), py_str("savoury tart")]), py_str("Quiche keeps 3 days in the fridge, covered, and eat it cold or reheat it to piping hot all the way through rather than warm. A quiche freezes for 2 months, cut in slices and wrapped: thaw it in the fridge overnight and warm it through. Once the egg custard smells sour or the top has a sheen of water, it is done.")]), py_tuple(vec![py_tuple(vec![py_str("sausage rolls"), py_str("sausage roll"), py_str("leftover sausage rolls")]), py_str("Sausage rolls keep 2 days in the fridge, in a tub or a paper bag rather than cling film, or they go soft. Reheat them at 180C for 8 to 10 minutes to get the pastry crisp again, and eat them hot rather than warm. They freeze well for 2 months unbaked or baked, and the pastry is what goes first: a soggy bottom means it was wrapped while it was still warm.")]), py_tuple(vec![py_tuple(vec![py_str("smoked salmon"), py_str("smoked salmon slices"), py_str("lox")]), py_str("Smoked salmon keeps 3 days in the fridge once the pack is opened, wrapped tight, and it is cured rather than cooked so the date on the pack matters more than the days. It freezes for a month in portions, though the texture goes softer. Eat it within 24 hours once it is out on a plate, and bin it if it smells sharp or feels tacky.")]), py_tuple(vec![py_tuple(vec![py_str("coconut milk"), py_str("tinned coconut milk"), py_str("tin of coconut milk"), py_str("coconut cream")]), py_str("An opened tin of coconut milk keeps 4 to 5 days in the fridge in a tub or a jug, and the thick cream separates to the top, so stir it back in. An unopened tin keeps a year in the cupboard. Freeze what is left in an ice-cube tray and drop the cubes into a curry.")]), py_tuple(vec![py_tuple(vec![py_str("lemon curd"), py_str("curd"), py_str("homemade lemon curd")]), py_str("Lemon curd keeps 2 weeks in a jar in the fridge once it is opened, and homemade curd the same, though eat homemade within a week if you did not sterilise the jar. It freezes for 3 months and thaws in the fridge. Curd that has separated, or smells of nail varnish, goes in the bin - the egg in it is what spoils.")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ROWS"); }
    s_ROWS(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("soft"), py_str("gone soft"), py_str("bendy"), py_str("mouldy"), py_str("mould"), py_str("moldy"), py_str("dried out"), py_str("dry out"), py_str("wilt"), py_str("wilted"), py_str("limp"), py_str("rot"), py_str("rotten"), py_str("green"), py_str("greening"), py_str("sprout"), py_str("sprouts"), py_str("sprouting")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.PROBLEM_ONLY"); }
    s_PROBLEM_ONLY(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("how long"), py_str("how long does"), py_str("how long do"), py_str("how much longer"), py_str("keep for"), py_str("still ok"), py_str("still good"), py_str("ok to use"), py_str("ok to eat"), py_str("out of date"), py_str("use by"), py_str("past its date"), py_str("past its best"), py_str("the day after"), py_str("day after"), py_str("day later"), py_str("a day later"), py_str("the following day"), py_str("next day"), py_str("the next day"), py_str("day after that"), py_str("in the fridge"), py_str("out of the fridge")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.LONG_WORDS"); }
    s_LONG_WORDS(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.KEEPING_KEY"); }
    s_KEEPING_KEY(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&op_add(&g_KEEP_WORDS(), &g_LONG_WORDS())) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_PROBLEM_ONLY(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data5() -> Py { py_tuple(vec![py_str("cook"), py_str("cooking"), py_str("cooked through"), py_str("boil"), py_str("boiling"), py_str("minute"), py_str("minutes"), py_str("al dente"), py_str("recipe"), py_str("recipes"), py_str("take"), py_str("takes"), py_str("taking"), py_str("simmer"), py_str("simmering"), py_str("simmered"), py_str("fry"), py_str("fries"), py_str("frying"), py_str("fried"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("grill"), py_str("grilling"), py_str("grilled"), py_str("steam"), py_str("steaming"), py_str("steamed"), py_str("saute"), py_str("sauteed"), py_str("stir fry"), py_str("stir frying"), py_str("air fryer"), py_str("hob"), py_str("sear"), py_str("searing"), py_str("caramelise"), py_str("caramelize"), py_str("reduce it"), py_str("reduce the")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COOK_LINE_WORDS"); }
    s_COOK_LINE_WORDS(__data5());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FOLLOW_UP_WORDS"); }
    s_FOLLOW_UP_WORDS(py_tuple(vec![py_str("it take"), py_str("it takes"), py_str("does it take"), py_str("that take"), py_str("will it take"), py_str("long does it take"), py_str("long will it take"), py_str("how long does that take")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FLOP_WORDS"); }
    s_FLOP_WORDS(py_tuple(vec![py_str("limp"), py_str("limpy"), py_str("wilted"), py_str("wilting"), py_str("soggy"), py_str("floppy"), py_str("gone soft"), py_str("sad")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._MUSH_VERDICT"); }
    s__MUSH_VERDICT(py_str("A little slimy is still fine - it is water, not rot: wipe them and cook them soon. A sour smell, brown patches or a sticky feel that stays after wiping means bin them, because that is decay and no amount of cooking puts it right."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._MUSH_FIX"); }
    s__MUSH_FIX(py_str("A slimy mushroom is a wet one: they are about 90% water and a closed plastic bag traps the sweat that makes the surface slippery. Tip them into a paper bag in the fridge, wipe rather than wash them, and cook them soon in a hot dry pan."));
    fn __data6() -> Py { py_tuple(vec![py_str("eat"), py_str("eaten"), py_str("ok"), py_str("okay"), py_str("safe"), py_str("still"), py_str("bin"), py_str("throw"), py_str("throwing"), py_str("use"), py_str("useable"), py_str("usable"), py_str("edible"), py_str("bad"), py_str("off"), py_str("risky"), py_str("cook with")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._EATING_WORDS"); }
    s__EATING_WORDS(__data6());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FIX_FIRST"); }
    s_FIX_FIRST(py_dict(vec![(py_str("celery"), py_str("Limp celery comes back in 20 minutes in iced water, so shave the dry bottom off first: a head keeps 2 weeks wrapped in foil in the fridge, which keeps it crisp where a plastic bag turns it slimy.")), (py_str("lettuce"), py_str("Plunge limp lettuce into iced water for 10 minutes and it comes back crisp; dry the leaves, wrap them in kitchen paper inside a bag in the salad drawer and they keep about a week. Dress them at the table, since wet leaves and early dressing are what wilted them in the first place."))]));
    fn __data7() -> Py { py_tuple(vec![py_str("inhaler"), py_str("inhalers"), py_str("tablet"), py_str("tablets"), py_str("pill"), py_str("pills"), py_str("medicine"), py_str("medicines"), py_str("medication"), py_str("medications"), py_str("prescription"), py_str("prescriptions"), py_str("insulin"), py_str("epipen"), py_str("epi pen"), py_str("epipens"), py_str("eye drops"), py_str("eyedrops"), py_str("antibiotic"), py_str("antibiotics"), py_str("paracetamol"), py_str("ibuprofen"), py_str("painkiller"), py_str("painkillers"), py_str("calpol"), py_str("nurofen"), py_str("cough syrup"), py_str("antihistamine"), py_str("antihistamines"), py_str("steroid"), py_str("steroids"), py_str("vaccine"), py_str("jab"), py_str("wound dressing"), py_str("plasters"), py_str("bandage"), py_str("ointment"), py_str("eye ointment"), py_str("contact lens"), py_str("contact lenses"), py_str("nose drops"), py_str("ear drops"), py_str("vitamin"), py_str("vitamins"), py_str("supplement"), py_str("supplements"), py_str("test strip"), py_str("test strips")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.KEEP_NOT_MEDICINE"); }
    s_KEEP_NOT_MEDICINE(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("before bed"), py_str("before sleep"), py_str("before bedtime"), py_str("keep me awake"), py_str("keeps me awake"), py_str("wake me up"), py_str("keeps me up"), py_str("last coffee"), py_str("last cup"), py_str("stop drinking"), py_str("stop caffeine"), py_str("sleep better"), py_str("cant sleep"), py_str("can't sleep"), py_str("to sleep"), py_str("fall asleep")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.KEEP_NOT_SLEEP"); }
    s_KEEP_NOT_SLEEP(__data8());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.DISH_WORDS"); }
    s_DISH_WORDS(py_tuple(vec![py_str("soup"), py_str("soups"), py_str("stew"), py_str("stews"), py_str("broth"), py_str("broths"), py_str("chowder"), py_str("minestrone"), py_str("casserole"), py_str("casseroles"), py_str("tagine"), py_str("goulash")]));
    fn __data9() -> Py { py_tuple(vec![py_str("slug"), py_str("slugs"), py_str("snail"), py_str("snails"), py_str("caterpillar"), py_str("caterpillars"), py_str("aphid"), py_str("aphids"), py_str("greenfly"), py_str("blackfly"), py_str("whitefly"), py_str("pests"), py_str("pest"), py_str("eaten"), py_str("eating my"), py_str("eating the"), py_str("ate my"), py_str("holes in"), py_str("nibbled"), py_str("infestation"), py_str("allotment"), py_str("greenhouse"), py_str("raised bed"), py_str("vegetable patch"), py_str("veg patch"), py_str("in the garden"), py_str("my garden"), py_str("growing"), py_str("grow"), py_str("sowed"), py_str("sow"), py_str("planted"), py_str("seedling"), py_str("seedlings"), py_str("crop"), py_str("harvest")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.GROWING_WORDS"); }
    s_GROWING_WORDS(__data9());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FEEDING_WORDS"); }
    s_FEEDING_WORDS(py_tuple(vec![py_str("breastfeeding"), py_str("breast feeding"), py_str("breast-feeding"), py_str("nursing"), py_str("lactating"), py_str("chestfeeding"), py_str("while im feeding")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._STAND_DOWN"); }
    s__STAND_DOWN(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[g_COOKED_WORDS()]).unwrap_or_else(|e| exc::raise(e)), &g_USE_UP_WORDS()), &g_COOKING_WORDS()), &g_COOK_LINE_WORDS()), &g_KEEP_NOT_KEEPING()), &g_FOLLOW_UP_WORDS()), &g_KEEP_NOT_SLEEP()), &g_KEEP_NOT_MEDICINE()), &g_KEEP_QUALITY_WORDS()), &g_GROWING_WORDS()), &g_DISH_WORDS()), &g_FEEDING_WORDS()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.CALENDAR_DATE"); }
    s_CALENDAR_DATE(op_add(&g_PACK_DATE(), &py_tuple(vec![py_str("the date"), py_str("its date"), py_str("their date"), py_str("by date"), py_str("dates on"), py_str("date said")])));
    fn __data10() -> Py { py_tuple(vec![py_str("first date"), py_str("second date"), py_str("third date"), py_str("blind date"), py_str("date night"), py_str("on a date"), py_str("out on a date"), py_str("a date with"), py_str("date tomorrow"), py_str("the date went"), py_str("after a date"), py_str("calling after"), py_str("before texting"), py_str("after texting"), py_str("texting"), py_str("text her"), py_str("text him"), py_str("text them"), py_str("asking her out"), py_str("asking him out")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.DATING_DATE"); }
    s_DATING_DATE(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("cake"), py_str("cakes"), py_str("juice"), py_str("smoothie"), py_str("dried"), py_str("tinned"), py_str("tin"), py_str("can"), py_str("jar"), py_str("jam"), py_str("wine"), py_str("crumble"), py_str("tart"), py_str("salad"), py_str("chutney"), py_str("yoghurt"), py_str("yogurt"), py_str("grapefruit"), py_str("pie"), py_str("pies"), py_str("pastry"), py_str("kiwi"), py_str("flies"), py_str("fly"), py_str("bowl"), py_str("tree"), py_str("trees"), py_str("grow"), py_str("growing"), py_str("plant"), py_str("plants"), py_str("pick"), py_str("picking"), py_str("ripen"), py_str("ripening"), py_str("ripe"), py_str("buying"), py_str("choose"), py_str("choosing"), py_str("healthy"), py_str("calories"), py_str("sugar"), py_str("portion"), py_str("portions")]) }
    fn __data12() -> Py { py_tuple(vec![py_str("fridge"), py_str("refrigerator"), py_str("fridge freezer"), py_str("freezer"), py_str("frozen"), py_str("cold"), py_str("in the fridge"), py_str("left in the fridge"), py_str("stick"), py_str("sticks"), py_str("sticking"), py_str("stuck"), py_str("clump"), py_str("clumps"), py_str("clumping"), py_str("for tomorrow"), py_str("until tomorrow"), py_str("for the next day"), py_str("next day"), py_str("overnight"), py_str("ok tomorrow"), py_str("okay tomorrow"), py_str("still ok tomorrow"), py_str("tomorrow ok"), py_str("ok overnight"), py_str("okay overnight"), py_str("still ok overnight"), py_str("ok the next day"), py_str("okay the next day"), py_str("the day after"), py_str("day after"), py_str("a day after"), py_str("day later"), py_str("a day later"), py_str("the following day"), py_str("following day"), py_str("yesterday"), py_str("last night"), py_str("sauce"), py_str("pasta sauce"), py_str("tomato sauce"), py_str("marinara"), py_str("passata"), py_str("bolognese"), py_str("macaroni cheese"), py_str("mac and cheese"), py_str("mac n cheese"), py_str("cauliflower cheese"), py_str("cheese sauce"), py_str("pasta bake"), py_str("lasagne"), py_str("cannelloni")]) }
    fn __data13() -> Py { py_tuple(vec![py_str("to soak"), py_str("soak oats"), py_str("soaking oats"), py_str("soak them"), py_str("soaking oats"), py_str("oat milk"), py_str("oatmilk"), py_str("oat drink"), py_str("oat cakes"), py_str("oatcake"), py_str("oatcakes"), py_str("oat biscuits"), py_str("oat biscuit"), py_str("oat bars"), py_str("oat bar"), py_str("oat crackers"), py_str("flapjacks"), py_str("granola"), py_str("muesli"), py_str("oatly"), py_str("porridge pot"), py_str("porridge pots")]) }
    fn __data14() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("blend"), py_str("blitz"), py_str("healthy"), py_str("healthier"), py_str("calories"), py_str("protein"), py_str("good for you"), py_str("brown"), py_str("browns"), py_str("browning"), py_str("going brown"), py_str("stop it going brown"), py_str("thick"), py_str("thins"), py_str("watery"), py_str("lumpy"), py_str("dance"), py_str("dancing"), py_str("class"), py_str("classes"), py_str("swim"), py_str("swimming"), py_str("dip in"), py_str("dipped"), py_str("dipping"), py_str("what to dip"), py_str("with bread"), py_str("with carrots"), py_str("with crisps"), py_str("starter"), py_str("party")]) }
    fn __data15() -> Py { py_tuple(vec![py_str("oil change"), py_str("oil changes"), py_str("change the oil"), py_str("change my oil"), py_str("changing the oil"), py_str("between oil"), py_str("engine"), py_str("car"), py_str("cars"), py_str("van"), py_str("dipstick"), py_str("mot"), py_str("mileage"), py_str("garage"), py_str("mechanic"), py_str("gearbox")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ROW_STAND_DOWN"); }
    s_ROW_STAND_DOWN(py_dict(vec![(py_str("smoked salmon"), py_tuple(vec![py_str("cooked"), py_str("bake"), py_str("baked"), py_str("roast"), py_str("roasted"), py_str("fillets")])), (py_str("fruit"), __data11()), (py_str("pasta"), __data12()), (py_str("oats"), __data13()), (py_str("overnight oats"), py_tuple(vec![py_str("to soak"), py_str("how long to soak"), py_str("soaking time")])), (py_str("dried fruit"), op_add(&g_CALENDAR_DATE(), &g_DATING_DATE())), (py_str("houmous"), __data14()), (py_str("porridge"), py_tuple(vec![py_str("microwave"), py_str("microwaved"), py_str("simmer"), py_str("stir"), py_str("make"), py_str("how long to cook"), py_str("how long do i cook")])), (py_str("oil"), __data15())]));
    fn __data16() -> Py { py_tuple(vec![py_str("how do i cook"), py_str("how do you cook"), py_str("how long to cook"), py_str("how long do i cook"), py_str("how long should i cook"), py_str("cooking time"), py_str("cook for"), py_str("cook it"), py_str("boil"), py_str("boiling"), py_str("simmer"), py_str("simmering"), py_str("stirring"), py_str("microwave"), py_str("minute"), py_str("minutes"), py_str("take"), py_str("takes"), py_str("taking"), py_str("recipe"), py_str("recipes"), py_str("porridge oats to")]) }
    fn __data17() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how to make"), py_str("lumpy"), py_str("gluey"), py_str("butter"), py_str("cream in"), py_str("beat it"), py_str("reheat"), py_str("reheats"), py_str("reheating"), py_str("reheated")]) }
    fn __data18() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how to make"), py_str("bake"), py_str("baking"), py_str("oven"), py_str("pastry"), py_str("line the tin"), py_str("blind bake"), py_str("custard")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ROW_COOK_FORMS"); }
    s_ROW_COOK_FORMS(py_dict(vec![(py_str("oats"), __data16()), (py_str("guacamole"), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("blend"), py_str("blitz"), py_str("mash it"), py_str("dance"), py_str("dancing"), py_str("dip for"), py_str("what to dip")])), (py_str("mashed potato"), __data17()), (py_str("batter"), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how to make"), py_str("thin"), py_str("thick"), py_str("lumpy"), py_str("mix"), py_str("whisk"), py_str("rest")])), (py_str("lemon curd"), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("how to make"), py_str("set"), py_str("setting"), py_str("runny"), py_str("split"), py_str("why is my"), py_str("why did my")])), (py_str("quiche"), __data18()), (py_str("sausage rolls"), py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how to make"), py_str("bake"), py_str("baking"), py_str("oven"), py_str("pastry"), py_str("puff"), py_str("rough puff")])), (py_str("coconut milk"), py_tuple(vec![py_str("make"), py_str("making"), py_str("open"), py_str("opening"), py_str("shake"), py_str("separate"), py_str("separates"), py_str("curdle"), py_str("curdles"), py_str("thick"), py_str("thin")]))]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COOKED_IN_THE_ROW"); }
    s_COOKED_IN_THE_ROW(call_builtin("tuple", &[g_ROW_COOK_FORMS()]).unwrap_or_else(|e| exc::raise(e)));
    fn __data19() -> Py { py_tuple(vec![py_str("cooked vegetables"), py_str("cooked veg"), py_str("leftover vegetables"), py_str("leftover veg"), py_str("left over vegetables"), py_str("left over veg"), py_str("roast vegetables"), py_str("roast veg"), py_str("roasted vegetables"), py_str("roasted veg"), py_str("roast veggies"), py_str("cooked mushrooms"), py_str("fried mushrooms"), py_str("cooked broccoli"), py_str("cooked cauliflower"), py_str("cooked carrots"), py_str("cooked greens"), py_str("cooked cabbage"), py_str("cooked courgette"), py_str("cooked courgettes"), py_str("cooked peppers"), py_str("stir fried vegetables"), py_str("stir fried veg")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COOKED_VEG_NAMES"); }
    s_COOKED_VEG_NAMES(__data19());
    fn __data20() -> Py { py_tuple(vec![py_str("how do i cook"), py_str("how do you cook"), py_str("how long to cook"), py_str("how long do i cook"), py_str("how long should i cook"), py_str("cooking time"), py_str("how do i roast"), py_str("how long do i roast"), py_str("how long in the oven"), py_str("oven temperature"), py_str("what temperature"), py_str("how do i make"), py_str("how do i fry"), py_str("how long to roast"), py_str("how long to boil"), py_str("boil them"), py_str("boil it"), py_str("recipe"), py_str("recipes")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COOKED_VEG_COOK_WORDS"); }
    s_COOKED_VEG_COOK_WORDS(__data20());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COOKED_VEG_REPLY"); }
    s_COOKED_VEG_REPLY(py_str("Cooked vegetables keep 3 days in the fridge in a covered box, and roast veg 3 to 4 days, because the oven has already taken the water out of them. Get them in there within 2 hours of coming off the heat, in a box rather than on a plate, and reheat once, piping hot. Cooked mushrooms and courgette go first, in 2 days, and anything slimy or sour goes in the bin, because the bugs that make you ill have no smell."));
    fn __data21() -> Py { py_tuple(vec![py_str("keep warm"), py_str("keeps warm"), py_str("keeping warm"), py_str("keep it warm"), py_str("keep them warm"), py_str("keep it hot"), py_str("keep them hot"), py_str("from sticking"), py_str("from going soggy"), py_str("from drying"), py_str("drying out"), py_str("does not stick"), py_str("doesnt stick"), py_str("moist"), py_str("juicy"), py_str("tender"), py_str("crisp"), py_str("crispy"), py_str("warm"), py_str("hot"), py_str("sticking")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.KEEP_TECHNIQUE"); }
    s_KEEP_TECHNIQUE(__data21());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COOKED_VEG_STAND_DOWN"); }
    s_COOKED_VEG_STAND_DOWN(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STAND_DOWN()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_COOKED_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_COOK_LINE_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_COOKING_WORDS(), &word.clone())) } } } }) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._DAY_AFTER"); }
    s__DAY_AFTER(py_tuple(vec![py_str("the day after"), py_str("day after"), py_str("a day after"), py_str("day later"), py_str("a day later"), py_str("the following day"), py_str("following day"), py_str("the day later"), py_str("two days later"), py_str("2 days later"), py_str("the morning after")]));
    fn __data22() -> Py { py_tuple(vec![py_str("chicken"), py_str("rice"), py_str("prawn"), py_str("prawns"), py_str("ham"), py_str("beef"), py_str("pork"), py_str("lamb"), py_str("fish"), py_str("salmon"), py_str("cod"), py_str("turkey"), py_str("mince"), py_str("curry"), py_str("stew"), py_str("soup"), py_str("bolognese"), py_str("casserole"), py_str("noodles"), py_str("couscous"), py_str("quiche"), py_str("pizza"), py_str("sausage"), py_str("sausages"), py_str("egg"), py_str("eggs"), py_str("beans"), py_str("lentils"), py_str("tofu"), py_str("shepherds pie"), py_str("cottage pie"), py_str("lasagne"), py_str("leftovers"), py_str("leftover")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._DAY_AFTER_FOODS"); }
    s__DAY_AFTER_FOODS(__data22());
    fn __data23() -> Py { py_tuple(vec![py_str("workout"), py_str("work out"), py_str("training"), py_str("gym"), py_str("run"), py_str("running"), py_str("exercise"), py_str("race"), py_str("match"), py_str("surgery"), py_str("operation"), py_str("dentist"), py_str("hospital"), py_str("vaccine"), py_str("jab"), py_str("drinking"), py_str("drink"), py_str("hangover"), py_str("night out"), py_str("wedding"), py_str("funeral"), py_str("flight"), py_str("interview"), py_str("exam"), py_str("period"), py_str("pill"), py_str("morning after pill"), py_str("birth"), py_str("death"), py_str("died"), py_str("died"), py_str("holiday"), py_str("moving"), py_str("exam"), py_str("results")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._DAY_AFTER_AVOID"); }
    s__DAY_AFTER_AVOID(__data23());
    fn __data24() -> Py { py_tuple(vec![py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("freezer"), py_str("freezers"), py_str("frozen"), py_str("in the freezer"), py_str("in a freezer"), py_str("in my freezer"), py_str("put in the freezer"), py_str("put it in the freezer"), py_str("goes in the freezer"), py_str("go in the freezer")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FREEZE_WORDS"); }
    s_FREEZE_WORDS(__data24());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FREEZE_SWEETS"); }
    s_FREEZE_SWEETS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("cheesecake"), py_str("cheesecakes"), py_str("baked cheesecake"), py_str("new york cheesecake"), py_str("no bake cheesecake")]), py_str("Yes, and a cheesecake freezes better than almost any other pudding: slice it first and wrap each slice, or double-wrap the whole tin, and it keeps 1 to 2 months. Thaw the slices in the fridge overnight rather than on the side, because a warm thaw makes the cheese weep.")]), py_tuple(vec![py_tuple(vec![py_str("birthday cake"), py_str("sponge cake"), py_str("sponge"), py_str("victoria sponge"), py_str("victoria sandwich"), py_str("madeira cake"), py_str("layer cake"), py_str("cake"), py_str("cakes")]), py_str("Yes: cool it completely, leave the icing off, wrap the sponge in two layers and it keeps 2 to 3 months. Thaw it still wrapped in the fridge overnight, then ice it, because a cake iced before it goes in comes out with the icing stuck to the paper.")]), py_tuple(vec![py_tuple(vec![py_str("trifle"), py_str("trifles")]), py_str("No, and this is the pudding to eat fresh: jelly weeps, cream splits and the sponge goes soggy as it thaws, so what comes out is watery at the bottom and grainy on top. Make it the day before at most and keep it in the fridge.")]), py_tuple(vec![py_tuple(vec![py_str("panna cotta"), py_str("panna cottas")]), py_str("No: the gelatine weeps as it thaws and the cream sets grainy, so a frozen one never comes back to that just-set wobble. Make it the day before and keep it in the fridge, where it holds 2 to 3 days.")]), py_tuple(vec![py_tuple(vec![py_str("tiramisu"), py_str("tiramisus")]), py_str("It freezes, though it is better fresh: cover the dish and it keeps up to 2 months, then thaw it in the fridge overnight. Expect the sponge to come back a little wetter and dust the cocoa on after it thaws, not before.")]), py_tuple(vec![py_tuple(vec![py_str("custard"), py_str("home made custard"), py_str("custard sauce")]), py_str("Yes, in portions for up to 3 months, and cornflour in it is what makes it survive: a plain egg custard can look watery or grainy once thawed. Thaw it in the fridge and then warm it gently in a pan, whisking, rather than in the microwave, which splits it.")]), py_tuple(vec![py_tuple(vec![py_str("chocolate mousse"), py_str("mousse"), py_str("mousses"), py_str("lemon mousse"), py_str("chocolate pot"), py_str("chocolate pots")]), py_str("Yes, up to 2 months: freeze it in the pots or in a tub with the surface pressed flat, and thaw it in the fridge for a few hours. A mousse weeps a little as it thaws, so stir it through before you serve it and nobody can tell.")]), py_tuple(vec![py_tuple(vec![py_str("milk"), py_str("a pint of milk"), py_str("semi skimmed milk"), py_str("skimmed milk"), py_str("whole milk"), py_str("breast milk"), py_str("formula"), py_str("goats milk")]), py_str("Yes, and it keeps about a month in the freezer: pour off a little first, because milk expands and splits the bottle. Thaw it in the fridge and shake it well, since the fat and water separate; it is fine for cooking and on cereal, a bit thinner to drink.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FREEZE_SWEET_NAMES"); }
    s_FREEZE_SWEET_NAMES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FREEZE_SWEETS()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk1[0].clone(); let mut _u: Py = __unpk1[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data25() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how to make"), py_str("homemade"), py_str("from scratch"), py_str("how do i cook"), py_str("how long to bake"), py_str("cooking time"), py_str("healthy"), py_str("unhealthy"), py_str("fattening"), py_str("calories"), py_str("good for you"), py_str("pregnant"), py_str("pregnancy"), py_str("breastfeeding"), py_str("nursing"), py_str("weaning"), py_str("allergy"), py_str("allergic"), py_str("lactose"), py_str("how much milk"), py_str("ounces"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("use by"), py_str("out of date"), py_str("past the date"), py_str("still ok"), py_str("still good"), py_str("defrost"), py_str("thaw"), py_str("thawed"), py_str("thawing"), py_str("refreeze"), py_str("ice cream"), py_str("ice lolly"), py_str("ice lollies"), py_str("power cut"), py_str("broken"), py_str("stopped working"), py_str("left out"), py_str("out of the freezer")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.FREEZE_SWEET_AVOID"); }
    s_FREEZE_SWEET_AVOID(__data25());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._DAY_AFTER_ENTRY"); }
    s__DAY_AFTER_ENTRY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-the-day-after")); __args.push(py_tuple(vec![g__DAY_AFTER_FOODS(), g__DAY_AFTER()])); __args.push(py_str("Cooled within two hours and put straight in the fridge, cooked chicken, meat, fish and pasta keep two days and cooked rice one, and that is the whole rule - the date on the pack does not stretch it. After that it is a reheat question rather than a keep one: reheat it once, all the way through, until it is steaming hot, and never reheat rice twice. Bin anything that smells, feels slimy or was left out overnight.")); __kws.extend(vec![("avoid".to_string(), g__DAY_AFTER_AVOID()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.PASTA_SAUCE_WORDS"); }
    s_PASTA_SAUCE_WORDS(py_tuple(vec![py_str("pasta sauce"), py_str("tomato sauce"), py_str("sauce for pasta"), py_str("marinara"), py_str("passata"), py_str("bolognese sauce"), py_str("pasta sauce jar"), py_str("jar of pasta sauce"), py_str("chilli sauce"), py_str("tomato and basil sauce"), py_str("arrabbiata")]));
    fn __data26() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("store"), py_str("stores"), py_str("storing"), py_str("stored"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("how long"), py_str("still ok"), py_str("still good"), py_str("gone off"), py_str("go off"), py_str("off yet"), py_str("next day"), py_str("the day after"), py_str("tomorrow"), py_str("open jar"), py_str("opened"), py_str("once opened"), py_str("leftover"), py_str("left over"), py_str("batch"), py_str("in the fridge"), py_str("out of date"), py_str("use by"), py_str("freeze"), py_str("freezes"), py_str("freezing"), py_str("freezer"), py_str("freezers"), py_str("frozen")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.SAUCE_KEEPING_WORDS"); }
    s_SAUCE_KEEPING_WORDS(__data26());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.DATES_OWN"); }
    s_DATES_OWN({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-dates-own")); __args.push(py_tuple(vec![py_tuple(vec![py_str("eat dates"), py_str("dates past"), py_str("dates after"), py_str("dates last"), py_str("store dates"), py_str("dates keep"), py_str("dates go off"), py_str("do dates go off"), py_str("how long do dates")]), py_tuple(vec![py_str("past the date"), py_str("past its date"), py_str("past their date"), py_str("after the date"), py_str("out of date"), py_str("use them up")])])); __args.push(py_str("Dates keep 6 months in a sealed jar and a year in the fridge, and past the date on the pack they harden rather than spoil: the soft medjool in a tub go first, and the dry baking ones last longest. Ten minutes in a bowl of hot water brings a hard one back. White crystals on the outside are sugar, not mould.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("use by"), py_str("best before"), py_str("sell by"), py_str("expiry"), py_str("expires"), py_str("expired"), py_str("date on")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data27() -> Py { py_tuple(vec![py_str("go off"), py_str("goes off"), py_str("gone off"), py_str("off"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("sour"), py_str("mould"), py_str("mouldy"), py_str("mold"), py_str("moldy"), py_str("fizzy"), py_str("safe"), py_str("separated"), py_str("separation"), py_str("separating"), py_str("watery"), py_str("water on top"), py_str("curdled"), py_str("lumpy"), py_str("runny")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.YOGHURT"); }
    s_YOGHURT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-yoghurt")); __args.push(py_tuple(vec![py_tuple(vec![py_str("yoghurt"), py_str("yogurt"), py_str("greek yoghurt"), py_str("greek yogurt"), py_str("natural yoghurt"), py_str("natural yogurt"), py_str("plain yoghurt"), py_str("plain yogurt")]), g_KEEPING_KEY()])); __args.push(py_str("An unopened tub keeps about a week past the date on the lid, and once a spoon has been in it keep it 3 to 5 days - the date is for the sealed pot. Plain yoghurt that has separated with a little water on top is fine - stir it back in. Bin it if there is mould on the surface, or if it smells sour or fizzy rather than tangy.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_KEEPING_KEY()), ("avoid".to_string(), __data27())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) });
    fn __data28() -> Py { py_tuple(vec![py_str("past the date"), py_str("past its date"), py_str("past their date"), py_str("after the date"), py_str("out of date"), py_str("use by"), py_str("use by date"), py_str("best before"), py_str("ok to eat"), py_str("safe to eat"), py_str("eat it"), py_str("eat them"), py_str("still ok"), py_str("still good")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.DATE_ASK"); }
    s_DATE_ASK(__data28());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.DATE_ROWS"); }
    s_DATE_ROWS(py_list(vec![py_tuple(vec![py_tuple(vec![py_str("spinach"), py_str("baby spinach"), py_str("bag of spinach"), py_str("spinach leaves")]), py_str("A bag of spinach keeps about 5 days in the salad drawer and a whole unopened bag a week longer, and the date on the bag is the day to finish it by rather than a day it is still good after. Pick out anything slimy before it spreads, don't wash the leaves until you use them, and a bag that has only gone limp is fine wilted into a curry or an omelette.")]), py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("white rice"), py_str("basmati rice"), py_str("long grain rice"), py_str("brown rice")]), py_str("Dry rice keeps for years in a sealed jar in a cupboard, and the date on the bag is a best-before rather than a use-by. Cooked rice is the one to be strict about: cool it within an hour, box it, and keep it no more than 24 hours in the fridge - reheating does not kill the spores that grow in rice left warm.")]), py_tuple(vec![py_tuple(vec![py_str("guacamole"), py_str("taramasalata"), py_str("tarama"), py_str("tzatziki"), py_str("soured cream dip"), py_str("onion dip")]), py_str("Once opened, a tub of tzatziki or taramasalata keeps about 3 days in the fridge and guacamole a day or two, because avocado browns and sours fastest: the date on the lid is for the sealed pot, so judge it by look and smell after that. Press cling film down onto guacamole to keep the air off it, keep all of them cold, and bin anything that smells sour, fizzes or has mould."), py_tuple(vec![py_str("next day"), py_str("the next day")])]), py_tuple(vec![py_tuple(vec![py_str("hummus"), py_str("houmous"), py_str("humous")]), py_str("Hummus keeps about a week in the fridge in a sealed tub, and 3 to 4 days once a spoon has been in it: the date on the lid is for the sealed pot, so a couple of days past it is usually fine. A film of water on top is normal, so stir it in. Bin it if it smells sour or fizzy, or if there is mould."), py_tuple(vec![py_str("next day"), py_str("the next day")])])]));
    fn __data29() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("soy sauce"), py_str("light soy"), py_str("dark soy"), py_str("shoyu")]), py_str("Soy sauce keeps for years in the cupboard, and about a year once it is opened, because the salt stops anything growing in it. It darkens and a salt crust can form at the neck, and neither is spoilage; bin it only if it smells sour or has a film on top.")]), py_tuple(vec![py_tuple(vec![py_str("vinegar"), py_str("malt vinegar"), py_str("white vinegar"), py_str("balsamic"), py_str("cider vinegar"), py_str("wine vinegar")]), py_str("Vinegar does not go off: it keeps for years, opened or not, because it is already acid. A cloudy stringy blob in the bottle is the mother, which is harmless - strain it out and carry on - and a weaker taste after a couple of years is the only thing that changes.")]), py_tuple(vec![py_tuple(vec![py_str("tea"), py_str("tea bags"), py_str("teabags"), py_str("tea bag"), py_str("loose leaf"), py_str("green tea"), py_str("earl grey"), py_str("pg tips"), py_str("chamomile tea"), py_str("peppermint tea")]), py_str("Tea does not go off, it goes stale: about six months past the date in a sealed tin, and a year for loose leaf, after which the flavour fades rather than spoils. Keep it in a tin away from the light and the kettle's steam, and it is safe to drink long after it has lost its best.")]), py_tuple(vec![py_tuple(vec![py_str("peanut butter"), py_str("peanut butter jar"), py_str("nut butter"), py_str("almond butter")]), py_str("An opened jar of peanut butter keeps about three months in the cupboard, or six in the fridge if you want it to last. The oil that separates on top is normal - stir it back in rather than pouring it off - and the sign to bin it is a smell of paint or old oil, which is the nuts going rancid."), py_str("Peanut butter freezes well: a jar keeps about six months in the freezer and thaws overnight in the fridge, though it may need stirring again afterwards.")]), py_tuple(vec![py_tuple(vec![py_str("ham"), py_str("sliced ham"), py_str("cooked ham"), py_str("wafer thin ham"), py_str("honey roast ham")]), py_str("An opened pack of cooked ham keeps two to three days in the fridge, sealed in a box, and the use-by date on the pack only holds while it is unopened. It should feel dry and smell of nothing much; sliminess or a sour smell means bin it, and do not re-wrap it and hope."), py_str("Cooked ham freezes for about one to two months: wrap it in portions, because it goes watery and salty once thawed and will not slice thin again. Thaw it in the fridge overnight and use it in a cooked dish rather than a sandwich.")]), py_tuple(vec![py_tuple(vec![py_str("ketchup"), py_str("tomato ketchup"), py_str("brown sauce"), py_str("hp sauce")]), py_str("Opened ketchup keeps about a month in the cupboard and six months in the fridge, and the date on the bottle only counts while it is sealed. A dark colour or a little watery liquid is normal; a fizzy smell or mould at the neck means bin it.")]), py_tuple(vec![py_tuple(vec![py_str("marmite"), py_str("vegemite"), py_str("bovril")]), py_str("Marmite keeps for years - a couple of years past the date, easy - because it is salty and dark and nothing grows in it. It can dry out and go stiff in the jar; a spoonful of hot water stirred in brings it back, and it is safe to finish either way.")]), py_tuple(vec![py_tuple(vec![py_str("golden syrup"), py_str("treacle"), py_str("black treacle"), py_str("maple syrup")]), py_str("Golden syrup and treacle keep for years in the cupboard, and black treacle improves for the first year. Maple syrup is the one that needs the fridge once opened, about six months, and it is the only one here that can grow mould on top.")]), py_tuple(vec![py_tuple(vec![py_str("worcestershire sauce"), py_str("worcester sauce"), py_str("tabasco"), py_str("hot sauce"), py_str("chilli sauce"), py_str("sriracha"), py_str("franks")]), py_str("Worcestershire sauce and vinegar-based hot sauces keep for years in the cupboard, and about a year once opened; a hot sauce can darken and thicken, which is fine. The ones to refrigerate are the fresh chilli sauces and anything with fruit in it, which keep about three months.")]), py_tuple(vec![py_tuple(vec![py_str("salt"), py_str("table salt"), py_str("sea salt"), py_str("rock salt")]), py_str("Salt does not go off at all: it is a mineral and nothing grows in it, so the date means nothing. It clumps in damp air - a few grains of rice in the cellar, or a quick dry in a warm oven - and that is the only thing that changes.")]), py_tuple(vec![py_tuple(vec![py_str("baking powder"), py_str("bicarbonate of soda"), py_str("bicarb"), py_str("baking soda")]), py_str("Baking powder keeps about six months once the tin is opened, and bicarbonate of soda about a year, because they lose their fizz rather than spoil. Test it: a teaspoon in a cup of hot water should foam at once, and if it does not, the cake will not rise.")]), py_tuple(vec![py_tuple(vec![py_str("cornflour"), py_str("corn flour"), py_str("custard powder")]), py_str("Cornflour keeps about a year in a sealed tub in the cupboard, and custard powder about six months once opened, because both take up damp and go lumpy. It does not go mouldy or unsafe - it stops thickening properly, which is the sign to replace it.")]), py_tuple(vec![py_tuple(vec![py_str("ice cream"), py_str("ice cream tub"), py_str("vanilla ice cream"), py_str("gelato"), py_str("sorbet")]), py_str("Ice cream keeps about two to three months in the freezer before it dries out and goes icy, though it is safe far longer. Press a sheet of cling film onto the surface or keep the cardboard lid tight, and do not refreeze a tub that has melted, because that is when it turns grainy and grows ice crystals.")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.STORECUPBOARD_KEEP"); }
    s_STORECUPBOARD_KEEP(__data29());
    fn __data30() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("last"), py_str("lasts"), py_str("lasting"), py_str("go off"), py_str("goes off"), py_str("gone off"), py_str("going off"), py_str("how long"), py_str("store"), py_str("stores"), py_str("stored"), py_str("storing"), py_str("storage"), py_str("out of date"), py_str("past its date"), py_str("past its best"), py_str("use by"), py_str("still good"), py_str("still ok"), py_str("still safe"), py_str("shelf life"), py_str("expire"), py_str("expires"), py_str("expired"), py_str("in the fridge"), py_str("in the cupboard"), py_str("in a cupboard"), py_str("need to be in the fridge"), py_str("need to be refrigerated"), py_str("refrigerate"), py_str("refrigerated"), py_str("gone hard"), py_str("going hard"), py_str("gone stiff"), py_str("stale"), py_str("stales"), py_str("gone stale"), py_str("goes stale"), py_str("going stale"), py_str("went stale")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.SC_KEEP_ASKS"); }
    s_SC_KEEP_ASKS(__data30());
    fn __data31() -> Py { py_tuple(vec![py_str("last week"), py_str("last night"), py_str("last time"), py_str("last month"), py_str("last year"), py_str("last summer"), py_str("last winter"), py_str("last spring"), py_str("last autumn"), py_str("last fall"), py_str("last christmas"), py_str("last easter"), py_str("last weekend"), py_str("last sunday"), py_str("last monday"), py_str("last tuesday"), py_str("last wednesday"), py_str("last thursday"), py_str("last friday"), py_str("last saturday")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._LAST_TIME"); }
    s__LAST_TIME(__data31());
    fn __data32() -> Py { py_tuple(vec![py_str("wash"), py_str("washing"), py_str("washed"), py_str("washing machine"), py_str("dishwasher"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("smelled"), py_str("stink"), py_str("stinks"), py_str("stinking"), py_str("odour"), py_str("odor"), py_str("musty"), py_str("limescale"), py_str("hard water")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CLEANING_WITH_IT"); }
    s__CLEANING_WITH_IT(__data32());
    fn __data33() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("make"), py_str("making"), py_str("homemade"), py_str("from scratch"), py_str("how do i make"), py_str("how do i cook"), py_str("how much"), py_str("per person"), py_str("calories"), py_str("good for you"), py_str("healthy"), py_str("diet"), py_str("marinade"), py_str("marinate"), py_str("substitute"), py_str("instead of"), py_str("brand"), py_str("cheapest"), py_str("buy"), py_str("price"), py_str("offer"), py_str("clean"), py_str("cleaning"), py_str("descale"), py_str("kettle"), py_str("sink"), py_str("drain"), py_str("wasp"), py_str("ant"), py_str("weed"), py_str("pickle"), py_str("pickling"), py_str("chutney"), py_str("left out"), py_str("leave it out"), py_str("left on the side"), py_str("sitting out"), py_str("out of the fridge"), py_str("defrost"), py_str("thaw"), py_str("thawed"), py_str("refreeze"), py_str("re-freeze"), py_str("brew"), py_str("brewing"), py_str("brew it"), py_str("steep"), py_str("steeping"), py_str("steeped"), py_str("mash"), py_str("tea bag in"), py_str("teabag in"), py_str("the bag in"), py_str("leave the bag"), py_str("leaving the bag"), py_str("cuppa"), py_str("milk in first"), py_str("teapot"), py_str("infuse"), py_str("infusing"), py_str("cool"), py_str("cooling"), py_str("cool down"), py_str("cools down"), py_str("too hot to drink"), py_str("how long for"), py_str("how many minutes"), py_str("cook"), py_str("cooking"), py_str("roast"), py_str("roasting"), py_str("boil"), py_str("boiling"), py_str("joint"), py_str("gammon"), py_str("take to cook"), py_str("how long to cook"), py_str("in the oven"), py_str("air fryer"), py_str("slow cooker"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("pups"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("rabbit"), py_str("rabbits"), py_str("hamster"), py_str("hamsters"), py_str("budgie"), py_str("budgies"), py_str("guinea pig"), py_str("guinea pigs"), py_str("pet"), py_str("pets"), py_str("vet")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.SC_KEEP_AVOID"); }
    s_SC_KEEP_AVOID(op_add(&op_add(&g__LAST_TIME(), &g__CLEANING_WITH_IT()), &__data33()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.SC_KEEP_NAMES"); }
    s_SC_KEEP_NAMES(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STORECUPBOARD_KEEP()) { let mut row: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&row.clone(), &Py::Int(0))) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data34() -> Py { py_tuple(vec![py_str("leftover"), py_str("leftovers"), py_str("left over"), py_str("reheat"), py_str("reheated"), py_str("reheating"), py_str("next day"), py_str("the next day"), py_str("the day after"), py_str("day after"), py_str("day later"), py_str("days later"), py_str("yesterday"), py_str("in the fridge"), py_str("out of the fridge"), py_str("takeaway")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.DATED_STAND_DOWN"); }
    s_DATED_STAND_DOWN(py_dict(vec![(py_str("rice"), __data34())]));
    fn __data35() -> Py { py_tuple(vec![py_str("wash"), py_str("washing"), py_str("washed"), py_str("recipe"), py_str("recipes"), py_str("sauce"), py_str("soup"), py_str("risotto"), py_str("pizza"), py_str("lasagne"), py_str("stew"), py_str("pie"), py_str("curry"), py_str("pickle"), py_str("dry"), py_str("dried"), py_str("grow"), py_str("growing"), py_str("forage"), py_str("foraging"), py_str("magic")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("brown"), py_str("browns"), py_str("browning"), py_str("going brown"), py_str("goes brown"), py_str("go brown"), py_str("green"), py_str("stay green"), py_str("keep green"), py_str("fresh"), py_str("stop"), py_str("prevent"), py_str("why")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ENTRIES"); }
    s_ENTRIES(op_add(&op_add(&op_add(&op_add(&op_add(&py_list(vec![g__DAY_AFTER_ENTRY(), g_DATES_OWN(), g_YOGHURT()]), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DATE_ROWS()) { let mut row: Py = __item.clone(); __out.push(_dated_row(&[row.clone()])); } py_list(__out) })() }), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-cooked-veg")); __args.push(py_tuple(vec![g_COOKED_VEG_NAMES(), g_KEEPING_KEY()])); __args.push(g_COOKED_VEG_REPLY()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_COOKED_VEG_STAND_DOWN(), &g_COOKED_VEG_COOK_WORDS()), &g_USE_UP_WORDS()), &g_FLOP_WORDS()), &g_KEEP_TECHNIQUE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ROWS()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk1[0].clone(); let mut reply: Py = __unpk1[1].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("keep-%s"), &geti(&names.clone(), &Py::Int(0)))); __args.push(py_tuple(vec![names.clone(), g_KEEPING_KEY()])); __args.push(reply.clone()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STAND_DOWN()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&g_COOKED_IN_THE_ROW(), &geti(&names.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_COOKED_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_COOK_LINE_WORDS(), &word.clone())) } } } }))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &mcall(&g_ROW_COOK_FORMS(), "get", &[geti(&names.clone(), &Py::Int(0)), py_tuple(vec![])])), &g_FLOP_WORDS()), &g_KEEP_TECHNIQUE()), &mcall(&g_ROW_STAND_DOWN(), "get", &[geti(&names.clone(), &Py::Int(0)), py_tuple(vec![])])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() }), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ROWS()) { let __unpk1 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk1[0].clone(); let mut _reply: Py = __unpk1[1].clone(); if py_truthy(&Py::Bool(py_contains(&g_FIX_FIRST(), &geti(&names.clone(), &Py::Int(0))))) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("keep-%s-limp"), &geti(&names.clone(), &Py::Int(0)))); __args.push(py_tuple(vec![names.clone(), g_FLOP_WORDS()])); __args.push(geti(&g_FIX_FIRST(), &geti(&names.clone(), &Py::Int(0)))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_FLOP_WORDS()), ("avoid".to_string(), g__STAND_DOWN())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } } py_list(__out) })() }), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-mushroom-slimy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("slimy"), py_str("slimey"), py_str("slimier"), py_str("slimiest"), py_str("soggy"), py_str("sweaty"), py_str("wet")]), py_tuple(vec![py_str("mushroom"), py_str("mushrooms"), py_str("button mushrooms"), py_str("chestnut mushrooms"), py_str("closed cup"), py_str("portobello"), py_str("shiitake")])])); __args.push(py_func("_slimy_mushrooms", |args: &[Py]| _slimy_mushrooms(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data35())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("guacamole-browns")); __args.push(py_tuple(vec![py_tuple(vec![py_str("guacamole"), py_str("guac"), py_str("avocado dip"), py_str("avocado mash")]), __data36()])); __args.push(py_str("It is the air, not the lime: a cut avocado browns where oxygen reaches it, and the greenest guacamole is the one with the least air in it. Press cling film right onto the surface in a shallow tub, keep the stones in if you like, and a squeeze of lime plus a pinch of salt slows it. If it has already browned it is still fine to eat - stir the top layer in and it looks green again.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("has gone"), py_str("have gone"), py_str("went"), py_str("turned"), py_str("recipe"), py_str("recipes"), py_str("make"), py_str("making"), py_str("freeze"), py_str("frozen"), py_str("calories"), py_str("how much")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])));
    fn __data37() -> Py { py_tuple(vec![py_str("power cut"), py_str("power cuts"), py_str("power went off"), py_str("power goes off"), py_str("power has gone off"), py_str("power is off"), py_str("power was off"), py_str("power failure"), py_str("power outage"), py_str("power out"), py_str("lost power"), py_str("lose power"), py_str("losing power"), py_str("no power"), py_str("without power"), py_str("electricity went off"), py_str("electricity was off"), py_str("electricity is off"), py_str("no electricity"), py_str("without electricity"), py_str("lost the electricity"), py_str("freezer went off"), py_str("freezer was off"), py_str("freezer is off"), py_str("freezer has been off"), py_str("cut out"), py_str("cut off"), py_str("power goes"), py_str("power go"), py_str("power went"), py_str("power came back"), py_str("power is back"), py_str("when the power goes"), py_str("if the power goes"), py_str("if the power cuts")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.CUT_WORDS"); }
    s_CUT_WORDS(__data37());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.CUT_OUT_FOR"); }
    s_CUT_OUT_FOR(stdlib_call("re", "compile", &[py_str("\\bcut out (?:for|to)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data38() -> Py { py_tuple(vec![py_str("last night"), py_str("yesterday"), py_str("overnight"), py_str("this morning"), py_str("today"), py_str("hours"), py_str("hour"), py_str("day"), py_str("days"), py_str("two days"), py_str("3 days"), py_str("four hours"), py_str("6 hours"), py_str("since"), py_str("meanwhile"), py_str("still"), py_str("think"), py_str("worried"), py_str("safe"), py_str("ok"), py_str("fine"), py_str("food"), py_str("freezer"), py_str("fridge"), py_str("shut"), py_str("closed"), py_str("opened"), py_str("cold")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.WHEN_WORDS"); }
    s_WHEN_WORDS(__data38());
    fn __data39() -> Py { py_tuple(vec![py_str("thawed out"), py_str("thawed"), py_str("thaw"), py_str("thawing"), py_str("defrosted out"), py_str("defrosted"), py_str("defrost"), py_str("defrosting"), py_str("melted out"), py_str("melted"), py_str("unfrozen"), py_str("no longer frozen"), py_str("gone soft in the freezer"), py_str("soggy in the freezer"), py_str("soggy from the freezer"), py_str("soggy out of the freezer"), py_str("soggy after the freezer")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.THAW_WORDS"); }
    s_THAW_WORDS(__data39());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.REFREEZE_WORDS"); }
    s_REFREEZE_WORDS(py_tuple(vec![py_str("refreeze"), py_str("refrozen"), py_str("refroze"), py_str("re-freeze"), py_str("re freeze"), py_str("refreezing"), py_str("put it back in the freezer"), py_str("back in the freezer"), py_str("freeze it again")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.TIMES_A_THAW"); }
    s_TIMES_A_THAW(stdlib_call("re", "compile", &[py_str("\\b(?:defrost|thaw)\\w*\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ASKS_HOW_LONG"); }
    s_ASKS_HOW_LONG(stdlib_call("re", "compile", &[py_str("\\bhow (?:long|many (?:hours|minutes|days)|soon)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data40() -> Py { py_tuple(vec![py_str("tariff"), py_str("economy 7"), py_str("economy seven"), py_str("night rate"), py_str("day rate"), py_str("bill"), py_str("bills"), py_str("supplier"), py_str("suppliers"), py_str("standing charge"), py_str("direct debit"), py_str("kwh"), py_str("unit rate"), py_str("cheaper"), py_str("cheapest"), py_str("worth it"), py_str("should i buy"), py_str("buy a freezer"), py_str("chest or upright"), py_str("chest freezer or"), py_str("upright"), py_str("running cost"), py_str("energy use"), py_str("how much electricity"), py_str("switch"), py_str("solar"), py_str("battery"), py_str("generator"), py_str("camping"), py_str("pregnant"), py_str("pregnancy"), py_str("trimester"), py_str("weeks pregnant")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COLD_NOT"); }
    s_COLD_NOT(__data40());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.COLD_REPLIES"); }
    s_COLD_REPLIES(py_dict(vec![(py_str("cut"), py_str("Keep the doors shut and most of it is fine: a full freezer holds its cold for about 48 hours, a half-full one about 24, and a fridge about 4. Do not open them to look - that is what lets the cold out. When the power is back, cook and eat the thawing things first and leave the rest frozen.")), (py_str("thawed"), py_str("Judge it on ice crystals rather than on how long it sat: food that is still firm and icy can be cooked and eaten or refrozen, and meat that thawed in the fridge can always go back in. Anything soft and warm goes in the bin - meat, fish, shellfish, cooked rice and milk most of all - and do not taste it to check. Melted ice cream is binned whatever it feels like.")), (py_str("fridge"), py_str("Food in a fridge whose door stayed shut is safe for about 4 hours, and after that the perishables go: milk, soft cheese, cooked meat, cooked rice and leftovers, and anything past its date. Butter, hard cheese, jam, pickles and most fruit and veg come through a warm day, so a full clear-out is not needed.")), (py_str("icecream"), py_str("It is safe but it will be grainy: melting and refreezing separates the ice cream, so it goes icy and slushy rather than creamy. Eat it now rather than keep it. If the tub was actually warm for hours, bin it - dairy that has been warm is not worth the risk."))]));
    fn __data41() -> Py { py_tuple(vec![py_str("cheddar"), py_str("parmesan"), py_str("brie"), py_str("camembert"), py_str("feta"), py_str("stilton"), py_str("mozzarella"), py_str("gruyere"), py_str("gorgonzola"), py_str("blue cheese"), py_str("goats cheese"), py_str("goat cheese"), py_str("cream cheese"), py_str("cottage cheese"), py_str("halloumi"), py_str("red leicester"), py_str("wensleydale")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.CHEESE_NAMES"); }
    s_CHEESE_NAMES(__data41());
    fn __data42() -> Py { py_tuple(vec![py_str("store"), py_str("stores"), py_str("storing"), py_str("stored"), py_str("how do i store"), py_str("how should i store"), py_str("how to store"), py_str("how do you store"), py_str("best way to store"), py_str("where do i keep"), py_str("where should i keep"), py_str("wrap"), py_str("wrapped"), py_str("wrapping"), py_str("once opened")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.STORE_ASKS"); }
    s_STORE_ASKS(__data42());
    fn __data43() -> Py { py_tuple(vec![py_str("mould"), py_str("mouldy"), py_str("mold"), py_str("moldy"), py_str("moulds"), py_str("fuzzy"), py_str("furry"), py_str("gone mouldy"), py_str("green bits"), py_str("white bits"), py_str("spots"), py_str("speckled"), py_str("fur"), py_str("blue bits"), py_str("blue spots"), py_str("blue spot"), py_str("bits of blue")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.MOULD_WORDS"); }
    s_MOULD_WORDS(__data43());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.SOFT_CHEESES"); }
    s_SOFT_CHEESES(py_tuple(vec![py_str("cream cheese"), py_str("cottage cheese"), py_str("brie"), py_str("camembert"), py_str("mozzarella"), py_str("feta"), py_str("goat cheese"), py_str("goats cheese"), py_str("mascarpone"), py_str("ricotta")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.BLUE_CHEESES"); }
    s_BLUE_CHEESES(py_tuple(vec![py_str("blue cheese"), py_str("stilton"), py_str("gorgonzola"), py_str("roquefort"), py_str("dolcelatte")]));
    fn __data44() -> Py { py_tuple(vec![py_str("how do i defrost"), py_str("how to defrost"), py_str("how do you defrost"), py_str("how can i defrost"), py_str("defrost my freezer"), py_str("defrost the freezer"), py_str("defrost a freezer"), py_str("defrosting a freezer"), py_str("best way to defrost"), py_str("way to defrost"), py_str("needs defrosting"), py_str("needs a defrost"), py_str("frosting up"), py_str("should i defrost"), py_str("shall i defrost"), py_str("do i defrost it"), py_str("defrost it now")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("door"), py_str("left open"), py_str("left it open"), py_str("left the door open"), py_str("was left open"), py_str("got left open"), py_str("door was open"), py_str("door open"), py_str("open all night"), py_str("open overnight"), py_str("open all day"), py_str("ajar"), py_str("didnt shut"), py_str("did not shut"), py_str("didnt close"), py_str("not closed properly"), py_str("stayed open"), py_str("open for hours"), py_str("left ajar")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("left open"), py_str("left it open"), py_str("left the door open"), py_str("was left open"), py_str("got left open"), py_str("door was open"), py_str("door open"), py_str("open all night"), py_str("open overnight"), py_str("ajar"), py_str("didnt shut"), py_str("did not shut"), py_str("stayed open")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("seal"), py_str("seals"), py_str("gasket"), py_str("thermostat"), py_str("temperature setting"), py_str("too cold"), py_str("not cold enough"), py_str("frost"), py_str("freezing up"), py_str("ice box"), py_str("how do i defrost"), py_str("how to defrost"), py_str("defrost my freezer"), py_str("defrost the freezer")]) }
    let __aug48 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-cut")); __args.push(py_tuple(vec![g_CUT_WORDS(), g_WHEN_WORDS()])); __args.push(geti(&g_COLD_REPLIES(), &py_str("cut"))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_CUT_WORDS()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_cut_out_for_is_an_aptitude(&[understood.clone()]))) })), ("avoid".to_string(), g_COLD_NOT())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-thawed")); __args.push(py_tuple(vec![g_THAW_WORDS(), op_add(&g_WHEN_WORDS(), &py_tuple(vec![py_str("meat"), py_str("chicken"), py_str("food"), py_str("freezer")]))])); __args.push(geti(&g_COLD_REPLIES(), &py_str("thawed"))); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("ask".to_string(), g_THAW_WORDS()), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&_a_thaw_that_is_a_job(&[understood.clone()]))) })), ("avoid".to_string(), op_add(&op_add(&op_add(&g_COLD_NOT(), &__data44()), &g_REFREEZE_WORDS()), &py_tuple(vec![py_str("again"), py_str("second time")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("soggy-from-the-freezer")); __args.push(py_tuple(vec![py_tuple(vec![py_str("soggy"), py_str("soggyish"), py_str("mushy")]), py_tuple(vec![py_str("frozen"), py_str("freezer"), py_str("froze"), py_str("ice"), py_str("icy"), py_str("ice crystals")])])); __args.push(geti(&g_COLD_REPLIES(), &py_str("thawed"))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("soggy"), py_str("mushy")])), ("avoid".to_string(), py_tuple(vec![py_str("pan"), py_str("salad"), py_str("batter"), py_str("breaded"), py_str("sauce"), py_str("risotto"), py_str("soup"), py_str("chips"), py_str("fries"), py_str("roast"), py_str("cooked")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-kept-shut")); __args.push(py_tuple(vec![py_tuple(vec![py_str("freezer"), py_str("fridge"), py_str("food")]), py_tuple(vec![py_str("stayed shut"), py_str("kept it shut"), py_str("kept the door shut"), py_str("didnt open"), py_str("did not open"), py_str("stayed closed"), py_str("not opened"), py_str("still cold"), py_str("still frozen")])])); __args.push(geti(&g_COLD_REPLIES(), &py_str("cut"))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("stayed shut"), py_str("kept it shut"), py_str("kept the door shut"), py_str("didnt open"), py_str("did not open"), py_str("stayed closed"), py_str("not opened")])), ("avoid".to_string(), op_add(&g_COLD_NOT(), &py_tuple(vec![py_str("wont stay shut"), py_str("wont shut"), py_str("wont close"), py_str("will not stay shut"), py_str("will not shut"), py_str("will not close"), py_str("keeps opening"), py_str("pops open"), py_str("keeps popping open"), py_str("door gap"), py_str("not shutting properly")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-door-open")); __args.push(py_tuple(vec![py_tuple(vec![py_str("freezer"), py_str("fridge"), py_str("freezer door"), py_str("fridge door")]), __data45()])); __args.push(py_str("Check for ice crystals rather than counting the hours: anything still firm and icy can be cooked and eaten or put back, and nothing needs binning for the door alone. What went soft and warm goes in the bin - meat, fish, shellfish, cooked rice and milk most of all - and do not taste it to check. A fridge is the stricter one: milk, cooked meat, cooked rice and leftovers go if the door was open for more than about four hours, while butter, hard cheese, jam and pickles come through it. Melting ice cream is binned whatever it feels like, and a freezer that was open all night is worth running for an hour before you trust the ice.")); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), __data46()), ("avoid".to_string(), op_add(&op_add(&g_COLD_NOT(), &__data47()), &g_REFREEZE_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-clear-out")); __args.push(py_tuple(vec![py_tuple(vec![py_str("freezer"), py_str("fridge freezer")]), py_tuple(vec![py_str("everything"), py_str("all of it"), py_str("throw out"), py_str("throw away"), py_str("bin it all"), py_str("clear out"), py_str("chuck out"), py_str("whole lot")])])); __args.push(geti(&g_COLD_REPLIES(), &py_str("thawed"))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), py_tuple(vec![py_str("throw out"), py_str("throw away"), py_str("bin it all"), py_str("bin everything"), py_str("clear out"), py_str("chuck out"), py_str("should i bin")])), ("avoid".to_string(), g_COLD_NOT())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-fridge")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fridge was off"), py_str("fridge is off"), py_str("fridge has been off"), py_str("fridge went off"), py_str("fridge was warm"), py_str("fridge has been warm"), py_str("fridge is warm"), py_str("fridge off"), py_str("fridge")]), op_add(&g_WHEN_WORDS(), &py_tuple(vec![py_str("off"), py_str("warm"), py_str("warmth"), py_str("all day"), py_str("hours")]))])); __args.push(geti(&g_COLD_REPLIES(), &py_str("fridge"))); __kws.extend(vec![("bias".to_string(), Py::Int(5)), ("ask".to_string(), op_add(&g_CUT_WORDS(), &py_tuple(vec![py_str("fridge was off"), py_str("fridge is off"), py_str("fridge went off"), py_str("fridge off"), py_str("warm fridge"), py_str("fridge warm")]))), ("avoid".to_string(), op_add(&g_COLD_NOT(), &py_tuple(vec![py_str("fridge freezer")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cold-ice-cream")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ice cream"), py_str("icecream"), py_str("ice lolly"), py_str("ice lollies"), py_str("gelato"), py_str("frozen yogurt")]), op_add(&op_add(&g_THAW_WORDS(), &g_REFREEZE_WORDS()), &py_tuple(vec![py_str("ok"), py_str("safe"), py_str("still"), py_str("bin")]))])); __args.push(geti(&g_COLD_REPLIES(), &py_str("icecream"))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), py_tuple(vec![py_str("ice cream"), py_str("icecream"), py_str("ice lolly"), py_str("ice lollies"), py_str("gelato")])), ("avoid".to_string(), g_COLD_NOT())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ENTRIES"); }
    s_ENTRIES(__aug48);
    fn __data49() -> Py { py_tuple(vec![py_str("green"), py_str("still green"), py_str("too green"), py_str("very green"), py_str("so green"), py_str("green as"), py_str("not ripened"), py_str("hasnt ripened"), py_str("hasn't ripened"), py_str("didnt ripen"), py_str("didn't ripen"), py_str("wont ripen"), py_str("refusing to ripen")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.UNDER_RIPE"); }
    s_UNDER_RIPE(__data49());
    fn __data50() -> Py { py_tuple(vec![py_str("ripe"), py_str("riper"), py_str("ripen"), py_str("ripens"), py_str("ripening"), py_str("unripe"), py_str("underripe"), py_str("overripe"), py_str("hard"), py_str("firm"), py_str("still hard"), py_str("ready"), py_str("ready to eat"), py_str("soft enough"), py_str("tell if"), py_str("know if"), py_str("how do i tell"), py_str("how can i tell"), py_str("when is it ready"), py_str("sweet enough"), py_str("softer"), py_str("soften"), py_str("sour"), py_str("not ripe"), py_str("too hard"), py_str("rock hard"), py_str("tasteless"), py_str("soft"), py_str("squishy"), py_str("soft at the bottom"), py_str("soft on the bottom"), py_str("gone soft"), py_str("getting soft"), py_str("pick a good"), py_str("pick good"), py_str("picking a good"), py_str("pick the best"), py_str("pick a ripe"), py_str("pick ripe"), py_str("choose a good"), py_str("choosing a good"), py_str("choose the best"), py_str("choose a ripe"), py_str("choosing a ripe"), py_str("good one"), py_str("good ones"), py_str("best one"), py_str("how do i pick"), py_str("how do i choose"), py_str("how to pick"), py_str("how to choose"), py_str("which one to pick"), py_str("which one to choose"), py_str("which to pick"), py_str("which to choose"), py_str("pick one"), py_str("choose one")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.RIPEN_WORDS"); }
    s_RIPEN_WORDS(op_add(&g_UNDER_RIPE(), &__data50()));
    fn __data51() -> Py { py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("mango"), py_str("mangoes"), py_str("mangos")]), py_str("A ripe mango gives slightly under a gentle press, like a ripe peach, and smells sweet at the stem end; the red blush says nothing, so a green one can still be ready."), py_str("A hard one ripens on the counter in 2 to 4 days, faster in a paper bag with a banana, since the banana gives off the gas that does the work."), py_str("Once it gives, it keeps about 5 days in the fridge, and a cut mango keeps 3 days in a sealed tub.")]), py_tuple(vec![py_tuple(vec![py_str("melon"), py_str("melons"), py_str("cantaloupe"), py_str("honeydew"), py_str("galia")]), py_str("A ripe melon feels heavy for its size, and the end opposite the stem gives a little under your thumb; a cantaloupe smells sweet there too."), py_str("A whole one softens on the counter in 2 or 3 days, quicker in a paper bag, but once it is cut it will not ripen any further."), py_str("Wrap cut melon and keep it 3 days in the fridge; a whole ripe one keeps about a week there.")]), py_tuple(vec![py_tuple(vec![py_str("watermelon"), py_str("watermelons")]), py_str("A ripe watermelon is heavy for its size, has a creamy yellow patch where it lay on the ground, and gives a hollow thud when you tap it."), py_str("It will not ripen once it is off the vine, so a pale one stays pale and the only thing to do is keep it cool and eat it."), py_str("Whole, it keeps a week or two in a cool room, and cut watermelon keeps 3 days in the fridge in a sealed tub.")]), py_tuple(vec![py_tuple(vec![py_str("peach"), py_str("peaches"), py_str("nectarine"), py_str("nectarines")]), py_str("A ripe peach gives to a gentle press and smells sweet at the stem, and the skin between the red patches is cream rather than green."), py_str("Peaches ripen in a paper bag on the counter in 1 to 3 days, and the fridge is what stops them and turns them mealy."), py_str("Once they give, keep them in the fridge for 3 to 5 days and take them out an hour before you eat them.")]), py_tuple(vec![py_tuple(vec![py_str("plum"), py_str("plums"), py_str("apricot"), py_str("apricots"), py_str("greengage"), py_str("greengages")]), py_str("A ripe plum gives slightly under the thumb and smells sweet at the stem, and its skin should be tight rather than wrinkled."), py_str("Plums and apricots ripen in a paper bag on the counter in 2 to 4 days."), py_str("Once ripe they keep about a week in the fridge, and 3 days in a fruit bowl.")]), py_tuple(vec![py_tuple(vec![py_str("cherry"), py_str("cherries")]), py_str("A ripe cherry is firm and shiny, dark all over for its variety, with a green stem that bends rather than snaps."), py_str("Cherries do not ripen after picking, so a pale, soft one stays pale and soft."), py_str("Keep them unwashed in the fridge for about a week and wash them just before you eat, because a wet cherry splits and moulds.")]), py_tuple(vec![py_tuple(vec![py_str("pear"), py_str("pears")]), py_str("A ripe pear gives a little at the neck, just under the stem, and a pear that is soft in the belly is already past its best."), py_str("Pears ripen from the inside out, so buy them firm and leave them on the counter for 2 to 4 days, or in a paper bag with a banana."), py_str("Once the neck gives, they keep about 5 days in the fridge and go floury left out in the bowl.")]), py_tuple(vec![py_tuple(vec![py_str("kiwi"), py_str("kiwis"), py_str("kiwifruit")]), py_str("A ripe kiwi gives under a gentle press, the way a ripe avocado does."), py_str("Hard kiwis ripen on the counter in 3 to 5 days, faster in a paper bag with an apple or a banana."), py_str("Once soft, they keep about 2 weeks in the fridge, and apart from other fruit if you want them to stay firm.")]), py_tuple(vec![py_tuple(vec![py_str("pineapple"), py_str("pineapples")]), py_str("A ripe pineapple smells sweet at the base, feels heavy for its size, and a leaf from the middle of the crown pulls out easily. Softening at the bottom is part of it ripening, so a firm one with a soft base is ready rather than wrong."), py_str("A pineapple gets no sweeter once it is picked, only softer, and it starts at the base, so the one you buy is as good as it will get."), py_str("Whole, it keeps about a week on the counter, and cut pineapple keeps 3 to 4 days in the fridge in its own juice. One that is soft all over and smells of wine is overripe: eat it today or freeze it.")]), py_tuple(vec![py_tuple(vec![py_str("fig"), py_str("figs")]), py_str("A ripe fig is soft all over, droops at the neck and often has a bead of syrup at the eye."), py_str("Figs do not ripen once they are picked, so a hard one stays hard."), py_str("Keep them 2 or 3 days in the fridge, laid on a plate rather than piled up, and they taste best at room temperature.")]), py_tuple(vec![py_tuple(vec![py_str("grape"), py_str("grapes"), py_str("bunch of grapes")]), py_str("Ripe grapes are firm and come away from the bunch easily, with a dusty bloom on the skin rather than a sticky one."), py_str("Grapes do not ripen after picking, so a sour bunch stays sour."), py_str("Keep them unwashed in a bag in the fridge for a week or more, and wash them just before eating.")]), py_tuple(vec![py_tuple(vec![py_str("tomato"), py_str("tomatoes")]), py_str("A tomato is ready when it feels heavy and gives slightly in the hand, smells of tomato where the stalk was, and the colour is deep and even; a hard, pale one that smells of green is not there yet."), py_str("They ripen off the vine in a bowl on the counter, quicker with a banana or an apple beside them, and the fridge is what stops that and kills the flavour."), py_str("Keep them out of the fridge for up to a week on the counter, and a cut tomato in a tub for 2 days.")]), py_tuple(vec![py_tuple(vec![py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes")]), py_str("Citrus is picked ripe, so the test is weight rather than colour: a lemon or lime that feels heavy for its size and gives slightly under a thumb is full of juice, and a light, rock-hard one is dry inside."), py_str("Nothing ripens them at home - a hard one softens a little on the counter in a few days, and a soft one goes straight in the fridge."), py_str("Keep them a week or two in the bowl and a month in the fridge, and a lemon going soft is still good for juice and zest.")]), py_tuple(vec![py_tuple(vec![py_str("orange"), py_str("oranges"), py_str("satsuma"), py_str("satsumas"), py_str("clementine"), py_str("clementines"), py_str("tangerine"), py_str("tangerines")]), py_str("An orange or a satsuma is ready when it feels heavy for its size, which is juice rather than thick skin, and smells sweet at the stem; the small thin-skinned ones are usually the sweeter."), py_str("Citrus does not ripen any further once it is picked, so a pale, dry one stays that way."), py_str("Keep them a week or two in the bowl and a month in the fridge, and a clementine that is light and hollow-feeling has dried out at the ends.")]), py_tuple(vec![py_tuple(vec![py_str("apple"), py_str("apples"), py_str("braeburn"), py_str("cox"), py_str("gala"), py_str("granny smith")]), py_str("Apples are picked ripe and keep, so judge by feel: firm, heavy, a clean apple smell and a fresh green stem."), py_str("There is nothing to ripen - a soft or mealy one has been off the tree too long and will not come back."), py_str("Keep them for weeks in the fridge, far longer than on the counter, and away from other fruit because an apple gives off the gas that ripens everything beside it.")]), py_tuple(vec![py_tuple(vec![py_str("coconut"), py_str("coconuts")]), py_str("Shake it: a fresh coconut sloshes with liquid, and a silent one has dried out inside. Pick one heavy for its size with no cracks or damp patches at the eyes."), py_str("A coconut is picked ripe, so there is nothing to do at home but open it."), py_str("Drink the water the day it is opened, keep the flesh 3 to 4 days in a tub in the fridge, and freeze what you will not eat.")]), py_tuple(vec![py_tuple(vec![py_str("papaya"), py_str("papayas"), py_str("pawpaw"), py_str("pawpaws")]), py_str("A papaya is ready when the skin is mostly yellow-orange rather than green, it gives slightly under a thumb and smells faintly sweet rather than green."), py_str("A green one ripens on the counter in 2 or 3 days, faster in a paper bag, and the fridge holds it once it is soft."), py_str("Eat a ripe one within 2 days, and a cut one within 2 days from the fridge.")]), py_tuple(vec![py_tuple(vec![py_str("strawberry"), py_str("strawberries")]), py_str("Strawberries do not ripen after picking, so look for deep red right up to the tip under the leaves, a sweet smell and a dry, bright green cap."), py_str("There is nothing to ripen at home and a pale one will not colour up; take them out of the fridge half an hour before you eat them, which is the only thing that improves them."), py_str("Keep them 3 to 5 days in the fridge, unwashed and spread on kitchen paper, since washing them early is what turns them soft.")]), py_tuple(vec![py_tuple(vec![py_str("blueberry"), py_str("blueberries"), py_str("raspberry"), py_str("raspberries"), py_str("blackberry"), py_str("blackberries")]), py_str("Ripe berries are deep in colour all over with a dusty bloom, plump and only just soft, and a shrunken or leaking one is past it; taste one, because a punnet is better judged by flavour than by colour."), py_str("Berries do not ripen once they are picked, so a pale one stays pale and sharp."), py_str("Keep them 3 to 5 days in the fridge, spread on a plate lined with kitchen paper rather than heaped in a bag.")]), py_tuple(vec![py_tuple(vec![py_str("pomegranate"), py_str("pomegranates")]), py_str("A ripe pomegranate is heavy for its size, has square-ish sides rather than a round ball, and leathery, slightly scuffed skin rather than a shiny one."), py_str("Pomegranates are picked ripe and do not ripen further, so a pale, light one stays sharp."), py_str("Keep them for weeks in a cool cupboard, the seeds sweetest the day it is opened.")])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.RIPEN_TABLE"); }
    s_RIPEN_TABLE(__data51());
    fn __data52() -> Py { py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("store"), py_str("storing"), py_str("storage"), py_str("last"), py_str("lasts"), py_str("how long"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("how much longer")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.KEEP_ASKS"); }
    s_KEEP_ASKS(__data52());
    fn __data53() -> Py { py_tuple(vec![py_str("ripen"), py_str("ripens"), py_str("ripening"), py_str("hurry"), py_str("faster"), py_str("speed it up"), py_str("speed them up"), py_str("softer"), py_str("soften"), py_str("still hard"), py_str("rock hard"), py_str("hard as"), py_str("too hard"), py_str("not ripe"), py_str("unripe"), py_str("still firm"), py_str("sour"), py_str("tasteless"), py_str("no flavour"), py_str("no flavor"), py_str("get it ripe"), py_str("get them ripe"), py_str("make it ripe"), py_str("make them ripe"), py_str("hard"), py_str("sour"), py_str("tasteless"), py_str("bland")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.RIPEN_ASKS"); }
    s_RIPEN_ASKS(op_add(&g_UNDER_RIPE(), &__data53()));
    fn __data54() -> Py { py_tuple(vec![py_str("tell if"), py_str("know if"), py_str("how do i tell"), py_str("how can i tell"), py_str("how to tell"), py_str("is it ready"), py_str("is it ripe"), py_str("when is it"), py_str("when are they"), py_str("ready to eat"), py_str("supposed to be"), py_str("supposed to"), py_str("should it be"), py_str("should they be"), py_str("meant to be"), py_str("is it normal"), py_str("is that normal"), py_str("press"), py_str("squeeze"), py_str("smell"), py_str("feel like")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.TELL_ASKS"); }
    s_TELL_ASKS(__data54());
    fn __data55() -> Py { py_tuple(vec![py_str("pizza"), py_str("hawaiian"), py_str("topping"), py_str("toppings"), py_str("juice"), py_str("smoothie"), py_str("jam"), py_str("chutney"), py_str("pie"), py_str("tart"), py_str("crumble"), py_str("cake"), py_str("salad"), py_str("salsa"), py_str("sauce"), py_str("recipe"), py_str("recipes"), py_str("curry"), py_str("muffin"), py_str("loaf"), py_str("grow"), py_str("growing"), py_str("grew"), py_str("plant"), py_str("plants"), py_str("planted"), py_str("vine"), py_str("vines"), py_str("tree"), py_str("trees"), py_str("blight"), py_str("greenhouse"), py_str("allotment"), py_str("seed"), py_str("seeds"), py_str("sow"), py_str("sowing"), py_str("compost"), py_str("peel"), py_str("peeled"), py_str("peeling"), py_str("deseed"), py_str("stone"), py_str("stoned"), py_str("core"), py_str("cored"), py_str("cut up"), py_str("chop"), py_str("chopped"), py_str("tin"), py_str("tins"), py_str("tinned"), py_str("tin of"), py_str("tins of"), py_str("can of"), py_str("cans of"), py_str("jar"), py_str("jars"), py_str("opened"), py_str("in syrup"), py_str("dried"), py_str("dried fruit"), py_str("sundried"), py_str("in a can"), py_str("out of a can"), py_str("from a can"), py_str("in a jar"), py_str("out of a jar")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.RIPEN_ASIDE"); }
    s_RIPEN_ASIDE(__data55());
    let __aug56 = op_add(&g_ENTRIES(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RIPEN_TABLE()) { let __unpk1 = py_unpack(&__item.clone(), 4, None); let mut names: Py = __unpk1[0].clone(); let mut test: Py = __unpk1[1].clone(); let mut trick: Py = __unpk1[2].clone(); let mut keep: Py = __unpk1[3].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("ripen-%s"), &geti(&names.clone(), &Py::Int(0)))); __args.push(py_tuple(vec![names.clone(), op_add(&call_builtin("tuple", &[g_KEEPING_KEY()]).unwrap_or_else(|e| exc::raise(e)), &g_RIPEN_WORDS())])); __args.push(_ripens(&[test.clone(), trick.clone(), keep.clone()])); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g__STAND_DOWN(), &g_RIPEN_ASIDE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ENTRIES"); }
    s_ENTRIES(__aug56);
    fn __data57() -> Py { py_tuple(vec![py_str("hard"), py_str("hard as"), py_str("as hard as"), py_str("rock hard"), py_str("still hard"), py_str("so hard"), py_str("unripe"), py_str("not ripe"), py_str("nowhere near ripe"), py_str("too hard"), py_str("firm as"), py_str("as firm as"), py_str("like a stone"), py_str("like stones"), py_str("as stones"), py_str("as a stone"), py_str("sour"), py_str("tasteless"), py_str("no flavour"), py_str("no flavor"), py_str("won't ripen"), py_str("wont ripen"), py_str("will not ripen"), py_str("not softening"), py_str("refuses to ripen")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.UNRIPE_STATED"); }
    s_UNRIPE_STATED(op_add(&g_UNDER_RIPE(), &__data57()));
    fn __data58() -> Py { py_tuple(vec![py_str("pizza"), py_str("hawaiian"), py_str("topping"), py_str("toppings"), py_str("juice"), py_str("smoothie"), py_str("jam"), py_str("chutney"), py_str("pie"), py_str("tart"), py_str("crumble"), py_str("cake"), py_str("salad"), py_str("salsa"), py_str("sauce"), py_str("recipe"), py_str("recipes"), py_str("curry"), py_str("muffin"), py_str("loaf"), py_str("grow"), py_str("growing"), py_str("grew"), py_str("plant"), py_str("plants"), py_str("planted"), py_str("vine"), py_str("vines"), py_str("tree"), py_str("trees"), py_str("blight"), py_str("greenhouse"), py_str("allotment"), py_str("seed"), py_str("seeds"), py_str("sow"), py_str("sowing"), py_str("compost"), py_str("tin"), py_str("tins"), py_str("tinned"), py_str("tin of"), py_str("tins of"), py_str("can of"), py_str("cans of"), py_str("jar"), py_str("jars"), py_str("opened"), py_str("in syrup"), py_str("dried"), py_str("dried fruit"), py_str("sundried"), py_str("in a can"), py_str("out of a can"), py_str("from a can"), py_str("in a jar"), py_str("out of a jar")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._RIPEN_ASIDE_KEEPING"); }
    s__RIPEN_ASIDE_KEEPING(__data58());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._STAND_DOWN_HARD"); }
    s__STAND_DOWN_HARD(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__STAND_DOWN()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_DISH_ASK()) { let mut dish: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&word.clone(), &dish.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&dish.clone(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    let __aug59 = op_add(&g_ENTRIES(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RIPEN_TABLE()) { let __unpk1 = py_unpack(&__item.clone(), 4, None); let mut names: Py = __unpk1[0].clone(); let mut test: Py = __unpk1[1].clone(); let mut trick: Py = __unpk1[2].clone(); let mut keep: Py = __unpk1[3].clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("ripen-stated-%s"), &geti(&names.clone(), &Py::Int(0)))); __args.push(py_tuple(vec![names.clone(), g_UNRIPE_STATED()])); __args.push(_ripens(&[test.clone(), trick.clone(), keep.clone()])); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g__STAND_DOWN_HARD(), &g__RIPEN_ASIDE_KEEPING()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }); } py_list(__out) })() });
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ENTRIES"); }
    s_ENTRIES(__aug59);
    fn __data60() -> Py { py_tuple(vec![py_str("carrot"), py_str("carrots"), py_str("potato"), py_str("potatoes"), py_str("new potato"), py_str("new potatoes"), py_str("parsnip"), py_str("parsnips"), py_str("swede"), py_str("swedes"), py_str("turnip"), py_str("turnips"), py_str("beetroot"), py_str("cabbage"), py_str("cabbages"), py_str("broccoli"), py_str("cauliflower"), py_str("sprout"), py_str("sprouts"), py_str("onion"), py_str("onions"), py_str("leek"), py_str("leeks"), py_str("celery"), py_str("courgette"), py_str("courgettes"), py_str("aubergine"), py_str("aubergines"), py_str("pumpkin"), py_str("squash"), py_str("butternut"), py_str("sweet potato"), py_str("sweet potatoes"), py_str("green bean"), py_str("green beans"), py_str("runner bean"), py_str("runner beans"), py_str("pea"), py_str("peas"), py_str("sweetcorn"), py_str("spinach"), py_str("kale"), py_str("chard"), py_str("fennel"), py_str("asparagus"), py_str("artichoke"), py_str("artichokes"), py_str("mushroom"), py_str("mushrooms"), py_str("pepper"), py_str("peppers"), py_str("shallot"), py_str("shallots"), py_str("spring onion"), py_str("spring onions"), py_str("chestnut"), py_str("chestnuts"), py_str("lentil"), py_str("lentils"), py_str("chickpea"), py_str("chickpeas"), py_str("bean"), py_str("beans"), py_str("chickpeas"), py_str("pasta"), py_str("rice"), py_str("noodle"), py_str("noodles"), py_str("couscous"), py_str("quinoa"), py_str("barley"), py_str("stock"), py_str("stew"), py_str("soup"), py_str("curry"), py_str("casserole"), py_str("mince"), py_str("chicken"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("sausage"), py_str("sausages"), py_str("egg"), py_str("eggs")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._NOT_A_FRUIT"); }
    s__NOT_A_FRUIT(__data60());
    fn __data61() -> Py { py_tuple(vec![py_str("will they"), py_str("will it"), py_str("will these"), py_str("do they"), py_str("do these"), py_str("they ripen"), py_str("it ripen"), py_str("them ripen"), py_str("these ripen"), py_str("let them"), py_str("let these"), py_str("leave them"), py_str("how long"), py_str("until"), py_str("till they"), py_str("get them"), py_str("make them")]) }
    fn __data62() -> Py { py_tuple(vec![py_str("bread"), py_str("cake"), py_str("cheese"), py_str("banana bread"), py_str("yoghurt"), py_str("yogurt"), py_str("milk"), py_str("chicken"), py_str("meat"), py_str("fish"), py_str("wine"), py_str("beer"), py_str("sourdough"), py_str("dough"), py_str("starter"), py_str("breast"), py_str("bump"), py_str("pregnancy"), py_str("pregnant"), py_str("nappy"), py_str("nappies"), py_str("potty")]) }
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ripen-from-the-chat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ripen"), py_str("ripen them"), py_str("ripening"), py_str("ready"), py_str("soften"), py_str("ripe")]), __data61()])); __args.push(py_func("_ripen_from_the_chat", |args: &[Py]| _ripen_from_the_chat(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("avoid".to_string(), op_add(&__data62(), &g__NOT_A_FRUIT()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    fn __data63() -> Py { py_tuple(vec![py_str("how much"), py_str("per person"), py_str("cheeseboard"), py_str("board"), py_str("recipe"), py_str("what can i make"), py_str("melt"), py_str("melted"), py_str("stringy"), py_str("grate"), py_str("grated"), py_str("good for you"), py_str("calories"), py_str("healthy"), py_str("freeze"), py_str("freezer"), py_str("slimy"), py_str("gone hard"), py_str("gone dry"), py_str("dried out")]) }
    fn __data64() -> Py { py_tuple(vec![py_str("make"), py_str("making"), py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("cooking"), py_str("how do i cook"), py_str("simmer"), py_str("simmering"), py_str("simmered"), py_str("boil"), py_str("boiling"), py_str("fried"), py_str("fry"), py_str("fries"), py_str("frying"), py_str("hob"), py_str("pan"), py_str("stir"), py_str("minutes"), py_str("how many minutes"), py_str("tinned tomatoes"), py_str("chopped tomatoes"), py_str("tin of tomatoes"), py_str("how do i make"), py_str("lasagne"), py_str("lasagna"), py_str("pizza"), py_str("pasta bake"), py_str("what can i make"), py_str("what can i do"), py_str("serve"), py_str("serving"), py_str("saucepan"), py_str("ketchup"), py_str("soy sauce"), py_str("hot sauce"), py_str("worcestershire"), py_str("apple sauce"), py_str("cranberry sauce"), py_str("mint sauce"), py_str("bbq sauce"), py_str("barbecue sauce"), py_str("how much"), py_str("per person"), py_str("portion"), py_str("portions"), py_str("bolognese"), py_str("left out"), py_str("leave it out"), py_str("leaving it out"), py_str("left on the side"), py_str("stay out"), py_str("stays out"), py_str("sitting out"), py_str("out of the fridge"), py_str("on the counter"), py_str("overnight")]) }
    fn __data65() -> Py { py_tuple(vec![py_str("mould"), py_str("mouldy"), py_str("mold"), py_str("moldy"), py_str("fluff"), py_str("fluffy"), py_str("furry"), py_str("fur"), py_str("fuzz"), py_str("fuzzy"), py_str("spots"), py_str("speckled"), py_str("bloom"), py_str("white"), py_str("green"), py_str("blue"), py_str("black"), py_str("bits")]) }
    fn __data66() -> Py { py_tuple(vec![py_str("lid is"), py_str("lid has"), py_str("peanut butter"), py_str("bread"), py_str("loaf"), py_str("cheese"), py_str("meat"), py_str("ham"), py_str("yoghurt"), py_str("jam roly"), py_str("doughnut"), py_str("sponge"), py_str("cake"), py_str("tart"), py_str("recipe")]) }
    let __aug67 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cheese-store-by-name")); __args.push(py_tuple(vec![g_CHEESE_NAMES(), g_STORE_ASKS()])); __args.push(py_str("Hard cheese keeps about 2 weeks in the fridge once it is opened and soft cheese about a week. Wrap it in paper rather than cling film, which sweats it, and keep it in the drawer at the bottom rather than the door.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_STORE_ASKS()), ("avoid".to_string(), op_add(&__data63(), &g_MOULD_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cheese-mould-by-name")); __args.push(py_tuple(vec![op_add(&op_add(&g_CHEESE_NAMES(), &g_SOFT_CHEESES()), &g_BLUE_CHEESES()), op_add(&g_MOULD_WORDS(), &py_tuple(vec![py_str("eat"), py_str("safe"), py_str("ok"), py_str("cut"), py_str("around it"), py_str("bin"), py_str("throw")]))])); __args.push(py_func("_cheese_mould", |args: &[Py]| _cheese_mould(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), g_MOULD_WORDS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pasta-sauce-keep")); __args.push(py_tuple(vec![g_PASTA_SAUCE_WORDS(), g_SAUCE_KEEPING_WORDS()])); __args.push(py_func("_pasta_sauce_keep", |args: &[Py]| _pasta_sauce_keep(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_SAUCE_KEEPING_WORDS()), ("avoid".to_string(), __data64())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("storecupboard-keep")); __args.push(py_tuple(vec![g_SC_KEEP_NAMES(), g_SC_KEEP_ASKS()])); __args.push(py_func("_storecupboard_keep", |args: &[Py]| _storecupboard_keep(args))); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("ask".to_string(), g_SC_KEEP_ASKS()), ("avoid".to_string(), g_SC_KEEP_AVOID())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("jam-mould")); __args.push(py_tuple(vec![py_tuple(vec![py_str("jam"), py_str("marmalade"), py_str("preserve"), py_str("preserves"), py_str("chutney"), py_str("chutneys"), py_str("jam jar"), py_str("jam jars"), py_str("jar of jam"), py_str("marmalade jar"), py_str("kilner jar")]), __data65()])); __args.push(py_str("Mould on the top of a jam lifts off: take a spoon and the patch plus a good centimetre around it, and the rest is fine if it smells and tastes normal. Bin the jar instead if it is a low-sugar or homemade jam, if the mould runs down the side of the jar, or if it smells fermented - and never spoon it back in.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), __data66())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ENTRIES"); }
    s_ENTRIES(__aug67);
    fn __data68() -> Py { py_tuple(vec![py_str("crisps"), py_str("crisp"), py_str("potato crisps"), py_str("biscuits"), py_str("biscuit"), py_str("crackers"), py_str("cracker"), py_str("cereal"), py_str("cereals"), py_str("cornflakes"), py_str("muesli"), py_str("granola"), py_str("nuts"), py_str("crispbread"), py_str("pretzels"), py_str("tortilla chips"), py_str("breadsticks"), py_str("bread sticks"), py_str("oatcakes"), py_str("corn chips"), py_str("rice cakes"), py_str("pork scratchings"), py_str("peanuts"), py_str("cashews"), py_str("almonds"), py_str("walnuts")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_FOODS"); }
    s__CRUNCH_FOODS(__data68());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_STATE"); }
    s__CRUNCH_STATE(py_tuple(vec![py_str("gone soft"), py_str("soft"), py_str("gone stale"), py_str("stale"), py_str("chewy"), py_str("soggy"), py_str("damp"), py_str("rubbery"), py_str("lost its crunch"), py_str("not crunchy"), py_str("no crunch")]));
    fn __data69() -> Py { py_tuple(vec![py_str("crisps"), py_str("crisp"), py_str("potato crisps"), py_str("crackers"), py_str("cracker"), py_str("crispbread"), py_str("pretzels"), py_str("tortilla chips"), py_str("breadsticks"), py_str("bread sticks"), py_str("oatcakes"), py_str("corn chips"), py_str("rice cakes"), py_str("pork scratchings")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_OWN"); }
    s__CRUNCH_OWN(__data69());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_VERDICT"); }
    s__CRUNCH_VERDICT(py_str("Still fine to eat: soft is damp, not spoilage, and heat takes the damp back out. Bin them instead if there is mould on any of them, or if they smell of old paint or crayons, which is the oil in them going rancid."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_FIX"); }
    s__CRUNCH_FIX(py_str("Soft, not spoilt: they have taken damp out of the air. Spread them on a tray and give them 4 to 5 minutes at 150C, then leave them to cool - they crisp as they cool, not in the oven. Popped corn and nuts take the same and 3 minutes is enough; a biscuit that has gone chewy comes back the same way."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_KEEP"); }
    s__CRUNCH_KEEP(py_str("To keep the crunch, tip them into a jar or a tin with a tight lid rather than leaving them in the bag, which breathes, and keep them in a cupboard rather than the fridge, which is damp. A piece of kitchen paper in the box takes up the damp the same way."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._YOGHURT_WHEY"); }
    s__YOGHURT_WHEY(py_str("That clear liquid is whey, and it belongs to the yogurt: stir it back in. Yogurt that has separated into curds and watery liquid and smells sharp, fizzy or of alcohol is the one to bin."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._YOGHURT_FIX"); }
    s__YOGHURT_FIX(py_str("Yogurt separates as the whey rises, and spooning from the same spot makes it happen sooner: take from the side and stir the tub before you serve it. Plain yogurt keeps about a week past the date once it is open, and stirring is all a watery tub needs; a sharp or fizzy smell is when to bin it."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._SALT_CLUMPS"); }
    s__SALT_CLUMPS(py_str("Clumped salt has taken damp, not gone off: break it up with the back of a spoon and it still tastes of salt and works the same in cooking. A few grains of rice or a piece of dry cracker in the jar keeps it running, and a lidded jar in a dry cupboard keeps it clump-free where the paper packet does not."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._FLOUR_LUMPS"); }
    s__FLOUR_LUMPS(py_str("Lumps in flour are damp, not spoilage: sift it and it is fine to bake with. If it also smells musty or sour, or the lumps are moving, bin the bag - that is mites - wipe the shelf out and keep the next bag in a sealed jar."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._FLOUR_MITES"); }
    s__FLOUR_MITES(py_str("Those are flour mites or weevils and that bag has to go: seal it and bin it, then go through the dry cupboard - rice, oats, cereal, pasta, dried fruit - and bin anything open that has them in. Wipe the shelf out, and keep flour in a sealed jar from then on. They are not a health risk once cooked, but nothing worth keeping is left in that bag."));
    fn __data70() -> Py { py_tuple(vec![py_str("safe"), py_str("still eat"), py_str("still ok"), py_str("still good"), py_str("can i eat"), py_str("ok to eat"), py_str("edible"), py_str("bin"), py_str("throw"), py_str("off"), py_str("bad"), py_str("risky"), py_str("ill")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._CRUNCH_SAFE"); }
    s__CRUNCH_SAFE(__data70());
    fn __data71() -> Py { py_tuple(vec![py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("how do you make"), py_str("make my own"), py_str("homemade"), py_str("buy"), py_str("buying"), py_str("shop"), py_str("supermarket"), py_str("brand"), py_str("brands"), py_str("best"), py_str("cheapest"), py_str("where can i"), py_str("calories")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._STATE_NOT_FOOD"); }
    s__STATE_NOT_FOOD(op_add(&g_KEEP_NOT_KEEPING(), &__data71()));
    fn __data72() -> Py { py_tuple(vec![py_str("dip"), py_str("dips"), py_str("recipe"), py_str("chocolate"), py_str("cake"), py_str("bread"), py_str("loaf"), py_str("sandwich"), py_str("butter"), py_str("roast"), py_str("salted nuts"), py_str("nut allergy"), py_str("allergic"), py_str("peanut butter")]) }
    fn __data73() -> Py { py_tuple(vec![py_str("water on top"), py_str("water at the top"), py_str("water in it"), py_str("watery"), py_str("runny"), py_str("separated"), py_str("separating"), py_str("split"), py_str("whey"), py_str("liquid on top"), py_str("liquid in it"), py_str("curdled"), py_str("water out"), py_str("get the water out"), py_str("drain"), py_str("drain it"), py_str("sieve"), py_str("strain")]) }
    fn __data74() -> Py { py_tuple(vec![py_str("water"), py_str("watery"), py_str("runny"), py_str("separated"), py_str("split"), py_str("whey"), py_str("liquid"), py_str("curdled"), py_str("keep"), py_str("stop"), py_str("drain"), py_str("strain"), py_str("sieve")]) }
    fn __data75() -> Py { py_tuple(vec![py_str("bath"), py_str("bath salts"), py_str("epsom"), py_str("road"), py_str("grit"), py_str("lamp"), py_str("salt water"), py_str("sea water"), py_str("gargle"), py_str("blood pressure"), py_str("recipe"), py_str("bread"), py_str("dough"), py_str("brine"), py_str("rub"), py_str("seasoning"), py_str("how much salt")]) }
    fn __data76() -> Py { py_tuple(vec![py_str("lumpy"), py_str("lumps"), py_str("clumpy"), py_str("clumps"), py_str("weevils"), py_str("weevil"), py_str("mites"), py_str("mite"), py_str("bugs"), py_str("worms"), py_str("moving"), py_str("damp"), py_str("sour")]) }
    fn __data77() -> Py { py_tuple(vec![py_str("lumpy"), py_str("lumps"), py_str("clumpy"), py_str("clumps"), py_str("weevils"), py_str("weevil"), py_str("mites"), py_str("mite"), py_str("bugs"), py_str("worms"), py_str("moving"), py_str("damp"), py_str("sour")]) }
    let __aug78 = op_add(&g_ENTRIES(), &py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-crunch-gone")); __args.push(py_tuple(vec![g__CRUNCH_FOODS(), g__CRUNCH_STATE()])); __args.push(py_func("_crunch_reply", |args: &[Py]| _crunch_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("soft"), py_str("stale"), py_str("chewy"), py_str("soggy"), py_str("damp"), py_str("rubbery"), py_str("crunch"), py_str("crunchy"), py_str("gone"), py_str("lost")])), ("avoid".to_string(), op_add(&g__STATE_NOT_FOOD(), &__data72()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-crunch-fresh")); __args.push(py_tuple(vec![g__CRUNCH_OWN(), py_tuple(vec![py_str("fresh"), py_str("freshness"), py_str("stay fresh"), py_str("keep them fresh"), py_str("keeps them fresh")])])); __args.push(op_add(&op_add(&g__CRUNCH_KEEP(), &py_str(" ")), &g__CRUNCH_FIX())); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("fresh"), py_str("freshness")])), ("avoid".to_string(), op_add(&g__STATE_NOT_FOOD(), &py_tuple(vec![py_str("dip"), py_str("dips"), py_str("chocolate"), py_str("cake"), py_str("sandwich"), py_str("butter"), py_str("roast"), py_str("salted nuts"), py_str("nut allergy"), py_str("allergic")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-yoghurt-whey")); __args.push(py_tuple(vec![py_tuple(vec![py_str("yogurt"), py_str("yoghurt"), py_str("yogurts"), py_str("yoghurts"), py_str("greek yogurt"), py_str("greek yoghurt"), py_str("natural yogurt"), py_str("natural yoghurt"), py_str("plain yogurt"), py_str("plain yoghurt")]), __data73()])); __args.push(py_func("_yoghurt_reply", |args: &[Py]| _yoghurt_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), __data74()), ("avoid".to_string(), op_add(&g__STATE_NOT_FOOD(), &py_tuple(vec![py_str("starter"), py_str("culture"), py_str("make my own"), py_str("how do i make")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-salt-clumps")); __args.push(py_tuple(vec![py_tuple(vec![py_str("salt"), py_str("sea salt"), py_str("table salt"), py_str("rock salt"), py_str("kosher salt"), py_str("salt cellar"), py_str("salt shaker"), py_str("salt jar")]), py_tuple(vec![py_str("clumpy"), py_str("clumped"), py_str("lumpy"), py_str("stuck together"), py_str("caked"), py_str("caking"), py_str("gone hard"), py_str("hard"), py_str("damp"), py_str("block"), py_str("solid")])])); __args.push(g__SALT_CLUMPS()); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("clumpy"), py_str("clumped"), py_str("lumpy"), py_str("stuck"), py_str("caked"), py_str("caking"), py_str("hard"), py_str("damp"), py_str("block"), py_str("solid")])), ("avoid".to_string(), op_add(&g_KEEP_NOT_KEEPING(), &__data75()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("keep-flour-lumps")); __args.push(py_tuple(vec![py_tuple(vec![py_str("flour"), py_str("plain flour"), py_str("self raising flour"), py_str("self-raising flour"), py_str("bread flour"), py_str("wholemeal flour"), py_str("white flour")]), __data76()])); __args.push(py_func("_flour_reply", |args: &[Py]| _flour_reply(args))); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), __data77()), ("avoid".to_string(), op_add(&g_KEEP_NOT_KEEPING(), &py_tuple(vec![py_str("bread"), py_str("loaf"), py_str("cake"), py_str("recipe"), py_str("recipes"), py_str("how do i make"), py_str("bake"), py_str("baking"), py_str("sauce"), py_str("thicken"), py_str("roux"), py_str("how much")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.ENTRIES"); }
    s_ENTRIES(__aug78);
    fn __data79() -> Py { py_tuple(vec![py_str("strawberry"), py_str("strawberries"), py_str("raspberry"), py_str("raspberries"), py_str("blueberry"), py_str("blueberries"), py_str("blackberry"), py_str("blackberries"), py_str("gooseberry"), py_str("gooseberries"), py_str("cherry"), py_str("cherries"), py_str("grapes"), py_str("grape"), py_str("berry"), py_str("berries"), py_str("punnet"), py_str("punnet of"), py_str("soft fruit"), py_str("stone fruit")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_BERRIES"); }
    s_OFF_BERRIES(__data79());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_LEAVES"); }
    s_OFF_LEAVES(py_tuple(vec![py_str("lettuce"), py_str("spinach"), py_str("salad leaves"), py_str("leaves"), py_str("rocket"), py_str("watercress"), py_str("kale"), py_str("cabbage"), py_str("bag of salad"), py_str("salad bag"), py_str("salad"), py_str("coleslaw mix")]));
    fn __data80() -> Py { py_tuple(vec![py_str("cucumber"), py_str("cucumbers"), py_str("courgette"), py_str("courgettes"), py_str("zucchini"), py_str("tomato"), py_str("tomatoes"), py_str("pepper"), py_str("peppers"), py_str("aubergine"), py_str("aubergines"), py_str("avocado"), py_str("avocados"), py_str("mango"), py_str("mangoes"), py_str("peach"), py_str("peaches"), py_str("nectarine"), py_str("nectarines"), py_str("plum"), py_str("plums"), py_str("fig"), py_str("figs"), py_str("melon"), py_str("melons"), py_str("squash"), py_str("pumpkin"), py_str("mushy fruit")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_SOFT"); }
    s_OFF_SOFT(__data80());
    fn __data81() -> Py { py_tuple(vec![py_str("orange"), py_str("oranges"), py_str("lemon"), py_str("lemons"), py_str("lime"), py_str("limes"), py_str("clementine"), py_str("clementines"), py_str("satsuma"), py_str("satsumas"), py_str("tangerine"), py_str("tangerines"), py_str("grapefruit"), py_str("mandarin"), py_str("mandarins")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_CITRUS"); }
    s_OFF_CITRUS(__data81());
    fn __data82() -> Py { py_tuple(vec![py_str("apple"), py_str("apples"), py_str("pear"), py_str("pears"), py_str("carrot"), py_str("carrots"), py_str("swede"), py_str("turnip"), py_str("beetroot"), py_str("parsnip"), py_str("parsnips"), py_str("ginger"), py_str("cabbage head"), py_str("cauliflower"), py_str("broccoli")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_FIRM"); }
    s_OFF_FIRM(__data82());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_ANY"); }
    s_OFF_ANY(py_tuple(vec![py_str("fruit"), py_str("fruits"), py_str("veg"), py_str("vegetables"), py_str("produce"), py_str("punnet"), py_str("the drawer"), py_str("salad drawer"), py_str("veg drawer"), py_str("crisper"), py_str("greengrocer"), py_str("the shopping")]));
    fn __data83() -> Py { py_tuple(vec![py_str("mould"), py_str("mouldy"), py_str("molds"), py_str("mold"), py_str("moldy"), py_str("fuzzy"), py_str("furry"), py_str("fuzz"), py_str("fur"), py_str("going mouldy"), py_str("gone mouldy"), py_str("mould on it"), py_str("mould on them"), py_str("fluff on"), py_str("fluffy"), py_str("slimy"), py_str("slime"), py_str("slimey"), py_str("slimier"), py_str("gone off"), py_str("gone bad"), py_str("rotting"), py_str("rotten"), py_str("rot"), py_str("gone rotten"), py_str("going off"), py_str("gone past it"), py_str("on the turn")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_WORDS"); }
    s_OFF_WORDS(__data83());
    fn __data84() -> Py { py_tuple(vec![py_str("plant"), py_str("plants"), py_str("on the plant"), py_str("growing"), py_str("grow"), py_str("grew"), py_str("garden"), py_str("gardener"), py_str("allotment"), py_str("crop"), py_str("crops"), py_str("vine"), py_str("vines"), py_str("greenhouse"), py_str("polytunnel"), py_str("tree"), py_str("soil"), py_str("compost"), py_str("mulch"), py_str("water"), py_str("watering"), py_str("watered"), py_str("harvest"), py_str("pick"), py_str("picking"), py_str("picked them"), py_str("seedling"), py_str("seedlings"), py_str("bush"), py_str("fertiliser"), py_str("fertilizer"), py_str("feed"), py_str("blossom"), py_str("blossom end"), py_str("slug"), py_str("slugs"), py_str("snail"), py_str("snails"), py_str("sow"), py_str("sowing"), py_str("sowed"), py_str("transplant"), py_str("prune"), py_str("pruning"), py_str("planted"), py_str("recipe"), py_str("recipes"), py_str("cook"), py_str("cooking"), py_str("bake"), py_str("baking"), py_str("juice"), py_str("juicing"), py_str("smoothie"), py_str("jam"), py_str("chutney"), py_str("pie"), py_str("crumble"), py_str("muffin"), py_str("muffins"), py_str("cake"), py_str("compost bin"), py_str("food waste"), py_str("bin day"), py_str("wheelie"), py_str("mildew"), py_str("powdery"), py_str("powder"), py_str("white powder"), py_str("white mould"), py_str("grey mould"), py_str("gray mould"), py_str("on the leaves"), py_str("on the fruit"), py_str("on the stem"), py_str("cooked"), py_str("cooking water"), py_str("baked"), py_str("bake"), py_str("baked potato"), py_str("jacket"), py_str("jacket potato"), py_str("boiled"), py_str("boiling"), py_str("roast"), py_str("roasted"), py_str("mashed"), py_str("mash"), py_str("chips"), py_str("fries"), py_str("fried"), py_str("soup"), py_str("stew"), py_str("curry"), py_str("pie"), py_str("tart"), py_str("casserole"), py_str("leftover"), py_str("leftovers"), py_str("reheat"), py_str("reheated"), py_str("microwave"), py_str("salad dressing"), py_str("dressed"), py_str("shop"), py_str("shops"), py_str("supermarket"), py_str("sold"), py_str("sell"), py_str("selling"), py_str("refund"), py_str("receipt"), py_str("money back"), py_str("take it back"), py_str("took it back"), py_str("return it"), py_str("complaint"), py_str("complain"), py_str("council"), py_str("use by"), py_str("best before"), py_str("label"), py_str("packet"), py_str("packaging"), py_str("delivery"), py_str("stop"), py_str("stops"), py_str("stopping"), py_str("prevent"), py_str("preventing"), py_str("how do i keep"), py_str("keep them"), py_str("keep it from"), py_str("keep them from"), py_str("last longer"), py_str("make them last"), py_str("make it last"), py_str("so they dont go"), py_str("so it doesnt go"), py_str("so they don't go"), py_str("so it doesn't go")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_AVOID"); }
    s_OFF_AVOID(__data84());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping.OFF_NAMES"); }
    s_OFF_NAMES(op_add(&op_add(&op_add(&op_add(&op_add(&g_OFF_BERRIES(), &g_OFF_LEAVES()), &g_OFF_CITRUS()), &g_OFF_SOFT()), &g_OFF_FIRM()), &g_OFF_ANY()));
    fn __data85() -> Py { py_tuple(vec![py_str("berry"), py_str("berries"), py_str("punnet"), py_str("punnet of"), py_str("soft fruit"), py_str("stone fruit"), py_str("fruit"), py_str("fruits"), py_str("veg"), py_str("vegetables"), py_str("produce"), py_str("the drawer"), py_str("salad drawer"), py_str("veg drawer"), py_str("crisper"), py_str("greengrocer"), py_str("the shopping"), py_str("mushy fruit"), py_str("leaves"), py_str("salad"), py_str("bag of salad"), py_str("salad bag"), py_str("salad leaves"), py_str("coleslaw mix")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.keeping._OFF_GROUP_WORDS"); }
    s__OFF_GROUP_WORDS(call_builtin("frozenset", &[__data85()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("produce-gone-off")); __args.push(py_tuple(vec![op_add(&op_add(&op_add(&op_add(&op_add(&g_OFF_BERRIES(), &g_OFF_LEAVES()), &g_OFF_CITRUS()), &g_OFF_SOFT()), &g_OFF_FIRM()), &g_OFF_ANY()), g_OFF_WORDS()])); __args.push(py_func("_produce_gone_off", |args: &[Py]| _produce_gone_off(args))); __kws.extend(vec![("bias".to_string(), Py::Int(4)), ("ask".to_string(), g_OFF_WORDS()), ("avoid".to_string(), g_OFF_AVOID()), ("guard".to_string(), py_func("_produce_is_named", |args: &[Py]| _produce_is_named(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
    let _ = mcall(&g_ENTRIES(), "append", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("freeze-sweet")); __args.push(py_tuple(vec![g_FREEZE_SWEET_NAMES(), g_FREEZE_WORDS()])); __args.push(py_func("_freeze_a_sweet", |args: &[Py]| _freeze_a_sweet(args))); __kws.extend(vec![("bias".to_string(), Py::Int(3)), ("ask".to_string(), g_FREEZE_SWEET_NAMES()), ("avoid".to_string(), g_FREEZE_SWEET_AVOID())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]);
}
pub fn _oats_kept(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The bowl for \"how long does porridge keep\", the bag for \"how long do oats last\".");
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("norm")])) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[key.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &py_str("cooked"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("made"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("porridge"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &py_str("oats"))) } } } } } }) {
    return g_OATS_COOKED();
    }
    return g_OATS_BAG();
    Py::None
}
pub fn _slimy_mushrooms(args: &[Py]) -> Py {
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
    let mut k: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("text")])) { let mut k: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[k.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__EATING_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return op_add(&op_add(&g__MUSH_VERDICT(), &py_str(" ")), &g__MUSH_FIX());
    }
    return op_add(&op_add(&g__MUSH_FIX(), &py_str(" ")), &g__MUSH_VERDICT());
    Py::None
}
pub fn _freeze_a_sweet(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The freezer answer for the sweet or the milk the line names.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_FREEZE_SWEETS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    names = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &name.clone()))); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "startswith", &[op_add(&name.clone(), &py_str(" "))]) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), reply.clone()]);
    }
    break;
    }
    }
    }
    return if py_truthy(&best.clone()) { geti(&best.clone(), &Py::Int(1)) } else { Py::None };
    Py::None
}
pub fn _pasta_sauce_keep(args: &[Py]) -> Py {
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
    let _ = py_str("Three sauces, three lengths: a bought jar, a chilled carton, and a batch you made.\n\n    A jar is the one people ask about most, so it leads; the batch and the freezer follow in the\n    same breath, because the same person usually wants both.\n    ");
    line = op_add(&op_add(&py_str(" "), &{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }), &py_str(" "));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" freeze"), py_str(" freezer"), py_str(" frozen"), py_str(" freezing")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Sauce freezes well: cool it quickly, then freeze it in meal-sized portions for about 3 months, and reheat it once until it is steaming hot. A jar of sauce keeps 5 days in the fridge once it is opened, and sauce you made keeps 3 to 4 days - so if there is more than you will use, the freezer is the better place for it.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" carton"), py_str(" cartons"), py_str(" tub"), py_str(" fresh pasta sauce"), py_str(" chilled sauce"), py_str(" fresh sauce")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("A fresh or chilled carton of pasta sauce keeps 3 to 4 days in the fridge once it is opened, and it is worth transferring it out of the carton into a tub with a lid. It freezes well in portions for about 3 months; reheat it once, until it is steaming hot, and bin it if it smells sour or has mould on the surface.");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str(" homemade"), py_str(" made it"), py_str(" made my own"), py_str(" made this"), py_str(" batch"), py_str(" i made"), py_str(" my own sauce"), py_str(" left over from"), py_str(" from scratch")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("Sauce you made keeps 3 to 4 days in the fridge in a lidded tub, or 3 months in the freezer in portions, and it is better for being cooled quickly and put away while it is still warm rather than left out. Reheat it once, all the way through, and bin it if it smells sour, fizzes or has mould on top.");
    }
    return py_str("A jar of pasta sauce keeps 5 days in the fridge once it is opened, with the lid back on, and until the date on the jar while it is still sealed: the label's use-by is for the sealed jar, not for the open one. Sauce you made yourself keeps 3 to 4 days, and either freezes for about 3 months in portions - reheat it once, all the way through.");
    Py::None
}
pub fn _storecupboard_keep(args: &[Py]) -> Py {
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
    let mut frozen: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The keeping answer for the cupboard food the line names.");
    said = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    frozen = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("freezer"), py_str("freeze"), py_str("frozen"), py_str("defrost"), py_str("thaw")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    best = Py::None;
    let __iter1 = py_iter_or_raise(&g_STORECUPBOARD_KEEP());
    for __item in __iter1 {
    row = __item.clone();
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&row.clone(), &Py::Int(0)), geti(&row.clone(), &Py::Int(1))]), 2, None);
    names = __unpk2[0].clone();
    reply = __unpk2[1].clone();
    if py_truthy(&{ let __b = frozen.clone(); if !py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[row.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    reply = geti(&row.clone(), &Py::Int(2));
    }
    let __iter3 = py_iter_or_raise(&names.clone());
    for __item in __iter3 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &name.clone()))); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "startswith", &[op_add(&name.clone(), &py_str(" "))]) } }) {
    if py_truthy(&{ let __b = Py::Bool(py_is(&best.clone(), &Py::None)); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &geti(&best.clone(), &Py::Int(0))) } }) {
    best = py_tuple(vec![call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), reply.clone()]);
    }
    break;
    }
    }
    }
    if py_truthy(&best.clone()) {
    return geti(&best.clone(), &Py::Int(1));
    }
    return py_str("Most of these keep far longer than the date on the packet: soy sauce and vinegar for years, tea for about six months past its date, and an opened jar of peanut butter for about three months. The date is a quality date on most of them, not a safety one.");
    Py::None
}
pub fn _kept_a_while(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut reading: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let _ = py_str("True when the line says the thing has already been kept somewhere for a length of time.\n\n    \"is it ok to eat rice thats been in the fridge 3 days\" reached this row on \"rice\" and \"ok to\n    eat\", which is the date on the pack. The date is for the unopened pack; a thing that has been\n    in the fridge three days is the cooked-rice rule, said by the keeping row for that food.\n    ");
    return call_builtin("bool", &[mcall(&reading.clone(), "get", &[py_str("kept")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _dated_row(args: &[Py]) -> Py {
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
    let mut asks: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&row.clone(), &Py::Int(0)), geti(&row.clone(), &Py::Int(1))]), 2, None);
    names = __unpk1[0].clone();
    reply = __unpk1[1].clone();
    asks = op_add(&g_DATE_ASK(), &call_builtin("tuple", &[if py_truthy(&op_gt(&call_builtin("len", &[row.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { geti(&row.clone(), &Py::Int(2)) } else { py_tuple(vec![]) }]).unwrap_or_else(|e| exc::raise(e)));
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_modulo(&py_str("keep-%s-dated"), &geti(&names.clone(), &Py::Int(0)))); __args.push(py_tuple(vec![names.clone(), asks.clone()])); __args.push(reply.clone()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), mcall(&g_DATED_STAND_DOWN(), "get", &[geti(&names.clone(), &Py::Int(0)), py_tuple(vec![])])), ("guard".to_string(), { let asks = asks.clone(); let names = names.clone(); let reply = reply.clone(); let row = row.clone(); py_func("lambda", move |args: &[Py]| { let asks = asks.clone(); let names = names.clone(); let reply = reply.clone(); let row = row.clone(); let reading: Py = __kwp(args, 0, "reading").unwrap_or(Py::None); Py::Bool(!py_truthy(&_kept_a_while(&[reading.clone()]))) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) };
    Py::None
}
pub fn _cut_out_for_is_an_aptitude(args: &[Py]) -> Py {
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
    let _ = py_str("True when \"cut out\" is about being suited to a job, so the cold-chain row stands down.");
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[mcall(&g_CUT_OUT_FOR(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_thaw_that_is_a_job(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks how long thawing something takes, so the food-state card stands down.");
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_TIMES_A_THAW(), "search", &[said.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_ASKS_HOW_LONG(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _cheese_mould(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The mould verdict for the cheese the line names, and the general one when it names none.");
    said = { let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } };
    words = op_modulo(&py_str(" %s "), &mcall(&said.clone(), "lower", &[]));
    let __iter1 = py_iter_or_raise(&g_BLUE_CHEESES());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &name.clone()))) {
    return op_modulo(&py_str("The blue in %s is meant to be there. Green or white fuzz on it is not, so a blue cheese that has grown it goes in the bin."), &name.clone());
    }
    }
    let __iter2 = py_iter_or_raise(&g_SOFT_CHEESES());
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &name.clone()))) {
    return op_modulo(&py_str("Mould on %s means the whole piece goes in the bin: the threads run all the way through a soft cheese, so there is nothing to cut back to."), &name.clone());
    }
    }
    return py_str("Cut the mould off hard cheese an inch past it and the rest is fine. Soft cheese, cream cheese and anything already grated or sliced goes in the bin, because the threads reach further than you can see.");
    Py::None
}
pub fn _ripen_lead(args: &[Py]) -> Py {
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
    let _ = py_str("Which sentence of the row the line is asking for.");
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TELL_ASKS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("test");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RIPEN_ASKS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("trick");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KEEP_ASKS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("keep");
    }
    return py_str("test");
    Py::None
}
pub fn _ripens(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut test: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "test") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: test"))) };
    let mut trick: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trick") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: trick"))) };
    let mut keep: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keep") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: keep"))) };
    let mut reply: Py = Py::None;
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut keep: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut test: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut trick: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut lead: Py = Py::None;
    let mut leans: Py = Py::None;
    let mut read: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    read = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    text = mcall(&call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    lead = _ripen_lead(&[text.clone()]);
    leans = mcall(&read.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&lead.clone(), &py_str("test"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TELL_ASKS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    leans = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&leans.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    if py_truthy(&leans.clone()) {
    lead = _ripen_lead(&[op_add(&op_add(&text.clone(), &py_str(" ")), &mcall(&call_builtin("str", &[leans.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))]);
    }
    }
    if py_truthy(&Py::Bool(py_eq(&lead.clone(), &py_str("keep")))) {
    return keep.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&lead.clone(), &py_str("trick")))) {
    return op_add(&op_add(&trick.clone(), &py_str(" ")), &test.clone());
    }
    return op_add(&op_add(&test.clone(), &py_str(" ")), &trick.clone());
    Py::None
}
    reply = {{ let keep = keep.clone(); let reply = reply.clone(); let test = test.clone(); let trick = trick.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![keep.clone(), reply.clone(), test.clone(), trick.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _ripen_haystack(args: &[Py]) -> Py {
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
    let mut leans: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut read: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The current line with the line it leans on and the last few turns behind it.");
    read = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    parts = py_list(vec![call_builtin("str", &[{ let __b = mcall(&read.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&read.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e))]);
    leans = mcall(&read.clone(), "get", &[py_str("leans_on")]);
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&parts.clone(), "append", &[call_builtin("str", &[{ let __b = mcall(&leans.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[leans.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&parts.clone(), "append", &[mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&leans.clone()) { let mut word: Py = __item.clone(); __out.push(call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])]);
    }
    }
    turns = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = turns.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(3)), &Py::None, &Py::None));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&turn.clone(), "get", &[py_str("user")]);
    if py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&parts.clone(), "append", &[said.clone()]);
    }
    }
    }
    return mcall(&mcall(&py_str(" "), "join", &[parts.clone()]), "lower", &[]);
    Py::None
}
pub fn _ripen_row_for(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut row: Py = Py::None;
    let _ = py_str("The table row whose fruit the text names, or None.");
    let __iter1 = py_iter_or_raise(&g_RIPEN_TABLE());
    for __item in __iter1 {
    row = __item.clone();
    names = geti(&row.clone(), &Py::Int(0));
    let __iter2 = py_iter_or_raise(&names.clone());
    for __item in __iter2 {
    name = __item.clone();
    if py_truthy(&stdlib_call("re", "search", &[op_modulo(&py_str("\\b%s\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), text.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return row.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _ripen_from_the_chat(args: &[Py]) -> Py {
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
    let mut row: Py = Py::None;
    let _ = py_str("\"will they ripen\" and \"how long until they are ready\": the fruit was named a line ago.");
    row = _ripen_row_for(&[_ripen_haystack(&[understood.clone(), memory.clone()])]);
    if py_truthy(&Py::Bool(!py_is(&row.clone(), &Py::None))) {
    return op_add(&op_add(&geti(&row.clone(), &Py::Int(2)), &py_str(" ")), &geti(&row.clone(), &Py::Int(1)));
    }
    return py_str("Most fruit ripens on the counter in 2 to 4 days, and a paper bag with a banana in it halves that; the fridge is what stops it, so leave them out until they give. Once they give, they keep a few days in the fridge.");
    Py::None
}
pub fn _said_anything(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut k: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("text")])) { let mut k: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[k.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _crunch_reply(args: &[Py]) -> Py {
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
    if py_truthy(&_said_anything(&[understood.clone(), g__CRUNCH_SAFE()])) {
    return op_add(&op_add(&g__CRUNCH_VERDICT(), &py_str(" ")), &g__CRUNCH_FIX());
    }
    return op_add(&op_add(&g__CRUNCH_FIX(), &py_str(" ")), &g__CRUNCH_VERDICT());
    Py::None
}
pub fn _yoghurt_reply(args: &[Py]) -> Py {
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
    if py_truthy(&_said_anything(&[understood.clone(), py_tuple(vec![py_str("keep"), py_str("keeping"), py_str("stop"), py_str("runny"), py_str("watery"), py_str("separate")])])) {
    return op_add(&op_add(&g__YOGHURT_FIX(), &py_str(" ")), &g__YOGHURT_WHEY());
    }
    return g__YOGHURT_WHEY();
    Py::None
}
pub fn _flour_reply(args: &[Py]) -> Py {
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
    if py_truthy(&_said_anything(&[understood.clone(), py_tuple(vec![py_str("weevil"), py_str("weevils"), py_str("mite"), py_str("mites"), py_str("bug"), py_str("bugs"), py_str("worm"), py_str("worms"), py_str("moving"), py_str("insects"), py_str("maggots")])])) {
    return g__FLOUR_MITES();
    }
    return g__FLOUR_LUMPS();
    Py::None
}
pub fn _produce_is_named(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line names the fruit, the vegetable or the punnet that has turned.\n\n    A mould word alone is another row's line: \"the shop sold me mouldy bread\" is the refund and\n    \"there is mould in the bathroom\" is the cleaning row, and neither names produce. Cooked words\n    stand this row down in the `avoid` list rather than here, so the number of things this has to\n    know stays small.\n    ");
    low = op_modulo(&py_str(" %s "), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OFF_NAMES()) { let mut name: Py = __item.clone(); __out.push(Py::Bool(py_contains(&low.clone(), &op_modulo(&py_str(" %s "), &name.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _off_one(args: &[Py]) -> Py {
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
    let mut ending: Py = Py::None;
    let _ = py_str("The singular of a produce name, for the sentence that names it.");
    if py_truthy(&mcall(&name.clone(), "endswith", &[py_str("ies")])) {
    return op_add(&slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(3)), &Py::None), &py_str("y"));
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("oes"), py_str("ches"), py_str("shes"), py_str("sses")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&mcall(&name.clone(), "endswith", &[ending.clone()])) {
    return slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(2)), &Py::None);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("leaves"), py_str("leafes")]), &name.clone()))) {
    return py_str("leaf");
    }
    if py_truthy(&{ let __b = mcall(&name.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&name.clone(), "endswith", &[py_str("ss")]))) } }) {
    return slice_or_raise(&name.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    }
    return name.clone();
    Py::None
}
pub fn _produce_gone_off(args: &[Py]) -> Py {
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
    let mut berries: Py = Py::None;
    let mut citrus: Py = Py::None;
    let mut firm: Py = Py::None;
    let mut leaves: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let mut soft: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The mould verdict for the produce the line names, in the kitchen rather than the garden.");
    said = op_modulo(&py_str(" %s "), &mcall(&{ let __b = mcall(&{ let __b = reading.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]));
pub fn __nested_thing_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut berries: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut citrus: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut firm: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut leaves: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut phrase: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut reading: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut soft: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut thing: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut group: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "group").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut categories: Py = __pos.get(11).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "categories").map(|(_, v)| v.clone())).unwrap_or_else(|| Py::Bool(false));
    let mut hit: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The one produce name in the line from this group, or None.\n\n        The longest form wins so \"salad leaves\" is not read as \"salad\", and a category word\n        (\"berries\", \"punnet\", \"produce\") gives None, which keeps the generic sentence.\n\n        A branch that never names the thing it is talking about can take a category word, so the\n        leaves branch does: \"salad leaves gone slimy\" was read as \"salad\", a category, and\n        answered with the fuzz sentence, while \"lettuce gone slimy\" got the slime sentence. Say\n        so with categories=True and the rule is the branch's own.\n        ");
    hit = Py::None;
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(group.clone()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &name.clone()))); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "strip", &[]), "startswith", &[op_add(&name.clone(), &py_str(" "))]) } }) {
    if py_truthy(&Py::Bool(py_contains(&g__OFF_GROUP_WORDS(), &name.clone()))) {
    return if py_truthy(&categories.clone()) { name.clone() } else { Py::None };
    }
    hit = name.clone();
    break;
    }
    }
    return hit.clone();
    Py::None
}
    thing = {{ let berries = berries.clone(); let citrus = citrus.clone(); let firm = firm.clone(); let leaves = leaves.clone(); let memory = memory.clone(); let phrase = phrase.clone(); let reading = reading.clone(); let said = said.clone(); let soft = soft.clone(); let thing = thing.clone(); py_func("thing", move |args: &[Py]| { let mut full = vec![berries.clone(), citrus.clone(), firm.clone(), leaves.clone(), memory.clone(), phrase.clone(), reading.clone(), said.clone(), soft.clone(), thing.clone()]; full.extend_from_slice(args); __nested_thing_0(&full) }) }};
pub fn __nested_phrase_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut berries: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut citrus: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut firm: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut leaves: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut phrase: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut reading: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut said: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut soft: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut thing: Py = __pos.get(9).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "name").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut one: Py = Py::None;
    let _ = py_str("(\"the tomatoes\", True) for a plural, (\"the melon\", False) for one.");
    if py_truthy(&Py::Bool(py_is(&name.clone(), &Py::None))) {
    return py_tuple(vec![Py::None, Py::Bool(false)]);
    }
    one = _off_one(&[name.clone()]);
    return py_tuple(vec![op_add(&py_str("the "), &name.clone()), Py::Bool(!py_eq(&one.clone(), &name.clone()))]);
    Py::None
}
    phrase = {{ let berries = berries.clone(); let citrus = citrus.clone(); let firm = firm.clone(); let leaves = leaves.clone(); let memory = memory.clone(); let phrase = phrase.clone(); let reading = reading.clone(); let said = said.clone(); let soft = soft.clone(); let thing = thing.clone(); py_func("phrase", move |args: &[Py]| { let mut full = vec![berries.clone(), citrus.clone(), firm.clone(), leaves.clone(), memory.clone(), phrase.clone(), reading.clone(), said.clone(), soft.clone(), thing.clone()]; full.extend_from_slice(args); __nested_phrase_1(&full) }) }};
    berries = py_call(&phrase, &[py_call(&thing, &[g_OFF_BERRIES()])]);
    leaves = py_call(&phrase, &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_OFF_LEAVES()); __kws.extend(vec![("categories".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&thing, args))(&__args) }]);
    citrus = py_call(&phrase, &[py_call(&thing, &[g_OFF_CITRUS()])]);
    soft = py_call(&phrase, &[py_call(&thing, &[g_OFF_SOFT()])]);
    firm = py_call(&phrase, &[py_call(&thing, &[g_OFF_FIRM()])]);
    if py_truthy(&geti(&berries.clone(), &Py::Int(0))) {
    return op_modulo(&py_str("Bin every %s with fuzz on it and the ones touching it, then wash the rest and eat them today. On a berry the mould threads run under the skin where you cannot see them, so there is nothing to cut back to, and a closed punnet passes it on fast. Spread what is sound on kitchen paper in the fridge and it will hold a day or two."), &_off_one(&[slice_or_raise(&geti(&berries.clone(), &Py::Int(0)), &Py::Int(4), &Py::None, &Py::None)]));
    }
    if py_truthy(&geti(&leaves.clone(), &Py::Int(0))) {
    return py_str("Pick out the slimy or furry leaves and bin them: that slime is bacteria, not water, and cooking will not make it safe to eat. Wash the rest well and eat or cook them today, because in a closed bag it spreads to whatever touches it. Dry the next lot in a tea towel and keep them in a bag with a sheet of kitchen paper, because wet leaves in a sealed bag are what went wrong in the first place.");
    }
    if py_truthy(&geti(&citrus.clone(), &Py::Int(0))) {
    return op_modulo(&py_str("Bin %s and check the rest of the bag. Mould on citrus starts on the skin and the flesh under it is soft and wet, so there is nothing worth cutting back to - and dry the drawer and the bag, because a damp one passes it on. The sound ones keep a couple of weeks loose in the fridge."), &geti(&citrus.clone(), &Py::Int(0)));
    }
    if py_truthy(&geti(&soft.clone(), &Py::Int(0))) {
    return op_modulo(&py_str("Bin %s. %s mostly water, so the mould threads run right through %s and there is nothing to cut back to, and cooking does not make it safe again. A soft patch with no mould on it is just older - trim that away and cook the rest today."), &py_tuple(vec![geti(&soft.clone(), &Py::Int(0)), if py_truthy(&geti(&soft.clone(), &Py::Int(1))) { py_str("They are") } else { py_str("It is") }, if py_truthy(&geti(&soft.clone(), &Py::Int(1))) { py_str("them") } else { py_str("it") }]));
    }
    if py_truthy(&geti(&firm.clone(), &Py::Int(0))) {
    return op_modulo(&py_str("Cut the mould off with an inch to spare and use the rest, unless the flesh under it is soft or smells wrong. A firm fruit or root does not carry the threads deep, so a small patch on an apple or a carrot is not the whole of it - and bin %s, because it spreads to what it touches in a drawer."), &geti(&firm.clone(), &Py::Int(0)));
    }
    return py_str("Anything soft and wet - a berry, a tomato, a leaf - goes in the bin once it has fuzz on it, and the sound ones it touched are washed and eaten today. A firm fruit or root can have the patch cut away with an inch to spare, unless the flesh underneath is soft or smells off.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.keeping".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASKS_HOW_LONG" => Some(g_ASKS_HOW_LONG()),
            "BLUE_CHEESES" => Some(g_BLUE_CHEESES()),
            "CALENDAR_DATE" => Some(g_CALENDAR_DATE()),
            "CHEESE_NAMES" => Some(g_CHEESE_NAMES()),
            "COLD_NOT" => Some(g_COLD_NOT()),
            "COLD_REPLIES" => Some(g_COLD_REPLIES()),
            "COOKED_IN_THE_ROW" => Some(g_COOKED_IN_THE_ROW()),
            "COOKED_VEG_COOK_WORDS" => Some(g_COOKED_VEG_COOK_WORDS()),
            "COOKED_VEG_NAMES" => Some(g_COOKED_VEG_NAMES()),
            "COOKED_VEG_REPLY" => Some(g_COOKED_VEG_REPLY()),
            "COOKED_VEG_STAND_DOWN" => Some(g_COOKED_VEG_STAND_DOWN()),
            "COOKED_WORDS" => Some(g_COOKED_WORDS()),
            "COOKING_WORDS" => Some(g_COOKING_WORDS()),
            "COOK_LINE_WORDS" => Some(g_COOK_LINE_WORDS()),
            "CUT_OUT_FOR" => Some(g_CUT_OUT_FOR()),
            "CUT_WORDS" => Some(g_CUT_WORDS()),
            "DATED_STAND_DOWN" => Some(g_DATED_STAND_DOWN()),
            "DATES_OWN" => Some(g_DATES_OWN()),
            "DATE_ASK" => Some(g_DATE_ASK()),
            "DATE_ROWS" => Some(g_DATE_ROWS()),
            "DATING_DATE" => Some(g_DATING_DATE()),
            "DISH_ASK" => Some(g_DISH_ASK()),
            "DISH_WORDS" => Some(g_DISH_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "FEEDING_WORDS" => Some(g_FEEDING_WORDS()),
            "FIX_FIRST" => Some(g_FIX_FIRST()),
            "FLOP_WORDS" => Some(g_FLOP_WORDS()),
            "FOLLOW_UP_WORDS" => Some(g_FOLLOW_UP_WORDS()),
            "FREEZE_SWEETS" => Some(g_FREEZE_SWEETS()),
            "FREEZE_SWEET_AVOID" => Some(g_FREEZE_SWEET_AVOID()),
            "FREEZE_SWEET_NAMES" => Some(g_FREEZE_SWEET_NAMES()),
            "FREEZE_WORDS" => Some(g_FREEZE_WORDS()),
            "GROWING_WORDS" => Some(g_GROWING_WORDS()),
            "KEEPING_KEY" => Some(g_KEEPING_KEY()),
            "KEEP_ASKS" => Some(g_KEEP_ASKS()),
            "KEEP_NOT_KEEPING" => Some(g_KEEP_NOT_KEEPING()),
            "KEEP_NOT_MEDICINE" => Some(g_KEEP_NOT_MEDICINE()),
            "KEEP_NOT_SLEEP" => Some(g_KEEP_NOT_SLEEP()),
            "KEEP_QUALITY_WORDS" => Some(g_KEEP_QUALITY_WORDS()),
            "KEEP_TECHNIQUE" => Some(g_KEEP_TECHNIQUE()),
            "KEEP_WORDS" => Some(g_KEEP_WORDS()),
            "LONG_WORDS" => Some(g_LONG_WORDS()),
            "MOULD_WORDS" => Some(g_MOULD_WORDS()),
            "OATS_BAG" => Some(g_OATS_BAG()),
            "OATS_COOKED" => Some(g_OATS_COOKED()),
            "OFF_ANY" => Some(g_OFF_ANY()),
            "OFF_AVOID" => Some(g_OFF_AVOID()),
            "OFF_BERRIES" => Some(g_OFF_BERRIES()),
            "OFF_CITRUS" => Some(g_OFF_CITRUS()),
            "OFF_FIRM" => Some(g_OFF_FIRM()),
            "OFF_LEAVES" => Some(g_OFF_LEAVES()),
            "OFF_NAMES" => Some(g_OFF_NAMES()),
            "OFF_SOFT" => Some(g_OFF_SOFT()),
            "OFF_WORDS" => Some(g_OFF_WORDS()),
            "PACK_DATE" => Some(g_PACK_DATE()),
            "PASTA_SAUCE_WORDS" => Some(g_PASTA_SAUCE_WORDS()),
            "PROBLEM_ONLY" => Some(g_PROBLEM_ONLY()),
            "REFREEZE_WORDS" => Some(g_REFREEZE_WORDS()),
            "RIPEN_ASIDE" => Some(g_RIPEN_ASIDE()),
            "RIPEN_ASKS" => Some(g_RIPEN_ASKS()),
            "RIPEN_TABLE" => Some(g_RIPEN_TABLE()),
            "RIPEN_WORDS" => Some(g_RIPEN_WORDS()),
            "ROWS" => Some(g_ROWS()),
            "ROW_COOK_FORMS" => Some(g_ROW_COOK_FORMS()),
            "ROW_STAND_DOWN" => Some(g_ROW_STAND_DOWN()),
            "SAUCE_KEEPING_WORDS" => Some(g_SAUCE_KEEPING_WORDS()),
            "SC_KEEP_ASKS" => Some(g_SC_KEEP_ASKS()),
            "SC_KEEP_AVOID" => Some(g_SC_KEEP_AVOID()),
            "SC_KEEP_NAMES" => Some(g_SC_KEEP_NAMES()),
            "SOFT_CHEESES" => Some(g_SOFT_CHEESES()),
            "STORECUPBOARD_KEEP" => Some(g_STORECUPBOARD_KEEP()),
            "STORE_ASKS" => Some(g_STORE_ASKS()),
            "TELL_ASKS" => Some(g_TELL_ASKS()),
            "THAW_WORDS" => Some(g_THAW_WORDS()),
            "TIMES_A_THAW" => Some(g_TIMES_A_THAW()),
            "UNDER_RIPE" => Some(g_UNDER_RIPE()),
            "UNRIPE_STATED" => Some(g_UNRIPE_STATED()),
            "USE_UP_WORDS" => Some(g_USE_UP_WORDS()),
            "WHEN_WORDS" => Some(g_WHEN_WORDS()),
            "YOGHURT" => Some(g_YOGHURT()),
            "_CLEANING_WITH_IT" => Some(g__CLEANING_WITH_IT()),
            "_CRUNCH_FIX" => Some(g__CRUNCH_FIX()),
            "_CRUNCH_FOODS" => Some(g__CRUNCH_FOODS()),
            "_CRUNCH_KEEP" => Some(g__CRUNCH_KEEP()),
            "_CRUNCH_OWN" => Some(g__CRUNCH_OWN()),
            "_CRUNCH_SAFE" => Some(g__CRUNCH_SAFE()),
            "_CRUNCH_STATE" => Some(g__CRUNCH_STATE()),
            "_CRUNCH_VERDICT" => Some(g__CRUNCH_VERDICT()),
            "_DAY_AFTER" => Some(g__DAY_AFTER()),
            "_DAY_AFTER_AVOID" => Some(g__DAY_AFTER_AVOID()),
            "_DAY_AFTER_ENTRY" => Some(g__DAY_AFTER_ENTRY()),
            "_DAY_AFTER_FOODS" => Some(g__DAY_AFTER_FOODS()),
            "_EATING_WORDS" => Some(g__EATING_WORDS()),
            "_FLOUR_LUMPS" => Some(g__FLOUR_LUMPS()),
            "_FLOUR_MITES" => Some(g__FLOUR_MITES()),
            "_LAST_TIME" => Some(g__LAST_TIME()),
            "_MUSH_FIX" => Some(g__MUSH_FIX()),
            "_MUSH_VERDICT" => Some(g__MUSH_VERDICT()),
            "_NOT_A_FRUIT" => Some(g__NOT_A_FRUIT()),
            "_OFF_GROUP_WORDS" => Some(g__OFF_GROUP_WORDS()),
            "_RIPEN_ASIDE_KEEPING" => Some(g__RIPEN_ASIDE_KEEPING()),
            "_SALT_CLUMPS" => Some(g__SALT_CLUMPS()),
            "_STAND_DOWN" => Some(g__STAND_DOWN()),
            "_STAND_DOWN_HARD" => Some(g__STAND_DOWN_HARD()),
            "_STATE_NOT_FOOD" => Some(g__STATE_NOT_FOOD()),
            "_YOGHURT_FIX" => Some(g__YOGHURT_FIX()),
            "_YOGHURT_WHEY" => Some(g__YOGHURT_WHEY()),
            "_a_thaw_that_is_a_job" => Some(py_func("_a_thaw_that_is_a_job", |args: &[Py]| _a_thaw_that_is_a_job(args))),
            "_cheese_mould" => Some(py_func("_cheese_mould", |args: &[Py]| _cheese_mould(args))),
            "_crunch_reply" => Some(py_func("_crunch_reply", |args: &[Py]| _crunch_reply(args))),
            "_cut_out_for_is_an_aptitude" => Some(py_func("_cut_out_for_is_an_aptitude", |args: &[Py]| _cut_out_for_is_an_aptitude(args))),
            "_dated_row" => Some(py_func("_dated_row", |args: &[Py]| _dated_row(args))),
            "_flour_reply" => Some(py_func("_flour_reply", |args: &[Py]| _flour_reply(args))),
            "_freeze_a_sweet" => Some(py_func("_freeze_a_sweet", |args: &[Py]| _freeze_a_sweet(args))),
            "_kept_a_while" => Some(py_func("_kept_a_while", |args: &[Py]| _kept_a_while(args))),
            "_oats_kept" => Some(py_func("_oats_kept", |args: &[Py]| _oats_kept(args))),
            "_off_one" => Some(py_func("_off_one", |args: &[Py]| _off_one(args))),
            "_pasta_sauce_keep" => Some(py_func("_pasta_sauce_keep", |args: &[Py]| _pasta_sauce_keep(args))),
            "_produce_gone_off" => Some(py_func("_produce_gone_off", |args: &[Py]| _produce_gone_off(args))),
            "_produce_is_named" => Some(py_func("_produce_is_named", |args: &[Py]| _produce_is_named(args))),
            "_ripen_from_the_chat" => Some(py_func("_ripen_from_the_chat", |args: &[Py]| _ripen_from_the_chat(args))),
            "_ripen_haystack" => Some(py_func("_ripen_haystack", |args: &[Py]| _ripen_haystack(args))),
            "_ripen_lead" => Some(py_func("_ripen_lead", |args: &[Py]| _ripen_lead(args))),
            "_ripen_row_for" => Some(py_func("_ripen_row_for", |args: &[Py]| _ripen_row_for(args))),
            "_ripens" => Some(py_func("_ripens", |args: &[Py]| _ripens(args))),
            "_said_anything" => Some(py_func("_said_anything", |args: &[Py]| _said_anything(args))),
            "_slimy_mushrooms" => Some(py_func("_slimy_mushrooms", |args: &[Py]| _slimy_mushrooms(args))),
            "_storecupboard_keep" => Some(py_func("_storecupboard_keep", |args: &[Py]| _storecupboard_keep(args))),
            "_yoghurt_reply" => Some(py_func("_yoghurt_reply", |args: &[Py]| _yoghurt_reply(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASKS_HOW_LONG" => s_ASKS_HOW_LONG(v),
            "BLUE_CHEESES" => s_BLUE_CHEESES(v),
            "CALENDAR_DATE" => s_CALENDAR_DATE(v),
            "CHEESE_NAMES" => s_CHEESE_NAMES(v),
            "COLD_NOT" => s_COLD_NOT(v),
            "COLD_REPLIES" => s_COLD_REPLIES(v),
            "COOKED_IN_THE_ROW" => s_COOKED_IN_THE_ROW(v),
            "COOKED_VEG_COOK_WORDS" => s_COOKED_VEG_COOK_WORDS(v),
            "COOKED_VEG_NAMES" => s_COOKED_VEG_NAMES(v),
            "COOKED_VEG_REPLY" => s_COOKED_VEG_REPLY(v),
            "COOKED_VEG_STAND_DOWN" => s_COOKED_VEG_STAND_DOWN(v),
            "COOKED_WORDS" => s_COOKED_WORDS(v),
            "COOKING_WORDS" => s_COOKING_WORDS(v),
            "COOK_LINE_WORDS" => s_COOK_LINE_WORDS(v),
            "CUT_OUT_FOR" => s_CUT_OUT_FOR(v),
            "CUT_WORDS" => s_CUT_WORDS(v),
            "DATED_STAND_DOWN" => s_DATED_STAND_DOWN(v),
            "DATES_OWN" => s_DATES_OWN(v),
            "DATE_ASK" => s_DATE_ASK(v),
            "DATE_ROWS" => s_DATE_ROWS(v),
            "DATING_DATE" => s_DATING_DATE(v),
            "DISH_ASK" => s_DISH_ASK(v),
            "DISH_WORDS" => s_DISH_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "FEEDING_WORDS" => s_FEEDING_WORDS(v),
            "FIX_FIRST" => s_FIX_FIRST(v),
            "FLOP_WORDS" => s_FLOP_WORDS(v),
            "FOLLOW_UP_WORDS" => s_FOLLOW_UP_WORDS(v),
            "FREEZE_SWEETS" => s_FREEZE_SWEETS(v),
            "FREEZE_SWEET_AVOID" => s_FREEZE_SWEET_AVOID(v),
            "FREEZE_SWEET_NAMES" => s_FREEZE_SWEET_NAMES(v),
            "FREEZE_WORDS" => s_FREEZE_WORDS(v),
            "GROWING_WORDS" => s_GROWING_WORDS(v),
            "KEEPING_KEY" => s_KEEPING_KEY(v),
            "KEEP_ASKS" => s_KEEP_ASKS(v),
            "KEEP_NOT_KEEPING" => s_KEEP_NOT_KEEPING(v),
            "KEEP_NOT_MEDICINE" => s_KEEP_NOT_MEDICINE(v),
            "KEEP_NOT_SLEEP" => s_KEEP_NOT_SLEEP(v),
            "KEEP_QUALITY_WORDS" => s_KEEP_QUALITY_WORDS(v),
            "KEEP_TECHNIQUE" => s_KEEP_TECHNIQUE(v),
            "KEEP_WORDS" => s_KEEP_WORDS(v),
            "LONG_WORDS" => s_LONG_WORDS(v),
            "MOULD_WORDS" => s_MOULD_WORDS(v),
            "OATS_BAG" => s_OATS_BAG(v),
            "OATS_COOKED" => s_OATS_COOKED(v),
            "OFF_ANY" => s_OFF_ANY(v),
            "OFF_AVOID" => s_OFF_AVOID(v),
            "OFF_BERRIES" => s_OFF_BERRIES(v),
            "OFF_CITRUS" => s_OFF_CITRUS(v),
            "OFF_FIRM" => s_OFF_FIRM(v),
            "OFF_LEAVES" => s_OFF_LEAVES(v),
            "OFF_NAMES" => s_OFF_NAMES(v),
            "OFF_SOFT" => s_OFF_SOFT(v),
            "OFF_WORDS" => s_OFF_WORDS(v),
            "PACK_DATE" => s_PACK_DATE(v),
            "PASTA_SAUCE_WORDS" => s_PASTA_SAUCE_WORDS(v),
            "PROBLEM_ONLY" => s_PROBLEM_ONLY(v),
            "REFREEZE_WORDS" => s_REFREEZE_WORDS(v),
            "RIPEN_ASIDE" => s_RIPEN_ASIDE(v),
            "RIPEN_ASKS" => s_RIPEN_ASKS(v),
            "RIPEN_TABLE" => s_RIPEN_TABLE(v),
            "RIPEN_WORDS" => s_RIPEN_WORDS(v),
            "ROWS" => s_ROWS(v),
            "ROW_COOK_FORMS" => s_ROW_COOK_FORMS(v),
            "ROW_STAND_DOWN" => s_ROW_STAND_DOWN(v),
            "SAUCE_KEEPING_WORDS" => s_SAUCE_KEEPING_WORDS(v),
            "SC_KEEP_ASKS" => s_SC_KEEP_ASKS(v),
            "SC_KEEP_AVOID" => s_SC_KEEP_AVOID(v),
            "SC_KEEP_NAMES" => s_SC_KEEP_NAMES(v),
            "SOFT_CHEESES" => s_SOFT_CHEESES(v),
            "STORECUPBOARD_KEEP" => s_STORECUPBOARD_KEEP(v),
            "STORE_ASKS" => s_STORE_ASKS(v),
            "TELL_ASKS" => s_TELL_ASKS(v),
            "THAW_WORDS" => s_THAW_WORDS(v),
            "TIMES_A_THAW" => s_TIMES_A_THAW(v),
            "UNDER_RIPE" => s_UNDER_RIPE(v),
            "UNRIPE_STATED" => s_UNRIPE_STATED(v),
            "USE_UP_WORDS" => s_USE_UP_WORDS(v),
            "WHEN_WORDS" => s_WHEN_WORDS(v),
            "YOGHURT" => s_YOGHURT(v),
            "_CLEANING_WITH_IT" => s__CLEANING_WITH_IT(v),
            "_CRUNCH_FIX" => s__CRUNCH_FIX(v),
            "_CRUNCH_FOODS" => s__CRUNCH_FOODS(v),
            "_CRUNCH_KEEP" => s__CRUNCH_KEEP(v),
            "_CRUNCH_OWN" => s__CRUNCH_OWN(v),
            "_CRUNCH_SAFE" => s__CRUNCH_SAFE(v),
            "_CRUNCH_STATE" => s__CRUNCH_STATE(v),
            "_CRUNCH_VERDICT" => s__CRUNCH_VERDICT(v),
            "_DAY_AFTER" => s__DAY_AFTER(v),
            "_DAY_AFTER_AVOID" => s__DAY_AFTER_AVOID(v),
            "_DAY_AFTER_ENTRY" => s__DAY_AFTER_ENTRY(v),
            "_DAY_AFTER_FOODS" => s__DAY_AFTER_FOODS(v),
            "_EATING_WORDS" => s__EATING_WORDS(v),
            "_FLOUR_LUMPS" => s__FLOUR_LUMPS(v),
            "_FLOUR_MITES" => s__FLOUR_MITES(v),
            "_LAST_TIME" => s__LAST_TIME(v),
            "_MUSH_FIX" => s__MUSH_FIX(v),
            "_MUSH_VERDICT" => s__MUSH_VERDICT(v),
            "_NOT_A_FRUIT" => s__NOT_A_FRUIT(v),
            "_OFF_GROUP_WORDS" => s__OFF_GROUP_WORDS(v),
            "_RIPEN_ASIDE_KEEPING" => s__RIPEN_ASIDE_KEEPING(v),
            "_SALT_CLUMPS" => s__SALT_CLUMPS(v),
            "_STAND_DOWN" => s__STAND_DOWN(v),
            "_STAND_DOWN_HARD" => s__STAND_DOWN_HARD(v),
            "_STATE_NOT_FOOD" => s__STATE_NOT_FOOD(v),
            "_YOGHURT_FIX" => s__YOGHURT_FIX(v),
            "_YOGHURT_WHEY" => s__YOGHURT_WHEY(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.keeping", module_obj);
    pyrt::xrefs::reg_fn("knowledge.keeping", "_oats_kept", py_func("_oats_kept", |args: &[Py]| _oats_kept(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_slimy_mushrooms", py_func("_slimy_mushrooms", |args: &[Py]| _slimy_mushrooms(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_freeze_a_sweet", py_func("_freeze_a_sweet", |args: &[Py]| _freeze_a_sweet(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_pasta_sauce_keep", py_func("_pasta_sauce_keep", |args: &[Py]| _pasta_sauce_keep(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_storecupboard_keep", py_func("_storecupboard_keep", |args: &[Py]| _storecupboard_keep(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_kept_a_while", py_func("_kept_a_while", |args: &[Py]| _kept_a_while(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_dated_row", py_func("_dated_row", |args: &[Py]| _dated_row(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_cut_out_for_is_an_aptitude", py_func("_cut_out_for_is_an_aptitude", |args: &[Py]| _cut_out_for_is_an_aptitude(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_a_thaw_that_is_a_job", py_func("_a_thaw_that_is_a_job", |args: &[Py]| _a_thaw_that_is_a_job(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_cheese_mould", py_func("_cheese_mould", |args: &[Py]| _cheese_mould(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_ripen_lead", py_func("_ripen_lead", |args: &[Py]| _ripen_lead(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_ripens", py_func("_ripens", |args: &[Py]| _ripens(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_ripen_haystack", py_func("_ripen_haystack", |args: &[Py]| _ripen_haystack(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_ripen_row_for", py_func("_ripen_row_for", |args: &[Py]| _ripen_row_for(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_ripen_from_the_chat", py_func("_ripen_from_the_chat", |args: &[Py]| _ripen_from_the_chat(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_said_anything", py_func("_said_anything", |args: &[Py]| _said_anything(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_crunch_reply", py_func("_crunch_reply", |args: &[Py]| _crunch_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_yoghurt_reply", py_func("_yoghurt_reply", |args: &[Py]| _yoghurt_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_flour_reply", py_func("_flour_reply", |args: &[Py]| _flour_reply(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_produce_is_named", py_func("_produce_is_named", |args: &[Py]| _produce_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_off_one", py_func("_off_one", |args: &[Py]| _off_one(args)));
    pyrt::xrefs::reg_fn("knowledge.keeping", "_produce_gone_off", py_func("_produce_gone_off", |args: &[Py]| _produce_gone_off(args)));
    pyrt::xrefs::reg_access("knowledge.keeping", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.keeping", "ASKS_HOW_LONG", g_ASKS_HOW_LONG, s_ASKS_HOW_LONG);
    pyrt::xrefs::reg_access("knowledge.keeping", "BLUE_CHEESES", g_BLUE_CHEESES, s_BLUE_CHEESES);
    pyrt::xrefs::reg_access("knowledge.keeping", "CALENDAR_DATE", g_CALENDAR_DATE, s_CALENDAR_DATE);
    pyrt::xrefs::reg_access("knowledge.keeping", "CHEESE_NAMES", g_CHEESE_NAMES, s_CHEESE_NAMES);
    pyrt::xrefs::reg_access("knowledge.keeping", "COLD_NOT", g_COLD_NOT, s_COLD_NOT);
    pyrt::xrefs::reg_access("knowledge.keeping", "COLD_REPLIES", g_COLD_REPLIES, s_COLD_REPLIES);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKED_IN_THE_ROW", g_COOKED_IN_THE_ROW, s_COOKED_IN_THE_ROW);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKED_VEG_COOK_WORDS", g_COOKED_VEG_COOK_WORDS, s_COOKED_VEG_COOK_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKED_VEG_NAMES", g_COOKED_VEG_NAMES, s_COOKED_VEG_NAMES);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKED_VEG_REPLY", g_COOKED_VEG_REPLY, s_COOKED_VEG_REPLY);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKED_VEG_STAND_DOWN", g_COOKED_VEG_STAND_DOWN, s_COOKED_VEG_STAND_DOWN);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKED_WORDS", g_COOKED_WORDS, s_COOKED_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOKING_WORDS", g_COOKING_WORDS, s_COOKING_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "COOK_LINE_WORDS", g_COOK_LINE_WORDS, s_COOK_LINE_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "CUT_OUT_FOR", g_CUT_OUT_FOR, s_CUT_OUT_FOR);
    pyrt::xrefs::reg_access("knowledge.keeping", "CUT_WORDS", g_CUT_WORDS, s_CUT_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "DATED_STAND_DOWN", g_DATED_STAND_DOWN, s_DATED_STAND_DOWN);
    pyrt::xrefs::reg_access("knowledge.keeping", "DATES_OWN", g_DATES_OWN, s_DATES_OWN);
    pyrt::xrefs::reg_access("knowledge.keeping", "DATE_ASK", g_DATE_ASK, s_DATE_ASK);
    pyrt::xrefs::reg_access("knowledge.keeping", "DATE_ROWS", g_DATE_ROWS, s_DATE_ROWS);
    pyrt::xrefs::reg_access("knowledge.keeping", "DATING_DATE", g_DATING_DATE, s_DATING_DATE);
    pyrt::xrefs::reg_access("knowledge.keeping", "DISH_ASK", g_DISH_ASK, s_DISH_ASK);
    pyrt::xrefs::reg_access("knowledge.keeping", "DISH_WORDS", g_DISH_WORDS, s_DISH_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.keeping", "FEEDING_WORDS", g_FEEDING_WORDS, s_FEEDING_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "FIX_FIRST", g_FIX_FIRST, s_FIX_FIRST);
    pyrt::xrefs::reg_access("knowledge.keeping", "FLOP_WORDS", g_FLOP_WORDS, s_FLOP_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "FOLLOW_UP_WORDS", g_FOLLOW_UP_WORDS, s_FOLLOW_UP_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "FREEZE_SWEETS", g_FREEZE_SWEETS, s_FREEZE_SWEETS);
    pyrt::xrefs::reg_access("knowledge.keeping", "FREEZE_SWEET_AVOID", g_FREEZE_SWEET_AVOID, s_FREEZE_SWEET_AVOID);
    pyrt::xrefs::reg_access("knowledge.keeping", "FREEZE_SWEET_NAMES", g_FREEZE_SWEET_NAMES, s_FREEZE_SWEET_NAMES);
    pyrt::xrefs::reg_access("knowledge.keeping", "FREEZE_WORDS", g_FREEZE_WORDS, s_FREEZE_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "GROWING_WORDS", g_GROWING_WORDS, s_GROWING_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEPING_KEY", g_KEEPING_KEY, s_KEEPING_KEY);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_ASKS", g_KEEP_ASKS, s_KEEP_ASKS);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_NOT_KEEPING", g_KEEP_NOT_KEEPING, s_KEEP_NOT_KEEPING);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_NOT_MEDICINE", g_KEEP_NOT_MEDICINE, s_KEEP_NOT_MEDICINE);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_NOT_SLEEP", g_KEEP_NOT_SLEEP, s_KEEP_NOT_SLEEP);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_QUALITY_WORDS", g_KEEP_QUALITY_WORDS, s_KEEP_QUALITY_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_TECHNIQUE", g_KEEP_TECHNIQUE, s_KEEP_TECHNIQUE);
    pyrt::xrefs::reg_access("knowledge.keeping", "KEEP_WORDS", g_KEEP_WORDS, s_KEEP_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "LONG_WORDS", g_LONG_WORDS, s_LONG_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "MOULD_WORDS", g_MOULD_WORDS, s_MOULD_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "OATS_BAG", g_OATS_BAG, s_OATS_BAG);
    pyrt::xrefs::reg_access("knowledge.keeping", "OATS_COOKED", g_OATS_COOKED, s_OATS_COOKED);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_ANY", g_OFF_ANY, s_OFF_ANY);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_AVOID", g_OFF_AVOID, s_OFF_AVOID);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_BERRIES", g_OFF_BERRIES, s_OFF_BERRIES);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_CITRUS", g_OFF_CITRUS, s_OFF_CITRUS);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_FIRM", g_OFF_FIRM, s_OFF_FIRM);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_LEAVES", g_OFF_LEAVES, s_OFF_LEAVES);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_NAMES", g_OFF_NAMES, s_OFF_NAMES);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_SOFT", g_OFF_SOFT, s_OFF_SOFT);
    pyrt::xrefs::reg_access("knowledge.keeping", "OFF_WORDS", g_OFF_WORDS, s_OFF_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "PACK_DATE", g_PACK_DATE, s_PACK_DATE);
    pyrt::xrefs::reg_access("knowledge.keeping", "PASTA_SAUCE_WORDS", g_PASTA_SAUCE_WORDS, s_PASTA_SAUCE_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "PROBLEM_ONLY", g_PROBLEM_ONLY, s_PROBLEM_ONLY);
    pyrt::xrefs::reg_access("knowledge.keeping", "REFREEZE_WORDS", g_REFREEZE_WORDS, s_REFREEZE_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "RIPEN_ASIDE", g_RIPEN_ASIDE, s_RIPEN_ASIDE);
    pyrt::xrefs::reg_access("knowledge.keeping", "RIPEN_ASKS", g_RIPEN_ASKS, s_RIPEN_ASKS);
    pyrt::xrefs::reg_access("knowledge.keeping", "RIPEN_TABLE", g_RIPEN_TABLE, s_RIPEN_TABLE);
    pyrt::xrefs::reg_access("knowledge.keeping", "RIPEN_WORDS", g_RIPEN_WORDS, s_RIPEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "ROWS", g_ROWS, s_ROWS);
    pyrt::xrefs::reg_access("knowledge.keeping", "ROW_COOK_FORMS", g_ROW_COOK_FORMS, s_ROW_COOK_FORMS);
    pyrt::xrefs::reg_access("knowledge.keeping", "ROW_STAND_DOWN", g_ROW_STAND_DOWN, s_ROW_STAND_DOWN);
    pyrt::xrefs::reg_access("knowledge.keeping", "SAUCE_KEEPING_WORDS", g_SAUCE_KEEPING_WORDS, s_SAUCE_KEEPING_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "SC_KEEP_ASKS", g_SC_KEEP_ASKS, s_SC_KEEP_ASKS);
    pyrt::xrefs::reg_access("knowledge.keeping", "SC_KEEP_AVOID", g_SC_KEEP_AVOID, s_SC_KEEP_AVOID);
    pyrt::xrefs::reg_access("knowledge.keeping", "SC_KEEP_NAMES", g_SC_KEEP_NAMES, s_SC_KEEP_NAMES);
    pyrt::xrefs::reg_access("knowledge.keeping", "SOFT_CHEESES", g_SOFT_CHEESES, s_SOFT_CHEESES);
    pyrt::xrefs::reg_access("knowledge.keeping", "STORECUPBOARD_KEEP", g_STORECUPBOARD_KEEP, s_STORECUPBOARD_KEEP);
    pyrt::xrefs::reg_access("knowledge.keeping", "STORE_ASKS", g_STORE_ASKS, s_STORE_ASKS);
    pyrt::xrefs::reg_access("knowledge.keeping", "TELL_ASKS", g_TELL_ASKS, s_TELL_ASKS);
    pyrt::xrefs::reg_access("knowledge.keeping", "THAW_WORDS", g_THAW_WORDS, s_THAW_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "TIMES_A_THAW", g_TIMES_A_THAW, s_TIMES_A_THAW);
    pyrt::xrefs::reg_access("knowledge.keeping", "UNDER_RIPE", g_UNDER_RIPE, s_UNDER_RIPE);
    pyrt::xrefs::reg_access("knowledge.keeping", "UNRIPE_STATED", g_UNRIPE_STATED, s_UNRIPE_STATED);
    pyrt::xrefs::reg_access("knowledge.keeping", "USE_UP_WORDS", g_USE_UP_WORDS, s_USE_UP_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "WHEN_WORDS", g_WHEN_WORDS, s_WHEN_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "YOGHURT", g_YOGHURT, s_YOGHURT);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CLEANING_WITH_IT", g__CLEANING_WITH_IT, s__CLEANING_WITH_IT);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_FIX", g__CRUNCH_FIX, s__CRUNCH_FIX);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_FOODS", g__CRUNCH_FOODS, s__CRUNCH_FOODS);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_KEEP", g__CRUNCH_KEEP, s__CRUNCH_KEEP);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_OWN", g__CRUNCH_OWN, s__CRUNCH_OWN);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_SAFE", g__CRUNCH_SAFE, s__CRUNCH_SAFE);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_STATE", g__CRUNCH_STATE, s__CRUNCH_STATE);
    pyrt::xrefs::reg_access("knowledge.keeping", "_CRUNCH_VERDICT", g__CRUNCH_VERDICT, s__CRUNCH_VERDICT);
    pyrt::xrefs::reg_access("knowledge.keeping", "_DAY_AFTER", g__DAY_AFTER, s__DAY_AFTER);
    pyrt::xrefs::reg_access("knowledge.keeping", "_DAY_AFTER_AVOID", g__DAY_AFTER_AVOID, s__DAY_AFTER_AVOID);
    pyrt::xrefs::reg_access("knowledge.keeping", "_DAY_AFTER_ENTRY", g__DAY_AFTER_ENTRY, s__DAY_AFTER_ENTRY);
    pyrt::xrefs::reg_access("knowledge.keeping", "_DAY_AFTER_FOODS", g__DAY_AFTER_FOODS, s__DAY_AFTER_FOODS);
    pyrt::xrefs::reg_access("knowledge.keeping", "_EATING_WORDS", g__EATING_WORDS, s__EATING_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "_FLOUR_LUMPS", g__FLOUR_LUMPS, s__FLOUR_LUMPS);
    pyrt::xrefs::reg_access("knowledge.keeping", "_FLOUR_MITES", g__FLOUR_MITES, s__FLOUR_MITES);
    pyrt::xrefs::reg_access("knowledge.keeping", "_LAST_TIME", g__LAST_TIME, s__LAST_TIME);
    pyrt::xrefs::reg_access("knowledge.keeping", "_MUSH_FIX", g__MUSH_FIX, s__MUSH_FIX);
    pyrt::xrefs::reg_access("knowledge.keeping", "_MUSH_VERDICT", g__MUSH_VERDICT, s__MUSH_VERDICT);
    pyrt::xrefs::reg_access("knowledge.keeping", "_NOT_A_FRUIT", g__NOT_A_FRUIT, s__NOT_A_FRUIT);
    pyrt::xrefs::reg_access("knowledge.keeping", "_OFF_GROUP_WORDS", g__OFF_GROUP_WORDS, s__OFF_GROUP_WORDS);
    pyrt::xrefs::reg_access("knowledge.keeping", "_RIPEN_ASIDE_KEEPING", g__RIPEN_ASIDE_KEEPING, s__RIPEN_ASIDE_KEEPING);
    pyrt::xrefs::reg_access("knowledge.keeping", "_SALT_CLUMPS", g__SALT_CLUMPS, s__SALT_CLUMPS);
    pyrt::xrefs::reg_access("knowledge.keeping", "_STAND_DOWN", g__STAND_DOWN, s__STAND_DOWN);
    pyrt::xrefs::reg_access("knowledge.keeping", "_STAND_DOWN_HARD", g__STAND_DOWN_HARD, s__STAND_DOWN_HARD);
    pyrt::xrefs::reg_access("knowledge.keeping", "_STATE_NOT_FOOD", g__STATE_NOT_FOOD, s__STATE_NOT_FOOD);
    pyrt::xrefs::reg_access("knowledge.keeping", "_YOGHURT_FIX", g__YOGHURT_FIX, s__YOGHURT_FIX);
    pyrt::xrefs::reg_access("knowledge.keeping", "_YOGHURT_WHEY", g__YOGHURT_WHEY, s__YOGHURT_WHEY);
    pyrt::xrefs::reg_access("knowledge.keeping", "entry", g_entry, s_entry);
}

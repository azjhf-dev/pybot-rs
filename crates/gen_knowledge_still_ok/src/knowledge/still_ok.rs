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
thread_local! { static G_DIET_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXTRA_FOODS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINCE_KEEPING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINCE_VERDICT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MINCE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OFF_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PACK_DATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_LINE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POISONING_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POISONING_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RAW_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SPOILAGE_BIAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNDERCOOKED_PINK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNRIPE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_UNRIPE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_FAULT_IN_THE_DISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CABBAGES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CABBAGE_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COOKING_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COUNT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYS_AGO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DAYS_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ENDS_ON_AN_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEPING_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAFY_GREENS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAFY_SHOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAFY_SIGNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEFTOVERS_RULE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MONEY_IN_THE_LINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PAST_KEEPING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAW_VEGETABLE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAW_VEGETABLE_NOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAW_VEG_GROUPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAW_VEG_ONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RAW_VEG_PLURAL_S: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ROOM_ODOUR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__R_MONTHS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHELLFISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__VERDICT_ASKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__item: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.still_ok: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DIET_WORDS() -> Py { ensure_init(); G_DIET_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DIET_WORDS(v: Py) { ensure_init(); G_DIET_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EXTRA_FOODS() -> Py { ensure_init(); G_EXTRA_FOODS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXTRA_FOODS(v: Py) { ensure_init(); G_EXTRA_FOODS.with(|c| *c.borrow_mut() = v); }
pub fn g_MINCE_KEEPING_WORDS() -> Py { ensure_init(); G_MINCE_KEEPING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINCE_KEEPING_WORDS(v: Py) { ensure_init(); G_MINCE_KEEPING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MINCE_VERDICT_WORDS() -> Py { ensure_init(); G_MINCE_VERDICT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINCE_VERDICT_WORDS(v: Py) { ensure_init(); G_MINCE_VERDICT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_MINCE_WORDS() -> Py { ensure_init(); G_MINCE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MINCE_WORDS(v: Py) { ensure_init(); G_MINCE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_ASKS() -> Py { ensure_init(); G_OFF_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_ASKS(v: Py) { ensure_init(); G_OFF_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_THE_THING() -> Py { ensure_init(); G_OFF_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_THE_THING(v: Py) { ensure_init(); G_OFF_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_OFF_WORDS() -> Py { ensure_init(); G_OFF_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OFF_WORDS(v: Py) { ensure_init(); G_OFF_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PACK_DATE() -> Py { ensure_init(); G_PACK_DATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PACK_DATE(v: Py) { ensure_init(); G_PACK_DATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_LINE_WORDS() -> Py { ensure_init(); G_PET_LINE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_LINE_WORDS(v: Py) { ensure_init(); G_PET_LINE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_POISONING_NOT() -> Py { ensure_init(); G_POISONING_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POISONING_NOT(v: Py) { ensure_init(); G_POISONING_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_POISONING_WORDS() -> Py { ensure_init(); G_POISONING_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POISONING_WORDS(v: Py) { ensure_init(); G_POISONING_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_RAW_WORDS() -> Py { ensure_init(); G_RAW_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RAW_WORDS(v: Py) { ensure_init(); G_RAW_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SPOILAGE_BIAS() -> Py { ensure_init(); G_SPOILAGE_BIAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SPOILAGE_BIAS(v: Py) { ensure_init(); G_SPOILAGE_BIAS.with(|c| *c.borrow_mut() = v); }
pub fn g_UNDERCOOKED_PINK() -> Py { ensure_init(); G_UNDERCOOKED_PINK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNDERCOOKED_PINK(v: Py) { ensure_init(); G_UNDERCOOKED_PINK.with(|c| *c.borrow_mut() = v); }
pub fn g_UNRIPE_NOT() -> Py { ensure_init(); G_UNRIPE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNRIPE_NOT(v: Py) { ensure_init(); G_UNRIPE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g_UNRIPE_WORDS() -> Py { ensure_init(); G_UNRIPE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_UNRIPE_WORDS(v: Py) { ensure_init(); G_UNRIPE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__A_FAULT_IN_THE_DISH() -> Py { ensure_init(); G__A_FAULT_IN_THE_DISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_FAULT_IN_THE_DISH(v: Py) { ensure_init(); G__A_FAULT_IN_THE_DISH.with(|c| *c.borrow_mut() = v); }
pub fn g__CABBAGES() -> Py { ensure_init(); G__CABBAGES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CABBAGES(v: Py) { ensure_init(); G__CABBAGES.with(|c| *c.borrow_mut() = v); }
pub fn g__CABBAGE_SIGNS() -> Py { ensure_init(); G__CABBAGE_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CABBAGE_SIGNS(v: Py) { ensure_init(); G__CABBAGE_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g__COOKING_ASK() -> Py { ensure_init(); G__COOKING_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COOKING_ASK(v: Py) { ensure_init(); G__COOKING_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__COUNT_WORDS() -> Py { ensure_init(); G__COUNT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COUNT_WORDS(v: Py) { ensure_init(); G__COUNT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYS_AGO() -> Py { ensure_init(); G__DAYS_AGO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYS_AGO(v: Py) { ensure_init(); G__DAYS_AGO.with(|c| *c.borrow_mut() = v); }
pub fn g__DAYS_SAID() -> Py { ensure_init(); G__DAYS_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DAYS_SAID(v: Py) { ensure_init(); G__DAYS_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__ENDS_ON_AN_ASK() -> Py { ensure_init(); G__ENDS_ON_AN_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ENDS_ON_AN_ASK(v: Py) { ensure_init(); G__ENDS_ON_AN_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEPING_IN_THE_LINE() -> Py { ensure_init(); G__KEEPING_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEPING_IN_THE_LINE(v: Py) { ensure_init(); G__KEEPING_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAFY_GREENS() -> Py { ensure_init(); G__LEAFY_GREENS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAFY_GREENS(v: Py) { ensure_init(); G__LEAFY_GREENS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAFY_SHOWS() -> Py { ensure_init(); G__LEAFY_SHOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAFY_SHOWS(v: Py) { ensure_init(); G__LEAFY_SHOWS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAFY_SIGNS() -> Py { ensure_init(); G__LEAFY_SIGNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAFY_SIGNS(v: Py) { ensure_init(); G__LEAFY_SIGNS.with(|c| *c.borrow_mut() = v); }
pub fn g__LEFTOVERS_RULE() -> Py { ensure_init(); G__LEFTOVERS_RULE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEFTOVERS_RULE(v: Py) { ensure_init(); G__LEFTOVERS_RULE.with(|c| *c.borrow_mut() = v); }
pub fn g__MONEY_IN_THE_LINE() -> Py { ensure_init(); G__MONEY_IN_THE_LINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MONEY_IN_THE_LINE(v: Py) { ensure_init(); G__MONEY_IN_THE_LINE.with(|c| *c.borrow_mut() = v); }
pub fn g__NOT_THIS() -> Py { ensure_init(); G__NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NOT_THIS(v: Py) { ensure_init(); G__NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g__PAST_KEEPING() -> Py { ensure_init(); G__PAST_KEEPING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PAST_KEEPING(v: Py) { ensure_init(); G__PAST_KEEPING.with(|c| *c.borrow_mut() = v); }
pub fn g__RAW_VEGETABLE_NAMES() -> Py { ensure_init(); G__RAW_VEGETABLE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAW_VEGETABLE_NAMES(v: Py) { ensure_init(); G__RAW_VEGETABLE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g__RAW_VEGETABLE_NOT() -> Py { ensure_init(); G__RAW_VEGETABLE_NOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAW_VEGETABLE_NOT(v: Py) { ensure_init(); G__RAW_VEGETABLE_NOT.with(|c| *c.borrow_mut() = v); }
pub fn g__RAW_VEG_GROUPS() -> Py { ensure_init(); G__RAW_VEG_GROUPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAW_VEG_GROUPS(v: Py) { ensure_init(); G__RAW_VEG_GROUPS.with(|c| *c.borrow_mut() = v); }
pub fn g__RAW_VEG_ONE() -> Py { ensure_init(); G__RAW_VEG_ONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAW_VEG_ONE(v: Py) { ensure_init(); G__RAW_VEG_ONE.with(|c| *c.borrow_mut() = v); }
pub fn g__RAW_VEG_PLURAL_S() -> Py { ensure_init(); G__RAW_VEG_PLURAL_S.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RAW_VEG_PLURAL_S(v: Py) { ensure_init(); G__RAW_VEG_PLURAL_S.with(|c| *c.borrow_mut() = v); }
pub fn g__ROOM_ODOUR() -> Py { ensure_init(); G__ROOM_ODOUR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ROOM_ODOUR(v: Py) { ensure_init(); G__ROOM_ODOUR.with(|c| *c.borrow_mut() = v); }
pub fn g__R_MONTHS() -> Py { ensure_init(); G__R_MONTHS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__R_MONTHS(v: Py) { ensure_init(); G__R_MONTHS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHELLFISH() -> Py { ensure_init(); G__SHELLFISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHELLFISH(v: Py) { ensure_init(); G__SHELLFISH.with(|c| *c.borrow_mut() = v); }
pub fn g__VERDICT_ASKS() -> Py { ensure_init(); G__VERDICT_ASKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__VERDICT_ASKS(v: Py) { ensure_init(); G__VERDICT_ASKS.with(|c| *c.borrow_mut() = v); }
pub fn g__item() -> Py { ensure_init(); G__item.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__item(v: Py) { ensure_init(); G__item.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Gone off: \"is this still ok to eat\", about the thing in your hand.\n\nThe one food question where guessing is worst, and one people ask every week: a pack of chicken\ntwo days past its date, mince that smells a bit, a bit of blue on the cheddar. `cooking.py`\nanswers what to do with a food and its `KEEPS` table answers how long it keeps; neither answers\n\"is this one still all right\", which wants the test and the verdict.\n\nThe entries here are data, not a new mechanism: keys, `avoid`, `ask` and the shared food words\ncome from `bot.knowledge.lookup` and `bot.knowledge.cooking`, so there is one matcher and one set\nof food words in the bot.\n\nThree gates keep a row off the lines that belong to somebody else, and every one of them is a\nwrong answer rather than a dull one if it is missing:\n\n- `DIET_WORDS` - \"is cheese bad for you\", \"is it ok to eat 2 eggs a day\": a spoilage test there\n  reads as a diet answer, and the diet answer already exists.\n- `RAW_WORDS` - \"is it safe to eat raw pork\" asks how to cook it. A spoilage test reads as\n  permission to eat it raw, which is the worst possible reply.\n- keeping and cooking words - \"how long does mince keep\", \"is it ok to reheat chicken\": those\n  have their own answers, and a yes-or-no test given instead loses the number they asked for.\n\nA line that is not one of these questions passes, so the row fires only when the line names the\nfood *and* asks the spoilage question with one of `OFF_ASKS`.\n\nEggs and cheese are not here: main already tests a suspect egg in a glass of water and already\nsays to cut the mould off hard cheese, so those lines have their answer. Only the five foods\nthat get a hedge or a nutrition speech are in this table.\n");
    s_PET_LINE_WORDS(pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS"));
    s_PACK_DATE(pyrt::xrefs::xget("knowledge.lookup", "PACK_DATE"));
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("cooking"), py_str("food-and-drink")]));
    fn __data1() -> Py { py_tuple(vec![py_str("kidney beans"), py_str("red kidney beans"), py_str("cannellini beans"), py_str("haricot beans"), py_str("black beans"), py_str("pinto beans"), py_str("butter beans"), py_str("soya beans"), py_str("soy beans"), py_str("flageolet beans"), py_str("chickpeas"), py_str("chick peas"), py_str("black-eyed beans")]) }
    fn __data2() -> Py { py_tuple(vec![py_str("broccoli"), py_str("cauliflower"), py_str("kale"), py_str("spinach"), py_str("peas"), py_str("mangetout"), py_str("sugar snap peas"), py_str("carrot"), py_str("carrots"), py_str("cucumber"), py_str("courgette"), py_str("courgettes"), py_str("beetroot"), py_str("onion"), py_str("onions"), py_str("spring onion"), py_str("spring onions"), py_str("mushroom"), py_str("mushrooms"), py_str("cabbage"), py_str("red cabbage"), py_str("sweetcorn"), py_str("sweet corn"), py_str("pepper"), py_str("peppers"), py_str("bell pepper"), py_str("celery"), py_str("radish"), py_str("radishes"), py_str("lettuce"), py_str("fennel"), py_str("avocado"), py_str("tomato"), py_str("tomatoes"), py_str("cress"), py_str("watercress"), py_str("rocket"), py_str("leek"), py_str("garlic"), py_str("ginger"), py_str("parsnip"), py_str("swede"), py_str("turnip")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._RAW_VEG_GROUPS"); }
    s__RAW_VEG_GROUPS(py_tuple(vec![py_tuple(vec![py_str("potato"), py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("sweet potato"), py_str("sweet potatoes")]), py_str("Cook %(one)s rather than eating %(them)s raw: a raw potato and any green patch on it carry solanine, which gives stomach ache and vomiting, and the raw starch is hard to digest. Boil, roast or mash %(them)s, and bin any potato with green patches or sprouts.")]), py_tuple(vec![py_str("starchy"), py_tuple(vec![py_str("yam"), py_str("yams"), py_str("cassava"), py_str("plantain"), py_str("plantains")]), py_str("Cook %(one)s rather than eating %(them)s raw: the starch is hard to digest uncooked and %(one)s tastes bitter and astringent raw. Boil, roast or fry %(them)s until tender, and peel cassava and cook it right through - raw cassava carries cyanide, and boiling or frying is what takes it out.")]), py_tuple(vec![py_str("dried-bean"), __data1(), py_str("Dried %(one)s have to be boiled hard for at least ten minutes: raw and undercooked ones carry a protein that causes violent vomiting and cramps, and a slow cooker never gets hot enough. Tinned ones are already cooked, so rinse and eat them.")]), py_tuple(vec![py_str("green-bean"), py_tuple(vec![py_str("green beans"), py_str("runner beans"), py_str("broad beans"), py_str("french beans"), py_str("string beans")]), py_str("Cook %(one)s rather than eating them raw: they hold a protein that upsets the stomach, so boil or steam them four to five minutes until they are tender but still squeaky. A handful raw in a salad is not dangerous for most people, but they taste better cooked.")]), py_tuple(vec![py_str("salad"), __data2(), py_str("%(Cap)s %(are)s fine raw: wash %(them)s and eat %(them)s as %(they)s %(are)s - the crunch and the vitamins are what cooking takes out. A minute in boiling water softens %(them)s if you would rather have %(them)s cooked.")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._RAW_VEG_PLURAL_S"); }
    s__RAW_VEG_PLURAL_S(call_builtin("frozenset", &[py_tuple(vec![py_str("sweetcorn"), py_str("sweet corn"), py_str("cress"), py_str("watercress"), py_str("mangetout")])]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._RAW_VEGETABLE_NAMES"); }
    s__RAW_VEGETABLE_NAMES(call_builtin("tuple", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RAW_VEG_GROUPS()) { let __unpk1809 = py_unpack(&__item.clone(), 3, None); let mut _u: Py = __unpk1809[0].clone(); let mut names: Py = __unpk1809[1].clone(); let mut _u: Py = __unpk1809[2].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._RAW_VEG_ONE"); }
    s__RAW_VEG_ONE(call_builtin("dict", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RAW_VEG_GROUPS()) { let __unpk609 = py_unpack(&__item.clone(), 3, None); let mut group: Py = __unpk609[0].clone(); let mut names: Py = __unpk609[1].clone(); let mut text: Py = __unpk609[2].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(py_tuple(vec![name.clone(), py_tuple(vec![group.clone(), text.clone()])])); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    fn __data3() -> Py { py_tuple(vec![py_str("how long"), py_str("how do i cook"), py_str("how to cook"), py_str("how long to cook"), py_str("how long do i cook"), py_str("recipe"), py_str("recipes"), py_str("roast"), py_str("roasting"), py_str("roasted"), py_str("boil"), py_str("boiling"), py_str("boiled"), py_str("steam"), py_str("steaming"), py_str("fry"), py_str("frying"), py_str("fried"), py_str("air fry"), py_str("air fryer"), py_str("slow cooker"), py_str("soup"), py_str("curry"), py_str("stew"), py_str("bake"), py_str("baking"), py_str("baked"), py_str("microwave"), py_str("grow"), py_str("growing"), py_str("grown"), py_str("plant"), py_str("planting"), py_str("garden"), py_str("allotment"), py_str("seed"), py_str("seeds"), py_str("harvest"), py_str("blight"), py_str("fridge"), py_str("freezer"), py_str("freeze"), py_str("keeping"), py_str("keeps"), py_str("keep"), py_str("store"), py_str("shop"), py_str("buy"), py_str("cheapest"), py_str("price"), py_str("calories"), py_str("protein"), py_str("fibre"), py_str("vitamin"), py_str("nutrition"), py_str("diet"), py_str("sprout"), py_str("sprouts"), py_str("alfalfa"), py_str("beansprouts"), py_str("bean sprouts"), py_str("green potato"), py_str("green potatoes"), py_str("solanine"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("vet"), py_str("rabbit"), py_str("hamster"), py_str("guinea pig"), py_str("bird"), py_str("chicken"), py_str("chickens")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._RAW_VEGETABLE_NOT"); }
    s__RAW_VEGETABLE_NOT(__data3());
    fn __data4() -> Py { py_tuple(vec![py_str("ok"), py_str("okay"), py_str("safe"), py_str("still"), py_str("eat"), py_str("eat it"), py_str("bin"), py_str("throw"), py_str("use"), py_str("use it"), py_str("fine"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("sour"), py_str("bad"), py_str("should i keep"), py_str("should i"), py_str("slimy"), py_str("slime"), py_str("slimy feel"), py_str("tacky"), py_str("musty"), py_str("discoloured"), py_str("discolored"), py_str("grey green"), py_str("grey-green"), py_str("green patches"), py_str("gone slimy"), py_str("is slimy"), py_str("off"), py_str("gone off"), py_str("going off"), py_str("go off"), py_str("went off"), py_str("gone bad"), py_str("how do i know"), py_str("how can i tell"), py_str("how do you tell"), py_str("know if"), py_str("tell if"), py_str("fresh"), py_str("is fresh"), py_str("still fresh"), py_str("smells fresh"), py_str("looks fresh"), py_str("fresh to eat"), py_str("fresh enough"), py_str("is it fresh"), py_str("fresh or not"), py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("past their date"), py_str("use by"), py_str("use by date"), py_str("best before"), py_str("gone past"), py_str("next day"), py_str("the next day"), py_str("the day after"), py_str("day after"), py_str("day later"), py_str("days later"), py_str("yesterday"), py_str("in the fridge"), py_str("out of the fridge"), py_str("slimy"), py_str("gone slimy"), py_str("went slimy"), py_str("is slimy"), py_str("looks slimy"), py_str("smells off"), py_str("smelt off"), py_str("smells wrong"), py_str("smells funny"), py_str("has gone"), py_str("gone bad"), py_str("has gone bad"), py_str("looks off"), py_str("is off")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.OFF_ASKS"); }
    s_OFF_ASKS(__data4());
    fn __data5() -> Py { py_tuple(vec![py_str("off the"), py_str("off my"), py_str("off your"), py_str("off his"), py_str("off her"), py_str("off its"), py_str("off them"), py_str("off it"), py_str("cut off"), py_str("cutting off"), py_str("chop off"), py_str("chopping off"), py_str("slice off"), py_str("skin off"), py_str("take the skin"), py_str("fat off"), py_str("off the bone"), py_str("turn off"), py_str("turned off"), py_str("switch off"), py_str("switched off"), py_str("day off"), py_str("days off"), py_str("time off"), py_str("off work"), py_str("off sick"), py_str("show off"), py_str("better off"), py_str("worse off"), py_str("off the menu"), py_str("off the heat"), py_str("off the hob"), py_str("off the plate")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.OFF_THE_THING"); }
    s_OFF_THE_THING(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("still ok"), py_str("still ok to eat"), py_str("still good"), py_str("ok to eat"), py_str("okay to eat"), py_str("gone off"), py_str("going off"), py_str("off"), py_str("smells"), py_str("smell"), py_str("smelly"), py_str("slimy"), py_str("funny"), py_str("bad"), py_str("tacky"), py_str("musty"), py_str("discoloured"), py_str("discolored"), py_str("grey green"), py_str("grey-green"), py_str("gone bad"), py_str("past it"), py_str("past its date"), py_str("is it safe"), py_str("safe to eat"), py_str("out of date"), py_str("past the date"), py_str("past their date"), py_str("use by"), py_str("use by date"), py_str("best before"), py_str("gone past"), py_str("gone past its date"), py_str("fresh"), py_str("is fresh"), py_str("still fresh"), py_str("smells fresh"), py_str("looks fresh"), py_str("fresh to eat"), py_str("fresh enough"), py_str("is it fresh"), py_str("fresh or not"), py_str("next day"), py_str("the next day"), py_str("the day after"), py_str("day after"), py_str("yesterday"), py_str("tomorrow"), py_str("tomorow"), py_str("tmrw"), py_str("tomoz"), py_str("in the fridge"), py_str("out of the fridge"), py_str("after a day"), py_str("after one day"), py_str("after 2 days"), py_str("after two days"), py_str("after 3 days"), py_str("after three days"), py_str("after 4 days"), py_str("after four days"), py_str("after 5 days"), py_str("after five days"), py_str("a couple of days later"), py_str("a few days later"), py_str("bin"), py_str("bin it"), py_str("throw"), py_str("throw it"), py_str("throw away"), py_str("throw out"), py_str("chuck"), py_str("toss"), py_str("chuck it"), py_str("toss it"), py_str("still eat"), py_str("eat this"), py_str("eat that"), py_str("can i eat"), py_str("still alright"), py_str("still all right"), py_str("alright")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.OFF_WORDS"); }
    s_OFF_WORDS(op_add(&pyrt::xrefs::xget("knowledge.lookup", "PACK_DATE"), &__data6()));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.SPOILAGE_BIAS"); }
    s_SPOILAGE_BIAS(Py::Int(3));
    fn __data7() -> Py { py_tuple(vec![py_str("yogurt"), py_str("yoghurt"), py_str("cream"), py_str("soured cream"), py_str("single cream"), py_str("double cream"), py_str("ham"), py_str("gammon"), py_str("leftover"), py_str("leftovers"), py_str("pate"), py_str("smoked salmon"), py_str("prawn"), py_str("prawns"), py_str("sausage"), py_str("sausages"), py_str("burger"), py_str("burgers"), py_str("milk"), py_str("kefir")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.EXTRA_FOODS"); }
    s_EXTRA_FOODS(__data7());
    fn __data8() -> Py { py_tuple(vec![py_str("every day"), py_str("a day"), py_str("a week"), py_str("per day"), py_str("per week"), py_str("healthy"), py_str("unhealthy"), py_str("good for you"), py_str("bad for you"), py_str("diet"), py_str("dieting"), py_str("processed"), py_str("nutrition"), py_str("nutritious"), py_str("protein"), py_str("calories"), py_str("vitamins"), py_str("how many"), py_str("too much"), py_str("too many"), py_str("pregnant"), py_str("pregnancy"), py_str("listeria"), py_str("cholesterol"), py_str("diabetic"), py_str("diabetes"), py_str("allergic"), py_str("allergy"), py_str("intolerant"), py_str("lactose"), py_str("toddler"), py_str("baby"), py_str("for a toddler"), py_str("for a baby")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.DIET_WORDS"); }
    s_DIET_WORDS(__data8());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.RAW_WORDS"); }
    s_RAW_WORDS(py_tuple(vec![py_str("raw"), py_str("undercooked"), py_str("undercook"), py_str("underdone"), py_str("bloody"), py_str("raw inside"), py_str("pink in the middle"), py_str("still pink"), py_str("not cooked through"), py_str("rare"), py_str("medium rare"), py_str("blue steak")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.UNDERCOOKED_PINK"); }
    s_UNDERCOOKED_PINK(op_add(&g_RAW_WORDS(), &py_tuple(vec![py_str("pink")])));
    fn __data9() -> Py { py_tuple(vec![py_str("frozen"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing"), py_str("left out"), py_str("overnight"), py_str("buy"), py_str("buying"), py_str("to buy"), py_str("shop for"), py_str("shopping list"), py_str("room temperature"), py_str("reheat"), py_str("reheating"), py_str("reheated"), py_str("heat up"), py_str("heated up"), py_str("warm up"), py_str("microwave"), py_str("warm through"), py_str("piping hot"), py_str("fridge"), py_str("how many days"), py_str("how many day"), py_str("does it last"), py_str("do they last"), py_str("will it last"), py_str("days in the fridge"), py_str("day in the fridge"), py_str("days in a fridge"), py_str("how long"), py_str("why"), py_str("what causes"), py_str("cause"), py_str("how do i keep"), py_str("how do i store"), py_str("how to keep"), py_str("how to store"), py_str("how can i keep"), py_str("how should i keep"), py_str("how can i store"), py_str("dry"), py_str("dried out"), py_str("overcooked"), py_str("rubbery"), py_str("tough"), py_str("chewy"), py_str("refreeze"), py_str("refroze"), py_str("re-freeze"), py_str("refreezing"), py_str("freeze again"), py_str("frozen again"), py_str("sold me"), py_str("sold us"), py_str("sold it to me"), py_str("refund"), py_str("money back"), py_str("take it back"), py_str("take them back"), py_str("bring it back"), py_str("return it"), py_str("bought it like that"), py_str("illegal to sell"), py_str("sell food"), py_str("selling food"), py_str("sell by"), py_str("complaint"), py_str("complain"), py_str("bought out of date"), py_str("bought it out of date"), py_str("bought this out of date"), py_str("buying out of date"), py_str("out of date when i bought"), py_str("was out of date when i bought"), py_str("freezer burn"), py_str("freezer burnt"), py_str("freezerburn"), py_str("freezer-burn"), py_str("freezer burned"), py_str("frost bitten"), py_str("frostbite")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._NOT_THIS"); }
    s__NOT_THIS(__data9());
    fn __data10() -> Py { py_tuple(vec![py_str("unripe"), py_str("unripened"), py_str("underripe"), py_str("under ripe"), py_str("not ripe"), py_str("isnt ripe"), py_str("green tomatoes"), py_str("green tomato"), py_str("green bananas"), py_str("green banana"), py_str("green plums"), py_str("green mango"), py_str("hard avocado"), py_str("hard mango"), py_str("hard peach"), py_str("hard peaches"), py_str("hard pears")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.UNRIPE_WORDS"); }
    s_UNRIPE_WORDS(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("potato"), py_str("potatoes"), py_str("solanine"), py_str("chutney"), py_str("relish"), py_str("recipe"), py_str("recipes"), py_str("make"), py_str("making"), py_str("cook"), py_str("cooking"), py_str("cook them"), py_str("fried green"), py_str("fry"), py_str("how do i ripen"), py_str("ripen them"), py_str("get them to ripen"), py_str("plant"), py_str("growing"), py_str("grow"), py_str("shop"), py_str("buy"), py_str("pick"), py_str("when to pick"), py_str("harvest"), py_str("blight"), py_str("garden")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.UNRIPE_NOT"); }
    s_UNRIPE_NOT(__data11());
    fn __data12() -> Py { py_tuple(vec![py_str("food poisoning"), py_str("poisoning"), py_str("campylobacter"), py_str("salmonella"), py_str("listeria"), py_str("give you"), py_str("give me"), py_str("make you ill"), py_str("make you sick"), py_str("make me ill"), py_str("make me sick"), py_str("get ill"), py_str("get sick"), py_str("harmful"), py_str("carry"), py_str("causes"), py_str("cause food"), py_str("why is it bad"), py_str("why is chicken bad"), py_str("why dangerous")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.POISONING_WORDS"); }
    s_POISONING_WORDS(__data12());
    fn __data13() -> Py { py_tuple(vec![py_str("how long"), py_str("how many days"), py_str("how do i cook"), py_str("how do i cook it"), py_str("cook it"), py_str("cooking time"), py_str("roast"), py_str("roasting"), py_str("temperature"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing"), py_str("freeze"), py_str("freezing"), py_str("fridge"), py_str("freezer"), py_str("rice"), py_str("left out"), py_str("reheat"), py_str("reheating"), py_str("i have"), py_str("i got"), py_str("i ate"), py_str("i think i"), py_str("my stomach"), py_str("diarrhoea"), py_str("vomiting"), py_str("symptoms"), py_str("treatment"), py_str("how to treat"), py_str("restaurant"), py_str("takeaway"), py_str("cafe"), py_str("pub"), py_str("wedding"), py_str("buffet"), py_str("kebab"), py_str("gave me"), py_str("made me"), py_str("report"), py_str("complaint"), py_str("claim"), py_str("compensation"), py_str("sue"), py_str("advice"), py_str("what should i do")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.POISONING_NOT"); }
    s_POISONING_NOT(__data13());
    fn __data14() -> Py { py_tuple(vec![py_str("mince"), py_str("minced"), py_str("mincemeat"), py_str("mince meat"), py_str("ground beef"), py_str("ground meat"), py_str("ground lamb"), py_str("ground pork"), py_str("beef mince"), py_str("lamb mince"), py_str("pork mince"), py_str("turkey mince"), py_str("chicken mince")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.MINCE_WORDS"); }
    s_MINCE_WORDS(__data14());
    fn __data15() -> Py { py_tuple(vec![py_str("day"), py_str("days"), py_str("next day"), py_str("the day after"), py_str("day after"), py_str("days later"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("kept"), py_str("how long"), py_str("ok to eat"), py_str("safe to eat"), py_str("use by"), py_str("yesterday"), py_str("tomorrow"), py_str("tonight"), py_str("left out"), py_str("left over"), py_str("leftovers"), py_str("in the fridge"), py_str("out of the fridge"), py_str("fridge"), py_str("freezer"), py_str("frozen")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.MINCE_KEEPING_WORDS"); }
    s_MINCE_KEEPING_WORDS(__data15());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._A_FAULT_IN_THE_DISH"); }
    s__A_FAULT_IN_THE_DISH(stdlib_call("re", "compile", &[py_str("\\b(?:too (?:hot to eat|hot for|spicy|salt\\w*|sweet|sour|bland|thick|thin|dry)|way too (?:spicy|hot|salty)|less spicy|make it milder|too much (?:chilli|chili|salt|spice)|to cool it down)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data16() -> Py { py_tuple(vec![py_str("still ok"), py_str("still good"), py_str("still fine"), py_str("still safe"), py_str("still alright"), py_str("still all right"), py_str("smells"), py_str("smell"), py_str("smelly"), py_str("slimy"), py_str("sour"), py_str("off"), py_str("gone bad"), py_str("gone off"), py_str("went off"), py_str("looks off"), py_str("is off"), py_str("green")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.MINCE_VERDICT_WORDS"); }
    s_MINCE_VERDICT_WORDS(__data16());
    fn __data17() -> Py { py_tuple(vec![py_str("fridge"), py_str("freezer"), py_str("keep"), py_str("keeps"), py_str("keeping"), py_str("last"), py_str("lasts"), py_str("ok"), py_str("okay"), py_str("safe"), py_str("still"), py_str("next day"), py_str("tomorrow"), py_str("gone off"), py_str("go off"), py_str("went off"), py_str("smell"), py_str("smells"), py_str("slimy"), py_str("sour"), py_str("days"), py_str("how long"), py_str("bin"), py_str("throw"), py_str("eat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._KEEPING_IN_THE_LINE"); }
    s__KEEPING_IN_THE_LINE(__data17());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._PAST_KEEPING"); }
    s__PAST_KEEPING(py_tuple(vec![py_str("yesterday"), py_str("last night"), py_str("this morning"), py_str("days ago"), py_str("day ago"), py_str("the day before"), py_str("a couple of days"), py_str("a few days")]));
    fn __data18() -> Py { py_tuple(vec![py_str("what can i make"), py_str("what can i do with"), py_str("what should i make"), py_str("what can i cook"), py_str("what shall i cook"), py_str("what should i cook"), py_str("cook tonight"), py_str("cook for dinner"), py_str("make tonight"), py_str("recipe"), py_str("recipes"), py_str("meal idea"), py_str("ideas for"), py_str("idea for"), py_str("how do i make"), py_str("how to make"), py_str("how do i cook"), py_str("how to cook"), py_str("cook it"), py_str("reheat"), py_str("re-heat"), py_str("use up"), py_str("using up"), py_str("what goes with"), py_str("serve with"), py_str("side for"), py_str("freeze")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._COOKING_ASK"); }
    s__COOKING_ASK(__data18());
    fn __data19() -> Py { py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("fourteen"), Py::Int(14)), (py_str("a couple of"), Py::Int(2)), (py_str("a few"), Py::Int(3)), (py_str("a"), Py::Int(1))]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._COUNT_WORDS"); }
    s__COUNT_WORDS(__data19());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._DAYS_AGO"); }
    s__DAYS_AGO(stdlib_call("re", "compile", &[py_str("\\b(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fourteen)\\s+days?\\s+(?:ago|old|since)\\b|\\b(?:in the fridge|in there|sat there|sitting there|been there|in the pot|in the tub)(?:\\s+(?:for|about|nearly|over))?\\s+(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fourteen)\\s+days?\\b|\\b(?:made|cooked|did it|made it|cooked it)\\s+(\\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fourteen)\\s+days?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data20() -> Py { py_tuple(vec![py_tuple(vec![py_str("the day before yesterday"), Py::Int(2)]), py_tuple(vec![py_str("yesterday"), Py::Int(1)]), py_tuple(vec![py_str("this morning"), Py::Int(0)]), py_tuple(vec![py_str("last night"), Py::Int(1)]), py_tuple(vec![py_str("tomorrow"), Py::Int(1)]), py_tuple(vec![py_str("the next day"), Py::Int(1)]), py_tuple(vec![py_str("next day"), Py::Int(1)]), py_tuple(vec![py_str("a couple of days"), Py::Int(2)]), py_tuple(vec![py_str("two days"), Py::Int(2)]), py_tuple(vec![py_str("a few days"), Py::Int(3)]), py_tuple(vec![py_str("three days"), Py::Int(3)]), py_tuple(vec![py_str("a week"), Py::Int(7)]), py_tuple(vec![py_str("a fortnight"), Py::Int(14)]), py_tuple(vec![py_str("a month"), Py::Int(30)])]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._DAYS_SAID"); }
    s__DAYS_SAID(__data20());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._ENDS_ON_AN_ASK"); }
    s__ENDS_ON_AN_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:ok|okay|alright|all right|safe|edible)\\s*[.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data21() -> Py { py_tuple(vec![py_str("can i"), py_str("could i"), py_str("should i"), py_str("shall i"), py_str("do i"), py_str("is it"), py_str("is that"), py_str("is this"), py_str("will it"), py_str("would it"), py_str("are they"), py_str("does it"), py_str("ok to"), py_str("okay to"), py_str("safe to"), py_str("still ok"), py_str("still good"), py_str("still fine"), py_str("am i ok"), py_str("too late"), py_str("gone off")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._VERDICT_ASKS"); }
    s__VERDICT_ASKS(__data21());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._LEFTOVERS_RULE"); }
    s__LEFTOVERS_RULE(py_str("Cooled within two hours and kept covered in the fridge, a cooked dish keeps 3 to 4 days, and anything with rice in it only 1. Reheat only the portion you are eating, once, until it is piping hot right through - a second reheating is where people get caught. It is past it if it smells sour or looks slimy, and if it sat out warm for more than 2 hours, bin it whatever it looks like."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._MONEY_IN_THE_LINE"); }
    s__MONEY_IN_THE_LINE(stdlib_call("re", "compile", &[py_str("[£$€]\\s?\\d|\\b\\d+\\s?(?:pounds?|quid|dollars?|euros?|bucks?)\\b|\\b(?:money|budget|pay|paid|payday|salary|wages?|savings?|income|pension|rent|mortgage|bills?|overdraft|debt|direct debit|standing order)\\b|\\b\\d{2,}\\s*(?:left ?overs?|spare|over a month|over each|over every)\\b|\\b(?:left ?overs?|spare)\\b[^?.!]{0,20}\\b(?:each|every|per)\\s+(?:week|month|year|payday)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    fn __data23() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("swallowed"), py_str("swallow"), py_str("chewed"), py_str("choking"), py_str("choke"), py_str("vomiting")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("toxic to"), py_str("safe to eat"), py_str("safe to feed"), py_str("edible"), py_str("can i eat"), py_str("can you eat"), py_str("eat them"), py_str("eat the leaves"), py_str("harmful"), py_str("dangerous"), py_str("eat it")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("yellow"), py_str("yellowing"), py_str("pale"), py_str("blight"), py_str("spots"), py_str("curl"), py_str("curling"), py_str("wilt"), py_str("wilting"), py_str("holes"), py_str("black"), py_str("brown"), py_str("feed"), py_str("fertiliser"), py_str("compost"), py_str("prune"), py_str("pruning"), py_str("grow"), py_str("growing"), py_str("harvest"), py_str("plant them"), py_str("when to plant")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("fish"), py_str("salmon"), py_str("cod"), py_str("prawns"), py_str("prawn"), py_str("haddock"), py_str("tuna"), py_str("fish is off"), py_str("fish off"), py_str("fish gone off"), py_str("fish has gone off"), py_str("salmon is off"), py_str("prawns are off"), py_str("fish gone bad"), py_str("fish is bad"), py_str("fish is fresh"), py_str("fish fresh"), py_str("salmon is fresh"), py_str("salmon fresh"), py_str("prawns are fresh"), py_str("cod is fresh")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("kitchen"), py_str("fried"), py_str("frying"), py_str("fry"), py_str("get rid of the smell"), py_str("smell out"), py_str("tank"), py_str("fish tank"), py_str("aquarium"), py_str("goldfish"), py_str("pond"), py_str("ponds"), py_str("fish bowl"), py_str("tropical fish"), py_str("pet fish"), py_str("fish tank water"), py_str("dishwasher"), py_str("washing machine"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("microwave"), py_str("sink"), py_str("bin"), py_str("bins"), py_str("bin smells"), py_str("cupboard smells"), py_str("room smells"), py_str("house smells"), py_str("smells of fish"), py_str("smells like fish"), py_str("smelling of fish"), py_str("how often"), py_str("a week"), py_str("per week"), py_str("every day"), py_str("every other day"), py_str("twice a week"), py_str("three times a week"), py_str("times a week")]) }
    fn __data28() -> Py { py_tuple(vec![py_str("sun cream"), py_str("suncream"), py_str("sunscreen"), py_str("sunblock"), py_str("moisturiser"), py_str("moisturizer"), py_str("face cream"), py_str("hand cream"), py_str("body cream"), py_str("eczema cream"), py_str("nappy cream"), py_str("shaving cream"), py_str("cold cream"), py_str("ice cream"), py_str("eye cream"), py_str("facial cream"), py_str("spot cream"), py_str("out of date"), py_str("past the date"), py_str("past its date"), py_str("past their date"), py_str("use by"), py_str("use by date"), py_str("best before"), py_str("date on the lid"), py_str("went out of date"), py_str("starter"), py_str("starters"), py_str("sourdough"), py_str("ginger bug"), py_str("kombucha scoby")]) }
    fn __data29() -> Py { py_tuple(vec![py_str("out of date"), py_str("past its date"), py_str("past the date"), py_str("past their date"), py_str("use by"), py_str("use by date"), py_str("use-by"), py_str("best before"), py_str("best-before"), py_str("gone past"), py_str("after the date"), py_str("date on it"), py_str("date on the"), py_str("still edible")]) }
    fn __data30() -> Py { py_tuple(vec![py_str("is my"), py_str("is it"), py_str("is this"), py_str("is that"), py_str("are my"), py_str("are these"), py_str("my"), py_str("still"), py_str("ok"), py_str("safe"), py_str("good"), py_str("edible"), py_str("eat"), py_str("keep"), py_str("bin"), py_str("throw"), py_str("use")]) }
    fn __data31() -> Py { py_tuple(vec![py_str("out of milk"), py_str("out of bread"), py_str("out of rice"), py_str("out of chicken"), py_str("out of cheese"), py_str("out of eggs"), py_str("out of sugar"), py_str("out of flour"), py_str("run out"), py_str("ran out"), py_str("none left"), py_str("havent got"), py_str("recipe"), py_str("how do i cook"), py_str("how long to cook"), py_str("how long do i cook"), py_str("freeze"), py_str("defrost"), py_str("reheat"), py_str("sell by"), py_str("expiry"), py_str("passport"), py_str("licence"), py_str("license"), py_str("driving licence"), py_str("mot"), py_str("insurance"), py_str("warranty"), py_str("guarantee"), py_str("ticket"), py_str("tickets"), py_str("coupon"), py_str("voucher"), py_str("gift card"), py_str("card"), py_str("cards"), py_str("debit card"), py_str("credit card"), py_str("car is out of date"), py_str("car out of date"), py_str("car is past its"), py_str("van is out of date"), py_str("van out of date"), py_str("vehicle is out of date"), py_str("bike is out of date"), py_str("tyre"), py_str("tyres"), py_str("tire"), py_str("tires"), py_str("membership"), py_str("subscription"), py_str("battery"), py_str("batteries"), py_str("smoke alarm"), py_str("alarm"), py_str("fire extinguisher"), py_str("medicine"), py_str("medication"), py_str("tablets"), py_str("inhaler"), py_str("epipen"), py_str("food bank"), py_str("sell by date"), py_str("sell-by"), py_str("expiry date"), py_str("expires")]) }
    fn __data32() -> Py { py_tuple(vec![py_str("smells off"), py_str("smelt off"), py_str("smells funny"), py_str("gone off"), py_str("has gone off"), py_str("went off"), py_str("smells wrong"), py_str("smells a bit off"), py_str("gone bad"), py_str("has gone bad"), py_str("looks off"), py_str("smells iffy"), py_str("smells dodgy")]) }
    fn __data33() -> Py { py_tuple(vec![py_str("can i cook"), py_str("can i eat"), py_str("cook it"), py_str("eat it"), py_str("still cook"), py_str("still eat"), py_str("ok to eat"), py_str("safe to eat"), py_str("is it ok"), py_str("is it safe"), py_str("use it"), py_str("bin it"), py_str("throw it"), py_str("keep it"), py_str("should i")]) }
    fn __data34() -> Py { py_tuple(vec![py_str("kitchen"), py_str("room"), py_str("rooms"), py_str("house"), py_str("flat"), py_str("home"), py_str("bin"), py_str("bins"), py_str("drain"), py_str("drains"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("microwave"), py_str("dishwasher"), py_str("washing machine"), py_str("tumble dryer"), py_str("sink"), py_str("cupboard"), py_str("car"), py_str("cars"), py_str("van"), py_str("garage"), py_str("loft"), py_str("shed"), py_str("sofa"), py_str("carpet"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("puppy"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("smoke"), py_str("damp"), py_str("mould"), py_str("mold"), py_str("musty"), py_str("gas"), py_str("paint")]) }
    fn __data35() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("how do i cook"), py_str("boil"), py_str("boiling"), py_str("boiled"), py_str("poach"), py_str("poached"), py_str("scrambl"), py_str("fry"), py_str("frying"), py_str("fried"), py_str("omelette"), py_str("bake"), py_str("baking"), py_str("meringue"), py_str("mayonnaise"), py_str("white"), py_str("whites"), py_str("yolk"), py_str("yolks"), py_str("shell"), py_str("cracked"), py_str("crack"), py_str("float"), py_str("freshness"), py_str("how long"), py_str("keep"), py_str("keeps"), py_str("box"), py_str("date")]) }
    fn __data36() -> Py { py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("cat"), py_str("cats"), py_str("kitten"), py_str("pet"), py_str("pets"), py_str("vet"), py_str("pudding"), py_str("risotto"), py_str("paella"), py_str("sushi"), py_str("cook"), py_str("cooking"), py_str("boil"), py_str("boiling"), py_str("how much water"), py_str("basmati"), py_str("brown rice"), py_str("wild rice"), py_str("microwave")]) }
    fn __data37() -> Py { py_tuple(vec![py_str("raw"), py_str("eat"), py_str("eating"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("washed"), py_str("wash"), py_str("cook"), py_str("cooked"), py_str("cooking"), py_str("salad"), py_str("pregnant"), py_str("pregnancy"), py_str("should i"), py_str("can i"), py_str("are they"), py_str("in a sandwich")]) }
    fn __data38() -> Py { py_tuple(vec![py_str("brussels"), py_str("brussel"), py_str("christmas"), py_str("roast"), py_str("roasting"), py_str("boil"), py_str("boiling"), py_str("steam"), py_str("steaming"), py_str("how long"), py_str("minutes"), py_str("how do i cook"), py_str("how to cook"), py_str("grow"), py_str("growing"), py_str("grown"), py_str("seed"), py_str("seeds"), py_str("garden"), py_str("window sill"), py_str("windowsill"), py_str("jar"), py_str("potato"), py_str("potatoes"), py_str("get rid of"), py_str("pest"), py_str("aphid")]) }
    fn __data39() -> Py { py_tuple(vec![py_str("landed"), py_str("landing"), py_str("land"), py_str("on my food"), py_str("on my sandwich"), py_str("on my dinner"), py_str("on my drink"), py_str("in my drink"), py_str("in my food"), py_str("in my tea"), py_str("crawling"), py_str("walked on"), py_str("sat on")]) }
    fn __data40() -> Py { py_tuple(vec![py_str("airport"), py_str("security"), py_str("hand luggage"), py_str("check in"), py_str("flight"), py_str("flights"), py_str("plane"), py_str("heathrow"), py_str("gatwick"), py_str("holiday"), py_str("abroad"), py_str("how do i get rid of"), py_str("get rid of flies"), py_str("infestation"), py_str("traps"), py_str("trap"), py_str("kite"), py_str("flag"), py_str("butterfly"), py_str("butterflies"), py_str("drone"), py_str("dragonfl"), py_str("firefl"), py_str("fly a"), py_str("flying to"), py_str("flying out"), py_str("business"), py_str("screen"), py_str("net curtains"), py_str("on me"), py_str("landed on me"), py_str("landing on me"), py_str("on my arm"), py_str("on my leg"), py_str("on my face"), py_str("on my neck"), py_str("on my hand"), py_str("on my head"), py_str("keeps landing"), py_str("stinging"), py_str("stung"), py_str("sting"), py_str("chasing me"), py_str("buzzing"), py_str("swat")]) }
    fn __data41() -> Py { py_tuple(vec![py_str("leftovers"), py_str("leftover"), py_str("left over"), py_str("curry"), py_str("stew"), py_str("casserole"), py_str("chilli"), py_str("tagine"), py_str("goulash"), py_str("bolognese"), py_str("lasagne"), py_str("risotto"), py_str("soup"), py_str("stew"), py_str("rice"), py_str("shepherds pie"), py_str("cottage pie"), py_str("the sauce"), py_str("the broth"), py_str("the dish")]) }
    fn __data42() -> Py { py_tuple(vec![py_str("a week"), py_str("one week"), py_str("two weeks"), py_str("three weeks"), py_str("a fortnight"), py_str("fortnight"), py_str("weeks"), py_str("a month"), py_str("one month"), py_str("two months"), py_str("months"), py_str("week old"), py_str("weeks old"), py_str("last week"), py_str("the week before"), py_str("a while"), py_str("ages")]) }
    fn __data43() -> Py { py_tuple(vec![py_str("use by"), py_str("use-by"), py_str("best before"), py_str("sell by"), py_str("date on"), py_str("supermarket"), py_str("shop"), py_str("frozen"), py_str("freezer"), py_str("defrost"), py_str("thaw"), py_str("refreeze"), py_str("refroze"), py_str("reheat"), py_str("reheating"), py_str("microwave"), py_str("recipe"), py_str("what can i make"), py_str("what can i cook"), py_str("meal ideas"), py_str("how do i make"), py_str("how to make"), py_str("cook it"), py_str("how long do i cook"), py_str("fridge is"), py_str("fridge not"), py_str("not cold"), py_str("stopped working"), py_str("warm in the fridge"), py_str("temperature"), py_str("thermometer"), py_str("power cut"), py_str("smells"), py_str("sour"), py_str("slimy"), py_str("mould"), py_str("mouldy"), py_str("mold"), py_str("moldy"), py_str("gone off"), py_str("go off"), py_str("off"), py_str("rice"), py_str("chicken"), py_str("meat"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("fish"), py_str("prawns"), py_str("milk"), py_str("cream"), py_str("yoghurt"), py_str("yogurt"), py_str("cheese"), py_str("eggs"), py_str("egg"), py_str("pasta"), py_str("bread"), py_str("baby"), py_str("babies"), py_str("toddler"), py_str("child"), py_str("children"), py_str("dog"), py_str("cat"), py_str("pregnant"), py_str("each month"), py_str("every month"), py_str("each week"), py_str("every week"), py_str("each year"), py_str("every year"), py_str("per week"), py_str("per month"), py_str("per year"), py_str("each payday"), py_str("left over each"), py_str("left over every"), py_str("left over per"), py_str("leftover each"), py_str("leftover every"), py_str("leftover per"), py_str("leftover a month"), py_str("leftover this month"), py_str("leftovers each"), py_str("leftovers every")]) }
    fn __data44() -> Py { py_tuple(vec![py_str("leftovers"), py_str("leftover"), py_str("left over"), py_str("left in the fridge"), py_str("whats left"), py_str("what's left"), py_str("the rest of the"), py_str("curry"), py_str("stew"), py_str("casserole"), py_str("chilli"), py_str("tagine"), py_str("goulash"), py_str("bolognese"), py_str("shepherds pie"), py_str("cottage pie"), py_str("lasagne"), py_str("risotto")]) }
    fn __data45() -> Py { py_tuple(vec![py_str("ok"), py_str("okay"), py_str("is it ok"), py_str("is that ok"), py_str("safe"), py_str("is it safe"), py_str("safe to eat"), py_str("eat"), py_str("eat it"), py_str("still"), py_str("still good"), py_str("still ok"), py_str("next day"), py_str("the next day"), py_str("tomorrow"), py_str("fridge"), py_str("gone off"), py_str("go off"), py_str("went off"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("sour"), py_str("slimy"), py_str("days"), py_str("day"), py_str("how long"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("bin"), py_str("throw"), py_str("been in"), py_str("sat in"), py_str("made on"), py_str("cooked it on"), py_str("yesterday"), py_str("last night"), py_str("this morning"), py_str("a week"), py_str("one week"), py_str("two weeks"), py_str("three weeks"), py_str("a fortnight"), py_str("fortnight"), py_str("weeks"), py_str("a month"), py_str("one month"), py_str("two months"), py_str("months"), py_str("week old"), py_str("weeks old"), py_str("last week"), py_str("the week before"), py_str("a while"), py_str("ages")]) }
    fn __data46() -> Py { py_tuple(vec![py_str("frozen"), py_str("defrost"), py_str("defrosting"), py_str("thaw"), py_str("thawing"), py_str("left out"), py_str("overnight"), py_str("all night"), py_str("all day"), py_str("room temperature"), py_str("sat out"), py_str("sitting out"), py_str("how do i keep"), py_str("how do i store"), py_str("how to keep"), py_str("how to store"), py_str("how can i keep"), py_str("how should i keep"), py_str("refreeze"), py_str("refroze"), py_str("re-freeze"), py_str("freeze again"), py_str("reheat it twice"), py_str("how many times can i reheat"), py_str("how do i reheat"), py_str("how to reheat"), py_str("best way to reheat"), py_str("why"), py_str("what causes"), py_str("cause"), py_str("mouldy"), py_str("mould"), py_str("moldy"), py_str("mold")]) }
    fn __data47() -> Py { py_tuple(vec![py_str("chicken"), py_str("turkey"), py_str("meat"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("steak"), py_str("mince"), py_str("fish"), py_str("salmon"), py_str("cod"), py_str("prawns"), py_str("prawn"), py_str("ham"), py_str("bacon"), py_str("sausage"), py_str("egg"), py_str("eggs"), py_str("rice"), py_str("milk"), py_str("cheese"), py_str("yogurt"), py_str("yoghurt"), py_str("cream"), py_str("pasta"), py_str("pizza"), py_str("bread"), py_str("potato"), py_str("potatoes"), py_str("tomato"), py_str("tomatoes"), py_str("how long does"), py_str("how long do"), py_str("how long can i"), py_str("how long will"), py_str("how long should"), py_str("how long to"), py_str("simmer"), py_str("simmering"), py_str("boil"), py_str("boiling"), py_str("fry"), py_str("frying"), py_str("bake"), py_str("baking"), py_str("roast"), py_str("roasting"), py_str("grill"), py_str("grilling"), py_str("oven"), py_str("slow cooker"), py_str("air fryer"), py_str("minutes"), py_str("start to finish"), py_str("how do i cook"), py_str("how to cook"), py_str("how long do i cook"), py_str("how many days"), py_str("what can i make"), py_str("what can i eat"), py_str("what can i have"), py_str("what to eat"), py_str("what goes with"), py_str("goes with"), py_str("serve with"), py_str("side dish"), py_str("side for"), py_str("what can i do with"), py_str("recipe"), py_str("recipes"), py_str("use up"), py_str("using up"), py_str("idea"), py_str("ideas"), py_str("meal ideas"), py_str("dinner ideas"), py_str("how do i make"), py_str("how to make"), py_str("how do i cook"), py_str("how to cook"), py_str("how long do i cook"), py_str("how long to cook")]) }
    fn __data48() -> Py { py_tuple(vec![py_str("ok"), py_str("okay"), py_str("safe"), py_str("eat"), py_str("eat it"), py_str("still"), py_str("next day"), py_str("tomorrow"), py_str("fridge"), py_str("can i have"), py_str("have it"), py_str("have some"), py_str("can i still have"), py_str("can i eat it"), py_str("gone off"), py_str("go off"), py_str("went off"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("sour"), py_str("slimy"), py_str("days"), py_str("day"), py_str("how long"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("bin"), py_str("throw"), py_str("should i"), py_str("made it"), py_str("made on"), py_str("cooked it on"), py_str("been in"), py_str("sat in"), py_str("a week"), py_str("two weeks"), py_str("a month"), py_str("last week")]) }
    fn __data49() -> Py { py_tuple(vec![py_str("dropped"), py_str("drop"), py_str("fell"), py_str("fallen"), py_str("picked it up"), py_str("pick it up"), py_str("blew on it"), py_str("kiss it better"), py_str("true"), py_str("real"), py_str("myth"), py_str("a thing"), py_str("still ok"), py_str("safe"), py_str("eat it")]) }
    fn __data50() -> Py { py_tuple(vec![py_str("phone"), py_str("screen"), py_str("keys"), py_str("won't turn on"), py_str("wont turn on"), py_str("signal"), py_str("cracked"), py_str("dropped it"), py_str("dropped my phone"), py_str("dropped the"), py_str("dropped a"), py_str("dropped out"), py_str("prices dropped"), py_str("temperature dropped"), py_str("how do i stop"), py_str("my back"), py_str("my hip"), py_str("fell over"), py_str("fell down"), py_str("fell off the ladder"), py_str("fell off my bike"), py_str("fell out of bed")]) }
    fn __data22() -> Py { py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("chicken-food-poisoning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("chicken"), py_str("chickens"), py_str("turkey")]), g_POISONING_WORDS()])); __args.push(py_str("Yes - chicken is the commonest source of food poisoning in Britain, because campylobacter lives on the raw meat and a drop of the juice is enough to make you ill. Heat is what kills it: cook chicken right through to 74C in the thickest part, don't wash it raw (that splashes the bacteria round the sink), keep it away from anything you eat uncooked, and wash your hands, the board and the knife afterwards. Symptoms come 1 to 3 days later, usually cramps and diarrhoea for about a week, and the thing that matters is fluid; blood in it, a high fever or a week with no improvement is a GP.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g_POISONING_NOT(), &g_DIET_WORDS()), &g__NOT_THIS()), &g_OFF_THE_THING()), &g_RAW_WORDS()), &__data23())), ("bias".to_string(), Py::Int(6))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("plant-leaves-poisonous")); __args.push(py_tuple(vec![py_tuple(vec![py_str("tomato leaves"), py_str("tomato leaf"), py_str("tomato plant leaves"), py_str("tomato stems"), py_str("potato leaves"), py_str("potato leaf"), py_str("potato shoots"), py_str("potato sprouts"), py_str("rhubarb leaves"), py_str("rhubarb leaf"), py_str("aubergine leaves"), py_str("courgette leaves")]), __data24()])); __args.push(py_str("The leaves and stems of tomatoes, potatoes and rhubarb contain solanine and oxalic acid, which are mild poisons: a stray leaf in a salad will do you no harm, but a plateful brings a stomach ache, sickness and diarrhoea, and cooking does not destroy it. So don't eat them and don't brew them as tea, wash your hands after handling the plants, and keep pets and small children away from the leaves. The fruit, the tubers and the rhubarb stalks are the parts that are food.")); __kws.extend(vec![("avoid".to_string(), __data25()), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-chicken")); __args.push(py_tuple(vec![py_str("chicken"), g_OFF_WORDS()])); __args.push(py_str("Chicken is the one to be strict about: a sour or ammonia smell, a tacky or slimy feel, or any change of colour means bin it, and cooking it will not make it safe. Raw chicken keeps 1 to 2 days in the fridge and cooked chicken 3; if it has sat out warm for more than 2 hours, that is enough on its own.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g_UNDERCOOKED_PINK()), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING())), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), g_SPOILAGE_BIAS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-meat")); __args.push(py_tuple(vec![py_tuple(vec![py_str("meat"), py_str("beef"), py_str("lamb"), py_str("pork"), py_str("steak"), py_str("mince"), py_str("joint")]), g_OFF_WORDS()])); __args.push(py_str("Red meat is more forgiving than chicken: a grey-brown colour on the outside of a chop is just air and it is fine, but a sour or putrid smell, a sticky tacky surface or green patches means bin it. Raw beef, lamb and pork keep 3 to 5 days in the fridge, mince only 1 to 2.")); __kws.extend(vec![("guard".to_string(), py_func("_mince_keeping_is_its_own_row", |args: &[Py]| _mince_keeping_is_its_own_row(args))), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g_UNDERCOOKED_PINK()), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING())), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), g_SPOILAGE_BIAS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-fish")); __args.push(py_tuple(vec![__data26(), g_OFF_WORDS()])); __args.push(py_str("Fish goes off faster than anything else and does not forgive it: it should smell of the sea, so an ammonia or fishy-sharp smell, flesh that stays dented when you press it, or grey gills means bin it. Use it the day you buy it or freeze it that day.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g_RAW_WORDS()), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING()), &__data27())), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), op_add(&g_SPOILAGE_BIAS(), &Py::Int(1)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-ham")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ham"), py_str("bacon"), py_str("sausage"), py_str("sausages"), py_str("salami"), py_str("chorizo")]), g_OFF_WORDS()])); __args.push(py_str("Cured meat is not immune: a slimy feel, a sour or musty smell, or grey-green patches means bin it. Ham keeps about 3 days once the pack is open, dry-cured bacon up to a week, and sausages 2 to 3 days.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g_RAW_WORDS()), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING())), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), g_SPOILAGE_BIAS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-yogurt")); __args.push(py_tuple(vec![py_tuple(vec![py_str("yogurt"), py_str("yoghurt"), py_str("cream"), py_str("creme fraiche"), py_str("soured cream")]), g_OFF_WORDS()])); __args.push(py_str("Bin it if it smells sour or fizzy rather than tangy, or if there is any mould on the surface; a little watery separation on top is fine, so stir it back in. Cream keeps about 5 days once opened, and yogurt a week past its date if it has been in the fridge.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING()), &__data28())), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), g_SPOILAGE_BIAS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-date-rule")); __args.push(py_tuple(vec![__data29(), __data30()])); __args.push(py_str("Two dates mean two things: use-by is about safety, so the food goes in the bin after it, even if it looks and smells fine; best-before is about quality, so dry rice, pasta, flour and sugar are fine for months past it, and a tin for years. For anything fresh, smell it and touch it: a sour smell, a slimy surface or mould means bin it, and cooked food is 2 days in the fridge whatever the label said.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING()), &__data31())), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), op_neg(&Py::Int(3)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-unnamed")); __args.push(py_tuple(vec![__data32(), __data33()])); __args.push(py_str("If it smells sour, or wrong rather than just strong, bin it - cooking does not make an off food safe. Cooked rice, cooked meat and anything with a sauce are the strict ones; hard cheese and dry bread are the ones you can judge by looking.")); __kws.extend(vec![("avoid".to_string(), op_add(&__data34(), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS"))), ("ask".to_string(), g_OFF_ASKS()), ("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("_names_no_food", |args: &[Py]| _names_no_food(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-unripe")); __args.push(py_tuple(vec![g_UNRIPE_WORDS(), op_add(&g_OFF_WORDS(), &py_tuple(vec![py_str("eat"), py_str("safe"), py_str("ok"), py_str("okay"), py_str("fine"), py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("harm"), py_str("ill"), py_str("sick"), py_str("bad for")]))])); __args.push(py_str("Unripe fruit and veg are safe to eat - hard, sour and starchy rather than sweet, not poisonous - and a day or two on the counter, or in a paper bag with a banana, ripens most of them; cooking does the rest, as fried green tomatoes and green banana show. A green potato is the one exception: that green is solanine, it does not cook out, so bin it.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_DIET_WORDS(), &g__NOT_THIS()), &pyrt::xrefs::xget("knowledge.cooking", "PET_LINE_WORDS")), &g_OFF_THE_THING()), &g_UNRIPE_NOT())), ("ask".to_string(), op_add(&g_OFF_ASKS(), &py_tuple(vec![py_str("eat"), py_str("safe"), py_str("poison"), py_str("poisonous"), py_str("toxic"), py_str("harm"), py_str("ill"), py_str("sick")]))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("raw-vegetable")); __args.push(py_tuple(vec![call_builtin("tuple", &[g__RAW_VEGETABLE_NAMES()]).unwrap_or_else(|e| exc::raise(e)), py_tuple(vec![py_str("raw"), py_str("uncooked"), py_str("not cooked"), py_str("eat raw"), py_str("eat it raw"), py_str("eat them raw"), py_str("eat it uncooked"), py_str("without cooking"), py_str("safely eat raw"), py_str("can you eat raw")])])); __args.push(py_func("_raw_vegetable", |args: &[Py]| _raw_vegetable(args))); __kws.extend(vec![("avoid".to_string(), g__RAW_VEGETABLE_NOT()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("eggs-food-poisoning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("egg"), py_str("eggs"), py_str("egg white"), py_str("egg yolk"), py_str("runny egg"), py_str("runny eggs")]), g_POISONING_WORDS()])); __args.push(py_str("Eggs are the second commonest source after chicken, and the germ is salmonella, which sits on the shell and sometimes in the egg. In Britain a British Lion-marked egg is fine raw or runny, even in pregnancy, because those flocks are vaccinated; an unmarked egg, a duck egg or anything from abroad is the one to cook through. Keep eggs cold, don't wash them, and bin one that is cracked and dirty. Cramps and diarrhoea come 12 to 72 hours later and usually pass in a few days; fluid is the thing that matters.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&g_POISONING_NOT(), &g_DIET_WORDS()), &g__NOT_THIS()), &g_OFF_THE_THING()), &g_RAW_WORDS()), &__data35())), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rice-food-poisoning")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rice"), py_str("rices")]), g_POISONING_WORDS()])); __args.push(py_str("Yes - rice is the leftover worth being strict about. The germ is bacillus cereus, whose spores survive cooking, so rice left sitting warm grows them and reheating does not always clear what they made. Cool it fast (spread it on a tray or rinse it under cold water), box it within the hour, keep it in the fridge no more than a day, and reheat it once, piping hot all the way through. Cramps and vomiting inside a few hours of eating are the usual sign; fluid is what matters, and a day of it or blood in it is a GP.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&op_add(&call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_POISONING_NOT()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&w.clone(), &py_str("rice")))) {  __out.push(w.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)), &g_DIET_WORDS()), &g__NOT_THIS()), &g_OFF_THE_THING()), &g_RAW_WORDS()), &__data36())), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("raw-sprouts")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sprouts"), py_str("sprout"), py_str("alfalfa"), py_str("bean sprouts"), py_str("beansprouts"), py_str("mung beans"), py_str("radish sprouts")]), __data37()])); __args.push(py_str("Bean and alfalfa sprouts are the one salad thing to be careful with: they are grown warm and damp, which is exactly what bacteria like, and rinsing does not remove what has got inside the seed. Eat them cooked - stir-fried or dropped into soup for a minute - and if you want them raw, buy a bag that says ready to eat and keep it cold. Pregnant, elderly, or on treatment that lowers your immunity, cook them: the same advice covers a raw mung-bean salad.")); __kws.extend(vec![("avoid".to_string(), __data38()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("fly-on-food")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fly"), py_str("flies"), py_str("bluebottle"), py_str("bluebottles"), py_str("wasp"), py_str("wasps")]), __data39()])); __args.push(py_str("Brush it off and eat the food: a fly that lands for a moment leaves little behind, and the dose is what matters. If it has been walking about on it, cut away that patch or rinse what you can, and tip out a drink it fell into. What is worth avoiding is food a fly has been sitting on for a while, or anything from a bin or off the floor, because flies feed on waste and carry it on their legs.")); __kws.extend(vec![("avoid".to_string(), __data40()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("past-it-in-the-fridge")); __args.push(py_tuple(vec![__data41(), __data42()])); __args.push(py_str("Past it, I'm afraid - bin it. A cooked dish keeps 3 to 4 days in the fridge, and rice only 1, so a dish from last week is well beyond that, and reheating it does not make it safe: the toxins that build up in old food do not all break down with heat and they do not all smell. Scrape it out, wash the box with hot soapy water, and next time freeze the half you will not eat within the three days.")); __kws.extend(vec![("avoid".to_string(), __data43()), ("bias".to_string(), Py::Int(3)), ("guard".to_string(), py_func("_not_about_money", |args: &[Py]| _not_about_money(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("still-leftovers")); __args.push(py_tuple(vec![__data44(), __data45()])); __args.push(py_func("_the_leftovers_answer", |args: &[Py]| _the_leftovers_answer(args))); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&op_add(&g_DIET_WORDS(), &g_UNDERCOOKED_PINK()), &__data46()), &__data47())), ("ask".to_string(), __data48()), ("bias".to_string(), Py::Int(2)), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = _not_a_recipe_now(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _not_a_fault_in_the_dish(&[understood.clone()]) } } }))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("five-second-rule")); __args.push(py_tuple(vec![py_tuple(vec![py_str("floor"), py_str("ground"), py_str("carpet"), py_str("worktop"), py_str("pavement"), py_str("five second rule"), py_str("five seconds rule"), py_str("5 second rule"), py_str("five-second rule"), py_str("five second"), py_str("five seconds"), py_str("5 second")]), __data49()])); __args.push(py_str("The five-second rule is not real: bacteria stick the moment food touches the floor, and it is the wetness that decides how much, not the seconds. A dry biscuit or a piece of toast is a small risk - brush it off and use your judgement - but anything damp, sticky or eaten with your hands should go in the bin, and anything dropped outside or in a bathroom always should. If it did go down, it is a wait-and-see, not a hospital trip: a healthy stomach usually deals with it.")); __kws.extend(vec![("avoid".to_string(), __data50()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.ENTRIES"); }
    s_ENTRIES(__data22());
    fn __data51() -> Py { py_tuple(vec![py_str("out of the house"), py_str("out of my house"), py_str("out of the flat"), py_str("out of my flat"), py_str("out of the kitchen"), py_str("out of the room"), py_str("house smells"), py_str("flat smells"), py_str("kitchen smells"), py_str("room smells"), py_str("smell wont go"), py_str("smells wont go"), py_str("wont go away"), py_str("lingering smell"), py_str("stinks in here")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._ROOM_ODOUR"); }
    s__ROOM_ODOUR(__data51());
    let __iter52 = py_iter_or_raise(&g_ENTRIES());
    for __item in __iter52 {
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._item"); }
    s__item(__item.clone());
    let _ = py_setattr(&g__item(), "avoid", op_add(&gattr(&g__item(), "avoid"), &g__ROOM_ODOUR()));
    }
    fn __data53() -> Py { py_tuple(vec![py_str("mussels"), py_str("mussel"), py_str("oysters"), py_str("oyster"), py_str("clams"), py_str("clam"), py_str("cockles"), py_str("scallops"), py_str("shellfish"), py_str("lobster"), py_str("crab"), py_str("whelks"), py_str("razor clams")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._SHELLFISH"); }
    s__SHELLFISH(__data53());
    fn __data54() -> Py { py_tuple(vec![py_str("without an r"), py_str("without an 'r'"), py_str("without the letter r"), py_str("month with an r"), py_str("months with an r"), py_str("only in months"), py_str("in an r month"), py_str("no r in the month"), py_str("r in the month"), py_str("summer months"), py_str("in may"), py_str("in june"), py_str("in july"), py_str("in august"), py_str("in september"), py_str("spawning"), py_str("ok to eat"), py_str("okay to eat"), py_str("alright to eat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._R_MONTHS"); }
    s__R_MONTHS(__data54());
    fn __data55() -> Py { py_tuple(vec![py_str("pregnant"), py_str("pregnancy"), py_str("expecting"), py_str("baby"), py_str("toddler"), py_str("child"), py_str("weaning"), py_str("recipe"), py_str("white wine"), py_str("garlic"), py_str("restaurant"), py_str("frozen"), py_str("freeze"), py_str("how long do they keep"), py_str("dog"), py_str("cat"), py_str("open"), py_str("opened"), py_str("unopened"), py_str("didnt open"), py_str("didn't open"), py_str("stays open"), py_str("still alive"), py_str("alive"), py_str("dead"), py_str("gone off"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("bad ones"), py_str("keep in the fridge"), py_str("fridge")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.ENTRIES"); }
    s_ENTRIES(op_add(&g_ENTRIES(), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("shellfish-r-rule")); __args.push(py_tuple(vec![g__SHELLFISH(), g__R_MONTHS()])); __args.push(py_str("The rule about only eating shellfish in months with an r is about the days before fridges: May to August are the warm months, when shellfish spoil fastest and when mussels spawn and go watery. It is not a law now - farmed mussels and rope-grown ones are sold all year and are safe - so buy from a fishmonger or a shop with a busy counter, keep them cold, and use the tests that actually tell you: a live mussel is shut, or shuts when you tap it, and one that stays open goes in the bin; cook them in a covered pan for 5 to 8 minutes and throw away any that are still shut afterwards. Eat them the day you buy them, and never eat them raw.")); __kws.extend(vec![("avoid".to_string(), __data55()), ("bias".to_string(), Py::Int(4))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])));
    fn __data56() -> Py { py_tuple(vec![py_str("spinach"), py_str("baby spinach"), py_str("lettuce"), py_str("lettuce leaves"), py_str("little gem"), py_str("kale"), py_str("curly kale"), py_str("salad leaves"), py_str("salad leaf"), py_str("bag of salad"), py_str("salad bag"), py_str("rocket"), py_str("watercress"), py_str("cress"), py_str("chard"), py_str("spring greens"), py_str("leafy greens"), py_str("bok choy"), py_str("pak choi"), py_str("collard greens"), py_str("cavolo nero"), py_str("greens")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._LEAFY_GREENS"); }
    s__LEAFY_GREENS(__data56());
    fn __data57() -> Py { py_tuple(vec![py_str("yellow"), py_str("yellowed"), py_str("yellowing"), py_str("gone yellow"), py_str("going yellow"), py_str("turning yellow"), py_str("turned yellow"), py_str("gone a bit yellow"), py_str("slightly yellow"), py_str("gone slimy"), py_str("slimy"), py_str("slime"), py_str("brown"), py_str("browning"), py_str("gone brown"), py_str("turned brown"), py_str("black spots"), py_str("mushy"), py_str("gone soft"), py_str("yellow leaves"), py_str("yellow leaf"), py_str("yellowed leaves"), py_str("yellowing leaves"), py_str("brown leaves"), py_str("slimy leaves"), py_str("soggy"), py_str("soggy leaves"), py_str("use it"), py_str("eat it"), py_str("still use"), py_str("still eat")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._LEAFY_SIGNS"); }
    s__LEAFY_SIGNS(__data57());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._LEAFY_SHOWS"); }
    s__LEAFY_SHOWS(stdlib_call("re", "compile", &[py_str("\\b(?:yellow\\w*|brown\\w*|black\\w*|slimy|slime|mushy|sour\\w*|smell\\w*|smelly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._CABBAGES"); }
    s__CABBAGES(py_tuple(vec![py_str("cabbage"), py_str("white cabbage"), py_str("green cabbage"), py_str("red cabbage"), py_str("savoy"), py_str("savoy cabbage"), py_str("hispi"), py_str("pointed cabbage")]));
    fn __data58() -> Py { py_tuple(vec![py_str("yellow"), py_str("yellowed"), py_str("yellowing"), py_str("gone yellow"), py_str("going yellow"), py_str("turning yellow"), py_str("yellow leaves"), py_str("yellow leaf"), py_str("brown"), py_str("browning"), py_str("gone brown"), py_str("limp"), py_str("wilting"), py_str("wilted"), py_str("soft"), py_str("outer leaves"), py_str("slimy"), py_str("smelly")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok._CABBAGE_SIGNS"); }
    s__CABBAGE_SIGNS(__data58());
    fn __data59() -> Py { py_tuple(vec![py_str("garden"), py_str("gardening"), py_str("allotment"), py_str("plot"), py_str("compost"), py_str("feed"), py_str("fertiliser"), py_str("fertilizer"), py_str("blight"), py_str("grow"), py_str("growing"), py_str("grown"), py_str("plant"), py_str("plants"), py_str("planting"), py_str("seed"), py_str("seeds"), py_str("seedlings"), py_str("sow"), py_str("sowing"), py_str("soil"), py_str("hydroponics"), py_str("tomato"), py_str("courgette"), py_str("why are my leaves"), py_str("recipe"), py_str("recipes"), py_str("how do i cook"), py_str("how to cook"), py_str("how long does"), py_str("how long do"), py_str("how long can i"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("freeze"), py_str("frozen"), py_str("revive"), py_str("reviving"), py_str("bring it back"), py_str("crisp"), py_str("crispness"), py_str("fresh again"), py_str("use up"), py_str("using up"), py_str("what can i do with"), py_str("what can i make"), py_str("ideas"), py_str("cook"), py_str("cooking"), py_str("cooked"), py_str("pan"), py_str("boil"), py_str("boiling"), py_str("steam"), py_str("steaming"), py_str("wilt it"), py_str("wilted in a pan")]) }
    fn __data60() -> Py { py_tuple(vec![py_str("garden"), py_str("gardening"), py_str("allotment"), py_str("plot"), py_str("compost"), py_str("feed"), py_str("fertiliser"), py_str("fertilizer"), py_str("blight"), py_str("grow"), py_str("growing"), py_str("grown"), py_str("plant"), py_str("plants"), py_str("planting"), py_str("seed"), py_str("seeds"), py_str("seedlings"), py_str("sow"), py_str("sowing"), py_str("soil"), py_str("flower"), py_str("flowers"), py_str("flowering"), py_str("bolted"), py_str("bolting"), py_str("recipe"), py_str("recipes"), py_str("slaw"), py_str("coleslaw"), py_str("pickle"), py_str("pickling"), py_str("ferment"), py_str("sauerkraut"), py_str("how long does"), py_str("how long do"), py_str("how long can i"), py_str("keep"), py_str("keeps"), py_str("last"), py_str("lasts"), py_str("freeze"), py_str("frozen"), py_str("how do i cook"), py_str("how to cook"), py_str("roast")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.still_ok.ENTRIES"); }
    s_ENTRIES(op_add(&g_ENTRIES(), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("leafy-greens-yellow")); __args.push(py_tuple(vec![g__LEAFY_GREENS(), g__LEAFY_SIGNS()])); __args.push(py_str("Check for slime before you look at the colour: a yellow or floppy leaf is age rather than poison - light and time break the green down - so pick those out and use the rest. Slimy, sour-smelling or brown-and-soft leaves are not safe, and once one leaf is slimy the bag is best binned, because the slime is bacteria and rinsing does not take it off. Wilting on its own is only water loss, so 10 minutes in iced water brings the crunch back, and limp-but-dry leaves are perfect in a soup, a curry or a stir-fry. Keep the bag in the salad drawer with a piece of kitchen paper in it and eat it within 3 to 5 days of opening.")); __kws.extend(vec![("avoid".to_string(), __data59()), ("guard".to_string(), py_func("_the_leaf_shows_a_colour", |args: &[Py]| _the_leaf_shows_a_colour(args))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("cabbage-yellow-outer-leaves")); __args.push(py_tuple(vec![g__CABBAGES(), g__CABBAGE_SIGNS()])); __args.push(py_str("Yellow or brown outer leaves on a cabbage are just age: peel off the ones that have gone, trim the dried stem end, and the firm head inside is still good - a whole cabbage keeps a month or more in the fridge, and a cut half keeps a couple of weeks wrapped. The one to bin is a head that is slimy or sour-smelling right through, or soft all the way to the middle. If only the outside looks tired, it is a stir-fry or a soup rather than the bin.")); __kws.extend(vec![("avoid".to_string(), __data60()), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }])));
}
pub fn _raw_vegetable(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut name: Py = Py::None;
    let mut plural: Py = Py::None;
    let mut template: Py = Py::None;
    let mut text: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("The verdict for the vegetable the line names, in the group it belongs to.\n\n    A raw vegetable question is answered by which vegetable it is, not by which way it was asked:\n    \"can i eat raw potato\" and \"raw broccoli\" are the same shape and want opposite answers, so the\n    reply is chosen from the table above and only the name comes from the line.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "replace", &[py_str("'s"), py_str(" ")])), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g__RAW_VEGETABLE_NAMES());
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &name.clone())))) {
    continue;
    }
    let __unpk2 = py_unpack(&geti(&g__RAW_VEG_ONE(), &name.clone()), 2, None);
    _u = __unpk2[0].clone();
    template = __unpk2[1].clone();
    plural = { let __b = mcall(&name.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__RAW_VEG_PLURAL_S(), &name.clone())) } };
    return op_modulo(&template.clone(), &py_dict(vec![(py_str("one"), name.clone()), (py_str("Cap"), op_add(&mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&name.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("them"), if py_truthy(&plural.clone()) { py_str("them") } else { py_str("it") }), (py_str("they"), if py_truthy(&plural.clone()) { py_str("they") } else { py_str("it") }), (py_str("are"), if py_truthy(&plural.clone()) { py_str("are") } else { py_str("is") })]));
    }
    return Py::None;
    Py::None
}
pub fn _names_no_food(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let mut meals: Py = Py::None;
    let mut name: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names no food, so the unnamed spoilage row is the one to answer.\n\n    \"smells off can i still cook it\" has nothing to look up, so the questions below are the whole\n    question. \"my milk smells a bit off, should i still use it\" names the milk, and the milk card\n    says more about it than any general rule can; without this test the general row took that\n    line and gave back a rule about rice and hard cheese.\n\n    The pantry table is read here rather than imported at the top of the file, because it is\n    built from this module's own tables and the import would go round in a circle.\n    ");
    meals = pyrt::xrefs::xmodule("knowledge.meals");
    understood = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { understood.clone() } else { py_dict(vec![]) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_set(__out) })() };
    text = op_add(&op_add(&py_str(" "), &mcall(&call_builtin("str", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])), &py_str(" "));
    known = op_bitor(&op_bitor(&gattr(&meals.clone(), "IN_A_DISH"), &call_builtin("set", &[gattr(&meals.clone(), "INGREDIENTS")]).unwrap_or_else(|e| exc::raise(e))), &call_builtin("set", &[g_EXTRA_FOODS()]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&known.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&known.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&known.clone(), &name.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &op_modulo(&py_str(" %s "), &name.clone()))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _not_a_fault_in_the_dish(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line is a fault in the food, which the fix cards answer.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    return Py::Bool(!py_truthy(&mcall(&g__A_FAULT_IN_THE_DISH(), "search", &[said.clone()])));
    Py::None
}
pub fn _mince_keeping_is_its_own_row(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line is a keeping question about mince, which has a row of its own.");
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { mcall(&py_str(" "), "join", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]) } } } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    said = op_modulo(&py_str(" %s "), &mcall(&stdlib_call("re", "sub", &[py_str("[^a-z0-9']+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MINCE_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MINCE_VERDICT_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MINCE_KEEPING_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &op_modulo(&py_str(" %s "), &word.clone())))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _not_a_recipe_now(args: &[Py]) -> Py {
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
    let _ = py_str("False for \"what can i make with the leftover curry\", true for \"is the curry still ok\".");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COOKING_ASK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__KEEPING_IN_THE_LINE()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PAST_KEEPING()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _how_many_days_ago(args: &[Py]) -> Py {
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
    let mut days: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let mut part: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The day count the line gives for the dish, or None when the line gives none.");
    match_ = mcall(&g__DAYS_AGO(), "search", &[text.clone()]);
    if py_truthy(&match_.clone()) {
    said = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&match_.clone(), "groups", &[])) { let mut part: Py = __item.clone(); if py_truthy(&part.clone()) {  __out.push(part.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    number = mcall(&g__COUNT_WORDS(), "get", &[mcall(&said.clone(), "lower", &[])]);
    return if py_truthy(&Py::Bool(py_is(&number.clone(), &Py::None))) { call_builtin("int", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { number.clone() };
    }
    let __iter1 = py_iter_or_raise(&g__DAYS_SAID());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    phrase = __unpk2[0].clone();
    days = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &phrase.clone()))) {
    return days.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_verdict_on_the_days(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut days: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "days") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: days"))) };
    if py_truthy(&Py::Bool(py_is(&days.clone(), &Py::None))) {
    return py_str("");
    }
    if py_truthy(&op_le(&days.clone(), &Py::Int(0))) {
    return py_str("Yes - it is from today or last night, so it is fine.");
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(1)))) {
    return py_str("Yes - a day in the fridge is nothing, so it is fine.");
    }
    if py_truthy(&op_le(&days.clone(), &Py::Int(3))) {
    return op_modulo(&py_str("Yes - %d days is inside the 3 to 4 days a cooked dish keeps in the fridge, so it is fine."), &days.clone());
    }
    if py_truthy(&Py::Bool(py_eq(&days.clone(), &Py::Int(4)))) {
    return py_str("At the edge, I'm afraid: 3 to 4 days is the limit, so today is the last day I would eat it and I would not leave it for tomorrow.");
    }
    return op_modulo(&py_str("No, I would not: a cooked dish keeps 3 to 4 days in the fridge, and rice only 1, so %d days is past it."), &days.clone());
    Py::None
}
pub fn _the_leftovers_answer(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut said: Py = Py::None;
    let mut the_line: Py = Py::None;
    let mut verdict: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The keeping rule, opened by the verdict on the days this line gives.\n\n    A verdict is an answer to a question, so it is only given to one: \"i made a curry last night\"\n    is news about the pot rather than a question about it, and a \"Yes - a day in the fridge is\n    nothing\" in front of the rule would be answering something nobody asked.\n    ");
    the_line = py_func("the_line", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "the_line", args));
    reading = if py_truthy(&call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { reading.clone() } else { py_dict(vec![]) };
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    asked = { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__VERDICT_ASKS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&call_builtin("str", &[{ let __b = mcall(&reading.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("?"))); if py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g__ENDS_ON_AN_ASK(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return g__LEFTOVERS_RULE();
    }
    verdict = _the_verdict_on_the_days(&[_how_many_days_ago(&[py_call(&the_line, &[reading.clone()])])]);
    if py_truthy(&Py::Bool(!py_truthy(&verdict.clone()))) {
    return g__LEFTOVERS_RULE();
    }
    if py_truthy(&mcall(&verdict.clone(), "startswith", &[py_str("No")])) {
    return op_add(&verdict.clone(), &py_str(" Reheating does not make it safe: the toxins old food builds up do not all break down with heat and they do not all smell."));
    }
    return op_add(&op_add(&verdict.clone(), &py_str(" ")), &g__LEFTOVERS_RULE());
    Py::None
}
pub fn _not_about_money(args: &[Py]) -> Py {
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
    let mut value: Py = Py::None;
    let _ = py_str("Whether the sum in the line is something to keep rather than something to eat.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("text"), py_str("plain")])) { let mut key: Py = __item.clone(); __out.push(mcall(&understood.clone(), "get", &[key.clone()])); } py_list(__out) })() }) { let mut value: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]), "lower", &[]);
    return Py::Bool(py_is(&mcall(&g__MONEY_IN_THE_LINE(), "search", &[said.clone()]), &Py::None));
    Py::None
}
pub fn _the_leaf_shows_a_colour(args: &[Py]) -> Py {
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
    let _ = py_str("The line names a colour, a slime or a smell on the leaf, not just that it went limp.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__LEAFY_SHOWS(), "search", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } } } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.still_ok".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "DIET_WORDS" => Some(g_DIET_WORDS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EXTRA_FOODS" => Some(g_EXTRA_FOODS()),
            "MINCE_KEEPING_WORDS" => Some(g_MINCE_KEEPING_WORDS()),
            "MINCE_VERDICT_WORDS" => Some(g_MINCE_VERDICT_WORDS()),
            "MINCE_WORDS" => Some(g_MINCE_WORDS()),
            "OFF_ASKS" => Some(g_OFF_ASKS()),
            "OFF_THE_THING" => Some(g_OFF_THE_THING()),
            "OFF_WORDS" => Some(g_OFF_WORDS()),
            "PACK_DATE" => Some(g_PACK_DATE()),
            "PET_LINE_WORDS" => Some(g_PET_LINE_WORDS()),
            "POISONING_NOT" => Some(g_POISONING_NOT()),
            "POISONING_WORDS" => Some(g_POISONING_WORDS()),
            "RAW_WORDS" => Some(g_RAW_WORDS()),
            "SPOILAGE_BIAS" => Some(g_SPOILAGE_BIAS()),
            "UNDERCOOKED_PINK" => Some(g_UNDERCOOKED_PINK()),
            "UNRIPE_NOT" => Some(g_UNRIPE_NOT()),
            "UNRIPE_WORDS" => Some(g_UNRIPE_WORDS()),
            "_A_FAULT_IN_THE_DISH" => Some(g__A_FAULT_IN_THE_DISH()),
            "_CABBAGES" => Some(g__CABBAGES()),
            "_CABBAGE_SIGNS" => Some(g__CABBAGE_SIGNS()),
            "_COOKING_ASK" => Some(g__COOKING_ASK()),
            "_COUNT_WORDS" => Some(g__COUNT_WORDS()),
            "_DAYS_AGO" => Some(g__DAYS_AGO()),
            "_DAYS_SAID" => Some(g__DAYS_SAID()),
            "_ENDS_ON_AN_ASK" => Some(g__ENDS_ON_AN_ASK()),
            "_KEEPING_IN_THE_LINE" => Some(g__KEEPING_IN_THE_LINE()),
            "_LEAFY_GREENS" => Some(g__LEAFY_GREENS()),
            "_LEAFY_SHOWS" => Some(g__LEAFY_SHOWS()),
            "_LEAFY_SIGNS" => Some(g__LEAFY_SIGNS()),
            "_LEFTOVERS_RULE" => Some(g__LEFTOVERS_RULE()),
            "_MONEY_IN_THE_LINE" => Some(g__MONEY_IN_THE_LINE()),
            "_NOT_THIS" => Some(g__NOT_THIS()),
            "_PAST_KEEPING" => Some(g__PAST_KEEPING()),
            "_RAW_VEGETABLE_NAMES" => Some(g__RAW_VEGETABLE_NAMES()),
            "_RAW_VEGETABLE_NOT" => Some(g__RAW_VEGETABLE_NOT()),
            "_RAW_VEG_GROUPS" => Some(g__RAW_VEG_GROUPS()),
            "_RAW_VEG_ONE" => Some(g__RAW_VEG_ONE()),
            "_RAW_VEG_PLURAL_S" => Some(g__RAW_VEG_PLURAL_S()),
            "_ROOM_ODOUR" => Some(g__ROOM_ODOUR()),
            "_R_MONTHS" => Some(g__R_MONTHS()),
            "_SHELLFISH" => Some(g__SHELLFISH()),
            "_VERDICT_ASKS" => Some(g__VERDICT_ASKS()),
            "_how_many_days_ago" => Some(py_func("_how_many_days_ago", |args: &[Py]| _how_many_days_ago(args))),
            "_item" => Some(g__item()),
            "_mince_keeping_is_its_own_row" => Some(py_func("_mince_keeping_is_its_own_row", |args: &[Py]| _mince_keeping_is_its_own_row(args))),
            "_names_no_food" => Some(py_func("_names_no_food", |args: &[Py]| _names_no_food(args))),
            "_not_a_fault_in_the_dish" => Some(py_func("_not_a_fault_in_the_dish", |args: &[Py]| _not_a_fault_in_the_dish(args))),
            "_not_a_recipe_now" => Some(py_func("_not_a_recipe_now", |args: &[Py]| _not_a_recipe_now(args))),
            "_not_about_money" => Some(py_func("_not_about_money", |args: &[Py]| _not_about_money(args))),
            "_raw_vegetable" => Some(py_func("_raw_vegetable", |args: &[Py]| _raw_vegetable(args))),
            "_the_leaf_shows_a_colour" => Some(py_func("_the_leaf_shows_a_colour", |args: &[Py]| _the_leaf_shows_a_colour(args))),
            "_the_leftovers_answer" => Some(py_func("_the_leftovers_answer", |args: &[Py]| _the_leftovers_answer(args))),
            "_the_verdict_on_the_days" => Some(py_func("_the_verdict_on_the_days", |args: &[Py]| _the_verdict_on_the_days(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "DIET_WORDS" => s_DIET_WORDS(v),
            "ENTRIES" => s_ENTRIES(v),
            "EXTRA_FOODS" => s_EXTRA_FOODS(v),
            "MINCE_KEEPING_WORDS" => s_MINCE_KEEPING_WORDS(v),
            "MINCE_VERDICT_WORDS" => s_MINCE_VERDICT_WORDS(v),
            "MINCE_WORDS" => s_MINCE_WORDS(v),
            "OFF_ASKS" => s_OFF_ASKS(v),
            "OFF_THE_THING" => s_OFF_THE_THING(v),
            "OFF_WORDS" => s_OFF_WORDS(v),
            "PACK_DATE" => s_PACK_DATE(v),
            "PET_LINE_WORDS" => s_PET_LINE_WORDS(v),
            "POISONING_NOT" => s_POISONING_NOT(v),
            "POISONING_WORDS" => s_POISONING_WORDS(v),
            "RAW_WORDS" => s_RAW_WORDS(v),
            "SPOILAGE_BIAS" => s_SPOILAGE_BIAS(v),
            "UNDERCOOKED_PINK" => s_UNDERCOOKED_PINK(v),
            "UNRIPE_NOT" => s_UNRIPE_NOT(v),
            "UNRIPE_WORDS" => s_UNRIPE_WORDS(v),
            "_A_FAULT_IN_THE_DISH" => s__A_FAULT_IN_THE_DISH(v),
            "_CABBAGES" => s__CABBAGES(v),
            "_CABBAGE_SIGNS" => s__CABBAGE_SIGNS(v),
            "_COOKING_ASK" => s__COOKING_ASK(v),
            "_COUNT_WORDS" => s__COUNT_WORDS(v),
            "_DAYS_AGO" => s__DAYS_AGO(v),
            "_DAYS_SAID" => s__DAYS_SAID(v),
            "_ENDS_ON_AN_ASK" => s__ENDS_ON_AN_ASK(v),
            "_KEEPING_IN_THE_LINE" => s__KEEPING_IN_THE_LINE(v),
            "_LEAFY_GREENS" => s__LEAFY_GREENS(v),
            "_LEAFY_SHOWS" => s__LEAFY_SHOWS(v),
            "_LEAFY_SIGNS" => s__LEAFY_SIGNS(v),
            "_LEFTOVERS_RULE" => s__LEFTOVERS_RULE(v),
            "_MONEY_IN_THE_LINE" => s__MONEY_IN_THE_LINE(v),
            "_NOT_THIS" => s__NOT_THIS(v),
            "_PAST_KEEPING" => s__PAST_KEEPING(v),
            "_RAW_VEGETABLE_NAMES" => s__RAW_VEGETABLE_NAMES(v),
            "_RAW_VEGETABLE_NOT" => s__RAW_VEGETABLE_NOT(v),
            "_RAW_VEG_GROUPS" => s__RAW_VEG_GROUPS(v),
            "_RAW_VEG_ONE" => s__RAW_VEG_ONE(v),
            "_RAW_VEG_PLURAL_S" => s__RAW_VEG_PLURAL_S(v),
            "_ROOM_ODOUR" => s__ROOM_ODOUR(v),
            "_R_MONTHS" => s__R_MONTHS(v),
            "_SHELLFISH" => s__SHELLFISH(v),
            "_VERDICT_ASKS" => s__VERDICT_ASKS(v),
            "_item" => s__item(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.still_ok", module_obj);
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_raw_vegetable", py_func("_raw_vegetable", |args: &[Py]| _raw_vegetable(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_names_no_food", py_func("_names_no_food", |args: &[Py]| _names_no_food(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_not_a_fault_in_the_dish", py_func("_not_a_fault_in_the_dish", |args: &[Py]| _not_a_fault_in_the_dish(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_mince_keeping_is_its_own_row", py_func("_mince_keeping_is_its_own_row", |args: &[Py]| _mince_keeping_is_its_own_row(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_not_a_recipe_now", py_func("_not_a_recipe_now", |args: &[Py]| _not_a_recipe_now(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_how_many_days_ago", py_func("_how_many_days_ago", |args: &[Py]| _how_many_days_ago(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_the_verdict_on_the_days", py_func("_the_verdict_on_the_days", |args: &[Py]| _the_verdict_on_the_days(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_the_leftovers_answer", py_func("_the_leftovers_answer", |args: &[Py]| _the_leftovers_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_not_about_money", py_func("_not_about_money", |args: &[Py]| _not_about_money(args)));
    pyrt::xrefs::reg_fn("knowledge.still_ok", "_the_leaf_shows_a_colour", py_func("_the_leaf_shows_a_colour", |args: &[Py]| _the_leaf_shows_a_colour(args)));
    pyrt::xrefs::reg_access("knowledge.still_ok", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "DIET_WORDS", g_DIET_WORDS, s_DIET_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.still_ok", "EXTRA_FOODS", g_EXTRA_FOODS, s_EXTRA_FOODS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "MINCE_KEEPING_WORDS", g_MINCE_KEEPING_WORDS, s_MINCE_KEEPING_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "MINCE_VERDICT_WORDS", g_MINCE_VERDICT_WORDS, s_MINCE_VERDICT_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "MINCE_WORDS", g_MINCE_WORDS, s_MINCE_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "OFF_ASKS", g_OFF_ASKS, s_OFF_ASKS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "OFF_THE_THING", g_OFF_THE_THING, s_OFF_THE_THING);
    pyrt::xrefs::reg_access("knowledge.still_ok", "OFF_WORDS", g_OFF_WORDS, s_OFF_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "PACK_DATE", g_PACK_DATE, s_PACK_DATE);
    pyrt::xrefs::reg_access("knowledge.still_ok", "PET_LINE_WORDS", g_PET_LINE_WORDS, s_PET_LINE_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "POISONING_NOT", g_POISONING_NOT, s_POISONING_NOT);
    pyrt::xrefs::reg_access("knowledge.still_ok", "POISONING_WORDS", g_POISONING_WORDS, s_POISONING_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "RAW_WORDS", g_RAW_WORDS, s_RAW_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "SPOILAGE_BIAS", g_SPOILAGE_BIAS, s_SPOILAGE_BIAS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "UNDERCOOKED_PINK", g_UNDERCOOKED_PINK, s_UNDERCOOKED_PINK);
    pyrt::xrefs::reg_access("knowledge.still_ok", "UNRIPE_NOT", g_UNRIPE_NOT, s_UNRIPE_NOT);
    pyrt::xrefs::reg_access("knowledge.still_ok", "UNRIPE_WORDS", g_UNRIPE_WORDS, s_UNRIPE_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_A_FAULT_IN_THE_DISH", g__A_FAULT_IN_THE_DISH, s__A_FAULT_IN_THE_DISH);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_CABBAGES", g__CABBAGES, s__CABBAGES);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_CABBAGE_SIGNS", g__CABBAGE_SIGNS, s__CABBAGE_SIGNS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_COOKING_ASK", g__COOKING_ASK, s__COOKING_ASK);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_COUNT_WORDS", g__COUNT_WORDS, s__COUNT_WORDS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_DAYS_AGO", g__DAYS_AGO, s__DAYS_AGO);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_DAYS_SAID", g__DAYS_SAID, s__DAYS_SAID);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_ENDS_ON_AN_ASK", g__ENDS_ON_AN_ASK, s__ENDS_ON_AN_ASK);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_KEEPING_IN_THE_LINE", g__KEEPING_IN_THE_LINE, s__KEEPING_IN_THE_LINE);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_LEAFY_GREENS", g__LEAFY_GREENS, s__LEAFY_GREENS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_LEAFY_SHOWS", g__LEAFY_SHOWS, s__LEAFY_SHOWS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_LEAFY_SIGNS", g__LEAFY_SIGNS, s__LEAFY_SIGNS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_LEFTOVERS_RULE", g__LEFTOVERS_RULE, s__LEFTOVERS_RULE);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_MONEY_IN_THE_LINE", g__MONEY_IN_THE_LINE, s__MONEY_IN_THE_LINE);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_NOT_THIS", g__NOT_THIS, s__NOT_THIS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_PAST_KEEPING", g__PAST_KEEPING, s__PAST_KEEPING);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_RAW_VEGETABLE_NAMES", g__RAW_VEGETABLE_NAMES, s__RAW_VEGETABLE_NAMES);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_RAW_VEGETABLE_NOT", g__RAW_VEGETABLE_NOT, s__RAW_VEGETABLE_NOT);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_RAW_VEG_GROUPS", g__RAW_VEG_GROUPS, s__RAW_VEG_GROUPS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_RAW_VEG_ONE", g__RAW_VEG_ONE, s__RAW_VEG_ONE);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_RAW_VEG_PLURAL_S", g__RAW_VEG_PLURAL_S, s__RAW_VEG_PLURAL_S);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_ROOM_ODOUR", g__ROOM_ODOUR, s__ROOM_ODOUR);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_R_MONTHS", g__R_MONTHS, s__R_MONTHS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_SHELLFISH", g__SHELLFISH, s__SHELLFISH);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_VERDICT_ASKS", g__VERDICT_ASKS, s__VERDICT_ASKS);
    pyrt::xrefs::reg_access("knowledge.still_ok", "_item", g__item, s__item);
    pyrt::xrefs::reg_access("knowledge.still_ok", "entry", g_entry, s_entry);
}

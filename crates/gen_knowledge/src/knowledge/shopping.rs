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


thread_local! { static G_AGE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ANSWERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPLIANCE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_APPLIANCE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CARD_IS_NOT_A_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHEAP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_ABOUT_MONEY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_ON_A_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BUY_INTO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_AFTER_A_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP_ADULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHEAP_TEEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_GIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_PRESENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOICE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHOOSING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CRISIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EVERYDAY_WHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GIFT_FOR_A_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GROWN_GIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_INTEREST_GIFTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KITCHEN_MACHINE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KITCHEN_MACHINE_VALUE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MONEY_VERDICT_YES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THAT_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PERSON_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLURAL_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_BANDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_OPENINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRESENT_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_COMPARED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRICE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUPERMARKETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TASTE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TEEN_GIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TODDLER_GIFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TURNING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_YES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ACTION_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AGE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AN_AGE_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CHILDS_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHEAPEST_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CHOOSE_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COAT_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COAT_CHECK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__COAT_KINDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEAR_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DECISION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DRYER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOLD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__GIFT_ANYONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HEIGHT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__INSIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__KEEP_RECEIPT_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__METER_FAULT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__METER_READ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MIDDLE_TAIL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__MINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMED_BY_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PRESENT_PEOPLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__QUALITY_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__RANGE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SAID_AS_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SERVICE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHOP_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__SHOP_PLACES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__THE_EVERYDAY_THINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__TRADER_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATCH_PLAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATCH_SMART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATCH_SMART_SAY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATCH_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WATCH_VERB_SHAPE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__worth_it_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_AGE_ASK() -> Py { ensure_init(); G_AGE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_ASK(v: Py) { ensure_init(); G_AGE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_APPLIANCE_ASK() -> Py { ensure_init(); G_APPLIANCE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPLIANCE_ASK(v: Py) { ensure_init(); G_APPLIANCE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_APPLIANCE_VALUE() -> Py { ensure_init(); G_APPLIANCE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_APPLIANCE_VALUE(v: Py) { ensure_init(); G_APPLIANCE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CARD_IS_NOT_A_PRESENT() -> Py { ensure_init(); G_A_CARD_IS_NOT_A_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CARD_IS_NOT_A_PRESENT(v: Py) { ensure_init(); G_A_CARD_IS_NOT_A_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHEAP_ASK() -> Py { ensure_init(); G_A_CHEAP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHEAP_ASK(v: Py) { ensure_init(); G_A_CHEAP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_ABOUT_MONEY() -> Py { ensure_init(); G_A_VERDICT_ABOUT_MONEY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_ABOUT_MONEY(v: Py) { ensure_init(); G_A_VERDICT_ABOUT_MONEY.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_ON_A_PRESENT() -> Py { ensure_init(); G_A_VERDICT_ON_A_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_ON_A_PRESENT(v: Py) { ensure_init(); G_A_VERDICT_ON_A_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_THING() -> Py { ensure_init(); G_A_VERDICT_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_THING(v: Py) { ensure_init(); G_A_VERDICT_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT_WORD() -> Py { ensure_init(); G_A_VERDICT_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT_WORD(v: Py) { ensure_init(); G_A_VERDICT_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_BUY_INTO() -> Py { ensure_init(); G_BUY_INTO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BUY_INTO(v: Py) { ensure_init(); G_BUY_INTO.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_AFTER_A_PRESENT() -> Py { ensure_init(); G_CARD_AFTER_A_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_AFTER_A_PRESENT(v: Py) { ensure_init(); G_CARD_AFTER_A_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP_ADULT() -> Py { ensure_init(); G_CHEAP_ADULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP_ADULT(v: Py) { ensure_init(); G_CHEAP_ADULT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP_CHILD() -> Py { ensure_init(); G_CHEAP_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP_CHILD(v: Py) { ensure_init(); G_CHEAP_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_CHEAP_TEEN() -> Py { ensure_init(); G_CHEAP_TEEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHEAP_TEEN(v: Py) { ensure_init(); G_CHEAP_TEEN.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_GIFT() -> Py { ensure_init(); G_CHILD_GIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_GIFT(v: Py) { ensure_init(); G_CHILD_GIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_PEOPLE() -> Py { ensure_init(); G_CHILD_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_PEOPLE(v: Py) { ensure_init(); G_CHILD_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_PRESENT() -> Py { ensure_init(); G_CHILD_PRESENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_PRESENT(v: Py) { ensure_init(); G_CHILD_PRESENT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_WORDS() -> Py { ensure_init(); G_CHILD_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_WORDS(v: Py) { ensure_init(); G_CHILD_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOICE_VALUE() -> Py { ensure_init(); G_CHOICE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOICE_VALUE(v: Py) { ensure_init(); G_CHOICE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_CHOOSING() -> Py { ensure_init(); G_CHOOSING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHOOSING(v: Py) { ensure_init(); G_CHOOSING.with(|c| *c.borrow_mut() = v); }
pub fn g_CRISIS() -> Py { ensure_init(); G_CRISIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CRISIS(v: Py) { ensure_init(); G_CRISIS.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_EVERYDAY_WHERE() -> Py { ensure_init(); G_EVERYDAY_WHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EVERYDAY_WHERE(v: Py) { ensure_init(); G_EVERYDAY_WHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_ASK() -> Py { ensure_init(); G_GIFT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_ASK(v: Py) { ensure_init(); G_GIFT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_GIFT_FOR_A_CONDITION() -> Py { ensure_init(); G_GIFT_FOR_A_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GIFT_FOR_A_CONDITION(v: Py) { ensure_init(); G_GIFT_FOR_A_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_GROWN_GIFT() -> Py { ensure_init(); G_GROWN_GIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GROWN_GIFT(v: Py) { ensure_init(); G_GROWN_GIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_INTEREST_GIFTS() -> Py { ensure_init(); G_INTEREST_GIFTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_INTEREST_GIFTS(v: Py) { ensure_init(); G_INTEREST_GIFTS.with(|c| *c.borrow_mut() = v); }
pub fn g_KITCHEN_MACHINE_NAMES() -> Py { ensure_init(); G_KITCHEN_MACHINE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KITCHEN_MACHINE_NAMES(v: Py) { ensure_init(); G_KITCHEN_MACHINE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_KITCHEN_MACHINE_VALUE() -> Py { ensure_init(); G_KITCHEN_MACHINE_VALUE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KITCHEN_MACHINE_VALUE(v: Py) { ensure_init(); G_KITCHEN_MACHINE_VALUE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOSS() -> Py { ensure_init(); G_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOSS(v: Py) { ensure_init(); G_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_MONEY_VERDICT_YES() -> Py { ensure_init(); G_MONEY_VERDICT_YES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MONEY_VERDICT_YES(v: Py) { ensure_init(); G_MONEY_VERDICT_YES.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THAT_CARD() -> Py { ensure_init(); G_NOT_THAT_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THAT_CARD(v: Py) { ensure_init(); G_NOT_THAT_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); G_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); G_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_ASK() -> Py { ensure_init(); G_PERSON_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_ASK(v: Py) { ensure_init(); G_PERSON_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PERSON_WORDS() -> Py { ensure_init(); G_PERSON_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PERSON_WORDS(v: Py) { ensure_init(); G_PERSON_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_ONLY() -> Py { ensure_init(); G_PLURAL_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_ONLY(v: Py) { ensure_init(); G_PLURAL_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_PLURAL_WORDS() -> Py { ensure_init(); G_PLURAL_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLURAL_WORDS(v: Py) { ensure_init(); G_PLURAL_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_ASK() -> Py { ensure_init(); G_PRESENT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_ASK(v: Py) { ensure_init(); G_PRESENT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_BANDS() -> Py { ensure_init(); G_PRESENT_BANDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_BANDS(v: Py) { ensure_init(); G_PRESENT_BANDS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_OPENINGS() -> Py { ensure_init(); G_PRESENT_OPENINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_OPENINGS(v: Py) { ensure_init(); G_PRESENT_OPENINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_PRESENT_OVER() -> Py { ensure_init(); G_PRESENT_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRESENT_OVER(v: Py) { ensure_init(); G_PRESENT_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_COMPARED() -> Py { ensure_init(); G_PRICE_COMPARED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_COMPARED(v: Py) { ensure_init(); G_PRICE_COMPARED.with(|c| *c.borrow_mut() = v); }
pub fn g_PRICE_SAID() -> Py { ensure_init(); G_PRICE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRICE_SAID(v: Py) { ensure_init(); G_PRICE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_SUPERMARKETS() -> Py { ensure_init(); G_SUPERMARKETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUPERMARKETS(v: Py) { ensure_init(); G_SUPERMARKETS.with(|c| *c.borrow_mut() = v); }
pub fn g_TASTE_WORDS() -> Py { ensure_init(); G_TASTE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TASTE_WORDS(v: Py) { ensure_init(); G_TASTE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_TEEN_GIFT() -> Py { ensure_init(); G_TEEN_GIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TEEN_GIFT(v: Py) { ensure_init(); G_TEEN_GIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_TODDLER_GIFT() -> Py { ensure_init(); G_TODDLER_GIFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TODDLER_GIFT(v: Py) { ensure_init(); G_TODDLER_GIFT.with(|c| *c.borrow_mut() = v); }
pub fn g_TURNING() -> Py { ensure_init(); G_TURNING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TURNING(v: Py) { ensure_init(); G_TURNING.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_YES() -> Py { ensure_init(); G_VERDICT_YES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_YES(v: Py) { ensure_init(); G_VERDICT_YES.with(|c| *c.borrow_mut() = v); }
pub fn g__ACTION_NAME() -> Py { ensure_init(); G__ACTION_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ACTION_NAME(v: Py) { ensure_init(); G__ACTION_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g__AGE_WORDS() -> Py { ensure_init(); G__AGE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AGE_WORDS(v: Py) { ensure_init(); G__AGE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_AGE() -> Py { ensure_init(); G__AN_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_AGE(v: Py) { ensure_init(); G__AN_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__AN_AGE_SAID() -> Py { ensure_init(); G__AN_AGE_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AN_AGE_SAID(v: Py) { ensure_init(); G__AN_AGE_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CHILDS_AGE() -> Py { ensure_init(); G__A_CHILDS_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CHILDS_AGE(v: Py) { ensure_init(); G__A_CHILDS_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g__CHEAPEST_TAIL() -> Py { ensure_init(); G__CHEAPEST_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHEAPEST_TAIL(v: Py) { ensure_init(); G__CHEAPEST_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__CHOOSE_ASK() -> Py { ensure_init(); G__CHOOSE_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CHOOSE_ASK(v: Py) { ensure_init(); G__CHOOSE_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__COAT_CHECK() -> Py { ensure_init(); G__COAT_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COAT_CHECK(v: Py) { ensure_init(); G__COAT_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g__COAT_CHECK_WORDS() -> Py { ensure_init(); G__COAT_CHECK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COAT_CHECK_WORDS(v: Py) { ensure_init(); G__COAT_CHECK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__COAT_KINDS() -> Py { ensure_init(); G__COAT_KINDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__COAT_KINDS(v: Py) { ensure_init(); G__COAT_KINDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DEAR_TAIL() -> Py { ensure_init(); G__DEAR_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEAR_TAIL(v: Py) { ensure_init(); G__DEAR_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__DECISION_WORDS() -> Py { ensure_init(); G__DECISION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DECISION_WORDS(v: Py) { ensure_init(); G__DECISION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__DRYER_WORDS() -> Py { ensure_init(); G__DRYER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DRYER_WORDS(v: Py) { ensure_init(); G__DRYER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FOLD() -> Py { ensure_init(); G__FOLD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOLD(v: Py) { ensure_init(); G__FOLD.with(|c| *c.borrow_mut() = v); }
pub fn g__GIFT_ANYONE() -> Py { ensure_init(); G__GIFT_ANYONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__GIFT_ANYONE(v: Py) { ensure_init(); G__GIFT_ANYONE.with(|c| *c.borrow_mut() = v); }
pub fn g__HEIGHT() -> Py { ensure_init(); G__HEIGHT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HEIGHT(v: Py) { ensure_init(); G__HEIGHT.with(|c| *c.borrow_mut() = v); }
pub fn g__INSIDE() -> Py { ensure_init(); G__INSIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__INSIDE(v: Py) { ensure_init(); G__INSIDE.with(|c| *c.borrow_mut() = v); }
pub fn g__KEEP_RECEIPT_RE() -> Py { ensure_init(); G__KEEP_RECEIPT_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__KEEP_RECEIPT_RE(v: Py) { ensure_init(); G__KEEP_RECEIPT_RE.with(|c| *c.borrow_mut() = v); }
pub fn g__METER_FAULT() -> Py { ensure_init(); G__METER_FAULT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__METER_FAULT(v: Py) { ensure_init(); G__METER_FAULT.with(|c| *c.borrow_mut() = v); }
pub fn g__METER_READ() -> Py { ensure_init(); G__METER_READ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__METER_READ(v: Py) { ensure_init(); G__METER_READ.with(|c| *c.borrow_mut() = v); }
pub fn g__MIDDLE_TAIL() -> Py { ensure_init(); G__MIDDLE_TAIL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MIDDLE_TAIL(v: Py) { ensure_init(); G__MIDDLE_TAIL.with(|c| *c.borrow_mut() = v); }
pub fn g__MINE() -> Py { ensure_init(); G__MINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__MINE(v: Py) { ensure_init(); G__MINE.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMED_BY_PRONOUN() -> Py { ensure_init(); G__NAMED_BY_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMED_BY_PRONOUN(v: Py) { ensure_init(); G__NAMED_BY_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__PRESENT_PEOPLE() -> Py { ensure_init(); G__PRESENT_PEOPLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PRESENT_PEOPLE(v: Py) { ensure_init(); G__PRESENT_PEOPLE.with(|c| *c.borrow_mut() = v); }
pub fn g__QUALITY_WORDS() -> Py { ensure_init(); G__QUALITY_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__QUALITY_WORDS(v: Py) { ensure_init(); G__QUALITY_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__RANGE_WORDS() -> Py { ensure_init(); G__RANGE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__RANGE_WORDS(v: Py) { ensure_init(); G__RANGE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SAID_AS_WORDS() -> Py { ensure_init(); G__SAID_AS_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SAID_AS_WORDS(v: Py) { ensure_init(); G__SAID_AS_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SERVICE_WORDS() -> Py { ensure_init(); G__SERVICE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SERVICE_WORDS(v: Py) { ensure_init(); G__SERVICE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__SHOP_ASK() -> Py { ensure_init(); G__SHOP_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHOP_ASK(v: Py) { ensure_init(); G__SHOP_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__SHOP_PLACES() -> Py { ensure_init(); G__SHOP_PLACES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__SHOP_PLACES(v: Py) { ensure_init(); G__SHOP_PLACES.with(|c| *c.borrow_mut() = v); }
pub fn g__THE_EVERYDAY_THINGS() -> Py { ensure_init(); G__THE_EVERYDAY_THINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__THE_EVERYDAY_THINGS(v: Py) { ensure_init(); G__THE_EVERYDAY_THINGS.with(|c| *c.borrow_mut() = v); }
pub fn g__TRADER_WORDS() -> Py { ensure_init(); G__TRADER_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__TRADER_WORDS(v: Py) { ensure_init(); G__TRADER_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__WATCH_PLAIN() -> Py { ensure_init(); G__WATCH_PLAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATCH_PLAIN(v: Py) { ensure_init(); G__WATCH_PLAIN.with(|c| *c.borrow_mut() = v); }
pub fn g__WATCH_SMART() -> Py { ensure_init(); G__WATCH_SMART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATCH_SMART(v: Py) { ensure_init(); G__WATCH_SMART.with(|c| *c.borrow_mut() = v); }
pub fn g__WATCH_SMART_SAY() -> Py { ensure_init(); G__WATCH_SMART_SAY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATCH_SMART_SAY(v: Py) { ensure_init(); G__WATCH_SMART_SAY.with(|c| *c.borrow_mut() = v); }
pub fn g__WATCH_VERBS() -> Py { ensure_init(); G__WATCH_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATCH_VERBS(v: Py) { ensure_init(); G__WATCH_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g__WATCH_VERB_SHAPE() -> Py { ensure_init(); G__WATCH_VERB_SHAPE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WATCH_VERB_SHAPE(v: Py) { ensure_init(); G__WATCH_VERB_SHAPE.with(|c| *c.borrow_mut() = v); }
pub fn g__worth_it_entry() -> Py { ensure_init(); G__worth_it_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__worth_it_entry(v: Py) { ensure_init(); G__worth_it_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Buying things: laptops, shoes, bikes, mattresses, gifts and whether a deal is real.\n\nThe answers lane owns this domain. Each Entry says what actually matters when choosing,\nin the order it matters, and what to check before handing over money.\n");
    s_PRESENT_ASK(crate::knowledge::lookup::g_PRESENT_ASK());
    s_PRICE_COMPARED(crate::knowledge::lookup::g_PRICE_COMPARED());
    s_ANSWERS(py_tuple(vec![py_str("shopping"), py_str("money"), py_str("technology")]));
    s__GIFT_ANYONE(py_str("Buy into what they already do: a good version of something they use every week (a proper kitchen knife, warm socks, decent coffee), a voucher for a place they go anyway, or a day out the two of you share. Tell me what they do at the weekend and I'll narrow it down."));
    s_PERSON_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("mum mother dad father sister brother nan nana grandma grandad granny\n    son daughter wife husband partner friend mate flatmate boyfriend girlfriend colleague aunt\n    uncle cousin stepmum stepdad he she they him her his hers their stepmother stepfather"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TASTE_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("loves love likes like enjoys enjoy keen into fond obsessed collects\n    plays does goes went favourite favorite hobby hobbies garden gardening cooks cooking bakes\n    baking fishing reads reading runs running knitting sewing cycling walking football music\n    books films plants allotment"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__FOLD(py_tuple(vec![py_tuple(vec![py_str("'s"), py_str("")]), py_tuple(vec![py_str("s'"), py_str("")]), py_tuple(vec![py_str("s"), py_str("")]), py_tuple(vec![py_str("'"), py_str("")])]));
    s_SUPERMARKETS(py_tuple(vec![py_tuple(vec![py_str("Aldi"), py_tuple(vec![py_str("aldi")]), Py::Int(0), py_str("the cheapest of the big names"), py_str("own-brand staples run roughly a fifth less than at Tesco or Sainsbury's, on a smaller range with few named brands"), py_str("has the cheaper staples and a smaller range")]), py_tuple(vec![py_str("Lidl"), py_tuple(vec![py_str("lidl")]), Py::Int(0), py_str("one of the two cheapest of the big names"), py_str("own-brand staples run roughly a fifth less than at Tesco or Sainsbury's, with a strong bakery and few named brands"), py_str("is level on price and stronger on the bakery")]), py_tuple(vec![py_str("Farmfoods"), py_tuple(vec![py_str("farmfoods")]), Py::Int(0), py_str("cheap for frozen food"), py_str("the frozen and branded boxes are cheap and the fresh food is thin"), py_str("is cheaper on frozen and thinner on fresh")]), py_tuple(vec![py_str("Asda"), py_tuple(vec![py_str("asda")]), Py::Int(1), py_str("usually the cheapest of the big four"), py_str("a like-for-like basket comes out cheapest at Asda, and it price-matches Aldi and Lidl on the basics"), py_str("price-matches Aldi on the basics and runs more brands")]), py_tuple(vec![py_str("Iceland"), py_tuple(vec![py_str("iceland")]), Py::Int(1), py_str("cheap for frozen food"), py_str("frozen and ready meals are cheap and everything else is dearer"), py_str("is cheaper on frozen and dearer on the rest")]), py_tuple(vec![py_str("Morrisons"), py_tuple(vec![py_str("morrisons"), py_str("morrison")]), Py::Int(2), py_str("mid-priced"), py_str("it sits with Tesco and Sainsbury's, and is often the cheapest of the big four on fresh meat and fish"), py_str("is close on price with better fresh meat and fish")]), py_tuple(vec![py_str("Tesco"), py_tuple(vec![py_str("tesco"), py_str("tescos")]), Py::Int(2), py_str("mid-priced"), py_str("its Aldi Price Match holds the basics at Aldi's price, and the rest of the basket costs more"), py_str("holds the basics at Aldi's price and costs more for the rest")]), py_tuple(vec![py_str("Sainsbury's"), py_tuple(vec![py_str("sainsburys"), py_str("sainsbury's"), py_str("sainsbury")]), Py::Int(2), py_str("mid-priced like Tesco"), py_str("the basics are price-matched to Aldi and the brands are what cost more"), py_str("matches the basics and costs more for the brands")]), py_tuple(vec![py_str("Ocado"), py_tuple(vec![py_str("ocado")]), Py::Int(3), py_str("M&S food delivered"), py_str("it is priced like M&S rather than like a discounter"), py_str("is priced like M&S")]), py_tuple(vec![py_str("The Co-op"), py_tuple(vec![py_str("co-op"), py_str("co op"), py_str("coop"), py_str("co operative"), py_str("cooperative")]), Py::Int(3), py_str("a convenience shop"), py_str("it is dearer than the big four on almost everything, so it is a top-up shop"), py_str("is dearer, which is what the convenience buys")]), py_tuple(vec![py_str("Waitrose"), py_tuple(vec![py_str("waitrose")]), Py::Int(4), py_str("one of the two dearest"), py_str("it runs around 10-20% more than Tesco on the same basket, and the fresh food is better"), py_str("costs more and is better on fresh food")]), py_tuple(vec![py_str("M&S"), py_tuple(vec![py_str("marks and spencer"), py_str("marks spencer"), py_str("m and s"), py_str("m&s"), py_str("marks & spencer")]), Py::Int(4), py_str("the dearest of the big names"), py_str("it runs around 10-20% more than Tesco on the same basket, and the ready meals are why people go"), py_str("costs more, and the ready meals are why people go")])]));
    s__CHEAPEST_TAIL(py_str(" It is one of the cheapest places to do a full shop, though the range of brands is smaller, so anything specific may need a second stop."));
    s__MIDDLE_TAIL(py_str(" The basics are price-matched to the discounters, so it is the brands in the trolley that make the difference: a basket of own-brand staples will be cheaper at Aldi or Lidl."));
    s__DEAR_TAIL(py_str(" For a cheaper basket on the same list, the discounters save around a fifth and Asda or Tesco most of the rest."));
    s__QUALITY_WORDS(py_tuple(vec![py_str("good"), py_str("quality"), py_str("as good"), py_str("better"), py_str("worse"), py_str("taste"), py_str("tasty"), py_str("nicer"), py_str("fresh")]));
    s__RANGE_WORDS(py_tuple(vec![py_str("gluten"), py_str("free from"), py_str("free-from"), py_str("freefrom"), py_str("vegan"), py_str("vegetarian"), py_str("dairy free"), py_str("dairy-free"), py_str("nut free"), py_str("nut-free"), py_str("allergy"), py_str("allergies"), py_str("allergic"), py_str("coeliac"), py_str("celiac"), py_str("organic"), py_str("halal"), py_str("kosher"), py_str("good for"), py_str("fine for"), py_str("ok for"), py_str("okay for"), py_str("any good for"), py_str("better for"), py_str("stock"), py_str("stocks"), py_str("sell"), py_str("sells"), py_str("range of"), py_str("own brand")]));
    s__SERVICE_WORDS(py_tuple(vec![py_str("mobile"), py_str("broadband"), py_str("internet"), py_str("wifi"), py_str("phone"), py_str("phones"), py_str("sim"), py_str("sim only"), py_str("telly"), py_str("tv"), py_str("bank"), py_str("banking"), py_str("current account"), py_str("savings account"), py_str("insurance"), py_str("insure"), py_str("credit card"), py_str("loan"), py_str("mortgage"), py_str("tariff"), py_str("energy"), py_str("gas and electric"), py_str("electricity"), py_str("petrol"), py_str("diesel"), py_str("fuel"), py_str("travel money"), py_str("currency"), py_str("holiday"), py_str("holidays"), py_str("package holiday"), py_str("phone contract")]));
    s__SHOP_ASK(py_tuple(vec![py_str("should"), py_str("shall"), py_str("would"), py_str("worth"), py_str("better"), py_str("cheaper"), py_str("cheapest"), py_str("which"), py_str("what about"), py_str("instead of"), py_str("or "), py_str("vs"), py_str("versus"), py_str("good"), py_str("decent"), py_str("ok")]));
    s_CHILD_WORDS(py_tuple(vec![py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("childrens"), py_str("childs"), py_str("toddler"), py_str("toddlers"), py_str("baby"), py_str("babies"), py_str("boy"), py_str("boys"), py_str("girl"), py_str("girls"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("teens"), py_str("grandchild"), py_str("grandchildren"), py_str("grandson"), py_str("granddaughter"), py_str("niece"), py_str("nephew"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("school age"), py_str("year old"), py_str("year old")]));
    s_CHEAP_ADULT(py_str("Spend it on something consumable and a notch better than the everyday: a good candle, decent coffee or tea, a nice bar of chocolate, a scratch card, seeds and a pot, a paperback you would actually recommend, or a really good pair of socks. £10 goes a long way on any of those and nothing needs a size or a taste test; if the group has a joke running, a silly gift that gets a laugh beats an expensive one that gets a polite smile. Keep the receipt."));
    s_CHEAP_CHILD(py_str("For a child, spend it on things they use up or grow out of: bubble mix, a big sticker book, a colouring pad with good pens, bath crayons, play dough, a torch, a pack of cards, or a small Lego set on offer. Nothing needs a size and batteries are the only trap, so check the box. Socks, gloves and a book they will actually read are the safe last-minute ones."));
    s_CHEAP_TEEN(py_str("For a teenager, a tenner is best spent on one thing they pick themselves: a voucher for the cinema or a shop they already use, phone credit, decent earphones or the cable they keep losing, or a game they can play with friends. Ask what they are into and buy the small version of that: the money goes further on one thing they want than on three they do not."));
    s_PRICE_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\u00a3|\\d+\\s*(?:pounds?|quid)\\b|\\bunder\\s+\\d|\\bbudget\\b")); __args.push(stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_VERDICT_WORD(py_str("(?:weird|odd|tacky|rude|appropriate|all\\s+right|ok|okay|too\\s+much|too\\s+little|too\\s+expensive|too\\s+cheap|wrong|enough|worth\\s+it|worth\\s+the\\s+money|a\\s+good\\s+(?:present|gift|idea)|a\\s+bad\\s+(?:present|gift|idea))"));
    s_A_VERDICT_ON_A_PRESENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:is|would|will|was)\\s+(?:(?:that|it|this|a|an|my|the)\\s+)?[^?.!]{0,40}?\\b"), &g_A_VERDICT_WORD()), &py_str("\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_VERDICT_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b(?:is|would|will|was)\\s+(?:(?:that|it|this)\\s+)?([^?.!,]{2,30}?)\\s+(?:be\\s+)?(?:"), &g_A_VERDICT_WORD()), &py_str(")\\b"))); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_VERDICT_YES(py_tuple(vec![py_str("No, that is a good pick at {age}: {thing} gets used every day, and the plain one with room to grow beats the flashy one."), py_str("Not at all - at {age} {thing} is exactly the kind of present that gets used. Plain and sturdy is the right call there."), py_str("That sounds right to me: at {age}, {thing} is a good present, and asking the parents what they already have too much of saves you buying the second one.")]));
    s_MONEY_VERDICT_YES(py_tuple(vec![py_str("No need to spend much: at {age} {thing} is a good present, and the plain one is the one that gets used, so the extra money buys nothing the child will notice."), py_str("No, and cheap is right here: at {age} {thing} gets used hard, and the basic version lasts as long as the expensive one.")]));
    s_A_VERDICT_ABOUT_MONEY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:too\\s+expensive|too\\s+cheap|expensive|cost|costs|price|pricier|worth\\s+it|spend|spending|afford|afford\\s+it)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_A_CARD_IS_NOT_A_PRESENT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bcard(?:s)?\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_NOT_THAT_CARD({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:bank|debit|credit|sim|sd|memory|store|loyalty|id|payment|graphics|smart|gift)\\s*cards?\\b|\\bcard\\s+(?:declined|blocked|number|details|payment|reader|machine)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CARD_AFTER_A_PRESENT(py_tuple(vec![py_str("The card is the part people keep: two lines in your own words - one thing they did that you still think about, and a wish - beats anything printed inside it."), py_str("A card is worth the five minutes. One specific line about them, no joke off the shelf, and your name at the bottom, because that is the bit they keep."), py_str("Write the card yourself rather than signing a printed one: name the thing you like about them and what you hope for them, and stop there.")]));
    s_A_CHEAP_ASK({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:cheap|cheaper|cheapest|inexpensive|budget|tenner|fiver|stocking\\s+fillers?|stocking\\s+filler|secret\\s+santa|small\\s+gifts?|small\\s+presents?|little\\s+gifts?|under\\s+a\\s+(?:tenner|fiver|ten|five))\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_FOR_A_CONDITION(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("doesnt drink"), py_str("doesn't drink"), py_str("does not drink"), py_str("not drinking"), py_str("non drinker"), py_str("non-drinker"), py_str("nondrinker"), py_str("teetotal"), py_str("teetotaller"), py_str("sober"), py_str("given up drinking"), py_str("given up alcohol"), py_str("gave up drinking"), py_str("stopped drinking"), py_str("off the drink"), py_str("no alcohol"), py_str("cant drink"), py_str("can't drink"), py_str("cannot drink"), py_str("not a drinker")]), py_str("Keep the alcohol out of it, even in a hamper: a good alcohol-free bottle (Lyre's or Seedlip), a proper coffee or a loose-leaf tea set, or a voucher for a meal out."), py_tuple(vec![py_str("a decent bottle of what he already drinks, "), py_str("a decent bottle of what he already drinks"), py_str("a little of something she loves to eat, "), py_str("a bottle of what she already drinks, ")])]), py_tuple(vec![py_tuple(vec![py_str("vegetarian"), py_str("veggie"), py_str("vegan"), py_str("plant based"), py_str("doesnt eat meat"), py_str("doesn't eat meat"), py_str("no meat"), py_str("pescatarian")]), py_str("And buy into the vegetarian side of it: a good vegetarian cookbook, a voucher for a veggie restaurant, or a decent olive oil and a spice set rather than a hamper of pate."), py_tuple(vec![])]), py_tuple(vec![py_tuple(vec![py_str("diabetic"), py_str("diabetes"), py_str("on a diet"), py_str("dieting"), py_str("watching sugar"), py_str("no sugar"), py_str("cutting out sugar"), py_str("blood sugar")]), py_str("Keep the sugar out of it: no chocolates or biscuit hamper - a magazine or audiobook subscription, or something for a hobby they already have, works better."), py_tuple(vec![py_str("chocolates")])]), py_tuple(vec![py_tuple(vec![py_str("coeliac"), py_str("celiac"), py_str("gluten free"), py_str("gluten-free"), py_str("dairy free"), py_str("dairy-free"), py_str("lactose"), py_str("allergy"), py_str("allergic"), py_str("allergies"), py_str("nut allergy"), py_str("intolerance")]), py_str("And nothing that might hold what they cannot eat: a voucher, a plant, or something for a hobby beats a hamper or a box of chocolates."), py_tuple(vec![])])]));
    s__PRESENT_PEOPLE(py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("sister"), py_str("brother"), py_str("son"), py_str("daughter"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("grandad"), py_str("grandma"), py_str("nan"), py_str("granny"), py_str("aunt"), py_str("uncle"), py_str("cousin"), py_str("friend"), py_str("mate"), py_str("colleague"), py_str("coworker"), py_str("boss"), py_str("neighbour"), py_str("neighbor"), py_str("teacher"), py_str("carer"), py_str("nurse"), py_str("stepmum"), py_str("stepdad"), py_str("boyfriend"), py_str("girlfriend"), py_str("teenager"), py_str("child"), py_str("baby")]));
    s_NOT_THE_THING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\nthat this they them their there here what have has had with from your yours been being every\neach some more most much only also just like both other another same sure back over into than\nthen age ages day days hour hours week weeks month months year years time times little bit\nlot lots thing things present present gift gifts idea ideas kind sort\n"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLURAL_ONLY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:them|they|theirs|their)\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_PLURAL_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("they them their theirs"), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_GIFT_ASK(py_tuple(vec![py_str("gift"), py_str("present"), py_str("get"), py_str("got"), py_str("buy"), py_str("christmas"), py_str("birthday"), py_str("60th"), py_str("70th"), py_str("80th"), py_str("idea"), py_str("for my"), py_str("for his"), py_str("for her")]));
    s_LOSS(py_tuple(vec![py_str("died"), py_str("death"), py_str("passed away"), py_str("funeral"), py_str("buried"), py_str("grief"), py_str("grieving"), py_str("bereavement"), py_str("widow"), py_str("miss her"), py_str("miss him"), py_str("miss mum"), py_str("miss dad")]));
    s_CRISIS(py_tuple(vec![py_str("fell"), py_str("fall"), py_str("fallen"), py_str("floor"), py_str("cant get up"), py_str("can't get up"), py_str("cannot get up"), py_str("hospital"), py_str("ambulance"), py_str("unresponsive"), py_str("not breathing"), py_str("collapsed"), py_str("stroke"), py_str("chest pain"), py_str("in a coma")]));
    s_CHOOSING(py_tuple(vec![py_tuple(vec![py_str("pick"), py_str("picking"), py_str("choose"), py_str("choosing"), py_str("choice")]), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("get a"), py_str("getting a"), py_str("shopping"), py_str("need a"), py_str("need some"), py_str("want a"), py_str("looking for a")]), py_tuple(vec![py_str("which"), py_str("what should i"), py_str("what to look for"), py_str("look for"), py_str("recommend"), py_str("recommendation"), py_str("suggestion"), py_str("what to get"), py_str("which to get"), py_str("what to buy"), py_str("which to buy"), py_str("dont know what"), py_str("don't know what"), py_str("not sure what"), py_str("not sure which"), py_str("which laptop"), py_str("laptop to get"), py_str("what laptop")]), py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("best"), py_str("spend"), py_str("budget"), py_str("upgrade"), py_str("replace")]), py_tuple(vec![py_str("how much ram"), py_str("how much memory"), py_str("how much storage"), py_str("what specs"), py_str("specs"), py_str("processor"), py_str("how fast"), py_str("what size"), py_str("how big"), py_str("screen size")])]));
    s_PERSON_ASK(py_tuple(vec![py_str("gift"), py_str("present"), py_str("presents"), py_str("what should i get"), py_str("what should i buy"), py_str("what can i get"), py_str("what can i buy"), py_str("what to get"), py_str("what to buy"), py_str("what would you get"), py_str("what would you buy"), py_str("what do i get"), py_str("what do i buy"), py_str("what do you get"), py_str("what do you buy"), py_str("what do you give"), py_str("what do you get them"), py_str("what do you buy them"), py_str("what to get them"), py_str("what should i bring"), py_str("what can i give"), py_str("what to give"), py_str("what should i give"), py_str("what could i give"), py_str("what do i give"), py_str("what would you give")]));
    s_TODDLER_GIFT(py_str("For a toddler, one thing they will use every day beats a pile of toys: a set of chunky blocks, a ball, a book with flaps to lift, or a scooter they will still fit next year. Ask the parents what they already have too much of, and buy the batteries with it."));
    s_CHILD_GIFT(py_str("For that age, one thing they will use every day beats a pile of toys: a scooter, duplo, sticker and activity books, or a decent football. Ask the parents what they already have too much of, and buy the batteries with it. Wrap it in something they can play with afterwards."));
    s_TEEN_GIFT(py_str("For a teenager, money or a voucher beats a guess, and headphones, gaming credit, a hoodie from a shop they like or a ticket to something are the safe few. Ask them once, buy the thing they named, and keep the change in a card with a real message in it."));
    s_GROWN_GIFT(py_str("For that age, buy something they would use every week and would not buy themselves: good coffee or tea, a bottle of what they already drink, a voucher for a place they like, or a book by an author they follow. One useful thing beats a pile of small ones."));
    s_PRESENT_OPENINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_TODDLER_GIFT()); __args.push(g_CHILD_GIFT()); __args.push(g_TEEN_GIFT()); __args.push(g_GROWN_GIFT()); __args.push(g__GIFT_ANYONE()); __args.push(g_CHEAP_ADULT()); __args.push(g_CHEAP_CHILD()); __args.push(g_CHEAP_TEEN()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _openings(args))(&__args) });
    s_AGE_ASK(py_tuple(vec![py_str("gift"), py_str("presents"), py_str("present"), py_str("what should i get"), py_str("what should i buy"), py_str("what can i get"), py_str("what can i buy"), py_str("what to get"), py_str("what to buy"), py_str("what would you get"), py_str("what would you buy"), py_str("what do i get"), py_str("what do i buy"), py_str("what do you get"), py_str("what do you buy"), py_str("what do you give"), py_str("what do you get them"), py_str("what do you buy them"), py_str("what to get them"), py_str("what can i give"), py_str("what to give"), py_str("what should i give"), py_str("what do i give")]));
    s_APPLIANCE_VALUE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("air fryer"), py_str("air fryers"), py_str("airfryer")]), py_str("An air fryer is a small oven that blows hot air fast: chips, roast veg and salmon in 10 to 15 minutes with a spoon of oil. Worth it for one or two people who hate heating a big oven; skip it for four, since a 4-litre basket is one tray. £50 to £80 is plenty.")]), py_tuple(vec![py_tuple(vec![py_str("slow cooker"), py_str("slow cookers")]), py_str("A slow cooker is worth it if you are out all day: load it before work and cheap cuts are tender by dinner. Skip it if you want anything crisp or fast. A £30 one cooks the same as a £100 one.")]), py_tuple(vec![py_tuple(vec![py_str("pressure cooker"), py_str("instant pot"), py_str("pressure cookers")]), py_str("A pressure cooker is worth it if you cook beans, stews or tough meat from scratch: a beef stew is 40 minutes instead of two hours, and dried beans are 25. Skip it for anything quick or crisp, and know that it is a big pan to store. A 6-litre one suits most kitchens.")]), py_tuple(vec![py_tuple(vec![py_str("bread maker"), py_str("bread machine"), py_str("bread makers")]), py_str("A bread maker is not worth it for most people: skip it unless you eat a loaf a day, since it takes three to four hours for a small loaf with a soft crust, and a £15 tin in a hot oven makes better bread. It earns its space only if you want the dough made overnight with no hands.")]), py_tuple(vec![py_tuple(vec![py_str("dishwasher"), py_str("dishwashers")]), py_str("A dishwasher is worth it for three or more people: it uses less water than washing up by hand and saves about half an hour a day. Skip it if you live alone and cook little, since filling it takes a week. Check the noise (under 45dB) if the kitchen is also the living room, and a 45cm slimline model fits a small kitchen.")]), py_tuple(vec![py_tuple(vec![py_str("heat pump dryer"), py_str("heat pump dryers"), py_str("heat pump tumble dryer"), py_str("heat pump tumble drier"), py_str("heat pump drier"), py_str("heat pump condensing dryer")]), py_str("A heat pump dryer is worth it if you dry two or more loads a week: it uses about half the electricity of a vented one, roughly 25p a cycle against 60p, so 150 to 200 pounds more up front pays back in about two years. It runs cooler, so a load takes an hour longer, and the fluff filter and condenser want clearing often. Skip it if you dry rarely, if the room is cold, or if you have nowhere to empty the water.")]), py_tuple(vec![py_tuple(vec![py_str("tumble dryer"), py_str("tumble dryers")]), py_str("A tumble dryer is worth it if you have no outside space or a house full of laundry: a heat-pump dryer costs more (£350 up) but uses about half the electricity of a vented one. Skip it if you have a garden and an airer: it is the most expensive way to dry clothes.")]), py_tuple(vec![py_tuple(vec![py_str("smart thermostat"), py_str("smart thermostats"), py_str("heating thermostat"), py_str("nest thermostat"), py_str("hive thermostat"), py_str("tado"), py_str("thermostatic radiator valve"), py_str("thermostatic radiator valves"), py_str("trv"), py_str("trvs")]), py_str("A smart thermostat is worth it if the house is empty in the day: about £50 to £100 a year off a gas bill, so £150 to £250 fitted pays back in two to three years. Skip it if someone is home all day, or if the heating is already set to come on only when you are in: turning the flow temperature down and putting thermostatic valves on the radiators gets much of the saving for a fraction of the price.")]), py_tuple(vec![py_tuple(vec![py_str("water filter jug"), py_str("water filter jugs"), py_str("filter jug"), py_str("brita"), py_str("water filter")]), py_str("A water filter changes the taste and takes out the chlorine, and a plumbed-in one under the sink is the same job done once instead of every jug: neither softens hard water nor takes out much else. Worth it if your tap water tastes of chlorine and you drink it cold all day, at about £20 for the jug and £4 a month for cartridges. A jug of tap water left in the fridge overnight does most of the same for nothing, so try that first.")]), py_tuple(vec![py_tuple(vec![py_str("humidifier"), py_str("humidifiers")]), py_str("A humidifier is worth it in one dry room, usually a bedroom with the heating on and the windows shut, if you wake with a dry throat or nosebleeds, or have a cold. Skip it in a room that has damp on the walls or condensation on the glass, because adding moisture makes that worse, and clean it every week: a dirty humidifier sprays mould into the air. £30 to £60 for a cool-mist one.")]), py_tuple(vec![py_tuple(vec![py_str("mattress topper"), py_str("mattress toppers"), py_str("bed topper")]), py_str("A mattress topper is worth it if the mattress is sound but too firm, or you want it softer: 5cm of memory foam or a wool topper costs £60 to £120 and changes the feel for a fraction of a new mattress. It will not fix a mattress that sags, is over about eight years old or gives you backache, and a topper on a bad mattress is £100 spent on the wrong thing.")]), py_tuple(vec![py_tuple(vec![py_str("water softener"), py_str("water softeners"), py_str("water softening")]), py_str("A water softener is worth it in a hard water area if scale is a daily nuisance: kettles, shower screens and the washing machine stop furring up, and the boiler keeps its efficiency. It is about £1,000 to £2,000 fitted plus £100 to £150 a year in salt, so it is a family and hard-water answer rather than a flat one. A filter jug does not soften water, and a magnetic or electronic conditioner does not take the calcium out.")]), py_tuple(vec![py_tuple(vec![py_str("robot mower"), py_str("robot mowers"), py_str("robot lawn mower"), py_str("robotic mower")]), py_str("A robot mower is worth it for a flat, open lawn with no narrow gaps: it cuts a little every day, so the grass stays tidy and there is no grass to collect. It costs £500 to £1,500 and most need a boundary wire laid round the edge. Skip it for a small lawn, a steep slope, or one full of bulbs and toys.")]), py_tuple(vec![py_tuple(vec![py_str("video doorbell"), py_str("video doorbells"), py_str("ring doorbell"), py_str("doorbell camera")]), py_str("A video doorbell is worth it if parcels go missing or you cannot hear the door from where you work: £50 to £120, and stored clips usually need a subscription of £2 to £4 a month unless you take a local-storage model. Wired is more reliable than battery. Point it at your own path and door rather than the pavement, and if you rent, ask before drilling.")]), py_tuple(vec![py_tuple(vec![py_str("fan heater"), py_str("fan heaters"), py_str("electric heater"), py_str("electric heaters"), py_str("oil filled radiator"), py_str("convector heater"), py_str("space heater"), py_str("halogen heater")]), py_str("A fan heater is worth it for one room for twenty minutes, not for a whole evening or a whole house: cheap to buy at £20 to £40 and dear to run, since 2kW is about 60p an hour on current electricity prices. For a room you sit in, an oil-filled radiator is silent and holds the heat after it clicks off, and heating the person beats heating the room: a throw, a hot water bottle and warm socks are free after the first winter.")]), py_tuple(vec![py_tuple(vec![py_str("fan"), py_str("fans"), py_str("electric fan"), py_str("tower fan"), py_str("pedestal fan"), py_str("desk fan"), py_str("floor fan"), py_str("standing fan"), py_str("cooling fan")]), py_str("A fan does not cool the air, it moves it, and moving air over your skin is enough to feel a few degrees cooler, so it is worth it for sleeping in a hot room: a tower or pedestal fan at £25 to £60 is much quieter than a cheap one, and it costs a penny or two an hour to run. Put it in the window at night to pull cool air in, and skip portable air conditioning unless the room is also humid, since that is the expensive way round.")]), py_tuple(vec![py_tuple(vec![py_str("robot vacuum"), py_str("robot hoover"), py_str("robot vacuums")]), py_str("A robot vacuum is worth it as a daily top-up on hard floors and one level, not as a deep clean: skip it if the floor has stairs, thick rugs or a trail of cables and toys, since it will not get them and you still empty it. Best value in a house with a pet that sheds.")]), py_tuple(vec![py_tuple(vec![py_str("dehumidifier"), py_str("dehumidifiers")]), py_str("A dehumidifier is worth it in a flat that steams up and grows black mould on the window frames: a 10 to 12 litre one dries washing indoors and stops the mould, which costs more to fix than the machine. Skip it if the damp is a leak or a broken gutter, since that needs fixing at the source.")]), py_tuple(vec![py_tuple(vec![py_str("coffee machine"), py_str("bean to cup"), py_str("coffee machines")]), py_str("A coffee machine is worth it if you drink two coffees a day at home: a £150 bean-to-cup pays for itself in about three months against buying them out. Skip it if you have one coffee a day: a £30 filter machine with fresh ground coffee tastes better than a cheap pod machine, and the pods are the expensive part.")]), py_tuple(vec![py_tuple(vec![py_str("stand mixer"), py_str("kitchenaid"), py_str("stand mixers")]), py_str("A stand mixer is worth it if you bake most weeks: it creams butter and kneads dough for you and lasts decades. Skip it if you bake a few times a year: a £15 hand mixer does the same job and lives in a drawer.")]), py_tuple(vec![py_tuple(vec![py_str("meal kit"), py_str("meal kits"), py_str("recipe box"), py_str("hello fresh"), py_str("gousto")]), py_str("A meal kit is worth it for the decision taken away, not the food: about £5 a portion and no waste. Skip it if you already cook, since the same dinner is £1.50 a portion from a supermarket. Try two boxes and cancel.")]), py_tuple(vec![py_tuple(vec![py_str("kindle"), py_str("e reader"), py_str("ereader"), py_str("e-reader")]), py_str("A Kindle is worth it if you read in bed or travel: a Paperwhite is readable in the dark, holds hundreds of books and the battery lasts weeks. Skip it if you mostly read at home: library ebooks through Libby or BorrowBox are free on a phone, and paper is better for anything with maps or photos. Buy last year's model, not the newest.")]), py_tuple(vec![py_tuple(vec![py_str("noise cancelling headphones"), py_str("noise cancelling headphone"), py_str("noise cancelling")]), py_str("Noise cancelling headphones are worth it if you commute by train, plane or sit in a loud office: over-ear ones cut the engine noise that earbuds let through. About £70 to £120 is the sweet spot, and last year's model sounds the same as the new one for half the price. Skip them for a quiet room or for cycling, where hearing traffic matters.")]), py_tuple(vec![py_tuple(vec![py_str("fitness tracker"), py_str("fitness trackers"), py_str("fitbit")]), py_str("A fitness tracker is worth it for steps and sleep, not for the heart-rate number: a £25 band counts steps and sleep well, and a watch ten times the price adds mostly a screen and GPS. Skip it if you already walk enough and know it, since most people stop looking after a month.")]), py_tuple(vec![py_tuple(vec![py_str("smart watch"), py_str("smart watches"), py_str("apple watch")]), py_str("A smart watch is worth it if you want notifications, contactless payment and a run tracker on your wrist without getting your phone out. Skip it if you only want steps, where a £25 band does the same. An iPhone pairs best with an Apple Watch, an Android with a Wear OS watch or a Garmin.")]), py_tuple(vec![py_tuple(vec![py_str("standing desk"), py_str("standing desks"), py_str("sit stand desk")]), py_str("A standing desk is worth it if you sit for eight hours and your back aches, and the useful part is changing position rather than standing all day: a £25 riser on the desk you have does most of it. Skip the motorised ones unless you know you will raise them, and stand for 20 minutes at a time rather than the afternoon.")]), py_tuple(vec![py_tuple(vec![py_str("weighted blanket"), py_str("weighted blankets")]), py_str("A weighted blanket is worth it if the feeling of weight settles you: about 10% of your body weight is the usual guide, and it helps some people fall asleep and does nothing for others. Skip it for a toddler, or for anyone who could not push it off, and try one under a duvet before spending £60.")]), py_tuple(vec![py_tuple(vec![py_str("steam cleaner"), py_str("steam cleaners"), py_str("steam mop")]), py_str("A steam cleaner is worth it for hard floors, tiles and grout, and the bathroom: it lifts grease with no chemicals and the water is free after the machine. Skip it on laminate, unsealed wood and anywhere the moisture can get under the floor, and skip it for carpets, where a hire machine once a year is cheaper than owning one.")]), py_tuple(vec![py_tuple(vec![py_str("a cleaner"), py_str("the cleaner"), py_str("my cleaner"), py_str("our cleaner"), py_str("cleaners worth"), py_str("cleaning lady"), py_str("cleaning service"), py_str("housekeeper")]), py_str("A cleaner is worth it if two hours back at the weekend matter more than £15 to £20 an hour: two hours a fortnight covers a flat. Go through an agency with insurance, agree in writing what is included and be in the first time. Skip it while you are paying off a debt.")]), py_tuple(vec![py_tuple(vec![py_str("food processor"), py_str("food processors"), py_str("kitchen machine")]), py_str("A food processor is worth it if you chop a lot of onions or make pastry: it does in a minute what a knife takes ten. Skip it if you cook simply, since it is heavy to wash up for one job. A £40 one chops as well as a £200 one.")]), py_tuple(vec![py_tuple(vec![py_str("air purifier"), py_str("air purifiers")]), py_str("An air purifier is worth it in a bedroom if you have hay fever, a cat or a sealed window: a HEPA filter on low all night changes how you sleep. Skip it for damp, which needs a dehumidifier, and check the replacement filters are cheap before you buy.")]), py_tuple(vec![py_tuple(vec![py_str("heated airer"), py_str("heated clothes airer"), py_str("heated drying rack"), py_str("heated clothes horse")]), py_str("A heated airer is worth it if you dry clothes indoors in winter: about 6p an hour against 50p or more for a tumble dryer, and it does not shrink anything. Skip it if you have a garden and dry outside in summer, and buy the larger winged one so a full wash fits on it. A cover cuts the drying time by half.")]), py_tuple(vec![py_tuple(vec![py_str("electric blanket"), py_str("electric blankets"), py_str("heated blanket")]), py_str("An electric blanket is worth it for a cold bedroom: 20 minutes before bed costs pennies and beats heating the room all evening. Skip it if it would be on all night, since a hot water bottle does the same for nothing, and replace one over ten years old.")]), py_tuple(vec![py_tuple(vec![py_str("microwave"), py_str("microwaves")]), py_str("A microwave is worth it for reheating, porridge, scrambled eggs and defrosting, not for cooking a meal from scratch: skip it if you want to roast or bake, since an oven does that better. A basic 800W one is enough; pay more for a bigger turntable and easier controls, not for modes nobody uses.")])]));
    s_CHOICE_VALUE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("gym membership"), py_str("gym memberships"), py_str("join a gym"), py_str("joining a gym"), py_str("gym worth"), py_str("should i go to the gym"), py_str("should i go to gym"), py_str("shall i go to the gym"), py_str("should i go back to the gym"), py_str("should i go to the gym again"), py_str("should i bother going to the gym"), py_str("should i bother with the gym"), py_str("should i bother with a gym"), py_str("should i start going to the gym"), py_str("is it worth going to the gym"), py_str("worth going to the gym"), py_str("any point going to the gym"), py_str("should i go to the gym or")]), py_str("A gym is worth it if the door is within ten minutes of you and you will go twice a week: the walk decides it, not the equipment. Skip it if the nearest one is a drive away, and pay monthly rather than a year up front. A £20 leisure-centre one you use beats a £60 one you dread.")]), py_tuple(vec![py_tuple(vec![py_str("heat pump dryer"), py_str("heat pump tumble dryer"), py_str("heat pump drier"), py_str("heat pump tumble drier"), py_str("heat pump dryers"), py_str("heat pump condensing dryer")]), py_str("A heat pump dryer is worth it if you dry two or more loads a week: the cycle uses about half the electricity of a vented one, roughly 25p against 60p, so it pays back its extra 150 to 200 pounds in about two years. It runs cooler and slower, so a load takes two hours and more, and it needs the fluff filter and condenser cleared often. Skip it if you dry rarely or have nowhere for the waste water, and never buy one without a heat pump if the room is cold.")]), py_tuple(vec![py_tuple(vec![py_str("heat pump"), py_str("heat pumps")]), py_str("A heat pump is worth it if you are replacing the boiler anyway and the house is insulated: the grant covers a few thousand, the running cost is close to gas, and it heats a well-insulated home steadily rather than in bursts. Skip it in a draughty house with no wall or loft insulation, since the insulation comes first and pays back faster.")]), py_tuple(vec![py_tuple(vec![py_str("electric car"), py_str("electric cars"), py_str("ev ")]), py_str("An electric car is worth it if you can charge at home and do under about 200 miles a week: home charging is roughly 3p a mile against 15p for petrol, and there is almost no servicing. Skip it if you cannot charge at home, or if you regularly do long motorway runs, where public charging costs nearly as much as fuel.")]), py_tuple(vec![py_tuple(vec![py_str("smart meter"), py_str("smart meters")]), py_str("A smart meter is worth having: your supplier fits one free, and the in-home display shows what the kettle and the heating cost as you use them. Ask for a second-generation (SMETS2) one so it keeps working if you switch supplier. It will not cut the bill on its own.")]), py_tuple(vec![py_tuple(vec![py_str("water meter"), py_str("water meters")]), py_str("A water meter usually saves money if there are more bedrooms than people: the rule of thumb is that you pay less on a meter when there are fewer of you than rooms. Check the bill estimate first, and remember you can go back to an unmetered bill within the first year in England and Wales if it costs you more.")]), py_tuple(vec![py_tuple(vec![py_str("season ticket"), py_str("season tickets")]), py_str("A season ticket pays off at about three days a week: below that, the flexible or weekly options are usually cheaper. Work out the yearly cost against 40 weeks of daily returns before you commit, and check whether a part-time or flexi season covers the days you actually travel.")])]));
    s_APPLIANCE_ASK(py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("worth the money"), py_str("should i get"), py_str("should i buy"), py_str("do i need"), py_str("need one"), py_str("shall i get"), py_str("good buy"), py_str("any good"), py_str("is it good"), py_str("a good"), py_str("good cheap"), py_str("good and cheap"), py_str("the best"), py_str("best one"), py_str("buy"), py_str("buying"), py_str("getting"), py_str("get one"), py_str("should i use"), py_str("should we use"), py_str("should i try"), py_str("is it ok"), py_str("is it safe"), py_str("should i go"), py_str("shall i go"), py_str("should we go"), py_str("should i start"), py_str("should i bother"), py_str("is it worth going"), py_str("worth going"), py_str("would you"), py_str("will it help"), py_str("does it help"), py_str("recommend"), py_str("recommended"), py_str("recommendation"), py_str("recommendations"), py_str("will a"), py_str("will an"), py_str("does a"), py_str("does an"), py_str("would a"), py_str("would an"), py_str("will my"), py_str("does my"), py_str("would my"), py_str("any point in"), py_str("any point to"), py_str("point in a"), py_str("point in an"), py_str("point in"), py_str("point to"), py_str("point of a"), py_str("point of an"), py_str("waste of money"), py_str("wasting money"), py_str("money well spent"), py_str("bother with"), py_str("bother")]));
    s__METER_FAULT(py_tuple(vec![py_str("blank"), py_str("stopped"), py_str("broken"), py_str("faulty"), py_str("dead"), py_str("warning"), py_str("error"), py_str("gone"), py_str("connect"), py_str("sending"), py_str("signal"), py_str("wrong"), py_str("wtong")]));
    s__METER_READ(py_tuple(vec![py_str("read"), py_str("reading"), py_str("readings"), py_str("button"), py_str("show"), py_str("shows"), py_str("find"), py_str("mean"), py_str("means"), py_str("numbers"), py_str("figure"), py_str("figures"), py_str("kwh")]));
    s__DECISION_WORDS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("i"), py_str("me"), py_str("my"), py_str("we"), py_str("us"), py_str("our"), py_str("you"), py_str("your"), py_str("it"), py_str("its"), py_str("im"), py_str("ive"), py_str("id"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("am"), py_str("do"), py_str("does"), py_str("did"), py_str("doing"), py_str("done"), py_str("be"), py_str("been"), py_str("being"), py_str("should"), py_str("shall"), py_str("would"), py_str("will"), py_str("can"), py_str("could"), py_str("may"), py_str("might"), py_str("must"), py_str("worth"), py_str("any"), py_str("point"), py_str("in"), py_str("to"), py_str("of"), py_str("for"), py_str("at"), py_str("on"), py_str("with"), py_str("and"), py_str("or"), py_str("so"), py_str("if"), py_str("that"), py_str("this"), py_str("these"), py_str("those"), py_str("then"), py_str("than"), py_str("now"), py_str("today"), py_str("tomorrow"), py_str("tonight"), py_str("later"), py_str("still"), py_str("just"), py_str("really"), py_str("actually"), py_str("honestly"), py_str("though"), py_str("again"), py_str("back"), py_str("go"), py_str("goes"), py_str("going"), py_str("went"), py_str("get"), py_str("gets"), py_str("getting"), py_str("got"), py_str("bother"), py_str("bothering"), py_str("start"), py_str("starting"), py_str("started"), py_str("try"), py_str("trying"), py_str("buy"), py_str("buying"), py_str("join"), py_str("joining"), py_str("one"), py_str("ones"), py_str("thing"), py_str("things"), py_str("time"), py_str("times"), py_str("not"), py_str("no"), py_str("yes"), py_str("ok")])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__ACTION_NAME({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(go|going|went|start|starting|bother|bothering|try|trying)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__TRADER_WORDS(py_tuple(vec![py_str("builder"), py_str("builders"), py_str("plumber"), py_str("plumbers"), py_str("electrician"), py_str("electricians"), py_str("decorator"), py_str("decorators"), py_str("plasterer"), py_str("plasterers"), py_str("roofer"), py_str("roofers"), py_str("joiner"), py_str("joiners"), py_str("tiler"), py_str("tilers"), py_str("carpenter"), py_str("carpenters"), py_str("gardener"), py_str("gardeners"), py_str("tradesman"), py_str("tradesmen"), py_str("trade"), py_str("contractor"), py_str("contractors"), py_str("quote"), py_str("quotes"), py_str("quotation"), py_str("quotations"), py_str("day rate"), py_str("labour"), py_str("estimate")]));
    s__DRYER_WORDS(py_tuple(vec![py_str("dryer"), py_str("dryers"), py_str("drier"), py_str("driers"), py_str("tumble dryer"), py_str("tumble drier")]));
    s_EVERYDAY_WHERE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("stamp"), py_str("stamps"), py_str("postage"), py_str("first class"), py_str("second class"), py_str("signed for"), py_str("recorded delivery")]), py_str("Any post office counter has them, and so do most supermarkets, corner shops and newsagents. A book of 12 costs less per stamp than singles, and Royal Mail's own site will post a book out if you are not going past a counter.")]), py_tuple(vec![py_tuple(vec![py_str("birthday card"), py_str("birthday cards"), py_str("christmas cards"), py_str("christmas card"), py_str("greetings card"), py_str("greetings cards"), py_str("card shop"), py_str("cards")]), py_str("A card shop, a supermarket or a petrol station will have one, and the supermarket's own range is two pounds rather than four. If it is a big one, buy it this week rather than the day before - the good ones go first.")]), py_tuple(vec![py_tuple(vec![py_str("envelope"), py_str("envelopes"), py_str("stationery"), py_str("wrapping paper"), py_str("note paper"), py_str("printer paper"), py_str("pens"), py_str("a pen"), py_str("felt tips"), py_str("sellotape"), py_str("sticky tape"), py_str("glue"), py_str("a folder")]), py_str("A stationery shop or the back-to-school aisle of a supermarket has it, and for a batch envelopes or a box of paper an office supplier is cheaper and delivers. A card shop sells envelopes on their own if you only need a few.")]), py_tuple(vec![py_tuple(vec![py_str("sleeping bag"), py_str("sleeping bags"), py_str("tent"), py_str("rucksack"), py_str("camping gear"), py_str("camping stove"), py_str("walking boots")]), py_str("An outdoor shop is where the real ones are: Decathlon for the cheap end, Go Outdoors or a proper mountaineering shop for the rest. A charity shop or a car boot sale often has a decent sleeping bag for a tenner, and a borrowed one for a first trip is better than buying.")]), py_tuple(vec![py_tuple(vec![py_str("paint"), py_str("paintbrush"), py_str("paintbrushes"), py_str("roller"), py_str("dust sheet"), py_str("dust sheets"), py_str("undercoat"), py_str("primer")]), py_str("A decorator's merchant rather than a chain for the paint itself: the trade brands cover better, cost less and they will mix a colour to a sample. For one room, a DIY shop for the brushes, trays and sheets is fine, and cheap brushes are the false economy - they shed.")]), py_tuple(vec![py_tuple(vec![py_str("batteries"), py_str("battery"), py_str("light bulb"), py_str("lightbulb"), py_str("bulb"), py_str("fuse"), py_str("plug"), py_str("extension lead")]), py_str("A supermarket, a hardware shop or a pound shop has them. Take the old one with you, or photograph the numbers printed on it, because the size is the thing people get wrong: a bulb needs its fitting and its wattage, a battery its number, a fuse its amps.")]), py_tuple(vec![py_tuple(vec![py_str("boxes"), py_str("moving boxes"), py_str("cardboard boxes"), py_str("bubble wrap"), py_str("packing tape")]), py_str("A supermarket will give you banana boxes free if you ask at the customer desk, and a removal firm sells proper double-walled ones for a few pounds each. Ask on a Monday, before the delivery cages are taken back.")])]));
    s__INSIDE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:in|inside|under|behind|on top of|on)\\s+(?:the|my|our|your|their|his|her)\\s+([a-z'\\u2019]+)")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__MINE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:my|our)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SHOP_PLACES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str("\n    shop shops store stores supermarket supermarkets online internet ebay amazon argos\n    town city high street market sale sales chemist chemist's counter department\n    "), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__THE_EVERYDAY_THINGS({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_EVERYDAY_WHERE()) { let __unpk6962 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk6962[0].clone(); let mut _u: Py = __unpk6962[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_set(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_TURNING({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:turning|turns)\\s+(?P<age>\\d{1,3})\\b")); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_CHILD_PEOPLE(py_tuple(vec![py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("niece"), py_str("nephew"), py_str("grandson"), py_str("granddaughter"), py_str("grandchild"), py_str("grandchildren"), py_str("boy"), py_str("girl"), py_str("boys"), py_str("girls"), py_str("lad"), py_str("lads"), py_str("lass"), py_str("little one"), py_str("youngest"), py_str("eldest"), py_str("stepson"), py_str("stepdaughter"), py_str("godson"), py_str("goddaughter"), py_str("toddler"), py_str("schoolboy"), py_str("schoolgirl")]));
    s_CHILD_PRESENT(py_str("For a child the age is most of the answer, so it is worth asking. Under three, something to push, stack or post. Four to seven, something to build, draw or dress up. Eight to eleven, a kit or a game with real pieces. Twelve and up, money towards one thing they have chosen. With no age to go on, a good book and a small craft set is the safe pair, and paper and ribbon do the rest."));
    s__AGE_WORDS(py_str("one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen"));
    s__AN_AGE(op_add(&op_add(&py_str("(?:\\d{1,3}|"), &g__AGE_WORDS()), &py_str(")")));
    s__A_CHILDS_AGE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:"), &g__AN_AGE()), &py_str("\\s*(?:yr|yrs|year|years|yr-old|year-old)\\s*(?:old|ole)?\\b|")), &g__AN_AGE()), &py_str("\\s*months?\\b|\\b(?:hes|shes|he's|she's|he is|she is|who is|that is|is|aged|age|turning)\\s*")), &g__AN_AGE()), &py_str("\\b|\\b(?:1[0-9]|[1-9])(?:st|nd|rd|th)\\b|\\b(?:toddler|toddlers|preschool|preschooler|newborn|new born|teen|teens|teenager|teenagers|teenage)\\b)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__SAID_AS_WORDS(py_dict(vec![(py_str("one"), Py::Int(1)), (py_str("two"), Py::Int(2)), (py_str("three"), Py::Int(3)), (py_str("four"), Py::Int(4)), (py_str("five"), Py::Int(5)), (py_str("six"), Py::Int(6)), (py_str("seven"), Py::Int(7)), (py_str("eight"), Py::Int(8)), (py_str("nine"), Py::Int(9)), (py_str("ten"), Py::Int(10)), (py_str("eleven"), Py::Int(11)), (py_str("twelve"), Py::Int(12)), (py_str("thirteen"), Py::Int(13)), (py_str("fourteen"), Py::Int(14)), (py_str("fifteen"), Py::Int(15)), (py_str("sixteen"), Py::Int(16)), (py_str("seventeen"), Py::Int(17)), (py_str("eighteen"), Py::Int(18)), (py_str("nineteen"), Py::Int(19))]));
    s__AN_AGE_SAID({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&op_add(&op_add(&op_add(&op_add(&py_str("\\b(?:(?P<years>"), &g__AN_AGE()), &py_str(")\\s*(?:yr|yrs|year|years|yr-old|year-old)\\b|(?P<months>\\d{1,2}|")), &g__AGE_WORDS()), &py_str(")\\s*months?\\b|(?:hes|shes|he's|she's|he is|she is|who is|that is|is|aged|age|turning)\\s*(?P<near>")), &g__AN_AGE()), &py_str(")\\b|(?P<ordinal>1[0-9]|[1-9])(?:st|nd|rd|th)\\b)"))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__WATCH_SMART(py_tuple(vec![py_str("smart watch"), py_str("smartwatch"), py_str("smart watches"), py_str("smartwatches"), py_str("apple watch"), py_str("garmin"), py_str("fitbit"), py_str("galaxy watch"), py_str("fitness watch"), py_str("watch for running"), py_str("watch for steps"), py_str("track my steps"), py_str("track my sleep")]));
    s__WATCH_PLAIN(py_str("For a first proper watch, a plain quartz from Casio or Timex at £40 to £150 keeps better time than anything dearer; a mechanical watch is for people who enjoy them, not for accuracy. Look for a sapphire crystal if you can, 50m water resistance and a strap you can change yourself. What will you mostly wear it with?"));
    s__WATCH_SMART_SAY(py_str("Buy for the battery first: most smartwatches last a day or two, so one that lasts a week changes how you use it. A fitness band at £40 to £80 covers steps, sleep and notifications; pay £200 to £300 only if you want a screen and apps on your wrist, and expect the step and heart-rate numbers to be rough rather than medical. Is it for exercise, notifications, or looks?"));
    s__WATCH_VERBS(py_tuple(vec![py_str("watching"), py_str("watched"), py_str("to watch"), py_str("worth watching"), py_str("worth a watch"), py_str("watch it"), py_str("watch on"), py_str("should i watch"), py_str("binge")]));
    s__WATCH_VERB_SHAPE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\bwatch\\s+(?:out\\b|for\\s*(?:$|[?.!,;]|\\b(?:what|how|when|if|that|anything|something|signs?|symptoms?|changes?|it|them|him|her|me|us|you)\\b))")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_INTEREST_GIFTS(py_dict(vec![(py_str("fishing"), py_tuple(vec![py_tuple(vec![py_str("a float and hook kit"), Py::Int(12)]), py_tuple(vec![py_str("a decent tackle box"), Py::Int(30)]), py_tuple(vec![py_str("a landing net"), Py::Int(25)]), py_tuple(vec![py_str("a good folding chair for the bank"), Py::Int(40)])])), (py_str("angling"), py_tuple(vec![py_tuple(vec![py_str("a float and hook kit"), Py::Int(12)]), py_tuple(vec![py_str("a decent tackle box"), Py::Int(30)]), py_tuple(vec![py_str("a landing net"), Py::Int(25)]), py_tuple(vec![py_str("a good folding chair for the bank"), Py::Int(40)])])), (py_str("tackle"), py_tuple(vec![py_tuple(vec![py_str("a float and hook kit"), Py::Int(12)]), py_tuple(vec![py_str("a decent tackle box"), Py::Int(30)])])), (py_str("gardening"), py_tuple(vec![py_tuple(vec![py_str("good gloves that fit"), Py::Int(12)]), py_tuple(vec![py_str("a decent trowel"), Py::Int(20)]), py_tuple(vec![py_str("secateurs"), Py::Int(30)]), py_tuple(vec![py_str("a voucher for the nearest garden centre"), Py::Int(25)])])), (py_str("garden"), py_tuple(vec![py_tuple(vec![py_str("good gloves that fit"), Py::Int(12)]), py_tuple(vec![py_str("a decent trowel"), Py::Int(20)]), py_tuple(vec![py_str("secateurs"), Py::Int(30)]), py_tuple(vec![py_str("a voucher for the nearest garden centre"), Py::Int(25)])])), (py_str("plants"), py_tuple(vec![py_tuple(vec![py_str("a good houseplant in a proper pot"), Py::Int(15)]), py_tuple(vec![py_str("a nice watering can"), Py::Int(25)])])), (py_str("allotment"), py_tuple(vec![py_tuple(vec![py_str("good gloves that fit"), Py::Int(12)]), py_tuple(vec![py_str("secateurs"), Py::Int(30)]), py_tuple(vec![py_str("a voucher for the nearest garden centre"), Py::Int(25)])])), (py_str("cooking"), py_tuple(vec![py_tuple(vec![py_str("a decent chef's knife"), Py::Int(35)]), py_tuple(vec![py_str("a good olive oil and a nice tin"), Py::Int(20)]), py_tuple(vec![py_str("an apron and a decent wooden spoon"), Py::Int(15)])])), (py_str("baking"), py_tuple(vec![py_tuple(vec![py_str("a good loaf tin"), Py::Int(12)]), py_tuple(vec![py_str("a set of measuring spoons and a decent vanilla"), Py::Int(12)])])), (py_str("reading"), py_tuple(vec![py_tuple(vec![py_str("a paperback they would not pick themselves"), Py::Int(10)]), py_tuple(vec![py_str("a book token"), Py::Int(20)]), py_tuple(vec![py_str("a nice bookmark and a good bar of chocolate"), Py::Int(8)])])), (py_str("books"), py_tuple(vec![py_tuple(vec![py_str("a paperback they would not pick themselves"), Py::Int(10)]), py_tuple(vec![py_str("a book token"), Py::Int(20)])])), (py_str("music"), py_tuple(vec![py_tuple(vec![py_str("a record or CD they would not buy"), Py::Int(20)]), py_tuple(vec![py_str("a voucher for a record shop"), Py::Int(25)])])), (py_str("running"), py_tuple(vec![py_tuple(vec![py_str("a head torch"), Py::Int(20)]), py_tuple(vec![py_str("good running socks"), Py::Int(15)]), py_tuple(vec![py_str("a decent water bottle"), Py::Int(15)])])), (py_str("cycling"), py_tuple(vec![py_tuple(vec![py_str("a good bike light set"), Py::Int(25)]), py_tuple(vec![py_str("a decent multi-tool"), Py::Int(20)])])), (py_str("football"), py_tuple(vec![py_tuple(vec![py_str("a club shirt"), Py::Int(45)]), py_tuple(vec![py_str("good socks and a proper ball"), Py::Int(25)])])), (py_str("films"), py_tuple(vec![py_tuple(vec![py_str("a cinema voucher"), Py::Int(20)]), py_tuple(vec![py_str("a good film on disc"), Py::Int(12)])])), (py_str("photography"), py_tuple(vec![py_tuple(vec![py_str("a decent camera strap"), Py::Int(20)]), py_tuple(vec![py_str("a good memory card"), Py::Int(20)])])), (py_str("knitting"), py_tuple(vec![py_tuple(vec![py_str("a skein of good wool"), Py::Int(15)]), py_tuple(vec![py_str("a decent set of needles"), Py::Int(20)])])), (py_str("sewing"), py_tuple(vec![py_tuple(vec![py_str("a good pair of fabric scissors"), Py::Int(20)]), py_tuple(vec![py_str("a decent seam ripper and pins"), Py::Int(12)])])), (py_str("coffee"), py_tuple(vec![py_tuple(vec![py_str("good ground coffee"), Py::Int(15)]), py_tuple(vec![py_str("a decent cafetiere"), Py::Int(25)]), py_tuple(vec![py_str("a nice mug and a bag of good beans"), Py::Int(20)])])), (py_str("tea"), py_tuple(vec![py_tuple(vec![py_str("a proper loose-leaf tea and a strainer"), Py::Int(15)]), py_tuple(vec![py_str("a nice mug and good tea"), Py::Int(18)])])), (py_str("gaming"), py_tuple(vec![py_tuple(vec![py_str("a voucher for the shop they use"), Py::Int(25)]), py_tuple(vec![py_str("a good headset"), Py::Int(35)])])), (py_str("birdwatching"), py_tuple(vec![py_tuple(vec![py_str("a decent pair of binoculars"), Py::Int(45)]), py_tuple(vec![py_str("a bird book for their area"), Py::Int(15)])])), (py_str("diy"), py_tuple(vec![py_tuple(vec![py_str("a good screwdriver set"), Py::Int(25)]), py_tuple(vec![py_str("a decent tape measure"), Py::Int(12)])]))]));
    s_PRESENT_BANDS(py_tuple(vec![py_tuple(vec![Py::Int(10), py_str("Under {money}, buy one small thing properly rather than three cheap ones: a bar of the good chocolate, a decent paperback, a nice bar of soap, or good coffee.")]), py_tuple(vec![Py::Int(20), py_str("For under {money}, buy one thing they will use this month: a decent paperback, good chocolate, a jar of something they like, or a small plant in a proper pot.")]), py_tuple(vec![Py::Int(40), py_str("For under {money}, one good thing beats three safe ones: a decent thermos, woolly socks and a book, good olive oil and a nice tin, or a voucher for something they already do at the weekend.")]), py_tuple(vec![Py::Int(100), py_str("Under {money} you can buy an experience rather than an object: two cinema tickets and dinner, a day out they would not book themselves, or a decent bottle with a good glass to drink it from.")])]));
    s_PRESENT_OVER(py_str("At {money} there is room for the proper version of something they use every day and would not spend on themselves - a good bag, a decent pair of boots, or the thing they have been putting off - and a real message in the card."));
    s_BUY_INTO(py_str(" If you know what they are into, buy into that instead: secateurs for a gardener, a head torch for a runner, a knife for someone who cooks."));
    s__COAT_KINDS(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("down"), py_str("puffer"), py_str("padded"), py_str("padding"), py_str("quilted"), py_str("synthetic"), py_str("insulated"), py_str("insulation")]), py_str("Down is the warmest for its weight and packs down small, so look for 600 fill power or more and it will last a decade, but it insulates badly once it is wet. Synthetic padding costs about half as much, is heavier and bulkier, and still keeps you warm when damp, which suits a wet winter better than down unless you want it for real cold.")]), py_tuple(vec![py_tuple(vec![py_str("wedding"), py_str("funeral"), py_str("christening"), py_str("interview"), py_str("smart"), py_str("formal"), py_str("dress code"), py_str("party"), py_str("office"), py_str("town"), py_str("work")]), py_str("For anything smart - a wedding, a funeral, a winter interview - a wool or wool-blend coat is the answer: it hangs properly over a dress or a suit, and a mid-calf length in navy, grey, camel or black goes with everything. Check that it closes over the jacket you will wear underneath and try it on in the shoes you will be standing in, because a heavy coat pushes the shoulders up.")]), py_tuple(vec![py_tuple(vec![py_str("wool"), py_str("woollen"), py_str("woolen"), py_str("duffle"), py_str("tweed"), py_str("overcoat")]), py_str("A wool coat is warm, smart enough for town and fine in a shower, but it gets heavy once it is soaked and wants dry cleaning rather than a wash. Check the lining and the buttons, buy it loose enough for a jumper under it, and keep it for the days you are not out in the rain.")]), py_tuple(vec![py_tuple(vec![py_str("waterproof"), py_str("rain"), py_str("shell"), py_str("gore"), py_str("storm"), py_str("wet")]), py_str("For rain, a waterproof shell with taped seams (Gore-Tex or similar) at £150 to £250 outlasts three cheap ones: check the hood, the zip and the seams under the arms, and try it on over a jumper. A shower-resistant jacket is fine for a dash to the shops and no good for a wet walk.")]), py_tuple(vec![py_tuple(vec![py_str("worth"), py_str("expensive"), py_str("spend"), py_str("how much"), py_str("cheaper"), py_str("budget"), py_str("value"), py_str("price")]), py_str("Yes, if it is one you will wear for years: a coat is the piece of clothing where the money shows, and £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders. Cheap coats cost more in the end because the zip goes and the seams soak through, so buy the best you can in a plain colour you will still wear in five years.")])]));
    s__COAT_CHECK(py_str("Check the hood stays up in wind, the zip works with gloves on, the seams are taped and there is room for a jumper underneath. Look at the lining where it rubs under the arm and at the stitching on the pockets: those are the two places a cheap coat shows itself first."));
    s__HEIGHT({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(\\d)\\s*(?:foot|feet|ft|')\\s*(\\d{1,2})\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s__COAT_CHECK_WORDS(py_tuple(vec![py_str("look for"), py_str("look out for"), py_str("check"), py_str("what should i"), py_str("fit"), py_str("fits"), py_str("long"), py_str("length"), py_str("short"), py_str("tall"), py_str("size"), py_str("sizing"), py_str("cuff"), py_str("hood"), py_str("sleeve"), py_str("sleeves"), py_str("quality"), py_str("warm enough")]));
    s__NAMED_BY_PRONOUN(py_set(vec![py_str("her"), py_str("him"), py_str("them"), py_str("hers"), py_str("his"), py_str("theirs")]));
    s__KEEP_RECEIPT_RE({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:how long|how many|years?|months?|decades?|tax return|self assessment|expense claim|expenses claim|employer|for work|at work|work expenses)\\b")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "compile", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("wine-pick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wine"), py_str("wines"), py_str("red wine"), py_str("white wine"), py_str("rose"), py_str("prosecco"), py_str("supermarket wine"), py_str("red"), py_str("fizz"), py_str("champagne")]), py_tuple(vec![py_str("cheap"), py_str("cheaper"), py_str("good"), py_str("nice"), py_str("decent"), py_str("best"), py_str("pick"), py_str("choose"), py_str("buy"), py_str("buying"), py_str("recommend"), py_str("bargain"), py_str("tenner"), py_str("budget"), py_str("bottle"), py_str("bottles"), py_str("supermarket"), py_str("worth"), py_str("shopping for")]), py_tuple(vec![py_str("pick"), py_str("picking"), py_str("which"), py_str("choose"), py_str("choosing"), py_str("buying"), py_str("recommend"), py_str("looking for"), py_str("under a tenner"), py_str("good value"), py_str("worth buying")])])); __args.push(py_str("Buy the shop's own-label bottle from a named region rather than a brand at the same price: a £7 Côtes du Rhône, Rioja or Chianti from the supermarket's own range usually beats a £7 brand, because you are paying for the wine and not the label. On a red, look for the words reserva, crianza or a named village; for a white, Picpoul, Muscadet and an own-label Albariño are the safest cheap buys, and a screw cap keeps it fresher. Skip the half-price 'was £12' bottles - the plain £6 to £9 rows are where the value is. For fizz, the word Valdobbiadene on a prosecco, or a Crémant instead of champagne, gets you the same evening for a third of the price.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("wine"), py_str("wines"), py_str("red"), py_str("white"), py_str("rose"), py_str("prosecco"), py_str("fizz"), py_str("champagne"), py_str("bottle"), py_str("bottles"), py_str("merlot"), py_str("rioja"), py_str("shiraz"), py_str("pinot"), py_str("chardonnay")])), ("avoid".to_string(), py_tuple(vec![py_str("goes with"), py_str("go with"), py_str("pair"), py_str("pairing"), py_str("with lamb"), py_str("with steak"), py_str("with fish"), py_str("with chicken"), py_str("sauce"), py_str("stew"), py_str("deglaze"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("open bottle"), py_str("opened"), py_str("keeps"), py_str("keep"), py_str("freeze"), py_str("leftover"), py_str("how long"), py_str("gift"), py_str("present"), py_str("birthday"), py_str("christmas"), py_str("units"), py_str("calorie"), py_str("calories"), py_str("pregnan"), py_str("breastfeed"), py_str("allergy"), py_str("sulphite"), py_str("stain"), py_str("carpet"), py_str("spill"), py_str("hangover"), py_str("how many"), py_str("cabbage"), py_str("meat"), py_str("beef"), py_str("onion"), py_str("onions"), py_str("pepper"), py_str("peppers"), py_str("lentil"), py_str("lentils"), py_str("bean"), py_str("beans"), py_str("rice"), py_str("curry"), py_str("good for you"), py_str("healthy"), py_str("healthier"), py_str("vitamin"), py_str("vitamins"), py_str("iron"), py_str("protein"), py_str("sugar"), py_str("salt"), py_str("paint"), py_str("paintwork"), py_str("primer"), py_str("wall"), py_str("walls"), py_str("ceiling"), py_str("shirt"), py_str("shirts"), py_str("dress"), py_str("dresses"), py_str("curtain"), py_str("curtains"), py_str("sofa"), py_str("sofas"), py_str("shoes"), py_str("trainers"), py_str("socks"), py_str("jumper"), py_str("bedding"), py_str("sheet"), py_str("sheets"), py_str("ink"), py_str("printer")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("applecare")); __args.push(py_tuple(vec![py_tuple(vec![py_str("apple care"), py_str("applecare"), py_str("apple care plus"), py_str("applecare plus"), py_str("protection plan")]), py_tuple(vec![py_str("laptop"), py_str("macbook"), py_str("mac"), py_str("iphone"), py_str("ipad"), py_str("phone"), py_str("device"), py_str("worth"), py_str("buy"), py_str("get"), py_str("cancel"), py_str("need"), py_str("should i")])])); __args.push(py_str("Worth it mainly for accidental damage on a Mac or an iPhone: a screen repair is around £100 to £300 without it, and the plan covers two incidents a year with an excess of roughly £25 to £79 depending on the device. Skip it if your home contents insurance or the card you paid with already covers the device; the free one-year warranty covers faults, never drops.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("warranty-claim")); __args.push(py_tuple(vec![py_tuple(vec![py_str("warranty"), py_str("guarantee"), py_str("extended warranty"), py_str("out of warranty"), py_str("still covered")]), py_tuple(vec![py_str("claim"), py_str("broke"), py_str("broken"), py_str("faulty"), py_str("fix"), py_str("fixed"), py_str("repair"), py_str("replace"), py_str("replacement"), py_str("expired"), py_str("ran out"), py_str("run out"), py_str("cover"), py_str("covered"), py_str("receipt"), py_str("shop"), py_str("store"), py_str("currys"), py_str("argos"), py_str("john lewis"), py_str("back"), py_str("money")])])); __args.push(py_str("Claim from the shop you bought it from, not the warranty: in the UK the retailer owes you a repair or replacement for up to six years in England, Wales and Northern Ireland (five in Scotland), and a manufacturer's guarantee sits on top of that rather than replacing it. Put it in writing with the order number, the fault and the date, and ask for a repair, a replacement or a refund in that order. Out of warranty, ask anyway: a goodwill repair is common, and the card you paid with may carry its own cover.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("boiler"), py_str("washing machine"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("dishwasher"), py_str("tumble"), py_str("car"), py_str("van"), py_str("tyres"), py_str("service"), py_str("mot"), py_str("parts"), py_str("engine")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("laptop-pick")); __args.push(py_tuple(vec![py_str("laptop"), py_tuple(vec![py_str("pick"), py_str("buy"), py_str("choose"), py_str("recommend"), py_str("good"), py_str("need"), py_str("new"), py_str("one"), py_str("help"), py_str("look for"), py_str("what to look for"), py_str("specs"), py_str("spec"), py_str("ram"), py_str("memory"), py_str("storage"), py_str("upgrade"), py_str("upgrading"), py_str("upgrad"), py_str("which"), py_str("what"), py_str("get")])])); __args.push(g__laptop_general()); __kws.extend(vec![("ask".to_string(), g_CHOOSING()), ("avoid".to_string(), py_tuple(vec![py_str("battery"), py_str("cracked"), py_str("screen"), py_str("keyboard"), py_str("keys"), py_str("charger"), py_str("charging"), py_str("slow"), py_str("fan"), py_str("loud"), py_str("hot"), py_str("overheat"), py_str("crash"), py_str("freeze"), py_str("virus"), py_str("wont"), py_str("won't"), py_str("broken"), py_str("repair"), py_str("fix"), py_str("pdf"), py_str("form"), py_str("password"), py_str("reset"), py_str("install"), py_str("security"), py_str("airport"), py_str("tray"), py_str("did you"), py_str("you like"), py_str("your favourite"), py_str("your favorite")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("laptop-upgrade")); __args.push(py_tuple(vec![py_str("laptop"), py_tuple(vec![py_str("upgrade"), py_str("upgrading"), py_str("upgrad"), py_str("more ram"), py_str("more memory")])])); __args.push(py_str("Yes, if you keep running out of memory: going from 8GB to 16GB is the upgrade that shows up most, and it is £30 to £60 if the laptop has a slot you can open. Check the model takes a stick first, because many thin ones are soldered, and a bigger SSD helps more than RAM if the drive is the thing that is always full.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_tuple(vec![py_str("upgrade"), py_str("upgrading"), py_str("upgrad"), py_str("more ram"), py_str("more memory")])])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("which laptop"), py_str("buy a laptop"), py_str("new laptop"), py_str("replace my laptop"), py_str("ssd"), py_str("ssds"), py_str("hard drive"), py_str("hard disk"), py_str("disk")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("laptop-creative")); __args.push(py_tuple(vec![py_str("laptop"), py_tuple(vec![py_str("video editing"), py_str("video edit"), py_str("editing"), py_str("gaming"), py_str("games"), py_str("photoshop"), py_str("architecture"), py_str("cad"), py_str("design"), py_str("3d"), py_str("render"), py_str("blender")])])); __args.push(py_str("For video editing, gaming or CAD you do need a dedicated graphics card: look for 16GB of RAM (32GB for 4K video) and a screen that covers 100% sRGB. Expect to pay £900 to £1,300, and check the weight before you commit to carrying it.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("laptop-student")); __args.push(py_tuple(vec![py_str("laptop"), py_tuple(vec![py_str("student"), py_str("uni"), py_str("university"), py_str("school"), py_str("cheap"), py_str("budget"), py_str("essays"), py_str("coursework")])])); __args.push(py_str("For essays, video calls and Netflix, almost anything current works: 8GB RAM, a 256GB SSD and a 14 inch screen, around £400 to £550. A Chromebook is fine if everything lives in Google Docs, but avoid it for a course that needs Windows software.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("phone-refurbished")); __args.push(py_tuple(vec![py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("android"), py_str("handset"), py_str("mobile")]), py_tuple(vec![py_str("refurbished"), py_str("used"), py_str("second hand"), py_str("cheap"), py_str("new")])])); __args.push(py_str("A refurbished phone is a good way to save 30 to 50%: buy from a seller who grades the condition and gives a 12-month warranty, and check the battery health figure. Avoid anything sold as \"for parts\" or with an iCloud lock.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("bought"), py_str("worth"), py_str("cheap"), py_str("cheaper"), py_str("refurbished"), py_str("refurb"), py_str("used"), py_str("second hand"), py_str("secondhand"), py_str("recommend"), py_str("should i get"), py_str("pay"), py_str("price"), py_str("need a"), py_str("need some"), py_str("want a"), py_str("budget"), py_str("afford"), py_str("spend"), py_str("looking for")])), ("avoid".to_string(), py_tuple(vec![py_str("fix"), py_str("fixing"), py_str("repair"), py_str("repairing"), py_str("broken"), py_str("cracked"), py_str("wont charge"), py_str("not charging"), py_str("battery drains")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("phone-pick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("android"), py_str("handset")]), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("choose"), py_str("choosing"), py_str("look for"), py_str("look out for"), py_str("best"), py_str("worth"), py_str("recommend"), py_str("which"), py_str("upgrade"), py_str("buying one")])])); __args.push(py_str("Spend on the battery and the update years, not the camera: a mid-range phone at £250 to £400 lasts as long as a flagship if the maker promises four or five years of security updates (Samsung, Google and Apple do; many cheap brands promise two). Get at least 128GB, check the battery is 5,000mAh or more, and make sure it takes a physical SIM as well as an eSIM if you travel.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("battery"), py_str("batteries"), py_str("charge"), py_str("charging"), py_str("screen"), py_str("cracked"), py_str("crack"), py_str("lost"), py_str("lost it"), py_str("water"), py_str("wet"), py_str("hot"), py_str("slow"), py_str("storage"), py_str("full"), py_str("transfer"), py_str("wont turn on"), py_str("turn on"), py_str("screenshot"), py_str("print"), py_str("repair"), py_str("fix"), py_str("broken"), py_str("move"), py_str("wifi"), py_str("everything")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("running-shoes")); __args.push(py_tuple(vec![py_tuple(vec![py_str("running shoes"), py_str("running trainers"), py_str("trainers"), py_str("shoes"), py_str("runners")]), py_tuple(vec![py_str("expensive"), py_str("worth"), py_str("running"), py_str("run"), py_str("jog"), py_str("jogging"), py_str("marathon"), py_str("parkrun"), py_str("5k"), py_str("10k"), py_str("pay"), py_str("price")])])); __args.push(py_str("For running, yes: a proper pair makes a real difference to your knees, and a running shop will watch you run and fit you for £90 to £130. Replace them every 500 to 800km, or when the midsole feels flat.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("personal trainer"), py_str("dog trainer"), py_str("a trainer"), py_str("gym trainer"), py_str("trainer at"), py_str("school"), py_str("schools"), py_str("nursery"), py_str("uniform"), py_str("plimsolls"), py_str("dry"), py_str("drying"), py_str("dried"), py_str("soaked"), py_str("soaking"), py_str("newspaper"), py_str("radiator"), py_str("washing machine"), py_str("smelly"), py_str("smell"), py_str("clean them"), py_str("clean my")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("shoes-buy-cheap")); __args.push(py_tuple(vec![py_tuple(vec![py_str("school shoes"), py_str("school shoe"), py_str("school trainers"), py_str("school footwear"), py_str("shoes for school"), py_str("school uniform shoes"), py_str("uniform shoes"), py_str("kids shoes"), py_str("kids trainers"), py_str("kids school shoes"), py_str("childrens shoes"), py_str("children's shoes"), py_str("nursery shoes"), py_str("reception shoes"), py_str("plimsolls")]), py_tuple(vec![py_str("cheap"), py_str("cheapest"), py_str("afford"), py_str("where"), py_str("shop"), py_str("shops"), py_str("buy cheap"), py_str("buying"), py_str("buy"), py_str("price"), py_str("prices"), py_str("cost"), py_str("spend"), py_str("pairs"), py_str("pair of"), py_str("pair"), py_str("year old"), py_str("old"), py_str("age"), py_str("teenager"), py_str("teen"), py_str("boys"), py_str("girls"), py_str("september"), py_str("start school"), py_str("starting school"), py_str("back to school"), py_str("which"), py_str("what school"), py_str("what kind of school"), py_str("which school")])])); __args.push(py_str("Cheap is fine if the fit and the sole are right: supermarket own-brands are £10 to £25, and an outlet or the January sales gets a good pair at £25 to £35 rather than £50. Check for a thumb's width at the toe, room across the widest part and a sole you can bend - a school-age child will need the next size in four to six months.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("too small"), py_str("too tight"), py_str("fit"), py_str("fits"), py_str("fitting"), py_str("size"), py_str("sizes"), py_str("measure"), py_str("blister"), py_str("blisters"), py_str("rub"), py_str("rubs"), py_str("rubbing"), py_str("hurt"), py_str("hurts"), py_str("hurting"), py_str("sore"), py_str("uncomfortable"), py_str("smelly"), py_str("smell"), py_str("clean"), py_str("polish"), py_str("scuff"), py_str("repair"), py_str("split"), py_str("return"), py_str("refund"), py_str("lost"), py_str("wet"), py_str("muddy"), py_str("new"), py_str("running"), py_str("run"), py_str("jog"), py_str("jogging"), py_str("marathon"), py_str("parkrun"), py_str("5k"), py_str("gym"), py_str("wedding"), py_str("work"), py_str("safety"), py_str("steel toe"), py_str("heels"), py_str("personal trainer"), py_str("personal trainers"), py_str("dog trainer"), py_str("dog trainers"), py_str("gym trainer"), py_str("trainer at"), py_str("a trainer"), py_str("pt session")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("shoes-standing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("shoes"), py_str("trainers"), py_str("boots")]), py_tuple(vec![py_str("standing"), py_str("walking"), py_str("comfortable"), py_str("all day"), py_str("work"), py_str("feet")])])); __args.push(py_str("For standing all day, prioritise a cushioned sole, room at the toes and a snug heel, and buy at the end of the day when your feet are at their biggest. Two pairs worn alternately last far longer than one.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("squeaky"), py_str("creak"), py_str("creaking"), py_str("noise"), py_str("noisy"), py_str("mould"), py_str("moulds"), py_str("mold"), py_str("molds"), py_str("mouldy"), py_str("moldy"), py_str("mildew"), py_str("mildewy"), py_str("musty"), py_str("mustiness"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("damp"), py_str("spores")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coat-worth")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coat"), py_str("jacket"), py_str("raincoat"), py_str("parka"), py_str("puffer"), py_str("anorak"), py_str("duffle"), py_str("overcoat"), py_str("shell")]), py_tuple(vec![py_str("worth"), py_str("expensive"), py_str("buy"), py_str("buying"), py_str("waterproof"), py_str("warm"), py_str("warmest"), py_str("good"), py_str("look for"), py_str("look out for"), py_str("choose"), py_str("choosing"), py_str("pick"), py_str("pick out"), py_str("last"), py_str("lasts"), py_str("which"), py_str("what"), py_str("best"), py_str("material"), py_str("fabric"), py_str("winter"), py_str("cold"), py_str("wind"), py_str("rain"), py_str("spend"), py_str("cheap"), py_str("budget"), py_str("price"), py_str("under"), py_str("quid"), py_str("recommend"), py_str("need"), py_str("get")])])); __args.push(g__coat_to_buy()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("zip"), py_str("zipper"), py_str("broken"), py_str("broke"), py_str("torn"), py_str("tear"), py_str("tore"), py_str("hole"), py_str("holes"), py_str("ripped"), py_str("rip"), py_str("seam"), py_str("seams"), py_str("stuck"), py_str("lost"), py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("wet inside"), py_str("damp"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("toddler"), py_str("baby"), py_str("babies"), py_str("pram"), py_str("pushchair"), py_str("buggy"), py_str("car seat"), py_str("boy"), py_str("girl"), py_str("year old"), py_str("tonight"), py_str("today"), py_str("tomorrow"), py_str("right now"), py_str("this morning"), py_str("this afternoon"), py_str("this evening"), py_str("weather"), py_str("forecast"), py_str("cold out"), py_str("chilly out"), py_str("warm out"), py_str("raining"), py_str("rain today"), py_str("umbrella"), py_str("stain"), py_str("stains"), py_str("stained"), py_str("mark"), py_str("marks"), py_str("dirty"), py_str("dirt"), py_str("mud"), py_str("grease"), py_str("wash"), py_str("washing"), py_str("clean"), py_str("cleaned"), py_str("cleaner"), py_str("sponge"), py_str("brush"), py_str("store"), py_str("stores"), py_str("stored"), py_str("storing"), py_str("storage"), py_str("put away"), py_str("pack away"), py_str("packed away"), py_str("put it away"), py_str("away for the summer"), py_str("until next winter"), py_str("till next winter"), py_str("moths"), py_str("moth"), py_str("cedar"), py_str("vacuum bag"), py_str("garment bag"), py_str("dry clean"), py_str("dry-clean"), py_str("dry cleaning"), py_str("dry-cleaned")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bike-second-hand")); __args.push(py_tuple(vec![py_str("bike"), py_tuple(vec![py_str("second hand"), py_str("used"), py_str("check"), py_str("buy"), py_str("before"), py_str("worth"), py_str("choose"), py_str("choosing"), py_str("look for"), py_str("look out for")])])); __args.push(py_str("Check the frame for cracks and dents around the welds, spin both wheels to see if they wobble, and test the brakes and gears on a ride. Ask for the frame number and check it against a stolen-bike register before you pay; a service costs £50 to £95, so leave room in the budget.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("gears"), py_str("gear"), py_str("slip"), py_str("slips"), py_str("slipping"), py_str("chain"), py_str("brakes"), py_str("brake"), py_str("puncture"), py_str("punctures"), py_str("tyre"), py_str("tyres"), py_str("wheel"), py_str("wheels"), py_str("rusty"), py_str("rust"), py_str("squeak"), py_str("squeaks"), py_str("squeaking"), py_str("click"), py_str("clicking"), py_str("noisy")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bike-kids")); __args.push(py_tuple(vec![py_str("bike"), py_tuple(vec![py_str("child"), py_str("kid"), py_str("kids"), py_str("age"), py_str("size"), py_str("grow")])])); __args.push(py_str("Go by inside leg, not age: a child should be able to put both feet flat on the ground with the saddle at its lowest. A bike one size up with the seat down is safer than an oversized one they can't control, and balance bikes beat stabilisers.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("learn-to-ride")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ride"), py_str("riding"), py_str("pedal"), py_str("pedalling"), py_str("cycle"), py_str("cycling")]), py_tuple(vec![py_str("teach"), py_str("teaching"), py_str("learn"), py_str("learning"), py_str("child"), py_str("kid"), py_str("kids"), py_str("son"), py_str("daughter"), py_str("toddler"), py_str("stabilisers"), py_str("balance bike"), py_str("cant"), py_str("can't"), py_str("cannot"), py_str("never learnt"), py_str("never learned")])])); __args.push(py_str("Take the pedals off and drop the saddle until both feet reach the ground, then let them push along and coast with their feet up - a child who can balance for five seconds can ride, and an adult learning does the same on a bike with the saddle low. Hold the back of the saddle, not the handlebars, when they first pedal, and let go without saying so. Twenty minutes at a time beats an afternoon, and a flat, empty car park beats a narrow pavement.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("verruca"), py_str("verrucas"), py_str("wart"), py_str("warts"), py_str("foot"), py_str("feet"), py_str("itch"), py_str("itchy"), py_str("sore"), py_str("hurts"), py_str("pain"), py_str("blister"), py_str("blisters"), py_str("toe"), py_str("toes"), py_str("nail"), py_str("nails"), py_str("skin"), py_str("rash")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("office-chair")); __args.push(py_tuple(vec![py_tuple(vec![py_str("office chair"), py_str("desk chair"), py_str("chair")]), py_tuple(vec![py_str("cost"), py_str("much"), py_str("price"), py_str("spend"), py_str("decent"), py_str("budget"), py_str("pay"), py_str("worth"), py_str("good"), py_str("buy"), py_str("choose"), py_str("look for"), py_str("look out for"), py_str("recommend"), py_str("recommended"), py_str("recommendation"), py_str("recommendations")])])); __args.push(py_str("About £100 to £200 buys a decent one: adjustable height and armrests, a seat you can set to the right depth, and a five-year guarantee on the gas strut. Cheaper chairs lose their height and wobble within a year; a used Steelcase or Herman Miller for the same money is the better buy if you can sit in it first.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("high chair"), py_str("highchair"), py_str("high chairs"), py_str("baby"), py_str("babies"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("booster"), py_str("nursery")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("frying-pan")); __args.push(py_tuple(vec![py_tuple(vec![py_str("frying pan"), py_str("pan"), py_str("wok")]), py_tuple(vec![py_str("buy"), py_str("cost"), py_str("much"), py_str("price"), py_str("best"), py_str("good"), py_str("decent"), py_str("non-stick"), py_str("nonstick"), py_str("replace"), py_str("choose"), py_str("look for"), py_str("look out for")])])); __args.push(py_str("£25 to £40 buys a good non-stick one: a thick base, a metal handle you can finish in the oven, and no rivets inside where food catches. Cheap non-stick dies in a year or two, so either buy mid-price and expect to replace it, or spend a bit more on carbon steel and season it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("burnt"), py_str("burned"), py_str("burnt on"), py_str("burnt bits"), py_str("burned bits"), py_str("clean a pan"), py_str("clean the pan"), py_str("clean my pan"), py_str("clean a frying pan"), py_str("clean the frying pan"), py_str("scour"), py_str("scrub"), py_str("burnt food"), py_str("stuck to the bottom"), py_str("stuck on the bottom"), py_str("bottom of the pan")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mattress-buy")); __args.push(py_tuple(vec![py_str("mattress"), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("choose"), py_str("choosing"), py_str("firm"), py_str("soft"), py_str("best"), py_str("good"), py_str("replace"), py_str("cost"), py_str("much"), py_str("price"), py_str("decent"), py_str("spend"), py_str("which"), py_str("need"), py_str("look for"), py_str("look out for"), py_str("pick"), py_str("picking"), py_str("back"), py_str("get"), py_str("getting"), py_str("should"), py_str("cheap"), py_str("cheaper"), py_str("budget")])])); __args.push(py_str("Spend your budget on support, not springs: a medium-firm pocket-sprung or foam mattress suits most people, and you should lie on it for 15 minutes in the shop. Buy somewhere with a 100-night trial, and replace it every 8 to 10 years.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("wash"), py_str("washing"), py_str("vacuum"), py_str("wet"), py_str("accident"), py_str("bedbugs"), py_str("bed bugs"), py_str("turn"), py_str("turning"), py_str("turned"), py_str("flip"), py_str("flipping"), py_str("flipped"), py_str("rotate"), py_str("rotating"), py_str("rotation"), py_str("head to foot"), py_str("two sided"), py_str("sag"), py_str("sagging"), py_str("saggy"), py_str("dip"), py_str("dips"), py_str("lumpy"), py_str("worn out"), py_str("carried"), py_str("carrying"), py_str("lifted"), py_str("lifting"), py_str("hauled"), py_str("dragged"), py_str("shifted"), py_str("sack truck"), py_str("up the stairs"), py_str("up two flights"), py_str("topper"), py_str("toppers"), py_str("mattress topper"), py_str("mattress toppers"), py_str("protector"), py_str("mattress protector")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("washing-machine")); __args.push(py_tuple(vec![py_str("washing machine"), py_tuple(vec![py_str("buy"), py_str("choose"), py_str("choosing"), py_str("best"), py_str("good"), py_str("need"), py_str("size"), py_str("look for"), py_str("look out for"), py_str("recommend"), py_str("which washing machine"), py_str("what washing machine")])])); __args.push(py_str("Get an 8kg drum for a family, a 1,400 spin speed and the best energy rating you can afford: the running cost over ten years beats the purchase price difference. Measure the gap and the door before you order, and check the warranty length.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("leaked"), py_str("drain"), py_str("drains"), py_str("draining"), py_str("drained"), py_str("spin"), py_str("spins"), py_str("spinning"), py_str("broken"), py_str("broke"), py_str("fix"), py_str("fixed"), py_str("fixing"), py_str("repair"), py_str("repairs"), py_str("not working"), py_str("stopped working"), py_str("wont start"), py_str("won't start"), py_str("noise"), py_str("noisy"), py_str("banging"), py_str("stuck"), py_str("error"), py_str("smell"), py_str("smells")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("furniture-second-hand")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sofa"), py_str("furniture"), py_str("table"), py_str("wardrobe")]), py_tuple(vec![py_str("second hand"), py_str("used"), py_str("buy"), py_str("check"), py_str("charity")])])); __args.push(py_str("Second-hand furniture is the best value there is, and charity shops and Freecycle are the places to look. Check for woodworm holes, sagging cushions and smoke smell, and measure your door and stairs before you collect anything big.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("washing"), py_str("laundry"), py_str("detergent"), py_str("powder"), py_str("washing powder"), py_str("soap"), py_str("bleach"), py_str("cleaner"), py_str("cleaners"), py_str("nappies"), py_str("toilet roll"), py_str("loo roll"), py_str("kitchen roll"), py_str("coffee"), py_str("tea bags"), py_str("cereal"), py_str("tins"), py_str("pet food"), py_str("food"), py_str("drink"), py_str("sauce")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("is-the-deal-real")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sale"), py_str("deal"), py_str("deals"), py_str("discount"), py_str("offer"), py_str("offers"), py_str("bargain"), py_str("price drop"), py_str("price cuts"), py_str("reduced")]), py_tuple(vec![py_str("real"), py_str("really"), py_str("worth"), py_str("genuine"), py_str("good"), py_str("actually"), py_str("check"), py_str("too good")])])); __args.push(py_str("Check the price history before believing a sale: CamelCamelCamel does it for Amazon and Keepa for many shops, and a \"was £200\" tag that was £200 for one week tells you nothing. Compare the unit price (per kg or per wash) rather than the pack price.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("house"), py_str("houses"), py_str("flat"), py_str("property"), py_str("asking price"), py_str("estate agent"), py_str("below asking"), py_str("under asking"), py_str("mortgage"), py_str("seller"), py_str("landlord")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("reviews-fake")); __args.push(py_tuple(vec![py_tuple(vec![py_str("review"), py_str("reviews"), py_str("rating"), py_str("ratings"), py_str("stars")]), py_tuple(vec![py_str("fake"), py_str("fooled"), py_str("fool"), py_str("trust"), py_str("reliable"), py_str("believe"), py_str("real"), py_str("genuine"), py_str("sponsored"), py_str("paid"), py_str("honest")])])); __args.push(py_str("Read the two- and three-star reviews first: they say what actually goes wrong, while five-star ones are often written the day the thing arrived. Then check whether the reviews are spread over months or all landed in one week (a burst is bought), look for the line that says the purchase was verified, and search the product name with \"problem\" or \"broke\" to find the complaints. A brand-new product with hundreds of glowing reviews is the one to distrust most.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("review-leave")); __args.push(py_tuple(vec![py_tuple(vec![py_str("leave a review"), py_str("leave reviews"), py_str("leaving a review"), py_str("write a review"), py_str("writing a review"), py_str("post a review"), py_str("leave a rating"), py_str("leave feedback"), py_str("leave a bad review"), py_str("write a bad review"), py_str("leave a negative review"), py_str("1 star review"), py_str("one star review"), py_str("edit a review"), py_str("change a review"), py_str("review i left"), py_str("review a product"), py_str("review a restaurant"), py_str("review a shop"), py_str("review a hotel"), py_str("review a company"), py_str("where to leave a review"), py_str("good review"), py_str("good reviews"), py_str("helpful review"), py_str("honest review"), py_str("useful review"), py_str("what makes a good review")]), py_tuple(vec![py_str("how"), py_str("should"), py_str("where"), py_str("can"), py_str("worth"), py_str("want"), py_str("going to"), py_str("do i"), py_str("what makes"), py_str("what counts as"), py_str("google"), py_str("amazon"), py_str("tripadvisor"), py_str("trustpilot"), py_str("yelp"), py_str("product"), py_str("restaurant"), py_str("shop"), py_str("hotel"), py_str("star"), py_str("online"), py_str("bad")])])); __args.push(py_str("Leave it where people look, and where the business will see it: the order page for an online shop (that is the one the seller can answer), Google Maps for a cafe, shop or restaurant, TripAdvisor for a hotel. Three or four lines beat a rant - what you bought, when, what went wrong, and what you want - and give the stars you actually mean, since a 2 with a fair paragraph is read and a 1 with none is not. Post within a week or so while the details are checkable, keep the order number and any photos, and you can edit or delete your own review later from your account.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("reply"), py_str("replies"), py_str("reply to"), py_str("respond"), py_str("responding"), py_str("answer a"), py_str("apologise"), py_str("apologize"), py_str("removed"), py_str("remove"), py_str("taken down"), py_str("delete"), py_str("my app store"), py_str("subscription"), py_str("subscriptions"), py_str("chargeback"), py_str("appraisal"), py_str("performance review"), py_str("my boss"), py_str("my manager"), py_str("at work"), py_str("colleague"), py_str("colleagues"), py_str("coworker"), py_str("co-worker"), py_str("teammate"), py_str("team"), py_str("staff"), py_str("employee"), py_str("employees"), py_str("the team"), py_str("my team"), py_str("presentation"), py_str("customer"), py_str("customers"), py_str("my business"), py_str("my shop"), py_str("my restaurant")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("review-taken-down")); __args.push(py_tuple(vec![py_tuple(vec![py_str("review removed"), py_str("review taken down"), py_str("remove a review"), py_str("delete a review"), py_str("get a review removed"), py_str("get a bad review removed"), py_str("bad review removed"), py_str("bad review taken down"), py_str("bad review of my business"), py_str("review about my business"), py_str("negative review"), py_str("unfair review"), py_str("can i report a review"), py_str("report a review"), py_str("review of my shop"), py_str("review of my business"), py_str("review of my restaurant"), py_str("review of my company"), py_str("review of my cafe"), py_str("review of our shop"), py_str("a customer left"), py_str("review on my"), py_str("review for my"), py_str("one star review"), py_str("1 star review")]), py_tuple(vec![py_str("how"), py_str("can"), py_str("someone"), py_str("business"), py_str("shop"), py_str("restaurant"), py_str("customer"), py_str("customers"), py_str("left"), py_str("got"), py_str("google"), py_str("tripadvisor"), py_str("amazon"), py_str("report"), py_str("remove"), py_str("removed"), py_str("taken"), py_str("take down"), py_str("down"), py_str("delete"), py_str("deleted"), py_str("untrue"), py_str("lies"), py_str("defamatory"), py_str("slander"), py_str("my shop"), py_str("my business")])])); __args.push(py_str("Only the platform can take a review down, and only when it breaks their rules: no proof of purchase, abuse, the wrong business, or a rant about something else. Open the review, use its report button and pick the rule it breaks, and keep a screenshot. If the platform says no, ask once more with the order number as proof of what really happened, and reply once in public with the fix - readers weigh that calm reply more than the complaint. Never offer a refund or a freebie for a good review or a deletion: that breaks the rules itself and can cost the listing its stars.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("how do i reply"), py_str("reply to a bad review"), py_str("respond to a review"), py_str("apologise to a customer"), py_str("appraisal"), py_str("performance review"), py_str("my boss"), py_str("my manager")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("where-to-buy-cheapest")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cheapest"), py_str("cheapest place"), py_str("cheapest shop"), py_str("where to buy"), py_str("where can i buy"), py_str("where should i buy"), py_str("where do i buy"), py_str("best place to buy"), py_str("best price"), py_str("who sells"), py_str("where sells"), py_str("where do i get"), py_str("where can i get"), py_str("where to get"), py_str("where should i get")]), op_add(&py_tuple(vec![py_str("get one")]), &crate::knowledge::lookup::g_PRICE_COMPARED())])); __args.push(py_str("Compare the exact model across two or three shops, and check its price history first: a \"was £500\" tag is often a week old. For appliances the cheapest safe route is usually refurbished or graded stock with a real warranty, or ex-display if you ask. Buy at the end of the season or when a new model lands, and pay by credit card over £100 for the extra protection.")); __kws.extend(vec![("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); { let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::asks_where(args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::asks_where_a_shop_answers(args))(&__args) } } } })), ("avoid".to_string(), py_tuple(vec![py_str("groceries"), py_str("grocery"), py_str("weekly shop"), py_str("food shop"), py_str("supermarket"), py_str("milk"), py_str("meat"), py_str("vegetables"), py_str("dinner"), py_str("meal"), py_str("meals"), py_str("recipe"), py_str("recipes"), py_str("broken"), py_str("not working"), py_str("stopped working"), py_str("wont start"), py_str("won't start"), py_str("repair"), py_str("repairs"), py_str("fix"), py_str("faulty"), py_str("leaking"), py_str("leaks"), py_str("error code"), py_str("sofa"), py_str("furniture"), py_str("table"), py_str("wardrobe"), py_str("laptop"), py_str("phone"), py_str("iphone"), py_str("android"), py_str("tablet"), py_str("mattress"), py_str("washing machine"), py_str("washer dryer"), py_str("running shoes"), py_str("trainers"), py_str("coat"), py_str("jacket"), py_str("office chair"), py_str("frying pan"), py_str("bike"), py_str("used car"), py_str("secondhand car"), py_str("second hand car"), py_str("new car"), py_str("cheap car"), py_str("car for sale"), py_str("printer"), py_str("pram"), py_str("prams"), py_str("contact lenses"), py_str("contact lens"), py_str("glasses frames"), py_str("which"), py_str("to run"), py_str("running cost"), py_str("cost to run"), py_str("per year"), py_str("a year"), py_str("energy"), py_str("electricity"), py_str("water use"), py_str("uses"), py_str("efficient"), py_str("descaler"), py_str("descale"), py_str("descaling"), py_str("limescale"), py_str("plant"), py_str("plants"), py_str("flower"), py_str("flowers"), py_str("tree"), py_str("trees"), py_str("seed"), py_str("seeds"), py_str("bulbs"), py_str("shrub"), py_str("bush"), py_str("houseplant"), py_str("hanging basket"), py_str("compost"), py_str("chicken"), py_str("chickens"), py_str("fish"), py_str("eggs"), py_str("fruit"), py_str("veg"), py_str("vegetables")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("where-to-buy-everyday")); __args.push(py_tuple(vec![py_tuple(vec![py_str("where can i buy"), py_str("where do i buy"), py_str("where to buy"), py_str("where can i get"), py_str("where do i get"), py_str("where can i find"), py_str("where do i find"), py_str("best place to buy"), py_str("best place to get"), py_str("who sells"), py_str("where sells")]), g__THE_EVERYDAY_THINGS()])); __args.push(g__everyday_where()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("boiler"), py_str("boilers"), py_str("consumer unit"), py_str("fuse box"), py_str("fuse board"), py_str("trip switch"), py_str("rcd"), py_str("colour code"), py_str("color code"), py_str("paint code"), py_str("code for the paint")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("haggle-price")); __args.push(py_tuple(vec![py_tuple(vec![py_str("haggle"), py_str("haggling"), py_str("haggled"), py_str("bargain"), py_str("bargaining"), py_str("bargained"), py_str("negotiate"), py_str("negotiating"), py_str("negotiation"), py_str("talk them down"), py_str("money off"), py_str("discount"), py_str("discounts"), py_str("lower price"), py_str("better price"), py_str("cheaper price"), py_str("best price"), py_str("price down")]), py_tuple(vec![py_str("how"), py_str("is"), py_str("it"), py_str("can i"), py_str("should i"), py_str("do i"), py_str("is it"), py_str("you"), py_str("my"), py_str("what"), py_str("tips"), py_str("rude"), py_str("worth"), py_str("price"), py_str("prices"), py_str("shop"), py_str("shops"), py_str("market"), py_str("car boot"), py_str("boot sale"), py_str("sofa"), py_str("mattress"), py_str("builder"), py_str("builders"), py_str("plumber"), py_str("tradesman"), py_str("tradesmen"), py_str("quote"), py_str("quotes"), py_str("deal"), py_str("cheaper"), py_str("down"), py_str("washing machine"), py_str("fridge"), py_str("car")])])); __args.push(g__haggle()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("used car"), py_str("second hand car"), py_str("dealership"), py_str("test drive"), py_str("mileage"), py_str("car dealer"), py_str("real bargain"), py_str("actually a bargain"), py_str("good bargain"), py_str("is it worth"), py_str("worth it"), py_str("voucher"), py_str("vouchers"), py_str("promo code"), py_str("discount code"), py_str("coupon"), py_str("coupons"), py_str("student discount"), py_str("loyalty"), py_str("clubcard"), py_str("nectar"), py_str("cashback"), py_str("what is a bargain"), py_str("whats a bargain"), py_str("is this a bargain"), py_str("is that a bargain"), py_str("a real bargain"), py_str("such a bargain"), py_str("bargain hunter"), py_str("council tax"), py_str("energy"), py_str("bill"), py_str("bills"), py_str("gas"), py_str("electric"), py_str("electricity"), py_str("water bill"), py_str("train"), py_str("trains"), py_str("rail"), py_str("railcard"), py_str("bus"), py_str("buses"), py_str("flight"), py_str("flights"), py_str("house"), py_str("houses"), py_str("mortgage"), py_str("offer on a house"), py_str("estate agent"), py_str("sell"), py_str("sells"), py_str("selling"), py_str("sold"), py_str("ebay"), py_str("vinted"), py_str("gumtree"), py_str("insurance"), py_str("salary"), py_str("salaries"), py_str("pay rise"), py_str("payrise"), py_str("raise"), py_str("wages"), py_str("wage"), py_str("job offer"), py_str("new job"), py_str("promotion"), py_str("appraisal"), py_str("interview"), py_str("flexible"), py_str("flexible working"), py_str("flexible hours"), py_str("hours"), py_str("boss"), py_str("manager"), py_str("employer"), py_str("employers"), py_str("employee"), py_str("work from home"), py_str("wfh"), py_str("part time"), py_str("full time"), py_str("shift"), py_str("shifts"), py_str("rota"), py_str("annual leave"), py_str("holiday entitlement"), py_str("redundancy"), py_str("union"), py_str("colleague"), py_str("colleagues"), py_str("hr"), py_str("rent"), py_str("rents"), py_str("rent increase"), py_str("rent rise"), py_str("landlord"), py_str("landlords"), py_str("tenancy"), py_str("tenancies"), py_str("lease"), py_str("deposit"), py_str("section 13"), py_str("mobile"), py_str("phone"), py_str("phones"), py_str("handset"), py_str("sim"), py_str("sim only"), py_str("tariff"), py_str("contract"), py_str("contracts"), py_str("broadband"), py_str("tv licence"), py_str("gym"), py_str("gym membership"), py_str("insurance quote"), py_str("new car"), py_str("new cars"), py_str("brand new car"), py_str("buy a car"), py_str("buying a car"), py_str("car finance")])), ("guard".to_string(), py_func("a_price_not_a_loan", |args: &[Py]| crate::knowledge::lookup::a_price_not_a_loan(args)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("supermarket-cheapest")); __args.push(py_tuple(vec![py_tuple(vec![py_str("aldi"), py_str("lidl"), py_str("asda"), py_str("tesco"), py_str("sainsburys"), py_str("sainsbury's"), py_str("morrisons"), py_str("waitrose"), py_str("marks and spencer"), py_str("marks and spencers"), py_str("m and s"), py_str("m s"), py_str("ms"), py_str("marks & spencer"), py_str("co-op"), py_str("co op"), py_str("coop"), py_str("iceland"), py_str("farmfoods"), py_str("ocado"), py_str("supermarket"), py_str("supermarkets"), py_str("groceries"), py_str("weekly shop")]), py_tuple(vec![py_str("cheaper"), py_str("cheapest"), py_str("expensive"), py_str("compare"), py_str("comparison"), py_str("best value"), py_str("better value"), py_str("which is"), py_str("worth switching"), py_str("switch"), py_str("price"), py_str("prices"), py_str("value for money"), py_str("or"), py_str("shop at"), py_str("shopping at"), py_str("worth it"), py_str("worth shopping"), py_str("as good"), py_str("good quality"), py_str("quality"), py_str("which shop"), py_str("which one"), py_str("good"), py_str("better"), py_str("worse"), py_str("fresh"), py_str("fresher"), py_str("taste"), py_str("tasty"), py_str("nicer"), py_str("worth"), py_str("worth the money"), py_str("decide"), py_str("deciding"), py_str("choose"), py_str("choosing"), py_str("pick"), py_str("between"), py_str("should i"), py_str("shall i"), py_str("should we"), py_str("shop at"), py_str("shopping at")])])); __args.push(g__supermarket_cheapest()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("cheaper"), py_str("cheapest"), py_str("expensive"), py_str("compare"), py_str("comparison"), py_str("best value"), py_str("better value"), py_str("better"), py_str("worse"), py_str("as good"), py_str("fresh"), py_str("fresher"), py_str("which"), py_str("which is"), py_str("which shop"), py_str("which one"), py_str("worth"), py_str("worth it"), py_str("switch"), py_str("prices"), py_str("price"), py_str("value"), py_str("quality"), py_str("good"), py_str("shop at"), py_str("shopping at"), py_str("or"), py_str("vs"), py_str("versus"), py_str("instead of"), py_str("should i"), py_str("shall i"), py_str("should we"), py_str("should"), py_str("decide"), py_str("deciding"), py_str("choose"), py_str("choosing"), py_str("pick"), py_str("help me")])), ("guard".to_string(), g__the_place_not_the_shop()), ("avoid".to_string(), op_add(&py_tuple(vec![py_str("meal plan"), py_str("meal planning"), py_str("meal prep"), py_str("what should i cook"), py_str("recipes"), py_str("food waste"), py_str("waste food"), py_str("refund"), py_str("refunds"), py_str("complaint"), py_str("complain"), py_str("delivery"), py_str("deliveroo"), py_str("just eat"), py_str("substitute"), py_str("out of stock"), py_str("parking"), py_str("opening hours"), py_str("open today"), py_str("nearest"), py_str("clubcard"), py_str("club card"), py_str("loyalty"), py_str("loyalty card"), py_str("nectar"), py_str("reward points"), py_str("petrol"), py_str("fuel"), py_str("diesel"), py_str("gluten"), py_str("free from"), py_str("free-from"), py_str("vegan"), py_str("vegetarian"), py_str("coeliac"), py_str("celiac"), py_str("dairy free"), py_str("nut free"), py_str("allergy"), py_str("organic"), py_str("halal"), py_str("kosher"), py_str("any good for"), py_str("good for"), py_str("fine for"), py_str("ok for"), py_str("okay for"), py_str("meal deal"), py_str("my weekly shop"), py_str("would a weekly shop"), py_str("is a weekly shop cheaper"), py_str("cheaper to do a weekly shop"), py_str("my shopping"), py_str("i do my shopping"), py_str("too expensive"), py_str("my basket"), py_str("shop at the"), py_str("shopping at the"), py_str("i always shop"), py_str("i only shop")]), &g__SERVICE_WORDS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bulk-buying")); __args.push(py_tuple(vec![py_tuple(vec![py_str("supermarket"), py_str("shopping"), py_str("groceries")]), py_tuple(vec![py_str("cheaper"), py_str("save"), py_str("save money"), py_str("bulk"), py_str("value"), py_str("budget")])])); __args.push(py_str("Compare the price per 100g on the shelf label, not the pack size: bigger packs are often, but not always, cheaper. Own-brand staples (rice, pasta, tinned tomatoes, oats) are usually the same product, and the savings there pay for the things you care about.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-dad-fishing")); __args.push(py_tuple(vec![py_tuple(vec![py_str("fishing"), py_str("fish"), py_str("tackle"), py_str("angling")]), py_tuple(vec![py_str("dad"), py_str("father"), py_str("grandad"), py_str("stepdad"), py_str("gift"), py_str("present")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{they} {has} told you what {they} {likes}, so buy into it: a decent tackle box, or a float and hook kit if {they} {is} starting out, fits any budget and gets used. Add one small thing {they} wouldn't buy {self} - a box set of the old films {they} {loves}, a decent flask - and a card that says why. One thing {they} will actually use beats three safe ones.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::about_them(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&g_LOSS(), &g_CRISIS())), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_GIFT_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-dad")); __args.push(py_tuple(vec![py_tuple(vec![py_str("dad"), py_str("father"), py_str("grandad"), py_str("stepdad")]), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("60th"), py_str("70th"), py_str("what to get"), py_str("what to buy"), py_str("what should i get"), py_str("what should i buy"), py_str("what can i get"), py_str("what can i buy"), py_str("what do i get"), py_str("what do i buy"), py_str("gift ideas"), py_str("present ideas"), py_str("christmas ideas"), py_str("birthday ideas"), py_str("ideas for a gift"), py_str("ideas for a present"), py_str("christmas present"), py_str("christmas gift"), py_str("fathers day"), py_str("father's day"), py_str("get him"), py_str("get her"), py_str("buy him"), py_str("buy her"), py_str("what to give"), py_str("what should i give"), py_str("what can i give")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Go consumable and a notch better than he'd buy himself: good coffee, a decent bottle of what he already drinks, or a voucher for something he already does at the weekend. If he insists he wants nothing, replace one small thing he has been putting off - a wallet, a torch, a decent belt - and add a card that says why. One thoughtful thing beats three safe ones.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _present_answer(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("speech"), py_str("toast"), py_str("best man"), py_str("maid of honour"), py_str("few words"), py_str("card"), py_str("cards"), py_str("message"), py_str("messages"), py_str("write"), py_str("writing"), py_str("what do i say"), py_str("what to write"), py_str("what do i write")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-mum-garden")); __args.push(py_tuple(vec![py_tuple(vec![py_str("garden"), py_str("gardening"), py_str("plants"), py_str("allotment"), py_str("garden centre")]), py_tuple(vec![py_str("mum"), py_str("mother"), py_str("nan"), py_str("grandma"), py_str("gift"), py_str("present")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("{they} {has} told you what {they} {likes}, so buy into it: good gloves that fit properly, a decent trowel, a bag of good compost, or a voucher for the nearest garden centre. Add one small thing {they} would not buy {self} and a card that says why.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::about_them(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&g_LOSS(), &g_CRISIS())), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_GIFT_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-mum")); __args.push(py_tuple(vec![py_tuple(vec![py_str("mum"), py_str("mother"), py_str("nan"), py_str("grandma")]), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("mother's day"), py_str("mothers day"), py_str("what to get"), py_str("what to buy"), py_str("what should i get"), py_str("what should i buy"), py_str("what can i get"), py_str("what can i buy"), py_str("what do i get"), py_str("what do i buy"), py_str("gift ideas"), py_str("present ideas"), py_str("christmas ideas"), py_str("birthday ideas"), py_str("ideas for a gift"), py_str("ideas for a present"), py_str("christmas present"), py_str("christmas gift"), py_str("get her"), py_str("get him"), py_str("buy her"), py_str("buy him"), py_str("what to give"), py_str("what should i give"), py_str("what can i give")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Make something she uses every week a bit nicer: a proper thermos, a voucher for the place she likes, a photo book of the family, or a little of something she loves to eat. Experience gifts work well for someone who already has everything.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _present_answer(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&g_LOSS(), &g_CRISIS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-someone")); __args.push(py_tuple(vec![py_str("present for someone"), py_str("present for somebody"), py_str("gift for someone"), py_str("gift for somebody"), py_str("presents for someone"), py_str("gifts for someone"), py_str("present for anyone"), py_str("gift for anyone"), py_str("get someone something"), py_str("get somebody something")])); __args.push(g__gift_anyone()); __kws.extend(vec![("avoid".to_string(), op_add(&g_LOSS(), &g_CRISIS()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-leaving-work")); __args.push(py_tuple(vec![py_tuple(vec![py_str("retiring"), py_str("retires"), py_str("retired"), py_str("retirement"), py_str("leaving present"), py_str("leaving do"), py_str("leaving work"), py_str("leaves work"), py_str("last day at work")]), py_tuple(vec![py_str("present"), py_str("presents"), py_str("gift"), py_str("gifts"), py_str("gift ideas"), py_str("present ideas"), py_str("what to get"), py_str("what to give"), py_str("what to buy"), py_str("what do i get"), py_str("what do i give"), py_str("what do i buy"), py_str("what should i get"), py_str("what should i give"), py_str("what should i buy"), py_str("what can i get"), py_str("what can i give"), py_str("what can i buy"), py_str("what is a good present"), py_str("what is a good gift"), py_str("get them"), py_str("get her"), py_str("get him"), py_str("buy them"), py_str("give them")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("A leaving present is about what comes next rather than the job: something for the thing they will now have time for - a book, a course, a voucher for the garden centre or the cinema, a decent bottle for the first Friday. If the team is chipping in, £10 to £15 each buys one good thing, which beats five fillers, and the card everyone signs with one real memory each is the part people keep. Ask first whether they want a fuss at all: some people would rather have a pint after work.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _present_answer(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("mum"), py_str("mother"), py_str("dad"), py_str("father"), py_str("nan"), py_str("grandma"), py_str("grandad"), py_str("granny"), py_str("sister"), py_str("brother"), py_str("aunt"), py_str("uncle"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("son"), py_str("daughter"), py_str("baby"), py_str("child"), py_str("speech"), py_str("toast"), py_str("few words"), py_str("card"), py_str("cards"), py_str("message"), py_str("what do i say"), py_str("what to write"), py_str("what do i write")]))), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-hospital")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hospital"), py_str("ward"), py_str("in hospital"), py_str("hospital bed"), py_str("hospice"), py_str("in a hospice"), py_str("hospice bed"), py_str("having an operation"), py_str("operation"), py_str("surgery"), py_str("recovering"), py_str("in recovery")]), py_tuple(vec![py_str("gift"), py_str("present"), py_str("presents"), py_str("idea"), py_str("ideas"), py_str("what to get"), py_str("what should i get"), py_str("what can i get"), py_str("what do i get"), py_str("what to buy"), py_str("what should i buy"), py_str("what can i buy"), py_str("what do i buy"), py_str("what to bring"), py_str("what should i bring"), py_str("what can i bring"), py_str("what do i bring"), py_str("what to take"), py_str("what should i take"), py_str("what can i take"), py_str("what do i take"), py_str("what do you buy"), py_str("what would you buy"), py_str("what do you get"), py_str("what would you get"), py_str("what do you bring"), py_str("what would you bring"), py_str("what do you take"), py_str("what would you take"), py_str("what should i get them"), py_str("what can i get them"), py_str("flowers"), py_str("ok to bring"), py_str("is it ok to bring"), py_str("get her"), py_str("get him"), py_str("get them"), py_str("get my mum"), py_str("get my dad"), py_str("get my nan"), py_str("buy her"), py_str("buy him"), py_str("buy them"), py_str("bring her"), py_str("bring him"), py_str("bring them"), py_str("take her"), py_str("take him"), py_str("take them"), py_str("visit"), py_str("visit her"), py_str("visit him"), py_str("see her"), py_str("see him"), py_str("cheer"), py_str("cheer her"), py_str("cheer him")])])); __args.push(py_str("Small things they can use lying in bed: a long phone cable and a charged power bank, earphones, a puzzle book or a magazine, a decent lip balm, a light blanket, or a printed photo of the family to put on the locker. Skip flowers on most wards and skip food unless you know the diet. Ask the ward what is allowed before you bring it in, and go when you can stay a while - the visit is the gift.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("visiting hours"), py_str("visiting times"), py_str("visiting time"), py_str("when can i visit"), py_str("what time can i visit"), py_str("when can we visit"), py_str("when can you visit"), py_str("what time can you visit"), py_str("can i visit"), py_str("can we visit"), py_str("is it ok to visit"), py_str("ok to visit"), py_str("should i visit"), py_str("let visitors in"), py_str("visitors allowed"), py_str("visitors in")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-someone-ill")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ill"), py_str("sick"), py_str("poorly"), py_str("unwell"), py_str("flu"), py_str("a cold"), py_str("chest infection"), py_str("virus"), py_str("lurgy"), py_str("man flu"), py_str("recovering"), py_str("getting over"), py_str("in bed"), py_str("off work ill"), py_str("was in hospital"), py_str("out of hospital"), py_str("had an operation"), py_str("after an operation")]), op_add(&g_PERSON_ASK(), &py_tuple(vec![py_str("cheer her up"), py_str("cheer him up"), py_str("cheer them up"), py_str("cheer up"), py_str("take round"), py_str("bring round"), py_str("something to take round"), py_str("what can i bring"), py_str("what to bring"), py_str("what do i bring"), py_str("what can i take round"), py_str("anything i can"), py_str("anything we can")]))])); __args.push(py_str("Something that makes a day in bed easier: a flask of soup with bread, a box of their favourite tea, a magazine or puzzle book, and a link to a film you know they like. Offer one specific thing at one specific time - \"I'll leave soup on the step on Thursday\" - because \"let me know if you need anything\" is the thing a poorly person never does. Skip flowers and skip food you do not know they can face.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&py_tuple(vec![py_str("do about"), py_str("anything i can do"), py_str("what can i do"), py_str("dose"), py_str("doses"), py_str("medicine"), py_str("medication"), py_str("paracetamol"), py_str("ibuprofen"), py_str("calpol"), py_str("syrup"), py_str("painkiller"), py_str("painkillers"), py_str("how much"), py_str("how many"), py_str("fever"), py_str("temperature"), py_str("rash"), py_str("vomiting"), py_str("diarrhoea"), py_str("cough"), py_str("coughs"), py_str("sore throat"), py_str("is it normal"), py_str("see a doctor"), py_str("how long"), py_str("hospital"), py_str("ward"), py_str("a and e"), py_str("999"), py_str("111"), py_str("symptom"), py_str("symptoms"), py_str("for a fever"), py_str("for a cough"), py_str("for the cough"), py_str("for a temperature"), py_str("for the pain"), py_str("for a cold"), py_str("for flu"), py_str("for the flu"), py_str("for a sore throat"), py_str("for a headache"), py_str("for sickness"), py_str("my son"), py_str("my daughter"), py_str("my child"), py_str("my baby"), py_str("my kid"), py_str("my kids"), py_str("my toddler"), py_str("my boy"), py_str("my girl"), py_str("has the flu"), py_str("had the flu"), py_str("has flu"), py_str("is in bed ill")]), &g_LOSS()), &g_CRISIS())), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-person")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sister"), py_str("brother"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("cousin"), py_str("aunt"), py_str("uncle"), py_str("colleague"), py_str("coworker"), py_str("boss"), py_str("manager"), py_str("neighbour"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("son"), py_str("daughter"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("niece"), py_str("nephew"), py_str("grandson"), py_str("granddaughter"), py_str("grandchild"), py_str("grandchildren"), py_str("godson"), py_str("goddaughter"), py_str("godchild"), py_str("stepson"), py_str("stepdaughter"), py_str("neighbor"), py_str("teacher"), py_str("carer"), py_str("nurse"), py_str("stepmum"), py_str("stepdad"), py_str("them")]), py_tuple(vec![py_str("gift"), py_str("present"), py_str("get"), py_str("got"), py_str("buy"), py_str("birthday"), py_str("christmas"), py_str("30th"), py_str("40th"), py_str("50th"), py_str("idea"), py_str("ideas")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Pick something they would use every week and would not buy themselves: good coffee and a proper mug, woolly socks, a decent notebook, or a voucher for the thing they always say they will do. If you know what they are into, buy into it - a good knife for someone who cooks, a head torch for a runner, secateurs for a gardener - and put a real message in the card, which does half the work.")); __args.push(g_CHILD_PRESENT()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_person_row(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), op_add(&g_LOSS(), &g_CRISIS())), ("bias".to_string(), Py::Int(2)), ("ask".to_string(), g_PERSON_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-child-age")); __args.push(py_tuple(vec![op_add(&py_tuple(vec![py_str("year old"), py_str("years old"), py_str("year-old"), py_str("yr old"), py_str("yrs old"), py_str("month old"), py_str("months old"), py_str("aged"), py_str("toddler"), py_str("preschool"), py_str("school age"), py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("teens"), py_str("teenage")]), &g_CHILD_PEOPLE()), py_tuple(vec![py_str("gift"), py_str("present"), py_str("get"), py_str("got"), py_str("buy"), py_str("birthday"), py_str("christmas"), py_str("1st"), py_str("2nd"), py_str("3rd"), py_str("4th"), py_str("5th"), py_str("6th"), py_str("7th"), py_str("8th"), py_str("9th"), py_str("10th"), py_str("11th"), py_str("12th"), py_str("idea"), py_str("ideas")])])); __args.push(g__the_age_row()); __kws.extend(vec![("shape".to_string(), py_tuple(vec![g__names_a_childs_age()])), ("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("cheap"), py_str("cheaper"), py_str("cheapest"), py_str("inexpensive"), py_str("tenner"), py_str("fiver"), py_str("stocking"), py_str("small"), py_str("money"), py_str("cash"), py_str("voucher")]))), ("guard".to_string(), g__this_row_answers_the_line()), ("bias".to_string(), Py::Int(3)), ("ask".to_string(), g_AGE_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-new-baby")); __args.push(py_tuple(vec![py_tuple(vec![py_str("new baby"), py_str("newborn"), py_str("new born"), py_str("baby shower"), py_str("new arrival"), py_str("baby that has"), py_str("friend had a baby"), py_str("given birth"), py_str("just had a baby"), py_str("had a baby"), py_str("had her baby"), py_str("had the baby"), py_str("mate had a baby"), py_str("sister had a baby"), py_str("brother had a baby"), py_str("wife had a baby"), py_str("wifes had a baby"), py_str("wife has had a baby"), py_str("partner had a baby"), py_str("daughter had a baby"), py_str("son had a baby"), py_str("colleague had a baby"), py_str("coworker had a baby"), py_str("neighbour had a baby"), py_str("neighbor had a baby"), py_str("she had a baby"), py_str("they had a baby"), py_str("someone had a baby")]), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("what should i get"), py_str("what do i get"), py_str("what can i get"), py_str("what to get"), py_str("what do you get"), py_str("take to"), py_str("take round"), py_str("bring"), py_str("what should i send"), py_str("what to send"), py_str("what do i send"), py_str("what do i bring"), py_str("what should i bring"), py_str("what to bring"), py_str("what should i take"), py_str("what to take"), py_str("what do i take"), py_str("what can i take"), py_str("get her something"), py_str("get him something"), py_str("get them something"), py_str("something for the baby"), py_str("something for her"), py_str("something for him"), py_str("idea"), py_str("ideas")])])); __args.push(py_str("Buy the boring things that get used up: clothes in the three to six month size, a pack of muslins, a board book, or a voucher for a meal they do not have to cook. Skip newborn sizes (everyone buys them and the baby outgrows them in a fortnight), skip toys (a baby cannot hold one) and skip anything big, since they have no room. Offer to hold the baby while they sleep: that is the thing they actually want.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("health visitor"), py_str("fever"), py_str("temperature"), py_str("feeding"), py_str("feed"), py_str("sleep"), py_str("jaundice"), py_str("rash"), py_str("crying"), py_str("will not stop"), py_str("wont stop"), py_str("weaning"), py_str("milk"), py_str("nappy"), py_str("nappies"), py_str("cord"), py_str("weight"), py_str("is it normal"), py_str("how do i"), py_str("what if"), py_str("shower food"), py_str("food ideas"), py_str("food for a shower"), py_str("food for the shower")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-for-a-pet")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cat"), py_str("cats"), py_str("kitten"), py_str("kittens"), py_str("dog"), py_str("dogs"), py_str("puppy"), py_str("puppies"), py_str("pup"), py_str("rabbit"), py_str("rabbits"), py_str("bunny"), py_str("hamster"), py_str("gerbil"), py_str("guinea pig"), py_str("guinea pigs"), py_str("budgie"), py_str("parrot"), py_str("horse"), py_str("pony"), py_str("tortoise"), py_str("pet"), py_str("pets")]), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("gift idea"), py_str("gift ideas"), py_str("present idea"), py_str("present ideas"), py_str("christmas present"), py_str("christmas gift"), py_str("birthday present"), py_str("birthday gift"), py_str("stocking"), py_str("christmas ideas"), py_str("presents for"), py_str("for christmas"), py_str("for his christmas"), py_str("for her christmas"), py_str("for its christmas"), py_str("for his birthday"), py_str("for her birthday"), py_str("for its birthday"), py_str("for their birthday"), py_str("what should i get my"), py_str("what can i get my"), py_str("what do i get my"), py_str("what should i buy my"), py_str("what can i buy my"), py_str("what do i buy my"), py_str("what should i get for my"), py_str("what to get my"), py_str("what to buy my")])])); __args.push(py_str("For a pet, buy the comfort they use every day rather than another toy: a cat wants a warm bed by a radiator, a scratch post tall enough to stretch up on, or a food-puzzle ball; a dog wants a long-lasting chew it only gets on the day, a snuffle mat, or a spare lead for the car. A rabbit or a guinea pig wants a bigger hidey house and a chew stick, and a box with a blanket in it is famously the best present of all. Keep chocolate, grapes, raisins and anything with xylitol away from a dog, and skip the novelty toys they will ignore by Boxing Day.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("to eat"), py_str("eat"), py_str("eats"), py_str("eating"), py_str("feed"), py_str("feeds"), py_str("feeding"), py_str("food"), py_str("diet"), py_str("drink"), py_str("drinks"), py_str("water"), py_str("litter"), py_str("poo"), py_str("wees"), py_str("wee"), py_str("sick"), py_str("ill"), py_str("poorly"), py_str("vet"), py_str("vets"), py_str("limp"), py_str("limping"), py_str("pain"), py_str("hurt"), py_str("wont eat"), py_str("will not eat"), py_str("not eating"), py_str("off her food"), py_str("off his food"), py_str("diarrhoea"), py_str("vomiting"), py_str("scratching"), py_str("chewing"), py_str("barking"), py_str("biting"), py_str("training"), py_str("train"), py_str("walk"), py_str("walks"), py_str("walking"), py_str("to stop"), py_str("stop it"), py_str("behaviour"), py_str("behaving"), py_str("shampoo"), py_str("coat"), py_str("flea"), py_str("fleas"), py_str("worm"), py_str("worming"), py_str("neutered"), py_str("spay"), py_str("should i buy"), py_str("worth it"), py_str("is it worth"), py_str("need a"), py_str("harness"), py_str("lead"), py_str("collar"), py_str("muzzle"), py_str("crate"), py_str("carrier"), py_str("litter tray"), py_str("to sleep in"), py_str("to sleep on"), py_str("lover"), py_str("lovers"), py_str("someone who has"), py_str("somebody who has"), py_str("who has a"), py_str("who has an"), py_str("who owns"), py_str("owner"), py_str("owners"), py_str("anyone who"), py_str("person who"), py_str("bringing"), py_str("brings"), py_str("bring in"), py_str("brings in"), py_str("brought in"), py_str("brought me"), py_str("dead"), py_str("mice"), py_str("mouse"), py_str("bird"), py_str("birds"), py_str("vole"), py_str("voles"), py_str("rat"), py_str("rats"), py_str("hunting"), py_str("hunted"), py_str("killed"), py_str("caught"), py_str("cat flap")]))), ("bias".to_string(), Py::Int(3))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-wedding")); __args.push(py_tuple(vec![py_tuple(vec![py_str("wedding"), py_str("registry"), py_str("marrying"), py_str("married")]), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("registry"), py_str("money"), py_str("buy"), py_str("buying"), py_str("spend"), py_str("spending"), py_str("idea"), py_str("ideas"), py_str("what should i get"), py_str("what do i get"), py_str("what can i get"), py_str("what to get"), py_str("what shall i get"), py_str("get them"), py_str("get her"), py_str("get him"), py_str("give them"), py_str("give her"), py_str("give him"), py_str("bring")])])); __args.push(py_str("About £50 is the usual gift from a friend, £100 or more from a close friend or when the two of you go as a couple; what you can afford beats the table you are given. Check the registry next, since most couples list what they actually need, and a contribution to the honeymoon beats a guess. If there is no list, money in a card with a real note is never wrong, and one small thing for their home lands better than something big and impersonal. Say yes to the registry rather than worry about being original.")); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("getting married"), py_str("getting wed"), py_str("get married"), py_str("we are engaged"), py_str("engaged"), py_str("engagement"), py_str("planning"), py_str("venue"), py_str("guest list"), py_str("tax"), py_str("taxed"), py_str("taxable"), py_str("tax free"), py_str("declare"), py_str("hmrc"), py_str("our wedding"), py_str("my wedding"), py_str("save the date"), py_str("thank you card"), py_str("thank you letter"), py_str("thank you note"), py_str("thank you message"), py_str("thank you wording"), py_str("music"), py_str("song"), py_str("songs"), py_str("playlist"), py_str("band"), py_str("dj"), py_str("first dance"), py_str("aisle"), py_str("food"), py_str("menu"), py_str("menus"), py_str("catering"), py_str("caterer"), py_str("buffet"), py_str("meal"), py_str("eat"), py_str("feed"), py_str("canapes"), py_str("hog roast"), py_str("supper"), py_str("drinks"), py_str("food ideas"), py_str("flowers"), py_str("florist"), py_str("bouquet"), py_str("buttonhole"), py_str("photographer"), py_str("photographs"), py_str("photos"), py_str("readings"), py_str("vows"), py_str("order of the day"), py_str("what time")]))), ("bias".to_string(), Py::Int(3)), ("ask".to_string(), g_GIFT_ASK())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-anniversary")); __args.push(py_tuple(vec![py_str("anniversary"), py_tuple(vec![py_str("gift"), py_str("present"), py_str("ideas"), py_str("idea"), py_str("what to get"), py_str("what should i get"), py_str("what should we do")])])); __args.push(py_str("Go for something shared rather than an object: dinner somewhere she has mentioned, a night away, or a small thing tied to a memory you both have. Write a card that says why, and it will beat anything expensive and impersonal.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-her")); __args.push(py_tuple(vec![py_tuple(vec![py_str("her"), py_str("him"), py_str("his"), py_str("their"), py_str("them"), py_str("they")]), py_tuple(vec![py_str("get"), py_str("got"), py_str("buy"), py_str("bought"), py_str("give"), py_str("gives"), py_str("giving"), py_str("gave"), py_str("present for"), py_str("presents for"), py_str("gift for"), py_str("gifts for")])])); __args.push(g__gift_anyone()); __kws.extend(vec![("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("bought him"), py_str("bought her"), py_str("bought them"), py_str("gave him"), py_str("gave her"), py_str("gave them"), py_str("i bought"), py_str("i gave"), py_str("already bought"), py_str("already got"), py_str("have bought")]))), ("ask".to_string(), py_tuple(vec![py_str("what"), py_str("which"), py_str("should"), py_str("could"), py_str("would"), py_str("any"), py_str("idea"), py_str("ideas"), py_str("suggest"), py_str("recommend"), py_str("stuck"), py_str("christmas"), py_str("birthday"), py_str("anniversary"), py_str("present"), py_str("gift"), py_str("still"), py_str("yet"), py_str("havent"), py_str("anything"), py_str("nothing")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-generic")); __args.push(py_tuple(vec![py_tuple(vec![py_str("present"), py_str("gift")]), py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("stuck"), py_str("no idea"), py_str("what should"), py_str("suggest"), py_str("for my"), py_str("for a"), py_str("for his"), py_str("for her"), py_str("christmas"), py_str("good"), py_str("best"), py_str("nice"), py_str("any"), py_str("decent")])])); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("Pick something they'd use every week and wouldn't buy themselves: good coffee and a proper mug, woolly socks, nice soap, a book about something they care about. If you know what they do at the weekend, buy into that instead - a voucher for the pool, the cinema or a class. Something consumable and slightly better than usual is the safest bet for someone you don't know well.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_row(args))(&__args) }); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("say thank"), py_str("say thanks"), py_str("what do i say"), py_str("what should i say"), py_str("what to say"), py_str("thank you for"), py_str("thank them"), py_str("thanks for the"), py_str("giving me"), py_str("gave me"), py_str("got me"), py_str("i dont like"), py_str("i don't like"), py_str("dont like it"), py_str("don't like it"), py_str("hate it"), py_str("returns"), py_str("return it"), py_str("regift")])), ("guard".to_string(), g__nobody_named_by_pronoun())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-money")); __args.push(py_tuple(vec![py_tuple(vec![py_str("voucher"), py_str("vouchers"), py_str("gift card"), py_str("gift cards"), py_str("gift voucher"), py_str("gift vouchers"), py_str("cash"), py_str("money"), py_str("banknote"), py_str("banknotes"), py_str("tenner in a card"), py_str("a tenner in a")]), py_tuple(vec![py_str("cop out"), py_str("lazy"), py_str("rude"), py_str("cheeky"), py_str("impersonal"), py_str("tacky"), py_str("boring"), py_str("unoriginal"), py_str("lame"), py_str("bad present"), py_str("bad gift"), py_str("good present"), py_str("good gift"), py_str("nice present"), py_str("acceptable present"), py_str("ok present"), py_str("okay present"), py_str("ok gift"), py_str("okay gift"), py_str("fine present"), py_str("instead of a gift"), py_str("instead of a present"), py_str("give money"), py_str("giving money"), py_str("give cash"), py_str("giving cash"), py_str("give a voucher"), py_str("give a gift card"), py_str("gift"), py_str("present"), py_str("presents"), py_str("sensible"), py_str("safe bet")])])); __args.push(py_str("A voucher is not a cop out - a vague one is. Make it specific: a card for somewhere they already go (the cinema, a bookshop, the garden centre, their favourite cafe), or for a thing rather than a shop, and write on the card what you would do with it. Cash is fine too, and nobody is insulted by it: 'I did not know which one you wanted, so put it towards it' is the whole note. What reads as lazy is the generic card for a shop they never go in, handed over with nothing written on it - and a small thing plus a voucher beats both.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("expired"), py_str("expire"), py_str("expires"), py_str("expiry"), py_str("run out"), py_str("ran out"), py_str("refund"), py_str("refunded"), py_str("went bust"), py_str("has closed"), py_str("closed down"), py_str("cashback"), py_str("atm"), py_str("withdraw"), py_str("withdrawal"), py_str("borrow"), py_str("borrowed"), py_str("lend"), py_str("lending"), py_str("loan"), py_str("pay rise"), py_str("salary"), py_str("wage"), py_str("bank transfer"), py_str("wedding"), py_str("christening"), py_str("how much money should i give"), py_str("how much should i spend"), py_str("how much is usual"), py_str("tax"), py_str("voucher code"), py_str("discount code"), py_str("coupon"), py_str("balance"), py_str("left on"), py_str("how much is left"), py_str("how much left"), py_str("how much is on"), py_str("how much is there"), py_str("how much have i got"), py_str("top up"), py_str("top it up"), py_str("load it"), py_str("reload"), py_str("remaining"), py_str("still valid")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-cheap")); __args.push(py_tuple(vec![py_tuple(vec![py_str("secret santa"), py_str("santa"), py_str("stocking filler"), py_str("stocking fillers"), py_str("tenner"), py_str("under a tenner"), py_str("under 10"), py_str("under ten pounds"), py_str("under a fiver"), py_str("cheap present"), py_str("cheap gift"), py_str("small gift"), py_str("little gift"), py_str("cheap ideas"), py_str("stocking filler for kids"), py_str("gifts for kids"), py_str("gift for kids"), py_str("gifts for children"), py_str("gift for children"), py_str("presents for kids"), py_str("presents for children"), py_str("kids presents"), py_str("childrens presents"), py_str("little gift for a child")]), py_tuple(vec![py_str("idea"), py_str("ideas"), py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("get"), py_str("got"), py_str("buy"), py_str("bought"), py_str("under"), py_str("cheap"), py_str("what"), py_str("suggest"), py_str("suggestion"), py_str("suggestions"), py_str("christmas"), py_str("limit"), py_str("limits"), py_str("spend"), py_str("spending"), py_str("child"), py_str("children"), py_str("kid"), py_str("kids"), py_str("toddler"), py_str("toddlers"), py_str("boy"), py_str("girl"), py_str("son"), py_str("daughter"), py_str("niece"), py_str("nephew"), py_str("grandchildren"), py_str("grandchild"), py_str("teenager")])])); __args.push(g__gift_cheap()); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("how much"), py_str("budget"), py_str("day out"), py_str("days out"), py_str("meal"), py_str("meals"), py_str("dinner"), py_str("food"), py_str("eat"), py_str("cook"), py_str("cooking"), py_str("recipe"), py_str("recipes"), py_str("holiday"), py_str("days out with the kids"), py_str("something to do"), py_str("things to do"), py_str("cinema tickets"), py_str("friend"), py_str("friends"), py_str("mate"), py_str("mates"), py_str("colleague"), py_str("colleagues"), py_str("workmate"), py_str("mum"), py_str("mother"), py_str("nan"), py_str("grandma"), py_str("dad"), py_str("father"), py_str("grandad"), py_str("sister"), py_str("brother"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("teacher"), py_str("neighbour"), py_str("neighbor"), py_str("cousin"), py_str("auntie"), py_str("uncle")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-has-everything")); __args.push(py_tuple(vec![py_tuple(vec![py_str("has everything"), py_str("who has everything"), py_str("wants nothing"), py_str("needs nothing"), py_str("hard to buy for"), py_str("impossible to buy for"), py_str("doesnt want anything"), py_str("does not want anything"), py_str("says they want nothing"), py_str("want for nothing")]), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("get"), py_str("got"), py_str("buy"), py_str("bought"), py_str("idea"), py_str("ideas"), py_str("what"), py_str("christmas"), py_str("birthday"), py_str("suggest"), py_str("suggestions"), py_str("already")])])); __args.push(py_str("Go for time and something used up rather than another object: a meal or a night away somewhere they have mentioned, tickets to something on their list, a lesson in a thing they keep talking about, or a very good version of a daily thing - coffee, olive oil, socks - with a note saying why you picked it. Someone who wants for nothing is short of time and good consumables, not of things.")); __kws.extend(vec![("bias".to_string(), Py::Int(1)), ("avoid".to_string(), py_tuple(vec![py_str("nothing to eat"), py_str("no money"), py_str("cant afford"), py_str("food bank"), py_str("hospital"), py_str("ward"), py_str("dying"), py_str("died"), py_str("funeral"), py_str("grief"), py_str("grieving"), py_str("cancer"), py_str("care home"), py_str("job"), py_str("rent"), py_str("benefits"), py_str("mum"), py_str("mother"), py_str("nan"), py_str("grandma"), py_str("granny"), py_str("dad"), py_str("father"), py_str("grandad"), py_str("stepdad"), py_str("sister"), py_str("brother"), py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("colleague"), py_str("workmate"), py_str("teacher"), py_str("neighbour"), py_str("neighbor"), py_str("write"), py_str("writing"), py_str("written"), py_str("card message"), py_str("card say"), py_str("card says"), py_str("what to say"), py_str("the message"), py_str("message in"), py_str("words")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("used-car")); __args.push(py_tuple(vec![py_tuple(vec![py_str("used car"), py_str("second-hand car"), py_str("second hand car"), py_str("buying a car"), py_str("car for sale")]), py_tuple(vec![py_str("used"), py_str("second hand"), py_str("buy"), py_str("buying"), py_str("check"), py_str("seller"), py_str("inspector"), py_str("deposit"), py_str("mot"), py_str("mileage"), py_str("history")])])); __args.push(py_str("Ask how long they've owned it and why they're selling, then check the MOT history online for gaps and advisories before you look at the paint. On the drive, listen for knocks over bumps and make sure every gear and switch works; pay for an inspection on anything over a couple of thousand, and never send a deposit before you've seen the car and the V5 in their name.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("lease"), py_str("leasing"), py_str("leased"), py_str("pcp"), py_str("contract hire"), py_str("hire purchase"), py_str("finance"), py_str("on finance"), py_str("car finance"), py_str("car loan"), py_str("loan for a car"), py_str("monthly payment"), py_str("monthly payments"), py_str("per month"), py_str("rent or buy")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("car-history-check")); __args.push(py_tuple(vec![py_tuple(vec![py_str("car"), py_str("cars"), py_str("vehicle"), py_str("van"), py_str("motor"), py_str("hpi"), py_str("motorbike"), py_str("v5c"), py_str("v5"), py_str("log book"), py_str("history check"), py_str("vehicle check"), py_str("hpi check")]), py_tuple(vec![py_str("car history"), py_str("vehicle history"), py_str("history check"), py_str("history on"), py_str("hpi"), py_str("hpi check"), py_str("been in an accident"), py_str("in an accident"), py_str("any accidents"), py_str("written off"), py_str("write off"), py_str("write-off"), py_str("clocked"), py_str("mileage on"), py_str("the mileage"), py_str("outstanding finance"), py_str("finance on it"), py_str("finance outstanding"), py_str("not stolen"), py_str("is stolen"), py_str("been stolen"), py_str("stolen"), py_str("v5"), py_str("v5c"), py_str("log book"), py_str("vin"), py_str("previous owner"), py_str("previous keepers"), py_str("before buying"), py_str("genuine"), py_str("real"), py_str("fake"), py_str("forged"), py_str("verify"), py_str("legit"), py_str("history")])])); __args.push(py_str("Yes, two checks cover it, one free and one about fifteen pounds. Put the number plate into gov.uk's MOT history page first: every test since 2005, the mileage at each one, and the gaps and repeated advisories that a tidy advert hides. Then buy a full history check - HPI, the AA or Total Car Check - which tells you about outstanding finance, a stolen record, an insurance write-off and a mileage discrepancy. Before you pay, match the VIN on the windscreen to the V5C, check the V5C is in the seller's name at the address you are standing at, and walk away from anything that does not line up.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("ask".to_string(), py_tuple(vec![py_str("check"), py_str("checks"), py_str("checking"), py_str("checked"), py_str("history"), py_str("safe"), py_str("genuine"), py_str("real"), py_str("trust"), py_str("verify"), py_str("make sure"), py_str("before"), py_str("buy"), py_str("buying"), py_str("worth"), py_str("how do i"), py_str("is there"), py_str("does it"), py_str("has it"), py_str("can i"), py_str("should i")])), ("avoid".to_string(), py_tuple(vec![py_str("insurance"), py_str("claim"), py_str("claims"), py_str("payout"), py_str("write it off"), py_str("my car"), py_str("our car"), py_str("i crashed"), py_str("crashed"), py_str("i hit"), py_str("scratch"), py_str("scratched"), py_str("dented"), py_str("repair"), py_str("repairs"), py_str("garage"), py_str("bodywork"), py_str("injury"), py_str("injured"), py_str("hurt"), py_str("police"), py_str("crash"), py_str("bump"), py_str("no claims"), py_str("excess"), py_str("premium"), py_str("quote"), py_str("renewal"), py_str("sell my"), py_str("selling my"), py_str("part exchange"), py_str("trade in"), py_str("trade-in"), py_str("family"), py_str("medical"), py_str("health"), py_str("symptoms"), py_str("heart"), py_str("cancer"), py_str("mental"), py_str("doctor"), py_str("gp"), py_str("hospital"), py_str("history of"), py_str("long history"), py_str("history month"), py_str("the history")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("write-off-category")); __args.push(py_tuple(vec![py_tuple(vec![py_str("cat s"), py_str("cat n"), py_str("cat c"), py_str("cat d"), py_str("cat a"), py_str("cat b"), py_str("category s"), py_str("category n"), py_str("category c"), py_str("category d"), py_str("write off"), py_str("write-off"), py_str("written off"), py_str("insurance write off"), py_str("write off category"), py_str("kill the car")]), py_tuple(vec![py_str("car"), py_str("van"), py_str("vehicle"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("safe"), py_str("worth"), py_str("should i"), py_str("what"), py_str("mean"), py_str("means"), py_str("difference"), py_str("between"), py_str("explain"), py_str("any good"), py_str("avoid"), py_str("insure"), py_str("insurance"), py_str("cheaper"), py_str("value"), py_str("sell")])])); __args.push(py_str("Both mean an insurer wrote the car off instead of repairing it, and both are legal to buy and drive once the repair has been signed off. Cat N is non-structural damage - usually a stolen-and-recovered car, a bump or flood - and cat S is structural damage that was pulled straight again. (Cat A and B are breaker-only and must never go back on the road.) An S car is worth less than an N, both are worth noticeably less than a clean one, and both are harder to insure and to sell on: ask for the damage photos and the repair invoice, have an inspection done, and get an insurance quote on the registration before you commit.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("crashed"), py_str("crash"), py_str("i hit"), py_str("accident"), py_str("claim"), py_str("claims"), py_str("payout"), py_str("my insurance"), py_str("the insurance"), py_str("other driver"), py_str("police"), py_str("repair it"), py_str("repairs"), py_str("garage"), py_str("injury"), py_str("injured"), py_str("hurt")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("used-car-seller")); __args.push(py_tuple(vec![py_tuple(vec![py_str("seller")]), py_tuple(vec![py_str("ask"), py_str("question"), py_str("questions"), py_str("happy"), py_str("know"), py_str("check"), py_str("car"), py_str("hpi"), py_str("history")])])); __args.push(py_str("Ask how long they've owned it and why they're selling, whether it's ever been in an accident, what the mileage is, when the cambelt or chain was last done, and whether the service history and V5 are both in their name. Ask to see it at their home rather than a car park: it tells you how the car has been kept, and it's where a private seller's story tends to come apart.")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("first-car")); __args.push(py_tuple(vec![py_tuple(vec![py_str("first car"), py_str("new driver"), py_str("learner driver"), py_str("car for a new driver"), py_str("car for a learner"), py_str("learner car"), py_str("car for a learner driver")]), py_tuple(vec![py_str("good"), py_str("best"), py_str("buy"), py_str("buying"), py_str("recommend"), py_str("cheap"), py_str("what"), py_str("which"), py_str("how do i"), py_str("should i"), py_str("advice"), py_str("get"), py_str("start")])])); __args.push(py_str("Small, petrol, and as dull as you can bear: a hatchback like a Corsa, Fiesta, Polo or Yaris with a small engine. Check two things before the car itself - the insurance group, because a 1.0-litre in a low group can cost half what a bigger car costs to insure, and the MOT history on gov.uk for gaps and advisories. Spend under half your budget on the car and keep the rest for insurance, tax and the first repair, and avoid anything modified, since insurers charge more for it.")); __kws.extend(vec![("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("i passed my test"), py_str("passed my driving test"), py_str("taxi"), py_str("van"), py_str("motorbike"), py_str("insurance quote"), py_str("how much is insurance"), py_str("tyre"), py_str("tyres"), py_str("battery"), py_str("oil")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("used-car-inspection")); __args.push(py_tuple(vec![py_tuple(vec![py_str("inspection"), py_str("inspections"), py_str("inspecting"), py_str("checked over"), py_str("pre purchase"), py_str("pre-purchase"), py_str("aa check"), py_str("rac check"), py_str("vehicle check"), py_str("vehicle inspection"), py_str("history check"), py_str("hpi"), py_str("hpi check"), py_str("buyers report"), py_str("buyer's report")]), py_tuple(vec![py_str("worth"), py_str("pay"), py_str("paying"), py_str("car"), py_str("cars"), py_str("need"), py_str("should"), py_str("cost"), py_str("costs"), py_str("how much"), py_str("money"), py_str("expensive"), py_str("bother"), py_str("any point"), py_str("get one"), py_str("get it checked"), py_str("get checked")])])); __args.push(py_str("Yes, and about a hundred to two hundred pounds: a pre-purchase inspection from the AA, RAC or a good local garage costs that on anything over about two thousand pounds, and it catches the hidden faults a test drive can't. Book it after you've seen the car and agreed a price in principle, and walk away if the seller won't allow one.")); __kws.extend(vec![("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("used-car-price")); __args.push(py_tuple(vec![py_tuple(vec![py_str("used car"), py_str("second-hand car"), py_str("second hand car"), py_str("secondhand car")]), py_tuple(vec![py_str("how much"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("spend"), py_str("pay"), py_str("budget"), py_str("afford"), py_str("expensive"), py_str("cheap"), py_str("too much")])])); __args.push(py_str("A used car is about £2,000 to £4,000 for 10 to 15 years old and 80,000 miles or more, £5,000 to £8,000 for 5 to 8 years old, and £9,000 to £14,000 for 3 to 5. Whatever the price, check the MOT history free on gov.uk and keep £500 back for the first repair.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how much"), py_str("cost"), py_str("price"), py_str("spend"), py_str("pay"), py_str("budget"), py_str("expensive"), py_str("much")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("insurance"), py_str("insure"), py_str("tax"), py_str("fuel"), py_str("petrol"), py_str("diesel"), py_str("running"), py_str("service"), py_str("servicing"), py_str("repair"), py_str("repairs"), py_str("fix"), py_str("mot"), py_str("tyre"), py_str("tyres"), py_str("lease"), py_str("leasing"), py_str("finance"), py_str("rent"), py_str("rental"), py_str("hire"), py_str("sell"), py_str("selling"), py_str("worth"), py_str("trade in"), py_str("part exchange"), py_str("haggle"), py_str("bike"), py_str("bicycle"), py_str("van"), py_str("caravan"), py_str("motorbike"), py_str("scooter"), py_str("taxi"), py_str("train"), py_str("parking"), py_str("clamp"), py_str("first car"), py_str("learner"), py_str("good"), py_str("best"), py_str("weigh"), py_str("weight"), py_str("run"), py_str("running costs"), py_str("wash"), py_str("valet"), py_str("battery"), py_str("clutch"), py_str("breakdown"), py_str("new"), py_str("brand new"), py_str("brand-new")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("new-car-price")); __args.push(py_tuple(vec![py_tuple(vec![py_str("new car"), py_str("brand new car"), py_str("brand-new car")]), py_tuple(vec![py_str("how much"), py_str("cost"), py_str("costs"), py_str("price"), py_str("prices"), py_str("spend"), py_str("pay"), py_str("budget"), py_str("afford"), py_str("expensive"), py_str("cheap"), py_str("worth")])])); __args.push(py_str("A new small car starts around £18,000 and a new family car around £28,000, and either loses a fifth to a third of that in the first year, most of it in the first hour. A two or three year old version of the same car is about half the new price, which is why that age is where most people buy.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("how much"), py_str("cost"), py_str("price"), py_str("spend"), py_str("pay"), py_str("budget"), py_str("expensive"), py_str("worth")])), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), py_tuple(vec![py_str("used"), py_str("second hand"), py_str("secondhand"), py_str("insurance"), py_str("insure"), py_str("tax"), py_str("fuel"), py_str("petrol"), py_str("diesel"), py_str("running"), py_str("service"), py_str("servicing"), py_str("repair"), py_str("fix"), py_str("mot"), py_str("tyres"), py_str("lease"), py_str("leasing"), py_str("finance"), py_str("rent"), py_str("hire"), py_str("sell"), py_str("selling"), py_str("trade in"), py_str("part exchange"), py_str("haggle"), py_str("bike"), py_str("van"), py_str("caravan"), py_str("weigh"), py_str("weight"), py_str("wash"), py_str("valet"), py_str("breakdown"), py_str("first car"), py_str("good"), py_str("best"), py_str("electric"), py_str("ev"), py_str("hybrid")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("flowers-or-plant")); __args.push(py_tuple(vec![py_tuple(vec![py_str("flowers"), py_str("flower"), py_str("bouquet"), py_str("plant"), py_str("orchid"), py_str("houseplant")]), py_tuple(vec![py_str("or"), py_str("instead"), py_str("better"), py_str("gift"), py_str("present"), py_str("get"), py_str("buy"), py_str("thank"), py_str("sorry")])])); __args.push(py_str("Flowers, most of the time: go to a florist rather than a garage and ask for what is in season, and a mixed bunch is always safe. A plant lasts years and is the better present for someone who likes looking after things, but give an orchid or a houseplant only to a person who will remember to water it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("garden"), py_str("gardening"), py_str("grow"), py_str("growing"), py_str("seed"), py_str("seeds"), py_str("soil"), py_str("compost"), py_str("lawn"), py_str("prune"), py_str("allergic"), py_str("dying"), py_str("wilt"), py_str("cheaper"), py_str("cheapest"), py_str("price"), py_str("cost"), py_str("online"), py_str("delivery"), py_str("sale"), py_str("water"), py_str("watering"), py_str("stem"), py_str("stems"), py_str("roots"), py_str("root"), py_str("drink"), py_str("drinking"), py_str("leaves"), py_str("leaf"), py_str("yellow"), py_str("yellowing"), py_str("tree"), py_str("trees"), py_str("shrub"), py_str("shrubs"), py_str("hedge"), py_str("hedges"), py_str("bulbs"), py_str("sow"), py_str("sowing"), py_str("plant out"), py_str("planting out"), py_str("when to plant"), py_str("photosynthesis"), py_str("make food"), py_str("makes food"), py_str("their food"), py_str("own food"), py_str("food for"), py_str("energy"), py_str("sunlight"), py_str("carbon dioxide"), py_str("chlorophyll"), py_str("feed"), py_str("feeds"), py_str("feeding"), py_str("fertiliser"), py_str("fertilizer"), py_str("mould"), py_str("mold"), py_str("mouldy"), py_str("moldy"), py_str("mildew"), py_str("fungus"), py_str("fuzzy"), py_str("furry"), py_str("rot"), py_str("rotting"), py_str("repot"), py_str("repotting"), py_str("gnat"), py_str("gnats"), py_str("sorry for itself"), py_str("look sad"), py_str("looks sad"), py_str("looking sad"), py_str("unhappy"), py_str("miserable"), py_str("sulking"), py_str("drooping"), py_str("droopy"), py_str("limp"), py_str("floppy"), py_str("cactus"), py_str("cacti"), py_str("cactuses"), py_str("succulent"), py_str("succulents"), py_str("to flower"), py_str("flower again"), py_str("flowers again"), py_str("in flower"), py_str("bloom"), py_str("blooms"), py_str("blooming"), py_str("blossom"), py_str("buds"), py_str("no flowers"), py_str("not flowering"), py_str("won't flower"), py_str("wont flower"), py_str("died"), py_str("dies"), py_str("repot"), py_str("repotting")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("appliance-worth")); __args.push(py_tuple(vec![py_tuple(vec![py_str("air fryer"), py_str("air fryers"), py_str("airfryer"), py_str("slow cooker"), py_str("slow cookers"), py_str("pressure cooker"), py_str("instant pot"), py_str("bread maker"), py_str("bread machine"), py_str("dishwasher"), py_str("dishwashers"), py_str("tumble dryer"), py_str("tumble dryers"), py_str("robot vacuum"), py_str("robot hoover"), py_str("dehumidifier"), py_str("heat pump dryer"), py_str("heat pump dryers"), py_str("heat pump drier"), py_str("heat pump tumble dryer"), py_str("heat pump tumble drier"), py_str("heat pump condensing dryer"), py_str("vented dryer"), py_str("vented tumble dryer"), py_str("condenser dryer"), py_str("condenser tumble dryer"), py_str("coffee machine"), py_str("bean to cup"), py_str("stand mixer"), py_str("microwave"), py_str("microwaves"), py_str("food processor"), py_str("kitchen machine"), py_str("air purifier"), py_str("heated airer"), py_str("meal kit"), py_str("meal kits"), py_str("recipe box"), py_str("hello fresh"), py_str("gousto"), py_str("kindle"), py_str("e reader"), py_str("ereader"), py_str("noise cancelling headphones"), py_str("noise cancelling"), py_str("fitness tracker"), py_str("fitness trackers"), py_str("fitbit"), py_str("smart watch"), py_str("smart watches"), py_str("apple watch"), py_str("standing desk"), py_str("standing desks"), py_str("sit stand desk"), py_str("weighted blanket"), py_str("weighted blankets"), py_str("steam cleaner"), py_str("steam cleaners"), py_str("steam mop"), py_str("cleaner"), py_str("cleaners"), py_str("cleaning service"), py_str("housekeeper"), py_str("heated clothes airer"), py_str("heated drying rack"), py_str("electric blanket"), py_str("heated blanket"), py_str("smart thermostat"), py_str("thermostat"), py_str("water filter jug"), py_str("water filter"), py_str("filter jug"), py_str("humidifier"), py_str("mattress topper"), py_str("bed topper"), py_str("water softener"), py_str("robot mower"), py_str("robot lawn mower"), py_str("video doorbell"), py_str("ring doorbell"), py_str("fan"), py_str("fans"), py_str("tower fan"), py_str("pedestal fan"), py_str("fan heater"), py_str("electric heater"), py_str("oil filled radiator"), py_str("space heater")]), py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("worth the money"), py_str("should i"), py_str("do i need"), py_str("need"), py_str("buy"), py_str("buying"), py_str("get"), py_str("getting"), py_str("good"), py_str("any good"), py_str("value"), py_str("money"), py_str("recommend"), py_str("recommended"), py_str("recommendation"), py_str("recommendations"), py_str("any point in"), py_str("any point to"), py_str("point in"), py_str("point to"), py_str("point of"), py_str("waste of money"), py_str("wasting money"), py_str("money well spent"), py_str("bother with"), py_str("bother"), py_str("help"), py_str("help with"), py_str("helped")])])); __args.push(g__appliance_value()); __kws.extend(vec![("ask".to_string(), g_APPLIANCE_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("battery life"), py_str("battery"), py_str("charge"), py_str("charging"), py_str("pair"), py_str("pairing"), py_str("pairs with"), py_str("sync"), py_str("syncs"), py_str("syncing"), py_str("steps"), py_str("step count"), py_str("heart rate"), py_str("companion app"), py_str("notifications"), py_str("fitness tracker"), py_str("fitness watch"), py_str("my "), py_str("our "), py_str("broken"), py_str("broke"), py_str("repair"), py_str("repairing"), py_str("fix"), py_str("fixing"), py_str("leaking"), py_str("noisy"), py_str("not heating"), py_str("wont"), py_str("won't"), py_str("error"), py_str("fault"), py_str("warranty"), py_str("guarantee"), py_str("manual"), py_str("clean"), py_str("cleaning"), py_str("descale"), py_str("how do i"), py_str("how to"), py_str("recipes for"), py_str("recipe for"), py_str("recipes with"), py_str("recipe with"), py_str("recipe ideas"), py_str("recipe idea"), py_str("cooker recipe"), py_str("cooker recipes"), py_str("fryer recipe"), py_str("fryer recipes"), py_str("maker recipe"), py_str("maker recipes"), py_str("cook recipe"), py_str("cook recipes"), py_str("what to cook"), py_str("what can i cook"), py_str("meal ideas"), py_str("dinner ideas"), py_str("to cook in"), py_str("cook in a"), py_str("cook in an"), py_str("cook in it"), py_str("cook with it"), py_str("something to cook")])), ("guard".to_string(), py_func("lambda", move |args: &[Py]| {  let understood: Py = __kwp(args, 0, "understood").unwrap_or(Py::None); Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::asks_where_a_shop_answers(args))(&__args) })) })), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("choice-worth")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gym membership"), py_str("gym memberships"), py_str("join a gym"), py_str("joining a gym"), py_str("go to the gym"), py_str("go the gym"), py_str("going to the gym"), py_str("back to the gym"), py_str("bother going to the gym"), py_str("bother with the gym"), py_str("bother with a gym"), py_str("start going to the gym"), py_str("start at the gym"), py_str("heat pump"), py_str("heat pumps"), py_str("electric car"), py_str("electric cars"), py_str("season ticket"), py_str("season tickets"), py_str("smart meter"), py_str("smart meters"), py_str("water meter"), py_str("water meters")]), py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("worth the money"), py_str("should i"), py_str("shall i"), py_str("shall we"), py_str("should we"), py_str("do i need"), py_str("buy"), py_str("buying"), py_str("get"), py_str("join"), py_str("joining"), py_str("pay"), py_str("paying"), py_str("better"), py_str("off"), py_str("recommend"), py_str("recommended"), py_str("recommendation"), py_str("recommendations"), py_str("any point in"), py_str("point in"), py_str("point to"), py_str("waste of money"), py_str("money well spent")])])); __args.push(g__choice_value()); __kws.extend(vec![("ask".to_string(), g_APPLIANCE_ASK()), ("avoid".to_string(), py_tuple(vec![py_str("insurance"), py_str("warranty"), py_str("uni"), py_str("university"), py_str("degree"), py_str("mattress"), py_str("inspection"), py_str("how often"), py_str("how many times a week"), py_str("how many times a month"), py_str("how regular"), py_str("how busy"), py_str("what time"), py_str("what should i do"), py_str("what to do at"), py_str("how do i"), py_str("how to"), py_str("tax"), py_str("lease"), py_str("hire purchase"), py_str("prices"), py_str("price"), py_str("cost per"), py_str("how much"), py_str("read"), py_str("reading"), py_str("blank"), py_str("display"), py_str("sending"), py_str("signal"), py_str("stopped"), py_str("faulty"), py_str("not working"), py_str("meter fitted"), py_str("meter fitting"), py_str("fit the meter"), py_str("fit a meter"), py_str("fitting the meter"), py_str("fitting a meter"), py_str("install the meter"), py_str("install a meter"), py_str("installing the meter"), py_str("installing a meter"), py_str("installation of the meter"), py_str("installation of a meter")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smart-meter-what")); __args.push(py_tuple(vec![py_tuple(vec![py_str("smart meter"), py_str("smart meters"), py_str("in home display"), py_str("in-home display")]), py_tuple(vec![py_str("what is"), py_str("whats"), py_str("what are"), py_str("what does"), py_str("what do"), py_str("explain"), py_str("definition"), py_str("mean"), py_str("means"), py_str("work out")])])); __args.push(py_str("A smart meter is the gas and electricity meter that sends your readings to your supplier itself, so nobody has to read it and the bill is based on what you used rather than an estimate. It comes with a small display for the kitchen that shows what you are spending as you use it. It is free from your supplier and it does not change your tariff.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("should i"), py_str("any good"), py_str("good buy"), py_str("read"), py_str("reading"), py_str("blank"), py_str("stopped"), py_str("faulty"), py_str("broken"), py_str("not working"), py_str("display is"), py_str("fitted"), py_str("fitting"), py_str("install"), py_str("installed"), py_str("installation"), py_str("appointment")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smart-meter-fitting")); __args.push(py_tuple(vec![py_tuple(vec![py_str("smart meter"), py_str("smart meters"), py_str("meter"), py_str("meters")]), py_tuple(vec![py_str("fitting"), py_str("fit one"), py_str("fit mine"), py_str("fit my"), py_str("fit a"), py_str("fit the"), py_str("fitted"), py_str("install"), py_str("installed"), py_str("installation"), py_str("appointment"), py_str("coming round"), py_str("coming to"), py_str("engineer"), py_str("next week"), py_str("next month"), py_str("on the day"), py_str("switch over"), py_str("swapping"), py_str("change my meter"), py_str("upgrade")])])); __args.push(py_str("Clear the cupboard the meters are in and take a photo of the readings first, because the old ones go with the fitter. Someone over 18 has to be in, and it takes about an hour per fuel; the gas and electricity go off for a few minutes each. The fitter should leave the in-home display set up and working, and the old meters away - if a display is not showing, ring the supplier rather than living with it.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("smart meter"), py_str("smart meters"), py_str("meter"), py_str("meters")])), ("avoid".to_string(), py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("should i"), py_str("broken"), py_str("faulty"), py_str("not working"), py_str("blank"), py_str("leaking"), py_str("water meter"), py_str("parking"), py_str("electricity bill"), py_str("read my")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("smart-meter-how")); __args.push(py_tuple(vec![py_tuple(vec![py_str("smart meter"), py_str("smart meters"), py_str("in home display"), py_str("in-home display")]), py_tuple(vec![py_str("read"), py_str("reading"), py_str("readings"), py_str("button"), py_str("blank"), py_str("stopped"), py_str("faulty"), py_str("broken"), py_str("working"), py_str("display"), py_str("sending"), py_str("signal"), py_str("connect"), py_str("wrong"), py_str("show"), py_str("shows"), py_str("mean"), py_str("means"), py_str("numbers")])])); __args.push(g__smart_meter_how()); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("worth"), py_str("worth it"), py_str("should i"), py_str("good buy"), py_str("any good")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("watch-buy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("watch"), py_str("watches"), py_str("wristwatch"), py_str("wrist watch"), py_str("smartwatch"), py_str("smart watch"), py_str("apple watch"), py_str("garmin"), py_str("fitbit"), py_str("casio"), py_str("timex"), py_str("seiko"), py_str("normal watch"), py_str("ordinary watch"), py_str("mechanical watch"), py_str("analogue watch"), py_str("analog watch")]), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("get"), py_str("getting"), py_str("new"), py_str("which"), py_str("recommend"), py_str("worth"), py_str("good"), py_str("first"), py_str("choose"), py_str("choosing"), py_str("spend"), py_str("spending"), py_str("best"), py_str("pick"), py_str("should"), py_str("vs"), py_str("versus"), py_str("or")])])); __args.push(g__watch_help()); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("get"), py_str("getting"), py_str("new"), py_str("which"), py_str("recommend"), py_str("worth"), py_str("good"), py_str("first"), py_str("choose"), py_str("choosing"), py_str("spend"), py_str("spending"), py_str("best"), py_str("pick"), py_str("should")])), ("avoid".to_string(), py_tuple(vec![py_str("tonight"), py_str("film"), py_str("films"), py_str("movie"), py_str("movies"), py_str("series"), py_str("tv"), py_str("programme"), py_str("program"), py_str("show"), py_str("netflix"), py_str("episode"), py_str("stream"), py_str("streaming"), py_str("cinema"), py_str("box set"), py_str("match"), py_str("game"), py_str("football"), py_str("stopped"), py_str("broken"), py_str("broke"), py_str("fault"), py_str("repair"), py_str("strap"), py_str("battery"), py_str("not working"), py_str("screen"), py_str("loses time"), py_str("fast"), py_str("slow"), py_str("to watch"), py_str("watch tonight"), py_str("watch on"), py_str("whats on"), py_str("watch it"), py_str("should i watch"), py_str("should we watch"), py_str("shall i watch"), py_str("shall we watch"), py_str("can i watch"), py_str("can we watch"), py_str("i watch"), py_str("we watch"), py_str("worth watching")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("buy-a-vacuum")); __args.push(py_tuple(vec![py_tuple(vec![py_str("hoover"), py_str("hoovers"), py_str("vacuum cleaner"), py_str("vacuum cleaners"), py_str("vacuums")]), py_tuple(vec![py_str("buy"), py_str("buying"), py_str("look for"), py_str("choose"), py_str("choosing"), py_str("best"), py_str("worth"), py_str("which"), py_str("recommend"), py_str("new")])])); __args.push(py_str("Check the weight and the cord length before anything else, because the best vacuum is the one you will actually push round the house. A bagged machine holds its suction and suits anyone with allergies; a bagless one is cheaper to run but needs emptying over a bin. A cordless is a quick-clean tool and rarely enough on its own in a carpeted house, so try one for weight and check the filters can be washed.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("robot"), py_str("car"), py_str("flask"), py_str("seal"), py_str("bags for"), py_str("washing machine")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("price-match")); __args.push(py_tuple(vec![py_tuple(vec![py_str("price match"), py_str("price match it"), py_str("match the price"), py_str("match a price"), py_str("price promise"), py_str("price guarantee"), py_str("cheaper elsewhere"), py_str("cheaper in another shop"), py_str("cheaper online"), py_str("found it cheaper"), py_str("same thing cheaper"), py_str("beat the price"), py_str("beat it by")]), py_tuple(vec![py_str("can i"), py_str("will they"), py_str("do they"), py_str("ask"), py_str("get"), py_str("how"), py_str("shop"), py_str("store"), py_str("retailer"), py_str("online"), py_str("till"), py_str("checkout"), py_str("worth"), py_str("any point"), py_str("does"), py_str("you"), py_str("they"), py_str("it"), py_str("elsewhere"), py_str("i found")])])); __args.push(py_str("Ask at the till before you pay, because almost no retailer matches a price after the sale. Have the competitor's page open on your phone with the exact model, the price and the date, and check the small print first: most promises exclude marketplace and third-party sellers, sale and clearance prices, membership prices, vouchers and mis-priced listings. Some chains match online prices only against their own website, and a few knock off the difference rather than the whole amount. If the assistant says no, ask for the manager and ask which rule you missed - a written promise on the wall is worth quoting.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("flights"), py_str("hotel"), py_str("insurance"), py_str("energy"), py_str("broadband"), py_str("mobile"), py_str("mortgage"), py_str("train"), py_str("supermarket"), py_str("petrol")])), ("ask".to_string(), py_tuple(vec![py_str("do"), py_str("does"), py_str("can"), py_str("will"), py_str("should"), py_str("how"), py_str("worth"), py_str("found"), py_str("ask"), py_str("try")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("online-or-in-store")); __args.push(py_tuple(vec![py_tuple(vec![py_str("buy online"), py_str("buying online"), py_str("buy it online"), py_str("order online"), py_str("order it online"), py_str("in store"), py_str("in the shop"), py_str("in a shop"), py_str("in person"), py_str("go to the shop"), py_str("collect"), py_str("click and collect"), py_str("online or in store"), py_str("online or in the shop"), py_str("online or shop"), py_str("shop or online")]), py_tuple(vec![py_str("cheaper"), py_str("cheap"), py_str("better"), py_str("should i"), py_str("worth"), py_str("or"), py_str("which"), py_str("safer"), py_str("returns"), py_str("return"), py_str("rights"), py_str("guarantee"), py_str("warranty"), py_str("problem")])])); __args.push(py_str("Buying online gives you a 14-day right to change your mind from the day it arrives, and you only pay the return postage unless the shop says otherwise; buying in a shop gives you no right to change your mind at all, only the shop's own policy. That is the real difference, not the price. So for anything you might not keep - clothes, a mattress, furniture in a colour you have not seen - order online and check the returns page before you pay. For something you need today, or want to try first, buy in the shop and ask what their returns policy is in writing.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("flight"), py_str("hotel"), py_str("train"), py_str("ticket"), py_str("insurance"), py_str("car insurance"), py_str("energy"), py_str("broadband"), py_str("mobile phone contract"), py_str("grocery"), py_str("food shop")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("bag-charge")); __args.push(py_tuple(vec![py_tuple(vec![py_str("bag charge"), py_str("bag charges"), py_str("charge for bags"), py_str("carrier bag"), py_str("carrier bags"), py_str("plastic bag"), py_str("plastic bags"), py_str("5p bag"), py_str("10p bag"), py_str("bag for life"), py_str("reusable bag"), py_str("pay for bags"), py_str("pay for a bag"), py_str("bags cost"), py_str("charged for a bag"), py_str("charged for bags"), py_str("charged me for a bag"), py_str("charged me for bags"), py_str("a bag at the till"), py_str("charge me for a bag")]), py_tuple(vec![py_str("why"), py_str("how much"), py_str("have to"), py_str("must i"), py_str("do i"), py_str("charge"), py_str("cost"), py_str("free"), py_str("law"), py_str("shop"), py_str("supermarket"), py_str("still"), py_str("tax"), py_str("gone up"), py_str("till"), py_str("checkout"), py_str("charged me"), py_str("i was"), py_str("didnt ask"), py_str("didn't ask")])])); __args.push(py_str("Shops in the UK have to charge for single-use carrier bags, and the money goes to good causes rather than to the shop, which is why the charge exists at all: it is there to cut how many bags are thrown away, and it works. A bag for life is the cheap way round it - most supermarkets replace it free when it finally splits - and small paper bags for loose fruit and veg are usually free and are not the charge. If you are charged for a bag you did not ask for, say so at the till and it comes off.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("hand luggage"), py_str("airline"), py_str("flight"), py_str("carrier bag levy ireland"), py_str("bin bag"), py_str("vacuum"), py_str("hoover"), py_str("sleeping bag"), py_str("tea bag"), py_str("bags for"), py_str("didnt want"), py_str("didn't want"), py_str("did not want"), py_str("didnt need"), py_str("not ordered"), py_str("didnt order"), py_str("did not order")])), ("guard".to_string(), g__the_bag_is_not_packaging())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("complain-about-a-shop")); __args.push(py_tuple(vec![py_tuple(vec![py_str("complain about a shop"), py_str("complaint about a shop"), py_str("complain about a store"), py_str("complain to a shop"), py_str("complain to the shop"), py_str("complain about a purchase"), py_str("complain about the service"), py_str("bad service"), py_str("rude staff"), py_str("complaint about"), py_str("complain about a company"), py_str("escalate a complaint"), py_str("ombudsman"), py_str("small claims"), py_str("trading standards"), py_str("resolver"), py_str("complain"), py_str("complains"), py_str("complained"), py_str("complaint"), py_str("complaints")]), py_tuple(vec![py_str("how"), py_str("how do i"), py_str("shop"), py_str("store"), py_str("retailer"), py_str("company"), py_str("where"), py_str("next"), py_str("still not"), py_str("nothing happened"), py_str("no reply"), py_str("ignored"), py_str("refund"), py_str("money back")])])); __args.push(py_str("Complain to the shop first, in writing, and keep it to facts: what you bought, when, what went wrong, what you want and by when. Use email or the online form rather than the phone so there is a record, and give them 14 days. If they do not fix it, escalate inside the company - ask for the complaints procedure and the name of the person dealing with it - then go outside: Citizens Advice or Trading Standards for a legal point, the relevant ombudsman for a service, and the small claims track for money you are owed. Card payment is your spare lever: a claim under section 75 over 100 pounds, or a chargeback under it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("landlord"), py_str("council"), py_str("council tax"), py_str("nhs"), py_str("hospital"), py_str("dentist"), py_str("school"), py_str("doctor"), py_str("doctors"), py_str("gp"), py_str("surgery"), py_str("nurse"), py_str("clinic"), py_str("consultant"), py_str("midwife"), py_str("treatment"), py_str("care home"), py_str("care homes"), py_str("builder"), py_str("builders"), py_str("tradesman"), py_str("tradesmen"), py_str("plumber"), py_str("plumbing"), py_str("electrician"), py_str("decorator"), py_str("painter"), py_str("contractor"), py_str("rogue trader"), py_str("teacher"), py_str("boss"), py_str("employer"), py_str("bank"), py_str("energy"), py_str("broadband"), py_str("mobile"), py_str("manager"), py_str("managers"), py_str("management"), py_str("colleague"), py_str("colleagues"), py_str("coworker"), py_str("co-worker"), py_str("hr"), py_str("insurer"), py_str("insurers"), py_str("insurance"), py_str("policy"), py_str("claim"), py_str("claims"), py_str("flight"), py_str("airline"), py_str("train"), py_str("hotel"), py_str("delivery"), py_str("parcel"), py_str("neighbour"), py_str("neighbor"), py_str("noise"), py_str("my ex"), py_str("family"), py_str("restaurant"), py_str("takeaway"), py_str("cafe"), py_str("coffee shop"), py_str("pizza"), py_str("meal"), py_str("my order"), py_str("food order"), py_str("wrong order"), py_str("courier")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("return-without-receipt")); __args.push(py_tuple(vec![py_tuple(vec![py_str("receipt"), py_str("receipts"), py_str("proof of purchase"), py_str("no receipt"), py_str("without a receipt"), py_str("gift receipt"), py_str("lost the receipt"), py_str("lost my receipt"), py_str("bank statement"), py_str("card statement")]), py_tuple(vec![py_str("return"), py_str("returns"), py_str("returning"), py_str("refund"), py_str("refunds"), py_str("exchange"), py_str("back"), py_str("take it back"), py_str("take back"), py_str("without"), py_str("lost"), py_str("need"), py_str("still"), py_str("need to keep")])])); __args.push(py_str("You do not need the paper slip. Proof of purchase can be a bank or card statement showing the payment, an order confirmation email, a photo of the receipt or a gift receipt, and a shop that can see the purchase on its own system cannot insist on the paper. For anything faulty your claim is against the shop, not the maker: a full refund within 30 days, then repair or replacement, and in England, Wales and Northern Ireland up to six years to claim (five in Scotland) - a statement from your bank proves it just as well. If you are returning something because you changed your mind, there is no such right in a shop at all, so you are on their returns policy; ask the manager politely what proof they will accept, and if the goods were faulty and they still refuse, say you are going to Citizens Advice and put the claim in writing.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("read receipt"), py_str("read receipts"), py_str("blue tick"), py_str("blue ticks"), py_str("delivery"), py_str("parcel"), py_str("courier"), py_str("wan"), py_str("rent"), py_str("landlord"), py_str("tenant"), py_str("tenancy")])), ("guard".to_string(), g__not_a_length()), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("a-copy-of-the-receipt")); __args.push(py_tuple(vec![py_tuple(vec![py_str("receipt"), py_str("receipts"), py_str("till receipt"), py_str("proof of purchase"), py_str("vat receipt"), py_str("bank statement"), py_str("card statement")]), py_tuple(vec![py_str("copy"), py_str("duplicate"), py_str("replacement"), py_str("another"), py_str("reprint"), py_str("print"), py_str("printed"), py_str("emailed"), py_str("email"), py_str("didnt give"), py_str("didn't give"), py_str("never gave"), py_str("not given"), py_str("no receipt"), py_str("ask for"), py_str("asked for"), py_str("can i get"), py_str("how do i get"), py_str("cash")])])); __args.push(py_str("Any shop can print or email another copy, because it can find the sale from the date, the time on the till roll and the last four digits of your card, so ask at the till and say roughly when you paid. If it cannot, a card or bank statement showing the payment is proof of purchase for a return, and a photo of the paper slip in your phone covers most warranties and expenses. Ask for a VAT receipt when you pay if you need to reclaim the VAT, since an ordinary till receipt often has no VAT number on it.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("tax return"), py_str("self assessment"), py_str("hmrc"), py_str("expenses"), py_str("expense"), py_str("employer"), py_str("keep"), py_str("keeping"), py_str("how long"), py_str("read receipt"), py_str("delivery"), py_str("parcel"), py_str("courier"), py_str("donation"), py_str("donations"), py_str("charity"), py_str("gift aid"), py_str("rent"), py_str("landlord"), py_str("tenant"), py_str("tenancy")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("refund-how-long")); __args.push(py_tuple(vec![py_tuple(vec![py_str("refund"), py_str("refunds"), py_str("refunded"), py_str("money back"), py_str("returns money")]), py_tuple(vec![py_str("how long"), py_str("long"), py_str("when"), py_str("how many days"), py_str("days"), py_str("waiting"), py_str("take"), py_str("takes"), py_str("not had"), py_str("still"), py_str("not received"), py_str("chase"), py_str("chasing")])])); __args.push(py_str("Usually three to five working days on a card, and up to 14 days is still normal for online returns once the shop has the item back. Cash comes back at the till straight away, PayPal is often a day or two, and a card refund can take a full statement period to show because the bank holds it after the shop has paid it. If nothing has appeared after 14 days, send the shop a short written request with your order number and the date you returned it, then take it to your card provider, who can raise a chargeback.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("faulty"), py_str("broken"), py_str("sale"), py_str("flight"), py_str("airline"), py_str("train"), py_str("hotel"), py_str("holiday"), py_str("deposit"), py_str("rent"), py_str("landlord"), py_str("council"), py_str("tax"), py_str("ticket"), py_str("concert"), py_str("course"), py_str("how do i get"), py_str("entitled"), py_str("can i get")])), ("bias".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }]));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("olive-oil-pick")); __args.push(py_tuple(vec![py_tuple(vec![py_str("olive oil"), py_str("olive oils"), py_str("extra virgin"), py_str("extra virgin olive oil"), py_str("evoo")]), py_tuple(vec![py_str("pick"), py_str("choose"), py_str("choosing"), py_str("buy"), py_str("buying"), py_str("best"), py_str("good"), py_str("which"), py_str("worth"), py_str("supermarket"), py_str("brand"), py_str("bottle"), py_str("tin"), py_str("cheap"), py_str("expensive"), py_str("difference"), py_str("use"), py_str("cooking"), py_str("kitchen")])])); __args.push(py_str("Look for \"extra virgin\" and a harvest date, not a best-before: oil is a fruit juice and it fades, so the newest pressing tastes best. A dark glass bottle or a tin keeps it that way, and a peppery catch at the back of your throat means the good bitter polyphenols are still there. Spend on the bottle you use raw - dressing, bread, a finished dish - and use a cheaper olive oil or a plain oil for roasting and frying. \"Light\" on the label means light in taste, not in calories.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("car"), py_str("cars"), py_str("engine"), py_str("motorbike"), py_str("bike"), py_str("hair"), py_str("skin"), py_str("essential oil"), py_str("olive tree"), py_str("grow"), py_str("fryer"), py_str("deep fat"), py_str("chip pan"), py_str("paint"), py_str("lamp"), py_str("cake"), py_str("cakes"), py_str("sponge"), py_str("muffin"), py_str("muffins"), py_str("loaf"), py_str("loaves"), py_str("brownies"), py_str("brownie"), py_str("traybake"), py_str("tray bake"), py_str("carrot cake"), py_str("chocolate cake"), py_str("banana bread"), py_str("pastry"), py_str("shortbread"), py_str("biscuits"), py_str("cookies"), py_str("good for you"), py_str("good for me"), py_str("good for us"), py_str("bad for you"), py_str("bad for me"), py_str("healthy"), py_str("unhealthy"), py_str("fattening"), py_str("calorie"), py_str("calories"), py_str("cholesterol"), py_str("heart"), py_str("fry"), py_str("fries"), py_str("frying"), py_str("deep fry"), py_str("roasting"), py_str("roast potatoes")])), ("ask".to_string(), py_tuple(vec![py_str("pick"), py_str("choose"), py_str("choosing"), py_str("buy"), py_str("buying"), py_str("best"), py_str("good"), py_str("which"), py_str("worth"), py_str("supermarket"), py_str("brand"), py_str("difference"), py_str("use"), py_str("cooking")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("gift-house-move")); __args.push(py_tuple(vec![py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("what should i get"), py_str("what can i get"), py_str("what do i get"), py_str("what to get"), py_str("what should i buy"), py_str("what can i buy"), py_str("what to buy"), py_str("what can i give"), py_str("what should i give"), py_str("what to give"), py_str("what do i give"), py_str("what should i bring"), py_str("what to bring"), py_str("what can i bring"), py_str("what do you get"), py_str("what do you buy"), py_str("what do you give"), py_str("housewarming present"), py_str("housewarming gift"), py_str("ideas"), py_str("idea")]), py_tuple(vec![py_str("moving house"), py_str("moving home"), py_str("moving in"), py_str("moving into"), py_str("just moved"), py_str("moved house"), py_str("moved in"), py_str("new house"), py_str("new flat"), py_str("new home"), py_str("first home"), py_str("first flat"), py_str("housewarming"), py_str("house warming"), py_str("moving away"), py_str("bought a house"), py_str("got the keys"), py_str("new place"), py_str("new apartment")])])); __args.push(py_str("Buy for the first night, not for the shelves: a takeaway voucher for the evening the boxes are still out, a box with loo roll, tea, milk, mugs, bin bags, a corkscrew and a phone charger, or a hot meal you turn up with. A plant, a good bottle or a decent set of tea towels all last longer than another candle, and a voucher for a hardware shop is the unromantic one people are pleased about. Ask first whether they want things or help: an afternoon of unpacking and a trip to the tip is often the present.")); __kws.extend(vec![("ask".to_string(), py_tuple(vec![py_str("gift"), py_str("gifts"), py_str("present"), py_str("presents"), py_str("get"), py_str("buy"), py_str("give"), py_str("bring"), py_str("take round"), py_str("ideas"), py_str("idea")])), ("avoid".to_string(), op_add(&op_add(&g_LOSS(), &g_CRISIS()), &py_tuple(vec![py_str("where to start"), py_str("where do i start"), py_str("where do we start"), py_str("where should i start"), py_str("where to begin"), py_str("where do i begin"), py_str("no idea where to start"), py_str("dont know where to start"), py_str("don't know where to start"), py_str("not sure where to start"), py_str("my new flat"), py_str("my new house"), py_str("my new place"), py_str("our new house"), py_str("our new flat"), py_str("my first flat"), py_str("my first house"), py_str("im moving"), py_str("i am moving"), py_str("were moving"), py_str("we are moving"), py_str("i move"), py_str("we move"), py_str("removals"), py_str("van hire"), py_str("packing"), py_str("council tax"), py_str("utilities"), py_str("school"), py_str("doctor"), py_str("deposit"), py_str("landlord"), py_str("mortgage"), py_str("hire a van"), py_str("man and van")]))), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    s__CHOOSE_ASK(py_tuple(vec![py_str("pick"), py_str("picking"), py_str("choose"), py_str("choosing"), py_str("choice"), py_str("buy"), py_str("buying"), py_str("bought"), py_str("best"), py_str("good"), py_str("decent"), py_str("which"), py_str("what should i look for"), py_str("look for"), py_str("look out for"), py_str("worth"), py_str("spend"), py_str("budget"), py_str("recommend"), py_str("recommendation"), py_str("what to get"), py_str("which to get"), py_str("what should i get"), py_str("need a"), py_str("need some"), py_str("want a"), py_str("size"), py_str("much"), py_str("price"), py_str("cost"), py_str("cheap"), py_str("expensive"), py_str("difference"), py_str("better"), py_str("worse"), py_str("bigger"), py_str("smaller"), py_str("cheaper"), py_str("dearer")]));
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("duvet-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("duvet"), py_str("duvets"), py_str("quilt"), py_str("quilts"), py_str("continental quilt"), py_str("tog")]), g__CHOOSE_ASK()])); __args.push(py_str("Go by tog: 4.5 for summer, 9 to 10.5 for spring and autumn, 13.5 for winter, or one 10.5 all year with a 4.5 on top in January. Match the size to the bed - single 135x200cm, double 200x200, king 225x220, super king 260x220 - because a duvet smaller than the bed leaves a cold edge. Down and feather are warmest for the weight, wool breathes and handles damp best, and hollowfibre is the cheap washable one; a synthetic duvet goes in a 9kg drum at home, while down wants a launderette and a tumble dryer with dryer balls to bring the loft back.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("wash"), py_str("washing"), py_str("washed"), py_str("clean"), py_str("cleaning"), py_str("dry clean"), py_str("dry clean only"), py_str("dryer"), py_str("tumble"), py_str("machine"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("store"), py_str("storing"), py_str("storage"), py_str("put away"), py_str("pack away"), py_str("summer storage"), py_str("cover"), py_str("covers"), py_str("coverless"), py_str("sheet"), py_str("sheets"), py_str("bedding set"), py_str("mend"), py_str("repair"), py_str("torn"), py_str("rip"), py_str("hole"), py_str("allergy"), py_str("allergies"), py_str("asthma"), py_str("eczema"), py_str("dust mite")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pillow-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("pillow"), py_str("pillows"), py_str("pillowcase")]), g__CHOOSE_ASK()])); __args.push(py_str("Match the pillow to how you sleep: on your side you want a firm, high one that fills the gap between shoulder and ear, on your back a medium one, and on your front a soft, thin one so your neck is not cranked up. A pillow that folds in half and stays folded is worn out - most last two to three years, not ten. Memory foam and latex hold their shape, hollowfibre is cheap and washable at 60C, and down is soft but needs plumping; a washable protector keeps whichever you pick clean.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("wash"), py_str("washing"), py_str("washed"), py_str("clean"), py_str("cleaning"), py_str("dry clean"), py_str("dryer"), py_str("tumble"), py_str("machine"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("yellow"), py_str("mould"), py_str("mold"), py_str("neck"), py_str("stiff neck"), py_str("sore"), py_str("ache"), py_str("aches"), py_str("aching"), py_str("pain"), py_str("hurt"), py_str("hurts"), py_str("sciatica"), py_str("migraine"), py_str("headache"), py_str("sew"), py_str("sewing"), py_str("make"), py_str("making"), py_str("pattern"), py_str("toddler"), py_str("baby"), py_str("cot"), py_str("cushion"), py_str("cushions"), py_str("throw")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("kettle-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("kettle"), py_str("kettles")]), g__CHOOSE_ASK()])); __args.push(py_str("Get a 3kW one for speed and 1.7 litres for a family, or a 1 litre one if it is mostly you - a small kettle boiled often costs less than a big one boiled for two cups. A concealed element and a wide lid make descaling easy, a water-level window on both sides saves guessing, and temperature settings are worth it only if you make green tea or filter coffee. Decide stainless steel or glass by looks, then check the minimum fill line: a kettle that needs half a litre before it will switch on wastes water and power.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("descale"), py_str("descaling"), py_str("limescale"), py_str("scale"), py_str("fur"), py_str("furred"), py_str("broken"), py_str("not working"), py_str("stopped"), py_str("faulty"), py_str("gone"), py_str("went"), py_str("knackered"), py_str("on the blink"), py_str("leak"), py_str("leaks"), py_str("leaking"), py_str("switch"), py_str("element"), py_str("noisy"), py_str("rattle"), py_str("boil"), py_str("boiling"), py_str("boils"), py_str("how long"), py_str("vinegar"), py_str("citric"), py_str("bicarbonate"), py_str("tea"), py_str("teapot"), py_str("coffee"), py_str("cup of tea"), py_str("electricity"), py_str("energy")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("suitcase-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("suitcase"), py_str("suitcases"), py_str("cabin bag"), py_str("cabin case"), py_str("cabin luggage"), py_str("hand luggage"), py_str("carry on"), py_str("carry-on"), py_str("holdall"), py_str("luggage"), py_str("case")]), g__CHOOSE_ASK()])); __args.push(py_str("Start with the airline, because cabin sizes differ: a 55x40x20cm case fits most paid cabin allowances, while the free underseat bag is around 40x20x25cm, and a bag outside the airline's own limit is the one thing that gets charged at the gate. Four wheels and a hard polycarbonate shell survive the hold better, soft bags give and squash into a car; either way check the empty weight against a 23kg allowance and buy from a brand that covers wheels and handles for years rather than months. A TSA lock matters only for the US. For a week away, 40 to 45 litres of cabin case is enough if you pack light and wash a shirt.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("lost"), py_str("missing"), py_str("delayed"), py_str("damaged"), py_str("broken"), py_str("wheel broke"), py_str("zip"), py_str("zips"), py_str("lock stuck"), py_str("stolen"), py_str("claim"), py_str("compensation"), py_str("airline"), py_str("flight"), py_str("pack"), py_str("packing"), py_str("packed"), py_str("unpack"), py_str("unpacking"), py_str("what to take"), py_str("what to pack"), py_str("holiday packing"), py_str("storage"), py_str("store"), py_str("handbag"), py_str("handbag"), py_str("briefcase"), py_str("laptop bag"), py_str("phone"), py_str("iphone"), py_str("mobile"), py_str("tablet"), py_str("ipad"), py_str("laptop"), py_str("glasses"), py_str("spectacles"), py_str("hearing aid"), py_str("airpods"), py_str("earbuds"), py_str("camera"), py_str("console"), py_str("switch case")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sofa-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sofa"), py_str("sofas"), py_str("settee"), py_str("couch"), py_str("three piece suite"), py_str("armchair")]), g__CHOOSE_ASK()])); __args.push(py_str("Measure the room, the door and the stairs before you fall for one, and keep 60cm of walking space past it. Then look at the build: a hardwood or beech frame screwed and glued, not stapled chipboard, and seat cushions of 30 to 35kg foam with a fibre wrap, or feather if you will plump them daily. Sit on it for ten minutes rather than three, and check the fabric's rub count - 25,000 for adults, 30,000 and up for children and pets. Made-to-order sofas take six to twelve weeks, so ask about the return policy before you pay.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("washer"), py_str("wash"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("pet hair"), py_str("sag"), py_str("sagging"), py_str("saggy"), py_str("squeak"), py_str("squeaks"), py_str("broken"), py_str("repair"), py_str("re-upholster"), py_str("reupholster"), py_str("moving house"), py_str("move a sofa"), py_str("get it upstairs"), py_str("cover"), py_str("covers"), py_str("throw"), py_str("throws"), py_str("cushion"), py_str("cushions"), py_str("leather"), py_str("scratched")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("coffee-to-buy")); __args.push(py_tuple(vec![py_tuple(vec![py_str("coffee"), py_str("coffees"), py_str("coffee beans"), py_str("ground coffee"), py_str("instant coffee"), py_str("filter coffee")]), py_tuple(vec![py_str("pick"), py_str("picking"), py_str("choose"), py_str("choosing"), py_str("buy"), py_str("buying"), py_str("best"), py_str("good"), py_str("decent"), py_str("which"), py_str("what should i look for"), py_str("look for"), py_str("worth"), py_str("roast"), py_str("bean"), py_str("beans"), py_str("brand"), py_str("supermarket"), py_str("grind"), py_str("ground"), py_str("roasted"), py_str("fresh"), py_str("strength"), py_str("strong")])])); __args.push(py_str("Look for a roast date, not a best-before: beans are best from a few days to about six weeks after roasting, and supermarket bags with only a best-before are usually months old. Buy whole bean and grind per brew - fine for espresso, medium for a filter machine, coarse for a cafetiere - or ask the shop to grind it for the pot you use. A medium roast suits most brewing, and about 60g of coffee to a litre of water is the strength to start from. Keep the bag sealed and in a cupboard, not the fridge.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("make"), py_str("making"), py_str("brew"), py_str("brewing"), py_str("brewed"), py_str("cup"), py_str("cups"), py_str("mug"), py_str("machine"), py_str("machine descale"), py_str("descale"), py_str("caffeine"), py_str("sleep"), py_str("anxiety"), py_str("good for you"), py_str("bad for you"), py_str("healthy"), py_str("pregnant"), py_str("pregnancy"), py_str("withdraw"), py_str("withdrawal"), py_str("stop"), py_str("give up"), py_str("quit"), py_str("cafe"), py_str("cafes"), py_str("barista"), py_str("shop"), py_str("starbucks"), py_str("costa"), py_str("stain"), py_str("stains"), py_str("grinder"), py_str("frother"), py_str("milk"), py_str("milks"), py_str("dairy")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("rug-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("rug"), py_str("rugs"), py_str("carpet rug"), py_str("runner"), py_str("runners")]), g__CHOOSE_ASK()])); __args.push(py_str("Go bigger than feels right: in a living room the rug should reach under the front legs of the sofa, so about 160x230cm for a small room and 200x290cm behind a three-seater, and a bedroom rug wants to run at least 60cm past each side of the bed. Wool wears and cleans better than polyester and does not flatten as fast; a thick underlay makes a cheap rug feel expensive and stops it creeping. Take the size you are considering and tape it out on the floor before you buy, and check it can be washed or spot-cleaned.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("clean"), py_str("cleaning"), py_str("wash"), py_str("washing"), py_str("stain"), py_str("stains"), py_str("smell"), py_str("smells"), py_str("moth"), py_str("moths"), py_str("slipping"), py_str("slips"), py_str("slides"), py_str("curl"), py_str("curling"), py_str("creases"), py_str("shed"), py_str("shedding"), py_str("hoover"), py_str("vacuum"), py_str("flat"), py_str("flattened"), py_str("repair"), py_str("fringe"), py_str("fringes"), py_str("lay"), py_str("laying")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("curtains-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("curtain"), py_str("curtains"), py_str("blind"), py_str("blinds"), py_str("drapes"), py_str("curtain rail"), py_str("curtain pole")]), g__CHOOSE_ASK()])); __args.push(py_str("Buy one and a half to twice the window width in fabric, or the curtains hang flat and let light round the edges, and measure the drop from the pole or track to where you want them to end - just below the sill, or to the floor for a longer look. A lined curtain keeps heat in and light out far better than an unlined one, and a thermal lining is worth it in a bedroom. Eyelet headings slide easily on a pole, pencil pleat gives a fuller look on a track; for anything over 2m wide, two panels rather than one.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("hang"), py_str("hanging"), py_str("hem"), py_str("hemming"), py_str("shorten"), py_str("lengthen"), py_str("sew"), py_str("sewing"), py_str("iron"), py_str("ironing"), py_str("wash"), py_str("washing"), py_str("clean"), py_str("cleaning"), py_str("dry clean"), py_str("stain"), py_str("stains"), py_str("mould"), py_str("mold"), py_str("damp"), py_str("black mould"), py_str("repair"), py_str("broken"), py_str("cord"), py_str("track broken"), py_str("child safety"), py_str("blind cord")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("toaster-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("toaster"), py_str("toasters")]), g__CHOOSE_ASK()])); __args.push(py_str("Two slots do for one or two people and four for a family, and the slot width matters more than the wattage: look for wide slots if you toast bagels, crumpets or thick-cut bread. Variable browning plus a cancel button is the pair that stops you scraping burnt toast, and a defrost setting is worth having if you freeze bread. A removable crumb tray and a cord that tucks away are the two things you will notice every week; anything over about 1,500W toasts no better than 900W.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("broken"), py_str("not working"), py_str("stopped"), py_str("faulty"), py_str("smoking"), py_str("smoke"), py_str("burnt"), py_str("burning"), py_str("stuck"), py_str("wont pop"), py_str("won't pop"), py_str("pop up"), py_str("clean"), py_str("cleaning"), py_str("crumbs"), py_str("crumb tray"), py_str("repair"), py_str("fuse"), py_str("element"), py_str("sandwich"), py_str("toastie"), py_str("air fryer"), py_str("grill")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    s_KITCHEN_MACHINE_VALUE(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("rice cooker"), py_str("rice cookers")]), py_str("A rice cooker is worth it if rice is a daily thing: it times it, keeps it warm for an hour and leaves the hob free, and a 40 pound one cooks as well as a 200 pound one. Skip it if you eat rice twice a month, since a pan with a lid is the same rice in 12 minutes.")]), py_tuple(vec![py_tuple(vec![py_str("soup maker"), py_str("soup makers")]), py_str("A soup maker is worth it if you make soup most weeks and want one jug and no pan to watch: it cooks and blends in about 20 minutes. Skip it if you like soup with chunks in it, or cook for four, because most jugs hold 1.6 litres and the blade takes everything smooth.")]), py_tuple(vec![py_tuple(vec![py_str("sous vide"), py_str("sous vide stick"), py_str("sous vide machine"), py_str("sous vide cooker")]), py_str("Sous vide is worth it if you cook steak, chicken breast or eggs to a temperature and do not mind waiting: a 60 pound stick in a lidded pan holds the water steady for an hour or more, and nothing overcooks. Skip it if dinner has to be on the table in 20 minutes, where a thermometer and a hot pan get most of the way.")]), py_tuple(vec![py_tuple(vec![py_str("dehydrator"), py_str("dehydrators"), py_str("food dehydrator")]), py_str("A dehydrator is worth it if you dry things often: herbs, apple rings, jerky or vegetables for stock, in 4 to 12 hours at 40 to 60C with no oven on. Skip it if that is a twice-a-year job, since an oven on its lowest setting with the door ajar dries a tray at a time.")]), py_tuple(vec![py_tuple(vec![py_str("juicer"), py_str("juicers")]), py_str("A juicer is worth it only if you drink juice most mornings: it separates the pulp in seconds, and cleaning the mesh takes five minutes every time. Skip it if you would rather eat the fruit, which keeps the fibre the juice leaves behind.")]), py_tuple(vec![py_tuple(vec![py_str("yogurt maker"), py_str("yoghurt maker"), py_str("yogurt makers")]), py_str("A yogurt maker is worth it if you get through a tub a week: a litre of milk makes a litre of yogurt for about 60p, and it holds the warmth steady for 8 to 12 hours. Skip it if you eat yogurt now and then, since a flask and a warm cupboard do the same job.")]), py_tuple(vec![py_tuple(vec![py_str("ice cream maker"), py_str("ice cream makers")]), py_str("An ice cream maker is worth it if you make it more than a few times a summer: the bowl has to be frozen overnight, so it is never a last-minute thing, and a 40 pound one with a pre-frozen bowl is fine. Skip it if you would use it twice, where a loaf tin and a fork every half hour gets close.")]), py_tuple(vec![py_tuple(vec![py_str("contact grill"), py_str("panini press"), py_str("sandwich press"), py_str("george foreman"), py_str("health grill")]), py_str("A contact grill is worth it if you toast sandwiches or cook chops and vegetables for one or two: it heats both sides at once and the fat runs off. Skip it if you cook for four, since the plates take two sandwiches at a time, and skip it if you have a frying pan and a lid.")]), py_tuple(vec![py_tuple(vec![py_str("electric steamer"), py_str("steam cooker"), py_str("food steamer")]), py_str("An electric steamer is worth it if you steam vegetables, dumplings or fish often and like setting a timer and walking away: the tiers hold a whole dinner over one ring. Skip it if a pan with a colander and a lid already does it, which for most people it does.")]), py_tuple(vec![py_tuple(vec![py_str("electric whisk"), py_str("hand whisk"), py_str("hand mixer"), py_str("electric hand whisk")]), py_str("An electric hand whisk is worth it for anyone who creams butter or whips cream even a few times a month: about 15 pounds, a minute instead of ten with a fork, and it lives in a drawer. Skip it only if you bake twice a year.")]), py_tuple(vec![py_tuple(vec![py_str("mandoline"), py_str("mandolines"), py_str("mandoline slicer")]), py_str("A mandoline is worth it if you slice a lot of vegetables thin and even - gratins, slaw, pickles - and you will use the hand guard every single time. Skip it if you cook for one: a sharp knife is safer and nearly as quick.")]), py_tuple(vec![py_tuple(vec![py_str("spiralizer"), py_str("spiralizers"), py_str("spiraliser")]), py_str("A spiralizer is worth it only if you really eat courgette or carrot noodles: a 10 pound hand-held one does it in a minute and the counter-top ones add nothing. Skip it if you would use it once, where a julienne peeler does the same job and fits in a drawer.")]), py_tuple(vec![py_tuple(vec![py_str("soda stream"), py_str("sodastream"), py_str("soda maker"), py_str("sparkling water maker")]), py_str("A soda maker is worth it if you drink sparkling water every day: a gas cylinder makes about 60 litres for 10 pounds to swap, so it pays for itself in a few months and saves carrying bottles. Skip it if you drink it now and then, or would mostly use the flavour syrups, which cost more than supermarket bottles.")]), py_tuple(vec![py_tuple(vec![py_str("vacuum sealer"), py_str("vacuum sealers"), py_str("food sealer"), py_str("food sealers")]), py_str("A vacuum sealer is worth it if you buy meat in bulk or freeze portions: it stops freezer burn and the bags can be boiled, so a batch of chilli reheats in the bag. Skip it if you freeze little, since a freezer bag with the air pressed out does most of it.")])]));
    s_APPLIANCE_VALUE(op_add(&g_APPLIANCE_VALUE(), &g_KITCHEN_MACHINE_VALUE()));
    s_KITCHEN_MACHINE_NAMES({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_KITCHEN_MACHINE_VALUE()) { let __unpk1441 = py_unpack(&__item.clone(), 2, None); let mut names: Py = __unpk1441[0].clone(); let mut _trade: Py = __unpk1441[1].clone(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push(name.clone()); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let __iter1 = py_iter_or_raise(&g_ENTRIES());
    for __item in __iter1 {
    s__worth_it_entry(__item.clone());
    if py_truthy(&Py::Bool(py_eq(&gattr(&g__worth_it_entry(), "id"), &py_str("appliance-worth")))) {
    let _ = py_setattr(&g__worth_it_entry(), "key", op_add(&py_tuple(vec![op_add(&geti(&gattr(&g__worth_it_entry(), "key"), &Py::Int(0)), &g_KITCHEN_MACHINE_NAMES())]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&gattr(&g__worth_it_entry(), "key"), &Py::Int(1), &Py::None, &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sleeping-bag-choose")); __args.push(py_tuple(vec![py_tuple(vec![py_str("sleeping bag"), py_str("sleeping bags"), py_str("camping bag"), py_str("down bag"), py_str("mummy bag")]), g__CHOOSE_ASK()])); __args.push(py_str("Ignore the \"extreme\" number and look at the comfort rating: a 2 or 3 season bag rated to about 0C comfort covers British camping from spring to autumn, and a 3 season one is the usual first buy. Down is lighter and packs smaller but useless once wet, synthetic keeps some warmth damp and is cheaper and washable, and a mummy shape is warmer for its weight than a rectangular one. The mat under you decides how warm you sleep as much as the bag does, so spend there too.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("wash"), py_str("washing"), py_str("clean"), py_str("cleaning"), py_str("damp"), py_str("wet"), py_str("smell"), py_str("smells"), py_str("smelly"), py_str("mould"), py_str("mold"), py_str("store"), py_str("storing"), py_str("storage"), py_str("pack"), py_str("packing"), py_str("stuff sack"), py_str("zip"), py_str("zips"), py_str("repair"), py_str("torn"), py_str("cold at night"), py_str("cant sleep"), py_str("can't sleep"), py_str("sleeping badly"), py_str("insomnia"), py_str("tent"), py_str("mat"), py_str("airbed"), py_str("camping site")])), ("bias".to_string(), Py::Int(2))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("second-hand-worth-it")); __args.push(py_tuple(vec![py_tuple(vec![py_str("second hand"), py_str("secondhand"), py_str("second-hand"), py_str("pre owned"), py_str("preowned"), py_str("pre-owned"), py_str("used"), py_str("buying used"), py_str("buy used"), py_str("charity shop"), py_str("car boot"), py_str("facebook marketplace"), py_str("gumtree"), py_str("vinted"), py_str("ebay"), py_str("refurbished"), py_str("refurb")]), py_tuple(vec![py_str("cheaper"), py_str("cheap"), py_str("save"), py_str("saving"), py_str("worse"), py_str("worth"), py_str("worth it"), py_str("should i buy"), py_str("should i get"), py_str("should we buy"), py_str("should we get"), py_str("good idea"), py_str("buy"), py_str("buying"), py_str("or new"), py_str("vs new"), py_str("instead of new"), py_str("money"), py_str("value"), py_str("second hand"), py_str("used")])])); __args.push(py_str("Yes, usually by a third to a half, and the older the thing gets the better the saving: furniture, bikes, tools, a pram, kids' clothes and kitchen kit lose most of their value in the first year and then almost nothing. Buy new the things that wear unseen or are used against your body - a mattress, running shoes, a bike helmet, anything with a safety job (a car seat, a life jacket, a smoke alarm) - and buy new where a warranty is what you are really paying for, like a laptop or a washing machine. Check a used phone or laptop's battery health, ask for the age of a washing machine's drum, and never pay before you see it. From a shop, second-hand goods carry the same rights as new ones: 30 days to reject it, then repair or replacement; from a private seller it is buyer beware unless the advert lied.")); __kws.extend(vec![("avoid".to_string(), py_tuple(vec![py_str("car"), py_str("cars"), py_str("van"), py_str("vans"), py_str("motorbike"), py_str("motorbikes"), py_str("vehicle"), py_str("mileage"), py_str("mot"), py_str("v5"), py_str("bike"), py_str("bikes"), py_str("bicycle"), py_str("cycle"), py_str("house"), py_str("flat"), py_str("mortgage"), py_str("phone"), py_str("phones"), py_str("mobile"), py_str("iphone"), py_str("android"), py_str("laptop"), py_str("laptops"), py_str("tablet"), py_str("tablets"), py_str("computer"), py_str("computer"), py_str("console"), py_str("games console"), py_str("sell"), py_str("selling"), py_str("sold"), py_str("sale"), py_str("declare"), py_str("tax"), py_str("to the taxman"), py_str("furniture"), py_str("sofa"), py_str("sofas"), py_str("couch"), py_str("couches"), py_str("settee"), py_str("armchair"), py_str("armchairs"), py_str("wardrobe"), py_str("wardrobes"), py_str("dresser"), py_str("sideboard"), py_str("sideboards"), py_str("bookshelf"), py_str("bookshelves"), py_str("bookcase"), py_str("bookcases"), py_str("bed frame"), py_str("table"), py_str("tables"), py_str("chairs"), py_str("cabinet"), py_str("cabinets"), py_str("chest of drawers"), py_str("look for"), py_str("what should i look for"), py_str("what to look for"), py_str("what should i check"), py_str("what to check"), py_str("check"), py_str("checks"), py_str("checking"), py_str("checked"), py_str("safe"), py_str("safely"), py_str("scam"), py_str("scams"), py_str("scammer"), py_str("scammed"), py_str("trust"), py_str("trustworthy"), py_str("risky"), py_str("risk"), py_str("insurance"), py_str("watch"), py_str("watches"), py_str("wristwatch"), py_str("clock"), py_str("clocks"), py_str("jewellery"), py_str("jewelry"), py_str("ring"), py_str("rings"), py_str("necklace"), py_str("bracelet"), py_str("earrings"), py_str("brooch"), py_str("antique"), py_str("antiques"), py_str("collectible"), py_str("collectibles"), py_str("vinyl"), py_str("record"), py_str("records"), py_str("book"), py_str("books"), py_str("novel"), py_str("novels")])), ("ask".to_string(), py_tuple(vec![py_str("cheaper"), py_str("worth"), py_str("should i"), py_str("buy"), py_str("buying"), py_str("save"), py_str("value")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::entry(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_ENTRIES(), "append", args))(&__args) };
}
pub fn _supermarket_quality(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut named: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: named"))) };
    let mut cheap: Py = Py::None;
    let mut contrast: Py = Py::None;
    let mut dear: Py = Py::None;
    let mut name: Py = Py::None;
    let mut rank: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut why: Py = Py::None;
    let _ = py_str("How shops compare on quality, which is a different answer from how they compare on price.\n\n    Each shop's own line from the table leads, because \"is farmfoods any good\" is about frozen\n    food and a thin fresh range rather than about the rank, and no card here names its own list of\n    staples: the coeliac rewrite in `cooking.py` turns a listed \"pasta\" into \"gluten-free pasta\",\n    which would have this row claim the discounters sell the same food as everyone else.\n    ");
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "sort", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    let __unpk1 = py_unpack(&py_tuple(vec![geti(&named.clone(), &Py::Int(0)), geti(&named.clone(), &op_neg(&Py::Int(1)))]), 2, None);
    cheap = __unpk1[0].clone();
    dear = __unpk1[1].clone();
    if py_truthy(&op_lt(&geti(&cheap.clone(), &Py::Int(0)), &geti(&dear.clone(), &Py::Int(0)))) {
    return op_modulo(&py_str("On quality they are close on the staples: most own-brand basics are made in the same factories as the named ones, so the store cupboard is the same food under a different label. It shows in the fresh food and the range - %s %s, where %s %s. For a full shop the cheaper one is fine; for a specific ingredient, or for a weekend meal, the dearer one earns it."), &py_tuple(vec![geti(&cheap.clone(), &Py::Int(1)), geti(&cheap.clone(), &Py::Int(4)), geti(&dear.clone(), &Py::Int(1)), geti(&dear.clone(), &Py::Int(4))]));
    }
    return op_modulo(&py_str("On quality they are closer than on price: the basics are the same food under a different label, so what you pay for is the fresh food and the range - %s %s, and %s %s."), &py_tuple(vec![geti(&cheap.clone(), &Py::Int(1)), geti(&cheap.clone(), &Py::Int(4)), geti(&dear.clone(), &Py::Int(1)), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(geti(&cheap.clone(), &Py::Int(4))); __args.push(geti(&dear.clone(), &Py::Int(4))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _without_the_same_opening(args))(&__args) }]));
    }
    let __unpk2 = py_unpack(&geti(&named.clone(), &Py::Int(0)), 5, None);
    rank = __unpk2[0].clone();
    name = __unpk2[1].clone();
    sentence = __unpk2[2].clone();
    why = __unpk2[3].clone();
    contrast = __unpk2[4].clone();
    if py_truthy(&op_ge(&rank.clone(), &Py::Int(4))) {
    return op_modulo(&py_str("%s is %s: %s. The extra money goes on the parts of the shop the staples do not cover, not on the basics, which are the same items under a different label, so a full shop there costs more for much the same basket."), &py_tuple(vec![name.clone(), sentence.clone(), why.clone()]));
    }
    if py_truthy(&op_le(&rank.clone(), &Py::Int(1))) {
    return op_modulo(&py_str("%s is %s: %s. None of that makes the staples worse: most own-brand basics come from the same factories as the named ones, so the saving is on the same food, and a basket built on ingredients rather than ready-made versions of them misses the weakest part."), &py_tuple(vec![name.clone(), sentence.clone(), why.clone()]));
    }
    if py_truthy(&Py::Bool(py_eq(&rank.clone(), &Py::Int(3)))) {
    return op_modulo(&py_str("%s is %s: %s. Much the same as the rest on quality: the basics are the same items, and what the extra buys is convenience."), &py_tuple(vec![name.clone(), sentence.clone(), why.clone()]));
    }
    return op_modulo(&py_str("%s is %s: %s. Much the same as the rest on quality: the basics are the same items at a price-matched price, and what it adds is range and fresh food."), &py_tuple(vec![name.clone(), sentence.clone(), why.clone()]));
    Py::None
}
pub fn _without_the_same_opening(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut first: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "first") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: first"))) };
    let mut second: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "second") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: second"))) };
    let mut lead: Py = Py::None;
    let _ = py_str("The second shop's clause with the words the first has already used taken off the front.\n\n    Two shops on the same rank both \"cost more\", so the sentence said it twice (\"M&S costs more,\n    and the ready meals are why people go, and Waitrose costs more and is better on fresh food\").\n    ");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("costs more, and "), py_str("costs more and "), py_str("costs more, "), py_str("costs more ")]));
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("costs more")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&first.clone(), "startswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&second.clone(), "startswith", args))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&slice_or_raise(&second.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lead.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::None, &Py::None), "strip", args))(&__args) };
    }
    }
    return second.clone();
    Py::None
}
pub fn _a_service_not_a_basket(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is about something a shop sells besides food.");
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SERVICE_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_turn_just_typed(args: &[Py]) -> Py {
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
    let _ = py_str("The words of the line the person has just sent, whatever reading carried the entry here.");
    let mut recent: Py = py_func("recent", |args: &[Py]| gen_memory::memory::recent(args));
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&recent, args))(&__args) };
    return if py_truthy(&turns.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { py_str("") };
    Py::None
}
pub fn _a_statement_about_where_i_shop(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut typed: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "typed") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: typed"))) };
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line just says where they shop, with no question about it.\n\n    \"i shop at asda\" and \"i do my shopping at asda\" are the two ways people tell you their shop,\n    and both were answered with a price card (\"Tesco is mid-priced\"). A question in front of the\n    same words - \"should i shop at asda or tesco\", \"is asda good for the weekly shop\" - has one of\n    the asking words in it, and that is the whole difference.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("i shop at"), py_str("i shop in"), py_str("i get my shopping")])) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&typed.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__SHOP_ASK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&typed.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    Py::None
}
pub fn _supermarket_cheapest(args: &[Py]) -> Py {
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
    let mut cheap: Py = Py::None;
    let mut contrast: Py = Py::None;
    let mut dear: Py = Py::None;
    let mut line: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut rank: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut spelling: Py = Py::None;
    let mut spellings: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut why: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Where the shops named in the line sit on price, or None when the line names none.");
    typed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_turn_just_typed(args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RANGE_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&typed.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("’")); __args.push(py_str("'")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }, "lower", args))(&__args) }, "replace", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__RANGE_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_service_not_a_basket(args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_statement_about_where_i_shop(args))(&__args) }) {
    return Py::None;
    }
    named = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&g_SUPERMARKETS());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 6, None);
    name = __unpk2[0].clone();
    spellings = __unpk2[1].clone();
    rank = __unpk2[2].clone();
    sentence = __unpk2[3].clone();
    why = __unpk2[4].clone();
    contrast = __unpk2[5].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&spellings.clone()) { let mut spelling: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &spelling.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![rank.clone(), name.clone(), sentence.clone(), why.clone(), contrast.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "append", args))(&__args) };
    }
    }
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__QUALITY_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&line.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _supermarket_quality(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_str("Aldi and Lidl are the cheapest of the big names for own-brand staples, around a fifth less than Tesco or Sainsbury's. Asda is usually the cheapest of the big four, Morrisons, Tesco and Sainsbury's come next and price-match Aldi on the basics, and Waitrose and M&S are the dearest. A convenience shop - the Co-op, a corner shop - costs more again, so it is for a top-up, not the weekly shop.");
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&named.clone(), "sort", args))(&__args) };
    if py_truthy(&op_ge(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2))) {
    let __unpk3 = py_unpack(&py_tuple(vec![geti(&named.clone(), &Py::Int(0)), geti(&named.clone(), &op_neg(&Py::Int(1)))]), 2, None);
    cheap = __unpk3[0].clone();
    dear = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&geti(&cheap.clone(), &Py::Int(0)), &geti(&dear.clone(), &Py::Int(0))))) {
    return op_modulo(&py_str("%s and %s are level on price, so pick on what you cook: %s %s, and %s %s. Go to whichever is nearer."), &py_tuple(vec![geti(&cheap.clone(), &Py::Int(1)), geti(&dear.clone(), &Py::Int(1)), geti(&cheap.clone(), &Py::Int(1)), geti(&cheap.clone(), &Py::Int(4)), geti(&dear.clone(), &Py::Int(1)), geti(&dear.clone(), &Py::Int(4))]));
    }
    return op_add(&op_modulo(&py_str("%s is the cheaper of the two: %s. %s %s."), &py_tuple(vec![geti(&cheap.clone(), &Py::Int(1)), geti(&cheap.clone(), &Py::Int(3)), geti(&dear.clone(), &Py::Int(1)), geti(&dear.clone(), &Py::Int(4))])), &if py_truthy(&op_ge(&geti(&cheap.clone(), &Py::Int(0)), &Py::Int(2))) { py_str(" Check the price per 100g on the shelf label rather than the pack size, because the price match only covers the basics.") } else { py_str("") });
    }
    let __unpk4 = py_unpack(&geti(&named.clone(), &Py::Int(0)), 5, None);
    rank = __unpk4[0].clone();
    name = __unpk4[1].clone();
    sentence = __unpk4[2].clone();
    why = __unpk4[3].clone();
    contrast = __unpk4[4].clone();
    tail = if py_truthy(&op_le(&rank.clone(), &Py::Int(1))) { g__CHEAPEST_TAIL() } else { if py_truthy(&Py::Bool(py_eq(&rank.clone(), &Py::Int(2)))) { g__MIDDLE_TAIL() } else { g__DEAR_TAIL() } };
    return op_modulo(&py_str("%s is %s: %s.%s"), &py_tuple(vec![name.clone(), sentence.clone(), why.clone(), tail.clone()]));
    Py::None
}
pub fn _fold(args: &[Py]) -> Py {
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
    let mut cut: Py = Py::None;
    let mut join: Py = Py::None;
    let _ = py_str("The word as the lists spell it, with a plural, a possessive or a contraction taken off.");
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PERSON_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_TASTE_WORDS(), &word.clone())) } }) {
    return word.clone();
    }
    let __iter1 = py_iter_or_raise(&g__FOLD());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    cut = __unpk2[0].clone();
    join = __unpk2[1].clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&word.clone(), "endswith", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&op_bitor(&g_PERSON_WORDS(), &g_TASTE_WORDS()), &op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), &join.clone()))) } }) {
    return op_add(&slice_or_raise(&word.clone(), &Py::None, &op_neg(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cut.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &Py::None), &join.clone());
    }
    }
    return word.clone();
    Py::None
}
pub fn _the_words(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("The words of a line as the person and taste lists spell them.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fold(args))(&__args) }); } py_set(__out) })() };
    Py::None
}
pub fn _said_line(args: &[Py]) -> Py {
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
    let _ = py_str("The line being answered, as text: the reading carries it, and an entry only gets the dict.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return py_str("");
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _knows_a_taste(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a line said who the person is and something they like, this one or a recent one.");
    let mut recent: Py = py_func("recent", |args: &[Py]| gen_memory::memory::recent(args));
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(8)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&recent, args))(&__args) }) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) };
    if py_truthy(&said.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&lines.clone());
    for __item in __iter1 {
    line = __item.clone();
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words(args))(&__args) };
    if py_truthy(&{ let __b = op_bitand(&words.clone(), &g_PERSON_WORDS()); if !py_truthy(&__b) { __b } else { op_bitand(&words.clone(), &g_TASTE_WORDS()) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _gift_cheap(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut word: Py = Py::None;
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_in_the_line(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::the_age_of_the_person_they_mean(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) {
    if py_truthy(&op_lt(&age.clone(), &Py::Int(13))) {
    return g_CHEAP_CHILD();
    }
    if py_truthy(&op_lt(&age.clone(), &Py::Int(20))) {
    return g_CHEAP_TEEN();
    }
    return g_CHEAP_ADULT();
    }
    clean = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } };
    let __iter1 = py_iter_or_raise(&g_CHILD_WORDS());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("(?<![a-z])"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("(?![a-z])"))); __args.push(clean.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_CHEAP_CHILD();
    }
    }
    return g_CHEAP_ADULT();
    Py::None
}
pub fn _the_tastes(args: &[Py]) -> Py {
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
    let mut candidate: Py = Py::None;
    let mut here: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The hobbies the chat named, as INTEREST_GIFTS spells them, newest line first.\n\n    The last few lines and the reading's own subject both count: \"shes into gardening\" a turn\n    before the ask is what makes \"what should i get her\" a question about a gardener, and a line\n    that names the hobby itself (\"a present for someone into fishing\") is the same fact.\n    ");
    let mut recent: Py = py_func("recent", |args: &[Py]| gen_memory::memory::recent(args));
    said = py_list(vec![]);
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(Py::Int(4)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&recent, args))(&__args) }) { let mut turn: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() };
    here = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) };
    if py_truthy(&here.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(here.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lines.clone(), "append", args))(&__args) };
    }
    let __iter1 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(lines.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("reversed", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter1 {
    turn = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    for __item in __iter2 {
    word = __item.clone();
    name = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _fold(args))(&__args) };
    let __iter3 = py_iter_or_raise(&py_tuple(vec![name.clone(), op_add(&name.clone(), &py_str("ing"))]));
    for __item in __iter3 {
    candidate = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_INTEREST_GIFTS(), &candidate.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&said.clone(), &candidate.clone())) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(candidate.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "append", args))(&__args) };
    }
    }
    }
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    return said.clone();
    Py::None
}
pub fn present_known(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut interests: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "interests") { v.clone() }
    else { py_tuple(vec![]) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut _u: Py = Py::None;
    let mut interest: Py = Py::None;
    let mut item: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut picks: Py = Py::None;
    let mut price: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("A present for a taste the chat already told us, with no price named.\n\n    The picks come from INTEREST_GIFTS, the same table the budget answers read, so one row here\n    covers every hobby in it instead of a row per hobby: the point of the answer is that it names\n    the thing they do rather than a safe list that would suit anyone. The sentence is written the\n    way the person rows are, with `about_them` sorting out he, she or they from the line.\n    ");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(interests.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _interest_phrase(args))(&__args) };
    picks = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = interests.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    interest = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(interest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INTEREST_GIFTS(), "get", args))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    item = __unpk3[0].clone();
    price = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&picks.clone()) { let __unpk7591 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk7591[0].clone(); let mut _u: Py = __unpk7591[1].clone(); __out.push(name.clone()); } py_list(__out) })() }, &item.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![item.clone(), price.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&picks.clone(), "append", args))(&__args) };
    }
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&picks.clone()))) {
    return Py::None;
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let interest = interest.clone(); let interests = interests.clone(); let item = item.clone(); let listed = listed.clone(); let memory = memory.clone(); let name = name.clone(); let named = named.clone(); let picks = picks.clone(); let price = price.clone(); let sentence = sentence.clone(); let understood = understood.clone(); let wanted = wanted.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let interest = interest.clone(); let interests = interests.clone(); let item = item.clone(); let listed = listed.clone(); let memory = memory.clone(); let name = name.clone(); let named = named.clone(); let picks = picks.clone(); let price = price.clone(); let sentence = sentence.clone(); let understood = understood.clone(); let wanted = wanted.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); geti(&pair.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&picks.clone(), "sort", args))(&__args) };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&picks.clone(), &Py::None, &Py::Int(3), &Py::None)) { let __unpk5049 = py_unpack(&__item.clone(), 2, None); let mut item: Py = __unpk5049[0].clone(); let mut price: Py = __unpk5049[1].clone(); __out.push(op_modulo(&py_str("%s (%s)"), &py_tuple(vec![item.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(price.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) }]))); } py_list(__out) })() };
    listed = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) { geti(&named.clone(), &Py::Int(0)) } else { op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&named.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(", "), "join", args))(&__args) }, &py_str(" or ")), &geti(&named.clone(), &op_neg(&Py::Int(1)))) };
    sentence = op_modulo(&py_str("For someone into %s: %s. Buy the one {they} will actually use rather than three {they} might, and put a real message in the card."), &py_tuple(vec![wanted.clone(), listed.clone()]));
    if py_truthy(&{ let __b = Py::Bool(py_is(&memory.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&understood.clone(), &Py::None)) } }) {
    return sentence.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push({ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(sentence.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::about_them(args))(&__args) }, args))(&__args) };
    Py::None
}
pub fn the_card_after_a_present(args: &[Py]) -> Py {
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
    let mut line: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "line") { v.clone() }
    else { Py::None };
    let mut said: Py = Py::None;
    let _ = py_str("The words for the card, when the turn after a present list asks about the card.\n\n    The line typed is read in preference to the reading, because on a follow-up the reading can\n    be the earlier line the turn leans on: \"and what about a card\" after a present ask arrives\n    here as \"what can i get them\", which names no card at all.\n    ");
    said = if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "strip", args))(&__args) } } }) { line.clone() } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) } };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CARD_IS_NOT_A_PRESENT(), "search", args))(&__args) })); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_NOT_THAT_CARD(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CARD_AFTER_A_PRESENT()); __args.push({ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) };
    Py::None
}
pub fn not_a_present(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line is asking what to give somebody, so a condition row stands down.\n\n    Public: the rows a condition word also reaches - the weekly alcohol limit, the vegetarian\n    dinner, the nut-free list - carry `guard=not_a_present`, because \"present for someone with a\n    nut allergy\" is a present and not a dinner.\n    ");
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::topics::gift_ask(args))(&__args) }));
    Py::None
}
pub fn _the_condition_in(args: &[Py]) -> Py {
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
    let mut drop: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut phrases: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("(the sentence to add, the phrases to take back out) for a condition named in this text.");
    text = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }), &py_str(" "));
    let __iter1 = py_iter_or_raise(&g_GIFT_FOR_A_CONDITION());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    phrases = __unpk2[0].clone();
    sentence = __unpk2[1].clone();
    drop = __unpk2[2].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&phrases.clone()) { let mut phrase: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &phrase.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_tuple(vec![sentence.clone(), drop.clone()]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_condition_gift(args: &[Py]) -> Py {
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
    let mut earlier: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut low: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("What the person's condition changes: the sentence to add, and what to take back out.\n\n    A present for somebody who has given up drinking must not go on to recommend a bottle, so the\n    table carries the phrases an answer says that the condition rules out.\n\n    The condition is often said a turn earlier - \"my mum has just given up drinking\", then \"what\n    should i get her for christmas\" - so the last two things the person said are read as well, but\n    only for a line that names nobody: a line naming its own person is that person's present.\n    ");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_condition_in(args))(&__args) };
    if py_truthy(&Py::Bool(py_is(&found.clone(), &Py::None))) {
    low = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "lower", args))(&__args) }), &py_str(" "));
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__PRESENT_PEOPLE()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str(" ")))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &op_add(&op_add(&py_str(" "), &word.clone()), &py_str("s ")))) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    turns = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("turns")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turns.clone()); __args.push(py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    earlier = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &op_neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut turn: Py = __item.clone(); if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(turn.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("user")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turn.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(earlier.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_condition_in(args))(&__args) };
    }
    }
    }
    return { let __b = found.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![py_str(""), py_tuple(vec![])]) } };
    Py::None
}
pub fn _present_answer(args: &[Py]) -> Py {
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
    let mut reply: Py = Py::None;
    let _ = py_str("A written present answer that still reads the condition the person was described by.\n\n    The rows for a named person (a dad, a mum) are one good paragraph each, and the condition is\n    in the line rather than in the taste table, so they pass through the same note the generic\n    rows do.\n    ");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut reply: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    Py::None
}
    reply = {{ let reply = reply.clone(); let text = text.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![reply.clone(), text.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _the_thing_a_verdict_is_about(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The present a verdict ask names, without the little words around it.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VERDICT_THING(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:(?:that|it|this)\\s+)+")); __args.push(py_str("")); __args.push(thing.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("some"), py_str("a gift"), py_str("a present"), py_str("that"), py_str("it"), py_str("this"), py_str("them"), py_str("they")]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) }))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z]{3}")); __args.push(thing.clone()); __args.push(stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return py_str("");
    }
    return thing.clone();
    Py::None
}
pub fn _the_only_age_in_the_store(args: &[Py]) -> Py {
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
    let mut ages: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let _ = py_str("The age of the one person the chat holds an age for, or None.\n\n    A verdict ask names no person - \"would a scooter be too much\" is about the present the last\n    answer was for - so the person is read off the chat instead: when the store holds the age of\n    exactly one person, that is who the present is for. Two ages and there is no saying which,\n    and the answer stands down rather than judging the wrong child's present.\n    ");
    ages = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("others")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entry.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entry.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::_with_an_age(args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(entry.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::_age(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ages.clone(), "append", args))(&__args) };
    }
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("their_age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::_with_an_age(args))(&__args) } } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::_age(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&ages.clone(), "append", args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ages.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&geti(&ages.clone(), &Py::Int(0)), &Py::None)) } }) {
    return geti(&ages.clone(), &Py::Int(0));
    }
    return Py::None;
    Py::None
}
pub fn _a_pick_for_that_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pool: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pool") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pool"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut low: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing named is one of the picks the age's row already puts on the table.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&pool.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&thing.clone())) } }) {
    return Py::Bool(false);
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:a|an|the|my|some)\\s+")); __args.push(py_str("")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "strip", args))(&__args) }); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z]+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&thing.clone(), "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_NOT_THE_THING(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&pool.clone(), "lower", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("s?\\b"))); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn the_verdict_on_a_present(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut line: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pool_of_words: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("A yes to a verdict ask about a pick the age row already listed, or None to stand down.");
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VERDICT_ON_A_PRESENT(), "search", args))(&__args) }))) {
    return Py::None;
    }
    thing = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_thing_a_verdict_is_about(args))(&__args) };
    held = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_in_the_line(args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::the_age_of_the_person_they_mean(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_only_age_in_the_store(args))(&__args) } } } } };
    if py_truthy(&Py::Bool(!py_truthy(&held.clone()))) {
    return Py::None;
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_for_an_age(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(thing.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_pick_for_that_age(args))(&__args) }))) {
    return Py::None;
    }
    age = if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } };
    if py_truthy(&op_lt(&held.clone(), &Py::Int(1))) {
    age = py_str("a baby");
    } else {
    if py_truthy(&Py::Bool(py_eq(&held.clone(), &Py::Int(1)))) {
    age = py_str("one");
    }
    }
    pool_of_words = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VERDICT_ABOUT_MONEY(), "search", args))(&__args) }) { g_MONEY_VERDICT_YES() } else { g_VERDICT_YES() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("age".to_string(), age.clone()), ("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool_of_words.clone()); __args.push(memory.clone()); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("compose", "pick", args))(&__args) }, "format", args))(&__args) };
    Py::None
}
pub fn _gift_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut fallback: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "fallback") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: fallback"))) };
    let mut reply: Py = Py::None;
    let _ = py_str("A generic gift answer that names the hobby when the chat has already said one.\n\n    Every \"what should i get her\" row ends the same way when it knows nothing: a safe list and a\n    note to buy into what they are into. When the chat has just said what they are into, the\n    picks for that hobby are the answer, and INTEREST_GIFTS already holds them, so the rows share\n    this instead of each learning the table.\n    ");
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut fallback: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(3).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut baby: Py = Py::None;
    let mut cheap: Py = Py::None;
    let mut picks: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut verdict: Py = Py::None;
    verdict = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_verdict_on_a_present(args))(&__args) };
    if py_truthy(&verdict.clone()) {
    return verdict.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_VERDICT_ON_A_PRESENT(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::topics::gift_ask(args))(&__args) }))) {
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CARD_IS_NOT_A_PRESENT(), "search", args))(&__args) }) {
    return Py::None;
    }
    return fallback.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRICE_SAID(), "search", args))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_A_CHEAP_ASK(), "search", args))(&__args) }) {
    cheap = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_cheap(args))(&__args) };
    return if py_truthy(&cheap.clone()) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(cheap.clone()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) } } else { Py::None };
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _knows_a_taste(args))(&__args) }))) {
    baby = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_newborn_present(args))(&__args) };
    if py_truthy(&baby.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(baby.clone()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    }
    pool = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_in_the_line(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::the_age_of_the_person_they_mean(args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_for_an_age(args))(&__args) };
    if py_truthy(&pool.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _only_a_them_says_who(args))(&__args) }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fallback.clone()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRICE_SAID(), "search", args))(&__args) }) {
    return Py::None;
    }
    picks = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_tastes(args))(&__args) }); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| present_known(args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = picks.clone(); if py_truthy(&__b) { __b } else { fallback.clone() } }); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    Py::None
}
    reply = {{ let fallback = fallback.clone(); let reply = reply.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![fallback.clone(), reply.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _only_a_them_says_who(args: &[Py]) -> Py {
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
    let _ = py_str("True when the only person the line names is \"them\", so the store is the whole answer.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PLURAL_ONLY(), "search", args))(&__args) }))) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&op_bitand(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words(args))(&__args) }, &op_sub(&g_PERSON_WORDS(), &g_PLURAL_WORDS()))));
    Py::None
}
pub fn _the_newborn_present(args: &[Py]) -> Py {
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
    let _ = py_str("The present for a baby the chat is already talking about, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::newborn::the_thread_has_a_new_baby(args))(&__args) }))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Float(0.5)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_for_an_age(args))(&__args) };
    Py::None
}
pub fn _with_the_condition(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: understood"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut answer: Py = Py::None;
    let mut drop: Py = Py::None;
    let mut note: Py = Py::None;
    let mut phrase: Py = Py::None;
    let _ = py_str("A present answer, plus what the person's condition changes about the pick.");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return text.clone();
    }
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_condition_gift(args))(&__args) }, 2, None);
    note = __unpk1[0].clone();
    drop = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&note.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&note.clone(), "lower", args))(&__args) })) } }) {
    return text.clone();
    }
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter2 = py_iter_or_raise(&drop.clone());
    for __item in __iter2 {
    phrase = __item.clone();
    answer = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); __args.push(py_str("")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "replace", args))(&__args) };
    }
    return op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&answer.clone(), "rstrip", args))(&__args) }, &py_str(" ")), &note.clone());
    Py::None
}
pub fn _gift_anyone(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut picks: Py = Py::None;
    let mut pool: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_understanding::understanding::topics::gift_ask(args))(&__args) }))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _knows_a_taste(args))(&__args) }) {
    picks = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_tastes(args))(&__args) }); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| present_known(args))(&__args) };
    if py_truthy(&picks.clone()) {
    return picks.clone();
    }
    return Py::None;
    }
    held = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::the_age_of_the_person_they_mean(args))(&__args) };
    pool = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(held.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_for_an_age(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_newborn_present(args))(&__args) } } };
    if py_truthy(&pool.clone()) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| gen_memory::memory::people::a_child_the_chat_is_about(args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CHILD_GIFT()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g__GIFT_ANYONE()); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_the_condition(args))(&__args) };
    Py::None
}
pub fn _laptop_general(args: &[Py]) -> Py {
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
    return py_str("Get 16GB of RAM and a 512GB SSD if the budget stretches, 8GB and 256GB if it doesn't, and a 13 or 14 inch screen for carrying about. Don't pay for a graphics card unless you game or edit video; a good screen and keyboard matter far more day to day.");
    Py::None
}
pub fn _openings(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pools: Py = py_list(__pos.get(0..).unwrap_or(&[]).to_vec());
    let mut out: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut pools: Py = Py::None;
    let mut row: Py = Py::None;
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&pools.clone());
    for __item in __iter1 {
    pool = __item.clone();
    let __iter2 = py_iter_or_raise(&if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(pool.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { py_tuple(vec![pool.clone()]) } else { pool.clone() });
    for __item in __iter2 {
    row = __item.clone();
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(row.clone()); __args.push(py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&row.clone(), "strip", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(20)) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&row.clone(), "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, &Py::None, &Py::Int(28), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&out.clone(), "append", args))(&__args) };
    }
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(out.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn a_present_was_answered(args: &[Py]) -> Py {
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
    let mut opening: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether the answer on the screen is one of the present lists, rather than talk around one.");
    let mut last_bot: Py = py_func("last_bot", |args: &[Py]| gen_memory::memory::last_bot(args));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&last_bot, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PRESENT_OPENINGS()) { let mut opening: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &opening.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _smart_meter_how(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__METER_FAULT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("not working"))) } }) {
    return py_str("Ring your supplier and say the meter or its display has stopped - it is their equipment, so fixing it is free, and it is usually a firmware update or a new display they send out. Photograph the reading on the meter itself before you ring, and send them a reading weekly until it is sorted, so you are not billed on an estimate in the meantime.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__METER_READ(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Press the button marked A (some meters say B) until the screen shows a number with kWh after it - that is your reading, and the one the supplier wants. On a smart meter the in-home display shows it too, and with a single-rate tariff it is the only number that matters; a two-rate one shows day and night separately, so send both.");
    }
    return py_str("");
    Py::None
}
pub fn _bare_decision(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut ask: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "ask") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: ask"))) };
    let mut phrase: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Whether the line asks about `name` and names no occasion of its own.");
    rest = said.clone();
    let __iter1 = py_iter_or_raise(&op_add(&py_tuple(vec![name.clone()]), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(ask.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }));
    for __item in __iter1 {
    phrase = __item.clone();
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&phrase.clone(), "strip", args))(&__args) };
    if py_truthy(&phrase.clone()) {
    rest = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(phrase.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(py_str(" ")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z0-9']+")); __args.push(rest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__DECISION_WORDS(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("all", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _longest_match_named(args: &[Py]) -> Py {
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
    let mut table: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: table"))) };
    let mut best: Py = Py::None;
    let mut found: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The longest name the line holds, and the reply it belongs to.");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, op_neg(&Py::Int(1)), py_str("")]), 3, None);
    best = __unpk1[0].clone();
    longest = __unpk1[1].clone();
    found = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&table.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    names = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    name = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &longest.clone()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![reply.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, name.clone()]), 3, None);
    best = __unpk5[0].clone();
    longest = __unpk5[1].clone();
    found = __unpk5[2].clone();
    }
    }
    }
    return py_tuple(vec![best.clone(), found.clone()]);
    Py::None
}
pub fn _longest_match(args: &[Py]) -> Py {
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
    let mut table: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "table") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: table"))) };
    let mut best: Py = Py::None;
    let mut longest: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The row whose name covers the most of the line: \"steam cleaner\" is not \"a cleaner\".");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, op_neg(&Py::Int(1))]), 2, None);
    best = __unpk1[0].clone();
    longest = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&table.clone());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    names = __unpk3[0].clone();
    reply = __unpk3[1].clone();
    let __iter4 = py_iter_or_raise(&names.clone());
    for __item in __iter4 {
    name = __item.clone();
    name = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&name.clone(), "strip", args))(&__args) };
    if py_truthy(&{ let __b = op_gt(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &longest.clone()); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }), &py_str("\\b"))); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![reply.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    best = __unpk5[0].clone();
    longest = __unpk5[1].clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _haggle(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The price question, said to a shop or said to a trader.\n\n    A shop has a price tag and a margin, so the ask is one question, the display model and the\n    readiness to walk away. A trader has a quote and a diary, so it is the materials, the dates\n    and the deposit. STYLE asks for the answer first, and it is not the same sentence for both.\n    ");
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { understood.clone() } else { py_dict(vec![]) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }])) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\brude\\b|\\bawkward\\b|\\bembarrass")); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("No - asking once is normal in a market, a car boot sale, a small independent shop or on anything big like a sofa, and in a chain with price tags it is still fine to ask. One question, then take the answer and either buy or leave: asking twice or arguing after a no is the part that gets awkward.");
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__TRADER_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return py_str("Get three written quotes first, then ask each trader the same question: what would bring that down? Supplying the materials, being flexible on dates so the job fills a gap in their diary, and paying a deposit up front are the three that work. Compare the quotes line by line rather than on the total, and treat one miles under the others as a warning: the extras come later.");
    }
    return py_str("Ask once, cheerfully, once you have picked the thing - \"is there anything you can do on the price?\" - then stop talking and be ready to walk away: silence and a customer who will leave are what move a price. Take the display model or last season's colour, offer to pay in full today, check the online price so they can match it, and never say your maximum. At a market or car boot sale, offer about two thirds and settle in the middle.");
    Py::None
}
pub fn _choice_value(args: &[Py]) -> Py {
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
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut part: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut row: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The maths for the bigger thing the line names, or None when it names none of these.");
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { understood.clone() } else { py_dict(vec![]) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }])) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    let __unpk1 = py_unpack(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_CHOICE_VALUE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _longest_match_named(args))(&__args) }, 2, None);
    row = __unpk1[0].clone();
    name = __unpk1[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&row.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__ACTION_NAME(), "search", args))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(name.clone()); __args.push(g_APPLIANCE_ASK()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _bare_decision(args))(&__args) })) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_is(&row.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DRYER_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    let __iter2 = py_iter_or_raise(&g_CHOICE_VALUE());
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    names = __unpk3[0].clone();
    answer = __unpk3[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&name.clone(), &py_str("dryer"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&name.clone(), &py_str("drier"))) } }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return answer.clone();
    }
    }
    }
    return row.clone();
    Py::None
}
pub fn _appliance_value(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The trade for the appliance the line names, or None when it names none of these.");
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { understood.clone() } else { py_dict(vec![]) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }])) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_APPLIANCE_VALUE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _longest_match(args))(&__args) };
    Py::None
}
pub fn _everyday_where(args: &[Py]) -> Py {
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
    let mut inside: Py = Py::None;
    let mut part: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Where a small everyday thing is bought, or None when the line names none of them.\n\n    \"where can i buy a stamp\" is a where question with a short answer - a post office counter -\n    and the price-comparison advice is written for something with a model, a warranty and a\n    \"was £500\" tag, which a stamp, a card and a sleeping bag do not have. One table, read by one\n    row, so a new everyday thing is a line here rather than a row of its own.\n    ");
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { understood.clone() } else { py_dict(vec![]) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }])) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    inside = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__INSIDE(), "search", args))(&__args) };
    if py_truthy(&{ let __b = inside.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g__SHOP_PLACES(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&inside.clone(), "group", args))(&__args) })) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "split", args))(&__args) }, &py_str("find"))); if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__MINE(), "search", args))(&__args) } } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(g_EVERYDAY_WHERE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _longest_match(args))(&__args) };
    Py::None
}
pub fn _gift_for_an_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut age: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let _ = py_str("The present pool for an age in years, or None when there is no age to go on.");
    if py_truthy(&Py::Bool(py_is(&age.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&op_lt(&age.clone(), &Py::Int(1))) {
    return py_str("Under a year, buy the boring things the parents will use: clothes in the size for three to six months ahead, a pack of muslins, a board book, or a soft ball. Skip anything that needs batteries or a cot to hang it from, and skip newborn sizes, because everyone buys those and the baby grows out of them in a fortnight.");
    }
    if py_truthy(&op_ge(&age.clone(), &Py::Int(20))) {
    return g_GROWN_GIFT();
    }
    if py_truthy(&op_ge(&age.clone(), &Py::Int(13))) {
    return g_TEEN_GIFT();
    }
    return if py_truthy(&op_le(&age.clone(), &Py::Int(2))) { g_TODDLER_GIFT() } else { g_CHILD_GIFT() };
    Py::None
}
pub fn _an_age_in_the_line(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut number: Py = Py::None;
    let mut turning: Py = Py::None;
    let _ = py_str("The age the line itself says, in years, or None.\n\n    A baby is asked for in months (\"what do you get a 6 month old\"), and \"for her 9th birthday\"\n    says the age in the day as plainly as \"a 9 year old\" does.\n    ");
    let __iter1 = py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numbers")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    number = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) }, &py_str("months")))) {
    return op_truediv(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) }, &Py::Float(12.0));
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unit")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) }, &py_str("years")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) };
    }
    }
    line = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }, "split", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    turning = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_TURNING(), "search", args))(&__args) };
    if py_truthy(&turning.clone()) {
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("age")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&turning.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = { let __cmp21: Py = age.clone(); if py_truthy(&op_le(&Py::Int(0), &__cmp21)) { op_le(&__cmp21, &Py::Int(120)) } else { Py::Bool(false) } }; if !py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("numbers")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut number: Py = __item.clone(); __out.push(Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("value")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&number.clone(), "get", args))(&__args) }, &age.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } }) {
    return age.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_child_is_the_one_they_buy_for(args: &[Py]) -> Py {
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
    let _ = py_str("True when the person the present is for is a child, with no age in the line.\n\n    The by-age row answers a line that says the age, and the person row answered every other\n    line with the grown-up list - so \"what should i get my nephew\", \"a present for my grandson\"\n    and \"what should i get my child\" were offered good coffee, a mug and woolly socks. The\n    person is enough to know that list is wrong; only the age can choose the right one, so this\n    decides which of the two answers the person row gives, not whether it answers.\n    ");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _said_line(args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_words(args))(&__args) };
    if py_truthy(&op_bitand(&words.clone(), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CHILD_PEOPLE()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) {
    return Py::Bool(true);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_bitand(&words.clone(), &py_set(vec![py_str("teenager"), py_str("teenagers"), py_str("teen"), py_str("teens"), py_str("teenage"), py_str("schoolkid"), py_str("schoolkids"), py_str("schoolboy"), py_str("schoolgirl")]))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _the_person_row(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut adult: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "adult") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: adult"))) };
    let mut child: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "child") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: child"))) };
    let mut grown_up: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut young: Py = Py::None;
    let _ = py_str("The person row, with the answer a child's present gets instead of the grown-up list.\n\n    Both answers keep everything else the row does - the hobby the chat has named, the age the\n    store already holds, the price and the cheap ask - because those are what make it a good\n    answer; only the last-resort list is swapped.\n    ");
    grown_up = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(adult.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_row(args))(&__args) };
    young = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(child.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_row(args))(&__args) };
pub fn __nested_reply_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut adult: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut child: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut grown_up: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut young: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "memory").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut understood: Py = __pos.get(6).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "understood").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut chosen: Py = Py::None;
    chosen = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_child_is_the_one_they_buy_for(args))(&__args) }) { young.clone() } else { grown_up.clone() };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&chosen, args))(&__args) };
    Py::None
}
    reply = {{ let adult = adult.clone(); let child = child.clone(); let grown_up = grown_up.clone(); let reply = reply.clone(); let young = young.clone(); py_func("reply", move |args: &[Py]| { let mut full = vec![adult.clone(), child.clone(), grown_up.clone(), reply.clone(), young.clone()]; full.extend_from_slice(args); __nested_reply_0(&full) }) }};
    return reply.clone();
    Py::None
}
pub fn _an_age_given(args: &[Py]) -> Py {
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
    let _ = py_str("The number of years a matched age says, counting a word as a number.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&said.clone(), "isdigit", args))(&__args) }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__SAID_AS_WORDS(), "get", args))(&__args) };
    Py::None
}
pub fn _an_age_said_in_the_line(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut found: Py = Py::None;
    let mut months: Py = Py::None;
    let mut name: Py = Py::None;
    let _ = py_str("The age a line gives, as a number of years, or None when it gives none.");
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__AN_AGE_SAID(), "search", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("months")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }) {
    months = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("months")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_given(args))(&__args) };
    return if py_truthy(&months.clone()) { op_truediv(&months.clone(), &Py::Float(12.0)) } else { Py::None };
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("years"), py_str("near"), py_str("ordinal")]));
    for __item in __iter1 {
    name = __item.clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }) {
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(name.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&found.clone(), "group", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_given(args))(&__args) };
    return if py_truthy(&Py::Bool(!py_is(&age.clone(), &Py::None))) { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(age.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) } } else { Py::None };
    }
    }
    return Py::None;
    Py::None
}
pub fn _names_a_childs_age(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("True when the line gives an age at all, however it says it.\n\n    \"4 yr old\", \"hes 4\", \"his 5th birthday\", \"aged 8\", \"a two year old\" and \"6 months old\" are\n    one fact, and it is the fact that decides which present pool answers. The row keyed only on\n    the literal \"4 year old\", so every other shape fell through to the grown-up list, which\n    offers a four-year-old good coffee and woolly socks. A grown-up named by an ordinal birthday\n    (\"her 60th\", \"my son for his 30th\") does not match, so the person row keeps those.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    line = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CHILDS_AGE(), "search", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("bool", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _this_row_answers_the_line(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut named: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("True when the age row is the one to answer, and not the person row standing beside it.\n\n    Two questions look alike and are not: \"what should i get my niece, shes 6\" is an age\n    question, and \"what should i get my niece for her birthday\" is not - the child list the\n    person row already holds is the right answer there, and an age pool would be a guess. So a\n    line that names a child and gives no age stands this row down.\n\n    A line that gives no figure and names no child stands it down on nothing: that is the turn\n    after this row answered (\"he likes lego and football\", \"budget 30 quid\"), and the answer\n    the person already got carries on from it. A guard that asked again for an age refused\n    that turn and the chat fell to a neighbour instead - the adult-sport card on \"football\",\n    the budget bands on \"40\" - so the memory keeping this row's keys is what makes the narrow\n    guard the only safe one.\n    ");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(line.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    line = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } };
    }
    text = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::Bool(false);
    }
    named = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_child_is_named(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__A_CHILDS_AGE(), "search", args))(&__args) }))) {
    return Py::Bool(!py_truthy(&named.clone()));
    }
    age = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_said_in_the_line(args))(&__args) };
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&age.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_gt(&age.clone(), &Py::Int(19)) } } } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _a_child_is_named(args: &[Py]) -> Py {
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
    let mut person: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line names a child of someone (see CHILD_PEOPLE).");
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("[a-z']+")); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "findall", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    let __iter1 = py_iter_or_raise(&g_CHILD_PEOPLE());
    for __item in __iter1 {
    person = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&person.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &person.clone()))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &person.clone()))) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _gift_by_age(args: &[Py]) -> Py {
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
    let mut chosen: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("A present for a named age: a toddler, a teenager and a 60-year-old want different things.\n\n    The row used to answer every \"for a X year old\" with scooters and duplo. The age is already\n    read off the line, so one entry can pick the right pool instead of treating a grown-up\n    birthday as a child's.\n    ");
    low = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    chosen = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_in_the_line(args))(&__args) }; if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _an_age_said_in_the_line(args))(&__args) } } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_for_an_age(args))(&__args) };
    if py_truthy(&chosen.clone()) {
    return chosen.clone();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:toddler|toddlers|preschool|preschooler|baby|babies)\\b")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_TODDLER_GIFT();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\b(?:teen|teens|teenager|teenagers|teenage)\\b")); __args.push(low.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g_TEEN_GIFT();
    }
    return g_CHILD_GIFT();
    Py::None
}
pub fn _the_age_row(args: &[Py]) -> Py {
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
    let mut card: Py = Py::None;
    let _ = py_str("The age row, and the card the next turn asks about instead of another present.");
    card = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| the_card_after_a_present(args))(&__args) };
    if py_truthy(&card.clone()) {
    return card.clone();
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _gift_by_age(args))(&__args) };
    Py::None
}
pub fn _watch_help(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A wristwatch is not a film, and a smartwatch is a different question from a first watch.");
    forms = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("clean"), py_str("norm_text"), py_str("norm")])) { let mut key: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(key.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = understood.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(forms.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) };
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WATCH_SMART()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return g__WATCH_SMART_SAY();
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__WATCH_VERBS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&forms.clone()) { let mut form: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(form.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__WATCH_VERB_SHAPE(), "search", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::None;
    }
    return g__WATCH_PLAIN();
    Py::None
}
pub fn _money(args: &[Py]) -> Py {
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
    let _ = py_str("A price the way people write it: whole pounds stay whole.");
    let __try1 = exc::catch(|| {
    amount = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(amount.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("float", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("round", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
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
    return py_str("");
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    return op_modulo(&py_str("£%d"), &amount.clone());
    Py::None
}
pub fn _interest_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut interests: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "interests") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: interests"))) };
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"fishing\", \"fishing and gardening\", \"fishing, gardening and baking\" - and no he or she.");
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&interests.clone()) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&named.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(named.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(1)))) {
    return geti(&named.clone(), &Py::Int(0));
    }
    return op_add(&op_add(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(slice_or_raise(&named.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(", "), "join", args))(&__args) }, &py_str(" and ")), &geti(&named.clone(), &op_neg(&Py::Int(1))));
    Py::None
}
pub fn present_within(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut money: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "money") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: money"))) };
    let mut interests: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "interests") { v.clone() }
    else { py_tuple(vec![]) };
    let mut advice: Py = Py::None;
    let mut band: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut interest: Py = Py::None;
    let mut item: Py = Py::None;
    let mut listed: Py = Py::None;
    let mut price: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("A present at a price: the picks that fit under it, or one good thing at that money.\n\n    The interest comes from the conversation (\"he likes fishing\" a turn earlier), so the limit is\n    answered about the same person the earlier answer was about, and it is never said back with a\n    pronoun that might be wrong.\n    ");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(interests.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _interest_phrase(args))(&__args) };
    fits = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = interests.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    interest = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(interest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INTEREST_GIFTS(), "get", args))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    item = __unpk3[0].clone();
    price = __unpk3[1].clone();
    if py_truthy(&{ let __b = op_le(&price.clone(), &money.clone()); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&fits.clone(), &py_tuple(vec![item.clone(), price.clone()]))) } }) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![item.clone(), price.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fits.clone(), "append", args))(&__args) };
    }
    }
    }
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let advice = advice.clone(); let band = band.clone(); let fits = fits.clone(); let interest = interest.clone(); let interests = interests.clone(); let item = item.clone(); let listed = listed.clone(); let money = money.clone(); let price = price.clone(); let wanted = wanted.clone(); py_func("lambda", move |args: &[Py]| { let advice = advice.clone(); let band = band.clone(); let fits = fits.clone(); let interest = interest.clone(); let interests = interests.clone(); let item = item.clone(); let listed = listed.clone(); let money = money.clone(); let price = price.clone(); let wanted = wanted.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); op_neg(&geti(&pair.clone(), &Py::Int(1))) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fits.clone(), "sort", args))(&__args) };
    if py_truthy(&fits.clone()) {
    listed = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&fits.clone(), &Py::None, &Py::Int(2), &Py::None)) { let __unpk9935 = py_unpack(&__item.clone(), 2, None); let mut item: Py = __unpk9935[0].clone(); let mut price: Py = __unpk9935[1].clone(); __out.push(op_modulo(&py_str("%s (%s)"), &py_tuple(vec![item.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(price.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) }]))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(", "), "join", args))(&__args) };
    if py_truthy(&{ let __b = op_ge(&money.clone(), &Py::Int(40)); if !py_truthy(&__b) { __b } else { op_lt(&geti(&geti(&fits.clone(), &Py::Int(0)), &Py::Int(1)), &op_truediv(&money.clone(), &Py::Int(2))) } }) {
    return op_modulo(&py_str("%s is plenty for someone into %s: %s, and the rest towards something they have been putting off."), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(money.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) }, wanted.clone(), listed.clone()]));
    }
    return op_modulo(&py_str("%s or less, for someone into %s: %s. Buy the one thing they will actually use rather than three they might, and put a real message in the card."), &py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(money.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) }, wanted.clone(), listed.clone()]));
    }
    if py_truthy(&Py::Bool(!py_truthy(&money.clone()))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("money".to_string(), py_str("that"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRESENT_OVER(), "format", args))(&__args) };
    }
    let __iter4 = py_iter_or_raise(&g_PRESENT_BANDS());
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    band = __unpk5[0].clone();
    advice = __unpk5[1].clone();
    if py_truthy(&op_le(&money.clone(), &band.clone())) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("money".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(money.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&advice.clone(), "format", args))(&__args) }); __args.push(interests.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_buy_into(args))(&__args) };
    }
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("money".to_string(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(money.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PRESENT_OVER(), "format", args))(&__args) }); __args.push(interests.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _with_buy_into(args))(&__args) };
    Py::None
}
pub fn _with_buy_into(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut advice: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "advice") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: advice"))) };
    let mut interests: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "interests") { v.clone() }
    else { py_tuple(vec![]) };
    let _ = py_str("The band, plus the one line a band cannot know: buy into what they are already into.");
    return if py_truthy(&interests.clone()) { advice.clone() } else { op_add(&advice.clone(), &g_BUY_INTO()) };
    Py::None
}
pub fn present_less(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut interests: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "interests") { v.clone() }
    else { py_tuple(vec![]) };
    let mut _u: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut interest: Py = Py::None;
    let mut item: Py = Py::None;
    let mut name: Py = Py::None;
    let mut price: Py = Py::None;
    let mut wanted: Py = Py::None;
    let _ = py_str("\"Anything cheaper?\": the dearest pick under a tenner, or the small-things band.");
    wanted = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(interests.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _interest_phrase(args))(&__args) };
    fits = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = interests.clone(); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    interest = __item.clone();
    let __iter2 = py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(interest.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); __args.push(py_tuple(vec![])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_INTEREST_GIFTS(), "get", args))(&__args) });
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    item = __unpk3[0].clone();
    price = __unpk3[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&fits.clone()) { let __unpk2917 = py_unpack(&__item.clone(), 2, None); let mut name: Py = __unpk2917[0].clone(); let mut _u: Py = __unpk2917[1].clone(); __out.push(name.clone()); } py_list(__out) })() }, &item.clone()))) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_tuple(vec![item.clone(), price.clone()])); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fits.clone(), "append", args))(&__args) };
    }
    }
    }
    if py_truthy(&fits.clone()) {
    let _ = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("key".to_string(), { let _u = _u.clone(); let fits = fits.clone(); let interest = interest.clone(); let interests = interests.clone(); let item = item.clone(); let name = name.clone(); let price = price.clone(); let wanted = wanted.clone(); py_func("lambda", move |args: &[Py]| { let _u = _u.clone(); let fits = fits.clone(); let interest = interest.clone(); let interests = interests.clone(); let item = item.clone(); let name = name.clone(); let price = price.clone(); let wanted = wanted.clone(); let pair: Py = __kwp(args, 0, "pair").unwrap_or(Py::None); geti(&pair.clone(), &Py::Int(1)) }) })]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&fits.clone(), "sort", args))(&__args) };
    let __unpk4 = py_unpack(&geti(&fits.clone(), &Py::Int(0)), 2, None);
    item = __unpk4[0].clone();
    price = __unpk4[1].clone();
    return op_modulo(&py_str("Cheaper than that: %s (%s) is the small version of the same idea for someone into %s, and a good one beats something bigger and cheaper."), &py_tuple(vec![item.clone(), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(price.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _money(args))(&__args) }, wanted.clone()]));
    }
    return py_str("Cheaper than that: go for the small version of the same idea - a good bar of chocolate, a decent paperback, nice soap, or a packet of good coffee - and let the card do the rest.");
    Py::None
}
pub fn _coat_to_buy(args: &[Py]) -> Py {
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
    let mut feet: Py = Py::None;
    let mut generic: Py = Py::None;
    let mut high: Py = Py::None;
    let mut inches: Py = Py::None;
    let mut low: Py = Py::None;
    let mut part: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tall: Py = Py::None;
    let mut total: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The coat for the weather the line names, or the choice between them when it names none.");
    reading = if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { understood.clone() } else { py_dict(vec![]) };
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&gen_understanding::understanding::topics::g_get(), args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }])) { let mut part: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&py_str(" "), "join", args))(&__args) }, "lower", args))(&__args) };
    tall = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__HEIGHT(), "search", args))(&__args) };
    if py_truthy(&tall.clone()) {
    let __unpk1 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(1)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tall.clone(), "group", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(Py::Int(2)); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&tall.clone(), "group", args))(&__args) }]), 2, None);
    feet = __unpk1[0].clone();
    inches = __unpk1[1].clone();
    total = op_add(&op_mul(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(feet.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(12)), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(inches.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    let __unpk2 = py_unpack(&py_tuple(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&total.clone(), &Py::Float(0.42))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_mul(&total.clone(), &Py::Float(0.5))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]), 2, None);
    low = __unpk2[0].clone();
    high = __unpk2[1].clone();
    return op_modulo(&py_str("At %s foot %s, look for a coat about %d to %d inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own."), &py_tuple(vec![feet.clone(), inches.clone(), low.clone(), high.clone()]));
    }
    let __iter3 = py_iter_or_raise(&g__COAT_KINDS());
    for __item in __iter3 {
    let __unpk4 = py_unpack(&__item.clone(), 2, None);
    words = __unpk4[0].clone();
    reply = __unpk4[1].clone();
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return reply.clone();
    }
    }
    generic = py_str("Pick the coat by the weather you are actually out in: a waterproof shell for rain, down or padded for cold and dry, wool for town. £80 to £150 buys a padded coat that lasts a decade, £150 to £250 a waterproof shell that does not leak at the shoulders.");
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _mentions_coat_check(args))(&__args) }) {
    return if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _coat_said_already(args))(&__args) }) { g__COAT_CHECK() } else { op_add(&op_add(&generic.clone(), &py_str(" ")), &g__COAT_CHECK()) };
    }
    return generic.clone();
    Py::None
}
pub fn _coat_said_already(args: &[Py]) -> Py {
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
    let _ = py_str("True when the coat choice has already been given in this conversation.");
    let mut last_bot: Py = py_func("last_bot", |args: &[Py]| gen_memory::memory::last_bot(args));
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&last_bot, args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) };
    return { let __b = Py::Bool(py_contains(&said.clone(), &py_str("buys a padded coat"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&said.clone(), &py_str("pick the coat by the weather"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("waterproof shell with taped seams"))) } } } };
    Py::None
}
pub fn _nobody_named_by_pronoun(args: &[Py]) -> Py {
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
    let _ = py_str("False when the line names the person by pronoun, so the pronoun rows keep their line.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(true);
    }
    words = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("words")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("norm")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_tuple(vec![]) } } } };
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, "lower", args))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__NAMED_BY_PRONOUN(), "intersection", args))(&__args) }));
    Py::None
}
pub fn _mentions_coat_check(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__COAT_CHECK_WORDS()) { let mut word: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(op_add(&py_str("\\b"), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "escape", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "search", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    Py::None
}
pub fn _not_a_length(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line is about keeping papers for a length or for tax, which is another card.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); __args.push(py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("isinstance", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }))) {
    return Py::Bool(true);
    }
    said = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&op_modulo(&py_str("%s %s"), &py_tuple(vec![{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }, { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } }])), "lower", args))(&__args) };
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g__KEEP_RECEIPT_RE(), "search", args))(&__args) }));
    Py::None
}
pub fn _the_bag_is_not_packaging(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is asking about the bag rather than about the food in it.\n\n    \"Why does bread go mouldy in a plastic bag\" names a plastic bag and a loaf, and the loaf is\n    the question. A line that does name a keeping question about a food is the food's, so this\n    row stands down for it.\n    ");
    return Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(understood.clone()); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::knowledge::lookup::a_food_is_being_kept(args))(&__args) }));
    Py::None
}
pub fn _the_place_not_the_shop(args: &[Py]) -> Py {
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
    let mut going: Py = Py::None;
    let mut place: Py = Py::None;
    let mut price: Py = Py::None;
    let mut said: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A line about going to a place is not a question about the shop of the same name.\n\n    \"what should i pack for iceland\", \"packing for a trip to iceland\" and \"what should i pack for\n    iceland in october\" were all answered \"Iceland is cheap for frozen food ... one of the\n    cheapest places to do a full shop\": the country, read as the shop. The row keys on the shop\n    names in the table and asks with \"should i\", so a country in the line was enough, and Iceland\n    is the one name in the table that is also a place people travel to. A line that says what is\n    being taken, or that asks about the trip itself, is the travel lanes' question and this row\n    stands down. One that names a price or a shop as well keeps the row, because \"is iceland\n    cheaper than asda\" and \"is iceland as good as farmfoods\" are about the shop.\n    ");
    said = op_add(&op_add(&py_str(" "), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("clean")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("text")); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&understood.clone(), "get", args))(&__args) }; if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", args))(&__args) }), &py_str(" "));
    going = py_tuple(vec![py_str(" pack"), py_str(" packing"), py_str(" trip"), py_str(" travel"), py_str(" travelling"), py_str(" traveling"), py_str(" flight"), py_str(" fly"), py_str(" holiday"), py_str(" holidays"), py_str(" visit"), py_str(" visiting"), py_str(" hotel"), py_str(" weather"), py_str(" things to do"), py_str(" airport"), py_str(" tourist"), py_str(" sightseeing"), py_str(" northern lights"), py_str(" what to wear"), py_str(" what to take"), py_str(" what to see"), py_str(" in october"), py_str(" in november"), py_str(" in december"), py_str(" in january"), py_str(" in february"), py_str(" in march")]);
    place = py_tuple(vec![py_str(" with kids"), py_str(" with children"), py_str(" with a family"), py_str(" with the family"), py_str(" with a baby"), py_str(" how many days"), py_str(" how long"), py_str(" best time to"), py_str(" when to go"), py_str(" worth visiting"), py_str(" things to do"), py_str(" it worth going"), py_str(" reykjavik"), py_str(" blue lagoon"), py_str(" golden circle"), py_str(" whale watching"), py_str(" northern lights"), py_str(" hire car"), py_str(" car hire"), py_str(" road trip"), py_str(" visa"), py_str(" what to pack"), py_str(" what to wear"), py_str(" is it safe"), py_str(" safe to")]);
    price = py_tuple(vec![py_str(" cheap"), py_str(" cheaper"), py_str(" price"), py_str(" prices"), py_str(" quality"), py_str(" fresh"), py_str(" value"), py_str(" compare"), py_str(" comparison"), py_str(" expensive"), py_str(" basket"), py_str(" groceries"), py_str(" supermarket"), py_str(" shop")]);
    if py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&place.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&going.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&price.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&said.clone(), &word.clone()))); } py_list(__out) })() }); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("any", args).unwrap_or_else(|e| exc::raise(e)))(&__args) })) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.shopping".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "AGE_ASK" => Some(g_AGE_ASK()),
            "ANSWERS" => Some(g_ANSWERS()),
            "APPLIANCE_ASK" => Some(g_APPLIANCE_ASK()),
            "APPLIANCE_VALUE" => Some(g_APPLIANCE_VALUE()),
            "A_CARD_IS_NOT_A_PRESENT" => Some(g_A_CARD_IS_NOT_A_PRESENT()),
            "A_CHEAP_ASK" => Some(g_A_CHEAP_ASK()),
            "A_VERDICT_ABOUT_MONEY" => Some(g_A_VERDICT_ABOUT_MONEY()),
            "A_VERDICT_ON_A_PRESENT" => Some(g_A_VERDICT_ON_A_PRESENT()),
            "A_VERDICT_THING" => Some(g_A_VERDICT_THING()),
            "A_VERDICT_WORD" => Some(g_A_VERDICT_WORD()),
            "BUY_INTO" => Some(g_BUY_INTO()),
            "CARD_AFTER_A_PRESENT" => Some(g_CARD_AFTER_A_PRESENT()),
            "CHEAP_ADULT" => Some(g_CHEAP_ADULT()),
            "CHEAP_CHILD" => Some(g_CHEAP_CHILD()),
            "CHEAP_TEEN" => Some(g_CHEAP_TEEN()),
            "CHILD_GIFT" => Some(g_CHILD_GIFT()),
            "CHILD_PEOPLE" => Some(g_CHILD_PEOPLE()),
            "CHILD_PRESENT" => Some(g_CHILD_PRESENT()),
            "CHILD_WORDS" => Some(g_CHILD_WORDS()),
            "CHOICE_VALUE" => Some(g_CHOICE_VALUE()),
            "CHOOSING" => Some(g_CHOOSING()),
            "CRISIS" => Some(g_CRISIS()),
            "ENTRIES" => Some(g_ENTRIES()),
            "EVERYDAY_WHERE" => Some(g_EVERYDAY_WHERE()),
            "GIFT_ASK" => Some(g_GIFT_ASK()),
            "GIFT_FOR_A_CONDITION" => Some(g_GIFT_FOR_A_CONDITION()),
            "GROWN_GIFT" => Some(g_GROWN_GIFT()),
            "INTEREST_GIFTS" => Some(g_INTEREST_GIFTS()),
            "KITCHEN_MACHINE_NAMES" => Some(g_KITCHEN_MACHINE_NAMES()),
            "KITCHEN_MACHINE_VALUE" => Some(g_KITCHEN_MACHINE_VALUE()),
            "LOSS" => Some(g_LOSS()),
            "MONEY_VERDICT_YES" => Some(g_MONEY_VERDICT_YES()),
            "NOT_THAT_CARD" => Some(g_NOT_THAT_CARD()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "PERSON_ASK" => Some(g_PERSON_ASK()),
            "PERSON_WORDS" => Some(g_PERSON_WORDS()),
            "PLURAL_ONLY" => Some(g_PLURAL_ONLY()),
            "PLURAL_WORDS" => Some(g_PLURAL_WORDS()),
            "PRESENT_ASK" => Some(g_PRESENT_ASK()),
            "PRESENT_BANDS" => Some(g_PRESENT_BANDS()),
            "PRESENT_OPENINGS" => Some(g_PRESENT_OPENINGS()),
            "PRESENT_OVER" => Some(g_PRESENT_OVER()),
            "PRICE_COMPARED" => Some(g_PRICE_COMPARED()),
            "PRICE_SAID" => Some(g_PRICE_SAID()),
            "SUPERMARKETS" => Some(g_SUPERMARKETS()),
            "TASTE_WORDS" => Some(g_TASTE_WORDS()),
            "TEEN_GIFT" => Some(g_TEEN_GIFT()),
            "TODDLER_GIFT" => Some(g_TODDLER_GIFT()),
            "TURNING" => Some(g_TURNING()),
            "VERDICT_YES" => Some(g_VERDICT_YES()),
            "_ACTION_NAME" => Some(g__ACTION_NAME()),
            "_AGE_WORDS" => Some(g__AGE_WORDS()),
            "_AN_AGE" => Some(g__AN_AGE()),
            "_AN_AGE_SAID" => Some(g__AN_AGE_SAID()),
            "_A_CHILDS_AGE" => Some(g__A_CHILDS_AGE()),
            "_CHEAPEST_TAIL" => Some(g__CHEAPEST_TAIL()),
            "_CHOOSE_ASK" => Some(g__CHOOSE_ASK()),
            "_COAT_CHECK" => Some(g__COAT_CHECK()),
            "_COAT_CHECK_WORDS" => Some(g__COAT_CHECK_WORDS()),
            "_COAT_KINDS" => Some(g__COAT_KINDS()),
            "_DEAR_TAIL" => Some(g__DEAR_TAIL()),
            "_DECISION_WORDS" => Some(g__DECISION_WORDS()),
            "_DRYER_WORDS" => Some(g__DRYER_WORDS()),
            "_FOLD" => Some(g__FOLD()),
            "_GIFT_ANYONE" => Some(g__GIFT_ANYONE()),
            "_HEIGHT" => Some(g__HEIGHT()),
            "_INSIDE" => Some(g__INSIDE()),
            "_KEEP_RECEIPT_RE" => Some(g__KEEP_RECEIPT_RE()),
            "_METER_FAULT" => Some(g__METER_FAULT()),
            "_METER_READ" => Some(g__METER_READ()),
            "_MIDDLE_TAIL" => Some(g__MIDDLE_TAIL()),
            "_MINE" => Some(g__MINE()),
            "_NAMED_BY_PRONOUN" => Some(g__NAMED_BY_PRONOUN()),
            "_PRESENT_PEOPLE" => Some(g__PRESENT_PEOPLE()),
            "_QUALITY_WORDS" => Some(g__QUALITY_WORDS()),
            "_RANGE_WORDS" => Some(g__RANGE_WORDS()),
            "_SAID_AS_WORDS" => Some(g__SAID_AS_WORDS()),
            "_SERVICE_WORDS" => Some(g__SERVICE_WORDS()),
            "_SHOP_ASK" => Some(g__SHOP_ASK()),
            "_SHOP_PLACES" => Some(g__SHOP_PLACES()),
            "_THE_EVERYDAY_THINGS" => Some(g__THE_EVERYDAY_THINGS()),
            "_TRADER_WORDS" => Some(g__TRADER_WORDS()),
            "_WATCH_PLAIN" => Some(g__WATCH_PLAIN()),
            "_WATCH_SMART" => Some(g__WATCH_SMART()),
            "_WATCH_SMART_SAY" => Some(g__WATCH_SMART_SAY()),
            "_WATCH_VERBS" => Some(g__WATCH_VERBS()),
            "_WATCH_VERB_SHAPE" => Some(g__WATCH_VERB_SHAPE()),
            "_a_child_is_named" => Some(py_func("_a_child_is_named", |args: &[Py]| _a_child_is_named(args))),
            "_a_child_is_the_one_they_buy_for" => Some(py_func("_a_child_is_the_one_they_buy_for", |args: &[Py]| _a_child_is_the_one_they_buy_for(args))),
            "_a_pick_for_that_age" => Some(py_func("_a_pick_for_that_age", |args: &[Py]| _a_pick_for_that_age(args))),
            "_a_service_not_a_basket" => Some(py_func("_a_service_not_a_basket", |args: &[Py]| _a_service_not_a_basket(args))),
            "_a_statement_about_where_i_shop" => Some(py_func("_a_statement_about_where_i_shop", |args: &[Py]| _a_statement_about_where_i_shop(args))),
            "_an_age_given" => Some(py_func("_an_age_given", |args: &[Py]| _an_age_given(args))),
            "_an_age_in_the_line" => Some(py_func("_an_age_in_the_line", |args: &[Py]| _an_age_in_the_line(args))),
            "_an_age_said_in_the_line" => Some(py_func("_an_age_said_in_the_line", |args: &[Py]| _an_age_said_in_the_line(args))),
            "_appliance_value" => Some(py_func("_appliance_value", |args: &[Py]| _appliance_value(args))),
            "_bare_decision" => Some(py_func("_bare_decision", |args: &[Py]| _bare_decision(args))),
            "_choice_value" => Some(py_func("_choice_value", |args: &[Py]| _choice_value(args))),
            "_coat_said_already" => Some(py_func("_coat_said_already", |args: &[Py]| _coat_said_already(args))),
            "_coat_to_buy" => Some(py_func("_coat_to_buy", |args: &[Py]| _coat_to_buy(args))),
            "_everyday_where" => Some(py_func("_everyday_where", |args: &[Py]| _everyday_where(args))),
            "_fold" => Some(py_func("_fold", |args: &[Py]| _fold(args))),
            "_gift_anyone" => Some(py_func("_gift_anyone", |args: &[Py]| _gift_anyone(args))),
            "_gift_by_age" => Some(py_func("_gift_by_age", |args: &[Py]| _gift_by_age(args))),
            "_gift_cheap" => Some(py_func("_gift_cheap", |args: &[Py]| _gift_cheap(args))),
            "_gift_for_an_age" => Some(py_func("_gift_for_an_age", |args: &[Py]| _gift_for_an_age(args))),
            "_gift_row" => Some(py_func("_gift_row", |args: &[Py]| _gift_row(args))),
            "_haggle" => Some(py_func("_haggle", |args: &[Py]| _haggle(args))),
            "_interest_phrase" => Some(py_func("_interest_phrase", |args: &[Py]| _interest_phrase(args))),
            "_knows_a_taste" => Some(py_func("_knows_a_taste", |args: &[Py]| _knows_a_taste(args))),
            "_laptop_general" => Some(py_func("_laptop_general", |args: &[Py]| _laptop_general(args))),
            "_longest_match" => Some(py_func("_longest_match", |args: &[Py]| _longest_match(args))),
            "_longest_match_named" => Some(py_func("_longest_match_named", |args: &[Py]| _longest_match_named(args))),
            "_mentions_coat_check" => Some(py_func("_mentions_coat_check", |args: &[Py]| _mentions_coat_check(args))),
            "_money" => Some(py_func("_money", |args: &[Py]| _money(args))),
            "_names_a_childs_age" => Some(py_func("_names_a_childs_age", |args: &[Py]| _names_a_childs_age(args))),
            "_nobody_named_by_pronoun" => Some(py_func("_nobody_named_by_pronoun", |args: &[Py]| _nobody_named_by_pronoun(args))),
            "_not_a_length" => Some(py_func("_not_a_length", |args: &[Py]| _not_a_length(args))),
            "_only_a_them_says_who" => Some(py_func("_only_a_them_says_who", |args: &[Py]| _only_a_them_says_who(args))),
            "_openings" => Some(py_func("_openings", |args: &[Py]| _openings(args))),
            "_present_answer" => Some(py_func("_present_answer", |args: &[Py]| _present_answer(args))),
            "_said_line" => Some(py_func("_said_line", |args: &[Py]| _said_line(args))),
            "_smart_meter_how" => Some(py_func("_smart_meter_how", |args: &[Py]| _smart_meter_how(args))),
            "_supermarket_cheapest" => Some(py_func("_supermarket_cheapest", |args: &[Py]| _supermarket_cheapest(args))),
            "_supermarket_quality" => Some(py_func("_supermarket_quality", |args: &[Py]| _supermarket_quality(args))),
            "_the_age_row" => Some(py_func("_the_age_row", |args: &[Py]| _the_age_row(args))),
            "_the_bag_is_not_packaging" => Some(py_func("_the_bag_is_not_packaging", |args: &[Py]| _the_bag_is_not_packaging(args))),
            "_the_condition_gift" => Some(py_func("_the_condition_gift", |args: &[Py]| _the_condition_gift(args))),
            "_the_condition_in" => Some(py_func("_the_condition_in", |args: &[Py]| _the_condition_in(args))),
            "_the_newborn_present" => Some(py_func("_the_newborn_present", |args: &[Py]| _the_newborn_present(args))),
            "_the_only_age_in_the_store" => Some(py_func("_the_only_age_in_the_store", |args: &[Py]| _the_only_age_in_the_store(args))),
            "_the_person_row" => Some(py_func("_the_person_row", |args: &[Py]| _the_person_row(args))),
            "_the_place_not_the_shop" => Some(py_func("_the_place_not_the_shop", |args: &[Py]| _the_place_not_the_shop(args))),
            "_the_tastes" => Some(py_func("_the_tastes", |args: &[Py]| _the_tastes(args))),
            "_the_thing_a_verdict_is_about" => Some(py_func("_the_thing_a_verdict_is_about", |args: &[Py]| _the_thing_a_verdict_is_about(args))),
            "_the_turn_just_typed" => Some(py_func("_the_turn_just_typed", |args: &[Py]| _the_turn_just_typed(args))),
            "_the_words" => Some(py_func("_the_words", |args: &[Py]| _the_words(args))),
            "_this_row_answers_the_line" => Some(py_func("_this_row_answers_the_line", |args: &[Py]| _this_row_answers_the_line(args))),
            "_watch_help" => Some(py_func("_watch_help", |args: &[Py]| _watch_help(args))),
            "_with_buy_into" => Some(py_func("_with_buy_into", |args: &[Py]| _with_buy_into(args))),
            "_with_the_condition" => Some(py_func("_with_the_condition", |args: &[Py]| _with_the_condition(args))),
            "_without_the_same_opening" => Some(py_func("_without_the_same_opening", |args: &[Py]| _without_the_same_opening(args))),
            "_worth_it_entry" => Some(g__worth_it_entry()),
            "a_present_was_answered" => Some(py_func("a_present_was_answered", |args: &[Py]| a_present_was_answered(args))),
            "not_a_present" => Some(py_func("not_a_present", |args: &[Py]| not_a_present(args))),
            "present_known" => Some(py_func("present_known", |args: &[Py]| present_known(args))),
            "present_less" => Some(py_func("present_less", |args: &[Py]| present_less(args))),
            "present_within" => Some(py_func("present_within", |args: &[Py]| present_within(args))),
            "the_card_after_a_present" => Some(py_func("the_card_after_a_present", |args: &[Py]| the_card_after_a_present(args))),
            "the_verdict_on_a_present" => Some(py_func("the_verdict_on_a_present", |args: &[Py]| the_verdict_on_a_present(args))),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "AGE_ASK" => s_AGE_ASK(v),
            "ANSWERS" => s_ANSWERS(v),
            "APPLIANCE_ASK" => s_APPLIANCE_ASK(v),
            "APPLIANCE_VALUE" => s_APPLIANCE_VALUE(v),
            "A_CARD_IS_NOT_A_PRESENT" => s_A_CARD_IS_NOT_A_PRESENT(v),
            "A_CHEAP_ASK" => s_A_CHEAP_ASK(v),
            "A_VERDICT_ABOUT_MONEY" => s_A_VERDICT_ABOUT_MONEY(v),
            "A_VERDICT_ON_A_PRESENT" => s_A_VERDICT_ON_A_PRESENT(v),
            "A_VERDICT_THING" => s_A_VERDICT_THING(v),
            "A_VERDICT_WORD" => s_A_VERDICT_WORD(v),
            "BUY_INTO" => s_BUY_INTO(v),
            "CARD_AFTER_A_PRESENT" => s_CARD_AFTER_A_PRESENT(v),
            "CHEAP_ADULT" => s_CHEAP_ADULT(v),
            "CHEAP_CHILD" => s_CHEAP_CHILD(v),
            "CHEAP_TEEN" => s_CHEAP_TEEN(v),
            "CHILD_GIFT" => s_CHILD_GIFT(v),
            "CHILD_PEOPLE" => s_CHILD_PEOPLE(v),
            "CHILD_PRESENT" => s_CHILD_PRESENT(v),
            "CHILD_WORDS" => s_CHILD_WORDS(v),
            "CHOICE_VALUE" => s_CHOICE_VALUE(v),
            "CHOOSING" => s_CHOOSING(v),
            "CRISIS" => s_CRISIS(v),
            "ENTRIES" => s_ENTRIES(v),
            "EVERYDAY_WHERE" => s_EVERYDAY_WHERE(v),
            "GIFT_ASK" => s_GIFT_ASK(v),
            "GIFT_FOR_A_CONDITION" => s_GIFT_FOR_A_CONDITION(v),
            "GROWN_GIFT" => s_GROWN_GIFT(v),
            "INTEREST_GIFTS" => s_INTEREST_GIFTS(v),
            "KITCHEN_MACHINE_NAMES" => s_KITCHEN_MACHINE_NAMES(v),
            "KITCHEN_MACHINE_VALUE" => s_KITCHEN_MACHINE_VALUE(v),
            "LOSS" => s_LOSS(v),
            "MONEY_VERDICT_YES" => s_MONEY_VERDICT_YES(v),
            "NOT_THAT_CARD" => s_NOT_THAT_CARD(v),
            "NOT_THE_THING" => s_NOT_THE_THING(v),
            "PERSON_ASK" => s_PERSON_ASK(v),
            "PERSON_WORDS" => s_PERSON_WORDS(v),
            "PLURAL_ONLY" => s_PLURAL_ONLY(v),
            "PLURAL_WORDS" => s_PLURAL_WORDS(v),
            "PRESENT_ASK" => s_PRESENT_ASK(v),
            "PRESENT_BANDS" => s_PRESENT_BANDS(v),
            "PRESENT_OPENINGS" => s_PRESENT_OPENINGS(v),
            "PRESENT_OVER" => s_PRESENT_OVER(v),
            "PRICE_COMPARED" => s_PRICE_COMPARED(v),
            "PRICE_SAID" => s_PRICE_SAID(v),
            "SUPERMARKETS" => s_SUPERMARKETS(v),
            "TASTE_WORDS" => s_TASTE_WORDS(v),
            "TEEN_GIFT" => s_TEEN_GIFT(v),
            "TODDLER_GIFT" => s_TODDLER_GIFT(v),
            "TURNING" => s_TURNING(v),
            "VERDICT_YES" => s_VERDICT_YES(v),
            "_ACTION_NAME" => s__ACTION_NAME(v),
            "_AGE_WORDS" => s__AGE_WORDS(v),
            "_AN_AGE" => s__AN_AGE(v),
            "_AN_AGE_SAID" => s__AN_AGE_SAID(v),
            "_A_CHILDS_AGE" => s__A_CHILDS_AGE(v),
            "_CHEAPEST_TAIL" => s__CHEAPEST_TAIL(v),
            "_CHOOSE_ASK" => s__CHOOSE_ASK(v),
            "_COAT_CHECK" => s__COAT_CHECK(v),
            "_COAT_CHECK_WORDS" => s__COAT_CHECK_WORDS(v),
            "_COAT_KINDS" => s__COAT_KINDS(v),
            "_DEAR_TAIL" => s__DEAR_TAIL(v),
            "_DECISION_WORDS" => s__DECISION_WORDS(v),
            "_DRYER_WORDS" => s__DRYER_WORDS(v),
            "_FOLD" => s__FOLD(v),
            "_GIFT_ANYONE" => s__GIFT_ANYONE(v),
            "_HEIGHT" => s__HEIGHT(v),
            "_INSIDE" => s__INSIDE(v),
            "_KEEP_RECEIPT_RE" => s__KEEP_RECEIPT_RE(v),
            "_METER_FAULT" => s__METER_FAULT(v),
            "_METER_READ" => s__METER_READ(v),
            "_MIDDLE_TAIL" => s__MIDDLE_TAIL(v),
            "_MINE" => s__MINE(v),
            "_NAMED_BY_PRONOUN" => s__NAMED_BY_PRONOUN(v),
            "_PRESENT_PEOPLE" => s__PRESENT_PEOPLE(v),
            "_QUALITY_WORDS" => s__QUALITY_WORDS(v),
            "_RANGE_WORDS" => s__RANGE_WORDS(v),
            "_SAID_AS_WORDS" => s__SAID_AS_WORDS(v),
            "_SERVICE_WORDS" => s__SERVICE_WORDS(v),
            "_SHOP_ASK" => s__SHOP_ASK(v),
            "_SHOP_PLACES" => s__SHOP_PLACES(v),
            "_THE_EVERYDAY_THINGS" => s__THE_EVERYDAY_THINGS(v),
            "_TRADER_WORDS" => s__TRADER_WORDS(v),
            "_WATCH_PLAIN" => s__WATCH_PLAIN(v),
            "_WATCH_SMART" => s__WATCH_SMART(v),
            "_WATCH_SMART_SAY" => s__WATCH_SMART_SAY(v),
            "_WATCH_VERBS" => s__WATCH_VERBS(v),
            "_WATCH_VERB_SHAPE" => s__WATCH_VERB_SHAPE(v),
            "_worth_it_entry" => s__worth_it_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.shopping", module_obj);
    pyrt::xrefs::reg_fn("knowledge.shopping", "_supermarket_quality", py_func("_supermarket_quality", |args: &[Py]| _supermarket_quality(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_without_the_same_opening", py_func("_without_the_same_opening", |args: &[Py]| _without_the_same_opening(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_a_service_not_a_basket", py_func("_a_service_not_a_basket", |args: &[Py]| _a_service_not_a_basket(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_turn_just_typed", py_func("_the_turn_just_typed", |args: &[Py]| _the_turn_just_typed(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_a_statement_about_where_i_shop", py_func("_a_statement_about_where_i_shop", |args: &[Py]| _a_statement_about_where_i_shop(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_supermarket_cheapest", py_func("_supermarket_cheapest", |args: &[Py]| _supermarket_cheapest(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_fold", py_func("_fold", |args: &[Py]| _fold(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_words", py_func("_the_words", |args: &[Py]| _the_words(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_said_line", py_func("_said_line", |args: &[Py]| _said_line(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_knows_a_taste", py_func("_knows_a_taste", |args: &[Py]| _knows_a_taste(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_gift_cheap", py_func("_gift_cheap", |args: &[Py]| _gift_cheap(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_tastes", py_func("_the_tastes", |args: &[Py]| _the_tastes(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "present_known", py_func("present_known", |args: &[Py]| present_known(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "the_card_after_a_present", py_func("the_card_after_a_present", |args: &[Py]| the_card_after_a_present(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "not_a_present", py_func("not_a_present", |args: &[Py]| not_a_present(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_condition_in", py_func("_the_condition_in", |args: &[Py]| _the_condition_in(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_condition_gift", py_func("_the_condition_gift", |args: &[Py]| _the_condition_gift(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_present_answer", py_func("_present_answer", |args: &[Py]| _present_answer(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_thing_a_verdict_is_about", py_func("_the_thing_a_verdict_is_about", |args: &[Py]| _the_thing_a_verdict_is_about(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_only_age_in_the_store", py_func("_the_only_age_in_the_store", |args: &[Py]| _the_only_age_in_the_store(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_a_pick_for_that_age", py_func("_a_pick_for_that_age", |args: &[Py]| _a_pick_for_that_age(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "the_verdict_on_a_present", py_func("the_verdict_on_a_present", |args: &[Py]| the_verdict_on_a_present(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_gift_row", py_func("_gift_row", |args: &[Py]| _gift_row(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_only_a_them_says_who", py_func("_only_a_them_says_who", |args: &[Py]| _only_a_them_says_who(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_newborn_present", py_func("_the_newborn_present", |args: &[Py]| _the_newborn_present(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_with_the_condition", py_func("_with_the_condition", |args: &[Py]| _with_the_condition(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_gift_anyone", py_func("_gift_anyone", |args: &[Py]| _gift_anyone(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_laptop_general", py_func("_laptop_general", |args: &[Py]| _laptop_general(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_openings", py_func("_openings", |args: &[Py]| _openings(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "a_present_was_answered", py_func("a_present_was_answered", |args: &[Py]| a_present_was_answered(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_smart_meter_how", py_func("_smart_meter_how", |args: &[Py]| _smart_meter_how(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_bare_decision", py_func("_bare_decision", |args: &[Py]| _bare_decision(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_longest_match_named", py_func("_longest_match_named", |args: &[Py]| _longest_match_named(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_longest_match", py_func("_longest_match", |args: &[Py]| _longest_match(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_haggle", py_func("_haggle", |args: &[Py]| _haggle(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_choice_value", py_func("_choice_value", |args: &[Py]| _choice_value(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_appliance_value", py_func("_appliance_value", |args: &[Py]| _appliance_value(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_everyday_where", py_func("_everyday_where", |args: &[Py]| _everyday_where(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_gift_for_an_age", py_func("_gift_for_an_age", |args: &[Py]| _gift_for_an_age(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_an_age_in_the_line", py_func("_an_age_in_the_line", |args: &[Py]| _an_age_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_a_child_is_the_one_they_buy_for", py_func("_a_child_is_the_one_they_buy_for", |args: &[Py]| _a_child_is_the_one_they_buy_for(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_person_row", py_func("_the_person_row", |args: &[Py]| _the_person_row(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_an_age_given", py_func("_an_age_given", |args: &[Py]| _an_age_given(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_an_age_said_in_the_line", py_func("_an_age_said_in_the_line", |args: &[Py]| _an_age_said_in_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_names_a_childs_age", py_func("_names_a_childs_age", |args: &[Py]| _names_a_childs_age(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_this_row_answers_the_line", py_func("_this_row_answers_the_line", |args: &[Py]| _this_row_answers_the_line(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_a_child_is_named", py_func("_a_child_is_named", |args: &[Py]| _a_child_is_named(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_gift_by_age", py_func("_gift_by_age", |args: &[Py]| _gift_by_age(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_age_row", py_func("_the_age_row", |args: &[Py]| _the_age_row(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_watch_help", py_func("_watch_help", |args: &[Py]| _watch_help(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_money", py_func("_money", |args: &[Py]| _money(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_interest_phrase", py_func("_interest_phrase", |args: &[Py]| _interest_phrase(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "present_within", py_func("present_within", |args: &[Py]| present_within(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_with_buy_into", py_func("_with_buy_into", |args: &[Py]| _with_buy_into(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "present_less", py_func("present_less", |args: &[Py]| present_less(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_coat_to_buy", py_func("_coat_to_buy", |args: &[Py]| _coat_to_buy(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_coat_said_already", py_func("_coat_said_already", |args: &[Py]| _coat_said_already(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_nobody_named_by_pronoun", py_func("_nobody_named_by_pronoun", |args: &[Py]| _nobody_named_by_pronoun(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_mentions_coat_check", py_func("_mentions_coat_check", |args: &[Py]| _mentions_coat_check(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_not_a_length", py_func("_not_a_length", |args: &[Py]| _not_a_length(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_bag_is_not_packaging", py_func("_the_bag_is_not_packaging", |args: &[Py]| _the_bag_is_not_packaging(args)));
    pyrt::xrefs::reg_fn("knowledge.shopping", "_the_place_not_the_shop", py_func("_the_place_not_the_shop", |args: &[Py]| _the_place_not_the_shop(args)));
    pyrt::xrefs::reg_access("knowledge.shopping", "AGE_ASK", g_AGE_ASK, s_AGE_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.shopping", "APPLIANCE_ASK", g_APPLIANCE_ASK, s_APPLIANCE_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "APPLIANCE_VALUE", g_APPLIANCE_VALUE, s_APPLIANCE_VALUE);
    pyrt::xrefs::reg_access("knowledge.shopping", "A_CARD_IS_NOT_A_PRESENT", g_A_CARD_IS_NOT_A_PRESENT, s_A_CARD_IS_NOT_A_PRESENT);
    pyrt::xrefs::reg_access("knowledge.shopping", "A_CHEAP_ASK", g_A_CHEAP_ASK, s_A_CHEAP_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "A_VERDICT_ABOUT_MONEY", g_A_VERDICT_ABOUT_MONEY, s_A_VERDICT_ABOUT_MONEY);
    pyrt::xrefs::reg_access("knowledge.shopping", "A_VERDICT_ON_A_PRESENT", g_A_VERDICT_ON_A_PRESENT, s_A_VERDICT_ON_A_PRESENT);
    pyrt::xrefs::reg_access("knowledge.shopping", "A_VERDICT_THING", g_A_VERDICT_THING, s_A_VERDICT_THING);
    pyrt::xrefs::reg_access("knowledge.shopping", "A_VERDICT_WORD", g_A_VERDICT_WORD, s_A_VERDICT_WORD);
    pyrt::xrefs::reg_access("knowledge.shopping", "BUY_INTO", g_BUY_INTO, s_BUY_INTO);
    pyrt::xrefs::reg_access("knowledge.shopping", "CARD_AFTER_A_PRESENT", g_CARD_AFTER_A_PRESENT, s_CARD_AFTER_A_PRESENT);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHEAP_ADULT", g_CHEAP_ADULT, s_CHEAP_ADULT);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHEAP_CHILD", g_CHEAP_CHILD, s_CHEAP_CHILD);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHEAP_TEEN", g_CHEAP_TEEN, s_CHEAP_TEEN);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHILD_GIFT", g_CHILD_GIFT, s_CHILD_GIFT);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHILD_PEOPLE", g_CHILD_PEOPLE, s_CHILD_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHILD_PRESENT", g_CHILD_PRESENT, s_CHILD_PRESENT);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHILD_WORDS", g_CHILD_WORDS, s_CHILD_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHOICE_VALUE", g_CHOICE_VALUE, s_CHOICE_VALUE);
    pyrt::xrefs::reg_access("knowledge.shopping", "CHOOSING", g_CHOOSING, s_CHOOSING);
    pyrt::xrefs::reg_access("knowledge.shopping", "CRISIS", g_CRISIS, s_CRISIS);
    pyrt::xrefs::reg_access("knowledge.shopping", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.shopping", "EVERYDAY_WHERE", g_EVERYDAY_WHERE, s_EVERYDAY_WHERE);
    pyrt::xrefs::reg_access("knowledge.shopping", "GIFT_ASK", g_GIFT_ASK, s_GIFT_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "GIFT_FOR_A_CONDITION", g_GIFT_FOR_A_CONDITION, s_GIFT_FOR_A_CONDITION);
    pyrt::xrefs::reg_access("knowledge.shopping", "GROWN_GIFT", g_GROWN_GIFT, s_GROWN_GIFT);
    pyrt::xrefs::reg_access("knowledge.shopping", "INTEREST_GIFTS", g_INTEREST_GIFTS, s_INTEREST_GIFTS);
    pyrt::xrefs::reg_access("knowledge.shopping", "KITCHEN_MACHINE_NAMES", g_KITCHEN_MACHINE_NAMES, s_KITCHEN_MACHINE_NAMES);
    pyrt::xrefs::reg_access("knowledge.shopping", "KITCHEN_MACHINE_VALUE", g_KITCHEN_MACHINE_VALUE, s_KITCHEN_MACHINE_VALUE);
    pyrt::xrefs::reg_access("knowledge.shopping", "LOSS", g_LOSS, s_LOSS);
    pyrt::xrefs::reg_access("knowledge.shopping", "MONEY_VERDICT_YES", g_MONEY_VERDICT_YES, s_MONEY_VERDICT_YES);
    pyrt::xrefs::reg_access("knowledge.shopping", "NOT_THAT_CARD", g_NOT_THAT_CARD, s_NOT_THAT_CARD);
    pyrt::xrefs::reg_access("knowledge.shopping", "NOT_THE_THING", g_NOT_THE_THING, s_NOT_THE_THING);
    pyrt::xrefs::reg_access("knowledge.shopping", "PERSON_ASK", g_PERSON_ASK, s_PERSON_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "PERSON_WORDS", g_PERSON_WORDS, s_PERSON_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "PLURAL_ONLY", g_PLURAL_ONLY, s_PLURAL_ONLY);
    pyrt::xrefs::reg_access("knowledge.shopping", "PLURAL_WORDS", g_PLURAL_WORDS, s_PLURAL_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "PRESENT_ASK", g_PRESENT_ASK, s_PRESENT_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "PRESENT_BANDS", g_PRESENT_BANDS, s_PRESENT_BANDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "PRESENT_OPENINGS", g_PRESENT_OPENINGS, s_PRESENT_OPENINGS);
    pyrt::xrefs::reg_access("knowledge.shopping", "PRESENT_OVER", g_PRESENT_OVER, s_PRESENT_OVER);
    pyrt::xrefs::reg_access("knowledge.shopping", "PRICE_COMPARED", g_PRICE_COMPARED, s_PRICE_COMPARED);
    pyrt::xrefs::reg_access("knowledge.shopping", "PRICE_SAID", g_PRICE_SAID, s_PRICE_SAID);
    pyrt::xrefs::reg_access("knowledge.shopping", "SUPERMARKETS", g_SUPERMARKETS, s_SUPERMARKETS);
    pyrt::xrefs::reg_access("knowledge.shopping", "TASTE_WORDS", g_TASTE_WORDS, s_TASTE_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "TEEN_GIFT", g_TEEN_GIFT, s_TEEN_GIFT);
    pyrt::xrefs::reg_access("knowledge.shopping", "TODDLER_GIFT", g_TODDLER_GIFT, s_TODDLER_GIFT);
    pyrt::xrefs::reg_access("knowledge.shopping", "TURNING", g_TURNING, s_TURNING);
    pyrt::xrefs::reg_access("knowledge.shopping", "VERDICT_YES", g_VERDICT_YES, s_VERDICT_YES);
    pyrt::xrefs::reg_access("knowledge.shopping", "_ACTION_NAME", g__ACTION_NAME, s__ACTION_NAME);
    pyrt::xrefs::reg_access("knowledge.shopping", "_AGE_WORDS", g__AGE_WORDS, s__AGE_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_AN_AGE", g__AN_AGE, s__AN_AGE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_AN_AGE_SAID", g__AN_AGE_SAID, s__AN_AGE_SAID);
    pyrt::xrefs::reg_access("knowledge.shopping", "_A_CHILDS_AGE", g__A_CHILDS_AGE, s__A_CHILDS_AGE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_CHEAPEST_TAIL", g__CHEAPEST_TAIL, s__CHEAPEST_TAIL);
    pyrt::xrefs::reg_access("knowledge.shopping", "_CHOOSE_ASK", g__CHOOSE_ASK, s__CHOOSE_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "_COAT_CHECK", g__COAT_CHECK, s__COAT_CHECK);
    pyrt::xrefs::reg_access("knowledge.shopping", "_COAT_CHECK_WORDS", g__COAT_CHECK_WORDS, s__COAT_CHECK_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_COAT_KINDS", g__COAT_KINDS, s__COAT_KINDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_DEAR_TAIL", g__DEAR_TAIL, s__DEAR_TAIL);
    pyrt::xrefs::reg_access("knowledge.shopping", "_DECISION_WORDS", g__DECISION_WORDS, s__DECISION_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_DRYER_WORDS", g__DRYER_WORDS, s__DRYER_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_FOLD", g__FOLD, s__FOLD);
    pyrt::xrefs::reg_access("knowledge.shopping", "_GIFT_ANYONE", g__GIFT_ANYONE, s__GIFT_ANYONE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_HEIGHT", g__HEIGHT, s__HEIGHT);
    pyrt::xrefs::reg_access("knowledge.shopping", "_INSIDE", g__INSIDE, s__INSIDE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_KEEP_RECEIPT_RE", g__KEEP_RECEIPT_RE, s__KEEP_RECEIPT_RE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_METER_FAULT", g__METER_FAULT, s__METER_FAULT);
    pyrt::xrefs::reg_access("knowledge.shopping", "_METER_READ", g__METER_READ, s__METER_READ);
    pyrt::xrefs::reg_access("knowledge.shopping", "_MIDDLE_TAIL", g__MIDDLE_TAIL, s__MIDDLE_TAIL);
    pyrt::xrefs::reg_access("knowledge.shopping", "_MINE", g__MINE, s__MINE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_NAMED_BY_PRONOUN", g__NAMED_BY_PRONOUN, s__NAMED_BY_PRONOUN);
    pyrt::xrefs::reg_access("knowledge.shopping", "_PRESENT_PEOPLE", g__PRESENT_PEOPLE, s__PRESENT_PEOPLE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_QUALITY_WORDS", g__QUALITY_WORDS, s__QUALITY_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_RANGE_WORDS", g__RANGE_WORDS, s__RANGE_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_SAID_AS_WORDS", g__SAID_AS_WORDS, s__SAID_AS_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_SERVICE_WORDS", g__SERVICE_WORDS, s__SERVICE_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_SHOP_ASK", g__SHOP_ASK, s__SHOP_ASK);
    pyrt::xrefs::reg_access("knowledge.shopping", "_SHOP_PLACES", g__SHOP_PLACES, s__SHOP_PLACES);
    pyrt::xrefs::reg_access("knowledge.shopping", "_THE_EVERYDAY_THINGS", g__THE_EVERYDAY_THINGS, s__THE_EVERYDAY_THINGS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_TRADER_WORDS", g__TRADER_WORDS, s__TRADER_WORDS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_WATCH_PLAIN", g__WATCH_PLAIN, s__WATCH_PLAIN);
    pyrt::xrefs::reg_access("knowledge.shopping", "_WATCH_SMART", g__WATCH_SMART, s__WATCH_SMART);
    pyrt::xrefs::reg_access("knowledge.shopping", "_WATCH_SMART_SAY", g__WATCH_SMART_SAY, s__WATCH_SMART_SAY);
    pyrt::xrefs::reg_access("knowledge.shopping", "_WATCH_VERBS", g__WATCH_VERBS, s__WATCH_VERBS);
    pyrt::xrefs::reg_access("knowledge.shopping", "_WATCH_VERB_SHAPE", g__WATCH_VERB_SHAPE, s__WATCH_VERB_SHAPE);
    pyrt::xrefs::reg_access("knowledge.shopping", "_worth_it_entry", g__worth_it_entry, s__worth_it_entry);
}

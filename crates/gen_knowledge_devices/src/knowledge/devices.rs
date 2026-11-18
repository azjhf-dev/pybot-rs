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
thread_local! { static G_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOTH_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BOTH_SPACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARD_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ENTRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_GLASS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOUSE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_MUCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HOW_THEY_KNOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAC_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MAC_SPACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_SOCIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_STORAGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_WHERE_I_AM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PC: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PC_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PC_CHECK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PC_SPACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PHONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ROOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SD_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SD_CARD_BUY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SD_CARD_COPY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHARE_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SHARE_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIM_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIM_NOT_THIS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SOCIAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHERE_I_AM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHO_KNOWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_FOR_THE_STEPS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_HOW_IT_WORKS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_HOW_THEY_KNOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_TO_STOP_SHARING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_WHETHER_TO_SHARE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ASKS_WHO_CAN_SEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CARD_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_CARD_BEING_CHOSEN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__BUYING_A_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_FULL_IN_A_PHONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_INTO_A_DEVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_IN_ANDROID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_IN_DEVICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_IN_IPHONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_IN_LAPTOP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_IN_SWITCH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_IN_TABLET: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_NOT_READ: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARD_NOT_READING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEVICE_OF_THE_CARD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DISK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__FOOD_NOT_A_MACHINE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_CARD_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NO_SLOT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_entry: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT knowledge.devices: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ANSWERS() -> Py { ensure_init(); G_ANSWERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ANSWERS(v: Py) { ensure_init(); G_ANSWERS.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK() -> Py { ensure_init(); G_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK(v: Py) { ensure_init(); G_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_BOTH_CHECK() -> Py { ensure_init(); G_BOTH_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOTH_CHECK(v: Py) { ensure_init(); G_BOTH_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_BOTH_SPACE() -> Py { ensure_init(); G_BOTH_SPACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BOTH_SPACE(v: Py) { ensure_init(); G_BOTH_SPACE.with(|c| *c.borrow_mut() = v); }
pub fn g_CARD_ROOM() -> Py { ensure_init(); G_CARD_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARD_ROOM(v: Py) { ensure_init(); G_CARD_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK() -> Py { ensure_init(); G_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK(v: Py) { ensure_init(); G_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_ENTRIES() -> Py { ensure_init(); G_ENTRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ENTRIES(v: Py) { ensure_init(); G_ENTRIES.with(|c| *c.borrow_mut() = v); }
pub fn g_GLASS() -> Py { ensure_init(); G_GLASS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_GLASS(v: Py) { ensure_init(); G_GLASS.with(|c| *c.borrow_mut() = v); }
pub fn g_HOUSE() -> Py { ensure_init(); G_HOUSE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOUSE(v: Py) { ensure_init(); G_HOUSE.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_MUCH() -> Py { ensure_init(); G_HOW_MUCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_MUCH(v: Py) { ensure_init(); G_HOW_MUCH.with(|c| *c.borrow_mut() = v); }
pub fn g_HOW_THEY_KNOW() -> Py { ensure_init(); G_HOW_THEY_KNOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HOW_THEY_KNOW(v: Py) { ensure_init(); G_HOW_THEY_KNOW.with(|c| *c.borrow_mut() = v); }
pub fn g_MAC() -> Py { ensure_init(); G_MAC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAC(v: Py) { ensure_init(); G_MAC.with(|c| *c.borrow_mut() = v); }
pub fn g_MAC_CHECK() -> Py { ensure_init(); G_MAC_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAC_CHECK(v: Py) { ensure_init(); G_MAC_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_MAC_SPACE() -> Py { ensure_init(); G_MAC_SPACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MAC_SPACE(v: Py) { ensure_init(); G_MAC_SPACE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_SOCIAL() -> Py { ensure_init(); G_NOT_SOCIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_SOCIAL(v: Py) { ensure_init(); G_NOT_SOCIAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_STORAGE() -> Py { ensure_init(); G_NOT_STORAGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_STORAGE(v: Py) { ensure_init(); G_NOT_STORAGE.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_WHERE_I_AM() -> Py { ensure_init(); G_NOT_WHERE_I_AM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_WHERE_I_AM(v: Py) { ensure_init(); G_NOT_WHERE_I_AM.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHER() -> Py { ensure_init(); G_OTHER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHER(v: Py) { ensure_init(); G_OTHER.with(|c| *c.borrow_mut() = v); }
pub fn g_PC() -> Py { ensure_init(); G_PC.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PC(v: Py) { ensure_init(); G_PC.with(|c| *c.borrow_mut() = v); }
pub fn g_PC_ASK() -> Py { ensure_init(); G_PC_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PC_ASK(v: Py) { ensure_init(); G_PC_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_PC_CHECK() -> Py { ensure_init(); G_PC_CHECK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PC_CHECK(v: Py) { ensure_init(); G_PC_CHECK.with(|c| *c.borrow_mut() = v); }
pub fn g_PC_SPACE() -> Py { ensure_init(); G_PC_SPACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PC_SPACE(v: Py) { ensure_init(); G_PC_SPACE.with(|c| *c.borrow_mut() = v); }
pub fn g_PHONE() -> Py { ensure_init(); G_PHONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PHONE(v: Py) { ensure_init(); G_PHONE.with(|c| *c.borrow_mut() = v); }
pub fn g_ROOM() -> Py { ensure_init(); G_ROOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ROOM(v: Py) { ensure_init(); G_ROOM.with(|c| *c.borrow_mut() = v); }
pub fn g_SD_CARD() -> Py { ensure_init(); G_SD_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SD_CARD(v: Py) { ensure_init(); G_SD_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_SD_CARD_BUY() -> Py { ensure_init(); G_SD_CARD_BUY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SD_CARD_BUY(v: Py) { ensure_init(); G_SD_CARD_BUY.with(|c| *c.borrow_mut() = v); }
pub fn g_SD_CARD_COPY() -> Py { ensure_init(); G_SD_CARD_COPY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SD_CARD_COPY(v: Py) { ensure_init(); G_SD_CARD_COPY.with(|c| *c.borrow_mut() = v); }
pub fn g_SHARE_IT() -> Py { ensure_init(); G_SHARE_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHARE_IT(v: Py) { ensure_init(); G_SHARE_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_SHARE_STEPS() -> Py { ensure_init(); G_SHARE_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SHARE_STEPS(v: Py) { ensure_init(); G_SHARE_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g_SIM_CARD() -> Py { ensure_init(); G_SIM_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIM_CARD(v: Py) { ensure_init(); G_SIM_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g_SIM_NOT_THIS() -> Py { ensure_init(); G_SIM_NOT_THIS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIM_NOT_THIS(v: Py) { ensure_init(); G_SIM_NOT_THIS.with(|c| *c.borrow_mut() = v); }
pub fn g_SOCIAL() -> Py { ensure_init(); G_SOCIAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SOCIAL(v: Py) { ensure_init(); G_SOCIAL.with(|c| *c.borrow_mut() = v); }
pub fn g_WHERE_I_AM() -> Py { ensure_init(); G_WHERE_I_AM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHERE_I_AM(v: Py) { ensure_init(); G_WHERE_I_AM.with(|c| *c.borrow_mut() = v); }
pub fn g_WHO_KNOWS() -> Py { ensure_init(); G_WHO_KNOWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHO_KNOWS(v: Py) { ensure_init(); G_WHO_KNOWS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_FOR_THE_STEPS() -> Py { ensure_init(); G__ASKS_FOR_THE_STEPS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_FOR_THE_STEPS(v: Py) { ensure_init(); G__ASKS_FOR_THE_STEPS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_HOW_IT_WORKS() -> Py { ensure_init(); G__ASKS_HOW_IT_WORKS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_HOW_IT_WORKS(v: Py) { ensure_init(); G__ASKS_HOW_IT_WORKS.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_HOW_THEY_KNOW() -> Py { ensure_init(); G__ASKS_HOW_THEY_KNOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_HOW_THEY_KNOW(v: Py) { ensure_init(); G__ASKS_HOW_THEY_KNOW.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_TO_STOP_SHARING() -> Py { ensure_init(); G__ASKS_TO_STOP_SHARING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_TO_STOP_SHARING(v: Py) { ensure_init(); G__ASKS_TO_STOP_SHARING.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_WHETHER_TO_SHARE() -> Py { ensure_init(); G__ASKS_WHETHER_TO_SHARE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_WHETHER_TO_SHARE(v: Py) { ensure_init(); G__ASKS_WHETHER_TO_SHARE.with(|c| *c.borrow_mut() = v); }
pub fn g__ASKS_WHO_CAN_SEE() -> Py { ensure_init(); G__ASKS_WHO_CAN_SEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ASKS_WHO_CAN_SEE(v: Py) { ensure_init(); G__ASKS_WHO_CAN_SEE.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CARD_ASK() -> Py { ensure_init(); G__A_CARD_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CARD_ASK(v: Py) { ensure_init(); G__A_CARD_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g__A_CARD_BEING_CHOSEN() -> Py { ensure_init(); G__A_CARD_BEING_CHOSEN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_CARD_BEING_CHOSEN(v: Py) { ensure_init(); G__A_CARD_BEING_CHOSEN.with(|c| *c.borrow_mut() = v); }
pub fn g__BUYING_A_CARD() -> Py { ensure_init(); G__BUYING_A_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__BUYING_A_CARD(v: Py) { ensure_init(); G__BUYING_A_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_FULL_IN_A_PHONE() -> Py { ensure_init(); G__CARD_FULL_IN_A_PHONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_FULL_IN_A_PHONE(v: Py) { ensure_init(); G__CARD_FULL_IN_A_PHONE.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_INTO_A_DEVICE() -> Py { ensure_init(); G__CARD_INTO_A_DEVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_INTO_A_DEVICE(v: Py) { ensure_init(); G__CARD_INTO_A_DEVICE.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_IN_ANDROID() -> Py { ensure_init(); G__CARD_IN_ANDROID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_IN_ANDROID(v: Py) { ensure_init(); G__CARD_IN_ANDROID.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_IN_DEVICE() -> Py { ensure_init(); G__CARD_IN_DEVICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_IN_DEVICE(v: Py) { ensure_init(); G__CARD_IN_DEVICE.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_IN_IPHONE() -> Py { ensure_init(); G__CARD_IN_IPHONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_IN_IPHONE(v: Py) { ensure_init(); G__CARD_IN_IPHONE.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_IN_LAPTOP() -> Py { ensure_init(); G__CARD_IN_LAPTOP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_IN_LAPTOP(v: Py) { ensure_init(); G__CARD_IN_LAPTOP.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_IN_SWITCH() -> Py { ensure_init(); G__CARD_IN_SWITCH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_IN_SWITCH(v: Py) { ensure_init(); G__CARD_IN_SWITCH.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_IN_TABLET() -> Py { ensure_init(); G__CARD_IN_TABLET.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_IN_TABLET(v: Py) { ensure_init(); G__CARD_IN_TABLET.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_NOT_READ() -> Py { ensure_init(); G__CARD_NOT_READ.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_NOT_READ(v: Py) { ensure_init(); G__CARD_NOT_READ.with(|c| *c.borrow_mut() = v); }
pub fn g__CARD_NOT_READING() -> Py { ensure_init(); G__CARD_NOT_READING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARD_NOT_READING(v: Py) { ensure_init(); G__CARD_NOT_READING.with(|c| *c.borrow_mut() = v); }
pub fn g__DEVICE_OF_THE_CARD() -> Py { ensure_init(); G__DEVICE_OF_THE_CARD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEVICE_OF_THE_CARD(v: Py) { ensure_init(); G__DEVICE_OF_THE_CARD.with(|c| *c.borrow_mut() = v); }
pub fn g__DISK_WORDS() -> Py { ensure_init(); G__DISK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DISK_WORDS(v: Py) { ensure_init(); G__DISK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__FOOD_NOT_A_MACHINE() -> Py { ensure_init(); G__FOOD_NOT_A_MACHINE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__FOOD_NOT_A_MACHINE(v: Py) { ensure_init(); G__FOOD_NOT_A_MACHINE.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_CARD_SLOT() -> Py { ensure_init(); G__NO_CARD_SLOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_CARD_SLOT(v: Py) { ensure_init(); G__NO_CARD_SLOT.with(|c| *c.borrow_mut() = v); }
pub fn g__NO_SLOT() -> Py { ensure_init(); G__NO_SLOT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NO_SLOT(v: Py) { ensure_init(); G__NO_SLOT.with(|c| *c.borrow_mut() = v); }
pub fn g_entry() -> Py { ensure_init(); G_entry.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_entry(v: Py) { ensure_init(); G_entry.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Storage on a computer: a Mac and a Windows PC are emptied in different places.\n\n`technology`'s storage rows keyed on the bare words \"storage\", \"space\" and \"free up\" and\nanswered with Settings > General > iPhone Storage. That screen does not exist on a Mac or a\nPC, so the commonest laptop question of all -\n\n    how do i free up space on my mac      -> Settings > General > iPhone Storage ...\n\ngot a phone answer, and \"i need to make room on my mac\" got nothing at all. A Mac and a PC\nare also not the same job: a Mac hides photos, iPhone backups and the Bin behind one Storage\npane, while Windows puts its leftovers under Temporary files and Storage Sense.\n\nSo this domain answers one question per machine, and reads the machine out of the line. It\npasses on a line that names no computer, which leaves the phone rows and everything else in\n`technology` exactly as they were.\n");
    s_entry(py_func("entry", |args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.ANSWERS"); }
    s_ANSWERS(py_tuple(vec![py_str("technology")]));
    fn __data1() -> Py { py_tuple(vec![py_str("mac"), py_str("macs"), py_str("macbook"), py_str("macbooks"), py_str("mac book"), py_str("imac"), py_str("i mac"), py_str("mac os"), py_str("macos"), py_str("osx"), py_str("os x"), py_str("apple computer"), py_str("apple laptop")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.MAC"); }
    s_MAC(__data1());
    fn __data2() -> Py { py_tuple(vec![py_str("windows"), py_str("pc"), py_str("pcs"), py_str("desktop"), py_str("desktops"), py_str("laptop"), py_str("laptops"), py_str("thinkpad"), py_str("chromebook"), py_str("surface"), py_str("computer"), py_str("computers"), py_str("hard drive"), py_str("hard disk"), py_str("ssd"), py_str("disk")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.PC"); }
    s_PC(__data2());
    fn __data3() -> Py { py_tuple(vec![py_str("storage"), py_str("space"), py_str("full"), py_str("free up"), py_str("clear"), py_str("delete"), py_str("disk"), py_str("running out"), py_str("make room"), py_str("room"), py_str("no space"), py_str("space left"), py_str("out of space"), py_str("no room")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.ROOM"); }
    s_ROOM(__data3());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.ASK"); }
    s_ASK(py_tuple(vec![py_str("storage"), py_str("space"), py_str("full"), py_str("free up"), py_str("clear"), py_str("delete"), py_str("disk"), py_str("room"), py_str("running out"), py_str("out of")]));
    fn __data4() -> Py { py_tuple(vec![py_str("windows"), py_str("pc"), py_str("pcs"), py_str("desktop"), py_str("disk"), py_str("hard drive"), py_str("hard disk"), py_str("ssd"), py_str("computer"), py_str("computers"), py_str("thinkpad"), py_str("chromebook"), py_str("surface"), py_str("space left"), py_str("space is left"), py_str("no space"), py_str("out of space")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.PC_ASK"); }
    s_PC_ASK(__data4());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.CHECK"); }
    s_CHECK(py_tuple(vec![py_str("how much"), py_str("how full"), py_str("how many gb"), py_str("how big"), py_str("space left"), py_str("space is left"), py_str("no space left"), py_str("enough space"), py_str("percent"), py_str("%")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.HOW_MUCH"); }
    s_HOW_MUCH(py_tuple(vec![py_str("space left"), py_str("is left"), py_str("left"), py_str("shows"), py_str("how much"), py_str("how full"), py_str("how many")]));
    fn __data5() -> Py { py_tuple(vec![py_str("delete an app"), py_str("delete apps"), py_str("delete a app"), py_str("deleting an app"), py_str("delete the app"), py_str("delete my app"), py_str("delete an application"), py_str("delete the application"), py_str("remove an app"), py_str("removing an app"), py_str("uninstall"), py_str("uninstalling"), py_str("calorie"), py_str("calories"), py_str("kcal"), py_str("mac and cheese"), py_str("mac n cheese"), py_str("macaroni and cheese"), py_str("macaroni cheese"), py_str("macaroni"), py_str("program"), py_str("programs"), py_str("uninstalled"), py_str("reinstalled"), py_str("paint a room"), py_str("painting a room"), py_str("paint the room"), py_str("paint a wall"), py_str("paint the walls"), py_str("paint over"), py_str("painting over"), py_str("decorating"), py_str("decorate"), py_str("roller"), py_str("emulsion"), py_str("undercoat"), py_str("skirting"), py_str("wallpaper"), py_str("gloss paint")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.NOT_STORAGE"); }
    s_NOT_STORAGE(__data5());
    fn __data6() -> Py { py_tuple(vec![py_str("condensation"), py_str("condensation on"), py_str("condensation off"), py_str("condensed"), py_str("steamed up"), py_str("steaming up"), py_str("steams up"), py_str("misted up"), py_str("misting up"), py_str("fogged up"), py_str("fogging up"), py_str("wet windows"), py_str("windows wet"), py_str("damp on the windows"), py_str("pane"), py_str("panes"), py_str("squeegee"), py_str("windscreen"), py_str("windshield"), py_str("cleaning the windows"), py_str("clean the windows"), py_str("clean windows")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.GLASS"); }
    s_GLASS(__data6());
    fn __data7() -> Py { py_tuple(vec![py_str("living room"), py_str("bedroom"), py_str("bedrooms"), py_str("kitchen"), py_str("bathroom"), py_str("hallway"), py_str("conservatory"), py_str("floorboards"), py_str("wallpaper"), py_str("skirting"), py_str("no window"), py_str("no windows"), py_str("windowless"), py_str("without a window"), py_str("bigger window"), py_str("curtains"), py_str("blinds")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.HOUSE"); }
    s_HOUSE(op_add(&__data7(), &g_GLASS()));
    fn __data8() -> Py { py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("ipad"), py_str("android"), py_str("tablet"), py_str("handset"), py_str("sd card"), py_str("memory card"), py_str("usb stick"), py_str("usb drive"), py_str("external drive"), py_str("self storage"), py_str("warehouse"), py_str("garage"), py_str("shed"), py_str("loft"), py_str("attic"), py_str("rent")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.PHONE"); }
    s_PHONE(__data8());
    fn __data9() -> Py { py_tuple(vec![py_str("battery"), py_str("charge"), py_str("charging"), py_str("charger"), py_str("power"), py_str("slow"), py_str("sluggish"), py_str("hot"), py_str("overheating"), py_str("fan"), py_str("noisy"), py_str("loud"), py_str("memory"), py_str("ram"), py_str("backup"), py_str("time machine"), py_str("broken"), py_str("repair"), py_str("replace"), py_str("buy"), py_str("worth"), py_str("price"), py_str("cost")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.OTHER"); }
    s_OTHER(__data9());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.MAC_SPACE"); }
    s_MAC_SPACE(py_str("On a Mac: Apple menu > System Settings > General > Storage, click the bar and let it finish scanning - Photos and old iPhone backups are usually the two biggest. Empty the Bin, clear the Downloads folder, and turn on Optimise Mac Storage in Photos. A phone backup can be 50GB on its own: Finder > your phone > Manage Backups."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.PC_SPACE"); }
    s_PC_SPACE(py_str("On Windows: Settings > System > Storage, then Temporary files - Windows Update leftovers and old installers are usually a few gigabytes. Turn on Storage Sense so they go on their own, then sort the Apps list by size and move photos and videos to an external drive or OneDrive. The Downloads folder is the other quiet hog."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.BOTH_SPACE"); }
    s_BOTH_SPACE(py_str("On a Mac: Apple menu > System Settings > General > Storage, then empty the Bin and clear Downloads. On Windows: Settings > System > Storage > Temporary files, and turn on Storage Sense. Photos, videos and old installers are the biggest three either way."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.BOTH_CHECK"); }
    s_BOTH_CHECK(py_str("On a Mac: Apple menu > System Settings > General > Storage. On Windows: Settings > System > Storage. Both show what is left and what is taking it, biggest first; keep 10 to 20% free."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.MAC_CHECK"); }
    s_MAC_CHECK(py_str("Apple menu > System Settings > General > Storage shows the bar and what is taking it, biggest first. Keep about 10 to 20% free: below that a Mac slows down and updates refuse to install."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.PC_CHECK"); }
    s_PC_CHECK(py_str("Settings > System > Storage shows what is left and what is taking it, biggest first. Keep about 10 to 20% free: below that Windows slows down and updates fail to install."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SD_CARD_COPY"); }
    s_SD_CARD_COPY(py_str("Copy things off it before you delete anything: a card that fills up in a camera is often a confused file table, and bulk deleting on the camera loses shots. Plug the card into a laptop with a reader, drag the photos and videos to the computer or to Google Photos, then let the camera format the card from its own menu - not the computer. A bigger card is cheap: 64GB is about a tenner, and take one labelled U3 or V30 if it is for video."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SD_CARD_BUY"); }
    s_SD_CARD_BUY(py_str("64GB to 128GB is plenty for photos, and 128GB or more for video, because a minute of 4K is about 400MB. Look for U3 or V30 on the label - that is a guaranteed write speed, and it is what 4K video needs - and buy from a shop you trust rather than a marketplace stall, because fake cards report more room than they have and then lose files. SanDisk, Samsung and Lexar are the safe names; a card is cheap enough to buy two."));
    fn __data10() -> Py { py_tuple(vec![py_str("which"), py_str("what card"), py_str("what sd"), py_str("should i buy"), py_str("should i get"), py_str("recommend"), py_str("best card"), py_str("best sd"), py_str("best memory"), py_str("worth buying"), py_str("how big"), py_str("what size"), py_str("which size"), py_str("for video"), py_str("for a camera"), py_str("for my camera"), py_str("buying"), py_str("bigger"), py_str("larger"), py_str("bigger card"), py_str("bigger sd"), py_str("bigger memory"), py_str("more space"), py_str("more storage"), py_str("upgrade"), py_str("how much space")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._BUYING_A_CARD"); }
    s__BUYING_A_CARD(__data10());
    fn __data11() -> Py { py_tuple(vec![py_str("phone"), py_str("iphone"), py_str("ipad"), py_str("tablet"), py_str("android"), py_str("samsung"), py_str("galaxy"), py_str("pixel"), py_str("switch"), py_str("nintendo"), py_str("steam deck"), py_str("console"), py_str("laptop"), py_str("macbook"), py_str("computer"), py_str("slot"), py_str("put in"), py_str("put a card"), py_str("bigger card"), py_str("extra space"), py_str("more memory"), py_str("adoptable"), py_str("expand"), py_str("expandable")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_INTO_A_DEVICE"); }
    s__CARD_INTO_A_DEVICE(__data11());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._A_CARD_BEING_CHOSEN"); }
    s__A_CARD_BEING_CHOSEN(py_tuple(vec![py_str("what size"), py_str("which size"), py_str("how big"), py_str("which card"), py_str("what card"), py_str("should i buy"), py_str("should i get"), py_str("recommend"), py_str("best card"), py_str("need")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._A_CARD_ASK"); }
    s__A_CARD_ASK(py_tuple(vec![py_str("does"), py_str("do i"), py_str("how do i"), py_str("can i"), py_str("is there a"), py_str("will it"), py_str("where")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_IN_IPHONE"); }
    s__CARD_IN_IPHONE(py_str("There is no card slot in an iPhone, so no card goes in it: the room has to come from the phone or the cloud. Turn on iCloud Photos and Optimise iPhone Storage (Settings > Photos) and the full-size originals move off while small versions stay on the phone, then delete the videos you have already watched. If the card is a camera's, a card reader for the charging port copies the photos across."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_IN_ANDROID"); }
    s__CARD_IN_ANDROID(py_str("Most Android phones that still have a slot take a microSD card in the SIM tray, but recent flagships and the Pixel line have dropped it, so check the spec for your exact model first. If it has one, put the card in with the phone off and then choose: portable storage, where photos and videos go on the card, or internal storage (Settings > Storage > the card > Format as internal), where apps install there but the card is tied to that phone. 64GB or 128GB labelled A1 or A2 is the right buy."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_IN_TABLET"); }
    s__CARD_IN_TABLET(py_str("An iPad has no card slot, so its room comes from iCloud or from offloading photos and apps. An Android tablet usually takes a microSD, with the same choice as a phone: portable storage, or formatted as internal. A USB-C card reader copies a camera's photos across on either one."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_IN_SWITCH"); }
    s__CARD_IN_SWITCH(py_str("The Switch takes a microSD card under the kickstand at the back, behind the little flap: microSD only, not the full-size card a camera uses. 128GB or 256GB labelled U3 or A1 is plenty - games are 5GB to 15GB each - and after you put it in, Settings > Data Management moves the games across."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_IN_LAPTOP"); }
    s__CARD_IN_LAPTOP(py_str("If the laptop has its own SD slot, push the card in until it clicks and it shows up as a drive; if it has none, a USB card reader is a few pounds. A card is for moving photos around rather than for more room on the laptop itself - for that an external SSD or a bigger internal drive is the answer. Copy the files across rather than working off the card, then format it in the camera."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._DEVICE_OF_THE_CARD"); }
    s__DEVICE_OF_THE_CARD(py_tuple(vec![py_tuple(vec![py_str("switch"), py_str("nintendo"), py_str("steam deck"), py_str("console")]), py_tuple(vec![py_str("iphone")]), py_tuple(vec![py_str("laptop"), py_str("macbook"), py_str("notebook")]), py_tuple(vec![py_str("ipad"), py_str("tablet")]), py_tuple(vec![py_str("phone"), py_str("android"), py_str("samsung"), py_str("galaxy"), py_str("pixel"), py_str("handset")])]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_IN_DEVICE"); }
    s__CARD_IN_DEVICE(py_dict(vec![(Py::Int(0), g__CARD_IN_SWITCH()), (Py::Int(1), g__CARD_IN_IPHONE()), (Py::Int(2), g__CARD_IN_LAPTOP()), (Py::Int(3), g__CARD_IN_TABLET()), (Py::Int(4), g__CARD_IN_ANDROID())]));
    fn __data12() -> Py { py_tuple(vec![py_str("wont read"), py_str("won't read"), py_str("wont work"), py_str("won't work"), py_str("not reading"), py_str("not recognised"), py_str("not recognized"), py_str("unexpectedly removed"), py_str("not detected"), py_str("card error"), py_str("not showing"), py_str("doesnt show"), py_str("doesn't show"), py_str("not working"), py_str("stopped working"), py_str("keeps saying"), py_str("failed"), py_str("corrupt"), py_str("damaged"), py_str("unreadable"), py_str("cant read"), py_str("can't read")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_NOT_READ"); }
    s__CARD_NOT_READ(__data12());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_NOT_READING"); }
    s__CARD_NOT_READING(py_str("A card the phone cannot read is usually its own file system or its contacts rather than the phone: take it out, check the gold contacts are clean, and put it back with the phone off. If another device can read it, copy everything off now and format it in the phone (Settings > Storage > the card > Format); if nothing reads it, the card has failed and a new one is the only fix. A card that comes and goes is on its way out, so back it up first."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._CARD_FULL_IN_A_PHONE"); }
    s__CARD_FULL_IN_A_PHONE(py_str("Copy the photos and videos off it before you delete anything, because that is nearly all that is on it: plug the phone into a computer and drag them across, or let Google Photos upload them, then format the card in the phone under Settings > Storage. A bigger card is the real fix - 64GB or 128GB labelled A1 or A2 - and move the camera's settings so new photos and video save to the card."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._NO_SLOT"); }
    s__NO_SLOT(py_tuple(vec![py_str("no sd card slot"), py_str("no card slot"), py_str("no sd slot"), py_str("no slot"), py_str("doesnt have a slot"), py_str("doesn't have a slot"), py_str("without a card slot")]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._NO_CARD_SLOT"); }
    s__NO_CARD_SLOT(py_str("With no card slot the room has to come from the phone or the cloud: turn on Google Photos or iCloud Photos and let it back up, delete the videos you have watched and offload the apps you do not use, and let Photos keep small versions rather than originals. If it is photos on a card you need to move, a USB-C card reader or an OTG drive plugs into the charging port."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._DISK_WORDS"); }
    s__DISK_WORDS(py_tuple(vec![py_str("disk"), py_str("disc"), py_str("hard drive"), py_str("hard disk"), py_str("ssd"), py_str("disk space")]));
    fn __data13() -> Py { py_tuple(vec![py_str("sd card"), py_str("sdcard"), py_str("sd cards"), py_str("sd-cards"), py_str("micro sd"), py_str("microsd"), py_str("micro sd card"), py_str("memory card"), py_str("memory cards"), py_str("storage card"), py_str("flash card"), py_str("camera card"), py_str("photo card"), py_str("card in my camera"), py_str("card in the camera"), py_str("cards in my camera"), py_str("card for my camera"), py_str("card for the camera"), py_str("card slot"), py_str("memory card slot"), py_str("sd card slot"), py_str("microsd slot"), py_str("slot for a card")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SD_CARD"); }
    s_SD_CARD(__data13());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SIM_CARD"); }
    s_SIM_CARD(py_tuple(vec![py_str("sim card"), py_str("sim cards"), py_str("simcard"), py_str("simcards"), py_str("sim")]));
    fn __data14() -> Py { py_tuple(vec![py_str("full"), py_str("no room"), py_str("no space"), py_str("space"), py_str("storage"), py_str("free up"), py_str("clear"), py_str("delete"), py_str("copy"), py_str("move"), py_str("moving"), py_str("running out"), py_str("out of space"), py_str("fill up"), py_str("filled up"), py_str("filling up"), py_str("clean up"), py_str("sort out"), py_str("back up"), py_str("backup"), py_str("how much"), py_str("room left"), py_str("replace"), py_str("bigger"), py_str("buy")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.CARD_ROOM"); }
    s_CARD_ROOM(__data14());
    fn __data15() -> Py { py_tuple(vec![py_str("sim only"), py_str("sim-only"), py_str("sim deal"), py_str("sim deals"), py_str("sim contract"), py_str("esim"), py_str("e-sim"), py_str("pay monthly"), py_str("pay as you go"), py_str("tariff"), py_str("rolling contract"), py_str("cheapest sim"), py_str("best sim"), py_str("which sim"), py_str("new sim"), py_str("order a sim"), py_str("sim free"), py_str("sim-free"), py_str("unlock"), py_str("network"), py_str("port my number"), py_str("switch network"), py_str("data plan"), py_str("upgrade"), py_str("cancel"), py_str("price"), py_str("cost"), py_str("cheap"), py_str("deal")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SIM_NOT_THIS"); }
    s_SIM_NOT_THIS(__data15());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._FOOD_NOT_A_MACHINE"); }
    s__FOOD_NOT_A_MACHINE(py_tuple(vec![py_str("big mac"), py_str("mac and cheese"), py_str("mac cheese"), py_str("macaroni"), py_str("calorie"), py_str("calories"), py_str("burger"), py_str("burgers"), py_str("mcdonalds"), py_str("mcdonald"), py_str("kfc")]));
    fn __data16() -> Py { py_tuple(vec![py_str("location"), py_str("locations"), py_str("live location"), py_str("my location"), py_str("where i am"), py_str("where im"), py_str("where i'm"), py_str("where i was"), py_str("where i went"), py_str("where i've been"), py_str("where ive been"), py_str("whereabouts"), py_str("gps"), py_str("find me"), py_str("track my phone"), py_str("tracking my phone"), py_str("track my location"), py_str("tracking my location"), py_str("see where i am")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.WHERE_I_AM"); }
    s_WHERE_I_AM(__data16());
    fn __data17() -> Py { py_tuple(vec![py_str("share"), py_str("shares"), py_str("sharing"), py_str("send"), py_str("sends"), py_str("show"), py_str("let them see"), py_str("let him see"), py_str("let her see"), py_str("let someone see"), py_str("track"), py_str("tracking"), py_str("see where"), py_str("sees where"), py_str("know where"), py_str("knows where"), py_str("find where"), py_str("who can see"), py_str("can they see"), py_str("can he see"), py_str("can she see"), py_str("sees my location"), py_str("knows my location")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SHARE_IT"); }
    s_SHARE_IT(__data17());
    fn __data18() -> Py { py_tuple(vec![py_str("file"), py_str("files"), py_str("folder"), py_str("folders"), py_str("screen"), py_str("screenshare"), py_str("photos"), py_str("photo"), py_str("pictures"), py_str("picture"), py_str("album"), py_str("document"), py_str("documents"), py_str("dropbox"), py_str("google drive"), py_str("onedrive"), py_str("icloud"), py_str("memory card"), py_str("usb"), py_str("netflix"), py_str("spotify"), py_str("streaming"), py_str("password"), py_str("passwords"), py_str("account"), py_str("wifi"), py_str("wi-fi"), py_str("printer"), py_str("flat"), py_str("house"), py_str("room"), py_str("rooms"), py_str("bill"), py_str("bills"), py_str("rent"), py_str("money"), py_str("cost"), py_str("costs"), py_str("split"), py_str("custody"), py_str("car"), py_str("lift"), py_str("ride"), py_str("taxi"), py_str("cooking"), py_str("toilet"), py_str("bathroom"), py_str("police"), py_str("999"), py_str("911"), py_str("112"), py_str("ambulance"), py_str("fire brigade"), py_str("emergency"), py_str("emergency services"), py_str("coastguard"), py_str("mountain rescue")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.NOT_WHERE_I_AM"); }
    s_NOT_WHERE_I_AM(__data18());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._ASKS_TO_STOP_SHARING"); }
    s__ASKS_TO_STOP_SHARING(stdlib_call("re", "compile", &[py_str("\\b(?:stop|stops|stopped|stopping|quit|prevent)\\b[^.?!;]{0,48}\\b(?:shar(?:e|es|ed|ing)|send(?:s|ing)?|broadcast\\w*|location|gps|found|see|sees|seeing|seen|know|knows|knowing|tracked|tracking)\\b|\\b(?:turn|turns|turning|switch|switches|switching|shut|shutting)\\b[^.?!;]{0,20}\\b(?:off|out)\\b|\\b(?:hide|hides|hiding|hidden|disable|disables|disabling|withhold|withholding)\\b|\\b(?:don'?t|dont|do not|not)\\s+(?:want\\s+(?:to\\s+)?)?(?:shar(?:e|ing)|be\\s+(?:found|seen|tracked))\\b|\\b(?:don'?t|dont|do not|doesn'?t|not)\\s+(?:want|wants|wanted|like)\\b[^.?!;]{0,40}\\b(?:see|sees|seeing|know|knows|knowing|find|finds|finding|track|tracks|tracking|shar(?:e|ing))\\b|\\bwithout\\b[^.?!;]{0,32}\\b(?:knowing|seeing|finding\\s+out|being\\s+seen)\\b|\\b(?:nobody|no\\s+one|no-one|anyone|anybody)\\b[^.?!;]{0,24}\\b(?:can|could|should|will|would|must|to)\\b[^.?!;]{0,16}\\b(?:see|sees|know|knows|find|track)\\b|\\bkeep\\b[^.?!;]{0,24}\\b(?:private|secret|hidden|to\\s+myself)\\b|\\bwho\\s+(?:else\\s+)?(?:can|could)\\s+see\\b|\\bwho\\s+sees\\b|\\b(?:privacy|opt out|opting out|unshare|stop sharing)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._ASKS_HOW_IT_WORKS"); }
    s__ASKS_HOW_IT_WORKS(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:does|do|did|is|are|can)\\b[^.?!;]{0,24}\\b(?:gps|phone|iphone|android|it|this|that|app|maps?|location\\s+services?|tracking|signal|satellites?)\\b[^.?!;]{0,24}\\b(?:know|knows|work|works|worked|find|finds|track|tracks|get|gets|locate|locates)\\b|\\bwhat\\s+is\\s+gps\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data19() -> Py { py_tuple(vec![py_str("husband"), py_str("wife"), py_str("partner"), py_str("boyfriend"), py_str("girlfriend"), py_str("ex"), py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("son"), py_str("daughter"), py_str("brother"), py_str("sister"), py_str("boss"), py_str("friend"), py_str("friends"), py_str("someone"), py_str("somebody"), py_str("he"), py_str("she"), py_str("they"), py_str("them"), py_str("him"), py_str("her"), py_str("snapchat"), py_str("whatsapp"), py_str("instagram"), py_str("insta"), py_str("facebook"), py_str("messenger"), py_str("tiktok"), py_str("find my"), py_str("google"), py_str("life360"), py_str("app"), py_str("apps")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.WHO_KNOWS"); }
    s_WHO_KNOWS(__data19());
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._ASKS_HOW_THEY_KNOW"); }
    s__ASKS_HOW_THEY_KNOW(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\bhow\\s+(?:does|do|did|can|could|would|will)\\s+(?:my|our|the|this|that|an?)?\\s*\\b(?:"), &mcall(&py_str("|"), "join", &[g_WHO_KNOWS()])), &py_str(")\\b[^.?!;]{0,26}\\b(?:know|knows|knew|knowing|see|sees|saw|seeing|find|finds|found|found\\s+out|track|tracks|tracked|tracking|locate|locates|worked\\s+out|work\\s+out)\\b[^.?!;]{0,26}\\b(?:where\\s+(?:i|im|i'?m|we)\\s+(?:am|was|is|are|went|go|been|at|now|right\\s+now)|my\\s+location|my\\s+whereabouts)\\b")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._ASKS_WHETHER_TO_SHARE"); }
    s__ASKS_WHETHER_TO_SHARE(stdlib_call("re", "compile", &[py_str("\\b(?:should|shall|ought)\\s+(?:i|we)\\b|\\bis\\s+it\\s+(?:safe|wise|ok|okay|alright|a\\s+good\\s+idea|worth|sensible|risky)\\b|\\bwould\\s+it\\s+be\\b|\\bdo\\s+(?:i|we)\\s+need\\s+to\\b|\\bam\\s+i\\s+(?:safe|risky|wrong)\\b|\\bbetter\\s+to\\b|\\bany\\s+(?:risk|danger)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SHARE_STEPS"); }
    s_SHARE_STEPS(py_str("Share it from the app you already message each other in, because then it is live and they can see you moving. WhatsApp: open the chat, tap the + or the paperclip, choose Location, then Share live location and pick 15 minutes, 1 hour or 8 hours. iPhone: in Messages tap the person's name at the top, then Share My Location, or use Find My > People > Share My Location. Android: Google Maps > your picture > Location sharing > New share. It keeps updating until you stop it, so stop it when you are there."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.HOW_THEY_KNOW"); }
    s_HOW_THEY_KNOW(py_str("Somebody sees where you are because an app on the phone is sharing it, and sharing is the default in the ones people use to keep in touch: Find My on an iPhone and Google's location sharing on Android both keep reporting until they are turned off, and anyone in a family group, a shared account or a live-location message is already watching. Look at who is on the list rather than at the phone: on an iPhone Settings > Privacy & Security > Location Services, and in Google Maps your picture then Location sharing, both show who can see you and for how long, and each name can be removed from there. If nobody is on either list, a shared account or a car's app is the next place to look."));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._ASKS_WHO_CAN_SEE"); }
    s__ASKS_WHO_CAN_SEE(stdlib_call("re", "compile", &[py_str("\\b(?:can|could|does|do|did|is|are|has|have)\\b[^?.!]{0,30}\\b(?:see|sees|know|knows|show|shows|reveal|reveals|tell|tells|track|tracking|watch|watching)\\b[^?.!]{0,25}\\bwhere\\s+(?:i|we)\\s+(?:am|are|was|were)\\b|\\bwho\\s+can\\s+see\\b|\\bwho\\s+sees\\b|\\bwho\\s+knows\\s+where\\b|\\b(?:see|sees|know|knows|show|shows)\\b[^?.!]{0,20}\\bmy\\s+(?:location|whereabouts)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices._ASKS_FOR_THE_STEPS"); }
    s__ASKS_FOR_THE_STEPS(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:do|can|would|should)\\s+i\\b|\\bhow\\s+to\\b|\\blet\\s+(?:them|him|her|someone|somebody|my)\\b|\\bmake\\s+(?:them|him|her|it)\\b|\\bso\\s+(?:they|he|she)\\s+can\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    fn __data20() -> Py { py_tuple(vec![py_str("facebook"), py_str("fb"), py_str("instagram"), py_str("insta"), py_str("social media"), py_str("social"), py_str("friend"), py_str("friends"), py_str("someone"), py_str("somebody"), py_str("him"), py_str("her"), py_str("them"), py_str("my ex"), py_str("mum"), py_str("dad"), py_str("brother"), py_str("sister"), py_str("husband"), py_str("wife")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.SOCIAL"); }
    s_SOCIAL(__data20());
    fn __data21() -> Py { py_tuple(vec![py_str("number"), py_str("numbers"), py_str("call"), py_str("calls"), py_str("caller"), py_str("text"), py_str("texts"), py_str("sms"), py_str("spam"), py_str("phone"), py_str("iphone"), py_str("android"), py_str("whatsapp"), py_str("email"), py_str("e mail"), py_str("mail"), py_str("door"), py_str("road"), py_str("street"), py_str("pip"), py_str("drain"), py_str("chimney"), py_str("sink"), py_str("toilet"), py_str("wall"), py_str("garden"), py_str("parking"), py_str("drive"), py_str("artery"), py_str("arteries"), py_str("vein"), py_str("veins"), py_str("stent"), py_str("stents"), py_str("sinus"), py_str("sinuses"), py_str("airway"), py_str("bowel"), py_str("lung"), py_str("lungs"), py_str("catheter"), py_str("nose"), py_str("ear"), py_str("ears"), py_str("throat"), py_str("pipe"), py_str("pipes"), py_str("hose"), py_str("duct"), py_str("ducting"), py_str("gutter"), py_str("gully"), py_str("sewer"), py_str("filter"), py_str("filters"), py_str("blocked me"), py_str("blocks me"), py_str("has blocked me"), py_str("have blocked me"), py_str("they blocked me"), py_str("he blocked me"), py_str("she blocked me"), py_str("blocked my number"), py_str("blocked my account"), py_str("got blocked"), py_str("been blocked"), py_str("i was blocked"), py_str("i am blocked"), py_str("i got blocked")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.NOT_SOCIAL"); }
    s_NOT_SOCIAL(__data21());
    fn __data22() -> Py { py_tuple(vec![py_str("know where"), py_str("knows where"), py_str("knew where"), py_str("know where i am"), py_str("see where"), py_str("sees where"), py_str("saw where"), py_str("find where"), py_str("find out where"), py_str("finds out where"), py_str("found out where"), py_str("found where"), py_str("tracking where"), py_str("knows my location"), py_str("worked out where"), py_str("work out where")]) }
    fn __data23() -> Py { py_tuple(vec![py_str("unfriend"), py_str("unfriending"), py_str("unfriended"), py_str("defriend"), py_str("unfollow"), py_str("unfollowing"), py_str("remove"), py_str("block"), py_str("blocking"), py_str("blocked"), py_str("how"), py_str("can i"), py_str("do i")]) }
    fn __data24() -> Py { py_tuple(vec![py_str("windows"), py_str("pc"), py_str("pcs"), py_str("desktop"), py_str("desktops"), py_str("thinkpad"), py_str("chromebook"), py_str("surface"), py_str("wifi"), py_str("wi-fi"), py_str("signal"), py_str("bars"), py_str("internet"), py_str("broadband"), py_str("loads"), py_str("loading"), py_str("browser"), py_str("big mac"), py_str("big macs")]) }
    fn __data25() -> Py { py_tuple(vec![py_str("hdd"), py_str("hdd's"), py_str("hard drive"), py_str("hard disk"), py_str("spinning disk"), py_str("disk"), py_str("difference"), py_str("better"), py_str("faster"), py_str("slower"), py_str("worth"), py_str("upgrade"), py_str("vs"), py_str("versus"), py_str("compare"), py_str("choose"), py_str("buy"), py_str("replace")]) }
    fn __data26() -> Py { py_tuple(vec![py_str("difference"), py_str("better"), py_str("faster"), py_str("slower"), py_str("worth"), py_str("upgrade"), py_str("upgrading"), py_str("upgrad"), py_str("vs"), py_str("versus"), py_str("compare"), py_str("choose"), py_str("buy"), py_str("replace"), py_str("what"), py_str("which"), py_str("should")]) }
    fn __data27() -> Py { py_tuple(vec![py_str("credit card"), py_str("bank card"), py_str("debit card"), py_str("payment"), py_str("pay"), py_str("owe"), py_str("debt"), py_str("bill"), py_str("money"), py_str("borrow"), py_str("loan"), py_str("santander"), py_str("barclays"), py_str("halifax"), py_str("gift card"), py_str("voucher"), py_str("birthday card"), py_str("christmas card"), py_str("greeting card"), py_str("playing card"), py_str("card game"), py_str("loyalty card"), py_str("store card"), py_str("nectar"), py_str("clubcard"), py_str("format"), py_str("formatted"), py_str("formatting"), py_str("exfat"), py_str("fat32"), py_str("file system")]) }
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.ENTRIES"); }
    s_ENTRIES(py_list(vec![{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("share-where-i-am")); __args.push(py_tuple(vec![g_WHERE_I_AM(), g_SHARE_IT()])); __args.push(py_func("_format_sharing_where_i_am", |args: &[Py]| _format_sharing_where_i_am(args))); __kws.extend(vec![("ask".to_string(), g_SHARE_IT()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_not_a_sharing_ask", |args: &[Py]| _not_a_sharing_ask(args))), ("avoid".to_string(), g_NOT_WHERE_I_AM())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("how-they-know-where-i-am")); __args.push(py_tuple(vec![g_WHERE_I_AM(), op_add(&g_SHARE_IT(), &py_tuple(vec![py_str("find out where"), py_str("finds out where"), py_str("found out where"), py_str("find where"), py_str("found where"), py_str("knew where"), py_str("saw where"), py_str("work out where"), py_str("worked out where"), py_str("knows my location")]))])); __args.push(g_HOW_THEY_KNOW()); __kws.extend(vec![("ask".to_string(), __data22()), ("bias".to_string(), Py::Int(1)), ("guard".to_string(), py_func("_asks_how_they_know", |args: &[Py]| _asks_how_they_know(args))), ("avoid".to_string(), op_add(&g_NOT_WHERE_I_AM(), &py_tuple(vec![py_str("stop"), py_str("turn off"), py_str("hide"), py_str("dont want"), py_str("don't want"), py_str("without"), py_str("nobody"), py_str("no one"), py_str("private")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("unfriend-or-block-on-social")); __args.push(py_tuple(vec![py_tuple(vec![py_str("unfriend"), py_str("unfriending"), py_str("unfriended"), py_str("defriend"), py_str("unfollow"), py_str("unfollowing"), py_str("remove a friend"), py_str("remove friend"), py_str("remove them from my friends"), py_str("block"), py_str("blocking"), py_str("blocked")]), g_SOCIAL()])); __args.push(py_func("_unfriend_or_block", |args: &[Py]| _unfriend_or_block(args))); __kws.extend(vec![("ask".to_string(), __data23()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), g_NOT_SOCIAL())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("mac-storage-full")); __args.push(py_tuple(vec![g_MAC(), g_ROOM(), g_HOW_MUCH()])); __args.push(py_func("_mac_space", |args: &[Py]| _mac_space(args))); __kws.extend(vec![("ask".to_string(), g_MAC()), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_PHONE(), &g_OTHER()), &g_HOUSE()), &__data24()), &g_NOT_STORAGE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("pc-storage-full")); __args.push(py_tuple(vec![g_PC(), g_ROOM(), g_HOW_MUCH()])); __args.push(py_func("_pc_space", |args: &[Py]| _pc_space(args))); __kws.extend(vec![("ask".to_string(), g_PC_ASK()), ("bias".to_string(), Py::Int(1)), ("avoid".to_string(), op_add(&op_add(&op_add(&op_add(&g_PHONE(), &g_OTHER()), &g_HOUSE()), &g_MAC()), &g_NOT_STORAGE()))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("ssd-vs-hdd")); __args.push(py_tuple(vec![py_tuple(vec![py_str("ssd"), py_str("ssds"), py_str("solid state drive"), py_str("solid state"), py_str("nvme")]), __data25()])); __args.push(py_str("An SSD has no moving parts, so the machine starts in seconds and apps open at once; a hard drive is a spinning disc and only wins on price for each gigabyte. For a laptop the SSD is the one upgrade that makes an old machine feel new - a 500GB one is cheap now and cloning the old disk across is a job a shop will do in an hour. Buy a big hard drive only for bulk storage and backups.")); __kws.extend(vec![("ask".to_string(), __data26()), ("avoid".to_string(), py_tuple(vec![py_str("space"), py_str("full"), py_str("free up"), py_str("left"), py_str("how much"), py_str("how many"), py_str("storage"), py_str("make room"), py_str("no room"), py_str("running out"), py_str("clean up"), py_str("delete")]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sd-card-full")); __args.push(py_tuple(vec![g_SD_CARD(), op_add(&op_add(&g_CARD_ROOM(), &g__CARD_INTO_A_DEVICE()), &g__A_CARD_BEING_CHOSEN())])); __args.push(py_func("_sd_card", |args: &[Py]| _sd_card(args))); __kws.extend(vec![("ask".to_string(), op_add(&op_add(&op_add(&g_CARD_ROOM(), &g__CARD_INTO_A_DEVICE()), &g__A_CARD_BEING_CHOSEN()), &g__A_CARD_ASK())), ("bias".to_string(), Py::Int(3)), ("avoid".to_string(), __data27())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }, { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("sim-card-full")); __args.push(py_tuple(vec![g_SIM_CARD(), op_add(&g_CARD_ROOM(), &py_tuple(vec![py_str("messages"), py_str("texts"), py_str("contacts"), py_str("numbers")]))])); __args.push(py_str("A sim holds your number, your contacts and your texts, not photos, so what fills it is old messages. Delete the old texts first, and picture messages go first because they are big. Copy the contacts to the phone itself or to a Google or Apple account so they do not live on the card, then back up the texts you want to keep - a new sim does not bring the old messages with it.")); __kws.extend(vec![("ask".to_string(), op_add(&g_CARD_ROOM(), &py_tuple(vec![py_str("messages"), py_str("texts"), py_str("contacts"), py_str("numbers")]))), ("bias".to_string(), Py::Int(2)), ("avoid".to_string(), op_add(&g_SIM_NOT_THIS(), &py_tuple(vec![py_str("sim card slot"), py_str("sim tray"), py_str("no sim"), py_str("sim card is not"), py_str("insert sim"), py_str("sim not working")])))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| pyrt::xrefs::xcall("knowledge.lookup", "entry", args))(&__args) }]));
    if std::env::var_os("PYBOT_TRACE_INIT").is_some() { eprintln!("SET knowledge.devices.ENTRIES"); }
    s_ENTRIES({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_ENTRIES()) { let mut one: Py = __item.clone(); __out.push(_stands_down_for_food(&[one.clone()])); } py_list(__out) })() });
}
pub fn _the_device_of_the_card(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Which device the card is going into, or None when the line names no device.");
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[g__DEVICE_OF_THE_CARD()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    words = __unpk2[1].clone();
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return geti(&g__CARD_IN_DEVICE(), &index.clone());
    }
    }
    return Py::None;
    Py::None
}
pub fn _sd_card(args: &[Py]) -> Py {
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
    let mut in_a_device: Py = Py::None;
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A full card wants its photos copied off; a card being chosen wants a size and a speed.");
    text = _text(&[understood.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__NO_SLOT()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__NO_CARD_SLOT();
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__CARD_NOT_READ()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return g__CARD_NOT_READING();
    }
    in_a_device = _the_device_of_the_card(&[text.clone()]);
    if py_truthy(&in_a_device.clone()) {
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("full"), py_str("no room"), py_str("no space"), py_str("filled")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("phone"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("android"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("samsung"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("tablet"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("galaxy"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("pixel"))) } } } } } } } } } } } }) {
    return g__CARD_FULL_IN_A_PHONE();
    }
    return in_a_device.clone();
    }
    if py_truthy(&{ let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__BUYING_A_CARD()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("full"), py_str("no room"), py_str("no space"), py_str("running out")])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return g_SD_CARD_BUY();
    }
    return g_SD_CARD_COPY();
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
    let mut part: Py = Py::None;
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![mcall(&understood.clone(), "get", &[py_str("text")]), mcall(&understood.clone(), "get", &[py_str("clean")])])) { let mut part: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "lower", &[]);
    Py::None
}
pub fn _names_mac(args: &[Py]) -> Py {
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
    text = _text(&[understood.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_MAC()) { let mut word: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_contains(&op_add(&py_str(" "), &text.clone()), &op_add(&py_str(" "), &word.clone()))); if py_truthy(&__b) { __b } else { mcall(&text.clone(), "startswith", &[word.clone()]) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_pc(args: &[Py]) -> Py {
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
    text = _text(&[understood.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PC()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&word.clone(), &py_str("disk")))) {  __out.push({ let __b = Py::Bool(py_contains(&op_add(&py_str(" "), &text.clone()), &op_add(&py_str(" "), &word.clone()))); if py_truthy(&__b) { __b } else { mcall(&text.clone(), "startswith", &[word.clone()]) } }); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_a_disk(args: &[Py]) -> Py {
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
    text = _text(&[understood.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__DISK_WORDS()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asking_how_much(args: &[Py]) -> Py {
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
    text = _text(&[understood.clone()]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CHECK()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&text.clone(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _mac_space(args: &[Py]) -> Py {
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
    if py_truthy(&Py::Bool(!py_truthy(&_names_mac(&[understood.clone()])))) {
    return Py::None;
    }
    return if py_truthy(&_asking_how_much(&[understood.clone()])) { g_MAC_CHECK() } else { g_MAC_SPACE() };
    Py::None
}
pub fn _pc_space(args: &[Py]) -> Py {
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
    let mut how_much: Py = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _names_pc(&[understood.clone()]); if py_truthy(&__b) { __b } else { { let __b = _names_mac(&[understood.clone()]); if py_truthy(&__b) { __b } else { _names_a_disk(&[understood.clone()]) } } } }))) {
    return Py::None;
    }
    how_much = _asking_how_much(&[understood.clone()]);
    if py_truthy(&{ let __b = _names_mac(&[understood.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_pc(&[understood.clone()]))) } }) {
    return if py_truthy(&how_much.clone()) { g_MAC_CHECK() } else { g_MAC_SPACE() };
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_pc(&[understood.clone()])))) {
    return if py_truthy(&how_much.clone()) { g_BOTH_CHECK() } else { g_BOTH_SPACE() };
    }
    return if py_truthy(&how_much.clone()) { g_PC_CHECK() } else { g_PC_SPACE() };
    Py::None
}
pub fn _stands_down_for_food(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut one: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "one") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: one"))) };
    let mut part: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("A Mac entry, with the food words added to its stay-away list.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gattr(&one.clone(), "key")) { let mut part: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[part.clone(), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&part.clone(), &py_str("mac"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return one.clone();
    }
    let _ = py_setattr(&one.clone(), "avoid", op_add(&gattr(&one.clone(), "avoid"), &call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g__FOOD_NOT_A_MACHINE()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&gattr(&one.clone(), "avoid"), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    return one.clone();
    Py::None
}
pub fn _not_asking_to_stop(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line asks for sharing to stop, which the turn-off row answers.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__ASKS_TO_STOP_SHARING(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _the_said(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("The line's own words, from whichever reading a callable is handed.");
    if py_truthy(&Py::Bool(py_is(&understood.clone(), &Py::None))) {
    return py_str("");
    }
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean"), py_str("norm_text"), py_str("plain")]));
    for __item in __iter1 {
    key = __item.clone();
    if py_truthy(&mcall(&understood.clone(), "get", &[key.clone()])) {
    return call_builtin("str", &[geti(&understood.clone(), &key.clone())]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![py_str("text"), py_str("clean"), py_str("norm_text"), py_str("plain"), py_str("said")]));
    for __item in __iter2 {
    key = __item.clone();
    said = call_builtin("getattr", &[understood.clone(), key.clone(), py_str("")]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&said.clone()) {
    return call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return call_builtin("str", &[understood.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_whether_to_share(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks whether to share rather than how.");
    return call_builtin("bool", &[mcall(&g__ASKS_WHETHER_TO_SHARE(), "search", &[_the_said(&[understood.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_who_can_see(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks who can already see where the person is.");
    said = _the_said(&[understood.clone()]);
    if py_truthy(&mcall(&g__ASKS_FOR_THE_STEPS(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g__ASKS_WHO_CAN_SEE(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _format_sharing_where_i_am(args: &[Py]) -> Py {
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
    let _ = py_str("The steps for sharing where you are, or what the shape of the share is when the line asks\n    whether to. \"should i share my location on a dating app\" is a judgement, and it used to reach\n    a dating-invitation card, and \"is it safe to share my location with someone i met online\" a\n    one-off virtual card, both of them other rows reading the word \"share\": a how-to does not\n    answer a whether-to. The setting decides how much a person sees, not whether to trust them,\n    so say that much and leave the weighing to them.");
    if py_truthy(&_asks_who_can_see(&[understood.clone()])) {
    return g_HOW_THEY_KNOW();
    }
    if py_truthy(&_asks_whether_to_share(&[understood.clone()])) {
    return py_str("That is yours to weigh, and the setting decides how much they see rather than whether to trust them: live sharing keeps updating for as long as the timer runs, so they watch you move and still have it afterwards, while a location sent once says only where you were at that moment. In WhatsApp tap the + or the paperclip, choose Location, then Send your current location for the point or Share live location for 15 minutes, 1 hour or 8 hours; Google Maps, your picture then Location sharing, lists every share you have running and stops any of them.");
    }
    return g_SHARE_STEPS();
    Py::None
}
pub fn _not_asking_whether_to_share(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line asks whether to share rather than how.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__ASKS_WHETHER_TO_SHARE(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _not_asking_how_they_know(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line asks how somebody came to know where the person is.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__ASKS_HOW_THEY_KNOW(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _asks_how_they_know(args: &[Py]) -> Py {
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
    let _ = py_str("True only when the line asks how somebody came to know where the person is.");
    return Py::Bool(!py_truthy(&_not_asking_how_they_know(&[understood.clone()])));
    Py::None
}
pub fn _not_asking_how_it_works(args: &[Py]) -> Py {
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
    let _ = py_str("True unless the line asks how the phone knows, which the satellites card answers.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(true);
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]);
    }
    return Py::Bool(!py_truthy(&mcall(&g__ASKS_HOW_IT_WORKS(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))])));
    Py::None
}
pub fn _not_a_sharing_ask(args: &[Py]) -> Py {
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
    let _ = py_str("The stand-downs the sharing row needs: not the opposite ask, not a how-do-they-know,\n    and not the how-it-works one.\n\n    \"who can see my location\" is the one exception to the first: it says nothing about turning\n    anything on, and the card that answers it is the one that lists who can already see and how\n    to take each name off, which is the same help the turn-off row gives from the other end.\n    ");
    if py_truthy(&_asks_who_can_see(&[understood.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = _not_asking_to_stop(&[understood.clone()]); if !py_truthy(&__b) { __b } else { { let __b = _not_asking_how_they_know(&[understood.clone()]); if !py_truthy(&__b) { __b } else { _not_asking_how_it_works(&[understood.clone()]) } } } };
    Py::None
}
pub fn _unfriend_or_block(args: &[Py]) -> Py {
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
    let mut block: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unfollow: Py = Py::None;
    let mut unfriend: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The button that was asked for first, then the other one, then how far it goes.\n\n    \"how do i unfollow my ex\" was led with Unfriend, which is one step further than the\n    question and ends the friendship, so the verb the person used decides the order.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = { let __b = mcall(&understood.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&understood.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } } } };
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&understood.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    line = mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    unfollow = stdlib_call("re", "search", &[py_str("\\bunfollow"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    block = stdlib_call("re", "search", &[py_str("\\bblock(?:ed|ing)?\\b"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    unfriend = stdlib_call("re", "search", &[py_str("\\b(?:unfriend|defriend|remove (?:a )?friend)"), line.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = unfollow.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&unfriend.clone())) } }) {
    return py_str("On Facebook open their profile, tap Following, then Unfollow - they are not told, and it takes their posts out of your feed without ending the friendship. On Instagram it is the same two taps: their profile, then Following, then Unfollow. If you want them gone further than that, the ... menu on their profile has Block, which stops them finding you, messaging you or seeing anything you post, and you can undo it in Settings > Blocked.");
    }
    if py_truthy(&{ let __b = block.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&unfriend.clone())) } }) {
    return py_str("Blocking stops them finding you, messaging you or seeing anything you post: open their profile, tap the ... menu, then Block. They are not told, and you can undo it in Settings > Blocked. Unfriending is the smaller step - Friends, then Unfriend - which stops the posts but still lets them message you.");
    }
    return py_str("On Facebook open their profile, tap Friends, then Unfriend - they are not told. Blocking goes further: the ... menu on their profile, then Block. On Instagram open their profile, tap Following to Unfollow, or the ... menu to Block. Unfriending only stops the posts; blocking stops them finding you, messaging you or seeing anything you post, and you can undo it in Settings > Blocked.");
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.knowledge.devices".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ANSWERS" => Some(g_ANSWERS()),
            "ASK" => Some(g_ASK()),
            "BOTH_CHECK" => Some(g_BOTH_CHECK()),
            "BOTH_SPACE" => Some(g_BOTH_SPACE()),
            "CARD_ROOM" => Some(g_CARD_ROOM()),
            "CHECK" => Some(g_CHECK()),
            "ENTRIES" => Some(g_ENTRIES()),
            "GLASS" => Some(g_GLASS()),
            "HOUSE" => Some(g_HOUSE()),
            "HOW_MUCH" => Some(g_HOW_MUCH()),
            "HOW_THEY_KNOW" => Some(g_HOW_THEY_KNOW()),
            "MAC" => Some(g_MAC()),
            "MAC_CHECK" => Some(g_MAC_CHECK()),
            "MAC_SPACE" => Some(g_MAC_SPACE()),
            "NOT_SOCIAL" => Some(g_NOT_SOCIAL()),
            "NOT_STORAGE" => Some(g_NOT_STORAGE()),
            "NOT_WHERE_I_AM" => Some(g_NOT_WHERE_I_AM()),
            "OTHER" => Some(g_OTHER()),
            "PC" => Some(g_PC()),
            "PC_ASK" => Some(g_PC_ASK()),
            "PC_CHECK" => Some(g_PC_CHECK()),
            "PC_SPACE" => Some(g_PC_SPACE()),
            "PHONE" => Some(g_PHONE()),
            "ROOM" => Some(g_ROOM()),
            "SD_CARD" => Some(g_SD_CARD()),
            "SD_CARD_BUY" => Some(g_SD_CARD_BUY()),
            "SD_CARD_COPY" => Some(g_SD_CARD_COPY()),
            "SHARE_IT" => Some(g_SHARE_IT()),
            "SHARE_STEPS" => Some(g_SHARE_STEPS()),
            "SIM_CARD" => Some(g_SIM_CARD()),
            "SIM_NOT_THIS" => Some(g_SIM_NOT_THIS()),
            "SOCIAL" => Some(g_SOCIAL()),
            "WHERE_I_AM" => Some(g_WHERE_I_AM()),
            "WHO_KNOWS" => Some(g_WHO_KNOWS()),
            "_ASKS_FOR_THE_STEPS" => Some(g__ASKS_FOR_THE_STEPS()),
            "_ASKS_HOW_IT_WORKS" => Some(g__ASKS_HOW_IT_WORKS()),
            "_ASKS_HOW_THEY_KNOW" => Some(g__ASKS_HOW_THEY_KNOW()),
            "_ASKS_TO_STOP_SHARING" => Some(g__ASKS_TO_STOP_SHARING()),
            "_ASKS_WHETHER_TO_SHARE" => Some(g__ASKS_WHETHER_TO_SHARE()),
            "_ASKS_WHO_CAN_SEE" => Some(g__ASKS_WHO_CAN_SEE()),
            "_A_CARD_ASK" => Some(g__A_CARD_ASK()),
            "_A_CARD_BEING_CHOSEN" => Some(g__A_CARD_BEING_CHOSEN()),
            "_BUYING_A_CARD" => Some(g__BUYING_A_CARD()),
            "_CARD_FULL_IN_A_PHONE" => Some(g__CARD_FULL_IN_A_PHONE()),
            "_CARD_INTO_A_DEVICE" => Some(g__CARD_INTO_A_DEVICE()),
            "_CARD_IN_ANDROID" => Some(g__CARD_IN_ANDROID()),
            "_CARD_IN_DEVICE" => Some(g__CARD_IN_DEVICE()),
            "_CARD_IN_IPHONE" => Some(g__CARD_IN_IPHONE()),
            "_CARD_IN_LAPTOP" => Some(g__CARD_IN_LAPTOP()),
            "_CARD_IN_SWITCH" => Some(g__CARD_IN_SWITCH()),
            "_CARD_IN_TABLET" => Some(g__CARD_IN_TABLET()),
            "_CARD_NOT_READ" => Some(g__CARD_NOT_READ()),
            "_CARD_NOT_READING" => Some(g__CARD_NOT_READING()),
            "_DEVICE_OF_THE_CARD" => Some(g__DEVICE_OF_THE_CARD()),
            "_DISK_WORDS" => Some(g__DISK_WORDS()),
            "_FOOD_NOT_A_MACHINE" => Some(g__FOOD_NOT_A_MACHINE()),
            "_NO_CARD_SLOT" => Some(g__NO_CARD_SLOT()),
            "_NO_SLOT" => Some(g__NO_SLOT()),
            "_asking_how_much" => Some(py_func("_asking_how_much", |args: &[Py]| _asking_how_much(args))),
            "_asks_how_they_know" => Some(py_func("_asks_how_they_know", |args: &[Py]| _asks_how_they_know(args))),
            "_asks_whether_to_share" => Some(py_func("_asks_whether_to_share", |args: &[Py]| _asks_whether_to_share(args))),
            "_asks_who_can_see" => Some(py_func("_asks_who_can_see", |args: &[Py]| _asks_who_can_see(args))),
            "_format_sharing_where_i_am" => Some(py_func("_format_sharing_where_i_am", |args: &[Py]| _format_sharing_where_i_am(args))),
            "_mac_space" => Some(py_func("_mac_space", |args: &[Py]| _mac_space(args))),
            "_names_a_disk" => Some(py_func("_names_a_disk", |args: &[Py]| _names_a_disk(args))),
            "_names_mac" => Some(py_func("_names_mac", |args: &[Py]| _names_mac(args))),
            "_names_pc" => Some(py_func("_names_pc", |args: &[Py]| _names_pc(args))),
            "_not_a_sharing_ask" => Some(py_func("_not_a_sharing_ask", |args: &[Py]| _not_a_sharing_ask(args))),
            "_not_asking_how_it_works" => Some(py_func("_not_asking_how_it_works", |args: &[Py]| _not_asking_how_it_works(args))),
            "_not_asking_how_they_know" => Some(py_func("_not_asking_how_they_know", |args: &[Py]| _not_asking_how_they_know(args))),
            "_not_asking_to_stop" => Some(py_func("_not_asking_to_stop", |args: &[Py]| _not_asking_to_stop(args))),
            "_not_asking_whether_to_share" => Some(py_func("_not_asking_whether_to_share", |args: &[Py]| _not_asking_whether_to_share(args))),
            "_pc_space" => Some(py_func("_pc_space", |args: &[Py]| _pc_space(args))),
            "_sd_card" => Some(py_func("_sd_card", |args: &[Py]| _sd_card(args))),
            "_stands_down_for_food" => Some(py_func("_stands_down_for_food", |args: &[Py]| _stands_down_for_food(args))),
            "_text" => Some(py_func("_text", |args: &[Py]| _text(args))),
            "_the_device_of_the_card" => Some(py_func("_the_device_of_the_card", |args: &[Py]| _the_device_of_the_card(args))),
            "_the_said" => Some(py_func("_the_said", |args: &[Py]| _the_said(args))),
            "_unfriend_or_block" => Some(py_func("_unfriend_or_block", |args: &[Py]| _unfriend_or_block(args))),
            "entry" => Some(g_entry()),
            "re" => Some(stdlib_module("re")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ANSWERS" => s_ANSWERS(v),
            "ASK" => s_ASK(v),
            "BOTH_CHECK" => s_BOTH_CHECK(v),
            "BOTH_SPACE" => s_BOTH_SPACE(v),
            "CARD_ROOM" => s_CARD_ROOM(v),
            "CHECK" => s_CHECK(v),
            "ENTRIES" => s_ENTRIES(v),
            "GLASS" => s_GLASS(v),
            "HOUSE" => s_HOUSE(v),
            "HOW_MUCH" => s_HOW_MUCH(v),
            "HOW_THEY_KNOW" => s_HOW_THEY_KNOW(v),
            "MAC" => s_MAC(v),
            "MAC_CHECK" => s_MAC_CHECK(v),
            "MAC_SPACE" => s_MAC_SPACE(v),
            "NOT_SOCIAL" => s_NOT_SOCIAL(v),
            "NOT_STORAGE" => s_NOT_STORAGE(v),
            "NOT_WHERE_I_AM" => s_NOT_WHERE_I_AM(v),
            "OTHER" => s_OTHER(v),
            "PC" => s_PC(v),
            "PC_ASK" => s_PC_ASK(v),
            "PC_CHECK" => s_PC_CHECK(v),
            "PC_SPACE" => s_PC_SPACE(v),
            "PHONE" => s_PHONE(v),
            "ROOM" => s_ROOM(v),
            "SD_CARD" => s_SD_CARD(v),
            "SD_CARD_BUY" => s_SD_CARD_BUY(v),
            "SD_CARD_COPY" => s_SD_CARD_COPY(v),
            "SHARE_IT" => s_SHARE_IT(v),
            "SHARE_STEPS" => s_SHARE_STEPS(v),
            "SIM_CARD" => s_SIM_CARD(v),
            "SIM_NOT_THIS" => s_SIM_NOT_THIS(v),
            "SOCIAL" => s_SOCIAL(v),
            "WHERE_I_AM" => s_WHERE_I_AM(v),
            "WHO_KNOWS" => s_WHO_KNOWS(v),
            "_ASKS_FOR_THE_STEPS" => s__ASKS_FOR_THE_STEPS(v),
            "_ASKS_HOW_IT_WORKS" => s__ASKS_HOW_IT_WORKS(v),
            "_ASKS_HOW_THEY_KNOW" => s__ASKS_HOW_THEY_KNOW(v),
            "_ASKS_TO_STOP_SHARING" => s__ASKS_TO_STOP_SHARING(v),
            "_ASKS_WHETHER_TO_SHARE" => s__ASKS_WHETHER_TO_SHARE(v),
            "_ASKS_WHO_CAN_SEE" => s__ASKS_WHO_CAN_SEE(v),
            "_A_CARD_ASK" => s__A_CARD_ASK(v),
            "_A_CARD_BEING_CHOSEN" => s__A_CARD_BEING_CHOSEN(v),
            "_BUYING_A_CARD" => s__BUYING_A_CARD(v),
            "_CARD_FULL_IN_A_PHONE" => s__CARD_FULL_IN_A_PHONE(v),
            "_CARD_INTO_A_DEVICE" => s__CARD_INTO_A_DEVICE(v),
            "_CARD_IN_ANDROID" => s__CARD_IN_ANDROID(v),
            "_CARD_IN_DEVICE" => s__CARD_IN_DEVICE(v),
            "_CARD_IN_IPHONE" => s__CARD_IN_IPHONE(v),
            "_CARD_IN_LAPTOP" => s__CARD_IN_LAPTOP(v),
            "_CARD_IN_SWITCH" => s__CARD_IN_SWITCH(v),
            "_CARD_IN_TABLET" => s__CARD_IN_TABLET(v),
            "_CARD_NOT_READ" => s__CARD_NOT_READ(v),
            "_CARD_NOT_READING" => s__CARD_NOT_READING(v),
            "_DEVICE_OF_THE_CARD" => s__DEVICE_OF_THE_CARD(v),
            "_DISK_WORDS" => s__DISK_WORDS(v),
            "_FOOD_NOT_A_MACHINE" => s__FOOD_NOT_A_MACHINE(v),
            "_NO_CARD_SLOT" => s__NO_CARD_SLOT(v),
            "_NO_SLOT" => s__NO_SLOT(v),
            "entry" => s_entry(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("knowledge.devices", module_obj);
    pyrt::xrefs::reg_fn("knowledge.devices", "_the_device_of_the_card", py_func("_the_device_of_the_card", |args: &[Py]| _the_device_of_the_card(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_sd_card", py_func("_sd_card", |args: &[Py]| _sd_card(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_text", py_func("_text", |args: &[Py]| _text(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_names_mac", py_func("_names_mac", |args: &[Py]| _names_mac(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_names_pc", py_func("_names_pc", |args: &[Py]| _names_pc(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_names_a_disk", py_func("_names_a_disk", |args: &[Py]| _names_a_disk(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_asking_how_much", py_func("_asking_how_much", |args: &[Py]| _asking_how_much(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_mac_space", py_func("_mac_space", |args: &[Py]| _mac_space(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_pc_space", py_func("_pc_space", |args: &[Py]| _pc_space(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_stands_down_for_food", py_func("_stands_down_for_food", |args: &[Py]| _stands_down_for_food(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_not_asking_to_stop", py_func("_not_asking_to_stop", |args: &[Py]| _not_asking_to_stop(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_the_said", py_func("_the_said", |args: &[Py]| _the_said(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_asks_whether_to_share", py_func("_asks_whether_to_share", |args: &[Py]| _asks_whether_to_share(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_asks_who_can_see", py_func("_asks_who_can_see", |args: &[Py]| _asks_who_can_see(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_format_sharing_where_i_am", py_func("_format_sharing_where_i_am", |args: &[Py]| _format_sharing_where_i_am(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_not_asking_whether_to_share", py_func("_not_asking_whether_to_share", |args: &[Py]| _not_asking_whether_to_share(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_not_asking_how_they_know", py_func("_not_asking_how_they_know", |args: &[Py]| _not_asking_how_they_know(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_asks_how_they_know", py_func("_asks_how_they_know", |args: &[Py]| _asks_how_they_know(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_not_asking_how_it_works", py_func("_not_asking_how_it_works", |args: &[Py]| _not_asking_how_it_works(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_not_a_sharing_ask", py_func("_not_a_sharing_ask", |args: &[Py]| _not_a_sharing_ask(args)));
    pyrt::xrefs::reg_fn("knowledge.devices", "_unfriend_or_block", py_func("_unfriend_or_block", |args: &[Py]| _unfriend_or_block(args)));
    pyrt::xrefs::reg_access("knowledge.devices", "ANSWERS", g_ANSWERS, s_ANSWERS);
    pyrt::xrefs::reg_access("knowledge.devices", "ASK", g_ASK, s_ASK);
    pyrt::xrefs::reg_access("knowledge.devices", "BOTH_CHECK", g_BOTH_CHECK, s_BOTH_CHECK);
    pyrt::xrefs::reg_access("knowledge.devices", "BOTH_SPACE", g_BOTH_SPACE, s_BOTH_SPACE);
    pyrt::xrefs::reg_access("knowledge.devices", "CARD_ROOM", g_CARD_ROOM, s_CARD_ROOM);
    pyrt::xrefs::reg_access("knowledge.devices", "CHECK", g_CHECK, s_CHECK);
    pyrt::xrefs::reg_access("knowledge.devices", "ENTRIES", g_ENTRIES, s_ENTRIES);
    pyrt::xrefs::reg_access("knowledge.devices", "GLASS", g_GLASS, s_GLASS);
    pyrt::xrefs::reg_access("knowledge.devices", "HOUSE", g_HOUSE, s_HOUSE);
    pyrt::xrefs::reg_access("knowledge.devices", "HOW_MUCH", g_HOW_MUCH, s_HOW_MUCH);
    pyrt::xrefs::reg_access("knowledge.devices", "HOW_THEY_KNOW", g_HOW_THEY_KNOW, s_HOW_THEY_KNOW);
    pyrt::xrefs::reg_access("knowledge.devices", "MAC", g_MAC, s_MAC);
    pyrt::xrefs::reg_access("knowledge.devices", "MAC_CHECK", g_MAC_CHECK, s_MAC_CHECK);
    pyrt::xrefs::reg_access("knowledge.devices", "MAC_SPACE", g_MAC_SPACE, s_MAC_SPACE);
    pyrt::xrefs::reg_access("knowledge.devices", "NOT_SOCIAL", g_NOT_SOCIAL, s_NOT_SOCIAL);
    pyrt::xrefs::reg_access("knowledge.devices", "NOT_STORAGE", g_NOT_STORAGE, s_NOT_STORAGE);
    pyrt::xrefs::reg_access("knowledge.devices", "NOT_WHERE_I_AM", g_NOT_WHERE_I_AM, s_NOT_WHERE_I_AM);
    pyrt::xrefs::reg_access("knowledge.devices", "OTHER", g_OTHER, s_OTHER);
    pyrt::xrefs::reg_access("knowledge.devices", "PC", g_PC, s_PC);
    pyrt::xrefs::reg_access("knowledge.devices", "PC_ASK", g_PC_ASK, s_PC_ASK);
    pyrt::xrefs::reg_access("knowledge.devices", "PC_CHECK", g_PC_CHECK, s_PC_CHECK);
    pyrt::xrefs::reg_access("knowledge.devices", "PC_SPACE", g_PC_SPACE, s_PC_SPACE);
    pyrt::xrefs::reg_access("knowledge.devices", "PHONE", g_PHONE, s_PHONE);
    pyrt::xrefs::reg_access("knowledge.devices", "ROOM", g_ROOM, s_ROOM);
    pyrt::xrefs::reg_access("knowledge.devices", "SD_CARD", g_SD_CARD, s_SD_CARD);
    pyrt::xrefs::reg_access("knowledge.devices", "SD_CARD_BUY", g_SD_CARD_BUY, s_SD_CARD_BUY);
    pyrt::xrefs::reg_access("knowledge.devices", "SD_CARD_COPY", g_SD_CARD_COPY, s_SD_CARD_COPY);
    pyrt::xrefs::reg_access("knowledge.devices", "SHARE_IT", g_SHARE_IT, s_SHARE_IT);
    pyrt::xrefs::reg_access("knowledge.devices", "SHARE_STEPS", g_SHARE_STEPS, s_SHARE_STEPS);
    pyrt::xrefs::reg_access("knowledge.devices", "SIM_CARD", g_SIM_CARD, s_SIM_CARD);
    pyrt::xrefs::reg_access("knowledge.devices", "SIM_NOT_THIS", g_SIM_NOT_THIS, s_SIM_NOT_THIS);
    pyrt::xrefs::reg_access("knowledge.devices", "SOCIAL", g_SOCIAL, s_SOCIAL);
    pyrt::xrefs::reg_access("knowledge.devices", "WHERE_I_AM", g_WHERE_I_AM, s_WHERE_I_AM);
    pyrt::xrefs::reg_access("knowledge.devices", "WHO_KNOWS", g_WHO_KNOWS, s_WHO_KNOWS);
    pyrt::xrefs::reg_access("knowledge.devices", "_ASKS_FOR_THE_STEPS", g__ASKS_FOR_THE_STEPS, s__ASKS_FOR_THE_STEPS);
    pyrt::xrefs::reg_access("knowledge.devices", "_ASKS_HOW_IT_WORKS", g__ASKS_HOW_IT_WORKS, s__ASKS_HOW_IT_WORKS);
    pyrt::xrefs::reg_access("knowledge.devices", "_ASKS_HOW_THEY_KNOW", g__ASKS_HOW_THEY_KNOW, s__ASKS_HOW_THEY_KNOW);
    pyrt::xrefs::reg_access("knowledge.devices", "_ASKS_TO_STOP_SHARING", g__ASKS_TO_STOP_SHARING, s__ASKS_TO_STOP_SHARING);
    pyrt::xrefs::reg_access("knowledge.devices", "_ASKS_WHETHER_TO_SHARE", g__ASKS_WHETHER_TO_SHARE, s__ASKS_WHETHER_TO_SHARE);
    pyrt::xrefs::reg_access("knowledge.devices", "_ASKS_WHO_CAN_SEE", g__ASKS_WHO_CAN_SEE, s__ASKS_WHO_CAN_SEE);
    pyrt::xrefs::reg_access("knowledge.devices", "_A_CARD_ASK", g__A_CARD_ASK, s__A_CARD_ASK);
    pyrt::xrefs::reg_access("knowledge.devices", "_A_CARD_BEING_CHOSEN", g__A_CARD_BEING_CHOSEN, s__A_CARD_BEING_CHOSEN);
    pyrt::xrefs::reg_access("knowledge.devices", "_BUYING_A_CARD", g__BUYING_A_CARD, s__BUYING_A_CARD);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_FULL_IN_A_PHONE", g__CARD_FULL_IN_A_PHONE, s__CARD_FULL_IN_A_PHONE);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_INTO_A_DEVICE", g__CARD_INTO_A_DEVICE, s__CARD_INTO_A_DEVICE);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_IN_ANDROID", g__CARD_IN_ANDROID, s__CARD_IN_ANDROID);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_IN_DEVICE", g__CARD_IN_DEVICE, s__CARD_IN_DEVICE);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_IN_IPHONE", g__CARD_IN_IPHONE, s__CARD_IN_IPHONE);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_IN_LAPTOP", g__CARD_IN_LAPTOP, s__CARD_IN_LAPTOP);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_IN_SWITCH", g__CARD_IN_SWITCH, s__CARD_IN_SWITCH);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_IN_TABLET", g__CARD_IN_TABLET, s__CARD_IN_TABLET);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_NOT_READ", g__CARD_NOT_READ, s__CARD_NOT_READ);
    pyrt::xrefs::reg_access("knowledge.devices", "_CARD_NOT_READING", g__CARD_NOT_READING, s__CARD_NOT_READING);
    pyrt::xrefs::reg_access("knowledge.devices", "_DEVICE_OF_THE_CARD", g__DEVICE_OF_THE_CARD, s__DEVICE_OF_THE_CARD);
    pyrt::xrefs::reg_access("knowledge.devices", "_DISK_WORDS", g__DISK_WORDS, s__DISK_WORDS);
    pyrt::xrefs::reg_access("knowledge.devices", "_FOOD_NOT_A_MACHINE", g__FOOD_NOT_A_MACHINE, s__FOOD_NOT_A_MACHINE);
    pyrt::xrefs::reg_access("knowledge.devices", "_NO_CARD_SLOT", g__NO_CARD_SLOT, s__NO_CARD_SLOT);
    pyrt::xrefs::reg_access("knowledge.devices", "_NO_SLOT", g__NO_SLOT, s__NO_SLOT);
    pyrt::xrefs::reg_access("knowledge.devices", "entry", g_entry, s_entry);
}

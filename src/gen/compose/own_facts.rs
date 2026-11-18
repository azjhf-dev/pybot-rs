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


pub static ABOUT_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AS_THE_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_HAND_ON_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RECORD_OF_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHAT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FREQUENCY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static GENERIC_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAVEN_T_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_MANY_GOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_OFTEN_MINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_OLD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HOW_TO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEARNING_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NOUN_PHRASE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_RECORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_RECORD_BUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_OTHER_CHATS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_TRAINING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_PAYSLIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_ROTA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_THAN_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OWN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAY_DAY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAY_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PROPERTY_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROTA_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ROTA_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_IN_A_PAST_CHAT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_POOL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STOP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAUGHT_BY_THIS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_OWN_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_OTHER_CHATS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_MY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_AUX_MY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_I: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_MY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHERE_MY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _THE_PLACE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_BOT() -> Py { ensure_init(); ABOUT_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_BOT(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_BOT.write() { *w = v; } }
pub fn g_AS_THE_QUESTION() -> Py { ensure_init(); AS_THE_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AS_THE_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = AS_THE_QUESTION.write() { *w = v; } }
pub fn g_A_HAND_ON_THEM() -> Py { ensure_init(); A_HAND_ON_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_HAND_ON_THEM(v: Py) { ensure_init(); if let Ok(mut w) = A_HAND_ON_THEM.write() { *w = v; } }
pub fn g_A_RATE() -> Py { ensure_init(); A_RATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RATE(v: Py) { ensure_init(); if let Ok(mut w) = A_RATE.write() { *w = v; } }
pub fn g_A_RECORD_OF_THEM() -> Py { ensure_init(); A_RECORD_OF_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RECORD_OF_THEM(v: Py) { ensure_init(); if let Ok(mut w) = A_RECORD_OF_THEM.write() { *w = v; } }
pub fn g_CHAT_WORDS() -> Py { ensure_init(); CHAT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHAT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = CHAT_WORDS.write() { *w = v; } }
pub fn g_FREQUENCY_ASK() -> Py { ensure_init(); FREQUENCY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FREQUENCY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = FREQUENCY_ASK.write() { *w = v; } }
pub fn g_GENERIC_WORDS() -> Py { ensure_init(); GENERIC_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_GENERIC_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = GENERIC_WORDS.write() { *w = v; } }
pub fn g_HAVEN_T_SAID() -> Py { ensure_init(); HAVEN_T_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAVEN_T_SAID(v: Py) { ensure_init(); if let Ok(mut w) = HAVEN_T_SAID.write() { *w = v; } }
pub fn g_HOW_MANY() -> Py { ensure_init(); HOW_MANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MANY.write() { *w = v; } }
pub fn g_HOW_MANY_GOT() -> Py { ensure_init(); HOW_MANY_GOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_MANY_GOT(v: Py) { ensure_init(); if let Ok(mut w) = HOW_MANY_GOT.write() { *w = v; } }
pub fn g_HOW_OFTEN_MINE() -> Py { ensure_init(); HOW_OFTEN_MINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_OFTEN_MINE(v: Py) { ensure_init(); if let Ok(mut w) = HOW_OFTEN_MINE.write() { *w = v; } }
pub fn g_HOW_OLD() -> Py { ensure_init(); HOW_OLD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_OLD(v: Py) { ensure_init(); if let Ok(mut w) = HOW_OLD.write() { *w = v; } }
pub fn g_HOW_TO() -> Py { ensure_init(); HOW_TO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HOW_TO(v: Py) { ensure_init(); if let Ok(mut w) = HOW_TO.write() { *w = v; } }
pub fn g_LEARNING_ASK() -> Py { ensure_init(); LEARNING_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEARNING_ASK(v: Py) { ensure_init(); if let Ok(mut w) = LEARNING_ASK.write() { *w = v; } }
pub fn g_NOT_A_NOUN_PHRASE() -> Py { ensure_init(); NOT_A_NOUN_PHRASE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NOUN_PHRASE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NOUN_PHRASE.write() { *w = v; } }
pub fn g_NOT_A_RECORD() -> Py { ensure_init(); NOT_A_RECORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RECORD(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_RECORD.write() { *w = v; } }
pub fn g_NOT_A_RECORD_BUT() -> Py { ensure_init(); NOT_A_RECORD_BUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_RECORD_BUT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_RECORD_BUT.write() { *w = v; } }
pub fn g_NO_OTHER_CHATS() -> Py { ensure_init(); NO_OTHER_CHATS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_OTHER_CHATS(v: Py) { ensure_init(); if let Ok(mut w) = NO_OTHER_CHATS.write() { *w = v; } }
pub fn g_NO_TRAINING() -> Py { ensure_init(); NO_TRAINING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_TRAINING(v: Py) { ensure_init(); if let Ok(mut w) = NO_TRAINING.write() { *w = v; } }
pub fn g_ON_THE_PAYSLIP() -> Py { ensure_init(); ON_THE_PAYSLIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_PAYSLIP(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_PAYSLIP.write() { *w = v; } }
pub fn g_ON_THE_ROTA() -> Py { ensure_init(); ON_THE_ROTA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_ROTA(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_ROTA.write() { *w = v; } }
pub fn g_OTHER_THAN_THIS() -> Py { ensure_init(); OTHER_THAN_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_THAN_THIS(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_THAN_THIS.write() { *w = v; } }
pub fn g_OWN_WORDS() -> Py { ensure_init(); OWN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OWN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = OWN_WORDS.write() { *w = v; } }
pub fn g_PAY_DAY() -> Py { ensure_init(); PAY_DAY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAY_DAY(v: Py) { ensure_init(); if let Ok(mut w) = PAY_DAY.write() { *w = v; } }
pub fn g_PAY_WORDS() -> Py { ensure_init(); PAY_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAY_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = PAY_WORDS.write() { *w = v; } }
pub fn g_PROPERTY_ASK() -> Py { ensure_init(); PROPERTY_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PROPERTY_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PROPERTY_ASK.write() { *w = v; } }
pub fn g_ROTA_NOUNS() -> Py { ensure_init(); ROTA_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROTA_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = ROTA_NOUNS.write() { *w = v; } }
pub fn g_ROTA_WORDS() -> Py { ensure_init(); ROTA_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ROTA_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ROTA_WORDS.write() { *w = v; } }
pub fn g_SAID_ASK() -> Py { ensure_init(); SAID_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_ASK(v: Py) { ensure_init(); if let Ok(mut w) = SAID_ASK.write() { *w = v; } }
pub fn g_SAID_BEFORE() -> Py { ensure_init(); SAID_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = SAID_BEFORE.write() { *w = v; } }
pub fn g_SAID_IN_A_PAST_CHAT() -> Py { ensure_init(); SAID_IN_A_PAST_CHAT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_IN_A_PAST_CHAT(v: Py) { ensure_init(); if let Ok(mut w) = SAID_IN_A_PAST_CHAT.write() { *w = v; } }
pub fn g_SAID_POOL() -> Py { ensure_init(); SAID_POOL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_POOL(v: Py) { ensure_init(); if let Ok(mut w) = SAID_POOL.write() { *w = v; } }
pub fn g_STOP_WORDS() -> Py { ensure_init(); STOP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STOP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = STOP_WORDS.write() { *w = v; } }
pub fn g_TAUGHT_BY_THIS() -> Py { ensure_init(); TAUGHT_BY_THIS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAUGHT_BY_THIS(v: Py) { ensure_init(); if let Ok(mut w) = TAUGHT_BY_THIS.write() { *w = v; } }
pub fn g_THEIR_OWN_WORD() -> Py { ensure_init(); THEIR_OWN_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_OWN_WORD(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_OWN_WORD.write() { *w = v; } }
pub fn g_THE_OTHER_CHATS() -> Py { ensure_init(); THE_OTHER_CHATS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_OTHER_CHATS(v: Py) { ensure_init(); if let Ok(mut w) = THE_OTHER_CHATS.write() { *w = v; } }
pub fn g_WHAT_MY() -> Py { ensure_init(); WHAT_MY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_MY(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_MY.write() { *w = v; } }
pub fn g_WHEN_AUX_MY() -> Py { ensure_init(); WHEN_AUX_MY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_AUX_MY(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_AUX_MY.write() { *w = v; } }
pub fn g_WHEN_I() -> Py { ensure_init(); WHEN_I.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_I(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_I.write() { *w = v; } }
pub fn g_WHEN_MY() -> Py { ensure_init(); WHEN_MY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_MY(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_MY.write() { *w = v; } }
pub fn g_WHERE_MY() -> Py { ensure_init(); WHERE_MY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHERE_MY(v: Py) { ensure_init(); if let Ok(mut w) = WHERE_MY.write() { *w = v; } }
pub fn g__THE_PLACE() -> Py { ensure_init(); _THE_PLACE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__THE_PLACE(v: Py) { ensure_init(); if let Ok(mut w) = _THE_PLACE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A question about a fact of the person's own life, where the bot has not been told it.\n\n\"how many brothers do i have\" got the search hedge (\"I don't know enough to answer that honestly.\nA search should sort you out quickly\") - which is nonsense: no search engine knows how many\nbrothers the person typing has. \"whats my middle name\" got the same. The store answers these fine\nwhen it was told (`memory_store`: \"You told me you're 34\", \"Your sister is called Emma\"), so what\nis left is the case where nobody ever said it, and the honest answer there is not a search engine.\nIt is STYLE.md's \"Not knowing, honestly\" in its plainest form: only they know, say so in a few\nwords, and ask - which is also the one move that turns the gap into the fact the store can keep.\n\nThe question is read here, not composed from a list of facts: \"how many X do i have\" becomes \"how\nmany X you have\", \"whats my X\" becomes \"what your X is\", \"when is my X\" becomes \"when your X is\".\nSo the reply says the thing back in the person's own words and then says the true part.\n\nNarrow on purpose. It takes a question about themselves - \"my\", \"i have\", \"am i\" - whose subject\nis a thing of their own life: family, a name, an age, a birthday, a home, work, a pet, an\nappointment. A how-to (\"how do i lose weight\"), a fact about the world (\"how many people live in\nwales\"), a question about the bot (\"whats your name\"), an ask with no subject of theirs in it, and\nanything the store or a card already answers keep the reply they had.\n");
    s_STOP_WORDS(call_builtin("frozenset", &[mcall(&py_str("about the a an and or but that this it its was were is are be been\ndid do does say said tell told mention mentioned you your me my i we us our again"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_OWN_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("mum"), py_str("mom"), py_str("mother"), py_str("dad"), py_str("father"), py_str("parent"), py_str("parents"), py_str("brother"), py_str("brothers"), py_str("sister"), py_str("sisters"), py_str("sibling"), py_str("siblings"), py_str("son"), py_str("sons"), py_str("daughter"), py_str("daughters"), py_str("kid"), py_str("kids"), py_str("child"), py_str("children"), py_str("baby"), py_str("wife"), py_str("husband"), py_str("partner"), py_str("nan"), py_str("nana"), py_str("gran"), py_str("granny"), py_str("grandma"), py_str("grandad"), py_str("grandpa"), py_str("grandparents"), py_str("aunt"), py_str("auntie"), py_str("uncle"), py_str("cousin"), py_str("cousins"), py_str("niece"), py_str("nephew"), py_str("inlaws"), py_str("family"), py_str("flatmate"), py_str("housemate"), py_str("landlord"), py_str("neighbour"), py_str("neighbours"), py_str("boss"), py_str("manager"), py_str("colleague"), py_str("carer"), py_str("name"), py_str("surname"), py_str("middlename"), py_str("nickname"), py_str("age"), py_str("birthday"), py_str("birthdate"), py_str("dob"), py_str("address"), py_str("postcode"), py_str("house"), py_str("flat"), py_str("home"), py_str("rent"), py_str("mortgage"), py_str("job"), py_str("work"), py_str("shift"), py_str("shifts"), py_str("rota"), py_str("wage"), py_str("wages"), py_str("salary"), py_str("pay"), py_str("holiday"), py_str("holidays"), py_str("leave"), py_str("appointment"), py_str("appointments"), py_str("referral"), py_str("prescription"), py_str("meds"), py_str("medication"), py_str("tablets"), py_str("results"), py_str("record"), py_str("records"), py_str("account"), py_str("bill"), py_str("bills"), py_str("order"), py_str("budget"), py_str("savings"), py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("pet"), py_str("pets"), py_str("puppy"), py_str("kitten"), py_str("car"), py_str("van"), py_str("bike")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY(stdlib_call("re", "compile", &[py_str("^how (many|much)\\s+(.+?)\\s+(?:do|did) i have(?: left)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_MANY_GOT(stdlib_call("re", "compile", &[py_str("^how (many|much)\\s+(.+?)\\s+have i got(?: left)?$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HOW_OLD(stdlib_call("re", "compile", &[py_str("^how old am i$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_MY(stdlib_call("re", "compile", &[py_str("^what(?:s| is| was)?\\s+my\\s+(.+?)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_MY(stdlib_call("re", "compile", &[py_str("^(?:when|what time)(?:s| is| was)?\\s+my\\s+(.+?)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_AUX_MY(stdlib_call("re", "compile", &[py_str("^(?:when|what time)\\s+(?:does|do|did|will|should|can)\\s+my\\s+(.+?)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHERE_MY(stdlib_call("re", "compile", &[py_str("^where(?:s| is| was)?\\s+my\\s+(.+?)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_HAVEN_T_SAID(py_tuple(vec![py_str("You haven't told me {q}, so I've no way to know - tell me and I'll remember it."), py_str("I don't know {q}: that's yours, not something I can look up. Tell me and I'll keep it."), py_str("I only know what you tell me, and you've not said {q}. What's the answer?")]));
    s_AS_THE_QUESTION(py_tuple(vec![py_str("{Q}? You've not said, so only you know - tell me and I'll remember it."), py_str("{Q}? I can't look that up: it's about you. Say it once and I'll keep it.")]));
    s_HOW_TO(stdlib_call("re", "compile", &[py_str("\\bhow (?:do|can|could|should|would|might) i\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ABOUT_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:you|your|yours|yourself)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CHAT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("chat"), py_str("chats"), py_str("conversation"), py_str("conversations"), py_str("message"), py_str("messages"), py_str("history"), py_str("transcript"), py_str("transcripts"), py_str("logs")])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_HAND_ON_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:see|read|open|look at|look back|check|bring up|pull up|access|scroll back|go back to|remember|recall|keep|store|save|hold|show me|do with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_OTHER_CHATS(stdlib_call("re", "compile", &[py_str("\\b(?:see|read|open|look at|look back|check|bring up|pull up|access|scroll back|go back to|remember|recall|find|keep|store|save|hold|show me|know about|do with)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_THAN_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:before|earlier|previous|previously|past|old|last|other|others|yesterday|prior|last time|history)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RECORD_OF_THEM(stdlib_call("re", "compile", &[py_str("\\b(?:is|are) there (?:a |any |some )?\\b.{0,24}\\b(?:list|record|records|history|log|logs|copy|transcript)\\b|\\bdo you (?:have|keep|store) (?:a |any )?(?:record|records|list|history|log|logs|copy|transcript)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_IN_A_PAST_CHAT(stdlib_call("re", "compile", &[py_str("\\b(?:what|everything|all) (?:we|i) (?:said|talked about|wrote|asked)\\b.{0,30}\\b(?:before|earlier|yesterday|last time|previous|in another chat|in other chats)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEARNING_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:learn|learns|learning|train|trains|training|trained|teach|teaches|teaching|update|updates|updating|improve|improves|improving|pick (?:things |stuff )?up|pick up on)\\b|\\b(?:do|will|would|are|did)\\s+(?:you|u)\\s+get(?:ting)?\\s+better\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TAUGHT_BY_THIS(stdlib_call("re", "compile", &[py_str("\\b(?:teach|teaches|show|shows|tell|tells|give|gives|feed|feeds)\\s+(?:you|u)\\b|\\b(?:you|u)\\s+(?:learn|learning|train|training|pick\\w*)\\b.{0,30}\\b(?:from|on|off)\\s+(?:me|what i|us|this|my|our|these|the chats)\\b|\\b(?:learn|train)\\w*\\s+(?:on|from)\\s+(?:my|our|these|this)\\b|\\blearn from me\\b|\\b(?:do|will|would|are|did)\\s+(?:you|u)\\s+get(?:ting)?\\s+better\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_BEFORE(stdlib_call("re", "compile", &[py_str("\\b(?:again|remind me|did i say|did i tell you)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_OTHER_CHATS(py_tuple(vec![py_str("No: each chat starts empty for me, so nothing from another conversation comes through. Everything you tell me in this one stays with me, though - say it here and I'll use it. What was it about?"), py_str("I can't open earlier chats - a new one starts with nothing in it. What you and I say in this chat I do keep, so tell me here and I'll pick it up from there. What were you trying to find?"), py_str("There's no history on my side: what we say stays in this conversation, and a new chat begins blank. So say it once more here and I'll have it. What was it?")]));
    s_NO_TRAINING(py_tuple(vec![py_str("No training as we talk: I'm a program with a lot of everyday things in it, and what I pick up is only what you say here, while we are in this chat. Nothing you tell me teaches me for the next person."), py_str("I don't learn from the chat, no - nothing is fed back in. I'm the same program at the end of it as at the start, with everything you said in front of me while we talk."), py_str("Nothing I hear changes what I know: no training, no memory of it afterwards. What you say does get used - here, in this conversation, as we go.")]));
    s_THEIR_OWN_WORD(call_builtin("frozenset", &[py_tuple(vec![py_str("my"), py_str("our"), py_str("the"), py_str("these"), py_str("those"), py_str("your"), py_str("all"), py_str("any"), py_str("every"), py_str("this"), py_str("that"), py_str("previous"), py_str("past"), py_str("old"), py_str("last"), py_str("earlier"), py_str("other"), py_str("another"), py_str("same")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_I(stdlib_call("re", "compile", &[py_str("^(?:when|what time|what day|which day|what days|which days|how many hours)\\s+(?:do|did|will|should|am|can|have)\\s+i\\b(.*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAY_DAY(stdlib_call("re", "compile", &[py_str("^when(?:s| is| was)?\\s+(?:my\\s+)?pay ?day$")]).unwrap_or_else(|e| exc::raise(e)));
    s_ROTA_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("work"), py_str("works"), py_str("working"), py_str("shift"), py_str("shifts"), py_str("rota"), py_str("hours"), py_str("overtime"), py_str("oncall")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ROTA_NOUNS(call_builtin("frozenset", &[py_tuple(vec![py_str("shift"), py_str("shifts"), py_str("rota"), py_str("hours"), py_str("overtime"), py_str("oncall")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAY_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("paid"), py_str("payday"), py_str("wage"), py_str("wages"), py_str("salary")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THE_ROTA(py_tuple(vec![py_str("Your rota isn't something I can see, so read the hours off it and tell me and I'll keep them."), py_str("That lives on your rota rather than anywhere I can see, so tell me what it says."), py_str("No window on your rota from this end, sorry, so say the hours and I'll remember them.")]));
    s_ON_THE_PAYSLIP(py_tuple(vec![py_str("A pay date is set in your contract, not known here, so tell me the date and I'll keep it."), py_str("Your pay date is on the contract you signed, so say it once and I'll remember it.")]));
    s__THE_PLACE(py_dict(vec![(py_str("rota"), g_ON_THE_ROTA()), (py_str("payslip"), g_ON_THE_PAYSLIP())]));
    s_NOT_A_NOUN_PHRASE(stdlib_call("re", "compile", &[py_str("\\b(?:to|for|with|about|at|in|on|from|by|due|like|doing|going|coming|getting|allowed|before|after|now|then|anymore|originally|anyway)\\b|\\w+ing\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_ASK(stdlib_call("re", "compile", &[py_str("\\b(?:(where|what|when|why|who|how\\s+often|how\\s+many\\s+times)\\s+)?did\\s+(?:i|we)\\s+(?:ever\\s+)?(?:say|tell\\s+you|mention)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FREQUENCY_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:often|many\\s+times)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PROPERTY_ASK(stdlib_call("re", "compile", &[py_str("\\bhow\\s+(?:old|big|heavy|long|far|tall|late|early)\\b|\\bwhat\\s+colou?r\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE(stdlib_call("re", "compile", &[py_str("\\b\\d+\\s*(?:times?|x)\\b|\\b(?:once|twice|daily|weekly|monthly)\\b|\\bevery\\s+(?:day|week|month|year|morning|night)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_GENERIC_WORDS(call_builtin("frozenset", &[mcall(&py_str("go goes going went do does doing get gets got have has had make makes\nmade take takes took put see saw look start stop carry"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_POOL(py_tuple(vec![py_str("You told me: “{said}”"), py_str("That was you, not me: “{said}”"), py_str("From your own words: “{said}”")]));
    s_NOT_A_RECORD(py_tuple(vec![py_str("You've not said that to me yet, so there's nothing to hand back - tell me and I'll keep it."), py_str("I've nothing on that from you: it's yours to tell me, and then it's kept.")]));
    s_NOT_A_RECORD_BUT(py_tuple(vec![py_str("You've not said that to me, but the nearest you've given me is: “{said}”"), py_str("Nothing of yours on that yet - the closest is “{said}”, and I'll keep the rest when you say it.")]));
    s_HOW_OFTEN_MINE(stdlib_call("re", "compile", &[py_str("^how\\s+(?:often|many\\s+times)\\s+(?:do|did|does)\\s+(?:i|we)\\s+(\\w+)(?:\\s+(?:it|that|this|them|now))?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _names_their_chats(args: &[Py]) -> Py {
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
    let mut index: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line speaks of the person's own chats, not of the word by itself.");
    tokens = stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_CHAT_WORDS(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = index.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THEIR_OWN_WORD(), &geti(&tokens.clone(), &sub(&index.clone(), &Py::Int(1))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_THEIR_OWN_WORD(), &geti(&tokens.clone(), &sub(&index.clone(), &Py::Int(2))))) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = gt(&index.clone(), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_CHAT_WORDS(), &geti(&tokens.clone(), &sub(&index.clone(), &Py::Int(2))))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn about_earlier_chats(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("A reply for a question about the chats before this one, or None.\n\n    The bot is the only thing that can answer it, so a search hedge is the one reply that is\n    certainly wrong. Narrow on purpose: the line has to name those chats (\"chats\", \"conversation\",\n    \"history\") with a word that puts them in the past (\"before\", \"last\", \"old\", \"yesterday\"), or\n    ask for a record of them. A line about this conversation - \"what did i tell you my name was\" -\n    is the store's, and one that names something to be recalled keeps its own answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    clean = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&Py::Bool(!py_truthy(&clean.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_names_their_chats(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_SAID_IN_A_PAST_CHAT(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&_names_their_chats(&[clean.clone()])) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_THE_OTHER_CHATS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_RECORD_OF_THEM(), "search", &[clean.clone()]) } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_OTHER_THAN_THIS(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_A_RECORD_OF_THEM(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_HAND_ON_THEM(), "search", &[clean.clone()]) } } } }))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:you|your|u|show me)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_A_RECORD_OF_THEM(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[g_NO_OTHER_CHATS(), memory.clone(), text.clone()]);
    Py::None
}
pub fn learns_from_the_chat(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let _ = py_str("A reply for \"do you learn from this chat\", \"will you get better\", or None.\n\n    The same rule as `about_earlier_chats`: the bot is the only thing that knows the answer, so a\n    search hedge is certainly wrong. Narrow - the line has to be about the bot learning, from this\n    conversation or from the person, and a question about the person learning (\"how do i learn to\n    drive\", \"can i learn python at 40\") is not this line.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    clean = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_LEARNING_ASK(), "search", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:you|u)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_TAUGHT_BY_THIS(), "search", &[clean.clone()])))) {
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _names_their_chats(&[clean.clone()]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:these|this|our|the)\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\b(?:help|teach|show) me\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\bhow (?:do|can|could) i\\b"), clean.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    return crate::gen::compose::pick(&[g_NO_TRAINING(), memory.clone(), text.clone()]);
    Py::None
}
pub fn _the_place(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("Which place settles a question about their own timetable, or None.\n\n    \"when do i work again\" and \"which days do i work\" are the rota; \"when do i get paid\" and\n    \"when is payday\" are the contract. A line that says start, finish or clock in with no work\n    word in it is about something else - a course, a race, a roast - and keeps what it had.\n    ");
    if py_truthy(&mcall(&g_PAY_DAY(), "match", &[text.clone()])) {
    return py_str("payslip");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_WHEN_I(), g_WHEN_MY(), g_WHEN_AUX_MY()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    tail = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&found.clone(), "group", &[Py::Int(1)])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bitand(&tail.clone(), &g_PAY_WORDS()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&tail.clone(), &py_str("pay"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&tail.clone(), &py_str("in"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&tail.clone(), &py_str("into"))) } } } } } }) {
    return py_str("payslip");
    }
    if py_truthy(&bitand(&tail.clone(), &g_ROTA_WORDS())) {
    said = mcall(&found.clone(), "group", &[Py::Int(1)]);
    if py_truthy(&Py::Bool(!py_truthy(&bitand(&tail.clone(), &g_ROTA_NOUNS())))) {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bon[-\\s]?call\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("rota");
    }
    said = stdlib_call("re", "sub", &[py_str("\\bwork(?:s|ing)?\\s+(?:out|on)\\b"), py_str(""), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    tail = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&bitand(&tail.clone(), &g_ROTA_WORDS())) {
    return py_str("rota");
    }
    return Py::None;
    }
    return py_str("rota");
    }
    return Py::None;
    }
    return Py::None;
    Py::None
}
pub fn _has_a_shape(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("True when the line is one of the question shapes below, whatever its tail is.");
    if py_truthy(&{ let __b = mcall(&g_HOW_MANY(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_HOW_MANY_GOT(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_OLD(), "match", &[text.clone()]) } } } }) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![g_WHAT_MY(), g_WHEN_MY(), g_WHERE_MY()])) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "match", &[text.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _as_clause(args: &[Py]) -> Py {
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
    let mut many: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The question as the thing they have not said: \"how many brothers you have\".");
    found = { let __b = mcall(&g_HOW_MANY(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_MANY_GOT(), "match", &[text.clone()]) } };
    if py_truthy(&found.clone()) {
    let __unpk1 = py_unpack(&mcall(&found.clone(), "groups", &[]), 2, None);
    many = __unpk1[0].clone();
    thing = __unpk1[1].clone();
    return { let mut __s = String::new(); __s.push_str("how "); __s.push_str(&fstr(&many.clone(), -1, "")); __s.push_str(" "); __s.push_str(&fstr(&thing.clone(), -1, "")); __s.push_str(" you have"); py_string(__s) };
    }
    if py_truthy(&mcall(&g_HOW_OLD(), "match", &[text.clone()])) {
    return py_str("how old you are");
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![g_WHAT_MY(), g_WHEN_MY(), g_WHERE_MY()]));
    for __item in __iter2 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[text.clone()]);
    if py_truthy(&found.clone()) {
    tail = mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]);
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\bcalled$"), tail.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return { let mut __s = String::new(); __s.push_str("what your "); __s.push_str(&fstr(&tail.clone(), -1, "")); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &g_WHEN_MY()))) {
    return { let mut __s = String::new(); __s.push_str("when your "); __s.push_str(&fstr(&tail.clone(), -1, "")); __s.push_str(" is"); py_string(__s) };
    }
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &g_WHERE_MY()))) {
    return { let mut __s = String::new(); __s.push_str("where your "); __s.push_str(&fstr(&tail.clone(), -1, "")); __s.push_str(" is"); py_string(__s) };
    }
    return { let mut __s = String::new(); __s.push_str("what your "); __s.push_str(&fstr(&tail.clone(), -1, "")); __s.push_str(" is"); py_string(__s) };
    }
    }
    return Py::None;
    Py::None
}
pub fn _as_question(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut ask: Py = Py::None;
    let mut head: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The same clause turned back into the question a person would ask: \"how many brothers you\n    have\" -> \"how many brothers do you have\".");
    if py_truthy(&mcall(&clause.clone(), "startswith", &[py_tuple(vec![py_str("how many"), py_str("how much")])])) {
    return mcall(&clause.clone(), "replace", &[py_str(" you have"), py_str(" do you have")]);
    }
    if py_truthy(&mcall(&clause.clone(), "startswith", &[py_str("how old you are")])) {
    return py_str("how old are you");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("what your "), py_str("what is ")]), py_tuple(vec![py_str("when your "), py_str("when is ")]), py_tuple(vec![py_str("where your "), py_str("where is ")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    head = __unpk2[0].clone();
    ask = __unpk2[1].clone();
    if py_truthy(&mcall(&clause.clone(), "startswith", &[head.clone()])) {
    tail = slice_or_raise(&clause.clone(), &call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    if py_truthy(&mcall(&tail.clone(), "endswith", &[py_str(" is")])) {
    tail = slice_or_raise(&tail.clone(), &Py::None, &neg(&Py::Int(3)), &Py::None);
    }
    return add(&add(&ask.clone(), &py_str("your ")), &tail.clone());
    }
    }
    return clause.clone();
    Py::None
}
pub fn _the_thing(args: &[Py]) -> Py {
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
    let _ = py_str("The word the question turns on, when it is a thing of their own life.");
    words = stdlib_call("re", "findall", &[py_str("[a-z]+"), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_OWN_WORDS(), &word.clone()))) {
    return word.clone();
    }
    }
    let __iter2 = py_iter_or_raise(&words.clone());
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_OWN_WORDS(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    }
    return Py::None;
    Py::None
}
pub fn _second_person(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let _ = py_str("The person's own question with themselves in the second person, every word in its place:\n    \"what was my budget\" -> \"what was your budget\".");
    out = stdlib_call("re", "sub", &[py_str("\\b(whats|wheres|whens|hows)\\b"), py_func("lambda", move |args: &[Py]| { let out = out.clone(); let text = text.clone(); let m: Py = __kwp(args, 0, "m").unwrap_or(Py::None); geti(&py_dict(vec![(py_str("whats"), py_str("what's")), (py_str("wheres"), py_str("where's")), (py_str("whens"), py_str("when's")), (py_str("hows"), py_str("how's"))]), &mcall(&m.clone(), "group", &[Py::Int(1)])) }), text.clone()]).unwrap_or_else(|e| exc::raise(e));
    out = stdlib_call("re", "sub", &[py_str("\\bmy\\b"), py_str("your"), out.clone()]).unwrap_or_else(|e| exc::raise(e));
    out = stdlib_call("re", "sub", &[py_str("\\bdo i have\\b"), py_str("do you have"), out.clone()]).unwrap_or_else(|e| exc::raise(e));
    out = stdlib_call("re", "sub", &[py_str("\\bhave i got\\b"), py_str("have you got"), out.clone()]).unwrap_or_else(|e| exc::raise(e));
    out = stdlib_call("re", "sub", &[py_str("\\bam i\\b"), py_str("are you"), out.clone()]).unwrap_or_else(|e| exc::raise(e));
    return out.clone();
    Py::None
}
pub fn _capital(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut clause: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    return add(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn answer_for(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut line: Py = Py::None;
    let mut place: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut question: Py = Py::None;
    let _ = py_str("A reply for a fact question about the person that nobody ever told the bot, or None.");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    clean = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clean.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&clean.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(10)) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_ABOUT_BOT(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_HOW_TO(), "search", &[clean.clone()]) } }) {
    return Py::None;
    }
    place = _the_place(&[clean.clone()]);
    if py_truthy(&place.clone()) {
    return crate::gen::compose::pick(&[geti(&g__THE_PLACE(), &place.clone()), memory.clone(), text.clone()]);
    }
    if py_truthy(&mcall(&g_SAID_BEFORE(), "search", &[clean.clone()])) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_the_thing(&[clean.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HOW_OLD(), "match", &[clean.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_has_a_shape(&[clean.clone()])))) {
    return Py::None;
    }
    clause = _as_clause(&[clean.clone()]);
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_NOT_A_NOUN_PHRASE(), "search", &[geti(&mcall(&clause.clone(), "split", &[py_str("your"), Py::Int(1)]), &neg(&Py::Int(1)))]) } }) {
    clause = Py::None;
    }
    if py_truthy(&Py::Bool(py_is(&clause.clone(), &Py::None))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("Q".to_string(), _capital(&[_second_person(&[clean.clone()])])), ("q".to_string(), clean.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_AS_THE_QUESTION(), memory.clone(), text.clone()]), "format", args))(&__args) };
    }
    pool = if py_truthy(&gt(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) { g_HAVEN_T_SAID() } else { g_AS_THE_QUESTION() };
    line = crate::gen::compose::pick(&[pool.clone(), memory.clone(), text.clone()]);
    question = _capital(&[_second_person(&[clean.clone()])]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("q".to_string(), clause.clone()), ("Q".to_string(), question.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) };
    Py::None
}
pub fn _their_own_lines(args: &[Py]) -> Py {
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
    let mut lines: Py = Py::None;
    let mut said: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The lines the person said, newest first, from the store's own record of the turns.");
    lines = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&said.clone(), "lower", &[]))) } }) {
    let _ = mcall(&lines.clone(), "append", &[said.clone()]);
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topics")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    topic = __item.clone();
    said = mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = topic.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&said.clone(), "lower", &[]))) } }) {
    let _ = mcall(&lines.clone(), "append", &[said.clone()]);
    }
    }
    return lines.clone();
    Py::None
}
pub fn _stem(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("Enough of a stem to see \"walking\" and \"walk\" as one subject, and no more.");
    word = mcall(&word.clone(), "lower", &[]);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    tail = __item.clone();
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[tail.clone()]) } }) {
    return slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    }
    return word.clone();
    Py::None
}
pub fn said_back(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut best: Py = Py::None;
    let mut best_score: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut score: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line of theirs the ask is about, said back, or None when the line is not this ask.\n\n    Only a recall is read here - the wrapper is \"did i say/tell/mention\" - and the reply is a\n    quotation, never a fact, so nothing can be invented. Where the ask names something, the line\n    that shares the most of it is chosen, so a redone \"what did i say about walking\" does not come\n    back with the trip; where it names something no line carries, the honest answer is that it was\n    never said, rather than the newest line put in their mouth.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return Py::None;
    }
    clean = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    found = mcall(&g_SAID_ASK(), "search", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return _the_one_rate_on_record(&[clean.clone(), memory.clone(), text.clone()]);
    }
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_their_own_lines(&[memory.clone()])) { let mut line: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&_stemmed(&[line.clone()]), &_stemmed(&[clean.clone()])))) {  __out.push(line.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&lines.clone()))) {
    return crate::gen::compose::pick(&[g_NOT_A_RECORD(), memory.clone(), text.clone()]);
    }
    if py_truthy(&mcall(&g_FREQUENCY_ASK(), "search", &[clean.clone()])) {
    let __iter1 = py_iter_or_raise(&slice_or_raise(&lines.clone(), &Py::None, &Py::Int(8), &Py::None));
    for __item in __iter1 {
    line = __item.clone();
    if py_truthy(&mcall(&g_A_RATE(), "search", &[line.clone()])) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), line.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_SAID_POOL(), memory.clone(), text.clone()]), "format", args))(&__args) };
    }
    }
    }
    asked = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), slice_or_raise(&clean.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STOP_WORDS(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_GENERIC_WORDS(), &_stem(&[word.clone()]))) } }) {  __out.push(_stem(&[word.clone()])); } } py_list(__out) })() };
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&lines.clone(), &Py::Int(0)), Py::Int(0)]), 2, None);
    best = __unpk2[0].clone();
    best_score = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&slice_or_raise(&lines.clone(), &Py::None, &Py::Int(8), &Py::None));
    for __item in __iter3 {
    line = __item.clone();
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), line.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    score = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&asked.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&words.clone(), &word.clone()))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&gt(&score.clone(), &best_score.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![line.clone(), score.clone()]), 2, None);
    best = __unpk4[0].clone();
    best_score = __unpk4[1].clone();
    }
    }
    if py_truthy(&{ let __b = asked.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&best_score.clone())) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), geti(&lines.clone(), &Py::Int(0)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_NOT_A_RECORD_BUT(), memory.clone(), text.clone()]), "format", args))(&__args) };
    }
    if py_truthy(&{ let __b = mcall(&g_PROPERTY_ASK(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\d"), best.clone()]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), best.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_NOT_A_RECORD_BUT(), memory.clone(), text.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), best.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_SAID_POOL(), memory.clone(), text.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn _stemmed(args: &[Py]) -> Py {
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
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_list(__out) })() }]);
    Py::None
}
pub fn _the_one_rate_on_record(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut found: Py = Py::None;
    let mut rate: Py = Py::None;
    let mut rates: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut what: Py = Py::None;
    found = mcall(&g_HOW_OFTEN_MINE(), "match", &[clean.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    rates = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("rates")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut rate: Py = __item.clone(); if py_truthy(&mcall(&rate.clone(), "get", &[py_str("said")])) {  __out.push(rate.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[rates.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    rate = geti(&rates.clone(), &Py::Int(0));
    verb = _stem(&[mcall(&found.clone(), "group", &[Py::Int(1)])]);
    what = if py_truthy(&mcall(&rate.clone(), "get", &[py_str("what")])) { _stem(&[geti(&mcall(&stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(" "), mcall(&call_builtin("str", &[{ let __b = mcall(&rate.clone(), "get", &[py_str("what")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)), "split", &[]), &Py::Int(0))]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_GENERIC_WORDS(), &verb.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&verb.clone(), &what.clone())) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("said".to_string(), geti(&rate.clone(), &py_str("said")))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&crate::gen::compose::pick(&[g_SAID_POOL(), memory.clone(), text.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.own_facts".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_BOT" => Some(g_ABOUT_BOT()),
            "AS_THE_QUESTION" => Some(g_AS_THE_QUESTION()),
            "A_HAND_ON_THEM" => Some(g_A_HAND_ON_THEM()),
            "A_RATE" => Some(g_A_RATE()),
            "A_RECORD_OF_THEM" => Some(g_A_RECORD_OF_THEM()),
            "CHAT_WORDS" => Some(g_CHAT_WORDS()),
            "FREQUENCY_ASK" => Some(g_FREQUENCY_ASK()),
            "GENERIC_WORDS" => Some(g_GENERIC_WORDS()),
            "HAVEN_T_SAID" => Some(g_HAVEN_T_SAID()),
            "HOW_MANY" => Some(g_HOW_MANY()),
            "HOW_MANY_GOT" => Some(g_HOW_MANY_GOT()),
            "HOW_OFTEN_MINE" => Some(g_HOW_OFTEN_MINE()),
            "HOW_OLD" => Some(g_HOW_OLD()),
            "HOW_TO" => Some(g_HOW_TO()),
            "LEARNING_ASK" => Some(g_LEARNING_ASK()),
            "NOT_A_NOUN_PHRASE" => Some(g_NOT_A_NOUN_PHRASE()),
            "NOT_A_RECORD" => Some(g_NOT_A_RECORD()),
            "NOT_A_RECORD_BUT" => Some(g_NOT_A_RECORD_BUT()),
            "NO_OTHER_CHATS" => Some(g_NO_OTHER_CHATS()),
            "NO_TRAINING" => Some(g_NO_TRAINING()),
            "ON_THE_PAYSLIP" => Some(g_ON_THE_PAYSLIP()),
            "ON_THE_ROTA" => Some(g_ON_THE_ROTA()),
            "OTHER_THAN_THIS" => Some(g_OTHER_THAN_THIS()),
            "OWN_WORDS" => Some(g_OWN_WORDS()),
            "PAY_DAY" => Some(g_PAY_DAY()),
            "PAY_WORDS" => Some(g_PAY_WORDS()),
            "PROPERTY_ASK" => Some(g_PROPERTY_ASK()),
            "ROTA_NOUNS" => Some(g_ROTA_NOUNS()),
            "ROTA_WORDS" => Some(g_ROTA_WORDS()),
            "SAID_ASK" => Some(g_SAID_ASK()),
            "SAID_BEFORE" => Some(g_SAID_BEFORE()),
            "SAID_IN_A_PAST_CHAT" => Some(g_SAID_IN_A_PAST_CHAT()),
            "SAID_POOL" => Some(g_SAID_POOL()),
            "STOP_WORDS" => Some(g_STOP_WORDS()),
            "TAUGHT_BY_THIS" => Some(g_TAUGHT_BY_THIS()),
            "THEIR_OWN_WORD" => Some(g_THEIR_OWN_WORD()),
            "THE_OTHER_CHATS" => Some(g_THE_OTHER_CHATS()),
            "WHAT_MY" => Some(g_WHAT_MY()),
            "WHEN_AUX_MY" => Some(g_WHEN_AUX_MY()),
            "WHEN_I" => Some(g_WHEN_I()),
            "WHEN_MY" => Some(g_WHEN_MY()),
            "WHERE_MY" => Some(g_WHERE_MY()),
            "_THE_PLACE" => Some(g__THE_PLACE()),
            "_as_clause" => Some(py_func("_as_clause", |args: &[Py]| _as_clause(args))),
            "_as_question" => Some(py_func("_as_question", |args: &[Py]| _as_question(args))),
            "_capital" => Some(py_func("_capital", |args: &[Py]| _capital(args))),
            "_has_a_shape" => Some(py_func("_has_a_shape", |args: &[Py]| _has_a_shape(args))),
            "_names_their_chats" => Some(py_func("_names_their_chats", |args: &[Py]| _names_their_chats(args))),
            "_second_person" => Some(py_func("_second_person", |args: &[Py]| _second_person(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_stemmed" => Some(py_func("_stemmed", |args: &[Py]| _stemmed(args))),
            "_the_one_rate_on_record" => Some(py_func("_the_one_rate_on_record", |args: &[Py]| _the_one_rate_on_record(args))),
            "_the_place" => Some(py_func("_the_place", |args: &[Py]| _the_place(args))),
            "_the_thing" => Some(py_func("_the_thing", |args: &[Py]| _the_thing(args))),
            "_their_own_lines" => Some(py_func("_their_own_lines", |args: &[Py]| _their_own_lines(args))),
            "about_earlier_chats" => Some(py_func("about_earlier_chats", |args: &[Py]| about_earlier_chats(args))),
            "answer_for" => Some(py_func("answer_for", |args: &[Py]| answer_for(args))),
            "learns_from_the_chat" => Some(py_func("learns_from_the_chat", |args: &[Py]| learns_from_the_chat(args))),
            "said_back" => Some(py_func("said_back", |args: &[Py]| said_back(args))),
            _ => None,
        }),
        set: None,
    }))
}

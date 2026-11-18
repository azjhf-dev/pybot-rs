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


pub static ACTION_AMOUNT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTION_DURATION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGES_IN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGE_FIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGE_IN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANOTHER_ANIMAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_ABOUT_THEM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_OPENERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_MONEY_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAME_AFTER_A_PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NAME_WITH_ITS_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PERCENT_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RATE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RATE_STATED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_RECALL_IN_ANOTHER_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CORRECTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CORRECTS_A_MAKE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CORRECTS_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CORRECTS_A_NAME_POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FACT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE_AFTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIGURE_MONEY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LISTED_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_FACT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_FIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_GIVEN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAME_NOT_A_NAME: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_FACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NAME_AFTER_CALLED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NAME_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_NEW_VALUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_FROM_THE_REPAIR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OTHER_NOUNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAIR_WITHOUT_A_MARKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PERSON_ADVICE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_AGE_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PET_NAME_AGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREGNANCY_SAID_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_NAMES_NOTHING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_SLOTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_SLOTS_BY_POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECALL_WRAPPER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REMIND_QUESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHE_OR_HE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SLOT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SORRY_OPENS_A_CORRECTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_PLACE_IS_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOO_AS_WELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEEKS_FIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORK_ON_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _concept: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _country: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _enough_reply: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _word: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ACTION_AMOUNT() -> Py { ensure_init(); ACTION_AMOUNT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTION_AMOUNT(v: Py) { ensure_init(); if let Ok(mut w) = ACTION_AMOUNT.write() { *w = v; } }
pub fn g_ACTION_DURATION() -> Py { ensure_init(); ACTION_DURATION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTION_DURATION(v: Py) { ensure_init(); if let Ok(mut w) = ACTION_DURATION.write() { *w = v; } }
pub fn g_AGES_IN_WORDS() -> Py { ensure_init(); AGES_IN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGES_IN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = AGES_IN_WORDS.write() { *w = v; } }
pub fn g_AGE_FIX() -> Py { ensure_init(); AGE_FIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGE_FIX(v: Py) { ensure_init(); if let Ok(mut w) = AGE_FIX.write() { *w = v; } }
pub fn g_AGE_IN_WORDS() -> Py { ensure_init(); AGE_IN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGE_IN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = AGE_IN_WORDS.write() { *w = v; } }
pub fn g_ANOTHER_ANIMAL() -> Py { ensure_init(); ANOTHER_ANIMAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANOTHER_ANIMAL(v: Py) { ensure_init(); if let Ok(mut w) = ANOTHER_ANIMAL.write() { *w = v; } }
pub fn g_ASK_ABOUT_THEM() -> Py { ensure_init(); ASK_ABOUT_THEM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_ABOUT_THEM(v: Py) { ensure_init(); if let Ok(mut w) = ASK_ABOUT_THEM.write() { *w = v; } }
pub fn g_ASK_OPENERS() -> Py { ensure_init(); ASK_OPENERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_OPENERS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_OPENERS.write() { *w = v; } }
pub fn g_A_MONEY_FIGURE() -> Py { ensure_init(); A_MONEY_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_MONEY_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = A_MONEY_FIGURE.write() { *w = v; } }
pub fn g_A_NAME_AFTER_A_PRONOUN() -> Py { ensure_init(); A_NAME_AFTER_A_PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAME_AFTER_A_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = A_NAME_AFTER_A_PRONOUN.write() { *w = v; } }
pub fn g_A_NAME_WITH_ITS_VERB() -> Py { ensure_init(); A_NAME_WITH_ITS_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NAME_WITH_ITS_VERB(v: Py) { ensure_init(); if let Ok(mut w) = A_NAME_WITH_ITS_VERB.write() { *w = v; } }
pub fn g_A_PERCENT_FIGURE() -> Py { ensure_init(); A_PERCENT_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PERCENT_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = A_PERCENT_FIGURE.write() { *w = v; } }
pub fn g_A_RATE_ASK() -> Py { ensure_init(); A_RATE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RATE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = A_RATE_ASK.write() { *w = v; } }
pub fn g_A_RATE_STATED() -> Py { ensure_init(); A_RATE_STATED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RATE_STATED(v: Py) { ensure_init(); if let Ok(mut w) = A_RATE_STATED.write() { *w = v; } }
pub fn g_A_RECALL_IN_ANOTHER_SHAPE() -> Py { ensure_init(); A_RECALL_IN_ANOTHER_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_RECALL_IN_ANOTHER_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = A_RECALL_IN_ANOTHER_SHAPE.write() { *w = v; } }
pub fn g_CORRECTING() -> Py { ensure_init(); CORRECTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CORRECTING(v: Py) { ensure_init(); if let Ok(mut w) = CORRECTING.write() { *w = v; } }
pub fn g_CORRECTS_A_MAKE() -> Py { ensure_init(); CORRECTS_A_MAKE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CORRECTS_A_MAKE(v: Py) { ensure_init(); if let Ok(mut w) = CORRECTS_A_MAKE.write() { *w = v; } }
pub fn g_CORRECTS_A_NAME() -> Py { ensure_init(); CORRECTS_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CORRECTS_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = CORRECTS_A_NAME.write() { *w = v; } }
pub fn g_CORRECTS_A_NAME_POSSESSIVE() -> Py { ensure_init(); CORRECTS_A_NAME_POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CORRECTS_A_NAME_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = CORRECTS_A_NAME_POSSESSIVE.write() { *w = v; } }
pub fn g_FACT_WORDS() -> Py { ensure_init(); FACT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FACT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = FACT_WORDS.write() { *w = v; } }
pub fn g_FIGURE_AFTER() -> Py { ensure_init(); FIGURE_AFTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE_AFTER(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE_AFTER.write() { *w = v; } }
pub fn g_FIGURE_ASK() -> Py { ensure_init(); FIGURE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE_ASK.write() { *w = v; } }
pub fn g_FIGURE_FILLER() -> Py { ensure_init(); FIGURE_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE_FILLER.write() { *w = v; } }
pub fn g_FIGURE_MONEY() -> Py { ensure_init(); FIGURE_MONEY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIGURE_MONEY(v: Py) { ensure_init(); if let Ok(mut w) = FIGURE_MONEY.write() { *w = v; } }
pub fn g_LISTED_FIGURE() -> Py { ensure_init(); LISTED_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LISTED_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = LISTED_FIGURE.write() { *w = v; } }
pub fn g_NAMED_FACT_ASK() -> Py { ensure_init(); NAMED_FACT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_FACT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_FACT_ASK.write() { *w = v; } }
pub fn g_NAME_FIX() -> Py { ensure_init(); NAME_FIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_FIX(v: Py) { ensure_init(); if let Ok(mut w) = NAME_FIX.write() { *w = v; } }
pub fn g_NAME_GIVEN() -> Py { ensure_init(); NAME_GIVEN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_GIVEN(v: Py) { ensure_init(); if let Ok(mut w) = NAME_GIVEN.write() { *w = v; } }
pub fn g_NAME_NOT_A_NAME() -> Py { ensure_init(); NAME_NOT_A_NAME.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAME_NOT_A_NAME(v: Py) { ensure_init(); if let Ok(mut w) = NAME_NOT_A_NAME.write() { *w = v; } }
pub fn g_NOT_A_FACT() -> Py { ensure_init(); NOT_A_FACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_FACT.write() { *w = v; } }
pub fn g_NOT_A_NAME_AFTER_CALLED() -> Py { ensure_init(); NOT_A_NAME_AFTER_CALLED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_AFTER_CALLED(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NAME_AFTER_CALLED.write() { *w = v; } }
pub fn g_NOT_A_NAME_BEFORE() -> Py { ensure_init(); NOT_A_NAME_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NAME_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NAME_BEFORE.write() { *w = v; } }
pub fn g_NOT_A_NEW_VALUE() -> Py { ensure_init(); NOT_A_NEW_VALUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_NEW_VALUE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_NEW_VALUE.write() { *w = v; } }
pub fn g_NOT_FROM_THE_REPAIR() -> Py { ensure_init(); NOT_FROM_THE_REPAIR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_FROM_THE_REPAIR(v: Py) { ensure_init(); if let Ok(mut w) = NOT_FROM_THE_REPAIR.write() { *w = v; } }
pub fn g_NUMBER() -> Py { ensure_init(); NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER.write() { *w = v; } }
pub fn g_OTHER_NOUNS() -> Py { ensure_init(); OTHER_NOUNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OTHER_NOUNS(v: Py) { ensure_init(); if let Ok(mut w) = OTHER_NOUNS.write() { *w = v; } }
pub fn g_PAIR_WITHOUT_A_MARKER() -> Py { ensure_init(); PAIR_WITHOUT_A_MARKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAIR_WITHOUT_A_MARKER(v: Py) { ensure_init(); if let Ok(mut w) = PAIR_WITHOUT_A_MARKER.write() { *w = v; } }
pub fn g_PERSON_ADVICE_ASK() -> Py { ensure_init(); PERSON_ADVICE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PERSON_ADVICE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = PERSON_ADVICE_ASK.write() { *w = v; } }
pub fn g_PET_AGE_ANSWER() -> Py { ensure_init(); PET_AGE_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_AGE_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = PET_AGE_ANSWER.write() { *w = v; } }
pub fn g_PET_LEAD_IN() -> Py { ensure_init(); PET_LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = PET_LEAD_IN.write() { *w = v; } }
pub fn g_PET_NAME_AGE() -> Py { ensure_init(); PET_NAME_AGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PET_NAME_AGE(v: Py) { ensure_init(); if let Ok(mut w) = PET_NAME_AGE.write() { *w = v; } }
pub fn g_PREGNANCY_SAID_AGAIN() -> Py { ensure_init(); PREGNANCY_SAID_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_SAID_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = PREGNANCY_SAID_AGAIN.write() { *w = v; } }
pub fn g_RECALL_NAMES_NOTHING() -> Py { ensure_init(); RECALL_NAMES_NOTHING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_NAMES_NOTHING(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_NAMES_NOTHING.write() { *w = v; } }
pub fn g_RECALL_POSSESSIVE() -> Py { ensure_init(); RECALL_POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_POSSESSIVE.write() { *w = v; } }
pub fn g_RECALL_SLOTS() -> Py { ensure_init(); RECALL_SLOTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_SLOTS(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_SLOTS.write() { *w = v; } }
pub fn g_RECALL_SLOTS_BY_POSSESSIVE() -> Py { ensure_init(); RECALL_SLOTS_BY_POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_SLOTS_BY_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_SLOTS_BY_POSSESSIVE.write() { *w = v; } }
pub fn g_RECALL_WRAPPER() -> Py { ensure_init(); RECALL_WRAPPER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECALL_WRAPPER(v: Py) { ensure_init(); if let Ok(mut w) = RECALL_WRAPPER.write() { *w = v; } }
pub fn g_REMIND_QUESTION() -> Py { ensure_init(); REMIND_QUESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REMIND_QUESTION(v: Py) { ensure_init(); if let Ok(mut w) = REMIND_QUESTION.write() { *w = v; } }
pub fn g_SAID_AGAIN() -> Py { ensure_init(); SAID_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = SAID_AGAIN.write() { *w = v; } }
pub fn g_SHE_OR_HE() -> Py { ensure_init(); SHE_OR_HE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHE_OR_HE(v: Py) { ensure_init(); if let Ok(mut w) = SHE_OR_HE.write() { *w = v; } }
pub fn g_SLOT_WORDS() -> Py { ensure_init(); SLOT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SLOT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = SLOT_WORDS.write() { *w = v; } }
pub fn g_SORRY_OPENS_A_CORRECTION() -> Py { ensure_init(); SORRY_OPENS_A_CORRECTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SORRY_OPENS_A_CORRECTION(v: Py) { ensure_init(); if let Ok(mut w) = SORRY_OPENS_A_CORRECTION.write() { *w = v; } }
pub fn g_THE_PLACE_IS_NAMED() -> Py { ensure_init(); THE_PLACE_IS_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_PLACE_IS_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = THE_PLACE_IS_NAMED.write() { *w = v; } }
pub fn g_TOO_AS_WELL() -> Py { ensure_init(); TOO_AS_WELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOO_AS_WELL(v: Py) { ensure_init(); if let Ok(mut w) = TOO_AS_WELL.write() { *w = v; } }
pub fn g_WEEKS_FIX() -> Py { ensure_init(); WEEKS_FIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEEKS_FIX(v: Py) { ensure_init(); if let Ok(mut w) = WEEKS_FIX.write() { *w = v; } }
pub fn g_WORK_ON_IT() -> Py { ensure_init(); WORK_ON_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORK_ON_IT(v: Py) { ensure_init(); if let Ok(mut w) = WORK_ON_IT.write() { *w = v; } }
pub fn g__concept() -> Py { ensure_init(); _concept.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__concept(v: Py) { ensure_init(); if let Ok(mut w) = _concept.write() { *w = v; } }
pub fn g__country() -> Py { ensure_init(); _country.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__country(v: Py) { ensure_init(); if let Ok(mut w) = _country.write() { *w = v; } }
pub fn g__enough_reply() -> Py { ensure_init(); _enough_reply.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__enough_reply(v: Py) { ensure_init(); if let Ok(mut w) = _enough_reply.write() { *w = v; } }
pub fn g__word() -> Py { ensure_init(); _word.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__word(v: Py) { ensure_init(); if let Ok(mut w) = _word.write() { *w = v; } }
pub fn g__words() -> Py { ensure_init(); _words.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__words(v: Py) { ensure_init(); if let Ok(mut w) = _words.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The memory store at work: remember what the user says, and answer questions about it.\n\nPRIORITY 7000 (README routing bands: memory recall that must lead). It observes every line and\nkeeps the shared keys in bot/memory/; it only proposes a reply when the user asks about the\nconversation itself (\"what did I say my job was?\", \"what were we talking about?\"), so it passes\non every ordinary question.\n\nOne exception, in the same spirit: when the person hands a choice back (\"you pick\", \"your\ncall\"), the answer is one of the options the bot itself offered a turn ago, so the store reads\nthe bot's own last reply for them (bot/memory/choice.py).\n");
    s__enough_reply(crate::gen::memory::store_replies::g_enough_reply());
    s_REMIND_QUESTION(stdlib_call("re", "compile", &[py_str("\\bremind me\\b[^.!?]{0,24}\\b(?:what|when|where|who|whom|whose|how|which|why|if)\\b|\\bremind me\\b(?:[\\s,]*(?:again|please))*[\\s,]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_FACT(stdlib_call("re", "compile", &[py_str("\\bhaven't (?:told|said|mentioned|given|got)\\b|\\bhaven't your\\b|\\bdon't know\\b|\\bnot sure\\b|\\bnothing comes to mind\\b|\\bdidn't say\\b|\\bnever told\\b|\\bno idea\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_FROM_THE_REPAIR(call_builtin("frozenset", &[mcall(&py_str("plan plans drop retract correction"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME_BEFORE(stdlib_call("re", "compile", &[py_str("(?:\\b(?:a|an|the|some|another|his|her|their|our|your|my)\\s+)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_PLACE_IS_NAMED(stdlib_call("re", "compile", &[py_str("(?:^|[.!?;:]\\s+|\\n|\\b(?:to|in|into|from|at|near|of|around|through|toward|towards|via|going|coming|off|back|flying|travelling|traveling|moving|heading|headed|staying|visiting|visit|bound|left)\\s*(?:\\d+|two|three|four|five|six|seven|eight|nine|ten)?\\s*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_OPENERS(call_builtin("frozenset", &[mcall(&py_str("what whats which when where who whose why how hows\ndo does did is are was were am can could should would will shall have has had\nany anyone anything"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_AGES_IN_WORDS({ (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&crate::gen::memory::facts::g_FIGURE_WORDS(), "items", &[])) { let __unpk179 = py_unpack(&__item.clone(), 2, None); let mut word: Py = __unpk179[0].clone(); let mut number: Py = __unpk179[1].clone(); __out.push((call_builtin("int", &[number.clone()]).unwrap_or_else(|e| exc::raise(e)), word.clone())); } py_dict(__out) })() });
    s_NAMED_FACT_ASK(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("\\bwhat(?:'s|s| is| are| was| were)?\\s+my\\s+"), &crate::gen::memory::store_lines::g__FACT_SLOT()), &py_str("\\b|\\bwhat\\s+(?:is|are|was|were)\\s+my\\s+")), &crate::gen::memory::store_lines::g__FACT_SLOT()), &py_str("\\b|\\bwhere do i (?:work|live|come from)\\b(?=\\s*(?:[?.!,;:]|$)|\\s+(?:now|again|then|at|from|these|days|properly|though|honestly|anymore|either|oh|wait|sorry|hmm|actually|um|right|please|thanks)\\b)|\\bwhat do i work as\\b|\\bwho am i\\b|\\bhow old am i\\b|\\bwhat do i do for a living\\b")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_ABOUT_THEM(stdlib_call("re", "compile", &[py_str("^\\W*(?:is|are|does|do|did|was|were|can|could|has|have|will)\\b|\\bwhat (?:does|do|did|can|could|is|are|was|were) (?:he|she|they|your|my|his|her|their)\\b|\\bwhat(?:'s|s| is| are)\\s+(?:my|his|her|their|he|she|they)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PERSON_ADVICE_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:can|could|should|shall|would|will|may|might|is it (?:ok|okay|fine|safe)|what can|what could|what should|how (?:do|can|should)|any (?:tips|ideas|advice|suggestions|ways))\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS_A_NAME(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:no|nope|sorry|actually|oops|wait|hold on|correction)[,.]?\\s+)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS_A_NAME_POSSESSIVE(stdlib_call("re", "compile", &[py_str("^\\W*(?:his|her|its|their)\\s+names?\\s+(?:actually|really)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_IN_WORDS(mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(crate::gen::memory::facts::g_FIGURE_WORDS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }]));
    s_PET_NAME_AGE(stdlib_call("re", "compile", &[add(&add(&py_str("^\\W*(?:[a-z][a-z'-]{2,20})\\s+(?:is|was|are|were)\\s+(?P<age>\\d{1,2}|"), &g_AGE_IN_WORDS()), &py_str(")(?![\\d.,]\\d)(?!\\s+of\\b)(?!\\s*(?:kg|kilo\\w*|lbs?|pounds?|stone|st|cm|metres?|meters?|miles?|hours?|mins?|minutes?|percent|%))\\s*(?P<unit>months?|weeks?)?\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_LEAD_IN(stdlib_call("re", "compile", &[py_str("^(?:and|also|oh|but|well|yeah|yes|no|sorry|actually|ok|okay|right|wait|god|hang on)\\b[\\s,]*"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PET_AGE_ANSWER(stdlib_call("re", "compile", &[add(&add(&py_str("^\\W*(?:(?:he|she|it|they)(?:'s|s|\\s+is|\\s+are|\\s+is\\s+now)?\\s+)?(?P<age>\\d{1,2}|"), &g_AGE_IN_WORDS()), &py_str(")\\s*(?P<unit>years?|months?|weeks?)?\\s*(?:old)?\\W*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_FIX(stdlib_call("re", "compile", &[add(&add(&py_str("\\b(?:he|she|they)(?:'s|s| is| are| was)\\s+(?:now\\s+)?(?P<age>\\d{1,3}|"), &g_AGE_IN_WORDS()), &py_str(")\\b(?!\\s+of\\b)")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTING(stdlib_call("re", "compile", &[py_str("\\b(?:actually|no|nope|sorry|wait|i mean|i meant|make it|makes it)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEEKS_FIX(stdlib_call("re", "compile", &[py_str("\\b(?P<weeks>\\d{1,2})\\s*(?:weeks?|wks?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PREGNANCY_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("\\b(?:pregnant|expecting)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_FIX(stdlib_call("re", "compile", &[py_str("\\b(?:(?:he|she|they)(?:'s|s| is| was)?\\s+(?:actually\\s+|really\\s+)?(?:called|named)|(?:his|her|their)\\s+names?\\s+(?:is|was|actually|really))\\s+(?P<name>[a-z][a-z'\\u2019-]{1,19})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_NOT_A_NAME(call_builtin("frozenset", &[mcall(&py_str("not called named now really also still a an the"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_AFTER_A_PRONOUN(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they)(?:'s|s| is| was)\\s+(?P<name>[a-z][a-z'\\u2019-]{1,19})\\s*[,.!]?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NAME_WITH_ITS_VERB(stdlib_call("re", "compile", &[py_str("\\b(?:called|named)\\s+(?P<name>[A-Za-z][A-Za-z'-]{1,19})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ANOTHER_ANIMAL(stdlib_call("re", "compile", &[py_str("\\b(?:other|second|another|also|third)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CORRECTS_A_MAKE(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|actually|no|i mean|make that|or)?[\\s,]*(?:its|it's|it is)\\s+(?:actually\\s+|really\\s+|just\\s+)?(?:a|an|the)\\s+(?P<value>[a-z0-9][a-z0-9' .+-]{0,31})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAME_GIVEN(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they)(?:'s|s| is| was)?\\s+(?:called|named)\\s+(?P<name>[a-z][a-z'\\-]{1,19})\\b|\\b(?:his|her|their)\\s+name(?:'s|\\s+is)\\s+(?P<name2>[a-z][a-z'\\-]{1,19})\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_NAME_AFTER_CALLED(call_builtin("frozenset", &[mcall(&py_str("\nin out off up down back away over round around about on at by for with without into onto\nsick ill late busy tired poorly bad good home here there now soon again today tonight\ntomorrow yesterday it them him her me us you one ones the a an\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER(stdlib_call("re", "compile", &[py_str("[£$€]?\\d+(?:[.,]\\d+)?(?:\\s?(?:pounds|quid|euros?|dollars?|k))?(?![a-z])")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAIR_WITHOUT_A_MARKER(py_tuple(vec![stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|oh|well|so|and|hmm)[,. ]+)*(?:(?:it'?s|its|that'?s|thats)\\s+)?(?P<to>[a-z0-9][a-z0-9'-]{0,19})\\s*,?\\s+not\\s+(?P<wrong>[a-z0-9][a-z0-9'-]{0,19})[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|oh|well|so|and|hmm)[,. ]+)*not\\s+(?P<wrong>[a-z0-9][a-z0-9'-]{0,19})\\s*[,\\u2013\\u2014;-]\\s*(?:(?:it'?s|its|that'?s)\\s+)?(?P<to>[a-z0-9][a-z0-9'-]{0,19})[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:ok|okay|oh|well|so|and|hmm|sorry|actually|no|nope)[,. ]+)*(?:i'?m|i am|im|it'?s|its|it is|that'?s|thats|we'?re|we are)\\s+not\\s+(?P<wrong>[a-z0-9][a-z0-9'.\\-]{0,19})[\\s,;.!-]*(?:but\\s+)?(?:i'?m|i am|im|it'?s|its|it is|that'?s|thats|we'?re|we are)\\s+(?P<to>[a-z0-9][a-z0-9'.\\-]{0,19})[.!]?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_NOT_A_NEW_VALUE(call_builtin("frozenset", &[mcall(&py_str("a an the me you it that this these those my your his her our their\nthanks thank sure fair right wrong now then today tomorrow yesterday fine bad good great sorry\nplease yes yeah no not ok okay yet even both all much many one two three too so there here what\nwhy how who when where true false"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_AS_WELL(stdlib_call("re", "compile", &[py_str("^(?:(?:and|also|but|plus)\\s+)?(?:my|our)\\s+(?P<who>[a-z]+(?:[ -]in[ -]law)?)\\s+(?P<agree>is\\s+as\\s+well|is\\s+too|as\\s+well|also|too)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHER_NOUNS(call_builtin("frozenset", &[mcall(&py_str("friend mate partner baby child kid children kids cousin parent parents"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_MONEY(py_tuple(vec![py_str("money")]));
    s_FIGURE_ASK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|ok|okay|right|then|oh|well)\\W+)*how\\s+(?P<asked>many|much|long|far)\\b(?P<rest>.*)$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the it its that this them those there here then now again\n    roughly exactly approximately about around all told overall altogether total more less much\n    many long do does did will would should can could you i we they he she reckon think suppose\n    guess say again please for of in at on to be have has had is are was were and but so if\n    when after before time times per each"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_FIGURE_AFTER(call_builtin("frozenset", &[mcall(&py_str("is are was were and or a an the in for of to on at by after before\n    now then so but if when that from with without each every per of till until into about\n    to around roughly or while than which where"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTION_DURATION(stdlib_call("re", "compile", &[py_str("\\bat a time\\b|\\bfor the first\\b|\\bfor the next\\b|\\bin a row\\b|\\beach time\\b|\\bthe (?:first|next|last|coming|following|remaining|final)\\b|\\b(?:once|twice|three times|a few times) a day\\b|\\ba day or two\\b|\\bper session\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTION_AMOUNT(stdlib_call("re", "compile", &[py_str("\\b(?:whenever|when|if|once|each time|any time|every time|in case|unless)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_MONEY_FIGURE(stdlib_call("re", "compile", &[py_str("(?:£|\\$|€)\\s*\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:to|-|or)\\s*(?:£|\\$|€)?\\s*\\d[\\d,]*(?:\\.\\d+)?)?|(?<![\\d.,])\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:to|-|or)\\s*\\d[\\d,]*(?:\\.\\d+)?)?\\s*(?:pounds?|quid|dollars?|euros?|pence)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PERCENT_FIGURE(stdlib_call("re", "compile", &[py_str("(?<![\\d.,])\\d[\\d,]*(?:\\.\\d+)?(?:\\s*(?:to|-|or)\\s*\\d[\\d,]*(?:\\.\\d+)?)?\\s*(?:per\\s?cent|percent|%)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LISTED_FIGURE(stdlib_call("re", "compile", &[py_str("(?:,|;)\\s*\\d+(?:\\s*(?:to|-|or)\\s*\\d+)?\\s+for\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_ASK(stdlib_call("re", "compile", &[py_str("how\\s+much\\b[^.?!]*\\b(?:a|per|each|every)\\s+(?:day|week|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_RATE_STATED(stdlib_call("re", "compile", &[py_str("(?:£|\\bgbp\\s*)?\\d[\\d,.]*\\s*(?:pounds?\\s+)?(?:a|per|each|every)\\s+(?:day|week|month|year)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("\\bi\\s+(?:mean|meant)(?:\\s+to\\s+say)?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SORRY_OPENS_A_CORRECTION(stdlib_call("re", "compile", &[py_str("\\b(?:sorry|soz)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_WRAPPER(stdlib_call("re", "compile", &[py_str("^\\s*(?:and\\s+|so\\s+|but\\s+|also\\s+|again\\s+)?(?P<word>what|when|how\\s+old)\\s+(?:did|do)\\s+(?:i|we)\\s+(?:say|call|tell\\s+(?:you|u)|told\\s+(?:you|u)|mention)\\s+(?P<rest>.+?)\\s*\\??\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SLOTS(py_tuple(vec![py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:names?|called)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("what"), py_str("name")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:birthday|bday|dob|date\\s+of\\s+birth)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("when"), py_str("birthday")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:ages?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("how old"), py_str("age")]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:budget|salary|wage|rent|target|savings?)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("what"), Py::None]), py_tuple(vec![stdlib_call("re", "compile", &[py_str("\\b(?:address|postcode|number|time|date)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), py_str("what"), Py::None])]));
    s_RECALL_POSSESSIVE(stdlib_call("re", "compile", &[py_str("^\\s*(?P<poss>my|our|his|her|their)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_SLOTS_BY_POSSESSIVE(stdlib_call("re", "compile", &[py_str("^\\s*(?:my|our|his|her|their)\\s+(?P<attribute>names?|birthday|bday|dob|ages?|budget|salary|wage|societys?|address|postcode)\\??\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHE_OR_HE(py_dict(vec![(py_str("his"), py_tuple(vec![py_str("he"), py_str("is")])), (py_str("her"), py_tuple(vec![py_str("she"), py_str("is")])), (py_str("their"), py_tuple(vec![py_str("they"), py_str("are")])), (py_str("my"), py_tuple(vec![py_str("i"), py_str("am")])), (py_str("our"), py_tuple(vec![py_str("we"), py_str("are")]))]));
    s_SLOT_WORDS(py_dict(vec![(py_str("age"), stdlib_call("re", "compile", &[py_str("\\b\\d|\\b(?:age|aged|old)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("name"), stdlib_call("re", "compile", &[py_str("\\b(?:name|called|named|calls)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))), (py_str("birthday"), stdlib_call("re", "compile", &[py_str("\\b(?:birthday|bday|dob|january|february|march|april|may|june|july|august|september|october|november|december)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)))]));
    s_WORK_ON_IT(stdlib_call("re", "compile", &[py_str("\\b[a-z]{3,}\\s+it\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("name"), py_str("called"), py_str("age"), py_str("old"), py_str("job"), py_str("work"), py_str("works"), py_str("working"), py_str("city"), py_str("town"), py_str("live"), py_str("lives"), py_str("living"), py_str("moved"), py_str("moving"), py_str("from"), py_str("diet"), py_str("dieting"), py_str("eating"), py_str("food"), py_str("budget"), py_str("money"), py_str("afford"), py_str("allergy"), py_str("allergies"), py_str("allergic"), py_str("pet"), py_str("pets"), py_str("dog"), py_str("cat"), py_str("height"), py_str("tall")])]).unwrap_or_else(|e| exc::raise(e)));
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![py_str("mother"), py_str("mum mom mummy ma mam mother mumma")]), py_tuple(vec![py_str("father"), py_str("dad daddy pa papa father")]), py_tuple(vec![py_str("parent"), py_str("parent parents folks")]), py_tuple(vec![py_str("child"), py_str("kid kids child children baby toddler")]), py_tuple(vec![py_str("dog"), py_str("dog puppy pup dogs")]), py_tuple(vec![py_str("cat"), py_str("cat kitten kitty cats")]), py_tuple(vec![py_str("pet"), py_str("pet pets")]), py_tuple(vec![py_str("car"), py_str("car vehicle motor")]), py_tuple(vec![py_str("home"), py_str("home house flat apartment place")]), py_tuple(vec![py_str("party"), py_str("party birthday bday celebration bash")]), py_tuple(vec![py_str("job"), py_str("job work career")]), py_tuple(vec![py_str("food"), py_str("food dinner tea meal supper")]), py_tuple(vec![py_str("money"), py_str("money budget cash savings")]), py_tuple(vec![py_str("diet"), py_str("diet vegetarian vegan pescatarian")]), py_tuple(vec![py_str("doctor"), py_str("doctor gp surgery hospital")]), py_tuple(vec![py_str("wedding"), py_str("wedding weddings married marry marriage engaged engagement bride groom fiance fiancee honeymoon")]), py_tuple(vec![py_str("holiday"), py_str("holiday holidays vacation trip trips break abroad travelling travel")]), py_tuple(vec![py_str("exam"), py_str("exam exams revision revise revising coursework dissertation thesis mock")]), py_tuple(vec![py_str("gym"), py_str("gym gyms fitter fitness exercise exercising workout workouts jogging running")]), py_tuple(vec![py_str("smoking"), py_str("smoking smoke smoker cigarettes cigarette fags vaping vape nicotine")])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    s__concept(__unpk2[0].clone());
    s__words(__unpk2[1].clone());
    let __iter3 = py_iter_or_raise(&mcall(&g__words(), "split", &[]));
    for __item in __iter3 {
    s__word(__item.clone());
    let _ = py_setitem(&crate::gen::memory::store_lines::g_CONCEPTS(), &g__word(), g__concept());
    }
    }
    let __iter4 = py_iter_or_raise(&crate::gen::memory::facts::g_COUNTRIES());
    for __item in __iter4 {
    s__country(__item.clone());
    let _ = mcall(&crate::gen::memory::store_lines::g_CONCEPTS(), "setdefault", &[mcall(&g__country(), "lower", &[]), py_str("holiday")]);
    }
    s_A_RECALL_IN_ANOTHER_SHAPE(stdlib_call("re", "compile", &[py_str("^(?:and|so|but|also|again|ok|okay|well)?\\s*(?:tell\\s+me\\s+what\\s+(?:i|we)\\s+(?:said|say|told\\s+(?:you|u)|tell\\s+(?:you|u)|mentioned?)\\b|(?:did|do)\\s+(?:i|we)\\s+(?:say|tell\\s+(?:you|u)|told\\s+(?:you|u)|mentioned?)\\s+(?:anything|something|much)\\b|what\\s+(?:was|were)\\s+(?:the\\s+thing|that\\s+thing|that)\\s+(?:that\\s+)?(?:i|we)\\s+(?:said|told\\s+(?:you|u)|mentioned?)\\b|remind\\s+me\\s+what\\s+(?:that|it|this|those|they)\\s+(?:was|were|is|are)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RECALL_NAMES_NOTHING(call_builtin("frozenset", &[mcall(&py_str("it that this them those these one ones there anything something\nmuch thing things about of on in at to for from with the a an was were is are be been being\ni we you u he she they me us my our his her their said say told tell mention mentioned mentioning\nand or so but also again then just really actually exactly then"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _a_kept_fact_read(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line asks back a fact they gave, in whatever words, and the store has it.\n\n    \"my budget is 300\" ... \"im going to rome in may\" ... \"whats my budget\" was answered with\n    the cost of a day in Rome. The ask carries no topic of its own, so the arbiter read the\n    conversation's topic on it and handed the turn to the module that declares that topic. A\n    recall of a fact they gave is not about the topic in the air, so the store takes it as it\n    comes - the same rule a plan asked for and the line they said already have.\n\n    The key is read by `recall.ask`, which knows the shapes and not the store, so the fact has\n    to be one the store really holds: with nothing kept the line is left to the entries, which\n    can answer it as advice.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } })) } }) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::memory::recall::ask(&[text.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    said = respond(&[text.clone(), stdlib_call("copy", "deepcopy", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_is(&mcall(&g_NOT_A_FACT(), "search", &[call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))]), &Py::None));
    Py::None
}
pub fn _their_own_line_ask(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True on the one ask the store must win even against a topic entry: their own line back.\n\n    \"what was my plan?\" is read as the topic the plan is about (\"what paint should i use for a\n    hallway?\" then \"what was my plan?\" was answered by the money entry), and the store's own plan\n    line is the answer.\n\n    \"what was i cooking?\" is read as a cooking line, and the cooking entry answers a different\n    question - a chilli recipe for a chilli they had already told us they were making, and the\n    recipe's timings for \"how long does that take\" after it. The person asked what they said,\n    not what to do. The store answers only from a line they really said, so this is said for\n    exactly the asks whose answer is that line, and every other line the store proposes on is\n    left to the arbiter's own rules.\n    ");
    let __try1 = exc::catch(|| {
    low = { let __b = crate::gen::memory::store_replies::_bare(&[text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    if py_truthy(&mcall(&crate::gen::memory::store_replies::g_PLAN_MINE(), "search", &[low.clone()])) {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = mcall(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plans")]); if py_truthy(&__b) { __b } else { mcall(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("plan")]) } }]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::gen::memory::store_replies::g_ACTIVITY_ASK(), "search", &[low.clone()])))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[crate::gen::memory::store_replies::_activity_line(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _with_repaired(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut kind: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut entry: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut more: Py = Py::None;
    let mut one: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The facts the understanding engine's own repairs make plain.\n\n    The store read the line as it was typed, so a word the engine had already repaired hid the\n    fact from it: \"im alergic to peanuts\" filed nothing at all, and \"im alergic to peanits\" filed\n    a food called \"peanits\". The repaired line is read as well, and a fact is taken from it only\n    where the typed line gave nothing for that key - so this can add what a typo hid, and cannot\n    overwrite what they actually said.\n    ");
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    plain = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("plain")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[plain.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&plain.clone(), "strip", &[]))) } }) {
    return found.clone();
    }
    if py_truthy(&Py::Bool(py_eq(&plain.clone(), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[text.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])))) {
    return found.clone();
    }
    more = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(plain.clone()); __kws.extend(vec![("question".to_string(), Py::Bool(py_eq(&kind.clone(), &py_str("question"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::extract(args))(&__args) }; if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    let __iter1 = py_iter_or_raise(&mcall(&more.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_NOT_FROM_THE_REPAIR(), &key.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[key.clone()]))); if !py_truthy(&__b) { __b } else { value.clone() } } } }) {
    let _ = py_setitem(&found.clone(), &key.clone(), value.clone());
    }
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("others")]); if !py_truthy(&__b) { __b } else { mcall(&more.clone(), "get", &[py_str("others")]) } }) {
    let __iter3 = py_iter_or_raise(&geti(&more.clone(), &py_str("others")));
    for __item in __iter3 {
    entry = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("who")]))) } }) {
    continue;
    }
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&geti(&found.clone(), &py_str("others"))) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &geti(&entry.clone(), &py_str("who")))) } }) {  __out.push(one.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_is(&known.clone(), &Py::None))) {
    let _ = mcall(&geti(&found.clone(), &py_str("others")), "append", &[call_builtin("dict", &[entry.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    continue;
    }
    let __iter4 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    key = __unpk5[0].clone();
    value = __unpk5[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&key.clone(), &py_str("who"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[key.clone()]))) } }) {
    let _ = py_setitem(&known.clone(), &key.clone(), value.clone());
    }
    }
    }
    }
    return found.clone();
    Py::None
}
pub fn _the_places_are_written_as_names(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut city: Py = Py::None;
    let mut known: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut swap: Py = Py::None;
    let _ = py_str("A place the store holds is written the way the store holds it.\n\n    A plan is kept and quoted in the words they typed, which is right for the plan and wrong for\n    the name in it: \"we r going 2 spain in may\" came back as \"You r going 2 spain in may.\", and\n    \"im going to lisbon in may\" as \"You're going to lisbon in may.\" The line that named the place\n    told the store the name (\"Spain\", \"Lisbon\"), so the reply is written with it. Only a place the\n    store is holding is touched, and a determiner in front of the word says it is a common noun,\n    not a place: \"You're planning a bath and an early night\" is not the city of Bath.\n    ");
    known = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&reply.clone(), "strip", &[]))) } } } }) {
    return reply.clone();
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut name: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[name.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&name.clone(), "strip", &[]) } }) {  __out.push(name.clone()); } } py_list(__out) })() };
    city = mcall(&known.clone(), "get", &[py_str("city")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[city.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&city.clone(), "strip", &[]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&names.clone(), &city.clone())) } } } }) {
    let _ = mcall(&names.clone(), "append", &[city.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&names.clone()))) {
    return reply.clone();
    }
    out = reply.clone();
    let __iter1 = py_iter_or_raise(&names.clone());
    for __item in __iter1 {
    name = __item.clone();
    pattern = stdlib_call("re", "compile", &[add(&add(&py_str("(?<![\\w'])"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("(?![\\w'])")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
pub fn __nested_swap_1(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut city: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut known: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut memory: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut name: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut names: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut out: Py = __pos.get(5).cloned().unwrap_or(Py::None);
    let mut pattern: Py = __pos.get(6).cloned().unwrap_or(Py::None);
    let mut reply: Py = __pos.get(7).cloned().unwrap_or(Py::None);
    let mut swap: Py = __pos.get(8).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(9).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    let mut name: Py = __pos.get(10).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "name").map(|(_, v)| v.clone())).unwrap_or_else(|| name.clone());
    let mut before: Py = Py::None;
    before = slice_or_raise(&gattr(&match_.clone(), "string"), &call_builtin("max", &[Py::Int(0), sub(&mcall(&match_.clone(), "start", &[]), &Py::Int(28))]).unwrap_or_else(|e| exc::raise(e)), &mcall(&match_.clone(), "start", &[]), &Py::None);
    if py_truthy(&mcall(&g_NOT_A_NAME_BEFORE(), "search", &[before.clone()])) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_THE_PLACE_IS_NAMED(), "search", &[before.clone()])))) {
    return mcall(&match_.clone(), "group", &[Py::Int(0)]);
    }
    return name.clone();
    Py::None
}
    swap = py_func("swap", move |args: &[Py]| { let mut full = vec![city.clone(), known.clone(), memory.clone(), name.clone(), names.clone(), out.clone(), pattern.clone(), reply.clone(), swap.clone()]; full.extend_from_slice(args); __nested_swap_1(&full) });
    out = mcall(&pattern.clone(), "sub", &[swap.clone(), out.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn note_the_hours(args: &[Py]) -> Py {
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
    let mut _pool: Py = Py::None;
    let mut family: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut understood: Py = Py::None;
    let _ = py_str("Keep the kind of week the line names, for the follow-up on the next turn.\n\n    The reading is bot/memory/ack.py's (`_hours_family`), so the six shapes are written down in\n    one place: a night shift, an early start, a late finish, long shifts, a rota that moves, and\n    working from home. Asked here rather than in the reply, because the hours are often answered\n    by a knowledge entry that never reaches the ack module, and the following line - \"does that\n    get easier\", \"i never know when to stop\" - is the same follow-up however it was answered.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[]))) } }) {
    return Py::None;
    }
    understood = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("understood")]) } else { Py::None };
    if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    topic = mcall(&understood.clone(), "get", &[py_str("topic")]);
    if py_truthy(&{ let __b = topic.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&topic.clone(), &py_str("work"))) } }) {
    return Py::None;
    }
    }
    let __try1 = exc::catch(|| {
    let mut ack: Py = crate::gen::memory::ack::module_obj();
    let __unpk6 = py_unpack(&crate::gen::memory::ack::_hours_family(&[mcall(&text.clone(), "lower", &[])]), 2, None);
    family = __unpk6[0].clone();
    _pool = __unpk6[1].clone();
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
    if py_truthy(&family.clone()) {
    let _ = crate::gen::memory::hours::note(&[memory.clone(), family.clone()]);
    }
    Py::None
}
pub fn _a_detail_does_not_replace_the_job(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut found: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut held: Py = Py::None;
    let mut held_frame: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_frame: Py = Py::None;
    let mut said_weight: Py = Py::None;
    let _ = py_str("A line that only says when or where they work keeps the job they already gave.\n\n    \"im a nurse\" then \"i work nights\" put \"nights\" in the store as their job and dropped the\n    nurse, so \"and my job?\" answered \"You work nights\" and the one fact they had given was gone.\n    A shift is a detail of the work, and a detail does not take the place of the fact.\n\n    The same for where they work: \"im a teacher\" then \"i work in a school\" said back \"You work in\n    a school\" and the teacher was gone, so \"what do you know about me\" lost the role altogether.\n    A place is a detail beside the role, and the summary now says both.\n    ");
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("job")))) {
    return found.clone();
    }
    held = mcall(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("job")]);
    held_frame = call_builtin("str", &[{ let __b = mcall(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("job_frame")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&found.clone(), "get", &[py_str("job")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&held.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return found.clone();
    }
    said_frame = call_builtin("str", &[{ let __b = mcall(&found.clone(), "get", &[py_str("job_frame")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    said_weight = crate::gen::memory::facts::work_weight(&[said.clone(), said_frame.clone()]);
    if py_truthy(&lt(&said_weight.clone(), &crate::gen::memory::facts::work_weight(&[held.clone(), held_frame.clone()]))) {
    kept = call_builtin("dict", &[found.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = mcall(&kept.clone(), "pop", &[py_str("job"), Py::None]);
    let _ = mcall(&kept.clone(), "pop", &[py_str("job_frame"), Py::None]);
    if py_truthy(&Py::Bool(py_eq(&said_weight.clone(), &Py::Int(2)))) {
    let _ = py_setitem(&kept.clone(), &py_str("workplace"), mcall(&add(&add(&said_frame.clone(), &py_str(" ")), &call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))), "strip", &[]));
    } else {
    let _ = py_setitem(&kept.clone(), &py_str("shift"), said.clone());
    }
    return kept.clone();
    }
    if py_truthy(&{ let __b = crate::gen::memory::facts::is_a_place_of_work(&[held_frame.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_is(&mcall(&found.clone(), "get", &[py_str("job_frame")]), &Py::None)) } }) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("workplace")])))) {
    let _ = py_setitem(&found.clone(), &py_str("workplace"), mcall(&add(&add(&held_frame.clone(), &py_str(" ")), &call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))), "strip", &[]));
    }
    }
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("job_frame")))) {
    let _ = mcall(&{ let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "pop", &[py_str("job_frame"), Py::None]);
    }
    return found.clone();
    Py::None
}
pub fn _without_the_shrug_at_the_end(args: &[Py]) -> Py {
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
    let mut clean: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The line without the chat-speak hung on the end of it, when that leaves words behind.\n\n    `bot/memory/filler` holds the shrugs people put on the end of a line, and everything the store\n    says back reads a line through it. What the store *files* did not, so the shrug became part of\n    the thing told: \"how old is my son by the way\" reached no reader at all and was answered with\n    the search hedge, and \"when do we go to cornwall by the way\" filed a place called \"cornwall\n    bies the way\" and said it back: \"You go to cornwall bies the way in August.\"\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    words = mcall(&mcall(&mcall(&said.clone(), "lower", &[]), "strip", &[py_str(" .,!;:")]), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&said.clone(), &py_str("?"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ASK_OPENERS(), &geti(&words.clone(), &Py::Int(0)))) } })) } }) {
    return said.clone();
    }
    clean = crate::gen::memory::filler::without_the_end_shrug(&[said.clone()]);
    return if py_truthy(&{ let __b = clean.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&clean.clone(), &said.clone())) } }) { clean.clone() } else { said.clone() };
    Py::None
}
pub fn observe(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut hurt_before: Py = Py::None;
    let _ = py_str("Remember what the line says, and note a second animal of a kind as the turn's news.\n\n    Everything below writes what the line says. This wrapper is here because the one thing the\n    store does with the result is not a fact but the turn: a line that leaves the store holding\n    two cats called Milo and Max is a line about the two of them, and the reply should say so.\n    The wording is not read - \"no my other cat is called max\", \"no we also have a cat called max\"\n    and \"i also have a cat called tom\" all file the second cat, and all three were left to the\n    greeting, which answered with the person's own line and \"Got it\" in front of it. A rule per\n    phrasing would have covered two of the three; the store's own record covers every phrasing.\n    ");
    before = _the_pets_on_record(&[memory.clone()]);
    let _ = py_setitem(&memory.clone(), &py_str("_pets_before"), before.clone());
    hurt_before = crate::gen::memory::whathurts::before(&[memory.clone()]);
    let __try1 = exc::catch(|| {
    let _ = _observe(&[text.clone(), memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    let _ = mcall(&memory.clone(), "pop", &[py_str("_pets_before"), Py::None]);
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    let _ = crate::gen::memory::whathurts::after(&[memory.clone(), hurt_before.clone()]);
    let _ = _note_a_pet_that_is_new(&[text.clone(), memory.clone(), before.clone()]);
    let __try6 = exc::catch(|| {
    let _ = crate::gen::memory::store_replies::_the_limit_they_share(&[text.clone(), memory.clone()]);
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret7: Py = Py::Bool(false);
    let mut __val8: Py = Py::None;
    let mut __unhandled9: Option<exc::PyExc> = None;
    match __try6 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret7 = __p[0].clone(); __val8 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled10 = false;
            if !__handled10 && exc::matches_any(&__e, &["Exception"]) {
                __handled10 = true;
            }
            if !__handled10 { __unhandled9 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled9 { exc::raise(__e); }
    if py_truthy(&__ret7) { return __val8; }
    Py::None
}
pub fn after(args: &[Py]) -> Py {
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
    let mut reply: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reply") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reply"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let _ = py_str("Keep a correction the chat was given, whichever module gave the reply to it.\n\n    \"how long does a chicken take to roast\" / \"i meant a turkey\" reads right on the turn that says\n    it - the turkey row answers the correction by itself - and the chat was still on the chicken:\n    asked \"what about a duck\" the next line reached no row at all and got a search hedge. Whether\n    the bookkeeping happened used to depend on which module won the turn, because each module\n    replies to its own copy of the memory and only the winner's copy is kept. The correction is\n    the chat's, not the reply's, so it happens here, once, after the reply has been chosen.\n    ");
    let __try1 = exc::catch(|| {
    let _ = crate::gen::memory::repair::keep_the_correction(&[text.clone(), memory.clone()]);
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
    Py::None
}
pub fn _the_ages_of_the_children(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut ages: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The reply for the ages of the children the store counts, given one at a time, or \"\".\n\n    \"i have two kids\" then \"ones 7 and ones 4\": the second line names no group word, so every\n    reader for a group passes it on, the store kept nothing, and the composer handed the line back\n    (\"Got it - ones 7 and ones 4.\"). The ages are the answer to \"how old are they?\", so they are\n    kept and said as ages. A bare list of figures is read only when the line before counted the\n    children, because \"7 and 4\" on its own could be anything.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    held = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("kids")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&held.clone(), &Py::Int(2)) } }) {
    return py_str("");
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(gt(&call_builtin("len", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("ages")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return py_str("");
    }
    ages = crate::gen::memory::facts::ages_said_one_by_one(&[text.clone(), held.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&ages.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = crate::gen::memory::facts::the_ones_are_given_one_at_a_time(&[text.clone()]); if py_truthy(&__b) { __b } else { _the_count_was_the_line_before(&[memory.clone()]) } }))) {
    return py_str("");
    }
    let _ = py_call(&crate::gen::memory::store_replies::g_remember_other(), &[memory.clone(), py_dict(vec![(py_str("who"), py_str("kids")), (py_str("ages"), ages.clone())]), text.clone()]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&ages.clone()) { let mut age: Py = __item.clone(); __out.push(mcall(&g_AGES_IN_WORDS(), "get", &[age.clone(), call_builtin("str", &[age.clone()]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() };
    said = if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) { add(&add(&mcall(&py_str(", "), "join", &[slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&words.clone(), &neg(&Py::Int(1)))) } else { mcall(&py_str(" and "), "join", &[words.clone()]) };
    return py_call(&crate::gen::memory::store_replies::g_pick(), &[py_list(vec![mcall(&py_str("{} — got it."), "format", &[add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None))]), mcall(&py_str("Got it — {}."), "format", &[said.clone()]), mcall(&py_str("{}."), "format", &[add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None))])]), memory.clone()]);
    Py::None
}
pub fn _the_count_was_the_line_before(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line before this one counted the children, so a bare list of ages is theirs.");
    said = call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("topic")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[stdlib_call("re", "search", &[py_str("\\b(?:kids?|children|child)\\b"), said.clone(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_pets_on_record(args: &[Py]) -> Py {
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
    let mut held: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The (animal, name) pairs the store holds, for comparing one turn with the next.");
    held = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return held.clone();
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = mcall(&held.clone(), "add", &[py_tuple(vec![mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])])]);
    }
    return held.clone();
    Py::None
}
pub fn _note_a_pet_that_is_new(args: &[Py]) -> Py {
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
    let mut before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut added: Py = Py::None;
    let mut animal: Py = Py::None;
    let mut held: Py = Py::None;
    let mut name: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut same: Py = Py::None;
    let _ = py_str("A line that leaves two of an animal on record is answered as a line about the two.\n\n    Nothing is read out of the words: the store is compared with itself, so a phrasing nobody\n    wrote a rule for is answered the same as the ones people type most. A first pet is left\n    alone - bot/memory/ack.py already answers the line that gives one (\"A cat called Milo.\n    What's it like to live with?\") - and so is a rename, which leaves one animal behind.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    held = mcall(&memory.clone(), "get", &[py_str("correction")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&held.clone(), "get", &[py_str("said")]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))) } }) {
    return Py::None;
    }
    added = sub(&_the_pets_on_record(&[memory.clone()]), &before.clone());
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[added.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    animal = __unpk2[0].clone();
    name = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&animal.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&name.clone())) } }) {
    continue;
    }
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &animal.clone())); if !py_truthy(&__b) { __b } else { mcall(&pet.clone(), "get", &[py_str("name")]) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[same.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    continue;
    }
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pets_more")), (py_str("value"), name.clone()), (py_str("animal"), animal.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), (py_str("text"), text.clone())]));
    return geti(&memory.clone(), &py_str("correction"));
    }
    return Py::None;
    Py::None
}
pub fn _observe(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut answered: Py = Py::None;
    let mut budget: Py = Py::None;
    let mut correction: Py = Py::None;
    let mut counted: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut folded: Py = Py::None;
    let mut found: Py = Py::None;
    let mut held: Py = Py::None;
    let mut held_name: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut known: Py = Py::None;
    let mut more: Py = Py::None;
    let mut named_here: Py = Py::None;
    let mut one: Py = Py::None;
    let mut planned: Py = Py::None;
    let mut pronoun: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut retracted: Py = Py::None;
    let mut said: Py = Py::None;
    let mut second: Py = Py::None;
    let mut sign: Py = Py::None;
    let mut understood: Py = Py::None;
    let mut value: Py = Py::None;
    let mut was: Py = Py::None;
    let mut who: Py = Py::None;
    let mut ruled_out_place: Py = crate::gen::memory::not_this::module_obj();
    if py_truthy(&mcall(&ruled_out_place.clone(), "the_place_ruled_out", &[text.clone(), memory.clone()])) {
    return Py::None;
    }
    understood = mcall(&memory.clone(), "get", &[py_str("understood")]);
    let _ = note_the_hours(&[text.clone(), memory.clone()]);
    let _ = crate::gen::memory::counts::observe(&[text.clone(), memory.clone()]);
    let _ = crate::gen::memory::surname::observe(&[text.clone(), memory.clone()]);
    planned = if py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { crate::gen::memory::plans::answer(&[text.clone(), memory.clone()]) } else { Py::None };
    if py_truthy(&planned.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("plan_answer"), py_dict(vec![(py_str("text"), planned.clone()), (py_str("user"), text.clone())]));
    }
    kind = if py_truthy(&call_builtin("isinstance", &[understood.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&understood.clone(), "get", &[py_str("kind")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = crate::gen::memory::store_replies::_behind_a_remembering_frame(&[text.clone()]);
    if py_truthy(&said.clone()) {
    text = said.clone();
    }
    if py_truthy(&{ let __b = crate::gen::memory::recall::ask(&[text.clone()]); if py_truthy(&__b) { __b } else { _reads_the_store(&[text.clone(), memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_REMIND_QUESTION(), "search", &[text.clone()])) {
    return Py::None;
    }
    if py_truthy(&_asks_about_someone_else(&[text.clone()])) {
    rest = crate::gen::memory::store_replies::_after_the_yes_no_frame(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return Py::None;
    }
    text = rest.clone();
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_statement(&[text.clone()]) } }) {
    return Py::None;
    }
    let _ = crate::gen::memory::decided::observe(&[text.clone(), memory.clone()]);
    let _ = crate::gen::memory::company::observe(&[text.clone(), memory.clone()]);
    let _ = mcall(&memory.clone(), "pop", &[py_str("pet_age"), Py::None]);
    let _ = crate::gen::memory::times::keep(&[text.clone(), memory.clone()]);
    let _ = crate::gen::memory::rates::keep(&[text.clone(), memory.clone()]);
    found = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("question".to_string(), Py::Bool(py_eq(&kind.clone(), &py_str("question"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::extract(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = crate::gen::memory::facts::a_correction_without_a_subject(&[text.clone(), py_call(&crate::gen::memory::store_replies::g_last_user(), &[memory.clone()])]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("name")])))) {
    answered = _their_name_answered(&[text.clone(), memory.clone()]);
    if py_truthy(&answered.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("name"), answered.clone());
    }
    }
    found = _with_repaired(&[found.clone(), text.clone(), memory.clone(), kind.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("kids")])))) {
    counted = crate::gen::memory::facts::count_of_the_people(&[text.clone(), memory.clone()]);
    if py_truthy(&counted.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("kids"), counted.clone());
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("budget")])))) {
    folded = py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = folded.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&folded.clone(), &mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]))) } }) {
    budget = mcall(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(folded.clone()); __kws.extend(vec![("question".to_string(), Py::Bool(py_eq(&kind.clone(), &py_str("question"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::extract(args))(&__args) }, "get", &[py_str("budget")]);
    if py_truthy(&budget.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("budget"), budget.clone());
    }
    }
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("budget")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("[\\u00a3$\\u20ac]"), geti(&found.clone(), &py_str("budget"))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    budget = call_builtin("str", &[geti(&found.clone(), &py_str("budget"))]).unwrap_or_else(|e| exc::raise(e));
    sign = _budget_sign(&[text.clone(), budget.clone()]);
    if py_truthy(&sign.clone()) {
    let _ = py_setitem(&found.clone(), &py_str("budget"), add(&sign.clone(), &mcall(&budget.clone(), "lstrip", &[])));
    }
    }
    correction = mcall(&found.clone(), "pop", &[py_str("correction"), Py::None]);
    if py_truthy(&Py::Bool(py_is(&correction.clone(), &Py::None))) {
    correction = _a_correction_without_a_marker(&[text.clone(), memory.clone()]);
    }
    dropped = mcall(&found.clone(), "pop", &[py_str("drop"), Py::None]);
    retracted = mcall(&found.clone(), "pop", &[py_str("retract"), Py::None]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("allergies")])))) {
    more = crate::gen::memory::facts::one_more(&[text.clone()]);
    held = { let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&{ let __b = more.clone(); if !py_truthy(&__b) { __b } else { held.clone() } }) {
    let _ = py_setitem(&found.clone(), &py_str("allergies"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&more.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&held.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() });
    }
    }
    named_here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&found.clone(), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut who: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[who.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(who.clone()); } } py_list(__out) })() };
    let _ = _remember_dislikes(&[text.clone(), memory.clone(), named_here.clone()]);
    second = _a_second_persons_limit(&[text.clone(), memory.clone()]);
    if py_truthy(&second.clone()) {
    let _ = mcall(&mcall(&found.clone(), "setdefault", &[py_str("others"), py_list(vec![])]), "append", &[second.clone()]);
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&found.clone(), "pop", &[py_str("others"), py_list(vec![])]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entry = __item.clone();
    pronoun = mcall(&entry.clone(), "pop", &[py_str("pronoun"), Py::None]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&entry.clone(), "get", &[py_str("who")])))) {
    if py_truthy(&{ let __b = pronoun.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("she"), py_str("he")]), &pronoun.clone())) } }) {
    age = crate::gen::memory::store_replies::_a_number(&[mcall(&entry.clone(), "get", &[py_str("age")])]);
    if py_truthy(&age.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("their_age"), py_dict(vec![(py_str("age"), age.clone()), (py_str("turns"), { let __b = mcall(&entry.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_str("") } })]));
    }
    continue;
    }
    who = py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone(), named_here.clone()]);
    if py_truthy(&pronoun.clone()) {
    fits = if py_truthy(&Py::Bool(py_eq(&pronoun.clone(), &py_str("she")))) { crate::gen::memory::facts::g_SHE() } else { crate::gen::memory::facts::g_HE() };
    who = if py_truthy(&crate::gen::memory::relations::_pronoun_fits(&[who.clone(), fits.clone()])) { who.clone() } else { py_str("") };
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("she"), py_str("he")]), &pronoun.clone())); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[mcall(&entry.clone(), "get", &[py_str("months")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } }) {
    let _ = py_setitem(&entry.clone(), &py_str("pronoun"), pronoun.clone());
    let _ = py_call(&crate::gen::memory::store_replies::g_remember_other(), &[memory.clone(), entry.clone(), text.clone()]);
    }
    continue;
    }
    let _ = py_setitem(&entry.clone(), &py_str("who"), who.clone());
    }
    if py_truthy(&{ let __b = pronoun.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&entry.clone(), &py_str("age"))); if !py_truthy(&__b) { __b } else { mcall(&g_CORRECTING(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } } } }) {
    known = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &geti(&entry.clone(), &py_str("who"))))) {  __out.push(one.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = known.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("age"))) } }))) {
    continue;
    }
    }
    was = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &mcall(&entry.clone(), "get", &[py_str("who")])))) {  __out.push(call_builtin("dict", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_call(&crate::gen::memory::store_replies::g_remember_other(), &[memory.clone(), entry.clone(), text.clone()]);
    let _ = _note_a_corrected_fact(&[memory.clone(), text.clone(), entry.clone(), was.clone()]);
    let _ = _note_a_fact_moved(&[memory.clone(), text.clone(), entry.clone(), was.clone()]);
    }
    found = _a_detail_does_not_replace_the_job(&[found.clone(), memory.clone()]);
    held_name = call_builtin("str", &[{ let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&mcall(&found.clone(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    key = __unpk3[0].clone();
    value = __unpk3[1].clone();
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), key.clone(), value.clone(), text.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { { let __b = held_name.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[geti(&found.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&held_name.clone(), "lower", &[]))) } } } }) {
    let _ = py_setitem(&memory.clone(), &py_str("name_fixed"), py_dict(vec![(py_str("name"), call_builtin("str", &[geti(&found.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))), (py_str("from"), held_name.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    }
    if py_truthy(&{ let __b = mcall(&found.clone(), "get", &[py_str("job")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&found.clone(), "get", &[py_str("job_frame")]))) } }) {
    let _ = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "pop", &[py_str("job_frame"), Py::None]);
    }
    let _ = _fix_a_pets_name(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("pets")))) {
    let _ = _name_the_last_pet(&[text.clone(), memory.clone()]);
    let _ = _age_the_named_pet(&[text.clone(), memory.clone()]);
    let _ = _age_they_answered(&[text.clone(), memory.clone()]);
    let _ = _age_said_of_the_only_animal(&[text.clone(), memory.clone()]);
    let _ = _fix_a_pets_age(&[text.clone(), memory.clone()]);
    }
    let _ = _fix_a_persons_age(&[text.clone(), memory.clone()]);
    let _ = _fix_the_weeks_of_a_pregnancy(&[text.clone(), memory.clone()]);
    fixed = _fix_a_persons_name(&[text.clone(), memory.clone()]);
    if py_truthy(&fixed.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("name_fixed"), { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(fixed.clone()); __kws.extend(vec![("said".to_string(), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) });
    }
    if py_truthy(&Py::Bool(!py_contains(&found.clone(), &py_str("owns")))) {
    let _ = _fix_the_make_of_a_thing(&[text.clone(), memory.clone()]);
    }
    let _ = _the_name_they_gave_to_the_person_they_mean(&[text.clone(), memory.clone()]);
    let _ = _a_trip_booked_named(&[text.clone(), memory.clone()]);
    if py_truthy(&correction.clone()) {
    let _ = crate::gen::memory::corrected::_apply_correction(&[memory.clone(), correction.clone(), text.clone()]);
    let _ = _fix_a_pets_name(&[text.clone(), memory.clone()]);
    return Py::None;
    }
    if py_truthy(&{ let __b = dropped.clone(); if !py_truthy(&__b) { __b } else { _drop_value(&[memory.clone(), dropped.clone(), text.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = retracted.clone(); if !py_truthy(&__b) { __b } else { _retract_last(&[memory.clone(), text.clone()]) } }) {
    return Py::None;
    }
    let _ = _forget_denied(&[text.clone(), memory.clone()]);
    let _ = _the_back_on_the_plate(&[text.clone(), memory.clone()]);
    if py_truthy(&crate::gen::memory::store_replies::_about_this_chat(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&crate::gen::memory::told::a_callback_the_chat_has(&[text.clone(), memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_this_line_put_something_right(&[memory.clone(), text.clone()])))) {
    let _ = py_call(&crate::gen::memory::store_replies::g_note(), &[memory.clone(), text.clone(), kind.clone()]);
    }
    Py::None
}
pub fn _a_trip_booked_named(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let mut one: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut when: Py = Py::None;
    let _ = py_str("A trip that is already booked is a trip the chat is on, and the store keeps it.\n\n    \"we booked a cottage in cornwall for august\" was answered about Cornwall and then kept\n    nothing: \"where are we going\" said \"You haven't said - where are you going?\", and a correction\n    after it had no plan to land on. The place is filed, and the plan in the shape the other trips\n    are kept in (\"Cornwall in August\"), unless the store is already on that plan.\n    ");
    let mut trips: Py = crate::gen::memory::trips::module_obj();
    let __unpk1 = py_unpack(&mcall(&trips.clone(), "the_trip_they_booked", &[text.clone()]), 2, None);
    place = __unpk1[0].clone();
    when = __unpk1[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&place.clone()))) {
    return Py::None;
    }
    known = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[known.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    place = add(&mcall(&slice_or_raise(&place.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&place.clone(), &Py::Int(1), &Py::None, &Py::None));
    plan = if py_truthy(&when.clone()) { mcall(&py_str(" "), "join", &[mcall(&{ let mut __s = String::new(); __s.push_str(&fstr(&place.clone(), -1, "")); __s.push_str(" in "); __s.push_str(&fstr(&when.clone(), -1, "")); py_string(__s) }, "split", &[])]) } else { place.clone() };
    places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("places")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "lower", &[])); } py_set(__out) })() }, &mcall(&place.clone(), "lower", &[])))) {
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("places"), place.clone(), text.clone()]);
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("plan")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &mcall(&plan.clone(), "lower", &[])))) {
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("plan"), plan.clone(), text.clone()]);
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("plans"), plan.clone(), text.clone()]);
    }
    Py::None
}
pub fn _this_line_put_something_right(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut kept: Py = Py::None;
    let mut key: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("Whether this very line is the correction one of the readers has just applied.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("correction"), py_str("name_fixed")]));
    for __item in __iter1 {
    key = __item.clone();
    kept = mcall(&memory.clone(), "get", &[key.clone()]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[kept.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&said.clone(), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&kept.clone(), "get", &[py_str("said")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _on_record_before(args: &[Py]) -> Py {
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
    let mut records: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "records") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: records"))) };
    let mut kept: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The animals an earlier line put on record, out of the ones this line has just taken back.\n\n    A take-back only says itself when there was something to take back. \"i dont have a rabbit\"\n    from someone who has a cat is a thing they never had, and \"Right — no rabbit, then.\"\n    answers a line they did not say; the ordinary answer to it is the one they get.\n    ");
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = turn.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() }]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(py_call(&crate::gen::memory::store_replies::g_stem(), &[word.clone()])); } py_set(__out) })() };
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&{ let __b = records.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[record.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_tuple(vec![mcall(&record.clone(), "get", &[py_str("animal")]), mcall(&record.clone(), "get", &[py_str("name")])]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&words.clone(), &py_call(&crate::gen::memory::store_replies::g_stem(), &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]))) } }) {
    let _ = mcall(&kept.clone(), "append", &[record.clone()]);
    break;
    }
    }
    }
    return kept.clone();
    Py::None
}
pub fn _forget_denied(args: &[Py]) -> Py {
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
    let mut denied: Py = Py::None;
    let mut died: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut given: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A line that says they do not have a fact takes the old one out of the store.\n\n    \"im coeliac\" and then \"im not coeliac\", \"im allergic to peanuts and nuts\" and then \"im not\n    allergic to nuts\": the same fact being taken back. A store that keeps it goes on warning\n    about, or cooking around, the thing the person has just said they are fine with.\n    ");
    held = { let __b = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    denied = crate::gen::memory::facts::denials(&[text.clone(), held.clone()]);
    theirs = if py_truthy(&call_builtin("isinstance", &[denied.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&denied.clone(), "pop", &[py_str("theirs"), Py::None]) } else { Py::None };
    died = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_losses_taking(&[text.clone(), held.clone(), memory.clone()])) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&denied.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(crate::gen::memory::store_replies::_kind(&[item.clone()])); } py_list(__out) })() }, &call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    given = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::facts::given_away_one(&[text.clone(), held.clone()])) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&denied.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(item.clone()); } } py_list(__out) })() }, &pet.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&denied.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&theirs.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&died.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&given.clone())) } } } } } }) {
    return Py::None;
    }
    gone = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&denied.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    key = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("diet")))) {
    value = _the_diet_they_are_back_on(&[memory.clone(), value.clone()]);
    }
    dropped = py_call(&crate::gen::memory::store_replies::g_forget(), &[memory.clone(), key.clone(), value.clone(), py_str("gone")]);
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pets")))) {
    dropped = _on_record_before(&[memory.clone(), dropped.clone()]);
    }
    if py_truthy(&dropped.clone()) {
    let _ = py_setitem(&gone.clone(), &key.clone(), dropped.clone());
    if py_truthy(&{ let __b = Py::Bool(py_eq(&key.clone(), &py_str("pets"))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }, &py_str("*"))) } }) {
    let _ = py_setitem(&gone.clone(), &py_str("pets_all"), Py::Bool(true));
    }
    }
    }
    if py_truthy(&{ let __b = died.clone(); if !py_truthy(&__b) { __b } else { py_call(&crate::gen::memory::store_replies::g_forget(), &[memory.clone(), py_str("pets"), died.clone(), py_str("died")]) } }) {
    let _ = py_setitem(&gone.clone(), &py_str("pets_died"), Py::Bool(true));
    }
    if py_truthy(&{ let __b = given.clone(); if !py_truthy(&__b) { __b } else { py_call(&crate::gen::memory::store_replies::g_forget(), &[memory.clone(), py_str("pets"), given.clone(), py_str("given away")]) } }) {
    let _ = py_setitem(&gone.clone(), &py_str("pets_given"), Py::Bool(true));
    }
    let __iter3 = py_iter_or_raise(&{ let __b = theirs.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter3 {
    entry = __item.clone();
    taken = _off_their_list(&[memory.clone(), entry.clone(), text.clone()]);
    if py_truthy(&taken.clone()) {
    let _ = mcall(&gone.clone(), "update", &[taken.clone()]);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&gone.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&gone.clone(), &py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]));
    let _ = py_setitem(&memory.clone(), &py_str("denial"), gone.clone());
    return gone.clone();
    Py::None
}
pub fn _the_back_on_the_plate(args: &[Py]) -> Py {
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
    let mut food: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut likes: Py = Py::None;
    let mut record: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("A line that puts back what somebody else does not eat takes it off their record.\n\n    \"my son doesnt eat fish\" then \"actually he eats fish\": the store kept \"no fish\" against him\n    and went on answering \"what does my son eat\" with \"Your son doesn't eat fish.\", which is the\n    opposite of the last thing the person said. A taste is the same shape - \"my wife hates\n    mushrooms\" then \"actually she loves them\" puts mushrooms on her likes and off her dislikes.\n    Only what is already on that person's record is lifted, so a line that describes somebody for\n    the first time (\"my son eats fish\") is left to the modules that read it on its own.\n    ");
    let __iter1 = py_iter_or_raise(&crate::gen::memory::facts::their_back(&[text.clone()]));
    for __item in __iter1 {
    said = __item.clone();
    who = { let __b = mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]); if py_truthy(&__b) { __b } else { py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    continue;
    }
    let __iter2 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter2 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&who.clone(), "lower", &[])))) {
    continue;
    }
    kind = mcall(&said.clone(), "get", &[py_str("kind")]);
    food = mcall(&call_builtin("str", &[{ let __b = mcall(&said.clone(), "get", &[py_str("food")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("them"), py_str("it"), py_str("that")]), &mcall(&food.clone(), "lower", &[])))) {
    food = _the_one_they_are_back_on(&[record.clone(), kind.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&food.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("diet")))) {
    held = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![add(&py_str("no "), &mcall(&food.clone(), "lower", &[])), add(&add(&py_str("no "), &mcall(&food.clone(), "lower", &[])), &py_str("s"))]), &held.clone()))) {
    continue;
    }
    let _ = py_setitem(&record.clone(), &py_str("diet"), add(&py_str("eats "), &food.clone()));
    } else {
    gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("dislikes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut item: Py = __item.clone(); __out.push(mcall(&item.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&food.clone(), "lower", &[])))) {
    continue;
    }
    let _ = py_setitem(&record.clone(), &py_str("dislikes"), { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&item.clone(), "lower", &[]), &mcall(&food.clone(), "lower", &[])))) {  __out.push(item.clone()); } } py_list(__out) })() });
    likes = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("likes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&likes.clone()) { let mut item: Py = __item.clone(); __out.push(mcall(&item.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&food.clone(), "lower", &[])))) {
    let _ = mcall(&likes.clone(), "append", &[food.clone()]);
    }
    let _ = py_setitem(&record.clone(), &py_str("likes"), likes.clone());
    }
    let _ = py_setitem(&memory.clone(), &py_str("reversal"), py_dict(vec![(py_str("who"), who.clone()), (py_str("food"), food.clone()), (py_str("kind"), kind.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_one_they_are_back_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut record: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "record") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: record"))) };
    let mut kind: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "kind") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: kind"))) };
    let mut gone: Py = Py::None;
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let _ = py_str("The one thing on a person's record a \"them\" can point at, or \"\" when there are two.\n\n    \"my wife hates mushrooms\" then \"actually she loves them\" is mushrooms because they are the\n    only thing she does not like; with mushrooms and olives both on the list the pointing word\n    says nothing, and guessing one of them would put a taste in her mouth she never gave.\n    ");
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("diet")))) {
    held = mcall(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("diet")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    return if py_truthy(&mcall(&mcall(&held.clone(), "lower", &[]), "startswith", &[py_str("no ")])) { mcall(&slice_or_raise(&held.clone(), &Py::Int(3), &Py::None, &Py::None), "strip", &[]) } else { py_str("") };
    }
    gone = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("dislikes")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    return if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[gone.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) { geti(&gone.clone(), &Py::Int(0)) } else { py_str("") };
    Py::None
}
pub fn _off_their_list(args: &[Py]) -> Py {
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
    let mut entry: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut held: Py = Py::None;
    let mut item: Py = Py::None;
    let mut items: Py = Py::None;
    let mut left: Py = Py::None;
    let mut one: Py = Py::None;
    let mut record: Py = Py::None;
    let mut taken: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The take-back a line makes about somebody else, taken off the record kept for them.\n\n    \"my son is allergic to nuts\" then \"he isnt allergic to nuts\": the store files facts about\n    other people under their own entry, so the denial has to find that entry and name the person\n    it belongs to when it says the note back - \"Nuts back on the table, then.\" reads as the\n    speaker's own allergy, which they never mentioned.\n\n    The note is said even when they never had it on record: \"my daughter is 6\" then \"she isnt\n    allergic to eggs\" is a thing the person said about her, and the fallback's \"I'm listening\"\n    is no answer to it.\n    ");
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); if py_truthy(&call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&items.clone()))) {
    return Py::None;
    }
    who = { let __b = call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    taken = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    record = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&record.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &who.clone()))) {
    continue;
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&record.clone(), "get", &[py_str("allergies")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut item: Py = __item.clone(); __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&items.clone()) { let mut one: Py = __item.clone(); __out.push(mcall(&one.clone(), "lower", &[])); } py_set(__out) })() }, &mcall(&item.clone(), "lower", &[])))) {  __out.push(item.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = mcall(&taken.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&left.clone(), &item.clone()))) {  __out.push(item.clone()); } } py_list(__out) })() }]);
    let _ = py_setitem(&record.clone(), &py_str("allergies"), left.clone());
    }
    return py_dict(vec![(py_str("allergies"), items.clone()), (py_str("who"), who.clone())]);
    Py::None
}
pub fn _the_diet_they_are_back_on(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut held: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The diet as the store holds it, when the take-back names only the food.\n\n    \"im cutting out sugar\" is kept as that whole phrase, and the line that takes it back can\n    name nothing but the food - \"im back on sugar\", \"im eating bread again\". Matched against the\n    phrase word by word, or `forget` finds nothing and the store goes on telling them to avoid\n    what they eat now.\n    ");
    held = mcall(&{ let __b = mcall(&memory.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("diet")]);
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return value.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&mcall(&py_str(" "), "join", &[mcall(&mcall(&held.clone(), "lower", &[]), "split", &[])]), &said.clone()))) {
    return held.clone();
    }
    return value.clone();
    Py::None
}
pub fn _the_last_line(args: &[Py]) -> Py {
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
    let _ = py_str("The line just before this one, so a bare \"he died\" can point back at what it named.\n\n    \"i have a dog\" then \"he died\" is the dog; \"my grandad is in hospital\" then \"he died\" is the\n    grandad. Without the turn behind it a \"he\" says too little, and taking the only animal off the\n    store on that is the worst thing this reader could do.\n    ");
    turns = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&{ let __b = geti(&turns.clone(), &neg(&Py::Int(1))); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _losses_taking(args: &[Py]) -> Py {
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
    let mut held: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { Py::None };
    let mut before: Py = Py::None;
    let mut everything: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut named: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut same: Py = Py::None;
    let mut taken: Py = Py::None;
    let _ = py_str("The animals a line about a death takes off the store, one record at a time.\n\n    \"my cat died\" from someone with two cats is one of them: the store keeps the other instead of\n    answering \"you haven't told me about a pet yet\" a turn after they grieved one. \"both cats\n    died\" is the whole lot.\n    ");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    everything = call_builtin("bool", &[mcall(&crate::gen::memory::facts::g_LOSS_ALL(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    taken = py_list(vec![]);
    before = _the_last_line(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&crate::gen::memory::facts::losses(&[text.clone(), held.clone(), before.clone()]));
    for __item in __iter1 {
    kind = __item.clone();
    same = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&py_call(&crate::gen::memory::store_replies::g__animal_family(), &[mcall(&pet.clone(), "get", &[py_str("animal")])]), &py_call(&crate::gen::memory::store_replies::g__animal_family(), &[kind.clone()]))) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&same.clone()))) {
    continue;
    }
    if py_truthy(&everything.clone()) {
    let _ = mcall(&taken.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&same.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&taken.clone(), &pet.clone()))) {  __out.push(pet.clone()); } } py_list(__out) })() }]);
    continue;
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&same.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    let _ = mcall(&taken.clone(), "append", &[if py_truthy(&named.clone()) { geti(&named.clone(), &neg(&Py::Int(1))) } else { geti(&same.clone(), &neg(&Py::Int(1))) }]);
    }
    return taken.clone();
    Py::None
}
pub fn _last_stated(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The facts the line before this one put in the store.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = py_call(&crate::gen::memory::store_replies::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_list(vec![]);
    }
    kept = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&crate::gen::memory::log(&[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &said.clone()))) {
    continue;
    }
    let _ = mcall(&kept.clone(), "append", &[entry.clone()]);
    }
    return kept.clone();
    Py::None
}
pub fn _retract_last(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut animal: Py = Py::None;
    let mut dropped: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut item: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut shown: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("A line that takes back the last thing they said without naming it: \"take that back\", \"scratch\n    that\".\n\n    \"im allergic to peanuts and shellfish\" then \"take that back\" left the allergy on record and the\n    next ask answered \"You told me you're allergic to peanuts, shellfish\" - a note they had just\n    asked to take off. A retraction names no value, so the only thing it can be about is the line\n    before it: every fact that line put in the store goes. A taste or a fact about someone else is\n    left alone - those are not what \"take that back\" means, and losing them silently would be worse\n    than keeping them.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![])]), 2, None);
    gone = __unpk1[0].clone();
    shown = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&_last_stated(&[memory.clone()]));
    for __item in __iter2 {
    entry = __item.clone();
    key = mcall(&entry.clone(), "get", &[py_str("key")]);
    value = mcall(&entry.clone(), "get", &[py_str("value")]);
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("pet")))) {
    known = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]);
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_is(&pet.clone(), &value.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&pet.clone(), &value.clone())) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&known.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&pets.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("pets"), pets.clone());
    } else {
    let _ = mcall(&known.clone(), "pop", &[py_str("pets"), Py::None]);
    }
    let _ = mcall(&gone.clone(), "append", &[py_str("pets")]);
    animal = { let __b = mcall(&{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("pet") } } } };
    let _ = mcall(&shown.clone(), "append", &[call_builtin("str", &[animal.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&key.clone(), &py_str("plans")))) {
    known = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]);
    if py_truthy(&py_call(&crate::gen::memory::store_replies::g_forget(), &[memory.clone(), py_str("plans"), py_list(vec![value.clone()])])) {
    let _ = mcall(&gone.clone(), "append", &[py_str("plans")]);
    let _ = mcall(&shown.clone(), "append", &[call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("plan")]), &value.clone()))) {
    let _ = mcall(&known.clone(), "pop", &[py_str("plan"), Py::None]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::memory::store_lines::g_NOT_RETRACTED(), &key.clone()))) {
    continue;
    }
    dropped = py_call(&crate::gen::memory::store_replies::g_forget(), &[memory.clone(), key.clone(), value.clone()]);
    if py_truthy(&dropped.clone()) {
    let _ = mcall(&gone.clone(), "append", &[key.clone()]);
    let _ = mcall(&shown.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&dropped.clone()) { let mut item: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[item.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {  __out.push(call_builtin("str", &[item.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() }]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&gone.clone()))) {
    return Py::None;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&gone.clone()) { let mut key: Py = __item.clone(); __out.push(Py::Bool(py_contains(&py_tuple(vec![py_str("plans"), py_str("places")]), &key.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = py_setitem(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), &py_str("plan_dropped"), call_builtin("str", &[geti(&shown.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)));
    }
    let _ = py_setitem(&memory.clone(), &py_str("retraction"), py_dict(vec![(py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])), (py_str("keys"), gone.clone()), (py_str("shown"), shown.clone())]));
    return geti(&memory.clone(), &py_str("retraction"));
    Py::None
}
pub fn _reads_the_store(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("True for a line that only asks for something already kept, so it states nothing new.\n\n    `facts` extracts what a line mentions, and an ask mentions the very thing it wants back:\n    \"i'm allergic to nuts\" then \"am i allergic to peanuts or shellfish?\" used to keep both and\n    answer *you're allergic to nuts, peanuts or shellfish* - a claim they never made.\n\n    An ask about a person's allergy or a food they cannot have is the same kind of line: it\n    names nothing of its own and asks for a fact the store keeps (\"my son is allergic to nuts\"\n    then \"what is he allergic to\" was answered with what an allergy is, by a topic entry that\n    matched the word). Said with nobody the store knows, it claims nothing.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = crate::gen::memory::store_replies::_bare(&[py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = mcall(&g_NAMED_FACT_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_TOPIC_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_ASK_WITHOUT_MY(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::modules::memory_tables::g_HOW_MANY_OF_MINE(), "search", &[low.clone()]) } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::store_replies::g_OTHER_FACT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_CANT_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_CAN_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_CAN_THEY_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_a_bare_limit_ask(&[low.clone(), memory.clone()]) } } } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::memory::store_replies::_asks_about_someone(&[low.clone(), text.clone(), memory.clone()])))) {
    return Py::Bool(false);
    }
    whole = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&crate::gen::memory::store_replies::g_STATES_A_LIMIT(), "search", &[whole.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::memory::store_replies::g_A_HAVE_QUESTION(), "search", &[whole.clone()]))) } }) {
    return Py::Bool(false);
    }
    said = crate::gen::memory::store_replies::_other_person_reply(&[{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, low.clone()]);
    return call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_plan_they_took_back(args: &[Py]) -> Py {
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
    let mut gone: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("A where ask about a trip they have just taken back is the store's line, not a repeat.\n\n    \"im going to lisbon in may\" / \"take it back\" / \"where am i going\" was answered \"Same as\n    before: In Lisbon: Alfama on foot and the 28 tram...\" - the trip's paragraph read back to\n    someone the bot had just told \"Right, that place is off the record, then.\" The ask names no\n    subject of its own, so the knowledge base leaned on the topic record for the line they had\n    just retracted and the answer on the screen came back. The store is the one part that knows\n    the plan went: its own answer to a where ask names what was taken back (\"Lisbon is off, then\n    - where are you going?\"). Only a where ask, and only while the thing they took back is the\n    newest thing in the store, so every other line after a take-back is read exactly as before.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    gone = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("plan_dropped")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[gone.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&gone.clone(), "strip", &[]))) } }) {
    return Py::Bool(false);
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return call_builtin("bool", &[mcall(&crate::gen::memory::store_replies::g_WHERE_PLAN(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_clock_in_their_line(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the ask wants a clock or a day the person gave, and the store can place it.\n\n    \"what time is my appointment\" after \"i have a dentist appointment on friday at 3\" was\n    answered \"Ring practices near you and ask straight out whether they take new NHS patients\":\n    the store had the hour and proposed it, but the claim that hands the store its own asks was\n    gated on the word \"when\", so a clock ask fell to the knowledge base's entry about the same\n    noun. The hour or the day is in what the person said, so the ask is the store's - and every\n    such ask it cannot place (no plan naming it, nothing to read the hour off) is left alone,\n    which keeps \"what time do i start work\" with whoever can answer it.\n    ");
    let __try1 = exc::catch(|| {
    low = mcall(&text.clone(), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::store_replies::g_TIME_ASKS(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_TIME_THING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_TIME_IT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_DAY_THING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::memory::store_replies::g_DAY_IT(), "search", &[low.clone()]) } } } } } } } }))) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[crate::gen::memory::store_replies::_plans_reply(&[memory.clone(), low.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _an_ask_about_someone_else(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut known: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("True when the line asks about a fact the store keeps for someone else.\n\n    \"i'm cooking for my mother in law on sunday\" then \"shes vegetarian\" then \"is she vegetarian\"\n    is an ask about the mother in law, and the diet is on record; the knowledge base reads the\n    word \"vegetarian\" and offers a curry instead, so the store has to say the line is its own\n    (`TAKES_AS_IT_COMES` below). Only a line that names or points at a person whose fact is on\n    record is claimed: with nothing stored for them the ask goes to whoever can answer it.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_ASK_ABOUT_THEM(), "search", &[mcall(&text.clone(), "strip", &[])])))) {
    return Py::Bool(false);
    }
    keys = py_list(vec![]);
    if py_truthy(&mcall(&crate::gen::memory::store_replies::g_OTHER_FACT(), "search", &[text.clone()])) {
    let _ = mcall(&keys.clone(), "append", &[py_str("allergies")]);
    }
    if py_truthy(&mcall(&crate::gen::memory::store_replies::g_EAT_ASK(), "search", &[text.clone()])) {
    let _ = mcall(&keys.clone(), "append", &[py_str("diet")]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&keys.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    who = crate::gen::memory::store_replies::_someone_else_named(&[crate::gen::memory::store_replies::_bare(&[py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]), memory.clone()]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::Bool(false);
    }
    let __iter6 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter6 {
    known = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone()))) {
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(mcall(&known.clone(), "get", &[key.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_take_back_about_someone(args: &[Py]) -> Py {
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
    let mut gone: Py = Py::None;
    let _ = py_str("A line that takes a fact back off somebody else's list is the store's, not the topic's.\n\n    \"my son is allergic to nuts\" then \"he isnt allergic to nuts\" was answered with the nut-free\n    lecture from the knowledge base: the line names nuts, so `answers` read food on it and took\n    the turn from the store that had just taken them off his list. The store's note is about\n    what the chat keeps, not about nuts, so it says so in front of its reply.\n    ");
    gone = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("denial")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[gone.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&gone.clone(), "get", &[py_str("who")]))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(py_eq(&mcall(&gone.clone(), "get", &[py_str("said")]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])));
    Py::None
}
pub fn _the_part_that_hurts_asked(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("The ask for the part that hurts, and the side it was said with.\n\n    \"my knee hurts\", then \"its on the outside of the knee\", then \"and which knee was it\" is\n    answered from what the person said, so nothing that only echoes their last line can take the\n    turn: `You said: \"its on the outside of the knee\"` says their own words again and answers\n    nothing. The claim is made for every line the hurting-part reader can answer, and for no\n    other.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return { let __b = call_builtin("bool", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[crate::gen::memory::whathurts::the_part_asked(&[memory.clone(), low.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _the_store_readings(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut line: Py = Py::None;
    let _ = py_str("The lines this store read for certain: a read of what it keeps, the ask whose answer is a\n    line they said themselves, a `when` ask it can place, and a question about what the chat\n    already holds. All are answered from what the person said, so a topic the knowledge base\n    declares must not take the turn.\n\n    This is a reading, not the hook: the exported name is `TAKES_AS_IT_COMES` at the end of the\n    file, and it asks this one first. A module's name is bound once, at import, so a second `def`\n    of the exported name is not an addition - it is a whole new hook, and everything under the\n    first one stops answering without a single test going red.\n\n    \"when is that again\" after \"i've got a dentist appointment on tuesday\" was answered\n    \"Same as before: Ring practices near you and ask straight out whether they take new NHS\n    patients...\" - the knowledge answer that happened to be the turn before, said again behind a\n    repeat lead-in. The date is in what the person said, so this is the memory store's line and\n    it says so in front of it. Every ask it cannot place is left alone.\n\n    \"have we talked about food\" is the same kind of line: the answers lane proposed from the word\n    `food` and won before the store was asked, so the person asking the bot to remember got a\n    recipe list instead.\n    ");
    if py_truthy(&{ let __b = _reads_the_store(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _their_own_line_ask(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _the_part_that_hurts_asked(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _their_fact_ask(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = _a_kept_fact_read(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::memory::decided::asks(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = crate::gen::memory::told::ABOUT_THE_CHAT(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { crate::gen::memory::told::A_CALLBACK(&[text.clone(), memory.clone()]) } } } } } } } } } } } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::memory::store_replies::g_WHOLE_ASK(), "match", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]); if !py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_a_whole_answer_to_hand_back(&[memory.clone(), text.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::store_replies::_a_state_that_was(&[text.clone(), memory.clone(), py_str("")])) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_feeling_about_their_own_news(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_plan_they_took_back(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_take_back_about_someone(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::store_replies::_can_close(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::store_replies::_the_call_from_the_advice(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_asks_whether_we_followed(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::rates::is_a_rate_ask(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::times::is_a_time_ask(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_an_ask_about_someone_else(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_group_size_ask(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = crate::gen::memory::repeat::how_long_its_been(&[text.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_figure_the_answer_names(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::store_replies::_the_budget_on_the_answer(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::store_replies::_the_back_this_turn(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::store_replies::_the_same_fact_about_someone(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    let __try1 = exc::catch(|| {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    line = crate::gen::memory::store_replies::_bare(&[py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = { let __b = mcall(&crate::gen::memory::store_replies::g_POINTS_BACK_AT_IT(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORK_ON_IT(), "search", &[line.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::store_replies::_the_prep_day(&[memory.clone(), line.clone()]))) } }) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(false)]);
    }
    if py_truthy(&_the_clock_in_their_line(&[text.clone(), memory.clone()])) {
    return py_tuple(vec![Py::Bool(true), Py::Bool(true)]);
    }
    if py_truthy(&Py::Bool(!py_contains(&mcall(&text.clone(), "lower", &[]), &py_str("when")))) {
    return py_tuple(vec![Py::Bool(true), _a_count_from_the_date(&[text.clone(), memory.clone()])]);
    }
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&crate::gen::memory::store_replies::g_when_answer(), &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _their_fact_ask(args: &[Py]) -> Py {
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
    else { Py::None };
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut undeclared: Py = Py::None;
    let _ = py_str("True for a line that only asks back a fact kept about someone else.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = crate::gen::memory::store_replies::_bare(&[py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])]);
    if py_truthy(&{ let __b = mcall(&g_PERSON_ADVICE_ASK(), "match", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::store_replies::g_CAN_THEY_HAVE(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_a_limit_kept_for_them(&[low.clone(), memory.clone()]) } })) } }) {
    return Py::Bool(false);
    }
    undeclared = crate::gen::memory::store_replies::_the_person_still_in_hand(&[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = undeclared.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_OTHER_FACT(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_EAT_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_CANT_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_CAN_HAVE(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::memory::store_replies::g_CAN_THEY_HAVE(), "search", &[low.clone()]) } } } } } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = undeclared.clone(); if py_truthy(&__b) { __b } else { { let __b = py_call(&crate::gen::memory::store_replies::g__asks(), &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&text.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { _asks_about_someone_else(&[text.clone()]) } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = undeclared.clone(); if py_truthy(&__b) { __b } else { { let __b = mcall(&crate::gen::memory::store_replies::g_THEM(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { { let __b = py_call(&crate::gen::memory::store_replies::g__named_person(), &[low.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[add(&add(&py_str("\\bmy\\s+(?:"), &crate::gen::memory::store_replies::g__RELATION()), &py_str(")\\b(?!'s|s\\b)(?!s\\b)")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } } } } } }))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&crate::gen::memory::store_replies::g_CAN_THEY_HAVE_A_FOOD(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::store_replies::_a_limit_kept_for_them(&[low.clone(), memory.clone()]))) } }) {
    return Py::Bool(false);
    }
    said = crate::gen::memory::store_replies::_other_person_reply(&[memory.clone(), low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&crate::gen::memory::store_replies::g_NO_RECORD(), "search", &[said.clone()])) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _name_the_last_pet(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut before: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut last: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut old: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("\"we've also got a cat\" then \"and her name is Biscuit\" is one cat with a name.\n\n    A name given a second time is a correction when the line opens like one - \"sorry, his name\n    is milo\" after \"we have a dog called max\". The store kept the first name and said it back\n    (\"You've got a dog called Max\") while the person had already taken it back, and the same line\n    with the old name on the end (\"sorry, his name is milo not max\") was read as not having a dog\n    at all. Only the opener makes it a rename: \"we have a dog called max / his name is milo\" on\n    its own is the same dog said twice as easily as a correction.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    pets = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]);
    named = if py_truthy(&{ let __b = call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = pets.clone(); if !py_truthy(&__b) { __b } else { call_builtin("isinstance", &[geti(&pets.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)) } } } }) { geti(&pets.clone(), &neg(&Py::Int(1))) } else { py_dict(vec![]) };
    name = { let __b = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __kws.extend(vec![("lower".to_string(), call_builtin("bool", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::pet_name_only(args))(&__args) }; if py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_name_they_answered(&[text.clone(), memory.clone()]) } };
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pets.clone())) } }) {
    animal = crate::gen::memory::store_replies::_the_animal_of_the_pick(&[memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&animal.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&animal.clone(), &py_str("pet"))) } }) {
    return Py::None;
    }
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("pets"), py_list(vec![py_dict(vec![(py_str("animal"), animal.clone()), (py_str("name"), name.clone())])]), text.clone()]);
    return Py::None;
    }
    last = geti(&pets.clone(), &neg(&Py::Int(1)));
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&last.clone(), "get", &[py_str("name")]))) } }) {
    let _ = py_setitem(&last.clone(), &py_str("name"), name.clone());
    kept = py_dict(vec![(py_str("animal"), mcall(&last.clone(), "get", &[py_str("animal"), py_str("pet")])), (py_str("name"), name.clone())]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&last.clone(), "get", &[py_str("count")]), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { gt(&geti(&last.clone(), &py_str("count")), &Py::Int(1)) } }) {
    let _ = py_setitem(&kept.clone(), &py_str("count"), geti(&last.clone(), &py_str("count")));
    }
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("pets"), py_list(vec![kept.clone()]), text.clone()]);
    return Py::None;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&name.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&g_CORRECTS_A_NAME(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CORRECTS_A_NAME_POSSESSIVE(), "match", &[text.clone()]) } }; if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_a_pet_here(&[text.clone(), memory.clone()]))) } } } } } }) {
    before = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&last.clone(), &py_str("name"), name.clone());
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), py_dict(vec![(py_str("animal"), mcall(&last.clone(), "get", &[py_str("animal")])), (py_str("name"), name.clone())])), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    if py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&name.clone(), "lower", &[]))) } }) {
    was = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&last.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut old: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&name.clone(), "lower", &[])))) {  __out.push(call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    let _ = py_setitem(&last.clone(), &py_str("was"), add(&was.clone(), &py_list(vec![before.clone()])));
    }
    if py_truthy(&before.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), name.clone()), (py_str("animal"), { let __b = mcall(&last.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    }
    }
    Py::None
}
pub fn _the_animals_named(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut here: Py = Py::None;
    let mut low: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("The animals of the store's pets the line names: \"no the cat is called tom\" names the cat.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    here = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("animal")]))) } }) {
    continue;
    }
    animal = mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("animal"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[animal.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&here.clone(), "add", &[animal.clone()]);
    }
    }
    return here.clone();
    Py::None
}
pub fn _the_name_it_had(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut animal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut memory: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut entry: Py = Py::None;
    let mut name: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The name the store had for this animal before the line that named it again.");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ let __b = crate::gen::memory::log(&[memory.clone()]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    value = if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&entry.clone(), "get", &[py_str("value")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &animal.clone()))) {
    continue;
    }
    name = call_builtin("str", &[{ let __b = mcall(&value.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&name.clone(), "lower", &[]), &mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return name.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _names_a_pet_here(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("Whether the line names an animal the store keeps a pet of (\"the cat\", \"the dogs\").");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter1 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&pet.clone(), "get", &[py_str("animal")]))) } }) {
    continue;
    }
    if py_truthy(&stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("animal"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_figure_said(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("The age a reader's own regex caught, as a number, whether it was digits or a word.");
    said = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("age")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::None;
    }
    value = crate::gen::memory::store_replies::_a_number(&[mcall(&crate::gen::memory::facts::g_FIGURE_WORDS(), "get", &[said.clone(), said.clone()])]);
    return if py_truthy(&call_builtin("isinstance", &[value.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { value.clone() } else { Py::None };
    Py::None
}
pub fn _without_a_lead_in(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let mut shorter: Py = Py::None;
    let _ = py_str("The line with the lead-ins people open a short sentence with taken off.");
    said = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[Py::Int(4)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    _u = __item.clone();
    shorter = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("")); __args.push(said.clone()); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_PET_LEAD_IN(), "sub", args))(&__args) };
    if py_truthy(&Py::Bool(py_eq(&shorter.clone(), &said.clone()))) {
    break;
    }
    said = mcall(&shorter.clone(), "strip", &[]);
    }
    return said.clone();
    Py::None
}
pub fn _put_a_pets_age(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut age: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "age") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: age"))) };
    let mut unit: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "unit") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: unit"))) };
    let mut text: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let _ = py_str("Store an age on one animal, or None when it already says exactly that.");
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &age.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &unit.clone())) } }) {
    return Py::None;
    }
    let _ = py_setitem(&pet.clone(), &py_str("age"), age.clone());
    let _ = mcall(&pet.clone(), "pop", &[py_str("unit"), Py::None]);
    if py_truthy(&unit.clone()) {
    let _ = py_setitem(&pet.clone(), &py_str("unit"), unit.clone());
    }
    return py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), call_builtin("dict", &[pet.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())]);
    Py::None
}
pub fn _age_the_named_pet(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("\"my dog is called fred\" then \"fred is 5\" is an age for the dog, not a fact about nobody.\n\n    The line names no animal at all, only the name the store has for one, so the store is what\n    says whose age it is. Without this the 5 went nowhere and \"how old is fred\" had no answer.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    low = mcall(&_without_a_lead_in(&[text.clone()]), "lower", &[]);
    match_ = mcall(&g_PET_NAME_AGE(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = age.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(30)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    unit = if py_truthy(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "startswith", &[py_str("month")])) { py_str("months") } else { py_str("") };
    said = if py_truthy(&mcall(&low.clone(), "strip", &[])) { mcall(&geti(&mcall(&mcall(&low.clone(), "strip", &[]), "split", &[]), &Py::Int(0)), "strip", &[py_str(".,!?")]) } else { py_str("") };
    pets = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[pets.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &said.clone())) } }) {
    continue;
    }
    return _put_a_pets_age(&[pet.clone(), age.clone(), unit.clone(), text.clone()]);
    }
    return Py::None;
    Py::None
}
pub fn _fix_a_pets_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut aged: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("\"my dog is 5\" then \"sorry he is 6\": the correction goes on the animal it corrects.\n\n    The line names no animal - it names a pronoun and a figure - and the store already has one\n    age for the animal the chat is on, so that is the age the figure replaces. Read the same way\n    a person's corrected age is read (`_fix_a_persons_age`): only a short line, only with a\n    correction word in it, and only when there is an age stored to replace, because a\n    correction replaces a fact rather than being the first one said.\n\n    Without this the store kept the old figure and the bot said it as fact: \"my dog is 5\" /\n    \"sorry he is 6\" / \"how old is my dog\" answered \"Your dog is 5.\" after the person had said 6.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    low = _without_a_lead_in(&[said.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::memory::store_lines::g_PET_AGE_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&crate::gen::memory::relations::g__PET_WORD(), "search", &[low.clone()])) {
    return Py::None;
    }
    match_ = mcall(&g_AGE_FIX(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = age.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(30)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    who = py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&{ let __b = who.clone(); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut known: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("age"))) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::None;
    }
    unit = if py_truthy(&stdlib_call("re", "search", &[py_str("\\bmonths?\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) { py_str("months") } else { py_str("") };
    aged = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&mcall(&pet.clone(), "get", &[py_str("age")]), &Py::None)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[aged.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    return _put_a_pets_age(&[geti(&aged.clone(), &Py::Int(0)), age.clone(), unit.clone(), text.clone()]);
    Py::None
}
pub fn _age_said_of_the_only_animal(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut called: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut one: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut said: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("\"i have a greyhound called sid\" then \"hes 6\": the figure is that animal's age.\n\n    `_age_they_answered` reads a bare figure only when the bot asked for one, so a figure the\n    person volunteers (\"hes 6\", \"shes 12 now\") went nowhere and \"how old is sid\" answered that\n    they had never said. It is only read when the line before it named an animal or the one the\n    store knows by name, and when the chat holds one animal - with two, \"hes 6\" says nothing\n    about which one.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::None;
    }
    previous = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = py_call(&crate::gen::memory::store_replies::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    pet = geti(&pets.clone(), &Py::Int(0));
    } else {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&mcall(&call_builtin("str", &[geti(&one.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), previous.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    pet = geti(&named.clone(), &Py::Int(0));
    }
    if py_truthy(&gt(&call_builtin("int", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("count")]); if py_truthy(&__b) { __b } else { Py::Int(1) } }]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::None;
    }
    name = mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]);
    called = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("animal"), py_str("breed")])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[word.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::relations::g__PET_WORD(), "search", &[previous.clone()]); if py_truthy(&__b) { __b } else { { let __b = { let __b = gt(&call_builtin("len", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&previous.clone(), &name.clone())) } }; if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&called.clone()) { let mut word: Py = __item.clone(); __out.push({ let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&previous.clone(), &word.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|she|it|they)(?:'s|s|\\s+is|\\s+are)?\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    match_ = mcall(&g_PET_AGE_ANSWER(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = age.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(30)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    if py_truthy(&py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[said.clone(), memory.clone()])) {
    return Py::None;
    }
    unit = mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    unit = if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) { py_str("months") } else { if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) { py_str("weeks") } else { py_str("") } };
    return _put_a_pets_age(&[pet.clone(), age.clone(), unit.clone(), text.clone()]);
    Py::None
}
pub fn _age_they_answered(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut names: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut unit: Py = Py::None;
    let _ = py_str("The bot asked how old their animal was, so the figure in this line is that animal's age:\n\n        i have a dog / You haven't told me how old your dog is. / hes 3\n\n    A bare age names nobody, so it is read only when the reply before it asked, the same way a\n    bare name is only read when the bot asked for one.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    asked = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_contains(&mcall(&asked.clone(), "lower", &[]), &py_str("how old")))) {
    return Py::None;
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&pets.clone()))) {
    return Py::None;
    }
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::relations::g__PET_WORD(), "search", &[asked.clone()]); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&names.clone()) { let mut name: Py = __item.clone(); __out.push({ let __b = name.clone(); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[name.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), mcall(&asked.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e)) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    match_ = mcall(&g_PET_AGE_ANSWER(), "match", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&lt(&Py::Int(0), &{ let __cmp10 = age.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(30)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    unit = mcall(&call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    unit = if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("month")])) { py_str("months") } else { if py_truthy(&mcall(&unit.clone(), "startswith", &[py_str("week")])) { py_str("weeks") } else { py_str("") } };
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&pet.clone(), "get", &[py_str("age")]), &age.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("unit")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &unit.clone())) } }) {
    return Py::None;
    }
    let _ = py_setitem(&pet.clone(), &py_str("age"), age.clone());
    let _ = mcall(&pet.clone(), "pop", &[py_str("unit"), Py::None]);
    if py_truthy(&unit.clone()) {
    let _ = py_setitem(&pet.clone(), &py_str("unit"), unit.clone());
    }
    let _ = py_setitem(&memory.clone(), &py_str("pet_age"), call_builtin("dict", &[pet.clone()]).unwrap_or_else(|e| exc::raise(e)));
    return py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), call_builtin("dict", &[pet.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _their_name_answered(args: &[Py]) -> Py {
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
    let mut answered: Py = Py::None;
    let mut asked: Py = Py::None;
    let mut last: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own name, given as the answer to the bot's own question:\n\n        can you remember my name / I don't know it yet - you haven't said / sam\n        can you remember my name / I don't know it yet - you haven't said / its sam\n\n    A word on its own says nothing, so it is read as a name only when the reply right before it\n    asked for one, and only when no name is on record yet (\"sam\" to a bot that already has the\n    name is something else). \"its\" in front of the word is only an introduction there.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("name")])) {
    return Py::None;
    }
    asked = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&memory.clone(), "get", &[py_str("asked_for")]), &py_str("name"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&crate::gen::memory::facts::g_USER_NAME_ASKED(), "search", &[asked.clone()]))) } }) {
    return Py::None;
    }
    answered = crate::gen::memory::facts::without_the_hedges(&[text.clone()]);
    match_ = mcall(&crate::gen::memory::facts::g_NAME_ANSWER_ANNOUNCED(), "match", &[answered.clone()]);
    if py_truthy(&match_.clone()) {
    return crate::gen::memory::facts::reads_as_a_name(&[mcall(&match_.clone(), "group", &[Py::Int(1)])]);
    }
    last = mcall(&crate::gen::memory::facts::g_NAME_ANSWER_ANNOUNCED_LAST(), "search", &[answered.clone()]);
    if py_truthy(&last.clone()) {
    return crate::gen::memory::facts::reads_as_a_name(&[mcall(&last.clone(), "group", &[Py::Int(1)])]);
    }
    return crate::gen::memory::facts::reads_as_a_name(&[answered.clone()]);
    Py::None
}
pub fn _fix_a_persons_age(args: &[Py]) -> Py {
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
    let mut age: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Put a corrected age on the person the pronoun points at.\n\n    \"my son is 7\" then \"actually hes 8\" leaves the store saying 7 while the bot has already said\n    \"Got it - he's 8.\", so the person thinks the correction landed. The line names no relation,\n    so who it is about comes from the pronoun, and the age it replaces has to be one the store\n    already has.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::memory::store_lines::g_AGE_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[low.clone()]))) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_AGE_FIX(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    age = _the_figure_said(&[match_.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(0), &{ let __cmp10 = age.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(120)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    who = py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter2 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("age"))) } }) {
    let _ = py_setitem(&known.clone(), &py_str("age"), age.clone());
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _fix_the_weeks_of_a_pregnancy(args: &[Py]) -> Py {
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
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut own: Py = Py::None;
    let mut said_mine: Py = Py::None;
    let mut said_theirs: Py = Py::None;
    let mut weeks: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Move the length the store holds to the one this line gives, for the person it points at.");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)) } }) {
    return Py::None;
    }
    match_ = mcall(&g_WEEKS_FIX(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    if py_truthy(&mcall(&g_PREGNANCY_SAID_AGAIN(), "search", &[low.clone()])) {
    return Py::None;
    }
    weeks = call_builtin("int", &[mcall(&match_.clone(), "group", &[py_str("weeks")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&if py_truthy(&le(&Py::Int(1), &{ let __cmp10 = weeks.clone(); __cmp10.clone() })) { le(&__cmp10.clone(), &Py::Int(45)) } else { Py::Bool(false) }))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_CORRECTING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\b(?:now|today)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }))) {
    return Py::None;
    }
    said_mine = stdlib_call("re", "search", &[py_str("\\b(?:im|i'm|i am|i)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    said_theirs = stdlib_call("re", "search", &[py_str("\\b(?:my|our|she|he|they|her|his|their|hers)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    own = { let __b = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    if py_truthy(&{ let __b = said_mine.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said_theirs.clone())); if !py_truthy(&__b) { __b } else { mcall(&own.clone(), "get", &[py_str("pregnant")]) } } } }) {
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("weeks"), weeks.clone(), text.clone()]);
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(own.clone()); __kws.extend(vec![("weeks".to_string(), weeks.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    }
    who = py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter2 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter2 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { { let __b = mcall(&known.clone(), "get", &[py_str("pregnant")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&known.clone(), &py_str("weeks"))) } } } }) {
    let _ = py_setitem(&known.clone(), &py_str("weeks"), weeks.clone());
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _fix_a_pets_name(args: &[Py]) -> Py {
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
    let mut animal: Py = Py::None;
    let mut before: Py = Py::None;
    let mut filed: Py = Py::None;
    let mut found: Py = Py::None;
    let mut here: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut kept_note: Py = Py::None;
    let mut key: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut moves: Py = Py::None;
    let mut named: Py = Py::None;
    let mut older: Py = Py::None;
    let mut other: Py = Py::None;
    let mut pet: Py = Py::None;
    let mut pets: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut said: Py = Py::None;
    let mut target: Py = Py::None;
    let mut was: Py = Py::None;
    let _ = py_str("A pet's name put right in the sentence moves the name on the pet.\n\n    \"my cat is called milo\" then \"no hes called max\": the person correction path needs a pointer\n    to a relation and a name that is a spelling of the old one, and a pet is neither, so the store\n    kept Milo and the next ask answered Milo. The pet is the animal the line names, or else the\n    pet the turn before was about.\n    ");
    let mut pet_rename: Py = crate::gen::memory::rename::module_obj();
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    moves = mcall(&pet_rename.clone(), "moves", &[low.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::memory::store_lines::g_NAME_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_CORRECTING(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { moves.clone() } })) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_A_NAME_WITH_ITS_VERB(), "search", &[low.clone()]);
    raw = py_str("");
    if py_truthy(&match_.clone()) {
    raw = { let __b = mcall(&match_.clone(), "group", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } };
    } else {
    if py_truthy(&moves.clone()) {
    raw = mcall(&pet_rename.clone(), "the_name_a_line_moved_on", &[low.clone(), mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]), call_builtin("bool", &[mcall(&crate::gen::memory::facts::g_PETS(), "search", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&raw.clone()))) {
    return Py::None;
    }
    said = crate::gen::memory::facts::_finish(&[raw.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(raw.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::_not_a_name(args))(&__args) } } }) {
    return Py::None;
    }
    pets = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut pet: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&mcall(&pet.clone(), "get", &[py_str("name")])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    target = Py::None;
    if py_truthy(&_names_a_pet_here(&[text.clone(), memory.clone()])) {
    if py_truthy(&_another_animal_is_named(&[text.clone()])) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("sorted", &[_the_animals_named(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    animal = __item.clone();
    kept = _pets_of(&[animal.clone(), pets.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&kept.clone()))) {
    continue;
    }
    filed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&_a_pet_new_this_line(&[pet.clone(), memory.clone()])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    older = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&kept.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_a_pet_new_this_line(&[pet.clone(), memory.clone()])))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&older.clone()) {
    target = geti(&older.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) {
    let _ = py_setitem(&target.clone(), &py_str("name"), add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)));
    let _ = mcall(&target.clone(), "pop", &[py_str("turns"), Py::None]);
    was = _the_name_it_had(&[animal.clone(), said.clone(), memory.clone()]);
    if py_truthy(&{ let __b = was.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("correction")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&memory.clone(), &py_str("correction")), "get", &[py_str("said")]), &low.clone())) } })) } }) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&target.clone(), &py_str("name"))), (py_str("wrong"), add(&mcall(&slice_or_raise(&was.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&was.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("animal"), animal.clone()), (py_str("said"), low.clone()), (py_str("text"), text.clone())]));
    }
    }
    let __iter2 = py_iter_or_raise(&filed.clone());
    for __item in __iter2 {
    pet = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&pet.clone(), &target.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }, &pet.clone())) } }) {
    let _ = mcall(&geti(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), &py_str("pets")), "remove", &[pet.clone()]);
    }
    }
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&filed.clone());
    for __item in __iter3 {
    pet = __item.clone();
    let _ = py_setitem(&pet.clone(), &py_str("name"), add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)));
    was = _the_name_it_had(&[animal.clone(), said.clone(), memory.clone()]);
    if py_truthy(&{ let __b = was.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[mcall(&memory.clone(), "get", &[py_str("correction")]), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&geti(&memory.clone(), &py_str("correction")), "get", &[py_str("said")]), &low.clone())) } })) } }) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&pet.clone(), &py_str("name"))), (py_str("wrong"), add(&mcall(&slice_or_raise(&was.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&was.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("animal"), animal.clone()), (py_str("said"), low.clone()), (py_str("text"), text.clone())]));
    }
    }
    return Py::None;
    }
    }
    if py_truthy(&_another_animal_is_named(&[text.clone()])) {
    return Py::None;
    }
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if !py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[geti(&pet.clone(), &py_str("animal"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), low.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    here = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&here.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&Py::Bool(!py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[here.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    target = geti(&here.clone(), &Py::Int(0));
    } else {
    let __iter4 = py_iter_or_raise(&py_tuple(vec![py_str("name"), py_str("animal")]));
    for __item in __iter4 {
    key = __item.clone();
    found = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&_the_turn_before_said(&[mcall(&pet.clone(), "get", &[key.clone()]), memory.clone()])) {  __out.push(pet.clone()); } } py_list(__out) })() };
    if py_truthy(&found.clone()) {
    target = geti(&found.clone(), &neg(&Py::Int(1)));
    break;
    }
    }
    }
    if py_truthy(&Py::Bool(py_is(&target.clone(), &Py::None))) {
    return Py::None;
    }
    before = call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&said.clone(), "lower", &[])))) {
    return Py::None;
    }
    let _ = py_setitem(&target.clone(), &py_str("name"), add(&mcall(&slice_or_raise(&said.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&said.clone(), &Py::Int(1), &Py::None, &Py::None)));
    let _ = mcall(&target.clone(), "pop", &[py_str("turns"), Py::None]);
    kept_note = mcall(&memory.clone(), "get", &[py_str("correction")]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[kept_note.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&kept_note.clone(), "get", &[py_str("said")]), &low.clone())) } }))) {
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), geti(&target.clone(), &py_str("name"))), (py_str("wrong"), add(&mcall(&slice_or_raise(&before.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&before.clone(), &Py::Int(1), &Py::None, &Py::None))), (py_str("animal"), call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("pet") } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("said"), low.clone()), (py_str("text"), text.clone())]));
    }
    animal = mcall(&call_builtin("str", &[{ let __b = mcall(&target.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    let __iter5 = py_iter_or_raise(&call_builtin("list", &[pets.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    other = __item.clone();
    if py_truthy(&Py::Bool(py_is(&other.clone(), &target.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&other.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &animal.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&other.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &mcall(&said.clone(), "lower", &[]))) } }) {
    let _ = mcall(&pets.clone(), "remove", &[other.clone()]);
    }
    }
    let _ = py_setitem(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), &py_str("pets"), pets.clone());
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("pet")), (py_str("value"), call_builtin("dict", &[target.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return target.clone();
    Py::None
}
pub fn _pets_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut animal: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut pets: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut pet: Py = Py::None;
    let _ = py_str("The pets of one animal in the store's list, in the order the store keeps them.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &animal.clone())) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _a_pet_new_this_line(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pet: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pet") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pet"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut held: Py = Py::None;
    let _ = py_str("Whether this pet entry was filed by the line being read.\n\n    The store before the line was read is carried in the memory for the length of the turn\n    (`_pets_before`), so an entry that was not there before is the line's own: it is the name\n    the line said, filed beside the pets it was filed with, and not a pet of its own.\n    ");
    held = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("_pets_before")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_contains(&held.clone(), &py_tuple(vec![mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])])));
    Py::None
}
pub fn _another_animal_is_named(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line says the name belongs to another animal, not the one on record.");
    return call_builtin("bool", &[mcall(&g_ANOTHER_ANIMAL(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _two_of_the_animal_are_kept(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut pets: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "pets") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: pets"))) };
    let mut animal: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "animal") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: animal"))) };
    let mut kept: Py = Py::None;
    let mut pet: Py = Py::None;
    let _ = py_str("Whether the store holds more than one pet of this animal.\n\n    When it does, a name filed against that animal was filed beside the pet already there. A\n    rename leaves one entry (the rename drops the echo the same line filed), so two entries of\n    the animal said with the new name is the store keeping both, not one name put right.\n    ");
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pets.clone()) { let mut pet: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[pet.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&call_builtin("str", &[{ let __b = mcall(&pet.clone(), "get", &[py_str("animal")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &animal.clone())) } }) {  __out.push(pet.clone()); } } py_list(__out) })() };
    return gt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _the_turn_before_said(args: &[Py]) -> Py {
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
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("Whether the line before this one said this word about the pet in question.");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } } } }) {
    return Py::Bool(false);
    }
    last = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))), &py_str("s?\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _fix_the_make_of_a_thing(args: &[Py]) -> Py {
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
    let mut entry: Py = Py::None;
    let mut fits: Py = Py::None;
    let mut held: Py = Py::None;
    let mut last_said: Py = Py::None;
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("A correction that names only the make goes on the last thing the store was told about.\n\n    \"my car is a ford fiesta\" and then \"sorry, its a focus\" is one car with its make put right,\n    and a store that keeps the fiesta answers the next ask with a car they have just corrected.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&line.clone(), "endswith", &[py_str("?")]); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_CORRECTS_A_MAKE(), "search", &[mcall(&line.clone(), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    held = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("owns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&held.clone())) } }) {
    return Py::None;
    }
    fits = py_dict(vec![]);
    let __iter1 = py_iter_or_raise(&held.clone());
    for __item in __iter1 {
    thing = __item.clone();
    name = crate::gen::memory::facts::_the_model(&[mcall(&match_.clone(), "group", &[py_str("value")]), thing.clone()]);
    if py_truthy(&name.clone()) {
    let _ = py_setitem(&fits.clone(), &thing.clone(), name.clone());
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&fits.clone()))) {
    return Py::None;
    }
    last_said = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&crate::gen::memory::log(&[memory.clone()]));
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &py_str("owns")))) {
    let _ = mcall(&last_said.clone(), "extend", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("value")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }]);
    }
    }
    thing = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("reversed", &[last_said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut one: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&fits.clone(), &one.clone()))) {  __out.push(one.clone()); } } py_list(__out) })() }, call_builtin("next", &[call_builtin("iter", &[fits.clone()]).unwrap_or_else(|e| exc::raise(e)), py_str("")]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    name = mcall(&fits.clone(), "get", &[thing.clone(), py_str("")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&name.clone(), &mcall(&held.clone(), "get", &[thing.clone()]))) } }) {
    return Py::None;
    }
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("owns"), py_dict(vec![(thing.clone(), name.clone())]), text.clone()]);
    Py::None
}
pub fn _fix_a_persons_name(args: &[Py]) -> Py {
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
    let mut about: Py = Py::None;
    let mut bare: Py = Py::None;
    let mut before: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Put a corrected name on the person the pronoun points at.\n\n    \"my sister is called meg\" then \"actually she's called maggie\" left the store saying Meg, so\n    the ask after it answered with the name the person had just put right. The line names no\n    relation, so who it is about comes from the pronoun, and the name it replaces has to be one\n    the store already has: a correction replaces a fact, it is not the first one said.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::memory::store_lines::g_NAME_FIX_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CORRECTING(), "search", &[low.clone()]))) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_NAME_FIX(), "search", &[low.clone()]);
    bare = Py::None;
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    bare = mcall(&g_A_NAME_AFTER_A_PRONOUN(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&bare.clone()))) {
    return Py::None;
    }
    word = mcall(&mcall(&bare.clone(), "group", &[py_str("name")]), "strip", &[py_str("'’")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_COMMON_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_JOB_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_KIN(), &word.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(word.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::_not_a_name(args))(&__args) } } } } } } }) {
    return Py::None;
    }
    match_ = bare.clone();
    }
    said = mcall(&mcall(&match_.clone(), "group", &[py_str("name")]), "strip", &[py_str("'’")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NAME_NOT_A_NAME(), &said.clone())); if py_truthy(&__b) { __b } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("strict_ly".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::facts::_not_a_name(args))(&__args) } } } } }) {
    return Py::None;
    }
    name = crate::gen::memory::facts::_tidy(&[{ let __b = crate::gen::memory::facts::_finish(&[said.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::None;
    }
    about = stdlib_call("re", "sub", &[py_str("\\bhis\\b(\\s+name)"), py_str("he\\1"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    about = stdlib_call("re", "sub", &[py_str("\\bher\\b(\\s+name)"), py_str("she\\1"), about.clone()]).unwrap_or_else(|e| exc::raise(e));
    who = py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[about.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&who.clone())); if !py_truthy(&__b) { __b } else { bare.clone() } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut entry: Py = __item.clone(); if py_truthy(&mcall(&entry.clone(), "get", &[py_str("name")])) {  __out.push(entry.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[named.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    who = call_builtin("str", &[{ let __b = mcall(&geti(&named.clone(), &Py::Int(0)), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    }
    let __iter1 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { mcall(&known.clone(), "get", &[py_str("name")]) } }) {
    before = call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = bare.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&before.clone(), "lower", &[]), &mcall(&crate::gen::memory::facts::_tidy(&[name.clone()]), "lower", &[]))) } }) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &py_str("name"), crate::gen::memory::facts::_tidy(&[name.clone()]));
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return py_dict(vec![(py_str("who"), who.clone()), (py_str("name"), call_builtin("str", &[geti(&known.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e))), (py_str("from"), before.clone())]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _the_name_they_gave_to_the_person_they_mean(args: &[Py]) -> Py {
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
    let mut fits: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut known: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut name: Py = Py::None;
    let mut one: Py = Py::None;
    let mut raw: Py = Py::None;
    let mut unnamed: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Put a name given with a pronoun on the person the store already holds.\n\n    \"my son is called jack\" then \"hes called jake\" left the store saying Jack while the reply said\n    \"He's called jake, good to know\": the person is told it landed and it did not, so the next\n    \"what is my son called\" answers with the name they have just given. Who it is about comes from\n    the pronoun.\n\n    A name the store already holds is replaced. A person it holds and never named gets the name as\n    their first one - \"im taking my daughter to london\" then \"her name is maggie\" - because the\n    reply already promises to remember it, and only when exactly one unnamed person fits the\n    pronoun.\n\n    A bare name after the word that opens a correction (\"no, jake\") is not this reader's: that is\n    a correction of a name said wrong, and `_correct_a_persons_name` owns it.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::memory::store_lines::g_NAME_GIVEN_WORDS()); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&low.clone(), &py_str("?"))) } } } }) {
    return Py::None;
    }
    match_ = mcall(&g_NAME_GIVEN(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    raw = call_builtin("str", &[{ let __b = mcall(&match_.clone(), "group", &[py_str("name")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("name2")]); if py_truthy(&__b) { __b } else { py_str("") } } } }]).unwrap_or_else(|e| exc::raise(e));
    name = mcall(&raw.clone(), "strip", &[py_str(" .,!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&crate::gen::memory::facts::_may_be_a_name(&[name.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_NAME_AFTER_CALLED(), &mcall(&name.clone(), "lower", &[]))) } } } }) {
    return Py::None;
    }
    if py_truthy(&mcall(&slice_or_raise(&low.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[py_str(" .,!;:")])) {
    return Py::None;
    }
    who = py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[low.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&known.clone(), "get", &[py_str("who")]), &who.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[py_str("name")]))) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&call_builtin("str", &[mcall(&known.clone(), "get", &[py_str("name")])]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), &name.clone()))) {
    return Py::None;
    }
    let _ = py_setitem(&known.clone(), &py_str("name"), crate::gen::memory::facts::_tidy(&[name.clone()]));
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    if py_truthy(&mcall(&crate::gen::memory::facts::g_PETS(), "search", &[low.clone()])) {
    return Py::None;
    }
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:she|her)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    fits = crate::gen::memory::facts::g_SHE();
    } else {
    if py_truthy(&stdlib_call("re", "search", &[py_str("\\b(?:he|his)\\b"), low.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    fits = crate::gen::memory::facts::g_HE();
    } else {
    fits = Py::None;
    }
    }
    unnamed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&one.clone(), "get", &[py_str("name")]))) } }) {  __out.push(call_builtin("str", &[mcall(&one.clone(), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = mcall(&one.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_set(__out) })() };
    let __aug2 = add(&unnamed.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("people")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[word.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&kept.clone(), &mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_list(__out) })() });
    unnamed = __aug2;
    unnamed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e))), "fromkeys", &[unnamed.clone()])) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&fits.clone())); if py_truthy(&__b) { __b } else { crate::gen::memory::relations::_pronoun_fits(&[word.clone(), fits.clone()]) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[unnamed.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return Py::None;
    }
    let __iter3 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter3 {
    known = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&known.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &geti(&unnamed.clone(), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&known.clone(), "get", &[py_str("name")]))) } }) {
    let _ = py_setitem(&known.clone(), &py_str("name"), crate::gen::memory::facts::_tidy(&[name.clone()]));
    let _ = mcall(&known.clone(), "pop", &[py_str("turns"), Py::None]);
    let _ = mcall(&crate::gen::memory::log(&[memory.clone()]), "append", &[py_dict(vec![(py_str("key"), py_str("other")), (py_str("value"), call_builtin("dict", &[known.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("turn"), call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("text"), text.clone())])]);
    return known.clone();
    }
    }
    let _ = py_call(&crate::gen::memory::store_replies::g_remember_other(), &[memory.clone(), py_dict(vec![(py_str("who"), geti(&unnamed.clone(), &Py::Int(0))), (py_str("name"), crate::gen::memory::facts::_tidy(&[name.clone()]))]), text.clone()]);
    return py_dict(vec![(py_str("who"), geti(&unnamed.clone(), &Py::Int(0))), (py_str("name"), crate::gen::memory::facts::_tidy(&[name.clone()]))]);
    Py::None
}
pub fn _a_person(args: &[Py]) -> Py {
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
    let mut relation: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the corrected value names a person (\"sorry, i meant my wife\").\n\n    A person put right is answered better by the sentence that says whose the fact was, so the\n    short echo stays out of the way and the turn is left to it.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(mcall(&word.clone(), "rstrip", &[py_str("s")])); } py_set(__out) })() };
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::facts::g_RELATIONS()) { let mut relation: Py = __item.clone(); __out.push(Py::Bool(py_contains(&words.clone(), &mcall(&relation.clone(), "rstrip", &[py_str("s")])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_correction_without_a_marker(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut right: Py = Py::None;
    let mut said: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut wrong: Py = Py::None;
    let _ = py_str("The {to, from} of a pair correction said with no \"no\"/\"sorry\" in front, or None.\n\n    Only a value the store already holds can be taken back this way, so the line has to name an\n    older fact in the half it corrects: \"its Preeya not Priya\" after \"my name is Priya\" is a\n    rename, and \"its raining not sunny\" is a sentence about the weather that leaves the store\n    alone.\n    ");
    said = call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&g_PAIR_WITHOUT_A_MARKER());
    for __item in __iter1 {
    shape = __item.clone();
    found = mcall(&shape.clone(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    let __unpk2 = py_unpack(&py_tuple(vec![crate::gen::memory::facts::_tidy(&[mcall(&found.clone(), "group", &[py_str("to")])]), crate::gen::memory::facts::_tidy(&[mcall(&found.clone(), "group", &[py_str("wrong")])])]), 2, None);
    right = __unpk2[0].clone();
    wrong = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&right.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&wrong.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&right.clone(), "lower", &[]), &mcall(&wrong.clone(), "lower", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_NOT_A_NEW_VALUE(), &mcall(&right.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_NEW_VALUE(), &mcall(&wrong.clone(), "lower", &[]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&crate::gen::memory::corrected::_fact_naming(&[memory.clone(), wrong.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_person_called(&[memory.clone(), wrong.clone()]))) } }) {
    continue;
    }
    return py_dict(vec![(py_str("to"), right.clone()), (py_str("from"), wrong.clone())]);
    }
    return Py::None;
    Py::None
}
pub fn _a_person_called(args: &[Py]) -> Py {
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
    let mut name: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "name") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: name"))) };
    let mut entry: Py = Py::None;
    let mut low: Py = Py::None;
    let mut old: Py = Py::None;
    let _ = py_str("Whether the store holds a person under that name.\n\n    \"my sister is called anna\" then \"its hannah not anna\" names both values with no \"no\" in\n    front, and the name it takes back is one the store keeps for a person rather than in a fact\n    of the user's own - so the pair reader has to look in both places before it says the line\n    corrects nothing.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = name.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()]));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("name")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &low.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&entry.clone(), "get", &[py_str("was")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut old: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&mcall(&call_builtin("str", &[old.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), &low.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _stated_value(args: &[Py]) -> Py {
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
    let mut key: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "key") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: key"))) };
    let mut entries: Py = Py::None;
    let mut entry: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The value of `key` the line before this one stated, or None.\n\n    A correction replaces what was just said, so the old value is the one on that line: \"im\n    allergic to peanuts\" then \"make that tree nuts\" takes back peanuts, and not a food named\n    three turns earlier.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = py_call(&crate::gen::memory::store_replies::g_last_user(), &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    entries = mcall(&memory.clone(), "get", &[py_str("facts_log")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[entries.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[entries.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    entry = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), &said.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&entry.clone(), "get", &[py_str("key")]), &key.clone()))) {
    return mcall(&entry.clone(), "get", &[py_str("value")]);
    }
    }
    return Py::None;
    Py::None
}
pub fn _a_second_persons_limit(args: &[Py]) -> Py {
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
    let mut allergies: Py = Py::None;
    let mut diet: Py = Py::None;
    let mut foods: Py = Py::None;
    let mut giving: Py = Py::None;
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut one: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("The limit the line before gave, carried onto the person this line names.\n\n    Only the limit is carried, and only when the line before stated one: \"my son is too\" after a\n    line about the weather carries nothing, and the person is filed with the words the store\n    already uses for an allergy and for a diet.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    match_ = mcall(&g_TOO_AS_WELL(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    who = py_call(&crate::gen::memory::store_replies::g__canon_relation(), &[crate::gen::memory::facts::a_person_word(&[mcall(&match_.clone(), "group", &[py_str("who")])])]);
    if py_truthy(&Py::Bool(!py_contains(&crate::gen::memory::facts::g__A_WORD_FOR_A_PERSON(), &who.clone()))) {
    return Py::None;
    }
    theirs = crate::gen::memory::store_replies::_the_limit_the_last_line_gave(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&theirs.clone()))) {
    return Py::None;
    }
    giving = py_dict(vec![(py_str("who"), who.clone())]);
    allergies = mcall(&theirs.clone(), "get", &[py_str("allergies")]);
    if py_truthy(&call_builtin("isinstance", &[allergies.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    foods = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&allergies.clone()) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e))); } } py_list(__out) })() };
    if py_truthy(&foods.clone()) {
    let _ = py_setitem(&giving.clone(), &py_str("allergies"), foods.clone());
    }
    }
    diet = mcall(&theirs.clone(), "get", &[py_str("diet")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[diet.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&diet.clone(), "strip", &[]) } }) {
    let _ = py_setitem(&giving.clone(), &py_str("diet"), diet.clone());
    }
    if py_truthy(&lt(&call_builtin("len", &[giving.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    return giving.clone();
    Py::None
}
pub fn _drop_value(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut held: Py = Py::None;
    let mut key: Py = Py::None;
    let mut known: Py = Py::None;
    let mut left: Py = Py::None;
    let mut low: Py = Py::None;
    let mut v: Py = Py::None;
    let _ = py_str("'Sorry, not peanuts' takes that one back: it comes off the list of things they named.\n\n    The list is the fact the line before it stated, and only a value on it can go, so a \"not\"\n    about anything else (\"no, not today\") leaves memory alone. Nothing is put in its place, so\n    the note of the turn says the value went instead of a new one arriving.\n    ");
    known = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]);
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&crate::gen::memory::store_lines::g_DROP_LISTS());
    for __item in __iter1 {
    key = __item.clone();
    held = mcall(&known.clone(), "get", &[key.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[held.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut v: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[v.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_this(&[v.clone(), low.clone()]))) } }) {  __out.push(v.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    let _ = py_setitem(&known.clone(), &key.clone(), left.clone());
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), key.clone()), (py_str("value"), call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("dropped"), Py::Bool(true)), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return geti(&memory.clone(), &py_str("correction"));
    }
    return _drop_from_plan(&[memory.clone(), value.clone(), text.clone()]);
    Py::None
}
pub fn _plan_names(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Does the plan say this value? A plan is kept as the sentence it arrived in, so the thing\n    taken back is a word inside it (\"Lisbon\" in \"Lisbon in may\") and not an item on a list.");
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = plan.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &low.clone())); if py_truthy(&__b) { __b } else { { let __b = { let __b = mcall(&low.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&word.clone(), &slice_or_raise(&low.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }; if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&low.clone(), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } } } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _drop_from_plan(args: &[Py]) -> Py {
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
    let mut value: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "value") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: value"))) };
    let mut text: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { py_str("") };
    let mut city: Py = Py::None;
    let mut current: Py = Py::None;
    let mut held: Py = Py::None;
    let mut home: Py = Py::None;
    let mut known: Py = Py::None;
    let mut left: Py = Py::None;
    let mut left_places: Py = Py::None;
    let mut low: Py = Py::None;
    let mut place: Py = Py::None;
    let mut places: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut plans: Py = Py::None;
    let _ = py_str("'Sorry, not Lisbon' after 'im going to Lisbon in May' takes the trip back with the place.\n\n    A value taken back can be a word inside a plan rather than an item on a list, and a plan has\n    no key of its own: it is the sentence the person said. Only a plan that names the value goes,\n    so \"sorry, not tuesday\" leaves the trip to Lisbon alone, and the place goes with it - a store\n    that still answers \"you're going to lisbon in may\" is telling them the thing they just took\n    back. A city they also stated on its own (\"i live in lisbon\") stays: it is a fact about where\n    they live, not the plan they dropped.\n    ");
    known = py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]);
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::None;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::store_replies::g_MONTHS(), &low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::memory::facts::g_DAYS(), &low.clone())) } }) {
    return Py::None;
    }
    plans = mcall(&known.clone(), "get", &[py_str("plans")]);
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&if py_truthy(&call_builtin("isinstance", &[plans.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { plans.clone() } else { py_list(vec![]) }) { let mut plan: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[plan.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(plan.clone()); } } py_list(__out) })() };
    current = mcall(&known.clone(), "get", &[py_str("plan")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[current.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &current.clone())) } }) {
    let _ = mcall(&held.clone(), "append", &[current.clone()]);
    }
    left = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&held.clone()) { let mut plan: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_plan_names(&[plan.clone(), low.clone()])))) {  __out.push(plan.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[held.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&left.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("plans"), slice_or_raise(&left.clone(), &neg(&Py::Int(6)), &Py::None, &Py::None));
    let _ = py_setitem(&known.clone(), &py_str("plan"), geti(&left.clone(), &neg(&Py::Int(1))));
    } else {
    let _ = mcall(&known.clone(), "pop", &[py_str("plans"), Py::None]);
    let _ = mcall(&known.clone(), "pop", &[py_str("plan"), Py::None]);
    }
    city = mcall(&known.clone(), "get", &[py_str("city")]);
    home = { let __b = call_builtin("isinstance", &[city.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _plan_names(&[city.clone(), low.clone()]) } };
    places = mcall(&known.clone(), "get", &[py_str("places")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[places.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&home.clone())) } }) {
    left_places = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&places.clone()) { let mut place: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[place.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _plan_names(&[place.clone(), low.clone()]) } }))) {  __out.push(place.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("len", &[left_places.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[places.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    if py_truthy(&left_places.clone()) {
    let _ = py_setitem(&known.clone(), &py_str("places"), left_places.clone());
    } else {
    let _ = mcall(&known.clone(), "pop", &[py_str("places"), Py::None]);
    }
    }
    }
    let _ = py_setitem(&known.clone(), &py_str("plan_dropped"), low.clone());
    let _ = py_setitem(&memory.clone(), &py_str("correction"), py_dict(vec![(py_str("key"), py_str("plan")), (py_str("value"), call_builtin("str", &[value.clone()]).unwrap_or_else(|e| exc::raise(e))), (py_str("dropped"), Py::Bool(true)), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return geti(&memory.clone(), &py_str("correction"));
    Py::None
}
pub fn _names_this(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut held: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "held") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: held"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let _ = py_str("Does the value the person named name the thing on the list? Only the word itself and its\n    singular or plural: \"peanut\" is \"peanuts\", and \"nuts\" is not something else that went.");
    held = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[held.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_eq(&held.clone(), &low.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_tuple(vec![held.clone(), low.clone()]), py_tuple(vec![low.clone(), held.clone()])]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    a = __unpk2[0].clone();
    b = __unpk2[1].clone();
    if py_truthy(&{ let __b = mcall(&b.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&a.clone(), &slice_or_raise(&b.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _budget_sign(args: &[Py]) -> Py {
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
    let mut budget: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "budget") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: budget"))) };
    let mut match_: Py = Py::None;
    let mut number: Py = Py::None;
    let _ = py_str("The currency sign before the same amount in the line they typed, or \"\".");
    number = stdlib_call("re", "sub", &[py_str("[^\\d.,]"), py_str(""), call_builtin("str", &[{ let __b = budget.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&number.clone()))) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "finditer", &[py_str("([\\u00a3$\\u20ac])\\s?([\\d][\\d,.]*)"), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    match_ = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&stdlib_call("re", "sub", &[py_str("[^\\d.,]"), py_str(""), mcall(&match_.clone(), "group", &[Py::Int(2)])]).unwrap_or_else(|e| exc::raise(e)), &number.clone()))) {
    return mcall(&match_.clone(), "group", &[Py::Int(1)]);
    }
    }
    return py_str("");
    Py::None
}
pub fn _a_feeling_about_their_own_news(args: &[Py]) -> Py {
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
    let _ = py_str("This line is a feeling about the thing they just said, and the store answers it.\n\n    A bare \"im scared\" is read by the small-talk topics as small talk, so a candidate from there\n    would take the turn; this says the store read the line whole and names what it is about. It is\n    one of the lines the arbiter's single hook asks about, not a hook of its own: a second\n    `TAKES_AS_IT_COMES` in this module would shadow the first one and take the store's recall and\n    `when` asks out of service.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&crate::gen::memory::store_replies::_a_feeling_about_what_they_said(&[memory.clone(), crate::gen::memory::store_replies::_bare(&[py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])])]), &Py::None))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn takes_as_it_comes(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("True when the line is an ask for the bot's own last answer, so the store answers it.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    return crate::gen::memory::store_replies::_repeat_ask(&[text.clone(), py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]), memory.clone()]);
    Py::None
}
pub fn _the_ask_about_how_it_went(args: &[Py]) -> Py {
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
    let _ = py_str("This line asks how the thing they told us about went, and only they can say.\n\n    \"i had an interview today\" / \"how did it go\" is a question about the person's own day: the\n    knowledge base read the word \"interview\" and answered how to prepare for one, one line after\n    they said it was over, and the specialist rule handed it the turn. The ask names no subject\n    of its own on a line that does not, so the store says it read this line whole\n    (bot/memory/thread.went).\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), Py::Bool(!py_is(&crate::gen::memory::thread::went(&[text.clone(), memory.clone()]), &Py::None))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _a_count_from_the_date(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the ask is a count from a date already on file: \"how many weeks till that\".\n\n    The date is in a line they said, so the answer is arithmetic on it and this store's line; a\n    knowledge entry for the thing named in the ask (\"the driving test\") answers what that thing\n    is, which is a different question.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[py_call(&crate::gen::memory::store_replies::g_until_answer(), &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _the_unit_asked(args: &[Py]) -> Py {
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
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut name: Py = Py::None;
    let mut per_week: Py = Py::None;
    let mut units: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The unit families this line asks a figure in, or None when it names a thing of its own.");
    found = mcall(&g_FIGURE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    units = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&found.clone(), "group", &[py_str("rest")])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_FIGURE_FILLER(), &word.clone()))) {
    continue;
    }
    family = Py::None;
    let __iter2 = py_iter_or_raise(&mcall(&crate::gen::memory::store_lines::g_FIGURE_UNITS(), "items", &[]));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    name = __unpk3[0].clone();
    words = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_contains(&words.clone(), &word.clone()))) {
    family = name.clone();
    break;
    }
    }
    if py_truthy(&Py::Bool(py_is(&family.clone(), &Py::None))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_contains(&units.clone(), &family.clone()))) {
    let _ = py_call(&crate::gen::knowledge::units::g_append(), &[family.clone()]);
    }
    }
    per_week = stdlib_call("re", "match", &[modulo(&py_str("^\\W*(?:a|an|per|each)\\s+(?:%s)\\b"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&crate::gen::memory::store_lines::g_FIGURE_TIME()) { let mut family: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&crate::gen::memory::store_lines::g_FIGURE_UNITS(), &family.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }])), { let __b = mcall(&found.clone(), "group", &[py_str("rest")]); if py_truthy(&__b) { __b } else { py_str("") } }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = per_week.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("asked")]), "lower", &[]), &py_str("much"))); if !py_truthy(&__b) { __b } else { le(&call_builtin("set", &[units.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("set", &[crate::gen::memory::store_lines::g_FIGURE_TIME()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return g_FIGURE_MONEY();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&units.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("asked")]), "lower", &[]), &py_str("much"))) } }) {
    return g_FIGURE_MONEY();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&units.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&mcall(&found.clone(), "group", &[py_str("asked")]), "lower", &[]), &py_str("many"))) } }) {
    return Py::None;
    }
    return { let __b = call_builtin("tuple", &[units.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { crate::gen::memory::store_lines::g_FIGURE_TIME() } };
    Py::None
}
pub fn _the_clause(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut end: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "end") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: end"))) };
    let mut at: Py = Py::None;
    let mut left: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut rights: Py = Py::None;
    let _ = py_str("The clause of `said` a figure sits in, between the punctuation either side of it.");
    left = call_builtin("max", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_str(".;:")) { let mut mark: Py = __item.clone(); __out.push(mcall(&said.clone(), "rfind", &[mark.clone(), Py::Int(0), start.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    rights = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_str(".;:")) { let mut mark: Py = __item.clone(); __out.push(mcall(&said.clone(), "find", &[mark.clone(), end.clone()])); } py_list(__out) })() }) { let mut at: Py = __item.clone(); if py_truthy(&ge(&at.clone(), &Py::Int(0))) {  __out.push(at.clone()); } } py_list(__out) })() };
    return slice_or_raise(&said.clone(), &add(&left.clone(), &Py::Int(1)), &if py_truthy(&rights.clone()) { call_builtin("min", &[rights.clone()]).unwrap_or_else(|e| exc::raise(e)) } else { call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)) }, &Py::None);
    Py::None
}
pub fn _the_money_figures(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("Every amount of money the answer states, in order, as (words, definite).");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&g_A_MONEY_FIGURE(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&mcall(&g_ACTION_AMOUNT(), "search", &[_the_clause(&[said.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[])])])) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]), "rstrip", &[py_str(",;:")]), Py::Bool(true)])]);
    }
    if py_truthy(&out.clone()) {
    return out.clone();
    }
    let __iter2 = py_iter_or_raise(&mcall(&g_A_PERCENT_FIGURE(), "finditer", &[said.clone()]));
    for __item in __iter2 {
    found = __item.clone();
    if py_truthy(&mcall(&g_ACTION_AMOUNT(), "search", &[_the_clause(&[said.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[])])])) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "strip", &[]), "rstrip", &[py_str(",;:")]), Py::Bool(true)])]);
    }
    return out.clone();
    Py::None
}
pub fn _the_figures_in(args: &[Py]) -> Py {
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
    let mut units: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "units") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: units"))) };
    let mut after: Py = Py::None;
    let mut definite: Py = Py::None;
    let mut family: Py = Py::None;
    let mut found: Py = Py::None;
    let mut next_word: Py = Py::None;
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut said_figure: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut unit_word: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Every figure of these units the answer states, in order, as (words, definite).");
    if py_truthy(&Py::Bool(py_contains(&units.clone(), &py_str("money")))) {
    return _the_money_figures(&[said.clone()]);
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&units.clone()) { let mut unit: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&crate::gen::memory::store_lines::g_FIGURE_UNITS(), &unit.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    unit = modulo(&py_str("(?-i:%s)"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    pattern = stdlib_call("re", "compile", &[modulo(&py_str("(?<![\\d,])\\b(?P<qual>%s\\s+)?(?P<num>%s)(?:\\s*(?:-|to|or)\\s*(?P<more>%s))?\\s*(?P<unit>%s)\\b(?P<extra>\\s+\\d{1,3})?(?P<after>[^.;:!?,]*)"), &py_tuple(vec![crate::gen::memory::store_lines::g_FIGURE_QUALIFIER(), crate::gen::memory::store_lines::g_FIGURE_NUMBER(), crate::gen::memory::store_lines::g_FIGURE_NUMBER(), unit.clone()])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&pattern.clone(), "finditer", &[said.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    after = mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("after")]), "strip", &[]), "lower", &[]);
    next_word = geti(&{ let __b = stdlib_call("re", "findall", &[py_str("[a-z']+"), after.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0));
    unit_word = mcall(&mcall(&found.clone(), "group", &[py_str("unit")]), "lower", &[]);
    one = { let __b = Py::Bool(!py_truthy(&mcall(&unit_word.clone(), "endswith", &[py_str("s")]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("km"), py_str("kg"), py_str("lb"), py_str("lbs")]), &unit_word.clone())) } };
    if py_truthy(&{ let __b = one.clone(); if !py_truthy(&__b) { __b } else { { let __b = next_word.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_FIGURE_AFTER(), &next_word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&units.clone()) { let mut family: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&crate::gen::memory::store_lines::g_FIGURE_UNITS(), &family.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }, &next_word.clone())) } } } } } }) {
    continue;
    }
    pieces = py_list(vec![{ let __b = mcall(&found.clone(), "group", &[py_str("qual")]); if py_truthy(&__b) { __b } else { py_str("") } }, mcall(&found.clone(), "group", &[py_str("num")]), if py_truthy(&mcall(&found.clone(), "group", &[py_str("more")])) { add(&py_str("to "), &mcall(&found.clone(), "group", &[py_str("more")])) } else { py_str("") }, mcall(&found.clone(), "group", &[py_str("unit")]), { let __b = mcall(&found.clone(), "group", &[py_str("extra")]); if py_truthy(&__b) { __b } else { py_str("") } }]);
    said_figure = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&pieces.clone()) { let mut piece: Py = __item.clone(); if py_truthy(&mcall(&piece.clone(), "strip", &[])) {  __out.push(mcall(&piece.clone(), "strip", &[])); } } py_list(__out) })() }]);
    if py_truthy(&{ let __b = stdlib_call("re", "match", &[py_str("(?:a|an|per)\\s+"), after.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&next_word.clone(), &py_str("per"))) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_ACTION_DURATION(), "search", &[_the_clause(&[said.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "end", &[])])])) {
    continue;
    }
    definite = Py::Bool(!py_contains(&py_tuple(vec![py_str("a"), py_str("an")]), &mcall(&mcall(&found.clone(), "group", &[py_str("num")]), "lower", &[])));
    if py_truthy(&Py::Bool(!py_truthy(&definite.clone()))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![said_figure.clone(), definite.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn _the_figure_the_answer_names(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut back: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut found: Py = Py::None;
    let mut rate_ask: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stated: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut units: Py = Py::None;
    let _ = py_str("The figure the answer on the screen named, on a line that asks only for a figure, else None.\n\n    \"how many weeks\" after \"Each week add a minute to the jog ... in about eight weeks you will run\n    20 minutes\" names no thing, so no entry is reached and the entry the topic does reach answers\n    the *unit* instead: the warm-up card, \"Five to ten minutes.\" Nobody asked that. The figure is\n    in the answer already on the screen, and `bot/memory/repeat.py` has read the line all along -\n    but its reply was only ever asked for under the composer's last-resort module, and an entry\n    that answers the wrong card wins the turn first.\n\n    The store is where the answer already said belongs, so the reading is claimed here. It is the\n    figure said back and not the sentence it is in, because the sentence is one the person has\n    just read.\n\n    Narrow on purpose: the line asks for one kind of figure and names nothing else, the answer is\n    a knowledge answer from a turn or two back, the figure is definite and of the kind asked for,\n    the unit is the noun of it, a rate (\"three runs a week\") is not a figure, and a length the\n    answer gives for doing something (\"for the first two days\", \"20 minutes at a time\") is a\n    duration of the action, not the length of the thing the person asked about.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    units = _the_unit_asked(&[text.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&units.clone()))) {
    return Py::None;
    }
    theirs = crate::gen::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    rate_ask = call_builtin("bool", &[mcall(&g_A_RATE_ASK(), "match", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&crate::gen::memory::store_lines::g_LOOK_BACK_FIGURE()), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)), Py::Int(1)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    back = __unpk2[0].clone();
    turn = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&crate::gen::memory::store_replies::g_ANSWER_MODULES(), &mcall(&turn.clone(), "get", &[py_str("module")])))) {
    continue;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = lt(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &crate::gen::memory::store_lines::g_FIGURE_ANSWER_LEN()); if py_truthy(&__b) { __b } else { mcall(&mcall(&said.clone(), "rstrip", &[]), "endswith", &[py_str("?")]) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = rate_ask.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_RATE_STATED(), "search", &[said.clone()]) } }) {
    continue;
    }
    if py_truthy(&mcall(&g_LISTED_FIGURE(), "search", &[said.clone()])) {
    continue;
    }
    found = _the_figures_in(&[said.clone(), units.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    if py_truthy(&_a_figure_of_those_units(&[said.clone(), units.clone()])) {
    return Py::None;
    }
    continue;
    }
    stated = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&found.clone()) { let __unpk8719 = py_unpack(&__item.clone(), 2, None); let mut figure: Py = __unpk8719[0].clone(); let mut _u: Py = __unpk8719[1].clone(); __out.push(mcall(&figure.clone(), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&gt(&call_builtin("len", &[stated.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::None;
    }
    figure = geti(&geti(&found.clone(), &Py::Int(0)), &Py::Int(0));
    return add(&add(&mcall(&slice_or_raise(&figure.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&figure.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    }
    return Py::None;
    Py::None
}
pub fn _a_figure_of_those_units(args: &[Py]) -> Py {
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
    let mut units: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "units") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: units"))) };
    let mut pattern: Py = Py::None;
    let mut unit: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the answer states a number with a unit of one of these families after it.\n\n    The loose reading, used only to tell \"this answer says a length and I cannot read which\"\n    from \"this answer says no length at all\": the second may hand the line to an older answer,\n    the first may not.\n    ");
    if py_truthy(&Py::Bool(py_contains(&units.clone(), &py_str("money")))) {
    return call_builtin("bool", &[{ let __b = mcall(&g_A_MONEY_FIGURE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_PERCENT_FIGURE(), "search", &[said.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    }
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push({ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&units.clone()) { let mut unit: Py = __item.clone(); for __item in py_iter_or_raise(&geti(&crate::gen::memory::store_lines::g_FIGURE_UNITS(), &unit.clone())) { let mut word: Py = __item.clone(); __out.push(word.clone()); } } py_set(__out) })() }); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    unit = modulo(&py_str("(?-i:%s)"), &mcall(&py_str("|"), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(stdlib_call("re", "escape", &[word.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]));
    pattern = stdlib_call("re", "compile", &[modulo(&py_str("(?<![\\d,])(?:%s|%s)\\s*(?:-|to|or)?\\s*%s\\b"), &py_tuple(vec![crate::gen::memory::store_lines::g_FIGURE_NUMBER(), py_str("\\d[\\d,.]*"), unit.clone()])), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[mcall(&pattern.clone(), "search", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_time_they_put_right(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut fixed: Py = Py::None;
    let mut said_back: Py = Py::None;
    let mut was: Py = Py::None;
    let mut what: Py = Py::None;
    let _ = py_str("The reply to a line that does nothing but move a routine time, or None.\n\n    \"i get up at 6\" then \"i meant seven\" is one word put right, and the store has always kept it\n    (`times.keep`). But the reply to that line came from the composer's shrug (\"I'm\n    listening - say more if it helps.\") or, after \"make it 7\", from the search hedge: the bot had\n    just moved the time and said nothing about it. So the time is said back in the words the store\n    holds it in, and the one it replaced is named, because a correction is what the line is:\n    \"Got it - you're up at 7, not 6.\"\n    ");
    fixed = mcall(&memory.clone(), "get", &[py_str("fixed_time")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[fixed.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("int", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("turn")]); if py_truthy(&__b) { __b } else { neg(&Py::Int(1)) } }]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    at = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("at")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&at.clone()))) {
    return Py::None;
    }
    what = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("what")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    said_back = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("at".to_string(), at.clone()), ("what".to_string(), what.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&{ let __b = mcall(&crate::gen::memory::store_lines::g_ROUTINE_SAY_BACK(), "get", &[what.clone()]); if py_truthy(&__b) { __b } else { py_str("You {what} at {at}.") } }, "format", args))(&__args) };
    said_back = mcall(&said_back.clone(), "rstrip", &[py_str(" .")]);
    was = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&fixed.clone(), "get", &[py_str("from")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = was.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&was.clone(), &at.clone())) } }) {
    let __aug1 = add(&said_back.clone(), &add(&py_str(", not "), &was.clone()));
    said_back = __aug1;
    }
    return add(&add(&add(&py_str("Got it - "), &mcall(&geti(&said_back.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&said_back.clone(), &Py::Int(1), &Py::None, &Py::None)), &py_str("."));
    Py::None
}
pub fn respond(args: &[Py]) -> Py {
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
    let mut ages_of_the_children: Py = Py::None;
    let mut along: Py = Py::None;
    let mut answer: Py = Py::None;
    let mut ask_about_nothing: Py = Py::None;
    let mut attribute: Py = Py::None;
    let mut behind: Py = Py::None;
    let mut direct: Py = Py::None;
    let mut figure: Py = Py::None;
    let mut framed: Py = Py::None;
    let mut going: Py = Py::None;
    let mut pointed_at_nothing: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut put_right: Py = Py::None;
    let mut recall_about_nothing: Py = Py::None;
    let mut recall_frame: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut ruled_out: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut told_about_the_trip: Py = Py::None;
    let _ = py_str("The store's reply, with the names in it written as the store holds them.\n\n    A plan is quoted back in the words they typed, so a name in it came back in their spelling:\n    \"we r going 2 spain in may\" -> \"You r going 2 spain in may.\". Every route out of the store\n    ends here, so the names are written once, at the end.\n\n    Then the floor under it: never the same words twice in a row. The store answers each line on\n    its own terms, so two different lines can be answered by the same words - \"is she vegan\" and\n    then \"what does my mum eat\" both reached the one line the store writes about a diet nobody\n    gave, and the person read the same sentence twice. Whatever route the reply came by,\n    `bot/memory/again.instead` frames it as the repeat it is - after the names are written, so\n    the words it compares are the words on the screen.\n    ");
    let mut same_again: Py = crate::gen::memory::again::module_obj();
    let mut not_this: Py = crate::gen::memory::not_this::module_obj();
    let mut trips: Py = crate::gen::memory::trips::module_obj();
    text = _without_the_shrug_at_the_end(&[text.clone()]);
    put_right = _the_time_they_put_right(&[memory.clone()]);
    if py_truthy(&put_right.clone()) {
    return put_right.clone();
    }
    pointed_at_nothing = _points_back_at_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&pointed_at_nothing.clone()) {
    return pointed_at_nothing.clone();
    }
    recall_frame = _a_recall_frame_about_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&recall_frame.clone()) {
    return recall_frame.clone();
    }
    ask_about_nothing = _an_ask_about_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&ask_about_nothing.clone()) {
    return ask_about_nothing.clone();
    }
    let mut _goals_progress: Py = crate::gen::memory::goals::module_obj();
    going = mcall(&_goals_progress.clone(), "a_progress_asked", &[text.clone(), memory.clone()]);
    if py_truthy(&going.clone()) {
    return going.clone();
    }
    recall_about_nothing = _a_recall_about_nothing(&[text.clone(), memory.clone()]);
    if py_truthy(&recall_about_nothing.clone()) {
    return recall_about_nothing.clone();
    }
    behind = _the_ask_behind_the_recall(&[text.clone()]);
    if py_truthy(&behind.clone()) {
    let __unpk1 = py_unpack(&behind.clone(), 3, None);
    direct = __unpk1[0].clone();
    attribute = __unpk1[1].clone();
    possessive = __unpk1[2].clone();
    answer = respond(&[direct.clone(), memory.clone()]);
    if py_truthy(&answer.clone()) {
    return answer.clone();
    }
    if py_truthy(&_a_word_for_the_slot(&[memory.clone(), attribute.clone()])) {
    behind = Py::None;
    } else {
    theirs = if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our")]), &possessive.clone()))) { py_str("your") } else { possessive.clone() };
    return py_call(&crate::gen::memory::store_replies::g_pick(), &[py_list(vec![add(&add(&add(&add(&py_str("You haven't told me "), &theirs.clone()), &py_str(" ")), &attribute.clone()), &py_str(".")), add(&add(&add(&add(&py_str("I don't have "), &theirs.clone()), &py_str(" ")), &attribute.clone()), &py_str(" from you yet.")), add(&add(&add(&add(&py_str("You haven't mentioned "), &theirs.clone()), &py_str(" ")), &attribute.clone()), &py_str(" to me."))]), memory.clone()]);
    }
    }
    ages_of_the_children = _the_ages_of_the_children(&[text.clone(), memory.clone()]);
    if py_truthy(&ages_of_the_children.clone()) {
    return ages_of_the_children.clone();
    }
    ruled_out = mcall(&not_this.clone(), "reply_for", &[text.clone(), memory.clone()]);
    if py_truthy(&ruled_out.clone()) {
    return ruled_out.clone();
    }
    told_about_the_trip = mcall(&trips.clone(), "reply_for", &[text.clone(), memory.clone()]);
    if py_truthy(&told_about_the_trip.clone()) {
    return told_about_the_trip.clone();
    }
    if py_truthy(&mcall(&crate::gen::memory::repeat::g_A_BARE_DURATION_ASK(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    theirs = crate::gen::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    if py_truthy(&mcall(&crate::gen::memory::repeat::g_A_PAY_WORD(), "search", &[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])])) {
    theirs = crate::gen::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    if py_truthy(&crate::gen::memory::repeat::how_long_its_been(&[text.clone()])) {
    theirs = crate::gen::memory::repeat::the_figure_they_gave(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    }
    figure = _the_figure_the_answer_names(&[text.clone(), memory.clone()]);
    if py_truthy(&figure.clone()) {
    return figure.clone();
    }
    along = crate::gen::memory::store_replies::_how_far_along(&[memory.clone(), crate::gen::memory::store_replies::_bare(&[py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])])]);
    if py_truthy(&along.clone()) {
    return along.clone();
    }
    theirs = crate::gen::memory::repeat::asked_back(&[text.clone(), memory.clone()]);
    if py_truthy(&theirs.clone()) {
    return theirs.clone();
    }
    reply = crate::gen::memory::store_replies::_the_store_reply(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[reply.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return reply.clone();
    }
    reply = _the_places_are_written_as_names(&[reply.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::memory::store_replies::g_WHOLE_ASK(), "match", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[])]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks_to_hear_it_again(&[text.clone(), memory.clone()]))) } }) {
    framed = mcall(&same_again.clone(), "instead", &[reply.clone(), text.clone(), memory.clone()]);
    if py_truthy(&framed.clone()) {
    return framed.clone();
    }
    }
    return reply.clone();
    Py::None
}
pub fn _asks_to_hear_it_again(args: &[Py]) -> Py {
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
    let _ = py_str("True for the one turn where the same words are the answer: \"what did you say earlier\".\n\n    STYLE.md's rule is about answering twice, not about being asked twice: a person who asks to\n    hear the last answer again is handed it, and framing that as \"Same answer, and it covers\n    this:\" says the words were volunteered again rather than asked for.\n    ");
    let __try1 = exc::catch(|| {
    return py_tuple(vec![Py::Bool(true), crate::gen::memory::store_replies::_repeat_ask(&[text.clone(), py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()])])]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _asks_about_someone_else(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let _ = py_str("True for a yes/no question about a person (\"is he allergic to peanuts\").\n\n    The question mark is not asked for. Most people type an ask about their son on a phone\n    without one, and \"is he allergic to peanuts\" filed peanuts on the son while the same line\n    with a \"?\" did not - the fault this guard exists for, in the one shape where a wrong stored\n    fact matters most. What makes the line an ask is the auxiliary at the front and a person\n    after it: \"is my son\", \"does he\", \"can they\" never open a statement. A word that is not a\n    person (\"is it expensive\", \"is the boiler new\") keeps the question mark it needs, because\n    those openings are also how a statement about the weather goes.\n    ");
    line = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    return call_builtin("bool", &[mcall(&crate::gen::memory::store_replies::g_YES_NO_ASK(), "match", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _note_a_corrected_fact(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut entry: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut was: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: was"))) };
    let mut before: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let _ = py_str("Remember that this turn put a fact about someone right, so the reply can say which one.\n\n    \"my son is 7\" then \"sorry, i meant my son is 8\" is a correction, and the store took it - it\n    had 7 and now has 8 - but nobody said so: the line was answered with \"Thanks for telling me -\n    I'm here if you want to dig into any of that\", which is the reply for news and tells the\n    person the correction did not land. Worse, \"sorry, i meant my son is 8\" carries the word\n    \"sorry\" and a relation, so it reached the row on how to word an apology and got advice about\n    apologising to him. A correction is answered with the fact, so this one is.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[was.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SAID_AGAIN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SORRY_OPENS_A_CORRECTION(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("who"), py_str("pronoun"), py_str("turns")]), &kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), Py::None]), &value.clone())) } }) {
    continue;
    }
    before = mcall(&was.clone(), "get", &[kind.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str(""), Py::None]), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&before.clone(), &value.clone())) } }) {
    continue;
    }
    let _ = py_setitem(&memory.clone(), &py_str("fact_fixed"), py_dict(vec![(py_str("who"), call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))), (py_str("kind"), kind.clone()), (py_str("value"), value.clone()), (py_str("was"), before.clone()), (py_str("said"), mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]))]));
    return Py::None;
    }
    Py::None
}
pub fn _note_a_fact_moved(args: &[Py]) -> Py {
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
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut entry: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry"))) };
    let mut was: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "was") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: was"))) };
    let mut hold: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut one: Py = Py::None;
    let mut other: Py = Py::None;
    let mut said: Py = Py::None;
    let mut value: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Remember a correction that moves a fact from one person to another.\n\n    \"my son is 9\", then \"sorry i meant my daughter is 9\": the store filed the age on the\n    daughter, but the son kept his, so the chat ended up with two nine year olds and no word\n    said about it. The line also carries the word \"sorry\" and a relation, so it reached the row\n    on how to word an apology and the person who had just put a fact right was told to name what\n    they had done wrong. A correction that names another person takes the fact off the first one,\n    and the reply says which person it now belongs to.\n\n    Only the shape a correction is said in (\"i meant\", \"sorry\") counts, and only when a\n    different person already holds the same value: \"my daughter is 9 too\" is news about a second\n    child, not a correction, and it files both.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_SAID_AGAIN(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_SORRY_OPENS_A_CORRECTION(), "search", &[said.clone()]) } }))) {
    return Py::None;
    }
    who = mcall(&call_builtin("str", &[{ let __b = mcall(&entry.clone(), "get", &[py_str("who")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&who.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&stdlib_call("re", "search", &[add(&add(&py_str("\\b"), &stdlib_call("re", "escape", &[who.clone()]).unwrap_or_else(|e| exc::raise(e))), &py_str("\\b")), said.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&mcall(&entry.clone(), "items", &[]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    kind = __unpk2[0].clone();
    value = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("who"), py_str("pronoun"), py_str("turns")]), &kind.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str(""), Py::None]), &value.clone())) } }) {
    continue;
    }
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = call_builtin("isinstance", &[was.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&py_tuple(vec![py_str(""), Py::None]), &mcall(&was.clone(), "get", &[kind.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&was.clone(), "get", &[kind.clone()]), &value.clone())) } } } }) {
    continue;
    }
    hold = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_others(), &[memory.clone()])) { let mut one: Py = __item.clone(); if py_truthy(&{ let __b = call_builtin("isinstance", &[one.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&one.clone(), "get", &[py_str("who")]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_eq(&mcall(&one.clone(), "get", &[py_str("who")]), &who.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&one.clone(), "get", &[kind.clone()]), &value.clone())) } } } } } }) {  __out.push(one.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&hold.clone()))) {
    continue;
    }
    other = geti(&hold.clone(), &Py::Int(0));
    let _ = py_setitem(&memory.clone(), &py_str("fact_moved"), py_dict(vec![(py_str("who"), who.clone()), (py_str("was_who"), call_builtin("str", &[mcall(&other.clone(), "get", &[py_str("who")])]).unwrap_or_else(|e| exc::raise(e))), (py_str("kind"), kind.clone()), (py_str("value"), value.clone()), (py_str("said"), said.clone())]));
    let _ = py_setitem(&memory.clone(), &py_str("moved_to_who"), said.clone());
    let _ = mcall(&other.clone(), "pop", &[kind.clone(), Py::None]);
    return Py::None;
    }
    Py::None
}
pub fn _a_word_for_the_slot(args: &[Py]) -> Py {
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
    let mut attribute: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "attribute") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: attribute"))) };
    let mut pattern: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("Whether a line of theirs in this chat holds a word for this slot.");
    pattern = mcall(&g_SLOT_WORDS(), "get", &[mcall(&call_builtin("str", &[{ let __b = attribute.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&pattern.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_call(&crate::gen::memory::store_replies::g_recent(), &[memory.clone(), Py::Int(8)])) { let mut turn: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_ask_behind_the_recall(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut attribute: Py = Py::None;
    let mut name: Py = Py::None;
    let mut only_slot: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut person: Py = Py::None;
    let mut plain: Py = Py::None;
    let mut possessive: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut said: Py = Py::None;
    let mut slot: Py = Py::None;
    let mut slot_word: Py = Py::None;
    let mut verb: Py = Py::None;
    let mut who: Py = Py::None;
    let mut word: Py = Py::None;
    let mut wrapped: Py = Py::None;
    let _ = py_str("The direct ask and the slot a recall ask is about, or None.\n\n    Only a slot the person could have given in words is read this way, and only with a possessive\n    in front of it (\"his name\", \"my budget\"): \"what did i say about the cake\" and \"what did i say\n    was wrong with it\" are not asks about a slot, and they keep their own readers.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[low.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    wrapped = mcall(&g_RECALL_WRAPPER(), "match", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wrapped.clone()))) {
    return Py::None;
    }
    rest = stdlib_call("re", "sub", &[py_str("\\s+(?:was|is|were|are)\\s*$"), py_str(""), mcall(&mcall(&wrapped.clone(), "group", &[py_str("rest")]), "strip", &[])]).unwrap_or_else(|e| exc::raise(e));
    said = mcall(&mcall(&mcall(&wrapped.clone(), "group", &[py_str("word")]), "lower", &[]), "replace", &[py_str("  "), py_str(" ")]);
    slot = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_RECALL_SLOTS()) { let __unpk7499 = py_unpack(&__item.clone(), 3, None); let mut pattern: Py = __unpk7499[0].clone(); let mut word: Py = __unpk7499[1].clone(); let mut name: Py = __unpk7499[2].clone(); if py_truthy(&mcall(&pattern.clone(), "search", &[rest.clone()])) {  __out.push(py_tuple(vec![word.clone(), name.clone()])); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&slot.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&slot.clone(), 2, None);
    word = __unpk1[0].clone();
    attribute = __unpk1[1].clone();
    possessive = mcall(&g_RECALL_POSSESSIVE(), "match", &[rest.clone()]);
    if py_truthy(&mcall(&said.clone(), "startswith", &[py_str("how old")])) {
    person = mcall(&mcall(&rest.clone(), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_SHE_OR_HE(), &person.clone()))) {
    return Py::None;
    }
    let __unpk2 = py_unpack(&geti(&g_SHE_OR_HE(), &person.clone()), 2, None);
    who = __unpk2[0].clone();
    verb = __unpk2[1].clone();
    return py_tuple(vec![add(&add(&py_str("how old "), &verb.clone()), &if py_truthy(&Py::Bool(py_eq(&who.clone(), &py_str("i")))) { py_str(" i") } else { add(&py_str(" "), &who.clone()) }), py_str("age"), person.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&possessive.clone()))) {
    return Py::None;
    }
    person = mcall(&mcall(&possessive.clone(), "group", &[py_str("poss")]), "lower", &[]);
    plain = mcall(&py_str(" "), "join", &[mcall(&stdlib_call("re", "sub", &[py_str("\\b(?:was|is|were|are)\\b"), py_str(" "), rest.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&plain.clone()))) {
    return Py::None;
    }
    word = if py_truthy(&Py::Bool(py_eq(&attribute.clone(), &py_str("birthday")))) { py_str("when is ") } else { py_str("what is ") };
    slot_word = mcall(&g_RECALL_SLOTS_BY_POSSESSIVE(), "match", &[plain.clone()]);
    only_slot = if py_truthy(&slot_word.clone()) { mcall(&slot_word.clone(), "group", &[py_str("attribute")]) } else { Py::None };
    return py_tuple(vec![add(&word.clone(), &plain.clone()), { let __b = only_slot.clone(); if py_truthy(&__b) { __b } else { { let __b = attribute.clone(); if py_truthy(&__b) { __b } else { py_str("answer") } } } }, person.clone()]);
    Py::None
}
pub fn TAKES_AS_IT_COMES(args: &[Py]) -> Py {
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
    else { Py::None };
    let _ = py_str("Claim every line the store can answer - the bare \"what do i do\" included.\n\n    The arbiter reads one `TAKES_AS_IT_COMES` per module and the last `def` in the file is the one\n    it reads, so a hook written above this one is not read at all: its line goes unclaimed, and the\n    specialist rule hands it to whatever entry the topic fits. That is what happened to the store's\n    own readings for a while (\"my son is 5\" then \"is my son 5\", \"he isnt allergic to nuts\", a plan\n    asked about by \"when\"), which is why every reading now has a private name and is asked from\n    here, at the end of the file, rather than from a hook of its own.\n\n    The job ask is the last reading: the bare \"what do i do\" said right after the person gave their\n    job is the fact, whatever else read. The line names nothing, so the understanding engine hands\n    it the topic of the turn before, and a knowledge entry for that trade answers it: an answer\n    about the entryways into nursing, given to someone who already is one and only asked what they\n    do. The store's reading is the one that answers what they asked, and it wins the turn on this\n    line alone. False everywhere else - a turn later the same words ask about whatever the chat has\n    moved on to.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&memory.clone(), "get", &[py_str("moved_to_who")]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])])))) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_store_readings(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::told::A_CALLBACK(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_the_ask_about_how_it_went(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&_a_callback_that_names_its_thing(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&crate::gen::memory::reminded::about_a_thing_they_said(&[text.clone(), memory.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&crate::gen::memory::store_replies::g_BARE_WHAT_I_DO(), "match", &[crate::gen::memory::store_replies::_bare(&[{ let __b = py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { text.clone() } }])])))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("job")])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_the_job_was_just_said(&[memory.clone()]) } };
    Py::None
}
pub fn _about_the_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut plans: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "plans") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: plans"))) };
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut found: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "found") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: found"))) };
    let mut named: Py = Py::None;
    let mut plan: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The plan a when or what-time ask is about: the one that says its verb, and its name too.\n\n    Two plans can say the same verb - \"meeting sam at 6 tomorrow\" and \"meeting jo at 8 on friday\"\n    both say \"meeting\" - and the verb on its own answers \"what time am I meeting sam\" with jo's\n    hour, which is an answer about somebody the person did not ask about. The words the ask puts\n    after the verb are what tell the two plans apart, so a plan that names one of them wins.\n    When no plan names them the verb-only reading still stands: \"when do I start\" names nobody,\n    and \"what time do I start work\" keeps its answer even though the plan says \"job\".\n    ");
    named = py_call(&crate::gen::memory::store_replies::g_content(), &[slice_or_raise(&low.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)]);
    if py_truthy(&named.clone()) {
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    plan = __item.clone();
    if py_truthy(&{ let __b = crate::gen::memory::store_replies::_says_the_verb(&[plan.clone(), mcall(&found.clone(), "group", &[Py::Int(1)])]); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&named.clone()) { let mut word: Py = __item.clone(); __out.push(crate::gen::memory::store_replies::_names_the_thing(&[plan.clone(), word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return plan.clone();
    }
    }
    }
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[plans.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    plan = __item.clone();
    if py_truthy(&crate::gen::memory::store_replies::_says_the_verb(&[plan.clone(), mcall(&found.clone(), "group", &[Py::Int(1)])])) {
    return plan.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn _remember_dislikes(args: &[Py]) -> Py {
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
    let mut named_here: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named_here") { v.clone() }
    else { py_tuple(vec![]) };
    let mut at: Py = Py::None;
    let mut food: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("Keep a food someone else will not eat against that person: \"she hates mushrooms\".");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    let __iter1 = py_iter_or_raise(&crate::gen::memory::guest::found(&[text.clone()]));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    food = __unpk2[0].clone();
    at = __unpk2[1].clone();
    let __unpk3 = py_unpack(&crate::gen::memory::facts::subject(&[text.clone(), at.clone()]), 2, None);
    kind = __unpk3[0].clone();
    who = __unpk3[1].clone();
    if py_truthy(&Py::Bool(py_eq(&kind.clone(), &py_str("user")))) {
    food = crate::gen::memory::guest::plain(&[food.clone()]);
    if py_truthy(&food.clone()) {
    let _ = py_call(&crate::gen::memory::store_replies::g_remember(), &[memory.clone(), py_str("dislikes"), py_list(vec![food.clone()]), text.clone()]);
    }
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&kind.clone(), &py_str("other")))) {
    continue;
    }
    who = { let __b = who.clone(); if py_truthy(&__b) { __b } else { py_call(&crate::gen::memory::store_replies::g__person_from_pronoun(), &[text.clone(), memory.clone(), named_here.clone()]) } };
    let _ = py_call(&crate::gen::memory::store_replies::g_remember_other(), &[memory.clone(), py_dict(vec![(py_str("who"), who.clone()), (py_str("dislikes"), py_list(vec![food.clone()]))]), text.clone()]);
    }
    Py::None
}
pub fn _diet_reply(args: &[Py]) -> Py {
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
    let mut low: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { py_str("") };
    let mut changed_from: Py = Py::None;
    let mut diet: Py = Py::None;
    let mut gone: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut left_out: Py = Py::None;
    let mut said: Py = Py::None;
    let mut said_so: Py = Py::None;
    let mut take_back: Py = Py::None;
    let mut theirs: Py = Py::None;
    let mut was: Py = Py::None;
    let mut who: Py = Py::None;
    let _ = py_str("How they eat, said back when they ask: \"You told me you're vegetarian.\"\n\n    The diet is kept on every line they say it in, so the answer is the fact and nothing else.\n    A diet stored as a negation comes back with its verb (\"you don't eat meat\"), because \"you're\n    no meat\" says nothing. What they cannot eat is not one key, though: \"im coeliac\" is a\n    condition and \"im allergic to nuts\" is an allergy, and a question about the fact is about\n    all of it - so the read says back whichever the store holds, and only when the line asks\n    about the fact (a line asking for a meal keeps its meal).\n    ");
    said = crate::gen::memory::change::reported(&[low.clone(), py_str("diet")]);
    if py_truthy(&said.clone()) {
    was = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("diet")]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&said.clone(), &py_str("changed"))); if !py_truthy(&__b) { __b } else { was.clone() } }) {
    let _ = py_call(&crate::gen::memory::store_replies::g_forget(), &[memory.clone(), py_str("diet"), py_list(vec![was.clone()])]);
    let _ = py_setitem(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), &py_str("diet_changed"), was.clone());
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(py_str("diet")); __kws.extend(vec![("was".to_string(), was.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::change::reply(args))(&__args) };
    }
    diet = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("diet")]);
    if py_truthy(&Py::Bool(!py_truthy(&diet.clone()))) {
    if py_truthy(&{ let __b = call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))); if !py_truthy(&__b) { __b } else { py_call(&g__food_facts(), &[memory.clone()]) } } } }) {
    let __unpk1 = py_unpack(&py_call(&g__what_a_dinner_has_to_leave_out(), &[memory.clone()]), 2, None);
    left_out = __unpk1[0].clone();
    said_so = __unpk1[1].clone();
    if py_truthy(&left_out.clone()) {
    return add(&add(&add(&add(&py_str("Nothing with "), &py_call(&g__say_a_list(), &[left_out.clone()])), &py_str(" in it — ")), &mcall(&py_str(" and "), "join", &[said_so.clone()])), &py_str(". Tell me what is in the fridge and I'll pick a dinner around it."));
    }
    }
    kept = py_call(&g__food_facts(), &[memory.clone()]);
    if py_truthy(&kept.clone()) {
    return add(&add(&py_str("You told me you're "), &py_call(&g__say_a_list(), &[kept.clone()])), &py_str("."));
    }
    if py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    kept = py_call(&g__food_facts(), &[memory.clone()]);
    if py_truthy(&kept.clone()) {
    return add(&add(&py_str("You told me you're "), &py_call(&g__say_a_list(), &[kept.clone()])), &py_str("."));
    }
    }
    let __unpk2 = py_unpack(&py_call(&g__other_fact(), &[memory.clone(), py_str("diet")]), 2, None);
    who = __unpk2[0].clone();
    theirs = __unpk2[1].clone();
    if py_truthy(&{ let __b = theirs.clone(); if !py_truthy(&__b) { __b } else { who.clone() } }) {
    return add(&add(&py_str("You haven't told me about yours — the diet you mentioned is your "), &who.clone()), &py_str("'s."));
    }
    changed_from = mcall(&py_call(&crate::gen::memory::store_replies::g_user(), &[memory.clone()]), "get", &[py_str("diet_changed")]);
    if py_truthy(&{ let __b = changed_from.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]) } }) {
    was = mcall(&call_builtin("str", &[changed_from.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    was = if py_truthy(&mcall(&mcall(&was.clone(), "lower", &[]), "startswith", &[py_str("no ")])) { add(&py_str("not eating "), &slice_or_raise(&was.clone(), &Py::Int(3), &Py::None, &Py::None)) } else { add(&py_str("being "), &py_call(&g__diet_words(), &[was.clone()])) };
    return add(&add(&py_str("You told me your diet had changed from "), &was.clone()), &py_str(". What is it now?"));
    }
    let __unpk3 = py_unpack(&py_call(&g__the_take_back(), &[memory.clone(), py_str("diet")]), 2, None);
    take_back = __unpk3[0].clone();
    gone = __unpk3[1].clone();
    if py_truthy(&gone.clone()) {
    return py_call(&g__a_take_back_said_back(), &[take_back.clone(), gone.clone()]);
    }
    if py_truthy(&{ let __b = mcall(&g_MEAL_WANTED(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(memory.clone()); __args.push(py_str("diet")); __args.push(py_str("You haven't told me about your diet.")); __kws.extend(vec![("own".to_string(), g__their_own_diet())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| py_call(&g__deny(), args))(&__args) };
    }
    diet = call_builtin("str", &[diet.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = mcall(&g_MEAL_WANTED(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_DIET_FACT_ASK(), "search", &[call_builtin("str", &[{ let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]))) } }) {
    let __unpk4 = py_unpack(&py_call(&g__what_a_dinner_has_to_leave_out(), &[memory.clone()]), 2, None);
    left_out = __unpk4[0].clone();
    said_so = __unpk4[1].clone();
    if py_truthy(&left_out.clone()) {
    return add(&add(&add(&add(&py_str("Nothing with "), &py_call(&g__say_a_list(), &[left_out.clone()])), &py_str(" in it — ")), &mcall(&py_str(" and "), "join", &[said_so.clone()])), &py_str(". Tell me what is in the fridge and I'll pick a dinner around it."));
    }
    return Py::None;
    }
    if py_truthy(&mcall(&diet.clone(), "startswith", &[py_str("no ")])) {
    return add(&add(&add(&add(&py_str("You told me you don't "), &py_call(&g__diet_how(), &[diet.clone()])), &py_str(" ")), &slice_or_raise(&diet.clone(), &Py::Int(3), &Py::None, &Py::None)), &py_str("."));
    }
    if py_truthy(&Py::Bool(py_contains(&g_WAY_OF_EATING(), &diet.clone()))) {
    return add(&add(&py_str("You told me you're "), &py_call(&g__diet_words(), &[diet.clone()])), &py_str("."));
    }
    if py_truthy(&Py::Bool(py_contains(&g_PLAN_DIETS(), &diet.clone()))) {
    return add(&add(&py_str("You told me you're doing "), &py_call(&g__diet_words(), &[diet.clone()])), &py_str("."));
    }
    return add(&add(&py_str("You told me you're "), &diet.clone()), &py_str("."));
    Py::None
}
pub fn _a_fact_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut low: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "low") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: low"))) };
    let mut after: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the thing a callback names is one of the facts the store keeps.");
    after = stdlib_call("re", "search", &[py_str("\\babout\\b\\s+(.+)$"), { let __b = low.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    words = call_builtin("set", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&if py_truthy(&after.clone()) { mcall(&after.clone(), "group", &[Py::Int(1)]) } else { py_str("") }, "lower", &[])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[bitand(&words.clone(), &g_FACT_WORDS())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_callback_that_names_its_thing(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the line asks about the conversation and names the thing it asks about.\n\n    \"what have i been saying about the boiler\" is a recap by name (`_about_this_chat`, the reading\n    the store answers with) and the name is what makes it about that thing rather than the chat as\n    a whole (`_names_its_thing`). Both, because either on its own is too wide for a claim: a line\n    that names nothing is the chat's own topic, and a line that names a boiler is not a recap\n    unless the store read it as one.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]);
    return py_tuple(vec![Py::Bool(true), call_builtin("bool", &[{ let __b = crate::gen::memory::store_replies::_names_its_thing(&[low.clone()]); if !py_truthy(&__b) { __b } else { crate::gen::memory::store_replies::_about_this_chat(&[text.clone(), memory.clone()]) } }]).unwrap_or_else(|e| exc::raise(e))]);
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
    return Py::Bool(false);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    Py::None
}
pub fn _points_back_at_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("The store's answer to a pointer with nothing behind it, or None when it is not that line.\n\n    \"same as before\" as the first thing in a chat is not a request the store can answer from what\n    it holds, and it is not small talk either: it is the person picking up a conversation that is\n    not there. Saying that plainly is the answer, and it is the same answer NOTHING_STARTED gives\n    to \"what have we talked about\" - so a chat that opens this way gets a line that names the\n    missing thing and asks for it, not a welcome and not a receipt.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&crate::gen::memory::thread::points_back(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::thread::nothing_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    return crate::gen::memory::store_replies::_vary(&[crate::gen::memory::store_lines::g_POINTS_BACK_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _a_recall_about_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("The store's answer to \"what was it i was worried about\" with nothing said, or None.\n\n    The line names no slot and no fact: it asks the chat to give back something that was never\n    put in it. The readers below all want a thing to look for, so none of them reads it, and it\n    fell through to the chatter band as a line about a worry.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&crate::gen::memory::thread::asks_what_i_said(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::thread::nothing_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    return crate::gen::memory::store_replies::_vary(&[crate::gen::memory::store_lines::g_WHAT_I_SAID_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _names_nothing_in_a_recall(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut rest: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "rest") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: rest"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when what follows a recall ask names no thing of its own.");
    words = stdlib_call("re", "findall", &[py_str("[a-z]+"), mcall(&mcall(&call_builtin("str", &[{ let __b = rest.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")])]).unwrap_or_else(|e| exc::raise(e));
    return Py::Bool(!py_truthy(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_RECALL_NAMES_NOTHING(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }));
    Py::None
}
pub fn _a_recall_frame_about_nothing(args: &[Py]) -> Py {
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
    let mut another: Py = Py::None;
    let mut low: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut wrapped: Py = Py::None;
    let mut wrapped_low: Py = Py::None;
    let _ = py_str("The store's answer to \"what did i say about it\" with nothing said, or None.\n\n    The recall frame - \"what did i say\", \"what did i say about that\", \"what did i say i lost\" -\n    is the ask for their own words, and it is that whatever the frame is filled with. The \"it\"\n    or the \"that\" inside it is not a thing the chat could name: the person is asking what they\n    themselves said, so there is nothing to give them, and saying that is the answer.\n\n    Read before the pointer asks, because the pointer reader looked at the same \"it\" and answered\n    a request for the chat's own words as if the person had asked a question about a subject\n    (\"Nothing said yet for that to be: tell me the thing itself...\"). A rule-out, not a new pool:\n    it hands back the same WHAT_I_SAID_NOTHING line the feelings recall gets.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&crate::gen::memory::thread::nothing_said(&[memory.clone()])))) {
    return Py::None;
    }
    low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    wrapped_low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    another = mcall(&g_A_RECALL_IN_ANOTHER_SHAPE(), "match", &[wrapped_low.clone()]);
    if py_truthy(&{ let __b = another.clone(); if !py_truthy(&__b) { __b } else { _names_nothing_in_a_recall(&[slice_or_raise(&wrapped_low.clone(), &mcall(&another.clone(), "end", &[]), &Py::None, &Py::None)]) } }) {
    return crate::gen::memory::store_replies::_vary(&[crate::gen::memory::store_lines::g_WHAT_I_SAID_NOTHING(), memory.clone()]);
    }
    wrapped = mcall(&g_RECALL_WRAPPER(), "match", &[wrapped_low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&wrapped.clone()))) {
    return Py::None;
    }
    rest = mcall(&wrapped.clone(), "group", &[py_str("rest")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&crate::gen::memory::thread::g_POINTING_WORD(), "search", &[rest.clone()]))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[stdlib_call("re", "findall", &[py_str("[a-z]+"), rest.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    return Py::None;
    }
    return crate::gen::memory::store_replies::_vary(&[crate::gen::memory::store_lines::g_WHAT_I_SAID_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _an_ask_about_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("The store's answer to an ask about nothing at all, or None when it is not that line.\n\n    \"how do i do it\" as the first line of a chat names one thing - the \"it\" - and the chat holds\n    nothing it can be. Nothing below reads the line: no fact is asked for, no plan is stated, so\n    the turn went to a search hedge over a question the person could be helped with in one line.\n    The same goes for \"do that again\" and \"is it worth it\". The answer says the thing is missing\n    and asks for it, which is what a friend does with a question whose subject they never heard.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&crate::gen::memory::thread::points_at_a_word_alone(&[text.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::thread::nothing_said(&[memory.clone()]))) } }) {
    return Py::None;
    }
    let mut fallback: Py = crate::gen::compose::fallback::module_obj();
    if py_truthy(&mcall(&fallback.clone(), "_points_at_nothing", &[text.clone(), memory.clone()])) {
    return Py::None;
    }
    return crate::gen::memory::store_replies::_vary(&[crate::gen::memory::store_lines::g_POINTING_WORD_NOTHING(), memory.clone()]);
    Py::None
}
pub fn _the_last_ask_about_the_bot(args: &[Py]) -> Py {
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
    let mut asked: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The last thing they asked about the bot, as a sentence, when that is all they have said.\n\n    A chat that opens \"are you a real person\" and then asks what we have talked about has nothing\n    of theirs to go back over - but \"nothing yet\" is not true either, because they asked something.\n    The question is said back in their own words (\"whether I am a real person\") as the one thing\n    the chat has covered, which is what a friend would say.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[call_builtin("list", &[crate::gen::memory::thread::user_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    text = __unpk2[0].clone();
    reading = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = call_builtin("isinstance", &[reading.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&reading.clone(), "get", &[py_str("about_bot")]) } }))) {
    return Py::None;
    }
    asked = crate::gen::memory::thread::restate(&[text.clone()]);
    if py_truthy(&asked.clone()) {
    return asked.clone();
    }
    }
    return Py::None;
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
    let mut answer: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "answer") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: answer"))) };
    let mut room: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "room") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: room"))) };
    let mut best: Py = Py::None;
    let mut best_seconds: Py = Py::None;
    let mut names: Py = Py::None;
    let mut part: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut took: Py = Py::None;
    let _ = py_str("The move of an answer whose own length is the largest one that still fits `room`.\n\n    The largest that fits, not the shortest: somebody with 20 minutes wants the twenty-minute way\n    of doing it, not the two-minute half of a paragraph that is about something else.\n    ");
    let __unpk1 = py_unpack(&py_tuple(vec![Py::None, neg(&Py::Int(1))]), 2, None);
    best = __unpk1[0].clone();
    best_seconds = __unpk1[1].clone();
    names = crate::gen::memory::store_replies::_names_minutes(&[answer.clone()]);
    let __iter2 = py_iter_or_raise(&crate::gen::memory::store_replies::_sentences(&[answer.clone()]));
    for __item in __iter2 {
    sentence = __item.clone();
    let __iter3 = py_iter_or_raise(&crate::gen::memory::store_replies::_halves(&[sentence.clone()]));
    for __item in __iter3 {
    part = __item.clone();
    took = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(part.clone()); __kws.extend(vec![("bare".to_string(), names.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| crate::gen::memory::store_replies::_longest(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_is(&took.clone(), &Py::None)); if py_truthy(&__b) { __b } else { { let __b = gt(&took.clone(), &room.clone()); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&crate::gen::memory::store_replies::_says_something(&[part.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&gt(&took.clone(), &best_seconds.clone())) {
    let __unpk4 = py_unpack(&py_tuple(vec![part.clone(), took.clone()]), 2, None);
    best = __unpk4[0].clone();
    best_seconds = __unpk4[1].clone();
    }
    }
    }
    return best.clone();
    Py::None
}
pub fn _asks_whether_we_followed(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("True when the whole line asks whether the last thing they said was taken in.");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    low = crate::gen::memory::store_replies::_plain_line(&[if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { py_call(&crate::gen::memory::store_replies::g__folded(), &[text.clone(), memory.clone()]) } else { text.clone() }]);
    if py_truthy(&Py::Bool(!py_truthy(&low.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&low.clone(), "endswith", &[py_str("?")])) {
    low = mcall(&slice_or_raise(&low.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), "strip", &[]);
    }
    if py_truthy(&crate::gen::memory::store_replies::_reads_as_the_invitation(&[low.clone(), memory.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&crate::gen::memory::store_replies::g_CHECK_FOLLOWED(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_group_size_ask(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let _ = py_str("Whether this is a question about how many the person's own group is, and the number is kept.\n\n    The ask has to be about the group itself - \"how many of us\", \"how many people are there\",\n    \"how big is my family\" - and never about how many are going somewhere (\"how many of us are\n    going to the park\"), which `counts.GROUP_ASK` already tells apart. Nothing is claimed unless\n    the store actually holds a size, so a \"how many people\" with nothing said before it keeps\n    whatever answer the line has of its own.\n    ");
    low = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&crate::gen::memory::counts::g_GROUP_ASK(), "search", &[low.clone()]); if py_truthy(&__b) { __b } else { mcall(&crate::gen::memory::counts::g_BARE_COUNT_ASK(), "search", &[low.clone()]) } }))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[crate::gen::memory::counts::answer(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.modules.memory_store".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACTION_AMOUNT" => Some(g_ACTION_AMOUNT()),
            "ACTION_DURATION" => Some(g_ACTION_DURATION()),
            "AGES_IN_WORDS" => Some(g_AGES_IN_WORDS()),
            "AGE_FIX" => Some(g_AGE_FIX()),
            "AGE_IN_WORDS" => Some(g_AGE_IN_WORDS()),
            "ANOTHER_ANIMAL" => Some(g_ANOTHER_ANIMAL()),
            "ASK_ABOUT_THEM" => Some(g_ASK_ABOUT_THEM()),
            "ASK_OPENERS" => Some(g_ASK_OPENERS()),
            "A_MONEY_FIGURE" => Some(g_A_MONEY_FIGURE()),
            "A_NAME_AFTER_A_PRONOUN" => Some(g_A_NAME_AFTER_A_PRONOUN()),
            "A_NAME_WITH_ITS_VERB" => Some(g_A_NAME_WITH_ITS_VERB()),
            "A_PERCENT_FIGURE" => Some(g_A_PERCENT_FIGURE()),
            "A_RATE_ASK" => Some(g_A_RATE_ASK()),
            "A_RATE_STATED" => Some(g_A_RATE_STATED()),
            "A_RECALL_IN_ANOTHER_SHAPE" => Some(g_A_RECALL_IN_ANOTHER_SHAPE()),
            "CORRECTING" => Some(g_CORRECTING()),
            "CORRECTS_A_MAKE" => Some(g_CORRECTS_A_MAKE()),
            "CORRECTS_A_NAME" => Some(g_CORRECTS_A_NAME()),
            "CORRECTS_A_NAME_POSSESSIVE" => Some(g_CORRECTS_A_NAME_POSSESSIVE()),
            "FACT_WORDS" => Some(g_FACT_WORDS()),
            "FIGURE_AFTER" => Some(g_FIGURE_AFTER()),
            "FIGURE_ASK" => Some(g_FIGURE_ASK()),
            "FIGURE_FILLER" => Some(g_FIGURE_FILLER()),
            "FIGURE_MONEY" => Some(g_FIGURE_MONEY()),
            "LISTED_FIGURE" => Some(g_LISTED_FIGURE()),
            "NAMED_FACT_ASK" => Some(g_NAMED_FACT_ASK()),
            "NAME_FIX" => Some(g_NAME_FIX()),
            "NAME_GIVEN" => Some(g_NAME_GIVEN()),
            "NAME_NOT_A_NAME" => Some(g_NAME_NOT_A_NAME()),
            "NOT_A_FACT" => Some(g_NOT_A_FACT()),
            "NOT_A_NAME_AFTER_CALLED" => Some(g_NOT_A_NAME_AFTER_CALLED()),
            "NOT_A_NAME_BEFORE" => Some(g_NOT_A_NAME_BEFORE()),
            "NOT_A_NEW_VALUE" => Some(g_NOT_A_NEW_VALUE()),
            "NOT_FROM_THE_REPAIR" => Some(g_NOT_FROM_THE_REPAIR()),
            "NUMBER" => Some(g_NUMBER()),
            "OTHER_NOUNS" => Some(g_OTHER_NOUNS()),
            "PAIR_WITHOUT_A_MARKER" => Some(g_PAIR_WITHOUT_A_MARKER()),
            "PERSON_ADVICE_ASK" => Some(g_PERSON_ADVICE_ASK()),
            "PET_AGE_ANSWER" => Some(g_PET_AGE_ANSWER()),
            "PET_LEAD_IN" => Some(g_PET_LEAD_IN()),
            "PET_NAME_AGE" => Some(g_PET_NAME_AGE()),
            "PREGNANCY_SAID_AGAIN" => Some(g_PREGNANCY_SAID_AGAIN()),
            "RECALL_NAMES_NOTHING" => Some(g_RECALL_NAMES_NOTHING()),
            "RECALL_POSSESSIVE" => Some(g_RECALL_POSSESSIVE()),
            "RECALL_SLOTS" => Some(g_RECALL_SLOTS()),
            "RECALL_SLOTS_BY_POSSESSIVE" => Some(g_RECALL_SLOTS_BY_POSSESSIVE()),
            "RECALL_WRAPPER" => Some(g_RECALL_WRAPPER()),
            "REMIND_QUESTION" => Some(g_REMIND_QUESTION()),
            "SAID_AGAIN" => Some(g_SAID_AGAIN()),
            "SHE_OR_HE" => Some(g_SHE_OR_HE()),
            "SLOT_WORDS" => Some(g_SLOT_WORDS()),
            "SORRY_OPENS_A_CORRECTION" => Some(g_SORRY_OPENS_A_CORRECTION()),
            "TAKES_AS_IT_COMES" => Some(py_func("TAKES_AS_IT_COMES", |args: &[Py]| TAKES_AS_IT_COMES(args))),
            "THE_PLACE_IS_NAMED" => Some(g_THE_PLACE_IS_NAMED()),
            "TOO_AS_WELL" => Some(g_TOO_AS_WELL()),
            "WEEKS_FIX" => Some(g_WEEKS_FIX()),
            "WORK_ON_IT" => Some(g_WORK_ON_IT()),
            "_a_callback_that_names_its_thing" => Some(py_func("_a_callback_that_names_its_thing", |args: &[Py]| _a_callback_that_names_its_thing(args))),
            "_a_correction_without_a_marker" => Some(py_func("_a_correction_without_a_marker", |args: &[Py]| _a_correction_without_a_marker(args))),
            "_a_count_from_the_date" => Some(py_func("_a_count_from_the_date", |args: &[Py]| _a_count_from_the_date(args))),
            "_a_detail_does_not_replace_the_job" => Some(py_func("_a_detail_does_not_replace_the_job", |args: &[Py]| _a_detail_does_not_replace_the_job(args))),
            "_a_fact_word" => Some(py_func("_a_fact_word", |args: &[Py]| _a_fact_word(args))),
            "_a_feeling_about_their_own_news" => Some(py_func("_a_feeling_about_their_own_news", |args: &[Py]| _a_feeling_about_their_own_news(args))),
            "_a_figure_of_those_units" => Some(py_func("_a_figure_of_those_units", |args: &[Py]| _a_figure_of_those_units(args))),
            "_a_kept_fact_read" => Some(py_func("_a_kept_fact_read", |args: &[Py]| _a_kept_fact_read(args))),
            "_a_person" => Some(py_func("_a_person", |args: &[Py]| _a_person(args))),
            "_a_person_called" => Some(py_func("_a_person_called", |args: &[Py]| _a_person_called(args))),
            "_a_pet_new_this_line" => Some(py_func("_a_pet_new_this_line", |args: &[Py]| _a_pet_new_this_line(args))),
            "_a_recall_about_nothing" => Some(py_func("_a_recall_about_nothing", |args: &[Py]| _a_recall_about_nothing(args))),
            "_a_recall_frame_about_nothing" => Some(py_func("_a_recall_frame_about_nothing", |args: &[Py]| _a_recall_frame_about_nothing(args))),
            "_a_second_persons_limit" => Some(py_func("_a_second_persons_limit", |args: &[Py]| _a_second_persons_limit(args))),
            "_a_trip_booked_named" => Some(py_func("_a_trip_booked_named", |args: &[Py]| _a_trip_booked_named(args))),
            "_a_word_for_the_slot" => Some(py_func("_a_word_for_the_slot", |args: &[Py]| _a_word_for_the_slot(args))),
            "_about_the_verb" => Some(py_func("_about_the_verb", |args: &[Py]| _about_the_verb(args))),
            "_age_said_of_the_only_animal" => Some(py_func("_age_said_of_the_only_animal", |args: &[Py]| _age_said_of_the_only_animal(args))),
            "_age_the_named_pet" => Some(py_func("_age_the_named_pet", |args: &[Py]| _age_the_named_pet(args))),
            "_age_they_answered" => Some(py_func("_age_they_answered", |args: &[Py]| _age_they_answered(args))),
            "_an_ask_about_nothing" => Some(py_func("_an_ask_about_nothing", |args: &[Py]| _an_ask_about_nothing(args))),
            "_an_ask_about_someone_else" => Some(py_func("_an_ask_about_someone_else", |args: &[Py]| _an_ask_about_someone_else(args))),
            "_another_animal_is_named" => Some(py_func("_another_animal_is_named", |args: &[Py]| _another_animal_is_named(args))),
            "_asks_about_someone_else" => Some(py_func("_asks_about_someone_else", |args: &[Py]| _asks_about_someone_else(args))),
            "_asks_to_hear_it_again" => Some(py_func("_asks_to_hear_it_again", |args: &[Py]| _asks_to_hear_it_again(args))),
            "_asks_whether_we_followed" => Some(py_func("_asks_whether_we_followed", |args: &[Py]| _asks_whether_we_followed(args))),
            "_budget_sign" => Some(py_func("_budget_sign", |args: &[Py]| _budget_sign(args))),
            "_concept" => Some(g__concept()),
            "_country" => Some(g__country()),
            "_diet_reply" => Some(py_func("_diet_reply", |args: &[Py]| _diet_reply(args))),
            "_drop_from_plan" => Some(py_func("_drop_from_plan", |args: &[Py]| _drop_from_plan(args))),
            "_drop_value" => Some(py_func("_drop_value", |args: &[Py]| _drop_value(args))),
            "_enough_reply" => Some(g__enough_reply()),
            "_fits" => Some(py_func("_fits", |args: &[Py]| _fits(args))),
            "_fix_a_persons_age" => Some(py_func("_fix_a_persons_age", |args: &[Py]| _fix_a_persons_age(args))),
            "_fix_a_persons_name" => Some(py_func("_fix_a_persons_name", |args: &[Py]| _fix_a_persons_name(args))),
            "_fix_a_pets_age" => Some(py_func("_fix_a_pets_age", |args: &[Py]| _fix_a_pets_age(args))),
            "_fix_a_pets_name" => Some(py_func("_fix_a_pets_name", |args: &[Py]| _fix_a_pets_name(args))),
            "_fix_the_make_of_a_thing" => Some(py_func("_fix_the_make_of_a_thing", |args: &[Py]| _fix_the_make_of_a_thing(args))),
            "_fix_the_weeks_of_a_pregnancy" => Some(py_func("_fix_the_weeks_of_a_pregnancy", |args: &[Py]| _fix_the_weeks_of_a_pregnancy(args))),
            "_forget_denied" => Some(py_func("_forget_denied", |args: &[Py]| _forget_denied(args))),
            "_last_stated" => Some(py_func("_last_stated", |args: &[Py]| _last_stated(args))),
            "_losses_taking" => Some(py_func("_losses_taking", |args: &[Py]| _losses_taking(args))),
            "_name_the_last_pet" => Some(py_func("_name_the_last_pet", |args: &[Py]| _name_the_last_pet(args))),
            "_names_a_pet_here" => Some(py_func("_names_a_pet_here", |args: &[Py]| _names_a_pet_here(args))),
            "_names_nothing_in_a_recall" => Some(py_func("_names_nothing_in_a_recall", |args: &[Py]| _names_nothing_in_a_recall(args))),
            "_names_this" => Some(py_func("_names_this", |args: &[Py]| _names_this(args))),
            "_note_a_corrected_fact" => Some(py_func("_note_a_corrected_fact", |args: &[Py]| _note_a_corrected_fact(args))),
            "_note_a_fact_moved" => Some(py_func("_note_a_fact_moved", |args: &[Py]| _note_a_fact_moved(args))),
            "_note_a_pet_that_is_new" => Some(py_func("_note_a_pet_that_is_new", |args: &[Py]| _note_a_pet_that_is_new(args))),
            "_observe" => Some(py_func("_observe", |args: &[Py]| _observe(args))),
            "_off_their_list" => Some(py_func("_off_their_list", |args: &[Py]| _off_their_list(args))),
            "_on_record_before" => Some(py_func("_on_record_before", |args: &[Py]| _on_record_before(args))),
            "_pets_of" => Some(py_func("_pets_of", |args: &[Py]| _pets_of(args))),
            "_plan_names" => Some(py_func("_plan_names", |args: &[Py]| _plan_names(args))),
            "_points_back_at_nothing" => Some(py_func("_points_back_at_nothing", |args: &[Py]| _points_back_at_nothing(args))),
            "_put_a_pets_age" => Some(py_func("_put_a_pets_age", |args: &[Py]| _put_a_pets_age(args))),
            "_reads_the_store" => Some(py_func("_reads_the_store", |args: &[Py]| _reads_the_store(args))),
            "_remember_dislikes" => Some(py_func("_remember_dislikes", |args: &[Py]| _remember_dislikes(args))),
            "_retract_last" => Some(py_func("_retract_last", |args: &[Py]| _retract_last(args))),
            "_stated_value" => Some(py_func("_stated_value", |args: &[Py]| _stated_value(args))),
            "_the_ages_of_the_children" => Some(py_func("_the_ages_of_the_children", |args: &[Py]| _the_ages_of_the_children(args))),
            "_the_animals_named" => Some(py_func("_the_animals_named", |args: &[Py]| _the_animals_named(args))),
            "_the_ask_about_how_it_went" => Some(py_func("_the_ask_about_how_it_went", |args: &[Py]| _the_ask_about_how_it_went(args))),
            "_the_ask_behind_the_recall" => Some(py_func("_the_ask_behind_the_recall", |args: &[Py]| _the_ask_behind_the_recall(args))),
            "_the_back_on_the_plate" => Some(py_func("_the_back_on_the_plate", |args: &[Py]| _the_back_on_the_plate(args))),
            "_the_clause" => Some(py_func("_the_clause", |args: &[Py]| _the_clause(args))),
            "_the_clock_in_their_line" => Some(py_func("_the_clock_in_their_line", |args: &[Py]| _the_clock_in_their_line(args))),
            "_the_count_was_the_line_before" => Some(py_func("_the_count_was_the_line_before", |args: &[Py]| _the_count_was_the_line_before(args))),
            "_the_diet_they_are_back_on" => Some(py_func("_the_diet_they_are_back_on", |args: &[Py]| _the_diet_they_are_back_on(args))),
            "_the_figure_said" => Some(py_func("_the_figure_said", |args: &[Py]| _the_figure_said(args))),
            "_the_figure_the_answer_names" => Some(py_func("_the_figure_the_answer_names", |args: &[Py]| _the_figure_the_answer_names(args))),
            "_the_figures_in" => Some(py_func("_the_figures_in", |args: &[Py]| _the_figures_in(args))),
            "_the_group_size_ask" => Some(py_func("_the_group_size_ask", |args: &[Py]| _the_group_size_ask(args))),
            "_the_last_ask_about_the_bot" => Some(py_func("_the_last_ask_about_the_bot", |args: &[Py]| _the_last_ask_about_the_bot(args))),
            "_the_last_line" => Some(py_func("_the_last_line", |args: &[Py]| _the_last_line(args))),
            "_the_money_figures" => Some(py_func("_the_money_figures", |args: &[Py]| _the_money_figures(args))),
            "_the_name_it_had" => Some(py_func("_the_name_it_had", |args: &[Py]| _the_name_it_had(args))),
            "_the_name_they_gave_to_the_person_they_mean" => Some(py_func("_the_name_they_gave_to_the_person_they_mean", |args: &[Py]| _the_name_they_gave_to_the_person_they_mean(args))),
            "_the_one_they_are_back_on" => Some(py_func("_the_one_they_are_back_on", |args: &[Py]| _the_one_they_are_back_on(args))),
            "_the_part_that_hurts_asked" => Some(py_func("_the_part_that_hurts_asked", |args: &[Py]| _the_part_that_hurts_asked(args))),
            "_the_pets_on_record" => Some(py_func("_the_pets_on_record", |args: &[Py]| _the_pets_on_record(args))),
            "_the_places_are_written_as_names" => Some(py_func("_the_places_are_written_as_names", |args: &[Py]| _the_places_are_written_as_names(args))),
            "_the_plan_they_took_back" => Some(py_func("_the_plan_they_took_back", |args: &[Py]| _the_plan_they_took_back(args))),
            "_the_store_readings" => Some(py_func("_the_store_readings", |args: &[Py]| _the_store_readings(args))),
            "_the_take_back_about_someone" => Some(py_func("_the_take_back_about_someone", |args: &[Py]| _the_take_back_about_someone(args))),
            "_the_time_they_put_right" => Some(py_func("_the_time_they_put_right", |args: &[Py]| _the_time_they_put_right(args))),
            "_the_turn_before_said" => Some(py_func("_the_turn_before_said", |args: &[Py]| _the_turn_before_said(args))),
            "_the_unit_asked" => Some(py_func("_the_unit_asked", |args: &[Py]| _the_unit_asked(args))),
            "_their_fact_ask" => Some(py_func("_their_fact_ask", |args: &[Py]| _their_fact_ask(args))),
            "_their_name_answered" => Some(py_func("_their_name_answered", |args: &[Py]| _their_name_answered(args))),
            "_their_own_line_ask" => Some(py_func("_their_own_line_ask", |args: &[Py]| _their_own_line_ask(args))),
            "_this_line_put_something_right" => Some(py_func("_this_line_put_something_right", |args: &[Py]| _this_line_put_something_right(args))),
            "_two_of_the_animal_are_kept" => Some(py_func("_two_of_the_animal_are_kept", |args: &[Py]| _two_of_the_animal_are_kept(args))),
            "_with_repaired" => Some(py_func("_with_repaired", |args: &[Py]| _with_repaired(args))),
            "_without_a_lead_in" => Some(py_func("_without_a_lead_in", |args: &[Py]| _without_a_lead_in(args))),
            "_without_the_shrug_at_the_end" => Some(py_func("_without_the_shrug_at_the_end", |args: &[Py]| _without_the_shrug_at_the_end(args))),
            "_word" => Some(g__word()),
            "_words" => Some(g__words()),
            "after" => Some(py_func("after", |args: &[Py]| after(args))),
            "note_the_hours" => Some(py_func("note_the_hours", |args: &[Py]| note_the_hours(args))),
            "observe" => Some(py_func("observe", |args: &[Py]| observe(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}

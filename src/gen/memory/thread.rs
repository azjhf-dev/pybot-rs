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


pub static ADDRESSEE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ADVERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CHOICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CALENDAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHECK_A_FACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHECK_VERDICT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOSINGS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLOSING_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONTRACTIONS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static COPULA: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETAIL_OPENER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DETERMINERS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DONE_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DROPPED_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPAND: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FACT_FRAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_PERSON_AUX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FLIP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAD_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HAPPENED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IS_NOT_POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IT_WAS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LOST_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MEASURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MODAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEVER_A_THING_THAT_WENT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NON_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_FACT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_FACT_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_FRAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_USED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NUMBER_WORD_HEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_TABLE_MISSES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ON_THE_TABLE_TURNS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PLACE_NAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POSSESSIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREDICATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PREP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static QUALITY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECEIPT_LONGEST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RECEIPT_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SECOND_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SIGNS_OFF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STATE_FILLER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBJECTISH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBJECT_YOU: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THING_PARTICLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THIRD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_BEFORE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERB_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static VERDICT_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WEAK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_FRAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_FRAMES_NO_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_TOLD_BAD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WENT_TOLD_WELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_I_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YES_FRAMES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YES_USED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DESCRIPTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_NAME_FOR_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_STATE_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_THING_DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ONLY_THE_ARRIVAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _WENT_WELL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ADDRESSEE() -> Py { ensure_init(); ADDRESSEE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADDRESSEE(v: Py) { ensure_init(); if let Ok(mut w) = ADDRESSEE.write() { *w = v; } }
pub fn g_ADVERB() -> Py { ensure_init(); ADVERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ADVERB(v: Py) { ensure_init(); if let Ok(mut w) = ADVERB.write() { *w = v; } }
pub fn g_ASK_VERB() -> Py { ensure_init(); ASK_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_VERB(v: Py) { ensure_init(); if let Ok(mut w) = ASK_VERB.write() { *w = v; } }
pub fn g_ASK_WORD() -> Py { ensure_init(); ASK_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_WORD(v: Py) { ensure_init(); if let Ok(mut w) = ASK_WORD.write() { *w = v; } }
pub fn g_ASK_WORDS() -> Py { ensure_init(); ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = ASK_WORDS.write() { *w = v; } }
pub fn g_AUX() -> Py { ensure_init(); AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AUX(v: Py) { ensure_init(); if let Ok(mut w) = AUX.write() { *w = v; } }
pub fn g_A_CHOICE() -> Py { ensure_init(); A_CHOICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CHOICE(v: Py) { ensure_init(); if let Ok(mut w) = A_CHOICE.write() { *w = v; } }
pub fn g_A_VERDICT() -> Py { ensure_init(); A_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = A_VERDICT.write() { *w = v; } }
pub fn g_BARE_VERBS() -> Py { ensure_init(); BARE_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = BARE_VERBS.write() { *w = v; } }
pub fn g_CALENDAR() -> Py { ensure_init(); CALENDAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CALENDAR(v: Py) { ensure_init(); if let Ok(mut w) = CALENDAR.write() { *w = v; } }
pub fn g_CHECK_A_FACT() -> Py { ensure_init(); CHECK_A_FACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHECK_A_FACT(v: Py) { ensure_init(); if let Ok(mut w) = CHECK_A_FACT.write() { *w = v; } }
pub fn g_CHECK_VERDICT() -> Py { ensure_init(); CHECK_VERDICT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHECK_VERDICT(v: Py) { ensure_init(); if let Ok(mut w) = CHECK_VERDICT.write() { *w = v; } }
pub fn g_CLOSINGS() -> Py { ensure_init(); CLOSINGS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOSINGS(v: Py) { ensure_init(); if let Ok(mut w) = CLOSINGS.write() { *w = v; } }
pub fn g_CLOSING_FILLER() -> Py { ensure_init(); CLOSING_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLOSING_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = CLOSING_FILLER.write() { *w = v; } }
pub fn g_CONTRACTIONS() -> Py { ensure_init(); CONTRACTIONS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONTRACTIONS(v: Py) { ensure_init(); if let Ok(mut w) = CONTRACTIONS.write() { *w = v; } }
pub fn g_COPULA() -> Py { ensure_init(); COPULA.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); if let Ok(mut w) = COPULA.write() { *w = v; } }
pub fn g_DETAIL_OPENER() -> Py { ensure_init(); DETAIL_OPENER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETAIL_OPENER(v: Py) { ensure_init(); if let Ok(mut w) = DETAIL_OPENER.write() { *w = v; } }
pub fn g_DETERMINERS() -> Py { ensure_init(); DETERMINERS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); if let Ok(mut w) = DETERMINERS.write() { *w = v; } }
pub fn g_DONE_WORDS() -> Py { ensure_init(); DONE_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DONE_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = DONE_WORDS.write() { *w = v; } }
pub fn g_DROPPED_SUBJECT() -> Py { ensure_init(); DROPPED_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DROPPED_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = DROPPED_SUBJECT.write() { *w = v; } }
pub fn g_EXPAND() -> Py { ensure_init(); EXPAND.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPAND(v: Py) { ensure_init(); if let Ok(mut w) = EXPAND.write() { *w = v; } }
pub fn g_FACT_FRAMES() -> Py { ensure_init(); FACT_FRAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FACT_FRAMES(v: Py) { ensure_init(); if let Ok(mut w) = FACT_FRAMES.write() { *w = v; } }
pub fn g_FIRST_PERSON_AUX() -> Py { ensure_init(); FIRST_PERSON_AUX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON_AUX(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_PERSON_AUX.write() { *w = v; } }
pub fn g_FLIP() -> Py { ensure_init(); FLIP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FLIP(v: Py) { ensure_init(); if let Ok(mut w) = FLIP.write() { *w = v; } }
pub fn g_HAD_IT() -> Py { ensure_init(); HAD_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAD_IT(v: Py) { ensure_init(); if let Ok(mut w) = HAD_IT.write() { *w = v; } }
pub fn g_HAPPENED() -> Py { ensure_init(); HAPPENED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HAPPENED(v: Py) { ensure_init(); if let Ok(mut w) = HAPPENED.write() { *w = v; } }
pub fn g_IS_NOT_POSSESSIVE() -> Py { ensure_init(); IS_NOT_POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IS_NOT_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = IS_NOT_POSSESSIVE.write() { *w = v; } }
pub fn g_IT_WAS() -> Py { ensure_init(); IT_WAS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IT_WAS(v: Py) { ensure_init(); if let Ok(mut w) = IT_WAS.write() { *w = v; } }
pub fn g_LEAD() -> Py { ensure_init(); LEAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); if let Ok(mut w) = LEAD.write() { *w = v; } }
pub fn g_LOST_SUBJECT() -> Py { ensure_init(); LOST_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LOST_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = LOST_SUBJECT.write() { *w = v; } }
pub fn g_MEASURE() -> Py { ensure_init(); MEASURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MEASURE(v: Py) { ensure_init(); if let Ok(mut w) = MEASURE.write() { *w = v; } }
pub fn g_MODAL() -> Py { ensure_init(); MODAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MODAL(v: Py) { ensure_init(); if let Ok(mut w) = MODAL.write() { *w = v; } }
pub fn g_NEVER_A_THING_THAT_WENT() -> Py { ensure_init(); NEVER_A_THING_THAT_WENT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEVER_A_THING_THAT_WENT(v: Py) { ensure_init(); if let Ok(mut w) = NEVER_A_THING_THAT_WENT.write() { *w = v; } }
pub fn g_NON_NOUN() -> Py { ensure_init(); NON_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NON_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = NON_NOUN.write() { *w = v; } }
pub fn g_NOT_A_FACT() -> Py { ensure_init(); NOT_A_FACT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_FACT.write() { *w = v; } }
pub fn g_NOT_A_THING() -> Py { ensure_init(); NOT_A_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_THING.write() { *w = v; } }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); NOT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = NOT_THE_THING.write() { *w = v; } }
pub fn g_NO_FACT_SAID() -> Py { ensure_init(); NO_FACT_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_FACT_SAID(v: Py) { ensure_init(); if let Ok(mut w) = NO_FACT_SAID.write() { *w = v; } }
pub fn g_NO_FRAMES() -> Py { ensure_init(); NO_FRAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_FRAMES(v: Py) { ensure_init(); if let Ok(mut w) = NO_FRAMES.write() { *w = v; } }
pub fn g_NO_USED() -> Py { ensure_init(); NO_USED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_USED(v: Py) { ensure_init(); if let Ok(mut w) = NO_USED.write() { *w = v; } }
pub fn g_NUMBER() -> Py { ensure_init(); NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER.write() { *w = v; } }
pub fn g_NUMBER_WORD_HEADS() -> Py { ensure_init(); NUMBER_WORD_HEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORD_HEADS(v: Py) { ensure_init(); if let Ok(mut w) = NUMBER_WORD_HEADS.write() { *w = v; } }
pub fn g_ON_THE_TABLE_MISSES() -> Py { ensure_init(); ON_THE_TABLE_MISSES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_TABLE_MISSES(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_TABLE_MISSES.write() { *w = v; } }
pub fn g_ON_THE_TABLE_TURNS() -> Py { ensure_init(); ON_THE_TABLE_TURNS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ON_THE_TABLE_TURNS(v: Py) { ensure_init(); if let Ok(mut w) = ON_THE_TABLE_TURNS.write() { *w = v; } }
pub fn g_PAST() -> Py { ensure_init(); PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST(v: Py) { ensure_init(); if let Ok(mut w) = PAST.write() { *w = v; } }
pub fn g_PLACE_NAMES() -> Py { ensure_init(); PLACE_NAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PLACE_NAMES(v: Py) { ensure_init(); if let Ok(mut w) = PLACE_NAMES.write() { *w = v; } }
pub fn g_POINTING_WORD() -> Py { ensure_init(); POINTING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = POINTING_WORD.write() { *w = v; } }
pub fn g_POINTS_BACK() -> Py { ensure_init(); POINTS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_BACK.write() { *w = v; } }
pub fn g_POSSESSIVE() -> Py { ensure_init(); POSSESSIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); if let Ok(mut w) = POSSESSIVE.write() { *w = v; } }
pub fn g_PREDICATE() -> Py { ensure_init(); PREDICATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREDICATE(v: Py) { ensure_init(); if let Ok(mut w) = PREDICATE.write() { *w = v; } }
pub fn g_PREP() -> Py { ensure_init(); PREP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PREP(v: Py) { ensure_init(); if let Ok(mut w) = PREP.write() { *w = v; } }
pub fn g_PRONOUN() -> Py { ensure_init(); PRONOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUN.write() { *w = v; } }
pub fn g_QUALITY() -> Py { ensure_init(); QUALITY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_QUALITY(v: Py) { ensure_init(); if let Ok(mut w) = QUALITY.write() { *w = v; } }
pub fn g_RECEIPT_LONGEST() -> Py { ensure_init(); RECEIPT_LONGEST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECEIPT_LONGEST(v: Py) { ensure_init(); if let Ok(mut w) = RECEIPT_LONGEST.write() { *w = v; } }
pub fn g_RECEIPT_WORDS() -> Py { ensure_init(); RECEIPT_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RECEIPT_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = RECEIPT_WORDS.write() { *w = v; } }
pub fn g_SECOND_PERSON() -> Py { ensure_init(); SECOND_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SECOND_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = SECOND_PERSON.write() { *w = v; } }
pub fn g_SIGNS_OFF() -> Py { ensure_init(); SIGNS_OFF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SIGNS_OFF(v: Py) { ensure_init(); if let Ok(mut w) = SIGNS_OFF.write() { *w = v; } }
pub fn g_STATE_FILLER() -> Py { ensure_init(); STATE_FILLER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STATE_FILLER(v: Py) { ensure_init(); if let Ok(mut w) = STATE_FILLER.write() { *w = v; } }
pub fn g_SUBJECTISH() -> Py { ensure_init(); SUBJECTISH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBJECTISH(v: Py) { ensure_init(); if let Ok(mut w) = SUBJECTISH.write() { *w = v; } }
pub fn g_SUBJECT_YOU() -> Py { ensure_init(); SUBJECT_YOU.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_YOU(v: Py) { ensure_init(); if let Ok(mut w) = SUBJECT_YOU.write() { *w = v; } }
pub fn g_THIN() -> Py { ensure_init(); THIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THIN(v: Py) { ensure_init(); if let Ok(mut w) = THIN.write() { *w = v; } }
pub fn g_THING_END() -> Py { ensure_init(); THING_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_END(v: Py) { ensure_init(); if let Ok(mut w) = THING_END.write() { *w = v; } }
pub fn g_THING_PARTICLE() -> Py { ensure_init(); THING_PARTICLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THING_PARTICLE(v: Py) { ensure_init(); if let Ok(mut w) = THING_PARTICLE.write() { *w = v; } }
pub fn g_THIRD() -> Py { ensure_init(); THIRD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THIRD(v: Py) { ensure_init(); if let Ok(mut w) = THIRD.write() { *w = v; } }
pub fn g_VERB_BEFORE() -> Py { ensure_init(); VERB_BEFORE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_BEFORE(v: Py) { ensure_init(); if let Ok(mut w) = VERB_BEFORE.write() { *w = v; } }
pub fn g_VERB_WORDS() -> Py { ensure_init(); VERB_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERB_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = VERB_WORDS.write() { *w = v; } }
pub fn g_VERDICT_SAID() -> Py { ensure_init(); VERDICT_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_VERDICT_SAID(v: Py) { ensure_init(); if let Ok(mut w) = VERDICT_SAID.write() { *w = v; } }
pub fn g_WEAK() -> Py { ensure_init(); WEAK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WEAK(v: Py) { ensure_init(); if let Ok(mut w) = WEAK.write() { *w = v; } }
pub fn g_WENT_AGAIN() -> Py { ensure_init(); WENT_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = WENT_AGAIN.write() { *w = v; } }
pub fn g_WENT_ASK() -> Py { ensure_init(); WENT_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_ASK(v: Py) { ensure_init(); if let Ok(mut w) = WENT_ASK.write() { *w = v; } }
pub fn g_WENT_FRAMES() -> Py { ensure_init(); WENT_FRAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_FRAMES(v: Py) { ensure_init(); if let Ok(mut w) = WENT_FRAMES.write() { *w = v; } }
pub fn g_WENT_FRAMES_NO_THING() -> Py { ensure_init(); WENT_FRAMES_NO_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_FRAMES_NO_THING(v: Py) { ensure_init(); if let Ok(mut w) = WENT_FRAMES_NO_THING.write() { *w = v; } }
pub fn g_WENT_TOLD_BAD() -> Py { ensure_init(); WENT_TOLD_BAD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_TOLD_BAD(v: Py) { ensure_init(); if let Ok(mut w) = WENT_TOLD_BAD.write() { *w = v; } }
pub fn g_WENT_TOLD_WELL() -> Py { ensure_init(); WENT_TOLD_WELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WENT_TOLD_WELL(v: Py) { ensure_init(); if let Ok(mut w) = WENT_TOLD_WELL.write() { *w = v; } }
pub fn g_WH() -> Py { ensure_init(); WH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WH(v: Py) { ensure_init(); if let Ok(mut w) = WH.write() { *w = v; } }
pub fn g_WHAT_I_SAID() -> Py { ensure_init(); WHAT_I_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_I_SAID.write() { *w = v; } }
pub fn g_YES_FRAMES() -> Py { ensure_init(); YES_FRAMES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YES_FRAMES(v: Py) { ensure_init(); if let Ok(mut w) = YES_FRAMES.write() { *w = v; } }
pub fn g_YES_USED() -> Py { ensure_init(); YES_USED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YES_USED(v: Py) { ensure_init(); if let Ok(mut w) = YES_USED.write() { *w = v; } }
pub fn g__A_DESCRIPTION() -> Py { ensure_init(); _A_DESCRIPTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DESCRIPTION(v: Py) { ensure_init(); if let Ok(mut w) = _A_DESCRIPTION.write() { *w = v; } }
pub fn g__A_NAME_FOR_IT() -> Py { ensure_init(); _A_NAME_FOR_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_NAME_FOR_IT(v: Py) { ensure_init(); if let Ok(mut w) = _A_NAME_FOR_IT.write() { *w = v; } }
pub fn g__A_STATE_ITSELF() -> Py { ensure_init(); _A_STATE_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_STATE_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = _A_STATE_ITSELF.write() { *w = v; } }
pub fn g__A_THING_DONE() -> Py { ensure_init(); _A_THING_DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_THING_DONE(v: Py) { ensure_init(); if let Ok(mut w) = _A_THING_DONE.write() { *w = v; } }
pub fn g__ONLY_THE_ARRIVAL() -> Py { ensure_init(); _ONLY_THE_ARRIVAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ONLY_THE_ARRIVAL(v: Py) { ensure_init(); if let Ok(mut w) = _ONLY_THE_ARRIVAL.write() { *w = v; } }
pub fn g__WENT_WELL() -> Py { ensure_init(); _WENT_WELL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__WENT_WELL(v: Py) { ensure_init(); if let Ok(mut w) = _WENT_WELL.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The thread of the conversation: what we are on, and how to say the user's earlier lines.\n\nThe conversation lane owns this. A module that answers a question about the conversation\nitself (\"where were we?\", \"what did I just ask you?\") needs two things that no other module\nprovides:\n\n* `subject(memory)`: the thing the last few turns were about, in a phrase the bot can name\n  (\"the birthday cake\", \"the chicken\", \"Lisbon\"), read from the user's own lines.\n* `restate(text)`: the user's earlier line said again inside a sentence, in their words and\n  in grammar that fits (\"can i make it the day before\" -> \"whether you can make it the day\n  before\"; \"how long should i cook a 2kg chicken\" -> \"how long you should cook a 2kg\n  chicken\"). First person becomes second person, the question is embedded, nothing is\n  invented.\n\nBoth read `memory[\"turns\"]` and the shared reading of each line (`bot.understanding`), so they\nwork for any topic and any wording. Nothing here writes to memory or answers a turn.\n");
    s_THIN(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("k"), py_str("cool"), py_str("nice"), py_str("great"), py_str("good"), py_str("yeah"), py_str("yep"), py_str("yes"), py_str("no"), py_str("nah"), py_str("nope"), py_str("lol"), py_str("haha"), py_str("ha"), py_str("mm"), py_str("hmm"), py_str("right"), py_str("fair"), py_str("sure"), py_str("same"), py_str("true"), py_str("thanks"), py_str("thx"), py_str("ta"), py_str("cheers"), py_str("please")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("night"), py_str("nite"), py_str("goodnight"), py_str("goodnite"), py_str("nighty"), py_str("bed"), py_str("bedtime"), py_str("bye"), py_str("byebye"), py_str("cya"), py_str("sleep")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSING_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("right"), py_str("ok"), py_str("okay"), py_str("well"), py_str("anyway"), py_str("im"), py_str("i"), py_str("am"), py_str("we"), py_str("were"), py_str("re"), py_str("going"), py_str("gonna"), py_str("gotta"), py_str("off"), py_str("to"), py_str("the"), py_str("a"), py_str("now"), py_str("so"), py_str("then"), py_str("just"), py_str("time"), py_str("for"), py_str("up")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGNS_OFF(stdlib_call("re", "compile", &[add(&add(&add(&add(&py_str("^(?:(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CLOSING_FILLER()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s+)*(?:")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CLOSINGS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEAK(call_builtin("frozenset", &[py_tuple(vec![py_str("name"), py_str("time"), py_str("day"), py_str("week"), py_str("weekend"), py_str("month"), py_str("year"), py_str("thing"), py_str("lot"), py_str("bit"), py_str("way"), py_str("kind"), py_str("sort"), py_str("part"), py_str("place"), py_str("trip"), py_str("plan"), py_str("idea"), py_str("question"), py_str("answer"), py_str("chat"), py_str("talk"), py_str("conversation"), py_str("help"), py_str("advice")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NON_NOUN(bitor(&bitor(&g_WEAK(), &g_THIN()), &call_builtin("frozenset", &[py_tuple(vec![py_str("before"), py_str("after"), py_str("again"), py_str("still"), py_str("just"), py_str("only"), py_str("maybe"), py_str("perhaps"), py_str("really"), py_str("nervous"), py_str("worried"), py_str("scared"), py_str("fine"), py_str("ok"), py_str("better"), py_str("worse"), py_str("best"), py_str("worst"), py_str("one"), py_str("ones"), py_str("two"), py_str("three"), py_str("first"), py_str("last"), py_str("next"), py_str("ask"), py_str("asked"), py_str("say"), py_str("said"), py_str("want"), py_str("need"), py_str("try"), py_str("trying"), py_str("started"), py_str("start"), py_str("stop"), py_str("make"), py_str("made"), py_str("get"), py_str("getting"), py_str("think"), py_str("thought"), py_str("going"), py_str("went"), py_str("come"), py_str("came"), py_str("much"), py_str("many"), py_str("more"), py_str("less"), py_str("all"), py_str("any"), py_str("some"), py_str("else"), py_str("something"), py_str("anything"), py_str("everything"), py_str("nothing"), py_str("remember"), py_str("recall"), py_str("forget"), py_str("said"), py_str("saying"), py_str("talked"), py_str("covered"), py_str("discussed"), py_str("meant"), py_str("rid")])]).unwrap_or_else(|e| exc::raise(e))));
    s_LEAD(py_tuple(vec![py_str("by the way"), py_str("btw"), py_str("anyway"), py_str("anyways"), py_str("so"), py_str("and"), py_str("but"), py_str("ok"), py_str("okay"), py_str("well"), py_str("oh"), py_str("ah"), py_str("right"), py_str("great"), py_str("sorry"), py_str("also"), py_str("then"), py_str("now"), py_str("ha"), py_str("hmm"), py_str("just"), py_str("cool"), py_str("nice"), py_str("yes"), py_str("yeah")]));
    s_MODAL(call_builtin("frozenset", &[py_tuple(vec![py_str("would"), py_str("should"), py_str("will"), py_str("can"), py_str("could"), py_str("may"), py_str("might"), py_str("must"), py_str("shall"), py_str("ought")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AUX(call_builtin("frozenset", &[py_tuple(vec![py_str("am"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("do"), py_str("does"), py_str("did"), py_str("have"), py_str("has"), py_str("had"), py_str("can"), py_str("could"), py_str("will"), py_str("would"), py_str("shall"), py_str("should"), py_str("may"), py_str("might"), py_str("must")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WH(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("whom"), py_str("whose"), py_str("which")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("it"), py_str("they"), py_str("he"), py_str("she"), py_str("this"), py_str("that"), py_str("there"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECTISH(bitor(&g_PRONOUN(), &call_builtin("frozenset", &[py_tuple(vec![py_str("people"), py_str("anyone"), py_str("someone"), py_str("everyone"), py_str("kids"), py_str("children")])]).unwrap_or_else(|e| exc::raise(e))));
    s_FLIP(py_dict(vec![(py_str("i"), py_str("you")), (py_str("me"), py_str("you")), (py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("myself"), py_str("yourself")), (py_str("we"), py_str("you")), (py_str("us"), py_str("you")), (py_str("our"), py_str("your")), (py_str("ours"), py_str("yours")), (py_str("ourselves"), py_str("yourselves")), (py_str("i'm"), py_str("you're")), (py_str("i've"), py_str("you've")), (py_str("i'd"), py_str("you'd")), (py_str("i'll"), py_str("you'll")), (py_str("we're"), py_str("you're")), (py_str("we've"), py_str("you've")), (py_str("we'd"), py_str("you'd")), (py_str("we'll"), py_str("you'll")), (py_str("im"), py_str("you're")), (py_str("ive"), py_str("you've")), (py_str("am"), py_str("are"))]));
    s_PLACE_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("place"), py_str("name")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR(call_builtin("frozenset", &[mcall(&py_str("today tomorrow yesterday tonight morning afternoon evening night week\nweekend month year hour minute second monday tuesday wednesday thursday friday saturday sunday\njanuary february march april may june july august september october november december\nspring summer autumn winter christmas easter holidays"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUALITY(bitor(&crate::gen::understanding::kinds::g_POSITIVE(), &crate::gen::understanding::kinds::g_NEGATIVE()));
    s_CONTRACTIONS(call_builtin("frozenset", &[py_tuple(vec![py_str("it's"), py_str("that's"), py_str("there's"), py_str("they're"), py_str("we're"), py_str("he's"), py_str("she's"), py_str("you're"), py_str("i'm"), py_str("i've"), py_str("i'd"), py_str("i'll"), py_str("there"), py_str("it"), py_str("this"), py_str("that"), py_str("they"), py_str("these"), py_str("those"), py_str("here")])]).unwrap_or_else(|e| exc::raise(e)));
    s__ONLY_THE_ARRIVAL(stdlib_call("re", "compile", &[py_str("^(?:(?:hi|hiya|hey|hello|ok|okay|right|so|well|anyway)[,\\s]+)*(?:im|i am|iam|we are|were|we're)?\\s*back(?:\\s+(?:again|home|now|too|from\\s+(?:work|the\\s+\\w+|holiday|holidays|town|school|shopping|the\\s+shop|hospital|abroad|lunch|a\\s+break)))?[\\s.!?]*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_RECEIPT_WORDS(call_builtin("frozenset", &[mcall(&py_str("\ngot it alright then will do ok okay right noted thanks thank you ta cheers sure cool great\nnice one lovely perfect brilliant grand fine fair enough no worries problem understood\nmakes sense course yes yeah yep yup mhm okay-doke done\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_RECEIPT_LONGEST(Py::Int(4));
    s_POINTS_BACK(stdlib_call("re", "compile", &[py_str("^(?:and |so |but |or |ok |okay |well |no |nah |yes |yeah |right |actually |sorry |please|how about |what about )*(?:(?:the )?same(?: one| thing)?(?: as (?:before|last time|last night|last week|last year|yesterday|the other day|the usual|usual|that|then|them|it|earlier))?|(?:just )?like (?:before|last time|last night|last week|yesterday|the usual|that|them|it)|(?:anything|something|what) else like (?:that|before|it|them)|the usual|as (?:we|i) (?:said|discussed|agreed)|(?:the )?(?:other|second|first|last) one|(?:that|this) one|(?:the )?(?:same|other) thing|(?:do |try |use )?what (?:you|we) said|the thing (?:you|we) said|(?:sorry )?(?:i|we) meant (?:the )?(?:other|that)(?: one| thing)?)(?: (?:then|please|again|really|though|now|too|instead))*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING_WORD(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|this|them|those|these|one|ones|there)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WORD(call_builtin("frozenset", &[mcall(&py_str("how what when where why which who whose do does did doing done can could\nshould would will shall am is are was were be been being have has had may may might must i you we\nme my our your it that this them those these there one ones a an the and or but so then again\nplease about for of in on at to with from much many long soon often far time worth better best\ngood right now just really any ok okay well hey hmm hm oh ah sorry up down off out back\naway over around through"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_VERB(call_builtin("frozenset", &[mcall(&py_str("do does did make makes get gets take takes cost costs try use uses go goes\nwork works need needs start starts keep keeps put puts cook cooks buy buys fix fixes help helps\nworth better eat eats drink drinks say says"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CHOICE(stdlib_call("re", "compile", &[py_str("\\bwhich\\b[^?]*\\b(?:should|better|best|instead)\\b|\\bshould i\\b[^?]*\\b(?:buy|choose|pick|go for)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_I_SAID(stdlib_call("re", "compile", &[py_str("\\bwhat (?:was|were) (?:it )?(?:i|we) (?:(?:was|were|am|are|had been) )?(?:worried|upset|angry|annoyed|sad|unhappy|thinking|saying)(?: (?:about|of))?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERB_BEFORE(bitor(&g_AUX(), &call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("they"), py_str("he"), py_str("she"), py_str("it"), py_str("not"), py_str("please"), py_str("lets"), py_str("let's"), py_str("who"), py_str("dont"), py_str("doesnt"), py_str("didnt"), py_str("cant"), py_str("wont"), py_str("wouldnt"), py_str("couldnt"), py_str("shouldnt")])]).unwrap_or_else(|e| exc::raise(e))));
    s_DETERMINERS(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("your"), py_str("our"), py_str("their"), py_str("his"), py_str("her"), py_str("its"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("some"), py_str("any"), py_str("many"), py_str("much"), py_str("no"), py_str("every"), py_str("each")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_VERBS(call_builtin("frozenset", &[mcall(&py_str("do give get go try keep sort call tell know feel be been have wait\n    stop start find leave come put use take see say think love like want look check help send\n    make book buy fix sort plan"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORD_HEADS(call_builtin("frozenset", &[mcall(&py_str("\nzero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen\nsixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety\nhundred thousand million\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WORDS(py_dict(vec![(py_str("how_long"), py_tuple(vec![py_str("how"), py_str("long")])), (py_str("how_much"), py_tuple(vec![py_str("how"), py_str("much")])), (py_str("how_many"), py_tuple(vec![py_str("how"), py_str("many")])), (py_str("how_far"), py_tuple(vec![py_str("how"), py_str("far")])), (py_str("how_often"), py_tuple(vec![py_str("how"), py_str("often")]))]));
    s_ADDRESSEE(py_dict(vec![(py_str("you"), py_str("me")), (py_str("your"), py_str("my")), (py_str("yours"), py_str("mine")), (py_str("yourself"), py_str("myself")), (py_str("you're"), py_str("i am")), (py_str("youve"), py_str("i have")), (py_str("you've"), py_str("i have")), (py_str("you'd"), py_str("i would")), (py_str("you'll"), py_str("i will"))]));
    s_SUBJECT_YOU(py_str("I"));
    s_LOST_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("cant"), py_str("can't"), py_str("couldnt"), py_str("couldn't"), py_str("dont"), py_str("don't"), py_str("didnt"), py_str("didn't"), py_str("wont"), py_str("won't"), py_str("havent"), py_str("haven't"), py_str("hadnt"), py_str("hadn't"), py_str("wouldnt"), py_str("wouldn't"), py_str("shouldnt"), py_str("shouldn't")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DROPPED_SUBJECT(bitor(&g_LOST_SUBJECT(), &call_builtin("frozenset", &[py_tuple(vec![py_str("am"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("will"), py_str("might")])]).unwrap_or_else(|e| exc::raise(e))));
    s_SECOND_PERSON(py_dict(vec![(py_str("am"), py_tuple(vec![py_str("you"), py_str("are")])), (py_str("is"), py_tuple(vec![py_str("you"), py_str("are")])), (py_str("are"), py_tuple(vec![py_str("you"), py_str("are")])), (py_str("was"), py_tuple(vec![py_str("you"), py_str("were")])), (py_str("were"), py_tuple(vec![py_str("you"), py_str("were")]))]));
    s_FIRST_PERSON_AUX(py_dict(vec![(py_str("are"), py_str("am")), (py_str("were"), py_str("was")), (py_str("is"), py_str("am")), (py_str("am"), py_str("am"))]));
    s_EXPAND(py_dict(vec![(py_str("i'm"), py_str("i am")), (py_str("im"), py_str("i am")), (py_str("i've"), py_str("i have")), (py_str("i'd"), py_str("i would")), (py_str("i'll"), py_str("i will")), (py_str("it's"), py_str("it is")), (py_str("that's"), py_str("that is")), (py_str("there's"), py_str("there is")), (py_str("what's"), py_str("what is")), (py_str("we're"), py_str("we are")), (py_str("they're"), py_str("they are")), (py_str("you're"), py_str("you are")), (py_str("he's"), py_str("he is")), (py_str("she's"), py_str("she is")), (py_str("don't"), py_str("do not")), (py_str("doesn't"), py_str("does not")), (py_str("didn't"), py_str("did not")), (py_str("can't"), py_str("cannot")), (py_str("won't"), py_str("will not")), (py_str("isn't"), py_str("is not")), (py_str("aren't"), py_str("are not")), (py_str("wasn't"), py_str("was not")), (py_str("weren't"), py_str("were not")), (py_str("haven't"), py_str("have not")), (py_str("hasn't"), py_str("has not")), (py_str("couldn't"), py_str("could not")), (py_str("wouldn't"), py_str("would not")), (py_str("shouldn't"), py_str("should not"))]));
    s_PREDICATE(call_builtin("frozenset", &[py_tuple(vec![py_str("safe"), py_str("ok"), py_str("okay"), py_str("normal"), py_str("fine"), py_str("good"), py_str("bad"), py_str("better"), py_str("worse"), py_str("worth"), py_str("hard"), py_str("easy"), py_str("difficult"), py_str("available"), py_str("allowed"), py_str("legal"), py_str("dangerous"), py_str("broken"), py_str("working"), py_str("fixed"), py_str("cheap"), py_str("cheaper"), py_str("expensive"), py_str("possible"), py_str("true"), py_str("wrong"), py_str("open"), py_str("closed"), py_str("included"), py_str("covered"), py_str("needed"), py_str("sick"), py_str("ill"), py_str("dead"), py_str("alive"), py_str("serious"), py_str("urgent"), py_str("done"), py_str("ready"), py_str("likely"), py_str("fair"), py_str("free"), py_str("late"), py_str("busy"), py_str("hot"), py_str("cold"), py_str("warm"), py_str("healthy"), py_str("insured"), py_str("enough")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PREP(call_builtin("frozenset", &[py_tuple(vec![py_str("to"), py_str("for"), py_str("in"), py_str("on"), py_str("at"), py_str("with"), py_str("from"), py_str("by"), py_str("about"), py_str("of"), py_str("than"), py_str("after"), py_str("before"), py_str("during"), py_str("into"), py_str("over"), py_str("under"), py_str("out"), py_str("up"), py_str("down"), py_str("when"), py_str("if"), py_str("because"), py_str("as"), py_str("so"), py_str("like"), py_str("near")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MEASURE(call_builtin("frozenset", &[py_tuple(vec![py_str("pound"), py_str("pounds"), py_str("quid"), py_str("euro"), py_str("euros"), py_str("dollar"), py_str("dollars"), py_str("cent"), py_str("cents"), py_str("penny"), py_str("kilo"), py_str("kilos"), py_str("kilogram"), py_str("kilograms"), py_str("gram"), py_str("grams"), py_str("ounces"), py_str("ounce"), py_str("stone"), py_str("litre"), py_str("litres"), py_str("liter"), py_str("liters"), py_str("mile"), py_str("miles"), py_str("kilometre"), py_str("kilometres"), py_str("kilometer"), py_str("kilometers"), py_str("percent"), py_str("degree"), py_str("degrees"), py_str("minute"), py_str("minutes"), py_str("hour"), py_str("hours")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ADVERB(call_builtin("frozenset", &[py_tuple(vec![py_str("just"), py_str("ever"), py_str("already"), py_str("actually"), py_str("really"), py_str("first"), py_str("originally"), py_str("previously"), py_str("recently"), py_str("still"), py_str("also"), py_str("always"), py_str("never"), py_str("even")])]).unwrap_or_else(|e| exc::raise(e)));
    s_POSSESSIVE(stdlib_call("re", "compile", &[py_str("([A-Za-z]+)(?:'s|\\u2019s)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_IS_NOT_POSSESSIVE(call_builtin("frozenset", &[mcall(&py_str("it that this there here what who he she one everyone nobody\nsomebody anyone"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER(stdlib_call("re", "compile", &[py_str("\\d[\\d,]*(?:\\.\\d+)?")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST(py_dict(vec![(py_str("say"), py_str("said")), (py_str("tell"), py_str("told")), (py_str("go"), py_str("went")), (py_str("get"), py_str("got")), (py_str("make"), py_str("made")), (py_str("take"), py_str("took")), (py_str("buy"), py_str("bought")), (py_str("think"), py_str("thought")), (py_str("see"), py_str("saw")), (py_str("do"), py_str("did")), (py_str("have"), py_str("had")), (py_str("eat"), py_str("ate")), (py_str("drink"), py_str("drank")), (py_str("come"), py_str("came")), (py_str("find"), py_str("found")), (py_str("give"), py_str("gave")), (py_str("know"), py_str("knew")), (py_str("leave"), py_str("left")), (py_str("pay"), py_str("paid")), (py_str("put"), py_str("put")), (py_str("read"), py_str("read")), (py_str("run"), py_str("ran")), (py_str("send"), py_str("sent")), (py_str("sit"), py_str("sat")), (py_str("speak"), py_str("spoke")), (py_str("spend"), py_str("spent")), (py_str("write"), py_str("wrote")), (py_str("fly"), py_str("flew")), (py_str("drive"), py_str("drove")), (py_str("meet"), py_str("met")), (py_str("hear"), py_str("heard")), (py_str("feel"), py_str("felt")), (py_str("keep"), py_str("kept")), (py_str("sleep"), py_str("slept")), (py_str("win"), py_str("won")), (py_str("lose"), py_str("lost")), (py_str("break"), py_str("broke")), (py_str("choose"), py_str("chose")), (py_str("begin"), py_str("began")), (py_str("become"), py_str("became")), (py_str("bring"), py_str("brought")), (py_str("build"), py_str("built")), (py_str("catch"), py_str("caught")), (py_str("cost"), py_str("cost")), (py_str("cut"), py_str("cut")), (py_str("fall"), py_str("fell")), (py_str("forget"), py_str("forgot")), (py_str("hold"), py_str("held")), (py_str("hit"), py_str("hit")), (py_str("hurt"), py_str("hurt")), (py_str("lend"), py_str("lent")), (py_str("let"), py_str("let")), (py_str("mean"), py_str("meant")), (py_str("sing"), py_str("sang")), (py_str("teach"), py_str("taught")), (py_str("understand"), py_str("understood")), (py_str("wear"), py_str("wore")), (py_str("wake"), py_str("woke"))]));
    s_THIRD(py_dict(vec![(py_str("have"), py_str("has")), (py_str("be"), py_str("is")), (py_str("do"), py_str("does")), (py_str("go"), py_str("goes")), (py_str("say"), py_str("says"))]));
    s_NOT_A_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("keep"), py_str("keeps"), py_str("kept"), py_str("do"), py_str("does"), py_str("did"), py_str("get"), py_str("gets"), py_str("got"), py_str("go"), py_str("goes"), py_str("going"), py_str("went"), py_str("want"), py_str("wants"), py_str("wanted"), py_str("need"), py_str("needs"), py_str("needed"), py_str("try"), py_str("tries"), py_str("tried"), py_str("make"), py_str("makes"), py_str("made"), py_str("start"), py_str("starts"), py_str("started"), py_str("stop"), py_str("stops"), py_str("say"), py_str("says"), py_str("said"), py_str("think"), py_str("thought"), py_str("happen"), py_str("happens"), py_str("happening"), py_str("thing"), py_str("things"), py_str("it"), py_str("something"), py_str("anything")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ON_THE_TABLE_TURNS(Py::Int(12));
    s_ON_THE_TABLE_MISSES(Py::Int(3));
    s_COPULA(call_builtin("frozenset", &[mcall(&py_str("im i'm am is are was were been being feel feels felt feeling look looks\n    seem seems get gets got getting become"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STATE_FILLER(call_builtin("frozenset", &[mcall(&py_str("a an the bit little very really so quite pretty too that this\n    honestly actually just totally proper still always kinda sorta dead well rather"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_VERB_WORDS(bitor(&crate::gen::understanding::kinds::g_VERB_FORMS(), &crate::gen::understanding::kinds::g_VERBISH()));
    s_DETAIL_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:its|it's|it is|thats|that's|that is|theyre|they're|they are|hes|he's|she's|shes)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECK_VERDICT(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:(?:is|will|would|does|do|was)\\s+(?:that|it|this)(?:\\s+be)?|(?:that|it|this)(?:'s|s|\\s+is)?)\\s+(?:alright|all right|ok|okay|fine|safe|allowed|work|works)(?:\\s+for (?:me|us|him|her|them))?(?:\\s+(?:though|then|now))?|^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:are|r)\\s+(?:you\\s+)?(?:really\\s+)?sure(?:\\s+(?:about|of)\\s+(?:that|it|this))?$|^(?:you\\s+)?(?:really\\s+)?sure(?:\\s+(?:about|of)\\s+(?:that|it|this))?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_SAID(stdlib_call("re", "compile", &[py_str("^(?:still\\s+(?:a\\s+)?)?(yes|no|afraid not|no, sorry|no, afraid not)\\s*[:\\u2014\\u2013-]\\s*(.+)$"), bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_FRAMES(py_tuple(vec![py_str("Afraid not — {reason}."), py_str("Still a no — {reason}."), py_str("No, sorry — {reason}.")]));
    s_YES_FRAMES(py_tuple(vec![py_str("Yes — {reason}."), py_str("Still yes — {reason}.")]));
    s_NO_USED(py_tuple(vec![py_tuple(vec![py_str("afraid not"), Py::Int(1)]), py_tuple(vec![py_str("still a no"), Py::Int(2)]), py_tuple(vec![py_str("no, sorry"), Py::Int(0)])]));
    s_YES_USED(py_tuple(vec![py_tuple(vec![py_str("yes —"), Py::Int(1)]), py_tuple(vec![py_str("still yes"), Py::Int(0)])]));
    s_CHECK_A_FACT(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then|thanks|ta|great|good|nice)[, ]+)*(?:(?:is|was|are)\\s+(?:that|it|this|they|those)(?:\\s+(?:really|actually|definitely|all|exactly|still))?\\s+(?:right|true|correct|accurate|the right (?:answer|thing)|how it works|what you meant)(?:\\s+(?:then|though|now|yeah|yes|or not|or what))?|(?:that|it|this)(?:'s|s|\\s+is)?\\s+(?:right|true|correct|accurate)|(?:right|correct|true))\\s*[?.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NO_FACT_SAID(stdlib_call("re", "compile", &[py_str("^(?:i'm not sure|i am not sure|i don't know|i do not know|i'd rather not guess|that's beyond what i know|not something i know)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FACT_FRAMES(py_tuple(vec![py_str("That's right."), py_str("Yes — that's the usual guidance."), py_str("That is the answer I have, yes."), py_str("That's how I understand it too."), py_str("Yes — that's the standard advice."), py_str("That's right, as far as I know.")]));
    s_NOT_A_FACT(stdlib_call("re", "compile", &[py_str("(?:^|\\b)(?:i'll remember|i will remember|noted\\b|got it\\b|alright then|sounds good|fair enough|i'm listening|say more|tell me more|back on the table)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WENT_ASK(stdlib_call("re", "compile", &[py_str("^(?:(?:so|and|then|but|well|ok|okay|right|oh|ah|hey|great|good|nice|hmm+|anyway|go on|aw+|yes|yeah)\\b[\\s,]*)*(?:how\\s+(?:did|does|has|have|was|were)?\\s*(?:it|that|things|everything)\\s*(?:go|gone|been|turn\\s+out|work\\s+out)?(?:\\s+(?:ok|okay|well|fine|alright|good|bad))?|how(?:'s|d|s)?\\s+(?:it|that)\\s+(?:go|gone|been|turn\\s+out)|how'd\\s+(?:it|that)\\s+go|did\\s+(?:it|that|things|everything)\\s+(?:go|turn\\s+out)(?:\\s+(?:ok|okay|well|fine|alright|good|bad))?|how\\s+(?:did|was|were)\\s+(?:the|your|my|that)\\s+(?P<thing>[a-z][a-z' -]{1,40}?)(?:\\s+(?:go|gone|turn\\s+out))?)(?:[\\s,]*(?:then|though|today|tonight|in the end|honestly|please|anyway|huh|so far))*[\\s?.!]*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THE_THING(call_builtin("frozenset", &[py_tuple(vec![py_str("day"), py_str("days"), py_str("week"), py_str("weeks"), py_str("weekend"), py_str("weekends"), py_str("evening"), py_str("evenings"), py_str("morning"), py_str("mornings"), py_str("afternoon"), py_str("night"), py_str("nights"), py_str("holiday"), py_str("holidays"), py_str("break"), py_str("time"), py_str("weather"), py_str("sleep")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NEVER_A_THING_THAT_WENT(call_builtin("frozenset", &[py_tuple(vec![py_str("baby"), py_str("birth"), py_str("stillbirth"), py_str("miscarriage"), py_str("loss"), py_str("death"), py_str("passing"), py_str("funeral"), py_str("news"), py_str("fall"), py_str("tumble"), py_str("panic"), py_str("attack"), py_str("seizure"), py_str("fit"), py_str("convulsion"), py_str("bleed"), py_str("bleeding"), py_str("symptom"), py_str("diagnosis"), py_str("result"), py_str("fever"), py_str("pain"), py_str("ache"), py_str("flu"), py_str("bug"), py_str("infection"), py_str("migraine"), py_str("headache"), py_str("period"), py_str("cramp"), py_str("rash"), py_str("shock"), py_str("scare"), py_str("fright")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_STATE_ITSELF(call_builtin("frozenset", &[py_tuple(vec![py_str("day"), py_str("week"), py_str("night"), py_str("morning"), py_str("afternoon"), py_str("evening"), py_str("weekend"), py_str("holiday"), py_str("break"), py_str("time"), py_str("cold"), py_str("temperature"), py_str("argument"), py_str("row"), py_str("cry"), py_str("crying"), py_str("upset"), py_str("tiredness")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_NAME_FOR_IT(call_builtin("frozenset", &[py_tuple(vec![py_str("first"), py_str("second"), py_str("third"), py_str("last"), py_str("next"), py_str("final"), py_str("new"), py_str("other")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DESCRIPTION(call_builtin("frozenset", &[py_tuple(vec![py_str("some"), py_str("any"), py_str("such"), py_str("really"), py_str("very"), py_str("quite"), py_str("rather"), py_str("proper"), py_str("bit"), py_str("lot"), py_str("few"), py_str("another"), py_str("little"), py_str("bad"), py_str("awful"), py_str("terrible"), py_str("rough"), py_str("dreadful"), py_str("horrible"), py_str("hard"), py_str("tough"), py_str("rotten")])]).unwrap_or_else(|e| exc::raise(e)));
    s_HAPPENED(stdlib_call("re", "compile", &[py_str("\\b(?:today|tonight|this\\s+(?:morning|afternoon|evening)|last\\s+night|yesterday|earlier)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_HAD_IT(stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:had|went\\s+(?:for|to|on)|did|took|sat|attended|started|finished|gave)\\s+(?:an|a|my|our|the)?\\s*(?P<thing>[a-z][a-z' -]{1,40})"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IT_WAS(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?P<thing>[a-z][a-z' -]{2,40}?)\\s+(?:was|were|is|are)(?:\\s+(?:today|tonight|yesterday|this\\s+[a-z]+|last\\s+night|earlier))?(?:\\s*[,;]?\\s*(?:and|but|so)\\b.*)?\\s*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_END(call_builtin("frozenset", &[py_tuple(vec![py_str("today"), py_str("tonight"), py_str("yesterday"), py_str("this"), py_str("last"), py_str("and"), py_str("but"), py_str("so"), py_str("then"), py_str("which"), py_str("because"), py_str("when"), py_str("where"), py_str("it"), py_str("that"), py_str("was"), py_str("were"), py_str("is"), py_str("at"), py_str("in"), py_str("on"), py_str("with"), py_str("for"), py_str("to"), py_str("after"), py_str("before"), py_str("earlier")])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_THING_DONE(call_builtin("frozenset", &[py_tuple(vec![py_str("go"), py_str("try"), py_str("look"), py_str("glance"), py_str("peek"), py_str("chat"), py_str("talk"), py_str("word"), py_str("think"), py_str("laugh"), py_str("cry"), py_str("moan"), py_str("nag"), py_str("row"), py_str("argument"), py_str("argue"), py_str("fight"), py_str("lie"), py_str("bath"), py_str("shower"), py_str("wash"), py_str("rest"), py_str("nap"), py_str("snooze"), py_str("kip"), py_str("sleep"), py_str("sit"), py_str("drink"), py_str("bite"), py_str("cuddle"), py_str("listen"), py_str("wobble"), py_str("turn"), py_str("moment"), py_str("blip"), py_str("dip")])]).unwrap_or_else(|e| exc::raise(e)));
    s_THING_PARTICLE(call_builtin("frozenset", &[py_tuple(vec![py_str("off"), py_str("out"), py_str("up"), py_str("down"), py_str("away"), py_str("back"), py_str("over"), py_str("round"), py_str("around"), py_str("through"), py_str("along"), py_str("about"), py_str("in"), py_str("on")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_WORDS(call_builtin("frozenset", &[py_tuple(vec![py_str("cut"), py_str("put"), py_str("set"), py_str("shut"), py_str("hit"), py_str("let"), py_str("cost"), py_str("hurt"), py_str("burst"), py_str("spread"), py_str("rid"), py_str("shed"), py_str("split"), py_str("stuck"), py_str("sent"), py_str("built"), py_str("frozen")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WENT_FRAMES(py_tuple(vec![py_str("So how did the {thing} go? You'd know before I would."), py_str("You tell me — how did the {thing} go?"), py_str("Go on then: how did the {thing} go?"), py_str("How did the {thing} go? I only know what you tell me.")]));
    s_WENT_FRAMES_NO_THING(py_tuple(vec![py_str("Then you'll have to tell me — how did it go?"), py_str("Go on then, how did it go?"), py_str("I can't see your day from here. How did it go?")]));
    s_WENT_AGAIN(py_tuple(vec![py_str("I've no way of seeing it from here — you'll have to tell me."), py_str("Still yours to tell me: how did it go?"), py_str("I can't see it from where I am, so tell me how it went.")]));
    s_A_VERDICT(stdlib_call("re", "compile", &[py_str("\\b(?:it|that|they)\\s+(?:was|were|is|are)\\s+(?P<v>fine|good|great|ok|okay|all right|alright|lovely|brilliant|awful|terrible|horrible|dreadful|rubbish|a disaster|a nightmare)|\\bwent\\s+(?P<w>well|fine|badly|great|ok|okay|awful|terrible)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__WENT_WELL(call_builtin("frozenset", &[py_tuple(vec![py_str("fine"), py_str("good"), py_str("great"), py_str("ok"), py_str("okay"), py_str("all right"), py_str("alright"), py_str("lovely"), py_str("brilliant"), py_str("well")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WENT_TOLD_WELL(py_tuple(vec![py_str("Good — you told me it was {v}. That's the main thing."), py_str("{V} is a relief, then. Tell me more if there's more in it."), py_str("So it went {v}. Nothing for me to ask there.")]));
    s_WENT_TOLD_BAD(py_tuple(vec![py_str("Sorry it went {v}. What happened?"), py_str("{V} — that's rough. Tell me the worst bit, if you want to."), py_str("That's a rotten one to have had. What went wrong?")]));
}
pub fn _asks_about_the_bot(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks about the bot itself rather than about something of their own.\n\n    \"Are you a real person\" is not a subject the chat was on, so it must not be named back as\n    one: on main the next \"what did we talk about\" answered \"We were on the real person\", a\n    phrase nobody said, cut out of a question about the bot. The reading is the understanding\n    engine's own (`about_bot`), so every shape of the question is covered rather than a list of\n    phrasings here: \"are you a robot\", \"are you human\", \"do you sleep\", \"what model are you\".\n    A line that asks about the bot and also says something of their own is not in this: the\n    reading only sets `about_bot` when the whole line is about the bot.\n    ");
    return call_builtin("bool", &[mcall(&reading.clone(), "get", &[py_str("about_bot")])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _asks_about_the_chat(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: skip"))) };
    let _ = py_str("True when the store says this line is about the conversation, not about the world.\n\n    The store owns that test (`memory_store._about_this_chat`), so it is passed in rather than\n    copied: a second list of the same phrasings would drift. Without it the reading's own\n    `topic_own`/`kind` still catches most of these lines.\n    ");
    return { let __b = call_builtin("bool", &[skip.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[py_call(&skip, &[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn _turns(args: &[Py]) -> Py {
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
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    return if py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { turns.clone() } else { py_list(vec![]) };
    Py::None
}
pub fn earlier_turn(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let _ = py_str("True when there is an earlier turn for a reply to point back at.\n\n    One check, in one place, for every reply that refers back to something said before: a\n    point back at turns that never happened is the same fault wherever it appears - the first\n    reply answering \"im back\" with \"What were we saying?\", a receipt for a question nobody\n    asked, a nod at a feeling no one expressed. Modules ask this rather than each testing\n    `memory` their own way, so a new pool cannot reintroduce the fault by forgetting to.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_turns(&[memory.clone()])) { let mut turn: Py = __item.clone(); __out.push({ let __b = call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { mcall(&turn.clone(), "get", &[py_str("reply")]) } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn only_the_arrival(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line says nothing but that the person is back.\n\n    A return carries no symptom, no subject and no question: \"im back\", \"back again\", \"back\n    from work\", \"we are back home\". A chat with a bad knee in it still read the word \"back\" as\n    the part and answered a return with the back-pain card, which is the message talking about\n    somebody's spine when they said they were back from the shop. The check is here, once, so\n    any row that reads a body word can ask it rather than each one learning the word order.\n    ");
    said = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "replace", &[py_str("'"), py_str("")]);
    said = stdlib_call("re", "sub", &[py_str("[.,!?]+"), py_str(" "), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("bool", &[mcall(&g__ONLY_THE_ARRIVAL(), "match", &[mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn user_lines(args: &[Py]) -> Py {
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
    let mut out: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let _ = py_str("The user's earlier lines, oldest first, as (text, reading).");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&_turns(&[memory.clone()]));
    for __item in __iter1 {
    turn = __item.clone();
    text = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&{ let __b = call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&text.clone(), "strip", &[]) } }) {
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![text.clone(), crate::gen::understanding::understand(&[text.clone(), py_dict(vec![])])])]);
    }
    }
    return out.clone();
    Py::None
}
pub fn _is_a_receipt(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the line only takes the reply in and says nothing else.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?-—–()")])); } py_list(__out) })() };
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); if py_truthy(&w.clone()) {  __out.push(mcall(&w.clone(), "lower", &[])); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RECEIPT_LONGEST()); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_RECEIPT_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _is_thin(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&_is_a_receipt(&[reading.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THIN(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn signs_off(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for a line that is only a closing: \"night\", \"im off to bed\", \"see ya\".\n\n    A closing ends a chat and does not move it on, so it is not what the chat was on: a chat\n    with a real line in it and two closings at the end was told \"there's nothing of yours to\n    look back on\", because the closings had been read as the whole of the last thing said.\n    ");
    reading = crate::gen::understanding::understand(&[text.clone(), py_dict(vec![])]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?")])); } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("intent")]), &py_str("farewell")))) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_SIGNS_OFF(), "match", &[mcall(&py_str(" "), "join", &[words.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn carries_subject(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("True when the line can be the thing the chat was on: \"hi\" and \"ok\" cannot.");
    said = { let __b = restate(&[text.clone()]); if py_truthy(&__b) { __b } else { py_str("") } };
    return { let __b = call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _is_subject_phrase(&[said.clone()]) } };
    Py::None
}
pub fn thin(args: &[Py]) -> Py {
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
    let mut reading: Py = Py::None;
    let _ = py_str("True for a line that carries no subject: \"hi\", \"ok\", \"lol\", \"thanks\".\n\n    A greeting is as thin as an \"ok\": \"hi\" and then \"what were we talking about?\" is not a\n    conversation about hi, and \"We were on hi.\" answers a subject nobody raised. A line that\n    asks about the conversation itself carries no subject either, for the same reason.\n    ");
    reading = crate::gen::understanding::understand(&[text.clone(), py_dict(vec![])]);
    return { let __b = _is_thin(&[reading.clone()]); if py_truthy(&__b) { __b } else { _is_memory_line(&[reading.clone()]) } };
    Py::None
}
pub fn points_back(args: &[Py]) -> Py {
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
    let _ = py_str("True for a line that names nothing of its own and points at what was said before.\n\n    The words are the whole line (\"and the other one then\"), because a line with a subject in\n    it is that subject's: \"the other flat\" names a flat, and \"same as the boiler\" names the\n    boiler. Nothing here is a question about the chat, which the store reads elsewhere.\n    ");
    low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    return { let __b = call_builtin("bool", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = le(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_POINTS_BACK(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn points_at_a_word_alone(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut low: Py = Py::None;
    let mut other: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True for an ask whose only subject is a pointing word: \"how do i do it\".\n\n    \"how do i do it\" as the first line of a chat has no thing in it to answer: the \"it\" points\n    at something the chat has not got, and the turn was answered with a search hedge (\"I'm not\n    sure about that one, sorry\") while the honest reply is that there is nothing for \"it\" to be\n    yet. The words outside the asking words are what is tested, so a line that names a thing of\n    its own (\"how do i fix a bike\", \"how long does a chicken take\") is not this, and neither is\n    \"what about that\", which is the pointer with no ask around it.\n    ");
    low = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    words = stdlib_call("re", "findall", &[py_str("[a-z]+"), low.clone()]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_POINTING_WORD(), "search", &[low.clone()]))) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_A_CHOICE(), "search", &[low.clone()])) {
    return Py::Bool(false);
    }
    other = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_ASK_WORD(), &word.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_ASK_VERB(), &word.clone())) } }) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&other.clone()))) {
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_ASK_VERB(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    if py_truthy(&gt(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    at = mcall(&words.clone(), "index", &[geti(&other.clone(), &Py::Int(0))]);
    return { let __b = lt(&add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_POINTING_WORD(), "fullmatch", &[geti(&words.clone(), &add(&at.clone(), &Py::Int(1)))])]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn asks_what_i_said(args: &[Py]) -> Py {
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
    let _ = py_str("True for an ask about what was said or felt in this chat, naming no slot of its own.");
    return call_builtin("bool", &[mcall(&g_WHAT_I_SAID(), "search", &[call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn nothing_said(args: &[Py]) -> Py {
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
    let mut turn: Py = Py::None;
    let _ = py_str("True when no earlier line of theirs has carried a subject yet.\n\n    \"hi\" and then \"same as before\" is a chat with nothing to be the same as, and the same goes\n    for \"ok\" or \"thanks\" in front of it: a recap or a pointer needs a subject behind it, and a\n    greeting has none.\n    ");
    return Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_turns(&[memory.clone()])) { let mut turn: Py = __item.clone(); __out.push(carries_subject(&[call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))));
    Py::None
}
pub fn _is_memory_line(args: &[Py]) -> Py {
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
    let _ = py_str("A line about the conversation itself (\"do you remember what we talked about\") is not\n    something the conversation is about.");
    return { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("about-the-bot"), py_str("memory")]), &mcall(&reading.clone(), "get", &[py_str("topic_own")]))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("about_bot"), py_str("greeting")]), &mcall(&reading.clone(), "get", &[py_str("intent")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("greeting"), py_str("memory")]), &mcall(&reading.clone(), "get", &[py_str("kind")]))) } } } };
    Py::None
}
pub fn _named_entities(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut name: Py = Py::None;
    let mut out: Py = Py::None;
    let _ = py_str("The entities worth naming: no documents, no pronouns, no one-word filler.");
    out = py_list(vec![]);
    if py_truthy(&_is_memory_line(&[reading.clone()])) {
    return out.clone();
    }
    let __iter1 = py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter1 {
    entity = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    name = mcall(&{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    core = mcall(&name.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WEAK(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_FLIP(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THIN(), &core.clone())); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } } } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_CONTRACTIONS(), &geti(&mcall(&core.clone(), "split", &[]), &Py::Int(0))))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MEASURE(), &core.clone())); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "isdigit", &[]) } }) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[entity.clone()]);
    }
    return out.clone();
    Py::None
}
pub fn _groups(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut entities: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entities") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entities"))) };
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut entity: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut last: Py = Py::None;
    let mut name: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Entities that sit next to each other in the line, joined: \"birthday\" + \"cake\".");
    text = { let __b = mcall(&reading.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } };
    groups = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&entities.clone());
    for __item in __iter1 {
    entity = __item.clone();
    name = mcall(&geti(&entity.clone(), &py_str("text")), "strip", &[]);
    if py_truthy(&groups.clone()) {
    last = geti(&groups.clone(), &neg(&Py::Int(1)));
    joined = add(&add(&mcall(&geti(&geti(&last.clone(), &neg(&Py::Int(1))), &py_str("text")), "strip", &[]), &py_str(" ")), &name.clone());
    if py_truthy(&Py::Bool(py_contains(&text.clone(), &mcall(&joined.clone(), "lower", &[])))) {
    let _ = mcall(&last.clone(), "append", &[entity.clone()]);
    continue;
    }
    }
    let _ = mcall(&groups.clone(), "append", &[py_list(vec![entity.clone()])]);
    }
    return groups.clone();
    Py::None
}
pub fn _is_the_verb(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut index: Py = Py::None;
    let mut token: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when `word` sits where the sentence puts its verb (\"should i put\", \"we moved\").");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    token = geti(&words.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&token.clone(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "startswith", &[mcall(&word.clone(), "lower", &[])]))) } }) {
    continue;
    }
    return { let __b = gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VERB_BEFORE(), &geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))))) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _verb_after_a_subject(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut owner: Py = Py::None;
    let mut token: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the word is the verb of \"my back hurts\", \"the shed leaks\".\n\n    A noun the line has already determined (\"my back\", \"the shed\") cannot be followed by another\n    bare noun, so a word that ends in -s there is what the clause does, not a second thing. Only\n    that shape: a plural said on its own (\"the chair legs\") is read the same way only because the\n    word before it is a noun the line determined, and \"the chair\" is a fair subject for it too.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("s")]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("ss")]); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } } } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    token = geti(&words.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&token.clone(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "startswith", &[mcall(&word.clone(), "lower", &[])]))) } }) {
    continue;
    }
    if py_truthy(&lt(&index.clone(), &Py::Int(2))) {
    return Py::Bool(false);
    }
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))), geti(&words.clone(), &sub(&index.clone(), &Py::Int(2)))]), 2, None);
    before = __unpk2[0].clone();
    owner = __unpk2[1].clone();
    return { let __b = Py::Bool(py_contains(&g_DETERMINERS(), &owner.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("your"), py_str("his"), py_str("her"), py_str("their")]), &owner.clone())) } };
    }
    return Py::Bool(false);
    Py::None
}
pub fn _word_behind_prep(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the only place this word holds in the line is behind a preposition.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&geti(&words.clone(), &index.clone()), &mcall(&word.clone(), "lower", &[])))) {
    continue;
    }
    before = sub(&index.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = ge(&before.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&words.clone(), &before.clone()))) } }) {
    let __aug2 = sub(&before.clone(), &Py::Int(1));
    before = __aug2;
    }
    if py_truthy(&{ let __b = ge(&before.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREP(), &geti(&words.clone(), &before.clone()))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _behind_a_prep(args: &[Py]) -> Py {
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
    let mut entity: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entity") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entity"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the words just before this entity are a preposition and its article.");
    words = { let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } };
    at = mcall(&entity.clone(), "get", &[py_str("at")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[at.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = le(&at.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { gt(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::Bool(false);
    }
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), sub(&at.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "lower", &[])); } py_list(__out) })() };
    while py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&before.clone(), &neg(&Py::Int(1))))) } }) {
    let _ = mcall(&before.clone(), "pop", &[]);
    }
    return { let __b = call_builtin("bool", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREP(), &geti(&before.clone(), &neg(&Py::Int(1))))) } };
    Py::None
}
pub fn _the_phrase_around(args: &[Py]) -> Py {
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
    let mut word: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "word") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: word"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut token: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The phrase `word` sits in: (\"your\", \"back\") for \"my back\", (\"the\", \"cake\"), or ().\n\n    A line whose question word is the only thing before the noun (\"where should i go on\n    holiday\") has no phrase around it, and a bare word there is as likely to be half a verb.\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    token = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&token.clone(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "startswith", &[mcall(&word.clone(), "lower", &[])]))) } }) {
    continue;
    }
    before = if py_truthy(&index.clone()) { geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("my"), py_str("our"), py_str("your")]), &before.clone()))) {
    return py_tuple(vec![py_str("your"), word.clone()]);
    }
    if py_truthy(&Py::Bool(py_contains(&g_DETERMINERS(), &before.clone()))) {
    return py_tuple(vec![before.clone(), word.clone()]);
    }
    return py_tuple(vec![]);
    }
    return py_tuple(vec![]);
    Py::None
}
pub fn _phrase_for(args: &[Py]) -> Py {
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
    let mut reading: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reading") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reading"))) };
    let mut named_only: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named_only") { v.clone() }
    else { Py::Bool(false) };
    let mut around: Py = Py::None;
    let mut before: Py = Py::None;
    let mut chosen: Py = Py::None;
    let mut e: Py = Py::None;
    let mut entities: Py = Py::None;
    let mut g: Py = Py::None;
    let mut groups: Py = Py::None;
    let mut kept: Py = Py::None;
    let mut keywords: Py = Py::None;
    let mut kind: Py = Py::None;
    let mut name: Py = Py::None;
    let mut nouns: Py = Py::None;
    let mut things: Py = Py::None;
    let mut w: Py = Py::None;
    let _ = py_str("The thing this line is about, as a phrase with its article, or None.");
    keywords = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("keywords")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_WEAK(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_THIN(), &w.clone())) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    entities = _named_entities(&[reading.clone()]);
    chosen = Py::None;
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("place"), py_str("person"), py_str("name"), py_str("thing")]));
    for __item in __iter1 {
    kind = __item.clone();
    groups = _groups(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&entities.clone()) { let mut e: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&mcall(&e.clone(), "get", &[py_str("type")]), &kind.clone()))) {  __out.push(e.clone()); } } py_list(__out) })() }, reading.clone()]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("person"), py_str("thing")]), &kind.clone()))) {
    groups = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&groups.clone()) { let mut g: Py = __item.clone(); if py_truthy(&Py::Bool(!py_truthy(&_behind_a_prep(&[reading.clone(), geti(&g.clone(), &Py::Int(0))])))) {  __out.push(g.clone()); } } py_list(__out) })() };
    }
    if py_truthy(&groups.clone()) {
    chosen = geti(&groups.clone(), &neg(&Py::Int(1)));
    break;
    }
    }
    if py_truthy(&chosen.clone()) {
    name = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&chosen.clone()) { let mut e: Py = __item.clone(); __out.push(mcall(&geti(&e.clone(), &py_str("text")), "strip", &[])); } py_list(__out) })() }]);
    before = _qualifier(&[reading.clone(), name.clone(), keywords.clone()]);
    if py_truthy(&before.clone()) {
    name = add(&add(&before.clone(), &py_str(" ")), &name.clone());
    }
    name = _same_spelling(&[text.clone(), name.clone()]);
    kind = mcall(&geti(&chosen.clone(), &Py::Int(0)), "get", &[py_str("type")]);
    if py_truthy(&mcall(&mcall(&name.clone(), "lower", &[]), "startswith", &[py_str("my ")])) {
    name = add(&py_str("your "), &slice_or_raise(&name.clone(), &Py::Int(3), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_NAMES(), &kind.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("name"))); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]) } } } }) {
    return _place_case(&[name.clone()]);
    }
    if py_truthy(&mcall(&mcall(&name.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("your "), py_str("a "), py_str("an "), py_str("the ")])])) {
    return name.clone();
    }
    return add(&py_str("the "), &name.clone());
    }
    nouns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keywords.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&w.clone(), "endswith", &[py_tuple(vec![py_str("ing"), py_str("ed")])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NON_NOUN(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_QUALITY(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CALENDAR(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_the_verb(&[reading.clone(), w.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_verb_after_a_subject(&[reading.clone(), w.clone()]))) } } } } } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&nouns.clone()))) {
    return Py::None;
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&nouns.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&crate::gen::understanding::kinds::g_VERB_FORMS(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&things.clone()) {
    nouns = things.clone();
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&nouns.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::understanding::topics::g_PLACES(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_word_behind_prep(&[reading.clone(), w.clone()]))) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[nouns.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    nouns = kept.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&crate::gen::understanding::topics::g_PLACES(), &geti(&nouns.clone(), &neg(&Py::Int(1)))))) {
    return _place_case(&[_same_spelling(&[text.clone(), geti(&nouns.clone(), &neg(&Py::Int(1)))])]);
    }
    around = _the_phrase_around(&[reading.clone(), geti(&nouns.clone(), &neg(&Py::Int(1)))]);
    if py_truthy(&{ let __b = named_only.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&around.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = around.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&around.clone(), &Py::Int(0)), &py_str("your"))) } }) {
    return add(&py_str("your "), &geti(&nouns.clone(), &neg(&Py::Int(1))));
    }
    return add(&py_str("the "), &geti(&nouns.clone(), &neg(&Py::Int(1))));
    Py::None
}
pub fn _place_case(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("A place is a name, so it keeps its capital however it was typed: \"rome\" -> \"Rome\".");
    words = mcall(&name.clone(), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return name.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::understanding::topics::g_PLACES(), &mcall(&name.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&crate::gen::understanding::topics::g_PLACES(), &mcall(&w.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(_capital(&[w.clone()])); } py_list(__out) })() }]);
    }
    return name.clone();
    Py::None
}
pub fn _same_spelling(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut index: Py = Py::None;
    let mut lowered: Py = Py::None;
    let _ = py_str("`phrase` as it was typed, so a name keeps its capitals (\"Lisbon\", not \"lisbon\").");
    lowered = mcall(&text.clone(), "lower", &[]);
    index = mcall(&lowered.clone(), "find", &[mcall(&phrase.clone(), "lower", &[])]);
    if py_truthy(&ge(&index.clone(), &Py::Int(0))) {
    return slice_or_raise(&text.clone(), &index.clone(), &add(&index.clone(), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    }
    return phrase.clone();
    Py::None
}
pub fn _qualifier(args: &[Py]) -> Py {
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
    let mut phrase: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut keywords: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "keywords") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: keywords"))) };
    let mut before: Py = Py::None;
    let mut index: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The word that belongs to `phrase`: \"birthday cake\", \"job interview\", \"rescue dog\".\n\n    Only a word that sits between a determiner and the thing counts, so \"to save money\" keeps\n    \"money\" and \"a rescue dog\" becomes \"rescue dog\".\n    ");
    text = { let __b = mcall(&reading.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } };
    index = mcall(&text.clone(), "find", &[mcall(&phrase.clone(), "lower", &[])]);
    if py_truthy(&le(&index.clone(), &Py::Int(0))) {
    return Py::None;
    }
    words = mcall(&slice_or_raise(&text.clone(), &Py::None, &index.clone(), &Py::None), "split", &[]);
    if py_truthy(&lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    before = mcall(&geti(&words.clone(), &neg(&Py::Int(1))), "strip", &[py_str(".,!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&keywords.clone(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &mcall(&geti(&words.clone(), &neg(&Py::Int(2))), "strip", &[py_str(".,!?")]))) } }) {
    return Py::None;
    }
    return before.clone();
    Py::None
}
pub fn about(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { Py::None };
    let mut lines: Py = Py::None;
    let mut names_something: Py = Py::None;
    let mut pass_kind: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The thing the conversation is on, as a phrase (\"the birthday cake\", \"Lisbon\"), or None.\n\n    The newest line that carries a subject wins, so a return to something said earlier is\n    named before the small talk that followed it. A line that names something beats one that\n    only carries a content word, so \"and can I make it the day before?\" never becomes the\n    subject of a conversation about a cake.\n\n    When no line carries a subject of its own it defers to `topics.subject`, so this module\n    adds a reader on top of the store's answer instead of a second answer to the same\n    question.\n    ");
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&user_lines(&[memory.clone()])) { let __unpk8126 = py_unpack(&__item.clone(), 2, None); let mut text: Py = __unpk8126[0].clone(); let mut reading: Py = __unpk8126[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_is_thin(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_memory_line(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_asks_about_the_chat(&[text.clone(), memory.clone(), skip.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_asks_about_the_bot(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&signs_off(&[text.clone()]))) } } } } } } } }) {  __out.push(py_tuple(vec![text.clone(), reading.clone()])); } } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("entity"), py_str("statement"), py_str("any")]));
    for __item in __iter1 {
    pass_kind = __item.clone();
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    text = __unpk3[0].clone();
    reading = __unpk3[1].clone();
    names_something = call_builtin("bool", &[_named_entities(&[reading.clone()])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(py_eq(&pass_kind.clone(), &py_str("entity"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&names_something.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&pass_kind.clone(), &py_str("statement"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&names_something.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&pass_kind.clone(), &py_str("entity"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("intent")]), &py_str("feeling"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("topic_own")]), &py_str("feelings"))) } } } }) {
    continue;
    }
    phrase = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(text.clone()); __args.push(reading.clone()); __kws.extend(vec![("named_only".to_string(), Py::Bool(py_eq(&pass_kind.clone(), &py_str("any"))))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _phrase_for(args))(&__args) };
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { _is_subject_phrase(&[phrase.clone()]) } }) {
    return phrase.clone();
    }
    }
    }
    return _from_topics(&[memory.clone()]);
    Py::None
}
pub fn _is_subject_phrase(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut head: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the phrase can stand after \"we were on\".\n\n    A greeting or an \"ok\" the line carried is not a thing to be on (\"we were on hi\"), and\n    neither is a verb someone used to ask for help (\"can you take a look at this\" is not \"the\n    look\"). The store has a line to say back instead, or an honest \"we haven't started yet\".\n    ");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(",.;:")])); } py_list(__out) })() };
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    head = if py_truthy(&words.clone()) { mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut _topics: Py = crate::gen::memory::topics::module_obj();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ImportError"]) {
                __handled5 = true;
    return Py::Bool(true);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    if py_truthy(&{ let __b = mcall(&head.clone(), "isdigit", &[]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NUMBER_WORD_HEADS(), &head.clone())) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&gattr(&_topics.clone(), "GREET"), &head.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&gattr(&_topics.clone(), "ACKS"), &head.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_BARE_VERBS(), &head.clone())) } } } }));
    Py::None
}
pub fn _from_topics(args: &[Py]) -> Py {
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
    let mut phrase: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("What the memory store calls the subject, with the article a sentence needs.\n\n    `topics.subject` answers in keywords (\"birthday cake\", \"trip portugal june\"), which is\n    right for a key and reads wrong after \"we were on\". A proper noun keeps no article.\n    ");
    let __try1 = exc::catch(|| {
    let mut topics: Py = crate::gen::memory::topics::module_obj();
        py_tuple(vec![Py::Bool(false), Py::None])
    });
    let mut __ret2: Py = Py::Bool(false);
    let mut __val3: Py = Py::None;
    let mut __unhandled4: Option<exc::PyExc> = None;
    match __try1 {
        Ok(__t) => { let __p = py_iter_or_raise(&__t); __ret2 = __p[0].clone(); __val3 = __p[1].clone(); }
        Err(__e) => {
            let mut __handled5 = false;
            if !__handled5 && exc::matches_any(&__e, &["ImportError"]) {
                __handled5 = true;
    return Py::None;
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    if py_truthy(&__ret2) { return __val3; }
    phrase = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&topics.clone(), "subject", &[memory.clone()]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_is_subject_phrase(&[phrase.clone()])))) {
    return Py::None;
    }
    words = mcall(&phrase.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DETERMINERS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&geti(&words.clone(), &Py::Int(0)), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]) } }) {
    return phrase.clone();
    }
    return add(&py_str("the "), &phrase.clone());
    Py::None
}
pub fn _is_wh(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let _ = py_str("True when the word is a question word, contraction and all.\n\n    \"whats the cheapest way\" is the question word with its verb on the end, so a question word\n    put in front of it made \"what whats the cheapest way to that\" - the same sentence with the\n    ask said twice, once broken. The apostrophe is often not typed, so it is not required.\n    ");
    core = mcall(&mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = word.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[]), "replace", &[py_str("'"), py_str("")]), "replace", &[py_str("’"), py_str("")]);
    if py_truthy(&Py::Bool(py_contains(&g_WH(), &core.clone()))) {
    return Py::Bool(true);
    }
    return { let __b = mcall(&core.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WH(), &slice_or_raise(&core.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } };
    Py::None
}
pub fn _with_wh(args: &[Py]) -> Py {
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
    let mut words: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: words"))) };
    let mut ask: Py = Py::None;
    let mut held: Py = Py::None;
    let mut index: Py = Py::None;
    let mut norm: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word2: Py = Py::None;
    let _ = py_str("Some readings keep the question word out of the focus (\"how many days would you give it\"\n    reads as \"would you give it\"), so take it from the words the user typed.");
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&words.clone())) } }) {
    return words.clone();
    }
    if py_truthy(&_is_wh(&[geti(&words.clone(), &Py::Int(0))])) {
    return words.clone();
    }
    held = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?;:")]), "lower", &[])); } py_set(__out) })() };
    norm = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("norm")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[norm.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_contains(&g_WH(), &word.clone()))) {
    continue;
    }
    phrase = py_list(vec![]);
    let __iter3 = py_iter_or_raise(&slice_or_raise(&norm.clone(), &index.clone(), &add(&index.clone(), &Py::Int(3)), &Py::None));
    for __item in __iter3 {
    word2 = __item.clone();
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_AUX(), &word2.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SUBJECTISH(), &word2.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &word2.clone())) } } } } } }) {
    break;
    }
    let _ = mcall(&phrase.clone(), "append", &[word2.clone()]);
    }
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &mcall(&call_builtin("str", &[geti(&phrase.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return add(&phrase.clone(), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    }
    break;
    }
    let __iter4 = py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter4 {
    ask = __item.clone();
    phrase = { let __b = mcall(&g_ASK_WORDS(), "get", &[ask.clone()]); if py_truthy(&__b) { __b } else { if py_truthy(&Py::Bool(py_contains(&g_WH(), &ask.clone()))) { py_tuple(vec![ask.clone()]) } else { py_tuple(vec![]) } } };
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &mcall(&call_builtin("str", &[geti(&phrase.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return add(&call_builtin("list", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    }
    }
    return words.clone();
    Py::None
}
pub fn _strip_lead(args: &[Py]) -> Py {
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
    let mut changed: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("Drop the discourse markers that open a follow-up (\"and\", \"so\", \"ok\").");
    words = call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    changed = Py::Bool(true);
    while py_truthy(&{ let __b = changed.clone(); if !py_truthy(&__b) { __b } else { words.clone() } }) {
    changed = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&g_LEAD());
    for __item in __iter1 {
    lead = __item.clone();
    parts = mcall(&lead.clone(), "split", &[]);
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&words.clone(), &Py::None, &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &parts.clone()))) {
    words = slice_or_raise(&words.clone(), &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    changed = Py::Bool(true);
    }
    }
    }
    return words.clone();
    Py::None
}
pub fn _with_you(args: &[Py]) -> Py {
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
    let mut first: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The subject a line that opens on an auxiliary left out, and the verb that goes with it.");
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return py_list(vec![py_str("you")]);
    }
    first = mcall(&{ let __b = mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?;:")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]);
    lead = mcall(&g_SECOND_PERSON(), "get", &[first.clone()]);
    if py_truthy(&Py::Bool(!py_is(&lead.clone(), &Py::None))) {
    return add(&call_builtin("list", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    }
    return add(&py_list(vec![py_str("you")]), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _first_person_aux(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let mut tail: Py = Py::None;
    core = mcall(&word.clone(), "strip", &[py_str(".,!?;:")]);
    tail = slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    return add(&mcall(&g_FIRST_PERSON_AUX(), "get", &[mcall(&core.clone(), "lower", &[]), core.clone()]), &tail.clone());
    Py::None
}
pub fn _flip(args: &[Py]) -> Py {
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
    let mut addressee: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "addressee") { v.clone() }
    else { Py::Bool(false) };
    let mut before: Py = Py::None;
    let mut core: Py = Py::None;
    let mut index: Py = Py::None;
    let mut out: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("First person the user used becomes second person when the bot says it back.\n\n    With addressee, the person's \"you\" is the bot: it is \"I\" where the question makes it the\n    subject (\"can you suggest\" -> \"can I suggest\") and \"me\" where it is the object (\"did i tell\n    you\" -> \"whether you told me\").\n    ");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    core = mcall(&word.clone(), "strip", &[py_str(".,!?;:")]);
    tail = slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    if py_truthy(&{ let __b = addressee.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ADDRESSEE(), &core.clone())) } }) {
    before = if py_truthy(&index.clone()) { mcall(&mcall(&geti(&words.clone(), &sub(&index.clone(), &Py::Int(1))), "strip", &[py_str(".,!?;:")]), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&core.clone(), &py_str("you"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&index.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_AUX(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WH(), &before.clone())) } } } } } }) {
    if py_truthy(&out.clone()) {
    let _ = py_setitem(&out.clone(), &neg(&Py::Int(1)), _first_person_aux(&[geti(&out.clone(), &neg(&Py::Int(1)))]));
    }
    let _ = mcall(&out.clone(), "append", &[add(&g_SUBJECT_YOU(), &tail.clone())]);
    } else {
    let _ = mcall(&out.clone(), "append", &[add(&geti(&g_ADDRESSEE(), &core.clone()), &tail.clone())]);
    }
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[add(&mcall(&g_FLIP(), "get", &[core.clone(), core.clone()]), &tail.clone())]);
    }
    return out.clone();
    Py::None
}
pub fn your_words(args: &[Py]) -> Py {
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
    let mut words: Py = Py::None;
    let _ = py_str("The person's own phrase in the second person, for a reply that says it back.\n\n    A stock line drops the phrase straight into a sentence (\"{X} keeps happening\"), and the\n    phrase is the person's own words: \"comparing myself to others\" reads as the bot's own\n    trouble unless it is flipped to \"comparing yourself to others\", and \"my keys\" to \"your\n    keys\". Only the person's first person moves; everything else is left as they said it.\n    ");
    words = mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    return mcall(&py_str(" "), "join", &[_flip(&[words.clone()])]);
    Py::None
}
pub fn _expand(args: &[Py]) -> Py {
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
    let _ = py_str("The line with its contractions opened and no full stop at the end.");
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&g_EXPAND(), "get", &[mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[]), word.clone()])); } py_list(__out) })() };
    return mcall(&mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:")]), "rstrip", &[py_str(".")]);
    Py::None
}
pub fn _as_text(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("The reading gives `focus` as words or as a string; take either.");
    if py_truthy(&call_builtin("isinstance", &[value.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    return mcall(&mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&value.clone()) { let mut w: Py = __item.clone(); __out.push(call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]), "strip", &[]);
    }
    return mcall(&call_builtin("str", &[{ let __b = value.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    Py::None
}
pub fn _subjects_at(args: &[Py]) -> Py {
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
    let mut index: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "index") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: index"))) };
    let mut names: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { py_tuple(vec![]) };
    let mut core: Py = Py::None;
    let _ = py_str("True when words[index] can be the subject right after an auxiliary.");
    if py_truthy(&ge(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return Py::Bool(false);
    }
    core = mcall(&mcall(&geti(&words.clone(), &index.clone()), "strip", &[py_str(".,!?")]), "lower", &[]);
    return { let __b = Py::Bool(py_contains(&g_SUBJECTISH(), &core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&names.clone(), &core.clone())) } };
    Py::None
}
pub fn _np_end(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut most: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "most") { v.clone() }
    else { Py::Int(3) };
    let mut core: Py = Py::None;
    let mut end: Py = Py::None;
    let _ = py_str("Where the subject after an auxiliary ends: \"my boiler\" in \"is my boiler safe\".\n\n    A pronoun is the whole subject; a determiner opens a phrase that runs to the word before\n    the predicate (\"is the best way to cook it\" keeps \"the best way\").\n    ");
    if py_truthy(&ge(&start.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return start.clone();
    }
    if py_truthy(&Py::Bool(!py_contains(&g_DETERMINERS(), &mcall(&mcall(&geti(&words.clone(), &start.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[])))) {
    return add(&start.clone(), &Py::Int(1));
    }
    end = add(&start.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = lt(&end.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { lt(&sub(&end.clone(), &start.clone()), &most.clone()) } }) {
    core = mcall(&mcall(&geti(&words.clone(), &end.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AUX(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WH(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PREDICATE(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PREP(), &core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ADVERB(), &core.clone())) } } } } } } } }) {
    break;
    }
    let __aug1 = add(&end.clone(), &Py::Int(1));
    end = __aug1;
    }
    return call_builtin("max", &[end.clone(), add(&start.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _embed(args: &[Py]) -> Py {
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
    let mut names: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "names") { v.clone() }
    else { py_tuple(vec![]) };
    let mut aux: Py = Py::None;
    let mut end: Py = Py::None;
    let mut index: Py = Py::None;
    let mut start: Py = Py::None;
    let mut subject: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut verb: Py = Py::None;
    let _ = py_str("The question with its auxiliary behind the subject, i.e. said back inside a sentence.\n\n    \"is my boiler safe\" -> \"my boiler is safe\"; \"does he need a booster seat\" -> \"he needs a\n    booster seat\"; \"how long should i cook it\" -> \"how long you should cook it\". A yes/no\n    question keeps its auxiliary (the caller puts \"whether\" in front); a do/does/did question\n    drops it and inflects the one verb instead, so nothing is inflected twice.\n    ");
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    aux = mcall(&mcall(&geti(&words.clone(), &index.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_AUX(), &aux.clone()))) {
    continue;
    }
    start = add(&index.clone(), &Py::Int(1));
    if py_truthy(&Py::Bool(!py_truthy(&_subjects_at(&[words.clone(), start.clone(), names.clone()])))) {
    continue;
    }
    end = _np_end(&[words.clone(), start.clone()]);
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&words.clone(), &start.clone(), &end.clone(), &Py::None), slice_or_raise(&words.clone(), &end.clone(), &Py::None, &Py::None)]), 2, None);
    subject = __unpk2[0].clone();
    tail = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MODAL(), &aux.clone())); if !py_truthy(&__b) { __b } else { { let __b = gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&tail.clone())) } } } } } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![slice_or_raise(&subject.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), slice_or_raise(&subject.clone(), &neg(&Py::Int(1)), &Py::None, &Py::None)]), 2, None);
    subject = __unpk3[0].clone();
    tail = __unpk3[1].clone();
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("do"), py_str("does"), py_str("did")]), &aux.clone()))) {
    verb = Py::Int(0);
    while py_truthy(&{ let __b = lt(&verb.clone(), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ADVERB(), &mcall(&mcall(&geti(&tail.clone(), &verb.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[]))) } }) {
    let __aug4 = add(&verb.clone(), &Py::Int(1));
    verb = __aug4;
    }
    if py_truthy(&lt(&verb.clone(), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    let _ = py_setitem(&tail.clone(), &verb.clone(), _agree(&[geti(&tail.clone(), &verb.clone()), aux.clone()]));
    } else {
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&bitor(&bitor(&g_DETERMINERS(), &g_PREDICATE()), &g_MEASURE()), &mcall(&mcall(&geti(&subject.clone(), &neg(&Py::Int(1))), "strip", &[py_str(".,!?;:")]), "lower", &[]))) } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![slice_or_raise(&subject.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), py_list(vec![_agree(&[geti(&subject.clone(), &neg(&Py::Int(1))), aux.clone()])])]), 2, None);
    subject = __unpk5[0].clone();
    tail = __unpk5[1].clone();
    }
    }
    return add(&add(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None), &subject.clone()), &tail.clone());
    }
    return add(&add(&add(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None), &subject.clone()), &py_list(vec![geti(&words.clone(), &index.clone())])), &tail.clone());
    }
    return words.clone();
    Py::None
}
pub fn _keep_possessives(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut core: Py = Py::None;
    let mut index: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut out: Py = Py::None;
    let mut stems: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Put back the \"'s\" the reading drops: \"my mum's birthday\" is \"your mum's birthday\".\n\n    The reading's focus words carry the noun without its possessive, so a line said back loses\n    it (\"your mum birthday\"). Only the stems that really had one in the line get it back.");
    stems = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_POSSESSIVE(), "finditer", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut match_: Py = __item.clone(); __out.push(mcall(&mcall(&match_.clone(), "group", &[Py::Int(1)]), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&stems.clone()))) {
    return clause.clone();
    }
    words = mcall(&call_builtin("str", &[{ let __b = clause.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    core = mcall(&word.clone(), "strip", &[py_str(".,!?;:")]);
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))))) {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&stems.clone(), &mcall(&core.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&core.clone(), &py_str("'"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&core.clone(), &py_str("’"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IS_NOT_POSSESSIVE(), &mcall(&core.clone(), "lower", &[]))) } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[add(&add(&core.clone(), &py_str("'s")), &slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))]);
    } else {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    }
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
    Py::None
}
pub fn _truncated(args: &[Py]) -> Py {
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
    let mut clean: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut clean_words: Py = Py::None;
    let mut focus_words: Py = Py::None;
    let _ = py_str("True when the reading's focus stops before the line does (\"am i right about the boiler\"\n    leaves \"am i\"), so saying it back would lose the question.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&clause.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clean.clone())) } }) {
    return Py::Bool(false);
    }
    focus_words = mcall(&mcall(&mcall(&clause.clone(), "replace", &[py_str("?"), py_str("")]), "lower", &[]), "split", &[]);
    clean_words = mcall(&mcall(&mcall(&clean.clone(), "replace", &[py_str("?"), py_str("")]), "lower", &[]), "split", &[]);
    return { let __b = lt(&call_builtin("len", &[focus_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[clean_words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&clean_words.clone(), &Py::None, &call_builtin("len", &[focus_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &focus_words.clone())) } };
    Py::None
}
pub fn _numbers(args: &[Py]) -> Py {
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
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut keep: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut typed: Py = Py::None;
    let _ = py_str("The numbers in `clause` as the person typed them: \"6000\" -> \"6,000\".");
    typed = { (|| -> Py { let mut __out: Vec<(Py, Py)> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_NUMBER(), "finditer", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])) { let mut match_: Py = __item.clone(); __out.push((mcall(&mcall(&match_.clone(), "group", &[Py::Int(0)]), "replace", &[py_str(","), py_str("")]), mcall(&match_.clone(), "group", &[Py::Int(0)]))); } py_dict(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&typed.clone()))) {
    return clause.clone();
    }
pub fn __nested_keep_0(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = args.to_vec();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    let mut __split = __pos.len();
    for (i, a) in args.iter().enumerate() { if let Py::Kwargs(kws) = a { __split = i; for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } break; } }
    let __pos = &args[..__split];
    let mut clause: Py = __pos.get(0).cloned().unwrap_or(Py::None);
    let mut keep: Py = __pos.get(1).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(2).cloned().unwrap_or(Py::None);
    let mut text: Py = __pos.get(3).cloned().unwrap_or(Py::None);
    let mut typed: Py = __pos.get(4).cloned().unwrap_or(Py::None);
    let mut match_: Py = __pos.get(5).cloned().or_else(|| __kw.iter().find(|(n, _)| n == "match").map(|(_, v)| v.clone())).unwrap_or(Py::None);
    return { let __b = mcall(&typed.clone(), "get", &[mcall(&match_.clone(), "group", &[Py::Int(0)])]); if py_truthy(&__b) { __b } else { mcall(&match_.clone(), "group", &[Py::Int(0)]) } };
    Py::None
}
    keep = py_func("keep", move |args: &[Py]| { let mut full = vec![clause.clone(), keep.clone(), match_.clone(), text.clone(), typed.clone()]; full.extend_from_slice(args); __nested_keep_0(&full) });
    return stdlib_call("re", "sub", &[py_str("\\d[\\d,]*(?:\\.\\d+)?"), keep.clone(), clause.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn restate(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut clean: Py = Py::None;
    let mut core: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut first: Py = Py::None;
    let mut first_word: Py = Py::None;
    let mut index: Py = Py::None;
    let mut line_words: Py = Py::None;
    let mut names: Py = Py::None;
    let mut opening: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut typed_aux: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The user's line said again inside a sentence, in their words, or None.\n\n    A yes/no question becomes a \"whether ...\" clause, a wh-question is embedded\n    (\"how long should i cook it\" -> \"how long you should cook it\"), and first person becomes\n    second person. Statements are returned as they are, minus a leading \"and\"/\"so\".\n    ");
    reading = crate::gen::understanding::understand(&[text.clone(), py_dict(vec![])]);
    clean = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    clause = _as_text(&[mcall(&reading.clone(), "get", &[py_str("focus")])]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    clause = mcall(&geti(&mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[py_str(" | ")]), &neg(&Py::Int(1))), "strip", &[]);
    }
    if py_truthy(&_truncated(&[clause.clone(), clean.clone()])) {
    clause = clean.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    clause = clean.clone();
    }
    first_word = mcall(&mcall(&geti(&{ let __b = mcall(&clause.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    line_words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&clean.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[])); } py_set(__out) })() };
    if py_truthy(&{ let __b = clean.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PREP(), &first_word.clone())); if py_truthy(&__b) { __b } else { { let __b = first_word.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&line_words.clone(), &first_word.clone())) } } } } } }) {
    clause = _expand(&[clean.clone()]);
    }
    words = _strip_lead(&[mcall(&mcall(&clause.clone(), "replace", &[py_str("?"), py_str("")]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::None;
    }
    words = _with_wh(&[reading.clone(), words.clone()]);
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[])); } py_list(__out) })() };
    typed_aux = call_builtin("next", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&typed.clone()) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("do"), py_str("does"), py_str("did")]), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() }, Py::None]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&typed_aux.clone()) {
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    word = __unpk2[1].clone();
    core = mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("do"), py_str("does"), py_str("did")]), &core.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&core.clone(), &typed_aux.clone())) } }) {
    let _ = py_setitem(&words.clone(), &index.clone(), add(&typed_aux.clone(), &slice_or_raise(&word.clone(), &call_builtin("len", &[mcall(&word.clone(), "strip", &[py_str(".,!?;:")])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)));
    break;
    }
    }
    }
    first = mcall(&mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?")]), "lower", &[]);
    names = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut entity: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&mcall(&{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "lower", &[])); } } py_set(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AUX(), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WH(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DROPPED_SUBJECT(), &first.clone())) } } } }) {
    words = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(words.clone()); __kws.extend(vec![("addressee".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _flip(args))(&__args) };
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_DROPPED_SUBJECT(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question"))) } }) {
    words = _with_you(&[words.clone()]);
    said = mcall(&py_str(" "), "join", &[_embed(&[words.clone(), names.clone()])]);
    clause = said.clone();
    } else {
    said = mcall(&py_str(" "), "join", &[_embed(&[words.clone(), names.clone()])]);
    clause = if py_truthy(&Py::Bool(py_contains(&g_AUX(), &first.clone()))) { add(&py_str("whether "), &said.clone()) } else { said.clone() };
    }
    } else {
    clause = crate::gen::memory::_from_the_bot(&[mcall(&py_str(" "), "join", &[_flip(&[mcall(&crate::gen::memory::_to_the_bot(&[mcall(&py_str(" "), "join", &[words.clone()])]), "split", &[])])])]);
    }
    clause = _conditional(&[clause.clone()]);
    clause = _keep_possessives(&[clause.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question")))) {
    core = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&clause.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[])); } py_set(__out) })() };
    opening = geti(&{ let __b = mcall(&clause.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&bitand(&core.clone(), &bitor(&g_AUX(), &g_WH())))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_wh(&[opening.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&core.clone(), &py_str("whether"))) } } } }) {
    return Py::None;
    }
    }
    clause = _numbers(&[text.clone(), clause.clone()]);
    clause = crate::gen::memory::facts::no_tone(&[clause.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    return Py::None;
    }
    return { let __b = _restore_case(&[text.clone(), mcall(&clause.clone(), "strip", &[py_str(" ,;:")])]); if py_truthy(&__b) { __b } else { Py::None } };
    Py::None
}
pub fn _past(args: &[Py]) -> Py {
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
    let mut core: Py = Py::None;
    let mut suffix: Py = Py::None;
    core = mcall(&word.clone(), "strip", &[py_str(".,!?;:")]);
    suffix = slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(py_contains(&g_PAST(), &core.clone()))) {
    return add(&geti(&g_PAST(), &core.clone()), &suffix.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&core.clone(), &neg(&Py::Int(1)), &Py::None, &Py::None), "isalpha", &[]))) } }) {
    return word.clone();
    }
    if py_truthy(&mcall(&core.clone(), "endswith", &[py_str("e")])) {
    return add(&add(&core.clone(), &py_str("d")), &suffix.clone());
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&core.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&core.clone(), &neg(&Py::Int(2))))) } } } }) {
    return add(&add(&slice_or_raise(&core.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str("ied")), &suffix.clone());
    }
    return add(&add(&core.clone(), &py_str("ed")), &suffix.clone());
    Py::None
}
pub fn _agree(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut verb: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut aux: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "aux") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: aux"))) };
    let _ = py_str("The verb form that goes with the subject once \"do/does/did\" is dropped.");
    if py_truthy(&Py::Bool(py_eq(&aux.clone(), &py_str("did")))) {
    return _past(&[verb.clone()]);
    }
    if py_truthy(&Py::Bool(py_eq(&aux.clone(), &py_str("does")))) {
    return _third(&[verb.clone()]);
    }
    return verb.clone();
    Py::None
}
pub fn _third(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut verb: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "verb") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: verb"))) };
    let mut core: Py = Py::None;
    let mut low: Py = Py::None;
    let mut suffix: Py = Py::None;
    let _ = py_str("A base verb in the third person: \"need\" -> \"needs\", \"try\" -> \"tries\", \"have\" -> \"has\".\n\n    A do/does question takes the base form, so nothing here is already inflected except the\n    irregulars; \"need\" ends in -ed and must still become \"needs\".\n    ");
    core = mcall(&verb.clone(), "strip", &[py_str(".,!?;:")]);
    suffix = slice_or_raise(&verb.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    low = mcall(&core.clone(), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&core.clone(), &neg(&Py::Int(1)), &Py::None, &Py::None), "isalpha", &[]))) } }) {
    return verb.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_THIRD(), &low.clone()))) {
    return add(&geti(&g_THIRD(), &low.clone()), &suffix.clone());
    }
    if py_truthy(&Py::Bool(py_contains(&g_AUX(), &low.clone()))) {
    return verb.clone();
    }
    if py_truthy(&mcall(&low.clone(), "endswith", &[py_tuple(vec![py_str("s"), py_str("sh"), py_str("ch"), py_str("x"), py_str("z"), py_str("o")])])) {
    return add(&add(&core.clone(), &py_str("es")), &suffix.clone());
    }
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&low.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&low.clone(), &neg(&Py::Int(2))))) } } } }) {
    return add(&add(&slice_or_raise(&core.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None), &py_str("ies")), &suffix.clone());
    }
    return add(&add(&core.clone(), &py_str("s")), &suffix.clone());
    Py::None
}
pub fn _conditional(args: &[Py]) -> Py {
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
    let _ = py_str("An \"if\" question is said back as what happens in that case: \"if i stuff it\" ->\n    \"what happens if you stuff it\".");
    if py_truthy(&mcall(&clause.clone(), "startswith", &[py_str("if ")])) {
    return add(&py_str("what happens "), &clause.clone());
    }
    return clause.clone();
    Py::None
}
pub fn _restore_case(args: &[Py]) -> Py {
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
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut capitals: Py = Py::None;
    let mut core: Py = Py::None;
    let mut out: Py = Py::None;
    let mut typed: Py = Py::None;
    let mut w: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("Put back the capitals the reading lowers: \"priya\" -> \"Priya\", \"leeds\" -> \"Leeds\".");
    typed = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "strip", &[py_str(".,!?;:")])); } py_list(__out) })() };
    capitals = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&typed.clone(), &Py::Int(1), &Py::None, &Py::None)) { let mut w: Py = __item.clone(); if py_truthy(&mcall(&slice_or_raise(&w.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[])) {  __out.push(mcall(&w.clone(), "lower", &[])); } } py_set(__out) })() };
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&clause.clone(), "split", &[]));
    for __item in __iter1 {
    word = __item.clone();
    core = mcall(&word.clone(), "strip", &[py_str(".,!?;:")]);
    let _ = mcall(&out.clone(), "append", &[if py_truthy(&Py::Bool(py_contains(&capitals.clone(), &core.clone()))) { add(&_capital(&[core.clone()]), &slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)) } else { word.clone() }]);
    }
    return mcall(&py_str(" "), "join", &[out.clone()]);
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
    let mut text: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    return if py_truthy(&text.clone()) { add(&mcall(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&text.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { text.clone() };
    Py::None
}
pub fn on_the_table(args: &[Py]) -> Py {
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
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(3) };
    let mut head: Py = Py::None;
    let mut misses: Py = Py::None;
    let mut one: Py = Py::None;
    let mut out: Py = Py::None;
    let mut phrase: Py = Py::None;
    let mut recent: Py = Py::None;
    let mut shorter: Py = Py::None;
    let mut size: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The things the chat is holding, newest first, as phrases the bot can name.\n\n    `about` answers with the one thing the conversation is on. A person apologising for the shape\n    of the chat (\"sorry i keep jumping about\", \"sorry for jumping between things\") is asking,\n    underneath it, whether the bot still has hold of what they brought - so this walks the same\n    readings over the chat and collects the two or three things said, newest first: \"your sister,\n    the mince and your manager\".\n\n    Each phrase comes from `about` on the chat up to that turn, so this adds no second reading of\n    a line; a phrase whose head is a verb (\"the keeps\") is dropped, and a repeated one is said\n    once. Nothing here writes to memory or answers a turn.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_list(vec![]);
    }
    recent = slice_or_raise(&turns.clone(), &neg(&g_ON_THE_TABLE_TURNS()), &Py::None, &Py::None);
    out = py_list(vec![]);
    misses = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[recent.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    size = __item.clone();
    shorter = call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&shorter.clone(), &py_str("turns"), slice_or_raise(&recent.clone(), &Py::None, &size.clone(), &Py::None));
    phrase = about(&[shorter.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    continue;
    }
    head = mcall(&geti(&{ let __b = mcall(&phrase.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &neg(&Py::Int(1))), "strip", &[py_str(".,!?")]);
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &head.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_said_in_the_chat(&[phrase.clone(), slice_or_raise(&recent.clone(), &Py::None, &size.clone(), &Py::None)])))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&phrase.clone(), "lower", &[]), &mcall(&one.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug2 = add(&misses.clone(), &Py::Int(1));
    misses = __aug2;
    if py_truthy(&ge(&misses.clone(), &g_ON_THE_TABLE_MISSES())) {
    break;
    }
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[phrase.clone()]);
    misses = Py::Int(0);
    if py_truthy(&ge(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
    break;
    }
    }
    return out.clone();
    Py::None
}
pub fn _said_in_the_chat(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrase: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrase") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrase"))) };
    let mut turns: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "turns") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: turns"))) };
    let mut entity: Py = Py::None;
    let mut head: Py = Py::None;
    let mut run: Py = Py::None;
    let mut said: Py = Py::None;
    let mut start: Py = Py::None;
    let mut text: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the phrase's words sit side by side in something the person said.\n\n    The phrase is built from the reading, so it can join words the chat only had apart (\"the\"\n    from one place, \"leaks\" from another). A thing on the table has to be a thing they said:\n    the check is a run of words, with the store's \"your\" put back to their \"my\".\n    ");
    words = mcall(&mcall(&call_builtin("str", &[{ let __b = phrase.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&geti(&words.clone(), &Py::Int(0)), &py_str("your")))) {
    words = add(&py_list(vec![py_str("my")]), &slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    head = geti(&words.clone(), &neg(&Py::Int(1)));
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    text = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&mcall(&text.clone(), "lower", &[]), "split", &[]);
    run = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[add(&sub(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &run.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&said.clone(), &start.clone(), &add(&start.clone(), &run.clone()), &Py::None), &words.clone()))) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_VERB_WORDS(), &head.clone()))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&crate::gen::understanding::understand(&[text.clone(), py_dict(vec![])]), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    entity = __item.clone();
    if py_truthy(&Py::Bool(!py_contains(&py_tuple(vec![py_str("thing"), py_str("person")]), &mcall(&entity.clone(), "get", &[py_str("type")])))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_eq(&mcall(&{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "lower", &[]), &head.clone()))) {
    continue;
    }
    if py_truthy(&_said_as_a_state(&[head.clone(), text.clone()])) {
    continue;
    }
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _said_as_a_state(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut head: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "head") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: head"))) };
    let mut text: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "text") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: text"))) };
    let mut i: Py = Py::None;
    let mut j: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when every time the chat said this word, it was after a copula: \"im skint\".");
    said = mcall(&mcall(&mcall(&text.clone(), "lower", &[]), "replace", &[py_str("’"), py_str("'")]), "split", &[]);
    seen = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    word = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_eq(&mcall(&word.clone(), "strip", &[py_str(".,!?;:\"'")]), &head.clone()))) {
    continue;
    }
    seen = Py::Bool(true);
    j = sub(&i.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = ge(&j.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_FILLER(), &mcall(&geti(&said.clone(), &j.clone()), "strip", &[py_str(".,!?;:\"'")]))) } }) {
    let __aug3 = sub(&j.clone(), &Py::Int(1));
    j = __aug3;
    }
    if py_truthy(&{ let __b = lt(&j.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_COPULA(), &mcall(&geti(&said.clone(), &j.clone()), "strip", &[py_str(".,!?;:\"'")]))) } }) {
    return Py::Bool(false);
    }
    }
    return seen.clone();
    Py::None
}
pub fn in_a_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut phrases: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "phrases") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: phrases"))) };
    let mut items: Py = Py::None;
    let mut one: Py = Py::None;
    let _ = py_str("The things as one clause: \"your sister, the mince and your manager\".\n\n    Empty when there is nothing to name, so a caller can tell a chat with something on the table\n    from one without: the reply that lists the things is only honest when the store has them.\n    ");
    items = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = phrases.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut one: Py = __item.clone(); if py_truthy(&mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])) {  __out.push(mcall(&call_builtin("str", &[one.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&items.clone()))) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[items.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)))) {
    return geti(&items.clone(), &Py::Int(0));
    }
    return add(&add(&mcall(&py_str(", "), "join", &[slice_or_raise(&items.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&items.clone(), &neg(&Py::Int(1))));
    Py::None
}
pub fn topic_index(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { Py::None };
    let mut index: Py = Py::None;
    let mut keep: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut require_entity: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Where in `user_lines` the newest line that names the thing we are on sits, or None.\n\n    `about` reads the phrase out of that line; this is the same search, keeping the position so a\n    caller can tell an ask about the thing we are on from one asked two subjects ago.\n    ");
    keep = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[user_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    index = __unpk2[0].clone();
    let __unpk3 = py_unpack(&__unpk2[1].clone(), 2, None);
    text = __unpk3[0].clone();
    reading = __unpk3[1].clone();
    if py_truthy(&{ let __b = _is_thin(&[reading.clone()]); if py_truthy(&__b) { __b } else { _is_memory_line(&[reading.clone()]) } }) {
    continue;
    }
    if py_truthy(&_asks_about_the_chat(&[text.clone(), memory.clone(), skip.clone()])) {
    continue;
    }
    let _ = mcall(&keep.clone(), "append", &[py_tuple(vec![index.clone(), text.clone(), reading.clone()])]);
    }
    let __iter4 = py_iter_or_raise(&py_tuple(vec![Py::Bool(true), Py::Bool(false)]));
    for __item in __iter4 {
    require_entity = __item.clone();
    let __iter5 = py_iter_or_raise(&call_builtin("reversed", &[keep.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 3, None);
    index = __unpk6[0].clone();
    text = __unpk6[1].clone();
    reading = __unpk6[2].clone();
    if py_truthy(&{ let __b = require_entity.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_named_entities(&[reading.clone()]))) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&require_entity.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("intent")]), &py_str("feeling"))); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("topic_own")]), &py_str("feelings"))) } } } }) {
    continue;
    }
    if py_truthy(&_phrase_for(&[text.clone(), reading.clone()])) {
    return index.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn last_ask(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { Py::None };
    let mut not_before: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "not_before") { v.clone() }
    else { Py::None };
    let mut index: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The last thing the user asked, restated, or None.\n\n    `not_before` is a `user_lines` position: an ask older than that was asked about something\n    else, and saying it back beside the thing we are on now is a claim about them they never made\n    (\"we were on the bathroom — you asked how long it will take\", from a cake two subjects ago).\n    ");
    lines = user_lines(&[memory.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), neg(&Py::Int(1)), neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&not_before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { lt(&index.clone(), &not_before.clone()) } }) {
    break;
    }
    let __unpk2 = py_unpack(&geti(&lines.clone(), &index.clone()), 2, None);
    text = __unpk2[0].clone();
    reading = __unpk2[1].clone();
    if py_truthy(&{ let __b = _is_memory_line(&[reading.clone()]); if py_truthy(&__b) { __b } else { _asks_about_the_chat(&[text.clone(), memory.clone(), skip.clone()]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&text.clone(), &py_str("?"))) } }) {
    said = restate(&[text.clone()]);
    if py_truthy(&said.clone()) {
    return said.clone();
    }
    }
    }
    return Py::None;
    Py::None
}
pub fn _only_a_detail(args: &[Py]) -> Py {
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
    let mut got: Py = Py::None;
    let mut line: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the whole line is a detail about something already on the table.");
    line = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "strip", &[py_str(" .!?")]);
    rest = mcall(&g_DETAIL_OPENER(), "sub", &[py_str(""), line.clone()]);
    if py_truthy(&Py::Bool(py_eq(&rest.clone(), &line.clone()))) {
    return Py::Bool(false);
    }
    got = crate::gen::memory::detail::the_detail(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&got.clone()))) {
    return Py::Bool(false);
    }
    words = py_func("lambda", move |args: &[Py]| { let got = got.clone(); let line = line.clone(); let rest = rest.clone(); let text = text.clone(); let words = words.clone(); let t: Py = __kwp(args, 0, "t").unwrap_or(Py::None); { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = t.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "lower", &[])); } py_list(__out) })() } });
    return Py::Bool(py_eq(&py_call(&words, &[got.clone()]), &py_call(&words, &[rest.clone()])));
    Py::None
}
pub fn last_said(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { Py::None };
    let mut past_a_closing: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The last substantial thing the user told the bot, restated, or None.\n\n    A closing is walked past: \"night\" after a chat about a knee is not what the chat was on,\n    and handing it back as the last thing said (\"you are off to bed\") answers a question nobody\n    asked while the real line sat one turn back.\n\n    Where that walk stepped over a closing, the chat had already ended, and the thing the chat\n    was on is the better answer (\"we were on the deposit\" rather than \"you said work is mad at\n    the moment\", which is only what was said just before they went to bed). So nothing is handed\n    back then, and the caller falls through to the subject it already knows.\n    ");
    let __unpk1 = py_unpack(&last_said_past_closings(&[memory.clone(), skip.clone()]), 2, None);
    said = __unpk1[0].clone();
    past_a_closing = __unpk1[1].clone();
    if py_truthy(&{ let __b = past_a_closing.clone(); if !py_truthy(&__b) { __b } else { { let __b = said.clone(); if !py_truthy(&__b) { __b } else { about(&[memory.clone(), skip.clone()]) } } } }) {
    return Py::None;
    }
    return said.clone();
    Py::None
}
pub fn last_said_past_closings(args: &[Py]) -> Py {
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
    let mut skip: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "skip") { v.clone() }
    else { Py::None };
    let mut past_a_closing: Py = Py::None;
    let mut reading: Py = Py::None;
    let mut said: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The last thing said, plus whether a closing had to be walked past to reach it.\n\n    A recap that had to step over \"ok goodnight\" is answering about a chat that ended, so the\n    subject the chat was on (\"we were on the deposit\") is the better thing to hand back than the\n    last line before the closing. A recap with nothing to step over answers with the last line.\n    ");
    past_a_closing = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[user_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    text = __unpk2[0].clone();
    reading = __unpk2[1].clone();
    if py_truthy(&{ let __b = _is_thin(&[reading.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question"))); if py_truthy(&__b) { __b } else { _is_memory_line(&[reading.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&signs_off(&[text.clone()])) {
    past_a_closing = Py::Bool(true);
    continue;
    }
    if py_truthy(&_asks_about_the_chat(&[text.clone(), memory.clone(), skip.clone()])) {
    continue;
    }
    if py_truthy(&_only_a_detail(&[text.clone()])) {
    continue;
    }
    said = restate(&[text.clone()]);
    if py_truthy(&said.clone()) {
    return py_tuple(vec![said.clone(), past_a_closing.clone()]);
    }
    }
    return py_tuple(vec![Py::None, past_a_closing.clone()]);
    Py::None
}
pub fn named(args: &[Py]) -> Py {
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
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(2) };
    let mut _text: Py = Py::None;
    let mut entity: Py = Py::None;
    let mut n: Py = Py::None;
    let mut name: Py = Py::None;
    let mut out: Py = Py::None;
    let mut reading: Py = Py::None;
    let _ = py_str("The things and places the user has named, newest first, each used once.");
    out = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[user_lines(&[memory.clone()])]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    _text = __unpk2[0].clone();
    reading = __unpk2[1].clone();
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter3 {
    entity = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[entity.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    name = mcall(&{ let __b = mcall(&entity.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&entity.clone(), "get", &[py_str("type")]), &py_str("thing"))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WEAK(), &mcall(&name.clone(), "lower", &[]))) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FLIP(), &mcall(&name.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { le(&call_builtin("len", &[mcall(&name.clone(), "strip", &[py_str(".,")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    continue;
    }
    if py_truthy(&mcall(&mcall(&name.clone(), "lower", &[]), "startswith", &[py_str("my ")])) {
    name = add(&py_str("your "), &slice_or_raise(&name.clone(), &Py::Int(3), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut n: Py = __item.clone(); __out.push(mcall(&n.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&name.clone(), "lower", &[])))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[name.clone()]);
    if py_truthy(&ge(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
    return out.clone();
    }
    }
    }
    return out.clone();
    Py::None
}
pub fn checking_the_verdict(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut verdict: Py = Py::None;
    let _ = py_str("The verdict said again, or None.\n\n    \"is that alright\" after \"No: a satay is made with nuts, and that is off the table for you\"\n    got the ack pool: *Sounds good.* The bot agreed with a plan it had just ruled out, and the\n    person is left thinking the satay is fine. A check with no subject of its own is about the\n    answer the bot just gave, so the answer holds: the same direction, and the reason it gave,\n    in a sentence short enough to be an answer to a yes-or-no question rather than a repeat.\n\n    Only a line that names nothing and asks nothing else gets here, and only when the bot's last\n    line was a verdict with a reason in it. Everything else is left to the modules that answered\n    it before, so this can add a reply and never take one away.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CHECK_VERDICT(), "match", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    last = py_str("");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { turns.clone() } }) {
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    }
    verdict = mcall(&g_VERDICT_SAID(), "match", &[last.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&verdict.clone()))) {
    return Py::None;
    }
    reason = _first_clause(&[mcall(&verdict.clone(), "group", &[Py::Int(2)])]);
    if py_truthy(&Py::Bool(!py_truthy(&reason.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&mcall(&verdict.clone(), "group", &[Py::Int(1)]), "lower", &[]), &py_str("yes")))) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reason".to_string(), reason.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_next_frame(&[g_YES_FRAMES(), g_YES_USED(), last.clone()]), "format", args))(&__args) };
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("reason".to_string(), reason.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&_next_frame(&[g_NO_FRAMES(), g_NO_USED(), last.clone()]), "format", args))(&__args) };
    Py::None
}
pub fn checking_a_fact(args: &[Py]) -> Py {
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
    let mut answer: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The answer on screen confirmed, or None.\n\n    Only for a line that asks nothing else, and only when the answer on screen is a fact: the\n    turn before it was answered from a knowledge entry, or it is a confirmation this function\n    itself gave (a check asked twice is answered, not parroted). A store line, an echo or a\n    question back is not a fact to confirm, so those lines keep the reply they had.\n\n    The answer is not read out again. The arbiter passes over a reply that says the last answer\n    again behind a nod, and rightly: the person has those words in front of them. What they are\n    asking for is the bot standing behind them.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::None;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .!?,")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CHECK_A_FACT(), "match", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &neg(&Py::Int(1))) } else { py_dict(vec![]) };
    answer = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { mcall(&answer.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_fact_from(&[answer.clone()])))) {
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&answer.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NO_FACT_SAID(), "match", &[answer.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_FACT(), "search", &[answer.clone()]) } } } }) {
    return Py::None;
    }
    let mut topics: Py = crate::gen::memory::topics::module_obj();
    if py_truthy(&Py::Bool(py_is(&mcall(&topics.clone(), "an_entry_for", &[call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]), &Py::None))) {
    return Py::None;
    }
    }
    return _a_frame_not_said(&[g_FACT_FRAMES(), memory.clone()]);
    Py::None
}
pub fn _fact_from(args: &[Py]) -> Py {
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
    let mut frame: Py = Py::None;
    let mut low: Py = Py::None;
    let _ = py_str("True when the answer on screen is a confirmation this function gave a turn ago.");
    low = mcall(&answer.clone(), "lower", &[]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_FACT_FRAMES()) { let mut frame: Py = __item.clone(); __out.push(mcall(&low.clone(), "startswith", &[mcall(&mcall(&frame.clone(), "lower", &[]), "rstrip", &[py_str(".")])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _a_frame_not_said(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frames: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frames") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frames"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut frame: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let mut stem: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The first of `frames` the bot has not said in this conversation yet.\n\n    A check asked a fourth time in a row would otherwise come back to the frame used first, and\n    the arbiter passes over a reply already on screen: the person would get the whole answer read\n    out again instead of a confirmation.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    said = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])); } } py_list(__out) })() }, &neg(&Py::Int(6)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&frames.clone());
    for __item in __iter1 {
    frame = __item.clone();
    stem = mcall(&mcall(&frame.clone(), "lower", &[]), "rstrip", &[py_str(".")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut line: Py = __item.clone(); __out.push(mcall(&line.clone(), "startswith", &[stem.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return frame.clone();
    }
    }
    return geti(&frames.clone(), &Py::Int(0));
    Py::None
}
pub fn _next_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frames: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frames") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frames"))) };
    let mut used: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "used") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: used"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut going_to: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut stem: Py = Py::None;
    let _ = py_str("The frame this verdict is said in, moving on from the one the bot used last time.");
    stem = mcall(&mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]), "lower", &[]);
    let __iter1 = py_iter_or_raise(&used.clone());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    lead = __unpk2[0].clone();
    going_to = __unpk2[1].clone();
    if py_truthy(&mcall(&stem.clone(), "startswith", &[lead.clone()])) {
    return geti(&frames.clone(), &going_to.clone());
    }
    }
    return geti(&frames.clone(), &Py::Int(0));
    Py::None
}
pub fn _first_clause(args: &[Py]) -> Py {
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
    let mut limit: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "limit") { v.clone() }
    else { Py::Int(13) };
    let mut cut: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The first reason in a verdict: \"a satay is made with nuts\" out of a two-sentence answer.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    cut = geti(&{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("(?<=[.!?])\\s|;\\s|,\\s+(?:and|so|but)\\s")); __args.push(said.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }, &Py::Int(0));
    cut = mcall(&cut.clone(), "strip", &[py_str(" .,;:")]);
    words = mcall(&cut.clone(), "split", &[]);
    if py_truthy(&gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
    cut = mcall(&mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &limit.clone(), &Py::None)]), "rstrip", &[py_str(",;:")]);
    }
    return cut.clone();
    Py::None
}
pub fn _a_done_word(args: &[Py]) -> Py {
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
    let _ = py_str("Whether this word is a participle, so what follows \"i had\" is the doing, not the thing.");
    return { let __b = mcall(&word.clone(), "endswith", &[py_str("ed")]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DONE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::understanding::normalize::g_IRREGULAR_PARTICIPLES(), &word.clone())) } } } };
    Py::None
}
pub fn _the_verdict_they_gave(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The word the report used for how it went, or \"\".\n\n    \"i had a scan yesterday and it was fine\" has already answered the ask, so the reply would be\n    asking the person to say again what they just said.\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    match_ = mcall(&g_A_VERDICT(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    return mcall(&{ let __b = mcall(&match_.clone(), "group", &[py_str("v")]); if py_truthy(&__b) { __b } else { { let __b = mcall(&match_.clone(), "group", &[py_str("w")]); if py_truthy(&__b) { __b } else { py_str("") } } } }, "lower", &[]);
    Py::None
}
pub fn _the_thing_they_had(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The thing the line says they had, did or went to, in a few words, or \"\".\n\n    \"i had an interview today\" is \"interview\", \"i had a dentist appointment today\" is \"dentist\n    appointment\", and \"my first day at the new job was today\" is \"first day at the new job\".\n    ");
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&low.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_HAPPENED(), "search", &[low.clone()]))) } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_IT_WAS(), g_HAD_IT()]));
    for __item in __iter1 {
    pattern = __item.clone();
    match_ = mcall(&pattern.clone(), "search", &[low.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    continue;
    }
    words = py_list(vec![]);
    let __iter2 = py_iter_or_raise(&mcall(&mcall(&match_.clone(), "group", &[py_str("thing")]), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_THING_END(), &word.clone()))) {
    break;
    }
    let _ = mcall(&words.clone(), "append", &[word.clone()]);
    }
    thing = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_THE_THING(), &thing.clone())) } }) {
    continue;
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_THING_DONE(), &geti(&words.clone(), &Py::Int(0)))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NEVER_A_THING_THAT_WENT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_STATE_ITSELF(), &geti(&words.clone(), &neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_NAME_FOR_IT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_DESCRIPTION(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THING_PARTICLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { _a_done_word(&[geti(&words.clone(), &neg(&Py::Int(1)))]) } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    thing = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_THE_THING(), &thing.clone())) } }) {
    continue;
    }
    } else {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(_a_done_word(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    }
    return thing.clone();
    }
    return py_str("");
    Py::None
}
pub fn went(args: &[Py]) -> Py {
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
    let mut asked_about: Py = Py::None;
    let mut asked_before: Py = Py::None;
    let mut frame: Py = Py::None;
    let mut frames: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turn: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut verdict: Py = Py::None;
    let _ = py_str("The reply to \"how did it go\" after they told the chat about a thing they had today.\n\n    The ask is about the person's own day, and the bot was not there: the honest reply is to ask\n    them. Only their own report of a thing that is over can be the subject, and only the ask\n    shapes above are read, so a how-to (\"how did you fix the boiler\", \"how do i get there\") is\n    left to the modules that answer it. When the bot's own last line was already this question,\n    it is not asked a second time.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    return Py::None;
    }
    said = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&text.clone(), "lower", &[]), "split", &[])]), "strip", &[py_str(" ?.!,")]);
    match_ = mcall(&g_WENT_ASK(), "match", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_turns(&[memory.clone()])) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(turn.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    asked_before = _asked_already(&[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("reply")]) } }]);
    thing = py_str("");
    verdict = py_str("");
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    turn = __item.clone();
    thing = _the_thing_they_had(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    if py_truthy(&thing.clone()) {
    verdict = _the_verdict_they_gave(&[mcall(&turn.clone(), "get", &[py_str("user")])]);
    break;
    }
    }
    asked_about = mcall(&{ let __b = mcall(&mcall(&match_.clone(), "groupdict", &[]), "get", &[py_str("thing")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&asked_about.clone()) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&thing.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_NOT_THE_THING(), &asked_about.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__A_THING_DONE(), &asked_about.clone())) } } } }) {
    return Py::None;
    }
    thing = asked_about.clone();
    }
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    if py_truthy(&verdict.clone()) {
    frames = if py_truthy(&Py::Bool(py_contains(&g__WENT_WELL(), &verdict.clone()))) { g_WENT_TOLD_WELL() } else { g_WENT_TOLD_BAD() };
    return crate::gen::compose::pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut frame: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("v".to_string(), verdict.clone()), ("V".to_string(), mcall(&verdict.clone(), "capitalize", &[]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone()]);
    }
    if py_truthy(&asked_before.clone()) {
    return crate::gen::compose::pick(&[call_builtin("list", &[g_WENT_AGAIN()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    }
    return crate::gen::compose::pick(&[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WENT_FRAMES()) { let mut frame: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone()]);
    Py::None
}
pub fn _asked_already(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the bot's own line was this question, so the chat does not ask it twice.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&said.clone(), "endswith", &[py_str("?")])))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[mcall(&g_WENT_ASK(), "match", &[mcall(&mcall(&said.clone(), "rstrip", &[py_str("?")]), "strip", &[])])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { call_builtin("bool", &[stdlib_call("re", "search", &[py_str("how (?:did|was) (?:it|that|the|your)\\b.*\\bgo\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)) } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.thread".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADDRESSEE" => Some(g_ADDRESSEE()),
            "ADVERB" => Some(g_ADVERB()),
            "ASK_VERB" => Some(g_ASK_VERB()),
            "ASK_WORD" => Some(g_ASK_WORD()),
            "ASK_WORDS" => Some(g_ASK_WORDS()),
            "AUX" => Some(g_AUX()),
            "A_CHOICE" => Some(g_A_CHOICE()),
            "A_VERDICT" => Some(g_A_VERDICT()),
            "BARE_VERBS" => Some(g_BARE_VERBS()),
            "CALENDAR" => Some(g_CALENDAR()),
            "CHECK_A_FACT" => Some(g_CHECK_A_FACT()),
            "CHECK_VERDICT" => Some(g_CHECK_VERDICT()),
            "CLOSINGS" => Some(g_CLOSINGS()),
            "CLOSING_FILLER" => Some(g_CLOSING_FILLER()),
            "CONTRACTIONS" => Some(g_CONTRACTIONS()),
            "COPULA" => Some(g_COPULA()),
            "DETAIL_OPENER" => Some(g_DETAIL_OPENER()),
            "DETERMINERS" => Some(g_DETERMINERS()),
            "DONE_WORDS" => Some(g_DONE_WORDS()),
            "DROPPED_SUBJECT" => Some(g_DROPPED_SUBJECT()),
            "EXPAND" => Some(g_EXPAND()),
            "FACT_FRAMES" => Some(g_FACT_FRAMES()),
            "FIRST_PERSON_AUX" => Some(g_FIRST_PERSON_AUX()),
            "FLIP" => Some(g_FLIP()),
            "HAD_IT" => Some(g_HAD_IT()),
            "HAPPENED" => Some(g_HAPPENED()),
            "IS_NOT_POSSESSIVE" => Some(g_IS_NOT_POSSESSIVE()),
            "IT_WAS" => Some(g_IT_WAS()),
            "LEAD" => Some(g_LEAD()),
            "LOST_SUBJECT" => Some(g_LOST_SUBJECT()),
            "MEASURE" => Some(g_MEASURE()),
            "MODAL" => Some(g_MODAL()),
            "NEVER_A_THING_THAT_WENT" => Some(g_NEVER_A_THING_THAT_WENT()),
            "NON_NOUN" => Some(g_NON_NOUN()),
            "NOT_A_FACT" => Some(g_NOT_A_FACT()),
            "NOT_A_THING" => Some(g_NOT_A_THING()),
            "NOT_THE_THING" => Some(g_NOT_THE_THING()),
            "NO_FACT_SAID" => Some(g_NO_FACT_SAID()),
            "NO_FRAMES" => Some(g_NO_FRAMES()),
            "NO_USED" => Some(g_NO_USED()),
            "NUMBER" => Some(g_NUMBER()),
            "NUMBER_WORD_HEADS" => Some(g_NUMBER_WORD_HEADS()),
            "ON_THE_TABLE_MISSES" => Some(g_ON_THE_TABLE_MISSES()),
            "ON_THE_TABLE_TURNS" => Some(g_ON_THE_TABLE_TURNS()),
            "PAST" => Some(g_PAST()),
            "PLACE_NAMES" => Some(g_PLACE_NAMES()),
            "POINTING_WORD" => Some(g_POINTING_WORD()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POSSESSIVE" => Some(g_POSSESSIVE()),
            "PREDICATE" => Some(g_PREDICATE()),
            "PREP" => Some(g_PREP()),
            "PRONOUN" => Some(g_PRONOUN()),
            "QUALITY" => Some(g_QUALITY()),
            "RECEIPT_LONGEST" => Some(g_RECEIPT_LONGEST()),
            "RECEIPT_WORDS" => Some(g_RECEIPT_WORDS()),
            "SECOND_PERSON" => Some(g_SECOND_PERSON()),
            "SIGNS_OFF" => Some(g_SIGNS_OFF()),
            "STATE_FILLER" => Some(g_STATE_FILLER()),
            "SUBJECTISH" => Some(g_SUBJECTISH()),
            "SUBJECT_YOU" => Some(g_SUBJECT_YOU()),
            "THIN" => Some(g_THIN()),
            "THING_END" => Some(g_THING_END()),
            "THING_PARTICLE" => Some(g_THING_PARTICLE()),
            "THIRD" => Some(g_THIRD()),
            "VERB_BEFORE" => Some(g_VERB_BEFORE()),
            "VERB_WORDS" => Some(g_VERB_WORDS()),
            "VERDICT_SAID" => Some(g_VERDICT_SAID()),
            "WEAK" => Some(g_WEAK()),
            "WENT_AGAIN" => Some(g_WENT_AGAIN()),
            "WENT_ASK" => Some(g_WENT_ASK()),
            "WENT_FRAMES" => Some(g_WENT_FRAMES()),
            "WENT_FRAMES_NO_THING" => Some(g_WENT_FRAMES_NO_THING()),
            "WENT_TOLD_BAD" => Some(g_WENT_TOLD_BAD()),
            "WENT_TOLD_WELL" => Some(g_WENT_TOLD_WELL()),
            "WH" => Some(g_WH()),
            "WHAT_I_SAID" => Some(g_WHAT_I_SAID()),
            "YES_FRAMES" => Some(g_YES_FRAMES()),
            "YES_USED" => Some(g_YES_USED()),
            "_A_DESCRIPTION" => Some(g__A_DESCRIPTION()),
            "_A_NAME_FOR_IT" => Some(g__A_NAME_FOR_IT()),
            "_A_STATE_ITSELF" => Some(g__A_STATE_ITSELF()),
            "_A_THING_DONE" => Some(g__A_THING_DONE()),
            "_ONLY_THE_ARRIVAL" => Some(g__ONLY_THE_ARRIVAL()),
            "_WENT_WELL" => Some(g__WENT_WELL()),
            "_a_done_word" => Some(py_func("_a_done_word", |args: &[Py]| _a_done_word(args))),
            "_a_frame_not_said" => Some(py_func("_a_frame_not_said", |args: &[Py]| _a_frame_not_said(args))),
            "_agree" => Some(py_func("_agree", |args: &[Py]| _agree(args))),
            "_as_text" => Some(py_func("_as_text", |args: &[Py]| _as_text(args))),
            "_asked_already" => Some(py_func("_asked_already", |args: &[Py]| _asked_already(args))),
            "_asks_about_the_bot" => Some(py_func("_asks_about_the_bot", |args: &[Py]| _asks_about_the_bot(args))),
            "_asks_about_the_chat" => Some(py_func("_asks_about_the_chat", |args: &[Py]| _asks_about_the_chat(args))),
            "_behind_a_prep" => Some(py_func("_behind_a_prep", |args: &[Py]| _behind_a_prep(args))),
            "_capital" => Some(py_func("_capital", |args: &[Py]| _capital(args))),
            "_conditional" => Some(py_func("_conditional", |args: &[Py]| _conditional(args))),
            "_embed" => Some(py_func("_embed", |args: &[Py]| _embed(args))),
            "_expand" => Some(py_func("_expand", |args: &[Py]| _expand(args))),
            "_fact_from" => Some(py_func("_fact_from", |args: &[Py]| _fact_from(args))),
            "_first_clause" => Some(py_func("_first_clause", |args: &[Py]| _first_clause(args))),
            "_first_person_aux" => Some(py_func("_first_person_aux", |args: &[Py]| _first_person_aux(args))),
            "_flip" => Some(py_func("_flip", |args: &[Py]| _flip(args))),
            "_from_topics" => Some(py_func("_from_topics", |args: &[Py]| _from_topics(args))),
            "_groups" => Some(py_func("_groups", |args: &[Py]| _groups(args))),
            "_is_a_receipt" => Some(py_func("_is_a_receipt", |args: &[Py]| _is_a_receipt(args))),
            "_is_memory_line" => Some(py_func("_is_memory_line", |args: &[Py]| _is_memory_line(args))),
            "_is_subject_phrase" => Some(py_func("_is_subject_phrase", |args: &[Py]| _is_subject_phrase(args))),
            "_is_the_verb" => Some(py_func("_is_the_verb", |args: &[Py]| _is_the_verb(args))),
            "_is_thin" => Some(py_func("_is_thin", |args: &[Py]| _is_thin(args))),
            "_is_wh" => Some(py_func("_is_wh", |args: &[Py]| _is_wh(args))),
            "_keep_possessives" => Some(py_func("_keep_possessives", |args: &[Py]| _keep_possessives(args))),
            "_named_entities" => Some(py_func("_named_entities", |args: &[Py]| _named_entities(args))),
            "_next_frame" => Some(py_func("_next_frame", |args: &[Py]| _next_frame(args))),
            "_np_end" => Some(py_func("_np_end", |args: &[Py]| _np_end(args))),
            "_numbers" => Some(py_func("_numbers", |args: &[Py]| _numbers(args))),
            "_only_a_detail" => Some(py_func("_only_a_detail", |args: &[Py]| _only_a_detail(args))),
            "_past" => Some(py_func("_past", |args: &[Py]| _past(args))),
            "_phrase_for" => Some(py_func("_phrase_for", |args: &[Py]| _phrase_for(args))),
            "_place_case" => Some(py_func("_place_case", |args: &[Py]| _place_case(args))),
            "_qualifier" => Some(py_func("_qualifier", |args: &[Py]| _qualifier(args))),
            "_restore_case" => Some(py_func("_restore_case", |args: &[Py]| _restore_case(args))),
            "_said_as_a_state" => Some(py_func("_said_as_a_state", |args: &[Py]| _said_as_a_state(args))),
            "_said_in_the_chat" => Some(py_func("_said_in_the_chat", |args: &[Py]| _said_in_the_chat(args))),
            "_same_spelling" => Some(py_func("_same_spelling", |args: &[Py]| _same_spelling(args))),
            "_strip_lead" => Some(py_func("_strip_lead", |args: &[Py]| _strip_lead(args))),
            "_subjects_at" => Some(py_func("_subjects_at", |args: &[Py]| _subjects_at(args))),
            "_the_phrase_around" => Some(py_func("_the_phrase_around", |args: &[Py]| _the_phrase_around(args))),
            "_the_thing_they_had" => Some(py_func("_the_thing_they_had", |args: &[Py]| _the_thing_they_had(args))),
            "_the_verdict_they_gave" => Some(py_func("_the_verdict_they_gave", |args: &[Py]| _the_verdict_they_gave(args))),
            "_third" => Some(py_func("_third", |args: &[Py]| _third(args))),
            "_truncated" => Some(py_func("_truncated", |args: &[Py]| _truncated(args))),
            "_turns" => Some(py_func("_turns", |args: &[Py]| _turns(args))),
            "_verb_after_a_subject" => Some(py_func("_verb_after_a_subject", |args: &[Py]| _verb_after_a_subject(args))),
            "_with_wh" => Some(py_func("_with_wh", |args: &[Py]| _with_wh(args))),
            "_with_you" => Some(py_func("_with_you", |args: &[Py]| _with_you(args))),
            "_word_behind_prep" => Some(py_func("_word_behind_prep", |args: &[Py]| _word_behind_prep(args))),
            "about" => Some(py_func("about", |args: &[Py]| about(args))),
            "asks_what_i_said" => Some(py_func("asks_what_i_said", |args: &[Py]| asks_what_i_said(args))),
            "carries_subject" => Some(py_func("carries_subject", |args: &[Py]| carries_subject(args))),
            "checking_a_fact" => Some(py_func("checking_a_fact", |args: &[Py]| checking_a_fact(args))),
            "checking_the_verdict" => Some(py_func("checking_the_verdict", |args: &[Py]| checking_the_verdict(args))),
            "earlier_turn" => Some(py_func("earlier_turn", |args: &[Py]| earlier_turn(args))),
            "in_a_list" => Some(py_func("in_a_list", |args: &[Py]| in_a_list(args))),
            "last_ask" => Some(py_func("last_ask", |args: &[Py]| last_ask(args))),
            "last_said" => Some(py_func("last_said", |args: &[Py]| last_said(args))),
            "last_said_past_closings" => Some(py_func("last_said_past_closings", |args: &[Py]| last_said_past_closings(args))),
            "named" => Some(py_func("named", |args: &[Py]| named(args))),
            "nothing_said" => Some(py_func("nothing_said", |args: &[Py]| nothing_said(args))),
            "on_the_table" => Some(py_func("on_the_table", |args: &[Py]| on_the_table(args))),
            "only_the_arrival" => Some(py_func("only_the_arrival", |args: &[Py]| only_the_arrival(args))),
            "points_at_a_word_alone" => Some(py_func("points_at_a_word_alone", |args: &[Py]| points_at_a_word_alone(args))),
            "points_back" => Some(py_func("points_back", |args: &[Py]| points_back(args))),
            "restate" => Some(py_func("restate", |args: &[Py]| restate(args))),
            "signs_off" => Some(py_func("signs_off", |args: &[Py]| signs_off(args))),
            "thin" => Some(py_func("thin", |args: &[Py]| thin(args))),
            "topic_index" => Some(py_func("topic_index", |args: &[Py]| topic_index(args))),
            "user_lines" => Some(py_func("user_lines", |args: &[Py]| user_lines(args))),
            "went" => Some(py_func("went", |args: &[Py]| went(args))),
            "your_words" => Some(py_func("your_words", |args: &[Py]| your_words(args))),
            _ => None,
        }),
        set: None,
    }))
}

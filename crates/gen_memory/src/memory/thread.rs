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


thread_local! { static G_ADDRESSEE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ADVERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_VERB: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ASK_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_CHOICE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_A_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_VERBS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CALENDAR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_A_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHECK_VERDICT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSINGS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSING_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONTRACTIONS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_COPULA: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETAIL_OPENER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DETERMINERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DONE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DROPPED_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_EXPAND: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FACT_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FIRST_PERSON_AUX: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_FLIP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAD_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HAPPENED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IRREGULAR_PARTICIPLES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IS_NOT_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IT_WAS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MEASURE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MODAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEGATIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEVER_A_THING_THAT_WENT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NON_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_FACT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THE_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_FACT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NO_USED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NUMBER_WORD_HEADS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_TABLE_MISSES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ON_THE_TABLE_TURNS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PAST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_NAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PLACE_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTING_WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POINTS_BACK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSITIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_POSSESSIVE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREDICATE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PRONOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_QUALITY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECEIPT_LONGEST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_RECEIPT_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SECOND_PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SIGNS_OFF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STATE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECTISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUBJECT_YOU: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_END: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THING_PARTICLE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_THIRD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERBISH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_BEFORE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_FORMS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERB_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VERDICT_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WEAK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_AGAIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_ASK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_FRAMES_NO_THING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_TOLD_BAD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WENT_TOLD_WELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WH: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_WHAT_I_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_FRAMES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_YES_USED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_DESCRIPTION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_NAME_FOR_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_STATE_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__A_THING_DONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__ONLY_THE_ARRIVAL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WENT_WELL: RefCell<Py> = const { RefCell::new(Py::None) }; }
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
pub fn g_ADDRESSEE() -> Py { ensure_init(); G_ADDRESSEE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADDRESSEE(v: Py) { ensure_init(); G_ADDRESSEE.with(|c| *c.borrow_mut() = v); }
pub fn g_ADVERB() -> Py { ensure_init(); G_ADVERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADVERB(v: Py) { ensure_init(); G_ADVERB.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_VERB() -> Py { ensure_init(); G_ASK_VERB.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_VERB(v: Py) { ensure_init(); G_ASK_VERB.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORD() -> Py { ensure_init(); G_ASK_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORD(v: Py) { ensure_init(); G_ASK_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_ASK_WORDS() -> Py { ensure_init(); G_ASK_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ASK_WORDS(v: Py) { ensure_init(); G_ASK_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AUX() -> Py { ensure_init(); G_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AUX(v: Py) { ensure_init(); G_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_A_CHOICE() -> Py { ensure_init(); G_A_CHOICE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_CHOICE(v: Py) { ensure_init(); G_A_CHOICE.with(|c| *c.borrow_mut() = v); }
pub fn g_A_VERDICT() -> Py { ensure_init(); G_A_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_A_VERDICT(v: Py) { ensure_init(); G_A_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_VERBS() -> Py { ensure_init(); G_BARE_VERBS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_VERBS(v: Py) { ensure_init(); G_BARE_VERBS.with(|c| *c.borrow_mut() = v); }
pub fn g_CALENDAR() -> Py { ensure_init(); G_CALENDAR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CALENDAR(v: Py) { ensure_init(); G_CALENDAR.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_A_FACT() -> Py { ensure_init(); G_CHECK_A_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_A_FACT(v: Py) { ensure_init(); G_CHECK_A_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_CHECK_VERDICT() -> Py { ensure_init(); G_CHECK_VERDICT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHECK_VERDICT(v: Py) { ensure_init(); G_CHECK_VERDICT.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSINGS() -> Py { ensure_init(); G_CLOSINGS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSINGS(v: Py) { ensure_init(); G_CLOSINGS.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSING_FILLER() -> Py { ensure_init(); G_CLOSING_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSING_FILLER(v: Py) { ensure_init(); G_CLOSING_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_CONTRACTIONS() -> Py { ensure_init(); G_CONTRACTIONS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONTRACTIONS(v: Py) { ensure_init(); G_CONTRACTIONS.with(|c| *c.borrow_mut() = v); }
pub fn g_COPULA() -> Py { ensure_init(); G_COPULA.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_COPULA(v: Py) { ensure_init(); G_COPULA.with(|c| *c.borrow_mut() = v); }
pub fn g_DETAIL_OPENER() -> Py { ensure_init(); G_DETAIL_OPENER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETAIL_OPENER(v: Py) { ensure_init(); G_DETAIL_OPENER.with(|c| *c.borrow_mut() = v); }
pub fn g_DETERMINERS() -> Py { ensure_init(); G_DETERMINERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DETERMINERS(v: Py) { ensure_init(); G_DETERMINERS.with(|c| *c.borrow_mut() = v); }
pub fn g_DONE_WORDS() -> Py { ensure_init(); G_DONE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DONE_WORDS(v: Py) { ensure_init(); G_DONE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_DROPPED_SUBJECT() -> Py { ensure_init(); G_DROPPED_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DROPPED_SUBJECT(v: Py) { ensure_init(); G_DROPPED_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_EXPAND() -> Py { ensure_init(); G_EXPAND.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_EXPAND(v: Py) { ensure_init(); G_EXPAND.with(|c| *c.borrow_mut() = v); }
pub fn g_FACT_FRAMES() -> Py { ensure_init(); G_FACT_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FACT_FRAMES(v: Py) { ensure_init(); G_FACT_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_FIRST_PERSON_AUX() -> Py { ensure_init(); G_FIRST_PERSON_AUX.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FIRST_PERSON_AUX(v: Py) { ensure_init(); G_FIRST_PERSON_AUX.with(|c| *c.borrow_mut() = v); }
pub fn g_FLIP() -> Py { ensure_init(); G_FLIP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_FLIP(v: Py) { ensure_init(); G_FLIP.with(|c| *c.borrow_mut() = v); }
pub fn g_HAD_IT() -> Py { ensure_init(); G_HAD_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAD_IT(v: Py) { ensure_init(); G_HAD_IT.with(|c| *c.borrow_mut() = v); }
pub fn g_HAPPENED() -> Py { ensure_init(); G_HAPPENED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HAPPENED(v: Py) { ensure_init(); G_HAPPENED.with(|c| *c.borrow_mut() = v); }
pub fn g_IRREGULAR_PARTICIPLES() -> Py { ensure_init(); G_IRREGULAR_PARTICIPLES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IRREGULAR_PARTICIPLES(v: Py) { ensure_init(); G_IRREGULAR_PARTICIPLES.with(|c| *c.borrow_mut() = v); }
pub fn g_IS_NOT_POSSESSIVE() -> Py { ensure_init(); G_IS_NOT_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IS_NOT_POSSESSIVE(v: Py) { ensure_init(); G_IS_NOT_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_IT_WAS() -> Py { ensure_init(); G_IT_WAS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IT_WAS(v: Py) { ensure_init(); G_IT_WAS.with(|c| *c.borrow_mut() = v); }
pub fn g_LEAD() -> Py { ensure_init(); G_LEAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEAD(v: Py) { ensure_init(); G_LEAD.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_SUBJECT() -> Py { ensure_init(); G_LOST_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_SUBJECT(v: Py) { ensure_init(); G_LOST_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g_MEASURE() -> Py { ensure_init(); G_MEASURE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MEASURE(v: Py) { ensure_init(); G_MEASURE.with(|c| *c.borrow_mut() = v); }
pub fn g_MODAL() -> Py { ensure_init(); G_MODAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MODAL(v: Py) { ensure_init(); G_MODAL.with(|c| *c.borrow_mut() = v); }
pub fn g_NEGATIVE() -> Py { ensure_init(); G_NEGATIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE(v: Py) { ensure_init(); G_NEGATIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEVER_A_THING_THAT_WENT() -> Py { ensure_init(); G_NEVER_A_THING_THAT_WENT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEVER_A_THING_THAT_WENT(v: Py) { ensure_init(); G_NEVER_A_THING_THAT_WENT.with(|c| *c.borrow_mut() = v); }
pub fn g_NON_NOUN() -> Py { ensure_init(); G_NON_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NON_NOUN(v: Py) { ensure_init(); G_NON_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_FACT() -> Py { ensure_init(); G_NOT_A_FACT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_FACT(v: Py) { ensure_init(); G_NOT_A_FACT.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_THING() -> Py { ensure_init(); G_NOT_A_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_THING(v: Py) { ensure_init(); G_NOT_A_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THE_THING() -> Py { ensure_init(); G_NOT_THE_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THE_THING(v: Py) { ensure_init(); G_NOT_THE_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_FACT_SAID() -> Py { ensure_init(); G_NO_FACT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_FACT_SAID(v: Py) { ensure_init(); G_NO_FACT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_FRAMES() -> Py { ensure_init(); G_NO_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_FRAMES(v: Py) { ensure_init(); G_NO_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_NO_USED() -> Py { ensure_init(); G_NO_USED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NO_USED(v: Py) { ensure_init(); G_NO_USED.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER() -> Py { ensure_init(); G_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER(v: Py) { ensure_init(); G_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_NUMBER_WORD_HEADS() -> Py { ensure_init(); G_NUMBER_WORD_HEADS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NUMBER_WORD_HEADS(v: Py) { ensure_init(); G_NUMBER_WORD_HEADS.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_TABLE_MISSES() -> Py { ensure_init(); G_ON_THE_TABLE_MISSES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_TABLE_MISSES(v: Py) { ensure_init(); G_ON_THE_TABLE_MISSES.with(|c| *c.borrow_mut() = v); }
pub fn g_ON_THE_TABLE_TURNS() -> Py { ensure_init(); G_ON_THE_TABLE_TURNS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ON_THE_TABLE_TURNS(v: Py) { ensure_init(); G_ON_THE_TABLE_TURNS.with(|c| *c.borrow_mut() = v); }
pub fn g_PAST() -> Py { ensure_init(); G_PAST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PAST(v: Py) { ensure_init(); G_PAST.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_NAMES() -> Py { ensure_init(); G_PLACE_NAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_NAMES(v: Py) { ensure_init(); G_PLACE_NAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_PLACE_WORDS() -> Py { ensure_init(); G_PLACE_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PLACE_WORDS(v: Py) { ensure_init(); G_PLACE_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTING_WORD() -> Py { ensure_init(); G_POINTING_WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTING_WORD(v: Py) { ensure_init(); G_POINTING_WORD.with(|c| *c.borrow_mut() = v); }
pub fn g_POINTS_BACK() -> Py { ensure_init(); G_POINTS_BACK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POINTS_BACK(v: Py) { ensure_init(); G_POINTS_BACK.with(|c| *c.borrow_mut() = v); }
pub fn g_POSITIVE() -> Py { ensure_init(); G_POSITIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSITIVE(v: Py) { ensure_init(); G_POSITIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_POSSESSIVE() -> Py { ensure_init(); G_POSSESSIVE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_POSSESSIVE(v: Py) { ensure_init(); G_POSSESSIVE.with(|c| *c.borrow_mut() = v); }
pub fn g_PREDICATE() -> Py { ensure_init(); G_PREDICATE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREDICATE(v: Py) { ensure_init(); G_PREDICATE.with(|c| *c.borrow_mut() = v); }
pub fn g_PREP() -> Py { ensure_init(); G_PREP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREP(v: Py) { ensure_init(); G_PREP.with(|c| *c.borrow_mut() = v); }
pub fn g_PRONOUN() -> Py { ensure_init(); G_PRONOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PRONOUN(v: Py) { ensure_init(); G_PRONOUN.with(|c| *c.borrow_mut() = v); }
pub fn g_QUALITY() -> Py { ensure_init(); G_QUALITY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_QUALITY(v: Py) { ensure_init(); G_QUALITY.with(|c| *c.borrow_mut() = v); }
pub fn g_RECEIPT_LONGEST() -> Py { ensure_init(); G_RECEIPT_LONGEST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECEIPT_LONGEST(v: Py) { ensure_init(); G_RECEIPT_LONGEST.with(|c| *c.borrow_mut() = v); }
pub fn g_RECEIPT_WORDS() -> Py { ensure_init(); G_RECEIPT_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_RECEIPT_WORDS(v: Py) { ensure_init(); G_RECEIPT_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_SECOND_PERSON() -> Py { ensure_init(); G_SECOND_PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SECOND_PERSON(v: Py) { ensure_init(); G_SECOND_PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g_SIGNS_OFF() -> Py { ensure_init(); G_SIGNS_OFF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SIGNS_OFF(v: Py) { ensure_init(); G_SIGNS_OFF.with(|c| *c.borrow_mut() = v); }
pub fn g_STATE_FILLER() -> Py { ensure_init(); G_STATE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STATE_FILLER(v: Py) { ensure_init(); G_STATE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECTISH() -> Py { ensure_init(); G_SUBJECTISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECTISH(v: Py) { ensure_init(); G_SUBJECTISH.with(|c| *c.borrow_mut() = v); }
pub fn g_SUBJECT_YOU() -> Py { ensure_init(); G_SUBJECT_YOU.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUBJECT_YOU(v: Py) { ensure_init(); G_SUBJECT_YOU.with(|c| *c.borrow_mut() = v); }
pub fn g_THIN() -> Py { ensure_init(); G_THIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIN(v: Py) { ensure_init(); G_THIN.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_END() -> Py { ensure_init(); G_THING_END.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_END(v: Py) { ensure_init(); G_THING_END.with(|c| *c.borrow_mut() = v); }
pub fn g_THING_PARTICLE() -> Py { ensure_init(); G_THING_PARTICLE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THING_PARTICLE(v: Py) { ensure_init(); G_THING_PARTICLE.with(|c| *c.borrow_mut() = v); }
pub fn g_THIRD() -> Py { ensure_init(); G_THIRD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_THIRD(v: Py) { ensure_init(); G_THIRD.with(|c| *c.borrow_mut() = v); }
pub fn g_VERBISH() -> Py { ensure_init(); G_VERBISH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERBISH(v: Py) { ensure_init(); G_VERBISH.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_BEFORE() -> Py { ensure_init(); G_VERB_BEFORE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_BEFORE(v: Py) { ensure_init(); G_VERB_BEFORE.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_FORMS() -> Py { ensure_init(); G_VERB_FORMS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_FORMS(v: Py) { ensure_init(); G_VERB_FORMS.with(|c| *c.borrow_mut() = v); }
pub fn g_VERB_WORDS() -> Py { ensure_init(); G_VERB_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERB_WORDS(v: Py) { ensure_init(); G_VERB_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_VERDICT_SAID() -> Py { ensure_init(); G_VERDICT_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VERDICT_SAID(v: Py) { ensure_init(); G_VERDICT_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_WEAK() -> Py { ensure_init(); G_WEAK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WEAK(v: Py) { ensure_init(); G_WEAK.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_AGAIN() -> Py { ensure_init(); G_WENT_AGAIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_AGAIN(v: Py) { ensure_init(); G_WENT_AGAIN.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_ASK() -> Py { ensure_init(); G_WENT_ASK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_ASK(v: Py) { ensure_init(); G_WENT_ASK.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_FRAMES() -> Py { ensure_init(); G_WENT_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_FRAMES(v: Py) { ensure_init(); G_WENT_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_FRAMES_NO_THING() -> Py { ensure_init(); G_WENT_FRAMES_NO_THING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_FRAMES_NO_THING(v: Py) { ensure_init(); G_WENT_FRAMES_NO_THING.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_TOLD_BAD() -> Py { ensure_init(); G_WENT_TOLD_BAD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_TOLD_BAD(v: Py) { ensure_init(); G_WENT_TOLD_BAD.with(|c| *c.borrow_mut() = v); }
pub fn g_WENT_TOLD_WELL() -> Py { ensure_init(); G_WENT_TOLD_WELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WENT_TOLD_WELL(v: Py) { ensure_init(); G_WENT_TOLD_WELL.with(|c| *c.borrow_mut() = v); }
pub fn g_WH() -> Py { ensure_init(); G_WH.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WH(v: Py) { ensure_init(); G_WH.with(|c| *c.borrow_mut() = v); }
pub fn g_WHAT_I_SAID() -> Py { ensure_init(); G_WHAT_I_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_WHAT_I_SAID(v: Py) { ensure_init(); G_WHAT_I_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_FRAMES() -> Py { ensure_init(); G_YES_FRAMES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_FRAMES(v: Py) { ensure_init(); G_YES_FRAMES.with(|c| *c.borrow_mut() = v); }
pub fn g_YES_USED() -> Py { ensure_init(); G_YES_USED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_YES_USED(v: Py) { ensure_init(); G_YES_USED.with(|c| *c.borrow_mut() = v); }
pub fn g__A_DESCRIPTION() -> Py { ensure_init(); G__A_DESCRIPTION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_DESCRIPTION(v: Py) { ensure_init(); G__A_DESCRIPTION.with(|c| *c.borrow_mut() = v); }
pub fn g__A_NAME_FOR_IT() -> Py { ensure_init(); G__A_NAME_FOR_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_NAME_FOR_IT(v: Py) { ensure_init(); G__A_NAME_FOR_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__A_STATE_ITSELF() -> Py { ensure_init(); G__A_STATE_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_STATE_ITSELF(v: Py) { ensure_init(); G__A_STATE_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g__A_THING_DONE() -> Py { ensure_init(); G__A_THING_DONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__A_THING_DONE(v: Py) { ensure_init(); G__A_THING_DONE.with(|c| *c.borrow_mut() = v); }
pub fn g__ONLY_THE_ARRIVAL() -> Py { ensure_init(); G__ONLY_THE_ARRIVAL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__ONLY_THE_ARRIVAL(v: Py) { ensure_init(); G__ONLY_THE_ARRIVAL.with(|c| *c.borrow_mut() = v); }
pub fn g__WENT_WELL() -> Py { ensure_init(); G__WENT_WELL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WENT_WELL(v: Py) { ensure_init(); G__WENT_WELL.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("The thread of the conversation: what we are on, and how to say the user's earlier lines.\n\nThe conversation lane owns this. A module that answers a question about the conversation\nitself (\"where were we?\", \"what did I just ask you?\") needs two things that no other module\nprovides:\n\n* `subject(memory)`: the thing the last few turns were about, in a phrase the bot can name\n  (\"the birthday cake\", \"the chicken\", \"Lisbon\"), read from the user's own lines.\n* `restate(text)`: the user's earlier line said again inside a sentence, in their words and\n  in grammar that fits (\"can i make it the day before\" -> \"whether you can make it the day\n  before\"; \"how long should i cook a 2kg chicken\" -> \"how long you should cook a 2kg\n  chicken\"). First person becomes second person, the question is embedded, nothing is\n  invented.\n\nBoth read `memory[\"turns\"]` and the shared reading of each line (`bot.understanding`), so they\nwork for any topic and any wording. Nothing here writes to memory or answers a turn.\n");
    s_NEGATIVE(gen_understanding::understanding::kinds::g_NEGATIVE());
    s_POSITIVE(gen_understanding::understanding::kinds::g_POSITIVE());
    s_VERB_FORMS(gen_understanding::understanding::kinds::g_VERB_FORMS());
    s_VERBISH(gen_understanding::understanding::kinds::g_VERBISH());
    s_IRREGULAR_PARTICIPLES(gen_understanding::understanding::normalize::g_IRREGULAR_PARTICIPLES());
    s_PLACE_WORDS(gen_understanding::understanding::topics::g_PLACES());
    s_THIN(call_builtin("frozenset", &[py_tuple(vec![py_str("ok"), py_str("okay"), py_str("k"), py_str("cool"), py_str("nice"), py_str("great"), py_str("good"), py_str("yeah"), py_str("yep"), py_str("yes"), py_str("no"), py_str("nah"), py_str("nope"), py_str("lol"), py_str("haha"), py_str("ha"), py_str("mm"), py_str("hmm"), py_str("right"), py_str("fair"), py_str("sure"), py_str("same"), py_str("true"), py_str("thanks"), py_str("thx"), py_str("ta"), py_str("cheers"), py_str("please")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSINGS(call_builtin("frozenset", &[py_tuple(vec![py_str("night"), py_str("nite"), py_str("goodnight"), py_str("goodnite"), py_str("nighty"), py_str("bed"), py_str("bedtime"), py_str("bye"), py_str("byebye"), py_str("cya"), py_str("sleep")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSING_FILLER(call_builtin("frozenset", &[py_tuple(vec![py_str("right"), py_str("ok"), py_str("okay"), py_str("well"), py_str("anyway"), py_str("im"), py_str("i"), py_str("am"), py_str("we"), py_str("were"), py_str("re"), py_str("going"), py_str("gonna"), py_str("gotta"), py_str("off"), py_str("to"), py_str("the"), py_str("a"), py_str("now"), py_str("so"), py_str("then"), py_str("just"), py_str("time"), py_str("for"), py_str("up")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SIGNS_OFF(stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("^(?:(?:"), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CLOSING_FILLER()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")\\s+)*(?:")), &mcall(&py_str("|"), "join", &[{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_CLOSINGS()); __kws.extend(vec![("key".to_string(), py_func("len", |args: &[Py]| call_builtin("len", args).unwrap_or_else(|e| exc::raise(e)))), ("reverse".to_string(), Py::Bool(true))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| call_builtin("sorted", args).unwrap_or_else(|e| exc::raise(e)))(&__args) }])), &py_str(")$"))]).unwrap_or_else(|e| exc::raise(e)));
    s_WEAK(call_builtin("frozenset", &[py_tuple(vec![py_str("name"), py_str("time"), py_str("day"), py_str("week"), py_str("weekend"), py_str("month"), py_str("year"), py_str("thing"), py_str("lot"), py_str("bit"), py_str("way"), py_str("kind"), py_str("sort"), py_str("part"), py_str("place"), py_str("trip"), py_str("plan"), py_str("idea"), py_str("question"), py_str("answer"), py_str("chat"), py_str("talk"), py_str("conversation"), py_str("help"), py_str("advice")])]).unwrap_or_else(|e| exc::raise(e)));
    s_NON_NOUN(op_bitor(&op_bitor(&g_WEAK(), &g_THIN()), &call_builtin("frozenset", &[py_tuple(vec![py_str("before"), py_str("after"), py_str("again"), py_str("still"), py_str("just"), py_str("only"), py_str("maybe"), py_str("perhaps"), py_str("really"), py_str("nervous"), py_str("worried"), py_str("scared"), py_str("fine"), py_str("ok"), py_str("better"), py_str("worse"), py_str("best"), py_str("worst"), py_str("one"), py_str("ones"), py_str("two"), py_str("three"), py_str("first"), py_str("last"), py_str("next"), py_str("ask"), py_str("asked"), py_str("say"), py_str("said"), py_str("want"), py_str("need"), py_str("try"), py_str("trying"), py_str("started"), py_str("start"), py_str("stop"), py_str("make"), py_str("made"), py_str("get"), py_str("getting"), py_str("think"), py_str("thought"), py_str("going"), py_str("went"), py_str("come"), py_str("came"), py_str("much"), py_str("many"), py_str("more"), py_str("less"), py_str("all"), py_str("any"), py_str("some"), py_str("else"), py_str("something"), py_str("anything"), py_str("everything"), py_str("nothing"), py_str("remember"), py_str("recall"), py_str("forget"), py_str("said"), py_str("saying"), py_str("talked"), py_str("covered"), py_str("discussed"), py_str("meant"), py_str("rid")])]).unwrap_or_else(|e| exc::raise(e))));
    s_LEAD(py_tuple(vec![py_str("by the way"), py_str("btw"), py_str("anyway"), py_str("anyways"), py_str("so"), py_str("and"), py_str("but"), py_str("ok"), py_str("okay"), py_str("well"), py_str("oh"), py_str("ah"), py_str("right"), py_str("great"), py_str("sorry"), py_str("also"), py_str("then"), py_str("now"), py_str("ha"), py_str("hmm"), py_str("just"), py_str("cool"), py_str("nice"), py_str("yes"), py_str("yeah")]));
    s_MODAL(call_builtin("frozenset", &[py_tuple(vec![py_str("would"), py_str("should"), py_str("will"), py_str("can"), py_str("could"), py_str("may"), py_str("might"), py_str("must"), py_str("shall"), py_str("ought")])]).unwrap_or_else(|e| exc::raise(e)));
    s_AUX(call_builtin("frozenset", &[py_tuple(vec![py_str("am"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("do"), py_str("does"), py_str("did"), py_str("have"), py_str("has"), py_str("had"), py_str("can"), py_str("could"), py_str("will"), py_str("would"), py_str("shall"), py_str("should"), py_str("may"), py_str("might"), py_str("must")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WH(call_builtin("frozenset", &[py_tuple(vec![py_str("what"), py_str("why"), py_str("how"), py_str("when"), py_str("where"), py_str("who"), py_str("whom"), py_str("whose"), py_str("which")])]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("it"), py_str("they"), py_str("he"), py_str("she"), py_str("this"), py_str("that"), py_str("there"), py_str("the"), py_str("a"), py_str("an"), py_str("my"), py_str("our"), py_str("your")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBJECTISH(op_bitor(&g_PRONOUN(), &call_builtin("frozenset", &[py_tuple(vec![py_str("people"), py_str("anyone"), py_str("someone"), py_str("everyone"), py_str("kids"), py_str("children")])]).unwrap_or_else(|e| exc::raise(e))));
    s_FLIP(py_dict(vec![(py_str("i"), py_str("you")), (py_str("me"), py_str("you")), (py_str("my"), py_str("your")), (py_str("mine"), py_str("yours")), (py_str("myself"), py_str("yourself")), (py_str("we"), py_str("you")), (py_str("us"), py_str("you")), (py_str("our"), py_str("your")), (py_str("ours"), py_str("yours")), (py_str("ourselves"), py_str("yourselves")), (py_str("i'm"), py_str("you're")), (py_str("i've"), py_str("you've")), (py_str("i'd"), py_str("you'd")), (py_str("i'll"), py_str("you'll")), (py_str("we're"), py_str("you're")), (py_str("we've"), py_str("you've")), (py_str("we'd"), py_str("you'd")), (py_str("we'll"), py_str("you'll")), (py_str("im"), py_str("you're")), (py_str("ive"), py_str("you've")), (py_str("am"), py_str("are"))]));
    s_PLACE_NAMES(call_builtin("frozenset", &[py_tuple(vec![py_str("place"), py_str("name")])]).unwrap_or_else(|e| exc::raise(e)));
    s_CALENDAR(call_builtin("frozenset", &[mcall(&py_str("today tomorrow yesterday tonight morning afternoon evening night week\nweekend month year hour minute second monday tuesday wednesday thursday friday saturday sunday\njanuary february march april may june july august september october november december\nspring summer autumn winter christmas easter holidays"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_QUALITY(op_bitor(&gen_understanding::understanding::kinds::g_POSITIVE(), &gen_understanding::understanding::kinds::g_NEGATIVE()));
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
    s_VERB_BEFORE(op_bitor(&g_AUX(), &call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("we"), py_str("you"), py_str("they"), py_str("he"), py_str("she"), py_str("it"), py_str("not"), py_str("please"), py_str("lets"), py_str("let's"), py_str("who"), py_str("dont"), py_str("doesnt"), py_str("didnt"), py_str("cant"), py_str("wont"), py_str("wouldnt"), py_str("couldnt"), py_str("shouldnt")])]).unwrap_or_else(|e| exc::raise(e))));
    s_DETERMINERS(call_builtin("frozenset", &[py_tuple(vec![py_str("a"), py_str("an"), py_str("the"), py_str("my"), py_str("your"), py_str("our"), py_str("their"), py_str("his"), py_str("her"), py_str("its"), py_str("this"), py_str("that"), py_str("these"), py_str("those"), py_str("some"), py_str("any"), py_str("many"), py_str("much"), py_str("no"), py_str("every"), py_str("each")])]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_VERBS(call_builtin("frozenset", &[mcall(&py_str("do give get go try keep sort call tell know feel be been have wait\n    stop start find leave come put use take see say think love like want look check help send\n    make book buy fix sort plan"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_NUMBER_WORD_HEADS(call_builtin("frozenset", &[mcall(&py_str("\nzero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen\nsixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety\nhundred thousand million\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_ASK_WORDS(py_dict(vec![(py_str("how_long"), py_tuple(vec![py_str("how"), py_str("long")])), (py_str("how_much"), py_tuple(vec![py_str("how"), py_str("much")])), (py_str("how_many"), py_tuple(vec![py_str("how"), py_str("many")])), (py_str("how_far"), py_tuple(vec![py_str("how"), py_str("far")])), (py_str("how_often"), py_tuple(vec![py_str("how"), py_str("often")]))]));
    s_ADDRESSEE(py_dict(vec![(py_str("you"), py_str("me")), (py_str("your"), py_str("my")), (py_str("yours"), py_str("mine")), (py_str("yourself"), py_str("myself")), (py_str("you're"), py_str("i am")), (py_str("youve"), py_str("i have")), (py_str("you've"), py_str("i have")), (py_str("you'd"), py_str("i would")), (py_str("you'll"), py_str("i will"))]));
    s_SUBJECT_YOU(py_str("I"));
    s_LOST_SUBJECT(call_builtin("frozenset", &[py_tuple(vec![py_str("cant"), py_str("can't"), py_str("couldnt"), py_str("couldn't"), py_str("dont"), py_str("don't"), py_str("didnt"), py_str("didn't"), py_str("wont"), py_str("won't"), py_str("havent"), py_str("haven't"), py_str("hadnt"), py_str("hadn't"), py_str("wouldnt"), py_str("wouldn't"), py_str("shouldnt"), py_str("shouldn't")])]).unwrap_or_else(|e| exc::raise(e)));
    s_DROPPED_SUBJECT(op_bitor(&g_LOST_SUBJECT(), &call_builtin("frozenset", &[py_tuple(vec![py_str("am"), py_str("is"), py_str("are"), py_str("was"), py_str("were"), py_str("will"), py_str("might")])]).unwrap_or_else(|e| exc::raise(e))));
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
    s_VERB_WORDS(op_bitor(&gen_understanding::understanding::kinds::g_VERB_FORMS(), &gen_understanding::understanding::kinds::g_VERBISH()));
    s_DETAIL_OPENER(stdlib_call("re", "compile", &[py_str("^\\s*(?:its|it's|it is|thats|that's|that is|theyre|they're|they are|hes|he's|she's|shes)\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHECK_VERDICT(stdlib_call("re", "compile", &[py_str("^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:(?:is|will|would|does|do|was)\\s+(?:that|it|this)(?:\\s+be)?|(?:that|it|this)(?:'s|s|\\s+is)?)\\s+(?:alright|all right|ok|okay|fine|safe|allowed|work|works)(?:\\s+for (?:me|us|him|her|them))?(?:\\s+(?:though|then|now))?|^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:are|r)\\s+(?:you\\s+)?(?:really\\s+)?sure(?:\\s+(?:about|of)\\s+(?:that|it|this))?$|^(?:you\\s+)?(?:really\\s+)?sure(?:\\s+(?:about|of)\\s+(?:that|it|this))?$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_VERDICT_SAID(stdlib_call("re", "compile", &[py_str("^(?:still\\s+(?:a\\s+)?)?(yes|no|afraid not|no, sorry|no, afraid not)\\s*[:\\u2014\\u2013-]\\s*(.+)$"), op_bitor(&stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)), &stdlib_attr("re", "S").unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)));
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
    let _ = mcall(&out.clone(), "append", &[py_tuple(vec![text.clone(), gen_understanding::understanding::understand(&[text.clone(), py_dict(vec![])])])]);
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
    return { let __b = call_builtin("bool", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_RECEIPT_LONGEST()); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_RECEIPT_WORDS(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
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
    return { let __b = op_le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THIN(), &w.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } };
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
    reading = gen_understanding::understanding::understand(&[text.clone(), py_dict(vec![])]);
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
    reading = gen_understanding::understanding::understand(&[text.clone(), py_dict(vec![])]);
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
    return { let __b = call_builtin("bool", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = op_le(&call_builtin("len", &[mcall(&low.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_POINTS_BACK(), "match", &[low.clone()])]).unwrap_or_else(|e| exc::raise(e)) } } } };
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
    if py_truthy(&op_gt(&call_builtin("len", &[other.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    return Py::Bool(false);
    }
    at = mcall(&words.clone(), "index", &[geti(&other.clone(), &Py::Int(0))]);
    return { let __b = op_lt(&op_add(&at.clone(), &Py::Int(1)), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { call_builtin("bool", &[mcall(&g_POINTING_WORD(), "fullmatch", &[geti(&words.clone(), &op_add(&at.clone(), &Py::Int(1)))])]).unwrap_or_else(|e| exc::raise(e)) } };
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&name.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WEAK(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_FLIP(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_THIN(), &core.clone())); if py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } } } } } } } }) {
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
    last = geti(&groups.clone(), &op_neg(&Py::Int(1)));
    joined = op_add(&op_add(&mcall(&geti(&geti(&last.clone(), &op_neg(&Py::Int(1))), &py_str("text")), "strip", &[]), &py_str(" ")), &name.clone());
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
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    token = geti(&words.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&token.clone(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "startswith", &[mcall(&word.clone(), "lower", &[])]))) } }) {
    continue;
    }
    return { let __b = op_gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VERB_BEFORE(), &geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))))) } };
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&word.clone(), "endswith", &[py_str("s")]))); if py_truthy(&__b) { __b } else { { let __b = mcall(&word.clone(), "endswith", &[py_str("ss")]); if py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)) } } } }) {
    return Py::Bool(false);
    }
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut w: Py = __item.clone(); __out.push(mcall(&mcall(&call_builtin("str", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[py_str(".,!?")]), "lower", &[])); } py_list(__out) })() };
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    token = geti(&words.clone(), &index.clone());
    if py_truthy(&{ let __b = Py::Bool(!py_eq(&token.clone(), &mcall(&word.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&token.clone(), "startswith", &[mcall(&word.clone(), "lower", &[])]))) } }) {
    continue;
    }
    if py_truthy(&op_lt(&index.clone(), &Py::Int(2))) {
    return Py::Bool(false);
    }
    let __unpk2 = py_unpack(&py_tuple(vec![geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))), geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(2)))]), 2, None);
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
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&Py::Bool(!py_eq(&geti(&words.clone(), &index.clone()), &mcall(&word.clone(), "lower", &[])))) {
    continue;
    }
    before = op_sub(&index.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = op_ge(&before.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&words.clone(), &before.clone()))) } }) {
    let __aug2 = op_sub(&before.clone(), &Py::Int(1));
    before = __aug2;
    }
    if py_truthy(&{ let __b = op_ge(&before.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREP(), &geti(&words.clone(), &before.clone()))) } }) {
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[at.clone(), py_func("int", |args: &[Py]| call_builtin("int", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = op_le(&at.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { op_gt(&at.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::Bool(false);
    }
    before = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &call_builtin("max", &[Py::Int(0), op_sub(&at.clone(), &Py::Int(2))]).unwrap_or_else(|e| exc::raise(e)), &at.clone(), &Py::None)) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "lower", &[])); } py_list(__out) })() };
    while py_truthy(&{ let __b = before.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &geti(&before.clone(), &op_neg(&Py::Int(1))))) } }) {
    let _ = mcall(&before.clone(), "pop", &[]);
    }
    return { let __b = call_builtin("bool", &[before.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PREP(), &geti(&before.clone(), &op_neg(&Py::Int(1))))) } };
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
    before = if py_truthy(&index.clone()) { geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))) } else { py_str("") };
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
    chosen = geti(&groups.clone(), &op_neg(&Py::Int(1)));
    break;
    }
    }
    if py_truthy(&chosen.clone()) {
    name = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&chosen.clone()) { let mut e: Py = __item.clone(); __out.push(mcall(&geti(&e.clone(), &py_str("text")), "strip", &[])); } py_list(__out) })() }]);
    before = _qualifier(&[reading.clone(), name.clone(), keywords.clone()]);
    if py_truthy(&before.clone()) {
    name = op_add(&op_add(&before.clone(), &py_str(" ")), &name.clone());
    }
    name = _same_spelling(&[text.clone(), name.clone()]);
    kind = mcall(&geti(&chosen.clone(), &Py::Int(0)), "get", &[py_str("type")]);
    if py_truthy(&mcall(&mcall(&name.clone(), "lower", &[]), "startswith", &[py_str("my ")])) {
    name = op_add(&py_str("your "), &slice_or_raise(&name.clone(), &Py::Int(3), &Py::None, &Py::None));
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PLACE_NAMES(), &kind.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&kind.clone(), &py_str("name"))); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&name.clone(), &Py::None, &Py::Int(1), &Py::None), "isupper", &[]) } } } }) {
    return _place_case(&[name.clone()]);
    }
    if py_truthy(&mcall(&mcall(&name.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("your "), py_str("a "), py_str("an "), py_str("the ")])])) {
    return name.clone();
    }
    return op_add(&py_str("the "), &name.clone());
    }
    nouns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keywords.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&w.clone(), "endswith", &[py_tuple(vec![py_str("ing"), py_str("ed")])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_NON_NOUN(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_QUALITY(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&g_CALENDAR(), &w.clone())); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_the_verb(&[reading.clone(), w.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_verb_after_a_subject(&[reading.clone(), w.clone()]))) } } } } } } } } } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&nouns.clone()))) {
    return Py::None;
    }
    things = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&nouns.clone()) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&gen_understanding::understanding::kinds::g_VERB_FORMS(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&things.clone()) {
    nouns = things.clone();
    }
    kept = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&nouns.clone()) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(py_contains(&gen_understanding::understanding::topics::g_PLACES(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_word_behind_prep(&[reading.clone(), w.clone()]))) } }) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = kept.clone(); if !py_truthy(&__b) { __b } else { op_lt(&call_builtin("len", &[kept.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[nouns.clone()]).unwrap_or_else(|e| exc::raise(e))) } }) {
    nouns = kept.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&gen_understanding::understanding::topics::g_PLACES(), &geti(&nouns.clone(), &op_neg(&Py::Int(1)))))) {
    return _place_case(&[_same_spelling(&[text.clone(), geti(&nouns.clone(), &op_neg(&Py::Int(1)))])]);
    }
    around = _the_phrase_around(&[reading.clone(), geti(&nouns.clone(), &op_neg(&Py::Int(1)))]);
    if py_truthy(&{ let __b = named_only.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&around.clone())) } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = around.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&around.clone(), &Py::Int(0)), &py_str("your"))) } }) {
    return op_add(&py_str("your "), &geti(&nouns.clone(), &op_neg(&Py::Int(1))));
    }
    return op_add(&py_str("the "), &geti(&nouns.clone(), &op_neg(&Py::Int(1))));
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
    if py_truthy(&{ let __b = Py::Bool(py_contains(&gen_understanding::understanding::topics::g_PLACES(), &mcall(&name.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push(Py::Bool(py_contains(&gen_understanding::understanding::topics::g_PLACES(), &mcall(&w.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
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
    if py_truthy(&op_ge(&index.clone(), &Py::Int(0))) {
    return slice_or_raise(&text.clone(), &index.clone(), &op_add(&index.clone(), &call_builtin("len", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
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
    if py_truthy(&op_le(&index.clone(), &Py::Int(0))) {
    return Py::None;
    }
    words = mcall(&slice_or_raise(&text.clone(), &Py::None, &index.clone(), &Py::None), "split", &[]);
    if py_truthy(&op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    before = mcall(&geti(&words.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]);
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&keywords.clone(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_DETERMINERS(), &mcall(&geti(&words.clone(), &op_neg(&Py::Int(2))), "strip", &[py_str(".,!?")]))) } }) {
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
    lines = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&user_lines(&[memory.clone()])) { let __unpk2654 = py_unpack(&__item.clone(), 2, None); let mut text: Py = __unpk2654[0].clone(); let mut reading: Py = __unpk2654[1].clone(); if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_is_thin(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_memory_line(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_asks_about_the_chat(&[text.clone(), memory.clone(), skip.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_asks_about_the_bot(&[reading.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&signs_off(&[text.clone()]))) } } } } } } } }) {  __out.push(py_tuple(vec![text.clone(), reading.clone()])); } } py_list(__out) })() };
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
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_DETERMINERS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]))) } }) {
    words = slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None);
    }
    head = if py_truthy(&words.clone()) { mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[]) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&head.clone()))) {
    return Py::Bool(false);
    }
    let __try1 = exc::catch(|| {
    let mut _topics: Py = crate::memory::topics::module_obj();
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
    let mut topics: Py = crate::memory::topics::module_obj();
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
    return op_add(&py_str("the "), &phrase.clone());
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
    return { let __b = mcall(&core.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WH(), &slice_or_raise(&core.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None))) } };
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
    let __iter3 = py_iter_or_raise(&slice_or_raise(&norm.clone(), &index.clone(), &op_add(&index.clone(), &Py::Int(3)), &Py::None));
    for __item in __iter3 {
    word2 = __item.clone();
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_AUX(), &word2.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SUBJECTISH(), &word2.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&py_tuple(vec![py_str("the"), py_str("a"), py_str("an")]), &word2.clone())) } } } } } }) {
    break;
    }
    let _ = mcall(&phrase.clone(), "append", &[word2.clone()]);
    }
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &mcall(&call_builtin("str", &[geti(&phrase.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return op_add(&phrase.clone(), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    }
    break;
    }
    let __iter4 = py_iter_or_raise(&{ let __b = mcall(&reading.clone(), "get", &[py_str("asks")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } });
    for __item in __iter4 {
    ask = __item.clone();
    phrase = { let __b = mcall(&g_ASK_WORDS(), "get", &[ask.clone()]); if py_truthy(&__b) { __b } else { if py_truthy(&Py::Bool(py_contains(&g_WH(), &ask.clone()))) { py_tuple(vec![ask.clone()]) } else { py_tuple(vec![]) } } };
    if py_truthy(&{ let __b = phrase.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&held.clone(), &mcall(&call_builtin("str", &[geti(&phrase.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]))) } }) {
    return op_add(&call_builtin("list", &[phrase.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
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
    return op_add(&call_builtin("list", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("list", &[slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    }
    return op_add(&py_list(vec![py_str("you")]), &call_builtin("list", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
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
    return op_add(&mcall(&g_FIRST_PERSON_AUX(), "get", &[mcall(&core.clone(), "lower", &[]), core.clone()]), &tail.clone());
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
    before = if py_truthy(&index.clone()) { mcall(&mcall(&geti(&words.clone(), &op_sub(&index.clone(), &Py::Int(1))), "strip", &[py_str(".,!?;:")]), "lower", &[]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&core.clone(), &py_str("you"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&index.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_AUX(), &before.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WH(), &before.clone())) } } } } } }) {
    if py_truthy(&out.clone()) {
    let _ = py_setitem(&out.clone(), &op_neg(&Py::Int(1)), _first_person_aux(&[geti(&out.clone(), &op_neg(&Py::Int(1)))]));
    }
    let _ = mcall(&out.clone(), "append", &[op_add(&g_SUBJECT_YOU(), &tail.clone())]);
    } else {
    let _ = mcall(&out.clone(), "append", &[op_add(&geti(&g_ADDRESSEE(), &core.clone()), &tail.clone())]);
    }
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[op_add(&mcall(&g_FLIP(), "get", &[core.clone(), core.clone()]), &tail.clone())]);
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
    if py_truthy(&op_ge(&index.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
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
    if py_truthy(&op_ge(&start.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    return start.clone();
    }
    if py_truthy(&Py::Bool(!py_contains(&g_DETERMINERS(), &mcall(&mcall(&geti(&words.clone(), &start.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[])))) {
    return op_add(&start.clone(), &Py::Int(1));
    }
    end = op_add(&start.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = op_lt(&end.clone(), &call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { op_lt(&op_sub(&end.clone(), &start.clone()), &most.clone()) } }) {
    core = mcall(&mcall(&geti(&words.clone(), &end.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_AUX(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WH(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PREDICATE(), &core.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_PREP(), &core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ADVERB(), &core.clone())) } } } } } } } }) {
    break;
    }
    let __aug1 = op_add(&end.clone(), &Py::Int(1));
    end = __aug1;
    }
    return call_builtin("max", &[end.clone(), op_add(&start.clone(), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e));
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
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    aux = mcall(&mcall(&geti(&words.clone(), &index.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_AUX(), &aux.clone()))) {
    continue;
    }
    start = op_add(&index.clone(), &Py::Int(1));
    if py_truthy(&Py::Bool(!py_truthy(&_subjects_at(&[words.clone(), start.clone(), names.clone()])))) {
    continue;
    }
    end = _np_end(&[words.clone(), start.clone()]);
    let __unpk2 = py_unpack(&py_tuple(vec![slice_or_raise(&words.clone(), &start.clone(), &end.clone(), &Py::None), slice_or_raise(&words.clone(), &end.clone(), &Py::None, &Py::None)]), 2, None);
    subject = __unpk2[0].clone();
    tail = __unpk2[1].clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_MODAL(), &aux.clone())); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&index.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&tail.clone())) } } } } } }) {
    let __unpk3 = py_unpack(&py_tuple(vec![slice_or_raise(&subject.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), slice_or_raise(&subject.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None)]), 2, None);
    subject = __unpk3[0].clone();
    tail = __unpk3[1].clone();
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("do"), py_str("does"), py_str("did")]), &aux.clone()))) {
    verb = Py::Int(0);
    while py_truthy(&{ let __b = op_lt(&verb.clone(), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_ADVERB(), &mcall(&mcall(&geti(&tail.clone(), &verb.clone()), "strip", &[py_str(".,!?;:")]), "lower", &[]))) } }) {
    let __aug4 = op_add(&verb.clone(), &Py::Int(1));
    verb = __aug4;
    }
    if py_truthy(&op_lt(&verb.clone(), &call_builtin("len", &[tail.clone()]).unwrap_or_else(|e| exc::raise(e)))) {
    let _ = py_setitem(&tail.clone(), &verb.clone(), _agree(&[geti(&tail.clone(), &verb.clone()), aux.clone()]));
    } else {
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[subject.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&op_bitor(&op_bitor(&g_DETERMINERS(), &g_PREDICATE()), &g_MEASURE()), &mcall(&mcall(&geti(&subject.clone(), &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?;:")]), "lower", &[]))) } }) {
    let __unpk5 = py_unpack(&py_tuple(vec![slice_or_raise(&subject.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), py_list(vec![_agree(&[geti(&subject.clone(), &op_neg(&Py::Int(1))), aux.clone()])])]), 2, None);
    subject = __unpk5[0].clone();
    tail = __unpk5[1].clone();
    }
    }
    return op_add(&op_add(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None), &subject.clone()), &tail.clone());
    }
    return op_add(&op_add(&op_add(&slice_or_raise(&words.clone(), &Py::None, &index.clone(), &Py::None), &subject.clone()), &py_list(vec![geti(&words.clone(), &index.clone())])), &tail.clone());
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
    if py_truthy(&Py::Bool(py_eq(&index.clone(), &op_sub(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))))) {
    let _ = mcall(&out.clone(), "append", &[word.clone()]);
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&stems.clone(), &mcall(&core.clone(), "lower", &[]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&core.clone(), &py_str("'"))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_contains(&core.clone(), &py_str("’"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_IS_NOT_POSSESSIVE(), &mcall(&core.clone(), "lower", &[]))) } } } } } }) {
    let _ = mcall(&out.clone(), "append", &[op_add(&op_add(&core.clone(), &py_str("'s")), &slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))]);
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
    return { let __b = op_lt(&call_builtin("len", &[focus_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[clean_words.clone()]).unwrap_or_else(|e| exc::raise(e))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&slice_or_raise(&clean_words.clone(), &Py::None, &call_builtin("len", &[focus_words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None), &focus_words.clone())) } };
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
    keep = {{ let clause = clause.clone(); let keep = keep.clone(); let match_ = match_.clone(); let text = text.clone(); let typed = typed.clone(); py_func("keep", move |args: &[Py]| { let mut full = vec![clause.clone(), keep.clone(), match_.clone(), text.clone(), typed.clone()]; full.extend_from_slice(args); __nested_keep_0(&full) }) }};
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
    reading = gen_understanding::understanding::understand(&[text.clone(), py_dict(vec![])]);
    clean = mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("clean")]); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    clause = _as_text(&[mcall(&reading.clone(), "get", &[py_str("focus")])]);
    if py_truthy(&Py::Bool(!py_truthy(&clause.clone()))) {
    clause = mcall(&geti(&mcall(&{ let __b = mcall(&reading.clone(), "get", &[py_str("norm_text")]); if py_truthy(&__b) { __b } else { py_str("") } }, "split", &[py_str(" | ")]), &op_neg(&Py::Int(1))), "strip", &[]);
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
    let _ = py_setitem(&words.clone(), &index.clone(), op_add(&typed_aux.clone(), &slice_or_raise(&word.clone(), &call_builtin("len", &[mcall(&word.clone(), "strip", &[py_str(".,!?;:")])]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)));
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
    clause = if py_truthy(&Py::Bool(py_contains(&g_AUX(), &first.clone()))) { op_add(&py_str("whether "), &said.clone()) } else { said.clone() };
    }
    } else {
    clause = crate::memory::_from_the_bot(&[mcall(&py_str(" "), "join", &[_flip(&[mcall(&crate::memory::_to_the_bot(&[mcall(&py_str(" "), "join", &[words.clone()])]), "split", &[])])])]);
    }
    clause = _conditional(&[clause.clone()]);
    clause = _keep_possessives(&[clause.clone(), clean.clone()]);
    if py_truthy(&Py::Bool(py_eq(&mcall(&reading.clone(), "get", &[py_str("kind")]), &py_str("question")))) {
    core = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&clause.clone(), "split", &[])) { let mut word: Py = __item.clone(); __out.push(mcall(&mcall(&word.clone(), "strip", &[py_str(".,!?;:")]), "lower", &[])); } py_set(__out) })() };
    opening = geti(&{ let __b = mcall(&clause.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&op_bitand(&core.clone(), &op_bitor(&g_AUX(), &g_WH())))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_is_wh(&[opening.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&core.clone(), &py_str("whether"))) } } } }) {
    return Py::None;
    }
    }
    clause = _numbers(&[text.clone(), clause.clone()]);
    clause = crate::memory::facts::no_tone(&[clause.clone()]);
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
    return op_add(&geti(&g_PAST(), &core.clone()), &suffix.clone());
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&core.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None), "isalpha", &[]))) } }) {
    return word.clone();
    }
    if py_truthy(&mcall(&core.clone(), "endswith", &[py_str("e")])) {
    return op_add(&op_add(&core.clone(), &py_str("d")), &suffix.clone());
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&core.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&core.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&op_add(&slice_or_raise(&core.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ied")), &suffix.clone());
    }
    return op_add(&op_add(&core.clone(), &py_str("ed")), &suffix.clone());
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&core.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&slice_or_raise(&core.clone(), &op_neg(&Py::Int(1)), &Py::None, &Py::None), "isalpha", &[]))) } }) {
    return verb.clone();
    }
    if py_truthy(&Py::Bool(py_contains(&g_THIRD(), &low.clone()))) {
    return op_add(&geti(&g_THIRD(), &low.clone()), &suffix.clone());
    }
    if py_truthy(&Py::Bool(py_contains(&g_AUX(), &low.clone()))) {
    return verb.clone();
    }
    if py_truthy(&mcall(&low.clone(), "endswith", &[py_tuple(vec![py_str("s"), py_str("sh"), py_str("ch"), py_str("x"), py_str("z"), py_str("o")])])) {
    return op_add(&op_add(&core.clone(), &py_str("es")), &suffix.clone());
    }
    if py_truthy(&{ let __b = op_gt(&call_builtin("len", &[low.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&low.clone(), "endswith", &[py_str("y")]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_str("aeiou"), &geti(&low.clone(), &op_neg(&Py::Int(2))))) } } } }) {
    return op_add(&op_add(&slice_or_raise(&core.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("ies")), &suffix.clone());
    }
    return op_add(&op_add(&core.clone(), &py_str("s")), &suffix.clone());
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
    return op_add(&py_str("what happens "), &clause.clone());
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
    let _ = mcall(&out.clone(), "append", &[if py_truthy(&Py::Bool(py_contains(&capitals.clone(), &core.clone()))) { op_add(&_capital(&[core.clone()]), &slice_or_raise(&word.clone(), &call_builtin("len", &[core.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)) } else { word.clone() }]);
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
    return if py_truthy(&text.clone()) { op_add(&mcall(&slice_or_raise(&text.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&text.clone(), &Py::Int(1), &Py::None, &Py::None)) } else { text.clone() };
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
    recent = slice_or_raise(&turns.clone(), &op_neg(&g_ON_THE_TABLE_TURNS()), &Py::None, &Py::None);
    out = py_list(vec![]);
    misses = Py::Int(0);
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[recent.clone()]).unwrap_or_else(|e| exc::raise(e)), Py::Int(0), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    size = __item.clone();
    shorter = call_builtin("dict", &[memory.clone()]).unwrap_or_else(|e| exc::raise(e));
    let _ = py_setitem(&shorter.clone(), &py_str("turns"), slice_or_raise(&recent.clone(), &Py::None, &size.clone(), &Py::None));
    phrase = about(&[shorter.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&phrase.clone()))) {
    continue;
    }
    head = mcall(&geti(&{ let __b = mcall(&phrase.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &op_neg(&Py::Int(1))), "strip", &[py_str(".,!?")]);
    if py_truthy(&Py::Bool(py_contains(&g_NOT_A_THING(), &head.clone()))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_said_in_the_chat(&[phrase.clone(), slice_or_raise(&recent.clone(), &Py::None, &size.clone(), &Py::None)])))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut one: Py = __item.clone(); __out.push(Py::Bool(py_eq(&mcall(&phrase.clone(), "lower", &[]), &mcall(&one.clone(), "lower", &[])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    let __aug2 = op_add(&misses.clone(), &Py::Int(1));
    misses = __aug2;
    if py_truthy(&op_ge(&misses.clone(), &g_ON_THE_TABLE_MISSES())) {
    break;
    }
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[phrase.clone()]);
    misses = Py::Int(0);
    if py_truthy(&op_ge(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
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
    words = op_add(&py_list(vec![py_str("my")]), &slice_or_raise(&words.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    head = geti(&words.clone(), &op_neg(&Py::Int(1)));
    let __iter1 = py_iter_or_raise(&turns.clone());
    for __item in __iter1 {
    turn = __item.clone();
    text = if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&turn.clone(), "get", &[py_str("user")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[text.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = mcall(&mcall(&text.clone(), "lower", &[]), "split", &[]);
    run = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[op_add(&op_sub(&call_builtin("len", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), &run.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    start = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&slice_or_raise(&said.clone(), &start.clone(), &op_add(&start.clone(), &run.clone()), &Py::None), &words.clone()))) {
    return Py::Bool(true);
    }
    }
    if py_truthy(&Py::Bool(py_contains(&g_VERB_WORDS(), &head.clone()))) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&gen_understanding::understanding::understand(&[text.clone(), py_dict(vec![])]), "get", &[py_str("entities")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
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
    j = op_sub(&i.clone(), &Py::Int(1));
    while py_truthy(&{ let __b = op_ge(&j.clone(), &Py::Int(0)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STATE_FILLER(), &mcall(&geti(&said.clone(), &j.clone()), "strip", &[py_str(".,!?;:\"'")]))) } }) {
    let __aug3 = op_sub(&j.clone(), &Py::Int(1));
    j = __aug3;
    }
    if py_truthy(&{ let __b = op_lt(&j.clone(), &Py::Int(0)); if py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_COPULA(), &mcall(&geti(&said.clone(), &j.clone()), "strip", &[py_str(".,!?;:\"'")]))) } }) {
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
    return op_add(&op_add(&mcall(&py_str(", "), "join", &[slice_or_raise(&items.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)]), &py_str(" and ")), &geti(&items.clone(), &op_neg(&Py::Int(1))));
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
    let __iter1 = py_iter_or_raise(&call_builtin("range", &[op_sub(&call_builtin("len", &[lines.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)), op_neg(&Py::Int(1)), op_neg(&Py::Int(1))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    index = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(!py_is(&not_before.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { op_lt(&index.clone(), &not_before.clone()) } }) {
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
    got = crate::memory::detail::the_detail(&[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&got.clone()))) {
    return Py::Bool(false);
    }
    words = { let got = got.clone(); let line = line.clone(); let rest = rest.clone(); let text = text.clone(); let words = words.clone(); py_func("lambda", move |args: &[Py]| { let got = got.clone(); let line = line.clone(); let rest = rest.clone(); let text = text.clone(); let words = words.clone(); let t: Py = __kwp(args, 0, "t").unwrap_or(Py::None); { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&call_builtin("str", &[{ let __b = t.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])) { let mut w: Py = __item.clone(); __out.push(mcall(&w.clone(), "lower", &[])); } py_list(__out) })() } }) };
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
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_FLIP(), &mcall(&name.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { op_le(&call_builtin("len", &[mcall(&name.clone(), "strip", &[py_str(".,")])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)) } }) {
    continue;
    }
    if py_truthy(&mcall(&mcall(&name.clone(), "lower", &[]), "startswith", &[py_str("my ")])) {
    name = op_add(&py_str("your "), &slice_or_raise(&name.clone(), &Py::Int(3), &Py::None, &Py::None));
    }
    if py_truthy(&Py::Bool(py_contains(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut n: Py = __item.clone(); __out.push(mcall(&n.clone(), "lower", &[])); } py_list(__out) })() }, &mcall(&name.clone(), "lower", &[])))) {
    continue;
    }
    let _ = mcall(&out.clone(), "append", &[name.clone()]);
    if py_truthy(&op_ge(&call_builtin("len", &[out.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(9)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CHECK_VERDICT(), "match", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    last = py_str("");
    if py_truthy(&{ let __b = call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { turns.clone() } }) {
    last = mcall(&call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
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
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = op_gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CHECK_A_FACT(), "match", &[said.clone()]))) } } } }) {
    return Py::None;
    }
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return Py::None;
    }
    last = if py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &op_neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { geti(&turns.clone(), &op_neg(&Py::Int(1))) } else { py_dict(vec![]) };
    answer = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&answer.clone())); if py_truthy(&__b) { __b } else { mcall(&answer.clone(), "endswith", &[py_str("?")]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_fact_from(&[answer.clone()])))) {
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[mcall(&answer.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(8)); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_NO_FACT_SAID(), "match", &[answer.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NOT_A_FACT(), "search", &[answer.clone()]) } } } }) {
    return Py::None;
    }
    let mut topics: Py = crate::memory::topics::module_obj();
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
    said = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = turns.clone(); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut turn: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[turn.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&turn.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])); } } py_list(__out) })() }, &op_neg(&Py::Int(6)), &Py::None, &Py::None);
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
    if py_truthy(&op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &limit.clone())) {
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
    return { let __b = mcall(&word.clone(), "endswith", &[py_str("ed")]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_DONE_WORDS(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&gen_understanding::understanding::normalize::g_IRREGULAR_PARTICIPLES(), &word.clone())) } } } };
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
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__A_STATE_ITSELF(), &geti(&words.clone(), &op_neg(&Py::Int(1))))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_NAME_FOR_IT(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)))) } }) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_DESCRIPTION(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_THING_PARTICLE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { _a_done_word(&[geti(&words.clone(), &op_neg(&Py::Int(1)))]) } }) {
    words = slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None);
    thing = mcall(&mcall(&py_str(" "), "join", &[words.clone()]), "strip", &[py_str(" ,;:-")]);
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_THE_THING(), &thing.clone())) } }) {
    continue;
    }
    } else {
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::None, &op_neg(&Py::Int(1)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(_a_done_word(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
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
    asked_before = _asked_already(&[{ let __b = mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { mcall(&geti(&turns.clone(), &op_neg(&Py::Int(1))), "get", &[py_str("reply")]) } }]);
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
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&frames.clone()) { let mut frame: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("v".to_string(), verdict.clone()), ("V".to_string(), mcall(&verdict.clone(), "capitalize", &[]))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone()]);
    }
    if py_truthy(&asked_before.clone()) {
    return pyrt::xrefs::xcall("compose", "pick", &[call_builtin("list", &[g_WENT_AGAIN()]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    }
    return pyrt::xrefs::xcall("compose", "pick", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WENT_FRAMES()) { let mut frame: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&frame.clone(), "format", args))(&__args) }); } py_list(__out) })() }, memory.clone()]);
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
            "IRREGULAR_PARTICIPLES" => Some(g_IRREGULAR_PARTICIPLES()),
            "IS_NOT_POSSESSIVE" => Some(g_IS_NOT_POSSESSIVE()),
            "IT_WAS" => Some(g_IT_WAS()),
            "LEAD" => Some(g_LEAD()),
            "LOST_SUBJECT" => Some(g_LOST_SUBJECT()),
            "MEASURE" => Some(g_MEASURE()),
            "MODAL" => Some(g_MODAL()),
            "NEGATIVE" => Some(g_NEGATIVE()),
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
            "PLACE_WORDS" => Some(g_PLACE_WORDS()),
            "POINTING_WORD" => Some(g_POINTING_WORD()),
            "POINTS_BACK" => Some(g_POINTS_BACK()),
            "POSITIVE" => Some(g_POSITIVE()),
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
            "VERBISH" => Some(g_VERBISH()),
            "VERB_BEFORE" => Some(g_VERB_BEFORE()),
            "VERB_FORMS" => Some(g_VERB_FORMS()),
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
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADDRESSEE" => s_ADDRESSEE(v),
            "ADVERB" => s_ADVERB(v),
            "ASK_VERB" => s_ASK_VERB(v),
            "ASK_WORD" => s_ASK_WORD(v),
            "ASK_WORDS" => s_ASK_WORDS(v),
            "AUX" => s_AUX(v),
            "A_CHOICE" => s_A_CHOICE(v),
            "A_VERDICT" => s_A_VERDICT(v),
            "BARE_VERBS" => s_BARE_VERBS(v),
            "CALENDAR" => s_CALENDAR(v),
            "CHECK_A_FACT" => s_CHECK_A_FACT(v),
            "CHECK_VERDICT" => s_CHECK_VERDICT(v),
            "CLOSINGS" => s_CLOSINGS(v),
            "CLOSING_FILLER" => s_CLOSING_FILLER(v),
            "CONTRACTIONS" => s_CONTRACTIONS(v),
            "COPULA" => s_COPULA(v),
            "DETAIL_OPENER" => s_DETAIL_OPENER(v),
            "DETERMINERS" => s_DETERMINERS(v),
            "DONE_WORDS" => s_DONE_WORDS(v),
            "DROPPED_SUBJECT" => s_DROPPED_SUBJECT(v),
            "EXPAND" => s_EXPAND(v),
            "FACT_FRAMES" => s_FACT_FRAMES(v),
            "FIRST_PERSON_AUX" => s_FIRST_PERSON_AUX(v),
            "FLIP" => s_FLIP(v),
            "HAD_IT" => s_HAD_IT(v),
            "HAPPENED" => s_HAPPENED(v),
            "IRREGULAR_PARTICIPLES" => s_IRREGULAR_PARTICIPLES(v),
            "IS_NOT_POSSESSIVE" => s_IS_NOT_POSSESSIVE(v),
            "IT_WAS" => s_IT_WAS(v),
            "LEAD" => s_LEAD(v),
            "LOST_SUBJECT" => s_LOST_SUBJECT(v),
            "MEASURE" => s_MEASURE(v),
            "MODAL" => s_MODAL(v),
            "NEGATIVE" => s_NEGATIVE(v),
            "NEVER_A_THING_THAT_WENT" => s_NEVER_A_THING_THAT_WENT(v),
            "NON_NOUN" => s_NON_NOUN(v),
            "NOT_A_FACT" => s_NOT_A_FACT(v),
            "NOT_A_THING" => s_NOT_A_THING(v),
            "NOT_THE_THING" => s_NOT_THE_THING(v),
            "NO_FACT_SAID" => s_NO_FACT_SAID(v),
            "NO_FRAMES" => s_NO_FRAMES(v),
            "NO_USED" => s_NO_USED(v),
            "NUMBER" => s_NUMBER(v),
            "NUMBER_WORD_HEADS" => s_NUMBER_WORD_HEADS(v),
            "ON_THE_TABLE_MISSES" => s_ON_THE_TABLE_MISSES(v),
            "ON_THE_TABLE_TURNS" => s_ON_THE_TABLE_TURNS(v),
            "PAST" => s_PAST(v),
            "PLACE_NAMES" => s_PLACE_NAMES(v),
            "PLACE_WORDS" => s_PLACE_WORDS(v),
            "POINTING_WORD" => s_POINTING_WORD(v),
            "POINTS_BACK" => s_POINTS_BACK(v),
            "POSITIVE" => s_POSITIVE(v),
            "POSSESSIVE" => s_POSSESSIVE(v),
            "PREDICATE" => s_PREDICATE(v),
            "PREP" => s_PREP(v),
            "PRONOUN" => s_PRONOUN(v),
            "QUALITY" => s_QUALITY(v),
            "RECEIPT_LONGEST" => s_RECEIPT_LONGEST(v),
            "RECEIPT_WORDS" => s_RECEIPT_WORDS(v),
            "SECOND_PERSON" => s_SECOND_PERSON(v),
            "SIGNS_OFF" => s_SIGNS_OFF(v),
            "STATE_FILLER" => s_STATE_FILLER(v),
            "SUBJECTISH" => s_SUBJECTISH(v),
            "SUBJECT_YOU" => s_SUBJECT_YOU(v),
            "THIN" => s_THIN(v),
            "THING_END" => s_THING_END(v),
            "THING_PARTICLE" => s_THING_PARTICLE(v),
            "THIRD" => s_THIRD(v),
            "VERBISH" => s_VERBISH(v),
            "VERB_BEFORE" => s_VERB_BEFORE(v),
            "VERB_FORMS" => s_VERB_FORMS(v),
            "VERB_WORDS" => s_VERB_WORDS(v),
            "VERDICT_SAID" => s_VERDICT_SAID(v),
            "WEAK" => s_WEAK(v),
            "WENT_AGAIN" => s_WENT_AGAIN(v),
            "WENT_ASK" => s_WENT_ASK(v),
            "WENT_FRAMES" => s_WENT_FRAMES(v),
            "WENT_FRAMES_NO_THING" => s_WENT_FRAMES_NO_THING(v),
            "WENT_TOLD_BAD" => s_WENT_TOLD_BAD(v),
            "WENT_TOLD_WELL" => s_WENT_TOLD_WELL(v),
            "WH" => s_WH(v),
            "WHAT_I_SAID" => s_WHAT_I_SAID(v),
            "YES_FRAMES" => s_YES_FRAMES(v),
            "YES_USED" => s_YES_USED(v),
            "_A_DESCRIPTION" => s__A_DESCRIPTION(v),
            "_A_NAME_FOR_IT" => s__A_NAME_FOR_IT(v),
            "_A_STATE_ITSELF" => s__A_STATE_ITSELF(v),
            "_A_THING_DONE" => s__A_THING_DONE(v),
            "_ONLY_THE_ARRIVAL" => s__ONLY_THE_ARRIVAL(v),
            "_WENT_WELL" => s__WENT_WELL(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("memory.thread", module_obj);
    pyrt::xrefs::reg_fn("memory.thread", "_asks_about_the_bot", py_func("_asks_about_the_bot", |args: &[Py]| _asks_about_the_bot(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_asks_about_the_chat", py_func("_asks_about_the_chat", |args: &[Py]| _asks_about_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_turns", py_func("_turns", |args: &[Py]| _turns(args)));
    pyrt::xrefs::reg_fn("memory.thread", "earlier_turn", py_func("earlier_turn", |args: &[Py]| earlier_turn(args)));
    pyrt::xrefs::reg_fn("memory.thread", "only_the_arrival", py_func("only_the_arrival", |args: &[Py]| only_the_arrival(args)));
    pyrt::xrefs::reg_fn("memory.thread", "user_lines", py_func("user_lines", |args: &[Py]| user_lines(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_is_a_receipt", py_func("_is_a_receipt", |args: &[Py]| _is_a_receipt(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_is_thin", py_func("_is_thin", |args: &[Py]| _is_thin(args)));
    pyrt::xrefs::reg_fn("memory.thread", "signs_off", py_func("signs_off", |args: &[Py]| signs_off(args)));
    pyrt::xrefs::reg_fn("memory.thread", "carries_subject", py_func("carries_subject", |args: &[Py]| carries_subject(args)));
    pyrt::xrefs::reg_fn("memory.thread", "thin", py_func("thin", |args: &[Py]| thin(args)));
    pyrt::xrefs::reg_fn("memory.thread", "points_back", py_func("points_back", |args: &[Py]| points_back(args)));
    pyrt::xrefs::reg_fn("memory.thread", "points_at_a_word_alone", py_func("points_at_a_word_alone", |args: &[Py]| points_at_a_word_alone(args)));
    pyrt::xrefs::reg_fn("memory.thread", "asks_what_i_said", py_func("asks_what_i_said", |args: &[Py]| asks_what_i_said(args)));
    pyrt::xrefs::reg_fn("memory.thread", "nothing_said", py_func("nothing_said", |args: &[Py]| nothing_said(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_is_memory_line", py_func("_is_memory_line", |args: &[Py]| _is_memory_line(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_named_entities", py_func("_named_entities", |args: &[Py]| _named_entities(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_groups", py_func("_groups", |args: &[Py]| _groups(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_is_the_verb", py_func("_is_the_verb", |args: &[Py]| _is_the_verb(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_verb_after_a_subject", py_func("_verb_after_a_subject", |args: &[Py]| _verb_after_a_subject(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_word_behind_prep", py_func("_word_behind_prep", |args: &[Py]| _word_behind_prep(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_behind_a_prep", py_func("_behind_a_prep", |args: &[Py]| _behind_a_prep(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_the_phrase_around", py_func("_the_phrase_around", |args: &[Py]| _the_phrase_around(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_phrase_for", py_func("_phrase_for", |args: &[Py]| _phrase_for(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_place_case", py_func("_place_case", |args: &[Py]| _place_case(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_same_spelling", py_func("_same_spelling", |args: &[Py]| _same_spelling(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_qualifier", py_func("_qualifier", |args: &[Py]| _qualifier(args)));
    pyrt::xrefs::reg_fn("memory.thread", "about", py_func("about", |args: &[Py]| about(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_is_subject_phrase", py_func("_is_subject_phrase", |args: &[Py]| _is_subject_phrase(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_from_topics", py_func("_from_topics", |args: &[Py]| _from_topics(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_is_wh", py_func("_is_wh", |args: &[Py]| _is_wh(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_with_wh", py_func("_with_wh", |args: &[Py]| _with_wh(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_strip_lead", py_func("_strip_lead", |args: &[Py]| _strip_lead(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_with_you", py_func("_with_you", |args: &[Py]| _with_you(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_first_person_aux", py_func("_first_person_aux", |args: &[Py]| _first_person_aux(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_flip", py_func("_flip", |args: &[Py]| _flip(args)));
    pyrt::xrefs::reg_fn("memory.thread", "your_words", py_func("your_words", |args: &[Py]| your_words(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_expand", py_func("_expand", |args: &[Py]| _expand(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_as_text", py_func("_as_text", |args: &[Py]| _as_text(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_subjects_at", py_func("_subjects_at", |args: &[Py]| _subjects_at(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_np_end", py_func("_np_end", |args: &[Py]| _np_end(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_embed", py_func("_embed", |args: &[Py]| _embed(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_keep_possessives", py_func("_keep_possessives", |args: &[Py]| _keep_possessives(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_truncated", py_func("_truncated", |args: &[Py]| _truncated(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_numbers", py_func("_numbers", |args: &[Py]| _numbers(args)));
    pyrt::xrefs::reg_fn("memory.thread", "restate", py_func("restate", |args: &[Py]| restate(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_past", py_func("_past", |args: &[Py]| _past(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_agree", py_func("_agree", |args: &[Py]| _agree(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_third", py_func("_third", |args: &[Py]| _third(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_conditional", py_func("_conditional", |args: &[Py]| _conditional(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_restore_case", py_func("_restore_case", |args: &[Py]| _restore_case(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_capital", py_func("_capital", |args: &[Py]| _capital(args)));
    pyrt::xrefs::reg_fn("memory.thread", "on_the_table", py_func("on_the_table", |args: &[Py]| on_the_table(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_said_in_the_chat", py_func("_said_in_the_chat", |args: &[Py]| _said_in_the_chat(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_said_as_a_state", py_func("_said_as_a_state", |args: &[Py]| _said_as_a_state(args)));
    pyrt::xrefs::reg_fn("memory.thread", "in_a_list", py_func("in_a_list", |args: &[Py]| in_a_list(args)));
    pyrt::xrefs::reg_fn("memory.thread", "topic_index", py_func("topic_index", |args: &[Py]| topic_index(args)));
    pyrt::xrefs::reg_fn("memory.thread", "last_ask", py_func("last_ask", |args: &[Py]| last_ask(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_only_a_detail", py_func("_only_a_detail", |args: &[Py]| _only_a_detail(args)));
    pyrt::xrefs::reg_fn("memory.thread", "last_said", py_func("last_said", |args: &[Py]| last_said(args)));
    pyrt::xrefs::reg_fn("memory.thread", "last_said_past_closings", py_func("last_said_past_closings", |args: &[Py]| last_said_past_closings(args)));
    pyrt::xrefs::reg_fn("memory.thread", "named", py_func("named", |args: &[Py]| named(args)));
    pyrt::xrefs::reg_fn("memory.thread", "checking_the_verdict", py_func("checking_the_verdict", |args: &[Py]| checking_the_verdict(args)));
    pyrt::xrefs::reg_fn("memory.thread", "checking_a_fact", py_func("checking_a_fact", |args: &[Py]| checking_a_fact(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_fact_from", py_func("_fact_from", |args: &[Py]| _fact_from(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_a_frame_not_said", py_func("_a_frame_not_said", |args: &[Py]| _a_frame_not_said(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_next_frame", py_func("_next_frame", |args: &[Py]| _next_frame(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_first_clause", py_func("_first_clause", |args: &[Py]| _first_clause(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_a_done_word", py_func("_a_done_word", |args: &[Py]| _a_done_word(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_the_verdict_they_gave", py_func("_the_verdict_they_gave", |args: &[Py]| _the_verdict_they_gave(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_the_thing_they_had", py_func("_the_thing_they_had", |args: &[Py]| _the_thing_they_had(args)));
    pyrt::xrefs::reg_fn("memory.thread", "went", py_func("went", |args: &[Py]| went(args)));
    pyrt::xrefs::reg_fn("memory.thread", "_asked_already", py_func("_asked_already", |args: &[Py]| _asked_already(args)));
    pyrt::xrefs::reg_access("memory.thread", "ADDRESSEE", g_ADDRESSEE, s_ADDRESSEE);
    pyrt::xrefs::reg_access("memory.thread", "ADVERB", g_ADVERB, s_ADVERB);
    pyrt::xrefs::reg_access("memory.thread", "ASK_VERB", g_ASK_VERB, s_ASK_VERB);
    pyrt::xrefs::reg_access("memory.thread", "ASK_WORD", g_ASK_WORD, s_ASK_WORD);
    pyrt::xrefs::reg_access("memory.thread", "ASK_WORDS", g_ASK_WORDS, s_ASK_WORDS);
    pyrt::xrefs::reg_access("memory.thread", "AUX", g_AUX, s_AUX);
    pyrt::xrefs::reg_access("memory.thread", "A_CHOICE", g_A_CHOICE, s_A_CHOICE);
    pyrt::xrefs::reg_access("memory.thread", "A_VERDICT", g_A_VERDICT, s_A_VERDICT);
    pyrt::xrefs::reg_access("memory.thread", "BARE_VERBS", g_BARE_VERBS, s_BARE_VERBS);
    pyrt::xrefs::reg_access("memory.thread", "CALENDAR", g_CALENDAR, s_CALENDAR);
    pyrt::xrefs::reg_access("memory.thread", "CHECK_A_FACT", g_CHECK_A_FACT, s_CHECK_A_FACT);
    pyrt::xrefs::reg_access("memory.thread", "CHECK_VERDICT", g_CHECK_VERDICT, s_CHECK_VERDICT);
    pyrt::xrefs::reg_access("memory.thread", "CLOSINGS", g_CLOSINGS, s_CLOSINGS);
    pyrt::xrefs::reg_access("memory.thread", "CLOSING_FILLER", g_CLOSING_FILLER, s_CLOSING_FILLER);
    pyrt::xrefs::reg_access("memory.thread", "CONTRACTIONS", g_CONTRACTIONS, s_CONTRACTIONS);
    pyrt::xrefs::reg_access("memory.thread", "COPULA", g_COPULA, s_COPULA);
    pyrt::xrefs::reg_access("memory.thread", "DETAIL_OPENER", g_DETAIL_OPENER, s_DETAIL_OPENER);
    pyrt::xrefs::reg_access("memory.thread", "DETERMINERS", g_DETERMINERS, s_DETERMINERS);
    pyrt::xrefs::reg_access("memory.thread", "DONE_WORDS", g_DONE_WORDS, s_DONE_WORDS);
    pyrt::xrefs::reg_access("memory.thread", "DROPPED_SUBJECT", g_DROPPED_SUBJECT, s_DROPPED_SUBJECT);
    pyrt::xrefs::reg_access("memory.thread", "EXPAND", g_EXPAND, s_EXPAND);
    pyrt::xrefs::reg_access("memory.thread", "FACT_FRAMES", g_FACT_FRAMES, s_FACT_FRAMES);
    pyrt::xrefs::reg_access("memory.thread", "FIRST_PERSON_AUX", g_FIRST_PERSON_AUX, s_FIRST_PERSON_AUX);
    pyrt::xrefs::reg_access("memory.thread", "FLIP", g_FLIP, s_FLIP);
    pyrt::xrefs::reg_access("memory.thread", "HAD_IT", g_HAD_IT, s_HAD_IT);
    pyrt::xrefs::reg_access("memory.thread", "HAPPENED", g_HAPPENED, s_HAPPENED);
    pyrt::xrefs::reg_access("memory.thread", "IRREGULAR_PARTICIPLES", g_IRREGULAR_PARTICIPLES, s_IRREGULAR_PARTICIPLES);
    pyrt::xrefs::reg_access("memory.thread", "IS_NOT_POSSESSIVE", g_IS_NOT_POSSESSIVE, s_IS_NOT_POSSESSIVE);
    pyrt::xrefs::reg_access("memory.thread", "IT_WAS", g_IT_WAS, s_IT_WAS);
    pyrt::xrefs::reg_access("memory.thread", "LEAD", g_LEAD, s_LEAD);
    pyrt::xrefs::reg_access("memory.thread", "LOST_SUBJECT", g_LOST_SUBJECT, s_LOST_SUBJECT);
    pyrt::xrefs::reg_access("memory.thread", "MEASURE", g_MEASURE, s_MEASURE);
    pyrt::xrefs::reg_access("memory.thread", "MODAL", g_MODAL, s_MODAL);
    pyrt::xrefs::reg_access("memory.thread", "NEGATIVE", g_NEGATIVE, s_NEGATIVE);
    pyrt::xrefs::reg_access("memory.thread", "NEVER_A_THING_THAT_WENT", g_NEVER_A_THING_THAT_WENT, s_NEVER_A_THING_THAT_WENT);
    pyrt::xrefs::reg_access("memory.thread", "NON_NOUN", g_NON_NOUN, s_NON_NOUN);
    pyrt::xrefs::reg_access("memory.thread", "NOT_A_FACT", g_NOT_A_FACT, s_NOT_A_FACT);
    pyrt::xrefs::reg_access("memory.thread", "NOT_A_THING", g_NOT_A_THING, s_NOT_A_THING);
    pyrt::xrefs::reg_access("memory.thread", "NOT_THE_THING", g_NOT_THE_THING, s_NOT_THE_THING);
    pyrt::xrefs::reg_access("memory.thread", "NO_FACT_SAID", g_NO_FACT_SAID, s_NO_FACT_SAID);
    pyrt::xrefs::reg_access("memory.thread", "NO_FRAMES", g_NO_FRAMES, s_NO_FRAMES);
    pyrt::xrefs::reg_access("memory.thread", "NO_USED", g_NO_USED, s_NO_USED);
    pyrt::xrefs::reg_access("memory.thread", "NUMBER", g_NUMBER, s_NUMBER);
    pyrt::xrefs::reg_access("memory.thread", "NUMBER_WORD_HEADS", g_NUMBER_WORD_HEADS, s_NUMBER_WORD_HEADS);
    pyrt::xrefs::reg_access("memory.thread", "ON_THE_TABLE_MISSES", g_ON_THE_TABLE_MISSES, s_ON_THE_TABLE_MISSES);
    pyrt::xrefs::reg_access("memory.thread", "ON_THE_TABLE_TURNS", g_ON_THE_TABLE_TURNS, s_ON_THE_TABLE_TURNS);
    pyrt::xrefs::reg_access("memory.thread", "PAST", g_PAST, s_PAST);
    pyrt::xrefs::reg_access("memory.thread", "PLACE_NAMES", g_PLACE_NAMES, s_PLACE_NAMES);
    pyrt::xrefs::reg_access("memory.thread", "PLACE_WORDS", g_PLACE_WORDS, s_PLACE_WORDS);
    pyrt::xrefs::reg_access("memory.thread", "POINTING_WORD", g_POINTING_WORD, s_POINTING_WORD);
    pyrt::xrefs::reg_access("memory.thread", "POINTS_BACK", g_POINTS_BACK, s_POINTS_BACK);
    pyrt::xrefs::reg_access("memory.thread", "POSITIVE", g_POSITIVE, s_POSITIVE);
    pyrt::xrefs::reg_access("memory.thread", "POSSESSIVE", g_POSSESSIVE, s_POSSESSIVE);
    pyrt::xrefs::reg_access("memory.thread", "PREDICATE", g_PREDICATE, s_PREDICATE);
    pyrt::xrefs::reg_access("memory.thread", "PREP", g_PREP, s_PREP);
    pyrt::xrefs::reg_access("memory.thread", "PRONOUN", g_PRONOUN, s_PRONOUN);
    pyrt::xrefs::reg_access("memory.thread", "QUALITY", g_QUALITY, s_QUALITY);
    pyrt::xrefs::reg_access("memory.thread", "RECEIPT_LONGEST", g_RECEIPT_LONGEST, s_RECEIPT_LONGEST);
    pyrt::xrefs::reg_access("memory.thread", "RECEIPT_WORDS", g_RECEIPT_WORDS, s_RECEIPT_WORDS);
    pyrt::xrefs::reg_access("memory.thread", "SECOND_PERSON", g_SECOND_PERSON, s_SECOND_PERSON);
    pyrt::xrefs::reg_access("memory.thread", "SIGNS_OFF", g_SIGNS_OFF, s_SIGNS_OFF);
    pyrt::xrefs::reg_access("memory.thread", "STATE_FILLER", g_STATE_FILLER, s_STATE_FILLER);
    pyrt::xrefs::reg_access("memory.thread", "SUBJECTISH", g_SUBJECTISH, s_SUBJECTISH);
    pyrt::xrefs::reg_access("memory.thread", "SUBJECT_YOU", g_SUBJECT_YOU, s_SUBJECT_YOU);
    pyrt::xrefs::reg_access("memory.thread", "THIN", g_THIN, s_THIN);
    pyrt::xrefs::reg_access("memory.thread", "THING_END", g_THING_END, s_THING_END);
    pyrt::xrefs::reg_access("memory.thread", "THING_PARTICLE", g_THING_PARTICLE, s_THING_PARTICLE);
    pyrt::xrefs::reg_access("memory.thread", "THIRD", g_THIRD, s_THIRD);
    pyrt::xrefs::reg_access("memory.thread", "VERBISH", g_VERBISH, s_VERBISH);
    pyrt::xrefs::reg_access("memory.thread", "VERB_BEFORE", g_VERB_BEFORE, s_VERB_BEFORE);
    pyrt::xrefs::reg_access("memory.thread", "VERB_FORMS", g_VERB_FORMS, s_VERB_FORMS);
    pyrt::xrefs::reg_access("memory.thread", "VERB_WORDS", g_VERB_WORDS, s_VERB_WORDS);
    pyrt::xrefs::reg_access("memory.thread", "VERDICT_SAID", g_VERDICT_SAID, s_VERDICT_SAID);
    pyrt::xrefs::reg_access("memory.thread", "WEAK", g_WEAK, s_WEAK);
    pyrt::xrefs::reg_access("memory.thread", "WENT_AGAIN", g_WENT_AGAIN, s_WENT_AGAIN);
    pyrt::xrefs::reg_access("memory.thread", "WENT_ASK", g_WENT_ASK, s_WENT_ASK);
    pyrt::xrefs::reg_access("memory.thread", "WENT_FRAMES", g_WENT_FRAMES, s_WENT_FRAMES);
    pyrt::xrefs::reg_access("memory.thread", "WENT_FRAMES_NO_THING", g_WENT_FRAMES_NO_THING, s_WENT_FRAMES_NO_THING);
    pyrt::xrefs::reg_access("memory.thread", "WENT_TOLD_BAD", g_WENT_TOLD_BAD, s_WENT_TOLD_BAD);
    pyrt::xrefs::reg_access("memory.thread", "WENT_TOLD_WELL", g_WENT_TOLD_WELL, s_WENT_TOLD_WELL);
    pyrt::xrefs::reg_access("memory.thread", "WH", g_WH, s_WH);
    pyrt::xrefs::reg_access("memory.thread", "WHAT_I_SAID", g_WHAT_I_SAID, s_WHAT_I_SAID);
    pyrt::xrefs::reg_access("memory.thread", "YES_FRAMES", g_YES_FRAMES, s_YES_FRAMES);
    pyrt::xrefs::reg_access("memory.thread", "YES_USED", g_YES_USED, s_YES_USED);
    pyrt::xrefs::reg_access("memory.thread", "_A_DESCRIPTION", g__A_DESCRIPTION, s__A_DESCRIPTION);
    pyrt::xrefs::reg_access("memory.thread", "_A_NAME_FOR_IT", g__A_NAME_FOR_IT, s__A_NAME_FOR_IT);
    pyrt::xrefs::reg_access("memory.thread", "_A_STATE_ITSELF", g__A_STATE_ITSELF, s__A_STATE_ITSELF);
    pyrt::xrefs::reg_access("memory.thread", "_A_THING_DONE", g__A_THING_DONE, s__A_THING_DONE);
    pyrt::xrefs::reg_access("memory.thread", "_ONLY_THE_ARRIVAL", g__ONLY_THE_ARRIVAL, s__ONLY_THE_ARRIVAL);
    pyrt::xrefs::reg_access("memory.thread", "_WENT_WELL", g__WENT_WELL, s__WENT_WELL);
}

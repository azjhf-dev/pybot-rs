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


thread_local! { static G_ADOPTION_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_CHILD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AGE_NUMBER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_AWARD_LOST: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_NEWS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BAD_NEWS_ITSELF: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ACHE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_FILLER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_HURTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_ILL: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BARE_WORD_ONLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARED_SICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARED_SIGN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARER_BODY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CARER_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CHILD_BY_AGE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR_ELSEWHERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR_ELSE_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR_LINES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR_WORK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CLOSED_DOOR_WORK_OWN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_CONDITION_OWNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_DEATH_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_HURTS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_ILL_PART: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_IWE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_JOKED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_KIN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_GONE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_ME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_ME_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEFT_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LEXICON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LIVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_CHANCE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LOST_PATTERN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_LUMP_NOT_A_LUMP: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MARRIAGE_ENDED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_MARRIAGE_ENDED_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEARLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NEWS_WITH_A_NAME: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_A_TYPO: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_NOT_THEIRS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OTHERS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_OWN_PLAN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTNER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTNER_LEAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTNER_LEAVING_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTNER_LEFT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PARTNER_LEFT_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PETS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_LOW: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_LOW_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_NAMED_CONDITION: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PET_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_PREGNANCY_BLEEDING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED_LEAVING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_REPORTED_LEAVING_RE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SACKED_FIRED: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SICK: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_COMING: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_STILL_HERE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUDDEN_LOSS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SUICIDE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_SYMPTOM: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_TROUBLE_OVER: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_VOWELS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__AFTER_IT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__CARRIES: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__DEATH_WORDS: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__HOW_IT_IS_SAID: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__LEAVE_THE_NOUN: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NAMES_ITS_SUBJECT: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__NEARLY: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PERSON: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__PLACE: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__WORD: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G__dream: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_line: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_low: RefCell<Py> = const { RefCell::new(Py::None) }; }
thread_local! { static G_without_the_figure_of_speech: RefCell<Py> = const { RefCell::new(Py::None) }; }
static INIT: Once = Once::new();
thread_local! { static INITIALIZING: Cell<bool> = const { Cell::new(false) }; }
pub fn ensure_init() {
    if INITIALIZING.with(|c| c.get()) { return; }
    INIT.call_once(|| {
        INITIALIZING.with(|c| c.set(true));
        let _ = exc::catch(|| { module_body(); Py::None }).map_err(|e| { if std::env::var_os("PYBOT_TRACE").is_some() { eprintln!("INIT compose.badnews: {} {}", e.class_name, e.message); } });
        INITIALIZING.with(|c| c.set(false));
    });
}
pub fn g_ADOPTION_WORDS() -> Py { ensure_init(); G_ADOPTION_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ADOPTION_WORDS(v: Py) { ensure_init(); G_ADOPTION_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_CHILD() -> Py { ensure_init(); G_AGE_CHILD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_CHILD(v: Py) { ensure_init(); G_AGE_CHILD.with(|c| *c.borrow_mut() = v); }
pub fn g_AGE_NUMBER() -> Py { ensure_init(); G_AGE_NUMBER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AGE_NUMBER(v: Py) { ensure_init(); G_AGE_NUMBER.with(|c| *c.borrow_mut() = v); }
pub fn g_AWARD_LOST() -> Py { ensure_init(); G_AWARD_LOST.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_AWARD_LOST(v: Py) { ensure_init(); G_AWARD_LOST.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_NEWS() -> Py { ensure_init(); G_BAD_NEWS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_NEWS(v: Py) { ensure_init(); G_BAD_NEWS.with(|c| *c.borrow_mut() = v); }
pub fn g_BAD_NEWS_ITSELF() -> Py { ensure_init(); G_BAD_NEWS_ITSELF.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BAD_NEWS_ITSELF(v: Py) { ensure_init(); G_BAD_NEWS_ITSELF.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ACHE() -> Py { ensure_init(); G_BARE_ACHE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ACHE(v: Py) { ensure_init(); G_BARE_ACHE.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_BODY() -> Py { ensure_init(); G_BARE_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_BODY(v: Py) { ensure_init(); G_BARE_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_FILLER() -> Py { ensure_init(); G_BARE_FILLER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_FILLER(v: Py) { ensure_init(); G_BARE_FILLER.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_HURTS() -> Py { ensure_init(); G_BARE_HURTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_HURTS(v: Py) { ensure_init(); G_BARE_HURTS.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_ILL() -> Py { ensure_init(); G_BARE_ILL.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_ILL(v: Py) { ensure_init(); G_BARE_ILL.with(|c| *c.borrow_mut() = v); }
pub fn g_BARE_WORD_ONLY() -> Py { ensure_init(); G_BARE_WORD_ONLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BARE_WORD_ONLY(v: Py) { ensure_init(); G_BARE_WORD_ONLY.with(|c| *c.borrow_mut() = v); }
pub fn g_BODY() -> Py { ensure_init(); G_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_BODY(v: Py) { ensure_init(); G_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_CARED_SICK() -> Py { ensure_init(); G_CARED_SICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARED_SICK(v: Py) { ensure_init(); G_CARED_SICK.with(|c| *c.borrow_mut() = v); }
pub fn g_CARED_SIGN() -> Py { ensure_init(); G_CARED_SIGN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARED_SIGN(v: Py) { ensure_init(); G_CARED_SIGN.with(|c| *c.borrow_mut() = v); }
pub fn g_CARER_BODY() -> Py { ensure_init(); G_CARER_BODY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARER_BODY(v: Py) { ensure_init(); G_CARER_BODY.with(|c| *c.borrow_mut() = v); }
pub fn g_CARER_SYMPTOM() -> Py { ensure_init(); G_CARER_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CARER_SYMPTOM(v: Py) { ensure_init(); G_CARER_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_CHILD_BY_AGE() -> Py { ensure_init(); G_CHILD_BY_AGE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CHILD_BY_AGE(v: Py) { ensure_init(); G_CHILD_BY_AGE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR() -> Py { ensure_init(); G_CLOSED_DOOR.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR(v: Py) { ensure_init(); G_CLOSED_DOOR.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR_ELSEWHERE() -> Py { ensure_init(); G_CLOSED_DOOR_ELSEWHERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR_ELSEWHERE(v: Py) { ensure_init(); G_CLOSED_DOOR_ELSEWHERE.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR_ELSE_OWN() -> Py { ensure_init(); G_CLOSED_DOOR_ELSE_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR_ELSE_OWN(v: Py) { ensure_init(); G_CLOSED_DOOR_ELSE_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR_LINES() -> Py { ensure_init(); G_CLOSED_DOOR_LINES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR_LINES(v: Py) { ensure_init(); G_CLOSED_DOOR_LINES.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR_WORK() -> Py { ensure_init(); G_CLOSED_DOOR_WORK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR_WORK(v: Py) { ensure_init(); G_CLOSED_DOOR_WORK.with(|c| *c.borrow_mut() = v); }
pub fn g_CLOSED_DOOR_WORK_OWN() -> Py { ensure_init(); G_CLOSED_DOOR_WORK_OWN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CLOSED_DOOR_WORK_OWN(v: Py) { ensure_init(); G_CLOSED_DOOR_WORK_OWN.with(|c| *c.borrow_mut() = v); }
pub fn g_CONDITION_OWNER() -> Py { ensure_init(); G_CONDITION_OWNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_CONDITION_OWNER(v: Py) { ensure_init(); G_CONDITION_OWNER.with(|c| *c.borrow_mut() = v); }
pub fn g_DEATH_SAID() -> Py { ensure_init(); G_DEATH_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_DEATH_SAID(v: Py) { ensure_init(); G_DEATH_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g_HURT() -> Py { ensure_init(); G_HURT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURT(v: Py) { ensure_init(); G_HURT.with(|c| *c.borrow_mut() = v); }
pub fn g_HURTS() -> Py { ensure_init(); G_HURTS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_HURTS(v: Py) { ensure_init(); G_HURTS.with(|c| *c.borrow_mut() = v); }
pub fn g_ILL_PART() -> Py { ensure_init(); G_ILL_PART.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_ILL_PART(v: Py) { ensure_init(); G_ILL_PART.with(|c| *c.borrow_mut() = v); }
pub fn g_IWE() -> Py { ensure_init(); G_IWE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_IWE(v: Py) { ensure_init(); G_IWE.with(|c| *c.borrow_mut() = v); }
pub fn g_JOKED() -> Py { ensure_init(); G_JOKED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_JOKED(v: Py) { ensure_init(); G_JOKED.with(|c| *c.borrow_mut() = v); }
pub fn g_KIN() -> Py { ensure_init(); G_KIN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_KIN(v: Py) { ensure_init(); G_KIN.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_GONE() -> Py { ensure_init(); G_LEFT_GONE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_GONE(v: Py) { ensure_init(); G_LEFT_GONE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_ME() -> Py { ensure_init(); G_LEFT_ME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_ME(v: Py) { ensure_init(); G_LEFT_ME.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_ME_RE() -> Py { ensure_init(); G_LEFT_ME_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_ME_RE(v: Py) { ensure_init(); G_LEFT_ME_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_LEFT_PLAN() -> Py { ensure_init(); G_LEFT_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEFT_PLAN(v: Py) { ensure_init(); G_LEFT_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_LEXICON() -> Py { ensure_init(); G_LEXICON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LEXICON(v: Py) { ensure_init(); G_LEXICON.with(|c| *c.borrow_mut() = v); }
pub fn g_LIVING() -> Py { ensure_init(); G_LIVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LIVING(v: Py) { ensure_init(); G_LIVING.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_CHANCE() -> Py { ensure_init(); G_LOST_CHANCE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_CHANCE(v: Py) { ensure_init(); G_LOST_CHANCE.with(|c| *c.borrow_mut() = v); }
pub fn g_LOST_PATTERN() -> Py { ensure_init(); G_LOST_PATTERN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LOST_PATTERN(v: Py) { ensure_init(); G_LOST_PATTERN.with(|c| *c.borrow_mut() = v); }
pub fn g_LUMP_NOT_A_LUMP() -> Py { ensure_init(); G_LUMP_NOT_A_LUMP.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_LUMP_NOT_A_LUMP(v: Py) { ensure_init(); G_LUMP_NOT_A_LUMP.with(|c| *c.borrow_mut() = v); }
pub fn g_MARRIAGE_ENDED() -> Py { ensure_init(); G_MARRIAGE_ENDED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MARRIAGE_ENDED(v: Py) { ensure_init(); G_MARRIAGE_ENDED.with(|c| *c.borrow_mut() = v); }
pub fn g_MARRIAGE_ENDED_RE() -> Py { ensure_init(); G_MARRIAGE_ENDED_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_MARRIAGE_ENDED_RE(v: Py) { ensure_init(); G_MARRIAGE_ENDED_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_NEARLY() -> Py { ensure_init(); G_NEARLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEARLY(v: Py) { ensure_init(); G_NEARLY.with(|c| *c.borrow_mut() = v); }
pub fn g_NEWS_WITH_A_NAME() -> Py { ensure_init(); G_NEWS_WITH_A_NAME.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NEWS_WITH_A_NAME(v: Py) { ensure_init(); G_NEWS_WITH_A_NAME.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_A_TYPO() -> Py { ensure_init(); G_NOT_A_TYPO.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_A_TYPO(v: Py) { ensure_init(); G_NOT_A_TYPO.with(|c| *c.borrow_mut() = v); }
pub fn g_NOT_THEIRS() -> Py { ensure_init(); G_NOT_THEIRS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_NOT_THEIRS(v: Py) { ensure_init(); G_NOT_THEIRS.with(|c| *c.borrow_mut() = v); }
pub fn g_OTHERS() -> Py { ensure_init(); G_OTHERS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OTHERS(v: Py) { ensure_init(); G_OTHERS.with(|c| *c.borrow_mut() = v); }
pub fn g_OWN_PLAN() -> Py { ensure_init(); G_OWN_PLAN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_OWN_PLAN(v: Py) { ensure_init(); G_OWN_PLAN.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTNER() -> Py { ensure_init(); G_PARTNER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTNER(v: Py) { ensure_init(); G_PARTNER.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTNER_LEAVING() -> Py { ensure_init(); G_PARTNER_LEAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTNER_LEAVING(v: Py) { ensure_init(); G_PARTNER_LEAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTNER_LEAVING_RE() -> Py { ensure_init(); G_PARTNER_LEAVING_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTNER_LEAVING_RE(v: Py) { ensure_init(); G_PARTNER_LEAVING_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTNER_LEFT() -> Py { ensure_init(); G_PARTNER_LEFT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTNER_LEFT(v: Py) { ensure_init(); G_PARTNER_LEFT.with(|c| *c.borrow_mut() = v); }
pub fn g_PARTNER_LEFT_RE() -> Py { ensure_init(); G_PARTNER_LEFT_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PARTNER_LEFT_RE(v: Py) { ensure_init(); G_PARTNER_LEFT_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_PETS() -> Py { ensure_init(); G_PETS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PETS(v: Py) { ensure_init(); G_PETS.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_CONDITION() -> Py { ensure_init(); G_PET_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_CONDITION(v: Py) { ensure_init(); G_PET_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_LOW() -> Py { ensure_init(); G_PET_LOW.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_LOW(v: Py) { ensure_init(); G_PET_LOW.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_LOW_SYMPTOM() -> Py { ensure_init(); G_PET_LOW_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_LOW_SYMPTOM(v: Py) { ensure_init(); G_PET_LOW_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_NAMED_CONDITION() -> Py { ensure_init(); G_PET_NAMED_CONDITION.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_NAMED_CONDITION(v: Py) { ensure_init(); G_PET_NAMED_CONDITION.with(|c| *c.borrow_mut() = v); }
pub fn g_PET_SYMPTOM() -> Py { ensure_init(); G_PET_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PET_SYMPTOM(v: Py) { ensure_init(); G_PET_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_PREGNANCY_BLEEDING() -> Py { ensure_init(); G_PREGNANCY_BLEEDING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_PREGNANCY_BLEEDING(v: Py) { ensure_init(); G_PREGNANCY_BLEEDING.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED_LEAVING() -> Py { ensure_init(); G_REPORTED_LEAVING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED_LEAVING(v: Py) { ensure_init(); G_REPORTED_LEAVING.with(|c| *c.borrow_mut() = v); }
pub fn g_REPORTED_LEAVING_RE() -> Py { ensure_init(); G_REPORTED_LEAVING_RE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_REPORTED_LEAVING_RE(v: Py) { ensure_init(); G_REPORTED_LEAVING_RE.with(|c| *c.borrow_mut() = v); }
pub fn g_SACKED_FIRED() -> Py { ensure_init(); G_SACKED_FIRED.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SACKED_FIRED(v: Py) { ensure_init(); G_SACKED_FIRED.with(|c| *c.borrow_mut() = v); }
pub fn g_SICK() -> Py { ensure_init(); G_SICK.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SICK(v: Py) { ensure_init(); G_SICK.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_COMING() -> Py { ensure_init(); G_STILL_COMING.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_COMING(v: Py) { ensure_init(); G_STILL_COMING.with(|c| *c.borrow_mut() = v); }
pub fn g_STILL_HERE() -> Py { ensure_init(); G_STILL_HERE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_STILL_HERE(v: Py) { ensure_init(); G_STILL_HERE.with(|c| *c.borrow_mut() = v); }
pub fn g_SUDDEN_LOSS() -> Py { ensure_init(); G_SUDDEN_LOSS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUDDEN_LOSS(v: Py) { ensure_init(); G_SUDDEN_LOSS.with(|c| *c.borrow_mut() = v); }
pub fn g_SUICIDE() -> Py { ensure_init(); G_SUICIDE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SUICIDE(v: Py) { ensure_init(); G_SUICIDE.with(|c| *c.borrow_mut() = v); }
pub fn g_SYMPTOM() -> Py { ensure_init(); G_SYMPTOM.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_SYMPTOM(v: Py) { ensure_init(); G_SYMPTOM.with(|c| *c.borrow_mut() = v); }
pub fn g_TROUBLE_OVER() -> Py { ensure_init(); G_TROUBLE_OVER.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_TROUBLE_OVER(v: Py) { ensure_init(); G_TROUBLE_OVER.with(|c| *c.borrow_mut() = v); }
pub fn g_VOWELS() -> Py { ensure_init(); G_VOWELS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_VOWELS(v: Py) { ensure_init(); G_VOWELS.with(|c| *c.borrow_mut() = v); }
pub fn g__AFTER_IT() -> Py { ensure_init(); G__AFTER_IT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__AFTER_IT(v: Py) { ensure_init(); G__AFTER_IT.with(|c| *c.borrow_mut() = v); }
pub fn g__CARRIES() -> Py { ensure_init(); G__CARRIES.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__CARRIES(v: Py) { ensure_init(); G__CARRIES.with(|c| *c.borrow_mut() = v); }
pub fn g__DEATH_WORDS() -> Py { ensure_init(); G__DEATH_WORDS.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__DEATH_WORDS(v: Py) { ensure_init(); G__DEATH_WORDS.with(|c| *c.borrow_mut() = v); }
pub fn g__HOW_IT_IS_SAID() -> Py { ensure_init(); G__HOW_IT_IS_SAID.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__HOW_IT_IS_SAID(v: Py) { ensure_init(); G__HOW_IT_IS_SAID.with(|c| *c.borrow_mut() = v); }
pub fn g__LEAVE_THE_NOUN() -> Py { ensure_init(); G__LEAVE_THE_NOUN.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__LEAVE_THE_NOUN(v: Py) { ensure_init(); G__LEAVE_THE_NOUN.with(|c| *c.borrow_mut() = v); }
pub fn g__NAMES_ITS_SUBJECT() -> Py { ensure_init(); G__NAMES_ITS_SUBJECT.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NAMES_ITS_SUBJECT(v: Py) { ensure_init(); G__NAMES_ITS_SUBJECT.with(|c| *c.borrow_mut() = v); }
pub fn g__NEARLY() -> Py { ensure_init(); G__NEARLY.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__NEARLY(v: Py) { ensure_init(); G__NEARLY.with(|c| *c.borrow_mut() = v); }
pub fn g__PERSON() -> Py { ensure_init(); G__PERSON.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PERSON(v: Py) { ensure_init(); G__PERSON.with(|c| *c.borrow_mut() = v); }
pub fn g__PLACE() -> Py { ensure_init(); G__PLACE.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__PLACE(v: Py) { ensure_init(); G__PLACE.with(|c| *c.borrow_mut() = v); }
pub fn g__WORD() -> Py { ensure_init(); G__WORD.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__WORD(v: Py) { ensure_init(); G__WORD.with(|c| *c.borrow_mut() = v); }
pub fn g__dream() -> Py { ensure_init(); G__dream.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s__dream(v: Py) { ensure_init(); G__dream.with(|c| *c.borrow_mut() = v); }
pub fn g_line() -> Py { ensure_init(); G_line.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_line(v: Py) { ensure_init(); G_line.with(|c| *c.borrow_mut() = v); }
pub fn g_low() -> Py { ensure_init(); G_low.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_low(v: Py) { ensure_init(); G_low.with(|c| *c.borrow_mut() = v); }
pub fn g_without_the_figure_of_speech() -> Py { ensure_init(); G_without_the_figure_of_speech.with(|c| c.borrow().clone()) }
#[allow(non_snake_case)]
pub fn s_without_the_figure_of_speech(v: Py) { ensure_init(); G_without_the_figure_of_speech.with(|c| *c.borrow_mut() = v); }
fn module_body() {
    let _ = py_str("Lines that are news of the hard kind: an illness, an emergency, money trouble, a loss.\n\nThey share no word, so they are patterns rather than a word list: \"our cat was put down\" and\n\"my mum has cancer\" and \"we are behind on the rent\" are one kind of line to a friend, and that\nkind is never answered with \"Noted!\" or by repeating it back. Two callers need the same list,\nwhich is why it lives here on its own:\n\n* bot/compose/fallback.py settles the mood on it, so the stock reply is a gentle one;\n* bot/memory/ack.py refuses to echo it, because \"your cat was put down yesterday — noted\"\n  would be awful.\n\nEvery pattern is matched against the cleaned, lowercased line (understanding's memory\n[\"understood\"][\"clean\"]), so it must not depend on capitals or punctuation.\n\nA death is left to bot/compose/loss.py, which reads it properly and answers it before any of\nthis is asked; a loss that loss.py does not read (\"a miscarriage\", \"terminally ill\") is here so\nthat it is still never noted.\n\nThe hard part is that the same word is news in one line and ordinary in another: \"my dad is in\nhospital\" is news, \"i work at the hospital\" is a job, \"my son took all my pills\" is an\nemergency, and a paramedic talking about shifts is neither. So the words that only mean\nsomething about a person or an animal are written that way: the person or the animal has to be\nthere in the line.\n");
    let __try1 = exc::catch(|| {
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
    s__dream(Py::None);
            }
            if !__handled5 { __unhandled4 = Some(__e); }
        }
    }
    if let Some(__e) = __unhandled4 { exc::raise(__e); }
    s_LEXICON(pyrt::xrefs::xget("understanding.normalize", "LEXICON"));
    s_without_the_figure_of_speech(py_func("without_the_figure_of_speech", |args: &[Py]| pyrt::xrefs::xcall("understanding.normalize", "without_the_figure_of_speech", args)));
    s_LIVING(py_str("(?:mum|mom|mother|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|brother|sister|son|daughter|child|baby|wife|husband|partner|boyfriend|girlfriend|friend|mate|uncle|aunt|cousin|neighbour|neighbor|colleague|boss|teacher|ex|dog|cat|pet|horse|rabbit|puppy|kitten|bird)"));
    s_CONDITION_OWNER(op_add(&slice_or_raise(&g_LIVING(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str("|nephew|niece|stepson|stepdaughter|stepmum|stepdad|stepmother|stepfather|grandson|granddaughter|mother in law|father in law|son in law|daughter in law|grandchild|grandchildren)")));
    s__HOW_IT_IS_SAID(py_str("died(?!\\s+(?:laughing|giggling|smiling|of\\s+(?:embarrassment|boredom|shame|laughter|fear)|inside|a\\s+(?:little|bit)|and\\s+gone|(?:her|his|their|my|our|your|its)\\s+hair))|dead(?!\\s+(?:funny|good|nice|cheap|easy|serious|certain|jealous|tired|right|wrong|set|on\\b|beat|last\\b|chuffed|proud|pleased|happy|keen|excited|thrilled|impressed|lucky|jammy|buzzing|gutted|made\\s+up|against|ahead|centre|straight|silence|giveaway|heat|calm|in\\s+(?:the\\s+middle|a\\s+row)|ringer|to\\s+(?:the\\s+world|rights|me|us|him|her|them)\\b))"));
    s__DEATH_WORDS(op_add(&g__HOW_IT_IS_SAID(), &py_str("|passed away|killed himself|killed herself|killed themselves|hanged himself|hanged herself|hanged themselves|took his own life|took her own life|took their own lives|committed suicide|suicid\\w*|found dead|passed on(?!\\s+(?:the|a|an|my|his|her|their|our|your|its|it|this|that|these|those|some|all|both)\\b)|didn'?t make it(?!\\s+(?:(?:to|for)\\s+\\w|in\\s+time|on\\s+time))|did not make it(?!\\s+(?:(?:to|for)\\s+\\w|in\\s+time|on\\s+time))")));
    s__NEARLY(stdlib_call("re", "compile", &[py_str("\\b(?:nearly|almost|practically|might\\s+have|could\\s+have|would\\s+have|like\\s+to\\s+have)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__AFTER_IT(stdlib_call("re", "compile", &[py_str("^\\s*(?:laughing|giggling|smiling|of\\s+(?:embarrassment|boredom|shame|laughter|fear)|to\\s+(?:me|us|him|her|them)\\b)"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DEATH_SAID(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:"); __s.push_str(&fstr(&g__DEATH_WORDS(), -1, "")); __s.push_str(")\\b"); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e)));
    s_OTHERS(py_str("(?:boy|girl|lad|lass|kid|child|man|woman|person|someone|anyone|classmate|schoolmate|teammate|workmate|student|pupil|neighbour|neighbor|friend|mate|fella|guy|colleague|he|she|they|everyone|everybody)"));
    s__CARRIES(call_builtin("frozenset", &[mcall(&py_str("is was were are am be been being has had have just also nearly almost\nstill of in at on to for with that who which my his her their our your its the a an and but so\nthen today yesterday now last this from like about after before when while if sadly tragically\nunexpectedly peacefully suddenly quietly finally recently apparently presumably unfortunately\nhonestly basically literally obviously definitely really very"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PLACE(call_builtin("frozenset", &[mcall(&py_str("class classes year school work office team family ward house hospital\nfloor street town village college uni university club gym shop company firm site shift course\ngroup nursery classroom road estate"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__PERSON(op_bitor(&call_builtin("frozenset", &[mcall(&mcall(&mcall(&g_LIVING(), "lstrip", &[py_str("(?:")]), "rstrip", &[py_str(")")]), "split", &[py_str("|")])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("frozenset", &[mcall(&mcall(&mcall(&g_OTHERS(), "lstrip", &[py_str("(?:")]), "rstrip", &[py_str(")")]), "split", &[py_str("|")])]).unwrap_or_else(|e| exc::raise(e))));
    s__WORD(stdlib_call("re", "compile", &[py_str("[a-z']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_NEARLY(call_builtin("frozenset", &[py_tuple(vec![py_str("seizure"), py_str("unconscious"), py_str("overdose"), py_str("choking"), py_str("breathing"), py_str("collapsed"), py_str("hospital"), py_str("ambulance"), py_str("cancer"), py_str("stroke"), py_str("bleeding"), py_str("diagnosed")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LOST_CHANCE(py_str("jobs?|work|role|position|match(?:es)?|games?|deals?|contracts?|shift|hours|clients?|customers?|accounts?|bids?|bets?|elections?|deposits?|offers?|interviews?|places?|spots?|funding|grants?|appeals?|races?|finals?|trials?|orders?|bookings?|tenanc(?:y|ies)|tenders?|placement|promotion|apprenticeship"));
    s_LOST_PATTERN(op_add(&op_add(&py_str("\\b(?:i|we|he|she|they|ive|i've|my [a-z]+|our [a-z]+)\\s+(?:have\\s+|had\\s+)?(?:lost|losing)\\s+(?:out on\\s+)?(?:the |my |our |his |her |their |your |that |this |a |an )?(?:"), &g_LOST_CHANCE()), &py_str(")\\b")));
    s_LEFT_ME(py_str("(?:\\b(?:he|she|they|my (?:wife|husband|partner|boyfriend|girlfriend|fianc\\w*|ex))[^.!?]{0,24}\\b(?:left me|leaving me|leave me|left and|walked out|dumped me|cheated on me)\\b(?!\\s+(?:her|his|their|the|a|an|this|that|alone|be)\\b))"));
    s_LEFT_ME_RE(stdlib_call("re", "compile", &[g_LEFT_ME(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTNER_LEFT(py_str("\\b(?:my (?:wife|husband|partner|boyfriend|girlfriend|fianc\\w*|ex)|my marriage)\\b[^.!?]{0,30}?\\b(?:has|have|had|just|recently|already|suddenly|now|then|finally)?\\s*\\bleft\\b(?=\\s*(?:$|[.!?,;]|\\b(?:me|us|and|but|because|so|now|already|again|too|last|this|yesterday|today|tonight|morning|evening|week|month|year|night|day|suddenly|recently|at\\s+(?:christmas|new\\s+year|the\\s+weekend|easter)|in\\s+\\w+|after\\s+\\w+|before\\s+\\w+|since\\s+\\w+)\\b))"));
    s_PARTNER_LEFT_RE(stdlib_call("re", "compile", &[g_PARTNER_LEFT(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTNER_LEAVING(py_str("\\b(?:my (?:wife|husband|partner|boyfriend|girlfriend|fianc\\w*|ex)|my marriage)\\b[^.!?]{0,30}?\\b(?:is|are|was|were|keeps?|kept|been|got|has|have|says?|said|told me|tells me|wants?|wanted|might|may|will|would|thinks?|thinking)\\b[^.!?]{0,26}?\\bleav(?:e|es|ing)\\b(?!\\s+(?:me\\b|for\\b|to\\b|on\\b|at\\b|in\\b|early\\b|home\\b|work\\b|school\\b|the\\b|her\\b|his\\b|their\\b|a\\b|an\\b|this\\b|that\\b|approved\\b|booked\\b|due\\b|owing\\b|remaining\\b|allowance\\b|entitlement\\b|days?\\b|pay\\b|request\\b|year\\b|starts?\\b|begins?\\b|ends?\\b|cancell?ed\\b|denied\\b|refused\\b|pending\\b|confirmed\\b|carried\\b|unused\\b|balance\\b))"));
    s_PARTNER_LEAVING_RE(stdlib_call("re", "compile", &[g_PARTNER_LEAVING(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_REPORTED_LEAVING(py_str("\\b(?:he|she|they|hes|shes|theyre)\\b[^.!?]{0,24}?\\b(?:said|says|told me|tells me|reckons|thinks|keeps saying)\\b(?:\\s+(?:that|he|she|they|hes|shes|theyre|he is|she is|they are))?(?:\\s+(?:is|are))?(?:\\s+(?:going|planning|thinking|wants?|wanted|wanted to|might|may|will|would|ready|about|seriously)\\b(?:\\s+(?:to|of|about))?)?\\s+\\bleav(?:e|es|ing)\\b(?!\\s+(?:me\\b|for\\b|to\\b|on\\b|at\\b|in\\b|early\\b|home\\b|work\\b|school\\b|the\\b|her\\b|his\\b|their\\b|him\\b|a\\b|an\\b|this\\b|that\\b|soon\\b|now\\b|tonight\\b|tomorrow\\b|today\\b|next\\b|after\\b|before\\b|hospital\\b|care\\b|rehab\\b|prison\\b|jail\\b|army\\b|navy\\b|force\\b|uni\\b|university\\b|college\\b|again\\b))"));
    s_REPORTED_LEAVING_RE(stdlib_call("re", "compile", &[g_REPORTED_LEAVING(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEFT_GONE(py_str("\\b(?:left me|left and|walked out|dumped me|moved out|packed (?:her|his|their) (?:things|bags)|gone|broke up|broken up|split up|separated|divorced)\\b"));
    s_STILL_COMING(py_str("\\b(?:is|are|was|were|keeps?|kept|said|says|told|tells|wants?|wanted|might|may|will|would|going|planning|thinking|decided)\\b[^.!?]{0,26}?\\bleav(?:e|es|ing)\\b"));
    s_STILL_HERE(py_str("\\b(?:still (?:live|living|here|in the house|together)|lives? with me|under the same roof|separated but)\\b"));
    s_MARRIAGE_ENDED(py_str("\\b(?:my|the)\\s+(?:marriage|relationship)\\b[^.!?]{0,24}?\\b(?:is|are|was|were|has|have|seems?|looks?)\\b[^.!?]{0,16}?\\b(?:over|done|finished|ended)\\b(?!\\s+(?:the|it|with|there|here|that|this|to|by|at|and)\\b)|\\b(?:we|me\\s+and\\s+my\\s+\\w+)\\b[^.!?]{0,20}?\\b(?:split\\s+up|broke\\s+up|broken\\s+up|separated|divorced)\\b(?!\\s+(?:the|a|into|over|it)\\b)|\\b(?:getting|got|had|filed\\s+for|asked\\s+for|asking\\s+for|wants?|wanted)\\s+(?:a\\s+)?divorce\\w*\\b(?!\\s+(?:lawyer|solicitor|court|papers|settlement|money|bill|forms?)\\b)"));
    s_MARRIAGE_ENDED_RE(stdlib_call("re", "compile", &[g_MARRIAGE_ENDED(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_THEIRS(stdlib_call("re", "compile", &[py_str("\\bmy\\s+(?:mate|mates|friend|friends|best\\s+friend|brother|sister|colleague|boss|cousin|neighbour|neighbor|aunt|uncle|son|daughter|mum|mom|dad|mother|father|nan|gran|grandad|parents?|family|in[-\\s]?laws?|step\\w+|grand\\w+|landlord)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTNER(stdlib_call("re", "compile", &[py_str("\\b(?:wife|husband|partner|boyfriend|girlfriend|fianc\\w*|ex)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s__LEAVE_THE_NOUN(stdlib_call("re", "compile", &[py_str("\\b(?:his|her|their|my|our|your|its|the|some|any|no|annual|maternity|paternity|parental|shared|sick|paid|unpaid|compassionate|study|garden|special|sabbatical|holiday)\\s+leave\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_LEFT_PLAN(py_str("\\b(?:thinking (?:of|about)|thought (?:of|about)|want(?:s|ed)? to|wanting to|going to|planning (?:on|to)|plan to|decided to|ready to|about to|meant to|should i|shall i|do i)\\s+(?:just\\s+|actually\\s+|really\\s+)?(?:leave|leaving|walk out on|end)\\s+(?:(?:my|our) (?:wife|husband|partner|boyfriend|girlfriend|fianc\\w*|marriage|ex|other half)|him|her|them|us)\\b|\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+(?:just\\s+|actually\\s+)?(?:leaving|walking out on)\\s+(?:my (?:wife|husband|partner|boyfriend|girlfriend|fianc\\w*|ex|other half)|him|her|them)\\b(?!\\s+(?:a|an|the|some|his|her|my|our)\\b)"));
    s_OWN_PLAN(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+(?:just\\s+|actually\\s+|finally\\s+)?(?:going to|gonna|planning to|plan to|decided to|ready to|about to|wanting to|thinking (?:of|about)|want to)\\s+(?:just\\s+|actually\\s+|really\\s+)?(?:leave|leaving|walk out on|end it with)\\s+"), py_str("\\b(?:i'?m|im|i am|we'?re|were|we are)\\s+(?:just\\s+|actually\\s+|finally\\s+)?(?:leaving|walking out on)\\s+"), py_str("\\b(?:i|we)\\s+(?:have|'ve|ve)\\s+(?:decided|made up my mind)\\b"), py_str("\\bwe(?:'?re| are)?\\s+(?:going to|gonna)\\s+(?:split|separate|divorce)\\b")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_SACKED_FIRED(py_str("\\b(?:i|we|he|she|they|my [a-z]+|our [a-z]+)\\s*(?:just\\s+|only\\s+|then\\s+|now\\s+|today\\s+)?(?:'?ve|have|has|had|was|were|been|being|got|get|gets|getting|is|are|am)\\s+(?:been\\s+|being\\s+)?(?:sacked|fired(?!\\s+up\\b)|dismissed)\\b|\\b(?:they|he|she|the company|the firm|the shop|work|my boss|my manager|my employer)\\s+(?:just\\s+)?(?:sacked|fired(?!\\s+up\\b)|dismissed)\\s+(?:me|him|them|us)\\b|\\b(?:they|the company|the firm|the shop|work|my boss|my manager|my employer)\\s+let\\s+(?:me|him|her|them)\\s+go\\b(?!\\s+(?:home|early|out|to|and|for a)\\b)(?!\\s+on\\s+(?:holiday|strike|leave|a\\b|the\\b|maternity|paternity|benefit|tour|safari))|\\b(?:got|gotten|gave me|handed me|give me)\\s+the (?:sack|boot|push)\\b(?!\\s+of\\b)"));
    s_ADOPTION_WORDS(py_str("\\b(?:adopt\\w*|adoption)\\b"));
    s_BAD_NEWS_ITSELF(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("\\b(?:diagnos\\w+|cancer|chemo\\w*|tumou?r|biopsy)\\b"), py_str("(?<!\\bwork )(?<!\\bworks )(?<!\\bworked )\\bin (?:the )?hospitals?\\b"), py_str("\\b(?:a&e|icu|ambulances?|overdose|surgery|operation|anaesthetic)\\b"), py_str("\\b(?:strokes?|heart attacks?|seizures?|chest pains?)\\b"), py_str("\\bcan'?t keep (?:anything|any food|food|fluids?|liquids?|water|solids?) down\\b"), { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("\\b[^.!?]{0,16}\\bcut (?:her|his|their|its)self\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("\\b[^.!?]{0,16}\\b(?:has|had|got) (?:a |an )?(?:\\w+ )?(?:cut|wound|gash|burn)\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("\\b[^.!?]{0,16}\\b(?:cut|burnt|burned|bleeding|bled)\\b[^.!?]{0,12}\\b(?:her|his|their)\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("\\b[^.!?]{0,24}\\b(?:collaps(?:e|ed|ing)|fainted|passed out|blacked out|not breathing|can'?t breathe|choking|bleeding badly)\\b"); py_string(__s) }, py_str("\\bi\\b[^.!?]{0,16}\\b(?:collapsed|fainted|passed out|blacked out|threw up|vomited|was sick all|been sick all)\\b"), py_str("\\b(?:kill myself|killing myself|end my life|take my own life|want to die|better off dead)\\b(?!\\s+(?:my|your|his|her|their|our|the|a|an|this|that|it)\\b)"), py_str("\\b(?:won'?t let me leave|hits me|hit me|hurts? me|hurting me|threatened me|took my phone|takes my phone)\\b"), py_str("\\b(?:on fire|smoke in|gas leak|smell(?:s)? (?:of )?gas)\\b"), py_str("\\b(?:took|taken) (?:all|too many|the rest of) (?:my|his|her|their|the) pills\\b"), py_str("\\b(?:broke up|split up|divorce\\w*|redundant|laid off|evicted|stolen|robbed|scammed)\\b"), g_SACKED_FIRED(), g_LEFT_ME(), g_LEFT_PLAN(), py_str("\\b(?:in debt|arrears|behind on (?:the )?(?:rent|mortgage|payments|bills)|can'?t afford|can not afford)\\b"), py_str("\\b(?:didn't get|didn't work out|didn't make it|turned down|turned it down|no luck|wasn't picked|not chosen)\\b"), g_LOST_PATTERN(), py_str("\\bmiscarriage(?:s)?\\b(?! of justice)|\\bmiscarried\\b"), py_str("\\bstill ?born\\b|\\bstill ?birth\\b"), py_str("\\b(?:lost|losing|we lost) (?:the|our|her|his|their) baby\\b(?!\\s+(?:weight|belly|tummy|fat|pounds?|stones?|kilos?|inches|clothes|things|stuff|room|pram|toys?|seat|carrier|monitor|name|bag|book)\\b)"), { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("\\b[^.!?]{0,24}\\bkilled (?:him|her|them)sel(?:f|ves)\\b(?!\\s+(?:laughing|giggling))"); py_string(__s) }, py_str("\\b(?:took|taken|taking) (?:his|her|their) own life\\b|\\bcommitted suicide\\b"), py_str("\\bpregnan\\w+\\b[^.!?]{0,24}\\b(?:spotting|bleeding|cramping)\\b"), py_str("\\b(?:spotting|bleeding)\\b[^.!?]{0,24}\\bpregnan\\w+\\b")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_AWARD_LOST(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("\\b(?:pip|dla|esa|universal credit|housing benefit|child benefit|tax credits?|benefits?|allowance|pension|pay|wages|hours)\\b[^.!?]{0,24}\\b(?:stopped|cancell?ed|cut|suspended|frozen|refused|sanctioned|withdrawn|reduced|turned down|taken away|not renewed)\\b"), py_str("\\b(?:stopped|cancell?ed|cut|suspended|frozen|sanctioned|withdrawn|refused|reduced)\\b[^.!?]{0,20}\\b(?:my|our)\\s+(?:pip|dla|esa|universal credit|benefits?|pay|wages|pension|allowance|hours)\\b"), py_str("\\b(?:got|gotten|been|being|was|were|just|now)\\s+sanctioned\\b(?!\\s+(?:leave|holiday|absence|time off|day off|overtime|pay rise)\\b)")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_NEWS_WITH_A_NAME(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![{ let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_CONDITION_OWNER(), -1, "")); __s.push_str("\\b[^.!?]{0,20}\\b(?:has|have|had|got|is|was|were|been|being)\\b[^.!?]{0,24}\\b(?:dementia|alzheim\\w*|altzheim\\w*|parkinson\\w*|motor neurone|huntington\\w*|multiple sclerosis|muscular dystrophy|leuka?emia|lymphoma|myeloma|mesothelioma|pancreatitis|copd|emphysema|cirrhosis|kidney failure|heart failure|brain (?:damage|injury)|cerebral palsy|terminal)\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_CONDITION_OWNER(), -1, "")); __s.push_str("\\b[^.!?]{0,24}\\b(?:is|are|was|were|has been|have been|had been|got|getting|become|becoming|been)\\b[^.!?]{0,16}\\b(?:ill|poorly|unwell|not well|off (?:it|colour|their food)|in a bad way|taken ill|gone downhill|deteriorat\\w+)\\b"); py_string(__s) }, { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_CONDITION_OWNER(), -1, "")); __s.push_str("\\b[^.!?]{0,24}\\bd(?:iagnos\\w+|x\\b)"); py_string(__s) }, py_str("\\b(?:been|being|just|newly|recently)\\s+(?:diagnos(?:ed|is)\\w*)\\b"), py_str("\\b(?:not allowed|no longer allowed|allowed|banned|barred|stopped|prevented)\\b[^.!?]{0,24}\\b(?:see|seeing|visit|visiting|contact(?: with)?)\\b[^.!?]{0,20}\\b(?:kids?|children|child|son|daughter|grandchild\\w*|baby)\\b"), g_AWARD_LOST(), py_str("\\b(?:have|has|had|having|got|gotten|getting|being (?:forced|made))\\s*(?:to\\s+)?(?:sell|give up|hand back|move out of)\\b[^.!?]{0,24}\\b(?:the|our|my|his|her)\\s+(?:house|home|flat|bungalow|place)\\b"), { let mut __s = String::new(); __s.push_str("(?!.*"); __s.push_str(&fstr(&g_ADOPTION_WORDS(), -1, "")); __s.push_str(")\\b(?:got|gotten|had|received|been sent|sent me|sending me|came through|arrived)\\b[^.!?]{0,28}\\b(?:court (?:date|letter|order|summons|hearing)|summons|notice to quit|eviction notice|possession (?:claim|order)|letter before (?:claim|action))\\b"); py_string(__s) }, py_str("\\bbailiffs?\\b"), { let mut __s = String::new(); __s.push_str("(?!.*"); __s.push_str(&fstr(&g_ADOPTION_WORDS(), -1, "")); __s.push_str(")\\b(?:going|have to go|has to go|got to go|due|off)\\s+(?:in |to |before )?court\\b"); py_string(__s) }, py_str("\\b(?:taking|took|taken|summoned|prosecuting|suing)\\s+(?:me|us|him|her|them)\\s+to court\\b"), py_str("\\b(?:i'?m|im|i am|we'?re|we are|i was|we were)\\s+(?:being\\s+)?(?:taken|summoned|prosecuted|sued)\\s+(?:to|into|before)\\s+court\\b"), py_str("\\b(?:hearing|trial|case)\\s+(?:is |was )?(?:next|this)\\s+(?:week|month|monday|tuesday|wednesday|thursday|friday|morning|afternoon)\\b"), py_str("\\b(?:going|went|moved|transferred|have to go|has to go|got to go|off|sent|sentenced|remanded|due)\\b[^.!?]{0,16}\\b(?:to |in |into )?(?:prison|jail|young offenders)\\b(?!\\s+(?:to\\s+(?:visit|see|meet)|visiting|museum|tour|open\\s+day))"), py_str("\\b(?:is|are|was|were|been|being|stays?|sits?|sitting|kept|held|locked)\\s+(?:up\\s+|inside\\s+|away\\s+)?(?:in|at|inside)\\s+(?:the\\s+)?(?:prison|jail|jailhouse|young offenders|pentonville|wormwood scrubs|brixton|wandsworth)\\b"), py_str("\\bsent down\\b[^.!?]{0,16}\\b(?:for|to)\\s+(?:\\d+|two|three|four|five|six|seven|eight|nine|ten|twelve|eighteen|twenty|a\\s+(?:year|month|stretch|while)|life)\\b"), py_str("\\b(?:was|were|is|are|been|being|got|gets|getting|keeps being)\\s+sent down\\b"), py_str("\\b(?:taken|took|put|puts|removed|removes)\\b[^.!?]{0,20}\\b(?:into care|in care)\\b"), { let mut __s = String::new(); __s.push_str("\\b(?:taken|took|removed)\\b[^.!?]{0,24}\\b(?:my|our|the)\\s+(?:kids?|children|child|son|daughter|baby)\\b[^.!?]{0,16}\\baway\\b"); py_string(__s) }, py_str("\\bsocial services\\b[^.!?]{0,24}\\b(?:took|taken|removed|involved|came)\\b"), py_str("\\b(?:losing|lose|lost|going to lose|about to lose|might lose|will lose|going to be losing)\\b[^.!?]{0,12}\\b(?:the|our|my)\\s+(?:house|home|flat|bungalow)\\b"), py_str("\\b(?:got|get|getting|been|being|was|were)\\s+sacked\\b"), py_str("\\b(?:bad|sad|awful|terrible|dreadful|grim|rotten|horrible|horrid)\\s+(?:news|tidings)\\b")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_BAD_NEWS(op_add(&g_BAD_NEWS_ITSELF(), &g_NEWS_WITH_A_NAME()));
    s_CLOSED_DOOR_LINES(py_tuple(vec![py_str("\\b(?:nobody|no\\s?one|none of them|not one)\\s+(?:will|would|wants? to|is willing to|has|have|ever)\\s+(?:\\w+\\s+){0,3}?(?:hire|employ|interview|take (?:me|him|her|us|them) on|give (?:me|him|her|us|them) a (?:chance|look|shot|start))\\b(?=\\s*(?:$|[,.!?]|(?:me|him|her|us|them|someone|anyone|people|at my age)\\b))"), py_str("\\b(?:nobody|no\\s?one)\\s+(?:(?:gets?|has|have|ever)\\s+(?:got|gotten)?\\s*back to)\\s*(?:me|him|her|us)\\b"), py_str("\\b(?:nobody|no\\s?one)\\s+(?:ever\\s+)?(?:gives?|gave|will give|would give)\\s+(?:me|him|her|us)\\s+(?:a\\s+)?(?:chance|look|shot|start|break)\\b"), py_str("\\b(?:can'?t|cannot|cant|struggling to|never)\\s+(?:get|land|find|even get)\\s+(?:an? )?(?:interview|job|work|reply|response|answer|call ?back)\\b"), py_str("\\btoo old\\b[^.!?]{0,40}\\b(?:job|jobs|work|hire|hired|interview|employ|employer)\\b"), py_str("\\b(?:job|jobs|work|hire|hired|interview|employer)\\b[^.!?]{0,30}\\btoo old\\b"), py_str("\\b(?:turned down|rejected|knocked back)\\b[^.!?]{0,20}\\b(?:again|for (?:the|a) (?:job|role|position|interview))\\b")]));
    s_CLOSED_DOOR_WORK(stdlib_call("re", "compile", &[py_str("\\b(?:job|jobs|work|working|hire|hiring|hired|interview|interviews|application|applications|applied|apply|applying|role|roles|position|positions|employer|employers|employ|employed|recruit\\w*|career|careers|shift|shifts|cv|vacanc\\w*|prospects?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSED_DOOR(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CLOSED_DOOR_LINES()) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSED_DOOR_WORK_OWN(stdlib_call("re", "compile", &[py_str("\\b(?:take|takes|took|taken|taking)\\s+(?:me|him|her|us|them)\\s+on\\b(?=\\s*(?:$|[,.!?]|(?:again|as|at|last|this|for|because|but|and)\\b))")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSED_DOOR_ELSEWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|his|their|my (?:son|daughter|child|kid|husband|wife|partner|boyfriend|girlfriend|mum|mom|mam|dad|father|mother|brother|sister|friend|mate|flatmate|neighbour|neighbor|colleague|carer))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_CLOSED_DOOR_ELSE_OWN(py_tuple(vec![Py::Bool(false), Py::Bool(false), Py::Bool(false), Py::Bool(true), Py::Bool(false), Py::Bool(false), Py::Bool(false)]));
    s_JOKED(stdlib_call("re", "compile", &[py_str("\\b(?:seizures?|choking|strokes?|heart attacks?|collapsed|passed out)\\b[^.!?]{0,14}\\b(?:laughing|giggl\\w*|cracking up|in stitches)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_VOWELS(call_builtin("frozenset", &[py_str("aeiou")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_TYPO(call_builtin("frozenset", &[py_tuple(vec![py_str("strike"), py_str("stroked"), py_str("strokes"), py_str("stroking"), py_str("collapse")])]).unwrap_or_else(|e| exc::raise(e)));
    s_LUMP_NOT_A_LUMP(py_str("(?![\\s-]+(?:sums?|of|sugar)\\b)(?!\\s+in\\s+(?:my|your|his|her|their|our|the)\\s+throat\\b(?![^.!?]*\\b(?:swallow|swallowing|voice|breath|breathing|breathe)\\b))"));
    s_BODY(py_str("(?:head|throat|ear|ears|eye|eyes|tooth|teeth|back|knee|shoulder|arm|leg|legs|foot|feet|hand|hands|stomach|tummy|chest|neck|hip|ankle|wrist|elbow|toe|finger|skin|nose|sinus|jaw|jaws|cheek|cheeks|chin|lips?|tongue|gums?|mouth|face|forehead|temple|temples|scalp|spine|ribs?|shins?|calf|calves|thighs?|groin|kidneys?|bladder|nails?|heels?|palms?|soles?|thumbs?|"));
    s_BODY(op_add(&slice_or_raise(&g_BODY(), &Py::None, &op_neg(&Py::Int(1)), &Py::None), &py_str(")")));
    s_HURTS(py_str("(?:back|knee|knees|hip|hips|shoulder|shoulders|arm|arms|leg|legs|foot|feet|ankle|ankles|wrist|wrists|elbow|elbows|neck|hand|hands|rib|ribs|shin|shins|calf|calves|thigh|thighs|spine|jaw|heel|heels|thumb|thumbs|toe|toes|finger|fingers|hip|knees|joints?)"));
    s_ILL_PART(py_str("(?:chest|throat|stomach|tummy|head|ear|ears|earache|tooth|teeth|toothache|nose|sinus|eye|eyes|gums?|mouth|cheek|cheeks)"));
    s_SICK(op_add(&op_add(&py_str("(?:sore|cough|coughing|headache|migraine|toothache|earache|rash|itchy|itching|infection|infected|cramps?|nauseous|nausea|dizzy|swollen|swelling|aching|aches?|throw up|throws up|threw up|throwing up|vomit\\w*|constipated|bloated|diarrhoea|diarrhea|unwell|poorly|sick|temperature|flu|migraines|lumps?"), &g_LUMP_NOT_A_LUMP()), &py_str("|growths?)")));
    s_IWE(py_str("\\b(?:i|we)(?:'ve|ve|'m|m| have| am| feel| felt| got| keep| woke up with)\\b"));
    s_HURT(py_str("hurts?|hurting|aches?|aching|sore|pain|painful|agony|agonising|agonizing|killing me|is killing me|are killing me|tender|raw|swollen|swelling|puffy|stiff|throbbing|itchy|itches|burning|on fire|hot|bad"));
    s_SYMPTOM(py_tuple(vec![stdlib_call("re", "compile", &[op_add(&op_add(&g_IWE(), &py_str("[^.!?]{0,25}?\\b")), &g_SICK()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&op_add(&op_add(&op_add(&py_str("\\bmy "), &g_BODY()), &py_str("\\b[^.!?]{0,15}?\\b(?:")), &g_HURT()), &py_str(")\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:i|we)\\s+(?:throw\\w* up|vomit\\w*|(?:am|are|'m|'re)?\\s*being sick)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&py_str("\\b(?:my|a|the)\\s+(?:stiff|sore|bad|dodgy|aching|swollen|gammy|sore)\\s+"), &g_BODY()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&g_IWE(), &py_str("[^.!?]{0,30}?\\b(?:the runs|the trots|the squits|bad guts|bad belly|dicky (?:tummy|stomach)|dodgy (?:tummy|stomach)|upset (?:tummy|stomach)|(?:tummy|stomach|sick) bug|food poisoning|being sick|keep(?:t|s)? being sick|can'?t? keep anything down|loose stools|runny (?:tummy|bottom)|running to the (?:loo|toilet))\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\bmy (?:stomach|tummy|belly|guts)\\s+(?:is |feels |has been )?(?:off|queasy|bad|dodgy|playing up)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[op_add(&py_str("\\b(?:i|we)\\s+(?:found|felt|noticed|spotted)\\b[^.!?]{0,20}?\\b(?:lumps?|growths?)\\b"), &g_LUMP_NOT_A_LUMP()), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[py_str("\\b(?:i|we)(?:'ve|ve|'m|m| have| am)?\\s*(?:got|had|been|felt|feel)?\\s*(?:a|an|the)\\s+(?:bad |nasty |chesty |sore )?(?:cold|cough|temperature|sore throat|rash|migraine|headache|infection)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]));
    s_BARE_ACHE(stdlib_call("re", "compile", &[py_str("\\b\\w*aches?\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_ILL(stdlib_call("re", "compile", &[py_str("\\b(?:migraines?|heartburn|indigestion|tonsillitis|sciatica|cystitis|gout|sinusitis)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_BODY(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b"), &g_BODY()), &py_str("\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_HURTS(stdlib_call("re", "compile", &[op_add(&op_add(&py_str("\\b(?:"), &g_HURT()), &py_str("|dodgy|constant|dull)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_FILLER(call_builtin("frozenset", &[mcall(&py_str("\noh ugh urgh god gosh really very so proper right left both top lower upper my me a an the this\nthat and but also still now just actually honestly awful terrible nasty bad little bit of on off\nim i ive had got since yesterday today tonight morning night all day days weeks week now too\nthe worst ever\n"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_BARE_WORD_ONLY(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|have|has|had|got|get|feel|feeling|been|do|does|did|can|could|should|would|will|helps?|stop|stopping|cure|treat|remedy|remedies|take|need|want|make|makes|go|going|come|came|keep|keeps|said|says|tell|think|know|why|what|how|when)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGE_CHILD(py_str("(?:(?:\\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|a|an)[\\s-]+(?:month|months|week|weeks|year|years)[\\s-]+old(?:[\\s-]+(?:baby|boy|girl|son|daughter|child|kid|toddler|little one))?)"));
    s_KIN(op_add(&op_add(&py_str("(?:mum|mom|mother|mam|dad|father|gran|granny|grandma|grandad|grandpa|nana|nan|son|daughter|child|kid|baby|boy|girl|toddler|teenager|little one|wife|husband|partner|boyfriend|girlfriend|fiance|fiancee|brother|sister|uncle|aunt|"), &g_AGE_CHILD()), &py_str(")")));
    s_PETS(py_str("(?:dog|puppy|cat|kitten|horse|pony|rabbit|hamster|guinea pig|gerbil|ferret|budgie|parrot|bird|tortoise|snake|lizard|chicken|hens?|goat|sheep|pig|pet)"));
    s_CARED_SIGN(py_str("pale|pallid|flushed|clammy|listless|lethargic|washed out|off colou?r|out of sorts|peaky|groggy|doughy|waxy|sallow|not (?:himself|herself|themselves)|clingy|whinge?y|fractious|not sleeping|sleep(?:ing (?:a lot|more|all the time)|ier)|off (?:his|her|their) (?:food|bottle|drink)"));
    s_CARED_SICK(op_add(&op_add(&py_str("(?:sore\\s+(?:throat|head|tummy|ear|ears|back|arm|leg|legs|foot|feet|hand|knee|chest|neck|tooth|mouth)|cough|coughing|headache|migraine|toothache|earache|rash|itchy|itching|infection|infected|cramps?|nauseous|nausea|dizzy|swollen|swelling|aching|aches?|throw\\w* up|throw up|vomit\\w*|being sick|diarrhoea|diarrhea|unwell|poorly|sick(?!\\s+(?:note|pay|days?|leave|of|bag|joke|sense))|temperature|fever|flu|bugs?|cold(?!\\s+(?:sore|sores|feet|hands|fingers|toes|air|weather|room|drink|water|days?|morning|night|outside|shoulder|call))|chickenpox|chicken pox|croup|colic|teething|eczema|conjunctivitis|hand foot and mouth|runny nose|blocked nose|chesty|sniffles?|nits|head lice|worms|thrush|migraines|"), &g_CARED_SIGN()), &py_str(")")));
    s_PET_NAMED_CONDITION(py_str("(?:kidney|liver|heart|lung|thyroid)\\s+(?:failure|disease|problems?|trouble|condition)|renal failure|diabet\\w*|epilep\\w*|heart murmur|cushing\\w*|addison\\w*|pancreatitis|blocked bladder|urinary tract"));
    s_AGE_NUMBER(py_str("(?:\\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty)"));
    s_CHILD_BY_AGE(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:my|our|the|his|her|their)\\s+(?:little\\s+|poor\\s+|tiny\\s+|new\\s+)?"); __s.push_str(&fstr(&g_AGE_NUMBER(), -1, "")); __s.push_str("\\s*-?\\s*(?:month|months|mth|mths|week|weeks|wk|wks|year|years|yr|yrs)\\s*-?\\s*old\\s+(?:has|have|had|got|keeps\\s+getting|came\\s+down\\s+with|is|was|been|looks|seems)\\b[^.!?]{0,30}?\\b(?:"); __s.push_str(&fstr(&g_CARED_SICK(), -1, "")); __s.push_str(")\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARER_SYMPTOM(op_add(&op_add(&_someone_elses_symptoms(&[g_KIN()]), &py_tuple(vec![g_CHILD_BY_AGE()])), &{ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_KIN()); __kws.extend(vec![("generic".to_string(), Py::Bool(false)), ("body_only".to_string(), Py::Bool(true)), ("part".to_string(), g_ILL_PART())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _someone_elses_symptoms(args))(&__args) }));
    s_CARER_BODY({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_KIN()); __kws.extend(vec![("body_only".to_string(), Py::Bool(true)), ("part".to_string(), g_HURTS())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _someone_elses_symptoms(args))(&__args) });
    s_PET_SYMPTOM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PETS()); __args.push(op_add(&op_add(&g_CARED_SICK(), &py_str("|")), &g_PET_NAMED_CONDITION())); __kws.extend(vec![("owner".to_string(), py_str("my|our|the")), ("named".to_string(), g_PET_NAMED_CONDITION())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _someone_elses_symptoms(args))(&__args) });
    s_PET_LOW(py_str("(?:depressed|sad|unhappy|miserable|withdrawn|lethargic|listless|moping|mopey|not\\s+(?:him|her|them)sel(?:f|ves)|off\\s+(?:his|her|their)\\s+food|gone\\s+quiet|hiding|not\\s+eating|won'?t\\s+eat)"));
    s_PET_LOW_SYMPTOM({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PETS()); __args.push(g_PET_LOW()); __kws.extend(vec![("owner".to_string(), py_str("my|our|the"))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _someone_elses_symptoms(args))(&__args) });
    s_PET_CONDITION({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(g_PETS()); __args.push(g_PET_NAMED_CONDITION()); __kws.extend(vec![("owner".to_string(), py_str("my|our|the")), ("named".to_string(), g_PET_NAMED_CONDITION()), ("generic".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _someone_elses_symptoms(args))(&__args) });
    s_TROUBLE_OVER(stdlib_call("re", "compile", &[py_str("\\b(?:has|have|had)\\s+gone(?:\\s+now)?\\s*$|\\b(?:has|have|had)\\s+(?:cleared(?:\\s+up)?|eased(?:\\s+off)?|settled(?:\\s+down)?)\\s*$|\\b(?:got|getting)\\s+over\\s+\\w+|\\b(?:is|are|was|were|feels?|feeling)\\s+(?:better|fine|ok|okay|under\\s+control)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUICIDE(stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("|"); __s.push_str(&fstr(&g_OTHERS(), -1, "")); __s.push_str(")\\b[^.!?]{0,26}\\b(?:killed (?:him|her|them)sel(?:f|ves)|hanged (?:him|her|them)sel(?:f|ves)|took (?:his|her|their) own life|committed suicide)\\b(?!\\s+(?:laughing|giggling))"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUDDEN_LOSS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("\\bmiscarriage(?:s)?\\b(?! of justice)|\\bmiscarried\\b"), py_str("\\bstill ?born\\b|\\bstill ?birth\\b"), py_str("\\b(?:lost|losing|we lost) (?:the|our|her|his|their) baby\\b(?!\\s+(?:weight|belly|tummy|fat|pounds?|stones?|kilos?|inches|clothes|things|stuff|room|pram|toys?|seat|carrier|monitor|name|bag|book)\\b)"), { let mut __s = String::new(); __s.push_str("\\b"); __s.push_str(&fstr(&g_LIVING(), -1, "")); __s.push_str("\\b[^.!?]{0,24}\\bkilled (?:him|her|them)sel(?:f|ves)\\b(?!\\s+(?:laughing|giggling))"); py_string(__s) }, py_str("\\b(?:took|taken|taking) (?:his|her|their) own life\\b|\\bcommitted suicide\\b")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_PREGNANCY_BLEEDING(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&py_tuple(vec![py_str("\\bpregnan\\w+\\b[^.!?]{0,24}\\b(?:spotting|bleeding|cramping)\\b"), py_str("\\b(?:spotting|bleeding)\\b[^.!?]{0,24}\\bpregnan\\w+\\b")])) { let mut pattern: Py = __item.clone(); __out.push(stdlib_call("re", "compile", &[pattern.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s__NAMES_ITS_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?:killed|hanged)\\s+(?:him|her|them)sel(?:f|ves)\\b|^took\\s+(?:his|her|their)\\s+own\\s+li(?:fe|ves)\\b|^committed\\s+suicide\\b|^found\\s+dead\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    if py_truthy(&Py::Bool(py_eq(&py_str("bot.compose.badnews"), &py_str("__main__")))) {
    let __iter6 = py_iter_or_raise(&slice_or_raise(&stdlib_attr("sys", "argv").unwrap_or_else(|e| exc::raise(e)), &Py::Int(1), &Py::None, &Py::None));
    for __item in __iter6 {
    s_line(__item.clone());
    s_low(mcall(&g_line(), "lower", &[]));
    let _ = call_builtin("print", &[{ let mut __s = String::new(); __s.push_str(&fstr(&reads_as_bad_news(&[g_low()]), 115, &"5")); __s.push_str(" "); __s.push_str(&fstr(&reads_like_a_symptom(&[g_low()]), 115, &"5")); __s.push_str(" "); __s.push_str(&fstr(&g_line(), -1, "")); py_string(__s) }]).unwrap_or_else(|e| exc::raise(e));
    }
    }
}
pub fn _without_the_noun_leave(args: &[Py]) -> Py {
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
    let _ = py_str("The line with \"his leave\" and \"annual leave\" taken out: time off is not a leaving.");
    return mcall(&py_str(" "), "join", &[mcall(&mcall(&g__LEAVE_THE_NOUN(), "sub", &[py_str(" "), said.clone()]), "split", &[])]);
    Py::None
}
pub fn _a_leaving_at_all(args: &[Py]) -> Py {
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
    let _ = py_str("The leaving in any of its wordings: left me, is leaving, or the marriage ending said plain.");
    said = _without_the_noun_leave(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_LEFT_ME_RE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PARTNER_LEAVING_RE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PARTNER_LEFT_RE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MARRIAGE_ENDED_RE(), "search", &[said.clone()]) } } } } } }))) {
    return call_builtin("bool", &[{ let __b = mcall(&g_REPORTED_LEAVING_RE(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_NOT_THEIRS(), "search", &[said.clone()]))) } }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g_NOT_THEIRS(), "search", &[said.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_PARTNER(), "search", &[said.clone()]))) } }));
    Py::None
}
pub fn reads_as_a_leaving_already_gone(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the leaving has happened, so the pool's question about the house does not fit.");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&_a_leaving_at_all(&[said.clone()])))) {
    return Py::Bool(false);
    }
    if py_truthy(&stdlib_call("re", "search", &[g_STILL_HERE(), said.clone()]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("bool", &[stdlib_call("re", "search", &[g_LEFT_GONE(), said.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stdlib_call("re", "search", &[g_STILL_COMING(), said.clone()]).unwrap_or_else(|e| exc::raise(e)))) } };
    Py::None
}
pub fn reads_as_a_reported_leaving(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the leaving is reported in the third person rather than said of the partner.\n\n    \"she said shes leaving\" is the same news as \"my wife is leaving\" and it takes the same pool,\n    so the reading does not tell them apart. What does is what has an entry behind it: a partner\n    leaving named in the first person reaches the leaving counsel, and the ask that follows -\n    \"what do i do\" - is answered by that counsel. A reported leaving has no entry behind it, so\n    after it the ask is better answered by whoever reads the ask. Kept apart here so that\n    `compose.step_when_low` can stand down for it without losing the carve-out the partner\n    leaving needs.\n    ");
    said = _without_the_noun_leave(&[mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_REPORTED_LEAVING_RE(), "search", &[said.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_NOT_THEIRS(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return Py::Bool(!py_truthy(&{ let __b = mcall(&g_LEFT_ME_RE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_PARTNER_LEAVING_RE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_MARRIAGE_ENDED_RE(), "search", &[said.clone()]) } } } }));
    Py::None
}
pub fn reads_as_the_leaving(args: &[Py]) -> Py {
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
    let mut said: Py = Py::None;
    let _ = py_str("Whether the line is the person being left by the one they live with.\n\n    The same news before it happens as after it: \"my wife wants to leave me\" is not a plan the\n    person is deciding on, it is the leaving landing on them, and the answer it wants is the one\n    for being left. It is the same answer when the partner's leaving is said without the \"me\" -\n    \"my wife is leaving and i dont know what to do\" was answered \"Thanks for telling me - I'm\n    here if you want to dig into any of that\", which says nothing about the leaving at all.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    return _a_leaving_at_all(&[said.clone()]);
    Py::None
}
pub fn reads_as_an_award_lost(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether the line is a benefit, a pension or a wage stopped, refused or cut.\n\n    The one shape of named news that a service can settle, so the caller that has a pointer to that\n    service asks this before it chooses the gentle line.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AWARD_LOST()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_sacking(args: &[Py]) -> Py {
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
    let _ = py_str("Whether the line is somebody losing their job, and nothing else.\n\n    One caller: the stand-down in `compose.step_when_low`. Every other kind of news has an entry\n    or a card behind it, so the ask that follows it is better answered there - \"bailiffs turned\n    up\" then \"what do i do\" reaches what to do about the bailiffs. A sacking has no such entry,\n    and the ask then landed on \"There is nothing in this to look up. Tell me what has to happen\n    next\", a shrug at somebody who has just lost their job. The step pool was the answer to that\n    ask before the news reading existed, and it is a better one than the shrug.\n    ");
    return call_builtin("bool", &[stdlib_call("re", "search", &[g_SACKED_FIRED(), clean.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_news_with_a_name(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether the line is the news that has a name in it, rather than the medical list.\n\n    One caller: the gentle pool for a dad with parkinsons or a roof that has to go drops the line\n    that calls it frustrating (`fallback.NAMED_NEGATIVE`), because the smallest of the four words is\n    the wrong register for a person. Everything else that asks \"is this news\" asks\n    `reads_as_bad_news`, which reads both lists.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_NEWS_WITH_A_NAME()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_heavy_news(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a line is news that a bare ask after it would be asking about.\n\n    The medical list only, on purpose: the news in `NEWS_WITH_A_NAME` - a refused payment, a court\n    letter, a house on the market - is news a person can still ask a plain question after, and the\n    reader that stands the ask pool down (`_news_above`) asks this rather than `reads_as_bad_news`.\n    ");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BAD_NEWS_ITSELF()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_closed_door(args: &[Py]) -> Py {
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
    let mut else_owns_it: Py = Py::None;
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is the person being turned away from work.");
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = mcall(&g_CLOSED_DOOR_WORK(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CLOSED_DOOR_WORK_OWN(), "search", &[clean.clone()]) } }))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("zip", &[g_CLOSED_DOOR(), g_CLOSED_DOOR_ELSE_OWN()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    pattern = __unpk2[0].clone();
    else_owns_it = __unpk2[1].clone();
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&pattern.clone(), "search", &[clean.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = else_owns_it.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_CLOSED_DOOR_ELSEWHERE(), "search", &[clean.clone()]) } }) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _nearly(args: &[Py]) -> Py {
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
    let mut target: Py = Py::None;
    let _ = py_str("Whether `word` is an emergency word with one letter typed wrong.");
    if py_truthy(&{ let __b = op_lt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_NOT_A_TYPO(), &word.clone())) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&g_NEARLY());
    for __item in __iter1 {
    target = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&word.clone(), &target.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("abs", &[op_sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[target.clone()]).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&pyrt::xrefs::xget("understanding.normalize", "LEXICON"), &word.clone()))) {
    continue;
    }
    if py_truthy(&_one_edit(&[word.clone(), target.clone()])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_a_bare_ailment(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the whole line is just the name of something that hurts.\n\n    \"toothache\", \"sore throat\", \"bad back\", \"stomach ache\", \"stiff neck\", \"migraine\", and the\n    same with a filler or two around it (\"ugh toothache\", \"really sore throat\"). The line has to\n    name an ache or an illness by itself, or a body part together with the word that says it\n    hurts: \"my back\" alone is not a complaint, and \"sore throat remedies\" is a question about a\n    remedy, which its words this reading does not allow.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[clean.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&clean.clone(), "strip", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&clean.clone(), &py_str("?"))) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_BARE_WORD_ONLY(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&clean.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { op_gt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)) } }) {
    return Py::Bool(false);
    }
    named = call_builtin("bool", &[{ let __b = mcall(&g_BARE_ACHE(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BARE_ILL(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&named.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_BARE_BODY(), "search", &[clean.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_BARE_HURTS(), "search", &[clean.clone()]) } })) } }) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&g_BARE_FILLER(), &word.clone()))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_BARE_ACHE(), "fullmatch", &[word.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BARE_ILL(), "fullmatch", &[word.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_BARE_BODY(), "fullmatch", &[word.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_BARE_HURTS(), "fullmatch", &[word.clone()]) } } } } } }) {
    continue;
    }
    return Py::Bool(false);
    }
    return Py::Bool(true);
    Py::None
}
pub fn _someone_elses_symptoms(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut who: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "who") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: who"))) };
    let mut sick: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sick") { v.clone() }
    else { g_CARED_SICK() };
    let mut owner: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "owner") { v.clone() }
    else { py_str("my|our") };
    let mut named: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "named") { v.clone() }
    else { Py::None };
    let mut generic: Py = if __pos.len() > 4 { __pos[4].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "generic") { v.clone() }
    else { Py::Bool(true) };
    let mut body_only: Py = if __pos.len() > 5 { __pos[5].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body_only") { v.clone() }
    else { Py::Bool(false) };
    let mut part: Py = if __pos.len() > 6 { __pos[6].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { Py::None };
    let mut shapes: Py = Py::None;
    let _ = py_str("The shapes a line about somebody else being unwell takes, for the names in `who`.\n\n    A person and an animal are described the same way (\"my son has a cough\", \"my dog has a\n    cough\"), so the shapes are written once and the names passed in. What differs is who gets the\n    reply, which is the caller's business; an animal can also have a named condition that a\n    person's line would not be read for, so the named/possessive shape is only added when `named`\n    is given. Without that fence \"my son's eczema has cleared up\" was read as the illness going\n    on, and the pet shape asked about a rash for diabetes.\n    ");
    shapes = py_list(vec![]);
    if py_truthy(&{ let __b = generic.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&body_only.clone())) } }) {
    let _ = mcall(&shapes.clone(), "extend", &[py_list(vec![stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:"); __s.push_str(&fstr(&owner.clone(), -1, "")); __s.push_str(")\\s+(?:poor\\s+|little\\s+|eldest\\s+|youngest\\s+|old\\s+)?"); __s.push_str(&fstr(&who.clone(), -1, "")); __s.push_str("(?:'s|s)?\\s+(?:has|have|had|got|has got|keeps getting|keeps|kept|keep|came down with|come down with|is|was|been|looks|seems)\\b[^.!?]{0,30}?\\b(?:"); __s.push_str(&fstr(&sick.clone(), -1, "")); __s.push_str(")\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:"); __s.push_str(&fstr(&owner.clone(), -1, "")); __s.push_str(")\\s+"); __s.push_str(&fstr(&who.clone(), -1, "")); __s.push_str("\\s+(?:"); __s.push_str(&fstr(&sick.clone(), -1, "")); __s.push_str(")\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])]);
    }
    if py_truthy(&generic.clone()) {
    let _ = mcall(&shapes.clone(), "extend", &[py_list(vec![stdlib_call("re", "compile", &[op_add(&op_add(&{ let mut __s = String::new(); __s.push_str("\\b(?:my|our)\\s+"); __s.push_str(&fstr(&who.clone(), -1, "")); __s.push_str("(?:'s|s)\\s+"); __s.push_str(&fstr(&g_BODY(), -1, "")); __s.push_str("\\b[^.!?]{0,18}?\\b(?:"); py_string(__s) }, &g_HURT()), &py_str("|infected|playing up)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:"); __s.push_str(&fstr(&owner.clone(), -1, "")); __s.push_str(")\\s+"); __s.push_str(&fstr(&who.clone(), -1, "")); __s.push_str("(?:'s|s)?\\s+(?:has|have|had|got)(?:\\s+got)?\\s+(?:a|an)?\\s*(?:bad|sore|dodgy|gammy|weak|dicky|knackered|broken|cracked|twisted|swollen|stiff|aching)\\s+"); __s.push_str(&fstr(&{ let __b = part.clone(); if py_truthy(&__b) { __b } else { g_BODY() } }, -1, "")); __s.push_str("\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))])]);
    }
    if py_truthy(&named.clone()) {
    let _ = mcall(&shapes.clone(), "append", &[stdlib_call("re", "compile", &[{ let mut __s = String::new(); __s.push_str("\\b(?:"); __s.push_str(&fstr(&owner.clone(), -1, "")); __s.push_str(")\\s+(?:poor\\s+|little\\s+|eldest\\s+|youngest\\s+|old\\s+)?"); __s.push_str(&fstr(&who.clone(), -1, "")); __s.push_str("(?:'s|s)?\\b[^.!?]{0,24}?\\b(?:"); __s.push_str(&fstr(&named.clone(), -1, "")); __s.push_str(")\\b"); py_string(__s) }, stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e))]);
    }
    return call_builtin("tuple", &[shapes.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_for_a_carer(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is about someone the person looks after being ill.");
    if py_truthy(&mcall(&g_TROUBLE_OVER(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CARER_SYMPTOM()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_a_carer_body(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut whose_body: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is somebody else's body, rather than their illness.");
    if py_truthy(&mcall(&g_TROUBLE_OVER(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_CARER_BODY()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_is(&memory.clone(), &Py::None))) {
    whose_body = pyrt::xrefs::xmodule("memory.whose_body");
    return mcall(&whose_body.clone(), "somebody_elses_body", &[clean.clone(), memory.clone()]);
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_for_a_pet(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is about an animal of theirs being ill or hurt.");
    if py_truthy(&mcall(&g_TROUBLE_OVER(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PET_LOW_SYMPTOM()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PET_SYMPTOM()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_pet_low(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is about an animal of theirs that seems low.");
    if py_truthy(&mcall(&g_TROUBLE_OVER(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PET_LOW_SYMPTOM()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_a_pet_condition(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a pet line names a condition rather than a symptom.");
    if py_truthy(&mcall(&g_TROUBLE_OVER(), "search", &[clean.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PET_CONDITION()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_like_a_symptom(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is the person's own body complaining.");
    clean = pyrt::xrefs::xcall("understanding.normalize", "without_the_figure_of_speech", &[clean.clone()]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SYMPTOM()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    return reads_as_a_bare_ailment(&[clean.clone()]);
    Py::None
}
pub fn _one_edit(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut left: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "left") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: left"))) };
    let mut right: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "right") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: right"))) };
    let mut a: Py = Py::None;
    let mut b: Py = Py::None;
    let mut different: Py = Py::None;
    let mut index: Py = Py::None;
    let mut long: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("Whether `left` is `right` with one letter typed wrong, the way a hurried typist does it.\n\n    A letter dropped (\"sezure\"), a letter added (\"chocking\") or a vowel written for the vowel\n    it sounded like (\"hospitel\") is a typo, and the same letter typed as a different consonant\n    is not worth that trust: \"cooking\" is one letter from \"choking\" and \"cancel\" is one from\n    \"cancer\", and reading those two as emergencies answers a curry and a gym subscription with\n    sympathy. Every everyday word that a consonant apart would give is dropped by that rule,\n    and the emergency words whose everyday neighbour is a vowel apart (\"strike\", \"stroke\") is\n    named in NOT_A_TYPO above.\n    ");
    if py_truthy(&Py::Bool(py_eq(&left.clone(), &right.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[right.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    different = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("zip", &[left.clone(), right.clone()]).unwrap_or_else(|e| exc::raise(e))) { let __unpk9681 = py_unpack(&__item.clone(), 2, None); let mut a: Py = __unpk9681[0].clone(); let mut b: Py = __unpk9681[1].clone(); if py_truthy(&Py::Bool(!py_eq(&a.clone(), &b.clone()))) {  __out.push(py_tuple(vec![a.clone(), b.clone()])); } } py_list(__out) })() };
    return { let __b = Py::Bool(py_eq(&call_builtin("len", &[different.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_VOWELS(), &geti(&geti(&different.clone(), &Py::Int(0)), &Py::Int(0)))); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_VOWELS(), &geti(&geti(&different.clone(), &Py::Int(0)), &Py::Int(1)))) } } } };
    }
    let __unpk1 = py_unpack(&if py_truthy(&op_lt(&call_builtin("len", &[left.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[right.clone()]).unwrap_or_else(|e| exc::raise(e)))) { py_tuple(vec![left.clone(), right.clone()]) } else { py_tuple(vec![right.clone(), left.clone()]) }, 2, None);
    short = __unpk1[0].clone();
    long = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("range", &[call_builtin("len", &[long.clone()]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    index = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&op_add(&slice_or_raise(&long.clone(), &Py::None, &index.clone(), &Py::None), &slice_or_raise(&long.clone(), &op_add(&index.clone(), &Py::Int(1)), &Py::None, &Py::None)), &short.clone()))) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_sudden_loss(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("Whether a line is a pregnancy that has ended or someone who took their own life.");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&g__dream(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = pyrt::xrefs::xcall("memory.dream", "a_dream_is_not_happening", &[text.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&pyrt::xrefs::xcall("memory.dream", "a_death_came_first", &[text.clone()]))) } } } }) {
    return Py::Bool(false);
    }
    return { let __b = call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_SUDDEN_LOSS()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[text.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { { let __b = call_builtin("bool", &[mcall(&g_SUICIDE(), "search", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { reads_as_someones_death(&[text.clone()]) } } } };
    Py::None
}
pub fn reads_as_someones_death(args: &[Py]) -> Py {
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
    let mut text: Py = Py::None;
    let _ = py_str("Whether the line reports that a person or an animal has died.\n\n    A death is not an achievement and not a fault to be talked through: \"a boy in my class\n    killed himself\" was congratulated (\"a boy\" is how a birth is announced) and \"someone in my\n    class died\" was asked what it was doing. Both readers ask this one first.\n    ");
    text = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("died"), py_str("dies"), py_str("she died"), py_str("he died")]), &text.clone()))) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_DEATH_SAID(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    found = __item.clone();
    if py_truthy(&mcall(&g__NEARLY(), "search", &[slice_or_raise(&text.clone(), &Py::None, &mcall(&found.clone(), "end", &[]), &Py::None)])) {
    continue;
    }
    if py_truthy(&mcall(&g__AFTER_IT(), "match", &[slice_or_raise(&text.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None)])) {
    continue;
    }
    if py_truthy(&_subject_before(&[text.clone(), mcall(&found.clone(), "start", &[]), mcall(&found.clone(), "group", &[Py::Int(0)])])) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _subject_before(args: &[Py]) -> Py {
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
    let mut start: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "start") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: start"))) };
    let mut said: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { py_str("") };
    let mut clause: Py = Py::None;
    let mut cut: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether the word the death belongs to is a person or an animal.\n\n    The nearest word before the death decides, stepping over the words that carry a clause to a\n    verb and over a place or a group. So \"my phone died\" is the phone, \"he said his phone died\" is\n    still the phone, \"my mate said the wifi is dead\" is the wifi, and \"someone in my class died\"\n    is someone.\n\n    A line with no person in it at all is not a death, however the word is used: \"dead broke\",\n    \"dead cert\", \"dead in the water\", \"dead end job\", \"dead as a doornail\" and \"my leg went dead\"\n    are ordinary lines, and a list of the words that can follow \"dead\" never ends. Two shapes\n    need no person in front: a reflexive death (\"killed himself\", \"took her own life\") and the\n    way a death is reported before anyone is named (\"found dead in his flat\").\n    ");
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { mcall(&g__NAMES_ITS_SUBJECT(), "match", &[said.clone()]) } }) {
    return Py::Bool(true);
    }
    clause = slice_or_raise(&text.clone(), &Py::None, &start.clone(), &Py::None);
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("."), py_str("!"), py_str("?")]));
    for __item in __iter1 {
    mark = __item.clone();
    cut = mcall(&clause.clone(), "rfind", &[mark.clone()]);
    if py_truthy(&Py::Bool(!py_eq(&cut.clone(), &op_neg(&Py::Int(1))))) {
    clause = slice_or_raise(&clause.clone(), &op_add(&cut.clone(), &Py::Int(1)), &Py::None, &Py::None);
    }
    }
    words = mcall(&g__WORD(), "findall", &[clause.clone()]);
    let __iter2 = py_iter_or_raise(&call_builtin("reversed", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    word = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g__CARRIES(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g__PLACE(), &word.clone())) } }) {
    continue;
    }
    return Py::Bool(py_contains(&g__PERSON(), &word.clone()));
    }
    return Py::Bool(false);
    Py::None
}
pub fn reads_as_pregnancy_bleeding(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether a line is bleeding or spotting in pregnancy: a midwife today, not a chat.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_PREGNANCY_BLEEDING()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn reads_as_the_speakers_own_plan(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let _ = py_str("Whether the line is a plan the speaker is making, rather than news that happened to them.");
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_OWN_PLAN()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[clean.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_result_came_back_well(args: &[Py]) -> Py {
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
    let mut reads_as_a_clear_result: Py = Py::None;
    let _ = py_str("Whether the line is a clear result, which is news of the other kind (`goodnews`).\n\n    The illness word is what makes a clear result look like bad news: \"my mums cancer is in\n    remission\" was read on the word \"cancer\" and met with \"Sorry to hear that. What would help\n    most right now?\", and \"the biopsy was benign\" with \"I'm sorry - that's frustrating\". The\n    reading lives in `bot/compose/goodnews.py` with the rest of the good news; the import is here\n    rather than at the top because goodnews imports this module (`reads_as_someones_death`).\n    ");
    reads_as_a_clear_result = py_func("reads_as_a_clear_result", |args: &[Py]| pyrt::xrefs::xcall("compose.goodnews", "reads_as_a_clear_result", args));
    return py_call(&reads_as_a_clear_result, &[clean.clone()]);
    Py::None
}
pub fn reads_as_bad_news(args: &[Py]) -> Py {
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
    let mut pattern: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("Whether a cleaned, lowercased line is news a friend would answer gently.");
    if py_truthy(&_the_result_came_back_well(&[clean.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_BAD_NEWS()) { let mut pattern: Py = __item.clone(); __out.push(mcall(&pattern.clone(), "search", &[mcall(&g_JOKED(), "sub", &[py_str(" "), clean.clone()])])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), clean.clone()]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut word: Py = __item.clone(); __out.push(_nearly(&[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.badnews".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ADOPTION_WORDS" => Some(g_ADOPTION_WORDS()),
            "AGE_CHILD" => Some(g_AGE_CHILD()),
            "AGE_NUMBER" => Some(g_AGE_NUMBER()),
            "AWARD_LOST" => Some(g_AWARD_LOST()),
            "BAD_NEWS" => Some(g_BAD_NEWS()),
            "BAD_NEWS_ITSELF" => Some(g_BAD_NEWS_ITSELF()),
            "BARE_ACHE" => Some(g_BARE_ACHE()),
            "BARE_BODY" => Some(g_BARE_BODY()),
            "BARE_FILLER" => Some(g_BARE_FILLER()),
            "BARE_HURTS" => Some(g_BARE_HURTS()),
            "BARE_ILL" => Some(g_BARE_ILL()),
            "BARE_WORD_ONLY" => Some(g_BARE_WORD_ONLY()),
            "BODY" => Some(g_BODY()),
            "CARED_SICK" => Some(g_CARED_SICK()),
            "CARED_SIGN" => Some(g_CARED_SIGN()),
            "CARER_BODY" => Some(g_CARER_BODY()),
            "CARER_SYMPTOM" => Some(g_CARER_SYMPTOM()),
            "CHILD_BY_AGE" => Some(g_CHILD_BY_AGE()),
            "CLOSED_DOOR" => Some(g_CLOSED_DOOR()),
            "CLOSED_DOOR_ELSEWHERE" => Some(g_CLOSED_DOOR_ELSEWHERE()),
            "CLOSED_DOOR_ELSE_OWN" => Some(g_CLOSED_DOOR_ELSE_OWN()),
            "CLOSED_DOOR_LINES" => Some(g_CLOSED_DOOR_LINES()),
            "CLOSED_DOOR_WORK" => Some(g_CLOSED_DOOR_WORK()),
            "CLOSED_DOOR_WORK_OWN" => Some(g_CLOSED_DOOR_WORK_OWN()),
            "CONDITION_OWNER" => Some(g_CONDITION_OWNER()),
            "DEATH_SAID" => Some(g_DEATH_SAID()),
            "HURT" => Some(g_HURT()),
            "HURTS" => Some(g_HURTS()),
            "ILL_PART" => Some(g_ILL_PART()),
            "IWE" => Some(g_IWE()),
            "JOKED" => Some(g_JOKED()),
            "KIN" => Some(g_KIN()),
            "LEFT_GONE" => Some(g_LEFT_GONE()),
            "LEFT_ME" => Some(g_LEFT_ME()),
            "LEFT_ME_RE" => Some(g_LEFT_ME_RE()),
            "LEFT_PLAN" => Some(g_LEFT_PLAN()),
            "LEXICON" => Some(g_LEXICON()),
            "LIVING" => Some(g_LIVING()),
            "LOST_CHANCE" => Some(g_LOST_CHANCE()),
            "LOST_PATTERN" => Some(g_LOST_PATTERN()),
            "LUMP_NOT_A_LUMP" => Some(g_LUMP_NOT_A_LUMP()),
            "MARRIAGE_ENDED" => Some(g_MARRIAGE_ENDED()),
            "MARRIAGE_ENDED_RE" => Some(g_MARRIAGE_ENDED_RE()),
            "NEARLY" => Some(g_NEARLY()),
            "NEWS_WITH_A_NAME" => Some(g_NEWS_WITH_A_NAME()),
            "NOT_A_TYPO" => Some(g_NOT_A_TYPO()),
            "NOT_THEIRS" => Some(g_NOT_THEIRS()),
            "OTHERS" => Some(g_OTHERS()),
            "OWN_PLAN" => Some(g_OWN_PLAN()),
            "PARTNER" => Some(g_PARTNER()),
            "PARTNER_LEAVING" => Some(g_PARTNER_LEAVING()),
            "PARTNER_LEAVING_RE" => Some(g_PARTNER_LEAVING_RE()),
            "PARTNER_LEFT" => Some(g_PARTNER_LEFT()),
            "PARTNER_LEFT_RE" => Some(g_PARTNER_LEFT_RE()),
            "PETS" => Some(g_PETS()),
            "PET_CONDITION" => Some(g_PET_CONDITION()),
            "PET_LOW" => Some(g_PET_LOW()),
            "PET_LOW_SYMPTOM" => Some(g_PET_LOW_SYMPTOM()),
            "PET_NAMED_CONDITION" => Some(g_PET_NAMED_CONDITION()),
            "PET_SYMPTOM" => Some(g_PET_SYMPTOM()),
            "PREGNANCY_BLEEDING" => Some(g_PREGNANCY_BLEEDING()),
            "REPORTED_LEAVING" => Some(g_REPORTED_LEAVING()),
            "REPORTED_LEAVING_RE" => Some(g_REPORTED_LEAVING_RE()),
            "SACKED_FIRED" => Some(g_SACKED_FIRED()),
            "SICK" => Some(g_SICK()),
            "STILL_COMING" => Some(g_STILL_COMING()),
            "STILL_HERE" => Some(g_STILL_HERE()),
            "SUDDEN_LOSS" => Some(g_SUDDEN_LOSS()),
            "SUICIDE" => Some(g_SUICIDE()),
            "SYMPTOM" => Some(g_SYMPTOM()),
            "TROUBLE_OVER" => Some(g_TROUBLE_OVER()),
            "VOWELS" => Some(g_VOWELS()),
            "_AFTER_IT" => Some(g__AFTER_IT()),
            "_CARRIES" => Some(g__CARRIES()),
            "_DEATH_WORDS" => Some(g__DEATH_WORDS()),
            "_HOW_IT_IS_SAID" => Some(g__HOW_IT_IS_SAID()),
            "_LEAVE_THE_NOUN" => Some(g__LEAVE_THE_NOUN()),
            "_NAMES_ITS_SUBJECT" => Some(g__NAMES_ITS_SUBJECT()),
            "_NEARLY" => Some(g__NEARLY()),
            "_PERSON" => Some(g__PERSON()),
            "_PLACE" => Some(g__PLACE()),
            "_WORD" => Some(g__WORD()),
            "_a_leaving_at_all" => Some(py_func("_a_leaving_at_all", |args: &[Py]| _a_leaving_at_all(args))),
            "_dream" => Some(g__dream()),
            "_nearly" => Some(py_func("_nearly", |args: &[Py]| _nearly(args))),
            "_one_edit" => Some(py_func("_one_edit", |args: &[Py]| _one_edit(args))),
            "_someone_elses_symptoms" => Some(py_func("_someone_elses_symptoms", |args: &[Py]| _someone_elses_symptoms(args))),
            "_subject_before" => Some(py_func("_subject_before", |args: &[Py]| _subject_before(args))),
            "_the_result_came_back_well" => Some(py_func("_the_result_came_back_well", |args: &[Py]| _the_result_came_back_well(args))),
            "_without_the_noun_leave" => Some(py_func("_without_the_noun_leave", |args: &[Py]| _without_the_noun_leave(args))),
            "line" => Some(g_line()),
            "low" => Some(g_low()),
            "reads_as_a_bare_ailment" => Some(py_func("reads_as_a_bare_ailment", |args: &[Py]| reads_as_a_bare_ailment(args))),
            "reads_as_a_carer_body" => Some(py_func("reads_as_a_carer_body", |args: &[Py]| reads_as_a_carer_body(args))),
            "reads_as_a_closed_door" => Some(py_func("reads_as_a_closed_door", |args: &[Py]| reads_as_a_closed_door(args))),
            "reads_as_a_leaving_already_gone" => Some(py_func("reads_as_a_leaving_already_gone", |args: &[Py]| reads_as_a_leaving_already_gone(args))),
            "reads_as_a_pet_condition" => Some(py_func("reads_as_a_pet_condition", |args: &[Py]| reads_as_a_pet_condition(args))),
            "reads_as_a_pet_low" => Some(py_func("reads_as_a_pet_low", |args: &[Py]| reads_as_a_pet_low(args))),
            "reads_as_a_reported_leaving" => Some(py_func("reads_as_a_reported_leaving", |args: &[Py]| reads_as_a_reported_leaving(args))),
            "reads_as_a_sacking" => Some(py_func("reads_as_a_sacking", |args: &[Py]| reads_as_a_sacking(args))),
            "reads_as_an_award_lost" => Some(py_func("reads_as_an_award_lost", |args: &[Py]| reads_as_an_award_lost(args))),
            "reads_as_bad_news" => Some(py_func("reads_as_bad_news", |args: &[Py]| reads_as_bad_news(args))),
            "reads_as_heavy_news" => Some(py_func("reads_as_heavy_news", |args: &[Py]| reads_as_heavy_news(args))),
            "reads_as_news_with_a_name" => Some(py_func("reads_as_news_with_a_name", |args: &[Py]| reads_as_news_with_a_name(args))),
            "reads_as_pregnancy_bleeding" => Some(py_func("reads_as_pregnancy_bleeding", |args: &[Py]| reads_as_pregnancy_bleeding(args))),
            "reads_as_someones_death" => Some(py_func("reads_as_someones_death", |args: &[Py]| reads_as_someones_death(args))),
            "reads_as_sudden_loss" => Some(py_func("reads_as_sudden_loss", |args: &[Py]| reads_as_sudden_loss(args))),
            "reads_as_the_leaving" => Some(py_func("reads_as_the_leaving", |args: &[Py]| reads_as_the_leaving(args))),
            "reads_as_the_speakers_own_plan" => Some(py_func("reads_as_the_speakers_own_plan", |args: &[Py]| reads_as_the_speakers_own_plan(args))),
            "reads_for_a_carer" => Some(py_func("reads_for_a_carer", |args: &[Py]| reads_for_a_carer(args))),
            "reads_for_a_pet" => Some(py_func("reads_for_a_pet", |args: &[Py]| reads_for_a_pet(args))),
            "reads_like_a_symptom" => Some(py_func("reads_like_a_symptom", |args: &[Py]| reads_like_a_symptom(args))),
            "without_the_figure_of_speech" => Some(g_without_the_figure_of_speech()),
            "re" => Some(stdlib_module("re")),
            "sys" => Some(stdlib_module("sys")),
            _ => None,
        }),
        set: Some(Rc::new(|attr: &str, v: Py| match attr {
            "ADOPTION_WORDS" => s_ADOPTION_WORDS(v),
            "AGE_CHILD" => s_AGE_CHILD(v),
            "AGE_NUMBER" => s_AGE_NUMBER(v),
            "AWARD_LOST" => s_AWARD_LOST(v),
            "BAD_NEWS" => s_BAD_NEWS(v),
            "BAD_NEWS_ITSELF" => s_BAD_NEWS_ITSELF(v),
            "BARE_ACHE" => s_BARE_ACHE(v),
            "BARE_BODY" => s_BARE_BODY(v),
            "BARE_FILLER" => s_BARE_FILLER(v),
            "BARE_HURTS" => s_BARE_HURTS(v),
            "BARE_ILL" => s_BARE_ILL(v),
            "BARE_WORD_ONLY" => s_BARE_WORD_ONLY(v),
            "BODY" => s_BODY(v),
            "CARED_SICK" => s_CARED_SICK(v),
            "CARED_SIGN" => s_CARED_SIGN(v),
            "CARER_BODY" => s_CARER_BODY(v),
            "CARER_SYMPTOM" => s_CARER_SYMPTOM(v),
            "CHILD_BY_AGE" => s_CHILD_BY_AGE(v),
            "CLOSED_DOOR" => s_CLOSED_DOOR(v),
            "CLOSED_DOOR_ELSEWHERE" => s_CLOSED_DOOR_ELSEWHERE(v),
            "CLOSED_DOOR_ELSE_OWN" => s_CLOSED_DOOR_ELSE_OWN(v),
            "CLOSED_DOOR_LINES" => s_CLOSED_DOOR_LINES(v),
            "CLOSED_DOOR_WORK" => s_CLOSED_DOOR_WORK(v),
            "CLOSED_DOOR_WORK_OWN" => s_CLOSED_DOOR_WORK_OWN(v),
            "CONDITION_OWNER" => s_CONDITION_OWNER(v),
            "DEATH_SAID" => s_DEATH_SAID(v),
            "HURT" => s_HURT(v),
            "HURTS" => s_HURTS(v),
            "ILL_PART" => s_ILL_PART(v),
            "IWE" => s_IWE(v),
            "JOKED" => s_JOKED(v),
            "KIN" => s_KIN(v),
            "LEFT_GONE" => s_LEFT_GONE(v),
            "LEFT_ME" => s_LEFT_ME(v),
            "LEFT_ME_RE" => s_LEFT_ME_RE(v),
            "LEFT_PLAN" => s_LEFT_PLAN(v),
            "LEXICON" => s_LEXICON(v),
            "LIVING" => s_LIVING(v),
            "LOST_CHANCE" => s_LOST_CHANCE(v),
            "LOST_PATTERN" => s_LOST_PATTERN(v),
            "LUMP_NOT_A_LUMP" => s_LUMP_NOT_A_LUMP(v),
            "MARRIAGE_ENDED" => s_MARRIAGE_ENDED(v),
            "MARRIAGE_ENDED_RE" => s_MARRIAGE_ENDED_RE(v),
            "NEARLY" => s_NEARLY(v),
            "NEWS_WITH_A_NAME" => s_NEWS_WITH_A_NAME(v),
            "NOT_A_TYPO" => s_NOT_A_TYPO(v),
            "NOT_THEIRS" => s_NOT_THEIRS(v),
            "OTHERS" => s_OTHERS(v),
            "OWN_PLAN" => s_OWN_PLAN(v),
            "PARTNER" => s_PARTNER(v),
            "PARTNER_LEAVING" => s_PARTNER_LEAVING(v),
            "PARTNER_LEAVING_RE" => s_PARTNER_LEAVING_RE(v),
            "PARTNER_LEFT" => s_PARTNER_LEFT(v),
            "PARTNER_LEFT_RE" => s_PARTNER_LEFT_RE(v),
            "PETS" => s_PETS(v),
            "PET_CONDITION" => s_PET_CONDITION(v),
            "PET_LOW" => s_PET_LOW(v),
            "PET_LOW_SYMPTOM" => s_PET_LOW_SYMPTOM(v),
            "PET_NAMED_CONDITION" => s_PET_NAMED_CONDITION(v),
            "PET_SYMPTOM" => s_PET_SYMPTOM(v),
            "PREGNANCY_BLEEDING" => s_PREGNANCY_BLEEDING(v),
            "REPORTED_LEAVING" => s_REPORTED_LEAVING(v),
            "REPORTED_LEAVING_RE" => s_REPORTED_LEAVING_RE(v),
            "SACKED_FIRED" => s_SACKED_FIRED(v),
            "SICK" => s_SICK(v),
            "STILL_COMING" => s_STILL_COMING(v),
            "STILL_HERE" => s_STILL_HERE(v),
            "SUDDEN_LOSS" => s_SUDDEN_LOSS(v),
            "SUICIDE" => s_SUICIDE(v),
            "SYMPTOM" => s_SYMPTOM(v),
            "TROUBLE_OVER" => s_TROUBLE_OVER(v),
            "VOWELS" => s_VOWELS(v),
            "_AFTER_IT" => s__AFTER_IT(v),
            "_CARRIES" => s__CARRIES(v),
            "_DEATH_WORDS" => s__DEATH_WORDS(v),
            "_HOW_IT_IS_SAID" => s__HOW_IT_IS_SAID(v),
            "_LEAVE_THE_NOUN" => s__LEAVE_THE_NOUN(v),
            "_NAMES_ITS_SUBJECT" => s__NAMES_ITS_SUBJECT(v),
            "_NEARLY" => s__NEARLY(v),
            "_PERSON" => s__PERSON(v),
            "_PLACE" => s__PLACE(v),
            "_WORD" => s__WORD(v),
            "_dream" => s__dream(v),
            "line" => s_line(v),
            "low" => s_low(v),
            "without_the_figure_of_speech" => s_without_the_figure_of_speech(v),
            _ => {},
        })),
    }))
}
pub fn register() {
    pyrt::xrefs::reg_module("compose.badnews", module_obj);
    pyrt::xrefs::reg_fn("compose.badnews", "_without_the_noun_leave", py_func("_without_the_noun_leave", |args: &[Py]| _without_the_noun_leave(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "_a_leaving_at_all", py_func("_a_leaving_at_all", |args: &[Py]| _a_leaving_at_all(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_leaving_already_gone", py_func("reads_as_a_leaving_already_gone", |args: &[Py]| reads_as_a_leaving_already_gone(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_reported_leaving", py_func("reads_as_a_reported_leaving", |args: &[Py]| reads_as_a_reported_leaving(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_the_leaving", py_func("reads_as_the_leaving", |args: &[Py]| reads_as_the_leaving(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_an_award_lost", py_func("reads_as_an_award_lost", |args: &[Py]| reads_as_an_award_lost(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_sacking", py_func("reads_as_a_sacking", |args: &[Py]| reads_as_a_sacking(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_news_with_a_name", py_func("reads_as_news_with_a_name", |args: &[Py]| reads_as_news_with_a_name(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_heavy_news", py_func("reads_as_heavy_news", |args: &[Py]| reads_as_heavy_news(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_closed_door", py_func("reads_as_a_closed_door", |args: &[Py]| reads_as_a_closed_door(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "_nearly", py_func("_nearly", |args: &[Py]| _nearly(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_bare_ailment", py_func("reads_as_a_bare_ailment", |args: &[Py]| reads_as_a_bare_ailment(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "_someone_elses_symptoms", py_func("_someone_elses_symptoms", |args: &[Py]| _someone_elses_symptoms(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_for_a_carer", py_func("reads_for_a_carer", |args: &[Py]| reads_for_a_carer(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_carer_body", py_func("reads_as_a_carer_body", |args: &[Py]| reads_as_a_carer_body(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_for_a_pet", py_func("reads_for_a_pet", |args: &[Py]| reads_for_a_pet(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_pet_low", py_func("reads_as_a_pet_low", |args: &[Py]| reads_as_a_pet_low(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_a_pet_condition", py_func("reads_as_a_pet_condition", |args: &[Py]| reads_as_a_pet_condition(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_like_a_symptom", py_func("reads_like_a_symptom", |args: &[Py]| reads_like_a_symptom(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "_one_edit", py_func("_one_edit", |args: &[Py]| _one_edit(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_sudden_loss", py_func("reads_as_sudden_loss", |args: &[Py]| reads_as_sudden_loss(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_someones_death", py_func("reads_as_someones_death", |args: &[Py]| reads_as_someones_death(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "_subject_before", py_func("_subject_before", |args: &[Py]| _subject_before(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_pregnancy_bleeding", py_func("reads_as_pregnancy_bleeding", |args: &[Py]| reads_as_pregnancy_bleeding(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_the_speakers_own_plan", py_func("reads_as_the_speakers_own_plan", |args: &[Py]| reads_as_the_speakers_own_plan(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "_the_result_came_back_well", py_func("_the_result_came_back_well", |args: &[Py]| _the_result_came_back_well(args)));
    pyrt::xrefs::reg_fn("compose.badnews", "reads_as_bad_news", py_func("reads_as_bad_news", |args: &[Py]| reads_as_bad_news(args)));
    pyrt::xrefs::reg_access("compose.badnews", "ADOPTION_WORDS", g_ADOPTION_WORDS, s_ADOPTION_WORDS);
    pyrt::xrefs::reg_access("compose.badnews", "AGE_CHILD", g_AGE_CHILD, s_AGE_CHILD);
    pyrt::xrefs::reg_access("compose.badnews", "AGE_NUMBER", g_AGE_NUMBER, s_AGE_NUMBER);
    pyrt::xrefs::reg_access("compose.badnews", "AWARD_LOST", g_AWARD_LOST, s_AWARD_LOST);
    pyrt::xrefs::reg_access("compose.badnews", "BAD_NEWS", g_BAD_NEWS, s_BAD_NEWS);
    pyrt::xrefs::reg_access("compose.badnews", "BAD_NEWS_ITSELF", g_BAD_NEWS_ITSELF, s_BAD_NEWS_ITSELF);
    pyrt::xrefs::reg_access("compose.badnews", "BARE_ACHE", g_BARE_ACHE, s_BARE_ACHE);
    pyrt::xrefs::reg_access("compose.badnews", "BARE_BODY", g_BARE_BODY, s_BARE_BODY);
    pyrt::xrefs::reg_access("compose.badnews", "BARE_FILLER", g_BARE_FILLER, s_BARE_FILLER);
    pyrt::xrefs::reg_access("compose.badnews", "BARE_HURTS", g_BARE_HURTS, s_BARE_HURTS);
    pyrt::xrefs::reg_access("compose.badnews", "BARE_ILL", g_BARE_ILL, s_BARE_ILL);
    pyrt::xrefs::reg_access("compose.badnews", "BARE_WORD_ONLY", g_BARE_WORD_ONLY, s_BARE_WORD_ONLY);
    pyrt::xrefs::reg_access("compose.badnews", "BODY", g_BODY, s_BODY);
    pyrt::xrefs::reg_access("compose.badnews", "CARED_SICK", g_CARED_SICK, s_CARED_SICK);
    pyrt::xrefs::reg_access("compose.badnews", "CARED_SIGN", g_CARED_SIGN, s_CARED_SIGN);
    pyrt::xrefs::reg_access("compose.badnews", "CARER_BODY", g_CARER_BODY, s_CARER_BODY);
    pyrt::xrefs::reg_access("compose.badnews", "CARER_SYMPTOM", g_CARER_SYMPTOM, s_CARER_SYMPTOM);
    pyrt::xrefs::reg_access("compose.badnews", "CHILD_BY_AGE", g_CHILD_BY_AGE, s_CHILD_BY_AGE);
    pyrt::xrefs::reg_access("compose.badnews", "CLOSED_DOOR", g_CLOSED_DOOR, s_CLOSED_DOOR);
    pyrt::xrefs::reg_access("compose.badnews", "CLOSED_DOOR_ELSEWHERE", g_CLOSED_DOOR_ELSEWHERE, s_CLOSED_DOOR_ELSEWHERE);
    pyrt::xrefs::reg_access("compose.badnews", "CLOSED_DOOR_ELSE_OWN", g_CLOSED_DOOR_ELSE_OWN, s_CLOSED_DOOR_ELSE_OWN);
    pyrt::xrefs::reg_access("compose.badnews", "CLOSED_DOOR_LINES", g_CLOSED_DOOR_LINES, s_CLOSED_DOOR_LINES);
    pyrt::xrefs::reg_access("compose.badnews", "CLOSED_DOOR_WORK", g_CLOSED_DOOR_WORK, s_CLOSED_DOOR_WORK);
    pyrt::xrefs::reg_access("compose.badnews", "CLOSED_DOOR_WORK_OWN", g_CLOSED_DOOR_WORK_OWN, s_CLOSED_DOOR_WORK_OWN);
    pyrt::xrefs::reg_access("compose.badnews", "CONDITION_OWNER", g_CONDITION_OWNER, s_CONDITION_OWNER);
    pyrt::xrefs::reg_access("compose.badnews", "DEATH_SAID", g_DEATH_SAID, s_DEATH_SAID);
    pyrt::xrefs::reg_access("compose.badnews", "HURT", g_HURT, s_HURT);
    pyrt::xrefs::reg_access("compose.badnews", "HURTS", g_HURTS, s_HURTS);
    pyrt::xrefs::reg_access("compose.badnews", "ILL_PART", g_ILL_PART, s_ILL_PART);
    pyrt::xrefs::reg_access("compose.badnews", "IWE", g_IWE, s_IWE);
    pyrt::xrefs::reg_access("compose.badnews", "JOKED", g_JOKED, s_JOKED);
    pyrt::xrefs::reg_access("compose.badnews", "KIN", g_KIN, s_KIN);
    pyrt::xrefs::reg_access("compose.badnews", "LEFT_GONE", g_LEFT_GONE, s_LEFT_GONE);
    pyrt::xrefs::reg_access("compose.badnews", "LEFT_ME", g_LEFT_ME, s_LEFT_ME);
    pyrt::xrefs::reg_access("compose.badnews", "LEFT_ME_RE", g_LEFT_ME_RE, s_LEFT_ME_RE);
    pyrt::xrefs::reg_access("compose.badnews", "LEFT_PLAN", g_LEFT_PLAN, s_LEFT_PLAN);
    pyrt::xrefs::reg_access("compose.badnews", "LEXICON", g_LEXICON, s_LEXICON);
    pyrt::xrefs::reg_access("compose.badnews", "LIVING", g_LIVING, s_LIVING);
    pyrt::xrefs::reg_access("compose.badnews", "LOST_CHANCE", g_LOST_CHANCE, s_LOST_CHANCE);
    pyrt::xrefs::reg_access("compose.badnews", "LOST_PATTERN", g_LOST_PATTERN, s_LOST_PATTERN);
    pyrt::xrefs::reg_access("compose.badnews", "LUMP_NOT_A_LUMP", g_LUMP_NOT_A_LUMP, s_LUMP_NOT_A_LUMP);
    pyrt::xrefs::reg_access("compose.badnews", "MARRIAGE_ENDED", g_MARRIAGE_ENDED, s_MARRIAGE_ENDED);
    pyrt::xrefs::reg_access("compose.badnews", "MARRIAGE_ENDED_RE", g_MARRIAGE_ENDED_RE, s_MARRIAGE_ENDED_RE);
    pyrt::xrefs::reg_access("compose.badnews", "NEARLY", g_NEARLY, s_NEARLY);
    pyrt::xrefs::reg_access("compose.badnews", "NEWS_WITH_A_NAME", g_NEWS_WITH_A_NAME, s_NEWS_WITH_A_NAME);
    pyrt::xrefs::reg_access("compose.badnews", "NOT_A_TYPO", g_NOT_A_TYPO, s_NOT_A_TYPO);
    pyrt::xrefs::reg_access("compose.badnews", "NOT_THEIRS", g_NOT_THEIRS, s_NOT_THEIRS);
    pyrt::xrefs::reg_access("compose.badnews", "OTHERS", g_OTHERS, s_OTHERS);
    pyrt::xrefs::reg_access("compose.badnews", "OWN_PLAN", g_OWN_PLAN, s_OWN_PLAN);
    pyrt::xrefs::reg_access("compose.badnews", "PARTNER", g_PARTNER, s_PARTNER);
    pyrt::xrefs::reg_access("compose.badnews", "PARTNER_LEAVING", g_PARTNER_LEAVING, s_PARTNER_LEAVING);
    pyrt::xrefs::reg_access("compose.badnews", "PARTNER_LEAVING_RE", g_PARTNER_LEAVING_RE, s_PARTNER_LEAVING_RE);
    pyrt::xrefs::reg_access("compose.badnews", "PARTNER_LEFT", g_PARTNER_LEFT, s_PARTNER_LEFT);
    pyrt::xrefs::reg_access("compose.badnews", "PARTNER_LEFT_RE", g_PARTNER_LEFT_RE, s_PARTNER_LEFT_RE);
    pyrt::xrefs::reg_access("compose.badnews", "PETS", g_PETS, s_PETS);
    pyrt::xrefs::reg_access("compose.badnews", "PET_CONDITION", g_PET_CONDITION, s_PET_CONDITION);
    pyrt::xrefs::reg_access("compose.badnews", "PET_LOW", g_PET_LOW, s_PET_LOW);
    pyrt::xrefs::reg_access("compose.badnews", "PET_LOW_SYMPTOM", g_PET_LOW_SYMPTOM, s_PET_LOW_SYMPTOM);
    pyrt::xrefs::reg_access("compose.badnews", "PET_NAMED_CONDITION", g_PET_NAMED_CONDITION, s_PET_NAMED_CONDITION);
    pyrt::xrefs::reg_access("compose.badnews", "PET_SYMPTOM", g_PET_SYMPTOM, s_PET_SYMPTOM);
    pyrt::xrefs::reg_access("compose.badnews", "PREGNANCY_BLEEDING", g_PREGNANCY_BLEEDING, s_PREGNANCY_BLEEDING);
    pyrt::xrefs::reg_access("compose.badnews", "REPORTED_LEAVING", g_REPORTED_LEAVING, s_REPORTED_LEAVING);
    pyrt::xrefs::reg_access("compose.badnews", "REPORTED_LEAVING_RE", g_REPORTED_LEAVING_RE, s_REPORTED_LEAVING_RE);
    pyrt::xrefs::reg_access("compose.badnews", "SACKED_FIRED", g_SACKED_FIRED, s_SACKED_FIRED);
    pyrt::xrefs::reg_access("compose.badnews", "SICK", g_SICK, s_SICK);
    pyrt::xrefs::reg_access("compose.badnews", "STILL_COMING", g_STILL_COMING, s_STILL_COMING);
    pyrt::xrefs::reg_access("compose.badnews", "STILL_HERE", g_STILL_HERE, s_STILL_HERE);
    pyrt::xrefs::reg_access("compose.badnews", "SUDDEN_LOSS", g_SUDDEN_LOSS, s_SUDDEN_LOSS);
    pyrt::xrefs::reg_access("compose.badnews", "SUICIDE", g_SUICIDE, s_SUICIDE);
    pyrt::xrefs::reg_access("compose.badnews", "SYMPTOM", g_SYMPTOM, s_SYMPTOM);
    pyrt::xrefs::reg_access("compose.badnews", "TROUBLE_OVER", g_TROUBLE_OVER, s_TROUBLE_OVER);
    pyrt::xrefs::reg_access("compose.badnews", "VOWELS", g_VOWELS, s_VOWELS);
    pyrt::xrefs::reg_access("compose.badnews", "_AFTER_IT", g__AFTER_IT, s__AFTER_IT);
    pyrt::xrefs::reg_access("compose.badnews", "_CARRIES", g__CARRIES, s__CARRIES);
    pyrt::xrefs::reg_access("compose.badnews", "_DEATH_WORDS", g__DEATH_WORDS, s__DEATH_WORDS);
    pyrt::xrefs::reg_access("compose.badnews", "_HOW_IT_IS_SAID", g__HOW_IT_IS_SAID, s__HOW_IT_IS_SAID);
    pyrt::xrefs::reg_access("compose.badnews", "_LEAVE_THE_NOUN", g__LEAVE_THE_NOUN, s__LEAVE_THE_NOUN);
    pyrt::xrefs::reg_access("compose.badnews", "_NAMES_ITS_SUBJECT", g__NAMES_ITS_SUBJECT, s__NAMES_ITS_SUBJECT);
    pyrt::xrefs::reg_access("compose.badnews", "_NEARLY", g__NEARLY, s__NEARLY);
    pyrt::xrefs::reg_access("compose.badnews", "_PERSON", g__PERSON, s__PERSON);
    pyrt::xrefs::reg_access("compose.badnews", "_PLACE", g__PLACE, s__PLACE);
    pyrt::xrefs::reg_access("compose.badnews", "_WORD", g__WORD, s__WORD);
    pyrt::xrefs::reg_access("compose.badnews", "_dream", g__dream, s__dream);
    pyrt::xrefs::reg_access("compose.badnews", "line", g_line, s_line);
    pyrt::xrefs::reg_access("compose.badnews", "low", g_low, s_low);
    pyrt::xrefs::reg_access("compose.badnews", "without_the_figure_of_speech", g_without_the_figure_of_speech, s_without_the_figure_of_speech);
}

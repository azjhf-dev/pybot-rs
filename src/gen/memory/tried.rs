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


pub static ACTED_REPLIES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACTED_SAID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ALREADY_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWER_MODULES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_NEXT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ASK_WHAT_TRIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_CONDITION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BUYING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOING_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DONE_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ELSEWHERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EVIDENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FAULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEALTH: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPROVED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_CARE_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_CARE_ASK_UNNAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IN_CARE_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEPS_IT_RIGHT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MADE_IT_WORSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MARK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MORE_STEP_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static MOVES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_TRY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEXT_ASK_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEXT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_A_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NOT_WORKED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARTICIPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_IRREGULAR: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_STEP_ED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PAST_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_AT_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static POINTS_AT_THE_THING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRESENT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PROBLEM_OR_FIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PRONOUN_TAIL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAME_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAME_STILL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAY_ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_PAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_ALREADY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_DID: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_DONE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_EMPTY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_LAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_TAIL_NOUN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_TIMING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_TRIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_VAGUE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_ABOUT_A_PERSON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_ASK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_ASKS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_GOING_ON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_ONLY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_SPECIFIC: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STILL_STATE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TAIL_WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOLD_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TOO_COMMON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRADES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static TRIED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WORSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static YET: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ACTED_REPLIES() -> Py { ensure_init(); ACTED_REPLIES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTED_REPLIES(v: Py) { ensure_init(); if let Ok(mut w) = ACTED_REPLIES.write() { *w = v; } }
pub fn g_ACTED_SAID() -> Py { ensure_init(); ACTED_SAID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACTED_SAID(v: Py) { ensure_init(); if let Ok(mut w) = ACTED_SAID.write() { *w = v; } }
pub fn g_ALREADY_LEADS() -> Py { ensure_init(); ALREADY_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ALREADY_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = ALREADY_LEADS.write() { *w = v; } }
pub fn g_ANSWER_MODULES() -> Py { ensure_init(); ANSWER_MODULES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWER_MODULES(v: Py) { ensure_init(); if let Ok(mut w) = ANSWER_MODULES.write() { *w = v; } }
pub fn g_ASK_NEXT() -> Py { ensure_init(); ASK_NEXT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_NEXT(v: Py) { ensure_init(); if let Ok(mut w) = ASK_NEXT.write() { *w = v; } }
pub fn g_ASK_WHAT_TRIED() -> Py { ensure_init(); ASK_WHAT_TRIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ASK_WHAT_TRIED(v: Py) { ensure_init(); if let Ok(mut w) = ASK_WHAT_TRIED.write() { *w = v; } }
pub fn g_A_CONDITION() -> Py { ensure_init(); A_CONDITION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_CONDITION(v: Py) { ensure_init(); if let Ok(mut w) = A_CONDITION.write() { *w = v; } }
pub fn g_BUYING() -> Py { ensure_init(); BUYING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BUYING(v: Py) { ensure_init(); if let Ok(mut w) = BUYING.write() { *w = v; } }
pub fn g_CLAUSE() -> Py { ensure_init(); CLAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE.write() { *w = v; } }
pub fn g_DOING() -> Py { ensure_init(); DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING(v: Py) { ensure_init(); if let Ok(mut w) = DOING.write() { *w = v; } }
pub fn g_DOING_WORD() -> Py { ensure_init(); DOING_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOING_WORD(v: Py) { ensure_init(); if let Ok(mut w) = DOING_WORD.write() { *w = v; } }
pub fn g_DONE_LEADS() -> Py { ensure_init(); DONE_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DONE_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = DONE_LEADS.write() { *w = v; } }
pub fn g_ELSEWHERE() -> Py { ensure_init(); ELSEWHERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ELSEWHERE(v: Py) { ensure_init(); if let Ok(mut w) = ELSEWHERE.write() { *w = v; } }
pub fn g_END() -> Py { ensure_init(); END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END(v: Py) { ensure_init(); if let Ok(mut w) = END.write() { *w = v; } }
pub fn g_EVIDENCE() -> Py { ensure_init(); EVIDENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EVIDENCE(v: Py) { ensure_init(); if let Ok(mut w) = EVIDENCE.write() { *w = v; } }
pub fn g_FAULT() -> Py { ensure_init(); FAULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FAULT(v: Py) { ensure_init(); if let Ok(mut w) = FAULT.write() { *w = v; } }
pub fn g_HE() -> Py { ensure_init(); HE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HE(v: Py) { ensure_init(); if let Ok(mut w) = HE.write() { *w = v; } }
pub fn g_HEALTH() -> Py { ensure_init(); HEALTH.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEALTH(v: Py) { ensure_init(); if let Ok(mut w) = HEALTH.write() { *w = v; } }
pub fn g_IMPROVED() -> Py { ensure_init(); IMPROVED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPROVED(v: Py) { ensure_init(); if let Ok(mut w) = IMPROVED.write() { *w = v; } }
pub fn g_IN_CARE() -> Py { ensure_init(); IN_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_CARE(v: Py) { ensure_init(); if let Ok(mut w) = IN_CARE.write() { *w = v; } }
pub fn g_IN_CARE_ASK() -> Py { ensure_init(); IN_CARE_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_CARE_ASK(v: Py) { ensure_init(); if let Ok(mut w) = IN_CARE_ASK.write() { *w = v; } }
pub fn g_IN_CARE_ASK_UNNAMED() -> Py { ensure_init(); IN_CARE_ASK_UNNAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_CARE_ASK_UNNAMED(v: Py) { ensure_init(); if let Ok(mut w) = IN_CARE_ASK_UNNAMED.write() { *w = v; } }
pub fn g_IN_CARE_SELF() -> Py { ensure_init(); IN_CARE_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IN_CARE_SELF(v: Py) { ensure_init(); if let Ok(mut w) = IN_CARE_SELF.write() { *w = v; } }
pub fn g_KEEPS_IT_RIGHT() -> Py { ensure_init(); KEEPS_IT_RIGHT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEPS_IT_RIGHT(v: Py) { ensure_init(); if let Ok(mut w) = KEEPS_IT_RIGHT.write() { *w = v; } }
pub fn g_LEADS() -> Py { ensure_init(); LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = LEADS.write() { *w = v; } }
pub fn g_LEAD_IN() -> Py { ensure_init(); LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = LEAD_IN.write() { *w = v; } }
pub fn g_MADE_IT_WORSE() -> Py { ensure_init(); MADE_IT_WORSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MADE_IT_WORSE(v: Py) { ensure_init(); if let Ok(mut w) = MADE_IT_WORSE.write() { *w = v; } }
pub fn g_MARK() -> Py { ensure_init(); MARK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MARK(v: Py) { ensure_init(); if let Ok(mut w) = MARK.write() { *w = v; } }
pub fn g_MORE_STEP_VERBS() -> Py { ensure_init(); MORE_STEP_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MORE_STEP_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = MORE_STEP_VERBS.write() { *w = v; } }
pub fn g_MOVES() -> Py { ensure_init(); MOVES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_MOVES(v: Py) { ensure_init(); if let Ok(mut w) = MOVES.write() { *w = v; } }
pub fn g_NAMED_TRY() -> Py { ensure_init(); NAMED_TRY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_TRY(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_TRY.write() { *w = v; } }
pub fn g_NEXT_ASK_WORDS() -> Py { ensure_init(); NEXT_ASK_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEXT_ASK_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = NEXT_ASK_WORDS.write() { *w = v; } }
pub fn g_NEXT_LEADS() -> Py { ensure_init(); NEXT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEXT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = NEXT_LEADS.write() { *w = v; } }
pub fn g_NOT_A_STATE() -> Py { ensure_init(); NOT_A_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_A_STATE(v: Py) { ensure_init(); if let Ok(mut w) = NOT_A_STATE.write() { *w = v; } }
pub fn g_NOT_WORKED() -> Py { ensure_init(); NOT_WORKED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NOT_WORKED(v: Py) { ensure_init(); if let Ok(mut w) = NOT_WORKED.write() { *w = v; } }
pub fn g_PARTICIPLE() -> Py { ensure_init(); PARTICIPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARTICIPLE(v: Py) { ensure_init(); if let Ok(mut w) = PARTICIPLE.write() { *w = v; } }
pub fn g_PAST_IRREGULAR() -> Py { ensure_init(); PAST_IRREGULAR.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_IRREGULAR(v: Py) { ensure_init(); if let Ok(mut w) = PAST_IRREGULAR.write() { *w = v; } }
pub fn g_PAST_STEP() -> Py { ensure_init(); PAST_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_STEP(v: Py) { ensure_init(); if let Ok(mut w) = PAST_STEP.write() { *w = v; } }
pub fn g_PAST_STEP_ED() -> Py { ensure_init(); PAST_STEP_ED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_STEP_ED(v: Py) { ensure_init(); if let Ok(mut w) = PAST_STEP_ED.write() { *w = v; } }
pub fn g_PAST_WORD() -> Py { ensure_init(); PAST_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PAST_WORD(v: Py) { ensure_init(); if let Ok(mut w) = PAST_WORD.write() { *w = v; } }
pub fn g_POINTING() -> Py { ensure_init(); POINTING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTING(v: Py) { ensure_init(); if let Ok(mut w) = POINTING.write() { *w = v; } }
pub fn g_POINTS_AT_IT() -> Py { ensure_init(); POINTS_AT_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT_IT(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_AT_IT.write() { *w = v; } }
pub fn g_POINTS_AT_THE_THING() -> Py { ensure_init(); POINTS_AT_THE_THING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_POINTS_AT_THE_THING(v: Py) { ensure_init(); if let Ok(mut w) = POINTS_AT_THE_THING.write() { *w = v; } }
pub fn g_PRESENT_LEADS() -> Py { ensure_init(); PRESENT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRESENT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = PRESENT_LEADS.write() { *w = v; } }
pub fn g_PROBLEM_OR_FIX() -> Py { ensure_init(); PROBLEM_OR_FIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PROBLEM_OR_FIX(v: Py) { ensure_init(); if let Ok(mut w) = PROBLEM_OR_FIX.write() { *w = v; } }
pub fn g_PRONOUN_TAIL() -> Py { ensure_init(); PRONOUN_TAIL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PRONOUN_TAIL(v: Py) { ensure_init(); if let Ok(mut w) = PRONOUN_TAIL.write() { *w = v; } }
pub fn g_SAME_STEP() -> Py { ensure_init(); SAME_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAME_STEP(v: Py) { ensure_init(); if let Ok(mut w) = SAME_STEP.write() { *w = v; } }
pub fn g_SAME_STILL() -> Py { ensure_init(); SAME_STILL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAME_STILL(v: Py) { ensure_init(); if let Ok(mut w) = SAME_STILL.write() { *w = v; } }
pub fn g_SAY_ADVICE() -> Py { ensure_init(); SAY_ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAY_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = SAY_ADVICE.write() { *w = v; } }
pub fn g_SHE() -> Py { ensure_init(); SHE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHE(v: Py) { ensure_init(); if let Ok(mut w) = SHE.write() { *w = v; } }
pub fn g_SHORT_PAST() -> Py { ensure_init(); SHORT_PAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_PAST(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_PAST.write() { *w = v; } }
pub fn g_STEP_ALREADY() -> Py { ensure_init(); STEP_ALREADY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_ALREADY(v: Py) { ensure_init(); if let Ok(mut w) = STEP_ALREADY.write() { *w = v; } }
pub fn g_STEP_DID() -> Py { ensure_init(); STEP_DID.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_DID(v: Py) { ensure_init(); if let Ok(mut w) = STEP_DID.write() { *w = v; } }
pub fn g_STEP_DONE() -> Py { ensure_init(); STEP_DONE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_DONE(v: Py) { ensure_init(); if let Ok(mut w) = STEP_DONE.write() { *w = v; } }
pub fn g_STEP_EMPTY() -> Py { ensure_init(); STEP_EMPTY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_EMPTY(v: Py) { ensure_init(); if let Ok(mut w) = STEP_EMPTY.write() { *w = v; } }
pub fn g_STEP_LAST() -> Py { ensure_init(); STEP_LAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_LAST(v: Py) { ensure_init(); if let Ok(mut w) = STEP_LAST.write() { *w = v; } }
pub fn g_STEP_LEADS() -> Py { ensure_init(); STEP_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = STEP_LEADS.write() { *w = v; } }
pub fn g_STEP_TAIL_NOUN() -> Py { ensure_init(); STEP_TAIL_NOUN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_TAIL_NOUN(v: Py) { ensure_init(); if let Ok(mut w) = STEP_TAIL_NOUN.write() { *w = v; } }
pub fn g_STEP_TIMING() -> Py { ensure_init(); STEP_TIMING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_TIMING(v: Py) { ensure_init(); if let Ok(mut w) = STEP_TIMING.write() { *w = v; } }
pub fn g_STEP_TRIED() -> Py { ensure_init(); STEP_TRIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_TRIED(v: Py) { ensure_init(); if let Ok(mut w) = STEP_TRIED.write() { *w = v; } }
pub fn g_STEP_VAGUE() -> Py { ensure_init(); STEP_VAGUE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_VAGUE(v: Py) { ensure_init(); if let Ok(mut w) = STEP_VAGUE.write() { *w = v; } }
pub fn g_STEP_WORDS() -> Py { ensure_init(); STEP_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = STEP_WORDS.write() { *w = v; } }
pub fn g_STILL_ABOUT_A_PERSON() -> Py { ensure_init(); STILL_ABOUT_A_PERSON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_ABOUT_A_PERSON(v: Py) { ensure_init(); if let Ok(mut w) = STILL_ABOUT_A_PERSON.write() { *w = v; } }
pub fn g_STILL_ASK() -> Py { ensure_init(); STILL_ASK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_ASK(v: Py) { ensure_init(); if let Ok(mut w) = STILL_ASK.write() { *w = v; } }
pub fn g_STILL_ASKS() -> Py { ensure_init(); STILL_ASKS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_ASKS(v: Py) { ensure_init(); if let Ok(mut w) = STILL_ASKS.write() { *w = v; } }
pub fn g_STILL_GOING_ON() -> Py { ensure_init(); STILL_GOING_ON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_GOING_ON(v: Py) { ensure_init(); if let Ok(mut w) = STILL_GOING_ON.write() { *w = v; } }
pub fn g_STILL_ONLY() -> Py { ensure_init(); STILL_ONLY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_ONLY(v: Py) { ensure_init(); if let Ok(mut w) = STILL_ONLY.write() { *w = v; } }
pub fn g_STILL_SPECIFIC() -> Py { ensure_init(); STILL_SPECIFIC.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_SPECIFIC(v: Py) { ensure_init(); if let Ok(mut w) = STILL_SPECIFIC.write() { *w = v; } }
pub fn g_STILL_STATE() -> Py { ensure_init(); STILL_STATE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STILL_STATE(v: Py) { ensure_init(); if let Ok(mut w) = STILL_STATE.write() { *w = v; } }
pub fn g_TAIL_WORD() -> Py { ensure_init(); TAIL_WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TAIL_WORD(v: Py) { ensure_init(); if let Ok(mut w) = TAIL_WORD.write() { *w = v; } }
pub fn g_TOLD_THE_BOT() -> Py { ensure_init(); TOLD_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOLD_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = TOLD_THE_BOT.write() { *w = v; } }
pub fn g_TOO_COMMON() -> Py { ensure_init(); TOO_COMMON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TOO_COMMON(v: Py) { ensure_init(); if let Ok(mut w) = TOO_COMMON.write() { *w = v; } }
pub fn g_TRADES() -> Py { ensure_init(); TRADES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRADES(v: Py) { ensure_init(); if let Ok(mut w) = TRADES.write() { *w = v; } }
pub fn g_TRIED() -> Py { ensure_init(); TRIED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_TRIED(v: Py) { ensure_init(); if let Ok(mut w) = TRIED.write() { *w = v; } }
pub fn g_WORD() -> Py { ensure_init(); WORD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORD(v: Py) { ensure_init(); if let Ok(mut w) = WORD.write() { *w = v; } }
pub fn g_WORSE() -> Py { ensure_init(); WORSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WORSE(v: Py) { ensure_init(); if let Ok(mut w) = WORSE.write() { *w = v; } }
pub fn g_YET() -> Py { ensure_init(); YET.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_YET(v: Py) { ensure_init(); if let Ok(mut w) = YET.write() { *w = v; } }
fn module_body() {
    let _ = py_str("A follow-up that says the last answer did not work.\n\n\"i tried that and it didn't work\", \"still blocked\", \"no luck\" is about the answer the bot has\njust given, the way \"how long will it take\" is. Nothing read it as one, in both directions: the\nline fell to \"Noted! What would be most useful to talk about?\" at one end, and at the other the\nanswer modules handed over the same instructions again, word for word - the one thing a friend\nnever does with a problem that has not moved.\n\nSo this module reads the line as what it is and says the next thing: when the home fix has done\nall it can, who does it for a living. It only fires on the turn after an answer, and only for a\nshort line, so an ordinary sentence that happens to contain \"no luck\" (\"no luck finding a\nplumber\") is not read as a report on the answer.\n\nThe trade is looked up from the words of the answer that did not work, not from the new line:\n\"still blocked\" after the sink entry has no word of its own to say what is blocked.\n");
    s_ANSWER_MODULES(py_tuple(vec![py_str("answers")]));
    s_NOT_WORKED(stdlib_call("re", "compile", &[py_str("\\b(?:did(?:n'?t| not) (?:work|help|do (?:anything|much|the trick|it)|shift (?:it|them)|budge|move|make (?:any |a )?difference|change anything|touch (?:it|them))|has(?:n'?t| not) worked|is(?:n'?t| not) working|made (?:no|not much) difference|no luck|nothing (?:happened|worked)|no (?:change|better|improvement)|what if (?:that|it|this) (?:doesn'?t|didn'?t|isn'?t) work(?:ed|ing)?|(?:it'?s|it is|they'?re|they are|its|there'?s) still there|still (?:not (?:working|worked|fixed|shifted|moved|helped)|blocked|broken|stuck|the same|doing it|coming back|leaking|clogged|dripping))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_ONLY(stdlib_call("re", "compile", &[py_str("^(?:it|its|it's|it is|they|theyre|they're|they are|that|thats|that's|that is|this|this is|there|theres|there's|he|hes|he's|he is|she|shes|she's|she is)?\\s*still\\s+(?P<tail>[a-z][a-z' -]*)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_AT_IT(call_builtin("frozenset", &[py_tuple(vec![py_str("there"), py_str("the same"), py_str("like that"), py_str("like this"), py_str("doing it"), py_str("at it"), py_str("the same thing"), py_str("no better"), py_str("not working")])]).unwrap_or_else(|e| exc::raise(e)));
    s_TOO_COMMON(call_builtin("frozenset", &[mcall(&py_str("the and but for not still it its this that them they there here\nout off now just very really again same doing like been being was were are had has have\none two three"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_TRIED(stdlib_call("re", "compile", &[py_str("\\b(?:i|we|ive|i've|weve|we've|they)\\s+(?:already\\s+|just\\s+)?(?:tried|did|used|gave)\\s+(?:that|this|it|them|those)\\b|\\btried (?:that|this|it|everything|all of it)\\b|\\btried everything\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MADE_IT_WORSE(stdlib_call("re", "compile", &[py_str("\\bnow\\b[^.?!]{0,24}\\b(?:worse|faster|slower|louder|stuck|jammed|seized|stripped|snapped|broke|broken|leaking|dripping|spraying|sparking|dead|wont|won'?t|stopped (?:working|moving|spinning|draining)|wobbl\\w*|cracked|bent)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTS_AT_THE_THING(stdlib_call("re", "compile", &[py_str("\\b(?:it|its|it's|them|they|that|those)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ELSEWHERE(stdlib_call("re", "compile", &[py_str("\\b(?:find|finding|found|book|booking|booked|get hold of|reach|reaching|phone|phoning|ring|ringing)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_YET(stdlib_call("re", "compile", &[py_str("\\byet\\b\\W*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_IMPROVED(stdlib_call("re", "compile", &[py_str("\\b(?:(?:it|that|this|they|its|thats) (?:has |have |is |are |'s |seems? )?(?:actually |really |definitely |properly |much |a bit )?(?:helped|helping|works|worked|working|done the trick|did the trick|sorted it)|(?:a bit|lots|much|loads|far|slightly|no end) better|feeling better|on the mend|clearing up|eased off)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORSE(stdlib_call("re", "compile", &[py_str("\\b(?:made (?:it|things|me) (?:worse|sorer)|makes it worse|worse than before|sorer|in agony|unbearable)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_WORD(stdlib_call("re", "compile", &[py_str("[a-z0-9']+")]).unwrap_or_else(|e| exc::raise(e)));
    s_TRADES(py_tuple(vec![py_tuple(vec![py_tuple(vec![py_str("ant"), py_str("ants"), py_str("mouse"), py_str("mice"), py_str("rat"), py_str("rats"), py_str("wasp"), py_str("wasps"), py_str("wasp nest"), py_str("flea"), py_str("fleas"), py_str("moth"), py_str("moths"), py_str("cockroach"), py_str("cockroaches"), py_str("bedbug"), py_str("bedbugs"), py_str("bed bug"), py_str("bed bugs"), py_str("silverfish"), py_str("pest"), py_str("pests"), py_str("pigeon"), py_str("pigeons"), py_str("squirrel"), py_str("squirrels")]), py_str("a pest control firm, and the council's pest service is usually the cheapest way in"), Py::None]), py_tuple(vec![py_tuple(vec![py_str("stain"), py_str("stains"), py_str("carpet"), py_str("sofa"), py_str("mattress"), py_str("curtain"), py_str("curtains"), py_str("upholstery"), py_str("dry clean"), py_str("dryclean")]), py_str("a dry cleaner, who will know what the mark is and say whether it will come out"), py_str("mark")]), py_tuple(vec![py_tuple(vec![py_str("laptop"), py_str("laptop"), py_str("phone"), py_str("iphone"), py_str("android"), py_str("tablet"), py_str("printer"), py_str("router"), py_str("broadband"), py_str("wifi"), py_str("computer"), py_str("pc")]), py_str("the maker's support line, where they can watch it happen on a call"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("car"), py_str("cars"), py_str("engine"), py_str("brake"), py_str("brakes"), py_str("clutch"), py_str("mot"), py_str("gearbox"), py_str("battery"), py_str("tyre"), py_str("tyres"), py_str("steering"), py_str("exhaust")]), py_str("a garage, and it is worth saying what you have already changed"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("washing machine"), py_str("dishwasher"), py_str("fridge"), py_str("freezer"), py_str("oven"), py_str("tumble"), py_str("dryer"), py_str("vacuum"), py_str("hoover"), py_str("microwave"), py_str("kettle"), py_str("toaster"), py_str("appliance"), py_str("appliances")]), py_str("the maker's repair line, and five minutes on the phone settles whether it is worth fixing"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("boiler"), py_str("boilers"), py_str("radiator"), py_str("radiators"), py_str("heating"), py_str("pilot"), py_str("thermostat"), py_str("flue"), py_str("pump"), py_str("gas"), py_str("combi"), py_str("immersion")]), py_str("a Gas Safe engineer, since anything on the gas side is theirs by law"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("tap"), py_str("taps"), py_str("pipe"), py_str("pipes"), py_str("leak"), py_str("leaking"), py_str("joint"), py_str("washer"), py_str("valve"), py_str("drip"), py_str("dripping")]), py_str("a plumber, who can say whether it is a five-minute washer or a bigger job"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("sink"), py_str("sinks"), py_str("drain"), py_str("drains"), py_str("plunger"), py_str("trap"), py_str("toilet"), py_str("waste"), py_str("sewer"), py_str("sewage"), py_str("overflow"), py_str("shower"), py_str("showers")]), py_str("a plumber, with the plunger and the trap already ruled out"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("door"), py_str("doors"), py_str("window"), py_str("windows"), py_str("hinge"), py_str("hinges"), py_str("lock"), py_str("locks"), py_str("handle"), py_str("frame"), py_str("latch"), py_str("bolt"), py_str("draught"), py_str("draft")]), py_str("a joiner or a handyman, who will tell you whether it is the hinge or the frame"), py_str("fault")]), py_tuple(vec![py_tuple(vec![py_str("dog"), py_str("dogs"), py_str("cat"), py_str("cats"), py_str("rabbit"), py_str("hamster"), py_str("guinea"), py_str("parrot"), py_str("puppy"), py_str("kitten")]), py_str("a vet, since an animal that is not mending is not something to wait on"), py_str("health")])]));
    s_ASK_WHAT_TRIED(py_tuple(vec![py_str("Which things have you tried? I'll skip those and go at what's left."), py_str("Tell me what you've already done and I'll not send you round it again.")]));
    s_STILL_GOING_ON(py_tuple(vec![py_str("Still there, then. Any better than it was, or much the same?"), py_str("Still with you — has it eased at all since it started?"), py_str("So it hasn't gone. Is it the same as when it started, or worse?")]));
    s_SAME_STILL(py_tuple(vec![py_str("Right, so that hasn't shifted. What happens now when you try — nothing at all, or something different from before?"), py_str("So it's still going on, then. What have you noticed since — the same as before, or has it changed?"), py_str("Then it hasn't moved. Tell me what it does now and I'll take it from there.")]));
    s_ASK_NEXT(py_tuple(vec![py_str("Then the easy fixes have had their go. Tell me what it's doing now — still the same, different, or worse — and I'll see where that points."), py_str("Right, so that's not it. What's it doing now: the same as before, or has it changed?"), py_str("So the usual thing hasn't touched it. What happens now when you try — nothing at all, or something different from before?")]));
    s_STILL_ASK(py_tuple(vec![py_str("Still {state}, then. What happens when you try it again — nothing at all, or something different?"), py_str("Right, still {state}. How long has it been going on, and is it all the time now?"), py_str("Still {state}, noted. What happens just before it goes like that?")]));
    s_END(py_tuple(vec![py_str("Then I'd stop spending on it and get someone in to look — that's the point where a home fix stops being the cheap option."), py_str("Honestly, at that point I'd have someone look at it properly rather than try a fourth thing."), py_str("Then it's past what a weekend and a how-to video can do. Get it looked at properly.")]));
    s_LEADS(py_tuple(vec![py_str("Then it's past a home fix: {trade}."), py_str("That's where the home fix ends, I'm afraid — {trade}."), py_str("Then it wants someone with the right kit: {trade}."), py_str("So it needs {trade}.")]));
    s_DOING(py_str("(?:re-?start\\w*|re-?boot\\w*|re-?set\\w*|clean\\w*|chang\\w*|replac\\w*|ble[wd]|topp?\\w*|unblock\\w*|plung\\w*|wash\\w*|vacuum\\w*|hoover\\w*|re-?install\\w*|uninstall\\w*|updat\\w*|plug\\w*|switch\\w*|check\\w*|tried|us(?:e|ed|ing)\\b|cut\\w*|stop\\w*|start\\w*|took|taken|drunk|drank|walk\\w*|exercis\\w*|stretch\\w*|ice[ds]?|icing|elevat\\w*|book\\w*|air\\w*|vent\\w*|rins\\w*|soak\\w*|scrub\\w*|fit\\w*|fix\\w*|adjust\\w*|turn\\w*|drain\\w*|empt\\w*|defrost\\w*|descal\\w*|brush\\w*|pok\\w*|twist\\w*|tighten\\w*|loosen\\w*|tap\\w*|seal\\w*|open\\w*|clos\\w*|mov\\w*|put\\w*|gave|given?|add\\w*|reduc\\w*|increas\\w*|rais\\w*|lower\\w*)"));
    s_LEAD_IN(py_str("^\\W*(?:(?:and|but|so|oh|ok|okay|well|yeah|yep|no|nah|hmm|ah|right|alright|then)\\W+)*"));
    s_STEP_ALREADY(stdlib_call("re", "compile", &[add(&add(&add(&g_LEAD_IN(), &py_str("(?:i\\s*(?:'ve|ve|have)\\s+|i\\s+|ive\\s+)?already\\s+(?P<what>")), &g_DOING()), &py_str("\\b(?:[a-z0-9' -]|(?<=\\d)\\.(?=\\d)){0,40})$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_TRIED(stdlib_call("re", "compile", &[add(&g_LEAD_IN(), &py_str("(?:i\\s*(?:'ve|ve|have)\\s+|i\\s+|ive\\s+)?(?:already\\s+|just\\s+)?(?:tried|used)\\s+(?P<what>\\S.*)$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_LAST(stdlib_call("re", "compile", &[add(&add(&add(&g_LEAD_IN(), &py_str("(?:i\\s*(?:'ve|ve|have)\\s+|i\\s+|ive\\s+)?(?P<what>")), &g_DOING()), &py_str("\\b(?:[a-z0-9' -]|(?<=\\d)\\.(?=\\d)){0,40})\\s+already\\W*$")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTICIPLE(add(&add(&py_str("(?:"), &g_DOING()), &py_str("|[a-z]{4,}ed|bled|wrung|swept|dug|spun|taken|done|ran|made|left|held|shut|set|put|cut)")));
    s_STEP_DONE(stdlib_call("re", "compile", &[add(&add(&add(&g_LEAD_IN(), &py_str("(?:(?:i|we)\\s*(?:'ve|ve|have)|ive|weve)\\s+(?P<what>")), &g_PARTICIPLE()), &py_str("\\b(?:[a-z0-9' -]|(?<=\\d)\\.(?=\\d)){0,40})")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_DID(stdlib_call("re", "compile", &[add(&add(&add(&g_LEAD_IN(), &py_str("(?:i|we)\\s+(?P<what>")), &g_DOING()), &py_str("\\b(?:[a-z0-9' -]|(?<=\\d)\\.(?=\\d)){0,40})")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_VAGUE(call_builtin("frozenset", &[mcall(&py_str("that this it them those these everything all anything something\n    whatever lot one ones thing things stuff"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_EMPTY(call_builtin("frozenset", &[mcall(&py_str("a an the and but or so oh well yeah yep ok okay right alright already\n    just really very quite still now again too much more some any of in on off out up down with\n    for to at it its it's that this them those my our your his her their"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_LEADS(py_tuple(vec![py_str("You've tried {thing}. Now {step}."), py_str("Right, so {thing} is done. {Step}."), py_str("Then {step}.")]));
    s_ALREADY_LEADS(py_tuple(vec![py_str("You've already {thing}. {Step}."), py_str("Since you've already {thing}, {step}."), py_str("Then {step}.")]));
    s_PAST_WORD(stdlib_call("re", "compile", &[py_str("^[a-z]{2,}(?:ed|en|ght)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT_PAST(call_builtin("frozenset", &[mcall(&py_str("bled fled shed sped"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_STEP_ED(stdlib_call("re", "compile", &[py_str("^[a-z]{3,}ed$")]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_IRREGULAR(call_builtin("frozenset", &[mcall(&py_str("ran did made took gave put set cut kept left found got went saw\n    wrote sold sent spent built burnt read shot told brought bought thought caught taught"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_DONE_LEADS(py_tuple(vec![py_str("You {thing}. Now {step}."), py_str("Right, so you {thing}. Then {step}."), py_str("Then {step}.")]));
    s_PRESENT_LEADS(py_tuple(vec![py_str("You already {thing}. Now {step}."), py_str("Then {step}.")]));
    s_DOING_WORD(stdlib_call("re", "compile", &[add(&add(&py_str("(?:"), &g_DOING()), &py_str("|[a-z]{2,}ed)\\b")), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_TAIL_WORD(stdlib_call("re", "compile", &[py_str("^(?:and|but|or|not|if|so|then|which|that|because|while|rather)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PRONOUN_TAIL(stdlib_call("re", "compile", &[py_str("^(?:put|leave|check|try|give|keep|take|clean|run|top|set|turn|move|wipe|replace|change|open|close|hold|press|unplug|switch|bleed|fill|repeat|do)\\s+(?:it|them|that|this|those|these)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_MORE_STEP_VERBS(call_builtin("frozenset", &[mcall(&py_str("undo unplug reinstall upgrade reboot restart reset force update\n    wipe swap plug thaw rinse scrub polish vacuum hoover descale defrost sand trim scrape blot\n    clamp loosen tighten snip shave share write print send take keep hold hang mount have mow\n    scarify aerate reseed edge"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_WORDS(Py::Int(22));
    s_CLAUSE(stdlib_call("re", "compile", &[py_str("\\s*;\\s*|:\\s+|\\s+then\\s+|\\s+-\\s+|,\\s+and\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_STATE(stdlib_call("re", "compile", &[py_str("\\bstill\\s+(?P<state>[a-z' ]{2,30})\\W*$")]).unwrap_or_else(|e| exc::raise(e)));
    s_NOT_A_STATE(call_builtin("frozenset", &[mcall(&py_str("here there waiting hungry tired thirsty awake asleep up alive\nhurts hurting hurt sore aching achy painful pain ill sick poorly unwell dizzy faint sickly\nupset sad anxious stressed worried down low rough rubbish wobbly shaky weak numb\ndepressed miserable weepy tearful\ngoing coming learning working trying looking thinking hoping planning deciding wondering\nworse better ok okay fine good great bad nice rubbish\nat work in bed on my way at it on it at home on hold in touch\na chance time young single busy late early on off out in the same"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_ASKS(stdlib_call("re", "compile", &[py_str("^(?:are|is|do|does|did|will|would|can|could|should|shall|have|has|why|how|what|when|where|who|which)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_PROBLEM_OR_FIX(stdlib_call("re", "compile", &[py_str("\\b(?:how (?:do|can|should|would|could) i|how to|what (?:do|can|could) i do|any (?:tips|advice)|what should i do)\\b|\\b(?:is|are|keeps|keep|was|wont|won'?t|doesn'?t|doesnt|isn'?t|isnt|not) (?:\\w+ )?(?:working|worked|blocked|broken|stuck|sticking|jammed|jamming|dragging|leaking|dripping|clogged|smelling|smells|noisy|coming back|moving|rising)\\b|\\b(?:blocked|broken|stuck|leaking|clogged)\\b|\\bi (?:can'?t|cant|couldn'?t|couldnt)\\b|\\b(?:wont|won'?t) (?:come|get|go away|stop|budge)\\b|\\b(?:wont|won'?t|doesn'?t|doesnt|keeps? (?:on )?)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_FAULT(stdlib_call("re", "compile", &[py_str("\\b(?:broke|broken|breaking|not working|stopped working|stopped|wont|won'?t|doesn'?t|doesnt|isn'?t|isnt|leak|leaks|leaking|drip|dripping|noise|noisy|banging|knocking|rattl\\w*|blocked|blockage|unblock\\w*|stuck|sticking|jammed|jamming|clogged|dragging|not (?:shutting|opening|closing)|smell|smells|smelling|smelly|smoke|smoking|sparking|dead|fault|faulty|overheat\\w*|flashing|damaged|gone wrong|packed in|keeps? (?:cutting out|going off|stopping|tripping|switching off|playing up|packing in)|cuts? out|cutting out|played up|playing up|on the blink|packing in|not firing|drains? slowly|slow to drain)")]).unwrap_or_else(|e| exc::raise(e)));
    s_HEALTH(stdlib_call("re", "compile", &[py_str("\\b(?:ill|sick|poorly|unwell|pain|hurt\\w*|limp\\w*|vomit\\w*|swollen|swelling|not eating|off (?:his|her|their|its) food|wont eat|won'?t eat|blood|pus|constipated|breathing|wheez\\w*|shaking|trembling|dying|discharge|losing (?:fur|weight))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_MARK(stdlib_call("re", "compile", &[py_str("\\b(?:stain\\w*|mark|marks|smudge|spill\\w*|ink|wine|coffee|grease|scuff)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_EVIDENCE(py_dict(vec![(py_str("fault"), g_FAULT()), (py_str("health"), g_HEALTH()), (py_str("mark"), g_MARK())]));
    s_BUYING(stdlib_call("re", "compile", &[py_str("\\b(?:which one|which (?:\\w+ )?(?:is|to|should)|recommend\\w*|best|any good|worth (?:buying|getting)|should i (?:buy|get)|need a new|needs a new|thinking of (?:buying|getting)|going to (?:buy|get)|want to (?:buy|get)|shopping for|looking (?:for|at) (?:a|some)|what should i (?:buy|get)|how much (?:do|does|are|is) (?:they|it|these))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_SPECIFIC(stdlib_call("re", "compile", &[py_str("\\bstill (?:blocked|leaking|dripping|clogged|stuck|broken|coming back|noisy|smelling|jammed|squeaking|the same|not draining|wont|won'?t|there)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_TRY(stdlib_call("re", "compile", &[py_str("\\b(?:tried|used|gave)\\s+(?:a|an|the|my|some|his|her|their|this new)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTED_SAID(stdlib_call("re", "compile", &[py_str("^(?:i|we)\\s+(?:(?:have|had|already|just|finally|actually)\\s+)*(?:told|done|did|sent|texted|messaged|emailed|called|rang|phoned|asked|said\\s+(?:(?:it|that)\\s+)?to|spoke\\s+(?:to|with)|talked\\s+(?:to|with)|said\\s+what\\s+you\\s+(?:suggested|said)|used\\s+(?:it|that|your\\s+(?:wording|words|line|suggestion|advice|text)))\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAY_ADVICE(stdlib_call("re", "compile", &[py_str("\\b(?:what|how|which)\\s+(?:(?:should|shall|do|can|could|would)\\s+)?(?:i|we)\\s+(?:say|tell|text|message|write|email|call|ask|talk|speak|word)\\b|\\bwhat\\s+to\\s+(?:say|tell|text|message|write|email)\\b|\\b(?:how|what)\\s+(?:would|could|should)\\s+you\\s+(?:say|word|phrase)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_TOLD_THE_BOT(stdlib_call("re", "compile", &[py_str("\\b(?:told|said|texted|messaged|emailed|called|asked)\\s+you\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_ACTED_REPLIES(py_tuple(vec![py_str("How did it land?"), py_str("What did they say?"), py_str("Did it come across the way you meant?"), py_str("How did that go over?"), py_str("Did it help?")]));
    s_STEP_TAIL_NOUN(stdlib_call("re", "compile", &[py_str("^(?:the|a|an|my|its|their|our|your|his|her|this|that)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PAST_STEP(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:and|but|so|oh|ok|okay|well|yeah|yep|no|nah|hmm|ah|right|alright|then)\\W+)*(?:i|we)\\s+(?P<what>[a-z0-9' ]{3,80})$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAME_STEP(call_builtin("frozenset", &[py_tuple(vec![py_str(""), py_str("s"), py_str("es"), py_str("ed"), py_str("d"), py_str("ing")])]).unwrap_or_else(|e| exc::raise(e)));
    s_MOVES(stdlib_call("re", "compile", &[py_str(",\\s*|\\s+so\\s+|\\s+and\\s+"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_CONDITION(stdlib_call("re", "compile", &[py_str("^(?:if|when|once|unless|whenever|wherever)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_TIMING(call_builtin("frozenset", &[mcall(&py_str("last week weeks month months year years today yesterday already now twice\n    once again day days night morning time"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_KEEPS_IT_RIGHT(stdlib_call("re", "compile", &[py_str("\\b(?:stops?|keeps?|prevents?|avoids?|is what|are what|helps?|means)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEXT_LEADS(py_tuple(vec![py_str("Then {step}"), py_str("Next: {step}"), py_str("After that, {step}"), py_str("Then this: {step}")]));
    s_NEXT_ASK_WORDS(call_builtin("frozenset", &[mcall(&py_str("what else anything next now other then after that and can could\n    should would i we you me my it be is are there more one another again try do does done\n    something new also help suggest recommend"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_CARE(stdlib_call("re", "compile", &[py_str("\\b(?:in|at|on) (?:the |a |an )?(?:hospital|infirmary|hospice|care home|nursing home|residential home|ward|icu|itu|hdu|a&e|ae|rehab|clinic)\\b")]).unwrap_or_else(|e| exc::raise(e)));
    s_SHE(call_builtin("frozenset", &[mcall(&py_str("mum mam mummy mom mother mummy nan nana gran granny grandma grandmother\n                   nanny daughter sister wife aunt auntie girlfriend niece"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_HE(call_builtin("frozenset", &[mcall(&py_str("dad daddy father grandad granddad grandpa grandfather son brother husband\n                  uncle boyfriend nephew"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_STILL_ABOUT_A_PERSON(call_builtin("frozenset", &[mcall(&py_str("there the same in now hospital ward home out yet better\n                                    no not like that doing it again care"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_POINTING(call_builtin("frozenset", &[mcall(&py_str("it its it's she she's he he's they they're theyre that theres there's and\n                        but so is are was were s"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s_IN_CARE_ASK(py_tuple(vec![py_str("Still in, then. How is {subj} doing today?"), py_str("No change yet, then. How has your {word} been today?"), py_str("Still there. The waiting is the worst of it, and no news is not bad news."), py_str("Still in, then. Has anyone there said what happens next?"), py_str("Still in. How are you doing with all of it?"), py_str("Still in, then. Did you get to see {obj} today?")]));
    s_IN_CARE_ASK_UNNAMED(py_tuple(vec![py_str("Still in, then. How are they doing today?"), py_str("No change yet, then. How have they been today?"), py_str("Still there. The waiting is the worst of it, and no news is not bad news."), py_str("Still in, then. Has anyone there said what happens next?"), py_str("Still in. How are you doing with all of it?"), py_str("Still in, then. Did you get to see them today?")]));
    s_IN_CARE_SELF(py_tuple(vec![py_str("Still in, then. How are you doing today?"), py_str("Still in. Is someone bringing you the things you need?"), py_str("Still in, then. Has anyone said how long they're keeping you in?")]));
}
pub fn words_of(args: &[Py]) -> Py {
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
    return mcall(&g_WORD(), "findall", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "replace", &[py_str("’"), py_str("'")])]);
    Py::None
}
pub fn reads_as_not_worked(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line is a report that the last answer did not do the job.");
    line = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_ELSEWHERE(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_IMPROVED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORSE(), "search", &[line.clone()]) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_YET(), "search", &[line.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_NOT_WORKED(), "search", &[line.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = mcall(&g_MADE_IT_WORSE(), "search", &[line.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_POINTS_AT_THE_THING(), "search", &[line.clone()]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g_TRIED(), "search", &[line.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn the_still_state(args: &[Py]) -> Py {
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
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut state: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The state a \"still ...\" line reports on the thing, or \"\" when it is not that report.\n\n    \"my shower is cold\" then \"still cold\": the word after \"still\" belongs to the thing, so the\n    line says the answer has not moved it. The state has to be one the chat has already named - in\n    the person's own line or in the words their line was answered by - which is what keeps the\n    bot's own prose out of it: an answer about films says \"sofa\", and \"still sofa\" is nobody's\n    line.\n    ");
    said = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "endswith", &[py_str("?")]) } } } }) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_STILL_ASKS(), "match", &[said.clone()])) {
    return py_str("");
    }
    found = mcall(&g_STILL_STATE(), "search", &[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    state = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[mcall(&found.clone(), "group", &[py_str("state")])])) { let mut word: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STEP_EMPTY(), &word.clone()))) {  __out.push(word.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&state.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&state.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NOT_A_STATE(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    named = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[person_said(&[memory.clone()])])) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() };
    let __aug1 = bitor(&named.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&last_answer_words(&[memory.clone()])) { let mut word: Py = __item.clone(); __out.push(_stem(&[word.clone()])); } py_set(__out) })() });
    named = __aug1;
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&state.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&named.clone(), &_stem(&[word.clone()])))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    if py_truthy(&mcall(&g_HEALTH(), "search", &[mcall(&py_str(" "), "join", &[state.clone()])])) {
    return py_str("");
    }
    return mcall(&py_str(" "), "join", &[state.clone()]);
    Py::None
}
pub fn still_the_state(args: &[Py]) -> Py {
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
    let _ = py_str("True when a short \"still ...\" line names a state the chat has already named.");
    return call_builtin("bool", &[the_still_state(&[text.clone(), memory.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn might_be_a_report(args: &[Py]) -> Py {
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
    said = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(20)) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_NOT_WORKED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MADE_IT_WORSE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TRIED(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STILL_STATE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STEP_DONE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_STEP_DID(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_YET(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_TRY(), "search", &[said.clone()]) } } } } } } } } } } } } } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn names_a_trade(args: &[Py]) -> Py {
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
    let mut _entry_words: Py = Py::None;
    let mut _needs: Py = Py::None;
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trade: Py = Py::None;
    let _ = py_str("True when the line already hands the thing over to a trade.");
    said = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    let __iter1 = py_iter_or_raise(&g_TRADES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    _entry_words = __unpk2[0].clone();
    trade = __unpk2[1].clone();
    _needs = __unpk2[2].clone();
    name = mcall(&mcall(&geti(&mcall(&call_builtin("str", &[{ let __b = trade.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str(",")]), &Py::Int(0)), "strip", &[]), "lower", &[]);
    if py_truthy(&{ let __b = name.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &name.clone())) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn after_an_answer(args: &[Py]) -> Py {
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
    let mut module: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last turn was an answer, so \"still blocked\" is about that answer.\n\n    A turn where the reply already named a trade counts too: \"i tried that\" then \"it didnt work\"\n    is the person reporting twice on the same thing, and the second report is still about an\n    answer. A shrug never counts, so nothing is escalated out of nothing.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    module = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(py_contains(&g_ANSWER_MODULES(), &module.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_eq(&module.clone(), &py_str("chat")))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_eq(&mcall(&memory.clone(), "get", &[py_str("handed_back_step")]), &sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))))) {
    return Py::Bool(true);
    }
    return { let __b = names_a_trade(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")])]); if py_truthy(&__b) { __b } else { asked_what_its_doing(&[memory.clone()]) } };
    Py::None
}
pub fn last_answer_words(args: &[Py]) -> Py {
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
    let mut key: Py = Py::None;
    let mut keys: Py = Py::None;
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The words the person used for the thing, so the trade can be found from them.\n\n    Only the person's own words: the question that was answered and the words the entry was\n    reached by. The bot's prose is deliberately not read, because it is full of everyday nouns -\n    an answer about films says \"a sofa with no planning\", and \"sofa\" would have sent a person\n    whose film did not help off to a dry cleaner.\n    ");
    said = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_lines(&[memory.clone()])) { let mut line: Py = __item.clone(); __out.push(call_builtin("str", &[{ let __b = line.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() };
    last = mcall(&memory.clone(), "get", &[py_str("last_answer")]);
    if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    keys = mcall(&last.clone(), "get", &[py_str("keys")]);
    if py_truthy(&call_builtin("isinstance", &[keys.clone(), py_tuple(vec![py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e))), py_func("tuple", |args: &[Py]| call_builtin("tuple", args).unwrap_or_else(|e| exc::raise(e))), py_func("set", |args: &[Py]| call_builtin("set", args).unwrap_or_else(|e| exc::raise(e))), py_func("frozenset", |args: &[Py]| call_builtin("frozenset", args).unwrap_or_else(|e| exc::raise(e)))])]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&said.clone(), "extend", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&keys.clone()) { let mut key: Py = __item.clone(); __out.push(call_builtin("str", &[key.clone()]).unwrap_or_else(|e| exc::raise(e))); } py_list(__out) })() }]);
    }
    }
    return call_builtin("set", &[words_of(&[mcall(&py_str(" "), "join", &[said.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asks_for_a_fix(args: &[Py]) -> Py {
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
    let mut before: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the turn that was answered was a problem or a how-to, not a recommendation.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    asked = mcall(&py_str(" "), "join", &[words_of(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])])]);
    if py_truthy(&Py::Bool(!py_truthy(&asked.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_PROBLEM_OR_FIX(), "search", &[asked.clone()])) {
    return Py::Bool(true);
    }
    if py_truthy(&gt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))) {
    before = mcall(&py_str(" "), "join", &[words_of(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(2))), "get", &[py_str("user")])])]);
    return call_builtin("bool", &[mcall(&g_PROBLEM_OR_FIX(), "search", &[before.clone()])]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn buying(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the turn that was answered was about choosing or buying, not about a fault.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_BUYING(), "search", &[mcall(&py_str(" "), "join", &[words_of(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])])])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn person_said(args: &[Py]) -> Py {
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
    let _ = py_str("The person's own last two lines, which are the only evidence for a fault.\n\n    Not the words the answer was reached by: an oven-cleaning entry is reached by \"smoke\" and\n    \"grease\", and reading those as a fault sends a person who is cleaning an oven to a repair\n    line. The person's sentence is the one that says whether something has stopped working.\n    ");
    return mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&_the_lines(&[memory.clone()])) { let mut line: Py = __item.clone(); if py_truthy(&line.clone()) {  __out.push(mcall(&py_str(" "), "join", &[words_of(&[line.clone()])])); } } py_list(__out) })() }]);
    Py::None
}
pub fn _the_lines(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The person's own lines that can name the thing, newest first.\n\n    The answered line always counts. The line before it counts only when the answered line is\n    itself a report that a fix did not work (\"it didnt work\", \"the reset did nothing\"), because\n    that is when the thing was named a turn earlier. When the answered line asks about something\n    of its own the chat has moved on, and a thing from before it is the wrong subject: after \"my\n    boiler is broken\" / \"my kids wont sleep\", the report \"tried everything\" was handed \"Then it's\n    past a home fix: a Gas Safe engineer\" - an answer about a boiler to a parent asking about a\n    child's sleep.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_list(vec![]);
    }
    said = py_list(vec![mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])]);
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if !py_truthy(&__b) { __b } else { reads_as_not_worked(&[mcall(&py_str(" "), "join", &[words_of(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])])])]) } }) {
    let _ = mcall(&said.clone(), "append", &[mcall(&geti(&turns.clone(), &neg(&Py::Int(2))), "get", &[py_str("user")])]);
    }
    return said.clone();
    Py::None
}
pub fn _names(args: &[Py]) -> Py {
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
    let mut blob: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "blob") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: blob"))) };
    let mut entry_words: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "entry_words") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: entry_words"))) };
    let mut token: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the words name the thing: the word itself, its plural or -ing, or a phrase.\n\n    \"the washing machine isnt draining\" names a washing machine and a drain; the word list holds\n    both \"washing machine\" (two words, so a whole-token set will never match it) and \"drain\"\n    (which the line has as \"draining\").\n    ");
    if py_truthy(&mcall(&said.clone(), "intersection", &[entry_words.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&entry_words.clone());
    for __item in __iter1 {
    word = __item.clone();
    if py_truthy(&Py::Bool(py_contains(&word.clone(), &py_str(" ")))) {
    if py_truthy(&Py::Bool(py_contains(&blob.clone(), &word.clone()))) {
    return Py::Bool(true);
    }
    } else {
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&said.clone()) { let mut token: Py = __item.clone(); __out.push(mcall(&token.clone(), "startswith", &[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn names_a_thing(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut blob: Py = Py::None;
    let mut entry_words: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("True when the person's own words name something a home fix could be about.\n\n    \"my kids wont eat vegetables\" / \"ive tried everything\" was handed the boiler's next line -\n    \"Tell me what it's doing now - still the same, different, or worse\" - because \"wont\" is a\n    fault word and the turn before was an answer. But a child's dinner is not a thing that is\n    doing anything, and the question is the one asked of a machine. The things that can be\n    waiting on a trade are the ones TRADES is made of, so a line that names none of them has\n    nothing for \"what is it doing now\" to be about and gets no next step from here.\n\n    Only the person's own words count: an answer reached by \"grease\" or \"smoke\" is about an\n    oven, and that is not a person saying their oven is broken.\n    ");
    said = call_builtin("set", &[words_of(&[person_said(&[memory.clone()])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    blob = mcall(&py_str(" "), "join", &[call_builtin("sorted", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))]);
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TRADES()) { let __unpk5832 = py_unpack(&__item.clone(), 3, None); let mut entry_words: Py = __unpk5832[0].clone(); let mut _u: Py = __unpk5832[1].clone(); let mut _u: Py = __unpk5832[2].clone(); __out.push(_names(&[said.clone(), blob.clone(), entry_words.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn has_evidence(args: &[Py]) -> Py {
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
    let mut report: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when there is something for \"what is it doing now\" to be about.\n\n    A vague \"no luck\" after advice about starting to run, cleaning an oven or getting rid of a\n    cold leaves nothing broken to ask about, and \"Tell me what it's doing now\" then reads as a\n    machine talking about an object. So the question is only asked when the turn that was\n    answered names a fault, or the report says what is still wrong, or names what was tried.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    asked = mcall(&py_str(" "), "join", &[words_of(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")])])]);
    if py_truthy(&{ let __b = mcall(&g_FAULT(), "search", &[asked.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_MARK(), "search", &[asked.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_TRY(), "search", &[asked.clone()]) } } } }) {
    return Py::Bool(true);
    }
    report = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&{ let __b = mcall(&g_STILL_SPECIFIC(), "search", &[report.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_TRY(), "search", &[report.clone()]) } }) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[{ let __b = mcall(&g_MADE_IT_WORSE(), "search", &[report.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_POINTS_AT_THE_THING(), "search", &[report.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn asked_what_its_doing(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last turn asked what the thing is doing now.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::Bool(false);
    }
    said = mcall(&py_str(" "), "join", &[words_of(&[mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")])])]);
    return { let __b = Py::Bool(py_contains(&said.clone(), &py_str("doing now"))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &py_str("happens now when you try"))) } };
    Py::None
}
pub fn already_named(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut trade: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "trade") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: trade"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut name: Py = Py::None;
    let mut said: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the last turn already said this trade, so saying it again is a loop.\n\n    \"i tried that\" then \"it didnt work\" is a person reporting twice on the same fix; naming the\n    plumber twice reads as not listening. The second report asks what the thing is doing instead.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    name = mcall(&mcall(&geti(&mcall(&call_builtin("str", &[{ let __b = trade.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str(",")]), &Py::Int(0)), "strip", &[]), "lower", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&name.clone()))) {
    return Py::Bool(false);
    }
    said = mcall(&py_str(" "), "join", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&turns.clone(), &neg(&Py::Int(2)), &Py::None, &Py::None)) { let mut t: Py = __item.clone(); if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&t.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("chat")))) {  __out.push(mcall(&py_str(" "), "join", &[words_of(&[mcall(&t.clone(), "get", &[py_str("bot")])])])); } } py_list(__out) })() }]);
    return { let __b = call_builtin("bool", &[name.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&said.clone(), &name.clone())) } };
    Py::None
}
pub fn _the_answered_line(args: &[Py]) -> Py {
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
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line the last answer answered, which says what the turn was about.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    return if py_truthy(&turns.clone()) { call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)) } else { py_str("") };
    Py::None
}
pub fn trade_for(args: &[Py]) -> Py {
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
    let mut about_a_fault: Py = Py::None;
    let mut blob: Py = Py::None;
    let mut entry_words: Py = Py::None;
    let mut needs: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut said: Py = Py::None;
    let mut trade: Py = Py::None;
    let mut words: Py = Py::None;
    let mut worse: Py = Py::None;
    let _ = py_str("The trade that takes this over, as a phrase, or None when none fits.");
    said = last_answer_words(&[memory.clone()]);
    words = person_said(&[memory.clone()]);
    blob = add(&add(&mcall(&py_str(" "), "join", &[call_builtin("sorted", &[said.clone()]).unwrap_or_else(|e| exc::raise(e))]), &py_str(" ")), &words.clone());
    worse = call_builtin("bool", &[{ let __b = mcall(&g_MADE_IT_WORSE(), "search", &[words.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_POINTS_AT_THE_THING(), "search", &[words.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    about_a_fault = call_builtin("bool", &[mcall(&g_FAULT(), "search", &[mcall(&py_str(" "), "join", &[words_of(&[_the_answered_line(&[memory.clone()])])])])]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_TRADES());
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 3, None);
    entry_words = __unpk2[0].clone();
    trade = __unpk2[1].clone();
    needs = __unpk2[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&_names(&[said.clone(), blob.clone(), entry_words.clone()])))) {
    continue;
    }
    if py_truthy(&Py::Bool(py_is(&needs.clone(), &Py::None))) {
    return trade.clone();
    }
    pattern = mcall(&g_EVIDENCE(), "get", &[needs.clone()]);
    if py_truthy(&Py::Bool(py_is(&pattern.clone(), &Py::None))) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&pattern.clone(), "search", &[words.clone()]); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&needs.clone(), &py_str("fault"))); if !py_truthy(&__b) { __b } else { { let __b = worse.clone(); if !py_truthy(&__b) { __b } else { about_a_fault.clone() } } } } } }) {
    return trade.clone();
    }
    }
    return Py::None;
    Py::None
}
pub fn acted_pool(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut line: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The line after advice that was acted on: ask how it went, not what the advice was.\n\n    \"what should i say to her\" / \"i already told her that\" was answered with\n    \"Got it. Is there anything you'd like a hand with?\", which drops the thread. The person has\n    done the thing the bot suggested; the next useful thing a friend says is a question about\n    what happened, and the store already knows the last turn was advice.\n    ");
    line = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_ACTED_SAID(), "match", &[line.clone()]))); if py_truthy(&__b) { __b } else { mcall(&g_TOLD_THE_BOT(), "search", &[line.clone()]) } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return Py::None;
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("safety")))) {
    return Py::None;
    }
    asked = mcall(&py_str(" "), "join", &[words_of(&[mcall(&last.clone(), "get", &[py_str("user")])])]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SAY_ADVICE(), "search", &[asked.clone()])))) {
    return Py::None;
    }
    return g_ACTED_REPLIES();
    Py::None
}
pub fn _up_to_the_contrast(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut what: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "what") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: what"))) };
    let mut head: Py = Py::None;
    let mut joint: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The step the line names, with a second object kept and a change of subject dropped.");
    pieces = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("\\s+(but|and|though|although|yet)\\s+")); __args.push(what.clone()); __kws.extend(vec![("maxsplit".to_string(), Py::Int(1)), ("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "split", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&lt(&call_builtin("len", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return what.clone();
    }
    let __unpk1 = py_unpack(&pieces.clone(), 3, None);
    head = __unpk1[0].clone();
    joint = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&joint.clone(), "lower", &[]), &py_str("and"))); if !py_truthy(&__b) { __b } else { mcall(&g_STEP_TAIL_NOUN(), "match", &[mcall(&tail.clone(), "strip", &[])]) } }) {
    return what.clone();
    }
    return head.clone();
    Py::None
}
pub fn _read_step_tried(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut pattern: Py = Py::None;
    let mut w: Py = Py::None;
    let mut what: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("(the step the person says they have done, the shape it was said in) or (None, None).\n\n    \"ive tried the plunger\" -> \"the plunger\", \"i already restarted the router\" -> \"restarted the\n    router\", \"cleaned the filter already\" -> \"cleaned the filter\". None when the line names no\n    step (\"tried that\" is the report above, not a step of the answer).\n    ");
    line = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[]), "strip", &[py_str(".,!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&line.clone(), &py_str("?"))); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } } } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_WORKED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_IMPROVED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORSE(), "search", &[line.clone()]) } } } }) {
    return py_tuple(vec![Py::None, Py::None]);
    }
    let __iter1 = py_iter_or_raise(&py_tuple(vec![g_STEP_ALREADY(), g_STEP_TRIED(), g_STEP_LAST(), g_STEP_DONE(), g_STEP_DID()]));
    for __item in __iter1 {
    pattern = __item.clone();
    found = mcall(&pattern.clone(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    what = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("what")]), "split", &[])]), "strip", &[py_str(" .,!'")]);
    what = stdlib_call("re", "sub", &[py_str("\\s+already$"), py_str(""), what.clone()]).unwrap_or_else(|e| exc::raise(e));
    what = stdlib_call("re", "sub", &[py_str("^(?:tried|used)\\s+"), py_str(""), what.clone()]).unwrap_or_else(|e| exc::raise(e));
    what = mcall(&_up_to_the_contrast(&[what.clone()]), "strip", &[py_str(" .,!'")]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[what.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STEP_EMPTY(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&words.clone())); if py_truthy(&__b) { __b } else { le(&call_builtin("set", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_VAGUE()) } }) {
    continue;
    }
    return py_tuple(vec![what.clone(), pattern.clone()]);
    }
    return py_tuple(vec![Py::None, Py::None]);
    Py::None
}
pub fn the_step_tried(args: &[Py]) -> Py {
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
    let _ = py_str("The step the person says they have done, as they put it, or None.");
    return geti(&_read_step_tried(&[text.clone()]), &Py::Int(0));
    Py::None
}
pub fn _the_past_step(args: &[Py]) -> Py {
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
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut w: Py = Py::None;
    let mut what: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The step the line reports in the plain past tense, when it is one of the answer's own.");
    line = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[]), "strip", &[py_str(".,!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&line.clone(), &py_str("?"))) } } } }) {
    return Py::None;
    }
    if py_truthy(&{ let __b = mcall(&g_NOT_WORKED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_IMPROVED(), "search", &[line.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_WORSE(), "search", &[line.clone()]) } } } }) {
    return Py::None;
    }
    found = mcall(&g_PAST_STEP(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    what = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&found.clone(), "group", &[py_str("what")]), "split", &[])]), "strip", &[py_str(" .,!'")]);
    what = stdlib_call("re", "sub", &[py_str("\\s+already$"), py_str(""), what.clone()]).unwrap_or_else(|e| exc::raise(e));
    what = mcall(&geti(&stdlib_call("re", "split", &[py_str("\\s+(?:but|and|though|although|yet)\\s+"), what.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[py_str(" .,!'")]);
    words = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[what.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&Py::Bool(!py_contains(&g_STEP_EMPTY(), &w.clone()))) {  __out.push(w.clone()); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return Py::None;
    }
    first = geti(&words.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_PAST_IRREGULAR(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_PAST_STEP_ED(), "match", &[first.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SHORT_PAST(), &first.clone())) } })) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&in_the_answer(&[what.clone(), said.clone()])))) {
    return Py::None;
    }
    return what.clone();
    Py::None
}
pub fn _the_last_answer(args: &[Py]) -> Py {
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
    let mut last: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The knowledge base's last reply, or \"\" - the answer a report on it is about.");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("answers")))) {
    return py_str("");
    }
    return call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
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
    let mut ending: Py = Py::None;
    let _ = py_str("The word without its ending, so \"topping\", \"topped\" and \"top\" are the same step.");
    let __iter1 = py_iter_or_raise(&py_tuple(vec![py_str("ing"), py_str("ed"), py_str("es"), py_str("s")]));
    for __item in __iter1 {
    ending = __item.clone();
    if py_truthy(&{ let __b = ge(&sub(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(3)); if !py_truthy(&__b) { __b } else { mcall(&word.clone(), "endswith", &[ending.clone()]) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &neg(&call_builtin("len", &[ending.clone()]).unwrap_or_else(|e| exc::raise(e))), &Py::None);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("ing"), py_str("ed")]), &ending.clone())); if !py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&geti(&word.clone(), &neg(&Py::Int(1))), &geti(&word.clone(), &neg(&Py::Int(2))))) } } } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    break;
    }
    }
    return word.clone();
    Py::None
}
pub fn _step_stems(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[text.clone()])) { let mut w: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&g_STEP_EMPTY(), &w.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&g_STEP_VAGUE(), &w.clone())) } }) {  __out.push(_stem(&[w.clone()])); } } py_set(__out) })() };
    Py::None
}
pub fn _names_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut stems: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stems") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stems"))) };
    let mut got: Py = Py::None;
    let mut tried: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when this piece of the answer is the step the person has already done.");
    let __iter1 = py_iter_or_raise(&words_of(&[part.clone()]));
    for __item in __iter1 {
    word = __item.clone();
    got = _stem(&[word.clone()]);
    let __iter2 = py_iter_or_raise(&stems.clone());
    for __item in __iter2 {
    tried = __item.clone();
    if py_truthy(&Py::Bool(py_eq(&got.clone(), &tried.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[got.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&tried.clone(), "startswith", &[got.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAME_STEP(), &slice_or_raise(&tried.clone(), &call_builtin("len", &[got.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    if py_truthy(&{ let __b = ge(&call_builtin("len", &[tried.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&got.clone(), "startswith", &[tried.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAME_STEP(), &slice_or_raise(&got.clone(), &call_builtin("len", &[tried.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } } } }) {
    return Py::Bool(true);
    }
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_move_after(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut stems: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stems") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stems"))) };
    let _ = py_str("The move left in this part once the step the person has done is taken out of it.");
    return geti(&_the_move_pieces(&[part.clone(), stems.clone()]), &Py::Int(1));
    Py::None
}
pub fn _the_move_pieces(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut stems: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "stems") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: stems"))) };
    let mut best: Py = Py::None;
    let mut hits: Py = Py::None;
    let mut i: Py = Py::None;
    let mut last: Py = Py::None;
    let mut left: Py = Py::None;
    let mut p: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut pieces: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tried: Py = Py::None;
    let mut w: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The piece that names the step they did, and the moves left after it in the same part.");
    if py_truthy(&mcall(&g_A_CONDITION(), "match", &[mcall(&call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])])) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    pieces = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_MOVES(), "split", &[call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[py_str(" .,;:")])) {  __out.push(mcall(&p.clone(), "strip", &[py_str(" .,;:")])); } } py_list(__out) })() };
    let __unpk1 = py_unpack(&py_tuple(vec![Py::Int(0), Py::None]), 2, None);
    best = __unpk1[0].clone();
    last = __unpk1[1].clone();
    let __iter2 = py_iter_or_raise(&call_builtin("enumerate", &[pieces.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter2 {
    let __unpk3 = py_unpack(&__item.clone(), 2, None);
    i = __unpk3[0].clone();
    piece = __unpk3[1].clone();
    words = call_builtin("set", &[words_of(&[piece.clone()])]).unwrap_or_else(|e| exc::raise(e));
    hits = call_builtin("sum", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stems.clone()) { let mut tried: Py = __item.clone(); if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words.clone()) { let mut w: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&w.clone(), &tried.clone())); if py_truthy(&__b) { __b } else { { let __b = { let __b = ge(&call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&tried.clone(), "startswith", &[w.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAME_STEP(), &slice_or_raise(&tried.clone(), &call_builtin("len", &[w.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } } } }; if py_truthy(&__b) { __b } else { { let __b = ge(&call_builtin("len", &[tried.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5)); if !py_truthy(&__b) { __b } else { { let __b = mcall(&w.clone(), "startswith", &[tried.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SAME_STEP(), &slice_or_raise(&w.clone(), &call_builtin("len", &[tried.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None))) } } } } } } } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(Py::Int(1)); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = hits.clone(); if !py_truthy(&__b) { __b } else { ge(&hits.clone(), &best.clone()) } }) {
    let __unpk4 = py_unpack(&py_tuple(vec![hits.clone(), i.clone()]), 2, None);
    best = __unpk4[0].clone();
    last = __unpk4[1].clone();
    }
    }
    if py_truthy(&Py::Bool(py_is(&last.clone(), &Py::None))) {
    return py_tuple(vec![py_str(""), py_str("")]);
    }
    rest = slice_or_raise(&pieces.clone(), &add(&last.clone(), &Py::Int(1)), &Py::None, &Py::None);
    if py_truthy(&Py::Bool(!py_truthy(&rest.clone()))) {
    return py_tuple(vec![geti(&pieces.clone(), &last.clone()), py_str("")]);
    }
    left = mcall(&py_str(" and "), "join", &[rest.clone()]);
    if py_truthy(&{ let __b = mcall(&g_PRONOUN_TAIL(), "match", &[left.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TAIL_WORD(), "match", &[left.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_a_step(&[left.clone()]))) } } } }) {
    return py_tuple(vec![geti(&pieces.clone(), &last.clone()), py_str("")]);
    }
    return py_tuple(vec![geti(&pieces.clone(), &last.clone()), if py_truthy(&_opens_a_step(&[left.clone()])) { left.clone() } else { py_str("") }]);
    Py::None
}
pub fn _clauses(args: &[Py]) -> Py {
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
    let mut chunk: Py = Py::None;
    let mut depth: Py = Py::None;
    let mut out: Py = Py::None;
    let mut p: Py = Py::None;
    let mut piece: Py = Py::None;
    let _ = py_str("The move cut where one thing to do ends and the next begins.");
    let __unpk1 = py_unpack(&py_tuple(vec![py_list(vec![]), py_list(vec![]), Py::Int(0)]), 3, None);
    out = __unpk1[0].clone();
    piece = __unpk1[1].clone();
    depth = __unpk1[2].clone();
    let __iter2 = py_iter_or_raise(&mcall(&g_CLAUSE(), "split", &[mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])])]));
    for __item in __iter2 {
    chunk = __item.clone();
    let __aug3 = add(&depth.clone(), &sub(&mcall(&chunk.clone(), "count", &[py_str("(")]), &mcall(&chunk.clone(), "count", &[py_str(")")])));
    depth = __aug3;
    if py_truthy(&{ let __b = piece.clone(); if !py_truthy(&__b) { __b } else { gt(&depth.clone(), &Py::Int(0)) } }) {
    let _ = mcall(&piece.clone(), "append", &[chunk.clone()]);
    continue;
    }
    if py_truthy(&piece.clone()) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(", "), "join", &[piece.clone()])]);
    }
    piece = py_list(vec![chunk.clone()]);
    }
    if py_truthy(&piece.clone()) {
    let _ = mcall(&out.clone(), "append", &[mcall(&py_str(", "), "join", &[piece.clone()])]);
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&out.clone()) { let mut p: Py = __item.clone(); if py_truthy(&mcall(&p.clone(), "strip", &[py_str(" .,;:")])) {  __out.push(mcall(&p.clone(), "strip", &[py_str(" .,;:")])); } } py_list(__out) })() };
    Py::None
}
pub fn _a_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut piece: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "piece") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: piece"))) };
    let mut comma: Py = Py::None;
    let mut head: Py = Py::None;
    let mut step: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("One clause as a step: the next thing to do, not the paragraph it came out of.");
    piece = mcall(&piece.clone(), "strip", &[py_str(" .,;:")]);
    if py_truthy(&gt(&call_builtin("len", &[mcall(&piece.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS())) {
    head = mcall(&geti(&mcall(&piece.clone(), "split", &[py_str(": ")]), &Py::Int(0)), "strip", &[py_str(" .,;:")]);
    if py_truthy(&{ let __b = head.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_eq(&head.clone(), &piece.clone())) } }) {
    piece = head.clone();
    }
    }
    if py_truthy(&{ let __b = piece.clone(); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&piece.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS()) } }) {
    let __unpk1 = py_unpack(&mcall(&piece.clone(), "partition", &[py_str(", ")]), 3, None);
    head = __unpk1[0].clone();
    comma = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = comma.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_CONDITION(), "match", &[head.clone()]) } }) {
    step = mcall(&geti(&mcall(&tail.clone(), "split", &[py_str(", ")]), &Py::Int(0)), "strip", &[py_str(" .,;:")]);
    if py_truthy(&{ let __b = step.clone(); if !py_truthy(&__b) { __b } else { _opens_a_step(&[step.clone()]) } }) {
    piece = add(&add(&head.clone(), &py_str(", ")), &step.clone());
    }
    }
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&piece.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[mcall(&piece.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_STEP_WORDS()); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[piece.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(180)) } } } }) {
    return py_str("");
    }
    return piece.clone();
    Py::None
}
pub fn _opens_a_step(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut comma: Py = Py::None;
    let mut first: Py = Py::None;
    let mut head: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut tail: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the piece opens on a doing word, which is what a step looks like.");
    let mut IMPERATIVES: Py = crate::gen::compose::verdict::g_IMPERATIVES();
    words = words_of(&[part.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    first = geti(&words.clone(), &Py::Int(0));
    if py_truthy(&{ let __b = Py::Bool(py_contains(&IMPERATIVES.clone(), &first.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MORE_STEP_VERBS(), &first.clone())) } }) {
    return Py::Bool(true);
    }
    let __unpk1 = py_unpack(&mcall(&mcall(&call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "partition", &[py_str(", ")]), 3, None);
    head = __unpk1[0].clone();
    comma = __unpk1[1].clone();
    tail = __unpk1[2].clone();
    if py_truthy(&{ let __b = comma.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_A_CONDITION(), "match", &[head.clone()]) } }) {
    rest = words_of(&[tail.clone()]);
    if py_truthy(&{ let __b = rest.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&IMPERATIVES.clone(), &geti(&rest.clone(), &Py::Int(0)))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_MORE_STEP_VERBS(), &geti(&rest.clone(), &Py::Int(0)))) } } } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_step_after(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut last: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The next step of the answer just given, in its own words, or \"\".\n\n    Only an answer from the knowledge base is walked: the reply before this line has to hold the\n    steps, and a chat or a safety answer does not. The step named has to be in that answer, so a\n    line about something else entirely is left to the modules that can answer it.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_truthy(&turns.clone()))) {
    return py_str("");
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    if py_truthy(&Py::Bool(!py_eq(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("module")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), &py_str("answers")))) {
    return py_str("");
    }
    return _walk_answer(&[call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), thing.clone()]);
    Py::None
}
pub fn _the_steps_of(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The answer cut into the pieces that could be a step, in the order it was written.");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_list(vec![]);
    }
    parts = py_list(vec![]);
    let __iter1 = py_iter_or_raise(&mcall(&gattr(&repeat.clone(), "SENTENCE"), "split", &[mcall(&call_builtin("str", &[said.clone()]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]));
    for __item in __iter1 {
    sentence = __item.clone();
    let __iter2 = py_iter_or_raise(&_clauses(&[sentence.clone()]));
    for __item in __iter2 {
    clause = __item.clone();
    piece = _a_step(&[clause.clone()]);
    if py_truthy(&{ let __b = piece.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&parts.clone(), &piece.clone())) } }) {
    let _ = mcall(&parts.clone(), "append", &[piece.clone()]);
    }
    }
    }
    return parts.clone();
    Py::None
}
pub fn _list_heads(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut colon: Py = Py::None;
    let mut head: Py = Py::None;
    let mut heads: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut rest: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("The pieces that only introduce the list that follows them.\n\n    \"Change a few things you can keep doing rather than going on a diet: fill half the plate with\n    vegetables, ...\" opens on a doing word, so the walk read the heading as a step of its own and\n    handed it back as the next thing to do - a sentence with nothing in it to act on, given to a\n    person who had just said the walking was already part of their day. A sentence that ends at a\n    colon is a heading, not a step (`_the_steps_of` cuts the items of the list into parts of their\n    own, which are the steps).\n    ");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    heads = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&mcall(&gattr(&repeat.clone(), "SENTENCE"), "split", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]));
    for __item in __iter1 {
    sentence = __item.clone();
    let __unpk2 = py_unpack(&mcall(&sentence.clone(), "partition", &[py_str(":")]), 3, None);
    head = __unpk2[0].clone();
    colon = __unpk2[1].clone();
    rest = __unpk2[2].clone();
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&colon.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&rest.clone(), "strip", &[]))) } }) {
    continue;
    }
    let __iter3 = py_iter_or_raise(&_clauses(&[head.clone()]));
    for __item in __iter3 {
    clause = __item.clone();
    piece = _a_step(&[clause.clone()]);
    if py_truthy(&piece.clone()) {
    let _ = mcall(&heads.clone(), "add", &[_flat(&[piece.clone()])]);
    }
    }
    }
    return heads.clone();
    Py::None
}
pub fn _object_stems(args: &[Py]) -> Py {
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
    let mut w: Py = Py::None;
    let _ = py_str("The words of a step that name the thing it is done to, leaving out the doing word.");
    out = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&words_of(&[text.clone()]));
    for __item in __iter1 {
    w = __item.clone();
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_STEP_EMPTY(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_STEP_VAGUE(), &w.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_STEP_TIMING(), &w.clone())) } } } }) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_PAST_IRREGULAR(), &w.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_SHORT_PAST(), &w.clone())); if py_truthy(&__b) { __b } else { mcall(&g_PAST_STEP_ED(), "match", &[w.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_DOING_WORD(), "match", &[w.clone()])) {
    continue;
    }
    let _ = mcall(&out.clone(), "add", &[_stem(&[w.clone()])]);
    }
    return out.clone();
    Py::None
}
pub fn in_the_answer(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut thing: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut said: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "said") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: said"))) };
    let mut objects: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut stems: Py = Py::None;
    let _ = py_str("True when one of these pieces names what the person says they have done.");
    stems = _step_stems(&[thing.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&stems.clone()))) {
    return Py::Bool(false);
    }
    parts = _the_steps_of(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(_names_it(&[part.clone(), stems.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    objects = _object_stems(&[thing.clone()]);
    if py_truthy(&objects.clone()) {
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(_names_it(&[part.clone(), objects.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(true);
    Py::None
}
pub fn _flat(args: &[Py]) -> Py {
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
    return mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,;:")]);
    Py::None
}
pub fn _the_first_step_left(args: &[Py]) -> Py {
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
    let mut seen: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "seen") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: seen"))) };
    let mut after: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { py_str("") };
    let mut flat: Py = Py::None;
    let mut heads: Py = Py::None;
    let mut i: Py = Py::None;
    let mut joined: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut was: Py = Py::None;
    let mut where_: Py = Py::None;
    let _ = py_str("The answer's first step that has not been handed back yet, or \"\".\n\n    `after` is the step the bot has just given: asked what comes next, only the parts after it\n    are the answer. Without it the walk wraps back to the top of the answer and hands over the\n    part they have already been given - \"then what\" after \"pull the gunk out\" got \"Start with a\n    plunger over the plughole\" again instead of the kettle of hot water that comes after it.\n    Asked for the next part twice on an answer with no step after the ones given, the walk has\n    nothing left to hand over and the line fell to a pool that repeated the whole first sentence\n    of the answer with a lead on it. The answer's own step one is what a person who asks again is\n    owed, said once, without the step they have done inside it.\n    ");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    heads = _list_heads(&[said.clone()]);
    parts = _the_steps_of(&[said.clone()]);
    if py_truthy(&after.clone()) {
    where_ = _flat(&[after.clone()]);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    part = __unpk2[1].clone();
    flat = _flat(&[part.clone()]);
    if py_truthy(&{ let __b = where_.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_eq(&flat.clone(), &where_.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&flat.clone(), &where_.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&where_.clone(), &flat.clone())) } } } } } }) {
    parts = slice_or_raise(&parts.clone(), &add(&i.clone(), &Py::Int(1)), &Py::None, &Py::None);
    break;
    }
    }
    }
    let __iter3 = py_iter_or_raise(&parts.clone());
    for __item in __iter3 {
    part = __item.clone();
    flat = _flat(&[part.clone()]);
    if py_truthy(&Py::Bool(py_contains(&heads.clone(), &flat.clone()))) {
    continue;
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&seen.clone()) { let mut was: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&flat.clone(), &was.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &was.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    continue;
    }
    if py_truthy(&{ let __b = _opens_a_step(&[part.clone()]); if !py_truthy(&__b) { __b } else { mcall(&repeat.clone(), "_says_what_to_do", &[part.clone()]) } }) {
    return part.clone();
    }
    joined = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("^(?:and|but|or)\\s+")); __args.push(py_str("")); __args.push(flat.clone()); __kws.extend(vec![("flags".to_string(), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| stdlib_call("re", "sub", args).unwrap_or_else(|e| exc::raise(e)))(&__args) };
    if py_truthy(&{ let __b = mcall(&g_A_CONDITION(), "match", &[joined.clone()]); if !py_truthy(&__b) { __b } else { mcall(&repeat.clone(), "_says_what_to_do", &[joined.clone()]) } }) {
    if py_truthy(&Py::Bool(!py_eq(&joined.clone(), &flat.clone()))) {
    joined = add(&mcall(&slice_or_raise(&joined.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&joined.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return joined.clone();
    }
    }
    if py_truthy(&after.clone()) {
    let __iter4 = py_iter_or_raise(&parts.clone());
    for __item in __iter4 {
    part = __item.clone();
    flat = _flat(&[part.clone()]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&heads.clone(), &flat.clone())); if py_truthy(&__b) { __b } else { call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&seen.clone()) { let mut was: Py = __item.clone(); __out.push({ let __b = Py::Bool(py_eq(&flat.clone(), &was.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&flat.clone(), &was.clone())) } }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } }) {
    continue;
    }
    if py_truthy(&mcall(&gattr(&repeat.clone(), "ASKING_THEM"), "search", &[part.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&repeat.clone(), "_says_what_to_do", &[part.clone()])) {
    return part.clone();
    }
    }
    }
    return py_str("");
    Py::None
}
pub fn _with_its_tail(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut part: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "part") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: part"))) };
    let mut parts: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts"))) };
    let mut i: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "i") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: i"))) };
    let mut nxt: Py = Py::None;
    let _ = py_str("The step and the part after it, when the answer wrote them as one instruction.\n\n    \"Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not\n    come back\" is one sentence of the answer, and the walk handed over the first half alone. The\n    person who had tried the plunger was told to skip the cleaner and lost the hair collector.\n    Only a part that itself says what to do is joined, and only into one sentence.\n    ");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    if py_truthy(&Py::Bool(!py_eq(&add(&i.clone(), &Py::Int(2)), &call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e))))) {
    return part.clone();
    }
    nxt = geti(&parts.clone(), &add(&i.clone(), &Py::Int(1)));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&repeat.clone(), "_says_what_to_do", &[nxt.clone()]))); if py_truthy(&__b) { __b } else { mcall(&gattr(&repeat.clone(), "ASKING_THEM"), "search", &[nxt.clone()]) } }) {
    return part.clone();
    }
    if py_truthy(&{ let __b = mcall(&g_A_CONDITION(), "match", &[nxt.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&_list_heads(&[nxt.clone()]), &_flat(&[nxt.clone()]))) } }) {
    return part.clone();
    }
    if py_truthy(&gt(&add(&call_builtin("len", &[words_of(&[part.clone()])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[words_of(&[nxt.clone()])]).unwrap_or_else(|e| exc::raise(e))), &Py::Int(40))) {
    return part.clone();
    }
    return add(&add(&add(&mcall(&part.clone(), "rstrip", &[py_str(" .,;:")]), &py_str(", and ")), &mcall(&slice_or_raise(&nxt.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[])), &slice_or_raise(&nxt.clone(), &Py::Int(1), &Py::None, &Py::None));
    Py::None
}
pub fn _the_advice_after(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut parts: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "parts") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: parts"))) };
    let mut part: Py = Py::None;
    let _ = py_str("The answer's own line on keeping the thing right, among the parts still to be used.");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    let mut topics: Py = crate::gen::memory::topics::module_obj();
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    part = __item.clone();
    if py_truthy(&lt(&call_builtin("len", &[words_of(&[part.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(5))) {
    continue;
    }
    if py_truthy(&mcall(&gattr(&repeat.clone(), "ASKING_THEM"), "search", &[part.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&gattr(&topics.clone(), "RED_FLAG"), "match", &[part.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_PRONOUN_TAIL(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TAIL_WORD(), "match", &[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_a_step(&[part.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_KEEPS_IT_RIGHT(), "search", &[part.clone()])) {
    return part.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn _walk_answer(args: &[Py]) -> Py {
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
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut wrap: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "wrap") { v.clone() }
    else { Py::Bool(true) };
    let mut advice: Py = Py::None;
    let mut heads: Py = Py::None;
    let mut i: Py = Py::None;
    let mut move_: Py = Py::None;
    let mut named: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut start: Py = Py::None;
    let mut stems: Py = Py::None;
    let mut theirs_is_a_step: Py = Py::None;
    let _ = py_str("The first step of this answer after the part that names what they have done, or \"\".\n\n    `wrap` is off for the ask after a step the bot has already handed over: two steps on, the\n    answer's first step is one the person has had or has done (\"what else can i try\" after the\n    plunger was handed back is the kettle, not the plunger again).\n    ");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    let mut topics: Py = crate::gen::memory::topics::module_obj();
    parts = _the_steps_of(&[said.clone()]);
    stems = _step_stems(&[thing.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&parts.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&stems.clone())) } }) {
    return py_str("");
    }
    heads = _list_heads(&[said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); __out.push(_names_it(&[part.clone(), stems.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_str("");
    }
    start = Py::Int(0);
    theirs_is_a_step = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("enumerate", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    let __unpk2 = py_unpack(&__item.clone(), 2, None);
    i = __unpk2[0].clone();
    part = __unpk2[1].clone();
    if py_truthy(&_names_it(&[part.clone(), stems.clone()])) {
    let __unpk3 = py_unpack(&_the_move_pieces(&[part.clone(), stems.clone()]), 2, None);
    named = __unpk3[0].clone();
    move_ = __unpk3[1].clone();
    if py_truthy(&{ let __b = move_.clone(); if !py_truthy(&__b) { __b } else { mcall(&repeat.clone(), "_says_what_to_do", &[move_.clone()]) } }) {
    return move_.clone();
    }
    theirs_is_a_step = { let __b = _opens_a_step(&[part.clone()]); if py_truthy(&__b) { __b } else { _opens_a_step(&[named.clone()]) } };
    start = add(&i.clone(), &Py::Int(1));
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&theirs_is_a_step.clone()))) {
    return if py_truthy(&Py::Bool(!py_truthy(&wrap.clone()))) { _the_advice_after(&[slice_or_raise(&parts.clone(), &start.clone(), &Py::None, &Py::None)]) } else { py_str("") };
    }
    let __iter4 = py_iter_or_raise(&call_builtin("enumerate", &[slice_or_raise(&parts.clone(), &start.clone(), &Py::None, &Py::None), start.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter4 {
    let __unpk5 = py_unpack(&__item.clone(), 2, None);
    i = __unpk5[0].clone();
    part = __unpk5[1].clone();
    if py_truthy(&_names_it(&[part.clone(), stems.clone()])) {
    move_ = _the_move_after(&[part.clone(), stems.clone()]);
    if py_truthy(&{ let __b = move_.clone(); if !py_truthy(&__b) { __b } else { mcall(&repeat.clone(), "_says_what_to_do", &[move_.clone()]) } }) {
    return move_.clone();
    }
    continue;
    }
    if py_truthy(&mcall(&gattr(&repeat.clone(), "ASKING_THEM"), "search", &[part.clone()])) {
    continue;
    }
    if py_truthy(&mcall(&gattr(&topics.clone(), "RED_FLAG"), "match", &[part.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = mcall(&g_PRONOUN_TAIL(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TAIL_WORD(), "match", &[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_a_step(&[part.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&heads.clone(), &_flat(&[part.clone()])))) {
    continue;
    }
    if py_truthy(&_opens_a_step(&[part.clone()])) {
    return _with_its_tail(&[part.clone(), parts.clone(), i.clone()]);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&wrap.clone()))) {
    advice = _the_advice_after(&[slice_or_raise(&parts.clone(), &start.clone(), &Py::None, &Py::None)]);
    if py_truthy(&advice.clone()) {
    return advice.clone();
    }
    return py_str("");
    }
    let __iter6 = py_iter_or_raise(&slice_or_raise(&parts.clone(), &Py::None, &call_builtin("max", &[sub(&start.clone(), &Py::Int(1)), Py::Int(0)]).unwrap_or_else(|e| exc::raise(e)), &Py::None));
    for __item in __iter6 {
    part = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_PRONOUN_TAIL(), "match", &[part.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_TAIL_WORD(), "match", &[part.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_a_step(&[part.clone()]))) } } } }) {
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&heads.clone(), &_flat(&[part.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = _opens_a_step(&[part.clone()]); if !py_truthy(&__b) { __b } else { mcall(&repeat.clone(), "_says_what_to_do", &[part.clone()]) } }) {
    return part.clone();
    }
    }
    return py_str("");
    Py::None
}
pub fn reads_as_a_step_done(args: &[Py]) -> Py {
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
    let mut _shape: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut walk: Py = Py::None;
    let _ = py_str("True when the line says a step of the answer just given is done, whatever comes next.\n\n    `next_step_pool` hands over the step after it; this says only that the line is a report on\n    the answer, so a fact entry about a different thing cannot take the turn. Both are needed:\n    an answer whose last step is the one they did has no step after it to hand over, and the line\n    still is not a question about the world.\n    ");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&memory.clone())) } }) {
    return Py::Bool(false);
    }
    let __unpk1 = py_unpack(&_read_step_tried(&[text.clone()]), 2, None);
    thing = __unpk1[0].clone();
    _shape = __unpk1[1].clone();
    said = _the_last_answer(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    thing = _the_past_step(&[text.clone(), said.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::Bool(false);
    }
    } else {
    if py_truthy(&{ let __b = said.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&in_the_answer(&[thing.clone(), said.clone()]))) } }) {
    return Py::Bool(false);
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    walk = _walk_answer(&[said.clone(), thing.clone()]);
    if py_truthy(&walk.clone()) {
    return Py::Bool(true);
    }
    return le(&call_builtin("len", &[_the_steps_of(&[said.clone()])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1));
    Py::None
}
pub fn _as_a_lead(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut lead: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "lead") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: lead"))) };
    let mut thing: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "thing") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: thing"))) };
    let mut step: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "step") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: step"))) };
    let mut first: Py = Py::None;
    let mut reply: Py = Py::None;
    let _ = py_str("The lead filled in: the step opens lower down when it follows a comma.");
    thing = crate::gen::memory::you(&[thing.clone()]);
    step = mcall(&step.clone(), "strip", &[py_str(" .,;:")]);
    step = mcall(&stdlib_call("re", "sub", &[py_str(",?\\s*\\bthen$"), py_str(""), step.clone()]).unwrap_or_else(|e| exc::raise(e)), "rstrip", &[py_str(" ,;:")]);
    first = if py_truthy(&mcall(&step.clone(), "split", &[])) { geti(&mcall(&step.clone(), "split", &[]), &Py::Int(0)) } else { py_str("") };
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("I"), py_str("I'm"), py_str("I've"), py_str("I'd")]), &first.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&first.clone(), "isupper", &[]); if !py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[first.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)) } } } }))) {
    step = add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    reply = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("thing".to_string(), thing.clone()), ("Thing".to_string(), add(&mcall(&slice_or_raise(&thing.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&thing.clone(), &Py::Int(1), &Py::None, &Py::None))), ("step".to_string(), step.clone()), ("Step".to_string(), add(&mcall(&slice_or_raise(&step.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&step.clone(), &Py::Int(1), &Py::None, &Py::None)))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) };
    return add(&mcall(&reply.clone(), "rstrip", &[py_str(" .")]), &py_str("."));
    Py::None
}
pub fn next_step_pool(args: &[Py]) -> Py {
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
    let mut as_a_habit: Py = Py::None;
    let mut first_word: Py = Py::None;
    let mut leads: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said_as_the_past: Py = Py::None;
    let mut says_the_doing: Py = Py::None;
    let mut shape: Py = Py::None;
    let mut step: Py = Py::None;
    let mut t: Py = Py::None;
    let mut thing: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The step after the one the person has tried, or None.\n\n    A pool of one, so the caller picks it with the pickers it already uses and nothing is said\n    twice. None leaves the line to the report below, which hands over the trade.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&memory.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&_read_step_tried(&[text.clone()]), 2, None);
    thing = __unpk1[0].clone();
    shape = __unpk1[1].clone();
    said_as_the_past = Py::Bool(false);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    thing = _the_past_step(&[text.clone(), _the_last_answer(&[memory.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&thing.clone()))) {
    return Py::None;
    }
    said_as_the_past = Py::Bool(true);
    } else {
    if py_truthy(&{ let __b = _the_last_answer(&[memory.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&in_the_answer(&[thing.clone(), _the_last_answer(&[memory.clone()])]))) } }) {
    return Py::None;
    }
    }
    if py_truthy(&buying(&[memory.clone()])) {
    return Py::None;
    }
    step = _the_step_after(&[thing.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&step.clone()))) {
    return Py::None;
    }
    let mut pick: Py = py_func("pick", |args: &[Py]| crate::gen::compose::pick(args));
    says_the_doing = { let __b = call_builtin("bool", &[mcall(&g_DOING_WORD(), "match", &[thing.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&shape.clone(), &g_STEP_TRIED())) } };
    first_word = mcall(&geti(&{ let __b = mcall(&thing.clone(), "split", &[]); if py_truthy(&__b) { __b } else { py_list(vec![py_str("")]) } }, &Py::Int(0)), "lower", &[]);
    as_a_habit = { let __b = says_the_doing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_PAST_WORD(), "match", &[first_word.clone()]); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_PAST_IRREGULAR(), &first_word.clone())) } })) } };
    if py_truthy(&said_as_the_past.clone()) {
    leads = g_DONE_LEADS();
    } else {
    if py_truthy(&as_a_habit.clone()) {
    leads = g_PRESENT_LEADS();
    } else {
    leads = if py_truthy(&says_the_doing.clone()) { g_ALREADY_LEADS() } else { g_STEP_LEADS() };
    }
    }
    reply = _as_a_lead(&[py_call(&pick, &[leads.clone(), memory.clone(), text.clone()]), thing.clone(), step.clone()]);
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_step"), call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_text"), step.clone());
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_seen"), add(&call_builtin("list", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("handed_back_seen")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e)), &py_list(vec![_flat(&[step.clone()])])));
    if py_truthy(&turns.clone()) {
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_answer"), call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)));
    }
    return py_list(vec![reply.clone()]);
    Py::None
}
pub fn _asks_for_the_next_part(args: &[Py]) -> Py {
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
    let mut word: Py = Py::None;
    let _ = py_str("True for \"and then what\" / \"what else can i try\", and not for a line naming a thing.");
    let mut repeat: Py = crate::gen::memory::repeat::module_obj();
    line = mcall(&mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[]), "strip", &[py_str(".,!")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(7)) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&gattr(&repeat.clone(), "NEXT_ASK"), "match", &[line.clone()])))) {
    return Py::Bool(false);
    }
    return call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&words_of(&[line.clone()])) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_NEXT_ASK_WORDS(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn next_step_ask_pool(args: &[Py]) -> Py {
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
    let mut more: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut seen: Py = Py::None;
    let mut step: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The part of the answer after the step the bot just handed back, or None.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&memory.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_asks_for_the_next_part(&[text.clone()]))) } }) {
    return Py::None;
    }
    step = mcall(&call_builtin("str", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("handed_back_text")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    said = call_builtin("str", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("handed_back_answer")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&step.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&said.clone())) } }) {
    return Py::None;
    }
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    if py_truthy(&Py::Bool(!py_eq(&mcall(&memory.clone(), "get", &[py_str("handed_back_step")]), &sub(&call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1))))) {
    return Py::None;
    }
    seen = call_builtin("set", &[{ let __b = mcall(&memory.clone(), "get", &[py_str("handed_back_seen")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }]).unwrap_or_else(|e| exc::raise(e));
    more = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(step.clone()); __kws.extend(vec![("wrap".to_string(), Py::Bool(false))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _walk_answer(args))(&__args) };
    if py_truthy(&Py::Bool(!py_truthy(&more.clone()))) {
    more = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __args.push(seen.clone()); __kws.extend(vec![("after".to_string(), step.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _the_first_step_left(args))(&__args) };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&more.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&py_str(" "), "join", &[mcall(&mcall(&more.clone(), "lower", &[]), "split", &[])]), &mcall(&py_str(" "), "join", &[mcall(&mcall(&step.clone(), "lower", &[]), "split", &[])]))) } }) {
    return Py::None;
    }
    let mut pick: Py = py_func("pick", |args: &[Py]| crate::gen::compose::pick(args));
    reply = _as_a_lead(&[py_call(&pick, &[g_NEXT_LEADS(), memory.clone(), text.clone()]), py_str(""), more.clone()]);
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_step"), call_builtin("len", &[turns.clone()]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_text"), more.clone());
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_seen"), call_builtin("list", &[bitor(&seen.clone(), &py_set(vec![_flat(&[more.clone()])]))]).unwrap_or_else(|e| exc::raise(e)));
    let _ = py_setitem(&memory.clone(), &py_str("handed_back_answer"), said.clone());
    return py_list(vec![reply.clone()]);
    Py::None
}
pub fn _somebody_is_the_subject(args: &[Py]) -> Py {
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
    let mut names: Py = Py::None;
    let mut topic: Py = Py::None;
    let mut user: Py = Py::None;
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the chat is about a person or an animal rather than the person's own thing.");
    let mut facts: Py = crate::gen::memory::facts::module_obj();
    let mut named: Py = crate::gen::memory::named::module_obj();
    topic = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("topic")]);
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[topic.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    words = call_builtin("set", &[words_of(&[{ let __b = mcall(&topic.clone(), "get", &[py_str("text")]); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    let __aug1 = bitor(&words.clone(), &{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&topic.clone(), "get", &[py_str("words")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } }) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(mcall(&call_builtin("str", &[word.clone()]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])); } } py_set(__out) })() });
    words = __aug1;
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = bitand(&words.clone(), &call_builtin("set", &[gattr(&facts.clone(), "KIN")]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { { let __b = bitand(&words.clone(), &call_builtin("set", &[gattr(&named.clone(), "PEOPLE_WORDS")]).unwrap_or_else(|e| exc::raise(e))); if py_truthy(&__b) { __b } else { bitand(&words.clone(), &call_builtin("set", &[gattr(&facts.clone(), "PET_ANIMALS")]).unwrap_or_else(|e| exc::raise(e))) } } } }) {
    return Py::Bool(true);
    }
    user = { let __b = mcall(&{ let __b = memory.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_dict(vec![]) } };
    names = call_builtin("set", &[]).unwrap_or_else(|e| exc::raise(e));
    let __iter2 = py_iter_or_raise(&{ let __b = mcall(&user.clone(), "get", &[py_str("pets")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter2 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("name")]) } }) {
    let _ = mcall(&names.clone(), "add", &[mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    }
    }
    let __iter3 = py_iter_or_raise(&{ let __b = mcall(&user.clone(), "get", &[py_str("others")]); if py_truthy(&__b) { __b } else { py_tuple(vec![]) } });
    for __item in __iter3 {
    entry = __item.clone();
    if py_truthy(&{ let __b = call_builtin("isinstance", &[entry.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&entry.clone(), "get", &[py_str("name")]) } }) {
    let _ = mcall(&names.clone(), "add", &[mcall(&call_builtin("str", &[geti(&entry.clone(), &py_str("name"))]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]);
    } else {
    if py_truthy(&call_builtin("isinstance", &[entry.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {
    let _ = mcall(&names.clone(), "add", &[mcall(&entry.clone(), "lower", &[])]);
    }
    }
    }
    return call_builtin("bool", &[bitand(&words.clone(), &names.clone())]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn still_tail(args: &[Py]) -> Py {
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
    let mut line: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut state: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The thing said again with \"still\" on it, or \"\".\n\n    Two readings, both of them the chat's rather than the line's:\n\n    * a state the chat has already named, which `the_still_state` reads - \"still cold\" after \"my\n      shower is cold\", \"it still clicks\" after \"the car cranks but will not start\";\n    * the thing said whole, with a word that only points at it: \"still there\" after a headache is\n      the headache, and on its own it is the bot being asked whether it is there, which is why\n      `still_going_on` wants an answer behind it.\n\n    `the_still_state` drops a body and a feeling on purpose, and it drops the point-at-it words,\n    so both are read here. What comes back is only used to keep a card for one word of the line\n    from taking the turn away from the thing the chat is on.\n    ");
    state = the_still_state(&[text.clone(), memory.clone()]);
    if py_truthy(&state.clone()) {
    return state.clone();
    }
    line = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&line.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&line.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)) } }) {
    return py_str("");
    }
    match_ = mcall(&g_STILL_ONLY(), "match", &[line.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&match_.clone()))) {
    return py_str("");
    }
    tail = mcall(&mcall(&match_.clone(), "group", &[py_str("tail")]), "strip", &[]);
    if py_truthy(&Py::Bool(!py_contains(&g_POINTS_AT_IT(), &tail.clone()))) {
    return py_str("");
    }
    if py_truthy(&_somebody_is_the_subject(&[memory.clone()])) {
    return py_str("");
    }
    return tail.clone();
    Py::None
}
pub fn a_card_is_not_the_answer(args: &[Py]) -> Py {
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
    let mut tail: Py = Py::None;
    let _ = py_str("True when the card one word of this line belongs to must not answer it.\n\n    `bot/modules/answers.py` asks this before it reads anything, because a card is opened by a\n    single word and a line that is only the thing again carries that one word: \"still cold\" after\n    \"my shower is cold\" found the common cold, \"still freezing\" after \"my laptop keeps freezing\"\n    found the freezer. The thing the chat is on keeps the turn instead.\n\n    A body and a feeling are the exception: \"still tired\" after the tiredness answer wants that\n    answer's own advice, and the card for it is not a different subject. So a card is stepped back\n    from only where the tail points at the thing, or where the chat is holding a thing at all.\n    ");
    tail = still_tail(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&tail.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(py_contains(&g_POINTS_AT_IT(), &tail.clone()))) {
    return after_an_answer(&[memory.clone()]);
    }
    return { let __b = names_a_thing(&[memory.clone()]); if py_truthy(&__b) { __b } else { after_an_answer(&[memory.clone()]) } };
    Py::None
}
pub fn still_going_on(args: &[Py]) -> Py {
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
    let mut pool: Py = Py::None;
    let mut tail: Py = Py::None;
    let _ = py_str("The pool for a line that is only the thing again with \"still\" on it, or None.\n\n    `pool_for` already reads these lines, but only the modules that ask it do: a line of one\n    word opened whatever card that word belongs to first (\"still cold\" the common cold, \"still\n    freezing\" the freezer), and the reading never reached the person. The store answers it before\n    any card is looked at, so the thing the chat is on keeps the turn.\n    ");
    tail = still_tail(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tail.clone())); if py_truthy(&__b) { __b } else { buying(&[memory.clone()]) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(py_contains(&g_POINTS_AT_IT(), &tail.clone()))) {
    if py_truthy(&Py::Bool(!py_truthy(&after_an_answer(&[memory.clone()])))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&names_a_thing(&[memory.clone()])))) {
    return g_STILL_GOING_ON();
    }
    } else {
    if py_truthy(&Py::Bool(!py_truthy(&names_a_thing(&[memory.clone()])))) {
    return Py::None;
    }
    }
    pool = pool_for(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_is(&pool.clone(), &Py::None)); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_is(&pool.clone(), &g_ASK_WHAT_TRIED())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_is(&pool.clone(), &g_SAME_STILL())) } } } }) {
    return Py::None;
    }
    return g_SAME_STILL();
    Py::None
}
pub fn reply_for(args: &[Py]) -> Py {
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
    let mut pool: Py = Py::None;
    let _ = py_str("The reply to a line that is only the thing again with \"still\" on it, or None.\n\n    Called from `bot.memory.not_this`, which is the reader the store already asks before any card\n    is looked at: the thing the chat is on keeps the turn, and no card for the one word the line\n    carries gets it. It lives here rather than in the store because the store is over the size the\n    tools take, so this reading adds nothing to it.\n    ");
    let mut pick: Py = py_func("pick", |args: &[Py]| crate::gen::compose::pick(args));
    pool = still_going_on(&[text.clone(), memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&pool.clone()))) {
    return Py::None;
    }
    return py_call(&pick, &[pool.clone(), memory.clone(), call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn pool_for(args: &[Py]) -> Py {
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
    let mut care: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut line: Py = Py::None;
    let mut made_it_worse: Py = Py::None;
    let mut more: Py = Py::None;
    let mut report: Py = Py::None;
    let mut state: Py = Py::None;
    let mut step: Py = Py::None;
    let mut trade: Py = Py::None;
    let _ = py_str("The pool the next step comes from, or None when the line is not a report on the answer.\n\n    The caller picks from it, so nothing is said twice: the picker remembers what it has used,\n    which matters here because these lines come in runs (\"tried that\" ... \"and that\").\n    ");
    care = in_care_pool(&[text.clone(), memory.clone()]);
    if py_truthy(&care.clone()) {
    return care.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&{ let __b = reads_as_not_worked(&[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = still_the_state(&[text.clone(), memory.clone()]); if py_truthy(&__b) { __b } else { still_tail(&[text.clone(), memory.clone()]) } } } })); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&after_an_answer(&[memory.clone()]))) } }) {
    step = next_step_pool(&[text.clone(), memory.clone()]);
    if py_truthy(&step.clone()) {
    return step.clone();
    }
    more = next_step_ask_pool(&[text.clone(), memory.clone()]);
    if py_truthy(&more.clone()) {
    return more.clone();
    }
    return Py::None;
    }
    report = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    made_it_worse = call_builtin("bool", &[{ let __b = mcall(&g_MADE_IT_WORSE(), "search", &[report.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_POINTS_AT_THE_THING(), "search", &[report.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = made_it_worse.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&asks_for_a_fix(&[memory.clone()]))) } }) {
    return Py::None;
    }
    if py_truthy(&buying(&[memory.clone()])) {
    return Py::None;
    }
    if py_truthy(&asked_what_its_doing(&[memory.clone()])) {
    return g_END();
    }
    trade = trade_for(&[memory.clone()]);
    if py_truthy(&trade.clone()) {
    if py_truthy(&already_named(&[trade.clone(), memory.clone()])) {
    return g_ASK_NEXT();
    }
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_LEADS()) { let mut lead: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("trade".to_string(), trade.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&lead.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    }
    state = the_still_state(&[text.clone(), memory.clone()]);
    if py_truthy(&{ let __b = state.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = asks_for_a_fix(&[memory.clone()]); if !py_truthy(&__b) { __b } else { has_evidence(&[text.clone(), memory.clone()]) } })) } }) {
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_STILL_ASK()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("state".to_string(), state.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() };
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&asks_for_a_fix(&[memory.clone()]))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&has_evidence(&[text.clone(), memory.clone()]))) } }) {
    if py_truthy(&{ let __b = still_tail(&[text.clone(), memory.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_POINTS_AT_IT(), &still_tail(&[text.clone(), memory.clone()]))); if py_truthy(&__b) { __b } else { names_a_thing(&[memory.clone()]) } } } }) {
    return g_SAME_STILL();
    }
    return Py::None;
    }
    if py_truthy(&{ let __b = made_it_worse.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = mcall(&g_NOT_WORKED(), "search", &[report.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_TRIED(), "search", &[report.clone()]) } })) } }) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&names_a_thing(&[memory.clone()])))) {
    return g_ASK_WHAT_TRIED();
    }
    return g_ASK_NEXT();
    Py::None
}
pub fn _the_person_word(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut token: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "token") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: token"))) };
    let mut word: Py = Py::None;
    let _ = py_str("The person the word names (\"mum\", \"dad\"), or \"\" for a word that names nobody.");
    word = if py_truthy(&mcall(&token.clone(), "endswith", &[py_str("'s")])) { slice_or_raise(&token.clone(), &Py::None, &neg(&Py::Int(2)), &Py::None) } else { token.clone() };
    if py_truthy(&{ let __b = mcall(&word.clone(), "endswith", &[py_str("s")]); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&bitor(&g_SHE(), &g_HE()), &slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None))) } }) {
    word = slice_or_raise(&word.clone(), &Py::None, &neg(&Py::Int(1)), &Py::None);
    }
    return if py_truthy(&{ let __b = Py::Bool(py_contains(&g_SHE(), &word.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_HE(), &word.clone())) } }) { word.clone() } else { py_str("") };
    Py::None
}
pub fn _a_report_on_a_person(args: &[Py]) -> Py {
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
    let mut after: Py = Py::None;
    let mut at: Py = Py::None;
    let mut said: Py = Py::None;
    let mut tokens: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when the line is only \"still\" and pointing words: \"still there\", \"still the same\",\n    \"still in hospital\", \"she's still in\". A line with a word of its own is a new subject and is\n    left to the readers that know it.");
    said = mcall(&py_str(" "), "join", &[words_of(&[text.clone()])]);
    tokens = mcall(&said.clone(), "split", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&tokens.clone())); if py_truthy(&__b) { __b } else { { let __b = gt(&call_builtin("len", &[tokens.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(6)); if py_truthy(&__b) { __b } else { mcall(&said.clone(), "endswith", &[py_str("?")]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_contains(&tokens.clone(), &py_str("still")))) {
    return Py::Bool(false);
    }
    at = mcall(&tokens.clone(), "index", &[py_str("still")]);
    after = slice_or_raise(&tokens.clone(), &add(&at.clone(), &Py::Int(1)), &Py::None, &Py::None);
    return { let __b = call_builtin("bool", &[after.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&after.clone()) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_STILL_ABOUT_A_PERSON(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { call_builtin("all", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&tokens.clone(), &Py::None, &at.clone(), &Py::None)) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g_POINTING(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)) } } } };
    Py::None
}
pub fn _in_care(args: &[Py]) -> Py {
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
    let mut _u: Py = Py::None;
    let mut entry_words: Py = Py::None;
    let mut l: Py = Py::None;
    let mut line: Py = Py::None;
    let mut lines: Py = Py::None;
    let mut obj: Py = Py::None;
    let mut person: Py = Py::None;
    let mut person_with: Py = Py::None;
    let mut said: Py = Py::None;
    let mut subj: Py = Py::None;
    let mut t: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut with_someone: Py = Py::None;
    let mut word: Py = Py::None;
    let mut word_with: Py = Py::None;
    let _ = py_str("(the person as \"your mum\" wants it, subject, object) for the person in a place of care, or\n    None.\n\n    The person's own lines only, newest first, and not the line being answered: the place of care\n    is something they said, not something an answer said, because a card about a broken wrist can\n    say \"hospital\" and send \"still there\" to a stranger's stay.\n    ");
    turns = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ let __b = mcall(&memory.clone(), "get", &[py_str("turns")]); if py_truthy(&__b) { __b } else { py_list(vec![]) } }) { let mut t: Py = __item.clone(); if py_truthy(&call_builtin("isinstance", &[t.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) {  __out.push(t.clone()); } } py_list(__out) })() };
    lines = slice_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&turns.clone()) { let mut t: Py = __item.clone(); __out.push(mcall(&t.clone(), "get", &[py_str("user")])); } py_list(__out) })() }, &neg(&Py::Int(6)), &Py::None, &Py::None);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&lines.clone()) { let mut l: Py = __item.clone(); if py_truthy(&l.clone()) {  __out.push(l.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    line = __item.clone();
    said = mcall(&py_str(" "), "join", &[words_of(&[line.clone()])]);
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_TRADES()) { let __unpk5328 = py_unpack(&__item.clone(), 3, None); let mut entry_words: Py = __unpk5328[0].clone(); let mut _u: Py = __unpk5328[1].clone(); let mut _u: Py = __unpk5328[2].clone(); __out.push(_names(&[call_builtin("set", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), said.clone(), entry_words.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_IN_CARE(), "search", &[said.clone()])))) {
    continue;
    }
    with_someone = stdlib_call("re", "search", &[py_str("\\bwith\\s+(?:my\\s+|our\\s+|the\\s+)?([a-z']+)"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    person_with = if py_truthy(&with_someone.clone()) { _the_person_word(&[mcall(&with_someone.clone(), "group", &[Py::Int(1)])]) } else { py_str("") };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&person_with.clone())); if !py_truthy(&__b) { __b } else { with_someone.clone() } }) {
    word_with = mcall(&mcall(&with_someone.clone(), "group", &[Py::Int(1)]), "rstrip", &[py_str("'s")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_RELATIONS(), &word_with.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&crate::gen::memory::facts::g_RELATIONS(), &mcall(&word_with.clone(), "rstrip", &[py_str("s")]))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&crate::gen::memory::facts::g_PEOPLE_AGED(), &word_with.clone())) } } } }) {
    return py_tuple(vec![py_str(""), py_str("they"), py_str("them")]);
    }
    }
    if py_truthy(&person_with.clone()) {
    return py_tuple(vec![person_with.clone(), if py_truthy(&Py::Bool(py_contains(&g_SHE(), &person_with.clone()))) { py_str("she") } else { if py_truthy(&Py::Bool(py_contains(&g_HE(), &person_with.clone()))) { py_str("he") } else { py_str("they") } }, if py_truthy(&Py::Bool(py_contains(&g_SHE(), &person_with.clone()))) { py_str("her") } else { if py_truthy(&Py::Bool(py_contains(&g_HE(), &person_with.clone()))) { py_str("him") } else { py_str("them") } }]);
    }
    if py_truthy(&{ let __b = stdlib_call("re", "search", &[py_str("\\bi(?:'?m| am)\\b.*\\bin\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { stdlib_call("re", "match", &[py_str("^im\\b"), said.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_tuple(vec![py_str(""), py_str("you"), py_str("you")]);
    }
    let __iter2 = py_iter_or_raise(&mcall(&said.clone(), "split", &[]));
    for __item in __iter2 {
    word = __item.clone();
    person = _the_person_word(&[word.clone()]);
    if py_truthy(&person.clone()) {
    subj = if py_truthy(&Py::Bool(py_contains(&g_SHE(), &person.clone()))) { py_str("she") } else { if py_truthy(&Py::Bool(py_contains(&g_HE(), &person.clone()))) { py_str("he") } else { py_str("they") } };
    obj = if py_truthy(&Py::Bool(py_eq(&subj.clone(), &py_str("she")))) { py_str("her") } else { if py_truthy(&Py::Bool(py_eq(&subj.clone(), &py_str("he")))) { py_str("him") } else { py_str("them") } };
    return py_tuple(vec![person.clone(), subj.clone(), obj.clone()]);
    }
    }
    return py_tuple(vec![py_str(""), py_str("they"), py_str("them")]);
    }
    return Py::None;
    Py::None
}
pub fn in_care_pool(args: &[Py]) -> Py {
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
    let mut filled: Py = Py::None;
    let mut found: Py = Py::None;
    let mut line: Py = Py::None;
    let mut obj: Py = Py::None;
    let mut subj: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The lines that answer a bare \"still ...\" about a person of theirs who is in a place of\n    care, or None for every other line.");
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&memory.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_a_report_on_a_person(&[text.clone()]))) } }) {
    return Py::None;
    }
    found = _in_care(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return Py::None;
    }
    let __unpk1 = py_unpack(&found.clone(), 3, None);
    word = __unpk1[0].clone();
    subj = __unpk1[1].clone();
    obj = __unpk1[2].clone();
    if py_truthy(&Py::Bool(!py_truthy(&word.clone()))) {
    if py_truthy(&Py::Bool(py_eq(&subj.clone(), &py_str("you")))) {
    return g_IN_CARE_SELF();
    }
    return g_IN_CARE_ASK_UNNAMED();
    }
    filled = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_IN_CARE_ASK()) { let mut line: Py = __item.clone(); __out.push({ let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new();  __kws.extend(vec![("subj".to_string(), subj.clone()), ("obj".to_string(), obj.clone()), ("word".to_string(), word.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&line.clone(), "format", args))(&__args) }); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&filled.clone()) { let mut line: Py = __item.clone(); if py_truthy(&{ let __b = Py::Bool(!py_contains(&line.clone(), &py_str("{"))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&line.clone(), &py_str("}"))) } }) {  __out.push(line.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)); if py_truthy(&__b) { __b } else { g_IN_CARE_ASK_UNNAMED() } };
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.memory.tried".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ACTED_REPLIES" => Some(g_ACTED_REPLIES()),
            "ACTED_SAID" => Some(g_ACTED_SAID()),
            "ALREADY_LEADS" => Some(g_ALREADY_LEADS()),
            "ANSWER_MODULES" => Some(g_ANSWER_MODULES()),
            "ASK_NEXT" => Some(g_ASK_NEXT()),
            "ASK_WHAT_TRIED" => Some(g_ASK_WHAT_TRIED()),
            "A_CONDITION" => Some(g_A_CONDITION()),
            "BUYING" => Some(g_BUYING()),
            "CLAUSE" => Some(g_CLAUSE()),
            "DOING" => Some(g_DOING()),
            "DOING_WORD" => Some(g_DOING_WORD()),
            "DONE_LEADS" => Some(g_DONE_LEADS()),
            "ELSEWHERE" => Some(g_ELSEWHERE()),
            "END" => Some(g_END()),
            "EVIDENCE" => Some(g_EVIDENCE()),
            "FAULT" => Some(g_FAULT()),
            "HE" => Some(g_HE()),
            "HEALTH" => Some(g_HEALTH()),
            "IMPROVED" => Some(g_IMPROVED()),
            "IN_CARE" => Some(g_IN_CARE()),
            "IN_CARE_ASK" => Some(g_IN_CARE_ASK()),
            "IN_CARE_ASK_UNNAMED" => Some(g_IN_CARE_ASK_UNNAMED()),
            "IN_CARE_SELF" => Some(g_IN_CARE_SELF()),
            "KEEPS_IT_RIGHT" => Some(g_KEEPS_IT_RIGHT()),
            "LEADS" => Some(g_LEADS()),
            "LEAD_IN" => Some(g_LEAD_IN()),
            "MADE_IT_WORSE" => Some(g_MADE_IT_WORSE()),
            "MARK" => Some(g_MARK()),
            "MORE_STEP_VERBS" => Some(g_MORE_STEP_VERBS()),
            "MOVES" => Some(g_MOVES()),
            "NAMED_TRY" => Some(g_NAMED_TRY()),
            "NEXT_ASK_WORDS" => Some(g_NEXT_ASK_WORDS()),
            "NEXT_LEADS" => Some(g_NEXT_LEADS()),
            "NOT_A_STATE" => Some(g_NOT_A_STATE()),
            "NOT_WORKED" => Some(g_NOT_WORKED()),
            "PARTICIPLE" => Some(g_PARTICIPLE()),
            "PAST_IRREGULAR" => Some(g_PAST_IRREGULAR()),
            "PAST_STEP" => Some(g_PAST_STEP()),
            "PAST_STEP_ED" => Some(g_PAST_STEP_ED()),
            "PAST_WORD" => Some(g_PAST_WORD()),
            "POINTING" => Some(g_POINTING()),
            "POINTS_AT_IT" => Some(g_POINTS_AT_IT()),
            "POINTS_AT_THE_THING" => Some(g_POINTS_AT_THE_THING()),
            "PRESENT_LEADS" => Some(g_PRESENT_LEADS()),
            "PROBLEM_OR_FIX" => Some(g_PROBLEM_OR_FIX()),
            "PRONOUN_TAIL" => Some(g_PRONOUN_TAIL()),
            "SAME_STEP" => Some(g_SAME_STEP()),
            "SAME_STILL" => Some(g_SAME_STILL()),
            "SAY_ADVICE" => Some(g_SAY_ADVICE()),
            "SHE" => Some(g_SHE()),
            "SHORT_PAST" => Some(g_SHORT_PAST()),
            "STEP_ALREADY" => Some(g_STEP_ALREADY()),
            "STEP_DID" => Some(g_STEP_DID()),
            "STEP_DONE" => Some(g_STEP_DONE()),
            "STEP_EMPTY" => Some(g_STEP_EMPTY()),
            "STEP_LAST" => Some(g_STEP_LAST()),
            "STEP_LEADS" => Some(g_STEP_LEADS()),
            "STEP_TAIL_NOUN" => Some(g_STEP_TAIL_NOUN()),
            "STEP_TIMING" => Some(g_STEP_TIMING()),
            "STEP_TRIED" => Some(g_STEP_TRIED()),
            "STEP_VAGUE" => Some(g_STEP_VAGUE()),
            "STEP_WORDS" => Some(g_STEP_WORDS()),
            "STILL_ABOUT_A_PERSON" => Some(g_STILL_ABOUT_A_PERSON()),
            "STILL_ASK" => Some(g_STILL_ASK()),
            "STILL_ASKS" => Some(g_STILL_ASKS()),
            "STILL_GOING_ON" => Some(g_STILL_GOING_ON()),
            "STILL_ONLY" => Some(g_STILL_ONLY()),
            "STILL_SPECIFIC" => Some(g_STILL_SPECIFIC()),
            "STILL_STATE" => Some(g_STILL_STATE()),
            "TAIL_WORD" => Some(g_TAIL_WORD()),
            "TOLD_THE_BOT" => Some(g_TOLD_THE_BOT()),
            "TOO_COMMON" => Some(g_TOO_COMMON()),
            "TRADES" => Some(g_TRADES()),
            "TRIED" => Some(g_TRIED()),
            "WORD" => Some(g_WORD()),
            "WORSE" => Some(g_WORSE()),
            "YET" => Some(g_YET()),
            "_a_report_on_a_person" => Some(py_func("_a_report_on_a_person", |args: &[Py]| _a_report_on_a_person(args))),
            "_a_step" => Some(py_func("_a_step", |args: &[Py]| _a_step(args))),
            "_as_a_lead" => Some(py_func("_as_a_lead", |args: &[Py]| _as_a_lead(args))),
            "_asks_for_the_next_part" => Some(py_func("_asks_for_the_next_part", |args: &[Py]| _asks_for_the_next_part(args))),
            "_clauses" => Some(py_func("_clauses", |args: &[Py]| _clauses(args))),
            "_flat" => Some(py_func("_flat", |args: &[Py]| _flat(args))),
            "_in_care" => Some(py_func("_in_care", |args: &[Py]| _in_care(args))),
            "_list_heads" => Some(py_func("_list_heads", |args: &[Py]| _list_heads(args))),
            "_names" => Some(py_func("_names", |args: &[Py]| _names(args))),
            "_names_it" => Some(py_func("_names_it", |args: &[Py]| _names_it(args))),
            "_object_stems" => Some(py_func("_object_stems", |args: &[Py]| _object_stems(args))),
            "_opens_a_step" => Some(py_func("_opens_a_step", |args: &[Py]| _opens_a_step(args))),
            "_read_step_tried" => Some(py_func("_read_step_tried", |args: &[Py]| _read_step_tried(args))),
            "_somebody_is_the_subject" => Some(py_func("_somebody_is_the_subject", |args: &[Py]| _somebody_is_the_subject(args))),
            "_stem" => Some(py_func("_stem", |args: &[Py]| _stem(args))),
            "_step_stems" => Some(py_func("_step_stems", |args: &[Py]| _step_stems(args))),
            "_the_advice_after" => Some(py_func("_the_advice_after", |args: &[Py]| _the_advice_after(args))),
            "_the_answered_line" => Some(py_func("_the_answered_line", |args: &[Py]| _the_answered_line(args))),
            "_the_first_step_left" => Some(py_func("_the_first_step_left", |args: &[Py]| _the_first_step_left(args))),
            "_the_last_answer" => Some(py_func("_the_last_answer", |args: &[Py]| _the_last_answer(args))),
            "_the_lines" => Some(py_func("_the_lines", |args: &[Py]| _the_lines(args))),
            "_the_move_after" => Some(py_func("_the_move_after", |args: &[Py]| _the_move_after(args))),
            "_the_move_pieces" => Some(py_func("_the_move_pieces", |args: &[Py]| _the_move_pieces(args))),
            "_the_past_step" => Some(py_func("_the_past_step", |args: &[Py]| _the_past_step(args))),
            "_the_person_word" => Some(py_func("_the_person_word", |args: &[Py]| _the_person_word(args))),
            "_the_step_after" => Some(py_func("_the_step_after", |args: &[Py]| _the_step_after(args))),
            "_the_steps_of" => Some(py_func("_the_steps_of", |args: &[Py]| _the_steps_of(args))),
            "_up_to_the_contrast" => Some(py_func("_up_to_the_contrast", |args: &[Py]| _up_to_the_contrast(args))),
            "_walk_answer" => Some(py_func("_walk_answer", |args: &[Py]| _walk_answer(args))),
            "_with_its_tail" => Some(py_func("_with_its_tail", |args: &[Py]| _with_its_tail(args))),
            "a_card_is_not_the_answer" => Some(py_func("a_card_is_not_the_answer", |args: &[Py]| a_card_is_not_the_answer(args))),
            "acted_pool" => Some(py_func("acted_pool", |args: &[Py]| acted_pool(args))),
            "after_an_answer" => Some(py_func("after_an_answer", |args: &[Py]| after_an_answer(args))),
            "already_named" => Some(py_func("already_named", |args: &[Py]| already_named(args))),
            "asked_what_its_doing" => Some(py_func("asked_what_its_doing", |args: &[Py]| asked_what_its_doing(args))),
            "asks_for_a_fix" => Some(py_func("asks_for_a_fix", |args: &[Py]| asks_for_a_fix(args))),
            "buying" => Some(py_func("buying", |args: &[Py]| buying(args))),
            "has_evidence" => Some(py_func("has_evidence", |args: &[Py]| has_evidence(args))),
            "in_care_pool" => Some(py_func("in_care_pool", |args: &[Py]| in_care_pool(args))),
            "in_the_answer" => Some(py_func("in_the_answer", |args: &[Py]| in_the_answer(args))),
            "last_answer_words" => Some(py_func("last_answer_words", |args: &[Py]| last_answer_words(args))),
            "might_be_a_report" => Some(py_func("might_be_a_report", |args: &[Py]| might_be_a_report(args))),
            "names_a_thing" => Some(py_func("names_a_thing", |args: &[Py]| names_a_thing(args))),
            "names_a_trade" => Some(py_func("names_a_trade", |args: &[Py]| names_a_trade(args))),
            "next_step_ask_pool" => Some(py_func("next_step_ask_pool", |args: &[Py]| next_step_ask_pool(args))),
            "next_step_pool" => Some(py_func("next_step_pool", |args: &[Py]| next_step_pool(args))),
            "person_said" => Some(py_func("person_said", |args: &[Py]| person_said(args))),
            "pool_for" => Some(py_func("pool_for", |args: &[Py]| pool_for(args))),
            "reads_as_a_step_done" => Some(py_func("reads_as_a_step_done", |args: &[Py]| reads_as_a_step_done(args))),
            "reads_as_not_worked" => Some(py_func("reads_as_not_worked", |args: &[Py]| reads_as_not_worked(args))),
            "reply_for" => Some(py_func("reply_for", |args: &[Py]| reply_for(args))),
            "still_going_on" => Some(py_func("still_going_on", |args: &[Py]| still_going_on(args))),
            "still_tail" => Some(py_func("still_tail", |args: &[Py]| still_tail(args))),
            "still_the_state" => Some(py_func("still_the_state", |args: &[Py]| still_the_state(args))),
            "the_step_tried" => Some(py_func("the_step_tried", |args: &[Py]| the_step_tried(args))),
            "the_still_state" => Some(py_func("the_still_state", |args: &[Py]| the_still_state(args))),
            "trade_for" => Some(py_func("trade_for", |args: &[Py]| trade_for(args))),
            "words_of" => Some(py_func("words_of", |args: &[Py]| words_of(args))),
            _ => None,
        }),
        set: None,
    }))
}

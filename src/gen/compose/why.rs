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


pub static ABOUT_THE_BOT_LINE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_NOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ACT_NOW_SOON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AFTER_THE_MARK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGAIN_SHORTER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AGAIN_WORDS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AND_ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static AND_STEP: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWERED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ANSWER_LONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_GENERAL_CLAIM: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NOD_IS_SHORT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_NOD_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_PLACEHOLDER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static A_THING_NAMED_BEFORE_A_COLON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static BARE_WHY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CARE_ANSWER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CAUSING_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CHANGE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAIM_BEFORE_COLON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAIM_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CLAUSE_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITIONAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static CONDITIONAL_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DANGLING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static DOSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static ENDS_THE_CLASS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static END_OF_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXAMPLE_IN_QUOTES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static EXPLAINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static FIRST_SENTENCES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static HEDGED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static IMPERATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static INSTRUCTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static KEEP_CAPITAL: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LEADING_MARKER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static LIFE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NAMED_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NEGATIVE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static NO_CUT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static OPENS_ON_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static PARTS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static REPEAT_LEADS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static RESULT_NUMBER: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAID_NO: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SAYS_WHAT_IT_DOES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SHORT_OF_A_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SO_ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SO_FROM_HERE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SO_REASON: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static STEP_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUBSTANCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static SUGGESTED_VERBS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_DECISION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THEIR_SUGGESTION: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_BOT_DESCRIBED_ITSELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_FIX: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THE_REASON_POINTS_AT_THE_ADVICE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static THINKS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHAT_IT_TAKES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHEN_CAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_AGAIN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_ANSWERED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_BODY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_CARE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_END: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_FRONT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_METHOD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_NAMED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_NAMED_BECAUSE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_NAMED_SELF: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_NOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_NOT_ANSWERED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_NOT_ITS_OWN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_THE_BOT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WHY_YOU_SAID_IT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRONG: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRONG_ANSWERED: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static WRONG_NOT_A_FAULT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _ANNOUNCES: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_CATEGORY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_CAUSE_OPENS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_CLAUSE_LEAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DOING: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_DOING_WINDOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LEAD_IN: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_LONG_SENTENCE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_PARTICIPLE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_REASON_LEAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_SAVE_LEAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_STRIDE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _A_VERB: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _EXPLAINS: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEANS_BACK: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _LEANS_BACK_WINDOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _NEEDS_A_SUBJECT: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _OPENS_ON_A_FIGURE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_FORWARD: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_FORWARD_ANY: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _POINTS_FORWARD_LEAST: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _SAYS_HOW: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
pub static _VERB_SHAPE: Lazy<RwLock<Py>> = Lazy::new(|| RwLock::new(Py::None));
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
pub fn g_ABOUT_THE_BOT_LINE() -> Py { ensure_init(); ABOUT_THE_BOT_LINE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ABOUT_THE_BOT_LINE(v: Py) { ensure_init(); if let Ok(mut w) = ABOUT_THE_BOT_LINE.write() { *w = v; } }
pub fn g_ACT_NOW() -> Py { ensure_init(); ACT_NOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_NOW(v: Py) { ensure_init(); if let Ok(mut w) = ACT_NOW.write() { *w = v; } }
pub fn g_ACT_NOW_SOON() -> Py { ensure_init(); ACT_NOW_SOON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ACT_NOW_SOON(v: Py) { ensure_init(); if let Ok(mut w) = ACT_NOW_SOON.write() { *w = v; } }
pub fn g_AFTER_THE_MARK() -> Py { ensure_init(); AFTER_THE_MARK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AFTER_THE_MARK(v: Py) { ensure_init(); if let Ok(mut w) = AFTER_THE_MARK.write() { *w = v; } }
pub fn g_AGAIN_SHORTER() -> Py { ensure_init(); AGAIN_SHORTER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGAIN_SHORTER(v: Py) { ensure_init(); if let Ok(mut w) = AGAIN_SHORTER.write() { *w = v; } }
pub fn g_AGAIN_WORDS() -> Py { ensure_init(); AGAIN_WORDS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AGAIN_WORDS(v: Py) { ensure_init(); if let Ok(mut w) = AGAIN_WORDS.write() { *w = v; } }
pub fn g_AND_ADVICE() -> Py { ensure_init(); AND_ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AND_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = AND_ADVICE.write() { *w = v; } }
pub fn g_AND_STEP() -> Py { ensure_init(); AND_STEP.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_AND_STEP(v: Py) { ensure_init(); if let Ok(mut w) = AND_STEP.write() { *w = v; } }
pub fn g_ANSWERED() -> Py { ensure_init(); ANSWERED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWERED(v: Py) { ensure_init(); if let Ok(mut w) = ANSWERED.write() { *w = v; } }
pub fn g_ANSWER_LONG() -> Py { ensure_init(); ANSWER_LONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ANSWER_LONG(v: Py) { ensure_init(); if let Ok(mut w) = ANSWER_LONG.write() { *w = v; } }
pub fn g_A_GENERAL_CLAIM() -> Py { ensure_init(); A_GENERAL_CLAIM.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_GENERAL_CLAIM(v: Py) { ensure_init(); if let Ok(mut w) = A_GENERAL_CLAIM.write() { *w = v; } }
pub fn g_A_LEAD_IN() -> Py { ensure_init(); A_LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = A_LEAD_IN.write() { *w = v; } }
pub fn g_A_NOD_IS_SHORT() -> Py { ensure_init(); A_NOD_IS_SHORT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NOD_IS_SHORT(v: Py) { ensure_init(); if let Ok(mut w) = A_NOD_IS_SHORT.write() { *w = v; } }
pub fn g_A_NOD_OPENS() -> Py { ensure_init(); A_NOD_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_NOD_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = A_NOD_OPENS.write() { *w = v; } }
pub fn g_A_PLACEHOLDER() -> Py { ensure_init(); A_PLACEHOLDER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_PLACEHOLDER(v: Py) { ensure_init(); if let Ok(mut w) = A_PLACEHOLDER.write() { *w = v; } }
pub fn g_A_THING_NAMED_BEFORE_A_COLON() -> Py { ensure_init(); A_THING_NAMED_BEFORE_A_COLON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_A_THING_NAMED_BEFORE_A_COLON(v: Py) { ensure_init(); if let Ok(mut w) = A_THING_NAMED_BEFORE_A_COLON.write() { *w = v; } }
pub fn g_BARE_WHY() -> Py { ensure_init(); BARE_WHY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_BARE_WHY(v: Py) { ensure_init(); if let Ok(mut w) = BARE_WHY.write() { *w = v; } }
pub fn g_CARE_ANSWER() -> Py { ensure_init(); CARE_ANSWER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CARE_ANSWER(v: Py) { ensure_init(); if let Ok(mut w) = CARE_ANSWER.write() { *w = v; } }
pub fn g_CAUSE() -> Py { ensure_init(); CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = CAUSE.write() { *w = v; } }
pub fn g_CAUSING_IT() -> Py { ensure_init(); CAUSING_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CAUSING_IT(v: Py) { ensure_init(); if let Ok(mut w) = CAUSING_IT.write() { *w = v; } }
pub fn g_CHANGE() -> Py { ensure_init(); CHANGE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CHANGE(v: Py) { ensure_init(); if let Ok(mut w) = CHANGE.write() { *w = v; } }
pub fn g_CLAIM_BEFORE_COLON() -> Py { ensure_init(); CLAIM_BEFORE_COLON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAIM_BEFORE_COLON(v: Py) { ensure_init(); if let Ok(mut w) = CLAIM_BEFORE_COLON.write() { *w = v; } }
pub fn g_CLAIM_SUBJECT() -> Py { ensure_init(); CLAIM_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAIM_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = CLAIM_SUBJECT.write() { *w = v; } }
pub fn g_CLAUSE_END() -> Py { ensure_init(); CLAUSE_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CLAUSE_END(v: Py) { ensure_init(); if let Ok(mut w) = CLAUSE_END.write() { *w = v; } }
pub fn g_CONDITIONAL() -> Py { ensure_init(); CONDITIONAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL(v: Py) { ensure_init(); if let Ok(mut w) = CONDITIONAL.write() { *w = v; } }
pub fn g_CONDITIONAL_SENTENCE() -> Py { ensure_init(); CONDITIONAL_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_CONDITIONAL_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = CONDITIONAL_SENTENCE.write() { *w = v; } }
pub fn g_DANGLING() -> Py { ensure_init(); DANGLING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DANGLING(v: Py) { ensure_init(); if let Ok(mut w) = DANGLING.write() { *w = v; } }
pub fn g_DOSE() -> Py { ensure_init(); DOSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_DOSE(v: Py) { ensure_init(); if let Ok(mut w) = DOSE.write() { *w = v; } }
pub fn g_ENDS_THE_CLASS() -> Py { ensure_init(); ENDS_THE_CLASS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_ENDS_THE_CLASS(v: Py) { ensure_init(); if let Ok(mut w) = ENDS_THE_CLASS.write() { *w = v; } }
pub fn g_END_OF_SENTENCE() -> Py { ensure_init(); END_OF_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_END_OF_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = END_OF_SENTENCE.write() { *w = v; } }
pub fn g_EXAMPLE_IN_QUOTES() -> Py { ensure_init(); EXAMPLE_IN_QUOTES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXAMPLE_IN_QUOTES(v: Py) { ensure_init(); if let Ok(mut w) = EXAMPLE_IN_QUOTES.write() { *w = v; } }
pub fn g_EXPLAINS() -> Py { ensure_init(); EXPLAINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_EXPLAINS(v: Py) { ensure_init(); if let Ok(mut w) = EXPLAINS.write() { *w = v; } }
pub fn g_FIRST_SENTENCES() -> Py { ensure_init(); FIRST_SENTENCES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_FIRST_SENTENCES(v: Py) { ensure_init(); if let Ok(mut w) = FIRST_SENTENCES.write() { *w = v; } }
pub fn g_HEDGED() -> Py { ensure_init(); HEDGED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_HEDGED(v: Py) { ensure_init(); if let Ok(mut w) = HEDGED.write() { *w = v; } }
pub fn g_IMPERATIVE() -> Py { ensure_init(); IMPERATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_IMPERATIVE(v: Py) { ensure_init(); if let Ok(mut w) = IMPERATIVE.write() { *w = v; } }
pub fn g_INSTRUCTION() -> Py { ensure_init(); INSTRUCTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_INSTRUCTION(v: Py) { ensure_init(); if let Ok(mut w) = INSTRUCTION.write() { *w = v; } }
pub fn g_KEEP_CAPITAL() -> Py { ensure_init(); KEEP_CAPITAL.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_KEEP_CAPITAL(v: Py) { ensure_init(); if let Ok(mut w) = KEEP_CAPITAL.write() { *w = v; } }
pub fn g_LEADING_MARKER() -> Py { ensure_init(); LEADING_MARKER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LEADING_MARKER(v: Py) { ensure_init(); if let Ok(mut w) = LEADING_MARKER.write() { *w = v; } }
pub fn g_LIFE() -> Py { ensure_init(); LIFE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_LIFE(v: Py) { ensure_init(); if let Ok(mut w) = LIFE.write() { *w = v; } }
pub fn g_NAMED_CAUSE() -> Py { ensure_init(); NAMED_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NAMED_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = NAMED_CAUSE.write() { *w = v; } }
pub fn g_NEGATIVE() -> Py { ensure_init(); NEGATIVE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NEGATIVE(v: Py) { ensure_init(); if let Ok(mut w) = NEGATIVE.write() { *w = v; } }
pub fn g_NO_CUT() -> Py { ensure_init(); NO_CUT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_NO_CUT(v: Py) { ensure_init(); if let Ok(mut w) = NO_CUT.write() { *w = v; } }
pub fn g_OPENS_ON_CARE() -> Py { ensure_init(); OPENS_ON_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_OPENS_ON_CARE(v: Py) { ensure_init(); if let Ok(mut w) = OPENS_ON_CARE.write() { *w = v; } }
pub fn g_PARTS() -> Py { ensure_init(); PARTS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_PARTS(v: Py) { ensure_init(); if let Ok(mut w) = PARTS.write() { *w = v; } }
pub fn g_REPEAT_LEADS() -> Py { ensure_init(); REPEAT_LEADS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_REPEAT_LEADS(v: Py) { ensure_init(); if let Ok(mut w) = REPEAT_LEADS.write() { *w = v; } }
pub fn g_RESULT_NUMBER() -> Py { ensure_init(); RESULT_NUMBER.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_RESULT_NUMBER(v: Py) { ensure_init(); if let Ok(mut w) = RESULT_NUMBER.write() { *w = v; } }
pub fn g_SAID_AGAIN() -> Py { ensure_init(); SAID_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = SAID_AGAIN.write() { *w = v; } }
pub fn g_SAID_NO() -> Py { ensure_init(); SAID_NO.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAID_NO(v: Py) { ensure_init(); if let Ok(mut w) = SAID_NO.write() { *w = v; } }
pub fn g_SAYS_WHAT_IT_DOES() -> Py { ensure_init(); SAYS_WHAT_IT_DOES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SAYS_WHAT_IT_DOES(v: Py) { ensure_init(); if let Ok(mut w) = SAYS_WHAT_IT_DOES.write() { *w = v; } }
pub fn g_SENTENCE() -> Py { ensure_init(); SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = SENTENCE.write() { *w = v; } }
pub fn g_SHORT_OF_A_CAUSE() -> Py { ensure_init(); SHORT_OF_A_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SHORT_OF_A_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = SHORT_OF_A_CAUSE.write() { *w = v; } }
pub fn g_SO_ADVICE() -> Py { ensure_init(); SO_ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SO_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = SO_ADVICE.write() { *w = v; } }
pub fn g_SO_FROM_HERE() -> Py { ensure_init(); SO_FROM_HERE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SO_FROM_HERE(v: Py) { ensure_init(); if let Ok(mut w) = SO_FROM_HERE.write() { *w = v; } }
pub fn g_SO_REASON() -> Py { ensure_init(); SO_REASON.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SO_REASON(v: Py) { ensure_init(); if let Ok(mut w) = SO_REASON.write() { *w = v; } }
pub fn g_STEP_VERBS() -> Py { ensure_init(); STEP_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_STEP_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = STEP_VERBS.write() { *w = v; } }
pub fn g_SUBSTANCE() -> Py { ensure_init(); SUBSTANCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUBSTANCE(v: Py) { ensure_init(); if let Ok(mut w) = SUBSTANCE.write() { *w = v; } }
pub fn g_SUGGESTED_VERBS() -> Py { ensure_init(); SUGGESTED_VERBS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_SUGGESTED_VERBS(v: Py) { ensure_init(); if let Ok(mut w) = SUGGESTED_VERBS.write() { *w = v; } }
pub fn g_THEIR_DECISION() -> Py { ensure_init(); THEIR_DECISION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_DECISION(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_DECISION.write() { *w = v; } }
pub fn g_THEIR_SUGGESTION() -> Py { ensure_init(); THEIR_SUGGESTION.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THEIR_SUGGESTION(v: Py) { ensure_init(); if let Ok(mut w) = THEIR_SUGGESTION.write() { *w = v; } }
pub fn g_THE_BOT_DESCRIBED_ITSELF() -> Py { ensure_init(); THE_BOT_DESCRIBED_ITSELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_BOT_DESCRIBED_ITSELF(v: Py) { ensure_init(); if let Ok(mut w) = THE_BOT_DESCRIBED_ITSELF.write() { *w = v; } }
pub fn g_THE_FIX() -> Py { ensure_init(); THE_FIX.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_FIX(v: Py) { ensure_init(); if let Ok(mut w) = THE_FIX.write() { *w = v; } }
pub fn g_THE_REASON_POINTS_AT_THE_ADVICE() -> Py { ensure_init(); THE_REASON_POINTS_AT_THE_ADVICE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THE_REASON_POINTS_AT_THE_ADVICE(v: Py) { ensure_init(); if let Ok(mut w) = THE_REASON_POINTS_AT_THE_ADVICE.write() { *w = v; } }
pub fn g_THINKS_BACK() -> Py { ensure_init(); THINKS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_THINKS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = THINKS_BACK.write() { *w = v; } }
pub fn g_WHAT_IT_TAKES() -> Py { ensure_init(); WHAT_IT_TAKES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHAT_IT_TAKES(v: Py) { ensure_init(); if let Ok(mut w) = WHAT_IT_TAKES.write() { *w = v; } }
pub fn g_WHEN_CAUSE() -> Py { ensure_init(); WHEN_CAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHEN_CAUSE(v: Py) { ensure_init(); if let Ok(mut w) = WHEN_CAUSE.write() { *w = v; } }
pub fn g_WHY_AGAIN() -> Py { ensure_init(); WHY_AGAIN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_AGAIN(v: Py) { ensure_init(); if let Ok(mut w) = WHY_AGAIN.write() { *w = v; } }
pub fn g_WHY_ANSWERED() -> Py { ensure_init(); WHY_ANSWERED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_ANSWERED(v: Py) { ensure_init(); if let Ok(mut w) = WHY_ANSWERED.write() { *w = v; } }
pub fn g_WHY_BODY() -> Py { ensure_init(); WHY_BODY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_BODY(v: Py) { ensure_init(); if let Ok(mut w) = WHY_BODY.write() { *w = v; } }
pub fn g_WHY_CARE() -> Py { ensure_init(); WHY_CARE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_CARE(v: Py) { ensure_init(); if let Ok(mut w) = WHY_CARE.write() { *w = v; } }
pub fn g_WHY_END() -> Py { ensure_init(); WHY_END.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_END(v: Py) { ensure_init(); if let Ok(mut w) = WHY_END.write() { *w = v; } }
pub fn g_WHY_FRONT() -> Py { ensure_init(); WHY_FRONT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_FRONT(v: Py) { ensure_init(); if let Ok(mut w) = WHY_FRONT.write() { *w = v; } }
pub fn g_WHY_METHOD() -> Py { ensure_init(); WHY_METHOD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_METHOD(v: Py) { ensure_init(); if let Ok(mut w) = WHY_METHOD.write() { *w = v; } }
pub fn g_WHY_NAMED() -> Py { ensure_init(); WHY_NAMED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_NAMED(v: Py) { ensure_init(); if let Ok(mut w) = WHY_NAMED.write() { *w = v; } }
pub fn g_WHY_NAMED_BECAUSE() -> Py { ensure_init(); WHY_NAMED_BECAUSE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_NAMED_BECAUSE(v: Py) { ensure_init(); if let Ok(mut w) = WHY_NAMED_BECAUSE.write() { *w = v; } }
pub fn g_WHY_NAMED_SELF() -> Py { ensure_init(); WHY_NAMED_SELF.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_NAMED_SELF(v: Py) { ensure_init(); if let Ok(mut w) = WHY_NAMED_SELF.write() { *w = v; } }
pub fn g_WHY_NOT() -> Py { ensure_init(); WHY_NOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_NOT(v: Py) { ensure_init(); if let Ok(mut w) = WHY_NOT.write() { *w = v; } }
pub fn g_WHY_NOT_ANSWERED() -> Py { ensure_init(); WHY_NOT_ANSWERED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_NOT_ANSWERED(v: Py) { ensure_init(); if let Ok(mut w) = WHY_NOT_ANSWERED.write() { *w = v; } }
pub fn g_WHY_NOT_ITS_OWN() -> Py { ensure_init(); WHY_NOT_ITS_OWN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_NOT_ITS_OWN(v: Py) { ensure_init(); if let Ok(mut w) = WHY_NOT_ITS_OWN.write() { *w = v; } }
pub fn g_WHY_THE_BOT() -> Py { ensure_init(); WHY_THE_BOT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_THE_BOT(v: Py) { ensure_init(); if let Ok(mut w) = WHY_THE_BOT.write() { *w = v; } }
pub fn g_WHY_YOU_SAID_IT() -> Py { ensure_init(); WHY_YOU_SAID_IT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WHY_YOU_SAID_IT(v: Py) { ensure_init(); if let Ok(mut w) = WHY_YOU_SAID_IT.write() { *w = v; } }
pub fn g_WRONG() -> Py { ensure_init(); WRONG.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRONG(v: Py) { ensure_init(); if let Ok(mut w) = WRONG.write() { *w = v; } }
pub fn g_WRONG_ANSWERED() -> Py { ensure_init(); WRONG_ANSWERED.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRONG_ANSWERED(v: Py) { ensure_init(); if let Ok(mut w) = WRONG_ANSWERED.write() { *w = v; } }
pub fn g_WRONG_NOT_A_FAULT() -> Py { ensure_init(); WRONG_NOT_A_FAULT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s_WRONG_NOT_A_FAULT(v: Py) { ensure_init(); if let Ok(mut w) = WRONG_NOT_A_FAULT.write() { *w = v; } }
pub fn g__ANNOUNCES() -> Py { ensure_init(); _ANNOUNCES.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__ANNOUNCES(v: Py) { ensure_init(); if let Ok(mut w) = _ANNOUNCES.write() { *w = v; } }
pub fn g__A_CATEGORY() -> Py { ensure_init(); _A_CATEGORY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_CATEGORY(v: Py) { ensure_init(); if let Ok(mut w) = _A_CATEGORY.write() { *w = v; } }
pub fn g__A_CAUSE_OPENS() -> Py { ensure_init(); _A_CAUSE_OPENS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_CAUSE_OPENS(v: Py) { ensure_init(); if let Ok(mut w) = _A_CAUSE_OPENS.write() { *w = v; } }
pub fn g__A_CLAUSE_LEAST() -> Py { ensure_init(); _A_CLAUSE_LEAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_CLAUSE_LEAST(v: Py) { ensure_init(); if let Ok(mut w) = _A_CLAUSE_LEAST.write() { *w = v; } }
pub fn g__A_DOING() -> Py { ensure_init(); _A_DOING.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DOING(v: Py) { ensure_init(); if let Ok(mut w) = _A_DOING.write() { *w = v; } }
pub fn g__A_DOING_WINDOW() -> Py { ensure_init(); _A_DOING_WINDOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_DOING_WINDOW(v: Py) { ensure_init(); if let Ok(mut w) = _A_DOING_WINDOW.write() { *w = v; } }
pub fn g__A_FIGURE() -> Py { ensure_init(); _A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = _A_FIGURE.write() { *w = v; } }
pub fn g__A_LEAD_IN() -> Py { ensure_init(); _A_LEAD_IN.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LEAD_IN(v: Py) { ensure_init(); if let Ok(mut w) = _A_LEAD_IN.write() { *w = v; } }
pub fn g__A_LONG_SENTENCE() -> Py { ensure_init(); _A_LONG_SENTENCE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_LONG_SENTENCE(v: Py) { ensure_init(); if let Ok(mut w) = _A_LONG_SENTENCE.write() { *w = v; } }
pub fn g__A_PARTICIPLE() -> Py { ensure_init(); _A_PARTICIPLE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_PARTICIPLE(v: Py) { ensure_init(); if let Ok(mut w) = _A_PARTICIPLE.write() { *w = v; } }
pub fn g__A_REASON_LEAST() -> Py { ensure_init(); _A_REASON_LEAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_REASON_LEAST(v: Py) { ensure_init(); if let Ok(mut w) = _A_REASON_LEAST.write() { *w = v; } }
pub fn g__A_SAVE_LEAST() -> Py { ensure_init(); _A_SAVE_LEAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_SAVE_LEAST(v: Py) { ensure_init(); if let Ok(mut w) = _A_SAVE_LEAST.write() { *w = v; } }
pub fn g__A_STRIDE() -> Py { ensure_init(); _A_STRIDE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_STRIDE(v: Py) { ensure_init(); if let Ok(mut w) = _A_STRIDE.write() { *w = v; } }
pub fn g__A_VERB() -> Py { ensure_init(); _A_VERB.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__A_VERB(v: Py) { ensure_init(); if let Ok(mut w) = _A_VERB.write() { *w = v; } }
pub fn g__EXPLAINS() -> Py { ensure_init(); _EXPLAINS.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__EXPLAINS(v: Py) { ensure_init(); if let Ok(mut w) = _EXPLAINS.write() { *w = v; } }
pub fn g__LEANS_BACK() -> Py { ensure_init(); _LEANS_BACK.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEANS_BACK(v: Py) { ensure_init(); if let Ok(mut w) = _LEANS_BACK.write() { *w = v; } }
pub fn g__LEANS_BACK_WINDOW() -> Py { ensure_init(); _LEANS_BACK_WINDOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__LEANS_BACK_WINDOW(v: Py) { ensure_init(); if let Ok(mut w) = _LEANS_BACK_WINDOW.write() { *w = v; } }
pub fn g__NEEDS_A_SUBJECT() -> Py { ensure_init(); _NEEDS_A_SUBJECT.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__NEEDS_A_SUBJECT(v: Py) { ensure_init(); if let Ok(mut w) = _NEEDS_A_SUBJECT.write() { *w = v; } }
pub fn g__OPENS_ON_A_FIGURE() -> Py { ensure_init(); _OPENS_ON_A_FIGURE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__OPENS_ON_A_FIGURE(v: Py) { ensure_init(); if let Ok(mut w) = _OPENS_ON_A_FIGURE.write() { *w = v; } }
pub fn g__POINTS_FORWARD() -> Py { ensure_init(); _POINTS_FORWARD.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_FORWARD(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_FORWARD.write() { *w = v; } }
pub fn g__POINTS_FORWARD_ANY() -> Py { ensure_init(); _POINTS_FORWARD_ANY.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_FORWARD_ANY(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_FORWARD_ANY.write() { *w = v; } }
pub fn g__POINTS_FORWARD_LEAST() -> Py { ensure_init(); _POINTS_FORWARD_LEAST.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__POINTS_FORWARD_LEAST(v: Py) { ensure_init(); if let Ok(mut w) = _POINTS_FORWARD_LEAST.write() { *w = v; } }
pub fn g__SAYS_HOW() -> Py { ensure_init(); _SAYS_HOW.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__SAYS_HOW(v: Py) { ensure_init(); if let Ok(mut w) = _SAYS_HOW.write() { *w = v; } }
pub fn g__VERB_SHAPE() -> Py { ensure_init(); _VERB_SHAPE.read().map(|r| r.clone()).unwrap_or(Py::None) }
#[allow(non_snake_case)]
pub fn s__VERB_SHAPE(v: Py) { ensure_init(); if let Ok(mut w) = _VERB_SHAPE.write() { *w = v; } }
fn module_body() {
    let _ = py_str("The reason behind the answer just given: a bare \"why\" as a follow-up.\n\n\"why\", \"why not\" and \"how come\" carry no subject of their own, so nothing that matches words on\na line can take them: the knowledge base has no key to match, and the small-talk pools want a\nmood, not a question. Main answered them with the search hedge - \"I'd rather not guess, a quick\nsearch online will get you a solid answer\" - which throws away the turn just given and sends the\nperson off to look up something they already have half of.\n\nWhat a friend does with the nudge is say plainly that the reason is not something they can see\nfrom here, and ask the one question that would narrow it. That is all this module says. It never\ninvents a cause: a confident wrong reason is worse than an honest short one (STYLE.md,\n\"Honest\"), and the thing that would let the bot give a real reason is an entry\nin the knowledge base, not a phrase put together here.\n\nTwo things have to hold before it speaks. The line has to be the whole question, so a why with\nwords of its own (\"why is my car overheating\") goes to whatever can match those words. And the\nturn above has to be an answer that made a claim: a condolence, an emergency line, a recall or\nthe bot's own shrug is not something to ask the reason for, so those keep whatever they would\nhave said. That is what the module's ANSWERED list is for.\n");
    s_BARE_WHY(stdlib_call("re", "compile", &[py_str("^(?:(?:but|and|so|ok|okay|yeah|no|well|right)\\s+)*(?:(?:why|y|how come)(?:\\s+(?:not|tho|though|so|then|is that|is it|was that|does it|do they|did it|did they|would it|wouldnt it|should i|shouldnt i)(?:\\s+(?:tho|though|then))?)?(?:\\s+(?:did|does|do|is|was|were|would|will|should)\\s+(?:that|it|this|they|those|these)(?:\\s+(?:happen|happened|happening|keep happening|keeps happening|work|works|not work|never work|be|matter|help|go|goes|go wrong|goes wrong|come|do that|do it))?)?|how come\\s+(?:that|it|this)(?:\\s+(?:happens?|happened|went wrong|goes wrong|keeps happening))?|what(?:'?s|\\s+is|\\s+was|\\s+(?:could|might|would|can|may)\\s+be)\\s+causing\\s+(?:that|it|this)|what(?:'?s|\\s+is|\\s+was)\\s+(?:going wrong|wrong|the (?:problem|issue|matter))|what\\s+(?:am|are|was|were)\\s+(?:i|we)\\s+(?:doing\\s+wrong|missing|getting\\s+wrong)|what\\s+did\\s+(?:i|we)\\s+do\\s+wrong)[\\s?!.,]*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CAUSING_IT(stdlib_call("re", "compile", &[py_str("what(?:'?s|\\s+is|\\s+was|\\s+(?:could|might|would|can|may)\\s+be)\\s+causing\\s+(?:that|it|this)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WRONG(stdlib_call("re", "compile", &[py_str("\\bwhat(?:'?s|\\s+is|\\s+was)?\\s*(?:am\\s+i\\s+)?(?:going\\s+wrong|wrong|doing\\s+wrong|getting\\s+wrong|missing|the\\s+(?:problem|issue|matter))\\b|\\bwhat\\s+did\\s+(?:i|we)\\s+do\\s+wrong\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LIFE(stdlib_call("re", "compile", &[py_str("\\b(?:my (?:job|work|boss|career|business|kids?|children|son|daughter|mum|mom|dad|husband|wife|partner|boyfriend|girlfriend|friend|sister|brother|family|marriage|relationship|life|money|salary|wages|benefits|rent|interview|exam|degree|course)|job|jobs|jobless|unemployed|redundant|sacked|fired|divorce|divorced|breakup|broke up|shouting|shout at|arguing|argued|argument|falling out|fell out|not talking to me|grieving|funeral|depressed|anxious|lonely|ashamed|guilty)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHY_NOT(stdlib_call("re", "compile", &[py_str("\\bnot\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_DECISION(stdlib_call("re", "compile", &[py_str("\\b(?:he|she|they|him|her|them|his|their|my (?:boss|manager|landlord|landlady|wife|husband|partner|boyfriend|girlfriend|ex|mum|mom|mam|dad|father|mother|son|daughter|child|kid|friend|mate|neighbour|neighbor|colleague|council|company|employer|school|teacher|gp|doctor|nurse|worker|agent|insurer)\\b|the (?:council|company|school|landlord|manager|boss|insurer|bank|hospital|surgery)\\b)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_NO(stdlib_call("re", "compile", &[py_str("\\b(?:said|says|saying|told|tells|refused|refuses|refusing|won'?t|wont|will not|can'?t|cant|cannot|denied|rejected|decided|decides|reckons|thinks|banned|stopped|turned (?:me|it|us) down|not allowed|no chance)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THEIR_SUGGESTION(stdlib_call("re", "compile", &[py_str("\\A(?:you (?:could|should|might|may|can) (?:try|just|also)|try|maybe try|have you tried|why don'?t you|what about|how about|consider)\\b|\\b(?:you (?:could|should|might) (?:try|also try)|have you tried)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SUGGESTED_VERBS(call_builtin("frozenset", &[py_tuple(vec![py_str("try"), py_str("cut"), py_str("stop"), py_str("quit"), py_str("feed"), py_str("turn"), py_str("bleed"), py_str("top"), py_str("add"), py_str("start"), py_str("use"), py_str("take"), py_str("buy"), py_str("get"), py_str("give"), py_str("keep"), py_str("put"), py_str("run"), py_str("walk"), py_str("drink"), py_str("eat"), py_str("avoid"), py_str("ask"), py_str("go"), py_str("see"), py_str("check"), py_str("swap"), py_str("replace"), py_str("wash"), py_str("clean"), py_str("sort"), py_str("leave"), py_str("make"), py_str("do")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWERED(call_builtin("frozenset", &[py_tuple(vec![py_str("answers"), py_str("chat")])]).unwrap_or_else(|e| exc::raise(e)));
    s_SUBSTANCE(Py::Int(8));
    s_HEDGED(stdlib_call("re", "compile", &[py_str("\\b(?:search|searching|beyond what i know|don'?t know enough|not something i can see|rather not guess|no way of knowing)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHY_ANSWERED(py_tuple(vec![py_str("I'd be guessing at the reason - the cause is not something I can see from here. Which part of that do you want the why for?"), py_str("I don't know why, and I'd rather say so than invent one. What happens just before it goes wrong?"), py_str("Honestly the why is the bit I can't tell from this side. Tell me which part you mean and I will help you work it out.")]));
    s_WHY_BODY(py_tuple(vec![py_str("Honestly, I can't tell you why from here - I'd be guessing. What sets it off?"), py_str("I don't know the reason, and I'd rather say so than invent one. When did you first notice it?"), py_str("The why is the part I can't see from what you've told me. What is it like at its worst?"), py_str("No idea why, honestly. Does it happen at a particular time, or after anything in particular?")]));
    s_CAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:because|since|that'?s why|which is why|that is why|is why|comes? from|caus(?:e|es|ed) by|means that|so that|the reason (?:is|was)|scatters?|keeps? \\w+ (?:from|off|out)|stops? [^.]{0,28}?\\b(?:forming|coming|happening|going|building|getting|spreading)|is down to|which (?:makes|means|is why)|that'?s what (?:makes|keeps|stops|drives|turns)|is what (?:drives|turns|causes|makes|keeps|stops)|(?:has|have|had|got) (?:too (?:little|much)|not enough)|(?:is|are) (?:almost |nearly |usually |normally |often |mostly )+(?:always )?\\w|live?s? in |breaks? (?:the |it |them )?\\w+ into |\\b(?:sinks?|go(?:es)?|turns?|comes?|happens?|fails?|breaks?|stops?|sticks?|cracks?|curdl(?:e|es)|collapses?|dries?|burns?|smells?|tastes?|looks?)\\b[^.;:]{0,40}?\\bwhen\\b|\\b(?:is|are|was|were)\\s+[^.;:]{0,40}?\\b\\w+(?:ed|en)\\b[^.;:]{0,20}?\\b(?:by|from)\\b)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_STEP_VERBS(py_str("(?:ventilate|open|keep|use|take|leave|put|run|wipe|wash|give|avoid|let|get|ask|check|try|swap|cover|move|hold|store|cook|drink|eat|sit|add|lower|raise|turn|clean|buy|restrict|limit|reduce|cut|change|switch|replace|bin|tip|throw|sort|fit|hire|book|pick|start|stop|do|make|go|wear|ring|call|fill|write|read|set|work|stay|carry|delete|update|download|install|uninstall|restart|reboot|reinstall|clear|free|close|move|rotate|treat|rinse|hoover|vacuum|dust|air|dry|iron|seal|label)"));
    s_SO_ADVICE(stdlib_call("re", "compile", &[add(&add(&py_str(",? ?so (?="), &g_STEP_VERBS()), &py_str("\\b)")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DOSE(stdlib_call("re", "compile", &[py_str("\\b(?:once|twice|three times|four times|daily|weekly|nightly|every (?:day|week|night|morning)|a (?:day|week|night|month)|per (?:day|week|month)|\\d+ ?(?:times|minutes|hours|glasses|litres|grams|g|mg|mg|ml|degrees|sessions))\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AND_STEP(stdlib_call("re", "compile", &[add(&add(&py_str(",? ?and (?=(?!air\\b)"), &g_STEP_VERBS()), &py_str("\\b)")), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AND_ADVICE(stdlib_call("re", "compile", &[py_str(",? ?(?:and|but) (?:it|they|that|this|you|then|if)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NEGATIVE(stdlib_call("re", "compile", &[py_str("\\b(?:never|don'?t|do not|avoid|no need to|there'?s no need)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|once|when|before|after|until)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL_SENTENCE(stdlib_call("re", "compile", &[py_str("^(?:if|unless|once|whenever|when|before|after|until|in case)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_INSTRUCTION(stdlib_call("re", "compile", &[py_str("\\b(?:never|don'?t|do not|keep|call|ring|take|ask|check|make sure|you should|if you|try to|avoid|be careful|watch|use|put|put it|store|leave it|cover it|fit|swap|replace)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_PARTS(py_tuple(vec![py_str(": "), py_str(" - "), py_str("; ")]));
    s_SO_REASON(stdlib_call("re", "compile", &[py_str("\\bso\\s+(?:the|it|they|you|your|that|this|air|water|heat|warmth)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_LEADING_MARKER(stdlib_call("re", "compile", &[py_str("^(?:since|as|so)\\s+(?=\\w)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIM_BEFORE_COLON(stdlib_call("re", "compile", &[py_str("^[^:]{4,}?\\b(?:is|are|was|were)\\b[^:]*:\\s*(?P<why>[^:]{12,})$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAIM_SUBJECT(stdlib_call("re", "compile", &[py_str("^\\s*(?:here|there|this|these|those|what|below|following)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_IMPERATIVE(stdlib_call("re", "compile", &[py_str("^(?:soak|add|mix|stir|do|use|put|take|keep|leave|cover|store|check|try|avoid|don'?t|never|give|wipe|wash|open|close|turn|set|ring|call|ask|make|let|get|go|run|start|stop|fill|brush|floss|drink|eat|sit|stand|sleep|walk|stretch|reduce|cut|change|swap|replace|clean|remove|apply|spread|heat|cook|boil|bake|roast|freeze|thaw|rest|massage|wear|hang|bin|throw|unplug|label|write|book|hire|buy|sell|learn|practise|practice|repeat|look|watch|listen|say|tell|note|remember|think|pick|choose|plan|sort|fit|trim|prune|water|feed|move|top|blot|rinse|scrape|vacuum|spray|chop|slice|simmer|grill|season|serve|cool|chill|drain|pat|press|squeeze|peel|whisk|steam|blanch|defrost|shake|dust|sweep|mop|scour|polish|iron|sand|prime|seal|charge|reboot|restart|reset|update|empty|plug|tighten|loosen|screw|glue|nail|drill|measure|mark|taste|knead|fold|roll|save|delete)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_FIX(stdlib_call("re", "compile", &[py_str("\\b(?:is|was|are|'s) the (?:fix|answer|thing to do|way to do it)\\b|\\bfix next time\\b|\\bthe fix (?:is|for|next time)\\b|\\bis how you (?:do|fix|stop|get)\\b|\\bwhat (?:you do|to do) (?:next time|instead|now)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_NAMED_CAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:the\\s+)?(?:cause|reason)\\s+(?:is|was)\\s+(.+)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_EXPLAINS(stdlib_call("re", "compile", &[py_str("\\b(?:because|since|so\\s+(?:air|the|it|they|you)|so\\s+\\w+(?:\\s+\\w+){0,4}\\s+(?:are|is|was|were)\\b|when\\b[^.!?]*\\b(?:are|is|was|were)\\b|which\\s+(?:makes|means|traps|tightens|widens)|making\\b|heats?\\s+(?:\\w+ )?(?:up\\b|the\\b|it\\b|them\\b|your\\b|a\\b)|expands?|traps?|tightens?|wide(?:ns?|ned|ning)?|carries|evaporat(?:es|ed|ing)|scatters?|tell(?:s|ing)?\\b[^.!?]*\\bto\\b|seems?\\s+to)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_DANGLING(stdlib_call("re", "compile", &[py_str("^(?:(?:but|and|so|though)\\s+)?(?:if\\s+)?(?:it|they|he|she|that|this|these|those)\\b|^(?:then|also|which|the same)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_RESULT_NUMBER(stdlib_call("re", "compile", &[py_str(",\\s*so\\b[^.!?]*\\d"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CHANGE(stdlib_call("re", "compile", &[py_str("\\b(?:sinks?|sank|rises?|rose|drops?|dropped|collapses?|collapsed|sags?|sagged|cracks?|cracked|splits?|curdles?|curdled|separates?|separated|goes|went|gets|got|turns?|turned|becomes?|became|sticks?|stuck|burns?|burnt|dries?|dried|toughens?|toughened|shrinks?|shrank|smells?|smelt|tastes?|tasted|wrinkles?|wrinkled|tears?|tore|melts?|melted|sets?|yellows?|yellowed|browns?|browned|wilts?|wilted|droops?|drooped|leaks?|leaked|clogs?|clogged|overcooks?|overcooked|undercooks?|undercooked|overbakes?|overbaked|underbakes?|underbaked|overmixes?|overmixed)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHEN_CAUSE(stdlib_call("re", "compile", &[py_str("\\b(?:when|whenever)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SO_FROM_HERE(stdlib_call("re", "compile", &[py_str(",?\\s+so\\s+|;\\s*"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHY_NAMED(py_tuple(vec![py_str("That's what is behind it: {c}"), py_str("It comes down to this: {c}"), py_str("{c} - that's the reason.")]));
    s_KEEP_CAPITAL(call_builtin("frozenset", &[py_tuple(vec![py_str("i"), py_str("i'm"), py_str("i'd"), py_str("i've"), py_str("i'll"), py_str("monday"), py_str("tuesday"), py_str("wednesday"), py_str("thursday"), py_str("friday"), py_str("saturday"), py_str("sunday"), py_str("january"), py_str("february"), py_str("march"), py_str("april"), py_str("may"), py_str("june"), py_str("july"), py_str("august"), py_str("september"), py_str("october"), py_str("november"), py_str("december"), py_str("britain"), py_str("british"), py_str("england"), py_str("english"), py_str("london"), py_str("scotland"), py_str("wales"), py_str("ireland"), py_str("america"), py_str("american"), py_str("europe"), py_str("christmas"), py_str("easter"), py_str("god")])]).unwrap_or_else(|e| exc::raise(e)));
    s_WHY_NAMED_BECAUSE(py_tuple(vec![py_str("Because {c}"), py_str("That's because {c}"), py_str("{c} - that's the reason.")]));
    s_WHY_NAMED_SELF(py_tuple(vec![py_str("The cause is {c}"), py_str("The reason is {c}")]));
    s_WHY_FRONT(call_builtin("frozenset", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&add(&add(&g_WHY_NAMED(), &g_WHY_NAMED_BECAUSE()), &g_WHY_NAMED_SELF())) { let mut frame: Py = __item.clone(); __out.push(mcall(&mcall(&mcall(&mcall(&geti(&mcall(&frame.clone(), "split", &[py_str("{")]), &Py::Int(0)), "strip", &[]), "rstrip", &[py_str(":")]), "strip", &[]), "lower", &[])); } py_list(__out) })() }) { let mut front: Py = __item.clone(); if py_truthy(&front.clone()) {  __out.push(front.clone()); } } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_WHY_END(py_str("- that's the reason"));
    s_WHY_AGAIN(py_tuple(vec![py_str("Because all I have is what you've told me, and the cause isn't in it - one more detail and I might see it."), py_str("I still can't see the cause from here: it isn't in what you've told me. What does it do just before it goes wrong?"), py_str("The reason is still the part I can't reach from this side. When it goes wrong, what has just happened?")]));
    s_WRONG_ANSWERED(py_tuple(vec![py_str("I can't see what is going wrong from here, and I'd rather say so than invent a cause. Does it go wrong the same way every time?"), py_str("Nothing you've told me points at the cause, so I'd be guessing. Which part goes wrong - the start, the middle, or the end?"), py_str("I can't see what is wrong from here, and guessing would not help you. Talk me through what happens and I'll tell you what I can see in it.")]));
    s_WRONG_NOT_A_FAULT(py_tuple(vec![py_str("I can't see a reason in what you've told me, and I'd rather say that than invent one. What happened just before it?"), py_str("Nothing you've told me points at a cause, and I won't guess at one. Talk me through what happened and I'll tell you what I can see in it."), py_str("I don't know why, honestly. Tell me a bit more about how it came about and I'll say what I can see.")]));
    s_WHY_NOT_ANSWERED(py_tuple(vec![py_str("I can't speak for anyone else's reasons - only for what you can do about it. What was said when you raised it?"), py_str("Fair question, and I have no way of knowing what is behind it. What is their side of it?"), py_str("I can't tell why not from here. What is stopping it, as far as you can see?")]));
    s_WHY_NOT_ITS_OWN(py_tuple(vec![py_str("No reason it has to be that way - it is the version that works for most people, not a rule. What is it about it that does not sit right?"), py_str("Only because it tends to be the one that works. If it does not suit you, say so and we will find another way."), py_str("There is nothing wrong with the other way if it suits you. What are you weighing up?"), py_str("Mostly because it works for most people, not because it is the only way. What would you rather do?")]));
    s_THINKS_BACK(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:but|and|so|ok|okay|yeah|no|well|right)\\s+)*(?:why|y)\\s+(?:do|did|would)\\s+(?:you|u)\\s+(?:think|thinking)\\s+(?:that|so|this|it)(?:\\s+(?:then|though|tho))?[\\s?!.,]*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NOD_OPENS(stdlib_call("re", "compile", &[py_str("^(?:alright|all\\s+right|ok|okay|sure|got\\s+it|noted|right|of\\s+course|no\\s+problem|lovely|great|good|glad|happy|sorry|thanks|thank\\s+you|fair\\s+enough|makes\\s+sense|understood|will\\s+do|cool|nice)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_NOD_IS_SHORT(Py::Int(12));
    s_THE_REASON_POINTS_AT_THE_ADVICE(stdlib_call("re", "compile", &[py_str("^(?:because\\s+|since\\s+)?(?:that|this)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_FIRST_SENTENCES(Py::Int(2));
    s_WHY_METHOD(py_tuple(vec![py_str("Those steps are the how-to; the reason under them is the part I'd be guessing at. Tell me what happens and I'll say what I can see."), py_str("Everything I gave you there is what to do, not why it happens - that half isn't in what I have. What is it doing when it goes wrong?"), py_str("The method is what I can stand behind. The cause is the bit I can't see from here, so tell me what it does and I'll work it back from that.")]));
    s_EXAMPLE_IN_QUOTES(stdlib_call("re", "compile", &[py_str("\"[^\"]{4,}\"|“[^”]{4,}”")]).unwrap_or_else(|e| exc::raise(e)));
    s_WHAT_IT_TAKES(stdlib_call("re", "compile", &[py_str("^(?P<thing>[^.;:]{4,}?\\b(?:is|are)\\s+what\\b[^,.;]{0,40}?(?:needs?|wants?|feeds?|thrives? on|likes?|goes? for)\\b[^,.;]*)[,.;]?\\s+so\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_THING_NAMED_BEFORE_A_COLON(stdlib_call("re", "compile", &[py_str("\\A(?:it|that|this)\\s+(?:is|was|isn'?t|wasn'?t)\\s+(?:almost\\s+always\\s+|nearly\\s+always\\s+|usually\\s+|often\\s+|just\\s+)?(?:the|a|an|your|too\\s+much|not\\s+enough|too\\s+little)\\s+(?P<thing>[a-z][^:,.;!?]{1,40}?)(?:,\\s*(?:not|rather\\s+than)\\s+(?:the\\s+|a\\s+|your\\s+)?[a-z][^:,.;!?]{1,25})?\\s*:\\s*\\S"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_LEAD_IN(stdlib_call("re", "compile", &[py_str("\\A(?:(?:nine|eight|seven|six|five|four|three|two)\\s+times\\s+(?:in|out\\s+of)\\s+(?:ten|twenty|a\\s+hundred|\\d+)|most\\s+of\\s+the\\s+time|more\\s+often\\s+than\\s+not|usually|nearly\\s+always|almost\\s+always|in\\s+my\\s+experience|as\\s+a\\s+rule|nine\\s+times\\s+out\\s+of\\s+ten)\\s+"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SHORT_OF_A_CAUSE(stdlib_call("re", "compile", &[py_str("\\A(?!(?:it|that|this|there|which)\\b)[^.;:]{3,60}?\\s+(?:is|are|was|were)\\s+(?:usually\\s+|nearly\\s+always\\s+|almost\\s+always\\s+|often\\s+|probably\\s+|just\\s+)?(?P<cause>(?:short\\s+of|low\\s+on|lacking|too\\s+much|too\\s+little|not\\s+enough)\\b[^.;:]{1,60})"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AFTER_THE_MARK(stdlib_call("re", "compile", &[py_str("^\\s*(?::| - |; )\\s*(?P<rest>.+)$")]).unwrap_or_else(|e| exc::raise(e)));
    s_A_GENERAL_CLAIM(stdlib_call("re", "compile", &[py_str("\\A(?:most|almost\\s+all|nearly\\s+all|in\\s+most\\s+cases|the\\s+majority\\s+of|mostly)\\s+(?P<subject>[^.;:,]{2,60}?)\\s+(?:is|are|was|were)\\s+(?P<what>[^.;:]{2,140})"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ENDS_THE_CLASS(stdlib_call("re", "compile", &[py_str("(?::|,| - | and | so | which | but | worse )"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SAYS_WHAT_IT_DOES(stdlib_call("re", "compile", &[py_str("\\b(?:want|wants|need|needs|live|lives|take|takes|last|lasts|go|goes|come|comes|get|gets|settle|settles|heal|heals|die|dies|cost|costs|help|helps|work|works|hold|holds|keep|keeps|put|puts|eat|eats|do|does|say|says|give|gives|pay|pays|rise|rises|fall|falls|start|starts|finish|finishes|run|runs|turn|turns|sit|sits|charge|charges|cover|covers|include|includes|pass|passes|count|counts|matter|matters|mean|means|stay|stays|carry|carries)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_A_PLACEHOLDER(call_builtin("frozenset", &[py_tuple(vec![py_str("it"), py_str("this"), py_str("that"), py_str("them"), py_str("they"), py_str("one"), py_str("so"), py_str("true"), py_str("fine"), py_str("ok"), py_str("okay"), py_str("normal"), py_str("harmless"), py_str("nothing"), py_str("none"), py_str("air"), py_str("you"), py_str("me"), py_str("us")])]).unwrap_or_else(|e| exc::raise(e)));
    s_ANSWER_LONG(Py::Int(18));
    s_ACT_NOW(stdlib_call("re", "compile", &[py_str("\\b(?:999|911|112|a&e|a and e|ambulance|paramedic|unconscious|not breathing|can'?t breathe|scald\\w*|never open|still hot|poison\\w*|gas leak|on fire|stop driving|pull over|don'?t wait|do not wait|stay on the line)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_ACT_NOW_SOON(stdlib_call("re", "compile", &[py_str("\\b(?:111|same[- ]day|straight away|right away|immediately|today)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CONDITIONAL(stdlib_call("re", "compile", &[py_str("\\b(?:if|unless|should|in case|any time)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_SENTENCE(stdlib_call("re", "compile", &[py_str("[^.?!;]+[.?!;]?")]).unwrap_or_else(|e| exc::raise(e)));
    s_SAID_AGAIN(stdlib_call("re", "compile", &[py_str("^(?!so\\s+what\\b)(?:(?:sorry|ok|okay|er|erm|um|eh|and|so|but|wait|no|yes)[\\s,]+)*(?:what|wat|wot|wut|huh|pardon|what\\s+did\\s+(?:you|u)\\s+say|what\\s+(?:was|is)\\s+that|say\\s+(?:that|it)\\s+again|say\\s+again|come\\s+again|run\\s+that\\s+by\\s+me\\s+again)(?:\\s+(?:sorry|please|mate|then|now))?[\\s?!.,]*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_AGAIN_SHORTER(py_tuple(vec![py_str("In short: {short}"), py_str("Shorter: {short}"), py_str("The short of it: {short}"), py_str("The gist of it: {short}")]));
    s_REPEAT_LEADS(call_builtin("tuple", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_AGAIN_SHORTER()) { let mut frame: Py = __item.clone(); __out.push(geti(&mcall(&call_builtin("str", &[frame.clone()]).unwrap_or_else(|e| exc::raise(e)), "split", &[py_str("{short}")]), &Py::Int(0))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e)));
    s_CLAUSE_END(stdlib_call("re", "compile", &[py_str(":\\s+|;\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_SAVE_LEAST(Py::Int(5));
    s__A_CLAUSE_LEAST(Py::Int(6));
    s__A_LONG_SENTENCE(Py::Int(25));
    s__A_STRIDE(Py::Int(3));
    s_AGAIN_WORDS(Py::Int(40));
    s_NO_CUT(stdlib_call("re", "compile", &[py_str("\\b(?:gp|doctor|nurse|pharmacist|chemist|dentist|physio\\w*|midwife|health visitor|111|999|a&e|hospital|ambulance|paramedic)\\b|\\b(?:get|have)\\s+it\\s+(?:checked|looked at|seen)\\b|\\bworth\\s+(?:getting|having)\\s+.{0,30}?(?:checked|looked at)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_END_OF_SENTENCE(stdlib_call("re", "compile", &[py_str("(?<=[.!?])\\s+")]).unwrap_or_else(|e| exc::raise(e)));
    s_OPENS_ON_CARE(stdlib_call("re", "compile", &[py_str("^(?:ok|okay|right|well|oh|ah|aw|ugh|sorry)?[\\s,\\u2014-]*(?:i'?m (?:really |so )?sorry|sorry|that (?:is|was) (?:a rough|rough|tough|hard|a lot)|no wonder|that sounds|lousy|horrible|awful|poor you|rough one)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DOING(stdlib_call("re", "compile", &[py_str("\\b(?:turn|switch|take|keep|put|use|try|check|give|add|buy|swap|replace|clean|wash|rinse|leave|stand|wrap|store|feed|water|encourage|bleed|call|ask|book|avoid|stop|start|run|set|hold|move|cover|cut|open|close|repeat|ring|email|text|pick|press|fill|mix|stir|boil|blot|pour|write|say|tell|offer|lower|heat|defrost|freeze|lock|shut|prick|scrub|soak|scrape|wipe|sweep|vacuum|iron|sand|seal|tape|glue|screw|drill|tie|hang|fold|chop|slice|drain|compare|measure|weigh|test|log|track|budget|plan|read|learn|practise|split|share|anchor|aim|ask|check|brush|clear|empty|refill|label|freeze|work|go|look|listen|watch|wait|sit|step|point|answer|name|note|mark|count|sort|tidy|eat|drink|cook|bake|grill|roast|serve|pack|carry|lift|bend|stretch|breathe|rest|get|give|help|do|redo|rub|polish|dry|spray|apply|dab|swab|wax|shampoo|polish)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_DOING_WINDOW(Py::Int(3));
    s__A_FIGURE(stdlib_call("re", "compile", &[py_str("\\d|\\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|forty|fifty|half|quarter|once|twice|per ?cent)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_LEAD_IN(call_builtin("frozenset", &[mcall(&py_str("on for in with at to from after before if when once and then also"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__SAYS_HOW(stdlib_call("re", "compile", &[py_str("\\b(?:is|are|was|were|has|have|had|does|do|will|would|can|could|wants?|needs?|keeps?|stops?|strips?|holds?|works?|matters?|helps?|makes?|means?|costs?|takes?|gets?|gives?|comes?|stays?|lasts?)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__ANNOUNCES(stdlib_call("re", "compile", &[py_str("\\b(?:it|them|these|those|this|that)\\b(?:\\s+[\\w'-]+){0,2}\\s+(?:order|list|steps?|way|shape)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__NEEDS_A_SUBJECT(stdlib_call("re", "compile", &[py_str("^(?:he|she|it|they|them|this|that|these|those|his|her|their|its|itself|themselves)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_PARTICIPLE(call_builtin("frozenset", &[mcall(&py_str("laid launched built made given taken held written born grown founded\nopened designed painted published signed set sold bought won lost sent kept left put begun"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__VERB_SHAPE(stdlib_call("re", "compile", &[py_str("\\b\\w{4,}(?:ed|ing)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_VERB(call_builtin("frozenset", &[mcall(&py_str("is are was were be been am has have had do does did can could will would\nshould must might may need needs want wants take takes give gives make makes keep keeps leave\nleaves put puts get gets add adds use uses turn turns go goes come comes cost costs last lasts\nhelp helps hold holds cut cuts run runs set sets let lets seem seems feel feels work works mean\nmeans change changes choose chooses find finds show shows matter matters depend depends ring\nrings wash washes clean cleans"), "split", &[])]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CAUSE_OPENS(stdlib_call("re", "compile", &[py_str("^(?:nearly|usually|mostly|often|almost|probably|generally|mainly|sometimes|always|never|rarely|hardly)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEANS_BACK(stdlib_call("re", "compile", &[py_str("\\b(?:them|these|those|both|each|either|neither|the rest|all of (?:the|them)|the others?|any of (?:the|them)|that end|this end|this order|these steps|the ones|as follows)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__LEANS_BACK_WINDOW(Py::Int(6));
    s__EXPLAINS(stdlib_call("re", "compile", &[py_str("^(?:a|an|the|most|some|many|much|which|because|so|that|this)\\b"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__A_REASON_LEAST(Py::Int(12));
    s__OPENS_ON_A_FIGURE(stdlib_call("re", "compile", &[py_str("^\\s*(?:[£$\\u20ac]\\s*\\d|\\d)")]).unwrap_or_else(|e| exc::raise(e)));
    s__A_CATEGORY(stdlib_call("re", "compile", &[py_str("\\b(?:it|them|these|those|things|stuff|bits|ones|kinds|sorts|lots)\\b|\\b(?:need|needs|want|wants|have|has|how|what|where|when|who)\\s+to\\W*$"), stdlib_attr("re", "I").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_FORWARD(stdlib_call("re", "compile", &[py_str("\\b(?:it|them|these|those)\\W*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_FORWARD_ANY(stdlib_call("re", "compile", &[py_str("\\bthe (?:most|best|worst|biggest|main|first|last|rest)\\W*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s__POINTS_FORWARD_LEAST(Py::Int(10));
    s_WHY_YOU_SAID_IT(stdlib_call("re", "compile", &[py_str("^(?:(?:but|and|so|ok|okay|yeah|no|well|right)\\s+)*(?:(?:why|y|how come)\\s+(?:(?:do|did|would|will|are|were)\\s+)?(?:you|u)\\s+(?:say|saying|said|tell me|told me|reckon|claim)|what\\s+makes\\s+(?:you|u)\\s+say|why\\s+(?:do\\s+you\\s+|would\\s+you\\s+)?think)\\s+(?:that|this|so|it|those|these)(?:\\s+is)?(?:\\s+(?:then|though|tho))?[\\s?!.,]*$"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_THE_BOT_DESCRIBED_ITSELF(stdlib_call("re", "compile", &[py_str("\\b(?:a program|a chatbot|a bot|not a person|no feelings|nothing in me|just rules|written rules|rules someone wrote|rules doing their job|no mind behind|behind the curtain|nothing to report on my side|no inner)\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_CARE_ANSWER(stdlib_call("re", "compile", &[py_str("\\b(?:i'?m (?:really |so )?sorry|so sorry|sorry to hear|sorry about|a real loss|be gentle with yourself|no words needed|you did the right thing|i'?m here for you|here if you want|nothing you have to say|how are you (?:holding up|doing))\\b"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    s_WHY_CARE(py_tuple(vec![py_str("Nothing behind it: you told me something hard, and I meant it."), py_str("No reason beyond what you said - I'm sorry, and I still mean it."), py_str("There's nothing to work out in it: you said something hard, and that was enough for me to be sorry about it.")]));
    s_WHY_THE_BOT(py_tuple(vec![py_str("Because there is nothing in here to feel it with: I'm rules someone wrote for ordinary questions, and the words come from those rather than from anything I have lived."), py_str("Because that is all I am made of - written rules. Nothing in me feels something first and reports it after, so there is no inner reason behind the line."), py_str("Because there is no machinery for it in here, which is the whole of the answer: I pick the words from what is written into me and from what you have told me here.")]));
    s_ABOUT_THE_BOT_LINE(stdlib_call("re", "compile", &[py_str("^\\W*(?:(?:so|ok|okay|and|but|well|right|hey|hi)\\s+)*(?:(?:are|do|did|does|can|could|would|will|have|has|were|is)\\s+(?:you|u|ya)\\b|(?:you|u)\\s+(?:are|re)\\b|tell me about yourself\\b|what are you\\b|who are you\\b|what do you do\\b)"), stdlib_attr("re", "IGNORECASE").unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
}
pub fn _named_as_a_sentence(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut reason: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "reason") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: reason"))) };
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut want: Py = Py::None;
    let _ = py_str("True when the answer named this very clause as its cause, as \"the cause is ...\".");
    want = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = reason.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,;:")]);
    if py_truthy(&Py::Bool(!py_truthy(&want.clone()))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    sentence = __item.clone();
    found = mcall(&g_NAMED_CAUSE(), "search", &[sentence.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    continue;
    }
    clause = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "lower", &[]), "split", &[])]), "strip", &[py_str(" .,;:")]);
    if py_truthy(&Py::Bool(!py_eq(&clause.clone(), &want.clone()))) {
    continue;
    }
    return Py::Bool(true);
    }
    return Py::Bool(false);
    Py::None
}
pub fn _reads_a_fault(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut said: Py = Py::None;
    let mut them: Py = Py::None;
    let _ = py_str("True when the turn above was about a thing that can go wrong, not about the person's life.\n\n    The fault wording asks which part of the process goes wrong, so it needs a process: a loaf,\n    a boiler, a laptop, a washing machine. A line about a job, a row with a friend or a funeral\n    is not one, and the honest reply there is the shrug that asks what happened.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    them = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = mcall(&{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_dict(vec![]) } }, "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&g_LIFE(), "search", &[them.clone()])) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[said.clone()]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _about_someone_elses_decision(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("True when the person's own last line was about somebody else deciding or refusing.\n\n    Read on their line rather than the bot's, because the bot's last line is the advice the \"why\n    not\" is asking about. Only their line can say whether there is another party in the question.\n    ");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&{ let __b = mcall(&g_THEIR_DECISION(), "search", &[low.clone()]); if !py_truthy(&__b) { __b } else { mcall(&g_SAID_NO(), "search", &[low.clone()]) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _the_suggestion_was_theirs(args: &[Py]) -> Py {
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
    let mut low: Py = Py::None;
    let mut said: Py = Py::None;
    let mut turns: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the person's own line handed on the advice the \"why not\" is asking about.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return Py::Bool(false);
    }
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    continue;
    }
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    continue;
    }
    low = mcall(&py_str(" "), "join", &[mcall(&mcall(&said.clone(), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&g_THEIR_SUGGESTION(), "search", &[low.clone()])) {
    return Py::Bool(true);
    }
    words = mcall(&mcall(&low.clone(), "strip", &[py_str(" .,!?:;")]), "split", &[]);
    if py_truthy(&{ let __b = words.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_SUGGESTED_VERBS(), &mcall(&geti(&words.clone(), &Py::Int(0)), "strip", &[py_str(".,!?:;")]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _in_frame(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut frame: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "frame") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: frame"))) };
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut keep: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("The clause put into a frame, with its first letter following the frame's sentence.");
    word = mcall(&geti(&mcall(&clause.clone(), "split", &[py_str(" "), Py::Int(1)]), &Py::Int(0)), "strip", &[py_str("\"'(*[")]);
    keep = { let __b = Py::Bool(py_contains(&g_KEEP_CAPITAL(), &mcall(&mcall(&word.clone(), "lower", &[]), "strip", &[py_str(".,!?:;")]))); if py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&word.clone(), &Py::Int(1), &Py::Int(2), &Py::None), "isupper", &[]) } };
    if py_truthy(&mcall(&frame.clone(), "startswith", &[py_str("{c}")])) {
    return mcall(&frame.clone(), "replace", &[py_str("{c}"), clause.clone()]);
    }
    if py_truthy(&{ let __b = keep.clone(); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&clause.clone())) } }) {
    return mcall(&frame.clone(), "replace", &[py_str("{c}"), clause.clone()]);
    }
    return mcall(&frame.clone(), "replace", &[py_str("{c}"), add(&mcall(&slice_or_raise(&clause.clone(), &Py::None, &Py::Int(1), &Py::None), "lower", &[]), &slice_or_raise(&clause.clone(), &Py::Int(1), &Py::None, &Py::None))]);
    Py::None
}
pub fn is_a_reason_line(args: &[Py]) -> Py {
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
    let mut front: Py = Py::None;
    let mut line: Py = Py::None;
    let _ = py_str("True when this is a cause this module handed back, not an answer of its own.\n\n    \"my phone battery drains fast\" / \"why is that\" is answered \"It's the reason I gave: social and\n    video apps in the background are usually it\", and the person then asks \"is it dangerous\". That\n    question is about the phone, so whoever answers it has to read past this line to the answer the\n    reason came out of: without this the verdict composer read the reason line as the answer and\n    sent a worried person to a search engine instead of saying the boiler's bang is ordinary.\n    ");
    line = mcall(&mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]), "lower", &[]), "replace", &[py_str("’"), py_str("'")]);
    if py_truthy(&Py::Bool(!py_truthy(&line.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&mcall(&line.clone(), "rstrip", &[py_str(" .")]), "endswith", &[g_WHY_END()])) {
    return Py::Bool(true);
    }
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&g_WHY_FRONT()) { let mut front: Py = __item.clone(); __out.push(mcall(&line.clone(), "startswith", &[front.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_the_reason_cannot_be_seen(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("True when the bot's last line was this module's own answer to the same question.\n\n    \"why\" asked twice in a row, or a \"why\" after \"what could be causing it\", is the same question\n    unchanged, and a search hedge the second time reads as if the first answer had not happened.\n    Only this module's own pools count, matched whole: a framed line carries the answer's own\n    words, so it is not a shrug and a second \"why\" on it is not this case.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&turns.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&call_builtin("isinstance", &[geti(&turns.clone(), &neg(&Py::Int(1))), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))) } } } }) {
    return Py::Bool(false);
    }
    said = call_builtin("str", &[{ let __b = mcall(&geti(&turns.clone(), &neg(&Py::Int(1))), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = Py::Bool(py_contains(&g_WHY_ANSWERED(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WHY_BODY(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WRONG_ANSWERED(), &said.clone())); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&g_WHY_METHOD(), &said.clone())); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&g_WRONG_NOT_A_FAULT(), &said.clone())) } } } } } } } };
    Py::None
}
pub fn _is_bare_why(args: &[Py]) -> Py {
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
    let _ = py_str("True when the whole line asks for the reason and nothing else.");
    return call_builtin("bool", &[mcall(&g_BARE_WHY(), "match", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _last_turn(args: &[Py]) -> Py {
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
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let mut skipped_a_nod: Py = Py::None;
    let mut turns: Py = Py::None;
    let _ = py_str("The turn above this one, or {} when there is nothing to explain.\n\n    Turns memory keeps {\"user\", \"bot\", \"module\"} for each line. Only an answer that made a claim\n    is worth asking the reason for: one of the ANSWERED modules, and long enough to be an answer.\n    ");
    turns = mcall(&memory.clone(), "get", &[py_str("turns")]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_dict(vec![]);
    }
    skipped_a_nod = Py::Bool(false);
    let __iter1 = py_iter_or_raise(&call_builtin("reversed", &[slice_or_raise(&turns.clone(), &neg(&Py::Int(3)), &Py::None, &Py::None)]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter1 {
    last = __item.clone();
    if py_truthy(&Py::Bool(!py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))))) {
    return py_dict(vec![]);
    }
    said = mcall(&last.clone(), "get", &[py_str("bot")]);
    if py_truthy(&{ let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { _answered_nothing(&[said.clone()]) } }) {
    skipped_a_nod = Py::Bool(true);
    continue;
    }
    if py_truthy(&Py::Bool(py_contains(&g_ANSWERED(), &mcall(&last.clone(), "get", &[py_str("module")])))) {
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_SUBSTANCE()) } }) {
    return py_dict(vec![]);
    }
    if py_truthy(&skipped_a_nod.clone()) {
    reason = _reason_in(&[last.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&reason.clone())); if py_truthy(&__b) { __b } else { mcall(&g_THE_REASON_POINTS_AT_THE_ADVICE(), "match", &[reason.clone()]) } }) {
    return py_dict(vec![]);
    }
    }
    return last.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(py_eq(&mcall(&last.clone(), "get", &[py_str("module")]), &py_str("greeting"))); if !py_truthy(&__b) { __b } else { { let __b = call_builtin("isinstance", &[said.clone(), py_func("str", |args: &[Py]| call_builtin("str", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { mcall(&g_HEDGED(), "search", &[said.clone()]) } } } }) {
    continue;
    }
    return py_dict(vec![]);
    }
    return py_dict(vec![]);
    Py::None
}
pub fn _answered_nothing(args: &[Py]) -> Py {
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
    let _ = py_str("True when a bot line is a nod rather than an answer: it gives no step and names no cause.");
    said = mcall(&{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&said.clone())); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_A_NOD_IS_SHORT()) } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_IMPERATIVE(), "match", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_INSTRUCTION(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CAUSE(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_EXPLAINS(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_CAUSE(), "search", &[said.clone()]) } }) {
    return Py::Bool(false);
    }
    return call_builtin("bool", &[mcall(&g_A_NOD_OPENS(), "match", &[said.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _gave_a_method(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut s: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut steps: Py = Py::None;
    let _ = py_str("True when the answer above was the how-to and said nothing about why it goes wrong.");
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g_CAUSE(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_EXPLAINS(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NAMED_CAUSE(), "search", &[said.clone()]) } } } }) {
    return Py::Bool(false);
    }
    sentences = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), said.clone()]).unwrap_or_else(|e| exc::raise(e))) { let mut s: Py = __item.clone(); if py_truthy(&mcall(&s.clone(), "strip", &[])) {  __out.push(mcall(&s.clone(), "strip", &[])); } } py_list(__out) })() };
    steps = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&sentences.clone()) { let mut s: Py = __item.clone(); if py_truthy(&{ let __b = mcall(&g_IMPERATIVE(), "match", &[s.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_INSTRUCTION(), "search", &[s.clone()]) } }) {  __out.push(s.clone()); } } py_list(__out) })() };
    return { let __b = call_builtin("bool", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { ge(&mul(&call_builtin("len", &[steps.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e))) } };
    Py::None
}
pub fn _a_cause_named_outright(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut cause: Py = Py::None;
    let mut named: Py = Py::None;
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut text: Py = Py::None;
    let mut thing: Py = Py::None;
    let _ = py_str("The cause the answer above names as a cause, or \"\" when it only has a \"because\" clause.\n\n    For \"why not\", where the answer's positive reason said back as a reason not to is the\n    opposite of what it wrote. The two shapes that name a cause on their face are the thing in\n    front of a colon (\"It is the caffeine: it peaks about half an hour after the cup ...\") and\n    \"the cause is ...\".\n    ");
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&slice_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), said.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &g_FIRST_SENTENCES(), &Py::None));
    for __item in __iter1 {
    sentence = __item.clone();
    text = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&mcall(&g_IMPERATIVE(), "match", &[text.clone()])) {
    continue;
    }
    named = mcall(&g_NAMED_CAUSE(), "search", &[text.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THE_FIX(), "search", &[text.clone()]))) } }) {
    cause = mcall(&mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&ge(&call_builtin("len", &[mcall(&cause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return cause.clone();
    }
    }
    thing = mcall(&g_A_THING_NAMED_BEFORE_A_COLON(), "match", &[text.clone()]);
    if py_truthy(&thing.clone()) {
    return add(&py_str("the "), &mcall(&mcall(&thing.clone(), "group", &[py_str("thing")]), "strip", &[]));
    }
    }
    return py_str("");
    Py::None
}
pub fn _reason_in(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut strict: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "strict") { v.clone() }
    else { Py::Bool(false) };
    let mut reason: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The cause the answer above already named, as one clause, or \"\" when it named none.\n\n    An answer sometimes carries an example in its own quotation marks, and the \"because\" in it is\n    the example's reason, not the answer's: \"A thesis statement is one sentence that says the\n    claim your essay will prove, and why it matters: `Cheaper public transport would cut city\n    emissions more than electric cars, because most journeys are short.` Put it at the end of your\n    first paragraph\" answered \"and why does it matter\" with \"Because most journeys are short.\" Put\n    it at the end of your first paragraph - the reason out of somebody else's sample sentence, cut\n    where its own sentence ended. So a reason found inside a quotation is thrown away and the\n    answer is read again without its examples. What is left is the answer's own claim, and if it\n    names no reason the shrug is the honest reply.\n    ");
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    reason = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(said.clone()); __kws.extend(vec![("strict".to_string(), strict.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reason_in(args))(&__args) };
    if py_truthy(&{ let __b = reason.clone(); if !py_truthy(&__b) { __b } else { _inside_a_quotation(&[said.clone(), reason.clone()]) } }) {
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(_without_examples(&[said.clone()])); __kws.extend(vec![("strict".to_string(), strict.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _a_reason_in(args))(&__args) };
    }
    return reason.clone();
    Py::None
}
pub fn _without_examples(args: &[Py]) -> Py {
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
    let _ = py_str("The answer with its quoted examples taken out, so their words cannot be handed back.");
    return mcall(&g_EXAMPLE_IN_QUOTES(), "sub", &[py_str(" "), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _inside_a_quotation(args: &[Py]) -> Py {
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
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut at: Py = Py::None;
    let mut before: Py = Py::None;
    let mut marks: Py = Py::None;
    let _ = py_str("True when the clause sits inside a quotation in the answer, so it is an example's words.");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    at = mcall(&said.clone(), "find", &[clause.clone()]);
    if py_truthy(&le(&at.clone(), &Py::Int(0))) {
    return Py::Bool(false);
    }
    before = slice_or_raise(&said.clone(), &Py::None, &at.clone(), &Py::None);
    marks = add(&add(&mcall(&before.clone(), "count", &[py_str("\"")]), &mcall(&before.clone(), "count", &[py_str("“")])), &mcall(&before.clone(), "count", &[py_str("”")]));
    return Py::Bool(py_eq(&modulo(&marks.clone(), &Py::Int(2)), &Py::Int(1)));
    Py::None
}
pub fn _a_reason_in(args: &[Py]) -> Py {
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
    let mut strict: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "strict") { v.clone() }
    else { Py::Bool(false) };
    let mut cause: Py = Py::None;
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut instruction_ok: Py = Py::None;
    let mut mark: Py = Py::None;
    let mut named: Py = Py::None;
    let mut named_class: Py = Py::None;
    let mut number: Py = Py::None;
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let mut passes: Py = Py::None;
    let mut piece: Py = Py::None;
    let mut sentence: Py = Py::None;
    let mut sentences: Py = Py::None;
    let mut short: Py = Py::None;
    let mut text: Py = Py::None;
    let mut trigger: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("The cause the answer above already named, as one clause, or \"\" when it named none.\n\n    Quoting the bot's own answer invents nothing: the words were said on the turn before. It is\n    cut to the clause that carries the cause, because the rest of the answer is the advice the\n    person already has.\n\n    The sentences are read twice. The first reading only takes a part that is not an instruction,\n    because a part that starts with one is what to do, not why it happened: \"take it off the heat,\n    because the smoky taste spreads\" and \"wipe the mould off, then stop it coming back\" carry\n    cause words and would answer \"why\" with a to-do list. Only when the answer names its cause\n    nowhere else does the second reading take those parts too, which is what it did before the\n    line shapes here were widened - so a line that used to get a reason still gets one.\n    ");
    whole = stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), said.clone()]).unwrap_or_else(|e| exc::raise(e));
    sentences = slice_or_raise(&whole.clone(), &Py::None, &g_FIRST_SENTENCES(), &Py::None);
    let __iter1 = py_iter_or_raise(&sentences.clone());
    for __item in __iter1 {
    sentence = __item.clone();
    text = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&mcall(&g_IMPERATIVE(), "match", &[text.clone()])) {
    continue;
    }
    named = { let __b = mcall(&g_A_THING_NAMED_BEFORE_A_COLON(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_A_THING_NAMED_BEFORE_A_COLON(), "match", &[mcall(&g_A_LEAD_IN(), "sub", &[py_str(""), text.clone()])]) } };
    if py_truthy(&named.clone()) {
    return add(&py_str("the "), &mcall(&mcall(&named.clone(), "group", &[py_str("thing")]), "strip", &[]));
    }
    short = mcall(&g_SHORT_OF_A_CAUSE(), "match", &[text.clone()]);
    if py_truthy(&short.clone()) {
    cause = mcall(&mcall(&py_str(" "), "join", &[mcall(&mcall(&short.clone(), "group", &[py_str("cause")]), "split", &[])]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&ge(&call_builtin("len", &[mcall(&cause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return cause.clone();
    }
    }
    }
    let __iter2 = py_iter_or_raise(&whole.clone());
    for __item in __iter2 {
    sentence = __item.clone();
    found = mcall(&g_NAMED_CAUSE(), "search", &[sentence.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_INSTRUCTION(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_RESULT_NUMBER(), "search", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_THE_FIX(), "search", &[sentence.clone()]) } } } } } }) {
    continue;
    }
    clause = mcall(&mcall(&mcall(&found.clone(), "group", &[Py::Int(1)]), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&ge(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return clause.clone();
    }
    }
    let __iter3 = py_iter_or_raise(&sentences.clone());
    for __item in __iter3 {
    sentence = __item.clone();
    named = mcall(&g_WHAT_IT_TAKES(), "search", &[sentence.clone()]);
    if py_truthy(&{ let __b = named.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_IMPERATIVE(), "match", &[mcall(&sentence.clone(), "strip", &[])]))) } }) {
    clause = mcall(&mcall(&mcall(&named.clone(), "group", &[Py::Int(1)]), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&ge(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return clause.clone();
    }
    }
    }
    passes = if py_truthy(&strict.clone()) { py_tuple(vec![Py::Bool(false)]) } else { py_tuple(vec![Py::Bool(false), Py::Bool(true)]) };
    let __iter4 = py_iter_or_raise(&passes.clone());
    for __item in __iter4 {
    instruction_ok = __item.clone();
    let __iter5 = py_iter_or_raise(&call_builtin("enumerate", &[sentences.clone()]).unwrap_or_else(|e| exc::raise(e)));
    for __item in __iter5 {
    let __unpk6 = py_unpack(&__item.clone(), 2, None);
    number = __unpk6[0].clone();
    sentence = __unpk6[1].clone();
    if py_truthy(&mcall(&g_RESULT_NUMBER(), "search", &[sentence.clone()])) {
    continue;
    }
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_DANGLING(), "match", &[mcall(&sentence.clone(), "strip", &[])]) } }) {
    continue;
    }
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&instruction_ok.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_INSTRUCTION(), "search", &[sentence.clone()]) } } } }) {
    continue;
    }
    if py_truthy(&mcall(&g_CONDITIONAL_SENTENCE(), "match", &[mcall(&sentence.clone(), "strip", &[])])) {
    continue;
    }
    trigger = _trigger_in(&[sentence.clone()]);
    if py_truthy(&{ let __b = trigger.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_THE_FIX(), "search", &[trigger.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_INSTRUCTION(), "search", &[sentence.clone()]) } })) } } } }) {
    return trigger.clone();
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g_CAUSE(), "search", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_CLAIM_BEFORE_COLON(), "match", &[mcall(&sentence.clone(), "strip", &[])]))); if !py_truthy(&__b) { __b } else { { let __b = mcall(&g_EXPLAINS(), "search", &[sentence.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_IMPERATIVE(), "match", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_THE_FIX(), "search", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_CONDITIONAL_SENTENCE(), "match", &[mcall(&sentence.clone(), "strip", &[])]))); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_DOSE(), "search", &[sentence.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_is_the_answer_itself(&[said.clone(), sentence.clone()]))) } } } } } } } } } } } } } }) {
    clause = mcall(&mcall(&sentence.clone(), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&ge(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return clause.clone();
    }
    }
    if py_truthy(&mcall(&g_CONDITIONAL_SENTENCE(), "match", &[mcall(&sentence.clone(), "strip", &[])])) {
    continue;
    }
    parts = py_list(vec![sentence.clone()]);
    let __iter7 = py_iter_or_raise(&g_PARTS());
    for __item in __iter7 {
    mark = __item.clone();
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&parts.clone()) { let mut part: Py = __item.clone(); for __item in py_iter_or_raise(&mcall(&part.clone(), "split", &[mark.clone()])) { let mut piece: Py = __item.clone(); __out.push(piece.clone()); } } py_list(__out) })() };
    }
    let __iter8 = py_iter_or_raise(&parts.clone());
    for __item in __iter8 {
    part = __item.clone();
    clause = _cause_clause(&[part.clone(), number.clone(), instruction_ok.clone()]);
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_THE_FIX(), "search", &[clause.clone()]))) } }) {
    if py_truthy(&{ let __b = strict.clone(); if !py_truthy(&__b) { __b } else { _carries_the_fix(&[sentence.clone(), clause.clone()]) } }) {
    return mcall(&mcall(&sentence.clone(), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    }
    return clause.clone();
    }
    trigger = _trigger_in(&[part.clone()]);
    if py_truthy(&{ let __b = trigger.clone(); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&mcall(&g_THE_FIX(), "search", &[trigger.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_INSTRUCTION(), "search", &[part.clone()]) } })) } } } }) {
    return trigger.clone();
    }
    }
    }
    }
    named_class = _named_by_definition(&[said.clone()]);
    if py_truthy(&named_class.clone()) {
    return named_class.clone();
    }
    return _the_class_it_belongs_to(&[said.clone()]);
    Py::None
}
pub fn _is_the_answer_itself(args: &[Py]) -> Py {
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
    let mut sentence: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut part: Py = Py::None;
    let mut whole: Py = Py::None;
    let _ = py_str("True when this sentence is the whole answer, so handing it back explains nothing.\n\n    A one-sentence answer has no paragraph for the sentence to be a part of. Handing it back to\n    \"why\" gives the person the line they have just read with a frame in front of it: \"That's what\n    is behind it: Six to eight glasses a day, about 1.5 to 2 litres, is plenty, and more in heat\n    or after exercise...\" is not one word of reason, and the advice in it reads as if it were the\n    reason. The reading goes on to the parts below, where a `so` clause inside the sentence can\n    still name the cause (\"spit rather than rinse so the fluoride stays on\"); when there is none\n    the honest shrug is the reply, which is more use than the answer again.\n\n    Only a sentence that carries nearly every word of the answer counts. A sentence of a longer\n    answer is a part of it, and one that is most of a two-sentence answer is still not the whole\n    thing.\n    ");
    whole = _words(&[said.clone()]);
    part = _words(&[sentence.clone()]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&whole.clone())); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&part.clone())) } }) {
    return Py::Bool(false);
    }
    return ge(&call_builtin("len", &[part.clone()]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("max", &[Py::Int(3), call_builtin("int", &[mul(&call_builtin("len", &[whole.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Float(0.9))]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)));
    Py::None
}
pub fn _words(args: &[Py]) -> Py {
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
    let _ = py_str("The words of a line, folded, so punctuation and case cannot hide a repeat.");
    return { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "split", &[py_str("[^a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); if py_truthy(&word.clone()) {  __out.push(word.clone()); } } py_list(__out) })() };
    Py::None
}
pub fn _carries_the_fix(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut clause: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clause") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clause"))) };
    let mut found: Py = Py::None;
    let mut rest: Py = Py::None;
    let _ = py_str("True when the sentence says the cause and then the fix after its colon or dash.");
    rest = mcall(&sentence.clone(), "strip", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&rest.clone(), "startswith", &[clause.clone()])))) {
    return Py::Bool(false);
    }
    found = mcall(&g_AFTER_THE_MARK(), "match", &[slice_or_raise(&rest.clone(), &call_builtin("len", &[clause.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None)]);
    return call_builtin("bool", &[{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_IMPERATIVE(), "match", &[mcall(&mcall(&found.clone(), "group", &[py_str("rest")]), "strip", &[])]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _trigger_in(args: &[Py]) -> Py {
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
    let mut clause: Py = Py::None;
    let mut head: Py = Py::None;
    let mut match_: Py = Py::None;
    let mut text: Py = Py::None;
    let _ = py_str("The reason when the answer explains a fault by its trigger, else \"\".\n\n    \"A cake sinks when it's underbaked\", \"pastry cracks when it's too dry or too warm\": the\n    change comes first and the condition that makes it happen comes after `when`, so the whole\n    clause, cut where the advice starts, is the reason. It invents nothing - the words were said\n    on the turn before - and it is the shape most keeping and cooking entries use to explain a\n    fault. An instruction (\"replace them when the midsole feels flat\") and a definition (\"it's\n    done when a skewer comes out clean\") are not causes, so they keep the honest shrug.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = part.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&text.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_IMPERATIVE(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CONDITIONAL(), "match", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_NEGATIVE(), "search", &[text.clone()]) } } } } } }) {
    return py_str("");
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_WHEN_CAUSE(), "finditer", &[text.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    head = mcall(&slice_or_raise(&text.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&head.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CHANGE(), "search", &[head.clone()]))) } }) {
    continue;
    }
    clause = geti(&mcall(&g_SO_FROM_HERE(), "split", &[slice_or_raise(&text.clone(), &mcall(&match_.clone(), "start", &[]), &Py::None, &Py::None)]), &Py::Int(0));
    clause = geti(&mcall(&clause.clone(), "split", &[py_str(": ")]), &Py::Int(0));
    clause = mcall(&mcall(&mcall(&add(&add(&head.clone(), &py_str(" ")), &clause.clone()), "strip", &[]), "rstrip", &[py_str(".,;:")]), "strip", &[]);
    clause = _cut_long(&[clause.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    continue;
    }
    return clause.clone();
    }
    return py_str("");
    Py::None
}
pub fn _cause_clause(args: &[Py]) -> Py {
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
    let mut number: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "number") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: number"))) };
    let mut instruction_ok: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "instruction_ok") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: instruction_ok"))) };
    let mut clause: Py = Py::None;
    let mut found: Py = Py::None;
    let mut from_the_marker: Py = Py::None;
    let mut prefix: Py = Py::None;
    let _ = py_str("The cause in one part of a sentence, as a clause, or \"\" when it names none.");
    found = mcall(&g_CAUSE(), "search", &[part.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    found = mcall(&g_SO_REASON(), "search", &[part.clone()]);
    }
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = number.clone(); if !py_truthy(&__b) { __b } else { mcall(&g_INSTRUCTION(), "search", &[part.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&instruction_ok.clone())); if !py_truthy(&__b) { __b } else { mcall(&g_IMPERATIVE(), "match", &[mcall(&part.clone(), "strip", &[])]) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(py_contains(&py_tuple(vec![py_str("because"), py_str("since")]), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[])))) {
    prefix = slice_or_raise(&part.clone(), &Py::None, &mcall(&mcall(&part.clone(), "lower", &[]), "find", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[])]), &Py::None);
    if py_truthy(&{ let __b = mcall(&g_NEGATIVE(), "search", &[prefix.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONDITIONAL(), "search", &[prefix.clone()]) } }) {
    return py_str("");
    }
    }
    from_the_marker = { let __b = Py::Bool(py_contains(&py_tuple(vec![py_str("because"), py_str("since")]), &mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[]))); if py_truthy(&__b) { __b } else { Py::Bool(py_is(&gattr(&found.clone(), "re"), &g_SO_REASON())) } };
    clause = if py_truthy(&from_the_marker.clone()) { slice_or_raise(&part.clone(), &mcall(&mcall(&part.clone(), "lower", &[]), "find", &[mcall(&mcall(&found.clone(), "group", &[Py::Int(0)]), "lower", &[])]), &Py::None, &Py::None) } else { part.clone() };
    clause = geti(&mcall(&g_SO_ADVICE(), "split", &[clause.clone()]), &Py::Int(0));
    clause = geti(&mcall(&g_AND_STEP(), "split", &[clause.clone()]), &Py::Int(0));
    clause = geti(&mcall(&g_AND_ADVICE(), "split", &[clause.clone()]), &Py::Int(0));
    clause = { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("because ")); __args.push(mcall(&clause.clone(), "strip", &[])); __kws.extend(vec![("count".to_string(), Py::Int(1))]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| mcall(&g_LEADING_MARKER(), "sub", args))(&__args) };
    clause = mcall(&mcall(&mcall(&clause.clone(), "strip", &[]), "rstrip", &[py_str(".,;:")]), "strip", &[]);
    clause = _cut_long(&[clause.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&mcall(&clause.clone(), "lower", &[]), "startswith", &[py_tuple(vec![py_str("because "), py_str("since ")])]))); if !py_truthy(&__b) { __b } else { mcall(&g_IMPERATIVE(), "match", &[clause.clone()]) } }) {
    return py_str("");
    }
    return clause.clone();
    Py::None
}
pub fn _the_class_it_belongs_to(args: &[Py]) -> Py {
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
    let mut claim: Py = Py::None;
    let mut first: Py = Py::None;
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let mut subject: Py = Py::None;
    let _ = py_str("The class the answer said the thing belongs to, as the reason for it, or \"\".\n\n    Read from the answer's own opening sentence, because that is where an entry puts what the thing\n    is: \"Most back pain is muscular and settles in two to six weeks, and staying active is what\n    helps most\". What comes back is the claim as it stands, not a rewrite of it, so the reply is the\n    bot's own words: quoting them invents nothing.\n    ");
    first = mcall(&geti(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0)), "strip", &[]);
    if py_truthy(&{ let __b = mcall(&g_IMPERATIVE(), "match", &[first.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CONDITIONAL_SENTENCE(), "match", &[first.clone()]) } }) {
    return py_str("");
    }
    found = mcall(&g_A_GENERAL_CLAIM(), "match", &[first.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&found.clone()))) {
    return py_str("");
    }
    claim = mcall(&mcall(&geti(&mcall(&g_ENDS_THE_CLASS(), "split", &[mcall(&found.clone(), "group", &[py_str("what")]), Py::Int(1)]), &Py::Int(0)), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[mcall(&claim.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(1)); if py_truthy(&__b) { __b } else { gt(&call_builtin("len", &[mcall(&claim.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(14)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_SAYS_WHAT_IT_DOES(), "search", &[claim.clone()]); if py_truthy(&__b) { __b } else { stdlib_call("re", "search", &[py_str("\\d"), claim.clone()]).unwrap_or_else(|e| exc::raise(e)) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(py_contains(&g_A_PLACEHOLDER(), &mcall(&claim.clone(), "lower", &[]))); if py_truthy(&__b) { __b } else { mcall(&g_THE_FIX(), "search", &[claim.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_INSTRUCTION(), "search", &[claim.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_DOSE(), "search", &[claim.clone()]) } }) {
    return py_str("");
    }
    subject = mcall(&mcall(&found.clone(), "group", &[py_str("subject")]), "strip", &[]);
    lead = mcall(&slice_or_raise(&first.clone(), &Py::None, &mcall(&found.clone(), "start", &[py_str("what")]), &Py::None), "strip", &[]);
    return mcall(&py_str("{} {}"), "format", &[{ let __b = lead.clone(); if py_truthy(&__b) { __b } else { add(&py_str("Most "), &subject.clone()) } }, claim.clone()]);
    Py::None
}
pub fn _named_by_definition(args: &[Py]) -> Py {
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
    let mut sentence: Py = Py::None;
    let mut why: Py = Py::None;
    let _ = py_str("The reason when the answer gives it as a definition: \"Most headaches are tension: ...\".\n\n    Some answers say what the thing is and then say what that means, and the part after the colon\n    is the reason the person is asking for. It is only taken when the claim in front of the colon\n    is about the thing and not an announcement (\"here is what to do\"), and when what follows reads\n    as a clause rather than as instructions, so the reason handed back is never an order.\n    ");
    let __iter1 = py_iter_or_raise(&slice_or_raise(&stdlib_call("re", "split", &[py_str("(?<=[.!?])\\s+"), call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::Int(1), &Py::None));
    for __item in __iter1 {
    sentence = __item.clone();
    found = mcall(&g_CLAIM_BEFORE_COLON(), "match", &[mcall(&sentence.clone(), "strip", &[])]);
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&found.clone())); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_CLAIM_SUBJECT(), "match", &[sentence.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_IMPERATIVE(), "match", &[mcall(&sentence.clone(), "strip", &[])]) } } } }) {
    continue;
    }
    why = mcall(&mcall(&mcall(&found.clone(), "group", &[py_str("why")]), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    if py_truthy(&{ let __b = Py::Bool(py_contains(&py_str("\"'‘“"), &slice_or_raise(&why.clone(), &Py::None, &Py::Int(1), &Py::None))); if py_truthy(&__b) { __b } else { { let __b = mcall(&g_IMPERATIVE(), "match", &[why.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_INSTRUCTION(), "match", &[why.clone()]) } } } }) {
    continue;
    }
    why = mcall(&mcall(&geti(&mcall(&g_SO_ADVICE(), "split", &[why.clone()]), &Py::Int(0)), "strip", &[]), "rstrip", &[py_str(".,;:")]);
    why = _cut_long(&[why.clone()]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&why.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) {
    continue;
    }
    return add(&add(&py_str("because "), &mcall(&geti(&why.clone(), &Py::Int(0)), "lower", &[])), &slice_or_raise(&why.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    Py::None
}
pub fn _cut_long(args: &[Py]) -> Py {
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
    let mut at: Py = Py::None;
    let mut head: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("The clause at its own comma, or \"\" when it only ends where the character limit is.\n\n    `\"...and if that fails make...\"` is a cut-off instruction handed back as a reason. A clause\n    that has to be chopped is not a reason this module can stand behind.\n    ");
    words = mcall(&clause.clone(), "split", &[]);
    if py_truthy(&le(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(24))) {
    return clause.clone();
    }
    head = mcall(&py_str(" "), "join", &[slice_or_raise(&words.clone(), &Py::None, &Py::Int(24), &Py::None)]);
    at = call_builtin("max", &[mcall(&head.clone(), "rfind", &[py_str(", ")]), mcall(&head.clone(), "rfind", &[py_str(" - ")])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&at.clone(), &floordiv(&call_builtin("len", &[head.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)))) {
    return py_str("");
    }
    return slice_or_raise(&head.clone(), &Py::None, &at.clone(), &Py::None);
    Py::None
}
pub fn _an_answer_to_act_on(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut last: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut said: Py = Py::None;
    let mut sentence: Py = Py::None;
    let _ = py_str("True when the answer above says to act now, so the reason is not read out of it.");
    said = call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g_ACT_NOW(), "search", &[said.clone()])) {
    return Py::Bool(true);
    }
    let __iter1 = py_iter_or_raise(&mcall(&g_SENTENCE(), "findall", &[said.clone()]));
    for __item in __iter1 {
    sentence = __item.clone();
    if py_truthy(&{ let __b = mcall(&g_ACT_NOW_SOON(), "search", &[sentence.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&mcall(&g_CONDITIONAL(), "search", &[sentence.clone()]))) } }) {
    return Py::Bool(true);
    }
    }
    return Py::Bool(false);
    Py::None
}
pub fn _first_sentence(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The answer's own opening sentence, or \"\" when it has only one.");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_END_OF_SENTENCE(), "split", &[said.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return py_str("");
    }
    return mcall(&geti(&parts.clone(), &Py::Int(0)), "rstrip", &[py_str(",;:- ")]);
    Py::None
}
pub fn _a_shorter_way(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut first: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The answer above said shorter, or \"\" when there is nothing in it to cut down to.\n\n    In order: the answer's opening sentence, cut at a colon or semicolon when that sentence is a\n    long one and the cut is a real clause; the opening sentence itself when it is one; and a clause\n    of the whole answer for an answer that is a single sentence.\n\n    The cut comes first only for a long sentence, because \"In short:\" in front of a sentence the\n    length of a paragraph is not a short form of anything: \"Defrost it when the ice is more than\n    about 5mm thick: switch it off at the wall, move the food into a cool box, and leave bowls of\n    hot water inside with the door shut.\" was handed back whole behind the frame at 37 words, where\n    the answer two turns up says \"I said: Defrost it when the ice is more than about 5mm thick.\"\n    A sentence short enough to be read again is kept whole even when it has a colon in it, because\n    its own opening is what a person who did not follow it needs (\"A cold stays in the nose and\n    throat: a runny nose, sneezing, a sore throat\" says more than \"A cold stays in the nose and\n    throat.\"). A clause is only taken when it is a real one - \"The things that work: same wake-up\n    time every day...\" opens on a label, and handing back \"In short: The things that work.\" is the\n    label and none of the answer.\n\n    Only the answer's own words, in their own order, so a shortened answer cannot be wrong in a\n    way the long one was not.\n    ");
    said = mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]);
    if py_truthy(&Py::Bool(!py_truthy(&said.clone()))) {
    return py_str("");
    }
    lead = _past_a_lead_in(&[said.clone()]);
    first = _first_sentence(&[lead.clone()]);
    if py_truthy(&first.clone()) {
    doing = _the_doing_of(&[first.clone()]);
    if py_truthy(&doing.clone()) {
    return doing.clone();
    }
    clause = _a_clause_of(&[first.clone()]);
    if py_truthy(&gt(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__A_LONG_SENTENCE())) {
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { le(&mul(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__A_STRIDE()), &call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return clause.clone();
    }
    if py_truthy(&Py::Bool(!py_eq(&lead.clone(), &mcall(&py_str(" "), "join", &[mcall(&said.clone(), "split", &[])])))) {
    return _end_a_sentence(&[first.clone()]);
    }
    return py_str("");
    } else {
    if py_truthy(&{ let __b = clause.clone(); if !py_truthy(&__b) { __b } else { ge(&mul(&call_builtin("len", &[mcall(&clause.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2)), &call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return clause.clone();
    }
    }
    if py_truthy(&le(&call_builtin("len", &[mcall(&first.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_AGAIN_WORDS())) {
    return _end_a_sentence(&[first.clone()]);
    }
    return clause.clone();
    }
    return _a_clause_of(&[_past_a_lead_in(&[said.clone()])]);
    Py::None
}
pub fn _past_a_lead_in(args: &[Py]) -> Py {
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
    let mut part: Py = Py::None;
    let mut parts: Py = Py::None;
    let _ = py_str("The answer without a lead-in sentence in front of it, which is a label and not the answer.\n\n    \"Take it in this order. First clear any credit card...\" hands back \"In short: Take it in this\n    order.\" when the lead-in is read as the opening: five words that say what the answer is about\n    to do and none of it. A short opening sentence with more after it is stepped over, so the\n    sentence under it is the one kept.\n    ");
    parts = { (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&mcall(&g_END_OF_SENTENCE(), "split", &[said.clone()])) { let mut part: Py = __item.clone(); if py_truthy(&mcall(&part.clone(), "strip", &[])) {  __out.push(mcall(&part.clone(), "strip", &[])); } } py_list(__out) })() };
    if py_truthy(&lt(&call_builtin("len", &[parts.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(2))) {
    return said.clone();
    }
    if py_truthy(&ge(&call_builtin("len", &[mcall(&geti(&parts.clone(), &Py::Int(0)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &add(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    return said.clone();
    }
    return mcall(&slice_or_raise(&said.clone(), &call_builtin("len", &[geti(&parts.clone(), &Py::Int(0))]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    Py::None
}
pub fn _end_a_sentence(args: &[Py]) -> Py {
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
    let _ = py_str("The sentence with its full stop, so the frame in front of it reads as a sentence.");
    text = mcall(&mcall(&{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "rstrip", &[py_str(",;:- ")]);
    if py_truthy(&Py::Bool(!py_truthy(&text.clone()))) {
    return py_str("");
    }
    return if py_truthy(&Py::Bool(py_contains(&py_str(".!?"), &geti(&text.clone(), &neg(&Py::Int(1)))))) { text.clone() } else { add(&text.clone(), &py_str(".")) };
    Py::None
}
pub fn _a_clause_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut cut: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The first real clause of one long sentence, or \"\" when it has none worth handing over.");
    sentence = mcall(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE_END(), "finditer", &[sentence.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    cut = mcall(&mcall(&slice_or_raise(&sentence.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[]), "rstrip", &[py_str(",;:-— ")]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &add(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_brackets_closed(&[cut.clone()])))) {
    continue;
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_carries_a_verb(&[cut.clone()]))); if py_truthy(&__b) { __b } else { _leans_back(&[cut.clone()]) } }) {
    continue;
    }
    if py_truthy(&_points_at_what_follows(&[cut.clone()])) {
    continue;
    }
    return _end_a_sentence(&[cut.clone()]);
    }
    return py_str("");
    Py::None
}
pub fn _the_doing_of(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut sentence: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "sentence") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: sentence"))) };
    let mut cut: Py = Py::None;
    let mut match_: Py = Py::None;
    let _ = py_str("The half after the sentence's colon when that half is what to do, or \"\" otherwise.\n\n    Read before the opening cut, because the stride rule below asks whether the opening half is\n    enough of the sentence to stand for it - and the half that says what to do is the answer to\n    \"what?\" at any length, which is the opposite of the opening half.\n    ");
    sentence = mcall(&{ let __b = sentence.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    let __iter1 = py_iter_or_raise(&mcall(&g_CLAUSE_END(), "finditer", &[sentence.clone()]));
    for __item in __iter1 {
    match_ = __item.clone();
    cut = mcall(&mcall(&slice_or_raise(&sentence.clone(), &Py::None, &mcall(&match_.clone(), "start", &[]), &Py::None), "strip", &[]), "rstrip", &[py_str(",;:-— ")]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &add(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    continue;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_brackets_closed(&[cut.clone()])))) {
    continue;
    }
    if py_truthy(&_points_at_what_follows(&[cut.clone()])) {
    continue;
    }
    return _end_a_sentence(&[_the_half_that_does_it(&[cut.clone(), slice_or_raise(&sentence.clone(), &mcall(&match_.clone(), "end", &[]), &Py::None, &Py::None)])]);
    }
    return py_str("");
    Py::None
}
pub fn _opens_a_doing(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let mut word: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when the half opens on a thing to do rather than on a thing to know.\n\n    The first word is what is read, not any word in the first three: \"warm tyres read a few psi\n    high\" is a statement with the verb \"read\" in it, and handing it back as the step for \"how do i\n    check my tyre pressure\" is the reason rather than the answer.\n    ");
    words = stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&call_builtin("str", &[{ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(false);
    }
    if py_truthy(&mcall(&g__A_DOING(), "fullmatch", &[geti(&words.clone(), &Py::Int(0))])) {
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(py_contains(&g__A_LEAD_IN(), &geti(&words.clone(), &Py::Int(0))))) {
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&slice_or_raise(&words.clone(), &Py::Int(1), &add(&g__A_DOING_WINDOW(), &Py::Int(2)), &Py::None)) { let mut word: Py = __item.clone(); __out.push(mcall(&g__A_DOING(), "fullmatch", &[word.clone()])); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    }
    return Py::Bool(false);
    Py::None
}
pub fn _says_how_it_works(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let mut first: Py = Py::None;
    let _ = py_str("True when a colon half is a clause about the thing rather than a list of the things.");
    first = geti(&stdlib_call("re", "split", &[py_str("[,;]"), call_builtin("str", &[{ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(0));
    return call_builtin("bool", &[{ let __b = mcall(&g__SAYS_HOW(), "search", &[first.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__VERB_SHAPE(), "search", &[first.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _announces_the_list(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let _ = py_str("True when a half is a heading for the list in the other half rather than the doing itself.");
    return call_builtin("bool", &[mcall(&g__ANNOUNCES(), "search", &[{ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _needs_its_subject(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let mut first: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when a half cannot stand alone as a short form: it opens on a pronoun or a participle.");
    words = mcall(&call_builtin("str", &[{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]);
    if py_truthy(&Py::Bool(!py_truthy(&words.clone()))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__NEEDS_A_SUBJECT(), "match", &[geti(&words.clone(), &Py::Int(0))])) {
    return Py::Bool(true);
    }
    first = stdlib_call("re", "sub", &[py_str("[^a-z]"), py_str(""), mcall(&geti(&words.clone(), &Py::Int(0)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    return { let __b = Py::Bool(py_contains(&g__A_PARTICIPLE(), &first.clone())); if !py_truthy(&__b) { __b } else { Py::Bool(!py_contains(&py_tuple(vec![py_str("set"), py_str("left"), py_str("put"), py_str("lost")]), &first.clone())) } };
    Py::None
}
pub fn _carries_a_verb(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let mut text: Py = Py::None;
    let mut word: Py = Py::None;
    let _ = py_str("True when a half is a sentence rather than a list of places or things.");
    text = call_builtin("str", &[{ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&text.clone(), "strip", &[])))) {
    return Py::Bool(false);
    }
    if py_truthy(&{ let __b = mcall(&g__A_DOING(), "search", &[text.clone()]); if py_truthy(&__b) { __b } else { mcall(&g__VERB_SHAPE(), "search", &[text.clone()]) } }) {
    return Py::Bool(true);
    }
    if py_truthy(&call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&stdlib_call("re", "findall", &[py_str("[a-z']+"), mcall(&text.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e))) { let mut word: Py = __item.clone(); __out.push(Py::Bool(py_contains(&g__A_VERB(), &word.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e))) {
    return Py::Bool(true);
    }
    if py_truthy(&mcall(&g__A_FIGURE(), "search", &[text.clone()])) {
    return Py::Bool(true);
    }
    return call_builtin("bool", &[mcall(&g__A_CAUSE_OPENS(), "match", &[text.clone()])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _leans_back(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut half: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "half") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: half"))) };
    let _ = py_str("True when a half needs the half before it to mean anything.\n\n    Only in its first few words: \"work through the examples with the book open, then close it and do\n    them again\" carries its own \"them\" - the examples - while \"Read them, and if half of them bore\n    you\" opens on a word with nothing behind it.\n    ");
    return call_builtin("bool", &[mcall(&g__LEANS_BACK(), "search", &[mcall(&py_str(" "), "join", &[slice_or_raise(&mcall(&call_builtin("str", &[{ let __b = half.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[]), &Py::None, &g__LEANS_BACK_WINDOW(), &Py::None)])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_reason(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("True when the half after the colon justifies the half before it rather than completing it.");
    after = call_builtin("str", &[{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = gt(&call_builtin("len", &[mcall(&after.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__A_REASON_LEAST()); if py_truthy(&__b) { __b } else { { let __b = Py::Bool(py_contains(&after.clone(), &py_str(","))); if py_truthy(&__b) { __b } else { Py::Bool(py_contains(&after.clone(), &py_str(";"))) } } } }) {
    return Py::Bool(false);
    }
    return _carries_a_verb(&[after.clone()]);
    Py::None
}
pub fn _opens_on_a_figure(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut after: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("True when the half after the colon opens on an amount or a count.");
    return call_builtin("bool", &[mcall(&g__OPENS_ON_A_FIGURE(), "match", &[call_builtin("str", &[{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _names_no_particular(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut before: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let _ = py_str("True when the opening is general enough that only the colon half says what to do.");
    return call_builtin("bool", &[mcall(&g__A_CATEGORY(), "search", &[{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _the_half_that_does_it(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut before: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "before") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: before"))) };
    let mut after: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "after") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: after"))) };
    let _ = py_str("The half of a colon sentence a person who did not follow it needs, or \"\" for the opening.\n\n    The opening half is kept when it is itself the doing (\"Take away the food and the easy way in\")\n    or when it carries the figure asked for (\"A pound or two, or round it up\"). The half after the\n    colon is handed over when the opening only says what is wrong or what a thing is and the doing\n    or the number is on the other side (\"A dripping tap is nearly always a worn washer: turn the\n    water off at the valve under the sink\").\n    ");
    before = mcall(&{ let __b = before.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    after = mcall(&mcall(&{ let __b = after.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]), "rstrip", &[py_str(",;:- ")]);
    if py_truthy(&lt(&call_builtin("len", &[mcall(&after.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &floordiv(&g__A_CLAUSE_LEAST(), &Py::Int(2)))) {
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&_carries_a_verb(&[after.clone()]))); if py_truthy(&__b) { __b } else { _leans_back(&[after.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&_needs_its_subject(&[after.clone()])) {
    return py_str("");
    }
    if py_truthy(&{ let __b = _opens_a_doing(&[before.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_announces_the_list(&[before.clone()]))) } }) {
    if py_truthy(&_opens_a_doing(&[after.clone()])) {
    if py_truthy(&ge(&call_builtin("len", &[mcall(&after.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&before.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)))) {
    return add(&mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    }
    if py_truthy(&{ let __b = _the_reason(&[after.clone()]); if py_truthy(&__b) { __b } else { _opens_on_a_figure(&[after.clone()]) } }) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g__A_FIGURE(), "search", &[before.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_on_a_figure(&[after.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_no_particular(&[before.clone()])))) {
    return py_str("");
    }
    if py_truthy(&_says_how_it_works(&[after.clone()])) {
    return py_str("");
    }
    return add(&mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&_opens_a_doing(&[after.clone()])) {
    if py_truthy(&{ let __b = mcall(&g__A_FIGURE(), "search", &[before.clone()]); if !py_truthy(&__b) { __b } else { { let __b = Py::Bool(!py_truthy(&_opens_on_a_figure(&[before.clone()]))); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_names_no_particular(&[before.clone()]))) } } } }) {
    return py_str("");
    }
    return add(&mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    if py_truthy(&mcall(&g__EXPLAINS(), "match", &[after.clone()])) {
    if py_truthy(&{ let __b = mcall(&g__A_FIGURE(), "search", &[before.clone()]); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&_opens_on_a_figure(&[after.clone()]))) } }) {
    return py_str("");
    }
    if py_truthy(&Py::Bool(!py_truthy(&_names_no_particular(&[before.clone()])))) {
    return py_str("");
    }
    if py_truthy(&_the_reason(&[after.clone()])) {
    return py_str("");
    }
    if py_truthy(&_carries_a_verb(&[before.clone()])) {
    return add(&mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&mcall(&g__A_FIGURE(), "search", &[before.clone()]))); if !py_truthy(&__b) { __b } else { mcall(&g__A_FIGURE(), "search", &[after.clone()]) } }) {
    return add(&mcall(&slice_or_raise(&after.clone(), &Py::None, &Py::Int(1), &Py::None), "upper", &[]), &slice_or_raise(&after.clone(), &Py::Int(1), &Py::None, &Py::None));
    }
    return py_str("");
    Py::None
}
pub fn _points_at_what_follows(args: &[Py]) -> Py {
    ensure_init();
    let mut __pos: Vec<Py> = Vec::new();
    let mut __kw: Vec<(String, Py)> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => { for (n, v) in kws.iter() { __kw.push((n.clone(), v.clone())); } }
            other => __pos.push(other.clone()),
        }
    }
    let mut cut: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "cut") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: cut"))) };
    let _ = py_str("True when a cut is too short and too vague to be anything but a heading.");
    cut = mcall(&{ let __b = cut.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[]);
    if py_truthy(&mcall(&g__POINTS_FORWARD_ANY(), "search", &[cut.clone()])) {
    return Py::Bool(true);
    }
    return { let __b = call_builtin("bool", &[mcall(&g__POINTS_FORWARD(), "search", &[cut.clone()])]).unwrap_or_else(|e| exc::raise(e)); if !py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[mcall(&cut.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g__POINTS_FORWARD_LEAST()) } };
    Py::None
}
pub fn _brackets_closed(args: &[Py]) -> Py {
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
    let _ = py_str("True when every bracket in `text` is opened and closed in it.");
    text = { let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } };
    return { let __b = Py::Bool(py_eq(&mcall(&text.clone(), "count", &[py_str("(")]), &mcall(&text.clone(), "count", &[py_str(")")]))); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&mcall(&text.clone(), "count", &[py_str("[")]), &mcall(&text.clone(), "count", &[py_str("]")]))) } };
    Py::None
}
pub fn _the_route_is_in_it(args: &[Py]) -> Py {
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
    let _ = py_str("True when the answer names the way out of it, so no part of it can be handed back alone.");
    return call_builtin("bool", &[mcall(&g_NO_CUT(), "search", &[call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn said_again(args: &[Py]) -> Py {
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
    let mut short: Py = Py::None;
    let _ = py_str("The reply to a bare \"say that again\", or None when the line is not one this reads.\n\n    Public because the composer's own repeat pool (`REACTION_REPEAT` in fallback.py) is reached\n    first for \"what?\" and \"huh\", and there is no reason for it to say a stock \"which bit?\" when\n    the answer above can simply be said shorter. When it cannot be said shorter the turn is left\n    to that pool: a stock \"which part?\" gives the person nothing where the answer's own opening\n    would, and the pool's \"I said: <the opening>\" is the answer's own words.\n    ");
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&g_SAID_AGAIN(), "match", &[clean.clone()])))) {
    return Py::None;
    }
    if py_truthy(&_an_answer_to_act_on(&[_last_turn(&[memory.clone()])])) {
    return Py::None;
    }
    short = _not_shortened(&[_said_above(&[memory.clone()])]);
    if py_truthy(&Py::Bool(!py_truthy(&short.clone()))) {
    return Py::None;
    }
    return mcall(&crate::gen::compose::pick(&[g_AGAIN_SHORTER(), memory.clone(), clean.clone()]), "replace", &[py_str("{short}"), short.clone()]);
    Py::None
}
pub fn is_the_answer_shorter(args: &[Py]) -> Py {
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
    let mut last: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "last") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: last"))) };
    let mut lead: Py = Py::None;
    let mut previous: Py = Py::None;
    let mut short: Py = Py::None;
    let mut size: Py = Py::None;
    let mut start: Py = Py::None;
    let mut text: Py = Py::None;
    let mut words: Py = Py::None;
    let _ = py_str("True when `reply` is the last answer's own words, shorter, in this module's frame.\n\n    The arbiter's never-repeat rule needs one thing this module knows and it cannot see: whether a\n    candidate that would otherwise be passed over is a second telling of the answer or a cut of\n    it. Every real candidate on a bare repeat is the answer again (\"Sure - <the paragraph>\"), and\n    the reply this module wrote for the same line is a cut of the same words - so it is the only\n    thing on the table that says anything new, and the arbiter may take it where the whole\n    paragraph is all that is beside it. Only the module's own frame is recognised and only where\n    the words behind it are the answer's own, in their own order, so a stock line that happens to\n    share a phrase is never read as a short form of anything.\n    ");
    text = mcall(&call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "strip", &[]);
    short = py_str("");
    let __iter1 = py_iter_or_raise(&g_REPEAT_LEADS());
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&mcall(&text.clone(), "startswith", &[lead.clone()])) {
    short = slice_or_raise(&text.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None);
    break;
    }
    }
    if py_truthy(&Py::Bool(!py_truthy(&short.clone()))) {
    return Py::Bool(false);
    }
    words = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&short.clone(), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    previous = stdlib_call("re", "findall", &[py_str("[a-z0-9']+"), mcall(&call_builtin("str", &[{ let __b = last.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&{ let __b = lt(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(3)); if py_truthy(&__b) { __b } else { lt(&call_builtin("len", &[previous.clone()]).unwrap_or_else(|e| exc::raise(e)), &add(&call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))) } }) {
    return Py::Bool(false);
    }
    size = call_builtin("len", &[words.clone()]).unwrap_or_else(|e| exc::raise(e));
    return call_builtin("any", &[{ (|| -> Py { let mut __out: Vec<Py> = Vec::new(); for __item in py_iter_or_raise(&call_builtin("range", &[Py::Int(0), add(&sub(&call_builtin("len", &[previous.clone()]).unwrap_or_else(|e| exc::raise(e)), &size.clone()), &Py::Int(1))]).unwrap_or_else(|e| exc::raise(e))) { let mut start: Py = __item.clone(); __out.push(Py::Bool(py_eq(&slice_or_raise(&previous.clone(), &start.clone(), &add(&start.clone(), &size.clone()), &Py::None), &words.clone()))); } py_list(__out) })() }]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn _said_above(args: &[Py]) -> Py {
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
    let _ = py_str("The answer the person is asking about, or \"\" when there is not one.\n\n    An answer that is itself a repeat (\"From just now: Titanic was built in Belfast...\") is read\n    for what it says, not for the frame it came in: shortening the framed line stacked one frame\n    on another (\"The gist of it: From just now: Titanic was built in Belfast\").\n    ");
    last = _last_turn(&[memory.clone()]);
    return _unframed(&[call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))]);
    Py::None
}
pub fn _unframed(args: &[Py]) -> Py {
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
    let mut found: Py = Py::None;
    let mut lead: Py = Py::None;
    let _ = py_str("The answer under a repeat's lead-in (\"From just now: ...\", \"You've got it: ...\").\n\n    Every module that hands an answer back puts a lead-in in front of it, and none of those leads\n    is part of the answer: shortening the framed line stacked one frame on another (\"The gist of\n    it: From just now: Titanic was built in Belfast\"). `bot/memory/another.FRAME` is the one place\n    that reads those leads, so this asks it rather than keeping a list of its own.\n    ");
    reply = call_builtin("str", &[{ let __b = reply.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    let __iter1 = py_iter_or_raise(&g_REPEAT_LEADS());
    for __item in __iter1 {
    lead = __item.clone();
    if py_truthy(&{ let __b = mcall(&reply.clone(), "startswith", &[lead.clone()]); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&reply.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]) } }) {
    return mcall(&slice_or_raise(&reply.clone(), &call_builtin("len", &[lead.clone()]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None), "strip", &[]);
    }
    }
    found = mcall(&crate::gen::memory::another::g_FRAME(), "match", &[reply.clone()]);
    if py_truthy(&{ let __b = found.clone(); if !py_truthy(&__b) { __b } else { mcall(&slice_or_raise(&reply.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]) } }) {
    return mcall(&slice_or_raise(&reply.clone(), &mcall(&found.clone(), "end", &[]), &Py::None, &Py::None), "strip", &[]);
    }
    return { let __b = crate::gen::memory::again::unframed(&[reply.clone()]); if py_truthy(&__b) { __b } else { reply.clone() } };
    Py::None
}
pub fn _not_shortened(args: &[Py]) -> Py {
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
    let mut doing: Py = Py::None;
    let mut echo: Py = Py::None;
    let mut saved: Py = Py::None;
    let mut short: Py = Py::None;
    let _ = py_str("The answer said shorter, or \"\" when the answer is not this module's to shorten.\n\n    The four kinds left alone are the answer that says to act now (its own words are what the\n    person needs), the answer that opens by taking bad news in (a condolence said shorter reads as\n    a form letter), the answer that names the way out (the opening is the harmless half and the\n    route is the sentence that matters), and the answer with nothing in it to cut.\n    ");
    said = call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&lt(&call_builtin("len", &[mcall(&said.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWER_LONG())) {
    return py_str("");
    }
    if py_truthy(&{ let __b = mcall(&g_CARE_ANSWER(), "search", &[said.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_OPENS_ON_CARE(), "match", &[{ let __b = _first_sentence(&[said.clone()]); if py_truthy(&__b) { __b } else { said.clone() } }]) } }) {
    return py_str("");
    }
    if py_truthy(&_the_route_is_in_it(&[said.clone()])) {
    return py_str("");
    }
    short = _a_shorter_way(&[said.clone()]);
    echo = { let __b = _first_sentence(&[_past_a_lead_in(&[said.clone()])]); if py_truthy(&__b) { __b } else { said.clone() } };
    doing = _end_a_sentence(&[_the_doing_of(&[echo.clone()])]);
    if py_truthy(&{ let __b = doing.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(py_eq(&short.clone(), &doing.clone())) } }) {
    echo = said.clone();
    }
    saved = if py_truthy(&short.clone()) { sub(&call_builtin("len", &[mcall(&echo.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &call_builtin("len", &[mcall(&short.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e))) } else { Py::Int(0) };
    if py_truthy(&{ let __b = Py::Bool(py_eq(&_past_a_lead_in(&[said.clone()]), &mcall(&py_str(" "), "join", &[mcall(&call_builtin("str", &[{ let __b = said.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]))); if !py_truthy(&__b) { __b } else { lt(&saved.clone(), &call_builtin("max", &[g__A_SAVE_LEAST(), floordiv(&call_builtin("len", &[mcall(&echo.clone(), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &Py::Int(4))]).unwrap_or_else(|e| exc::raise(e))) } }) {
    return py_str("");
    }
    return short.clone();
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
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut clean: Py = Py::None;
    let mut last: Py = Py::None;
    let _ = py_str("Read by the arbiter: on a bare \"why\" after an answer, the reason is this module's turn.\n\n    The line has no words of its own, so the module that answered first owns it by default and its\n    follow-up path hands the answer above back - \"Same as before: <the whole paragraph>\". That\n    paragraph is what the person just read, so nothing in it answers the line they typed. This\n    module reads that turn and gives the cause out of it, or says it cannot see one and asks, so\n    the arbiter asks for this declaration before the never-repeat fallback (README.md,\n    \"Interface\"). It says no to every line that is not a bare why after a substantive answer.\n\n    A card that warns of harm in the next few minutes is not claimed either, so the module that\n    wrote it hands it back whole (see `respond`). A card that says \"the same day\" is claimed: it is\n    the ordinary GP-this-week answer, and stepping aside for it handed the paragraph back.\n\n    Everything else is claimed even when no reason can be read out of the answer above, because\n    the alternative is not silence: the module that answered first hands its own answer back as\n    \"Same as before: <the whole paragraph>\", which is the paragraph the person just read. That\n    echo is what this claim exists to stop (it is the one reply the repeat fallback cannot see).\n    ");
    clean = mcall(&py_str(" "), "join", &[mcall(&mcall(&call_builtin("str", &[{ let __b = text.clone(); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "lower", &[]), "split", &[])]);
    if py_truthy(&mcall(&g_SAID_AGAIN(), "match", &[clean.clone()])) {
    if py_truthy(&_an_answer_to_act_on(&[_last_turn(&[memory.clone()])])) {
    return Py::Bool(false);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_not_shortened(&[_said_above(&[memory.clone()])])))) {
    return Py::Bool(false);
    }
    return Py::Bool(true);
    }
    if py_truthy(&Py::Bool(!py_truthy(&_is_bare_why(&[clean.clone()])))) {
    return Py::Bool(false);
    }
    last = _last_turn(&[memory.clone()]);
    if py_truthy(&_an_answer_to_act_on(&[last.clone()])) {
    return Py::Bool(false);
    }
    if py_truthy(&_said_the_reason_cannot_be_seen(&[memory.clone()])) {
    return Py::Bool(true);
    }
    return ge(&call_builtin("len", &[mcall(&call_builtin("str", &[{ let __b = mcall(&last.clone(), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), "split", &[])]).unwrap_or_else(|e| exc::raise(e)), &g_ANSWER_LONG());
    Py::None
}
pub fn asks_about_the_last_answer(args: &[Py]) -> Py {
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
    let _ = py_str("True when the line asks about the words the bot just used (\"why do you say that\").");
    return call_builtin("bool", &[mcall(&g_WHY_YOU_SAID_IT(), "match", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    Py::None
}
pub fn respond_to_a_saying_ask(args: &[Py]) -> Py {
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
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut body: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { Py::Bool(false) };
    let mut above: Py = Py::None;
    let mut last: Py = Py::None;
    let mut said: Py = Py::None;
    let _ = py_str("The reply to \"why do you say that\", or None when the line is not one of those.\n\n    It is the bare ask in more words, so the answer is the bare ask's answer: the reason read out\n    of the turn above. The one turn that gets its own reason is a saying ask about an answer that\n    described the bot itself: the reason asked for is the bot's own, and it is said from here\n    rather than left to a pool that no longer reads the line (see WHY_THE_BOT).\n\n    Two answers do not come out of the reason reading, and both are named here rather than handed\n    back to whatever is left, because what is left is the search hedge (\"that's beyond what I know\n    well, sorry\"): an answer that described the bot (WHY_THE_BOT) and an answer that was care\n    (WHY_CARE). Everything else is the reason, and when the answer above names no cause at all the\n    turn is left exactly as it was rather than turned into \"which part of that do you want the why\n    for?\" - a shove at a person who has just been told a fact they did not ask about.\n\n    Half of the composer's work is done before this function is reached: a line that says what it\n    is asking about (\"why did you say that about my sister\") is not this ask at all, and the pool\n    for a question about the bot answers the rest.\n    ");
    if py_truthy(&mcall(&g_THINKS_BACK(), "match", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])) {
    if py_truthy(&Py::Bool(!py_truthy(&mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("bot")])))) {
    return Py::None;
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(clean.clone()); __args.push(memory.clone()); __args.push(understood.clone()); __kws.extend(vec![("body".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| respond(args))(&__args) };
    }
    if py_truthy(&Py::Bool(!py_truthy(&asks_about_the_last_answer(&[clean.clone()])))) {
    return Py::None;
    }
    said = call_builtin("str", &[{ let __b = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("user")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_ABOUT_THE_BOT_LINE(), "match", &[said.clone()])) {
    return crate::gen::compose::pick(&[g_WHY_THE_BOT(), memory.clone(), clean.clone()]);
    }
    above = call_builtin("str", &[{ let __b = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_CARE_ANSWER(), "search", &[above.clone()])) {
    return crate::gen::compose::pick(&[g_WHY_CARE(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&mcall(&g_THE_BOT_DESCRIBED_ITSELF(), "search", &[above.clone()])) {
    return crate::gen::compose::pick(&[g_WHY_THE_BOT(), memory.clone(), clean.clone()]);
    }
    last = _last_turn(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    return Py::None;
    }
    if py_truthy(&Py::Bool(!py_truthy(&_reason_in(&[last.clone()])))) {
    let mut topics: Py = crate::gen::memory::topics::module_obj();
    return mcall(&topics.clone(), "said_again", &[call_builtin("str", &[{ let __b = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    }
    return { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(py_str("why")); __args.push(memory.clone()); __args.push(understood.clone()); __kws.extend(vec![("body".to_string(), body.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| respond(args))(&__args) };
    Py::None
}
pub fn _the_turn_above(args: &[Py]) -> Py {
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
    let mut turns: Py = Py::None;
    let _ = py_str("The last turn, whatever module answered it, or {} when there is none.");
    turns = if py_truthy(&call_builtin("isinstance", &[memory.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { mcall(&memory.clone(), "get", &[py_str("turns")]) } else { Py::None };
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&call_builtin("isinstance", &[turns.clone(), py_func("list", |args: &[Py]| call_builtin("list", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e)))); if py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&turns.clone())) } }) {
    return py_dict(vec![]);
    }
    last = geti(&turns.clone(), &neg(&Py::Int(1)));
    return if py_truthy(&call_builtin("isinstance", &[last.clone(), py_func("dict", |args: &[Py]| call_builtin("dict", args).unwrap_or_else(|e| exc::raise(e)))]).unwrap_or_else(|e| exc::raise(e))) { last.clone() } else { py_dict(vec![]) };
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
    let mut clean: Py = if __pos.len() > 0 { __pos[0].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "clean") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: clean"))) };
    let mut memory: Py = if __pos.len() > 1 { __pos[1].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "memory") { v.clone() }
    else { exc::raise(type_err(&format!("missing required argument: memory"))) };
    let mut understood: Py = if __pos.len() > 2 { __pos[2].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "understood") { v.clone() }
    else { Py::None };
    let mut body: Py = if __pos.len() > 3 { __pos[3].clone() }
    else if let Some((_, v)) = __kw.iter().find(|(n, _)| n == "body") { v.clone() }
    else { Py::Bool(false) };
    let mut act_now: Py = Py::None;
    let mut again: Py = Py::None;
    let mut handed_back: Py = Py::None;
    let mut last: Py = Py::None;
    let mut pool: Py = Py::None;
    let mut reason: Py = Py::None;
    let mut reply: Py = Py::None;
    let mut said: Py = Py::None;
    let mut thinks_back: Py = Py::None;
    let mut whole_thing: Py = Py::None;
    let _ = py_str("The reply to a bare \"why\" follow-up, or None when the line is not one.\n\n    `clean` is the tidied line, `memory` the conversation so far and `body` the caller's reading\n    of whether the last few lines were about their body. `understood` is unused here and kept so\n    every composer helper takes the same call shape.\n    ");
    again = said_again(&[clean.clone(), memory.clone()]);
    if py_truthy(&again.clone()) {
    return again.clone();
    }
    thinks_back = call_builtin("bool", &[mcall(&g_THINKS_BACK(), "match", &[mcall(&{ let __b = clean.clone(); if py_truthy(&__b) { __b } else { py_str("") } }, "strip", &[])])]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&Py::Bool(!py_truthy(&{ let __b = _is_bare_why(&[clean.clone()]); if py_truthy(&__b) { __b } else { thinks_back.clone() } }))) {
    return Py::None;
    }
    last = _last_turn(&[memory.clone()]);
    if py_truthy(&Py::Bool(!py_truthy(&last.clone()))) {
    if py_truthy(&mcall(&g_THE_BOT_DESCRIBED_ITSELF(), "search", &[call_builtin("str", &[{ let __b = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e))])) {
    return crate::gen::compose::pick(&[g_WHY_THE_BOT(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_said_the_reason_cannot_be_seen(&[memory.clone()])) {
    return crate::gen::compose::pick(&[g_WHY_AGAIN(), memory.clone(), clean.clone()]);
    }
    return Py::None;
    }
    act_now = _an_answer_to_act_on(&[last.clone()]);
    whole_thing = call_builtin("bool", &[{ let __b = mcall(&g_WRONG(), "search", &[clean.clone()]); if py_truthy(&__b) { __b } else { mcall(&g_CAUSING_IT(), "search", &[clean.clone()]) } }]).unwrap_or_else(|e| exc::raise(e));
    if py_truthy(&mcall(&g_WHY_NOT(), "search", &[clean.clone()])) {
    reason = if py_truthy(&act_now.clone()) { py_str("") } else { _a_cause_named_outright(&[last.clone()]) };
    if py_truthy(&Py::Bool(!py_truthy(&reason.clone()))) {
    if py_truthy(&_about_someone_elses_decision(&[memory.clone()])) {
    return crate::gen::compose::pick(&[g_WHY_NOT_ANSWERED(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&_the_suggestion_was_theirs(&[memory.clone()])) {
    return crate::gen::compose::pick(&[g_WHY_NOT_ITS_OWN(), memory.clone(), clean.clone()]);
    }
    return crate::gen::compose::pick(&[g_WHY_NOT_ANSWERED(), memory.clone(), clean.clone()]);
    }
    } else {
    reason = if py_truthy(&act_now.clone()) { py_str("") } else { { let mut __args: Vec<Py> = Vec::new(); let mut __kws: Vec<(String, Py)> = Vec::new(); __args.push(last.clone()); __kws.extend(vec![("strict".to_string(), whole_thing.clone())]); if !__kws.is_empty() { __args.push(Py::Kwargs(Rc::new(__kws))); } (|args: &[Py]| _reason_in(args))(&__args) } };
    }
    if py_truthy(&reason.clone()) {
    said = if py_truthy(&mcall(&mcall(&reason.clone(), "lower", &[]), "startswith", &[py_str("because ")])) { slice_or_raise(&reason.clone(), &call_builtin("len", &[py_str("because ")]).unwrap_or_else(|e| exc::raise(e)), &Py::None, &Py::None) } else { reason.clone() };
    if py_truthy(&_named_as_a_sentence(&[last.clone(), said.clone()])) {
    pool = g_WHY_NAMED_SELF();
    } else {
    pool = if py_truthy(&Py::Bool(!py_is(&said.clone(), &reason.clone()))) { g_WHY_NAMED_BECAUSE() } else { g_WHY_NAMED() };
    }
    reply = _in_frame(&[crate::gen::compose::pick(&[pool.clone(), memory.clone(), clean.clone()]), said.clone()]);
    reply = if py_truthy(&mcall(&reply.clone(), "endswith", &[py_tuple(vec![py_str("."), py_str("!"), py_str("?")])])) { reply.clone() } else { add(&reply.clone(), &py_str(".")) };
    reply = add(&mcall(&geti(&reply.clone(), &Py::Int(0)), "upper", &[]), &slice_or_raise(&reply.clone(), &Py::Int(1), &Py::None, &Py::None));
    return reply.clone();
    }
    if py_truthy(&{ let __b = thinks_back.clone(); if !py_truthy(&__b) { __b } else { Py::Bool(!py_truthy(&act_now.clone())) } }) {
    let mut topics: Py = crate::gen::memory::topics::module_obj();
    handed_back = mcall(&topics.clone(), "said_again", &[call_builtin("str", &[{ let __b = mcall(&_the_turn_above(&[memory.clone()]), "get", &[py_str("bot")]); if py_truthy(&__b) { __b } else { py_str("") } }]).unwrap_or_else(|e| exc::raise(e)), memory.clone()]);
    if py_truthy(&handed_back.clone()) {
    return handed_back.clone();
    }
    }
    if py_truthy(&body.clone()) {
    return crate::gen::compose::pick(&[g_WHY_BODY(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&{ let __b = Py::Bool(!py_truthy(&act_now.clone())); if !py_truthy(&__b) { __b } else { { let __b = whole_thing.clone(); if !py_truthy(&__b) { __b } else { { let __b = _gave_a_method(&[last.clone()]); if !py_truthy(&__b) { __b } else { _reads_a_fault(&[last.clone()]) } } } } } }) {
    return crate::gen::compose::pick(&[g_WHY_METHOD(), memory.clone(), clean.clone()]);
    }
    if py_truthy(&whole_thing.clone()) {
    pool = if py_truthy(&_reads_a_fault(&[last.clone()])) { g_WRONG_ANSWERED() } else { g_WRONG_NOT_A_FAULT() };
    return crate::gen::compose::pick(&[pool.clone(), memory.clone(), clean.clone()]);
    }
    return crate::gen::compose::pick(&[g_WHY_ANSWERED(), memory.clone(), clean.clone()]);
    Py::None
}
pub fn module_obj() -> Py {
    ensure_init();
    Py::Module(Rc::new(PyModule { name: "bot.compose.why".to_string(),
        get: Rc::new(|attr: &str| match attr {
            "ABOUT_THE_BOT_LINE" => Some(g_ABOUT_THE_BOT_LINE()),
            "ACT_NOW" => Some(g_ACT_NOW()),
            "ACT_NOW_SOON" => Some(g_ACT_NOW_SOON()),
            "AFTER_THE_MARK" => Some(g_AFTER_THE_MARK()),
            "AGAIN_SHORTER" => Some(g_AGAIN_SHORTER()),
            "AGAIN_WORDS" => Some(g_AGAIN_WORDS()),
            "AND_ADVICE" => Some(g_AND_ADVICE()),
            "AND_STEP" => Some(g_AND_STEP()),
            "ANSWERED" => Some(g_ANSWERED()),
            "ANSWER_LONG" => Some(g_ANSWER_LONG()),
            "A_GENERAL_CLAIM" => Some(g_A_GENERAL_CLAIM()),
            "A_LEAD_IN" => Some(g_A_LEAD_IN()),
            "A_NOD_IS_SHORT" => Some(g_A_NOD_IS_SHORT()),
            "A_NOD_OPENS" => Some(g_A_NOD_OPENS()),
            "A_PLACEHOLDER" => Some(g_A_PLACEHOLDER()),
            "A_THING_NAMED_BEFORE_A_COLON" => Some(g_A_THING_NAMED_BEFORE_A_COLON()),
            "BARE_WHY" => Some(g_BARE_WHY()),
            "CARE_ANSWER" => Some(g_CARE_ANSWER()),
            "CAUSE" => Some(g_CAUSE()),
            "CAUSING_IT" => Some(g_CAUSING_IT()),
            "CHANGE" => Some(g_CHANGE()),
            "CLAIM_BEFORE_COLON" => Some(g_CLAIM_BEFORE_COLON()),
            "CLAIM_SUBJECT" => Some(g_CLAIM_SUBJECT()),
            "CLAUSE_END" => Some(g_CLAUSE_END()),
            "CONDITIONAL" => Some(g_CONDITIONAL()),
            "CONDITIONAL_SENTENCE" => Some(g_CONDITIONAL_SENTENCE()),
            "DANGLING" => Some(g_DANGLING()),
            "DOSE" => Some(g_DOSE()),
            "ENDS_THE_CLASS" => Some(g_ENDS_THE_CLASS()),
            "END_OF_SENTENCE" => Some(g_END_OF_SENTENCE()),
            "EXAMPLE_IN_QUOTES" => Some(g_EXAMPLE_IN_QUOTES()),
            "EXPLAINS" => Some(g_EXPLAINS()),
            "FIRST_SENTENCES" => Some(g_FIRST_SENTENCES()),
            "HEDGED" => Some(g_HEDGED()),
            "IMPERATIVE" => Some(g_IMPERATIVE()),
            "INSTRUCTION" => Some(g_INSTRUCTION()),
            "KEEP_CAPITAL" => Some(g_KEEP_CAPITAL()),
            "LEADING_MARKER" => Some(g_LEADING_MARKER()),
            "LIFE" => Some(g_LIFE()),
            "NAMED_CAUSE" => Some(g_NAMED_CAUSE()),
            "NEGATIVE" => Some(g_NEGATIVE()),
            "NO_CUT" => Some(g_NO_CUT()),
            "OPENS_ON_CARE" => Some(g_OPENS_ON_CARE()),
            "PARTS" => Some(g_PARTS()),
            "REPEAT_LEADS" => Some(g_REPEAT_LEADS()),
            "RESULT_NUMBER" => Some(g_RESULT_NUMBER()),
            "SAID_AGAIN" => Some(g_SAID_AGAIN()),
            "SAID_NO" => Some(g_SAID_NO()),
            "SAYS_WHAT_IT_DOES" => Some(g_SAYS_WHAT_IT_DOES()),
            "SENTENCE" => Some(g_SENTENCE()),
            "SHORT_OF_A_CAUSE" => Some(g_SHORT_OF_A_CAUSE()),
            "SO_ADVICE" => Some(g_SO_ADVICE()),
            "SO_FROM_HERE" => Some(g_SO_FROM_HERE()),
            "SO_REASON" => Some(g_SO_REASON()),
            "STEP_VERBS" => Some(g_STEP_VERBS()),
            "SUBSTANCE" => Some(g_SUBSTANCE()),
            "SUGGESTED_VERBS" => Some(g_SUGGESTED_VERBS()),
            "THEIR_DECISION" => Some(g_THEIR_DECISION()),
            "THEIR_SUGGESTION" => Some(g_THEIR_SUGGESTION()),
            "THE_BOT_DESCRIBED_ITSELF" => Some(g_THE_BOT_DESCRIBED_ITSELF()),
            "THE_FIX" => Some(g_THE_FIX()),
            "THE_REASON_POINTS_AT_THE_ADVICE" => Some(g_THE_REASON_POINTS_AT_THE_ADVICE()),
            "THINKS_BACK" => Some(g_THINKS_BACK()),
            "WHAT_IT_TAKES" => Some(g_WHAT_IT_TAKES()),
            "WHEN_CAUSE" => Some(g_WHEN_CAUSE()),
            "WHY_AGAIN" => Some(g_WHY_AGAIN()),
            "WHY_ANSWERED" => Some(g_WHY_ANSWERED()),
            "WHY_BODY" => Some(g_WHY_BODY()),
            "WHY_CARE" => Some(g_WHY_CARE()),
            "WHY_END" => Some(g_WHY_END()),
            "WHY_FRONT" => Some(g_WHY_FRONT()),
            "WHY_METHOD" => Some(g_WHY_METHOD()),
            "WHY_NAMED" => Some(g_WHY_NAMED()),
            "WHY_NAMED_BECAUSE" => Some(g_WHY_NAMED_BECAUSE()),
            "WHY_NAMED_SELF" => Some(g_WHY_NAMED_SELF()),
            "WHY_NOT" => Some(g_WHY_NOT()),
            "WHY_NOT_ANSWERED" => Some(g_WHY_NOT_ANSWERED()),
            "WHY_NOT_ITS_OWN" => Some(g_WHY_NOT_ITS_OWN()),
            "WHY_THE_BOT" => Some(g_WHY_THE_BOT()),
            "WHY_YOU_SAID_IT" => Some(g_WHY_YOU_SAID_IT()),
            "WRONG" => Some(g_WRONG()),
            "WRONG_ANSWERED" => Some(g_WRONG_ANSWERED()),
            "WRONG_NOT_A_FAULT" => Some(g_WRONG_NOT_A_FAULT()),
            "_ANNOUNCES" => Some(g__ANNOUNCES()),
            "_A_CATEGORY" => Some(g__A_CATEGORY()),
            "_A_CAUSE_OPENS" => Some(g__A_CAUSE_OPENS()),
            "_A_CLAUSE_LEAST" => Some(g__A_CLAUSE_LEAST()),
            "_A_DOING" => Some(g__A_DOING()),
            "_A_DOING_WINDOW" => Some(g__A_DOING_WINDOW()),
            "_A_FIGURE" => Some(g__A_FIGURE()),
            "_A_LEAD_IN" => Some(g__A_LEAD_IN()),
            "_A_LONG_SENTENCE" => Some(g__A_LONG_SENTENCE()),
            "_A_PARTICIPLE" => Some(g__A_PARTICIPLE()),
            "_A_REASON_LEAST" => Some(g__A_REASON_LEAST()),
            "_A_SAVE_LEAST" => Some(g__A_SAVE_LEAST()),
            "_A_STRIDE" => Some(g__A_STRIDE()),
            "_A_VERB" => Some(g__A_VERB()),
            "_EXPLAINS" => Some(g__EXPLAINS()),
            "_LEANS_BACK" => Some(g__LEANS_BACK()),
            "_LEANS_BACK_WINDOW" => Some(g__LEANS_BACK_WINDOW()),
            "_NEEDS_A_SUBJECT" => Some(g__NEEDS_A_SUBJECT()),
            "_OPENS_ON_A_FIGURE" => Some(g__OPENS_ON_A_FIGURE()),
            "_POINTS_FORWARD" => Some(g__POINTS_FORWARD()),
            "_POINTS_FORWARD_ANY" => Some(g__POINTS_FORWARD_ANY()),
            "_POINTS_FORWARD_LEAST" => Some(g__POINTS_FORWARD_LEAST()),
            "_SAYS_HOW" => Some(g__SAYS_HOW()),
            "_VERB_SHAPE" => Some(g__VERB_SHAPE()),
            "_a_cause_named_outright" => Some(py_func("_a_cause_named_outright", |args: &[Py]| _a_cause_named_outright(args))),
            "_a_clause_of" => Some(py_func("_a_clause_of", |args: &[Py]| _a_clause_of(args))),
            "_a_reason_in" => Some(py_func("_a_reason_in", |args: &[Py]| _a_reason_in(args))),
            "_a_shorter_way" => Some(py_func("_a_shorter_way", |args: &[Py]| _a_shorter_way(args))),
            "_about_someone_elses_decision" => Some(py_func("_about_someone_elses_decision", |args: &[Py]| _about_someone_elses_decision(args))),
            "_an_answer_to_act_on" => Some(py_func("_an_answer_to_act_on", |args: &[Py]| _an_answer_to_act_on(args))),
            "_announces_the_list" => Some(py_func("_announces_the_list", |args: &[Py]| _announces_the_list(args))),
            "_answered_nothing" => Some(py_func("_answered_nothing", |args: &[Py]| _answered_nothing(args))),
            "_brackets_closed" => Some(py_func("_brackets_closed", |args: &[Py]| _brackets_closed(args))),
            "_carries_a_verb" => Some(py_func("_carries_a_verb", |args: &[Py]| _carries_a_verb(args))),
            "_carries_the_fix" => Some(py_func("_carries_the_fix", |args: &[Py]| _carries_the_fix(args))),
            "_cause_clause" => Some(py_func("_cause_clause", |args: &[Py]| _cause_clause(args))),
            "_cut_long" => Some(py_func("_cut_long", |args: &[Py]| _cut_long(args))),
            "_end_a_sentence" => Some(py_func("_end_a_sentence", |args: &[Py]| _end_a_sentence(args))),
            "_first_sentence" => Some(py_func("_first_sentence", |args: &[Py]| _first_sentence(args))),
            "_gave_a_method" => Some(py_func("_gave_a_method", |args: &[Py]| _gave_a_method(args))),
            "_in_frame" => Some(py_func("_in_frame", |args: &[Py]| _in_frame(args))),
            "_inside_a_quotation" => Some(py_func("_inside_a_quotation", |args: &[Py]| _inside_a_quotation(args))),
            "_is_bare_why" => Some(py_func("_is_bare_why", |args: &[Py]| _is_bare_why(args))),
            "_is_the_answer_itself" => Some(py_func("_is_the_answer_itself", |args: &[Py]| _is_the_answer_itself(args))),
            "_last_turn" => Some(py_func("_last_turn", |args: &[Py]| _last_turn(args))),
            "_leans_back" => Some(py_func("_leans_back", |args: &[Py]| _leans_back(args))),
            "_named_as_a_sentence" => Some(py_func("_named_as_a_sentence", |args: &[Py]| _named_as_a_sentence(args))),
            "_named_by_definition" => Some(py_func("_named_by_definition", |args: &[Py]| _named_by_definition(args))),
            "_names_no_particular" => Some(py_func("_names_no_particular", |args: &[Py]| _names_no_particular(args))),
            "_needs_its_subject" => Some(py_func("_needs_its_subject", |args: &[Py]| _needs_its_subject(args))),
            "_not_shortened" => Some(py_func("_not_shortened", |args: &[Py]| _not_shortened(args))),
            "_opens_a_doing" => Some(py_func("_opens_a_doing", |args: &[Py]| _opens_a_doing(args))),
            "_opens_on_a_figure" => Some(py_func("_opens_on_a_figure", |args: &[Py]| _opens_on_a_figure(args))),
            "_past_a_lead_in" => Some(py_func("_past_a_lead_in", |args: &[Py]| _past_a_lead_in(args))),
            "_points_at_what_follows" => Some(py_func("_points_at_what_follows", |args: &[Py]| _points_at_what_follows(args))),
            "_reads_a_fault" => Some(py_func("_reads_a_fault", |args: &[Py]| _reads_a_fault(args))),
            "_reason_in" => Some(py_func("_reason_in", |args: &[Py]| _reason_in(args))),
            "_said_above" => Some(py_func("_said_above", |args: &[Py]| _said_above(args))),
            "_said_the_reason_cannot_be_seen" => Some(py_func("_said_the_reason_cannot_be_seen", |args: &[Py]| _said_the_reason_cannot_be_seen(args))),
            "_says_how_it_works" => Some(py_func("_says_how_it_works", |args: &[Py]| _says_how_it_works(args))),
            "_the_class_it_belongs_to" => Some(py_func("_the_class_it_belongs_to", |args: &[Py]| _the_class_it_belongs_to(args))),
            "_the_doing_of" => Some(py_func("_the_doing_of", |args: &[Py]| _the_doing_of(args))),
            "_the_half_that_does_it" => Some(py_func("_the_half_that_does_it", |args: &[Py]| _the_half_that_does_it(args))),
            "_the_reason" => Some(py_func("_the_reason", |args: &[Py]| _the_reason(args))),
            "_the_route_is_in_it" => Some(py_func("_the_route_is_in_it", |args: &[Py]| _the_route_is_in_it(args))),
            "_the_suggestion_was_theirs" => Some(py_func("_the_suggestion_was_theirs", |args: &[Py]| _the_suggestion_was_theirs(args))),
            "_the_turn_above" => Some(py_func("_the_turn_above", |args: &[Py]| _the_turn_above(args))),
            "_trigger_in" => Some(py_func("_trigger_in", |args: &[Py]| _trigger_in(args))),
            "_unframed" => Some(py_func("_unframed", |args: &[Py]| _unframed(args))),
            "_without_examples" => Some(py_func("_without_examples", |args: &[Py]| _without_examples(args))),
            "_words" => Some(py_func("_words", |args: &[Py]| _words(args))),
            "asks_about_the_last_answer" => Some(py_func("asks_about_the_last_answer", |args: &[Py]| asks_about_the_last_answer(args))),
            "is_a_reason_line" => Some(py_func("is_a_reason_line", |args: &[Py]| is_a_reason_line(args))),
            "is_the_answer_shorter" => Some(py_func("is_the_answer_shorter", |args: &[Py]| is_the_answer_shorter(args))),
            "respond" => Some(py_func("respond", |args: &[Py]| respond(args))),
            "respond_to_a_saying_ask" => Some(py_func("respond_to_a_saying_ask", |args: &[Py]| respond_to_a_saying_ask(args))),
            "said_again" => Some(py_func("said_again", |args: &[Py]| said_again(args))),
            "takes_as_it_comes" => Some(py_func("takes_as_it_comes", |args: &[Py]| takes_as_it_comes(args))),
            _ => None,
        }),
        set: None,
    }))
}
